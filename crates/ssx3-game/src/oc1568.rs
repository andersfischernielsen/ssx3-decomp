//! Staging module oc1568 (wave wave278): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1568`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0035_b200, multi_spline_modifier_v22, "MultiSplineModifier_v22"),
    (0x0037_37d0, bezier_patch_cache_free, "BezierPatchCache_Free"),
    (0x0037_cc98, draw_transition, "DrawTransition"),
    (0x0038_7ec0, ps2_graphics_man_emit_shadow_strips, "PS2GraphicsMan_EmitShadowStrips"),
    (0x0038_85e0, ps2_graphics_man_emit_board_wake_strips, "PS2GraphicsMan_EmitBoardWakeStrips"),
];

/// `void cMultiSplineModifier::updateIntersectInfo(tIntersectInfo&)` (0x35b200, `MultiSplineModifier_v22`): finds the
/// spline whose index list entry (`this+0x44`, count at `+4`) equals the key at `info+0x50`; for that spline
/// record (`this+0x3c`, stride 0x60) it builds a rotation/offset from the record's vectors (scalar FPU path
/// when its first component is small, else a VU0 normalise), scaled by `this+0x14`, and adds it
/// to the intersect info's two position vectors at `info+0x20` / `info+0x30`. A leaf; VU0 ops kept as generated.
pub fn multi_spline_modifier_v22(c: &mut Cpu) {
    guest(c, |c| {
'b_35b410: {
c.cycles += 5;
c.g[T1][0] = c.g[A0][0];
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x4));
let t = (c.g[V0][0] as i64) <= 0;
c.g[A0][0] = 0u64;
if t {
break 'b_35b410;
} else {
'b_35b250: {
c.cycles += 6;
c.g[A2][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x44));
c.g[T0][0] = c.g[V0][0];
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x50));
c.g[V0][0] = c.lw((c.g[A2][0] as u32));
let t = c.g[V0][0] == c.g[V1][0];
c.g[A3][0] = c.g[V1][0];
if t {
break 'b_35b250;
} else {
c.cycles += 3;
c.g[V1][0] = c.g[A2][0];
c.g[A2][0] = c.g[T0][0];
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
'l_35b238: loop {
c.cycles += 3;
c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A2][0] as i64)) as u64;
let t = c.g[V0][0] == 0u64;
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
if t {
break 'b_35b410;
} else {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[V1][0] as u32));
if c.g[V0][0] != c.g[A3][0] {
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
if event_due(c) { deliver_event_here(c, 0x0035b238); } continue 'l_35b238;
} else {
break 'b_35b250;
}
}
}
}
}
c.cycles += 3;
c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[T0][0] as i64)) as u64;
let t = c.g[V0][0] == 0u64;
c.g[A3][0] = sx((c.g[T1][0] as u32).wrapping_add(0x4));
if t {
break 'b_35b410;
} else {
'b_35b2ec: {
c.cycles += 12;
c.g[V0][0] = 0x60;
c.g[V1][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x3c));
c.mult(0, 2, 4, 2);
c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x2a10));
c.mtc1(0, 5);
c.mov_s(8, 5);
c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0x10));
c.abs_s(0, 2);
c.c_lt_s(1, 0);
let t = !c.fcc();

if t {
break 'b_35b2ec;
} else {
c.cycles += 24;
c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x24));
c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0x20));
c.div_s(7, 1, 2);
c.lwc1(6, (c.g[A2][0] as u32).wrapping_add(0x50));
c.add_s(1, 0, 1);
c.lwc1(3, (c.g[A2][0] as u32).wrapping_add(0x54));
c.lwc1(4, (c.g[A2][0] as u32).wrapping_add(0x14));
c.neg_s(8, 6);
c.lwc1(2, (c.g[A2][0] as u32).wrapping_add(0x18));
c.add_s(3, 6, 3);
c.mul_s(4, 4, 0);
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 5);
c.mul_s(2, 2, 1);
c.sqrt_s(0, 3);
c.div_s(5, 5, 0);
c.lwc1(1, (c.g[A2][0] as u32).wrapping_add(0x28));
c.div_s(4, 4, 6);
c.div_s(2, 2, 3);
c.div_s(8, 8, 3);
c.sub_s(7, 7, 4);
c.sub_s(1, 1, 2);
c.mul_s(8, 8, 7);
c.mul_s(5, 5, 1);
break 'b_35b2ec;
}
}
c.cycles += 73;
c.lqc2(3, (c.g[A2][0] as u32).wrapping_add(0x10));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.vu0().rsqrt(0, 3, 4, 0);
c.vu0().waitq();
c.vu0().mul(0b1111, 5, 3, Src::Q);
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lwc1(0, (c.g[A3][0] as u32).wrapping_add(0x10));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.mfc1(4, 0);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
c.qmtc2(4, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lwc1(1, (c.g[T1][0] as u32).wrapping_add(0x14));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.mul_s(0, 5, 1);
c.g[A0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x44));
c.mul_s(1, 8, 1);
c.sw((c.g[SP][0] as u32).wrapping_add(0x10), 0u64);
c.sw((c.g[SP][0] as u32).wrapping_add(0x1c), 0u64);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x18));
c.sq(2, (c.g[SP][0] as u32));
c.g[V0][0] = c.lw((c.g[V1][0] as u32));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x10));
c.lqc2(4, (c.g[V0][0] as u32));
c.lqc2(5, (c.g[V0][0] as u32).wrapping_add(0x10));
c.lqc2(6, (c.g[V0][0] as u32).wrapping_add(0x20));
c.lqc2(7, (c.g[V0][0] as u32).wrapping_add(0x30));
c.vu0().mula(0b1111, 4, Src::Bc(8, X));
c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(3, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x10));
c.lqc2(3, (c.g[A1][0] as u32));
c.lqc2(4, (c.g[A2][0] as u32));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
c.vu0().opmula(4, 5);
c.vu0().opmsub(6, 5, 4);
c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
c.sqc2(6, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lqc2(3, (c.g[SP][0] as u32));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lqc2(3, (c.g[A1][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[A1][0] as u32).wrapping_add(0x20));
c.lqc2(3, (c.g[A1][0] as u32).wrapping_add(0x30));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[A1][0] as u32).wrapping_add(0x30));
break 'b_35b410;
}
}
}
c.cycles += 2;
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
c.pc = t; return Ok(());
    });
}

/// `void BezierPatchCache_Free(BezierPatchCache*)` (0x3737d0): releases the seven groups of cached
/// buffers (three pointers each at +0xc, +0x24, +0x48, +0x54, +0x60, +0x1b4, +0x1c0) through `EA_MemFree`
/// when non-null, clearing each field.
pub fn bezier_patch_cache_free(c: &mut Cpu) {
    guest(c, |c| {
'b_3737f8: {
c.cycles += 7;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sd((c.g[SP][0] as u32), c.g[RA][0]);
c.g[S0][0] = c.g[A0][0];
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
if c.g[A0][0] == 0u64 {
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
break 'b_3737f8;
} else {
c.cycles += 3;
call_ptr(c, 0x00317e98, 0x003737f4)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
break 'b_3737f8;
}
}
'b_37380c: {
c.cycles += 2;
if c.g[A0][0] == 0u64 {
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
break 'b_37380c;
} else {
c.cycles += 3;
call_ptr(c, 0x00317e98, 0x00373808)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
break 'b_37380c;
}
}
'b_373820: {
c.cycles += 2;
if c.g[A0][0] == 0u64 {
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
break 'b_373820;
} else {
c.cycles += 3;
call_ptr(c, 0x00317e98, 0x0037381c)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x24));
break 'b_373820;
}
}
'b_373838: {
c.cycles += 4;
c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
let t = c.g[A0][0] == 0u64;
c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
if t {
break 'b_373838;
} else {
c.cycles += 2;
call_ptr(c, 0x00317e98, 0x00373838)?;
break 'b_373838;
}
}
'b_373850: {
c.cycles += 3;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x28));
if c.g[A0][0] == 0u64 {
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2c));
break 'b_373850;
} else {
c.cycles += 3;
call_ptr(c, 0x00317e98, 0x0037384c)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x2c));
break 'b_373850;
}
}
'b_373864: {
c.cycles += 2;
if c.g[A0][0] == 0u64 {
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
break 'b_373864;
} else {
c.cycles += 3;
call_ptr(c, 0x00317e98, 0x00373860)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
break 'b_373864;
}
}
'b_37387c: {
c.cycles += 4;
c.sw((c.g[S0][0] as u32).wrapping_add(0x24), 0u64);
c.sw((c.g[S0][0] as u32).wrapping_add(0x28), 0u64);
let t = c.g[A0][0] == 0u64;
c.sw((c.g[S0][0] as u32).wrapping_add(0x2c), 0u64);
if t {
break 'b_37387c;
} else {
c.cycles += 2;
call_ptr(c, 0x00317e98, 0x0037387c)?;
break 'b_37387c;
}
}
'b_373894: {
c.cycles += 3;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
if c.g[A0][0] == 0u64 {
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
break 'b_373894;
} else {
c.cycles += 3;
call_ptr(c, 0x00317e98, 0x00373890)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
break 'b_373894;
}
}
'b_3738a8: {
c.cycles += 2;
if c.g[A0][0] == 0u64 {
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
break 'b_3738a8;
} else {
c.cycles += 3;
call_ptr(c, 0x00317e98, 0x003738a4)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
break 'b_3738a8;
}
}
'b_3738c0: {
c.cycles += 4;
c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
let t = c.g[A0][0] == 0u64;
c.sw((c.g[S0][0] as u32).wrapping_add(0x50), 0u64);
if t {
break 'b_3738c0;
} else {
c.cycles += 2;
call_ptr(c, 0x00317e98, 0x003738c0)?;
break 'b_3738c0;
}
}
'b_3738d8: {
c.cycles += 3;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
if c.g[A0][0] == 0u64 {
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
break 'b_3738d8;
} else {
c.cycles += 3;
call_ptr(c, 0x00317e98, 0x003738d4)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
break 'b_3738d8;
}
}
'b_3738ec: {
c.cycles += 2;
if c.g[A0][0] == 0u64 {
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x60));
break 'b_3738ec;
} else {
c.cycles += 3;
call_ptr(c, 0x00317e98, 0x003738e8)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x60));
break 'b_3738ec;
}
}
'b_373904: {
c.cycles += 4;
c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
c.sw((c.g[S0][0] as u32).wrapping_add(0x58), 0u64);
let t = c.g[A0][0] == 0u64;
c.sw((c.g[S0][0] as u32).wrapping_add(0x5c), 0u64);
if t {
break 'b_373904;
} else {
c.cycles += 2;
call_ptr(c, 0x00317e98, 0x00373904)?;
break 'b_373904;
}
}
'b_37391c: {
c.cycles += 3;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64));
if c.g[A0][0] == 0u64 {
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
break 'b_37391c;
} else {
c.cycles += 3;
call_ptr(c, 0x00317e98, 0x00373918)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x68));
break 'b_37391c;
}
}
'b_373930: {
c.cycles += 2;
if c.g[A0][0] == 0u64 {
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1b4));
break 'b_373930;
} else {
c.cycles += 3;
call_ptr(c, 0x00317e98, 0x0037392c)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1b4));
break 'b_373930;
}
}
'b_373948: {
c.cycles += 4;
c.sw((c.g[S0][0] as u32).wrapping_add(0x60), 0u64);
c.sw((c.g[S0][0] as u32).wrapping_add(0x64), 0u64);
let t = c.g[A0][0] == 0u64;
c.sw((c.g[S0][0] as u32).wrapping_add(0x68), 0u64);
if t {
break 'b_373948;
} else {
c.cycles += 2;
call_ptr(c, 0x00317e98, 0x00373948)?;
break 'b_373948;
}
}
'b_373960: {
c.cycles += 3;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1b8));
if c.g[A0][0] == 0u64 {
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1bc));
break 'b_373960;
} else {
c.cycles += 3;
call_ptr(c, 0x00317e98, 0x0037395c)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1bc));
break 'b_373960;
}
}
'b_373974: {
c.cycles += 2;
if c.g[A0][0] == 0u64 {
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
break 'b_373974;
} else {
c.cycles += 3;
call_ptr(c, 0x00317e98, 0x00373970)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
break 'b_373974;
}
}
'b_37398c: {
c.cycles += 4;
c.sw((c.g[S0][0] as u32).wrapping_add(0x1b4), 0u64);
c.sw((c.g[S0][0] as u32).wrapping_add(0x1b8), 0u64);
let t = c.g[A0][0] == 0u64;
c.sw((c.g[S0][0] as u32).wrapping_add(0x1bc), 0u64);
if t {
break 'b_37398c;
} else {
c.cycles += 2;
call_ptr(c, 0x00317e98, 0x0037398c)?;
break 'b_37398c;
}
}
'b_3739a4: {
c.cycles += 3;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c4));
if c.g[A0][0] == 0u64 {
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c8));
break 'b_3739a4;
} else {
c.cycles += 3;
call_ptr(c, 0x00317e98, 0x003739a0)?;
c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c8));
break 'b_3739a4;
}
}
'b_3739b8: {
c.cycles += 2;
if c.g[A0][0] == 0u64 {
c.sw((c.g[S0][0] as u32).wrapping_add(0x1c8), 0u64);
break 'b_3739b8;
} else {
c.cycles += 3;
call_ptr(c, 0x00317e98, 0x003739b4)?;
c.sw((c.g[S0][0] as u32).wrapping_add(0x1c8), 0u64);
break 'b_3739b8;
}
}
c.cycles += 6;
c.sw((c.g[S0][0] as u32).wrapping_add(0x1c0), 0u64);
c.sw((c.g[S0][0] as u32).wrapping_add(0x1c4), 0u64);
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[RA][0] = c.ld((c.g[SP][0] as u32));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
c.pc = t; return Ok(());
    });
}

/// `void PS2GraphicsMan::DrawTransition(PS2GraphicsMan*, tex)` (0x37cc98): builds the GIF packet that copies
/// the screen into the transition texture inside an SPR stream block (`SprStream_BeginBlock` /
/// `SprStream_EndBlockAligned`), and records the packet in the frame list while fewer than 2600 are queued.
pub fn draw_transition(c: &mut Cpu) {
    guest(c, |c| {
'b_37d040: {
c.cycles += 200;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
c.g[T4][0] = 0xfffffffffffffc3f;
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
c.g[T5][0] = 0xffffffffc0000000;
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
c.g[T2][0] = 0xffffffffffffffff;
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[A2][0] = 0xffffffffffffffff;
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[FP][0] = c.g[A0][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
c.ldl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
c.ldr(3, (c.g[V0][0] as u32));
c.ldl(4, (c.g[V0][0] as u32).wrapping_add(0xf));
c.ldr(4, (c.g[V0][0] as u32).wrapping_add(0x8));
c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x1b));
c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x14));
c.sdl(4, (c.g[V0][0] as u32).wrapping_add(0x23));
c.sdr(4, (c.g[V0][0] as u32).wrapping_add(0x1c));
c.sw((c.g[V0][0] as u32).wrapping_add(0x24), c.g[A3][0]);
c.g[A3][0] = 0u64;
c.g[T1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x14));
c.g[T0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x18f4));
c.sw((c.g[FP][0] as u32).wrapping_add(0xe84), c.g[V1][0]);
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
c.g[V1][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x14));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[V1][0] = c.g[V1][0] & c.g[T4][0];
c.g[T3][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
c.sw((c.g[T1][0] as u32).wrapping_add(0x14), c.g[V1][0]);
c.g[A1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
c.g[V0][0] = c.g[V0][0] & c.g[T5][0];
c.sw((c.g[A1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x5a00));
c.sw((c.g[SP][0] as u32), c.g[V0][0]);
c.g[V0][0] = c.lw((c.g[T3][0] as u32).wrapping_add(0x28));
c.g[S3][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x5a88));
c.g[T2][0] = ((c.g[T2][0] as i64) < (c.g[V0][0] as i64)) as u64;
c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1f));
c.g[A1][0] = c.lw((c.g[SP][0] as u32));
if c.g[T2][0] != 0 { c.g[S1][0] = c.g[V0][0]; }
c.g[S3][0] = sx((c.g[S3][0] as u32) << 5);
c.g[S1][0] = sx(((c.g[S1][0] as u32) as i32 >> 5) as u32);
c.g[S2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x5a5c));
c.g[RA][0] = sx(0x0037cd7c);
c.g[S0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x5a80));
Callee { addr: 0x0038f460, f: ee::s12::SprStream_BeginBlock }.call(c, 0x0037cd7c)?;
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 0);
c.g[V1][0] = 0x1000000;
c.g[S0][0] = c.g[S0][0] | c.g[V1][0];
c.g[A0][0] = 0x50000000;
c.g[A0][0] = c.g[A0][0] | 0x12;
c.g[A0][0] = c.g[A0][0] << 32;
c.mfc1(3, 0);
c.g[S0][0] = c.g[S0][0] << 32;
c.sd((c.g[V0][0] as u32).wrapping_add(0x18), c.g[A0][0]);
c.g[S0][0] = c.g[S0][0] >> 32;
c.g[A3][0] = c.g[V1][0];
c.g[A0][0] = 0x8000;
c.g[A0][0] = c.g[A0][0] << 17;
c.por(3, 0, 0);
c.g[V1][0] = 0x10000000;
c.g[V1][0] = c.g[V1][0] | 0x13;
c.sd((c.g[V0][0] as u32).wrapping_add(0x10), 0u64);
c.sq(3, (c.g[V0][0] as u32));
c.g[S0][0] = c.g[S0][0] | c.g[A0][0];
c.g[S2][0] = c.g[S2][0] << 14;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
c.g[S2][0] = c.g[S3][0] | c.g[S2][0];
c.g[A3][0] = c.g[A3][0] << 32;
c.g[V1][0] = 0x40000;
c.g[A1][0] = 0xaa00;
c.g[A1][0] = c.g[A1][0] << 18;
c.g[A2][0] = 0x8080;
c.g[A2][0] = c.g[A2][0] << 16;
c.g[A2][0] = c.g[A2][0] | 0x8080;
c.g[A0][0] = 0xe;
c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
c.g[S1][0] = c.g[S1][0] | c.g[V1][0];
c.g[A3][0] = c.g[A3][0] | c.g[A2][0];
c.g[S2][0] = c.g[S2][0] | c.g[A1][0];
c.g[V1][0] = 0x10000000;
c.g[V1][0] = c.g[V1][0] << 32;
c.g[V1][0] = c.g[V1][0] | 0x8011;
c.g[A1][0] = 0x4c;
c.g[A2][0] = 0x4e;
c.g[T0][0] = 0xff0000;
c.g[T0][0] = c.g[T0][0] << 16;
c.g[T0][0] = c.g[T0][0] | 0xff;
c.g[T0][0] = c.g[T0][0] << 16;
c.g[T1][0] = 0x30000;
c.g[T2][0] = 0x47;
c.g[T3][0] = 0x18;
c.g[T4][0] = 0x2a;
c.g[T5][0] = 0x42;
c.g[T6][0] = 0x80;
c.g[T6][0] = c.g[T6][0] << 32;
c.g[T6][0] = c.g[T6][0] | 0x80;
c.g[T7][0] = 0x3b;
c.g[A0][0] = 0x40;
c.g[S3][0] = 0x14;
c.g[S4][0] = 0x6;
c.g[S5][0] = 0x116;
c.g[S6][0] = 0x1;
c.sd((c.g[V0][0] as u32).wrapping_add(0x10), c.g[S1][0]);
c.g[T8][0] = 0x5;
c.sd((c.g[V0][0] as u32).wrapping_add(0x18), c.g[A1][0]);
c.g[S7][0] = 0x3;
c.sd((c.g[V0][0] as u32), c.g[V1][0]);
c.g[T9][0] = 0x80000;
c.g[T9][0] = c.g[T9][0] | 0x8;
c.sd((c.g[V0][0] as u32).wrapping_add(0x20), c.g[S0][0]);
c.g[S1][0] = 0xffff;
c.g[S1][0] = c.g[S1][0] << 32;
c.sd((c.g[V0][0] as u32).wrapping_add(0x28), c.g[A2][0]);
c.g[A1][0] = 0xffff;
c.g[A1][0] = c.g[A1][0] << 16;
c.g[A1][0] = c.g[A1][0] | 0x1000;
c.g[A1][0] = c.g[A1][0] << 16;
c.g[A1][0] = c.g[A1][0] | 0x1000;
c.sd((c.g[V0][0] as u32).wrapping_add(0x30), c.g[T0][0]);
c.g[S0][0] = 0x3f;
c.sd((c.g[V0][0] as u32).wrapping_add(0x40), c.g[T1][0]);
c.g[A2][0] = 0x4;
c.sd((c.g[V0][0] as u32).wrapping_add(0xb0), c.g[A3][0]);
c.sd((c.g[V0][0] as u32).wrapping_add(0x48), c.g[T2][0]);
c.sd((c.g[V0][0] as u32).wrapping_add(0x58), c.g[T3][0]);
c.sd((c.g[V0][0] as u32).wrapping_add(0x60), c.g[T4][0]);
c.sd((c.g[V0][0] as u32).wrapping_add(0x68), c.g[T5][0]);
c.sd((c.g[V0][0] as u32).wrapping_add(0x70), c.g[T6][0]);
c.sd((c.g[V0][0] as u32).wrapping_add(0x78), c.g[T7][0]);
c.sd((c.g[V0][0] as u32).wrapping_add(0x88), c.g[S3][0]);
c.sd((c.g[V0][0] as u32).wrapping_add(0x90), c.g[S2][0]);
c.sd((c.g[V0][0] as u32).wrapping_add(0x98), c.g[S4][0]);
c.sd((c.g[V0][0] as u32).wrapping_add(0xa0), c.g[S5][0]);
c.sd((c.g[V0][0] as u32).wrapping_add(0xb8), c.g[S6][0]);
c.sd((c.g[V0][0] as u32).wrapping_add(0x38), c.g[A0][0]);
c.sd((c.g[V0][0] as u32).wrapping_add(0x50), 0u64);
c.sd((c.g[V0][0] as u32).wrapping_add(0x80), 0u64);
c.sd((c.g[V0][0] as u32).wrapping_add(0xa8), 0u64);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xc0));
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x5a58));
c.g[A0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x5a54));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
c.sd((c.g[V0][0] as u32), c.g[T9][0]);
c.g[A0][0] = sx((c.g[A0][0] as u32) << 4);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x8));
c.g[V1][0] = c.g[V1][0] << 16;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x8));
c.g[A0][0] = c.g[A0][0] | c.g[V1][0];
c.sd((c.g[V0][0] as u32).wrapping_add(0x10), c.g[S1][0]);
c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[S7][0]);
c.sd((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T8][0]);
c.sd((c.g[V0][0] as u32).wrapping_add(0x20), c.g[A0][0]);
c.sd((c.g[V0][0] as u32).wrapping_add(0x30), c.g[A1][0]);
c.sd((c.g[V0][0] as u32).wrapping_add(0x28), c.g[S7][0]);
c.sd((c.g[V0][0] as u32).wrapping_add(0x38), c.g[T8][0]);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x40));
c.sd((c.g[V0][0] as u32).wrapping_add(0x8), c.g[S0][0]);
c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
c.sd((c.g[V0][0] as u32), 0u64);
c.g[V1][0] = c.lhu((c.g[FP][0] as u32).wrapping_add(0x5900));
c.g[T0][0] = c.lhu((c.g[FP][0] as u32).wrapping_add(0x5904));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
c.g[A3][0] = c.lhu((c.g[FP][0] as u32).wrapping_add(0x5902));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[T0][0] as u32)));
c.g[T1][0] = c.lhu((c.g[FP][0] as u32).wrapping_add(0x5906));
c.g[A0][0] = c.lhu((c.g[FP][0] as u32).wrapping_add(0x5900));
c.g[V1][0] = c.g[V1][0] << 16;
c.g[T0][0] = c.lhu((c.g[FP][0] as u32).wrapping_add(0x5902));
c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[T1][0] as u32)));
c.g[A0][0] = c.g[A0][0] | c.g[V1][0];
c.g[T0][0] = c.g[T0][0] << 32;
c.g[A3][0] = c.g[A3][0] << 48;
c.g[A0][0] = c.g[A0][0] | c.g[T0][0];
c.g[A0][0] = c.g[A0][0] | c.g[A3][0];
c.g[T0][0] = 0x40;
c.sd((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T0][0]);
c.sd((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A0][0]);
c.g[RA][0] = sx(0x0037cf9c);
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
Callee { addr: 0x0038f668, f: ee::s12::SprStream_EndBlockAligned }.call(c, 0x0037cf9c)?;
c.g[T1][0] = c.g[V0][0];
c.g[A3][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x18f0));
c.sw((c.g[FP][0] as u32).wrapping_add(0x5a00), c.g[T1][0]);
c.g[V0][0] = c.lw((c.g[A3][0] as u32));
c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[T0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
if t {
break 'b_37d040;
} else {
c.cycles += 34;
c.g[A1][0] = 0x60000;
c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x8));
c.g[A1][0] = c.g[A1][0] | 0x8000;
c.g[V1][0] = 0xffffffffffffffe0;
c.g[A1][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[A0][0] = c.g[A0][0] & c.g[V1][0];
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc4));
c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x80));
c.g[V1][0] = 0x30000000;
c.g[V0][0] = c.g[V0][0] & 0x1f;
c.g[A2][0] = c.g[A2][0] | c.g[V1][0];
c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
c.sw((c.g[T0][0] as u32).wrapping_add(0x8), c.g[A0][0]);
c.g[V1][0] = c.lw((c.g[A3][0] as u32));
c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1cc0));
c.g[V0][0] = sx((c.g[V1][0] as u32) << 7);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.sw((c.g[A3][0] as u32), c.g[V1][0]);
c.ldl(3, (c.g[T0][0] as u32).wrapping_add(0x7));
c.ldr(3, (c.g[T0][0] as u32));
c.ldl(5, (c.g[T0][0] as u32).wrapping_add(0xf));
c.ldr(5, (c.g[T0][0] as u32).wrapping_add(0x8));
c.g[A2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
c.sdr(3, (c.g[V0][0] as u32));
c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0xf));
c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x8));
c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[A2][0]);
c.g[T0][0] = c.lw((c.g[SP][0] as u32));
c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[T0][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), 0u64);
break 'b_37d040;
}
}
c.cycles += 20;
c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x5a00));
c.g[A0][0] = 0x1;
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
c.sw((c.g[FP][0] as u32).wrapping_add(0xf44), c.g[A0][0]);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x14));
c.sw((c.g[FP][0] as u32).wrapping_add(0x5a00), c.g[V0][0]);
c.sw((c.g[FP][0] as u32).wrapping_add(0xe84), c.g[V1][0]);
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x20));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
c.pc = t; return Ok(());
    });
}

/// `PS2GraphicsMan_EmitShadowStrips` (0x387ec0): emits the sun-shadow triangle strips in batches of up to 36
/// vertices (per-batch GIF header, vertex and index copies with 16-bit field packing) into the packet
/// buffer at `mgr+0x5a00`, which it advances. A leaf; the batching divides with `div` as generated.
pub fn ps2_graphics_man_emit_shadow_strips(c: &mut Cpu) {
    guest(c, |c| {
'b_387f10: {
'b_387f08: {
c.cycles += 15;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xc0));
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x12b0));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[T4][0] = c.g[A1][0];
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[S0][0] = c.g[A2][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
let t = c.g[V0][0] == 0u64;
c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A0][0]);
if t {
break 'b_387f08;
} else {
c.cycles += 3;
c.g[V0][0] = 0x0;
let t = true;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3a00));
if t {
break 'b_387f10;
} else {
break 'b_387f08;
}
}
}
c.cycles += 2;
c.g[V0][0] = 0x0;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3a08));
break 'b_387f10;
}
'b_387f30: {
'b_387f28: {
c.cycles += 3;
c.g[T3][0] = sx((c.g[V0][0] as u32) >> 3);
let t = c.g[T2][0] == 0u64;
c.sw((c.g[GP][0] as u32).wrapping_add(0x12b0), 0u64);
if t {
break 'b_387f28;
} else {
c.cycles += 3;
c.g[A2][0] = 0x5c;
let t = true;
c.g[S7][0] = 0x412;
if t {
break 'b_387f30;
} else {
break 'b_387f28;
}
}
}
c.cycles += 2;
c.g[A2][0] = 0x4;
c.g[S7][0] = 0x41f;
break 'b_387f30;
}
'b_388374: {
c.cycles += 5;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5a00));
c.g[V0][0] = 0x30000000;
let t = c.g[A3][0] == 0u64;
c.g[T2][0] = c.g[V1][0] | c.g[V0][0];
if t {
break 'b_388374;
} else {
c.cycles += 29;
c.mfc1(3, 13);
c.g[A0][0] = 0x14000000;
c.g[A0][0] = c.g[T3][0] | c.g[A0][0];
c.g[A2][0] = c.g[A2][0] << 47;
c.g[V0][0] = c.g[V1][0];
c.g[A0][0] = c.g[A0][0] << 32;
c.g[V0][0] = c.g[V0][0] << 32;
c.g[A1][0] = sx((c.g[T0][0] as u32) << 2);
c.g[V0][0] = c.g[V0][0] >> 32;
c.g[V1][0] = sx((c.g[T1][0] as u32) << 2);
c.sd((c.g[SP][0] as u32).wrapping_add(0x18), c.g[V0][0]);
c.g[S3][0] = 0x60000;
c.g[V0][0] = 0x8000;
c.g[V0][0] = c.g[V0][0] << 31;
c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[A0][0]);
c.g[T1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[T4][0] as u32)));
c.g[T0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[T4][0] as u32)));
c.g[FP][0] = c.g[A2][0] | c.g[V0][0];
c.g[T8][0] = 0x36;
c.g[S5][0] = 0u64;
c.g[S2][0] = 0x10000000;
c.g[S2][0] = c.g[S2][0] | 0x1;
c.g[S4][0] = 0x30000000;
c.g[S1][0] = 0x30;
c.g[S6][0] = 0x10000000;
c.g[S6][0] = c.g[S6][0] | 0x7;
c.g[S3][0] = c.g[S3][0] | 0x8000;
c.g[V0][0] = ((c.g[A3][0] as i64) < 37i64) as u64;
'l_387fb8: loop {
'b_38804c: {
c.cycles += 17;
c.g[T6][0] = 0x24;
if c.g[V0][0] != 0 { c.g[T6][0] = c.g[A3][0]; }
c.g[T4][0] = sx((c.g[T8][0] as u32).wrapping_sub((c.g[S0][0] as u32)));
c.g[V0][0] = ((c.g[T6][0] as i64) < (c.g[T4][0] as i64)) as u64;
c.g[V1][0] = 0xffffffffffffffff;
if c.g[V0][0] != 0 { c.g[T4][0] = c.g[T6][0]; }
c.g[A0][0] = 0x4a0000;
c.g[T3][0] = sx((c.g[T6][0] as u32).wrapping_sub((c.g[T4][0] as u32)));
c.g[V0][0] = ((c.g[T6][0] as i64) < (c.g[A3][0] as i64)) as u64;
c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[T3][0] as i64)) as u64;
c.g[T7][0] = sx((c.g[A0][0] as u32).wrapping_add(0x43a8));
c.ldl(5, (c.g[T7][0] as u32).wrapping_add(0x7));
c.ldr(5, (c.g[T7][0] as u32));
c.sdl(5, (c.g[SP][0] as u32).wrapping_add(0x7));
c.sdr(5, (c.g[SP][0] as u32));
let t = c.g[V0][0] != 0u64;
if c.g[V1][0] == 0 { c.g[T3][0] = 0u64; }
if t {
break 'b_38804c;
} else {
'b_388008: {
c.cycles += 3;
c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[T6][0] as u32)));
if c.g[T8][0] == c.g[S5][0] {
break 'b_388008;
} else {
break 'b_388008;
}
}
'b_38802c: {
c.cycles += 9;
c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2));
c.div(0, 3, 24);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
c.g[V1][0] = ((c.g[T3][0] as i64) < 2i64) as u64;
c.g[A0][0] = c.hi[0];
c.div(0, 2, 24);
c.sw((c.g[SP][0] as u32), c.g[A0][0]);
if c.g[T8][0] == c.g[S5][0] {
break 'b_38802c;
} else {
break 'b_38802c;
}
}
'b_388040: {
c.cycles += 3;
c.g[A1][0] = c.hi[0];
let t = c.g[V1][0] != 0u64;
c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A1][0]);
if t {
break 'b_388040;
} else {
c.cycles += 2;
let t = true;
c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_sub(0x2));
if t {
break 'b_38804c;
} else {
break 'b_388040;
}
}
}
c.cycles += 3;
c.g[V0][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x2));
c.g[T4][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T3][0] as u32)));
c.g[T3][0] = 0u64;
break 'b_38804c;
}
}
'b_388120: {
c.cycles += 33;
c.mfc1(2, 12);
c.g[A0][0] = sx((c.g[T6][0] as u32) << 1);
c.g[V1][0] = c.g[T6][0] << 32;
c.g[T7][0] = 0x1000000;
c.g[T7][0] = c.g[T7][0] | 0x101;
c.g[T7][0] = c.g[T7][0] << 32;
c.g[A1][0] = c.g[V0][0];
c.g[T9][0] = 0x6c000000;
c.g[V0][0] = c.g[A0][0] | 0x8000;
c.g[A1][0] = c.g[A1][0] << 32;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T6][0] as u32)));
c.g[A2][0] = c.g[V0][0] | c.g[FP][0];
c.g[A0][0] = c.g[A0][0] | c.g[V1][0];
c.g[V0][0] = 0xc000;
c.g[V0][0] = c.g[V0][0] << 46;
c.g[V1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x18));
c.g[A2][0] = c.g[A2][0] | c.g[V0][0];
c.g[V0][0] = 0x10000000;
c.g[V0][0] = c.g[V0][0] | 0x3;
c.sd((c.g[T2][0] as u32).wrapping_add(0x20), c.g[A2][0]);
c.g[A1][0] = c.g[V1][0] | c.g[A1][0];
c.sd((c.g[T2][0] as u32), c.g[V0][0]);
c.g[V1][0] = 0xd805;
c.g[V1][0] = c.g[V1][0] << 47;
c.sd((c.g[T2][0] as u32).wrapping_add(0x30), c.g[A0][0]);
c.sd((c.g[T2][0] as u32).wrapping_add(0x18), c.g[V1][0]);
c.g[T5][0] = 0x2;
c.sd((c.g[T2][0] as u32).wrapping_add(0x38), c.g[A1][0]);
c.sd((c.g[T2][0] as u32).wrapping_add(0x8), 0u64);
c.sd((c.g[T2][0] as u32).wrapping_add(0x10), c.g[T7][0]);
c.sd((c.g[T2][0] as u32).wrapping_add(0x28), c.g[S7][0]);
let t = c.g[T4][0] == 0u64;
c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x40));
if t {
break 'b_388120;
} else {
c.cycles += 20;
c.g[V0][0] = sx((c.g[T4][0] as u32) << 1);
c.sd((c.g[T2][0] as u32), c.g[S2][0]);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T4][0] as u32)));
c.sd((c.g[T2][0] as u32).wrapping_add(0x8), 0u64);
c.g[V1][0] = sx((c.g[V0][0] as u32) << 16);
c.sd((c.g[T2][0] as u32).wrapping_add(0x10), c.g[T7][0]);
c.g[V1][0] = c.g[V1][0] | 0x8002;
c.mult(0, 4, 16, 17);
c.g[V1][0] = c.g[V1][0] | c.g[T9][0];
c.g[A1][0] = c.g[V0][0] | c.g[S4][0];
c.g[V1][0] = c.g[V1][0] << 32;
c.g[T5][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
c.sd((c.g[T2][0] as u32).wrapping_add(0x18), c.g[V1][0]);
c.g[V0][0] = c.lw((c.g[T0][0] as u32));
c.sd((c.g[T2][0] as u32).wrapping_add(0x28), 0u64);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[V0][0] = c.g[V0][0] << 32;
c.g[V0][0] = c.g[A1][0] | c.g[V0][0];
c.sd((c.g[T2][0] as u32).wrapping_add(0x20), c.g[V0][0]);
c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x30));
break 'b_388120;
}
}
'b_388170: {
c.cycles += 2;
let t = c.g[T3][0] == 0u64;
c.g[V1][0] = sx((c.g[T3][0] as u32) << 1);
if t {
break 'b_388170;
} else {
c.cycles += 18;
c.sd((c.g[T2][0] as u32), c.g[S2][0]);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[T3][0] as u32)));
c.sd((c.g[T2][0] as u32).wrapping_add(0x8), 0u64);
c.g[V0][0] = sx((c.g[V1][0] as u32) << 16);
c.sd((c.g[T2][0] as u32).wrapping_add(0x10), c.g[T7][0]);
c.g[V0][0] = c.g[V0][0] | 0x8000;
c.g[A0][0] = c.g[V1][0] | c.g[S4][0];
c.g[V0][0] = c.g[T5][0] | c.g[V0][0];
c.g[V0][0] = c.g[V0][0] | c.g[T9][0];
c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[V0][0] = c.g[V0][0] << 32;
c.sd((c.g[T2][0] as u32).wrapping_add(0x18), c.g[V0][0]);
c.g[V1][0] = c.lwu((c.g[T0][0] as u32));
c.sd((c.g[T2][0] as u32).wrapping_add(0x28), 0u64);
c.g[V1][0] = c.g[V1][0] << 32;
c.g[V1][0] = c.g[A0][0] | c.g[V1][0];
c.sd((c.g[T2][0] as u32).wrapping_add(0x20), c.g[V1][0]);
c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x30));
break 'b_388170;
}
}
'b_3881f4: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[SP][0] as u32));
let t = (c.g[V0][0] as i64) < 0;
c.g[V0][0] = c.g[T5][0] | c.g[S3][0];
if t {
break 'b_3881f4;
} else {
c.cycles += 30;
c.sd((c.g[T2][0] as u32), c.g[S6][0]);
c.g[V0][0] = c.g[V0][0] | c.g[T9][0];
c.sd((c.g[T2][0] as u32).wrapping_add(0x8), 0u64);
c.g[V0][0] = c.g[V0][0] << 32;
c.sd((c.g[T2][0] as u32).wrapping_add(0x10), c.g[T7][0]);
c.sd((c.g[T2][0] as u32).wrapping_add(0x18), c.g[V0][0]);
c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_add(0x6));
c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x20));
c.g[A0][0] = c.lw((c.g[SP][0] as u32));
c.g[V1][0] = c.lw((c.g[T0][0] as u32));
c.mult(0, 4, 4, 17);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.lq(2, (c.g[A0][0] as u32));
c.sq(2, (c.g[T2][0] as u32));
c.lq(3, (c.g[A0][0] as u32).wrapping_add(0x10));
c.sq(3, (c.g[T2][0] as u32).wrapping_add(0x10));
c.lq(2, (c.g[A0][0] as u32).wrapping_add(0x20));
c.sq(2, (c.g[T2][0] as u32).wrapping_add(0x20));
c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x30));
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
c.g[V1][0] = c.lw((c.g[T0][0] as u32));
c.mult(0, 4, 4, 17);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.lq(2, (c.g[A0][0] as u32));
c.sq(2, (c.g[T2][0] as u32));
c.lq(3, (c.g[A0][0] as u32).wrapping_add(0x10));
c.sq(3, (c.g[T2][0] as u32).wrapping_add(0x10));
c.lq(2, (c.g[A0][0] as u32).wrapping_add(0x20));
c.sq(2, (c.g[T2][0] as u32).wrapping_add(0x20));
c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x30));
break 'b_3881f4;
}
}
'b_38824c: {
c.cycles += 2;
let t = c.g[T4][0] == 0u64;
c.g[V0][0] = sx((c.g[T4][0] as u32) << 1);
if t {
break 'b_38824c;
} else {
c.cycles += 20;
c.sd((c.g[T2][0] as u32), c.g[S2][0]);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T4][0] as u32)));
c.sd((c.g[T2][0] as u32).wrapping_add(0x8), 0u64);
c.g[V1][0] = sx((c.g[V0][0] as u32) << 16);
c.sd((c.g[T2][0] as u32).wrapping_add(0x10), c.g[T7][0]);
c.g[V1][0] = c.g[V1][0] | 0x8000;
c.mult(0, 4, 16, 17);
c.g[V1][0] = c.g[T5][0] | c.g[V1][0];
c.g[A1][0] = c.g[V0][0] | c.g[S4][0];
c.g[V1][0] = c.g[V1][0] | c.g[T9][0];
c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[V1][0] = c.g[V1][0] << 32;
c.sd((c.g[T2][0] as u32).wrapping_add(0x18), c.g[V1][0]);
c.g[V0][0] = c.lw((c.g[T1][0] as u32));
c.sd((c.g[T2][0] as u32).wrapping_add(0x28), 0u64);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[V0][0] = c.g[V0][0] << 32;
c.g[V0][0] = c.g[A1][0] | c.g[V0][0];
c.sd((c.g[T2][0] as u32).wrapping_add(0x20), c.g[V0][0]);
c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x30));
break 'b_38824c;
}
}
'b_38829c: {
c.cycles += 2;
let t = c.g[T3][0] == 0u64;
c.g[V1][0] = sx((c.g[T3][0] as u32) << 1);
if t {
break 'b_38829c;
} else {
c.cycles += 18;
c.sd((c.g[T2][0] as u32), c.g[S2][0]);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[T3][0] as u32)));
c.sd((c.g[T2][0] as u32).wrapping_add(0x8), 0u64);
c.g[V0][0] = sx((c.g[V1][0] as u32) << 16);
c.sd((c.g[T2][0] as u32).wrapping_add(0x10), c.g[T7][0]);
c.g[V0][0] = c.g[V0][0] | 0x8000;
c.g[A0][0] = c.g[V1][0] | c.g[S4][0];
c.g[V0][0] = c.g[T5][0] | c.g[V0][0];
c.g[V0][0] = c.g[V0][0] | c.g[T9][0];
c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[V0][0] = c.g[V0][0] << 32;
c.sd((c.g[T2][0] as u32).wrapping_add(0x18), c.g[V0][0]);
c.g[V1][0] = c.lwu((c.g[T1][0] as u32));
c.sd((c.g[T2][0] as u32).wrapping_add(0x28), 0u64);
c.g[V1][0] = c.g[V1][0] << 32;
c.g[V1][0] = c.g[A0][0] | c.g[V1][0];
c.sd((c.g[T2][0] as u32).wrapping_add(0x20), c.g[V1][0]);
c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x30));
break 'b_38829c;
}
}
'b_38831c: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[SP][0] as u32));
let t = (c.g[V0][0] as i64) < 0;
c.g[V0][0] = c.g[T5][0] | c.g[S3][0];
if t {
break 'b_38831c;
} else {
c.cycles += 29;
c.sd((c.g[T2][0] as u32).wrapping_add(0x10), c.g[T7][0]);
c.g[V0][0] = c.g[V0][0] | c.g[T9][0];
c.sd((c.g[T2][0] as u32), c.g[S6][0]);
c.g[V0][0] = c.g[V0][0] << 32;
c.sd((c.g[T2][0] as u32).wrapping_add(0x8), 0u64);
c.sd((c.g[T2][0] as u32).wrapping_add(0x18), c.g[V0][0]);
c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x20));
c.g[A0][0] = c.lw((c.g[SP][0] as u32));
c.g[V1][0] = c.lw((c.g[T1][0] as u32));
c.mult(0, 4, 4, 17);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.lq(2, (c.g[A0][0] as u32));
c.sq(2, (c.g[T2][0] as u32));
c.lq(3, (c.g[A0][0] as u32).wrapping_add(0x10));
c.sq(3, (c.g[T2][0] as u32).wrapping_add(0x10));
c.lq(2, (c.g[A0][0] as u32).wrapping_add(0x20));
c.sq(2, (c.g[T2][0] as u32).wrapping_add(0x20));
c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x30));
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
c.g[V1][0] = c.lw((c.g[T1][0] as u32));
c.mult(0, 4, 4, 17);
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.lq(2, (c.g[A0][0] as u32));
c.sq(2, (c.g[T2][0] as u32));
c.lq(3, (c.g[A0][0] as u32).wrapping_add(0x10));
c.sq(3, (c.g[T2][0] as u32).wrapping_add(0x10));
c.lq(2, (c.g[A0][0] as u32).wrapping_add(0x20));
c.sq(2, (c.g[T2][0] as u32).wrapping_add(0x20));
c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x30));
break 'b_38831c;
}
}
'b_38836c: {
c.cycles += 12;
c.mtc1(14, 0);
c.cvt_s_w(0, 0);
c.sd((c.g[T2][0] as u32), c.g[S2][0]);
c.sd((c.g[T2][0] as u32).wrapping_add(0x8), 0u64);
c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub((c.g[T6][0] as u32)));
c.sd((c.g[T2][0] as u32).wrapping_add(0x10), 0u64);
c.mul_s(0, 13, 0);
c.g[A1][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
c.sd((c.g[T2][0] as u32).wrapping_add(0x18), c.g[A1][0]);
c.add_s(12, 12, 0);
let t = (c.g[A3][0] as i64) <= 0;
c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x20));
if t {
break 'b_38836c;
} else {
'b_388358: {
c.cycles += 3;
c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[T6][0] as u32)));
if c.g[T8][0] == c.g[S5][0] {
break 'b_388358;
} else {
break 'b_388358;
}
}
c.cycles += 5;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
c.div(0, 2, 24);
c.g[V1][0] = c.hi[0];
c.g[S0][0] = c.g[V1][0];
break 'b_38836c;
}
}
c.cycles += 2;
let t = c.g[A3][0] != 0u64;
c.g[V0][0] = ((c.g[A3][0] as i64) < 37i64) as u64;
if t {
if event_due(c) { deliver_event_here(c, 0x00387fb8); } continue 'l_387fb8;
} else {
break 'b_388374;
}
}
}
}
c.cycles += 16;
c.g[V0][0] = 0xfff0000;
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[V0][0] = c.g[V0][0] | 0xffff;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[V0][0] = c.g[T2][0] & c.g[V0][0];
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x90));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sw((c.g[V1][0] as u32).wrapping_add(0x5a00), c.g[V0][0]);
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
c.pc = t; return Ok(());
    });
}

/// `PS2GraphicsMan_EmitBoardWakeStrips` (0x3885e0): emits the board-wake triangle strips in batches of up to 36
/// vertices (GIF header, vertex and index copies) into the packet buffer at `mgr+0x5a00`, which
/// it advances. A leaf; sibling of `EmitShadowStrips` with its own register allocation.
pub fn ps2_graphics_man_emit_board_wake_strips(c: &mut Cpu) {
    guest(c, |c| {
'b_388634: {
'b_38862c: {
c.cycles += 16;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x12c0));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
c.g[T4][0] = c.g[A1][0];
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
c.g[T7][0] = c.g[A2][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
c.g[S0][0] = c.g[A3][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
let t = c.g[V0][0] == 0u64;
c.sw((c.g[SP][0] as u32), c.g[A0][0]);
if t {
break 'b_38862c;
} else {
c.cycles += 3;
c.g[V0][0] = 0x0;
let t = true;
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3a00));
if t {
break 'b_388634;
} else {
break 'b_38862c;
}
}
}
c.cycles += 2;
c.g[V0][0] = 0x0;
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3a08));
break 'b_388634;
}
'b_3889ac: {
c.cycles += 8;
c.g[V0][0] = c.lw((c.g[SP][0] as u32));
c.g[A2][0] = sx((c.g[A0][0] as u32) >> 3);
c.g[S6][0] = 0x1;
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x5a00));
c.g[V0][0] = 0x30000000;
c.sw((c.g[GP][0] as u32).wrapping_add(0x12c0), 0u64);
let t = c.g[T0][0] == 0u64;
c.g[T3][0] = c.g[V1][0] | c.g[V0][0];
if t {
break 'b_3889ac;
} else {
c.cycles += 19;
c.mfc1(2, 13);
c.g[A0][0] = 0x14000000;
c.g[A0][0] = c.g[A2][0] | c.g[A0][0];
c.g[A1][0] = sx((c.g[T1][0] as u32) << 2);
c.g[V1][0] = c.g[V0][0];
c.g[S1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[T4][0] as u32)));
c.g[V0][0] = sx((c.g[T2][0] as u32) << 2);
c.g[V1][0] = c.g[V1][0] << 32;
c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T4][0] as u32)));
c.g[FP][0] = c.g[V1][0] >> 32;
c.g[S7][0] = c.g[A0][0] << 32;
c.g[S5][0] = 0x10000000;
c.g[S5][0] = c.g[S5][0] | 0x4;
c.g[S4][0] = 0x30;
c.g[T2][0] = 0x10000000;
c.g[T2][0] = c.g[T2][0] | 0x1;
c.g[S2][0] = 0x30000000;
c.g[V0][0] = ((c.g[T0][0] as i64) < 37i64) as u64;
'l_3886a0: loop {
'b_3886d8: {
c.cycles += 11;
c.g[T8][0] = 0x24;
if c.g[V0][0] != 0 { c.g[T8][0] = c.g[T0][0]; }
c.g[T4][0] = sx((c.g[S0][0] as u32).wrapping_sub((c.g[T7][0] as u32)));
c.g[V1][0] = ((c.g[T8][0] as i64) < (c.g[T4][0] as i64)) as u64;
c.g[V0][0] = 0xffffffffffffffff;
if c.g[V1][0] != 0 { c.g[T4][0] = c.g[T8][0]; }
c.g[A3][0] = 0x1;
c.g[T5][0] = sx((c.g[T8][0] as u32).wrapping_sub((c.g[T4][0] as u32)));
c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[T5][0] as i64)) as u64;
let t = c.g[S6][0] == 0u64;
if c.g[V0][0] == 0 { c.g[T5][0] = 0u64; }
if t {
break 'b_3886d8;
} else {
c.cycles += 3;
c.g[A3][0] = c.g[T7][0];
c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_sub(0x1));
c.g[T7][0] = sx((c.g[T7][0] as u32).wrapping_add(0x1));
break 'b_3886d8;
}
}
'b_3887b0: {
c.cycles += 35;
c.mfc1(3, 12);
c.g[A0][0] = sx((c.g[T8][0] as u32) << 1);
c.g[V0][0] = c.g[A0][0] | 0x8000;
c.g[T9][0] = 0x1000000;
c.g[T9][0] = c.g[T9][0] | 0x101;
c.g[T9][0] = c.g[T9][0] << 32;
c.g[A1][0] = c.g[V1][0];
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T8][0] as u32)));
c.g[V1][0] = 0xb900;
c.g[V1][0] = c.g[V1][0] << 38;
c.g[A1][0] = c.g[A1][0] << 32;
c.g[A2][0] = c.g[V0][0] | c.g[V1][0];
c.g[A1][0] = c.g[FP][0] | c.g[A1][0];
c.g[V0][0] = 0xc000;
c.g[V0][0] = c.g[V0][0] << 46;
c.g[V1][0] = c.g[T8][0] << 32;
c.g[A2][0] = c.g[A2][0] | c.g[V0][0];
c.g[A0][0] = c.g[A0][0] | c.g[V1][0];
c.g[V0][0] = 0x10000000;
c.g[V0][0] = c.g[V0][0] | 0x3;
c.g[V1][0] = 0xd805;
c.g[V1][0] = c.g[V1][0] << 47;
c.sd((c.g[T3][0] as u32), c.g[V0][0]);
c.g[T1][0] = 0x6c000000;
c.g[V0][0] = 0x412;
c.sd((c.g[T3][0] as u32).wrapping_add(0x18), c.g[V1][0]);
c.sd((c.g[T3][0] as u32).wrapping_add(0x20), c.g[A2][0]);
c.g[T6][0] = 0x2;
c.sd((c.g[T3][0] as u32).wrapping_add(0x30), c.g[A0][0]);
c.sd((c.g[T3][0] as u32).wrapping_add(0x38), c.g[A1][0]);
c.sd((c.g[T3][0] as u32).wrapping_add(0x8), 0u64);
c.sd((c.g[T3][0] as u32).wrapping_add(0x10), c.g[T9][0]);
c.sd((c.g[T3][0] as u32).wrapping_add(0x28), c.g[V0][0]);
let t = (c.g[A3][0] as i64) < 0;
c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x40));
if t {
break 'b_3887b0;
} else {
c.cycles += 19;
c.g[V0][0] = 0x6c030000;
c.g[V0][0] = c.g[V0][0] | 0x8002;
c.g[V0][0] = c.g[V0][0] << 32;
c.sd((c.g[T3][0] as u32), c.g[S5][0]);
c.sd((c.g[T3][0] as u32).wrapping_add(0x18), c.g[V0][0]);
c.mult(0, 4, 7, 20);
c.sd((c.g[T3][0] as u32).wrapping_add(0x8), 0u64);
c.g[T6][0] = 0x5;
c.sd((c.g[T3][0] as u32).wrapping_add(0x10), c.g[T9][0]);
c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x20));
c.g[V1][0] = c.lw((c.g[S1][0] as u32));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.lq(2, (c.g[A0][0] as u32));
c.sq(2, (c.g[T3][0] as u32));
c.lq(3, (c.g[A0][0] as u32).wrapping_add(0x10));
c.sq(3, (c.g[T3][0] as u32).wrapping_add(0x10));
c.lq(2, (c.g[A0][0] as u32).wrapping_add(0x20));
c.sq(2, (c.g[T3][0] as u32).wrapping_add(0x20));
c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x30));
break 'b_3887b0;
}
}
'b_388808: {
c.cycles += 2;
let t = c.g[T4][0] == 0u64;
c.g[V0][0] = sx((c.g[T4][0] as u32) << 1);
if t {
break 'b_388808;
} else {
c.cycles += 20;
c.sd((c.g[T3][0] as u32), c.g[T2][0]);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T4][0] as u32)));
c.sd((c.g[T3][0] as u32).wrapping_add(0x8), 0u64);
c.g[V1][0] = sx((c.g[V0][0] as u32) << 16);
c.sd((c.g[T3][0] as u32).wrapping_add(0x10), c.g[T9][0]);
c.g[V1][0] = c.g[V1][0] | 0x8000;
c.mult(0, 4, 15, 20);
c.g[V1][0] = c.g[T6][0] | c.g[V1][0];
c.g[A1][0] = c.g[V0][0] | c.g[S2][0];
c.g[V1][0] = c.g[V1][0] | c.g[T1][0];
c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[V1][0] = c.g[V1][0] << 32;
c.sd((c.g[T3][0] as u32).wrapping_add(0x18), c.g[V1][0]);
c.g[V0][0] = c.lw((c.g[S1][0] as u32));
c.sd((c.g[T3][0] as u32).wrapping_add(0x28), 0u64);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[V0][0] = c.g[V0][0] << 32;
c.g[V0][0] = c.g[A1][0] | c.g[V0][0];
c.sd((c.g[T3][0] as u32).wrapping_add(0x20), c.g[V0][0]);
c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x30));
break 'b_388808;
}
}
'b_388858: {
c.cycles += 2;
let t = c.g[T5][0] == 0u64;
c.g[V1][0] = sx((c.g[T5][0] as u32) << 1);
if t {
break 'b_388858;
} else {
c.cycles += 18;
c.sd((c.g[T3][0] as u32), c.g[T2][0]);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[T5][0] as u32)));
c.sd((c.g[T3][0] as u32).wrapping_add(0x8), 0u64);
c.g[V0][0] = sx((c.g[V1][0] as u32) << 16);
c.sd((c.g[T3][0] as u32).wrapping_add(0x10), c.g[T9][0]);
c.g[V0][0] = c.g[V0][0] | 0x8000;
c.g[A0][0] = c.g[V1][0] | c.g[S2][0];
c.g[V0][0] = c.g[T6][0] | c.g[V0][0];
c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[V0][0] = c.g[V0][0] << 32;
c.sd((c.g[T3][0] as u32).wrapping_add(0x18), c.g[V0][0]);
c.g[V1][0] = c.lwu((c.g[S1][0] as u32));
c.sd((c.g[T3][0] as u32).wrapping_add(0x28), 0u64);
c.g[V1][0] = c.g[V1][0] << 32;
c.g[V1][0] = c.g[A0][0] | c.g[V1][0];
c.sd((c.g[T3][0] as u32).wrapping_add(0x20), c.g[V1][0]);
c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x30));
break 'b_388858;
}
}
'b_3888b0: {
c.cycles += 2;
let t = (c.g[A3][0] as i64) < 0;
c.g[V0][0] = 0x30000;
if t {
break 'b_3888b0;
} else {
c.cycles += 20;
c.sd((c.g[T3][0] as u32), c.g[S5][0]);
c.g[V0][0] = c.g[V0][0] | 0x8000;
c.sd((c.g[T3][0] as u32).wrapping_add(0x8), 0u64);
c.g[V0][0] = c.g[T6][0] | c.g[V0][0];
c.sd((c.g[T3][0] as u32).wrapping_add(0x10), c.g[T9][0]);
c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
c.mult(0, 4, 7, 20);
c.g[V0][0] = c.g[V0][0] << 32;
c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_add(0x3));
c.sd((c.g[T3][0] as u32).wrapping_add(0x18), c.g[V0][0]);
c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x20));
c.g[V1][0] = c.lw((c.g[S3][0] as u32));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.lq(2, (c.g[A0][0] as u32));
c.sq(2, (c.g[T3][0] as u32));
c.lq(3, (c.g[A0][0] as u32).wrapping_add(0x10));
c.sq(3, (c.g[T3][0] as u32).wrapping_add(0x10));
c.lq(2, (c.g[A0][0] as u32).wrapping_add(0x20));
c.sq(2, (c.g[T3][0] as u32).wrapping_add(0x20));
c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x30));
break 'b_3888b0;
}
}
'b_388908: {
c.cycles += 2;
let t = c.g[T4][0] == 0u64;
c.g[V0][0] = sx((c.g[T4][0] as u32) << 1);
if t {
break 'b_388908;
} else {
c.cycles += 20;
c.sd((c.g[T3][0] as u32), c.g[T2][0]);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T4][0] as u32)));
c.sd((c.g[T3][0] as u32).wrapping_add(0x8), 0u64);
c.g[V1][0] = sx((c.g[V0][0] as u32) << 16);
c.sd((c.g[T3][0] as u32).wrapping_add(0x10), c.g[T9][0]);
c.g[V1][0] = c.g[V1][0] | 0x8000;
c.mult(0, 4, 15, 20);
c.g[V1][0] = c.g[T6][0] | c.g[V1][0];
c.g[A1][0] = c.g[V0][0] | c.g[S2][0];
c.g[V1][0] = c.g[V1][0] | c.g[T1][0];
c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[V1][0] = c.g[V1][0] << 32;
c.sd((c.g[T3][0] as u32).wrapping_add(0x18), c.g[V1][0]);
c.g[V0][0] = c.lw((c.g[S3][0] as u32));
c.sd((c.g[T3][0] as u32).wrapping_add(0x28), 0u64);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[V0][0] = c.g[V0][0] << 32;
c.g[V0][0] = c.g[A1][0] | c.g[V0][0];
c.sd((c.g[T3][0] as u32).wrapping_add(0x20), c.g[V0][0]);
c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x30));
break 'b_388908;
}
}
'b_388954: {
c.cycles += 2;
let t = c.g[T5][0] == 0u64;
c.g[V0][0] = sx((c.g[T5][0] as u32) << 1);
if t {
break 'b_388954;
} else {
c.cycles += 17;
c.sd((c.g[T3][0] as u32).wrapping_add(0x10), c.g[T9][0]);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T5][0] as u32)));
c.sd((c.g[T3][0] as u32), c.g[T2][0]);
c.g[V1][0] = sx((c.g[V0][0] as u32) << 16);
c.sd((c.g[T3][0] as u32).wrapping_add(0x8), 0u64);
c.g[V1][0] = c.g[V1][0] | 0x8000;
c.g[V0][0] = c.g[V0][0] | c.g[S2][0];
c.g[V1][0] = c.g[T6][0] | c.g[V1][0];
c.g[V1][0] = c.g[V1][0] | c.g[T1][0];
c.g[V1][0] = c.g[V1][0] << 32;
c.sd((c.g[T3][0] as u32).wrapping_add(0x18), c.g[V1][0]);
c.g[A0][0] = c.lwu((c.g[S3][0] as u32));
c.sd((c.g[T3][0] as u32).wrapping_add(0x28), 0u64);
c.g[A0][0] = c.g[A0][0] << 32;
c.g[A0][0] = c.g[V0][0] | c.g[A0][0];
c.sd((c.g[T3][0] as u32).wrapping_add(0x20), c.g[A0][0]);
c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x30));
break 'b_388954;
}
}
'b_3889a4: {
c.cycles += 11;
c.mtc1(24, 0);
c.cvt_s_w(0, 0);
c.sd((c.g[T3][0] as u32), c.g[T2][0]);
c.sd((c.g[T3][0] as u32).wrapping_add(0x8), 0u64);
c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub((c.g[T8][0] as u32)));
c.sd((c.g[T3][0] as u32).wrapping_add(0x10), 0u64);
c.mul_s(0, 13, 0);
c.sd((c.g[T3][0] as u32).wrapping_add(0x18), c.g[S7][0]);
c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x20));
let t = (c.g[T0][0] as i64) <= 0;
c.add_s(12, 12, 0);
if t {
break 'b_3889a4;
} else {
'b_38898c: {
c.cycles += 3;
c.g[V0][0] = sx((c.g[T7][0] as u32).wrapping_add((c.g[T8][0] as u32)));
if c.g[S0][0] == 0u64 {
break 'b_38898c;
} else {
break 'b_38898c;
}
}
c.cycles += 6;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x1));
c.div(0, 2, 16);
c.g[S6][0] = 0u64;
c.g[V1][0] = c.hi[0];
c.g[T7][0] = c.g[V1][0];
break 'b_3889a4;
}
}
c.cycles += 2;
let t = c.g[T0][0] != 0u64;
c.g[V0][0] = ((c.g[T0][0] as i64) < 37i64) as u64;
if t {
if event_due(c) { deliver_event_here(c, 0x003886a0); } continue 'l_3886a0;
} else {
break 'b_3889ac;
}
}
}
}
c.cycles += 16;
c.g[V0][0] = 0xfff0000;
c.g[V1][0] = c.lw((c.g[SP][0] as u32));
c.g[V0][0] = c.g[V0][0] | 0xffff;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
c.g[V0][0] = c.g[T3][0] & c.g[V0][0];
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sw((c.g[V1][0] as u32).wrapping_add(0x5a00), c.g[V0][0]);
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
c.pc = t; return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::{check_lohi, scramble, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Gate word of the oc1568 fakes (own word and magic).
    const OC1568_CTRL: u32 = DATA + 0xe_8000;
    const OC1568_MAGIC: u32 = 0x1568_0278;

    /// Fixture addresses, all outside the fakes' pointer range DATA..DATA+0x40000.
    const OC1568_OBJ: u32 = DATA + 0x5_0000;
    const OC1568_INFO: u32 = DATA + 0x5_2000;
    const OC1568_RECS: u32 = DATA + 0x5_3000;
    const OC1568_LIST: u32 = DATA + 0x5_4000;
    const OC1568_MATS: u32 = DATA + 0x5_5000;
    const OC1568_COUNTER: u32 = DATA + 0x7_0000;
    const OC1568_GS: u32 = DATA + 0x8_1000;
    const OC1568_TBL: u32 = DATA + 0x8_2000;
    const OC1568_TEX: u32 = DATA + 0x8_3000;
    const OC1568_ARGS: u32 = DATA + 0x8_4000;
    const OC1568_PKT: u32 = DATA + 0xa_0000;
    const OC1568_ARR: u32 = DATA + 0xb_0000;
    const OC1568_VERTS: u32 = DATA + 0xc_0000;

    /// Gated fakes for the callees that have override slots: with the gate
    /// word set they log the registers (`fakes::fake`) and return; with it
    /// clear they run what the slot held before.
    macro_rules! oc1568_fakes {
        ($($fake:ident, $slot:path, $gen:path, $addr:expr;)*) => {
            $(
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1568_CTRL) != OC1568_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), $gen);
                    }
                    let ra = c.g[RA][0] as u32;
                    fakes::fake(c, $addr, ra);
                }
            )*

            fn oc1568_hooks() {
                $(
                    fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake));
                )*
            }
        };
    }

    oc1568_fakes! {
        oc1568_fake_mem_free, ee::s10::OVERRIDE_00317E98, ee::s10::f_00317e98__generated, 0x0031_7e98;
        oc1568_fake_sprs_begin, ee::s12::OVERRIDE_0038F460, ee::s12::f_0038f460__generated, 0x0038_f460;
        oc1568_fake_sprs_end, ee::s12::OVERRIDE_0038F668, ee::s12::f_0038f668__generated, 0x0038_f668;
    }

    /// `fakes::machine` plus the gated fakes and the gate word.
    fn oc1568_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = fakes::machine(c, seed, salt, calls);
        oc1568_hooks();
        c.bus.write32(OC1568_CTRL, OC1568_MAGIC);
        r
    }

    /// Small unit floats (both signs, some exact zeros) at `addr..addr + 4 * n`.
    fn oc1568_floats(c: &mut Cpu, r: &mut Rng, addr: u32, n: u32) {
        for k in 0..n {
            let v = if r.next_u32() % 8 == 0 { 0 } else { r.f32(2.0) };
            c.bus.write32(addr + 4 * k, v);
        }
    }

    /// `MultiSplineModifier_v22`: the modifier at `OBJ` has `n` spline records
    /// (stride 0x60) and an index list; the info's key (`INFO+0x50`) matches
    /// list entry `seed % (n + 1)` (`n`: no match, early exit). The first
    /// vector component of the record is small or large (scalar FPU path or
    /// VU0 normalise).
    #[test]
    fn multi_spline_modifier_v22_matches() {
        check_lohi(
            ee::s11::f_0035b200__generated,
            multi_spline_modifier_v22,
            300,
            &|c, seed| {
                let mut r = Rng(0x35b200 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.bus.write32(GAME_GP - 0x2a10, 0.5f32.to_bits());
                let n = if seed % 11 == 0 { 0 } else { 1 + seed % 4 };
                c.bus.write32(OC1568_OBJ + 4, n);
                c.bus.write32(OC1568_OBJ + 0x3c, OC1568_RECS);
                c.bus.write32(OC1568_OBJ + 0x44, OC1568_LIST);
                oc1568_floats(c, &mut r, OC1568_OBJ + 0x10, 8);
                let key = OC1568_MATS + 0x100 * (r.next_u32() % 4);
                c.bus.write32(OC1568_INFO + 0x50, key);
                oc1568_floats(c, &mut r, OC1568_INFO, 16);
                let hit = (seed / 4) % (n.max(1) + 1);
                for i in 0..4u32 {
                    let entry = if i == hit { key } else { key + 0x40 * (1 + i) };
                    c.bus.write32(OC1568_LIST + 4 * i, entry);
                }
                // The matched list entry is also the matrix pointer (`+ 0x10`).
                oc1568_floats(c, &mut r, OC1568_MATS, 0x200);
                oc1568_floats(c, &mut r, OC1568_RECS, 0x18 * 4);
                for i in 0..4u32 {
                    let big = (seed / 3 + i) % 2 == 0;
                    let first = if big { 1.5f32 } else { 0.25f32 };
                    let sign = if (seed / 5 + i) % 2 == 0 { 1.0 } else { -1.0 };
                    c.bus.write32(OC1568_RECS + 0x60 * i + 0x10, (first * sign).to_bits());
                    c.bus.write32(OC1568_RECS + 0x60 * i + 0x50, 1.0f32.to_bits());
                    c.bus.write32(OC1568_RECS + 0x60 * i + 0x54, (0.5f32 + i as f32).to_bits());
                }
                c.g[A0][0] = sx(OC1568_OBJ);
                c.g[A1][0] = sx(OC1568_INFO);
            },
        );
    }

    /// The fields of a `BezierPatchCache` that hold owned buffers.
    const OC1568_BPC_FIELDS: [u32; 21] = [
        0xc, 0x10, 0x14, 0x24, 0x28, 0x2c, 0x48, 0x4c, 0x50, 0x54, 0x58, 0x5c, 0x60, 0x64, 0x68,
        0x1b4, 0x1b8, 0x1bc, 0x1c0, 0x1c4, 0x1c8,
    ];

    /// `BezierPatchCache_Free`: each of the 21 buffer pointers is null or not
    /// (seed bits); every non-null one is released through the (gated fake)
    /// `EA_MemFree`, and one call unwinds.
    #[test]
    fn bezier_patch_cache_free_matches() {
        fakes::covered(
            ee::s12::f_003737d0__generated,
            bezier_patch_cache_free,
            200,
            &|c, seed| {
                let mut r = oc1568_machine(c, seed, 0x3737d0, 12);
                c.g[A0][0] = sx(OC1568_OBJ);
                let mask = if seed % 5 == 0 { 0x1f_ffff } else { r.next_u32() & r.next_u32() };
                for (i, off) in OC1568_BPC_FIELDS.iter().enumerate() {
                    let v = if (mask >> i) & 1 != 0 { 0x0050_0000 + 0x100 * i as u32 } else { 0 };
                    c.bus.write32(OC1568_OBJ + off, v);
                }
            },
        );
    }

    /// `DrawTransition`: manager at `OBJ` with GS state, texture table and
    /// packet counter; the SPR stream calls are gated fakes. Seeds vary the
    /// texture size sign, the counter (below or above 2600) and the unwind.
    #[test]
    fn draw_transition_matches() {
        fakes::covered(
            ee::s12::f_0037cc98__generated,
            draw_transition,
            200,
            &|c, seed| {
                let mut r = oc1568_machine(c, seed, 0x37cc98, 2);
                c.pc = 0x0037_cc98;
                c.g[A0][0] = sx(OC1568_OBJ);
                c.g[A1][0] = sx(OC1568_ARGS);
                c.bus.write32(OC1568_ARGS + 4, seed % 3);
                c.bus.write32(OC1568_OBJ + 0xe84, OC1568_GS);
                for k in 0..0x20 {
                    c.bus.write32(OC1568_GS + 4 * k, r.next_u32());
                }
                c.bus.write32(OC1568_OBJ + 0x18f4, OC1568_TBL);
                for i in 0..3u32 {
                    c.bus.write32(OC1568_TBL + 8 + 4 * i, OC1568_TEX + 0x40 * i);
                    let size = if seed % 3 == 0 { r.next_u32() | 0x8000_0000 } else { r.next_u32() >> 8 };
                    c.bus.write32(OC1568_TEX + 0x40 * i + 0x28, size);
                }
                c.bus.write32(OC1568_OBJ + 0x18f0, OC1568_COUNTER);
                let count = if seed % 4 == 0 { 2600 + seed } else { seed % 40 };
                c.bus.write32(OC1568_COUNTER, count);
                for k in 0..2 {
                    c.bus.write32(OC1568_COUNTER + 0x68000 + 0x1cc0 + 4 * k, r.next_u32());
                }
                for off in [0x5a00u32, 0x5a54, 0x5a58, 0x5a5c, 0x5a80, 0x5a88, 0x5900, 0x5904] {
                    c.bus.write32(OC1568_OBJ + off, r.next_u32() & 0x3ff);
                }
                c.bus.write32(GAME_GP + 0x1384, r.next_u32());
            },
        );
    }

    /// Shared fixture of the two strip emitters: the manager's packet buffer
    /// pointer at `OBJ+0x5a00`, the vertex-offset arrays and floats.
    fn oc1568_strip_setup(c: &mut Cpu, seed: u32, salt: u64, flag_off: u32) -> Rng {
        let mut r = Rng(salt ^ (seed as u64) << 8);
        scramble(c, &mut r);
        c.g[GP][0] = sx(GAME_GP);
        c.bus.write32(OC1568_OBJ + 0x5a00, OC1568_PKT);
        c.bus.write32(GAME_GP + flag_off, (seed % 3 != 0) as u32);
        for k in 0..16u32 {
            c.bus.write32(OC1568_ARR + 4 * k, OC1568_VERTS + 0x10 * (r.next_u32() % 64));
        }
        for k in 0..0x300u32 {
            c.bus.write32(OC1568_VERTS + 4 * k, r.next_u32());
        }
        c.f[12] = r.f32(4.0);
        c.f[13] = r.f32(4.0);
        c.g[A0][0] = sx(OC1568_OBJ);
        c.g[A1][0] = sx(OC1568_ARR);
        r
    }

    /// Vertex counts that hit zero, partial and full 36-vertex batches.
    const OC1568_COUNTS: [u32; 11] = [0, 1, 2, 3, 35, 36, 37, 38, 72, 73, 100];

    /// `EmitShadowStrips`: seeds vary the start offset, the vertex count
    /// (no / one / several batches), the flag word and the `a6` test.
    #[test]
    fn ps2_graphics_man_emit_shadow_strips_matches() {
        check_lohi(
            ee::s12::f_00387ec0__generated,
            ps2_graphics_man_emit_shadow_strips,
            330,
            &|c, seed| {
                let mut r = oc1568_strip_setup(c, seed, 0x387ec0, 0x12b0);
                c.g[A2][0] = ((seed / 11) % 50) as u64;
                c.g[A3][0] = OC1568_COUNTS[(seed % 11) as usize] as u64;
                c.g[T0][0] = (r.next_u32() % 6) as u64;
                c.g[T1][0] = (r.next_u32() % 6) as u64;
                if seed % 4 == 0 {
                    c.g[T2][0] = 0;
                }
            },
        );
    }

    /// `EmitBoardWakeStrips`: seeds vary the start offset, the end index, the
    /// vertex count and the flag word; `a2..a3` as the generated body reads.
    #[test]
    fn ps2_graphics_man_emit_board_wake_strips_matches() {
        check_lohi(
            ee::s12::f_003885e0__generated,
            ps2_graphics_man_emit_board_wake_strips,
            330,
            &|c, seed| {
                let mut r = oc1568_strip_setup(c, seed, 0x3885e0, 0x12c0);
                c.g[A2][0] = ((seed / 11) % 40) as u64;
                c.g[A3][0] = ((seed / 3) % 60) as u64;
                c.g[T0][0] = OC1568_COUNTS[(seed % 11) as usize] as u64;
                c.g[T1][0] = (r.next_u32() % 6) as u64;
                c.g[T2][0] = (r.next_u32() % 6) as u64;
            },
        );
    }
}
