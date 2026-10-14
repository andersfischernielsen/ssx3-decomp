//! Structured Rust emission for host-thread mode.
//!
//! Turns a function's CFG into structured Rust (`if`/`else`, labelled `loop`s
//! and labelled blocks with `break`/`continue`, no `pc` variable), following
//! N. Ramsey, "Beyond Relooper: recursive translation of unstructured control
//! flow to structured control flow" (ICFP 2022):
//!
//! * a loop header becomes `'l_X: loop { … }`, and back edges `continue 'l_X`;
//! * a merge node (≥ 2 forward predecessors) `Y` becomes the code that follows
//!   a labelled block `'b_Y: { … }`; edges to it are `break 'b_Y`;
//! * any other successor is dominated by its only predecessor and is emitted
//!   inline.
//!
//! Only reducible CFGs without computed intra-function jumps are structured;
//! the caller falls back to the block-dispatch form otherwise. Functions that
//! can be entered somewhere other than their start (cross-function branches,
//! jump-table/data references) get a dispatch-form twin used for those entries.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use crate::eegen::{
    branch_cond_pub, classify_pub, direct_callee_pub, fn_name, is_lowerable, lower, tail_pub, Ctl, Ctx,
    Func, Image,
};

enum Edge {
    To(u32),
    Out(String),
}

enum Term {
    /// Computed jump within the function: `match t { targets… , _ => out }`.
    Switch { prelude: String, targets: Vec<u32>, other: String },
    Goto(Edge),
    Cond { prelude: String, cond: String, then_extra: String, taken: Edge, fall: Edge },
    Leave(String),
}

struct Block {
    code: String,
    term: Term,
}

const MAX_NEST: usize = 64;

/// Try to emit `f` as structured Rust. Returns `None` if the function needs
/// the dispatch form (computed jumps, irreducible flow, very deep nesting).
pub fn emit_structured(img: &dyn Image, f: &Func, shard: usize, ctx: &Ctx, external: &BTreeSet<u32>) -> Option<String> {
    let mut insns: Vec<(u32, u32)> = Vec::new();
    for &(lo, hi) in &f.runs {
        let mut a = lo;
        while a < hi {
            insns.push((a, img.word(a).unwrap_or(0)));
            a += 4;
        }
    }
    let is_insn: BTreeSet<u32> = insns.iter().map(|(a, _)| *a).collect();
    let index: BTreeMap<u32, usize> = insns.iter().enumerate().map(|(i, (a, _))| (*a, i)).collect();
    let inside = |a: u32| a >= f.start && a < f.end && is_insn.contains(&a);

    // Block starts: entry + every intra-function branch/jump target + the
    // fallthrough of every branch.
    let mut starts: BTreeSet<u32> = BTreeSet::new();
    starts.insert(f.start);
    let mut has_switch = false;
    for &(pc, w) in &insns {
        match classify_pub(pc, w) {
            Ctl::Branch { target, link, .. } => {
                if !link && inside(target) {
                    starts.insert(target);
                }
                if inside(pc + 8) {
                    starts.insert(pc + 8);
                }
            }
            Ctl::Jump { target } => {
                if inside(target) {
                    starts.insert(target);
                }
            }
            Ctl::JumpReg { rs } if rs != 31 => has_switch = true,
            _ => {}
        }
        if !is_lowerable(pc, w) {
            return None;
        }
    }

    // Jump-table targets: in-function addresses that data refers to.
    let switch_targets: Vec<u32> = if has_switch {
        ctx.data_refs.range(f.start..f.end).copied().filter(|a| inside(*a)).collect()
    } else {
        Vec::new()
    };
    if has_switch && switch_targets.is_empty() {
        return None;
    }
    starts.extend(switch_targets.iter().copied());

    // Build blocks.
    let mut blocks: BTreeMap<u32, Block> = BTreeMap::new();
    for &l in &starts {
        let mut i = index[&l];
        let mut code = String::new();
        let mut count = 0usize;
        let term;
        loop {
            if i >= insns.len() {
                let after = insns.last().map(|(a, _)| a + 4).unwrap_or(f.end);
                term = Term::Leave(tail_pub(after, shard, ctx));
                break;
            }
            let (pc, w) = insns[i];
            if pc != l && (starts.contains(&pc) || insns[i - 1].0 + 4 != pc) {
                term = Term::Goto(if inside(pc) { Edge::To(pc) } else { Edge::Out(tail_pub(pc, shard, ctx)) });
                break;
            }
            count += 1;
            let ctl = classify_pub(pc, w);
            let delay = insns.get(i + 1).filter(|(a, _)| *a == pc + 4).map(|(_, dw)| *dw);
            let ds = match delay {
                Some(dw) if classify_pub(pc + 4, dw) == Ctl::None => lower(pc + 4, dw).unwrap_or_default(),
                _ => String::new(),
            };
            let edge = |t: u32| if inside(t) { Edge::To(t) } else { Edge::Out(tail_pub(t, shard, ctx)) };
            let after_call = |code: &mut String, ret: u32| {
                let _ = writeln!(
                    code,
                    "if c.pc != {ret:#010x} {{ return; }}"
                );
            };
            match ctl {
                Ctl::None => {
                    let s = lower(pc, w)?;
                    if !s.is_empty() {
                        let _ = writeln!(code, "{s}");
                    }
                    i += 1;
                    continue;
                }
                Ctl::Call { target } => {
                    count += delay.is_some() as usize;
                    let ret = pc + 8;
                    let _ = writeln!(code, "c.g[31][0] = sx({ret:#010x});");
                    if !ds.is_empty() {
                        let _ = writeln!(code, "{ds}");
                    }
                    let _ = writeln!(code, "c.pc = {target:#010x}; {};", direct_callee_pub(target, shard, ctx));
                    after_call(&mut code, ret);
                    if !inside(ret) {
                        term = Term::Leave(tail_pub(ret, shard, ctx));
                        break;
                    }
                    i = index[&ret];
                    if starts.contains(&ret) {
                        term = Term::Goto(Edge::To(ret));
                        break;
                    }
                    continue;
                }
                Ctl::CallReg { rd, rs } => {
                    count += delay.is_some() as usize;
                    let ret = pc + 8;
                    let _ = writeln!(code, "let t = c.g[{rs}][0] as u32;");
                    if rd != 0 {
                        let _ = writeln!(code, "c.g[{rd}][0] = sx({ret:#010x});");
                    }
                    if !ds.is_empty() {
                        let _ = writeln!(code, "{ds}");
                    }
                    let _ = writeln!(code, "c.pc = t; ssx3_ps2::machine::call_indirect(c, t);");
                    after_call(&mut code, ret);
                    if !inside(ret) {
                        term = Term::Leave(tail_pub(ret, shard, ctx));
                        break;
                    }
                    i = index[&ret];
                    if starts.contains(&ret) {
                        term = Term::Goto(Edge::To(ret));
                        break;
                    }
                    continue;
                }
                Ctl::Syscall => {
                    let ret = pc + 4;
                    let _ = writeln!(code, "c.pc = {ret:#010x}; ssx3_ps2::kernel::syscall(c, {pc:#010x});");
                    after_call(&mut code, ret);
                    let _ = writeln!(code, "if c.cycles >= c.next_event {{ c.event(); }}");
                    if !inside(ret) {
                        term = Term::Leave(tail_pub(ret, shard, ctx));
                        break;
                    }
                    i = index[&ret];
                    if starts.contains(&ret) {
                        term = Term::Goto(Edge::To(ret));
                        break;
                    }
                    continue;
                }
                Ctl::Branch { target, likely, link } => {
                    count += delay.is_some() as usize;
                    let cond = branch_cond_pub(w);
                    let link_s = if link { format!("c.g[31][0] = sx({:#010x});", pc + 8) } else { String::new() };
                    if link {
                        // bal/bltzal...: a conditional call, then continue at pc+8.
                        let call = format!(
                            "c.pc = {target:#010x}; {}; if c.pc != {:#010x} {{ return; }}",
                            direct_callee_pub(target, shard, ctx),
                            pc + 8
                        );
                        if likely {
                            let _ = writeln!(code, "{link_s} if {cond} {{ {ds} {call} }}");
                        } else {
                            let _ = writeln!(code, "let t = {cond}; {link_s} {ds} if t {{ {call} }}");
                        }
                        term = Term::Goto(edge(pc + 8));
                    } else if likely {
                        term = Term::Cond {
                            prelude: String::new(),
                            cond,
                            then_extra: ds,
                            taken: edge(target),
                            fall: edge(pc + 8),
                        };
                    } else {
                        term = Term::Cond {
                            prelude: format!("let t = {cond};\n{ds}"),
                            cond: "t".into(),
                            then_extra: String::new(),
                            taken: edge(target),
                            fall: edge(pc + 8),
                        };
                    }
                    break;
                }
                Ctl::Jump { target } => {
                    count += delay.is_some() as usize;
                    if !ds.is_empty() {
                        let _ = writeln!(code, "{ds}");
                    }
                    term = Term::Goto(edge(target));
                    break;
                }
                Ctl::JumpReg { rs } => {
                    count += delay.is_some() as usize;
                    let _ = writeln!(code, "let t = c.g[{rs}][0] as u32;");
                    if !ds.is_empty() {
                        let _ = writeln!(code, "{ds}");
                    }
                    if rs == 31 {
                        term = Term::Leave("c.pc = t; return;".into());
                    } else {
                        term = Term::Switch {
                            prelude: String::new(),
                            targets: switch_targets.clone(),
                            other: format!(
                                "ssx3_ps2::machine::jump_out(c, t, {:#010x}, {:#010x}); return;",
                                f.start, f.end
                            ),
                        };
                    }
                    break;
                }
                Ctl::Stop => {
                    term = Term::Leave(if (w >> 26) == 0x10 {
                        "c.pc = c.cop0[14]; return;".into()
                    } else {
                        format!("c.brk({pc:#010x}, {}); return;", (w >> 6) & 0xFFFFF)
                    });
                    break;
                }
            }
        }
        let code = format!("c.cycles += {count};\n{code}");
        blocks.insert(l, Block { code, term });
    }

    // CFG over reachable blocks.
    let succ = |b: &Block| -> Vec<u32> {
        let mut v = Vec::new();
        match &b.term {
            Term::Goto(Edge::To(t)) => v.push(*t),
            Term::Cond { taken, fall, .. } => {
                if let Edge::To(t) = taken {
                    v.push(*t);
                }
                if let Edge::To(t) = fall {
                    v.push(*t);
                }
            }
            Term::Switch { targets, .. } => v.extend(targets.iter().copied()),
            _ => {}
        }
        v
    };
    // RPO via iterative DFS.
    let mut order: Vec<u32> = Vec::new();
    let mut seen: BTreeSet<u32> = BTreeSet::new();
    let mut stack: Vec<(u32, usize)> = vec![(f.start, 0)];
    seen.insert(f.start);
    while let Some((n, k)) = stack.pop() {
        let s = succ(&blocks[&n]);
        if k < s.len() {
            stack.push((n, k + 1));
            let m = s[k];
            if seen.insert(m) {
                stack.push((m, 0));
            }
        } else {
            order.push(n);
        }
    }
    order.reverse();
    let rpo: BTreeMap<u32, usize> = order.iter().enumerate().map(|(i, n)| (*n, i)).collect();
    let mut preds: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
    for &n in &order {
        for s in succ(&blocks[&n]) {
            preds.entry(s).or_default().push(n);
        }
    }
    // Dominators (Cooper-Harvey-Kennedy).
    let mut idom: BTreeMap<u32, u32> = BTreeMap::new();
    idom.insert(f.start, f.start);
    let intersect = |idom: &BTreeMap<u32, u32>, mut a: u32, mut b: u32| {
        while a != b {
            while rpo[&a] > rpo[&b] {
                a = idom[&a];
            }
            while rpo[&b] > rpo[&a] {
                b = idom[&b];
            }
        }
        a
    };
    let mut changed = true;
    while changed {
        changed = false;
        for &n in order.iter().skip(1) {
            let ps: Vec<u32> = preds.get(&n).cloned().unwrap_or_default().into_iter().filter(|p| idom.contains_key(p)).collect();
            let Some(&first) = ps.first() else { continue };
            let mut new = first;
            for &p in &ps[1..] {
                new = intersect(&idom, p, new);
            }
            if idom.get(&n) != Some(&new) {
                idom.insert(n, new);
                changed = true;
            }
        }
    }
    let dominates = |a: u32, mut b: u32| loop {
        if a == b {
            return true;
        }
        let p = idom[&b];
        if p == b {
            return false;
        }
        b = p;
    };
    // Reducibility, loop headers, merge nodes.
    let mut headers: BTreeSet<u32> = BTreeSet::new();
    let mut merges: BTreeSet<u32> = BTreeSet::new();
    for &n in &order {
        let mut forward = 0;
        for &p in preds.get(&n).map(|v| v.as_slice()).unwrap_or(&[]) {
            if rpo[&p] >= rpo[&n] {
                if !dominates(n, p) {
                    return None; // irreducible
                }
                headers.insert(n);
            } else {
                forward += 1;
            }
        }
        if forward >= 2 {
            merges.insert(n);
        }
    }
    let mut children: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
    for &n in order.iter().skip(1) {
        children.entry(idom[&n]).or_default().push(n);
    }

    let mut g = Gen { blocks: &blocks, rpo: &rpo, headers: &headers, merges: &merges, children: &children, ok: true };
    let mut body = String::new();
    g.do_tree(f.start, &mut body, 0);
    if !g.ok {
        return None;
    }

    let name = fn_name(f.start);
    let mut out = String::new();
    if let Some(sym) = &f.name {
        let _ = writeln!(out, "/// `{sym}`");
    }
    let _ = writeln!(out, "pub fn {name}(c: &mut Cpu) {{");
    let has_external = external.range(f.start + 1..f.end).next().is_some();
    if has_external {
        let _ = writeln!(out, "if c.pc != {:#010x} {{ return {name}__entries(c); }}", f.start);
    }
    out.push_str(&body);
    let _ = writeln!(out, "}}");
    Some(out)
}

struct Gen<'a> {
    blocks: &'a BTreeMap<u32, Block>,
    rpo: &'a BTreeMap<u32, usize>,
    headers: &'a BTreeSet<u32>,
    merges: &'a BTreeSet<u32>,
    children: &'a BTreeMap<u32, Vec<u32>>,
    ok: bool,
}

impl Gen<'_> {
    fn do_tree(&mut self, x: u32, out: &mut String, depth: usize) {
        if depth > MAX_NEST {
            self.ok = false;
            return;
        }
        let mut ys: Vec<u32> = self.children.get(&x).cloned().unwrap_or_default().into_iter().filter(|y| self.merges.contains(y)).collect();
        ys.sort_by_key(|y| std::cmp::Reverse(self.rpo[y]));
        if self.headers.contains(&x) {
            let _ = writeln!(out, "'l_{x:x}: loop {{");
            self.node_within(x, &ys, out, depth + 1);
            let _ = writeln!(out, "}}");
        } else {
            self.node_within(x, &ys, out, depth);
        }
    }

    fn node_within(&mut self, x: u32, ys: &[u32], out: &mut String, depth: usize) {
        if depth > MAX_NEST {
            self.ok = false;
            return;
        }
        match ys.split_first() {
            None => {
                let b = &self.blocks[&x];
                out.push_str(&b.code);
                match &b.term {
                    Term::Leave(s) => {
                        let _ = writeln!(out, "{s}");
                    }
                    Term::Goto(e) => self.edge(x, e, out, depth),
                    Term::Switch { prelude, targets, other } => {
                        if !prelude.is_empty() {
                            let _ = writeln!(out, "{prelude}");
                        }
                        let _ = writeln!(out, "match t {{");
                        for &y in targets {
                            let _ = writeln!(out, "{y:#010x} => {{");
                            self.edge(x, &Edge::To(y), out, depth + 1);
                            let _ = writeln!(out, "}}");
                        }
                        let _ = writeln!(out, "_ => {{ {other} }}");
                        let _ = writeln!(out, "}}");
                    }
                    Term::Cond { prelude, cond, then_extra, taken, fall } => {
                        if !prelude.is_empty() {
                            let _ = writeln!(out, "{prelude}");
                        }
                        let _ = writeln!(out, "if {cond} {{");
                        if !then_extra.is_empty() {
                            let _ = writeln!(out, "{then_extra}");
                        }
                        self.edge(x, taken, out, depth + 1);
                        let _ = writeln!(out, "}} else {{");
                        self.edge(x, fall, out, depth + 1);
                        let _ = writeln!(out, "}}");
                    }
                }
            }
            Some((&y, rest)) => {
                let _ = writeln!(out, "'b_{y:x}: {{");
                self.node_within(x, rest, out, depth + 1);
                let _ = writeln!(out, "}}");
                self.do_tree(y, out, depth);
            }
        }
    }

    fn edge(&mut self, x: u32, e: &Edge, out: &mut String, depth: usize) {
        match e {
            Edge::Out(s) => {
                let _ = writeln!(out, "{s}");
            }
            Edge::To(y) => {
                let y = *y;
                if self.rpo[&x] >= self.rpo[&y] {
                    // Back edge: deliver events, then continue the loop.
                    let _ = writeln!(
                        out,
                        "if c.cycles >= c.next_event {{ c.pc = {y:#010x}; c.event(); }} continue 'l_{y:x};"
                    );
                } else if self.merges.contains(&y) {
                    let _ = writeln!(out, "break 'b_{y:x};");
                } else {
                    self.do_tree(y, out, depth + 1);
                }
            }
        }
    }
}
