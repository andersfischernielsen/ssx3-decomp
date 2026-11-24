//! Staging module oc450 (wave wave224): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc450`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::game::ai::control::handplantcontrol::{
    dot_ab, dot_self, qcopy, v_length, v_normalize, v_sub,
};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0013_8ba0,
    rider_handplant_state_setup,
    "RiderHandplantState_Setup",
)];

/// `RiderHandplantState` fields written by the setup (offsets from `this`).
pub(crate) const SETUP_INIT_POS: u32 = 0x00;
pub(crate) const SETUP_EXIT_VEL: u32 = 0x10;
pub(crate) const SETUP_SPLINE_DIR: u32 = 0x20;
pub(crate) const SETUP_SPLINE_POS: u32 = 0x30;
pub(crate) const SETUP_APPROACH_DIR: u32 = 0x40;
pub(crate) const SETUP_SNAP_AXIS: u32 = 0x70;
pub(crate) const SETUP_SNAP_RATE: u32 = 0x80;
pub(crate) const SETUP_ENTER_SEC: u32 = 0x84;
pub(crate) const SETUP_ELAPSED_SEC: u32 = 0x88;
pub(crate) const SETUP_INIT_TANGENT_SPEED: u32 = 0x8c;
pub(crate) const SETUP_IS_RAIL_SPLINE: u32 = 0x90;
pub(crate) const SETUP_IS_TOESIDE: u32 = 0x94;
pub(crate) const SETUP_RIDER: u32 = 0xa0;

/// The rider fields the setup reads (`RiderHandplantState::mRider` points here).
pub(crate) const SETUP_RIDER_INIT_POS: u32 = 0x110;
/// The orientation quaternion (x, y, z, w).
pub(crate) const SETUP_RIDER_ORIENTATION: u32 = 0x120;
/// The row that `up_8BA0(rider)` reads.
pub(crate) const SETUP_RIDER_UP: u32 = 0x1c0;
pub(crate) const SETUP_RIDER_VELOCITY: u32 = 0x1e0;
pub(crate) const SETUP_RIDER_TOESIDE_FLIP: u32 = 0x320;
pub(crate) const SETUP_RIDER_POSE: u32 = 0x780;
pub(crate) const SETUP_RIDER_BONE_INDEX: u32 = 0x8a8;
/// The pose's bone list and the spline hit's fields.
pub(crate) const SETUP_MODEL_BONES: u32 = 0x2c;
pub(crate) const SETUP_HIT_POS: u32 = 0x0;
pub(crate) const SETUP_HIT_NORMAL: u32 = 0x10;
pub(crate) const SETUP_HIT_OBJ: u32 = 0x58;
pub(crate) const SETUP_OBJ_FLAGS: u32 = 0x1c;

/// Game constants at fixed addresses: the exit velocity, the cross operand
/// (`D_004FF160`) and the rider's X axis (`t4vect_XAxis`).
pub(crate) const SETUP_EXIT_VEL_CONST: u32 = 0x004f_f120;
pub(crate) const SETUP_CROSS_AXIS_CONST: u32 = 0x004f_f160;
pub(crate) const SETUP_X_AXIS_CONST: u32 = 0x004f_f140;

/// gp-relative literals: 0.001 (the axis length threshold), pi/2 and 1e-5 for
/// the snap angle. The negative-w branch reads its own copies of the last two.
pub(crate) const SETUP_GP_LEN_EPS: i32 = -0x7218;
pub(crate) const SETUP_GP_HALF_PI: i32 = -0x7214;
pub(crate) const SETUP_GP_ANGLE_EPS: i32 = -0x7210;
pub(crate) const SETUP_GP_HALF_PI_NEG: i32 = -0x720c;
pub(crate) const SETUP_GP_ANGLE_EPS_NEG: i32 = -0x7208;

/// A gp-relative literal's address.
#[inline(always)]
fn setup_gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// The cross product of the VU sequence: `vf6 = vf4 x vf5` (lanes x, y, z).
#[inline(always)]
fn setup_cross_vf6(c: &mut Cpu) {
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
}

/// The quaternion product of the VU sequence: `out = a * b`, with `a` loaded
/// into vf4 and `b` into vf5 (x, y, z, w), stored to `out`.
#[inline(always)]
fn setup_quat_mul(c: &mut Cpu, a: u32, b: u32, out: u32) {
    c.lqc2(4, a);
    c.lqc2(5, b);
    c.vu0().mul(0b1111, 7, 4, Src::Vec(5));
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().mula(0b1110, 4, Src::Bc(5, W));
    c.vu0().madda(0b1110, 5, Src::Bc(4, W));
    c.vu0().suba(0b0001, 7, Src::Bc(7, X));
    c.vu0().msuba(0b0001, 0, Src::Bc(7, Y));
    c.vu0().msub(0b0001, 8, 0, Src::Bc(7, Z));
    c.vu0().madd(0b1110, 8, 6, Src::Bc(0, W));
    c.sqc2(8, out);
}

/// The axis-angle quaternion: `out = (sin * axis, cos)`, with the sine and
/// cosine that `Math_SinCos` wrote at `sin_at` and `cos_at`, and the axis
/// (x, y, z) at `axis`.
fn setup_axis_angle(c: &mut Cpu, axis: u32, sin_at: u32, cos_at: u32, out: u32) {
    c.lwc1(0, sin_at);
    c.lwc1(1, axis);
    c.lwc1(2, axis + 4);
    c.lwc1(4, axis + 8);
    c.mul_s(1, 0, 1);
    c.mul_s(2, 0, 2);
    c.lwc1(3, cos_at);
    c.mul_s(0, 0, 4);
    c.swc1(3, out + 0xc);
    c.swc1(1, out);
    c.swc1(2, out + 4);
    c.swc1(0, out + 8);
}

/// `void RiderHandplantState_Setup(RiderHandplantState* this, sHit* hit)`
/// (`cHandplantMotion_init`, 0x138ba0). `f12` is the duration. Copies the
/// rider's pose and the spline end points into the state, the approach
/// direction, the tangent speed and rail flag, the toeside side from the
/// forward vector and the spline normal, and the snap rotation (axis, angle
/// and rate over the duration) from the rider's orientation to the ideal one.
pub fn rider_handplant_state_setup(c: &mut Cpu) {
    guest(c, setup_body)
}

fn setup_body(c: &mut Cpu) -> Flow {
    // Prologue and the spline's copies (0x138c98).
    c.cycles += 59;
    let frame = Frame::open(c, 0xe0);
    let s = sp(c);
    c.g[A3][0] = 0;
    frame.save(c, S0, 0xb0);
    frame.save_ra(c, 0xa0);
    c.g[S0][0] = c.g[A0][0];
    frame.save_fpr(c, 22, 0xd0);
    c.g[A0][0] = sx(SETUP_EXIT_VEL_CONST);
    frame.save_fpr(c, 21, 0xc8);
    frame.save_fpr(c, 20, 0xc0);

    c.g[A2][0] = Addr(r32(c, S0).wrapping_add(SETUP_RIDER)).lw(c);
    let this = r32(c, S0);
    let rider = r32(c, A2);
    qcopy(
        c,
        V0,
        rider.wrapping_add(SETUP_RIDER_INIT_POS),
        this + SETUP_INIT_POS,
    );
    let exit_vel = r32(c, A0);
    qcopy(c, V1, exit_vel, this + SETUP_EXIT_VEL);
    let hit = r32(c, A1);
    qcopy(c, V0, hit + SETUP_HIT_NORMAL, this + SETUP_SPLINE_DIR);
    qcopy(c, V1, hit + SETUP_HIT_POS, this + SETUP_SPLINE_POS);

    // ApproachDir = Normalize(SplinePos - bone), bone = pose bones[index].
    c.g[A0][0] = Addr(rider.wrapping_add(SETUP_RIDER_POSE)).lw(c);
    c.g[V0][0] = Addr(rider.wrapping_add(SETUP_RIDER_BONE_INDEX)).lw(c);
    c.g[V1][0] = Addr(r32(c, A0).wrapping_add(SETUP_MODEL_BONES)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let bone = r32(c, V1);
    v_sub(c, this + SETUP_SPLINE_POS, bone, s + 0x20);
    qcopy(c, V0, s + 0x20, s + 0x10);
    v_normalize(c, s + 0x10, s + 0x20);
    c.lq(V0, s + 0x20);
    let this = r32(c, S0);
    c.swc1(12, this + SETUP_ENTER_SEC);
    c.sq(V0, this + SETUP_APPROACH_DIR);
    c.sw(this + SETUP_ELAPSED_SEC, 0);

    // InitTangentSpeed = Dot(rider velocity, SplineDir).
    c.g[A0][0] = Addr(r32(c, S0).wrapping_add(SETUP_RIDER)).lw(c);
    c.sq(V0, s);
    let rider_a0 = r32(c, A0);
    let this = r32(c, S0);
    dot_ab(
        c,
        rider_a0.wrapping_add(SETUP_RIDER_VELOCITY),
        this + SETUP_SPLINE_DIR,
    );
    c.qmfc2(V1, 4);
    c.mtc1(V1, 0);
    c.swc1(0, this + SETUP_INIT_TANGENT_SPEED);

    // IsRailSpline = hit->obj ? hit->obj->flags & 1 : 0.
    c.g[V0][0] = Addr(r32(c, A1).wrapping_add(SETUP_HIT_OBJ)).lw(c);
    if c.g[V0][0] == 0 {
        let this = r32(c, S0);
        c.sw(this + SETUP_IS_RAIL_SPLINE, c.g[A3][0]);
    } else {
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, V0).wrapping_add(SETUP_OBJ_FLAGS)).lw(c);
        c.g[A3][0] = c.g[V0][0] & 0x1;
        let this = r32(c, S0);
        c.sw(this + SETUP_IS_RAIL_SPLINE, c.g[A3][0]);
    }

    // Ideal orientation, and the first rotation about Cross(up, D_004FF160)
    // when the axis is longer than 0.001 (0x138dcc).
    c.cycles += 30;
    let len_eps = setup_gp(c, SETUP_GP_LEN_EPS);
    c.lwc1(22, len_eps);
    let rider0 = r32(c, A0);
    qcopy(c, V0, rider0.wrapping_add(SETUP_RIDER_ORIENTATION), s);
    let up_row = r32(c, A0).wrapping_add(SETUP_RIDER_UP);
    c.lq(V1, up_row);
    c.g[V0][0] = sx(SETUP_CROSS_AXIS_CONST);
    c.sq(V1, s + 0x40);
    c.lqc2(4, s + 0x40);
    let cross_axis = r32(c, V0);
    c.lqc2(5, cross_axis);
    setup_cross_vf6(c);
    c.sqc2(6, s + 0x50);
    qcopy(c, V0, s + 0x50, s + 0x20);
    c.sq(V0, s + 0x30);
    v_length(c, s + 0x20, V1, 0);
    c.c_lt_s(22, 0);
    let t = !c.fcc();
    c.lwc1(3, s + 0x4);
    if !t {
        c.cycles += 47;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 12);
        c.mfc1(2, 0);
        c.qmtc2(2, 3);
        c.vu0().div(0, 3, 3, 0);
        c.lqc2(4, s + 0x20);
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 4, Src::Q);
        c.sqc2(5, s + 0x20);
        c.min_s(12, 0, 12);
        call_ptr(c, 0x0031_c128, 0x0013_8d3c)?;
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 12);
        c.g[A0][0] = sx(s + 0x90);
        c.g[A1][0] = sx(s + 0x94);
        c.mul_s(12, 0, 12);
        call_ptr(c, 0x0031_be50, 0x0013_8d54)?;
        setup_axis_angle(c, s + 0x20, s + 0x90, s + 0x94, s + 0x30);
        qcopy(c, V0, s + 0x30, s + 0x10);
        setup_quat_mul(c, s + 0x10, s, s + 0x40);
        qcopy(c, V0, s + 0x40, s);
        c.sq(V0, s + 0x30);
        c.lwc1(3, s + 0x4);
    }

    // fwd from the ideal orientation, the flat spline normal n, and
    // IsToeside = Dot(n, fwd) < 0 (0x138e9c).
    c.cycles += 52;
    let this = r32(c, S0);
    c.g[A0][0] = 1;
    c.lwc1(4, s + 0x8);
    c.lwc1(0, s);
    c.mul_s(1, 3, 3);
    c.lwc1(5, s + 0xc);
    c.mul_s(6, 4, 4);
    c.mul_s(2, 0, 4);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 20);
    c.mul_s(7, 5, 3);
    c.lq(V0, this + SETUP_SPLINE_DIR);
    c.add_s(1, 1, 6);
    c.sw(s + 0x3c, 0);
    c.mul_s(0, 0, 3);
    c.sq(V0, s + 0x40);
    c.mul_s(5, 5, 4);
    c.sw(s + 0x48, 0);
    c.sub_s(2, 2, 7);
    c.add_s(1, 1, 1);
    c.add_s(0, 0, 5);
    c.add_s(2, 2, 2);
    c.sub_s(1, 20, 1);
    c.add_s(0, 0, 0);
    c.swc1(2, s + 0x38);
    c.swc1(1, s + 0x30);
    c.swc1(0, s + 0x34);
    v_normalize(c, s + 0x40, s + 0x60);
    c.mtc1(0, 21);
    qcopy(c, V0, s + 0x60, s + 0x50);
    dot_ab(c, s + 0x50, s + 0x30);
    c.qmfc2(V1, 4);
    c.mtc1(V1, 0);
    c.c_lt_s(0, 21);
    if !c.fcc() {
        c.g[A0][0] = 0;
    }

    // IsToeside, and the flip of n when toeside (0x138ec0).
    c.cycles += 2;
    let not_toeside = c.g[A0][0] == 0;
    let this = r32(c, S0);
    c.sw(this + SETUP_IS_TOESIDE, c.g[A0][0]);
    if !not_toeside {
        c.cycles += 7;
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 0);
        c.mfc1(2, 0);
        c.lqc2(4, s + 0x50);
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, s + 0x50);
    }

    // The rider's flag flips the toeside side (0x138edc).
    c.cycles += 4;
    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(SETUP_RIDER)).lw(c);
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(SETUP_RIDER_TOESIDE_FLIP)).lw(c);
    if c.g[V1][0] != 0 {
        c.cycles += 3;
        let this = r32(c, S0);
        c.g[V0][0] = Addr(this.wrapping_add(SETUP_IS_TOESIDE)).lw(c);
        c.g[V0][0] = c.g[V0][0] ^ 0x1;
        let this = r32(c, S0);
        c.sw(this + SETUP_IS_TOESIDE, c.g[V0][0]);
    }

    // axis = Cross(fwd, n); rotate fwd onto n when the axis is longer than
    // 0.001 (0x138fec).
    c.cycles += 23;
    c.lqc2(4, s + 0x30);
    c.lqc2(5, s + 0x50);
    setup_cross_vf6(c);
    c.sqc2(6, s + 0x70);
    qcopy(c, V0, s + 0x70, s + 0x20);
    c.sq(V0, s + 0x60);
    v_length(c, s + 0x20, V1, 0);
    c.c_lt_s(22, 0);
    if !c.fcc() {
        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(SETUP_RIDER)).lw(c);
    } else {
        c.cycles += 45;
        c.mfc1(2, 0);
        c.qmtc2(2, 3);
        c.vu0().div(0, 3, 3, 0);
        c.lqc2(4, s + 0x20);
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 4, Src::Q);
        c.sqc2(5, s + 0x20);
        c.min_s(12, 0, 20);
        call_ptr(c, 0x0031_c128, 0x0013_8f5c)?;
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 12);
        c.g[A0][0] = sx(s + 0x98);
        c.g[A1][0] = sx(s + 0x9c);
        c.mul_s(12, 0, 12);
        call_ptr(c, 0x0031_be50, 0x0013_8f74)?;
        setup_axis_angle(c, s + 0x20, s + 0x98, s + 0x9c, s + 0x60);
        qcopy(c, V0, s + 0x60, s + 0x10);
        setup_quat_mul(c, s + 0x10, s, s + 0x70);
        qcopy(c, V0, s + 0x70, s);
        c.sq(V0, s + 0x60);
        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(SETUP_RIDER)).lw(c);
    }

    // Snap rotation: deltaRot = ideal * Conj(rider orientation), its angle
    // from w (0x139148 and 0x1390ac).
    c.cycles += 34;
    let rider = r32(c, V0);
    c.lwc1(0, rider.wrapping_add(SETUP_RIDER_ORIENTATION + 0xc));
    c.swc1(0, s + 0x8c);
    c.lwc1(2, rider.wrapping_add(SETUP_RIDER_ORIENTATION));
    c.swc1(2, s + 0x80);
    c.neg_s(2, 2);
    c.lwc1(1, rider.wrapping_add(SETUP_RIDER_ORIENTATION + 0x4));
    c.swc1(1, s + 0x84);
    c.neg_s(1, 1);
    c.lwc1(0, rider.wrapping_add(SETUP_RIDER_ORIENTATION + 0x8));
    c.swc1(2, s + 0x80);
    c.neg_s(0, 0);
    c.swc1(1, s + 0x84);
    c.swc1(0, s + 0x88);
    qcopy(c, V0, s + 0x80, s + 0x70);
    setup_quat_mul(c, s, s + 0x70, s + 0x80);
    qcopy(c, V0, s + 0x80, s + 0x10);
    c.sq(V0, s + 0x60);
    c.lwc1(0, s + 0x1c);
    c.c_lt_s(21, 0);
    let t = !c.fcc();

    // true: the axis comes from Sin (MID); false: the X axis (XAXIS).
    let to_mid = if !t {
        // w > 0: ang = (pi/2 - Asin(min(w, 1))) * 2.
        c.cycles += 10;
        c.min_s(12, 0, 20);
        call_ptr(c, 0x0031_c128, 0x0013_907c)?;
        let half_pi = setup_gp(c, SETUP_GP_HALF_PI);
        let eps = setup_gp(c, SETUP_GP_ANGLE_EPS);
        c.lwc1(1, half_pi);
        c.lwc1(2, eps);
        c.sub_s(1, 1, 0);
        c.add_s(1, 1, 1);
        c.c_le_s(2, 1);
        let t2 = !c.fcc();
        let this = r32(c, S0);
        c.swc1(1, this + SETUP_SNAP_RATE);
        if t2 {
            false
        } else {
            c.cycles += 4;
            c.g[AT][0] = 0x3f00_0000;
            c.mtc1(1, 12);
            c.lwc1(2, s + 0x10);
            true
        }
    } else {
        // w <= 0: ang = (pi/2 - Asin(-max(w, -1))) * 2.
        c.cycles += 13;
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 12);
        c.max_s(12, 0, 12);
        c.neg_s(12, 12);
        call_ptr(c, 0x0031_c128, 0x0013_90c0)?;
        let half_pi = setup_gp(c, SETUP_GP_HALF_PI_NEG);
        let eps = setup_gp(c, SETUP_GP_ANGLE_EPS_NEG);
        c.lwc1(1, half_pi);
        c.lwc1(2, eps);
        c.sub_s(1, 1, 0);
        c.add_s(1, 1, 1);
        c.c_le_s(2, 1);
        let t3 = !c.fcc();
        let this = r32(c, S0);
        c.swc1(1, this + SETUP_SNAP_RATE);
        if t3 {
            false
        } else {
            c.cycles += 3;
            c.g[AT][0] = 0xffff_ffff_bf00_0000;
            c.mtc1(1, 12);
            c.lwc1(2, s + 0x10);
            true
        }
    };

    if to_mid {
        // SnapRotationAxis = deltaRot.xyz / Sin(ang * 0.5) (f12 = ang * +-0.5).
        c.cycles += 19;
        c.mul_s(12, 1, 12);
        c.lwc1(0, s + 0x14);
        c.lwc1(1, s + 0x18);
        c.swc1(2, s + 0x70);
        c.swc1(0, s + 0x74);
        c.swc1(1, s + 0x78);
        c.swc1(21, s + 0x7c);
        call_ptr(c, 0x0031_bf60, 0x0013_910c)?;
        c.mfc1(3, 0);
        c.qmtc2(3, 3);
        c.vu0().div(0, 3, 3, 0);
        c.lqc2(4, s + 0x70);
        c.vu0().waitq();
        c.vu0().mul(0b1111, 4, 4, Src::Q);
        c.sqc2(4, s + 0x80);
        c.lq(V0, s + 0x80);
        let this = r32(c, S0);
        c.sq(V0, this + SETUP_SNAP_AXIS);
        c.sq(V0, s + 0x60);
    } else {
        // SnapRotationAxis = t4vect_XAxis.
        c.cycles += 4;
        c.g[V0][0] = sx(SETUP_X_AXIS_CONST);
        let x_axis = r32(c, V0);
        c.lq(V1, x_axis);
        let this = r32(c, S0);
        c.sq(V1, this + SETUP_SNAP_AXIS);
    }

    // Epilogue: SnapRotationRate /= EnterSec (0x139148 tail).
    c.cycles += 11;
    let this = r32(c, S0);
    c.lwc1(0, this + SETUP_SNAP_RATE);
    c.lwc1(1, this + SETUP_ENTER_SEC);
    frame.restore_ra(c, 0xa0);
    c.div_s(0, 0, 1);
    frame.restore_fpr(c, 22, 0xd0);
    frame.restore_fpr(c, 21, 0xc8);
    frame.restore_fpr(c, 20, 0xc0);
    let this = r32(c, S0);
    c.swc1(0, this + SETUP_SNAP_RATE);
    frame.restore(c, S0, 0xb0);
    frame.close(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, elf_image, put_vec, scramble, Rng, DATA, GAME_GP};

    /// The fixture's objects, all inside the test data window.
    const SELF: u32 = DATA + 0x4000;
    const RIDER: u32 = DATA + 0x5000;
    const MODEL: u32 = DATA + 0x6000;
    const BONES: u32 = DATA + 0x7000;
    const HIT: u32 = DATA + 0x8000;
    const OBJ: u32 = DATA + 0x9000;

    fn rand_vec(r: &mut Rng) -> [u32; 4] {
        [r.f32(4.0), r.f32(4.0), r.f32(4.0), r.f32(4.0)]
    }

    fn rand_quat(r: &mut Rng) -> [u32; 4] {
        [r.f32(1.0), r.f32(1.0), r.f32(1.0), r.f32(1.0)]
    }

    /// Twice the game's cross operand (its cross product with it is zero).
    fn cross_axis_doubled(c: &mut Cpu) -> [u32; 4] {
        let mut v = [0u32; 4];
        for (i, lane) in v.iter_mut().take(3).enumerate() {
            let d = f32::from_bits(c.bus.read32(SETUP_CROSS_AXIS_CONST + 4 * i as u32));
            *lane = (d * 2.0).to_bits();
        }
        v
    }

    #[test]
    fn rider_handplant_state_setup_matches() {
        check(
            ee::s01::f_00138ba0__generated,
            rider_handplant_state_setup,
            300,
            &|c, seed| {
                let mut r = Rng(0x138b_a0 ^ seed as u64);
                scramble(c, &mut r);
                elf_image(c);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0013_8ba0;
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(HIT);
                c.f[12] = (0.25f32 + (seed % 8) as f32 * 0.5).to_bits();

                // Mode 1 keeps the first rotation off (up parallel to the
                // cross operand); mode 2 is the identity orientation with
                // the spline normal along X, so both rotations stay off and
                // the snap angle is zero.
                let mode = seed % 6;
                let orientation = if mode == 2 {
                    [0, 0, 0, 0x3f80_0000]
                } else {
                    rand_quat(&mut r)
                };
                put_vec(c, RIDER + SETUP_RIDER_ORIENTATION, orientation);
                put_vec(c, RIDER + SETUP_RIDER_INIT_POS, rand_vec(&mut r));
                put_vec(c, RIDER + SETUP_RIDER_VELOCITY, rand_vec(&mut r));
                let up = if mode == 1 || mode == 2 {
                    cross_axis_doubled(c)
                } else {
                    rand_vec(&mut r)
                };
                put_vec(c, RIDER + SETUP_RIDER_UP, up);
                c.bus
                    .write32(RIDER + SETUP_RIDER_TOESIDE_FLIP, u32::from(seed % 3 == 0));
                c.bus.write32(RIDER + SETUP_RIDER_POSE, MODEL);
                c.bus
                    .write32(RIDER + SETUP_RIDER_BONE_INDEX, r.next_u32() % 3);
                c.bus.write32(MODEL + SETUP_MODEL_BONES, BONES);
                for i in 0..0x40 {
                    c.bus.write32(BONES + 4 * i, r.f32(4.0));
                }
                c.bus.write32(SELF + SETUP_RIDER, RIDER);

                let normal = if mode == 2 {
                    let sign = if seed % 2 == 0 {
                        0x3f80_0000
                    } else {
                        0xbf80_0000
                    };
                    [sign, 0, 0, 0]
                } else {
                    rand_vec(&mut r)
                };
                put_vec(c, HIT + SETUP_HIT_POS, rand_vec(&mut r));
                put_vec(c, HIT + SETUP_HIT_NORMAL, normal);
                let obj = if seed % 4 == 0 { 0 } else { OBJ };
                c.bus.write32(HIT + SETUP_HIT_OBJ, obj);
                c.bus.write32(OBJ + SETUP_OBJ_FLAGS, r.next_u32());
            },
        );
    }
}
