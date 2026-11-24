//! Staging module oc495 (wave wave227): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc495`.

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
    0x0017_28f8,
    camera_box_trigger_contains_point,
    "CameraBoxTrigger_ContainsPoint",
)];

/// `base + off` with `base` a register's low word (the generated
/// `(c.g[R][0] as u32).wrapping_add(off)`).
#[inline(always)]
fn cvb_at(c: &Cpu, reg: usize, off: u32) -> u32 {
    r32(c, reg).wrapping_add(off)
}

/// `base - back` with `base` a register's low word (the generated
/// `wrapping_sub`).
#[inline(always)]
fn cvb_back(c: &Cpu, reg: usize, back: u32) -> u32 {
    r32(c, reg).wrapping_sub(back)
}

/// `bool cCTVolumeBox_containsPoint(cCTVolumeBox* box, t4Vector* pt)`
/// (0x1728f8), the `CameraBoxTrigger` containment test. The point is mapped
/// into the box's local frame: the identity matrix is turned by `-rot[1]`
/// about x and `-rot[0]` about z (`Math_SinCos` for each angle, the matrix
/// products as VU0 operations on the stack copies) and translated by `-pos`.
/// The box extends `size` either way along each axis; the point is inside
/// when it is on the non-positive side of all six slab planes. The three axis
/// vectors are copied into the static table at `0x4c5480` on the first call
/// (the flag is `gp + 0x1cfc`). Returns 1 inside, 0 outside.
///
/// Transcribed op for op from the generated body: the VU0 and FPU steps and
/// the stack spills stay as the original issues them.
pub fn camera_box_trigger_contains_point(c: &mut Cpu) {
    guest(c, |c| {
        'b_172d38: {
            c.cycles += 260;
            c.g[SP][0] = sx(cvb_back(c, SP, 0x130));
            c.g[V0][0] = 0x500000;
            c.sq(17, cvb_at(c, SP, 0x100));
            c.g[V0][0] = sx(cvb_back(c, V0, 0xe60));
            c.sq(20, cvb_at(c, SP, 0xd0));
            c.g[S1][0] = c.g[A0][0];
            c.g[S4][0] = c.g[A1][0];
            c.sq(16, cvb_at(c, SP, 0x110));
            c.sq(18, cvb_at(c, SP, 0xf0));
            c.sq(19, cvb_at(c, SP, 0xe0));
            c.sq(21, cvb_at(c, SP, 0xc0));
            c.sq(22, cvb_at(c, SP, 0xb0));
            c.sd(cvb_at(c, SP, 0xa0), c.g[RA][0]);
            c.swc1(20, cvb_at(c, SP, 0x120));
            c.lqc2(1, r32(c, V0));
            c.lqc2(2, cvb_at(c, V0, 0x10));
            c.lqc2(3, cvb_at(c, V0, 0x20));
            c.lqc2(4, cvb_at(c, V0, 0x30));
            c.sqc2(1, r32(c, SP));
            c.sqc2(2, cvb_at(c, SP, 0x10));
            c.sqc2(3, cvb_at(c, SP, 0x20));
            c.sqc2(4, cvb_at(c, SP, 0x30));
            c.lwc1(12, cvb_at(c, S1, 0x10));
            c.g[S2][0] = c.g[SP][0];
            c.g[A0][0] = sx(cvb_at(c, SP, 0x90));
            c.g[A1][0] = sx(cvb_at(c, SP, 0x94));
            c.neg_s(12, 12);
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 20);
            c.g[S0][0] = 0x500000;
            c.g[S6][0] = sx(cvb_back(c, S0, 0xec0));
            call_ptr(c, 0x0031be50, 0x00172978)?;
            c.lwc1(11, cvb_at(c, SP, 0x94));
            c.g[S3][0] = sx(cvb_at(c, SP, 0x40));
            c.lwc1(3, cvb_at(c, S6, 0x8));
            c.sub_s(1, 20, 11);
            c.lwc1(0, cvb_back(c, S0, 0xec0));
            c.lwc1(6, cvb_at(c, S6, 0x4));
            c.lwc1(8, cvb_at(c, SP, 0x90));
            c.mul_s(2, 1, 3);
            c.sw(cvb_at(c, SP, 0x70), 0u64);
            c.mul_s(4, 1, 0);
            c.sw(cvb_at(c, SP, 0x74), 0u64);
            c.mul_s(1, 1, 6);
            c.sw(cvb_at(c, SP, 0x78), 0u64);
            c.mul_s(13, 8, 3);
            c.sw(cvb_at(c, SP, 0x4c), 0u64);
            c.mul_s(12, 8, 0);
            c.sw(cvb_at(c, SP, 0x5c), 0u64);
            c.mul_s(9, 2, 3);
            c.sw(cvb_at(c, SP, 0x6c), 0u64);
            c.mul_s(10, 4, 3);
            c.swc1(20, cvb_at(c, SP, 0x7c));
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
            c.swc1(5, cvb_at(c, SP, 0x48));
            c.add_s(7, 7, 11);
            c.swc1(0, cvb_at(c, SP, 0x44));
            c.add_s(4, 4, 13);
            c.swc1(2, cvb_at(c, SP, 0x58));
            c.sub_s(10, 10, 8);
            c.swc1(9, cvb_at(c, SP, 0x68));
            c.add_s(1, 1, 11);
            c.swc1(7, cvb_at(c, SP, 0x40));
            c.add_s(3, 3, 12);
            c.swc1(4, cvb_at(c, SP, 0x50));
            c.swc1(10, cvb_at(c, SP, 0x60));
            c.swc1(1, cvb_at(c, SP, 0x54));
            c.swc1(3, cvb_at(c, SP, 0x64));
            c.lqc2(4, r32(c, S2));
            c.lqc2(5, cvb_at(c, S2, 0x10));
            c.lqc2(6, cvb_at(c, S2, 0x20));
            c.lqc2(7, cvb_at(c, S2, 0x30));
            c.lqc2(8, r32(c, S3));
            c.lqc2(9, cvb_at(c, S3, 0x10));
            c.lqc2(10, cvb_at(c, S3, 0x20));
            c.lqc2(11, cvb_at(c, S3, 0x30));
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
            c.sqc2(12, r32(c, S2));
            c.sqc2(13, cvb_at(c, S2, 0x10));
            c.sqc2(14, cvb_at(c, S2, 0x20));
            c.sqc2(15, cvb_at(c, S2, 0x30));
            c.lwc1(12, cvb_at(c, S1, 0xc));
            c.g[S2][0] = c.g[SP][0];
            c.g[A0][0] = sx(cvb_at(c, SP, 0x98));
            c.g[A1][0] = sx(cvb_at(c, SP, 0x9c));
            c.neg_s(12, 12);
            c.g[S0][0] = 0x500000;
            c.g[S5][0] = sx(cvb_back(c, S0, 0xea0));
            call_ptr(c, 0x0031be50, 0x00172ac4)?;
            c.lwc1(11, cvb_at(c, SP, 0x9c));
            c.lwc1(3, cvb_at(c, S5, 0x8));
            c.sub_s(1, 20, 11);
            c.lwc1(0, cvb_back(c, S0, 0xea0));
            c.lwc1(6, cvb_at(c, S5, 0x4));
            c.lwc1(8, cvb_at(c, SP, 0x98));
            c.mul_s(2, 1, 3);
            c.sw(cvb_at(c, SP, 0x70), 0u64);
            c.mul_s(4, 1, 0);
            c.sw(cvb_at(c, SP, 0x74), 0u64);
            c.mul_s(1, 1, 6);
            c.sw(cvb_at(c, SP, 0x78), 0u64);
            c.mul_s(13, 8, 3);
            c.sw(cvb_at(c, SP, 0x4c), 0u64);
            c.mul_s(12, 8, 0);
            c.sw(cvb_at(c, SP, 0x5c), 0u64);
            c.mul_s(9, 2, 3);
            c.sw(cvb_at(c, SP, 0x6c), 0u64);
            c.mul_s(10, 4, 3);
            c.swc1(20, cvb_at(c, SP, 0x7c));
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
            c.swc1(5, cvb_at(c, SP, 0x48));
            c.add_s(7, 7, 11);
            c.swc1(0, cvb_at(c, SP, 0x44));
            c.add_s(4, 4, 13);
            c.swc1(2, cvb_at(c, SP, 0x58));
            c.sub_s(10, 10, 8);
            c.swc1(9, cvb_at(c, SP, 0x68));
            c.add_s(1, 1, 11);
            c.swc1(7, cvb_at(c, SP, 0x40));
            c.add_s(3, 3, 12);
            c.swc1(4, cvb_at(c, SP, 0x50));
            c.swc1(10, cvb_at(c, SP, 0x60));
            c.swc1(1, cvb_at(c, SP, 0x54));
            c.swc1(3, cvb_at(c, SP, 0x64));
            c.lqc2(4, r32(c, S2));
            c.lqc2(5, cvb_at(c, S2, 0x10));
            c.lqc2(6, cvb_at(c, S2, 0x20));
            c.lqc2(7, cvb_at(c, S2, 0x30));
            c.lqc2(8, r32(c, S3));
            c.lqc2(9, cvb_at(c, S3, 0x10));
            c.lqc2(10, cvb_at(c, S3, 0x20));
            c.lqc2(11, cvb_at(c, S3, 0x30));
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
            c.sqc2(12, r32(c, S2));
            c.sqc2(13, cvb_at(c, S2, 0x10));
            c.sqc2(14, cvb_at(c, S2, 0x20));
            c.sqc2(15, cvb_at(c, S2, 0x30));
            c.lwc1(0, r32(c, S1));
            c.lwc1(1, cvb_at(c, S1, 0x4));
            c.lwc1(2, cvb_at(c, S1, 0x8));
            c.neg_s(0, 0);
            c.neg_s(1, 1);
            c.swc1(20, cvb_at(c, SP, 0x4c));
            c.neg_s(2, 2);
            c.swc1(0, cvb_at(c, SP, 0x40));
            c.swc1(1, cvb_at(c, SP, 0x44));
            c.swc1(2, cvb_at(c, SP, 0x48));
            c.swc1(0, cvb_at(c, SP, 0x50));
            c.swc1(1, cvb_at(c, SP, 0x54));
            c.swc1(2, cvb_at(c, SP, 0x58));
            c.lqc2(4, r32(c, SP));
            c.lqc2(5, cvb_at(c, SP, 0x10));
            c.lqc2(6, cvb_at(c, SP, 0x20));
            c.lqc2(7, cvb_at(c, SP, 0x30));
            c.lqc2(8, cvb_at(c, SP, 0x40));
            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
            c.sqc2(12, cvb_at(c, SP, 0x30));
            c.lqc2(8, r32(c, S4));
            c.lqc2(4, r32(c, SP));
            c.lqc2(5, cvb_at(c, SP, 0x10));
            c.lqc2(6, cvb_at(c, SP, 0x20));
            c.lqc2(7, cvb_at(c, SP, 0x30));
            c.vu0().mula(0b1111, 4, Src::Bc(8, X));
            c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
            c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
            c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
            c.sqc2(12, cvb_at(c, SP, 0x50));
            c.lwc1(0, cvb_at(c, S1, 0x18));
            c.lwc1(1, cvb_at(c, S1, 0x1c));
            c.lwc1(2, cvb_at(c, S1, 0x20));
            c.neg_s(0, 0);
            c.lq(2, cvb_at(c, SP, 0x50));
            c.neg_s(1, 1);
            c.neg_s(2, 2);
            c.swc1(20, cvb_at(c, SP, 0x6c));
            c.sq(2, cvb_at(c, SP, 0x40));
            c.swc1(0, cvb_at(c, SP, 0x70));
            c.swc1(1, cvb_at(c, SP, 0x74));
            c.swc1(2, cvb_at(c, SP, 0x78));
            c.swc1(0, cvb_at(c, SP, 0x60));
            c.swc1(1, cvb_at(c, SP, 0x64));
            c.swc1(2, cvb_at(c, SP, 0x68));
            c.swc1(20, cvb_at(c, SP, 0x4c));
            c.lqc2(3, cvb_at(c, SP, 0x40));
            c.lqc2(4, cvb_at(c, SP, 0x60));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, cvb_at(c, SP, 0x80));
            c.lwc1(2, cvb_at(c, S1, 0x20));
            c.lwc1(0, cvb_at(c, S1, 0x18));
            c.lwc1(1, cvb_at(c, S1, 0x1c));
            c.lq(2, cvb_at(c, SP, 0x80));
            c.swc1(0, cvb_at(c, SP, 0x70));
            c.sq(2, cvb_at(c, SP, 0x50));
            c.swc1(1, cvb_at(c, SP, 0x74));
            c.swc1(2, cvb_at(c, SP, 0x78));
            c.swc1(20, cvb_at(c, SP, 0x7c));
            c.lqc2(3, cvb_at(c, SP, 0x40));
            c.lqc2(4, cvb_at(c, SP, 0x70));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, cvb_at(c, SP, 0x80));
            c.lq(2, cvb_at(c, SP, 0x80));
            c.sq(2, cvb_at(c, SP, 0x60));
            c.g[V1][0] = c.lw(cvb_at(c, GP, 0x1cfc));
            let t = c.g[V1][0] != 0u64;
            c.g[V0][0] = 0x4c0000;
            if t {
                break 'b_172d38;
            } else {
                c.cycles += 12;
                c.g[V1][0] = 0x500000;
                c.lq(6, r32(c, S6));
                c.g[V1][0] = sx(cvb_back(c, V1, 0xeb0));
                c.lq(4, r32(c, S5));
                c.lq(5, r32(c, V1));
                c.g[V0][0] = sx(cvb_at(c, V0, 0x5480));
                c.g[V1][0] = 0x1;
                c.sq(4, cvb_at(c, V0, 0x20));
                c.sq(6, r32(c, V0));
                c.sq(5, cvb_at(c, V0, 0x10));
                c.sw(cvb_at(c, GP, 0x1cfc), c.g[V1][0]);
                c.g[V0][0] = 0x4c0000;
                break 'b_172d38;
            }
        }
        c.cycles += 6;
        c.g[AT][0] = 0xffffffffbf800000;
        c.mtc1(1, 2);
        c.mtc1(0, 1);
        c.g[A0][0] = sx(cvb_at(c, V0, 0x5480));
        c.g[A1][0] = 0u64;
        'l_172d50: loop {
            'b_172de8: {
                c.cycles += 20;
                c.mfc1(2, 2);
                c.lqc2(4, r32(c, A0));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, cvb_at(c, SP, 0x80));
                c.lq(3, cvb_at(c, SP, 0x80));
                c.sq(3, cvb_at(c, SP, 0x70));
                c.lqc2(3, cvb_at(c, SP, 0x70));
                c.lqc2(5, cvb_at(c, SP, 0x50));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(2, 4);
                c.mtc1(2, 0);
                c.c_lt_s(1, 0);
                let t = c.fcc();
                c.g[V0][0] = 0u64;
                if t {
                    break 'b_172de8;
                } else {
                    c.cycles += 13;
                    c.lqc2(3, r32(c, A0));
                    c.lqc2(5, cvb_at(c, SP, 0x60));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(2, 4);
                    c.mtc1(2, 0);
                    c.c_lt_s(1, 0);
                    let t = c.fcc();
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_172de8;
                    } else {
                        c.cycles += 4;
                        c.g[A1][0] = sx(cvb_at(c, A1, 0x1));
                        c.g[V0][0] = ((c.g[A1][0] as i64) < 3i64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A0][0] = sx(cvb_at(c, A0, 0x10));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0017_2d50);
                            }
                            continue 'l_172d50;
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = 0x1;
                            break 'b_172de8;
                        }
                    }
                }
            }
            c.cycles += 11;
            c.lq(16, cvb_at(c, SP, 0x110));
            c.lq(17, cvb_at(c, SP, 0x100));
            c.lq(18, cvb_at(c, SP, 0xf0));
            c.lq(19, cvb_at(c, SP, 0xe0));
            c.lq(20, cvb_at(c, SP, 0xd0));
            c.lq(21, cvb_at(c, SP, 0xc0));
            c.lq(22, cvb_at(c, SP, 0xb0));
            c.g[RA][0] = c.ld(cvb_at(c, SP, 0xa0));
            c.lwc1(20, cvb_at(c, SP, 0x120));
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx(cvb_at(c, SP, 0x130));
            c.pc = t;
            return Ok(());
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, elf_image, fill, put_vec, scramble, Rng, DATA, GAME_GP};

    /// The box the function reads (position, rot[0..3], size).
    const CVB_TEST_BOX: u32 = DATA;
    /// The `Vec4` point.
    const CVB_TEST_PT: u32 = DATA + 0x100;

    #[test]
    fn camera_box_trigger_contains_point_matches() {
        check(
            ee::s02::f_001728f8__generated,
            camera_box_trigger_contains_point,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = Rng(0x0017_28f8 ^ (seed as u64) << 20 ^ 0x9e37_79b9);
                scramble(c, &mut r);
                c.pc = 0x0017_28f8;
                fill(c, &mut r, crate::difftest::SP - 0x200, 0x80);
                // A value in -m..m, as float bits.
                let sym = |r: &mut Rng, m: f32| {
                    (((r.next_u32() as f32 / u32::MAX as f32) * 2.0 - 1.0) * m).to_bits()
                };
                // Position in +-50, angles in +-7 rad, size in 0.5..3.5.
                let pos: [f32; 3] = std::array::from_fn(|_| f32::from_bits(sym(&mut r, 50.0)));
                for (k, p) in pos.iter().enumerate() {
                    c.bus.write32(CVB_TEST_BOX + 4 * k as u32, p.to_bits());
                }
                for k in 0..3u32 {
                    c.bus.write32(CVB_TEST_BOX + 0xc + 4 * k, sym(&mut r, 7.0));
                }
                for k in 0..3u32 {
                    let s = 0.5 + (r.next_u32() as f32 / u32::MAX as f32) * 3.0;
                    c.bus.write32(CVB_TEST_BOX + 0x18 + 4 * k, s.to_bits());
                }
                // Even seeds put the point near the box (usually inside), odd
                // seeds far away (outside).
                let spread = if seed % 2 == 0 { 0.3 } else { 20.0 };
                let mut v = [0u32; 4];
                for k in 0..3 {
                    v[k] = (pos[k] + f32::from_bits(sym(&mut r, spread))).to_bits();
                }
                v[3] = r.next_u32();
                put_vec(c, CVB_TEST_PT, v);
                // The axis table's first-call flag: both states.
                c.bus.write32(
                    GAME_GP.wrapping_add(0x1cfc),
                    if seed % 4 == 0 { 1 } else { 0 },
                );
                c.g[GP][0] = sx(GAME_GP);
                c.g[A0][0] = sx(CVB_TEST_BOX);
                c.g[A1][0] = sx(CVB_TEST_PT);
            },
        );
    }
}
