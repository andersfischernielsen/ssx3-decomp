//! Staging wave 212 (oc304): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0029_6868,
        ssx_audio_system_start_wipeout_loop,
        "SSXAudioSystem_StartWipeoutLoop",
    ),
];

// ---------------------------------------------------------------------------
// SSXAudioSystem_StartWipeoutLoop (0x296868).
//
// The first of two wipeout loops: the same code as
// SSXAudioSystem_StartWipeoutLoop2 (0x296e80) with its own handle slot
// (+0x774), record kind (0x33) and record default (0x4a36b0).

/// `rider->audio` (rider+0x80): the rider's audio state.
pub(crate) const WOL_RIDER_AUDIO: u32 = 0x80;

/// Rider audio: embedded object (vtable at +0) asked whether the rider is
/// local (slot 0x48) and for its position (slot 0x28).
pub(crate) const WOL_AUDIO_OBJ: u32 = 0x6c0;

pub(crate) const WOL_SLOT_IS_LOCAL: u32 = 0x48;

pub(crate) const WOL_SLOT_GET_POS: u32 = 0x28;

/// Rider audio: wipeout intensity (float) fed to the volume curve.
pub(crate) const WOL_INTENSITY: u32 = 0x770;

/// Rider audio: the wipeout-loop voice handle (< 0: not playing).
pub(crate) const WOL_HANDLE: u32 = 0x774;

/// Rider audio: listener index, "has a listener", "listener valid".
pub(crate) const WOL_LISTENER_INDEX: u32 = 0x870;

pub(crate) const WOL_HAS_LISTENER: u32 = 0x874;

pub(crate) const WOL_LISTENER_VALID: u32 = 0x87c;

/// Rider audio: the sound source position (record source pointer).
pub(crate) const WOL_SOURCE_POS: u32 = 0x110;

/// SSXAudioSystem: the channel holder whose first word is the bank monitor
/// the voice record is completed in.
pub(crate) const WOL_CHANNEL: u32 = 0x118;

/// Bank monitor: record write cursor, record descriptor, parameter block,
/// per-record source pointers.
pub(crate) const WOL_MON_CURSOR: u32 = 0x1f4;

pub(crate) const WOL_MON_DESC: u32 = 0x1d8;

pub(crate) const WOL_MON_PARAMS: u32 = 0x1dc;

pub(crate) const WOL_MON_SOURCE: u32 = 0x1fc;

/// The record kind written into the audio system's kind array (0x33).
pub(crate) const WOL_RECORD_KIND: u64 = 0x33;

/// Rodata: the five-point volume curve, the 8-byte record default, and the
/// speech mode byte (2: replay check needed).
pub(crate) const WOL_VOLUME_CURVE: u32 = 0x0048_2da0;

pub(crate) const WOL_RECORD_DEFAULT: u32 = 0x004a_36b0;

pub(crate) const WOL_SPEECH_MODE: u32 = 0x0053_5c11;

/// Callees (all reached through the function table).
pub(crate) const WOL_EVAL_CURVE5: u32 = 0x0029_0b58;

pub(crate) const WOL_GET_BE: u32 = 0x0014_dc80;

pub(crate) const WOL_GET_INTERFACE: u32 = 0x0014_dd58;

pub(crate) const WOL_IS_REPLAY_PLAYING: u32 = 0x0028_8ae0;

pub(crate) const WOL_FIND_VIEW_IN_RANGE: u32 = 0x0028_8b40;

pub(crate) const WOL_MEMSET: u32 = 0x0041_6210;

pub(crate) const WOL_GET_CHANNEL_GAIN_PTR: u32 = 0x0028_7968;

pub(crate) const WOL_GET_RIDER_LISTENER_INDEX: u32 = 0x0028_8a20;

pub(crate) const WOL_PLAY_PUSHED_SOUND: u32 = 0x0029_06b8;

/// `reg + off` as an address.
#[inline(always)]
fn wol_ea(c: &Cpu, r: usize, off: u32) -> u32 {
    (c.g[r][0] as u32).wrapping_add(off)
}

/// `lw rd, off(rb)`.
#[inline(always)]
fn wol_lw(c: &mut Cpu, rd: usize, rb: usize, off: u32) {
    let a = wol_ea(c, rb, off);
    c.g[rd][0] = c.lw(a);
}

/// `sll rd, rd, shift; addu rd, rd, rb` (an array element address).
#[inline(always)]
fn wol_index(c: &mut Cpu, rd: usize, shift: u32, rb: usize) {
    c.g[rd][0] = sx(((c.g[rd][0] as u32) << shift).wrapping_add(c.g[rb][0] as u32));
}

/// Store `val` into element `cursor` of the monitor's (s0) record array at
/// descriptor (s1) field `field` (element size `1 << shift`, at `+off`),
/// with the original's register pair `ri`/`rb`.
fn wol_record_store(
    c: &mut Cpu,
    ri: usize,
    rb: usize,
    field: u32,
    shift: u32,
    off: u32,
    val: u64,
) {
    wol_lw(c, ri, S0, WOL_MON_CURSOR);
    wol_lw(c, rb, S1, field);
    wol_index(c, ri, shift, rb);
    let a = wol_ea(c, ri, off);
    c.sw(a, val);
}

/// Unaligned doubleword copy through `regs` (`ldl/ldr` all, then
/// `sdl/sdr` all).
fn wol_copy_dwords(c: &mut Cpu, regs: &[usize], src: u32, dst: u32) {
    for (i, &r) in regs.iter().enumerate() {
        let s = src.wrapping_add(8 * i as u32);
        c.ldl(r, s.wrapping_add(7));
        c.ldr(r, s);
    }
    for (i, &r) in regs.iter().enumerate() {
        let d = dst.wrapping_add(8 * i as u32);
        c.sdl(r, d.wrapping_add(7));
        c.sdr(r, d);
    }
}

/// `void SSXAudioSystem_StartWipeoutLoop(SSXAudioSystem *audio, Rider
/// *rider)` (0x296868): start the rider's wipeout loop unless it is already
/// playing. A rider that is not the local one (virtual slot 0x48) and has
/// no valid listener must be within 100 units of a view. The volume comes
/// from the five-point curve over the wipeout intensity (clamped to
/// 0..127); a voice record (kind 0x33) is pushed on the audio system's bank
/// monitor (gain, pan, source, listener, replay flag) and played, and the
/// handle is stored in the rider's audio state (+0x774).
pub fn ssx_audio_system_start_wipeout_loop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S3, 0x80);
        frame.save(c, S5, 0x60);
        frame.save(c, S0, 0xb0);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S1, 0xa0);
        frame.save(c, S2, 0x90);
        frame.save(c, S4, 0x70);
        frame.save(c, S6, 0x50);
        frame.save_ra(c, 0x40);
        wol_lw(c, A1, S5, WOL_RIDER_AUDIO);
        wol_lw(c, V0, A1, WOL_HANDLE);
        c.g[S3][0] = c.g[A0][0];
        if (c.g[V0][0] as i64) >= 0 {
            // Already playing.
            c.cycles += 1;
            frame.restore(c, S0, 0xb0);
        } else if wol_should_start(c, frame)? {
            wol_start(c)?;
            c.cycles += 1;
            frame.restore(c, S0, 0xb0);
        }
        // (Out of range: s0 was restored by the range check.)
        c.cycles += 9;
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore(c, S6, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// Whether the loop should start: a local rider (slot 0x48 true) or one
/// without a valid listener is checked against the views (100 units around
/// its slot-0x28 position); a rider with a valid listener starts at once.
/// Leaves the rider audio in v1 when it starts.
fn wol_should_start(c: &mut Cpu, frame: Frame) -> Result<bool, crate::call::Unwound> {
    c.cycles += 8;
    wol_lw(c, V1, A1, WOL_AUDIO_OBJ);
    c.g[A1][0] = sx(wol_ea(c, A1, WOL_AUDIO_OBJ));
    c.g[A0][0] = c.lh(wol_ea(c, V1, WOL_SLOT_IS_LOCAL));
    wol_lw(c, V0, V1, WOL_SLOT_IS_LOCAL + 4);
    let method = c.g[V0][0] as u32;
    c.g[A0][0] = sx(wol_ea(c, A1, c.g[A0][0] as u32));
    call_ptr(c, method, 0x0029_68b8)?;
    let check_views = if c.g[V0][0] != 0 {
        wol_lw(c, A0, S5, WOL_RIDER_AUDIO);
        true
    } else {
        c.cycles += 4;
        wol_lw(c, V0, S5, WOL_RIDER_AUDIO);
        wol_lw(c, V1, V0, WOL_LISTENER_VALID);
        if c.g[V1][0] != 0 {
            wol_lw(c, V1, S5, WOL_RIDER_AUDIO);
            false
        } else {
            c.cycles += 1;
            wol_lw(c, A0, S5, WOL_RIDER_AUDIO);
            true
        }
    };
    if !check_views {
        return Ok(true);
    }
    c.cycles += 13;
    wol_lw(c, V1, A0, WOL_AUDIO_OBJ);
    c.g[A0][0] = sx(wol_ea(c, A0, WOL_AUDIO_OBJ));
    c.g[A1][0] = c.lh(wol_ea(c, V1, WOL_SLOT_GET_POS));
    wol_lw(c, V0, V1, WOL_SLOT_GET_POS + 4);
    let method = c.g[V0][0] as u32;
    c.g[A0][0] = sx(wol_ea(c, A0, c.g[A1][0] as u32));
    call_ptr(c, method, 0x0029_68ec)?;
    // FindViewInRange(audio, pos, 100.0f)
    c.g[AT][0] = 0x42c8_0000;
    c.mtc1(AT, 12);
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, WOL_FIND_VIEW_IN_RANGE, 0x0029_6900)?;
    let none = (c.g[V0][0] as i64) < 0;
    frame.restore(c, S0, 0xb0);
    if none {
        return Ok(false);
    }
    c.cycles += 1;
    wol_lw(c, V1, S5, WOL_RIDER_AUDIO);
    Ok(true)
}

/// Start the loop: volume from the curve, the replay flag, the voice
/// record, play it and keep the handle.
fn wol_start(c: &mut Cpu) -> crate::call::Flow {
    // Volume = EvalCurve5(curve copied to the stack, intensity).
    c.cycles += 38;
    c.g[V0][0] = 0x48_0000;
    c.g[A3][0] = sx(WOL_VOLUME_CURVE);
    let curve = sp(c);
    wol_copy_dwords(c, &[A0, A1, A2], WOL_VOLUME_CURVE, curve);
    wol_copy_dwords(c, &[A0, A1], WOL_VOLUME_CURVE + 0x18, curve + 0x18);
    c.g[S6][0] = 0;
    c.g[A0][0] = c.g[SP][0];
    let intensity = wol_ea(c, V1, WOL_INTENSITY);
    c.lwc1(12, intensity);
    call_ptr(c, WOL_EVAL_CURVE5, 0x0029_6974)?;
    c.cvt_w_s(1, 0);
    c.mfc1(S4, 1);
    wol_lw(c, A2, S5, WOL_RIDER_AUDIO);
    c.g[A0][0] = 0x7f;
    c.g[V1][0] = u64::MAX;
    wol_lw(c, A1, A2, WOL_HAS_LISTENER);
    c.g[A3][0] = 0;
    // Clamp the volume to 0..=127.
    c.g[V0][0] = ((c.g[S4][0] as i64) < 128) as u64;
    if c.g[V0][0] == 0 {
        c.g[S4][0] = c.g[A0][0];
    }
    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[S4][0] as i64)) as u64;
    if c.g[V1][0] == 0 {
        c.g[S4][0] = 0;
    }
    if c.g[A1][0] != 0 {
        c.cycles += 2;
        wol_lw(c, V0, A2, WOL_LISTENER_VALID);
        c.g[A3][0] = (c.g[V0][0] != 0) as u64;
    }
    wol_replay_flag(c)?;
    wol_push_record(c)?;
    wol_finish_record(c)?;
    // handle = PlayPushedSound(audio)
    c.cycles += 4;
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, WOL_PLAY_PUSHED_SOUND, 0x0029_6de4)?;
    wol_lw(c, V1, S5, WOL_RIDER_AUDIO);
    let a = wol_ea(c, V1, WOL_HANDLE);
    c.sw(a, c.g[V0][0]);
    Ok(())
}

/// s6 = "a replay is playing" for a rider with a valid listener in speech
/// mode 2 (the BE library is fetched first); then s0 = the bank monitor.
fn wol_replay_flag(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 2;
    if c.g[A3][0] != 0 {
        c.cycles += 10;
        call_ptr(c, WOL_GET_BE, 0x0029_69bc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, WOL_GET_INTERFACE, 0x0029_69c8)?;
        c.g[V1][0] = 0x53_0000;
        c.g[A0][0] = 2;
        c.g[V0][0] = c.lb(WOL_SPEECH_MODE);
        if c.g[V0][0] == c.g[A0][0] {
            c.cycles += 4;
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, WOL_IS_REPLAY_PLAYING, 0x0029_69e4)?;
            c.g[S6][0] = (c.g[V0][0] < 1) as u64;
        }
    }
    wol_lw(c, S0, S3, 0);
    Ok(())
}

/// Push a voice record on the bank monitor in s0 (advance the cursor, copy
/// the parameter block, clear/default the per-record fields), then write
/// the gain (s4) into the channel monitor and compute its pan byte
/// (clamped to 0..127; stored by `wol_finish_record`).
fn wol_push_record(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 154;
    c.g[S2][0] = 0x18;
    c.g[A1][0] = 0;
    c.g[A2][0] = 8;
    // ++cursor
    wol_lw(c, V0, S0, WOL_MON_CURSOR);
    c.g[S1][0] = sx(wol_ea(c, S0, WOL_MON_DESC));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    let a = wol_ea(c, S0, WOL_MON_CURSOR);
    c.sw(a, c.g[V0][0]);
    // records[cursor] (0x18 bytes) = params
    wol_lw(c, V1, S0, WOL_MON_CURSOR);
    wol_lw(c, V0, S1, 0x20);
    c.mult(0, V1, V1, S2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let (params, rec) = (wol_ea(c, S0, WOL_MON_PARAMS), c.g[V1][0] as u32);
    wol_copy_dwords(c, &[V0, A0, A3], params, rec);
    // source[cursor] = 0
    wol_lw(c, V0, S0, WOL_MON_CURSOR);
    wol_index(c, V0, 2, S0);
    let a = wol_ea(c, V0, WOL_MON_SOURCE);
    c.sw(a, 0);
    wol_record_store(c, V1, V0, 0x34, 2, 0, 0);
    // field38[cursor] = (s8) records[cursor].byte0
    wol_lw(c, A3, S0, WOL_MON_CURSOR);
    wol_lw(c, A0, S0, WOL_MON_CURSOR);
    wol_lw(c, T0, S1, 0x20);
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
    c.mult(0, A0, A0, S2);
    wol_lw(c, V0, S1, 0x38);
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
    c.g[V1][0] = c.lb(c.g[A0][0] as u32);
    c.sw(c.g[A3][0] as u32, c.g[V1][0]);
    wol_record_store(c, V0, V1, 0x3c, 2, 0, 0);
    // memset(&desc->field40[cursor], 0, 8)
    wol_lw(c, A0, S0, WOL_MON_CURSOR);
    wol_index(c, A0, 3, S1);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    call_ptr(c, WOL_MEMSET, 0x0029_6abc)?;
    // Per-record defaults.
    wol_lw(c, V1, S0, WOL_MON_CURSOR);
    c.g[A0][0] = 0x64;
    wol_lw(c, V0, S1, 0x60);
    c.g[A1][0] = 0x5a;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[A2][0] = 0x32;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A3][0] = 0x7f;
    c.sw(c.g[V1][0] as u32, 0);
    c.g[T0][0] = 1;
    wol_record_store(c, V0, V1, 0x64, 2, 0, 0);
    wol_record_store(c, V1, V0, 0x68, 2, 0, 0);
    wol_record_store(c, V0, V1, 0x6c, 2, 0, 0);
    wol_record_store(c, V1, V0, 0x70, 4, 4, 0);
    let (a0, a1, a2, a3, t0) = (c.g[A0][0], c.g[A1][0], c.g[A2][0], c.g[A3][0], c.g[T0][0]);
    wol_record_store(c, V0, V1, 0x70, 4, 0, a0);
    wol_record_store(c, V1, V0, 0x70, 4, 8, a1);
    wol_record_store(c, V0, V1, 0x70, 4, 0xc, a2);
    wol_record_store(c, V1, V0, 0x74, 2, 0, 0);
    wol_record_store(c, V0, V1, 0x80, 2, 0, 0);
    wol_record_store(c, V1, V0, 0x84, 2, 0, 0);
    wol_record_store(c, V0, V1, 0x78, 2, 0, a3);
    wol_record_store(c, V1, V0, 0x7c, 2, 0, t0);
    // audio->array4[cursor] = 0; audio->kinds[cursor] = 0x33
    wol_lw(c, A0, S3, 0);
    wol_lw(c, V1, S3, 4);
    wol_lw(c, V0, A0, WOL_MON_CURSOR);
    wol_index(c, V0, 2, V1);
    c.sw(c.g[V0][0] as u32, 0);
    wol_lw(c, V1, S3, 0);
    wol_lw(c, A0, S3, 8);
    wol_lw(c, V0, V1, WOL_MON_CURSOR);
    c.g[V1][0] = WOL_RECORD_KIND;
    wol_index(c, V0, 2, A0);
    c.sw(c.g[V0][0] as u32, c.g[V1][0]);
    // Channel monitor: gain[cursor] = volume.
    wol_lw(c, A0, S3, WOL_CHANNEL);
    wol_lw(c, A1, A0, 0);
    wol_lw(c, V0, A1, WOL_MON_CURSOR);
    c.g[A2][0] = sx(wol_ea(c, A1, WOL_MON_DESC));
    wol_lw(c, V1, A2, 0x38);
    wol_index(c, V0, 2, V1);
    c.sw(c.g[V0][0] as u32, c.g[S4][0]);
    // Pan byte of the channel record = clamp(gain[desc.cursor], 0, 127).
    wol_lw(c, A0, A1, WOL_MON_CURSOR);
    wol_lw(c, V1, A2, 0x1c);
    wol_lw(c, A3, A2, 0x38);
    c.mult(0, A0, A0, S2);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    wol_lw(c, A1, A2, 0x20);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
    c.g[V0][0] = ((c.g[V0][0] as i64) < 127) as u64;
    let at_most = c.g[V0][0] == 0;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    let mut negative = false;
    if !at_most {
        c.cycles += 6;
        wol_lw(c, V0, A2, 0x1c);
        wol_index(c, V0, 2, A3);
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        negative = (c.g[V1][0] as i64) < 0;
        c.g[V0][0] = 0;
    }
    if !negative {
        c.cycles += 7;
        wol_lw(c, V0, A2, 0x1c);
        wol_index(c, V0, 2, A3);
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        c.g[V1][0] = ((c.g[V1][0] as i64) < 127) as u64;
        if c.g[V1][0] == 0 {
            c.g[V0][0] = 0x7f;
        } else {
            c.cycles += 4;
            wol_lw(c, V0, A2, 0x1c);
            wol_index(c, V0, 2, A3);
            c.g[V0][0] = c.lbu(c.g[V0][0] as u32);
        }
    }
    Ok(())
}

/// Store the pan byte, then fill the channel monitor's record: gain slot
/// pointer, source position, listener index, rider, default, and the
/// replay flag with the rider's listener index.
fn wol_finish_record(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 23;
    c.sb(c.g[A0][0] as u32, c.g[V0][0]);
    // gainPtr[cursor] = GetChannelGainPtr(audio, 5, 0)
    c.g[A1][0] = 5;
    c.g[A0][0] = c.g[S3][0];
    c.g[A2][0] = 0;
    wol_lw(c, S0, S3, WOL_CHANNEL);
    call_ptr(c, WOL_GET_CHANNEL_GAIN_PTR, 0x0029_6cb0)?;
    wol_lw(c, A0, S0, 0);
    c.g[A2][0] = 0;
    wol_lw(c, V1, A0, WOL_MON_CURSOR);
    wol_lw(c, A1, A0, 0x214);
    wol_index(c, V1, 2, A1);
    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
    // source[cursor] = &rider->audio->sourcePos
    wol_lw(c, V0, S3, WOL_CHANNEL);
    wol_lw(c, A0, S5, WOL_RIDER_AUDIO);
    wol_lw(c, V1, V0, 0);
    c.g[A0][0] = sx(wol_ea(c, A0, WOL_SOURCE_POS));
    wol_lw(c, V0, V1, WOL_MON_CURSOR);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let a = wol_ea(c, V1, WOL_MON_SOURCE);
    c.sw(a, c.g[A0][0]);
    let no_rider = c.g[S5][0] == 0;
    wol_lw(c, S0, S3, WOL_CHANNEL);
    if !no_rider {
        c.cycles += 1;
        wol_lw(c, A2, S5, WOL_RIDER_AUDIO);
    }
    // listener[cursor] = GetRiderListenerIndex(audio, rider->audio)
    c.cycles += 40;
    c.g[A1][0] = c.g[A2][0];
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, WOL_GET_RIDER_LISTENER_INDEX, 0x0029_6d04)?;
    wol_lw(c, A0, S0, 0);
    c.g[A2][0] = 0x4a_0000;
    wol_lw(c, V1, A0, WOL_MON_CURSOR);
    wol_lw(c, A1, A0, 0x23c);
    wol_index(c, V1, 2, A1);
    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
    // rider[cursor] = rider
    wol_lw(c, A0, S3, WOL_CHANNEL);
    wol_lw(c, V1, A0, 0);
    wol_lw(c, V0, V1, WOL_MON_CURSOR);
    wol_lw(c, A0, V1, 0x238);
    wol_index(c, V0, 2, A0);
    c.sw(c.g[V0][0] as u32, c.g[S5][0]);
    // desc.field40[cursor] (8 bytes) = default (through the stack)
    wol_lw(c, V1, S3, WOL_CHANNEL);
    wol_lw(c, A0, V1, 0);
    c.g[A3][0] = sx(wol_ea(c, A2, WOL_RECORD_DEFAULT & 0xffff));
    let tmp = sp(c).wrapping_add(0x30);
    wol_copy_dwords(c, &[V0], WOL_RECORD_DEFAULT, tmp);
    c.g[V1][0] = sx(wol_ea(c, A0, WOL_MON_DESC));
    wol_lw(c, V0, A0, WOL_MON_CURSOR);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let dst = wol_ea(c, V1, 0x40);
    wol_copy_dwords(c, &[T0], tmp, dst);
    // replay[cursor] = 0
    wol_lw(c, A0, S3, WOL_CHANNEL);
    wol_lw(c, V1, A0, 0);
    wol_lw(c, V0, V1, WOL_MON_CURSOR);
    wol_lw(c, A0, V1, 0x254);
    wol_index(c, V0, 2, A0);
    let replaying = c.g[S6][0] != 0;
    c.sw(c.g[V0][0] as u32, 0);
    if replaying {
        // replay[cursor] = 1; replayListener[cursor] = rider->audio->listenerIndex
        c.cycles += 17;
        wol_lw(c, A0, S3, WOL_CHANNEL);
        c.g[A1][0] = 1;
        wol_lw(c, V1, A0, 0);
        wol_lw(c, V0, V1, WOL_MON_CURSOR);
        wol_lw(c, A0, V1, 0x258);
        wol_index(c, V0, 2, A0);
        c.sw(c.g[V0][0] as u32, c.g[A1][0]);
        wol_lw(c, V1, S3, WOL_CHANNEL);
        wol_lw(c, A0, S5, WOL_RIDER_AUDIO);
        wol_lw(c, A1, V1, 0);
        wol_lw(c, A2, A0, WOL_LISTENER_INDEX);
        wol_lw(c, V0, A1, WOL_MON_CURSOR);
        wol_lw(c, V1, A1, 0x25c);
        wol_index(c, V0, 2, V1);
        c.sw(c.g[V0][0] as u32, c.g[A2][0]);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// SSXAudioSystem_StartWipeoutLoop on the MIDI-bank fixture of the
    /// PlaySound tests (real record arrays, real GetChannelGainPtr,
    /// GetRiderListenerIndex and PlayPushedSound), a world with one view
    /// (real FindViewInRange: the view near or far from the rider's position
    /// from a fake slot-0x28 method), the local-rider method a fake (true)
    /// or a real `return 0` leaf, listener flags on/off, speech mode 2 or
    /// not, replay playing or not, intensities below/inside/above the curve
    /// (volume clamped at 0 and 127), and the loop already playing.
    #[test]
    fn ssx_audio_system_start_wipeout_loop_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
        use crate::difftest::{put_vec, put_words, DATA, GAME_GP};
        use crate::game::sound::soundsys::tests::midi_bank;
        const WOL_T_AUDIO: u32 = DATA + 0x100; // midi_bank's SSXAudioSystem
        const WOL_T_RIDER: u32 = DATA + 0x2_0000;
        const WOL_T_RIDER_AUDIO: u32 = DATA + 0x2_1000;
        const WOL_T_VTABLE: u32 = DATA + 0x2_2000;
        const WOL_T_POS: u32 = DATA + 0x2_3000;
        const WOL_T_WORLD: u32 = DATA + 0x2_4000;
        const WOL_T_GAME_MODULE: u32 = DATA + 0x2_5000;
        const WOL_T_VIEW_LIST: u32 = DATA + 0x2_6000;
        const WOL_T_VIEW: u32 = DATA + 0x2_7000;
        const WOL_T_REPLAY: u32 = DATA + 0x2_8000;
        const WOL_T_BE_LIB: u32 = DATA + 0x2_9000;
        const WOL_T_BE_IFACE: u32 = DATA + 0x2_a000;
        const WOL_T_BE_VTABLE: u32 = DATA + 0x2_b000;
        const WOL_T_RETURN_ZERO: u32 = 0x0014_06c0;
        let coord =
            |r: &mut crate::difftest::Rng| ((r.next_u32() % 40_000) as f32 - 20_000.0).to_bits();
        covered(
            ee::s08::f_00296868__generated,
            ssx_audio_system_start_wipeout_loop,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x29_6868, 2);
                c.pc = 0x0029_6868;
                midi_bank(c, &mut r, seed);
                c.g[A0][0] = sx(WOL_T_AUDIO);
                c.g[A1][0] = sx(WOL_T_RIDER);
                c.bus.write32(WOL_T_RIDER + WOL_RIDER_AUDIO, WOL_T_RIDER_AUDIO);
                // The loop handle: already playing on some seeds.
                let handle = if seed % 6 == 0 { seed } else { u32::MAX };
                c.bus.write32(WOL_T_RIDER_AUDIO + WOL_HANDLE, handle);
                // Audio object vtable: slot 0x48 a fake (true) or a real
                // `return 0`; slot 0x28 a fake returning the position.
                c.bus.write32(WOL_T_RIDER_AUDIO + WOL_AUDIO_OBJ, WOL_T_VTABLE);
                c.bus
                    .write16(WOL_T_VTABLE + WOL_SLOT_IS_LOCAL, (r.next_u32() & 0x30) as u16);
                let is_local = if seed % 2 == 0 { FAKE } else { WOL_T_RETURN_ZERO };
                c.bus.write32(WOL_T_VTABLE + WOL_SLOT_IS_LOCAL + 4, is_local);
                c.bus
                    .write16(WOL_T_VTABLE + WOL_SLOT_GET_POS, (r.next_u32() & 0x30) as u16);
                c.bus.write32(WOL_T_VTABLE + WOL_SLOT_GET_POS + 4, FAKE + 0x10);
                c.bus.write32(LOG_RESULTS, WOL_T_POS);
                c.bus.write32(LOG_RESULTS + 4, WOL_T_POS);
                let p = [coord(&mut r), coord(&mut r), coord(&mut r), 0];
                put_vec(c, WOL_T_POS, p);
                // Intensity -5..45 over the curve's 0..40 range.
                let intensity = ((r.next_u32() % 500) as f32 / 10.0 - 5.0).to_bits();
                c.bus.write32(WOL_T_RIDER_AUDIO + WOL_INTENSITY, intensity);
                c.bus
                    .write32(WOL_T_RIDER_AUDIO + WOL_LISTENER_INDEX, r.next_u32() % 4);
                c.bus
                    .write32(WOL_T_RIDER_AUDIO + WOL_HAS_LISTENER, (seed % 3 != 0) as u32);
                let valid = if (seed / 3) % 3 != 0 {
                    1 + r.next_u32() % 3
                } else {
                    0
                };
                c.bus.write32(WOL_T_RIDER_AUDIO + WOL_LISTENER_VALID, valid);
                // The volume curve (times, then values) and the record default.
                let curve = [
                    0.0f32, 10.0, 20.0, 30.0, 40.0, -40.0, 30.0, 90.0, 127.5, 300.0,
                ];
                put_words(c, WOL_VOLUME_CURVE, &curve.map(f32::to_bits));
                put_words(c, WOL_RECORD_DEFAULT, &[r.next_u32(), r.next_u32()]);
                c.bus
                    .write8(WOL_SPEECH_MODE, if seed % 4 == 3 { 1 } else { 2 });
                // World -> game module -> one view near or far.
                c.bus.write32(GAME_GP - 0x848, WOL_T_WORLD);
                c.bus.write32(WOL_T_WORLD + 0x84, WOL_T_GAME_MODULE);
                c.bus.write32(WOL_T_GAME_MODULE + 0x84, WOL_T_VIEW_LIST);
                c.bus.write32(WOL_T_VIEW_LIST + 0x10, 1);
                c.bus.write32(WOL_T_VIEW_LIST + 4, WOL_T_VIEW);
                let near = seed % 5 < 2;
                let v = if near {
                    let d = |x: u32| (f32::from_bits(x) + 50.0).to_bits();
                    [d(p[0]), d(p[1]), p[2], 0]
                } else {
                    [coord(&mut r), coord(&mut r), coord(&mut r), 0]
                };
                put_vec(c, WOL_T_VIEW + 0x20, v);
                // Replay state (GameModule+0x28 -> replay mode word).
                let replay = if seed % 7 < 3 { 0 } else { WOL_T_REPLAY };
                c.bus.write32(WOL_T_GAME_MODULE + 0x28, replay);
                c.bus.write32(WOL_T_REPLAY, r.next_u32() % 14);
                // BELibrary singleton, interface 0 with a real `return 0` slot.
                c.bus.write32(GAME_GP - 0x1f04, WOL_T_BE_LIB);
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.bus.write32(0x0053_08a8, WOL_T_BE_IFACE);
                c.bus.write32(WOL_T_BE_IFACE + 4, WOL_T_BE_VTABLE);
                c.bus.write16(WOL_T_BE_VTABLE + 8, 0);
                c.bus.write32(WOL_T_BE_VTABLE + 0xc, WOL_T_RETURN_ZERO);
            },
        );
    }
}
