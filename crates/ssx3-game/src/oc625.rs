//! Staging module oc625 (wave234): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc625`.

#![allow(unused_imports)]

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::{call_ptr, guest, Callee, Flow, Frame};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0013_28b0, rider_handplant_sub_enter, "RiderHandplantSub_Enter"),
    (0x0013_29b0, rider_handplant_control_init, "RiderHandplantSub_Init"),
    (
        0x0013_2fb8,
        rider_handplant_sub_play_anim,
        "RiderHandplantSub_PlayAnim",
    ),
    (
        0x0013_6268,
        rider_uber_grind_sub_enter,
        "RiderUberGrindSub_Enter",
    ),
    (
        0x0013_9548,
        rider_handplant_state_update_velocity,
        "RiderHandplantState_UpdateVelocity",
    ),
];

/// Shared callee addresses (generated bodies, reached through the table).
const CALL_BX_AUDIO: u32 = 0x0028_b180;
const CALL_HANDPLANT_SOUND: u32 = 0x0029_dc48;
const CALL_ANIM_DURATION: u32 = 0x0031_2790;
const CALL_ANIM_PLAY: u32 = 0x0031_28e8;
const CALL_ANIM_BLEND_OUT_ALL: u32 = 0x0031_1e88;
const CALL_ANIM_CHANGE_HEADING: u32 = 0x0031_1b48;
const CALL_RIDER_UPDATE_ORIENTATION: u32 = 0x0011_e098;

/// `void RiderHandplantSub_Enter(RiderHandplantSub *self, int a1)` (0x1328b0):
/// the handplant's entry. Zero the slew targets of both rider slots (rates
/// 0.05), set the rider's handplant mode to 1, play the hand-plant sound,
/// set the entry animation's frame-rate factor from the animation duration
/// over `self->EnterSec`, play the handplant animation, then blend the
/// rider's animation out on both layers with blend 0.33 and clear
/// `self->CurBalance`.
pub fn rider_handplant_sub_enter(c: &mut Cpu) {
    guest(c, |c| {
        // Offsets of the handplant controller, its rider and the rider animation.
        const SELF_BALANCE: u32 = 0x4;
        const SELF_TYPE: u32 = 0x10;
        const SELF_RIDER: u32 = 0x14;
        const RIDER_SLEW1_RATE: u32 = 0x200;
        const RIDER_SLEW1_TARGET: u32 = 0x204;
        const RIDER_SLEW0_RATE: u32 = 0x1f4;
        const RIDER_SLEW0_TARGET: u32 = 0x1f8;
        const RIDER_MODE: u32 = 0x32c;
        const RIDER_ANIM: u32 = 0x784;
        const ANIM_FRAME_RATE: u32 = 0x1c;
        const SAVED_RA: u32 = 0x0;
        const SAVED_S0: u32 = 0x10;
        const SAVED_F20: u32 = 0x20;

        c.cycles += 63;
        let frame = Frame::open(c, 0x30);
        // f0 = slew rate 0.05, f20 = blend 0.33 (both constants in gp).
        c.lwc1(0, r32(c, GP).wrapping_sub(0x756c));
        frame.save(c, S0, SAVED_S0);
        frame.save_ra(c, SAVED_RA);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 20, SAVED_F20);
        c.g[A0][0] = 1;
        c.lwc1(20, r32(c, GP).wrapping_sub(0x7568));

        // Both slew targets to zero at the slew rate; the mode becomes 1.
        let rider = Addr(r32(c, S0).wrapping_add(SELF_RIDER)).lw(c);
        c.g[V0][0] = rider;
        let a = r32(c, V0).wrapping_add(RIDER_SLEW1_RATE);
        c.swc1(0, a);
        let a = r32(c, V0).wrapping_add(RIDER_SLEW1_TARGET);
        Addr(a).sw(c, 0);
        let rider = Addr(r32(c, S0).wrapping_add(SELF_RIDER)).lw(c);
        c.g[V1][0] = rider;
        let a = r32(c, V1).wrapping_add(RIDER_SLEW0_RATE);
        c.swc1(0, a);
        let a = r32(c, V1).wrapping_add(RIDER_SLEW0_TARGET);
        Addr(a).sw(c, 0);
        let rider = Addr(r32(c, S0).wrapping_add(SELF_RIDER)).lw(c);
        c.g[V0][0] = rider;
        c.g[RA][0] = sx(0x0013_28f4);
        let a = r32(c, V0).wrapping_add(RIDER_MODE);
        let mode_one = c.g[A0][0];
        Addr(a).sw(c, mode_one);
        // The hand-plant sound (bxAudio returns the audio interface).
        call_ptr(c, CALL_BX_AUDIO, 0x0013_28f4)?;
        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(SELF_RIDER)).lw(c);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, CALL_HANDPLANT_SOUND, 0x0013_2900)?;

        // The entry animation's duration for this mode and type.
        c.g[A2][0] = Addr(r32(c, S0).wrapping_add(SELF_RIDER)).lw(c);
        c.g[A1][0] = 0x0046_0000;
        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(SELF_TYPE)).lw(c);
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x6038));
        c.g[V1][0] = Addr(r32(c, A2).wrapping_add(RIDER_MODE)).lw(c);
        c.g[V0][0] = sx((r32(c, V0)) << 3);
        c.g[A0][0] = Addr(r32(c, A2).wrapping_add(RIDER_ANIM)).lw(c);
        c.g[V1][0] = sx((r32(c, V1)) << 4);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
        c.g[A1][0] = Addr(r32(c, V0)).lw(c);
        call_ptr(c, CALL_ANIM_DURATION, 0x0013_2930)?;

        // Frame-rate factor = duration / EnterSec.
        c.lwc1(1, r32(c, S0));
        c.g[A0][0] = c.g[S0][0];
        c.g[V1][0] = Addr(r32(c, S0).wrapping_add(SELF_RIDER)).lw(c);
        c.div_s(0, 0, 1);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(RIDER_ANIM)).lw(c);
        let a = r32(c, V0).wrapping_add(ANIM_FRAME_RATE);
        c.swc1(0, a);

        // The handplant animation for this mode and type.
        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(SELF_RIDER)).lw(c);
        c.g[A2][0] = Addr(r32(c, S0).wrapping_add(SELF_TYPE)).lw(c);
        c.g[A1][0] = Addr(r32(c, V0).wrapping_add(RIDER_MODE)).lw(c);
        Callee {
            addr: 0x0013_2fb8,
            f: ee::s01::f_00132fb8,
        }
        .call(c, 0x0013_2958)?;

        // Back to the rest: frame rate 1.0, balance 0, both layers blended out.
        c.g[V1][0] = Addr(r32(c, S0).wrapping_add(SELF_RIDER)).lw(c);
        c.mov_s(12, 20);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0);
        c.g[A1][0] = 1;
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(RIDER_ANIM)).lw(c);
        let a = r32(c, V0).wrapping_add(ANIM_FRAME_RATE);
        c.swc1(0, a);
        Addr(r32(c, S0).wrapping_add(SELF_BALANCE)).sw(c, 0);
        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(SELF_RIDER)).lw(c);
        c.g[RA][0] = sx(0x0013_2984);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(RIDER_ANIM)).lw(c);
        call_ptr(c, CALL_ANIM_BLEND_OUT_ALL, 0x0013_2984)?;
        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(SELF_RIDER)).lw(c);
        c.mov_s(12, 20);
        c.g[A1][0] = 0;
        c.g[RA][0] = sx(0x0013_2998);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(RIDER_ANIM)).lw(c);
        call_ptr(c, CALL_ANIM_BLEND_OUT_ALL, 0x0013_2998)?;

        frame.restore(c, S0, SAVED_S0);
        frame.restore_ra(c, SAVED_RA);
        frame.restore_fpr(c, 20, SAVED_F20);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderHandplantSub_Init(float enterSec, HandplantControl *self,
/// tIntersectInfo *hit, int flag)` (0x1329b0): store the entry time and
/// the handplant type (`flag` ? 1 : 0); flag a missed handplant when the
/// rider's velocity is faster than 2222.2 (VU0 length); flag a rail when the
/// hit spline has its rail bit (1) set.
pub fn rider_handplant_control_init(c: &mut Cpu) {
    const SELF_MISSED: u32 = 0x8;
    const SELF_RAIL: u32 = 0xc;
    const SELF_TYPE: u32 = 0x10;
    const SELF_RIDER: u32 = 0x14;
    const RIDER_VELOCITY: u32 = 0x1e0;
    const HIT_SPLINE: u32 = 0x58;
    const SPLINE_FLAGS: u32 = 0x1c;

    c.cycles += 22;
    c.g[A3][0] = 1;
    c.g[V0][0] = 0;
    if c.g[A2][0] != 0 {
        c.g[V0][0] = c.g[A3][0];
    }
    let this = r32(c, A0);
    c.swc1(12, this);
    let on = c.g[V0][0];
    Addr(this.wrapping_add(SELF_TYPE)).sw(c, on);
    c.g[A2][0] = 1;
    c.g[V0][0] = Addr(this.wrapping_add(SELF_RIDER)).lw(c);
    c.lwc1(1, r32(c, GP).wrapping_sub(0x7564));
    let velocity = r32(c, V0).wrapping_add(RIDER_VELOCITY);
    c.lqc2(3, velocity);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(3, 22);
    c.mtc1(3, 0);
    c.c_lt_s(1, 0);
    if !c.fcc() {
        c.g[A2][0] = 0;
    }
    c.cycles += 4;
    let missed = c.g[A2][0];
    Addr(this.wrapping_add(SELF_MISSED)).sw(c, missed);
    c.g[A1][0] = Addr(r32(c, A1).wrapping_add(HIT_SPLINE)).lw(c);
    let no_spline = c.g[A1][0] == 0;
    c.g[V1][0] = 0;
    if !no_spline {
        c.cycles += 4;
        c.g[V0][0] = Addr(r32(c, A1).wrapping_add(SPLINE_FLAGS)).lw(c);
        c.g[V1][0] = c.g[A3][0];
        c.g[V0][0] &= 1;
        if c.g[V0][0] == 0 {
            c.g[V1][0] = 0;
        }
    }
    c.cycles += 2;
    let rail = c.g[V1][0];
    Addr(this.wrapping_add(SELF_RAIL)).sw(c, rail);
    ret(c);
}

/// `RiderHandplantSub_PlayAnim(self, mode, type)` (0x132fb8): play the
/// handplant animation of `kHandplantAnim[mode][type]` on the rider. A table
/// entry with the rotate-180 flag first turns the rider's orientation by
/// half a turn about its own up axis (VU0 quaternion product with the
/// rider's matrix row 2), then the animation plays at speed -1.
pub fn rider_handplant_sub_play_anim(c: &mut Cpu) {
    guest(c, |c| {
        const SELF_RIDER: u32 = 0x14;
        const RIDER_ANIM: u32 = 0x784;
        const SAVED_RA: u32 = 0x50;
        const SAVED_S0: u32 = 0x80;
        const SAVED_S1: u32 = 0x70;
        const SAVED_S2: u32 = 0x60;

        c.cycles += 14;
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = 0x0046_0000;
        frame.save(c, S1, SAVED_S1);
        c.g[A2][0] = sx(r32(c, A2) << 3);
        frame.save(c, S2, SAVED_S2);
        c.g[A1][0] = sx(r32(c, A1) << 4);
        frame.save(c, S0, SAVED_S0);
        c.g[S1][0] = sx(r32(c, A2).wrapping_add(r32(c, A1)));
        frame.save_ra(c, SAVED_RA);
        c.g[S2][0] = sx(r32(c, V0).wrapping_sub(0x6038));
        c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, S1)));
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
        let rotate_180 = c.g[V1][0] != 0;
        c.g[S0][0] = c.g[A0][0];
        if rotate_180 {
            handplant_rotate_180(c, frame)?;
        }

        c.cycles += 14;
        c.g[V1][0] = Addr(r32(c, S0).wrapping_add(SELF_RIDER)).lw(c);
        c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, S2)));
        c.g[A1][0] = Addr(r32(c, V0)).lw(c);
        c.g[A2][0] = 0;
        c.g[AT][0] = sx(0xbf80_0000);
        c.mtc1(AT, 12);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(RIDER_ANIM)).lw(c);
        call_ptr(c, CALL_ANIM_PLAY, 0x0013_309c)?;

        frame.restore(c, S0, SAVED_S0);
        frame.restore(c, S1, SAVED_S1);
        frame.restore(c, S2, SAVED_S2);
        frame.restore_ra(c, SAVED_RA);
        frame.close(c);
        Ok(())
    })
}

/// The rotate-180 part of `RiderHandplantSub_PlayAnim`: the rider's matrix
/// row 2 (VU0) times its rotation quaternion, written to the rotation, then
/// `updateOrientationImplicit` and a heading offset of pi on the animation.
fn handplant_rotate_180(c: &mut Cpu, frame: Frame) -> Flow {
    const SELF_RIDER: u32 = 0x14;
    const RIDER_MATRIX_ROW2: u32 = 0x1c0;
    const RIDER_ROTATION: u32 = 0x120;
    const RIDER_ANIM: u32 = 0x784;

    c.cycles += 35;
    c.g[V1][0] = Addr(r32(c, S0).wrapping_add(SELF_RIDER)).lw(c);
    c.lq(2, r32(c, V1).wrapping_add(RIDER_MATRIX_ROW2));
    let zero_word = frame.at(c, 0x1c);
    Addr(zero_word).sw(c, 0);
    let spill = frame.at(c, 0x30);
    c.sq(2, spill);
    c.lwc1(0, frame.at(c, 0x30));
    c.lwc1(1, frame.at(c, 0x34));
    c.lwc1(2, frame.at(c, 0x38));
    c.swc1(0, frame.at(c, 0x10));
    c.swc1(1, frame.at(c, 0x14));
    c.swc1(2, frame.at(c, 0x18));
    c.swc1(0, frame.at(c, 0x20));
    c.swc1(1, frame.at(c, 0x24));
    c.swc1(2, frame.at(c, 0x28));
    let scratch = frame.at(c, 0x10);
    c.lqc2(4, scratch);
    let rotation = r32(c, V1).wrapping_add(RIDER_ROTATION);
    c.lqc2(5, rotation);
    c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().mula(0b1110, 4, Src::Bc(5, W));
    c.vu0().madda(0b1110, 5, Src::Bc(4, W));
    c.vu0().suba(0b0001, 7, Src::Bc(7, X));
    c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
    c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
    c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
    let quad_spill = frame.at(c, 0x40);
    c.sqc2(8, quad_spill);
    c.lq(2, quad_spill);
    c.g[V1][0] = Addr(r32(c, S0).wrapping_add(SELF_RIDER)).lw(c);
    c.sq(2, frame.at(c, 0));
    let rotation = r32(c, V1).wrapping_add(RIDER_ROTATION);
    c.sq(2, rotation);
    c.g[A0][0] = Addr(r32(c, S0).wrapping_add(SELF_RIDER)).lw(c);
    call_ptr(c, CALL_RIDER_UPDATE_ORIENTATION, 0x0013_306c)?;
    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(SELF_RIDER)).lw(c);
    c.lwc1(12, r32(c, GP).wrapping_sub(0x7548));
    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(RIDER_ANIM)).lw(c);
    call_ptr(c, CALL_ANIM_CHANGE_HEADING, 0x0013_307c)?;
    Ok(())
}

/// `RiderHandplantState_UpdateVelocity(state)` (0x139548): the exit velocity
/// of a handplant. Returns 1 when the state's exit-velocity seed vector
/// (+0x10..+0x1c) differs from the zero vector (nothing to do), else 0. For
/// the handplant modes 3..6 it then builds the exit velocity from the spline
/// direction and the approach direction (VU0), stores it at the state's
/// +0x20 and returns 1; other modes return 0. A leaf: VU0 and FPU only.
pub fn rider_handplant_state_update_velocity(c: &mut Cpu) {
    'b_1395b4: {
        'b_1395b0: {
            c.cycles += 9;
            c.g[V0][0] = 0x0050_0000;
            c.lwc1(0, r32(c, A0).wrapping_add(0x10));
            c.lwc1(1, r32(c, V0).wrapping_sub(0xee0));
            c.g[V1][0] = sx(r32(c, V0).wrapping_sub(0xee0));
            c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x50));
            c.c_eq_s(0, 1);
            let t = !c.fcc();
            c.g[V0][0] = 0;
            if t {
                break 'b_1395b0;
            }
            c.cycles += 6;
            c.lwc1(1, r32(c, A0).wrapping_add(0x14));
            c.lwc1(0, r32(c, V1).wrapping_add(0x4));
            c.c_eq_s(1, 0);
            if !c.fcc() {
                c.g[V0][0] = 1;
                break 'b_1395b4;
            }
            c.cycles += 6;
            c.lwc1(1, r32(c, A0).wrapping_add(0x18));
            c.lwc1(0, r32(c, V1).wrapping_add(0x8));
            c.c_eq_s(1, 0);
            if !c.fcc() {
                c.g[V0][0] = 1;
                break 'b_1395b4;
            }
            c.cycles += 5;
            c.lwc1(1, r32(c, V1).wrapping_add(0xc));
            c.lwc1(0, r32(c, A0).wrapping_add(0x1c));
            c.c_eq_s(0, 1);
            if c.fcc() {
                break 'b_1395b4;
            }
            break 'b_1395b0;
        }
        c.cycles += 1;
        c.g[V0][0] = 1;
        break 'b_1395b4;
    }

    'b_139968: {
        c.cycles += 2;
        let t = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if t {
            break 'b_139968;
        }
        'b_139964: {
            // Mode 3: the sideways exit velocity along the spline.
            c.cycles += 5;
            c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0xa0)).lw(c);
            c.g[V1][0] = 3;
            c.g[A1][0] = Addr(r32(c, V0).wrapping_add(0x32c)).lw(c);
            let t = c.g[A1][0] != c.g[V1][0];
            c.g[V0][0] = 4;
            if !t {
                uv_mode3(c);
                c.cycles += 3;
                c.mul_s(0, 0, 5);
                c.g[V0][0] = 1;
                break 'b_139964;
            }
            // Mode 4: the spline-normal exit velocity (5 units, 15 on rails).
            c.cycles += 2;
            let t = c.g[A1][0] != c.g[V0][0];
            c.g[V0][0] = 5;
            if !t {
                uv_sign_probe(c, false);
                uv_sign_branch(c);
                uv_speed_scale(c, 0x71e8);
                c.cycles += 3;
                c.mul_s(0, 1, 2);
                c.g[V0][0] = 1;
                break 'b_139964;
            }
            // Mode 5: the same shape as mode 4 with the mirrored sign probe.
            c.cycles += 2;
            let t = c.g[A1][0] != c.g[V0][0];
            c.g[V0][0] = 6;
            if !t {
                uv_sign_probe(c, true);
                uv_sign_branch(c);
                uv_speed_scale(c, 0x71e4);
                c.cycles += 3;
                c.mul_s(0, 1, 2);
                c.g[V0][0] = 1;
                break 'b_139964;
            }
            // Mode 6: the along-spline velocity at 833.3 per tick.
            c.cycles += 2;
            let t = c.g[A1][0] != c.g[V0][0];
            c.g[V0][0] = 0;
            if t {
                break 'b_139968;
            }
            c.cycles += 13;
            c.lwc1(0, r32(c, GP).wrapping_sub(0x71e0));
            c.g[V0][0] = 1;
            c.mfc1(3, 0);
            c.lqc2(4, r32(c, A0).wrapping_add(0x20));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            let spill = uv_sp(c, 0x10);
            c.sqc2(5, spill);
            c.lwc1(1, r32(c, GP).wrapping_sub(0x71dc));
            c.lq(3, spill);
            let state_seed = r32(c, A0).wrapping_add(0x10);
            c.sq(3, state_seed);
            let frame_seed = uv_sp(c, 0);
            c.sq(3, frame_seed);
            c.lwc1(0, r32(c, A0).wrapping_add(0x18));
            c.add_s(0, 0, 1);
            break 'b_139964;
        }
        c.cycles += 1;
        let z = r32(c, A0).wrapping_add(0x18);
        c.swc1(0, z);
        break 'b_139968;
    }
    c.cycles += 2;
    c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x50));
    ret(c);
}

/// `sp + off` (the 0x50-byte frame of the velocity function, read at each use).
fn uv_sp(c: &Cpu, off: u32) -> u32 {
    r32(c, SP).wrapping_add(off)
}

/// Mode 3 of `RiderHandplantState_UpdateVelocity`: the exit velocity along
/// the spline direction (VU0 cross and normalise with the approach
/// direction), the sideways part from the spline's rail flag (2 or 15 units),
/// stored at the state's +0x20 (x, y, z) with the z speed from the
/// vertical constant.
fn uv_mode3(c: &mut Cpu) {
    c.cycles += 29;
    c.g[V0][0] = 0x0050_0000;
    c.lwc1(1, r32(c, GP).wrapping_sub(0x71f0));
    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xea0));
    c.g[AT][0] = 0x4000_0000;
    c.mtc1(AT, 4);
    c.lqc2(4, r32(c, V0));
    c.lqc2(5, r32(c, A0).wrapping_add(0x20));
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
    let s20 = uv_sp(c, 0x20);
    c.sqc2(6, s20);
    c.lq(2, s20);
    let s10 = uv_sp(c, 0x10);
    c.sq(2, s10);
    c.lqc2(3, s10);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().rsqrt(0, 3, 4, 0);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 5, 3, Src::Q);
    c.sqc2(5, s20);
    c.lq(2, s20);
    c.lwc1(0, r32(c, A0).wrapping_add(0x8c));
    let s0 = uv_sp(c, 0);
    c.g[V1][0] = Addr(r32(c, A0).wrapping_add(0x90)).lw(c);
    c.mul_s(2, 0, 1);
    let t = c.g[V1][0] == 0;
    c.sq(2, s0);
    if !t {
        c.cycles += 2;
        c.g[AT][0] = 0x4170_0000;
        c.mtc1(AT, 4);
    }
    // The sideways part: the spline's direction, scaled, to the state.
    c.cycles += 15;
    c.mtc1(0, 3);
    c.lqc2(3, r32(c, A0).wrapping_add(0x40));
    let s0_again = uv_sp(c, 0);
    c.lqc2(5, s0_again);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(2, 4);
    c.mtc1(2, 0);
    c.lwc1(5, r32(c, GP).wrapping_sub(0x71ec));
    c.c_lt_s(0, 3);
    c.mul_s(1, 4, 5);
    if c.fcc() {
        c.cycles += 1;
        c.neg_s(1, 1);
    }
    c.cycles += 24;
    c.mfc1(2, 2);
    c.lqc2(4, r32(c, A0).wrapping_add(0x20));
    c.qmtc2(2, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    let s30 = uv_sp(c, 0x30);
    c.sqc2(5, s30);
    c.mov_s(0, 3);
    c.lq(2, s30);
    let s20b = uv_sp(c, 0x20);
    c.sq(2, s20b);
    c.mfc1(3, 1);
    let s0c = uv_sp(c, 0);
    c.lqc2(4, s0c);
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    let s40 = uv_sp(c, 0x40);
    c.sqc2(5, s40);
    c.lq(2, s40);
    let s30b = uv_sp(c, 0x30);
    c.sq(2, s30b);
    let s20c = uv_sp(c, 0x20);
    c.lqc2(3, s20c);
    c.lqc2(4, s30b);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, s40);
    c.lq(2, s40);
    c.g[V1][0] = Addr(r32(c, A0).wrapping_add(0x90)).lw(c);
    let state_scratch = r32(c, A0).wrapping_add(0x10);
    c.sq(2, state_scratch);
    let t = c.g[V1][0] == 0;
    let s10b = uv_sp(c, 0x10);
    c.sq(2, s10b);
    if !t {
        c.cycles += 2;
        c.g[AT][0] = 0x4120_0000;
        c.mtc1(1, 0);
    }
}

/// The sign probe of modes 4 and 5: the exit direction against the approach
/// (VU0 dot product with the z axis, 36 cycles). `swapped` flips the compare
/// order (mode 5). Leaves the sign bit in the FPU condition and the
/// approach-scaled vector in the state's +0x10 quad.
fn uv_sign_probe(c: &mut Cpu, swapped: bool) {
    c.cycles += 36;
    c.g[V0][0] = 0x0050_0000;
    c.mtc1(0, 1);
    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xea0));
    let approach = r32(c, A0).wrapping_add(0x20);
    c.lqc2(4, approach);
    let z_axis = r32(c, V0);
    c.lqc2(5, z_axis);
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
    let s20 = uv_sp(c, 0x20);
    c.sqc2(6, s20);
    c.lq(2, s20);
    let s10 = uv_sp(c, 0x10);
    c.sq(2, s10);
    c.lqc2(3, s10);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().rsqrt(0, 3, 4, 0);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 5, 3, Src::Q);
    c.sqc2(5, s20);
    c.lq(2, s20);
    let s0 = uv_sp(c, 0);
    c.sq(2, s0);
    c.lqc2(3, s0);
    let spline_side = r32(c, A0).wrapping_add(0x40);
    c.lqc2(5, spline_side);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(3, 4);
    c.mtc1(3, 0);
    if swapped {
        c.c_lt_s(1, 0);
    } else {
        c.c_lt_s(0, 1);
    }
    if !c.fcc() {
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x90)).lw(c);
    } else {
        c.cycles += 8;
        c.g[AT][0] = sx(0xbf80_0000);
        c.mtc1(1, 0);
        c.mfc1(2, 0);
        let s0b = uv_sp(c, 0);
        c.lqc2(4, s0b);
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        let s0c = uv_sp(c, 0);
        c.sqc2(5, s0c);
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x90)).lw(c);
    }
}

/// The zero-or-one branch that follows the sign probe in modes 4 and 5: the
/// speed sign (-1 or +1 scale), 4 cycles, 2 more for the positive case.
fn uv_sign_branch(c: &mut Cpu) {
    c.cycles += 4;
    c.g[AT][0] = 0x40a0_0000;
    c.mtc1(1, 0);
    let t = c.g[V0][0] == 0;
    if !t {
        c.cycles += 2;
        c.g[AT][0] = 0x4170_0000;
        c.mtc1(1, 0);
    }
}

/// The speed scale that ends modes 4 and 5: f0 times the gp speed constant
/// (`speed_gp`), spilled to the state's vector and the frame; the state's
/// +0x90 selects a 5-unit or 15-unit lateral scale for the next tick.
fn uv_speed_scale(c: &mut Cpu, speed_gp: u32) {
    c.cycles += 12;
    c.lwc1(2, r32(c, GP).wrapping_sub(speed_gp));
    c.mul_s(0, 0, 2);
    c.mfc1(3, 0);
    let s0 = uv_sp(c, 0);
    c.lqc2(4, s0);
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    let s20 = uv_sp(c, 0x20);
    c.sqc2(5, s20);
    c.lq(2, s20);
    c.g[V1][0] = Addr(r32(c, A0).wrapping_add(0x90)).lw(c);
    let state_vec = r32(c, A0).wrapping_add(0x10);
    c.sq(2, state_vec);
    let t = c.g[V1][0] == 0;
    let s10 = uv_sp(c, 0x10);
    c.sq(2, s10);
    if !t {
        c.cycles += 2;
        c.g[AT][0] = 0x40a0_0000;
        c.mtc1(1, 1);
    }
}

/// `RiderUberGrindSub_Enter(self)` (0x136268): start the uber grind. The
/// rider's railslide style picks the entry animation: 4 into `BSIntoAnim`
/// (after a half-turn orientation offset), 3 into `FSIntoAnim`, 2 into
/// `FakieIntoAnim` (clearing the fakie flag and the flipped flag), else the
/// regular `RegIntoAnim` (after the half turn). Then the grind event and the
/// boost flag, the railslide style becomes 3, `self` is cleared, and both
/// animation layers blend out at 0.33.
pub fn rider_uber_grind_sub_enter(c: &mut Cpu) {
    guest(c, |c| {
        const SELF_GRIND: u32 = 0x4;
        const SELF_RIDER: u32 = 0x8;
        const RIDER_RAIL_STYLE: u32 = 0x328;
        const RIDER_ANIM: u32 = 0x784;
        const RIDER_METRIX: u32 = 0x790;
        const SAVED_RA: u32 = 0x30;
        const SAVED_S0: u32 = 0x80;
        const SAVED_S1: u32 = 0x70;
        const SAVED_S2: u32 = 0x60;
        const SAVED_S3: u32 = 0x50;
        const SAVED_S4: u32 = 0x40;
        const SAVED_F20: u32 = 0x90;

        c.cycles += 19;
        let frame = Frame::open(c, 0xa0);
        c.g[A1][0] = 0x24;
        frame.save(c, S3, SAVED_S3);
        c.g[V1][0] = 0x0046_0000;
        frame.save(c, S4, SAVED_S4);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S0, SAVED_S0);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0x5fc8));
        frame.save(c, S1, SAVED_S1);
        c.g[A0][0] = 4;
        frame.save(c, S2, SAVED_S2);
        frame.save_ra(c, SAVED_RA);
        frame.save_fpr(c, 20, SAVED_F20);
        c.g[V0][0] = Addr(r32(c, S3).wrapping_add(SELF_GRIND)).lw(c);
        c.g[A3][0] = Addr(r32(c, S3).wrapping_add(SELF_RIDER)).lw(c);
        c.mult(0, 2, 2, 5);
        c.g[A2][0] = Addr(r32(c, A3).wrapping_add(RIDER_RAIL_STYLE)).lw(c);
        let back_side = c.g[A2][0] == c.g[A0][0];
        c.g[S4][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        if back_side {
            uber_enter_back_side(c)?;
        } else {
            uber_enter_other_style(c)?;
        }

        // The shared tail: the grind event, the boost flag, the railslide mode.
        c.cycles += 34;
        c.g[A1][0] = Addr(r32(c, S4).wrapping_add(0x20)).lw(c);
        c.g[A2][0] = Addr(r32(c, V0).wrapping_add(RIDER_RAIL_STYLE)).lw(c);
        c.lwc1(20, r32(c, GP).wrapping_sub(0x72dc));
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(RIDER_METRIX)).lw(c);
        call_ptr(c, 0x0011_9938, 0x0013_6494)?;
        c.g[A0][0] = Addr(r32(c, S3).wrapping_add(SELF_RIDER)).lw(c);
        c.mov_s(12, 0);
        c.g[A1][0] = 1;
        call_ptr(c, 0x0010_e098, 0x0013_64a4)?;
        c.g[V1][0] = Addr(r32(c, S3).wrapping_add(SELF_RIDER)).lw(c);
        c.g[V0][0] = 3;
        let rail_style = c.g[V0][0];
        Addr(r32(c, V1).wrapping_add(RIDER_RAIL_STYLE)).sw(c, rail_style);
        Addr(r32(c, S3)).sw(c, 0);
        c.g[A0][0] = Addr(r32(c, S3).wrapping_add(SELF_RIDER)).lw(c);
        call_ptr(c, 0x0011_6930, 0x0013_64bc)?;
        c.g[V0][0] = Addr(r32(c, S3).wrapping_add(SELF_RIDER)).lw(c);
        c.mov_s(12, 20);
        c.g[A1][0] = 1;
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(RIDER_ANIM)).lw(c);
        call_ptr(c, 0x0031_1e88, 0x0013_64d0)?;
        c.g[V0][0] = Addr(r32(c, S3).wrapping_add(SELF_RIDER)).lw(c);
        c.mov_s(12, 20);
        c.g[A1][0] = 0;
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(RIDER_ANIM)).lw(c);
        call_ptr(c, 0x0031_1e88, 0x0013_64e4)?;

        frame.restore(c, S0, SAVED_S0);
        frame.restore(c, S1, SAVED_S1);
        frame.restore(c, S2, SAVED_S2);
        frame.restore(c, S3, SAVED_S3);
        frame.restore(c, S4, SAVED_S4);
        frame.restore_ra(c, SAVED_RA);
        frame.restore_fpr(c, 20, SAVED_F20);
        frame.close(c);
        Ok(())
    })
}

/// The rail style 4 entry: the back-side animation `BSIntoAnim` (table +0),
/// after a half turn of the orientation offset.
fn uber_enter_back_side(c: &mut Cpu) -> Flow {
    c.cycles += 31;
    c.g[V0][0] = 0x0050_0000;
    c.g[S2][0] = Addr(r32(c, A3).wrapping_add(0x784)).lw(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xed0));
    c.lwc1(12, r32(c, GP).wrapping_sub(0x72e8));
    c.lq(3, r32(c, V0));
    c.g[S0][0] = 0x0050_0000;
    c.g[A0][0] = sx(uv_sp_of(c, 0x10));
    c.g[A1][0] = sx(uv_sp_of(c, 0x14));
    let s2_origin = r32(c, S2).wrapping_add(0x30);
    c.sq(3, s2_origin);
    c.g[S1][0] = sx(r32(c, S0).wrapping_sub(0xea0));
    call_ptr(c, 0x0031_be50, 0x0013_62e0)?;
    uber_axis_finish(c, 0x10);
    c.g[A1][0] = Addr(r32(c, S4)).lw(c);
    c.cycles += 4;
    uber_play(c, 0x0013_63f8)?;
    Ok(())
}

/// The other rail styles: 3 (frontside), 2 (fakie) and the default, which
/// uses `RegIntoAnim`.
fn uber_enter_other_style(c: &mut Cpu) -> Flow {
    c.cycles += 3;
    c.g[V0][0] = 3;
    let not_frontside = c.g[A2][0] != c.g[V0][0];
    c.g[V0][0] = 2;
    if !not_frontside {
        // Frontside: `FSIntoAnim` (table +4), no orientation offset.
        c.cycles += 8;
        c.g[A0][0] = Addr(r32(c, A3).wrapping_add(0x784)).lw(c);
        c.g[A2][0] = 0;
        c.g[AT][0] = sx(0xbf80_0000);
        c.mtc1(AT, 12);
        c.g[A1][0] = Addr(r32(c, S4).wrapping_add(4)).lw(c);
        call_ptr(c, 0x0031_28e8, 0x0013_6354)?;
        c.g[V0][0] = Addr(r32(c, S3).wrapping_add(8)).lw(c);
        return Ok(());
    }
    c.cycles += 2;
    let not_fakie = c.g[A2][0] != c.g[V0][0];
    c.g[V0][0] = 0x0050_0000;
    if not_fakie {
        // Default: `RegIntoAnim` after the half turn (its own return addresses).
        c.cycles += 32;
        c.g[S2][0] = Addr(r32(c, A3).wrapping_add(0x784)).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xed0));
        c.lwc1(12, r32(c, GP).wrapping_sub(0x72e0));
        c.lq(3, r32(c, V0));
        c.g[S0][0] = 0x0050_0000;
        c.g[A0][0] = sx(uv_sp_of(c, 0x20));
        c.g[A1][0] = sx(uv_sp_of(c, 0x24));
        let s2_origin = r32(c, S2).wrapping_add(0x30);
        c.sq(3, s2_origin);
        c.g[S1][0] = sx(r32(c, S0).wrapping_sub(0xea0));
        call_ptr(c, 0x0031_be50, 0x0013_6428)?;
        uber_axis_finish(c, 0x20);
        c.g[A1][0] = Addr(r32(c, S4).wrapping_add(0xc)).lw(c);
        uber_play(c, 0x0013_647c)?;
        return Ok(());
    }
    // Fakie: `FakieIntoAnim` (table +8), flags cleared, the half turn.
    c.cycles += 35;
    Addr(r32(c, A3).wrapping_add(0x320)).sw(c, 0);
    c.g[A2][0] = 0x0050_0000;
    c.g[A2][0] = sx(r32(c, A2).wrapping_sub(0xed0));
    c.lwc1(12, r32(c, GP).wrapping_sub(0x72e4));
    c.g[V1][0] = Addr(r32(c, S3).wrapping_add(8)).lw(c);
    c.g[S0][0] = 0x0050_0000;
    c.g[A0][0] = sx(uv_sp_of(c, 0x18));
    c.g[A1][0] = sx(uv_sp_of(c, 0x1c));
    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x784)).lw(c);
    c.g[S1][0] = sx(r32(c, S0).wrapping_sub(0xea0));
    Addr(r32(c, V0).wrapping_add(0x18)).sw(c, 0);
    c.g[V1][0] = Addr(r32(c, S3).wrapping_add(8)).lw(c);
    c.lq(2, r32(c, A2));
    c.g[S2][0] = Addr(r32(c, V1).wrapping_add(0x784)).lw(c);
    let s2_origin = r32(c, S2).wrapping_add(0x30);
    c.sq(2, s2_origin);
    call_ptr(c, 0x0031_be50, 0x0013_63a4)?;
    uber_axis_finish(c, 0x18);
    c.g[A1][0] = Addr(r32(c, S4).wrapping_add(8)).lw(c);
    c.cycles += 4;
    uber_play(c, 0x0013_63f8)?;
    Ok(())
}

/// `sp + off` for the uber-grind frame (read at each use).
fn uv_sp_of(c: &Cpu, off: u32) -> u32 {
    r32(c, SP).wrapping_add(off)
}

/// What the entry branches share after the axis-angle call: the orientation
/// quaternion (VU0-free FPU product of the axis with the scalar), stored at the
/// animation's `OrientationOffset.Q` (`S2 + 0x40`), the play speed -1 in f12
/// and `V1` = the rider.
fn uber_axis_finish(c: &mut Cpu, scratch: u32) {
    c.lwc1(1, r32(c, S1).wrapping_add(8));
    c.g[A2][0] = 0;
    c.lwc1(0, uv_sp_of(c, scratch));
    c.lwc1(2, r32(c, S0).wrapping_sub(0xea0));
    c.lwc1(4, r32(c, S1).wrapping_add(4));
    c.mul_s(1, 0, 1);
    c.mul_s(2, 0, 2);
    c.lwc1(3, uv_sp_of(c, scratch.wrapping_add(4)));
    c.mul_s(0, 0, 4);
    c.g[AT][0] = sx(0xbf80_0000);
    c.mtc1(AT, 12);
    c.swc1(3, uv_sp_of(c, 0xc));
    c.swc1(1, uv_sp_of(c, 0x8));
    c.swc1(2, uv_sp_of(c, 0));
    c.swc1(0, uv_sp_of(c, 0x4));
    c.lq(2, uv_sp_of(c, 0));
    let s2_quat = r32(c, S2).wrapping_add(0x40);
    c.sq(2, s2_quat);
    c.g[V1][0] = Addr(r32(c, S3).wrapping_add(8)).lw(c);
}

/// The play call that closes the entry branches (`RiderAnimBase_play` with
/// speed -1, from `V1`'s rider), then `V0` = the rider.
fn uber_play(c: &mut Cpu, ret: u32) -> Flow {
    c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x784)).lw(c);
    call_ptr(c, 0x0031_28e8, ret)?;
    c.g[V0][0] = Addr(r32(c, S3).wrapping_add(8)).lw(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE, LOG_RESULTS};
    use crate::difftest::{check, put_vec, put_words, Rng, DATA, GAME_GP};

    /// `kHandplantAnim` in the game's data (the table the head's constants
    /// describe), read by the handplant tests.
    const HANDPLANT_TABLE_TEST: u32 = 0x0045_9fc8;

    /// A rider and its real animation object: the layer selector is the one
    /// fake (returns the no-layer sentinel), so no animation resource loads.
    fn rider_anim_fixture(c: &mut Cpu, r: &mut Rng, rider: u32) {
        let anim = DATA + 0x5000;
        let vt = DATA + 0x6000;
        let lists = DATA + 0x7000;
        let seq = DATA + 0x7100;
        c.bus.write32(rider + 0x784, anim);
        c.bus.write32(anim + 0x50, lists);
        c.bus.write32(anim + 0x58, vt);
        c.bus.write32(vt + 0x24, FAKE);
        for i in 0..8 {
            c.bus.write32(LOG_RESULTS + i * 4, 0x207);
        }
        for i in 0..4 {
            c.bus.write32(lists + 8 * i + 4, seq);
        }
        c.bus.write32(anim + 8, 9 + r.next_u32() % 4);
        c.bus.write32(seq + 0xc0, r.next_u32() % 2);
        c.bus.write32(seq + 0xb0, 0);
        for id in 9..=13 {
            c.bus.write32(0x0044_6990 + id * 28, 0);
        }
        put_vec(c, rider + 0x120, [0, 0, 0, 1.0f32.to_bits()]);
        put_vec(c, rider + 0x1c0, [0, 0, 1.0f32.to_bits(), 0]);
    }

    #[test]
    fn rider_handplant_sub_enter_matches() {
        check(ee::s01::f_001328b0__generated, rider_handplant_sub_enter, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x1328b0, 0);
            const SELF: u32 = DATA;
            const RIDER: u32 = DATA + 0x400;
            const AUDIO: u32 = DATA + 0x20000;
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = SELF as u64;
            c.g[A1][0] = seed as u64;
            c.pc = 0x0013_28b0;
            put_words(c, GAME_GP - 0x756c, &[0.05f32.to_bits()]);
            put_words(c, GAME_GP - 0x7568, &[0.33f32.to_bits()]);
            put_words(c, GAME_GP - 0x7548, &[(-1.0f32).to_bits()]);
            c.bus.write32(GAME_GP + 0x410, AUDIO);
            c.bus.write32(SELF, (0.5f32 + (seed % 4) as f32).to_bits());
            c.bus.write32(SELF + 4, seed % 2);
            c.bus.write32(SELF + 0x10, seed % 3);
            c.bus.write32(SELF + 0x14, RIDER);
            // Mode 1 (set by the function) row of the handplant table.
            for t in 0..3u32 {
                let row = HANDPLANT_TABLE_TEST + 16 + t * 8;
                c.bus.write32(row, 9 + t);
                c.bus.write32(row + 4, ((seed / 3 + t) % 2) as u32);
            }
            rider_anim_fixture(c, &mut r, RIDER);
        });
    }

    #[test]
    fn rider_handplant_control_init_matches() {
        check(ee::s01::f_001329b0__generated, rider_handplant_control_init, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x1329b0, 0);
            const SELF: u32 = DATA;
            const HIT: u32 = DATA + 0x100;
            const SPLINE: u32 = DATA + 0x200;
            const RIDER: u32 = DATA + 0x400;
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = SELF as u64;
            c.g[A1][0] = HIT as u64;
            c.g[A2][0] = (seed % 2) as u64;
            c.f[12] = r.edge_f32();
            c.pc = 0x0013_29b0;
            put_words(c, GAME_GP - 0x7564, &[2222.2222f32.to_bits()]);
            c.bus.write32(SELF + 0x14, RIDER);
            let speed = if seed % 3 == 0 { 3000.0f32 } else { 100.0f32 };
            put_vec(
                c,
                RIDER + 0x1e0,
                [speed.to_bits(), (speed * 0.5).to_bits(), 0, 0],
            );
            c.bus.write32(HIT + 0x58, if seed % 4 == 0 { 0 } else { SPLINE });
            c.bus.write32(SPLINE + 0x1c, (seed % 2) * 1);
        });
    }

    #[test]
    fn rider_handplant_sub_play_anim_matches() {
        check(ee::s01::f_00132fb8__generated, rider_handplant_sub_play_anim, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x132fb8, 0);
            const SELF: u32 = DATA;
            const RIDER: u32 = DATA + 0x400;
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = SELF as u64;
            let mode = (seed % 5) as u64;
            let ty = (seed / 5 % 3) as u64;
            c.g[A1][0] = mode;
            c.g[A2][0] = ty;
            c.pc = 0x0013_2fb8;
            put_words(c, GAME_GP - 0x7548, &[1.5707963705062866f32.to_bits()]);
            c.bus.write32(SELF + 0x14, RIDER);
            let row = HANDPLANT_TABLE_TEST + (mode as u32) * 16 + (ty as u32) * 8;
            c.bus.write32(row, 9 + (seed % 5));
            c.bus.write32(row + 4, (seed / 2 % 2) as u32);
            put_vec(
                c,
                RIDER + 0x120,
                [r.next_u32(), r.next_u32(), r.next_u32(), r.next_u32()],
            );
            rider_anim_fixture(c, &mut r, RIDER);
        });
    }

    #[test]
    fn rider_uber_grind_sub_enter_matches() {
        check(ee::s01::f_00136268__generated, rider_uber_grind_sub_enter, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x136268, 0);
            const SELF: u32 = DATA;
            const RIDER: u32 = DATA + 0x400;
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = SELF as u64;
            c.pc = 0x0013_6268;
            let kind = seed % 4;
            c.bus.write32(SELF + 4, kind);
            c.bus.write32(SELF + 8, RIDER);
            let style = [0u32, 2, 3, 4, 5][(seed / 4 % 5) as usize];
            c.bus.write32(RIDER + 0x328, style);
            let uber_table = 0x0045_a038 + 0x24 * kind;
            for (off, anim) in [(0x0, 9), (0x4, 10), (0x8, 11), (0xc, 12), (0x20, 13)] {
                c.bus.write32(uber_table + off, anim);
            }
            for off in [0x72e8, 0x72e4, 0x72e0] {
                put_words(c, GAME_GP - off, &[1.5707963705062866f32.to_bits()]);
            }
            put_words(c, GAME_GP - 0x72dc, &[0.33f32.to_bits()]);
            put_vec(c, 0x004f_f130, [0, 0, 0, 1.0f32.to_bits()]);
            put_vec(c, 0x004f_f160, [0, 0, 1.0f32.to_bits(), 0]);
            rider_anim_fixture(c, &mut r, RIDER);
        });
    }

    #[test]
    fn rider_handplant_state_update_velocity_matches() {
        check(
            ee::s01::f_00139548__generated,
            rider_handplant_state_update_velocity,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x139548, 0);
                const STATE: u32 = DATA + 0x100;
                const RIDER: u32 = DATA + 0x300;
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = STATE as u64;
                c.pc = 0x0013_9548;
                // The zero vector (the seed vector's compare) and the Z axis.
                put_vec(c, 0x004f_f120, [0, 0, 0, 0]);
                put_vec(c, 0x004f_f160, [0, 0, 1.0f32.to_bits(), 0]);
                for off in [0x71f0, 0x71ec, 0x71e8, 0x71e4, 0x71e0, 0x71dc] {
                    put_words(c, GAME_GP - off, &[r.edge_f32()]);
                }
                let mode = [3u32, 4, 5, 6, 7][(seed / 4 % 5) as usize];
                c.bus.write32(STATE + 0xa0, RIDER);
                c.bus.write32(RIDER + 0x32c, mode);
                let seed_vec = if seed % 4 == 0 {
                    [0, 0, 0, 0]
                } else {
                    [1.0f32.to_bits(), 0, 0, 0]
                };
                put_vec(c, STATE + 0x10, seed_vec);
                put_vec(
                    c,
                    STATE + 0x20,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                put_vec(
                    c,
                    STATE + 0x40,
                    [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                );
                c.bus.write32(STATE + 0x8c, r.edge_f32());
                c.bus.write32(STATE + 0x90, (seed % 3 != 0) as u32);
            },
        );
    }
}
