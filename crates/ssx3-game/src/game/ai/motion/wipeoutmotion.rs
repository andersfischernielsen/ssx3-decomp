//! `src/ai/motion/wipeoutmotion.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
pub(crate) use crate::game::ai::ai::save_state_write;
pub(crate) use crate::game::ai::ai::virtual_call_slot;
pub(crate) use crate::game::seg::seg_1fbe38::gp_oc322;
pub(crate) use crate::game::wscript::wscriptman::frame_at;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw;
pub(crate) use crate::game::wscript::wscriptman::p;
pub(crate) use crate::game::wscript::wscriptman::sw;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0013_6d40,
        rider_wipeout_state_enable_root_pose_override,
        "RiderWipeoutState_EnableRootPoseOverride",
    ),
    (
        0x0013_6f28,
        rider_wipeout_state_exit,
        "RiderWipeoutState_Exit",
    ),
    (
        0x0013_8b10,
        rider_wipeout_state_load_state,
        "RiderWipeoutState_LoadState",
    ),
    (
        0x0013_6f30,
        rider_wipeout_state_update_root_pose,
        "RiderWipeoutState_UpdateRootPose",
    ),
    (
        0x0013_8960,
        rider_wipeout_state_cast_ground_ray,
        "RiderWipeoutState_CastGroundRay",
    ),
    (
        0x0013_7860,
        rider_wipeout_state_post_update_air,
        "RiderWipeoutState_PostUpdateAir",
    ),
    (
        0x0013_7550,
        rider_wipeout_state_build_board_spheres,
        "RiderWipeoutState_BuildBoardSpheres",
    ),
    (
        0x0013_7750,
        rider_wipeout_state_update_air,
        "RiderWipeoutState_UpdateAir",
    ),
    (
        0x0013_8ad8,
        rider_wipeout_state_save_state,
        "RiderWipeoutState_SaveState",
    ),
    (
        0x0013_6c40,
        rider_wipeout_state_enter,
        "RiderWipeoutState_Enter",
    ),
    (
        0x0013_6de0,
        rider_wipeout_state_set_velocity,
        "RiderWipeoutState_SetVelocity",
    ),
    (
        0x0013_7138,
        rider_wipeout_state_post_update_root_pose,
        "RiderWipeoutState_PostUpdateRootPose",
    ),
    (
        0x0013_7d18,
        rider_wipeout_state_update_ground,
        "RiderWipeoutState_UpdateGround",
    ),
    (
        0x0013_8640,
        rider_wipeout_state_post_update_ground,
        "RiderWipeoutState_PostUpdateGround",
    ),
];

/// `RiderWipeoutState_LoadState(self, iface)` (0x138b10): `0x40`.
pub fn rider_wipeout_state_load_state(c: &mut Cpu) {
    guest(c, |c| virtual_call_slot(c, 0x0013_8b38, Some(0x40)))
}

/// `void RiderWipeoutState_Exit(RiderWipeoutState* s)` (0x136f28): nothing to
/// tear down.
pub fn rider_wipeout_state_exit(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}

/// `RiderWipeoutState_BuildBoardSpheres(state)` (0x137550): four collision
/// spheres along the board bone's scaled axis. The 70/30 offsets are lengths.
pub fn rider_wipeout_state_build_board_spheres(c: &mut Cpu) {
    guest(c, |c| {
        // Fields from the matching BuildBoardSpheres C++ reference (0x137550).
        const SPHERE_TREE: u32 = 0x44;
        const MODEL: u32 = 0x780;
        const BOARD_BONE_INDEX: u32 = 0x8a4;
        const BONES: u32 = 0x2c;
        const MODEL_SCALE: u32 = 0x140;
        const BONE_ROTATION: u32 = 0x10;
        // Stack vectors are reused exactly as in the compiled body.
        const POINTS: u32 = 0;
        const RADII: u32 = 0x40;
        const SCALED_AXIS: u32 = 0x50;
        const AXIS_OR_POINT: u32 = 0x60;
        const OFFSET: u32 = 0x70;
        const VECTOR_RESULT: u32 = 0x80;
        const SAVED_RA: u32 = 0x90;
        c.cycles += 127;
        let frame = Frame::open(c, 0xa0);
        c.g[T1][0] = c.g[A0][0];
        frame.save_ra(c, SAVED_RA);
        c.g[V0][0] = 3;
        c.g[A3][0] = sx(frame.at(c, RADII));
        c.g[V1][0] = u64::MAX;
        c.g[T0][0] = Addr(r32(c, T1)).off(WIPEOUT_STATE_RIDER).lw(c);
        c.g[V0][0] = 0x460000;
        // 0x458220: the four board-sphere radii.
        c.g[A2][0] = sx(r32(c, V0).wrapping_sub(0x7de0));
        // Preserve the unaligned copy and its register effects.
        c.ldl(V1, r32(c, A2).wrapping_add(7));
        c.ldr(V1, r32(c, A2));
        c.ldl(A0, r32(c, A2).wrapping_add(15));
        c.ldr(A0, r32(c, A2).wrapping_add(8));
        c.sdl(V1, frame.at(c, RADII + 7));
        c.sdr(V1, frame.at(c, RADII));
        c.sdl(A0, frame.at(c, RADII + 15));
        c.sdr(A0, frame.at(c, RADII + 8));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 9);
        c.g[A1][0] = 4;
        c.g[V1][0] = Addr(r32(c, T0)).off(MODEL).lw(c);
        c.g[A2][0] = c.g[SP][0];
        c.g[V0][0] = Addr(r32(c, T0)).off(BOARD_BONE_INDEX).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).off(BONES).lw(c);
        c.g[V0][0] = sx(r32(c, V0) << 5);
        c.lwc1(11, r32(c, V1).wrapping_add(MODEL_SCALE));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
        c.g[AT][0] = 0x428c_0000;
        c.mtc1(AT, 10);
        // Quaternion's first rotation axis, with the original FPU ordering.
        c.lwc1(5, r32(c, A0).wrapping_add(BONE_ROTATION + 4));
        c.g[T0][0] = c.g[A0][0];
        c.lwc1(4, r32(c, A0).wrapping_add(BONE_ROTATION + 8));
        c.lwc1(0, r32(c, A0).wrapping_add(BONE_ROTATION));
        c.mul_s(3, 5, 5);
        c.lwc1(2, r32(c, A0).wrapping_add(BONE_ROTATION + 12));
        c.mul_s(6, 4, 4);
        c.mul_s(1, 0, 4);
        c.sw(frame.at(c, AXIS_OR_POINT + 12), 0);
        c.mul_s(7, 2, 5);
        c.g[AT][0] = 0x41f0_0000;
        c.mtc1(AT, 8);
        c.add_s(3, 3, 6);
        c.g[AT][0] = 0x42b4_0000;
        c.mtc1(AT, 13);
        c.mul_s(0, 0, 5);
        c.mul_s(2, 2, 4);
        c.sub_s(1, 1, 7);
        c.add_s(3, 3, 3);
        c.add_s(0, 0, 2);
        c.add_s(1, 1, 1);
        c.sub_s(9, 9, 3);
        c.add_s(0, 0, 0);
        c.swc1(1, frame.at(c, AXIS_OR_POINT + 8));
        c.swc1(9, frame.at(c, AXIS_OR_POINT));
        c.swc1(0, frame.at(c, AXIS_OR_POINT + 4));
        c.mfc1(V1, 11);
        c.lqc2(4, frame.at(c, AXIS_OR_POINT));
        c.qmtc2(V1, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame.at(c, OFFSET));
        c.lq(V0, frame.at(c, OFFSET));
        c.sq(V0, frame.at(c, SCALED_AXIS));
        // +70, +30, -30, -70 along that axis. Keep scratch stores intact.
        for (sphere, scale_register) in [10, 8, 8, 10].into_iter().enumerate() {
            c.mfc1(V1, scale_register);
            c.lqc2(4, frame.at(c, SCALED_AXIS));
            c.qmtc2(V1, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, frame.at(c, VECTOR_RESULT));
            c.lq(V0, frame.at(c, VECTOR_RESULT));
            c.sq(V0, frame.at(c, OFFSET));
            c.lqc2(3, r32(c, A0));
            c.lqc2(4, frame.at(c, OFFSET));
            if sphere < 2 {
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            } else {
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            }
            c.sqc2(5, frame.at(c, VECTOR_RESULT));
            c.lq(V0, frame.at(c, VECTOR_RESULT));
            if sphere == 3 {
                c.g[V1][0] = Addr(r32(c, T1)).off(WIPEOUT_STATE_RIDER).lw(c);
            }
            if sphere < 2 {
                c.sq(V0, frame.at(c, AXIS_OR_POINT));
                c.sq(V0, frame.at(c, POINTS + 0x10 * sphere as u32));
            } else {
                c.sq(V0, frame.at(c, POINTS + 0x10 * sphere as u32));
                c.sq(V0, frame.at(c, AXIS_OR_POINT));
            }
        }
        c.g[V0][0] = Addr(r32(c, V1)).off(MODEL).lw(c);
        c.g[A0][0] = Addr(r32(c, T1)).off(SPHERE_TREE).lw(c);
        c.lwc1(12, r32(c, V0).wrapping_add(MODEL_SCALE));
        c.mul_s(12, 12, 13);
        call_ptr(c, 0x0032_9a90, 0x0013_7740)?;
        frame.restore_ra(c, SAVED_RA);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider wipeout.

/// `void RiderWipeoutState_EnableRootPoseOverride(Rider* rider, Vec4* pose,
/// Vec4* offset, Vec4* root)` (0x136d40): store the pose/offset/root and mark
/// the model's root pose overridden.
pub fn rider_wipeout_state_enable_root_pose_override(c: &mut Cpu) {
    c.cycles += 16;
    c.g[T1][0] = c.g[A0][0];
    c.lq(3, c.g[A3][0] as u32);
    c.g[A0][0] = lw(c, p(c.g[T1][0] as u32, 0x40));
    c.g[T0][0] = 1;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
    c.sq(3, p(c.g[A0][0] as u32, 0x130));
    c.lq(2, p(c.g[A3][0] as u32, 0x10));
    c.sq(2, p(c.g[A0][0] as u32, 0x140));
    c.g[V1][0] = lw(c, p(c.g[T1][0] as u32, 0x40));
    sw(c, p(c.g[V1][0] as u32, 0x150), c.g[T0][0]);
    c.lq(4, c.g[A2][0] as u32);
    c.g[V1][0] = lw(c, c.g[T1][0] as u32);
    c.sq(4, p(c.g[T1][0] as u32, 0x20));
    c.lq(2, c.g[A1][0] as u32);
    let no_override = c.g[V1][0] != 0;
    c.sq(2, p(c.g[T1][0] as u32, 0x10));
    if !no_override {
        c.cycles += 21;
        c.g[V0][0] = lw(c, p(c.g[T1][0] as u32, 0x40));
        c.lqc2(3, p(c.g[V0][0] as u32, 0x370));
        c.lqc2(5, c.g[A1][0] as u32);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(3, 4);
        c.mtc1(3, 0);
        c.mfc1(4, 0);
        c.lqc2(4, p(c.g[V0][0] as u32, 0x370));
        c.qmtc2(4, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, frame_at(c, 0x10));
        c.lq(3, frame_at(c, 0x10));
        c.sq(3, frame_at(c, 0));
        c.lqc2(3, p(c.g[T1][0] as u32, 0x10));
        c.lqc2(4, frame_at(c, 0));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, p(c.g[T1][0] as u32, 0x10));
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
    c.pc = t;
}

pub(crate) mod wipeout_pose {

    /// Wipeout state: tumble velocity (vec4) at +0x10, angular velocity at
    /// +0x20, the rider at +0x40.
    pub const VEL: u32 = 0x10;

    pub const ANG_VEL: u32 = 0x20;

    pub const RIDER: u32 = 0x40;

    /// Rider: root position (vec4) at +0x130, root rotation quaternion at
    /// +0x140, active flag at +0x150, time scale at +0x300.
    pub const R_POS: u32 = 0x130;

    pub const R_QUAT: u32 = 0x140;

    pub const R_ACTIVE: u32 = 0x150;

    pub const R_RATE: u32 = 0x300;

    /// `.lit4`: 1/60 (seconds per tick), -0.2 (horizontal drag), 3333.33
    /// (top tumble speed).
    pub const LIT_TICK: u32 = 0x72b8;

    pub const LIT_DRAG: u32 = 0x72b4;

    pub const LIT_MAX_SPEED: u32 = 0x72b0;
}

/// `void RiderWipeoutState_UpdateRootPose(RiderWipeoutState* self)`
/// (0x136f30): integrate the tumbling rider's root while wiped out (if the
/// rider is active), with `dt = rate / 60`: position += vel * dt; vel +=
/// (vel.x * -0.2, vel.y * -0.2, -1800, 0) * dt (drag and gravity), clamped
/// to length 3333.33; angular velocity *= 1 - dt / 2; the root quaternion
/// += its derivative from the angular velocity (VU0 quaternion product) *
/// dt, then renormalised (`vrsqrt`). VU0 macro ops as in the original; the
/// vector temporaries live on the stack.
pub fn rider_wipeout_state_update_root_pose(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
    use wipeout_pose::*;
    let gp = r32(c, GP);
    c.cycles += 5;
    c.g[A1][0] = c.g[A0][0];
    let me = r32(c, A1);
    c.g[V1][0] = c.lw(me + RIDER);
    c.g[V0][0] = c.lw(r32(c, V1) + R_ACTIVE);
    let active = c.g[V0][0] != 0;
    let frame = Frame::open(c, 0x30);
    let s = sp(c);
    if active {
        // dt (f4) = rate / 60; position += vel * dt.
        c.cycles += 51;
        c.lwc1(1, r32(c, V1) + R_RATE);
        c.lwc1(0, gp.wrapping_sub(LIT_TICK));
        c.lwc1(2, gp.wrapping_sub(LIT_DRAG));
        c.mul_s(4, 1, 0);
        c.g[AT][0] = 0xffffffffc4e10000;
        c.mtc1(1, 3);
        c.mfc1(2, 4);
        c.lqc2(4, me + VEL);
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, s + 0x10);
        c.lwc1(5, gp.wrapping_sub(LIT_MAX_SPEED));
        c.lq(2, s + 0x10);
        c.g[V1][0] = c.lw(me + RIDER);
        c.sq(2, s);
        c.lqc2(3, r32(c, V1) + R_POS);
        c.lqc2(4, s);
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, r32(c, V1) + R_POS);
        c.lwc1(0, me + VEL);
        c.lwc1(1, me + VEL + 4);
        c.mul_s(0, 0, 2);
        c.swc1(3, s + 0x18);
        c.mul_s(1, 1, 2);
        c.sw(s + 0x1c, 0u64);
        c.swc1(0, s + 0x10);
        c.swc1(1, s + 0x14);
        c.mfc1(3, 4);
        c.lqc2(4, s + 0x10);
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, s + 0x20);
        c.lq(2, s + 0x20);
        c.sq(2, s);
        c.lqc2(3, me + VEL);
        c.lqc2(4, s);
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, me + VEL);
        c.lqc2(3, me + VEL);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(2, 22);
        c.mtc1(2, 0);
        c.c_lt_s(5, 0);
        if c.fcc() {
            // Faster than the top speed: vel *= max / |vel|.
            c.cycles += 6;
            c.div_s(0, 5, 0);
            c.mfc1(2, 0);
            c.lqc2(4, me + VEL);
            c.qmtc2(2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, me + VEL);
        }
        // Angular damping, the quaternion derivative and renormalisation.
        c.cycles += 65;
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 1);
        c.g[A0][0] = 0x3f000000;
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.mul_s(1, 4, 1);
        c.sub_s(0, 0, 1);
        c.mfc1(2, 0);
        c.lqc2(4, me + ANG_VEL);
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, me + ANG_VEL);
        c.g[V1][0] = c.lw(me + RIDER);
        c.qmtc2(4, 1);
        c.lqc2(4, r32(c, V1) + R_QUAT);
        c.lqc2(5, me + ANG_VEL);
        c.vu0().suba(0b0001, 0, Src::Vec(0));
        c.vu0().msub(0b0001, 2, 0, Src::Bc(1, X));
        c.vu0().mul(0b1110, 7, 4, Src::Vec(5));
        c.vu0().mul(0b1110, 3, 5, Src::Bc(4, W));
        c.vu0().opmula(5, 4);
        c.vu0().opmsub(6, 4, 5);
        c.vu0().mula(0b0001, 2, Src::Bc(7, X));
        c.vu0().madda(0b0001, 2, Src::Bc(7, Y));
        c.vu0().madd(0b0001, 8, 2, Src::Bc(7, Z));
        c.vu0().mula(0b1110, 3, Src::Bc(1, X));
        c.vu0().madd(0b1110, 8, 6, Src::Bc(1, X));
        c.sqc2(8, s + 0x20);
        c.lq(2, s + 0x20);
        c.sq(2, s + 0x10);
        c.mfc1(3, 4);
        c.lqc2(4, s + 0x10);
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, s + 0x20);
        c.lq(2, s + 0x20);
        c.g[V1][0] = c.lw(me + RIDER);
        c.sq(2, s);
        c.lwc1(0, s);
        c.lwc1(1, r32(c, V1) + R_QUAT);
        c.lwc1(2, r32(c, V1) + R_QUAT + 4);
        c.add_s(1, 1, 0);
        c.lwc1(3, r32(c, V1) + R_QUAT + 8);
        c.lwc1(4, r32(c, V1) + R_QUAT + 0xc);
        c.swc1(1, r32(c, V1) + R_QUAT);
        c.lwc1(0, s + 0x4);
        c.add_s(2, 2, 0);
        c.swc1(2, r32(c, V1) + R_QUAT + 4);
        c.lwc1(0, s + 0x8);
        c.add_s(3, 3, 0);
        c.swc1(3, r32(c, V1) + R_QUAT + 8);
        c.lwc1(0, s + 0xc);
        c.add_s(4, 4, 0);
        c.swc1(4, r32(c, V1) + R_QUAT + 0xc);
        c.g[V0][0] = c.lw(me + RIDER);
        c.lqc2(3, r32(c, V0) + R_QUAT);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().rsqrt(0, 3, 4, 0);
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 3, Src::Q);
        c.sqc2(5, r32(c, V0) + R_QUAT);
    }
    c.cycles += 2;
    frame.close(c);
}

// ---------------------------------------------------------------------------
// RiderWipeoutState_CastGroundRay (0x138960, wipeoutmotion.cpp)

pub(crate) const WO_RIDER: u32 = 0x40;

/// Rider fields: position, the surface id / contact uv / surface flags the
/// ray stores, the overlap list and its collision flags.
pub(crate) const WO_R_POS: u32 = 0x110;

pub(crate) const WO_R_SURFACE: u32 = 0x430;

pub(crate) const WO_R_CONTACT_U: u32 = 0xaac;

pub(crate) const WO_R_CONTACT_V: u32 = 0xab0;

pub(crate) const WO_R_SURFACE_FLAGS: u32 = 0x2d4;

pub(crate) const WO_R_OVERLAP: u32 = 0x860;

pub(crate) const WO_R_OVERLAP_FLAGS: u32 = 0x864;

/// Contact fields: object, surface, uv.
pub(crate) const HIT_OBJECT: u32 = 0x50;

pub(crate) const HIT_SURFACE: u32 = 0x54;

pub(crate) const HIT_U: u32 = 0x6c;

pub(crate) const HIT_V: u32 = 0x70;

pub(crate) const WO_COLL_SEGMENT_CTOR: u32 = 0x0032_e100;

pub(crate) const WO_FIND_CLOSEST_CONTACT: u32 = 0x0033_42d0;

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
// RiderWipeoutState_PostUpdateAir (0x137860)

/// The wipeout state's rider.
pub(crate) const PUA_STATE_RIDER: u32 = 0x40;

/// Rider fields.
pub(crate) const PUA_R_POS: u32 = 0x110;

pub(crate) const PUA_R_VEL: u32 = 0x1e0;

pub(crate) const PUA_R_GROUND_NORMAL: u32 = 0x370;

pub(crate) const PUA_R_3D0: u32 = 0x3d0;

pub(crate) const PUA_R_SURFACE: u32 = 0x438;

pub(crate) const PUA_R_GROUND_HEIGHT: u32 = 0x454;

pub(crate) const PUA_R_CONTACT_POINT: u32 = 0x460;

pub(crate) const PUA_R_VTABLE: u32 = 0x6c0;

pub(crate) const PUA_R_HANDLER: u32 = 0x77c;

pub(crate) const PUA_R_AIR: u32 = 0x788;

pub(crate) const PUA_R_OVERLAP: u32 = 0x860;

pub(crate) const PUA_R_WALL_HINT: u32 = 0x868;

pub(crate) const PUA_R_SPHERE_TREE: u32 = 0xaa0;

/// Rider handler: "landed" flag and landing velocity.
pub(crate) const PUA_H_LANDED: u32 = 0x314;

pub(crate) const PUA_H_LAND_VEL: u32 = 0x320;

/// Stack frame: hit A (the sphere's closest contact; its normal at +0x10),
/// the collision hull, hit B (the ground ray: position, normal, +0x20,
/// surface at +0x4c, object at +0x50) and three vector temporaries.
pub(crate) const PUA_HIT_A_NORMAL: u32 = 0x10;

pub(crate) const PUA_HULL: u32 = 0x80;

pub(crate) const PUA_HIT_B: u32 = 0x3a0;

pub(crate) const PUA_HIT_B_NORMAL: u32 = 0x3b0;

pub(crate) const PUA_HIT_B_20: u32 = 0x3c0;

pub(crate) const PUA_HIT_B_SURFACE: u32 = 0x3ec;

pub(crate) const PUA_HIT_B_OBJECT: u32 = 0x3f0;

pub(crate) const PUA_T420: u32 = 0x420;

pub(crate) const PUA_T430: u32 = 0x430;

pub(crate) const PUA_T440: u32 = 0x440;

pub(crate) const PUA_T450: u32 = 0x450;

pub(crate) const PUA_CAST_GROUND_RAY: Callee = Callee {
    addr: 0x0013_8960,
    f: ee::s01::RiderWipeoutState_CastGroundRay,
};

pub(crate) const PUA_COLL_HULL_CTOR: u32 = 0x0032_f650;

pub(crate) const PUA_FIND_CLOSEST_CONTACT: u32 = 0x0033_42d0;

pub(crate) const PUA_CLEAR_AIR_TIME: u32 = 0x0011_3998;

pub(crate) const PUA_TRIGGER_SPRAY: u32 = 0x0011_1aa0;

pub(crate) const PUA_TRANSLATE: u32 = 0x0010_6538;

pub(crate) const PUA_AIR_PRIME: u32 = 0x0011_35b8;

pub(crate) const PUA_UPDATE_BOUNDS: u32 = 0x0011_e150;

pub(crate) const PUA_WALL_COLLISION: u32 = 0x0010_5398;

pub(crate) const PUA_RIDER_COLLISIONS: u32 = 0x0010_7888;

pub(crate) const PUA_COLL_HULL_DTOR: u32 = 0x0032_f708;

/// The game's VU0 dot-product macro sequence: `vf3 = *a`, `vf5 = *b`,
/// x*x + y*y + z*z into vf4.w, moved to GPR `gpr` (`qmfc2`).
pub(crate) fn pua_dot(c: &mut Cpu, a: u32, b: u32, gpr: usize) {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    c.lqc2(3, a);
    c.lqc2(5, b);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(gpr, 4);
}

/// `*dst = *v * f[fs]` through VU0 (`mfc1 gpr; qmtc2 gpr, vf3; vmul.xyzw
/// vf5, vf4, vf3x`).
pub(crate) fn pua_scale(c: &mut Cpu, v: u32, fs: usize, gpr: usize, dst: u32) {
    use ssx3_ee_rt::vu0::{Src, X};
    c.mfc1(gpr, fs);
    c.lqc2(4, v);
    c.qmtc2(gpr, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, dst);
}

/// `*dst = *a - *b` (`vsub`) or `*a + *b` (`vadd`) through VU0.
pub(crate) fn pua_add_sub(c: &mut Cpu, a: u32, b: u32, dst: u32, sub: bool) {
    use ssx3_ee_rt::vu0::Src;
    c.lqc2(3, a);
    c.lqc2(4, b);
    if sub {
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    } else {
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    }
    c.sqc2(5, dst);
}

/// `self->rider` (read from s2 each time, as the original does).
pub(crate) fn pua_rider(c: &mut Cpu) -> u64 {
    Addr(r32(c, S2)).off(PUA_STATE_RIDER).lw(c)
}

/// `void RiderWipeoutState_PostUpdateAir(RiderWipeoutState *self)`
/// (0x137860): after a wipeout tumble step in the air, test the rider's
/// collision sphere against the world. On a contact, cast a ground ray
/// along the contact normal: a ground hit (ray distance >= 0.4) the rider
/// moves into (`vel . n <= 0`) within 10 units lands him (air time cleared,
/// dynamic spray, the handler's landed flag and landing velocity, ground
/// normal / surface / contact point / height, the normal velocity removed,
/// the hit object pushed by the impulse scaled by the world's int at +0x10,
/// pushed out of the ground, and `self->state = 0`). Otherwise the rider is
/// pushed out of the contact, loses the velocity into it, and the air
/// predictor is re-primed. Then always: bounds, wall and rider collisions,
/// and the hull is destroyed.
pub fn rider_wipeout_state_post_update_air(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x4d0);
        let at = |c: &Cpu, o: u32| sp(c).wrapping_add(o);
        frame.save(c, S1, 0x490);
        frame.save(c, S2, 0x480);
        c.g[S1][0] = sx(at(c, PUA_HULL));
        frame.save(c, S3, 0x470);
        c.g[S2][0] = c.g[A0][0];
        frame.save_fpr(c, 23, 0x4c8);
        c.g[S3][0] = c.g[S1][0];
        frame.save_fpr(c, 21, 0x4b8);
        frame.save_ra(c, 0x460);
        frame.save_fpr(c, 22, 0x4c0);
        frame.save_fpr(c, 20, 0x4b0);
        frame.save(c, S0, 0x4a0);
        c.g[S0][0] = pua_rider(c);
        c.mtc1(0, 23);
        // is_ai = rider->vtable->IsHuman() ^ 1 (slot 0x40).
        let rider = r32(c, S0);
        c.g[V1][0] = Addr(rider).off(PUA_R_VTABLE).lw(c);
        c.g[A1][0] = sx(rider.wrapping_add(PUA_R_VTABLE));
        c.g[A0][0] = Addr(r32(c, V1)).off(0x40).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x44).lw(c);
        let method = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, method, 0x0013_78b4)?;
        // CollHull hull(rider->sphere_tree, !human)
        c.g[A1][0] = Addr(r32(c, S0)).off(PUA_R_SPHERE_TREE).lw(c);
        c.g[A2][0] = c.g[V0][0] ^ 1;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, PUA_COLL_HULL_CTOR, 0x0013_78c4)?;
        // d = FindClosestContact(rider->overlap, &hull, &hit_a, wall_hint)
        c.g[V1][0] = pua_rider(c);
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[SP][0];
        c.g[A3][0] = Addr(r32(c, V1)).off(PUA_R_WALL_HINT).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).off(PUA_R_OVERLAP).lw(c);
        call_ptr(c, PUA_FIND_CLOSEST_CONTACT, 0x0013_78dc)?;
        c.mov_s(21, 0);
        c.c_le_s(23, 21);
        let contact = c.fcc();
        c.g[S0][0] = sx(at(c, PUA_HIT_B));
        if contact {
            pua_contact(c)?;
        }

        c.cycles += 22;
        c.g[A0][0] = pua_rider(c);
        c.g[A1][0] = 0;
        call_ptr(c, PUA_UPDATE_BOUNDS, 0x0013_7cc8)?;
        c.g[A0][0] = pua_rider(c);
        c.g[A1][0] = 0;
        call_ptr(c, PUA_WALL_COLLISION, 0x0013_7cd4)?;
        c.g[A0][0] = pua_rider(c);
        call_ptr(c, PUA_RIDER_COLLISIONS, 0x0013_7cdc)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 2;
        call_ptr(c, PUA_COLL_HULL_DTOR, 0x0013_7ce8)?;
        frame.restore(c, S0, 0x4a0);
        frame.restore(c, S1, 0x490);
        frame.restore(c, S2, 0x480);
        frame.restore(c, S3, 0x470);
        frame.restore_ra(c, 0x460);
        frame.restore_fpr(c, 23, 0x4c8);
        frame.restore_fpr(c, 22, 0x4c0);
        frame.restore_fpr(c, 21, 0x4b8);
        frame.restore_fpr(c, 20, 0x4b0);
        frame.close(c);
        Ok(())
    })
}

/// The sphere touched something (f21 = its distance): cast the ground ray
/// and land, or push out of the contact and re-prime the air predictor.
pub(crate) fn pua_contact(c: &mut Cpu) -> crate::call::Flow {
    let s = sp(c);
    // t = CastGroundRay(self, &hit_b, &hit_a.normal)
    c.cycles += 11;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = sx(s + PUA_HIT_A_NORMAL);
    PUA_CAST_GROUND_RAY.call(c, 0x0013_7900)?;
    c.mov_s(22, 23);
    c.mov_s(20, 0);
    c.lwc1(0, r32(c, GP).wrapping_sub(0x7268)); // 0.4
    c.c_le_s(0, 20);
    let ground = c.fcc();
    c.mov_s(1, 22);
    if ground {
        // f22 = vel . n, f1 = (pos - hit) . n
        c.cycles += 26;
        c.g[V0][0] = pua_rider(c);
        let rider = r32(c, V0);
        pua_dot(c, rider.wrapping_add(PUA_R_VEL), s + PUA_HIT_B_NORMAL, V1);
        c.mtc1(V1, 22);
        pua_add_sub(
            c,
            rider.wrapping_add(PUA_R_POS),
            s + PUA_HIT_B,
            s + PUA_T430,
            true,
        );
        c.lq(V1, s + PUA_T430);
        c.sq(V1, s + PUA_T420);
        pua_dot(c, s + PUA_T420, s + PUA_HIT_B_NORMAL, V0);
        c.mtc1(V0, 1);
        c.c_le_s(0, 20);
    }

    c.cycles += 2;
    let mut land = c.fcc();
    if land {
        c.cycles += 3;
        c.c_le_s(22, 23);
        land = c.fcc();
    }
    if land {
        c.cycles += 5;
        c.g[AT][0] = 0x4120_0000;
        c.mtc1(AT, 0); // 10.0
        c.c_le_s(1, 0);
        land = c.fcc();
    }
    if land {
        pua_land(c)?;
        return Ok(());
    }

    // Push out along hit_a's normal by d * k.
    c.cycles += 23;
    c.lwc1(0, r32(c, GP).wrapping_sub(0x7264));
    c.g[A1][0] = sx(s + PUA_T420);
    pua_scale(c, s + PUA_HIT_A_NORMAL, 0, V1, s + PUA_T440);
    c.lq(V0, s + PUA_T440);
    c.sq(V0, s + PUA_T430);
    pua_scale(c, s + PUA_T430, 21, V1, s + PUA_T440);
    c.lq(V0, s + PUA_T440);
    c.g[A0][0] = pua_rider(c);
    c.sq(V0, s + PUA_T420);
    call_ptr(c, PUA_TRANSLATE, 0x0013_7bf0)?;
    c.mtc1(0, 2);
    c.c_lt_s(20, 2);
    // Kill the velocity into hit_a's normal unless the ray hit something
    // facing it (n_b . n_a >= k).
    let mut kill = c.fcc();
    if !kill {
        c.cycles += 14;
        c.lwc1(0, r32(c, GP).wrapping_sub(0x7260));
        pua_dot(c, s + PUA_HIT_B_NORMAL, s + PUA_HIT_A_NORMAL, V0);
        c.mtc1(V0, 1);
        c.c_lt_s(1, 0);
        kill = c.fcc();
        if kill {
            c.cycles += 1;
        }
    }
    if kill {
        c.g[V0][0] = pua_rider(c);
        c.cycles += 13;
        let rider = r32(c, V0);
        pua_dot(c, s + PUA_HIT_A_NORMAL, rider.wrapping_add(PUA_R_VEL), V1);
        c.mtc1(V1, 0);
        c.c_lt_s(0, 2);
        if c.fcc() {
            // vel -= n_a * (vel . n_a)
            c.cycles += 13;
            pua_scale(c, s + PUA_HIT_A_NORMAL, 0, V0, s + PUA_T440);
            c.lq(V1, s + PUA_T440);
            c.g[V0][0] = pua_rider(c);
            c.sq(V1, s + PUA_T430);
            let rider = r32(c, V0);
            pua_add_sub(
                c,
                rider.wrapping_add(PUA_R_VEL),
                s + PUA_T430,
                rider.wrapping_add(PUA_R_VEL),
                true,
            );
        }
    }
    c.g[A1][0] = pua_rider(c);

    // AirPredictor_Prime(rider->air, &rider->pos, &rider->vel, k)
    c.cycles += 5;
    c.lwc1(12, r32(c, GP).wrapping_sub(0x725c));
    let rider = r32(c, A1);
    c.g[A0][0] = Addr(rider).off(PUA_R_AIR).lw(c);
    c.g[A2][0] = sx(rider.wrapping_add(PUA_R_VEL));
    c.g[A1][0] = sx(rider.wrapping_add(PUA_R_POS));
    call_ptr(c, PUA_AIR_PRIME, 0x0013_7cbc)?;
    Ok(())
}

/// Landing on hit_b (f22 = vel . n): see the parent's doc.
pub(crate) fn pua_land(c: &mut Cpu) -> crate::call::Flow {
    let s = sp(c);
    c.cycles += 65;
    // AirPredictor_ClearAirTime(rider->air, &rider->pos, &rider->vel)
    c.g[A1][0] = pua_rider(c);
    let rider = r32(c, A1);
    c.g[A0][0] = Addr(rider).off(PUA_R_AIR).lw(c);
    c.g[A2][0] = sx(rider.wrapping_add(PUA_R_VEL));
    c.g[A1][0] = sx(rider.wrapping_add(PUA_R_POS));
    call_ptr(c, PUA_CLEAR_AIR_TIME, 0x0013_79c0)?;
    // RiderBase_TriggerDynamicSpray(handler, &hit_b, &hit_b.normal, surface, vn)
    c.g[V0][0] = pua_rider(c);
    c.g[A1][0] = c.g[S0][0];
    c.g[A3][0] = Addr(s + PUA_HIT_B_SURFACE).lw(c);
    c.g[A2][0] = sx(s + PUA_HIT_B_NORMAL);
    c.g[A0][0] = Addr(r32(c, V0)).off(PUA_R_HANDLER).lw(c);
    c.mov_s(12, 22);
    call_ptr(c, PUA_TRIGGER_SPRAY, 0x0013_79dc)?;
    // handler->landed = 1, handler->land_vel = rider->vel
    c.g[A1][0] = pua_rider(c);
    c.g[A0][0] = 1;
    c.neg_s(1, 22);
    let rider = r32(c, A1);
    c.g[V1][0] = Addr(rider).off(PUA_R_HANDLER).lw(c);
    Addr(r32(c, V1)).off(PUA_H_LANDED).sw(c, 1);
    c.lq(V0, rider.wrapping_add(PUA_R_VEL));
    c.sq(V0, r32(c, V1).wrapping_add(PUA_H_LAND_VEL));
    // Ground state from hit_b.
    c.g[A0][0] = pua_rider(c);
    c.lq(V0, s + PUA_HIT_B_20);
    c.sq(V0, r32(c, A0).wrapping_add(PUA_R_3D0));
    c.g[V1][0] = pua_rider(c);
    c.lq(V0, s + PUA_HIT_B_NORMAL);
    c.sq(V0, r32(c, V1).wrapping_add(PUA_R_GROUND_NORMAL));
    c.g[A0][0] = pua_rider(c);
    c.g[V0][0] = Addr(s + PUA_HIT_B_SURFACE).lw(c);
    let surface = c.g[V0][0];
    Addr(r32(c, A0)).off(PUA_R_SURFACE).sw(c, surface);
    c.g[A1][0] = pua_rider(c);
    c.lq(V0, s + PUA_HIT_B);
    c.sq(V0, r32(c, A1).wrapping_add(PUA_R_CONTACT_POINT));
    // rider->ground_height = (pos - hit) . ground_normal
    c.g[V1][0] = pua_rider(c);
    let rider = r32(c, V1);
    pua_add_sub(
        c,
        rider.wrapping_add(PUA_R_POS),
        s + PUA_HIT_B,
        s + PUA_T430,
        true,
    );
    c.lq(V0, s + PUA_T430);
    c.g[V1][0] = pua_rider(c);
    c.sq(V0, s + PUA_T420);
    let rider = r32(c, V1);
    pua_dot(c, s + PUA_T420, rider.wrapping_add(PUA_R_GROUND_NORMAL), A0);
    c.mtc1(A0, 0);
    c.swc1(0, r32(c, V1).wrapping_add(PUA_R_GROUND_HEIGHT));
    // impulse = ground_normal * -vn; vel += impulse
    c.g[V0][0] = pua_rider(c);
    let rider = r32(c, V0);
    pua_scale(
        c,
        rider.wrapping_add(PUA_R_GROUND_NORMAL),
        1,
        A0,
        s + PUA_T430,
    );
    c.lq(V1, s + PUA_T430);
    c.g[V0][0] = pua_rider(c);
    c.sq(V1, s + PUA_T420);
    let rider = r32(c, V0);
    pua_add_sub(
        c,
        rider.wrapping_add(PUA_R_VEL),
        s + PUA_T420,
        rider.wrapping_add(PUA_R_VEL),
        false,
    );
    // The hit object's owner feels -impulse * world int (vtable slot 0x158).
    c.g[A0][0] = Addr(s + PUA_HIT_B_OBJECT).lw(c);
    if c.g[A0][0] != 0 {
        c.cycles += 3;
        c.g[T0][0] = Addr(r32(c, A0)).off(0xc).lw(c);
        c.g[A1][0] = c.g[S0][0];
        if c.g[T0][0] != 0 {
            c.cycles += 26;
            let owner = r32(c, T0);
            c.g[A3][0] = Addr(owner).off(0xc).lw(c);
            let vt = r32(c, A3);
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.mtc1(AT, 0); // -1.0
            c.g[A2][0] = sx(s + PUA_T430);
            c.g[A0][0] = Addr(vt).off(0x158).lh(c);
            pua_scale(c, s + PUA_T420, 0, V1, s + PUA_T450);
            c.lq(V0, s + PUA_T450);
            c.g[A0][0] = sx(owner.wrapping_add(r32(c, A0)));
            c.g[V1][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
            c.sq(V0, s + PUA_T440);
            c.lwc1(0, r32(c, V1).wrapping_add(0x10));
            c.cvt_s_w(0, 0);
            pua_scale(c, s + PUA_T440, 0, T0, s + PUA_T450);
            c.lq(V0, s + PUA_T450);
            c.sq(V0, s + PUA_T430);
            c.g[V1][0] = Addr(vt).off(0x15c).lw(c);
            let method = r32(c, V1);
            call_ptr(c, method, 0x0013_7b24)?;
        }
        c.cycles += 1;
    }
    c.g[V0][0] = pua_rider(c);

    // Push out of the ground: pen = (pos - hit) . n; if < 0, translate by -pen * n.
    c.cycles += 20;
    let rider = r32(c, V0);
    pua_add_sub(
        c,
        rider.wrapping_add(PUA_R_POS),
        s + PUA_HIT_B,
        s + PUA_T440,
        true,
    );
    c.mtc1(0, 0);
    c.lq(V0, s + PUA_T440);
    c.sq(V0, s + PUA_T430);
    pua_dot(c, s + PUA_T430, s + PUA_HIT_B_NORMAL, V1);
    c.mtc1(V1, 1);
    c.c_lt_s(1, 0);
    let below = c.fcc();
    c.g[A1][0] = sx(s + PUA_T430);
    if below {
        c.cycles += 10;
        c.neg_s(0, 1);
        pua_scale(c, s + PUA_HIT_B_NORMAL, 0, V1, s + PUA_T440);
        c.lq(V0, s + PUA_T440);
        c.g[A0][0] = pua_rider(c);
        c.sq(V0, s + PUA_T430);
        call_ptr(c, PUA_TRANSLATE, 0x0013_7ba0)?;
    }
    c.cycles += 2;
    Addr(r32(c, S2)).sw(c, 0);
    Ok(())
}

// ---------------------------------------------------------------------------
// RiderWipeoutState_UpdateAir.

/// `WipeoutState::spin` (the tumble vector damped every tick).
pub(crate) const WIPEOUT_STATE_SPIN: u32 = 0x30;

/// `WipeoutState::rider`.
pub(crate) const WIPEOUT_STATE_RIDER: u32 = 0x40;

/// `Rider::timeScale` (ticks this frame).
pub(crate) const WIPEOUT_RIDER_TIME_SCALE: u32 = 0x300;

/// `Rider::airPredictor`.
pub(crate) const WIPEOUT_RIDER_AIR_PREDICTOR: u32 = 0x788;

/// `AirPredictor::mode` (1 and 3 are ballistic flights).
pub(crate) const WIPEOUT_AP_MODE: u32 = 0xac;

/// `void RiderWipeoutState_UpdateAir(WipeoutState* self)` (0x137750): one
/// airborne tick of a wipeout. Advances the rider's air predictor by
/// `timeScale * (1/60)` (`AirPredictor_Update` on position +0x110 and
/// velocity +0x1e0); in a ballistic flight (mode 1 or 3) re-aims the rider
/// along the predicted path (`Rider_UpdateOrientation` with the time left,
/// `timeScale / (end - now)` or `* 60` when under a tick, and the turn
/// rate `timeScale * gp-0x726c`); damps the tumble by `1 - 1.5 dt` and
/// updates the implicit orientation.
pub fn rider_wipeout_state_update_air(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x7278)); // 1/60
        frame.save(c, S0, 0x10);
        frame.save_fpr(c, 20, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let rider = Addr((c.g[S0][0] as u32).wrapping_add(WIPEOUT_STATE_RIDER)).lw(c);
        c.g[A1][0] = rider;
        c.lwc1(0, (rider as u32).wrapping_add(WIPEOUT_RIDER_TIME_SCALE));
        c.g[A2][0] = sx((rider as u32).wrapping_add(0x1e0));
        c.g[A0][0] = Addr((rider as u32).wrapping_add(WIPEOUT_RIDER_AIR_PREDICTOR)).lw(c);
        c.mul_s(20, 0, 1); // dt
        c.g[A1][0] = sx((rider as u32).wrapping_add(0x110));
        c.mov_s(12, 20);
        call_ptr(c, 0x0011_3648, 0x0013_7788)?;

        // Ballistic flight (mode 1 or 3)?
        c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(WIPEOUT_STATE_RIDER)).lw(c);
        c.g[A0][0] = 1;
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(WIPEOUT_RIDER_AIR_PREDICTOR)).lw(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(WIPEOUT_AP_MODE)).lw(c);
        c.g[A1][0] = 0;
        let mut ballistic = c.g[V1][0] == 1;
        if !ballistic {
            c.cycles += 3;
            c.g[V0][0] = 3;
            ballistic = c.g[V1][0] == 3;
        }
        if ballistic {
            c.cycles += 1;
            c.g[A1][0] = 1;
        }
        c.cycles += 2;
        if ballistic {
            c.cycles += 11;
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(WIPEOUT_STATE_RIDER)).lw(c);
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x7274)); // one tick
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(WIPEOUT_RIDER_AIR_PREDICTOR)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            c.lwc1(1, (c.g[V1][0] as u32).wrapping_add(0xa0));
            c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x98));
            c.sub_s(0, 0, 1); // time left
            c.c_le_s(2, 0);
            let long = c.fcc();
            c.lwc1(
                12,
                (c.g[V0][0] as u32).wrapping_add(WIPEOUT_RIDER_TIME_SCALE),
            );
            if long {
                c.cycles += 3;
                c.div_s(12, 12, 0);
            } else {
                c.cycles += 3;
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x7270)); // 60
                c.mul_s(12, 12, 0);
            }
            c.lwc1(
                0,
                (c.g[A1][0] as u32).wrapping_add(WIPEOUT_RIDER_TIME_SCALE),
            );
            c.cycles += 7;
            c.g[A0][0] = c.g[A1][0];
            c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x726c));
            c.g[A1][0] = Addr((c.g[A1][0] as u32).wrapping_add(WIPEOUT_RIDER_AIR_PREDICTOR)).lw(c);
            c.mul_s(13, 0, 13);
            c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
            call_ptr(c, 0x0012_1aa0, 0x0013_7818)?;
        }

        // spin *= 1 - 1.5 * dt (VU0).
        c.cycles += 18;
        c.g[AT][0] = 0x3fc0_0000;
        c.mtc1(AT, 0);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 1);
        c.mul_s(0, 20, 0);
        c.sub_s(1, 1, 0);
        c.mfc1(2, 1);
        c.lqc2(4, (c.g[S0][0] as u32).wrapping_add(WIPEOUT_STATE_SPIN));
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, (c.g[S0][0] as u32).wrapping_add(WIPEOUT_STATE_SPIN));
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(WIPEOUT_STATE_RIDER)).lw(c);
        call_ptr(c, 0x0011_e098, 0x0013_784c)?;

        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void RiderWipeoutState_SaveState(RiderWipeoutState* self, Stream* s)`
/// (0x138ad8): writes the 0x40-byte state to the stream.
pub fn rider_wipeout_state_save_state(c: &mut Cpu) {
    save_state_write(c, 0x40, 0x0013_8b00);
}

// ---------------------------------------------------------------------------
// RiderWipeoutState_Enter (0x136c40)
// ---------------------------------------------------------------------------

/// The wipeout motion's rider pointer and its sphere tree.
pub(crate) const RIDER_WIPEOUT_STATE_ENTER_RIDER: u32 = 0x40;

pub(crate) const RIDER_WIPEOUT_STATE_ENTER_TREE: u32 = 0x44;

/// The rider's root position (`+0x110`), surface normal (`+0x370`), velocity
/// (`+0x1e0`) and the air predictor (`+0x788`).
pub(crate) const RIDER_WIPEOUT_STATE_ENTER_POS: u32 = 0x110;

pub(crate) const RIDER_WIPEOUT_STATE_ENTER_UP: u32 = 0x370;

pub(crate) const RIDER_WIPEOUT_STATE_ENTER_VEL: u32 = 0x1e0;

pub(crate) const RIDER_WIPEOUT_STATE_ENTER_AIR: u32 = 0x788;

/// The motion's own copy of the position (`+0x50`, `+0x60`).
pub(crate) const RIDER_WIPEOUT_STATE_ENTER_POS_A: u32 = 0x50;

pub(crate) const RIDER_WIPEOUT_STATE_ENTER_POS_B: u32 = 0x60;

/// The launch speed literal (`3333.3335f`) in the `.lit4` pool at `gp-0x72c0`.
#[cfg(test)]
pub(crate) const RIDER_WIPEOUT_STATE_ENTER_LAUNCH_SPEED: u32 = 0x4550_5556;

/// `void RiderWipeoutState_Enter(motion, int launched)` (0x136c40): enter the
/// wipeout rider state; create the motion's sphere tree on first use, copy
/// the rider's position into the motion, and either pull the rider back along
/// the surface normal (a plain crash) or kick off the launch predictor.
pub fn rider_wipeout_state_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RIDER_WIPEOUT_STATE_ENTER_TREE));
        let has_tree = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A1][0];
        if !has_tree {
            c.cycles += 9;
            c.g[A0][0] = 0x2b0;
            c.g[A1][0] = sx(0x0046_0000u32.wrapping_sub(0x7df0)); // &D_00458210
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x0013_6c78)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0032_9910, 0x0013_6c80)?;
            c.sw(
                (c.g[S0][0] as u32).wrapping_add(RIDER_WIPEOUT_STATE_ENTER_TREE),
                c.g[V0][0],
            );
        }
        c.cycles += 7;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RIDER_WIPEOUT_STATE_ENTER_RIDER));
        let rider = c.g[A0][0] as u32;
        c.lq(2, rider.wrapping_add(RIDER_WIPEOUT_STATE_ENTER_POS));
        c.g[A1][0] = sx(rider.wrapping_add(RIDER_WIPEOUT_STATE_ENTER_POS));
        c.sq(
            2,
            (c.g[S0][0] as u32).wrapping_add(RIDER_WIPEOUT_STATE_ENTER_POS_A),
        );
        c.lq(3, rider.wrapping_add(RIDER_WIPEOUT_STATE_ENTER_POS));
        let launched = c.g[S1][0] != 0;
        c.sq(
            3,
            (c.g[S0][0] as u32).wrapping_add(RIDER_WIPEOUT_STATE_ENTER_POS_B),
        );
        if launched {
            c.cycles += 6;
            c.g[V0][0] = 1;
            c.g[A2][0] = sx(rider.wrapping_add(RIDER_WIPEOUT_STATE_ENTER_VEL));
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            c.lwc1(12, gp_oc322(c, 0xffff_8d40)); // -0x72c0
            c.g[A0][0] = c.lw(rider.wrapping_add(RIDER_WIPEOUT_STATE_ENTER_AIR));
            call_ptr(c, 0x0011_35b8, 0x0013_6d14)?;
        } else {
            // `pos -= n * dot(pos, n)`: slide the rider back onto the surface.
            c.cycles += 23;
            c.lqc2(3, rider.wrapping_add(RIDER_WIPEOUT_STATE_ENTER_VEL));
            c.lqc2(5, rider.wrapping_add(RIDER_WIPEOUT_STATE_ENTER_UP));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(2, 4);
            c.mtc1(2, 0);
            c.mfc1(3, 0);
            c.lqc2(4, rider.wrapping_add(RIDER_WIPEOUT_STATE_ENTER_UP));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.lq(2, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RIDER_WIPEOUT_STATE_ENTER_RIDER));
            c.sq(2, c.g[SP][0] as u32);
            c.lqc2(
                3,
                (c.g[V1][0] as u32).wrapping_add(RIDER_WIPEOUT_STATE_ENTER_VEL),
            );
            c.lqc2(4, c.g[SP][0] as u32);
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(
                5,
                (c.g[V1][0] as u32).wrapping_add(RIDER_WIPEOUT_STATE_ENTER_VEL),
            );
            c.sw(c.g[S0][0] as u32, 0);
        }
        c.cycles += 10;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RIDER_WIPEOUT_STATE_ENTER_RIDER));
        c.sw((c.g[S0][0] as u32).wrapping_add(4), 0);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x2dc), 0);
        frame.restore(c, 17, 0x30);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(RIDER_WIPEOUT_STATE_ENTER_RIDER));
        frame.restore(c, 16, 0x40);
        frame.restore_ra(c, 0x20);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x2e0), 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderWipeoutState_SetVelocity (0x136de0)

/// The wipeout state's rider pointer (`+0x40`); the rider's angular velocity
/// (`+0x30`), up vector (`+0x370`), position (`+0x110`), velocity (`+0x1e0`)
/// and air predictor (`+0x788`); the "launched" flag (`+0x4`).
pub(crate) const OC326_RWSV_RIDER: u32 = 0x40;

pub(crate) const OC326_RWSV_ANG_VEL: u32 = 0x30;

pub(crate) const OC326_RWSV_UP: u32 = 0x370;

pub(crate) const OC326_RWSV_POS: u32 = 0x110;

pub(crate) const OC326_RWSV_VEL: u32 = 0x1e0;

pub(crate) const OC326_RWSV_AIR: u32 = 0x788;

pub(crate) const OC326_RWSV_LAUNCHED: u32 = 0x4;

/// The offset (`gp-0x72bc`) of the launch speed (`3333.3335f`) in the
/// `.lit4` pool.
pub(crate) const OC326_RWSV_LAUNCH_SPEED_LIT: u32 = 0x72bc;

/// `AirPredictor_Prime` (0x1135b8), through the function table.
pub(crate) const OC326_AIR_PREDICTOR_PRIME: u32 = 0x0011_35b8;

/// `void RiderWipeoutState_SetVelocity(motion, Vec4* vel, Vec4* angVel)`
/// (0x136de0): store the new angular velocity and velocity into the rider;
/// when not launched, slide the velocity back onto the surface along the
/// up vector (`vel -= up * dot(vel, up)`); when launched, kick off the air
/// predictor with the launch speed. Clears the launched flag either way.
pub fn rider_wipeout_state_set_velocity(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A0][0];
        // lq v1, *angVel ; sq v1, self+0x30
        c.lq(3, r32(c, A2));
        c.g[A0][0] = Addr(r32(c, S0)).off(OC326_RWSV_RIDER).lw(c);
        c.sq(3, r32(c, S0).wrapping_add(OC326_RWSV_ANG_VEL));
        // lq v0, *vel ; sq v0, rider->vel
        c.lq(2, r32(c, A1));
        c.sq(2, r32(c, A0).wrapping_add(OC326_RWSV_VEL));
        c.g[V1][0] = Addr(r32(c, S0)).lw(c); // self->launched
        if c.g[V1][0] != 0 {
            c.g[A1][0] = Addr(r32(c, S0)).off(OC326_RWSV_RIDER).lw(c);
            c.cycles += 6;
            c.lwc1(12, r32(c, GP).wrapping_sub(OC326_RWSV_LAUNCH_SPEED_LIT));
            c.g[A0][0] = Addr(r32(c, A1)).off(OC326_RWSV_AIR).lw(c);
            c.g[A2][0] = sx(r32(c, A1).wrapping_add(OC326_RWSV_VEL));
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(OC326_RWSV_POS));
            call_ptr(c, OC326_AIR_PREDICTOR_PRIME, 0x0013_6e84)?;
            c.sw(r32(c, S0).wrapping_add(OC326_RWSV_LAUNCHED), 0);
        } else {
            c.cycles += 24;
            c.g[V0][0] = Addr(r32(c, S0)).off(OC326_RWSV_RIDER).lw(c);
            c.lqc2(3, r32(c, V0).wrapping_add(OC326_RWSV_UP));
            c.lqc2(5, r32(c, A1));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(3, 4);
            c.mtc1(3, 0);
            c.mfc1(4, 0);
            c.lqc2(4, r32(c, V0).wrapping_add(OC326_RWSV_UP));
            c.qmtc2(4, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, sp(c).wrapping_add(0x10));
            c.lq(3, sp(c).wrapping_add(0x10));
            c.g[V0][0] = Addr(r32(c, S0)).off(OC326_RWSV_RIDER).lw(c);
            c.sq(3, sp(c));
            c.lqc2(3, r32(c, V0).wrapping_add(OC326_RWSV_VEL));
            c.lqc2(4, sp(c));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, r32(c, V0).wrapping_add(OC326_RWSV_VEL));
            c.sw(r32(c, S0).wrapping_add(OC326_RWSV_LAUNCHED), 0);
        }
        c.cycles += 4;
        frame.restore(c, 16, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// RiderWipeoutState_UpdateGround (0x137d18, wipeoutmotion.cpp)

/// Wipeout state: the spin vector (rescaled toward the speed) and the rider.
pub(crate) const WUG_SPIN: u32 = 0x30;

pub(crate) const WUG_RIDER: u32 = 0x40;

/// Wipeout state: 1 once the rider is launched into the air.
pub(crate) const WUG_IN_AIR: u32 = 0x4;

/// Rider fields.
pub(crate) const WUG_R_POS: u32 = 0x110;

pub(crate) const WUG_R_ROOT_POSE_OVERRIDE: u32 = 0x150;

pub(crate) const WUG_R_FRONT: u32 = 0x1b0;

pub(crate) const WUG_R_VEL: u32 = 0x1e0;

pub(crate) const WUG_R_TIME_SCALE: u32 = 0x300;

pub(crate) const WUG_R_GROUND_NORMAL: u32 = 0x370;

pub(crate) const WUG_R_GROUND_VEL: u32 = 0x3d0;

pub(crate) const WUG_R_SURFACE: u32 = 0x438;

pub(crate) const WUG_R_GROUND_HEIGHT: u32 = 0x454;

pub(crate) const WUG_R_CONTACT_POINT: u32 = 0x460;

pub(crate) const WUG_R_ANIM_MODEL: u32 = 0x780;

pub(crate) const WUG_R_HANDLER: u32 = 0x77c;

pub(crate) const WUG_R_ANIM: u32 = 0x784;

pub(crate) const WUG_R_AIR: u32 = 0x788;

/// Surface material record (0xb0 bytes): penetration factor, "reset rider"
/// flag, and the friction curve's four (x, y) points.
pub(crate) const WUG_MAT_SIZE: u32 = 0xb0;

pub(crate) const WUG_MAT_PEN: u32 = 0x18;

pub(crate) const WUG_MAT_RESET: u32 = 0x44;

pub(crate) const WUG_MAT_CURVE: u32 = 0x90;

/// The ground ray's hit record on the stack (`sp + 0xb0`): position, normal,
/// ground velocity, material, object, triangle.
pub(crate) const WUG_HIT: u32 = 0xb0;

pub(crate) const WUG_HIT_NORMAL: u32 = 0xc0;

pub(crate) const WUG_HIT_VEL: u32 = 0xd0;

pub(crate) const WUG_HIT_MATERIAL: u32 = 0xfc;

pub(crate) const WUG_HIT_OBJECT: u32 = 0x100;

pub(crate) const WUG_HIT_TRI: u32 = 0x104;

pub(crate) const WUG_GET_LAYER_ANIM: u32 = 0x0031_1ae8;

pub(crate) const WUG_UPDATE_ORIENTATION: u32 = 0x0012_1aa0;

pub(crate) const WUG_TRIGGER_SPRAY: u32 = 0x0011_1aa0;

pub(crate) const WUG_LAUNCH_INTO_AIR: u32 = 0x0011_35b8;

pub(crate) const WUG_CHECK_RESET: u32 = 0x0011_6120;

pub(crate) const WUG_BUILD_WORLD_MATRIX: u32 = 0x0011_e098;

pub(crate) const WUG_CAST_GROUND_RAY: Callee = Callee {
    addr: 0x0013_8960,
    f: ee::s01::RiderWipeoutState_CastGroundRay,
};

/// `|*v|` with the game's VU0 length sequence (4 components, `vsqrt`), out
/// through GPR `gpr` (`cfc2 gpr, Q`) into `f[fd]`.
pub(crate) fn wug_len(c: &mut Cpu, v: u32, gpr: usize, fd: usize) {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    c.lqc2(3, v);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(gpr, 22);
    c.mtc1(gpr, fd);
}

/// `*a . *b` (4 components) through VU0, out through GPR `gpr` into `f[fd]`.
pub(crate) fn wug_dot(c: &mut Cpu, a: u32, b: u32, gpr: usize, fd: usize) {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    c.lqc2(3, a);
    c.lqc2(5, b);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(gpr, 4);
    c.mtc1(gpr, fd);
}

/// `*dst = *v * f[fs]` through VU0 (the scalar via GPR `gpr`).
pub(crate) fn wug_scale(c: &mut Cpu, v: u32, fs: usize, gpr: usize, dst: u32) {
    use ssx3_ee_rt::vu0::{Src, X};
    c.mfc1(gpr, fs);
    c.lqc2(4, v);
    c.qmtc2(gpr, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, dst);
}

/// `*dst = *a + *b` (`add`) or `*a - *b` through VU0.
pub(crate) fn wug_add_sub(c: &mut Cpu, a: u32, b: u32, dst: u32, add: bool) {
    use ssx3_ee_rt::vu0::Src;
    c.lqc2(3, a);
    c.lqc2(4, b);
    if add {
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    } else {
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    }
    c.sqc2(5, dst);
}

/// `self->rider` (from s0, as the original reads it each time).
pub(crate) fn wug_rider(c: &mut Cpu) -> u64 {
    Addr(r32(c, S0)).off(WUG_RIDER).lw(c)
}

/// The friction curve (f1 = curve(f2), x in f2, f4 = p1.x on entry; s1 the
/// material): clamped to the first / last point's y, else the linear
/// interpolation between the two points around x.
pub(crate) fn wug_friction_curve(c: &mut Cpu) {
    let m = r32(c, S1);
    let p = |i: u32| m.wrapping_add(WUG_MAT_CURVE + 8 * i);
    c.c_lt_s(4, 2);
    if !c.fcc() {
        // x <= p1.x.
        c.lwc1(1, p(0));
        c.cycles += 4;
        c.c_lt_s(2, 1);
        if c.fcc() {
            // x < p0.x: p0.y.
            c.cycles += 2;
            c.lwc1(1, p(0) + 4);
            return;
        }
        c.lwc1(3, p(0) + 4);
        c.cycles += 2;
        c.sub_s(2, 2, 1);
        c.lwc1(0, p(1) + 4);
        c.cycles += 1;
        c.sub_s(1, 4, 1);
    } else {
        c.cycles += 5;
        c.lwc1(1, p(2));
        c.c_lt_s(1, 2);
        if !c.fcc() {
            // p1.x < x <= p2.x.
            c.lwc1(3, p(1) + 4);
            c.cycles += 4;
            c.sub_s(2, 2, 4);
            c.lwc1(0, p(2) + 4);
            c.sub_s(1, 1, 4);
        } else {
            c.cycles += 5;
            c.lwc1(4, p(3));
            c.c_lt_s(4, 2);
            if c.fcc() {
                // x > p3.x: p3.y.
                c.cycles += 2;
                c.lwc1(1, p(3) + 4);
                return;
            }
            // p2.x < x <= p3.x.
            c.lwc1(3, p(2) + 4);
            c.cycles += 3;
            c.sub_s(2, 2, 1);
            c.lwc1(0, p(3) + 4);
            c.cycles += 1;
            c.sub_s(1, 4, 1);
        }
    }
    // y = y0 + (y1 - y0) * (x - x0) / (x1 - x0).
    c.cycles += 4;
    c.sub_s(0, 0, 3);
    c.mul_s(0, 0, 2);
    c.div_s(0, 0, 1);
    c.add_s(1, 3, 0);
}

/// `void RiderWipeoutState_UpdateGround(RiderWipeoutState* self)` (0x137d18):
/// one tick of a wiped-out rider tumbling along the ground (`dt` = time
/// scale / 60). The spin vector eases toward the speed; the sliding
/// velocity (relative to the ground's) loses its tangential part through
/// the surface's friction curve (stronger at low speed in the stick poses
/// 0x1d / 0x19 or with the root pose overridden, scaled by how flat the
/// ground is, at most 1/dt); gravity; position and velocity integrate, the
/// speed is clamped to 3333.33; the rider turns toward front + velocity.
/// Then a ground ray: a hit takes the ground's velocity, normal, surface and
/// contact point, and by the rider's penetration either pushes him out
/// (deep: velocity into the ground removed, spray), springs him back
/// (shallow: spray), or launches him (more than 10 above); a hit object's
/// owner gets the impulse (vtable slot 0x158); a "reset" surface or a
/// triangle with flag 2 checks for a reset. No hit launches him into the
/// air. Finally the world matrix is rebuilt.
pub fn rider_wipeout_state_update_ground(c: &mut Cpu) {
    guest(c, |c| {
        let gp = r32(c, GP);
        c.cycles += 44;
        let frame = Frame::open(c, 0x190);
        let s = sp(c);
        c.lwc1(2, gp.wrapping_sub(0x7258)); // 1/60
        frame.save(c, S0, 0x160);
        frame.save_fpr(c, 22, 0x180);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x150);
        frame.save(c, S2, 0x140);
        frame.save_ra(c, 0x130);
        frame.save_fpr(c, 21, 0x178);
        frame.save_fpr(c, 20, 0x170);
        let me = r32(c, S0);

        // Spin length eases toward |vel| * 0.0125: len + (target - len) * 5 * dt.
        wug_len(c, me + WUG_SPIN, V1, 5);
        c.lwc1(3, gp.wrapping_sub(0x7254));
        c.g[V0][0] = Addr(me + WUG_RIDER).lw(c);
        let rider = r32(c, V0);
        wug_len(c, rider + WUG_R_VEL, V1, 0);
        c.g[AT][0] = 0x40a0_0000;
        c.mtc1(AT, 4);
        c.mul_s(0, 0, 2);
        c.lwc1(1, rider + WUG_R_TIME_SCALE);
        c.lwc1(2, gp.wrapping_sub(0x7250));
        c.mul_s(22, 1, 3); // dt
        c.sub_s(0, 0, 5);
        c.c_lt_s(2, 5);
        c.mul_s(0, 0, 4);
        c.mul_s(0, 0, 22);
        let rescale = c.fcc();
        c.add_s(0, 5, 0);
        if rescale {
            c.cycles += 6;
            c.div_s(0, 0, 5);
            wug_scale(c, me + WUG_SPIN, 0, V0, me + WUG_SPIN);
        }

        // Tangential sliding velocity and the surface's friction.
        c.cycles += 54;
        c.g[A1][0] = Addr(gp.wrapping_sub(0x848)).lw(c);
        c.g[A2][0] = WUG_MAT_SIZE as u64;
        c.g[V0][0] = Addr(me + WUG_RIDER).lw(c);
        c.g[V1][0] = Addr(r32(c, A1) + 0x84).lw(c);
        let rider = r32(c, V0);
        c.g[A0][0] = Addr(rider + WUG_R_SURFACE).lw(c);
        c.g[A1][0] = Addr(r32(c, V1) + 0x44).lw(c);
        wug_add_sub(
            c,
            rider + WUG_R_VEL,
            rider + WUG_R_GROUND_VEL,
            s + 0x10,
            false,
        );
        c.mult(0, A0, A0, A2);
        c.lq(V1, s + 0x10);
        c.g[V0][0] = Addr(me + WUG_RIDER).lw(c);
        c.sq(V1, s);
        let rider = r32(c, V0);
        wug_dot(c, s, rider + WUG_R_GROUND_NORMAL, A2, 0);
        c.lwc1(1, gp.wrapping_sub(0x724c));
        wug_scale(c, rider + WUG_R_GROUND_NORMAL, 0, V1, s + 0x20);
        c.g[S1][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        c.lq(V0, s + 0x20);
        c.sq(V0, s + 0x10);
        wug_add_sub(c, s, s + 0x10, s + 0x30, false);
        c.lq(V0, s + 0x30);
        c.sq(V0, s + 0x20); // tangent
        wug_len(c, s + 0x20, V1, 21);
        c.mul_s(2, 21, 1);
        c.lwc1(4, r32(c, S1) + WUG_MAT_CURVE + 8);
        wug_friction_curve(c);

        // friction = curve * 3 + 1.5; at low speed in a stick pose, blend
        // toward 40 by (1 - tl * 0.0009)^4.
        c.cycles += 10;
        c.g[AT][0] = 0x4040_0000;
        c.mtc1(AT, 0);
        c.lwc1(2, gp.wrapping_sub(0x7248));
        c.mul_s(0, 1, 0);
        c.g[AT][0] = 0x3fc0_0000;
        c.mtc1(AT, 1);
        c.c_lt_s(21, 2);
        let slow = c.fcc();
        c.add_s(20, 0, 1);
        let mut reload = true;
        if slow {
            c.cycles += 4;
            c.g[V1][0] = Addr(r32(c, S0) + WUG_RIDER).lw(c);
            c.g[V0][0] = Addr(r32(c, V1) + WUG_R_ROOT_POSE_OVERRIDE).lw(c);
            let mut stick = c.g[V0][0] != 0;
            if !stick {
                c.cycles += 6;
                c.g[A0][0] = Addr(r32(c, V1) + WUG_R_ANIM).lw(c);
                c.g[A1][0] = 2;
                call_ptr(c, WUG_GET_LAYER_ANIM, 0x0013_7f7c)?;
                c.g[V1][0] = 0x1d;
                stick = c.g[V0][0] == c.g[V1][0];
                c.g[A1][0] = 2;
                if !stick {
                    c.cycles += 6;
                    c.g[V0][0] = wug_rider(c);
                    c.g[A0][0] = Addr(r32(c, V0) + WUG_R_ANIM).lw(c);
                    call_ptr(c, WUG_GET_LAYER_ANIM, 0x0013_7f94)?;
                    c.g[V1][0] = 0x19;
                    stick = c.g[V0][0] == c.g[V1][0];
                    if !stick {
                        c.g[V0][0] = wug_rider(c);
                        reload = false;
                    }
                }
            }
            if stick {
                c.cycles += 13;
                c.lwc1(0, gp.wrapping_sub(0x7244));
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(AT, 1);
                c.mul_s(0, 21, 0);
                c.g[AT][0] = 0x4220_0000;
                c.mtc1(AT, 2);
                c.sub_s(0, 1, 0);
                c.mul_s(0, 0, 0);
                c.mul_s(0, 0, 0);
                c.sub_s(1, 1, 0);
                c.mul_s(0, 0, 2);
                c.mul_s(1, 1, 20);
                c.add_s(20, 0, 1);
            }
        }
        if reload {
            c.cycles += 1;
            c.g[V0][0] = wug_rider(c);
        }

        // friction *= clamp(normal.z + 0.3, 0, 1); at most 1 / dt. f21 = 0.
        c.cycles += 10;
        c.lwc1(1, gp.wrapping_sub(0x7240));
        c.lwc1(0, r32(c, V0) + WUG_R_GROUND_NORMAL + 8);
        c.mtc1(0, 21);
        c.add_s(0, 0, 1);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 1);
        c.c_le_s(21, 0);
        let positive = c.fcc();
        c.mov_s(2, 21);
        if positive {
            c.cycles += 1;
            c.min_s(2, 0, 1);
        }
        c.cycles += 6;
        c.div_s(0, 1, 22);
        c.mul_s(20, 20, 2);
        c.c_lt_s(0, 20);
        if c.fcc() {
            c.mov_s(20, 0);
        }

        // acc = gravity * -1800 + tangent * -friction; pos += vel * dt;
        // vel += acc * dt; |vel| <= 3333.33.
        c.cycles += 66;
        c.g[V1][0] = 0x50_0000;
        c.g[AT][0] = 0xffff_ffff_c4e1_0000;
        c.mtc1(AT, 0);
        c.g[V1][0] = sx(0x004f_f160);
        c.neg_s(1, 20);
        wug_scale(c, 0x004f_f160, 0, V0, s + 0x40);
        c.lwc1(20, gp.wrapping_sub(0x723c));
        c.lq(V0, s + 0x40);
        c.sq(V0, s + 0x30);
        wug_scale(c, s + 0x20, 1, V1, s + 0x50);
        c.lq(V0, s + 0x50);
        c.sq(V0, s + 0x40);
        wug_add_sub(c, s + 0x30, s + 0x40, s + 0x60, true);
        c.lq(V0, s + 0x60);
        c.g[V1][0] = wug_rider(c);
        c.sq(V0, s + 0x50);
        let rider = r32(c, V1);
        wug_scale(c, rider + WUG_R_VEL, 22, A0, s + 0x70);
        c.lq(V0, s + 0x70);
        c.g[V1][0] = wug_rider(c);
        c.sq(V0, s + 0x60);
        let rider = r32(c, V1);
        wug_add_sub(c, rider + WUG_R_POS, s + 0x60, rider + WUG_R_POS, true);
        wug_scale(c, s + 0x50, 22, A0, s + 0x70);
        c.lq(V0, s + 0x70);
        c.sq(V0, s + 0x60);
        c.g[V1][0] = wug_rider(c);
        let rider = r32(c, V1);
        wug_add_sub(c, rider + WUG_R_VEL, s + 0x60, rider + WUG_R_VEL, true);
        c.g[V1][0] = wug_rider(c);
        let rider = r32(c, V1);
        wug_len(c, rider + WUG_R_VEL, V0, 0);
        c.c_lt_s(20, 0);
        if c.fcc() {
            c.cycles += 8;
            c.div_s(0, 20, 0);
            wug_scale(c, rider + WUG_R_VEL, 0, V0, rider + WUG_R_VEL);
            c.g[V1][0] = wug_rider(c);
        }
        c.g[A2][0] = sx(s + 0x60);

        // Turn toward normalize(front + vel * 0.0036).
        c.cycles += 46;
        let rider = r32(c, V1);
        c.lwc1(0, gp.wrapping_sub(0x7238));
        c.lq(V0, rider + WUG_R_FRONT);
        c.lwc1(13, gp.wrapping_sub(0x7234));
        c.sq(V0, s + 0x80);
        wug_scale(c, rider + WUG_R_VEL, 0, A0, s + 0xa0);
        c.lq(V0, s + 0xa0);
        c.sq(V0, s + 0x90);
        wug_add_sub(c, s + 0x80, s + 0x90, s + 0xa0, true);
        c.lq(V0, s + 0xa0);
        c.sq(V0, s + 0x70);
        {
            use ssx3_ee_rt::vu0::{Src, W, Y, Z};
            c.lqc2(3, s + 0x70);
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().rsqrt(0, 3, 4, 0);
            c.vu0().waitq();
            c.vu0().mul(0b1111, 5, 3, Src::Q);
            c.sqc2(5, s + 0xa0);
        }
        c.lq(V0, s + 0xa0);
        c.g[V1][0] = wug_rider(c);
        c.sq(V0, s + 0x60);
        c.g[A0][0] = c.g[V1][0];
        let rider = r32(c, V1);
        c.lwc1(12, rider + WUG_R_TIME_SCALE);
        c.g[A1][0] = sx(rider + WUG_R_GROUND_NORMAL);
        c.mul_s(13, 12, 13);
        c.add_s(12, 12, 12);
        call_ptr(c, WUG_UPDATE_ORIENTATION, 0x0013_81d4)?;

        // The ground ray.
        c.g[A2][0] = wug_rider(c);
        c.g[A1][0] = sx(sp(c) + WUG_HIT);
        c.g[S2][0] = c.g[A1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = sx(r32(c, A2) + WUG_R_GROUND_NORMAL);
        WUG_CAST_GROUND_RAY.call(c, 0x0013_81ec)?;
        let s = sp(c);
        c.c_le_s(21, 0);
        let hit = c.fcc();
        c.lq(V0, s + WUG_HIT_VEL);
        if hit {
            wug_ground_hit(c)?;
        } else {
            c.cycles += 8;
            c.g[A1][0] = wug_rider(c);
            wug_launch(c, 0x0013_8608)?;
        }

        c.cycles += 11;
        c.g[A0][0] = wug_rider(c);
        call_ptr(c, WUG_BUILD_WORLD_MATRIX, 0x0013_8618)?;
        frame.restore(c, S0, 0x160);
        frame.restore(c, S1, 0x150);
        frame.restore(c, S2, 0x140);
        frame.restore_ra(c, 0x130);
        frame.restore_fpr(c, 22, 0x180);
        frame.restore_fpr(c, 21, 0x178);
        frame.restore_fpr(c, 20, 0x170);
        frame.close(c);
        Ok(())
    })
}

/// `AirPredictor_StartLaunchIntoAir(rider->air, &rider->pos, &rider->vel,
/// 3333.33)` (a1 = rider), then `self->inAir = 1`.
pub(crate) fn wug_launch(c: &mut Cpu, ret: u32) -> crate::call::Flow {
    c.mov_s(12, 20);
    let rider = r32(c, A1);
    c.g[A0][0] = Addr(rider + WUG_R_AIR).lw(c);
    c.g[A2][0] = sx(rider + WUG_R_VEL);
    c.g[A1][0] = sx(rider + WUG_R_POS);
    call_ptr(c, WUG_LAUNCH_INTO_AIR, ret)?;
    c.g[V0][0] = 1;
    c.sw(r32(c, S0) + WUG_IN_AIR, 1);
    Ok(())
}

/// The ground ray hit (v0 = the hit's ground velocity, 128 bits): take the
/// ground, react to the penetration, push the hit object's owner, check
/// for a reset.
pub(crate) fn wug_ground_hit(c: &mut Cpu) -> crate::call::Flow {
    let s = sp(c);
    c.cycles += 53;
    c.g[V1][0] = wug_rider(c);
    c.sq(V0, r32(c, V1) + WUG_R_GROUND_VEL);
    c.g[A0][0] = wug_rider(c);
    c.lq(V0, s + WUG_HIT_NORMAL);
    c.sq(V0, r32(c, A0) + WUG_R_GROUND_NORMAL);
    c.g[V1][0] = wug_rider(c);
    c.g[V0][0] = Addr(s + WUG_HIT_MATERIAL).lw(c);
    c.sw(r32(c, V1) + WUG_R_SURFACE, c.g[V0][0]);
    c.g[A0][0] = wug_rider(c);
    c.lq(V0, s + WUG_HIT);
    c.sq(V0, r32(c, A0) + WUG_R_CONTACT_POINT);
    c.g[V1][0] = wug_rider(c);
    let rider = r32(c, V1);
    wug_add_sub(c, rider + WUG_R_POS, s + WUG_HIT, s + 0x80, false);
    c.lq(A0, s + 0x80);
    c.g[V0][0] = wug_rider(c);
    c.sq(A0, s + 0x70);
    let rider = r32(c, V0);
    // Height above the ground.
    wug_dot(c, s + 0x70, rider + WUG_R_GROUND_NORMAL, V1, 0);
    c.swc1(0, rider + WUG_R_GROUND_HEIGHT);
    c.g[V1][0] = wug_rider(c);
    let rider = r32(c, V1);
    c.lwc1(1, r32(c, S1) + WUG_MAT_PEN);
    c.g[V0][0] = Addr(rider + WUG_R_ANIM_MODEL).lw(c);
    c.lwc1(0, r32(c, V0) + 0x140);
    wug_add_sub(c, rider + WUG_R_POS, s + WUG_HIT, s + 0x80, false);
    c.mul_s(2, 0, 1); // limit
    c.lq(V0, s + 0x80);
    c.sq(V0, s + 0x70);
    c.neg_s(0, 2);
    // Penetration along the hit normal (f1).
    wug_dot(c, s + 0x70, s + WUG_HIT_NORMAL, V0, 1);
    c.c_lt_s(1, 0);
    let spray;
    if c.fcc() {
        // Deep: drop the velocity into the ground, push out to the limit.
        c.cycles += 24;
        c.g[V0][0] = wug_rider(c);
        let rider = r32(c, V0);
        wug_len(c, s + WUG_HIT_VEL, V1, 12);
        wug_dot(c, rider + WUG_R_VEL, s + WUG_HIT_NORMAL, A0, 0);
        c.c_lt_s(0, 21);
        if c.fcc() {
            c.cycles += 13;
            wug_scale(c, s + WUG_HIT_NORMAL, 0, V0, s + 0x80);
            c.lq(V1, s + 0x80);
            c.g[V0][0] = wug_rider(c);
            c.sq(V1, s + 0x70);
            let rider = r32(c, V0);
            wug_add_sub(c, rider + WUG_R_VEL, s + 0x70, rider + WUG_R_VEL, false);
        }
        c.add_s(0, 1, 2);
        c.cycles += 16;
        c.g[A1][0] = c.g[S2][0];
        wug_scale(c, s + WUG_HIT_NORMAL, 0, V0, s + 0x80);
        c.g[A2][0] = sx(s + WUG_HIT_NORMAL);
        c.lq(V1, s + 0x80);
        c.g[V0][0] = wug_rider(c);
        c.sq(V1, s + 0x70);
        let rider = r32(c, V0);
        wug_add_sub(c, rider + WUG_R_POS, s + 0x70, rider + WUG_R_POS, false);
        c.g[V1][0] = wug_rider(c);
        spray = true;
    } else {
        c.cycles += 4;
        c.c_lt_s(1, 21);
        let below = c.fcc();
        c.g[A1][0] = c.g[S2][0];
        if below {
            // Shallow: spring back, vel += n * dt * (pen * -2700 / limit - vel . n).
            c.cycles += 39;
            c.lwc1(0, r32(c, GP).wrapping_sub(0x7230));
            c.g[V0][0] = wug_rider(c);
            c.g[A2][0] = sx(s + WUG_HIT_NORMAL);
            c.mul_s(0, 1, 0);
            let rider = r32(c, V0);
            wug_len(c, rider + WUG_R_VEL, A0, 12);
            wug_dot(c, rider + WUG_R_VEL, rider + WUG_R_GROUND_NORMAL, V1, 1);
            c.div_s(0, 0, 2);
            c.sub_s(0, 0, 1);
            c.mul_s(0, 22, 0);
            wug_scale(c, rider + WUG_R_GROUND_NORMAL, 0, A0, s + 0x80);
            c.lq(V1, s + 0x80);
            c.g[V0][0] = wug_rider(c);
            c.sq(V1, s + 0x70);
            let rider = r32(c, V0);
            wug_add_sub(c, rider + WUG_R_VEL, s + 0x70, rider + WUG_R_VEL, true);
            c.g[V1][0] = wug_rider(c);
            spray = true;
        } else {
            spray = false;
            c.cycles += 6;
            c.g[AT][0] = 0x4120_0000;
            c.mtc1(AT, 0);
            c.c_lt_s(0, 1);
            let above = c.fcc();
            c.g[A0][0] = Addr(s + WUG_HIT_OBJECT).lw(c);
            if above {
                // More than 10 above: drop the velocity into the ground, launch.
                c.cycles += 14;
                c.g[V1][0] = wug_rider(c);
                let rider = r32(c, V1);
                wug_dot(c, rider + WUG_R_VEL, rider + WUG_R_GROUND_NORMAL, V0, 0);
                c.c_lt_s(0, 21);
                if c.fcc() {
                    c.cycles += 13;
                    wug_scale(c, rider + WUG_R_GROUND_NORMAL, 0, V0, s + 0x80);
                    c.lq(V1, s + 0x80);
                    c.g[V0][0] = wug_rider(c);
                    c.sq(V1, s + 0x70);
                    let rider = r32(c, V0);
                    wug_add_sub(c, rider + WUG_R_VEL, s + 0x70, rider + WUG_R_VEL, false);
                }
                c.g[A1][0] = wug_rider(c);
                c.cycles += 8;
                wug_launch(c, 0x0013_84fc)?;
                c.g[A0][0] = Addr(s + WUG_HIT_OBJECT).lw(c);
            }
        }
    }
    if spray {
        // RiderBase_TriggerDynamicSpray(rider->handler, &hit, &hit.normal,
        // hit.material, speed).
        c.cycles += 5;
        c.g[A3][0] = Addr(s + WUG_HIT_MATERIAL).lw(c);
        c.g[A0][0] = Addr(r32(c, V1) + WUG_R_HANDLER).lw(c);
        call_ptr(c, WUG_TRIGGER_SPRAY, 0x0013_845c)?;
        c.g[A0][0] = Addr(s + WUG_HIT_OBJECT).lw(c);
    }

    // The hit object's owner takes the impulse (v - vel + groundVel) / dt.
    c.cycles += 2;
    let object = c.g[A0][0];
    c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
    if object != 0 {
        c.cycles += 3;
        c.g[T0][0] = Addr(object as u32 + 0xc).lw(c);
        c.g[A1][0] = c.g[S2][0];
        if c.g[T0][0] != 0 {
            c.cycles += 31;
            let owner = r32(c, T0);
            c.g[A3][0] = Addr(owner + 0xc).lw(c);
            c.g[V1][0] = wug_rider(c);
            c.g[A2][0] = sx(s + 0x70);
            let vt = r32(c, A3);
            c.g[A0][0] = Addr(vt + 0x158).lh(c);
            let rider = r32(c, V1);
            wug_add_sub(c, s, rider + WUG_R_VEL, s + 0xa0, false);
            c.lq(V0, s + 0xa0);
            c.g[A0][0] = sx(owner.wrapping_add(r32(c, A0)));
            c.g[V1][0] = wug_rider(c);
            c.sq(V0, s + 0x90);
            let rider = r32(c, V1);
            wug_add_sub(c, s + 0x90, rider + WUG_R_GROUND_VEL, s + 0xa0, true);
            c.lq(V0, s + 0xa0);
            c.sq(V0, s + 0x80);
            {
                use ssx3_ee_rt::vu0::Src;
                c.mfc1(V1, 22);
                c.qmtc2(V1, 3);
                c.vu0().div(0, 3, 3, 0);
                c.lqc2(4, s + 0x80);
                c.vu0().waitq();
                c.vu0().mul(0b1111, 4, 4, Src::Q);
                c.sqc2(4, s + 0xa0);
            }
            c.lq(V0, s + 0xa0);
            c.sq(V0, s + 0x70);
            c.g[V1][0] = Addr(vt + 0x15c).lw(c);
            let method = r32(c, V1);
            call_ptr(c, method, 0x0013_8594)?;
            c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
        }
    }

    // A "reset" ground material, or a triangle flagged 2: check for a reset.
    c.cycles += 9;
    c.g[A1][0] = WUG_MAT_SIZE as u64;
    c.g[V1][0] = Addr(s + WUG_HIT_MATERIAL).lw(c);
    c.g[A0][0] = Addr(r32(c, V0) + 0x84).lw(c);
    c.mult(0, V1, V1, A1);
    c.g[V0][0] = Addr(r32(c, A0) + 0x44).lw(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    c.g[A0][0] = Addr(r32(c, V0) + WUG_MAT_RESET).lw(c);
    let mut reset = c.g[A0][0] != 0;
    if !reset {
        c.cycles += 3;
        c.g[V0][0] = Addr(s + WUG_HIT_TRI).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, V0) + 0xa).lh(c);
            c.g[V0][0] &= 2;
            if c.g[V0][0] != 0 {
                c.cycles += 1;
                reset = true;
            }
        }
    }
    if reset {
        c.g[A0][0] = wug_rider(c);
        c.cycles += 5;
        c.g[A1][0] = 0;
        c.g[A2][0] = 1;
        call_ptr(c, WUG_CHECK_RESET, 0x0013_85e8)?;
    }
    Ok(())
}

/// `vf4.w = a . b` with the game's VU0 dot macro (`vf3` and `vf5` the
/// operands, `vf6` scratch).
pub(crate) fn racw_dot_vf3_vf5(c: &mut Cpu) {
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// `*dst = *v * f[fs]` through VU0 (`mfc1 v1; qmtc2 v1, vf3; vmul vf5,
/// vf4, vf3x`).
pub(crate) fn racw_scale(c: &mut Cpu, gpr: usize, fs: usize, v: u32, dst: u32) {
    c.mfc1(gpr, fs);
    c.lqc2(4, v);
    c.qmtc2(gpr, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, dst);
}

// ---------------------------------------------------------------------------
// RiderWipeoutState_PostUpdateRootPose (0x137138)

/// Wipeout state fields: the body's velocity (+0x10) and angular velocity
/// (+0x20), the body (+0x40; active flag at +0x150, position at +0x130),
/// the sphere tree (+0x44), the swept segment (+0x50, +0x60).
pub(crate) const PURP_VEL: u32 = 0x10;

pub(crate) const PURP_ANG_VEL: u32 = 0x20;

pub(crate) const PURP_BODY: u32 = 0x40;

pub(crate) const PURP_TREE: u32 = 0x44;

pub(crate) const PURP_SEG: u32 = 0x50;

pub(crate) const PURP_BODY_POS: u32 = 0x130;

pub(crate) const PURP_BODY_ACTIVE: u32 = 0x150;

/// Stack frame: the hit (normal at +0x10, object at +0x50), the sphere at
/// +0x80 (its segment at +0xa0/+0xb0), vector temporaries, the sines and
/// cosines at +0x410.
pub(crate) const PURP_HIT_NORMAL: u32 = 0x10;

pub(crate) const PURP_HIT_OBJECT: u32 = 0x50;

pub(crate) const PURP_SPHERE: u32 = 0x80;

pub(crate) const PURP_BUILD_SPHERES: Callee = Callee {
    addr: 0x0013_7550,
    f: ee::s01::RiderWipeoutState_BuildBoardSpheres,
};

pub(crate) const PURP_COLL_HULL_CTOR: u32 = 0x0032_f650;

pub(crate) const PURP_FIND_CLOSEST_CONTACT: u32 = 0x0033_6850;

pub(crate) const PURP_TREE_TRANSLATE: u32 = 0x0032_9b40;

pub(crate) const PURP_RAND_RANGE: u32 = 0x0031_7830;

pub(crate) const PURP_SIN_COS: u32 = 0x0031_be50;

pub(crate) const PURP_COLL_HULL_DTOR: u32 = 0x0032_f708;

/// `hit.object->owner->OnPush(&hit, impulse)` (vtable slot 0x158), the
/// impulse `src * (world->int_at_0x10 * k)` built at `sp+tmp`, the
/// argument at `sp+arg`. a0 = hit.object.
pub(crate) fn purp_push_object(
    c: &mut Cpu,
    k_off: u32,
    src: u32,
    tmp: u32,
    arg: u32,
    ret: u32,
) -> crate::call::Flow {
    c.cycles += 3;
    c.g[V1][0] = Addr(r32(c, A0)).off(0xc).lw(c);
    c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
    if c.g[V1][0] == 0 {
        return Ok(());
    }
    c.cycles += 19;
    let s = sp(c);
    c.g[A1][0] = c.g[SP][0];
    c.g[A3][0] = Addr(r32(c, V1)).off(0xc).lw(c);
    c.g[A2][0] = sx(s.wrapping_add(arg));
    c.lwc1(0, r32(c, V0).wrapping_add(0x10));
    c.cvt_s_w(0, 0);
    c.lwc1(1, r32(c, GP).wrapping_sub(k_off));
    c.g[A0][0] = Addr(r32(c, A3)).off(0x158).lh(c);
    c.mul_s(0, 0, 1);
    c.g[A0][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
    racw_scale(c, V1, 0, src, s.wrapping_add(tmp));
    c.lq(V0, s.wrapping_add(tmp));
    c.sq(V0, s.wrapping_add(arg));
    c.g[V1][0] = Addr(r32(c, A3)).off(0x15c).lw(c);
    let method = r32(c, V1);
    call_ptr(c, method, ret)
}

/// `void RiderWipeoutState_PostUpdateRootPose(RiderWipeoutState* self)`
/// (0x137138): with the body active, rebuild the board spheres and test
/// them against the world. On a contact at distance `d >= 0`: push the
/// body, its sphere tree and swept segment out along the hit normal by
/// `d * k`. Hitting hard (`-n . vel >= k1`) or a wall (`n.z < k2`)
/// bounces: the velocity gains `n * (v + max(v / 2, k3))`, a hit object
/// is pushed by the impulse, the spin is damped and, if it has slowed below
/// k4, re-randomised (a random direction times a random 45-180 degrees).
/// Otherwise the body comes to rest (a hit object is pushed by the
/// velocity; velocity and spin take the zero vector at 0x4ff120).
pub fn rider_wipeout_state_post_update_root_pose(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x470);
        frame.save(c, S1, 0x440);
        frame.save(c, S0, 0x450);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x430);
        frame.save_ra(c, 0x420);
        frame.save_fpr(c, 21, 0x468);
        frame.save_fpr(c, 20, 0x460);
        let this = r32(c, S1);
        c.g[V0][0] = Addr(this).off(PURP_BODY).lw(c);
        c.g[V1][0] = Addr(r32(c, V0)).off(PURP_BODY_ACTIVE).lw(c);
        frame.restore(c, S0, 0x450);
        'done: {
            if c.g[V1][0] == 0 {
                break 'done;
            }
            c.cycles += 27;
            PURP_BUILD_SPHERES.call(c, 0x0013_7170)?;
            let this = r32(c, S1);
            c.g[S0][0] = sx(sp(c).wrapping_add(PURP_SPHERE));
            c.g[A1][0] = Addr(this).off(PURP_TREE).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = 1;
            call_ptr(c, PURP_COLL_HULL_CTOR, 0x0013_7184)?;
            // self->seg = the sphere's segment; d = FindClosestContact(
            // world->collision, &sphere, &hit, 0)
            let (this, s) = (r32(c, S1), sp(c));
            c.g[S2][0] = c.g[S0][0];
            c.lq(A0, s + 0xb0);
            c.g[A1][0] = c.g[S0][0];
            c.lq(V1, s + 0xa0);
            c.g[A2][0] = c.g[SP][0];
            c.sq(A0, this.wrapping_add(PURP_SEG));
            c.g[A3][0] = 0;
            c.sq(V1, this.wrapping_add(PURP_SEG + 0x10));
            c.sq(A0, s + 0x3a0);
            c.sq(V1, s + 0x3b0);
            c.g[V0][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(0x84).lw(c);
            c.g[A0][0] = Addr(r32(c, V1)).off(0x20).lw(c);
            call_ptr(c, PURP_FIND_CLOSEST_CONTACT, 0x0013_71bc)?;
            c.mov_s(1, 0);
            c.mtc1(0, 0);
            c.c_lt_s(1, 0);
            let missed = c.fcc();
            c.g[A0][0] = c.g[S2][0];
            if missed {
                c.cycles += 4;
                c.g[A1][0] = 2;
                call_ptr(c, PURP_COLL_HULL_DTOR, 0x0013_71dc)?;
                frame.restore(c, S0, 0x450);
                break 'done;
            }

            // push = n * k * d: the body, its tree and the segment move.
            c.cycles += 58;
            let s = sp(c);
            c.lwc1(0, r32(c, GP).wrapping_sub(0x72ac));
            c.g[A1][0] = sx(s + 0x3c0);
            racw_scale(c, V1, 0, s + PURP_HIT_NORMAL, s + 0x3e0);
            c.lq(V0, s + 0x3e0);
            c.sq(V0, s + 0x3d0);
            racw_scale(c, V1, 1, s + 0x3d0, s + 0x3e0);
            c.lq(V0, s + 0x3e0);
            c.g[V1][0] = Addr(r32(c, S1)).off(PURP_BODY).lw(c);
            c.sq(V0, s + 0x3c0);
            let pos = r32(c, V1).wrapping_add(PURP_BODY_POS);
            c.lqc2(3, pos);
            c.lqc2(4, s + 0x3c0);
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, pos);
            c.g[A0][0] = Addr(r32(c, S1)).off(PURP_TREE).lw(c);
            call_ptr(c, PURP_TREE_TRANSLATE, 0x0013_7240)?;
            let (this, s) = (r32(c, S1), sp(c));
            c.lq(V0, this.wrapping_add(PURP_SEG));
            c.lq(V1, this.wrapping_add(PURP_SEG + 0x10));
            c.sq(V0, s + 0x3a0);
            c.sq(V1, s + 0x3b0);
            for seg in [PURP_SEG, PURP_SEG + 0x10] {
                if seg != PURP_SEG {
                    c.g[AT][0] = 0xffff_ffff_bf80_0000;
                    c.mtc1(AT, 0);
                }
                c.lqc2(3, this.wrapping_add(seg));
                c.lqc2(4, s + 0x3c0);
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, this.wrapping_add(seg));
            }
            // f2 = -n . vel
            c.lwc1(1, r32(c, GP).wrapping_sub(0x72a8));
            racw_scale(c, V1, 0, s + PURP_HIT_NORMAL, s + 0x3e0);
            c.lq(V0, s + 0x3e0);
            c.sq(V0, s + 0x3d0);
            c.lqc2(3, s + 0x3d0);
            c.lqc2(5, this.wrapping_add(PURP_VEL));
            racw_dot_vf3_vf5(c);
            c.qmfc2(V1, 4);
            c.mtc1(V1, 2);
            c.c_le_s(1, 2);
            let mut bounce = c.fcc();
            c.lwc1(1, s + PURP_HIT_NORMAL + 8);
            if !bounce {
                c.cycles += 5;
                c.lwc1(0, r32(c, GP).wrapping_sub(0x72a4));
                c.c_lt_s(1, 0);
                bounce = c.fcc();
                c.g[A0][0] = Addr(s + PURP_HIT_OBJECT).lw(c);
            }

            if bounce {
                purp_bounce(c)?;
            } else {
                // Come to rest.
                c.cycles += 2;
                c.g[V0][0] = 0x50_0000;
                if c.g[A0][0] != 0 {
                    let vel = r32(c, S1).wrapping_add(PURP_VEL);
                    purp_push_object(c, 0x727c, vel, 0x3e0, 0x3d0, 0x0013_7508)?;
                    c.cycles += 1;
                    c.g[V0][0] = 0x50_0000;
                }
                c.cycles += 6;
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xee0));
                let this = r32(c, S1);
                c.lq(V1, r32(c, V0));
                c.sq(V1, this.wrapping_add(PURP_VEL));
                c.lq(A0, r32(c, V0));
                c.sq(A0, this.wrapping_add(PURP_ANG_VEL));
                c.g[A0][0] = c.g[S2][0];
            }

            c.cycles += 3;
            c.g[A1][0] = 2;
            call_ptr(c, PURP_COLL_HULL_DTOR, 0x0013_752c)?;
            frame.restore(c, S0, 0x450);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x440);
        frame.restore(c, S2, 0x430);
        frame.restore_ra(c, 0x420);
        frame.restore_fpr(c, 21, 0x468);
        frame.restore_fpr(c, 20, 0x460);
        frame.close(c);
        Ok(())
    })
}

/// The bounce of `RiderWipeoutState_PostUpdateRootPose` (f2 = v): the
/// impulse, the pushed hit object, the damped and maybe re-randomised
/// spin. Ends with a0 = the sphere (s2).
pub(crate) fn purp_bounce(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 8;
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(AT, 0);
    c.lwc1(1, r32(c, GP).wrapping_sub(0x72a0));
    c.mul_s(0, 2, 0);
    c.c_lt_s(0, 1);
    if c.fcc() {
        c.mov_s(0, 1);
    }
    // vel += n * (v + max(v / 2, k3))
    c.cycles += 15;
    c.add_s(0, 2, 0);
    let s = sp(c);
    racw_scale(c, V1, 0, s + PURP_HIT_NORMAL, s + 0x3e0);
    c.lq(V0, s + 0x3e0);
    c.sq(V0, s + 0x3d0);
    let vel = r32(c, S1).wrapping_add(PURP_VEL);
    c.lqc2(3, vel);
    c.lqc2(4, s + 0x3d0);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, vel);
    c.g[A0][0] = Addr(s + PURP_HIT_OBJECT).lw(c);
    if c.g[A0][0] != 0 {
        purp_push_object(c, 0x729c, s + 0x3d0, 0x3f0, 0x3e0, 0x0013_7394)?;
    }

    // spin *= k; if |spin| < k4: re-randomise it.
    c.cycles += 21;
    let spin = r32(c, S1).wrapping_add(PURP_ANG_VEL);
    c.lwc1(0, r32(c, GP).wrapping_sub(0x7298));
    racw_scale(c, V0, 0, spin, spin);
    c.lwc1(1, r32(c, GP).wrapping_sub(0x7294));
    c.lqc2(3, spin);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(V0, 22);
    c.mtc1(V0, 0);
    c.c_lt_s(0, 1);
    let slow = c.fcc();
    c.g[A0][0] = c.g[S2][0];
    if !slow {
        return Ok(());
    }
    c.cycles += 48;
    let gp = r32(c, GP);
    c.lwc1(12, gp.wrapping_sub(0x7290));
    c.lwc1(13, gp.wrapping_sub(0x728c));
    call_ptr(c, PURP_RAND_RANGE, 0x0013_73f8)?;
    let gp = r32(c, GP);
    c.lwc1(13, gp.wrapping_sub(0x7288));
    c.lwc1(12, gp.wrapping_sub(0x7284));
    c.mov_s(20, 0);
    call_ptr(c, PURP_RAND_RANGE, 0x0013_7408)?;
    c.mov_s(21, 0);
    let s = sp(c);
    c.g[A0][0] = sx(s + 0x410);
    c.g[A1][0] = sx(s + 0x414);
    c.mov_s(12, 20);
    call_ptr(c, PURP_SIN_COS, 0x0013_741c)?;
    c.mov_s(12, 21);
    let s = sp(c);
    c.g[A0][0] = sx(s + 0x418);
    c.g[A1][0] = sx(s + 0x41c);
    call_ptr(c, PURP_SIN_COS, 0x0013_742c)?;
    c.g[AT][0] = 0x4234_0000;
    c.mtc1(AT, 12);
    c.g[AT][0] = 0x4334_0000;
    c.mtc1(AT, 13);
    call_ptr(c, PURP_RAND_RANGE, 0x0013_7444)?;
    // spin += (cp * sy, cp * cy, sp, 0) * deg * (pi / 180)
    let s = sp(c);
    c.lwc1(1, s + 0x414);
    c.lwc1(2, s + 0x418);
    c.lwc1(4, s + 0x41c);
    c.mul_s(2, 1, 2);
    c.lwc1(5, s + 0x410);
    c.mul_s(1, 1, 4);
    c.lwc1(3, r32(c, GP).wrapping_sub(0x7280));
    c.swc1(5, s + 0x3f8);
    c.swc1(2, s + 0x3f0);
    c.mul_s(0, 0, 3);
    c.swc1(1, s + 0x3f4);
    Addr(s + 0x3fc).sw(c, 0);
    racw_scale(c, V1, 0, s + 0x3f0, s + 0x400);
    c.lq(V0, s + 0x400);
    c.sq(V0, s + 0x3e0);
    let spin = r32(c, S1).wrapping_add(PURP_ANG_VEL);
    c.lqc2(3, spin);
    c.lqc2(4, s + 0x3e0);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, spin);
    c.g[A0][0] = c.g[S2][0];
    Ok(())
}

// ---------------------------------------------------------------------------
// RiderWipeoutState_PostUpdateGround (0x138640)

/// The wipeout state: landed flag (+0), pending flag (+4), the rider
/// (+0x40). Rider fields: velocity, ground normal, overlap list, wall
/// hint, sphere tree.
pub(crate) const PUG_RIDER: u32 = 0x40;

pub(crate) const PUG_R_VEL: u32 = 0x1e0;

pub(crate) const PUG_R_GROUND_NORMAL: u32 = 0x370;

pub(crate) const PUG_R_OVERLAP: u32 = 0x860;

pub(crate) const PUG_R_WALL_HINT: u32 = 0x868;

pub(crate) const PUG_R_SPHERE_TREE: u32 = 0xaa0;

/// Stack frame: the hit (normal +0x10, contact velocity +0x20, object
/// +0x50), the sphere at +0x80, vector temporaries from +0x3a0.
pub(crate) const PUG_HIT_NORMAL: u32 = 0x10;

pub(crate) const PUG_HIT_VEL: u32 = 0x20;

pub(crate) const PUG_HIT_OBJECT: u32 = 0x50;

pub(crate) const PUG_SPHERE: u32 = 0x80;

pub(crate) const PUG_COLL_HULL_CTOR: u32 = 0x0032_f650;

pub(crate) const PUG_FIND_CLOSEST_CONTACT: u32 = 0x0033_42d0;

pub(crate) const PUG_TRANSLATE: u32 = 0x0010_6538;

pub(crate) const PUG_UPDATE_BOUNDS: u32 = 0x0011_e150;

pub(crate) const PUG_WALL_COLLISION: u32 = 0x0010_5398;

pub(crate) const PUG_RIDER_COLLISIONS: u32 = 0x0010_7888;

pub(crate) const PUG_COLL_HULL_DTOR: u32 = 0x0032_f708;

/// `self->rider` (s1 = self).
pub(crate) fn pug_rider(c: &mut Cpu) -> u64 {
    Addr(r32(c, S1)).off(PUG_RIDER).lw(c)
}

/// `void RiderWipeoutState_PostUpdateGround(RiderWipeoutState* self)`
/// (0x138640): the sliding rider's sphere (along the ground normal) against
/// the world. A contact at `d >= 0` whose normal is not opposite the
/// ground (`ground . n > k`): a normal leaning into the ground is
/// flattened (the ground part removed, renormalised), a hit object's
/// owner is told (vtable slot 0x150), the rider moves out by `n * k2 * d`,
/// and moving into the contact (`-n . (vel - contact vel) >= 0`) bounces
/// him (`vel += n * (v + clamp(v / 2, lo, hi))`). Then bounds, wall and
/// rider collisions; a pending flag becomes the landed flag; a velocity
/// slower than 50 stops; the hull is destroyed.
pub fn rider_wipeout_state_post_update_ground(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x430);
        c.g[A2][0] = 1;
        frame.save(c, S0, 0x410);
        frame.save(c, S1, 0x400);
        c.g[S0][0] = sx(sp(c).wrapping_add(PUG_SPHERE));
        frame.save(c, S2, 0x3f0);
        c.g[S1][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x420);
        c.g[A0][0] = c.g[S0][0];
        frame.save_ra(c, 0x3e0);
        c.g[S2][0] = c.g[S0][0];
        c.g[V0][0] = pug_rider(c);
        c.g[A1][0] = Addr(r32(c, V0)).off(PUG_R_SPHERE_TREE).lw(c);
        call_ptr(c, PUG_COLL_HULL_CTOR, 0x0013_8678)?;
        // sphere.dir = ground normal; d = FindClosestContact(overlap,
        // &sphere, &hit, wall_hint)
        c.g[V1][0] = pug_rider(c);
        let (rider, s) = (r32(c, V1), sp(c));
        c.g[A0][0] = 1;
        Addr(s + 0x88).sw(c, 1);
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[SP][0];
        c.lq(V0, rider.wrapping_add(PUG_R_GROUND_NORMAL));
        c.sq(V0, s + 0x90);
        c.g[A3][0] = Addr(rider).off(PUG_R_WALL_HINT).lw(c);
        c.g[A0][0] = Addr(rider).off(PUG_R_OVERLAP).lw(c);
        call_ptr(c, PUG_FIND_CLOSEST_CONTACT, 0x0013_86a0)?;
        c.mtc1(0, 2);
        c.mov_s(20, 0);
        c.c_le_s(2, 20);
        if c.fcc() {
            pug_contact(c)?;
        }
        c.g[A0][0] = pug_rider(c);

        c.cycles += 10;
        c.g[A1][0] = 0;
        call_ptr(c, PUG_UPDATE_BOUNDS, 0x0013_88c0)?;
        c.g[A0][0] = pug_rider(c);
        c.g[A1][0] = 0;
        call_ptr(c, PUG_WALL_COLLISION, 0x0013_88cc)?;
        c.g[A0][0] = pug_rider(c);
        call_ptr(c, PUG_RIDER_COLLISIONS, 0x0013_88d4)?;
        let this = r32(c, S1);
        c.g[V0][0] = Addr(this).off(4).lw(c);
        let pending = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if pending {
            c.cycles += 2;
            Addr(this).off(4).sw(c, 0);
            Addr(this).sw(c, 1);
        }

        // Stop below 50.
        c.cycles += 17;
        c.g[A0][0] = pug_rider(c);
        let vel = r32(c, A0).wrapping_add(PUG_R_VEL);
        c.g[AT][0] = 0x4248_0000;
        c.mtc1(AT, 0);
        c.lqc2(3, vel);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(V0, 22);
        c.mtc1(V0, 1);
        c.c_lt_s(1, 0);
        let slow = c.fcc();
        c.g[V0][0] = 0x50_0000;
        if slow {
            c.cycles += 3;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xee0));
            c.lq(V1, r32(c, V0));
            c.sq(V1, vel);
        }

        c.cycles += 10;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 2;
        call_ptr(c, PUG_COLL_HULL_DTOR, 0x0013_8944)?;
        frame.restore(c, S0, 0x410);
        frame.restore(c, S1, 0x400);
        frame.restore(c, S2, 0x3f0);
        frame.restore_ra(c, 0x3e0);
        frame.restore_fpr(c, 20, 0x420);
        frame.close(c);
        Ok(())
    })
}

/// The contact of `RiderWipeoutState_PostUpdateGround` (f20 = d).
pub(crate) fn pug_contact(c: &mut Cpu) -> crate::call::Flow {
    // f1 = ground . n
    c.cycles += 15;
    c.g[V1][0] = pug_rider(c);
    let (rider, s) = (r32(c, V1), sp(c));
    c.lwc1(0, r32(c, GP).wrapping_sub(0x722c));
    c.lqc2(3, rider.wrapping_add(PUG_R_GROUND_NORMAL));
    c.lqc2(5, s + PUG_HIT_NORMAL);
    racw_dot_vf3_vf5(c);
    c.qmfc2(V0, 4);
    c.mtc1(V0, 1);
    c.c_lt_s(0, 1);
    if !c.fcc() {
        return Ok(());
    }
    c.cycles += 4;
    c.c_lt_s(1, 2);
    let into_ground = c.fcc();
    c.g[A0][0] = Addr(s + PUG_HIT_OBJECT).lw(c);
    if into_ground {
        // n = normalize(n - ground * (ground . n))
        c.cycles += 27;
        racw_scale(c, V0, 1, rider.wrapping_add(PUG_R_GROUND_NORMAL), s + 0x3d0);
        c.lq(V0, s + 0x3d0);
        c.sq(V0, s + 0x3c0);
        c.lqc2(3, s + PUG_HIT_NORMAL);
        c.lqc2(4, s + 0x3c0);
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, s + 0x3d0);
        c.lq(V0, s + 0x3d0);
        c.sq(V0, s + 0x3b0);
        c.lqc2(3, s + 0x3b0);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().rsqrt(0, 3, 4, 0);
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 3, Src::Q);
        c.sqc2(5, s + 0x3d0);
        c.lq(V0, s + 0x3d0);
        c.sq(V0, s + PUG_HIT_NORMAL);
        c.sq(V0, s + 0x3a0);
        c.g[A0][0] = Addr(s + PUG_HIT_OBJECT).lw(c);
    }
    // hit.object->owner->OnContact(&hit) (vtable slot 0x150)
    c.cycles += 2;
    if c.g[A0][0] != 0 {
        c.cycles += 3;
        c.g[A2][0] = Addr(r32(c, A0)).off(0xc).lw(c);
        c.g[A1][0] = c.g[SP][0];
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            let owner = r32(c, A2);
            c.g[V0][0] = Addr(owner).off(0xc).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).off(0x150).lh(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(0x154).lw(c);
            let method = r32(c, V1);
            c.g[A0][0] = sx(owner.wrapping_add(r32(c, A0)));
            call_ptr(c, method, 0x0013_8798)?;
        }
    }

    // Move out: n * k2 * d.
    c.cycles += 48;
    let s = sp(c);
    c.lwc1(0, r32(c, GP).wrapping_sub(0x7228));
    c.g[A1][0] = sx(s + 0x3a0);
    racw_scale(c, V1, 0, s + PUG_HIT_NORMAL, s + 0x3c0);
    c.lq(V0, s + 0x3c0);
    c.sq(V0, s + 0x3b0);
    racw_scale(c, V1, 20, s + 0x3b0, s + 0x3c0);
    c.lq(V0, s + 0x3c0);
    c.g[A0][0] = pug_rider(c);
    c.sq(V0, s + 0x3a0);
    call_ptr(c, PUG_TRANSLATE, 0x0013_87e0)?;

    // f3 = -n . (vel - contact vel)
    c.g[V0][0] = pug_rider(c);
    let (rider, s) = (r32(c, V0), sp(c));
    c.lqc2(3, rider.wrapping_add(PUG_R_VEL));
    c.lqc2(4, s + PUG_HIT_VEL);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, s + 0x3c0);
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(AT, 0);
    c.lq(V0, s + 0x3c0);
    c.mtc1(0, 1);
    c.sq(V0, s + 0x3b0);
    racw_scale(c, V1, 0, s + PUG_HIT_NORMAL, s + 0x3d0);
    c.lq(V0, s + 0x3d0);
    c.sq(V0, s + 0x3c0);
    c.lqc2(3, s + 0x3c0);
    c.lqc2(5, s + 0x3b0);
    racw_dot_vf3_vf5(c);
    c.qmfc2(V1, 4);
    c.mtc1(V1, 3);
    c.c_le_s(1, 3);
    if !c.fcc() {
        return Ok(());
    }
    // vel += n * (v + clamp(v / 2, lo, hi))
    c.cycles += 9;
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(AT, 0);
    c.lwc1(1, r32(c, GP).wrapping_sub(0x7224));
    c.mul_s(2, 3, 0);
    c.lwc1(4, r32(c, GP).wrapping_sub(0x7220));
    c.c_le_s(1, 2);
    let above = c.fcc();
    c.mov_s(0, 1);
    if above {
        c.cycles += 1;
        c.min_s(0, 2, 4);
    }
    c.cycles += 14;
    c.add_s(0, 3, 0);
    racw_scale(c, V1, 0, s + PUG_HIT_NORMAL, s + 0x3d0);
    c.lq(V0, s + 0x3d0);
    c.g[V1][0] = pug_rider(c);
    c.sq(V0, s + 0x3c0);
    let vel = r32(c, V1).wrapping_add(PUG_R_VEL);
    c.lqc2(3, vel);
    c.lqc2(4, s + 0x3c0);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, vel);
    Ok(())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::elf_image;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;

    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::ai::tests::leaf;

    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear;

    pub(crate) use crate::game::wscript::wscriptman::tests::w;

    #[test]
    fn rider_wipeout_state_exit_matches() {
        leaf(
            ssx3_ee::s01::f_00136f28__generated,
            rider_wipeout_state_exit,
            15,
        );
    }

    fn wipeout_board_spheres_setup(c: &mut Cpu, seed: u32) {
        use crate::difftest::put_vec;

        const RIDER: u32 = DATA + 0x1000;

        const MODEL: u32 = DATA + 0x2000;

        const BONES: u32 = DATA + 0x3000;

        const TREE: u32 = DATA + 0x4000;

        let mut rng = machine(c, seed, 0x137550, 0);
        c.pc = 0x0013_7550;
        c.g[A0][0] = DATA as u64;
        c.bus.write32(DATA + 0x40, RIDER);
        c.bus.write32(DATA + 0x44, TREE);
        c.bus.write32(RIDER + 0x780, MODEL);
        c.bus.write32(RIDER + 0x8a4, seed % 4);
        c.bus.write32(MODEL + 0x2c, BONES);
        c.bus.write32(
            MODEL + 0x140,
            [0.0f32, 0.5, 1.0, 1.5, -1.0][(seed / 4 % 5) as usize].to_bits(),
        );
        for i in 0..4 {
            put_vec(
                c,
                BONES + i * 0x20,
                [
                    rng.f32(100.0),
                    rng.f32(100.0),
                    rng.f32(100.0),
                    1.0f32.to_bits(),
                ],
            );
            put_vec(
                c,
                BONES + i * 0x20 + 0x10,
                [rng.f32(1.0), rng.f32(1.0), rng.f32(1.0), rng.f32(1.0)],
            );
            // All four table entries are live memory inputs, not baked radii.
            c.bus.write32(0x458220 + i * 4, rng.f32(10.0));
        }
    }

    #[test]
    fn rider_wipeout_state_build_board_spheres_matches() {
        crate::difftest::check(
            ee::s01::f_00137550__generated,
            rider_wipeout_state_build_board_spheres,
            160,
            &wipeout_board_spheres_setup,
        );
        // The caller is straight-line; its real SetSpheres callee visits all
        // four children and the root on every seed (127 + 3 + 1 + 44 + 4).
        crate::difftest::require_paths(
            ee::s01::f_00137550__generated,
            160,
            &[179],
            &wipeout_board_spheres_setup,
        );
        for seed in 0..20 {
            let mut c = crate::difftest::fresh();
            wipeout_board_spheres_setup(&mut c, seed);
            crate::difftest::run_generated(&mut c, rider_wipeout_state_build_board_spheres);
            assert_eq!(c.pc, crate::difftest::RA);
            let tree = crate::difftest::DATA + 0x4000;
            assert_eq!(c.bus.read32(tree + 0x2c), 4);
            for i in 0..4 {
                assert_eq!(
                    c.bus.read32(tree + 0x40 + i * 0x20),
                    c.bus.read32(0x458220 + i * 4)
                );
            }
        }
    }

    // ------------------------------------------------------ rider wipeout

    #[test]
    fn rider_wipeout_state_enable_root_pose_override_matches() {
        check(
            ssx3_ee::s01::f_00136d40__generated,
            rider_wipeout_state_enable_root_pose_override,
            12,
            &|c, seed| {
                let mut r = base(c, seed, 0x0013_6d40);
                let rider = DATA + 0x100;
                let model = DATA + 0x2000;
                let pose = DATA + 0x3000;
                let offset = DATA + 0x3100;
                let root = DATA + 0x3200;
                clear(c, rider, 0x100);
                clear(c, model, 0x400);
                for a in [pose, offset, root] {
                    for k in 0..8 {
                        w(c, a + 4 * k, r.next_u32());
                    }
                }
                w(c, rider + 0x40, model);
                w(c, rider, (seed % 3 == 0) as u32);
                c.g[A0][0] = sx(rider);
                c.g[A1][0] = sx(offset);
                c.g[A2][0] = sx(root);
                c.g[A3][0] = sx(pose);
            },
        );
    }

    use crate::difftest::fill;

    /// `RiderWipeoutState_UpdateRootPose`: active and inactive riders,
    /// tumble speeds below and above the 3333.33 clamp, random angular
    /// velocities, root positions and quaternions, rates around 1; the
    /// real `.lit4` constants.
    #[test]
    fn rider_wipeout_state_update_root_pose_matches() {
        use super::wipeout_pose::*;
        use crate::difftest::GAME_GP;
        crate::difftest::check(
            ee::s01::f_00136f30__generated,
            rider_wipeout_state_update_root_pose,
            300,
            &|c, seed| {
                let mut r = Rng(0x0013_6f30 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0013_6f30;
                c.g[GP][0] = GAME_GP as u64;
                put_words(
                    c,
                    GAME_GP - LIT_TICK,
                    &[
                        (1.0f32 / 60.0).to_bits(),
                        (-0.2f32).to_bits(),
                        3333.3335f32.to_bits(),
                    ],
                );
                let (me, rider) = (DATA, DATA + 0x1000);
                c.g[A0][0] = sx(me);
                c.bus.write32(me + RIDER, rider);
                c.bus.write32(
                    rider + R_ACTIVE,
                    if seed % 5 == 0 { 0 } else { r.next_u32() | 1 },
                );
                let speed = if seed % 2 == 0 { 2000.0 } else { 6000.0 };
                for i in 0..4 {
                    c.bus.write32(me + VEL + 4 * i, r.f32(speed));
                    c.bus.write32(me + ANG_VEL + 4 * i, r.f32(10.0));
                    c.bus.write32(rider + R_POS + 4 * i, r.f32(5000.0));
                    c.bus.write32(rider + R_QUAT + 4 * i, r.f32(1.0));
                }
                c.bus.write32(
                    rider + R_RATE,
                    (0.5 + (r.next_u32() % 100) as f32 / 50.0).to_bits(),
                );
            },
        );
    }

    use crate::difftest::put_vec;

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
        check(
            ee::s01::f_00138960__generated,
            rider_wipeout_state_cast_ground_ray,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x13_8960, 1);
                elf_image(c);
                c.pc = 0x0013_8960;
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(HIT);
                c.g[A2][0] = sx(DIR);
                put_vec(c, DIR, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
                c.bus.write32(SELF + WO_RIDER, RIDER);
                put_vec(
                    c,
                    RIDER + WO_R_POS,
                    [r.f32(100.0), r.f32(100.0), r.f32(100.0), 0x3f80_0000],
                );
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
                    let handler = if (seed / 4 + k) % 3 == 0 {
                        0
                    } else {
                        obj + 0x80
                    };
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
            },
        );
    }

    use crate::difftest::fakes::{self, FAKE};

    // ------------------------------------------- RiderWipeoutState_PostUpdateAir

    /// Control block of the gated fakes below: magic word (without it they
    /// run the generated body, so other tests are unaffected), the contact
    /// distance `FindClosestContact` returns, the ray distance
    /// `CastGroundRay` returns, and the hits they write.
    pub(crate) const PUA_CTRL: u32 = DATA + 0x5_8000;

    pub(crate) const PUA_MAGIC: u32 = 0x1378_60a1;

    pub(crate) const PUA_HIT_A_TEMPLATE: u32 = PUA_CTRL + 0x100;

    pub(crate) const PUA_HIT_B_TEMPLATE: u32 = PUA_CTRL + 0x200;

    /// A logged fake (`fakes::fake`); the two collision queries also return
    /// their chosen distance in f0 and write their hit.
    pub(crate) fn pua_fake(c: &mut Cpu, target: u32) {
        let (a1, a2) = (c.g[A1][0] as u32, c.g[A2][0] as u32);
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, target, ra);
        if c.pc != ra {
            return;
        }
        let (template, hit, dist) = match target {
            PUA_FIND_CLOSEST_CONTACT => (PUA_HIT_A_TEMPLATE, a2, PUA_CTRL + 4),
            0x0013_8960 => (PUA_HIT_B_TEMPLATE, a1, PUA_CTRL + 8),
            _ => return,
        };
        for i in 0..0x18 {
            let w = c.bus.read32(template + 4 * i);
            c.bus.write32(hit + 4 * i, w);
        }
        c.f[0] = c.bus.read32(dist);
    }

    macro_rules! pua_gated {
        ($name:ident, $addr:expr, $gen:path) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(PUA_CTRL) != PUA_MAGIC {
                    return crate::difftest::fakes::fall_back(
                        c,
                        crate::difftest::fakes::prev_cell($name),
                        $gen,
                    );
                }
                pua_fake(c, $addr);
            }
        };
    }

    pua_gated!(
        pua_fake_hull_ctor,
        PUA_COLL_HULL_CTOR,
        ee::s11::f_0032f650__generated
    );

    pua_gated!(
        pua_fake_closest,
        PUA_FIND_CLOSEST_CONTACT,
        ee::s11::f_003342d0__generated
    );

    pua_gated!(
        pua_fake_hull_dtor,
        PUA_COLL_HULL_DTOR,
        ee::s11::f_0032f708__generated
    );

    pua_gated!(
        pua_fake_clear_air,
        PUA_CLEAR_AIR_TIME,
        ee::s00::f_00113998__generated
    );

    pua_gated!(
        pua_fake_translate,
        PUA_TRANSLATE,
        ee::s00::f_00106538__generated
    );

    pua_gated!(
        pua_fake_bounds,
        PUA_UPDATE_BOUNDS,
        ee::s00::f_0011e150__generated
    );

    pua_gated!(
        pua_fake_wall,
        PUA_WALL_COLLISION,
        ee::s00::f_00105398__generated
    );

    pua_gated!(
        pua_fake_riders,
        PUA_RIDER_COLLISIONS,
        ee::s00::f_00107888__generated
    );

    pua_gated!(
        pua_fake_prime,
        PUA_AIR_PRIME,
        ee::s00::f_001135b8__generated
    );

    pua_gated!(pua_fake_spray, 0x002e_23e0, ee::s09::f_002e23e0__generated);

    pua_gated!(
        pua_fake_ground_ray,
        0x0013_8960,
        ee::s01::f_00138960__generated
    );

    /// Hook the gated fakes in their callees' slots (once per test process;
    /// with the gate off each runs what its slot held before).
    pub(crate) fn pua_install_fakes() {
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s11::OVERRIDE_0032F650),
            pua_fake_hull_ctor,
            crate::difftest::fakes::prev_cell(pua_fake_hull_ctor),
        );
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s11::OVERRIDE_003342D0),
            pua_fake_closest,
            crate::difftest::fakes::prev_cell(pua_fake_closest),
        );
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s11::OVERRIDE_0032F708),
            pua_fake_hull_dtor,
            crate::difftest::fakes::prev_cell(pua_fake_hull_dtor),
        );
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s00::OVERRIDE_00113998),
            pua_fake_clear_air,
            crate::difftest::fakes::prev_cell(pua_fake_clear_air),
        );
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s00::OVERRIDE_00106538),
            pua_fake_translate,
            crate::difftest::fakes::prev_cell(pua_fake_translate),
        );
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s00::OVERRIDE_0011E150),
            pua_fake_bounds,
            crate::difftest::fakes::prev_cell(pua_fake_bounds),
        );
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s00::OVERRIDE_00105398),
            pua_fake_wall,
            crate::difftest::fakes::prev_cell(pua_fake_wall),
        );
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s00::OVERRIDE_00107888),
            pua_fake_riders,
            crate::difftest::fakes::prev_cell(pua_fake_riders),
        );
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s00::OVERRIDE_001135B8),
            pua_fake_prime,
            crate::difftest::fakes::prev_cell(pua_fake_prime),
        );
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s09::OVERRIDE_002E23E0),
            pua_fake_spray,
            crate::difftest::fakes::prev_cell(pua_fake_spray),
        );
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s01::OVERRIDE_00138960),
            pua_fake_ground_ray,
            crate::difftest::fakes::prev_cell(pua_fake_ground_ray),
        );
    }

    /// The sphere contact missed / hit; the ground ray short, behind, the
    /// rider moving away from or far above the ground, or a landing (hit
    /// object none / ownerless / with a push handler, pushed out of the
    /// ground or not); the push-out path with and without the velocity
    /// kill. The callees are logged fakes (one may unwind); the real
    /// `RiderBase_TriggerDynamicSpray` wrapper runs.
    #[test]
    fn rider_wipeout_state_post_update_air_matches() {
        pua_install_fakes();
        covered(
            ee::s01::f_00137860__generated,
            rider_wipeout_state_post_update_air,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x0013_7860, 12);
                c.pc = 0x0013_7860;
                let state = DATA + 0x5_0000;
                let rider = DATA + 0x5_1000;
                let vt = DATA + 0x5_3000;
                let handler = DATA + 0x5_4000;
                let obj = DATA + 0x5_6000;
                let owner = DATA + 0x5_6100;
                let owner_vt = DATA + 0x5_6200;
                let world = DATA + 0x5_7000;
                let pick = |r: &mut Rng, v: &[f32]| v[r.next_u32() as usize % v.len()].to_bits();
                let comps = [-12.0f32, -3.0, -1.0, -0.5, 0.0, 0.5, 1.0, 3.0, 12.0];
                let vec = |c: &mut Cpu, r: &mut Rng, at: u32| {
                    for i in 0..3 {
                        c.bus.write32(at + 4 * i, pick(r, &comps));
                    }
                    c.bus.write32(at + 12, pick(r, &[0.0, 1.0]));
                };
                c.bus.write32(PUA_CTRL, PUA_MAGIC);
                c.g[A0][0] = sx(state);
                fill(c, &mut r, state, 4);
                c.bus.write32(state + PUA_STATE_RIDER, rider);
                // Rider.
                c.bus.write32(rider + PUA_R_VTABLE, vt);
                c.bus
                    .write16(vt + 0x40, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(vt + 0x44, FAKE);
                for off in [PUA_R_AIR, PUA_R_OVERLAP, PUA_R_WALL_HINT, PUA_R_SPHERE_TREE] {
                    c.bus.write32(rider + off, r.next_u32());
                }
                c.bus.write32(rider + PUA_R_HANDLER, handler);
                vec(c, &mut r, rider + PUA_R_POS);
                vec(c, &mut r, rider + PUA_R_VEL);
                vec(c, &mut r, rider + PUA_R_GROUND_NORMAL);
                // The distances: the sphere contact, then the ground ray.
                c.bus
                    .write32(PUA_CTRL + 4, pick(&mut r, &[-1.0, 0.0, 0.25, 2.0, 2.0]));
                c.bus.write32(
                    PUA_CTRL + 8,
                    pick(&mut r, &[-1.0, 0.2, 0.4, 0.5, 3.0, 3.0, 3.0]),
                );
                fill(c, &mut r, PUA_HIT_A_TEMPLATE, 0x18);
                vec(c, &mut r, PUA_HIT_A_TEMPLATE + 0x10);
                fill(c, &mut r, PUA_HIT_B_TEMPLATE, 0x18);
                vec(c, &mut r, PUA_HIT_B_TEMPLATE);
                vec(c, &mut r, PUA_HIT_B_TEMPLATE + 0x10);
                // Hit object: none, ownerless, or an owner with a push handler.
                let object = match r.next_u32() % 3 {
                    0 => 0,
                    1 => {
                        c.bus.write32(obj + 0xc, 0);
                        obj
                    }
                    _ => {
                        c.bus.write32(obj + 0xc, owner);
                        obj
                    }
                };
                c.bus.write32(PUA_HIT_B_TEMPLATE + 0x50, object);
                c.bus.write32(owner + 0xc, owner_vt);
                c.bus
                    .write16(owner_vt + 0x158, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(owner_vt + 0x15c, FAKE + 0x20);
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x10, r.next_u32() % 5);
            },
        );
    }

    /// RiderWipeoutState_UpdateAir with the real AirPredictor_Update,
    /// Rider_UpdateOrientation and orientation update (game literals from
    /// the ELF image): the predictor in a flight with mode 1, 3 or another
    /// (no re-aim), the time left above or below one tick.
    #[test]
    fn rider_wipeout_state_update_air_matches() {
        use crate::difftest::{elf_image, put_vec};
        const STATE: u32 = DATA;
        const RIDER: u32 = DATA + 0x1000;
        const AP: u32 = DATA + 0x3000;
        check(
            ee::s01::f_00137750__generated,
            rider_wipeout_state_update_air,
            120,
            &|c, seed| {
                let mut r = Rng(0x0013_7750 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                elf_image(c);
                c.g[GP][0] = sx(GAME_GP);
                c.pc = 0x0013_7750;
                let small = |r: &mut Rng, k: f32| f32::from_bits(r.f32(k));
                // The wipeout state: spin vector, rider.
                put_vec(
                    c,
                    STATE + WIPEOUT_STATE_SPIN,
                    [r.f32(3.0), r.f32(3.0), r.f32(3.0), r.f32(1.0)],
                );
                c.bus.write32(STATE + WIPEOUT_STATE_RIDER, RIDER);
                // The rider: time scale ~1, unit orientation, position/velocity.
                c.bus.write32(
                    RIDER + WIPEOUT_RIDER_TIME_SCALE,
                    (1.0 + small(&mut r, 0.5)).to_bits(),
                );
                c.bus.write32(RIDER + WIPEOUT_RIDER_AIR_PREDICTOR, AP);
                put_vec(
                    c,
                    RIDER + 0x110,
                    [r.f32(100.0), r.f32(100.0), r.f32(100.0), 0x3f80_0000],
                );
                put_vec(c, RIDER + 0x1e0, [r.f32(10.0), r.f32(10.0), r.f32(10.0), 0]);
                for o in [0x100u32, 0x120, 0x130, 0x140] {
                    put_vec(c, RIDER + o, [0, 0, 0, 0x3f80_0000]);
                }
                // The predictor: a flight in progress (as AirPredictor_Update's test).
                for o in (0..0xb0).step_by(4) {
                    c.bus.write32(AP + o, 0);
                }
                for i in 0..3 {
                    c.bus
                        .write32(AP + 0x70 + 4 * i, small(&mut r, 1000.0).to_bits());
                    c.bus
                        .write32(AP + 0x80 + 4 * i, small(&mut r, 500.0).to_bits());
                    c.bus
                        .write32(AP + 0x10 + 4 * i, small(&mut r, 1.0).to_bits());
                    c.bus
                        .write32(AP + 0x20 + 4 * i, small(&mut r, 1.0).to_bits());
                }
                c.bus.write32(AP + 0x7c, 1.0f32.to_bits());
                c.bus.write32(AP + 0xa8, 2000.0f32.to_bits());
                c.bus
                    .write32(AP + WIPEOUT_AP_MODE, [1u32, 3, 0, 2][(seed % 4) as usize]);
                let now = (seed % 7) as f32 / 60.0;
                let end = now
                    + if seed % 3 == 0 {
                        0.005
                    } else {
                        0.5 + (seed % 5) as f32
                    };
                c.bus.write32(AP + 0xa0, now.to_bits());
                c.bus.write32(AP + 0xa4, now.to_bits());
                c.bus.write32(AP + 0x98, end.to_bits());
                c.g[A0][0] = sx(STATE);
            },
        );
    }

    pub(crate) use crate::game::ai::ai::tests::save_state_fixture;

    #[test]
    fn rider_wipeout_state_save_state_matches() {
        crate::difftest::fakes::covered(
            ee::s01::f_00138ad8__generated,
            rider_wipeout_state_save_state,
            300,
            &|c, seed| {
                save_state_fixture(c, seed, 0x0013_8ad8);
                c.pc = 0x0013_8ad8;
            },
        );
    }

    /// `RiderWipeoutState_Enter`: with and without a sphere tree (the pool
    /// allocator runs over a test heap) and both the crash (normal slide) and
    /// the launch (air predictor) paths.
    #[test]
    fn rider_wipeout_state_enter_matches() {
        check(
            ee::s01::f_00136c40__generated,
            rider_wipeout_state_enter,
            200,
            &|c, seed| {
                let mut r = Rng(0x136c_4000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                let motion = DATA;
                let rider = DATA + 0x800;
                let has_tree = seed % 4 != 1;
                if !has_tree {
                    // `DLHeap_InitArena(&MEM_HEAP, arena, 0x40000)`: the game heap
                    // over test memory, for the allocation path.
                    c.g[A0][0] = 0x004f_f250;
                    c.g[A1][0] = (DATA + 0x4_0000) as u64;
                    c.g[A2][0] = 0x4_0000;
                    c.g[RA][0] = crate::difftest::RA as u64;
                    c.pc = 0x0031_ff60;
                    ee::s10::DLHeap_InitArena(c);
                }
                c.g[A0][0] = motion as u64;
                c.g[A1][0] = if seed % 3 == 0 { 1 } else { 0 };
                c.bus.write32(motion + 0x40, rider);
                c.bus
                    .write32(motion + 0x44, if has_tree { DATA + 0x1800 } else { 0 });
                for off in [0x110u32, 0x1e0, 0x370] {
                    put_vec(
                        c,
                        rider + off,
                        [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()],
                    );
                }
                c.bus.write32(rider + 0x788, DATA + 0x2000);
                c.bus.write32(rider + 0x2dc, r.next_u32());
                c.bus.write32(rider + 0x2e0, r.next_u32());
                put_words(
                    c,
                    GAME_GP.wrapping_sub(0x72c0),
                    &[RIDER_WIPEOUT_STATE_ENTER_LAUNCH_SPEED],
                );
            },
        );
    }

    // --------------------------------------- RiderWipeoutState_SetVelocity

    /// RiderWipeoutState_SetVelocity: launched (the real AirPredictor_Prime,
    /// its constants written by hand) or not (the VU0 projection onto the up
    /// vector), with random angular velocity, velocity, position and up
    /// vector.
    #[test]
    fn rider_wipeout_state_set_velocity_matches() {
        const SELF: u32 = DATA;
        const RIDER: u32 = DATA + 0x1000;
        const VEL: u32 = DATA + 0x2000;
        const ANG_VEL: u32 = DATA + 0x2100;
        const AIR: u32 = DATA + 0x2200;
        check(
            ee::s01::f_00136de0__generated,
            rider_wipeout_state_set_velocity,
            200,
            &|c, seed| {
                let mut r = Rng(0x0013_6de0 ^ (seed as u64) << 8);
                crate::difftest::scramble(c, &mut r);
                c.pc = 0x0013_6de0;
                c.g[GP][0] = GAME_GP as u64;
                // The air predictor's constants (reset and launch state).
                for i in 0..4u32 {
                    c.bus.write32(0x4ff160 + 4 * i, r.next_u32());
                    c.bus.write32(0x4ff120 + 4 * i, r.next_u32());
                }
                for i in 0..2u32 {
                    c.bus.write32(0x4a5960 + 4 * i, r.next_u32());
                }
                put_words(
                    c,
                    GAME_GP.wrapping_sub(OC326_RWSV_LAUNCH_SPEED_LIT),
                    &[0x4550_5556],
                );
                // The rider's vectors and its air predictor.
                put_words(
                    c,
                    RIDER + OC326_RWSV_POS,
                    &[r.f32(100.0), r.f32(100.0), r.f32(100.0), 0],
                );
                put_words(
                    c,
                    RIDER + OC326_RWSV_VEL,
                    &[r.f32(100.0), r.f32(100.0), r.f32(100.0), 0],
                );
                put_words(
                    c,
                    RIDER + OC326_RWSV_UP,
                    &[r.f32(1.0), r.f32(1.0), r.f32(1.0), 0],
                );
                put_words(c, RIDER + OC326_RWSV_AIR, &[AIR]);
                put_words(c, SELF + OC326_RWSV_RIDER, &[RIDER]);
                put_words(c, SELF, &[if seed % 2 == 0 { 0 } else { r.next_u32() | 1 }]);
                put_words(c, VEL, &[r.f32(100.0), r.f32(100.0), r.f32(100.0), 0]);
                put_words(c, ANG_VEL, &[r.f32(5.0), r.f32(5.0), r.f32(5.0), 0]);
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(VEL);
                c.g[A2][0] = sx(ANG_VEL);
            },
        );
    }

    use std::sync::atomic::AtomicUsize;

    /// Gate of this test's fakes (off: the slot's previous body runs).
    pub(crate) const WUG_CTRL: u32 = DATA + 0x6_8000;

    pub(crate) const WUG_MAGIC: u32 = 0x137d_18a5;

    macro_rules! wug_gated {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(WUG_CTRL) != WUG_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                fakes::fake_call(c, $addr);
            }
        };
    }

    pub(crate) static WUG_PREV_LAYER_ANIM: AtomicUsize = AtomicUsize::new(0);

    /// `AnimController_GetLayerAnimEntry`: logged, returns the anim id the
    /// fixture puts at `WUG_CTRL + 4`.
    pub(crate) fn wug_fake_layer_anim(c: &mut Cpu) {
        if c.bus.read32(WUG_CTRL) != WUG_MAGIC {
            return fakes::fall_back(c, &WUG_PREV_LAYER_ANIM, ee::s10::f_00311ae8__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, WUG_GET_LAYER_ANIM);
        if c.pc == ra {
            c.g[V0][0] = c.bus.read32(WUG_CTRL + 4) as u64;
        }
    }

    wug_gated!(
        wug_fake_orientation,
        WUG_PREV_ORIENTATION,
        WUG_UPDATE_ORIENTATION,
        ee::s00::f_00121aa0__generated
    );

    wug_gated!(
        wug_fake_spray,
        WUG_PREV_SPRAY,
        WUG_TRIGGER_SPRAY,
        ee::s00::f_00111aa0__generated
    );

    wug_gated!(
        wug_fake_check_reset,
        WUG_PREV_CHECK_RESET,
        WUG_CHECK_RESET,
        ee::s00::f_00116120__generated
    );

    wug_gated!(
        wug_fake_world_matrix,
        WUG_PREV_WORLD_MATRIX,
        WUG_BUILD_WORLD_MATRIX,
        ee::s00::f_0011e098__generated
    );

    #[allow(unused_unsafe)]
    pub(crate) fn wug_install_fakes() {
        use std::ptr::addr_of_mut;
        // The ground ray and the air launch: PostUpdateAir's gated fakes.
        crate::game::ai::motion::wipeoutmotion::tests::pua_install_fakes();
        unsafe {
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_00311AE8),
                wug_fake_layer_anim,
                &WUG_PREV_LAYER_ANIM,
            );
            fakes::hook(
                addr_of_mut!(ee::s00::OVERRIDE_00121AA0),
                wug_fake_orientation,
                &WUG_PREV_ORIENTATION,
            );
            fakes::hook(
                addr_of_mut!(ee::s00::OVERRIDE_00111AA0),
                wug_fake_spray,
                &WUG_PREV_SPRAY,
            );
            fakes::hook(
                addr_of_mut!(ee::s00::OVERRIDE_00116120),
                wug_fake_check_reset,
                &WUG_PREV_CHECK_RESET,
            );
            fakes::hook(
                addr_of_mut!(ee::s00::OVERRIDE_0011E098),
                wug_fake_world_matrix,
                &WUG_PREV_WORLD_MATRIX,
            );
        }
    }

    pub(crate) fn wug_dot3(a: [f32; 4], b: [f32; 4]) -> f32 {
        a[0] * b[0] + a[1] * b[1] + a[2] * b[2] + a[3] * b[3]
    }

    /// RiderWipeoutState_UpdateGround: spin rescaled or not; the friction
    /// curve's five ranges (points placed around this seed's tangential
    /// speed); stick poses (root pose override, layer anim 0x1d / 0x19 /
    /// other / none) at low and high speed; ground steep, mid and flat; friction above 1/dt;
    /// the speed clamp; the ray missing or hitting with a deep, shallow,
    /// small or large (launch) penetration and the velocity into or out of
    /// the ground; hit object none / ownerless / with an impulse handler;
    /// reset material or flagged triangle. Callees are gated logged fakes
    /// (PostUpdateAir's for the ground ray and the launch), one may unwind.
    #[test]
    fn rider_wipeout_state_update_ground_matches() {
        use crate::game::ai::computer::tests::{WT_CTRL, WT_MAGIC};
        use crate::game::ai::motion::wipeoutmotion::tests::{
            PUA_CTRL, PUA_HIT_B_TEMPLATE, PUA_MAGIC,
        };
        const SELF: u32 = DATA + 0x6_0000;
        const RIDER: u32 = DATA + 0x6_1000;
        const MODEL: u32 = DATA + 0x6_2000;
        const WORLD: u32 = DATA + 0x6_3000;
        const MATS: u32 = DATA + 0x6_4000;
        const TRI: u32 = DATA + 0x6_5000;
        const OBJ: u32 = DATA + 0x6_6000;
        const OWNER: u32 = OBJ + 0x100;
        const OWNER_VT: u32 = OBJ + 0x200;
        wug_install_fakes();
        covered(
            ee::s01::f_00137d18__generated,
            rider_wipeout_state_update_ground,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x0013_7d18, 8);
                c.pc = 0x0013_7d18;
                c.bus.write32(WUG_CTRL, WUG_MAGIC);
                c.bus.write32(PUA_CTRL, PUA_MAGIC);
                // ComputerActorRider's wipeout fakes (put back by their test on
                // 0x111aa0 / 0x11e098) log just like ours.
                c.bus.write32(WT_CTRL, WT_MAGIC);
                let pick = |r: &mut Rng, v: &[f32]| v[r.next_u32() as usize % v.len()];
                c.g[A0][0] = sx(SELF);
                c.bus.write32(SELF + WUG_IN_AIR, r.next_u32());
                c.bus.write32(SELF + WUG_RIDER, RIDER);
                let spin = [0.0f32, 0.0001, -3.0, 5.0];
                put_vec(
                    c,
                    SELF + WUG_SPIN,
                    [
                        pick(&mut r, &spin).to_bits(),
                        pick(&mut r, &spin).to_bits(),
                        pick(&mut r, &spin).to_bits(),
                        0,
                    ],
                );

                // Ground velocity u, sliding velocity t, vel = t + u.
                let big = [0.0f32, 0.0, 50.0, -50.0, 3000.0, -3000.0];
                let u = [pick(&mut r, &big), pick(&mut r, &big), 0.0, 0.0];
                let tc = [
                    -800.0f32, -300.0, -40.0, -10.0, 0.0, 10.0, 40.0, 300.0, 800.0,
                ];
                let vz = [-600.0f32, -50.0, 0.0, 50.0, 600.0];
                let t = [
                    pick(&mut r, &tc),
                    pick(&mut r, &tc) + 1.0,
                    pick(&mut r, &vz),
                    0.0,
                ];
                let vel = [t[0] + u[0], t[1] + u[1], t[2] + u[2], 0.0];
                let n = [
                    pick(&mut r, &[0.0, 0.0, 0.3]),
                    pick(&mut r, &[0.0, 0.3]),
                    pick(&mut r, &[1.0, 1.0, 0.2, -0.5]),
                    0.0,
                ];
                let bits = |v: [f32; 4]| v.map(f32::to_bits);
                put_vec(c, RIDER + WUG_R_VEL, bits(vel));
                put_vec(c, RIDER + WUG_R_GROUND_VEL, bits(u));
                put_vec(c, RIDER + WUG_R_GROUND_NORMAL, bits(n));
                let ts = pick(&mut r, &[1.0, 1.0, 0.5, 2.0]);
                c.bus.write32(RIDER + WUG_R_TIME_SCALE, ts.to_bits());
                put_vec(
                    c,
                    RIDER + WUG_R_FRONT,
                    [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0],
                );

                // The friction curve around this seed's x = |tangent| * 0.036.
                let v = [vel[0] - u[0], vel[1] - u[1], vel[2] - u[2], 0.0];
                let vn = wug_dot3(v, n);
                let tang = [v[0] - n[0] * vn, v[1] - n[1] * vn, v[2] - n[2] * vn, 0.0];
                let x = wug_dot3(tang, tang).sqrt() * 0.036;
                let ranges = [
                    [1.5f32, 2.0, 3.0, 4.0],
                    [0.5, 1.5, 2.0, 3.0],
                    [0.3, 0.6, 1.5, 2.0],
                    [0.2, 0.4, 0.7, 1.5],
                    [0.1, 0.2, 0.3, 0.5],
                ];
                let xs = ranges[(seed % 5) as usize];
                let lim_k = pick(&mut r, &[0.5, 1.0, 2.0]);
                for m in 0..4 {
                    let mat = MATS + m * WUG_MAT_SIZE;
                    for i in 0..4u32 {
                        let p = mat + WUG_MAT_CURVE + 8 * i;
                        c.bus.write32(p, (x * xs[i as usize]).to_bits());
                        c.bus
                            .write32(p + 4, (f32::from_bits(r.f32(1.0)).abs() * 30.0).to_bits());
                    }
                    c.bus.write32(mat + WUG_MAT_PEN, lim_k.to_bits());
                    c.bus
                        .write32(mat + WUG_MAT_RESET, (r.next_u32() % 3 == 0) as u32);
                }
                c.bus.write32(GAME_GP_WORLD, WORLD);
                c.bus.write32(WORLD + 0x84, WORLD + 0x100);
                c.bus.write32(WORLD + 0x100 + 0x44, MATS);
                c.bus.write32(RIDER + WUG_R_SURFACE, r.next_u32() % 4);

                // Stick pose: root pose override, or the layer anim's id.
                c.bus.write32(
                    RIDER + WUG_R_ROOT_POSE_OVERRIDE,
                    (r.next_u32() % 4 == 0) as u32,
                );
                c.bus.write32(RIDER + WUG_R_ANIM, r.next_u32());
                c.bus.write32(
                    WUG_CTRL + 4,
                    [0x1du32, 0x19, 3, 0][r.next_u32() as usize % 4],
                );
                c.bus.write32(RIDER + WUG_R_ANIM_MODEL, MODEL);
                c.bus.write32(MODEL + 0x140, 1.0f32.to_bits());
                c.bus.write32(RIDER + WUG_R_HANDLER, r.next_u32());
                c.bus.write32(RIDER + WUG_R_AIR, r.next_u32());

                // The ray: miss, or a hit at the origin with normal (0,0,s)
                // and the rider at penetration `pen` after the move.
                c.bus
                    .write32(PUA_CTRL + 8, pick(&mut r, &[-1.0, 0.0, 2.0, 2.0]).to_bits());
                let sign = pick(&mut r, &[1.0, -1.0]);
                let lim = lim_k;
                let pen = pick(&mut r, &[-3.0 * lim, -0.5 * lim, 5.0, 25.0]);
                let dt = ts / 60.0;
                let pz = pen * sign - vel[2] * dt;
                put_vec(
                    c,
                    RIDER + WUG_R_POS,
                    [r.f32(100.0), r.f32(100.0), pz.to_bits(), 0],
                );
                let hit = PUA_HIT_B_TEMPLATE;
                for w in 0..0x18 {
                    c.bus.write32(hit + 4 * w, r.next_u32());
                }
                put_vec(c, hit, [0, 0, 0, 0]);
                put_vec(c, hit + 0x10, bits([0.0, 0.0, sign, 0.0]));
                put_vec(c, hit + 0x20, [r.f32(50.0), r.f32(50.0), r.f32(5.0), 0]);
                c.bus.write32(hit + 0x4c, r.next_u32() % 4);
                let object = match r.next_u32() % 3 {
                    0 => 0,
                    1 => {
                        c.bus.write32(OBJ + 0xc, 0);
                        OBJ
                    }
                    _ => {
                        c.bus.write32(OBJ + 0xc, OWNER);
                        OBJ
                    }
                };
                c.bus.write32(hit + 0x50, object);
                c.bus.write32(OWNER + 0xc, OWNER_VT);
                c.bus
                    .write16(OWNER_VT + 0x158, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(OWNER_VT + 0x15c, fakes::FAKE + 0x20);
                let tri = if r.next_u32() % 2 == 0 { 0 } else { TRI };
                c.bus.write32(hit + 0x54, tri);
                c.bus.write16(TRI + 0xa, r.next_u32() as u16);
            },
        );
    }

    /// `gp - 0x848` (the world) with the test gp.
    pub(crate) const GAME_GP_WORLD: u32 = crate::difftest::GAME_GP - 0x848;

    // ------------------------------------------- shared gated fakes

    /// Control block of this module's gated fakes: the magic word (without
    /// it they run the body they replaced), the `PopFront` queue (cursor at
    /// +0x10, results at +0x20, 0-terminated) and per-target results.
    pub(crate) const OC360_CTRL: u32 = DATA + 0x7_0000;

    pub(crate) const OC360_MAGIC: u32 = 0x0360_f00d;

    pub(crate) const OC360_POP_NEXT: u32 = OC360_CTRL + 0x10;

    pub(crate) const OC360_POP_QUEUE: u32 = OC360_CTRL + 0x20;

    /// What `AnimSeqList_PushAnim` returns, and the base of the sequences
    /// `AnimSeqList_GetAt` returns (0x100 apart).
    pub(crate) const OC360_NEW_SEQ: u32 = OC360_CTRL + 0x8;

    pub(crate) const OC360_SEQS: u32 = OC360_CTRL + 0xc;

    /// The contact distance and hit `Nash_FindClosestContact` returns.
    pub(crate) const OC360_DIST: u32 = OC360_CTRL + 0x40;

    pub(crate) const OC360_HIT: u32 = OC360_CTRL + 0x100;

    /// Scripted result queues: a cursor word, then 8 results (the last
    /// one repeats).
    pub(crate) const OC360_SELECT_QUEUE: u32 = OC360_CTRL + 0x200;

    pub(crate) const OC360_BUILD_QUEUE: u32 = OC360_CTRL + 0x240;

    pub(crate) fn oc360_next(c: &mut Cpu, queue: u32) -> u32 {
        let n = c.bus.read32(queue);
        c.bus.write32(queue, n + 1);
        c.bus.read32(queue + 4 + 4 * n.min(7))
    }

    /// What `AnimController_IsLayerAnimFinished` and
    /// `AnimController_GetLayerAnimEntry` return.
    pub(crate) const OC360_LAYER_FINISHED: u32 = OC360_CTRL + 0x4c;

    pub(crate) const OC360_LAYER_ENTRY: u32 = OC360_CTRL + 0x50;

    /// What `AI_CountRidersOnPath` returns.
    pub(crate) const OC360_COUNT: u32 = OC360_CTRL + 0x48;

    /// What `UIStateMgr_GetActiveState` returns.
    pub(crate) const OC360_ACTIVE_STATE: u32 = OC360_CTRL + 0x44;

    /// A logged fake (`fakes::fake`, which may unwind) with scripted
    /// answers.
    pub(crate) fn oc360_fake(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        let (a1, a2, a3) = (c.g[A1][0] as u32, c.g[A2][0] as u32, c.g[A3][0] as u32);
        fakes::fake(c, target, ra);
        if c.pc != ra {
            return;
        }
        match target {
            0x0030_b208 => {
                let n = c.bus.read32(OC360_POP_NEXT);
                c.bus.write32(OC360_POP_NEXT, n + 1);
                c.g[V0][0] = c.bus.read32(OC360_POP_QUEUE + 4 * n.min(7)) as u64;
            }
            // AnimSeqList_PushAnim / PushLoopingAnim: the new sequence.
            0x0031_4518 | 0x0031_4588 => c.g[V0][0] = c.bus.read32(OC360_NEW_SEQ) as u64,
            // AnimSeqList_GetAt(list, i): sequence i.
            0x0031_4760 => {
                c.g[V0][0] = (c.bus.read32(OC360_SEQS) + 0x100 * (a1 & 7)) as u64;
            }
            // SPCH_SelectRequestForChannel / SPCH_BuildSentenceWithFallback:
            // scripted queues (sign-extended).
            0x003d_7a50 => c.g[V0][0] = sx(oc360_next(c, OC360_SELECT_QUEUE)),
            0x003d_a418 => c.g[V0][0] = sx(oc360_next(c, OC360_BUILD_QUEUE)),
            // Path_GetClosestPoint(path, &pos, &out, &dist = a3, 1).
            0x0026_a428 => {
                let d = c.bus.read32(OC360_DIST);
                c.bus.write32(a3, d);
            }
            // AI_CountRidersOnPath: the scripted count.
            0x0010_fc30 => c.g[V0][0] = c.bus.read32(OC360_COUNT) as u64,
            // AnimController_IsLayerAnimFinished / GetLayerAnimEntry.
            0x0031_2ae8 => c.g[V0][0] = c.bus.read32(OC360_LAYER_FINISHED) as u64,
            0x0031_1ae8 => c.g[V0][0] = c.bus.read32(OC360_LAYER_ENTRY) as u64,
            // SSXAudioSystem_IsSpeechVolumeOn: off (the grunt returns at once).
            0x0029_f0a0 => c.g[V0][0] = 0,
            // UIStateMgr_GetActiveState: the scripted state (or 0).
            0x0039_fa88 => c.g[V0][0] = c.bus.read32(OC360_ACTIVE_STATE) as u64,
            // Nash_FindClosestContact(.., hit = a2, ..): the hit and the
            // distance.
            0x0033_6850 => {
                for i in 0..0x18 {
                    let w = c.bus.read32(OC360_HIT + 4 * i);
                    c.bus.write32(a2 + 4 * i, w);
                }
                c.f[0] = c.bus.read32(OC360_DIST);
            }
            _ => {}
        }
    }

    macro_rules! oc360_gated {
        ($name:ident, $prev:ident, $addr:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC360_CTRL) != OC360_MAGIC {
                    return fakes::fall_back(c, &$prev, $gen);
                }
                oc360_fake(c, $addr);
            }
        };
    }
    pub(crate) use oc360_gated;

    // ------------------------------------------- RiderWipeoutState_PostUpdateRootPose

    oc360_gated!(
        oc360_fake_build_spheres,
        OC360_PREV_BUILD_SPHERES,
        0x0013_7550,
        ee::s01::f_00137550__generated
    );

    oc360_gated!(
        oc360_fake_nash_closest,
        OC360_PREV_NASH_CLOSEST,
        0x0033_6850,
        ee::s11::f_00336850__generated
    );

    oc360_gated!(
        oc360_fake_tree_translate,
        OC360_PREV_TREE_TRANSLATE,
        0x0032_9b40,
        ee::s10::f_00329b40__generated
    );

    /// The hull and its destructor are the wipeout tests' logged fakes
    /// (their gate set too); the sphere builder, the sweep and the tree
    /// translation are this module's.
    pub(crate) fn oc360_install_wipeout_fakes() {
        use std::ptr::addr_of_mut;
        crate::game::ai::motion::wipeoutmotion::tests::pua_install_fakes();
        {
            fakes::hook(
                addr_of_mut!(ee::s01::OVERRIDE_00137550),
                oc360_fake_build_spheres,
                &OC360_PREV_BUILD_SPHERES,
            );
            fakes::hook(
                addr_of_mut!(ee::s11::OVERRIDE_00336850),
                oc360_fake_nash_closest,
                &OC360_PREV_NASH_CLOSEST,
            );
            fakes::hook(
                addr_of_mut!(ee::s10::OVERRIDE_00329B40),
                oc360_fake_tree_translate,
                &OC360_PREV_TREE_TRANSLATE,
            );
        }
    }

    /// The wipeout state fixture shared by the root-pose and ground tests:
    /// body, tree, velocity and spin, the world (collision at
    /// `+0x84 -> +0x20`, push scale at +0x10), the hit (normal, object
    /// none / ownerless / with an owner whose slot 0x158 is a fake) and
    /// the distance.
    pub(crate) fn purp_fixture(c: &mut Cpu, r: &mut Rng, this: u32) {
        use crate::game::ai::motion::wipeoutmotion::tests::{PUA_CTRL, PUA_MAGIC};
        let body = this + 0x200;
        let world = this + 0x400;
        let obj = this + 0x500;
        let owner = this + 0x600;
        let owner_vt = this + 0x700;
        let pick = |r: &mut Rng, v: &[f32]| v[r.next_u32() as usize % v.len()].to_bits();
        let comps = [-700.0f32, -12.0, -1.0, -0.5, 0.0, 0.5, 0.9, 1.0, 3.0, 700.0];
        let vec = |c: &mut Cpu, r: &mut Rng, at: u32| {
            for i in 0..3 {
                c.bus.write32(at + 4 * i, pick(r, &comps));
            }
            c.bus.write32(at + 12, pick(r, &[0.0, 1.0]));
        };
        c.bus.write32(OC360_CTRL, OC360_MAGIC);
        c.bus.write32(PUA_CTRL, PUA_MAGIC);
        fill(c, r, this, 0x20);
        c.bus.write32(this + PURP_BODY, body);
        c.bus.write32(this + PURP_TREE, r.next_u32());
        vec(c, r, this + PURP_VEL);
        // Spin: small (re-randomised) or large.
        let spin = [0.0f32, 1.0, 2.0, 20.0, -30.0];
        for i in 0..3 {
            c.bus.write32(this + PURP_ANG_VEL + 4 * i, pick(r, &spin));
        }
        c.bus.write32(this + PURP_ANG_VEL + 12, 0);
        vec(c, r, this + PURP_SEG);
        vec(c, r, this + PURP_SEG + 0x10);
        vec(c, r, body + PURP_BODY_POS);
        c.bus
            .write32(body + PURP_BODY_ACTIVE, (r.next_u32() % 6 != 0) as u32);
        c.bus.write32(GAME_GP - 0x848, world);
        c.bus.write32(world + 0x84, world + 0x80);
        c.bus.write32(world + 0xa0, r.next_u32());
        c.bus.write32(world + 0x10, r.next_u32() % 5);
        c.bus
            .write32(OC360_DIST, pick(r, &[-1.0, 0.0, 0.5, 2.0, 2.0]));
        fill(c, r, OC360_HIT, 0x18);
        vec(c, r, OC360_HIT + PURP_HIT_NORMAL);
        let object = match r.next_u32() % 3 {
            0 => 0,
            1 => {
                c.bus.write32(obj + 0xc, 0);
                obj
            }
            _ => {
                c.bus.write32(obj + 0xc, owner);
                obj
            }
        };
        c.bus.write32(OC360_HIT + PURP_HIT_OBJECT, object);
        c.bus.write32(owner + 0xc, owner_vt);
        c.bus
            .write16(owner_vt + 0x158, (r.next_u32() as i16 as i32 & !3) as u16);
        c.bus.write32(owner_vt + 0x15c, FAKE);
    }

    /// Inactive body, no contact, a contact that brings the body to rest
    /// (hit object none / ownerless / owned), and bounces (hard or off a
    /// wall) with the spin kept or re-randomised (the real
    /// `Rand_FloatRange` and `Math_SinCos`).
    #[test]
    fn rider_wipeout_state_post_update_root_pose_matches() {
        oc360_install_wipeout_fakes();
        covered(
            ee::s01::f_00137138__generated,
            rider_wipeout_state_post_update_root_pose,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x0013_7138, 6);
                c.pc = 0x0013_7138;
                let this = DATA + 0x9_0000;
                purp_fixture(c, &mut r, this);
                c.g[A0][0] = sx(this);
            },
        );
    }

    // ------------------------------------------- RiderWipeoutState_PostUpdateGround

    /// No contact, a contact opposite the ground, one leaning into the
    /// ground (flattened) or not, a hit object none / ownerless / owned,
    /// moving into the contact (bounce, clamped low / high / between) or
    /// away; the pending flag; fast or slow (stopped). The callees are the
    /// wipeout tests' logged fakes (one may unwind).
    #[test]
    fn rider_wipeout_state_post_update_ground_matches() {
        use crate::game::ai::motion::wipeoutmotion::tests::{
            pua_install_fakes, PUA_CTRL, PUA_HIT_A_TEMPLATE, PUA_MAGIC,
        };
        pua_install_fakes();
        covered(
            ee::s01::f_00138640__generated,
            rider_wipeout_state_post_update_ground,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x0013_8640, 8);
                c.pc = 0x0013_8640;
                let this = DATA + 0xb_0000;
                let rider = this + 0x100;
                let obj = this + 0x1000;
                let owner = this + 0x1100;
                let owner_vt = this + 0x1200;
                let pick = |r: &mut Rng, v: &[f32]| v[r.next_u32() as usize % v.len()].to_bits();
                let comps = [
                    -700.0f32, -60.0, -1.0, -0.5, 0.0, 0.5, 1.0, 3.0, 60.0, 700.0,
                ];
                let vec = |c: &mut Cpu, r: &mut Rng, at: u32| {
                    for i in 0..3 {
                        c.bus.write32(at + 4 * i, pick(r, &comps));
                    }
                    c.bus.write32(at + 12, pick(r, &[0.0, 1.0]));
                };
                let unit = |c: &mut Cpu, r: &mut Rng, at: u32| {
                    let dirs: [[f32; 3]; 6] = [
                        [0.0, 0.0, 1.0],
                        [0.0, 0.0, -1.0],
                        [0.6, 0.0, 0.8],
                        [0.6, 0.0, -0.8],
                        [1.0, 0.0, 0.0],
                        [0.0, 0.8, 0.6],
                    ];
                    let d = dirs[r.next_u32() as usize % 6];
                    for i in 0..3 {
                        c.bus.write32(at + 4 * i as u32, d[i].to_bits());
                    }
                    c.bus.write32(at + 12, 0);
                };
                c.bus.write32(PUA_CTRL, PUA_MAGIC);
                c.g[A0][0] = sx(this);
                c.bus.write32(this, r.next_u32());
                c.bus.write32(this + 4, r.next_u32() % 2);
                c.bus.write32(this + PUG_RIDER, rider);
                for off in [PUG_R_OVERLAP, PUG_R_WALL_HINT, PUG_R_SPHERE_TREE] {
                    c.bus.write32(rider + off, r.next_u32());
                }
                vec(c, &mut r, rider + PUG_R_VEL);
                unit(c, &mut r, rider + PUG_R_GROUND_NORMAL);
                c.bus
                    .write32(PUA_CTRL + 4, pick(&mut r, &[-1.0, 0.0, 0.25, 2.0, 2.0]));
                fill(c, &mut r, PUA_HIT_A_TEMPLATE, 0x18);
                unit(c, &mut r, PUA_HIT_A_TEMPLATE + PUG_HIT_NORMAL);
                vec(c, &mut r, PUA_HIT_A_TEMPLATE + PUG_HIT_VEL);
                let object = match r.next_u32() % 3 {
                    0 => 0,
                    1 => {
                        c.bus.write32(obj + 0xc, 0);
                        obj
                    }
                    _ => {
                        c.bus.write32(obj + 0xc, owner);
                        obj
                    }
                };
                c.bus.write32(PUA_HIT_A_TEMPLATE + PUG_HIT_OBJECT, object);
                c.bus.write32(owner + 0xc, owner_vt);
                c.bus
                    .write16(owner_vt + 0x150, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(owner_vt + 0x154, FAKE);
            },
        );
    }
}
