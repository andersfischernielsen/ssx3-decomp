//! Differential VU tester: replays captured program invocations
//! (`SSX3_VU_CAPTURE`, see `vu::capture`) through our translated microcode
//! and compares with the recorded in-game result or with the output of the
//! reference interpreter harness (`tools/vu-diff/`).
//!
//!   vu_diff self  <capture>                 replay vs. recorded in-game output
//!   vu_diff cmp   <capture> <ref.out> [-v]  replay vs. reference output
//!   vu_diff trace <capture> <index>         per-pair trace (reference-format)
//!   vu_diff list  <capture>                 one line per record
//!   vu_diff find  <capture> <hex word>      records whose XGKICK packets contain a word

use ssx3_ps2::vu::capture::{load, read_kicks, ArchState, Reader, Record};
use ssx3_ps2::vu::{reference, Vu};
use ssx3_ps2::Qword;
use std::collections::BTreeMap;

struct Out {
    kicks: Vec<Vec<Qword>>,
    end: ArchState,
    data: Vec<u8>,
    pairs: u64,
}

fn ref_model() -> bool {
    std::env::args().any(|a| a == "--ref-model")
}

fn replay(rec: &Record) -> (Out, Vu) {
    let mut vu = rec.setup();
    vu.instant_qp = ref_model();
    let mut kicks = Vec::new();
    let p0 = vu.pairs;
    vu.run(4_000_000, &mut |p: &[Qword]| kicks.push(p.to_vec()));
    (
        Out {
            kicks,
            end: ArchState::of(&vu),
            data: vu.data.clone(),
            pairs: vu.pairs - p0,
        },
        vu,
    )
}

fn read_ref(path: &str) -> BTreeMap<u32, Out> {
    let b = std::fs::read(path).expect("ref output");
    let mut r = Reader::new(&b);
    let mut m = BTreeMap::new();
    while r.pos < b.len() {
        let idx = r.u32().unwrap();
        let kicks = read_kicks(&mut r).unwrap();
        let end = read_arch(&mut r);
        let data = r.bytes().unwrap();
        let pairs = r.u64().unwrap();
        m.insert(idx, Out { kicks, end, data, pairs });
    }
    m
}

fn read_arch(r: &mut Reader) -> ArchState {
    // Same layout as ArchState::write (private): reuse via a tiny record.
    let mut s = ArchState::default();
    for v in s.vf.iter_mut() {
        for x in v.iter_mut() {
            *x = r.u32().unwrap();
        }
    }
    for x in s.acc.iter_mut() {
        *x = r.u32().unwrap();
    }
    for x in s.vi.iter_mut() {
        *x = r.u16().unwrap();
    }
    s.q = r.u32().unwrap();
    s.p = r.u32().unwrap();
    s.i = r.u32().unwrap();
    s.r = r.u32().unwrap();
    s.mac = r.u32().unwrap();
    s.status = r.u32().unwrap();
    s.clip = r.u32().unwrap();
    s.tpc = r.u32().unwrap();
    s
}

fn ulp(a: u32, b: u32) -> u32 {
    if (a ^ b) & 0x8000_0000 != 0 {
        return if (a | b) & 0x7fff_ffff == 0 { 0 } else { u32::MAX };
    }
    (a as i64 - b as i64).unsigned_abs() as u32
}

/// Compare two outputs; returns a list of (category, detail).
fn compare(a: &Out, b: &Out, start: &ArchState) -> Vec<(String, String)> {
    let mut d = Vec::new();
    if a.kicks.len() != b.kicks.len() {
        d.push(("kick-count".into(), format!("ours {} ref {}", a.kicks.len(), b.kicks.len())));
    }
    for (k, (x, y)) in a.kicks.iter().zip(&b.kicks).enumerate() {
        if x != y {
            let first = x.iter().zip(y.iter()).position(|(p, q)| p != q);
            let mut maxulp = 0u32;
            let mut words = 0;
            for (p, q) in x.iter().zip(y.iter()) {
                for l in 0..4 {
                    let (u, v) = ((*p >> (32 * l)) as u32, (*q >> (32 * l)) as u32);
                    if u != v {
                        words += 1;
                        maxulp = maxulp.max(ulp(u, v));
                    }
                }
            }
            let cat = if x.len() != y.len() {
                "kick-len"
            } else if maxulp <= 4 {
                "kick-ulp"
            } else {
                "kick-data"
            };
            d.push((
                cat.into(),
                format!(
                    "kick {k}: len {}/{} first diff qw {:?} ({} words, max ulp {}) ours {:#034x} ref {:#034x}",
                    x.len(),
                    y.len(),
                    first,
                    words,
                    maxulp,
                    first.map(|i| x[i]).unwrap_or(0),
                    first.map(|i| y[i]).unwrap_or(0)
                ),
            ));
        }
    }
    for r in 1..16 {
        if a.end.vi[r] != b.end.vi[r] {
            d.push((
                format!("vi"),
                format!("vi{r}: ours {:#06x} ref {:#06x} (start {:#06x})", a.end.vi[r], b.end.vi[r], start.vi[r]),
            ));
        }
    }
    for r in 0..32 {
        if a.end.vf[r] != b.end.vf[r] {
            let m = (0..4).map(|l| ulp(a.end.vf[r][l], b.end.vf[r][l])).max().unwrap();
            d.push((
                if m <= 4 { "vf-ulp" } else { "vf" }.into(),
                format!("vf{r}: ours {:08x?} ref {:08x?} (max ulp {m})", a.end.vf[r], b.end.vf[r]),
            ));
        }
    }
    if a.end.acc != b.end.acc {
        let m = (0..4).map(|l| ulp(a.end.acc[l], b.end.acc[l])).max().unwrap();
        d.push((
            if m <= 4 { "acc-ulp" } else { "acc" }.into(),
            format!("acc: ours {:08x?} ref {:08x?}", a.end.acc, b.end.acc),
        ));
    }
    for (n, x, y) in [
        ("q", a.end.q, b.end.q),
        ("p", a.end.p, b.end.p),
        ("i", a.end.i, b.end.i),
        ("r", a.end.r, b.end.r),
        ("mac", a.end.mac, b.end.mac),
        ("status", a.end.status, b.end.status),
        ("clip", a.end.clip, b.end.clip),
        ("tpc", a.end.tpc, b.end.tpc),
    ] {
        if x != y {
            let cat = if (n == "q" || n == "p") && ulp(x, y) <= 4 { format!("{n}-ulp") } else { n.to_string() };
            d.push((cat, format!("{n}: ours {x:#010x} ref {y:#010x}")));
        }
    }
    let mut nq = 0;
    let mut first = None;
    let mut maxulp = 0;
    for (q, (x, y)) in a.data.chunks(16).zip(b.data.chunks(16)).enumerate() {
        if x != y {
            nq += 1;
            first.get_or_insert(q);
            for l in 0..4 {
                let u = u32::from_le_bytes(x[l * 4..l * 4 + 4].try_into().unwrap());
                let v = u32::from_le_bytes(y[l * 4..l * 4 + 4].try_into().unwrap());
                if u != v {
                    maxulp = maxulp.max(ulp(u, v));
                }
            }
        }
    }
    if nq > 0 {
        d.push((
            if maxulp <= 4 { "mem-ulp" } else { "mem" }.into(),
            format!("data: {nq} qwords differ, first {:#x}, max ulp {maxulp}", first.unwrap()),
        ));
    }
    if a.pairs != b.pairs {
        d.push(("pairs".into(), format!("pairs: ours {} ref {}", a.pairs, b.pairs)));
    }
    d
}

struct Rng(u64, bool);
impl Rng {
    fn next(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 16) as u32
    }
    /// A mix of small integers, ordinary floats, extreme floats (overflow /
    /// underflow / denormal / exponent-255 inputs) and qword addresses.
    fn word(&mut self) -> u32 {
        if self.1 {
            // "Sane" data: no exponent-255 / overflow-prone values.
            const F: [f32; 10] = [1.0, 0.5, -2.0, 3.25, 100.0, -0.125, 7.0, 0.0, -0.3, 1e-30];
            return match self.next() % 5 {
                0 => self.next() % 8,
                1 | 4 => F[(self.next() % 10) as usize].to_bits(),
                2 => (self.next() % 64) | 0x8000,
                _ => self.next() % 0x400,
            };
        }
        const F: [f32; 12] = [1.0, 0.5, -2.0, 3.25, 100.0, -0.125, 7.0, 0.0, 1e30, -3e38, 1e-30, -0.3];
        match self.next() % 6 {
            0 => self.next() % 8,
            1 | 4 => F[(self.next() % 12) as usize].to_bits(),
            2 => (self.next() % 64) | 0x8000,
            3 => self.next() % 0x400,
            _ => match self.next() % 4 {
                0 => self.next() & 0x807f_ffff,      // denormal
                1 => self.next() | 0x7f80_0000,      // exponent 255
                _ => (self.next() & 0x8fff_ffff) + 0x3000_0000, // random normal
            },
        }
    }
}

fn vutext_section(elf: &[u8]) -> &[u8] {
    let rd16 = |o: usize| u16::from_le_bytes([elf[o], elf[o + 1]]) as usize;
    let rd32 = |o: usize| u32::from_le_bytes(elf[o..o + 4].try_into().unwrap()) as usize;
    let shoff = rd32(0x20);
    let strtab = rd32(shoff + rd16(0x32) * 40 + 16);
    for i in 0..rd16(0x30) {
        let sh = shoff + i * 40;
        let n = strtab + rd32(sh);
        if elf[n..].starts_with(b".vutext\0") {
            let (off, size) = (rd32(sh + 16), rd32(sh + 20));
            return &elf[off..off + size];
        }
    }
    panic!("no .vutext");
}

/// Synthetic records: every program of `.vutext`, from each entry point
/// (load address and each pair after an E-bit delay slot), with `seeds`
/// random states each. Outputs are filled by replaying our translation.
fn synth(out: &str, seeds: u64, sane: bool) {
    let elf = std::fs::read("orig/SLUS_207.72").expect("orig/SLUS_207.72");
    let words: Vec<u32> = vutext_section(&elf)
        .chunks_exact(4)
        .map(|c| u32::from_le_bytes(c.try_into().unwrap()))
        .collect();
    let mut buf = Vec::new();
    let mut n = 0;
    for s in ssx3_ps2::vu::recomp::extract_mpg_segments(&words) {
        let hash = ssx3_ps2::vu::program_hash(s.addr, &s.bytes);
        let unit = if hash == 0x2764_9fa5_5223_cbbf { 0 } else { 1 };
        let mut entries = vec![s.addr];
        for (i, c) in s.bytes.chunks_exact(8).enumerate() {
            let hi = u32::from_le_bytes(c[4..8].try_into().unwrap());
            if hi & 0x4000_0000 != 0 && (i + 2) * 8 < s.bytes.len() {
                entries.push(s.addr + (i as u32 + 2) * 8);
            }
        }
        for (k, &pc) in entries.iter().enumerate() {
            for seed in 0..seeds {
                let mut r = Rng((hash ^ (k as u64) << 20 ^ seed.wrapping_mul(0x9e37_79b9)) | 1, sane);
                let mut vu = Vu::new(unit);
                vu.write_code(s.addr, &s.bytes);
                for i in 0..vu.data.len() / 4 {
                    let w = r.word();
                    vu.data[i * 4..i * 4 + 4].copy_from_slice(&w.to_le_bytes());
                }
                for i in 1..32 {
                    for l in 0..4 {
                        vu.vf[i][l] = r.word();
                    }
                }
                for l in 0..4 {
                    vu.acc[l] = r.word();
                }
                for i in 1..16 {
                    vu.vi[i] = (r.next() % 0x100) as u16;
                }
                vu.vi[14] = (entries[(r.next() as usize) % entries.len()] / 8) as u16;
                vu.vi[15] = (entries[(r.next() as usize) % entries.len()] / 8) as u16;
                vu.i = r.word();
                vu.q = r.word();
                vu.r = 0x3f80_0000 | (r.next() & 0x7f_ffff);
                let rec = Record {
                    unit: unit as u32,
                    pc,
                    top: 0x100,
                    itop: 0x20,
                    frame: 0,
                    seq: seed,
                    hash,
                    segments: vu.segments.iter().map(|s| (s.start, s.end)).collect(),
                    start: ArchState::of(&vu),
                    code: vu.code.clone(),
                    data_in: vu.data.clone(),
                    ..Default::default()
                };
                let (o, _) = replay(&rec);
                let rec = Record { kicks: o.kicks, end: o.end, data_out: o.data, pairs: o.pairs, ..rec };
                rec.write(&mut buf);
                n += 1;
            }
        }
    }
    std::fs::write(out, &buf).expect("write");
    eprintln!("synth: {n} records -> {out}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(String::as_str).unwrap_or("");
    if cmd == "synth" {
        return synth(
            &args[2],
            args.get(3).and_then(|s| s.parse().ok()).unwrap_or(4),
            args.iter().any(|a| a == "--sane"),
        );
    }
    let recs = load(&args[2]).expect("capture");
    match cmd {
        "kickhaz" => {
            // XGKICK hazard scan: on hardware (and PCSX2) the GIF reads the
            // packet while the program keeps running; we snapshot it at the
            // XGKICK. Report kicked qwords the same program rewrites later,
            // with the pair distance from the kick.
            use ssx3_ps2::vu::decode::{decode_lower, Lower};
            let mut hits = 0;
            for (i, r) in recs.iter().enumerate() {
                let mut vu = r.setup();
                vu.budget = 4_000_000;
                // (pair index of kick, qword range) and the data at kick time.
                let mut kicks: Vec<(u64, u32, u32, Vec<u8>)> = Vec::new();
                let mut first_write: Vec<Option<u64>> = Vec::new();
                let mut n = 0u64;
                let mut prev = vu.data.clone();
                let mask = vu.code.len() as u32 - 1;
                let nq = (vu.data.len() / 16) as u32;
                reference::run_traced(&mut vu, r.pc, &mut |_| {}, &mut |v: &Vu, pc| {
                    n += 1;
                    // Writes since the previous pair into kicked ranges.
                    if v.data != prev {
                        for (k, (_, a, len, _)) in kicks.iter().enumerate() {
                            if first_write[k].is_some() {
                                continue;
                            }
                            for q in 0..*len {
                                let o = (((a + q) % nq) * 16) as usize;
                                if v.data[o..o + 16] != prev[o..o + 16] {
                                    first_write[k] = Some(n);
                                    break;
                                }
                            }
                        }
                        prev.copy_from_slice(&v.data);
                    }
                    let pc = pc & mask;
                    let w = |o: u32| u32::from_le_bytes(v.code[o as usize..o as usize + 4].try_into().unwrap());
                    let (lo, hi) = (w(pc), w(pc + 4));
                    if hi & 0x8000_0000 == 0 {
                        if let Lower::Xgkick { is } = decode_lower(lo) {
                            let a = v.vi[is as usize & 15] as u32 % nq;
                            let len = v.gif_packet(a).len() as u32;
                            kicks.push((n, a, len, Vec::new()));
                            first_write.push(None);
                        }
                    }
                });
                for (k, (kn, a, len, _)) in kicks.iter().enumerate() {
                    if let Some(wn) = first_write[k] {
                        hits += 1;
                        println!(
                            "#{i} {:016x} pc {:#06x}: kick at pair {kn} of qw {a:#x}+{len} rewritten {} pairs later",
                            r.hash,
                            r.pc,
                            wn - kn
                        );
                    }
                }
            }
            println!("{hits} kicked packets rewritten later in the same program");
        }
        "cover" => {
            // Executed pairs per program over all records (reference tracer).
            let mut hit: BTreeMap<u64, std::collections::BTreeSet<u32>> = BTreeMap::new();
            for r in &recs {
                let mut vu = r.setup();
                vu.budget = 4_000_000;
                let set = hit.entry(r.hash).or_default();
                reference::run_traced(&mut vu, r.pc, &mut |_| {}, &mut |_, pc| {
                    set.insert(pc);
                });
            }
            for e in ssx3_ps2::vu::generated::PROGRAMS {
                let n = hit
                    .get(&e.hash)
                    .map_or(0, |s| s.iter().filter(|&&pc| pc >= e.start && pc < e.start + e.len).count());
                println!("{:016x}: {n} / {} pairs executed", e.hash, e.len / 8);
            }
        }
        "find" => {
            // Records whose XGKICK packets contain a 32-bit word (hex),
            // e.g. a Q value seen in a GS trace (`gs_replay --packets`).
            let w = u32::from_str_radix(args[3].trim_start_matches("0x"), 16).expect("hex word");
            for (i, r) in recs.iter().enumerate() {
                for (k, p) in r.kicks.iter().enumerate() {
                    for (j, q) in p.iter().enumerate() {
                        if (0..4).any(|l| (*q >> (32 * l)) as u32 == w) {
                            println!("#{i} vu{} pc {:#06x} hash {:016x} kick {k} qword {j}", r.unit, r.pc, r.hash);
                        }
                    }
                }
            }
        }
        "list" => {
            for (i, r) in recs.iter().enumerate() {
                println!(
                    "{i:4} vu{} frame {:5} pc {:#06x} hash {:016x} top {:#x} itop {:#x} pairs {} kicks {} errors {}",
                    r.unit,
                    r.frame,
                    r.pc,
                    r.hash,
                    r.top,
                    r.itop,
                    r.pairs,
                    r.kicks.len(),
                    r.errors
                );
            }
        }
        "self" => {
            let mut bad = 0;
            for (i, r) in recs.iter().enumerate() {
                let (o, _) = replay(r);
                let rec = Out {
                    kicks: r.kicks.clone(),
                    end: r.end.clone(),
                    data: r.data_out.clone(),
                    pairs: r.pairs,
                };
                let d = compare(&o, &rec, &r.start);
                if !d.is_empty() {
                    bad += 1;
                    println!("#{i}: {d:?}");
                }
            }
            println!("{} records, {bad} differ from the in-game result", recs.len());
        }
        "cmp" => {
            let refs = read_ref(&args[3]);
            let verbose = args.iter().any(|a| a == "-v");
            let mut cats: BTreeMap<String, Vec<usize>> = BTreeMap::new();
            let mut per_prog: BTreeMap<(u32, u64, u32), (usize, usize)> = BTreeMap::new();
            let mut clean = 0;
            for (i, r) in recs.iter().enumerate() {
                let Some(b) = refs.get(&(i as u32)) else { continue };
                let (o, _) = replay(r);
                let d = compare(&o, b, &r.start);
                let e = per_prog.entry((r.unit, r.hash, r.pc)).or_default();
                e.0 += 1;
                if d.is_empty() {
                    clean += 1;
                    continue;
                }
                e.1 += 1;
                let mut seen = std::collections::BTreeSet::new();
                for (c, _) in &d {
                    if seen.insert(c.clone()) {
                        cats.entry(c.clone()).or_default().push(i);
                    }
                }
                if verbose {
                    println!("#{i} vu{} pc {:#06x} hash {:016x} frame {}:", r.unit, r.pc, r.hash, r.frame);
                    for (c, s) in d.iter().take(24) {
                        println!("    [{c}] {s}");
                    }
                }
            }
            println!("{} records compared, {clean} identical", refs.len());
            for (c, v) in &cats {
                println!("  {c:12} {:4} records, e.g. {:?}", v.len(), &v[..v.len().min(8)]);
            }
            println!("per program (unit, hash, pc): records / differing");
            for ((u, h, pc), (n, bad)) in &per_prog {
                println!("  vu{u} {h:016x} {pc:#06x}: {n} / {bad}");
            }
        }
        "trace" => {
            let idx: usize = args[3].parse().unwrap();
            let r = &recs[idx];
            let mut vu = r.setup();
            vu.instant_qp = ref_model();
            vu.budget = 4_000_000;
            let pc = r.pc;
            reference::run_traced(&mut vu, pc, &mut |_| {}, &mut |v: &Vu, pc| {
                print!("{pc:04x} vi");
                for k in 1..16 {
                    print!(" {:04x}", v.vi[k]);
                }
                println!(
                    " q {:08x} p {:08x} mac {:04x} st {:03x} clip {:06x}",
                    v.q, v.p, v.mac, v.status, v.clip
                );
                for k in 1..32 {
                    let f = v.vf[k];
                    println!("  vf{k:02} {:08x} {:08x} {:08x} {:08x}", f[0], f[1], f[2], f[3]);
                }
                let a = v.acc;
                println!("  acc  {:08x} {:08x} {:08x} {:08x}", a[0], a[1], a[2], a[3]);
            });
        }
        _ => eprintln!("usage: vu_diff list|self|cmp|trace|find|cover|kickhaz <capture> ..."),
    }
}
