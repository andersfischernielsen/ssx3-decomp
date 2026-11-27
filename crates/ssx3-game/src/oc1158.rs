//! Staging module oc1158 (wave wave273): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1158`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
    0x003c_aa28,
    stretch_crossfade,
    "SND_DspStretch_Crossfade",
),
    (
        0x003c_ab28,
        stretch_choose_splice,
        "SND_DspStretch_ChooseSplice",
    ),
    (
        0x003c_ac88,
        stretch_splice,
        "SND_DspStretch_Splice",
    ),
    (
        0x003c_adf0,
        stretch_drain,
        "SND_DspStretch_Drain",
    ),
    (
        0x003c_aea0,
        stretch_calc_input,
        "SND_DspStretch_CalcInput",
    ),
    (
        0x003c_afd0,
        stretch_run,
        "SND_DspStretch_Run",
    ),
    (
        0x003c_b110,
        stretch_set_ratio,
        "SND_DspStretch_SetRatio",
    ),
    (
        0x003c_b1a8,
        stretch_process,
        "SND_DspStretch_Process",
    ),
];

/// Read float register `n` as the FPU sees it.
#[inline]
fn stretch_fr(c: &Cpu, n: usize) -> crate::ps2f::FpuF32 {
    crate::ps2f::FpuF32(c.f[n])
}

/// Write float register `n`.
#[inline]
fn stretch_set_fr(c: &mut Cpu, n: usize, v: crate::ps2f::FpuF32) {
    c.f[n] = v.0;
}

/// `void SND_DspStretch_Crossfade(k, n, ...)` (0x3caa28): blends the current
/// and next block over the `n` samples of a splice. `t0` is the splice offset
/// `k` (negated when it is not positive), `a3` is `n`. The first loop blends
/// `cur[i]` and `cur[i-k]` into `a2`, the second blends `next` into the tail
/// of `a2`, both with a weight stepped by `1/n` (or `-1/n` when `k > 0`).
/// Leaves `f0`/`f1`/`f2`/`f3`/`f4`/`f5` and `at` as the generated body does.
pub fn stretch_crossfade(c: &mut Cpu) {
    // Weight and step: k > 0 starts at weight 1 and steps by -1/n; k <= 0
    // negates k, starts at weight 0 and steps by 1/n.
    c.cycles += 2;
    if (c.g[T0][0] as i64) <= 0 {
        c.g[T0][0] = sx(0u32.wrapping_sub(c.g[T0][0] as u32));
        c.cycles += 5;
        stretch_set_fr(c, 1, crate::ps2f::FpuF32::from_i32(c.g[A3][0] as u32 as i32));
        c.g[AT][0] = 0x3f80_0000;
        stretch_set_fr(c, 0, crate::ps2f::FpuF32(c.g[AT][0] as u32));
        stretch_set_fr(c, 3, crate::ps2f::FpuF32(0));
    } else {
        c.cycles += 8;
        stretch_set_fr(c, 1, crate::ps2f::FpuF32::from_i32(c.g[A3][0] as u32 as i32));
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        stretch_set_fr(c, 0, crate::ps2f::FpuF32(c.g[AT][0] as u32));
        c.g[AT][0] = 0x3f80_0000;
        stretch_set_fr(c, 3, crate::ps2f::FpuF32(c.g[AT][0] as u32));
    }

    // f4 = step = f0 / f1.
    c.cycles += 4;
    let step = stretch_fr(c, 0).div(stretch_fr(c, 1));
    stretch_set_fr(c, 4, step);
    c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[A3][0] as i64)) as u64;
    if c.g[V0][0] != 0 {
        // First blend: cur[i] (from a0 + 4k) with cur[i - k] (from a0), out at a2.
        c.cycles += 7;
        c.g[AT][0] = 0x3f80_0000;
        stretch_set_fr(c, 5, crate::ps2f::FpuF32(c.g[AT][0] as u32));
        c.g[V0][0] = sx((c.g[T0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[T2][0] = c.g[A2][0];
        c.g[T1][0] = c.g[A0][0];
        c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub(c.g[T0][0] as u32));
        loop {
            c.cycles += 13;
            let one_minus_w = stretch_fr(c, 5) - stretch_fr(c, 3);
            stretch_set_fr(c, 0, one_minus_w);
            { let __v = Addr(c.g[V0][0] as u32).read_fpu(c); stretch_set_fr(c, 1, __v) };
            { let __v = Addr(c.g[T1][0] as u32).read_fpu(c); stretch_set_fr(c, 2, __v) };
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
            stretch_set_fr(c, 1, stretch_fr(c, 3) * stretch_fr(c, 1));
            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x4));
            stretch_set_fr(c, 0, stretch_fr(c, 0) * stretch_fr(c, 2));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            stretch_set_fr(c, 3, stretch_fr(c, 3) + stretch_fr(c, 4));
            stretch_set_fr(c, 1, stretch_fr(c, 1) + stretch_fr(c, 0));
            Addr(c.g[T2][0] as u32).write_fpu(c, stretch_fr(c, 1));
            let more = c.g[V1][0] != 0;
            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_aa90);
            }
        }
    }

    // Second blend: `next` (from a0 + 4(n-k)) into the tail of a2.
    c.cycles += 2;
    let skip = (c.g[T0][0] as i64) <= 0;
    c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_sub(c.g[T0][0] as u32));
    if !skip {
        c.cycles += 7;
        c.g[AT][0] = 0x3f80_0000;
        stretch_set_fr(c, 5, crate::ps2f::FpuF32(c.g[AT][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V1][0] = c.g[T0][0];
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        loop {
            c.cycles += 13;
            let one_minus_w = stretch_fr(c, 5) - stretch_fr(c, 3);
            stretch_set_fr(c, 0, one_minus_w);
            { let __v = Addr(c.g[A1][0] as u32).read_fpu(c); stretch_set_fr(c, 1, __v) };
            { let __v = Addr(c.g[A0][0] as u32).read_fpu(c); stretch_set_fr(c, 2, __v) };
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
            stretch_set_fr(c, 1, stretch_fr(c, 3) * stretch_fr(c, 1));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
            stretch_set_fr(c, 0, stretch_fr(c, 0) * stretch_fr(c, 2));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
            stretch_set_fr(c, 3, stretch_fr(c, 3) + stretch_fr(c, 4));
            stretch_set_fr(c, 1, stretch_fr(c, 1) + stretch_fr(c, 0));
            Addr(c.g[V0][0] as u32).write_fpu(c, stretch_fr(c, 1));
            let more = c.g[V1][0] != 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_aae8);
            }
        }
    }

    c.cycles += 2;
    ret(c);
}

/// `SND_DspStretch_ChooseSplice` (0x003cab28): the normal entry, converted from the generated body
/// (labelled-block body: every register, cycle and call kept).
pub fn stretch_choose_splice(c: &mut Cpu) {
    guest(c, |c| {
'b_3cabd8: {
c.cycles += 11;
c.g[A3][0] = c.g[A0][0];
c.g[V1][0] = c.lw((c.g[A1][0] as u32));
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 0);
c.g[V0][0] = sx((c.g[A3][0] as u32) >> 31);
c.g[A0][0] = c.lbu((c.g[V1][0] as u32));
c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.c_lt_s(12, 0);
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
let t = !c.fcc();
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
if t {
'b_3cac44: {
c.cycles += 16;
c.mtc1(7, 1);
c.cvt_s_w(1, 1);
c.lwc1(3, (c.g[A2][0] as u32));
c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.mtc1(0, 4);
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.mul_s(2, 12, 1);
c.add_s(1, 1, 3);
c.add_s(0, 0, 3);
c.sub_s(3, 1, 2);
c.sub_s(1, 0, 2);
c.c_lt_s(3, 4);
let t = !c.fcc();
c.mov_s(5, 3);
if t {
break 'b_3cac44;
} else {
c.cycles += 1;
c.neg_s(5, 3);
break 'b_3cac44;
}
}
'b_3cac68: {
'b_3cac60: {
c.cycles += 3;
c.c_lt_s(1, 4);
let t = !c.fcc();

if t {
break 'b_3cac60;
} else {
c.cycles += 4;
c.neg_s(0, 1);
c.c_lt_s(5, 0);
let t = true;

if t {
break 'b_3cac68;
} else {
break 'b_3cac60;
}
}
}
c.cycles += 2;
c.c_lt_s(5, 1);
break 'b_3cac68;
}
c.cycles += 2;
let t = c.fcc();
c.g[V1][0] = c.lw((c.g[A1][0] as u32));
if t {
break 'b_3cabd8;
} else {
c.cycles += 5;
c.g[V0][0] = c.g[A0][0];
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
c.sw((c.g[A1][0] as u32), c.g[V1][0]);
let t = c.g[RA][0] as u32;
c.swc1(1, (c.g[A2][0] as u32));
c.pc = t; return Ok(());
}
} else {
'b_3caba0: {
c.cycles += 18;
c.mtc1(7, 1);
c.cvt_s_w(1, 1);
c.lwc1(5, (c.g[A2][0] as u32));
c.g[V0][0] = sx((c.g[A3][0] as u32) << 1);
c.add_s(2, 12, 12);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
c.mtc1(0, 6);
c.add_s(4, 1, 5);
c.mul_s(3, 12, 1);
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.mul_s(2, 2, 1);
c.sub_s(3, 4, 3);
c.add_s(0, 0, 5);
c.c_lt_s(3, 6);
c.mov_s(5, 3);
let t = !c.fcc();
c.sub_s(1, 0, 2);
if t {
break 'b_3caba0;
} else {
c.cycles += 1;
c.neg_s(5, 3);
break 'b_3caba0;
}
}
'b_3cabf0: {
'b_3cabc8: {
c.cycles += 3;
c.c_lt_s(1, 6);
let t = !c.fcc();

if t {
break 'b_3cabc8;
} else {
c.cycles += 5;
c.neg_s(0, 1);
c.c_lt_s(5, 0);
let t = c.fcc();
c.g[V1][0] = c.lw((c.g[A1][0] as u32));
if t {
break 'b_3cabd8;
} else {
c.cycles += 2;
let t = true;
c.g[V0][0] = sx(0u32.wrapping_sub((c.g[A0][0] as u32)));
if t {
break 'b_3cabf0;
} else {
break 'b_3cabc8;
}
}
}
}
c.cycles += 4;
c.c_lt_s(5, 1);
let t = !c.fcc();
c.g[V1][0] = c.lw((c.g[A1][0] as u32));
if t {
c.cycles += 1;
c.g[V0][0] = sx(0u32.wrapping_sub((c.g[A0][0] as u32)));
break 'b_3cabf0;
} else {
break 'b_3cabd8;
}
}
c.cycles += 4;
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
c.sw((c.g[A1][0] as u32), c.g[V1][0]);
let t = c.g[RA][0] as u32;
c.swc1(1, (c.g[A2][0] as u32));
c.pc = t; return Ok(());
}
}
c.cycles += 5;
c.g[V0][0] = 0u64;
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
c.sw((c.g[A1][0] as u32), c.g[V1][0]);
let t = c.g[RA][0] as u32;
c.swc1(3, (c.g[A2][0] as u32));
c.pc = t; return Ok(());
    });
}

/// `SND_DspStretch_Splice` (0x003cac88): the normal entry, converted from the generated body
/// (labelled-block body: every register, cycle and call kept).
pub fn stretch_splice(c: &mut Cpu) {
    guest(c, |c| {
'b_3cacd8: {
'b_3cacc8: {
c.cycles += 13;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S1][0] = c.g[A0][0];
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[S3][0] = c.g[A2][0];
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
let t = (c.g[V0][0] as i64) <= 0;
c.g[S5][0] = c.g[A1][0];
if t {
break 'b_3cacc8;
} else {
c.cycles += 3;
c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x40));
let t = true;
c.g[S4][0] = c.lw((c.g[S5][0] as u32));
if t {
break 'b_3cacd8;
} else {
break 'b_3cacc8;
}
}
}
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2c));
c.g[S2][0] = c.lw((c.g[S5][0] as u32));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
break 'b_3cacd8;
}
'b_3cadc4: {
'b_3cad0c: {
c.cycles += 2;
let t = c.g[S3][0] != 0u64;
c.g[A1][0] = c.g[S2][0];
if t {
break 'b_3cad0c;
} else {
c.cycles += 11;
c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2c));
c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x838));
c.g[RA][0] = sx(0x003cacf0);
c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
call_ptr(c, 0x003c51e8, 0x003cacf0)?;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2c));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
c.sw((c.g[S1][0] as u32).wrapping_add(0x38), c.g[V1][0]);
c.sw((c.g[S1][0] as u32).wrapping_add(0x30), c.g[V0][0]);
let t = true;
c.sw((c.g[S5][0] as u32), c.g[S4][0]);
if t {
break 'b_3cadc4;
} else {
break 'b_3cad0c;
}
}
}
'b_3cad60: {
c.cycles += 2;
let t = (c.g[S3][0] as i64) <= 0;
c.g[S0][0] = sx((c.g[S3][0] as u32) << 2);
if t {
break 'b_3cad60;
} else {
c.cycles += 19;
c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x838));
c.g[A2][0] = c.g[S0][0];
c.g[RA][0] = sx(0x003cad24);
c.g[A1][0] = c.g[S2][0];
call_ptr(c, 0x003c51e8, 0x003cad24)?;
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x838));
c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2c));
c.g[A0][0] = c.g[S2][0];
c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
c.g[A1][0] = c.g[S4][0];
c.g[RA][0] = sx(0x003cad40);
c.g[T0][0] = c.g[S3][0];
Callee { addr: 0x003caa28, f: ee::s14::SND_DspStretch_Crossfade }.call(c, 0x003cad40)?;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2c));
c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S3][0] as u32)));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
c.sw((c.g[S1][0] as u32).wrapping_add(0x30), c.g[V0][0]);
c.sw((c.g[S1][0] as u32).wrapping_add(0x38), c.g[A0][0]);
let t = true;
c.sw((c.g[S5][0] as u32), c.g[S4][0]);
if t {
break 'b_3cadc4;
} else {
break 'b_3cad60;
}
}
}
c.cycles += 25;
c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2c));
c.g[A0][0] = c.g[S2][0];
c.g[A1][0] = c.g[S4][0];
c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x838));
c.g[RA][0] = sx(0x003cad78);
c.g[T0][0] = c.g[S3][0];
Callee { addr: 0x003caa28, f: ee::s14::SND_DspStretch_Crossfade }.call(c, 0x003cad78)?;
c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2c));
c.g[A1][0] = sx((c.g[S3][0] as u32) << 2);
c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_sub((c.g[A1][0] as u32)));
c.g[A0][0] = sx((c.g[A2][0] as u32) << 2);
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[S3][0] as u32)));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x838));
c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[RA][0] = sx(0x003cad9c);
c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
call_ptr(c, 0x003c51e8, 0x003cad9c)?;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2c));
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
c.g[A0][0] = sx((c.g[V0][0] as u32) << 1);
c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.sw((c.g[S1][0] as u32).wrapping_add(0x30), c.g[V1][0]);
c.sw((c.g[S1][0] as u32).wrapping_add(0x38), c.g[A1][0]);
c.sw((c.g[S5][0] as u32), c.g[V0][0]);
break 'b_3cadc4;
}
c.cycles += 11;
c.sw((c.g[S1][0] as u32).wrapping_add(0x3c), 0u64);
c.sw((c.g[S1][0] as u32).wrapping_add(0x34), 0u64);
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
c.pc = t; return Ok(());
    });
}

/// `SND_DspStretch_Drain` (0x003cadf0): the normal entry, converted from the generated body
/// (labelled-block body: every register, cycle and call kept).
pub fn stretch_drain(c: &mut Cpu) {
    guest(c, |c| {
c.cycles += 43;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S1][0] = c.g[A0][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[S2][0] = c.g[A1][0];
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[S3][0] = c.g[A2][0];
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
c.g[V1][0] = c.lw((c.g[S2][0] as u32));
c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
c.g[A0][0] = c.lw((c.g[S3][0] as u32));
if c.g[V0][0] == 0 { c.g[S0][0] = c.g[V1][0]; }
c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
c.g[S4][0] = sx((c.g[S0][0] as u32) << 2);
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x838));
c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[RA][0] = sx(0x003cae48);
c.g[A2][0] = c.g[S4][0];
call_ptr(c, 0x003c51e8, 0x003cae48)?;
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
c.g[V0][0] = c.g[S0][0];
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub((c.g[S0][0] as u32)));
c.sw((c.g[S1][0] as u32).wrapping_add(0x3c), c.g[V1][0]);
c.sw((c.g[S1][0] as u32).wrapping_add(0x38), c.g[A0][0]);
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
c.g[V1][0] = c.lw((c.g[S2][0] as u32));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub((c.g[S0][0] as u32)));
c.sw((c.g[S2][0] as u32), c.g[V1][0]);
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[A0][0] = c.lw((c.g[S3][0] as u32));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
c.sw((c.g[S3][0] as u32), c.g[A0][0]);
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
c.pc = t; return Ok(());
    });
}

/// `SND_DspStretch_CalcInput` (0x003caea0): the normal entry, converted from the generated body
/// (labelled-block body: every register, cycle and call kept).
pub fn stretch_calc_input(c: &mut Cpu) {
    guest(c, |c| {
'b_3cafb4: {
'b_3cafb0: {
c.cycles += 12;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
c.g[S0][0] = c.g[A1][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S1][0] = c.g[A0][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S0][0] as i64)) as u64;
let t = c.g[V0][0] == 0u64;
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
if t {
break 'b_3cafb0;
} else {
'b_3caf7c: {
c.cycles += 8;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1c));
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x28));
c.g[S3][0] = 0u64;
c.sw((c.g[SP][0] as u32), c.g[V0][0]);
c.g[A2][0] = 0u64;
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
let t = (c.g[S0][0] as i64) <= 0;
c.g[S2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
if t {
break 'b_3caf7c;
} else {
c.cycles += 4;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2c));
c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;

if t {
break 'b_3caf7c;
} else {
c.cycles += 2;
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2c));
'l_3caf08: loop {
'b_3caf2c: {
c.cycles += 4;
c.g[V0][0] = sx((c.g[A0][0] as u32) << 1);
c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[A2][0] = 0u64;
if t {
break 'b_3caf2c;
} else {
c.cycles += 5;
c.lwc1(12, (c.g[S1][0] as u32).wrapping_add(0x24));
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4));
c.g[RA][0] = sx(0x003caf28);
c.g[A1][0] = c.g[SP][0];
Callee { addr: 0x003cab28, f: ee::s14::SND_DspStretch_ChooseSplice }.call(c, 0x003caf28)?;
c.g[A2][0] = c.g[V0][0];
break 'b_3caf2c;
}
}
'b_3caf50: {
'b_3caf44: {
c.cycles += 2;
if (c.g[A2][0] as i64) < 0 {
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2c));
break 'b_3caf44;
} else {
c.cycles += 4;
c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2c));
c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
let t = true;
c.g[A1][0] = c.g[V1][0];
if t {
break 'b_3caf50;
} else {
break 'b_3caf44;
}
}
}
c.cycles += 3;
c.g[V1][0] = sx((c.g[V0][0] as u32) << 1);
c.g[A1][0] = c.g[V0][0];
c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
break 'b_3caf50;
}
'b_3caf6c: {
'b_3caf68: {
c.cycles += 4;
c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[S0][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V1][0] as u32)));
if t {
break 'b_3caf68;
} else {
c.cycles += 2;
let t = true;
c.g[S0][0] = 0u64;
if t {
break 'b_3caf6c;
} else {
break 'b_3caf68;
}
}
}
c.cycles += 1;
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
break 'b_3caf6c;
}
c.cycles += 2;
let t = (c.g[S0][0] as i64) <= 0;
c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[A1][0] as i64)) as u64;
if t {
break 'b_3caf7c;
} else {
c.cycles += 2;
if c.g[V0][0] == 0u64 {
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2c));
if event_due(c) { deliver_event_here(c, 0x003caf08); } continue 'l_3caf08;
} else {
break 'b_3caf7c;
}
}
}
}
}
}
'b_3caf8c: {
c.cycles += 2;
let t = (c.g[A2][0] as i64) <= 0;

if t {
break 'b_3caf8c;
} else {
c.cycles += 2;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2c));
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
break 'b_3caf8c;
}
}
'b_3cafa0: {
c.cycles += 2;
let t = (c.g[S0][0] as i64) <= 0;
c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S0][0] as i64)) as u64;
if t {
break 'b_3cafa0;
} else {
c.cycles += 2;
if c.g[V0][0] == 0u64 {
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[S0][0] as u32)));
break 'b_3cafa0;
} else {
c.cycles += 1;
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[S2][0] as u32)));
break 'b_3cafa0;
}
}
}
c.cycles += 4;
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[S3][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
if t {
break 'b_3cafb4;
} else {
break 'b_3cafb0;
}
}
}
c.cycles += 1;
c.g[V0][0] = 0u64;
break 'b_3cafb4;
}
c.cycles += 7;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
c.pc = t; return Ok(());
    });
}

/// `SND_DspStretch_Run` (0x003cafd0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn stretch_run(c: &mut Cpu) {
    if c.pc != 0x003cafd0 {
        return ee::s14::f_003cafd0__generated(c);
    }
    guest(c, |c| {
    let mut pc: u32 = 0x003cafd0;
    loop {
        match pc {
            0x003cafd0 => {
                c.cycles += 17;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                c.g[S1][0] = c.g[A0][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x4));
                c.sw((c.g[SP][0] as u32), c.g[A1][0]);
                c.g[S3][0] = c.g[A2][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[A1][0] = c.g[SP][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
                c.g[RA][0] = sx(0x003cb008);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A3][0]);
                Callee { addr: 0x003cadf0, f: ee::s14::SND_DspStretch_Drain }.call(c, 0x003cb008)?;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32));
                let t = c.g[V1][0] == 0u64; 
                c.g[S2][0] = c.g[V0][0];
                if t { pc = 0x003cb0f4; continue; }
                pc = 0x003cb014; continue;
            }
            0x003cb014 => {
                c.cycles += 2;
                let t = true; 
                c.g[S0][0] = 0u64;
                if t { pc = 0x003cb06c; continue; }
                pc = 0x003cb01c; continue;
            }
            0x003cb01c => {
                c.cycles += 1;
                pc = 0x003cb020; continue;
            }
            0x003cb020 => {
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[A0][0] as u32) << 1);
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64; 
                c.g[S0][0] = 0u64;
                if t { pc = 0x003cb044; continue; }
                pc = 0x003cb030; continue;
            }
            0x003cb030 => {
                c.cycles += 5;
                c.lwc1(12, (c.g[S1][0] as u32).wrapping_add(0x24));
                c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1c));
                c.g[RA][0] = sx(0x003cb040);
                c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x28));
                Callee { addr: 0x003cab28, f: ee::s14::SND_DspStretch_ChooseSplice }.call(c, 0x003cb040)?;
                c.g[S0][0] = c.g[V0][0];
                pc = 0x003cb044; continue;
            }
            0x003cb044 => {
                c.cycles += 10;
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x003cb054);
                c.g[A2][0] = c.g[S0][0];
                Callee { addr: 0x003cac88, f: ee::s14::SND_DspStretch_Splice }.call(c, 0x003cb054)?;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[SP][0];
                c.g[RA][0] = sx(0x003cb064);
                c.g[A2][0] = c.g[S3][0];
                Callee { addr: 0x003cadf0, f: ee::s14::SND_DspStretch_Drain }.call(c, 0x003cb064)?;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                pc = 0x003cb06c; continue;
            }
            0x003cb06c => {
                c.cycles += 2;
                let t = (c.g[V1][0] as i64) <= 0; 
                
                if t { pc = 0x003cb08c; continue; }
                pc = 0x003cb074; continue;
            }
            0x003cb074 => {
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2c));
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let t = c.g[V0][0] == 0u64; 
                c.g[A0][0] = c.g[V1][0];
                if t { if back_edge_unwinds(c, 0x003cb020) { return Ok(()); } pc = 0x003cb020; continue; }
                pc = 0x003cb08c; continue;
            }
            0x003cb08c => {
                c.cycles += 2;
                let t = (c.g[S0][0] as i64) <= 0; 
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x40));
                if t { pc = 0x003cb0bc; continue; }
                pc = 0x003cb094; continue;
            }
            0x003cb094 => {
                c.cycles += 10;
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2c));
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.g[RA][0] = sx(0x003cb0a4);
                c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
                call_ptr(c, 0x003c51e8, 0x003cb0a4)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2c));
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = sx((c.g[V1][0] as u32) << 2);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x34), c.g[V1][0]);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                pc = 0x003cb0bc; continue;
            }
            0x003cb0bc => {
                c.cycles += 3;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32));
                let t = (c.g[V1][0] as i64) <= 0; 
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                if t { pc = 0x003cb0f0; continue; }
                pc = 0x003cb0c8; continue;
            }
            0x003cb0c8 => {
                c.cycles += 10;
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                if c.g[V0][0] == 0 { c.g[S0][0] = c.g[V1][0]; }
                c.g[A2][0] = sx((c.g[S0][0] as u32) << 2);
                c.g[RA][0] = sx(0x003cb0e4);
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                call_ptr(c, 0x003c51e8, 0x003cb0e4)?;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[S0][0] as u32)));
                c.sw((c.g[S1][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                pc = 0x003cb0f0; continue;
            }
            0x003cb0f0 => {
                c.cycles += 1;
                c.g[V0][0] = c.g[S2][0];
                pc = 0x003cb0f4; continue;
            }
            0x003cb0f4 => {
                c.cycles += 7;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.pc = t; return Ok(());
            }
            _ => { c.jump_out(pc, 0x003cafd0, 0x003cb110); return Ok(()); }
        }
    }
    });
}

/// `SND_DspStretch_SetRatio` (0x003cb110): the normal entry, converted from the generated body
/// (labelled-block body: every register, cycle and call kept).
pub fn stretch_set_ratio(c: &mut Cpu) {
    guest(c, |c| {
'b_3cb198: {
'b_3cb194: {
'b_3cb168: {
c.cycles += 18;
c.mtc1(5, 0);
c.cvt_s_w(0, 0);
c.g[AT][0] = 0x39800000;
c.mtc1(1, 1);
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
c.mul_s(0, 0, 1);
c.g[S0][0] = c.g[A0][0];
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.mov_s(12, 0);
c.g[RA][0] = sx(0x003cb140);
c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x24));
call_ptr(c, 0x00413af8, 0x003cb140)?;
c.g[A1][0] = 0xff80;
c.g[A1][0] = c.g[A1][0] << 46;
c.g[RA][0] = sx(0x003cb150);
c.g[A0][0] = c.g[V0][0];
call_ptr(c, 0x00412500, 0x003cb150)?;
let t = (c.g[V0][0] as i64) >= 0;

if t {
break 'b_3cb168;
} else {
c.cycles += 4;
c.g[AT][0] = 0x3f000000;
c.mtc1(1, 0);
let t = true;
c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x24));
if t {
break 'b_3cb194;
} else {
break 'b_3cb168;
}
}
}
c.cycles += 8;
c.g[RA][0] = sx(0x003cb170);
c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x24));
call_ptr(c, 0x00413af8, 0x003cb170)?;
c.g[A1][0] = 0x8000;
c.g[A1][0] = c.g[A1][0] << 47;
c.g[RA][0] = sx(0x003cb180);
c.g[A0][0] = c.g[V0][0];
call_ptr(c, 0x00412500, 0x003cb180)?;
if (c.g[V0][0] as i64) <= 0 {
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
break 'b_3cb198;
} else {
c.cycles += 3;
c.g[AT][0] = 0x40000000;
c.mtc1(1, 0);
c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x24));
break 'b_3cb194;
}
}
c.cycles += 1;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
break 'b_3cb198;
}
c.cycles += 3;
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
c.pc = t; return Ok(());
    });
}

/// `SND_DspStretch_Process` (0x003cb1a8): the normal entry, converted from the generated body
/// (labelled-block body: every register, cycle and call kept).
pub fn stretch_process(c: &mut Cpu) {
    guest(c, |c| {
'b_3cb264: {
'b_3cb260: {
'b_3cb1f8: {
c.cycles += 15;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S1][0] = c.g[A0][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[S2][0] = c.g[A1][0];
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[S4][0] = c.g[A2][0];
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[RA][0] = sx(0x003cb1d8);
c.g[S3][0] = c.g[A3][0];
Callee { addr: 0x003caea0, f: ee::s14::SND_DspStretch_CalcInput }.call(c, 0x003cb1d8)?;
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
let t = (c.g[A0][0] as i64) < 0;
c.g[S0][0] = c.g[V0][0];
if t {
break 'b_3cb1f8;
} else {
c.cycles += 5;
c.g[RA][0] = sx(0x003cb1ec);
call_ptr(c, 0x003c6c98, 0x003cb1ec)?;
c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = 0u64;
if t {
break 'b_3cb260;
} else {
break 'b_3cb1f8;
}
}
}
'b_3cb224: {
c.cycles += 2;
let t = (c.g[S0][0] as i64) <= 0;
c.g[A1][0] = c.g[S0][0];
if t {
break 'b_3cb224;
} else {
c.cycles += 9;
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
c.g[T0][0] = c.lbu((c.g[S1][0] as u32).wrapping_add(0x1a));
c.g[A2][0] = c.g[S3][0];
c.g[V0][0] = c.lw((c.g[A0][0] as u32));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x003cb218);
c.g[A3][0] = c.g[S4][0];
call_ptr(c, t, 0x003cb218)?;
c.g[A0][0] = c.g[V0][0];
let t = (c.g[A0][0] as i64) <= 0;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
if t {
break 'b_3cb264;
} else {
break 'b_3cb224;
}
}
}
'b_3cb25c: {
c.cycles += 9;
c.g[A0][0] = c.g[S1][0];
c.g[A2][0] = c.g[S4][0];
c.g[A1][0] = c.g[S2][0];
c.g[RA][0] = sx(0x003cb238);
c.g[A3][0] = c.g[S3][0];
Callee { addr: 0x003cafd0, f: ee::s14::SND_DspStretch_Run }.call(c, 0x003cb238)?;
c.g[A0][0] = c.g[V0][0];
c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[S2][0] as i64)) as u64;
let t = c.g[V0][0] == 0u64;
c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
if t {
break 'b_3cb25c;
} else {
c.cycles += 5;
c.g[A1][0] = 0u64;
c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
c.g[RA][0] = sx(0x003cb25c);
c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, 0x00416210, 0x003cb25c)?;
break 'b_3cb25c;
}
}
c.cycles += 1;
c.g[V0][0] = c.g[S2][0];
break 'b_3cb260;
}
c.cycles += 1;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
break 'b_3cb264;
}
c.cycles += 7;
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
c.pc = t; return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA};

    /// Both signs of the splice offset, `k >= n` (loops skipped), and `n = 0..5`.
    #[test]
    fn stretch_crossfade_matches() {
        check(
            ee::s14::f_003caa28__generated,
            stretch_crossfade,
            300,
            &|c, seed| {
                let mut r = Rng(0x0c1158_01 ^ seed as u64);
                scramble(c, &mut r);
                let n = (seed % 6) as i32;
                let k = ((seed / 6) % 9) as i32 - 4;
                c.g[T0][0] = sx(k as u32);
                c.g[A3][0] = sx(n as u32);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = sx(DATA + 0x3000);
                for i in 0..24u32 {
                    c.bus.write32(DATA + 0x1000 + 4 * i, r.f32(4.0));
                    c.bus.write32(DATA + 0x2000 + 4 * i, r.f32(4.0));
                    c.bus.write32(DATA + 0x3000 + 4 * i, r.f32(4.0));
                }
            },
        );
    }

    /// Stretcher object at `DATA + 0x1000` (`+0x1c` splice table pointer, `+0x24`
    /// time ratio, `+0x28` error, `+0x2c` window size, `+0x30` frames left,
    /// `+0x34` input samples, `+0x38` output samples, `+0x3c` output start,
    /// `+0x40` input, `+0x838` output).
    const STRETCH_ST: u32 = DATA + 0x1000;
    /// Splice offset table (bytes) the `+0x1c` pointer walks.
    const STRETCH_TABLE: u32 = DATA + 0x8000;
    /// Source audio the run/splice cursor reads (cursor starts in the middle).
    const STRETCH_SRC: u32 = DATA + 0x20400;
    /// Destination audio buffer.
    const STRETCH_OUT: u32 = DATA + 0x30000;

    /// Build a stretcher with window size 1..8 and every field random in its
    /// useful range; returns the window size.
    fn stretch_world(c: &mut Cpu, r: &mut Rng) -> u32 {
        use crate::difftest::fill;
        let w = 1 + r.next_u32() % 8;
        fill(c, r, STRETCH_ST + 0x40, 0x400);
        fill(c, r, STRETCH_SRC - 0x400, 0x400);
        fill(c, r, STRETCH_OUT, 0x80);
        for i in 0..64u32 {
            let v = r.next_u32() & 0x0f0f_0f0f;
            c.bus.write32(STRETCH_TABLE + 4 * i, v);
        }
        c.bus.write32(STRETCH_ST + 0x1c, STRETCH_TABLE + r.next_u32() % 16);
        let ratio = 0.25 + (r.next_u32() % 1000) as f32 / 400.0;
        let err = ((r.next_u32() % 2000) as f32 - 1000.0) / 1000.0;
        c.bus.write32(STRETCH_ST + 0x24, ratio.to_bits());
        c.bus.write32(STRETCH_ST + 0x28, err.to_bits());
        c.bus.write32(STRETCH_ST + 0x2c, w);
        c.bus.write32(STRETCH_ST + 0x30, r.next_u32() % 48);
        c.bus.write32(STRETCH_ST + 0x34, r.next_u32() % (2 * w + 2));
        c.bus.write32(STRETCH_ST + 0x38, r.next_u32() % (3 * w));
        c.bus.write32(STRETCH_ST + 0x3c, r.next_u32() % 4);
        w
    }

    /// `SND_DspStretch_ChooseSplice`: both ratio sides (`< 1`, `>= 1`), both
    /// error comparisons, random table bytes and window sizes.
    #[test]
    fn stretch_choose_splice_matches() {
        check(
            ee::s14::f_003cab28__generated,
            stretch_choose_splice,
            400,
            &|c, seed| {
                let mut r = Rng(0x0c1158_02 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let ratio = 0.2 + (r.next_u32() % 1000) as f32 / 400.0;
                let err = ((r.next_u32() % 2000) as f32 - 1000.0) / 1000.0;
                c.f[12] = ratio.to_bits();
                c.g[A0][0] = sx(1 + r.next_u32() % 16);
                c.g[A1][0] = sx(DATA + 0x1000);
                c.g[A2][0] = sx(DATA + 0x1010);
                c.bus.write32(DATA + 0x1000, STRETCH_TABLE + r.next_u32() % 16);
                c.bus.write32(DATA + 0x1010, err.to_bits());
                for i in 0..64u32 {
                    let v = r.next_u32() & 0x0f0f_0f0f;
                    c.bus.write32(STRETCH_TABLE + 4 * i, v);
                }
            },
        );
    }

    /// `SND_DspStretch_Splice`: shift 0, `> 0` and `< 0`, with and without
    /// pending input samples (`+0x34`), calling `SND_MemCopy` and the crossfade.
    #[test]
    fn stretch_splice_matches() {
        check(
            ee::s14::f_003cac88__generated,
            stretch_splice,
            400,
            &|c, seed| {
                let mut r = crate::difftest::fakes::machine(c, seed, 0x1158_03, 0);
                let w = stretch_world(c, &mut r);
                if seed % 3 == 0 {
                    c.bus.write32(STRETCH_ST + 0x34, 0);
                }
                let shift = (seed % (2 * w + 1)) as i32 - w as i32;
                c.g[A0][0] = sx(STRETCH_ST);
                c.g[A1][0] = sx(DATA + 0x4000);
                c.g[A2][0] = sx(shift as u32);
                c.bus.write32(DATA + 0x4000, STRETCH_SRC);
            },
        );
    }

    /// `SND_DspStretch_Drain`: copy `min(outputsamples, n)` output samples out.
    #[test]
    fn stretch_drain_matches() {
        check(
            ee::s14::f_003cadf0__generated,
            stretch_drain,
            300,
            &|c, seed| {
                let mut r = crate::difftest::fakes::machine(c, seed, 0x1158_04, 0);
                stretch_world(c, &mut r);
                c.g[A0][0] = sx(STRETCH_ST);
                c.g[A1][0] = sx(DATA + 0x4000);
                c.g[A2][0] = sx(DATA + 0x4010);
                c.bus.write32(DATA + 0x4000, r.next_u32() % 32);
                c.bus.write32(DATA + 0x4010, STRETCH_OUT);
            },
        );
    }

    /// `SND_DspStretch_CalcInput`: how many input samples a block of `n`
    /// output samples needs (loop over `ChooseSplice`, every exit).
    #[test]
    fn stretch_calc_input_matches() {
        check(
            ee::s14::f_003caea0__generated,
            stretch_calc_input,
            400,
            &|c, seed| {
                let mut r = crate::difftest::fakes::machine(c, seed, 0x1158_05, 0);
                stretch_world(c, &mut r);
                c.g[A0][0] = sx(STRETCH_ST);
                c.g[A1][0] = sx(r.next_u32() % 48);
            },
        );
    }

    /// `SND_DspStretch_Run` (a resumable dispatcher; the normal entry): the
    /// splice/drain loop over several windows, including `n = 0`.
    #[test]
    fn stretch_run_matches() {
        check(
            ee::s14::f_003cafd0__generated,
            stretch_run,
            400,
            &|c, seed| {
                let mut r = crate::difftest::fakes::machine(c, seed, 0x1158_06, 0);
                stretch_world(c, &mut r);
                c.pc = 0x003c_afd0;
                c.g[A0][0] = sx(STRETCH_ST);
                c.g[A1][0] = sx(r.next_u32() % 41);
                c.g[A2][0] = sx(STRETCH_SRC);
                c.g[A3][0] = sx(STRETCH_OUT);
            },
        );
    }

    /// `SND_DspStretch_SetRatio`: the new ratio `a1 / 4096`, clamped to
    /// `[0.5, 2.0]` (both clamps and the pass-through).
    #[test]
    fn stretch_set_ratio_matches() {
        check(
            ee::s14::f_003cb110__generated,
            stretch_set_ratio,
            300,
            &|c, seed| {
                let mut r = crate::difftest::fakes::machine(c, seed, 0x1158_07, 0);
                c.g[A0][0] = sx(STRETCH_ST);
                let v: i32 = match seed % 8 {
                    0 => 2048,
                    1 => 8192,
                    2 => 0,
                    3 => -5,
                    _ => (r.next_u32() % 24000) as i32 - 2000,
                };
                c.g[A1][0] = sx(v as u32);
            },
        );
    }

    /// `SND_DspStretch_Process`: pull input through the next stage's function
    /// pointer (a fake, or a real leaf returning 0), run the stretch and clear
    /// the tail of the output; packet slot `< 0` or a real buffered count.
    #[test]
    fn stretch_process_matches() {
        crate::difftest::fakes::covered(
            ee::s14::f_003cb1a8__generated,
            stretch_process,
            300,
            &|c, seed| {
                let mut r = crate::difftest::fakes::machine(c, seed, 0x1158_08, 1);
                stretch_world(c, &mut r);
                c.g[A0][0] = sx(STRETCH_ST);
                c.g[A1][0] = sx(r.next_u32() % 41);
                c.g[A2][0] = sx(STRETCH_SRC);
                c.g[A3][0] = sx(STRETCH_OUT);
                // Next stage object: its first word is the function pointer.
                let obj = DATA + 0x6000;
                c.bus.write32(STRETCH_ST + 8, obj);
                c.bus.write32(obj, if seed % 4 == 3 { 0x0014_06c0 } else { 0x4000 });
                c.bus.write32(STRETCH_ST + 0x1a, r.next_u32() & 0xff);
                let slot: i32 = if seed % 2 == 0 { -1 } else { (r.next_u32() % 8) as i32 };
                c.bus.write32(STRETCH_ST + 0x20, slot as u32);
                if slot >= 0 {
                    let stream = DATA + 0x7000 + 0x100 * slot as u32;
                    let table = 0x0051_57b8 + 0x304;
                    c.bus.write32(table + 4 * slot as u32, stream);
                    c.bus.write32(stream + 0x48, r.next_u32() % 40);
                    c.bus.write32(stream + 0x4c, r.next_u32() % 40);
                }
            },
        );
    }
}
