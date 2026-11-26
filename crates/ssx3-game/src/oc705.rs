//! Staging module oc705 (wave wave239): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc705`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0033_6f30,
    oc705_nash_collide_objects,
    "Nash_CollideObjects",
)];

/// The stack frame of `Nash_CollideObjects`.
const OC705_NASH_FRAME: u32 = 0x100;
/// The query box copied onto the stack (two 16-byte quads).
const OC705_BOX_OFF: u32 = 0x20;
/// The `int` hit counter.
const OC705_COUNT_OFF: u32 = 0x40;
/// `SpatialKey_ClassifyAABB(node, box)` (0x328360), through the table. Its
/// result: 0 (test the cell's objects and children), 1 (nothing), 2 (wholly
/// inside: recurse into the whole subtree).
const OC705_CLASSIFY: u32 = 0x0032_8360;

/// `Cell_CollideObjectEx(ctx, obj)` (0x336d40).
const OC705_COLLIDE_EX: Callee = Callee {
    addr: 0x0033_6d40,
    f: ee::s11::f_00336d40,
};

/// `Cell_CollideObjectsSubtree(cell, ctx)` (0x3410c0).
const OC705_COLLIDE_SUBTREE: Callee = Callee {
    addr: 0x0034_10c0,
    f: ee::s11::f_003410c0,
};

/// `Cell_CollideObjectsRecursive(out, cell, node, ctx, ctx, box)` (0x33dfd8).
const OC705_COLLIDE_RECURSIVE: Callee = Callee {
    addr: 0x0033_dfd8,
    f: ee::s11::f_0033dfd8,
};

/// One of the eight sub-nodes the function walks in order. Node `k` lives at
/// `nash + node_off`, its cell pointer at +0x10. The stack slots, return
/// addresses and cycle charges are the generated body's own.
#[derive(Clone, Copy)]
struct Oc705Phase {
    /// Offset of the node inside the nash object.
    node_off: u32,
    /// Stack byte the recursion writes its flag through (`sp + out_off`).
    out_off: u32,
    /// The register that holds `out` (s0, or s3 for the second node).
    out_reg: usize,
    /// The generated body stores `out` into `out_reg` before testing the cell
    /// (phases 2 to 5); the others store it after the test.
    out_early: bool,
    /// Cycles charged on entry, before the cell test.
    entry_cycles: u64,
    /// Cycles charged before the classify call.
    box_cycles: u64,
    /// Cycles charged when the cell has objects (on top of the 3 always).
    list_else: u64,
    /// Return address of the classify call.
    box_ra: u32,
    /// Return address of the object call (the list loop).
    list_ra: u32,
    /// Back-edge pc of the object loop (event delivery).
    list_head: u32,
    /// Return address of the first child call; child `i` returns at `+0x14 * i`.
    child_ra: u32,
    /// Return address of the recursion.
    rec_ra: u32,
    /// Offset of the next node (its cell is the exit value); `None` returns
    /// the object count instead.
    next_off: Option<u32>,
    /// The last phase sets v0 to the object count before its last child call
    /// and before the recursion (the generated body does this before the
    /// branch); the others set v0 after those calls.
    exit_before_calls: bool,
}

const OC705_PHASES: [Oc705Phase; 8] = [
    Oc705Phase {
        node_off: 0x00,
        out_off: 0x44,
        out_reg: S0,
        out_early: false,
        entry_cycles: 0,
        box_cycles: 7,
        list_else: 2,
        box_ra: 0x0033_6fa8,
        list_ra: 0x0033_6fd0,
        list_head: 0x0033_6fc8,
        child_ra: 0x0033_6ff0,
        rec_ra: 0x0033_70a8,
        next_off: Some(0x14),
        exit_before_calls: false,
    },
    Oc705Phase {
        node_off: 0x14,
        out_off: 0x45,
        out_reg: S3,
        out_early: false,
        entry_cycles: 10,
        box_cycles: 8,
        list_else: 2,
        box_ra: 0x0033_70e8,
        list_ra: 0x0033_7110,
        list_head: 0x0033_7108,
        child_ra: 0x0033_7130,
        rec_ra: 0x0033_71e8,
        next_off: Some(0x28),
        exit_before_calls: false,
    },
    Oc705Phase {
        node_off: 0x28,
        out_off: 0x46,
        out_reg: S0,
        out_early: true,
        entry_cycles: 2,
        box_cycles: 7,
        list_else: 1,
        box_ra: 0x0033_7204,
        list_ra: 0x0033_7228,
        list_head: 0x0033_7220,
        child_ra: 0x0033_7248,
        rec_ra: 0x0033_7300,
        next_off: Some(0x3c),
        exit_before_calls: false,
    },
    Oc705Phase {
        node_off: 0x3c,
        out_off: 0x47,
        out_reg: S0,
        out_early: true,
        entry_cycles: 2,
        box_cycles: 7,
        list_else: 1,
        box_ra: 0x0033_731c,
        list_ra: 0x0033_7340,
        list_head: 0x0033_7338,
        child_ra: 0x0033_7360,
        rec_ra: 0x0033_7418,
        next_off: Some(0x50),
        exit_before_calls: false,
    },
    Oc705Phase {
        node_off: 0x50,
        out_off: 0x48,
        out_reg: S0,
        out_early: true,
        entry_cycles: 2,
        box_cycles: 7,
        list_else: 1,
        box_ra: 0x0033_7434,
        list_ra: 0x0033_7458,
        list_head: 0x0033_7450,
        child_ra: 0x0033_7478,
        rec_ra: 0x0033_7530,
        next_off: Some(0x64),
        exit_before_calls: false,
    },
    Oc705Phase {
        node_off: 0x64,
        out_off: 0x49,
        out_reg: S0,
        out_early: true,
        entry_cycles: 2,
        box_cycles: 7,
        list_else: 1,
        box_ra: 0x0033_754c,
        list_ra: 0x0033_7570,
        list_head: 0x0033_7568,
        child_ra: 0x0033_7590,
        rec_ra: 0x0033_7648,
        next_off: Some(0x78),
        exit_before_calls: false,
    },
    Oc705Phase {
        node_off: 0x78,
        out_off: 0x4a,
        out_reg: S0,
        out_early: false,
        entry_cycles: 2,
        box_cycles: 8,
        list_else: 2,
        box_ra: 0x0033_7668,
        list_ra: 0x0033_7690,
        list_head: 0x0033_7688,
        child_ra: 0x0033_76b0,
        rec_ra: 0x0033_7768,
        next_off: Some(0x8c),
        exit_before_calls: false,
    },
    Oc705Phase {
        node_off: 0x8c,
        out_off: 0x4b,
        out_reg: S0,
        out_early: false,
        entry_cycles: 2,
        box_cycles: 8,
        list_else: 2,
        box_ra: 0x0033_7788,
        list_ra: 0x0033_77b0,
        list_head: 0x0033_77a8,
        child_ra: 0x0033_77d0,
        rec_ra: 0x0033_7888,
        next_off: None,
        exit_before_calls: true,
    },
];

/// The cell pointer the phase reads from `s2` (callee-saved: read it after
/// each call).
#[inline]
fn oc705_s2(c: &Cpu) -> u32 {
    c.g[S2][0] as u32
}

/// The value a phase leaves in v0: the next node's cell, or the object count
/// after the last node.
fn oc705_exit(c: &mut Cpu, nash: u32, sp_: u32, ph: &Oc705Phase) -> u64 {
    match ph.next_off {
        Some(off) => Addr(nash.wrapping_add(off).wrapping_add(0x10)).lw(c),
        None => Addr(sp_.wrapping_add(OC705_COUNT_OFF)).lw(c),
    }
}

/// One node's pass: classify the node's box, then either recurse into the
/// whole subtree (inside), or test the cell's objects and its eight children
/// (partly overlapping). `cell` is the node's cell pointer (already read by
/// the previous phase, in v0).
fn oc705_phase(c: &mut Cpu, nash: u32, ph: &Oc705Phase, cell: u64) -> Flow {
    let sp_ = sp(c);
    c.cycles += ph.entry_cycles;
    if ph.out_early {
        c.g[ph.out_reg][0] = sx(sp_.wrapping_add(ph.out_off));
    }
    if cell == 0 {
        c.cycles += 1;
        c.g[V0][0] = oc705_exit(c, nash, sp_, ph);
        return Ok(());
    }
    let node = nash.wrapping_add(ph.node_off);
    let out = sp_.wrapping_add(ph.out_off);
    let boxp = sp_.wrapping_add(OC705_BOX_OFF);
    c.cycles += ph.box_cycles;
    c.g[ph.out_reg][0] = sx(out);
    c.g[S2][0] = cell;
    c.g[A0][0] = sx(node);
    c.g[A1][0] = sx(boxp);
    call_ptr(c, OC705_CLASSIFY, ph.box_ra)?;
    c.g[V1][0] = c.g[V0][0];
    c.g[V0][0] = 2;
    if c.g[V1][0] != 0 {
        c.cycles += 2;
        if c.g[V1][0] != c.g[V0][0] {
            c.g[V0][0] = oc705_exit(c, nash, sp_, ph);
            return Ok(());
        }
        // Wholly inside: the recursion takes the whole subtree.
        if ph.exit_before_calls {
            c.g[V0][0] = oc705_exit(c, nash, sp_, ph);
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[ph.out_reg][0];
        c.g[T0][0] = c.g[S2][0];
        c.g[T1][0] = sx(node);
        c.g[A1][0] = sx(sp_);
        c.g[A2][0] = sx(sp_);
        c.g[A3][0] = sx(boxp);
        OC705_COLLIDE_RECURSIVE.call(c, ph.rec_ra)?;
        c.cycles += 1;
        c.g[V0][0] = oc705_exit(c, nash, sp_, ph);
        return Ok(());
    }

    // Partly overlapping: the cell's own objects, then its eight children.
    c.cycles += 3;
    c.g[S0][0] = Addr(oc705_s2(c).wrapping_add(0x20)).lw(c);
    if c.g[S0][0] == 0 {
        c.g[A0][0] = Addr(oc705_s2(c)).lw(c);
    } else {
        c.cycles += ph.list_else;
        c.g[A1][0] = c.g[S0][0];
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            OC705_COLLIDE_EX.call(c, ph.list_ra)?;
            let next = Addr(c.g[S0][0] as u32).lw(c);
            c.g[S0][0] = next;
            c.g[A1][0] = c.g[S0][0];
            if c.g[S0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, ph.list_head);
            }
        }
        c.cycles += 1;
        c.g[A0][0] = Addr(oc705_s2(c)).lw(c);
    }
    for i in 0..8u32 {
        c.cycles += 2;
        if c.g[A0][0] == 0 {
            if i == 7 {
                break;
            }
            let kid = Addr(oc705_s2(c).wrapping_add(4 * (i + 1))).lw(c);
            c.g[A0][0] = kid;
            continue;
        }
        c.cycles += if i == 7 { 4 } else { 3 };
        c.g[A1][0] = c.g[S1][0];
        if i == 7 && ph.exit_before_calls {
            c.g[V0][0] = oc705_exit(c, nash, sp_, ph);
        }
        OC705_COLLIDE_SUBTREE.call(c, ph.child_ra.wrapping_add(0x14 * i))?;
        if i == 7 {
            break;
        }
        let kid = Addr(oc705_s2(c).wrapping_add(4 * (i + 1))).lw(c);
        c.g[A0][0] = kid;
    }
    c.g[V0][0] = oc705_exit(c, nash, sp_, ph);
    Ok(())
}

/// `int Nash_CollideObjects(Nash* nash, Query* q, Hit* hits, int max, int
/// full)` (0x336f30, 2444 bytes): collides the query box with the nash's
/// eight sub-nodes in turn. Each node's cell holds its objects (through
/// `Cell_CollideObjectEx`) and eight children (through
/// `Cell_CollideObjectsSubtree`); a node wholly inside the box recurses into
/// its whole subtree. Hits are counted in the stack context. Returns the hit
/// count in v0.
pub fn oc705_nash_collide_objects(c: &mut Cpu) {
    guest(c, oc705_nash_collide_objects_body);
}

fn oc705_nash_collide_objects_body(c: &mut Cpu) -> Flow {
    c.cycles += 26;
    let frame = Frame::open(c, OC705_NASH_FRAME);
    frame.save(c, S1, 0xe0);
    c.g[V0][0] = sx(sp(c).wrapping_add(OC705_COUNT_OFF));
    frame.save(c, S4, 0xb0);
    c.g[S1][0] = c.g[SP][0];
    frame.save(c, S0, 0xf0);
    c.g[S4][0] = c.g[A0][0];
    frame.save(c, S2, 0xd0);
    frame.save(c, S3, 0xc0);
    frame.save(c, S5, 0xa0);
    frame.save(c, S6, 0x90);
    frame.save(c, S7, 0x80);
    frame.save(c, FP, 0x70);
    frame.save_ra(c, 0x60);

    // The query box, copied to the stack; the context fields.
    let query = c.g[A1][0] as u32;
    c.lq(A0, query.wrapping_add(0x20));
    c.lq(V1, query.wrapping_add(0x30));
    let nash = c.g[S4][0] as u32;
    c.g[S2][0] = Addr(nash.wrapping_add(0x10)).lw(c);
    let sp_ = sp(c);
    let a2 = c.g[A2][0];
    c.sw(sp_.wrapping_add(4), a2);
    let a3 = c.g[A3][0];
    c.sw(sp_.wrapping_add(8), a3);
    let v0 = c.g[V0][0];
    c.sw(sp_.wrapping_add(0xc), v0);
    let t0 = c.g[T0][0];
    c.sw(sp_.wrapping_add(0x10), t0);
    c.sq(V1, sp_.wrapping_add(OC705_BOX_OFF));
    c.sq(A0, sp_.wrapping_add(0x30));
    let a1 = c.g[A1][0];
    c.sw(sp_, a1);
    c.sw(sp_.wrapping_add(OC705_COUNT_OFF), 0);

    // Node 0: the nash itself.
    let cell = c.g[S2][0];
    oc705_phase(c, nash, &OC705_PHASES[0], cell)?;

    // Node 1. Its entry also stores the addresses of nodes 6 and 7 on the
    // stack and loads the node pointers the later phases use.
    let v1_six = sx(nash.wrapping_add(0x78));
    c.g[V1][0] = v1_six;
    c.sw(sp_.wrapping_add(0x4c), v1_six);
    c.g[S0][0] = sx(nash.wrapping_add(0x14));
    let v1_seven = sx(nash.wrapping_add(0x8c));
    c.g[V1][0] = v1_seven;
    c.g[S5][0] = sx(nash.wrapping_add(0x28));
    c.g[S6][0] = sx(nash.wrapping_add(0x3c));
    c.g[S7][0] = sx(nash.wrapping_add(0x50));
    c.g[FP][0] = sx(nash.wrapping_add(0x64));
    c.sw(sp_.wrapping_add(0x50), v1_seven);
    let cell = c.g[V0][0];
    oc705_phase(c, nash, &OC705_PHASES[1], cell)?;

    // Nodes 2 to 5.
    for k in 2..6 {
        let cell = c.g[V0][0];
        oc705_phase(c, nash, &OC705_PHASES[k], cell)?;
    }

    // Node 6: its pointer comes from the stack slot, into v0 (after the
    // test on the cell that the previous node left in v0).
    let cell = c.g[V0][0];
    c.g[V0][0] = Addr(sp_.wrapping_add(0x4c)).lw(c);
    oc705_phase(c, nash, &OC705_PHASES[6], cell)?;

    // Node 7: its pointer comes from the stack slot, into v1.
    let cell = c.g[V0][0];
    c.g[V1][0] = Addr(sp_.wrapping_add(0x50)).lw(c);
    oc705_phase(c, nash, &OC705_PHASES[7], cell)?;

    c.cycles += 12;
    frame.restore(c, S0, 0xf0);
    frame.restore(c, S1, 0xe0);
    frame.restore(c, S2, 0xd0);
    frame.restore(c, S3, 0xc0);
    frame.restore(c, S4, 0xb0);
    frame.restore(c, S5, 0xa0);
    frame.restore(c, S6, 0x90);
    frame.restore(c, S7, 0x80);
    frame.restore(c, FP, 0x70);
    frame.restore_ra(c, 0x60);
    frame.close(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate of the callee fakes: they answer only while this word holds
    /// `OC705_MAGIC` (otherwise the body they replaced runs).
    const OC705_CTRL: u32 = DATA + 0x000e_3000;
    const OC705_MAGIC: u32 = 0x4e41_5353;
    /// Classify results, one word per call index (0, 1 or 2).
    const OC705_CLS: u32 = DATA + 0x000e_3100;
    const OC705_NASH: u32 = DATA + 0x1000;
    const OC705_BOX: u32 = DATA + 0x2000;
    const OC705_CELLS: u32 = DATA + 0x4000;
    const OC705_OBJS: u32 = DATA + 0x6000;
    const OC705_KIDS: u32 = DATA + 0x8000;
    /// Offsets of the eight nodes inside the nash object.
    const OC705_NODE_OFFS: [u32; 8] = [0x00, 0x14, 0x28, 0x3c, 0x50, 0x64, 0x78, 0x8c];

    static OC705_PREV_CLASSIFY: AtomicUsize = AtomicUsize::new(0);
    static OC705_PREV_EX: AtomicUsize = AtomicUsize::new(0);
    static OC705_PREV_SUBTREE: AtomicUsize = AtomicUsize::new(0);
    static OC705_PREV_RECURSIVE: AtomicUsize = AtomicUsize::new(0);

    fn oc705_gated(c: &mut Cpu) -> bool {
        c.bus.read32(OC705_CTRL) == OC705_MAGIC
    }

    /// `SpatialKey_ClassifyAABB`: logs the call, answers the seeded result.
    fn oc705_fake_classify(c: &mut Cpu) {
        if !oc705_gated(c) {
            return fakes::fall_back(c, &OC705_PREV_CLASSIFY, ee::s10::f_00328360__generated);
        }
        let n = c.bus.read32(fakes::LOG_COUNT);
        let result = c.bus.read32(OC705_CLS + 4 * (n % 64));
        fakes::fake_call(c, OC705_CLASSIFY);
        c.g[V0][0] = result as u64;
    }

    fn oc705_fake_ex(c: &mut Cpu) {
        if !oc705_gated(c) {
            return fakes::fall_back(c, &OC705_PREV_EX, ee::s11::f_00336d40__generated);
        }
        fakes::fake_call(c, 0x0033_6d40);
    }

    fn oc705_fake_subtree(c: &mut Cpu) {
        if !oc705_gated(c) {
            return fakes::fall_back(c, &OC705_PREV_SUBTREE, ee::s11::f_003410c0__generated);
        }
        fakes::fake_call(c, 0x0034_10c0);
    }

    fn oc705_fake_recursive(c: &mut Cpu) {
        if !oc705_gated(c) {
            return fakes::fall_back(c, &OC705_PREV_RECURSIVE, ee::s11::f_0033dfd8__generated);
        }
        fakes::fake_call(c, 0x0033_dfd8);
    }

    fn oc705_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00328360),
            oc705_fake_classify,
            &OC705_PREV_CLASSIFY,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00336D40),
            oc705_fake_ex,
            &OC705_PREV_EX,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_003410C0),
            oc705_fake_subtree,
            &OC705_PREV_SUBTREE,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_0033DFD8),
            oc705_fake_recursive,
            &OC705_PREV_RECURSIVE,
        );
    }

    /// A cell for node `k`: eight children (some zero) and an object list of
    /// 0 to 3 objects, chained through their first word.
    fn oc705_fill_cell(c: &mut Cpu, r: &mut Rng, cell: u32, k: u32) {
        for i in 0..8u32 {
            let kid = if r.next_u32() % 3 == 0 {
                0
            } else {
                OC705_KIDS + 0x40 * (k * 8 + i)
            };
            c.bus.write32(cell + 4 * i, kid);
        }
        let len = r.next_u32() % 4;
        let mut head = 0;
        for j in (0..len).rev() {
            let obj = OC705_OBJS + 0x40 * k + 0x10 * j;
            c.bus.write32(obj, head);
            head = obj;
        }
        c.bus.write32(cell + 0x20, head);
    }

    #[test]
    fn oc705_nash_collide_objects_matches() {
        oc705_install_fakes();
        check(
            ee::s11::f_00336f30__generated,
            oc705_nash_collide_objects,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0705_0000, 40);
                c.pc = 0x0033_6f30;
                c.bus.write32(OC705_CTRL, OC705_MAGIC);
                c.g[A0][0] = sx(OC705_NASH);
                c.g[A1][0] = sx(OC705_BOX);
                for w in 0..8u32 {
                    c.bus.write32(OC705_BOX + 0x20 + 4 * w, r.next_u32());
                }
                for n in 0..64u32 {
                    c.bus.write32(OC705_CLS + 4 * n, r.next_u32() % 3);
                }
                // Node cells, some zero (the phase skips its work).
                for (k, &off) in OC705_NODE_OFFS.iter().enumerate() {
                    let k = k as u32;
                    let cell = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        OC705_CELLS + 0x100 * k
                    };
                    c.bus.write32(OC705_NASH + off + 0x10, cell);
                    if cell != 0 {
                        oc705_fill_cell(c, &mut r, cell, k);
                    }
                }
            },
        );
    }
}
