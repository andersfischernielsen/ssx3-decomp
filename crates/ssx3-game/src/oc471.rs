//! Staging module oc471 (wave wave226): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc471`.

#![allow(unused_imports)]

use crate::abi;
use crate::call::{guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0033_dfd8,
    cell_collide_objects_recursive,
    "Cell_CollideObjectsRecursive",
)];

/// `Cell_CollideObjectEx` (0x336d40): report one static object to `ctx`.
const COLLIDE_OBJECT: Callee = Callee {
    addr: 0x0033_6d40,
    f: ee::s11::Cell_CollideObjectEx,
};

/// `Cell_CollideObjectsSubtree` (0x3410c0): report a whole subtree to `ctx2`.
const COLLIDE_SUBTREE: Callee = Callee {
    addr: 0x0034_10c0,
    f: ee::s11::Cell_CollideObjectsSubtree,
};

/// This function (0x33dfd8), called for the children that only partly overlap.
const COLLIDE_RECURSIVE: Callee = Callee {
    addr: 0x0033_dfd8,
    f: ee::s11::Cell_CollideObjectsRecursive,
};

// Stack frame of the cell walk: the child's key `{ level, x, y, z }` at sp+0,
// then one flag word per overlap test (1 when the sub-cell's expanded range
// overlaps the box; `cx`/`cy`/`cz` when the sub-cell is fully inside it).
const SUB_LEVEL: u32 = 0x00;
const SUB_X: u32 = 0x04;
const SUB_Y: u32 = 0x08;
const SUB_Z: u32 = 0x0c;
const F_OX1: u32 = 0x10;
const F_OY0: u32 = 0x14;
const F_OY1: u32 = 0x18;
const F_OZ0: u32 = 0x1c;
const F_OZ1: u32 = 0x20;
const F_CX1: u32 = 0x24;
const F_CY0: u32 = 0x28;
const F_CY1: u32 = 0x2c;
const F_CZ0: u32 = 0x30;
const F_CZ1: u32 = 0x34;

/// Node layout: eight child pointers at +0x00..+0x1c, the object list at +0x20.
const NODE_OBJECTS: u32 = 0x20;
/// Box layout: min x/y/z at +0/+4/+8, max x/y/z at +0x10/+0x14/+0x18.
const BOX_MAX: u32 = 0x10;

/// Where a child's test reads its flag: the s8 register (x0 overlap), the s7
/// register (x0 fully inside), or a frame word.
#[derive(Clone, Copy)]
enum Flag {
    Fp,
    S7,
    Slot(u32),
}

/// One child of the node, in the order the original visits them.
struct Visit {
    /// Offset of the child pointer in the node.
    child: u32,
    /// Frame word of the sub-cell coordinate this child steps (`sub.x++` ...).
    coord: u32,
    /// Step applied to that coordinate (0 for the first child).
    step: i32,
    /// Cycles charged on entry (3 for the first child, 4 for the rest).
    enter: u64,
    /// Cycles of the first overlap test.
    first: u64,
    /// Flags: ox, oy, oz, fully-inside x, y, z.
    ox: Flag,
    oy: u32,
    oz: u32,
    cx: Flag,
    cy: u32,
    cz: u32,
    /// Frame word the oz test leaves in v0 (the next coordinate, or cx1).
    after_oz: u32,
    /// Leave without the tail when the child pointer is null.
    s1_end: bool,
    /// Leave without the tail when the oz test fails.
    oz_end: bool,
    /// Frame word the tail loads into v0 (None: the last child).
    next: Option<u32>,
    /// Cycles charged before the object loop (1 or 2).
    pre_obj: u64,
    /// Return address of the object loop's call (the loop head is 8 below).
    obj_ret: u32,
    /// Return addresses of the eight subtree calls.
    sub_ret: [u32; 8],
    /// Return address of the recursive call.
    rec_ret: u32,
}

const VISITS: [Visit; 8] = [
    // child 0: [0][0][0]; sub.z stays
    Visit {
        child: 0x00,
        coord: SUB_Z,
        step: 0,
        enter: 3,
        first: 2,
        ox: Flag::Fp,
        oy: F_OY0,
        oz: F_OZ0,
        cx: Flag::S7,
        cy: F_CY0,
        cz: F_CZ0,
        after_oz: SUB_Z,
        s1_end: true,
        oz_end: true,
        next: Some(SUB_Z),
        pre_obj: 1,
        obj_ret: 0x0033_e410,
        sub_ret: [
            0x0033_e430,
            0x0033_e444,
            0x0033_e458,
            0x0033_e46c,
            0x0033_e480,
            0x0033_e494,
            0x0033_e4a8,
            0x0033_e4bc,
        ],
        rec_ret: 0x0033_e4e0,
    },
    // child 1: [0][0][1]; sub.z++
    Visit {
        child: 0x04,
        coord: SUB_Z,
        step: 1,
        enter: 4,
        first: 2,
        ox: Flag::Fp,
        oy: F_OY0,
        oz: F_OZ1,
        cx: Flag::S7,
        cy: F_CY0,
        cz: F_CZ1,
        after_oz: SUB_Y,
        s1_end: false,
        oz_end: true,
        next: Some(SUB_Y),
        pre_obj: 2,
        obj_ret: 0x0033_e540,
        sub_ret: [
            0x0033_e560,
            0x0033_e574,
            0x0033_e588,
            0x0033_e59c,
            0x0033_e5b0,
            0x0033_e5c4,
            0x0033_e5d8,
            0x0033_e5ec,
        ],
        rec_ret: 0x0033_e610,
    },
    // child 3: [0][1][1]; sub.y++
    Visit {
        child: 0x0c,
        coord: SUB_Y,
        step: 1,
        enter: 4,
        first: 2,
        ox: Flag::Fp,
        oy: F_OY1,
        oz: F_OZ1,
        cx: Flag::S7,
        cy: F_CY1,
        cz: F_CZ1,
        after_oz: SUB_Z,
        s1_end: false,
        oz_end: true,
        next: Some(SUB_Z),
        pre_obj: 2,
        obj_ret: 0x0033_e670,
        sub_ret: [
            0x0033_e690,
            0x0033_e6a4,
            0x0033_e6b8,
            0x0033_e6cc,
            0x0033_e6e0,
            0x0033_e6f4,
            0x0033_e708,
            0x0033_e71c,
        ],
        rec_ret: 0x0033_e740,
    },
    // child 2: [0][1][0]; sub.z--
    Visit {
        child: 0x08,
        coord: SUB_Z,
        step: -1,
        enter: 4,
        first: 2,
        ox: Flag::Fp,
        oy: F_OY1,
        oz: F_OZ0,
        cx: Flag::S7,
        cy: F_CY1,
        cz: F_CZ0,
        after_oz: SUB_X,
        s1_end: false,
        oz_end: true,
        next: Some(SUB_X),
        pre_obj: 2,
        obj_ret: 0x0033_e7a0,
        sub_ret: [
            0x0033_e7c0,
            0x0033_e7d4,
            0x0033_e7e8,
            0x0033_e7fc,
            0x0033_e810,
            0x0033_e824,
            0x0033_e838,
            0x0033_e84c,
        ],
        rec_ret: 0x0033_e870,
    },
    // child 6: [1][1][0]; sub.x++
    Visit {
        child: 0x18,
        coord: SUB_X,
        step: 1,
        enter: 4,
        first: 3,
        ox: Flag::Slot(F_OX1),
        oy: F_OY1,
        oz: F_OZ0,
        cx: Flag::Slot(F_CX1),
        cy: F_CY1,
        cz: F_CZ0,
        after_oz: F_CX1,
        s1_end: false,
        oz_end: false,
        next: Some(SUB_Z),
        pre_obj: 1,
        obj_ret: 0x0033_e8d0,
        sub_ret: [
            0x0033_e8f0,
            0x0033_e904,
            0x0033_e918,
            0x0033_e92c,
            0x0033_e940,
            0x0033_e954,
            0x0033_e968,
            0x0033_e97c,
        ],
        rec_ret: 0x0033_e9a0,
    },
    // child 7: [1][1][1]; sub.z++
    Visit {
        child: 0x1c,
        coord: SUB_Z,
        step: 1,
        enter: 4,
        first: 3,
        ox: Flag::Slot(F_OX1),
        oy: F_OY1,
        oz: F_OZ1,
        cx: Flag::Slot(F_CX1),
        cy: F_CY1,
        cz: F_CZ1,
        after_oz: F_CX1,
        s1_end: false,
        oz_end: false,
        next: Some(SUB_Y),
        pre_obj: 1,
        obj_ret: 0x0033_ea00,
        sub_ret: [
            0x0033_ea20,
            0x0033_ea34,
            0x0033_ea48,
            0x0033_ea5c,
            0x0033_ea70,
            0x0033_ea84,
            0x0033_ea98,
            0x0033_eaac,
        ],
        rec_ret: 0x0033_ead0,
    },
    // child 5: [1][0][1]; sub.y--
    Visit {
        child: 0x14,
        coord: SUB_Y,
        step: -1,
        enter: 4,
        first: 3,
        ox: Flag::Slot(F_OX1),
        oy: F_OY0,
        oz: F_OZ1,
        cx: Flag::Slot(F_CX1),
        cy: F_CY0,
        cz: F_CZ1,
        after_oz: F_CX1,
        s1_end: false,
        oz_end: false,
        next: Some(SUB_Z),
        pre_obj: 1,
        obj_ret: 0x0033_eb30,
        sub_ret: [
            0x0033_eb50,
            0x0033_eb64,
            0x0033_eb78,
            0x0033_eb8c,
            0x0033_eba0,
            0x0033_ebb4,
            0x0033_ebc8,
            0x0033_ebdc,
        ],
        rec_ret: 0x0033_ec00,
    },
    // child 4: [1][0][0]; sub.z--; the last child, so no tail load
    Visit {
        child: 0x10,
        coord: SUB_Z,
        step: -1,
        enter: 4,
        first: 3,
        ox: Flag::Slot(F_OX1),
        oy: F_OY0,
        oz: F_OZ0,
        cx: Flag::Slot(F_CX1),
        cy: F_CY0,
        cz: F_CZ0,
        after_oz: F_CX1,
        s1_end: false,
        oz_end: false,
        next: None,
        pre_obj: 1,
        obj_ret: 0x0033_ec60,
        sub_ret: [
            0x0033_ec80,
            0x0033_ec94,
            0x0033_eca8,
            0x0033_ecbc,
            0x0033_ecd0,
            0x0033_ece4,
            0x0033_ecf8,
            0x0033_ed0c,
        ],
        rec_ret: 0x0033_ed30,
    },
];

// ---------------------------------------------------------------------------
// Small helpers: frame words, the flags, and the FPU (COP1) registers.

fn slot(c: &mut Cpu, off: u32) -> u64 {
    Addr(sp(c).wrapping_add(off)).lw(c)
}

fn set_slot(c: &mut Cpu, off: u32, v: u64) {
    Addr(sp(c).wrapping_add(off)).sw(c, v)
}

fn load_v0(c: &mut Cpu, off: Option<u32>) {
    if let Some(off) = off {
        let v = slot(c, off);
        c.g[V0][0] = v;
    }
}

fn flag_is_clear(c: &mut Cpu, f: Flag) -> bool {
    match f {
        Flag::Fp => c.g[FP][0] == 0,
        Flag::S7 => c.g[S7][0] == 0,
        Flag::Slot(off) => slot(c, off) == 0,
    }
}

fn fr(c: &Cpu, n: usize) -> FpuF32 {
    FpuF32(c.f[n])
}

fn fw(c: &mut Cpu, n: usize, v: FpuF32) {
    c.f[n] = v.0;
}

/// `cvt.s.w` in place.
fn cvt_w(c: &mut Cpu, n: usize) {
    c.f[n] = FpuF32::from_i32(c.f[n] as i32).0;
}

/// `lwc1 f, addr`.
fn load_f(c: &mut Cpu, n: usize, addr: u32) {
    c.f[n] = Addr(addr).read_u32(c);
}

/// `lwc1 f, GP - off` (the float constants the game keeps off gp).
fn load_gp_f(c: &mut Cpu, n: usize, off: u32) {
    let addr = (c.g[GP][0] as u32).wrapping_sub(off);
    load_f(c, n, addr);
}

/// `c.lt.s`: sets the FPU condition bit and returns it.
fn lt_fcc(c: &mut Cpu, a: usize, b: usize) -> bool {
    let r = fr(c, a).lt(fr(c, b));
    abi::set_fcc(c, r);
    r
}

// ---------------------------------------------------------------------------
// The overlap tests of the cell's eight children, in the original's order.
// Each block returns the flags it stores in the frame; `false` from
// `overlap_flags` is the original's early `return` (the cell is not in the box).

/// x0 slab (ox0 in s8): the sub-cell's expanded x range against the box.
fn x0_slab(c: &mut Cpu) {
    let s1 = c.g[S1][0] as u32;
    let s3 = c.g[S3][0] as u32;
    c.cycles += 28;
    c.g[A0][0] = Addr(s1.wrapping_add(4)).lw(c); // cell->x
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 23); // cell size: 2^(sub.level)
    load_gp_f(c, 2, 0x2cdc); // 0.2
    c.f[4] = c.g[V0][0] as u32;
    let sub_x = (c.g[A0][0] as u32) << 1;
    c.g[A0][0] = sx(sub_x);
    c.g[V0][0] = sx(sub_x.wrapping_add(1));
    c.f[0] = sub_x;
    cvt_w(c, 0);
    c.f[1] = c.g[V0][0] as u32;
    cvt_w(c, 1);
    load_f(c, 3, s3); // box min x
    c.g[V1][0] = Addr(s1.wrapping_add(0xc)).lw(c); // cell->z
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1)); // sub.level
    c.g[V0][0] = Addr(s1.wrapping_add(8)).lw(c); // cell->y
    let lo = fr(c, 0) - fr(c, 2);
    fw(c, 0, lo);
    let hi = fr(c, 1) + fr(c, 2);
    fw(c, 1, hi);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 1); // sub.z
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1); // sub.y
    let level = c.g[A1][0];
    set_slot(c, SUB_LEVEL, level);
    c.g[FP][0] = 0;
    let x0lo = fr(c, 0) * fr(c, 4);
    fw(c, 15, x0lo);
    let x0hi = fr(c, 1) * fr(c, 4);
    fw(c, 16, x0hi);
    let sy = c.g[V0][0];
    set_slot(c, SUB_Y, sy);
    let sz = c.g[V1][0];
    set_slot(c, SUB_Z, sz);
    let t = !lt_fcc(c, 3, 16);
    let sx_val = c.g[A0][0];
    set_slot(c, SUB_X, sx_val);
    if !t {
        c.cycles += 5;
        load_f(c, 0, s3.wrapping_add(BOX_MAX)); // box max x
        if lt_fcc(c, 15, 0) {
            c.g[FP][0] = 1;
        }
    }
}

/// x1 slab (ox1 in frame word 0x10).
fn x1_slab(c: &mut Cpu) {
    let s3 = c.g[S3][0] as u32;
    c.cycles += 18;
    c.g[V0][0] = slot(c, SUB_X);
    load_gp_f(c, 3, 0x2cd8);
    let v1 = (c.g[V0][0] as u32).wrapping_add(2);
    c.g[V1][0] = sx(v1);
    set_slot(c, F_OX1, 0);
    c.f[1] = v1;
    cvt_w(c, 1);
    let v0 = sx((c.g[V0][0] as u32).wrapping_add(1));
    c.g[V0][0] = v0;
    c.f[0] = v0 as u32;
    cvt_w(c, 0);
    load_f(c, 2, s3); // box min x
    let hi = fr(c, 1) + fr(c, 3);
    fw(c, 1, hi);
    c.f[13] = c.f[2];
    let lo = fr(c, 0) - fr(c, 3);
    fw(c, 0, lo);
    let x1hi = fr(c, 1) * fr(c, 4);
    fw(c, 14, x1hi);
    let t = !lt_fcc(c, 2, 14);
    let x1lo = fr(c, 0) * fr(c, 4);
    fw(c, 12, x1lo);
    if !t {
        c.cycles += 5;
        load_f(c, 0, s3.wrapping_add(BOX_MAX));
        let t2 = !lt_fcc(c, 12, 0);
        c.g[V0][0] = 1;
        if !t2 {
            c.cycles += 1;
            let one = c.g[V0][0];
            set_slot(c, F_OX1, one);
        }
    }
}

/// y0 slab (oy0 in frame word 0x14).
fn y0_slab(c: &mut Cpu) {
    let s3 = c.g[S3][0] as u32;
    c.cycles += 17;
    load_gp_f(c, 3, 0x2cd4);
    let v0 = sx((c.g[V1][0] as u32).wrapping_add(1));
    c.g[V0][0] = v0;
    set_slot(c, F_OY0, 0);
    c.f[1] = v0 as u32;
    cvt_w(c, 1);
    c.g[A0][0] = c.g[V1][0];
    c.f[0] = c.g[V1][0] as u32;
    cvt_w(c, 0);
    load_f(c, 2, s3.wrapping_add(4)); // box min y
    let hi = fr(c, 1) + fr(c, 3);
    fw(c, 1, hi);
    c.f[6] = c.f[2];
    let lo = fr(c, 0) - fr(c, 3);
    fw(c, 0, lo);
    let y0hi = fr(c, 1) * fr(c, 4);
    fw(c, 11, y0hi);
    let t = !lt_fcc(c, 2, 11);
    let y0lo = fr(c, 0) * fr(c, 4);
    fw(c, 10, y0lo);
    if !t {
        c.cycles += 5;
        load_f(c, 0, s3.wrapping_add(BOX_MAX + 4));
        let t2 = !lt_fcc(c, 10, 0);
        c.g[V0][0] = 1;
        if !t2 {
            c.cycles += 1;
            let one = c.g[V0][0];
            set_slot(c, F_OY0, one);
        }
    }
}

/// y1 slab (oy1 in frame word 0x18).
fn y1_slab(c: &mut Cpu) {
    let s3 = c.g[S3][0] as u32;
    c.cycles += 15;
    let v1 = sx((c.g[A0][0] as u32).wrapping_add(2));
    c.g[V1][0] = v1;
    load_gp_f(c, 2, 0x2cd0);
    c.f[1] = v1 as u32;
    cvt_w(c, 1);
    let v0 = sx((c.g[A0][0] as u32).wrapping_add(1));
    c.g[V0][0] = v0;
    c.f[0] = v0 as u32;
    cvt_w(c, 0);
    set_slot(c, F_OY1, 0);
    let hi = fr(c, 1) + fr(c, 2);
    fw(c, 1, hi);
    let lo = fr(c, 0) - fr(c, 2);
    fw(c, 0, lo);
    let y1hi = fr(c, 1) * fr(c, 4);
    fw(c, 9, y1hi);
    let t = !lt_fcc(c, 6, 9);
    let y1lo = fr(c, 0) * fr(c, 4);
    fw(c, 8, y1lo);
    if !t {
        c.cycles += 5;
        load_f(c, 0, s3.wrapping_add(BOX_MAX + 4));
        let t2 = !lt_fcc(c, 8, 0);
        c.g[V0][0] = 1;
        if !t2 {
            c.cycles += 1;
            let one = c.g[V0][0];
            set_slot(c, F_OY1, one);
        }
    }
}

/// z0 slab (oz0 in frame word 0x1c).
fn z0_slab(c: &mut Cpu) {
    let s3 = c.g[S3][0] as u32;
    c.cycles += 17;
    load_gp_f(c, 3, 0x2ccc);
    let v0 = sx((c.g[V1][0] as u32).wrapping_add(1));
    c.g[V0][0] = v0;
    set_slot(c, F_OZ0, 0);
    c.f[1] = v0 as u32;
    cvt_w(c, 1);
    c.g[A0][0] = c.g[V1][0];
    c.f[0] = c.g[V1][0] as u32;
    cvt_w(c, 0);
    load_f(c, 2, s3.wrapping_add(8)); // box min z
    let hi = fr(c, 1) + fr(c, 3);
    fw(c, 1, hi);
    c.f[5] = c.f[2];
    let lo = fr(c, 0) - fr(c, 3);
    fw(c, 0, lo);
    let z0hi = fr(c, 1) * fr(c, 4);
    fw(c, 7, z0hi);
    let t = !lt_fcc(c, 2, 7);
    let z0lo = fr(c, 0) * fr(c, 4);
    fw(c, 3, z0lo);
    if !t {
        c.cycles += 5;
        load_f(c, 0, s3.wrapping_add(BOX_MAX + 8));
        let t2 = !lt_fcc(c, 3, 0);
        c.g[V0][0] = 1;
        if !t2 {
            c.cycles += 1;
            let one = c.g[V0][0];
            set_slot(c, F_OZ0, one);
        }
    }
}

/// z1 slab (oz1 in frame word 0x20).
fn z1_slab(c: &mut Cpu) {
    let s3 = c.g[S3][0] as u32;
    c.cycles += 15;
    let v1 = sx((c.g[A0][0] as u32).wrapping_add(2));
    c.g[V1][0] = v1;
    load_gp_f(c, 2, 0x2cc8);
    c.f[1] = v1 as u32;
    cvt_w(c, 1);
    let v0 = sx((c.g[A0][0] as u32).wrapping_add(1));
    c.g[V0][0] = v0;
    c.f[0] = v0 as u32;
    cvt_w(c, 0);
    set_slot(c, F_OZ1, 0);
    let hi = fr(c, 1) + fr(c, 2);
    fw(c, 1, hi);
    let lo = fr(c, 0) - fr(c, 2);
    fw(c, 0, lo);
    let z1hi = fr(c, 1) * fr(c, 4);
    fw(c, 1, z1hi);
    let t = !lt_fcc(c, 5, 1);
    let z1lo = fr(c, 0) * fr(c, 4);
    fw(c, 2, z1lo);
    if !t {
        c.cycles += 5;
        load_f(c, 0, s3.wrapping_add(BOX_MAX + 8));
        let t2 = !lt_fcc(c, 2, 0);
        c.g[V0][0] = 1;
        if !t2 {
            c.cycles += 1;
            let one = c.g[V0][0];
            set_slot(c, F_OZ1, one);
        }
    }
}

/// `cx0`: the x0 core inside the box (kept in s7).: the x0 core inside the box (s7).
fn cx0_block(c: &mut Cpu) {
    let s3 = c.g[S3][0] as u32;
    c.cycles += 4;
    let t = !lt_fcc(c, 13, 15);
    c.g[S7][0] = 0;
    if !t {
        c.cycles += 5;
        load_f(c, 0, s3.wrapping_add(BOX_MAX));
        if lt_fcc(c, 16, 0) {
            c.g[S7][0] = 1;
        }
    }
}

/// `cx1`: the x1 core inside the box.
fn cx1_block(c: &mut Cpu) {
    let s3 = c.g[S3][0] as u32;
    c.cycles += 4;
    let t = !lt_fcc(c, 13, 12);
    set_slot(c, F_CX1, 0);
    if !t {
        c.cycles += 5;
        load_f(c, 0, s3.wrapping_add(BOX_MAX));
        let t2 = !lt_fcc(c, 14, 0);
        c.g[V0][0] = 1;
        if !t2 {
            c.cycles += 1;
            let one = c.g[V0][0];
            set_slot(c, F_CX1, one);
        }
    }
}

/// `cy0`: the y0 core inside the box.
fn cy0_block(c: &mut Cpu) {
    let s3 = c.g[S3][0] as u32;
    c.cycles += 4;
    let t = !lt_fcc(c, 6, 10);
    set_slot(c, F_CY0, 0);
    if !t {
        c.cycles += 5;
        load_f(c, 0, s3.wrapping_add(BOX_MAX + 4));
        let t2 = !lt_fcc(c, 11, 0);
        c.g[V0][0] = 1;
        if !t2 {
            c.cycles += 1;
            let one = c.g[V0][0];
            set_slot(c, F_CY0, one);
        }
    }
}

/// `cy1`: the y1 core inside the box.
fn cy1_block(c: &mut Cpu) {
    let s3 = c.g[S3][0] as u32;
    c.cycles += 4;
    let t = !lt_fcc(c, 6, 8);
    set_slot(c, F_CY1, 0);
    if !t {
        c.cycles += 5;
        load_f(c, 0, s3.wrapping_add(BOX_MAX + 4));
        let t2 = !lt_fcc(c, 9, 0);
        c.g[V0][0] = 1;
        if !t2 {
            c.cycles += 1;
            let one = c.g[V0][0];
            set_slot(c, F_CY1, one);
        }
    }
}

/// `cz0`: the z0 core inside the box.
fn cz0_block(c: &mut Cpu) {
    let s3 = c.g[S3][0] as u32;
    c.cycles += 4;
    let t = !lt_fcc(c, 5, 3);
    set_slot(c, F_CZ0, 0);
    if !t {
        c.cycles += 5;
        load_f(c, 0, s3.wrapping_add(BOX_MAX + 8));
        let t2 = !lt_fcc(c, 7, 0);
        c.g[V0][0] = 1;
        if !t2 {
            c.cycles += 1;
            let one = c.g[V0][0];
            set_slot(c, F_CZ0, one);
        }
    }
}

/// `cz1`: the z1 core inside the box.
fn cz1_block(c: &mut Cpu) {
    let s3 = c.g[S3][0] as u32;
    c.cycles += 4;
    let t = !lt_fcc(c, 5, 2);
    set_slot(c, F_CZ1, 0);
    if !t {
        c.cycles += 5;
        load_f(c, 0, s3.wrapping_add(BOX_MAX + 8));
        let t2 = !lt_fcc(c, 1, 0);
        c.g[V0][0] = 1;
        if !t2 {
            c.cycles += 1;
            let one = c.g[V0][0];
            set_slot(c, F_CZ1, one);
        }
    }
}

/// All the flag tests before the children. `false`: the box misses the cell
/// on some axis (the original returns without visiting children).
fn overlap_flags(c: &mut Cpu) -> bool {
    x0_slab(c);
    x1_slab(c);
    c.cycles += 2;
    let ox0 = c.g[FP][0] != 0;
    c.g[V1][0] = slot(c, SUB_Y);
    if !ox0 {
        c.cycles += 3;
        c.g[V0][0] = slot(c, F_OX1);
        if c.g[V0][0] == 0 {
            return false;
        }
    }
    y0_slab(c);
    y1_slab(c);
    c.cycles += 3;
    c.g[V0][0] = slot(c, F_OY0);
    let oy0 = c.g[V0][0] != 0;
    c.g[V1][0] = slot(c, SUB_Z);
    if !oy0 {
        c.cycles += 3;
        c.g[V0][0] = slot(c, F_OY1);
        if c.g[V0][0] == 0 {
            return false;
        }
    }
    z0_slab(c);
    z1_slab(c);
    c.cycles += 3;
    c.g[V0][0] = slot(c, F_OZ0);
    let oz0 = c.g[V0][0] != 0;
    c.g[V0][0] = slot(c, F_OZ1);
    if !oz0 {
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            return false;
        }
    }
    cx0_block(c);
    cx1_block(c);
    cy0_block(c);
    cy1_block(c);
    cz0_block(c);
    cz1_block(c);
    true
}

// ---------------------------------------------------------------------------
// The children.

/// Leave a child's visit: the tail charges one cycle and loads the next
/// coordinate into v0 (when the child has a successor).
fn leave(c: &mut Cpu, v: &Visit, tail: bool) -> Flow {
    if tail {
        c.cycles += 1;
        load_v0(c, v.next);
    }
    Ok(())
}

/// The recursion into a child that only partly overlaps the box.
fn recurse(c: &mut Cpu, v: &Visit) -> Flow {
    c.cycles += 6;
    c.g[A0][0] = c.g[S6][0];
    c.g[A1][0] = c.g[S4][0];
    c.g[A2][0] = c.g[S2][0];
    c.g[A3][0] = c.g[S3][0];
    c.g[T1][0] = c.g[SP][0];
    COLLIDE_RECURSIVE.call(c, v.rec_ret)?;
    leave(c, v, true)
}

/// A child the box fully covers: report its objects to `ctx2` and then its
/// whole subtree, child by child.
fn collect(c: &mut Cpu, v: &Visit) -> Flow {
    c.cycles += 3;
    let head = Addr((c.g[S1][0] as u32).wrapping_add(NODE_OBJECTS)).lw(c);
    c.g[S0][0] = head;
    if head == 0 {
        let first = Addr(c.g[S1][0] as u32).lw(c);
        c.g[A0][0] = first;
    } else {
        c.cycles += v.pre_obj;
        c.g[A1][0] = c.g[S0][0];
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S2][0];
            COLLIDE_OBJECT.call(c, v.obj_ret)?;
            let next = Addr(c.g[S0][0] as u32).lw(c);
            c.g[S0][0] = next;
            c.g[A1][0] = next;
            if next == 0 {
                c.cycles += 1;
                let first = Addr(c.g[S1][0] as u32).lw(c);
                c.g[A0][0] = first;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, v.obj_ret - 8);
            }
        }
    }
    // children 0..6: call on the pointer in a0, then load the next one
    for k in 0..7u32 {
        c.cycles += 2;
        if c.g[A0][0] == 0 {
            let next = Addr((c.g[S1][0] as u32).wrapping_add(4 * (k + 1))).lw(c);
            c.g[A0][0] = next;
        } else {
            c.cycles += 3;
            c.g[A1][0] = c.g[S2][0];
            COLLIDE_SUBTREE.call(c, v.sub_ret[k as usize])?;
            let next = Addr((c.g[S1][0] as u32).wrapping_add(4 * (k + 1))).lw(c);
            c.g[A0][0] = next;
        }
    }
    // child 7
    c.cycles += 2;
    let null = c.g[A0][0] == 0;
    load_v0(c, v.next);
    if null {
        return Ok(());
    }
    c.cycles += 4;
    c.g[A1][0] = c.g[S2][0];
    COLLIDE_SUBTREE.call(c, v.sub_ret[7])?;
    load_v0(c, v.next);
    Ok(())
}

/// One child: the overlap test of its eight-way position, then either
/// collect, recurse, or nothing.
fn visit(c: &mut Cpu, v: &Visit) -> Flow {
    c.cycles += v.enter;
    let child = Addr((c.g[S5][0] as u32).wrapping_add(v.child)).lw(c);
    c.g[S1][0] = child;
    if v.step == 0 {
        c.g[V0][0] = slot(c, v.coord);
    } else {
        let stepped = sx((c.g[V0][0] as u32).wrapping_add(v.step as u32));
        c.g[V0][0] = stepped;
        set_slot(c, v.coord, stepped);
    }
    if child == 0 {
        return leave(c, v, !v.s1_end);
    }
    // ox (x0 children read s8; x1 children read a frame word)
    c.cycles += v.first;
    let ox_clear = flag_is_clear(c, v.ox);
    c.g[V0][0] = slot(c, v.oy);
    if ox_clear {
        return leave(c, v, true);
    }
    c.cycles += 2;
    let oy_clear = slot(c, v.oy) == 0;
    c.g[V0][0] = slot(c, v.oz);
    if oy_clear {
        return leave(c, v, true);
    }
    c.cycles += 2;
    let oz_clear = slot(c, v.oz) == 0;
    c.g[V0][0] = slot(c, v.after_oz);
    if oz_clear {
        return leave(c, v, !v.oz_end);
    }
    // cx, cy, cz: fully inside on every axis collects, else recurses
    c.cycles += 2;
    let cx_clear = flag_is_clear(c, v.cx);
    c.g[V0][0] = slot(c, v.cy);
    if !cx_clear {
        c.cycles += 2;
        let cy_clear = slot(c, v.cy) == 0;
        c.g[V0][0] = slot(c, v.cz);
        if !cy_clear {
            c.cycles += 2;
            let cz_clear = slot(c, v.cz) == 0;
            c.g[T0][0] = c.g[S1][0];
            if !cz_clear {
                return collect(c, v);
            }
            return recurse(c, v);
        }
    }
    c.cycles += 1;
    c.g[T0][0] = c.g[S1][0];
    recurse(c, v)
}

// ---------------------------------------------------------------------------
// The body and the entry.

/// The body after the prologue: the node's own objects, then the cell's
/// eight children (unless the cell is at the leaf level, 11).
fn cell_body(c: &mut Cpu) -> Flow {
    if c.g[S0][0] != 0 {
        c.cycles += 1;
        c.g[A1][0] = c.g[S0][0];
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S4][0];
            COLLIDE_OBJECT.call(c, 0x0033_e030)?;
            let next = Addr(c.g[S0][0] as u32).lw(c);
            c.g[S0][0] = next;
            c.g[A1][0] = next;
            if next == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0033_e028);
            }
        }
    }
    c.cycles += 4;
    c.g[A1][0] = Addr(c.g[S1][0] as u32).lw(c); // cell level
    c.g[V0][0] = 0xb;
    let leaf = c.g[A1][0] == c.g[V0][0];
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7e));
    if leaf {
        c.cycles += 1;
        return Ok(());
    }
    if !overlap_flags(c) {
        return Ok(());
    }
    for v in VISITS.iter() {
        visit(c, v)?;
    }
    Ok(())
}

/// `void Cell_CollideObjectsRecursive(flag a0, ctx a1, ctx2 a2, box a3,
/// node t0, cell key t1)` (0x33dfd8): report the static objects of one
/// octree cell that a box touches to `ctx`, and the subtrees of its children
/// that the box covers to `ctx2`; the partly covered children recurse.
pub fn cell_collide_objects_recursive(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0xe0);
        frame.save(c, S1, 0xc0);
        frame.save(c, S2, 0xb0);
        frame.save(c, S3, 0xa0);
        frame.save(c, S4, 0x90);
        frame.save(c, S5, 0x80);
        frame.save(c, S6, 0x70);
        frame.save(c, S7, 0x60);
        frame.save(c, FP, 0x50);
        frame.save_ra(c, 0x40);
        frame.save(c, S0, 0xd0);
        c.g[S1][0] = c.g[T1][0];
        c.g[S2][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A3][0];
        c.g[S4][0] = c.g[A1][0];
        c.g[S5][0] = c.g[T0][0];
        let node_objects = Addr((c.g[S5][0] as u32).wrapping_add(NODE_OBJECTS)).lw(c);
        c.g[S0][0] = node_objects;
        c.g[S6][0] = c.g[A0][0];
        cell_body(c)?;
        // epilogue: every exit above falls through to here (the body's own
        // exits charge their tail cycles before returning)
        c.cycles += 11;
        frame.restore(c, S0, 0xd0);
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore(c, S5, 0x80);
        frame.restore(c, S6, 0x70);
        frame.restore(c, S7, 0x60);
        frame.restore(c, FP, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, Rng, DATA, GAME_GP};
    use ssx3_ee_rt::cpu::reg::*;

    const NODE: u32 = DATA + 0x1000;
    const CHILD: u32 = DATA + 0x2000;
    const CELL: u32 = DATA + 0x3000;
    const BOX: u32 = DATA + 0x3100;
    const OBJ: u32 = DATA + 0x3200;
    const CTX: u32 = DATA + 0x3300;
    const CTX2: u32 = DATA + 0x3400;
    const FLAG: u32 = DATA + 0x3500;
    /// The float constants the body loads off gp (0.2f each).
    const GP_CONSTS: [u32; 6] = [0x2cdc, 0x2cd8, 0x2cd4, 0x2cd0, 0x2ccc, 0x2cc8];

    #[test]
    fn cell_collide_objects_recursive_matches() {
        check(
            ee::s11::f_0033dfd8__generated,
            cell_collide_objects_recursive,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x0471, 0);
                for off in GP_CONSTS {
                    c.bus.write32(GAME_GP.wrapping_sub(off), 0x3e4c_cccd);
                }
                // the cell: level 8..=11, a key of small coordinates
                c.bus.write32(CELL, 8 + r.next_u32() % 4);
                c.bus.write32(CELL + 4, r.next_u32() % 4);
                c.bus.write32(CELL + 8, r.next_u32() % 4);
                c.bus.write32(CELL + 0xc, r.next_u32() % 4);
                // the box, some misses on some axes
                for i in 0..3u32 {
                    let min = (r.next_u32() % 8000) as f32 - 1000.0;
                    let max = min + (r.next_u32() % 3000) as f32;
                    c.bus.write32(BOX + 4 * i, min.to_bits());
                    c.bus.write32(BOX + 0x10 + 4 * i, max.to_bits());
                }
                // the node: children (some null), empty zeroed children, and
                // every fifth seed one object in the node's list
                for k in 0..8u32 {
                    let p = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        CHILD + 0x40 * k
                    };
                    c.bus.write32(NODE + 4 * k, p);
                }
                c.bus.write32(NODE + NODE_OBJECTS, if seed % 5 == 0 { OBJ } else { 0 });
                c.g[A0][0] = sx(FLAG);
                c.g[A1][0] = sx(CTX);
                c.g[A2][0] = sx(CTX2);
                c.g[A3][0] = sx(BOX);
                c.g[T0][0] = sx(NODE);
                c.g[T1][0] = sx(CELL);
                c.pc = 0x0033_dfd8;
            },
        );
    }
}
