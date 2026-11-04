//! Staging wave 201 (oc208): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size),
//! one function per writer. Each reproduces the generated body's registers,
//! cycles, memory and call order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::Flow;
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0035_fc20,
        uv_scroll_modifier_get_matrix,
        "UVScrollModifier_GetMatrix",
    ),
    (
        0x002b_7908,
        c_world_sound_process_emitter,
        "cWorldSound_ProcessEmitter",
    ),
];

// ---------------------------------------------------------------------------
// UVScrollModifier_GetMatrix (0x35fc20)

/// The UV scroll modifier (`railmodifier.cpp`): a texture-coordinate
/// rotation about an axis plus a 2D translation.
mod uv_scroll {
    /// Rotation angle (radians); 0 means no rotation.
    pub const ANGLE: u32 = 0x10;
    /// Rotation axis x, y, z.
    pub const AXIS_X: u32 = 0x20;
    pub const AXIS_Y: u32 = 0x24;
    pub const AXIS_Z: u32 = 0x28;
    /// Translation (u, v), written into the matrix's last row.
    pub const TRANS_U: u32 = 0x30;
    pub const TRANS_V: u32 = 0x34;
}

/// The identity matrix in `.rodata` (0x4ff1a0) copied when there is no
/// rotation.
const IDENTITY_MATRIX: u32 = 0x004f_f1a0;

/// `Math_SinCos(float angle /* f12 */, float* sin, float* cos)`.
const MATH_SIN_COS: Callee = Callee {
    addr: 0x0031_be50,
    f: ee::s10::f_0031be50,
};

/// `float* UVScrollModifier_GetMatrix(UVScrollModifier* self, float m[4][4])`
/// (0x35fc20), hot during races (rail/surface UV animation): builds the texture
/// matrix of a scrolling UV modifier. With a non-zero angle, `m` is the
/// rotation by `angle` about the axis (the Rodrigues form
/// `t*a*a^T + c*I + s*[a]x` with `t = 1 - c`; rows 0..2, column 3 zero,
/// `m[3][3] = 1`); with angle 0 it is the identity (a VU0 quadword copy).
/// Then `m[3][0] = u`, `m[3][1] = v` (the UV scroll offset). Returns `m`.
pub fn uv_scroll_modifier_get_matrix(c: &mut Cpu) {
    use crate::guest::Addr;
    use crate::ps2f::FpuF32;
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save_fpr(c, 20, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        let this = Addr(r32(c, S1));
        let zero = FpuF32(0);
        c.f[20] = zero.0;
        let angle = this.off(uv_scroll::ANGLE).read_fpu(c);
        c.f[12] = angle.0;
        let no_rotation = angle.eq_s(zero);
        crate::abi::set_fcc(c, no_rotation);
        c.g[S0][0] = c.g[A1][0];
        let m = Addr(r32(c, S0));

        if no_rotation {
            c.cycles += 10;
            c.g[V0][0] = sx(IDENTITY_MATRIX);
            for row in 0..4u32 {
                c.lqc2(1 + row as usize, IDENTITY_MATRIX + row * 0x10);
            }
            for row in 0..4u32 {
                c.sqc2(1 + row as usize, m.0.wrapping_add(row * 0x10));
            }
        } else {
            c.cycles += 60;
            // Math_SinCos(angle, &sin /* sp+0 */, &cos /* sp+4 */).
            let s = sp(c);
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = c.g[SP][0] | 0x4;
            MATH_SIN_COS.call(c, 0x0035_fc5c)?;
            let ax_at = this.off(uv_scroll::AXIS_X);
            let ay_at = this.off(uv_scroll::AXIS_Y);
            let az_at = this.off(uv_scroll::AXIS_Z);

            let cos = Addr(s.wrapping_add(4)).read_fpu(c);
            c.g[AT][0] = 0x3f80_0000;
            let one = FpuF32(0x3f80_0000);
            let ax = ax_at.read_fpu(c);
            let t = one - cos;
            let sin = Addr(s).read_fpu(c);
            let ay = ay_at.read_fpu(c);
            let az = az_at.read_fpu(c);
            let tx = t * ax;
            let sy = sin * ay;
            let sz = sin * az;
            let m00 = tx * ax + cos;
            let sx_ = sin * ax;
            let ty = t * ay;
            let tz = t * az;
            m.write_fpu(c, m00);
            // The axis is re-read before each use (m may alias self).
            let m10 = tx * ay_at.read_fpu(c) + sz;
            m.off(0x10).write_fpu(c, m10);
            let az1 = az_at.read_fpu(c);
            m.off(0x30).write_fpu(c, zero);
            let m20 = tx * az1 - sy;
            m.off(0x20).write_fpu(c, m20);
            let m01 = ty * ax_at.read_fpu(c) - sz;
            m.off(0x04).write_fpu(c, m01);
            let m11 = ty * ay_at.read_fpu(c) + cos;
            m.off(0x14).write_fpu(c, m11);
            let az2 = az_at.read_fpu(c);
            m.off(0x34).write_fpu(c, zero);
            let m21 = ty * az2 + sx_;
            m.off(0x24).write_fpu(c, m21);
            let m02 = tz * ax_at.read_fpu(c) + sy;
            m.off(0x08).write_fpu(c, m02);
            let m12 = tz * ay_at.read_fpu(c) - sx_;
            m.off(0x18).write_fpu(c, m12);
            let az3 = az_at.read_fpu(c);
            m.off(0x2c).write_fpu(c, zero);
            let tz_az = tz * az3;
            m.off(0x3c).write_fpu(c, one);
            m.off(0x38).write_fpu(c, zero);
            m.off(0x0c).write_fpu(c, zero);
            let m22 = tz_az + cos;
            m.off(0x1c).write_fpu(c, zero);
            m.off(0x28).write_fpu(c, m22);

            // FPU registers as the original leaves them.
            c.f[0] = az3.0;
            c.f[1] = m12.0;
            c.f[2] = m20.0;
            c.f[3] = m21.0;
            c.f[4] = m22.0;
            c.f[5] = sx_.0;
            c.f[6] = az.0;
            c.f[7] = cos.0;
            c.f[8] = sz.0;
            c.f[9] = sy.0;
            c.f[10] = one.0;
        }

        // Translation into the last row, then return m.
        c.cycles += 11;
        let u = this.off(uv_scroll::TRANS_U).read_fpu(c);
        c.f[1] = u.0;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0x10);
        m.off(0x30).write_fpu(c, u);
        frame.restore_fpr(c, 20, 0x40);
        let v = this.off(uv_scroll::TRANS_V).read_fpu(c);
        c.f[0] = v.0;
        frame.restore(c, S1, 0x20);
        m.off(0x34).write_fpu(c, v);
        frame.restore(c, S0, 0x30);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cWorldSound_ProcessEmitter (0x2b7908)

/// A world-sound emitter's data block (`worldtriggermanager.cpp`): a header
/// `{ skip_words, count }`, `skip_words` words, then `count` records of four
/// shapes. Every record starts with its kind, a trigger id and an offset from
/// the owning object's position.
mod emitter {
    /// Header: words to skip, then the record count.
    pub const SKIP_WORDS: u32 = 0x0;
    pub const COUNT: u32 = 0x4;
    pub const RECORDS: u32 = 0x8;
    /// Record: kind (0 sphere, 1 ellipsoid, 2 cone, 3 zone).
    pub const KIND: u32 = 0x00;
    /// Trigger id handed to `WorldTriggerManager_AddTrigger`.
    pub const ID: u32 = 0x04;
    /// Offset x, y, z from the object's position.
    pub const OFFSET: u32 = 0x08;
    /// Sphere/cone/zone radius; the ellipsoid's radius along its axis.
    pub const RADIUS: u32 = 0x14;
    /// Sphere: falloff curve.
    pub const SPHERE_CURVE: u32 = 0x18;
    /// Ellipsoid: radii across the axis (0 = unbounded), axis, curve.
    pub const ELL_RADIUS_N: u32 = 0x18;
    pub const ELL_RADIUS_M: u32 = 0x1c;
    pub const ELL_AXIS: u32 = 0x20;
    pub const ELL_CURVE: u32 = 0x2c;
    /// Cone: direction, distance curve, cosine of the half-angle, angle curve.
    pub const CONE_DIR: u32 = 0x18;
    pub const CONE_DIST_CURVE: u32 = 0x24;
    pub const CONE_COS_LIMIT: u32 = 0x28;
    pub const CONE_ANGLE_CURVE: u32 = 0x2c;
    /// Record sizes by kind.
    pub const SIZE_SPHERE: u32 = 0x1c;
    pub const SIZE_ELLIPSOID: u32 = 0x30;
    pub const SIZE_CONE: u32 = 0x30;
    pub const SIZE_ZONE: u32 = 0x18;
}

/// The emitter source (a3): the placed object and the data block.
const EMITTER_OBJECT: u32 = 0xc;
const EMITTER_DATA: u32 = 0x10;
/// The object's world position (x, y, z).
const OBJECT_POS: u32 = 0x40;

/// `float WorldSound_ComputeFalloff(int curve /* f12 as float */, float t /* f13 */)`.
const WORLD_SOUND_COMPUTE_FALLOFF: Callee = Callee {
    addr: 0x002b_82b8,
    f: ee::s08::f_002b82b8,
};
/// `WorldTriggerManager_AddTrigger(mgr, object, index, a3, id, flag, Vec3* pos)`.
const WORLD_TRIGGER_MANAGER_ADD_TRIGGER: Callee = Callee {
    addr: 0x002b_5c68,
    f: ee::s08::f_002b5c68,
};
/// `acosf` (called through the function table).
const ACOSF: u32 = 0x0031_c128;

/// `.lit4` constants (gp-relative): pi/2 and the zone's full-volume distance.
const GP_HALF_PI: u32 = 0x453c;
const GP_ZONE_NEAR: u32 = 0x4538;
/// The world trigger manager pointer (gp-relative).
const GP_TRIGGER_MANAGER: u32 = 0x21e4;

/// A float on the frame.
fn pe_spf(c: &mut Cpu, off: u32) -> FpuF32 {
    Addr(sp(c).wrapping_add(off)).read_fpu(c)
}

/// Store a float on the frame.
fn pe_set_spf(c: &mut Cpu, off: u32, v: FpuF32) {
    Addr(sp(c).wrapping_add(off)).write_fpu(c, v)
}

/// A Vec3 on the frame.
fn pe_spv3(c: &mut Cpu, off: u32) -> [FpuF32; 3] {
    [pe_spf(c, off), pe_spf(c, off + 4), pe_spf(c, off + 8)]
}

/// Store a Vec3 on the frame.
fn pe_set_spv3(c: &mut Cpu, off: u32, v: [FpuF32; 3]) {
    for (i, x) in v.iter().enumerate() {
        pe_set_spf(c, off + 4 * i as u32, *x);
    }
}

/// A Vec3 in memory.
fn pe_v3(c: &mut Cpu, at: u32) -> [FpuF32; 3] {
    [
        Addr(at).read_fpu(c),
        Addr(at.wrapping_add(4)).read_fpu(c),
        Addr(at.wrapping_add(8)).read_fpu(c),
    ]
}

/// FPU registers as the original leaves them.
fn pe_set_f(c: &mut Cpu, regs: &[(usize, FpuF32)]) {
    for &(r, v) in regs {
        c.f[r] = v.0;
    }
}

/// gcc's 12-byte struct copy on the frame: `ldl/ldr` of the first 8 bytes
/// into `reg`, `lw` of the last word into `tmp`, `sdl/sdr` + `sw`.
fn pe_copy_vec3(c: &mut Cpu, reg: usize, tmp: usize, src: u32, dst: u32) {
    let s = sp(c);
    c.ldl(reg, s.wrapping_add(src + 7));
    c.ldr(reg, s.wrapping_add(src));
    c.g[tmp][0] = c.lw(s.wrapping_add(src + 8));
    c.sdl(reg, s.wrapping_add(dst + 7));
    c.sdr(reg, s.wrapping_add(dst));
    c.sw(s.wrapping_add(dst + 8), c.g[tmp][0]);
}

/// The record's world position: object position (frame +0x00) + offset.
/// Leaves the offset at +0x50, the position at +0x40 and its copy (through
/// `reg`/`tmp`) at +0x20; f0..f2 = position, f3..f5 = offset.
fn pe_record_position(c: &mut Cpu, rec: u32, reg: usize, tmp: usize) -> [FpuF32; 3] {
    let off = pe_v3(c, rec.wrapping_add(emitter::OFFSET));
    let base = pe_spv3(c, 0);
    let pos = [base[0] + off[0], base[1] + off[1], base[2] + off[2]];
    pe_set_spv3(c, 0x50, off);
    pe_set_spv3(c, 0x40, pos);
    pe_copy_vec3(c, reg, tmp, 0x40, 0x20);
    pe_set_f(
        c,
        &[
            (0, pos[0]),
            (1, pos[1]),
            (2, pos[2]),
            (3, off[0]),
            (4, off[1]),
            (5, off[2]),
        ],
    );
    pos
}

/// listener - position on VU0: the position (from +0x20) goes to `at` with
/// w = 0, the difference to `at + 0x10` and (through v0, `lq`/`sq`) to +0x40.
fn pe_listener_delta(c: &mut Cpu, at: u32) -> [FpuF32; 3] {
    let s = sp(c);
    c.sw(s.wrapping_add(at + 0xc), 0);
    let pos = pe_spv3(c, 0x20);
    pe_set_spv3(c, at, pos);
    c.lqc2(3, r32(c, S3));
    c.lqc2(4, s.wrapping_add(at));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, s.wrapping_add(at + 0x10));
    c.lq(V0, s.wrapping_add(at + 0x10));
    c.sq(V0, s.wrapping_add(0x40));
    pe_spv3(c, 0x40)
}

/// Register the trigger: `AddTrigger(*gp[0x21e4], source->object, index /* s1 */,
/// a2 of the caller, id /* t0, set by the caller */, flag, pos)`.
fn pe_add_trigger(c: &mut Cpu, pos: u64, flag: u64, ret: u32) -> Flow {
    let gp = r32(c, GP);
    c.g[A0][0] = Addr(gp.wrapping_add(GP_TRIGGER_MANAGER)).lw(c);
    c.g[T2][0] = pos;
    c.g[A1][0] = Addr(r32(c, S4).wrapping_add(EMITTER_OBJECT)).lw(c);
    c.g[A2][0] = c.g[S1][0];
    c.g[A3][0] = c.g[S5][0];
    c.g[T1][0] = flag;
    WORLD_TRIGGER_MANAGER_ADD_TRIGGER.call(c, ret)
}

/// The shared tail of the sphere and cone (0x2b8004): register with flag 1 at
/// the position copy in s2, next record.
fn pe_add_trigger_common(c: &mut Cpu) -> Flow {
    c.cycles += 9;
    let pos = c.g[S2][0];
    pe_add_trigger(c, pos, 1, 0x002b_8020)?;
    pe_next_record(c);
    Ok(())
}

/// `s1++`.
fn pe_next_record(c: &mut Cpu) {
    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
}

/// Kind 0 (0x2b7a78): a sphere; volume by the distance over the radius.
fn pe_sphere(c: &mut Cpu, rec: u32) -> Flow {
    c.cycles += 52;
    c.g[S2][0] = sx(sp(c).wrapping_add(0x20));
    pe_record_position(c, rec, V0, V1);
    let d = pe_listener_delta(c, 0x60);
    pe_set_spv3(c, 0x30, d);
    let (dx2, dy2, dz2) = (d[0] * d[0], d[1] * d[1], d[2] * d[2]);
    let dist = (dx2 + dy2 + dz2).sqrt();
    let radius = Addr(rec.wrapping_add(emitter::RADIUS)).read_fpu(c);
    let adist = dist.abs();
    let inside = adist.lt(radius);
    crate::abi::set_fcc(c, inside);
    pe_set_f(
        c,
        &[
            (0, dist),
            (1, radius),
            (2, d[2]),
            (3, d[1]),
            (4, dy2),
            (5, dz2),
            (8, adist),
        ],
    );
    if !inside {
        pe_next_record(c);
        return Ok(());
    }
    c.cycles += 7;
    c.f[13] = adist.div(radius).0;
    c.f[12] = Addr(rec.wrapping_add(emitter::SPHERE_CURVE)).read_u32(c);
    c.g[A0][0] = c.g[S6][0];
    WORLD_SOUND_COMPUTE_FALLOFF.call(c, 0x002b_7b58)?;
    c.g[T0][0] = Addr(rec.wrapping_add(emitter::ID)).lw(c);
    c.f[12] = c.f[0];
    pe_add_trigger_common(c)
}

/// One axis term of the ellipsoid test: `(dot / radius)^2`, 0 for an
/// unbounded (zero) radius.
fn pe_axis_term(dot: FpuF32, radius: FpuF32, zero: FpuF32) -> Option<FpuF32> {
    if radius.eq_s(zero) {
        None
    } else {
        let dot2 = dot * dot;
        let r2 = radius * radius;
        Some(dot2.div(r2))
    }
}

/// Kind 1 (0x2b7b64): an ellipsoid around `axis` (radii along the axis and
/// across it, in the frame `n = normalize(z x axis)`, `m = axis x n`); volume
/// by the ellipsoid distance.
fn pe_ellipsoid(c: &mut Cpu, rec: u32) -> Flow {
    c.cycles += 161;
    let one = FpuF32(c.f[23]);
    let zero = FpuF32(c.f[24]);
    pe_record_position(c, rec, A2, A3);
    let d = pe_listener_delta(c, 0x50);
    pe_set_spv3(c, 0x30, d);
    // The axis, to +0x40.
    let a = pe_v3(c, rec.wrapping_add(emitter::ELL_AXIS));
    pe_set_spv3(c, 0x90, a);
    pe_copy_vec3(c, A0, A1, 0x90, 0x40);
    // up = (0, 0, 1), to +0x80.
    c.sw(sp(c).wrapping_add(0x90), 0);
    c.sw(sp(c).wrapping_add(0x94), 0);
    pe_set_spf(c, 0x98, one);
    pe_copy_vec3(c, V0, V1, 0x90, 0x80);
    let up = pe_spv3(c, 0x80);
    // b = up x axis, to +0x50.
    let b = [
        up[1] * a[2] - up[2] * a[1],
        up[2] * a[0] - up[0] * a[2],
        up[0] * a[1] - up[1] * a[0],
    ];
    pe_set_spv3(c, 0x90, b);
    pe_copy_vec3(c, A2, A3, 0x90, 0x50);
    // n = b / |b|, to +0x50.
    let len = (b[0] * b[0] + b[1] * b[1] + b[2] * b[2]).sqrt();
    let inv = one.div(len);
    let n = [b[0] * inv, b[1] * inv, b[2] * inv];
    pe_set_spv3(c, 0x90, n);
    pe_copy_vec3(c, A0, A1, 0x90, 0x50);
    // m = axis x n, to +0x60.
    let m = [
        a[1] * n[2] - a[2] * n[1],
        a[2] * n[0] - a[0] * n[2],
        a[0] * n[1] - a[1] * n[0],
    ];
    pe_set_spv3(c, 0x90, m);
    pe_copy_vec3(c, V0, V1, 0x90, 0x60);
    // The listener offset in that frame.
    let axy = a[0] * d[0] + a[1] * d[1];
    let da = axy + a[2] * d[2];
    let dn = (n[0] * d[0] + n[1] * d[1]) + n[2] * d[2];
    let mz_dz = m[2] * d[2];
    let dm = (m[0] * d[0] + m[1] * d[1]) + mz_dz;
    let radius_a = Addr(rec.wrapping_add(emitter::RADIUS)).read_fpu(c);
    let radius_n = Addr(rec.wrapping_add(emitter::ELL_RADIUS_N)).read_fpu(c);
    let radius_m = Addr(rec.wrapping_add(emitter::ELL_RADIUS_M)).read_fpu(c);
    pe_set_f(
        c,
        &[
            (2, d[2]),
            (3, a[2] * d[2]),
            (4, n[2] * d[2]),
            (5, dm),
            (6, n[1] * d[1]),
            (7, a[1] * d[1]),
            (10, radius_a),
        ],
    );
    // Sum of the three axis terms.
    let term_a = match pe_axis_term(da, radius_a, zero) {
        None => {
            c.cycles += 2;
            FpuF32::ZERO
        }
        Some(t) => {
            c.cycles += 5;
            t
        }
    };
    c.cycles += 4;
    let term_n = match pe_axis_term(dn, radius_n, zero) {
        None => {
            c.cycles += 1;
            FpuF32::ZERO
        }
        Some(t) => {
            c.cycles += 5;
            t
        }
    };
    c.cycles += 4;
    let term_m = match pe_axis_term(dm, radius_m, zero) {
        None => {
            c.cycles += 1;
            FpuF32::ZERO
        }
        Some(t) => {
            c.cycles += 5;
            t
        }
    };
    let an = term_a + term_n;
    c.cycles += 5;
    let total = an + term_m;
    let inside = total.lt(one);
    crate::abi::set_fcc(c, inside);
    pe_set_f(c, &[(0, an), (1, term_m), (8, total), (9, term_a)]);
    c.g[A0][0] = c.g[S6][0];
    if !inside {
        c.cycles += 1;
        pe_next_record(c);
        return Ok(());
    }
    c.cycles += 14;
    c.f[12] = Addr(rec.wrapping_add(emitter::ELL_CURVE)).read_u32(c);
    c.f[13] = total.0;
    WORLD_SOUND_COMPUTE_FALLOFF.call(c, 0x002b_7e74)?;
    c.g[T0][0] = Addr(rec.wrapping_add(emitter::ID)).lw(c);
    c.f[12] = c.f[0];
    let pos = sx(sp(c).wrapping_add(0x20));
    pe_add_trigger(c, pos, 1, 0x002b_7e98)?;
    pe_next_record(c);
    Ok(())
}

/// Kind 2 (0x2b7ea0): a cone; inside the radius and within the half-angle,
/// volume = distance falloff * angle falloff.
fn pe_cone(c: &mut Cpu, rec: u32) -> Flow {
    c.cycles += 49;
    let one = FpuF32(c.f[23]);
    c.g[S2][0] = sx(sp(c).wrapping_add(0x20));
    pe_record_position(c, rec, A2, A3);
    let listener = r32(c, S3);
    let l = pe_v3(c, listener);
    let p = pe_spv3(c, 0x20);
    let d = [l[0] - p[0], l[1] - p[1], l[2] - p[2]];
    pe_set_spv3(c, 0x50, l);
    pe_set_spv3(c, 0x40, d);
    let (dx2, dy2, dz2) = (d[0] * d[0], d[1] * d[1], d[2] * d[2]);
    let dist = (dx2 + dy2 + dz2).sqrt();
    let radius = Addr(rec.wrapping_add(emitter::RADIUS)).read_fpu(c);
    let adist = dist.abs();
    let inside = adist.lt(radius);
    crate::abi::set_fcc(c, inside);
    pe_set_f(
        c,
        &[
            (0, dist),
            (1, radius),
            (2, dz2),
            (3, dy2),
            (4, l[1]),
            (5, d[2]),
            (6, d[1]),
            (7, d[0]),
            (8, adist),
        ],
    );
    if !inside {
        pe_next_record(c);
        return Ok(());
    }
    c.cycles += 23;
    // The cosine between the direction to the listener and the cone's.
    let inv = one.div(adist);
    let to = [d[0] * inv, d[1] * inv, d[2] * inv];
    pe_set_spv3(c, 0x40, to);
    let dir = pe_v3(c, rec.wrapping_add(emitter::CONE_DIR));
    pe_set_spv3(c, 0x50, dir);
    let xy = to[0] * dir[0];
    let yy = to[1] * dir[1];
    let zz = to[2] * dir[2];
    let xy_sum = xy + yy;
    let cos = xy_sum + zz;
    let limit = Addr(rec.wrapping_add(emitter::CONE_COS_LIMIT)).read_fpu(c);
    let within = limit.lt(cos);
    crate::abi::set_fcc(c, within);
    pe_set_f(
        c,
        &[
            (0, yy),
            (1, limit),
            (2, xy_sum),
            (3, dir[2]),
            (4, dir[1]),
            (5, zz),
            (22, cos),
        ],
    );
    c.g[A0][0] = c.g[S6][0];
    if !within {
        c.cycles += 1;
        pe_next_record(c);
        return Ok(());
    }
    c.cycles += 17;
    // Distance falloff.
    let r = Addr(rec.wrapping_add(emitter::RADIUS)).read_fpu(c);
    c.f[13] = adist.div(r).0;
    c.f[12] = Addr(rec.wrapping_add(emitter::CONE_DIST_CURVE)).read_u32(c);
    WORLD_SOUND_COMPUTE_FALLOFF.call(c, 0x002b_7fd0)?;
    // Angle falloff: (pi/2 - acos(cos)) / (pi/2 - acos(limit)).
    c.f[12] = Addr(rec.wrapping_add(emitter::CONE_COS_LIMIT)).read_u32(c);
    c.f[21] = c.f[0];
    call_ptr(c, ACOSF, 0x002b_7fdc)?;
    let half_pi = FpuF32(c.f[25]);
    let limit_angle = half_pi - FpuF32(c.f[0]);
    c.f[20] = limit_angle.0;
    c.f[12] = c.f[22];
    call_ptr(c, ACOSF, 0x002b_7fe8)?;
    let angle = half_pi - FpuF32(c.f[0]);
    c.f[0] = angle.0;
    c.f[12] = Addr(rec.wrapping_add(emitter::CONE_ANGLE_CURVE)).read_u32(c);
    c.f[13] = angle.div(FpuF32(c.f[20])).0;
    c.g[A0][0] = c.g[S6][0];
    WORLD_SOUND_COMPUTE_FALLOFF.call(c, 0x002b_7ffc)?;
    c.f[12] = (FpuF32(c.f[21]) * FpuF32(c.f[0])).0;
    c.g[T0][0] = Addr(rec.wrapping_add(emitter::ID)).lw(c);
    pe_add_trigger_common(c)
}

/// Kind 3 (0x2b8028): an ambient zone; full volume up to 0.9 of the radius
/// unit, then `(1 - distance) * 10`. Registers with flag 0.
fn pe_zone(c: &mut Cpu, rec: u32) -> Flow {
    c.cycles += 49;
    pe_record_position(c, rec, A0, A1);
    pe_listener_delta(c, 0x50);
    // The distance on VU0 (Q), as the original computes it.
    c.lqc2(3, sp(c).wrapping_add(0x40));
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(V1, 22);
    c.mtc1(V1, 0);
    let dist = FpuF32(c.f[0]);
    let radius = Addr(rec.wrapping_add(emitter::RADIUS)).read_fpu(c);
    let adist = dist.abs();
    let inside = adist.lt(radius);
    crate::abi::set_fcc(c, inside);
    pe_set_f(c, &[(1, radius), (8, adist)]);
    if !inside {
        pe_next_record(c);
        return Ok(());
    }
    c.cycles += 7;
    let gp = r32(c, GP);
    let near = Addr(gp.wrapping_sub(GP_ZONE_NEAR)).read_fpu(c);
    c.f[0] = near.0;
    c.g[AT][0] = 0x3f80_0000;
    c.f[21] = 0x3f80_0000;
    let full = adist.le(near);
    crate::abi::set_fcc(c, full);
    c.g[T0][0] = Addr(rec.wrapping_add(emitter::ID)).lw(c);
    if !full {
        c.cycles += 4;
        c.g[AT][0] = 0x4120_0000;
        let ten = FpuF32(0x4120_0000);
        c.f[0] = ten.0;
        c.f[21] = ((FpuF32(c.f[23]) - adist) * ten).0;
    }
    c.cycles += 8;
    c.f[12] = c.f[21];
    let pos = sx(sp(c).wrapping_add(0x20));
    pe_add_trigger(c, pos, 0, 0x002b_8138)?;
    c.cycles += 1;
    pe_next_record(c);
    Ok(())
}

/// `void cWorldSound::ProcessEmitter(cWorldSound* self, Vec4* listener,
/// int a2, Emitter* source)` (0x2b7908), hot during races (ambient world
/// sounds): for each record of the emitter's data block (a sphere,
/// ellipsoid, cone or ambient zone placed at the object's position plus an
/// offset), test the listener against its shape and, when inside, compute
/// the volume through `WorldSound_ComputeFalloff` and register the trigger
/// with `WorldTriggerManager_AddTrigger`. The record pointers are first
/// gathered into a frame array.
pub fn c_world_sound_process_emitter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x160);
        frame.save(c, S3, 0xf0);
        frame.save(c, S4, 0xe0);
        frame.save(c, S5, 0xd0);
        frame.save(c, S6, 0xc0);
        frame.save(c, S0, 0x120);
        frame.save(c, S1, 0x110);
        frame.save(c, S2, 0x100);
        frame.save(c, S7, 0xb0);
        frame.save_ra(c, 0xa0);
        for (f, off) in [
            (25, 0x158),
            (24, 0x150),
            (23, 0x148),
            (22, 0x140),
            (21, 0x138),
            (20, 0x130),
        ] {
            frame.save_fpr(c, f, off);
        }
        c.g[S3][0] = c.g[A1][0];
        c.g[S4][0] = c.g[A3][0];
        c.g[S5][0] = c.g[A2][0];
        c.g[S6][0] = c.g[A0][0];

        // The object's position, to +0x00 (through +0x40).
        let source = r32(c, S4);
        let object = Addr(source.wrapping_add(EMITTER_OBJECT)).lw(c) as u32;
        let data = Addr(source.wrapping_add(EMITTER_DATA)).lw(c);
        c.g[V1][0] = data;
        let data = data as u32;
        let pos = pe_v3(c, object.wrapping_add(OBJECT_POS));
        pe_set_spv3(c, 0x40, pos);
        pe_copy_vec3(c, V0, A1, 0x40, 0x00);
        pe_set_f(c, &[(0, pos[1]), (1, pos[0]), (2, pos[2])]);
        let skip = Addr(data.wrapping_add(emitter::SKIP_WORDS)).lw(c) as u32;
        let count = Addr(data.wrapping_add(emitter::COUNT)).lw(c);
        c.g[S7][0] = count;
        c.g[V0][0] = sx(skip << 2);
        c.g[A0][0] = sx(data.wrapping_add(emitter::RECORDS).wrapping_add(skip << 2));

        // Gather the record pointers into +0x10.. (a0 = record, a2 = slot,
        // a1 = records left).
        if (count as i64) > 0 {
            c.cycles += 6;
            c.g[T1][0] = 1;
            c.g[T0][0] = 2;
            c.g[A3][0] = 3;
            c.g[A2][0] = sx(sp(c).wrapping_add(0x10));
            c.g[A1][0] = count;
            c.sw(r32(c, A2), c.g[A0][0]);
            loop {
                c.cycles += 3;
                let rec = r32(c, A0);
                let kind = Addr(rec.wrapping_add(emitter::KIND)).lw(c);
                c.g[V1][0] = kind;
                c.g[V0][0] = ((kind as i64) < 2) as u64;
                let (cycles, size) = match kind as i64 {
                    0 => (8, emitter::SIZE_SPHERE),
                    1 => (5, emitter::SIZE_ELLIPSOID),
                    2 => (8, emitter::SIZE_CONE),
                    3 => (10, emitter::SIZE_ZONE),
                    k if k < 2 => (9, 0),
                    _ => (11, 0),
                };
                c.cycles += cycles;
                if size != 0 {
                    c.g[A0][0] = sx(rec.wrapping_add(size));
                }
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(4));
                c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
                if c.g[A1][0] == 0 {
                    break;
                }
                c.sw(r32(c, A2), c.g[A0][0]);
                if event_due(c) {
                    deliver_event_here(c, 0x002b_79c0);
                }
            }
        }

        // Test each record.
        c.cycles += 2;
        c.g[S1][0] = 0;
        if (c.g[S7][0] as i64) > 0 {
            c.cycles += 6;
            c.g[AT][0] = 0x3f80_0000;
            c.f[23] = 0x3f80_0000;
            c.f[24] = 0;
            let gp = r32(c, GP);
            c.f[25] = Addr(gp.wrapping_sub(GP_HALF_PI)).read_u32(c);
            c.g[V0][0] = 0;
            loop {
                c.cycles += 6;
                c.g[A0][0] = 1;
                let slot = sp(c).wrapping_add(r32(c, V0));
                c.g[V1][0] = sx(slot);
                let rec = Addr(slot.wrapping_add(0x10)).lw(c);
                c.g[S0][0] = rec;
                let rec = rec as u32;
                let kind = Addr(rec.wrapping_add(emitter::KIND)).lw(c);
                c.g[V1][0] = kind;
                c.g[V0][0] = ((kind as i64) < 2) as u64;
                match kind as i64 {
                    1 => pe_ellipsoid(c, rec)?,
                    0 => {
                        c.cycles += 4;
                        c.g[V0][0] = 2;
                        pe_sphere(c, rec)?;
                    }
                    2 => {
                        c.cycles += 4;
                        c.g[V0][0] = 3;
                        pe_cone(c, rec)?;
                    }
                    3 => {
                        c.cycles += 6;
                        c.g[V0][0] = 3;
                        pe_zone(c, rec)?;
                    }
                    k => {
                        // Unknown kind: skipped. Negative kinds fail the
                        // `== 0` test (6 cycles); kinds >= 4 fail the
                        // `< 2`, `== 2` and `== 3` tests (8 cycles).
                        if k < 2 {
                            c.cycles += 6;
                            c.g[V0][0] = 2;
                        } else {
                            c.cycles += 8;
                            c.g[V0][0] = 3;
                        }
                        pe_next_record(c);
                    }
                }
                c.cycles += 3;
                let more = (c.g[S1][0] as i64) < (c.g[S7][0] as i64);
                c.g[V0][0] = sx(r32(c, S1) << 2);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002b_7a30);
                }
            }
        }

        c.cycles += 17;
        frame.restore(c, S0, 0x120);
        frame.restore(c, S1, 0x110);
        frame.restore(c, S2, 0x100);
        frame.restore(c, S3, 0xf0);
        frame.restore(c, S4, 0xe0);
        frame.restore(c, S5, 0xd0);
        frame.restore(c, S6, 0xc0);
        frame.restore(c, S7, 0xb0);
        frame.restore_ra(c, 0xa0);
        for (f, off) in [
            (25, 0x158),
            (24, 0x150),
            (23, 0x148),
            (22, 0x140),
            (21, 0x138),
            (20, 0x130),
        ] {
            frame.restore_fpr(c, f, off);
        }
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// UVScrollModifier_GetMatrix: rotation (angle != 0, through the real
    /// Math_SinCos) and identity (angle == 0, also -0) paths, with edge-case
    /// axis and translation values, and an `m` that overlaps `self` for some
    /// seeds (the original re-reads the axis between stores).
    #[test]
    fn uv_scroll_modifier_get_matrix_matches() {
        use crate::difftest::{check_paths, fill, scramble, Rng, DATA, GAME_GP};
        const MATRIX: u32 = DATA + 0x100;
        check_paths(
            ee::s12::f_0035fc20__generated,
            uv_scroll_modifier_get_matrix,
            300,
            &[12 + 10 + 11],
            &|c, seed| {
                let mut r = Rng(0x35fc_2000 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                // Math_SinCos's .lit4 constants and the identity matrix.
                fill(c, &mut r, GAME_GP - 0x2ec0, 8);
                for i in 0..16u32 {
                    let one = if i % 5 == 0 { 0x3f80_0000 } else { 0 };
                    c.bus.write32(IDENTITY_MATRIX + i * 4, one);
                }
                fill(c, &mut r, DATA, 0x20);
                let angle = match seed % 4 {
                    0 => 0,
                    1 => 0x8000_0000,
                    _ => r.f32(6.3),
                };
                c.bus.write32(DATA + uv_scroll::ANGLE, angle);
                for off in [
                    uv_scroll::AXIS_X,
                    uv_scroll::AXIS_Y,
                    uv_scroll::AXIS_Z,
                    uv_scroll::TRANS_U,
                    uv_scroll::TRANS_V,
                ] {
                    let v = if seed % 3 == 0 {
                        r.edge_f32()
                    } else {
                        r.f32(2.0)
                    };
                    c.bus.write32(DATA + off, v);
                }
                fill(c, &mut r, MATRIX, 16);
                c.g[A0][0] = sx(DATA);
                // Some seeds: m overlaps self's axis (aliasing order).
                c.g[A1][0] = sx(if seed % 7 == 5 { DATA + 0x10 } else { MATRIX });
                c.pc = 0x0035_fc20;
            },
        );
    }

    /// cWorldSound_ProcessEmitter: one to four records of every kind (sphere,
    /// ellipsoid, cone, zone, unknown kinds that stall the gather), with the
    /// listener near or far so each shape's inside/outside tests (ellipsoid
    /// terms with zero radii, the cone's half-angle, the zone's full-volume
    /// distance) both go either way; also empty and negative counts. The real
    /// WorldSound_ComputeFalloff, acosf and WorldTriggerManager_AddTrigger
    /// run (the manager has no instance table: AddTrigger's short path); the
    /// game image provides the `.lit4` constants and the falloff jump table.
    #[test]
    fn c_world_sound_process_emitter_matches() {
        use crate::difftest::{check, elf_image, put_vec, scramble, Rng, DATA, GAME_GP};
        /// The emitter source (a3): +0xc object, +0x10 data block.
        const SOURCE: u32 = DATA;
        const OBJECT: u32 = DATA + 0x100;
        const BLOCK: u32 = DATA + 0x200;
        const LISTENER: u32 = DATA + 0x800;
        const SELF: u32 = DATA + 0x900;
        const MANAGER: u32 = DATA + 0xa00;
        fn unit(r: &mut Rng) -> f32 {
            r.next_u32() as f32 / u32::MAX as f32
        }
        fn signed(r: &mut Rng, range: f32) -> f32 {
            (unit(r) * 2.0 - 1.0) * range
        }
        fn w(c: &mut Cpu, a: u32, v: u32) {
            c.bus.write32(a, v);
        }
        fn curve(r: &mut Rng) -> u32 {
            ((r.next_u32() % 8) as f32).to_bits()
        }
        check(
            ee::s08::f_002b7908__generated,
            c_world_sound_process_emitter,
            240,
            &|c, seed| {
                let mut r = Rng(0x2b79_0800 ^ (seed as u64).wrapping_mul(0x9e37_79b9));
                scramble(c, &mut r);
                elf_image(c);
                c.g[GP][0] = sx(GAME_GP);
                w(c, GAME_GP + GP_TRIGGER_MANAGER, MANAGER);
                w(c, MANAGER + 0x334, 0);
                w(c, SOURCE + EMITTER_OBJECT, OBJECT);
                w(c, SOURCE + EMITTER_DATA, BLOCK);

                // Object position; the listener near the emitter (scaled).
                let obj = [
                    signed(&mut r, 10.0),
                    signed(&mut r, 10.0),
                    signed(&mut r, 10.0),
                ];
                for i in 0..3 {
                    w(c, OBJECT + OBJECT_POS + 4 * i, obj[i as usize].to_bits());
                }
                let off0 = [
                    signed(&mut r, 1.0),
                    signed(&mut r, 1.0),
                    signed(&mut r, 1.0),
                ];
                let scale = [0.4f32, 1.5, 6.0, 30.0][(seed % 4) as usize];
                let lis = [
                    obj[0] + off0[0] + signed(&mut r, scale),
                    obj[1] + off0[1] + signed(&mut r, scale),
                    obj[2] + off0[2] + signed(&mut r, scale),
                ];
                put_vec(
                    c,
                    LISTENER,
                    [
                        lis[0].to_bits(),
                        lis[1].to_bits(),
                        lis[2].to_bits(),
                        r.next_u32(),
                    ],
                );

                // Header and records.
                let skip = seed % 3;
                let count: u32 = match seed % 13 {
                    0 => 0,
                    1 => 0xffff_ffff,
                    _ => 1 + (seed / 3) % 4,
                };
                w(c, BLOCK + emitter::SKIP_WORDS, skip);
                w(c, BLOCK + emitter::COUNT, count);
                let mut at = BLOCK + emitter::RECORDS + 4 * skip;
                for i in 0..skip {
                    w(c, BLOCK + emitter::RECORDS + 4 * i, r.next_u32());
                }
                for _ in 0..count.min(4) {
                    let kind: u32 = [0, 1, 2, 3, 0, 1, 2, 3, 1, 2, 0xffff_ffff, 7]
                        [(r.next_u32() % 12) as usize];
                    for k in 0..12 {
                        w(c, at + 4 * k, r.next_u32());
                    }
                    w(c, at + emitter::KIND, kind);
                    w(c, at + emitter::ID, r.next_u32() & 0xff);
                    for k in 0..3 {
                        let o = off0[k as usize] + signed(&mut r, 0.1);
                        w(c, at + emitter::OFFSET + 4 * k, o.to_bits());
                    }
                    let radius = |r: &mut Rng, max: f32| (unit(r) * max).to_bits();
                    let size = match kind {
                        0 => {
                            w(c, at + emitter::RADIUS, radius(&mut r, 3.0 * scale));
                            w(c, at + emitter::SPHERE_CURVE, curve(&mut r));
                            emitter::SIZE_SPHERE
                        }
                        1 => {
                            for f in [
                                emitter::RADIUS,
                                emitter::ELL_RADIUS_N,
                                emitter::ELL_RADIUS_M,
                            ] {
                                let v = if r.next_u32() % 4 == 0 {
                                    0
                                } else {
                                    radius(&mut r, 3.0 * scale)
                                };
                                w(c, at + f, v);
                            }
                            // Axis: random, or parallel to z (a degenerate frame).
                            let axis = if r.next_u32() % 6 == 0 {
                                [0.0, 0.0, 1.0]
                            } else {
                                [
                                    signed(&mut r, 1.0),
                                    signed(&mut r, 1.0),
                                    signed(&mut r, 1.0),
                                ]
                            };
                            for k in 0..3 {
                                w(
                                    c,
                                    at + emitter::ELL_AXIS + 4 * k,
                                    axis[k as usize].to_bits(),
                                );
                            }
                            w(c, at + emitter::ELL_CURVE, curve(&mut r));
                            emitter::SIZE_ELLIPSOID
                        }
                        2 => {
                            w(c, at + emitter::RADIUS, radius(&mut r, 3.0 * scale));
                            let dir = [
                                signed(&mut r, 1.0),
                                signed(&mut r, 1.0),
                                signed(&mut r, 1.0),
                            ];
                            let n = (dir[0] * dir[0] + dir[1] * dir[1] + dir[2] * dir[2]).sqrt();
                            for k in 0..3 {
                                w(
                                    c,
                                    at + emitter::CONE_DIR + 4 * k,
                                    (dir[k as usize] / n).to_bits(),
                                );
                            }
                            w(c, at + emitter::CONE_DIST_CURVE, curve(&mut r));
                            w(
                                c,
                                at + emitter::CONE_COS_LIMIT,
                                signed(&mut r, 1.0).to_bits(),
                            );
                            w(c, at + emitter::CONE_ANGLE_CURVE, curve(&mut r));
                            emitter::SIZE_CONE
                        }
                        3 => {
                            w(c, at + emitter::RADIUS, radius(&mut r, 1.6));
                            emitter::SIZE_ZONE
                        }
                        _ => 0,
                    };
                    if size == 0 {
                        // The gather stalls here: every later record is this one.
                        break;
                    }
                    at += size;
                }

                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(LISTENER);
                c.g[A2][0] = (r.next_u32() & 0xffff) as u64;
                c.g[A3][0] = sx(SOURCE);
                c.pc = 0x002b_7908;
            },
        );
    }
}
