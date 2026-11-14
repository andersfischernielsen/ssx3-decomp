//! Staging module oc358 (wave wave216): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc358`.

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
    0x0033_9598,
    cell_gather_type2_recursive,
    "Cell_GatherType2Recursive",
)];

/// `Cell_GatherType2Subtree(node, ctx)` (0x00340b18): append a whole
/// subtree's type-2 objects to `ctx`.
const GT2_SUBTREE: Callee = Callee {
    addr: 0x0034_0b18,
    f: ee::s11::Cell_GatherType2Subtree,
};

/// `Cell_GatherType2Recursive` itself (through its generated alias, so the
/// recursion goes through the override table once installed).
const GT2_RECURSIVE: Callee = Callee {
    addr: 0x0033_9598,
    f: ee::s11::Cell_GatherType2Recursive,
};

/// Octree node: `child[8]` at +0, the type-2 object list at +0x28.
pub(crate) const GT2_NODE_OBJS: u32 = 0x28;

/// Where a flag a child test reads lives.
#[derive(Clone, Copy)]
enum Gt2Flag {
    /// A callee-saved register holding the flag (`s7` = x0 overlaps, `fp` =
    /// y1 overlaps, `s6` = x0 contained).
    Reg(usize),
    /// `v0`, loaded from the stack by the previous test's delay slot.
    V0,
    /// The x1-overlap word at `sp+0x10`, loaded first (one more cycle).
    StackX1,
}

/// Where a failed overlap test goes.
#[derive(Clone, Copy, PartialEq)]
enum Gt2Miss {
    /// One cycle, then the next coordinate load (`Gt2Child::next`).
    Skip,
    /// Straight to the next child (the delay slot already loaded `next`).
    End,
}

/// What the block leaves for the next child: `v0` = a sub-cell coordinate
/// word (`sp+off`), or, for the last child, the restored `s0`.
#[derive(Clone, Copy)]
enum Gt2Next {
    Coord(u32),
    RestoreS0,
}

/// The register move in the delay slot of the last "contained" test.
#[derive(Clone, Copy)]
enum Gt2LastDelay {
    /// `t0 = s0` on both paths.
    T0,
    /// `t0 = s0` only when the branch is taken (branch-likely).
    T0IfTaken,
    /// `a0 = s5` on both paths.
    A0,
}

/// One child octant's visit, as the compiler laid it out (the eight blocks
/// share their shape but not their register allocation or delay slots).
struct Gt2Child {
    /// Byte offset of the child pointer in the node.
    slot: u32,
    /// The sub-cell coordinate word to step (`sp+off += delta`) first; none
    /// for the first child.
    step: Option<(u32, i32)>,
    /// The three overlap tests: flag, stack word loaded in the delay slot,
    /// where a miss goes.
    overlap: [(Gt2Flag, u32, Gt2Miss); 3],
    /// The first two "contained" tests (flag, stack word loaded).
    contain: [(Gt2Flag, u32); 2],
    last_delay: Gt2LastDelay,
    /// Cycles before the object-list loop.
    list_cycles: u64,
    next: Gt2Next,
    /// The object-list loop head (event check); the subtree calls return
    /// to `head + 0x3c + 0x14 * i`, the recursion to `head + 0xec`.
    head: u32,
}

use Gt2Flag::{Reg as Gt2R, StackX1 as Gt2X1, V0 as Gt2V0};
use Gt2Miss::{End as Gt2End, Skip as Gt2Skip};

/// The eight children in visiting order (child 0, 1, 3, 2, 6, 7, 5, 4: the
/// sub-cell coordinates on the stack walk z, y, z, x, z, y, z).
/// Stack flags: +0x10 x1 overlap, +0x14 y0, +0x18 z0, +0x1c z1 overlap,
/// +0x20 x1 contained, +0x24 y0, +0x28 y1, +0x2c z0, +0x30 z1 contained.
const GT2_CHILDREN: [Gt2Child; 8] = [
    Gt2Child {
        slot: 0x0,
        step: None,
        overlap: [
            (Gt2R(S7), 0x14, Gt2Skip),
            (Gt2V0, 0x18, Gt2Skip),
            (Gt2V0, 0xc, Gt2End),
        ],
        contain: [(Gt2R(S6), 0x24), (Gt2V0, 0x2c)],
        last_delay: Gt2LastDelay::T0IfTaken,
        list_cycles: 2,
        next: Gt2Next::Coord(0xc),
        head: 0x0033_99d8,
    },
    Gt2Child {
        slot: 0x4,
        step: Some((0xc, 1)),
        overlap: [
            (Gt2R(S7), 0x14, Gt2Skip),
            (Gt2V0, 0x1c, Gt2Skip),
            (Gt2V0, 0x8, Gt2End),
        ],
        contain: [(Gt2R(S6), 0x24), (Gt2V0, 0x30)],
        last_delay: Gt2LastDelay::T0,
        list_cycles: 1,
        next: Gt2Next::Coord(0x8),
        head: 0x0033_9b18,
    },
    Gt2Child {
        slot: 0xc,
        step: Some((0x8, 1)),
        overlap: [
            (Gt2R(S7), 0xc, Gt2End),
            (Gt2R(FP), 0x1c, Gt2Skip),
            (Gt2V0, 0xc, Gt2End),
        ],
        contain: [(Gt2R(S6), 0x28), (Gt2V0, 0x30)],
        last_delay: Gt2LastDelay::T0,
        list_cycles: 1,
        next: Gt2Next::Coord(0xc),
        head: 0x0033_9c58,
    },
    Gt2Child {
        slot: 0x8,
        step: Some((0xc, -1)),
        overlap: [
            (Gt2R(S7), 0x4, Gt2End),
            (Gt2R(FP), 0x18, Gt2Skip),
            (Gt2V0, 0x4, Gt2End),
        ],
        contain: [(Gt2R(S6), 0x28), (Gt2V0, 0x2c)],
        last_delay: Gt2LastDelay::T0,
        list_cycles: 1,
        next: Gt2Next::Coord(0x4),
        head: 0x0033_9d98,
    },
    Gt2Child {
        slot: 0x18,
        step: Some((0x4, 1)),
        overlap: [
            (Gt2X1, 0xc, Gt2End),
            (Gt2R(FP), 0x18, Gt2Skip),
            (Gt2V0, 0x20, Gt2Skip),
        ],
        contain: [(Gt2V0, 0x28), (Gt2V0, 0x2c)],
        last_delay: Gt2LastDelay::T0,
        list_cycles: 2,
        next: Gt2Next::Coord(0xc),
        head: 0x0033_9ee0,
    },
    Gt2Child {
        slot: 0x1c,
        step: Some((0xc, 1)),
        overlap: [
            (Gt2X1, 0x8, Gt2End),
            (Gt2R(FP), 0x1c, Gt2Skip),
            (Gt2V0, 0x20, Gt2Skip),
        ],
        contain: [(Gt2V0, 0x28), (Gt2V0, 0x30)],
        last_delay: Gt2LastDelay::T0,
        list_cycles: 2,
        next: Gt2Next::Coord(0x8),
        head: 0x0033_a028,
    },
    Gt2Child {
        slot: 0x14,
        step: Some((0x8, -1)),
        overlap: [
            (Gt2X1, 0x14, Gt2Skip),
            (Gt2V0, 0x1c, Gt2Skip),
            (Gt2V0, 0x20, Gt2Skip),
        ],
        contain: [(Gt2V0, 0x24), (Gt2V0, 0x30)],
        last_delay: Gt2LastDelay::T0,
        list_cycles: 2,
        next: Gt2Next::Coord(0xc),
        head: 0x0033_a170,
    },
    Gt2Child {
        slot: 0x10,
        step: Some((0xc, -1)),
        overlap: [
            (Gt2X1, 0x14, Gt2Skip),
            (Gt2V0, 0x18, Gt2Skip),
            (Gt2V0, 0x20, Gt2Skip),
        ],
        contain: [(Gt2V0, 0x24), (Gt2V0, 0x2c)],
        last_delay: Gt2LastDelay::A0,
        list_cycles: 2,
        next: Gt2Next::RestoreS0,
        head: 0x0033_a2b8,
    },
];

/// `v0 = sp[off]` (a word load, sign-extended).
fn gt2_stack_word(c: &mut Cpu, off: u32) {
    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(off));
}

/// The load a child block ends with (see [`Gt2Next`]).
fn gt2_next(c: &mut Cpu, next: Gt2Next) {
    match next {
        Gt2Next::Coord(off) => gt2_stack_word(c, off),
        Gt2Next::RestoreS0 => c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0)),
    }
}

/// A flag test's cycles and value.
fn gt2_flag(c: &mut Cpu, flag: Gt2Flag) -> u64 {
    match flag {
        Gt2Flag::Reg(r) => {
            c.cycles += 2;
            c.g[r][0]
        }
        Gt2Flag::V0 => {
            c.cycles += 2;
            c.g[V0][0]
        }
        Gt2Flag::StackX1 => {
            c.cycles += 3;
            gt2_stack_word(c, 0x10);
            c.g[V0][0]
        }
    }
}

/// Append the list of objects starting at `a2` to the gather context in
/// `s1` (`list[(*count)++] = obj`), loop head `head`.
fn gt2_append_list(c: &mut Cpu, head: u32) {
    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
    loop {
        c.cycles += 10;
        c.g[A0][0] = c.lw(c.g[S1][0] as u32);
        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
        c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
        c.sw(c.g[V0][0] as u32, c.g[A2][0]);
        c.sw(c.g[A1][0] as u32, c.g[V1][0]);
        c.g[A2][0] = c.lw(c.g[A2][0] as u32);
        if c.g[A2][0] == 0 {
            return;
        }
        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// A child wholly inside the box (node in `s0`): append its objects and
/// its children's whole subtrees to the collect context (`s1`).
fn gt2_collect(c: &mut Cpu, ch: &Gt2Child) -> Flow {
    c.cycles += 3;
    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(GT2_NODE_OBJS));
    if c.g[A2][0] != 0 {
        c.cycles += ch.list_cycles;
        gt2_append_list(c, ch.head);
        c.cycles += 1;
    }
    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
    let first_ret = ch.head + 0x3c;
    for i in 0..7u32 {
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = c.g[S1][0];
            GT2_SUBTREE.call(c, first_ret + 0x14 * i)?;
        }
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(4 * (i + 1)));
    }
    c.cycles += 2;
    let last = c.g[A0][0];
    gt2_next(c, ch.next);
    if last != 0 {
        c.cycles += 4;
        c.g[A1][0] = c.g[S1][0];
        GT2_SUBTREE.call(c, first_ret + 0x8c)?;
        gt2_next(c, ch.next);
    }
    Ok(())
}

/// Visit one child octant: skip it when it is absent or misses the box,
/// collect it whole when the box contains it, else recurse into it with the
/// sub-cell on the stack.
fn gt2_visit_child(c: &mut Cpu, ch: &Gt2Child) -> Flow {
    let skip = 'visit: {
        let sp = c.g[SP][0] as u32;
        match ch.step {
            None => {
                c.cycles += 3;
                c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(ch.slot));
                let absent = c.g[S0][0] == 0;
                gt2_next(c, ch.next);
                if absent {
                    break 'visit false;
                }
            }
            Some((off, delta)) => {
                c.cycles += 4;
                c.g[S0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(ch.slot));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(delta as u32));
                let absent = c.g[S0][0] == 0;
                c.sw(sp.wrapping_add(off), c.g[V0][0]);
                if absent {
                    break 'visit true;
                }
            }
        }
        for (flag, load, miss) in ch.overlap {
            let v = gt2_flag(c, flag);
            gt2_stack_word(c, load);
            if v == 0 {
                break 'visit miss == Gt2Miss::Skip;
            }
        }
        let recurse_after_cycle = 'contained: {
            for (flag, load) in ch.contain {
                let v = gt2_flag(c, flag);
                gt2_stack_word(c, load);
                if v == 0 {
                    break 'contained true;
                }
            }
            c.cycles += 2;
            let partial = c.g[V0][0] == 0;
            match ch.last_delay {
                Gt2LastDelay::T0 => c.g[T0][0] = c.g[S0][0],
                Gt2LastDelay::T0IfTaken if partial => c.g[T0][0] = c.g[S0][0],
                Gt2LastDelay::T0IfTaken => {}
                Gt2LastDelay::A0 => c.g[A0][0] = c.g[S5][0],
            }
            if partial {
                break 'contained false;
            }
            gt2_collect(c, ch)?;
            break 'visit false;
        };
        if recurse_after_cycle {
            c.cycles += 1;
        }
        // Recurse: (flag, ctx, ctx2, box, child, &sub).
        c.cycles += 6;
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[T0][0] = c.g[S0][0];
        c.g[T1][0] = c.g[SP][0];
        GT2_RECURSIVE.call(c, ch.head + 0xec)?;
        true
    };
    if skip {
        c.cycles += 1;
        gt2_next(c, ch.next);
    }
    Ok(())
}

/// The 0.2-cell margins (`.lit4`, one load per test).
const GT2_MARGIN_X0: u32 = 0x2d40;
const GT2_MARGIN_X1: u32 = 0x2d3c;
const GT2_MARGIN_Y0: u32 = 0x2d38;
const GT2_MARGIN_Y1: u32 = 0x2d34;
const GT2_MARGIN_Z0: u32 = 0x2d30;
const GT2_MARGIN_Z1: u32 = 0x2d2c;

/// `f[margin] = *(gp - off)`.
fn gt2_margin(c: &mut Cpu, f: usize, off: u32) {
    c.lwc1(f, (c.g[GP][0] as u32).wrapping_sub(off));
}

/// Second half of a range test after `min < hi` held: `lo < box.max[axis]`
/// (max at `box + max_off`) sets the stack flag `sp+flag_off` (v0 = 1).
fn gt2_upper_word(c: &mut Cpu, lo: usize, max_off: u32, flag_off: u32) {
    c.cycles += 5;
    c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(max_off));
    c.c_lt_s(lo, 0);
    c.g[V0][0] = 0x1;
    if c.fcc() {
        c.cycles += 1;
        c.sw((c.g[SP][0] as u32).wrapping_add(flag_off), c.g[V0][0]);
    }
}

/// [`gt2_upper_word`] for a flag kept in a register.
fn gt2_upper_reg(c: &mut Cpu, lo: usize, max_off: u32, reg: usize) {
    c.cycles += 5;
    c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(max_off));
    c.c_lt_s(lo, 0);
    if c.fcc() {
        c.g[reg][0] = 0x1;
    }
}

/// Sub-cell setup and the x tests: stores the sub-cell `{level - 1, 2x,
/// 2y, 2z}` at `sp+0`, the scale `2^(level-1)` in f4, x0 overlap in s7
/// (range f15..f16) and x1 overlap at `sp+0x10` (range f12..f14), box
/// min x in f13. Entered with a1 = level, v0 = level + 0x7e.
fn gt2_classify_x(c: &mut Cpu) {
    let sp = c.g[SP][0] as u32;
    c.cycles += 28;
    c.g[A0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 23);
    gt2_margin(c, 2, GT2_MARGIN_X0);
    c.mtc1(V0, 4);
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 1);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
    c.mtc1(A0, 0);
    c.cvt_s_w(0, 0);
    c.mtc1(V0, 1);
    c.cvt_s_w(1, 1);
    c.lwc1(3, c.g[S2][0] as u32);
    c.g[V1][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0xc));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
    c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x8));
    c.sub_s(0, 0, 2);
    c.add_s(1, 1, 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
    c.sw(sp, c.g[A1][0]);
    c.g[S7][0] = 0;
    c.mul_s(15, 0, 4);
    c.mul_s(16, 1, 4);
    c.sw(sp + 0x8, c.g[V0][0]);
    c.sw(sp + 0xc, c.g[V1][0]);
    c.c_lt_s(3, 16);
    let hit = c.fcc();
    c.sw(sp + 0x4, c.g[A0][0]);
    if hit {
        gt2_upper_reg(c, 15, 0x10, S7);
    }

    c.cycles += 18;
    gt2_stack_word(c, 0x4);
    gt2_margin(c, 3, GT2_MARGIN_X1);
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
    c.sw(sp + 0x10, 0);
    c.mtc1(V1, 1);
    c.cvt_s_w(1, 1);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
    c.mtc1(V0, 0);
    c.cvt_s_w(0, 0);
    c.lwc1(2, c.g[S2][0] as u32);
    c.add_s(1, 1, 3);
    c.mov_s(13, 2);
    c.sub_s(0, 0, 3);
    c.mul_s(14, 1, 4);
    c.c_lt_s(2, 14);
    let hit = c.fcc();
    c.mul_s(12, 0, 4);
    if hit {
        gt2_upper_word(c, 12, 0x10, 0x10);
    }
}

/// The y tests (entered with v1 = 2y): y0 overlap at `sp+0x14` (range
/// f10..f11), y1 overlap in fp (f8..f9), box min y in f6.
fn gt2_classify_y(c: &mut Cpu) {
    let sp = c.g[SP][0] as u32;
    c.cycles += 17;
    gt2_margin(c, 3, GT2_MARGIN_Y0);
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
    c.sw(sp + 0x14, 0);
    c.mtc1(V0, 1);
    c.cvt_s_w(1, 1);
    c.g[A0][0] = c.g[V1][0];
    c.mtc1(V1, 0);
    c.cvt_s_w(0, 0);
    c.lwc1(2, (c.g[S2][0] as u32).wrapping_add(0x4));
    c.add_s(1, 1, 3);
    c.mov_s(6, 2);
    c.sub_s(0, 0, 3);
    c.mul_s(11, 1, 4);
    c.c_lt_s(2, 11);
    let hit = c.fcc();
    c.mul_s(10, 0, 4);
    if hit {
        gt2_upper_word(c, 10, 0x14, 0x14);
    }

    c.cycles += 15;
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2));
    gt2_margin(c, 2, GT2_MARGIN_Y1);
    c.mtc1(V1, 1);
    c.cvt_s_w(1, 1);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
    c.mtc1(V0, 0);
    c.cvt_s_w(0, 0);
    c.g[FP][0] = 0;
    c.add_s(1, 1, 2);
    c.sub_s(0, 0, 2);
    c.mul_s(9, 1, 4);
    c.c_lt_s(6, 9);
    let hit = c.fcc();
    c.mul_s(8, 0, 4);
    if hit {
        gt2_upper_reg(c, 8, 0x14, FP);
    }
}

/// The z tests (entered with v1 = 2z): z0 overlap at `sp+0x18` (range
/// f3..f7), z1 overlap at `sp+0x1c` (f2..f1), box min z in f5.
fn gt2_classify_z(c: &mut Cpu) {
    let sp = c.g[SP][0] as u32;
    c.cycles += 17;
    gt2_margin(c, 3, GT2_MARGIN_Z0);
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
    c.sw(sp + 0x18, 0);
    c.mtc1(V0, 1);
    c.cvt_s_w(1, 1);
    c.g[A0][0] = c.g[V1][0];
    c.mtc1(V1, 0);
    c.cvt_s_w(0, 0);
    c.lwc1(2, (c.g[S2][0] as u32).wrapping_add(0x8));
    c.add_s(1, 1, 3);
    c.mov_s(5, 2);
    c.sub_s(0, 0, 3);
    c.mul_s(7, 1, 4);
    c.c_lt_s(2, 7);
    let hit = c.fcc();
    c.mul_s(3, 0, 4);
    if hit {
        gt2_upper_word(c, 3, 0x18, 0x18);
    }

    c.cycles += 15;
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2));
    gt2_margin(c, 2, GT2_MARGIN_Z1);
    c.mtc1(V1, 1);
    c.cvt_s_w(1, 1);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
    c.mtc1(V0, 0);
    c.cvt_s_w(0, 0);
    c.sw(sp + 0x1c, 0);
    c.add_s(1, 1, 2);
    c.sub_s(0, 0, 2);
    c.mul_s(1, 1, 4);
    c.c_lt_s(5, 1);
    let hit = c.fcc();
    c.mul_s(2, 0, 4);
    if hit {
        gt2_upper_word(c, 2, 0x18, 0x1c);
    }
}

/// The "wholly inside" tests (`min < lo && hi < max`, no margin change):
/// x0 in s6, the rest at `sp+0x20..0x30` (x1, y0, y1, z0, z1).
fn gt2_classify_contained(c: &mut Cpu) {
    let sp = c.g[SP][0] as u32;
    c.cycles += 4;
    c.c_lt_s(13, 15);
    c.g[S6][0] = 0;
    if c.fcc() {
        gt2_upper_reg(c, 16, 0x10, S6);
    }
    // (box min, lo, hi, box max offset, flag offset)
    for (min, lo, hi, max_off, flag_off) in [
        (13, 12, 14, 0x10, 0x20),
        (6, 10, 11, 0x14, 0x24),
        (6, 8, 9, 0x14, 0x28),
        (5, 3, 7, 0x18, 0x2c),
        (5, 2, 1, 0x18, 0x30),
    ] {
        c.cycles += 4;
        c.c_lt_s(min, lo);
        c.sw(sp + flag_off, 0);
        if c.fcc() {
            gt2_upper_word(c, hi, max_off, flag_off);
        }
    }
}

/// `void Cell_GatherType2Recursive(char* flag, GatherCtx* ctx, GatherCtx*
/// collect, const float* box, OctNode* node, const Cell* cell)` (0x00339598):
/// append the node's type-2 objects (list at +0x28) to `ctx`, then, unless
/// the cell is at level 11, split it into its eight sub-cells (level - 1,
/// coordinates doubled, edge `2^(level-1)`, widened by 0.2 cell on each
/// side) and test each against the box (`min` at +0, `max` at +0x10): a
/// child wholly inside is gathered whole into `collect`
/// (`Cell_GatherType2Subtree` on its children, its own list inlined), a
/// child overlapping it is recursed into, others are skipped. A gather
/// context is `{ void** list; int* count; }`.
pub fn cell_gather_type2_recursive(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xe0));
        let sp = c.g[SP][0] as u32;
        c.sq(17, sp + 0xc0);
        c.sq(18, sp + 0xb0);
        c.g[S1][0] = c.g[A2][0];
        c.sq(19, sp + 0xa0);
        c.g[S2][0] = c.g[A3][0];
        c.sq(20, sp + 0x90);
        c.g[S3][0] = c.g[A1][0];
        c.sq(21, sp + 0x80);
        c.g[S4][0] = c.g[T0][0];
        c.sq(16, sp + 0xd0);
        c.sq(22, sp + 0x70);
        c.sq(23, sp + 0x60);
        c.sq(30, sp + 0x50);
        c.sd(sp + 0x40, c.g[RA][0]);
        // The node's own objects go to `ctx` (s3).
        c.g[T0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(GT2_NODE_OBJS));
        c.g[S5][0] = c.g[A0][0];
        if c.g[T0][0] != 0 {
            c.cycles += 2;
            c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
            loop {
                c.cycles += 10;
                c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                c.sw(c.g[V0][0] as u32, c.g[T0][0]);
                c.sw(c.g[A1][0] as u32, c.g[V1][0]);
                c.g[T0][0] = c.lw(c.g[T0][0] as u32);
                if c.g[T0][0] == 0 {
                    break;
                }
                c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                if event_due(c) {
                    deliver_event_here(c, 0x0033_95e8);
                }
            }
        }
        'epilogue: {
            'leaf: {
                // Level 11 is the deepest: no children to visit.
                c.cycles += 4;
                c.g[A1][0] = c.lw(c.g[T1][0] as u32);
                c.g[V0][0] = 0xb;
                let leaf = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7e));
                if leaf {
                    break 'leaf;
                }
                gt2_classify_x(c);
                // Any axis on which neither half overlaps the box: done
                // (each test restores s0 in its delay slot).
                c.cycles += 2;
                let ox0 = c.g[S7][0] != 0;
                c.g[V1][0] = c.lw(sp + 0x8);
                if !ox0 {
                    c.cycles += 3;
                    gt2_stack_word(c, 0x10);
                    let ox1 = c.g[V0][0] != 0;
                    c.lq(16, sp + 0xd0);
                    if !ox1 {
                        break 'epilogue;
                    }
                }
                gt2_classify_y(c);
                c.cycles += 3;
                gt2_stack_word(c, 0x14);
                let oy0 = c.g[V0][0] != 0;
                c.g[V1][0] = c.lw(sp + 0xc);
                if !oy0 {
                    c.cycles += 2;
                    let oy1 = c.g[FP][0] != 0;
                    c.lq(16, sp + 0xd0);
                    if !oy1 {
                        break 'epilogue;
                    }
                }
                gt2_classify_z(c);
                c.cycles += 3;
                gt2_stack_word(c, 0x18);
                let oz0 = c.g[V0][0] != 0;
                gt2_stack_word(c, 0x1c);
                if !oz0 {
                    c.cycles += 2;
                    let oz1 = c.g[V0][0] != 0;
                    c.lq(16, sp + 0xd0);
                    if !oz1 {
                        break 'epilogue;
                    }
                }
                gt2_classify_contained(c);
                // (The last child restores s0 itself, skipped or not.)
                for ch in &GT2_CHILDREN {
                    gt2_visit_child(c, ch)?;
                }
                break 'epilogue;
            }
            c.cycles += 1;
            c.lq(16, sp + 0xd0);
        }
        c.cycles += 11;
        c.lq(17, sp + 0xc0);
        c.lq(18, sp + 0xb0);
        c.lq(19, sp + 0xa0);
        c.lq(20, sp + 0x90);
        c.lq(21, sp + 0x80);
        c.lq(22, sp + 0x70);
        c.lq(23, sp + 0x60);
        c.lq(30, sp + 0x50);
        c.g[RA][0] = c.ld(sp + 0x40);
        c.g[SP][0] = sx(sp.wrapping_add(0xe0));
        ret(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

    /// Fixture octree: a bump allocator for nodes and object lists.
    struct Gt2Arena {
        nodes: u32,
        objs: u32,
    }

    /// A list of 0..3 objects (`next` at +0), or 0.
    fn gt2_obj_list(c: &mut Cpu, r: &mut Rng, a: &mut Gt2Arena) -> u32 {
        let mut head = 0;
        for _ in 0..r.next_u32() % 4 {
            let obj = a.objs;
            a.objs += 0x10;
            c.bus.write32(obj, head);
            c.bus.write32(obj + 4, r.next_u32());
            head = obj;
        }
        head
    }

    /// A node with random children down to `depth` more levels.
    fn gt2_node(c: &mut Cpu, r: &mut Rng, a: &mut Gt2Arena, depth: u32) -> u32 {
        let node = a.nodes;
        a.nodes += 0x40;
        for i in 0..8 {
            let child = if depth > 0 && r.next_u32() % 3 != 0 {
                gt2_node(c, r, a, depth - 1)
            } else {
                0
            };
            c.bus.write32(node + 4 * i, child);
        }
        c.bus.write32(node + 0x20, r.next_u32());
        c.bus.write32(node + 0x24, r.next_u32());
        let list = gt2_obj_list(c, r, a);
        c.bus.write32(node + GT2_NODE_OBJS, list);
        node
    }

    /// Gathers type-2 objects from a random two-level octree against a box
    /// chosen per axis (whole, either half, a margin edge, the middle, or
    /// disjoint), so every child is skipped, collected or recursed into on
    /// some seed; level 11 stops at the node's own list.
    #[test]
    fn cell_gather_type2_recursive_matches() {
        check(
            ee::s11::f_00339598__generated,
            cell_gather_type2_recursive,
            200,
            &|c, seed| {
                let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ (seed as u64) << 8 ^ 0x9598);
                scramble(c, &mut r);
                c.pc = 0x0033_9598;
                c.g[GP][0] = GAME_GP as u64;
                for off in [
                    GT2_MARGIN_X0,
                    GT2_MARGIN_X1,
                    GT2_MARGIN_Y0,
                    GT2_MARGIN_Y1,
                    GT2_MARGIN_Z0,
                    GT2_MARGIN_Z1,
                ] {
                    c.bus.write32(GAME_GP - off, 0.2f32.to_bits());
                }
                // Gather contexts `{ list, count* }`; the caller often passes the
                // same one twice.
                let (ctx, ctx2) = (DATA + 0x100, DATA + 0x200);
                c.bus.write32(ctx, DATA + 0x4000);
                c.bus.write32(ctx + 4, DATA + 0x110);
                c.bus.write32(DATA + 0x110, r.next_u32() % 4);
                c.bus.write32(ctx2, DATA + 0x6000);
                c.bus.write32(ctx2 + 4, DATA + 0x210);
                c.bus.write32(DATA + 0x210, r.next_u32() % 4);
                let mut arena = Gt2Arena {
                    nodes: DATA + 0x1_0000,
                    objs: DATA + 0x2_0000,
                };
                let root = gt2_node(c, &mut r, &mut arena, 2);
                // The cell and the box.
                let (cell, bx) = (DATA + 0x300, DATA + 0x400);
                let level = seed % 12;
                let coords = [r.next_u32() % 4, r.next_u32() % 4, r.next_u32() % 4];
                c.bus.write32(cell, level);
                for k in 0..3u32 {
                    c.bus.write32(cell + 4 + 4 * k, coords[k as usize]);
                }
                let s = f32::from_bits((level + 0x7e) << 23);
                for k in 0..3u32 {
                    let at = |u: f32| (2.0 * coords[k as usize] as f32 + u) * s;
                    let (min, max) = match r.next_u32() % 9 {
                        0 => (-1.0e6, 1.0e6),
                        1 => (at(0.5), 1.0e6),
                        2 => (-1.0e6, at(0.5)),
                        3 => (at(1.5), 1.0e6),
                        4 => (1.0e6, 1.0e6 + 1.0),
                        5 => (-1.0e6, at(1.0)),
                        6 => (at(1.0), 1.0e6),
                        7 => (-1.0e6, at(-0.5)),
                        _ => (at(0.3), at(1.7)),
                    };
                    c.bus.write32(bx + 4 * k, f32::to_bits(min));
                    c.bus.write32(bx + 0x10 + 4 * k, f32::to_bits(max));
                }
                c.g[A0][0] = sx(DATA + 0x800);
                c.g[A1][0] = sx(ctx);
                c.g[A2][0] = sx(if seed % 2 == 0 { ctx } else { ctx2 });
                c.g[A3][0] = sx(bx);
                c.g[T0][0] = sx(root);
                c.g[T1][0] = sx(cell);
            },
        );
    }
}
