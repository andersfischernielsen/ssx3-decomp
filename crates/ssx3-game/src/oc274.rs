//! Staging wave 209 (oc274): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi;
use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0010_d870, computer_actor_rider_get_path_class_weight, "ComputerActorRider_GetPathClassWeight"),
    (0x002a_ae78, stream_array_get_time_remaining, "StreamArray_GetTimeRemaining"),
    (0x0011_5640, rider_reset_grind_orientation, "Rider_ResetGrindOrientation"),
    (0x0029_5028, ssx_audio_system_is_tracked_sound_playing, "SSXAudioSystem_IsTrackedSoundPlaying"),
    (0x003e_0170, stream_file_on_read, "StreamFile_OnRead"),
    (0x003c_d6f0, snd_utk_parse_header, "SND_UtkParseHeader"),
];

// ---------------------------------------------------------------------------
// ComputerActorRider_GetPathClassWeight (0x10d870, ai/computer.cpp)

/// ComputerActorRider: the rider's preferred path class (0, 1, 2 or other).
const CAR_PATH_CLASS: u32 = 0xe00;
/// `gp - 0x7ce8`: the weight per unmatched path flag (33.3f).
const CAR_PATH_WEIGHT_STEP_GP: u32 = 0x7ce8;

/// `float ComputerActorRider_GetPathClassWeight(ComputerActorRider* self,
/// int flagA, int flagB, int flagC)` (0x10d870): how attractive a path with
/// the three class flags is to the AI rider. A path is rejected (0.0) when it
/// lacks the flag of the rider's preferred class (class 0: A, 1: B, 2: C;
/// any other class: `(A == B) != C`); an accepted path weighs
/// `(4 - number of set flags) * 33.3`.
pub fn computer_actor_rider_get_path_class_weight(c: &mut Cpu) {
    c.cycles += 3;
    let class = Addr(r32(c, A0).wrapping_add(CAR_PATH_CLASS)).lh(c);
    c.g[V1][0] = class;
    c.g[V0][0] = 1;
    let (flag_a, flag_b, flag_c) = (c.g[A1][0], c.g[A2][0], c.g[A3][0]);
    c.cycles += 2;
    let accepted = if class == 0 {
        c.g[V1][0] = 3;
        flag_a != 0
    } else if class == 1 {
        c.cycles += 2;
        c.g[V1][0] = 3;
        if flag_b == 0 {
            c.cycles += 2;
        }
        flag_b != 0
    } else {
        c.g[V0][0] = 2;
        c.cycles += 2;
        if class == 2 {
            c.cycles += 2;
            c.g[V1][0] = 3;
            if flag_c == 0 {
                c.cycles += 2;
            }
            flag_c != 0
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((flag_a ^ flag_b) < 1) as u64;
            c.g[V1][0] = 3;
            c.g[V0][0] == flag_c
        }
    };
    if accepted {
        c.cycles += 11;
        // movn chain: 4, minus one per set flag (v1 holds 3 here).
        let mut n = if flag_a != 0 { c.g[V1][0] } else { 4 };
        let gp = r32(c, GP);
        let step = Addr(gp.wrapping_sub(CAR_PATH_WEIGHT_STEP_GP)).read_fpu(c);
        abi::set_fpr(c, 1, step);
        c.g[V1][0] = sx((n as u32).wrapping_sub(1));
        if flag_b != 0 {
            n = c.g[V1][0];
        }
        c.g[A0][0] = sx((n as u32).wrapping_sub(1));
        if flag_c != 0 {
            n = c.g[A0][0];
        }
        c.g[V0][0] = n;
        abi::set_fret(c, FpuF32::from_i32(n as u32 as i32) * step);
    } else {
        c.cycles += 3;
        c.f[0] = 0;
    }
    ret(c);
}

// ---------------------------------------------------------------------------
// StreamArray_GetTimeRemaining (0x2aae78, sound/streamsys.cpp)

/// StreamArray: pointer to its array of StreamBuffer pointers.
const SAT_BUFFERS: u32 = 0x8;

/// `StreamBuffer_GetTimeRemaining` (0x2aa108).
const SAT_STREAM_BUFFER_GET_TIME_REMAINING: Callee = Callee {
    addr: 0x002a_a108,
    f: ee::s08::StreamBuffer_GetTimeRemaining,
};

/// `float StreamArray_GetTimeRemaining(StreamArray* self, int i)` (0x2aae78):
/// the time left in stream buffer `i` (`StreamBuffer_GetTimeRemaining`), or
/// 0.0 when `i` is negative or that slot is empty.
pub fn stream_array_get_time_remaining(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let mut present = false;
        if (c.g[A1][0] as i64) >= 0 {
            c.cycles += 6;
            let buffers = Addr(r32(c, A0).wrapping_add(SAT_BUFFERS)).lw(c);
            c.g[V1][0] = buffers;
            let slot = (r32(c, A1) << 2).wrapping_add(buffers as u32);
            c.g[V0][0] = sx(slot);
            c.g[A0][0] = Addr(slot).lw(c);
            present = c.g[A0][0] != 0;
        }
        if present {
            c.cycles += 3;
            SAT_STREAM_BUFFER_GET_TIME_REMAINING.call(c, 0x002a_aeb0)?;
        } else {
            c.cycles += 3;
            c.f[0] = 0;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider_ResetGrindOrientation (0x115640, seg/seg_14B10.cpp)

/// Rider: the grind orientation (0 none, 3 / 4 sideways left / right).
const RGO_GRIND_SIDE: u32 = 0x328;
/// Rider: the rotation axis quadword handed to `Rider_RotateAroundAxis`.
const RGO_AXIS: u32 = 0x1c0;
/// Rider: its animation controller.
const RGO_ANIM: u32 = 0x784;
/// AnimController: the root rotation quadword and the heading vector.
const RGO_ANIM_ROOT_ROT: u32 = 0x30;
const RGO_ANIM_HEADING: u32 = 0x40;
/// The identity rotation quadword (0x4ff130) and the up axis (0x4ff160).
const RGO_IDENTITY: u32 = 0x004f_f130;
const RGO_UP: u32 = 0x004f_f160;
/// `gp - 0x7b70` / `gp - 0x7b6c`: +pi/2 and -pi/2.
const RGO_QUARTER_TURN_GP: u32 = 0x7b70;
const RGO_NEG_QUARTER_TURN_GP: u32 = 0x7b6c;

const RGO_GET_MAIN_STATE: Callee = Callee {
    addr: 0x0011_fe98,
    f: ee::s00::Rider_GetMainState,
};
const RGO_ROTATE_AROUND_AXIS: Callee = Callee {
    addr: 0x0011_dfe0,
    f: ee::s00::Rider_RotateAroundAxis,
};
const RGO_EMPTY_STANCE_HOOK: Callee = Callee {
    addr: 0x0011_6930,
    f: ee::s00::Rider_EmptyStanceHook,
};
/// `Math_SinCos(float, float* sin, float* cos)`, `AnimController_RotateRoots`
/// and `cRiderAnimBase_play(anim, id, flags, -1.0)` (other shards).
const RGO_MATH_SIN_COS: u32 = 0x0031_be50;
const RGO_ROTATE_ROOTS: u32 = 0x0031_1b48;
const RGO_PLAY: u32 = 0x0031_28e8;

/// The heading vector `up * sin` (from the sincos results at `sin_at`,
/// `sin_at + 4`; the up axis in `up_reg`, its x at `s0 - 0xea0`) built in the frame's first quadword and stored into the
/// animation controller in `anim_reg`: shared by the two sideways arms.
fn rgo_store_heading(c: &mut Cpu, sin_at: u32, up_reg: usize, anim_reg: usize) {
    let sp_ = sp(c);
    let up = r32(c, up_reg);
    let s = Addr(sp_.wrapping_add(sin_at)).read_fpu(c);
    let y = Addr(up.wrapping_add(8)).read_fpu(c);
    let x = Addr(r32(c, S0).wrapping_sub(0xea0)).read_fpu(c);
    let z = Addr(up.wrapping_add(4)).read_fpu(c);
    let f1 = s * y;
    let f2 = s * x;
    let f3 = Addr(sp_.wrapping_add(sin_at + 4)).read_fpu(c);
    let f0 = s * z;
    abi::set_fpr(c, 1, f1);
    abi::set_fpr(c, 2, f2);
    abi::set_fpr(c, 3, f3);
    abi::set_fpr(c, 4, z);
    abi::set_fpr(c, 0, f0);
    Addr(sp_ + 0xc).write_fpu(c, f3);
    Addr(sp_ + 0x8).write_fpu(c, f1);
    Addr(sp_).write_fpu(c, f2);
    Addr(sp_ + 0x4).write_fpu(c, f0);
    c.lq(2, sp_);
    let anim = r32(c, anim_reg);
    c.sq(2, anim.wrapping_add(RGO_ANIM_HEADING));
}

/// Reset the animation controller's root rotation to identity and compute
/// sin/cos of -0.0 into `sin_at` (`Math_SinCos`), the controller in `anim_reg`.
fn rgo_reset_root(c: &mut Cpu, anim_reg: usize, sin_at: u32, ret_: u32) -> crate::call::Flow {
    c.f[12] = 0;
    c.g[V0][0] = sx(RGO_IDENTITY);
    c.g[anim_reg][0] = Addr(r32(c, S3).wrapping_add(RGO_ANIM)).lw(c);
    c.lq(3, RGO_IDENTITY);
    c.f[12] ^= 0x8000_0000;
    c.g[A0][0] = sx(sp(c).wrapping_add(sin_at));
    c.g[A1][0] = sx(sp(c).wrapping_add(sin_at + 4));
    let anim = r32(c, anim_reg);
    c.sq(3, anim.wrapping_add(RGO_ANIM_ROOT_ROT));
    call_ptr(c, RGO_MATH_SIN_COS, ret_)
}

/// A sideways grind arm: turn the rider by the quarter turn at `gp - lit`
/// around its axis, rotate the animation roots by the same angle, then
/// reset the root rotation and the heading vector.
fn rgo_turn_sideways(c: &mut Cpu, lit: u32, sin_at: u32, rets: [u32; 3]) -> crate::call::Flow {
    c.lq(2, r32(c, S3).wrapping_add(RGO_AXIS));
    c.g[A1][0] = c.g[SP][0];
    let gp = r32(c, GP);
    let angle = Addr(gp.wrapping_sub(lit)).read_fpu(c);
    abi::set_fpr(c, 20, angle);
    c.g[A0][0] = c.g[S3][0];
    c.sq(2, sp(c));
    c.g[S0][0] = 0x50_0000;
    abi::set_fpr(c, 12, angle);
    c.g[S1][0] = sx(RGO_UP);
    RGO_ROTATE_AROUND_AXIS.call(c, rets[0])?;
    c.g[A0][0] = Addr(r32(c, S3).wrapping_add(RGO_ANIM)).lw(c);
    c.f[12] = c.f[20];
    call_ptr(c, RGO_ROTATE_ROOTS, rets[1])?;
    rgo_reset_root(c, S2, sin_at, rets[2])
}

/// `void Rider_ResetGrindOrientation(Rider* rider)` (0x115640): when the
/// rider leaves a grind (orientation at +0x328 non-zero), restore its
/// animation. In main state 1, a sideways grind (3 / 4) resets the root
/// rotation and heading, and the matching end-of-grind animation plays
/// (0x11a / 0x115, else 0x10c). In main state 0, a sideways grind turns the
/// rider back by a quarter turn (+pi/2 for 3, -pi/2 for 4) first, and
/// animation 5 plays. Other main states only clear the orientation. Then
/// `Rider_EmptyStanceHook`.
pub fn rider_reset_grind_orientation(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S3, 0x40);
        frame.save(c, S0, 0x70);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        frame.save_ra(c, 0x30);
        frame.save_fpr(c, 20, 0x80);
        c.g[V0][0] = Addr(r32(c, S3).wrapping_add(RGO_GRIND_SIDE)).lw(c);
        let grinding = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x70);
        if grinding {
            c.cycles += 5;
            RGO_GET_MAIN_STATE.call(c, 0x0011_5674)?;
            c.g[V1][0] = 1;
            if c.g[V0][0] == 1 {
                c.cycles += 5;
                let side = Addr(r32(c, S3).wrapping_add(RGO_GRIND_SIDE)).lw(c);
                let sideways = (sx((side as u32).wrapping_sub(3)) < 2) as u64;
                c.g[V0][0] = 0x50_0000;
                if sideways != 0 {
                    c.cycles += 25;
                    c.g[S0][0] = 0x50_0000;
                    rgo_reset_root(c, S1, 0x10, 0x0011_56bc)?;
                    c.g[V0][0] = sx(r32(c, S0).wrapping_sub(0xea0));
                    rgo_store_heading(c, 0x10, V0, S1);
                }
                c.cycles += 4;
                let side = Addr(r32(c, S3).wrapping_add(RGO_GRIND_SIDE)).lw(c);
                c.g[V1][0] = side;
                c.g[V0][0] = 4;
                c.g[A0][0] = Addr(r32(c, S3).wrapping_add(RGO_ANIM)).lw(c);
                if side == 3 {
                    c.cycles += 3;
                    c.g[A1][0] = 0x11a;
                } else {
                    c.cycles += 2;
                    if side == 4 {
                        c.cycles += 2;
                        c.g[A1][0] = 0x115;
                    } else {
                        c.cycles += 1;
                        c.g[A1][0] = 0x10c;
                    }
                }
                c.cycles += 6;
                c.g[AT][0] = 0xffff_ffff_bf80_0000;
                c.f[12] = 0xbf80_0000;
                c.g[A2][0] = 0;
                call_ptr(c, RGO_PLAY, 0x0011_5738)?;
                Addr(r32(c, S3).wrapping_add(RGO_GRIND_SIDE)).sw(c, 0);
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[S3][0];
                RGO_GET_MAIN_STATE.call(c, 0x0011_5748)?;
                if c.g[V0][0] == 0 {
                    c.cycles += 4;
                    let side = Addr(r32(c, S3).wrapping_add(RGO_GRIND_SIDE)).lw(c);
                    c.g[V1][0] = side;
                    c.g[V0][0] = 4;
                    let turned = if side == 3 {
                        c.cycles += 30;
                        rgo_turn_sideways(
                            c,
                            RGO_QUARTER_TURN_GP,
                            0x18,
                            [0x0011_5784, 0x0011_5790, 0x0011_57b8],
                        )?;
                        true
                    } else {
                        c.cycles += 2;
                        if c.g[V1][0] == c.g[V0][0] {
                            c.cycles += 29;
                            rgo_turn_sideways(
                                c,
                                RGO_NEG_QUARTER_TURN_GP,
                                0x20,
                                [0x0011_5804, 0x0011_5810, 0x0011_5838],
                            )?;
                            true
                        } else {
                            false
                        }
                    };
                    if turned {
                        c.cycles += 8;
                        let sin_at = if side == 3 { 0x18 } else { 0x20 };
                        rgo_store_heading(c, sin_at, S1, S2);
                    }
                    c.g[A0][0] = Addr(r32(c, S3).wrapping_add(RGO_ANIM)).lw(c);
                    c.cycles += 6;
                    c.g[A1][0] = 5;
                    c.g[AT][0] = 0xffff_ffff_bf80_0000;
                    c.f[12] = 0xbf80_0000;
                    c.g[A2][0] = 0;
                    call_ptr(c, RGO_PLAY, 0x0011_5888)?;
                }
                Addr(r32(c, S3).wrapping_add(RGO_GRIND_SIDE)).sw(c, 0);
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S3][0];
            RGO_EMPTY_STANCE_HOOK.call(c, 0x0011_5894)?;
            frame.restore(c, S0, 0x70);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore_ra(c, 0x30);
        frame.restore_fpr(c, 20, 0x80);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_IsTrackedSoundPlaying (0x295028, sound/soundsys.cpp)

/// SSXAudioSystem: the tracked-sound ring `{ int size; int* words; int
/// write; int read; }` (records of 5 words: key, handle, ..., subkey) and
/// the `Audio**` whose `*p + 0x1d8` is the bank monitor.
const TSP_RING_SIZE: u32 = 0x597c;
const TSP_RING_WORDS: u32 = 0x4; // from the ring
const TSP_RING_WRITE: u32 = 0x5984;
const TSP_RING_READ: u32 = 0x5988;
const TSP_AUDIO: u32 = 0x118;
const TSP_BANK_MONITOR: u32 = 0x1d8;

const TSP_BANK_MONITOR_FIND_INSTANCE: Callee = Callee {
    addr: 0x002a_d550,
    f: ee::s08::BankMonitor_FindInstance,
};

/// Ring word `idx` (the register value the original compares) counted from
/// the read index, or -1 past the queued words or for an empty ring. The
/// caller has charged its prefix and holds write in a3 and read in a1;
/// leaves v0/v1/a0 as the original does. Returns the word (sign-extended).
fn tsp_ring_get(c: &mut Cpu, idx: u64) -> u64 {
    let this = r32(c, S0);
    if c.g[A3][0] == c.g[A1][0] {
        c.cycles += 2;
        return 0xffff_ffff_ffff_ffff;
    }
    c.cycles += 10;
    c.g[A0][0] = Addr(this.wrapping_add(TSP_RING_SIZE)).lw(c);
    c.g[V0][0] = sx(r32(c, A3).wrapping_sub(r32(c, A1)).wrapping_add(r32(c, A0)));
    let (_, queued) = { let (n, d) = (r32(c, V0), r32(c, A0)); abi::div(c, n, d) };
    let in_range = (idx as i64) < (sx(queued) as i64);
    c.g[V1][0] = sx(r32(c, A1).wrapping_add(idx as u32));
    c.cycles += 2;
    if !in_range {
        return 0xffff_ffff_ffff_ffff;
    }
    c.cycles += 6;
    let (_, slot) = { let (n, d) = (r32(c, V1), r32(c, A0)); abi::div(c, n, d) };
    c.g[A0][0] = Addr(r32(c, S2).wrapping_add(TSP_RING_WORDS)).lw(c);
    c.g[V0][0] = sx((slot << 2).wrapping_add(r32(c, A0)));
    Addr(r32(c, V0)).lw(c)
}

/// `bool SSXAudioSystem_IsTrackedSoundPlaying(SSXAudioSystem* self, int key,
/// int subkey)` (0x295028): whether a tracked-sound record with this key
/// and subkey is queued and still playing (handle 0, or a bank monitor
/// instance with that handle still exists).
pub fn ssx_audio_system_is_tracked_sound_playing(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S3, 0x30);
        c.g[S1][0] = 0;
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, S2, 0x40);
        c.g[S5][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let this = r32(c, S0);
        c.g[V1][0] = Addr(this.wrapping_add(TSP_RING_WRITE)).lw(c);
        c.g[A0][0] = Addr(this.wrapping_add(TSP_RING_READ)).lw(c);
        c.g[V0][0] = Addr(this.wrapping_add(TSP_RING_SIZE)).lw(c);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(r32(c, A0)).wrapping_add(r32(c, V0)));
        c.cycles += 4;
        let (_, queued) = { let (n, d) = (r32(c, V1), r32(c, V0)); abi::div(c, n, d) };
        c.g[S3][0] = sx(queued);
        c.g[S2][0] = sx(this.wrapping_add(TSP_RING_SIZE));
        let mut playing = false;
        if (c.g[S3][0] as i64) > 0 {
            c.cycles += 1;
            c.g[V1][0] = Addr(r32(c, S0).wrapping_add(TSP_RING_WRITE)).lw(c);
            loop {
                let this = r32(c, S0);
                c.cycles += 4;
                c.g[V0][0] = Addr(this.wrapping_add(TSP_RING_READ)).lw(c);
                c.g[A3][0] = c.g[V1][0];
                c.g[A1][0] = c.g[V0][0];
                let s1 = c.g[S1][0];
                c.g[T0][0] = tsp_ring_get(c, s1);
                c.cycles += 2;
                c.g[A2][0] = sx((s1 as u32).wrapping_add(4));
                let a2 = c.g[A2][0];
                c.g[V0][0] = tsp_ring_get(c, a2);
                c.cycles += 2;
                let mut matched = false;
                if c.g[S5][0] == c.g[T0][0] {
                    c.cycles += 2;
                    matched = c.g[S4][0] == c.g[V0][0];
                }
                if !matched {
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(5));
                } else {
                    c.cycles += 2;
                    c.g[A2][0] = sx(r32(c, S1).wrapping_add(1));
                    let a2 = c.g[A2][0];
                    c.g[A1][0] = tsp_ring_get(c, a2);
                    c.cycles += 2;
                    c.g[A2][0] = 1;
                    if c.g[A1][0] == 0 {
                        playing = true;
                    } else {
                        c.cycles += 6;
                        let audio = Addr(r32(c, S0).wrapping_add(TSP_AUDIO)).lw(c);
                        c.g[V0][0] = audio;
                        let sys = Addr(audio as u32).lw(c);
                        c.g[A0][0] = sx((sys as u32).wrapping_add(TSP_BANK_MONITOR));
                        TSP_BANK_MONITOR_FIND_INSTANCE.call(c, 0x0029_51c0)?;
                        c.g[S1][0] = sx(r32(c, S1).wrapping_add(5));
                        playing = c.g[V0][0] != 0;
                    }
                    if playing {
                        c.cycles += 2;
                        c.g[V0][0] = 1;
                        break;
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S3][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V1][0] = Addr(r32(c, S0).wrapping_add(TSP_RING_WRITE)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0029_5088);
                }
            }
        }
        if !playing {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamFile_OnRead (0x3e0170, ealib/seg_2DCAE8.c)

/// StreamFile fields: its mutex (+4), the read-ahead length (+0x40), the
/// fill cursor (+0x58), the stream header (+0x60), the bytes read so far
/// (+0x170), the EAFile request (+0x174) and the requested length (+0x178).
const SFR_MUTEX: u32 = 0x4;
const SFR_READ_AHEAD: u32 = 0x40;
const SFR_FILL: u32 = 0x58;
const SFR_HEADER: u32 = 0x60;
const SFR_BYTES_READ: u32 = 0x170;
const SFR_REQUEST: u32 = 0x174;
const SFR_REQUEST_LEN: u32 = 0x178;
/// Header fields: the stream state (+4; 3 = done, 4 = closing), the source
/// kind (+0x10; 1 = memory, no EAFile request) and the total size (+0x118).
const SFR_HDR_STATE: u32 = 0x4;
const SFR_HDR_SOURCE: u32 = 0x10;
const SFR_HDR_SIZE: u32 = 0x118;
const SFR_STATE_DONE: u64 = 3;
const SFR_STATE_CLOSING: u64 = 4;

const SFR_RELEASE_REQUEST: Callee = Callee {
    addr: 0x003d_d310,
    f: ee::s14::EAFile_ReleaseRequest,
};
const SFR_PARSE_CHUNKS: Callee = Callee {
    addr: 0x003d_fed0,
    f: ee::s14::StreamFile_ParseChunks,
};
const SFR_LOCK: Callee = Callee {
    addr: 0x003e_5700,
    f: ee::s14::EAMutex_Lock,
};
const SFR_UNLOCK: Callee = Callee {
    addr: 0x003e_5760,
    f: ee::s14::EAMutex_Unlock,
};
const SFR_START_NEXT_REQUEST: Callee = Callee {
    addr: 0x003e_0270,
    f: ee::s14::StreamFile_StartNextRequest,
};
const SFR_ISSUE_READ: Callee = Callee {
    addr: 0x003e_03d0,
    f: ee::s14::StreamFile_IssueRead,
};

/// `void StreamFile_OnRead(int, int, StreamFile* file)` (0x3e0170): the
/// read-completion callback. Counts the bytes that arrived (the requested
/// length for a memory source, else what `EAFile_ReleaseRequest` reports),
/// advances the fill cursor and parses the new chunks. When the stream is
/// closing, finished (end of the source, a short read) or an end chunk was
/// parsed, marks it done (state 3, under the mutex, unless closing) and
/// starts the next request; otherwise issues the next read.
pub fn stream_file_on_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = 1;
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        frame.save(c, S2, 0x20);
        let file = r32(c, S0);
        c.g[S2][0] = Addr(file.wrapping_add(SFR_HEADER)).lw(c);
        c.g[V0][0] = Addr(r32(c, S2).wrapping_add(SFR_HDR_SOURCE)).lw(c);
        if c.g[V0][0] == 1 {
            c.cycles += 7;
            c.g[A1][0] = Addr(file.wrapping_add(SFR_REQUEST_LEN)).lw(c);
            c.g[V1][0] = Addr(file.wrapping_add(SFR_BYTES_READ)).lw(c);
            c.g[V0][0] = Addr(r32(c, S2).wrapping_add(SFR_HDR_SIZE)).lw(c);
            let total = sx(r32(c, V1).wrapping_add(r32(c, A1)));
            c.g[V0][0] = ((total as i64) < (c.g[V0][0] as i64)) as u64;
            // At (or past) the end of the source.
            c.g[S1][0] = c.g[V0][0] ^ 1;
        } else {
            c.cycles += 6;
            c.g[A0][0] = Addr(file.wrapping_add(SFR_REQUEST)).lw(c);
            SFR_RELEASE_REQUEST.call(c, 0x003e_01c4)?;
            let file = r32(c, S0);
            c.g[V1][0] = Addr(file.wrapping_add(SFR_REQUEST_LEN)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            // A short read.
            c.g[S1][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
            c.g[V1][0] = Addr(file.wrapping_add(SFR_BYTES_READ)).lw(c);
        }
        c.cycles += 12;
        let file = r32(c, S0);
        c.g[A0][0] = c.g[S0][0];
        c.g[V0][0] = Addr(file.wrapping_add(SFR_FILL)).lw(c);
        c.g[S3][0] = SFR_STATE_CLOSING;
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
        let (bytes_read, fill) = (c.g[V1][0], c.g[V0][0]);
        Addr(file.wrapping_add(SFR_BYTES_READ)).sw(c, bytes_read);
        Addr(file.wrapping_add(SFR_FILL)).sw(c, fill);
        SFR_PARSE_CHUNKS.call(c, 0x003e_01f4)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = Addr(r32(c, S2).wrapping_add(SFR_HDR_STATE)).lw(c);
        let closing = c.g[V0][0] == c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        if !closing {
            c.cycles += 2;
            let finished = c.g[S1][0] != 0;
            c.g[S1][0] = sx(r32(c, S0).wrapping_add(SFR_MUTEX));
            let mut done = finished;
            if !finished {
                c.cycles += 2;
                if c.g[V1][0] == 0 {
                    // More to read: issue the next read.
                    c.g[A1][0] = Addr(r32(c, S0).wrapping_add(SFR_READ_AHEAD)).lw(c);
                    c.cycles += 3;
                    c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
                    SFR_ISSUE_READ.call(c, 0x003e_0250)?;
                    frame.restore(c, S0, 0x40);
                } else {
                    done = true;
                }
            }
            if done {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                SFR_LOCK.call(c, 0x003e_021c)?;
                c.g[V0][0] = Addr(r32(c, S2).wrapping_add(SFR_HDR_STATE)).lw(c);
                let closing = c.g[V0][0] == c.g[S3][0];
                c.g[V0][0] = SFR_STATE_DONE;
                if !closing {
                    c.cycles += 1;
                    Addr(r32(c, S2).wrapping_add(SFR_HDR_STATE)).sw(c, SFR_STATE_DONE);
                }
                c.cycles += 3;
                c.g[A0][0] = c.g[S1][0];
                SFR_UNLOCK.call(c, 0x003e_0234)?;
                c.g[A0][0] = c.g[S0][0];
            } else {
                c.cycles += 6;
                frame.restore(c, S1, 0x30);
                frame.restore(c, S2, 0x20);
                frame.restore(c, S3, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
        // Start the next request.
        c.cycles += 4;
        c.g[A1][0] = Addr(r32(c, A0).wrapping_add(SFR_READ_AHEAD)).lw(c);
        SFR_START_NEXT_REQUEST.call(c, 0x003e_0240)?;
        frame.restore(c, S0, 0x40);
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_UtkParseHeader (0x3cd6f0, ealib/seg_2B4578.c)

/// UTK decoder state: the byte cursor (+0), the bit buffer (+4), the bits
/// in it (+8), the header flag (+0xc), the excitation bit count (+0x10),
/// the fixed-codebook gain (+0x14), the 64 reflection gains (+0x18..), and
/// the two 12-word adaptive histories (+0x114, +0x144) and the 324-word
/// excitation buffer ending at +0x680.
const UTK_CURSOR: u32 = 0x0;
const UTK_BITS: u32 = 0x4;
const UTK_BIT_COUNT: u32 = 0x8;
const UTK_FLAG: u32 = 0xc;
const UTK_EXC_BITS: u32 = 0x10;
const UTK_FCB_GAIN: u32 = 0x14;
const UTK_GAINS: u32 = 0x18;
const UTK_HIST_A: u32 = 0x114;
const UTK_HIST_B: u32 = 0x144;
const UTK_EXCITATION: u32 = 0x174;

const UTK_READ_BITS: Callee = Callee {
    addr: 0x003c_d1f8,
    f: ee::s14::SND_UtkReadBits,
};

/// `cvt.s.w` of v0 as an unsigned word: halve, convert, double when the
/// value is "negative" (the original's unsigned-to-float idiom; v1 holds
/// `v0 >> 1`).
fn utk_unsigned_to_float(c: &mut Cpu, reg: usize) {
    c.cycles += 5;
    c.g[V0][0] = (c.g[V0][0] & 1) | c.g[V1][0];
    let f = FpuF32::from_i32(c.g[V0][0] as u32 as i32);
    abi::set_fpr(c, reg, f + f);
}

/// `void SND_UtkParseHeader(UtkHeader* h, uint8_t* data, UtkState* st, int
/// reset)` (0x3cd6f0): start the UTK bit reader at `data` (first byte
/// preloaded, 8 bits). When `reset`, read the frame header: a 1-bit flag,
/// the 4-bit excitation size (stored as 32 - n), the 4-bit codebook gain
/// index (gain = (n + 1) * 8), the 6-bit gain index from which the 64
/// reflection gains grow geometrically by `1.04 + n * 0.001`, and clear the
/// histories and the excitation buffer.
pub fn snd_utk_parse_header(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 8;
        frame.save(c, S0, 0x10);
        let data = r32(c, A1);
        c.g[V1][0] = sx(data.wrapping_add(1));
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        let st = r32(c, S0);
        Addr(st.wrapping_add(UTK_BIT_COUNT)).sw(c, 8);
        let next = c.g[V1][0];
        Addr(st.wrapping_add(UTK_CURSOR)).sw(c, next);
        c.g[V0][0] = Addr(data).lbu(c);
        let first = c.g[V0][0];
        Addr(st.wrapping_add(UTK_BITS)).sw(c, first);
        if c.g[A3][0] != 0 {
            c.cycles += 16;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 1;
            UTK_READ_BITS.call(c, 0x003c_d728)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 4;
            let flag = c.g[V0][0];
            Addr(r32(c, S0).wrapping_add(UTK_FLAG)).sw(c, flag);
            UTK_READ_BITS.call(c, 0x003c_d738)?;
            c.g[V1][0] = sx(0x20u32.wrapping_sub(r32(c, V0)));
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 4;
            let exc_bits = c.g[V1][0];
            Addr(r32(c, S0).wrapping_add(UTK_EXC_BITS)).sw(c, exc_bits);
            UTK_READ_BITS.call(c, 0x003c_d750)?;
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
            c.g[V1][0] = sx(r32(c, V0) >> 1);
            if (c.g[V0][0] as i64) >= 0 {
                c.cycles += 4;
                abi::set_fpr(c, 1, FpuF32::from_i32(c.g[V0][0] as u32 as i32));
            } else {
                utk_unsigned_to_float(c, 1);
            }
            c.cycles += 9;
            c.g[AT][0] = 0x4100_0000;
            let eight = FpuF32(0x4100_0000);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 6;
            let gain = FpuF32(c.f[1]) * eight;
            abi::set_fpr(c, 0, gain);
            Addr(r32(c, S0).wrapping_add(UTK_FCB_GAIN)).write_fpu(c, gain);
            UTK_READ_BITS.call(c, 0x003c_d79c)?;
            if (c.g[V0][0] as i64) < 0 {
                c.g[V1][0] = sx(r32(c, V0) >> 1);
                utk_unsigned_to_float(c, 2);
            } else {
                c.cycles += 4;
                abi::set_fpr(c, 2, FpuF32::from_i32(c.g[V0][0] as u32 as i32));
            }
            c.cycles += 14;
            let step = FpuF32(0x3a83_126f);
            let st = r32(c, S0);
            c.g[V1][0] = sx(st.wrapping_add(UTK_GAINS));
            c.g[AT][0] = 0x3f85_1eb8;
            c.g[A3][0] = sx(st.wrapping_add(UTK_HIST_A));
            let scaled = FpuF32(c.f[2]) * step;
            c.g[A1][0] = sx(st.wrapping_add(UTK_HIST_B));
            c.g[A0][0] = sx(st.wrapping_add(UTK_EXCITATION));
            c.g[V0][0] = 0x3e;
            let ratio = scaled + FpuF32(0x3f85_1eb8);
            abi::set_fpr(c, 0, scaled);
            abi::set_fpr(c, 1, ratio);
            // Gains: each the previous one times the ratio.
            loop {
                c.cycles += 6;
                let at = r32(c, V1);
                let g = Addr(at.wrapping_sub(4)).read_fpu(c) * ratio;
                abi::set_fpr(c, 0, g);
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                Addr(at).write_fpu(c, g);
                c.g[V1][0] = sx(at.wrapping_add(4));
                if (c.g[V0][0] as i64) < 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_d800);
                }
            }
            c.cycles += 2;
            c.g[V1][0] = c.g[A3][0];
            c.g[V0][0] = 0xb;
            loop {
                c.cycles += 7;
                Addr(r32(c, V1)).sw(c, 0);
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                Addr(r32(c, A1)).sw(c, 0);
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(4));
                if (c.g[V0][0] as i64) < 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_d820);
                }
            }
            c.cycles += 3;
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x50c));
            c.g[V0][0] = 0x143;
            loop {
                c.cycles += 7;
                Addr(r32(c, A0)).sw(c, 0);
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                c.g[A0][0] = sx(r32(c, A0).wrapping_sub(4));
                if (c.g[V0][0] as i64) < 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_d848);
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE, LOG_RESULTS};
    use crate::difftest::{check, elf_image, scramble, Rng, DATA, GAME_GP};

    /// GetPathClassWeight: every preferred class (0, 1, 2, other) with each
    /// flag zero or set (full 64-bit values), and a random weight literal.
    #[test]
    fn computer_actor_rider_get_path_class_weight_matches() {
        check(ee::s00::f_0010d870__generated, computer_actor_rider_get_path_class_weight, 300, &|c, seed| {
            let mut r = Rng(0x10d8_7000 ^ seed as u64);
            scramble(c, &mut r);
            c.pc = 0x0010_d870;
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = sx(DATA);
            let class = match seed % 5 {
                4 => r.next_u32() as u16,
                k => k as u16,
            };
            c.bus.write16(DATA + CAR_PATH_CLASS, class);
            c.bus.write32(GAME_GP - CAR_PATH_WEIGHT_STEP_GP, r.edge_f32());
            for (i, reg) in [A1, A2, A3].into_iter().enumerate() {
                c.g[reg][0] = if (seed / 5 >> i) & 1 == 0 {
                    0
                } else if r.next_u32() % 2 == 0 {
                    1
                } else {
                    (r.next_u32() as u64) << 32 | r.next_u32() as u64
                };
            }
        });
    }

    /// StreamArray_GetTimeRemaining: negative index, empty slot, and a
    /// present buffer on which the real StreamBuffer_GetTimeRemaining runs
    /// (fixture as in `stream_buffer_get_request_status_matches`: one stream
    /// slot, SND lock/unlock and the request lookup on fakes).
    #[test]
    fn stream_array_get_time_remaining_matches() {
        const ARR: u32 = DATA + 0x20_0000;
        const LIST: u32 = ARR + 0x100;
        const BUF: u32 = ARR + 0x400;
        const STREAM: u32 = ARR + 0x1000;
        const REQ: u32 = ARR + 0x2000;
        check(ee::s08::f_002aae78__generated, stream_array_get_time_remaining, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x2a_ae78, 4);
            c.pc = 0x002a_ae78;
            let i = seed % 8;
            c.g[A0][0] = sx(ARR);
            c.g[A1][0] = if seed % 5 == 0 { sx(0x8000_0000 | r.next_u32()) } else { i as u64 };
            c.bus.write32(ARR + SAT_BUFFERS, LIST);
            c.bus.write32(LIST + 4 * i, if seed % 5 == 1 { 0 } else { BUF });
            // SND lock / unlock hooks.
            c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
            c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
            // One stream slot.
            c.bus.write8(0x0051_0000 - 0x56d2, 1);
            c.bus.write32(0x0051_0000 - 0x5f60, STREAM);
            let id = if seed % 3 == 0 { 1 + r.next_u32() % 4 } else { 0 };
            c.bus.write32(BUF + 0x8, id);
            c.bus.write32(STREAM + 0x124, if seed % 7 == 1 { 0 } else { r.next_u32() | 1 });
            c.bus.write32(STREAM + 0x11c, REQ);
            c.bus.write32(REQ + 0xc, r.next_u32());
            c.bus.write16(STREAM + 0x14, 0);
            c.bus.write8(0x0051_0000 - 0x5718 + 0x176, r.next_u32() as u8);
        });
    }

    /// Rider_ResetGrindOrientation: not grinding; main state 1 with each
    /// orientation (3, 4, other); main state 0 with each orientation (the
    /// quarter turns run the real RotateAroundAxis / RotateRoots / SinCos);
    /// another main state. cRiderAnimBase_play runs for real on an animation
    /// controller whose virtual call is a fake (as in
    /// `rider_update_timed_event_anim_matches`).
    #[test]
    fn rider_reset_grind_orientation_matches() {
        check(ee::s00::f_00115640__generated, rider_reset_grind_orientation, 200, &|c, seed| {
            elf_image(c);
            let mut r = machine(c, seed, 0x11_5640, 1);
            c.pc = 0x0011_5640;
            let rider = DATA;
            let ctrl = DATA + 0x4000;
            let state = DATA + 0x6000;
            let seqs = DATA + 0x7000;
            c.g[A0][0] = sx(rider);
            let vt = fakes::vtable(c, &mut r, &[0x20]);
            c.bus.write32(rider + RGO_ANIM, ctrl);
            c.bus.write32(ctrl + 0x58, vt);
            c.bus.write32(ctrl + 4, 0x1b6);
            c.bus.write32(ctrl + 8, 0x1b6);
            c.bus.write32(ctrl + 0x50, seqs);
            for i in 0..8 {
                c.bus.write32(LOG_RESULTS + 4 * i, 0x207);
            }
            c.bus.write32(rider + 0x77c, state);
            let main_state = match seed % 3 {
                0 => 0,
                1 => 1,
                _ => 2 + r.next_u32() % 4,
            };
            c.bus.write32(state + 0xde0, main_state);
            let side = match (seed / 3) % 5 {
                0 => 0,
                1 | 2 => 3,
                3 => 4,
                _ => 1 + r.next_u32() % 8,
            };
            c.bus.write32(rider + RGO_GRIND_SIDE, side);
            for i in 0..4 {
                c.bus.write32(rider + RGO_AXIS + 4 * i, r.f32(1.0));
                c.bus.write32(rider + 0x120 + 4 * i, r.f32(1.0));
            }
        });
    }

    /// SSXAudioSystem_IsTrackedSoundPlaying: rings of several sizes (also 0
    /// and empty), records whose key / subkey match or not, handles 0, -1
    /// and small ids with or without a live bank monitor instance (fixture
    /// as in `ssx_audio_system_update_tracked_sounds_matches`).
    #[test]
    fn ssx_audio_system_is_tracked_sound_playing_matches() {
        check(ee::s08::f_00295028__generated, ssx_audio_system_is_tracked_sound_playing, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x29_5028, 0);
            c.pc = 0x0029_5028;
            let this = DATA;
            let buf = DATA + 0x1_0000;
            let audio = DATA + 0x2_0000;
            let sys = DATA + 0x2_1000;
            let bm = sys + TSP_BANK_MONITOR;
            let insts = DATA + 0x3_0000;
            let size = [0u32, 10, 15, 23, 40, 9][(seed % 6) as usize];
            let (read, write) = if size == 0 {
                (r.next_u32() % 8, r.next_u32() % 8)
            } else {
                let read = r.next_u32() % size;
                let queued = if seed % 7 == 0 { 0 } else { r.next_u32() % size };
                (read, (read + queued) % size)
            };
            c.bus.write32(this + TSP_RING_SIZE, size);
            c.bus.write32(this + TSP_RING_SIZE + TSP_RING_WORDS, buf);
            c.bus.write32(this + TSP_RING_WRITE, write);
            c.bus.write32(this + TSP_RING_READ, read);
            let key = r.next_u32() % 3;
            let subkey = r.next_u32() % 2;
            for k in 0..size.max(32) {
                let w = match k % 5 {
                    0 => r.next_u32() % 3,
                    4 => r.next_u32() % 2,
                    1 => [u32::MAX, 0, 1, 2, 3][(r.next_u32() % 5) as usize],
                    _ => r.next_u32(),
                };
                let at = if size == 0 { buf + 4 * k } else { buf + 4 * ((read + k) % size) };
                c.bus.write32(at, w);
            }
            c.g[A1][0] = sx(key);
            c.g[A2][0] = sx(subkey);
            c.bus.write32(this + TSP_AUDIO, audio);
            c.bus.write32(audio, sys);
            c.bus.write32(bm + 0x8ec, insts);
            for id in 1..=3u32 {
                if r.next_u32() % 3 != 0 {
                    let inst = insts + 0xc0 * (id * 10 + r.next_u32() % 10);
                    c.bus.write32(inst, 1);
                    c.bus.write32(inst + 4, id);
                    c.bus.write32(inst + 0x9c, 0);
                }
            }
            c.g[A0][0] = sx(this);
        });
    }

    /// StreamFile_OnRead: memory source (end reached or not) and EAFile
    /// source (the release finds an empty request slot); then a closing
    /// stream, a finished one, a parsed end chunk, or a further read. The
    /// real callees take short paths: ParseChunks sees no complete chunk
    /// (fill before the cursor) or one end-of-stream chunk (fixture as in
    /// `stream_file_parse_chunks_matches`); StartNextRequest finds no next
    /// header; IssueRead finds the ring idle and the buffer full (state 2).
    /// The mutex syscalls go to the fakes.
    #[test]
    fn stream_file_on_read_matches() {
        check(ee::s14::f_003e0170__generated, stream_file_on_read, 240, &|c, seed| {
            let mut r = machine(c, seed, 0x3e_0170, 0);
            c.pc = 0x003e_0170;
            let file = DATA;
            let header = DATA + 0x1000;
            let classes = DATA + 0x2000;
            let tallies = DATA + 0x3000;
            let buf = DATA + 0x4000;
            let issue = DATA + 0x5000;
            let end_id = 0x454e_4421u32;
            c.g[A2][0] = sx(file);
            c.bus.write32(file + SFR_HEADER, header);
            // 0: closing, 1: maybe finished, 2: end chunk, 3: further read.
            let case = seed % 4;
            let state = if case == 0 { 4 } else { [0u32, 2, 3][(r.next_u32() % 3) as usize] };
            c.bus.write32(header + SFR_HDR_STATE, state);
            c.bus.write32(header + 0xc, 0);
            c.bus.write32(header + 0x11c, end_id);
            let memory = seed % 3 == 0;
            c.bus.write32(header + SFR_HDR_SOURCE, if memory { 1 } else { r.next_u32() % 4 + 2 });
            // Case 3 is never finished: a memory source short of its end,
            // an EAFile read of the full (zero) length.
            let len = if case == 3 && !memory { 0 } else { r.next_u32() & 0xfff };
            c.bus.write32(file + SFR_REQUEST_LEN, len);
            c.bus.write32(file + SFR_BYTES_READ, r.next_u32() & 0xffff);
            let size = if case == 3 { 0x10_0000 } else { r.next_u32() & 0x1_ffff };
            c.bus.write32(header + SFR_HDR_SIZE, size);
            // EAFile request slot table (zeroed entries: nothing to release).
            c.bus.write32(0x0051_9ad8, DATA + 0xf_0000);
            c.bus.write32(file + SFR_REQUEST, r.next_u32() & 0xff);
            // Classifier and tallies for the end chunk.
            c.bus.write32(file + 0x1c, classes);
            c.bus.write32(file + 0x20, 1);
            c.bus.write32(classes, 0xffff_ffff);
            c.bus.write32(classes + 4, end_id);
            c.bus.write32(classes + 8, 3);
            c.bus.write32(file + 0x24, tallies);
            c.bus.write32(file + 0x44, r.next_u32() & 0xfff);
            c.bus.write32(file + 0x48, 1);
            c.bus.write32(file + 0x4c, r.next_u32() & 0xfff);
            // Mutex: owner (never the fake thread id), depth, semaphore.
            c.bus.write32(file + 0x8, 0x7700_0000 | r.next_u32() & 0xffff);
            c.bus.write32(file + 0xc, r.next_u32() & 3);
            c.bus.write32(file + 0x10, r.next_u32() & 0xff);
            // Cursor (+0x54) and fill (+0x58, OnRead adds the byte count):
            // the end chunk, or the fill short of the cursor.
            c.bus.write32(file + 0x54, buf);
            c.bus.write32(file + 0x50, buf);
            let n = if memory { len } else { 0 };
            let fill = if case == 2 {
                c.bus.write32(buf, end_id);
                c.bus.write32(buf + 4, 8);
                buf + 8
            } else {
                buf - 0x10
            };
            c.bus.write32(file + SFR_FILL, fill.wrapping_sub(n));
            // IssueRead: ring idle, buffer full.
            c.bus.write32(file + 0x5c, issue);
            c.bus.write32(issue + 0xc, 0);
            c.bus.write32(file + 0x17c, 0x1000);
            c.bus.write32(file + SFR_READ_AHEAD, r.next_u32() & 0xff);
        });
    }

    /// SND_UtkParseHeader: without and with the header read (the real
    /// SND_UtkReadBits on random stream bytes; its mask table comes from the
    /// ELF image), random state contents to be overwritten.
    #[test]
    fn snd_utk_parse_header_matches() {
        check(ee::s14::f_003cd6f0__generated, snd_utk_parse_header, 200, &|c, seed| {
            elf_image(c);
            let mut r = machine(c, seed, 0x3c_d6f0, 0);
            c.pc = 0x003c_d6f0;
            let data = DATA + 0x100 + seed % 4;
            let st = DATA + 0x1000;
            crate::difftest::fill(c, &mut r, DATA + 0x100, 8);
            crate::difftest::fill(c, &mut r, st, 0x6a0 / 4);
            c.g[A1][0] = sx(data);
            c.g[A2][0] = sx(st);
            c.g[A3][0] = if seed % 4 == 0 { 0 } else { (r.next_u32() as u64) << 32 | 1 };
        });
    }
}
