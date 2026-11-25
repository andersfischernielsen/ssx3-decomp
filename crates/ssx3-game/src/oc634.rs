//! Staging module oc634 (wave 234, list oc634): hand-written replacements, each
//! with its differential test against the generated body. Moved into the
//! source-tree units by `tools/reorg_modules.py apply oc634`.

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
    (0x0016_07e0, matrix_chase_camera_update, "MatrixChaseCameraAlgorithm_Update"),
    (0x0016_1060, spline_camera_ctor, "SplineCameraAlgorithm_ctor"),
    (0x0016_13f0, spline_camera_update, "SplineCameraAlgorithm_Update"),
];

/// `sp + off` (read when the access is made, as the compiled code does).
#[inline]
fn cam634_sp(c: &Cpu, off: u32) -> u32 {
    sp(c).wrapping_add(off)
}

/// `gp - k` (the float and pointer constants the camera code loads).
#[inline]
fn cam634_gp(c: &Cpu, k: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(k)
}

/// `reg + off` for a camera object pointer held in register `r`.
#[inline]
fn cam634_reg(c: &Cpu, r: usize, off: u32) -> u32 {
    (c.g[r][0] as u32).wrapping_add(off)
}

/// `MatrixChaseCameraAlgorithm_Update(self)` (0x1607e0): the per-frame update of
/// the matrix chase camera. It gathers the rider state, and unless the chase
/// is at rest it asks the owner (`self+0x30`, through its vtable) for the
/// target point, moves the slew at `self+0x390` toward the target with VU0
/// and the cosine/sine of the heading, and stores the eye vector at
/// `self+0x20`. Returns to the caller with its saved registers restored.
pub fn matrix_chase_camera_update(c: &mut Cpu) {
    guest(c, |c| {
        'b_1608b4: {
            'b_16085c: {
                'b_160854: {
                    c.cycles += 23;
                    c.g[SP][0] = sx(sp(c).wrapping_sub(0x140));
                    c.lwc1(12, cam634_gp(c, 0x6be4));
                    c.sq(16, cam634_sp(c, 0x120));
                    c.g[A1][0] = c.g[SP][0];
                    c.lwc1(13, cam634_gp(c, 0x6be0));
                    c.lwc1(14, cam634_gp(c, 0x6bdc));
                    c.lwc1(15, cam634_gp(c, 0x6bd8));
                    c.lwc1(16, cam634_gp(c, 0x6bd4));
                    c.lwc1(17, cam634_gp(c, 0x6bd0));
                    c.lwc1(18, cam634_gp(c, 0x6bcc));
                    c.lwc1(19, cam634_gp(c, 0x6bc8));
                    c.swc1(20, cam634_sp(c, 0x130));
                    c.sq(17, cam634_sp(c, 0x110));
                    c.sd(cam634_sp(c, 0x100), c.g[RA][0]);
                    c.g[S0][0] = c.g[A0][0];
                    Callee {
                        addr: 0x0016_2568,
                        f: ee::s01::f_00162568,
                    }
                    .call(c, 0x0016_0820)?;
                    c.lwc1(20, cam634_gp(c, 0x6bc4));
                    c.lwc1(0, cam634_sp(c, 0x20));
                    c.abs_s(0, 0);
                    c.c_lt_s(20, 0);
                    let t = c.fcc();
                    c.lq(2, cam634_sp(c, 0x20));
                    if t {
                        break 'b_160854;
                    }
                    c.cycles += 6;
                    c.lwc1(0, cam634_sp(c, 0x24));
                    c.abs_s(0, 0);
                    c.c_lt_s(20, 0);
                    if c.fcc() {
                        break 'b_160854;
                    }
                    c.g[A2][0] = c.lw(cam634_reg(c, S0, 0x30));
                    break 'b_16085c;
                }
                // The first branch exits: store v0 and leave.
                c.cycles += 2;
                c.sq(2, cam634_sp(c, 0x70));
                break 'b_1608b4;
            }
            // The owner's virtual method gives the target point.
            c.cycles += 14;
            c.g[A0][0] = sx(cam634_sp(c, 0x80));
            c.g[V0][0] = c.lw(c.g[A2][0] as u32);
            c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
            let t = c.g[V1][0] as u32;
            c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
            call_ptr(c, t, 0x0016_0874)?;
            c.lq(2, cam634_sp(c, 0x80));
            c.sq(2, cam634_sp(c, 0x70));
            c.lwc1(0, cam634_sp(c, 0x70));
            c.abs_s(0, 0);
            c.c_lt_s(0, 20);
            let t = !c.fcc();
            c.lwc1(0, cam634_sp(c, 0x74));
            if t {
                break 'b_1608b4;
            }
            c.cycles += 5;
            c.abs_s(0, 0);
            c.c_lt_s(0, 20);
            let t = !c.fcc();
            c.g[V0][0] = 0x500000;
            if t {
                break 'b_1608b4;
            }
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xec0));
            c.lq(3, c.g[V0][0] as u32);
            c.sq(3, cam634_sp(c, 0x70));
            break 'b_1608b4;
        }

        // The slew and the eye vector (VU0 normalises the target offset).
        c.cycles += 58;
        c.sw(cam634_sp(c, 0x78), 0u64);
        c.g[S1][0] = 0x500000;
        c.lqc2(4, (c.g[S1][0] as u32).wrapping_sub(0xea0));
        c.lqc2(5, cam634_sp(c, 0x70));
        c.vu0().opmula(4, 5);
        c.vu0().opmsub(6, 5, 4);
        c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
        c.sqc2(6, cam634_sp(c, 0x90));
        c.lwc1(4, cam634_gp(c, 0x6bc0));
        c.lq(2, cam634_sp(c, 0x90));
        c.lwc1(2, cam634_gp(c, 0x6bbc));
        c.sq(2, cam634_sp(c, 0x80));
        c.lqc2(3, cam634_sp(c, 0x70));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(3, 22);
        c.mtc1(3, 1);
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 3);
        c.mfc1(2, 1);
        c.qmtc2(2, 3);
        c.vu0().div(0, 3, 3, 0);
        c.lqc2(4, cam634_sp(c, 0x70));
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 4, Src::Q);
        c.sqc2(5, cam634_sp(c, 0x70));
        c.lqc2(3, cam634_sp(c, 0x80));
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
        c.lqc2(4, cam634_sp(c, 0x80));
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 4, Src::Q);
        c.sqc2(5, cam634_sp(c, 0x80));
        c.lwc1(1, cam634_reg(c, S0, 0x390));
        c.add_s(1, 1, 4);
        c.mul_s(2, 1, 2);
        c.swc1(1, cam634_reg(c, S0, 0x390));
        c.add_s(2, 2, 3);
        c.cvt_w_s(3, 2);
        c.cvt_s_w(3, 3);
        c.c_lt_s(2, 3);
        let skip = !c.fcc();
        if !skip {
            c.cycles += 3;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.sub_s(3, 3, 0);
        }

        c.cycles += 80;
        c.lwc1(0, cam634_gp(c, 0x6bb8));
        c.g[A0][0] = sx(cam634_sp(c, 0xa0));
        c.g[A2][0] = c.lw(cam634_reg(c, S0, 0x30));
        c.mul_s(0, 3, 0);
        c.lwc1(20, cam634_gp(c, 0x6bb4));
        c.sub_s(0, 1, 0);
        c.swc1(0, cam634_reg(c, S0, 0x390));
        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
        c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        let t = c.g[V1][0] as u32;
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, t, 0x0016_09d8)?;
        c.mtc1(0, 0);
        c.mfc1(3, 0);
        c.lqc2(4, (c.g[S1][0] as u32).wrapping_sub(0xea0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, cam634_sp(c, 0xc0));
        c.lq(2, cam634_sp(c, 0xc0));
        c.sq(2, cam634_sp(c, 0xb0));
        c.lqc2(3, cam634_sp(c, 0xa0));
        c.lqc2(4, cam634_sp(c, 0xb0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, cam634_sp(c, 0xc0));
        c.lq(2, cam634_sp(c, 0xc0));
        c.lwc1(12, cam634_reg(c, S0, 0x390));
        c.sq(2, cam634_reg(c, S0, 0x20));
        c.sq(2, cam634_sp(c, 0x90));
        call_ptr(c, 0x0031_c040, 0x0016_0a1c)?;
        c.mul_s(0, 0, 20);
        c.mfc1(3, 0);
        c.lqc2(4, cam634_sp(c, 0x70));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, cam634_sp(c, 0xd0));
        c.lq(2, cam634_sp(c, 0xd0));
        c.sq(2, cam634_sp(c, 0xc0));
        c.lqc2(3, cam634_reg(c, S0, 0x20));
        c.lqc2(4, cam634_sp(c, 0xc0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, cam634_sp(c, 0xd0));
        c.lq(2, cam634_sp(c, 0xd0));
        c.lwc1(12, cam634_reg(c, S0, 0x390));
        c.sq(2, cam634_sp(c, 0xb0));
        call_ptr(c, 0x0031_bf60, 0x0016_0a5c)?;
        c.mul_s(0, 0, 20);
        c.lwc1(1, cam634_gp(c, 0x6bb0));
        c.mfc1(3, 0);
        c.lqc2(4, cam634_sp(c, 0x80));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, cam634_sp(c, 0xe0));
        c.g[RA][0] = c.ld(cam634_sp(c, 0x100));
        c.lq(2, cam634_sp(c, 0xe0));
        c.lwc1(20, cam634_sp(c, 0x130));
        c.sq(2, cam634_sp(c, 0xd0));
        c.lqc2(3, cam634_sp(c, 0xb0));
        c.lqc2(4, cam634_sp(c, 0xd0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, cam634_sp(c, 0xe0));
        c.lq(2, cam634_sp(c, 0xe0));
        c.sq(2, cam634_sp(c, 0xa0));
        c.mfc1(3, 1);
        c.lqc2(4, (c.g[S1][0] as u32).wrapping_sub(0xea0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, cam634_sp(c, 0xf0));
        c.lq(2, cam634_sp(c, 0xf0));
        c.lq(17, cam634_sp(c, 0x110));
        c.sq(2, cam634_sp(c, 0xe0));
        c.lqc2(3, cam634_sp(c, 0xa0));
        c.lqc2(4, cam634_sp(c, 0xe0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, cam634_sp(c, 0xf0));
        c.lq(2, cam634_sp(c, 0xf0));
        c.sq(2, cam634_reg(c, S0, 0x40));
        c.lq(16, cam634_sp(c, 0x120));
        c.sq(2, cam634_sp(c, 0x90));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x140));
        ret(c);
        Ok(())
    });
}

/// `SplineCameraAlgorithm_ctor(self, p1, p2, p3, p4, dur, leadDistance,
/// heightOffset, fov)` (0x161060): the spline chase camera's constructor. It
/// builds the chase base, then four cubic splines (position, look-at, and
/// the two offsets; each with the vtable 0x45b968 at `self+0x10`), feeds the
/// four control points into each, and samples the look-at spline over the
/// duration (`fp` counts the samples up to 100). The slew at `self+0x3bc` is
/// set from the last sample's span. `self` is returned.
pub fn spline_camera_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 130;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x120));
        c.sq(23, cam634_sp(c, 0x60));
        c.sq(19, cam634_sp(c, 0xa0));
        c.g[S7][0] = c.g[A0][0];
        c.sq(20, cam634_sp(c, 0x90));
        c.g[S3][0] = c.g[A1][0];
        c.sq(21, cam634_sp(c, 0x80));
        c.g[S4][0] = c.g[A2][0];
        c.sq(22, cam634_sp(c, 0x70));
        c.g[S5][0] = c.g[A3][0];
        c.swc1(24, cam634_sp(c, 0x100));
        c.g[S6][0] = c.g[T0][0];
        c.swc1(23, cam634_sp(c, 0xf8));
        c.mov_s(24, 12);
        c.swc1(21, cam634_sp(c, 0xe8));
        c.mov_s(23, 15);
        c.swc1(20, cam634_sp(c, 0xe0));
        c.mov_s(21, 14);
        c.sq(16, cam634_sp(c, 0xd0));
        c.mov_s(20, 13);
        c.sq(17, cam634_sp(c, 0xc0));
        c.sq(18, cam634_sp(c, 0xb0));
        c.g[S0][0] = sx(cam634_reg(c, S7, 0x390));
        c.sq(30, cam634_sp(c, 0x50));
        c.g[S1][0] = sx(cam634_reg(c, S7, 0x39c));
        c.swc1(26, cam634_sp(c, 0x110));
        c.g[S2][0] = sx(cam634_reg(c, S7, 0x3a8));
        c.swc1(25, cam634_sp(c, 0x108));
        c.swc1(22, cam634_sp(c, 0xf0));
        c.sd(cam634_sp(c, 0x40), c.g[RA][0]);
        c.g[FP][0] = 0x1;
        Callee {
            addr: 0x0016_2318,
            f: ee::s01::f_00162318,
        }
        .call(c, 0x0016_10e0)?;
        c.mtc1(0, 22);
        c.g[V0][0] = 0x460000;
        c.g[A0][0] = c.g[S0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4698));
        c.mov_s(25, 22);
        c.sw(cam634_reg(c, S7, 0x10), c.g[V0][0]);
        call_ptr(c, 0x0031_d5e8, 0x0016_10fc)?;
        c.lwc1(26, cam634_gp(c, 0x6b70));
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0031_d5e8, 0x0016_1108)?;
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0031_d5e8, 0x0016_1110)?;
        c.g[V0][0] = 0x4b;
        c.swc1(20, cam634_reg(c, S7, 0x3c0));
        c.swc1(21, cam634_reg(c, S7, 0x3c4));
        c.g[A0][0] = c.g[S0][0];
        c.swc1(23, c.g[S7][0] as u32);
        c.g[A1][0] = 0x4;
        c.sw(cam634_reg(c, S7, 0xc), c.g[V0][0]);
        call_ptr(c, 0x0031_d700, 0x0016_1130)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0x4;
        call_ptr(c, 0x0031_d700, 0x0016_113c)?;
        c.g[A1][0] = 0x4;
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0031_d700, 0x0016_1148)?;
        c.lwc1(20, cam634_gp(c, 0x6b6c));
        c.g[A0][0] = c.g[S0][0];
        c.lwc1(21, cam634_gp(c, 0x6b68));
        c.mov_s(12, 22);
        c.lwc1(13, c.g[S3][0] as u32);
        c.mul_s(20, 24, 20);
        c.mul_s(21, 24, 21);
        call_ptr(c, 0x0031_d7e0, 0x0016_1168)?;
        c.lwc1(13, c.g[S4][0] as u32);
        c.mov_s(12, 20);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0031_d7e0, 0x0016_1178)?;
        c.lwc1(13, c.g[S5][0] as u32);
        c.mov_s(12, 21);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0031_d7e0, 0x0016_1188)?;
        c.lwc1(13, c.g[S6][0] as u32);
        c.g[A0][0] = c.g[S0][0];
        c.mov_s(12, 24);
        call_ptr(c, 0x0031_d7e0, 0x0016_1198)?;
        c.lwc1(13, (c.g[S3][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = c.g[S1][0];
        c.mov_s(12, 22);
        call_ptr(c, 0x0031_d7e0, 0x0016_11a8)?;
        c.lwc1(13, (c.g[S4][0] as u32).wrapping_add(0x4));
        c.mov_s(12, 20);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0031_d7e0, 0x0016_11b8)?;
        c.lwc1(13, (c.g[S5][0] as u32).wrapping_add(0x4));
        c.mov_s(12, 21);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0031_d7e0, 0x0016_11c8)?;
        c.lwc1(13, (c.g[S6][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = c.g[S1][0];
        c.mov_s(12, 24);
        call_ptr(c, 0x0031_d7e0, 0x0016_11d8)?;
        c.lwc1(13, (c.g[S3][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.g[S2][0];
        c.mov_s(12, 22);
        call_ptr(c, 0x0031_d7e0, 0x0016_11e8)?;
        c.lwc1(13, (c.g[S4][0] as u32).wrapping_add(0x8));
        c.mov_s(12, 20);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0031_d7e0, 0x0016_11f8)?;
        c.lwc1(13, (c.g[S5][0] as u32).wrapping_add(0x8));
        c.mov_s(12, 21);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0031_d7e0, 0x0016_1208)?;
        c.lwc1(13, (c.g[S6][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = c.g[S2][0];
        c.mov_s(12, 24);
        call_ptr(c, 0x0031_d7e0, 0x0016_1218)?;
        c.swc1(24, cam634_reg(c, S7, 0x3b4));
        c.g[A0][0] = c.g[S0][0];
        c.swc1(22, cam634_reg(c, S7, 0x3b8));
        c.mov_s(12, 25);
        call_ptr(c, 0x0031_dee0, 0x0016_122c)?;
        c.mov_s(21, 0);
        c.g[A0][0] = c.g[S1][0];
        c.mov_s(12, 25);
        call_ptr(c, 0x0031_dee0, 0x0016_123c)?;
        c.mov_s(20, 0);
        c.g[A0][0] = c.g[S2][0];
        c.mov_s(12, 25);
        call_ptr(c, 0x0031_dee0, 0x0016_124c)?;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 1);
        c.swc1(21, cam634_sp(c, 0));
        c.swc1(20, cam634_sp(c, 0x4));
        c.mov_s(23, 1);
        c.swc1(0, cam634_sp(c, 0x8));
        c.swc1(1, cam634_sp(c, 0xc));
        'l_161268: loop {
            c.cycles += 42;
            c.mtc1(30, 20);
            c.cvt_s_w(20, 20);
            c.g[A0][0] = sx(cam634_reg(c, S7, 0x390));
            c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x1));
            c.mul_s(20, 24, 20);
            c.mul_s(20, 20, 26);
            c.min_s(20, 20, 24);
            c.mov_s(12, 20);
            call_ptr(c, 0x0031_dee0, 0x0016_128c)?;
            c.mov_s(22, 0);
            c.g[A0][0] = sx(cam634_reg(c, S7, 0x39c));
            c.mov_s(12, 20);
            call_ptr(c, 0x0031_dee0, 0x0016_129c)?;
            c.mov_s(21, 0);
            c.g[A0][0] = sx(cam634_reg(c, S7, 0x3a8));
            c.mov_s(12, 20);
            call_ptr(c, 0x0031_dee0, 0x0016_12ac)?;
            c.swc1(22, cam634_sp(c, 0x10));
            c.g[A1][0] = ((c.g[FP][0] as i64) < 101i64) as u64;
            c.swc1(21, cam634_sp(c, 0x14));
            c.swc1(0, cam634_sp(c, 0x18));
            c.swc1(23, cam634_sp(c, 0x1c));
            c.lqc2(3, cam634_sp(c, 0x10));
            c.lqc2(4, sp(c));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, cam634_sp(c, 0x30));
            c.lq(2, cam634_sp(c, 0x30));
            c.lq(3, cam634_sp(c, 0x10));
            c.sq(2, cam634_sp(c, 0x20));
            c.lqc2(3, cam634_sp(c, 0x20));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(4, 22);
            c.mtc1(4, 0);
            c.add_s(25, 25, 0);
            let more = c.g[A1][0] != 0u64;
            c.sq(3, sp(c));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0016_1268);
                }
                continue 'l_161268;
            }
            c.cycles += 24;
            c.lwc1(0, cam634_gp(c, 0x6b64));
            c.g[V0][0] = c.g[S7][0];
            c.lq(16, cam634_sp(c, 0xd0));
            c.mul_s(0, 25, 0);
            c.lq(17, cam634_sp(c, 0xc0));
            c.lq(18, cam634_sp(c, 0xb0));
            c.lq(19, cam634_sp(c, 0xa0));
            c.div_s(0, 0, 24);
            c.lq(20, cam634_sp(c, 0x90));
            c.lq(21, cam634_sp(c, 0x80));
            c.lq(22, cam634_sp(c, 0x70));
            c.lq(30, cam634_sp(c, 0x50));
            c.g[RA][0] = c.ld(cam634_sp(c, 0x40));
            c.lwc1(26, cam634_sp(c, 0x110));
            c.swc1(0, cam634_reg(c, S7, 0x3bc));
            c.lq(23, cam634_sp(c, 0x60));
            c.lwc1(25, cam634_sp(c, 0x108));
            c.lwc1(24, cam634_sp(c, 0x100));
            c.lwc1(23, cam634_sp(c, 0xf8));
            c.lwc1(22, cam634_sp(c, 0xf0));
            c.lwc1(21, cam634_sp(c, 0xe8));
            c.lwc1(20, cam634_sp(c, 0xe0));
            c.g[SP][0] = sx(sp(c).wrapping_add(0x120));
            ret(c);
            return Ok(());
        }
    });
}

/// `SplineCameraAlgorithm_update(self)` (0x1613f0): the per-frame update of the
/// spline chase camera. It advances the sample time at `self+0x3b8` toward the
/// end of the path (clamped to `self+0x3b4`, at a rate from `self+0x3bc`),
/// samples the three splines at that time (the position at `self+0x390`, the
/// look-at at `+0x39c`, the offset at `+0x3a8`), and on the moving path
/// eases the time toward the target. Then it moves the eye vector at
/// `self+0x20` by the owner's offset (`self+0x30`, a virtual method) and the
/// heading.
pub fn spline_camera_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 62;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0xf0));
        c.lwc1(3, cam634_gp(c, 0x6b60));
        c.sq(16, cam634_sp(c, 0xb0));
        c.sq(17, cam634_sp(c, 0xa0));
        c.g[S0][0] = c.g[A0][0];
        c.sq(18, cam634_sp(c, 0x90));
        c.g[S1][0] = sx(cam634_reg(c, S0, 0x390));
        c.sq(19, cam634_sp(c, 0x80));
        c.g[A0][0] = c.g[S1][0];
        c.swc1(24, cam634_sp(c, 0xe0));
        c.g[S2][0] = sx(cam634_reg(c, S0, 0x39c));
        c.swc1(23, cam634_sp(c, 0xd8));
        c.g[S3][0] = sx(cam634_reg(c, S0, 0x3a8));
        c.swc1(22, cam634_sp(c, 0xd0));
        c.swc1(21, cam634_sp(c, 0xc8));
        c.swc1(20, cam634_sp(c, 0xc0));
        c.sd(cam634_sp(c, 0x70), c.g[RA][0]);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 24);
        c.lwc1(1, cam634_reg(c, S0, 0x3b8));
        c.lwc1(2, cam634_reg(c, S0, 0x3b4));
        c.add_s(22, 1, 3);
        c.lq(2, cam634_reg(c, S0, 0x40));
        c.lwc1(0, cam634_reg(c, S0, 0x3bc));
        c.sq(2, cam634_sp(c, 0));
        c.min_s(22, 22, 2);
        c.mul_s(23, 0, 3);
        c.mov_s(12, 22);
        call_ptr(c, 0x0031_dee0, 0x0016_1464)?;
        c.mov_s(21, 0);
        c.g[A0][0] = c.g[S2][0];
        c.mov_s(12, 22);
        call_ptr(c, 0x0031_dee0, 0x0016_1474)?;
        c.mov_s(20, 0);
        c.g[A0][0] = c.g[S3][0];
        c.mov_s(12, 22);
        call_ptr(c, 0x0031_dee0, 0x0016_1484)?;
        c.swc1(0, cam634_sp(c, 0x18));
        c.swc1(21, cam634_sp(c, 0x10));
        c.swc1(20, cam634_sp(c, 0x14));
        c.swc1(24, cam634_sp(c, 0x1c));
        c.lqc2(3, cam634_sp(c, 0x10));
        c.lqc2(4, sp(c));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, cam634_sp(c, 0x30));
        c.mtc1(0, 0);
        c.lq(2, cam634_sp(c, 0x30));
        c.sq(2, cam634_sp(c, 0x20));
        c.lqc2(3, cam634_sp(c, 0x20));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(3, 22);
        c.mtc1(3, 2);
        c.c_lt_s(0, 2);
        let moving = !c.fcc();
        c.g[A0][0] = c.g[S1][0];
        if !moving {
            c.cycles += 24;
            c.div_s(2, 2, 23);
            c.lwc1(20, cam634_reg(c, S0, 0x3b8));
            c.lwc1(1, cam634_reg(c, S0, 0x3b4));
            c.sub_s(0, 22, 20);
            c.div_s(0, 0, 2);
            c.add_s(20, 20, 0);
            c.min_s(20, 20, 1);
            c.swc1(20, cam634_reg(c, S0, 0x3b8));
            c.mov_s(12, 20);
            call_ptr(c, 0x0031_dee0, 0x0016_1510)?;
            c.mov_s(22, 0);
            c.g[A0][0] = c.g[S2][0];
            c.mov_s(12, 20);
            call_ptr(c, 0x0031_dee0, 0x0016_1520)?;
            c.mov_s(21, 0);
            c.g[A0][0] = c.g[S3][0];
            c.mov_s(12, 20);
            call_ptr(c, 0x0031_dee0, 0x0016_1530)?;
            c.swc1(22, cam634_sp(c, 0x30));
            c.swc1(21, cam634_sp(c, 0x34));
            c.swc1(0, cam634_sp(c, 0x38));
            c.swc1(24, cam634_sp(c, 0x3c));
            c.lq(2, cam634_sp(c, 0x30));
            c.sq(2, cam634_reg(c, S0, 0x40));
        }

        c.cycles += 57;
        c.g[A2][0] = c.lw(cam634_reg(c, S0, 0x30));
        c.g[A0][0] = sx(cam634_sp(c, 0x20));
        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
        c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        let t = c.g[V1][0] as u32;
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, t, 0x0016_1564)?;
        c.g[V0][0] = 0x500000;
        c.lwc1(0, cam634_reg(c, S0, 0x3c4));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xea0));
        c.g[A0][0] = sx(cam634_sp(c, 0x50));
        c.mfc1(3, 0);
        c.lqc2(4, c.g[V0][0] as u32);
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, cam634_sp(c, 0x40));
        c.lq(2, cam634_sp(c, 0x40));
        c.sq(2, cam634_sp(c, 0x30));
        c.lqc2(3, cam634_sp(c, 0x20));
        c.lqc2(4, cam634_sp(c, 0x30));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, cam634_sp(c, 0x40));
        c.lq(2, cam634_sp(c, 0x40));
        c.g[A2][0] = c.lw(cam634_reg(c, S0, 0x30));
        c.sq(2, cam634_sp(c, 0x10));
        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
        c.lwc1(20, cam634_reg(c, S0, 0x3c0));
        c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
        let t = c.g[V1][0] as u32;
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, t, 0x0016_15c4)?;
        c.mfc1(4, 20);
        c.lqc2(4, cam634_sp(c, 0x50));
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, cam634_sp(c, 0x60));
        c.lq(17, cam634_sp(c, 0xa0));
        c.lq(2, cam634_sp(c, 0x60));
        c.lq(18, cam634_sp(c, 0x90));
        c.sq(2, cam634_sp(c, 0x40));
        c.lqc2(3, cam634_sp(c, 0x10));
        c.lqc2(4, cam634_sp(c, 0x40));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, cam634_sp(c, 0x60));
        c.lq(19, cam634_sp(c, 0x80));
        c.lq(2, cam634_sp(c, 0x60));
        c.g[RA][0] = c.ld(cam634_sp(c, 0x70));
        c.sq(2, cam634_reg(c, S0, 0x20));
        c.lq(16, cam634_sp(c, 0xb0));
        c.lwc1(24, cam634_sp(c, 0xe0));
        c.lwc1(23, cam634_sp(c, 0xd8));
        c.lwc1(22, cam634_sp(c, 0xd0));
        c.lwc1(21, cam634_sp(c, 0xc8));
        c.lwc1(20, cam634_sp(c, 0xc0));
        c.sq(2, sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0xf0));
        ret(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE};
    use crate::difftest::{check, fill, Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word at `CTRL` in the machine's RAM: the callee fakes act only
    /// when it is set (other units' tests run the real callees).
    const CTRL: u32 = DATA + 0x7_0000;
    const MAGIC: u32 = 0x0634_0c0d;

    /// A gated fake for a callee with an override slot: logs the call with
    /// the usual fake clobbers, or runs the body in `PREV` with the gate off.
    macro_rules! gated {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(CTRL) != MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                fakes::fake_call(c, $addr);
            }
        };
    }

    gated!(
        oc634_fake_gather_rider,
        OC634_PREV_GATHER,
        0x0016_2568,
        ee::s01::f_00162568__generated
    );
    gated!(
        oc634_fake_chase_ctor,
        OC634_PREV_CHASE_CTOR,
        0x0016_2318,
        ee::s01::f_00162318__generated
    );
    gated!(
        oc634_fake_spline_ctor,
        OC634_PREV_SPLINE_CTOR,
        0x0031_d5e8,
        ee::s10::f_0031d5e8__generated
    );
    gated!(
        oc634_fake_spline_init,
        OC634_PREV_SPLINE_INIT,
        0x0031_d700,
        ee::s10::f_0031d700__generated
    );
    gated!(
        oc634_fake_spline_add,
        OC634_PREV_SPLINE_ADD,
        0x0031_d7e0,
        ee::s10::f_0031d7e0__generated
    );
    gated!(
        oc634_fake_spline_eval,
        OC634_PREV_SPLINE_EVAL,
        0x0031_dee0,
        ee::s10::f_0031dee0__generated
    );

    /// Hooks the gated fakes into the callees' override slots (once per process).
    fn install() {
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00162568),
            oc634_fake_gather_rider,
            &OC634_PREV_GATHER,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00162318),
            oc634_fake_chase_ctor,
            &OC634_PREV_CHASE_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_0031D5E8),
            oc634_fake_spline_ctor,
            &OC634_PREV_SPLINE_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_0031D700),
            oc634_fake_spline_init,
            &OC634_PREV_SPLINE_INIT,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_0031D7E0),
            oc634_fake_spline_add,
            &OC634_PREV_SPLINE_ADD,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_0031DEE0),
            oc634_fake_spline_eval,
            &OC634_PREV_SPLINE_EVAL,
        );
    }

    /// The float constants at `gp - 0x6c00..0x6bf0` the camera code loads.
    fn float_consts(c: &mut Cpu, r: &mut Rng) {
        for k in (0x6b60u32..0x6bf0).step_by(4) {
            c.bus.write32(GAME_GP.wrapping_sub(k), r.f32(4.0));
        }
    }

    /// The owner at `this+0x30`: an object at `DATA + 0x2000` whose vtable
    /// (`DATA + 0x3000`) holds a `{delta, fake}` slot at each offset in `offs`.
    fn owner(c: &mut Cpu, r: &mut Rng, this: u32, offs: &[u32]) {
        let obj = DATA + 0x2000;
        let vt = DATA + 0x3000;
        c.bus.write32(this + 0x30, obj);
        c.bus.write32(obj, vt);
        for &off in offs {
            c.bus.write16(vt + off, (r.next_u32() as u16) & 0xfffc);
            c.bus.write32(vt + off + 4, FAKE);
        }
    }

    #[test]
    fn matrix_chase_camera_update_matches() {
        install();
        check(ee::s01::f_001607e0__generated, matrix_chase_camera_update, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x1607e0, 0);
            c.bus.write32(CTRL, MAGIC);
            float_consts(c, &mut r);
            let this = DATA;
            c.g[A0][0] = this as u64;
            fill(c, &mut r, this, 0x100);
            fill(c, &mut r, DATA + 0x2000, 0x10);
            owner(c, &mut r, this, &[0x20]);
            c.bus.write32(this + 0x390, r.f32(2.0));
            c.bus.write32(this + 0x20, r.f32(2.0));
        });
    }

    #[test]
    fn spline_camera_ctor_matches() {
        install();
        check(ee::s01::f_00161060__generated, spline_camera_ctor, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x161060, 0);
            c.bus.write32(CTRL, MAGIC);
            float_consts(c, &mut r);
            let this = DATA;
            let points = DATA + 0x8000;
            fill(c, &mut r, points, 0x10);
            c.g[A0][0] = this as u64;
            c.g[A1][0] = points as u64;
            c.g[A2][0] = (points + 0x10) as u64;
            c.g[A3][0] = (points + 0x20) as u64;
            c.g[T0][0] = (points + 0x30) as u64;
            c.f[12] = r.f32(3.0);
            c.f[13] = r.f32(3.0);
            c.f[14] = r.f32(3.0);
            c.f[15] = r.f32(3.0);
        });
    }

    #[test]
    fn spline_camera_update_matches() {
        install();
        check(ee::s01::f_001613f0__generated, spline_camera_update, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x1613f0, 0);
            c.bus.write32(CTRL, MAGIC);
            float_consts(c, &mut r);
            let this = DATA;
            c.g[A0][0] = this as u64;
            fill(c, &mut r, this, 0x100);
            fill(c, &mut r, DATA + 0x2000, 0x10);
            owner(c, &mut r, this, &[0x8, 0x20]);
            c.bus.write32(this + 0x3b4, r.f32(2.0));
            c.bus.write32(this + 0x3b8, r.f32(2.0));
            c.bus.write32(this + 0x3bc, r.f32(2.0));
        });
    }
}
