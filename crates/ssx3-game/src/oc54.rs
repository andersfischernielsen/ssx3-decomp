//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for the script-camera `GetView` methods, the line-bound
//! camera's world-space endpoints and the DXT colour-block decoder, each with
//! a differential test against the generated body.
//!
//! - **Script camera views** (`{Manual,Target,Subject,Algorithmic}
//!   ScriptCameraAlgo_GetView`): the four are the same code (they differ only
//!   in the offset of the wobble sub-object and their return addresses), so
//!   they share [`script_camera_get_view`]. Start from the identity matrix,
//!   translate by the camera position plus the wobble offset, rotate about
//!   the world X, Z and Y axes by the script angles (plus the wobble phase),
//!   and hand the matrix to `Mat4_ToPosQuat`, which writes the position and
//!   the rotation quaternion into the object.
//! - **Line-bound endpoints** (`CameraLineBoundObj_GetWorldStart/End`): the
//!   same kind of matrix from a line bound's transform fields (translation,
//!   three axis rotations, non-uniform scale), applied to the bound's start
//!   (or end) point.
//! - **DXT colours** (`SHP_DecodeDXTColorBlock`): expand a DXT1 colour block
//!   into four 16-bit palette entries (the two endpoints, then either the
//!   1/3–2/3 blend or the average, depending on their order) and unpack the
//!   four 2-bit indices of each row into pixels.
//!
//! The matrix work is VU0 macro-mode code; it is issued with exactly the
//! original's COP2 operations, in order (the recurring "sum of scaled
//! columns" product, the axis-angle build and the column scale are helpers
//! below), so VF/ACC and the VU0 pipeline end where the original leaves them.
//! FPU arithmetic uses [`FpuF32`] and writes the registers the original
//! leaves (notably `f0`/`f1` at every return). Calls follow [`crate::call`].

use crate::abi;
use crate::call::{call_ptr, guest, sp, Flow, Frame};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0016_9828, manual_script_camera_get_view, "ManualScriptCameraAlgo_GetView"),
    (0x0016_9f88, target_script_camera_get_view, "TargetScriptCameraAlgo_GetView"),
    (0x0016_a868, subject_script_camera_get_view, "SubjectScriptCameraAlgo_GetView"),
    (0x0016_b7b8, algorithmic_script_camera_get_view, "AlgorithmicScriptCameraAlgo_GetView"),
    (0x0017_0240, camera_line_bound_world_start, "CameraLineBoundObj_GetWorldStart"),
    (0x0017_07b0, camera_line_bound_world_end, "CameraLineBoundObj_GetWorldEnd"),
    (0x003b_2e78, shp_decode_dxt_color_block, "SHP_DecodeDXTColorBlock"),
];

// ---------------------------------------------------------------------------
// Callees and fixed globals.

mod callee {
    pub const MATH_SIN_COS: u32 = 0x0031_be50;
    pub const MAT4_TO_POS_QUAT: u32 = 0x0031_b748;
    pub const CAMERA_WOBBLE_GET_OFFSET: u32 = 0x0016_0228;
    pub const CAMERA_WOBBLE_PHASE: u32 = 0x0016_03f0;
}

/// The identity matrix and the three world axes, in `.bss` at boot.
const IDENTITY_MAT: u32 = 0x004f_f1a0;
const AXIS_X: u32 = 0x004f_f140;
const AXIS_Z: u32 = 0x004f_f150;
const AXIS_Y: u32 = 0x004f_f160;

/// Return addresses of the four `GetView` bodies: the addresses their calls
/// come back to, relative to the function's entry.
mod ret {
    pub const WOBBLE: u32 = 0x6c;
    pub const PHASE: u32 = 0xbc;
    pub const ROT_X: u32 = 0xdc;
    pub const ROT_Z: u32 = 0x228;
    pub const ROT_Y: u32 = 0x368;
    pub const TO_POS_QUAT: u32 = 0x4a4;
    /// The two line-bound functions' call returns.
    pub const LINE_ROT_X: u32 = 0xac;
    pub const LINE_ROT_Z: u32 = 0x1f4;
    pub const LINE_ROT_Y: u32 = 0x33c;
}

/// Fields of the script camera both code paths read.
mod camera {
    /// The script's aim/rotation angles: pitch at +0x40, roll/yaw at +0x44
    /// and +0x48, world position at +0x30.
    pub const POSITION: u32 = 0x30;
    pub const ANGLE_PITCH: u32 = 0x40;
    pub const ANGLE_A: u32 = 0x44;
    pub const ANGLE_B: u32 = 0x48;
}

/// Fields of a line-bound object (the matrix source).
mod line_bound {
    /// Translation (x, y, z) at +0, the three rotation angles and the scale
    /// (x, y, z) at +0x18, then the local start point at +0x2c and the end
    /// point at +0x38.
    pub const TRANSLATION: u32 = 0x0;
    pub const ROT_Z: u32 = 0xc;
    pub const ROT_X: u32 = 0x10;
    pub const ROT_Y: u32 = 0x14;
    pub const SCALE: u32 = 0x18;
}

// ---------------------------------------------------------------------------
// Small helpers.

/// A 16-byte aligned quadword copy `lq`/`sq` through GPR 3.
#[inline]
fn quad_copy(c: &mut Cpu, from: u32, to: u32) {
    c.lq(3, from);
    c.sq(3, to);
}

/// `m = *(m0)` as the original copies the identity: four `lqc2`/`sqc2`
/// pairs through vf1..vf4.
fn copy_matrix(c: &mut Cpu, dst: u32, src: u32) {
    for i in 0..4u32 {
        c.lqc2(1 + i as usize, src + 0x10 * i);
    }
    for i in 0..4u32 {
        c.sqc2(1 + i as usize, dst + 0x10 * i);
    }
}

/// `out = m * v`, the VU0 "sum of scaled columns" product: vf4..vf7 are the
/// matrix columns, vf8 the vector, and the result lands in vf12 (left in the
/// accumulator as the original leaves it).
fn mat_vec(c: &mut Cpu, m: u32, v: u32, out: u32) {
    for i in 0..4u32 {
        c.lqc2(4 + i as usize, m + 0x10 * i);
    }
    c.lqc2(8, v);
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.sqc2(12, out);
}

/// `m = m * r` for two column-major 4x4 matrices, in place, as the original
/// emits it: four independent `MULA/MADDA/MADDA/MADD` chains into
/// vf12..vf15.
fn mat_mul(c: &mut Cpu, m: u32, r: u32) {
    for i in 0..4u32 {
        c.lqc2(4 + i as usize, m + 0x10 * i);
    }
    for i in 0..4u32 {
        c.lqc2(8 + i as usize, r + 0x10 * i);
    }
    for j in 0..4u32 {
        let s = (8 + j) as u8;
        let d = (12 + j) as u8;
        c.vu0().mula(0b1111, 4, Src::Bc(s, X));
        c.vu0().madda(0b1111, 5, Src::Bc(s, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(s, Z));
        c.vu0().madd(0b1111, d, 7, Src::Bc(s, W));
    }
    for i in 0..4u32 {
        c.sqc2(12 + i as usize, m + 0x10 * i);
    }
}

/// One `ctmRotAxis` step: `sincos(angle)` into `sin`/`cos`, build the
/// axis-angle rotation matrix at `r`, then `m = m * r`.
///
/// The matrix is the classic `a = 1 - cos` form, but with the products in
/// the original's order (`(a*z)*x` and `(a*x)*z` stay separate expressions
/// because they round differently):
///
/// ```text
/// [ a*xx+c   a*xy-sz  a*xz+sy ]
/// [ a*xy+sz  a*yy+c   a*yz-sx ]
/// [ a*xz-sy  a*yz+sx  a*zz+c  ]
/// ```
///
/// The 4th column is `(0, 0, 0, 1)`. Leaves `f0` and `f1` as the original's
/// last two results (`f0 = a*x*y - s*z`, `f1 = a*y*y + cos`).
#[allow(clippy::too_many_arguments)]
fn rot_axis(c: &mut Cpu, m: u32, r: u32, axis: u32, angle: FpuF32, sin: u32, cos: u32, ret: u32) -> Flow {
    c.f[12] = angle.0;
    c.g[A0][0] = sx(sin);
    c.g[A1][0] = sx(cos);
    call_ptr(c, callee::MATH_SIN_COS, ret)?;
    let cos_v = Addr(cos).read_fpu(c);
    let s = Addr(sin).read_fpu(c);
    let x = Addr(axis).read_fpu(c);
    let y = Addr(axis + 4).read_fpu(c);
    let z = Addr(axis + 8).read_fpu(c);
    let a = FpuF32::ONE - cos_v;

    let az = a * z;
    let ax = a * x;
    let ay = a * y;
    let sz = s * z;
    let sx_ = s * x;
    let azz = az * z;
    let axz = ax * z;
    let azx = az * x;
    let axx = ax * x;
    let ayz = ay * z;
    let axy = ay * x;
    let sy = s * y;
    let azy = az * y;
    let axy2 = ax * y;
    let ayy = ay * y;

    let m00 = axx + cos_v;
    let m01 = axy - sz;
    let m02 = azx + sy;
    let m10 = axy2 + sz;
    let m11 = ayy + cos_v;
    let m12 = azy - sx_;
    let m20 = axz - sy;
    let m21 = ayz + sx_;
    let m22 = azz + cos_v;
    let one = FpuF32::ONE;

    Addr(r + 0x00).write_fpu(c, m00);
    Addr(r + 0x04).write_fpu(c, m01);
    Addr(r + 0x08).write_fpu(c, m02);
    Addr(r + 0x0c).write_fpu(c, FpuF32::ZERO);
    Addr(r + 0x10).write_fpu(c, m10);
    Addr(r + 0x14).write_fpu(c, m11);
    Addr(r + 0x18).write_fpu(c, m12);
    Addr(r + 0x1c).write_fpu(c, FpuF32::ZERO);
    Addr(r + 0x20).write_fpu(c, m20);
    Addr(r + 0x24).write_fpu(c, m21);
    Addr(r + 0x28).write_fpu(c, m22);
    Addr(r + 0x2c).write_fpu(c, FpuF32::ZERO);
    Addr(r + 0x30).write_fpu(c, FpuF32::ZERO);
    Addr(r + 0x34).write_fpu(c, FpuF32::ZERO);
    Addr(r + 0x38).write_fpu(c, FpuF32::ZERO);
    Addr(r + 0x3c).write_fpu(c, one);

    // The last two values the original leaves in the FPU file.
    c.f[0] = m01.0;
    c.f[1] = m11.0;
    mat_mul(c, m, r);
    Ok(())
}

/// `m = m * diag(scale)`: each column of `m` scaled by one component of the
/// scale vector, through the stack scratch at `tmp`, then copied to `via`
/// and back into `m` (the original's double copy).
fn scale_columns(c: &mut Cpu, m: u32, scale: u32, tmp: u32, via: u32) {
    c.lqc2(1, scale);
    for i in 0..4u32 {
        c.lqc2(4 + i as usize, m + 0x10 * i);
    }
    c.vu0().mul(0b1111, 8, 4, Src::Bc(1, X));
    c.vu0().mul(0b1111, 9, 5, Src::Bc(1, Y));
    c.vu0().mul(0b1111, 10, 6, Src::Bc(1, Z));
    c.vu0().mul(0b1111, 11, 7, Src::Bc(1, W));
    for i in 0..4u32 {
        c.sqc2(8 + i as usize, tmp + 0x10 * i);
    }
    for i in 0..4u32 {
        c.lqc2(1 + i as usize, tmp + 0x10 * i);
    }
    for i in 0..4u32 {
        c.sqc2(1 + i as usize, via + 0x10 * i);
    }
    for i in 0..4u32 {
        c.lqc2(1 + i as usize, via + 0x10 * i);
    }
    for i in 0..4u32 {
        c.sqc2(1 + i as usize, m + 0x10 * i);
    }
}

// ---------------------------------------------------------------------------
// Script camera views.

/// Shared body of the four `*ScriptCameraAlgo_GetView` functions: `out` is
/// the object whose view (position at +0, quaternion at +0x10) is written,
/// `cam` the script camera, and `wobble` the offset of its wobble sub-object.
fn script_camera_get_view(c: &mut Cpu, out: u64, cam: u32, wobble: u32, base: u32) -> Flow {
    c.cycles += 308;
    let frame = Frame::open(c, 0x150);
    frame.save(c, S0, 0x130);
    frame.save(c, S1, 0x120);
    frame.save(c, S2, 0x110);
    frame.save(c, S3, 0x100);
    frame.save(c, S4, 0xf0);
    frame.save(c, S5, 0xe0);
    frame.save_ra(c, 0xd0);
    frame.save_fpr(c, 20, 0x140);

    let m = sp(c); // the matrix under construction, sp+0..sp+0x40
    copy_matrix(c, m, IDENTITY_MAT);

    // Registers at the calls: the original keeps the camera in s2, the result
    // object in s5 and the wobble sub-object in s0. Callees spill the
    // callee-saved registers into their own frames, so these have to hold the
    // original's values (the rest stay as the caller left them).
    c.g[S2][0] = c.g[A1][0];
    c.g[S5][0] = c.g[A0][0];
    c.g[S0][0] = sx(wobble);

    // Translate by (position + wobble offset): the wobble comes back as a
    // vector at sp+0x50, which the VU0 adds lane-wise to the position and
    // then pushes through the matrix as its new 4th column.
    c.f[13] = Addr(cam + camera::ANGLE_B).read_fpu(c).0;
    c.f[12] = Addr(cam + camera::ANGLE_A).read_fpu(c).0;
    c.g[A0][0] = sx(m + 0x50);
    c.g[A1][0] = sx(wobble);
    call_ptr(c, callee::CAMERA_WOBBLE_GET_OFFSET, base + ret::WOBBLE)?;
    c.lqc2(3, cam + camera::POSITION);
    c.lqc2(4, m + 0x50);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, m + 0x60);
    quad_copy(c, m + 0x60, m + 0x40);
    mat_vec(c, m, m + 0x40, m + 0x30);

    c.g[AT][0] = FpuF32::ONE.0 as u64;
    c.mtc1(1, 20);
    c.g[A0][0] = sx(wobble);
    call_ptr(c, callee::CAMERA_WOBBLE_PHASE, base + ret::PHASE)?;

    c.g[S1][0] = 0x0050_0000;
    c.g[S3][0] = sx(m);

    let phase = FpuF32(c.f[0]);
    let pitch = Addr(cam + camera::ANGLE_PITCH).read_fpu(c) + phase;
    c.g[S0][0] = sx(AXIS_X);
    rot_axis(c, m, m + 0x70, AXIS_X, pitch, m + 0xb0, m + 0xb4, base + ret::ROT_X)?;
    let roll = -Addr(cam + camera::ANGLE_B).read_fpu(c);
    c.g[S0][0] = 0x0050_0000;
    c.g[S1][0] = sx(AXIS_Z);
    c.g[S3][0] = sx(m);
    rot_axis(c, m, m + 0x40, AXIS_Z, roll, m + 0xb8, m + 0xbc, base + ret::ROT_Z)?;
    let yaw = Addr(cam + camera::ANGLE_A).read_fpu(c);
    c.g[S0][0] = sx(m);
    c.g[S4][0] = sx(m + 0x40);
    rot_axis(c, m, m + 0x40, AXIS_Y, yaw, m + 0xc0, m + 0xc4, base + ret::ROT_Y)?;

    // Incidental registers the original leaves in the 3rd rotation step.
    c.g[V0][0] = sx(0x0050_0000);
    c.g[V1][0] = sx(AXIS_Y);

    c.g[A0][0] = out;
    c.g[A1][0] = sx(m);
    call_ptr(c, callee::MAT4_TO_POS_QUAT, base + ret::TO_POS_QUAT)?;
    c.g[V0][0] = out;

    frame.restore(c, S0, 0x130);
    frame.restore(c, S1, 0x120);
    frame.restore(c, S2, 0x110);
    frame.restore(c, S3, 0x100);
    frame.restore(c, S4, 0xf0);
    frame.restore(c, S5, 0xe0);
    frame.restore_ra(c, 0xd0);
    frame.restore_fpr(c, 20, 0x140);
    frame.close(c);
    Ok(())
}

/// `void ManualScriptCameraAlgo_GetView(SomeObj *out, Camera *cam)` (0x169828).
pub fn manual_script_camera_get_view(c: &mut Cpu) {
    guest(c, |c| {
        let out = c.g[A0][0];
        let cam = abi::arg(c, 1);
        script_camera_get_view(c, out, cam, cam.wrapping_add(0x74), 0x0016_9828)
    })
}

/// `void TargetScriptCameraAlgo_GetView(SomeObj *out, Camera *cam)` (0x169f88).
pub fn target_script_camera_get_view(c: &mut Cpu) {
    guest(c, |c| {
        let out = c.g[A0][0];
        let cam = abi::arg(c, 1);
        script_camera_get_view(c, out, cam, cam.wrapping_add(0x74), 0x0016_9f88)
    })
}

/// `void SubjectScriptCameraAlgo_GetView(SomeObj *out, Camera *cam)` (0x16a868).
pub fn subject_script_camera_get_view(c: &mut Cpu) {
    guest(c, |c| {
        let out = c.g[A0][0];
        let cam = abi::arg(c, 1);
        script_camera_get_view(c, out, cam, cam.wrapping_add(0x70), 0x0016_a868)
    })
}

/// `void AlgorithmicScriptCameraAlgo_GetView(SomeObj *out, Camera *cam)`
/// (0x16b7b8).
pub fn algorithmic_script_camera_get_view(c: &mut Cpu) {
    guest(c, |c| {
        let out = c.g[A0][0];
        let cam = abi::arg(c, 1);
        script_camera_get_view(c, out, cam, cam.wrapping_add(0x70), 0x0016_b7b8)
    })
}

// ---------------------------------------------------------------------------
// Line-bound endpoints.

/// Shared body of `CameraLineBoundObj_GetWorldStart/End`: build the bound's
/// transform (translation, then rotations about X, Z and Y, then a
/// non-uniform scale) and transform `point` into world space at `out`.
fn camera_line_bound_world(c: &mut Cpu, out: u64, bound: u32, point: u32, base: u32) -> Flow {
    c.cycles += 348;
    let frame = Frame::open(c, 0x150);
    frame.save(c, S0, 0x130);
    frame.save(c, S1, 0x120);
    frame.save(c, S2, 0x110);
    frame.save(c, S3, 0x100);
    frame.save_ra(c, 0xf0);
    frame.save_fpr(c, 20, 0x140);

    let m = sp(c);
    copy_matrix(c, m, IDENTITY_MAT);

    // Translation: (x, y, z, 1) from the bound's first three words.
    c.f[0] = Addr(bound + line_bound::TRANSLATION).read_fpu(c).0;
    c.f[1] = Addr(bound + line_bound::TRANSLATION + 4).read_fpu(c).0;
    c.f[2] = Addr(bound + line_bound::TRANSLATION + 8).read_fpu(c).0;
    c.g[AT][0] = FpuF32::ONE.0 as u64;
    c.mtc1(1, 20);
    Addr(m + 0x40).write_fpu(c, FpuF32(c.f[0]));
    Addr(m + 0x44).write_fpu(c, FpuF32(c.f[1]));
    Addr(m + 0x48).write_fpu(c, FpuF32(c.f[2]));
    Addr(m + 0x4c).write_fpu(c, FpuF32::ONE);
    mat_vec(c, m, m + 0x40, m + 0x30);

    let a = Addr(bound + line_bound::ROT_X).read_fpu(c);
    rot_axis(c, m, m + 0x50, AXIS_X, a, m + 0xd0, m + 0xd4, base + ret::LINE_ROT_X)?;
    let b = Addr(bound + line_bound::ROT_Y).read_fpu(c);
    rot_axis(c, m, m + 0x40, AXIS_Z, b, m + 0xd8, m + 0xdc, base + ret::LINE_ROT_Z)?;
    let cc = Addr(bound + line_bound::ROT_Z).read_fpu(c);
    rot_axis(c, m, m + 0x40, AXIS_Y, cc, m + 0xe0, m + 0xe4, base + ret::LINE_ROT_Y)?;

    // Scale: (sx, sy, sz, 1) multiplies the matrix columns.
    c.f[2] = Addr(bound + line_bound::SCALE).read_fpu(c).0;
    c.f[1] = Addr(bound + line_bound::SCALE + 4).read_fpu(c).0;
    c.f[0] = Addr(bound + line_bound::SCALE + 8).read_fpu(c).0;
    Addr(m + 0x80).write_fpu(c, FpuF32(c.f[2]));
    Addr(m + 0x84).write_fpu(c, FpuF32(c.f[1]));
    Addr(m + 0x88).write_fpu(c, FpuF32(c.f[0]));
    Addr(m + 0x8c).write_fpu(c, FpuF32::ONE);
    scale_columns(c, m, m + 0x80, m + 0x90, m + 0x40);

    // Transform the point (x, y, z, 1) and copy the result to `out`.
    c.f[2] = Addr(bound + point + 8).read_fpu(c).0;
    c.g[V0][0] = out;
    c.f[0] = Addr(bound + point).read_fpu(c).0;
    c.f[1] = Addr(bound + point + 4).read_fpu(c).0;
    Addr(m + 0x4c).write_fpu(c, FpuF32::ONE);
    Addr(m + 0x40).write_fpu(c, FpuF32(c.f[0]));
    Addr(m + 0x44).write_fpu(c, FpuF32(c.f[1]));
    Addr(m + 0x48).write_fpu(c, FpuF32(c.f[2]));
    mat_vec(c, m, m + 0x40, m + 0x50);
    frame.restore(c, S0, 0x130);
    quad_copy(c, m + 0x50, out as u32);
    frame.restore(c, S1, 0x120);
    frame.restore(c, S3, 0x100);
    frame.restore(c, S2, 0x110);
    frame.restore_ra(c, 0xf0);
    frame.restore_fpr(c, 20, 0x140);
    frame.close(c);
    Ok(())
}

/// `void CameraLineBoundObj_GetWorldStart(vec4 *out, LineBound *b)` (0x170240).
pub fn camera_line_bound_world_start(c: &mut Cpu) {
    guest(c, |c| {
        let out = c.g[A0][0];
        let bound = abi::arg(c, 1);
        camera_line_bound_world(c, out, bound, 0x2c, 0x0017_0240)
    })
}

/// `void CameraLineBoundObj_GetWorldEnd(vec4 *out, LineBound *b)` (0x1707b0).
pub fn camera_line_bound_world_end(c: &mut Cpu) {
    guest(c, |c| {
        let out = c.g[A0][0];
        let bound = abi::arg(c, 1);
        camera_line_bound_world(c, out, bound, 0x38, 0x0017_07b0)
    })
}

// ---------------------------------------------------------------------------
// DXT colour block.

/// 16-bit RGB555 to the output's RGBA1555 form: red into bits 10-14, green
/// into 5-9, blue into 0-4 and the alpha bit set.
fn to_1555(v: u64) -> u64 {
    let v = v as u32 & 0xffff;
    ((((v & 0x1f) << 10) | ((v >> 1) & 0x3e0) | (v >> 11)) | 0x8000) as u64
}

/// `void SHP_DecodeDXTColorBlock(u64 *dst, int pitch, const u8 *block)`
/// (0x3b2e78): decode one DXT1 colour block into four 16-byte pixels per
/// row, four rows `pitch` bytes apart.
///
/// The block's first word holds the two 565 endpoints. Their order decides
/// the palette: four blended entries when `c0 >= c1`, otherwise the average
/// of the pair (and transparent black). Each index byte then selects four
/// entries 2 bits at a time.
pub fn shp_decode_dxt_color_block(c: &mut Cpu) {
    c.cycles += 34;
    let frame = Frame::open(c, 0x50);
    frame.save(c, S0, 0x40);
    frame.save(c, S1, 0x30);
    frame.save(c, S2, 0x20);
    let stack = sp(c);
    let mut dst = abi::arg(c, 0);
    let pitch = abi::arg(c, 1);
    let block = abi::arg(c, 2);

    // Unpack the two endpoints to 24-bit RGB (the odd `c & 0xf8000000`
    // term is what the original adds to the second one).
    let cw = c.ld(block);
    let c0 = ((cw & 0x1f) << 3) | ((cw & 0x7e0) << 5) | ((cw & 0xf800) << 8);
    let c1 = ((cw >> 13) & 0xf8) | ((cw >> 11) & 0xfc00) | ((cw & 0xf800_0000) >> 8);
    let alpha = 0x40204u64;
    c.sd(stack + 8, c1 | alpha);
    c.sd(stack, c0 | alpha);

    if c0 < c1 {
        // Endpoint order says "four-colour with transparency": the third and
        // fourth entries are the averages of the two.
        c.cycles += 30;
        let (r0, g0, b0) = (c0 & 0xff, c0 & 0xff00, c0 & 0xff0000);
        let (r1, g1, b1) = (c1 & 0xff, c1 & 0xff00, c1 & 0xff0000);
        let sum = (r0 + r1) | ((g0 + g1) | (b0 + b1));
        c.sd(stack + 0x18, 0);
        c.sd(stack + 0x10, (sum >> 1) | 0xff00_0000);
    } else {
        // Three-colour mode: 1/3 and 2/3 blends of the endpoints.
        c.cycles += 72;
        let (r0, g0, b0) = (c0 & 0xff, c0 & 0xff00, c0 & 0xff0000);
        let (r1, g1, b1) = (c1 & 0xff, c1 & 0xff00, c1 & 0xff0000);
        let pal2 = (((5 * r0 + 3 * r1) >> 3) & 0xff)
            | (((5 * g0 + 3 * g1) >> 3) & 0xff00)
            | (((5 * b0 + 3 * b1) >> 3) & 0xff0000);
        let pal3 = ((((3 * r0 + 5 * r1) >> 3) & 0xff)
            | (((3 * g0 + 5 * g1) >> 3) & 0xff00)
            | (((3 * b0 + 5 * b1) >> 3) & 0xff0000))
            | 0xff00_0000;
        c.sd(stack + 0x10, pal2);
        c.sd(stack + 0x18, pal3);
    }
    let pal0 = c.ld(stack) | 0xff00_0000;
    let pal1 = c.ld(stack + 8) | 0xff00_0000;
    let pal2 = c.ld(stack + 0x10) | 0xff00_0000;
    c.sd(stack, pal0);
    c.sd(stack + 8, pal1);
    c.sd(stack + 0x10, pal2);

    // The 16-bit entries are re-encoded in the output's own (RGB555 with an
    // alpha bit) ordering; a zero entry (the transparent one) stays zero.
    c.cycles += 13;
    let mut conv = [0u64; 4];
    let t0 = c.lhu(stack);
    if t0 != 0 {
        c.cycles += 10;
        conv[0] = to_1555(t0);
    }
    c.cycles += 3;
    let t1 = c.lhu(stack + 8);
    c.sd(stack, conv[0]);
    if t1 != 0 {
        c.cycles += 10;
        conv[1] = to_1555(t1);
    }
    c.cycles += 4;
    let t2 = c.lhu(stack + 0x10);
    c.sd(stack + 8, conv[1]);
    if t2 != 0 {
        c.cycles += 10;
        conv[2] = to_1555(t2);
    }
    c.cycles += 3;
    let t3 = c.lhu(stack + 0x18);
    c.sd(stack + 0x10, conv[2]);
    if t3 != 0 {
        c.cycles += 10;
        conv[3] = to_1555(t3);
    }

    // Four rows; each index byte picks four palette entries, two bits each.
    c.cycles += 93;
    c.sd(stack + 0x18, conv[3]);
    for row in 0..4u32 {
        let b = c.lbu(block + 4 + row);
        let idx = [(b & 3) << 3, (b << 1) & 0x18, (b >> 1) & 0x18, (b >> 6) << 3];
        let p0 = c.ld(stack + idx[0] as u32);
        let p1 = c.ld(stack + idx[1] as u32);
        let p2 = c.ld(stack + idx[2] as u32);
        let p3 = c.ld(stack + idx[3] as u32);
        c.sd(dst, p0);
        c.sd(dst + 8, p1);
        c.sd(dst + 0x10, p2);
        c.g[V0][0] = p3;
        c.g[V1][0] = sx(stack + idx[3] as u32);
        c.sd(dst + 0x18, p3);
        dst = dst.wrapping_add(pitch);
    }

    frame.restore(c, S0, 0x40);
    frame.restore(c, S1, 0x30);
    frame.restore(c, S2, 0x20);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    c.pc = t;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, put_vec, scramble, Rng, DATA};
    use ssx3_ee as ee;

    const SEEDS: u32 = 250;
    const OUT: u32 = DATA;
    const CAM: u32 = DATA + 0x800;
    const BOUND: u32 = DATA + 0x1000;
    const BLOCK: u32 = DATA + 0x2000;
    /// `1/(2 pi)`-style wobble angle scale in `.sdata`.
    const WOBBLE_SCALE: u32 = 0x0049_c4d4;
    /// The vector `CameraWobble_GetOffset` returns when the wobble is off.
    const NO_WOBBLE: u32 = 0x004f_f120;

    fn f(x: f32) -> u32 {
        x.to_bits()
    }

    /// The boot-time matrices/axes and the wobble support globals.
    fn put_globals(c: &mut Cpu, r: &mut Rng) {
        put_vec(c, IDENTITY_MAT, [f(1.0), 0, 0, 0]);
        put_vec(c, IDENTITY_MAT + 0x10, [0, f(1.0), 0, 0]);
        put_vec(c, IDENTITY_MAT + 0x20, [0, 0, f(1.0), 0]);
        put_vec(c, IDENTITY_MAT + 0x30, [0, 0, 0, f(1.0)]);
        put_vec(c, AXIS_X, [f(1.0), 0, 0, 0]);
        put_vec(c, AXIS_Z, [0, 0, f(1.0), 0]);
        put_vec(c, AXIS_Y, [0, f(1.0), 0, 0]);
        put_vec(c, NO_WOBBLE, [r.f32(3.0), r.f32(3.0), r.f32(3.0), r.f32(3.0)]);
        c.bus.write32(WOBBLE_SCALE, r.f32(1.0));
    }

    /// A script camera: position at +0x30, script angles at +0x40..0x48 and
    /// a wobble sub-object at +0x70 (also covering the +0x74 variant).
    fn put_script_camera(c: &mut Cpu, r: &mut Rng, seed: u32) {
        put_vec(c, CAM + camera::POSITION, [r.f32(500.0), r.f32(500.0), r.f32(500.0), 0]);
        for off in [camera::ANGLE_PITCH, camera::ANGLE_A, camera::ANGLE_B] {
            c.bus.write32(CAM + off, r.f32(6.0));
        }
        for k in 0..12u32 {
            c.bus.write32(CAM + 0x70 + 4 * k, r.f32(4.0));
        }
        // Every fourth seed takes the wobble's early out (offset 0x18 is the
        // disabled flag); some seeds divide by a zero period.
        let zero = if seed % 4 == 0 { 0 } else { r.f32(2.0) };
        c.bus.write32(CAM + 0x70 + 0x18, zero);
        c.bus.write32(CAM + 0x74 + 0x18, zero);
        if seed % 7 == 0 {
            c.bus.write32(CAM + 0x70 + 0x2c, 0);
            c.bus.write32(CAM + 0x74 + 0x2c, 0);
        }
        if seed % 5 == 0 {
            c.bus.write32(CAM + 0x74 + 0x1c, 0);
        }
    }

    #[test]
    fn manual_script_camera_get_view_matches() {
        check(ee::s02::f_00169828__generated, manual_script_camera_get_view, SEEDS, &|c, seed| {
            let mut r = Rng(0x0016_9828 ^ (seed as u64) << 9);
            scramble(c, &mut r);
            put_globals(c, &mut r);
            put_script_camera(c, &mut r, seed);
            c.g[A0][0] = sx(OUT);
            c.g[A1][0] = sx(CAM);
        });
    }

    #[test]
    fn target_script_camera_get_view_matches() {
        check(ee::s02::f_00169f88__generated, target_script_camera_get_view, SEEDS, &|c, seed| {
            let mut r = Rng(0x0016_9f88 ^ (seed as u64) << 9);
            scramble(c, &mut r);
            put_globals(c, &mut r);
            put_script_camera(c, &mut r, seed);
            c.g[A0][0] = sx(OUT);
            c.g[A1][0] = sx(CAM);
        });
    }

    #[test]
    fn subject_script_camera_get_view_matches() {
        check(ee::s02::f_0016a868__generated, subject_script_camera_get_view, SEEDS, &|c, seed| {
            let mut r = Rng(0x0016_a868 ^ (seed as u64) << 9);
            scramble(c, &mut r);
            put_globals(c, &mut r);
            put_script_camera(c, &mut r, seed);
            c.g[A0][0] = sx(OUT);
            c.g[A1][0] = sx(CAM);
        });
    }

    #[test]
    fn algorithmic_script_camera_get_view_matches() {
        check(ee::s02::f_0016b7b8__generated, algorithmic_script_camera_get_view, SEEDS, &|c, seed| {
            let mut r = Rng(0x0016_b7b8 ^ (seed as u64) << 9);
            scramble(c, &mut r);
            put_globals(c, &mut r);
            put_script_camera(c, &mut r, seed);
            c.g[A0][0] = sx(OUT);
            c.g[A1][0] = sx(CAM);
        });
    }

    /// A line bound: translation, three angles, scale, start and end points.
    fn put_line_bound(c: &mut Cpu, r: &mut Rng, seed: u32) {
        for k in 0..0x12u32 {
            c.bus.write32(BOUND + 4 * k, r.f32(3.0));
        }
        for k in 0..3u32 {
            let s = if seed % 5 == 0 && k == 1 { 0 } else { r.f32(2.0) };
            c.bus.write32(BOUND + line_bound::SCALE + 4 * k, s);
        }
    }

    #[test]
    fn camera_line_bound_world_start_matches() {
        check(ee::s02::f_00170240__generated, camera_line_bound_world_start, SEEDS, &|c, seed| {
            let mut r = Rng(0x0017_0240 ^ (seed as u64) << 7);
            scramble(c, &mut r);
            put_globals(c, &mut r);
            put_line_bound(c, &mut r, seed);
            c.g[A0][0] = sx(OUT);
            c.g[A1][0] = sx(BOUND);
        });
    }

    #[test]
    fn camera_line_bound_world_end_matches() {
        check(ee::s02::f_001707b0__generated, camera_line_bound_world_end, SEEDS, &|c, seed| {
            let mut r = Rng(0x0017_07b0 ^ (seed as u64) << 7);
            scramble(c, &mut r);
            put_globals(c, &mut r);
            put_line_bound(c, &mut r, seed);
            c.g[A0][0] = sx(OUT);
            c.g[A1][0] = sx(BOUND);
        });
    }

    #[test]
    fn shp_decode_dxt_color_block_matches() {
        check(ee::s13::f_003b2e78__generated, shp_decode_dxt_color_block, SEEDS, &|c, seed| {
            let mut r = Rng(0x003b_2e78 ^ (seed as u64) << 11);
            scramble(c, &mut r);
            // Both endpoint orders (four-colour mode and three-colour mode).
            let (a, b) = match seed % 4 {
                0 => (0x001f_u32, 0x7fff_u32),
                1 => (0x7fff, 0x001f),
                _ => (r.next_u32() & 0xffff, r.next_u32() & 0xffff),
            };
            let idx = r.next_u32();
            c.bus.write64(BLOCK, a as u64 | (b as u64) << 16 | (idx as u64) << 32);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(0x40);
            c.g[A2][0] = sx(BLOCK);
        });
    }
}
