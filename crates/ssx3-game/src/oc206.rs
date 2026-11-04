//! Staging wave 201 (oc206): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size),
//! one function per writer. Each reproduces the generated body's registers,
//! cycles, memory and call order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0038_9cb8,
    irradiance_add_camera_relative_light,
    "Irradiance_AddCameraRelativeLight",
)];

// ---------------------------------------------------------------------------
// Irradiance_AddCameraRelativeLight (0x389cb8).

/// Light direction (Vec4) the sun's elevation is measured against.
const CRL_UP: u32 = 0x004f_f140;
/// Rotation axes of the azimuth and elevation rotations (Vec3 each).
const CRL_AXIS_1: u32 = 0x004f_f150;
const CRL_AXIS_2: u32 = 0x004f_f160;
/// A quadword shuffled through the stack around the two `Math_SinCos` calls.
const CRL_SWAP: u32 = 0x004f_f130;
/// The base orientation the angle rotations are applied to (4x4).
const CRL_BASE: u32 = 0x004f_f1a0;
/// The axis-swap matrix built once (4x4) and its "built" flag (`gp+0x28b4`).
const CRL_SWAP_MATRIX: u32 = 0x0050_0c60;
const CRL_SWAP_BUILT: i32 = 0x28b4;
/// The camera object (`gp+0x2a90`), its vtable (+0x10d8) and the
/// "get view matrix" slot (this-delta at +0x118, method at +0x11c).
const CRL_CAMERA: i32 = 0x2a90;
const CRL_CAMERA_VT: u32 = 0x10d8;
const CRL_VIEW_MATRIX_SLOT: u32 = 0x118;

/// `gp + off`.
#[inline(always)]
fn crl_gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `sp + off`.
#[inline(always)]
fn crl_sp(c: &Cpu, off: u32) -> u32 {
    sp(c).wrapping_add(off)
}

/// Load a 4x4 matrix into vf[base]..vf[base+3].
#[inline(always)]
fn crl_load_mat(c: &mut Cpu, base: usize, m: u32) {
    for i in 0..4 {
        c.lqc2(base + i, m.wrapping_add(0x10 * i as u32));
    }
}

/// Store vf[base]..vf[base+3] as a 4x4 matrix.
#[inline(always)]
fn crl_store_mat(c: &mut Cpu, base: usize, m: u32) {
    for i in 0..4 {
        c.sqc2(base + i, m.wrapping_add(0x10 * i as u32));
    }
}

/// A matrix copy through vf1-vf4.
#[inline(always)]
fn crl_copy_mat(c: &mut Cpu, dst: u32, src: u32) {
    crl_load_mat(c, 1, src);
    crl_store_mat(c, 1, dst);
}

/// vf[out] = vf4 * v.x + vf5 * v.y + vf6 * v.z + vf7 * v.w (v = vf[v]).
#[inline(always)]
fn crl_mat_vec(c: &mut Cpu, out: u8, v: u8) {
    use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
    c.vu0().mula(0b1111, 4, Src::Bc(v, X));
    c.vu0().madda(0b1111, 5, Src::Bc(v, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(v, Z));
    c.vu0().madd(0b1111, out, 7, Src::Bc(v, W));
}

/// `dst = a * b` (vf4-7 = a, vf8-11 = b, product rows in vf12-15).
#[inline(always)]
fn crl_mat_mul(c: &mut Cpu, dst: u32, a: u32, b: u32) {
    crl_load_mat(c, 4, a);
    crl_load_mat(c, 8, b);
    for i in 0..4 {
        crl_mat_vec(c, 12 + i, 8 + i);
    }
    crl_store_mat(c, 12, dst);
}

/// vf4.x = the four-lane dot product of vf[a] and vf[b] (vf6.x = 1).
#[inline(always)]
fn crl_dot4(c: &mut Cpu, a: u8, b: u8) {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, a, Src::Vec(b));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// In-place transpose of the 4x4 matrix at `m` through GPRs: rows in
/// r+3, r+2, r+1, r; interleaves in r-1..r-4 (`pextlw/pextuw/pcpyld/pcpyud`).
#[inline(always)]
fn crl_transpose(c: &mut Cpu, m: u32, r: usize) {
    let rows = [r + 3, r + 2, r + 1, r];
    let t = [r - 1, r - 2, r - 3, r - 4];
    for (i, &row) in rows.iter().enumerate() {
        c.lq(row, m.wrapping_add(0x10 * i as u32));
    }
    c.pextlw(t[0], rows[1], rows[0]);
    c.pextuw(t[1], rows[1], rows[0]);
    c.pextlw(t[2], rows[3], rows[2]);
    c.pextuw(t[3], rows[3], rows[2]);
    c.pcpyld(rows[0], t[2], t[0]);
    c.pcpyud(rows[1], t[0], t[2]);
    c.pcpyld(rows[2], t[3], t[1]);
    c.pcpyud(rows[3], t[1], t[3]);
    for (i, &row) in rows.iter().enumerate() {
        c.sq(row, m.wrapping_add(0x10 * i as u32));
    }
}

/// The axis-angle rotation (Rodrigues) from `Math_SinCos`'s results at
/// `sp+sin_off` / `sp+cos_off` about the axis at `s1` (f20 = 1), written
/// as rows into the matrix at sp+0x120 (w lanes 0); sp+0x150 gets the
/// quadword from sp+0x1a0 (the row (0,0,0,1) stored first is overwritten).
fn crl_axis_rotation(c: &mut Cpu, sin_off: u32, cos_off: u32) {
    use crate::ps2f::FpuF32;
    let axis = r32(c, S1);
    let one = FpuF32(c.f[20]);
    c.lwc1(11, crl_sp(c, cos_off));
    let cs = FpuF32(c.f[11]);
    c.lwc1(3, axis.wrapping_add(8));
    let z = FpuF32(c.f[3]);
    c.lwc1(0, axis);
    let x = FpuF32(c.f[0]);
    c.lwc1(7, axis.wrapping_add(4));
    let y = FpuF32(c.f[7]);
    c.lwc1(8, crl_sp(c, sin_off));
    let sn = FpuF32(c.f[8]);
    let t = one - cs;
    let (tz, tx, ty) = (t * z, t * x, t * y);
    let (sz, sx_, sy) = (sn * z, sn * x, sn * y);
    c.lq(V0, crl_sp(c, 0x1a0));
    // Row 3 = (0, 0, 0, 1), then overwritten by the quadword just loaded.
    c.sw(crl_sp(c, 0x150), 0);
    c.sw(crl_sp(c, 0x154), 0);
    c.sw(crl_sp(c, 0x158), 0);
    c.swc1(20, crl_sp(c, 0x15c));
    c.sq(V0, crl_sp(c, 0x150));
    c.sw(crl_sp(c, 0x12c), 0);
    c.sw(crl_sp(c, 0x13c), 0);
    c.sw(crl_sp(c, 0x14c), 0);
    let m01 = ty * x - sz; // f0
    let m02 = tz * x + sy; // f5
    let m00 = tx * x + cs; // f6
    let m10 = tx * y + sz; // f4
    let m11 = ty * y + cs; // f1
    let m12 = tz * y - sx_; // f2
    let m20 = tx * z - sy; // f10
    let m21 = ty * z + sx_; // f3
    let m22 = tz * z + cs; // f9
    for (f, v) in [
        (0, m01),
        (1, m11),
        (2, m12),
        (3, m21),
        (4, m10),
        (5, m02),
        (6, m00),
        (8, sy),
        (9, m22),
        (10, m20),
        (12, sx_),
        (13, sz),
    ] {
        c.f[f] = v.0;
    }
    for (f, off) in [
        (5, 0x128),
        (0, 0x124),
        (2, 0x138),
        (9, 0x148),
        (6, 0x120),
        (4, 0x130),
        (10, 0x140),
        (1, 0x134),
        (3, 0x144),
    ] {
        let a = crl_sp(c, off);
        c.swc1(f, a);
    }
}

/// `void Irradiance_AddCameraRelativeLight(Irradiance* irr, const Vec4*
/// sunPos, float scale)` (0x389cb8): add the sun as a zonal SH light in
/// camera space. Builds a z-axis lobe at sp (w lanes of the 10 entries),
/// gets the camera's view matrix (virtual call), turns the sun position
/// into a camera-relative unit direction, derives its azimuth
/// (`Math_Atan`) and elevation (`Math_Asin` of the cosine against
/// `CRL_UP`, made an angle from the pole; mirrored for negative y),
/// rotates the base orientation by both angles (`Math_SinCos` + two
/// axis-angle rotations), transposes, applies the axis-swap matrix
/// (built on first use), rotates the lobe by the 3x3 part
/// (`Irradiance_Rotate`, order 3) and adds it to `irr` scaled by
/// `scale * k` (`Irradiance_AddScaledW`).
pub fn irradiance_add_camera_relative_light(c: &mut Cpu) {
    use ssx3_ee_rt::vu0::{Src, X};
    guest(c, |c| {
        // Prologue: frame 0x2e0, s0-s6 / ra / f20-f24 saved, the lobe's w
        // lanes (0, 0x10 ... 0x90).
        c.cycles += 128;
        let frame = Frame::open(c, 0x2e0);
        c.g[V0][0] = 0x004a_0000;
        frame.save(c, S0, 0x2a0);
        c.lwc1(3, 0x004a_43c0);
        c.g[S0][0] = c.g[A1][0];
        c.lwc1(2, crl_gp(c, -0x2700));
        c.lwc1(1, crl_gp(c, -0x26fc));
        c.lwc1(0, crl_gp(c, -0x26f8));
        frame.save(c, S6, 0x240);
        frame.save_fpr(c, 24, 0x2d0);
        c.g[S6][0] = c.g[A0][0];
        frame.save(c, S1, 0x290);
        c.f[24] = c.f[12];
        frame.save(c, S2, 0x280);
        frame.save(c, S3, 0x270);
        frame.save(c, S4, 0x260);
        frame.save(c, S5, 0x250);
        frame.save_ra(c, 0x230);
        frame.save_fpr(c, 23, 0x2c8);
        frame.save_fpr(c, 22, 0x2c0);
        frame.save_fpr(c, 21, 0x2b8);
        frame.save_fpr(c, 20, 0x2b0);
        for (f, off) in [(2, 0x0c), (3, 0x2c), (1, 0x3c), (0, 0x7c)] {
            let a = crl_sp(c, off);
            c.swc1(f, a);
        }
        for off in [0x1c, 0x4c, 0x5c, 0x6c, 0x8c, 0x9c] {
            let a = crl_sp(c, off);
            c.sw(a, 0);
        }

        // view = camera->GetViewMatrix() (virtual, this-adjusted).
        c.g[A1][0] = c.lw(crl_gp(c, CRL_CAMERA));
        c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(CRL_CAMERA_VT));
        c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(CRL_VIEW_MATRIX_SLOT));
        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(CRL_VIEW_MATRIX_SLOT + 4));
        let method = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, method, 0x0038_9d50)?;

        // Camera position: -(view^T * view.row3), w = 1 (sp+0xe0).
        let view = sp(c).wrapping_add(0xa0);
        c.g[V1][0] = sx(view);
        crl_copy_mat(c, view, r32(c, V0));
        c.lq(V0, crl_sp(c, 0xd0));
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.f[21] = 0xbf80_0000;
        c.sq(V0, crl_sp(c, 0xf0));
        c.mfc1(A0, 21);
        c.lqc2(4, crl_sp(c, 0xf0));
        c.qmtc2(A0, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, crl_sp(c, 0x100));
        c.lq(V0, crl_sp(c, 0x100));
        c.sq(V0, crl_sp(c, 0xe0));
        crl_transpose(c, view, 8);
        c.lqc2(8, crl_sp(c, 0xe0));
        crl_load_mat(c, 4, view);
        crl_mat_vec(c, 12, 8);
        c.sqc2(12, crl_sp(c, 0x100));
        c.g[AT][0] = 0x3f80_0000;
        c.f[22] = 0x3f80_0000;
        c.lq(V0, crl_sp(c, 0x100));
        c.sq(V0, crl_sp(c, 0xe0));
        c.sq(V0, crl_sp(c, 0xf0));
        c.swc1(22, crl_sp(c, 0xec));

        // dir = normalize(sunPos - camera) (sp+0xf0); f12 = dir.z, then
        // dir.z = 0 and f20 = |dir.xy|; f12 = dir.z / |dir.xy|.
        c.lqc2(3, r32(c, S0));
        c.lqc2(4, crl_sp(c, 0xe0));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, crl_sp(c, 0x110));
        c.lq(V0, crl_sp(c, 0x110));
        c.sq(V0, crl_sp(c, 0x100));
        c.lqc2(3, crl_sp(c, 0x100));
        crl_dot4(c, 3, 3);
        c.vu0().rsqrt(0, 3, 4, 0);
        c.vu0().waitq();
        c.vu0().mul(0b1111, 5, 3, Src::Q);
        c.sqc2(5, crl_sp(c, 0x110));
        c.lq(V0, crl_sp(c, 0x110));
        c.sq(V0, crl_sp(c, 0xf0));
        c.lwc1(12, crl_sp(c, 0xf8));
        c.sw(crl_sp(c, 0xf8), 0);
        c.lqc2(3, crl_sp(c, 0xf0));
        crl_dot4(c, 3, 3);
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(V0, 22);
        c.f[20] = c.g[V0][0] as u32;
        c.div_s(12, 12, 20);
        call_ptr(c, 0x0031_c228, 0x0038_9ea4)?; // Math_Atan

        // f23 = -azimuth; f21 = the elevation angle (0 without a
        // horizontal component).
        c.f[12] = 0;
        c.c_lt_s(12, 20);
        let horizontal = c.fcc();
        c.neg_s(23, 0);
        if !horizontal {
            c.cycles += 2;
            c.f[21] = c.f[12];
            c.g[V0][0] = 0x0050_0000;
        } else {
            c.cycles += 25;
            c.g[A0][0] = 0x0050_0000;
            // f1 = |dir| * |up|.
            c.lqc2(3, crl_sp(c, 0xf0));
            c.lqc2(5, crl_sp(c, 0xf0));
            crl_dot4(c, 3, 5);
            c.qmfc2(V0, 4);
            c.mtc1(V0, 0);
            c.lqc2(3, CRL_UP);
            c.lqc2(5, CRL_UP);
            crl_dot4(c, 3, 5);
            c.qmfc2(V1, 4);
            c.mtc1(V1, 1);
            c.lwc1(2, crl_gp(c, -0x26f4));
            c.mul_s(0, 0, 1);
            c.sqrt_s(1, 0);
            c.c_lt_s(1, 2);
            if c.fcc() {
                c.cycles += 2;
                c.f[21] = c.f[12];
            } else {
                // cos = dir . up / f1, clamped to [-1 (f21), 1 (f22)];
                // f21 = k - asin(cos).
                c.cycles += 14;
                c.lqc2(3, crl_sp(c, 0xf0));
                c.lqc2(5, CRL_UP);
                crl_dot4(c, 3, 5);
                c.qmfc2(V0, 4);
                c.mtc1(V0, 0);
                c.div_s(0, 0, 1);
                c.c_le_s(21, 0);
                let above_min = c.fcc();
                c.f[12] = c.f[21];
                if above_min {
                    c.cycles += 1;
                    c.min_s(12, 0, 22);
                }
                c.cycles += 4;
                call_ptr(c, 0x0031_c128, 0x0038_9f68)?; // Math_Asin
                c.lwc1(1, crl_gp(c, -0x26f0));
                c.sub_s(21, 1, 0);
            }
            // Below the horizon (dir.y < 0): f21 = k - f21.
            c.cycles += 6;
            c.lwc1(1, crl_sp(c, 0xf4));
            c.f[0] = 0;
            c.c_lt_s(1, 0);
            let below = c.fcc();
            c.g[V0][0] = 0x0050_0000;
            if below {
                c.cycles += 3;
                c.lwc1(0, crl_gp(c, -0x26ec));
                c.sub_s(21, 0, 21);
            }
        }

        // orient (sp+0x120) = base with row 3 = base * camera (vf12).
        c.cycles += 234;
        let orient = sp(c).wrapping_add(0x120);
        c.g[S4][0] = sx(orient);
        c.g[V0][0] = sx(CRL_BASE);
        crl_copy_mat(c, orient, CRL_BASE);
        crl_load_mat(c, 4, orient);
        c.lqc2(8, crl_sp(c, 0xe0));
        crl_mat_vec(c, 12, 8);
        c.sqc2(12, orient.wrapping_add(0x30));

        // Rotate by the azimuth (f23) about the first axis.
        let prev = sp(c).wrapping_add(0x160);
        c.g[S0][0] = 0x0050_0000;
        c.neg_s(12, 23);
        c.g[S1][0] = sx(CRL_AXIS_1);
        c.g[S5][0] = sx(prev);
        c.g[S3][0] = c.g[S4][0];
        crl_copy_mat(c, prev, orient);
        c.g[S2][0] = 0x0050_0000;
        c.lq(V0, crl_sp(c, 0x190));
        c.g[S2][0] = sx(CRL_SWAP);
        c.g[A0][0] = sx(crl_sp(c, 0x220));
        c.lq(V1, CRL_SWAP);
        c.g[A1][0] = sx(crl_sp(c, 0x224));
        c.sq(V0, crl_sp(c, 0x1a0));
        c.sq(V1, crl_sp(c, 0x190));
        call_ptr(c, 0x0031_be50, 0x0038_a044)?; // Math_SinCos
        c.g[AT][0] = 0x3f80_0000;
        c.f[20] = 0x3f80_0000;
        crl_axis_rotation(c, 0x220, 0x224);
        crl_mat_mul(c, orient, orient, prev);

        // Rotate by the elevation (f21) about the second axis.
        c.g[S0][0] = 0x0050_0000;
        c.neg_s(12, 21);
        c.g[S1][0] = sx(CRL_AXIS_2);
        c.g[S3][0] = c.g[S4][0];
        crl_copy_mat(c, prev, orient);
        c.lq(V0, CRL_SWAP);
        c.g[A0][0] = sx(crl_sp(c, 0x228));
        c.lq(V1, crl_sp(c, 0x190));
        c.g[A1][0] = sx(crl_sp(c, 0x22c));
        c.sq(V0, crl_sp(c, 0x190));
        c.sq(V1, crl_sp(c, 0x1a0));
        call_ptr(c, 0x0031_be50, 0x0038_a1c8)?; // Math_SinCos
        crl_axis_rotation(c, 0x228, 0x22c);
        crl_mat_mul(c, orient, orient, prev);
        crl_transpose(c, orient, 7);

        // The axis-swap matrix rows (0,0,1,0), (-1,0,0,0), (0,1,0,0),
        // (0,0,0,1), built on first use.
        c.g[V0][0] = c.lw(crl_gp(c, CRL_SWAP_BUILT));
        let built = c.g[V0][0] != 0;
        c.g[A0][0] = 0x0050_0000;
        if !built {
            c.cycles += 22;
            c.f[0] = 0;
            c.g[V0][0] = sx(CRL_SWAP_MATRIX);
            c.g[AT][0] = 0xffff_ffff_bf80_0000;
            c.f[1] = 0xbf80_0000;
            c.g[V1][0] = 1;
            let m = CRL_SWAP_MATRIX;
            c.swc1(20, m + 0x3c);
            c.swc1(1, m + 0x10);
            c.swc1(0, m + 0x2c);
            c.sw(crl_gp(c, CRL_SWAP_BUILT), 1);
            for (f, off) in [
                (0, 0x00),
                (0, 0x20),
                (0, 0x30),
                (0, 0x04),
                (0, 0x14),
                (20, 0x24),
                (0, 0x34),
                (20, 0x08),
                (0, 0x18),
                (0, 0x28),
                (0, 0x38),
                (0, 0x0c),
                (0, 0x1c),
            ] {
                c.swc1(f, m + off);
            }
        }

        // rot (sp+0x1b0, copied to sp+0x160) = swap * orient; its 3x3 part
        // packed at sp+0x1f0 and copied to sp+0x1a0.
        c.cycles += 100;
        let rot = sp(c).wrapping_add(0x1b0);
        c.g[V1][0] = sx(CRL_SWAP_MATRIX);
        c.g[V0][0] = sx(rot);
        c.g[A0][0] = c.g[S5][0];
        crl_mat_mul(c, rot, CRL_SWAP_MATRIX, orient);
        crl_copy_mat(c, prev, rot);
        for (f, off) in [
            (8, 0x160),
            (7, 0x164),
            (0, 0x168),
            (1, 0x170),
            (2, 0x174),
            (3, 0x178),
            (4, 0x180),
            (5, 0x184),
            (6, 0x188),
        ] {
            let a = crl_sp(c, off);
            c.lwc1(f, a);
        }
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = sx(crl_sp(c, 0x1a0));
        c.g[A2][0] = 3;
        for (i, f) in [8, 7, 0, 1, 2, 3, 4, 5, 6].into_iter().enumerate() {
            let a = crl_sp(c, 0x1f0 + 4 * i as u32);
            c.swc1(f, a);
        }
        // 36-byte struct copy (ldl/ldr, sdl/sdr on aligned doublewords).
        for (reg, off) in [(V0, 0x00), (V1, 0x08), (T0, 0x10), (T1, 0x18)] {
            c.g[reg][0] = c.ld(crl_sp(c, 0x1f0 + off));
        }
        for (reg, off) in [(V0, 0x00), (V1, 0x08), (T0, 0x10), (T1, 0x18)] {
            let a = crl_sp(c, 0x1a0 + off);
            c.sd(a, c.g[reg][0]);
        }
        c.g[V0][0] = c.lw(crl_sp(c, 0x210));
        c.sw(crl_sp(c, 0x1c0), c.g[V0][0]);
        c.g[A3][0] = 3;
        Callee {
            addr: 0x0038_9840,
            f: ee::s12::Irradiance_Rotate,
        }
        .call(c, 0x0038_a4dc)?;

        // irr += lobe * (scale * k).
        c.lwc1(12, crl_gp(c, -0x26e8));
        c.g[A0][0] = c.g[S6][0];
        c.g[A1][0] = c.g[SP][0];
        c.mul_s(12, 24, 12);
        Callee {
            addr: 0x0038_9620,
            f: ee::s12::Irradiance_AddScaledW,
        }
        .call(c, 0x0038_a4f0)?;

        frame.restore(c, S0, 0x2a0);
        frame.restore(c, S1, 0x290);
        frame.restore(c, S2, 0x280);
        frame.restore(c, S3, 0x270);
        frame.restore(c, S4, 0x260);
        frame.restore(c, S5, 0x250);
        frame.restore(c, S6, 0x240);
        frame.restore_ra(c, 0x230);
        frame.restore_fpr(c, 24, 0x2d0);
        frame.restore_fpr(c, 23, 0x2c8);
        frame.restore_fpr(c, 22, 0x2c0);
        frame.restore_fpr(c, 21, 0x2b8);
        frame.restore_fpr(c, 20, 0x2b0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `Irradiance_AddCameraRelativeLight`: the camera's view matrix comes
    /// from a fake virtual method (call 0, unwinding on some seeds);
    /// `Math_Atan/Asin/SinCos`, `Irradiance_Rotate` and
    /// `Irradiance_AddScaledW` run for real on the ELF image's constants.
    /// Branches: a zero view and a sun straight above (no horizontal
    /// component), a zero `up` vector (degenerate length), a huge `up`
    /// (cosine outside [-1, 1]: both clamps), dir.y of either sign, and the
    /// axis-swap matrix built or not.
    #[test]
    fn irradiance_add_camera_relative_light_matches() {
        use crate::difftest::fakes::{machine, FAKE, LOG_RESULTS};
        use crate::difftest::{check, elf_image, fill, put_vec, Rng, DATA, GAME_GP, SP as STACK};
        const VIEW: u32 = DATA + 0x1000;
        const CAMERA: u32 = DATA + 0x2000;
        const VTABLE: u32 = DATA + 0x3000;
        const SUN: u32 = DATA + 0x4000;
        const IRR: u32 = DATA + 0x5000;
        fn vec(r: &mut Rng, range: f32) -> [u32; 4] {
            [r.f32(range), r.f32(range), r.f32(range), r.f32(range)]
        }
        fn matrix(c: &mut Cpu, r: &mut Rng, at: u32, range: f32) {
            for i in 0..4 {
                let v = vec(r, range);
                put_vec(c, at + 0x10 * i, v);
            }
        }
        check(
            ee::s12::f_00389cb8__generated,
            irradiance_add_camera_relative_light,
            200,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x0038_9cb8, 1);
                c.pc = 0x0038_9cb8;
                // Stale stack under the frame (read before written).
                fill(c, &mut r, STACK - 0x300, 0xc0);
                // camera->GetViewMatrix() through the vtable, returning VIEW.
                c.bus.write32(GAME_GP + CRL_CAMERA as u32, CAMERA);
                c.bus.write32(CAMERA + CRL_CAMERA_VT, VTABLE);
                let delta = (r.next_u32() as i16 & !3) as u16;
                c.bus.write16(VTABLE + CRL_VIEW_MATRIX_SLOT, delta);
                c.bus.write32(VTABLE + CRL_VIEW_MATRIX_SLOT + 4, FAKE);
                c.bus.write32(LOG_RESULTS, VIEW);
                let overhead = seed % 5 == 0;
                if overhead {
                    for i in 0..4 {
                        put_vec(c, VIEW + 0x10 * i, [0; 4]);
                    }
                } else {
                    matrix(c, &mut r, VIEW, 2.0);
                }
                let mut sun = vec(&mut r, 100.0);
                if overhead {
                    // dir = (0, 0, z, 0): no horizontal part (w lane too).
                    sun[0] = 0;
                    sun[1] = 0;
                    sun[3] = 0x3f80_0000;
                }
                put_vec(c, SUN, sun);
                let up = match seed % 7 {
                    1 => [0; 4],
                    2 | 4 => vec(&mut r, 1e25),
                    _ => vec(&mut r, 1.0),
                };
                put_vec(c, CRL_UP, up);
                let a1 = vec(&mut r, 1.0);
                put_vec(c, CRL_AXIS_1, a1);
                let a2 = vec(&mut r, 1.0);
                put_vec(c, CRL_AXIS_2, a2);
                let q = vec(&mut r, 1.0);
                put_vec(c, CRL_SWAP, q);
                matrix(c, &mut r, CRL_BASE, 2.0);
                matrix(c, &mut r, CRL_SWAP_MATRIX, 2.0);
                c.bus.write32(GAME_GP + CRL_SWAP_BUILT as u32, seed & 1);
                for i in 0..40 {
                    let v = r.f32(2.0);
                    c.bus.write32(IRR + 4 * i, v);
                }
                c.g[A0][0] = sx(IRR);
                c.g[A1][0] = sx(SUN);
                c.f[12] = r.f32(4.0);
            },
        );
    }
}
