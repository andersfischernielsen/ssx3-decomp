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
use crate::call::{call_ptr, guest, sp, Callee, Frame, Flow};
use crate::events::{deliver_event_here, event_due};
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002e_5430, snow_box_wrap_offset, "SnowBox_WrapOffset"),
    (0x002d_e398, dynamic_spray_fx_add_random_velocity, "cDynamicSprayFX_AddRandomVelocity"),
    (0x002d_dab8, board_wake_fx_render, "cBoardWakeFX_Render"),
    (0x002e_0ee8, dynamic_spray_fx_update_emitter0, "cDynamicSprayFX_UpdateEmitter0"),
    (0x002d_b478, board_spark_fx_render, "cBoardSparkFX_Render"),
    (0x002e_55d8, snow_box_render, "SnowBox_Render"),
    (0x002e_add0, rider_base_update_power_up_trail, "RiderBase_UpdatePowerUpTrail"),
    (0x002d_4c08, anger_fx_update, "cAngerFX_Update"),
    (0x002e_1598, dynamic_spray_fx_update_emitter5, "cDynamicSprayFX_UpdateEmitter5"),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const ADD_RANDOM_VELOCITY: Callee = Callee {
        addr: 0x002d_e398,
        f: ee::s09::cDynamicSprayFX_AddRandomVelocity,
    };
    pub const ADD_SPLASH: Callee = Callee {
        addr: 0x002f_4118,
        f: ee::s09::cSplashManager_AddSplashAllViews,
    };
}

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
    let stack = sp(c);
    copy_q(c, 2, p(stack, 0x10), stack);
    vadd(c, out, stack, out);
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
    let stack = sp(c);
    copy_q(c, 2, p(stack, 0x10), stack);
    vadd(c, out, stack, out);
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
            c.g[A1][0] = 0xffff_ffff_ffbf_ffff; // clear bit 22
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
            c.g[A3][0] = c.lw(p(self_at, wake_dab8::FIRST));
            c.g[T2][0] = c.g[T1][0];
            c.g[T0][0] = c.lw(p(self_at, wake_dab8::MAX));
            c.g[V0][0] = add32(c.g[A3][0], c.g[T2][0]); // first + n
            c.g[V1][0] = sx(r32(c, T1).wrapping_add(2)); // over = n + 2
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1)); // first + n - 1
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
// Spray FX emitter update (the "speed spray" emitter).

/// `cDynamicSprayFX` fields read by the emitter update.
mod spray {
    /// The rider object (velocity source, up vector, trail point).
    pub const OBJ: u32 = 0x00;
    /// The dynamic emitter the spray feeds (`self->em`).
    pub const EMITTER: u32 = 0xb0;
    /// The emitter object pointer used for the position (`self->p28`).
    pub const EMITTER_PTR: u32 = 0x28;
    /// Curve table, +0xd8/+0xdc/+0xe0 are velocity scales.
    pub const CURVES: u32 = 0x2c;
    /// Spray direction.
    pub const DIR: u32 = 0x30;
    /// Colour (RGBA).
    pub const COL: u32 = 0x90;
    /// Position.
    pub const POS: u32 = 0xa0;
    /// Normalised speed (0..~2): gates and scales the spray.
    pub const SPEED: u32 = 0xbc;
    /// Configuration object, +0x58 enables the spray.
    pub const CFG: u32 = 0xd0;
}

/// `void cDynamicSprayFX_UpdateEmitter0(SprayFX* fx)` (0x2e0ee8): when the
/// spray is enabled, fast enough and the rider is not in some special mode,
/// spawn a particle 15 units along the spray direction, with the rider's
/// trail point and up-vector velocity; otherwise just advance the emitter at
/// the spray position.
pub fn dynamic_spray_fx_update_emitter0(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x80);
        let self_at = r32(c, S0);
        let sp0 = sp(c);
        c.g[V0][0] = c.lw(self_at);
        c.g[A0][0] = c.lw(p(self_at, spray::EMITTER));
        c.g[V1][0] = c.lw(p(r32(c, V0), 0x88c));
        c.g[S1][0] = c.lw(p(self_at, spray::EMITTER_PTR));
        let no_emitter = c.g[A0][0] == 0;
        c.g[A1][0] = c.lw(p(r32(c, V1), 0xa0));
        'out: {
            'on: {
                if no_emitter {
                    break 'on;
                }
                c.cycles += 5;
                c.lwc1(1, gp(c, -0x3afc)); // speed limit
                c.lwc1(0, p(self_at, spray::SPEED));
                c.c_lt_s(1, 0); // limit < speed ?
                let slow = !c.fcc();
                if slow {
                    break 'on;
                }
                c.cycles += 4;
                c.g[V0][0] = c.lw(p(self_at, spray::CFG));
                c.g[V1][0] = c.lw(p(r32(c, V0), 0x58));
                if c.g[V1][0] == 0 {
                    break 'on;
                }
                c.cycles += 2;
                let special = c.g[A1][0] != 0;
                c.g[A1][0] = sx(sp0);
                if special {
                    break 'on;
                }
                // pos = fx->pos + fx->dir * 15; then randomise the velocity.
                c.cycles += 82;
                fconst(c, 0, 0x4170_0000); // 15
                c.mfc1(2, 0);
                c.lqc2(4, p(self_at, spray::DIR));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, p(sp0, 0x20));
                c.g[A0][0] = c.g[S0][0];
                c.lq(2, p(sp0, 0x20));
                c.sq(2, p(sp0, 0x10));
                c.lqc2(3, p(self_at, spray::POS));
                c.lqc2(4, p(sp0, 0x10));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, p(sp0, 0x20));
                c.lq(2, p(sp0, 0x20));
                c.sq(2, sp0);
                callee::ADD_RANDOM_VELOCITY.call(c, 0x002e_0f84)?;
                // vt[2] of fx->obj + 0x6c0: the trail point the spray follows.
                c.g[A0][0] = c.lw(self_at);
                c.g[V1][0] = c.lw(p(r32(c, A0), 0x6c0));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x6c0));
                c.g[A1][0] = c.lh(p(r32(c, V1), 0x10));
                c.g[V0][0] = c.lw(p(r32(c, V1), 0x14));
                let target = r32(c, V0);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
                call_ptr(c, target, 0x002e_0fa0)?;
                // vel = *trail * curve(d8) + up * curve(dc)
                //     + (up * curve(e0)) * speed.
                c.g[V1][0] = c.lw(p(self_at, spray::CURVES));
                c.g[A0][0] = c.g[S1][0];
                c.lwc1(12, gp(c, -0x3af8)); // 1/60
                c.g[A3][0] = sx(r32(c, S0).wrapping_add(spray::COL));
                c.lwc1(0, p(r32(c, V1), 0xd8));
                c.g[A1][0] = sx(sp0);
                c.mfc1(3, 0);
                c.lqc2(4, r32(c, V0));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, p(sp0, 0x40));
                c.g[A2][0] = sx(p(sp0, 0x10));
                c.lq(2, p(sp0, 0x40));
                c.g[T0][0] = 1;
                c.g[T1][0] = c.lw(p(self_at, spray::CURVES));
                c.sq(2, p(sp0, 0x30));
                c.lwc1(0, p(r32(c, T1), 0xdc));
                c.g[V1][0] = c.lw(self_at);
                c.mfc1(10, 0);
                c.lqc2(4, p(r32(c, V1), 0x370));
                c.qmtc2(10, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, p(sp0, 0x50));
                c.lq(2, p(sp0, 0x50));
                c.sq(2, p(sp0, 0x40));
                c.lqc2(3, p(sp0, 0x30));
                c.lqc2(4, p(sp0, 0x40));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, p(sp0, 0x50));
                c.lq(2, p(sp0, 0x50));
                c.g[T1][0] = c.lw(p(self_at, spray::CURVES));
                c.sq(2, p(sp0, 0x20));
                c.lwc1(0, p(r32(c, T1), 0xe0));
                c.g[V1][0] = c.lw(self_at);
                c.mfc1(10, 0);
                c.lqc2(4, p(r32(c, V1), 0x370));
                c.qmtc2(10, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, p(sp0, 0x70));
                c.lq(2, p(sp0, 0x70));
                c.lwc1(0, p(self_at, spray::SPEED));
                c.sq(2, p(sp0, 0x60));
                c.mfc1(3, 0);
                c.lqc2(4, p(sp0, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, p(sp0, 0x70));
                c.lq(2, p(sp0, 0x70));
                c.sq(2, p(sp0, 0x50));
                c.lqc2(3, p(sp0, 0x20));
                c.lqc2(4, p(sp0, 0x50));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, p(sp0, 0x70));
                c.lq(2, p(sp0, 0x70));
                c.sq(2, p(sp0, 0x10));
                call_ptr(c, 0x0037_17c0, 0x002e_1080)?;
                frame.restore(c, S0, 0xa0);
                break 'out;
            }
            // Spray off: keep the emitter alive at the spray position.
            c.cycles += 8;
            c.lwc1(12, gp(c, -0x3af4));
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = sx(p(self_at, spray::POS));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0;
            call_ptr(c, 0x0037_17c0, 0x002e_10a4)?;
            frame.restore(c, S0, 0xa0);
            break 'out;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Board spark rendering.

/// `sSpark`: board spark particles.
mod spark {
    /// The dynamic emitter that owns the live particles.
    pub const EMITTER: u32 = 0x20;
    /// Argument passed to the trail renderer.
    pub const TRAIL_ARG: u32 = 0x28;
    /// The three quad positions.
    pub const QUADS: u32 = 0x30;
    /// Their sizes.
    pub const SIZES: u32 = 0x60;
    /// Draw the connected trail strip.
    pub const DRAW_TRAIL: u32 = 0x70;
    /// Draw the three spark quads.
    pub const DRAW_QUADS: u32 = 0x74;
    /// Saved renderer state for the trail (20 bytes).
    pub const TRAIL_STATE: u32 = 0x7c;
    /// Saved renderer state for the quads (20 bytes).
    pub const QUAD_STATE: u32 = 0x90;
}

/// `PushState`: copy the current 20-byte renderer state one slot up and leave
/// the new top in v0 (as the original's `stateTop[1] = stateTop[0];
/// stateTop++`). Returns the new top.
fn push_renderer_state(c: &mut Cpu) -> u32 {
    let gfxp = c.lw(gp(c, -0x854));
    c.g[V0][0] = c.lw(p(gfxp as u32, gfx::STATE_TOP));
    let top = r32(c, V0);
    c.ldl(3, p(top, 7));
    c.ldr(3, p(top, 0));
    c.ldl(5, p(top, 0xf));
    c.ldr(5, p(top, 8));
    c.g[A2][0] = c.lw(p(top, 0x10));
    c.sdl(3, p(top, 0x1b));
    c.sdr(3, p(top, 0x14));
    c.sdl(5, p(top, 0x23));
    c.sdr(5, p(top, 0x1c));
    c.sw(p(top, 0x24), c.g[A2][0]);
    c.g[V1][0] = c.lw(p(gfxp as u32, gfx::STATE_TOP));
    c.g[A1][0] = gfxp;
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x14));
    c.sw(p(gfxp as u32, gfx::STATE_TOP), c.g[V1][0]);
    c.g[V0][0] = c.lw(p(r32(c, A1), gfx::STATE_TOP));
    r32(c, V0)
}

/// `void cBoardSparkFX_Render(Spark* self)` (0x2db478): step the spark
/// emitter, then (if enabled) draw the trail strip and the three spark quads
/// as textured, coloured quads, each in its own pushed renderer state.
pub fn board_spark_fx_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save(c, S5, 0x20);
        frame.save_ra(c, 0x10);
        let self_at = r32(c, S0);
        let sp0 = sp(c);
        // Advance the spark emitter when it has live particles.
        c.g[A0][0] = c.lw(p(self_at, spark::EMITTER));
        c.g[V0][0] = c.lw(p(r32(c, A0), 0x1e0));
        if (c.g[V0][0] as i64) <= 0 {
            c.g[V0][0] = c.lw(p(self_at, spark::DRAW_TRAIL));
        } else {
            c.cycles += 3;
            c.g[A1][0] = 7;
            call_ptr(c, 0x0037_1688, 0x002d_b4b4)?;
            c.g[V0][0] = c.lw(p(self_at, spark::DRAW_TRAIL));
        }
        // The connected trail strip.
        c.cycles += 2;
        let no_trail = c.g[V0][0] == 0;
        c.g[A0][0] = c.lw(gp(c, -0x854));
        if !no_trail {
            c.cycles += 37;
            let top = push_renderer_state(c);
            c.ldl(3, p(self_at, spark::TRAIL_STATE + 7));
            c.ldr(3, p(self_at, spark::TRAIL_STATE));
            c.ldl(4, p(self_at, spark::TRAIL_STATE + 0xf));
            c.ldr(4, p(self_at, spark::TRAIL_STATE + 8));
            c.g[A1][0] = c.lw(p(self_at, spark::TRAIL_STATE + 0x10));
            c.sdl(3, p(top, 7));
            c.sdr(3, p(top, 0));
            c.sdl(4, p(top, 0xf));
            c.sdr(4, p(top, 8));
            c.sw(p(top, 0x10), c.g[A1][0]);
            c.g[A2][0] = c.lw(gp(c, -0x854));
            c.g[A1][0] = c.lw(p(self_at, spark::TRAIL_ARG));
            c.g[V0][0] = c.lw(p(r32(c, A2), gfx::VTABLE));
            c.g[A0][0] = c.lh(p(r32(c, V0), 0x290));
            c.g[V1][0] = c.lw(p(r32(c, V0), 0x294));
            let target = r32(c, V1);
            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            call_ptr(c, target, 0x002d_b544)?;
            c.g[A0][0] = c.lw(gp(c, -0x854));
            c.g[V0][0] = c.lw(p(r32(c, A0), gfx::STATE_TOP));
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x14));
            c.sw(p(r32(c, A0), gfx::STATE_TOP), c.g[V0][0]);
        }
        // The three spark quads.
        c.cycles += 3;
        c.g[V0][0] = c.lw(p(self_at, spark::DRAW_QUADS));
        if c.g[V0][0] != 0 {
            c.cycles += 30;
            let top = push_renderer_state(c);
            c.ldl(7, p(self_at, spark::QUAD_STATE + 7));
            c.ldr(7, p(self_at, spark::QUAD_STATE));
            c.ldl(3, p(self_at, spark::QUAD_STATE + 0xf));
            c.ldr(3, p(self_at, spark::QUAD_STATE + 8));
            c.g[A0][0] = c.lw(p(self_at, spark::QUAD_STATE + 0x10));
            c.sdl(7, p(top, 7));
            c.sdr(7, p(top, 0));
            c.sdl(3, p(top, 0xf));
            c.sdr(3, p(top, 8));
            c.sw(p(top, 0x10), c.g[A0][0]);
            // One-time setup of the shared sprite UVs and colour.
            c.g[V1][0] = c.lw(gp(c, 0x23e8));
            let t = c.g[V1][0] != 0;
            c.g[V0][0] = c.lw(gp(c, 0x23f8));
            if !t {
                c.cycles += 5;
                c.g[V0][0] = 1;
                c.sw(gp(c, 0x23e0), 0);
                c.sw(gp(c, 0x23e8), c.g[V0][0]);
                c.sw(gp(c, 0x23e4), 0);
                c.g[V0][0] = c.lw(gp(c, 0x23f8));
            }
            c.cycles += 2;
            let t = c.g[V0][0] != 0;
            c.g[V0][0] = c.lw(gp(c, 0x23fc));
            if !t {
                c.cycles += 7;
                fconst(c, 0, 0x3f80_0000);
                c.g[V0][0] = 1;
                c.sw(gp(c, 0x23f8), c.g[V0][0]);
                c.swc1(0, gp(c, 0x23f4));
                c.swc1(0, gp(c, 0x23f0));
                c.g[V0][0] = c.lw(gp(c, 0x23fc));
            }
            c.cycles += 2;
            let t = c.g[V0][0] != 0;
            c.g[S3][0] = 0x004d_0000;
            if !t {
                c.cycles += 9;
                fconst(c, 0, 0x3f80_0000);
                c.g[V0][0] = sx(r32(c, S3).wrapping_add(0x57a0));
                c.g[V1][0] = 1;
                c.swc1(0, p(r32(c, V0), 0xc));
                c.sw(gp(c, 0x23fc), c.g[V1][0]);
                c.swc1(0, p(r32(c, S3), 0x57a0));
                c.swc1(0, p(r32(c, V0), 4));
                c.swc1(0, p(r32(c, V0), 8));
            }
            c.cycles += 7;
            c.g[S1][0] = sx(self_at.wrapping_add(spark::SIZES));
            c.g[S5][0] = sx(r32(c, GP).wrapping_add(0x23e0));
            c.g[S4][0] = sx(r32(c, GP).wrapping_add(0x23f0));
            c.g[S0][0] = sx(self_at.wrapping_add(spark::QUADS));
            c.g[S2][0] = 2;
            c.lwc1(0, r32(c, S1));
            loop {
                c.cycles += 18;
                c.g[A1][0] = c.g[S0][0];
                c.g[T2][0] = c.lw(gp(c, 0x2a90));
                c.g[A2][0] = sx(sp0);
                c.swc1(0, p(sp0, 4));
                c.g[A3][0] = c.g[S5][0];
                c.swc1(0, sp0);
                c.g[T0][0] = c.g[S4][0];
                c.g[T1][0] = sx(r32(c, S3).wrapping_add(0x57a0));
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x10));
                c.g[V1][0] = c.lw(p(r32(c, T2), gfx::VTABLE));
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
                c.g[S2][0] = sx(r32(c, S2).wrapping_sub(1));
                c.g[A0][0] = c.lh(p(r32(c, V1), 0x270));
                c.g[V0][0] = c.lw(p(r32(c, V1), 0x274));
                let target = r32(c, V0);
                c.g[A0][0] = sx(r32(c, T2).wrapping_add(r32(c, A0)));
                call_ptr(c, target, 0x002d_b698)?;
                if (c.g[S2][0] as i64) >= 0 {
                    c.lwc1(0, r32(c, S1));
                    if event_due(c) {
                        deliver_event_here(c, 0x002d_b658);
                    }
                    continue;
                }
                c.cycles += 5;
                c.g[V1][0] = c.lw(gp(c, -0x854));
                c.g[V0][0] = c.lw(p(r32(c, V1), gfx::STATE_TOP));
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x14));
                c.sw(p(r32(c, V1), gfx::STATE_TOP), c.g[V0][0]);
                break;
            }
        }
        frame.restore(c, S0, 0x70);
        c.cycles += 8;
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Snow box cell rendering.

/// `SnowBox` cell fields read by the renderer.
mod snow_cell {
    /// 0: an ordinary cell, 1: the special cell, other: not drawn.
    pub const KIND: u32 = 0x04;
    /// Non-zero when the cell is on screen.
    pub const ACTIVE: u32 = 0x08;
    /// Cell size (used to scale the cell-local position).
    pub const SIZE: u32 = 0x0c;
    /// Extra alpha/height value, scaled by 128.
    pub const HEIGHT: u32 = 0x14;
    /// Colour scale, scaled by 128.
    pub const EXTRA: u32 = 0x18;
    /// Cell centre (divided by `size` into the cell grid).
    pub const POS: u32 = 0x20;
    /// The cell's model matrix.
    pub const MAT: u32 = 0x50;
    pub const CELL_ID: u32 = 0x98;
    pub const I9C: u32 = 0x9c;
    pub const IA0: u32 = 0xa0;
    /// RGB colour (0..1, scaled by 128 here).
    pub const COL_R: u32 = 0xa4;
    pub const COL_G: u32 = 0xa8;
    pub const COL_B: u32 = 0xac;
}

/// The global offset vectors used to keep the cell position in its cube.
const CELL_OFFSET_X: u32 = 0x004f_f140;
const CELL_OFFSET_Y: u32 = 0x004f_f150;
const CELL_OFFSET_Z: u32 = 0x004f_f160;

/// `lqc2 vf1..vf4, src; sqc2 vf1..vf4, dst` (a 64-byte matrix copy).
#[inline(always)]
fn copy_matrix(c: &mut Cpu, src: u32, dst: u32) {
    for k in 0..4u32 {
        c.lqc2(1 + k as usize, p(src, 0x10 * k));
        c.sqc2(1 + k as usize, p(dst, 0x10 * k));
    }
}

/// `void SnowBox_Render(Cell* cell, const Mat44* m, float x /*f12*/,
/// float y /*f13*/, float z /*f14*/)` (0x2e55d8): build the cell's render info
/// — model matrix `m * cell->mat`, cell-local position, colours and bounds —
/// nudge the position back into the cube the cell covers, then call the
/// renderer's cell draw (vtable slot 85) or the special cell draw (slot 86).
pub fn snow_box_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x120);
        c.g[A3][0] = c.g[A0][0];
        frame.save_ra(c, 0x110);
        let self_at = r32(c, A3);
        let m = abi::arg(c, 1);
        let sp0 = sp(c);
        c.g[V0][0] = c.lw(p(self_at, snow_cell::ACTIVE));
        let inactive = c.g[V0][0] == 0;
        c.g[V1][0] = sx(p(sp0, 0xd0));
        'outer: {
            'done: {
                if inactive {
                    break 'done;
                }
                // info.mat = m * cell->mat (and two stack copies of it).
                c.cycles += 104;
                fconst(c, 0, 0x3f80_0000); // 1
                c.g[A0][0] = sx(p(sp0, 0x90));
                c.g[V0][0] = sx(p(self_at, snow_cell::MAT));
                c.add_s(10, 14, 0); // z1
                c.add_s(8, 12, 0); // x1
                c.add_s(9, 13, 0); // y1
                c.lqc2(4, m);
                c.lqc2(5, p(m, 0x10));
                c.lqc2(6, p(m, 0x20));
                c.lqc2(7, p(m, 0x30));
                let b = r32(c, V0);
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
                c.sqc2(12, r32(c, V1));
                c.sqc2(13, p(r32(c, V1), 0x10));
                c.sqc2(14, p(r32(c, V1), 0x20));
                c.sqc2(15, p(r32(c, V1), 0x30));
                copy_matrix(c, p(sp0, 0xd0), p(sp0, 0x90));
                c.g[V0][0] = sx(p(sp0, 0x50));
                copy_matrix(c, p(sp0, 0x90), p(sp0, 0x50));
                // info.pos = cell->pos / cell->size.
                c.lwc1(0, p(self_at, snow_cell::SIZE));
                c.mfc1(2, 0);
                c.qmtc2(2, 3);
                c.vu0().div(0, 3, 3, 0);
                c.lqc2(4, p(self_at, snow_cell::POS));
                c.vu0().waitq();
                c.vu0().mul(0b1111, 4, 4, Src::Q);
                c.sqc2(4, p(sp0, 0xa0));
                // The colours, scaled into 0..128.
                fconst(c, 1, 0x4300_0000); // 128
                c.lwc1(3, p(self_at, snow_cell::COL_R));
                c.lwc1(0, p(self_at, snow_cell::COL_G));
                c.lwc1(2, p(self_at, snow_cell::COL_B));
                c.mul_s(3, 3, 1);
                c.lwc1(4, p(self_at, snow_cell::EXTRA));
                c.mul_s(0, 0, 1);
                c.lq(2, p(sp0, 0xa0));
                c.mul_s(2, 2, 1);
                c.mul_s(4, 4, 1);
                c.g[V1][0] = c.lw(p(self_at, snow_cell::ACTIVE));
                c.g[A0][0] = c.lw(p(self_at, snow_cell::CELL_ID));
                c.g[A1][0] = c.lw(p(self_at, snow_cell::I9C));
                c.g[A2][0] = c.lw(p(self_at, snow_cell::IA0));
                c.lwc1(1, p(self_at, snow_cell::HEIGHT));
                c.lwc1(5, p(self_at, snow_cell::SIZE));
                c.sq(2, p(sp0, 0x90));
                fconst(c, 7, 0xbf00_0000); // -0.5
                c.sq(2, sp0);
                c.swc1(0, p(sp0, 0x94));
                c.add_s(6, 12, 7); // x - 0.5
                c.sw(p(sp0, 0x20), c.g[V1][0]);
                c.sw(p(sp0, 0x24), c.g[A0][0]);
                c.sw(p(sp0, 0x28), c.g[A1][0]);
                c.sw(p(sp0, 0x2c), c.g[A2][0]);
                c.swc1(1, p(sp0, 0x3c));
                c.swc1(5, p(sp0, 0x4c));
                c.swc1(3, p(sp0, 0x90));
                c.swc1(2, p(sp0, 0x98));
                c.swc1(12, p(sp0, 0x30));
                c.swc1(13, p(sp0, 0x34));
                c.swc1(14, p(sp0, 0x38));
                c.swc1(8, p(sp0, 0x40));
                c.swc1(9, p(sp0, 0x44));
                c.swc1(10, p(sp0, 0x48));
                c.swc1(4, p(sp0, 0x9c));
                c.lwc1(0, sp0);
                c.lq(2, p(sp0, 0x90));
                c.c_lt_s(0, 6); // pos.x < x - 0.5 ?
                let inside_low_x = !c.fcc();
                c.sq(2, p(sp0, 0x10));
                if !inside_low_x {
                    c.cycles += 6;
                    c.g[V0][0] = sx(CELL_OFFSET_X);
                    c.lqc2(3, sp0);
                    c.lqc2(4, r32(c, V0));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, sp0);
                }
                // Nudge the position back into the cube.
                c.cycles += 8;
                fconst(c, 2, 0x3f00_0000); // 0.5
                c.lwc1(1, sp0);
                c.add_s(0, 8, 2);
                c.c_lt_s(0, 1); // x1 + 0.5 < pos.x ?
                let t = !c.fcc();
                c.g[V0][0] = sx(CELL_OFFSET_X);
                if !t {
                    c.cycles += 5;
                    c.lqc2(3, sp0);
                    c.lqc2(4, r32(c, V0));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, sp0);
                }
                c.cycles += 6;
                c.add_s(1, 13, 7); // y - 0.5
                c.lwc1(0, p(sp0, 4));
                c.c_lt_s(0, 1); // pos.y < y - 0.5 ?
                let t = !c.fcc();
                c.g[V0][0] = sx(CELL_OFFSET_Y);
                if !t {
                    c.cycles += 5;
                    c.lqc2(3, sp0);
                    c.lqc2(4, r32(c, V0));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, sp0);
                }
                c.cycles += 6;
                c.add_s(1, 9, 2); // y1 + 0.5
                c.lwc1(0, p(sp0, 4));
                c.c_lt_s(1, 0);
                let t = !c.fcc();
                c.g[V0][0] = sx(CELL_OFFSET_Y);
                if !t {
                    c.cycles += 5;
                    c.lqc2(3, sp0);
                    c.lqc2(4, r32(c, V0));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, sp0);
                }
                c.cycles += 6;
                c.add_s(1, 14, 7); // z - 0.5
                c.lwc1(0, p(sp0, 8));
                c.c_lt_s(0, 1);
                let t = !c.fcc();
                c.g[V0][0] = sx(CELL_OFFSET_Z);
                if !t {
                    c.cycles += 5;
                    c.lqc2(3, sp0);
                    c.lqc2(4, r32(c, V0));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, sp0);
                }
                c.cycles += 6;
                c.add_s(1, 10, 2); // z1 + 0.5
                c.lwc1(0, p(sp0, 8));
                c.c_lt_s(1, 0);
                let t = !c.fcc();
                c.g[V0][0] = sx(CELL_OFFSET_Z);
                if !t {
                    c.cycles += 5;
                    c.lqc2(3, sp0);
                    c.lqc2(4, r32(c, V0));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, sp0);
                }
                // pos -= (1.5, 1.5, 1.5, 0)
                c.cycles += 13;
                fconst(c, 0, 0x3fc0_0000);
                c.sw(p(sp0, 0x9c), 0);
                c.swc1(0, p(sp0, 0x98));
                c.swc1(0, p(sp0, 0x90));
                c.swc1(0, p(sp0, 0x94));
                c.lqc2(3, sp0);
                c.lqc2(4, p(sp0, 0x90));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp0);
                'kind: {
                    'maybe0: {
                        c.g[A0][0] = c.lw(p(self_at, snow_cell::KIND));
                        let ordinary = c.g[A0][0] == 0;
                        c.g[V0][0] = 1;
                        if ordinary {
                            break 'maybe0;
                        }
                        c.cycles += 2;
                        let special = c.g[A0][0] == c.g[V0][0];
                        frame.restore_ra(c, 0x110);
                        if special {
                            break 'kind;
                        }
                        c.cycles += 2;
                        break 'outer;
                    }
                    // The ordinary cell: draw through vtable slot 85.
                    c.cycles += 9;
                    c.g[A2][0] = c.lw(gp(c, -0x854));
                    c.g[A1][0] = sx(sp0);
                    c.g[V0][0] = c.lw(p(r32(c, A2), gfx::VTABLE));
                    c.g[A0][0] = c.lh(p(r32(c, V0), 0x2a8));
                    c.g[V1][0] = c.lw(p(r32(c, V0), 0x2ac));
                    let target = r32(c, V1);
                    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                    call_ptr(c, target, 0x002e_58ec)?;
                    frame.restore_ra(c, 0x110);
                    break 'outer;
                }
                // The special cell: draw through vtable slot 86.
                c.cycles += 7;
                c.g[A2][0] = c.lw(gp(c, -0x854));
                c.g[A1][0] = sx(sp0);
                c.g[V0][0] = c.lw(p(r32(c, A2), gfx::VTABLE));
                c.g[A0][0] = c.lh(p(r32(c, V0), 0x2b0));
                c.g[V1][0] = c.lw(p(r32(c, V0), 0x2b4));
                let target = r32(c, V1);
                c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                call_ptr(c, target, 0x002e_5910)?;
                break 'done;
            }
            c.cycles += 1;
            frame.restore_ra(c, 0x110);
            break 'outer;
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Rider power-up trail.

/// `RiderBase` trail state used by the update.
mod power_trail {
    /// The rider.
    pub const RIDER: u32 = 0x00;
    /// Number of live trail points (0..3).
    pub const COUNT: u32 = 0x0c;
    /// Ring slot being written.
    pub const CURSOR: u32 = 0x10;
    /// Ring of 0x40-byte slots at `cursor * 0x40` (four vertices each).
    pub const SLOTS: u32 = 0x20;
    /// The two 12-unit-wide edges of the current cross-section.
    pub const EDGE0: u32 = 0xe0;
    pub const EDGE1: u32 = 0xf0;
}

/// `Rider` fields read here.
mod pt_rider {
    pub const SPEED: u32 = 0x2ec;
    pub const MODEL: u32 = 0x77c;
    /// Index of the board bone in the skeleton.
    pub const BONE: u32 = 0x8a4;
    pub const SKELETON: u32 = 0x780;
    /// The skeleton's array of 0x40-byte node transforms.
    pub const NODES: u32 = 0x30;
}

/// `mfc1 rt, f; lqc2 vf4, v; qmtc2 rt, vf3; vmulx vf5, vf4, vf3x; sqc2 vf5,
/// out; lq r2, out; sq r2, store`: scale vector `v` by FPU register `f` and
/// leave a copy at `store`.
#[inline(always)]
fn scale_store(c: &mut Cpu, f: usize, v: u32, out: u32, store: u32) {
    c.mfc1(2, f);
    c.lqc2(4, v);
    c.qmtc2(2, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, out);
    c.lq(2, out);
    c.sq(2, store);
}

/// `vadd` plus the `lq`/`sq` copy to `store`.
#[inline(always)]
fn add_store(c: &mut Cpu, a: u32, b: u32, out: u32, store: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, out);
    c.lq(2, out);
    c.sq(2, store);
}

/// `vsub` plus the `lq`/`sq` copy to `store`.
#[inline(always)]
fn sub_store(c: &mut Cpu, a: u32, b: u32, out: u32, store: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, out);
    c.lq(2, out);
    c.sq(2, store);
}

/// `void RiderBase_UpdatePowerUpTrail(RiderBase* self)` (0x2eadd0): while the
/// rider is fast (or the debug override is set) keep a four-point trail
/// cross-section around the board: the two 12-unit side edges and four ring
/// slots (70 units forward/back ± 18 units sideways) rebuilt from the board
/// bone's frame each frame; otherwise clear the trail.
pub fn rider_base_update_power_up_trail(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0xc0);
        c.g[V0][0] = c.lw(gp(c, 0x1630));
        frame.save(c, S0, 0xa0);
        frame.save_ra(c, 0x90);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 21, 0xb8);
        frame.save_fpr(c, 20, 0xb0);
        let self_at = r32(c, S0);
        let sp0 = sp(c);
        c.g[V1][0] = c.lw(self_at);
        let hidden = c.g[V0][0] == 0;
        c.lwc1(20, p(r32(c, V1), pt_rider::SPEED));
        if !hidden {
            c.cycles += 2;
            fconst(c, 20, 0x4120_0000); // 10
        }
        // `zero < speed` (the zero comes from the flag register, a
        // denormal after `mtc1`, which the PS2 compares as 0).
        c.cycles += 5;
        c.mtc1(0, 21);
        c.c_lt_s(21, 20);
        let no_speed = !c.fcc();
        c.g[A1][0] = 1;
        if !no_speed {
            c.cycles += 22;
            c.g[A0][0] = c.lw(p(r32(c, V1), pt_rider::MODEL));
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0xd30));
            call_ptr(c, 0x0039_2de8, 0x002e_ae20)?;
            // Draw the glow: colour (0, 0, 1, 0), size 2.
            fconst(c, 0, 0x4000_0000); // 2
            c.g[A1][0] = sx(p(sp0, 0x10));
            c.g[V0][0] = c.lw(self_at);
            c.g[A2][0] = sx(sp0);
            c.swc1(0, p(sp0, 8));
            c.swc1(0, sp0);
            c.swc1(0, p(sp0, 4));
            fconst(c, 1, 0x3f80_0000); // 1
            c.g[A0][0] = c.lw(p(r32(c, V0), pt_rider::MODEL));
            c.swc1(1, p(sp0, 0x18));
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0xd30));
            c.swc1(21, p(sp0, 0x10));
            c.swc1(21, p(sp0, 0x14));
            c.swc1(21, p(sp0, 0x1c));
            call_ptr(c, 0x0039_2d90, 0x002e_ae64)?;
            c.c_lt_s(21, 20);
        }
        'out: {
            c.cycles += 2;
            if !c.fcc() {
                c.sw(p(self_at, power_trail::COUNT), 0);
                break 'out;
            }
            c.cycles += 5;
            c.g[A0][0] = c.lw(self_at);
            call_ptr(c, 0x0011_fe98, 0x002e_ae7c)?;
            c.g[V1][0] = 1;
            if c.g[V0][0] != c.g[V1][0] {
                c.sw(p(self_at, power_trail::COUNT), 0);
                break 'out;
            }
            // Advance the ring and rebuild the cross-section.
            c.cycles += 4;
            c.g[V1][0] = c.lw(p(self_at, power_trail::CURSOR));
            c.g[V0][0] = 3;
            c.cycles += 186;
            c.g[A1][0] = c.g[V0][0];
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(2));
            c.g[A0][0] = c.lw(p(self_at, power_trail::COUNT));
            c.div(0, 3, 2); // (cursor + 2) % 3
            c.g[A2][0] = c.lw(self_at);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
            fconst(c, 1, 0x4140_0000); // 12
            c.g[V0][0] = ((c.g[A0][0] as i64) < 4) as u64;
            fconst(c, 0, 0x428c_0000); // 70
            if c.g[V0][0] != 0 {
                c.g[A1][0] = c.g[A0][0];
            }
            fconst(c, 2, 0x4190_0000); // 18
            c.sw(p(self_at, power_trail::COUNT), c.g[A1][0]);
            c.g[A3][0] = c.hi[0];
            c.sw(p(self_at, power_trail::CURSOR), c.g[A3][0]);
            // node = skeleton->nodes[bone]
            c.g[V0][0] = c.lw(p(r32(c, A2), pt_rider::SKELETON));
            c.g[V1][0] = c.lw(p(r32(c, A2), pt_rider::BONE));
            c.g[A0][0] = c.lw(p(r32(c, V0), pt_rider::NODES));
            c.g[V1][0] = shl32(c.g[V1][0], 6);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
            c.lq(2, p(r32(c, A0), 0x30));
            c.sq(2, sp0);
            c.g[A1][0] = c.lw(p(r32(c, A2), pt_rider::SKELETON));
            c.g[V1][0] = c.lw(p(r32(c, A2), pt_rider::BONE));
            c.g[A0][0] = c.lw(p(r32(c, A1), pt_rider::NODES));
            c.g[V1][0] = shl32(c.g[V1][0], 6);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
            c.lq(2, r32(c, A0));
            c.sq(2, p(sp0, 0x20));
            c.g[A1][0] = c.lw(p(r32(c, A2), pt_rider::SKELETON));
            c.g[V1][0] = c.lw(p(r32(c, A2), pt_rider::BONE));
            c.g[A0][0] = c.lw(p(r32(c, A1), pt_rider::NODES));
            c.g[V1][0] = shl32(c.g[V1][0], 6);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
            c.lq(2, p(r32(c, A0), 0x20));
            c.sq(2, p(sp0, 0x30));
            // edge0 = pos - side * 12
            scale_store(c, 1, p(sp0, 0x30), p(sp0, 0x60), p(sp0, 0x50));
            sub_store(c, sp0, p(sp0, 0x50), p(sp0, 0x60), p(self_at, power_trail::EDGE0));
            c.sq(2, p(sp0, 0x40));
            // edge1 = pos + side * 12
            scale_store(c, 1, p(sp0, 0x30), p(sp0, 0x60), p(sp0, 0x50));
            add_store(c, sp0, p(sp0, 0x50), p(sp0, 0x60), p(self_at, power_trail::EDGE1));
            c.sq(2, p(sp0, 0x40));
            // slot v0 = pos + fwd * 70 - side * 18
            scale_store(c, 0, p(sp0, 0x20), p(sp0, 0x70), p(sp0, 0x60));
            add_store(c, sp0, p(sp0, 0x60), p(sp0, 0x70), p(sp0, 0x50));
            scale_store(c, 2, p(sp0, 0x30), p(sp0, 0x80), p(sp0, 0x70));
            sub_store(c, p(sp0, 0x50), p(sp0, 0x70), p(sp0, 0x80), p(sp0, 0x40));
            c.g[A0][0] = c.lw(p(self_at, power_trail::CURSOR));
            c.g[A0][0] = shl32(c.g[A0][0], 6);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(self_at));
            c.sq(2, p(r32(c, A0), power_trail::SLOTS));
            // slot v1 = pos + fwd * 70 + side * 18
            scale_store(c, 0, p(sp0, 0x20), p(sp0, 0x70), p(sp0, 0x60));
            add_store(c, sp0, p(sp0, 0x60), p(sp0, 0x70), p(sp0, 0x50));
            scale_store(c, 2, p(sp0, 0x30), p(sp0, 0x80), p(sp0, 0x70));
            add_store(c, p(sp0, 0x50), p(sp0, 0x70), p(sp0, 0x80), p(sp0, 0x40));
            c.g[V1][0] = c.lw(p(self_at, power_trail::CURSOR));
            c.g[V1][0] = shl32(c.g[V1][0], 6);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(self_at));
            c.sq(2, p(r32(c, V1), power_trail::SLOTS + 0x10));
            // slot v2 = pos - fwd * 70 + side * 18
            scale_store(c, 0, p(sp0, 0x20), p(sp0, 0x70), p(sp0, 0x60));
            sub_store(c, sp0, p(sp0, 0x60), p(sp0, 0x70), p(sp0, 0x50));
            scale_store(c, 2, p(sp0, 0x30), p(sp0, 0x80), p(sp0, 0x70));
            add_store(c, p(sp0, 0x50), p(sp0, 0x70), p(sp0, 0x80), p(sp0, 0x40));
            c.g[A0][0] = c.lw(p(self_at, power_trail::CURSOR));
            c.g[A0][0] = shl32(c.g[A0][0], 6);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(self_at));
            c.sq(2, p(r32(c, A0), power_trail::SLOTS + 0x20));
            // slot v3 = pos - fwd * 70 - side * 18
            scale_store(c, 0, p(sp0, 0x20), p(sp0, 0x70), p(sp0, 0x60));
            sub_store(c, sp0, p(sp0, 0x60), p(sp0, 0x70), p(sp0, 0x50));
            scale_store(c, 2, p(sp0, 0x30), p(sp0, 0x80), p(sp0, 0x70));
            sub_store(c, p(sp0, 0x50), p(sp0, 0x70), p(sp0, 0x80), p(sp0, 0x40));
            c.g[V1][0] = c.lw(p(self_at, power_trail::CURSOR));
            c.g[V1][0] = shl32(c.g[V1][0], 6);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(self_at));
            c.sq(2, p(r32(c, V1), power_trail::SLOTS + 0x30));
            break 'out;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0xa0);
        frame.restore_ra(c, 0x90);
        frame.restore_fpr(c, 21, 0xb8);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Anger FX.

/// `cAngerFX` fields.
mod anger {
    /// The rider.
    pub const RIDER: u32 = 0x00;
    /// Per-rider slots, 0x2c bytes each.
    pub const SLOTS: u32 = 0x04;
    pub const SLOT_STRIDE: u32 = 0x2c;
}

/// One rider's anger slot.
mod anger_slot {
    /// Glow level (approaches 1 while angry, 0 otherwise).
    pub const LEVEL: u32 = 0x00;
    /// RGBA colour (written as a 16-byte quad).
    pub const COLOUR: u32 = 0x04;
    /// Two glow intensities looked up from the animation tables.
    pub const GLOW_A: u32 = 0x14;
    pub const GLOW_B: u32 = 0x18;
    /// Last computed anger level (0..4+).
    pub const ANGER: u32 = 0x1c;
    /// Animation counters (mod 41, 31 and 24).
    pub const T20: u32 = 0x20;
    pub const T24: u32 = 0x24;
    pub const T28: u32 = 0x28;
}

/// Rider fields used by the anger update.
mod anger_rider {
    pub const VTABLE: u32 = 0x6c0;
    /// Set while the rider is in a state that hides the anger FX.
    pub const SPECIAL: u32 = 0xac4;
    /// The "other rider" link.
    pub const OTHER: u32 = 0xa8;
}

/// The four colour ramps (20 bytes each): level 4, 3, 2, 1.
const ANGER_COLOURS: [u32; 4] = [0x004d_5b30, 0x004d_5b40, 0x004d_5b50, 0x004d_5b60];

/// `ldl/ldr` two quadwords from `src` into `dst` (a 20-byte record copy).
#[inline(always)]
fn copy_record20(c: &mut Cpu, src: u32, dst: u32) {
    c.ldl(4, p(src, 7));
    c.ldr(4, src);
    c.ldl(7, p(src, 0xf));
    c.ldr(7, p(src, 8));
    c.sdl(4, p(dst, 7));
    c.sdr(4, dst);
    c.sdl(7, p(dst, 0xf));
    c.sdr(7, p(dst, 8));
}

/// `void cAngerFX_Update(AngerFX* self)` (0x2d4c08): for every rider, look up
/// how much the AI/rival aggression targets this rider and fade the anger glow
/// in (level 2+) or out, with the animation counters that drive the colour
/// pulse and the two glow tables.
pub fn anger_fx_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S4, 0x50);
        frame.save(c, S0, 0x90);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);
        let self_at = r32(c, S4);
        let sp0 = sp(c);
        // riderV(self->rider, 8) != 0 hides the FX.
        c.g[A0][0] = c.lw(self_at);
        c.g[V1][0] = c.lw(p(r32(c, A0), anger_rider::VTABLE));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(anger_rider::VTABLE));
        c.g[A1][0] = c.lh(p(r32(c, V1), 0x40));
        c.g[V0][0] = c.lw(p(r32(c, V1), 0x44));
        let target = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
        call_ptr(c, target, 0x002d_4c54)?;
        let hidden = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x90);
        'out: {
            if hidden {
                break 'out;
            }
            c.cycles += 7;
            c.g[V1][0] = c.lw(gp(c, -0x848));
            c.g[A0][0] = c.lw(p(r32(c, V1), 0x84));
            c.g[A1][0] = c.g[V1][0];
            c.g[V0][0] = c.lw(p(r32(c, A0), 0x84));
            c.g[V1][0] = c.lw(p(r32(c, V0), 0x10));
            let no_players = c.g[V1][0] == 0;
            c.g[S3][0] = 0;
            if no_players {
                break 'out;
            }
            c.cycles += 10;
            c.g[V0][0] = 0x0049_0000;
            c.g[V1][0] = 0x0049_0000;
            c.g[FP][0] = sx(r32(c, V0).wrapping_sub(0x6f10));
            c.g[S7][0] = sx(r32(c, V1).wrapping_sub(0x70b8));
            c.g[V0][0] = 0x0049_0000;
            c.g[V1][0] = 0x0049_0000;
            c.g[S6][0] = sx(r32(c, V0).wrapping_sub(0x6f90));
            c.g[S5][0] = sx(r32(c, V1).wrapping_sub(0x6eb0));
            c.g[A2][0] = c.lw(p(r32(c, A1), 0x84));
            'players: loop {
                'player: {
                    c.cycles += 9;
                    c.g[A0][0] = shl32(c.g[S3][0], 2);
                    c.g[V0][0] = c.lw(p(r32(c, A2), 0x84));
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    c.g[V1][0] = c.lw(p(r32(c, V0), 4));
                    c.g[A0][0] = c.lw(p(r32(c, V1), anger_rider::OTHER));
                    c.g[V0][0] = c.lw(p(r32(c, A0), 0x20));
                    c.g[S0][0] = c.lw(p(r32(c, V0), 4));
                    if c.g[S0][0] == 0 {
                        break 'player;
                    }
                    c.g[V0][0] = sx(anger::SLOT_STRIDE);
                    'slot: {
                        'zero: {
                            c.cycles += 6;
                            c.g[A1][0] = c.lw(self_at);
                            c.mult(0, 2, 19, 2); // slot offset = i * 0x2c
                            c.g[V1][0] = c.lw(p(self_at, anger::SLOTS));
                            c.g[A0][0] = c.lw(p(r32(c, A1), anger_rider::SPECIAL));
                            let special = c.g[A0][0] != 0;
                            c.g[S2][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                            if special {
                                break 'zero;
                            }
                            // n = **(world->+0x84 + 0x28); 0 < n < 10 hides it.
                            c.cycles += 4;
                            c.g[V0][0] = c.lw(p(r32(c, A2), 0x28));
                            c.g[V0][0] = c.lw(r32(c, V0));
                            let none = c.g[V0][0] == 0;
                            c.g[V1][0] = 0;
                            if !none {
                                c.cycles += 1;
                                c.g[V1][0] = ((c.g[V0][0] as i64) < 10) as u64;
                            }
                            c.cycles += 2;
                            if c.g[V1][0] == 0 {
                                break 'slot;
                            }
                        }
                        c.cycles += 2;
                        c.sw(r32(c, S2), 0);
                        c.cycles += 1;
                        frame.restore(c, S0, 0x90);
                        break 'players;
                    }
                    // Aggression level of this rider against `other`.
                    c.cycles += 26;
                    call_ptr(c, 0x0015_5a50, 0x002d_4d08)?;
                    c.g[A0][0] = c.lw(self_at);
                    c.g[S1][0] = c.g[V0][0];
                    c.g[V1][0] = c.lw(p(r32(c, A0), anger_rider::VTABLE));
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(anger_rider::VTABLE));
                    c.g[A1][0] = c.lh(p(r32(c, V1), 0x38));
                    c.g[V0][0] = c.lw(p(r32(c, V1), 0x3c));
                    let target = r32(c, V0);
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
                    call_ptr(c, target, 0x002d_4d28)?;
                    c.g[V1][0] = c.lw(p(r32(c, S0), anger_rider::VTABLE));
                    c.g[A1][0] = sx(r32(c, S0).wrapping_add(anger_rider::VTABLE));
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.lh(p(r32(c, V1), 0x38));
                    c.g[V0][0] = c.lw(p(r32(c, V1), 0x3c));
                    let target = r32(c, V0);
                    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                    call_ptr(c, target, 0x002d_4d44)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x0015_5b50, 0x002d_4d54)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A2][0] = c.lw(p(r32(c, S2), anger_slot::ANGER));
                    c.g[V0][0] = ((c.g[A1][0] as i64) < 4) as u64;
                    let below4 = c.g[V0][0] != 0;
                    c.sw(p(r32(c, S2), anger_slot::ANGER), c.g[A1][0]);
                    'colour: {
                        'lev: {
                            if below4 {
                                break 'lev;
                            }
                            c.cycles += 12;
                            c.g[V0][0] = 0x004d_0000;
                            copy_record20(c, ANGER_COLOURS[0], p(r32(c, S2), anger_slot::COLOUR));
                            c.g[V1][0] = ((c.g[A1][0] as i64) < 2) as u64;
                            break 'colour;
                        }
                        'three: {
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[A1][0] as i64) < 3) as u64;
                            let under3 = c.g[V0][0] != 0;
                            c.g[V1][0] = ((c.g[A1][0] as i64) < 2) as u64;
                            if under3 {
                                break 'three;
                            }
                            c.cycles += 12;
                            c.g[V0][0] = 0x004d_0000;
                            copy_record20(c, ANGER_COLOURS[1], p(r32(c, S2), anger_slot::COLOUR));
                            break 'colour;
                        }
                        'four: {
                            c.cycles += 2;
                            let under2 = c.g[V1][0] != 0;
                            c.g[V0][0] = 0x004d_0000;
                            if !under2 {
                                c.cycles += 12;
                                c.g[V0][0] = 0x004d_0000;
                                copy_record20(c, ANGER_COLOURS[2], p(r32(c, S2), anger_slot::COLOUR));
                                break 'colour;
                            }
                        }
                        c.cycles += 9;
                        c.g[T1][0] = sx(r32(c, V0).wrapping_add(0x5b60));
                        copy_record20(c, r32(c, T1), p(r32(c, S2), anger_slot::COLOUR));
                        break 'colour;
                    }
                    // approach(level, 1, 0.1) for angry, (level, 0, 0.0666)
                    // otherwise.
                    'approach: {
                        'target: {
                            'low: {
                                c.cycles += 2;
                                let not_angry = c.g[V1][0] != 0;
                                c.lwc1(2, r32(c, S2));
                                if not_angry {
                                    break 'low;
                                }
                                c.cycles += 6;
                                c.lwc1(0, gp(c, -0x3d38)); // 1.1
                                c.lwc1(3, gp(c, -0x3d34)); // 0.1
                                c.c_lt_s(0, 2);
                                if c.fcc() {
                                    c.sub_s(1, 2, 3);
                                    break 'approach;
                                }
                                c.cycles += 5;
                                c.lwc1(0, gp(c, -0x3d30)); // 0.9
                                c.g[AT][0] = 0x3f80_0000;
                                c.mtc1(1, 1);
                                break 'target;
                            }
                            'rise: {
                                c.cycles += 4;
                                c.lwc1(3, gp(c, -0x3d2c)); // 0.0666
                                c.c_lt_s(3, 2);
                                let t = !c.fcc();
                                if !t {
                                    c.cycles += 2;
                                    c.sub_s(1, 2, 3);
                                    break 'approach;
                                }
                            }
                            c.cycles += 2;
                            c.lwc1(0, gp(c, -0x3d28)); // -0.0666
                            c.mtc1(0, 1); // f1 = 0
                            break 'target;
                        }
                        c.cycles += 4;
                        c.c_lt_s(2, 0);
                        if c.fcc() {
                            c.add_s(1, 2, 3);
                        }
                        break 'approach;
                    }
                    // Fade in the glow, then advance the three counters.
                    't20_block: {
                        't20: {
                            c.cycles += 7;
                            c.swc1(1, r32(c, S2));
                            c.lwc1(0, r32(c, S2));
                            c.lwc1(1, gp(c, -0x3d24)); // 0.8
                            c.g[V0][0] = c.lw(p(r32(c, S2), anger_slot::T20));
                            c.mul_s(0, 0, 1);
                            let running = (c.g[V0][0] as i64) > 0;
                            c.swc1(0, p(r32(c, S2), anger_slot::COLOUR));
                            if !running {
                                c.cycles += 2;
                                if c.g[A2][0] == c.g[A1][0] {
                                    c.g[V1][0] = c.lw(p(r32(c, S2), anger_slot::T24));
                                    break 't20_block;
                                }
                            }
                        }
                        c.cycles += 6;
                        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                        c.g[V1][0] = 0x29;
                        c.div(0, 2, 3);
                        c.sw(p(r32(c, S2), anger_slot::T20), c.g[V0][0]);
                        c.cycles += 3;
                        c.g[A0][0] = c.hi[0];
                        c.sw(p(r32(c, S2), anger_slot::T20), c.g[A0][0]);
                        c.g[V1][0] = c.lw(p(r32(c, S2), anger_slot::T24));
                        break 't20_block;
                    }
                    't24_block: {
                        't24: {
                            c.cycles += 2;
                            let running = (c.g[V1][0] as i64) > 0;
                            c.g[V0][0] = sx(r32(c, V1).wrapping_add(1));
                            if running {
                                break 't24;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = ((c.g[A1][0] as i64) < 4) as u64;
                            if c.g[V0][0] != 0 {
                                c.g[V0][0] = c.lw(p(r32(c, S2), anger_slot::T28));
                                break 't24_block;
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(p(r32(c, S2), anger_slot::T20));
                            if c.g[V0][0] != 0 {
                                c.g[V0][0] = c.lw(p(r32(c, S2), anger_slot::T28));
                                break 't24_block;
                            }
                            c.cycles += 1;
                            c.g[V0][0] = sx(r32(c, V1).wrapping_add(1));
                        }
                        c.cycles += 5;
                        c.g[V1][0] = 0x1f;
                        c.sw(p(r32(c, S2), anger_slot::T24), c.g[V0][0]);
                        c.div(0, 2, 3);
                        c.cycles += 3;
                        c.g[A0][0] = c.hi[0];
                        c.sw(p(r32(c, S2), anger_slot::T24), c.g[A0][0]);
                        c.g[V0][0] = c.lw(p(r32(c, S2), anger_slot::T28));
                        break 't24_block;
                    }
                    't28_block: {
                        't28: {
                            c.cycles += 2;
                            let running = (c.g[V0][0] as i64) > 0;
                            c.g[V1][0] = ((c.g[A2][0] as i64) < (c.g[A1][0] as i64)) as u64;
                            if !running {
                                c.cycles += 2;
                                if c.g[V1][0] == 0 {
                                    c.g[V0][0] = c.lw(p(r32(c, S2), anger_slot::T20));
                                    break 't28_block;
                                }
                            }
                        }
                        c.cycles += 2;
                        if c.g[V1][0] != 0 {
                            c.sw(p(r32(c, S2), anger_slot::T28), 0);
                        }
                        c.cycles += 7;
                        c.g[V0][0] = c.lw(p(r32(c, S2), anger_slot::T28));
                        c.g[V1][0] = 0x18;
                        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                        c.div(0, 2, 3);
                        c.sw(p(r32(c, S2), anger_slot::T28), c.g[V0][0]);
                        c.cycles += 4;
                        c.g[V0][0] = c.lw(p(r32(c, S2), anger_slot::T20));
                        c.g[A0][0] = c.hi[0];
                        let running = (c.g[V0][0] as i64) > 0;
                        c.sw(p(r32(c, S2), anger_slot::T28), c.g[A0][0]);
                        if !running {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(p(r32(c, S2), anger_slot::T24));
                            if (c.g[V0][0] as i64) <= 0 {
                                c.g[V0][0] = c.lw(p(r32(c, S2), anger_slot::T20));
                                break 't28_block;
                            }
                        }
                        c.cycles += 2;
                        c.sw(p(r32(c, S2), anger_slot::T28), 0);
                        c.g[V0][0] = c.lw(p(r32(c, S2), anger_slot::T20));
                        break 't28_block;
                    }
                    // glow = level * A[t20] * C[t24] * E[t28]  (and the B/D/F
                    // tables for the second channel).
                    c.cycles += 30;
                    c.g[A1][0] = 0x0049_0000;
                    c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x7160));
                    c.g[V1][0] = c.lw(p(r32(c, S2), anger_slot::T24));
                    c.g[V0][0] = shl32(c.g[V0][0], 2);
                    c.lwc1(3, r32(c, S2));
                    c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, S7)));
                    c.g[V1][0] = shl32(c.g[V1][0], 2);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
                    c.lwc1(0, r32(c, A0));
                    c.lwc1(2, r32(c, V0));
                    c.g[A3][0] = 0x0049_0000;
                    c.g[A1][0] = sx(r32(c, V1).wrapping_add(r32(c, S6)));
                    c.g[A3][0] = sx(r32(c, A3).wrapping_sub(0x7010));
                    c.mul_s(2, 3, 2);
                    c.g[V0][0] = c.lw(p(r32(c, S2), anger_slot::T28));
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A3)));
                    c.mul_s(3, 3, 0);
                    c.lwc1(0, r32(c, V1));
                    c.g[V0][0] = shl32(c.g[V0][0], 2);
                    c.lwc1(1, r32(c, A1));
                    c.g[V1][0] = sx(r32(c, V0).wrapping_add(r32(c, S5)));
                    c.mul_s(2, 2, 0);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, FP)));
                    c.mul_s(3, 3, 1);
                    c.lwc1(0, r32(c, V0));
                    c.lwc1(1, r32(c, V1));
                    c.mul_s(2, 2, 0);
                    c.mul_s(3, 3, 1);
                    c.swc1(2, p(r32(c, S2), anger_slot::GLOW_A));
                    c.swc1(3, p(r32(c, S2), anger_slot::GLOW_B));
                    break 'player;
                }
                // Next player.
                c.cycles += 9;
                c.g[V0][0] = c.lw(gp(c, -0x848));
                c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
                c.g[A0][0] = c.lw(p(r32(c, V0), 0x84));
                c.g[A1][0] = c.g[V0][0];
                c.g[V1][0] = c.lw(p(r32(c, A0), 0x84));
                c.g[V0][0] = c.lw(p(r32(c, V1), 0x10));
                c.g[V0][0] = (c.g[S3][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] == 0 {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x90);
                    break 'players;
                }
                c.g[A2][0] = c.lw(p(r32(c, A1), 0x84));
                if event_due(c) {
                    deliver_event_here(c, 0x002d_4ca0);
                }
            }
        }
        c.cycles += 11;
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        let _ = sp0;
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Spray FX emitter update (the main spray, emitter slot 5).

/// `cDynamicSprayFX` fields used by the main emitter update.
mod spray5 {
    pub const OBJ: u32 = 0x00;
    /// Spray level (fades 0..2).
    pub const LEVEL: u32 = 0x04;
    /// Base of the dynamic emitter array.
    pub const P28: u32 = 0x28;
    /// Curve table used for the velocity scales.
    pub const P2C: u32 = 0x2c;
    pub const DIR: u32 = 0x30;
    pub const COL: u32 = 0x90;
    pub const POS: u32 = 0xa0;
    /// Configuration object.
    pub const CFG: u32 = 0xd0;
    /// Mode flag (2: double rate).
    pub const ID4: u32 = 0xd4;
    /// Spray strength (0..~3333), the fade gate.
    pub const SPEED: u32 = 0xe0;
    /// Position the splash/trail follows.
    pub const FRAME_POS: u32 = 0xf0;
    /// Direction the spray is thrown along.
    pub const THROW: u32 = 0x100;
    /// Non-zero switches to the slower/other rate.
    pub const I114: u32 = 0x114;
    /// Smoothed alpha.
    pub const ALPHA: u32 = 0x120;
    /// Non-zero disables the decay multiplier.
    pub const I124: u32 = 0x124;
}

/// The spray's configuration object.
mod spray_cfg {
    pub const ENABLED: u32 = 0x60;
    /// Enables the long-range mode.
    pub const FLAG: u32 = 0x64;
    /// Decay multiplier applied to the spray strength.
    pub const DECAY: u32 = 0x68;
}

/// `void cDynamicSprayFX_UpdateEmitter5(SprayFX* fx)` (0x2e1598): the main
/// board spray. At speed it spawns a particle stream from one of two emitter
/// slots (switching slot above 277 units/s), with random jitter, the rider's
/// up-vector and trail point, a smoothed alpha and the colour; otherwise it
/// keeps two emitter slots alive at the spray position.
pub fn dynamic_spray_fx_update_emitter5(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xc0);
        c.lwc1(0, gp(c, -0x3abc)); // 27.7778
        frame.save(c, S1, 0x90);
        frame.save(c, S0, 0xa0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        frame.save_ra(c, 0x40);
        frame.save_fpr(c, 20, 0xb0);
        let self_at = r32(c, S1);
        let sp0 = sp(c);
        c.lwc1(1, p(self_at, spray5::SPEED));
        c.c_lt_s(0, 1); // 27.7778 < speed ?
        let slow = !c.fcc();
        c.g[S2][0] = c.lw(p(self_at, spray5::CFG));
        'out: {
            'on: {
                if slow {
                    break 'on;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw(p(r32(c, S2), spray_cfg::ENABLED));
                let disabled = c.g[V0][0] == 0;
                c.g[S3][0] = 6;
                if disabled {
                    break 'on;
                }
                // Slots: a = 6, b = 5, swapping above 277.7778.
                'slots: {
                    'flag: {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(p(r32(c, S2), spray_cfg::FLAG));
                        let no_flag = c.g[V0][0] == 0;
                        c.g[S5][0] = 5;
                        if no_flag {
                            break 'flag;
                        }
                        c.cycles += 5;
                        c.lwc1(0, gp(c, -0x3ab8)); // 277.7778
                        c.c_lt_s(0, 1);
                        let under = !c.fcc();
                        c.g[V0][0] = 5;
                        if under {
                            break 'slots;
                        }
                        c.cycles += 2;
                        c.g[S3][0] = 5;
                        c.g[S5][0] = 6;
                        break 'flag;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 5;
                    break 'slots;
                }
                // v = fx->frame_pos; if a == 5, splash it.
                'v: {
                    'splash: {
                        c.cycles += 2;
                        let not_five = c.g[S3][0] != c.g[V0][0];
                        c.g[S4][0] = sx(p(self_at, spray5::FRAME_POS));
                        if !not_five {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw(self_at);
                            c.g[V1][0] = c.lw(p(r32(c, V0), 0xac4));
                            if c.g[V1][0] == 0 {
                                c.cycles += 3;
                                c.lwc1(12, p(self_at, spray5::SPEED));
                                c.g[A0][0] = c.g[S4][0];
                                callee::ADD_SPLASH.call(c, 0x002e_1634)?;
                                break 'splash;
                            }
                            c.lq(2, r32(c, S4));
                            break 'v;
                        }
                    }
                    c.cycles += 1;
                    c.lq(2, r32(c, S4));
                }
                // k = 0.5 and the throw vector scaled by 20 in long-range
                // mode, else k = 1 and the throw vector scaled by 45.
                'scale: {
                    'short: {
                        c.cycles += 5;
                        c.g[V1][0] = 6;
                        c.g[AT][0] = 0x3f80_0000;
                        c.mtc1(1, 5);
                        let not_long = c.g[S3][0] != c.g[V1][0];
                        c.sq(2, sp0);
                        if not_long {
                            break 'short;
                        }
                        c.cycles += 11;
                        fconst(c, 0, 0x41a0_0000); // 20
                        c.mfc1(2, 0);
                        c.lqc2(4, p(self_at, spray5::THROW));
                        c.qmtc2(2, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, p(sp0, 0x20));
                        c.g[AT][0] = 0x3f00_0000;
                        c.mtc1(1, 5);
                        c.lq(2, p(sp0, 0x20));
                        break 'scale;
                    }
                    c.cycles += 8;
                    fconst(c, 0, 0x4234_0000); // 45
                    c.mfc1(3, 0);
                    c.lqc2(4, p(self_at, spray5::THROW));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, p(sp0, 0x20));
                    c.lq(2, p(sp0, 0x20));
                }
                // pos += throw * k; r = clamp(speed / 3333.3335, 0, 1).
                c.cycles += 15;
                c.sq(2, p(sp0, 0x10));
                c.lqc2(3, sp0);
                c.lqc2(4, p(sp0, 0x10));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp0);
                c.lwc1(0, gp(c, -0x3ab4)); // 3333.3335
                c.lwc1(1, p(self_at, spray5::SPEED));
                c.mtc1(0, 2); // f2 = 0
                c.div_s(1, 1, 0);
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 0); // f0 = 1
                c.c_le_s(2, 1);
                let negative = !c.fcc();
                c.mov_s(4, 2);
                if negative {
                    // f4 = 0
                } else {
                    c.cycles += 1;
                    c.min_s(4, 1, 0);
                }
                // Level: fade towards 2 while a == 5 or the level is low.
                'level: {
                    'choose: {
                        c.cycles += 5;
                        c.lwc1(3, p(self_at, spray5::LEVEL));
                        c.c_lt_s(3, 0); // level < 1 ?
                        if c.fcc() {
                            c.g[V1][0] = c.lw(p(self_at, spray5::ID4));
                            break 'choose;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = 5;
                        if c.g[S3][0] != c.g[V0][0] {
                            c.g[V0][0] = c.lw(p(r32(c, S2), spray_cfg::FLAG));
                            break 'level;
                        }
                        c.cycles += 1;
                        c.g[V1][0] = c.lw(p(self_at, spray5::ID4));
                        break 'choose;
                    }
                    'update: {
                        'rate: {
                            c.cycles += 3;
                            c.g[V0][0] = 2;
                            if c.g[V1][0] != c.g[V0][0] {
                                c.g[V0][0] = c.lw(p(self_at, spray5::I114));
                                break 'rate;
                            }
                            c.cycles += 6;
                            fconst(c, 1, 0x4000_0000); // 2
                            c.c_lt_s(3, 1); // level < 2 ?
                            if !c.fcc() {
                                c.g[V0][0] = c.lw(p(r32(c, S2), spray_cfg::FLAG));
                                break 'level;
                            }
                            'clamp2: {
                                c.cycles += 7;
                                c.add_s(0, 4, 4);
                                c.mul_s(0, 0, 5);
                                c.add_s(0, 3, 0);
                                c.c_le_s(0, 1);
                                let t = !c.fcc();
                                c.mov_s(2, 1);
                                if t {
                                    // f2 = 2
                                } else {
                                    c.cycles += 1;
                                    c.mov_s(2, 0);
                                }
                            }
                            c.cycles += 2;
                            c.swc1(2, p(self_at, spray5::LEVEL));
                            break 'update;
                        }
                        'i114: {
                            'check114: {
                                c.cycles += 2;
                                let has114 = c.g[V0][0] != 0;
                                if has114 {
                                    break 'check114;
                                }
                                c.cycles += 6;
                                fconst(c, 0, 0x3fc0_0000); // 1.5
                                c.c_lt_s(3, 0); // level < 1.5 ?
                                if !c.fcc() {
                                    c.g[V0][0] = c.lw(p(r32(c, S2), spray_cfg::FLAG));
                                    break 'level;
                                }
                                c.cycles += 5;
                                c.mul_s(0, 5, 0);
                                c.g[AT][0] = 0x4000_0000;
                                c.mtc1(1, 1);
                                c.mul_s(0, 4, 0);
                                break 'i114;
                            }
                            c.cycles += 4;
                            c.c_lt_s(3, 0); // level < 1.5 ?
                            if !c.fcc() {
                                c.g[V0][0] = c.lw(p(r32(c, S2), spray_cfg::FLAG));
                                break 'level;
                            }
                            c.cycles += 3;
                            c.mul_s(0, 4, 5);
                            c.g[AT][0] = 0x4000_0000;
                            c.mtc1(1, 1);
                            break 'i114;
                        }
                        c.cycles += 3;
                        c.add_s(0, 3, 0);
                        c.min_s(0, 0, 1);
                        c.swc1(0, p(self_at, spray5::LEVEL));
                        break 'update;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(p(r32(c, S2), spray_cfg::FLAG));
                    break 'level;
                }
                // Jitter amplitude: 60 in long-range mode with i124 set,
                // else 25.
                'jitter: {
                    c.cycles += 4;
                    fconst(c, 2, 0x41c8_0000); // 25
                    let no_flag = c.g[V0][0] == 0;
                    c.g[V0][0] = c.lw(gp(c, 0xa0c));
                    if no_flag {
                        break 'jitter;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(p(self_at, spray5::I124));
                    let no_124 = c.g[V0][0] == 0;
                    c.g[V0][0] = c.lw(gp(c, 0xa0c));
                    if no_124 {
                        break 'jitter;
                    }
                    c.cycles += 2;
                    fconst(c, 2, 0x4270_0000); // 60
                }
                // Random jitter, two emitter calls, and the alpha/decay.
                c.cycles += 81;
                c.g[V1][0] = 0x0001_8fcd;
                c.g[A0][0] = 0x00e9_507c;
                c.mult(0, 2, 2, 3);
                c.g[A1][0] = 0x007f_ffff;
                c.g[A2][0] = 0x3f80_0000;
                c.neg_s(3, 2); // -j
                fconst(c, 20, 0x3f80_0000); // 1
                c.g[V0][0] = add32(c.g[V0][0], c.g[A0][0]);
                c.sw(p(sp0, 0x18), 0);
                c.g[V0][0] = c.g[V0][0] & c.g[A1][0];
                c.sub_s(2, 2, 3); // 2j
                c.g[V0][0] = c.g[V0][0] | c.g[A2][0];
                c.sw(p(sp0, 0x1c), 0);
                c.mult(0, 7, 2, 3);
                c.sw(gp(c, 0xa0c), c.g[V0][0]);
                c.lwc1(1, gp(c, 0xa0c));
                c.g[V1][0] = add32(c.g[A3][0], c.g[A0][0]);
                c.sub_s(1, 1, 20);
                c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
                c.g[V1][0] = c.g[V1][0] | c.g[A2][0];
                c.sw(gp(c, 0xa0c), c.g[V1][0]);
                c.mul_s(1, 2, 1);
                c.lwc1(0, gp(c, 0xa0c));
                c.add_s(1, 3, 1);
                c.sub_s(0, 0, 20);
                c.swc1(1, p(sp0, 0x10));
                c.mul_s(2, 2, 0);
                c.add_s(3, 3, 2);
                c.swc1(3, p(sp0, 0x14));
                c.lqc2(3, sp0);
                c.lqc2(4, p(sp0, 0x10));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, sp0);
                // The trail point (twice through the rider's vtable).
                c.g[A0][0] = c.lw(self_at);
                c.g[V1][0] = c.lw(p(r32(c, A0), 0x6c0));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x6c0));
                c.g[A1][0] = c.lh(p(r32(c, V1), 0x10));
                c.g[V0][0] = c.lw(p(r32(c, V1), 0x14));
                let target = r32(c, V0);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
                call_ptr(c, target, 0x002e_187c)?;
                c.g[A0][0] = c.lw(self_at);
                c.g[S0][0] = c.g[V0][0];
                c.g[V1][0] = c.lw(p(r32(c, A0), 0x6c0));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x6c0));
                c.g[A1][0] = c.lh(p(r32(c, V1), 0x10));
                c.g[V0][0] = c.lw(p(r32(c, V1), 0x14));
                let target = r32(c, V0);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
                call_ptr(c, target, 0x002e_189c)?;
                // vel = trail - up * dot(trail, up) * 0.75.
                c.g[V1][0] = c.lw(self_at);
                fconst(c, 1, 0x3f40_0000); // 0.75
                c.lqc2(3, r32(c, V0));
                c.lqc2(5, p(r32(c, V1), 0x370));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(4, 4);
                c.mtc1(4, 0);
                c.lwc1(2, gp(c, -0x3ab0)); // 0.3
                c.mul_s(0, 0, 1);
                c.mfc1(4, 0);
                c.lqc2(4, p(r32(c, V1), 0x370));
                c.qmtc2(4, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, p(sp0, 0x30));
                c.lq(2, p(sp0, 0x30));
                c.sq(2, p(sp0, 0x20));
                c.lqc2(3, r32(c, S0));
                c.lqc2(4, p(sp0, 0x20));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, p(sp0, 0x30));
                c.lq(2, p(sp0, 0x30));
                c.g[V1][0] = c.lw(p(self_at, spray5::I114));
                let has114 = c.g[V1][0] != 0;
                c.sq(2, p(sp0, 0x10));
                if !has114 {
                    c.cycles += 1;
                    c.lwc1(2, gp(c, -0x3aac)); // 0.4
                }
                c.cycles += 11;
                c.mfc1(2, 2);
                c.lqc2(4, p(sp0, 0x10));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, p(sp0, 0x30));
                c.g[AT][0] = 0x3f00_0000;
                c.mtc1(1, 3);
                c.lq(2, p(sp0, 0x30));
                c.g[V1][0] = c.lw(p(self_at, spray5::I114));
                let has114 = c.g[V1][0] != 0;
                c.sq(2, p(sp0, 0x20));
                if !has114 {
                    c.cycles += 1;
                    c.mov_s(3, 20);
                }
                // Alpha: snap up, otherwise blend 0.8/0.2.
                'alpha: {
                    'snap: {
                        c.cycles += 4;
                        c.lwc1(2, p(self_at, spray5::ALPHA));
                        c.c_lt_s(2, 3);
                        let t = !c.fcc();
                        if t {
                            break 'snap;
                        }
                        c.cycles += 2;
                        c.swc1(3, p(self_at, spray5::ALPHA));
                        break 'alpha;
                    }
                    c.cycles += 6;
                    c.lwc1(1, gp(c, -0x3aa8)); // 0.8
                    c.lwc1(0, gp(c, -0x3aa4)); // 0.2
                    c.mul_s(1, 2, 1);
                    c.mul_s(0, 3, 0);
                    c.add_s(1, 1, 0);
                    c.swc1(1, p(self_at, spray5::ALPHA));
                    break 'alpha;
                }
                // Spawn in slot a with the jittered velocity, then in slot b
                // at the frame position.
                c.cycles += 30;
                c.g[S0][0] = 0x210;
                c.lq(2, p(self_at, spray5::COL));
                c.mult(0, 3, 19, 16); // a * 0x210
                c.lwc1(0, p(self_at, spray5::ALPHA));
                c.g[A0][0] = c.lw(p(self_at, spray5::P28));
                c.g[A1][0] = sx(sp0);
                c.lwc1(20, gp(c, -0x3aa0)); // 1/60
                c.g[A2][0] = sx(p(sp0, 0x20));
                c.sq(2, p(sp0, 0x30));
                c.g[A3][0] = sx(p(sp0, 0x30));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
                c.swc1(0, p(sp0, 0x3c));
                c.g[T0][0] = 1;
                c.mov_s(12, 20);
                call_ptr(c, 0x0037_17c0, 0x002e_19b0)?;
                c.mult(0, 16, 21, 16); // b * 0x210
                c.g[A0][0] = c.lw(p(self_at, spray5::P28));
                c.g[A1][0] = c.g[S4][0];
                c.mov_s(12, 20);
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                c.g[T0][0] = 0;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S0)));
                call_ptr(c, 0x0037_17c0, 0x002e_19d4)?;
                // Decay the strength.
                c.lwc1(2, p(self_at, spray5::SPEED));
                c.lwc1(0, gp(c, -0x3a9c)); // 83.3333
                c.c_lt_s(2, 0);
                if c.fcc() {
                    c.sw(p(self_at, spray5::SPEED), 0);
                    break 'out;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw(p(self_at, spray5::I124));
                let has124 = c.g[V0][0] != 0;
                c.lwc1(1, p(r32(c, S2), spray_cfg::DECAY));
                if !has124 {
                    c.cycles += 2;
                    c.lwc1(0, gp(c, -0x3a98)); // 0.4
                    c.mul_s(1, 1, 0);
                }
                c.cycles += 3;
                c.mul_s(0, 2, 1);
                c.swc1(0, p(self_at, spray5::SPEED));
                break 'out;
            }
            // Spray off: keep the two emitter slots alive at the position.
            c.cycles += 19;
            c.g[A0][0] = c.lw(p(self_at, spray5::P28));
            c.g[S0][0] = sx(p(self_at, spray5::POS));
            c.lwc1(20, gp(c, -0x3a94)); // 1/60
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0xc60));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0;
            c.mov_s(12, 20);
            call_ptr(c, 0x0037_17c0, 0x002e_1a34)?;
            c.g[A0][0] = c.lw(p(self_at, spray5::P28));
            c.g[A1][0] = c.g[S0][0];
            c.mov_s(12, 20);
            c.g[A2][0] = 0;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0xa50));
            c.g[A3][0] = 0;
            c.g[T0][0] = 0;
            call_ptr(c, 0x0037_17c0, 0x002e_1a54)?;
            c.sw(p(self_at, spray5::SPEED), 0);
            break 'out;
        }
        c.cycles += 10;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore_ra(c, 0x40);
        frame.restore_fpr(c, 20, 0xb0);
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
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};

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

    /// A dynamic emitter with four-slot rings and the timing fields.
    fn put_spray_emitter(c: &mut Cpu, r: &mut Rng, e: u32) {
        c.bus.write32(e + 0x174, 0);
        c.bus.write32(e + 0x178, 4);
        c.bus.write32(e + 0x17c, r.next_u32() % 4);
        c.bus.write32(e + 0x1a0, DATA + 0x6000);
        c.bus.write32(e + 0x1a4, DATA + 0x7000);
        c.bus.write32(e + 0x204, DATA + 0x7800);
        c.bus.write32(e + 0x0c, 1);
        c.bus.write32(e + 0x10, f(0.0));
        c.bus.write32(e + 0x14, f(1.0));
        c.bus.write32(e + 0x1e0, 0);
        for k in 0..4u32 {
            c.bus.write32(e + 0x1b0 + 4 * k, r.edge_f32());
            c.bus.write32(e + 0x1c0 + 4 * k, r.edge_f32());
            c.bus.write32(e + 0x1d0 + 4 * k, r.edge_f32());
        }
    }

    #[test]
    fn dynamic_spray_fx_update_emitter0_matches() {
        check(ee::s09::f_002e0ee8__generated, dynamic_spray_fx_update_emitter0, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2e0e, 1);
            let (fx, obj, vt, curves, cfg, em) =
                (DATA, DATA + 0x4000, DATA + 0x4300, DATA + 0x4400, DATA + 0x4500, DATA + 0x5000);
            put_spray_emitter(c, &mut r, em);
            c.bus.write32(fx + 0x00, obj);
            c.bus.write32(fx + 0x28, em);
            c.bus.write32(fx + 0x2c, curves);
            for k in 0..4u32 {
                c.bus.write32(fx + 0x30 + 4 * k, r.edge_f32());
                c.bus.write32(fx + 0x90 + 4 * k, r.edge_f32());
                c.bus.write32(fx + 0xa0 + 4 * k, r.edge_f32());
                c.bus.write32(obj + 0x370 + 4 * k, r.edge_f32());
            }
            for k in 0..3u32 {
                c.bus.write32(curves + 0xd8 + 4 * k, r.edge_f32());
            }
            c.bus.write32(fx + 0xbc, f(0.2 + (r.next_u32() % 160) as f32 * 0.01));
            c.bus.write32(fx + 0xd0, cfg);
            c.bus.write32(cfg + 0x58, if seed % 3 == 0 { 0 } else { 1 });
            c.bus.write32(obj + 0x88c, obj + 0x4200);
            c.bus.write32(obj + 0x4200 + 0xa0, if seed % 5 == 0 { 1 } else { 0 });
            c.bus.write32(obj + 0x6c0, vt);
            c.bus.write16(vt + 0x10, (r.next_u32() & 0x7c) as u16);
            c.bus.write32(vt + 0x14, FAKE);
            c.bus.write32(gp_at(-0x3afc), f(1.0));
            c.bus.write32(gp_at(-0x3af8), f(1.0 / 60.0));
            c.bus.write32(gp_at(-0x3af4), f(1.0 / 60.0));
            if seed % 7 == 0 {
                c.bus.write32(fx + 0xb0, 0);
            }
            c.g[A0][0] = sx(fx);
        });
    }

    #[test]
    fn board_spark_fx_render_matches() {
        check(ee::s09::f_002db478__generated, board_spark_fx_render, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2db4, 2);
            let (s, em, gfx, state, vt1, ctx2, vt2) = (
                DATA,
                DATA + 0x5000,
                DATA + 0x1000,
                DATA + 0x2000,
                DATA + 0x3000,
                DATA + 0x7000,
                DATA + 0x7200,
            );
            // Emitter step: enabled = 0 makes the real callee return at once.
            c.bus.write32(em + 0x174, 0);
            c.bus.write32(em + 0x1e0, seed % 2);
            // The renderers' state stack and virtual tables.
            c.bus.write32(gp_at(-0x854), gfx);
            c.bus.write32(gfx + 0xe84, state);
            c.bus.write32(gfx + 0x10d8, vt1);
            c.bus.write16(vt1 + 0x290, (r.next_u32() & 0x7c) as u16);
            c.bus.write32(vt1 + 0x294, FAKE);
            c.bus.write32(gp_at(0x2a90), ctx2);
            c.bus.write32(ctx2 + 0x10d8, vt2);
            c.bus.write16(vt2 + 0x270, (r.next_u32() & 0x7c) as u16);
            c.bus.write32(vt2 + 0x274, FAKE);
            // The one-time UV/colour globals.
            c.bus.write32(gp_at(0x23e8), if seed % 3 == 0 { 0 } else { 1 });
            c.bus.write32(gp_at(0x23f8), if seed % 4 == 0 { 0 } else { 1 });
            c.bus.write32(gp_at(0x23fc), if seed % 5 == 0 { 0 } else { 1 });
            c.bus.write32(gp_at(0x23e0), r.next_u32());
            c.bus.write32(gp_at(0x23e4), r.next_u32());
            c.bus.write32(gp_at(0x23f0), r.next_u32());
            c.bus.write32(gp_at(0x23f4), r.next_u32());
            for k in 0..4u32 {
                c.bus.write32(0x004d_57a0 + 4 * k, r.edge_f32());
            }
            // The spark object.
            c.bus.write32(s + 0x20, em);
            c.bus.write32(s + 0x28, r.next_u32());
            for k in 0..12u32 {
                c.bus.write32(s + 0x30 + 4 * k, r.edge_f32());
            }
            for k in 0..3u32 {
                c.bus.write32(s + 0x60 + 4 * k, r.edge_f32());
            }
            for k in 0..5u32 {
                c.bus.write32(s + 0x7c + 4 * k, r.next_u32());
                c.bus.write32(s + 0x90 + 4 * k, r.next_u32());
            }
            c.bus.write32(s + 0x70, if seed % 6 == 0 { 0 } else { 1 });
            c.bus.write32(s + 0x74, if seed % 7 == 0 { 0 } else { 1 });
            c.g[A0][0] = sx(s);
        });
    }

    #[test]
    fn snow_box_render_matches() {
        check(ee::s09::f_002e55d8__generated, snow_box_render, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2e55, 2);
            let (cell, mat, gfx, vt) = (DATA, DATA + 0x2000, DATA + 0x1000, DATA + 0x3000);
            let size = 0.5 + (seed % 3) as f32 * 0.25;
            c.bus.write32(cell + 0x4, (seed / 3) % 4);
            c.bus.write32(cell + 0x8, if seed % 9 == 0 { 0 } else { 1 });
            c.bus.write32(cell + 0xc, f(size));
            // The cell-local position, one of in-range / below / above per axis.
            let mut xyz = [0.0f32; 3];
            for k in 0..3u32 {
                let local = (r.next_u32() as f32 / u32::MAX as f32 - 0.5) * 8.0;
                let case = (seed / (4u32.pow(k))) % 3;
                xyz[k as usize] = match case {
                    0 => local,
                    1 => local + 1.0,
                    _ => local - 2.0,
                };
                c.bus.write32(cell + 0x20 + 4 * k, f(local * size));
            }
            c.f[12] = f(xyz[0]);
            c.f[13] = f(xyz[1]);
            c.f[14] = f(xyz[2]);
            for k in 0..16u32 {
                c.bus.write32(mat + 4 * k, r.edge_f32());
                c.bus.write32(cell + 0x50 + 4 * k, r.edge_f32());
            }
            c.bus.write32(cell + 0x14, r.edge_f32());
            c.bus.write32(cell + 0x18, r.edge_f32());
            for off in [0x98u32, 0x9c, 0xa0, 0xa4, 0xa8, 0xac] {
                c.bus.write32(cell + off, r.edge_f32());
            }
            c.bus.write32(gp_at(-0x854), gfx);
            c.bus.write32(gfx + 0x10d8, vt);
            c.bus.write16(vt + 0x2a8, (r.next_u32() & 0x7c) as u16);
            c.bus.write32(vt + 0x2ac, FAKE);
            c.bus.write16(vt + 0x2b0, (r.next_u32() & 0x7c) as u16);
            c.bus.write32(vt + 0x2b4, FAKE);
            c.g[A0][0] = sx(cell);
            c.g[A1][0] = sx(mat);
        });
    }

    #[test]
    fn rider_base_update_power_up_trail_matches() {
        check(ee::s09::f_002eadd0__generated, rider_base_update_power_up_trail, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2ead, 0);
            let (s, rider, model, skel, nodes, buf) = (
                DATA,
                DATA + 0x2000,
                DATA + 0x4000,
                DATA + 0x4400,
                DATA + 0x6000,
                DATA + 0x4d30,
            );
            c.bus.write32(s, rider);
            c.bus.write32(s + 0xc, r.next_u32() % 5);
            c.bus.write32(s + 0x10, r.next_u32() % 3);
            for k in 0..4u32 {
                c.bus.write32(s + 0xe0 + 4 * k, r.next_u32());
                c.bus.write32(s + 0xf0 + 4 * k, r.next_u32());
            }
            c.bus.write32(rider + 0x2ec, f([-1.0f32, 0.5, 5.0][(seed % 3) as usize]));
            c.bus.write32(rider + 0x77c, model);
            c.bus.write32(rider + 0x780, skel);
            c.bus.write32(rider + 0x8a4, r.next_u32() % 4);
            c.bus.write32(skel + 0x30, nodes);
            c.bus.write32(model + 0xde0, (seed % 2) as u32);
            for off in [0u32, 0x10, 0x20, 0x30, 0x50, 0x64, 0x104, 0x114, 0x124] {
                c.bus.write32(buf + off, r.next_u32());
            }
            for i in 0..4u32 {
                for k in 0..16u32 {
                    c.bus.write32(nodes + 0x40 * i + 4 * k, r.edge_f32());
                }
            }
            c.bus.write32(gp_at(0x1630), if seed % 5 == 0 { 1 } else { 0 });
            c.g[A0][0] = sx(s);
        });
    }

    /// The rider virtual's three queries (riderv(.., 8) and twice (.., 7)),
    /// answered by address so both machines take the same path.
    fn anger_rider_v(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        c.g[V0][0] = match target {
            0x0000_5000 => 0, // riderV(rider, 8): not hidden
            0x0000_5010 => 1, // riderV(rider, 7): rider id
            0x0000_5020 => 2, // riderV(other, 7): rival id
            _ => 0,
        };
        c.g[V1][0] = 0;
        c.pc = ra;
    }

    #[test]
    fn anger_fx_update_matches() {
        check(ee::s09::f_002d4c08__generated, anger_fx_update, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2d4c, 3);
            c.hooks.miss = anger_rider_v;
            let (fx, rider, other, vt, vt2, world, g, players, aggr) = (
                DATA,
                DATA + 0x2000,
                DATA + 0x2400,
                DATA + 0x3000,
                DATA + 0x3100,
                DATA + 0x4000,
                DATA + 0x4200,
                DATA + 0x4400,
                DATA + 0x4800,
            );
            let count = 1 + r.next_u32() % 3;
            c.bus.write32(fx, rider);
            for i in 0..count {
                let slot = fx + 4 + 0x2c * i;
                c.bus.write32(slot + anger_slot::LEVEL, r.edge_f32());
                c.bus.write32(slot + anger_slot::ANGER, r.next_u32() % 6);
                c.bus.write32(slot + anger_slot::T20, r.next_u32() % 45);
                c.bus.write32(slot + anger_slot::T24, r.next_u32() % 35);
                c.bus.write32(slot + anger_slot::T28, r.next_u32() % 26);
                for k in 0..4u32 {
                    c.bus.write32(slot + anger_slot::COLOUR + 4 * k, r.next_u32());
                }
            }
            // The rider's vtable: riderV(.., 8) and riderV(.., 7).
            c.bus.write32(rider + 0x6c0, vt);
            c.bus.write16(vt + 0x40, 0);
            c.bus.write32(vt + 0x44, 0x0000_5000);
            c.bus.write16(vt + 0x38, 0);
            c.bus.write32(vt + 0x3c, 0x0000_5010);
            c.bus.write32(other + 0x6c0, vt2);
            c.bus.write16(vt2 + 0x38, 0);
            c.bus.write32(vt2 + 0x3c, 0x0000_5020);
            c.bus.write32(rider + 0x2ec, r.edge_f32());
            c.bus.write32(rider + anger_rider::SPECIAL, if seed % 11 == 0 { 1 } else { 0 });
            // world -> g -> players -> p[i] -> +0xa8 -> +0x20 -> +4 = other
            c.bus.write32(gp_at(-0x848), world);
            c.bus.write32(world + 0x84, g);
            c.bus.write32(g + 0x84, players);
            c.bus.write32(players + 0x10, count);
            for i in 0..count {
                let rec = DATA + 0x5000 + 0x40 * i;
                let node = DATA + 0x6000 + 0x40 * i;
                c.bus.write32(players + 4 + 4 * i, rec);
                c.bus.write32(rec + 0xa8, node);
                c.bus.write32(node + 0x20, DATA + 0x7000 + 0x40 * i);
                c.bus.write32(DATA + 0x7000 + 0x40 * i + 4, other);
            }
            let nptr = DATA + 0x7800;
            c.bus.write32(g + 0x28, nptr);
            c.bus.write32(nptr, if seed % 13 == 0 { 0 } else { r.next_u32() % 20 });
            // Pre-cached singletons: the aggression interface and the race.
            c.bus.write32(gp_at(-0x1ea4), aggr);
            c.bus.write32(gp_at(-0x1f3c), DATA + 0x4900);
            c.bus.write32(gp_at(-0x1f04), DATA + 0x4a00);
            c.g[A0][0] = sx(fx);
        });
    }

    /// A spray emitter slot at `e` (the fields `DynamicEmitter_SpawnAt`
    /// reads), with the particle ring disabled so it only writes the colour.
    fn put_spray_slot(c: &mut Cpu, r: &mut Rng, e: u32, colours: u32) {
        c.bus.write32(e + 0x174, 0);
        c.bus.write32(e + 0x178, 4);
        c.bus.write32(e + 0x17c, r.next_u32() % 4);
        c.bus.write32(e + 0x1a0, DATA + 0x7000);
        c.bus.write32(e + 0x1a4, DATA + 0x7400);
        c.bus.write32(e + 0x204, colours);
        c.bus.write32(e + 0x0c, 1);
        c.bus.write32(e + 0x10, f(0.0));
        c.bus.write32(e + 0x14, f(1.0));
        c.bus.write32(e + 0x1e0, 0);
        for k in 0..4u32 {
            c.bus.write32(e + 0x1b0 + 4 * k, r.edge_f32());
            c.bus.write32(e + 0x1c0 + 4 * k, r.edge_f32());
            c.bus.write32(e + 0x1d0 + 4 * k, r.edge_f32());
        }
    }

    #[test]
    fn dynamic_spray_fx_update_emitter5_matches() {
        check(ee::s09::f_002e1598__generated, dynamic_spray_fx_update_emitter5, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x2e15, 2);
            let (fx, obj, cfg, em, vt, world, g, players) = (
                DATA,
                DATA + 0x5000,
                DATA + 0x5200,
                DATA + 0x3000,
                DATA + 0x5400,
                DATA + 0x6000,
                DATA + 0x6200,
                DATA + 0x6400,
            );
            c.bus.write32(fx + spray5::OBJ, obj);
            c.bus.write32(fx + spray5::LEVEL, f([0.5f32, 1.2, 2.5][(seed % 3) as usize]));
            c.bus.write32(fx + spray5::P28, em);
            c.bus.write32(fx + spray5::P2C, DATA + 0x5600);
            for k in 0..3u32 {
                c.bus.write32(DATA + 0x5600 + 0xd8 + 4 * k, r.edge_f32());
            }
            for k in 0..4u32 {
                c.bus.write32(fx + spray5::DIR + 4 * k, r.edge_f32());
                c.bus.write32(fx + spray5::COL + 4 * k, r.edge_f32());
                c.bus.write32(fx + spray5::POS + 4 * k, r.edge_f32());
                c.bus.write32(fx + spray5::FRAME_POS + 4 * k, r.edge_f32());
                c.bus.write32(fx + spray5::THROW + 4 * k, r.edge_f32());
                c.bus.write32(obj + 0x370 + 4 * k, r.edge_f32());
            }
            c.bus.write32(fx + spray5::CFG, cfg);
            c.bus.write32(cfg + spray_cfg::ENABLED, if seed % 9 == 0 { 0 } else { 1 });
            c.bus.write32(cfg + spray_cfg::FLAG, if seed % 2 == 0 { 0 } else { 1 });
            c.bus.write32(cfg + spray_cfg::DECAY, f(0.5 + (r.next_u32() % 50) as f32 * 0.01));
            c.bus.write32(fx + spray5::ID4, [0u32, 2, 3][(seed % 3) as usize]);
            c.bus.write32(fx + spray5::SPEED, f([5.0f32, 100.0, 500.0][(seed % 3) as usize]));
            c.bus.write32(fx + spray5::I114, (seed % 2) as u32);
            c.bus.write32(fx + spray5::ALPHA, f([0.1f32, 0.75][(seed % 2) as usize]));
            c.bus.write32(fx + spray5::I124, ((seed / 3) % 2) as u32);
            c.bus.write32(obj + 0xac4, if seed % 7 == 0 { 0 } else { 1 });
            c.bus.write32(obj + 0x6c0, vt);
            c.bus.write16(vt + 0x10, (r.next_u32() & 0x7c) as u16);
            c.bus.write32(vt + 0x14, FAKE);
            put_spray_slot(c, &mut r, em + 5 * 0x210, DATA + 0x8000);
            put_spray_slot(c, &mut r, em + 6 * 0x210, DATA + 0x8100);
            c.bus.write32(gp_at(-0x848), world);
            c.bus.write32(world + 0x84, g);
            c.bus.write32(g + 0x84, players);
            c.bus.write32(players + 0x10, 0);
            c.bus.write32(gp_at(0xa0c), r.next_u32());
            for (off, v) in [
                (-0x3abc, 27.777778f32), (-0x3ab8, 277.77777), (-0x3ab4, 3333.3335),
                (-0x3ab0, 0.3), (-0x3aac, 0.4), (-0x3aa8, 0.8), (-0x3aa4, 0.2),
                (-0x3aa0, 1.0 / 60.0), (-0x3a9c, 83.333336), (-0x3a98, 0.4),
            ] {
                c.bus.write32(gp_at(off), f(v));
            }
            c.g[A0][0] = sx(fx);
        });
    }
}
