//! Staging module oc1463 (wave wave274): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1463`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, X};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0037_81a0, ps2_graphics_man_v79, "PS2GraphicsMan_v79")];

/// `void cPS2GraphicsMan::DrawRotatedSprite(const t4Vector* pos, float* size, int* uv0,
/// int* uv1, float* col, int count, float angle)` (0x003781a0, slot 79 of the
/// graphics manager vtable): draws `count` copies of one rotated, textured,
/// blended sprite. It projects the centre `pos` and the two edge points (the
/// view-space rotation of (cos, sin) and (-sin, cos), scaled by `size`) through
/// the manager's matrices, builds the four GS corners in 12.4 fixed point around
/// the screen centre, then writes `count` GIF packets (cnt tag, DIRECT, PRIM 0x54
/// with ST/RGBAQ/XYZ2 quads) at `mVifCursor | 0x30000000` and, while the defer
/// ring holds fewer than 2600 entries, appends a 0x80-byte ring entry (draw
/// state, packet start/end, view index). Rebuilds the view matrices first when
/// `mMatricesValid` is 0. Kept in the generated block structure (every
/// register, cycle and call).
/// Resume entries (any `pc` other than the entry) run the generated body.
pub fn ps2_graphics_man_v79(c: &mut Cpu) {
    if c.pc != 0x003781a0 {
        return ee::s12::f_003781a0__generated(c);
    }
    guest(c, |c| {
'b_3781fc: {
c.cycles += 21;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x170));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
c.g[S0][0] = c.g[A0][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x130));
c.g[S1][0] = c.g[T0][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x120));
c.g[S2][0] = c.g[A3][0];
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x110));
c.g[S3][0] = c.g[A1][0];
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x100));
c.g[S4][0] = c.g[T1][0];
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.g[S5][0] = c.g[A2][0];
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x160));
c.g[S6][0] = c.g[T2][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[RA][0]);
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x168));
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6b90));
let t = c.g[V0][0] != 0u64;
c.mov_s(20, 12);
if t {
break 'b_3781fc;
} else {
c.cycles += 2;
c.g[RA][0] = sx(0x003781fc);
Callee { addr: 0x0037d968, f: ee::s12::PS2GraphicsMan_UpdateViewMatrices }.call(c, 0x003781fc)?;
break 'b_3781fc;
}
}
'b_378724: {
c.cycles += 255;
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10d8));
c.g[A0][0] = c.g[SP][0];
c.g[A2][0] = c.g[S3][0];
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 21);
c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x2e0));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2e4));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x00378220);
c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
call_ptr(c, t, 0x00378220)?;
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd4));
c.mov_s(12, 20);
c.g[RA][0] = sx(0x00378230);
c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
call_ptr(c, 0x0031be50, 0x00378230)?;
c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0xd4));
c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sw((c.g[SP][0] as u32).wrapping_add(0x28), 0u64);
c.neg_s(7, 0);
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x24));
c.sw((c.g[SP][0] as u32).wrapping_add(0x2c), 0u64);
c.lwc1(11, (c.g[SP][0] as u32).wrapping_add(0x28));
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x13e4));
c.lwc1(10, (c.g[V0][0] as u32).wrapping_add(0x4));
c.lwc1(8, (c.g[V0][0] as u32).wrapping_add(0x14));
c.lwc1(4, (c.g[V0][0] as u32).wrapping_add(0x24));
c.mul_s(10, 0, 10);
c.lwc1(9, (c.g[V0][0] as u32).wrapping_add(0x34));
c.mul_s(8, 0, 8);
c.lwc1(6, (c.g[V0][0] as u32));
c.mul_s(4, 0, 4);
c.lwc1(5, (c.g[V0][0] as u32).wrapping_add(0x10));
c.mul_s(0, 0, 9);
c.lwc1(3, (c.g[V0][0] as u32).wrapping_add(0x20));
c.mul_s(6, 2, 6);
c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x30));
c.mul_s(5, 2, 5);
c.mul_s(3, 2, 3);
c.swc1(11, (c.g[SP][0] as u32).wrapping_add(0x3c));
c.mul_s(1, 2, 1);
c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x30));
c.add_s(6, 6, 10);
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x34));
c.add_s(5, 5, 8);
c.swc1(11, (c.g[SP][0] as u32).wrapping_add(0x38));
c.add_s(3, 3, 4);
c.add_s(1, 1, 0);
c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x10));
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x14));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x18));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x1c));
c.lwc1(6, (c.g[V0][0] as u32));
c.lwc1(8, (c.g[V0][0] as u32).wrapping_add(0x4));
c.lwc1(3, (c.g[V0][0] as u32).wrapping_add(0x10));
c.mul_s(6, 7, 6);
c.lwc1(4, (c.g[V0][0] as u32).wrapping_add(0x14));
c.mul_s(8, 2, 8);
c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x20));
c.mul_s(3, 7, 3);
c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x30));
c.mul_s(4, 2, 4);
c.lwc1(5, (c.g[V0][0] as u32).wrapping_add(0x34));
c.mul_s(0, 7, 0);
c.lwc1(9, (c.g[V0][0] as u32).wrapping_add(0x24));
c.mul_s(7, 7, 1);
c.mul_s(5, 2, 5);
c.mul_s(2, 2, 9);
c.add_s(6, 6, 8);
c.add_s(3, 3, 4);
c.add_s(7, 7, 5);
c.add_s(0, 0, 2);
c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x20));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x24));
c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x2c));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x28));
c.lwc1(0, (c.g[S5][0] as u32));
c.mfc1(3, 0);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x10));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lqc2(3, (c.g[S3][0] as u32));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lwc1(0, (c.g[S5][0] as u32).wrapping_add(0x4));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.mfc1(3, 0);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lqc2(3, (c.g[S3][0] as u32));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x3c));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x4c));
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10d8));
c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x2e0));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x2e4));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x003783b8);
c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
call_ptr(c, t, 0x003783b8)?;
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lqc2(4, (c.g[SP][0] as u32));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x70));
c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10d8));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x2e0));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2e4));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x003783ec);
c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
call_ptr(c, t, 0x003783ec)?;
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lqc2(4, (c.g[SP][0] as u32));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
c.g[AT][0] = 0x41800000;
c.mtc1(1, 0);
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
c.g[V1][0] = 0x30000000;
c.g[AT][0] = 0x43000000;
c.mtc1(1, 12);
c.g[A2][0] = 0x800;
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lqc2(3, (c.g[SP][0] as u32));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x60));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lqc2(3, (c.g[SP][0] as u32));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x90));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x60));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lqc2(3, (c.g[SP][0] as u32));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x60));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
c.lqc2(3, (c.g[SP][0] as u32));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x60));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[S5][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5a00));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.lwc1(5, (c.g[SP][0] as u32).wrapping_add(0x70));
c.g[T3][0] = c.g[S5][0] | c.g[V1][0];
c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x74));
c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0x80));
c.mul_s(5, 5, 0);
c.lwc1(6, (c.g[SP][0] as u32).wrapping_add(0x84));
c.mul_s(3, 3, 0);
c.lwc1(7, (c.g[SP][0] as u32).wrapping_add(0x90));
c.mul_s(4, 4, 0);
c.lwc1(8, (c.g[SP][0] as u32).wrapping_add(0x94));
c.mul_s(6, 6, 0);
c.lwc1(9, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.mul_s(7, 7, 0);
c.lwc1(10, (c.g[SP][0] as u32).wrapping_add(0xa4));
c.mul_s(8, 8, 0);
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5a34));
c.mul_s(9, 9, 0);
c.mul_s(10, 10, 0);
c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5a30));
c.lwc1(11, (c.g[S4][0] as u32));
c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
c.lwc1(0, (c.g[S4][0] as u32).wrapping_add(0x4));
c.g[V1][0] = sx((c.g[V1][0] as u32) >> 1);
c.lwc1(1, (c.g[S4][0] as u32).wrapping_add(0x8));
c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
c.lwc1(2, (c.g[S4][0] as u32).wrapping_add(0xc));
c.mul_s(0, 0, 12);
c.mul_s(11, 11, 12);
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
c.mul_s(1, 1, 12);
c.g[A2][0] = sx((c.g[A2][0] as u32) << 4);
c.mul_s(2, 2, 12);
c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
c.cvt_w_s(12, 5);
c.mfc1(8, 12);
c.cvt_w_s(5, 3);
c.mfc1(3, 5);
c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0x8));
c.cvt_w_s(3, 4);
c.mfc1(4, 3);
c.cvt_w_s(3, 6);
c.mfc1(5, 3);
c.g[S3][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[T0][0] as u32)));
c.cvt_w_s(3, 7);
c.mfc1(7, 3);
c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[T7][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.cvt_w_s(3, 8);
c.mfc1(3, 3);
c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.cvt_w_s(3, 9);
c.mfc1(4, 3);
c.g[T6][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.cvt_w_s(3, 10);
c.mfc1(5, 3);
c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.cvt_w_s(3, 0);
c.mfc1(7, 3);
c.g[T5][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.cvt_w_s(0, 1);
c.mfc1(3, 0);
c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.cvt_w_s(0, 12);
c.mfc1(12, 0);
c.cvt_w_s(0, 2);
c.mfc1(2, 0);
c.cvt_w_s(0, 11);
c.mfc1(5, 0);
let t = (c.g[S6][0] as i64) <= 0;
c.g[V1][0] = c.g[V1][0];
if t {
break 'b_378724;
} else {
c.cycles += 40;
c.g[V1][0] = c.g[V1][0] << 8;
c.g[V0][0] = c.g[V0][0] << 16;
c.g[V1][0] = c.g[A3][0] | c.g[V1][0];
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.g[A1][0] = c.g[A1][0] << 24;
c.mfc1(2, 21);
c.g[A3][0] = c.g[T2][0];
c.g[A2][0] = c.g[V0][0];
c.g[A2][0] = c.g[A2][0] << 32;
c.g[T1][0] = c.g[T1][0] << 16;
c.g[V1][0] = c.g[V1][0] | c.g[A1][0];
c.g[V0][0] = c.g[T4][0] << 32;
c.g[T1][0] = c.g[T6][0] | c.g[T1][0];
c.g[A3][0] = c.g[A3][0] << 16;
c.g[T0][0] = c.g[T0][0] << 16;
c.g[A0][0] = c.g[A0][0] << 16;
c.g[A1][0] = c.g[V1][0] | c.g[A2][0];
c.g[T0][0] = c.g[T7][0] | c.g[T0][0];
c.g[A0][0] = c.g[T5][0] | c.g[A0][0];
c.g[A3][0] = c.g[S3][0] | c.g[A3][0];
c.g[A2][0] = c.g[T1][0] | c.g[V0][0];
c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
c.g[A3][0] = c.g[A3][0] | c.g[V0][0];
c.g[T0][0] = c.g[T0][0] | c.g[V0][0];
c.g[T2][0] = c.g[S6][0];
c.g[T7][0] = 0x10000000;
c.g[T7][0] = c.g[T7][0] | 0x7;
c.g[T6][0] = 0x50000000;
c.g[T6][0] = c.g[T6][0] | 0x6;
c.g[T6][0] = c.g[T6][0] << 32;
c.g[T5][0] = 0xffffffffa4000000;
c.g[T5][0] = c.g[T5][0] << 32;
c.g[T5][0] = c.g[T5][0] | 0x8001;
c.g[T4][0] = 0x52;
c.g[T4][0] = c.g[T4][0] << 16;
c.g[T4][0] = c.g[T4][0] | 0x5252;
c.g[T4][0] = c.g[T4][0] << 16;
c.g[T4][0] = c.g[T4][0] | 0x5210;
c.g[T1][0] = 0x54;
'l_378698: loop {
c.cycles += 35;
c.sd((c.g[T3][0] as u32), c.g[T7][0]);
c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_sub(0x1));
c.sd((c.g[T3][0] as u32).wrapping_add(0x8), 0u64);
c.sd((c.g[T3][0] as u32).wrapping_add(0x10), 0u64);
c.sd((c.g[T3][0] as u32).wrapping_add(0x18), c.g[T6][0]);
c.sd((c.g[T3][0] as u32).wrapping_add(0x20), c.g[T5][0]);
c.sd((c.g[T3][0] as u32).wrapping_add(0x28), c.g[T4][0]);
c.sd((c.g[T3][0] as u32).wrapping_add(0x30), c.g[T1][0]);
c.sd((c.g[T3][0] as u32).wrapping_add(0x38), c.g[A1][0]);
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
c.g[V1][0] = c.lw((c.g[S2][0] as u32));
c.g[V0][0] = c.g[V0][0] << 32;
c.sd((c.g[T3][0] as u32).wrapping_add(0x48), c.g[A3][0]);
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sd((c.g[T3][0] as u32).wrapping_add(0x40), c.g[V1][0]);
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
c.g[V1][0] = c.lw((c.g[S1][0] as u32));
c.g[V0][0] = c.g[V0][0] << 32;
c.sd((c.g[T3][0] as u32).wrapping_add(0x58), c.g[T0][0]);
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sd((c.g[T3][0] as u32).wrapping_add(0x50), c.g[V1][0]);
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
c.g[V1][0] = c.lw((c.g[S2][0] as u32));
c.g[V0][0] = c.g[V0][0] << 32;
c.sd((c.g[T3][0] as u32).wrapping_add(0x68), c.g[A2][0]);
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sd((c.g[T3][0] as u32).wrapping_add(0x60), c.g[V1][0]);
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
c.g[V1][0] = c.lw((c.g[S1][0] as u32));
c.g[V0][0] = c.g[V0][0] << 32;
c.sd((c.g[T3][0] as u32).wrapping_add(0x78), c.g[A0][0]);
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sd((c.g[T3][0] as u32).wrapping_add(0x70), c.g[V1][0]);
let t = c.g[T2][0] != 0u64;
c.g[T3][0] = sx((c.g[T3][0] as u32).wrapping_add(0x80));
if t {
if event_due(c) { deliver_event_here(c, 0x00378698); } continue 'l_378698;
} else {
break 'b_378724;
}
}
}
}
'b_3787cc: {
c.cycles += 10;
c.g[V0][0] = 0xfff0000;
c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18f0));
c.g[V0][0] = c.g[V0][0] | 0xffff;
c.g[T0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe84));
c.g[T1][0] = c.g[T3][0] & c.g[V0][0];
c.sw((c.g[S0][0] as u32).wrapping_add(0x5a00), c.g[T1][0]);
c.g[V0][0] = c.lw((c.g[A3][0] as u32));
c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[A1][0] = 0x60000;
if t {
break 'b_3787cc;
} else {
c.cycles += 32;
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
c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[S5][0]);
c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), 0u64);
break 'b_3787cc;
}
}
c.cycles += 15;
c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5a00));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x140));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x130));
c.sw((c.g[S0][0] as u32).wrapping_add(0x5a00), c.g[V0][0]);
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x150));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x120));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x110));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x100));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x168));
c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x160));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x170));
c.pc = t; return Ok(());
    });
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{check_reaching, machine, FAKE};
    use crate::difftest::{elf_image, DATA, GAME_GP};

    /// `DrawRotatedSprite`: matrices valid or stale (real UpdateViewMatrices),
    /// the three vtable matrix calls faked (some unwind), counts <= 0 and 1..5,
    /// defer ring below, near and at the 2600-entry limit.
    #[test]
    fn ps2_graphics_man_v79_matches() {
        check_reaching(
            ee::s12::f_003781a0__generated,
            ps2_graphics_man_v79,
            200,
            &[0x0037_8220, 0x0037_83b8, 0x0037_83ec],
            &|c, seed| {
                let mut r = machine(c, seed, 0x3781a0, 3);
                elf_image(c);
                let gm = DATA;
                let vt = DATA + 0x7000;
                let cam = DATA + 0x7800;
                let state = DATA + 0x7c00;
                let ring = DATA + 0x10000;
                c.pc = 0x0037_81a0;
                let sp0 = c.g[SP][0] as u32;
                for i in 0..0x5c {
                    let v = r.f32(100.0);
                    c.bus.write32(sp0 - 0x170 + 4 * i, v);
                }
                c.bus.write32(gm + 0x6b90, if seed % 8 == 0 { 0 } else { 1 });
                c.bus.write32(gm + 0x10d8, vt);
                c.bus.write16(vt + 0x2e0, (r.next_u32() % 64 * 4) as u16);
                c.bus.write32(vt + 0x2e4, FAKE);
                c.bus.write32(gm + 0x13e4, cam);
                for i in 0..16 {
                    let v = r.f32(2.0);
                    c.bus.write32(cam + 4 * i, v);
                }
                c.bus.write32(gm + 0x5a00, DATA + 0x80000);
                c.bus.write32(gm + 0x5a30, [640, 512, r.next_u32()][(seed % 3) as usize]);
                c.bus.write32(gm + 0x5a34, [448, 224, r.next_u32()][(seed / 3 % 3) as usize]);
                c.bus.write32(gm + 0x18f0, ring);
                c.bus.write32(gm + 0xe84, state);
                for i in 0..5 {
                    c.bus.write32(state + 4 * i, r.next_u32());
                }
                let count = [0, 2599, 2600, 100, 2700][(seed / 5 % 5) as usize];
                c.bus.write32(ring, count);
                c.bus.write32(ring + 0x68000 + 0x1cc4, r.next_u32());
                c.bus.write32(ring + 0x68000 + 0x1cc0, r.next_u32());
                let (pos, size, uv0, uv1, col) = (
                    DATA + 0x8000,
                    DATA + 0x8100,
                    DATA + 0x8200,
                    DATA + 0x8300,
                    DATA + 0x8400,
                );
                for i in 0..4 {
                    let a = if seed % 7 == 0 { r.edge_f32() } else { r.f32(50.0) };
                    c.bus.write32(pos + 4 * i, a);
                    let b = r.f32(10.0);
                    c.bus.write32(col + 4 * i, b);
                }
                c.bus.write32(size, r.f32(5.0));
                c.bus.write32(size + 4, r.f32(5.0));
                for i in 0..2 {
                    let (a, b) = (r.next_u32(), r.next_u32());
                    c.bus.write32(uv0 + 4 * i, a);
                    c.bus.write32(uv1 + 4 * i, b);
                }
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = sx(gm);
                c.g[A1][0] = sx(pos);
                c.g[A2][0] = sx(size);
                c.g[A3][0] = sx(uv0);
                c.g[T0][0] = sx(uv1);
                c.g[T1][0] = sx(col);
                c.g[T2][0] = match seed % 6 {
                    0 => 0,
                    1 => sx(0xffff_ffff),
                    n => n as u64 - 1,
                };
                c.f[12] = if seed % 5 == 0 { r.edge_f32() } else { r.f32(7.0) };
            },
        );
    }
}
