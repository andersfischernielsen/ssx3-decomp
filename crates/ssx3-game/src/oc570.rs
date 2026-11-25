//! Staging module oc570 (wave 232, list s2): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc570`.

#![allow(unused_imports)]

use crate::abi::r32;
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0034_0a40,
        cell_gather_objects_subtree,
        "Cell_GatherObjectsSubtree",
    ),
    (
        0x0034_0c20,
        cell_gather_objinst_subtree,
        "Cell_GatherObjInstSubtree",
    ),
    (
        0x0034_0fa0,
        cell_collide_volume_subtree,
        "Cell_CollideVolumeSubtree",
    ),
    (0x0033_6d40, cell_collide_object_ex, "Cell_CollideObjectEx"),
];

/// The callees of the subtree walkers and of `Cell_CollideObjectEx`, through
/// their generated aliases (which go through the override slots).
const CALL_GATHER_OBJECTS: Callee = Callee {
    addr: 0x0033_09d8,
    f: ee::s11::Cell_GatherObjects,
};
const CALL_SUBTREE_OBJECTS: Callee = Callee {
    addr: 0x0034_0a40,
    f: ee::s11::Cell_GatherObjectsSubtree,
};
const CALL_SUBTREE_TYPE2: Callee = Callee {
    addr: 0x0034_0b18,
    f: ee::s11::Cell_GatherType2Subtree,
};
const CALL_SUBTREE_OBJINST: Callee = Callee {
    addr: 0x0034_0c20,
    f: ee::s11::Cell_GatherObjInstSubtree,
};
const CALL_SUBTREE_ALL: Callee = Callee {
    addr: 0x0034_0dc0,
    f: ee::s11::Cell_GatherAllSubtree,
};
const CALL_SUBTREE_VOLUME: Callee = Callee {
    addr: 0x0034_0fa0,
    f: ee::s11::Cell_CollideVolumeSubtree,
};
const CALL_CELL_COLLIDE_INSTANCE: Callee = Callee {
    addr: 0x0033_5960,
    f: ee::s11::Cell_CollideInstance,
};
const CALL_CELL_COLLIDE_OBJECT: Callee = Callee {
    addr: 0x0033_5b90,
    f: ee::s11::Cell_CollideObject,
};
const CALL_COLLIDE_VOLUME_OBJECT: Callee = Callee {
    addr: 0x0033_4888,
    f: ee::s11::Collide_VolumeObject,
};

/// `lw off(reg)`.
fn sub_lw(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = r32(c, reg).wrapping_add(off);
    Addr(a).lw(c)
}

/// `lh off(reg)`.
fn sub_lh(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = r32(c, reg).wrapping_add(off);
    Addr(a).lh(c)
}

/// One child of a subtree walk: child `k` is in `A0` when the block starts.
/// Charges `pre` cycles, restores `s0` from `sp + s0_off` before and after
/// the call when `s0_off` is given (the last child of a walk does), calls
/// `callee` with `A1 = arg` when the child is non-null (returning to `ret`),
/// then loads the next child `A0 = base[next]`.
#[allow(clippy::too_many_arguments)]
fn sub_child(
    c: &mut Cpu,
    pre: u64,
    base: usize,
    next: Option<u32>,
    s0_off: Option<u32>,
    callee: Callee,
    ret: u32,
    arg: usize,
) -> Flow {
    c.cycles += pre;
    if let Some(off) = s0_off {
        c.lq(16, sp(c).wrapping_add(off));
    }
    if c.g[A0][0] != 0 {
        c.cycles += 3;
        c.g[A1][0] = c.g[arg][0];
        callee.call(c, ret)?;
        if let Some(off) = s0_off {
            c.lq(16, sp(c).wrapping_add(off));
        }
    }
    if let Some(off) = next {
        c.g[A0][0] = sub_lw(c, base, off);
    }
    Ok(())
}

/// The eight children of a walked node (`base + 4k`, k = 0..8). `A0` holds
/// child 0 on entry. Child k returns to `ret0 + 0x14k`; child 7 restores `s0`
/// from `s0_off` around its call. `pre0` is child 0's test charge (the others
/// charge 2).
#[allow(clippy::too_many_arguments)]
fn sub_chain8(
    c: &mut Cpu,
    base: usize,
    callee: Callee,
    ret0: u32,
    arg: usize,
    pre0: u64,
    s0_off: u32,
) -> Flow {
    for k in 0..8u32 {
        let last = k == 7;
        let pre = if k == 0 { pre0 } else { 2 };
        let next = if last { None } else { Some(4 * (k + 1)) };
        let restore = if last { Some(s0_off) } else { None };
        sub_child(c, pre, base, next, restore, callee, ret0 + 0x14 * k, arg)?;
    }
    Ok(())
}

/// Appends the object in `A2` to the list pair at `pair` (`pair+0` the array
/// base, `pair+4` the address of the count word), then `A2` = the object's
/// next link. Sets every register the generated append leaves.
fn sub_push(c: &mut Cpu, pair: u32) {
    c.g[A1][0] = Addr(pair.wrapping_add(4)).lw(c);
    c.g[A0][0] = Addr(pair).lw(c);
    c.g[V1][0] = Addr(r32(c, A1)).lw(c);
    c.g[V0][0] = sx(r32(c, V1) << 2);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
    let slot = r32(c, V0);
    let obj = c.g[A2][0];
    Addr(slot).sw(c, obj);
    let count_at = r32(c, A1);
    let count = c.g[V1][0];
    Addr(count_at).sw(c, count);
    c.g[A2][0] = Addr(r32(c, A2)).lw(c);
}

/// One slab test of an object against a box: loads `f1` from `f1_at` and `f0`
/// from `f0_at` (after `cy` cycles), then `c.le.s` or `c.lt.s` with the
/// operands in the order the generated code uses (`lhs_f1`: `f1 OP f0`, else
/// `f0 OP f1`). Leaves the condition bit set; returns it.
fn sub_cmp(c: &mut Cpu, cy: u64, f1_at: u32, f0_at: u32, lt: bool, lhs_f1: bool) -> bool {
    c.cycles += cy;
    c.lwc1(1, f1_at);
    c.lwc1(0, f0_at);
    let f0 = FpuF32(c.f[0]);
    let f1 = FpuF32(c.f[1]);
    let t = match (lt, lhs_f1) {
        (false, false) => f0.le(f1),
        (false, true) => f1.le(f0),
        (true, false) => f0.lt(f1),
        (true, true) => f1.lt(f0),
    };
    crate::abi::set_fcc(c, t);
    t
}

/// The object's box test against the box `bx` (min x/y/z at `obj+0x158..`,
/// max at `obj+0x164..`): six slab tests, each stopping the walk at its first
/// miss, with the cycle charge of each.
fn sub_box_hit(c: &mut Cpu, obj: u32, bx: u32) -> bool {
    sub_cmp(c, 6, obj + 0x164, bx, false, false)
        && sub_cmp(c, 5, bx + 0x10, obj + 0x158, false, false)
        && sub_cmp(c, 5, bx + 4, obj + 0x168, false, true)
        && sub_cmp(c, 5, bx + 0x14, obj + 0x15c, false, false)
        && sub_cmp(c, 5, bx + 8, obj + 0x16c, false, true)
        && sub_cmp(c, 6, bx + 0x18, obj + 0x160, false, false)
}

/// The epilogue of the 0x30-byte walkers: restores `s1` and `ra`, pops.
fn sub_epilogue(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `Cell_GatherObjectsSubtree(tree, ctx)` (0x340a40): the node's objects go to
/// the callback (`Cell_GatherObjects(ctx, tree)`), then its eight children's
/// subtrees, each with the same `ctx`.
pub fn cell_gather_objects_subtree(c: &mut Cpu) {
    guest(c, gather_objects_subtree_body)
}

fn gather_objects_subtree_body(c: &mut Cpu) -> Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    c.g[S1][0] = c.g[A1][0];
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[S0][0];
    CALL_GATHER_OBJECTS.call(c, 0x0034_0a64)?;
    c.g[A0][0] = sub_lw(c, S0, 0);
    sub_chain8(c, S0, CALL_SUBTREE_OBJECTS, 0x0034_0a78, S1, 0, 0x20)?;
    sub_epilogue(c, frame)
}

/// `Cell_GatherType2Subtree(tree, ctx)` (0x340b18): appends the node's
/// type-2 list (`tree+0x28`) to the `ctx` pair at `ctx+0`, then its eight
/// children's subtrees.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn cell_gather_type2_subtree(c: &mut Cpu) {
    guest(c, gather_type2_subtree_body)
}

fn gather_type2_subtree_body(c: &mut Cpu) -> Flow {
    c.cycles += 8;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A0][0];
    c.g[A2][0] = sub_lw(c, S1, 0x28);
    let empty = c.g[A2][0] == 0;
    c.g[S0][0] = c.g[A1][0];
    if !empty {
        c.cycles += 2;
        c.g[A1][0] = sub_lw(c, S0, 4);
        loop {
            c.cycles += 10;
            let pair = r32(c, S0);
            sub_push(c, pair);
            if c.g[A2][0] == 0 {
                break;
            }
            c.g[A1][0] = sub_lw(c, S0, 4);
            if event_due(c) {
                deliver_event_here(c, 0x0034_0b40);
            }
        }
    }
    c.g[A0][0] = sub_lw(c, S1, 0);
    sub_chain8(c, S1, CALL_SUBTREE_TYPE2, 0x0034_0b7c, S0, 3, 0x20)?;
    sub_epilogue(c, frame)
}

/// `Cell_GatherObjInstSubtree(tree, ctx)` (0x340c20): the node's object list
/// (`tree+0x24`) whose boxes overlap the `ctx` box (`ctx+0x1c`) goes to the
/// `ctx+0x10` pair, then the eight children.
pub fn cell_gather_objinst_subtree(c: &mut Cpu) {
    guest(c, gather_objinst_subtree_body)
}

fn gather_objinst_subtree_body(c: &mut Cpu) -> Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A0][0];
    c.g[S1][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[S0][0];
    CALL_GATHER_OBJECTS.call(c, 0x0034_0c44)?;
    c.g[A2][0] = sub_lw(c, S0, 0x24);
    let empty = c.g[A2][0] == 0;
    c.g[A3][0] = sx(r32(c, S1).wrapping_add(0x10));
    if !empty {
        c.cycles += 2;
        c.g[V0][0] = sub_lw(c, A3, 0xc);
        loop {
            let obj = r32(c, A2);
            let bx = r32(c, V0);
            let hit = sub_box_hit(c, obj, bx);
            c.g[V1][0] = hit as u64;
            c.cycles += 2;
            if c.g[V1][0] == 0 {
                c.g[A2][0] = sub_lw(c, A2, 0);
            } else {
                c.cycles += 9;
                let pair = r32(c, A3);
                sub_push(c, pair);
            }
            c.cycles += 2;
            if c.g[A2][0] == 0 {
                break;
            }
            c.g[V0][0] = sub_lw(c, A3, 0xc);
            if event_due(c) {
                deliver_event_here(c, 0x0034_0c58);
            }
        }
    }
    c.g[A0][0] = sub_lw(c, S0, 0);
    sub_chain8(c, S0, CALL_SUBTREE_OBJINST, 0x0034_0d20, S1, 3, 0x20)?;
    sub_epilogue(c, frame)
}

/// `Cell_GatherAllSubtree(tree, ctx)` (0x340dc0): the object list with the
/// box test (`tree+0x24` into `ctx+0x10`), the all-list (`tree+0x28` into
/// `ctx+0x20`, no test), then the eight children.
/// Not installed: no passing test (see the wave notes).
#[allow(dead_code)]
pub fn cell_gather_all_subtree(c: &mut Cpu) {
    guest(c, gather_all_subtree_body)
}

fn gather_all_subtree_body(c: &mut Cpu) -> Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    c.g[S1][0] = c.g[A1][0];
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = c.g[S0][0];
    CALL_GATHER_OBJECTS.call(c, 0x0034_0de4)?;
    c.g[A2][0] = sub_lw(c, S0, 0x24);
    let empty = c.g[A2][0] == 0;
    c.g[A3][0] = sx(r32(c, S1).wrapping_add(0x10));
    if !empty {
        c.cycles += 2;
        c.g[V0][0] = sub_lw(c, A3, 0xc);
        loop {
            let obj = r32(c, A2);
            let bx = r32(c, V0);
            let hit = sub_box_hit(c, obj, bx);
            c.g[V1][0] = hit as u64;
            c.cycles += 2;
            if c.g[V1][0] == 0 {
                c.g[A2][0] = sub_lw(c, A2, 0);
            } else {
                c.cycles += 9;
                let pair = r32(c, A3);
                sub_push(c, pair);
            }
            c.cycles += 2;
            if c.g[A2][0] == 0 {
                break;
            }
            c.g[V0][0] = sub_lw(c, A3, 0xc);
            if event_due(c) {
                deliver_event_here(c, 0x0034_0df8);
            }
        }
    }
    c.cycles += 3;
    c.g[A2][0] = sub_lw(c, S0, 0x28);
    let empty2 = c.g[A2][0] == 0;
    c.g[A3][0] = sx(r32(c, S1).wrapping_add(0x20));
    if !empty2 {
        c.cycles += 2;
        c.g[A1][0] = sub_lw(c, A3, 4);
        loop {
            c.cycles += 10;
            let pair = r32(c, A3);
            sub_push(c, pair);
            if c.g[A2][0] == 0 {
                break;
            }
            c.g[A1][0] = sub_lw(c, A3, 4);
            if event_due(c) {
                deliver_event_here(c, 0x0034_0ec0);
            }
        }
    }
    c.cycles += 3;
    c.g[A0][0] = sub_lw(c, S0, 0);
    sub_chain8(c, S0, CALL_SUBTREE_ALL, 0x0034_0efc, S1, 3, 0x20)?;
    sub_epilogue(c, frame)
}

/// `Cell_CollideVolumeSubtree(tree, ctx)` (0x340fa0): `Cell_CollideInstance`
/// on each instance of `tree+0x24` and `Cell_CollideObject` on each object of
/// `tree+0x20` (both with `A0 = ctx`), then the eight children.
pub fn cell_collide_volume_subtree(c: &mut Cpu) {
    guest(c, collide_volume_subtree_body)
}

fn collide_volume_subtree_body(c: &mut Cpu) -> Flow {
    c.cycles += 9;
    let frame = Frame::open(c, 0x40);
    frame.save(c, S1, 0x20);
    frame.save(c, S2, 0x10);
    frame.save_ra(c, 0);
    c.g[S2][0] = c.g[A0][0];
    frame.save(c, S0, 0x30);
    c.g[S0][0] = sub_lw(c, S2, 0x24);
    let empty = c.g[S0][0] == 0;
    c.g[S1][0] = c.g[A1][0];
    if !empty {
        c.cycles += 1;
        c.g[A1][0] = c.g[S0][0];
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            CALL_CELL_COLLIDE_INSTANCE.call(c, 0x0034_0fd0)?;
            c.g[S0][0] = sub_lw(c, S0, 0);
            let more = c.g[S0][0] != 0;
            c.g[A1][0] = c.g[S0][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0034_0fc8);
            }
        }
    }
    c.cycles += 3;
    c.g[S0][0] = sub_lw(c, S2, 0x20);
    if c.g[S0][0] == 0 {
        c.g[A0][0] = sub_lw(c, S2, 0);
    } else {
        c.cycles += 2;
        c.g[A1][0] = c.g[S0][0];
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            CALL_CELL_COLLIDE_OBJECT.call(c, 0x0034_0ff8)?;
            c.g[S0][0] = sub_lw(c, S0, 0);
            let more = c.g[S0][0] != 0;
            c.g[A1][0] = c.g[S0][0];
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0034_0ff0);
                }
            } else {
                c.cycles += 1;
                c.g[A0][0] = sub_lw(c, S2, 0);
                break;
            }
        }
    }
    sub_chain8(c, S2, CALL_SUBTREE_VOLUME, 0x0034_1018, S1, 2, 0x30)?;
    c.cycles += 5;
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `Cell_CollideObjectEx` (`StaticObjectCallback_intersectinstance`, 0x336d40):
/// the callback for a static-object or dynamic-instance node (`flags` at
/// `+8`). Static nodes (0x20) test their box against the query (`ctx+0`), dynamic
/// ones (0x40) ask their active sub-object for its box through the vtable at
/// `+0xc`, and a hit is collected into `ctx+0xc` by `Collide_VolumeObject`.
pub fn cell_collide_object_ex(c: &mut Cpu) {
    guest(c, collide_object_ex_body)
}

fn collide_object_ex_body(c: &mut Cpu) -> Flow {
    c.cycles += 14;
    let frame = Frame::open(c, 0x40);
    c.g[V1][0] = 3;
    frame.save(c, S0, 0x30);
    frame.save(c, S1, 0x20);
    c.g[S0][0] = c.g[A1][0];
    frame.save(c, S2, 0x10);
    c.g[S1][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[S2][0] = 1;
    c.g[A1][0] = sub_lw(c, S0, 8);
    c.g[V0][0] = sub_lw(c, S1, 0x10);
    c.g[A0][0] = c.g[A1][0] & 0x20;
    let no_static = c.g[A0][0] == 0;
    if c.g[V0][0] != 0 {
        c.g[S2][0] = c.g[V1][0];
    }
    if !no_static {
        let hit = cx_static_test(c);
        c.cycles += 2;
        if !hit {
            return cx_exit(c, frame);
        }
        c.cycles += 2;
        c.g[V0][0] = sub_lw(c, S1, 0xc);
        return cx_collect(c, frame);
    }
    c.cycles += 3;
    c.g[V0][0] = c.g[A1][0] & 0x40;
    if c.g[V0][0] == 0 {
        return cx_exit(c, frame);
    }
    c.cycles += 8;
    c.g[A1][0] = sub_lw(c, S0, 0xc);
    c.g[V1][0] = sub_lw(c, A1, 0xc);
    c.g[A0][0] = sub_lh(c, V1, 0x160);
    c.g[V0][0] = sub_lw(c, V1, 0x164);
    let target = r32(c, V0);
    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
    call_ptr(c, target, 0x0033_6e30)?;
    if c.g[V0][0] == 0 {
        return cx_exit(c, frame);
    }
    // The second box, from the vtable's other slot, tested against the query.
    c.cycles += 13;
    c.g[A1][0] = sub_lw(c, S0, 0xc);
    c.g[V1][0] = sub_lw(c, A1, 0xc);
    c.g[A0][0] = sub_lh(c, V1, 0x168);
    c.g[V0][0] = sub_lw(c, V1, 0x16c);
    let target = r32(c, V0);
    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
    call_ptr(c, target, 0x0033_6e50)?;
    c.g[V1][0] = sub_lw(c, S1, 0);
    let bx = r32(c, V0);
    let q = r32(c, V1);
    c.g[A0][0] = 0;
    let mut ok = sub_cmp(c, 0, bx, q + 0x20, true, true);
    if ok {
        ok = sub_cmp(c, 5, q + 0x30, bx + 0x10, true, true);
    }
    if ok {
        ok = sub_cmp(c, 5, q + 0x24, bx + 4, true, false);
    }
    if ok {
        ok = sub_cmp(c, 5, q + 0x34, bx + 0x14, true, true);
    }
    if ok {
        ok = sub_cmp(c, 5, q + 0x28, bx + 8, true, false);
    }
    if ok {
        ok = sub_cmp(c, 6, q + 0x38, bx + 0x18, true, true);
    }
    if ok {
        c.g[A0][0] = 1;
    }
    c.cycles += 2;
    if c.g[A0][0] == 0 {
        return cx_exit(c, frame);
    }
    c.cycles += 1;
    c.g[V0][0] = sub_lw(c, S1, 0xc);
    cx_collect(c, frame)
}

/// The static box test of `Cell_CollideObjectEx` (`V0` = the query `ctx+0`):
/// six `c.lt.s` tests; leaves `V1` = the result and returns it.
fn cx_static_test(c: &mut Cpu) -> bool {
    c.g[V0][0] = sub_lw(c, S1, 0);
    let node = r32(c, S0);
    let q = r32(c, V0);
    let mut hit = sub_cmp(c, 7, node + 0x60, q + 0x20, true, true);
    if hit {
        hit = sub_cmp(c, 5, q + 0x30, node + 0x6c, true, true);
    }
    if hit {
        hit = sub_cmp(c, 5, q + 0x24, node + 0x64, true, false);
    }
    if hit {
        hit = sub_cmp(c, 5, q + 0x34, node + 0x70, true, true);
    }
    if hit {
        hit = sub_cmp(c, 5, q + 0x28, node + 0x68, true, false);
    }
    if hit {
        hit = sub_cmp(c, 6, q + 0x38, node + 0x74, true, true);
    }
    c.g[V1][0] = hit as u64;
    hit
}

/// `Cell_CollideObjectEx`'s collect: `Collide_VolumeObject(query, node, mode,
/// box)`, then the hit count at `ctx+0xc` grows by its result.
fn cx_collect(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 15;
    c.g[A1][0] = c.g[S0][0];
    c.g[V1][0] = sub_lw(c, S1, 8);
    c.g[T0][0] = c.g[S2][0];
    c.g[A3][0] = Addr(r32(c, V0)).lw(c);
    c.g[T1][0] = sub_lw(c, S1, 4);
    c.g[A2][0] = sx(r32(c, A3) << 7);
    c.g[A0][0] = sub_lw(c, S1, 0);
    c.g[A3][0] = sx(r32(c, V1).wrapping_sub(r32(c, A3)));
    c.g[A2][0] = sx(r32(c, T1).wrapping_add(r32(c, A2)));
    CALL_COLLIDE_VOLUME_OBJECT.call(c, 0x0033_6f08)?;
    c.g[A0][0] = sub_lw(c, S1, 0xc);
    c.g[V1][0] = Addr(r32(c, A0)).lw(c);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    let slot = r32(c, A0);
    let total = c.g[V1][0];
    Addr(slot).sw(c, total);
    frame.restore(c, S0, 0x30);
    cx_epilogue(c, frame)
}

/// Exit without collecting: restore `s0` and return.
fn cx_exit(c: &mut Cpu, frame: Frame) -> Flow {
    frame.restore(c, S0, 0x30);
    cx_epilogue(c, frame)
}

fn cx_epilogue(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 5;
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word in the machine's RAM: the fakes below forward to the real
    /// bodies unless it holds `OC570_MAGIC`.
    const OC570_GATE: u32 = DATA + 0x8_0000;
    const OC570_MAGIC: u32 = 0x5570_0570;
    /// The root node of the walk and its eight leaves.
    const OC570_ROOT: u32 = DATA + 0x2000;
    /// The callback context (`ctx`): array pairs at +0, +0x10, +0x20 and the box at +0x1c.
    const OC570_CTX: u32 = DATA + 0x3000;
    const OC570_ARR0: u32 = DATA + 0x3100;
    const OC570_CNT0: u32 = DATA + 0x3300;
    const OC570_ARR1: u32 = DATA + 0x3400;
    const OC570_CNT1: u32 = DATA + 0x3600;
    const OC570_BOX: u32 = DATA + 0x3700;
    const OC570_ARR2: u32 = DATA + 0x3800;
    const OC570_CNT2: u32 = DATA + 0x3a00;
    /// Object records (0x100 bytes each).
    const OC570_OBJS: u32 = DATA + 0x1_0000;

    static OC570_PREV_CGO: AtomicUsize = AtomicUsize::new(0);
    static OC570_PREV_CI: AtomicUsize = AtomicUsize::new(0);
    static OC570_PREV_CO: AtomicUsize = AtomicUsize::new(0);
    static OC570_PREV_CVO: AtomicUsize = AtomicUsize::new(0);

    /// Leaf `k` of the root.
    fn oc570_leaf(k: u32) -> u32 {
        DATA + 0x2100 + 0x100 * k
    }

    /// `Cell_GatherObjects`, gated: logged, returns to `ra`.
    fn oc570_cgo_fake(c: &mut Cpu) {
        if c.bus.read32(OC570_GATE) != OC570_MAGIC {
            return fakes::fall_back(c, &OC570_PREV_CGO, ee::s11::f_003309d8__generated);
        }
        fakes::fake_call(c, 0x0033_09d8);
    }

    /// `Cell_CollideInstance`, gated, logged.
    fn oc570_ci_fake(c: &mut Cpu) {
        if c.bus.read32(OC570_GATE) != OC570_MAGIC {
            return fakes::fall_back(c, &OC570_PREV_CI, ee::s11::f_00335960__generated);
        }
        fakes::fake_call(c, 0x0033_5960);
    }

    /// `Cell_CollideObject`, gated, logged.
    fn oc570_co_fake(c: &mut Cpu) {
        if c.bus.read32(OC570_GATE) != OC570_MAGIC {
            return fakes::fall_back(c, &OC570_PREV_CO, ee::s11::f_00335b90__generated);
        }
        fakes::fake_call(c, 0x0033_5b90);
    }

    /// `Collide_VolumeObject`, gated, logged.
    fn oc570_cvo_fake(c: &mut Cpu) {
        if c.bus.read32(OC570_GATE) != OC570_MAGIC {
            return fakes::fall_back(c, &OC570_PREV_CVO, ee::s11::f_00334888__generated);
        }
        fakes::fake_call(c, 0x0033_4888);
    }

    fn oc570_install() {
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_003309D8),
            oc570_cgo_fake,
            &OC570_PREV_CGO,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00335960),
            oc570_ci_fake,
            &OC570_PREV_CI,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00335B90),
            oc570_co_fake,
            &OC570_PREV_CO,
        );
        fakes::hook(
            addr_of_mut!(ee::s11::OVERRIDE_00334888),
            oc570_cvo_fake,
            &OC570_PREV_CVO,
        );
    }

    /// A list of up to three objects (random boxes), headed at `head_at`.
    fn oc570_list(c: &mut Cpu, r: &mut Rng, head_at: u32, objs: &mut u32) {
        let count = r.next_u32() % 4;
        let mut next = 0;
        for _ in 0..count {
            let o = OC570_OBJS + 0x100 * *objs;
            *objs += 1;
            c.bus.write32(o, next);
            for f in 0..6 {
                c.bus.write32(o + 0x158 + 4 * f, r.f32(4.0));
            }
            next = o;
        }
        c.bus.write32(head_at, next);
    }

    /// The walk's fixture: a root with random children (leaves or zero),
    /// leaves with no children, object lists on every node, the callback
    /// context with its arrays and counts, and the box.
    fn oc570_tree(c: &mut Cpu, r: &mut Rng) {
        let mut objs = 0;
        for k in 0..8u32 {
            let kid = if r.next_u32() % 3 == 0 {
                0
            } else {
                oc570_leaf(k)
            };
            c.bus.write32(OC570_ROOT + 4 * k, kid);
        }
        oc570_list(c, r, OC570_ROOT + 0x20, &mut objs);
        oc570_list(c, r, OC570_ROOT + 0x24, &mut objs);
        oc570_list(c, r, OC570_ROOT + 0x28, &mut objs);
        for k in 0..8u32 {
            let leaf = oc570_leaf(k);
            for j in 0..8u32 {
                c.bus.write32(leaf + 4 * j, 0);
            }
            oc570_list(c, r, leaf + 0x20, &mut objs);
            oc570_list(c, r, leaf + 0x24, &mut objs);
            oc570_list(c, r, leaf + 0x28, &mut objs);
        }
        c.bus.write32(OC570_CTX, OC570_ARR0);
        c.bus.write32(OC570_CTX + 4, OC570_CNT0);
        c.bus.write32(OC570_CTX + 0x10, OC570_ARR1);
        c.bus.write32(OC570_CTX + 0x14, OC570_CNT1);
        c.bus.write32(OC570_CTX + 0x1c, OC570_BOX);
        c.bus.write32(OC570_CTX + 0x20, OC570_ARR2);
        c.bus.write32(OC570_CTX + 0x24, OC570_CNT2);
        c.bus.write32(OC570_CNT0, r.next_u32() % 4);
        c.bus.write32(OC570_CNT1, r.next_u32() % 4);
        c.bus.write32(OC570_CNT2, r.next_u32() % 4);
        for k in 0..6u32 {
            c.bus.write32(OC570_BOX + 4 * k, r.f32(4.0));
        }
    }

    #[test]
    fn cell_gather_objects_subtree_matches() {
        oc570_install();
        check(
            ee::s11::f_00340a40__generated,
            cell_gather_objects_subtree,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0570_0a40, 30);
                c.bus.write32(OC570_GATE, OC570_MAGIC);
                oc570_tree(c, &mut r);
                c.g[A0][0] = sx(OC570_ROOT);
                c.g[A1][0] = sx(OC570_CTX);
                c.pc = 0x0034_0a40;
            },
        );
    }

    #[test]
    fn cell_gather_objinst_subtree_matches() {
        oc570_install();
        check(
            ee::s11::f_00340c20__generated,
            cell_gather_objinst_subtree,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0570_0c20, 30);
                c.bus.write32(OC570_GATE, OC570_MAGIC);
                oc570_tree(c, &mut r);
                c.g[A0][0] = sx(OC570_ROOT);
                c.g[A1][0] = sx(OC570_CTX);
                c.pc = 0x0034_0c20;
            },
        );
    }

    #[test]
    fn cell_collide_volume_subtree_matches() {
        oc570_install();
        check(
            ee::s11::f_00340fa0__generated,
            cell_collide_volume_subtree,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0570_0fa0, 30);
                c.bus.write32(OC570_GATE, OC570_MAGIC);
                oc570_tree(c, &mut r);
                c.g[A0][0] = sx(OC570_ROOT);
                c.g[A1][0] = sx(OC570_CTX);
                c.pc = 0x0034_0fa0;
            },
        );
    }

    /// `Cell_CollideObjectEx`: the static node, the dynamic node (through the
    /// vtable's two slots, which the fake callees answer) and the collect.
    const OC570_CX_CTX: u32 = DATA + 0x6c00;
    const OC570_CX_QRY: u32 = DATA + 0x7000;
    const OC570_CX_NODE: u32 = DATA + 0x6800;
    const OC570_CX_ACT: u32 = DATA + 0x6400;
    const OC570_CX_VT: u32 = DATA + 0x6600;
    const OC570_CX_OUT: u32 = DATA + 0x7400;

    #[test]
    fn cell_collide_object_ex_matches() {
        oc570_install();
        check(
            ee::s11::f_00336d40__generated,
            cell_collide_object_ex,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0570_6d40, 6);
                c.bus.write32(OC570_GATE, OC570_MAGIC);
                // The active sub-object's vtable: slot 0x160 and slot 0x168.
                c.bus
                    .write16(OC570_CX_VT + 0x160, (r.next_u32() as u16) & !3);
                c.bus.write32(OC570_CX_VT + 0x164, fakes::FAKE);
                c.bus
                    .write16(OC570_CX_VT + 0x168, (r.next_u32() as u16) & !3);
                c.bus.write32(OC570_CX_VT + 0x16c, fakes::FAKE + 0x10);
                c.bus.write32(OC570_CX_ACT + 0xc, OC570_CX_VT);
                // The node: flags (static 0x20, dynamic 0x40, or both),
                // the active sub-object, and the static box at +0x60..+0x74.
                let flags = [0x20u32, 0x40, 0x60, 0x00][(seed % 4) as usize];
                c.bus.write32(OC570_CX_NODE + 8, flags);
                c.bus.write32(OC570_CX_NODE + 0xc, OC570_CX_ACT);
                for off in (0x60..0x78).step_by(4) {
                    c.bus.write32(OC570_CX_NODE + off, r.f32(3.0));
                }
                // The context: the query, two words, the output word, the mode.
                c.bus.write32(OC570_CX_CTX, OC570_CX_QRY);
                c.bus.write32(OC570_CX_CTX + 4, r.next_u32());
                c.bus.write32(OC570_CX_CTX + 8, r.next_u32());
                c.bus.write32(OC570_CX_CTX + 0xc, OC570_CX_OUT);
                c.bus.write32(OC570_CX_CTX + 0x10, r.next_u32() & 1);
                c.bus.write32(OC570_CX_OUT, r.next_u32());
                for off in (0x20..0x3c).step_by(4) {
                    c.bus.write32(OC570_CX_QRY + off, r.f32(3.0));
                }
                c.g[A0][0] = sx(OC570_CX_CTX);
                c.g[A1][0] = sx(OC570_CX_NODE);
                c.pc = 0x0033_6d40;
            },
        );
    }
}
