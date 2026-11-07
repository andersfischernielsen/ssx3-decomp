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
        0x0013_7750,
        rider_wipeout_state_update_air,
        "RiderWipeoutState_UpdateAir",
    ),
    (
        0x0013_8ad8,
        rider_wipeout_state_save_state,
        "RiderWipeoutState_SaveState",
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
                    return $gen(c);
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

    /// Install the gated fakes in their callees' slots once per test process.
    pub(crate) fn pua_install_fakes() {
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| unsafe {
            ee::s11::OVERRIDE_0032F650 = pua_fake_hull_ctor;
            ee::s11::OVERRIDE_003342D0 = pua_fake_closest;
            ee::s11::OVERRIDE_0032F708 = pua_fake_hull_dtor;
            ee::s00::OVERRIDE_00113998 = pua_fake_clear_air;
            ee::s00::OVERRIDE_00106538 = pua_fake_translate;
            ee::s00::OVERRIDE_0011E150 = pua_fake_bounds;
            ee::s00::OVERRIDE_00105398 = pua_fake_wall;
            ee::s00::OVERRIDE_00107888 = pua_fake_riders;
            ee::s00::OVERRIDE_001135B8 = pua_fake_prime;
            ee::s09::OVERRIDE_002E23E0 = pua_fake_spray;
            ee::s01::OVERRIDE_00138960 = pua_fake_ground_ray;
        });
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
}
