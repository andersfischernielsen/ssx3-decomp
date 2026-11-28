//! Staging module oc1465 (wave wave274): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1465`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0037_9028, ps2_graphics_man_v81, "DrawGlyphsW")];

/// `PS2GraphicsMan_DrawGlyphsW(self, font, wchar* text, float* color, f12 x, f13 y,
/// f14 sx, f15 sy, f16 su, f17 sv, f18 z)` (0x00379028, slot `DrawGlyphsW`): lay out
/// a wide string with the font's glyph table into a sprite-stream block (flat
/// screen-space quads when the viewport projection is non-zero, VU0-projected
/// quads otherwise), then queue the draw in the deferred ring. Converted from
/// the generated body; every register, cycle and call kept.
pub fn ps2_graphics_man_v81(c: &mut Cpu) {
    guest(c, |c| {
'b_37909c: {
c.cycles += 27;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x140));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.g[S0][0] = c.g[A2][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.g[S1][0] = c.g[A1][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[S2][0] = c.g[A0][0];
c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0x130));
c.g[S3][0] = c.g[A3][0];
c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x128));
c.mov_s(26, 13);
c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x120));
c.mov_s(25, 18);
c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x118));
c.mov_s(24, 15);
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x110));
c.mov_s(23, 16);
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x108));
c.mov_s(22, 17);
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x100));
c.mov_s(21, 12);
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[RA][0]);
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6b90));
let t = c.g[V0][0] != 0u64;
c.mov_s(20, 14);
if t {
break 'b_37909c;
} else {
c.cycles += 2;
c.g[RA][0] = sx(0x0037909c);
Callee { addr: 0x0037d968, f: ee::s12::PS2GraphicsMan_UpdateViewMatrices }.call(c, 0x0037909c)?;
break 'b_37909c;
}
}
'b_379684: {
'b_379680: {
'b_379360: {
c.cycles += 15;
c.g[S4][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5a00));
c.g[A2][0] = 0xffffffffffffffff;
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
c.g[A3][0] = 0u64;
c.g[RA][0] = sx(0x003790b4);
c.g[A1][0] = c.g[S4][0];
Callee { addr: 0x0038f460, f: ee::s12::SprStream_BeginBlock }.call(c, 0x003790b4)?;
c.g[T3][0] = c.g[V0][0];
c.g[V1][0] = 0x230;
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10dc));
c.g[T0][0] = sx((c.g[T3][0] as u32).wrapping_add(0x50));
c.mult(0, 4, 2, 3);
c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x6d40));
let t = c.g[V1][0] == 0u64;
c.g[T4][0] = 0u64;
if t {
break 'b_379360;
} else {
c.cycles += 13;
c.lwc1(0, (c.g[S2][0] as u32).wrapping_add(0x6b18));
c.lwc1(2, (c.g[S2][0] as u32).wrapping_add(0x6b28));
c.mul_s(0, 0, 25);
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 1);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x1c));
c.add_s(0, 0, 2);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xc));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x18));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.lhu((c.g[S0][0] as u32));
let t = c.g[V0][0] == 0u64;
c.g[T5][0] = 0xc;
if t {
break 'b_379680;
} else {
c.cycles += 1;
c.g[V1][0] = c.lhu((c.g[S0][0] as u32));
'l_379110: loop {
'b_3791a0: {
'b_379128: {
c.cycles += 4;
c.g[T2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[T2][0] as i64)) as u64;
let t = c.g[V0][0] == 0u64;
c.g[T1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2));
if t {
break 'b_379128;
} else {
c.cycles += 2;
let t = true;
c.g[T2][0] = 0u64;
if t {
break 'b_3791a0;
} else {
break 'b_379128;
}
}
}
'b_379154: {
c.cycles += 4;
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
if c.g[V0][0] != 0u64 {
c.g[A2][0] = c.lw((c.g[S1][0] as u32));
break 'b_379154;
} else {
c.cycles += 7;
c.mult(0, 3, 3, 13);
c.mult(0, 10, 10, 13);
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
c.g[T1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
let t = true;
c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[T2][0] as u32)));
if t {
break 'b_3791a0;
} else {
break 'b_379154;
}
}
}
c.cycles += 5;
c.g[T2][0] = sx((c.g[A0][0] as u32).wrapping_sub((c.g[T2][0] as u32)));
c.g[T6][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
c.g[T1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2));
c.g[A3][0] = 0xc;
c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[A2][0] as u32)));
'l_379168: loop {
c.cycles += 6;
c.g[A1][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
c.mult(0, 4, 5, 7);
c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T6][0] as u32)));
c.g[A0][0] = c.lhu((c.g[V0][0] as u32));
if c.g[V1][0] == c.g[A0][0] {
c.g[T2][0] = c.g[V0][0];
break 'b_3791a0;
} else {
'b_379190: {
c.cycles += 3;
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
if c.g[V0][0] == 0u64 {
c.g[T2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
break 'b_379190;
} else {
c.cycles += 1;
c.g[A2][0] = c.g[A1][0];
break 'b_379190;
}
}
c.cycles += 3;
c.g[V0][0] = ((c.g[T2][0] as i64) < (c.g[A2][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[A2][0] as u32)));
if t {
if event_due(c) { deliver_event_here(c, 0x00379168); } continue 'l_379168;
} else {
c.cycles += 1;
c.g[T2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
break 'b_3791a0;
}
}
}
}
'b_37934c: {
c.cycles += 2;
let t = c.g[T2][0] == 0u64;
c.g[S0][0] = c.g[T1][0];
if t {
break 'b_37934c;
} else {
c.cycles += 105;
c.g[V1][0] = c.lhu((c.g[T2][0] as u32).wrapping_add(0x6));
c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x1));
c.g[V0][0] = c.lhu((c.g[T2][0] as u32).wrapping_add(0x4));
c.mtc1(3, 1);
c.cvt_s_w(1, 1);
c.lwc1(3, (c.g[S2][0] as u32).wrapping_add(0x6af0));
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.lwc1(5, (c.g[S2][0] as u32).wrapping_add(0x6b04));
c.lwc1(4, (c.g[S2][0] as u32).wrapping_add(0x6b20));
c.mul_s(1, 1, 22);
c.lwc1(6, (c.g[S2][0] as u32).wrapping_add(0x6b24));
c.mul_s(0, 0, 23);
c.mfc1(7, 1);
c.mfc1(5, 0);
c.g[A2][0] = c.g[A3][0];
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x24));
c.g[V0][0] = c.g[A1][0];
c.g[A2][0] = c.g[A2][0] << 32;
c.g[A2][0] = c.g[V0][0] | c.g[A2][0];
c.g[A1][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(0x3));
c.g[A0][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(0x2));
c.g[V1][0] = c.lhu((c.g[T2][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.lhu((c.g[T2][0] as u32).wrapping_add(0x6));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.mtc1(3, 1);
c.cvt_s_w(1, 1);
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.mul_s(1, 1, 23);
c.mul_s(0, 0, 22);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x30));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x34));
c.g[V0][0] = c.lb((c.g[T2][0] as u32).wrapping_add(0x9));
c.mtc1(2, 2);
c.cvt_s_w(2, 2);
c.mul_s(2, 2, 20);
c.add_s(2, 21, 2);
c.mul_s(2, 2, 3);
c.add_s(2, 2, 4);
c.swc1(2, (c.g[SP][0] as u32));
c.g[V0][0] = c.lb((c.g[T2][0] as u32).wrapping_add(0xa));
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
c.mul_s(1, 1, 24);
c.add_s(1, 26, 1);
c.mul_s(1, 1, 5);
c.add_s(1, 1, 6);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(0x2));
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.mul_s(0, 0, 20);
c.mul_s(0, 0, 3);
c.add_s(2, 2, 0);
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.g[V0][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(0x3));
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.mul_s(0, 0, 24);
c.mul_s(0, 0, 5);
c.add_s(1, 1, 0);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
c.sd((c.g[T0][0] as u32), c.g[A2][0]);
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
c.lwc1(1, (c.g[SP][0] as u32));
c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x8));
c.cvt_w_s(3, 0);
c.mfc1(2, 3);
c.cvt_w_s(0, 1);
c.mfc1(4, 0);
c.cvt_w_s(0, 2);
c.mfc1(3, 0);
c.g[V0][0] = c.g[V0][0] << 16;
c.g[V0][0] = c.g[A0][0] | c.g[V0][0];
c.g[V1][0] = c.g[V1][0] << 32;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x34));
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
c.g[V1][0] = c.g[V1][0] << 32;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
c.cvt_w_s(3, 0);
c.mfc1(2, 3);
c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x18));
c.cvt_w_s(0, 1);
c.mfc1(4, 0);
c.cvt_w_s(0, 2);
c.mfc1(3, 0);
c.g[V0][0] = c.g[V0][0] << 16;
c.g[V1][0] = c.g[V1][0] << 32;
c.g[V0][0] = c.g[A0][0] | c.g[V0][0];
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
c.g[V1][0] = c.lb((c.g[T2][0] as u32).wrapping_add(0x8));
c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
c.mtc1(3, 0);
c.cvt_s_w(0, 0);
c.mul_s(0, 0, 20);
c.add_s(21, 21, 0);
break 'b_37934c;
}
}
c.cycles += 3;
c.g[V0][0] = c.lhu((c.g[S0][0] as u32));
if c.g[V0][0] != 0u64 {
c.g[V1][0] = c.lhu((c.g[S0][0] as u32));
if event_due(c) { deliver_event_here(c, 0x00379110); } continue 'l_379110;
} else {
c.cycles += 2;
let t = true;
c.g[V1][0] = sx((c.g[T4][0] as u32) << 1);
if t {
break 'b_379684;
} else {
break 'b_379360;
}
}
}
}
}
}
c.cycles += 3;
c.g[V0][0] = c.lhu((c.g[S0][0] as u32));
let t = c.g[V0][0] == 0u64;
c.g[T5][0] = 0xc;
if t {
break 'b_379680;
} else {
c.cycles += 7;
c.g[AT][0] = 0x3f000000;
c.mtc1(1, 7);
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 6);
c.g[T6][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6b30));
c.g[V1][0] = c.lhu((c.g[S0][0] as u32));
'l_379388: loop {
'b_379418: {
'b_3793a0: {
c.cycles += 4;
c.g[T2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[T2][0] as i64)) as u64;
let t = c.g[V0][0] == 0u64;
c.g[T1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2));
if t {
break 'b_3793a0;
} else {
c.cycles += 2;
let t = true;
c.g[T2][0] = 0u64;
if t {
break 'b_379418;
} else {
break 'b_3793a0;
}
}
}
'b_3793cc: {
c.cycles += 4;
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
if c.g[V0][0] != 0u64 {
c.g[A2][0] = c.lw((c.g[S1][0] as u32));
break 'b_3793cc;
} else {
c.cycles += 7;
c.mult(0, 3, 3, 13);
c.mult(0, 10, 10, 13);
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
c.g[T1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
let t = true;
c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[T2][0] as u32)));
if t {
break 'b_379418;
} else {
break 'b_3793cc;
}
}
}
c.cycles += 5;
c.g[T2][0] = sx((c.g[A0][0] as u32).wrapping_sub((c.g[T2][0] as u32)));
c.g[T7][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
c.g[T1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2));
c.g[A3][0] = 0xc;
c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[A2][0] as u32)));
'l_3793e0: loop {
c.cycles += 6;
c.g[A1][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
c.mult(0, 4, 5, 7);
c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T7][0] as u32)));
c.g[A0][0] = c.lhu((c.g[V0][0] as u32));
if c.g[V1][0] == c.g[A0][0] {
c.g[T2][0] = c.g[V0][0];
break 'b_379418;
} else {
'b_379408: {
c.cycles += 3;
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
if c.g[V0][0] == 0u64 {
c.g[T2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
break 'b_379408;
} else {
c.cycles += 1;
c.g[A2][0] = c.g[A1][0];
break 'b_379408;
}
}
c.cycles += 3;
c.g[V0][0] = ((c.g[T2][0] as i64) < (c.g[A2][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[A2][0] as u32)));
if t {
if event_due(c) { deliver_event_here(c, 0x003793e0); } continue 'l_3793e0;
} else {
c.cycles += 1;
c.g[T2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
break 'b_379418;
}
}
}
}
'b_379674: {
c.cycles += 2;
let t = c.g[T2][0] == 0u64;
c.g[S0][0] = c.g[T1][0];
if t {
break 'b_379674;
} else {
c.cycles += 149;
c.g[V1][0] = c.lhu((c.g[T2][0] as u32).wrapping_add(0x4));
c.g[T4][0] = sx((c.g[T4][0] as u32).wrapping_add(0x1));
c.g[V0][0] = c.lhu((c.g[T2][0] as u32).wrapping_add(0x6));
c.mtc1(3, 0);
c.cvt_s_w(0, 0);
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
c.mul_s(0, 0, 23);
c.mul_s(1, 1, 22);
c.swc1(0, (c.g[SP][0] as u32));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(0x2));
c.g[A1][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(0x3));
c.g[V0][0] = c.lhu((c.g[T2][0] as u32).wrapping_add(0x4));
c.g[V1][0] = c.lhu((c.g[T2][0] as u32).wrapping_add(0x6));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
c.mtc1(3, 0);
c.cvt_s_w(0, 0);
c.mul_s(1, 1, 23);
c.mul_s(0, 0, 22);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x14));
c.g[V0][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(0x2));
c.g[V1][0] = c.lbu((c.g[T2][0] as u32).wrapping_add(0x3));
c.mtc1(2, 2);
c.cvt_s_w(2, 2);
c.mtc1(3, 3);
c.cvt_s_w(3, 3);
c.mul_s(2, 2, 20);
c.mul_s(3, 3, 24);
c.mul_s(2, 2, 7);
c.mul_s(3, 3, 7);
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x24));
c.g[V0][0] = c.lb((c.g[T2][0] as u32).wrapping_add(0x9));
c.g[V1][0] = c.lb((c.g[T2][0] as u32).wrapping_add(0xa));
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x48));
c.mtc1(3, 0);
c.cvt_s_w(0, 0);
c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x4c));
c.mul_s(1, 1, 20);
c.mul_s(0, 0, 24);
c.add_s(1, 21, 1);
c.add_s(0, 26, 0);
c.add_s(1, 1, 2);
c.add_s(0, 0, 3);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x40));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x44));
c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lqc2(4, (c.g[T6][0] as u32));
c.lqc2(5, (c.g[T6][0] as u32).wrapping_add(0x10));
c.lqc2(6, (c.g[T6][0] as u32).wrapping_add(0x20));
c.lqc2(7, (c.g[T6][0] as u32).wrapping_add(0x30));
c.vu0().mula(0b1111, 4, Src::Bc(8, X));
c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lwc1(5, (c.g[S2][0] as u32).wrapping_add(0x6af0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x5c));
c.mul_s(5, 5, 1);
c.lwc1(4, (c.g[S2][0] as u32).wrapping_add(0x6b04));
c.div_s(0, 6, 0);
c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x24));
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x54));
c.mul_s(4, 4, 3);
c.mul_s(1, 1, 0);
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x5c));
c.mul_s(2, 2, 0);
c.mul_s(5, 5, 0);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x50));
c.mul_s(4, 4, 0);
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x54));
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x60));
c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x64));
c.sw((c.g[SP][0] as u32).wrapping_add(0x68), 0u64);
c.sw((c.g[SP][0] as u32).wrapping_add(0x6c), 0u64);
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x60));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x60));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
c.lq(4, (c.g[SP][0] as u32).wrapping_add(0x90));
c.mfc1(5, 0);
c.g[V1][0] = c.lw((c.g[SP][0] as u32));
c.g[V0][0] = c.g[A1][0];
c.sq(4, (c.g[SP][0] as u32).wrapping_add(0x80));
c.g[V0][0] = c.g[V0][0] << 32;
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sd((c.g[T0][0] as u32), c.g[V1][0]);
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x7c));
c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x78));
c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x74));
c.mul_s(2, 2, 0);
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x70));
c.cvt_w_s(0, 3);
c.mfc1(2, 0);
c.cvt_w_s(0, 1);
c.mfc1(4, 0);
c.cvt_w_s(0, 2);
c.mfc1(3, 0);
c.g[V0][0] = c.g[V0][0] << 16;
c.g[V0][0] = c.g[A0][0] | c.g[V0][0];
c.g[V1][0] = c.g[V1][0] << 32;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sd((c.g[T0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[V1][0] = c.g[V1][0] << 32;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sd((c.g[T0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x8c));
c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x88));
c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x84));
c.mul_s(2, 2, 0);
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x80));
c.cvt_w_s(0, 3);
c.mfc1(2, 0);
c.cvt_w_s(0, 1);
c.mfc1(4, 0);
c.cvt_w_s(0, 2);
c.mfc1(3, 0);
c.g[V0][0] = c.g[V0][0] << 16;
c.g[V0][0] = c.g[A0][0] | c.g[V0][0];
c.g[V1][0] = c.g[V1][0] << 32;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sd((c.g[T0][0] as u32).wrapping_add(0x18), c.g[V0][0]);
c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x20));
c.g[V0][0] = c.lb((c.g[T2][0] as u32).wrapping_add(0x8));
c.mtc1(2, 0);
c.cvt_s_w(0, 0);
c.mul_s(0, 0, 20);
c.add_s(21, 21, 0);
c.g[S0][0] = c.g[T1][0];
break 'b_379674;
}
}
c.cycles += 3;
c.g[V0][0] = c.lhu((c.g[S0][0] as u32));
if c.g[V0][0] != 0u64 {
c.g[V1][0] = c.lhu((c.g[S0][0] as u32));
if event_due(c) { deliver_event_here(c, 0x00379388); } continue 'l_379388;
} else {
break 'b_379680;
}
}
}
}
c.cycles += 1;
c.g[V1][0] = sx((c.g[T4][0] as u32) << 1);
break 'b_379684;
}
'b_379824: {
c.cycles += 72;
c.g[A1][0] = 0x50000000;
c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3));
c.g[V0][0] = 0x10000000;
c.g[A0][0] = c.g[A0][0] | c.g[A1][0];
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.g[A0][0] = c.g[A0][0] << 32;
c.g[A1][0] = 0x24000000;
c.g[A1][0] = c.g[A1][0] << 32;
c.g[A1][0] = c.g[A1][0] | 0x1;
c.g[A2][0] = 0x10;
c.g[A3][0] = 0x56;
c.sd((c.g[T3][0] as u32), c.g[V1][0]);
c.sd((c.g[T3][0] as u32).wrapping_add(0x18), c.g[A0][0]);
c.g[V0][0] = c.g[T4][0] | 0x8000;
c.sd((c.g[T3][0] as u32).wrapping_add(0x20), c.g[A1][0]);
c.g[V1][0] = 0x5252;
c.sd((c.g[T3][0] as u32).wrapping_add(0x28), c.g[A2][0]);
c.g[T1][0] = 0x8800;
c.g[T1][0] = c.g[T1][0] << 47;
c.sd((c.g[T3][0] as u32).wrapping_add(0x30), c.g[A3][0]);
c.g[T1][0] = c.g[V0][0] | c.g[T1][0];
c.sd((c.g[T3][0] as u32).wrapping_add(0x8), 0u64);
c.g[A1][0] = c.g[T0][0];
c.sd((c.g[T3][0] as u32).wrapping_add(0x10), 0u64);
c.g[A2][0] = 0x4;
c.g[AT][0] = 0x434c0000;
c.mtc1(1, 0);
c.lwc1(1, (c.g[S3][0] as u32).wrapping_add(0x8));
c.lwc1(3, (c.g[S3][0] as u32).wrapping_add(0xc));
c.mul_s(1, 1, 0);
c.lwc1(2, (c.g[S3][0] as u32).wrapping_add(0x4));
c.lwc1(5, (c.g[S3][0] as u32));
c.mul_s(3, 3, 0);
c.mul_s(2, 2, 0);
c.sd((c.g[T3][0] as u32).wrapping_add(0x48), c.g[V1][0]);
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 6);
c.cvt_w_s(0, 1);
c.mfc1(2, 0);
c.g[AT][0] = 0x43000000;
c.mtc1(1, 4);
c.mfc1(3, 6);
c.mul_s(5, 5, 4);
c.sd((c.g[T3][0] as u32).wrapping_add(0x40), c.g[T1][0]);
c.g[A3][0] = c.g[V1][0];
c.cvt_w_s(0, 2);
c.mfc1(8, 0);
c.g[V0][0] = c.g[V0][0] << 8;
c.cvt_w_s(0, 3);
c.mfc1(4, 0);
c.g[A3][0] = c.g[A3][0] << 32;
c.cvt_w_s(0, 5);
c.mfc1(3, 0);
c.g[V0][0] = c.g[T0][0] | c.g[V0][0];
c.g[A0][0] = c.g[A0][0] << 16;
c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
c.g[V1][0] = c.g[V1][0] << 24;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
c.sd((c.g[T3][0] as u32).wrapping_add(0x38), c.g[V0][0]);
c.g[RA][0] = sx(0x00379780);
c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x1384));
Callee { addr: 0x0038f668, f: ee::s12::SprStream_EndBlockAligned }.call(c, 0x00379780)?;
c.g[T1][0] = c.g[V0][0];
c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18f0));
c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x10));
c.g[T0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xe84));
c.sw((c.g[S2][0] as u32).wrapping_add(0x5a00), c.g[V1][0]);
c.g[V0][0] = c.lw((c.g[A3][0] as u32));
c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[A1][0] = 0x60000;
if t {
break 'b_379824;
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
c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[S4][0]);
c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), 0u64);
break 'b_379824;
}
}
c.cycles += 15;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
c.lwc1(26, (c.g[SP][0] as u32).wrapping_add(0x130));
c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x128));
c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x120));
c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x118));
c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x110));
c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x108));
c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x100));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
c.pc = t; return Ok(());
    });
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fill, scramble, Rng, DATA, GAME_GP};

    const OC1465_SELF: u32 = DATA;
    const OC1465_FONT: u32 = DATA + 0x8000;
    const OC1465_TABLE: u32 = DATA + 0x8100;
    const OC1465_FALLBACK: u32 = DATA + 0x8200;
    const OC1465_TEXT: u32 = DATA + 0x8300;
    const OC1465_COLOR: u32 = DATA + 0x8400;
    const OC1465_RS: u32 = DATA + 0xb000;
    const OC1465_STREAM: u32 = DATA + 0x1_0000;
    const OC1465_RING: u32 = DATA + 0x20_0000;

    /// A float in [lo, hi) as bits (fixture values only).
    fn oc1465_fl(r: &mut Rng, lo: f32, hi: f32) -> u32 {
        let u = (r.next_u32() % 10000) as f32 / 10000.0;
        (lo + u * (hi - lo)).to_bits()
    }

    /// A 12-byte font glyph: char, two byte offsets, u/v, three signed bytes.
    fn oc1465_glyph(c: &mut Cpu, r: &mut Rng, at: u32, ch: u32) {
        c.bus.write16(at, ch as u16);
        c.bus.write8(at + 2, (r.next_u32() % 16) as u8);
        c.bus.write8(at + 3, (r.next_u32() % 16) as u8);
        c.bus.write16(at + 4, (r.next_u32() % 256) as u16);
        c.bus.write16(at + 6, (r.next_u32() % 256) as u16);
        for k in 8..11 {
            c.bus.write8(at + k, (r.next_u32() % 40) as u8);
        }
    }

    /// `PS2GraphicsMan_DrawGlyphsW`: both projection modes (flat / VU0), glyphs
    /// found directly, by binary search, through the fallback or not at all,
    /// empty strings, the matrix-update call (some seeds), and a full or
    /// free deferred ring.
    #[test]
    fn ps2_graphics_man_v81_matches() {
        check(
            ee::s12::f_00379028__generated,
            ps2_graphics_man_v81,
            300,
            &|c, seed| {
                let mut r = Rng(0x0037_9028 ^ (seed as u64) << 13);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                // The sprite stream the real callees work on.
                c.bus.write32(GAME_GP + 0x1384, OC1465_STREAM);
                let slot = r.next_u32() % 2;
                c.bus.write32(OC1465_STREAM + 4, 2);
                c.bus.write32(OC1465_STREAM + 8, slot);
                c.bus.write32(OC1465_STREAM + 0xc, slot);
                c.bus.write32(OC1465_STREAM + 0x10, 0x101);
                c.bus.write32(OC1465_STREAM + 0x14, DATA + 0x1_1000);
                c.bus.write32(OC1465_STREAM + 0x18, DATA + 0x1_3000);
                fill(c, &mut r, DATA + 0x1_1000, 0x400);
                fill(c, &mut r, DATA + 0x1_3000, 0x400);
                // The manager: floats, matrix, viewport stack, ring, state.
                let gm = OC1465_SELF;
                fill(c, &mut r, gm + 0x6ae0, 0xa0);
                for off in [0x6af0, 0x6b04, 0x6b18, 0x6b20, 0x6b24, 0x6b28] {
                    let v = oc1465_fl(&mut r, 0.3, 2.0);
                    c.bus.write32(gm + off, v);
                }
                for k in 0..16 {
                    let v = oc1465_fl(&mut r, -2.0, 2.0);
                    c.bus.write32(gm + 0x6b30 + 4 * k, v);
                }
                c.bus.write32(gm + 0x6b90, if seed % 10 == 0 { 0 } else { 1 });
                c.bus.write32(gm + 0x5a00, DATA + 0x1_8000 + (r.next_u32() & 0xff0));
                let depth = (seed / 2) % 2;
                c.bus.write32(gm + 0x10dc, depth);
                let proj = if seed % 2 == 0 { 0 } else { 1 + r.next_u32() % 3 };
                c.bus.write32(gm + 0x6d40 + 0x230 * depth, proj);
                c.bus.write32(gm + 0x6d40 + 0x230 * (1 - depth), r.next_u32());
                c.bus.write32(gm + 0xe84, OC1465_RS);
                fill(c, &mut r, OC1465_RS, 0x20);
                c.bus.write32(gm + 0x18f0, OC1465_RING);
                let count = if seed % 5 == 0 {
                    2600 + r.next_u32() % 20
                } else {
                    r.next_u32() % 40
                };
                c.bus.write32(OC1465_RING, count);
                c.bus.write32(OC1465_RING + 0x69cc0, r.next_u32());
                c.bus.write32(OC1465_RING + 0x69cc4, r.next_u32());
                // The font: direct range 0x20..=0x27, then sorted 0x100.. entries.
                c.bus.write32(OC1465_FONT, 16);
                c.bus.write32(OC1465_FONT + 4, OC1465_TABLE);
                c.bus
                    .write32(OC1465_FONT + 8, if seed % 3 == 0 { 0 } else { OC1465_FALLBACK });
                c.bus.write32(OC1465_FONT + 0xc, 0x20);
                c.bus.write32(OC1465_FONT + 0x10, 0x27);
                for k in 0..16u32 {
                    let ch = if k < 8 { 0x20 + k } else { 0x100 + 0x10 * (k - 8) };
                    oc1465_glyph(c, &mut r, OC1465_TABLE + 12 * k, ch);
                }
                oc1465_glyph(c, &mut r, OC1465_FALLBACK, 0x3f);
                // The text: none, or up to eight characters of every kind.
                let n = if seed % 7 == 0 { 0 } else { 1 + r.next_u32() % 8 };
                for k in 0..n {
                    let ch = match r.next_u32() % 4 {
                        0 => 0x10,
                        1 => 0x20 + r.next_u32() % 8,
                        2 => 0x100 + 0x10 * (r.next_u32() % 8),
                        _ => 0x1ff,
                    };
                    c.bus.write16(OC1465_TEXT + 2 * k, ch as u16);
                }
                c.bus.write16(OC1465_TEXT + 2 * n, 0);
                for k in 0..4 {
                    let v = oc1465_fl(&mut r, 0.0, 1.0);
                    c.bus.write32(OC1465_COLOR + 4 * k, v);
                }
                c.g[A0][0] = sx(gm);
                c.g[A1][0] = sx(OC1465_FONT);
                c.g[A2][0] = sx(OC1465_TEXT);
                c.g[A3][0] = sx(OC1465_COLOR);
                let args = [
                    (12, 0.0, 640.0),
                    (13, 0.0, 448.0),
                    (14, 0.5, 2.0),
                    (15, 0.5, 2.0),
                    (16, 0.001, 0.01),
                    (17, 0.001, 0.01),
                    (18, 0.0, 1.0),
                ];
                for (reg, lo, hi) in args {
                    c.f[reg] = oc1465_fl(&mut r, lo, hi);
                }
            },
        );
    }
}
