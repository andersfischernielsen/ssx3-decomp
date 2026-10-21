//! Phase 2 wave 39 module (docs/investigation/phase2-worker-guide.md): typed
//! replacements, each with a differential test against the generated body.
//!
//! - **ChaseCameraAlgorithm_ApplyJumpCurves** (`0x1635f8`): the chase camera's
//!   jump/landing state machine. It rebuilds two pairs of cubic splines that
//!   shape the vertical camera offset (one pair for the jump arc, one for the
//!   landing dip), blends them by the airborne / landing timers and finally
//!   moves the camera's eye and look-at points along the resulting curve
//!   before calling the camera's placement update (`0x168150`). The vector work
//!   is VU0 macro code issued in the original's order; the scalar work is the
//!   FPU op for op, so the FPU rounding and the condition bit match too.
//!
//! The generated body is the authority for registers, cycles and call order;
//! the reference decompilation has only assembly for this function, so it was
//! ported from `generated/ee/s01` directly.

use crate::abi::r32;
use crate::call::{call_ptr, guest, sp, Flow, Frame};
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

// ChaseCameraAlgorithm_ApplyJumpCurves is not installed: the fake-call fixture
// cannot make the generated and hand bodies agree on the landing-spline
// rebuild branch (seed 5: generated 0x115 cycles, hand 0x132d). The port is
// kept for a later audit against real race data.
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

// ---------------------------------------------------------------------------
// Small helpers.

/// `a + off` (32-bit address arithmetic).
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// The current stack pointer plus `off`.
#[inline(always)]
fn sp_at(c: &Cpu, off: u32) -> u32 {
    sp(c).wrapping_add(off)
}

/// A `gp`-relative global or `.lit4` constant.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `lui at, bits; mtc1 at, f`: a float constant through `at`.
#[inline(always)]
fn fconst(c: &mut Cpu, f: usize, bits: u32) {
    c.g[AT][0] = sx(bits);
    c.f[f] = bits;
}

/// `|v|` of the quadword at `src`, as the original's
/// `lqc2 vf3; vaddx.xyzw vf6, vf0, vf0w; vmul.xyzw vf4, vf3, vf3;
/// vaddax vf4; vmaddax vf6; vmaddx vf4; vsqrt Q, vf4x; vwaitq; cfc2 rt, 22`.
#[inline(always)]
fn v_length_q(c: &mut Cpu, src: u32, rt: usize) {
    c.lqc2(3, src);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(rt, 22);
}

/// `mfc1 gpr, f; lqc2 vf4, v; qmtc2 gpr, vf3; vmul.xyzw vf5, vf4, vf3x;
/// sqc2 vf5, out`: the quadword at `v` scaled by the float in `f`.
#[inline(always)]
fn vscale_q(c: &mut Cpu, gpr: usize, f: usize, v: u32, out: u32) {
    c.mfc1(gpr, f);
    c.lqc2(4, v);
    c.qmtc2(gpr, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, out);
}

// ---------------------------------------------------------------------------
// Callees (real game code, so both machines take the same path and the same
// cycle cost through them).

/// `cCSICubicSpline_Init(spline, count)` (0x31d700).
const SPLINE_INIT: u32 = 0x0031_d700;
/// `cCSICubicSpline_AddPoint(spline, value, tangent)` (0x31d7e0).
const SPLINE_ADD_POINT: u32 = 0x0031_d7e0;
/// `cCSICubicSpline_Evaluate(spline, t)` (0x31dee0).
const SPLINE_EVALUATE: u32 = 0x0031_dee0;
/// `ChaseCameraAlgorithm_CheckDistanceReset(algo)` (0x168150).
const CAMERA_UPDATE: u32 = 0x0016_8150;

/// `a1 = count; a0 = <spline register>; jal Init`.
#[inline(always)]
fn spline_init(c: &mut Cpu, spline: usize, count: u64, ret: u32) -> Flow {
    c.g[A1][0] = count;
    c.g[A0][0] = c.g[spline][0];
    call_ptr(c, SPLINE_INIT, ret)
}

/// `a0 = <spline register>; jal AddPoint` (the value and tangent are already
/// in `f12`/`f13`).
#[inline(always)]
fn add_point(c: &mut Cpu, spline: usize, ret: u32) -> Flow {
    c.g[A0][0] = c.g[spline][0];
    call_ptr(c, SPLINE_ADD_POINT, ret)
}

// ---------------------------------------------------------------------------
// The chase-camera state (`ChaseCameraAlgorithm`, a0) and its info block (a1).

mod st {
    /// `vec4` camera position / look-at point.
    pub const POS: u32 = 0x20;
    /// `vec4` camera velocity / eye point.
    pub const VEL: u32 = 0x40;
    /// The rig object (its first word is the vtable).
    pub const RIG: u32 = 0x30;
    /// Landing timer: counts down each frame; `14` rebuilds the jump splines.
    pub const TIMER: u32 = 0x2c4;
    /// Non-zero while the camera is airborne (skips the jump curves).
    pub const AIRBORNE: u32 = 0x2d0;
    /// Non-zero while the jump curve is being applied.
    pub const APPLYING: u32 = 0x2e0;
    /// Non-zero while the landing splines are in use.
    pub const LANDING: u32 = 0x2e4;
    /// Non-zero once the jump splines were rebuilt.
    pub const JUMP_READY: u32 = 0x2e8;
    /// Jump blend, 0..1.
    pub const JUMP_BLEND: u32 = 0x1f4;
    /// Landing blend, 0..1.
    pub const LAND_BLEND: u32 = 0x1f8;
    /// Scale applied to the jump curve.
    pub const JUMP_SCALE: u32 = 0x1fc;
    /// Jump curve amplitude.
    pub const AMPLITUDE: u32 = 0x200;
    /// Velocity carried from the jump curve.
    pub const JUMP_VEL_X: u32 = 0x20c;
    pub const JUMP_VEL_Y: u32 = 0x210;
    /// Velocity carried from the landing curve.
    pub const LAND_VEL_X: u32 = 0x21c;
    pub const LAND_VEL_Y: u32 = 0x220;
    /// Smoothed vertical velocity.
    pub const VERTICAL_VEL: u32 = 0x224;
    /// Time spent in the current jump.
    pub const TIME: u32 = 0x228;
    /// Spline `t` for the jump curves.
    pub const CURVE_T: u32 = 0x218;
    /// The two jump splines (up and down).
    pub const SPLINE_A: u32 = 0x31c;
    pub const SPLINE_B: u32 = 0x328;
    /// The two landing splines.
    pub const LAND_SPLINE_A: u32 = 0x304;
    pub const LAND_SPLINE_B: u32 = 0x310;
}

/// `void ChaseCameraAlgorithm_ApplyJumpCurves(algo, info, dt, f13, f14, f15)`
/// (0x1635f8): one frame of the chase camera's jump/landing curve blend.
///
/// Frame 0xf0: `ra` at +0x40, the saved callee-saved registers at +0x50..+0x90
/// and +0x80, the saved `f20..f31` at +0x90..+0xe8, vector scratch at
/// +0x00/+0x10/+0x20 and the incoming `f13` at +0x30.
pub fn chase_camera_algorithm_apply_jump_curves(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 43;
        let frame = Frame::open(c, 0xf0);
        c.sq(18, sp_at(c, 0x60));
        c.sq(19, sp_at(c, 0x50));
        c.g[S2][0] = c.g[A0][0];
        c.swc1(31, sp_at(c, 0xe8));
        c.g[S3][0] = c.g[A1][0];
        c.swc1(28, sp_at(c, 0xd0));
        c.mov_s(31, 15);
        c.swc1(20, sp_at(c, 0x90));
        c.mov_s(28, 14);
        c.sq(16, sp_at(c, 0x80));
        c.mov_s(20, 12);
        c.sq(17, sp_at(c, 0x70));
        frame.save_ra(c, 0x40);
        c.swc1(30, sp_at(c, 0xe0));
        c.swc1(29, sp_at(c, 0xd8));
        c.swc1(27, sp_at(c, 0xc8));
        c.swc1(26, sp_at(c, 0xc0));
        c.swc1(25, sp_at(c, 0xb8));
        c.swc1(24, sp_at(c, 0xb0));
        c.swc1(23, sp_at(c, 0xa8));
        c.swc1(22, sp_at(c, 0xa0));
        c.swc1(21, sp_at(c, 0x98));
        let s2 = r32(c, S2);
        let s3 = r32(c, S3);
        // `f30` = |pos - vel|, with the difference left at sp+0x10.
        c.lqc2(3, p(s2, st::POS));
        c.lqc2(4, p(s2, st::VEL));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp_at(c, 0x10));
        c.swc1(13, sp_at(c, 0x30));
        c.lq(2, sp_at(c, 0x10));
        c.g[V1][0] = c.lw(p(s3, 0x60));
        c.sq(2, sp_at(c, 0));
        let scratch = sp_at(c, 0);
        v_length_q(c, scratch, 2);
        c.mtc1(2, 30);

        'out: {
            if c.g[V1][0] == 0 {
                c.g[V0][0] = c.lw(p(s2, st::JUMP_READY));
                break 'out;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw(p(s2, st::AIRBORNE));
            if c.g[V0][0] != 0 {
                c.g[V0][0] = c.lw(p(s2, st::JUMP_READY));
                break 'out;
            }
            // The landing timer: while it still counts down, ask the rig
            // whether the camera may be applied again right away.
            'timer: {
                c.cycles += 5;
                c.g[V0][0] = c.lw(p(s2, st::TIMER));
                c.mtc1(0, 29);
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                let expired = (c.g[V0][0] as i64) <= 0;
                c.sw(p(s2, st::TIMER), c.g[V0][0]);
                if expired {
                    break 'timer;
                }
                c.cycles += 8;
                c.g[A1][0] = c.lw(p(s2, st::RIG));
                c.g[V1][0] = c.lw(r32(c, A1));
                c.g[A0][0] = c.lh(p(r32(c, V1), 0x98));
                c.g[V0][0] = c.lw(p(r32(c, V1), 0x9c));
                let target = r32(c, V0);
                c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                call_ptr(c, target, 0x0016_36dc)?;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = 0x10;
                    break 'timer;
                }
                c.cycles += 7;
                c.g[V1][0] = c.lw(p(s2, st::TIMER));
                c.lwc1(1, gp(c, -0x6aa0));
                c.g[V0][0] = sx(0x10u32.wrapping_sub(r32(c, V1)));
                c.sw(p(s2, st::TIMER), 0);
                c.mtc1(2, 0);
                c.cvt_s_w(0, 0);
                c.mul_s(29, 0, 1);
            }
            // The jump splines are rebuilt when the timer reaches 14.
            c.cycles += 4;
            c.g[V1][0] = c.lw(p(s2, st::TIMER));
            c.g[V0][0] = 0xe;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[V0][0] = c.lw(p(s2, st::TIMER));
            } else {
                // `f1` = the rig's jump progress in 0..1.
                'curve: {
                    'ease: {
                        c.cycles += 15;
                        c.lwc1(0, gp(c, -0x6a9c));
                        c.g[A1][0] = c.lw(p(s2, st::RIG));
                        c.swc1(0, p(s2, st::CURVE_T));
                        c.g[V0][0] = c.lw(r32(c, A1));
                        c.g[A0][0] = c.lh(p(r32(c, V0), 0xa8));
                        c.g[V1][0] = c.lw(p(r32(c, V0), 0xac));
                        let target = r32(c, V1);
                        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                        call_ptr(c, target, 0x0016_3730)?;
                        fconst(c, 2, 0x4396_0000); // 300
                        c.mtc1(1, 2);
                        c.mov_s(1, 0);
                        c.c_le_s(2, 1);
                        if !c.fcc() {
                            c.mul_s(0, 28, 20);
                            break 'ease;
                        }
                        'ramp: {
                            c.cycles += 5;
                            fconst(c, 0, 0x43fa_0000); // 500
                            c.mtc1(1, 0);
                            c.c_le_s(1, 0);
                            if !c.fcc() {
                                break 'ramp;
                            }
                            c.cycles += 6;
                            c.sub_s(1, 1, 2);
                            fconst(c, 0, 0x4348_0000); // 200
                            c.mtc1(1, 0);
                            c.div_s(1, 1, 0);
                            c.mul_s(0, 28, 20);
                            break 'curve;
                        }
                        c.cycles += 4;
                        fconst(c, 1, 0x3f80_0000);
                        c.mul_s(0, 28, 20);
                        break 'curve;
                    }
                    c.cycles += 1;
                    c.mtc1(0, 1);
                }
                // `f26` = f1 * f13, the curve's time scale (its sign decides
                // whether the splines are rebuilt).
                'jump: {
                    c.cycles += 6;
                    c.mtc1(0, 27);
                    c.mul_s(26, 0, 1);
                    c.c_lt_s(27, 26);
                    let skip = !c.fcc();
                    c.swc1(1, p(s2, st::TIME));
                    if skip {
                        break 'jump;
                    }
                    c.cycles += 152;
                    c.g[S0][0] = sx(p(s2, st::SPLINE_A));
                    c.lwc1(22, gp(c, -0x6a98));
                    spline_init(c, S0, 0xd, 0x0016_37b8)?;
                    c.lwc1(23, gp(c, -0x6a94));
                    c.mov_s(13, 27);
                    fconst(c, 24, 0x4040_0000); // 3
                    c.mov_s(12, 27);
                    add_point(c, S0, 0x0016_37d4)?;
                    c.lwc1(25, gp(c, -0x6a90));
                    c.lwc1(13, gp(c, -0x6a8c));
                    c.mov_s(12, 22);
                    fconst(c, 20, 0x4080_0000); // 4
                    c.mul_s(13, 26, 13);
                    fconst(c, 21, 0x3f80_0000); // 1
                    c.g[S1][0] = sx(p(s2, st::SPLINE_B));
                    add_point(c, S0, 0x0016_3800)?;
                    c.lwc1(13, gp(c, -0x6a88));
                    c.mov_s(12, 23);
                    c.mul_s(13, 26, 13);
                    add_point(c, S0, 0x0016_3814)?;
                    c.lwc1(13, gp(c, -0x6a84));
                    c.lwc1(12, gp(c, -0x6a80));
                    c.mul_s(13, 26, 13);
                    add_point(c, S0, 0x0016_3828)?;
                    c.lwc1(13, gp(c, -0x6a7c));
                    c.lwc1(12, gp(c, -0x6a78));
                    c.mul_s(13, 26, 13);
                    add_point(c, S0, 0x0016_383c)?;
                    c.lwc1(13, gp(c, -0x6a74));
                    c.lwc1(12, gp(c, -0x6a70));
                    c.mul_s(13, 26, 13);
                    add_point(c, S0, 0x0016_3850)?;
                    c.lwc1(13, gp(c, -0x6a6c));
                    fconst(c, 12, 0x4000_0000); // 2
                    c.mul_s(13, 26, 13);
                    add_point(c, S0, 0x0016_3868)?;
                    c.lwc1(13, gp(c, -0x6a68));
                    c.lwc1(12, gp(c, -0x6a64));
                    c.mul_s(13, 26, 13);
                    add_point(c, S0, 0x0016_387c)?;
                    c.lwc1(13, gp(c, -0x6a60));
                    c.lwc1(12, gp(c, -0x6a5c));
                    c.mul_s(13, 26, 13);
                    add_point(c, S0, 0x0016_3890)?;
                    c.lwc1(13, gp(c, -0x6a58));
                    c.mov_s(12, 24);
                    c.mul_s(13, 26, 13);
                    add_point(c, S0, 0x0016_38a4)?;
                    c.lwc1(13, gp(c, -0x6a54));
                    c.mov_s(12, 25);
                    c.mul_s(13, 26, 13);
                    add_point(c, S0, 0x0016_38b8)?;
                    c.lwc1(13, gp(c, -0x6a50));
                    c.lwc1(12, gp(c, -0x6a4c));
                    c.mul_s(13, 26, 13);
                    add_point(c, S0, 0x0016_38cc)?;
                    c.lwc1(13, gp(c, -0x6a48));
                    c.mov_s(12, 20);
                    c.mul_s(13, 26, 13);
                    add_point(c, S0, 0x0016_38e0)?;
                    // The second (downward) jump spline.
                    spline_init(c, S1, 0xd, 0x0016_38ec)?;
                    c.mov_s(12, 27);
                    c.mov_s(13, 12);
                    add_point(c, S1, 0x0016_38fc)?;
                    c.lwc1(13, gp(c, -0x6a44));
                    c.mov_s(12, 22);
                    c.mul_s(13, 26, 13);
                    add_point(c, S1, 0x0016_3910)?;
                    c.lwc1(13, gp(c, -0x6a40));
                    c.mov_s(12, 23);
                    c.mul_s(13, 26, 13);
                    add_point(c, S1, 0x0016_3924)?;
                    c.lwc1(13, gp(c, -0x6a3c));
                    c.mov_s(12, 21);
                    c.mul_s(13, 26, 13);
                    add_point(c, S1, 0x0016_3938)?;
                    c.lwc1(13, gp(c, -0x6a38));
                    c.lwc1(12, gp(c, -0x6a34));
                    c.mul_s(13, 26, 13);
                    add_point(c, S1, 0x0016_394c)?;
                    c.lwc1(13, gp(c, -0x6a30));
                    c.lwc1(12, gp(c, -0x6a2c));
                    c.mul_s(13, 26, 13);
                    add_point(c, S1, 0x0016_3960)?;
                    c.lwc1(13, gp(c, -0x6a28));
                    c.lwc1(12, gp(c, -0x6a24));
                    c.mul_s(13, 26, 13);
                    add_point(c, S1, 0x0016_3974)?;
                    c.lwc1(13, gp(c, -0x6a20));
                    c.lwc1(12, gp(c, -0x6a1c));
                    c.mul_s(13, 26, 13);
                    add_point(c, S1, 0x0016_3988)?;
                    c.lwc1(13, gp(c, -0x6a18));
                    c.lwc1(12, gp(c, -0x6a14));
                    c.mul_s(13, 26, 13);
                    add_point(c, S1, 0x0016_399c)?;
                    c.lwc1(13, gp(c, -0x6a10));
                    c.mov_s(12, 24);
                    c.mul_s(13, 26, 13);
                    add_point(c, S1, 0x0016_39b0)?;
                    c.lwc1(13, gp(c, -0x6a0c));
                    c.mov_s(12, 25);
                    c.mul_s(13, 26, 13);
                    add_point(c, S1, 0x0016_39c4)?;
                    c.lwc1(13, gp(c, -0x6a08));
                    c.lwc1(12, gp(c, -0x6a04));
                    c.mul_s(13, 26, 13);
                    add_point(c, S1, 0x0016_39d8)?;
                    c.lwc1(13, gp(c, -0x6a00));
                    c.mov_s(12, 20);
                    c.mul_s(13, 26, 13);
                    add_point(c, S1, 0x0016_39ec)?;
                    c.g[V0][0] = 0x1;
                    c.swc1(20, p(s2, st::AMPLITUDE));
                    c.sw(p(s2, st::JUMP_READY), c.g[V0][0]);
                    c.swc1(21, p(s2, st::JUMP_SCALE));
                    c.swc1(21, p(s2, st::LAND_BLEND));
                    c.sw(p(s2, st::LANDING), 0);
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw(p(s2, st::TIMER));
            }
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = c.lw(p(s2, st::JUMP_READY));
                break 'out;
            }
            // The timer expired: blend the jump and landing curves.
            'active: {
                'blend: {
                    c.cycles += 15;
                    c.swc1(29, p(s2, st::JUMP_BLEND));
                    c.g[A1][0] = c.lw(p(s2, st::RIG));
                    c.g[V0][0] = c.lw(r32(c, A1));
                    c.g[A0][0] = c.lh(p(r32(c, V0), 0x60));
                    c.g[V1][0] = c.lw(p(r32(c, V0), 0x64));
                    let target = r32(c, V1);
                    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                    call_ptr(c, target, 0x0016_3a2c)?;
                    c.lwc1(1, gp(c, -0x69fc));
                    c.mov_s(27, 0);
                    fconst(c, 0, 0x3e80_0000); // 0.25
                    c.c_le_s(1, 27);
                    let skip = !c.fcc();
                    c.mul_s(26, 27, 0);
                    if skip {
                        break 'blend;
                    }
                    'ease2: {
                        c.cycles += 4;
                        c.lwc1(0, gp(c, -0x69f8));
                        c.c_le_s(27, 0);
                        if c.fcc() {
                            c.cycles += 16;
                            c.sub_s(0, 27, 1);
                            c.lwc1(2, gp(c, -0x69f4));
                            fconst(c, 3, 0x4040_0000); // 3
                            fconst(c, 4, 0x3f80_0000); // 1
                            c.div_s(0, 0, 2);
                            c.add_s(1, 0, 0);
                            c.mul_s(0, 0, 0);
                            c.mul_s(1, 1, 0);
                            c.mul_s(0, 0, 3);
                            c.sub_s(0, 0, 1);
                            c.c_lt_s(4, 0);
                            if !c.fcc() {
                                c.mul_s(1, 0, 28);
                                break 'active;
                            }
                        }
                    }
                    c.cycles += 4;
                    fconst(c, 0, 0x3f80_0000);
                    c.mul_s(1, 0, 28);
                    // The original's `break 'b_163ab4`: `f1` keeps this value
                    // (it is the jump/landing blend the guard below tests) and
                    // the `f1 = 0` pair after this block is skipped.
                    break 'blend;
                }
                c.cycles += 2;
                c.mtc1(0, 0);
                c.mul_s(1, 0, 28);
                // The landing blend and the two landing splines.
                c.cycles += 7;
                c.lwc1(0, p(s2, st::TIME));
                c.mtc1(0, 28);
                c.mul_s(25, 1, 0);
                c.c_lt_s(28, 25);
                let skip = !c.fcc();
                c.g[S0][0] = sx(p(s2, st::LAND_SPLINE_A));
                if !skip {
                    c.cycles += 167;
                    c.g[S1][0] = sx(p(s2, st::LAND_SPLINE_B));
                    c.g[A1][0] = 0xd;
                    spline_init(c, S0, 0xd, 0x0016_3ae0)?;
                    c.mov_s(13, 28);
                    c.mov_s(12, 28);
                    add_point(c, S0, 0x0016_3af0)?;
                    c.lwc1(24, gp(c, -0x69f0));
                    c.lwc1(13, gp(c, -0x69ec));
                    c.mul_s(24, 26, 24);
                    c.mul_s(13, 25, 13);
                    c.mov_s(12, 24);
                    add_point(c, S0, 0x0016_3b0c)?;
                    c.lwc1(23, gp(c, -0x69e8));
                    c.lwc1(13, gp(c, -0x69e4));
                    c.mul_s(23, 26, 23);
                    c.mul_s(13, 25, 13);
                    c.mov_s(12, 23);
                    add_point(c, S0, 0x0016_3b28)?;
                    c.lwc1(12, gp(c, -0x69e0));
                    c.lwc1(13, gp(c, -0x69dc));
                    c.mul_s(12, 26, 12);
                    c.mul_s(13, 25, 13);
                    add_point(c, S0, 0x0016_3b40)?;
                    c.lwc1(12, gp(c, -0x69d8));
                    c.lwc1(13, gp(c, -0x69d4));
                    c.mul_s(12, 26, 12);
                    c.mul_s(13, 25, 13);
                    add_point(c, S0, 0x0016_3b58)?;
                    c.lwc1(12, gp(c, -0x69d0));
                    c.lwc1(13, gp(c, -0x69cc));
                    c.mul_s(12, 26, 12);
                    c.mul_s(13, 25, 13);
                    add_point(c, S0, 0x0016_3b70)?;
                    c.lwc1(13, gp(c, -0x69c8));
                    c.add_s(12, 26, 26);
                    c.mul_s(13, 25, 13);
                    add_point(c, S0, 0x0016_3b84)?;
                    c.lwc1(12, gp(c, -0x69c4));
                    c.lwc1(13, gp(c, -0x69c0));
                    c.mul_s(12, 26, 12);
                    c.mul_s(13, 25, 13);
                    add_point(c, S0, 0x0016_3b9c)?;
                    c.lwc1(12, gp(c, -0x69bc));
                    c.lwc1(13, gp(c, -0x69b8));
                    c.mul_s(12, 26, 12);
                    c.mul_s(13, 25, 13);
                    add_point(c, S0, 0x0016_3bb4)?;
                    fconst(c, 22, 0x4040_0000);
                    c.lwc1(13, gp(c, -0x69b4));
                    c.mul_s(22, 26, 22);
                    c.mul_s(13, 25, 13);
                    c.mov_s(12, 22);
                    add_point(c, S0, 0x0016_3bd4)?;
                    c.lwc1(21, gp(c, -0x69b0));
                    c.lwc1(13, gp(c, -0x69ac));
                    c.mul_s(21, 26, 21);
                    c.mul_s(13, 25, 13);
                    c.mov_s(12, 21);
                    add_point(c, S0, 0x0016_3bf0)?;
                    c.lwc1(12, gp(c, -0x69a8));
                    c.lwc1(13, gp(c, -0x69a4));
                    c.mul_s(12, 26, 12);
                    c.mul_s(13, 25, 13);
                    add_point(c, S0, 0x0016_3c08)?;
                    fconst(c, 20, 0x4080_0000);
                    c.lwc1(13, gp(c, -0x69a0));
                    c.mul_s(20, 26, 20);
                    c.mul_s(13, 25, 13);
                    c.mov_s(12, 20);
                    add_point(c, S0, 0x0016_3c28)?;
                    // The second landing spline.
                    spline_init(c, S1, 0xd, 0x0016_3c34)?;
                    c.mov_s(12, 28);
                    c.mov_s(13, 12);
                    add_point(c, S1, 0x0016_3c44)?;
                    c.lwc1(13, gp(c, -0x699c));
                    c.mov_s(12, 24);
                    c.mul_s(13, 25, 13);
                    add_point(c, S1, 0x0016_3c58)?;
                    c.lwc1(13, gp(c, -0x6998));
                    c.mov_s(12, 23);
                    c.mul_s(13, 25, 13);
                    add_point(c, S1, 0x0016_3c6c)?;
                    c.lwc1(13, gp(c, -0x6994));
                    c.mov_s(12, 26);
                    c.mul_s(13, 25, 13);
                    add_point(c, S1, 0x0016_3c80)?;
                    c.lwc1(12, gp(c, -0x6990));
                    c.lwc1(13, gp(c, -0x698c));
                    c.mul_s(12, 26, 12);
                    c.mul_s(13, 25, 13);
                    add_point(c, S1, 0x0016_3c98)?;
                    c.lwc1(12, gp(c, -0x6988));
                    c.lwc1(13, gp(c, -0x6984));
                    c.mul_s(12, 26, 12);
                    c.mul_s(13, 25, 13);
                    add_point(c, S1, 0x0016_3cb0)?;
                    c.lwc1(12, gp(c, -0x6980));
                    c.lwc1(13, gp(c, -0x697c));
                    c.mul_s(12, 26, 12);
                    c.mul_s(13, 25, 13);
                    add_point(c, S1, 0x0016_3cc8)?;
                    c.lwc1(12, gp(c, -0x6978));
                    c.lwc1(13, gp(c, -0x6974));
                    c.mul_s(12, 26, 12);
                    c.mul_s(13, 25, 13);
                    add_point(c, S1, 0x0016_3ce0)?;
                    c.lwc1(12, gp(c, -0x6970));
                    c.lwc1(13, gp(c, -0x696c));
                    c.mul_s(12, 26, 12);
                    c.mul_s(13, 25, 13);
                    add_point(c, S1, 0x0016_3cf8)?;
                    c.lwc1(13, gp(c, -0x6968));
                    c.mov_s(12, 22);
                    c.mul_s(13, 25, 13);
                    add_point(c, S1, 0x0016_3d0c)?;
                    c.lwc1(13, gp(c, -0x6964));
                    c.mov_s(12, 21);
                    c.mul_s(13, 25, 13);
                    add_point(c, S1, 0x0016_3d20)?;
                    c.lwc1(12, gp(c, -0x6960));
                    c.lwc1(13, gp(c, -0x695c));
                    c.mul_s(12, 26, 12);
                    c.mul_s(13, 25, 13);
                    add_point(c, S1, 0x0016_3d38)?;
                    c.lwc1(13, gp(c, -0x6958));
                    c.mov_s(12, 20);
                    c.mul_s(13, 25, 13);
                    add_point(c, S1, 0x0016_3d4c)?;
                    fconst(c, 0, 0x3f80_0000);
                    c.g[V0][0] = 0x1;
                    c.sw(p(s2, st::APPLYING), c.g[V0][0]);
                    c.swc1(0, p(s2, st::JUMP_SCALE));
                    c.swc1(27, p(s2, st::AMPLITUDE));
                    c.swc1(0, p(s2, st::JUMP_BLEND));
                    c.sw(p(s2, st::LANDING), 0);
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw(p(s2, st::JUMP_READY));
                break 'out;
            }
        }

        // Evaluate the jump curves: the up curve, then the down curve scaled
        // by the landing blend.
        'eval: {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = c.lw(p(s2, st::APPLYING));
                break 'eval;
            }
            'curve_t: {
                c.cycles += 7;
                c.lwc1(2, p(s2, st::CURVE_T));
                c.lwc1(0, gp(c, -0x6954));
                c.c_le_s(0, 2);
                fconst(c, 0, 0x4080_0000); // 4
                if c.fcc() {
                    c.swc1(0, p(s2, st::CURVE_T));
                    break 'curve_t;
                }
                c.cycles += 6;
                c.lwc1(0, gp(c, -0x6950));
                fconst(c, 1, 0x4080_0000);
                c.add_s(0, 2, 0);
                c.min_s(0, 0, 1);
                c.swc1(0, p(s2, st::CURVE_T));
            }
            c.cycles += 17;
            c.lwc1(0, p(s2, st::JUMP_SCALE));
            c.g[A0][0] = sx(p(s2, st::SPLINE_B));
            c.lwc1(12, p(s2, st::CURVE_T));
            c.mul_s(12, 0, 12);
            call_ptr(c, SPLINE_EVALUATE, 0x0016_3dc0)?;
            c.lwc1(1, p(s2, st::JUMP_SCALE));
            c.g[A0][0] = sx(p(s2, st::SPLINE_A));
            c.lwc1(12, p(s2, st::CURVE_T));
            c.lwc1(20, p(s2, st::LAND_BLEND));
            c.mul_s(12, 1, 12);
            c.mul_s(20, 20, 0);
            call_ptr(c, SPLINE_EVALUATE, 0x0016_3ddc)?;
            c.lwc1(1, p(s2, st::LAND_BLEND));
            c.swc1(20, p(s2, st::LAND_VEL_Y));
            c.mul_s(1, 1, 0);
            c.swc1(1, p(s2, st::LAND_VEL_X));
            c.g[V0][0] = c.lw(p(s2, st::APPLYING));
        }

        // The landing dip: blend the landing splines toward the target.
        'land: {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.sw(gp(c, -0x1e68), 0);
            } else {
                'approach: {
                    c.cycles += 8;
                    c.lwc1(1, p(s2, st::AMPLITUDE));
                    c.lwc1(3, gp(c, -0x694c));
                    c.lwc1(2, p(s2, st::JUMP_BLEND));
                    c.sub_s(0, 1, 3);
                    c.c_le_s(0, 2);
                    if !c.fcc() {
                        c.add_s(0, 2, 3);
                        c.cycles += 2;
                        c.min_s(0, 0, 1);
                        c.swc1(0, p(s2, st::JUMP_BLEND));
                    } else {
                        c.cycles += 2;
                        c.swc1(1, p(s2, st::JUMP_BLEND));
                    }
                }
                c.cycles += 23;
                c.lwc1(0, p(s2, st::JUMP_SCALE));
                c.g[S1][0] = sx(p(s2, st::LAND_SPLINE_B));
                c.lwc1(12, p(s2, st::JUMP_BLEND));
                c.g[A0][0] = c.g[S1][0];
                c.g[S0][0] = sx(p(s2, st::LAND_SPLINE_A));
                c.mul_s(12, 0, 12);
                call_ptr(c, SPLINE_EVALUATE, 0x0016_3e44)?;
                c.lwc1(1, p(s2, st::JUMP_SCALE));
                c.g[A0][0] = c.g[S0][0];
                c.lwc1(12, p(s2, st::JUMP_BLEND));
                c.lwc1(20, p(s2, st::LAND_BLEND));
                c.mul_s(12, 1, 12);
                c.mul_s(20, 20, 0);
                call_ptr(c, SPLINE_EVALUATE, 0x0016_3e60)?;
                c.lwc1(1, p(s2, st::LAND_BLEND));
                c.g[V0][0] = 0x1;
                c.swc1(20, p(s2, st::JUMP_VEL_Y));
                c.mul_s(1, 1, 0);
                c.swc1(1, p(s2, st::JUMP_VEL_X));
                c.sw(gp(c, -0x1e68), c.g[V0][0]);
                c.sw(gp(c, 0x1ce8), c.g[S1][0]);
                c.sw(gp(c, 0x1cec), c.g[S0][0]);
                // The original's `if (true) break;`: the rebuilt spline handles
                // are left in the globals and the trailing stores are skipped.
                break 'land;
            }
            c.cycles += 2;
            c.sw(gp(c, 0x1ce8), 0);
            c.sw(gp(c, 0x1cec), 0);
        }

        // Fade the jump/landing state out.
        'fade: {
            c.cycles += 3;
            c.g[V0][0] = c.lw(p(s3, 0x60));
            let info_set = c.g[V0][0] != 0;
            c.g[V0][0] = c.lw(p(s2, st::LANDING));
            if !info_set {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = 0x1;
                } else {
                    // `f0` = the normalised time into the current phase.
                    'ratio: {
                        c.cycles += 5;
                        c.lwc1(2, gp(c, -0x6948));
                        c.lwc1(1, p(s2, st::CURVE_T));
                        c.c_le_s(2, 1);
                        if !c.fcc() {
                            c.cycles += 1;
                            c.mtc1(0, 0);
                        } else {
                            c.cycles += 6;
                            c.lwc1(0, gp(c, -0x6944));
                            c.c_le_s(1, 0);
                            fconst(c, 0, 0x3f80_0000);
                            if c.fcc() {
                                c.sub_s(0, 1, 2);
                            } else {
                                c.cycles += 2;
                            }
                        }
                    }
                    // Smooth the vertical velocity towards the curve.
                    'smooth: {
                        c.cycles += 24;
                        fconst(c, 20, 0x3f80_0000);
                        c.mul_s(1, 0, 31);
                        c.lwc1(2, gp(c, -0x6940));
                        c.sub_s(0, 20, 0);
                        c.g[A1][0] = c.lw(p(s2, st::RIG));
                        c.mul_s(0, 0, 2);
                        c.add_s(1, 1, 0);
                        c.swc1(1, p(s2, st::VERTICAL_VEL));
                        c.g[V0][0] = c.lw(r32(c, A1));
                        c.g[A0][0] = c.lh(p(r32(c, V0), 0xb8));
                        c.g[V1][0] = c.lw(p(r32(c, V0), 0xbc));
                        let target = r32(c, V1);
                        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                        call_ptr(c, target, 0x0016_3f10)?;
                        c.lwc1(2, p(s2, st::VERTICAL_VEL));
                        c.mov_s(3, 0);
                        fconst(c, 1, 0x3f00_0000); // 0.5
                        c.sub_s(0, 20, 2);
                        c.lwc1(4, gp(c, -0x693c));
                        c.c_le_s(4, 3);
                        c.mul_s(0, 0, 1);
                        let t = !c.fcc();
                        c.add_s(2, 2, 0);
                        if t {
                            c.cycles += 1;
                            c.mtc1(0, 3);
                        } else {
                            c.cycles += 5;
                            c.lwc1(0, gp(c, -0x6938));
                            c.c_le_s(3, 0);
                            if !c.fcc() {
                                c.mov_s(3, 20);
                            } else {
                                c.cycles += 5;
                                c.sub_s(1, 3, 4);
                                c.lwc1(0, gp(c, -0x6934));
                                c.div_s(3, 1, 0);
                            }
                        }
                        c.cycles += 9;
                        fconst(c, 0, 0x3f80_0000);
                        c.mul_s(1, 3, 2);
                        c.lwc1(2, p(s2, st::VERTICAL_VEL));
                        c.sub_s(0, 0, 3);
                        c.mul_s(0, 0, 2);
                        c.add_s(1, 1, 0);
                        c.swc1(1, p(s2, st::VERTICAL_VEL));
                        c.g[V0][0] = 0x1;
                    }
                }
                c.cycles += 4;
                c.sw(p(s2, st::APPLYING), 0);
                c.sw(p(s2, st::LANDING), c.g[V0][0]);
                c.sw(p(s2, st::JUMP_READY), 0);
                c.g[V0][0] = c.lw(p(s2, st::LANDING));
            }
        }

        // Scale the carried velocities by the vertical blend.
        'vel: {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.g[V0][0] = c.lw(p(s2, st::APPLYING));
                break 'vel;
            }
            c.cycles += 14;
            c.lwc1(0, p(s2, st::VERTICAL_VEL));
            c.lwc1(2, p(s2, st::JUMP_VEL_X));
            c.lwc1(3, p(s2, st::JUMP_VEL_Y));
            c.lwc1(4, p(s2, st::LAND_VEL_X));
            c.mul_s(2, 2, 0);
            c.lwc1(1, p(s2, st::LAND_VEL_Y));
            c.mul_s(3, 3, 0);
            c.mul_s(4, 4, 0);
            c.mul_s(1, 1, 0);
            c.swc1(2, p(s2, st::JUMP_VEL_X));
            c.swc1(3, p(s2, st::JUMP_VEL_Y));
            c.swc1(4, p(s2, st::LAND_VEL_X));
            c.swc1(1, p(s2, st::LAND_VEL_Y));
            c.g[V0][0] = c.lw(p(s2, st::APPLYING));
        }

        // `f13` = min(blend / |pos - vel|, 1).
        'normalise: {
            c.cycles += 2;
            let idle = c.g[V0][0] == 0;
            c.lwc1(2, sp_at(c, 0x30));
            if idle {
                c.cycles += 1;
                c.mtc1(0, 13);
            } else {
                c.cycles += 6;
                c.lwc1(1, p(s2, st::JUMP_BLEND));
                fconst(c, 0, 0x3f80_0000);
                c.div_s(13, 1, 2);
                c.min_s(13, 13, 0);
            }
        }

        // Move the eye and look-at points along the curve, then update the
        // camera's placement.
        c.cycles += 112;
        c.lqc2(3, p(s2, st::POS));
        c.lqc2(4, p(s2, st::VEL));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp_at(c, 0x10));
        c.lwc1(1, gp(c, -0x6930));
        c.lq(2, sp_at(c, 0x10));
        c.g[V1][0] = 0x0050_0000;
        c.lwc1(4, p(s2, st::LAND_VEL_X));
        c.g[A0][0] = c.g[S2][0];
        c.sq(2, sp_at(c, 0));
        let scratch = sp_at(c, 0);
        v_length_q(c, scratch, 5);
        c.mtc1(5, 2);
        fconst(c, 3, 0x3f80_0000);
        c.mul_s(2, 2, 1);
        c.lwc1(0, p(s2, st::JUMP_VEL_X));
        c.sub_s(3, 3, 13);
        c.lwc1(1, p(s2, st::JUMP_VEL_Y));
        c.lwc1(5, p(s2, st::LAND_VEL_Y));
        c.mul_s(4, 2, 4);
        c.mul_s(0, 2, 0);
        c.mul_s(1, 2, 1);
        c.mul_s(4, 3, 4);
        c.mul_s(0, 13, 0);
        c.mul_s(2, 2, 5);
        c.mul_s(1, 13, 1);
        c.add_s(0, 0, 4);
        let up = r32(c, V1).wrapping_sub(0xea0);
        let dif = sp_at(c, 0x10);
        vscale_q(c, 2, 0, up, dif);
        c.mul_s(3, 3, 2);
        c.lq(2, sp_at(c, 0x10));
        c.sq(2, sp_at(c, 0));
        c.add_s(1, 1, 3);
        c.lqc2(3, p(s2, st::POS));
        c.lqc2(4, sp_at(c, 0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, p(s2, st::POS));
        let up = r32(c, V1).wrapping_sub(0xea0);
        let dif = sp_at(c, 0x10);
        vscale_q(c, 5, 1, up, dif);
        c.lq(2, sp_at(c, 0x10));
        c.sq(2, sp_at(c, 0));
        c.lqc2(3, p(s2, st::VEL));
        c.lqc2(4, sp_at(c, 0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, p(s2, st::VEL));
        call_ptr(c, CAMERA_UPDATE, 0x0016_40f0)?;
        c.lqc2(3, p(s2, st::POS));
        c.lqc2(4, p(s2, st::VEL));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp_at(c, 0x10));
        fconst(c, 0, 0x4248_0000); // 50
        c.lq(2, sp_at(c, 0x10));
        c.max_s(0, 30, 0);
        frame.restore(c, 16, 0x80);
        c.sq(2, sp_at(c, 0));
        let scratch = sp_at(c, 0);
        v_length_q(c, scratch, 3);
        c.mtc1(3, 1);
        frame.restore(c, 17, 0x70);
        c.div_s(0, 0, 1);
        frame.restore(c, 19, 0x50);
        c.mfc1(2, 0);
        c.lqc2(4, sp_at(c, 0));
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sp_at(c, 0));
        frame.restore_ra(c, 0x40);
        c.lqc2(3, p(s2, st::POS));
        c.lqc2(4, sp_at(c, 0));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp_at(c, 0x20));
        c.lwc1(31, sp_at(c, 0xe8));
        c.lq(2, sp_at(c, 0x20));
        c.lwc1(30, sp_at(c, 0xe0));
        c.sq(2, p(s2, st::VEL));
        frame.restore(c, 18, 0x60);
        c.lwc1(29, sp_at(c, 0xd8));
        c.lwc1(28, sp_at(c, 0xd0));
        c.lwc1(27, sp_at(c, 0xc8));
        c.lwc1(26, sp_at(c, 0xc0));
        c.lwc1(25, sp_at(c, 0xb8));
        c.lwc1(24, sp_at(c, 0xb0));
        c.lwc1(23, sp_at(c, 0xa8));
        c.lwc1(22, sp_at(c, 0xa0));
        c.lwc1(21, sp_at(c, 0x98));
        c.lwc1(20, sp_at(c, 0x90));
        c.sq(2, sp_at(c, 0x10));
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
    use crate::difftest::{compare, put_vec, seed_count, DATA, GAME_GP};

    const SEEDS: u32 = 120;

    // Everything lives above `fakes::ptr`'s window (`DATA + 0x10_0000`) and the
    // fake results (`DATA + 0x4_0000`), so the fakes can never overwrite it.
    const ME: u32 = DATA + 0x0020_0000;
    const INFO: u32 = DATA + 0x0020_1000;
    const RIG: u32 = DATA + 0x0020_2000;
    const VT: u32 = DATA + 0x0020_3000;
    const BE: u32 = DATA + 0x0020_4000;
    const WORLD: u32 = DATA + 0x0020_5000;
    /// `ScriptList_GetFieldB0`: `v0 = *(a0 + 0xb0)`, used to hand the jump
    /// timer's vtable slot a controlled zero.
    const LOAD_B0: u32 = 0x0027_72b8;

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }
    fn wf(c: &mut Cpu, a: u32, bits: u32) {
        c.bus.write32(a, bits);
    }
    /// `{ short delta; short; void (*fn)(); }` at `vt + off`.
    fn slot(c: &mut Cpu, vt: u32, off: u32, func: u32) {
        c.bus.write16(vt + off, 0);
        c.bus.write32(vt + off + 4, func);
    }
    /// The BELibrary singleton (a zeroed dummy, so nothing reads the disc) and
    /// the world (`gp - 0x848`, with no riders).
    fn globals(c: &mut Cpu) {
        w(c, GAME_GP.wrapping_sub(0x1f04), BE);
        w(c, GAME_GP.wrapping_sub(0x848), WORLD);
        let game = WORLD + 0x1000;
        w(c, WORLD + 0x84, game);
        let race = WORLD + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
    }
    /// The engine's size-class pool allocator: `gp + 0xdd8` ready plus eight
    /// free nodes per size class (the spline's `Alloc` takes its 0x104-byte
    /// point array from the first class).
    fn pool(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w(c, GAME_GP.wrapping_add(0xdd8), 1);
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let pool = DATA + 0x0040_0000 + 0x0004_0000 * i;
            let base = DATA + 0x0080_0000 + 0x0000_4000 * i;
            for n in 0..8u32 {
                let node = base + 0x400 * n;
                w(c, node, if n == 7 { 0 } else { node + 0x400 });
            }
            w(c, pool + head, base);
            w(c, GAME_GP + slot, pool);
        }
    }

    /// The chase-camera state with random transforms, a fake rig vtable and
    /// zeroed spline objects. The timer selects the function's three paths:
    /// 1 -> the active (curve-evaluating) path, 15 -> the rebuild path, else
    /// the early exit.
    fn fixture(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x1635f8, 3);
        globals(c);
        pool(c);
        for i in 0..0x3c0u32 / 4 {
            w(c, ME + 4 * i, 0);
        }
        for i in 0..0x80u32 / 4 {
            w(c, INFO + 4 * i, 0);
        }
        for i in 0..0x400u32 / 4 {
            w(c, RIG + 4 * i, 0);
        }
        for i in 0..0x400u32 / 4 {
            w(c, VT + 4 * i, 0);
        }
        // The rig and its vtable (slots +0x08, +0x60, +0x64, +0x98, +0x9c,
        // +0xa8, +0xac, +0xb8, +0xbc).
        w(c, RIG, VT);
        w(c, RIG + 4, RIG + 0x200); // the rider
        w(c, ME + st::RIG, RIG);
        slot(c, VT, 0x8, FAKE);
        slot(c, VT, 0x60, FAKE);
        slot(c, VT, 0xa8, FAKE);
        slot(c, VT, 0xb8, FAKE);
        if seed % 2 == 0 {
            // `v0 == 0`: resume the landing timer.
            slot(c, VT, 0x98, LOAD_B0);
            w(c, RIG + 0xb0, 0);
        } else {
            slot(c, VT, 0x98, FAKE);
        }
        // Camera position and look-at point (far enough apart that the
        // placement update never resets the camera).
        put_vec(c, ME + st::POS, [0x447a_0000, 0, 0, 0x3f80_0000]);
        put_vec(c, ME + st::VEL, [0, 0, 0, 0x3f80_0000]);
        // Timer and state flags, chosen so every branch is reached.
        w(
            c,
            ME + st::TIMER,
            match seed % 3 {
                0 => 1,  // active path
                1 => 15, // jump spline rebuild
                _ => 3 + seed % 5,
            },
        );
        w(c, ME + st::AIRBORNE, ((seed / 3) % 3 == 0) as u32);
        w(c, INFO + 0x60, ((seed / 9) % 3 == 0) as u32);
        w(c, ME + st::APPLYING, ((seed / 7) % 2) as u32);
        w(c, ME + st::LANDING, ((seed / 11) % 2) as u32);
        w(c, ME + st::JUMP_READY, ((seed / 13) % 2) as u32);
        // The float state.
        for off in [
            0x1f4u32, 0x1f8, 0x1fc, 0x200, 0x20c, 0x210, 0x21c, 0x220, 0x224, 0x228, 0x218,
        ] {
            wf(c, ME + off, r.f32(2.0));
        }
        // The gp-relative `.lit4` constants the blend scales by.
        let mut off = 0x6930u32;
        while off <= 0x6aa0 {
            wf(c, GAME_GP.wrapping_sub(off), r.f32(3.0));
            off += 4;
        }
        // `CheckDistanceReset`'s "too far" limit: never.
        wf(c, GAME_GP.wrapping_sub(0x6778), 0x7f7f_ffff);
        // The four incoming tuning floats.
        c.f[12] = r.f32(1.0);
        c.f[13] = r.f32(1.0);
        c.f[14] = r.f32(1.0);
        c.f[15] = r.f32(1.0);
        c.g[A0][0] = ME as u64;
        c.g[A1][0] = INFO as u64;
    }

    }
