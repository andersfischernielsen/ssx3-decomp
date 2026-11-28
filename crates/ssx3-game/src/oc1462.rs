//! Staging module oc1462 (wave wave274): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1462`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0032_e9a0, coll_segment_refine_triangle_hit, "CollSegment_RefineTriangleHit"),
];

/// `CollSegment_RefineTriangleHit(a0 = segment, a1 = bicubic patch, a2 = &u,
/// a3 = &v, t0 = outPos, t1 = outNrm)` (0x32e9a0, 3244 bytes, no calls).
///
/// Newton refinement of a line segment's hit on a bicubic surface patch: four
/// fixed iterations solving `[dP/du dP/dv -dir] * (du, dv, t) = p0 - P(u, v)`
/// (3x3 adjugate/determinant inverse), updating `*u`, `*v`; the function
/// returns early (nothing written to the outputs) once `u` or `v` leaves
/// [0, 1]. Otherwise it writes the surface position to `*outPos` and the
/// normalised `dv x du` to `*outNrm`. The body keeps the generated VU0/FPU
/// op sequence exactly (it is a stack-heavy hand-scheduled sequence whose
/// register and scratch results are part of the contract); the only
/// restructuring is the back-edge event check.
pub fn coll_segment_refine_triangle_hit(c: &mut Cpu) {
    c.cycles += 298;
    c.lwc1(1, (c.g[A3][0] as u32));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x1a0));
    c.lwc1(0, (c.g[A2][0] as u32));
    c.g[T4][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40));
    c.mul_s(16, 1, 1);
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 5);
    c.mul_s(4, 0, 0);
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x58));
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x68));
    c.g[T3][0] = sx((c.g[A1][0] as u32).wrapping_add(0x80));
    c.mul_s(2, 1, 16);
    c.swc1(16, (c.g[SP][0] as u32).wrapping_add(0x54));
    c.mul_s(3, 0, 4);
    c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x64));
    c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x198));
    c.g[T5][0] = sx((c.g[A1][0] as u32).wrapping_add(0x100));
    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xc0));
    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.g[T2][0] = c.g[A0][0];
    c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x190));
    c.g[T6][0] = 0u64;
    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x188));
    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x180));
    c.mov_s(21, 5);
    c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x5c));
    c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x6c));
    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lqc2(4, (c.g[T4][0] as u32));
    c.lqc2(5, (c.g[T4][0] as u32).wrapping_add(0x10));
    c.lqc2(6, (c.g[T4][0] as u32).wrapping_add(0x20));
    c.lqc2(7, (c.g[T4][0] as u32).wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.g[AT][0] = 0x40400000;
    c.mtc1(1, 3);
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.mul_s(18, 16, 3);
    c.mov_s(23, 3);
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.mul_s(3, 4, 3);
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lqc2(4, (c.g[T3][0] as u32));
    c.lqc2(5, (c.g[T3][0] as u32).wrapping_add(0x10));
    c.lqc2(6, (c.g[T3][0] as u32).wrapping_add(0x20));
    c.lqc2(7, (c.g[T3][0] as u32).wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.mtc1(0, 22);
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lqc2(4, (c.g[A1][0] as u32));
    c.lqc2(5, (c.g[A1][0] as u32).wrapping_add(0x10));
    c.lqc2(6, (c.g[A1][0] as u32).wrapping_add(0x20));
    c.lqc2(7, (c.g[A1][0] as u32).wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lqc2(4, (c.g[T5][0] as u32));
    c.lqc2(5, (c.g[T5][0] as u32).wrapping_add(0x10));
    c.lqc2(6, (c.g[T5][0] as u32).wrapping_add(0x20));
    c.lqc2(7, (c.g[T5][0] as u32).wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lwc1(6, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lwc1(7, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lwc1(8, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lwc1(9, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x84));
    c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0x88));
    c.swc1(9, (c.g[SP][0] as u32).wrapping_add(0x8c));
    c.lwc1(17, (c.g[SP][0] as u32).wrapping_add(0x14));
    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(2, 4);
    c.mtc1(2, 1);
    c.lwc1(16, (c.g[SP][0] as u32).wrapping_add(0x44));
    c.lwc1(14, (c.g[SP][0] as u32).wrapping_add(0x24));
    c.lwc1(15, (c.g[SP][0] as u32).wrapping_add(0x34));
    c.swc1(17, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.swc1(16, (c.g[SP][0] as u32).wrapping_add(0x9c));
    c.lwc1(13, (c.g[SP][0] as u32).wrapping_add(0x18));
    c.swc1(14, (c.g[SP][0] as u32).wrapping_add(0x94));
    c.swc1(15, (c.g[SP][0] as u32).wrapping_add(0x98));
    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(2, 4);
    c.mtc1(2, 4);
    c.lwc1(10, (c.g[SP][0] as u32).wrapping_add(0x28));
    c.lwc1(11, (c.g[SP][0] as u32).wrapping_add(0x38));
    c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0x48));
    c.swc1(13, (c.g[SP][0] as u32).wrapping_add(0xa0));
    c.swc1(10, (c.g[SP][0] as u32).wrapping_add(0xa4));
    c.swc1(11, (c.g[SP][0] as u32).wrapping_add(0xa8));
    c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0xac));
    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0xa0));
    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(2, 4);
    c.mtc1(2, 2);
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.lwc1(0, (c.g[A3][0] as u32));
    c.lwc1(1, (c.g[A2][0] as u32));
    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x78));
    c.add_s(0, 0, 0);
    c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x74));
    c.add_s(1, 1, 1);
    c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x7c));
    c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x94));
    c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0x98));
    c.swc1(18, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x74));
    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x84));
    c.swc1(9, (c.g[SP][0] as u32).wrapping_add(0x9c));
    c.sq(2, (c.g[SP][0] as u32));
    c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x78));
    c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x7c));
    c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x88));
    c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x8c));
    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(2, 4);
    c.mtc1(2, 19);
    c.swc1(17, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.swc1(14, (c.g[SP][0] as u32).wrapping_add(0x94));
    c.swc1(15, (c.g[SP][0] as u32).wrapping_add(0x98));
    c.swc1(16, (c.g[SP][0] as u32).wrapping_add(0x9c));
    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(2, 4);
    c.mtc1(2, 20);
    c.swc1(13, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.swc1(10, (c.g[SP][0] as u32).wrapping_add(0x94));
    c.swc1(11, (c.g[SP][0] as u32).wrapping_add(0x98));
    c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0x9c));
    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(2, 4);
    c.mtc1(2, 18);
    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.lqc2(4, (c.g[T4][0] as u32));
    c.lqc2(5, (c.g[T4][0] as u32).wrapping_add(0x10));
    c.lqc2(6, (c.g[T4][0] as u32).wrapping_add(0x20));
    c.lqc2(7, (c.g[T4][0] as u32).wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0xa0));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.lqc2(4, (c.g[T3][0] as u32));
    c.lqc2(5, (c.g[T3][0] as u32).wrapping_add(0x10));
    c.lqc2(6, (c.g[T3][0] as u32).wrapping_add(0x20));
    c.lqc2(7, (c.g[T3][0] as u32).wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0xa0));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.lqc2(4, (c.g[A1][0] as u32));
    c.lqc2(5, (c.g[A1][0] as u32).wrapping_add(0x10));
    c.lqc2(6, (c.g[A1][0] as u32).wrapping_add(0x20));
    c.lqc2(7, (c.g[A1][0] as u32).wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0xa0));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.lqc2(4, (c.g[T5][0] as u32));
    c.lqc2(5, (c.g[T5][0] as u32).wrapping_add(0x10));
    c.lqc2(6, (c.g[T5][0] as u32).wrapping_add(0x20));
    c.lqc2(7, (c.g[T5][0] as u32).wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0xa0));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
    c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x94));
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x98));
    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x9c));
    c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x14));
    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(2, 4);
    c.mtc1(2, 13);
    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x24));
    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x34));
    c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x44));
    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x94));
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x98));
    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x9c));
    c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x18));
    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(2, 4);
    c.mtc1(2, 17);
    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x28));
    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x38));
    c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x48));
    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x94));
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x98));
    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x9c));
    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(2, 4);
    c.mtc1(2, 16);
    'l_32ee48: loop {
    'b_32f638: {
    c.cycles += 172;
    c.lqc2(3, (c.g[T2][0] as u32).wrapping_add(0x60));
    c.lqc2(4, (c.g[SP][0] as u32));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.mul_s(0, 16, 20);
    c.lwc1(5, (c.g[T2][0] as u32).wrapping_add(0x74));
    c.mul_s(11, 17, 18);
    c.lwc1(2, (c.g[T2][0] as u32).wrapping_add(0x78));
    c.mul_s(15, 18, 13);
    c.neg_s(5, 5);
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.neg_s(2, 2);
    c.lwc1(1, (c.g[T2][0] as u32).wrapping_add(0x70));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
    c.sub_s(11, 11, 0);
    c.mul_s(9, 18, 5);
    c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xdc));
    c.mul_s(3, 2, 17);
    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xa0));
    c.mul_s(7, 20, 2);
    c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0xa4));
    c.mul_s(8, 5, 16);
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.neg_s(1, 1);
    c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x94));
    c.sub_s(7, 7, 9);
    c.lwc1(6, (c.g[SP][0] as u32).wrapping_add(0xa8));
    c.sub_s(8, 8, 3);
    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xe0));
    c.mul_s(4, 1, 11);
    c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x98));
    c.mul_s(0, 13, 7);
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xd8));
    c.mul_s(3, 19, 8);
    c.swc1(13, (c.g[SP][0] as u32).wrapping_add(0xc0));
    c.mul_s(10, 13, 2);
    c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0xf0));
    c.mul_s(14, 5, 13);
    c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0xf4));
    c.add_s(0, 0, 3);
    c.swc1(11, (c.g[SP][0] as u32).wrapping_add(0xf8));
    c.mul_s(3, 16, 1);
    c.swc1(19, (c.g[SP][0] as u32).wrapping_add(0xcc));
    c.mul_s(6, 1, 18);
    c.swc1(17, (c.g[SP][0] as u32).wrapping_add(0xc4));
    c.add_s(0, 0, 4);
    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xd0));
    c.mul_s(4, 20, 1);
    c.swc1(16, (c.g[SP][0] as u32).wrapping_add(0xc8));
    c.mul_s(9, 13, 20);
    c.swc1(18, (c.g[SP][0] as u32).wrapping_add(0xd4));
    c.div_s(0, 21, 0);
    c.mul_s(2, 2, 19);
    c.mul_s(12, 19, 16);
    c.mul_s(5, 19, 5);
    c.mul_s(1, 1, 17);
    c.mul_s(13, 17, 19);
    c.sub_s(6, 6, 2);
    c.sub_s(10, 10, 3);
    c.sub_s(5, 5, 4);
    c.sub_s(12, 12, 15);
    c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0xfc));
    c.sub_s(1, 1, 14);
    c.swc1(10, (c.g[SP][0] as u32).wrapping_add(0x100));
    c.sub_s(9, 9, 13);
    c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x108));
    c.mul_s(7, 7, 0);
    c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0x104));
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x10c));
    c.mul_s(8, 8, 0);
    c.mul_s(2, 9, 0);
    c.swc1(9, (c.g[SP][0] as u32).wrapping_add(0x110));
    c.mul_s(6, 6, 0);
    c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x150));
    c.mul_s(10, 10, 0);
    c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0x154));
    c.mul_s(5, 5, 0);
    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x170));
    c.mul_s(1, 1, 0);
    c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x15c));
    c.mul_s(11, 11, 0);
    c.swc1(10, (c.g[SP][0] as u32).wrapping_add(0x160));
    c.mul_s(12, 12, 0);
    c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x168));
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x16c));
    c.swc1(11, (c.g[SP][0] as u32).wrapping_add(0x158));
    c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0x164));
    c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x157));
    c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x150));
    c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x15f));
    c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x158));
    c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x167));
    c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x160));
    c.ldl(15, (c.g[SP][0] as u32).wrapping_add(0x16f));
    c.ldr(15, (c.g[SP][0] as u32).wrapping_add(0x168));
    c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x127));
    c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x120));
    c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x12f));
    c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x128));
    c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x137));
    c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0x130));
    c.sdl(15, (c.g[SP][0] as u32).wrapping_add(0x13f));
    c.sdr(15, (c.g[SP][0] as u32).wrapping_add(0x138));
    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x170));
    c.sw((c.g[SP][0] as u32).wrapping_add(0x140), c.g[V0][0]);
    c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x127));
    c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x120));
    c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x12f));
    c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x128));
    c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x137));
    c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x130));
    c.ldl(15, (c.g[SP][0] as u32).wrapping_add(0x13f));
    c.ldr(15, (c.g[SP][0] as u32).wrapping_add(0x138));
    c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0xc7));
    c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
    c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0xcf));
    c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0xc8));
    c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0xd7));
    c.sdr(4, (c.g[SP][0] as u32).wrapping_add(0xd0));
    c.sdl(15, (c.g[SP][0] as u32).wrapping_add(0xdf));
    c.sdr(15, (c.g[SP][0] as u32).wrapping_add(0xd8));
    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x140));
    c.sw((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[V0][0]);
    c.lwc1(10, (c.g[A2][0] as u32));
    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.lwc1(7, (c.g[SP][0] as u32).wrapping_add(0x94));
    c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0xc0));
    c.lwc1(9, (c.g[SP][0] as u32).wrapping_add(0xcc));
    c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0xc4));
    c.mul_s(3, 3, 0);
    c.lwc1(8, (c.g[SP][0] as u32).wrapping_add(0xd0));
    c.mul_s(9, 9, 7);
    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xc8));
    c.mul_s(2, 2, 0);
    c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0xd4));
    c.mul_s(8, 8, 7);
    c.mul_s(1, 1, 0);
    c.lwc1(5, (c.g[SP][0] as u32).wrapping_add(0x98));
    c.mul_s(4, 4, 7);
    c.lwc1(6, (c.g[SP][0] as u32).wrapping_add(0xd8));
    c.lwc1(7, (c.g[SP][0] as u32).wrapping_add(0xdc));
    c.add_s(3, 3, 9);
    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xe0));
    c.add_s(2, 2, 8);
    c.add_s(1, 1, 4);
    c.mul_s(0, 0, 5);
    c.mul_s(6, 6, 5);
    c.mul_s(7, 7, 5);
    c.add_s(1, 1, 0);
    c.add_s(3, 3, 6);
    c.add_s(2, 2, 7);
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xb8));
    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xb4));
    c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0xb7));
    c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xb8));
    c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0xa7));
    c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0xa0));
    c.sw((c.g[SP][0] as u32).wrapping_add(0xa8), c.g[V1][0]);
    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xa0));
    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0xa4));
    c.add_s(10, 10, 0);
    c.swc1(10, (c.g[A2][0] as u32));
    c.lwc1(0, (c.g[A3][0] as u32));
    c.add_s(0, 0, 1);
    c.swc1(0, (c.g[A3][0] as u32));
    c.lwc1(1, (c.g[A2][0] as u32));
    c.c_lt_s(1, 22);
    if c.fcc() {
    c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x198));
    break 'b_32f638;
    } else {
    c.cycles += 4;
    c.c_lt_s(21, 1);
    if c.fcc() {
    c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x198));
    break 'b_32f638;
    } else {
    c.cycles += 4;
    c.c_lt_s(0, 22);
    if c.fcc() {
    c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x198));
    break 'b_32f638;
    } else {
    c.cycles += 4;
    c.c_lt_s(21, 0);
    if c.fcc() {
    c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x198));
    break 'b_32f638;
    } else {
    c.cycles += 292;
    c.mul_s(16, 0, 0);
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xb8));
    c.mul_s(4, 1, 1);
    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xbc));
    c.g[T6][0] = sx((c.g[T6][0] as u32).wrapping_add(0x1));
    c.mul_s(2, 0, 16);
    c.swc1(16, (c.g[SP][0] as u32).wrapping_add(0xb4));
    c.mul_s(3, 1, 4);
    c.g[A0][0] = ((c.g[T6][0] as i64) < 4i64) as u64;
    c.mul_s(17, 4, 23);
    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.mul_s(16, 16, 23);
    c.lq(3, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xb8));
    c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0xb4));
    c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lqc2(4, (c.g[T4][0] as u32));
    c.lqc2(5, (c.g[T4][0] as u32).wrapping_add(0x10));
    c.lqc2(6, (c.g[T4][0] as u32).wrapping_add(0x20));
    c.lqc2(7, (c.g[T4][0] as u32).wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0xf0));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lqc2(4, (c.g[T3][0] as u32));
    c.lqc2(5, (c.g[T3][0] as u32).wrapping_add(0x10));
    c.lqc2(6, (c.g[T3][0] as u32).wrapping_add(0x20));
    c.lqc2(7, (c.g[T3][0] as u32).wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0xf0));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lqc2(4, (c.g[A1][0] as u32));
    c.lqc2(5, (c.g[A1][0] as u32).wrapping_add(0x10));
    c.lqc2(6, (c.g[A1][0] as u32).wrapping_add(0x20));
    c.lqc2(7, (c.g[A1][0] as u32).wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0xf0));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lqc2(4, (c.g[T5][0] as u32));
    c.lqc2(5, (c.g[T5][0] as u32).wrapping_add(0x10));
    c.lqc2(6, (c.g[T5][0] as u32).wrapping_add(0x20));
    c.lqc2(7, (c.g[T5][0] as u32).wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0xf0));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lwc1(12, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lwc1(13, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lwc1(14, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lwc1(15, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0xf0));
    c.swc1(13, (c.g[SP][0] as u32).wrapping_add(0xf4));
    c.swc1(14, (c.g[SP][0] as u32).wrapping_add(0xf8));
    c.swc1(15, (c.g[SP][0] as u32).wrapping_add(0xfc));
    c.lwc1(8, (c.g[SP][0] as u32).wrapping_add(0x14));
    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0xf0));
    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(2, 4);
    c.mtc1(2, 3);
    c.lwc1(9, (c.g[SP][0] as u32).wrapping_add(0x24));
    c.lwc1(10, (c.g[SP][0] as u32).wrapping_add(0x34));
    c.lwc1(11, (c.g[SP][0] as u32).wrapping_add(0x44));
    c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0x100));
    c.lwc1(4, (c.g[SP][0] as u32).wrapping_add(0x18));
    c.lwc1(5, (c.g[SP][0] as u32).wrapping_add(0x28));
    c.lwc1(6, (c.g[SP][0] as u32).wrapping_add(0x38));
    c.swc1(9, (c.g[SP][0] as u32).wrapping_add(0x104));
    c.swc1(10, (c.g[SP][0] as u32).wrapping_add(0x108));
    c.swc1(11, (c.g[SP][0] as u32).wrapping_add(0x10c));
    c.lwc1(7, (c.g[SP][0] as u32).wrapping_add(0x48));
    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x100));
    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(2, 4);
    c.mtc1(2, 2);
    c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x110));
    c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x114));
    c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0x118));
    c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0x11c));
    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x110));
    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(2, 4);
    c.mtc1(2, 1);
    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.lwc1(0, (c.g[A3][0] as u32));
    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xb4));
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xb8));
    c.add_s(0, 0, 0);
    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xbc));
    c.lwc1(1, (c.g[A2][0] as u32));
    c.lq(3, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.swc1(16, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.add_s(1, 1, 1);
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xb4));
    c.sw((c.g[SP][0] as u32).wrapping_add(0xbc), 0u64);
    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0xb8));
    c.sq(3, (c.g[SP][0] as u32));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.swc1(17, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xb4));
    c.sw((c.g[SP][0] as u32).wrapping_add(0xbc), 0u64);
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.lq(3, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.swc1(13, (c.g[SP][0] as u32).wrapping_add(0xb4));
    c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.swc1(14, (c.g[SP][0] as u32).wrapping_add(0xb8));
    c.swc1(15, (c.g[SP][0] as u32).wrapping_add(0xbc));
    c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(2, 4);
    c.mtc1(2, 12);
    c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.mov_s(19, 12);
    c.swc1(9, (c.g[SP][0] as u32).wrapping_add(0xb4));
    c.swc1(10, (c.g[SP][0] as u32).wrapping_add(0xb8));
    c.swc1(11, (c.g[SP][0] as u32).wrapping_add(0xbc));
    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(2, 4);
    c.mtc1(2, 8);
    c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.mov_s(20, 8);
    c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0xb4));
    c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0xb8));
    c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0xbc));
    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(2, 4);
    c.mtc1(2, 6);
    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.lqc2(4, (c.g[T4][0] as u32));
    c.lqc2(5, (c.g[T4][0] as u32).wrapping_add(0x10));
    c.lqc2(6, (c.g[T4][0] as u32).wrapping_add(0x20));
    c.lqc2(7, (c.g[T4][0] as u32).wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0xf0));
    c.mov_s(18, 6);
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.lqc2(4, (c.g[T3][0] as u32));
    c.lqc2(5, (c.g[T3][0] as u32).wrapping_add(0x10));
    c.lqc2(6, (c.g[T3][0] as u32).wrapping_add(0x20));
    c.lqc2(7, (c.g[T3][0] as u32).wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0xf0));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.lqc2(4, (c.g[A1][0] as u32));
    c.lqc2(5, (c.g[A1][0] as u32).wrapping_add(0x10));
    c.lqc2(6, (c.g[A1][0] as u32).wrapping_add(0x20));
    c.lqc2(7, (c.g[A1][0] as u32).wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0xf0));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.lqc2(8, (c.g[SP][0] as u32).wrapping_add(0x80));
    c.lqc2(4, (c.g[T5][0] as u32));
    c.lqc2(5, (c.g[T5][0] as u32).wrapping_add(0x10));
    c.lqc2(6, (c.g[T5][0] as u32).wrapping_add(0x20));
    c.lqc2(7, (c.g[T5][0] as u32).wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, (c.g[SP][0] as u32).wrapping_add(0xf0));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xf0));
    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xb4));
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xb8));
    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xbc));
    c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x14));
    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(2, 4);
    c.mtc1(2, 5);
    c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x24));
    c.mov_s(13, 5);
    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x34));
    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x44));
    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xb4));
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xb8));
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xbc));
    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x38));
    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(2, 4);
    c.mtc1(2, 4);
    c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x18));
    c.mov_s(17, 4);
    c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x28));
    c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x48));
    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xb4));
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0xb8));
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xbc));
    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(2, 4);
    c.mtc1(2, 0);
    let t = c.g[A0][0] != 0u64;
    c.mov_s(16, 0);
    if t {
    if event_due(c) { deliver_event_here(c, 0x0032_ee48); }
    continue 'l_32ee48;
    } else {
    c.cycles += 32;
    c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.swc1(4, (c.g[SP][0] as u32).wrapping_add(0x94));
    c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x98));
    c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0xa0));
    c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0xa4));
    c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0xa8));
    c.sw((c.g[SP][0] as u32).wrapping_add(0x9c), 0u64);
    c.sw((c.g[SP][0] as u32).wrapping_add(0xac), 0u64);
    c.lqc2(4, (c.g[SP][0] as u32).wrapping_add(0xa0));
    c.lqc2(5, (c.g[SP][0] as u32).wrapping_add(0x90));
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
    c.sqc2(6, (c.g[SP][0] as u32).wrapping_add(0xd0));
    c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
    c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xc0));
    c.lqc2(3, (c.g[SP][0] as u32).wrapping_add(0xc0));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().rsqrt(0, 3, 4, 0);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 5, 3, Src::Q);
    c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0xd0));
    c.lq(2, (c.g[SP][0] as u32));
    c.lq(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
    c.sq(2, (c.g[T0][0] as u32));
    c.sq(3, (c.g[T1][0] as u32));
    c.sq(3, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x198));
    break 'b_32f638;
    }
    }
    }
    }
    }
    }
    c.cycles += 5;
    c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x190));
    c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x188));
    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x180));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1a0));
    c.pc = t; return;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, scramble, Rng, DATA};

    const SEG: u32 = DATA;
    const PATCH: u32 = DATA + 0x400;
    const UV_U: u32 = DATA + 0x200;
    const UV_V: u32 = DATA + 0x204;
    const OUT_POS: u32 = DATA + 0x300;
    const OUT_NRM: u32 = DATA + 0x310;
    const STACK: u32 = DATA + 0x10000;

    fn put(c: &mut Cpu, addr: u32, v: f32) {
        c.bus.write32(addr, v.to_bits());
    }

    /// Segment p0 at +0x60, direction at +0x70; patch matrices at +0x40 + 0x40*k
    /// (four columns of four floats each).
    #[test]
    fn coll_segment_refine_triangle_hit_matches() {
        check(
            ee::s11::f_0032e9a0__generated,
            coll_segment_refine_triangle_hit,
            250,
            &|c, seed| {
                let mut r = Rng(0x32e9_a000 ^ (seed as u64 * 0x9e37_79b9 + 1));
                scramble(c, &mut r);
                c.g[SP][0] = sx(STACK);
                c.g[A0][0] = sx(SEG);
                c.g[A1][0] = sx(PATCH);
                c.g[A2][0] = sx(UV_U);
                c.g[A3][0] = sx(UV_V);
                c.g[T0][0] = sx(OUT_POS);
                c.g[T1][0] = sx(OUT_NRM);
                let unit = |r: &mut Rng, lo: f32, hi: f32| {
                    lo + (r.next_u32() as f32 / u32::MAX as f32) * (hi - lo)
                };
                match seed % 5 {
                    0 => {
                        // Random patch and segment: mostly leaves [0, 1] at once.
                        for k in 0..0x80u32 {
                            let w = r.f32(2.0);
                            c.bus.write32(PATCH + 0x40 + k * 4, w);
                        }
                        for o in [0x60u32, 0x64, 0x68, 0x6c, 0x70, 0x74, 0x78] {
                            let w = r.f32(2.0);
                            c.bus.write32(SEG + o, w);
                        }
                        let (u, v) = (unit(&mut r, 0.0, 1.0), unit(&mut r, 0.0, 1.0));
                        put(c, UV_U, u);
                        put(c, UV_V, v);
                    }
                    _ => {
                        // Affine patch P = o + b*u + a*v (matrix 2 column 3 = a,
                        // matrix 3 column 2 = b, column 3 = o), segment aimed at
                        // (u*, v*): Newton converges at once, so all four
                        // iterations run (seeds % 5 == 3: target outside [0, 1]).
                        for k in 0..0x80u32 {
                            c.bus.write32(PATCH + 0x40 + k * 4, 0);
                        }
                        let a = [1.0 + unit(&mut r, -0.2, 0.2), unit(&mut r, 0.0, 0.3), unit(&mut r, -0.2, 0.2)];
                        let b = [unit(&mut r, 0.0, 0.3), 1.0 + unit(&mut r, -0.2, 0.2), unit(&mut r, 0.0, 0.4)];
                        let o = [unit(&mut r, -1.0, 1.0), unit(&mut r, -1.0, 1.0), unit(&mut r, -1.0, 1.0)];
                        let dp = [unit(&mut r, 0.0, 0.4), unit(&mut r, -0.3, 0.3), 1.0 + unit(&mut r, -0.2, 0.2)];
                        let (us, vs) = if seed % 5 == 3 {
                            (unit(&mut r, 1.4, 2.0), unit(&mut r, 0.2, 0.8))
                        } else if seed % 5 == 4 {
                            (unit(&mut r, 0.2, 0.8), unit(&mut r, -1.0, -0.3))
                        } else {
                            (unit(&mut r, 0.2, 0.8), unit(&mut r, 0.2, 0.8))
                        };
                        let t = unit(&mut r, 0.0, 2.0);
                        for i in 0..3u32 {
                            let at = PATCH + 0x40 + 0x40 * 2 + 0x30 + i * 4; // matrix 2, col 3
                            put(c, at, a[i as usize]);
                            let bt = PATCH + 0x40 + 0x40 * 3 + 0x20 + i * 4; // matrix 3, col 2
                            put(c, bt, b[i as usize]);
                            let ot = PATCH + 0x40 + 0x40 * 3 + 0x30 + i * 4; // matrix 3, col 3
                            put(c, ot, o[i as usize]);
                            let p0 = o[i as usize] + b[i as usize] * us + a[i as usize] * vs + dp[i as usize] * t;
                            put(c, SEG + 0x60 + i * 4, p0);
                            put(c, SEG + 0x70 + i * 4, dp[i as usize]);
                        }
                        put(c, SEG + 0x6c, 1.0);
                        put(c, PATCH + 0x40 + 0x40 * 3 + 0x3c, 1.0);
                        // Start near the target (or off it, to vary the first step).
                        let (u0, v0) = if seed % 5 == 3 {
                            (unit(&mut r, 0.3, 0.7), unit(&mut r, 0.3, 0.7))
                        } else {
                            (unit(&mut r, 0.1, 0.9), unit(&mut r, 0.1, 0.9))
                        };
                        put(c, UV_U, u0);
                        put(c, UV_V, v0);
                    }
                }
                for k in 0..4u32 {
                    c.bus.write32(OUT_POS + k * 4, 0xdead_0000 + k);
                    c.bus.write32(OUT_NRM + k * 4, 0xbeef_0000 + k);
                }
            },
        );
    }
}
