//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements, each with a differential test against the generated body.
//!
//! - **ChaseCameraAlgorithm_ApplyAirSwing** (`0x164878`): one frame of the
//!   chase camera's "air swing" (the lazy pendulum the camera hangs behind the
//!   rider while airborne). It decays/blends the swing offset (`+0x20`) and
//!   direction (`+0x40`) from a per-frame spline, keeps the swing axis
//!   (`+0x120`/`+0x140`/`+0x150`/`+0x160`/`+0x170`/`+0x180`) orthogonal to the
//!   travel direction and finally calls the camera's placement update
//!   (`0x168150`). All of the vector work is VU0 macro code, issued here in
//!   the original's order, so VF/ACC/Q and the VU0 flags end as the original
//!   leaves them; the scalar work uses the FPU op-for-op, so the FPU rounding
//!   and the condition bit match too.
//!
//! The generated body is the authority for registers, cycles and call order;
//! the reference decompilation has only assembly for this function.

use crate::call::{call_ptr, guest, sp, Frame};
use crate::guest::Addr;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[];

// ---------------------------------------------------------------------------
// Helpers and the object layout.

/// `a + off`.
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// Fields of the chase-camera state (`a0`, `ChaseCameraAlgorithm`).
mod st {
    /// `vec4` swing position offset.
    pub const POS: u32 = 0x20;
    /// `vec4` swing direction (velocity) offset.
    pub const VEL: u32 = 0x40;
    /// The rider/model object (its first word is the vtable).
    pub const RIG: u32 = 0x30;
    /// Swing phase accumulator (radians).
    pub const PHASE: u32 = 0x22c;
    /// Swing frequency (phase advanced per second).
    pub const FREQ: u32 = 0x230;
    /// Swing amplitude (0 = no swing, 1 = full).
    pub const AMPLITUDE: u32 = 0x234;
    /// Swing gain, stored for the animation blend.
    pub const GAIN: u32 = 0x238;
    /// Time spent in the current swing (seconds).
    pub const TIME: u32 = 0x23c;
    /// Time at which the swing reached full amplitude.
    pub const IN_TIME: u32 = 0x240;
    /// Time at which the swing starts to decay.
    pub const OUT_TIME: u32 = 0x244;
    /// Non-zero while the camera is airborne (decides the air-swing path).
    pub const AIRBORNE: u32 = 0x2d0;
    /// Non-zero while the air swing is being applied.
    pub const SWINGING: u32 = 0x2d8;
    /// Cubic-spline state object used to shape the swing.
    pub const SPLINE: u32 = 0x334;
    /// `vec4` swing axis (unit).
    pub const AXIS: u32 = 0x110;
    /// `vec4` scaled axis copy.
    pub const AXIS_SCALED: u32 = 0x120;
    /// `vec3` scale (x, y, z).
    pub const SCALE: u32 = 0x130;
    /// `vec4` basis vector A.
    pub const BASIS_A: u32 = 0x140;
    /// `vec4` basis vector B.
    pub const BASIS_B: u32 = 0x150;
    /// `vec4` basis vector C.
    pub const BASIS_C: u32 = 0x160;
    /// `vec4` previous frame's `POS - VEL`.
    pub const PREV_DELTA: u32 = 0x170;
    /// `vec4` previous frame's `POS - POS0`.
    pub const PREV_MOVE: u32 = 0x180;
}

/// `0x0016_8150`: the chase camera's placement update, called at the end.
const CAMERA_UPDATE: u32 = 0x0016_8150;

/// `vec3`/`vec4` dot product of `src` with itself, left in VF4 (x) and Q, as
/// the original's `lqc2; add; mul; adda; madda; madd` sequence does.
#[inline(always)]
fn v_dot(c: &mut Cpu, src: u32) {
    c.lqc2(3, src);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// Dot product of two vectors: VF4 (x) and Q hold `a · b`.
#[inline(always)]
fn v_dot2(c: &mut Cpu, a: u32, b: u32) {
    c.lqc2(3, a);
    c.lqc2(5, b);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// [`v_dot`], then `sqrt`, `waitq`, `cfc2 rt, 22` and `mtc1 rt, fs`: the
/// length of `src` in FPU register `fs`.
#[inline(always)]
fn v_length(c: &mut Cpu, src: u32, rt: usize, fs: usize) {
    v_dot(c, src);
    c.vu0().sqrt(4, 0);
    c.vu0().waitq();
    c.cfc2(rt, 22);
    c.mtc1(rt, fs);
}

/// The original's `opmula/opmsub/sub` cross product: `dst = a × b`, then its
/// `x` lane is zeroed by the `sub(mask 1)`.
#[inline(always)]
fn v_cross(c: &mut Cpu, a: u32, b: u32, dst: u32) {
    c.lqc2(4, a);
    c.lqc2(5, b);
    c.vu0().opmula(4, 5);
    c.vu0().opmsub(6, 5, 4);
    c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
    c.sqc2(6, dst);
}

/// `lq 2, src; sq 2, dst`.
#[inline(always)]
fn qcopy(c: &mut Cpu, src: u32, dst: u32) {
    c.lq(2, src);
    c.sq(2, dst);
}

/// `ChaseCameraAlgorithm_ApplyAirSwing` (`0x164878`).
///
/// `void apply_air_swing(ChaseCameraAlgorithm* this, AirSwingInfo* info, float dt)`
///
/// `info+0x10` is a `vec4` offset, `info+0x20` a `vec4` direction.
pub fn chase_camera_apply_air_swing(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0xf0);
        // "air swing applied this frame" flag.
        Addr(gp(c, -0x1e64)).sw(c, 0);
        frame.save(c, 17, 0xc0);
        frame.save(c, 18, 0xb0);
        c.g[S1][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0xe0);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, 16, 0xd0);
        c.mov_s(20, 12);
        frame.save_ra(c, 0xa0);
        frame.save_fpr(c, 21, 0xe8);
        let me = c.g[S1][0] as u32;
        let info = c.g[S2][0] as u32;

        // The frame's travel direction, kept for the tail.
        c.lqc2(3, p(me, st::POS));
        c.lqc2(4, p(me, st::VEL));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x10));
        c.lq(2, frame.at(c, 0x10));
        c.lq(3, p(me, st::VEL));
        c.g[A0][0] = c.lw(p(me, st::AIRBORNE));
        c.sq(2, frame.at(c, 0));
        let airborne = c.g[A0][0] != 0;
        c.sq(3, frame.at(c, 0x10));
        // Set by the two `break 'b_16531c` paths: they jump past the tail's
        // `cycles += 1` and `v0 = lw(this+0x2d0)`.
        let mut skip_tail = false;
        // The second `break 'b_16531c` (from 0x100485) also jumps past the
        // whole `0x1650a4`/`0x1650f8`/`0x100596` region.
        let mut skip_496 = false;
        // `this->swinging` at entry: its path breaks 'b_164fb0 straight to
        // `0x1650a4`, skipping the `0x100485` block.
        let mut swinging = false;

        if airborne {
            // Airborne: keep the swing, or advance it while it is running.
            c.cycles += 3;
            c.g[V0][0] = c.lw(p(me, st::SWINGING));
            swinging = c.g[V0][0] != 0;
            if swinging {
                c.lwc1(1, p(me, st::AMPLITUDE));
            } else {
                // Not swinging yet: advance the phase and ask the rig for its
                // swing amount (vtable slot +0x9c).
                c.cycles += 12;
                c.lwc1(0, p(me, st::PHASE));
                c.lwc1(1, gp(c, -0x6920));
                c.g[A1][0] = c.lw(p(me, st::RIG));
                c.add_s(0, 0, 1);
                c.swc1(0, p(me, st::PHASE));
                c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x98));
                c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x9c));
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0016_4900)?;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = c.lw(p(me, st::SWINGING));
                } else {
                    // 0x1649a4: normalize the travel direction, project the
                    // rig's lean onto it and clamp the swing amount.
                    c.cycles += 31;
                    c.lqc2(3, p(me, st::POS));
                    c.lqc2(4, p(me, st::VEL));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, frame.at(c, 0x30));
                    c.g[S0][0] = sx(p(me, st::FREQ));
                    c.lq(2, frame.at(c, 0x30));
                    c.g[A1][0] = c.lw(p(me, st::RIG));
                    c.sq(2, p(me, st::AXIS));
                    c.sq(2, frame.at(c, 0x20));
                    v_length(c, p(me, st::AXIS), 3, 21);
                    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                    c.g[A0][0] = c.lh(p(c.g[V0][0] as u32, 0x60));
                    c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x64));
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0016_4968)?;
                    c.lwc1(1, p(me, st::PHASE));
                    c.lwc1(2, gp(c, -0x691c));
                    c.sub_s(0, 0, 1);
                    c.c_le_s(2, 0);
                    if !c.fcc() {
                        c.mov_s(0, 2);
                    } else {
                        // Past the limit: ask the rig for the other limit.
                        c.cycles += 8;
                        c.g[A1][0] = c.lw(p(me, st::RIG));
                        c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                        c.g[A0][0] = c.lh(p(c.g[V0][0] as u32, 0x60));
                        c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x64));
                        let target = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, target, 0x0016_499c)?;
                        c.lwc1(1, p(me, st::PHASE));
                        c.sub_s(0, 0, 1);
                    }

                    // 0x164a88: restart the swing from the lean direction:
                    // cross(travel_dir, one), normalized, dotted with the
                    // rig's forward direction.
                    c.cycles += 50;
                    c.swc1(0, c.g[S0][0] as u32);
                    c.swc1(20, p(me, st::TIME));
                    c.sw(p(me, st::AMPLITUDE), 0);
                    c.g[A1][0] = c.lw(p(me, st::RIG));
                    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                    c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x80));
                    c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x84));
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0016_49c8)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = 0x500000;
                    v_cross(
                        c,
                        p(c.g[A2][0] as u32, 0) - 0xea0,
                        c.g[A0][0] as u32,
                        frame.at(c, 0x30),
                    );
                    c.mtc1(0, 3);
                    qcopy(c, frame.at(c, 0x30), frame.at(c, 0x20));
                    c.g[A1][0] = c.g[A0][0];
                    v_length(c, frame.at(c, 0x20), 3, 0);
                    c.mfc1(2, 0);
                    c.qmtc2(2, 3);
                    c.vu0().div(0, 3, 3, 0);
                    c.lqc2(4, frame.at(c, 0x20));
                    c.vu0().waitq();
                    c.vu0().mul(0b1111, 5, 4, Src::Q);
                    c.sqc2(5, frame.at(c, 0x20));
                    // How far the swing axis has to lean towards the info
                    // direction.
                    v_dot2(c, frame.at(c, 0x20), p(info, 0x20));
                    c.qmfc2(3, 4);
                    c.mtc1(3, 1);
                    c.c_lt_s(1, 3);
                    if c.fcc() {
                        c.cycles += 7;
                        c.g[AT][0] = 0xffff_ffff_bf80_0000;
                        c.mtc1(1, 0);
                        c.mfc1(2, 0);
                        c.lqc2(4, frame.at(c, 0x20));
                        c.qmtc2(2, 3);
                        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                        c.sqc2(5, frame.at(c, 0x20));
                    }

                    // 0x164bb4: build the swing basis (axis scaled by the
                    // swing amount, rotated by 90 degrees, and the travel
                    // direction projected onto it).
                    c.cycles += 74;
                    v_length(c, frame.at(c, 0x20), 2, 0);
                    c.g[AT][0] = 0x40000000;
                    c.mtc1(1, 2);
                    c.div_s(0, 21, 0);
                    c.mfc1(2, 0);
                    c.lqc2(4, frame.at(c, 0x20));
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, frame.at(c, 0x20));
                    let a0 = c.g[A0][0] as u32;
                    c.lqc2(3, a0);
                    c.lqc2(5, a0);
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(2, 4);
                    c.mtc1(2, 0);
                    c.lqc2(3, p(me, st::AXIS));
                    c.lqc2(5, a0);
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(3, 4);
                    c.mtc1(3, 1);
                    c.div_s(1, 1, 0);
                    c.mfc1(3, 1);
                    c.lqc2(4, a0);
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, frame.at(c, 0x60));
                    qcopy(c, frame.at(c, 0x60), frame.at(c, 0x50));
                    c.mfc1(3, 2);
                    c.lqc2(4, frame.at(c, 0x50));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, frame.at(c, 0x60));
                    qcopy(c, frame.at(c, 0x60), frame.at(c, 0x40));
                    c.lqc2(3, p(me, st::AXIS));
                    c.lqc2(4, frame.at(c, 0x40));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, frame.at(c, 0x60));
                    c.lq(2, frame.at(c, 0x60));
                    c.sq(2, p(me, st::AXIS_SCALED));
                    c.sq(2, frame.at(c, 0x30));
                    c.lwc1(0, p(me, st::AXIS_SCALED + 8));
                    c.neg_s(0, 0);
                    c.swc1(0, p(me, st::AXIS_SCALED + 8));
                    c.lqc2(3, p(me, st::AXIS_SCALED));
                    c.lqc2(5, p(me, st::AXIS_SCALED));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(2, 4);
                    c.mtc1(2, 1);
                    c.c_le_s(1, 3);
                    let skip = !c.fcc();
                    c.lq(2, frame.at(c, 0x20));
                    if !skip {
                        c.cycles += 1;
                        c.sq(2, p(me, st::AXIS_SCALED));
                    }

                    // 0x164d98: second-order damping of the swing basis:
                    // blend the axis towards the scaled axis by the elapsed
                    // time, then clamp its length to [0, 200].
                    c.cycles += 80;
                    c.lwc1(0, gp(c, -0x6918));
                    c.mfc1(2, 0);
                    c.lqc2(4, p(me, st::AXIS_SCALED));
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, p(me, st::AXIS_SCALED));
                    c.lwc1(1, gp(c, -0x6914));
                    c.mfc1(2, 1);
                    c.lqc2(4, frame.at(c, 0x20));
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, frame.at(c, 0x40));
                    c.lwc1(2, gp(c, -0x6910));
                    qcopy(c, frame.at(c, 0x40), frame.at(c, 0x30));
                    c.lqc2(3, p(me, st::AXIS_SCALED));
                    c.lqc2(4, frame.at(c, 0x30));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, p(me, st::AXIS_SCALED));
                    v_length(c, p(me, st::AXIS_SCALED), 2, 0);
                    c.div_s(0, 21, 0);
                    c.mfc1(2, 0);
                    c.lqc2(4, p(me, st::AXIS_SCALED));
                    c.qmtc2(2, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, p(me, st::AXIS_SCALED));
                    let a1 = c.g[A1][0] as u32;
                    c.lqc2(3, a1);
                    c.lqc2(5, a1);
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(2, 4);
                    c.mtc1(2, 0);
                    c.lqc2(3, p(me, st::AXIS_SCALED));
                    c.lqc2(5, a1);
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(3, 4);
                    c.mtc1(3, 1);
                    c.div_s(1, 1, 0);
                    c.mfc1(3, 1);
                    c.lqc2(4, a1);
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, frame.at(c, 0x40));
                    qcopy(c, frame.at(c, 0x40), frame.at(c, 0x30));
                    c.sq(2, p(me, st::BASIS_B));
                    c.lqc2(3, p(me, st::AXIS_SCALED));
                    c.lqc2(4, p(me, st::BASIS_B));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, frame.at(c, 0x40));
                    qcopy(c, frame.at(c, 0x40), frame.at(c, 0x30));
                    c.lqc2(3, frame.at(c, 0x30));
                    c.lqc2(5, frame.at(c, 0x30));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(3, 4);
                    c.mtc1(3, 0);
                    c.c_lt_s(0, 2);
                    if c.fcc() {
                        c.cycles += 16;
                        c.swc1(3, frame.at(c, 0x38));
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 0);
                        v_length(c, frame.at(c, 0x30), 2, 2);
                        c.c_lt_s(0, 2);
                        if c.fcc() {
                            c.cycles += 25;
                            c.g[AT][0] = 0x43480000;
                            c.mtc1(1, 0);
                            c.g[AT][0] = 0x42c80000;
                            c.mtc1(1, 1);
                            c.div_s(0, 0, 2);
                            c.mfc1(2, 0);
                            c.lqc2(4, frame.at(c, 0x30));
                            c.qmtc2(2, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, frame.at(c, 0x30));
                            c.lqc2(3, p(me, st::AXIS_SCALED));
                            c.lqc2(4, frame.at(c, 0x30));
                            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                            c.sqc2(5, p(me, st::AXIS_SCALED));
                            c.mfc1(3, 1);
                            c.lqc2(4, p(c.g[A2][0] as u32, 0) - 0xea0);
                            c.qmtc2(3, 3);
                            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                            c.sqc2(5, frame.at(c, 0x50));
                            qcopy(c, frame.at(c, 0x50), frame.at(c, 0x40));
                            c.lqc2(3, p(me, st::AXIS_SCALED));
                            c.lqc2(4, frame.at(c, 0x40));
                            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                            c.sqc2(5, p(me, st::AXIS_SCALED));
                        }
                    }

                    // 0x164ec8: the spline path — scale the axis to the
                    // swing's unit length and, if it is degenerate, fall back
                    // through asin() of the sine of the swing angle.
                    c.cycles += 50;
                    v_cross(
                        c,
                        p(me, st::AXIS),
                        p(me, st::AXIS_SCALED),
                        frame.at(c, 0x50),
                    );
                    c.lwc1(2, gp(c, -0x690c));
                    qcopy(c, frame.at(c, 0x50), p(me, st::SCALE));
                    c.sq(2, frame.at(c, 0x40));
                    v_length(c, p(me, st::SCALE), 3, 0);
                    c.mfc1(2, 0);
                    c.qmtc2(2, 3);
                    c.vu0().div(0, 3, 3, 0);
                    c.lqc2(4, p(me, st::SCALE));
                    c.vu0().waitq();
                    c.vu0().mul(0b1111, 5, 4, Src::Q);
                    c.sqc2(5, p(me, st::SCALE));
                    c.lqc2(3, p(me, st::AXIS));
                    c.lqc2(5, p(me, st::AXIS));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(3, 4);
                    c.mtc1(3, 1);
                    c.lqc2(3, p(me, st::AXIS_SCALED));
                    c.lqc2(5, p(me, st::AXIS_SCALED));
                    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                    c.qmfc2(2, 4);
                    c.mtc1(2, 0);
                    c.mul_s(1, 1, 0);
                    c.sqrt_s(1, 1);
                    c.c_lt_s(1, 2);
                    if c.fcc() {
                        c.cycles += 3;
                        c.mtc1(0, 1);
                    } else {
                        // 0x164eb8: asin of the swing angle, stored as the
                        // swing gain.
                        c.cycles += 16;
                        c.lqc2(3, p(me, st::AXIS));
                        c.lqc2(5, p(me, st::AXIS_SCALED));
                        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                        c.qmfc2(2, 4);
                        c.mtc1(2, 0);
                        c.g[AT][0] = 0xffff_ffff_bf80_0000;
                        c.mtc1(1, 2);
                        c.div_s(12, 0, 1);
                        c.c_le_s(2, 12);
                        if !c.fcc() {
                            c.mov_s(12, 2);
                        } else {
                            c.cycles += 3;
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 0);
                            c.min_s(12, 12, 0);
                        }
                        c.cycles += 4;
                        call_ptr(c, 0x0031_c128, 0x0016_4ec0)?;
                        c.lwc1(1, gp(c, -0x6908));
                        c.sub_s(1, 1, 0);
                    }

                    // 0x164ee8: store the swing gain and push the spline
                    // through its eight keys.
                    c.cycles += 55;
                    c.mtc1(0, 20);
                    c.g[S0][0] = sx(p(me, st::SPLINE));
                    c.swc1(1, p(me, st::GAIN));
                    c.g[A1][0] = 0x9;
                    c.g[A0][0] = c.g[S0][0];
                    c.mov_s(12, 20);
                    c.mov_s(13, 20);
                    call_ptr(c, 0x0031_d738, 0x0016_4ee8)?;
                    c.mov_s(12, 20);
                    c.g[A0][0] = c.g[S0][0];
                    c.mov_s(13, 12);
                    call_ptr(c, 0x0031_d7e0, 0x0016_4ef8)?;
                    c.g[AT][0] = 0x3e000000;
                    c.mtc1(1, 12);
                    c.lwc1(13, gp(c, -0x6904));
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0031_d7e0, 0x0016_4f0c)?;
                    c.g[AT][0] = 0x3e800000;
                    c.mtc1(1, 12);
                    c.lwc1(13, gp(c, -0x6900));
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0031_d7e0, 0x0016_4f20)?;
                    c.g[AT][0] = 0x3ec00000;
                    c.mtc1(1, 12);
                    c.lwc1(13, gp(c, -0x68fc));
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0031_d7e0, 0x0016_4f34)?;
                    c.g[AT][0] = 0x3f000000;
                    c.mtc1(1, 12);
                    c.lwc1(13, gp(c, -0x68f8));
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0031_d7e0, 0x0016_4f48)?;
                    c.g[AT][0] = 0x3f200000;
                    c.mtc1(1, 12);
                    c.lwc1(13, gp(c, -0x68f4));
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0031_d7e0, 0x0016_4f5c)?;
                    c.g[AT][0] = 0x3f400000;
                    c.mtc1(1, 12);
                    c.lwc1(13, gp(c, -0x68f0));
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0031_d7e0, 0x0016_4f70)?;
                    c.g[AT][0] = 0x3f600000;
                    c.mtc1(1, 12);
                    c.lwc1(13, gp(c, -0x68ec));
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0031_d7e0, 0x0016_4f84)?;
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 12);
                    c.g[A0][0] = c.g[S0][0];
                    c.mov_s(13, 12);
                    call_ptr(c, 0x0031_d7e0, 0x0016_4f98)?;
                    c.g[V0][0] = 0x1;
                    c.sw(p(me, st::SWINGING), c.g[V0][0]);
                    c.g[V0][0] = c.lw(p(me, st::SWINGING));
                }
            }

            // 100485: the swing path is done (V0 from the spline calls). The
            // `SWINGING != 0` path broke 'b_164fb0 and skips this block.
            if !swinging {
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    // `break 'b_16531c`: past the tail's charge, the reload and
                    // the whole 0x1650a4 region.
                    c.g[V0][0] = c.lw(p(me, st::AIRBORNE));
                    skip_tail = true;
                    skip_496 = true;
                } else {
                    c.cycles += 1;
                    c.lwc1(1, p(me, st::AMPLITUDE));
                }
            }

            // 100496: ramp the amplitude up to the frequency; once it is
            // there, re-seed the spline and blend the frame.
            if !skip_496 {
                c.cycles += 7;
                c.lwc1(0, gp(c, -0x68e8));
                c.lwc1(2, p(me, st::FREQ));
                c.add_s(1, 1, 0);
                c.c_lt_s(1, 2);
                let ramping = !c.fcc();
                c.swc1(1, p(me, st::AMPLITUDE));
                if !ramping {
                    c.cycles += 54;
                    c.div_s(12, 1, 2);
                    c.g[V1][0] = sx(p(me, st::SPLINE));
                    c.g[V0][0] = 0x1;
                    c.g[A0][0] = c.g[V1][0];
                    c.sw(gp(c, -0x1e64), c.g[V0][0]);
                    c.sw(gp(c, 0x1cf0), c.g[V1][0]);
                    call_ptr(c, 0x0031_dee0, 0x0016_4fe8)?;
                    c.lwc1(12, p(me, st::GAIN));
                    c.mov_s(20, 0);
                    c.g[AT][0] = 0x3f000000;
                    c.mtc1(1, 0);
                    c.g[A0][0] = sx(p(sp(c), 0x90));
                    c.mul_s(12, 20, 12);
                    c.g[A1][0] = sx(p(sp(c), 0x94));
                    c.mul_s(12, 12, 0);
                    call_ptr(c, 0x0031_be50, 0x0016_500c)?;
                    // Scale the swing offset by the per-axis model scale.
                    c.lwc1(1, p(me, st::SCALE));
                    c.lwc1(2, p(me, st::SCALE + 4));
                    c.lwc1(0, frame.at(c, 0x90));
                    c.lwc1(4, p(me, st::SCALE + 8));
                    c.mul_s(1, 0, 1);
                    c.lwc1(3, frame.at(c, 0x94));
                    c.mul_s(2, 0, 2);
                    c.mul_s(0, 0, 4);
                    c.swc1(3, frame.at(c, 0x3c));
                    c.swc1(1, frame.at(c, 0x30));
                    c.swc1(2, frame.at(c, 0x34));
                    c.swc1(0, frame.at(c, 0x38));
                    qcopy(c, frame.at(c, 0x30), frame.at(c, 0x20));
                    // Rotate the offset about the swing axis (a quaternion
                    // product with the axis and its square).
                    c.lqc2(4, frame.at(c, 0x20));
                    c.lqc2(5, p(me, st::AXIS));
                    c.vu0().sub(0b0001, 8, 8, Src::Vec(8));
                    c.vu0().opmula(4, 5);
                    c.vu0().opmsub(6, 5, 4);
                    c.vu0().opmula(4, 6);
                    c.vu0().opmsub(7, 6, 4);
                    c.vu0().mula(0b1110, 5, Src::Bc(0, W));
                    c.vu0().madda(0b1110, 6, Src::Bc(4, W));
                    c.vu0().madda(0b1110, 6, Src::Bc(4, W));
                    c.vu0().madda(0b1110, 7, Src::Bc(0, W));
                    c.vu0().madd(0b1110, 8, 7, Src::Bc(0, W));
                    c.sqc2(8, frame.at(c, 0x40));
                    qcopy(c, frame.at(c, 0x40), p(me, st::BASIS_A));
                    c.sq(2, frame.at(c, 0x30));
                    c.mfc1(3, 20);
                    c.lqc2(4, p(me, st::BASIS_B));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, frame.at(c, 0x40));
                    qcopy(c, frame.at(c, 0x40), p(me, st::BASIS_C));
                    c.sq(2, frame.at(c, 0x30));
                }

                // 100569: smooth-step the amplitude and blend the frame towards
                // the info position/direction.
                c.cycles += 21;
                c.lwc1(1, gp(c, -0x68e4));
                c.lwc1(0, p(me, st::FREQ));
                c.lwc1(2, p(me, st::AMPLITUDE));
                c.mul_s(0, 0, 1);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 4);
                c.mtc1(0, 1);
                c.g[AT][0] = 0x40400000;
                c.mtc1(1, 3);
                c.div_s(2, 2, 0);
                c.min_s(2, 2, 4);
                c.max_s(2, 2, 1);
                c.mul_s(0, 2, 2);
                c.add_s(1, 0, 0);
                c.mul_s(0, 0, 3);
                c.mul_s(1, 1, 2);
                c.sub_s(0, 0, 1);
                c.c_lt_s(4, 0);
                if c.fcc() {
                    c.mov_s(0, 4);
                }

                // 100596: frame = frame * (1 - t) + info * t, then rebuild the
                // travel direction and the deltas.
                c.cycles += 37;
                c.mov_s(2, 0);
                c.sub_s(0, 4, 2);
                c.mfc1(3, 0);
                c.lqc2(4, p(me, st::POS));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x40));
                c.lq(2, frame.at(c, 0x40));
                c.sq(2, frame.at(c, 0x30));
                c.mfc1(3, 2);
                c.lqc2(4, p(info, 0x10));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x50));
                c.lq(2, frame.at(c, 0x50));
                c.sq(2, frame.at(c, 0x40));
                c.lqc2(3, frame.at(c, 0x30));
                c.lqc2(4, frame.at(c, 0x40));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x50));
                c.lq(2, frame.at(c, 0x50));
                c.sq(2, p(me, st::POS));
                c.sq(2, frame.at(c, 0x20));
                c.lqc2(3, p(me, st::POS));
                c.lqc2(4, p(me, st::BASIS_A));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x40));
                c.lq(2, frame.at(c, 0x40));
                c.sq(2, frame.at(c, 0x30));
                c.lqc2(3, frame.at(c, 0x30));
                c.lqc2(4, p(me, st::BASIS_C));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x40));
                c.lq(2, frame.at(c, 0x40));
                c.sq(2, p(me, st::VEL));
                c.sq(2, frame.at(c, 0x20));
            }
        } // if airborne

        // 100641: not airborne — run the swing out (or in) on the elapsed
        // time and blend the frame with the spline keys.
        if !airborne {
            c.cycles += 6;
            c.lwc1(3, p(me, st::TIME));
            c.mtc1(0, 1);
            c.c_lt_s(1, 3);
            if c.fcc() {
                c.cycles += 18;
                c.lwc1(0, gp(c, -0x68e0));
                c.g[AT][0] = 0x40400000;
                c.mtc1(1, 2);
                c.sub_s(0, 3, 0);
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 3);
                c.max_s(0, 0, 1);
                c.div_s(12, 0, 20);
                c.swc1(0, p(me, st::TIME));
                c.mul_s(1, 12, 12);
                c.add_s(0, 1, 1);
                c.mul_s(1, 1, 2);
                c.mul_s(0, 0, 12);
                c.sub_s(0, 1, 0);
                c.c_lt_s(3, 0);
                if c.fcc() {
                    c.mov_s(0, 3);
                }

                // 100673: blend towards the stored swing offset.
                c.cycles += 75;
                c.lq(2, p(me, st::POS));
                c.mov_s(12, 0);
                c.sub_s(0, 3, 12);
                c.sq(2, frame.at(c, 0x20));
                c.mfc1(3, 0);
                c.lqc2(4, p(me, st::POS));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x50));
                c.lq(2, frame.at(c, 0x50));
                c.sq(2, frame.at(c, 0x40));
                c.mfc1(3, 12);
                c.lqc2(4, p(info, 0x10));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x60));
                c.lq(2, frame.at(c, 0x60));
                c.sq(2, frame.at(c, 0x50));
                c.lqc2(3, frame.at(c, 0x40));
                c.lqc2(4, frame.at(c, 0x50));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x60));
                c.lq(2, frame.at(c, 0x60));
                c.sq(2, p(me, st::POS));
                c.sq(2, frame.at(c, 0x30));
                c.mfc1(3, 0);
                c.lqc2(4, p(me, st::VEL));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x50));
                c.lq(2, frame.at(c, 0x50));
                c.sq(2, frame.at(c, 0x40));
                c.lqc2(3, p(info, 0x10));
                c.lqc2(4, p(me, st::BASIS_A));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x80));
                c.lq(2, frame.at(c, 0x80));
                c.sq(2, frame.at(c, 0x70));
                c.lqc2(3, frame.at(c, 0x70));
                c.lqc2(4, p(me, st::BASIS_C));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x80));
                c.lq(2, frame.at(c, 0x80));
                c.sq(2, frame.at(c, 0x60));
                c.mfc1(3, 12);
                c.lqc2(4, frame.at(c, 0x60));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x80));
                c.lq(2, frame.at(c, 0x80));
                c.sq(2, frame.at(c, 0x50));
                c.lqc2(3, frame.at(c, 0x40));
                c.lqc2(4, frame.at(c, 0x50));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x80));
                c.lq(2, frame.at(c, 0x80));
                c.sq(2, p(me, st::VEL));
                c.sq(2, frame.at(c, 0x30));
                c.lqc2(3, p(me, st::POS));
                c.lqc2(4, p(me, st::VEL));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x40));
                c.lq(2, frame.at(c, 0x40));
                c.sq(2, p(me, st::PREV_DELTA));
                c.sq(2, frame.at(c, 0x30));
                c.lqc2(3, p(me, st::POS));
                c.lqc2(4, frame.at(c, 0x20));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x40));
                c.lq(2, frame.at(c, 0x40));
                c.lwc1(0, p(me, st::TIME));
                c.sq(2, p(me, st::PREV_MOVE));
                c.sq(2, frame.at(c, 0x30));
                c.swc1(0, p(me, st::IN_TIME));
                c.swc1(0, p(me, st::OUT_TIME));
            } else {
                // `break 'b_16531c`: past the tail's charge and reload.
                c.g[V0][0] = c.lw(p(me, st::AIRBORNE));
                skip_tail = true;
            }
        }

        // 100752: the tail shared by every path (the `break 'b_16531c` paths
        // skip the charge and the V0 reload).
        if !skip_tail {
            c.cycles += 1;
            c.g[V0][0] = c.lw(p(me, st::AIRBORNE));
        }

        // 0x165448: while the swing is inside its in/out window, settle the
        // frame back onto the stored offsets.
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            c.lwc1(4, p(me, st::IN_TIME));
            c.mtc1(0, 3);
            c.c_lt_s(3, 4);
            if c.fcc() {
                c.cycles += 4;
                c.lwc1(1, p(me, st::OUT_TIME));
                c.c_lt_s(3, 1);
                if c.fcc() {
                    c.cycles += 18;
                    c.lwc1(0, gp(c, -0x68dc));
                    c.g[AT][0] = 0x40400000;
                    c.mtc1(1, 2);
                    c.sub_s(0, 4, 0);
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 4);
                    c.max_s(0, 0, 3);
                    c.div_s(3, 0, 1);
                    c.swc1(0, p(me, st::IN_TIME));
                    c.mul_s(1, 3, 3);
                    c.add_s(0, 1, 1);
                    c.mul_s(1, 1, 2);
                    c.mul_s(0, 0, 3);
                    c.sub_s(0, 1, 0);
                    c.c_lt_s(4, 0);
                    if c.fcc() {
                        c.mov_s(0, 4);
                    }

                    // 100804: settle towards the stored deltas.
                    c.cycles += 46;
                    c.lqc2(3, p(me, st::POS));
                    c.lqc2(4, p(me, st::VEL));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, frame.at(c, 0x30));
                    c.mov_s(3, 0);
                    c.lq(2, frame.at(c, 0x30));
                    c.sub_s(0, 4, 3);
                    c.sq(2, frame.at(c, 0x20));
                    c.mfc1(3, 3);
                    c.lqc2(4, p(me, st::PREV_DELTA));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, frame.at(c, 0x50));
                    c.lq(2, frame.at(c, 0x50));
                    c.sq(2, frame.at(c, 0x40));
                    c.mfc1(3, 0);
                    c.lqc2(4, frame.at(c, 0x20));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, frame.at(c, 0x60));
                    c.lq(2, frame.at(c, 0x60));
                    c.sq(2, frame.at(c, 0x50));
                    c.lqc2(3, frame.at(c, 0x40));
                    c.lqc2(4, frame.at(c, 0x50));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, frame.at(c, 0x60));
                    c.lq(2, frame.at(c, 0x60));
                    c.sq(2, frame.at(c, 0x30));
                    c.mfc1(3, 3);
                    c.lqc2(4, p(me, st::PREV_MOVE));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, frame.at(c, 0x50));
                    c.lq(2, frame.at(c, 0x50));
                    c.sq(2, frame.at(c, 0x40));
                    c.lqc2(3, p(me, st::POS));
                    c.lqc2(4, frame.at(c, 0x40));
                    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, p(me, st::POS));
                    c.lqc2(3, p(me, st::POS));
                    c.lqc2(4, frame.at(c, 0x30));
                    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                    c.sqc2(5, frame.at(c, 0x50));
                    c.lq(2, frame.at(c, 0x50));
                    c.sq(2, p(me, st::VEL));
                    c.sq(2, frame.at(c, 0x40));
                }
            }
        }

        // 0x165518: damp the velocity towards `POS - POS0` so the camera
        // does not overshoot when the swing unwinds.
        c.cycles += 31;
        c.lqc2(3, p(me, st::POS));
        c.lqc2(4, p(me, st::VEL));
        c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, frame.at(c, 0x30));
        c.lwc1(1, gp(c, -0x68d8));
        qcopy(c, frame.at(c, 0x30), frame.at(c, 0x20));
        v_length(c, frame.at(c, 0), 2, 0);
        v_length(c, frame.at(c, 0x20), 3, 2);
        c.mul_s(1, 0, 1);
        c.c_lt_s(2, 1);
        if c.fcc() {
            c.cycles += 6;
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 0);
            c.c_lt_s(0, 2);
            let skip = !c.fcc();
            c.lq(2, frame.at(c, 0x10));
            if skip {
                c.cycles += 1;
                c.sq(2, p(me, st::VEL));
            } else {
                c.cycles += 14;
                c.div_s(0, 1, 2);
                c.mfc1(2, 0);
                c.lqc2(4, frame.at(c, 0x20));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, frame.at(c, 0x20));
                c.lqc2(3, p(me, st::POS));
                c.lqc2(4, frame.at(c, 0x20));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, frame.at(c, 0x40));
                c.lq(2, frame.at(c, 0x40));
                c.sq(2, p(me, st::VEL));
                c.sq(2, frame.at(c, 0x30));
            }
        }

        // 100929: let the camera place itself, then return.
        c.cycles += 10;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, CAMERA_UPDATE, 0x0016_5520)?;
        frame.restore(c, 16, 0xd0);
        frame.restore(c, 17, 0xc0);
        frame.restore(c, 18, 0xb0);
        frame.restore_ra(c, 0xa0);
        frame.restore_fpr(c, 21, 0xe8);
        frame.restore_fpr(c, 20, 0xe0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, vtable};
    use crate::difftest::{put_vec, Rng, DATA, GAME_GP};

    // The chase-camera state, its info block and the rider/model object (whose
    // first word is the vtable) live high enough that `fakes::ptr` (below
    // `DATA + 0x10_0000`) and the fake results can never overwrite them.
    const ME: u32 = DATA + 0x0020_0000;
    const INFO: u32 = DATA + 0x0020_1000;
    const RIG: u32 = DATA + 0x0020_2000;
    const BE: u32 = DATA + 0x0020_3000;
    const WORLD: u32 = DATA + 0x0020_4000;

    fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    fn wf(c: &mut Cpu, a: u32, bits: u32) {
        c.bus.write32(a, bits);
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
    /// free nodes per size class (the spline's `Alloc` takes a 160-byte block
    /// from the 128-byte class, `gp + 0x2a84`).
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

    /// The chase-camera state with random transforms, a fake rig vtable and a
    /// zeroed spline object.
    fn fixture(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = machine(c, seed, salt, 3);
        globals(c);
        pool(c);
        for i in 0..0x360u32 / 4 {
            w(c, ME + 4 * i, 0);
        }
        for i in 0..0x40u32 / 4 {
            w(c, INFO + 4 * i, 0);
        }
        // Rig + vtable: the slots the algorithm queries (+0x60, +0x80, +0x98).
        let vt = vtable(c, &mut r, &[0x60, 0x80, 0x98]);
        w(c, RIG, vt);
        w(c, ME + super::st::RIG, RIG);
        // Random transforms.
        put_vec(
            c,
            ME + super::st::POS,
            [r.f32(1e4), r.f32(1e4), r.f32(1e4), r.f32(1.0)],
        );
        put_vec(
            c,
            ME + super::st::VEL,
            [r.f32(1e3), r.f32(1e3), r.f32(1e3), r.f32(1.0)],
        );
        put_vec(
            c,
            ME + super::st::AXIS,
            [r.f32(2.0), r.f32(2.0), r.f32(2.0), r.f32(0.0)],
        );
        put_vec(
            c,
            ME + super::st::AXIS_SCALED,
            [r.f32(2.0), r.f32(2.0), r.f32(2.0), r.f32(0.0)],
        );
        put_vec(
            c,
            ME + super::st::BASIS_A,
            [r.f32(2.0), r.f32(2.0), r.f32(2.0), r.f32(0.0)],
        );
        put_vec(
            c,
            ME + super::st::BASIS_B,
            [r.f32(2.0), r.f32(2.0), r.f32(2.0), r.f32(0.0)],
        );
        put_vec(
            c,
            ME + super::st::BASIS_C,
            [r.f32(2.0), r.f32(2.0), r.f32(2.0), r.f32(0.0)],
        );
        put_vec(
            c,
            ME + super::st::PREV_DELTA,
            [r.f32(2.0), r.f32(2.0), r.f32(2.0), r.f32(0.0)],
        );
        put_vec(
            c,
            ME + super::st::PREV_MOVE,
            [r.f32(2.0), r.f32(2.0), r.f32(2.0), r.f32(0.0)],
        );
        wf(c, ME + super::st::SCALE, r.edge_f32());
        wf(c, ME + super::st::SCALE + 4, r.edge_f32());
        wf(c, ME + super::st::SCALE + 8, r.edge_f32());
        put_vec(
            c,
            INFO + 0x10,
            [r.f32(1.0), r.f32(1.0), r.f32(1.0), r.f32(0.0)],
        );
        put_vec(
            c,
            INFO + 0x20,
            [r.f32(1.0), r.f32(1.0), r.f32(1.0), r.f32(0.0)],
        );
        // Swing state: cover airborne / not, swinging / not, and both clamps.
        w(c, ME + super::st::AIRBORNE, (seed % 3 != 0) as u32);
        w(c, ME + super::st::SWINGING, (seed % 4 == 0) as u32);
        wf(c, ME + super::st::PHASE, r.f32(3.0));
        wf(
            c,
            ME + super::st::FREQ,
            if seed % 5 == 0 {
                0.0f32.to_bits()
            } else {
                r.f32(2.0)
            },
        );
        wf(
            c,
            ME + super::st::AMPLITUDE,
            if seed % 4 == 0 {
                0.0f32.to_bits()
            } else {
                r.f32(1.0)
            },
        );
        wf(c, ME + super::st::GAIN, r.f32(1.0));
        wf(
            c,
            ME + super::st::TIME,
            if seed % 5 == 1 {
                0.0f32.to_bits()
            } else {
                r.f32(2.0)
            },
        );
        wf(
            c,
            ME + super::st::IN_TIME,
            if seed % 6 == 0 {
                0.0f32.to_bits()
            } else {
                r.f32(2.0)
            },
        );
        wf(
            c,
            ME + super::st::OUT_TIME,
            if seed % 6 == 3 {
                0.0f32.to_bits()
            } else {
                r.f32(-2.0)
            },
        );
        // The gp-relative `.lit4` constants the code scales by.
        wf(c, GAME_GP.wrapping_sub(0x6920), 0.5f32.to_bits());
        wf(c, GAME_GP.wrapping_sub(0x691c), 1.0f32.to_bits());
        wf(c, GAME_GP.wrapping_sub(0x6918), 1.0f32.to_bits());
        wf(c, GAME_GP.wrapping_sub(0x6914), 1.0f32.to_bits());
        wf(c, GAME_GP.wrapping_sub(0x6910), 1.0f32.to_bits());
        wf(c, GAME_GP.wrapping_sub(0x690c), 1.0f32.to_bits());
        wf(c, GAME_GP.wrapping_sub(0x6908), 1.5707964f32.to_bits());
        for off in [0x6904u32, 0x6900, 0x68fc, 0x68f8, 0x68f4, 0x68f0, 0x68ec] {
            wf(c, GAME_GP.wrapping_sub(off), r.edge_f32());
        }
        wf(c, GAME_GP.wrapping_sub(0x68e8), 0.5f32.to_bits());
        wf(c, GAME_GP.wrapping_sub(0x68e4), 1.0f32.to_bits());
        wf(c, GAME_GP.wrapping_sub(0x68e0), 0.5f32.to_bits());
        wf(c, GAME_GP.wrapping_sub(0x68dc), 0.5f32.to_bits());
        wf(c, GAME_GP.wrapping_sub(0x68d8), 0.5f32.to_bits());
        // `float dt`.
        c.f[12] = (((r.next_u32() % 500) + 1) as f32 / 1000.0).to_bits();
        c.g[A0][0] = ME as u64;
        c.g[A1][0] = INFO as u64;
        r
    }

    
}
