//! Staging module oc1464 (wave wave274): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1464`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0037_9bd0,
    ps2_graphics_man_v72,
    "PS2GraphicsMan_v72",
)];

/// `PS2GraphicsMan_v72` (0x00379bd0): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
#[allow(unreachable_code, unused_assignments, unused_variables, unused_parens)]
pub fn ps2_graphics_man_v72(c: &mut Cpu) {
    if c.pc != 0x00379bd0 {
        return ee::s12::f_00379bd0__generated(c);
    }
    guest(c, |c| {
'b_379c4c: {
c.cycles += 29;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
c.g[A2][0] = 0xffffffffc0000000;
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.g[S0][0] = c.g[T3][0];
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.g[S1][0] = c.g[A1][0];
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.g[S2][0] = c.g[A0][0];
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.g[A0][0] = 0xfffffffffffffc3f;
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[S4][0] = c.g[T0][0];
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[S5][0] = c.g[A3][0];
c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[RA][0]);
c.g[S6][0] = c.g[T1][0];
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[V1][0] as u32));
c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
c.g[V0][0] = c.g[V0][0] | 0xc0;
c.sw((c.g[V1][0] as u32), c.g[V0][0]);
c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
c.sw((c.g[A0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6b90));
let t = c.g[V1][0] != 0u64;
c.g[S3][0] = c.g[T2][0];
if t {
break 'b_379c4c;
} else {
c.cycles += 2;
c.g[RA][0] = sx(0x00379c4c);
c.g[A0][0] = c.g[S2][0];
Callee { addr: 0x0037d968, f: ee::s12::PS2GraphicsMan_UpdateViewMatrices }.call(c, 0x00379c4c)?;
break 'b_379c4c;
}
}
'b_37a184: {
'b_379fbc: {
c.cycles += 83;
c.g[V0][0] = 0x500000;
c.g[T4][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a94));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe60));
c.lqc2(1, (c.g[V0][0] as u32));
c.lqc2(2, (c.g[V0][0] as u32).wrapping_add(0x10));
c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
c.sqc2(1, (c.g[T4][0] as u32));
c.sqc2(2, (c.g[T4][0] as u32).wrapping_add(0x10));
c.sqc2(3, (c.g[T4][0] as u32).wrapping_add(0x20));
c.sqc2(4, (c.g[T4][0] as u32).wrapping_add(0x30));
c.lwc1(2, (c.g[S0][0] as u32).wrapping_add(0x8));
c.lwc1(0, (c.g[S0][0] as u32));
c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x4));
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 3);
c.swc1(0, (c.g[SP][0] as u32));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x8));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xc));
c.lqc2(4, (c.g[T4][0] as u32));
c.lqc2(5, (c.g[T4][0] as u32).wrapping_add(0x10));
c.lqc2(6, (c.g[T4][0] as u32).wrapping_add(0x20));
c.lqc2(7, (c.g[T4][0] as u32).wrapping_add(0x30));
c.lqc2(8, (c.g[SP][0] as u32));
c.vu0().mula(0b1111, 4, Src::Bc(8, X));
c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
c.sqc2(12, (c.g[T4][0] as u32).wrapping_add(0x30));
c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6b30));
c.lqc2(4, (c.g[V0][0] as u32));
c.lqc2(5, (c.g[V0][0] as u32).wrapping_add(0x10));
c.lqc2(6, (c.g[V0][0] as u32).wrapping_add(0x20));
c.lqc2(7, (c.g[V0][0] as u32).wrapping_add(0x30));
c.lqc2(8, (c.g[T4][0] as u32));
c.lqc2(9, (c.g[T4][0] as u32).wrapping_add(0x10));
c.lqc2(10, (c.g[T4][0] as u32).wrapping_add(0x20));
c.lqc2(11, (c.g[T4][0] as u32).wrapping_add(0x30));
c.vu0().mula(0b1111, 4, Src::Bc(8, X));
c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
c.vu0().mula(0b1111, 4, Src::Bc(9, X));
c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
c.vu0().mula(0b1111, 4, Src::Bc(10, X));
c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
c.vu0().mula(0b1111, 4, Src::Bc(11, X));
c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
c.sqc2(12, (c.g[V1][0] as u32));
c.sqc2(13, (c.g[V1][0] as u32).wrapping_add(0x10));
c.sqc2(14, (c.g[V1][0] as u32).wrapping_add(0x20));
c.sqc2(15, (c.g[V1][0] as u32).wrapping_add(0x30));
c.lqc2(1, (c.g[V1][0] as u32));
c.lqc2(2, (c.g[V1][0] as u32).wrapping_add(0x10));
c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x30));
c.sqc2(1, (c.g[A0][0] as u32));
c.sqc2(2, (c.g[A0][0] as u32).wrapping_add(0x10));
c.sqc2(3, (c.g[A0][0] as u32).wrapping_add(0x20));
c.sqc2(4, (c.g[A0][0] as u32).wrapping_add(0x30));
c.lqc2(1, (c.g[A0][0] as u32));
c.lqc2(2, (c.g[A0][0] as u32).wrapping_add(0x10));
c.lqc2(3, (c.g[A0][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[A0][0] as u32).wrapping_add(0x30));
c.sqc2(1, (c.g[T4][0] as u32));
c.sqc2(2, (c.g[T4][0] as u32).wrapping_add(0x10));
c.sqc2(3, (c.g[T4][0] as u32).wrapping_add(0x20));
c.sqc2(4, (c.g[T4][0] as u32).wrapping_add(0x30));
c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xe84));
c.g[V1][0] = 0x30000000;
c.g[T5][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5a00));
c.g[V0][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0x10));
let t = (c.g[V0][0] as i64) < 0;
c.g[T1][0] = c.g[T5][0] | c.g[V1][0];
if t {
break 'b_379fbc;
} else {
'b_379fa8: {
c.cycles += 42;
c.g[V0][0] = sx((c.g[S1][0] as u32) << 3);
c.g[A2][0] = c.g[S1][0] & 0x1;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
c.g[T0][0] = 0x50000000;
c.g[V1][0] = sx((c.g[V0][0] as u32) >> 31);
c.g[A3][0] = 0x10000000;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[A0][0] = c.g[S1][0] | 0x8000;
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 1) as u32);
c.g[A1][0] = 0x9400;
c.g[A1][0] = c.g[A1][0] << 48;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.g[A1][0] = c.g[A0][0] | c.g[A1][0];
c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3));
c.g[A0][0] = 0x10000000;
c.g[A0][0] = c.g[A0][0] << 32;
c.g[A0][0] = c.g[A0][0] | 0x1;
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
c.g[V1][0] = c.g[V1][0] | c.g[T0][0];
c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
c.g[V1][0] = c.g[V1][0] << 32;
c.g[A2][0] = 0xe;
c.g[A3][0] = 0x5b;
c.g[T0][0] = 0x5;
c.g[T0][0] = c.g[T0][0] << 16;
c.g[T0][0] = c.g[T0][0] | 0x1251;
c.g[T0][0] = c.g[T0][0] << 16;
c.g[T0][0] = c.g[T0][0] | 0x2512;
c.sd((c.g[T1][0] as u32), c.g[V0][0]);
c.sd((c.g[T1][0] as u32).wrapping_add(0x18), c.g[V1][0]);
c.sd((c.g[T1][0] as u32).wrapping_add(0x20), c.g[A0][0]);
c.g[T2][0] = 0u64;
c.sd((c.g[T1][0] as u32).wrapping_add(0x28), c.g[A2][0]);
c.g[T3][0] = c.g[S1][0] & 0x1;
c.sd((c.g[T1][0] as u32).wrapping_add(0x30), c.g[A3][0]);
c.sd((c.g[T1][0] as u32).wrapping_add(0x40), c.g[A1][0]);
c.sd((c.g[T1][0] as u32).wrapping_add(0x48), c.g[T0][0]);
c.sd((c.g[T1][0] as u32).wrapping_add(0x8), 0u64);
c.sd((c.g[T1][0] as u32).wrapping_add(0x10), 0u64);
c.sd((c.g[T1][0] as u32).wrapping_add(0x38), 0u64);
let t = (c.g[S1][0] as i64) <= 0;
c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x50));
if t {
break 'b_379fa8;
} else {
c.cycles += 8;
c.mov_s(5, 3);
c.g[AT][0] = 0x3f000000;
c.mtc1(1, 4);
c.mfc1(3, 5);
c.g[T0][0] = 0xc;
c.g[V0][0] = c.g[V1][0];
c.g[A3][0] = c.g[V0][0] << 32;
'l_379e60: loop {
c.cycles += 82;
c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add((c.g[T2][0] as u32)));
c.g[A0][0] = c.lbu((c.g[V0][0] as u32));
c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
c.g[A2][0] = ((c.g[T2][0] as i64) < (c.g[S1][0] as i64)) as u64;
c.mult(0, 3, 4, 8);
c.g[A1][0] = sx((c.g[A0][0] as u32) << 3);
c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[A0][0] = sx((c.g[A0][0] as u32) << 4);
c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S4][0] as u32)));
c.lwc1(0, (c.g[V0][0] as u32));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x1c));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x18));
c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x10));
c.lqc2(4, (c.g[T4][0] as u32));
c.lqc2(5, (c.g[T4][0] as u32).wrapping_add(0x10));
c.lqc2(6, (c.g[T4][0] as u32).wrapping_add(0x20));
c.lqc2(7, (c.g[T4][0] as u32).wrapping_add(0x30));
c.vu0().mula(0b1111, 4, Src::Bc(8, X));
c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sq(2, (c.g[SP][0] as u32));
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
c.mfc1(3, 0);
c.qmtc2(3, 3);
c.vu0().div(0, 3, 3, 0);
c.lqc2(4, (c.g[SP][0] as u32));
c.vu0().waitq();
c.vu0().mul(0b1111, 5, 4, Src::Q);
c.sqc2(5, (c.g[SP][0] as u32));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
c.g[V1][0] = c.lw((c.g[A1][0] as u32));
c.g[V0][0] = c.g[V0][0] << 32;
c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
c.sd((c.g[T1][0] as u32), c.g[V1][0]);
c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x8));
c.lwc1(2, (c.g[A0][0] as u32).wrapping_add(0xc));
c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
c.mul_s(1, 1, 4);
c.lwc1(3, (c.g[A0][0] as u32));
c.mul_s(2, 2, 4);
c.mul_s(0, 0, 4);
c.mul_s(3, 3, 4);
c.cvt_w_s(6, 1);
c.mfc1(2, 6);
c.cvt_w_s(1, 0);
c.mfc1(5, 1);
c.cvt_w_s(1, 2);
c.mfc1(3, 1);
c.cvt_w_s(0, 3);
c.mfc1(4, 0);
c.g[V0][0] = c.g[V0][0] << 8;
c.g[V0][0] = c.g[A1][0] | c.g[V0][0];
c.g[V1][0] = c.g[V1][0] << 16;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.g[A0][0] = c.g[A0][0] << 24;
c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
c.sd((c.g[T1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
c.lwc1(1, (c.g[SP][0] as u32));
c.cvt_w_s(3, 0);
c.mfc1(2, 3);
c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x8));
c.cvt_w_s(0, 1);
c.mfc1(4, 0);
c.cvt_w_s(0, 2);
c.mfc1(3, 0);
c.g[V0][0] = c.g[V0][0] << 16;
c.g[V0][0] = c.g[A0][0] | c.g[V0][0];
c.g[V1][0] = c.g[V1][0] << 32;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sd((c.g[T1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
let t = c.g[A2][0] != 0u64;
c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x18));
if t {
if event_due(c) { deliver_event_here(c, 0x00379e60); } continue 'l_379e60;
} else {
break 'b_379fa8;
}
}
}
}
c.cycles += 2;
let t = c.g[T3][0] == 0u64;
c.g[V0][0] = 0xfff0000;
if t {
break 'b_37a184;
} else {
c.cycles += 3;
c.sd((c.g[T1][0] as u32), 0u64);
let t = true;
c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x8));
if t {
break 'b_37a184;
} else {
break 'b_379fbc;
}
}
}
}
'b_37a180: {
c.cycles += 33;
c.g[V0][0] = sx((c.g[S1][0] as u32) << 1);
c.g[A1][0] = 0x50000000;
c.g[T3][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
c.g[A2][0] = c.g[S1][0] | 0x8000;
c.g[V1][0] = sx((c.g[T3][0] as u32).wrapping_add(0x3));
c.g[V0][0] = 0x10000000;
c.g[V1][0] = c.g[V1][0] | c.g[A1][0];
c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_add(0x4));
c.g[A1][0] = 0xc800;
c.g[A1][0] = c.g[A1][0] << 47;
c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
c.g[A1][0] = c.g[A2][0] | c.g[A1][0];
c.g[V1][0] = c.g[V1][0] << 32;
c.g[V0][0] = 0x10000000;
c.g[V0][0] = c.g[V0][0] << 32;
c.g[V0][0] = c.g[V0][0] | 0x1;
c.g[A2][0] = 0xe;
c.g[A3][0] = 0x4b;
c.g[T0][0] = 0x510000;
c.g[T0][0] = c.g[T0][0] | 0x5151;
c.sd((c.g[T1][0] as u32), c.g[A0][0]);
c.g[T2][0] = 0u64;
c.sd((c.g[T1][0] as u32).wrapping_add(0x18), c.g[V1][0]);
c.sd((c.g[T1][0] as u32).wrapping_add(0x20), c.g[V0][0]);
c.sd((c.g[T1][0] as u32).wrapping_add(0x28), c.g[A2][0]);
c.sd((c.g[T1][0] as u32).wrapping_add(0x30), c.g[A3][0]);
c.sd((c.g[T1][0] as u32).wrapping_add(0x40), c.g[A1][0]);
c.sd((c.g[T1][0] as u32).wrapping_add(0x48), c.g[T0][0]);
c.sd((c.g[T1][0] as u32).wrapping_add(0x8), 0u64);
c.sd((c.g[T1][0] as u32).wrapping_add(0x10), 0u64);
c.sd((c.g[T1][0] as u32).wrapping_add(0x38), 0u64);
let t = (c.g[T3][0] as i64) <= 0;
c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x50));
if t {
break 'b_37a180;
} else {
c.cycles += 8;
c.mov_s(4, 3);
c.g[T0][0] = c.g[T3][0];
c.mfc1(3, 4);
c.g[T3][0] = 0xc;
c.g[AT][0] = 0x3f000000;
c.mtc1(1, 5);
c.g[V0][0] = c.g[V1][0];
c.g[A3][0] = c.g[V0][0] << 32;
'l_37a060: loop {
c.cycles += 72;
c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add((c.g[T2][0] as u32)));
c.g[V1][0] = c.lbu((c.g[V0][0] as u32));
c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(0x1));
c.g[A2][0] = ((c.g[T2][0] as i64) < (c.g[T0][0] as i64)) as u64;
c.mult(0, 4, 3, 11);
c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
c.lwc1(0, (c.g[V0][0] as u32));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x4));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x14));
c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x1c));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x18));
c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x10));
c.lqc2(4, (c.g[T4][0] as u32));
c.lqc2(5, (c.g[T4][0] as u32).wrapping_add(0x10));
c.lqc2(6, (c.g[T4][0] as u32).wrapping_add(0x20));
c.lqc2(7, (c.g[T4][0] as u32).wrapping_add(0x30));
c.vu0().mula(0b1111, 4, Src::Bc(8, X));
c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sq(2, (c.g[SP][0] as u32));
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
c.mfc1(2, 0);
c.qmtc2(2, 3);
c.vu0().div(0, 3, 3, 0);
c.lqc2(4, (c.g[SP][0] as u32));
c.vu0().waitq();
c.vu0().mul(0b1111, 5, 4, Src::Q);
c.sqc2(5, (c.g[SP][0] as u32));
c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0x8));
c.lwc1(1, (c.g[V1][0] as u32));
c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0xc));
c.cvt_w_s(6, 3);
c.mfc1(2, 6);
c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x4));
c.mul_s(1, 1, 5);
c.cvt_w_s(3, 0);
c.mfc1(3, 3);
c.cvt_w_s(0, 2);
c.mfc1(5, 0);
c.cvt_w_s(0, 1);
c.mfc1(4, 0);
c.g[V0][0] = c.g[V0][0] << 8;
c.g[V1][0] = c.g[V1][0] << 16;
c.g[V0][0] = c.g[A1][0] | c.g[V0][0];
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.g[A0][0] = c.g[A0][0] << 24;
c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
c.sd((c.g[T1][0] as u32), c.g[V0][0]);
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
c.lwc1(1, (c.g[SP][0] as u32));
c.cvt_w_s(3, 0);
c.mfc1(2, 3);
c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x8));
c.cvt_w_s(0, 1);
c.mfc1(4, 0);
c.cvt_w_s(0, 2);
c.mfc1(3, 0);
c.g[V0][0] = c.g[V0][0] << 16;
c.g[V0][0] = c.g[A0][0] | c.g[V0][0];
c.g[V1][0] = c.g[V1][0] << 32;
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sd((c.g[T1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
let t = c.g[A2][0] != 0u64;
c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x10));
if t {
if event_due(c) { deliver_event_here(c, 0x0037a060); } continue 'l_37a060;
} else {
break 'b_37a180;
}
}
}
}
c.cycles += 1;
c.g[V0][0] = 0xfff0000;
break 'b_37a184;
}
'b_37a228: {
c.cycles += 9;
c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18f0));
c.g[V0][0] = c.g[V0][0] | 0xffff;
c.g[T0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xe84));
c.g[T1][0] = c.g[T1][0] & c.g[V0][0];
c.sw((c.g[S2][0] as u32).wrapping_add(0x5a00), c.g[T1][0]);
c.g[V0][0] = c.lw((c.g[A3][0] as u32));
c.g[V0][0] = ((c.g[V0][0] as i64) < 2600i64) as u64;
let t = c.g[V0][0] == 0u64;
c.g[A1][0] = 0x60000;
if t {
break 'b_37a228;
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
c.sw((c.g[V0][0] as u32).wrapping_add(0x14), c.g[T5][0]);
c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
c.sh((c.g[V0][0] as u32).wrapping_add(0x1e), 0u64);
break 'b_37a228;
}
}
c.cycles += 13;
c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5a00));
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.sw((c.g[S2][0] as u32).wrapping_add(0x5a00), c.g[V0][0]);
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
c.pc = t; return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, put_words, require_paths, run_generated, DATA, GAME_GP};

    /// World (`a0`), GS register block, output packet buffer, draw-command
    /// ring, matrix scratch, and the vertex arrays, all inside `DATA`.
    const OC1464_WORLD: u32 = DATA + 0x1_0000;
    const OC1464_REGS: u32 = DATA + 0x2_0000;
    const OC1464_OUT: u32 = DATA + 0x3_0000;
    const OC1464_RING: u32 = DATA + 0x4_0000;
    const OC1464_MAT: u32 = DATA + 0x5_0000;
    const OC1464_PROJ: u32 = DATA + 0x6_0000;
    const OC1464_VEC: u32 = DATA + 0x7_0000;
    const OC1464_POS: u32 = DATA + 0x7_1000;
    const OC1464_UV: u32 = DATA + 0x7_2000;
    const OC1464_COL: u32 = DATA + 0x7_3000;
    const OC1464_IDX: u32 = DATA + 0x7_4000;

    /// Fixture for `PS2GraphicsMan_v72` (draws an indexed triangle list into a
    /// GIF packet): the count (0..6), the strip/textured mode (sign of the
    /// register block halfword), the view-matrix-valid flag and the full or
    /// free command ring all vary with the seed.
    fn oc1464_setup(c: &mut Cpu, seed: u32) {
        let mut r = fakes::machine(c, seed, 0x1464, 0);
        let count = seed % 7;
        let neg = (seed / 7) & 1 == 1;
        let have_view = (seed / 14) & 1 == 1;
        let full = (seed / 28) & 1 == 1;
        let w = OC1464_WORLD;
        let rand = |r: &mut crate::difftest::Rng, n: u32| -> Vec<u32> {
            (0..n).map(|_| r.f32(4.0)).collect()
        };
        c.bus.write32(w + 0xe84, OC1464_REGS);
        c.bus.write32(w + 0x5a00, OC1464_OUT + (r.next_u32() & 0xf0));
        c.bus.write32(w + 0x18f0, OC1464_RING);
        c.bus.write32(w + 0x6b90, have_view as u32);
        c.bus.write32(w + 0x13e4, OC1464_PROJ);
        let v = rand(&mut r, 16);
        put_words(c, w + 0x6af0, &v);
        let v = rand(&mut r, 16);
        put_words(c, w + 0x6b30, &v);
        let v = rand(&mut r, 16);
        put_words(c, OC1464_PROJ, &v);
        let v = rand(&mut r, 16);
        put_words(c, 0x004f_f1a0, &v);
        c.bus.write32(GAME_GP + 0x2a94, OC1464_MAT);
        for k in 0..4 {
            let v = r.next_u32();
            c.bus.write32(OC1464_REGS + 4 * k, v);
        }
        c.bus.write32(OC1464_REGS + 0x10, if neg { 0x8001 } else { 0x0001 });
        let n = if full { 2700 } else { r.next_u32() % 4 };
        c.bus.write32(OC1464_RING, n);
        for k in 0..4 {
            let v = r.next_u32();
            c.bus.write32(OC1464_RING + 0x69cc0 + 4 * k, v);
        }
        let v = rand(&mut r, 3);
        put_words(c, OC1464_VEC, &v);
        let v = rand(&mut r, 8 * 3);
        put_words(c, OC1464_POS, &v);
        let v: Vec<u32> = (0..16).map(|_| r.next_u32()).collect();
        put_words(c, OC1464_UV, &v);
        let v: Vec<u32> = (0..8 * 4).map(|_| r.f32(200.0).wrapping_add(0)).collect();
        put_words(c, OC1464_COL, &v);
        for k in 0..24 {
            let b = (r.next_u32() % 8) as u8;
            c.bus.write8(OC1464_IDX + k, b);
        }
        c.g[A0][0] = w as u64;
        c.g[A1][0] = count as u64;
        c.g[A3][0] = OC1464_IDX as u64;
        c.g[T0][0] = OC1464_POS as u64;
        c.g[T1][0] = OC1464_UV as u64;
        c.g[T2][0] = OC1464_COL as u64;
        c.g[T3][0] = OC1464_VEC as u64;
        c.pc = 0x0037_9bd0;
    }

    /// `PS2GraphicsMan_v72` (draw an indexed triangle list: textured or flat
    /// GIF packet, then the ring entry) against the generated body.
    #[test]
    fn ps2_graphics_man_v72_matches() {
        check(
            ee::s12::f_00379bd0__generated,
            ps2_graphics_man_v72,
            240,
            &oc1464_setup,
        );
    }

    /// The fixture reaches both packet kinds, empty and odd counts, the
    /// view-matrix update and the full-ring exit.
    #[test]
    fn ps2_graphics_man_v72_paths() {
        let mut totals = std::collections::BTreeSet::new();
        for seed in 0..240 {
            let mut c = crate::difftest::fresh();
            oc1464_setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, ee::s12::f_00379bd0__generated);
            totals.insert(c.cycles - before);
        }
        assert!(totals.len() >= 16, "only {} distinct paths", totals.len());
    }
}
