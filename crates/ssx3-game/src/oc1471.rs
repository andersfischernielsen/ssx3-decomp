//! Staging module oc1471 (wave wave274): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1471`.

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
    0x0016_4878,
    chase_camera_algorithm_apply_air_swing,
    "ChaseCameraAlgorithm_ApplyAirSwing",
)];

/// `ChaseCameraAlgorithm_ApplyAirSwing` (0x00164878): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn chase_camera_algorithm_apply_air_swing(c: &mut Cpu) {
    if c.pc != 0x00164878 {
        return ee::s01::f_00164878__generated(c);
    }
    guest(c, |c| {
'b_16531c: {
'b_165318: {
'b_16518c: {
c.cycles += 21;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xf0));
c.sw((c.g[GP][0] as u32).wrapping_sub(0x1e64), 0u64);
c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[S1][0] = c.g[A0][0];
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
c.g[S2][0] = c.g[A1][0];
c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.mov_s(20, 12);
c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[RA][0]);
c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xe8));
c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[S1][0] as u32).wrapping_add(0x40));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
c.lq(3, (c.g[S1][0] as u32).wrapping_add(0x40));
c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2d0));
c.sq(2, (c.g[SP][0] as u32));
let t = c.g[A0][0] == 0u64;
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x10));
if t {
break 'b_16518c;
} else {
'b_164fb0: {
c.cycles += 3;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2d8));
if c.g[V0][0] != 0u64 {
c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x234));
break 'b_164fb0;
} else {
'b_164fa4: {
c.cycles += 12;
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x22c));
c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x6920));
c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
c.add_s(0, 0, 1);
c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x22c));
c.g[V1][0] = c.lw((c.g[A1][0] as u32));
c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x98));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x9c));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x00164900);
c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x00164900)?;
if c.g[V0][0] == 0u64 {
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2d8));
break 'b_164fa4;
} else {
'b_1649a4: {
c.cycles += 31;
c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[S1][0] as u32).wrapping_add(0x40));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x230));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x110));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(0x110));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.vu0().sqrt(4, 0);
c.vu0().waitq();
c.cfc2(3, 22);
c.mtc1(3, 21);
c.g[V0][0] = c.lw((c.g[A1][0] as u32));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x60));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x64));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x00164968);
c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x00164968)?;
c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x22c));
c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x691c));
c.sub_s(0, 0, 1);
c.c_le_s(2, 0);
if !c.fcc() {
c.mov_s(0, 2);
break 'b_1649a4;
} else {
c.cycles += 8;
c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
c.g[V0][0] = c.lw((c.g[A1][0] as u32));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x60));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x64));
let t = c.g[V1][0] as u32;
c.g[RA][0] = sx(0x0016499c);
c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0016499c)?;
c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x22c));
c.sub_s(0, 0, 1);
break 'b_1649a4;
}
}
'b_164a88: {
c.cycles += 50;
c.swc1(0, (c.g[S0][0] as u32));
c.swc1(20, (c.g[S1][0] as u32).wrapping_add(0x23c));
c.sw((c.g[S1][0] as u32).wrapping_add(0x234), 0u64);
c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
c.g[V1][0] = c.lw((c.g[A1][0] as u32));
c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x80));
c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
let t = c.g[V0][0] as u32;
c.g[RA][0] = sx(0x001649c8);
c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x001649c8)?;
c.g[A0][0] = c.g[V0][0];
c.g[A2][0] = 0x500000;
c.lqc2(4, (c.g[A2][0] as u32).wrapping_sub(0xea0));
c.lqc2(5, (c.g[A0][0] as u32));
c.vu0().opmula(4, 5);
c.vu0().opmsub(6, 5, 4);
c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
c.sqc2(6, (c.g[SP][0] as u32).wrapping_add(0x30));
c.mtc1(0, 3);
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.g[A1][0] = c.g[A0][0];
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.vu0().sqrt(4, 0);
c.vu0().waitq();
c.cfc2(3, 22);
c.mtc1(3, 0);
c.mfc1(2, 0);
c.qmtc2(2, 3);
c.vu0().div(0, 3, 3, 0);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
c.vu0().waitq();
c.vu0().mul(0b1111, 5, 4, Src::Q);
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lqc2(5, (c.g[S2][0] as u32).wrapping_add(0x20));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.qmfc2(3, 4);
c.mtc1(3, 1);
c.c_lt_s(1, 3);
let t = !c.fcc();

if t {
break 'b_164a88;
} else {
c.cycles += 7;
c.g[AT][0] = 0xffffffffbf800000;
c.mtc1(1, 0);
c.mfc1(2, 0);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
c.qmtc2(2, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
break 'b_164a88;
}
}
'b_164bb4: {
c.cycles += 74;
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.vu0().sqrt(4, 0);
c.vu0().waitq();
c.cfc2(2, 22);
c.mtc1(2, 0);
c.g[AT][0] = 0x40000000;
c.mtc1(1, 2);
c.div_s(0, 21, 0);
c.mfc1(2, 0);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
c.qmtc2(2, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lqc2(3, (c.g[A0][0] as u32));
c.lqc2(5, (c.g[A0][0] as u32));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.qmfc2(2, 4);
c.mtc1(2, 0);
c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(0x110));
c.lqc2(5, (c.g[A0][0] as u32));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.qmfc2(3, 4);
c.mtc1(3, 1);
c.div_s(1, 1, 0);
c.mfc1(3, 1);
c.lqc2(4, (c.g[A0][0] as u32));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
c.mfc1(3, 2);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(0x110));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x120));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x128));
c.neg_s(0, 0);
c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x128));
c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(0x120));
c.lqc2(5, (c.g[S1][0] as u32).wrapping_add(0x120));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.qmfc2(2, 4);
c.mtc1(2, 1);
c.c_le_s(1, 3);
let t = !c.fcc();
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
if t {
break 'b_164bb4;
} else {
c.cycles += 1;
c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x120));
break 'b_164bb4;
}
}
'b_164d98: {
c.cycles += 80;
c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x6918));
c.mfc1(2, 0);
c.lqc2(4, (c.g[S1][0] as u32).wrapping_add(0x120));
c.qmtc2(2, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[S1][0] as u32).wrapping_add(0x120));
c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x6914));
c.mfc1(2, 1);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
c.qmtc2(2, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x6910));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(0x120));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[S1][0] as u32).wrapping_add(0x120));
c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(0x120));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.vu0().sqrt(4, 0);
c.vu0().waitq();
c.cfc2(2, 22);
c.mtc1(2, 0);
c.div_s(0, 21, 0);
c.mfc1(2, 0);
c.lqc2(4, (c.g[S1][0] as u32).wrapping_add(0x120));
c.qmtc2(2, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[S1][0] as u32).wrapping_add(0x120));
c.lqc2(3, (c.g[A1][0] as u32));
c.lqc2(5, (c.g[A1][0] as u32));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.qmfc2(2, 4);
c.mtc1(2, 0);
c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(0x120));
c.lqc2(5, (c.g[A1][0] as u32));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.qmfc2(3, 4);
c.mtc1(3, 1);
c.div_s(1, 1, 0);
c.mfc1(3, 1);
c.lqc2(4, (c.g[A1][0] as u32));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x150));
c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(0x120));
c.lqc2(4, (c.g[S1][0] as u32).wrapping_add(0x150));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.qmfc2(3, 4);
c.mtc1(3, 0);
c.c_lt_s(0, 2);
let t = !c.fcc();

if t {
break 'b_164d98;
} else {
c.cycles += 16;
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x38));
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 0);
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x30));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.vu0().sqrt(4, 0);
c.vu0().waitq();
c.cfc2(2, 22);
c.mtc1(2, 2);
c.c_lt_s(0, 2);
let t = !c.fcc();

if t {
break 'b_164d98;
} else {
c.cycles += 25;
c.g[AT][0] = 0x43480000;
c.mtc1(1, 0);
c.g[AT][0] = 0x42c80000;
c.mtc1(1, 1);
c.div_s(0, 0, 2);
c.mfc1(2, 0);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
c.qmtc2(2, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(0x120));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[S1][0] as u32).wrapping_add(0x120));
c.mfc1(3, 1);
c.lqc2(4, (c.g[A2][0] as u32).wrapping_sub(0xea0));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(0x120));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[S1][0] as u32).wrapping_add(0x120));
break 'b_164d98;
}
}
}
'b_164ec8: {
'b_164e6c: {
c.cycles += 50;
c.lqc2(4, (c.g[S1][0] as u32).wrapping_add(0x110));
c.lqc2(5, (c.g[S1][0] as u32).wrapping_add(0x120));
c.vu0().opmula(4, 5);
c.vu0().opmsub(6, 5, 4);
c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
c.sqc2(6, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x690c));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x130));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(0x130));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.vu0().sqrt(4, 0);
c.vu0().waitq();
c.cfc2(3, 22);
c.mtc1(3, 0);
c.mfc1(2, 0);
c.qmtc2(2, 3);
c.vu0().div(0, 3, 3, 0);
c.lqc2(4, (c.g[S1][0] as u32).wrapping_add(0x130));
c.vu0().waitq();
c.vu0().mul(0b1111, 5, 4, Src::Q);
c.sqc2(5, (c.g[S1][0] as u32).wrapping_add(0x130));
c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(0x110));
c.lqc2(5, (c.g[S1][0] as u32).wrapping_add(0x110));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.qmfc2(3, 4);
c.mtc1(3, 1);
c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(0x120));
c.lqc2(5, (c.g[S1][0] as u32).wrapping_add(0x120));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.qmfc2(2, 4);
c.mtc1(2, 0);
c.mul_s(1, 1, 0);
c.sqrt_s(1, 1);
c.c_lt_s(1, 2);
let t = !c.fcc();

if t {
break 'b_164e6c;
} else {
c.cycles += 3;
c.mtc1(0, 1);
let t = true;

if t {
break 'b_164ec8;
} else {
break 'b_164e6c;
}
}
}
'b_164eb8: {
c.cycles += 16;
c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(0x110));
c.lqc2(5, (c.g[S1][0] as u32).wrapping_add(0x120));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.qmfc2(2, 4);
c.mtc1(2, 0);
c.g[AT][0] = 0xffffffffbf800000;
c.mtc1(1, 2);
c.div_s(12, 0, 1);
c.c_le_s(2, 12);
if !c.fcc() {
c.mov_s(12, 2);
break 'b_164eb8;
} else {
c.cycles += 3;
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 0);
c.min_s(12, 12, 0);
break 'b_164eb8;
}
}
c.cycles += 4;
c.g[RA][0] = sx(0x00164ec0);
call_ptr(c, 0x0031c128, 0x00164ec0)?;
c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x6908));
c.sub_s(1, 1, 0);
break 'b_164ec8;
}
c.cycles += 55;
c.mtc1(0, 20);
c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x334));
c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x238));
c.g[A1][0] = 0x9;
c.g[A0][0] = c.g[S0][0];
c.mov_s(12, 20);
c.g[RA][0] = sx(0x00164ee8);
c.mov_s(13, 20);
call_ptr(c, 0x0031d738, 0x00164ee8)?;
c.mov_s(12, 20);
c.g[A0][0] = c.g[S0][0];
c.g[RA][0] = sx(0x00164ef8);
c.mov_s(13, 12);
call_ptr(c, 0x0031d7e0, 0x00164ef8)?;
c.g[AT][0] = 0x3e000000;
c.mtc1(1, 12);
c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x6904));
c.g[RA][0] = sx(0x00164f0c);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x0031d7e0, 0x00164f0c)?;
c.g[AT][0] = 0x3e800000;
c.mtc1(1, 12);
c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x6900));
c.g[RA][0] = sx(0x00164f20);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x0031d7e0, 0x00164f20)?;
c.g[AT][0] = 0x3ec00000;
c.mtc1(1, 12);
c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x68fc));
c.g[RA][0] = sx(0x00164f34);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x0031d7e0, 0x00164f34)?;
c.g[AT][0] = 0x3f000000;
c.mtc1(1, 12);
c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x68f8));
c.g[RA][0] = sx(0x00164f48);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x0031d7e0, 0x00164f48)?;
c.g[AT][0] = 0x3f200000;
c.mtc1(1, 12);
c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x68f4));
c.g[RA][0] = sx(0x00164f5c);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x0031d7e0, 0x00164f5c)?;
c.g[AT][0] = 0x3f400000;
c.mtc1(1, 12);
c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x68f0));
c.g[RA][0] = sx(0x00164f70);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x0031d7e0, 0x00164f70)?;
c.g[AT][0] = 0x3f600000;
c.mtc1(1, 12);
c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x68ec));
c.g[RA][0] = sx(0x00164f84);
c.g[A0][0] = c.g[S0][0];
call_ptr(c, 0x0031d7e0, 0x00164f84)?;
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 12);
c.g[A0][0] = c.g[S0][0];
c.g[RA][0] = sx(0x00164f98);
c.mov_s(13, 12);
call_ptr(c, 0x0031d7e0, 0x00164f98)?;
c.g[V0][0] = 0x1;
c.sw((c.g[S1][0] as u32).wrapping_add(0x2d8), c.g[V0][0]);
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2d8));
break 'b_164fa4;
}
}
c.cycles += 2;
if c.g[V0][0] == 0u64 {
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2d0));
break 'b_16531c;
} else {
c.cycles += 1;
c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x234));
break 'b_164fb0;
}
}
}
'b_1650a4: {
c.cycles += 7;
c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x68e8));
c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x230));
c.add_s(1, 1, 0);
c.c_lt_s(1, 2);
let t = !c.fcc();
c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x234));
if t {
break 'b_1650a4;
} else {
c.cycles += 54;
c.div_s(12, 1, 2);
c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x334));
c.g[V0][0] = 0x1;
c.g[A0][0] = c.g[V1][0];
c.sw((c.g[GP][0] as u32).wrapping_sub(0x1e64), c.g[V0][0]);
c.g[RA][0] = sx(0x00164fe8);
c.sw((c.g[GP][0] as u32).wrapping_add(0x1cf0), c.g[V1][0]);
call_ptr(c, 0x0031dee0, 0x00164fe8)?;
c.lwc1(12, (c.g[S1][0] as u32).wrapping_add(0x238));
c.mov_s(20, 0);
c.g[AT][0] = 0x3f000000;
c.mtc1(1, 0);
c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
c.mul_s(12, 20, 12);
c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x94));
c.g[RA][0] = sx(0x0016500c);
c.mul_s(12, 12, 0);
call_ptr(c, 0x0031be50, 0x0016500c)?;
c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x130));
c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x134));
c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x90));
c.lwc1(4, (c.g[S1][0] as u32).wrapping_add(0x138));
c.mul_s(1, 0, 1);
c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x94));
c.mul_s(2, 0, 2);
c.mul_s(0, 0, 4);
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x3c));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x30));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x34));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x38));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lqc2(5, (c.g[S1][0] as u32).wrapping_add(0x110));
c.vu0().sub(0b0001, 8, 8, Src::Vec(8));
c.vu0().opmula(4, 5);
c.vu0().opmsub(6, 5, 4);
c.vu0().opmula(4, 6);
c.vu0().opmsub(7, 6, 4);
c.vu0().mula(0b1110, 5, Src::Bc(0, W));
c.vu0().madda(0b1110, 6, Src::Bc(4, W));
c.vu0().madda(0b1110, 6, Src::Bc(4, W));
c.vu0().madda(0b1110, 7, Src::Bc(0, W));
c.vu0().madd(0b1110, 8, 7, Src::Bc(0, W));
c.sqc2(8, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x140));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.mfc1(3, 20);
c.lqc2(4, (c.g[S1][0] as u32).wrapping_add(0x150));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x160));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
break 'b_1650a4;
}
}
'b_1650f8: {
c.cycles += 21;
c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x68e4));
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x230));
c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x234));
c.mul_s(0, 0, 1);
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 4);
c.mtc1(0, 1);
c.g[AT][0] = 0x40400000;
c.mtc1(1, 3);
c.div_s(2, 2, 0);
c.min_s(2, 2, 4);
c.max_s(2, 2, 1);
c.mul_s(0, 2, 2);
c.add_s(1, 0, 0);
c.mul_s(0, 0, 3);
c.mul_s(1, 1, 2);
c.sub_s(0, 0, 1);
c.c_lt_s(4, 0);
if c.fcc() {
c.mov_s(0, 4);
break 'b_1650f8;
} else {
break 'b_1650f8;
}
}
c.cycles += 37;
c.mov_s(2, 0);
c.sub_s(0, 4, 2);
c.mfc1(3, 0);
c.lqc2(4, (c.g[S1][0] as u32).wrapping_add(0x20));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.mfc1(3, 2);
c.lqc2(4, (c.g[S2][0] as u32).wrapping_add(0x10));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x20));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[S1][0] as u32).wrapping_add(0x140));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lqc2(4, (c.g[S1][0] as u32).wrapping_add(0x160));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x40));
let t = true;
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
if t {
break 'b_165318;
} else {
break 'b_16518c;
}
}
}
c.cycles += 6;
c.lwc1(3, (c.g[S1][0] as u32).wrapping_add(0x23c));
c.mtc1(0, 1);
c.c_lt_s(1, 3);
if !c.fcc() {
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2d0));
break 'b_16531c;
} else {
'b_1651ec: {
c.cycles += 18;
c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x68e0));
c.g[AT][0] = 0x40400000;
c.mtc1(1, 2);
c.sub_s(0, 3, 0);
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 3);
c.max_s(0, 0, 1);
c.div_s(12, 0, 20);
c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x23c));
c.mul_s(1, 12, 12);
c.add_s(0, 1, 1);
c.mul_s(1, 1, 2);
c.mul_s(0, 0, 12);
c.sub_s(0, 1, 0);
c.c_lt_s(3, 0);
if c.fcc() {
c.mov_s(0, 3);
break 'b_1651ec;
} else {
break 'b_1651ec;
}
}
c.cycles += 75;
c.lq(2, (c.g[S1][0] as u32).wrapping_add(0x20));
c.mov_s(12, 0);
c.sub_s(0, 3, 12);
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.mfc1(3, 0);
c.lqc2(4, (c.g[S1][0] as u32).wrapping_add(0x20));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
c.mfc1(3, 12);
c.lqc2(4, (c.g[S2][0] as u32).wrapping_add(0x10));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x20));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.mfc1(3, 0);
c.lqc2(4, (c.g[S1][0] as u32).wrapping_add(0x40));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lqc2(3, (c.g[S2][0] as u32).wrapping_add(0x10));
c.lqc2(4, (c.g[S1][0] as u32).wrapping_add(0x140));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
c.lqc2(4, (c.g[S1][0] as u32).wrapping_add(0x160));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
c.mfc1(3, 12);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x60));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x80));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x40));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[S1][0] as u32).wrapping_add(0x40));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x170));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x23c));
c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x180));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x240));
c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x244));
break 'b_165318;
}
}
c.cycles += 1;
c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x2d0));
break 'b_16531c;
}
'b_165448: {
c.cycles += 2;
let t = c.g[V0][0] == 0u64;

if t {
break 'b_165448;
} else {
c.cycles += 5;
c.lwc1(4, (c.g[S1][0] as u32).wrapping_add(0x240));
c.mtc1(0, 3);
c.c_lt_s(3, 4);
let t = !c.fcc();

if t {
break 'b_165448;
} else {
c.cycles += 4;
c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x244));
c.c_lt_s(3, 1);
let t = !c.fcc();

if t {
break 'b_165448;
} else {
'b_165390: {
c.cycles += 18;
c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x68dc));
c.g[AT][0] = 0x40400000;
c.mtc1(1, 2);
c.sub_s(0, 4, 0);
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 4);
c.max_s(0, 0, 3);
c.div_s(3, 0, 1);
c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x240));
c.mul_s(1, 3, 3);
c.add_s(0, 1, 1);
c.mul_s(1, 1, 2);
c.mul_s(0, 0, 3);
c.sub_s(0, 1, 0);
c.c_lt_s(4, 0);
if c.fcc() {
c.mov_s(0, 4);
break 'b_165390;
} else {
break 'b_165390;
}
}
c.cycles += 46;
c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[S1][0] as u32).wrapping_add(0x40));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
c.mov_s(3, 0);
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sub_s(0, 4, 3);
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.mfc1(3, 3);
c.lqc2(4, (c.g[S1][0] as u32).wrapping_add(0x170));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
c.mfc1(3, 0);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x50));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.mfc1(3, 3);
c.lqc2(4, (c.g[S1][0] as u32).wrapping_add(0x180));
c.qmtc2(3, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x40));
c.vu0().add(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[S1][0] as u32).wrapping_add(0x20));
c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x30));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x50));
c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x40));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
break 'b_165448;
}
}
}
}
'b_165518: {
c.cycles += 31;
c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[S1][0] as u32).wrapping_add(0x40));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x30));
c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x68d8));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lqc2(3, (c.g[SP][0] as u32));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.vu0().sqrt(4, 0);
c.vu0().waitq();
c.cfc2(2, 22);
c.mtc1(2, 0);
c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x20));
c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
c.vu0().sqrt(4, 0);
c.vu0().waitq();
c.cfc2(3, 22);
c.mtc1(3, 2);
c.mul_s(1, 0, 1);
c.c_lt_s(2, 1);
let t = !c.fcc();

if t {
break 'b_165518;
} else {
'b_165514: {
c.cycles += 6;
c.g[AT][0] = 0x3f800000;
c.mtc1(1, 0);
c.c_lt_s(0, 2);
let t = !c.fcc();
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
if t {
break 'b_165514;
} else {
c.cycles += 14;
c.div_s(0, 1, 2);
c.mfc1(2, 0);
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
c.qmtc2(2, 3);
c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x20));
c.lqc2(3, (c.g[S1][0] as u32).wrapping_add(0x20));
c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0x20));
c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x40));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x40));
let t = true;
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
if t {
break 'b_165518;
} else {
break 'b_165514;
}
}
}
c.cycles += 1;
c.sq(2, (c.g[S1][0] as u32).wrapping_add(0x40));
break 'b_165518;
}
}
c.cycles += 10;
c.g[RA][0] = sx(0x00165520);
c.g[A0][0] = c.g[S1][0];
call_ptr(c, 0x00168150, 0x00165520)?;
c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xc0));
c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0xe8));
c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
let t = c.g[RA][0] as u32;
c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
c.pc = t; return Ok(());
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine_rt;
    use crate::difftest::{check, elf_image, fresh, run_generated, DATA};

    /// Seeds for the air-swing test (real callees: spline, asin, sincos,
    /// distance check, and real leaf getters behind the target's vtable).
    const OC1471_SEEDS: u32 = 300;
    const OC1471_SELF: u32 = DATA + 0x1000;
    const OC1471_TARGET: u32 = DATA + 0x8000;
    const OC1471_VTABLE: u32 = DATA + 0x9000;
    const OC1471_INFO: u32 = DATA + 0xa000;

    /// Spline knots the function itself adds (time, value).
    const OC1471_KNOTS: [(f32, f32); 9] = [
        (0.0, 0.0),
        (0.125, 0.0639725923538208),
        (0.25, 0.30785250663757324),
        (0.375, 0.5528385043144226),
        (0.5, 0.6577425003051758),
        (0.625, 0.728873610496521),
        (0.75, 0.827086865901947),
        (0.875, 0.9414384365081787),
        (1.0, 1.0),
    ];

    fn oc1471_put_vec(c: &mut Cpu, at: u32, r: &mut crate::difftest::Rng, range: f32) {
        for i in 0..4 {
            let bits = r.f32(range);
            c.bus.write32(at + 4 * i, bits);
        }
    }

    /// Builds the half-pipe spline exactly as the function does (real callees).
    fn oc1471_prime_spline(c: &mut Cpu, spl: u32) {
        for i in 0..4 {
            c.bus.write32(spl + 4 * i, 0);
        }
        c.g[A0][0] = spl as u64;
        c.g[A1][0] = 9;
        c.f[12] = 0;
        c.f[13] = 0;
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = 0x0031_d738;
        ee::s10::f_0031d738(c);
        for (t, v) in OC1471_KNOTS {
            c.g[A0][0] = spl as u64;
            c.f[12] = t.to_bits();
            c.f[13] = v.to_bits();
            c.g[RA][0] = crate::difftest::RA as u64;
            c.pc = 0x0031_d7e0;
            ee::s10::f_0031d7e0(c);
        }
    }

    fn oc1471_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine_rt(c, seed, 0x16_4878, 0);
        elf_image(c);
        let scale = [1.0f32, 20.0, 300.0, 3000.0][(seed % 4) as usize];
        // The camera algorithm: random floats over its fields.
        for off in (0..0x334).step_by(4) {
            let bits = r.f32(scale);
            c.bus.write32(OC1471_SELF + off, bits);
        }
        let v = r.next_u32();
        let doing = v & 1 != 0;
        let init = v & 2 != 0;
        let done = v & 4 != 0;
        let blend = v & 8 != 0;
        let third = v & 16 != 0;
        let long_air = v & 32 != 0;
        let zero_up = (v >> 8) & 7 == 0;
        let s = OC1471_SELF;
        c.bus.write32(s + 0x2d0, doing as u32);
        c.bus.write32(s + 0x2d8, init as u32);
        c.bus.write32(s + 0x30, OC1471_TARGET);
        let f = |x: f32| x.to_bits();
        let t0 = (r.next_u32() % 1000) as f32 / 1000.0;
        c.bus.write32(s + 0x22c, f(t0 * 2.0));
        c.bus
            .write32(s + 0x23c, if blend { f(0.05 + t0) } else { f(-t0) });
        c.bus
            .write32(s + 0x240, if third { f(0.02 + t0 / 2.0) } else { f(-t0) });
        c.bus
            .write32(s + 0x244, if third { f(0.05 + t0) } else { f(t0) });
        c.bus.write32(s + 0x238, f(t0 * 6.0 - 3.0));
        if doing && init {
            let dur = 2.3 + t0 * 6.0;
            let u = (r.next_u32() % 1200) as f32 / 1000.0;
            c.bus.write32(s + 0x230, f(dur));
            c.bus.write32(s + 0x234, f(dur * u));
            oc1471_prime_spline(c, s + 0x334);
        } else {
            for i in 0..4 {
                c.bus.write32(s + 0x334 + 4 * i, 0);
            }
        }
        // The target and its vtable: done-flag, air time and up-vector
        // getters are real leaf functions.
        c.bus.write32(OC1471_TARGET, OC1471_VTABLE);
        c.bus.write32(OC1471_TARGET + 0x2f0, done as u32);
        c.bus
            .write32(OC1471_TARGET + 0x14, f(if long_air { 5.0 * t0 } else { 2.0 * t0 }));
        oc1471_put_vec(c, OC1471_TARGET + 0x110, &mut r, 1.0);
        if zero_up {
            for i in 0..4 {
                c.bus.write32(OC1471_TARGET + 0x110 + 4 * i, 0);
            }
        }
        for (off, func) in [(0x98u32, 0x0016_7fb0u32), (0x60, 0x0011_7900), (0x80, 0x0014_08f0)] {
            c.bus.write16(OC1471_VTABLE + off, 0);
            c.bus.write32(OC1471_VTABLE + off + 4, func);
        }
        // The chase setup: velocity at +0x10, position at +0x20.
        oc1471_put_vec(c, OC1471_INFO + 0x10, &mut r, scale);
        oc1471_put_vec(c, OC1471_INFO + 0x20, &mut r, scale);
        c.g[A0][0] = s as u64;
        c.g[A1][0] = OC1471_INFO as u64;
        c.g[RA][0] = crate::difftest::RA as u64;
        c.g[SP][0] = crate::difftest::SP as u64;
        c.pc = 0x0016_4878;
    }

    /// `ChaseCameraAlgorithm_ApplyAirSwing`: the half-pipe camera swing during
    /// an air trick (spline-driven blend of eye and look-at), against the
    /// generated body, with every callee real. The seeds vary the half-pipe
    /// and init flags, the blend timers, air time and vector magnitudes.
    #[test]
    fn chase_camera_algorithm_apply_air_swing_matches() {
        check(
            ee::s01::f_00164878__generated,
            chase_camera_algorithm_apply_air_swing,
            OC1471_SEEDS,
            &oc1471_setup,
        );
        // The seeds reach many different paths (cycle totals).
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..OC1471_SEEDS {
            let mut c = fresh();
            oc1471_setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, ee::s01::f_00164878__generated);
            seen.insert(c.cycles - before);
        }
        assert!(seen.len() >= 16, "only {} paths: {seen:?}", seen.len());
    }
}
