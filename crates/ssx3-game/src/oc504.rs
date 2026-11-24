//! Staging module oc504 (wave wave228): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc504`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0017_85f0,
        superpipe_chase_camera_compute_desired_eye,
        "SuperpipeChaseCameraAlgorithm_ComputeDesiredEye",
    ),
    (
        0x0017_8758,
        superpipe_chase_camera_compute_look_at,
        "SuperpipeChaseCameraAlgorithm_ComputeLookAt",
    ),
    (
        0x0017_87e0,
        superpipe_chase_camera_update,
        "SuperpipeChaseCameraAlgorithm_Update",
    ),
    (
        0x0017_89e8,
        post_race_1_chase_camera_ctor,
        "Post_Race_1ChaseCameraAlgorithm_ctor",
    ),
    (
        0x0017_8b70,
        post_race_1_chase_camera_dtor,
        "Post_Race_1ChaseCameraAlgorithm_dtor",
    ),
    (
        0x0017_9738,
        post_race_1_chase_camera_global_ctor,
        "Post_Race_1ChaseCameraAlgorithm_GlobalCtor",
    ),
    (
        0x0017_9fa8,
        camera_consts_global_ctor_9fa8,
        "CameraConsts_GlobalCtor_9fa8",
    ),
];

/// The address `gp - off` (the generated code's `lwc1 f, -off(gp)`).
fn gp_minus(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(off)
}

/// `SuperpipeChaseCameraAlgorithm_ComputeDesiredEye` (0x1785f0): the ideal eye
/// point. The chase line runs from `Point1` (+0x390) to `Point2` (+0x3a0); the
/// eye sits on it at the projection of the target's position (`GetPos`, the
/// vtable slot at self+0x30), pulled back 1500 units along the line. The
/// `t4Vector` result goes to the hidden return buffer in a0; self is in a1.
pub fn superpipe_chase_camera_compute_desired_eye(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 90;
        let frame = Frame::open(c, 0xa0);
        let at = sp(c);
        frame.save(c, 16, 0x90);
        frame.save(c, 17, 0x80);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x70);
        c.g[S1][0] = c.g[A0][0];
        let this = r32(c, S0);
        // dir = normalize(Point1 - Point2)
        c.lqc2(3, this.wrapping_add(0x390));
        c.lqc2(4, this.wrapping_add(0x3a0));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at.wrapping_add(0x20));
        c.g[A0][0] = sx(at.wrapping_add(0x20));
        c.lq(2, at.wrapping_add(0x20));
        c.sq(2, at.wrapping_add(0x10));
        c.lqc2(3, at.wrapping_add(0x10));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().rsqrt(0, 3, 4, 0);
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 3, Src::Q);
        c.sqc2(5, at.wrapping_add(0x20));
        c.lq(2, at.wrapping_add(0x20));
        // GetPos(self): the target's position, through the vtable at self+0x30.
        let obj = c.lw(this.wrapping_add(0x30));
        c.g[A2][0] = obj;
        c.sq(2, at);
        c.g[V1][0] = c.lw(obj as u32);
        c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        let target = c.g[V0][0] as u32;
        c.g[A1][0] = sx((obj as u32).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, target, 0x0017_866c)?;
        // rel = GetPos - Point1
        c.lqc2(3, at.wrapping_add(0x20));
        c.lqc2(4, this.wrapping_add(0x390));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at.wrapping_add(0x30));
        c.lwc1(2, gp_minus(c, 0x5dac));
        c.lq(3, at.wrapping_add(0x30));
        c.g[V0][0] = c.g[S1][0];
        c.sq(3, at.wrapping_add(0x10));
        // t = dot(rel, dir) / dot(dir, dir)
        c.lqc2(3, at);
        c.lqc2(5, at);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(3, 4);
        c.mtc1(3, 1);
        c.lqc2(3, at.wrapping_add(0x10));
        c.lqc2(5, at);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(4, 4);
        c.mtc1(4, 0);
        c.div_s(0, 0, 1);
        c.mfc1(4, 0);
        c.lqc2(4, at);
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at.wrapping_add(0x30));
        c.lq(3, at.wrapping_add(0x30));
        c.sq(3, at.wrapping_add(0x20));
        // off = Point1 + dir * t
        c.lqc2(3, this.wrapping_add(0x390));
        c.lqc2(4, at.wrapping_add(0x20));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at.wrapping_add(0x50));
        c.lq(3, at.wrapping_add(0x50));
        c.sq(3, at.wrapping_add(0x40));
        // eye = off - dir * 1500
        c.mfc1(4, 2);
        c.lqc2(4, at);
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at.wrapping_add(0x60));
        c.lq(3, at.wrapping_add(0x60));
        c.sq(3, at.wrapping_add(0x50));
        c.lqc2(3, at.wrapping_add(0x40));
        c.lqc2(4, at.wrapping_add(0x50));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at.wrapping_add(0x60));
        c.lq(3, at.wrapping_add(0x60));
        c.lq(16, at.wrapping_add(0x90));
        let s1 = r32(c, S1);
        c.sq(3, s1);
        c.lq(17, at.wrapping_add(0x80));
        c.g[RA][0] = c.ld(at.wrapping_add(0x70));
        c.sq(3, at.wrapping_add(0x30));
        frame.close(c);
        Ok(())
    })
}

/// `SuperpipeChaseCameraAlgorithm_ComputeLookAt` (0x178758): the look-at point,
/// 100 units above the target's position (`GetPos`, vtable slot at self+0x30),
/// with the hidden return buffer in a0 and self in a1.
pub fn superpipe_chase_camera_compute_look_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x50);
        let at = sp(c);
        frame.save(c, 16, 0x40);
        frame.save_ra(c, 0x30);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.g[SP][0];
        let obj = c.lw(r32(c, A1).wrapping_add(0x30));
        c.g[A2][0] = obj;
        c.g[V0][0] = c.lw(obj as u32);
        c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        let target = c.g[V1][0] as u32;
        c.g[A1][0] = sx((obj as u32).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, target, 0x0017_8784)?;
        // The up-vector scaled by -100.0 (the constant at 0x4ff160), plus the position.
        c.g[V1][0] = 0x0050_0000;
        c.g[AT][0] = 0xffff_ffff_c2c8_0000;
        c.mtc1(1, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xea0));
        c.g[V0][0] = c.g[S0][0];
        c.mfc1(4, 0);
        c.lqc2(4, c.g[V1][0] as u32);
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at.wrapping_add(0x20));
        frame.restore_ra(c, 0x30);
        c.lq(3, at.wrapping_add(0x20));
        c.sq(3, at.wrapping_add(0x10));
        c.lqc2(3, at);
        c.lqc2(4, at.wrapping_add(0x10));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at.wrapping_add(0x20));
        c.lq(3, at.wrapping_add(0x20));
        let s0 = r32(c, S0);
        c.sq(3, s0);
        frame.restore(c, 16, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `SuperpipeChaseCameraAlgorithm_Update` (0x1787e0): one frame of the
/// superpipe chase camera. Runs the base chase update, then computes the eye
/// (`ComputeDesiredEye`) and the look-at point (`ComputeLookAt`), stores both,
/// and pulls the eye toward the target when the eye-to-look distance is out of
/// range (above 600 units it is clamped; below 1 unit it is pinned).
pub fn superpipe_chase_camera_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 50;
        let frame = Frame::open(c, 0xd0);
        let at = sp(c);
        c.lwc1(12, gp_minus(c, 0x5da8));
        frame.save(c, 17, 0xb0);
        c.g[A1][0] = c.g[SP][0];
        c.lwc1(13, gp_minus(c, 0x5da4));
        c.lwc1(14, gp_minus(c, 0x5da0));
        c.lwc1(15, gp_minus(c, 0x5d9c));
        c.lwc1(16, gp_minus(c, 0x5d98));
        c.lwc1(17, gp_minus(c, 0x5d94));
        c.lwc1(18, gp_minus(c, 0x5d90));
        c.lwc1(19, gp_minus(c, 0x5d8c));
        frame.save(c, 16, 0xc0);
        frame.save_ra(c, 0xa0);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, 0x0016_2568, 0x0017_881c)?;
        c.lwc1(0, gp_minus(c, 0x5d88));
        c.g[S0][0] = sx(at.wrapping_add(0x70));
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.swc1(0, r32(c, S1));
        Callee {
            addr: 0x0017_85f0,
            f: ee::s02::f_001785f0,
        }
        .call(c, 0x0017_8834)?;
        c.lq(2, at.wrapping_add(0x70));
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.sq(2, r32(c, S1).wrapping_add(0x40));
        Callee {
            addr: 0x0017_8758,
            f: ee::s02::f_00178758,
        }
        .call(c, 0x0017_8848)?;
        c.lq(2, at.wrapping_add(0x70));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.sq(2, r32(c, S1).wrapping_add(0x20));
        c.lqc2(3, r32(c, S1).wrapping_add(0x20));
        c.lqc2(4, r32(c, S1).wrapping_add(0x40));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at.wrapping_add(0x80));
        c.lq(2, at.wrapping_add(0x80));
        c.sq(2, at.wrapping_add(0x70));
        c.lqc2(3, at.wrapping_add(0x70));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(3, 22);
        c.mtc1(3, 1);
        // The distance |eye - look| against 1.0.
        'b_178924: {
            'b_17891c: {
                'b_1788d4: {
                    c.c_lt_s(1, 0);
                    let t = !c.fcc();
                    c.g[V0][0] = 0x0050_0000;
                    if t {
                        break 'b_1788d4;
                    }
                    // Distance below 1: pin the eye to a scaled up-vector.
                    c.cycles += 11;
                    c.g[AT][0] = 0x4416_0000;
                    c.mtc1(1, 0);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xec0));
                    c.mfc1(3, 0);
                    c.lqc2(4, c.g[V0][0] as u32);
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, at.wrapping_add(0x90));
                    c.lq(2, at.wrapping_add(0x90));
                    c.sq(2, at.wrapping_add(0x70));
                    break 'b_17891c;
                }
                // Distance at least 1: clamp the pull to 600 units.
                c.cycles += 6;
                c.g[AT][0] = 0x4416_0000;
                c.mtc1(1, 0);
                c.c_lt_s(0, 1);
                let t = !c.fcc();
                c.lq(16, at.wrapping_add(0xc0));
                if t {
                    break 'b_178924;
                }
                c.cycles += 12;
                c.div_s(0, 0, 1);
                c.mfc1(2, 0);
                c.lqc2(4, at.wrapping_add(0x70));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at.wrapping_add(0x70));
                c.lqc2(3, r32(c, S1).wrapping_add(0x20));
                c.lqc2(4, at.wrapping_add(0x70));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at.wrapping_add(0x90));
                c.lq(2, at.wrapping_add(0x90));
                c.sq(2, r32(c, S1).wrapping_add(0x40));
                break 'b_17891c;
            }
            c.cycles += 2;
            c.sq(2, at.wrapping_add(0x80));
            c.lq(16, at.wrapping_add(0xc0));
            break 'b_178924;
        }
        c.cycles += 4;
        frame.restore(c, 17, 0xb0);
        frame.restore_ra(c, 0xa0);
        frame.close(c);
        Ok(())
    })
}

/// `Post_Race_1ChaseCameraAlgorithm_ctor` (0x1789e8): the post-race chase
/// camera's constructor. Runs the base chase constructor, sets the vtable and
/// the initial fields, then normalises the rider's direction into `+0x390`
/// (or, when the vector is degenerate, the fallback at `+0x398`).
pub fn post_race_1_chase_camera_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x80);
        let at = sp(c);
        frame.save(c, 16, 0x70);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x60);
        frame.save(c, 18, 0x50);
        frame.save_ra(c, 0x40);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x0016_2318, 0x0017_8a08)?;
        let this = r32(c, S0);
        c.g[S2][0] = sx(this.wrapping_add(0x390));
        c.g[V1][0] = 0x0046_0000;
        c.g[V0][0] = 0x44;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3428));
        c.sw(this.wrapping_add(0xc), c.g[V0][0]);
        c.sw(this.wrapping_add(0x10), c.g[V1][0]);
        c.g[A0][0] = sx(at.wrapping_add(0x10));
        c.sw(this.wrapping_add(0x3a0), 0);
        // First virtual call on the rider object (slot +0x18): the direction.
        c.g[V0][0] = c.lw(r32(c, S1));
        c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
        let target = c.g[V1][0] as u32;
        c.g[A1][0] = sx(r32(c, S1).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, target, 0x0017_8a3c)?;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 1);
        c.lqc2(3, at.wrapping_add(0x10));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(2, 22);
        c.mtc1(2, 0);
        c.c_lt_s(1, 0);
        let t = !c.fcc();
        c.g[A0][0] = sx(at.wrapping_add(0x20));
        'b_178adc: {
            'b_178ac4: {
                if t {
                    break 'b_178ac4;
                }
                c.cycles += 18;
                c.g[V0][0] = c.lw(r32(c, S1));
                c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                let target = c.g[V1][0] as u32;
                c.g[A1][0] = sx(r32(c, S1).wrapping_add(c.g[A1][0] as u32));
                call_ptr(c, target, 0x0017_8a90)?;
                c.lqc2(3, at.wrapping_add(0x20));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().rsqrt(0, 3, 4, 0);
                c.vu0().waitq();
                c.vu0().mul(0b1111, 5, 3, Src::Q);
                c.sqc2(5, at.wrapping_add(0x30));
                c.lq(2, at.wrapping_add(0x30));
                c.sq(2, at);
                break 'b_178adc;
            }
            c.cycles += 6;
            c.g[V0][0] = c.lw(r32(c, S1));
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x20));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x24));
            let target = c.g[V1][0] as u32;
            c.g[A1][0] = sx(r32(c, S1).wrapping_add(c.g[A1][0] as u32));
            call_ptr(c, target, 0x0017_8adc)?;
        }
        'b_178b58: {
            'b_178b48: {
                c.cycles += 18;
                c.lq(2, at);
                c.lwc1(0, gp_minus(c, 0x5d80));
                c.sq(2, r32(c, S2));
                c.sw(this.wrapping_add(0x398), 0);
                c.lqc2(3, this.wrapping_add(0x390));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.vu0().sqrt(4, 0);
                c.vu0().waitq();
                c.cfc2(2, 22);
                c.mtc1(2, 1);
                c.c_lt_s(0, 1);
                let t = !c.fcc();
                c.g[V0][0] = 0x0050_0000;
                if t {
                    break 'b_178b48;
                }
                c.cycles += 9;
                c.mfc1(2, 1);
                c.qmtc2(2, 3);
                c.vu0().div(0, 3, 3, 0);
                c.lqc2(4, this.wrapping_add(0x390));
                c.vu0().waitq();
                c.vu0().mul(0b1111, 5, 4, Src::Q);
                c.sqc2(5, this.wrapping_add(0x390));
                c.g[V0][0] = c.g[S0][0];
                break 'b_178b58;
            }
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xec0));
            c.lq(3, c.g[V0][0] as u32);
            c.sq(3, this.wrapping_add(0x390));
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 6;
        frame.restore(c, 17, 0x60);
        frame.restore(c, 16, 0x70);
        frame.restore(c, 18, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `Post_Race_1ChaseCameraAlgorithm_dtor` (0x178b70): stores the post-race
/// vtable (0x45cbd8) at +0x10 and runs the base chase destructor.
pub fn post_race_1_chase_camera_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x0046_0000;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3428));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        call_ptr(c, 0x0016_2458, 0x0017_8b8c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Post_Race_1ChaseCameraAlgorithm_GlobalCtor` (0x179738): the static
/// initialiser of the post-race camera tables (`StaticInit(1, 0xffff)`).
pub fn post_race_1_chase_camera_global_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 1;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        Callee {
            addr: 0x0017_8f58,
            f: ee::s02::f_00178f58,
        }
        .call(c, 0x0017_974c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `CameraConsts_GlobalCtor_9fa8` (0x179fa8): the camera constants' static
/// initialiser (`CameraConsts_StaticInit(1, 0xffff)`).
pub fn camera_consts_global_ctor_9fa8(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 1;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xffff;
        Callee {
            addr: 0x0017_9798,
            f: ee::s02::f_00179798,
        }
        .call(c, 0x0017_9fbc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, FAKE};
    use crate::difftest::{check, put_vec, Rng, DATA, GAME_GP, SP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The gate word the fakes below check in the machine's RAM.
    const OC504_CTRL: u32 = DATA + 0x20_0000;
    const OC504_MAGIC: u32 = 0x0504_0504;

    static OC504_PREV_CHASE_CTOR: AtomicUsize = AtomicUsize::new(0);
    static OC504_PREV_CHASE_DTOR: AtomicUsize = AtomicUsize::new(0);
    static OC504_PREV_CHASE_GATHER: AtomicUsize = AtomicUsize::new(0);

    /// The base chase constructor, gated: with the gate on it only returns.
    fn oc504_fake_chase_ctor(c: &mut Cpu) {
        if c.bus.read32(OC504_CTRL) != OC504_MAGIC {
            return fakes::fall_back(c, &OC504_PREV_CHASE_CTOR, ee::s01::f_00162318__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    /// The base chase destructor, gated: with the gate on it only returns.
    fn oc504_fake_chase_dtor(c: &mut Cpu) {
        if c.bus.read32(OC504_CTRL) != OC504_MAGIC {
            return fakes::fall_back(c, &OC504_PREV_CHASE_DTOR, ee::s01::f_00162458__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    /// `ChaseCameraAlgorithm_GatherRiderState`, gated: with the gate on it only returns.
    fn oc504_fake_gather(c: &mut Cpu) {
        if c.bus.read32(OC504_CTRL) != OC504_MAGIC {
            return fakes::fall_back(c, &OC504_PREV_CHASE_GATHER, ee::s01::f_00162568__generated);
        }
        c.pc = c.g[RA][0] as u32;
    }

    fn oc504_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00162318),
            oc504_fake_chase_ctor,
            &OC504_PREV_CHASE_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00162458),
            oc504_fake_chase_dtor,
            &OC504_PREV_CHASE_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00162568),
            oc504_fake_gather,
            &OC504_PREV_CHASE_GATHER,
        );
    }

    /// A chase object at `this`: Point1 and Point2 random, the target object
    /// at +0x30 with a vtable whose slots (+0x18, +0x20, +0x8) call FAKE.
    fn chase_setup(c: &mut Cpu, r: &mut Rng, this: u32, obj: u32, vt: u32) {
        put_vec(c, this + 0x390, [r.f32(900.0), r.f32(900.0), r.f32(900.0), 0x3f80_0000]);
        put_vec(c, this + 0x3a0, [r.f32(900.0), r.f32(900.0), r.f32(900.0), 0x3f80_0000]);
        c.bus.write32(this + 0x30, obj);
        c.bus.write32(obj, vt);
        for off in [0x8u32, 0x18, 0x20] {
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(vt + off, delta as u16);
            c.bus.write32(vt + off + 4, FAKE);
        }
    }

    #[test]
    fn superpipe_compute_desired_eye_matches() {
        fakes::covered(
            ee::s02::f_001785f0__generated,
            superpipe_chase_camera_compute_desired_eye,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x504_1, 1);
                c.pc = 0x0017_85f0;
                c.bus.write32(GAME_GP.wrapping_sub(0x5dac), 1500.0f32.to_bits());
                c.g[A0][0] = (DATA + 0x1000) as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                chase_setup(c, &mut r, DATA + 0x2000, DATA + 0x3000, DATA + 0x4000);
            },
        );
    }

    #[test]
    fn superpipe_compute_look_at_matches() {
        fakes::covered(
            ee::s02::f_00178758__generated,
            superpipe_chase_camera_compute_look_at,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x504_2, 1);
                c.pc = 0x0017_8758;
                c.g[A0][0] = (DATA + 0x1000) as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                chase_setup(c, &mut r, DATA + 0x2000, DATA + 0x3000, DATA + 0x4000);
            },
        );
    }

    #[test]
    fn superpipe_chase_camera_update_matches() {
        oc504_install_fakes();
        check(
            ee::s02::f_001787e0__generated,
            superpipe_chase_camera_update,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x504_3, 3);
                c.bus.write32(OC504_CTRL, OC504_MAGIC);
                c.pc = 0x0017_87e0;
                c.g[A0][0] = (DATA + 0x2000) as u64;
                for off in [0x5da8u32, 0x5da4, 0x5da0, 0x5d9c, 0x5d98, 0x5d94, 0x5d90, 0x5d8c, 0x5d88] {
                    c.bus.write32(GAME_GP.wrapping_sub(off), r.f32(2000.0));
                }
                c.bus.write32(GAME_GP.wrapping_sub(0x5dac), 1500.0f32.to_bits());
                chase_setup(c, &mut r, DATA + 0x2000, DATA + 0x3000, DATA + 0x4000);
            },
        );
    }

    #[test]
    fn post_race_1_chase_camera_ctor_matches() {
        oc504_install_fakes();
        check(
            ee::s02::f_001789e8__generated,
            post_race_1_chase_camera_ctor,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x504_4, 2);
                c.bus.write32(OC504_CTRL, OC504_MAGIC);
                c.pc = 0x0017_89e8;
                c.g[A0][0] = (DATA + 0x1000) as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                let this = DATA + 0x1000;
                put_vec(c, this + 0x390, [r.f32(4.0), r.f32(4.0), r.f32(4.0), 0x3f80_0000]);
                // The temporary the first call's result lands in (the fake leaves it alone).
                // (The frame is 0x80 below SP, so this is its +0x10 slot.)
                put_vec(c, SP - 0x70, [r.f32(4.0), r.f32(4.0), r.f32(4.0), 0]);
                // Rider object at A1 with the vtable slots +0x18 and +0x20.
                let vt = DATA + 0x4000;
                c.bus.write32(DATA + 0x2000, vt);
                for off in [0x18u32, 0x20] {
                    let delta = (r.next_u32() as i16 as i32 & !3) as u32;
                    c.bus.write16(vt + off, delta as u16);
                    c.bus.write32(vt + off + 4, FAKE);
                }
            },
        );
    }

    #[test]
    fn post_race_1_chase_camera_dtor_matches() {
        oc504_install_fakes();
        check(
            ee::s02::f_00178b70__generated,
            post_race_1_chase_camera_dtor,
            200,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x504_5, 1);
                c.bus.write32(OC504_CTRL, OC504_MAGIC);
                c.pc = 0x0017_8b70;
                c.g[A0][0] = (DATA + 0x1000) as u64;
            },
        );
    }

    #[test]
    fn post_race_1_chase_camera_global_ctor_matches() {
        check(
            ee::s02::f_00179738__generated,
            post_race_1_chase_camera_global_ctor,
            4,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x504_6, 0);
                c.pc = 0x0017_9738;
            },
        );
    }

    #[test]
    fn camera_consts_global_ctor_9fa8_matches() {
        check(
            ee::s02::f_00179fa8__generated,
            camera_consts_global_ctor_9fa8,
            4,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x504_7, 0);
                c.pc = 0x0017_9fa8;
            },
        );
    }
}
