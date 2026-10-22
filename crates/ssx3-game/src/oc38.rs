//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for rider ground state and race progress (walls, boost, drag,
//! timers, speeds, AI path, cruise input), each with a differential test
//! against the generated body.
//!
//! - **Progress**: `Rider_UpdateRaceProgress` (per-frame race-path event
//!   bookkeeping), `Rider_UpdateAIPathProgress` (AI path follow + look-ahead),
//!   `Rider_CheckRaceFinish` (finish / did-not-finish handling).
//! - **Rider body**: `Rider_UpdateBoostTimers` (boost decay chain),
//!   `Rider_UpdateHeadingAndAudio` (ground-normal heading blend and the audio
//!   hooks), `Rider_UpdateVisibility` (on-screen test), `Rider_ApplyWindForce`
//!   (wind push along the weather direction).
//!
//! The replacements keep their state in the guest registers the originals use
//! where callers, callees or event handlers can see them, follow the call
//! protocol of [`crate::call`] and the event rules of [`crate::events`]; VU0
//! code is issued as the original's macro ops.

use crate::abi;
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0012_5970, rider_apply_wind_force, "Rider_ApplyWindForce"),
    (0x0012_1950, rider_update_heading_and_audio, "Rider_UpdateHeadingAndAudio"),
    (0x0012_2278, rider_update_visibility, "Rider_UpdateVisibility"),
    (0x0012_00d0, rider_update_boost_timers, "Rider_UpdateBoostTimers"),
    (0x0012_5228, rider_check_race_finish, "Rider_CheckRaceFinish"),
    (0x0011_2338, rider_update_race_progress, "Rider_UpdateRaceProgress"),
    (0x0011_25c0, rider_update_ai_path_progress, "Rider_UpdateAIPathProgress"),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const SSX_AUDIO_GET_INSTANCE: u32 = 0x0028_b180;
    pub const SSX_AUDIO_UPDATE_RIDER_WORLD_PAINT: u32 = 0x0028_98a8;
    pub const MATH_ATAN: u32 = 0x0031_c228;
    pub const WORLD_PAINTER_WEATHER28: u32 = 0x002e_f0a0;
    pub const WORLD_PAINTER_WEATHER20: u32 = 0x002e_e570;
    pub const BELIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;
    pub const BELIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;
    pub const FIXUNSSFSI: u32 = 0x0041_39f8;
    pub const SSX_AUDIO_ARCADE_SAY: u32 = 0x002a_3c00;
    pub const PATH_GET_CLOSEST_ON_PATH: u32 = 0x0026_a638;
    pub const PATH_GET_CLOSEST: u32 = 0x0026_a428;
    pub const PATH_GET_POINT_AT_DISTANCE: u32 = 0x0026_ab20;
    pub const PATH_GET_LENGTH: u32 = 0x0026_ac48;
    pub const EA_MEM_COPY: u32 = 0x003e_6574;

    pub const RIDER_IS_ON_GROUND: Callee =
        Callee { addr: 0x0012_31a8, f: ee::s00::Rider_IsOnGround };
    pub const RIDER_UPDATE_DANGER_STATE: Callee =
        Callee { addr: 0x0012_2cf0, f: ee::s00::Rider_UpdateDangerState };
    pub const RIDER_GET_MAIN_STATE: Callee =
        Callee { addr: 0x0011_fe98, f: ee::s00::Rider_GetMainState };
    pub const RIDER_GET_SUB_STATE: Callee =
        Callee { addr: 0x0011_fee8, f: ee::s00::Rider_GetSubState };
    pub const RIDER_SET_TIMED_EVENT: Callee =
        Callee { addr: 0x0010_e028, f: ee::s00::Rider_SetTimedEvent };
    pub const RIDER_UPDATE_DIST_TO_FINISH: Callee =
        Callee { addr: 0x0011_2fb0, f: ee::s00::Rider_UpdateDistToFinish };
    pub const RIDER_SELECT_RACE_PATH: Callee =
        Callee { addr: 0x0011_27f0, f: ee::s00::Rider_SelectRacePath };
    pub const RIDER_HANDLE_RACE_PATH_EVENT: Callee =
        Callee { addr: 0x0010_e5d8, f: ee::s00::Rider_HandleRacePathEvent };
    pub const RIDER_RETURN_ZERO: Callee =
        Callee { addr: 0x0011_25b8, f: ee::s00::Rider_ReturnZero_25b8 };
    pub const RIDER_AI_LOOK_AHEAD: Callee =
        Callee { addr: 0x0011_2588, f: ee::s00::Rider_GetAIPathLookAheadDist };
    pub const RIDER_SAVE_AI_LOOK_AHEAD: Callee =
        Callee { addr: 0x0011_25a8, f: ee::s00::Rider_SaveAIPathLookAhead };
    pub const RIDER_SELECT_AI_PATH: Callee =
        Callee { addr: 0x0011_2a50, f: ee::s00::Rider_SelectAIPath };
    pub const RIDER_ON_FINISH_RACE: Callee =
        Callee { addr: 0x0012_5108, f: ee::s00::Rider_OnFinishRace };
}

// ---------------------------------------------------------------------------
// Rider object fields (one `cRider`; the embedded rider-base object sits at
// +0x6c0 and its vtable is the pointer stored there).

mod rider {
    /// The rider base object (state machine + embedded vtable pointer).
    pub const BASE: u32 = 0x6c0;
    /// The rider handler object, whose base holds the main/sub state words at
    /// +0xde0/+0xde4.
    pub const HANDLER: u32 = 0x77c;
    /// World position (vec4).
    pub const POSITION: u32 = 0x110;
    /// Velocity (vec4).
    pub const VELOCITY: u32 = 0x1e0;
    /// Accumulated heading (blended toward the ground normal each frame).
    pub const HEADING: u32 = 0x5b4;
    /// Ground normal (vec4).
    pub const GROUND_NORMAL: u32 = 0x370;

    // Race / AI paths.
    pub const RACE_PATH: u32 = 0xab4;
    pub const RACE_PATH_CURSOR: u32 = 0xac0;
    pub const AI_PATH: u32 = 0xab8;
    pub const AI_PATH_CURSOR: u32 = 0xabc;
    /// Race events this frame: `{i32 a; i32 b}` records, 16 bytes each.
    pub const RACE_ENTRIES: u32 = 0x4dc;
    pub const RACE_ENTRY_COUNT: u32 = 0x59c;
    /// Distance to the finish and its estimate.
    pub const DIST_TO_FINISH: u32 = 0x4d4;
    pub const DIST_ESTIMATE: u32 = 0x4d0;

    // AI path following.
    pub const AI_POINT: u32 = 0x4a0;
    pub const AI_POINT_PREV: u32 = 0x4b0;
    pub const AI_POINT_TARGET: u32 = 0x490;
    pub const AI_ARC_LEN: u32 = 0x4c0;
    pub const AI_PROGRESS: u32 = 0x4c4;
    pub const AI_SEARCH_DIST: u32 = 0x4c8;
    pub const AI_HEADING: u32 = 0x4cc;

    // Body flags used by the visibility test.
    pub const RENDER_FLAG: u32 = 0x884;
    pub const RENDER_KIND: u32 = 0x880;
    pub const VISIBILITY_BLOCK: u32 = 0xad0;
    pub const ON_SCREEN: u32 = 0xb18;
    pub const UNK_150: u32 = 0x150;
    pub const BBOX0: u32 = 0x400;
    pub const BBOX1: u32 = 0x410;
    pub const BSPHERE: u32 = 0x420;

    // Boost timers.
    pub const DELTA_TIME: u32 = 0x300;
    pub const BOOST_A: u32 = 0x2e8;
    pub const BOOST_B: u32 = 0x2ec;
    /// Boost amount and its phase id.
    pub const BOOST: u32 = 0x2f0;
    pub const BOOST_PHASE: u32 = 0x2f4;
    /// Boost effect scale.
    pub const BOOST_SCALE: u32 = 0x2f8;
    pub const BOOST_KIND: u32 = 0x304;

    // Wind force.
    pub const IS_HUMAN: u32 = 0x874;
    pub const AI_INDEX: u32 = 0x86c;

    // Race finish.
    pub const FINISH_TIME: u32 = 0x470;
    pub const DID_NOT_FINISH: u32 = 0x480;
}

// ---------------------------------------------------------------------------
// Small register helpers.

/// The address `a + off`.
#[inline(always)]
fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
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

/// `c.lt.s a, b` leaving the condition bit.
#[inline(always)]
fn lt(c: &mut Cpu, a: FpuF32, b: FpuF32) -> bool {
    let r = a.lt(b);
    abi::set_fcc(c, r);
    r
}

/// `c.le.s a, b`.
#[inline(always)]
fn le(c: &mut Cpu, a: FpuF32, b: FpuF32) -> bool {
    let r = a.le(b);
    abi::set_fcc(c, r);
    r
}

/// `c.eq.s a, b`.
#[inline(always)]
fn eq(c: &mut Cpu, a: FpuF32, b: FpuF32) -> bool {
    let r = a.eq_s(b);
    abi::set_fcc(c, r);
    r
}

/// `x + y` of two sign-extended words (`addu`).
#[inline(always)]
fn add32(x: u64, y: u64) -> u64 {
    sx((x as u32).wrapping_add(y as u32))
}

/// `sll x, n` (32-bit, sign-extended).
#[inline(always)]
fn shl32(x: u64, n: u32) -> u64 {
    sx((x as u32) << n)
}

/// `sllv x, y` (32-bit, sign-extended).
#[inline(always)]
fn shlv32(x: u64, y: u64) -> u64 {
    sx((x as u32) << ((y as u32) & 31))
}

/// `sra x, n`.
#[inline(always)]
fn sra32(x: u64, n: u32) -> u64 {
    sx((((x as u32) as i32) >> n) as u32)
}

// ---------------------------------------------------------------------------
// Wind.

/// `void Rider_ApplyWindForce(Rider* r)` (0x125970): a human rider whose speed
/// (from the weather/AI speed model, capped at 1388.9) is at least 277.8 is
/// pushed along the weather direction (a 512-entry table at 0x504fb8) by
/// `(speed - v·dir) * dir * 0.015`, through vtable slot 0x20 of the rider base,
/// when that is ahead of the velocity.
pub fn rider_apply_wind_force(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x50);
        frame.save_ra(c, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 21, 0x68);
        frame.save_fpr(c, 20, 0x60);
        let rider = c.g[S0][0] as u32;
        let sp = sp(c);
        c.g[V0][0] = c.lw(p(rider, rider::IS_HUMAN));
        if c.g[V0][0] != 0 {
            // Speed in game units, capped at the maximum.
            c.cycles += 9;
            let max_speed = ldf(c, 21, gp(c, -0x7878));
            c.g[A0][0] = c.lw(p(rider, rider::AI_INDEX));
            call_ptr(c, callee::WORLD_PAINTER_WEATHER28, 0x001259a0)?;
            let per_ms = ldf(c, 1, gp(c, -0x7874));
            let speed = fr(c, 0) * per_ms;
            setf(c, 20, speed);
            if lt(c, max_speed, speed) {
                setf(c, 20, max_speed);
            }
            c.cycles += 5;
            let min_speed = ldf(c, 0, gp(c, -0x7870));
            if le(c, min_speed, fr(c, 20)) {
                // Wind direction for the weather's angle: (cos, sin, 0, 0).
                c.cycles += 37;
                c.g[A0][0] = c.lw(p(rider, rider::AI_INDEX));
                call_ptr(c, callee::WORLD_PAINTER_WEATHER20, 0x001259d4)?;
                let deg_to_rad = ldf(c, 1, gp(c, -0x786c));
                c.g[A0][0] = 0x0050_0000;
                let idx_scale = ldf(c, 2, gp(c, -0x7868));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4fb8));
                let rad = fr(c, 0) * deg_to_rad;
                setf(c, 0, rad);
                c.sw(p(sp, 8), 0);
                c.sw(p(sp, 0xc), 0);
                let zero = ldf(c, 3, p(sp, 8));
                let scaled = rad * idx_scale;
                setf(c, 0, scaled);
                c.cvt_w_s(1, 0);
                c.mfc1(V0, 1);
                c.g[V0][0] = c.g[V0][0] & 0x1ff;
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x80));
                c.g[V0][0] = shl32(c.g[V0][0], 2);
                c.g[V1][0] = shl32(c.g[V1][0], 2);
                c.g[V0][0] = add32(c.g[V0][0], c.g[A0][0]);
                c.g[V1][0] = add32(c.g[V1][0], c.g[A0][0]);
                let dir_y = ldf(c, 0, c.g[V0][0] as u32);
                let dir_x = ldf(c, 1, c.g[V1][0] as u32);
                c.swc1(0, p(sp, 4));
                c.swc1(1, sp);
                c.lqc2(3, p(rider, rider::VELOCITY));
                c.lqc2(5, sp);
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(V0, 4);
                c.mtc1(V0, 0);
                let along = fr(c, 20) - fr(c, 0);
                setf(c, 0, along);
                if lt(c, zero, along) {
                    // impulse = dir * d * 0.015
                    c.cycles += 23;
                    c.mfc1(V0, 0);
                    c.lqc2(4, sp);
                    c.qmtc2(V0, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, p(sp, 0x30));
                    c.g[A2][0] = sx(p(rider, rider::BASE));
                    c.lq(V0, p(sp, 0x30));
                    c.g[A1][0] = sx(p(sp, 0x10));
                    ldf(c, 0, gp(c, -0x7864));
                    c.sq(V0, p(sp, 0x20));
                    c.mfc1(V1, 0);
                    c.lqc2(4, p(sp, 0x20));
                    c.qmtc2(V1, 3);
                    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
                    c.sqc2(5, p(sp, 0x30));
                    c.lq(V0, p(sp, 0x30));
                    c.g[V1][0] = c.lw(p(rider, rider::BASE));
                    c.sq(V0, p(sp, 0x10));
                    c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x20));
                    c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x24));
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = add32(c.g[A2][0], c.g[A0][0]);
                    call_ptr(c, target, 0x00125ab8)?;
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x50);
        frame.restore_ra(c, 0x40);
        frame.restore_fpr(c, 21, 0x68);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Heading and audio.

/// `void Rider_UpdateHeadingAndAudio(Rider* r)` (0x121950): the audio hooks
/// (`SSXAudioSystem_UpdateRiderWorldPaint`), then, while the rider is on the
/// ground, blend the accumulated heading (+0x5b4) 15% of the way toward the
/// ground normal's polar angle (wrapped to (-pi, pi]); finally update the
/// danger state. The angle is `atan2(|n.xy|, n.z)`; with `n.z == 0` it is
/// +-pi/2 (0 for a degenerate normal).
pub fn rider_update_heading_and_audio(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 22, 0x40);
        frame.save_fpr(c, 21, 0x38);
        frame.save_fpr(c, 20, 0x30);
        let rider = c.g[S0][0] as u32;
        let sp = sp(c);
        call_ptr(c, callee::SSX_AUDIO_GET_INSTANCE, 0x00121970)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, callee::SSX_AUDIO_UPDATE_RIDER_WORLD_PAINT, 0x0012197c)?;
        c.g[A0][0] = c.g[S0][0];
        callee::RIDER_IS_ON_GROUND.call(c, 0x00121984)?;
        if c.g[V0][0] != 0 {
            c.cycles += 13;
            let nx = ldf(c, 0, p(rider, rider::GROUND_NORMAL));
            let ny = ldf(c, 1, p(rider, rider::GROUND_NORMAL + 4));
            let nz = ldf(c, 21, p(rider, rider::GROUND_NORMAL + 8));
            let nx2 = nx * nx;
            setf(c, 2, nx2);
            let ny2 = ny * ny;
            setf(c, 3, ny2);
            c.swc1(0, sp);
            c.mtc1(0, 22);
            c.swc1(1, p(sp, 4));
            let z_zero = eq(c, nz, fr(c, 22));
            let sum = nx2 + ny2;
            setf(c, 2, sum);
            let len = sum.sqrt();
            setf(c, 20, len);
            // Only the degenerate normal (len == 0) runs the `+=1; f0 = heading`
            // tail; both non-degenerate `n.z == 0` paths break out before it.
            let mut tail = 1u32;
            if z_zero {
                c.cycles += 4;
                if eq(c, len, fr(c, 22)) {
                    setf(c, 12, len);
                } else {
                    c.cycles += 4;
                    let pos = le(c, fr(c, 22), len);
                    let a = gp(c, -0x790c);
                    let v = ldf(c, 12, a);
                    setf(c, 12, v);
                    if pos {
                        c.cycles += 3;
                        let a = gp(c, -0x7908);
                        let v = ldf(c, 12, a);
                        setf(c, 12, v);
                    }
                    tail = 0;
                }
            } else {
                c.cycles += 7;
                c.div_s(12, 20, 21);
                call_ptr(c, callee::MATH_ATAN, 0x001219f8)?;
                let neg = lt(c, fr(c, 21), fr(c, 22));
                setf(c, 12, fr(c, 0));
                if neg {
                    c.cycles += 3;
                    if lt(c, fr(c, 22), fr(c, 20)) {
                        c.cycles += 3;
                        let pi = ldf(c, 0, gp(c, -0x7904));
                        setf(c, 12, fr(c, 12) + pi);
                    } else {
                        c.cycles += 2;
                        let pi = ldf(c, 0, gp(c, -0x7900));
                        setf(c, 12, fr(c, 12) - pi);
                    }
                }
            }
            c.cycles += tail as u64;
            let head = p(rider, rider::HEADING);
            let v = ldf(c, 0, head);
            setf(c, 0, v);
            // Wrap the difference to (-pi, pi] and blend 15%.
            c.cycles += 6;
            let pi = ldf(c, 1, gp(c, -0x78fc));
            let mut delta = fr(c, 12) - fr(c, 0);
            setf(c, 12, delta);
            let over = lt(c, pi, delta);
            setf(c, 2, fr(c, 0));
            if over {
                c.cycles += 3;
                let two_pi = ldf(c, 0, gp(c, -0x78f8));
                delta = delta - two_pi;
                setf(c, 12, delta);
            } else {
                c.cycles += 4;
                let neg_pi = ldf(c, 0, gp(c, -0x78f4));
                if lt(c, delta, neg_pi) {
                    c.cycles += 2;
                    let two_pi = ldf(c, 0, gp(c, -0x78f0));
                    delta = delta + two_pi;
                    setf(c, 12, delta);
                }
            }
            c.cycles += 4;
            let k = ldf(c, 0, gp(c, -0x78ec));
            let step = fr(c, 12) * k;
            setf(c, 0, step);
            let heading = fr(c, 2) + step;
            setf(c, 0, heading);
            c.swc1(0, p(rider, rider::HEADING));
        }
        c.cycles += 9;
        c.g[A0][0] = c.g[S0][0];
        callee::RIDER_UPDATE_DANGER_STATE.call(c, 0x00121a80)?;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 22, 0x40);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Visibility.

/// `int Rider_UpdateVisibility(Rider* r)` (0x122278): refreshes the on-screen
/// flag (+0xb18) and returns it. Riders that are not in the visible render
/// state (render flag 0, kind != 7, or the visibility-block word set) are off
/// screen. Otherwise two projection tests of the rider's bounding boxes
/// (slots 0x158/0x15c and 0x2ec of the world painter at the draw context's
/// +0x10d8) set the flag, the second against the rider handler's boxes; with no
/// handler, the frustum test at the painter's +0x2ec can clear it.
pub fn rider_update_visibility(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0xa0);
        let sp = sp(c);
        frame.save(c, S4, 0x50);
        frame.save(c, S0, 0x90);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        frame.save_ra(c, 0x40);
        let rider = c.g[S4][0] as u32;
        'exit: {
            c.g[V0][0] = c.lw(p(rider, rider::RENDER_FLAG));
            let flag_off = c.g[V0][0] == 0;
            c.g[V0][0] = 0x7;
            if flag_off {
                // b_1222bc fallen through: clear and return 0.
                c.cycles += 1;
                c.sw(p(rider, rider::ON_SCREEN), 0);
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'exit;
            }
            c.cycles += 3;
            c.g[V1][0] = c.lw(p(rider, rider::RENDER_KIND));
            if c.g[V1][0] != c.g[V0][0] {
                c.sw(p(rider, rider::ON_SCREEN), 0);
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'exit;
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw(p(rider, rider::VISIBILITY_BLOCK));
            let unblocked = c.g[V0][0] == 0;
            c.g[A1][0] = c.lw(gp(c, -0x854));
            if !unblocked {
                c.cycles += 1;
                c.sw(p(rider, rider::ON_SCREEN), 0);
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'exit;
            }
            // On-screen: projection test of the two bounding boxes.
            c.cycles += 21;
            c.g[A0][0] = c.g[SP][0];
            c.g[S2][0] = sx(p(rider, rider::BBOX0));
            c.g[S3][0] = sx(p(rider, rider::BBOX1));
            c.g[S0][0] = c.lw(p(c.g[A1][0] as u32, 0x10d8));
            c.g[S1][0] = c.lh(p(c.g[S0][0] as u32, 0x2e8));
            c.g[V0][0] = c.lh(p(c.g[S0][0] as u32, 0x158));
            c.g[V1][0] = c.lw(p(c.g[S0][0] as u32, 0x15c));
            c.g[S1][0] = add32(c.g[A1][0], c.g[S1][0]);
            let target = c.g[V1][0] as u32;
            c.g[A1][0] = add32(c.g[A1][0], c.g[V0][0]);
            call_ptr(c, target, 0x001222f0)?;
            c.g[V0][0] = c.lw(p(c.g[S0][0] as u32, 0x2ec));
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S3][0];
            let target = c.g[V0][0] as u32;
            c.g[A3][0] = c.g[SP][0];
            call_ptr(c, target, 0x00122308)?;
            c.g[V0][0] = c.g[V0][0] ^ 0x1;
            c.g[V1][0] = c.lw(p(rider, rider::UNK_150));
            c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
            let no_handler = c.g[V1][0] == 0;
            c.sw(p(rider, rider::ON_SCREEN), c.g[V0][0]);
            if !no_handler {
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    // The same test against the rider handler's boxes.
                    c.cycles += 22;
                    c.g[A1][0] = c.lw(gp(c, -0x854));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[S2][0] = c.lw(p(rider, rider::HANDLER));
                    let ctx = c.g[A1][0] as u32;
                    c.g[S1][0] = c.lw(p(ctx, 0x10d8));
                    c.g[S3][0] = add32(c.g[S2][0], 0x90);
                    c.g[S0][0] = c.lh(p(c.g[S1][0] as u32, 0x2e8));
                    c.g[S2][0] = add32(c.g[S2][0], 0x80);
                    c.g[V0][0] = c.lh(p(c.g[S1][0] as u32, 0x158));
                    c.g[S0][0] = add32(c.g[A1][0], c.g[S0][0]);
                    c.g[V1][0] = c.lw(p(c.g[S1][0] as u32, 0x15c));
                    let target = c.g[V1][0] as u32;
                    c.g[A1][0] = add32(c.g[A1][0], c.g[V0][0]);
                    call_ptr(c, target, 0x00122354)?;
                    c.g[V0][0] = c.lw(p(c.g[S1][0] as u32, 0x2ec));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A2][0] = c.g[S3][0];
                    let target = c.g[V0][0] as u32;
                    c.g[A3][0] = c.g[SP][0];
                    call_ptr(c, target, 0x0012236c)?;
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
                    c.sw(p(rider, rider::ON_SCREEN), c.g[V0][0]);
                    c.g[V0][0] = c.lw(p(rider, rider::UNK_150));
                }
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = c.lw(p(rider, rider::ON_SCREEN));
                    break 'exit;
                }
            }
            // No handler: the frustum test can clear the flag. It walks the
            // world's query list (race manager +0x8c, entries of 8 bytes,
            // count at +0x10c) skipping unset bits of the +0x110 mask.
            let mut hidden = 0u64;
            'frustum: {
                'list: {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw(gp(c, -0x848));
                    c.g[T1][0] = sx(p(rider, rider::BSPHERE));
                    c.g[A0][0] = 0;
                    c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x84));
                    c.g[A2][0] = c.lw(p(c.g[V1][0] as u32, 0x8c));
                    c.g[V0][0] = c.lw(p(c.g[A2][0] as u32, 0x10c));
                    let empty = (c.g[V0][0] as i64) <= 0;
                    c.g[T0][0] = c.lw(p(c.g[A2][0] as u32, 0x110));
                    if empty {
                        break 'list;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = 0;
                    c.g[T2][0] = 1;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x828));
                    c.g[A1][0] = add32(c.g[A2][0], 0x8);
                    c.g[A3][0] = sx(((c.g[V0][0] as u32) as i32 >> 3) as u32);
                    loop {
                        c.cycles += 4;
                        c.g[V0][0] = shlv32(c.g[T2][0], c.g[A0][0]);
                        c.g[V0][0] = c.g[T0][0] & c.g[V0][0];
                        if c.g[V0][0] == 0 {
                            c.g[V0][0] = c.lw(p(c.g[A2][0] as u32, 0x10c));
                        } else {
                            c.cycles += 15;
                            c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                            c.lqc2(1, p(c.g[V0][0] as u32, 0x50));
                            c.lqc2(2, p(c.g[V0][0] as u32, 0x60));
                            c.lqc2(3, p(c.g[V0][0] as u32, 0x70));
                            c.lqc2(4, p(c.g[V0][0] as u32, 0x80));
                            c.lqc2(5, p(c.g[V0][0] as u32, 0x90));
                            c.lqc2(6, c.g[T1][0] as u32);
                            c.ctc2(7, 27);
                            c.vu0().vnop();
                            c.vu0().vnop();
                            c.vcallmsr();
                            c.cfc2(3, 1);
                            c.g[V1][0] = c.g[V1][0] ^ 0x1;
                            let done = c.g[V1][0] == 0;
                            c.g[V0][0] = 1;
                            if done {
                                hidden = 1;
                                break 'frustum;
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.lw(p(c.g[A2][0] as u32, 0x10c));
                        }
                        c.cycles += 4;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[A1][0] = add32(c.g[A1][0], 8);
                        if !more {
                            // (Out of the list the way an empty one leaves:
                            // b_122418's cycle.)
                            break 'list;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x001223b8);
                        }
                    }
                }
                c.cycles += 1;
            }
            c.cycles += 2;
            if hidden != 0 {
                c.sw(p(rider, rider::ON_SCREEN), 0);
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw(p(rider, rider::ON_SCREEN));
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Boost timers.

/// `void Rider_UpdateBoostTimers(Rider* r)` (0x1200d0): decays the two boost
/// timers (+0x2e8/+0x2ec) toward zero by the frame's `dt`, then drives the
/// boost chain: the rider base's vtable slot 0x48 runs first; a boost amount
/// (+0x2f0) below one frame is raised to one frame while in the air (main state
/// 1 or sub state 12) or else decayed, firing timed event 6 when it reaches 0;
/// phase 10 restarts the boost at 20 frames once the amount runs out; otherwise
/// the effect scale (+0x2f8) is reset (kind 0) or decayed (kinds 1 and 2,
/// unless the rider is in sub state 6).
pub fn rider_update_boost_timers(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        let sp = sp(c);
        frame.save(c, S0, 0x10);
        frame.save_fpr(c, 20, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let rider = c.g[S0][0] as u32;
        // --- decay(+0x2e8, step)
        let per_frame = ldf(c, 1, gp(c, -0x7958));
        let delta = ldf(c, 0, p(rider, rider::DELTA_TIME));
        let a = ldf(c, 2, p(rider, rider::BOOST_A));
        let step = delta * per_frame;
        setf(c, 20, step);
        if lt(c, step, a) {
            c.cycles += 2;
            setf(c, 3, a - step);
        } else {
            setf(c, 0, -step);
            c.cycles += 5;
            setf(c, 3, FpuF32::ZERO);
            if lt(c, a, fr(c, 0)) {
                setf(c, 3, a + step);
            }
        }
        // --- decay(+0x2ec, step), unless the timer is already past one frame
        // and the rider is in main state 1.
        c.cycles += 6;
        let b = ldf(c, 1, p(rider, rider::BOOST_B));
        let one60 = ldf(c, 0, gp(c, -0x7954));
        let over = lt(c, one60, b);
        c.swc1(3, p(rider, rider::BOOST_A));
        let mut vt: u64 = 0;
        'decay_b: {
            if !over {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                callee::RIDER_GET_MAIN_STATE.call(c, 0x00120140)?;
                c.g[V1][0] = 1;
                if c.g[V0][0] == c.g[V1][0] {
                    vt = c.lw(p(rider, rider::BASE));
                    c.g[V1][0] = sx(vt as u32);
                    break 'decay_b;
                }
            }
            c.cycles += 5;
            let x = ldf(c, 2, p(rider, rider::BOOST_B));
            if lt(c, step, x) {
                c.cycles += 2;
                setf(c, 1, x - step);
            } else {
                c.cycles += 5;
                setf(c, 0, -step);
                setf(c, 1, FpuF32::ZERO);
                if lt(c, x, fr(c, 0)) {
                    setf(c, 1, x + step);
                }
            }
            c.cycles += 2;
            c.swc1(1, p(rider, rider::BOOST_B));
            vt = c.lw(p(rider, rider::BASE));
            c.g[V1][0] = sx(vt as u32);
        }
        // --- the rider base's per-frame hook
        c.cycles += 10;
        c.g[A1][0] = sx(p(rider, rider::BASE));
        let vt = vt as u32;
        c.g[A0][0] = c.lh(p(vt, 0x48));
        c.g[V0][0] = c.lw(p(vt, 0x4c));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = add32(c.g[A1][0], c.g[A0][0]);
        call_ptr(c, target, 0x00120198)?;
        c.g[A0][0] = c.g[S0][0];
        callee::RIDER_GET_MAIN_STATE.call(c, 0x001201a0)?;
        c.g[V1][0] = 1;
        // 0: the clamp block's give-up / 1: the clamp succeeded
        let mut give_up = false;
        if c.g[V0][0] == c.g[V1][0] {
            ldf(c, 1, p(rider, rider::BOOST));
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            callee::RIDER_GET_SUB_STATE.call(c, 0x001201b4)?;
            c.g[V1][0] = 0xc;
            if c.g[V0][0] != c.g[V1][0] {
                ldf(c, 2, p(rider, rider::BOOST));
                give_up = true;
            } else {
                c.cycles += 1;
                ldf(c, 1, p(rider, rider::BOOST));
            }
        }
        // The one-frame clamp: 0 < t <= 1/60 -> t = 1/60.
        if !give_up {
            c.cycles += 5;
            setf(c, 0, FpuF32::ZERO);
            if lt(c, fr(c, 0), fr(c, 1)) {
                c.cycles += 5;
                let one_frame = ldf(c, 0, gp(c, -0x7950));
                if le(c, fr(c, 1), one_frame) {
                    c.cycles += 2;
                    c.swc1(0, p(rider, rider::BOOST));
                    c.cycles += 1;
                    c.g[V1][0] = c.lw(p(rider, rider::BOOST_PHASE));
                } else {
                    ldf(c, 2, p(rider, rider::BOOST));
                    give_up = true;
                }
            } else {
                ldf(c, 2, p(rider, rider::BOOST));
                give_up = true;
            }
        }
        if give_up {
            // b_1201f4: decay the amount; at zero fire the timed event.
            c.cycles += 5;
            c.mtc1(0, 1);
            if lt(c, fr(c, 1), fr(c, 2)) {
                c.cycles += 4;
                if lt(c, step, fr(c, 2)) {
                    c.cycles += 2;
                    setf(c, 1, fr(c, 2) - step);
                } else {
                    c.cycles += 4;
                    setf(c, 0, -step);
                    setf(c, 1, FpuF32::ZERO);
                    if lt(c, fr(c, 2), fr(c, 0)) {
                        setf(c, 1, fr(c, 2) + step);
                    }
                }
                c.cycles += 5;
                c.mtc1(0, 12);
                let zero = eq(c, fr(c, 1), fr(c, 12));
                c.swc1(1, p(rider, rider::BOOST));
                if zero {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x6;
                    callee::RIDER_SET_TIMED_EVENT.call(c, 0x00120250)?;
                }
                c.cycles += 1;
                c.g[V1][0] = c.lw(p(rider, rider::BOOST_PHASE));
            } else {
                c.g[V1][0] = c.lw(p(rider, rider::BOOST_PHASE));
            }
        }
        // --- b_120368: phase 10 restarts the boost, else the kind switch.
        c.cycles += 3;
        c.g[V0][0] = 0xa;
        if c.g[V1][0] == c.g[V0][0] {
            c.cycles += 8;
            let amount = ldf(c, 0, p(rider, rider::BOOST));
            c.mtc1(0, 1);
            setf(c, 2, FpuF32::ONE);
            let spent = eq(c, amount, fr(c, 1));
            c.swc1(2, p(rider, rider::BOOST_SCALE));
            if spent {
                c.cycles += 6;
                c.g[AT][0] = 0x41a0_0000;
                c.mtc1(1, 0);
                c.g[V0][0] = 5;
                c.sw(p(rider, rider::BOOST_PHASE), c.g[V0][0]);
                c.swc1(0, p(rider, rider::BOOST));
            }
            c.cycles += 1;
            c.lq(16, p(sp, 0x10));
        } else {
            c.g[V1][0] = sx(c.lw(p(rider, rider::BOOST_KIND)) as u32);
            c.cycles += 3;
            c.g[V0][0] = 0x1;
            let kind = c.g[V1][0];
            c.g[V0][0] = ((kind as i64) < 2) as u64;
            if kind == 1 {
                // break 'b_1202dc: kind 1 decays the scale by step * 0.005
                // unless the rider is in sub state 6.
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                callee::RIDER_GET_SUB_STATE.call(c, 0x001202e4)?;
                c.g[V1][0] = 6;
                if c.g[V0][0] == c.g[V1][0] {
                    c.lq(16, p(sp, 0x10));
                } else {
                    c.cycles += 7;
                    let rate = ldf(c, 0, gp(c, -0x794c));
                    let scale = ldf(c, 3, p(rider, rider::BOOST_SCALE));
                    let dec = step * rate;
                    setf(c, 2, dec);
                    if lt(c, dec, scale) {
                        setf(c, 1, scale - dec);
                    } else {
                        c.cycles += 2;
                        setf(c, 0, -dec);
                        c.cycles += 5;
                        setf(c, 1, FpuF32::ZERO);
                        if lt(c, scale, fr(c, 0)) {
                            setf(c, 1, scale + dec);
                        }
                    }
                    c.cycles += 1;
                    c.swc1(1, p(rider, rider::BOOST_SCALE));
                    // `break 'b_120364` lands on the outer `+=1; lq16` block.
                    c.cycles += 1;
                    c.lq(16, p(sp, 0x10));
                }
            } else {
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    // kind >= 2: `v0 = 2; break 'b_1202bc`
                    c.g[V0][0] = 0x2;
                    c.cycles += 2;
                    if kind == 2 {
                        // break 'b_120314: kind 2 decays by step * 0.024.
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S0][0];
                        callee::RIDER_GET_SUB_STATE.call(c, 0x0012031c)?;
                        c.g[V1][0] = 6;
                        if c.g[V0][0] == c.g[V1][0] {
                            c.lq(16, p(sp, 0x10));
                        } else {
                            c.cycles += 7;
                            let rate = ldf(c, 0, gp(c, -0x7948));
                            let scale = ldf(c, 3, p(rider, rider::BOOST_SCALE));
                            let dec = step * rate;
                            setf(c, 2, dec);
                            if lt(c, dec, scale) {
                                c.cycles += 2;
                                setf(c, 1, scale - dec);
                            } else {
                                setf(c, 0, -dec);
                                c.cycles += 5;
                                setf(c, 1, FpuF32::ZERO);
                                if lt(c, scale, fr(c, 0)) {
                                    setf(c, 1, scale + dec);
                                }
                            }
                            c.cycles += 1;
                            c.swc1(1, p(rider, rider::BOOST_SCALE));
                            c.cycles += 1;
                            c.lq(16, p(sp, 0x10));
                        }
                    } else {
                        // Unknown kind: nothing to do.
                        c.cycles += 2;
                        c.lq(16, p(sp, 0x10));
                    }
                } else {
                    // kind 0: `break 'b_1202cc` resets the scale to 1
                    // (`lui at, 0x3f80; mtc1 at, f0` — the value goes to f0).
                    c.cycles += 2;
                    c.cycles += 4;
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 0);
                    c.swc1(0, p(rider, rider::BOOST_SCALE));
                    c.cycles += 1;
                    c.lq(16, p(sp, 0x10));
                }
            }
        }
        c.cycles += 4;
        c.g[RA][0] = c.ld(sp);
        c.lwc1(20, p(sp, 0x20));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp.wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Race finish.

/// `int Rider_CheckRaceFinish(Rider* r)` (0x125228): returns whether the rider
/// has finished (finish time >= 0). For a human who has not finished: when the
/// BE race state is not 4 and the race timer is running and the game's DNF
/// flag is clear, a rider whose remaining time (the entity's +0x78 scaled by
/// 1/60) is less than the race clock is marked did-not-finish, told
/// `Rider_OnFinishRace` and the arcade announcer line 2 plays. A DNF rider in
/// race state 4 is also finished.
pub fn rider_check_race_finish(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        let sp = sp(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(p(c.g[S0][0] as u32, rider::BASE));
        frame.save_fpr(c, 20, 0x30);
        let rider = c.g[S0][0] as u32;
        c.g[V1][0] = c.lw(p(rider, rider::BASE));
        c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x40));
        c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x44));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = add32(c.g[A1][0], c.g[A0][0]);
        call_ptr(c, target, 0x00125258)?;
        // 1: from b_12539c (v0 = 1), 2: from b_1253a0 (v0 = 1), 3: b_1253ac
        let mut tail = 2u32;
        let mut finish = FpuF32::ZERO;
        'body: {
            if c.g[V0][0] == 0 {
                finish = ldf(c, 1, p(rider, rider::FINISH_TIME));
                tail = 1;
                break 'body;
            }
            // done = finish >= 0
            c.cycles += 6;
            let h = ldf(c, 0, p(rider, rider::FINISH_TIME));
            c.mtc1(0, 20);
            let already = le(c, fr(c, 20), h);
            c.g[V0][0] = 1;
            if !already {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                finish = ldf(c, 1, p(rider, rider::FINISH_TIME));
                tail = 1;
                break 'body;
            }
            // Is the BE race state out of the running phase?
            c.cycles += 10;
            call_ptr(c, callee::BELIBRARY_GET_INSTANCE, 0x0012528c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, callee::BELIBRARY_GET_INTERFACE, 0x00125298)?;
            c.g[V1][0] = 0x0053_0000;
            c.g[A0][0] = 4;
            c.g[V0][0] = c.lb(p(c.g[V1][0] as u32, 0x5c10));
            let state4 = c.g[V0][0] == c.g[A0][0];
            c.g[A0][0] = c.lw(gp(c, -0x848));
            if state4 {
                c.cycles += 4;
                let h = ldf(c, 0, p(rider, rider::FINISH_TIME));
                le(c, fr(c, 20), h);
                c.g[V0][0] = 1;
                tail = 3;
                break 'body;
            }
            // The did-not-finish check.
            c.cycles += 4;
            c.g[A1][0] = c.lw(p(c.g[A0][0] as u32, 0xc0));
            c.g[V0][0] = c.lw(p(c.g[A1][0] as u32, 0x88));
            let no_timer = c.g[V0][0] == 0;
            c.g[V1][0] = 0x0053_0000;
            // The bit-set path jumps straight to the shared tail (v1 already
            // loaded there); the others run the tail's prologue.
            let mut tail_charge = 1u32;
            if !no_timer {
                c.cycles += 5;
                c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x8d0));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 9) as u32);
                c.g[V0][0] = c.g[V0][0] & 0x1;
                if c.g[V0][0] == 0 {
                    // Seconds left vs. the race clock.
                    c.cycles += 5;
                    c.g[V1][0] = c.lw(p(c.g[A0][0] as u32, 0x84));
                    c.g[A0][0] = c.lw(p(c.g[A1][0] as u32, 0x78));
                    c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0xc));
                    let negative = (c.g[A0][0] as i64) < 0;
                    c.g[S1][0] = c.lw(p(c.g[V0][0] as u32, 0xc));
                    if !negative {
                        c.cycles += 4;
                        c.mtc1(4, 0);
                        c.cvt_s_w(0, 0);
                    } else {
                        c.cycles += 6;
                        c.g[V1][0] = sx((c.g[A0][0] as u32) >> 1);
                        c.g[V0][0] = c.g[A0][0] & 0x1;
                        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                        c.mtc1(2, 0);
                        c.cvt_s_w(0, 0);
                        setf(c, 0, fr(c, 0) + fr(c, 0));
                    }
                    c.cycles += 8;
                    let per_sec = ldf(c, 12, gp(c, -0x787c));
                    let t = fr(c, 0) * per_sec;
                    setf(c, 12, t);
                    call_ptr(c, callee::FIXUNSSFSI, 0x00125328)?;
                    c.g[V1][0] = 0x3c;
                    c.mult(0, V0, V0, V1);
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    let keep_going = c.g[V0][0] == 0;
                    c.g[V0][0] = 1;
                    if !keep_going {
                        c.cycles += 11;
                        c.g[A0][0] = c.g[S0][0];
                        c.sw(p(rider, rider::DID_NOT_FINISH), c.g[V0][0]);
                        callee::RIDER_ON_FINISH_RACE.call(c, 0x00125348)?;
                        call_ptr(c, callee::SSX_AUDIO_GET_INSTANCE, 0x00125350)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A2][0] = 2;
                        call_ptr(c, callee::SSX_AUDIO_ARCADE_SAY, 0x00125360)?;
                        finish = ldf(c, 1, p(rider, rider::FINISH_TIME));
                        tail = 1;
                        break 'body;
                    }
                } else {
                    tail_charge = 0;
                }
            }
            // A DNF rider whose race state is 4 also finishes.
            c.cycles += tail_charge as u64;
            c.g[V1][0] = c.lw(p(rider, rider::DID_NOT_FINISH));
            c.cycles += 3;
            c.g[V0][0] = 1;
            if c.g[V1][0] != c.g[V0][0] {
                // `break 'b_1253a0`: skips the `+=1; v0 = 1` block.
                tail = 2;
            } else {
                c.cycles += 6;
                c.g[A0][0] = c.lw(gp(c, -0x848));
                c.g[A1][0] = 4;
                c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 0x84));
                c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x214));
                if c.g[V1][0] == c.g[A1][0] {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    callee::RIDER_ON_FINISH_RACE.call(c, 0x00125398)?;
                }
                // Both exits of the DNF block are `break 'b_12539c`.
                tail = 1;
            }
            finish = ldf(c, 1, p(rider, rider::FINISH_TIME));
        }
        if tail == 1 {
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
        if tail != 3 {
            c.cycles += 3;
            setf(c, 0, FpuF32::ZERO);
            le(c, FpuF32::ZERO, finish);
        }
        c.cycles += 2;
        if !c.fcc() {
            c.g[V0][0] = 0;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        let _ = sp;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Race progress.

/// The race-event record list shared by the riders (`D_004A5D60`).
const RACE_EVENTS: u32 = 0x004a_5d60;

/// `void Rider_UpdateRaceProgress(Rider* r)` (0x112338): compares the shared
/// race-event list with the rider's snapshot, refreshes the distance to the
/// finish, picks a new race path when the current one is 500 units behind, ends
/// within 200 units, or its event test reports one, then tells the race-event
/// handler about every entry (matched ones with `found`, the unmatched shared
/// list with 1,0).
pub fn rider_update_race_progress(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0xa0);
        let sp = sp(c);
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        frame.save(c, S3, 0x60);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x50);
        c.g[A0][0] = 0x004a_0000;
        frame.save(c, S0, 0x90);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5d60));
        frame.save(c, S5, 0x40);
        c.g[A1][0] = sx(p(c.g[S2][0] as u32, rider::RACE_ENTRIES));
        frame.save(c, S6, 0x30);
        frame.save_ra(c, 0x20);
        c.g[A2][0] = c.lw(p(c.g[S2][0] as u32, rider::RACE_ENTRY_COUNT));
        c.g[A2][0] = shl32(c.g[A2][0], 4);
        call_ptr(c, callee::EA_MEM_COPY, 0x00112378)?;
        let rider = c.g[S2][0] as u32;
        c.g[A1][0] = sx(p(sp, 0x10));
        c.g[A2][0] = sx(p(sp, 0x14));
        c.g[S3][0] = c.lw(p(rider, rider::RACE_ENTRY_COUNT));
        c.g[S4][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[A3][0] = c.g[SP][0];
        callee::RIDER_UPDATE_DIST_TO_FINISH.call(c, 0x00112398)?;
        let df = ldf(c, 1, p(rider, rider::DIST_TO_FINISH));
        setf(c, 0, FpuF32::ZERO);
        let has_dist = lt(c, FpuF32::ZERO, df);
        ldf(c, 1, p(sp, 0x10));
        if has_dist {
            // --- the path selection
            let mut new_path = 0u64;
            'select: {
                c.cycles += 6;
                c.g[AT][0] = 0x43fa_0000;
                c.mtc1(1, 0);
                let far = lt(c, fr(c, 0), fr(c, 1));
                c.g[S0][0] = c.lw(p(rider, rider::RACE_PATH));
                if far {
                    // The race clock modulo 60 check.
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(gp(c, -0x848));
                    c.g[A1][0] = 0x3c;
                    c.cycles += 7;
                    c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x84));
                    c.g[A0][0] = c.lw(p(c.g[V1][0] as u32, 0xc));
                    c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 8));
                    c.div(0, 2, 5);
                    c.g[V1][0] = c.hi[0];
                    let not_minute = c.g[V1][0] != 0;
                    c.g[A0][0] = c.g[S2][0];
                    if !not_minute {
                        c.cycles += 4;
                        c.g[A1][0] = 0;
                        callee::RIDER_SELECT_RACE_PATH.call(c, 0x001123fc)?;
                        new_path = c.lw(p(rider, rider::RACE_PATH));
                        break 'select;
                    }
                }
                // b_112440: the path's own end check.
                c.cycles += 10;
                c.g[A0][0] = c.lw(p(rider, rider::RACE_PATH));
                call_ptr(c, callee::PATH_GET_LENGTH, 0x0011240c)?;
                c.g[AT][0] = 0x4348_0000;
                c.mtc1(1, 1);
                ldf(c, 2, p(sp, 0x14));
                let left = fr(c, 0) - fr(c, 1);
                setf(c, 0, left);
                if !lt(c, left, fr(c, 2)) {
                    c.g[A1][0] = c.lw(p(rider, rider::RACE_PATH));
                    // fall into b_11246c
                } else {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = 1;
                    callee::RIDER_SELECT_RACE_PATH.call(c, 0x00112438)?;
                    new_path = c.lw(p(rider, rider::RACE_PATH));
                    break 'select;
                }
                // b_11246c: the path's event test.
                c.cycles += 9;
                ldf(c, 12, p(rider, rider::DIST_TO_FINISH));
                c.g[V1][0] = c.lw(p(c.g[A1][0] as u32, 0x34));
                ldf(c, 13, p(rider, rider::DIST_ESTIMATE));
                c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x10));
                c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x14));
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = add32(c.g[A1][0], c.g[A0][0]);
                call_ptr(c, target, 0x0011245c)?;
                let no_event = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S2][0];
                if !no_event {
                    c.cycles += 2;
                    c.g[A1][0] = 0;
                    callee::RIDER_SELECT_RACE_PATH.call(c, 0x0011246c)?;
                }
                c.cycles += 1;
                new_path = c.lw(p(rider, rider::RACE_PATH));
            }
            // The path changed: reset the new path's cursor and re-measure.
            c.cycles += 2;
            let same = new_path == c.g[S0][0];
            c.g[V0][0] = u64::MAX;
            if !same {
                c.cycles += 7;
                c.g[V1][0] = c.lw(p(rider, rider::RACE_PATH_CURSOR));
                c.g[A1][0] = c.g[S4][0];
                c.g[A2][0] = c.g[S1][0];
                c.sw(p(c.g[V1][0] as u32, 0x14), c.g[V0][0]);
                c.g[A0][0] = c.g[S2][0];
                c.g[A3][0] = c.g[SP][0];
                callee::RIDER_UPDATE_DIST_TO_FINISH.call(c, 0x00112494)?;
            }
        }
        // --- the event loops
        c.cycles += 3;
        c.g[V0][0] = c.lw(p(rider, rider::RACE_ENTRY_COUNT));
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[S0][0] = 0;
        if !none {
            c.cycles += 4;
            c.g[S5][0] = sx(p(rider, rider::RACE_ENTRIES));
            c.g[S6][0] = 0x004a_0000;
            c.g[S4][0] = sx(p(rider, rider::RACE_ENTRIES + 4));
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            loop {
                c.cycles += 3;
                c.g[A2][0] = 0;
                let no_path = (c.g[S3][0] as i64) <= 0;
                c.g[A1][0] = shl32(c.g[S0][0], 4);
                if !no_path {
                    c.cycles += 3;
                    c.g[T0][0] = add32(c.g[S5][0], c.g[A1][0]);
                    c.g[A3][0] = add32(c.g[S4][0], c.g[A1][0]);
                    c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x5d60));
                    loop {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw(c.g[T0][0] as u32);
                        c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                        let mut matched = false;
                        if c.g[V1][0] != c.g[V0][0] {
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                        } else {
                            c.cycles += 4;
                            c.g[V1][0] = c.lw(c.g[A3][0] as u32);
                            c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 4));
                            if c.g[V1][0] != c.g[V0][0] {
                                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                            } else {
                                c.cycles += 2;
                                c.sw(c.g[A0][0] as u32, 0);
                                matched = true;
                            }
                        }
                        if matched {
                            break;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[S3][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x001124c8);
                        }
                    }
                }
                c.cycles += 11;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4dc));
                c.g[A2][0] = ((c.g[A2][0] as i64) < (c.g[S3][0] as i64)) as u64;
                c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[A0][0] = c.g[S2][0];
                c.g[A3][0] = 0x1;
                c.g[S0][0] = c.g[S1][0];
                callee::RIDER_HANDLE_RACE_PATH_EVENT.call(c, 0x00112518)?;
                c.g[V0][0] = c.lw(p(rider, rider::RACE_ENTRY_COUNT));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x001124b0);
                }
            }
        }
        // --- the leftover shared list
        c.cycles += 2;
        let none2 = (c.g[S3][0] as i64) <= 0;
        c.g[V0][0] = 0x004a_0000;
        if !none2 {
            c.cycles += 2;
            c.g[S0][0] = c.g[S3][0];
            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5d60));
            loop {
                c.cycles += 3;
                c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                let empty = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S2][0];
                if !empty {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = 1;
                    c.g[A3][0] = 0;
                    callee::RIDER_HANDLE_RACE_PATH_EVENT.call(c, 0x00112554)?;
                }
                c.cycles += 3;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                let more = c.g[S0][0] != 0;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x00112538);
                }
            }
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// AI path progress.

/// `void Rider_UpdateAIPathProgress(Rider* r)` (0x1125c0): keeps the AI path
/// progress (arc length along the AI path, +0x4c4) and its look-ahead point
/// current, selects a new AI path when the current one is 500 units behind,
/// ends within 200 units or its event test fires, then computes the AI heading
/// (+0x4cc) from the look-ahead point's offset (`atan2(dy, dx)`, wrapped to
/// (-pi, pi]).
pub fn rider_update_ai_path_progress(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        let sp = sp(c);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        frame.save_fpr(c, 22, 0x50);
        frame.save_fpr(c, 21, 0x48);
        frame.save_fpr(c, 20, 0x40);
        let rider = c.g[S0][0] as u32;
        ldf(c, 0, p(rider, rider::AI_PROGRESS));
        c.swc1(0, p(rider, rider::AI_ARC_LEN));
        callee::RIDER_RETURN_ZERO.call(c, 0x001125ec)?;
        c.g[S1][0] = c.g[V0][0] ^ 0x1;
        let no_search = c.g[S1][0] == 0;
        c.g[A2][0] = sx(p(rider, rider::AI_POINT_TARGET));
        if !no_search {
            c.cycles += 8;
            c.g[A0][0] = c.lw(p(rider, rider::AI_PATH));
            c.g[A1][0] = sx(p(rider, rider::POSITION));
            c.g[T0][0] = c.lw(p(rider, rider::AI_PATH_CURSOR));
            c.g[A3][0] = sx(p(rider, rider::AI_SEARCH_DIST));
            c.g[T1][0] = c.g[S1][0];
            call_ptr(c, callee::PATH_GET_CLOSEST_ON_PATH, 0x00112610)?;
            c.swc1(0, p(rider, rider::AI_PROGRESS));
        } else {
            c.cycles += 6;
            c.g[A0][0] = c.lw(p(rider, rider::AI_PATH));
            c.g[A1][0] = sx(p(rider, rider::POSITION));
            c.g[A3][0] = sx(p(rider, rider::AI_SEARCH_DIST));
            c.g[T0][0] = 0;
            call_ptr(c, callee::PATH_GET_CLOSEST, 0x0011262c)?;
            c.swc1(0, p(rider, rider::AI_PROGRESS));
        }
        // b_11273c: the look-ahead point.
        c.cycles += 20;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        callee::RIDER_AI_LOOK_AHEAD.call(c, 0x0011263c)?;
        let progress = ldf(c, 12, p(rider, rider::AI_PROGRESS));
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = c.lw(p(rider, rider::AI_PATH));
        c.g[A2][0] = 0;
        setf(c, 12, progress + fr(c, 0));
        call_ptr(c, callee::PATH_GET_POINT_AT_DISTANCE, 0x00112654)?;
        c.lq(2, c.g[SP][0] as u32);
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.sq(2, p(rider, rider::AI_POINT));
        callee::RIDER_SAVE_AI_LOOK_AHEAD.call(c, 0x00112668)?;
        ldf(c, 1, p(rider, rider::DIST_TO_FINISH));
        c.mtc1(0, 0);
        let has_dist = lt(c, fr(c, 0), fr(c, 1));
        if has_dist {
            // Re-select the AI path when it ends too soon.
            let mut selected = false;
            'select: {
                c.cycles += 7;
                ldf(c, 1, p(rider, rider::AI_SEARCH_DIST));
                c.g[AT][0] = 0x43fa_0000;
                c.mtc1(1, 0);
                let far = lt(c, fr(c, 0), fr(c, 1));
                c.g[V0][0] = c.lw(gp(c, -0x848));
                // (The generated body runs the minute check when
                // 1000 < search_dist, i.e. when far.)
                if far {
                    // The race clock modulo 60 check.
                    c.cycles += 3;
                    c.g[A1][0] = 0x3c;
                    c.cycles += 7;
                    c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 0x84));
                    c.g[A0][0] = c.lw(p(c.g[V1][0] as u32, 0xc));
                    c.g[V0][0] = c.lw(p(c.g[A0][0] as u32, 8));
                    c.div(0, 2, 5);
                    c.g[V1][0] = c.hi[0];
                    let not_minute = c.g[V1][0] != 0;
                    c.g[A0][0] = c.g[S0][0];
                    if !not_minute {
                        c.cycles += 4;
                        c.g[A1][0] = 0;
                        callee::RIDER_SELECT_AI_PATH.call(c, 0x001126cc)?;
                        ldf(c, 0, p(rider, rider::AI_POINT_TARGET));
                        selected = true;
                        break 'select;
                    }
                }
                // b_112710: the path's own end check.
                c.cycles += 10;
                c.g[A0][0] = c.lw(p(rider, rider::AI_PATH));
                call_ptr(c, callee::PATH_GET_LENGTH, 0x001126dc)?;
                c.g[AT][0] = 0x4348_0000;
                c.mtc1(1, 1);
                ldf(c, 13, p(rider, rider::AI_PROGRESS));
                let left = fr(c, 0) - fr(c, 1);
                setf(c, 0, left);
                if lt(c, left, fr(c, 13)) {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 1;
                    callee::RIDER_SELECT_AI_PATH.call(c, 0x00112708)?;
                    ldf(c, 0, p(rider, rider::AI_POINT_TARGET));
                    selected = true;
                    break 'select;
                }
                c.g[A1][0] = c.lw(p(rider, rider::AI_PATH));
            }
            if !selected {
                // The path's event test: vtable slot 2 of the path.
                c.cycles += 8;
                ldf(c, 12, p(rider, rider::AI_ARC_LEN));
                c.g[V1][0] = c.lw(p(c.g[A1][0] as u32, 0x34));
                c.g[A0][0] = c.lh(p(c.g[V1][0] as u32, 0x10));
                c.g[V0][0] = c.lw(p(c.g[V1][0] as u32, 0x14));
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = add32(c.g[A1][0], c.g[A0][0]);
                call_ptr(c, target, 0x00112728)?;
                let no_event = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S0][0];
                if !no_event {
                    c.cycles += 2;
                    c.g[A1][0] = 0;
                    callee::RIDER_SELECT_AI_PATH.call(c, 0x00112738)?;
                }
                c.cycles += 1;
                ldf(c, 0, p(rider, rider::AI_POINT_TARGET));
            }
        } else {
            ldf(c, 0, p(rider, rider::AI_POINT_TARGET));
        }
        // b_1127d0: the AI heading from the look-ahead offset.
        'heading: {
            c.cycles += 9;
            let point = ldf(c, 2, p(rider, rider::AI_POINT));
            c.mtc1(0, 22);
            let dx = point - fr(c, 0);
            setf(c, 21, dx);
            let dy_raw = ldf(c, 1, p(rider, rider::AI_POINT + 4));
            let base_y = ldf(c, 0, p(rider, rider::AI_POINT_TARGET + 4));
            let dy = dy_raw - base_y;
            setf(c, 20, dy);
            let dx_zero = eq(c, fr(c, 21), fr(c, 22));
            if dx_zero {
                c.cycles += 4;
                let dy_zero = eq(c, fr(c, 20), fr(c, 22));
                setf(c, 0, fr(c, 20));
                if !dy_zero {
                    c.cycles += 4;
                    if !le(c, fr(c, 22), fr(c, 20)) {
                        let a = gp(c, -0x7c90);
                        let v = ldf(c, 0, a);
                        setf(c, 20, v);
                        c.swc1(20, p(rider, rider::AI_HEADING));
                        break 'heading;
                    }
                    c.cycles += 3;
                    let a = gp(c, -0x7c8c);
                    let v = ldf(c, 0, a);
                    setf(c, 20, v);
                    c.swc1(20, p(rider, rider::AI_HEADING));
                    break 'heading;
                }
            } else {
                c.cycles += 7;
                c.div_s(12, 20, 21);
                call_ptr(c, callee::MATH_ATAN, 0x00112798)?;
                let neg = lt(c, fr(c, 21), fr(c, 22));
                setf(c, 1, fr(c, 0));
                if neg {
                    c.cycles += 3;
                    if lt(c, fr(c, 22), fr(c, 20)) {
                        c.cycles += 3;
                        let pi = ldf(c, 0, gp(c, -0x7c88));
                        setf(c, 1, fr(c, 1) + pi);
                    } else {
                        c.cycles += 2;
                        let pi = ldf(c, 0, gp(c, -0x7c84));
                        setf(c, 1, fr(c, 1) - pi);
                    }
                }
                c.cycles += 1;
                setf(c, 0, fr(c, 1));
            }
            c.cycles += 1;
            c.swc1(0, p(rider, rider::AI_HEADING));
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 22, 0x50);
        frame.restore_fpr(c, 21, 0x48);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, put_vec, DATA, GAME_GP};

    /// The rider object all tests point `a0` at.
    const RIDER: u32 = DATA;
    /// A scratch object after the rider fields.
    const OBJ: u32 = DATA + 0x4000;
    /// A vtable page.
    const VT: u32 = DATA + 0x8000;

    /// A float at `gp + off` (the game's `.rodata` constants).
    fn wf(c: &mut Cpu, off: i32, v: f32) {
        c.bus.write32(GAME_GP.wrapping_add(off as u32), v.to_bits());
    }

    fn w(c: &mut Cpu, at: u32, v: u32) {
        c.bus.write32(at, v);
    }

    /// A vtable slot `{ short delta; short; fn }` at `vt + off`.
    fn slot(c: &mut Cpu, vt: u32, off: u32, delta: i16) {
        c.bus.write16(vt + off, delta as u16);
        c.bus.write32(vt + off + 4, fakes::FAKE);
    }

    /// The `Math_Atan` polynomial constants.
    fn atan_constants(c: &mut Cpu) {
        wf(c, 0xdf0, 0.167_702_27);
        wf(c, 0xdf4, 0.067_658_5);
        wf(c, 0xdf8, 0.052_005_03);
        wf(c, 0xdfc, 0.054_984_73);
        wf(c, -0x2e4c, 1.570_796_4);
        wf(c, -0x2e50, -1.570_796_4);
    }

    /// The heading-blend constants (`Rider_UpdateHeadingAndAudio`).
    fn heading_constants(c: &mut Cpu) {
        for (off, v) in [
            (-0x790c, -1.570_796_4f32),
            (-0x7908, 1.570_796_4),
            (-0x7904, std::f32::consts::PI),
            (-0x7900, std::f32::consts::PI),
            (-0x78fc, std::f32::consts::PI),
            (-0x78f8, std::f32::consts::TAU),
            (-0x78f4, -std::f32::consts::PI),
            (-0x78f0, std::f32::consts::TAU),
            (-0x78ec, 0.15),
        ] {
            wf(c, off, v);
        }
    }

    #[test]
    fn rider_apply_wind_force_matches() {
        check(ssx3_ee::s00::f_00125970__generated, rider_apply_wind_force, 200, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x125970, 4);
            let rider = RIDER;
            c.g[A0][0] = rider as u64;
            // The wind direction table (512 sin/cos entries).
            let dirs = 0x0050_4fb8u32;
            for i in 0..0x300u32 {
                let v = (i as f32 * 0.002).sin();
                c.bus.write32(dirs + 4 * i, v.to_bits());
            }
            slot(c, VT, 0x20, 0);
            w(c, rider + rider::BASE, VT);
            let speed = match seed % 3 {
                0 => 0.0f32,
                1 => 1000.0,
                _ => 1.0,
            };
            let angle = match seed % 5 {
                0 => 0.0f32,
                1 => 45.0,
                2 => -170.0,
                3 => 359.0,
                _ => 12.5,
            };
            let fv = OBJ + 0x100;
            w(c, fv, speed.to_bits());
            w(c, fv + 4, angle.to_bits());
            w(c, fakes::LOG_RESULTS, fv);
            w(c, fakes::LOG_RESULTS + 4, fv + 4);
            w(c, rider + rider::IS_HUMAN, if seed % 3 == 0 { 0 } else { 1 });
            w(c, rider + rider::AI_INDEX, r.next_u32() % 4);
            // Some seeds have a velocity that already outruns the wind.
            let v = if seed % 4 == 3 { 1e30f32 } else { f32::from_bits(r.f32(50.0)) };
            put_vec(c, rider + rider::VELOCITY, [v.to_bits(), v.to_bits(), 0, 0]);
            wf(c, -0x7878, 1388.888_9);
            wf(c, -0x7874, 27.777_779);
            wf(c, -0x7870, 277.777_77);
            wf(c, -0x786c, 0.017_453_293);
            wf(c, -0x7868, 81.487_33);
            wf(c, -0x7864, 0.015_000_001);
        });
    }

    #[test]
    fn rider_update_heading_and_audio_matches() {
        check(ssx3_ee::s00::f_00121950__generated, rider_update_heading_and_audio, 200, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x121950, 1);
            let rider = RIDER;
            c.g[A0][0] = rider as u64;
            heading_constants(c);
            atan_constants(c);
            // A created audio system (so its instance accessor is trivial).
            w(c, GAME_GP.wrapping_add(0x410), 0);
            // Skip the heavy audio-paint block of UpdateRiderWorldPaint.
            w(c, rider + 0x870, 2);
            w(c, rider + rider::HANDLER, 0);
            w(c, rider + 0x5b0, (seed % 2) as u32);
            w(c, rider + 0x86c, 0);
            // The ground normal: sometimes a zero z (the degenerate branch).
            let nx = f32::from_bits(r.f32(4.0));
            let ny = f32::from_bits(r.f32(4.0));
            let nz = if seed % 4 == 0 { 0.0 } else { f32::from_bits(r.f32(4.0)) };
            put_vec(c, rider + rider::GROUND_NORMAL, [nx.to_bits(), ny.to_bits(), nz.to_bits(), 0]);
            let heading = match seed % 4 {
                0 => 0.0f32,
                1 => 100.0,
                2 => -100.0,
                _ => f32::from_bits(r.f32(7.0)),
            };
            w(c, rider + rider::HEADING, heading.to_bits());
            wf(c, -0x78b4, 0.925_082);
            // A null world keeps the danger manager quiet.
            w(c, GAME_GP.wrapping_sub(0x848), 0);
            let _ = &mut r;
        });
    }

    #[test]
    fn rider_update_visibility_matches() {
        check(ssx3_ee::s00::f_00122278__generated, rider_update_visibility, 200, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x122278, 2);
            let rider = RIDER;
            c.g[A0][0] = rider as u64;
            // A draw context whose world painter is a fake vtable.
            let ctx = OBJ;
            let painter = VT;
            w(c, ctx + 0x10d8, painter);
            slot(c, painter, 0x158, 0);
            slot(c, painter, 0x2e8, 0);
            w(c, painter + 0x2ec, fakes::FAKE + 0x10);
            w(c, GAME_GP.wrapping_sub(0x854), ctx);
            // The render state: only kind 7 without a block word is tested.
            w(c, rider + rider::RENDER_FLAG, if seed % 5 == 0 { 0 } else { 1 });
            w(c, rider + rider::RENDER_KIND, if seed % 5 == 2 { 5 } else { 7 });
            w(c, rider + rider::VISIBILITY_BLOCK, if seed % 5 == 3 { 1 } else { 0 });
            w(c, rider + rider::ON_SCREEN, if seed % 2 == 0 { 0 } else { 1 });
            w(c, rider + rider::UNK_150, if seed % 7 == 0 { 1 } else { 0 });
            w(c, rider + rider::HANDLER, OBJ + 0x2000);
            // Make the projection probes return "not visible" sometimes, so the
            // handler re-test and its branches run.
            w(c, fakes::LOG_RESULTS, if seed % 3 == 0 { 1 } else { 0 });
            w(c, fakes::LOG_RESULTS + 4, if seed % 2 == 0 { 1 } else { 0 });
            // A null world: the frustum query list is empty.
            w(c, GAME_GP.wrapping_sub(0x848), 0);
            let _ = &mut r;
        });
    }

    #[test]
    fn rider_update_boost_timers_matches() {
        check(ssx3_ee::s00::f_001200d0__generated, rider_update_boost_timers, 200, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x1200d0, 1);
            let rider = RIDER;
            c.g[A0][0] = rider as u64;
            wf(c, -0x7958, 0.016_666_667);
            wf(c, -0x7954, 0.016_666_667);
            wf(c, -0x7950, 0.016_666_667);
            wf(c, -0x794c, 0.004_999_999_8);
            wf(c, -0x7948, 0.023_986_11);
            slot(c, VT, 0x48, 0);
            w(c, rider + rider::BASE, VT);
            // The rider handler holds the state words at +0xde0/+0xde4.
            w(c, rider + rider::HANDLER, OBJ + 0x1000);
            let (main, sub) = match seed % 4 {
                0 => (1u32, 0u32),
                1 => (0, 0xc),
                2 => (0, (seed / 4) % 16),
                _ => (1, 6),
            };
            w(c, OBJ + 0x1000 + 0xde0, main);
            w(c, OBJ + 0x1000 + 0xde4, sub);
            let dt = if seed % 2 == 0 { 1.0f32 } else { 0.05 };
            w(c, rider + rider::DELTA_TIME, dt.to_bits());
            w(c, rider + rider::BOOST_A, r.edge_f32());
            w(c, rider + rider::BOOST_B, r.edge_f32());
            w(
                c,
                rider + rider::BOOST,
                if seed % 3 == 0 { 0 } else { f32::from_bits(r.f32(30.0)).to_bits() },
            );
            w(
                c,
                rider + rider::BOOST_PHASE,
                match seed % 5 {
                    0 => 10,
                    1 => 5,
                    2 => 1,
                    3 => 2,
                    _ => 3,
                },
            );
            w(c, rider + rider::BOOST_SCALE, r.edge_f32());
            w(c, rider + rider::BOOST_KIND, (seed / 5) % 5);
            w(c, GAME_GP.wrapping_sub(0x848), 0);
        });
    }

    #[test]
    fn rider_check_race_finish_matches() {
        check(ssx3_ee::s00::f_00125228__generated, rider_check_race_finish, 200, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x125228, 6);
            let rider = RIDER;
            c.g[A0][0] = rider as u64;
            // The rider base's "is human" slot 0x40.
            slot(c, VT, 0x40, 0);
            w(c, rider + rider::BASE, VT);
            wf(c, -0x787c, 0.016_666_667);
            // A created BE library object (never let it build one).
            w(c, GAME_GP.wrapping_sub(0x1f04), OBJ + 0x3000);
            w(c, GAME_GP.wrapping_sub(0x1efc), 0);
            w(c, 0x0053_08a8, 0);
            // The world / race manager chain.
            let world = OBJ + 0x2000;
            w(c, GAME_GP.wrapping_sub(0x848), world);
            w(c, world + 0x84, world + 0x100);
            w(c, world + 0xc0, OBJ + 0x2200);
            w(c, world + 0x100 + 0x214, 0);
            w(c, world + 0x100 + 0x8, r.next_u32());
            w(c, OBJ + 0x2200 + 0x88, if seed % 6 == 4 { 1 } else { 0 });
            w(c, OBJ + 0x2200 + 0x78, r.next_u32());
            let finish = match seed % 3 {
                0 => 1.0f32,
                1 => -1.0,
                _ => -1000.0,
            };
            w(c, rider + rider::FINISH_TIME, finish.to_bits());
            w(c, rider + rider::DID_NOT_FINISH, if seed % 4 == 0 { 1 } else { 0 });
            w(c, rider + 0x87c, 0);
            w(c, rider + 0x150, 0);
            w(c, rider + rider::IS_HUMAN, 1);
            let _ = &mut r;
        });
    }

    #[test]
    fn rider_update_race_progress_matches() {
        check(ssx3_ee::s00::f_00112338__generated, rider_update_race_progress, 120, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x112338, 3);
            let rider = RIDER;
            c.g[A0][0] = rider as u64;
            w(c, GAME_GP.wrapping_sub(0x848), 0);
            // A created BE library object (the event handler reaches it).
            w(c, GAME_GP.wrapping_sub(0x1f04), OBJ + 0x3000);
            w(c, GAME_GP.wrapping_sub(0x1efc), 0);
            w(c, 0x0053_08a8, 0);
            // No race path (the null path reads as an empty one).
            w(c, rider + rider::RACE_PATH, 0);
            w(c, rider + rider::RACE_PATH_CURSOR, 0);
            w(c, rider + rider::DIST_TO_FINISH, 0);
            w(c, rider + rider::DIST_ESTIMATE, 0);
            let count = (seed % 3) as u32;
            w(c, rider + rider::RACE_ENTRY_COUNT, count);
            for i in 0..count {
                w(c, rider + rider::RACE_ENTRIES + 16 * i, 0);
                w(c, rider + rider::RACE_ENTRIES + 16 * i + 4, 0);
                w(c, rider + rider::RACE_ENTRIES + 16 * i + 8, r.next_u32());
            }
            // The shared list is empty: no path events to hand out.
            for i in 0..8u32 {
                w(c, RACE_EVENTS + 4 * i, 0);
            }
            let _ = &mut r;
        });
    }

    #[test]
    fn rider_update_ai_path_progress_matches() {
        check(ssx3_ee::s00::f_001125c0__generated, rider_update_ai_path_progress, 120, &|c, seed| {
            let mut r = fakes::machine(c, seed, 0x1125c0, 4);
            let rider = RIDER;
            c.g[A0][0] = rider as u64;
            atan_constants(c);
            w(c, GAME_GP.wrapping_sub(0x848), 0);
            // A null AI path reads as an empty one.
            w(c, rider + rider::AI_PATH, 0);
            w(c, rider + rider::AI_PATH_CURSOR, 0);
            w(c, rider + rider::DIST_TO_FINISH, 0);
            w(c, rider + rider::AI_ARC_LEN, r.edge_f32());
            w(c, rider + rider::AI_PROGRESS, f32::from_bits(r.f32(100.0)).to_bits());
            w(c, rider + rider::AI_SEARCH_DIST, f32::from_bits(r.f32(100.0)).to_bits());
            // The look-ahead offset: sometimes the degenerate branches.
            let target = match seed % 4 {
                0 => 0.0f32,
                1 => 1.0,
                2 => -1.0,
                _ => f32::from_bits(r.f32(5.0)),
            };
            let point = match seed % 3 {
                0 => 0.0f32,
                1 => 1.0,
                _ => f32::from_bits(r.f32(5.0)),
            };
            w(c, rider + rider::AI_POINT, point.to_bits());
            w(c, rider + rider::AI_POINT + 4, point.to_bits());
            w(c, rider + rider::AI_POINT_TARGET, target.to_bits());
            w(c, rider + rider::AI_POINT_TARGET + 4, target.to_bits());
        });
    }
}
