//! Phase 3 rendering, stage 2A check (`SSX3_VERIFY_BEZIER=1`): the native
//! Bezier terrain VU1 programs (`ssx3_render::mat1`, type 1, translated
//! `f4babac52752998a`; `ssx3_render::mat2`, type 2, `9790369fcc84762b`)
//! must produce exactly the XGKICK packets and the data memory of the
//! translated microprograms.
//!
//! As `mat3_verify`: on every VIF1 microprogram start in one of the two
//! programs, [`begin`] copies VU1 data memory; the translated program runs
//! as usual while its kicks are collected ([`Pending::kick`]); [`end`] runs
//! the native program on the copy and compares the kick sequence and the
//! whole data memory. The native side starts every call from a fresh
//! `VuState`, so a VU register carried between calls that the native code
//! does not model would show up as a mismatch.
//!
//! Counts per (program, entry, variant) are printed at exit ([`summary`]),
//! with the number of calls that kicked clipped fans. The variant is what
//! selects the code path inside an entry: for type 1 the grid size n (VU
//! qw39.x after the call), for type 2 the per-edge LODs ([`mat2::variant`]).
//!
//! `SSX3_VERIFY_BEZIER_DUMP=<path>` additionally writes a sample of calls
//! (input memory, translated kicks and output memory) for offline replay
//! by the `ssx3-render` tests (`BEZIER_DUMP`). Emulation itself is unchanged.

use std::collections::BTreeMap;
use std::io::Write;
use std::sync::Mutex;

use ssx3_render::{mat1, mat2};

use crate::vu::Vu;
use crate::Qword;

pub fn enabled() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("SSX3_VERIFY_BEZIER").is_some())
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Program {
    Type1,
    Type2,
}

#[derive(Default, Clone, Copy)]
struct Counts {
    ok: u64,
    mismatch: u64,
    not_implemented: u64,
    /// Calls that kicked at least one clipped fan (TRIFAN packet).
    with_fans: u64,
    /// Clipped fans kicked in total.
    fans: u64,
}

#[derive(Default)]
struct State {
    /// Keyed by (program, entry, variant).
    counts: BTreeMap<(Program, u32, u32), Counts>,
    reported: u32,
    dump: Option<std::fs::File>,
    dumped: BTreeMap<(Program, u32), u64>,
    dump_records: u64,
}

static STATE: Mutex<Option<State>> = Mutex::new(None);

/// One call in flight: the input copy and the translated program's kicks.
pub struct Pending {
    program: Program,
    entry: u32,
    top: u32,
    itop: u32,
    mem: Box<[u8; 16384]>,
    kicks: Vec<Vec<Qword>>,
}

impl Pending {
    pub fn kick(&mut self, p: &[Qword]) {
        self.kicks.push(p.to_vec());
    }
}

/// Before the translated program runs (VIF1 only).
pub fn begin(vu: &mut Vu, pc: u32, top: u32, itop: u32) -> Option<Pending> {
    if !enabled() {
        return None;
    }
    let program = match vu.segment_hash_pub(pc) {
        mat1::HASH => Program::Type1,
        mat2::HASH => Program::Type2,
        _ => return None,
    };
    Some(Pending {
        program,
        entry: pc,
        top,
        itop,
        mem: vu.data[..].try_into().map(Box::new).ok()?,
        kicks: Vec::new(),
    })
}

/// A TRIFAN packet: the GIF tag's PRIM (PRE set) has primitive type 5.
fn is_fan(p: &[Qword]) -> bool {
    p.first().is_some_and(|t| (t >> 46) & 1 == 1 && (t >> 47) & 7 == 5)
}

fn write_record(f: &mut std::fs::File, p: &Pending, input: &[u8], out: &[u8]) {
    let mut b = Vec::with_capacity(40_000);
    b.extend_from_slice(&p.entry.to_le_bytes());
    b.extend_from_slice(&p.top.to_le_bytes());
    b.extend_from_slice(&p.itop.to_le_bytes());
    b.extend_from_slice(&(p.program as u32).to_le_bytes());
    b.extend_from_slice(input);
    b.extend_from_slice(out);
    b.extend_from_slice(&(p.kicks.len() as u32).to_le_bytes());
    for k in &p.kicks {
        b.extend_from_slice(&(k.len() as u32).to_le_bytes());
        for q in k {
            b.extend_from_slice(&q.to_le_bytes());
        }
    }
    let _ = f.write_all(&b);
}

/// After the translated program finished: run the native one and compare.
pub fn end(mut p: Pending, vu: &Vu) {
    let input = p.mem.clone();
    let variant = match p.program {
        Program::Type1 => u32::from_le_bytes(vu.data[39 * 16..39 * 16 + 4].try_into().unwrap()),
        Program::Type2 => mat2::variant(p.entry, p.top, &input),
    };
    let mut native: Vec<Vec<Qword>> = Vec::new();
    let implemented = match p.program {
        Program::Type1 => {
            let mut regs = mat1::VuState::default();
            mat1::run(p.entry, p.top, p.itop, &mut p.mem, &mut regs, &mut |k| native.push(k.to_vec()))
        }
        Program::Type2 => {
            let mut regs = mat2::VuState::default();
            mat2::run(p.entry, p.top, p.itop, &mut p.mem, &mut regs, &mut |k| native.push(k.to_vec()))
        }
    };
    let mut g = STATE.lock().unwrap();
    let s = g.get_or_insert_with(|| State {
        dump: std::env::var("SSX3_VERIFY_BEZIER_DUMP").ok().and_then(|path| std::fs::File::create(path).ok()),
        ..State::default()
    });
    if let (Some(f), true) = (s.dump.as_mut(), s.dump_records < 6000) {
        let n = s.dumped.entry((p.program, p.entry)).or_default();
        *n += 1;
        if *n <= 40 || n.is_multiple_of(97) {
            s.dump_records += 1;
            write_record(f, &p, &input[..], &vu.data[..]);
        }
    }
    let c = s.counts.entry((p.program, p.entry, variant)).or_default();
    let fans = p.kicks.iter().filter(|k| is_fan(k)).count() as u64;
    if fans > 0 {
        c.with_fans += 1;
        c.fans += fans;
    }
    if !implemented {
        c.not_implemented += 1;
        return;
    }
    let kicks_ok = native == p.kicks;
    let mem_ok = p.mem[..] == vu.data[..];
    if kicks_ok && mem_ok {
        c.ok += 1;
        return;
    }
    c.mismatch += 1;
    if s.reported >= 10 {
        return;
    }
    s.reported += 1;
    let frame = crate::vu::capture::frame();
    eprintln!(
        "[bezier-verify] mismatch #{}: type {} entry {:#06x} variant {variant:#x} top {:#05x} itop {:#x} frame {frame}: kicks native {} translated {}",
        s.reported,
        p.program as u32 + 1,
        p.entry,
        p.top,
        p.itop,
        native.len(),
        p.kicks.len()
    );
    if let Some(i) = (0..native.len().max(p.kicks.len())).find(|&i| native.get(i) != p.kicks.get(i)) {
        let (a, b) = (native.get(i), p.kicks.get(i));
        eprintln!(
            "  kick {i}: native {} qw, translated {} qw",
            a.map_or(0, |k| k.len()),
            b.map_or(0, |k| k.len())
        );
        if let (Some(a), Some(b)) = (a, b) {
            for (j, (x, y)) in a.iter().zip(b).enumerate().filter(|(_, (x, y))| x != y).take(6) {
                eprintln!("    qw {j}: native {x:032x} translated {y:032x}");
            }
        }
    }
    let diff: Vec<usize> = (0..1024).filter(|q| p.mem[q * 16..q * 16 + 16] != vu.data[q * 16..q * 16 + 16]).collect();
    if !diff.is_empty() {
        eprintln!("  data memory: {} qwords differ, first at {:?}", diff.len(), &diff[..diff.len().min(8)]);
    }
}

/// Counts per entry, for the exit report (None when the check is off).
pub fn summary() -> Option<String> {
    let g = STATE.lock().unwrap();
    let s = g.as_ref()?;
    let mut out = String::from(
        "[bezier-verify] native Bezier types 1/2 vs translated f4babac5 / 9790369f (type entry variant: ok, mismatch, not implemented):",
    );
    let mut total = Counts::default();
    for (&(prog, entry, variant), c) in &s.counts {
        let what = match prog {
            Program::Type1 => format!("type 1 {entry:#06x} n={variant}"),
            Program::Type2 => format!("type 2 {entry:#06x} {}", mat2::describe_variant(variant)),
        };
        out += &format!(
            "\n  {what}: ok {} mismatch {} not-implemented {} (calls with clipped fans: {}, fans: {})",
            c.ok, c.mismatch, c.not_implemented, c.with_fans, c.fans
        );
        total.ok += c.ok;
        total.mismatch += c.mismatch;
        total.not_implemented += c.not_implemented;
        total.with_fans += c.with_fans;
    }
    out += &format!(
        "\n  total: ok {} mismatch {} not-implemented {} (calls with clipped fans: {})",
        total.ok, total.mismatch, total.not_implemented, total.with_fans
    );
    Some(out)
}
