//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements, each with a differential test against the generated body.
//!
//! - **`BoaderShadow_ComputeProjection`** (0x374d00): turns the board
//!   shadow's three input vectors (position `b`, direction `dir`, reference
//!   `a`) into the orthonormal basis and projection matrices the shadow
//!   renderer uses: two Gram-Schmidt-style cross products, three
//!   VU0-transformed 4x4 matrices (light/projection/view), the six shadow
//!   bone matrices gathered from the bone table, and the depth/scale
//!   constants. It writes the whole result into the object at `a0`.

use crate::abi;
use crate::events::{deliver_event_here, event_due};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x0037_4d00,
    boader_shadow_compute_projection,
    "BoaderShadow_ComputeProjection",
)];

/// `a + off`, wrapping like the original's `addiu`.
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// One of the three identical `vu0` matrix products: load the two matrices
/// from `a` and `b` into VF4..VF11, multiply rows into VF12..VF15 and store
/// them at `out`. The COP2 operations are issued in the original's order so
/// VF/ACC and the VU0 flags end exactly as they do there.
fn transform_matrix(c: &mut Cpu, a: u32, b: u32, out: u32) {
    c.lqc2(4, a);
    c.lqc2(5, p(a, 0x10));
    c.lqc2(6, p(a, 0x20));
    c.lqc2(7, p(a, 0x30));
    c.lqc2(8, b);
    c.lqc2(9, p(b, 0x10));
    c.lqc2(10, p(b, 0x20));
    c.lqc2(11, p(b, 0x30));
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
    c.sqc2(12, out);
    c.sqc2(13, p(out, 0x10));
    c.sqc2(14, p(out, 0x20));
    c.sqc2(15, p(out, 0x30));
}

/// A 4x4 matrix moved through the quad registers (the original does
/// `lqc2`/`sqc2` twice instead of a plain `lq`/`sq` to keep the quad pipeline).
fn copy_matrix(c: &mut Cpu, from: u32, to: u32) {
    c.lqc2(1, from);
    c.lqc2(2, p(from, 0x10));
    c.lqc2(3, p(from, 0x20));
    c.lqc2(4, p(from, 0x30));
    c.sqc2(1, to);
    c.sqc2(2, p(to, 0x10));
    c.sqc2(3, p(to, 0x20));
    c.sqc2(4, p(to, 0x30));
}

/// `Cell`/`BoaderShadow` object layout (the fields this function touches).
mod shadow {
    /// Input vector `b` normalised into place: `{x, y, z, 0}`.
    pub const VEC_B: u32 = 0x00;
    /// Reference vector `a`.
    pub const VEC_A: u32 = 0x0c;
    /// Normalised direction `dir`.
    pub const DIR: u32 = 0x18;
    /// `b . dir` (the dot product of the two normalised vectors).
    pub const DOT: u32 = 0x24;
    /// Result matrices (4x4 each, 16-byte aligned).
    pub const MAT_B: u32 = 0x28;
    pub const MAT_C: u32 = 0x34;
    pub const LIGHT: u32 = 0x50;
    pub const PROJECTION: u32 = 0x90;
    pub const VIEW: u32 = 0xd0;
    pub const BONES: u32 = 0x110;
    /// Six indices into the bone table, consumed by the gather loop.
    pub const BONE_INDEX: u32 = 0x17c;
    /// Depth scale / bias inputs (integers).
    pub const SCALE_IN: u32 = 0x174;
    pub const BIAS_IN: u32 = 0x178;
    pub const SCALE: u32 = 0x150;
    pub const DEPTH: u32 = 0x158;
}

/// `void BoaderShadow_ComputeProjection(void* obj, Vec3* a, Vec3* b,
/// Vec3* dir, Mat4* bones, int f)` (0x374d00).
///
/// `a0`/`t1` is the shadow object, `a1` a reference direction, `a2` the
/// position vector, `a3` the shadow direction, `t0` the bone matrix table.
/// The int argument is unused (the original immediately reuses `t1` for the
/// object pointer; the fixed count of six bone entries is hard-coded).
pub fn boader_shadow_compute_projection(c: &mut Cpu) {
    ssx3_ee_rt::prof::enter(c, 0x0037_4d00);

    // ---------------------------------------------------------------
    // Prologue and normalisation of the `b` vector (`a2`) into the frame
    // scratch vector at `sp+0`.
    // ---------------------------------------------------------------
    c.cycles += 30;
    let sp = (c.g[SP][0] as u32).wrapping_sub(0x190);
    c.g[SP][0] = sx(sp);
    c.mtc1(0, 3); // f3 = 0.0
    c.sq(16, p(sp, 0x120));
    c.g[T1][0] = c.g[A0][0];
    c.sq(17, p(sp, 0x110));
    c.swc1(31, p(sp, 0x188));
    c.swc1(30, p(sp, 0x180));
    c.swc1(29, p(sp, 0x178));
    c.swc1(28, p(sp, 0x170));
    c.swc1(27, p(sp, 0x168));
    c.swc1(26, p(sp, 0x160));
    c.swc1(25, p(sp, 0x158));
    c.swc1(24, p(sp, 0x150));
    c.swc1(23, p(sp, 0x148));
    c.swc1(22, p(sp, 0x140));
    c.swc1(21, p(sp, 0x138));
    c.swc1(20, p(sp, 0x130));
    let obj = c.g[T1][0] as u32;

    c.lwc1(4, c.g[A2][0] as u32);
    c.lwc1(5, p(c.g[A2][0] as u32, 0x4));
    c.mul_s(0, 4, 4);
    c.lwc1(6, p(c.g[A2][0] as u32, 0x8));
    c.mul_s(1, 5, 5);
    c.mul_s(2, 6, 6);
    c.add_s(0, 0, 1);
    c.add_s(0, 0, 2);
    c.sqrt_s(1, 0);
    c.c_eq_s(1, 3);
    if c.fcc() {
        // Zero length: keep the raw x component, leave y/z from `a2`.
        c.swc1(4, sp);
        c.cycles += 2;
        c.swc1(5, p(sp, 0x4));
        c.swc1(6, p(sp, 0x8));
    } else {
        c.cycles += 10;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.div_s(0, 0, 1);
        c.mul_s(1, 6, 0);
        c.mul_s(2, 4, 0);
        c.mul_s(0, 5, 0);
        c.swc1(1, p(sp, 0x8));
        c.swc1(2, sp);
        c.swc1(0, p(sp, 0x4));
    }

    // ---------------------------------------------------------------
    // Copy that vector to `obj+0`, then normalise the direction `a3` into
    // the frame and set `v0 = obj+0x18` for the next pass.
    // ---------------------------------------------------------------
    c.cycles += 21;
    c.ldl(2, p(sp, 0x7));
    c.ldr(2, sp);
    c.g[V1][0] = c.lw(p(sp, 0x8));
    c.sdl(2, p(obj, 0x7));
    c.sdr(2, obj);
    c.sw(p(obj, 0x8), c.g[V1][0]);
    c.mtc1(0, 3);
    c.g[V0][0] = sx(p(obj, 0x18));
    c.lwc1(4, c.g[A3][0] as u32);
    c.lwc1(5, p(c.g[A3][0] as u32, 0x4));
    c.mul_s(0, 4, 4);
    c.lwc1(6, p(c.g[A3][0] as u32, 0x8));
    c.mul_s(1, 5, 5);
    c.mul_s(2, 6, 6);
    c.add_s(0, 0, 1);
    c.add_s(0, 0, 2);
    c.sqrt_s(1, 0);
    c.c_eq_s(1, 3);
    if c.fcc() {
        c.swc1(4, sp);
        c.cycles += 2;
        c.swc1(5, p(sp, 0x4));
        c.swc1(6, p(sp, 0x8));
    } else {
        c.cycles += 10;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.div_s(0, 0, 1);
        c.mul_s(1, 6, 0);
        c.mul_s(2, 4, 0);
        c.mul_s(0, 5, 0);
        c.swc1(1, p(sp, 0x8));
        c.swc1(2, sp);
        c.swc1(0, p(sp, 0x4));
    }

    // ---------------------------------------------------------------
    // Direction to `obj+0x18`, reference vector `a1` to `obj+0x0c` and to
    // the scratch vector (negated), and the dot product into `obj+0x24`.
    // `v0` becomes 1 when `b` is exactly parallel to `dir`.
    // ---------------------------------------------------------------
    c.cycles += 44;
    let dir = p(obj, shadow::DIR);
    c.ldl(7, p(sp, 0x7));
    c.ldr(7, sp);
    c.g[T3][0] = c.lw(p(sp, 0x8));
    c.sdl(7, p(dir, 0x7));
    c.sdr(7, dir);
    c.sw(p(dir, 0x8), c.g[T3][0]);
    c.mtc1(0, 2); // f2 = 0.0
    c.g[V0][0] = 0;
    c.lwc1(3, p(c.g[A1][0] as u32, 0x8));
    c.lwc1(0, c.g[A1][0] as u32);
    c.lwc1(1, p(c.g[A1][0] as u32, 0x4));
    c.swc1(3, p(sp, 0x8));
    c.swc1(0, sp);
    c.swc1(1, p(sp, 0x4));
    c.swc1(2, p(sp, 0x18));
    c.swc1(2, p(sp, 0x10));
    c.swc1(2, p(sp, 0x14));
    c.ldl(5, p(sp, 0x7));
    c.ldr(5, sp);
    c.g[A2][0] = c.lw(p(sp, 0x8));
    c.sdl(5, p(obj, 0x13));
    c.sdr(5, p(obj, 0x0c));
    c.sw(p(obj, 0x14), c.g[A2][0]);
    let vec_a = p(obj, shadow::VEC_A);
    c.lwc1(7, p(dir, 0x0));
    c.lwc1(8, p(dir, 0x4));
    c.neg_s(3, 7);
    c.lwc1(9, p(dir, 0x8));
    c.neg_s(5, 8);
    c.lwc1(0, p(vec_a, 0x0));
    c.lwc1(1, p(vec_a, 0x4));
    c.neg_s(6, 9);
    c.mul_s(0, 3, 0);
    c.lwc1(2, p(vec_a, 0x8));
    c.mul_s(1, 5, 1);
    c.lwc1(4, p(obj, shadow::VEC_B));
    c.mul_s(2, 6, 2);
    c.swc1(3, sp);
    c.c_eq_s(4, 7);
    c.swc1(5, p(sp, 0x4));
    c.add_s(0, 0, 1);
    c.swc1(6, p(sp, 0x8));
    c.add_s(0, 0, 2);
    let different = !c.fcc();
    c.swc1(0, p(obj, shadow::DOT));
    if !different {
        // b.x == dir.x: try the remaining components.
        c.cycles += 4;
        c.lwc1(0, p(obj, shadow::VEC_B + 0x4));
        c.c_eq_s(0, 8);
        if c.fcc() {
            c.cycles += 5;
            c.lwc1(0, p(obj, shadow::VEC_B + 0x8));
            c.c_eq_s(0, 9);
            if c.fcc() {
                c.g[V0][0] = 0x1;
            }
        }
    }

    // ---------------------------------------------------------------
    // If `b` is not parallel to `dir`, check it against the negated `dir`;
    // when `b` is parallel to neither axis, bias `obj+0` by a global. `f5`
    // carries `obj+0x1c` into the first cross product below.
    // ---------------------------------------------------------------
    c.cycles += 2;
    let mut bias_b = false;
    if c.g[V0][0] != 0 {
        c.lwc1(0, p(obj, shadow::VEC_B));
        bias_b = true;
    } else {
        c.cycles += 13;
        c.lwc1(0, p(dir, 0x0));
        c.g[V0][0] = 0;
        c.lwc1(2, p(dir, 0x4));
        c.lwc1(3, p(dir, 0x8));
        c.neg_s(0, 0);
        c.lwc1(1, p(obj, shadow::VEC_B));
        c.neg_s(2, 2);
        c.neg_s(3, 3);
        c.c_eq_s(1, 0);
        c.swc1(0, sp);
        c.swc1(2, p(sp, 0x4));
        let different = !c.fcc();
        c.swc1(3, p(sp, 0x8));
        if !different {
            c.cycles += 4;
            c.lwc1(0, p(obj, shadow::VEC_B + 0x4));
            c.c_eq_s(0, 2);
            if c.fcc() {
                c.cycles += 5;
                c.lwc1(0, p(obj, shadow::VEC_B + 0x8));
                c.c_eq_s(0, 3);
                if c.fcc() {
                    c.g[V0][0] = 0x1;
                }
            }
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.lwc1(5, p(dir, 0x4));
        } else {
            c.cycles += 1;
            c.lwc1(0, p(obj, shadow::VEC_B));
            bias_b = true;
        }
    }
    if bias_b {
        c.cycles += 4;
        c.lwc1(1, p(c.g[GP][0] as u32, 0xffff_d808)); // gp - 0x27f8
        c.add_s(0, 0, 1);
        c.swc1(0, p(obj, shadow::VEC_B));
        c.lwc1(5, p(dir, 0x4));
    }

    // ---------------------------------------------------------------
    // First cross product (`dir` x `b`), normalised into the frame vector
    // at `sp+0`; `v0 = obj+0x34` receives it in the next block.
    // ---------------------------------------------------------------
    c.cycles += 29;
    c.g[V0][0] = sx(p(obj, shadow::MAT_C));
    c.lwc1(4, p(obj, 0x4));
    c.lwc1(1, p(obj, 0x20));
    c.lwc1(0, p(obj, 0x8));
    c.lwc1(6, p(dir, 0x0));
    c.mul_s(7, 4, 1);
    c.lwc1(2, p(obj, 0x0));
    c.mul_s(3, 0, 5);
    c.mul_s(0, 0, 6);
    c.mul_s(1, 2, 1);
    c.sub_s(7, 7, 3);
    c.mul_s(2, 2, 5);
    c.sub_s(8, 0, 1);
    c.mul_s(4, 4, 6);
    c.swc1(7, p(sp, 0x20));
    c.mul_s(0, 7, 7);
    c.mul_s(3, 8, 8);
    c.swc1(8, p(sp, 0x24));
    c.sub_s(2, 2, 4);
    c.mtc1(0, 5); // f5 = 0.0
    c.add_s(0, 0, 3);
    c.mul_s(1, 2, 2);
    c.swc1(2, p(sp, 0x28));
    c.add_s(0, 0, 1);
    c.sqrt_s(1, 0);
    c.c_eq_s(1, 5);
    if c.fcc() {
        c.swc1(7, sp);
        c.cycles += 2;
        c.swc1(8, p(sp, 0x4));
        c.swc1(2, p(sp, 0x8));
    } else {
        c.cycles += 10;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.div_s(0, 0, 1);
        c.mul_s(1, 2, 0);
        c.mul_s(2, 7, 0);
        c.mul_s(0, 8, 0);
        c.swc1(1, p(sp, 0x8));
        c.swc1(2, sp);
        c.swc1(0, p(sp, 0x4));
    }

    // ---------------------------------------------------------------
    // `obj+0x34` gets that vector; a second cross product (`dir` x the
    // `obj+0x1c..` plane row) lands in the frame at `sp+0x10..0x18` and is
    // normalised into `sp+0`, then written to `obj+0x28` below.
    // ---------------------------------------------------------------
    c.cycles += 36;
    let mat_c = p(obj, shadow::MAT_C);
    c.ldl(3, p(sp, 0x7));
    c.ldr(3, sp);
    c.g[A0][0] = c.lw(p(sp, 0x8));
    c.sdl(3, p(mat_c, 0x7));
    c.sdr(3, mat_c);
    c.sw(p(mat_c, 0x8), c.g[A0][0]);
    c.mtc1(0, 8); // f8 = 0.0
    c.g[V0][0] = sx(p(obj, shadow::MAT_B));
    c.lwc1(5, p(obj, 0x38));
    c.lwc1(4, p(obj, 0x1c));
    c.lwc1(1, p(obj, 0x3c));
    c.lwc1(0, p(obj, 0x20));
    c.lwc1(6, p(obj, 0x34));
    c.mul_s(7, 4, 1);
    c.lwc1(2, p(dir, 0x0));
    c.mul_s(3, 0, 5);
    c.mul_s(0, 0, 6);
    c.mul_s(1, 2, 1);
    c.mul_s(2, 2, 5);
    c.sub_s(7, 7, 3);
    c.sub_s(5, 0, 1);
    c.mul_s(4, 4, 6);
    c.mul_s(0, 7, 7);
    c.swc1(7, p(sp, 0x10));
    c.mul_s(3, 5, 5);
    c.swc1(5, p(sp, 0x14));
    c.sub_s(2, 2, 4);
    c.add_s(0, 0, 3);
    c.mul_s(1, 2, 2);
    c.swc1(2, p(sp, 0x18));
    c.add_s(0, 0, 1);
    c.sqrt_s(1, 0);
    c.c_eq_s(1, 8);
    if c.fcc() {
        c.swc1(7, sp);
        c.cycles += 2;
        c.swc1(5, p(sp, 0x4));
        c.swc1(2, p(sp, 0x8));
    } else {
        c.cycles += 10;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.div_s(0, 0, 1);
        c.mul_s(1, 2, 0);
        c.mul_s(2, 7, 0);
        c.mul_s(0, 5, 0);
        c.swc1(1, p(sp, 0x8));
        c.swc1(2, sp);
        c.swc1(0, p(sp, 0x4));
    }

    // ---------------------------------------------------------------
    // `obj+0x28` gets the last vector. Set up the basis (rows built from the
    // two cross products and the direction), the light matrix and the six
    // bone gathers.
    // ---------------------------------------------------------------
    c.cycles += 100;
    let mat_b = p(obj, shadow::MAT_B);
    c.ldl(7, p(sp, 0x7));
    c.ldr(7, sp);
    c.g[T3][0] = c.lw(p(sp, 0x8));
    c.sdl(7, p(mat_b, 0x7));
    c.sdr(7, mat_b);
    c.g[T5][0] = sx(p(sp, 0x50));
    c.sw(p(mat_b, 0x8), c.g[T3][0]);
    c.g[T4][0] = sx(p(obj, shadow::LIGHT));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 18); // f18 = 1.0
    c.g[V1][0] = 0x004a_0000;
    c.lwc1(17, p(obj, shadow::VEC_A));
    c.g[A0][0] = 0x004a_0000;
    c.lwc1(15, p(obj, 0x10));
    c.mov_s(31, 18);
    c.lwc1(5, p(obj, 0x34));
    c.g[T3][0] = sx(p(sp, 0xd0));
    c.lwc1(7, p(obj, 0x38));
    c.g[T6][0] = sx(p(sp, 0x90));
    c.lwc1(8, p(obj, 0x28));
    c.mul_s(3, 17, 5);
    c.lwc1(9, p(obj, 0x2c));
    c.mul_s(12, 15, 7);
    c.mul_s(2, 17, 8);
    c.lwc1(13, p(obj, 0x14));
    c.mul_s(14, 15, 9);
    c.lwc1(0, p(obj, 0x3c));
    c.lwc1(1, p(obj, 0x30));
    c.add_s(3, 3, 12);
    c.swc1(5, p(obj, 0x50));
    c.mov_s(28, 5);
    c.lwc1(10, p(dir, 0x0));
    c.mul_s(12, 13, 1);
    c.lwc1(11, p(dir, 0x4));
    c.mul_s(5, 13, 0);
    c.add_s(2, 2, 14);
    c.lwc1(4, p(obj, 0x20));
    c.mul_s(16, 15, 11);
    c.swc1(0, p(obj, 0x58));
    c.mul_s(6, 17, 10);
    c.mov_s(22, 0);
    c.add_s(2, 2, 12);
    c.swc1(1, p(obj, 0x68));
    c.add_s(3, 3, 5);
    c.swc1(10, p(obj, 0x70));
    c.swc1(11, p(obj, 0x74));
    c.mul_s(0, 13, 4);
    c.swc1(7, p(obj, 0x54));
    c.add_s(6, 6, 16);
    c.swc1(8, p(obj, 0x60));
    c.mov_s(21, 1);
    c.swc1(9, p(obj, 0x64));
    c.mov_s(24, 10);
    c.swc1(2, p(sp, 0x14));
    c.add_s(6, 6, 0);
    c.swc1(3, p(sp, 0x10));
    c.mov_s(23, 11);
    c.swc1(18, p(obj, 0x8c));
    c.mov_s(27, 7);
    c.swc1(4, p(obj, 0x78));
    c.mov_s(26, 8);
    c.sw(p(obj, 0x80), 0u64);
    c.mov_s(25, 9);
    c.sw(p(obj, 0x84), 0u64);
    c.mov_s(20, 4);
    c.sw(p(obj, 0x88), 0u64);
    c.g[A2][0] = sx(p(obj, shadow::PROJECTION));
    c.swc1(17, p(sp, 0x30));
    c.g[T8][0] = sx(p(obj, shadow::VIEW));
    c.swc1(15, p(sp, 0x34));
    c.g[T2][0] = sx(p(obj, shadow::BONES));
    c.swc1(13, p(sp, 0x38));
    c.g[A1][0] = sx(p(obj, shadow::BONE_INDEX));
    c.swc1(18, p(sp, 0x3c));
    c.g[A3][0] = 0x5;
    c.swc1(6, p(sp, 0x18));
    c.lwc1(16, p(obj, 0x88));
    c.swc1(18, p(sp, 0x1c));
    c.lwc1(11, p(c.g[V1][0] as u32, 0x431c));
    c.mov_s(30, 16);
    c.lq(2, p(sp, 0x10));
    c.mov_s(29, 16);
    c.lwc1(10, p(c.g[A0][0] as u32, 0x4320));
    c.mov_s(14, 11);
    c.sq(2, sp);
    c.mov_s(12, 11);
    c.mov_s(13, 10);
    c.mov_s(15, 10);
    c.lwc1(0, p(sp, 0x0));
    c.lwc1(1, p(sp, 0x4));
    c.lwc1(2, p(sp, 0x8));
    c.neg_s(0, 0);
    c.neg_s(1, 1);
    c.neg_s(2, 2);
    c.swc1(0, p(obj, 0x5c));
    c.mov_s(19, 0);
    c.swc1(1, p(obj, 0x6c));
    c.mov_s(18, 1);
    c.swc1(2, p(obj, 0x7c));
    c.mov_s(17, 2);

    // Six bone entries: index from `obj+0x17c..`, entry stride 0x40, row
    // taken from `+0x30` and transformed by the current basis.
    'gather: loop {
        c.cycles += 48;
        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 6);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.lq(2, p(c.g[V1][0] as u32, 0x30));
        c.sq(2, p(sp, 0x40));
        c.lwc1(1, p(sp, 0x40));
        c.lwc1(0, p(sp, 0x44));
        c.mul_s(6, 1, 30);
        c.lwc1(5, p(sp, 0x48));
        c.mul_s(8, 0, 27);
        c.lwc1(7, p(sp, 0x4c));
        c.mul_s(2, 0, 25);
        c.mul_s(9, 0, 16);
        c.mul_s(4, 1, 28);
        c.mul_s(3, 1, 26);
        c.mul_s(0, 0, 23);
        c.mul_s(1, 1, 24);
        c.add_s(4, 4, 8);
        c.add_s(3, 3, 2);
        c.add_s(1, 1, 0);
        c.mul_s(0, 5, 22);
        c.mul_s(2, 5, 21);
        c.mul_s(8, 5, 29);
        c.mul_s(5, 5, 20);
        c.add_s(6, 6, 9);
        c.add_s(4, 4, 0);
        c.add_s(3, 3, 2);
        c.add_s(1, 1, 5);
        c.mul_s(0, 7, 19);
        c.mul_s(5, 7, 31);
        c.mul_s(2, 7, 18);
        c.add_s(6, 6, 8);
        c.mul_s(7, 7, 17);
        c.add_s(4, 4, 0);
        c.add_s(3, 3, 2);
        c.add_s(1, 1, 7);
        c.add_s(6, 6, 5);
        c.swc1(4, p(sp, 0x20));
        c.swc1(3, p(sp, 0x24));
        c.swc1(1, p(sp, 0x28));
        c.swc1(6, p(sp, 0x2c));
        c.lq(2, p(sp, 0x20));
        c.sq(2, p(sp, 0x10));
        c.lwc1(0, p(sp, 0x10));
        c.c_lt_s(0, 14);
        if c.fcc() {
            c.mov_s(14, 0);
        }
        c.cycles += 4;
        c.c_lt_s(13, 0);
        if c.fcc() {
            c.mov_s(13, 0);
        }
        c.cycles += 5;
        c.lwc1(0, p(sp, 0x14));
        c.c_lt_s(0, 11);
        if c.fcc() {
            c.mov_s(11, 0);
        }
        c.cycles += 4;
        c.c_lt_s(10, 0);
        if c.fcc() {
            c.mov_s(10, 0);
        }
        c.cycles += 5;
        c.lwc1(0, p(sp, 0x18));
        c.c_lt_s(0, 12);
        if c.fcc() {
            c.mov_s(12, 0);
        }
        c.cycles += 4;
        c.c_lt_s(15, 0);
        if c.fcc() {
            c.mov_s(15, 0);
        }
        c.cycles += 3;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
        let more = (c.g[A3][0] as i64) >= 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0037_5270);
            }
            continue 'gather;
        }
        break;
    }

    // ---------------------------------------------------------------
    // Epilogue: the three projection passes and the depth/scale results.
    // ---------------------------------------------------------------
    c.cycles += 318;
    c.g[AT][0] = 0x428c_0000;
    c.mtc1(1, 0); // f0 = 70.0
    c.g[V0][0] = c.g[T6][0];
    c.lwc1(5, p(c.g[GP][0] as u32, 0xffff_d80c)); // gp - 0x27f4
    c.add_s(2, 10, 0);
    c.lwc1(4, p(c.g[GP][0] as u32, 0xffff_d810)); // gp - 0x27f0
    c.sub_s(3, 14, 0);
    c.swc1(5, p(sp, 0x78));
    c.add_s(1, 13, 0);
    c.swc1(4, p(sp, 0x7c));
    c.sub_s(0, 11, 0);
    c.swc1(2, p(obj, 0x48));
    c.g[AT][0] = 0x4000_0000;
    c.mtc1(1, 6); // f6 = 2.0
    c.add_s(5, 1, 3);
    c.swc1(1, p(obj, 0x44));
    c.add_s(4, 2, 0);
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 12); // f12 = 1.0
    c.sub_s(1, 1, 3);
    c.swc1(3, p(obj, 0x40));
    c.sub_s(2, 2, 0);
    c.swc1(0, p(obj, 0x4c));
    c.neg_s(5, 5);
    c.sw(p(sp, 0x54), 0u64);
    c.neg_s(4, 4);
    c.sw(p(sp, 0x58), 0u64);
    c.sw(p(sp, 0x60), 0u64);
    c.div_s(5, 5, 1);
    c.sw(p(sp, 0x68), 0u64);
    c.div_s(4, 4, 2);
    c.sw(p(sp, 0x70), 0u64);
    c.div_s(1, 6, 1);
    c.sw(p(sp, 0x74), 0u64);
    c.div_s(2, 6, 2);
    c.sw(p(sp, 0x80), 0u64);
    c.sw(p(sp, 0x84), 0u64);
    c.sw(p(sp, 0x88), 0u64);
    c.swc1(5, p(sp, 0x5c));
    c.swc1(4, p(sp, 0x6c));
    c.swc1(1, p(sp, 0x50));
    c.swc1(2, p(sp, 0x64));
    c.swc1(12, p(sp, 0x8c));
    let (t4, t5, t3) = (c.g[T4][0] as u32, c.g[T5][0] as u32, c.g[T3][0] as u32);
    let v0 = c.g[V0][0] as u32;
    let a2 = c.g[A2][0] as u32;
    transform_matrix(c, t4, t5, t3);
    copy_matrix(c, t3, v0);
    copy_matrix(c, v0, a2);

    // Second pass: light direction rows.
    c.lwc1(3, p(obj, 0x4c));
    c.lwc1(0, p(obj, 0x48));
    c.lwc1(2, p(obj, 0x40));
    c.lwc1(1, p(obj, 0x44));
    c.sub_s(0, 0, 3);
    c.neg_s(5, 2);
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(1, 4); // f4 = -1.0
    c.sub_s(1, 1, 2);
    c.sw(p(sp, 0x54), 0u64);
    c.div_s(3, 3, 0);
    c.sw(p(sp, 0x58), 0u64);
    c.div_s(4, 4, 0);
    c.sw(p(sp, 0x60), 0u64);
    c.div_s(5, 5, 1);
    c.sw(p(sp, 0x68), 0u64);
    c.div_s(1, 12, 1);
    c.sw(p(sp, 0x70), 0u64);
    c.sw(p(sp, 0x74), 0u64);
    c.sw(p(sp, 0x78), 0u64);
    c.add_s(3, 3, 12);
    c.swc1(12, p(sp, 0x7c));
    c.swc1(4, p(sp, 0x64));
    c.swc1(5, p(sp, 0x5c));
    c.swc1(1, p(sp, 0x50));
    c.swc1(3, p(sp, 0x6c));
    c.sw(p(sp, 0x80), 0u64);
    c.sw(p(sp, 0x84), 0u64);
    c.sw(p(sp, 0x88), 0u64);
    c.swc1(12, p(sp, 0x8c));
    let v0 = c.g[V0][0] as u32;
    let t8 = c.g[T8][0] as u32;
    transform_matrix(c, t4, t5, t3);
    copy_matrix(c, t3, v0);
    copy_matrix(c, v0, t8);

    // Third pass: view matrix.
    c.lwc1(1, p(obj, 0x40));
    c.lwc1(0, p(obj, 0x44));
    c.lwc1(2, p(obj, 0x48));
    c.lwc1(4, p(obj, 0x4c));
    c.add_s(3, 0, 1);
    c.sub_s(0, 0, 1);
    c.g[AT][0] = 0xffff_ffff_c000_0000;
    c.mtc1(1, 5); // f5 = -2.0
    c.add_s(1, 2, 4);
    c.swc1(12, p(sp, 0x8c));
    c.sub_s(2, 2, 4);
    c.sw(p(sp, 0x54), 0u64);
    c.neg_s(3, 3);
    c.sw(p(sp, 0x58), 0u64);
    c.div_s(6, 6, 0);
    c.sw(p(sp, 0x60), 0u64);
    c.div_s(1, 1, 2);
    c.sw(p(sp, 0x68), 0u64);
    c.div_s(3, 3, 0);
    c.sw(p(sp, 0x70), 0u64);
    c.div_s(5, 5, 2);
    c.sw(p(sp, 0x74), 0u64);
    c.sw(p(sp, 0x78), 0u64);
    c.swc1(12, p(sp, 0x7c));
    c.swc1(6, p(sp, 0x50));
    c.swc1(1, p(sp, 0x6c));
    c.swc1(3, p(sp, 0x5c));
    c.swc1(5, p(sp, 0x64));
    c.sw(p(sp, 0x80), 0u64);
    c.sw(p(sp, 0x84), 0u64);
    c.sw(p(sp, 0x88), 0u64);
    let t2 = c.g[T2][0] as u32;
    transform_matrix(c, t4, t5, t3);
    copy_matrix(c, t3, v0);
    copy_matrix(c, v0, t2);

    // Depth scale and the six gathered bone rows.
    c.lwc1(1, p(obj, shadow::SCALE_IN));
    c.cvt_s_w(1, 1);
    c.g[AT][0] = 0x3f00_0000;
    c.mtc1(1, 2); // f2 = 0.5
    c.lwc1(0, p(obj, shadow::BIAS_IN));
    c.cvt_s_w(0, 0);
    c.g[AT][0] = 0x4500_0000;
    c.mtc1(1, 3); // f3 = 2048.0
    c.sw(p(obj, 0x16c), 0u64);
    c.mul_s(1, 1, 2);
    c.swc1(3, p(obj, 0x164));
    c.mul_s(0, 0, 2);
    c.sw(p(obj, shadow::DEPTH), 0u64);
    c.sw(p(obj, shadow::DEPTH + 0x4), 0u64);
    c.swc1(1, p(obj, shadow::SCALE));
    c.swc1(0, p(obj, shadow::SCALE + 0x4));
    c.swc1(3, p(obj, 0x160));
    c.sw(p(obj, 0x168), 0u64);

    // Transpose each of the three matrices in place (MMI halfword shuffles).
    let proj = c.g[A2][0] as u32;
    c.lq(9, proj);
    c.lq(8, p(proj, 0x10));
    c.lq(7, p(proj, 0x20));
    c.lq(11, p(proj, 0x30));
    c.pextlw(5, 8, 9);
    c.pextuw(4, 8, 9);
    c.pextlw(3, 11, 7);
    c.pextuw(2, 11, 7);
    c.pcpyld(9, 3, 5);
    c.pcpyud(8, 5, 3);
    c.pcpyld(7, 2, 4);
    c.pcpyud(11, 4, 2);
    c.sq(9, proj);
    c.sq(8, p(proj, 0x10));
    c.sq(7, p(proj, 0x20));
    c.sq(11, p(proj, 0x30));

    let bones = c.g[T2][0] as u32;
    c.lq(17, bones);
    c.lq(16, p(bones, 0x10));
    c.lq(15, p(bones, 0x20));
    c.lq(14, p(bones, 0x30));
    c.pextlw(13, 16, 17);
    c.pextuw(12, 16, 17);
    c.pextlw(11, 14, 15);
    c.pextuw(2, 14, 15);
    c.pcpyld(17, 11, 13);
    c.pcpyud(16, 13, 11);
    c.pcpyld(15, 2, 12);
    c.pcpyud(14, 12, 2);
    c.sq(17, bones);
    c.sq(16, p(bones, 0x10));
    c.sq(15, p(bones, 0x20));
    c.sq(14, p(bones, 0x30));

    let view = c.g[T8][0] as u32;
    c.lq(9, view);
    c.lq(8, p(view, 0x10));
    c.lq(7, p(view, 0x20));
    c.lq(6, p(view, 0x30));
    c.pextlw(5, 8, 9);
    c.pextuw(4, 8, 9);
    c.pextlw(3, 6, 7);
    c.pextuw(2, 6, 7);
    c.pcpyld(9, 3, 5);
    c.pcpyud(8, 5, 3);
    c.pcpyld(7, 2, 4);
    c.pcpyud(6, 4, 2);
    c.sq(9, view);
    c.sq(8, p(view, 0x10));
    c.sq(7, p(view, 0x20));
    c.sq(6, p(view, 0x30));

    // Restore the callee-saved registers and the frame.
    c.lq(16, p(sp, 0x120));
    c.lq(17, p(sp, 0x110));
    c.lwc1(31, p(sp, 0x188));
    c.lwc1(30, p(sp, 0x180));
    c.lwc1(29, p(sp, 0x178));
    c.lwc1(28, p(sp, 0x170));
    c.lwc1(27, p(sp, 0x168));
    c.lwc1(26, p(sp, 0x160));
    c.lwc1(25, p(sp, 0x158));
    c.lwc1(24, p(sp, 0x150));
    c.lwc1(23, p(sp, 0x148));
    c.lwc1(22, p(sp, 0x140));
    c.lwc1(21, p(sp, 0x138));
    c.lwc1(20, p(sp, 0x130));
    c.g[SP][0] = sx(p(sp, 0x190));
    abi::ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fill, scramble, Rng, DATA, GAME_GP};

    fn put_vec(c: &mut Cpu, addr: u32, x: u32, y: u32, z: u32) {
        c.bus.write32(addr, x);
        c.bus.write32(addr + 4, y);
        c.bus.write32(addr + 8, z);
    }

    /// `BoaderShadow_ComputeProjection`: object at `DATA` (holds the input
    /// vectors after the first pass), the three vector arguments, a
    /// six-entry bone table, and the globals/constants it reads. The seed
    /// picks zero, parallel, antiparallel and random input vectors so every
    /// zero-length and parallel branch is taken.
    #[test]
    fn boader_shadow_compute_projection_matches() {
        check(
            ssx3_ee::s12::f_00374d00__generated,
            boader_shadow_compute_projection,
            240,
            &|c, seed| {
                let mut r = Rng(0x374d_00 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP.wrapping_sub(0x27f8), r.edge_f32());
                c.bus.write32(GAME_GP.wrapping_sub(0x27f4), r.edge_f32());
                c.bus.write32(GAME_GP.wrapping_sub(0x27f0), r.edge_f32());
                // Fixed data the original loads by absolute address.
                c.bus.write32(0x004a_431c, r.edge_f32());
                c.bus.write32(0x004a_4320, r.edge_f32());

                let obj = DATA;
                let a1 = DATA + 0x400;
                let a2 = DATA + 0x410;
                let a3 = DATA + 0x420;
                let bones = DATA + 0x800;
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx(a1);
                c.g[A2][0] = sx(a2);
                c.g[A3][0] = sx(a3);
                c.g[T0][0] = sx(bones);

                // Junk in every object field first, so a wrong offset in the
                // port cannot read a neighbour that happens to be zero too.
                fill(c, &mut r, obj, 0x68);

                // `b` (a2), `dir` (a3) and the reference vector (a1). The
                // structured seeds walk every parallel / antiparallel branch
                // (and the cycle charges they take); the rest are random.
                const ONE: u32 = 0x3f80_0000;
                const TWO: u32 = 0x4000_0000;
                const NEG_ONE: u32 = 0xbf80_0000;
                let (bx, by, bz, dx, dy, dz) = match seed % 9 {
                    0 => (0, 0, 0, 0, 0, 0),
                    // b == dir: the first parallel test sets v0.
                    1 => (ONE, ONE, ONE, ONE, ONE, ONE),
                    // b == -dir: only the second parallel test (and its +1) runs.
                    2 => (NEG_ONE, NEG_ONE, NEG_ONE, ONE, ONE, ONE),
                    // x equal (both zero), y different: the +4-only path of
                    // the first test (and of the second test's first compare).
                    3 => (0, ONE, TWO, 0, ONE, ONE),
                    // x opposite, y equal: the +4-only path of the second test.
                    4 => (NEG_ONE, ONE, ONE, ONE, ONE, ONE),
                    // x/y equal, z different: the third comparison fails.
                    5 => (ONE, ONE, NEG_ONE, ONE, ONE, ONE),
                    6 => (r.edge_f32(), r.edge_f32(), r.edge_f32(), 0, 0, 0),
                    7 => (0, 0, 0, r.edge_f32(), r.edge_f32(), r.edge_f32()),
                    _ => (
                        r.edge_f32(),
                        r.edge_f32(),
                        r.edge_f32(),
                        r.edge_f32(),
                        r.edge_f32(),
                        r.edge_f32(),
                    ),
                };
                put_vec(c, a2, bx, by, bz);
                put_vec(c, a3, dx, dy, dz);
                put_vec(c, a1, r.edge_f32(), r.edge_f32(), r.edge_f32());

                // Six bone indices and the integer depth inputs of the object.
                for i in 0..6u32 {
                    c.bus.write32(obj + 0x17c + i * 4, i);
                }
                let depth = if seed % 2 == 0 {
                    r.next_u32()
                } else {
                    r.next_u32() % 0x800
                };
                c.bus.write32(obj + 0x174, depth);
                c.bus.write32(obj + 0x178, r.next_u32() % 0x800);

                // The bone table (six 0x40-byte entries, row read at +0x30).
                fill(c, &mut r, bones, 0x80);
            },
        );
    }
}
