//! Staging module oc1467 (wave wave274): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1467`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(0x0038_b370, ps2_bezier_man_v18, "PS2BezierMan_v18")];

/// `PS2BezierMan_v18` (0x0038b370): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ps2_bezier_man_v18(c: &mut Cpu) {
    if c.pc != 0x0038b370 {
        return ee::s12::f_0038b370__generated(c);
    }
    guest(c, |c| {
'b_38b40c: {
c.cycles += 36;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x320));
c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x260));
c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x290));
c.g[FP][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
c.g[S5][0] = c.g[A0][0];
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x2e0));
c.g[A0][0] = 0xfffffffffffffff3;
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x2d0));
c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x5a00));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x2c0));
c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x2b0));
c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x2a0));
c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x280));
c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x270));
c.sd((c.g[SP][0] as u32).wrapping_add(0x250), c.g[RA][0]);
c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x318));
c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x310));
c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x308));
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x300));
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x2f8));
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x2f0));
c.sw((c.g[S5][0] as u32).wrapping_add(0x5364), 0u64);
c.sw((c.g[SP][0] as u32).wrapping_add(0x230), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[V1][0] as u32));
c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
c.sw((c.g[V1][0] as u32), c.g[V0][0]);
c.g[A0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32));
c.g[V0][0] = c.g[V0][0] | 0x30;
c.sw((c.g[A0][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x18f0));
c.sw((c.g[SP][0] as u32).wrapping_add(0x234), c.g[V1][0]);
c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x6b90));
if c.g[V0][0] != 0u64 {
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x234));
break 'b_38b40c;
} else {
c.cycles += 3;
c.g[RA][0] = sx(0x0038b408);
c.g[A0][0] = c.g[FP][0];
Callee { addr: 0x0037d968, f: ee::s12::PS2GraphicsMan_UpdateViewMatrices }.call(c, 0x0038b408)?;
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x234));
break 'b_38b40c;
}
}
'b_38bde0: {
c.cycles += 626;
c.g[V0][0] = 0x60000;
c.g[V0][0] = c.g[V0][0] | 0x8000;
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x10d8));
c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.sw((c.g[SP][0] as u32).wrapping_add(0x238), c.g[V0][0]);
c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1cc0));
c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x118));
c.sw((c.g[SP][0] as u32).wrapping_add(0x23c), c.g[A1][0]);
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x11c));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x0038b438);
c.g[A0][0] = sx((c.g[FP][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0038b438)?;
c.lqc2(1, (c.g[V0][0] as u32));
c.lqc2(2, (c.g[V0][0] as u32).wrapping_add(0x10));
c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
c.sqc2(1, (c.g[SP][0] as u32));
c.sqc2(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.sqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S6][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0x274));
c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
c.lqc2(1, (c.g[SP][0] as u32));
c.lqc2(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sqc2(1, (c.g[V1][0] as u32));
c.sqc2(2, (c.g[V1][0] as u32).wrapping_add(0x10));
c.sqc2(3, (c.g[V1][0] as u32).wrapping_add(0x20));
c.sqc2(4, (c.g[V1][0] as u32).wrapping_add(0x30));
c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x44));
c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1f0));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x50));
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1f4));
c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x48));
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x64));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x58));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x58));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x44));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x48));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x64));
c.g[AT][0] = 0xffffffffbf800000;
c.mtc1(1, 0);
c.mfc1(2, 0);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x90));
c.qmtc2(2, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 21);
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x26bc));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sw((c.g[SP][0] as u32).wrapping_add(0x8c), 0u64);
c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lqc2(4, (c.g[V1][0] as u32));
c.lqc2(5, (c.g[V1][0] as u32).wrapping_add(0x10));
c.lqc2(6, (c.g[V1][0] as u32).wrapping_add(0x20));
c.lqc2(7, (c.g[V1][0] as u32).wrapping_add(0x30));
c.vu0().mula(0b1111, 4, Src::Bc(8, X));
c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
c.sqc2(12, (c.g[A2][0] as u32).wrapping_add(0x20));
c.lwc1(22, (c.g[GP][0] as u32).wrapping_sub(0x26b8));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.lwc1(1, (c.g[GP][0] as u32).wrapping_add(0x1350));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
c.swc1(21, (c.g[A2][0] as u32).wrapping_add(0xc));
c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0x84));
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x80));
c.mul_s(12, 12, 1);
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
c.mul_s(23, 0, 1);
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
c.mul_s(12, 12, 20);
c.mul_s(0, 23, 20);
c.cvt_w_s(1, 12);
c.mfc1(2, 1);
c.cvt_w_s(1, 0);
c.mfc1(3, 1);
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
c.mtc1(3, 2);
c.cvt_s_w(2, 2);
c.sub_s(12, 12, 1);
c.sub_s(0, 0, 2);
c.mul_s(12, 12, 22);
c.g[RA][0] = sx(0x0038b58c);
c.mul_s(23, 0, 22);
call_ptr(c, 0x0031be50, 0x0038b58c)?;
c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1f8));
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1fc));
c.g[RA][0] = sx(0x0038b59c);
c.mov_s(12, 23);
call_ptr(c, 0x0031be50, 0x0038b59c)?;
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x1f4));
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x130));
c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x1fc));
c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x170));
c.neg_s(1, 0);
c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0x1f0));
c.neg_s(5, 3);
c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x1f8));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xc8));
c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xd4));
c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0xd8));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x110));
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xf8));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x118));
c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0xc4));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.sw((c.g[SP][0] as u32).wrapping_add(0xc0), 0u64);
c.sw((c.g[SP][0] as u32).wrapping_add(0xd0), 0u64);
c.sw((c.g[SP][0] as u32).wrapping_add(0xe0), 0u64);
c.sw((c.g[SP][0] as u32).wrapping_add(0xb4), 0u64);
c.sw((c.g[SP][0] as u32).wrapping_add(0xe4), 0u64);
c.sw((c.g[SP][0] as u32).wrapping_add(0xb8), 0u64);
c.sw((c.g[SP][0] as u32).wrapping_add(0xe8), 0u64);
c.sw((c.g[SP][0] as u32).wrapping_add(0xbc), 0u64);
c.sw((c.g[SP][0] as u32).wrapping_add(0xcc), 0u64);
c.sw((c.g[SP][0] as u32).wrapping_add(0xdc), 0u64);
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xec));
c.sw((c.g[SP][0] as u32).wrapping_add(0x100), 0u64);
c.sw((c.g[SP][0] as u32).wrapping_add(0x120), 0u64);
c.sw((c.g[SP][0] as u32).wrapping_add(0xf4), 0u64);
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x104));
c.sw((c.g[SP][0] as u32).wrapping_add(0x114), 0u64);
c.sw((c.g[SP][0] as u32).wrapping_add(0x124), 0u64);
c.sw((c.g[SP][0] as u32).wrapping_add(0x108), 0u64);
c.sw((c.g[SP][0] as u32).wrapping_add(0x128), 0u64);
c.sw((c.g[SP][0] as u32).wrapping_add(0xfc), 0u64);
c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), 0u64);
c.sw((c.g[SP][0] as u32).wrapping_add(0x11c), 0u64);
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x12c));
c.lqc2(4, (c.g[V1][0] as u32));
c.lqc2(5, (c.g[V1][0] as u32).wrapping_add(0x10));
c.lqc2(6, (c.g[V1][0] as u32).wrapping_add(0x20));
c.lqc2(7, (c.g[V1][0] as u32).wrapping_add(0x30));
c.lqc2(8, (c.g[V0][0] as u32));
c.lqc2(9, (c.g[V0][0] as u32).wrapping_add(0x10));
c.lqc2(10, (c.g[V0][0] as u32).wrapping_add(0x20));
c.lqc2(11, (c.g[V0][0] as u32).wrapping_add(0x30));
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
c.sqc2(12, (c.g[A0][0] as u32));
c.sqc2(13, (c.g[A0][0] as u32).wrapping_add(0x10));
c.sqc2(14, (c.g[A0][0] as u32).wrapping_add(0x20));
c.sqc2(15, (c.g[A0][0] as u32).wrapping_add(0x30));
c.lqc2(1, (c.g[A0][0] as u32));
c.lqc2(2, (c.g[A0][0] as u32).wrapping_add(0x10));
c.lqc2(3, (c.g[A0][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[A0][0] as u32).wrapping_add(0x30));
c.sqc2(1, (c.g[A1][0] as u32));
c.sqc2(2, (c.g[A1][0] as u32).wrapping_add(0x10));
c.sqc2(3, (c.g[A1][0] as u32).wrapping_add(0x20));
c.sqc2(4, (c.g[A1][0] as u32).wrapping_add(0x30));
c.g[V0][0] = 0x500000;
c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1b0));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x49c0));
c.lqc2(4, (c.g[V0][0] as u32));
c.lqc2(5, (c.g[V0][0] as u32).wrapping_add(0x10));
c.lqc2(6, (c.g[V0][0] as u32).wrapping_add(0x20));
c.lqc2(7, (c.g[V0][0] as u32).wrapping_add(0x30));
c.lqc2(8, (c.g[A1][0] as u32));
c.lqc2(9, (c.g[A1][0] as u32).wrapping_add(0x10));
c.lqc2(10, (c.g[A1][0] as u32).wrapping_add(0x20));
c.lqc2(11, (c.g[A1][0] as u32).wrapping_add(0x30));
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
c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x360));
c.lqc2(1, (c.g[A0][0] as u32));
c.lqc2(2, (c.g[A0][0] as u32).wrapping_add(0x10));
c.lqc2(3, (c.g[A0][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[A0][0] as u32).wrapping_add(0x30));
c.sqc2(1, (c.g[V0][0] as u32));
c.sqc2(2, (c.g[V0][0] as u32).wrapping_add(0x10));
c.sqc2(3, (c.g[V0][0] as u32).wrapping_add(0x20));
c.sqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x84));
c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x200));
c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x1254));
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x204));
c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x88));
c.mul_s(23, 3, 0);
c.mul_s(24, 2, 0);
c.mul_s(25, 1, 0);
c.mul_s(2, 23, 20);
c.mul_s(0, 24, 20);
c.mul_s(20, 25, 20);
c.cvt_w_s(1, 2);
c.mfc1(2, 1);
c.cvt_w_s(1, 0);
c.mfc1(3, 1);
c.cvt_w_s(1, 20);
c.mfc1(6, 1);
c.mtc1(2, 1);
c.cvt_s_w(1, 1);
c.mtc1(3, 3);
c.cvt_s_w(3, 3);
c.mtc1(6, 4);
c.cvt_s_w(4, 4);
c.sub_s(2, 2, 1);
c.sub_s(0, 0, 3);
c.sub_s(20, 20, 4);
c.mul_s(23, 2, 22);
c.mul_s(24, 0, 22);
c.mul_s(25, 20, 22);
c.g[RA][0] = sx(0x0038b814);
c.mov_s(12, 23);
call_ptr(c, 0x0031be50, 0x0038b814)?;
c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x208));
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20c));
c.g[RA][0] = sx(0x0038b824);
c.mov_s(12, 24);
call_ptr(c, 0x0031be50, 0x0038b824)?;
c.mov_s(12, 25);
c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x210));
c.g[RA][0] = sx(0x0038b834);
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x214));
call_ptr(c, 0x0031be50, 0x0038b834)?;
c.g[V0][0] = 0x500000;
c.g[S1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x3e0));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe60));
c.lqc2(1, (c.g[V0][0] as u32));
c.lqc2(2, (c.g[V0][0] as u32).wrapping_add(0x10));
c.lqc2(3, (c.g[V0][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[V0][0] as u32).wrapping_add(0x30));
c.sqc2(1, (c.g[S1][0] as u32));
c.sqc2(2, (c.g[S1][0] as u32).wrapping_add(0x10));
c.sqc2(3, (c.g[S1][0] as u32).wrapping_add(0x20));
c.sqc2(4, (c.g[S1][0] as u32).wrapping_add(0x30));
c.mov_s(12, 23);
c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x218));
c.g[RA][0] = sx(0x0038b870);
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x21c));
call_ptr(c, 0x0031be50, 0x0038b870)?;
c.lwc1(11, (c.g[SP][0] as u32).wrapping_add(0x21c));
c.g[V0][0] = 0x500000;
c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xec0));
c.lwc1(0, (c.g[V0][0] as u32).wrapping_sub(0xec0));
c.sub_s(1, 21, 11);
c.lwc1(3, (c.g[V1][0] as u32).wrapping_add(0x8));
c.lwc1(6, (c.g[V1][0] as u32).wrapping_add(0x4));
c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
c.lwc1(8, (c.g[SP][0] as u32).wrapping_add(0x218));
c.mul_s(2, 1, 3);
c.sw((c.g[SP][0] as u32).wrapping_add(0xc0), 0u64);
c.mul_s(4, 1, 0);
c.sw((c.g[SP][0] as u32).wrapping_add(0xc4), 0u64);
c.mul_s(1, 1, 6);
c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
c.mul_s(13, 8, 3);
c.sw((c.g[SP][0] as u32).wrapping_add(0x9c), 0u64);
c.mul_s(12, 8, 0);
c.sw((c.g[SP][0] as u32).wrapping_add(0xac), 0u64);
c.mul_s(9, 2, 3);
c.sw((c.g[SP][0] as u32).wrapping_add(0xbc), 0u64);
c.mul_s(10, 4, 3);
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xcc));
c.mul_s(5, 2, 0);
c.mul_s(7, 4, 0);
c.mul_s(3, 1, 3);
c.mul_s(0, 1, 0);
c.mul_s(8, 8, 6);
c.mul_s(2, 2, 6);
c.mul_s(4, 4, 6);
c.mul_s(1, 1, 6);
c.add_s(5, 5, 8);
c.sub_s(0, 0, 13);
c.sub_s(2, 2, 12);
c.add_s(9, 9, 11);
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x98));
c.add_s(7, 7, 11);
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x94));
c.add_s(4, 4, 13);
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xa8));
c.sub_s(10, 10, 8);
c.swc1(9, (c.g[SP][0] as u32).wrapping_add(0xb8));
c.add_s(1, 1, 11);
c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x90));
c.add_s(3, 3, 12);
c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.swc1(10, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xa4));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xb4));
c.lqc2(4, (c.g[S1][0] as u32));
c.lqc2(5, (c.g[S1][0] as u32).wrapping_add(0x10));
c.lqc2(6, (c.g[S1][0] as u32).wrapping_add(0x20));
c.lqc2(7, (c.g[S1][0] as u32).wrapping_add(0x30));
c.lqc2(8, (c.g[S2][0] as u32));
c.lqc2(9, (c.g[S2][0] as u32).wrapping_add(0x10));
c.lqc2(10, (c.g[S2][0] as u32).wrapping_add(0x20));
c.lqc2(11, (c.g[S2][0] as u32).wrapping_add(0x30));
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
c.sqc2(12, (c.g[S1][0] as u32));
c.sqc2(13, (c.g[S1][0] as u32).wrapping_add(0x10));
c.sqc2(14, (c.g[S1][0] as u32).wrapping_add(0x20));
c.sqc2(15, (c.g[S1][0] as u32).wrapping_add(0x30));
c.mov_s(12, 24);
c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x220));
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x224));
c.g[RA][0] = sx(0x0038b9b8);
c.g[S0][0] = 0x500000;
call_ptr(c, 0x0031be50, 0x0038b9b8)?;
c.lwc1(11, (c.g[SP][0] as u32).wrapping_add(0x224));
c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xeb0));
c.lwc1(3, (c.g[V0][0] as u32).wrapping_add(0x8));
c.sub_s(1, 21, 11);
c.lwc1(0, (c.g[S0][0] as u32).wrapping_sub(0xeb0));
c.lwc1(6, (c.g[V0][0] as u32).wrapping_add(0x4));
c.lwc1(8, (c.g[SP][0] as u32).wrapping_add(0x220));
c.mul_s(2, 1, 3);
c.sw((c.g[SP][0] as u32).wrapping_add(0xc0), 0u64);
c.mul_s(4, 1, 0);
c.sw((c.g[SP][0] as u32).wrapping_add(0xc4), 0u64);
c.mul_s(1, 1, 6);
c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
c.mul_s(13, 8, 3);
c.sw((c.g[SP][0] as u32).wrapping_add(0x9c), 0u64);
c.mul_s(12, 8, 0);
c.sw((c.g[SP][0] as u32).wrapping_add(0xac), 0u64);
c.mul_s(9, 2, 3);
c.sw((c.g[SP][0] as u32).wrapping_add(0xbc), 0u64);
c.mul_s(10, 4, 3);
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xcc));
c.mul_s(5, 2, 0);
c.mul_s(7, 4, 0);
c.mul_s(3, 1, 3);
c.mul_s(0, 1, 0);
c.mul_s(8, 8, 6);
c.mul_s(2, 2, 6);
c.mul_s(4, 4, 6);
c.mul_s(1, 1, 6);
c.add_s(5, 5, 8);
c.sub_s(0, 0, 13);
c.sub_s(2, 2, 12);
c.add_s(9, 9, 11);
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x98));
c.add_s(7, 7, 11);
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x94));
c.add_s(4, 4, 13);
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xa8));
c.sub_s(10, 10, 8);
c.swc1(9, (c.g[SP][0] as u32).wrapping_add(0xb8));
c.add_s(1, 1, 11);
c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x90));
c.add_s(3, 3, 12);
c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.swc1(10, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xa4));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xb4));
c.lqc2(4, (c.g[S1][0] as u32));
c.lqc2(5, (c.g[S1][0] as u32).wrapping_add(0x10));
c.lqc2(6, (c.g[S1][0] as u32).wrapping_add(0x20));
c.lqc2(7, (c.g[S1][0] as u32).wrapping_add(0x30));
c.lqc2(8, (c.g[S2][0] as u32));
c.lqc2(9, (c.g[S2][0] as u32).wrapping_add(0x10));
c.lqc2(10, (c.g[S2][0] as u32).wrapping_add(0x20));
c.lqc2(11, (c.g[S2][0] as u32).wrapping_add(0x30));
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
c.sqc2(12, (c.g[S1][0] as u32));
c.sqc2(13, (c.g[S1][0] as u32).wrapping_add(0x10));
c.sqc2(14, (c.g[S1][0] as u32).wrapping_add(0x20));
c.sqc2(15, (c.g[S1][0] as u32).wrapping_add(0x30));
c.mov_s(12, 25);
c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x228));
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x22c));
c.g[RA][0] = sx(0x0038baf8);
c.g[S0][0] = 0x500000;
call_ptr(c, 0x0031be50, 0x0038baf8)?;
c.lwc1(11, (c.g[SP][0] as u32).wrapping_add(0x22c));
c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xea0));
c.lwc1(3, (c.g[V0][0] as u32).wrapping_add(0x8));
c.sub_s(1, 21, 11);
c.lwc1(0, (c.g[S0][0] as u32).wrapping_sub(0xea0));
c.lwc1(6, (c.g[V0][0] as u32).wrapping_add(0x4));
c.lwc1(8, (c.g[SP][0] as u32).wrapping_add(0x228));
c.mul_s(2, 1, 3);
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xcc));
c.mul_s(4, 1, 0);
c.sw((c.g[SP][0] as u32).wrapping_add(0xc0), 0u64);
c.mul_s(1, 1, 6);
c.sw((c.g[SP][0] as u32).wrapping_add(0xc4), 0u64);
c.mul_s(13, 8, 3);
c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), 0u64);
c.mul_s(12, 8, 0);
c.sw((c.g[SP][0] as u32).wrapping_add(0x9c), 0u64);
c.mul_s(9, 2, 3);
c.sw((c.g[SP][0] as u32).wrapping_add(0xac), 0u64);
c.mul_s(10, 4, 3);
c.sw((c.g[SP][0] as u32).wrapping_add(0xbc), 0u64);
c.mul_s(5, 2, 0);
c.mul_s(7, 4, 0);
c.mul_s(3, 1, 3);
c.mul_s(0, 1, 0);
c.mul_s(8, 8, 6);
c.mul_s(2, 2, 6);
c.mul_s(4, 4, 6);
c.mul_s(1, 1, 6);
c.add_s(5, 5, 8);
c.sub_s(0, 0, 13);
c.sub_s(2, 2, 12);
c.add_s(9, 9, 11);
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x98));
c.add_s(7, 7, 11);
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x94));
c.add_s(4, 4, 13);
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xa8));
c.sub_s(10, 10, 8);
c.swc1(9, (c.g[SP][0] as u32).wrapping_add(0xb8));
c.add_s(1, 1, 11);
c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x90));
c.add_s(3, 3, 12);
c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0xa0));
c.swc1(10, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xa4));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xb4));
c.lqc2(4, (c.g[S1][0] as u32));
c.lqc2(5, (c.g[S1][0] as u32).wrapping_add(0x10));
c.lqc2(6, (c.g[S1][0] as u32).wrapping_add(0x20));
c.lqc2(7, (c.g[S1][0] as u32).wrapping_add(0x30));
c.lqc2(8, (c.g[S2][0] as u32));
c.lqc2(9, (c.g[S2][0] as u32).wrapping_add(0x10));
c.lqc2(10, (c.g[S2][0] as u32).wrapping_add(0x20));
c.lqc2(11, (c.g[S2][0] as u32).wrapping_add(0x30));
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
c.sqc2(12, (c.g[S1][0] as u32));
c.sqc2(13, (c.g[S1][0] as u32).wrapping_add(0x10));
c.sqc2(14, (c.g[S1][0] as u32).wrapping_add(0x20));
c.sqc2(15, (c.g[S1][0] as u32).wrapping_add(0x30));
c.g[AT][0] = 0x43000000;
c.mtc1(1, 0);
c.g[T6][0] = 0xfffffffffffffc1f;
c.g[T7][0] = 0xfffffffffffffffc;
c.g[T1][0] = 0xffffffffe0000000;
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x9c));
c.g[T1][0] = c.g[T1][0] | 0x3ff;
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x90));
c.g[T2][0] = 0xfffffffffe7f0000;
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x94));
c.g[T2][0] = c.g[T2][0] | 0xffff;
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x98));
c.g[S1][0] = 0x800000;
c.g[T3][0] = 0xffffffffffcf0000;
c.g[T4][0] = 0xfffffffffff00000;
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
c.g[T3][0] = c.g[T3][0] | 0xffff;
c.g[T4][0] = c.g[T4][0] | 0xfff;
c.g[T5][0] = 0x10000;
c.sq(2, (c.g[S5][0] as u32).wrapping_add(0x420));
c.g[T5][0] = c.g[T5][0] | 0x4000;
c.g[S2][0] = 0xffffffffffffff83;
c.g[S3][0] = 0xfffffffffffff07f;
c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x125c));
c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x10));
c.g[A2][0] = sx((c.g[S5][0] as u32).wrapping_add(0x45c));
c.g[A0][0] = c.g[S0][0];
c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x430));
c.g[A3][0] = 0u64;
c.g[T0][0] = c.g[S6][0];
c.g[S7][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4a64));
c.lwc1(1, (c.g[GP][0] as u32).wrapping_add(0x1260));
c.g[S4][0] = c.g[S0][0];
c.swc1(1, (c.g[S5][0] as u32).wrapping_add(0x434));
c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x1264));
c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x438));
c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
c.ldl(3, (c.g[V0][0] as u32).wrapping_add(0x7));
c.ldr(3, (c.g[V0][0] as u32));
c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0xf));
c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x8));
c.g[T8][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x1b));
c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x14));
c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x23));
c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x1c));
c.sw((c.g[V0][0] as u32).wrapping_add(0x24), c.g[T8][0]);
c.g[A1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x14));
c.sw((c.g[FP][0] as u32).wrapping_add(0xe84), c.g[V1][0]);
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
c.g[V0][0] = c.g[V0][0] & c.g[T6][0];
c.g[V0][0] = c.g[V0][0] | 0x40;
c.sw((c.g[A1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[V0][0] & c.g[T7][0];
c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[A1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[T1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[T1][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
c.g[V0][0] = c.g[V0][0] | c.g[S1][0];
c.sw((c.g[T1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[V0][0] & c.g[T3][0];
c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[A1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[V0][0] & c.g[T4][0];
c.g[V0][0] = c.g[V0][0] | c.g[T5][0];
c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[V0][0] & c.g[S2][0];
c.g[V0][0] = c.g[V0][0] | 0x4;
c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[A1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
c.g[V0][0] = c.g[V0][0] & c.g[S3][0];
c.g[V0][0] = c.g[V0][0] | 0x400;
c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
c.g[RA][0] = sx(0x0038bd84);
c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x458));
Callee { addr: 0x00374a88, f: ee::s12::BezierPatchCache_MarkUsed }.call(c, 0x0038bd84)?;
c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x3c9c));
c.g[A0][0] = c.g[S0][0];
c.g[A2][0] = sx((c.g[S5][0] as u32).wrapping_add(0x3ca0));
c.g[A3][0] = 0x1;
c.g[RA][0] = sx(0x0038bd9c);
c.g[T0][0] = c.g[S6][0];
Callee { addr: 0x00374a88, f: ee::s12::BezierPatchCache_MarkUsed }.call(c, 0x0038bd9c)?;
c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4a60));
c.g[A0][0] = c.g[S0][0];
c.g[A2][0] = c.g[S7][0];
c.g[A3][0] = 0x2;
c.g[RA][0] = sx(0x0038bdb4);
c.g[T0][0] = c.g[S6][0];
Callee { addr: 0x00374a88, f: ee::s12::BezierPatchCache_MarkUsed }.call(c, 0x0038bdb4)?;
c.g[A1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4d84));
c.g[A0][0] = c.g[S0][0];
c.g[A2][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4d88));
c.g[A3][0] = 0x2;
c.g[RA][0] = sx(0x0038bdcc);
c.g[T0][0] = c.g[S6][0];
Callee { addr: 0x00374ae0, f: ee::s12::BezierPatchCache_MarkUsed2 }.call(c, 0x0038bdcc)?;
if c.g[S6][0] != 0u64 {
c.g[A1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
break 'b_38bde0;
} else {
c.cycles += 3;
c.g[RA][0] = sx(0x0038bddc);
c.g[A0][0] = c.g[S4][0];
Callee { addr: 0x00374b38, f: ee::s12::BezierPatchCache_ReleaseUnused }.call(c, 0x0038bddc)?;
c.g[A1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
break 'b_38bde0;
}
}
'b_38bf34: {
c.cycles += 53;
c.g[V1][0] = 0xfffffffffffffc3f;
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x234));
c.g[A2][0] = 0xffffffffc0000000;
c.g[V0][0] = c.lw((c.g[A1][0] as u32));
c.g[A3][0] = 0xffffffffffffffe0;
c.g[S1][0] = 0u64;
c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
c.g[V0][0] = c.g[V0][0] | 0x40;
c.sw((c.g[A1][0] as u32), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[V0][0]);
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x238));
c.g[A1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1cc4));
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
c.g[V1][0] = c.g[V1][0] & 0x1f;
c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[A2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
c.g[A1][0] = c.g[A2][0];
c.g[V1][0] = c.lw((c.g[A2][0] as u32));
c.g[A3][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
c.g[T0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
c.g[V1][0] = c.g[V1][0] ^ c.g[V0][0];
c.g[V1][0] = c.g[V1][0] ^ c.g[A3][0];
c.g[V1][0] = c.g[V1][0] ^ c.g[T0][0];
c.g[A3][0] = sx(((c.g[V1][0] as u32) as i32 >> 8) as u32);
c.g[T0][0] = sx(((c.g[V1][0] as u32) as i32 >> 16) as u32);
c.g[A3][0] = sx((c.g[A3][0] as u32) << 5);
c.g[T0][0] = sx((c.g[T0][0] as u32) << 5);
c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[A2][0] = sx((c.g[V1][0] as u32) << 5);
c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32 >> 24) as u32);
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[A3][0] = sx(((c.g[A3][0] as u32) as i32 >> 8) as u32);
c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
c.g[A2][0] = c.g[A2][0] ^ c.g[A3][0];
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[T0][0] = sx(((c.g[T0][0] as u32) as i32 >> 16) as u32);
c.g[A2][0] = c.g[A2][0] ^ c.g[T0][0];
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
c.g[A2][0] = c.g[A2][0] ^ c.g[V0][0];
c.g[RA][0] = sx(0x0038bea4);
c.g[A2][0] = c.g[A2][0] & 0xff;
call_ptr(c, 0x00394ed0, 0x0038bea4)?;
c.sw((c.g[SP][0] as u32).wrapping_add(0x240), c.g[V0][0]);
c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4a60));
if (c.g[V0][0] as i64) <= 0 {
c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x3c9c));
break 'b_38bf34;
} else {
c.cycles += 3;
c.g[S0][0] = c.g[S7][0];
c.g[S2][0] = 0x600000;
'l_38bec0: loop {
'b_38bf04: {
c.cycles += 3;
c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0xc));
let t = (c.g[V0][0] as i64) >= 0;
c.g[A1][0] = c.lw((c.g[S0][0] as u32));
if t {
break 'b_38bf04;
} else {
c.cycles += 11;
c.g[T0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
c.g[A2][0] = 0x2;
c.g[A0][0] = c.g[S4][0];
c.g[A3][0] = c.g[S6][0];
c.g[T0][0] = c.g[T0][0] & c.g[S2][0];
c.g[RA][0] = sx(0x0038bee8);
c.g[T0][0] = (0u64 < c.g[T0][0]) as u64;
Callee { addr: 0x003747a0, f: ee::s12::BezierPatchCache_TessellatePatch }.call(c, 0x0038bee8)?;
c.g[V1][0] = sx((c.g[V0][0] as u32) << 16);
c.g[A2][0] = sx(((c.g[V1][0] as u32) as i32 >> 16) as u32);
let t = (c.g[A2][0] as i64) < 0;
c.sh((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
if t {
break 'b_38bf04;
} else {
c.cycles += 3;
c.g[A0][0] = c.g[S5][0];
c.g[RA][0] = sx(0x0038bf04);
c.g[A1][0] = 0x2;
Callee { addr: 0x0038c788, f: ee::s12::PS2BezierMan_BuildPatchPacket }.call(c, 0x0038bf04)?;
break 'b_38bf04;
}
}
}
c.cycles += 11;
c.g[A2][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0xc));
c.g[A3][0] = c.g[S0][0];
c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x230));
c.g[A0][0] = c.g[S5][0];
c.g[A1][0] = 0x2;
c.g[RA][0] = sx(0x0038bf20);
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
Callee { addr: 0x0038ca70, f: ee::s12::PS2BezierMan_EmitPatchType1 }.call(c, 0x0038bf20)?;
c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4a60));
c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
if t {
if event_due(c) { deliver_event_here(c, 0x0038bec0); } continue 'l_38bec0;
} else {
c.cycles += 1;
c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x3c9c));
break 'b_38bf34;
}
}
}
}
'b_38bfb8: {
c.cycles += 2;
let t = (c.g[V0][0] as i64) <= 0;
c.g[S1][0] = 0u64;
if t {
break 'b_38bfb8;
} else {
c.cycles += 3;
c.g[S2][0] = 0x600000;
c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x3ca0));
'l_38bf48: loop {
'b_38bf8c: {
c.cycles += 3;
c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0xc));
let t = (c.g[V0][0] as i64) >= 0;
c.g[A1][0] = c.lw((c.g[S0][0] as u32));
if t {
break 'b_38bf8c;
} else {
c.cycles += 11;
c.g[T0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
c.g[A2][0] = 0x1;
c.g[A0][0] = c.g[S4][0];
c.g[A3][0] = c.g[S6][0];
c.g[T0][0] = c.g[T0][0] & c.g[S2][0];
c.g[RA][0] = sx(0x0038bf70);
c.g[T0][0] = (0u64 < c.g[T0][0]) as u64;
Callee { addr: 0x003747a0, f: ee::s12::BezierPatchCache_TessellatePatch }.call(c, 0x0038bf70)?;
c.g[V1][0] = sx((c.g[V0][0] as u32) << 16);
c.g[A2][0] = sx(((c.g[V1][0] as u32) as i32 >> 16) as u32);
let t = (c.g[A2][0] as i64) < 0;
c.sh((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
if t {
break 'b_38bf8c;
} else {
c.cycles += 3;
c.g[A0][0] = c.g[S5][0];
c.g[RA][0] = sx(0x0038bf8c);
c.g[A1][0] = 0x1;
Callee { addr: 0x0038c788, f: ee::s12::PS2BezierMan_BuildPatchPacket }.call(c, 0x0038bf8c)?;
break 'b_38bf8c;
}
}
}
c.cycles += 11;
c.g[A2][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0xc));
c.g[A3][0] = c.g[S0][0];
c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x230));
c.g[A0][0] = c.g[S5][0];
c.g[A1][0] = 0x1;
c.g[RA][0] = sx(0x0038bfa8);
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
Callee { addr: 0x0038ca70, f: ee::s12::PS2BezierMan_EmitPatchType1 }.call(c, 0x0038bfa8)?;
c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x3c9c));
c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
if t {
if event_due(c) { deliver_event_here(c, 0x0038bf48); } continue 'l_38bf48;
} else {
break 'b_38bfb8;
}
}
}
}
'b_38c030: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x458));
let t = (c.g[V0][0] as i64) <= 0;
c.g[S1][0] = 0u64;
if t {
break 'b_38c030;
} else {
c.cycles += 1;
c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x45c));
'l_38bfc8: loop {
'b_38c004: {
c.cycles += 3;
c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0xc));
let t = (c.g[V0][0] as i64) >= 0;
c.g[A1][0] = c.lw((c.g[S0][0] as u32));
if t {
break 'b_38c004;
} else {
c.cycles += 9;
c.g[A2][0] = 0u64;
c.g[A0][0] = c.g[S4][0];
c.g[A3][0] = c.g[S6][0];
c.g[RA][0] = sx(0x0038bfe8);
c.g[T0][0] = 0u64;
Callee { addr: 0x003747a0, f: ee::s12::BezierPatchCache_TessellatePatch }.call(c, 0x0038bfe8)?;
c.g[V1][0] = sx((c.g[V0][0] as u32) << 16);
c.g[A2][0] = sx(((c.g[V1][0] as u32) as i32 >> 16) as u32);
let t = (c.g[A2][0] as i64) < 0;
c.sh((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
if t {
break 'b_38c004;
} else {
c.cycles += 3;
c.g[A0][0] = c.g[S5][0];
c.g[RA][0] = sx(0x0038c004);
c.g[A1][0] = 0u64;
Callee { addr: 0x0038c788, f: ee::s12::PS2BezierMan_BuildPatchPacket }.call(c, 0x0038c004)?;
break 'b_38c004;
}
}
}
c.cycles += 11;
c.g[A2][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0xc));
c.g[A3][0] = c.g[S0][0];
c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x230));
c.g[A0][0] = c.g[S5][0];
c.g[A1][0] = 0u64;
c.g[RA][0] = sx(0x0038c020);
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
Callee { addr: 0x0038ca70, f: ee::s12::PS2BezierMan_EmitPatchType1 }.call(c, 0x0038c020)?;
c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x458));
c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
if t {
if event_due(c) { deliver_event_here(c, 0x0038bfc8); } continue 'l_38bfc8;
} else {
break 'b_38c030;
}
}
}
}
'b_38c0ec: {
c.cycles += 14;
c.g[A1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
c.g[V1][0] = 0xfffffffffffffc3f;
c.g[A2][0] = 0xffffffffc0000000;
c.g[V0][0] = c.lw((c.g[A1][0] as u32));
c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
c.g[V0][0] = c.g[V0][0] | 0x80;
c.sw((c.g[A1][0] as u32), c.g[V0][0]);
c.g[A0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
c.g[V0][0] = c.g[V0][0] & c.g[A2][0];
c.sw((c.g[A0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4d84));
let t = (c.g[V1][0] as i64) <= 0;
c.g[S2][0] = 0u64;
if t {
break 'b_38c0ec;
} else {
c.cycles += 4;
c.g[S3][0] = 0x600000;
c.g[S1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4d98));
c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4d88));
'l_38c078: loop {
'b_38c0bc: {
c.cycles += 3;
c.g[V0][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0xc));
let t = (c.g[V0][0] as i64) >= 0;
c.g[A1][0] = c.lw((c.g[S0][0] as u32));
if t {
break 'b_38c0bc;
} else {
c.cycles += 11;
c.g[T0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
c.g[A2][0] = 0x2;
c.g[A0][0] = c.g[S4][0];
c.g[A3][0] = c.g[S6][0];
c.g[T0][0] = c.g[T0][0] & c.g[S3][0];
c.g[RA][0] = sx(0x0038c0a0);
c.g[T0][0] = (0u64 < c.g[T0][0]) as u64;
Callee { addr: 0x003747a0, f: ee::s12::BezierPatchCache_TessellatePatch }.call(c, 0x0038c0a0)?;
c.g[V1][0] = sx((c.g[V0][0] as u32) << 16);
c.g[A2][0] = sx(((c.g[V1][0] as u32) as i32 >> 16) as u32);
let t = (c.g[A2][0] as i64) < 0;
c.sh((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
if t {
break 'b_38c0bc;
} else {
c.cycles += 3;
c.g[A0][0] = c.g[S5][0];
c.g[RA][0] = sx(0x0038c0bc);
c.g[A1][0] = 0x2;
Callee { addr: 0x0038c788, f: ee::s12::PS2BezierMan_BuildPatchPacket }.call(c, 0x0038c0bc)?;
break 'b_38c0bc;
}
}
}
c.cycles += 12;
c.g[A1][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0xc));
c.g[A2][0] = c.g[S0][0];
c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x230));
c.g[A3][0] = c.g[S1][0];
c.g[A0][0] = c.g[S5][0];
c.g[RA][0] = sx(0x0038c0d8);
c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
Callee { addr: 0x0038ce20, f: ee::s12::PS2BezierMan_EmitPatchType2 }.call(c, 0x0038c0d8)?;
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x14));
c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x4d84));
c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
if t {
if event_due(c) { deliver_event_here(c, 0x0038c078); } continue 'l_38c078;
} else {
break 'b_38c0ec;
}
}
}
}
'b_38c4f8: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5364));
let t = c.g[V0][0] == 0u64;
c.g[T5][0] = 0xfffffffffffffff3;
if t {
break 'b_38c4f8;
} else {
'b_38c270: {
c.cycles += 50;
c.g[V1][0] = c.lh((c.g[S5][0] as u32).wrapping_add(0x5368));
c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
c.g[S4][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5368));
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x234));
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x240));
c.g[A2][0] = c.g[S4][0];
c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
c.g[V1][0] = sx(((c.g[S4][0] as u32) as i32 >> 16) as u32);
c.g[V0][0] = sx((c.g[S4][0] as u32) << 16);
c.g[T1][0] = c.lh((c.g[S5][0] as u32).wrapping_add(0x536a));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2));
c.g[A3][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 16);
c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x23c));
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
c.sh((c.g[A3][0] as u32).wrapping_add(0x12), c.g[T1][0]);
c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
c.g[T4][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x240));
c.g[T3][0] = c.lw((c.g[T4][0] as u32));
c.g[V0][0] = c.g[V1][0] ^ c.g[V0][0];
c.g[T1][0] = sx(((c.g[V0][0] as u32) as i32 >> 8) as u32);
c.g[T3][0] = c.g[T3][0] & c.g[T5][0];
c.sw((c.g[T4][0] as u32), c.g[T3][0]);
c.g[T2][0] = sx(((c.g[V0][0] as u32) as i32 >> 16) as u32);
c.g[T1][0] = sx((c.g[T1][0] as u32) << 5);
c.g[V1][0] = sx((c.g[V0][0] as u32) << 5);
c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A3][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
c.g[T2][0] = sx((c.g[T2][0] as u32) << 5);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[T3][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
c.g[T1][0] = sx(((c.g[T1][0] as u32) as i32 >> 8) as u32);
c.g[A3][0] = sx((c.g[A3][0] as u32) << 5);
c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[V1][0] = c.g[V1][0] ^ c.g[T1][0];
c.g[T2][0] = sx(((c.g[T2][0] as u32) as i32 >> 16) as u32);
c.g[A3][0] = sx(((c.g[A3][0] as u32) as i32 >> 24) as u32);
c.g[V0][0] = c.lw((c.g[T3][0] as u32));
c.g[V1][0] = c.g[V1][0] ^ c.g[T2][0];
c.g[V1][0] = c.g[V1][0] ^ c.g[A3][0];
c.g[S1][0] = c.g[V1][0] & 0xff;
c.g[V0][0] = c.g[V0][0] | 0x30;
c.sw((c.g[T3][0] as u32), c.g[V0][0]);
c.g[RA][0] = sx(0x0038c1b4);
c.g[A3][0] = c.g[S1][0];
call_ptr(c, 0x00395000, 0x0038c1b4)?;
c.g[S0][0] = c.g[V0][0];
let t = c.g[S0][0] != 0u64;
c.g[A3][0] = c.g[S0][0];
if t {
break 'b_38c270;
} else {
c.cycles += 44;
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x234));
c.g[A3][0] = 0x50000;
c.g[A2][0] = 0x50000;
c.g[A1][0] = c.g[S4][0];
c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
c.g[A2][0] = c.g[A2][0] | 0x7498;
c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x7494));
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x240));
c.g[V0][0] = sx((c.g[V1][0] as u32) << 5);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.sw((c.g[A3][0] as u32).wrapping_add(0x7494), c.g[V1][0]);
c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x240));
c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x234));
c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.sw((c.g[V0][0] as u32), c.g[A3][0]);
c.g[S0][0] = c.g[V0][0];
c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[S4][0]);
c.g[T8][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x23c));
c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0u64);
c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[T8][0]);
c.sw((c.g[V0][0] as u32).wrapping_add(0x14), 0u64);
c.g[RA][0] = sx(0x0038c220);
c.sw((c.g[V0][0] as u32).wrapping_add(0x18), 0u64);
Callee { addr: 0x00364240, f: ee::s12::DeferMan_ComputeSortKey }.call(c, 0x0038c220)?;
c.g[T9][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x234));
c.g[A1][0] = 0x60000;
c.g[A2][0] = sx((c.g[S1][0] as u32) << 2);
c.g[A3][0] = c.g[S0][0];
c.g[A1][0] = sx((c.g[T9][0] as u32).wrapping_add((c.g[A1][0] as u32)));
c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x78a0));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
c.g[V1][0] = c.lw((c.g[A0][0] as u32));
c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
c.sw((c.g[A0][0] as u32), c.g[S0][0]);
c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x7ca4));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.sw((c.g[V1][0] as u32).wrapping_add(0x7ca8), c.g[V0][0]);
c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x7ca4));
c.g[V0][0] = sx((c.g[A0][0] as u32) << 3);
c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
c.sw((c.g[V0][0] as u32).wrapping_add(0x7cac), c.g[S0][0]);
c.sw((c.g[A1][0] as u32).wrapping_add(0x7ca4), c.g[A0][0]);
break 'b_38c270;
}
}
'b_38c4c4: {
c.cycles += 7;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x230));
c.g[V1][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5364));
c.g[S7][0] = c.lw((c.g[V0][0] as u32));
c.g[V0][0] = 0x30000000;
c.sw((c.g[SP][0] as u32).wrapping_add(0x244), 0u64);
let t = (c.g[V1][0] as i64) <= 0;
c.g[S1][0] = c.g[S7][0] | c.g[V0][0];
if t {
break 'b_38c4c4;
} else {
c.cycles += 7;
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x234));
c.g[V0][0] = 0x50000;
c.g[V1][0] = 0x60000;
c.g[S3][0] = sx((c.g[S5][0] as u32).wrapping_add(0x5368));
c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[S6][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.sw((c.g[SP][0] as u32).wrapping_add(0x248), c.g[V0][0]);
'l_38c2a8: loop {
'b_38c438: {
c.cycles += 3;
c.g[S2][0] = c.lw((c.g[S3][0] as u32));
let t = c.g[S2][0] == c.g[S4][0];
c.g[A0][0] = c.g[A3][0];
if t {
break 'b_38c438;
} else {
'b_38c434: {
c.cycles += 56;
c.g[V0][0] = 0xfff0000;
c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x230));
c.g[V0][0] = c.g[V0][0] | 0xffff;
c.g[V0][0] = c.g[S1][0] & c.g[V0][0];
c.g[A1][0] = c.g[S7][0];
c.g[A2][0] = c.g[V0][0];
c.g[RA][0] = sx(0x0038c2d4);
c.sw((c.g[A3][0] as u32), c.g[V0][0]);
Callee { addr: 0x00362978, f: ee::s12::DeferMan_AppendPacket }.call(c, 0x0038c2d4)?;
c.g[V0][0] = sx(((c.g[S2][0] as u32) as i32 >> 16) as u32);
c.g[T8][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x230));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
c.g[A0][0] = sx((c.g[S2][0] as u32) << 16);
c.g[T9][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x240));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 16) as u32);
c.g[T0][0] = c.lw((c.g[T8][0] as u32));
c.g[A0][0] = c.g[A0][0] | c.g[V0][0];
c.g[T2][0] = 0x30000000;
c.g[A0][0] = c.g[T9][0] ^ c.g[A0][0];
c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x10));
c.g[A1][0] = sx(((c.g[A0][0] as u32) as i32 >> 8) as u32);
c.sw((c.g[T8][0] as u32), c.g[T0][0]);
c.g[A2][0] = sx(((c.g[A0][0] as u32) as i32 >> 16) as u32);
c.g[A1][0] = sx((c.g[A1][0] as u32) << 5);
c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[V1][0] = sx((c.g[A0][0] as u32) << 5);
c.g[V0][0] = sx(((c.g[A0][0] as u32) as i32 >> 24) as u32);
c.g[A2][0] = sx((c.g[A2][0] as u32) << 5);
c.g[A3][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[T1][0] = c.lh((c.g[S3][0] as u32));
c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 8) as u32);
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
c.g[V1][0] = c.g[V1][0] ^ c.g[A1][0];
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 16) as u32);
c.sh((c.g[A3][0] as u32).wrapping_add(0x10), c.g[T1][0]);
c.g[V1][0] = c.g[V1][0] ^ c.g[A2][0];
c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 24) as u32);
c.g[V1][0] = c.g[V1][0] ^ c.g[V0][0];
c.g[A1][0] = c.lh((c.g[S3][0] as u32).wrapping_add(0x2));
c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
c.g[S7][0] = c.g[T0][0];
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x234));
c.g[S4][0] = c.g[V1][0] & 0xff;
c.sh((c.g[V0][0] as u32).wrapping_add(0x12), c.g[A1][0]);
c.g[S1][0] = c.g[S7][0] | c.g[T2][0];
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x240));
c.g[A2][0] = c.g[S2][0];
c.g[T0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x23c));
c.g[RA][0] = sx(0x0038c388);
c.g[A3][0] = c.g[S4][0];
call_ptr(c, 0x00395000, 0x0038c388)?;
c.g[S0][0] = c.g[V0][0];
let t = c.g[S0][0] != 0u64;
c.g[A3][0] = c.g[S0][0];
if t {
break 'b_38c434;
} else {
c.cycles += 40;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x248));
c.g[V1][0] = 0x50000;
c.g[V1][0] = c.g[V1][0] | 0x7498;
c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x248));
c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x7494));
c.g[A1][0] = c.g[S2][0];
c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x240));
c.g[V0][0] = sx((c.g[A2][0] as u32) << 5);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x234));
c.sw((c.g[A3][0] as u32).wrapping_add(0x7494), c.g[A2][0]);
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[T8][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x240));
c.g[S0][0] = c.g[V0][0];
c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[S2][0]);
c.sw((c.g[V0][0] as u32), c.g[T8][0]);
c.g[T9][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x23c));
c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0u64);
c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[T9][0]);
c.sw((c.g[V0][0] as u32).wrapping_add(0x14), 0u64);
c.g[RA][0] = sx(0x0038c3f0);
c.sw((c.g[V0][0] as u32).wrapping_add(0x18), 0u64);
Callee { addr: 0x00364240, f: ee::s12::DeferMan_ComputeSortKey }.call(c, 0x0038c3f0)?;
c.g[V1][0] = sx((c.g[S4][0] as u32) << 2);
c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x78a0));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.g[A3][0] = c.g[S0][0];
c.g[V1][0] = c.lw((c.g[A0][0] as u32));
c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
c.sw((c.g[A0][0] as u32), c.g[S0][0]);
c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x7ca4));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.sw((c.g[V1][0] as u32).wrapping_add(0x7ca8), c.g[V0][0]);
c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x7ca4));
c.g[V0][0] = sx((c.g[A0][0] as u32) << 3);
c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
c.sw((c.g[V0][0] as u32).wrapping_add(0x7cac), c.g[S0][0]);
c.sw((c.g[S6][0] as u32).wrapping_add(0x7ca4), c.g[A0][0]);
break 'b_38c434;
}
}
c.cycles += 1;
c.g[S4][0] = c.g[S2][0];
break 'b_38c438;
}
}
c.cycles += 35;
c.g[V0][0] = c.lwu((c.g[S3][0] as u32).wrapping_add(0x8));
c.g[A0][0] = 0x50000000;
c.g[V1][0] = 0x10000000;
c.g[V1][0] = c.g[V1][0] | 0x1;
c.sd((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
c.g[V0][0] = c.g[V0][0] << 32;
c.sd((c.g[S1][0] as u32).wrapping_add(0x10), c.g[V1][0]);
c.sd((c.g[S1][0] as u32).wrapping_add(0x18), 0u64);
c.g[V0][0] = c.g[V0][0] | c.g[A0][0];
c.sd((c.g[S1][0] as u32), c.g[V0][0]);
c.g[A1][0] = 0x14000000;
c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x244));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
c.sw((c.g[SP][0] as u32).wrapping_add(0x244), c.g[V0][0]);
c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x10));
c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
c.g[A0][0] = 0x450000;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4630));
c.g[V1][0] = sx((c.g[V1][0] as u32) << 3);
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[A0][0] = c.lw((c.g[V0][0] as u32));
c.sd((c.g[S1][0] as u32).wrapping_add(0x20), 0u64);
c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
c.g[V0][0] = c.g[V0][0] | c.g[A1][0];
c.g[V0][0] = c.g[V0][0] << 32;
c.sd((c.g[S1][0] as u32).wrapping_add(0x28), c.g[V0][0]);
c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x5364));
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x244));
c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
let t = c.g[V0][0] != 0u64;
c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x30));
if t {
if event_due(c) { deliver_event_here(c, 0x0038c2a8); } continue 'l_38c2a8;
} else {
break 'b_38c4c4;
}
}
}
}
c.cycles += 13;
c.g[A0][0] = c.g[A3][0];
c.g[V0][0] = 0xfff0000;
c.g[V0][0] = c.g[V0][0] | 0xffff;
c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x230));
c.g[V0][0] = c.g[S1][0] & c.g[V0][0];
c.g[A1][0] = c.g[S7][0];
c.g[A2][0] = c.g[V0][0];
c.g[RA][0] = sx(0x0038c4e8);
c.sw((c.g[A3][0] as u32), c.g[V0][0]);
Callee { addr: 0x00362978, f: ee::s12::DeferMan_AppendPacket }.call(c, 0x0038c4e8)?;
c.g[T8][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x230));
c.g[V0][0] = c.lw((c.g[T8][0] as u32));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
c.sw((c.g[T8][0] as u32), c.g[V0][0]);
break 'b_38c4f8;
}
}
c.cycles += 38;
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
c.g[A2][0] = 0xffffffffffffffff;
c.g[A3][0] = 0xfffffffffffffc3f;
c.g[T0][0] = 0xffffffffc0000000;
c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x14));
c.g[A0][0] = c.g[S5][0];
c.sw((c.g[FP][0] as u32).wrapping_add(0xe84), c.g[V0][0]);
c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x230));
c.sh((c.g[V1][0] as u32).wrapping_sub(0x4), c.g[A2][0]);
c.g[V0][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
c.sh((c.g[V0][0] as u32).wrapping_add(0x12), c.g[A2][0]);
c.g[V1][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[V1][0] as u32));
c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
c.sw((c.g[V1][0] as u32), c.g[V0][0]);
c.g[A2][0] = c.lw((c.g[FP][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
c.g[RA][0] = sx(0x0038c548);
c.sw((c.g[A2][0] as u32).wrapping_add(0xc), c.g[V0][0]);
Callee { addr: 0x0038d968, f: ee::s12::PS2BezierMan_RenderPatchFX }.call(c, 0x0038c548)?;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2e0));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x2d0));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x2c0));
c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x2b0));
c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x2a0));
c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x290));
c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x280));
c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x270));
c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x260));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x250));
c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x318));
c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x310));
c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x308));
c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x300));
c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x2f8));
c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x2f0));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x320));
c.pc = t; return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{elf_image, fakes, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Gate word of this module's fakes, and the steering bits (own address and
    /// magic in the machine's RAM: other tests' machines run the real bodies).
    const OC1467_CTRL: u32 = DATA + 0x20_0000;
    const OC1467_MAGIC: u32 = 0x1467_f00d;
    const OC1467_STEER: u32 = OC1467_CTRL + 4;
    /// Fixture: the graphics manager, the display state stack, the bezier
    /// manager, the object the view matrix comes from, patch records, the
    /// packet cursor, and the deferred manager.
    const OC1467_GM: u32 = DATA + 0x5_0000;
    const OC1467_STATE: u32 = DATA + 0x5_8000;
    const OC1467_BM: u32 = DATA + 0x5_9000;
    const OC1467_OBJ: u32 = DATA + 0x5_f000;
    const OC1467_PTRS: u32 = DATA + 0x5_f400;
    const OC1467_CURSOR: u32 = DATA + 0x5_f800;
    const OC1467_PACKETS: u32 = DATA + 0x5_fa00;
    const OC1467_DM: u32 = DATA + 0x20_1000;

    macro_rules! oc1467_fakes {
        ($($name:ident, $addr:expr, $slot:path, $gen:path;)*) => {
            $(
                fn $name(c: &mut Cpu) {
                    if c.bus.read32(OC1467_CTRL) != OC1467_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                    }
                    fakes::fake_call(c, $addr);
                    oc1467_after(c, $addr);
                }
            )*
            /// Puts every fake in its override slot (once per process).
            fn oc1467_hook() {
                $( fakes::hook(addr_of_mut!($slot), $name, fakes::prev_cell($name)); )*
            }
        };
    }

    /// The bucket finder answers "none" for some steering bits.
    fn oc1467_after(c: &mut Cpu, target: u32) {
        // The generated body clears the deferred count (bm+0x5364) first: the
        // emitters (faked) are what fill it, so a steered fake does so here.
        if matches!(target, 0x0038_ca70 | 0x0038_ce20) {
            let n = c.bus.read32(fakes::LOG_COUNT).wrapping_sub(1);
            let s = c.bus.read32(OC1467_STEER) >> ((n + 5) % 28);
            if s & 1 != 0 {
                c.bus.write32(OC1467_BM + 0x5364, 1 + (s >> 1) % 3);
            }
        }
        if target == 0x0039_5000 {
            let n = c.bus.read32(fakes::LOG_COUNT).wrapping_sub(1);
            if c.bus.read32(OC1467_STEER) >> (n % 32) & 1 != 0 {
                c.g[V0][0] = 0;
            }
        }
    }

    oc1467_fakes! {
        oc1467_fake_0031be50, 0x0031_be50, ee::s10::OVERRIDE_0031BE50, ee::s10::f_0031be50__generated;
        oc1467_fake_00374a88, 0x0037_4a88, ee::s12::OVERRIDE_00374A88, ee::s12::f_00374a88__generated;
        oc1467_fake_00374ae0, 0x0037_4ae0, ee::s12::OVERRIDE_00374AE0, ee::s12::f_00374ae0__generated;
        oc1467_fake_00374b38, 0x0037_4b38, ee::s12::OVERRIDE_00374B38, ee::s12::f_00374b38__generated;
        oc1467_fake_00394ed0, 0x0039_4ed0, ee::s13::OVERRIDE_00394ED0, ee::s13::f_00394ed0__generated;
        oc1467_fake_003747a0, 0x0037_47a0, ee::s12::OVERRIDE_003747A0, ee::s12::f_003747a0__generated;
        oc1467_fake_0038c788, 0x0038_c788, ee::s12::OVERRIDE_0038C788, ee::s12::f_0038c788__generated;
        oc1467_fake_0038ca70, 0x0038_ca70, ee::s12::OVERRIDE_0038CA70, ee::s12::f_0038ca70__generated;
        oc1467_fake_0038ce20, 0x0038_ce20, ee::s12::OVERRIDE_0038CE20, ee::s12::f_0038ce20__generated;
        oc1467_fake_00395000, 0x0039_5000, ee::s13::OVERRIDE_00395000, ee::s13::f_00395000__generated;
        oc1467_fake_00364240, 0x0036_4240, ee::s12::OVERRIDE_00364240, ee::s12::f_00364240__generated;
        oc1467_fake_00362978, 0x0036_2978, ee::s12::OVERRIDE_00362978, ee::s12::f_00362978__generated;
        oc1467_fake_0038d968, 0x0038_d968, ee::s12::OVERRIDE_0038D968, ee::s12::f_0038d968__generated;
        oc1467_fake_0037d968, 0x0037_d968, ee::s12::OVERRIDE_0037D968, ee::s12::f_0037d968__generated;
    }

    /// The terrain end-of-pass fixture: hooked fakes, the game image, the
    /// managers, and queues of 0..3 patches (counts and records random).
    fn oc1467_setup(c: &mut Cpu, seed: u32) {
        oc1467_hook();
        let mut r = fakes::machine(c, seed, 0x1467, 16);
        elf_image(c);
        let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
        w(c, OC1467_CTRL, OC1467_MAGIC);
        w(c, OC1467_STEER, r.next_u32());
        w(c, GAME_GP + 0x2a90, OC1467_GM);
        w(c, OC1467_GM + 0xe84, OC1467_STATE);
        w(c, OC1467_GM + 0x18f0, OC1467_DM);
        w(c, OC1467_GM + 0x6b90, r.next_u32() & 1);
        w(c, OC1467_GM + 0x274, r.next_u32() & 1);
        w(c, OC1467_GM + 0x10d8, OC1467_OBJ);
        w(c, OC1467_GM + 0x5a00, OC1467_CURSOR);
        w(c, OC1467_OBJ + 0x118, 0x100);
        w(c, OC1467_OBJ + 0x11c, fakes::FAKE);
        w(c, OC1467_CURSOR, OC1467_PACKETS);
        crate::difftest::fill(c, &mut r, OC1467_STATE - 0x20, 12);
        crate::difftest::fill(c, &mut r, OC1467_PTRS, 0x100);
        crate::difftest::fill(c, &mut r, OC1467_DM + 0x5_0000 + 0x7490, 0x20);
        crate::difftest::fill(c, &mut r, OC1467_DM + 0x6_0000 + 0x7ca0, 0x10);
        for q in [0x458, 0x3c9c, 0x4a60, 0x4d84, 0x5364] {
            let n = r.next_u32() % 5;
            w(c, OC1467_BM + q, n.wrapping_sub(1));
            let step = if q == 0x4d84 { 0x14 } else { 0x10 };
            let first = if q == 0x5364 { 0x5368 } else { q + 4 };
            for i in 0..4u32 {
                let e = OC1467_BM + first + step * i;
                // Word 0 of the deferred records takes few values so that
                // neighbours sometimes match.
                let w0 = if q == 0x5364 {
                    0x0003_0000 | (r.next_u32() % 3)
                } else {
                    OC1467_PTRS + (r.next_u32() & 0x3f0)
                };
                w(c, e, w0);
                w(c, e + 4, r.next_u32() & 0x3);
                w(c, e + 8, r.next_u32());
                w(c, e + 0xc, if q == 0x5364 { OC1467_PTRS + (r.next_u32() & 0x3f0) } else { r.next_u32() });
            }
        }
        // The deferred records' state word 0 starts the dedupe key.
        c.g[A0][0] = sx(OC1467_BM);
        c.pc = 0x0038_b370;
    }

    /// `PS2BezierMan_v18(bm)`: the end of the terrain pass (state words, the
    /// shine and glitter matrices, the patch queues' emitters, the deferred
    /// groups), against the generated body with gated fakes for its callees;
    /// every call arm must be reached by some seed.
    #[test]
    fn ps2_bezier_man_v18_matches() {
        fakes::check_reaching(
            ee::s12::f_0038b370__generated,
            ps2_bezier_man_v18,
            300,
            &[
            0x0038_b408, 0x0038_b438, 0x0038_b58c, 0x0038_bd84, 0x0038_bdb4, 0x0038_bdcc,
            0x0038_bddc, 0x0038_bea4, 0x0038_bee8, 0x0038_bf04, 0x0038_bf20, 0x0038_c0a0,
            0x0038_c0d8, 0x0038_c1b4, 0x0038_c220, 0x0038_c2d4, 0x0038_c388, 0x0038_c4e8,
            0x0038_c548,
            ],
            &|c, seed| oc1467_setup(c, seed),
        );
    }

    /// The seeds take many distinct paths.
    #[test]
    fn ps2_bezier_man_v18_paths() {
        use crate::difftest::{fresh, run_generated};
        let mut totals = std::collections::BTreeSet::new();
        for seed in 0..300 {
            let mut c = fresh();
            oc1467_setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, ee::s12::f_0038b370__generated);
            totals.insert(c.cycles - before);
        }
        assert!(totals.len() >= 12, "only {} distinct paths", totals.len());
    }
}
