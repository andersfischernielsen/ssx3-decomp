//! Staging module oc494 (wave wave227): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc494`.

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
    0x0016_fbb8,
    camera_line_bound_obj_clamp_point,
    "CameraLineBoundObj_ClampPoint",
)];

/// `base + off` with `base` a register's low word (the generated
/// `(c.g[R][0] as u32).wrapping_add(off)`).
#[inline(always)]
fn lbcp_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    r32(c, reg).wrapping_add(off)
}

/// `base - back` with `base` a register's low word (the generated
/// `wrapping_sub`).
#[inline(always)]
fn lbcp_back(c: &Cpu, reg: usize, back: u32) -> u32 {
    r32(c, reg).wrapping_sub(back)
}

/// `void CameraLineBoundObj_ClampPoint(cCTBoundObjLine* src, Vec4* point)`
/// (0x16fbb8; C++ `cCTBoundObjLine_limitPoint`). The line's two end points
/// (`Point[0]`, `Point[1]`) are mapped by the object's transform (translate
/// by `Pos`, rotate by pitch, roll and heading, scale), and `point` is
/// replaced by its projection onto the line through them: `pts[0] + dir * t`,
/// with `t = dot(point - pts[0], dir) / dot(dir, dir)`. The generated body has
/// no clamp to the segment, so `t` is not limited to [0, 1]. The transform
/// is built as VU0 products on stack copies of the identity matrix.
pub fn camera_line_bound_obj_clamp_point(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 417;
        // Copy the identity matrix to the stack, then load the object (src) and
        // its transform inputs; the C++ builds the matrix from them.
        c.g[SP][0] = sx(lbcp_back(c, SP, 0x150));
        c.g[V0][0] = 0x500000;
        c.sq(18, lbcp_at(c, SP, 0x110));
        c.g[V0][0] = sx(lbcp_back(c, V0, 0xe60));
        c.sq(19, lbcp_at(c, SP, 0x100));
        c.g[S2][0] = c.g[A0][0];
        c.sq(16, lbcp_at(c, SP, 0x130));
        c.g[S3][0] = c.g[A1][0];
        c.sq(17, lbcp_at(c, SP, 0x120));
        c.sd(lbcp_at(c, SP, 0xf0), c.g[RA][0]);
        c.swc1(20, lbcp_at(c, SP, 0x140));
        c.lqc2(1, r32(c, V0));
        c.lqc2(2, lbcp_at(c, V0, 0x10));
        c.lqc2(3, lbcp_at(c, V0, 0x20));
        c.lqc2(4, lbcp_at(c, V0, 0x30));
        c.sqc2(1, r32(c, SP));
        c.sqc2(2, lbcp_at(c, SP, 0x10));
        c.sqc2(3, lbcp_at(c, SP, 0x20));
        c.sqc2(4, lbcp_at(c, SP, 0x30));
        c.lwc1(0, r32(c, S2));
        c.lwc1(1, lbcp_at(c, S2, 0x4));
        c.lwc1(2, lbcp_at(c, S2, 0x8));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 20);
        c.swc1(0, lbcp_at(c, SP, 0x40));
        c.swc1(1, lbcp_at(c, SP, 0x44));
        c.swc1(2, lbcp_at(c, SP, 0x48));
        c.swc1(20, lbcp_at(c, SP, 0x4c));
        c.lqc2(4, r32(c, SP));
        c.lqc2(5, lbcp_at(c, SP, 0x10));
        c.lqc2(6, lbcp_at(c, SP, 0x20));
        c.lqc2(7, lbcp_at(c, SP, 0x30));
        c.lqc2(8, lbcp_at(c, SP, 0x40));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, lbcp_at(c, SP, 0x30));
        c.lwc1(12, lbcp_at(c, S2, 0x10));
        c.g[A0][0] = sx(lbcp_at(c, SP, 0xd0));
        c.g[S0][0] = c.g[SP][0];
        c.g[A1][0] = sx(lbcp_at(c, SP, 0xd4));
        // fSinCos (0x31be50) on each angle: the rotation matrices.
        call_ptr(c, 0x0031be50, 0x0016fc64)?;
        c.lwc1(11, lbcp_at(c, SP, 0xd4));
        c.g[V0][0] = 0x500000;
        c.g[V1][0] = sx(lbcp_back(c, V0, 0xec0));
        c.lwc1(0, lbcp_back(c, V0, 0xec0));
        c.sub_s(1, 20, 11);
        c.lwc1(3, lbcp_at(c, V1, 0x8));
        c.lwc1(6, lbcp_at(c, V1, 0x4));
        c.g[A0][0] = sx(lbcp_at(c, SP, 0x50));
        c.lwc1(8, lbcp_at(c, SP, 0xd0));
        c.mul_s(2, 1, 3);
        c.sw(lbcp_at(c, SP, 0x80), 0u64);
        c.mul_s(4, 1, 0);
        c.sw(lbcp_at(c, SP, 0x84), 0u64);
        c.mul_s(1, 1, 6);
        c.sw(lbcp_at(c, SP, 0x88), 0u64);
        c.mul_s(13, 8, 3);
        c.sw(lbcp_at(c, SP, 0x5c), 0u64);
        c.mul_s(12, 8, 0);
        c.sw(lbcp_at(c, SP, 0x6c), 0u64);
        c.mul_s(9, 2, 3);
        c.sw(lbcp_at(c, SP, 0x7c), 0u64);
        c.mul_s(10, 4, 3);
        c.swc1(20, lbcp_at(c, SP, 0x8c));
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
        c.swc1(5, lbcp_at(c, SP, 0x58));
        c.add_s(7, 7, 11);
        c.swc1(0, lbcp_at(c, SP, 0x54));
        c.add_s(4, 4, 13);
        c.swc1(2, lbcp_at(c, SP, 0x68));
        c.sub_s(10, 10, 8);
        c.swc1(9, lbcp_at(c, SP, 0x78));
        c.add_s(1, 1, 11);
        c.swc1(7, lbcp_at(c, SP, 0x50));
        c.add_s(3, 3, 12);
        c.swc1(4, lbcp_at(c, SP, 0x60));
        c.swc1(10, lbcp_at(c, SP, 0x70));
        c.swc1(1, lbcp_at(c, SP, 0x64));
        c.swc1(3, lbcp_at(c, SP, 0x74));
        c.lqc2(4, r32(c, S0));
        c.lqc2(5, lbcp_at(c, S0, 0x10));
        c.lqc2(6, lbcp_at(c, S0, 0x20));
        c.lqc2(7, lbcp_at(c, S0, 0x30));
        c.lqc2(8, r32(c, A0));
        c.lqc2(9, lbcp_at(c, A0, 0x10));
        c.lqc2(10, lbcp_at(c, A0, 0x20));
        c.lqc2(11, lbcp_at(c, A0, 0x30));
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
        c.sqc2(12, r32(c, S0));
        c.sqc2(13, lbcp_at(c, S0, 0x10));
        c.sqc2(14, lbcp_at(c, S0, 0x20));
        c.sqc2(15, lbcp_at(c, S0, 0x30));
        c.lwc1(12, lbcp_at(c, S2, 0x14));
        c.g[S1][0] = c.g[SP][0];
        c.g[A0][0] = sx(lbcp_at(c, SP, 0xd8));
        c.g[A1][0] = sx(lbcp_at(c, SP, 0xdc));
        // fSinCos (0x31be50) on the next angle.
        call_ptr(c, 0x0031be50, 0x0016fdac)?;
        c.lwc1(11, lbcp_at(c, SP, 0xdc));
        c.g[V0][0] = 0x500000;
        c.g[V1][0] = sx(lbcp_back(c, V0, 0xeb0));
        c.lwc1(0, lbcp_back(c, V0, 0xeb0));
        c.sub_s(1, 20, 11);
        c.lwc1(3, lbcp_at(c, V1, 0x8));
        c.lwc1(6, lbcp_at(c, V1, 0x4));
        c.g[S0][0] = sx(lbcp_at(c, SP, 0x40));
        c.lwc1(8, lbcp_at(c, SP, 0xd8));
        c.mul_s(2, 1, 3);
        c.sw(lbcp_at(c, SP, 0x70), 0u64);
        c.mul_s(4, 1, 0);
        c.sw(lbcp_at(c, SP, 0x74), 0u64);
        c.mul_s(1, 1, 6);
        c.sw(lbcp_at(c, SP, 0x78), 0u64);
        c.mul_s(13, 8, 3);
        c.sw(lbcp_at(c, SP, 0x4c), 0u64);
        c.mul_s(12, 8, 0);
        c.sw(lbcp_at(c, SP, 0x5c), 0u64);
        c.mul_s(9, 2, 3);
        c.sw(lbcp_at(c, SP, 0x6c), 0u64);
        c.mul_s(10, 4, 3);
        c.swc1(20, lbcp_at(c, SP, 0x7c));
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
        c.swc1(5, lbcp_at(c, SP, 0x48));
        c.add_s(7, 7, 11);
        c.swc1(0, lbcp_at(c, SP, 0x44));
        c.add_s(4, 4, 13);
        c.swc1(2, lbcp_at(c, SP, 0x58));
        c.sub_s(10, 10, 8);
        c.swc1(9, lbcp_at(c, SP, 0x68));
        c.add_s(1, 1, 11);
        c.swc1(7, lbcp_at(c, SP, 0x40));
        c.add_s(3, 3, 12);
        c.swc1(4, lbcp_at(c, SP, 0x50));
        c.swc1(10, lbcp_at(c, SP, 0x60));
        c.swc1(1, lbcp_at(c, SP, 0x54));
        c.swc1(3, lbcp_at(c, SP, 0x64));
        c.lqc2(4, r32(c, S1));
        c.lqc2(5, lbcp_at(c, S1, 0x10));
        c.lqc2(6, lbcp_at(c, S1, 0x20));
        c.lqc2(7, lbcp_at(c, S1, 0x30));
        c.lqc2(8, r32(c, S0));
        c.lqc2(9, lbcp_at(c, S0, 0x10));
        c.lqc2(10, lbcp_at(c, S0, 0x20));
        c.lqc2(11, lbcp_at(c, S0, 0x30));
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
        c.sqc2(12, r32(c, S1));
        c.sqc2(13, lbcp_at(c, S1, 0x10));
        c.sqc2(14, lbcp_at(c, S1, 0x20));
        c.sqc2(15, lbcp_at(c, S1, 0x30));
        c.lwc1(12, lbcp_at(c, S2, 0xc));
        c.g[S1][0] = c.g[SP][0];
        c.g[A0][0] = sx(lbcp_at(c, SP, 0xe0));
        c.g[A1][0] = sx(lbcp_at(c, SP, 0xe4));
        // fSinCos (0x31be50) on the next angle.
        call_ptr(c, 0x0031be50, 0x0016fef4)?;
        c.lwc1(11, lbcp_at(c, SP, 0xe4));
        c.g[V0][0] = 0x500000;
        c.g[V1][0] = sx(lbcp_back(c, V0, 0xea0));
        c.lwc1(0, lbcp_back(c, V0, 0xea0));
        c.sub_s(1, 20, 11);
        c.lwc1(3, lbcp_at(c, V1, 0x8));
        c.lwc1(6, lbcp_at(c, V1, 0x4));
        c.lwc1(8, lbcp_at(c, SP, 0xe0));
        c.mul_s(2, 1, 3);
        c.sw(lbcp_at(c, SP, 0x70), 0u64);
        c.mul_s(4, 1, 0);
        c.sw(lbcp_at(c, SP, 0x74), 0u64);
        c.mul_s(1, 1, 6);
        c.sw(lbcp_at(c, SP, 0x78), 0u64);
        c.mul_s(13, 8, 3);
        c.sw(lbcp_at(c, SP, 0x4c), 0u64);
        c.mul_s(12, 8, 0);
        c.sw(lbcp_at(c, SP, 0x5c), 0u64);
        c.mul_s(9, 2, 3);
        c.sw(lbcp_at(c, SP, 0x6c), 0u64);
        c.mul_s(10, 4, 3);
        c.swc1(20, lbcp_at(c, SP, 0x7c));
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
        c.swc1(5, lbcp_at(c, SP, 0x48));
        c.add_s(7, 7, 11);
        c.swc1(0, lbcp_at(c, SP, 0x44));
        c.add_s(4, 4, 13);
        c.swc1(2, lbcp_at(c, SP, 0x58));
        c.sub_s(10, 10, 8);
        c.swc1(9, lbcp_at(c, SP, 0x68));
        c.add_s(1, 1, 11);
        c.swc1(7, lbcp_at(c, SP, 0x40));
        c.add_s(3, 3, 12);
        c.swc1(4, lbcp_at(c, SP, 0x50));
        c.swc1(10, lbcp_at(c, SP, 0x60));
        c.swc1(1, lbcp_at(c, SP, 0x54));
        c.swc1(3, lbcp_at(c, SP, 0x64));
        c.lqc2(4, r32(c, S1));
        c.lqc2(5, lbcp_at(c, S1, 0x10));
        c.lqc2(6, lbcp_at(c, S1, 0x20));
        c.lqc2(7, lbcp_at(c, S1, 0x30));
        c.lqc2(8, r32(c, S0));
        c.lqc2(9, lbcp_at(c, S0, 0x10));
        c.lqc2(10, lbcp_at(c, S0, 0x20));
        c.lqc2(11, lbcp_at(c, S0, 0x30));
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
        c.sqc2(12, r32(c, S1));
        c.sqc2(13, lbcp_at(c, S1, 0x10));
        c.sqc2(14, lbcp_at(c, S1, 0x20));
        c.sqc2(15, lbcp_at(c, S1, 0x30));
        c.lwc1(2, lbcp_at(c, S2, 0x18));
        c.g[V0][0] = sx(lbcp_at(c, SP, 0x90));
        c.lwc1(1, lbcp_at(c, S2, 0x1c));
        c.lwc1(0, lbcp_at(c, S2, 0x20));
        c.swc1(2, lbcp_at(c, SP, 0x80));
        c.swc1(1, lbcp_at(c, SP, 0x84));
        c.swc1(0, lbcp_at(c, SP, 0x88));
        c.swc1(20, lbcp_at(c, SP, 0x8c));
        c.lqc2(1, lbcp_at(c, SP, 0x80));
        c.lqc2(4, r32(c, SP));
        c.lqc2(5, lbcp_at(c, SP, 0x10));
        c.lqc2(6, lbcp_at(c, SP, 0x20));
        c.lqc2(7, lbcp_at(c, SP, 0x30));
        c.vu0().mul(0b1111, 8, 4, Src::Bc(1, X));
        c.vu0().mul(0b1111, 9, 5, Src::Bc(1, Y));
        c.vu0().mul(0b1111, 10, 6, Src::Bc(1, Z));
        c.vu0().mul(0b1111, 11, 7, Src::Bc(1, W));
        c.sqc2(8, r32(c, V0));
        c.sqc2(9, lbcp_at(c, V0, 0x10));
        c.sqc2(10, lbcp_at(c, V0, 0x20));
        c.sqc2(11, lbcp_at(c, V0, 0x30));
        c.lqc2(1, r32(c, V0));
        c.lqc2(2, lbcp_at(c, V0, 0x10));
        c.lqc2(3, lbcp_at(c, V0, 0x20));
        c.lqc2(4, lbcp_at(c, V0, 0x30));
        c.sqc2(1, r32(c, S0));
        c.sqc2(2, lbcp_at(c, S0, 0x10));
        c.sqc2(3, lbcp_at(c, S0, 0x20));
        c.sqc2(4, lbcp_at(c, S0, 0x30));
        c.lqc2(1, r32(c, S0));
        c.lqc2(2, lbcp_at(c, S0, 0x10));
        c.lqc2(3, lbcp_at(c, S0, 0x20));
        c.lqc2(4, lbcp_at(c, S0, 0x30));
        c.sqc2(1, r32(c, SP));
        c.sqc2(2, lbcp_at(c, SP, 0x10));
        c.sqc2(3, lbcp_at(c, SP, 0x20));
        c.sqc2(4, lbcp_at(c, SP, 0x30));
        c.g[V1][0] = 0xffffffffffffffff;
        c.g[V0][0] = 0x1;
        c.lwc1(0, lbcp_at(c, S2, 0x2c));
        c.lwc1(1, lbcp_at(c, S2, 0x30));
        c.lwc1(2, lbcp_at(c, S2, 0x34));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 3);
        c.swc1(0, lbcp_at(c, SP, 0x70));
        c.swc1(1, lbcp_at(c, SP, 0x74));
        c.swc1(2, lbcp_at(c, SP, 0x78));
        c.swc1(3, lbcp_at(c, SP, 0x7c));
        c.lqc2(8, lbcp_at(c, SP, 0x70));
        c.lqc2(4, r32(c, SP));
        c.lqc2(5, lbcp_at(c, SP, 0x10));
        c.lqc2(6, lbcp_at(c, SP, 0x20));
        c.lqc2(7, lbcp_at(c, SP, 0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, lbcp_at(c, SP, 0x80));
        c.lwc1(2, lbcp_at(c, S2, 0x40));
        c.lwc1(0, lbcp_at(c, S2, 0x38));
        c.lwc1(1, lbcp_at(c, S2, 0x3c));
        c.lq(2, lbcp_at(c, SP, 0x80));
        c.swc1(0, lbcp_at(c, SP, 0x70));
        c.sq(2, lbcp_at(c, SP, 0x60));
        c.swc1(1, lbcp_at(c, SP, 0x74));
        c.swc1(2, lbcp_at(c, SP, 0x78));
        c.swc1(3, lbcp_at(c, SP, 0x7c));
        c.sq(2, lbcp_at(c, SP, 0x40));
        c.lqc2(8, lbcp_at(c, SP, 0x70));
        c.lqc2(4, r32(c, SP));
        c.lqc2(5, lbcp_at(c, SP, 0x10));
        c.lqc2(6, lbcp_at(c, SP, 0x20));
        c.lqc2(7, lbcp_at(c, SP, 0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, lbcp_at(c, SP, 0x80));
        c.lq(2, lbcp_at(c, SP, 0x80));
        c.sq(2, lbcp_at(c, SP, 0x60));
        c.sq(2, lbcp_at(c, SP, 0x50));
        c.lqc2(3, lbcp_at(c, SP, 0x50));
        c.lqc2(4, lbcp_at(c, SP, 0x40));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, lbcp_at(c, SP, 0x70));
        c.lq(2, lbcp_at(c, SP, 0x70));
        c.sq(2, lbcp_at(c, SP, 0x60));
        c.lqc2(3, r32(c, S3));
        c.lqc2(4, lbcp_at(c, SP, 0x40));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, lbcp_at(c, SP, 0x80));
        c.lq(2, lbcp_at(c, SP, 0x80));
        c.lq(16, lbcp_at(c, SP, 0x130));
        c.sq(2, lbcp_at(c, SP, 0x70));
        c.lqc2(3, lbcp_at(c, SP, 0x60));
        c.lqc2(5, lbcp_at(c, SP, 0x60));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(2, 4);
        c.mtc1(2, 1);
        c.lqc2(3, lbcp_at(c, SP, 0x70));
        c.lqc2(5, lbcp_at(c, SP, 0x60));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(3, 4);
        c.mtc1(3, 0);
        c.div_s(0, 0, 1);
        c.lq(17, lbcp_at(c, SP, 0x120));
        c.mfc1(3, 0);
        c.lqc2(4, lbcp_at(c, SP, 0x60));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, lbcp_at(c, SP, 0x90));
        c.lq(18, lbcp_at(c, SP, 0x110));
        c.lq(2, lbcp_at(c, SP, 0x90));
        c.g[RA][0] = c.ld(lbcp_at(c, SP, 0xf0));
        c.sq(2, lbcp_at(c, SP, 0x80));
        c.lqc2(3, lbcp_at(c, SP, 0x40));
        c.lqc2(4, lbcp_at(c, SP, 0x80));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, lbcp_at(c, SP, 0x90));
        c.lwc1(20, lbcp_at(c, SP, 0x140));
        c.lq(2, lbcp_at(c, SP, 0x90));
        c.sq(2, r32(c, S3));
        c.lq(19, lbcp_at(c, SP, 0x100));
        c.sq(2, lbcp_at(c, SP, 0x70));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(lbcp_at(c, SP, 0x150));
        c.pc = ra;
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, elf_image, fill, put_vec, scramble, Rng, DATA};

    /// The `cCTBoundObjLine` the transform and the end points are read from.
    const LBCP_SRC: u32 = DATA;
    /// The `Vec4` the point is read from and written to.
    const LBCP_POINT: u32 = DATA + 0x100;

    #[test]
    fn camera_line_bound_obj_clamp_point_matches() {
        check(
            ee::s02::f_0016fbb8__generated,
            camera_line_bound_obj_clamp_point,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = Rng(0x0016_fbb8 ^ (seed as u64) << 20 ^ 0x2545_f491);
                scramble(c, &mut r);
                c.pc = 0x0016_fbb8;
                fill(c, &mut r, crate::difftest::SP - 0x200, 0x80);
                // A float in -m..m, as bits.
                let sym = |r: &mut Rng, m: f32| {
                    (((r.next_u32() as f32 / u32::MAX as f32) * 2.0 - 1.0) * m).to_bits()
                };
                // Position, angles and end points in a small range.
                for k in 0..0x18u32 {
                    c.bus.write32(LBCP_SRC + 4 * k, sym(&mut r, 4.0));
                }
                // The three scale-like words are divided by: keep them positive.
                for k in 0..3u32 {
                    let s = 0.5 + (r.next_u32() as f32 / u32::MAX as f32) * 3.0;
                    c.bus.write32(LBCP_SRC + 0x18 + 4 * k, s.to_bits());
                }
                for k in 0..0x0cu32 {
                    c.bus.write32(LBCP_SRC + 0x24 + 4 * k, sym(&mut r, 4.0));
                }
                // The point: even seeds near the line, odd seeds far away.
                let spread = if seed % 2 == 0 { 0.3 } else { 20.0 };
                let mut v = [0u32; 4];
                for k in 0..3 {
                    v[k] = sym(&mut r, spread);
                }
                v[3] = r.next_u32();
                put_vec(c, LBCP_POINT, v);
                c.g[A0][0] = sx(LBCP_SRC);
                c.g[A1][0] = sx(LBCP_POINT);
            },
        );
    }
}
