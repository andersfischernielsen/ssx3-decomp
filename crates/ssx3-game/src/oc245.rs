//! Staging wave 206 (oc245): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{self, r32, ret};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0035_9830, spline_modifier_update_transform, "SplineModifier_UpdateTransform"),
    (0x0012_78e8, player_update_rumble, "Player_UpdateRumble"),
    (0x0036_a1e8, deferred_delete_advance_frame, "DeferredDelete_AdvanceFrame"),
    (0x002b_ccc8, twpigd_mix_blend, "tWPIGD_Mix_Blend"),
    (0x0013_8960, rider_wipeout_state_cast_ground_ray, "RiderWipeoutState_CastGroundRay"),
    (0x003b_54e0, snd_run_change_callbacks, "SND_RunChangeCallbacks"),
    (0x0037_4440, bezier_patch_cache_tess_uv2, "BezierPatchCache_TessUV2"),
    (0x003b_7f18, sndstrm_get_volume, "SNDSTRM_GetVolume"),
    (0x002a_a020, stream_buffer_get_status, "StreamBuffer_GetStatus"),
    (0x002a_ac28, stream_array_update_all, "StreamArray_UpdateAll"),
];

// ---------------------------------------------------------------------------
// SplineModifier_UpdateTransform (0x359830, splinemodifier.cpp)

/// SplineModifier fields.
const SM_MODE: u32 = 0x34;
const SM_SPIN: u32 = 0x38;
const SM_DIST: u32 = 0x3c;
const SM_DIRTY: u32 = 0x58;
/// The driven object's 4x4 matrix (rows 0x60..0x9c).
const SM_MATRIX: u32 = 0x60;
const SM_POS: u32 = 0xa0;
const SM_DIR: u32 = 0xb0;
const SM_ACC: u32 = 0xc0;
/// dir.x^2, dir.y^2.
const SM_DIR_SQ: u32 = 0xd0;
const SM_PATH: u32 = 0xd8;

const SM_EVAL_PATH_AT_DISTANCE: Callee = Callee {
    addr: 0x0034_5248,
    f: ee::s11::SplineModifier_EvalPathAtDistance,
};
const SM_MATH_SIN_COS: u32 = 0x0031_be50;
const SM_MATH_ATAN: u32 = 0x0031_c228;
/// The world up axis (vec3) the spin rotates about.
const SM_UP_AXIS: u32 = 0x004f_f160;

/// One angle of the transform: `atan(y / x)` (f21 / f20) fixed into the
/// right quadrant with `+lits[2]` / `-lits[3]` (pi) when `x < 0`, or
/// `lits[0]` / `lits[1]` (-pi/2, +pi/2 by the sign of `y`) when `x == 0`,
/// or `y` itself (0) when both are 0; `zero` holds 0.0. Leaves the angle in
/// f0 and applies it: yaw (`pitch == false`) `f25 = f24 - angle`, pitch
/// `f24 += angle`. The caller charged the block that made the `x == 0`
/// compare.
fn sm_angle(c: &mut Cpu, zero: usize, lits: [u32; 4], ret_addr: u32, pitch: bool) -> crate::call::Flow {
    let apply = |c: &mut Cpu| {
        if pitch {
            c.add_s(24, 24, 0);
        } else {
            c.sub_s(25, 24, 0);
        }
    };
    let gp = r32(c, GP);
    if c.fcc() {
        // x == 0
        c.cycles += 4;
        c.c_eq_s(21, zero);
        let y_zero = c.fcc();
        c.mov_s(0, 21);
        if y_zero {
            c.cycles += 1;
        } else {
            c.cycles += 4;
            c.c_le_s(zero, 21);
            c.lwc1(0, gp.wrapping_sub(lits[0]));
            if c.fcc() {
                c.cycles += 3;
                c.lwc1(0, gp.wrapping_sub(lits[1]));
            }
        }
        apply(c);
        return Ok(());
    }
    c.cycles += 7;
    c.div_s(12, 21, 20);
    call_ptr(c, SM_MATH_ATAN, ret_addr)?;
    c.c_lt_s(20, zero);
    let x_neg = c.fcc();
    c.mov_s(12, 0);
    if x_neg {
        c.cycles += 3;
        c.c_lt_s(zero, 21);
        if c.fcc() {
            c.cycles += 3;
            c.lwc1(0, gp.wrapping_sub(lits[2]));
            c.add_s(12, 12, 0);
        } else {
            c.cycles += 2;
            c.lwc1(0, gp.wrapping_sub(lits[3]));
            c.sub_s(12, 12, 0);
        }
    }
    c.cycles += 2;
    c.mov_s(0, 12);
    apply(c);
    Ok(())
}

/// `void SplineModifier_UpdateTransform(SplineModifier* self)` (0x359830):
/// evaluates the path at the modifier's distance (position, direction,
/// third output), stores the squared direction x/y, derives the yaw
/// (`-atan2(dir.y, dir.x)` of the normalised direction, modes 2/3) and the
/// pitch (`atan2(dir.z, |dir.xy|)`, modes 1/3), writes the object matrix
/// from the sines/cosines of (roll 0, pitch, yaw) with the path position as
/// translation, and with a non-zero spin angle multiplies it by a rotation
/// about the world up axis. Clears the dirty flag. Register order of the
/// original.
pub fn spline_modifier_update_transform(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 43;
        let frame = Frame::open(c, 0x180);
        frame.save(c, S1, 0x130);
        frame.save_fpr(c, 25, 0x178);
        c.g[S1][0] = c.g[A0][0];
        let this = r32(c, S1);
        let at = |o: u32| this.wrapping_add(o);
        frame.save_fpr(c, 24, 0x170);
        frame.save_fpr(c, 23, 0x168);
        frame.save(c, S0, 0x140);
        frame.save(c, S2, 0x120);
        frame.save_ra(c, 0x110);
        frame.save_fpr(c, 22, 0x160);
        frame.save_fpr(c, 21, 0x158);
        frame.save_fpr(c, 20, 0x150);
        c.g[A0][0] = sx(at(SM_PATH));
        c.g[A1][0] = sx(at(SM_POS));
        c.g[A2][0] = sx(at(SM_DIR));
        c.g[A3][0] = sx(at(SM_ACC));
        c.mtc1(0, 23); // f23 = 0
        c.lwc1(12, at(SM_DIST));
        c.mov_s(24, 23); // pitch = 0
        c.mov_s(25, 23); // yaw = 0
        SM_EVAL_PATH_AT_DISTANCE.call(c, 0x0035_9884)?;

        // Squared direction x/y; the normalised direction to sp+0x10 and sp.
        c.lwc1(0, at(SM_DIR));
        c.lwc1(1, at(SM_DIR + 4));
        c.mul_s(0, 0, 0);
        c.mul_s(1, 1, 1);
        c.swc1(0, at(SM_DIR_SQ));
        c.swc1(1, at(SM_DIR_SQ + 4));
        c.lqc2(3, at(SM_DIR));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().rsqrt(0, 3, 4, 0);
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 3, Src::Q);
        let a = frame.at(c, 0x10);
        c.sqc2(5, a);
        c.g[V0][0] = Addr(at(SM_MODE)).lw(c);
        frame.restore(c, V1, 0x10);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(2));
        c.g[V0][0] = (c.g[V0][0] < 2) as u64;
        frame.save(c, V1, 0);
        if c.g[V0][0] != 0 {
            // yaw = -atan2(n.y, n.x) (modes 2, 3)
            c.cycles += 5;
            frame.restore_fpr(c, 20, 0);
            c.c_eq_s(20, 23);
            frame.restore_fpr(c, 21, 4);
            sm_angle(c, 23, [0x2a50, 0x2a4c, 0x2a48, 0x2a44], 0x0035_9928, false)?;
        }

        c.cycles += 4;
        c.g[V1][0] = Addr(at(SM_MODE)).lw(c);
        c.g[V0][0] = 1;
        let mut pitch = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 3;
        if !pitch {
            c.cycles += 2;
            pitch = c.g[V1][0] == c.g[V0][0];
        }
        if pitch {
            // pitch = atan2(dir.z, |dir.xy|) (modes 1, 3)
            c.cycles += 9;
            c.lwc1(0, at(SM_DIR_SQ));
            c.lwc1(1, at(SM_DIR_SQ + 4));
            c.mtc1(0, 22); // f22 = 0
            c.add_s(0, 0, 1);
            c.lwc1(21, at(SM_DIR + 8));
            c.sqrt_s(20, 0);
            c.c_eq_s(20, 22);
            sm_angle(c, 22, [0x2a40, 0x2a3c, 0x2a38, 0x2a34], 0x0035_99d4, true)?;
            c.cycles += 1;
        }
        c.lwc1(2, at(SM_POS));

        // sin/cos of roll (0), pitch and yaw.
        c.cycles += 73;
        c.g[A0][0] = sx(frame.at(c, 0xf0));
        c.lwc1(1, at(SM_POS + 4));
        c.g[A1][0] = sx(frame.at(c, 0xf4));
        c.lwc1(0, at(SM_POS + 8));
        c.mov_s(12, 23);
        frame.save_fpr(c, 2, 0x20);
        c.g[S2][0] = sx(at(SM_MATRIX));
        frame.save_fpr(c, 1, 0x24);
        frame.save_fpr(c, 0, 0x28);
        frame.save_fpr(c, 24, 0x14);
        frame.save_fpr(c, 25, 0x18);
        frame.save_fpr(c, 23, 0x10);
        call_ptr(c, SM_MATH_SIN_COS, 0x0035_9a44)?;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 21); // f21 = 1.0
        frame.restore_fpr(c, 12, 0x14);
        c.g[A0][0] = sx(frame.at(c, 0xf8));
        c.g[A1][0] = sx(frame.at(c, 0xfc));
        call_ptr(c, SM_MATH_SIN_COS, 0x0035_9a5c)?;
        frame.restore_fpr(c, 12, 0x18);
        c.g[A0][0] = sx(frame.at(c, 0x100));
        c.g[A1][0] = sx(frame.at(c, 0x104));
        call_ptr(c, SM_MATH_SIN_COS, 0x0035_9a6c)?;
        sm_object_matrix(c, frame);
        if !c.fcc() {
            // A spin angle: matrix = matrix * rotation(spin) about up.
            c.cycles += 99;
            c.lwc1(12, at(SM_SPIN));
            c.g[A0][0] = sx(frame.at(c, 0x108));
            c.g[A1][0] = sx(frame.at(c, 0x10c));
            c.g[S0][0] = 0x50_0000;
            call_ptr(c, SM_MATH_SIN_COS, 0x0035_9b48)?;
            sm_spin_matrix(c, frame);
            let (m, spin) = (r32(c, S2), frame.at(c, 0x30));
            for i in 0..4u8 {
                c.lqc2(4 + i as usize, m + 0x10 * i as u32);
            }
            for i in 0..4u8 {
                c.lqc2(8 + i as usize, spin + 0x10 * i as u32);
            }
            for i in 0..4u8 {
                let row = 8 + i;
                c.vu0().mula(0b1111, 4, Src::Bc(row, X));
                c.vu0().madda(0b1111, 5, Src::Bc(row, Y));
                c.vu0().madda(0b1111, 6, Src::Bc(row, Z));
                c.vu0().madd(0b1111, 12 + i, 7, Src::Bc(row, W));
            }
            let (b, a) = (r32(c, V1), r32(c, A0));
            for i in 0..4u8 {
                c.sqc2(12 + i as usize, b + 0x10 * i as u32);
            }
            for (from, to) in [(b, a), (a, m)] {
                for i in 0..4u8 {
                    c.lqc2(1 + i as usize, from + 0x10 * i as u32);
                }
                for i in 0..4u8 {
                    c.sqc2(1 + i as usize, to + 0x10 * i as u32);
                }
            }
        }

        c.cycles += 13;
        Addr(at(SM_DIRTY)).sw(c, 0);
        frame.restore(c, S0, 0x140);
        frame.restore(c, S1, 0x130);
        frame.restore(c, S2, 0x120);
        frame.restore_ra(c, 0x110);
        for (f, off) in [(25, 0x178), (24, 0x170), (23, 0x168), (22, 0x160), (21, 0x158), (20, 0x150)] {
            frame.restore_fpr(c, f, off);
        }
        frame.close(c);
        Ok(())
    })
}

/// The object matrix (s1 + 0x60) from the sines/cosines of roll (sp+0xf0/
/// 0xf4), pitch (0xf8/0xfc) and yaw (0x100/0x104), translation = the path
/// position (sp+0x20), w = 1.0 (f21); then compares the spin angle with 0
/// (the condition bit decides the spin rotation). Verbatim register order.
fn sm_object_matrix(c: &mut Cpu, frame: Frame) {
    let s1 = r32(c, S1);
    let m = |o: u32| s1.wrapping_add(o);
    let sp = frame.at(c, 0);
    let s = |o: u32| sp.wrapping_add(o);
    c.lwc1(0, s(0xf0));
    c.g[V0][0] = sx(m(0x10));
    c.lwc1(9, s(0xf8));
    c.lwc1(6, s(0xf4));
    c.neg_s(14, 0);
    c.lwc1(10, s(0x100));
    c.mul_s(7, 0, 9);
    c.lwc1(8, s(0x104));
    c.mul_s(2, 6, 9);
    c.mul_s(17, 6, 10);
    c.lwc1(3, s(0xfc));
    c.mul_s(15, 0, 8);
    c.sw(m(0x6c), 0);
    c.mul_s(16, 7, 10);
    c.lwc1(18, m(SM_SPIN));
    c.neg_s(5, 3);
    c.lwc1(20, m(0x6c));
    c.mul_s(1, 6, 8);
    c.lwc1(11, s(0x20));
    c.mul_s(4, 17, 9);
    c.lwc1(12, s(0x24));
    c.mul_s(0, 0, 10);
    c.lwc1(13, s(0x28));
    c.mul_s(2, 2, 8);
    c.swc1(11, m(0x90));
    c.mul_s(7, 7, 8);
    c.swc1(12, m(0x94));
    c.mul_s(6, 6, 3);
    c.swc1(13, m(0x98));
    c.mul_s(14, 14, 3);
    c.swc1(9, m(0x68));
    c.sub_s(0, 0, 2);
    c.sw(m(0x7c), 0);
    c.add_s(4, 4, 15);
    c.swc1(6, m(0x88));
    c.sub_s(1, 1, 16);
    c.swc1(14, m(0x78));
    c.mul_s(5, 5, 10);
    c.swc1(0, m(0x80));
    c.add_s(7, 7, 17);
    c.swc1(4, m(0x84));
    c.mul_s(3, 3, 8);
    c.swc1(1, m(0x74));
    c.c_eq_s(18, 20);
    c.swc1(5, m(0x64));
    c.swc1(7, m(0x70));
    c.swc1(3, m(0x60));
    c.sw(m(0x8c), 0);
    c.swc1(21, m(0x9c));
}

/// The spin rotation about the world up axis from sin (sp+0x108) and cos
/// (sp+0x10c) into the frame's 4x4 matrix at sp+0x30 (bottom row 0, 0, 0,
/// 1); leaves v1 = sp+0xb0, a0 = sp+0x70, a1 = sp+0x30. Verbatim register
/// order.
fn sm_spin_matrix(c: &mut Cpu, frame: Frame) {
    let sp = frame.at(c, 0);
    let s = |o: u32| sp.wrapping_add(o);
    c.lwc1(11, s(0x10c));
    c.g[V0][0] = sx(r32(c, S0).wrapping_sub(0xea0));
    c.lwc1(4, SM_UP_AXIS + 8);
    c.g[A0][0] = sx(s(0x70));
    c.sub_s(1, 21, 11);
    c.lwc1(0, SM_UP_AXIS);
    c.lwc1(6, SM_UP_AXIS + 4);
    c.g[V1][0] = sx(s(0xb0));
    c.lwc1(8, s(0x108));
    c.g[A1][0] = sx(s(0x30));
    c.mul_s(2, 1, 4);
    c.swc1(20, s(0x5c));
    c.mul_s(3, 1, 0);
    c.swc1(21, s(0x6c));
    c.mul_s(1, 1, 6);
    c.swc1(20, s(0x60));
    c.mul_s(13, 8, 4);
    c.swc1(20, s(0x64));
    c.mul_s(12, 8, 0);
    c.swc1(20, s(0x68));
    c.mul_s(9, 2, 4);
    c.swc1(20, s(0x3c));
    c.mul_s(10, 3, 4);
    c.swc1(20, s(0x4c));
    c.mul_s(5, 2, 0);
    c.mul_s(7, 3, 0);
    c.mul_s(4, 1, 4);
    c.mul_s(0, 1, 0);
    c.mul_s(8, 8, 6);
    c.mul_s(2, 2, 6);
    c.mul_s(3, 3, 6);
    c.mul_s(1, 1, 6);
    c.add_s(5, 5, 8);
    c.sub_s(0, 0, 13);
    c.sub_s(2, 2, 12);
    c.add_s(9, 9, 11);
    c.swc1(5, s(0x38));
    c.add_s(7, 7, 11);
    c.swc1(0, s(0x34));
    c.add_s(3, 3, 13);
    c.swc1(2, s(0x48));
    c.sub_s(10, 10, 8);
    c.swc1(9, s(0x58));
    c.add_s(1, 1, 11);
    c.swc1(7, s(0x30));
    c.add_s(4, 4, 12);
    c.swc1(3, s(0x40));
    c.swc1(10, s(0x50));
    c.swc1(1, s(0x44));
    c.swc1(4, s(0x54));
}

// ---------------------------------------------------------------------------
// Player_UpdateRumble (0x1278e8, rider.cpp)

/// Player fields.
const PL_RIDER: u32 = 0x18;
const PL_PAD: u32 = 0xdf4;
const PL_RUMBLE: u32 = 0xdfc;
/// The rider's controller-input sub-object (vtable slot 0x38: the pad id).
const RIDER_INPUT: u32 = 0x6c0;

const PLAYER_APPLY_RUMBLE: Callee = Callee {
    addr: 0x0012_5b18,
    f: ee::s00::Player_ApplyRumble,
};

/// `void Player_UpdateRumble(Player* self)` (0x1278e8): with a pad
/// attached, asks the BE player interface whether vibration is enabled for
/// the rider's pad; if so, rumbling is allowed only once the world's race
/// countdown (`world->0x84->0x28->[0]`) is 0 or at least 10. Then applies
/// the rumble state (`Player_ApplyRumble(&self->rumble, pad, allowed)`).
pub fn player_update_rumble(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        let this = r32(c, S1);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(this.wrapping_add(PL_PAD)).lw(c);
        frame.restore(c, S0, 0x30);
        if c.g[V0][0] != 0 {
            c.cycles += 18;
            c.g[S2][0] = 0;
            call_ptr(c, 0x0014_dc80, 0x0012_7914)?; // cBE_getBE
            c.g[A1][0] = 1;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_dd58, 0x0012_7920)?; // BELibrary_GetInterface(be, 1)
            // pad id = rider->input.vfunc_0x38()
            c.g[A0][0] = Addr(this.wrapping_add(PL_RIDER)).lw(c);
            c.g[S0][0] = c.g[V0][0];
            c.g[V1][0] = Addr(r32(c, A0).wrapping_add(RIDER_INPUT)).lw(c);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(RIDER_INPUT));
            c.g[A1][0] = Addr(r32(c, V1).wrapping_add(0x38)).lh(c);
            c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x3c)).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
            call_ptr(c, target, 0x0012_7940)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0014_74e8, 0x0012_794c)?; // BENewPlayerInt_GetVibration
            let vibration = c.g[V0][0] != 0;
            c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
            if vibration {
                c.cycles += 5;
                c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x84)).lw(c);
                c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x28)).lw(c);
                c.g[A0][0] = Addr(r32(c, A0)).lw(c);
                c.g[A2][0] = 0;
                if c.g[A0][0] != 0 {
                    c.cycles += 1;
                    c.g[A2][0] = ((c.g[A0][0] as i64) < 10) as u64;
                }
                c.cycles += 1;
                c.g[S2][0] = c.g[A2][0] ^ 1;
            }
            c.cycles += 5;
            c.g[A1][0] = Addr(this.wrapping_add(PL_PAD)).lw(c);
            c.g[A2][0] = c.g[S2][0];
            c.g[A0][0] = sx(this.wrapping_add(PL_RUMBLE));
            PLAYER_APPLY_RUMBLE.call(c, 0x0012_7980)?;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// DeferredDelete_AdvanceFrame (0x36a1e8, graphicsman.cpp)

/// gp-relative index of the current deferred-delete bucket (0..3).
const DD_INDEX: u32 = 0x2a98;
/// The four buckets of objects whose deletion is deferred (0x53ab40).
const DD_BUCKETS_HI: u32 = 0x0054_0000;
const DD_BUCKETS_LO: u32 = 0x54c0;

/// `void DeferredDelete_AdvanceFrame()` (0x36a1e8): advances the bucket
/// index (`(index + 1) % 4`), deletes every object queued in that bucket
/// four frames ago (virtual destructor with flags 3, through the list's
/// `next` links) and empties the bucket.
pub fn deferred_delete_advance_frame(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let index = Addr(r32(c, GP).wrapping_add(DD_INDEX));
        c.g[V0][0] = index.lw(c);
        c.g[V1][0] = u64::MAX;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = DD_BUCKETS_HI as u64;
        c.g[A0][0] = sx(r32(c, V0).wrapping_add(1));
        frame.save(c, S1, 0x10);
        // (index + 1) % 4, C remainder
        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(4));
        if c.g[V1][0] != 0 {
            c.g[V0][0] = c.g[A0][0];
        }
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = sx(((r32(c, V0) as i32) >> 2) as u32);
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(r32(c, V0) << 2);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_sub(r32(c, V0)));
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(DD_BUCKETS_LO));
        c.g[V1][0] = sx(r32(c, A0) << 2);
        let new_index = c.g[A0][0];
        index.sw(c, new_index);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
        c.g[A2][0] = Addr(r32(c, V1)).lw(c);
        let empty = c.g[A2][0] == 0;
        c.g[V0][0] = index.lw(c);
        if !empty {
            loop {
                c.cycles += 2;
                let node = r32(c, A2);
                c.g[S0][0] = Addr(node).lw(c); // next
                if c.g[A2][0] != 0 {
                    // node->~Object() (deleting, flags 3)
                    c.cycles += 6;
                    c.g[V0][0] = Addr(node.wrapping_add(4)).lw(c);
                    c.g[A1][0] = 3;
                    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(8)).lh(c);
                    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0xc)).lw(c);
                    let dtor = r32(c, V1);
                    c.g[A0][0] = sx(node.wrapping_add(r32(c, A0)));
                    call_ptr(c, dtor, 0x0036_a260)?;
                }
                c.cycles += 3;
                c.g[A2][0] = c.g[S0][0];
                c.g[V0][0] = index.lw(c);
                if c.g[A2][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0036_a240);
                }
            }
        }
        c.cycles += 9;
        c.g[V1][0] = sx(r32(c, S1).wrapping_sub(DD_BUCKETS_LO));
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = sx(r32(c, V0) << 2);
        frame.restore(c, S1, 0x10);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        frame.restore_ra(c, 0);
        Addr(r32(c, V0)).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// tWPIGD_Mix_Blend (0x2bccc8, seg_1BA100.cpp)

/// tWPIGD (audio parameter) fields: vtable at 4 (slot 0x218: lock/update),
/// the integer value at 8, the target's value copied to 0xc.
const WPIGD_VTABLE: u32 = 0x4;
const WPIGD_VALUE: u32 = 0x8;
const WPIGD_TARGET: u32 = 0xc;

/// `void tWPIGD_Mix_Blend(tWPIGD* self, tWPIGD* target, float t)`
/// (0x2bccc8): calls the vtable slot at 0x218, blends the integer value
/// towards the target's by `t^2` (`value = t^2 * target + (1 - t^2) *
/// value`, truncated), copies the target's value to +0xc and calls the
/// slot again.
pub fn twpigd_mix_blend(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        let (this, target) = (r32(c, S0), r32(c, S1));
        frame.save_fpr(c, 20, 0x30);
        c.mov_s(20, 12);
        c.g[V1][0] = Addr(this.wrapping_add(WPIGD_VTABLE)).lw(c);
        c.mul_s(20, 20, 20); // t^2
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x218)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x21c)).lw(c);
        let method = r32(c, V0);
        c.g[A0][0] = sx(this.wrapping_add(r32(c, A0)));
        call_ptr(c, method, 0x002b_cd00)?;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0); // f0 = 1.0
        c.lwc1(1, target.wrapping_add(4)); // the target's value
        c.cvt_s_w(1, 1);
        c.lwc1(2, this.wrapping_add(WPIGD_VALUE));
        c.cvt_s_w(2, 2);
        c.g[V1][0] = Addr(this.wrapping_add(WPIGD_VTABLE)).lw(c);
        c.sub_s(0, 0, 20);
        c.mul_s(20, 20, 1);
        c.mul_s(0, 0, 2);
        c.add_s(20, 20, 0);
        c.cvt_w_s(0, 20);
        c.swc1(0, this.wrapping_add(WPIGD_VALUE));
        c.g[V0][0] = Addr(target.wrapping_add(4)).lw(c);
        let v = c.g[V0][0];
        Addr(this.wrapping_add(WPIGD_TARGET)).sw(c, v);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x218)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x21c)).lw(c);
        let method = r32(c, V0);
        c.g[A0][0] = sx(this.wrapping_add(r32(c, A0)));
        call_ptr(c, method, 0x002b_cd4c)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderWipeoutState_CastGroundRay (0x138960, wipeoutmotion.cpp)

const WO_RIDER: u32 = 0x40;
/// Rider fields: position, the surface id / contact uv / surface flags the
/// ray stores, the overlap list and its collision flags.
const WO_R_POS: u32 = 0x110;
const WO_R_SURFACE: u32 = 0x430;
const WO_R_CONTACT_U: u32 = 0xaac;
const WO_R_CONTACT_V: u32 = 0xab0;
const WO_R_SURFACE_FLAGS: u32 = 0x2d4;
const WO_R_OVERLAP: u32 = 0x860;
const WO_R_OVERLAP_FLAGS: u32 = 0x864;
/// Contact fields: object, surface, uv.
const HIT_OBJECT: u32 = 0x50;
const HIT_SURFACE: u32 = 0x54;
const HIT_U: u32 = 0x6c;
const HIT_V: u32 = 0x70;

const WO_COLL_SEGMENT_CTOR: u32 = 0x0032_e100;
const WO_FIND_CLOSEST_CONTACT: u32 = 0x0033_42d0;

/// `float RiderWipeoutState_CastGroundRay(RiderWipeoutState* self, Hit*
/// hit, const Vec4* dir)` (0x138960): a capsule (radius `gp-0x721c`) from
/// `pos - 200 * dir` to `pos + 200 * dir` through the rider's overlap list;
/// on a contact (distance >= 0) stores the surface id (or -1 without a
/// surface), the contact uv and the surface flags in the rider, and lets a
/// hit object's handler (vtable slot 0x150) see the contact. Returns the
/// distance.
pub fn rider_wipeout_state_cast_ground_ray(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 55;
        let frame = Frame::open(c, 0x140);
        let at = |c: &Cpu, o: u32| sp(c).wrapping_add(o);
        c.g[V1][0] = c.g[A2][0]; // dir
        frame.save(c, S0, 0x120);
        c.g[A2][0] = sx(at(c, 0xd0));
        frame.save(c, S1, 0x110);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x130);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x100);
        let (this, hit) = (r32(c, S0), r32(c, S1));
        c.g[A0][0] = c.g[SP][0];
        c.g[AT][0] = 0x4348_0000;
        c.mtc1(AT, 0); // 200.0
        c.g[A1][0] = sx(at(c, 0xb0));
        c.g[T0][0] = Addr(this.wrapping_add(WO_RIDER)).lw(c);
        c.g[A3][0] = 2;
        // start = pos - dir * 200 (sp+0xb0)
        c.mfc1(V0, 0);
        c.lqc2(4, r32(c, V1));
        c.qmtc2(V0, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, 0xd0));
        c.lwc1(12, r32(c, GP).wrapping_sub(0x721c));
        c.lq(V0, at(c, 0xd0));
        c.sq(V0, at(c, 0xc0));
        c.lqc2(3, r32(c, T0).wrapping_add(WO_R_POS));
        c.lqc2(4, at(c, 0xc0));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, 0xd0));
        c.lq(V0, at(c, 0xd0));
        c.sq(V0, at(c, 0xb0));
        // end = pos + dir * 200 (sp+0xd0)
        c.mfc1(T1, 0);
        c.lqc2(4, r32(c, V1));
        c.qmtc2(T1, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, 0xf0));
        c.lq(V0, at(c, 0xf0));
        c.sq(V0, at(c, 0xe0));
        c.lqc2(3, r32(c, T0).wrapping_add(WO_R_POS));
        c.lqc2(4, at(c, 0xe0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, 0xf0));
        c.lq(V0, at(c, 0xf0));
        c.sq(V0, at(c, 0xd0));
        call_ptr(c, WO_COLL_SEGMENT_CTOR, 0x0013_8a0c)?;
        // d = FindClosestContact(rider->overlap, &seg, hit, rider->flags)
        c.g[V1][0] = Addr(this.wrapping_add(WO_RIDER)).lw(c);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = Addr(r32(c, V1).wrapping_add(WO_R_OVERLAP_FLAGS)).lw(c);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(WO_R_OVERLAP)).lw(c);
        call_ptr(c, WO_FIND_CLOSEST_CONTACT, 0x0013_8a24)?;
        c.mov_s(20, 0);
        c.mtc1(0, 0);
        c.c_le_s(0, 20);
        let contact = c.fcc();
        c.mov_s(0, 20);
        if contact {
            c.cycles += 3;
            c.g[V0][0] = Addr(hit.wrapping_add(HIT_SURFACE)).lw(c);
            if c.g[V0][0] == 0 {
                c.g[V1][0] = Addr(this.wrapping_add(WO_RIDER)).lw(c);
                c.cycles += 3;
                c.g[V0][0] = u64::MAX;
                Addr(r32(c, V1).wrapping_add(WO_R_SURFACE)).sw(c, u64::MAX);
            } else {
                c.cycles += 14;
                c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x150)).lw(c);
                c.g[V0][0] = Addr(this.wrapping_add(WO_RIDER)).lw(c);
                let v = c.g[V1][0];
                Addr(r32(c, V0).wrapping_add(WO_R_SURFACE)).sw(c, v);
                c.g[A0][0] = Addr(this.wrapping_add(WO_RIDER)).lw(c);
                c.lwc1(0, hit.wrapping_add(HIT_U));
                c.swc1(0, r32(c, A0).wrapping_add(WO_R_CONTACT_U));
                c.g[V0][0] = Addr(this.wrapping_add(WO_RIDER)).lw(c);
                c.lwc1(0, hit.wrapping_add(HIT_V));
                c.swc1(0, r32(c, V0).wrapping_add(WO_R_CONTACT_V));
                c.g[V1][0] = Addr(hit.wrapping_add(HIT_SURFACE)).lw(c);
                c.g[A0][0] = Addr(this.wrapping_add(WO_RIDER)).lw(c);
                c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0xa)).lhu(c);
                let v = c.g[V0][0];
                Addr(r32(c, A0).wrapping_add(WO_R_SURFACE_FLAGS)).sh(c, v);
            }
            c.cycles += 3;
            c.g[A0][0] = Addr(hit.wrapping_add(HIT_OBJECT)).lw(c);
            c.mov_s(0, 20);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                c.g[A2][0] = Addr(r32(c, A0).wrapping_add(0xc)).lw(c);
                c.g[A1][0] = c.g[S1][0];
                if c.g[A2][0] != 0 {
                    // handler->OnContact(hit) (vtable slot 0x150)
                    c.cycles += 6;
                    let handler = r32(c, A2);
                    c.g[V0][0] = Addr(handler.wrapping_add(0xc)).lw(c);
                    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x150)).lh(c);
                    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x154)).lw(c);
                    let method = r32(c, V1);
                    c.g[A0][0] = sx(handler.wrapping_add(r32(c, A0)));
                    call_ptr(c, method, 0x0013_8ab8)?;
                    c.mov_s(0, 20);
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x120);
        frame.restore(c, S1, 0x110);
        frame.restore_ra(c, 0x100);
        frame.restore_fpr(c, 20, 0x130);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_RunChangeCallbacks (0x3b54e0, seg_2B4578.c)

/// The sound library's globals (0x50a8e8): change-callback list (+0x1b4)
/// and the change counter (+0x1c0); the counter value last seen (0x44c468).
const SND_GLOBALS: u32 = 0x0051_0000 - 0x5718;
const SND_CB_LIST: u32 = 0x1b4;
const SND_CHANGE_COUNT: u32 = 0x1c0;
const SND_SEEN_HI: u32 = 0x0045_0000;
const SND_SEEN_LO: u32 = 0x3b98;

/// `void SND_RunChangeCallbacks()` (0x3b54e0): when the sound library's
/// change counter moved since the last call, remember it and call every
/// registered callback (`node->fn(node->arg)`, `{next, ?, fn, arg}`).
pub fn snd_run_change_callbacks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x51_0000;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = SND_SEEN_HI as u64;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(SND_GLOBALS);
        let seen = Addr(SND_SEEN_HI - SND_SEEN_LO);
        c.g[V1][0] = seen.lw(c);
        c.g[A0][0] = Addr(SND_GLOBALS + SND_CHANGE_COUNT).lw(c);
        let unchanged = c.g[A0][0] == c.g[V1][0];
        frame.restore(c, S0, 0x10);
        if !unchanged {
            c.cycles += 3;
            c.g[S0][0] = Addr(SND_GLOBALS + SND_CB_LIST).lw(c);
            let count = c.g[A0][0];
            seen.sw(c, count);
            if c.g[S0][0] != 0 {
                c.cycles += 1;
                c.g[V0][0] = Addr(r32(c, S0).wrapping_add(8)).lw(c);
                loop {
                    c.cycles += 5;
                    let f = r32(c, V0);
                    c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0xc)).lw(c);
                    call_ptr(c, f, 0x003b_5520)?;
                    c.g[S0][0] = Addr(r32(c, S0)).lw(c);
                    if c.g[S0][0] == 0 {
                        break;
                    }
                    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(8)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x003b_5518);
                    }
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BezierPatchCache_TessUV2 (0x374440, particle.cpp)

/// Per-level tessellation sizes (`n` vertices per side).
const BPC_TESS_N: u32 = 0x19c;

/// `void BezierPatchCache_TessUV2(BezierPatchCache* self, const Rect* uv,
/// Vec2* out, int level)` (0x374440): writes an `n x n` grid of uv pairs
/// (`n = self->tess_n[level]`) spanning `uv` (origin at +0x10, size at
/// +0x18; step = size / (n - 1)), row by row with v advancing fastest.
/// Leaf; the 8-byte pairs move with ldl/ldr, sdl/sdr as in the original.
pub fn bezier_patch_cache_tess_uv2(c: &mut Cpu) {
    c.cycles += 26;
    c.g[A3][0] = sx(r32(c, A3) << 2);
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A3)));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(AT, 2); // f2 = 1.0
    c.g[A0][0] = Addr(r32(c, A0).wrapping_add(BPC_TESS_N)).lw(c);
    let rect = r32(c, A1);
    let s = sp(c);
    // pos = uv->origin (sp+0), step = uv->size (sp+0x10)
    c.ldl(V0, rect.wrapping_add(0x17));
    c.ldr(V0, rect.wrapping_add(0x10));
    c.sdl(V0, s.wrapping_add(0x7));
    c.sdr(V0, s);
    c.g[V0][0] = sx(r32(c, A0).wrapping_sub(1));
    c.ldl(T1, rect.wrapping_add(0x1f));
    c.ldr(T1, rect.wrapping_add(0x18));
    c.sdl(T1, s.wrapping_add(0x17));
    c.sdr(T1, s.wrapping_add(0x10));
    c.mtc1(V0, 3);
    c.cvt_s_w(3, 3);
    c.g[V1][0] = 0;
    c.lwc1(1, s.wrapping_add(0x10));
    c.lwc1(0, s.wrapping_add(0x14));
    c.div_s(2, 2, 3); // 1 / (n - 1)
    c.mul_s(0, 0, 2);
    c.mul_s(1, 1, 2);
    c.swc1(0, s.wrapping_add(0x14));
    c.swc1(1, s.wrapping_add(0x10));
    if (c.g[A0][0] as i64) > 0 {
        loop {
            // A row: cur = pos (sp+0x20).
            c.cycles += 7;
            let s = sp(c);
            c.g[V0][0] = sx(r32(c, V1).wrapping_add(1));
            c.ldl(T1, s.wrapping_add(0x7));
            c.ldr(T1, s);
            c.sdl(T1, s.wrapping_add(0x27));
            c.sdr(T1, s.wrapping_add(0x20));
            let empty = (c.g[A0][0] as i64) <= 0;
            c.g[V1][0] = c.g[A0][0];
            if !empty {
                c.cycles += 1;
                loop {
                    // *out++ = cur; cur.y += step.y
                    c.cycles += 11;
                    c.lwc1(0, s.wrapping_add(0x24));
                    c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
                    c.lwc1(1, s.wrapping_add(0x14));
                    c.ldl(A3, s.wrapping_add(0x27));
                    c.ldr(A3, s.wrapping_add(0x20));
                    let out = r32(c, A2);
                    c.sdl(A3, out.wrapping_add(0x7));
                    c.sdr(A3, out);
                    c.add_s(0, 0, 1);
                    c.g[A2][0] = sx(out.wrapping_add(8));
                    let more = c.g[V1][0] != 0;
                    c.swc1(0, s.wrapping_add(0x24));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0037_44c8);
                    }
                }
            }
            // pos.x += step.x
            c.cycles += 7;
            c.lwc1(0, s);
            c.g[V1][0] = c.g[V0][0];
            c.lwc1(1, s.wrapping_add(0x10));
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            c.add_s(0, 0, 1);
            let more = c.g[V0][0] != 0;
            c.swc1(0, s);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0037_44a8);
            }
        }
    }
    c.cycles += 2;
    c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
    ret(c);
}

// ---------------------------------------------------------------------------
// SNDSTRM_GetVolume (0x3b7f18, seg_2B4578.c)

/// Stream fields: the sound handle, the volume it was started with.
const STRM_HANDLE: u32 = 0x4;
const STRM_VOLUME: u32 = 0xec;
/// SND error code: invalid handle.
const SND_ERR_HANDLE: u64 = -8i64 as u64;

const SNDSTRM_GET_STREAM: Callee = Callee {
    addr: 0x003b_6a38,
    f: ee::s13::SNDSTRM_GetStream,
};
const SND_GET_VOLUME: Callee = Callee {
    addr: 0x003b_86c0,
    f: ee::s13::SND_GetVolume,
};

/// `int SNDSTRM_GetVolume(int stream)` (0x3b7f18): the volume of the
/// stream's playing sound, or the volume it was started with (signed byte
/// at +0xec) when the sound is no longer playing; -8 for an unknown
/// stream.
pub fn sndstrm_get_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        SNDSTRM_GET_STREAM.call(c, 0x003b_7f2c)?;
        c.g[S0][0] = c.g[V0][0];
        let stream = r32(c, S0);
        let none = c.g[S0][0] == 0;
        c.g[V0][0] = SND_ERR_HANDLE;
        if !none {
            c.cycles += 5;
            c.g[A0][0] = Addr(stream.wrapping_add(STRM_HANDLE)).lw(c);
            SND_GET_VOLUME.call(c, 0x003b_7f44)?;
            if (c.g[V0][0] as i64) < 0 {
                c.g[V0][0] = Addr(stream.wrapping_add(STRM_VOLUME)).lb(c);
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamBuffer_GetStatus (0x2aa020, streamsys.cpp)

const SND_LOCK: u32 = 0x003b_58a0;
const SND_UNLOCK: u32 = 0x003b_58d8;
const SNDSTRM_GET_STATUS: u32 = 0x003b_7b48;
/// StreamBuffer field: the SND stream id.
const SB_STREAM: u32 = 0x8;

/// `int StreamBuffer_GetStatus(StreamBuffer* self, SndStreamStatus* out)`
/// (0x2aa020): `SNDSTRM_GetStatus(self->stream, out)` under the sound
/// library lock; returns its result.
pub fn stream_buffer_get_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        let this = r32(c, S0);
        call_ptr(c, SND_LOCK, 0x002a_a03c)?;
        c.g[A0][0] = Addr(this.wrapping_add(SB_STREAM)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, SNDSTRM_GET_STATUS, 0x002a_a048)?;
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, SND_UNLOCK, 0x002a_a050)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamArray_UpdateAll (0x2aac28, streamsys.cpp)

/// StreamArray fields: buffer count, buffer pointer array.
const SA_COUNT: u32 = 0x4;
const SA_BUFFERS: u32 = 0x8;

const STREAM_BUFFER_UPDATE_VOLUME: Callee = Callee {
    addr: 0x002a_9f80,
    f: ee::s08::StreamBuffer_UpdateVolume,
};

/// `void StreamArray_UpdateAll(StreamArray* self)` (0x2aac28): updates the
/// volume of every non-null stream buffer (`StreamBuffer_UpdateVolume`).
pub fn stream_array_update_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(SA_COUNT)).lw(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S0][0] = 0;
        if !empty {
            c.cycles += 2;
            c.g[V1][0] = Addr(r32(c, S1).wrapping_add(SA_BUFFERS)).lw(c);
            loop {
                c.cycles += 5;
                c.g[V0][0] = sx(r32(c, S0) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                if c.g[A0][0] != 0 {
                    c.cycles += 3;
                    STREAM_BUFFER_UPDATE_VOLUME.call(c, 0x002a_ac6c)?;
                }
                c.g[V0][0] = Addr(r32(c, S1).wrapping_add(SA_COUNT)).lw(c);
                c.cycles += 4;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V1][0] = Addr(r32(c, S1).wrapping_add(SA_BUFFERS)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x002a_ac50);
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, elf_image, put_vec, DATA};

    /// SplineModifier_UpdateTransform: every mode (0: no angle, 1: pitch,
    /// 2: yaw, 3: both, 4: none), with and without a spin angle. The path is
    /// one segment whose direction components are zeroed per seed (x / y / z
    /// = 0 reach the zero-denominator and zero-angle branches; negative
    /// components the quadrant fixes). Real callees (EvalPathAtDistance,
    /// Math_SinCos, Math_Atan) on the game image's constants.
    #[test]
    fn spline_modifier_update_transform_matches() {
        const SELF: u32 = DATA;
        const SEG: u32 = DATA + 0x3000;
        check(ee::s11::f_00359830__generated, spline_modifier_update_transform, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x35_9830, 0);
            elf_image(c);
            c.pc = 0x0035_9830;
            let f = |x: f32| x.to_bits();
            let len = 10.0 + (r.next_u32() % 100) as f32;
            c.g[A0][0] = sx(SELF);
            c.bus.write32(SELF + SM_MODE, seed % 5);
            let spin = if seed % 4 == 0 { 0 } else { r.f32(3.0) };
            c.bus.write32(SELF + SM_SPIN, spin);
            c.bus.write32(SELF + SM_DIST, f((r.next_u32() % 10) as f32));
            c.bus.write32(SELF + SM_DIRTY, 1);
            // The path: one segment covering everything.
            c.bus.write32(SELF + SM_PATH + 0x4, 0);
            c.bus.write32(SELF + SM_PATH + 0x8, SEG);
            c.bus.write32(SELF + SM_PATH + 0xc, f(len * 2.0));
            c.bus.write32(SEG + 0x84, 0);
            c.bus.write32(SEG + 0x60, 0);
            c.bus.write32(SEG + 0x64, 0);
            for k in 0..4 {
                c.bus.write32(SEG + 0x50 + 4 * k, f((r.next_u32() % 8) as f32 * 0.125));
            }
            // Direction columns, with components zeroed by seed.
            let zero = (seed / 5) % 8; // bit 0: x, 1: y, 2: z
            for row in 0..4u32 {
                let mut v = [
                    r.f32(4.0),
                    r.f32(4.0),
                    r.f32(4.0),
                    f(if row == 3 { 1.0 } else { 0.0 }),
                ];
                for k in 0..3usize {
                    if zero & (1 << k) != 0 {
                        v[k] = 0;
                    }
                }
                put_vec(c, SEG + 0x10 + 0x10 * row, v);
            }
            // The world up axis.
            put_vec(c, SM_UP_AXIS, [f(0.0), f(0.0), f(1.0), 0]);
            if seed % 3 == 0 {
                put_vec(c, SM_UP_AXIS, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
            }
        });
    }

    /// Player_UpdateRumble: no pad (early out); with a pad, vibration on/off
    /// from the BE player slot table, the race countdown 0 / below 10 / 10+,
    /// and Player_ApplyRumble's both paths (stop actuators with one or two
    /// motors when not allowed, the motor-type check when allowed). BE
    /// interface, rider input and pad methods are fakes (counts forced).
    #[test]
    fn player_update_rumble_matches() {
        use crate::difftest::fakes::{vtable, LOG_RESULTS};
        // Away from the fakes' random vtables (DATA..DATA+0xfffff).
        const PLAYER: u32 = DATA + 0x20_0000;
        const RIDER: u32 = PLAYER + 0x2000;
        const PAD: u32 = PLAYER + 0x3000;
        const IFACE_OBJ: u32 = PLAYER + 0x3100;
        const WORLD: u32 = PLAYER + 0x3200;
        const BE: u32 = PLAYER + 0x8000;
        check(ee::s00::f_001278e8__generated, player_update_rumble, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x12_78e8, 7);
            c.pc = 0x0012_78e8;
            let gp = crate::difftest::GAME_GP;
            c.g[A0][0] = sx(PLAYER);
            c.bus.write32(PLAYER + PL_PAD, if seed % 6 == 0 { 0 } else { PAD });
            c.bus.write32(PLAYER + PL_RIDER, RIDER);
            // BELibrary singleton, interface table entry 0.
            c.bus.write32(gp - 0x1f04, BE);
            c.bus.write32(gp - 0x1efc, 0);
            c.bus.write32(0x0053_08a8, IFACE_OBJ);
            let vt = vtable(c, &mut r, &[0x8]);
            c.bus.write32(IFACE_OBJ + 4, vt);
            // The rider input's pad id (forced 1..3), its slot and flags.
            let vt = vtable(c, &mut r, &[0x38]);
            c.bus.write32(RIDER + RIDER_INPUT, vt);
            let pad_id = 1 + seed % 3;
            c.bus.write32(LOG_RESULTS + 4, pad_id);
            let slot = r.next_u32() % 4;
            c.bus.write32(0x0053_05b0 + 4 * pad_id, slot);
            c.bus.write32(0x0053_4fe0 + slot * 0x1c + 0x10, r.next_u32());
            // The world's countdown.
            c.bus.write32(gp - 0x848, WORLD);
            c.bus.write32(WORLD + 0x84, WORLD + 0x100);
            c.bus.write32(WORLD + 0x100 + 0x28, WORLD + 0x200);
            let count = match (seed / 6) % 3 {
                0 => 0,
                1 => r.next_u32() % 10,
                _ => 10 + r.next_u32() % 100,
            };
            c.bus.write32(WORLD + 0x200, count);
            // The pad: motor count / type (1 or 2), motor setters.
            let vt = vtable(c, &mut r, &[0x18, 0x20]);
            c.bus.write32(PAD, vt);
            let k = 1 + (seed / 2) % 2;
            for n in [2, 4, 6] {
                c.bus.write32(LOG_RESULTS + 4 * n, k);
            }
            // Player_ApplyRumble's .lit4 constants.
            for off in [0x7860u32, 0x785c, 0x7858, 0x7854, 0x7850, 0x784c, 0x7848] {
                c.bus.write32(gp - off, r.f32(2.0));
            }
            c.bus.write32(PLAYER + PL_RUMBLE, r.f32(2.0));
            c.bus.write32(PLAYER + PL_RUMBLE + 4, r.f32(2.0));
        });
    }

    /// DeferredDelete_AdvanceFrame: bucket indices -6..5 (both remainder
    /// forms), buckets with 0-3 queued objects whose destructors are fakes.
    #[test]
    fn deferred_delete_advance_frame_matches() {
        use crate::difftest::fakes::vtable;
        const NODES: u32 = DATA + 0x20_0000;
        check(ee::s12::f_0036a1e8__generated, deferred_delete_advance_frame, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x36_a1e8, 4);
            c.pc = 0x0036_a1e8;
            let gp = crate::difftest::GAME_GP;
            let index = (seed % 12) as i32 - 6;
            c.bus.write32(gp + DD_INDEX, index as u32);
            let next = (index + 1) % 4;
            let bucket = DD_BUCKETS_HI - DD_BUCKETS_LO;
            for k in 0..4 {
                c.bus.write32(bucket + 4 * k, r.next_u32());
            }
            let n = (seed / 12) % 4;
            let mut head = 0;
            for i in (0..n).rev() {
                let node = NODES + 0x40 * i;
                c.bus.write32(node, head);
                let vt = vtable(c, &mut r, &[0x8]);
                c.bus.write32(node + 4, vt);
                head = node;
            }
            c.bus.write32(bucket.wrapping_add((next * 4) as u32), head);
        });
    }

    /// tWPIGD_Mix_Blend: random integer values and blend factors (edge
    /// floats), the vtable slot a fake.
    #[test]
    fn twpigd_mix_blend_matches() {
        use crate::difftest::fakes::vtable;
        const SELF: u32 = DATA + 0x20_0000;
        const TARGET: u32 = SELF + 0x100;
        check(ee::s08::f_002bccc8__generated, twpigd_mix_blend, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x2b_ccc8, 2);
            c.pc = 0x002b_ccc8;
            c.g[A0][0] = sx(SELF);
            c.g[A1][0] = sx(TARGET);
            c.f[12] = if seed % 2 == 0 { r.edge_f32() } else { r.f32(1.0) };
            let vt = vtable(c, &mut r, &[0x218]);
            c.bus.write32(SELF + WPIGD_VTABLE, vt);
            let v = |r: &mut crate::difftest::Rng| match r.next_u32() % 3 {
                0 => r.next_u32(),
                _ => r.next_u32() % 2000,
            };
            let (a, b) = (v(&mut r), v(&mut r));
            c.bus.write32(SELF + WPIGD_VALUE, a);
            c.bus.write32(TARGET + 4, b);
        });
    }

    /// RiderWipeoutState_CastGroundRay: no contact (empty collision result),
    /// one or a few template contacts (fake CollideVolume) with and
    /// without a surface, with and without an object, and objects with and
    /// without a contact handler (a fake). Real CollSegment_ctor and
    /// FindClosestContact on the game image.
    #[test]
    fn rider_wipeout_state_cast_ground_ray_matches() {
        use crate::difftest::fakes::vtable;
        use crate::game::ai::computer::tests::{install_fake_slots, FCC_CTRL, FCC_MAGIC};
        const SELF: u32 = DATA + 0x30_0000;
        const RIDER: u32 = SELF + 0x100;
        const HIT: u32 = SELF + 0x1000;
        const DIR: u32 = SELF + 0x1100;
        const TEMPLATE: u32 = SELF + 0x2000;
        const OBJECTS: u32 = SELF + 0x4000;
        const SURFACES: u32 = SELF + 0x5000;
        install_fake_slots();
        check(ee::s01::f_00138960__generated, rider_wipeout_state_cast_ground_ray, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x13_8960, 1);
            elf_image(c);
            c.pc = 0x0013_8960;
            c.g[A0][0] = sx(SELF);
            c.g[A1][0] = sx(HIT);
            c.g[A2][0] = sx(DIR);
            put_vec(c, DIR, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
            c.bus.write32(SELF + WO_RIDER, RIDER);
            put_vec(c, RIDER + WO_R_POS, [r.f32(100.0), r.f32(100.0), r.f32(100.0), 0x3f80_0000]);
            c.bus.write32(RIDER + WO_R_OVERLAP, SELF + 0x800);
            c.bus.write32(RIDER + WO_R_OVERLAP_FLAGS, r.next_u32() % 2);
            // The fake CollideVolume's contacts.
            let count = match seed % 4 {
                0 => 0,
                1 => 1,
                _ => 2 + r.next_u32() % 4,
            };
            c.bus.write32(FCC_CTRL, FCC_MAGIC);
            c.bus.write32(FCC_CTRL + 4, count);
            c.bus.write32(FCC_CTRL + 8, TEMPLATE);
            for k in 0..3 {
                let obj = OBJECTS + 0x100 * k;
                c.bus.write32(obj + 0x78, r.next_u32() % 3);
                let handler = if (seed / 4 + k) % 3 == 0 { 0 } else { obj + 0x80 };
                c.bus.write32(obj + 0xc, handler);
                let vt = vtable(c, &mut r, &[0x150]);
                c.bus.write32(obj + 0x80 + 0xc, vt);
                let surf = SURFACES + 0x200 * k;
                c.bus.write32(surf + 0x150, r.next_u32() % 3);
                c.bus.write16(surf + 0xa, r.next_u32() as u16);
            }
            for i in 0..count {
                let h = TEMPLATE + 0x80 * i;
                for w in 0..0x20 {
                    c.bus.write32(h + 4 * w, r.next_u32());
                }
                let dist = [0.0f32, 1.0, 2.0, 3.0, 1.5][r.next_u32() as usize % 5];
                c.bus.write32(h + 0x40, dist.to_bits());
                let object = match r.next_u32() % 3 {
                    0 => 0,
                    _ => OBJECTS + 0x100 * (r.next_u32() % 3),
                };
                c.bus.write32(h + HIT_OBJECT, object);
                let surface = match r.next_u32() % 3 {
                    0 => 0,
                    _ => SURFACES + 0x200 * (r.next_u32() % 3),
                };
                c.bus.write32(h + HIT_SURFACE, surface);
            }
        });
    }

    /// SND_RunChangeCallbacks: counter unchanged, changed with an empty
    /// list, and changed with 1-4 callbacks (fakes).
    #[test]
    fn snd_run_change_callbacks_matches() {
        use crate::difftest::fakes::FAKE;
        const NODES: u32 = DATA + 0x20_0000;
        check(ee::s13::f_003b54e0__generated, snd_run_change_callbacks, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x3b_54e0, 5);
            c.pc = 0x003b_54e0;
            let count = r.next_u32();
            c.bus.write32(SND_GLOBALS + SND_CHANGE_COUNT, count);
            let seen = if seed % 4 == 0 { count } else { r.next_u32() };
            c.bus.write32(SND_SEEN_HI - SND_SEEN_LO, seen);
            let n = (seed / 4) % 5;
            let mut head = 0;
            for i in (0..n).rev() {
                let node = NODES + 0x20 * i;
                c.bus.write32(node, head);
                c.bus.write32(node + 8, FAKE + 0x10 * i);
                c.bus.write32(node + 0xc, r.next_u32());
                head = node;
            }
            c.bus.write32(SND_GLOBALS + SND_CB_LIST, head);
        });
    }

    /// BezierPatchCache_TessUV2: sizes -1..7 (none, a single vertex, grids),
    /// 8-byte aligned and word-aligned rectangles and outputs.
    #[test]
    fn bezier_patch_cache_tess_uv2_matches() {
        use crate::difftest::{scramble, Rng};
        check(ee::s12::f_00374440__generated, bezier_patch_cache_tess_uv2, 300, &|c, seed| {
            let mut r = Rng(0x37_4440 ^ (seed as u64) << 20);
            scramble(c, &mut r);
            c.pc = 0x0037_4440;
            let level = r.next_u32() % 4;
            let this = DATA;
            for k in 0..4 {
                c.bus.write32(this + BPC_TESS_N + 4 * k, r.next_u32());
            }
            let n = (seed % 9) as i32 - 1;
            c.bus.write32(this + BPC_TESS_N + 4 * level, n as u32);
            let rect = DATA + 0x400 + 4 * (seed / 9 % 2);
            for k in 0..4 {
                c.bus.write32(rect + 0x10 + 4 * k, if seed % 7 == 0 { r.edge_f32() } else { r.f32(10.0) });
            }
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(rect);
            c.g[A2][0] = sx(DATA + 0x1000 + 4 * (seed / 18 % 2));
            c.g[A3][0] = level as u64 | (r.next_u32() as u64) << 32;
        });
    }

    /// SNDSTRM_GetVolume: stream ids out of range / negative / unused
    /// (-8), and a stream whose handle is negative, out of the channel
    /// range, on an inactive channel, on a channel playing another handle
    /// (fallback to the start volume) or playing it (the channel volume).
    #[test]
    fn sndstrm_get_volume_matches() {
        use crate::difftest::{scramble, Rng};
        const STREAMS: u32 = DATA + 0x1000;
        const CHANNELS: u32 = DATA + 0x2000;
        const VOLUMES: u32 = DATA + 0x4000;
        check(ee::s13::f_003b7f18__generated, sndstrm_get_volume, 300, &|c, seed| {
            let mut r = Rng(0x3b_7f18 ^ (seed as u64) << 20);
            scramble(c, &mut r);
            c.pc = 0x003b_7f18;
            // Four stream slots, slot 3 unused.
            c.bus.write8(0x0051_0000 - 0x56d2, 4);
            for k in 0..4 {
                let p = if k == 3 { 0 } else { STREAMS + 0x100 * k };
                c.bus.write32(0x0051_0000 - 0x5f60 + 4 * k, p);
            }
            let id = (seed % 7) as i64 - 1; // -1..5
            c.g[A0][0] = id as u64;
            // Six channels.
            let globals = SND_GLOBALS;
            c.bus.write16(globals + 0x17e, 6);
            c.bus.write32(globals + 0x1f0, CHANNELS);
            c.bus.write32(0x0051_0000 - 0x5528, VOLUMES);
            for k in 0..3 {
                let st = STREAMS + 0x100 * k;
                let ch = r.next_u32() % 8;
                let handle = match (seed / 7 + k) % 4 {
                    0 => 0x8000_0000 | ch,
                    1 => (r.next_u32() << 8) | ch,
                    _ => 0x100 * (k + 1) | ch,
                };
                c.bus.write32(st + STRM_HANDLE, handle);
                c.bus.write8(st + STRM_VOLUME, r.next_u32() as u8);
            }
            for ch in 0..8 {
                let cp = CHANNELS + 0x8c * ch;
                c.bus.write8(cp + 0x69, (r.next_u32() % 3 != 0) as u8);
                let k = r.next_u32() % 3;
                c.bus.write32(cp, if r.next_u32() % 4 == 0 { r.next_u32() } else { 0x100 * (k + 1) | ch });
                c.bus.write16(VOLUMES + 0x8c * ch + 0x3a, r.next_u32() as u16);
            }
        });
    }

    /// StreamBuffer_GetStatus: an unknown stream (-8), a stream with no
    /// request (0x124 = 0) and one with a request but no rate (+0x14 = 0),
    /// the lock/unlock hooks fakes.
    #[test]
    fn stream_buffer_get_status_matches() {
        use crate::difftest::fakes::FAKE;
        const SELF: u32 = DATA + 0x20_0000;
        const OUT: u32 = SELF + 0x100;
        const STREAM: u32 = SELF + 0x1000;
        const REQ: u32 = SELF + 0x2000;
        check(ee::s08::f_002aa020__generated, stream_buffer_get_status, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x2a_a020, 2);
            c.pc = 0x002a_a020;
            c.g[A0][0] = sx(SELF);
            c.g[A1][0] = sx(OUT);
            // SND lock / unlock hooks.
            c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
            c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
            // One stream slot.
            c.bus.write8(0x0051_0000 - 0x56d2, 1);
            c.bus.write32(0x0051_0000 - 0x5f60, STREAM);
            let id = if seed % 3 == 0 { 1 + r.next_u32() % 4 } else { 0 };
            c.bus.write32(SELF + SB_STREAM, id);
            c.bus.write32(STREAM + 0x124, if seed % 3 == 1 { 0 } else { r.next_u32() | 1 });
            c.bus.write32(STREAM + 0x11c, REQ);
            c.bus.write32(REQ + 0xc, r.next_u32());
            c.bus.write16(STREAM + 0x14, 0);
            c.bus.write8(0x0051_0000 - 0x5718 + 0x176, r.next_u32() as u8);
        });
    }

    /// StreamArray_UpdateAll: counts -1..4, null and real buffers; each
    /// buffer runs the real StreamBuffer_UpdateVolume on its short paths
    /// (no volume source, or a volume set on an unknown SND stream; the
    /// lock hooks are fakes, the age stays below the stop limit).
    #[test]
    fn stream_array_update_all_matches() {
        use crate::difftest::fakes::FAKE;
        const SELF: u32 = DATA + 0x20_0000;
        const ARRAY: u32 = SELF + 0x100;
        const BUFFERS: u32 = SELF + 0x1000;
        const VOLUME: u32 = SELF + 0x3000;
        check(ee::s08::f_002aac28__generated, stream_array_update_all, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x2a_ac28, 6);
            c.pc = 0x002a_ac28;
            c.g[A0][0] = sx(SELF);
            c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
            c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
            c.bus.write8(0x0051_0000 - 0x56d2, 0); // no SND streams
            let n = (seed % 6) as i32 - 1;
            c.bus.write32(SELF + SA_COUNT, n as u32);
            c.bus.write32(SELF + SA_BUFFERS, ARRAY);
            c.bus.write32(VOLUME, r.f32(1.0));
            for i in 0..5u32 {
                let b = BUFFERS + 0x100 * i;
                c.bus.write32(ARRAY + 4 * i, if r.next_u32() % 4 == 0 { 0 } else { b });
                c.bus.write32(b + 0x80, r.next_u32() % 121);
                c.bus.write32(b + 0x10, if r.next_u32() % 2 == 0 { 0 } else { VOLUME });
                c.bus.write8(b + 0xc, r.next_u32() as u8);
                c.bus.write32(b + 0x8, r.next_u32() % 4);
            }
        });
    }
}
