//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements, each with a differential test against the generated body.
//!
//! - `CameraTriggerMan_CollectOctreeTriggersInBox` (0x00174848): the camera
//!   trigger manager's octree box query. It is handed
//!     * `a0`  – passed on unchanged to the recursion,
//!     * `a1`  – the trigger manager (its `[0]` is the active-trigger list,
//!                `[4]` the object that owns the trigger predicates),
//!     * `a2`  – the same manager (used by the child/subtree walks),
//!     * `a3`  – the box: three min/max float pairs at +0/+4/+8, +0x10/+0x14/+0x18,
//!     * `t0`  – the octree node (`+0x28` its trigger list, `+0..+0x1c` its
//!                eight children),
//!     * `t1`  – the cell descriptor (`+0` level, `+4/+8/+0xc` integer cell
//!                coordinates).
//!
//!   It walks the node's own trigger list (every entry of type 4 whose object
//!   passes the two virtual tests is added to the active list), then tests the
//!   box against the twelve half-cell intervals it derives from the descriptor
//!   and visits the eight children: a child that does not overlap the box is
//!   skipped, a child that is wholly inside it is collected whole, anything in
//!   between is descended into with a doubled cell and the next level down.
//!   The recursion is handed `t1 = sp`, so the frame's first four words *are*
//!   the child descriptor.
//!
//! The replacement keeps the original's registers, cycle charges and call
//! order, including every call's return address (they are the resume points a
//! host-thread unwind returns to).

use crate::call::{guest, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::{call_indirect, GuestFn};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0017_4848,
    camera_trigger_man_collect_octree_triggers_in_box,
    "CameraTriggerMan_CollectOctreeTriggersInBox",
)];

/// `ActiveCameraTriggerList_Find` (first hit or 0).
const ACTIVE_FIND: u32 = 0x0016_c6b0;
/// `ActiveCameraTriggerList_Add`.
const ACTIVE_ADD: u32 = 0x0016_c718;
/// `CameraTriggerMan_CollectOctreeNodeTriggers` (the inlined subtree walk).
const COLLECT_NODE: u32 = 0x0017_6890;
/// The function itself (the recursion is a direct call of address 0x174848).
const IN_BOX: u32 = 0x0017_4848;

// ---------------------------------------------------------------------------
// Frame slots. The first four words double as the child descriptor the
// recursion receives in `t1`, so they are written by the box tests.

/// `t1`'s level - 1 (the child cell's level).
const CELL_TYPE: u32 = 0x00;
/// `2 * t1.u`.
const CELL_U: u32 = 0x04;
/// `2 * t1.v`.
const CELL_V: u32 = 0x08;
/// `2 * t1.w`.
const CELL_W: u32 = 0x0c;
/// The original `a0`.
const ARG0: u32 = 0x10;
/// The original `t0` (the node whose list and children are walked).
const NODE: u32 = 0x14;

// The twelve box/cell tests, in the order the original computes them:
// "the low/high half of the cell on this axis overlaps the box", then
// "the low/high half lies wholly inside the box".

const U_LO: u32 = 0x18;
const U_HI: u32 = 0x1c;
const V_LO: u32 = 0x20;
const V_HI: u32 = 0x24;
const W_LO: u32 = 0x28;
const W_HI: u32 = 0x2c;
const U_LO_IN: u32 = 0x30;
const U_HI_IN: u32 = 0x34;
const V_LO_IN: u32 = 0x38;
const V_HI_IN: u32 = 0x3c;
const W_LO_IN: u32 = 0x40;
const W_HI_IN: u32 = 0x44;

/// `lw` of a frame slot.
macro_rules! load_slot {
    ($c:ident, $f:ident, $off:expr) => {{
        let a = $f.at($c, $off);
        $c.lw(a)
    }};
}

/// `sw` of a value into a frame slot.
macro_rules! store_slot {
    ($c:ident, $f:ident, $off:expr, $v:expr) => {{
        let a = $f.at($c, $off);
        let v: u64 = $v;
        $c.sw(a, v);
    }};
}

/// One of the six "the half cell lies wholly inside the box" flags: the
/// original's `c_lt_s` pair with the box minimum, then with the box maximum
/// (4 cycles, +5/+1 when the flag is set).
macro_rules! inside_test {
    ($c:ident, $f:ident, $slot:ident, $box:ident, $lo_f:expr, $hi_f:expr, $max_off:expr,
     $cell_end_f:expr, $v:ident) => {{
        $c.cycles += 4;
        $c.c_lt_s($lo_f, $hi_f);
        let t = !$c.fcc();
        store_slot!($c, $f, $slot, 0);
        if !t {
            $c.cycles += 5;
            $c.lwc1(0, ($c.g[$box][0] as u32).wrapping_add($max_off));
            $c.c_lt_s($cell_end_f, 0);
            let t = !$c.fcc();
            $c.g[$v][0] = 1;
            if !t {
                $c.cycles += 1;
                store_slot!($c, $f, $slot, $c.g[$v][0]);
            }
        }
    }};
}

/// One of the child cases 1..7: the null and overlap tests, then either the
/// "collect the whole subtree" path or a recursive descent.
///
/// The expansion ends after the recursive call, so the caller's block provides
/// the case's tail (the `+1` cycle and the result slot). The collect path sets
/// `$skip` and leaves, skipping that tail, as the original does.
macro_rules! child_case {
    (
        $c:ident, $f:ident, $skip:ident, $ctr:ident, $up:expr, $child:expr,
        $f0:ident, $f1:ident, $f2:ident, $i0:ident, $i1:ident, $i2:ident,
        $site:ident, $init:expr, $s6s7:expr, $flag:ident, $finish:expr
    ) => {{
        'done: {
            $c.cycles += 5;
            $c.g[V1][0] = load_slot!($c, $f, NODE);
            $c.g[V0][0] = sx(($c.g[V0][0] as u32).wrapping_add($up));
            let node = $c.g[V1][0] as u32;
            $c.g[S4][0] = Addr(node.wrapping_add($child)).lw($c);
            let t = $c.g[S4][0] == 0;
            let ctr = $c.g[V0][0];
            let a = $f.at($c, $ctr);
            $c.sw(a, ctr);
            if t {
                break 'done;
            }
            $c.cycles += 3;
            $c.g[V0][0] = load_slot!($c, $f, $f0);
            let t = $c.g[V0][0] == 0;
            $c.g[V1][0] = load_slot!($c, $f, $f1);
            if t {
                break 'done;
            }
            $c.cycles += 2;
            let t = $c.g[V1][0] == 0;
            $c.g[V0][0] = load_slot!($c, $f, $f2);
            if t {
                break 'done;
            }
            $c.cycles += 2;
            let t = $c.g[V0][0] == 0;
            $c.g[V1][0] = load_slot!($c, $f, $i0);
            if t {
                break 'done;
            }
            'outer: {
                'inner: {
                    $c.cycles += 2;
                    let t = $c.g[V1][0] == 0;
                    $c.g[V0][0] = load_slot!($c, $f, $i1);
                    if t {
                        break 'inner;
                    }
                    $c.cycles += 2;
                    let t = $c.g[V0][0] == 0;
                    $c.g[V1][0] = load_slot!($c, $f, $i2);
                    if t {
                        break 'inner;
                    }
                    $c.cycles += 2;
                    let t = $c.g[V1][0] == 0;
                    $c.g[A0][0] = load_slot!($c, $f, ARG0);
                    if t {
                        break 'outer;
                    }
                    collect_subtree($c, $f, S5, $flag, $s6s7, $init, &$site, $finish)?;
                    $skip = true;
                    break 'done;
                }
                $c.cycles += 1;
                $c.g[A0][0] = load_slot!($c, $f, ARG0);
            }
            descend($c, &$site)?;
        }
    }};
}

/// What the last child-collect block leaves behind (the original loads the
/// case's result slot in every child case, but restores `s0` in the last).
#[derive(Clone, Copy)]
enum Finish {
    /// `v0 = lw(sp + slot)`.
    LoadV0(u32),
    /// `lq 16, sp + 0xe0` (the epilogue's `s0` restore, hoisted into the case).
    RestoreS0,
}

/// One child site: the return address of every call the original makes there,
/// and the loop head events are delivered at.
struct Site {
    /// The trigger-list loop's head (`c.pc` for an event delivery).
    head: u32,
    /// Return addresses of the two virtual trigger tests.
    test1: u32,
    test2: u32,
    /// Return addresses of `ActiveCameraTriggerList_Find` / `_Add`.
    find: u32,
    add: u32,
    /// Return addresses of the eight `CollectOctreeNodeTriggers` calls.
    children: [u32; 8],
    /// Return address of the recursive `CollectOctreeTriggersInBox` call.
    next: u32,
}

/// The root node's own list (0x1748a0).
const SITE_TOP: Site = Site {
    head: 0x0017_48a0,
    test1: 0x0017_48dc,
    test2: 0x0017_48ec,
    find: 0x0017_4900,
    add: 0x0017_4918,
    children: [0; 8],
    next: 0,
};

/// Child +0x00 (u lo, v lo, w lo).
const SITE_A: Site = Site {
    head: 0x0017_4d08,
    test1: 0x0017_4d44,
    test2: 0x0017_4d54,
    find: 0x0017_4d68,
    add: 0x0017_4d80,
    children: [
        0x0017_4da0, 0x0017_4db4, 0x0017_4dc8, 0x0017_4ddc, 0x0017_4df0, 0x0017_4e04, 0x0017_4e18,
        0x0017_4e2c,
    ],
    next: 0x0017_4e50,
};

/// Child +0x04 (u lo, v lo, w hi).
const SITE_B: Site = Site {
    head: 0x0017_4eb0,
    test1: 0x0017_4eec,
    test2: 0x0017_4efc,
    find: 0x0017_4f10,
    add: 0x0017_4f28,
    children: [
        0x0017_4f48, 0x0017_4f5c, 0x0017_4f70, 0x0017_4f84, 0x0017_4f98, 0x0017_4fac, 0x0017_4fc0,
        0x0017_4fd4,
    ],
    next: 0x0017_4ff8,
};

/// Child +0x0c (u lo, v hi, w hi).
const SITE_C: Site = Site {
    head: 0x0017_5058,
    test1: 0x0017_5094,
    test2: 0x0017_50a4,
    find: 0x0017_50b8,
    add: 0x0017_50d0,
    children: [
        0x0017_50f0, 0x0017_5104, 0x0017_5118, 0x0017_512c, 0x0017_5140, 0x0017_5154, 0x0017_5168,
        0x0017_517c,
    ],
    next: 0x0017_51a0,
};

/// Child +0x08 (u lo, v hi, w lo).
const SITE_D: Site = Site {
    head: 0x0017_5200,
    test1: 0x0017_523c,
    test2: 0x0017_524c,
    find: 0x0017_5260,
    add: 0x0017_5278,
    children: [
        0x0017_5298, 0x0017_52ac, 0x0017_52c0, 0x0017_52d4, 0x0017_52e8, 0x0017_52fc, 0x0017_5310,
        0x0017_5324,
    ],
    next: 0x0017_5348,
};

/// Child +0x18 (u hi, v hi, w lo).
const SITE_E: Site = Site {
    head: 0x0017_53a8,
    test1: 0x0017_53e4,
    test2: 0x0017_53f4,
    find: 0x0017_5408,
    add: 0x0017_5420,
    children: [
        0x0017_5440, 0x0017_5454, 0x0017_5468, 0x0017_547c, 0x0017_5490, 0x0017_54a4, 0x0017_54b8,
        0x0017_54cc,
    ],
    next: 0x0017_54f0,
};

/// Child +0x1c (u hi, v hi, w hi).
const SITE_F: Site = Site {
    head: 0x0017_5550,
    test1: 0x0017_558c,
    test2: 0x0017_559c,
    find: 0x0017_55b0,
    add: 0x0017_55c8,
    children: [
        0x0017_55e8, 0x0017_55fc, 0x0017_5610, 0x0017_5624, 0x0017_5638, 0x0017_564c, 0x0017_5660,
        0x0017_5674,
    ],
    next: 0x0017_5698,
};

/// Child +0x14 (u hi, v lo, w hi).
const SITE_G: Site = Site {
    head: 0x0017_56f8,
    test1: 0x0017_5734,
    test2: 0x0017_5744,
    find: 0x0017_5758,
    add: 0x0017_5770,
    children: [
        0x0017_5790, 0x0017_57a4, 0x0017_57b8, 0x0017_57cc, 0x0017_57e0, 0x0017_57f4, 0x0017_5808,
        0x0017_581c,
    ],
    next: 0x0017_5840,
};

/// Child +0x10 (u hi, v lo, w lo): the last case, which leaves through the
/// epilogue instead of passing a result slot to a following case.
const SITE_H: Site = Site {
    head: 0x0017_58a8,
    test1: 0x0017_58e0,
    test2: 0x0017_58f0,
    find: 0x0017_5904,
    add: 0x0017_591c,
    children: [
        0x0017_593c, 0x0017_5950, 0x0017_5964, 0x0017_5978, 0x0017_598c, 0x0017_59a0, 0x0017_59b4,
        0x0017_59c8,
    ],
    next: 0x0017_59ec,
};

// ---------------------------------------------------------------------------
// The function.

/// `void CameraTriggerMan_CollectOctreeTriggersInBox(...)` (0x00174848).
pub fn camera_trigger_man_collect_octree_triggers_in_box(c: &mut Cpu) {
    guest(c, collect_in_box);
}

/// The body of [`camera_trigger_man_collect_octree_triggers_in_box`].
fn collect_in_box(c: &mut Cpu) -> Flow {
    c.cycles += 19;
    let frame = Frame::open(c, 0xf0);
    frame.save(c, S4, 0xa0);
    frame.save(c, S5, 0x90);
    c.g[S4][0] = c.g[T1][0];
    frame.save(c, S6, 0x80);
    c.g[S5][0] = c.g[A2][0];
    frame.save(c, S7, 0x70);
    c.g[S6][0] = c.g[A3][0];
    frame.save(c, S0, 0xe0);
    frame.save(c, S1, 0xd0);
    frame.save(c, S2, 0xc0);
    frame.save(c, FP, 0x60);
    frame.save_ra(c, 0x50);
    let node = c.g[T0][0];
    store_slot!(c, frame, NODE, node);
    let arg0 = c.g[A0][0];
    store_slot!(c, frame, ARG0, arg0);
    frame.save(c, S3, 0xb0);
    c.g[S3][0] = Addr(node as u32).off(0x28).lw(c);
    let empty = c.g[S3][0] == 0;
    // `s7` keeps the `a1` manager: the root walk uses it, the child walks use
    // the `a2` one (`s5`).
    c.g[S7][0] = c.g[A1][0];
    if !empty {
        walk_trigger_list(c, S3, S2, S7, FP, false, 3, &SITE_TOP, false)?;
    }

    'epilogue: {
        'boxed: {
            c.cycles += 4;
            c.g[A1][0] = Addr(c.g[S4][0] as u32).lw(c);
            c.g[V0][0] = 0xb;
            let t = c.g[A1][0] == c.g[V0][0];
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7e));
            if t {
                break 'boxed;
            }

            // ---- the cell's size and its doubled coordinates (0x1749bc):
            //      size = 2^(level-1), and the frame's descriptor words for
            //      the child cell.
            c.cycles += 28;
            c.g[A0][0] = Addr(c.g[S4][0] as u32).off(4).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 23);
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x631c));
            c.mtc1(2, 4); // f4 = the cell size
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            c.mtc1(4, 0); // f0 = 2u
            c.cvt_s_w(0, 0);
            c.mtc1(2, 1); // f1 = 2u + 1
            c.cvt_s_w(1, 1);
            c.lwc1(3, c.g[S6][0] as u32);
            c.g[V1][0] = Addr(c.g[S4][0] as u32).off(0xc).lw(c);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
            c.g[V0][0] = Addr(c.g[S4][0] as u32).off(8).lw(c);
            c.sub_s(0, 0, 2); // f0 = 2u - e
            c.add_s(1, 1, 2); // f1 = 2u + 1 + e
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
            store_slot!(c, frame, CELL_TYPE, c.g[A1][0]);
            store_slot!(c, frame, U_LO, 0);
            c.mul_s(15, 0, 4); // f15 = low edge of the low half
            c.mul_s(16, 1, 4); // f16 = high edge of the low half
            store_slot!(c, frame, CELL_V, c.g[V0][0]);
            store_slot!(c, frame, CELL_W, c.g[V1][0]);
            c.c_lt_s(3, 16);
            let t = !c.fcc();
            store_slot!(c, frame, CELL_U, c.g[A0][0]);
            if !t {
                c.cycles += 5;
                c.lwc1(0, (c.g[S6][0] as u32).wrapping_add(0x10));
                c.c_lt_s(15, 0);
                let t = !c.fcc();
                c.g[V0][0] = 1;
                if !t {
                    c.cycles += 1;
                    store_slot!(c, frame, U_LO, c.g[V0][0]);
                }
            }

            // ---- the high half of u (0x174a1c)
            c.cycles += 18;
            c.g[V0][0] = load_slot!(c, frame, CELL_U);
            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x6318));
            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
            store_slot!(c, frame, U_HI, 0);
            c.mtc1(3, 1); // f1 = 2u + 2
            c.cvt_s_w(1, 1);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.mtc1(2, 0); // f0 = 2u + 1
            c.cvt_s_w(0, 0);
            c.lwc1(2, c.g[S6][0] as u32);
            c.add_s(1, 1, 3);
            c.mov_s(13, 2); // f13 = box.min.u
            c.sub_s(0, 0, 3);
            c.mul_s(14, 1, 4);
            c.c_lt_s(2, 14);
            let t = !c.fcc();
            c.mul_s(12, 0, 4);
            if !t {
                c.cycles += 5;
                c.lwc1(0, (c.g[S6][0] as u32).wrapping_add(0x10));
                c.c_lt_s(12, 0);
                let t = !c.fcc();
                c.g[V1][0] = 1;
                if !t {
                    c.cycles += 1;
                    store_slot!(c, frame, U_HI, c.g[V1][0]);
                }
            }

            // ---- neither half of u overlaps the box: nothing to do (0x174a34)
            c.cycles += 3;
            c.g[V0][0] = load_slot!(c, frame, U_LO);
            let t = c.g[V0][0] != 0;
            c.g[V1][0] = load_slot!(c, frame, CELL_V);
            if !t {
                c.cycles += 3;
                c.g[V1][0] = load_slot!(c, frame, U_HI);
                let t = c.g[V1][0] == 0;
                c.g[V1][0] = load_slot!(c, frame, CELL_V);
                if t {
                    break 'boxed;
                }
            }

            // ---- the low half of v (0x174a90)
            c.cycles += 17;
            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x6314));
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            store_slot!(c, frame, V_LO, 0);
            c.mtc1(2, 1); // f1 = 2v + 1
            c.cvt_s_w(1, 1);
            c.g[A0][0] = c.g[V1][0];
            c.mtc1(3, 0); // f0 = 2v
            c.cvt_s_w(0, 0);
            c.lwc1(2, (c.g[S6][0] as u32).wrapping_add(0x4));
            c.add_s(1, 1, 3);
            c.mov_s(6, 2); // f6 = box.min.v
            c.sub_s(0, 0, 3);
            c.mul_s(11, 1, 4);
            c.c_lt_s(2, 11);
            let t = !c.fcc();
            c.mul_s(10, 0, 4);
            if !t {
                c.cycles += 5;
                c.lwc1(0, (c.g[S6][0] as u32).wrapping_add(0x14));
                c.c_lt_s(10, 0);
                let t = !c.fcc();
                c.g[V0][0] = 1;
                if !t {
                    c.cycles += 1;
                    store_slot!(c, frame, V_LO, c.g[V0][0]);
                }
            }

            // ---- the high half of v (0x174ae4)
            c.cycles += 15;
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2));
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x6310));
            c.mtc1(3, 1); // f1 = 2v + 2
            c.cvt_s_w(1, 1);
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            c.mtc1(2, 0); // f0 = 2v + 1
            c.cvt_s_w(0, 0);
            store_slot!(c, frame, V_HI, 0);
            c.add_s(1, 1, 2);
            c.sub_s(0, 0, 2);
            c.mul_s(9, 1, 4);
            c.c_lt_s(6, 9);
            let t = !c.fcc();
            c.mul_s(8, 0, 4);
            if !t {
                c.cycles += 5;
                c.lwc1(0, (c.g[S6][0] as u32).wrapping_add(0x14));
                c.c_lt_s(8, 0);
                let t = !c.fcc();
                c.g[V1][0] = 1;
                if !t {
                    c.cycles += 1;
                    store_slot!(c, frame, V_HI, c.g[V1][0]);
                }
            }

            // ---- neither half of v overlaps the box (0x174afc)
            c.cycles += 3;
            c.g[V0][0] = load_slot!(c, frame, V_LO);
            let t = c.g[V0][0] != 0;
            c.g[V1][0] = load_slot!(c, frame, CELL_W);
            if !t {
                c.cycles += 3;
                c.g[V1][0] = load_slot!(c, frame, V_HI);
                let t = c.g[V1][0] == 0;
                c.g[V1][0] = load_slot!(c, frame, CELL_W);
                if t {
                    break 'boxed;
                }
            }

            // ---- the low half of w (0x174b58)
            c.cycles += 17;
            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x630c));
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
            store_slot!(c, frame, W_LO, 0);
            c.mtc1(2, 1); // f1 = 2w + 1
            c.cvt_s_w(1, 1);
            c.g[A0][0] = c.g[V1][0];
            c.mtc1(3, 0); // f0 = 2w
            c.cvt_s_w(0, 0);
            c.lwc1(2, (c.g[S6][0] as u32).wrapping_add(0x8));
            c.add_s(1, 1, 3);
            c.mov_s(5, 2); // f5 = box.min.w
            c.sub_s(0, 0, 3);
            c.mul_s(7, 1, 4);
            c.c_lt_s(2, 7);
            let t = !c.fcc();
            c.mul_s(3, 0, 4);
            if !t {
                c.cycles += 5;
                c.lwc1(0, (c.g[S6][0] as u32).wrapping_add(0x18));
                c.c_lt_s(3, 0);
                let t = !c.fcc();
                c.g[V0][0] = 1;
                if !t {
                    c.cycles += 1;
                    store_slot!(c, frame, W_LO, c.g[V0][0]);
                }
            }

            // ---- the high half of w (0x174bac)
            c.cycles += 15;
            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2));
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x6308));
            c.mtc1(3, 1); // f1 = 2w + 2
            c.cvt_s_w(1, 1);
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
            c.mtc1(2, 0); // f0 = 2w + 1
            c.cvt_s_w(0, 0);
            store_slot!(c, frame, W_HI, 0);
            c.add_s(1, 1, 2);
            c.sub_s(0, 0, 2);
            c.mul_s(1, 1, 4);
            c.c_lt_s(5, 1);
            let t = !c.fcc();
            c.mul_s(2, 0, 4);
            if !t {
                c.cycles += 5;
                c.lwc1(0, (c.g[S6][0] as u32).wrapping_add(0x18));
                c.c_lt_s(2, 0);
                let t = !c.fcc();
                c.g[V1][0] = 1;
                if !t {
                    c.cycles += 1;
                    store_slot!(c, frame, W_HI, c.g[V1][0]);
                }
            }

            // ---- neither half of w overlaps the box: the query is empty
            //      (0x174bc0; jumps straight to the epilogue).
            c.cycles += 3;
            c.g[V0][0] = load_slot!(c, frame, W_LO);
            let t = c.g[V0][0] != 0;
            c.g[V1][0] = load_slot!(c, frame, W_HI);
            if !t {
                c.cycles += 2;
                let t = c.g[V1][0] == 0;
                frame.restore(c, S0, 0xe0);
                if t {
                    break 'epilogue;
                }
            }

            // ---- the six "half cell wholly inside the box" tests
            //      (0x174be8 .. 0x174cb0).
            inside_test!(c, frame, U_LO_IN, S6, 13, 15, 0x10, 16, V0);
            inside_test!(c, frame, U_HI_IN, S6, 13, 12, 0x10, 14, V1);
            inside_test!(c, frame, V_LO_IN, S6, 6, 10, 0x14, 11, V0);
            inside_test!(c, frame, V_HI_IN, S6, 6, 8, 0x14, 9, V1);
            inside_test!(c, frame, W_LO_IN, S6, 5, 3, 0x18, 7, V0);
            inside_test!(c, frame, W_HI_IN, S6, 5, 2, 0x18, 1, V1);

            // ---- the eight children, in the original's visiting order.
            //      Child 0 (0x174e54) loads its flags in a slightly different
            //      order and keeps no counter, so it is written out.
            'case_a: {
                'done: {
                    c.cycles += 4;
                    c.g[V0][0] = load_slot!(c, frame, NODE);
                    let n = c.g[V0][0] as u32;
                    c.g[S4][0] = Addr(n).lw(c);
                    let t = c.g[S4][0] == 0;
                    c.g[V1][0] = load_slot!(c, frame, U_LO);
                    if t {
                        break 'done;
                    }
                    c.cycles += 2;
                    let t = c.g[V1][0] == 0;
                    c.g[V0][0] = load_slot!(c, frame, V_LO);
                    if t {
                        break 'done;
                    }
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0;
                    c.g[V1][0] = load_slot!(c, frame, W_LO);
                    if t {
                        break 'done;
                    }
                    c.cycles += 2;
                    let t = c.g[V1][0] == 0;
                    c.g[V0][0] = load_slot!(c, frame, U_LO_IN);
                    if t {
                        break 'done;
                    }
                    'outer: {
                        'inner: {
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0;
                            c.g[V1][0] = load_slot!(c, frame, V_LO_IN);
                            if t {
                                break 'inner;
                            }
                            c.cycles += 2;
                            let t = c.g[V1][0] == 0;
                            c.g[V0][0] = load_slot!(c, frame, W_LO_IN);
                            if t {
                                break 'inner;
                            }
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0;
                            c.g[A0][0] = load_slot!(c, frame, ARG0);
                            if t {
                                break 'outer;
                            }
                            collect_subtree(
                                c,
                                frame,
                                S5,
                                FP,
                                false,
                                3,
                                &SITE_A,
                                Finish::LoadV0(CELL_W),
                            )?;
                            break 'case_a;
                        }
                        c.cycles += 1;
                        c.g[A0][0] = load_slot!(c, frame, ARG0);
                    }
                    descend(c, &SITE_A)?;
                }
                c.cycles += 1;
                c.g[V0][0] = load_slot!(c, frame, CELL_W);
            }

            'case_b: {
                let mut skip = false;
                child_case!(
                    c, frame, skip, CELL_W, 1, 0x04, U_LO, V_LO, W_HI, U_LO_IN, V_LO_IN,
                    W_HI_IN, SITE_B, 2, false, FP, Finish::LoadV0(CELL_V)
                );
                if !skip {
                    c.cycles += 1;
                    c.g[V0][0] = load_slot!(c, frame, CELL_V);
                }
            }

            'case_c: {
                let mut skip = false;
                child_case!(
                    c, frame, skip, CELL_V, 1, 0x0c, U_LO, V_HI, W_HI, U_LO_IN, V_HI_IN,
                    W_HI_IN, SITE_C, 2, false, FP, Finish::LoadV0(CELL_W)
                );
                if !skip {
                    c.cycles += 1;
                    c.g[V0][0] = load_slot!(c, frame, CELL_W);
                }
            }

            'case_d: {
                let mut skip = false;
                child_case!(
                    c, frame, skip, CELL_W, 0xffff_ffff, 0x08, U_LO, V_HI, W_LO, U_LO_IN,
                    V_HI_IN, W_LO_IN, SITE_D, 2, false, FP, Finish::LoadV0(CELL_U)
                );
                if !skip {
                    c.cycles += 1;
                    c.g[V0][0] = load_slot!(c, frame, CELL_U);
                }
            }

            'case_e: {
                let mut skip = false;
                child_case!(
                    c, frame, skip, CELL_U, 1, 0x18, U_HI, V_HI, W_LO, U_HI_IN, V_HI_IN,
                    W_LO_IN, SITE_E, 2, false, FP, Finish::LoadV0(CELL_W)
                );
                if !skip {
                    c.cycles += 1;
                    c.g[V0][0] = load_slot!(c, frame, CELL_W);
                }
            }

            'case_f: {
                let mut skip = false;
                child_case!(
                    c, frame, skip, CELL_W, 1, 0x1c, U_HI, V_HI, W_HI, U_HI_IN, V_HI_IN,
                    W_HI_IN, SITE_F, 2, false, FP, Finish::LoadV0(CELL_V)
                );
                if !skip {
                    c.cycles += 1;
                    c.g[V0][0] = load_slot!(c, frame, CELL_V);
                }
            }

            'case_g: {
                let mut skip = false;
                child_case!(
                    c, frame, skip, CELL_V, 0xffff_ffff, 0x14, U_HI, V_LO, W_HI, U_HI_IN,
                    V_LO_IN, W_HI_IN, SITE_G, 2, false, FP, Finish::LoadV0(CELL_W)
                );
                if !skip {
                    c.cycles += 1;
                    c.g[V0][0] = load_slot!(c, frame, CELL_W);
                }
            }

            // The last case has no result slot of its own. Its early exits and
            // its descend end in the epilogue's `+1; lq 16` funnel, but its
            // collect path jumps *past* that funnel: the final child check
            // restores `s0` itself (the original's `break 'b_1759f0`).
            let mut skip = false;
            child_case!(
                c, frame, skip, CELL_W, 0xffff_ffff, 0x10, U_HI, V_LO, W_LO, U_HI_IN, V_LO_IN,
                W_LO_IN, SITE_H, 4, true, S6, Finish::RestoreS0
            );
            if skip {
                break 'epilogue;
            }
            break 'boxed;
        }
        c.cycles += 1;
        frame.restore(c, S0, 0xe0);
    }

    c.cycles += 11;
    frame.restore(c, S1, 0xd0);
    frame.restore(c, S2, 0xc0);
    frame.restore(c, S3, 0xb0);
    frame.restore(c, S4, 0xa0);
    frame.restore(c, S5, 0x90);
    frame.restore(c, S6, 0x80);
    frame.restore(c, S7, 0x70);
    frame.restore(c, FP, 0x60);
    frame.restore_ra(c, 0x50);
    frame.close(c);
    Ok(())
}

/// The recursive descent into a child cell (0x174e50 and its siblings).
fn descend(c: &mut Cpu, site: &Site) -> Flow {
    c.cycles += 6;
    c.g[T0][0] = c.g[S4][0];
    c.g[A1][0] = c.g[S7][0];
    c.g[A2][0] = c.g[S5][0];
    c.g[A3][0] = c.g[S6][0];
    c.g[RA][0] = sx(site.next);
    c.g[T1][0] = c.g[SP][0];
    c.pc = IN_BOX;
    ee::s02::CameraTriggerMan_CollectOctreeTriggersInBox(c);
    if c.pc != site.next {
        return Err(Unwound);
    }
    Ok(())
}

/// The "child cell wholly inside the box" path (0x174d90 and siblings):
/// collect the node's own triggers, then everything under its eight children
/// (the compiler inlined `CollectOctreeNodeTriggers` here).
fn collect_subtree(
    c: &mut Cpu,
    frame: Frame,
    mgr: usize,
    flag: usize,
    s6s7: bool,
    init_cycles: u64,
    site: &Site,
    finish: Finish,
) -> Flow {
    c.cycles += 3;
    let node = c.g[S4][0] as u32;
    c.g[S2][0] = Addr(node).off(0x28).lw(c);
    if c.g[S2][0] == 0 {
        c.g[A0][0] = Addr(node).lw(c);
    } else {
        walk_trigger_list(c, S2, S3, mgr, flag, s6s7, init_cycles, site, true)?;
    }
    collect_child_subtrees(c, frame, mgr, &site.children, finish)
}

/// `CollectOctreeNodeTriggers`'s tail: call it for each non-null child of the
/// node in `s4` (the first entry is already in `a0`), each with its own return
/// address.
fn collect_child_subtrees(
    c: &mut Cpu,
    frame: Frame,
    mgr: usize,
    rets: &[u32; 8],
    finish: Finish,
) -> Flow {
    let node = c.g[S4][0] as u32;
    for (i, ret) in rets.iter().enumerate().take(7) {
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[RA][0] = sx(*ret);
            c.g[A1][0] = c.g[mgr][0];
            c.pc = COLLECT_NODE;
            ee::s02::f_00176890(c);
            if c.pc != *ret {
                return Err(Unwound);
            }
        }
        let off = 4 * (i as u32 + 1);
        c.g[A0][0] = Addr(node.wrapping_add(off)).lw(c);
    }
    c.cycles += 2;
    let t = c.g[A0][0] == 0;
    match finish {
        Finish::LoadV0(off) => {
            c.g[V0][0] = load_slot!(c, frame, off);
        }
        Finish::RestoreS0 => frame.restore(c, S0, 0xe0),
    }
    if !t {
        c.cycles += 4;
        c.g[RA][0] = sx(rets[7]);
        c.g[A1][0] = c.g[mgr][0];
        c.pc = COLLECT_NODE;
        ee::s02::f_00176890(c);
        if c.pc != rets[7] {
            return Err(Unwound);
        }
        match finish {
            Finish::LoadV0(off) => {
                c.g[V0][0] = load_slot!(c, frame, off);
            }
            Finish::RestoreS0 => frame.restore(c, S0, 0xe0),
        }
    }
    Ok(())
}

/// One walk of a node's trigger list (the loop at 0x1748a0 / 0x174d08 and its
/// siblings): for every entry of type 4, run the object's two virtual tests
/// and, if both pass, mark the trigger active in the manager's list (or add it
/// there when it is not listed yet).
///
/// `list` is the register holding the entry cursor and `obj` the one the
/// trigger object is loaded into — the root walk keeps the cursor in `s3` and
/// the object in `s2`, the child walks the other way round. `mgr` is the
/// manager's register (`s7` at the root, `s5` below) and `flag` the register
/// holding 1 (the "active" marker); `s6s7` is the last site's variant, which
/// compares against `s7` and keeps `v1` untouched.
fn walk_trigger_list(
    c: &mut Cpu,
    list: usize,
    obj: usize,
    mgr: usize,
    flag: usize,
    s6s7: bool,
    init_cycles: u64,
    site: &Site,
    exit_loads_a0: bool,
) -> Flow {
    c.cycles += init_cycles;
    if s6s7 {
        c.g[S7][0] = 4;
        c.g[S6][0] = 1;
    } else {
        c.g[FP][0] = 1;
    }
    let p = c.g[list][0] as u32;
    c.g[V0][0] = Addr(p).off(8).lw(c);
    loop {
        let trigger = if s6s7 {
            c.cycles += 2;
            c.g[V0][0] == c.g[S7][0]
        } else {
            c.cycles += 3;
            c.g[V1][0] = 4;
            c.g[V0][0] == c.g[V1][0]
        };
        if trigger {
            c.cycles += 18;
            let entry = c.g[list][0] as u32;
            c.g[obj][0] = Addr(entry).off(0xc).lw(c);
            let m = c.g[mgr][0] as u32;
            c.g[A0][0] = Addr(m).off(4).lw(c);
            c.g[S0][0] = Addr(c.g[obj][0] as u32).off(8).lw(c);
            c.g[A2][0] = Addr(c.g[A0][0] as u32).off(0x6c0).lw(c);
            c.g[S1][0] = Addr(c.g[S0][0] as u32).off(0x24).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
            c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0x28).lh(c);
            c.g[A1][0] = Addr(c.g[S1][0] as u32).off(0x38).lh(c);
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x2c).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            let target = c.g[V0][0] as u32;
            c.g[RA][0] = sx(site.test1);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.pc = target;
            call_indirect(c, target);
            if c.pc != site.test1 {
                return Err(Unwound);
            }
            c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x3c).lw(c);
            c.g[A0][0] = c.g[S0][0];
            let target = c.g[V1][0] as u32;
            c.g[RA][0] = sx(site.test2);
            c.g[A1][0] = c.g[V0][0];
            c.pc = target;
            call_indirect(c, target);
            if c.pc != site.test2 {
                return Err(Unwound);
            }
            if c.g[V0][0] == 0 {
                let q = c.g[list][0] as u32;
                c.g[list][0] = Addr(q).lw(c);
            } else {
                'added: {
                    'found: {
                        c.cycles += 5;
                        let m = c.g[mgr][0] as u32;
                        c.g[A0][0] = Addr(m).lw(c);
                        c.g[RA][0] = sx(site.find);
                        c.g[A1][0] = c.g[obj][0];
                        c.pc = ACTIVE_FIND;
                        ee::s02::f_0016c6b0(c);
                        if c.pc != site.find {
                            return Err(Unwound);
                        }
                        if c.g[V0][0] == 0 {
                            let m = c.g[mgr][0] as u32;
                            c.g[A0][0] = Addr(m).lw(c);
                            break 'found;
                        }
                        c.cycles += 2;
                        let p = c.g[V0][0] as u32;
                        let v = c.g[flag][0];
                        Addr(p).off(0xc).sw(c, v);
                        break 'added;
                    }
                    c.cycles += 2;
                    c.g[RA][0] = sx(site.add);
                    c.g[A1][0] = c.g[obj][0];
                    c.pc = ACTIVE_ADD;
                    ee::s02::f_0016c718(c);
                    if c.pc != site.add {
                        return Err(Unwound);
                    }
                }
                c.cycles += 1;
                let q = c.g[list][0] as u32;
                c.g[list][0] = Addr(q).lw(c);
            }
        } else {
            let q = c.g[list][0] as u32;
            c.g[list][0] = Addr(q).lw(c);
        }
        c.cycles += 2;
        if c.g[list][0] == 0 {
            // The child sites charge one more cycle and reload `a0` from the
            // node they walked; the root one just leaves the loop.
            if exit_loads_a0 {
                c.cycles += 1;
                let n = c.g[S4][0] as u32;
                c.g[A0][0] = Addr(n).lw(c);
            }
            return Ok(());
        }
        let q = c.g[list][0] as u32;
        c.g[V0][0] = Addr(q).off(8).lw(c);
        if event_due(c) {
            deliver_event_here(c, site.head);
        }
    }
}

// ---------------------------------------------------------------------------
// Test fixtures.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{compare, fakes, put_words, DATA, GAME_GP};

    /// The fixture objects. They must stay clear of `fakes::LOG`
    /// (0x0090_0000, one 0x220-byte record per fake call) and of the pool
    /// regions below: `DATA + 0x60_0000` is above all of them.
    const BASE: u32 = DATA + 0x60_0000;
    /// The cell descriptor (`t1`).
    const DESC: u32 = BASE;
    /// The octree node (`t0`), its child node, and their lists.
    const NODE: u32 = BASE + 0x040;
    const NODE2: u32 = BASE + 0x080;
    const LIST: u32 = BASE + 0x0c0;
    /// A trigger: its object, the object it hangs off, and that object's
    /// second-predicate slot.
    const TRIG: u32 = BASE + 0x100;
    const SUB: u32 = BASE + 0x140;
    const VT2: u32 = BASE + 0x180;
    /// The manager (`a1`/`a2`), its object, and that object's first-predicate
    /// slot at `WOBJ + 0x6c0`.
    const MGR: u32 = BASE + 0x1c0;
    const WOBJ: u32 = BASE + 0x200;
    const VT1: u32 = WOBJ + 0x6c0;
    /// The box (`a3`).
    const BOX: u32 = BASE + 0x900;
    /// The active-list head slot and one list node.
    const HEAD: u32 = BASE + 0x940;
    const ACTIVE: u32 = BASE + 0x980;

    fn w(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    /// The BELibrary singleton (a zeroed dummy so nothing reads the disc) and
    /// the world (no riders).
    fn globals(c: &mut Cpu) {
        const BE: u32 = DATA + 0x20_0000;
        const WORLD: u32 = DATA + 0x21_0000;
        w(c, GAME_GP.wrapping_sub(0x1f04), BE);
        w(c, GAME_GP.wrapping_sub(0x848), WORLD);
        let game = WORLD + 0x1000;
        w(c, WORLD + 0x84, game);
        let race = WORLD + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
    }

    /// The engine's size-class pool allocator: `gp + 0xdd8` ready plus eight
    /// free nodes per size class (a zeroed heap walks garbage). The trigger
    /// list's `Add` allocates one 0x10-byte node.
    fn pool(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w(c, GAME_GP.wrapping_add(0xdd8), 1);
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let pool = DATA + 0x40_0000 + 0x4_0000 * i;
            let base = DATA + 0x80_0000 + 0x4000 * i;
            for n in 0..8u32 {
                let node = base + 0x400 * n;
                w(c, node, if n == 7 { 0 } else { node + 0x400 });
            }
            w(c, pool + head, base);
            w(c, GAME_GP + slot, pool);
        }
    }

    /// A bounded octree (one child level), a trigger list of at most one entry,
    /// predicate slots that are either fakes or `ActiveCameraTriggerList_Find`
    /// (to get a 0 result), and a box whose overlap with the cell varies with
    /// the seed so every case and every descend/collect branch is taken.
    fn setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x1748_4848, 8);
        globals(c);
        pool(c);
        // The four `.lit4` floats the box tests read (all 0.2f).
        put_words(c, GAME_GP.wrapping_sub(0x631c), &[0x3e4c_cccd; 4]);

        // Levels 0..2 (cell size 0.5/1/2) keep the cell intervals inside the
        // box's range; level 11 short-circuits the whole query.
        let level = match seed % 3 {
            0 => 11,
            _ => (seed / 3) % 3,
        };
        w(c, DESC, level);
        for i in 1..4 {
            w(c, DESC + 4 * i, r.next_u32() % 4);
        }

        // The box. Every fifth seed asks for everything (so the "wholly
        // inside" collect path is reached).
        let huge = seed % 5 == 0;
        for i in 0..3 {
            let (lo, hi) = if huge {
                (-1000.0f32, 1000.0f32)
            } else {
                let lo = f32::from_bits(r.f32(8.0));
                (lo, lo + (r.next_u32() % 4096) as f32 * 0.01)
            };
            w(c, BOX + 4 * i, lo.to_bits());
            w(c, BOX + 0x10 + 4 * i, hi.to_bits());
        }

        // The node's eight children (bit `i` of the seed) and its list.
        for i in 0..8u32 {
            let child = if (seed >> i) & 1 != 0 { NODE2 } else { 0 };
            w(c, NODE + 4 * i, child);
        }
        w(c, NODE + 0x28, if seed % 4 == 0 { 0 } else { LIST });
        for i in 0..8u32 {
            w(c, NODE2 + 4 * i, 0);
        }
        w(c, NODE2 + 0x28, 0);

        // One list entry: a trigger (type 4) or something the walk skips.
        w(c, LIST, 0);
        w(c, LIST + 4, 0);
        w(c, LIST + 8, if seed % 8 < 2 { 3 } else { 4 });
        w(c, LIST + 0xc, TRIG);

        // trigger -> sub -> the second predicate; the manager's object holds
        // the first one.
        w(c, TRIG + 8, SUB);
        w(c, SUB + 0x24, VT2);
        w(c, VT2 + 0x38, 0);
        // The second predicate returning 0 (a real empty-list `Find`) makes the
        // walk skip the trigger; a fake returns a pointer (Find/Add path).
        w(c, VT2 + 0x3c, if seed % 2 == 0 { ACTIVE_FIND } else { fakes::FAKE });
        w(c, MGR, HEAD);
        w(c, MGR + 4, WOBJ);
        w(c, WOBJ + 0x6c0, VT1);
        w(c, VT1 + 0x28, 0);
        w(c, VT1 + 0x2c, fakes::FAKE);

        // The manager's active list: empty / a hit / a miss.
        let head = match seed % 4 {
            1 => {
                w(c, ACTIVE, TRIG);
                ACTIVE
            }
            2 => {
                w(c, ACTIVE, 0x1234);
                ACTIVE
            }
            _ => 0,
        };
        w(c, HEAD, head);
        if seed % 4 >= 1 {
            w(c, ACTIVE + 4, 0);
        }

        c.g[A0][0] = 0;
        c.g[A1][0] = MGR as u64;
        c.g[A2][0] = MGR as u64;
        c.g[A3][0] = BOX as u64;
        c.g[T0][0] = NODE as u64;
        c.g[T1][0] = DESC as u64;
    }

    #[test]
    fn collect_octree_triggers_in_box_matches() {
        // The fixture must not sit in the fake callees' log area: every fake
        // call writes a 0x220-byte record there and would eat the node, its
        // list and the predicate slots mid-run.
        assert!(BASE > fakes::LOG + 0x1_0000, "fixture overlaps the fake call log");
        for seed in 0..200 {
            let report = compare(
                ee::s02::f_00174848__generated,
                camera_trigger_man_collect_octree_triggers_in_box,
                &|c| setup(c, seed),
            );
            assert!(report.hard.is_empty(), "seed {seed}: {:#?}", report.hard);
            assert!(report.soft.is_empty(), "seed {seed} (soft): {:#?}", report.soft);
            assert!(report.lohi.is_empty(), "seed {seed} (LO/HI): {:#?}", report.lohi);
        }
    }
}
