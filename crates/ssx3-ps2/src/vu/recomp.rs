//! Static VU microcode recompiler: one uploaded code segment -> one Rust
//! function over basic blocks that calls the `Vu` semantic library.
//!
//! Blocks start at *leaders*: the load address, branch targets, the pair
//! after every branch's delay slot (fall-through / return address) and the
//! pair after every E-bit delay slot (the next program's entry). A block runs
//! until a branch (its delay slot is emitted inline, then control transfers),
//! an E bit (delay slot inline, then `end_program`), or the next leader.
//! Any pc the function does not cover returns `VuExit::Jump(pc)` so the
//! runtime can re-dispatch (e.g. a JR into another upload).

use super::decode::{decode_lower, decode_macro, decode_upper, pair_reads, Flow, Fmac, Lower, Macro, Upper};
use super::exec::{lower_call, upper_call};
use super::{program_hash, Src};
use std::collections::BTreeSet;
use std::fmt::Write;

/// Code-address mask used for branch targets (VU1: 16 KiB).
const CODE_MASK: u32 = 0x3fff;

/// Result of translating one segment.
pub struct Translation {
    pub hash: u64,
    pub start: u32,
    pub len: u32,
    /// Module name (`vu_<hash>`).
    pub module: String,
    pub source: String,
    pub leaders: Vec<u32>,
    /// (pc, word) of every undecodable upper/lower word.
    pub unknown: Vec<(u32, u32)>,
    /// Pairs whose delay slot holds a branch or E bit (not supported).
    pub unsupported: Vec<u32>,
}

#[derive(Clone, Copy)]
struct Pair {
    pc: u32,
    lo: u32,
    hi: u32,
    upper: Upper,
    lower: Option<Lower>,
}

impl Pair {
    fn ebit(&self) -> bool {
        self.hi & 0x4000_0000 != 0
    }
    fn flow(&self) -> Flow {
        self.lower.map(|l| l.flow()).unwrap_or(Flow::None)
    }
}

fn decode_pairs(code: &[u8], base: u32) -> Vec<Pair> {
    code.chunks_exact(8)
        .enumerate()
        .map(|(i, c)| {
            let lo = u32::from_le_bytes(c[0..4].try_into().unwrap());
            let hi = u32::from_le_bytes(c[4..8].try_into().unwrap());
            let lower = if hi & 0x8000_0000 != 0 {
                None
            } else {
                Some(decode_lower(lo))
            };
            Pair {
                pc: base + i as u32 * 8,
                lo,
                hi,
                upper: decode_upper(hi),
                lower,
            }
        })
        .collect()
}

fn rel_target(pc: u32, off: i16) -> u32 {
    (pc as i32 + 8 + off as i32 * 8) as u32 & CODE_MASK
}

/// Translate `code` loaded at `base_pc` and return just the Rust source.
pub fn translate_program(code: &[u8], base_pc: u32) -> String {
    translate(code, base_pc).source
}

/// Successor of a pair in the translator's control-flow graph.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Node {
    /// Pair index.
    P(usize),
    /// The program ends (after an E-bit delay slot): pipelines drain.
    End,
    /// Unknown: JR/JALR, or a pc outside the segment.
    Dyn,
}

/// What a set of pairs within a window can observe.
#[derive(Clone, Copy, Default)]
struct Look {
    /// VF fields read (bit `reg * 4 + lane`).
    reads: u128,
    /// A flag op (FM*, FS*, FC*).
    flags: bool,
    /// A program end or a dynamic jump.
    end: bool,
    dyn_: bool,
}

impl Look {
    fn or(&mut self, o: &Look) {
        self.reads |= o.reads;
        self.flags |= o.flags;
        self.end |= o.end;
        self.dyn_ |= o.dyn_;
    }
}

/// Field set of VF `reg` with dest mask `m` (bit 3 = x).
fn fields(reg: u8, m: u8) -> u128 {
    if reg == 0 {
        return 0;
    }
    let mut f = 0u128;
    for l in 0..4 {
        if m & (8 >> l) != 0 {
            f |= 1u128 << (reg as u32 * 4 + l);
        }
    }
    f
}

/// Dest mask of VF `reg` within the field set `f`.
fn mask_of(f: u128, reg: u8) -> u8 {
    let mut m = 0;
    for l in 0..4 {
        if f & (1u128 << (reg as u32 * 4 + l)) != 0 {
            m |= 8 >> l;
        }
    }
    m
}

/// The VF register an upper op stages a result for (a lower write to the
/// same register is discarded), with its dest mask. vf0 included.
fn upper_target(u: &Upper) -> Option<(u8, u8)> {
    match *u {
        Upper::Fmac { acc: false, dest, fd, .. } => Some((fd, dest)),
        Upper::Opmsub { fd, .. } => Some((fd, 0b1110)),
        Upper::Itof { dest, ft, .. } | Upper::Ftoi { dest, ft, .. } | Upper::Abs { dest, ft, .. } => {
            Some((ft, dest))
        }
        _ => None,
    }
}

/// VF register and dest mask a lower op writes.
fn lower_target(l: &Lower) -> Option<(u8, u8)> {
    match *l {
        Lower::Lq { dest, ft, .. }
        | Lower::Lqi { dest, ft, .. }
        | Lower::Lqd { dest, ft, .. }
        | Lower::Move { dest, ft, .. }
        | Lower::Mr32 { dest, ft, .. }
        | Lower::Mfir { dest, ft, .. }
        | Lower::Rnext { dest, ft }
        | Lower::Rget { dest, ft }
        | Lower::Mfp { dest, ft } => Some((ft, dest)),
        _ => None,
    }
}

fn flag_reader(l: &Lower) -> bool {
    matches!(
        l,
        Lower::Fmeq { .. }
            | Lower::Fmand { .. }
            | Lower::Fmor { .. }
            | Lower::Fseq { .. }
            | Lower::Fsand { .. }
            | Lower::Fsor { .. }
            | Lower::Fsset { .. }
            | Lower::Fceq { .. }
            | Lower::Fcand { .. }
            | Lower::Fcor { .. }
            | Lower::Fcget { .. }
            | Lower::Fcset { .. }
    )
}

/// Per-pair translate-time facts (see `fast.rs` for what they buy).
#[derive(Clone, Copy, Default)]
struct Facts {
    /// Fields read (stall model).
    reads: u128,
    /// Fields the upper op writes (fd != 0) and the lower op writes (when
    /// not discarded by the upper).
    wu: u128,
    wl: u128,
    /// Fields that may have been written by the 3 preceding pairs along
    /// static edges: the only ones a fast-path read must check.
    recent: u128,
    /// What the 3 following pairs can observe.
    ahead: Look,
}

pub fn translate(code: &[u8], base_pc: u32) -> Translation {
    let pairs = decode_pairs(code, base_pc);
    let n = pairs.len();
    let end = base_pc + n as u32 * 8;
    let in_range = |pc: u32| pc >= base_pc && pc < end;
    let idx = |pc: u32| ((pc - base_pc) / 8) as usize;
    let at = |pc: u32| &pairs[idx(pc)];

    let mut leaders = BTreeSet::new();
    leaders.insert(base_pc);
    let mut unknown = Vec::new();
    let mut unsupported = Vec::new();
    for p in &pairs {
        if let Upper::Unknown(w) = p.upper {
            unknown.push((p.pc, w));
        }
        if let Some(Lower::Unknown(w)) = p.lower {
            unknown.push((p.pc, w));
        }
        let flow = p.flow();
        match flow {
            Flow::Jump { off } | Flow::Cond { off } => {
                leaders.insert(rel_target(p.pc, off));
            }
            _ => {}
        }
        if flow != Flow::None || p.ebit() {
            leaders.insert(p.pc + 16);
            if in_range(p.pc + 8) {
                let d = at(p.pc + 8);
                if d.flow() != Flow::None || (flow != Flow::None && d.ebit()) {
                    unsupported.push(p.pc);
                }
            }
        }
    }
    let leaders: Vec<u32> = leaders.into_iter().filter(|&pc| in_range(pc)).collect();
    let leader_set: BTreeSet<u32> = leaders.iter().copied().collect();

    // ---- control-flow graph (conservative: union of every way a pair can
    // be left, whether it runs as a delay slot or not) ----
    let node = |pc: u32| if in_range(pc) { Node::P(idx(pc)) } else { Node::Dyn };
    let succ: Vec<Vec<Node>> = (0..n)
        .map(|i| {
            let p = &pairs[i];
            let mut s = vec![if i + 1 < n {
                Node::P(i + 1)
            } else if p.ebit() {
                Node::End
            } else {
                Node::Dyn
            }];
            if i >= 1 {
                let b = &pairs[i - 1];
                if b.ebit() {
                    s.push(Node::End);
                }
                match b.flow() {
                    Flow::Jump { off } | Flow::Cond { off } => s.push(node(rel_target(b.pc, off))),
                    Flow::Indirect => s.push(Node::Dyn),
                    Flow::None => {}
                }
            }
            s.dedup();
            s
        })
        .collect();
    let mut preds: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (i, s) in succ.iter().enumerate() {
        for x in s {
            if let Node::P(j) = *x {
                if !preds[j].contains(&i) {
                    preds[j].push(i);
                }
            }
        }
    }

    let mut facts: Vec<Facts> = pairs
        .iter()
        .map(|p| {
            let mut reads = 0u128;
            for (r, m) in pair_reads(&p.upper, p.lower.as_ref()) {
                reads |= fields(r, m);
            }
            let ut = upper_target(&p.upper);
            let wu = ut.map_or(0, |(r, m)| fields(r, m));
            let wl = match p.lower.as_ref().and_then(lower_target) {
                Some((r, m)) if ut.map(|t| t.0) != Some(r) => fields(r, m),
                _ => 0,
            };
            Facts {
                reads,
                wu,
                wl,
                ..Default::default()
            }
        })
        .collect();

    // Backward window: fields written by the 1, 2, 3 preceding pairs.
    let w: Vec<u128> = facts.iter().map(|f| f.wu | f.wl).collect();
    let a1: Vec<u128> = (0..n).map(|j| preds[j].iter().fold(0, |a, &i| a | w[i])).collect();
    let a2: Vec<u128> = (0..n).map(|j| preds[j].iter().fold(a1[j], |a, &i| a | a1[i])).collect();
    let a3: Vec<u128> = (0..n).map(|j| preds[j].iter().fold(a1[j], |a, &i| a | a2[i])).collect();
    // Forward window: what the 1, 2, 3 following pairs observe.
    let look_of = |x: &Node| -> Look {
        match *x {
            Node::P(j) => Look {
                reads: facts[j].reads,
                flags: pairs[j].lower.as_ref().is_some_and(flag_reader),
                end: false,
                dyn_: false,
            },
            Node::End => Look {
                end: true,
                ..Default::default()
            },
            Node::Dyn => Look {
                dyn_: true,
                ..Default::default()
            },
        }
    };
    let f1: Vec<Look> = (0..n)
        .map(|i| {
            let mut l = Look::default();
            for x in &succ[i] {
                l.or(&look_of(x));
            }
            l
        })
        .collect();
    let step = |prev: &Vec<Look>| -> Vec<Look> {
        (0..n)
            .map(|i| {
                let mut l = f1[i];
                for x in &succ[i] {
                    if let Node::P(j) = *x {
                        l.or(&prev[j]);
                    }
                }
                l
            })
            .collect()
    };
    let f2 = step(&f1);
    let f3 = step(&f2);
    for i in 0..n {
        facts[i].recent = a3[i];
        facts[i].ahead = f3[i];
    }

    let hash = program_hash(base_pc, code);
    let module = format!("vu_{hash:016x}");
    let mut s = String::new();
    writeln!(
        s,
        "//! Recompiled VU microcode: {} bytes at {base_pc:#06x}, hash {hash:#018x}.",
        code.len()
    )
    .unwrap();
    writeln!(s, "//! Generated by `vu_recomp`; do not edit. Helpers: `vu/fast.rs`.").unwrap();
    writeln!(
        s,
        "#![allow(unused_imports, unused_variables, unused_mut, unreachable_code, unused_parens, unused_assignments, clippy::all)]"
    )
    .unwrap();
    writeln!(s, "use crate::vu::decode::Efu;").unwrap();
    writeln!(s, "use crate::vu::{{fast, float, Vu, VuExit}};").unwrap();
    writeln!(s, "use crate::Qword;").unwrap();
    writeln!(s).unwrap();
    writeln!(s, "pub const HASH: u64 = {hash:#018x};").unwrap();
    writeln!(s, "pub const START: u32 = {base_pc:#x};").unwrap();
    writeln!(s, "pub const LEN: u32 = {:#x};", code.len()).unwrap();
    write!(s, "pub static LEADERS: &[u32] = &[").unwrap();
    for (i, l) in leaders.iter().enumerate() {
        if i % 12 == 0 {
            write!(s, "\n    ").unwrap();
        }
        write!(s, "{l:#06x}, ").unwrap();
    }
    writeln!(s, "\n];\n").unwrap();
    writeln!(
        s,
        "/// Fast blocks run from leaders reached by static control flow. After any"
    )
    .unwrap();
    writeln!(
        s,
        "/// other entry (program start, budget resume, JR) `slow` single-pair arms"
    )
    .unwrap();
    writeln!(s, "/// that check every VF read for stalls run first.").unwrap();
    writeln!(
        s,
        "pub fn run(vu: &mut Vu, mut pc: u32, kick: &mut dyn FnMut(&[Qword])) -> VuExit {{"
    )
    .unwrap();
    writeln!(s, "    let mut slow: u32 = 3;").unwrap();
    writeln!(s, "    loop {{").unwrap();
    writeln!(s, "        if slow == 0 {{").unwrap();
    writeln!(s, "            match pc {{").unwrap();
    let em = Emitter {
        pairs: &pairs,
        facts: &facts,
        base: base_pc,
    };
    for &leader in &leaders {
        let (body, count, tail) = em.arm(leader, false, &leader_set);
        writeln!(s, "            {leader:#06x} => {{").unwrap();
        writeln!(
            s,
            "                if vu.out_of_budget({count}) {{\n                    return VuExit::Budget({leader:#06x});\n                }}"
        )
        .unwrap();
        writeln!(s, "                let pb = vu.pairs;\n                vu.pairs = pb + {count};").unwrap();
        s.push_str(&body);
        writeln!(s, "                {tail}").unwrap();
        writeln!(s, "            }}").unwrap();
    }
    writeln!(s, "            _ => {{}}").unwrap();
    writeln!(s, "            }}").unwrap();
    writeln!(s, "        }} else {{").unwrap();
    writeln!(s, "            slow -= 1;").unwrap();
    writeln!(s, "        }}").unwrap();
    writeln!(s, "        // Single pairs (plus delay slot), every read checked.").unwrap();
    writeln!(s, "        match pc {{").unwrap();
    for p in &pairs {
        let (body, count, tail) = em.arm(p.pc, true, &leader_set);
        writeln!(s, "            {:#06x} => {{", p.pc).unwrap();
        writeln!(
            s,
            "                if vu.out_of_budget({count}) {{\n                    return VuExit::Budget({:#06x});\n                }}",
            p.pc
        )
        .unwrap();
        writeln!(s, "                let pb = vu.pairs;\n                vu.pairs = pb + {count};").unwrap();
        s.push_str(&body);
        writeln!(s, "                {tail}").unwrap();
        writeln!(s, "            }}").unwrap();
    }
    writeln!(s, "            _ => return VuExit::Jump(pc),").unwrap();
    writeln!(s, "        }}").unwrap();
    writeln!(s, "    }}").unwrap();
    writeln!(s, "}}").unwrap();

    Translation {
        hash,
        start: base_pc,
        len: code.len() as u32,
        module,
        source: s,
        leaders,
        unknown,
        unsupported,
    }
}

/// Per-pair `vu.cycle += 1` merged into one add before the next line that
/// reads or moves the cycle counter (stall checks, ready times, pipelines,
/// DIV/EFU ops) and at the end of the arm.
fn defer_cycles(body: &str) -> String {
    const USES: &[&str] = &[
        "vu.cycle", "ready", "flags_push", "clip_commit", "q_sync", "p_sync", "flags_sync", "vu.div(",
        "vu.sqrt(", "vu.rsqrt(", "waitq", "waitp", "efu_", "end_program", "flush",
    ];
    let mut out = String::with_capacity(body.len());
    let mut n = 0u64;
    for line in body.lines() {
        let t = line.trim();
        if t == "vu.cycle += 1;" {
            n += 1;
            continue;
        }
        if n > 0 && !t.starts_with("//") && USES.iter().any(|u| t.contains(u)) {
            out.push_str(IND);
            writeln!(out, "vu.cycle += {n};").unwrap();
            n = 0;
        }
        out.push_str(line);
        out.push('\n');
    }
    if n > 0 {
        out.push_str(IND);
        writeln!(out, "vu.cycle += {n};").unwrap();
    }
    out
}

/// Deferred MAC/status commit within an arm (see `Emitter::pair`).
#[derive(Default)]
struct Pend {
    declared: bool,
    pending: bool,
    /// VF fields (and ACC lanes) known to hold an FMAC or ITOF result in
    /// this arm: never denormal or exponent 255, so `norm` can be skipped.
    known: u128,
    acc: u8,
}

const MAC_FLUSH: &str = "vu.mac_commit(fl, fa); vu.flags_eager(); fa = 0;";

struct Emitter<'a> {
    pairs: &'a [Pair],
    facts: &'a [Facts],
    base: u32,
}

const IND: &str = "                ";

impl Emitter<'_> {
    fn in_range(&self, pc: u32) -> bool {
        pc >= self.base && pc < self.base + self.pairs.len() as u32 * 8
    }
    fn at(&self, pc: u32) -> (&Pair, &Facts) {
        let i = ((pc - self.base) / 8) as usize;
        (&self.pairs[i], &self.facts[i])
    }

    /// One match arm starting at `start`: a whole basic block (`single` =
    /// false) or one pair plus its delay slot. Returns (body, pairs, tail).
    fn arm(&self, start: u32, single: bool, leader_set: &BTreeSet<u32>) -> (String, u64, String) {
        let mut body = String::new();
        let mut count = 0u64;
        let mut pc = start;
        let tail: String;
        // vf0 is (0, 0, 0, 1.0): normal.
        let mut st = Pend {
            known: 0xf,
            ..Default::default()
        };
        loop {
            let (p, _) = self.at(pc);
            let flow = p.flow();
            self.pair(&mut body, pc, count, true, single, &mut st);
            count += 1;
            if p.ebit() {
                let next = pc + 16;
                if self.in_range(pc + 8) {
                    self.pair(&mut body, pc + 8, count, false, single, &mut st);
                    count += 1;
                }
                tail = format!("vu.end_program({next:#06x});\n{IND}return VuExit::End;");
                break;
            }
            if flow != Flow::None {
                if self.in_range(pc + 8) {
                    let (d, _) = self.at(pc + 8);
                    self.pair(&mut body, pc + 8, count, false, single, &mut st);
                    count += 1;
                    if d.ebit() {
                        // E bit in a delay slot: hardware runs the branch
                        // target pair and then stops. Not supported; end here.
                        tail = format!(
                            "vu.unknown_op({:#06x}, 0x4000_0000);\n{IND}vu.end_program({:#06x});\n{IND}return VuExit::End;",
                            pc + 8,
                            pc + 16
                        );
                        break;
                    }
                }
                let fall = pc + 16;
                tail = match flow {
                    Flow::Jump { off } => format!("pc = {:#06x};\n{IND}continue;", rel_target(pc, off)),
                    Flow::Cond { off } => format!(
                        "pc = if taken {{ {:#06x} }} else {{ {fall:#06x} }};\n{IND}continue;",
                        rel_target(pc, off)
                    ),
                    _ => format!("pc = target;\n{IND}slow = 3;\n{IND}continue;"),
                };
                break;
            }
            let next = pc + 8;
            if single || !self.in_range(next) || leader_set.contains(&next) {
                tail = format!("pc = {next:#06x};\n{IND}continue;");
                break;
            }
            pc = next;
        }
        if st.pending {
            body.push_str(IND);
            body.push_str(MAC_FLUSH);
            body.push('\n');
        }
        (defer_cycles(&body), count, tail)
    }

    /// Emit pair `pc`, the `k`-th of its arm (its index is `pb + k`). A
    /// branch's condition / target is computed as `taken` / `target` when
    /// `branch`. `slow` arms check every read for stalls.
    fn pair(&self, out: &mut String, pc: u32, k: u64, branch: bool, slow: bool, st: &mut Pend) {
        let (p, f) = self.at(pc);
        let o = |out: &mut String, line: &str| {
            out.push_str(IND);
            out.push_str(line);
            out.push('\n');
        };
        if !slow {
            o(
                out,
                &format!(
                    "// {:#06x}: {:08x} {:08x}{}",
                    p.pc,
                    p.hi,
                    p.lo,
                    if p.ebit() { "  [E]" } else { "" }
                ),
            );
        }
        // Stall: the max ready time of the fields that can be in flight.
        let check = if slow { f.reads } else { f.reads & f.recent };
        if check != 0 {
            let mut e = String::from("vu.stall(vu.cycle");
            for b in 0..128u32 {
                if check & (1u128 << b) != 0 {
                    write!(e, ".max(vu.vf_ready[{}][{}])", b / 4, b % 4).unwrap();
                }
            }
            e.push_str(");");
            o(out, &e);
        }
        let look = f.ahead;
        let near_exit = look.end || look.dyn_;
        let push = look.flags || look.dyn_;
        // Ready-time bookkeeping for a write of `fl` fields to `reg`.
        let ready = |fl: u128, reg: u8| -> String {
            let need = if near_exit { fl } else { fl & look.reads };
            let m = mask_of(need, reg);
            let mut r = String::new();
            if m != 0 {
                write!(r, " vu.ready::<0b{m:04b}>({reg});").unwrap();
            }
            if near_exit && fl != 0 {
                r.push_str(" vu.ready_max_now();");
            }
            r
        };

        // ---- upper: computed into locals from the old state ----
        let pre = if slow { "fast::cold::" } else { "fast::" };
        let mut commit = String::new();
        let mut has_mac = false;
        let ut = upper_target(&p.upper);
        match p.upper {
            Upper::Nop => {}
            Upper::Unknown(w) => o(out, &format!("vu.unknown_op({pc:#06x}, {w:#010x});")),
            Upper::Fmac { op, acc, dest, fd, fs, src } => {
                let t = match src {
                    Src::Vec(t) => format!("&vu.vf[{t}]"),
                    Src::Bc(t, b) => format!("&[vu.vf[{t}][{}]; 4]", b & 3),
                    Src::Q => {
                        o(out, "vu.q_sync();");
                        "&[vu.q; 4]".to_string()
                    }
                    Src::I => "&[vu.i; 4]".to_string(),
                };
                let d = format!("0b{dest:04b}");
                // Operands that need `norm` (fast arms only).
                let unk = |f: u128| f & !st.known != 0;
                let ns = unk(if fs == 0 { 0 } else { fields(fs, dest) });
                let nt = match src {
                    Src::Vec(t) => unk(if t == 0 { 0 } else { fields(t, dest) }),
                    Src::Bc(t, b) => unk(if t == 0 { 0 } else { fields(t, 8 >> (b & 3)) }),
                    Src::Q | Src::I => true,
                };
                let na = dest & !st.acc != 0;
                let (g2, g3) = if slow {
                    (format!("<{d}>"), format!("<{d}>"))
                } else {
                    (format!("<{d}, {ns}, {nt}>"), format!("<{d}, {na}, {ns}, {nt}>"))
                };
                match op {
                    Fmac::Max | Fmac::Mini => {
                        let fun = if op == Fmac::Max { "max4" } else { "min4" };
                        o(out, &format!("let u = {pre}{fun}(&vu.vf[{fs}], {t});"));
                    }
                    Fmac::Add | Fmac::Sub | Fmac::Mul => {
                        o(out, &format!("let (u, um) = {pre}{}4::{g2}(&vu.vf[{fs}], {t});", op.name()));
                    }
                    Fmac::Madd | Fmac::Msub => {
                        o(out, &format!("let (u, um) = {pre}{}4::{g3}(&vu.acc, &vu.vf[{fs}], {t});", op.name()));
                    }
                }
                if acc {
                    write!(commit, "vu.put_acc::<{d}>(&u);").unwrap();
                } else if fd != 0 {
                    write!(commit, "vu.put_vf::<{d}>({fd}, &u);{}", ready(f.wu, fd)).unwrap();
                }
                if !matches!(op, Fmac::Max | Fmac::Mini) {
                    has_mac = true;
                }
            }
            Upper::Opmula { fs, ft } => {
                o(out, &format!("let s = vu.vf[{fs}];"));
                o(out, &format!("let t = vu.vf[{ft}];"));
                let g = if slow {
                    "<0b1110>".to_string()
                } else {
                    let unk = |r: u8| r != 0 && fields(r, 0b1110) & !st.known != 0;
                    format!("<0b1110, {}, {}>", unk(fs), unk(ft))
                };
                o(out, &format!("let (u, um) = {pre}mul4::{g}(&[s[1], s[2], s[0], 0], &[t[2], t[0], t[1], 0]);"));
                write!(commit, "vu.put_acc::<0b1110>(&u);").unwrap();
                has_mac = true;
            }
            Upper::Opmsub { fd, fs, ft } => {
                o(out, &format!("let s = vu.vf[{fs}];"));
                o(out, &format!("let t = vu.vf[{ft}];"));
                let g = if slow {
                    "<0b1110>".to_string()
                } else {
                    let unk = |r: u8| r != 0 && fields(r, 0b1110) & !st.known != 0;
                    format!("<0b1110, {}, {}, {}>", 0b1110 & !st.acc != 0, unk(fs), unk(ft))
                };
                o(out, &format!("let (u, um) = {pre}msub4::{g}(&vu.acc, &[s[1], s[2], s[0], 0], &[t[2], t[0], t[1], 0]);"));
                if fd != 0 {
                    write!(commit, "vu.put_vf::<0b1110>({fd}, &u);{}", ready(f.wu, fd)).unwrap();
                }
                has_mac = true;
            }
            Upper::Itof { frac, dest, ft, fs } | Upper::Ftoi { frac, dest, ft, fs } => {
                let fun = if matches!(p.upper, Upper::Itof { .. }) { "itof4" } else { "ftoi4" };
                o(out, &format!("let u = {pre}{fun}::<{frac}>(&vu.vf[{fs}]);"));
                if ft != 0 {
                    write!(commit, "vu.put_vf::<0b{dest:04b}>({ft}, &u);{}", ready(f.wu, ft)).unwrap();
                }
            }
            Upper::Abs { dest, ft, fs } => {
                o(out, &format!("let u = {pre}abs4(&vu.vf[{fs}]);"));
                if ft != 0 {
                    write!(commit, "vu.put_vf::<0b{dest:04b}>({ft}, &u);{}", ready(f.wu, ft)).unwrap();
                }
            }
            Upper::Clip { fs, ft } => {
                o(out, &format!("let uc = fast::clip(vu.vf[{fs}], vu.vf[{ft}][3]);"));
                if st.pending {
                    commit.push_str(MAC_FLUSH);
                    st.pending = false;
                }
                write!(commit, " vu.clip_commit(uc, {push});").unwrap();
            }
        }
        // LOI after the upper op: the upper of an I-bit pair reads the *old* I
        // (PCSX2 `_vu1Exec` / microVU `doIbit`; SSX 3 microcode relies on it).
        if p.hi & 0x8000_0000 != 0 {
            o(out, &format!("vu.loi({:#010x}); // {}", p.lo, f32::from_bits(p.lo)));
        }

        // ---- lower: runs against the old state ----
        if let Some(l) = p.lower {
            if flag_reader(&l) && st.pending {
                o(out, MAC_FLUSH);
                st.pending = false;
            }
            let tag = format!("pb + {k}");
            // A lower VF write: dropped when the upper targets the same VF.
            let lw = |v: &str, ft: u8, dest: u8| -> String {
                if ft == 0 || dest == 0 || ut.map(|t| t.0) == Some(ft) {
                    return String::new();
                }
                format!(" vu.put_vf::<0b{dest:04b}>({ft}, &{v});{}", ready(f.wl, ft))
            };
            let vi = |r: u8| format!("vu.vi[{}]", r & 15);
            let bk = |r: u8, v: String| -> String {
                if r & 15 == 0 {
                    String::new()
                } else {
                    format!("vu.vi_bk({}, {v}, {tag});", r & 15)
                }
            };
            let br = |r: u8| {
                if r & 15 == 0 {
                    "0u16".to_string()
                } else {
                    format!("vu.vi_brf({}, {tag})", r & 15)
                }
            };
            let addr = |r: u8, imm: i16| format!("({}.wrapping_add({}) as u32)", vi(r), imm as u16);
            let line = match l {
                Lower::Lq { dest, ft, is, imm } => {
                    let w = lw("lv", ft, dest);
                    if w.is_empty() {
                        String::new()
                    } else {
                        format!("let lv = vu.ldq({});{w}", addr(is, imm))
                    }
                }
                Lower::Sq { dest, fs, it, imm } => {
                    format!("let sv = vu.vf[{fs}]; vu.stq::<0b{dest:04b}>({}, &sv);", addr(it, imm))
                }
                Lower::Ilw { dest, it, is, imm } => format!(
                    "let lv = vu.ldq({}); vu.vi_set({it}, lv[{}] as u16);",
                    addr(is, imm),
                    (0..4).find(|b| dest & (8 >> b) != 0).unwrap_or(0)
                ),
                Lower::Ilwr { dest, it, is } => format!(
                    "let lv = vu.ldq({} as u32); vu.vi_set({it}, lv[{}] as u16);",
                    vi(is),
                    (0..4).find(|b| dest & (8 >> b) != 0).unwrap_or(0)
                ),
                Lower::Isw { dest, it, is, imm } => format!(
                    "let x = {} as u32; vu.stq::<0b{dest:04b}>({}, &[x; 4]);",
                    vi(it),
                    addr(is, imm)
                ),
                Lower::Iswr { dest, it, is } => format!(
                    "let x = {} as u32; vu.stq::<0b{dest:04b}>({} as u32, &[x; 4]);",
                    vi(it),
                    vi(is)
                ),
                Lower::Iaddiu { it, is, imm } => format!("let x = {}.wrapping_add({imm:#x}); {}", vi(is), bk(it, "x".into())),
                Lower::Isubiu { it, is, imm } => format!("let x = {}.wrapping_sub({imm:#x}); {}", vi(is), bk(it, "x".into())),
                Lower::Iadd { id, is, it } => format!("let x = {}.wrapping_add({}); {}", vi(is), vi(it), bk(id, "x".into())),
                Lower::Isub { id, is, it } => format!("let x = {}.wrapping_sub({}); {}", vi(is), vi(it), bk(id, "x".into())),
                Lower::Iaddi { it, is, imm } => format!(
                    "let x = {}.wrapping_add({}); {}",
                    vi(is),
                    imm as i16 as u16,
                    bk(it, "x".into())
                ),
                Lower::Iand { id, is, it } => format!("let x = {} & {}; {}", vi(is), vi(it), bk(id, "x".into())),
                Lower::Ior { id, is, it } => format!("let x = {} | {}; {}", vi(is), vi(it), bk(id, "x".into())),
                Lower::Lqi { dest, ft, is } => format!(
                    "let a = {}; let lv = vu.ldq(a as u32);{} {}",
                    vi(is),
                    lw("lv", ft, dest),
                    bk(is, "a.wrapping_add(1)".into())
                ),
                Lower::Lqd { dest, ft, is } => format!(
                    "let a = {}.wrapping_sub(1); {} let lv = vu.ldq(a as u32);{}",
                    vi(is),
                    bk(is, "a".into()),
                    lw("lv", ft, dest)
                ),
                Lower::Sqi { dest, fs, it } => format!(
                    "let a = {}; let sv = vu.vf[{fs}]; vu.stq::<0b{dest:04b}>(a as u32, &sv); {}",
                    vi(it),
                    bk(it, "a.wrapping_add(1)".into())
                ),
                Lower::Sqd { dest, fs, it } => format!(
                    "let a = {}.wrapping_sub(1); {} let sv = vu.vf[{fs}]; vu.stq::<0b{dest:04b}>(a as u32, &sv);",
                    vi(it),
                    bk(it, "a".into())
                ),
                Lower::Move { dest, ft, fs } => {
                    let w = lw("lv", ft, dest);
                    if w.is_empty() {
                        String::new()
                    } else {
                        format!("let lv = vu.vf[{fs}];{w}")
                    }
                }
                Lower::Mr32 { dest, ft, fs } => {
                    let w = lw("lv", ft, dest);
                    if w.is_empty() {
                        String::new()
                    } else {
                        format!("let s = vu.vf[{fs}]; let lv = [s[1], s[2], s[3], s[0]];{w}")
                    }
                }
                Lower::Mfir { dest, ft, is } => format!(
                    "let x = {} as i16 as i32 as u32; let lv = [x; 4];{}",
                    vi(is),
                    lw("lv", ft, dest)
                ),
                Lower::Mtir { it, fs, fsf } => format!(
                    "let x = vu.vf[{fs}][{}] as u16; {}",
                    fsf & 3,
                    bk(it, "x".into())
                ),
                Lower::Rnext { dest, ft } => format!("let x = vu.r_next(); let lv = [x; 4];{}", lw("lv", ft, dest)),
                Lower::Rget { dest, ft } => format!("let x = vu.r; let lv = [x; 4];{}", lw("lv", ft, dest)),
                Lower::Mfp { dest, ft } => format!("vu.p_sync(); let x = vu.p; let lv = [x; 4];{}", lw("lv", ft, dest)),
                Lower::Fseq { .. } | Lower::Fsand { .. } | Lower::Fsor { .. } | Lower::Fsset { .. } => {
                    format!("vu.q_sync(); vu.flags_sync(); {}", lower_call(&l).unwrap())
                }
                Lower::Fmeq { .. }
                | Lower::Fmand { .. }
                | Lower::Fmor { .. }
                | Lower::Fceq { .. }
                | Lower::Fcand { .. }
                | Lower::Fcor { .. }
                | Lower::Fcget { .. }
                | Lower::Fcset { .. } => format!("vu.flags_sync(); {}", lower_call(&l).unwrap()),
                Lower::B { .. } => String::new(),
                Lower::Bal { it, .. } => format!("vu.link({it}, {:#06x});", pc + 16),
                Lower::Jr { is } => format!("let target = vu.jr_target({is});"),
                Lower::Jalr { it, is } => format!("let target = vu.jr_target({is}); vu.link({it}, {:#06x});", pc + 16),
                Lower::Ibeq { it, is, .. } => format!("let taken = {} == {};", br(is), br(it)),
                Lower::Ibne { it, is, .. } => format!("let taken = {} != {};", br(is), br(it)),
                Lower::Ibltz { is, .. } => format!("let taken = ({} as i16) < 0;", br(is)),
                Lower::Ibgtz { is, .. } => format!("let taken = ({} as i16) > 0;", br(is)),
                Lower::Iblez { is, .. } => format!("let taken = ({} as i16) <= 0;", br(is)),
                Lower::Ibgez { is, .. } => format!("let taken = ({} as i16) >= 0;", br(is)),
                other => lower_call(&other)
                    .map(|c| c.replace("PC", &format!("{pc:#06x}")))
                    .unwrap_or_default(),
            };
            if l.flow() != Flow::None && !branch {
                // Branch in a delay slot: ignored (unsupported, never occurs).
                o(out, "// branch in delay slot ignored");
            } else if !line.is_empty() {
                o(out, line.trim());
            }
        }
        if !commit.is_empty() {
            o(out, commit.trim());
        }
        // Normal-value tracking: the lower write lands first, then the upper.
        st.known &= !f.wl;
        match p.upper {
            Upper::Fmac { op: Fmac::Max | Fmac::Mini, .. } | Upper::Ftoi { .. } | Upper::Abs { .. } => {
                st.known &= !f.wu
            }
            Upper::Fmac { acc: true, dest, .. } => st.acc |= dest,
            Upper::Opmula { .. } => st.acc |= 0b1110,
            Upper::Fmac { .. } | Upper::Opmsub { .. } | Upper::Itof { .. } => st.known |= f.wu,
            _ => {}
        }
        // MAC / status: an op whose flags no flag op can see in time only
        // accumulates (`fl` last MAC, `fa` OR of MACs for the sticky bits);
        // they are committed before the next observer or at the arm's end.
        if has_mac {
            if !st.declared {
                o(out, "let mut fl = 0u32; let mut fa = 0u32;");
                st.declared = true;
            }
            if push {
                // Earlier deferred ops become visible first (a flag op
                // within this op's latency sees them, not this one).
                if st.pending {
                    o(out, MAC_FLUSH);
                    st.pending = false;
                }
                o(out, "vu.set_mac(um); vu.flags_push();");
            } else {
                o(out, "fl = um; fa |= um;");
                st.pending = true;
            }
        }
        o(out, "vu.cycle += 1;");
    }
}

/// A microcode upload found in a VIF stream: load address and bytes.
pub struct MpgSegment {
    pub addr: u32,
    pub bytes: Vec<u8>,
    /// Word index of the first MPG VIFcode in the scanned buffer.
    pub source_word: usize,
}

/// Scan a buffer of VIF words (as embedded in DMA chains) for MPG commands
/// (CMD 0x4a, placed so its data is 64-bit aligned) and merge consecutive
/// uploads with contiguous load addresses into segments — the same rule
/// `Vu::write_code` applies at runtime.
pub fn extract_mpg_segments(words: &[u32]) -> Vec<MpgSegment> {
    let mut segs: Vec<MpgSegment> = Vec::new();
    let mut i = 0;
    let mut last_end: Option<usize> = None;
    while i < words.len() {
        let w = words[i];
        if (w >> 24) & 0x7f == 0x4a && i % 2 == 1 {
            let num = match (w >> 16) & 0xff {
                0 => 256,
                n => n as usize,
            };
            let addr = (w & 0xffff) * 8;
            if i + 1 + num * 2 > words.len() {
                break;
            }
            let bytes: Vec<u8> = words[i + 1..i + 1 + num * 2]
                .iter()
                .flat_map(|x| x.to_le_bytes())
                .collect();
            // Only NOP padding may separate merged MPGs.
            let contiguous = match (segs.last(), last_end) {
                (Some(s), Some(e)) => {
                    s.addr + s.bytes.len() as u32 == addr
                        && words[e..i].iter().all(|&x| x & 0x7f00_0000 == 0)
                }
                _ => false,
            };
            if contiguous {
                segs.last_mut().unwrap().bytes.extend_from_slice(&bytes);
            } else {
                segs.push(MpgSegment {
                    addr,
                    bytes,
                    source_word: i,
                });
            }
            i += 1 + num * 2;
            last_end = Some(i);
        } else {
            i += 1;
        }
    }
    segs
}

/// Render `generated/vu/mod.rs` for a set of translations.
pub fn registry_source(ts: &[Translation]) -> String {
    let mut s = String::new();
    writeln!(
        s,
        "//! Statically recompiled VU microcode, keyed by (FNV-1a hash of the"
    )
    .unwrap();
    writeln!(s, "//! uploaded segment incl. load address, start pc).").unwrap();
    writeln!(
        s,
        "//! Generated by `cargo run -p ssx3-ps2 --bin vu_recomp`; do not edit."
    )
    .unwrap();
    writeln!(s, "use crate::vu::ProgramEntry;").unwrap();
    writeln!(s).unwrap();
    for t in ts {
        writeln!(s, "pub mod {};", t.module).unwrap();
    }
    writeln!(s).unwrap();
    writeln!(s, "pub static PROGRAMS: &[ProgramEntry] = &[").unwrap();
    for t in ts {
        writeln!(
            s,
            "    ProgramEntry {{ hash: {m}::HASH, start: {m}::START, len: {m}::LEN, leaders: {m}::LEADERS, func: {m}::run }},",
            m = t.module
        )
        .unwrap();
    }
    writeln!(s, "];").unwrap();
    s
}

/// Rust source for one EE COP2 macro-mode instruction (VU0), with `vu.` as the
/// receiver, or `None` if it cannot be expressed statically (the caller then
/// falls back to `Vu::macro_op`). Mirrors `Vu::macro_op` exactly.
pub fn macro_source(word: u32) -> Option<String> {
    match decode_macro(word) {
        Macro::Upper(u) => {
            // VNOP has no call; keep the begin/end so pipeline state matches macro_op.
            let c = upper_call(&u).unwrap_or_default();
            Some(format!("vu.macro_begin(); {c} vu.macro_end_upper();"))
        }
        Macro::Lower(l) => {
            let c = lower_call(&l)?.replace("PC", "0");
            if c.contains("kick") {
                return None;
            }
            Some(format!("vu.macro_begin(); {c} vu.macro_end_lower();"))
        }
        _ => None,
    }
}
