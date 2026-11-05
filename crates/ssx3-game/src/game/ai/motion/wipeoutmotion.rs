//! `src/ai/motion/wipeoutmotion.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::ai::virtual_call_slot;
use crate::game::misc::STATUS_EIE;
pub(crate) use crate::game::wscript::wscriptman::frame_at;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw;
pub(crate) use crate::game::wscript::wscriptman::p;
pub(crate) use crate::game::wscript::wscriptman::sw;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::elf_image;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::SP as STACK;
    pub(crate) use crate::game::ai::ai::tests::leaf;
    pub(crate) use crate::game::ai::ai::tests::leaf_setup;
    pub(crate) use crate::game::ai::ai::tests::rng;
    pub(crate) use crate::game::lib::sif::tests::SEEDS;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::globals_3 as globals;
    pub(crate) use crate::game::wscript::wscriptman::tests::w;
    pub(crate) use crate::game::wscript::wscriptman::tests::BE_3 as BE;
    pub(crate) use crate::game::wscript::wscriptman::tests::GAME;
    pub(crate) use crate::game::wscript::wscriptman::tests::WORLD_3 as WORLD;

    #[test]
    fn rider_wipeout_state_exit_matches() {
        leaf(
            ssx3_ee::s01::f_00136f28__generated,
            rider_wipeout_state_exit,
            15,
        );
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
    use super::*;
    use crate::difftest::{check_lohi, fill};

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
}
