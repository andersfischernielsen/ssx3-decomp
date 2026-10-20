//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for the rider/board visual effects — the snow box wrap, the
//! spray FX random velocity, the board wake/spark strips and the rider power-up
//! trail and anger FX — each with a differential test against the generated
//! body.
//!
//! The effects are small scene-graph clients: they read the rider/board state,
//! push a renderer state (`cGfx` at gp-0x854), set a vertex format and draw a
//! strip through a virtual method of the renderer's vtable, or feed the
//! dynamic-particle emitter (0x3717c0). The replacements keep every register
//! the originals leave behind (argument registers at the virtual calls are
//! part of the observed state), issue VU0 operations as the originals do and
//! keep the same cycle cost, per the guide.

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, sp, Frame, Flow};
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, X};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002e_5430, snow_box_wrap_offset, "SnowBox_WrapOffset"),
    (0x002d_e398, dynamic_spray_fx_add_random_velocity, "cDynamicSprayFX_AddRandomVelocity"),
    (0x002d_dab8, board_wake_fx_render, "cBoardWakeFX_Render"),
];

// ---------------------------------------------------------------------------
// Small helpers.

/// `a + off` (32-bit address arithmetic).
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `lwc1 f, a`, returning the value.
#[inline(always)]
fn ldf(c: &mut Cpu, f: usize, a: u32) -> FpuF32 {
    c.lwc1(f, a);
    FpuF32(c.f[f])
}

/// FPU register `f`.
#[inline(always)]
fn fr(c: &Cpu, f: usize) -> FpuF32 {
    FpuF32(c.f[f])
}

/// Set FPU register `f`.
#[inline(always)]
fn setf(c: &mut Cpu, f: usize, v: FpuF32) {
    c.f[f] = v.0;
}

/// `lui at, hi; mtc1 at, f`: a float constant through `at`.
#[inline(always)]
fn fconst(c: &mut Cpu, f: usize, bits: u32) -> FpuF32 {
    c.g[AT][0] = sx(bits);
    c.f[f] = bits;
    FpuF32(bits)
}

/// `x + y` of two words (`addu`).
#[inline(always)]
fn add32(x: u64, y: u64) -> u64 {
    sx((x as u32).wrapping_add(y as u32))
}

/// `sll x, n` (32-bit, sign-extended).
#[inline(always)]
fn shl32(x: u64, n: u32) -> u64 {
    sx((x as u32) << n)
}

/// `lq gpr, from; sq gpr, to` (a quadword moved through a GPR).
#[inline(always)]
fn copy_q(c: &mut Cpu, gpr: usize, from: u32, to: u32) {
    c.lq(gpr, from);
    c.sq(gpr, to);
}

/// `lqc2 vf3, a; lqc2 vf4, b; vadd.xyzw vf5, vf3, vf4; sqc2 vf5, out`.
#[inline(always)]
fn vadd(c: &mut Cpu, a: u32, b: u32, out: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, out);
}

/// `lqc2 vf3, a; lqc2 vf4, b; vsub.xyzw vf5, vf3, vf4; sqc2 vf5, out`.
#[inline(always)]
fn vsub(c: &mut Cpu, a: u32, b: u32, out: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, out);
}

/// `lqc2 vf4, v; qmtc2 rt, vf3; vmulx.xyzw vf5, vf4, vf3x; sqc2 vf5, out`:
/// a vector scaled by the float bits held in GPR `rt`.
#[inline(always)]
fn vscale(c: &mut Cpu, v: u32, rt: usize, out: u32) {
    c.lqc2(4, v);
    c.qmtc2(rt, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, out);
}

/// The common epilogue tail of a leaf: `addiu sp, sp, size; jr ra`.
#[inline(always)]
fn leave(c: &mut Cpu, size: u32) {
    let ra = r32(c, RA);
    c.g[SP][0] = sx(sp(c).wrapping_add(size));
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// Snow box wrapping.

/// `SnowBox` fields used by the wrap.
mod snow_box {
    /// Side length of the snow grid cell.
    pub const SIZE: u32 = 0x0c;
    /// Position (x, y, z, w), already offset by the camera-relative shift.
    pub const POS: u32 = 0x20;
}

/// The `fmodf`-style remainder the original writes: `q = v / s` truncated
/// toward zero, result `(q - trunc(q)) * s`.
#[inline(always)]
fn frac_times(c: &mut Cpu, v: FpuF32, s: FpuF32) -> FpuF32 {
    let q = v.div(s);
    setf(c, 0, q);
    c.cvt_w_s(1, 0);
    c.mfc1(2, 1);
    c.mtc1(2, 1);
    c.cvt_s_w(1, 1);
    let trunc = fr(c, 1);
    let f = q - trunc;
    setf(c, 0, f);
    let r = f * s;
    setf(c, 0, r);
    r
}

/// `void SnowBox_WrapOffset(SnowBox* box, const Vec4* d)` (0x2e5430):
/// `box->pos -= *d`, then wrap each component into `(-size/2, size/2]` by the
/// fractional part of the division (and store `w = 1`).
///
/// The three components are wrapped by the same inlined code; the original
/// lays them out as three sibling blocks, each entering the next one.
pub fn snow_box_wrap_offset(c: &mut Cpu) {
    let this = abi::arg(c, 0);
    let d = abi::arg(c, 1);
    c.cycles += 17;
    c.lqc2(3, p(this, snow_box::POS));
    c.lqc2(4, d);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, p(this, snow_box::POS));
    fconst(c, 0, 0x3f00_0000); // f0 = 0.5
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x10));
    ldf(c, 2, p(this, snow_box::SIZE));
    ldf(c, 3, p(this, snow_box::POS));
    c.mul_s(0, 2, 0); // f0 = h = size * 0.5
    ldf(c, 6, p(this, snow_box::POS + 4));
    ldf(c, 7, p(this, snow_box::POS + 8));
    c.mov_s(4, 6); // f4 = original y
    c.c_lt_s(0, 3); // h < x ?
    let x_above = c.fcc();
    c.mov_s(5, 7); // f5 = original z
    // (x, y, z results are left in f3, f6, f7 as the original does.)
    'x: {
        'x_tail: {
            'x_inner: {
                if x_above {
                    c.cycles += 9;
                    let v = fr(c, 3);
                    let s = fr(c, 2);
                    let r = frac_times(c, v, s);
                    setf(c, 3, r - s);
                    break 'x_tail;
                }
            }
            c.cycles += 5;
            c.neg_s(0, 0); // f0 = -h
            c.c_lt_s(3, 0); // x < -h ?
            if !c.fcc() {
                c.lwc1(0, p(this, snow_box::SIZE));
                break 'x;
            }
            c.cycles += 8;
            let v = fr(c, 3);
            let s = fr(c, 2);
            let r = frac_times(c, v, s);
            setf(c, 3, r + s);
            break 'x_tail;
        }
        c.cycles += 1;
        c.lwc1(0, p(this, snow_box::SIZE));
    }
    'y: {
        'y_inner: {
            c.cycles += 8;
            fconst(c, 1, 0x3f00_0000);
            c.mov_s(2, 0); // f2 = size
            c.mul_s(0, 0, 1); // f0 = h
            c.c_lt_s(0, 4); // h < y ?
            if !c.fcc() {
                c.neg_s(0, 0);
                break 'y_inner;
            }
            c.cycles += 9;
            let v = fr(c, 4);
            let s = fr(c, 2);
            let r = frac_times(c, v, s);
            setf(c, 6, r - s);
            break 'y;
        }
        c.cycles += 3;
        c.c_lt_s(4, 0); // y < -h ?
        let t = !c.fcc();
        if t {
            break 'y;
        }
        c.cycles += 8;
        let v = fr(c, 4);
        let s = fr(c, 2);
        let r = frac_times(c, v, s);
        setf(c, 6, r + s);
        break 'y;
    }
    'z: {
        'z_inner: {
            c.cycles += 7;
            fconst(c, 0, 0x3f00_0000);
            c.mul_s(0, 2, 0); // f0 = h
            c.c_lt_s(0, 5); // h < z ?
            if !c.fcc() {
                c.neg_s(0, 0);
                break 'z_inner;
            }
            c.cycles += 9;
            let v = fr(c, 5);
            let s = fr(c, 2);
            let r = frac_times(c, v, s);
            setf(c, 7, r - s);
            break 'z;
        }
        c.cycles += 3;
        c.c_lt_s(5, 0); // z < -h ?
        let t = !c.fcc();
        if t {
            break 'z;
        }
        c.cycles += 8;
        let v = fr(c, 5);
        let s = fr(c, 2);
        let r = frac_times(c, v, s);
        setf(c, 7, r + s);
        break 'z;
    }
    c.cycles += 10;
    fconst(c, 0, 0x3f80_0000); // w = 1
    let sp = sp(c);
    c.swc1(3, sp);
    c.swc1(6, p(sp, 4));
    c.swc1(7, p(sp, 8));
    c.swc1(0, p(sp, 0xc));
    copy_q(c, 2, sp, p(this, snow_box::POS));
    leave(c, 0x10);
}

// ---------------------------------------------------------------------------
// Spray FX random velocity.

/// `void cDynamicSprayFX_AddRandomVelocity(SprayFX* fx, Vec4* out)`
/// (0x2de398): add two random vectors to `*out` — one along `fx->dir0`
/// (+0x50) scaled by `phase * 55 + 55 * rand +-`, one along `fx->dir1`
/// (+0x40) with `rand * 20 - 10`. The random source is the LCG word at
/// gp+0xa0c (`state = state * 0x18fcd + 0xe9507c`, mantissa bits give a
/// float in [1, 2)).
pub fn dynamic_spray_fx_add_random_velocity(c: &mut Cpu) {
    let this = abi::arg(c, 0);
    let out = abi::arg(c, 1);
    let state_at = gp(c, 0xa0c);
    c.cycles += 68;
    c.g[V0][0] = c.lw(state_at);
    c.g[A3][0] = 0x0001_8fcd;
    ldf(c, 3, p(this, 0x12c)); // phase (-1..1)
    fconst(c, 4, 0x3f80_0000);
    c.mult(0, 2, 2, 7); // state *= 0x18fcd
    c.abs_s(1, 3);
    c.g[A2][0] = 0x00e9_507c;
    c.sub_s(1, 4, 1); // 1 - |phase|
    fconst(c, 0, 0x425c_0000); // 55
    c.g[V1][0] = 0x007f_ffff;
    c.g[V0][0] = add32(c.g[V0][0], c.g[A2][0]);
    c.g[T0][0] = 0x3f80_0000;
    c.mul_s(1, 1, 0); // a = (1 - |phase|) * 55
    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
    c.mul_s(3, 3, 0); // phase * 55
    c.g[V0][0] = c.g[V0][0] | c.g[T0][0];
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
    c.sw(state_at, c.g[V0][0]);
    c.neg_s(2, 1); // lo = -a
    fconst(c, 6, 0x41a0_0000); // 20
    c.lwc1(0, state_at); // f0 in [1, 2)
    c.sub_s(1, 1, 2); // a - lo
    fconst(c, 5, 0xc120_0000); // -10
    c.sub_s(0, 0, 4); // rand - 1
    c.mul_s(1, 1, 0);
    c.add_s(2, 2, 1); // lo + (a - lo) * rand
    c.add_s(3, 3, 2); // phase * 55 + ...
    c.mfc1(9, 3);
    c.lqc2(4, p(this, 0x50));
    c.qmtc2(9, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, p(sp(c), 0x10));
    copy_q(c, 2, p(sp(c), 0x10), sp(c));
    vadd(c, out, sp(c), out);
    c.g[V0][0] = c.lw(state_at);
    c.mult(0, 9, 2, 7);
    c.g[V0][0] = add32(c.g[T1][0], c.g[A2][0]);
    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
    c.g[V0][0] = c.g[V0][0] | c.g[T0][0];
    c.sw(state_at, c.g[V0][0]);
    c.lwc1(0, state_at);
    c.sub_s(0, 0, 4); // rand - 1
    c.mul_s(0, 0, 6); // * 20
    c.add_s(0, 0, 5); // - 10
    c.mfc1(3, 0);
    c.lqc2(4, p(this, 0x40));
    c.qmtc2(3, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, p(sp(c), 0x10));
    copy_q(c, 2, p(sp(c), 0x10), sp(c));
    vadd(c, out, sp(c), out);
    leave(c, 0x20);
}

// ---------------------------------------------------------------------------
// Board wake strip rendering.

/// The renderer object (`cGfx`) at gp-0x854.
mod gfx {
    /// Top of the 20-byte state stack.
    pub const STATE_TOP: u32 = 0xe84;
    /// The renderer's frame counter (`Get1030`).
    pub const F1030: u32 = 0x1030;
    /// The renderer's virtual-method table.
    pub const VTABLE: u32 = 0x10d8;
}

/// `sWake_DAB8`: the board wake trail.
mod wake_dab8 {
    pub const COUNT: u32 = 0x04;
    pub const FIRST: u32 = 0x08;
    /// Pointers to the 0x70-byte trail records (age at +0x64).
    pub const VERTS: u32 = 0x0c;
    /// The strip vertex array drawn by the renderer.
    pub const STRIP: u32 = 0x10;
    pub const TEX: u32 = 0xb4;
    pub const MAX: u32 = 0xb8;
    pub const LIFE: u32 = 0xbc;
    pub const VERTEX_STRIDE: u32 = 0x70;
    pub const VERTEX_AGE: u32 = 0x64;
}

/// The renderer's default state record copied by the original (`D_00501420`).
const WAKE_STATE_RESET: u32 = 0x0050_1420;

/// `void cBoardWakeFX_Render(Wake* self)` (0x2ddab8): push a renderer state,
/// copy the default wake state into it, fill in the vertex format, then draw
/// the trail strip (alpha from the age of the newest point) through the
/// renderer's virtual `DrawStrip`.
pub fn board_wake_fx_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = c.lw(gp(c, 0x1438));
        frame.save(c, S0, 0x30);
        c.g[T8][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        let self_at = r32(c, T8);
        frame.save_ra(c, 0);
        'out: {
            if c.g[V0][0] != 0 {
                break 'out;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw(p(self_at, wake_dab8::COUNT));
            c.g[V1][0] = c.lw(gp(c, -0x854));
            if c.g[V0][0] == 0 {
                break 'out;
            }
            // PushState: copy the current 20-byte state to the new top and
            // advance it.
            c.cycles += 106;
            c.g[T3][0] = 0x0050_0000;
            c.g[T4][0] = sx(0xffff_fff3); // ~0xc
            c.g[A1][0] = 0xffff_ffff_ffff_bfff; // clear bit 22
            c.g[V0][0] = c.lw(p(r32(c, V1), gfx::STATE_TOP));
            c.g[T5][0] = 0x0040_0000;
            c.g[A3][0] = 0xffff_ffff_fe7f_ffff; // clear bit 23
            c.ldl(4, p(r32(c, V0), 7));
            c.ldr(4, p(r32(c, V0), 0));
            c.ldl(6, p(r32(c, V0), 0xf));
            c.ldr(6, p(r32(c, V0), 8));
            c.g[T0][0] = c.lw(p(r32(c, V0), 0x10));
            c.sdl(4, p(r32(c, V0), 0x1b));
            c.sdr(4, p(r32(c, V0), 0x14));
            c.sdl(6, p(r32(c, V0), 0x23));
            c.sdr(6, p(r32(c, V0), 0x1c));
            c.sw(p(r32(c, V0), 0x24), c.g[T0][0]);
            c.g[T0][0] = 0xffff_ffff_ffcf_ffff; // clear bits 20, 21
            c.g[T1][0] = 0xffff_ffff_fff0_0fff; // clear bits 12..19
            c.g[V0][0] = c.lw(p(r32(c, V1), gfx::STATE_TOP));
            c.g[A0][0] = c.lw(gp(c, -0x854));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x14));
            c.g[A2][0] = 0x0001_4000; // 0x14 in the f4_12 field
            c.sw(p(r32(c, V1), gfx::STATE_TOP), c.g[V0][0]);
            c.g[T6][0] = sx(0xffff_fffc); // clear bits 0, 1
            c.g[T2][0] = sx(0xe000_03ff); // clear bits 10..28
            c.g[V0][0] = c.lw(p(r32(c, A0), gfx::STATE_TOP));
            c.g[T7][0] = sx(0xffff_fc1f); // clear bits 5..9
            c.g[S0][0] = sx(0xffff_ff83); // clear bits 2..6
            c.g[A0][0] = sx(r32(c, T3).wrapping_add(0x1420));
            c.ldl(17, p(r32(c, A0), 7));
            c.ldr(17, p(r32(c, A0), 0));
            c.ldl(18, p(r32(c, A0), 0xf));
            c.ldr(18, p(r32(c, A0), 8));
            c.g[V1][0] = c.lw(p(r32(c, A0), 0x10));
            c.sdl(17, p(r32(c, V0), 7));
            c.sdr(17, p(r32(c, V0), 0));
            c.sdl(18, p(r32(c, V0), 0xf));
            c.sdr(18, p(r32(c, V0), 8));
            c.sw(p(r32(c, V0), 0x10), c.g[V1][0]);
            c.g[S1][0] = 0x70; // trail record stride
            ldf(c, 2, gp(c, -0x3be8)); // 0.8: the fade threshold
            c.g[V0][0] = c.lw(gp(c, -0x854));
            fconst(c, 12, 0x3fa0_0000); // k = 1.25
            c.g[A0][0] = c.lw(p(r32(c, V0), gfx::STATE_TOP));
            c.g[V1][0] = c.lw(p(r32(c, V0), gfx::F1030));
            fconst(c, 3, 0x3f80_0000); // a = 1.0
            c.sh(p(r32(c, A0), 0x10), c.g[V1][0]);
            // f0_2 = 0 (clear bits 2, 3)
            c.g[T3][0] = c.lw(gp(c, -0x854));
            c.g[V1][0] = c.lw(p(r32(c, T3), gfx::STATE_TOP));
            c.g[V0][0] = c.lw(r32(c, V1));
            c.g[V0][0] = c.g[V0][0] & c.g[T4][0];
            c.sw(r32(c, V1), c.g[V0][0]);
            // f4_22 = 1
            c.g[A0][0] = c.lw(p(r32(c, T3), gfx::STATE_TOP));
            c.g[V0][0] = c.lw(p(r32(c, A0), 4));
            c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
            c.g[V0][0] = c.g[V0][0] | c.g[T5][0];
            c.sw(p(r32(c, A0), 4), c.g[V0][0]);
            // f4_23 = 0
            c.g[V1][0] = c.lw(p(r32(c, T3), gfx::STATE_TOP));
            c.g[V0][0] = c.lw(p(r32(c, V1), 4));
            c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
            c.sw(p(r32(c, V1), 4), c.g[V0][0]);
            // f4_20 = 0
            c.g[A0][0] = c.lw(p(r32(c, T3), gfx::STATE_TOP));
            c.g[V0][0] = c.lw(p(r32(c, A0), 4));
            c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
            c.sw(p(r32(c, A0), 4), c.g[V0][0]);
            // f4_12 = 0x14
            c.g[V1][0] = c.lw(p(r32(c, T3), gfx::STATE_TOP));
            c.g[V0][0] = c.lw(p(r32(c, V1), 4));
            c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
            c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
            c.sw(p(r32(c, V1), 4), c.g[V0][0]);
            // f4_0 = 2
            c.g[A0][0] = c.lw(p(r32(c, T3), gfx::STATE_TOP));
            c.g[V0][0] = c.lw(p(r32(c, A0), 4));
            c.g[V0][0] = c.g[V0][0] & c.g[T6][0];
            c.g[V0][0] = c.g[V0][0] | 2;
            c.sw(p(r32(c, A0), 4), c.g[V0][0]);
            // f8_10 = 0
            c.g[V1][0] = c.lw(p(r32(c, T3), gfx::STATE_TOP));
            c.g[V0][0] = c.lw(p(r32(c, V1), 8));
            c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
            c.sw(p(r32(c, V1), 8), c.g[V0][0]);
            // f8_5 = 7
            c.g[A0][0] = c.lw(p(r32(c, T3), gfx::STATE_TOP));
            c.g[V0][0] = c.lw(p(r32(c, A0), 8));
            c.g[V0][0] = c.g[V0][0] & c.g[T7][0];
            c.g[V0][0] = c.g[V0][0] | 0xe0;
            c.sw(p(r32(c, A0), 8), c.g[V0][0]);
            // f4_2 = 5
            c.g[V1][0] = c.lw(p(r32(c, T3), gfx::STATE_TOP));
            c.g[V0][0] = c.lw(p(r32(c, V1), 4));
            c.g[V0][0] = c.g[V0][0] & c.g[S0][0];
            c.g[V0][0] = c.g[V0][0] | 0x14;
            c.sw(p(r32(c, V1), 4), c.g[V0][0]);
            // draw = count, over = count + 2 - max, draw -= over when over >= 1
            c.g[T1][0] = c.lw(p(self_at, wake_dab8::COUNT));
            c.g[A3][0] = c.lw(p(self_at, wake_dab8::MAX));
            c.g[T2][0] = c.g[T1][0];
            c.g[T0][0] = c.lw(p(self_at, wake_dab8::MAX));
            c.g[V0][0] = add32(c.g[A3][0], c.g[T2][0]); // max + n
            c.g[V1][0] = sx(r32(c, T1).wrapping_add(2)); // over = n + 2
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1)); // max + n - 1
            // last = (max + n - 1) % max; draw = n - over (if over >= 1)
            c.cycles += 16;
            c.div(0, 2, 8);
            c.g[A2][0] = c.lw(p(self_at, wake_dab8::VERTS));
            ldf(c, 1, p(self_at, wake_dab8::LIFE));
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(r32(c, T0)));
            c.g[A1][0] = ((c.g[V1][0] as i64) < 1) as u64;
            c.g[V1][0] = sx(r32(c, T1).wrapping_sub(r32(c, V1)));
            c.g[A0][0] = c.hi[0];
            c.mult(0, 4, 4, 17);
            c.g[A0][0] = add32(c.g[A0][0], c.g[A2][0]);
            ldf(c, 0, p(r32(c, A0), wake_dab8::VERTEX_AGE));
            c.sub_s(0, 1, 0); // life - age
            c.div_s(0, 0, 1); // t
            c.c_le_s(0, 2); // t <= 0.8
            let faded = !c.fcc();
            if c.g[A1][0] == 0 {
                c.g[T1][0] = c.g[V1][0];
            }
            if faded {
                // a stays 1.0 (f3)
            } else {
                c.cycles += 1;
                c.mul_s(3, 0, 12); // a = t * 1.25
            }
            // da = (a - k) / (float)count
            c.cycles += 7;
            c.sub_s(0, 3, 12);
            c.g[V0][0] = ((c.g[T1][0] as i64) < 2) as u64;
            c.mtc1(10, 1); // count (t2)
            c.cvt_s_w(1, 1);
            c.div_s(13, 0, 1);
            let few = c.g[V0][0] != 0;
            c.g[V1][0] = c.lw(gp(c, -0x854));
            if !few {
                c.cycles += 8;
                c.g[V0][0] = c.lw(p(r32(c, T3), gfx::VTABLE));
                c.g[A1][0] = sx(r32(c, T8).wrapping_add(wake_dab8::STRIP));
                c.g[A2][0] = c.lw(p(self_at, wake_dab8::TEX));
                c.g[A0][0] = c.lh(p(r32(c, V0), 0x2d0));
                c.g[V1][0] = c.lw(p(r32(c, V0), 0x2d4));
                let target = r32(c, V1);
                c.g[A0][0] = sx(r32(c, T3).wrapping_add(r32(c, A0)));
                call_ptr(c, target, 0x002d_dd08)?;
                c.g[V1][0] = c.lw(gp(c, -0x854));
            }
            // stateTop--
            c.cycles += 3;
            c.g[V0][0] = c.lw(p(r32(c, V1), gfx::STATE_TOP));
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x14));
            c.sw(p(r32(c, V1), gfx::STATE_TOP), c.g[V0][0]);
            break 'out;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, FAKE};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP, SP as TEST_SP};

    const SEEDS: u32 = 250;

    fn f(x: f32) -> u32 {
        x.to_bits()
    }

    /// Scrambled registers and the game's gp.
    fn base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ (seed as u64) << 8 ^ salt);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        r
    }

    fn gp_at(off: i32) -> u32 {
        GAME_GP.wrapping_add(off as u32)
    }

    #[test]
    fn snow_box_wrap_offset_matches() {
        check(ee::s09::f_002e5430__generated, snow_box_wrap_offset, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x2e54);
            let (b, d) = (DATA, DATA + 0x40);
            let size = 1.0 + (r.next_u32() % 200) as f32 * 0.25;
            let h = size * 0.5;
            c.bus.write32(b + 0xc, f(size));
            // One position per component with its own wrap case (in range,
            // above +h, below -h), so the seeds cover every branch triple.
            for k in 0..3u32 {
                let case = (seed / (3u32.pow(k))) % 3;
                let v = match case {
                    0 => (r.next_u32() as f32 / u32::MAX as f32 - 0.5) * h * 1.5,
                    1 => h + 1.0 + (r.next_u32() % 10) as f32,
                    _ => -h - 1.0 - (r.next_u32() % 10) as f32,
                };
                c.bus.write32(b + 0x20 + 4 * k, f(v));
                c.bus.write32(d + 4 * k, 0);
            }
            c.bus.write32(d + 0xc, f(0.0));
            c.g[A0][0] = sx(b);
            c.g[A1][0] = sx(d);
        });
    }

    #[test]
    fn dynamic_spray_fx_add_random_velocity_matches() {
        check(ee::s09::f_002de398__generated, dynamic_spray_fx_add_random_velocity, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 0x2de3);
            let (fx, out) = (DATA, DATA + 0x100);
            for k in 0..4u32 {
                c.bus.write32(fx + 0x40 + 4 * k, r.edge_f32());
                c.bus.write32(fx + 0x50 + 4 * k, r.edge_f32());
                c.bus.write32(out + 4 * k, r.edge_f32());
            }
            c.bus.write32(fx + 0x12c, r.edge_f32());
            c.bus.write32(gp_at(0xa0c), r.next_u32());
            c.g[A0][0] = sx(fx);
            c.g[A1][0] = sx(out);
        });
    }

    #[test]
    fn board_wake_fx_render_matches() {
        check(ee::s09::f_002ddab8__generated, board_wake_fx_render, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2dda, 1);
            let (w, gfx, state, vt, verts) =
                (DATA, DATA + 0x1000, DATA + 0x2000, DATA + 0x3000, DATA + 0x4000);
            // The renderer and its state stack.
            c.bus.write32(gp_at(-0x854), gfx);
            c.bus.write32(gfx + 0xe84, state);
            c.bus.write32(gfx + 0x1030, r.next_u32());
            c.bus.write32(gfx + 0x10d8, vt);
            c.bus.write16(vt + 0x2d0, (r.next_u32() & 0x7c) as u16);
            c.bus.write32(vt + 0x2d4, FAKE);
            // The default state the renderer copies.
            for k in 0..5u32 {
                c.bus.write32(0x0050_1420 + 4 * k, r.next_u32());
            }
            // The wake trail.
            c.bus.write32(w + 0x4, 1 + r.next_u32() % 4);
            c.bus.write32(w + 0x8, r.next_u32() % 4);
            c.bus.write32(w + 0xc, verts);
            c.bus.write32(w + 0xb4, r.next_u32());
            c.bus.write32(w + 0xb8, 2 + r.next_u32() % 3);
            c.bus.write32(w + 0xbc, f(0.5 + (r.next_u32() % 100) as f32 * 0.1));
            for k in 0..4u32 {
                c.bus.write32(verts + 0x70 * k + 0x64, f((r.next_u32() % 100) as f32 * 0.01));
            }
            // Cover the early-outs too.
            if seed % 11 == 0 {
                c.bus.write32(w + 0x4, 0);
            }
            c.bus.write32(gp_at(0x1438), if seed % 13 == 0 { 1 } else { 0 });
            c.g[A0][0] = sx(w);
        });
    }
}
