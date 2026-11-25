//! Staging module oc635 (wave g15.txt): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc635`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0016_5540,
    chase_camera_apply_fixed_direction,
    "ChaseCameraAlgorithm_ApplyFixedDirection",
)];

/// Fixed unit vector the game keeps at `gp-0xec0` (0x4ff140), read by
/// `ChaseCameraAlgorithm_ApplyFixedDirection`.
pub(crate) const APPLY_FIXED_DIR_AXIS: u32 = 0x004f_f140;
/// Rotation data the game keeps at `gp-0xea0` (0x4ff160): the words at +0 and
/// +4 and +8 scale the sine and cosine of the half angle.
pub(crate) const APPLY_FIXED_DIR_ROT: u32 = 0x004f_f160;
/// `void fSinCos(float angle, float* sin, float* cos)` (0x31be50).
const APPLY_FIXED_DIR_SIN_COS: u32 = 0x0031_be50;
/// `cLookAtChaseAlgorithm_verify(self)` (0x168150), the tail call.
const APPLY_FIXED_DIR_VERIFY: u32 = 0x0016_8150;

/// `void ChaseCameraAlgorithm_ApplyFixedDirection(ChaseCameraAlgorithm* self,
/// float ang)` (0x165540). When the horizontal part of the eye-to-target offset
/// (`CamTargetPos - Position`, z left out) is longer than zero, the eye is put on
/// the circle around the camera target at the fixed height, turned by half of
/// `ang`; then the look-at verify runs. The eye lives at `+0x40`, the camera
/// target at `+0x20`.
pub fn chase_camera_apply_fixed_direction(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0xb0));
        c.g[V1][0] = 0x500000;
        c.sq(18, sp(c).wrapping_add(0x80));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xec0));
        c.sq(16, sp(c).wrapping_add(0xa0));
        c.g[S2][0] = c.g[A0][0];
        c.sq(17, sp(c).wrapping_add(0x90));
        c.mov_s(3, 12);
        c.sd(sp(c).wrapping_add(0x70), c.g[RA][0]);
        // offset = CamTargetPos - Position (x, y, z)
        c.lqc2(3, (c.g[S2][0] as u32).wrapping_add(0x20));
        c.lqc2(4, (c.g[S2][0] as u32).wrapping_add(0x40));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp(c).wrapping_add(0x10));
        c.lq(4, sp(c).wrapping_add(0x10));
        c.lq(2, c.g[V1][0] as u32);
        c.sq(4, sp(c));
        c.sq(2, sp(c).wrapping_add(0x10));
        c.lwc1(2, sp(c).wrapping_add(0x8));
        c.sw(sp(c).wrapping_add(0x8), 0u64);
        c.lqc2(3, sp(c));
        // horizontal length of the offset (z zeroed in the stack copy)
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(2, 22);
        c.mtc1(2, 1);
        c.lwc1(0, sp(c).wrapping_add(0x8));
        c.c_lt_s(0, 1);
        if c.fcc() {
            c.cycles += 50;
            c.g[AT][0] = 0x3f000000;
            c.mtc1(1, 12);
            c.mfc1(2, 1);
            c.lqc2(4, sp(c).wrapping_add(0x10));
            c.qmtc2(2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, sp(c).wrapping_add(0x10));
            c.swc1(2, sp(c).wrapping_add(0x18));
            c.g[S1][0] = 0x500000;
            c.mul_s(12, 3, 12);
            c.g[A0][0] = sx(sp(c).wrapping_add(0x60));
            c.g[A1][0] = sx(sp(c).wrapping_add(0x64));
            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0xea0));
            call_ptr(c, APPLY_FIXED_DIR_SIN_COS, 0x001655fc)?;
            c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x8));
            c.lwc1(0, sp(c).wrapping_add(0x60));
            c.lwc1(2, (c.g[S1][0] as u32).wrapping_sub(0xea0));
            c.lwc1(4, (c.g[S0][0] as u32).wrapping_add(0x4));
            c.mul_s(1, 0, 1);
            c.mul_s(2, 0, 2);
            c.lwc1(3, sp(c).wrapping_add(0x64));
            c.mul_s(0, 0, 4);
            c.swc1(3, sp(c).wrapping_add(0x3c));
            c.swc1(1, sp(c).wrapping_add(0x38));
            c.swc1(2, sp(c).wrapping_add(0x30));
            c.swc1(0, sp(c).wrapping_add(0x34));
            c.lq(2, sp(c).wrapping_add(0x30));
            c.sq(2, sp(c).wrapping_add(0x20));
            c.lqc2(4, sp(c).wrapping_add(0x20));
            c.lqc2(5, sp(c).wrapping_add(0x10));
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
            c.sqc2(8, sp(c).wrapping_add(0x40));
            c.lq(2, sp(c).wrapping_add(0x40));
            c.sq(2, sp(c).wrapping_add(0x30));
            // Position = CamTargetPos - rotated offset
            c.lqc2(3, (c.g[S2][0] as u32).wrapping_add(0x20));
            c.lqc2(4, sp(c).wrapping_add(0x30));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, sp(c).wrapping_add(0x50));
            c.lq(2, sp(c).wrapping_add(0x50));
            c.sq(2, (c.g[S2][0] as u32).wrapping_add(0x40));
            c.sq(2, sp(c).wrapping_add(0x40));
        }
        // tail: verify(self)
        c.cycles += 8;
        c.g[RA][0] = sx(0x00165694);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, APPLY_FIXED_DIR_VERIFY, 0x00165694)?;
        c.lq(16, sp(c).wrapping_add(0xa0));
        c.lq(17, sp(c).wrapping_add(0x90));
        c.lq(18, sp(c).wrapping_add(0x80));
        c.g[RA][0] = c.ld(sp(c).wrapping_add(0x70));
        c.g[SP][0] = sx(sp(c).wrapping_add(0xb0));
        ret(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    #[test]
    fn chase_camera_apply_fixed_direction_matches() {
        check(
            ssx3_ee::s01::f_00165540__generated,
            chase_camera_apply_fixed_direction,
            200,
            &|c, seed| {
                let mut r = Rng(0x1655_4000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.f[12] = r.f32(3.0);
                for i in 0..4 {
                    c.bus
                        .write32(APPLY_FIXED_DIR_AXIS + i * 4, r.edge_f32());
                }
                c.bus.write32(APPLY_FIXED_DIR_ROT, r.f32(1.0));
                c.bus.write32(APPLY_FIXED_DIR_ROT + 4, r.f32(1.0));
                c.bus.write32(APPLY_FIXED_DIR_ROT + 8, r.f32(1.0));
                // CamTargetPos at +0x20, Position at +0x40. Every fourth seed
                // puts the eye on the target, so the horizontal length is 0.
                for i in 0..4 {
                    let target = r.f32(400.0);
                    let eye = if seed % 4 == 0 {
                        target
                    } else {
                        r.f32(400.0)
                    };
                    c.bus.write32(DATA + 0x20 + i * 4, target);
                    c.bus.write32(DATA + 0x40 + i * 4, eye);
                }
            },
        );
    }
}
