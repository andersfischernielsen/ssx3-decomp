//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements, each with a differential test against the generated body.
//!
//! - **Handle tests** (`PS2VisualEffectsMan_IsHandleNull`,
//!   `SSXAudioSystem_IsSoundMode2`): one load and a compare.
//! - **Camera trigger / bound matrices** (`CameraSphereTrigger_ContainsPoint`,
//!   `CameraBoundObj_GetWorldPoint`): both build a 4x4 matrix with the VU0
//!   helpers inlined (`vu0IdentityMtx`, `vu0ScaleMtx`, `vu0RotAxis`,
//!   `vu0TransMtx`, `vu0MulVec`), then transform a point with it. The VU0
//!   macro operations are issued in the original order and with the original
//!   registers (the register file outlives the function), and the work areas
//!   are written at the same stack offsets, so memory matches too.
//! - **`FlexBridge_SpringsStep`**: one implicit Euler step of the bridge's
//!   hanging spring chain (forces, torques, integration, then clearing the
//!   per-node accumulators). The frame arrays `F[16]`/`T[16]` live at the
//!   original's stack offsets and the four pass loops keep its event checks.

use crate::abi;
use crate::call::{guest, Callee, Flow};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002f_7be0, ps2_visual_effects_man_is_handle_null, "PS2VisualEffectsMan_IsHandleNull"),
    (0x0028_99e8, ssx_audio_system_is_sound_mode2, "SSXAudioSystem_IsSoundMode2"),
    (0x0017_2278, camera_sphere_trigger_contains_point, "CameraSphereTrigger_ContainsPoint"),
    (0x0017_0d20, camera_bound_obj_get_world_point, "CameraBoundObj_GetWorldPoint"),
    (0x0035_3fc0, flex_bridge_springs_step, "FlexBridge_SpringsStep"),
];

mod callee {
    use super::{ee, Callee};

    pub const MATH_SIN_COS: Callee = Callee { addr: 0x0031_be50, f: ee::s10::Math_SinCos };
}

// ---------------------------------------------------------------------------
// Handle tests.

/// `bool PS2VisualEffectsMan_IsHandleNull(Handle* h)` (0x2f7be0): true when the
/// handle's first word is 0.
pub fn ps2_visual_effects_man_is_handle_null(c: &mut Cpu) {
    c.cycles += 3;
    let h = abi::arg(c, 0);
    c.g[V0][0] = c.lw(h);
    c.g[V0][0] = (c.g[V0][0] < 1u64) as u64;
    abi::ret(c);
}

/// `bool SSXAudioSystem_IsSoundMode2(SSXAudioSystem* self)` (0x2899e8): true
/// when the sound-mode word at `+0x608c` is 2.
pub fn ssx_audio_system_is_sound_mode2(c: &mut Cpu) {
    c.cycles += 4;
    let this = abi::arg(c, 0);
    c.g[V0][0] = c.lw(this.wrapping_add(0x608c));
    c.g[V0][0] = c.g[V0][0] ^ 0x2;
    c.g[V0][0] = (c.g[V0][0] < 1u64) as u64;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// VU0 matrix helpers, inlined in the two camera functions below. Every one of
// them issues the original's COP2 macro operations (same destination and
// source registers, same order), because the VU0 register file is still
// observable when the function returns.

/// `vu0IdentityMtx` / `sqc2` ×4: the quadword at `src` copied to `dst`.
fn copy_matrix(c: &mut Cpu, dst: u32, src: u32) {
    c.lqc2(1, src);
    c.lqc2(2, src.wrapping_add(0x10));
    c.lqc2(3, src.wrapping_add(0x20));
    c.lqc2(4, src.wrapping_add(0x30));
    c.sqc2(1, dst);
    c.sqc2(2, dst.wrapping_add(0x10));
    c.sqc2(3, dst.wrapping_add(0x20));
    c.sqc2(4, dst.wrapping_add(0x30));
}

/// `vu0MulMtx` (`dst = m * n`, four rows at `dst + 0,0x10,0x20,0x30`). `dst`
/// may alias `m` or `n`: every row is loaded first.
fn mul_matrix(c: &mut Cpu, dst: u32, m: u32, n: u32) {
    c.lqc2(4, m);
    c.lqc2(5, m.wrapping_add(0x10));
    c.lqc2(6, m.wrapping_add(0x20));
    c.lqc2(7, m.wrapping_add(0x30));
    c.lqc2(8, n);
    c.lqc2(9, n.wrapping_add(0x10));
    c.lqc2(10, n.wrapping_add(0x20));
    c.lqc2(11, n.wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.vu0().mula(0b1111, 4, Src::Bc(9, X));
    c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
    c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
    c.vu0().mula(0b1111, 4, Src::Bc(10, X));
    c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
    c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
    c.vu0().mula(0b1111, 4, Src::Bc(11, X));
    c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
    c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
    c.sqc2(12, dst);
    c.sqc2(13, dst.wrapping_add(0x10));
    c.sqc2(14, dst.wrapping_add(0x20));
    c.sqc2(15, dst.wrapping_add(0x30));
}

/// `vu0MulVec` (`dst = m * v`, one lane per row): the four dot products land
/// in vf12's four lanes.
fn mul_matrix_vec(c: &mut Cpu, dst: u32, m: u32, v: u32) {
    c.lqc2(4, m);
    c.lqc2(5, m.wrapping_add(0x10));
    c.lqc2(6, m.wrapping_add(0x20));
    c.lqc2(7, m.wrapping_add(0x30));
    c.lqc2(8, v);
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, dst);
}

/// `vu0RotAxis(&m, axis, angle)`: `Math_SinCos(angle, &sin, &cos)` (the two
/// result words at `out_a` and `out_b`, in that argument order; `negate` is
/// set where the original passes the negated angle), then the Rodrigues
/// rotation matrix in the four rows at `work` (with the `1.0` in f20),
/// multiplied into `m` at `sp`.
///
/// `work` is `sp + 0x40` or `sp + 0x50` depending on the call site, exactly as
/// the original lays the temporary matrix out.
#[allow(clippy::too_many_arguments)]
fn rot_axis(
    c: &mut Cpu,
    negate: bool,
    work: u32,
    model: u32,
    angle_field: u32,
    axis: u32,
    out_a: u32,
    out_b: u32,
    ret: u32,
) -> Flow {
    let sp = c.g[SP][0] as u32;
    c.lwc1(12, model.wrapping_add(angle_field));
    c.g[S4][0] = sx(sp);
    c.g[A0][0] = sx(out_a);
    c.g[A1][0] = sx(out_b);
    if negate {
        c.neg_s(12, 12);
    }
    c.g[S0][0] = 0x500000;
    c.g[S3][0] = sx(work);
    c.g[S1][0] = sx(axis);
    callee::MATH_SIN_COS.call(c, ret)?;

    // Rodrigues from (cos, sin) and the axis `(x, y, z)`.
    c.lwc1(11, out_b);
    c.lwc1(3, axis.wrapping_add(0x8));
    c.sub_s(1, 20, 11);
    c.lwc1(0, axis);
    c.lwc1(6, axis.wrapping_add(0x4));
    c.lwc1(8, out_a);
    c.mul_s(2, 1, 3);
    c.sw(work.wrapping_add(0x30), 0);
    c.mul_s(4, 1, 0);
    c.sw(work.wrapping_add(0x34), 0);
    c.mul_s(1, 1, 6);
    c.sw(work.wrapping_add(0x38), 0);
    c.mul_s(13, 8, 3);
    c.sw(work.wrapping_add(0xc), 0);
    c.mul_s(12, 8, 0);
    c.sw(work.wrapping_add(0x1c), 0);
    c.mul_s(9, 2, 3);
    c.sw(work.wrapping_add(0x2c), 0);
    c.mul_s(10, 4, 3);
    c.swc1(20, work.wrapping_add(0x3c));
    c.mul_s(5, 2, 0);
    c.mul_s(7, 4, 0);
    c.mul_s(3, 1, 3);
    c.mul_s(0, 1, 0);
    c.mul_s(8, 8, 6);
    c.mul_s(2, 2, 6);
    c.mul_s(4, 4, 6);
    c.mul_s(1, 1, 6);
    c.add_s(5, 5, 8);
    c.sub_s(0, 0, 13);
    c.sub_s(2, 2, 12);
    c.add_s(9, 9, 11);
    c.swc1(5, work.wrapping_add(0x8));
    c.add_s(7, 7, 11);
    c.swc1(0, work.wrapping_add(0x4));
    c.add_s(4, 4, 13);
    c.swc1(2, work.wrapping_add(0x18));
    c.sub_s(10, 10, 8);
    c.swc1(9, work.wrapping_add(0x28));
    c.add_s(1, 1, 11);
    c.swc1(7, work);
    c.add_s(3, 3, 12);
    c.swc1(4, work.wrapping_add(0x10));
    c.swc1(10, work.wrapping_add(0x20));
    c.swc1(1, work.wrapping_add(0x14));
    c.swc1(3, work.wrapping_add(0x24));
    mul_matrix(c, sp, sp, work);
    Ok(())
}

// ---------------------------------------------------------------------------
// Camera sphere trigger.

/// `bool CameraSphereTrigger_ContainsPoint(CameraSphereTrigger* self, Vec4*
/// p)` (0x172278): un-scales and un-rotates `p - self->pos` by the trigger's
/// radii and angles and returns whether the result lies in the unit sphere
/// (`x^2 + y^2 + z^2 <= 1`).
pub fn camera_sphere_trigger_contains_point(c: &mut Cpu) {
    guest(c, camera_sphere_trigger_contains_point_body)
}

fn camera_sphere_trigger_contains_point_body(c: &mut Cpu) -> Flow {
    c.cycles += 357;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x170));
    let sp = c.g[SP][0] as u32;
    let trigger = abi::arg(c, 0);
    let point = abi::arg(c, 1);

    c.g[V0][0] = 0x500000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe60));
    let identity = c.g[V0][0] as u32;
    c.sq(18, sp.wrapping_add(0x130));
    c.sq(21, sp.wrapping_add(0x100));
    c.sq(16, sp.wrapping_add(0x150));
    c.sq(17, sp.wrapping_add(0x140));
    c.sq(19, sp.wrapping_add(0x120));
    c.sq(20, sp.wrapping_add(0x110));
    c.sd(sp.wrapping_add(0xf0), c.g[RA][0]);
    c.swc1(20, sp.wrapping_add(0x160));

    // m = identity.
    c.lqc2(1, identity);
    c.lqc2(2, identity.wrapping_add(0x10));
    c.lqc2(3, identity.wrapping_add(0x20));
    c.lqc2(4, identity.wrapping_add(0x30));
    c.sqc2(1, sp);
    c.sqc2(2, sp.wrapping_add(0x10));
    c.sqc2(3, sp.wrapping_add(0x20));
    c.sqc2(4, sp.wrapping_add(0x30));

    // m = m * scale(1/rx, 1/ry, 1/rz, 1), the scale quad written at sp+0x80.
    let work = sp.wrapping_add(0x90);
    c.lwc1(2, trigger.wrapping_add(0x18));
    c.lwc1(0, trigger.wrapping_add(0x1c));
    c.lwc1(1, trigger.wrapping_add(0x20));
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 20);
    c.div_s(2, 20, 2);
    c.swc1(20, sp.wrapping_add(0x8c));
    c.div_s(0, 20, 0);
    c.div_s(1, 20, 1);
    c.swc1(2, sp.wrapping_add(0x80));
    c.swc1(0, sp.wrapping_add(0x84));
    c.swc1(1, sp.wrapping_add(0x88));
    c.lqc2(1, sp.wrapping_add(0x80));
    c.lqc2(4, sp);
    c.lqc2(5, sp.wrapping_add(0x10));
    c.lqc2(6, sp.wrapping_add(0x20));
    c.lqc2(7, sp.wrapping_add(0x30));
    c.vu0().mul(0b1111, 8, 4, Src::Bc(1, X));
    c.vu0().mul(0b1111, 9, 5, Src::Bc(1, Y));
    c.vu0().mul(0b1111, 10, 6, Src::Bc(1, Z));
    c.vu0().mul(0b1111, 11, 7, Src::Bc(1, W));
    c.sqc2(8, work);
    c.sqc2(9, work.wrapping_add(0x10));
    c.sqc2(10, work.wrapping_add(0x20));
    c.sqc2(11, work.wrapping_add(0x30));
    copy_matrix(c, sp.wrapping_add(0x40), work);
    copy_matrix(c, sp, sp.wrapping_add(0x40));

    // Three axis rotations, angles negated on the way in.
    rot_axis(c, true, sp.wrapping_add(0x40), trigger, 0xc, 0x004f_f160, sp.wrapping_add(0xd0), sp.wrapping_add(0xd4), 0x0017_2398)?;
    rot_axis(c, true, sp.wrapping_add(0x40), trigger, 0x14, 0x004f_f150, sp.wrapping_add(0xd8), sp.wrapping_add(0xdc), 0x0017_24e0)?;
    rot_axis(c, true, sp.wrapping_add(0x40), trigger, 0x10, 0x004f_f140, sp.wrapping_add(0xe0), sp.wrapping_add(0xe4), 0x0017_2628)?;

    // m = m * translate(-self->pos).
    c.lwc1(2, trigger.wrapping_add(0x8));
    c.lwc1(0, trigger);
    c.lwc1(1, trigger.wrapping_add(0x4));
    c.neg_s(2, 2);
    c.neg_s(0, 0);
    c.swc1(20, sp.wrapping_add(0x4c));
    c.neg_s(1, 1);
    c.swc1(2, sp.wrapping_add(0x48));
    c.swc1(0, sp.wrapping_add(0x40));
    c.swc1(1, sp.wrapping_add(0x44));
    c.swc1(0, sp.wrapping_add(0x50));
    c.swc1(1, sp.wrapping_add(0x54));
    c.swc1(2, sp.wrapping_add(0x58));
    mul_matrix_vec(c, sp.wrapping_add(0x30), sp, sp.wrapping_add(0x40));

    // q = m * *p, then |q.xyz|^2 <= 1.
    mul_matrix_vec(c, sp.wrapping_add(0x50), sp, point);
    c.lq(2, sp.wrapping_add(0x50));
    c.sq(2, sp.wrapping_add(0x40));
    c.lwc1(0, sp.wrapping_add(0x40));
    c.lwc1(2, sp.wrapping_add(0x44));
    c.mul_s(0, 0, 0);
    c.lwc1(1, sp.wrapping_add(0x48));
    c.mul_s(2, 2, 2);
    c.mul_s(1, 1, 1);
    c.add_s(0, 0, 2);
    c.add_s(0, 0, 1);
    c.c_le_s(0, 20);
    let inside = c.fcc();
    c.g[V1][0] = 1;
    if !inside {
        c.cycles += 1;
        c.g[V1][0] = 0;
    }

    c.cycles += 11;
    c.lq(16, sp.wrapping_add(0x150));
    c.g[V0][0] = c.g[V1][0];
    c.lq(17, sp.wrapping_add(0x140));
    c.lq(18, sp.wrapping_add(0x130));
    c.lq(19, sp.wrapping_add(0x120));
    c.lq(20, sp.wrapping_add(0x110));
    c.lq(21, sp.wrapping_add(0x100));
    c.g[RA][0] = c.ld(sp.wrapping_add(0xf0));
    c.lwc1(20, sp.wrapping_add(0x160));
    c.g[SP][0] = sx(sp.wrapping_add(0x170));
    abi::ret(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// Camera bound object.

/// `void CameraBoundObj_GetWorldPoint(Vec4* out, void* src, int corner)`
/// (0x170d20): writes the world-space position of one of the object's eight
/// corners (`src + 0x28 + corner*12`) through the matrix built from `src`'s
/// translation, the three angles at `+0x10`, `+0x14`, `+0xc` and the scale at
/// `+0x18`.
pub fn camera_bound_obj_get_world_point(c: &mut Cpu) {
    guest(c, camera_bound_obj_get_world_point_body)
}

fn camera_bound_obj_get_world_point_body(c: &mut Cpu) -> Flow {
    c.cycles += 355;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x160));
    let sp = c.g[SP][0] as u32;
    let out = abi::arg(c, 0);
    let src = abi::arg(c, 1);
    let corner_index = abi::arg(c, 2);

    c.g[V0][0] = 0x500000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe60));
    let identity = c.g[V0][0] as u32;
    c.sq(16, sp.wrapping_add(0x140));
    c.sq(18, sp.wrapping_add(0x120));
    c.g[S2][0] = sx(corner_index);
    c.sq(20, sp.wrapping_add(0x100));
    c.sq(17, sp.wrapping_add(0x130));
    c.sq(19, sp.wrapping_add(0x110));
    c.sd(sp.wrapping_add(0xf0), c.g[RA][0]);
    c.swc1(20, sp.wrapping_add(0x150));

    // m = identity, then m = m * translate(src->pos).
    c.lqc2(1, identity);
    c.lqc2(2, identity.wrapping_add(0x10));
    c.lqc2(3, identity.wrapping_add(0x20));
    c.lqc2(4, identity.wrapping_add(0x30));
    c.sqc2(1, sp);
    c.sqc2(2, sp.wrapping_add(0x10));
    c.sqc2(3, sp.wrapping_add(0x20));
    c.sqc2(4, sp.wrapping_add(0x30));
    c.lwc1(0, src);
    c.lwc1(1, src.wrapping_add(0x4));
    c.lwc1(2, src.wrapping_add(0x8));
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 20);
    c.swc1(0, sp.wrapping_add(0x40));
    c.swc1(1, sp.wrapping_add(0x44));
    c.swc1(2, sp.wrapping_add(0x48));
    c.swc1(20, sp.wrapping_add(0x4c));
    mul_matrix_vec(c, sp.wrapping_add(0x30), sp, sp.wrapping_add(0x40));

    // Three axis rotations: x, then y, then z (angles not negated here).
    rot_axis(c, false, sp.wrapping_add(0x50), src, 0x10, 0x004f_f140, sp.wrapping_add(0xd0), sp.wrapping_add(0xd4), 0x0017_0dd4)?;
    rot_axis(c, false, sp.wrapping_add(0x40), src, 0x14, 0x004f_f150, sp.wrapping_add(0xd8), sp.wrapping_add(0xdc), 0x0017_0f1c)?;
    rot_axis(c, false, sp.wrapping_add(0x40), src, 0xc, 0x004f_f160, sp.wrapping_add(0xe0), sp.wrapping_add(0xe4), 0x0017_1064)?;

    // m = m * scale(src->scale).
    let work = sp.wrapping_add(0x90);
    c.lwc1(2, src.wrapping_add(0x18));
    c.lwc1(1, src.wrapping_add(0x1c));
    c.lwc1(0, src.wrapping_add(0x20));
    c.swc1(2, sp.wrapping_add(0x80));
    c.swc1(1, sp.wrapping_add(0x84));
    c.swc1(0, sp.wrapping_add(0x88));
    c.swc1(20, sp.wrapping_add(0x8c));
    c.lqc2(1, sp.wrapping_add(0x80));
    c.lqc2(4, sp);
    c.lqc2(5, sp.wrapping_add(0x10));
    c.lqc2(6, sp.wrapping_add(0x20));
    c.lqc2(7, sp.wrapping_add(0x30));
    c.vu0().mul(0b1111, 8, 4, Src::Bc(1, X));
    c.vu0().mul(0b1111, 9, 5, Src::Bc(1, Y));
    c.vu0().mul(0b1111, 10, 6, Src::Bc(1, Z));
    c.vu0().mul(0b1111, 11, 7, Src::Bc(1, W));
    c.sqc2(8, work);
    c.sqc2(9, work.wrapping_add(0x10));
    c.sqc2(10, work.wrapping_add(0x20));
    c.sqc2(11, work.wrapping_add(0x30));
    copy_matrix(c, sp.wrapping_add(0x40), work);
    copy_matrix(c, sp, sp.wrapping_add(0x40));

    // out = m * (src + 0x28 + corner*12), a 128-bit store.
    c.g[V1][0] = 0xc;
    c.g[V0][0] = sx(out);
    c.mult(0, 4, 18, 3);
    c.lq(17, sp.wrapping_add(0x130));
    c.lq(19, sp.wrapping_add(0x110));
    c.g[RA][0] = c.ld(sp.wrapping_add(0xf0));
    c.g[S2][0] = sx((c.g[A0][0] as u32).wrapping_add(src));
    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x28));
    c.lq(16, sp.wrapping_add(0x140));
    let corner = c.g[S2][0] as u32;
    c.lwc1(0, corner);
    c.swc1(0, sp.wrapping_add(0x40));
    c.lwc1(1, corner.wrapping_add(0x4));
    c.swc1(1, sp.wrapping_add(0x44));
    c.lwc1(0, corner.wrapping_add(0x8));
    c.swc1(20, sp.wrapping_add(0x4c));
    c.swc1(0, sp.wrapping_add(0x48));
    mul_matrix_vec(c, sp.wrapping_add(0x50), sp, sp.wrapping_add(0x40));
    c.lq(18, sp.wrapping_add(0x120));
    c.lq(3, sp.wrapping_add(0x50));
    c.lwc1(20, sp.wrapping_add(0x150));
    c.sq(3, out);
    c.lq(20, sp.wrapping_add(0x100));
    c.g[SP][0] = sx(sp.wrapping_add(0x160));
    abi::ret(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// FlexBridge spring chain.

/// `a + off`.
#[inline(always)]
fn at(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// The low 32 bits of a GPR.
#[inline(always)]
fn r(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

/// Set a GPR to a sign-extended word.
#[inline(always)]
fn set_r(c: &mut Cpu, reg: usize, v: u32) {
    c.g[reg][0] = ssx3_ee_rt::cpu::sx(v);
}

/// `void FlexBridge_SpringsStep(FlexBridge* self)` (0x353fc0): one step of the
/// spring chain hanging off the bridge -- an implicit Euler integration of a
/// chain of point masses.
///
/// Layout: `self->n` at `+0`, `rest` +0x4, `scale` +0x8, `c` +0xc, `damp`
/// +0x1c, `k` +0x1c? (see the field constants below), and `self->nodes` at
/// `+0x24`, an array of `0x50`-byte nodes. The stack holds `F[16]` (quads at
/// `sp+0x10`) and `T[16]` (floats at `sp+0x110`).
pub fn flex_bridge_springs_step(c: &mut Cpu) {
    c.cycles += 18;
    // g = (0, 0, -980, 0) as the first float of the frame scratch.
    c.g[AT][0] = 0xffffffff_c475_0000;
    c.mtc1(1, 0);
    let frame_base = r(c, SP).wrapping_sub(0x1c0);
    c.g[SP][0] = sx(frame_base);
    let sp = r(c, SP);
    c.sw(sp, 0);
    c.g[A3][0] = c.g[A0][0];
    c.swc1(0, at(sp, 0x8));
    c.g[V1][0] = 0xf;
    c.sw(at(sp, 0x4), 0);
    c.g[T0][0] = ssx3_ee_rt::cpu::sx(at(sp, 0x10));
    c.sw(at(sp, 0xc), 0);
    c.g[T2][0] = ssx3_ee_rt::cpu::sx(at(sp, 0x110));
    // dt = 1 / (float)frame_rate->denominator
    let clock = c.lw(at(r(c, GP), 0x2a74));
    c.g[V0][0] = clock;
    c.g[A0][0] = 0xffff_ffff_ffff_ffff;
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 1);
    c.lwc1(0, at(r(c, V0), 0x10));
    c.cvt_s_w(0, 0);
    c.div_s(6, 1, 0);

    // A 16-step synchronisation delay (`for (v1 = 15; v1 != -1; v1--)`).
    loop {
        c.cycles += 7;
        let v = r(c, V1).wrapping_sub(1);
        set_r(c, V1, v);
        let go = c.g[V1][0] != c.g[A0][0];
        c.g[V0][0] = 0x500000;
        if !go {
            break;
        }
        if c.cycles >= c.next_event {
            c.pc = 0x0035_4008;
            c.event();
        }
    }

    // F[0] = F[n-1] = the zero vector; T[0] = T[n-1] = 0.
    c.cycles += 18;
    let n = c.lw(r(c, A3));
    set_r(c, V1, n as u32);
    let zero_vec = r(c, V0).wrapping_sub(0xee0);
    set_r(c, V0, zero_vec);
    c.mtc1(0, 0);
    c.lq(6, zero_vec);
    let v = r(c, V1) << 4;
    c.g[A1][0] = sx(v);
    let last_slot = at(sp, r(c, A1));
    set_r(c, V0, last_slot);
    let v = r(c, V1).wrapping_sub(1);
    c.g[V1][0] = sx(v);
    c.sq(6, last_slot);
    let v = r(c, A1).wrapping_add(r(c, T0));
    c.g[A1][0] = sx(v);
    let v = r(c, V1) << 2;
    c.g[A0][0] = sx(v);
    c.g[T1][0] = 0x1;
    let prev_slot = r(c, A1).wrapping_sub(0x10);
    c.lq(2, prev_slot);
    let v = r(c, T2).wrapping_add(r(c, A0));
    c.g[A0][0] = sx(v);
    let v = ((r(c, T1) as i64) < (r(c, V1) as i64)) as u32;
    c.g[V1][0] = sx(v);
    c.sq(2, at(sp, 0x10));
    c.swc1(0, r(c, A0));
    let no_interior = c.g[V1][0] == 0;
    c.swc1(0, at(sp, 0x110));

    if !no_interior {
        // First pass: spring forces and torques for the interior nodes.
        c.cycles += 7;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 8);
        c.g[T0][0] = ssx3_ee_rt::cpu::sx(at(r(c, T2), 0x4));
        c.g[AT][0] = 0x4000_0000;
        c.mtc1(1, 7);
        c.g[A2][0] = ssx3_ee_rt::cpu::sx(at(sp, 0x10));
        c.g[A1][0] = 0x50;
        loop {
            c.cycles += 167;
            // d1 = nodes[i-1].p - nodes[i].p, d2 = nodes[i+1].p - nodes[i].p.
            let nodes = c.lw(at(r(c, A3), 0x24));
            set_r(c, V1, nodes as u32);
            let v = r(c, T1).wrapping_add(1);
            c.g[T1][0] = sx(v);
            let v = r(c, A1).wrapping_add(r(c, V1));
            c.g[A0][0] = sx(v);
            let v = r(c, A0);
            c.g[V1][0] = sx(v);
            let node = r(c, A0);
            let node_minus = r(c, V1).wrapping_sub(0x50);
            c.lqc2(3, node_minus);
            c.lqc2(4, node);
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, at(sp, 0x160));
            let nodes = c.lw(at(r(c, A3), 0x24));
            set_r(c, V1, nodes as u32);
            c.lq(2, at(sp, 0x160));
            let v = r(c, A1).wrapping_add(r(c, V1));
            c.g[A0][0] = sx(v);
            c.sq(2, at(sp, 0x150));
            let v = r(c, A0);
            c.g[V1][0] = sx(v);
            let node = r(c, A0);
            c.lqc2(3, at(r(c, V1), 0x50));
            c.lqc2(4, node);
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, at(sp, 0x170));
            c.lwc1(0, at(r(c, A3), 0x20));
            c.lwc1(1, at(r(c, A3), 0x1c));
            c.lq(2, at(sp, 0x170));
            c.mul_s(1, 1, 0);
            c.sq(2, at(sp, 0x160));
            // d1 -= d1 * (rest*scale/|d1|)
            c.lqc2(3, at(sp, 0x150));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(3, 22);
            c.mtc1(3, 0);
            c.div_s(1, 1, 0);
            c.sub_s(1, 8, 1);
            c.mfc1(2, 1);
            c.lqc2(4, at(sp, 0x150));
            c.qmtc2(2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, at(sp, 0x150));
            // d2 -= d2 * (rest*scale/|d2|)
            c.lwc1(1, at(r(c, A3), 0x20));
            c.lqc2(3, at(sp, 0x160));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(2, 22);
            c.mtc1(2, 2);
            c.lwc1(0, at(r(c, A3), 0x1c));
            c.mul_s(0, 0, 1);
            c.div_s(0, 0, 2);
            c.sub_s(0, 8, 0);
            c.mfc1(2, 0);
            c.lqc2(4, at(sp, 0x160));
            c.qmtc2(2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, at(sp, 0x160));
            // F[i] = k * (d1 + d2) + (dt*k) * (v[i-1] + v[i+1] - 2*v[i])
            c.lwc1(1, at(r(c, A3), 0x4));
            c.lqc2(3, at(sp, 0x150));
            c.lqc2(4, at(sp, 0x160));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, at(sp, 0x190));
            c.lq(2, at(sp, 0x190));
            c.sq(2, at(sp, 0x180));
            c.mfc1(3, 1);
            c.lqc2(4, at(sp, 0x180));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, at(sp, 0x190));
            c.lq(2, at(sp, 0x190));
            let nodes = c.lw(at(r(c, A3), 0x24));
            set_r(c, V1, nodes as u32);
            c.sq(2, at(sp, 0x170));
            c.sq(2, at(r(c, A2), 0x10));
            let v = r(c, A1).wrapping_add(r(c, V1));
            c.g[A0][0] = sx(v);
            let v = r(c, A0);
            c.g[V1][0] = sx(v);
            c.lwc1(0, at(r(c, A3), 0x4));
            let node = r(c, A0);
            c.lqc2(3, r(c, V1).wrapping_sub(0x40));
            c.lqc2(4, at(node, 0x60));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, at(sp, 0x1a0));
            let nodes = c.lw(at(r(c, A3), 0x24));
            set_r(c, V1, nodes as u32);
            c.mul_s(0, 6, 0);
            c.lq(2, at(sp, 0x1a0));
            let v = r(c, A1).wrapping_add(r(c, V1));
            c.g[V1][0] = sx(v);
            c.sq(2, at(sp, 0x190));
            c.mfc1(4, 7);
            c.lqc2(4, at(r(c, V1), 0x10));
            c.qmtc2(4, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, at(sp, 0x1b0));
            c.lq(2, at(sp, 0x1b0));
            c.sq(2, at(sp, 0x1a0));
            c.lqc2(3, at(sp, 0x190));
            c.lqc2(4, at(sp, 0x1a0));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, at(sp, 0x1b0));
            c.lq(2, at(sp, 0x1b0));
            c.sq(2, at(sp, 0x180));
            c.mfc1(3, 0);
            c.lqc2(4, at(sp, 0x180));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, at(sp, 0x1b0));
            c.lq(2, at(sp, 0x1b0));
            c.sq(2, at(sp, 0x170));
            c.lqc2(3, at(r(c, A2), 0x10));
            c.lqc2(4, at(sp, 0x170));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, at(r(c, A2), 0x10));
            // T[i] = c * (a[i-1] - a[i] + a[i+1] - a[i])
            //      + dt*c * (av[i-1] + av[i+1] - 2*av[i])
            let nodes = c.lw(at(r(c, A3), 0x24));
            let v = r(c, A1).wrapping_add(nodes as u32);
            c.g[A0][0] = sx(v);
            c.lwc1(3, at(r(c, A3), 0xc));
            c.lwc1(4, at(r(c, A3), 0x8));
            let node = r(c, A0);
            c.lwc1(2, at(node, 0x20));
            c.mul_s(5, 6, 3);
            c.lwc1(0, node.wrapping_sub(0x30));
            c.neg_s(4, 4);
            c.lwc1(1, at(node, 0x70));
            c.sub_s(0, 0, 2);
            c.add_s(0, 0, 1);
            c.sub_s(0, 0, 2);
            c.mul_s(3, 3, 0);
            c.swc1(3, r(c, T0));
            c.lwc1(2, at(node, 0x74));
            c.lwc1(1, node.wrapping_sub(0x2c));
            c.lwc1(0, at(node, 0x24));
            c.add_s(1, 1, 2);
            c.add_s(0, 0, 0);
            c.sub_s(1, 1, 0);
            c.mul_s(5, 5, 1);
            c.add_s(3, 3, 5);
            c.swc1(3, r(c, T0));
            // F[i] += v[i] * -damp + g
            c.mfc1(3, 4);
            c.lqc2(4, at(node, 0x10));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, at(sp, 0x190));
            c.lq(2, at(sp, 0x190));
            c.sq(2, at(sp, 0x180));
            c.lqc2(3, at(sp, 0x180));
            c.lqc2(4, sp);
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, at(sp, 0x190));
            c.lq(2, at(sp, 0x190));
            c.sq(2, at(sp, 0x170));
            c.lqc2(3, at(r(c, A2), 0x10));
            c.lqc2(4, at(sp, 0x170));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, at(r(c, A2), 0x10));
            // T[i] -= e * av[i]
            c.lwc1(0, at(r(c, A3), 0x10));
            let v = r(c, A2).wrapping_add(0x10);
            c.g[A2][0] = sx(v);
            let nodes = c.lw(at(r(c, A3), 0x24));
            set_r(c, V0, nodes as u32);
            c.neg_s(0, 0);
            c.lwc1(1, r(c, T0));
            let v = r(c, A1).wrapping_add(r(c, V0));
            c.g[V0][0] = sx(v);
            c.lwc1(2, at(r(c, V0), 0x24));
            let v = r(c, A1).wrapping_add(0x50);
            c.g[A1][0] = sx(v);
            c.mul_s(0, 0, 2);
            c.add_s(1, 1, 0);
            c.swc1(1, r(c, T0));
            let v = c.lw(r(c, A3)) as u32;
            c.g[V0][0] = sx(v);
            let v = r(c, V0).wrapping_sub(1);
            c.g[V0][0] = sx(v);
            let v = ((r(c, T1) as i64) < (r(c, V0) as i64)) as u32;
            c.g[V0][0] = sx(v);
            let more = c.g[V0][0] != 0;
            let v = r(c, T0).wrapping_add(0x4);
            c.g[T0][0] = sx(v);
            if !more {
                break;
            }
            if c.cycles >= c.next_event {
                c.pc = 0x0035_4088;
                c.event();
            }
        }
    }

    // Second pass: F[i] += nodes[i].f, T[i] += nodes[i].t.
    c.cycles += 6;
    let v = c.lw(r(c, A3)) as u32;
    c.g[V0][0] = sx(v);
    c.g[T0][0] = 0x1;
    let v = r(c, V0).wrapping_sub(1);
    c.g[V0][0] = sx(v);
    let v = ((r(c, T0) as i64) < (r(c, V0) as i64)) as u32;
    c.g[V0][0] = sx(v);
    let has_interior = c.g[V0][0] != 0;
    if !has_interior {
        let v = c.lw(r(c, A3)) as u32;
        c.g[V0][0] = sx(v);
    } else {
        c.cycles += 3;
        let v = at(r(c, T2), 0x4);
        c.g[A1][0] = sx(v);
        c.g[A2][0] = 0x50;
        set_r(c, A0, at(sp, 0x20));
        loop {
            c.cycles += 20;
            let nodes = c.lw(at(r(c, A3), 0x24));
            set_r(c, V0, nodes as u32);
            let v = r(c, T0).wrapping_add(1);
            c.g[T0][0] = sx(v);
            let v = r(c, A2).wrapping_add(r(c, V0));
            c.g[V0][0] = sx(v);
            c.lqc2(3, r(c, A0));
            let node = r(c, V0);
            c.lqc2(4, at(node, 0x30));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, r(c, A0));
            let nodes = c.lw(at(r(c, A3), 0x24));
            set_r(c, V1, nodes as u32);
            let v = r(c, A0).wrapping_add(0x10);
            c.g[A0][0] = sx(v);
            c.lwc1(0, r(c, A1));
            let v = r(c, A2).wrapping_add(r(c, V1));
            c.g[V1][0] = sx(v);
            let v = c.lw(r(c, A3)) as u32;
            c.g[V0][0] = sx(v);
            c.lwc1(1, at(r(c, V1), 0x40));
            let v = r(c, A2).wrapping_add(0x50);
            c.g[A2][0] = sx(v);
            let v = r(c, V0).wrapping_sub(1);
            c.g[V0][0] = sx(v);
            c.add_s(0, 0, 1);
            let v = ((r(c, T0) as i64) < (r(c, V0) as i64)) as u32;
            c.g[V0][0] = sx(v);
            c.swc1(0, r(c, A1));
            let more = c.g[V0][0] != 0;
            let v = r(c, A1).wrapping_add(0x4);
            c.g[A1][0] = sx(v);
            if !more {
                c.cycles += 1;
                let v = c.lw(r(c, A3)) as u32;
                c.g[V0][0] = sx(v);
                break;
            }
            if c.cycles >= c.next_event {
                c.pc = 0x0035_4348;
                c.event();
            }
        }
    }

    // Third pass: integrate the interior nodes.
    c.cycles += 5;
    c.g[T1][0] = 0x1;
    let v = r(c, V0).wrapping_sub(1);
    c.g[V0][0] = sx(v);
    let v = ((r(c, T1) as i64) < (r(c, V0) as i64)) as u32;
    c.g[V0][0] = sx(v);
    let no_interior = c.g[V0][0] == 0;
    let v = at(r(c, T2), 0x4);
    c.g[A2][0] = sx(v);
    if !no_interior {
        c.cycles += 8;
        c.mul_s(0, 6, 6);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 8);
        c.g[T0][0] = 0x50;
        set_r(c, A1, at(sp, 0x20));
        c.add_s(5, 0, 0);
        c.mul_s(7, 0, 6);
        loop {
            c.cycles += 110;
            c.mfc1(2, 6);
            c.lqc2(4, r(c, A1));
            c.qmtc2(2, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, at(sp, 0x170));
            let v = r(c, T1) << 4;
            c.g[A0][0] = sx(v);
            c.lq(3, at(sp, 0x170));
            set_r(c, V0, at(sp, 0x20));
            c.lwc1(1, at(r(c, A3), 0x4));
            let v = r(c, V0).wrapping_add(r(c, A0));
            c.g[V0][0] = sx(v);
            c.sq(3, at(sp, 0x160));
            let v = r(c, T1).wrapping_add(1);
            c.g[T1][0] = sx(v);
            c.lqc2(3, r(c, A1).wrapping_sub(0x10));
            c.lqc2(4, r(c, V0));
            let node = r(c, V0);
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, at(sp, 0x1a0));
            c.mul_s(1, 7, 1);
            c.lq(2, at(sp, 0x1a0));
            let v = r(c, A1).wrapping_add(0x10);
            c.g[A1][0] = sx(v);
            c.sq(2, at(sp, 0x190));
            c.mfc1(3, 1);
            c.lqc2(4, at(sp, 0x190));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, at(sp, 0x1a0));
            c.lwc1(0, at(r(c, A3), 0x4));
            c.lq(2, at(sp, 0x1a0));
            c.mul_s(0, 5, 0);
            c.sq(2, at(sp, 0x180));
            c.add_s(0, 0, 8);
            c.mfc1(3, 0);
            c.qmtc2(3, 3);
            c.vu0().div(0, 3, 3, 0);
            c.lqc2(4, at(sp, 0x180));
            c.vu0().waitq();
            c.vu0().mul(0b1111, 4, 4, Src::Q);
            c.sqc2(4, at(sp, 0x1a0));
            c.lq(2, at(sp, 0x1a0));
            c.sq(2, at(sp, 0x170));
            c.lqc2(3, at(sp, 0x160));
            c.lqc2(4, at(sp, 0x170));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, at(sp, 0x1a0));
            c.lwc1(0, at(r(c, A3), 0x4));
            c.lq(2, at(sp, 0x1a0));
            c.mul_s(0, 5, 0);
            c.sq(2, at(sp, 0x150));
            c.add_s(0, 0, 8);
            c.mfc1(3, 0);
            c.qmtc2(3, 3);
            c.vu0().div(0, 3, 3, 0);
            c.lqc2(4, at(sp, 0x150));
            c.vu0().waitq();
            c.vu0().mul(0b1111, 4, 4, Src::Q);
            c.sqc2(4, at(sp, 0x170));
            let nodes = c.lw(at(r(c, A3), 0x24));
            set_r(c, V1, nodes as u32);
            c.lq(2, at(sp, 0x170));
            let v = r(c, T0).wrapping_add(r(c, V1));
            c.g[V1][0] = sx(v);
            c.sq(2, at(sp, 0x160));
            c.sq(2, at(sp, 0x150));
            let node = r(c, V1);
            c.lqc2(3, at(node, 0x10));
            c.lqc2(4, at(sp, 0x150));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, at(node, 0x10));
            let nodes = c.lw(at(r(c, A3), 0x24));
            set_r(c, V0, nodes as u32);
            let v = r(c, T0).wrapping_add(r(c, V0));
            c.g[V0][0] = sx(v);
            c.mfc1(4, 6);
            let node = r(c, V0);
            c.lqc2(4, at(node, 0x10));
            c.qmtc2(4, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, at(sp, 0x170));
            let nodes = c.lw(at(r(c, A3), 0x24));
            c.lq(3, at(sp, 0x170));
            set_r(c, V0, nodes as u32);
            c.sq(3, at(sp, 0x160));
            let v = r(c, T0).wrapping_add(r(c, V0));
            c.g[V0][0] = sx(v);
            let node = r(c, V0);
            c.lqc2(3, node);
            c.lqc2(4, at(sp, 0x160));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, node);
            c.lwc1(1, at(r(c, A3), 0xc));
            c.lwc1(2, at(r(c, A2), 0x4));
            c.lwc1(4, r(c, A2).wrapping_sub(0x4));
            c.mul_s(3, 5, 1);
            c.mul_s(1, 7, 1);
            c.lwc1(0, r(c, A2));
            c.add_s(4, 4, 2);
            let nodes = c.lw(at(r(c, A3), 0x24));
            set_r(c, V0, nodes as u32);
            c.add_s(3, 3, 8);
            let v = r(c, A2).wrapping_add(0x4);
            c.g[A2][0] = sx(v);
            c.mul_s(0, 6, 0);
            let v = r(c, T0).wrapping_add(r(c, V0));
            c.g[V0][0] = sx(v);
            c.mul_s(1, 1, 4);
            let node = r(c, V0);
            c.lwc1(2, at(node, 0x24));
            c.div_s(1, 1, 3);
            c.add_s(0, 0, 1);
            c.div_s(0, 0, 3);
            c.add_s(2, 2, 0);
            c.swc1(2, at(node, 0x24));
            let nodes = c.lw(at(r(c, A3), 0x24));
            set_r(c, V1, nodes as u32);
            let v = r(c, T0).wrapping_add(r(c, V1));
            c.g[V1][0] = sx(v);
            let node = r(c, V1);
            c.lwc1(1, at(node, 0x24));
            c.lwc1(0, at(node, 0x20));
            c.mul_s(1, 6, 1);
            c.add_s(0, 0, 1);
            c.swc1(0, at(node, 0x20));
            let v = c.lw(r(c, A3)) as u32;
            c.g[V0][0] = sx(v);
            let v = r(c, V0).wrapping_sub(1);
            c.g[V0][0] = sx(v);
            let v = ((r(c, T1) as i64) < (r(c, V0) as i64)) as u32;
            c.g[V0][0] = sx(v);
            let more = c.g[V0][0] != 0;
            let v = r(c, T0).wrapping_add(0x50);
            c.g[T0][0] = sx(v);
            if !more {
                break;
            }
            if c.cycles >= c.next_event {
                c.pc = 0x0035_43d0;
                c.event();
            }
        }
    }

    // Fourth pass: clear the per-node force/torque accumulators.
    c.cycles += 3;
    let v = c.lw(r(c, A3)) as u32;
    c.g[V0][0] = sx(v);
    let none: bool = (c.g[V0][0] as i64) <= 0;
    c.g[A2][0] = 0;
    if !none {
        c.cycles += 3;
        c.g[V0][0] = 0x500000;
        c.g[A1][0] = 0;
        let v = r(c, V0).wrapping_sub(0xee0);
        c.g[T0][0] = sx(v);
        loop {
            c.cycles += 12;
            let nodes = c.lw(at(r(c, A3), 0x24));
            set_r(c, A0, nodes as u32);
            let v = r(c, A2).wrapping_add(1);
            c.g[A2][0] = sx(v);
            c.lq(3, r(c, T0));
            let v = r(c, A1).wrapping_add(r(c, A0));
            c.g[A0][0] = sx(v);
            let node = r(c, A0);
            c.sq(3, at(node, 0x30));
            let nodes = c.lw(at(r(c, A3), 0x24));
            set_r(c, V0, nodes as u32);
            let v = r(c, A1).wrapping_add(r(c, V0));
            c.g[V0][0] = sx(v);
            c.sw(at(r(c, V0), 0x40), 0);
            let v = c.lw(r(c, A3)) as u32;
            c.g[V0][0] = sx(v);
            let v = ((r(c, A2) as i64) < (r(c, V0) as i64)) as u32;
            c.g[V0][0] = sx(v);
            let more = c.g[V0][0] != 0;
            let v = r(c, A1).wrapping_add(0x50);
            c.g[A1][0] = sx(v);
            if !more {
                break;
            }
            if c.cycles >= c.next_event {
                c.pc = 0x0035_45a0;
                c.event();
            }
        }
    }

    c.cycles += 2;
    let frame_base = r(c, SP).wrapping_add(0x1c0);
    c.g[SP][0] = sx(frame_base);
    abi::ret(c);
}

// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, put_vec, scramble, Rng, DATA};

    const ONE: u32 = 0x3f80_0000;
    /// Where the springs test keeps the chain (16 nodes of 0x50 bytes).
    const NODES: u32 = 0x0000_0200;
    /// The 4x4 identity matrix the camera code copies before building.
    const IDENTITY: u32 = 0x004f_f1a0;
    const AXIS_X: u32 = 0x004f_f140;
    const AXIS_Y: u32 = 0x004f_f150;
    const AXIS_Z: u32 = 0x004f_f160;

    fn axes(c: &mut Cpu) {
        put_vec(c, IDENTITY, [ONE, 0, 0, 0]);
        put_vec(c, IDENTITY + 0x10, [0, ONE, 0, 0]);
        put_vec(c, IDENTITY + 0x20, [0, 0, ONE, 0]);
        put_vec(c, IDENTITY + 0x30, [0, 0, 0, ONE]);
        put_vec(c, AXIS_X, [ONE, 0, 0, 0]);
        put_vec(c, AXIS_Y, [0, ONE, 0, 0]);
        put_vec(c, AXIS_Z, [0, 0, ONE, 0]);
    }

    #[test]
    fn ps2_visual_effects_man_is_handle_null_matches() {
        check(
            ssx3_ee::s09::f_002f7be0__generated,
            ps2_visual_effects_man_is_handle_null,
            200,
            &|c, seed| {
                let mut r = Rng(0x2f7b_e001 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                let v = if seed % 3 == 0 { 0 } else { r.next_u32() };
                c.bus.write32(DATA, v);
                c.pc = 0x002f_7be0;
            },
        );
    }

    #[test]
    fn ssx_audio_system_is_sound_mode2_matches() {
        check(
            ssx3_ee::s07::f_002899e8__generated,
            ssx_audio_system_is_sound_mode2,
            200,
            &|c, seed| {
                let mut r = Rng(0x2899_e801 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                let v = match seed % 3 {
                    0 => 2,
                    1 => r.next_u32(),
                    _ => 3,
                };
                c.bus.write32(DATA + 0x608c, v);
                c.pc = 0x0028_99e8;
            },
        );
    }

    #[test]
    fn camera_sphere_trigger_contains_point_matches() {
        check(
            ssx3_ee::s02::f_00172278__generated,
            camera_sphere_trigger_contains_point,
            200,
            &|c, seed| {
                let mut r = Rng(0x1722_7801 ^ seed as u64);
                scramble(c, &mut r);
                axes(c);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x40);
                // Position, angles and radii of the trigger.
                for i in 0..6 {
                    c.bus.write32(DATA + 4 * i, r.edge_f32());
                }
                for i in 6..8 {
                    c.bus.write32(DATA + 4 * i, r.f32(2.0));
                }
                c.bus.write32(DATA + 0x18, (1.0f32 + (seed % 4) as f32).to_bits());
                c.bus.write32(DATA + 0x1c, (1.0f32 + (seed % 3) as f32).to_bits());
                c.bus.write32(DATA + 0x20, (1.0f32 + (seed % 5) as f32).to_bits());
                // The probe either sits on the trigger or far outside it, so
                // both results of the final compare are covered.
                let s: f32 = if seed % 2 == 0 { 0.25 } else { 50.0 };
                put_vec(c, DATA + 0x40, [s.to_bits(), 0, 0, ONE]);
                c.pc = 0x0017_2278;
            },
        );
    }

    #[test]
    fn camera_bound_obj_get_world_point_matches() {
        check(
            ssx3_ee::s02::f_00170d20__generated,
            camera_bound_obj_get_world_point,
            200,
            &|c, seed| {
                let mut r = Rng(0x170d_2001 ^ seed as u64);
                scramble(c, &mut r);
                axes(c);
                let out = DATA + 0x200;
                c.g[A0][0] = sx(out);
                c.g[A1][0] = sx(DATA);
                c.g[A2][0] = sx((seed % 8) as u32);
                // Position, three angles, scale, then the eight corners.
                for i in 0..8 {
                    c.bus.write32(DATA + 4 * i, r.edge_f32());
                }
                c.bus.write32(DATA + 0x18, r.f32(2.0));
                c.bus.write32(DATA + 0x1c, r.f32(2.0));
                c.bus.write32(DATA + 0x20, r.f32(2.0));
                for i in 0..8 * 3 {
                    c.bus.write32(DATA + 0x28 + 4 * i, r.edge_f32());
                }
                c.pc = 0x0017_0d20;
            },
        );
    }

    #[test]
    fn flex_bridge_springs_step_matches() {
        check(
            ssx3_ee::s11::f_00353fc0__generated,
            flex_bridge_springs_step,
            120,
            &|c, seed| {
                let mut r = Rng(0x353f_c001 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(crate::difftest::GAME_GP);
                // The frame-clock singleton the step length comes from.
                c.bus.write32(crate::difftest::GAME_GP + 0x2a74, DATA + 0x300);
                c.bus.write32(DATA + 0x310, 0x42f0_0000); // 120.0f
                // Zero vector used for F[0]/F[n-1] and the cleared forces.
                put_vec(c, 0x004f_f120, [0, 0, 0, ONE]);
                // Three or four nodes: both interior-loop lengths, and never
                // the empty chain (whose `F[0] = F[-1]` write is a no-op).
                let n = 3 + seed % 2;
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x4, r.edge_f32()); // k
                c.bus.write32(DATA + 0x8, r.edge_f32()); // damp
                c.bus.write32(DATA + 0xc, r.edge_f32()); // c
                c.bus.write32(DATA + 0x10, r.edge_f32()); // e
                c.bus.write32(DATA + 0x1c, r.edge_f32()); // rest
                c.bus.write32(DATA + 0x20, r.edge_f32()); // scale
                // The chain sits low in RAM: the node pointer is then small,
                // so the fixture stays bounded whatever the loops read as the
                // node count.
                c.bus.write32(DATA + 0x24, NODES);
                for i in 0..16 * 0x50 / 4 {
                    c.bus.write32(NODES + 4 * i, r.edge_f32());
                }
                // The count last, so nothing in the fixture can overwrite it.
                c.bus.write32(DATA, n);
                c.pc = 0x0035_3fc0;
            },
        );
    }
}
