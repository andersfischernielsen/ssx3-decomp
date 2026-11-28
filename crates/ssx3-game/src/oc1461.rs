//! Staging module oc1461 (wave wave274): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1461`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0037_8808, draw_glyphs, "DrawGlyphs")];

/// `PS2GraphicsMan_v80` / DrawGlyphs (0x00378808): draws a text string as glyph sprites (cFont binary search / direct table lookup, 2 packet layouts), the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn draw_glyphs(c: &mut Cpu) {
    if c.pc != 0x00378808 {
        return ee::s12::f_00378808__generated(c);
    }
    guest(c, |c| {
'b_378878: {
c.cycles += 26;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x130));
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.g[S0][0] = c.g[A2][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[S1][0] = c.g[A1][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[S2][0] = c.g[A0][0];
c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0x120));
c.g[S3][0] = c.g[A3][0];
c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x118));
c.mov_s(26, 13);
c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x110));
c.mov_s(25, 18);
c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x108));
c.mov_s(24, 15);
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x100));
c.mov_s(23, 16);
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xf8));
c.mov_s(22, 17);
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.mov_s(21, 12);
c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[RA][0]);
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6b90));
let t = c.g[V0][0] != 0u64;
c.mov_s(20, 14);
if t {
break 'b_378878;
} else {
c.cycles += 2;
c.g[RA][0] = sx(0x00378878);
Callee { addr: 0x0037d968, f: ee::s12::PS2GraphicsMan_UpdateViewMatrices }.call(c, 0x00378878)?;
break 'b_378878;
}
}
'b_378e54: {
'b_378b30: {
c.cycles += 11;
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10dc));
c.g[V0][0] = 0x230;
c.g[T7][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5a00));
c.g[A0][0] = 0x30000000;
c.mult(0, 5, 3, 2);
c.g[T4][0] = 0u64;
c.g[T3][0] = c.g[T7][0] | c.g[A0][0];
c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[S2][0] as u32)));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x6d40));
let t = c.g[V0][0] == 0u64;
c.g[T0][0] = sx((c.g[T3][0] as u32).wrapping_add(0x50));
if t {
break 'b_378b30;
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
c.g[V0][0] = c.lb((c.g[S0][0] as u32));
let t = c.g[V0][0] == 0u64;
c.g[V1][0] = sx((c.g[T4][0] as u32) << 1);
if t {
break 'b_378e54;
} else {
c.cycles += 2;
c.g[T5][0] = 0xc;
c.g[V1][0] = c.lbu((c.g[S0][0] as u32));
'l_3788e0: loop {
'b_378970: {
'b_3788f8: {
c.cycles += 4;
c.g[T2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[T2][0] as i64)) as u64;
let t = c.g[V0][0] == 0u64;
c.g[T1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
if t {
break 'b_3788f8;
} else {
c.cycles += 2;
let t = true;
c.g[T2][0] = 0u64;
if t {
break 'b_378970;
} else {
break 'b_3788f8;
}
}
}
'b_378924: {
c.cycles += 4;
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
if c.g[V0][0] != 0u64 {
c.g[A2][0] = c.lw((c.g[S1][0] as u32));
break 'b_378924;
} else {
c.cycles += 7;
c.mult(0, 3, 3, 13);
c.mult(0, 10, 10, 13);
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
c.g[T1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
let t = true;
c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[T2][0] as u32)));
if t {
break 'b_378970;
} else {
break 'b_378924;
}
}
}
c.cycles += 5;
c.g[T2][0] = sx((c.g[A0][0] as u32).wrapping_sub((c.g[T2][0] as u32)));
c.g[T6][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
c.g[T1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
c.g[A3][0] = 0xc;
c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[A2][0] as u32)));
'l_378938: loop {
c.cycles += 6;
c.g[A1][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
c.mult(0, 4, 5, 7);
c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T6][0] as u32)));
c.g[A0][0] = c.lhu((c.g[V0][0] as u32));
if c.g[V1][0] == c.g[A0][0] {
c.g[T2][0] = c.g[V0][0];
break 'b_378970;
} else {
'b_378960: {
c.cycles += 3;
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
if c.g[V0][0] == 0u64 {
c.g[T2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
break 'b_378960;
} else {
c.cycles += 1;
c.g[A2][0] = c.g[A1][0];
break 'b_378960;
}
}
c.cycles += 3;
c.g[V0][0] = ((c.g[T2][0] as i64) < (c.g[A2][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[A2][0] as u32)));
if t {
if event_due(c) { deliver_event_here(c, 0x00378938); } continue 'l_378938;
} else {
c.cycles += 1;
c.g[T2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
break 'b_378970;
}
}
}
}
'b_378b1c: {
c.cycles += 2;
let t = c.g[T2][0] == 0u64;
c.g[S0][0] = c.g[T1][0];
if t {
break 'b_378b1c;
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
break 'b_378b1c;
}
}
c.cycles += 3;
c.g[V0][0] = c.lb((c.g[S0][0] as u32));
if c.g[V0][0] != 0u64 {
c.g[V1][0] = c.lbu((c.g[S0][0] as u32));
if event_due(c) { deliver_event_here(c, 0x003788e0); } continue 'l_3788e0;
} else {
c.cycles += 2;
let t = true;
c.g[V1][0] = sx((c.g[T4][0] as u32) << 1);
if t {
break 'b_378e54;
} else {
break 'b_378b30;
}
}
}
}
}
}
c.cycles += 3;
c.g[V0][0] = c.lb((c.g[S0][0] as u32));
let t = c.g[V0][0] == 0u64;
c.g[V1][0] = sx((c.g[T4][0] as u32) << 1);
if t {
break 'b_378e54;
} else {
c.cycles += 7;
c.g[AT][0] = 0x3f000000;
c.mtc1(1, 7);
c.g[T5][0] = 0xc;
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 6);
c.g[T6][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6b30));
c.g[V1][0] = c.lbu((c.g[S0][0] as u32));
'l_378b58: loop {
'b_378be8: {
'b_378b70: {
c.cycles += 4;
c.g[T2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[T2][0] as i64)) as u64;
let t = c.g[V0][0] == 0u64;
c.g[T1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
if t {
break 'b_378b70;
} else {
c.cycles += 2;
let t = true;
c.g[T2][0] = 0u64;
if t {
break 'b_378be8;
} else {
break 'b_378b70;
}
}
}
'b_378b9c: {
c.cycles += 4;
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
if c.g[V0][0] != 0u64 {
c.g[A2][0] = c.lw((c.g[S1][0] as u32));
break 'b_378b9c;
} else {
c.cycles += 7;
c.mult(0, 3, 3, 13);
c.mult(0, 10, 10, 13);
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
c.g[T1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
let t = true;
c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[T2][0] as u32)));
if t {
break 'b_378be8;
} else {
break 'b_378b9c;
}
}
}
c.cycles += 5;
c.g[T2][0] = sx((c.g[A0][0] as u32).wrapping_sub((c.g[T2][0] as u32)));
c.g[T8][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
c.g[T1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
c.g[A3][0] = 0xc;
c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[A2][0] as u32)));
'l_378bb0: loop {
c.cycles += 6;
c.g[A1][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
c.mult(0, 4, 5, 7);
c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T8][0] as u32)));
c.g[A0][0] = c.lhu((c.g[V0][0] as u32));
if c.g[V1][0] == c.g[A0][0] {
c.g[T2][0] = c.g[V0][0];
break 'b_378be8;
} else {
'b_378bd8: {
c.cycles += 3;
c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
if c.g[V0][0] == 0u64 {
c.g[T2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
break 'b_378bd8;
} else {
c.cycles += 1;
c.g[A2][0] = c.g[A1][0];
break 'b_378bd8;
}
}
c.cycles += 3;
c.g[V0][0] = ((c.g[T2][0] as i64) < (c.g[A2][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[V0][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[A2][0] as u32)));
if t {
if event_due(c) { deliver_event_here(c, 0x00378bb0); } continue 'l_378bb0;
} else {
c.cycles += 1;
c.g[T2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
break 'b_378be8;
}
}
}
}
'b_378e44: {
c.cycles += 2;
let t = c.g[T2][0] == 0u64;
c.g[S0][0] = c.g[T1][0];
if t {
break 'b_378e44;
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
break 'b_378e44;
}
}
c.cycles += 3;
c.g[V0][0] = c.lb((c.g[S0][0] as u32));
if c.g[V0][0] != 0u64 {
c.g[V1][0] = c.lbu((c.g[S0][0] as u32));
if event_due(c) { deliver_event_here(c, 0x00378b58); } continue 'l_378b58;
} else {
c.cycles += 1;
c.g[V1][0] = sx((c.g[T4][0] as u32) << 1);
break 'b_378e54;
}
}
}
}
'b_378fec: {
c.cycles += 69;
c.g[A1][0] = 0x50000000;
c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3));
c.g[V0][0] = 0x10000000;
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.g[A0][0] | c.g[A1][0];
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.g[A2][0] = 0x56;
c.g[A0][0] = c.g[A0][0] << 32;
c.g[V0][0] = 0x24000000;
c.g[V0][0] = c.g[V0][0] << 32;
c.g[V0][0] = c.g[V0][0] | 0x1;
c.g[A1][0] = 0x10;
c.sd((c.g[T3][0] as u32).wrapping_add(0x30), c.g[A2][0]);
c.sd((c.g[T3][0] as u32), c.g[V1][0]);
c.g[A3][0] = 0x8800;
c.g[A3][0] = c.g[A3][0] << 47;
c.sd((c.g[T3][0] as u32).wrapping_add(0x18), c.g[A0][0]);
c.g[V1][0] = c.g[T4][0] | 0x8000;
c.sd((c.g[T3][0] as u32).wrapping_add(0x20), c.g[V0][0]);
c.g[A3][0] = c.g[V1][0] | c.g[A3][0];
c.sd((c.g[T3][0] as u32).wrapping_add(0x28), c.g[A1][0]);
c.g[V0][0] = 0xfff0000;
c.sd((c.g[T3][0] as u32).wrapping_add(0x8), 0u64);
c.g[V1][0] = 0x5252;
c.sd((c.g[T3][0] as u32).wrapping_add(0x10), 0u64);
c.g[V0][0] = c.g[V0][0] | 0xffff;
c.g[AT][0] = 0x434c0000;
c.mtc1(1, 0);
c.g[T1][0] = c.g[T0][0] & c.g[V0][0];
c.lwc1(2, (c.g[S3][0] as u32).wrapping_add(0x8));
c.g[T0][0] = sx((c.g[T1][0] as u32).wrapping_add(0x10));
c.lwc1(3, (c.g[S3][0] as u32).wrapping_add(0xc));
c.lwc1(1, (c.g[S3][0] as u32).wrapping_add(0x4));
c.mul_s(2, 2, 0);
c.lwc1(5, (c.g[S3][0] as u32));
c.mul_s(3, 3, 0);
c.sd((c.g[T3][0] as u32).wrapping_add(0x48), c.g[V1][0]);
c.mul_s(1, 1, 0);
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 6);
c.cvt_w_s(0, 2);
c.mfc1(2, 0);
c.g[AT][0] = 0x43000000;
c.mtc1(1, 4);
c.cvt_w_s(0, 3);
c.mfc1(4, 0);
c.mfc1(3, 6);
c.mul_s(5, 5, 4);
c.sd((c.g[T3][0] as u32).wrapping_add(0x40), c.g[A3][0]);
c.g[A1][0] = c.g[V1][0];
c.cvt_w_s(0, 1);
c.mfc1(6, 0);
c.g[V0][0] = c.g[V0][0] << 8;
c.cvt_w_s(0, 5);
c.mfc1(3, 0);
c.g[A0][0] = c.g[A0][0] << 16;
c.g[A1][0] = c.g[A1][0] << 32;
c.g[V0][0] = c.g[A2][0] | c.g[V0][0];
c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
c.g[V1][0] = c.g[V1][0] << 24;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
c.sd((c.g[T3][0] as u32).wrapping_add(0x38), c.g[V0][0]);
c.sw((c.g[S2][0] as u32).wrapping_add(0x5a00), c.g[T0][0]);
c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18f0));
c.g[V0][0] = c.lw((c.g[A3][0] as u32));
c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[T0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xe84));
if t {
break 'b_378fec;
} else {
c.cycles += 33;
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
c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[T7][0]);
c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), 0u64);
break 'b_378fec;
}
}
c.cycles += 14;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
c.lwc1(26, (c.g[SP][0] as u32).wrapping_add(0x120));
c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x118));
c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x110));
c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x108));
c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x100));
c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0xf8));
c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xf0));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x130));
c.pc = t; return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, check_reaching, machine};
    use crate::difftest::{fill, DATA};
    use std::sync::atomic::AtomicUsize;

    /// Gate of the oc1461 fake (other tests' machines run the real callee).
    const OC1461_GATE: u32 = DATA + 0xe_1460;
    const OC1461_MAGIC: u32 = 0x1461_0808;
    static OC1461_PREV_UVM: AtomicUsize = AtomicUsize::new(0);

    /// `PS2GraphicsMan_UpdateViewMatrices` (0x37d968) stands in as a logged fake.
    fn oc1461_fake_uvm(c: &mut Cpu) {
        if c.bus.read32(OC1461_GATE) != OC1461_MAGIC {
            return fakes::fall_back(c, &OC1461_PREV_UVM, ee::s12::f_0037d968__generated);
        }
        fakes::fake_call(c, 0x0037_d968);
    }

    /// Graphics manager `self`, font, string, packet buffer and colour for
    /// `DrawGlyphs(self, font, text, color, x, y, sx, sy, su, sv, z)`.
    fn oc1461_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x1461_0808, 1);
        fakes::hook(
            std::ptr::addr_of_mut!(ee::s12::OVERRIDE_0037D968),
            oc1461_fake_uvm,
            &OC1461_PREV_UVM,
        );
        c.bus.write32(OC1461_GATE, OC1461_MAGIC);
        c.pc = 0x0037_8808;
        let (gm, font, text, color) = (DATA + 0x1_0000, DATA + 0x3_0000, DATA + 0x3_8000, DATA + 0x3_9000);
        let (table, dflt, packet, dma) = (DATA + 0x3_1000, DATA + 0x3_4000, DATA + 0x20_0000, DATA + 0x30_0000);
        // Graphics manager: float constants, 3 matrix-state slots of 0x230 bytes.
        for w in 0..0xc0 {
            let v = if seed % 7 == 0 { r.edge_f32() } else { r.f32(4.0) };
            c.bus.write32(gm + 0x6a00 + 4 * w, v);
        }
        // `mMatricesValid`: 0 takes the UpdateViewMatrices call.
        c.bus.write32(gm + 0x6b90, if seed % 5 == 0 { 0 } else { 1 });
        let slot = r.next_u32() % 3;
        c.bus.write32(gm + 0x10dc, slot);
        // Selects the two glyph-packet layouts (non-zero: first, zero: second).
        c.bus.write32(gm + slot * 0x230 + 0x6d40, if seed % 2 == 0 { 0 } else { 1 + r.next_u32() % 5 });
        c.bus.write32(gm + 0x5a00, packet);
        c.bus.write32(gm + 0x18f0, dma);
        c.bus.write32(gm + 0xe84, DATA + 0x2_0000);
        fill(c, &mut r, DATA + 0x2_0000, 8);
        // DMA bookkeeping: counter < 2600 copies the packet header.
        let counts = [0, 7, 2599, 2600, 4000];
        c.bus.write32(dma, counts[(r.next_u32() % 5) as usize]);
        fill(c, &mut r, dma + 0x6_8000 + 0x1cc0, 2);
        // Font: [0] end index, [4] glyph table, [8] default glyph, [0xc] first char,
        // [0x10] last char of the direct table; records of 12 bytes keyed by u16.
        let (lo, hi, extra) = (0x20u32, 0x7eu32, 8u32);
        c.bus.write32(font, (hi - lo + 1) + extra);
        c.bus.write32(font + 4, table);
        c.bus.write32(font + 8, if seed % 9 == 0 { 0 } else { dflt });
        c.bus.write32(font + 0xc, lo);
        c.bus.write32(font + 0x10, hi);
        fill(c, &mut r, table, 3 * (hi - lo + 1 + extra));
        fill(c, &mut r, dflt, 3);
        for i in 0..(hi - lo + 1 + extra) {
            let key = if i <= hi - lo { lo + i } else { 0x80 + 0x10 * (i - (hi - lo + 1)) };
            let rec = table + 12 * i;
            let hi_bytes = r.next_u32() & 0xffff_0000;
            c.bus.write32(rec, key | hi_bytes);
        }
        // Text: direct, found-by-search, not-found, below-range and empty.
        let alphabet = [0x41u8, 0x6a, 0x20, 0x7e, 0x05, 0x7f, 0x80, 0xa0, 0xf0, 0x85, 0xff, 0x31];
        let len = if seed % 11 == 0 { 0 } else { 1 + r.next_u32() % 10 };
        for i in 0..len {
            let b = alphabet[(r.next_u32() % 12) as usize];
            c.bus.write8(text + i, b);
        }
        c.bus.write8(text + len, 0);
        for i in 0..4 {
            c.bus.write32(color + 4 * i, r.f32(1.0));
        }
        c.g[A0][0] = sx(gm);
        c.g[A1][0] = sx(font);
        c.g[A2][0] = sx(text);
        c.g[A3][0] = sx(color);
        for f in 12..=18 {
            c.f[f] = r.f32(8.0);
        }
    }

    /// `DrawGlyphs` builds the VIF/GIF packet for a string (both glyph layouts, the
    /// direct and the searched glyph lookup) against the generated body.
    #[test]
    fn draw_glyphs_matches() {
        check_reaching(
            ee::s12::f_00378808__generated,
            draw_glyphs,
            120,
            &[0x0037_8878],
            &oc1461_setup,
        );
        // Both layouts and the search arm: many distinct cycle totals.
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..120 {
            let mut c = crate::difftest::fresh();
            oc1461_setup(&mut c, seed);
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, ee::s12::f_00378808__generated);
            seen.insert(c.cycles - before);
        }
        assert!(seen.len() > 30, "only {} distinct paths", seen.len());
    }
}
