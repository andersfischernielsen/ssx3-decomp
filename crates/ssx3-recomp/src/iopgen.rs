//! IOP code generator: R3000A (MIPS I) machine code of IRX modules → Rust,
//! against `ssx3_iop::Iop`.
//!
//! Same shape as the EE generator (`eegen.rs`): every function is a block
//! dispatcher over its labels (entry, branch targets, return points after
//! calls, code addresses that data or `%hi/%lo` pairs refer to, and targets of
//! branches from other functions), so any label can be (re-)entered by the
//! runtime trampoline after a thread switch. Calls are direct Rust calls; a
//! callee that needs to switch threads leaves `UNWIND` in `pc` and every
//! frame returns.
//!
//! R3000 specifics:
//! * 32-bit registers (`c.r[n]: u32`), HI/LO, no 64-bit or MMI operations.
//! * Load delay slots: compiled code respects them (a `nop` or an unrelated
//!   instruction follows each load), so loads are translated as immediate.
//!   Every load whose result is read by the very next instruction is listed in
//!   the report as suspicious.
//! * No branch-likely instructions. Branches evaluate their condition before
//!   the delay slot runs.
//! * Calls to import stubs become native calls (`ssx3_iop::imports::call`) or,
//!   for a library exported by another translated module (libsd), direct calls
//!   to that module's function: the modules are linked statically at the fixed
//!   addresses in `ssx3_iop::irx::layout`.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use ssx3_iop::irx::{self, Irx};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Ctl {
    None,
    Branch { target: u32, link: bool },
    Jump { target: u32 },
    Call { target: u32 },
    JumpReg { rs: u32 },
    CallReg { rd: u32, rs: u32 },
    Syscall,
    Break,
}

fn branch_target(pc: u32, w: u32) -> u32 {
    pc.wrapping_add(4).wrapping_add(((w as i16 as i32) << 2) as u32)
}

fn classify(pc: u32, w: u32) -> Ctl {
    let op = w >> 26;
    let rt = (w >> 16) & 31;
    match op {
        0 => match w & 63 {
            0x08 => Ctl::JumpReg { rs: (w >> 21) & 31 },
            0x09 => Ctl::CallReg { rd: (w >> 11) & 31, rs: (w >> 21) & 31 },
            0x0C => Ctl::Syscall,
            0x0D => Ctl::Break,
            _ => Ctl::None,
        },
        1 => match rt {
            0x00 | 0x01 => Ctl::Branch { target: branch_target(pc, w), link: false },
            0x10 | 0x11 => Ctl::Branch { target: branch_target(pc, w), link: true },
            _ => Ctl::None,
        },
        2 => Ctl::Jump { target: (pc & 0xF000_0000) | ((w & 0x03FF_FFFF) << 2) },
        3 => Ctl::Call { target: (pc & 0xF000_0000) | ((w & 0x03FF_FFFF) << 2) },
        4..=7 => Ctl::Branch { target: branch_target(pc, w), link: false },
        _ => Ctl::None,
    }
}

fn r(n: u32) -> String {
    if n == 0 {
        "0u32".into()
    } else {
        format!("c.r[{n}]")
    }
}

fn set(rd: u32, e: String) -> String {
    if rd == 0 {
        String::new()
    } else {
        format!("c.r[{rd}] = {e};")
    }
}

fn ea(rs: u32, off: i16) -> String {
    if off == 0 {
        r(rs)
    } else {
        format!("{}.wrapping_add({:#x})", r(rs), off as i32 as u32)
    }
}

fn branch_cond(w: u32) -> String {
    let op = w >> 26;
    let rs = (w >> 21) & 31;
    let rt = (w >> 16) & 31;
    let s = |n: u32| if n == 0 { "0i32".to_string() } else { format!("(c.r[{n}] as i32)") };
    match op {
        4 => {
            if rs == rt {
                "true".into()
            } else {
                format!("{} == {}", r(rs), r(rt))
            }
        }
        5 => format!("{} != {}", r(rs), r(rt)),
        6 => format!("{} <= 0", s(rs)),
        7 => format!("{} > 0", s(rs)),
        1 => {
            if rt & 1 == 0 {
                format!("{} < 0", s(rs))
            } else {
                format!("{} >= 0", s(rs))
            }
        }
        _ => "false".into(),
    }
}

/// Lower a non-control instruction. `None` for an invalid encoding.
fn lower(w: u32) -> Option<String> {
    let op = w >> 26;
    let rs = (w >> 21) & 31;
    let rt = (w >> 16) & 31;
    let rd = (w >> 11) & 31;
    let sa = (w >> 6) & 31;
    let imm = w & 0xFFFF;
    let simm = w as i16;
    let load = |f: &str| {
        if rt == 0 {
            format!("let _ = c.{f}({});", ea(rs, simm))
        } else {
            format!("c.r[{rt}] = c.{f}({});", ea(rs, simm))
        }
    };
    Some(match op {
        0 => match w & 63 {
            0x00 => {
                if w == 0 {
                    String::new()
                } else {
                    set(rd, format!("{} << {sa}", r(rt)))
                }
            }
            0x02 => set(rd, format!("{} >> {sa}", r(rt))),
            0x03 => set(rd, format!("(({} as i32) >> {sa}) as u32", r(rt))),
            0x04 => set(rd, format!("{} << ({} & 31)", r(rt), r(rs))),
            0x06 => set(rd, format!("{} >> ({} & 31)", r(rt), r(rs))),
            0x07 => set(rd, format!("(({} as i32) >> ({} & 31)) as u32", r(rt), r(rs))),
            0x10 => set(rd, "c.hi".into()),
            0x11 => format!("c.hi = {};", r(rs)),
            0x12 => set(rd, "c.lo".into()),
            0x13 => format!("c.lo = {};", r(rs)),
            0x18 => format!("c.mult({rs}, {rt});"),
            0x19 => format!("c.multu({rs}, {rt});"),
            0x1A => format!("c.div({rs}, {rt});"),
            0x1B => format!("c.divu({rs}, {rt});"),
            // add/sub: overflow traps are not modelled.
            0x20 | 0x21 => set(rd, format!("{}.wrapping_add({})", r(rs), r(rt))),
            0x22 | 0x23 => set(rd, format!("{}.wrapping_sub({})", r(rs), r(rt))),
            0x24 => set(rd, format!("{} & {}", r(rs), r(rt))),
            0x25 => set(rd, format!("{} | {}", r(rs), r(rt))),
            0x26 => set(rd, format!("{} ^ {}", r(rs), r(rt))),
            0x27 => set(rd, format!("!({} | {})", r(rs), r(rt))),
            0x2A => set(rd, format!("(({} as i32) < ({} as i32)) as u32", r(rs), r(rt))),
            0x2B => set(rd, format!("({} < {}) as u32", r(rs), r(rt))),
            _ => return None,
        },
        0x08 | 0x09 => set(rt, format!("{}.wrapping_add({:#x})", r(rs), simm as i32 as u32)),
        0x0A => set(rt, format!("(({} as i32) < {}i32) as u32", r(rs), simm)),
        0x0B => set(rt, format!("({} < {:#x}u32) as u32", r(rs), simm as i32 as u32)),
        0x0C => set(rt, format!("{} & {imm:#x}", r(rs))),
        0x0D => set(rt, format!("{} | {imm:#x}", r(rs))),
        0x0E => set(rt, format!("{} ^ {imm:#x}", r(rs))),
        0x0F => set(rt, format!("{:#x}u32", imm << 16)),
        0x10 => match rs {
            0x00 => set(rt, format!("c.mfc0({rd})")),
            0x04 => format!("c.mtc0({rd}, {});", r(rt)),
            0x10 if w & 63 == 0x10 => String::new(), // rfe
            _ => return None,
        },
        0x20 => load("lb"),
        0x21 => load("lh"),
        0x22 => format!("c.lwl({rt}, {});", ea(rs, simm)),
        0x23 => load("lw"),
        0x24 => load("lbu"),
        0x25 => load("lhu"),
        0x26 => format!("c.lwr({rt}, {});", ea(rs, simm)),
        0x28 => format!("c.sb({}, {});", ea(rs, simm), r(rt)),
        0x29 => format!("c.sh({}, {});", ea(rs, simm), r(rt)),
        0x2A => format!("c.swl({rt}, {});", ea(rs, simm)),
        0x2B => format!("c.sw({}, {});", ea(rs, simm), r(rt)),
        0x2E => format!("c.swr({rt}, {});", ea(rs, simm)),
        _ => return None,
    })
}

fn is_load(w: u32) -> Option<u32> {
    let op = w >> 26;
    matches!(op, 0x20..=0x26).then_some((w >> 16) & 31)
}

/// Registers an instruction reads.
fn reads(w: u32) -> Vec<u32> {
    let op = w >> 26;
    let rs = (w >> 21) & 31;
    let rt = (w >> 16) & 31;
    match op {
        0 => match w & 63 {
            0x00 | 0x02 | 0x03 => vec![rt],
            0x10 | 0x12 | 0x0C | 0x0D => vec![],
            0x08 | 0x09 | 0x11 | 0x13 => vec![rs],
            _ => vec![rs, rt],
        },
        1 | 6 | 7 => vec![rs],
        4 | 5 => vec![rs, rt],
        0x08..=0x0E => vec![rs],
        0x10 => {
            if rs == 4 {
                vec![rt]
            } else {
                vec![]
            }
        }
        0x22 | 0x26 => vec![rs, rt],
        0x20..=0x25 => vec![rs],
        0x28..=0x2E => vec![rs, rt],
        _ => vec![],
    }
}

/// Call target resolution for import stubs.
#[derive(Clone, Copy, Debug)]
enum Stub {
    Native(usize, u16),
    Link(u32),
    Missing(u16),
}

struct Func {
    start: u32,
    end: u32,
}

pub struct Output {
    pub code: String,
    pub report: String,
    pub functions: usize,
    pub instructions: usize,
    pub invalid: usize,
    pub load_hazards: usize,
}

fn fn_name(a: u32) -> String {
    format!("f_{a:08x}")
}

/// Translate the given modules (already relocated at their fixed bases).
pub fn generate(mods: &[Irx]) -> Output {
    let mut report = String::new();
    // Exports by library name, for static linking of imports.
    let mut exports: BTreeMap<String, Vec<u32>> = BTreeMap::new();
    for m in mods {
        for e in &m.exports {
            exports.insert(e.lib.clone(), e.entries.clone());
        }
    }
    let mut stubs: BTreeMap<u32, Stub> = BTreeMap::new();
    for m in mods {
        for i in &m.imports {
            for &(a, idx) in &i.stubs {
                let s = if let Some(e) = exports.get(&i.lib) {
                    match e.get(idx as usize) {
                        Some(&t) => Stub::Link(t),
                        None => Stub::Missing(idx),
                    }
                } else if let Some(id) = irx::library_id(&i.lib) {
                    Stub::Native(id, idx)
                } else {
                    let _ = writeln!(report, "WARNING: {}: import from unknown library {} #{idx}", m.name, i.lib);
                    Stub::Missing(idx)
                };
                stubs.insert(a, s);
            }
        }
    }

    // Code ranges (text minus import/export tables) and functions.
    let mut words: BTreeMap<u32, u32> = BTreeMap::new();
    let mut funcs: Vec<Func> = Vec::new();
    let mut code_refs: BTreeSet<u32> = BTreeSet::new();
    for m in mods {
        let mut ranges: Vec<(u32, u32)> = Vec::new();
        let mut lo = m.text_start();
        for (s, e) in m.table_ranges() {
            if s > lo {
                ranges.push((lo, s));
            }
            lo = lo.max(e);
        }
        if m.text_end() > lo {
            ranges.push((lo, m.text_end()));
        }
        let in_code = |a: u32| ranges.iter().any(|&(s, e)| a >= s && a < e);
        for &(s, e) in &ranges {
            let mut a = s;
            while a < e {
                words.insert(a, m.word(a).unwrap());
                a += 4;
            }
        }
        let mut starts: BTreeSet<u32> = BTreeSet::new();
        starts.insert(m.entry);
        for &(s, _) in &ranges {
            starts.insert(s);
        }
        for e in &m.exports {
            starts.extend(e.entries.iter().copied().filter(|&a| in_code(a)));
        }
        for s in m.symbols.iter().filter(|s| s.func && in_code(s.addr)) {
            starts.insert(s.addr);
        }
        if m.symbols.iter().all(|s| !s.func) {
            starts.extend(m.hilo_code_ptrs.iter().copied().filter(|&a| in_code(a)));
        }
        for (&a, &w) in words.range(m.text_start()..m.text_end()) {
            if let Ctl::Call { target } = classify(a, w) {
                if in_code(target) {
                    starts.insert(target);
                }
            }
        }
        for &(s, e) in &ranges {
            let v: Vec<u32> = starts.range(s..e).copied().collect();
            for (i, &f) in v.iter().enumerate() {
                let end = v.get(i + 1).copied().unwrap_or(e);
                funcs.push(Func { start: f, end });
            }
        }
        code_refs.extend(m.code_refs.iter().copied().filter(|&a| in_code(a)));
        for e in &m.exports {
            code_refs.extend(e.entries.iter().copied().filter(|&a| in_code(a)));
        }
        let _ = writeln!(report, "{}", irx::describe(m).trim_end());
    }
    funcs.sort_by_key(|f| f.start);
    let container = |a: u32| -> Option<u32> {
        let i = funcs.partition_point(|f| f.start <= a);
        (i > 0 && a < funcs[i - 1].end).then(|| funcs[i - 1].start)
    };

    // Labels: per-function sets.
    let mut labels: BTreeMap<u32, BTreeSet<u32>> = funcs.iter().map(|f| (f.start, BTreeSet::from([f.start]))).collect();
    let add_label = |labels: &mut BTreeMap<u32, BTreeSet<u32>>, a: u32| {
        if let Some(f) = container(a) {
            labels.get_mut(&f).unwrap().insert(a);
        }
    };
    for (&a, &w) in &words {
        match classify(a, w) {
            Ctl::Branch { target, .. } => {
                add_label(&mut labels, target);
                add_label(&mut labels, a + 8);
            }
            Ctl::Jump { target } => add_label(&mut labels, target),
            Ctl::Call { target } => {
                add_label(&mut labels, target);
                add_label(&mut labels, a + 8);
            }
            Ctl::CallReg { .. } => add_label(&mut labels, a + 8),
            Ctl::Syscall => add_label(&mut labels, a + 4),
            _ => {}
        }
    }
    for &a in &code_refs {
        add_label(&mut labels, a);
    }
    for (_, s) in &stubs {
        if let Stub::Link(t) = s {
            add_label(&mut labels, *t);
        }
    }

    // Suspicious load-delay uses.
    let mut hazards = 0;
    for (&a, &w) in &words {
        let Some(rt) = is_load(w) else { continue };
        if rt == 0 {
            continue;
        }
        // A load in a branch delay slot is followed by the branch target.
        if let Some(&pw) = words.get(&(a.wrapping_sub(4))) {
            if classify(a - 4, pw) != Ctl::None {
                continue;
            }
        }
        let Some(&nw) = words.get(&(a + 4)) else { continue };
        let pair = matches!((w >> 26, nw >> 26), (0x22, 0x26) | (0x26, 0x22)) && (nw >> 16) & 31 == rt;
        if !pair && reads(nw).contains(&rt) {
            hazards += 1;
            let _ = writeln!(report, "load-delay hazard: {a:#08x} {w:08x} -> {:#08x} {nw:08x} reads r{rt}", a + 4);
        }
    }

    // Emit.
    let mut code = String::new();
    let mut invalid = 0;
    let mut instructions = 0;
    let call_code = |target: u32, ret: u32, stubs: &BTreeMap<u32, Stub>| -> String {
        match stubs.get(&target) {
            Some(Stub::Native(lib, idx)) => {
                format!("ssx3_iop::imports::call(c, {lib}, {idx}); if c.pc != {ret:#010x} {{ return; }}")
            }
            Some(Stub::Missing(idx)) => {
                format!("c.fatal({target:#010x}, \"call to an unresolved import #{idx}\"); return;")
            }
            Some(Stub::Link(t)) => direct_call(*t, ret, &container),
            None => direct_call(target, ret, &container),
        }
    };
    for f in &funcs {
        let fl = &labels[&f.start];
        let insns: Vec<(u32, u32)> = words.range(f.start..f.end).map(|(&a, &w)| (a, w)).collect();
        instructions += insns.len();
        let inside = |a: u32| a >= f.start && a < f.end && fl.contains(&a);
        let _ = writeln!(code, "pub fn {}(c: &mut Iop) {{", fn_name(f.start));
        let _ = writeln!(code, "    let mut pc: u32 = c.pc;");
        let _ = writeln!(code, "    loop {{");
        let _ = writeln!(code, "        match pc {{");
        let label_list: Vec<u32> = fl.iter().copied().collect();
        for &l in &label_list {
            let Some(start_i) = insns.iter().position(|&(a, _)| a == l) else { continue };
            let _ = writeln!(code, "            {l:#010x} => {{");
            let mut body = String::new();
            let mut count = 0usize;
            let mut i = start_i;
            let mut done = false;
            let goto = |target: u32, pc: u32| -> String {
                if inside(target) {
                    if target <= pc {
                        format!(
                            "if c.cycles >= c.next_event {{ c.pc = {target:#010x}; if c.event() {{ return; }} }} pc = {target:#010x}; continue;"
                        )
                    } else {
                        format!("pc = {target:#010x}; continue;")
                    }
                } else {
                    tail(target, &container)
                }
            };
            while i < insns.len() {
                let (pc, w) = insns[i];
                if pc != l && fl.contains(&pc) {
                    let _ = writeln!(body, "                pc = {pc:#010x}; continue;");
                    done = true;
                    break;
                }
                count += 1;
                let ctl = classify(pc, w);
                if ctl == Ctl::None {
                    match lower(w) {
                        Some(s) => {
                            if !s.is_empty() {
                                let _ = writeln!(body, "                {s}");
                            }
                        }
                        None => {
                            invalid += 1;
                            let _ = writeln!(body, "                c.fatal({pc:#010x}, \"invalid instruction {w:#010x}\"); return;");
                            done = true;
                            break;
                        }
                    }
                    i += 1;
                    continue;
                }
                let delay = insns.get(i + 1).filter(|(a, _)| *a == pc + 4).map(|(_, d)| *d);
                let delay = if matches!(ctl, Ctl::Syscall | Ctl::Break) { None } else { delay };
                let ds = match delay {
                    Some(d) if classify(pc + 4, d) == Ctl::None => match lower(d) {
                        Some(s) => s,
                        None => {
                            invalid += 1;
                            format!("c.fatal({:#010x}, \"invalid instruction in delay slot\"); return;", pc + 4)
                        }
                    },
                    Some(_) => {
                        let _ = writeln!(report, "WARNING: control transfer in delay slot at {:#x}", pc + 4);
                        String::new()
                    }
                    None => String::new(),
                };
                if delay.is_some() && !matches!(ctl, Ctl::Syscall | Ctl::Break) {
                    count += 1;
                }
                let after = pc + 8;
                let fall = if inside(after) { format!("pc = {after:#010x}; continue;") } else { tail(after, &container) };
                match ctl {
                    Ctl::Branch { target, link } => {
                        let cond = branch_cond(w);
                        if link {
                            // bltzal/bgezal: $ra is written whether or not the branch is taken.
                            let _ = writeln!(body, "                let t = {cond}; c.r[31] = {after:#010x};");
                            let _ = writeln!(body, "                {ds}");
                            let _ = writeln!(body, "                if t {{ {} }}", call_code(target, after, &stubs));
                        } else {
                            let _ = writeln!(body, "                let t = {cond};");
                            let _ = writeln!(body, "                {ds}");
                            let _ = writeln!(body, "                if t {{ {} }}", goto(target, pc));
                        }
                        let _ = writeln!(body, "                {fall}");
                    }
                    Ctl::Jump { target } => {
                        let _ = writeln!(body, "                {ds}");
                        let _ = writeln!(body, "                {}", goto(target, pc));
                    }
                    Ctl::Call { target } => {
                        let _ = writeln!(body, "                c.r[31] = {after:#010x};");
                        let _ = writeln!(body, "                {ds}");
                        let _ = writeln!(body, "                {}", call_code(target, after, &stubs));
                        let _ = writeln!(body, "                {fall}");
                    }
                    Ctl::JumpReg { rs } => {
                        let _ = writeln!(body, "                let t = {};", r(rs));
                        let _ = writeln!(body, "                {ds}");
                        if rs == 31 {
                            let _ = writeln!(body, "                c.pc = t; return;");
                        } else {
                            let _ = writeln!(body, "                pc = t; continue;");
                        }
                    }
                    Ctl::CallReg { rd, rs } => {
                        let _ = writeln!(body, "                let t = {};", r(rs));
                        if rd != 0 {
                            let _ = writeln!(body, "                c.r[{rd}] = {after:#010x};");
                        }
                        let _ = writeln!(body, "                {ds}");
                        let _ = writeln!(
                            body,
                            "                if !c.enter(t) {{ return; }} ssx3_iop::call_indirect(c, t); c.leave(); if c.pc != {after:#010x} {{ return; }}"
                        );
                        let _ = writeln!(body, "                {fall}");
                    }
                    Ctl::Syscall => {
                        let _ = writeln!(body, "                c.syscall({pc:#010x}); return;");
                    }
                    Ctl::Break => {
                        let _ = writeln!(body, "                c.brk({pc:#010x}, {}); return;", (w >> 6) & 0xFFFFF);
                    }
                    Ctl::None => unreachable!(),
                }
                done = true;
                break;
            }
            if !done {
                let _ = writeln!(body, "                {}", tail(f.end, &container));
            }
            let _ = writeln!(code, "                c.cycles += {count};");
            code.push_str(&body);
            let _ = writeln!(code, "            }}");
        }
        let _ = writeln!(
            code,
            "            _ => {{ ssx3_iop::jump_out(c, pc, {:#010x}, {:#010x}); return; }}",
            f.start, f.end
        );
        let _ = writeln!(code, "        }}");
        let _ = writeln!(code, "    }}");
        let _ = writeln!(code, "}}\n");
    }

    // Tables.
    let _ = writeln!(code, "pub static FUNCTIONS: [FnEntry; {}] = [", funcs.len());
    for f in &funcs {
        let _ = writeln!(code, "    FnEntry {{ start: {:#010x}, end: {:#010x}, f: {} }},", f.start, f.end, fn_name(f.start));
    }
    let _ = writeln!(code, "];\n");
    let native: Vec<(u32, String)> = stubs
        .iter()
        .filter_map(|(&a, s)| match s {
            Stub::Native(l, i) => Some((a, format!("StubKind::Native({l}, {i})"))),
            Stub::Link(t) => Some((a, format!("StubKind::Link({t:#010x})"))),
            Stub::Missing(_) => None,
        })
        .collect();
    let _ = writeln!(code, "pub static STUBS: [(u32, StubKind); {}] = [", native.len());
    for (a, k) in &native {
        let _ = writeln!(code, "    ({a:#010x}, {k}),");
    }
    let _ = writeln!(code, "];\n");
    let _ = writeln!(code, "pub static MODULES: [ModuleInfo; {}] = [", mods.len());
    for m in mods {
        let _ = writeln!(
            code,
            "    ModuleInfo {{ name: {:?}, base: {:#010x}, text_hash: {:#018x} }},",
            m.name_key(),
            m.base,
            m.text_hash()
        );
    }
    let _ = writeln!(code, "];\n");
    let _ = writeln!(
        code,
        "pub static GENERATED: Generated = Generated {{ functions: &FUNCTIONS, stubs: &STUBS, modules: &MODULES }};"
    );
    let header = "//! Generated by `recomp-iop` (ssx3-recomp/src/iopgen.rs) from the disc's IRX modules. Do not edit.\n\
#![allow(unused_mut, unused_variables, unreachable_code, unused_assignments, clippy::all)]\n\n\
use ssx3_iop::{FnEntry, Generated, Iop, ModuleInfo, StubKind};\n\n";
    let _ = writeln!(report, "functions {} instructions {instructions} invalid {invalid} load-delay hazards {hazards}", funcs.len());
    Output { code: format!("{header}{code}"), report, functions: funcs.len(), instructions, invalid, load_hazards: hazards }
}

fn direct_call(target: u32, ret: u32, container: &dyn Fn(u32) -> Option<u32>) -> String {
    match container(target) {
        Some(f) => format!(
            "if !c.enter({target:#010x}) {{ return; }} {}(c); c.leave(); if c.pc != {ret:#010x} {{ return; }}",
            fn_name(f)
        ),
        None => format!(
            "if !c.enter({target:#010x}) {{ return; }} ssx3_iop::call_indirect(c, {target:#010x}); c.leave(); if c.pc != {ret:#010x} {{ return; }}"
        ),
    }
}

fn tail(target: u32, container: &dyn Fn(u32) -> Option<u32>) -> String {
    match container(target) {
        Some(f) => format!("if !c.enter({target:#010x}) {{ return; }} {}(c); c.leave(); return;", fn_name(f)),
        None => format!(
            "if !c.enter({target:#010x}) {{ return; }} ssx3_iop::call_indirect(c, {target:#010x}); c.leave(); return;"
        ),
    }
}

/// Module key used by the runtime to match a loaded file (file stem, upper case).
trait NameKey {
    fn name_key(&self) -> String;
}

impl NameKey for Irx {
    fn name_key(&self) -> String {
        irx::layout::MODULES
            .iter()
            .find(|(_, b)| *b == self.base)
            .map(|(n, _)| n.to_string())
            .unwrap_or_else(|| self.name.clone())
    }
}

/// Read the sound modules from the disc image, translate them, and write the
/// crate `ssx3-iop-gen` to `out` (`generated/iop/modules`).
pub fn generate_from_disc(iso: &std::path::Path, out: &std::path::Path) -> Result<Output, String> {
    let mut disc = ssx3_ps2::iso::Disc::open(iso).map_err(|e| format!("{}: {e}", iso.display()))?;
    let mut mods = Vec::new();
    for (name, base) in irx::layout::MODULES {
        let path = format!("cdrom0:\\DATA\\MODULES\\{name}.IRX;1");
        let e = disc.find(&path).ok_or_else(|| format!("{path} not found on the disc"))?;
        let mut data = disc.read_vec(e.sector, e.size.div_ceil(2048));
        data.truncate(e.size as usize);
        mods.push(irx::load(&data, *base)?);
    }
    let o = generate(&mods);
    std::fs::create_dir_all(out.join("src")).map_err(|e| e.to_string())?;
    let toml = "[package]\nname = \"ssx3-iop-gen\"\nversion = \"0.1.0\"\nedition = \"2021\"\npublish = false\n\n[lib]\npath = \"src/lib.rs\"\n\n[dependencies]\nssx3-iop = { path = \"../../../crates/ssx3-iop\" }\n";
    std::fs::write(out.join("Cargo.toml"), toml).map_err(|e| e.to_string())?;
    std::fs::write(out.join("src/lib.rs"), &o.code).map_err(|e| e.to_string())?;
    std::fs::write(out.join("report.txt"), &o.report).map_err(|e| e.to_string())?;
    Ok(o)
}
