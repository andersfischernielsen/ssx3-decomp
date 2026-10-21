//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements, each with a differential test against the generated body.
//!
//! - **Weather save state** (`tWPIGD_Weather_SaveState`): forwards nineteen
//!   4-byte fields (and their owners) to the object's virtual `save` slot.
//! - **Steering** (`ComputerActorRider_ComputeSteer`): the angle between the
//!   board's velocity and the path direction, saturated and signed by the
//!   turn direction.
//! - **Spray emitters** (`cDynamicSprayFX_UpdateEmitter3/4`): particle spawn
//!   timers, spawn volume, weather-driven colour and emit call.
//! - **Timed event animation** (`Rider_UpdateTimedEventAnim`): picks and plays
//!   a landing animation when the rider's event counter changes.
//!
//! Calls follow [`crate::call`]; VU0 code issues the original's COP2 macro
//! operations in the same order (so VF/ACC/Q and the VU0 flags end as the
//! original leaves them); FPU arithmetic goes through the same `c.*_s`
//! operations the original uses.

use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002b_feb0, weather_save_state, "tWPIGD_Weather_SaveState"),
    (
        0x0010_0348,
        computer_actor_rider_compute_steer,
        "ComputerActorRider_ComputeSteer",
    ),
    (
        0x0011_5b58,
        rider_update_timed_event_anim,
        "Rider_UpdateTimedEventAnim",
    ),
    (
        0x002d_fe88,
        dynamic_spray_update_emitter3,
        "cDynamicSprayFX_UpdateEmitter3",
    ),
    (
        0x002e_1120,
        dynamic_spray_update_emitter4,
        "cDynamicSprayFX_UpdateEmitter4",
    ),
];

// ---------------------------------------------------------------------------
// Register and memory helpers.

/// `a + off`.
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// `lw` (sign-extended).
#[inline(always)]
fn lw(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lw(c)
}

/// `lh` (sign-extended).
#[inline(always)]
fn lh(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lh(c)
}

/// The low 32 bits of a GPR.
#[inline(always)]
fn r32(c: &Cpu, r: usize) -> u32 {
    c.g[r][0] as u32
}

/// `sw`.
#[inline(always)]
fn sw(c: &mut Cpu, a: u32, v: u64) {
    Addr(a).sw(c, v);
}

/// `sw (reg + off), v`.
#[inline(always)]
fn sw_at(c: &mut Cpu, base: usize, off: u32, v: u64) {
    let a = p(r32(c, base), off);
    Addr(a).sw(c, v);
}

/// `sw (gp + off), v`.
#[inline(always)]
fn sw_gp(c: &mut Cpu, off: i32, v: u64) {
    let a = gp(c, off);
    Addr(a).sw(c, v);
}

// ---------------------------------------------------------------------------
// Weather.

/// `void tWPIGD_Weather_SaveState(void* self /*a0*/, void* owner /*a1*/)`
/// (0x002bfeb0): for each of nineteen 4-byte fields, calls a virtual method of
/// `owner` (vtable slot `+0xc`, `this = owner + lh(vt+8)`) passing the field
/// address from `self` and a 4-byte size. The first source field is `self+8`;
/// the rest come in pairs: `self+8+8k` then `self+0`.
pub fn weather_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 277;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 4;
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        for i in 0..38u32 {
            // The vtable is re-read from the owner before every call.
            c.g[V1][0] = lw(c, r32(c, S0));
            let src = if i % 2 == 0 {
                sx(p(r32(c, S1), 8 + 8 * (i / 2)))
            } else {
                c.g[S1][0]
            };
            c.g[A2][0] = 4;
            let delta = lh(c, p(r32(c, V1), 8));
            c.g[V0][0] = lw(c, p(r32(c, V1), 0xc));
            let target = r32(c, V0);
            c.g[A1][0] = src;
            // The virtual method runs on the *owner* (a1), adjusted by the
            // vtable's this-delta; the fields come from `self` (a0).
            c.g[A0][0] = sx(p(r32(c, S0), delta as u32));
            call_ptr(c, target, 0x002b_fee4 + 0x1c * i)?;
        }
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Steering.

/// Rider fields used by the steering computation.
mod rider {
    /// `Rider+0x18`: the rider's model / animation object.
    pub const MODEL: u32 = 0x18;
    /// `RiderModel+0x6c0`: the motion-controller object (has a vtable).
    pub const MOTION: u32 = 0x6c0;
}

/// `gp + off` (the `.lit4` constants).
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `sp + off`.
#[inline(always)]
fn at(c: &Cpu, off: u32) -> u32 {
    p(sp(c), off)
}

/// A `gp`-relative `.lit4` float moved into `fN` (`lwc1`).
#[inline(always)]
fn ldf(c: &mut Cpu, n: usize, off: i32) {
    let a = gp(c, off);
    c.lwc1(n, a);
}

/// The dot product the original computes with the VU0 add/madd chain
/// (vf3 = a, vf5 = b), left in vf4.x.
#[inline(always)]
fn v_dot(c: &mut Cpu, a: u32, b: u32) {
    c.lqc2(3, a);
    c.lqc2(5, b);
    let vu = c.vu0();
    vu.add(0b1000, 6, 0, Src::Bc(0, W));
    vu.mul(0b1111, 4, 3, Src::Vec(5));
    vu.adda(0b1000, 4, Src::Bc(4, Y));
    vu.madda(0b1000, 6, Src::Bc(4, Z));
    vu.madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// [`v_dot`] of a vector with itself.
#[inline(always)]
fn v_dot_self(c: &mut Cpu, a: u32) {
    c.lqc2(3, a);
    let vu = c.vu0();
    vu.add(0b1000, 6, 0, Src::Bc(0, W));
    vu.mul(0b1111, 4, 3, Src::Vec(3));
    vu.adda(0b1000, 4, Src::Bc(4, Y));
    vu.madda(0b1000, 6, Src::Bc(4, Z));
    vu.madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// `out = a / |a|` (the `vrsqrt` + `vmul Q` sequence).
#[inline(always)]
fn v_normalize(c: &mut Cpu, a: u32, out: u32) {
    v_dot_self(c, a);
    c.vu0().rsqrt(0, 3, 4, 0);
    c.vu0().waitq();
    c.vu0().mul(0b1111, 5, 3, Src::Q);
    c.sqc2(5, out);
}

/// `float ComputerActorRider_ComputeSteer(Rider* rider /*a0*/,
/// const Vec4* velocity /*a1*/)` (0x00100348): the steering angle between the
/// part of `velocity` perpendicular to the forward axis and a reference axis
/// of the motion controller, normalised and clamped to `0.98` (scaled by 2π)
/// with its sign kept.
///
/// The two indirect calls are virtual methods of the object at
/// `rider->model+0x6c0` (slots `+0x10` and `+0x28`), returning the forward
/// vector and the motion basis vector.
pub fn computer_actor_rider_compute_steer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 22, 0xc0);
        frame.save_ra(c, 0x80);
        frame.save_fpr(c, 21, 0xb8);
        frame.save_fpr(c, 20, 0xb0);
        // f22 = 0.
        c.mtc1(0, 22);
        c.g[V0][0] = lw(c, p(r32(c, S0), rider::MODEL));
        let model = r32(c, V0);
        c.lq(3, p(model, 0x1c0));
        c.sq(3, sp(c));
        c.lwc1(0, p(sp(c), 8));
        c.c_le_s(0, 22);
        let backward = !c.fcc();
        c.g[S1][0] = c.g[A1][0];
        if !backward {
            // Not driving backwards: use the motion's forward axis instead.
            c.cycles += 4;
            c.g[V0][0] = 0x0050_0000;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xea0));
            c.lq(3, r32(c, V0));
            c.sq(3, sp(c));
        }

        'steer: {
            'math: {
                c.cycles += 100;
                // Forward vector: virtual slot +0x10 of the motion object.
                c.g[A0][0] = lw(c, p(r32(c, S0), rider::MODEL));
                c.g[V1][0] = lw(c, p(r32(c, A0), rider::MOTION));
                c.g[A0][0] = sx(p(r32(c, A0), rider::MOTION));
                c.g[A1][0] = lh(c, p(r32(c, V1), 0x10));
                c.g[V0][0] = lw(c, p(r32(c, V1), 0x14));
                let target = r32(c, V0);
                c.g[A0][0] = sx(p(r32(c, A0), r32(c, A1)));
                call_ptr(c, target, 0x0010_03b8)?;
                c.lq(3, r32(c, V0));
                c.sq(3, at(c, 0x10));
                // Side axis = forward × up (vf4 = forward, vf5 = up).
                c.lqc2(4, at(c, 0x10));
                c.lqc2(5, at(c, 0));
                c.vu0().opmula(4, 5);
                c.vu0().opmsub(6, 5, 4);
                c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                c.sqc2(6, at(c, 0x30));
                c.lq(2, at(c, 0x30));
                c.sq(2, at(c, 0x20));
                // Motion basis vector: virtual slot +0x28.
                c.g[A0][0] = lw(c, p(r32(c, S0), rider::MODEL));
                c.g[A1][0] = sx(p(r32(c, A0), rider::MOTION));
                c.g[V1][0] = lw(c, p(r32(c, A0), rider::MOTION));
                c.g[A0][0] = lh(c, p(r32(c, V1), 0x28));
                c.g[V0][0] = lw(c, p(r32(c, V1), 0x2c));
                let target = r32(c, V0);
                c.g[A0][0] = sx(p(r32(c, A1), r32(c, A0)));
                call_ptr(c, target, 0x0010_03fc)?;
                // The velocity's perpendicular component.
                c.lqc2(3, r32(c, S1));
                c.lqc2(4, r32(c, V0));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, 0x40));
                c.lq(2, at(c, 0x40));
                c.sq(2, at(c, 0x30));
                c.swc1(22, at(c, 0x38));
                v_normalize(c, at(c, 0x30), at(c, 0x50));
                c.lq(2, at(c, 0x50));
                c.sq(2, at(c, 0x30));
                c.sq(2, at(c, 0x40));
                // Remove the forward component.
                v_dot(c, at(c, 0x30), at(c, 0));
                c.qmfc2(3, 4);
                c.mtc1(3, 0);
                c.mfc1(4, 0);
                c.lqc2(4, at(c, 0));
                c.qmtc2(4, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, 0x70));
                c.lq(2, at(c, 0x70));
                c.sq(2, at(c, 0x60));
                c.lqc2(3, at(c, 0x30));
                c.lqc2(4, at(c, 0x60));
                c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, 0x70));
                c.lq(2, at(c, 0x70));
                c.sq(2, at(c, 0x50));
                v_normalize(c, at(c, 0x50), at(c, 0x70));
                c.lq(2, at(c, 0x70));
                c.sq(2, at(c, 0x40));
                c.sq(2, at(c, 0x30));
                // f20 = side · forward, f21 = side · basis.
                v_dot(c, at(c, 0x30), at(c, 0x10));
                c.qmfc2(3, 4);
                c.mtc1(3, 20);
                v_dot(c, at(c, 0x30), at(c, 0x20));
                c.qmfc2(2, 4);
                c.mtc1(2, 21);
                c.c_eq_s(20, 22);
                if !c.fcc() {
                    // f20 != 0: fall through to the atan path.
                } else {
                    // Degenerate: 0 or ±π/2.
                    c.cycles += 4;
                    c.c_eq_s(21, 22);
                    let axis_aligned = c.fcc();
                    c.mov_s(12, 21);
                    if axis_aligned {
                        break 'steer;
                    }
                    c.cycles += 4;
                    c.c_le_s(22, 21);
                    ldf(c, 12, -0x7fec);
                    if !c.fcc() {
                        break 'steer;
                    }
                    c.cycles += 3;
                    ldf(c, 12, -0x7fe8);
                    break 'steer;
                }
            }
            c.cycles += 7;
            c.div_s(12, 21, 20);
            call_ptr(c, 0x0031_c228, 0x0010_0564)?; // Math_Atan
            c.c_lt_s(20, 22);
            c.mov_s(12, 0);
            if !c.fcc() {
                break 'steer;
            }
            // f20 < 0: add π/2, or subtract π on the other side.
            'half_turn: {
                c.cycles += 3;
                c.c_lt_s(22, 21);
                if !c.fcc() {
                } else {
                    c.cycles += 3;
                    ldf(c, 0, -0x7fe4);
                    c.add_s(12, 12, 0);
                    break 'steer;
                }
            }
            c.cycles += 2;
            ldf(c, 0, -0x7fe0);
            c.sub_s(12, 12, 0);
            break 'steer;
        }

        // Scale to the 0..0.98 steering range and restore its sign.
        'clamp: {
            'scale: {
                c.cycles += 8;
                ldf(c, 0, -0x7fdc);
                c.abs_s(1, 12);
                ldf(c, 2, -0x7fd8);
                c.mul_s(1, 1, 0);
                c.c_lt_s(1, 2);
                c.lq(16, at(c, 0xa0));
                if c.fcc() {
                    // Below the dead zone: no clamp needed.
                    c.cycles += 3;
                    c.mtc1(0, 1);
                    c.mov_s(0, 1);
                    break 'clamp;
                }
            }
            'upper: {
                c.cycles += 5;
                ldf(c, 0, -0x7fd4);
                c.c_lt_s(0, 1);
                if c.fcc() {
                    c.mov_s(1, 0);
                }
            }
            'sign: {
                c.cycles += 5;
                c.mtc1(0, 0);
                c.c_lt_s(12, 0);
                if c.fcc() {
                    c.neg_s(1, 1);
                }
            }
            c.cycles += 2;
            c.lq(16, at(c, 0xa0));
            c.mov_s(0, 1);
        }

        c.cycles += 7;
        frame.restore(c, 17, 0x90);
        frame.restore_ra(c, 0x80);
        frame.restore_fpr(c, 22, 0xc0);
        frame.restore_fpr(c, 21, 0xb8);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Timed event animations.

mod rider_anim {
    /// Event counter (0: none) and the frame it last changed.
    pub const EVENT: u32 = 0x358;
    pub const LAST_FRAME: u32 = 0x354;
    /// Animation controller (has a vtable at `+0x58`).
    pub const ANIM: u32 = 0x784;
    /// A doubleword handed to the animation as its blend parameter.
    pub const BLEND_SOURCE: u32 = 0x8c8;
    /// The current sub-state's blend value.
    pub const SUB_BLEND: u32 = 0x1fc;
    /// `Rider+0x77c -> +0xde4`: the rider's sub-state (non-zero: busy).
    pub const STATE: u32 = 0x77c;
}

mod anim_callee {
    use super::Callee;
    use ssx3_ee as ee;

    pub const GET_LAYER_ENTRY: Callee = Callee { addr: 0x0031_1ae8, f: ee::s10::AnimController_GetLayerAnimEntry };
    pub const PLAY_ANIM: Callee = Callee { addr: 0x0031_28e8, f: ee::s10::AnimController_PlayAnim };
    pub const AUDIO_GET_INSTANCE: Callee = Callee { addr: 0x0028_b180, f: ee::s07::SSXAudioSystem_GetInstance };
    pub const AUDIO_RIDER_SAY: Callee = Callee { addr: 0x0029_ff80, f: ee::s08::SSXAudioSystem_RiderSay2082 };
    pub const FRAME_COUNTER: Callee = Callee { addr: 0x0012_98c8, f: ee::s00::AI_GetFrameCounter };
    pub const GET_SUB_STATE: Callee = Callee { addr: 0x0011_fee8, f: ee::s00::Rider_GetSubState };
    /// The animation-type jump table the original indexes by `event - 1`.
    pub const EVENT_TABLE: u32 = 0x0045_6cc0;
}

/// `ld` (a 64-bit load).
#[inline(always)]
fn ld(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).ld(c)
}

/// The common epilogue: `s0` back, `ra` back, `sp += 0x20`, return.
fn timed_event_epilogue(c: &mut Cpu, frame: Frame) {
    frame.restore(c, S0, 0x10);
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// The tail at 0x115d38: clear the event, remember the frame it ended on and
/// return (the original repeats the epilogue's 3 cycles here).
fn timed_event_tail(c: &mut Cpu, frame: Frame) {
    c.cycles += 3;
    let last = c.g[V0][0];
    sw_at(c, S0, rider_anim::EVENT, 0);
    sw_at(c, S0, rider_anim::LAST_FRAME, last);
    timed_event_epilogue(c, frame);
}

/// The two variants of the "landing sound" arm (events 1 and 2): play the
/// landing animation on layer 0x13b, reset the blend doubleword, then tell the
/// audio system which landing it was. `rets` are the original return
/// addresses (PlayAnim, GetInstance, RiderSay).
fn timed_event_land_sound(c: &mut Cpu, frame: Frame, say_arg: u64, rets: [u32; 3]) -> Flow {
    c.cycles += 19;
    c.g[V1][0] = lw(c, p(r32(c, S0), rider_anim::ANIM));
    c.g[A1][0] = 0x13b;
    c.g[V0][0] = ld(c, p(r32(c, S0), rider_anim::BLEND_SOURCE));
    c.g[A2][0] = 0;
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(1, 12);
    let v = c.g[V0][0];
    Addr(p(r32(c, V1), 0x20)).sd(c, v);
    c.g[A0][0] = lw(c, p(r32(c, S0), rider_anim::ANIM));
    anim_callee::PLAY_ANIM.call(c, rets[0])?;
    c.g[V1][0] = lw(c, p(r32(c, S0), rider_anim::ANIM));
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    Addr(p(r32(c, V1), 0x20)).sd(c, 0xffff_ffff_ffff_ffff);
    anim_callee::AUDIO_GET_INSTANCE.call(c, rets[1])?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = say_arg;
    anim_callee::AUDIO_RIDER_SAY.call(c, rets[2])?;
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    timed_event_tail(c, frame);
    Ok(())
}

/// `void Rider_UpdateTimedEventAnim(Rider* rider)` (0x00115b58): when the rider
/// has a pending timed event and the animation controller is idle (no layer
/// in the middle of a transition), pick the event's landing animation from
/// the rider's event code and start it; the "landing sound" events also tell
/// the audio system which landing played. The event clears after 180 frames,
/// or after the landing animation ran.
pub fn rider_update_timed_event_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw(c, p(r32(c, S0), rider_anim::EVENT));
        if c.g[V0][0] == 0 {
            timed_event_epilogue(c, frame);
            return Ok(());
        }
        // Busy checks: layer 2 must be idle (0), layer 1 idle, layer 2 not
        // in the 0xa state.
        c.cycles += 6;
        c.g[A0][0] = lw(c, p(r32(c, S0), rider_anim::ANIM));
        c.g[A1][0] = 2;
        anim_callee::GET_LAYER_ENTRY.call(c, 0x0011_5b80)?;
        c.g[V1][0] = 5;
        if c.g[V0][0] == c.g[V1][0] {
            timed_event_epilogue(c, frame);
            return Ok(());
        }
        c.cycles += 5;
        c.g[A0][0] = lw(c, p(r32(c, S0), rider_anim::ANIM));
        c.g[A1][0] = 1;
        anim_callee::GET_LAYER_ENTRY.call(c, 0x0011_5b98)?;
        if c.g[V0][0] != 0 {
            timed_event_epilogue(c, frame);
            return Ok(());
        }
        c.cycles += 6;
        c.g[A0][0] = lw(c, p(r32(c, S0), rider_anim::ANIM));
        c.g[A1][0] = 2;
        anim_callee::GET_LAYER_ENTRY.call(c, 0x0011_5bac)?;
        c.g[V1][0] = 0xa;
        if c.g[V0][0] == c.g[V1][0] {
            timed_event_epilogue(c, frame);
            return Ok(());
        }

        // Recent enough (under 180 frames) and not in the future.
        c.cycles += 7;
        anim_callee::FRAME_COUNTER.call(c, 0x0011_5bc0)?;
        c.g[V1][0] = lw(c, p(r32(c, S0), rider_anim::LAST_FRAME));
        c.g[V1][0] = sx(r32(c, V0).wrapping_sub(r32(c, V1)));
        c.g[V0][0] = ((c.g[V1][0] as i64) < 180) as u64;
        let recent = c.g[V0][0] != 0;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        if !recent {
            timed_event_tail(c, frame);
            return Ok(());
        }
        c.cycles += 2;
        if (c.g[V1][0] as i64) < 0 {
            timed_event_epilogue(c, frame);
            return Ok(());
        }
        c.cycles += 5;
        c.g[V0][0] = lw(c, p(r32(c, S0), rider_anim::EVENT));
        c.g[A0][0] = sx(r32(c, V0).wrapping_sub(1));
        c.g[V1][0] = (c.g[A0][0] < 6) as u64;
        let in_range = c.g[V1][0] != 0;
        c.g[V0][0] = 0x0045_0000;
        if !in_range {
            c.cycles += 1;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            timed_event_tail(c, frame);
            return Ok(());
        }
        c.cycles += 6;
        c.g[V1][0] = sx(r32(c, A0) << 2);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x6cc0));
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        c.g[A0][0] = lw(c, r32(c, V1));
        let target = r32(c, A0);
        match target {
            0x0011_5c08 => {
                timed_event_land_sound(c, frame, 0, [0x0011_5c2c, 0x0011_5c3c, 0x0011_5c4c])?;
                return Ok(());
            }
            0x0011_5c54 => {
                timed_event_land_sound(c, frame, 1, [0x0011_5c78, 0x0011_5c88, 0x0011_5c98])?;
                return Ok(());
            }
            0x0011_5ca0 => {
                c.cycles += 3;
                c.g[V1][0] = lw(c, p(r32(c, S0), rider_anim::ANIM));
                c.g[A1][0] = 0x13b;
            }
            0x0011_5cac => {
                c.cycles += 2;
                c.g[V1][0] = lw(c, p(r32(c, S0), rider_anim::ANIM));
                c.g[A1][0] = 0x13a;
            }
            0x0011_5ce0 => {
                // The landing-animation arm: only while the blend is below
                // 0.75 and the rider is not busy.
                c.cycles += 8;
                c.lwc1(0, p(r32(c, S0), rider_anim::SUB_BLEND));
                c.g[AT][0] = 0x3f40_0000;
                c.mtc1(1, 1);
                c.abs_s(0, 0);
                c.c_lt_s(0, 1);
                let high = !c.fcc();
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                if high {
                    timed_event_tail(c, frame);
                    return Ok(());
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                anim_callee::GET_SUB_STATE.call(c, 0x0011_5d08)?;
                let busy = c.g[V0][0] != 0;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                if busy {
                    timed_event_tail(c, frame);
                    return Ok(());
                }
                c.cycles += 6;
                c.g[A0][0] = lw(c, p(r32(c, S0), rider_anim::ANIM));
                c.g[A1][0] = 0x13e;
                c.g[AT][0] = 0xffff_ffff_bf80_0000;
                c.mtc1(1, 12);
                c.g[A2][0] = 0;
                anim_callee::PLAY_ANIM.call(c, 0x0011_5d28)?;
                c.cycles += 1;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                timed_event_tail(c, frame);
                return Ok(());
            }
            _ => {
                c.jump_out(target, 0x0011_5b58, 0x0011_5d44);
                return Ok(());
            }
        }

        // Shared by the 0x13b / 0x13a landing animations: start it, reset the
        // blend, then clear the event.
        c.cycles += 11;
        c.g[V0][0] = ld(c, p(r32(c, S0), rider_anim::BLEND_SOURCE));
        c.g[A2][0] = 0;
        c.g[AT][0] = 0xffff_ffff_bf80_0000;
        c.mtc1(1, 12);
        let v = c.g[V0][0];
        Addr(p(r32(c, V1), 0x20)).sd(c, v);
        c.g[A0][0] = lw(c, p(r32(c, S0), rider_anim::ANIM));
        anim_callee::PLAY_ANIM.call(c, 0x0011_5cd0)?;
        c.g[V1][0] = lw(c, p(r32(c, S0), rider_anim::ANIM));
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        Addr(p(r32(c, V1), 0x20)).sd(c, 0xffff_ffff_ffff_ffff);
        c.cycles += 1;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        timed_event_tail(c, frame);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Spray emitters.

mod spray {
    /// `cDynamicSprayFX+0x28`: the emitter (its particle block starts at
    /// `+0x630`), `+0xb0`: the spawn-interval timer.
    pub const EMITTER: u32 = 0x28;
    pub const INTERVAL: u32 = 0xb0;
    /// The random-number state (`gp+0xa0c`).
    pub const RANDOM_GP: i32 = 0xa0c;
}

mod spray_callee {
    use super::Callee;
    use ssx3_ee as ee;

    pub const ADD_RANDOM_VELOCITY: Callee = Callee {
        addr: 0x002d_e398,
        f: ee::s09::cDynamicSprayFX_AddRandomVelocity,
    };
    pub const SPAWN_AT: Callee = Callee { addr: 0x0037_17c0, f: ee::s12::DynamicEmitter_SpawnAt };
    pub const WORLD_PAINTER_GET_WEATHER: Callee = Callee {
        addr: 0x002e_e6f0,
        f: ee::s09::WorldPainter_GetWeather30,
    };
}

/// The emitter epilogue: `s0` is already back; `s1`, `s2`, `ra`, `f20` and the
/// frame follow.
fn emitter_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 6;
    frame.restore(c, S1, 0xe0);
    frame.restore(c, S2, 0xd0);
    frame.restore_ra(c, 0xc0);
    frame.restore_fpr(c, 20, 0x100);
    frame.close(c);
}

/// `void cDynamicSprayFX_UpdateEmitter3(cDynamicSprayFX* fx)` (0x002dfe88):
/// advance the spawn timer; when it expires, scatter the spawn velocity,
/// shorten the interval (clamped), build the spawn volume and hand it to the
/// emitter's spawn call. Without a live emitter (or while the interval has not
/// expired) the emitter is told to spawn with the default volume instead.
pub fn dynamic_spray_update_emitter3(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x110);
        frame.save(c, S0, 0xf0);
        frame.save(c, S2, 0xd0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0xe0);
        frame.save_ra(c, 0xc0);
        frame.save_fpr(c, 20, 0x100);
        c.g[V0][0] = lw(c, p(r32(c, S0), spray::EMITTER));
        c.g[V1][0] = lw(c, p(r32(c, S0), spray::INTERVAL));
        let idle = c.g[V1][0] == 0;
        c.g[S2][0] = sx(p(r32(c, V0), 0x630));
        if !idle {
            // Random state advance and the shortened spawn interval.
            c.cycles += 23;
            c.g[V1][0] = lw(c, gp(c, spray::RANDOM_GP));
            c.g[V0][0] = 0x0001_0000;
            c.g[V0][0] = sx(r32(c, V0) | 0x8fcd);
            c.g[A0][0] = 0x00e9_0000;
            c.mult(0, 3, 3, 2);
            c.g[A0][0] = sx(r32(c, A0) | 0x507c);
            c.g[V0][0] = 0x007f_0000;
            c.g[A1][0] = lw(c, p(r32(c, S0), 0xd0));
            c.g[V0][0] = sx(r32(c, V0) | 0xffff);
            c.g[A2][0] = 0x3f80_0000;
            c.lwc1(1, p(r32(c, A1), 0x6c));
            {
                let (a, b) = (r32(c, V1), r32(c, A0));
                c.g[V1][0] = sx(a.wrapping_add(b));
            }
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 20);
            {
                let (a, b) = (c.g[V1][0], c.g[V0][0]);
                c.g[V1][0] = a & b;
            }
            {
                let (a, b) = (c.g[V1][0], c.g[A2][0]);
                c.g[V1][0] = a | b;
            }
            let seed = c.g[V1][0];
            sw_gp(c, spray::RANDOM_GP, seed);
            c.lwc1(0, gp(c, spray::RANDOM_GP));
            c.sub_s(0, 0, 20);
            c.c_lt_s(0, 1);
            let wait = !c.fcc();
            c.g[A0][0] = c.g[S0][0];
            if !wait {
                // Scatter the new particle's velocity and build its volume.
                c.cycles += 36;
                c.g[AT][0] = 0x4170_0000;
                c.mtc1(1, 0);
                c.mfc1(2, 0);
                c.lqc2(4, p(r32(c, S0), 0x30));
                c.qmtc2(2, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, 0x20));
                c.g[A1][0] = c.g[SP][0];
                c.lq(2, at(c, 0x20));
                c.sq(2, at(c, 0x10));
                c.lqc2(3, p(r32(c, S0), 0xa0));
                c.lqc2(4, at(c, 0x10));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, 0x20));
                c.lq(2, at(c, 0x20));
                c.sq(2, at(c, 0));
                spray_callee::ADD_RANDOM_VELOCITY.call(c, 0x002d_ff54)?;
                // How far the particle is from the emitter centre.
                c.g[V1][0] = lw(c, r32(c, S0));
                c.mov_s(6, 20);
                c.lqc2(3, p(r32(c, V1), 0x3b0));
                c.lqc2(5, p(r32(c, S0), 0xc0));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(2, 4);
                c.mtc1(2, 0);
                c.g[AT][0] = 0x3fc0_0000;
                c.mtc1(1, 1);
                c.abs_s(0, 0);
                c.mul_s(0, 0, 1);
                c.c_lt_s(0, 20);
                if c.fcc() {
                    c.mov_s(6, 0);
                }

                // Spawn interval: blend towards the minimum while close.
                c.cycles += 19;
                ldf(c, 2, -0x3b7c);
                c.mov_s(5, 20);
                c.lwc1(1, p(r32(c, V1), 0x214));
                c.abs_s(1, 1);
                ldf(c, 3, -0x3b78);
                c.sub_s(1, 1, 2);
                c.lwc1(0, p(r32(c, S0), 0xb4));
                c.g[AT][0] = 0x4040_0000;
                c.mtc1(1, 2);
                c.abs_s(0, 0);
                c.sub_s(0, 0, 3);
                ldf(c, 4, -0x3b74);
                c.mul_s(1, 1, 2);
                c.mtc1(0, 3);
                c.mul_s(0, 0, 4);
                c.c_le_s(3, 1);
                c.mov_s(4, 3);
                let t = !c.fcc();
                c.add_s(2, 0, 6);
                if !t {
                    c.cycles += 1;
                    c.min_s(4, 1, 5);
                }
                c.cycles += 5;
                ldf(c, 0, -0x3b70);
                c.c_le_s(3, 2);
                c.mov_s(1, 3);
                let t = !c.fcc();
                c.mul_s(0, 4, 0);
                if !t {
                    c.cycles += 1;
                    c.min_s(1, 2, 5);
                }
                c.cycles += 4;
                c.c_lt_s(0, 1);
                if !c.fcc() {
                    c.mov_s(3, 0);
                } else {
                    c.cycles += 4;
                    c.c_le_s(3, 2);
                    let t = !c.fcc();
                    c.mov_s(0, 3);
                    if !t {
                        c.cycles += 1;
                        c.min_s(0, 2, 20);
                    }
                    c.cycles += 1;
                    c.mov_s(3, 0);
                }
                c.cycles += 15;
                ldf(c, 1, -0x3b6c);
                c.lwc1(0, p(r32(c, S0), 0xb8));
                c.g[AT][0] = 0x41a0_0000;
                c.mtc1(1, 2);
                c.mul_s(0, 0, 1);
                ldf(c, 20, -0x3b68);
                c.mtc1(0, 4);
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 1);
                c.sub_s(0, 0, 2);
                c.mul_s(0, 0, 20);
                c.c_le_s(4, 0);
                let t = !c.fcc();
                c.mov_s(2, 4);
                if !t {
                    c.cycles += 1;
                    c.min_s(2, 0, 1);
                }
                // The emitter's colour ramp random state.
                c.cycles += 20;
                c.g[V1][0] = lw(c, gp(c, spray::RANDOM_GP));
                c.g[V0][0] = 0x0001_0000;
                c.g[V0][0] = sx(r32(c, V0) | 0x8fcd);
                c.g[A0][0] = 0x00e9_0000;
                c.mult(0, 3, 3, 2);
                c.g[A0][0] = sx(r32(c, A0) | 0x507c);
                c.g[V0][0] = 0x007f_0000;
                c.g[A1][0] = 0x3f80_0000;
                c.g[V0][0] = sx(r32(c, V0) | 0xffff);
                c.mul_s(3, 3, 2);
                {
                    let (a, b) = (r32(c, V1), r32(c, A0));
                    c.g[V1][0] = sx(a.wrapping_add(b));
                }
                {
                    let (a, b) = (c.g[V1][0], c.g[V0][0]);
                    c.g[V1][0] = a & b;
                }
                {
                    let (a, b) = (c.g[V1][0], c.g[A1][0]);
                    c.g[V1][0] = a | b;
                }
                let seed = c.g[V1][0];
            sw_gp(c, spray::RANDOM_GP, seed);
                c.lwc1(0, gp(c, spray::RANDOM_GP));
                c.sub_s(0, 0, 1);
                c.c_lt_s(0, 3);
                let t = c.fcc();
                c.g[S1][0] = 1;
                if !t {
                    c.cycles += 1;
                    c.g[S1][0] = 0;
                }

                // Spawn volume: the emitter's axis and the size fields.
                c.cycles += 13;
                c.g[V0][0] = lw(c, r32(c, S0));
                c.lqc2(4, p(r32(c, S0), 0x50));
                c.lqc2(5, p(r32(c, V0), 0x370));
                c.vu0().opmula(4, 5);
                c.vu0().opmsub(6, 5, 4);
                c.vu0().sub(0b0001, 6, 6, Src::Vec(6));
                c.sqc2(6, at(c, 0x20));
                c.lwc1(0, p(r32(c, S0), 0xb4));
                c.lq(2, at(c, 0x20));
                c.c_lt_s(0, 4);
                let t = !c.fcc();
                c.sq(2, at(c, 0x10));
                if !t {
                    c.cycles += 10;
                    c.g[AT][0] = 0xffff_ffff_bf80_0000;
                    c.mtc1(1, 0);
                    c.mfc1(3, 0);
                    c.lqc2(4, at(c, 0x10));
                    c.qmtc2(3, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, at(c, 0x30));
                    c.lq(2, at(c, 0x30));
                    c.sq(2, at(c, 0x10));
                    c.sq(2, at(c, 0x20));
                }

                // Hand the volume to the emitter.
                c.cycles += 88;
                c.g[A0][0] = lw(c, r32(c, S0));
                c.g[V1][0] = lw(c, p(r32(c, A0), 0x6c0));
                c.g[A0][0] = sx(p(r32(c, A0), 0x6c0));
                c.g[A1][0] = lh(c, p(r32(c, V1), 0x10));
                c.g[V0][0] = lw(c, p(r32(c, V1), 0x14));
                let target = r32(c, V0);
                c.g[A0][0] = sx(p(r32(c, A0), r32(c, A1)));
                call_ptr(c, target, 0x002e_013c)?;
                c.g[V1][0] = lw(c, p(r32(c, S0), 0x2c));
                c.g[A0][0] = c.g[S2][0];
                c.g[A3][0] = sx(p(r32(c, S0), 0x90));
                c.g[T0][0] = c.g[S1][0];
                c.lwc1(0, p(r32(c, V1), 0x390));
                c.mov_s(12, 20);
                c.mfc1(3, 0);
                c.lqc2(4, r32(c, V0));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, 0x60));
                c.g[A1][0] = c.g[SP][0];
                c.lq(2, at(c, 0x60));
                c.g[A2][0] = sx(p(r32(c, SP), 0x20));
                c.g[T1][0] = lw(c, p(r32(c, S0), 0x2c));
                c.sq(2, at(c, 0x50));
                c.lwc1(0, p(r32(c, T1), 0x394));
                c.g[V1][0] = lw(c, r32(c, S0));
                c.mfc1(10, 0);
                c.lqc2(4, p(r32(c, V1), 0x370));
                c.qmtc2(10, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, 0x70));
                c.lq(2, at(c, 0x70));
                c.sq(2, at(c, 0x60));
                c.lqc2(3, at(c, 0x50));
                c.lqc2(4, at(c, 0x60));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, 0x70));
                c.lq(2, at(c, 0x70));
                c.g[T1][0] = lw(c, p(r32(c, S0), 0x2c));
                c.sq(2, at(c, 0x40));
                c.lwc1(0, p(r32(c, T1), 0x398));
                c.g[V1][0] = lw(c, r32(c, S0));
                c.mfc1(10, 0);
                c.lqc2(4, p(r32(c, V1), 0x370));
                c.qmtc2(10, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, 0x90));
                c.lq(2, at(c, 0x90));
                c.lwc1(0, p(r32(c, S0), 0xbc));
                c.sq(2, at(c, 0x80));
                c.mfc1(3, 0);
                c.lqc2(4, at(c, 0x80));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, 0x90));
                c.lq(2, at(c, 0x90));
                c.sq(2, at(c, 0x70));
                c.lqc2(3, at(c, 0x40));
                c.lqc2(4, at(c, 0x70));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, 0x90));
                c.lq(2, at(c, 0x90));
                c.g[V1][0] = lw(c, p(r32(c, S0), 0x2c));
                c.sq(2, at(c, 0x30));
                c.lwc1(0, p(r32(c, V1), 0x39c));
                c.mfc1(9, 0);
                c.lqc2(4, at(c, 0x10));
                c.qmtc2(9, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, 0xb0));
                c.lq(2, at(c, 0xb0));
                c.lwc1(0, p(r32(c, S0), 0xbc));
                c.sq(2, at(c, 0xa0));
                c.mfc1(3, 0);
                c.lqc2(4, at(c, 0xa0));
                c.qmtc2(3, 3);
                c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                c.sqc2(5, at(c, 0xb0));
                c.lq(2, at(c, 0xb0));
                c.sq(2, at(c, 0x90));
                c.lqc2(3, at(c, 0x30));
                c.lqc2(4, at(c, 0x90));
                c.vu0().add(0b1111, 5, 3, Src::Vec(4));
                c.sqc2(5, at(c, 0xb0));
                c.lq(2, at(c, 0xb0));
                c.sq(2, at(c, 0x20));
                spray_callee::SPAWN_AT.call(c, 0x002e_0278)?;
                frame.restore(c, S0, 0xf0);
                emitter_epilogue(c, frame);
                return Ok(());
            }
        }

        // No new interval: spawn with the default volume.
        c.cycles += 8;
        ldf(c, 12, -0x3b64);
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = sx(p(r32(c, S0), 0xa0));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        spray_callee::SPAWN_AT.call(c, 0x002e_029c)?;
        frame.restore(c, S0, 0xf0);
        emitter_epilogue(c, frame);
        Ok(())
    })
}

/// `cDynamicSprayFX_UpdateEmitter4`'s epilogue.
fn emitter4_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 8;
    frame.restore(c, S1, 0xf0);
    frame.restore(c, S2, 0xe0);
    frame.restore_ra(c, 0xd0);
    frame.restore_fpr(c, 22, 0x120);
    frame.restore_fpr(c, 21, 0x118);
    frame.restore_fpr(c, 20, 0x110);
    frame.close(c);
}

/// The default spawn from the "wobble timer not reached" path (0x2e1530).
fn emitter4_spawn_mid(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 9;
    c.g[A0][0] = c.g[S1][0];
    ldf(c, 12, -0x3ac4);
    c.g[A1][0] = sx(p(r32(c, SP), 0x40));
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    c.g[T0][0] = 0;
    spray_callee::SPAWN_AT.call(c, 0x002e_154c)?;
    frame.restore(c, S0, 0x100);
    emitter4_epilogue(c, frame);
    Ok(())
}

/// The default spawn once the wobble timer is spent (0x2e1554): also stop the
/// emitter's rate timer.
fn emitter4_spawn_late(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 9;
    sw_at(c, S0, 0x14, 0);
    c.g[A0][0] = c.g[S1][0];
    ldf(c, 12, -0x3ac0);
    c.g[A1][0] = sx(p(r32(c, SP), 0x40));
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    c.g[T0][0] = 0;
    spray_callee::SPAWN_AT.call(c, 0x002e_1574)?;
    frame.restore(c, S0, 0x100);
    emitter4_epilogue(c, frame);
    Ok(())
}

/// `void cDynamicSprayFX_UpdateEmitter4(cDynamicSprayFX* fx)` (0x002e1120):
/// the board-wake / weather-driven emitter. Copies the emitter's placement
/// matrix, advances the wobble and rate timers, gets the current weather and
/// derives the particle scale from it, then hands a spawn volume to the
/// emitter (or tells it to spawn with the default volume while the timers
/// have not expired).
pub fn dynamic_spray_update_emitter4(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 52;
        let frame = Frame::open(c, 0x130);
        frame.save(c, S0, 0x100);
        frame.save(c, S1, 0xf0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0xe0);
        frame.save_ra(c, 0xd0);
        frame.save_fpr(c, 22, 0x120);
        frame.save_fpr(c, 21, 0x118);
        frame.save_fpr(c, 20, 0x110);
        c.g[V1][0] = lw(c, r32(c, S0));
        c.g[A0][0] = lw(c, p(r32(c, S0), spray::EMITTER));
        c.g[A1][0] = lw(c, p(r32(c, V1), 0x780));
        c.g[V0][0] = lw(c, p(r32(c, V1), 0x8a8));
        c.g[S1][0] = sx(p(r32(c, A0), 0x840));
        c.g[V1][0] = lw(c, p(r32(c, A1), 0x30));
        c.g[V0][0] = sx(r32(c, V0) << 6);
        c.g[V0][0] = sx(p(r32(c, V0), r32(c, V1)));
        c.lqc2(1, r32(c, V0));
        c.lqc2(2, p(r32(c, V0), 0x10));
        c.lqc2(3, p(r32(c, V0), 0x20));
        c.lqc2(4, p(r32(c, V0), 0x30));
        c.sqc2(1, sp(c));
        c.sqc2(2, at(c, 0x10));
        c.sqc2(3, at(c, 0x20));
        c.sqc2(4, at(c, 0x30));
        c.g[AT][0] = 0xffff_ffff_c040_0000;
        c.mtc1(1, 0);
        c.g[AT][0] = 0x4080_0000;
        c.mtc1(1, 1);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 21);
        c.swc1(1, at(c, 0x58));
        c.swc1(0, at(c, 0x50));
        let v = p(sp(c), 0x54);
        sw(c, v, 0);
        c.swc1(21, at(c, 0x5c));
        c.lqc2(8, at(c, 0x50));
        c.lqc2(4, sp(c));
        c.lqc2(5, at(c, 0x10));
        c.lqc2(6, at(c, 0x20));
        c.lqc2(7, at(c, 0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, at(c, 0x60));
        c.lwc1(1, gp(c, -0x3ae8));
        c.lwc1(0, p(r32(c, S0), 0xbc));
        c.lq(2, at(c, 0x60));
        c.c_lt_s(1, 0);
        let t = !c.fcc();
        c.sq(2, at(c, 0x40));
        if !t {
            c.cycles += 2;
            sw_at(c, S0, 0x14, 0);
            return emitter4_spawn_mid(c, frame);
        }

        // Layer-2 animation state: 0x1e keeps the scale at one.
        c.cycles += 8;
        c.g[V0][0] = lw(c, r32(c, S0));
        c.g[A1][0] = 2;
        c.mtc1(0, 20);
        c.g[A0][0] = lw(c, p(r32(c, V0), 0x784));
        call_ptr(c, 0x0031_1ae8, 0x002e_120c)?;
        c.g[V1][0] = 0x1e;
        if c.g[V0][0] == c.g[V1][0] {
            c.mov_s(20, 21);
        }
        // Blend the animation's current time towards the target.
        c.cycles += 4;
        c.lwc1(1, p(r32(c, S0), 0x1c));
        c.c_le_s(1, 20);
        let above = !c.fcc();
        if above {
            c.cycles += 3;
            c.lwc1(0, gp(c, -0x3ae4));
            c.sub_s(0, 1, 0);
            c.swc1(0, p(r32(c, S0), 0x1c));
        } else {
            c.cycles += 2;
            c.swc1(20, p(r32(c, S0), 0x1c));
        }
        // Wobble timer: when it expires, toggle the wobble direction and
        // re-arm (or take the 0.4 s hold).
        c.cycles += 8;
        c.lwc1(0, p(r32(c, S0), 0x20));
        c.lwc1(1, gp(c, -0x3ae0));
        c.lwc1(2, p(r32(c, S0), 0x24));
        c.add_s(0, 0, 1);
        c.c_le_s(2, 0);
        let t = !c.fcc();
        c.swc1(0, p(r32(c, S0), 0x20));
        let mut wobble = false;
        if !t {
            c.cycles += 5;
            c.g[V0][0] = lw(c, p(r32(c, S0), 0x18));
            sw_at(c, S0, 0x20, 0);
            {
                let v = c.g[V0][0] ^ 1;
                c.g[V0][0] = v;
            }
            let turned_off = c.g[V0][0] == 0;
            let v = c.g[V0][0];
            sw_at(c, S0, 0x18, v);
            if turned_off {
                wobble = true;
            } else {
                c.cycles += 3;
                c.lwc1(0, gp(c, -0x3adc));
                c.swc1(0, p(r32(c, S0), 0x24));
            }
        }
        if wobble {
            c.cycles += 9;
            c.lwc1(2, p(r32(c, S0), 0x1c));
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            ldf(c, 3, -0x3ad8);
            c.sub_s(0, 0, 2);
            ldf(c, 1, -0x3ad4);
            c.mul_s(0, 0, 3);
            c.add_s(0, 0, 1);
            c.swc1(0, p(r32(c, S0), 0x24));
        }

        // Weather-driven colour/scale.
        c.cycles += 29;
        c.g[A0][0] = lw(c, r32(c, S0));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 20);
        c.g[A1][0] = lw(c, p(r32(c, A0), 0x6c0));
        c.g[A0][0] = sx(p(r32(c, A0), 0x6c0));
        c.mtc1(0, 22);
        c.g[V1][0] = lh(c, p(r32(c, A1), 0x38));
        c.g[V0][0] = lw(c, p(r32(c, A1), 0x3c));
        let target = r32(c, V0);
        c.g[A0][0] = sx(p(r32(c, A0), r32(c, V1)));
        call_ptr(c, target, 0x002e_12c8)?;
        c.g[A0][0] = c.g[V0][0];
        spray_callee::WORLD_PAINTER_GET_WEATHER.call(c, 0x002e_12d0)?;
        c.g[AT][0] = 0x40a0_0000;
        c.mtc1(1, 2);
        ldf(c, 3, -0x3ad0);
        c.add_s(0, 0, 2);
        c.lwc1(1, p(r32(c, S0), 0x1c));
        c.g[AT][0] = 0x3f00_0000;
        c.mtc1(1, 2);
        c.g[AT][0] = 0x4000_0000;
        c.mtc1(1, 5);
        c.mul_s(0, 0, 3);
        c.add_s(1, 1, 2);
        c.sub_s(0, 20, 0);
        c.mul_s(1, 1, 0);
        c.c_le_s(22, 1);
        let t = !c.fcc();
        c.mov_s(4, 22);
        if !t {
            c.cycles += 1;
            c.min_s(4, 1, 5);
        }

        // Spawn rate gate.
        c.cycles += 7;
        c.g[V0][0] = lw(c, p(r32(c, S0), 0x18));
        c.g[S2][0] = sx(p(r32(c, SP), 0x50));
        c.swc1(20, at(c, 0x50));
        c.swc1(20, at(c, 0x54));
        c.swc1(20, at(c, 0x58));
        let t = c.g[V0][0] == 0;
        c.swc1(4, at(c, 0x5c));
        if t {
            return emitter4_spawn_late(c, frame);
        }
        c.cycles += 5;
        c.lwc1(0, gp(c, -0x3acc));
        c.c_lt_s(0, 4);
        let t = !c.fcc();
        c.g[V1][0] = lw(c, gp(c, spray::RANDOM_GP));
        if t {
            return emitter4_spawn_late(c, frame);
        }

        // Spawn with the derived volume and scale.
        c.cycles += 28;
        c.g[V0][0] = 0x0001_0000;
        c.g[V0][0] = sx(r32(c, V0) | 0x8fcd);
        c.g[A0][0] = 0x00e9_0000;
        c.mult(0, 3, 3, 2);
        c.g[A0][0] = sx(r32(c, A0) | 0x507c);
        c.g[V0][0] = 0x007f_0000;
        c.g[A1][0] = 0x3f80_0000;
        c.g[V0][0] = sx(r32(c, V0) | 0xffff);
        c.lwc1(1, p(r32(c, S0), 0x14));
        c.g[AT][0] = 0x40c0_0000;
        c.mtc1(1, 2);
        {
            let (a, b) = (r32(c, V1), r32(c, A0));
            c.g[V1][0] = sx(a.wrapping_add(b));
        }
        c.g[AT][0] = 0x4190_0000;
        c.mtc1(1, 3);
        {
            let (a, b) = (c.g[V1][0], c.g[V0][0]);
            c.g[V1][0] = a & b;
        }
        ldf(c, 21, -0x3ac8);
        {
            let (a, b) = (c.g[V1][0], c.g[A1][0]);
            c.g[V1][0] = a | b;
        }
        let seed = c.g[V1][0];
            sw_gp(c, spray::RANDOM_GP, seed);
        c.lwc1(0, gp(c, spray::RANDOM_GP));
        c.sub_s(0, 0, 20);
        c.mul_s(0, 0, 2);
        c.add_s(0, 0, 3);
        c.mul_s(0, 0, 21);
        c.add_s(0, 1, 0);
        c.c_lt_s(20, 0);
        let t = !c.fcc();
        c.swc1(0, p(r32(c, S0), 0x14));
        if t {
            return emitter4_spawn_mid(c, frame);
        }

        c.cycles += 94;
        c.sub_s(0, 0, 20);
        c.g[V1][0] = 0x0050_0000;
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0xed0));
        c.g[A0][0] = lw(c, r32(c, S0));
        c.swc1(0, p(r32(c, S0), 0x14));
        c.g[A1][0] = sx(p(r32(c, A0), 0x6c0));
        c.lq(2, r32(c, V1));
        c.sq(2, at(c, 0x30));
        c.g[V1][0] = lw(c, p(r32(c, A0), 0x6c0));
        c.g[A0][0] = lh(c, p(r32(c, V1), 0x10));
        c.g[V0][0] = lw(c, p(r32(c, V1), 0x14));
        let target = r32(c, V0);
        c.g[A0][0] = sx(p(r32(c, A1), r32(c, A0)));
        call_ptr(c, target, 0x002e_13ec)?;
        c.g[V1][0] = lw(c, p(r32(c, S0), 0x2c));
        c.g[A0][0] = c.g[S1][0];
        c.g[AT][0] = 0x4316_0000;
        c.mtc1(1, 1);
        c.g[A3][0] = c.g[S2][0];
        c.lwc1(0, p(r32(c, V1), 0x478));
        c.mov_s(12, 21);
        c.mfc1(3, 0);
        c.lqc2(4, r32(c, V0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, 0x90));
        c.g[A1][0] = sx(p(r32(c, SP), 0x40));
        c.lq(2, at(c, 0x90));
        c.g[A2][0] = sx(p(r32(c, SP), 0x60));
        c.g[T1][0] = lw(c, p(r32(c, S0), 0x2c));
        c.g[T0][0] = 1;
        c.sq(2, at(c, 0x80));
        c.lwc1(0, p(r32(c, T1), 0x47c));
        c.g[V1][0] = lw(c, r32(c, S0));
        c.mfc1(10, 0);
        c.lqc2(4, p(r32(c, V1), 0x370));
        c.qmtc2(10, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, 0xa0));
        c.lq(2, at(c, 0xa0));
        c.sq(2, at(c, 0x90));
        c.lqc2(3, at(c, 0x80));
        c.lqc2(4, at(c, 0x90));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, 0xa0));
        c.lq(2, at(c, 0xa0));
        c.g[T1][0] = lw(c, p(r32(c, S0), 0x2c));
        c.sq(2, at(c, 0x70));
        c.lwc1(0, p(r32(c, T1), 0x480));
        c.g[V1][0] = lw(c, r32(c, S0));
        c.mfc1(10, 0);
        c.lqc2(4, p(r32(c, V1), 0x370));
        c.qmtc2(10, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, 0xc0));
        c.lq(2, at(c, 0xc0));
        c.lwc1(0, p(r32(c, S0), 0xbc));
        c.sq(2, at(c, 0xb0));
        c.mfc1(3, 0);
        c.lqc2(4, at(c, 0xb0));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, at(c, 0xc0));
        c.lq(2, at(c, 0xc0));
        c.sq(2, at(c, 0xa0));
        c.lqc2(3, at(c, 0x70));
        c.lqc2(4, at(c, 0xa0));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, 0xc0));
        c.lq(2, at(c, 0xc0));
        c.swc1(22, at(c, 0x84));
        c.sq(2, at(c, 0x60));
        c.swc1(1, at(c, 0x88));
        c.swc1(20, at(c, 0x8c));
        c.swc1(22, at(c, 0x80));
        c.lqc2(8, at(c, 0x80));
        c.lqc2(4, sp(c));
        c.lqc2(5, at(c, 0x10));
        c.lqc2(6, at(c, 0x20));
        c.lqc2(7, at(c, 0x30));
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, at(c, 0x90));
        c.lq(2, at(c, 0x90));
        c.sq(2, at(c, 0x70));
        c.lqc2(3, at(c, 0x60));
        c.lqc2(4, at(c, 0x70));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, at(c, 0x60));
        spray_callee::SPAWN_AT.call(c, 0x002e_1528)?;
        frame.restore(c, S0, 0x100);
        emitter4_epilogue(c, frame);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine, FAKE};
    use crate::difftest::{elf_image, put_words, Rng, DATA, GAME_GP};

    #[test]
    fn weather_save_state_matches() {
        covered(
            ssx3_ee::s08::f_002bfeb0__generated,
            weather_save_state,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x5a17, 38);
                c.pc = 0x002b_feb0;
                let owner = DATA + 0x1_0000;
                let vt = DATA + 0x2_0000;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = owner as u64;
                c.bus.write32(owner, vt);
                c.bus.write16(vt + 8, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(vt + 0xc, FAKE + 0x10 * (r.next_u32() % 4));
            },
        );
    }

    fn put_f32s(c: &mut Cpu, a: u32, v: [f32; 4]) {
        crate::difftest::put_vec(
            c,
            a,
            [v[0].to_bits(), v[1].to_bits(), v[2].to_bits(), v[3].to_bits()],
        );
    }

    #[test]
    fn compute_steer_matches() {
        covered(
            ssx3_ee::s00::f_00100348__generated,
            computer_actor_rider_compute_steer,
            200,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x51ce, 2);
                c.pc = 0x0010_0348;
                let rider = DATA;
                let model = DATA + 0x1_0000;
                let vt = DATA + 0x2_0000;
                let axis = DATA + 0x3_0000;
                let motion = DATA + 0x3_1000;
                let vel = DATA + 0x3_2000;
                c.g[A0][0] = rider as u64;
                c.g[A1][0] = vel as u64;
                c.bus.write32(rider + 0x18, model);
                c.bus.write32(model + 0x6c0, vt);
                c.bus.write16(vt + 0x10, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(vt + 0x14, FAKE);
                c.bus.write16(vt + 0x28, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(vt + 0x2c, FAKE + 0x10);
                // The two virtual calls return these two quadwords.
                c.bus.write32(fakes::LOG_RESULTS, axis);
                c.bus.write32(fakes::LOG_RESULTS + 4, motion);
                put_f32s(c, motion, [0.0; 4]);
                match seed % 6 {
                    // Exactly axis-aligned: the degenerate f20 == 0 paths.
                    0 => {
                        put_f32s(c, model + 0x1c0, [1.0, 0.0, 0.0, 0.0]);
                        put_f32s(c, axis, [1.0, 0.0, 0.0, 0.0]);
                        put_f32s(c, vel, [0.0, 1.0, 0.0, 0.0]);
                    }
                    1 => {
                        put_f32s(c, model + 0x1c0, [0.0, 1.0, 0.0, 0.0]);
                        put_f32s(c, axis, [0.0, 0.0, -1.0, 0.0]);
                        put_f32s(c, vel, [1.0, 0.0, 0.0, 0.0]);
                    }
                    2 => {
                        put_f32s(c, model + 0x1c0, [0.0, 1.0, 0.0, 0.0]);
                        put_f32s(c, axis, [0.0, 0.0, 1.0, 0.0]);
                        put_f32s(c, vel, [1.0, 0.0, 0.0, 0.0]);
                    }
                    // Random: the atan path; seed % 6 == 4 is the backwards
                    // branch (the model's z above zero).
                    4 => {
                        put_f32s(
                            c,
                            model + 0x1c0,
                            [f32::from_bits(r.f32(2.0)), f32::from_bits(r.f32(2.0)), 1.0, 1.0],
                        );
                        put_f32s(c, axis, [f32::from_bits(r.f32(2.0)), f32::from_bits(r.f32(2.0)), f32::from_bits(r.f32(2.0)), 0.0]);
                        put_f32s(c, vel, [f32::from_bits(r.f32(2.0)), f32::from_bits(r.f32(2.0)), 0.0, 0.0]);
                    }
                    _ => {
                        put_f32s(
                            c,
                            model + 0x1c0,
                            [f32::from_bits(r.f32(2.0)), f32::from_bits(r.f32(2.0)), -f32::from_bits(r.f32(1.0)), 0.0],
                        );
                        put_f32s(c, axis, [f32::from_bits(r.f32(2.0)), f32::from_bits(r.f32(2.0)), f32::from_bits(r.f32(2.0)), 0.0]);
                        put_f32s(c, vel, [f32::from_bits(r.f32(2.0)), f32::from_bits(r.f32(2.0)), 0.0, 0.0]);
                    }
                }
            },
        );
    }

    #[test]
    fn rider_update_timed_event_anim_matches() {
        covered(
            ssx3_ee::s00::f_00115b58__generated,
            rider_update_timed_event_anim,
            200,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x1b58, 1);
                c.pc = 0x0011_5b58;
                let rider = DATA;
                let ctrl = DATA + 0x4000;
                let vt = fakes::vtable(c, &mut r, &[0x20]);
                c.g[A0][0] = rider as u64;
                c.bus.write32(rider + 0x784, ctrl);
                c.bus.write32(ctrl + 0x58, vt);
                c.bus.write32(ctrl + 4, 0x1b6);
                c.bus.write32(ctrl + 8, 0x1b6);
                for i in 0..8 {
                    c.bus.write32(fakes::LOG_RESULTS + 4 * i, 0x207);
                }
                // AI_GetFrameCounter: [[[gp-0x848]+0x84]+0xc]+8.
                let (a, b, d) = (DATA + 0x5000, DATA + 0x5100, DATA + 0x5200);
                c.bus.write32(GAME_GP - 0x848, a);
                c.bus.write32(a + 0x84, b);
                c.bus.write32(b + 0xc, d);
                c.bus.write32(d + 8, 100);
                c.bus.write32(rider + 0x354, 90);
                c.bus.write32(rider + 0x1fc, 0.5f32.to_bits());
                c.bus.write64(rider + 0x8c8, 0x1234_5678_9abc_def0);
                put_words(
                    c,
                    0x0045_6cc0,
                    &[0x0011_5c08, 0x0011_5c54, 0x0011_5ca0, 0x0011_5cac, 0x0011_5ce0, 0x0011_5ce0],
                );
                let mut event = 1;
                match seed % 13 {
                    0 => event = 0,
                    1 => {
                        // 500 frames since the event: too old.
                        c.bus.write32(d + 8, 500);
                        c.bus.write32(rider + 0x354, 0);
                    }
                    2 => {
                        // In the future: ignore.
                        c.bus.write32(d + 8, 10);
                        c.bus.write32(rider + 0x354, 100);
                    }
                    4 => event = 2,
                    5 => event = 3,
                    6 => event = 4,
                    7 => event = 5,
                    8 => {
                        // The landing arm, blend above 0.75.
                        event = 5;
                        c.bus.write32(rider + 0x1fc, 1.5f32.to_bits());
                    }
                    9 => event = 6,
                    10 => event = 7,
                    11 => event = 8,
                    12 => {
                        // The landing arm with the rider busy.
                        event = 5;
                        let state = DATA + 0x6000;
                        c.bus.write32(rider + 0x77c, state);
                        c.bus.write32(state + 0xde4, 1);
                    }
                    _ => {}
                }
                c.bus.write32(rider + 0x358, event);
            },
        );
    }

    fn put_spray_fx(c: &mut Cpu, r: &mut Rng, seed: u32) -> u32 {
        let fx = DATA;
        let owner = DATA + 0x2000;
        let vt = fakes::vtable(c, r, &[0x10]);
        c.bus.write32(fx, owner);
        c.bus.write32(fx + 0x28, DATA + 0x3000);
        c.bus.write32(owner + 0x6c0, vt);
        c.bus.write32(fx + 0xd0, DATA + 0x4000);
        c.bus.write32(fx + 0x2c, DATA + 0x5000);
        // Random geometry and scales (both emitters read the same fields).
        for off in [0x30, 0xa0, 0xc0] {
            put_f32s(c, fx + off, [f32::from_bits(r.f32(10.0)), f32::from_bits(r.f32(10.0)), f32::from_bits(r.f32(10.0)), f32::from_bits(r.f32(1.0))]);
        }
        for off in [0xb4, 0xb8, 0xbc] {
            c.bus.write32(fx + off, r.f32(2.0));
        }
        for off in [0x390, 0x394, 0x398, 0x39c] {
            c.bus.write32(DATA + 0x5000 + off, r.f32(5.0));
        }
        c.bus.write32(DATA + 0x5000 + 0x214, r.f32(5.0));
        c.bus.write32(owner + 0x214, r.f32(5.0));
        for off in [0x370, 0x3b0] {
            put_f32s(c, owner + off, [f32::from_bits(r.f32(4.0)), f32::from_bits(r.f32(4.0)), f32::from_bits(r.f32(4.0)), 0.0]);
        }
        match seed % 6 {
            // No timer: the default spawn only.
            0 => c.bus.write32(fx + 0xb0, 0),
            // Timer still running: the default spawn only.
            1 => {
                c.bus.write32(fx + 0xb0, r.f32(1.0));
                c.bus.write32(DATA + 0x406c, (-2.0f32).to_bits());
            }
            // The elaborate spawn.
            2 => {
                c.bus.write32(fx + 0xb0, r.f32(1.0));
                c.bus.write32(DATA + 0x406c, 2.0f32.to_bits());
            }
            _ => {
                c.bus.write32(fx + 0xb0, r.f32(1.0));
                c.bus.write32(DATA + 0x406c, (f32::from_bits(r.f32(1.0)) + 2.0).to_bits());
            }
        }
        fx
    }

    #[test]
    fn dynamic_spray_update_emitter3_matches() {
        covered(
            ssx3_ee::s09::f_002dfe88__generated,
            dynamic_spray_update_emitter3,
            200,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x2dfe, 1);
                c.pc = 0x002d_fe88;
                let fx = put_spray_fx(c, &mut r, seed);
                c.g[A0][0] = fx as u64;
            },
        );
    }

    #[test]
    fn dynamic_spray_update_emitter4_matches() {
        covered(
            ssx3_ee::s09::f_002e1120__generated,
            dynamic_spray_update_emitter4,
            200,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x2e11, 2);
                c.pc = 0x002e_1120;
                let fx = DATA;
                let owner = DATA + 0x2000;
                let ctrl = DATA + 0x6000;
                let ctrl2 = DATA + 0x6400;
                let vt = fakes::vtable(c, &mut r, &[0x10, 0x38]);
                c.g[A0][0] = fx as u64;
                c.bus.write32(fx, owner);
                c.bus.write32(fx + 0x28, DATA + 0x3000);
                c.bus.write32(fx + 0x2c, DATA + 0x4000);
                c.bus.write32(owner + 0x780, ctrl);
                c.bus.write32(ctrl + 0x30, DATA + 0x6200);
                c.bus.write32(owner + 0x8a8, r.next_u32() % 4);
                c.bus.write32(owner + 0x784, ctrl2);
                c.bus.write32(ctrl2 + 8, 0x1b6);
                c.bus.write32(owner + 0x6c0, vt);
                for off in [0x370, 0x3b0] {
                    put_f32s(c, owner + off, [f32::from_bits(r.f32(4.0)), f32::from_bits(r.f32(4.0)), f32::from_bits(r.f32(4.0)), 0.0]);
                }
                for off in [0x478, 0x47c, 0x480] {
                    c.bus.write32(DATA + 0x4000 + off, r.f32(5.0));
                }
                c.bus.write32(fx + 0xbc, 500.0f32.to_bits());
                c.bus.write32(fx + 0x1c, 0.1f32.to_bits());
                c.bus.write32(fx + 0x20, 0);
                c.bus.write32(fx + 0x24, 0);
                match seed % 6 {
                    // Too far: the entry picks the default spawn.
                    0 => c.bus.write32(fx + 0xbc, 2000.0f32.to_bits()),
                    // Wobble timer spent: default spawn after the timers.
                    1 => {
                        c.bus.write32(fx + 0x18, 1);
                        c.bus.write32(fx + 0x14, 1.0f32.to_bits());
                    }
                    // Rate below one: default spawn from the rate gate.
                    2 => c.bus.write32(fx + 0x14, 0.0f32.to_bits()),
                    // The full spawn.
                    3 => {
                        c.bus.write32(fx + 0x14, 1.0f32.to_bits());
                        c.bus.write32(fx + 0x1c, r.f32(0.5));
                    }
                    _ => {
                        c.bus.write32(fx + 0x14, 1.0f32.to_bits());
                        c.bus.write32(fx + 0x18, r.next_u32() % 2);
                        c.bus.write32(fx + 0x1c, r.f32(0.5));
                    }
                }
            },
        );
    }
}
