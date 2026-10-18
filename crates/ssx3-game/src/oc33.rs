//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for rider update (IK, bounds, skeleton, lean, speed animations, metrics), each with a differential test against the generated
//! body.
//!
//! - **Bounds** (`Rider_UpdateBounds`): the eight corners of the rider's
//!   oriented box (centre ± 150·forward ± 100·up, +250/−50·side) grown
//!   into the world AABB at `+0x400`.
//! - **Board lean** (`Rider_UpdateBoardLean`): roll/pitch beyond twice the
//!   dead zone rotate the board transform about a bone pivot, the spin
//!   input yaws it, and the body offset moves it.
//! - **Leg IK** (`Rider_SolveLegIK`): blends the foot towards its target,
//!   bends the knee by the law of cosines and swings the hip onto the
//!   foot.
//! - **Skeleton** (`Rider_UpdateSkeleton`): root transform, board lean,
//!   forward kinematics, spine twist/tilt, both legs, world matrix.
//! - **Speed animations** (`Rider_UpdateSpeedAnims`), **slewed
//!   parameters** (`Rider_UpdateSlewedParams`) and **metrics**
//!   (`RiderMetrix_Update`).
//!
//! The VU0 code issues the original's COP2 macro operations in the same
//! order (the recurring sequences — quaternion product, quaternion
//! rotation, dot products — are helpers below), so VF/ACC/Q and the VU0
//! flags end as the original leaves them. FPU arithmetic uses [`FpuF32`]
//! and writes the FPU registers the original writes. Calls follow
//! [`crate::call`]: registers at each call as the original has them, the
//! unwind check after every call.

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0011_e150, rider_update_bounds, "Rider_UpdateBounds"),
    (0x0011_fa10, rider_update_board_lean, "Rider_UpdateBoardLean"),
    (0x0011_f3d8, rider_solve_leg_ik, "Rider_SolveLegIK"),
    (0x0011_eb98, rider_update_skeleton, "Rider_UpdateSkeleton"),
    (0x0012_0378, rider_update_speed_anims, "Rider_UpdateSpeedAnims"),
    (0x0012_11f8, rider_update_slewed_params, "Rider_UpdateSlewedParams"),
    (0x0011_7c28, rider_metrix_update, "RiderMetrix_Update"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

mod callee {
    use super::{ee, Callee};

    pub const RIDER_GET_SUB_STATE: Callee = Callee { addr: 0x0011_fee8, f: ee::s00::Rider_GetSubState };
    pub const RIDER_GET_MAIN_STATE: Callee = Callee { addr: 0x0011_fe98, f: ee::s00::Rider_GetMainState };
    pub const RIDER_UPDATE_BOARD_LEAN: Callee = Callee { addr: 0x0011_fa10, f: ee::s00::Rider_UpdateBoardLean };
    pub const RIDER_SOLVE_LEG_IK: Callee = Callee { addr: 0x0011_f3d8, f: ee::s00::Rider_SolveLegIK };
    pub const RIDER_UPDATE_SPHERE_TREE: Callee = Callee { addr: 0x0010_6828, f: ee::s00::Rider_UpdateSphereTree };
    pub const RIDER_UPDATE_SPEED_ANIMS: Callee = Callee { addr: 0x0012_0378, f: ee::s00::Rider_UpdateSpeedAnims };
    pub const AI_GET_FRAME_COUNTER: Callee = Callee { addr: 0x0012_98c8, f: ee::s00::AI_GetFrameCounter };
    pub const RIDER_METRIX_BANK_COMBO: Callee = Callee { addr: 0x0011_7718, f: ee::s00::RiderMetrix_BankCombo };
    pub const RIDER_METRIX_CHECK_SCORE_MILESTONE: Callee =
        Callee { addr: 0x0011_9210, f: ee::s00::RiderMetrix_CheckScoreMilestone };
    pub const RIDER_METRIX_UPDATE_TRICK_HUD: Callee =
        Callee { addr: 0x0011_7fe0, f: ee::s00::RiderMetrix_UpdateTrickHUD };
    pub const CHUDFX_UPDATE: Callee = Callee { addr: 0x0011_6fb8, f: ee::s00::cHUDFX_Update };

    // Cross-shard direct calls the generator emitted as `call_indirect`.
    pub const RIDER_SPIN_SUB_APPLY_POSE_ROTATION: u32 = 0x0013_4dd0;
    pub const RIDER_MODEL_APPLY_POSE: u32 = 0x0031_0200;
    pub const QUAT_SLERP: u32 = 0x0031_bcb0;
    pub const MATH_SIN_COS: u32 = 0x0031_be50;
    pub const MATH_ASIN: u32 = 0x0031_c128;
    pub const ANIM_CONTROLLER_GET_LAYER_SEQUENCE: u32 = 0x0031_1b20;
    pub const ANIM_CONTROLLER_GET_LAYER_ANIM_ID: u32 = 0x0031_2aa0;
    pub const ANIM_CONTROLLER_PLAY_ANIM: u32 = 0x0031_28e8;
    pub const MEMSET: u32 = 0x0041_6210;
    pub const SSX_AUDIO_SYSTEM_GET_INSTANCE: u32 = 0x0028_b180;
    pub const SSX_AUDIO_SYSTEM_SET_PLAYER_TRICK_SCORE: u32 = 0x0029_0f58;
}

/// Global vectors (`.bss`, set up at boot): the bone pivot subtracted
/// before rotating a bone offset, the three rotation axes, the identity
/// quaternion.
const PIVOT: u32 = 0x004f_f130;
const AXIS_X: u32 = 0x004f_f140;
const AXIS_Z: u32 = 0x004f_f150;
const AXIS_Y: u32 = 0x004f_f160;
const IDENTITY_QUAT: u32 = 0x004f_f220;

/// `RiderModel` tables: per-bone local offsets (16 bytes), local rotations
/// (16 bytes) and the posed bone transforms (`{pos, quat}`, 32 bytes);
/// `+0x140` the model scale (x, y, z, w).
mod model {
    pub const BONE_OFFSETS: u32 = 0x24;
    pub const BONE_ROTATIONS: u32 = 0x28;
    pub const BONES: u32 = 0x2c;
    pub const SCALE: u32 = 0x140;
}

/// `Rider` fields shared by several functions.
mod rider {
    pub const MODEL: u32 = 0x780;
    pub const STATE: u32 = 0x77c;
    pub const ANIM: u32 = 0x784;
}

// ---------------------------------------------------------------------------
// Register and memory helpers.

/// `a + off`.
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `lw` (sign-extended).
#[inline(always)]
fn lw(c: &mut Cpu, a: u32) -> u64 {
    c.lw(a)
}

/// FPU register `n`.
#[inline(always)]
fn fr(c: &Cpu, n: usize) -> FpuF32 {
    FpuF32(c.f[n])
}

/// `fN = v`.
#[inline(always)]
fn setf(c: &mut Cpu, n: usize, v: FpuF32) -> FpuF32 {
    c.f[n] = v.0;
    v
}

/// `lwc1 fN, a`.
#[inline(always)]
fn ldf(c: &mut Cpu, n: usize, a: u32) -> FpuF32 {
    let v = Addr(a).read_fpu(c);
    setf(c, n, v)
}

/// `swc1 fN, a`.
#[inline(always)]
fn stf(c: &mut Cpu, n: usize, a: u32) {
    let v = fr(c, n);
    Addr(a).write_fpu(c, v);
}

/// `li at, bits; mtc1 at, fN`.
#[inline(always)]
fn li_f(c: &mut Cpu, n: usize, bits: u32) -> FpuF32 {
    c.g[AT][0] = sx(bits);
    setf(c, n, FpuF32(bits))
}

/// Set the FPU condition bit; returns it.
#[inline(always)]
fn fcc(c: &mut Cpu, cond: bool) -> bool {
    abi::set_fcc(c, cond);
    cond
}

/// `lq reg, src; sq reg, dst` (a quadword copied through a GPR).
#[inline(always)]
fn qcopy(c: &mut Cpu, reg: usize, src: u32, dst: u32) {
    c.lq(reg, src);
    c.sq(reg, dst);
}

// ---------------------------------------------------------------------------
// VU0 sequences (macro mode), as the original issues them.

/// `out = a + b` (vf3, vf4 → vf5).
fn v_add(c: &mut Cpu, a: u32, b: u32, out: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, out);
}

/// `out = a - b` (vf3, vf4 → vf5).
fn v_sub(c: &mut Cpu, a: u32, b: u32, out: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, out);
}

/// [`v_add`] or [`v_sub`].
fn v_add_sub(c: &mut Cpu, add: bool, a: u32, b: u32, out: u32) {
    if add {
        v_add(c, a, b, out)
    } else {
        v_sub(c, a, b, out)
    }
}

/// `out = v * fF`, the scalar moved through GPR `via` into vf3
/// (`mfc1 via, fF; qmtc2 via, vf3` — vf3 keeps the GPR's upper half).
fn v_scale(c: &mut Cpu, via: usize, f: usize, v: u32, out: u32) {
    c.mfc1(via, f);
    c.lqc2(4, v);
    c.qmtc2(via, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, out);
}

/// The tail of the dot-product sequence (vf4 holds the products).
fn dot_tail(c: &mut Cpu) {
    let vu = c.vu0();
    vu.adda(0b1000, 4, Src::Bc(4, Y));
    vu.madda(0b1000, 6, Src::Bc(4, Z));
    vu.madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// `vf4.x = a · b` (vf3 = a, vf5 = b).
fn v_dot(c: &mut Cpu, a: u32, b: u32) {
    c.lqc2(3, a);
    c.lqc2(5, b);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    dot_tail(c);
}

/// `vf4.x = a · a` (vf3 = a).
fn v_dot_self(c: &mut Cpu, a: u32) {
    c.lqc2(3, a);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    dot_tail(c);
}

/// `|a|` through Q (`vsqrt`, `cfc2 via, Q`, `mtc1 via, fDst`).
fn v_length(c: &mut Cpu, a: u32, via: usize, dst: usize) -> FpuF32 {
    v_dot_self(c, a);
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(via, 22);
    c.mtc1(via, dst);
    fr(c, dst)
}

/// `out = a / |a|` (`vrsqrt`).
fn v_normalize(c: &mut Cpu, a: u32, out: u32) {
    v_dot_self(c, a);
    c.vu0().rsqrt(0, 3, 4, 0);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 5, 3, Src::Q);
    c.sqc2(5, out);
}

/// `out = v / fF` (`vdiv`, the scalar moved through v0).
fn v_div_by(c: &mut Cpu, f: usize, v: u32, out: u32) {
    c.mfc1(V0, f);
    c.qmtc2(V0, 3);
    c.vu0().div(0, 3, 3, 0);
    c.lqc2(4, v);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 5, 4, Src::Q);
    c.sqc2(5, out);
}

/// `out = a × b` (vf4 = a, vf5 = b → vf6).
fn v_cross(c: &mut Cpu, a: u32, b: u32, out: u32) {
    c.lqc2(4, a);
    c.lqc2(5, b);
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
    c.sqc2(6, out);
}

/// Quaternion product `out = a ⊗ b` (vf4 = a, vf5 = b → vf8).
fn q_mul(c: &mut Cpu, a: u32, b: u32, out: u32) {
    c.lqc2(4, a);
    c.lqc2(5, b);
    let vu = c.vu0();
    vu.mul(0b1111, 7, 4, Src::Vec(5));
    vu.opmula(4, 5);
    vu.opmsub(6, 5, 4);
    vu.mula(0b1110, 4, Src::Bc(5, W));
    vu.madda(0b1110, 5, Src::Bc(4, W));
    vu.suba(0b0001, 7, Src::Bc(7, X));
    vu.msuba(0b0001, 0, Src::Bc(7, Y));
    vu.msub(0b0001, 8, 0, Src::Bc(7, Z));
    vu.madd(0b1110, 8, 6, Src::Bc(0, W));
    c.sqc2(8, out);
}

/// Vector `v` rotated by quaternion `q`: `v + 2w(q×v) + 2q×(q×v)` (vf4 = q,
/// vf5 = v → vf8).
fn q_rotate(c: &mut Cpu, q: u32, v: u32, out: u32) {
    c.lqc2(4, q);
    c.lqc2(5, v);
    let vu = c.vu0();
    vu.sub(0b0001, 8, 8, Src::Vec(8));
    vu.opmula(4, 5);
    vu.opmsub(6, 5, 4);
    vu.opmula(4, 6);
    vu.opmsub(7, 6, 4);
    vu.mula(0b1110, 5, Src::Bc(0, W));
    vu.madda(0b1110, 6, Src::Bc(4, W));
    vu.madda(0b1110, 6, Src::Bc(4, W));
    vu.madda(0b1110, 7, Src::Bc(0, W));
    vu.madd(0b1110, 8, 7, Src::Bc(0, W));
    c.sqc2(8, out);
}

/// The quaternion `{axis * sin, cos}` from `Math_SinCos`'s outputs at
/// `sincos` (sin) and `sincos + 4` (cos), written to `out`. The original
/// reads the axis from a global (z into f1, x into `x_reg`, y into f4) and
/// the cosine into `cos_reg`; f0 ends as y·sin.
fn axis_quat(c: &mut Cpu, axis: u32, sincos: u32, out: u32, x_reg: usize, cos_reg: usize) {
    let az = ldf(c, 1, p(axis, 8));
    let s = ldf(c, 0, sincos);
    let ax = ldf(c, x_reg, axis);
    let ay = ldf(c, 4, p(axis, 4));
    let qz = setf(c, 1, s * az);
    let qx = setf(c, x_reg, s * ax);
    let w = ldf(c, cos_reg, p(sincos, 4));
    let qy = setf(c, 0, s * ay);
    Addr(p(out, 0xc)).write_fpu(c, w);
    Addr(p(out, 8)).write_fpu(c, qz);
    Addr(out).write_fpu(c, qx);
    Addr(p(out, 4)).write_fpu(c, qy);
}

/// [`axis_quat`] for an axis on the stack (x into f1, y into f2, z into
/// f4, cosine into f3; f0 ends as z·sin).
fn axis_quat_v(c: &mut Cpu, axis: u32, sincos: u32, out: u32) {
    let s = ldf(c, 0, sincos);
    let ax = ldf(c, 1, axis);
    let ay = ldf(c, 2, p(axis, 4));
    let az = ldf(c, 4, p(axis, 8));
    let qx = setf(c, 1, s * ax);
    let qy = setf(c, 2, s * ay);
    let w = ldf(c, 3, p(sincos, 4));
    let qz = setf(c, 0, s * az);
    Addr(p(out, 0xc)).write_fpu(c, w);
    Addr(out).write_fpu(c, qx);
    Addr(p(out, 4)).write_fpu(c, qy);
    Addr(p(out, 8)).write_fpu(c, qz);
}

// ---------------------------------------------------------------------------
// Rider_UpdateBounds.

/// Corners 2..8 of the box: (forward, up, side) signs (true: +) and the
/// side extent (+250 / −50).
const BOUNDS_CORNERS: [([bool; 3], u32); 7] = [
    ([true, true, false], 0x4248_0000),
    ([true, false, true], 0x437a_0000),
    ([true, false, false], 0x4248_0000),
    ([false, true, true], 0x437a_0000),
    ([false, true, false], 0x4248_0000),
    ([false, false, true], 0x437a_0000),
    ([false, false, false], 0x4248_0000),
];

/// `void Rider_UpdateBounds(Rider* r, const Vec4* centre)` (0x11e150): the
/// world AABB (`+0x400` min, `+0x410` max) of the box centred on `centre`
/// (or the rider position `+0x110` when null) spanned by forward
/// (`+0x1b0`, ±150), up (`+0x1a0`, ±100) and side (`+0x1c0`, +250/−50);
/// `+0x420` = the centre with w = 250. Leaf function, 0xa0 frame.
pub fn rider_update_bounds(c: &mut Cpu) {
    c.cycles += 65;
    c.g[A2][0] = c.g[A0][0];
    let frame = Frame::open(c, 0xa0);
    let r = r32(c, A2);
    c.lq(V0, p(r, 0x1b0));
    c.g[A3][0] = sx(p(r, 0x110));
    c.lq(V1, p(r, 0x1c0));
    if c.g[A1][0] != 0 {
        c.g[A3][0] = c.g[A1][0];
    }
    c.lq(A0, p(r, 0x1a0));
    c.g[T0][0] = sx(p(r, 0x400));
    let sp = sp(c);
    c.sq(V1, sp);
    c.sq(A0, p(sp, 0x10));
    c.sq(V0, p(sp, 0x20));
    li_f(c, 3, 0x4316_0000); // 150
    let centre = r32(c, A3);
    c.lq(V0, centre);
    li_f(c, 2, 0x42c8_0000); // 100
    c.sq(V0, p(r, 0x400));
    c.sq(V0, p(r, 0x410));
    bounds_corner(c, r, centre, V1, 0x437a_0000, [true, true, true]);
    for (i, (signs, side)) in BOUNDS_CORNERS.iter().enumerate() {
        c.cycles += if i == 1 { 49 } else { 47 };
        if i == 1 {
            c.g[A0][0] = sx(p(r, 0x400));
            c.g[A1][0] = c.g[A0][0];
        }
        bounds_corner(c, r, centre, V0, *side, *signs);
    }
    c.cycles += 7;
    c.lq(V0, centre);
    li_f(c, 0, 0x437a_0000);
    c.sq(V0, p(r, 0x420));
    stf(c, 0, p(r, 0x42c));
    frame.close(c);
}

/// One corner: `centre ± fwd*f3 ± up*f2 ± side*side_len` (the original's
/// stack round trips), then the box grown to it (the first compare is
/// part of the corner's block, each further one costs 6 cycles).
fn bounds_corner(c: &mut Cpu, r: u32, centre: u32, via: usize, side: u32, signs: [bool; 3]) {
    let sp = sp(c);
    v_scale(c, via, 3, p(sp, 0x20), p(sp, 0x70));
    li_f(c, 0, side);
    qcopy(c, V0, p(sp, 0x70), p(sp, 0x60));
    v_add_sub(c, signs[0], centre, p(sp, 0x60), p(sp, 0x70));
    qcopy(c, V0, p(sp, 0x70), p(sp, 0x50));
    v_scale(c, V1, 2, p(sp, 0x10), p(sp, 0x80));
    qcopy(c, V0, p(sp, 0x80), p(sp, 0x70));
    v_add_sub(c, signs[1], p(sp, 0x50), p(sp, 0x70), p(sp, 0x80));
    qcopy(c, V0, p(sp, 0x80), p(sp, 0x40));
    v_scale(c, V1, 0, sp, p(sp, 0x90));
    qcopy(c, V0, p(sp, 0x90), p(sp, 0x80));
    v_add_sub(c, signs[2], p(sp, 0x40), p(sp, 0x80), p(sp, 0x90));
    qcopy(c, V0, p(sp, 0x90), p(sp, 0x30));
    let corner = p(sp, 0x30);
    for k in 0..3 {
        if k > 0 {
            c.cycles += 6;
        }
        grow_bound(c, p(corner, 4 * k), p(r, 0x400 + 4 * k), true);
    }
    for k in 0..3 {
        c.cycles += 6;
        grow_bound(c, p(corner, 4 * k), p(r, 0x410 + 4 * k), false);
    }
}

/// `f1 = corner, f0 = bound`; store the corner when it is below (`lower`)
/// or above the bound.
fn grow_bound(c: &mut Cpu, corner: u32, bound: u32, lower: bool) {
    let v = ldf(c, 1, corner);
    let b = ldf(c, 0, bound);
    let out = if lower { v.lt(b) } else { b.lt(v) };
    if fcc(c, out) {
        Addr(bound).write_fpu(c, v);
    }
}

// ---------------------------------------------------------------------------
// Rider_UpdateBoardLean.

/// `void Rider_UpdateBoardLean(Rider* r, Xform* xf)` (0x11fa10): roll
/// (`+0x208`) and pitch (`+0x1f0`) beyond twice the dead zone (`+0x214`)
/// rotate `xf` (`{pos, quat}`) about the pivot bone (`+0x8a0`) by
/// quaternions about the X and Z axes; the spin input (`+0x250`) yaws it;
/// finally `xf.pos += +0x3b0 * scale.x * pitch * 45`.
pub fn rider_update_board_lean(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x120);
        c.f[2] = 0;
        frame.save(c, S4, 0xb0);
        frame.save(c, S5, 0xa0);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S0, 0xf0);
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, S1, 0xe0);
        frame.save(c, S2, 0xd0);
        frame.save(c, S3, 0xc0);
        frame.save_ra(c, 0x90);
        frame.save_fpr(c, 22, 0x110);
        frame.save_fpr(c, 21, 0x108);
        frame.save_fpr(c, 20, 0x100);
        let r = r32(c, S5);
        let xf = r32(c, S4);
        let roll = dead_zone(c, p(r, 0x208), p(r, 0x214), 6);
        c.cycles += 10;
        let pitch = dead_zone(c, p(r, 0x1f0), p(r, 0x214), 22);
        c.cycles += 5;
        c.f[0] = 0;
        let mut lean = !fcc(c, pitch.eq_s(FpuF32::ZERO));
        if !lean {
            c.cycles += 4;
            lean = !fcc(c, roll.eq_s(FpuF32::ZERO));
            if lean {
                c.cycles += 1;
            }
        }
        if lean {
            c.g[V1][0] = lw(c, p(r, rider::MODEL));
            c.cycles += 154;
            lean_board(c, r, xf, roll, pitch)?;
        } else {
            ldf(c, 1, p(r, 0x250));
        }
        c.cycles += 4;
        let spin = fr(c, 1);
        if fcc(c, spin.eq_s(FpuF32::ZERO)) {
            ldf(c, 1, p(r, 0x1f0));
        } else {
            c.cycles += 40;
            spin_board(c, r, xf, spin)?;
        }
        // xf.pos += body offset * scale.x * pitch * 45
        c.cycles += 29;
        let k = li_f(c, 2, 0x4234_0000);
        c.g[V0][0] = lw(c, p(r, rider::MODEL));
        let model = r32(c, V0);
        let pitch_now = fr(c, 1);
        let s = setf(c, 1, pitch_now * k);
        frame.restore(c, S0, 0xf0);
        let scale = ldf(c, 0, p(model, model::SCALE));
        frame.restore(c, S1, 0xe0);
        setf(c, 0, scale * s);
        frame.restore(c, S2, 0xd0);
        let sp = sp(c);
        v_scale(c, V1, 0, p(r, 0x3b0), p(sp, 0x10));
        frame.restore(c, S3, 0xc0);
        c.lq(V0, p(sp, 0x10));
        frame.restore(c, S5, 0xa0);
        c.sq(V0, sp);
        v_add(c, xf, sp, xf);
        frame.restore_ra(c, 0x90);
        frame.restore(c, S4, 0xb0);
        frame.restore_fpr(c, 22, 0x110);
        frame.restore_fpr(c, 21, 0x108);
        frame.restore_fpr(c, 20, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `fOut = sign(v) * max(|v| - 2|dead|, 0)` (f0 = 2|dead|, f1 = |v|, f3 =
/// v, f2 = 0 as the original leaves them).
fn dead_zone(c: &mut Cpu, value_at: u32, dead_at: u32, out: usize) -> FpuF32 {
    let dead = ldf(c, 0, dead_at);
    let dead = setf(c, 0, dead.abs());
    let v = ldf(c, 3, value_at);
    let dead2 = setf(c, 0, dead + dead);
    let mag = setf(c, 1, v.abs());
    c.f[2] = 0;
    let r = setf(c, out, mag - dead2);
    if fcc(c, r.lt(FpuF32::ZERO)) {
        c.cycles += 2;
        setf(c, out, FpuF32::ZERO)
    } else {
        c.cycles += 4;
        if fcc(c, v.lt(FpuF32::ZERO)) {
            setf(c, out, -r)
        } else {
            r
        }
    }
}

/// The lean rotation (154-cycle block; v1 = the model): move the pivot
/// bone offset into place, rotate about X by roll and Z by pitch (half
/// angles through `Math_SinCos`), move back. Ends with f1 = `+0x250`,
/// f0 = 0.
fn lean_board(c: &mut Cpu, r: u32, xf: u32, roll: FpuF32, pitch: FpuF32) -> Flow {
    let sp = sp(c);
    let m = r32(c, V1);
    c.g[A0][0] = sx(p(sp, 0x70));
    c.g[V0][0] = lw(c, p(r, 0x8a0));
    c.g[A1][0] = sx(p(sp, 0x74));
    c.g[A2][0] = lw(c, p(m, model::BONE_OFFSETS));
    c.g[S0][0] = 0x50_0000;
    let off = r32(c, V0) << 4;
    c.g[V0][0] = sx(off);
    let sw_ = ldf(c, 0, p(m, model::SCALE + 0xc));
    let bone = off.wrapping_add(r32(c, A2));
    c.g[V0][0] = sx(bone);
    let sx_ = ldf(c, 3, p(m, model::SCALE));
    let bw = ldf(c, 4, p(bone, 0xc));
    c.g[S1][0] = sx(AXIS_X);
    let bx = ldf(c, 1, bone);
    c.g[S3][0] = 0x50_0000;
    setf(c, 4, bw * sw_);
    let sy = ldf(c, 5, p(m, model::SCALE + 4));
    setf(c, 1, bx * sx_);
    let by = ldf(c, 2, p(bone, 4));
    let sz = ldf(c, 3, p(m, model::SCALE + 8));
    c.g[S2][0] = sx(AXIS_Z);
    let bz = ldf(c, 0, p(bone, 8));
    setf(c, 2, by * sy);
    c.g[V0][0] = 0x50_0000;
    stf(c, 1, p(sp, 0x10));
    setf(c, 0, bz * sz);
    stf(c, 4, p(sp, 0x1c));
    stf(c, 2, p(sp, 0x14));
    c.g[V0][0] = sx(PIVOT);
    let k_roll = ldf(c, 12, gp(c, -0x7970));
    stf(c, 0, p(sp, 0x18));
    // xf.pos += rotate(xf.rot, offset - pivot)
    v_sub(c, p(sp, 0x10), PIVOT, p(sp, 0x20));
    let a = setf(c, 12, roll * k_roll);
    c.lq(V0, p(sp, 0x20));
    let half = li_f(c, 21, 0x3f00_0000);
    c.sq(V0, sp);
    q_rotate(c, p(xf, 0x10), sp, p(sp, 0x20));
    setf(c, 12, a * half);
    c.lq(V0, p(sp, 0x20));
    let k_pitch = ldf(c, 20, gp(c, -0x796c));
    c.sq(V0, p(sp, 0x10));
    v_add(c, xf, p(sp, 0x10), xf);
    call_ptr(c, callee::MATH_SIN_COS, 0x0011_fbe0)?;
    // xf.rot ⊗= {X * sin, cos}
    let b = setf(c, 20, pitch * k_pitch);
    c.g[A0][0] = sx(p(sp, 0x78));
    c.g[A1][0] = sx(p(sp, 0x7c));
    axis_quat(c, AXIS_X, p(sp, 0x70), p(sp, 0x40), 3, 2);
    setf(c, 12, b * half);
    qcopy(c, V0, p(sp, 0x40), p(sp, 0x30));
    q_mul(c, p(xf, 0x10), p(sp, 0x30), p(sp, 0x60));
    c.lq(V0, p(sp, 0x60));
    c.sq(V0, p(xf, 0x10));
    c.sq(V0, p(sp, 0x50));
    call_ptr(c, callee::MATH_SIN_COS, 0x0011_fc68)?;
    // xf.rot ⊗= {Z * sin, cos}
    axis_quat(c, AXIS_Z, p(sp, 0x78), p(sp, 0x20), 2, 3);
    li_f(c, 5, 0xbf80_0000);
    qcopy(c, V0, p(sp, 0x20), p(sp, 0x10));
    q_mul(c, p(xf, 0x10), p(sp, 0x10), p(sp, 0x40));
    c.lq(V0, p(sp, 0x40));
    c.sq(V0, p(xf, 0x10));
    c.sq(V0, p(sp, 0x30));
    // xf.pos += rotate(xf.rot, -(offset - pivot))
    v_scale(c, V1, 5, sp, p(sp, 0x20));
    qcopy(c, V0, p(sp, 0x20), p(sp, 0x10));
    q_rotate(c, p(xf, 0x10), p(sp, 0x10), p(sp, 0x30));
    qcopy(c, V0, p(sp, 0x30), p(sp, 0x20));
    v_add(c, xf, p(sp, 0x20), xf);
    ldf(c, 1, p(r, 0x250));
    c.f[0] = 0;
    Ok(())
}

/// The spin yaw (40-cycle block): `xf.rot ⊗= {X * sin, cos}` of
/// `spin * k * 0.5`. Ends with f1 = `+0x1f0`.
fn spin_board(c: &mut Cpu, r: u32, xf: u32, spin: FpuF32) -> Flow {
    let sp = sp(c);
    let k = ldf(c, 12, gp(c, -0x7968));
    c.g[S1][0] = 0x50_0000;
    let half = li_f(c, 0, 0x3f00_0000);
    c.g[A0][0] = sx(p(sp, 0x80));
    let a = setf(c, 12, spin * k);
    c.g[A1][0] = sx(p(sp, 0x84));
    c.g[S0][0] = sx(AXIS_X);
    setf(c, 12, a * half);
    call_ptr(c, callee::MATH_SIN_COS, 0x0011_fd8c)?;
    axis_quat(c, AXIS_X, p(sp, 0x80), p(sp, 0x10), 2, 3);
    qcopy(c, V0, p(sp, 0x10), sp);
    q_mul(c, p(xf, 0x10), sp, p(sp, 0x30));
    c.lq(V0, p(sp, 0x30));
    c.sq(V0, p(xf, 0x10));
    c.sq(V0, p(sp, 0x20));
    ldf(c, 1, p(r, 0x1f0));
    Ok(())
}

// ---------------------------------------------------------------------------
// Rider_SolveLegIK.

/// `void Rider_SolveLegIK(Rider* r, const int bones[3], const Xform* target,
/// float blend)` (0x11f3d8): `bones` = {hip, knee, foot}. The foot
/// transform moves to the target (`blend == 1`) or towards it (position
/// lerp, `Quat_Slerp`); the knee angle follows from the two bone lengths
/// (the x of the scaled bone offsets) and the hip-foot distance; the knee
/// transform (written last) hangs off the hip, which is swung about
/// `knee × foot` so that the leg reaches the foot.
pub fn rider_solve_leg_ik(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x1b0);
        let one = li_f(c, 0, 0x3f80_0000);
        frame.save(c, S0, 0x180);
        let blend = fr(c, 12);
        setf(c, 1, blend);
        frame.save(c, S1, 0x170);
        let full = fcc(c, blend.eq_s(one));
        frame.save(c, S2, 0x160);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x150);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x140);
        frame.save_ra(c, 0x130);
        frame.save_fpr(c, 23, 0x1a8);
        frame.save_fpr(c, 22, 0x1a0);
        frame.save_fpr(c, 21, 0x198);
        frame.save_fpr(c, 20, 0x190);
        let r = r32(c, S2);
        let leg = r32(c, S1);
        let target = r32(c, A2);
        c.g[A0][0] = lw(c, p(r, rider::MODEL));
        let m = r32(c, A0);
        c.g[V0][0] = lw(c, p(leg, 8));
        c.g[V1][0] = lw(c, p(m, model::BONES));
        let off = r32(c, V0) << 5;
        c.g[V0][0] = sx(off);
        let foot = r32(c, V1).wrapping_add(off);
        c.g[S0][0] = sx(foot);
        if full {
            c.cycles += 5;
            qcopy(c, V0, target, foot);
            qcopy(c, V1, p(target, 0x10), p(foot, 0x10));
        } else {
            c.cycles += 27;
            let sp = sp(c);
            // foot.pos += (target.pos - foot.pos) * blend
            v_sub(c, target, foot, p(sp, 0x30));
            setf(c, 12, blend);
            c.lq(V0, p(sp, 0x30));
            c.g[A2][0] = sx(p(target, 0x10));
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = sx(p(foot, 0x10));
            c.sq(V0, p(sp, 0x20));
            v_scale(c, V1, 1, p(sp, 0x20), p(sp, 0x30));
            qcopy(c, V0, p(sp, 0x30), p(sp, 0x10));
            v_add(c, foot, p(sp, 0x10), p(sp, 0x30));
            c.lq(V0, p(sp, 0x30));
            c.sq(V0, sp);
            c.sq(V0, foot);
            // Quat_Slerp(&tmp, foot.rot, target.rot, blend)
            call_ptr(c, callee::QUAT_SLERP, 0x0011_f4ac)?;
            qcopy(c, V0, sp, p(foot, 0x10));
        }

        // Bone lengths and the knee angle.
        c.cycles += 79;
        let sp = sp(c);
        c.g[V1][0] = lw(c, p(r, rider::MODEL));
        let m = r32(c, V1);
        c.g[A1][0] = lw(c, p(leg, 4));
        c.g[A0][0] = lw(c, p(m, model::BONE_OFFSETS));
        let knee_i = r32(c, A1);
        let off = knee_i << 4;
        c.g[V0][0] = sx(off);
        let s_x = ldf(c, 3, p(m, model::SCALE));
        let bp = off.wrapping_add(r32(c, A0));
        c.g[V0][0] = sx(bp);
        let s_y = ldf(c, 5, p(m, model::SCALE + 4));
        let bx = ldf(c, 0, bp);
        c.g[A1][0] = sx(knee_i << 5);
        let bw = ldf(c, 4, p(bp, 0xc));
        let upper = setf(c, 20, bx * s_x);
        let by = ldf(c, 1, p(bp, 4));
        let s_z = ldf(c, 0, p(m, model::SCALE + 8));
        let s_w = ldf(c, 3, p(m, model::SCALE + 0xc));
        setf(c, 1, by * s_y);
        let bz = ldf(c, 2, p(bp, 8));
        let upper2 = setf(c, 5, upper * upper);
        setf(c, 4, bw * s_w);
        c.g[A2][0] = lw(c, p(m, model::BONES));
        setf(c, 2, bz * s_z);
        stf(c, 1, p(sp, 4));
        stf(c, 20, sp);
        let knee = r32(c, A2).wrapping_add(r32(c, A1));
        c.g[S4][0] = sx(knee);
        stf(c, 4, p(sp, 0xc));
        stf(c, 2, p(sp, 8));
        let half = li_f(c, 23, 0x3f00_0000);
        c.g[V0][0] = lw(c, p(leg, 8));
        c.g[A0][0] = lw(c, p(m, model::BONE_OFFSETS));
        let off = r32(c, V0) << 4;
        c.g[V0][0] = sx(off);
        let s_w = ldf(c, 3, p(m, model::SCALE + 0xc));
        let bp = off.wrapping_add(r32(c, A0));
        c.g[V0][0] = sx(bp);
        let s_x = ldf(c, 4, p(m, model::SCALE));
        let bx = ldf(c, 0, bp);
        let bw = ldf(c, 1, p(bp, 0xc));
        let lower = setf(c, 21, bx * s_x);
        let by = ldf(c, 2, p(bp, 4));
        setf(c, 1, bw * s_w);
        let bz = ldf(c, 0, p(bp, 8));
        let s_y = ldf(c, 3, p(m, model::SCALE + 4));
        let s_z = ldf(c, 4, p(m, model::SCALE + 8));
        c.g[V0][0] = lw(c, leg);
        setf(c, 2, by * s_y);
        setf(c, 0, bz * s_z);
        stf(c, 1, p(sp, 0xc));
        let hip_off = r32(c, V0) << 5;
        c.g[V0][0] = sx(hip_off);
        stf(c, 21, sp);
        stf(c, 2, p(sp, 4));
        let hip = r32(c, A2).wrapping_add(hip_off);
        c.g[S2][0] = sx(hip);
        stf(c, 0, p(sp, 8));
        let lower2 = setf(c, 3, lower * lower);
        v_sub(c, foot, hip, p(sp, 0x10));
        let prod = setf(c, 2, upper * lower);
        c.lq(V0, p(sp, 0x10));
        let neg_one = li_f(c, 1, 0xbf80_0000);
        c.sq(V0, sp);
        v_dot(c, sp, sp);
        c.qmfc2(V1, 4);
        c.mtc1(V1, 0);
        let dist2 = fr(c, 0);
        let t = setf(c, 0, dist2 - upper2);
        let one22 = li_f(c, 22, 0x3f80_0000);
        let t = setf(c, 0, t - lower2);
        let t = setf(c, 0, t * half);
        let cos = setf(c, 0, t.div(prod));
        let in_range = fcc(c, neg_one.le(cos));
        setf(c, 12, neg_one);
        if in_range {
            c.cycles += 1;
            setf(c, 12, cos.min(one22));
        }

        // The knee: rotate about AXIS_Y by asin(cos) - pi/2.
        c.cycles += 159;
        c.g[S0][0] = 0x50_0000;
        call_ptr(c, callee::MATH_ASIN, 0x0011_f5fc)?;
        let quarter = ldf(c, 1, gp(c, -0x7978));
        c.g[A0][0] = sx(p(sp, 0x120));
        stf(c, 20, p(sp, 0x30));
        c.g[A1][0] = sx(p(sp, 0x124));
        let a = fr(c, 0);
        let a = setf(c, 0, a - quarter);
        c.sw(p(sp, 0x34), 0);
        c.sw(p(sp, 0x38), 0);
        c.g[S1][0] = sx(AXIS_Y);
        stf(c, 22, p(sp, 0x3c));
        c.g[S3][0] = 0x50_0000;
        setf(c, 12, a * half);
        qcopy(c, V0, p(sp, 0x30), p(sp, 0x10));
        call_ptr(c, callee::MATH_SIN_COS, 0x0011_f634)?;
        axis_quat(c, AXIS_Y, p(sp, 0x120), p(sp, 0x30), 2, 3);
        let eps = ldf(c, 5, gp(c, -0x7974));
        qcopy(c, V0, p(sp, 0x30), p(sp, 0x20));
        // Where the knee and foot end up from the hip.
        q_mul(c, p(hip, 0x10), p(sp, 0x20), p(sp, 0x90));
        c.lq(V0, p(sp, 0x90));
        c.sq(V0, p(sp, 0x70));
        c.sq(V0, p(sp, 0x80));
        v_sub(c, p(sp, 0x10), PIVOT, p(sp, 0xb0));
        qcopy(c, V0, p(sp, 0xb0), p(sp, 0xa0));
        q_rotate(c, p(hip, 0x10), p(sp, 0xa0), p(sp, 0xb0));
        qcopy(c, V0, p(sp, 0xb0), p(sp, 0x90));
        v_add(c, p(sp, 0x90), hip, p(sp, 0xb0));
        c.lq(V0, p(sp, 0xb0));
        c.sq(V0, p(sp, 0x80));
        c.lq(V1, p(sp, 0x70));
        c.sq(V0, p(sp, 0x60));
        c.sq(V0, p(sp, 0x40));
        c.sq(V1, p(sp, 0x50));
        stf(c, 21, p(sp, 0x70));
        stf(c, 22, p(sp, 0x7c));
        c.sw(p(sp, 0x74), 0);
        c.sw(p(sp, 0x78), 0);
        v_sub(c, p(sp, 0x70), PIVOT, p(sp, 0xa0));
        qcopy(c, V0, p(sp, 0xa0), p(sp, 0x90));
        q_rotate(c, p(sp, 0x50), p(sp, 0x90), p(sp, 0xa0));
        qcopy(c, V0, p(sp, 0xa0), p(sp, 0x80));
        v_add(c, p(sp, 0x80), p(sp, 0x40), p(sp, 0xa0));
        qcopy(c, V0, p(sp, 0xa0), p(sp, 0x60));
        v_sub(c, p(sp, 0x60), hip, p(sp, 0x80));
        qcopy(c, V0, p(sp, 0x80), p(sp, 0x30));
        // The swing axis: reached-foot direction × wanted-foot direction.
        v_normalize(c, p(sp, 0x30), p(sp, 0x80));
        qcopy(c, V0, p(sp, 0x80), p(sp, 0x70));
        v_normalize(c, sp, p(sp, 0x90));
        qcopy(c, V0, p(sp, 0x90), p(sp, 0x80));
        v_cross(c, p(sp, 0x70), p(sp, 0x80), p(sp, 0x90));
        qcopy(c, V0, p(sp, 0x90), p(sp, 0x60));
        let len = v_length(c, p(sp, 0x60), V1, 0);
        if fcc(c, eps.lt(len)) {
            c.cycles += 42;
            v_div_by(c, 0, p(sp, 0x60), p(sp, 0x60));
            setf(c, 12, len);
            call_ptr(c, callee::MATH_ASIN, 0x0011_f894)?;
            let a = fr(c, 0);
            setf(c, 12, a * half);
            c.g[A0][0] = sx(p(sp, 0x128));
            c.g[A1][0] = sx(p(sp, 0x12c));
            call_ptr(c, callee::MATH_SIN_COS, 0x0011_f8a4)?;
            axis_quat_v(c, p(sp, 0x60), p(sp, 0x128), p(sp, 0x80));
            qcopy(c, V0, p(sp, 0x80), p(sp, 0x70));
            q_mul(c, p(sp, 0x70), p(hip, 0x10), p(sp, 0x90));
            c.lq(V0, p(sp, 0x90));
            c.sq(V0, p(sp, 0x80));
            c.sq(V0, p(hip, 0x10));
        }

        // The knee transform from the (swung) hip.
        c.cycles += 61;
        q_mul(c, p(hip, 0x10), p(sp, 0x20), p(sp, 0xf0));
        frame.restore(c, S0, 0x180);
        c.lq(V0, p(sp, 0xf0));
        frame.restore(c, S1, 0x170);
        c.sq(V0, p(sp, 0xd0));
        c.sq(V0, p(sp, 0xe0));
        v_sub(c, p(sp, 0x10), PIVOT, p(sp, 0x110));
        frame.restore_ra(c, 0x130);
        c.lq(V0, p(sp, 0x110));
        frame.restore(c, S3, 0x150);
        c.sq(V0, p(sp, 0x100));
        q_rotate(c, p(hip, 0x10), p(sp, 0x100), p(sp, 0x110));
        frame.restore_fpr(c, 23, 0x1a8);
        c.lq(V0, p(sp, 0x110));
        frame.restore_fpr(c, 22, 0x1a0);
        c.sq(V0, p(sp, 0xf0));
        v_add(c, p(sp, 0xf0), hip, p(sp, 0x110));
        frame.restore_fpr(c, 21, 0x198);
        c.lq(V1, p(sp, 0x110));
        c.lq(V0, p(sp, 0xd0));
        c.sq(V1, p(sp, 0xe0));
        c.sq(V0, p(sp, 0x80));
        c.sq(V1, p(sp, 0xc0));
        c.sq(V1, p(sp, 0x70));
        c.sq(V1, knee);
        frame.restore(c, S2, 0x160);
        c.lq(V0, p(sp, 0x80));
        frame.restore_fpr(c, 20, 0x190);
        c.sq(V0, p(knee, 0x10));
        frame.restore(c, S4, 0x140);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider_UpdateSkeleton.

/// `void Rider_UpdateSkeleton(Rider* r)` (0x11eb98): the per-frame pose.
/// The root transform (`+0x110` pos, `+0x120` rot) is copied to the stack,
/// spun (`RiderSpinSub_ApplyPoseRotation`, sub-state 5), leaned
/// (`Rider_UpdateBoardLean`) and offset by the spin input (`+0x2c8`); the
/// root bones are reset when `+0x150` is set; `RiderModel_ApplyPose`;
/// then the spine twist (`+0x2bc`, the bone `+0x8a0` turned towards
/// `+0x390`) and tilt (`+0x31c`), the pelvis (`+0x8a4`) re-hung, both legs
/// solved (`+0x318`, not in sub-state 13), and the world matrix (`+0x160`)
/// built from the root transform before `Rider_UpdateSphereTree`.
pub fn rider_update_skeleton(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x1b0);
        frame.save(c, S3, 0x170);
        frame.save(c, S0, 0x1a0);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x190);
        frame.save(c, S2, 0x180);
        frame.save(c, S4, 0x160);
        frame.save(c, S5, 0x150);
        frame.save_ra(c, 0x140);
        let r = r32(c, S3);
        let sp = sp(c);
        c.lq(V1, p(r, 0x120));
        c.lq(V0, p(r, 0x110));
        c.sq(V1, p(sp, 0x10));
        c.sq(V0, sp);
        callee::RIDER_GET_SUB_STATE.call(c, 0x0011_ebd0)?;
        c.g[V1][0] = 5;
        if c.g[V0][0] == c.g[V1][0] {
            c.cycles += 5;
            c.g[A0][0] = lw(c, p(r, rider::STATE));
            c.g[A1][0] = c.g[SP][0];
            let spin = r32(c, A0).wrapping_add(0x230);
            c.g[A0][0] = sx(spin);
            call_ptr(c, callee::RIDER_SPIN_SUB_APPLY_POSE_ROTATION, 0x0011_ebec)?;
        }
        c.g[A0][0] = c.g[S3][0];
        c.cycles += 8;
        c.g[A1][0] = c.g[SP][0];
        callee::RIDER_UPDATE_BOARD_LEAN.call(c, 0x0011_ebf8)?;
        c.g[S5][0] = sx(p(r, 0x160));
        c.g[S0][0] = sx(p(sp, 0x20));
        c.g[S4][0] = sx(p(sp, 0x10));
        c.g[V1][0] = u64::MAX;
        c.g[V0][0] = 1;
        // A loop the compiler left behind: two passes, nothing in it.
        loop {
            c.cycles += 7;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
            let more = c.g[V0][0] != c.g[V1][0];
            c.g[A0][0] = c.g[S3][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0011_ec10);
            }
        }

        // The pose root: the transform, offset along AXIS_Y by the spin.
        c.cycles += 10;
        c.lq(V1, sp);
        c.lq(V0, p(sp, 0x10));
        c.sq(V1, p(sp, 0x20));
        c.sq(V1, p(sp, 0x40));
        c.sq(V0, p(sp, 0x30));
        c.sq(V0, p(sp, 0x50));
        callee::RIDER_GET_SUB_STATE.call(c, 0x0011_ec48)?;
        c.g[V1][0] = 0xd;
        if c.g[V0][0] != c.g[V1][0] {
            c.cycles += 6;
            let spin = ldf(c, 1, p(r, 0x2c8));
            c.f[0] = 0;
            if !fcc(c, spin.eq_s(FpuF32::ZERO)) {
                c.cycles += 29;
                c.g[V1][0] = sx(AXIS_Y);
                v_scale(c, V0, 1, AXIS_Y, p(sp, 0x70));
                qcopy(c, V1, p(sp, 0x70), p(sp, 0x60));
                q_rotate(c, p(sp, 0x30), p(sp, 0x60), p(sp, 0x90));
                qcopy(c, V0, p(sp, 0x90), p(sp, 0x80));
                v_add(c, p(sp, 0x20), p(sp, 0x80), p(sp, 0x20));
            }
        }
        c.g[V0][0] = lw(c, p(r, 0x150));
        c.cycles += 2;
        c.g[A2][0] = 0x50_0000;
        if c.g[V0][0] != 0 {
            c.cycles += 35;
            reset_root_bones(c, r);
        }

        // Forward kinematics.
        c.cycles += 8;
        c.g[A0][0] = lw(c, p(r, rider::MODEL));
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, callee::RIDER_MODEL_APPLY_POSE, 0x0011_ed80)?;
        c.f[10] = 0;
        let twist = ldf(c, 0, p(r, 0x2bc));
        if !fcc(c, twist.eq_s(FpuF32::ZERO)) {
            c.cycles += 51;
            spine_twist(c, r)?;
            let bone = r32(c, S0);
            c.cycles += 5;
            c.f[0] = 0;
            let tilt = fr(c, 1);
            if fcc(c, FpuF32::ZERO.lt(tilt)) {
                c.cycles += 20;
                v_scale(c, V1, 1, p(r, 0x370), p(sp, 0xa0));
                c.lq(V0, p(sp, 0xa0));
                ldf(c, 0, p(r, 0x2bc));
                c.sq(V0, p(sp, 0x90));
                v_scale(c, V1, 0, p(sp, 0x90), p(sp, 0xa0));
                qcopy(c, V0, p(sp, 0xa0), p(sp, 0x80));
                v_add(c, bone, p(sp, 0x80), bone);
            }
            c.g[A0][0] = lw(c, p(r, rider::MODEL));
            c.cycles += 80;
            rehang_pelvis(c, r, bone);
        }

        // Legs.
        c.cycles += 5;
        c.g[A0][0] = c.g[S3][0];
        callee::RIDER_GET_SUB_STATE.call(c, 0x0011_f154)?;
        c.g[V1][0] = 0xd;
        if c.g[V0][0] != c.g[V1][0] {
            c.cycles += 6;
            let ik = ldf(c, 1, p(r, 0x318));
            c.f[0] = 0;
            let off = fcc(c, ik.eq_s(FpuF32::ZERO));
            c.g[S2][0] = 0x50_0000;
            if !off {
                c.cycles += 108;
                c.g[V1][0] = lw(c, p(r, rider::MODEL));
                c.g[V0][0] = lw(c, p(r, 0x8a4));
                c.g[S1][0] = sx(p(sp, 0x60));
                let m = r32(c, V1);
                c.g[S0][0] = lw(c, p(m, model::BONES));
                let o = r32(c, V0) << 5;
                c.g[V0][0] = sx(o);
                let pelvis = r32(c, S0).wrapping_add(o);
                c.g[S0][0] = sx(pelvis);
                leg(c, r, pelvis, 0x8e0, 0x0011_f25c)?;
                leg(c, r, pelvis, 0x910, 0x0011_f328)?;
            }
        }

        // World matrix from the root transform (quaternion → rows).
        c.cycles += 44;
        let rot = r32(c, S4);
        c.lqc2(4, rot);
        c.lqc2(3, sp);
        quat_to_matrix(c);
        let world = r32(c, S5);
        c.sqc2(3, p(world, 0x30));
        c.sqc2(10, world);
        c.sqc2(11, p(world, 0x10));
        c.sqc2(12, p(world, 0x20));
        c.g[A0][0] = c.g[S3][0];
        callee::RIDER_UPDATE_SPHERE_TREE.call(c, 0x0011_f3b4)?;
        frame.restore(c, S0, 0x1a0);
        frame.restore(c, S1, 0x190);
        frame.restore(c, S2, 0x180);
        frame.restore(c, S3, 0x170);
        frame.restore(c, S4, 0x160);
        frame.restore(c, S5, 0x150);
        frame.restore_ra(c, 0x140);
        frame.close(c);
        Ok(())
    })
}

/// Root bones `+0x8a0` and `+0x8a4`: offsets reset to [`PIVOT`], rotations
/// to the identity; the pose root becomes the rider's `+0x130` transform.
fn reset_root_bones(c: &mut Cpu, r: u32) {
    let sp = sp(c);
    c.g[V1][0] = lw(c, p(r, rider::MODEL));
    c.g[V0][0] = lw(c, p(r, 0x8a0));
    c.g[A2][0] = sx(PIVOT);
    let m = r32(c, V1);
    c.g[A0][0] = lw(c, p(m, model::BONE_OFFSETS));
    c.g[A3][0] = 0x50_0000;
    c.lq(V1, PIVOT);
    let o = r32(c, V0) << 4;
    c.g[V0][0] = sx(o);
    let dst = o.wrapping_add(r32(c, A0));
    c.g[V0][0] = sx(dst);
    c.g[A3][0] = sx(IDENTITY_QUAT);
    c.sq(V1, dst);
    for (index, table, src) in [
        (0x8a0, model::BONE_ROTATIONS, IDENTITY_QUAT),
        (0x8a4, model::BONE_OFFSETS, PIVOT),
        (0x8a4, model::BONE_ROTATIONS, IDENTITY_QUAT),
    ] {
        c.g[A0][0] = lw(c, p(r, rider::MODEL));
        c.g[V0][0] = lw(c, p(r, index));
        let m = r32(c, A0);
        c.g[A1][0] = lw(c, p(m, table));
        c.lq(V1, src);
        let o = r32(c, V0) << 4;
        c.g[V0][0] = sx(o);
        let dst = o.wrapping_add(r32(c, A1));
        c.g[V0][0] = sx(dst);
        c.sq(V1, dst);
    }
    c.lq(A0, p(r, 0x130));
    c.lq(V0, p(r, 0x140));
    c.sq(A0, p(sp, 0x40));
    c.sq(V0, p(sp, 0x50));
}

/// The spine twist (51-cycle block onwards): the bone `+0x8a0` (left in
/// s0) turns about `+0x390 × its x axis` by `+0x2bc` times the angle
/// between them (`asin` of the axis length, mirrored past 90° when the
/// dot product is negative). Ends with f1 = `+0x31c`.
fn spine_twist(c: &mut Cpu, r: u32) -> Flow {
    let sp = sp(c);
    c.g[V1][0] = lw(c, p(r, rider::MODEL));
    c.g[V0][0] = lw(c, p(r, 0x8a0));
    let m = r32(c, V1);
    c.g[A0][0] = lw(c, p(m, model::BONES));
    let o = r32(c, V0) << 5;
    c.g[V0][0] = sx(o);
    let one = li_f(c, 9, 0x3f80_0000);
    let bone = r32(c, A0).wrapping_add(o);
    c.g[S0][0] = sx(bone);
    let min_len = ldf(c, 8, gp(c, -0x7984));
    // The bone's x axis (first matrix column of its quaternion).
    let qy = ldf(c, 4, p(bone, 0x14));
    let qz = ldf(c, 5, p(bone, 0x18));
    let qx = ldf(c, 0, p(bone, 0x10));
    let yy = setf(c, 1, qy * qy);
    let qw = ldf(c, 3, p(bone, 0x1c));
    let zz = setf(c, 7, qz * qz);
    let xz = setf(c, 2, qx * qz);
    stf(c, 10, p(sp, 0x6c));
    let wy = setf(c, 6, qw * qy);
    let t1 = setf(c, 1, yy + zz);
    let xy = setf(c, 0, qx * qy);
    let wz = setf(c, 3, qw * qz);
    let t2 = setf(c, 2, xz - wy);
    let t1 = setf(c, 1, t1 + t1);
    let t0 = setf(c, 0, xy + wz);
    setf(c, 2, t2 + t2);
    setf(c, 1, one - t1);
    setf(c, 0, t0 + t0);
    stf(c, 2, p(sp, 0x68));
    stf(c, 1, p(sp, 0x60));
    stf(c, 0, p(sp, 0x64));
    v_cross(c, p(r, 0x390), p(sp, 0x60), p(sp, 0x80));
    qcopy(c, V0, p(sp, 0x80), p(sp, 0x70));
    let len = v_length(c, p(sp, 0x70), V1, 12);
    if !fcc(c, min_len.lt(len)) {
        ldf(c, 1, p(r, 0x31c));
        return Ok(());
    }
    c.cycles += 20;
    v_div_by(c, 12, p(sp, 0x70), p(sp, 0x70));
    v_dot(c, p(r, 0x390), p(sp, 0x60));
    c.qmfc2(V1, 4);
    c.mtc1(V1, 0);
    let dot = fr(c, 0);
    let zero = fr(c, 10);
    let behind = fcc(c, dot.lt(zero));
    setf(c, 12, len.min(one));
    let sincos = if behind {
        c.cycles += 20;
        call_ptr(c, callee::MATH_ASIN, 0x0011_eeb8)?;
        let k = ldf(c, 1, gp(c, -0x7980));
        c.g[A0][0] = sx(p(sp, 0x130));
        let twist = ldf(c, 2, p(r, 0x2bc));
        c.g[A1][0] = sx(p(sp, 0x134));
        let a = fr(c, 0);
        let t = setf(c, 0, a - k);
        let half = li_f(c, 12, 0x3f00_0000);
        let t = setf(c, 0, t * twist);
        setf(c, 12, t * half);
        call_ptr(c, callee::MATH_SIN_COS, 0x0011_eee0)?;
        p(sp, 0x130)
    } else {
        c.cycles += 19;
        call_ptr(c, callee::MATH_ASIN, 0x0011_ef08)?;
        let k = ldf(c, 12, gp(c, -0x797c));
        c.g[A0][0] = sx(p(sp, 0x138));
        let twist = ldf(c, 1, p(r, 0x2bc));
        c.g[A1][0] = sx(p(sp, 0x13c));
        let a = fr(c, 0);
        let t = setf(c, 12, k - a);
        let half = li_f(c, 0, 0x3f00_0000);
        let t = setf(c, 12, t * twist);
        setf(c, 12, t * half);
        call_ptr(c, callee::MATH_SIN_COS, 0x0011_ef30)?;
        p(sp, 0x138)
    };
    c.cycles += 23;
    axis_quat_v(c, p(sp, 0x70), sincos, p(sp, 0x90));
    qcopy(c, V0, p(sp, 0x90), p(sp, 0x80));
    q_mul(c, p(sp, 0x80), p(bone, 0x10), p(sp, 0xa0));
    c.lq(V0, p(sp, 0xa0));
    c.sq(V0, p(sp, 0x90));
    c.sq(V0, p(bone, 0x10));
    ldf(c, 1, p(r, 0x31c));
    Ok(())
}

/// The pelvis (`+0x8a4`) re-hung from the twisted spine bone `bone`
/// (80-cycle block; a0 = the model): rotation `bone.rot ⊗ local rot`,
/// position `bone.pos + rotate(bone.rot, scaled offset - pivot)`.
fn rehang_pelvis(c: &mut Cpu, r: u32, bone: u32) {
    let sp = sp(c);
    let m = r32(c, A0);
    c.g[A2][0] = lw(c, p(r, 0x8a4));
    c.g[V0][0] = lw(c, p(m, model::BONE_OFFSETS));
    let i = r32(c, A2);
    let o4 = i << 4;
    c.g[A1][0] = sx(o4);
    let s_x = ldf(c, 4, p(m, model::SCALE));
    let bp = o4.wrapping_add(r32(c, V0));
    c.g[V0][0] = sx(bp);
    let s_y = ldf(c, 6, p(m, model::SCALE + 4));
    let x = ldf(c, 0, bp);
    c.g[A2][0] = sx(i << 5);
    let w = ldf(c, 5, p(bp, 0xc));
    setf(c, 0, x * s_x);
    let y = ldf(c, 1, p(bp, 4));
    let z = ldf(c, 2, p(bp, 8));
    let s_z = ldf(c, 3, p(m, model::SCALE + 8));
    setf(c, 1, y * s_y);
    let s_w = ldf(c, 4, p(m, model::SCALE + 0xc));
    setf(c, 2, z * s_z);
    stf(c, 0, p(sp, 0xa0));
    setf(c, 5, w * s_w);
    stf(c, 1, p(sp, 0xa4));
    stf(c, 2, p(sp, 0xa8));
    stf(c, 5, p(sp, 0xac));
    qcopy(c, V0, p(sp, 0xa0), p(sp, 0x80));
    c.g[V1][0] = lw(c, p(m, model::BONE_ROTATIONS));
    let rot = r32(c, V1).wrapping_add(o4);
    c.g[V1][0] = sx(rot);
    qcopy(c, V0, rot, p(sp, 0x90));
    c.g[A1][0] = lw(c, p(m, model::BONES));
    q_mul(c, p(bone, 0x10), p(sp, 0x90), p(sp, 0x100));
    c.lq(V0, p(sp, 0x100));
    let dst = r32(c, A1).wrapping_add(r32(c, A2));
    c.g[A1][0] = sx(dst);
    c.sq(V0, p(sp, 0xe0));
    c.sq(V0, p(sp, 0xf0));
    c.g[V0][0] = sx(PIVOT);
    v_sub(c, p(sp, 0x80), PIVOT, p(sp, 0x120));
    qcopy(c, V0, p(sp, 0x120), p(sp, 0x110));
    q_rotate(c, p(bone, 0x10), p(sp, 0x110), p(sp, 0x120));
    qcopy(c, V0, p(sp, 0x120), p(sp, 0x100));
    v_add(c, p(sp, 0x100), bone, p(sp, 0x120));
    c.lq(V1, p(sp, 0x120));
    c.lq(V0, p(sp, 0xe0));
    c.sq(V1, p(sp, 0xf0));
    c.sq(V0, p(sp, 0xc0));
    c.sq(V1, p(sp, 0xd0));
    c.sq(V1, p(sp, 0xb0));
    c.sq(V1, dst);
    qcopy(c, V0, p(sp, 0xc0), p(dst, 0x10));
}

/// One leg: the hip target `pelvis ⊗ {offset (desc+0x10), rot (desc+0x20)}`
/// on the stack (s1 = sp+0x60), then `Rider_SolveLegIK(r, desc, &target,
/// +0x318)`.
fn leg(c: &mut Cpu, r: u32, pelvis: u32, desc: u32, ret: u32) -> Flow {
    let sp = sp(c);
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = sx(p(r, desc));
    c.g[A2][0] = c.g[S1][0];
    q_mul(c, p(pelvis, 0x10), p(r, desc + 0x20), p(sp, 0xb0));
    c.lq(V0, p(sp, 0xb0));
    c.sq(V0, p(sp, 0x90));
    c.sq(V0, p(sp, 0xa0));
    v_sub(c, p(r, desc + 0x10), PIVOT, p(sp, 0xd0));
    qcopy(c, V0, p(sp, 0xd0), p(sp, 0xc0));
    q_rotate(c, p(pelvis, 0x10), p(sp, 0xc0), p(sp, 0xd0));
    qcopy(c, V0, p(sp, 0xd0), p(sp, 0xb0));
    v_add(c, p(sp, 0xb0), pelvis, p(sp, 0xd0));
    c.lq(V0, p(sp, 0xd0));
    c.lq(V1, p(sp, 0x90));
    ldf(c, 12, p(r, 0x318));
    c.sq(V0, p(sp, 0x60));
    c.sq(V1, p(sp, 0x70));
    c.sq(V0, p(sp, 0xa0));
    c.sq(V0, p(sp, 0x80));
    callee::RIDER_SOLVE_LEG_IK.call(c, ret)
}

/// Rotation rows vf10..vf12 of the quaternion in vf4 (vf3, the position,
/// untouched), as the original's VU0 sequence computes them.
fn quat_to_matrix(c: &mut Cpu) {
    let vu = c.vu0();
    vu.add(0b1110, 1, 0, Src::Bc(0, W));
    vu.add(0b1110, 5, 4, Src::Vec(4));
    vu.sub(0b0001, 10, 10, Src::Vec(10));
    vu.sub(0b0001, 11, 11, Src::Vec(11));
    vu.sub(0b0001, 12, 12, Src::Vec(12));
    vu.mul(0b1110, 6, 5, Src::Vec(4));
    vu.mul(0b1110, 7, 5, Src::Bc(4, W));
    vu.opmula(5, 4);
    vu.madd(0b1110, 8, 0, Src::Vec(0));
    vu.suba(0b1000, 1, Src::Bc(6, Y));
    vu.msub(0b1000, 10, 1, Src::Bc(6, Z));
    vu.suba(0b0100, 1, Src::Bc(6, Z));
    vu.msub(0b0100, 11, 1, Src::Bc(6, X));
    vu.suba(0b0010, 1, Src::Bc(6, X));
    vu.msub(0b0010, 12, 1, Src::Bc(6, Y));
    vu.adda(0b0100, 0, Src::Bc(8, Z));
    vu.madd(0b0100, 10, 1, Src::Bc(7, Z));
    vu.adda(0b0010, 0, Src::Bc(8, X));
    vu.madd(0b0010, 11, 1, Src::Bc(7, X));
    vu.adda(0b0100, 0, Src::Bc(8, X));
    vu.msub(0b0100, 12, 1, Src::Bc(7, X));
    vu.adda(0b0010, 0, Src::Bc(8, Y));
    vu.msub(0b0010, 10, 1, Src::Bc(7, Y));
    vu.adda(0b1000, 0, Src::Bc(8, Z));
    vu.msub(0b1000, 11, 1, Src::Bc(7, Z));
    vu.adda(0b1000, 0, Src::Bc(8, Y));
    vu.madd(0b1000, 12, 1, Src::Bc(7, Y));
}

// ---------------------------------------------------------------------------
// Rider_UpdateSpeedAnims.

/// One body-part animation channel of `Rider_UpdateSpeedAnims`.
struct Channel {
    /// Animation layer.
    layer: u64,
    /// Seven animation ids: idle, side +/−, forward/back, still, upside down.
    first_anim: u64,
    /// The bone whose rotation orients the channel.
    bone: u32,
    /// The local rotation used when `+0x320` is set, and otherwise.
    quat_alt: u32,
    quat: u32,
    /// The next channel's enable flag (loaded in the original's delay
    /// slots), none for the last.
    next_enable: Option<u32>,
    ret_anim_id: u32,
    ret_play: u32,
}

const CHANNELS: [Channel; 3] = [
    Channel {
        layer: 3,
        first_anim: 0x19b,
        bone: 0x8a8,
        quat_alt: 0x960,
        quat: 0x950,
        next_enable: Some(0x970),
        ret_anim_id: 0x0012_050c,
        ret_play: 0x0012_07b4,
    },
    Channel {
        layer: 4,
        first_anim: 0x1a2,
        bone: 0x8ac,
        quat_alt: 0x990,
        quat: 0x980,
        next_enable: Some(0x9a0),
        ret_anim_id: 0x0012_07cc,
        ret_play: 0x0012_0a74,
    },
    Channel {
        layer: 5,
        first_anim: 0x1a9,
        bone: 0x89c,
        quat_alt: 0x9c0,
        quat: 0x9b0,
        next_enable: None,
        ret_anim_id: 0x0012_0a8c,
        ret_play: 0x0012_0d34,
    },
];

/// `void Rider_UpdateSpeedAnims(Rider* r)` (0x120378): the playback rate of
/// animation layers 3..5 follows the speed (`|+0x1e0 - +0x3d0| / max`,
/// clamped to [0.5, 2]); every sixth frame (phase `+0x86c`) each enabled
/// channel (`+0x940`, `+0x970`, `+0x9a0`) picks its animation from how the
/// velocity lies in the channel bone's frame and plays it if it changed.
pub fn rider_update_speed_anims(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 38;
        let frame = Frame::open(c, 0x90);
        let max_speed = ldf(c, 1, gp(c, -0x7944));
        frame.save(c, S0, 0x60);
        frame.save_fpr(c, 23, 0x88);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 22, 0x80);
        frame.save_fpr(c, 20, 0x70);
        frame.save(c, S1, 0x50);
        frame.save_ra(c, 0x40);
        frame.save_fpr(c, 21, 0x78);
        let r = r32(c, S0);
        let sp = sp(c);
        v_sub(c, p(r, 0x1e0), p(r, 0x3d0), p(sp, 0x10));
        let half = li_f(c, 0, 0x3f00_0000);
        c.lq(V0, p(sp, 0x10));
        setf(c, 20, half);
        ldf(c, 23, gp(c, -0x7940));
        c.sq(V0, sp);
        let speed = v_length(c, sp, V1, 21);
        ldf(c, 22, gp(c, -0x793c));
        let ratio = setf(c, 1, speed.div(max_speed));
        let two = li_f(c, 2, 0x4000_0000);
        if fcc(c, half.le(ratio)) {
            setf(c, 20, ratio.min(two));
        }

        // Layer playback rates.
        c.cycles += 5;
        c.g[A0][0] = lw(c, p(r, rider::ANIM));
        for (layer, ret_get, ret_set) in
            [(3u64, 0x0012_041c, 0x0012_042c), (4, 0x0012_043c, 0x0012_044c), (5, 0x0012_045c, 0x0012_0470)]
        {
            let last = layer == 5;
            if layer != 3 {
                c.cycles += 4;
            }
            c.g[A1][0] = layer;
            call_ptr(c, callee::ANIM_CONTROLLER_GET_LAYER_SEQUENCE, ret_get)?;
            let found = c.g[V0][0] != 0;
            if !last {
                c.g[A0][0] = lw(c, p(r, rider::ANIM));
            }
            if found {
                c.cycles += 4;
                if last {
                    c.g[A0][0] = lw(c, p(r, rider::ANIM));
                }
                c.g[A1][0] = layer;
                call_ptr(c, callee::ANIM_CONTROLLER_GET_LAYER_SEQUENCE, ret_set)?;
                let seq = r32(c, V0);
                stf(c, 20, p(seq, 0x90));
                if !last {
                    c.g[A0][0] = lw(c, p(r, rider::ANIM));
                }
            }
        }

        // Every sixth frame: the channels.
        c.cycles += 6;
        callee::AI_GET_FRAME_COUNTER.call(c, 0x0012_047c)?;
        c.g[V1][0] = 6;
        c.div(0, V0, V1);
        c.cycles += 4;
        c.g[V0][0] = lw(c, p(r, 0x86c));
        c.g[A0][0] = c.hi[0];
        if c.g[A0][0] == c.g[V0][0] {
            speed_anim_channels(c, r)?;
        }
        frame.restore(c, S0, 0x60);
        c.cycles += 8;
        frame.restore(c, S1, 0x50);
        frame.restore_ra(c, 0x40);
        frame.restore_fpr(c, 23, 0x88);
        frame.restore_fpr(c, 22, 0x80);
        frame.restore_fpr(c, 21, 0x78);
        frame.restore_fpr(c, 20, 0x70);
        frame.close(c);
        Ok(())
    })
}

/// The channel pass: s1 = "slow" (speed below `gp-0x7938`; never in
/// sub-state 11, which also pushes the velocity copy by 1000 along z).
fn speed_anim_channels(c: &mut Cpu, r: u32) -> Flow {
    c.cycles += 5;
    let slow_limit = ldf(c, 0, gp(c, -0x7938));
    let speed = fr(c, 21);
    let slow = fcc(c, speed.le(slow_limit));
    c.g[S1][0] = 1;
    if !slow {
        c.cycles += 1;
        c.g[S1][0] = 0;
    }
    c.cycles += 5;
    c.g[A0][0] = c.g[S0][0];
    callee::RIDER_GET_SUB_STATE.call(c, 0x0012_04bc)?;
    c.g[V1][0] = 0xb;
    if c.g[V0][0] == c.g[V1][0] {
        c.cycles += 12;
        li_f(c, 0, 0x447a_0000);
        c.g[S1][0] = 0;
        let sp = sp(c);
        c.sw(p(sp, 0x10), 0);
        stf(c, 0, p(sp, 0x18));
        c.sw(p(sp, 0x14), 0);
        c.sw(p(sp, 0x1c), 0);
        v_add(c, sp, p(sp, 0x10), sp);
    }
    c.g[V0][0] = lw(c, p(r, 0x940));
    for ch in &CHANNELS {
        speed_anim_channel(c, r, ch)?;
    }
    Ok(())
}

/// One channel (v0 = its enable flag): the wanted animation in a1, played
/// (`AnimController_PlayAnim(ctrl, layer, 0, -1)`) when the layer's
/// current one (a2) differs.
fn speed_anim_channel(c: &mut Cpu, r: u32, ch: &Channel) -> Flow {
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.cycles += 5;
        c.g[A0][0] = lw(c, p(r, rider::ANIM));
        c.g[A1][0] = ch.layer;
        call_ptr(c, callee::ANIM_CONTROLLER_GET_LAYER_ANIM_ID, ch.ret_anim_id)?;
        let slow = c.g[S1][0] != 0;
        c.g[A2][0] = c.g[V0][0];
        if slow {
            c.cycles += 2;
            c.g[A1][0] = ch.first_anim;
        } else {
            pick_speed_anim(c, r, ch);
        }
        c.cycles += 2;
        if c.g[A1][0] != c.g[A2][0] {
            c.cycles += 6;
            c.g[A0][0] = lw(c, p(r, rider::ANIM));
            li_f(c, 12, 0xbf80_0000);
            c.g[A2][0] = 0;
            call_ptr(c, callee::ANIM_CONTROLLER_PLAY_ANIM, ch.ret_play)?;
        }
    }
    if let Some(next) = ch.next_enable {
        c.g[V0][0] = lw(c, p(r, next));
    }
    Ok(())
}

/// The channel's animation (into a1) from the velocity (at sp) in the
/// frame `bone.rot ⊗ local rot`: sideways (|side| > `f22`) → +1/+2 by
/// sign, forward/back (|fwd| > `f23`) → +3/+4, otherwise +5, or +6 when
/// upside down.
fn pick_speed_anim(c: &mut Cpu, r: u32, ch: &Channel) {
    c.cycles += 3;
    c.g[V0][0] = lw(c, p(r, 0x320));
    let alt = c.g[V0][0] != 0;
    c.g[A0][0] = lw(c, p(r, rider::MODEL));
    let local = if alt {
        c.cycles += 18;
        ch.quat_alt
    } else {
        c.cycles += 17;
        ch.quat
    };
    let sp = sp(c);
    let m = r32(c, A0);
    c.g[V0][0] = lw(c, p(r, ch.bone));
    c.g[V1][0] = lw(c, p(m, model::BONES));
    let o = r32(c, V0) << 5;
    c.g[V0][0] = sx(o);
    let bone = r32(c, V1).wrapping_add(o);
    c.g[V1][0] = sx(bone);
    q_mul(c, p(bone, 0x10), p(r, local), p(sp, 0x30));
    c.lq(V0, p(sp, 0x30));

    // Matrix rows of the quaternion: x row (sp+0x20) and y row.
    c.cycles += 61;
    c.sq(V0, p(sp, 0x10));
    c.sq(V0, p(sp, 0x20));
    let qy = ldf(c, 3, p(sp, 0x14));
    let qz = ldf(c, 0, p(sp, 0x18));
    let qx = ldf(c, 4, p(sp, 0x10));
    let yy = setf(c, 11, qy * qy);
    let zz = setf(c, 1, qz * qz);
    let qw = ldf(c, 6, p(sp, 0x1c));
    let xx = setf(c, 7, qx * qx);
    let one = li_f(c, 9, 0x3f80_0000);
    let xz = setf(c, 2, qx * qz);
    c.sw(p(sp, 0x2c), 0);
    let wy = setf(c, 10, qw * qy);
    let wz = setf(c, 8, qw * qz);
    let xy = setf(c, 5, qx * qy);
    let t1 = setf(c, 1, yy + zz);
    let yz = setf(c, 0, qz * qy);
    let t7 = setf(c, 7, xx + yy);
    let wx = setf(c, 6, qw * qx);
    let t3 = setf(c, 3, xz - wy);
    let t5 = setf(c, 5, xy + wz);
    let t1 = setf(c, 1, t1 + t1);
    let t2 = setf(c, 2, xz + wy);
    let t0 = setf(c, 0, yz - wx);
    let t7 = setf(c, 7, t7 + t7);
    setf(c, 1, one - t1);
    setf(c, 3, t3 + t3);
    setf(c, 5, t5 + t5);
    setf(c, 2, t2 + t2);
    stf(c, 1, p(sp, 0x20));
    setf(c, 9, one - t7);
    stf(c, 3, p(sp, 0x28));
    setf(c, 0, t0 + t0);
    stf(c, 5, p(sp, 0x24));
    v_dot(c, sp, p(sp, 0x20));
    c.qmfc2(V0, 4);
    c.mtc1(V0, 3);
    let fwd = fr(c, 3);
    stf(c, 2, p(sp, 0x20));
    stf(c, 0, p(sp, 0x24));
    stf(c, 9, p(sp, 0x28));
    c.sw(p(sp, 0x2c), 0);
    v_dot(c, sp, p(sp, 0x20));
    c.qmfc2(V0, 4);
    c.mtc1(V0, 2);
    let side = fr(c, 2);
    let side_abs = setf(c, 1, side.abs());
    let side_limit = fr(c, 22);
    if fcc(c, side_limit.lt(side_abs)) {
        c.cycles += 5;
        let fwd_abs = setf(c, 0, fwd.abs());
        let mostly_side = fcc(c, fwd_abs.lt(side_abs));
        let zero = ldf(c, 0, p(sp, 0x2c));
        if mostly_side {
            c.cycles += 4;
            let positive = fcc(c, zero.lt(side));
            c.g[A1][0] = ch.first_anim + 2;
            if positive {
                c.cycles += 2;
                c.g[A1][0] = ch.first_anim + 1;
            }
            return;
        }
    }
    c.cycles += 5;
    let fwd_abs = setf(c, 0, fwd.abs());
    let fwd_limit = fr(c, 23);
    let moving = fcc(c, fwd_limit.lt(fwd_abs));
    let qx = ldf(c, 4, p(sp, 0x10));
    if moving {
        c.cycles += 5;
        c.f[0] = 0;
        let back = fcc(c, fwd.lt(FpuF32::ZERO));
        c.g[A1][0] = ch.first_anim + 3;
        if back {
            c.cycles += 2;
            c.g[A1][0] = ch.first_anim + 4;
        }
        return;
    }
    // Still: the z row decides whether the part is upside down.
    c.cycles += 37;
    c.g[A1][0] = ch.first_anim + 5;
    let qz = ldf(c, 5, p(sp, 0x18));
    let xx = setf(c, 8, qx * qx);
    let qy = ldf(c, 1, p(sp, 0x14));
    let qw = ldf(c, 3, p(sp, 0x1c));
    let zz = setf(c, 0, qz * qz);
    let yz = setf(c, 2, qy * qz);
    let one = li_f(c, 6, 0x3f80_0000);
    let wx = setf(c, 7, qw * qx);
    c.sw(p(sp, 0x2c), 0);
    let t0 = setf(c, 0, zz + xx);
    let xy = setf(c, 1, qy * qx);
    let zero = ldf(c, 8, p(sp, 0x2c));
    let wz = setf(c, 3, qw * qz);
    let t0 = setf(c, 0, t0 + t0);
    let t2 = setf(c, 2, yz + wx);
    let t1 = setf(c, 1, xy - wz);
    setf(c, 6, one - t0);
    setf(c, 2, t2 + t2);
    setf(c, 1, t1 + t1);
    stf(c, 6, p(sp, 0x24));
    stf(c, 2, p(sp, 0x28));
    stf(c, 1, p(sp, 0x20));
    v_dot(c, sp, p(sp, 0x20));
    c.qmfc2(V0, 4);
    c.mtc1(V0, 0);
    let up = fr(c, 0);
    if fcc(c, up.lt(zero)) {
        c.g[A1][0] = ch.first_anim + 6;
    }
}

// ---------------------------------------------------------------------------
// Rider_UpdateSlewedParams.

/// The slewed parameters: 19 `{value, rate, target}` triples from `+0x1f0`.
const SLEWED_FIRST: u32 = 0x1f0;
const SLEWED_STRIDE: u32 = 0xc;
const SLEWED_COUNT: u32 = 19;

/// `void Rider_UpdateSlewedParams(Rider* r)` (0x1211f8):
/// `Rider_UpdateSpeedAnims(r)`, then every slewed parameter moves towards
/// its target by at most its rate.
pub fn rider_update_slewed_params(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let r = r32(c, S0);
        callee::RIDER_UPDATE_SPEED_ANIMS.call(c, 0x0012_120c)?;
        for i in 0..SLEWED_COUNT {
            if i > 0 {
                c.cycles += 10;
                let prev = r32(c, V0);
                stf(c, 1, prev);
            }
            let param = p(r, SLEWED_FIRST + i * SLEWED_STRIDE);
            c.g[V0][0] = sx(param);
            slew(c, param);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        let last = r32(c, V0);
        stf(c, 1, last);
        frame.close(c);
        Ok(())
    })
}

/// `f1 = value moved towards target by rate` (f0 = target ± rate, f2 =
/// rate, f3 = value as the original leaves them).
fn slew(c: &mut Cpu, param: u32) {
    let target = ldf(c, 1, p(param, 8));
    let rate = ldf(c, 2, p(param, 4));
    let value = ldf(c, 3, param);
    let hi = setf(c, 0, target + rate);
    if fcc(c, hi.lt(value)) {
        c.cycles += 2;
        setf(c, 1, value - rate);
        return;
    }
    let lo = setf(c, 0, target - rate);
    c.cycles += 4;
    if fcc(c, value.lt(lo)) {
        setf(c, 1, value + rate);
    }
}

// ---------------------------------------------------------------------------
// RiderMetrix_Update.

/// `void RiderMetrix_Update(RiderMetrix* m)` (0x117c28): per-frame scoring
/// of the rider `+0x1ac`. With `dt = frame_dt * gp-0x7b00` and the speed:
/// the bank-combo timer `+0xa4` runs down (`RiderMetrix_BankCombo` when it
/// expires, unless in a main state or sub-state 1); the running timers
/// (`+0x30` air, `+0x40`, `+0x24` distance, `+0x2c`, `+0x6c`, `+0x78`)
/// advance while non-negative, add to the score `+0x14` and report score
/// milestones (`RiderMetrix_CheckScoreMilestone`, bonus into `+0x84`);
/// distance/peak/frame statistics; the trick HUD and HUD effects; then
/// the score goes to the audio system.
pub fn rider_metrix_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x80);
        let dt_scale = ldf(c, 0, gp(c, -0x7b00));
        frame.save(c, S0, 0x50);
        frame.save_fpr(c, 21, 0x68);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0x10);
        frame.save_fpr(c, 22, 0x70);
        c.f[21] = 0;
        let m = r32(c, S0);
        c.g[A0][0] = lw(c, p(m, 0x1ac));
        let r = r32(c, A0);
        let bank = ldf(c, 1, p(m, 0xa4));
        let speed = v_length(c, p(r, 0x1e0), V0, 22);
        let frame_dt = ldf(c, 2, p(r, 0x300));
        let zero = fr(c, 21);
        let banking = fcc(c, zero.le(bank));
        let dt = setf(c, 20, frame_dt * dt_scale);

        // The bank-combo timer. 0: timer path (reload air, clear f21),
        // 1: sub-state 1 (clear f21), 2: in a main state.
        let mut path = 0;
        if banking {
            c.cycles += 4;
            callee::RIDER_GET_MAIN_STATE.call(c, 0x0011_7ca4)?;
            if c.g[V0][0] != 0 {
                ldf(c, 0, p(m, 0x30));
                path = 2;
            } else {
                c.cycles += 5;
                c.g[A0][0] = lw(c, p(m, 0x1ac));
                callee::RIDER_GET_SUB_STATE.call(c, 0x0011_7cb4)?;
                c.g[V1][0] = 1;
                if c.g[V0][0] == c.g[V1][0] {
                    ldf(c, 0, p(m, 0x30));
                    path = 1;
                } else {
                    c.cycles += 6;
                    let left = ldf(c, 0, p(m, 0xa4));
                    let left = setf(c, 0, left - dt);
                    let zero = fr(c, 21);
                    let expired = fcc(c, left.le(zero));
                    stf(c, 0, p(m, 0xa4));
                    if expired {
                        c.cycles += 2;
                        c.g[A0][0] = c.g[S0][0];
                        callee::RIDER_METRIX_BANK_COMBO.call(c, 0x0011_7ce0)?;
                    }
                }
            }
        }
        if path == 0 {
            c.cycles += 1;
            ldf(c, 0, p(m, 0x30));
        }
        if path <= 1 {
            c.cycles += 1;
            c.f[21] = 0;
        }

        // Air time.
        c.cycles += 4;
        let air = fr(c, 0);
        let zero = fr(c, 21);
        c.g[A2][0] = 0x46_0000;
        if fcc(c, zero.le(air)) {
            c.cycles += 11;
            let air = setf(c, 0, air + dt);
            c.g[A2][0] = sx(0x0045_9e00);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx(p(m, 0x88));
            c.g[A3][0] = 0x1c;
            setf(c, 12, air);
            stf(c, 0, p(m, 0x30));
            add_milestone_score(c, m, 0x0011_7d18)?;
        }

        // Timer +0x40, scoring +0x3c per frame.
        c.cycles += 5;
        let t = ldf(c, 2, p(m, 0x40));
        let zero = fr(c, 21);
        c.g[A2][0] = 0x46_0000;
        if fcc(c, zero.le(t)) {
            c.cycles += 15;
            let score = ldf(c, 0, p(m, 0x14));
            let t = setf(c, 2, t + dt);
            let bonus = ldf(c, 1, p(m, 0x3c));
            c.g[A2][0] = sx(0x0045_9e20);
            c.g[A0][0] = c.g[S0][0];
            setf(c, 0, score + bonus);
            setf(c, 12, t);
            stf(c, 2, p(m, 0x40));
            c.g[A1][0] = sx(p(m, 0x8c));
            c.g[A3][0] = 0x20;
            stf(c, 0, p(m, 0x14));
            add_milestone_score(c, m, 0x0011_7d68)?;
        }

        // Distance +0x24 (speed * dt), and +0x1c while descending.
        c.cycles += 5;
        let t = ldf(c, 2, p(m, 0x24));
        let zero = fr(c, 21);
        if fcc(c, zero.le(t)) {
            c.cycles += 16;
            let step = setf(c, 3, speed * dt);
            let rate = ldf(c, 0, gp(c, -0x7afc));
            let score = ldf(c, 1, p(m, 0x14));
            c.g[V1][0] = lw(c, p(m, 0x1ac));
            let rv = r32(c, V1);
            let inc = setf(c, 0, step * rate);
            setf(c, 2, t + step);
            setf(c, 1, score + inc);
            stf(c, 2, p(m, 0x24));
            stf(c, 1, p(m, 0x14));
            let sp = sp(c);
            qcopy(c, V0, p(rv, 0x1c0), sp);
            let vz = ldf(c, 0, p(sp, 8));
            let zero = fr(c, 21);
            let down = fcc(c, vz.lt(zero));
            c.g[A2][0] = 0x46_0000;
            if down {
                c.cycles += 5;
                let k = ldf(c, 0, gp(c, -0x7af8));
                let d = ldf(c, 1, p(m, 0x1c));
                let k = setf(c, 0, step * k);
                setf(c, 1, d + k);
                stf(c, 1, p(m, 0x1c));
            }
            c.cycles += 10;
            ldf(c, 12, p(m, 0x24));
            c.g[A2][0] = sx(0x0045_9f68);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx(p(m, 0x98));
            c.g[A3][0] = 0x1d;
            add_milestone_score(c, m, 0x0011_7df4)?;
        }
        ldf(c, 2, p(m, 0x2c));

        // Timer +0x2c.
        c.cycles += 5;
        c.f[21] = 0;
        let t = fr(c, 2);
        let zero = fr(c, 21);
        c.g[A2][0] = 0x46_0000;
        if fcc(c, zero.le(t)) {
            timed_milestone(c, m, 0x2c, -0x7af4, 0x0045_9e40, 0x90, 0x1e, 0x0011_7e4c)?;
        }

        // Timer +0x6c (score only).
        c.cycles += 5;
        let t = ldf(c, 2, p(m, 0x6c));
        let zero = fr(c, 21);
        if fcc(c, zero.le(t)) {
            c.cycles += 8;
            let rate = ldf(c, 0, gp(c, -0x7af0));
            setf(c, 2, t + dt);
            let score = ldf(c, 1, p(m, 0x14));
            let inc = setf(c, 0, dt * rate);
            stf(c, 2, p(m, 0x6c));
            setf(c, 1, score + inc);
            stf(c, 1, p(m, 0x14));
        }
        ldf(c, 2, p(m, 0x78));

        // Timer +0x78.
        c.cycles += 4;
        let t = fr(c, 2);
        let zero = fr(c, 21);
        c.g[A2][0] = 0x46_0000;
        if fcc(c, zero.le(t)) {
            timed_milestone(c, m, 0x78, -0x7aec, 0x0045_9f08, 0x94, 0x1f, 0x0011_7ed0)?;
        }

        // Statistics: smoothed speed peak, distance, frames, top speed.
        c.cycles += 18;
        let avg = ldf(c, 3, p(m, 0x1a8));
        let k_keep = ldf(c, 0, gp(c, -0x7ae8));
        let k_new = ldf(c, 1, gp(c, -0x7ae4));
        let avg = setf(c, 3, avg * k_keep);
        let dist = ldf(c, 2, p(m, 0x164));
        let add = setf(c, 1, speed * k_new);
        let frames = ldf(c, 0, p(m, 0x168));
        let one = li_f(c, 4, 0x3f80_0000);
        setf(c, 2, dist + speed);
        let peak = ldf(c, 5, p(m, 0x160));
        let avg = setf(c, 3, avg + add);
        setf(c, 0, frames + one);
        stf(c, 2, p(m, 0x164));
        let new_peak = fcc(c, peak.lt(avg));
        stf(c, 3, p(m, 0x1a8));
        stf(c, 0, p(m, 0x168));
        if new_peak {
            c.cycles += 1;
            stf(c, 3, p(m, 0x160));
        }
        c.cycles += 5;
        let top = ldf(c, 0, p(m, 0x15c));
        if fcc(c, top.lt(speed)) {
            stf(c, 22, p(m, 0x15c));
        }

        // HUD.
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        c.g[S3][0] = sx(p(m, 0x1b8));
        callee::RIDER_METRIX_UPDATE_TRICK_HUD.call(c, 0x0011_7f48)?;
        c.g[V0][0] = lw(c, p(m, 0x1b0));
        let has_fx = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S3][0];
        if has_fx {
            c.cycles += 3;
            c.g[V0][0] = lw(c, p(m, 0x1b4));
            let none = (c.g[V0][0] as i64) <= 0;
            c.g[S1][0] = 0;
            if !none {
                c.cycles += 2;
                c.g[S2][0] = 0;
                loop {
                    c.cycles += 8;
                    let m = r32(c, S0);
                    c.g[A0][0] = lw(c, p(m, 0x1b0));
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                    let fx = r32(c, A0).wrapping_add(r32(c, S2));
                    c.g[A0][0] = sx(fx);
                    callee::CHUDFX_UPDATE.call(c, 0x0011_7f78)?;
                    c.g[V0][0] = lw(c, p(m, 0x1b4));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(0x9c));
                    if !more {
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S3][0];
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0011_7f68);
                    }
                }
            }
        }

        // memset(m + 0x1b8, 0, 8); m->hud_dirty = 1; audio trick score.
        c.cycles += 20;
        c.g[A1][0] = 0;
        c.g[A2][0] = 8;
        call_ptr(c, callee::MEMSET, 0x0011_7f98)?;
        c.g[V1][0] = 1;
        c.sw(p(m, 0x1c0), 1);
        call_ptr(c, callee::SSX_AUDIO_SYSTEM_GET_INSTANCE, 0x0011_7fa4)?;
        ldf(c, 12, p(m, 0x14));
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = lw(c, p(m, 0x1ac));
        call_ptr(c, callee::SSX_AUDIO_SYSTEM_SET_PLAYER_TRICK_SCORE, 0x0011_7fb4)?;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 22, 0x70);
        frame.restore_fpr(c, 21, 0x68);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// `RiderMetrix_CheckScoreMilestone(...)`, then `m->bonus (+0x84) += v0`.
fn add_milestone_score(c: &mut Cpu, m: u32, ret: u32) -> Flow {
    callee::RIDER_METRIX_CHECK_SCORE_MILESTONE.call(c, ret)?;
    c.g[V1][0] = lw(c, p(m, 0x84));
    let bonus = r32(c, V1).wrapping_add(r32(c, V0));
    c.g[V1][0] = sx(bonus);
    c.sw(p(m, 0x84), sx(bonus));
    Ok(())
}

/// A running timer (f2 = its value, non-negative; 16-cycle block):
/// `timer += dt`, `score += dt * rate`, then the milestone check on the
/// timer (`CheckScoreMilestone(m, m + slot, table, kind)`).
#[allow(clippy::too_many_arguments)]
fn timed_milestone(c: &mut Cpu, m: u32, timer: u32, rate_gp: i32, table: u32, slot: u32, kind: u64, ret: u32) -> Flow {
    c.cycles += 16;
    let dt = fr(c, 20);
    let t = fr(c, 2);
    let rate = ldf(c, 0, gp(c, rate_gp));
    let t = setf(c, 2, t + dt);
    let score = ldf(c, 1, p(m, 0x14));
    let inc = setf(c, 0, dt * rate);
    c.g[A2][0] = sx(table);
    setf(c, 12, t);
    stf(c, 2, p(m, timer));
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = sx(p(m, slot));
    setf(c, 1, score + inc);
    c.g[A3][0] = kind;
    stf(c, 1, p(m, 0x14));
    add_milestone_score(c, m, ret)
}

// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, vtable, LOG_RESULTS};
    use crate::difftest::{check, elf_image, put_vec, scramble, Rng, DATA, GAME_GP};

    const SEEDS: u32 = 250;

    const RIDER: u32 = DATA;
    const MODEL: u32 = DATA + 0x2000;
    const BONE_OFFSETS: u32 = DATA + 0x3000;
    const BONE_ROTATIONS: u32 = DATA + 0x3400;
    const BONES: u32 = DATA + 0x3800;
    const NBONES: u32 = 8;
    const STATE: u32 = DATA + 0x6000;

    fn f(x: f32) -> u32 {
        x.to_bits()
    }

    /// A float in [lo, hi).
    fn uf(r: &mut Rng, lo: f32, hi: f32) -> u32 {
        f(lo + (r.next_u32() as f32 / u32::MAX as f32) * (hi - lo))
    }

    fn rand_vec(c: &mut Cpu, r: &mut Rng, a: u32, range: f32) {
        let v = [r.f32(range), r.f32(range), r.f32(range), r.f32(range)];
        put_vec(c, a, v);
    }

    fn rand_quat(c: &mut Cpu, r: &mut Rng, a: u32) {
        rand_vec(c, r, a, 1.0);
    }

    /// The boot-time globals: pivot (zero for `zero_pivot`), axes, identity.
    fn put_globals(c: &mut Cpu, r: &mut Rng, zero_pivot: bool) {
        let pivot = if zero_pivot { [0; 4] } else { [r.f32(0.5), r.f32(0.5), r.f32(0.5), 0] };
        put_vec(c, PIVOT, pivot);
        put_vec(c, AXIS_X, [f(1.0), 0, 0, 0]);
        put_vec(c, AXIS_Z, [0, 0, f(1.0), 0]);
        put_vec(c, AXIS_Y, [0, f(1.0), 0, 0]);
        put_vec(c, IDENTITY_QUAT, [0, 0, 0, f(1.0)]);
    }

    /// The rider's model with its three bone tables (zero scale for
    /// `zero_scale`).
    fn put_model(c: &mut Cpu, r: &mut Rng, zero_scale: bool) {
        c.bus.write32(RIDER + 0x780, MODEL);
        c.bus.write32(MODEL + 0x24, BONE_OFFSETS);
        c.bus.write32(MODEL + 0x28, BONE_ROTATIONS);
        c.bus.write32(MODEL + 0x2c, BONES);
        for k in 0..4 {
            let s = if zero_scale { 0 } else { uf(r, 0.5, 1.5) };
            c.bus.write32(MODEL + 0x140 + 4 * k, s);
        }
        for i in 0..NBONES {
            rand_vec(c, r, BONE_OFFSETS + 0x10 * i, 2.0);
            rand_quat(c, r, BONE_ROTATIONS + 0x10 * i);
            rand_vec(c, r, BONES + 0x20 * i, 3.0);
            rand_quat(c, r, BONES + 0x20 * i + 0x10);
        }
    }

    /// The rider's state object (`+0x77c`): main state `+0xde0`, sub-state
    /// `+0xde4`.
    fn put_state(c: &mut Cpu, main: u32, sub: u32) {
        c.bus.write32(RIDER + 0x77c, STATE);
        c.bus.write32(STATE + 0xde0, main);
        c.bus.write32(STATE + 0xde4, sub);
    }

    /// The board-lean inputs.
    fn put_lean(c: &mut Cpu, r: &mut Rng, seed: u32) {
        c.bus.write32(RIDER + 0x8a0, r.next_u32() % NBONES);
        c.bus.write32(RIDER + 0x214, r.f32(0.3));
        let roll = if seed % 3 == 0 { r.f32(0.2) } else { r.f32(2.0) };
        c.bus.write32(RIDER + 0x208, roll);
        let pitch = if seed % 4 == 1 { r.f32(0.2) } else { r.f32(2.0) };
        c.bus.write32(RIDER + 0x1f0, pitch);
        let spin = if seed % 5 == 0 { 0 } else { r.f32(1.0) };
        c.bus.write32(RIDER + 0x250, spin);
        rand_vec(c, r, RIDER + 0x3b0, 2.0);
    }

    #[test]
    fn rider_update_bounds_matches() {
        check(ee::s00::f_0011e150__generated, rider_update_bounds, SEEDS, &|c, seed| {
            let mut r = Rng(0x0011_e150 ^ (seed as u64) << 12);
            scramble(c, &mut r);
            for off in [0x110, 0x1a0, 0x1b0, 0x1c0] {
                rand_vec(c, &mut r, RIDER + off, 3.0);
            }
            for k in 0..8 {
                c.bus.write32(RIDER + 0x400 + 4 * k, r.f32(600.0));
            }
            let centre = DATA + 0x5000;
            rand_vec(c, &mut r, centre, 100.0);
            c.g[A0][0] = sx(RIDER);
            c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(centre) };
        });
    }

    #[test]
    fn rider_update_board_lean_matches() {
        check(ee::s00::f_0011fa10__generated, rider_update_board_lean, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xfa10, 0);
            elf_image(c);
            put_globals(c, &mut r, seed % 4 == 0);
            put_model(c, &mut r, false);
            put_lean(c, &mut r, seed);
            let xf = DATA + 0x5000;
            rand_vec(c, &mut r, xf, 5.0);
            rand_quat(c, &mut r, xf + 0x10);
            c.g[A0][0] = sx(RIDER);
            c.g[A1][0] = sx(xf);
        });
    }

    #[test]
    fn rider_solve_leg_ik_matches() {
        check(ee::s00::f_0011f3d8__generated, rider_solve_leg_ik, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xf3d8, 0);
            elf_image(c);
            let degenerate = seed % 6 == 0;
            put_globals(c, &mut r, degenerate || seed % 4 == 0);
            put_model(c, &mut r, degenerate);
            let leg = DATA + 0x5000;
            for k in 0..3 {
                c.bus.write32(leg + 4 * k, r.next_u32() % NBONES);
            }
            let target = DATA + 0x5100;
            rand_vec(c, &mut r, target, 4.0);
            rand_quat(c, &mut r, target + 0x10);
            c.f[12] = if seed % 3 == 0 { f(1.0) } else { uf(&mut r, 0.0, 1.0) };
            c.g[A0][0] = sx(RIDER);
            c.g[A1][0] = sx(leg);
            c.g[A2][0] = sx(target);
        });
    }

    #[test]
    fn rider_update_skeleton_matches() {
        check(ee::s00::f_0011eb98__generated, rider_update_skeleton, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xeb98, 0);
            elf_image(c);
            put_globals(c, &mut r, seed % 4 == 0);
            put_model(c, &mut r, false);
            put_lean(c, &mut r, seed);
            let sub = [5, 0xd, 0xb, 0, 5, 1][(seed % 6) as usize];
            put_state(c, 0, sub);
            rand_vec(c, &mut r, RIDER + 0x110, 5.0);
            rand_quat(c, &mut r, RIDER + 0x120);
            rand_vec(c, &mut r, RIDER + 0x130, 5.0);
            rand_quat(c, &mut r, RIDER + 0x140);
            let spin = if seed % 7 == 0 { 0 } else { r.f32(2.0) };
            c.bus.write32(RIDER + 0x2c8, spin);
            c.bus.write32(RIDER + 0x150, (seed / 2) % 2);
            c.bus.write32(RIDER + 0x8a4, r.next_u32() % NBONES);
            let twist = if seed % 5 == 2 { 0 } else { r.f32(1.0) };
            c.bus.write32(RIDER + 0x2bc, twist);
            rand_vec(c, &mut r, RIDER + 0x390, 1.0);
            if seed % 9 == 4 {
                // Parallel to the bone's axis: no twist axis.
                put_vec(c, RIDER + 0x390, [0; 4]);
            }
            c.bus.write32(RIDER + 0x31c, r.f32(1.0));
            rand_vec(c, &mut r, RIDER + 0x370, 1.0);
            let ik = if seed % 8 == 3 { 0 } else { uf(&mut r, 0.0, 1.0) };
            c.bus.write32(RIDER + 0x318, ik);
            for desc in [0x8e0, 0x910] {
                for k in 0..3 {
                    c.bus.write32(RIDER + desc + 4 * k, r.next_u32() % NBONES);
                }
                rand_vec(c, &mut r, RIDER + desc + 0x10, 2.0);
                rand_quat(c, &mut r, RIDER + desc + 0x20);
            }
            c.g[A0][0] = sx(RIDER);
        });
    }

    /// The animation controller of `Rider_UpdateSpeedAnims` (layer
    /// sequences, current animation ids, a handler vtable whose `PlayAnim`
    /// hook is a fake that returns 0x207 so `PlayAnim` stops there), the
    /// frame counter and the channel inputs.
    fn put_speed_anims(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let ctrl = DATA + 0x7000;
        let layers = DATA + 0x7100;
        c.bus.write32(RIDER + 0x784, ctrl);
        c.bus.write32(ctrl + 0x50, layers);
        for layer in 3..6u32 {
            let seq = if r.next_u32() % 3 == 0 { 0 } else { DATA + 0x7200 + 0x100 * layer };
            c.bus.write32(layers + 8 * layer + 4, seq);
            let first = [0x19b, 0x1a2, 0x1a9][(layer - 3) as usize];
            c.bus.write32(ctrl + 4 * layer, first + r.next_u32() % 8);
        }
        let vt = vtable(c, r, &[0x20]);
        c.bus.write32(ctrl + 0x58, vt);
        for i in 0..8 {
            c.bus.write32(LOG_RESULTS + 4 * i, 0x207);
        }
        // AI_GetFrameCounter: [[[gp-0x848]+0x84]+0xc]+8.
        let (a, b, d) = (DATA + 0x7800, DATA + 0x7900, DATA + 0x7a00);
        c.bus.write32(GAME_GP - 0x848, a);
        c.bus.write32(a + 0x84, b);
        c.bus.write32(b + 0xc, d);
        let frame = r.next_u32() % 1000;
        c.bus.write32(d + 8, frame);
        let phase = if seed % 4 == 3 { (frame + 1) % 6 } else { frame % 6 };
        c.bus.write32(RIDER + 0x86c, phase);
        // Velocity relative to +0x3d0; speeds up to ~2500.
        let range = [100.0, 600.0, 1500.0][(seed % 3) as usize];
        rand_vec(c, r, RIDER + 0x1e0, range);
        rand_vec(c, r, RIDER + 0x3d0, 50.0);
        put_state(c, 0, if seed % 5 == 1 { 0xb } else { r.next_u32() % 16 });
        for en in [0x940, 0x970, 0x9a0] {
            c.bus.write32(RIDER + en, (r.next_u32() % 4 != 0) as u32);
        }
        c.bus.write32(RIDER + 0x320, r.next_u32() % 2);
        for bone in [0x8a8, 0x8ac, 0x89c] {
            c.bus.write32(RIDER + bone, r.next_u32() % NBONES);
        }
        for q in [0x950, 0x960, 0x980, 0x990, 0x9b0, 0x9c0] {
            rand_quat(c, r, RIDER + q);
        }
    }

    #[test]
    fn rider_update_speed_anims_matches() {
        check(ee::s00::f_00120378__generated, rider_update_speed_anims, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x0378, 3);
            elf_image(c);
            put_globals(c, &mut r, false);
            put_model(c, &mut r, false);
            put_speed_anims(c, &mut r, seed);
            c.g[A0][0] = sx(RIDER);
        });
    }

    #[test]
    fn rider_update_slewed_params_matches() {
        check(ee::s00::f_001211f8__generated, rider_update_slewed_params, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x11f8, 3);
            elf_image(c);
            put_globals(c, &mut r, false);
            put_model(c, &mut r, false);
            put_speed_anims(c, &mut r, seed);
            for i in 0..SLEWED_COUNT {
                let a = RIDER + SLEWED_FIRST + SLEWED_STRIDE * i;
                c.bus.write32(a, r.f32(10.0));
                c.bus.write32(a + 4, uf(&mut r, 0.0, 3.0));
                c.bus.write32(a + 8, r.f32(10.0));
            }
            c.g[A0][0] = sx(RIDER);
        });
    }

    #[test]
    fn rider_metrix_update_matches() {
        check(ee::s00::f_00117c28__generated, rider_metrix_update, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x7c28, 0);
            elf_image(c);
            let m = DATA + 0x8000;
            // The BELibrary singleton (gp-0x1f04) already exists: otherwise
            // `BELibrary_GetInstance` (reached from the trick HUD and
            // `RiderMetrix_BankCombo` → `RiderMetrix_AwardPoints`) builds it
            // and waits forever on its rewards file (`EAFile_QueueOpen` /
            // `EAFile_AllocRequest` with no request slots on a test machine).
            c.bus.write32(GAME_GP - 0x1f04, DATA + 0xa000);
            c.bus.write32(m + 0x1ac, RIDER);
            rand_vec(c, &mut r, RIDER + 0x1e0, 30.0);
            c.bus.write32(RIDER + 0x300, uf(&mut r, 0.5, 2.0));
            rand_vec(c, &mut r, RIDER + 0x1c0, 5.0);
            put_state(c, (seed % 3 == 1) as u32, if seed % 4 == 2 { 1 } else { 3 });
            for off in [0xa4, 0x30, 0x40, 0x24, 0x2c, 0x6c, 0x78] {
                let v = if r.next_u32() % 3 == 0 { r.f32(0.05) } else { r.f32(10.0) };
                c.bus.write32(m + off, v);
            }
            for off in [0x14, 0x3c, 0x1c, 0x1a8, 0x164, 0x168, 0x160, 0x15c] {
                c.bus.write32(m + off, r.f32(50.0));
            }
            c.bus.write32(m + 0x84, r.next_u32() % 1000);
            let n = [0, 1, 3, 0xffff_fffe][(seed % 4) as usize];
            c.bus.write32(m + 0x1b0, if seed % 5 == 0 { 0 } else { DATA + 0x9000 });
            c.bus.write32(m + 0x1b4, n);
            c.g[A0][0] = sx(m);
        });
    }
}
