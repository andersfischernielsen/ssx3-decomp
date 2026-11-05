//! Staging wave 207 (oc253): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi;
use crate::abi::{r32, ret};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, Y, Z};
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0011_9898, rider_metrix_set_spin_amount, "RiderMetrix_SetSpinAmount"),
    (0x0010_4940, rider_anim_seq_type01_update, "RiderAnim_UpdateSeqType01"),
    (0x003c_3ee0, snd_channel_start_voices, "SND_ChannelStartVoices"),
    (0x0031_47f0, anim_seq_list_save, "AnimSeqList_Save"),
    (0x0012_e778, rider_bump_sub_handle_input, "RiderBumpSub_HandleInput"),
    (0x002e_f0a0, world_painter_get_weather28, "WorldPainter_GetWeather28"),
    (0x002e_e930, world_painter_get_weather90, "WorldPainter_GetWeather90"),
    (0x002e_e4c0, world_painter_get_weather10, "WorldPainter_GetWeather10"),
    (0x002d_ba08, crowd_anim2d_step_frame, "cCrowdAnim2D_StepFrame"),
];

// ---------------------------------------------------------------------------
// RiderMetrix_SetSpinAmount (0x119898)

/// RiderMetrix: the spin amount of the current trick (float).
const SPIN_METRIX_SPIN_AMOUNT: u32 = 0x34;
/// RiderMetrix: the running rotation total (spin and flip, weighted).
const SPIN_METRIX_ROTATION_TOTAL: u32 = 0x14;
/// `gp`-relative weight of a spin degree in the rotation total (.sdata).
const SPIN_WEIGHT_GP: u32 = 0x7a14;

/// `void RiderMetrix_SetSpinAmount(RiderMetrix* m, float spin)` (0x119898):
/// replaces the stored spin amount and moves the rotation total by the
/// weighted difference of the magnitudes, clamped at zero:
/// `total = max(total - |old|*k + |spin|*k, 0)`. Leaves f0 = f2 = 0,
/// f3 = the total minus the old share, f1 = the new total.
pub fn rider_metrix_set_spin_amount(c: &mut Cpu) {
    c.cycles += 15;
    let m = Addr(abi::arg(c, 0));
    let gp = r32(c, GP);
    let k = Addr(gp.wrapping_sub(SPIN_WEIGHT_GP)).read_fpu(c);
    let spin = FpuF32(c.f[12]);
    let old = m.off(SPIN_METRIX_SPIN_AMOUNT).read_fpu(c);
    let total = m.off(SPIN_METRIX_ROTATION_TOTAL).read_fpu(c);
    let removed = old.abs() * k;
    m.off(SPIN_METRIX_SPIN_AMOUNT).write_fpu(c, spin);
    let added = spin.abs() * k;
    let less = total - removed;
    let new_total = (less + added).max(FpuF32::ZERO);
    c.f[3] = less.0;
    c.f[2] = 0;
    c.f[0] = 0;
    c.f[1] = new_total.0;
    m.off(SPIN_METRIX_ROTATION_TOTAL).write_fpu(c, new_total);
    ret(c);
}

// ---------------------------------------------------------------------------
// RiderAnim_UpdateSeqType01 (0x104940)

/// `AnimSequence_AdvanceTrack(seq, 0, step)`: nonzero when the track ended.
const SEQ01_ADVANCE_TRACK: u32 = 0x0031_35b0;
/// `AnimSequence_UpdateWeightFade(seq, step)`: nonzero when faded out.
const SEQ01_UPDATE_WEIGHT_FADE: u32 = 0x0031_3800;
/// `AnimSeqList_Remove(list, seq)`.
const SEQ01_LIST_REMOVE: u32 = 0x0031_45f8;
/// `gp-0x7f40`: 1/60, ticks to seconds (this type's own `.lit4` entry).
const SEQ01_TICK_GP: u32 = 0x7f40;

/// `int RiderAnim_UpdateSeqType01(RiderAnim* anim, SeqList* list,
/// AnimSequence* seq, float dt)` (0x104940): a one-shot sequence. Advance
/// its track by `dt * (1/60)`; when the track has ended, or else when its
/// weight fade has finished, remove it from the list. Returns whether it
/// was removed. (Same shape as type 02, with its own step constant.)
pub fn rider_anim_seq_type01_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x40);
        let k = Addr(r32(c, GP).wrapping_sub(SEQ01_TICK_GP)).read_fpu(c);
        c.f[0] = k.0;
        frame.save_fpr(c, 20, 0x30);
        let step = FpuF32(c.f[12]) * k;
        c.f[20] = step.0;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S0][0];
        c.f[12] = step.0;
        call_ptr(c, SEQ01_ADVANCE_TRACK, 0x0010_4974)?;
        let ended = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if !ended {
            c.cycles += 3;
            c.f[12] = c.f[20];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, SEQ01_UPDATE_WEIGHT_FADE, 0x0010_4988)?;
        }
        c.cycles += 2;
        let remove = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S1][0];
        if remove {
            c.cycles += 4;
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, SEQ01_LIST_REMOVE, 0x0010_4998)?;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// SND_ChannelStartVoices (0x3c3ee0)

/// The sound driver state block (`0x510000 - 0x5718`).
const SV_STATE: u32 = 0x0050_a8e8;
/// The second global block (`0x510000 - 0x5300`): voice start tables.
const SV_SYNC: u32 = 0x0050_ad00;
/// State: `SndChannel*` channel array.
const SV_STATE_CHANNELS: u32 = 0x1f0;
/// State `u8`: first EE voice number (after `SV_STATE_VOICE_BASE`).
const SV_STATE_EE_VOICE_BASE: u32 = 0x2f;
/// State `u8`: first voice number of the IOP DSP block.
const SV_STATE_VOICE_BASE: u32 = 0x30;
/// State `u8`: SPU output mode; 2 also drives the second voice block (+0x18).
const SV_STATE_OUTPUT_MODE: u32 = 0x49;
/// Sync: key-on sequence counter (`+= 0x800000` per started voice).
const SV_SYNC_KEY_SEQ: u32 = 0x1ab8;
/// Sync: `u8*` voice parameter blocks (SPU 8 bytes, IOP 16 bytes per voice).
const SV_SYNC_VOICE_PARAMS: u32 = 0xcb0;
/// Sync: per-voice start addresses (32 bytes per voice).
const SV_SYNC_START_ADDRS: u32 = 0xeb0;
/// Sync: SPU start-address shadow (8 bytes per voice).
const SV_SYNC_SPU_SHADOW: u32 = 0x840;
/// Sync: IOP DSP voice shadow (8 bytes per voice).
const SV_SYNC_IOP_SHADOW: u32 = 0x9c0;
/// `SndChannel` size.
const SV_CHAN_SIZE: u32 = 0x8c;
/// Channel `s16[]`: voice numbers.
const SV_CHAN_VOICES: u32 = 0x04;
/// Channel word passed to `SND_EeVoiceStart` (8th argument).
const SV_CHAN_EE_PARAM: u32 = 0x14;
/// Channel `u8`: voice count.
const SV_CHAN_VOICE_COUNT: u32 = 0x23;
/// Channel `u16`: flags (8: SPU voices, 0x100: IOP DSP voices, else EE).
const SV_CHAN_FLAGS: u32 = 0x24;
/// Start descriptor `s8` passed to `SND_EeVoiceStart` (13th argument).
const SV_DESC_PRIORITY: u32 = 0x02;
/// Start descriptor `u8`: codec.
const SV_DESC_CODEC: u32 = 0x13;
/// Start descriptor words passed to `SND_EeVoiceStart` (9th/10th args).
const SV_DESC_ARG5C: u32 = 0x5c;
const SV_DESC_ARG60: u32 = 0x60;
/// Start descriptor: pointer to the sample base address.
const SV_DESC_BASE_PTR: u32 = 0x64;
/// Start descriptor `int[]`: per-voice sample offsets.
const SV_DESC_OFFSETS: u32 = 0x68;
/// Start descriptor `int[]`: per-voice loop offsets.
const SV_DESC_LOOPS: u32 = 0x80;
/// Start descriptor `int[]`: per-voice stretch parameters.
const SV_DESC_STRETCH: u32 = 0x98;

const SV_APPLY_PITCH: Callee = Callee { addr: 0x003c_25e0, f: ee::s13::SND_ChannelApplyPitch };
const SV_APPLY_PAN: Callee = Callee { addr: 0x003c_38e0, f: ee::s13::SND_ChannelApplyPan };
const SV_APPLY_FX_ENABLE: Callee =
    Callee { addr: 0x003c_2ec8, f: ee::s13::SND_ChannelApplyFxEnable };
const SV_APPLY_SEND: Callee = Callee { addr: 0x003c_27e0, f: ee::s13::SND_ChannelApplySend };
const SV_APPLY_TIME_STRETCH: Callee =
    Callee { addr: 0x003c_3010, f: ee::s13::SND_ChannelApplyTimeStretch };
const SV_APPLY_DISTORTION: Callee =
    Callee { addr: 0x003c_30c8, f: ee::s13::SND_ChannelApplyDistortion };
const SV_APPLY_LOWPASS: Callee = Callee { addr: 0x003c_3178, f: ee::s13::SND_ChannelApplyLowpass };
const SV_APPLY_HIGHPASS: Callee =
    Callee { addr: 0x003c_3250, f: ee::s13::SND_ChannelApplyHighpass };
/// `SND_EeVoiceStart` (cross-shard: through the function table).
const SV_EE_VOICE_START: u32 = 0x003c_8158;
/// `SND_EeVoiceActivate` (cross-shard: through the function table).
const SV_EE_VOICE_ACTIVATE: u32 = 0x003c_83a0;

/// `int SND_ChannelStartVoices(StartDesc* d, u8* base, int ch, int stretch,
/// int distortion, int lowpass, int highpass)` (0x3c3ee0): start the voices
/// of sound channel `ch` (`state.channels + ch * 0x8c`).
///
/// - SPU channels (flag 8): apply pitch, pan, effect enable and send, then
///   per voice write the start address (`*d->base_ptr + d->offsets[i] -
///   d->offsets[0]`), bump the key-on sequence and store the tagged address
///   into the voice parameters and the SPU shadow (and the second block in
///   output mode 2).
/// - IOP DSP channels (flag 0x100): pitch, pan, send, then the same start
///   address into the IOP parameter block (+0x1a0) and a cleared shadow.
/// - EE channels: `SND_EeVoiceStart` per voice, then pitch, pan, send,
///   time stretch, distortion, low/high pass on the first voice, then
///   `SND_EeVoiceActivate` per voice.
///
/// Returns 0.
pub fn snd_channel_start_voices(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0xe0);
        c.g[V1][0] = SV_CHAN_SIZE as u64;
        frame.save(c, S0, 0xd0);
        c.g[V0][0] = 0x51_0000;
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0xb0);
        frame.save(c, S3, 0xa0);
        let ch_id = r32(c, S0);
        let ch_off = abi::mult(c, ch_id, SV_CHAN_SIZE);
        c.g[V1][0] = sx(ch_off);
        frame.save(c, S4, 0x90);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S5, 0x80);
        c.g[S4][0] = sx(SV_STATE);
        frame.save(c, S7, 0x60);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, FP, 0x50);
        c.g[S7][0] = c.g[A3][0];
        frame.save(c, S1, 0xc0);
        c.g[FP][0] = c.g[T0][0];
        frame.save(c, S6, 0x70);
        frame.save_ra(c, 0x40);
        let chans = Addr(SV_STATE + SV_STATE_CHANNELS).lw(c);
        c.g[V0][0] = chans;
        let lowpass = c.g[T1][0];
        Addr(sp(c) + 0x30).sw(c, lowpass);
        let ch = (chans as u32).wrapping_add(ch_off);
        c.g[S2][0] = sx(ch);
        let flags = Addr(ch + SV_CHAN_FLAGS).lhu(c);
        c.g[V1][0] = flags;
        c.g[V0][0] = flags & 0x8;
        let highpass = c.g[T2][0];
        Addr(sp(c) + 0x34).sw(c, highpass);

        if flags & 0x8 != 0 {
            // SPU voices.
            c.cycles += 13;
            c.g[A0][0] = c.g[S0][0];
            c.g[S1][0] = 0;
            SV_APPLY_PITCH.call(c, 0x003c_3f58)?;
            c.g[A0][0] = c.g[S0][0];
            SV_APPLY_PAN.call(c, 0x003c_3f60)?;
            c.g[A0][0] = c.g[S0][0];
            SV_APPLY_FX_ENABLE.call(c, 0x003c_3f68)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            SV_APPLY_SEND.call(c, 0x003c_3f74)?;
            let n = Addr(r32(c, S2) + SV_CHAN_VOICE_COUNT).lbu(c);
            c.g[V1][0] = n;
            c.g[V0][0] = 0x51_0000;
            if n == 0 {
                sv_epilogue(c, frame, 1);
                return Ok(());
            }
            c.cycles += 8;
            c.g[T6][0] = c.g[S4][0];
            c.g[T0][0] = sx(SV_SYNC);
            c.g[T5][0] = 0x80_0000;
            c.g[T1][0] = sx(SV_SYNC + SV_SYNC_START_ADDRS);
            c.g[T3][0] = sx(SV_SYNC + SV_SYNC_SPU_SHADOW);
            c.g[T4][0] = 0x2;
            c.g[T2][0] = sx(r32(c, S3).wrapping_add(SV_DESC_OFFSETS));
            c.g[A3][0] = sx(r32(c, S2).wrapping_add(SV_CHAN_VOICES));
            loop {
                c.cycles += 31;
                let desc = r32(c, S3);
                let sync = r32(c, T0);
                let starts = r32(c, T1);
                let shadow = r32(c, T3);
                let voice_at = Addr(r32(c, A3));
                // Start address: *base_ptr + (offsets[i] - offsets[0]).
                let base_ptr = Addr(desc + SV_DESC_BASE_PTR).lw(c);
                let off0 = Addr(desc + SV_DESC_OFFSETS).lw(c);
                let voice = voice_at.lh(c) as u32;
                let off_i = Addr(r32(c, T2)).lw(c);
                let base = Addr(base_ptr as u32).lw(c);
                let start = (base as u32).wrapping_add((off_i as u32).wrapping_sub(off0 as u32));
                Addr((voice << 5).wrapping_add(starts)).sw(c, sx(start));
                // Next key-on sequence tag.
                let seq = (Addr(sync + SV_SYNC_KEY_SEQ).lw(c) as u32).wrapping_add(r32(c, T5));
                let params = Addr(sync + SV_SYNC_VOICE_PARAMS).lw(c) as u32;
                Addr(sync + SV_SYNC_KEY_SEQ).sw(c, sx(seq));
                let voice = voice_at.lh(c) as u32;
                let addr = Addr((voice << 5).wrapping_add(starts)).lw(c) as u32;
                let param_at = params.wrapping_add(voice << 3);
                Addr(param_at + 0x20).sw(c, sx(addr | seq));
                let voice = voice_at.lh(c) as u32;
                let mode = Addr(r32(c, T6) + SV_STATE_OUTPUT_MODE).lbu(c);
                let e_at = (voice << 5).wrapping_add(starts);
                let addr = Addr(e_at).lw(c);
                let shadow_at = (voice << 3).wrapping_add(shadow);
                Addr(shadow_at).sw(c, addr);
                if mode != c.g[T4][0] {
                    c.g[A0][0] = sx(e_at);
                    c.g[A1][0] = mode;
                    c.g[A2][0] = sx(param_at);
                    c.g[V1][0] = sx(shadow_at);
                } else {
                    // Output mode 2: the second voice block (+0x18) too.
                    c.cycles += 19;
                    let voice = voice_at.lh(c) as u32;
                    let params = Addr(sync + SV_SYNC_VOICE_PARAMS).lw(c) as u32;
                    let seq = Addr(sync + SV_SYNC_KEY_SEQ).lw(c);
                    let addr = Addr((voice << 5).wrapping_add(starts)).lw(c);
                    let param2_at = params.wrapping_add(voice.wrapping_add(0x18) << 3);
                    Addr(param2_at + 0x20).sw(c, addr | seq);
                    let voice = voice_at.lh(c) as u32;
                    let e_at = (voice << 5).wrapping_add(starts);
                    let addr = Addr(e_at).lw(c);
                    let shadow2_at = (voice.wrapping_add(0x18) << 3).wrapping_add(shadow);
                    Addr(shadow2_at).sw(c, addr);
                    c.g[A0][0] = addr;
                    c.g[A1][0] = sx(param2_at);
                    c.g[A2][0] = seq;
                    c.g[V1][0] = sx(e_at);
                    c.g[V0][0] = sx(shadow2_at);
                }
                c.cycles += 6;
                let n = Addr(r32(c, S2) + SV_CHAN_VOICE_COUNT).lbu(c);
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                c.g[T2][0] = sx(r32(c, T2).wrapping_add(4));
                let more = (c.g[S1][0] as i64) < (n as i64);
                c.g[V0][0] = more as u64;
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(2));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_3fa0);
                }
            }
            sv_epilogue(c, frame, 2);
            return Ok(());
        }

        c.cycles += 3;
        c.g[V0][0] = c.g[V1][0] & 0x100;
        c.g[A0][0] = c.g[S0][0];
        if c.g[V0][0] != 0 {
            // IOP DSP voices.
            c.cycles += 10;
            c.g[S1][0] = 0;
            SV_APPLY_PITCH.call(c, 0x003c_409c)?;
            c.g[A0][0] = c.g[S0][0];
            SV_APPLY_PAN.call(c, 0x003c_40a4)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            SV_APPLY_SEND.call(c, 0x003c_40b0)?;
            let n = Addr(r32(c, S2) + SV_CHAN_VOICE_COUNT).lbu(c);
            c.g[V1][0] = n;
            c.g[V0][0] = 0x51_0000;
            if n == 0 {
                sv_epilogue(c, frame, 1);
                return Ok(());
            }
            c.cycles += 7;
            c.g[T4][0] = c.g[S4][0];
            c.g[T2][0] = sx(SV_SYNC);
            c.g[T6][0] = 0x80_0000;
            c.g[T3][0] = sx(SV_SYNC + SV_SYNC_START_ADDRS);
            c.g[T5][0] = sx(SV_SYNC + SV_SYNC_IOP_SHADOW);
            c.g[T1][0] = sx(r32(c, S3).wrapping_add(SV_DESC_OFFSETS));
            c.g[T0][0] = sx(r32(c, S2).wrapping_add(SV_CHAN_VOICES));
            loop {
                c.cycles += 36;
                let desc = r32(c, S3);
                let sync = r32(c, T2);
                let starts = r32(c, T3);
                let state = r32(c, T4);
                let voice_at = Addr(r32(c, T0));
                let base_ptr = Addr(desc + SV_DESC_BASE_PTR).lw(c);
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                let voice = voice_at.lh(c) as u32;
                let off_i = Addr(r32(c, T1)).lw(c);
                let off0 = Addr(desc + SV_DESC_OFFSETS).lw(c);
                let base = Addr(base_ptr as u32).lw(c);
                let first = Addr(state + SV_STATE_VOICE_BASE).lbu(c);
                let start = (base as u32).wrapping_add((off_i as u32).wrapping_sub(off0 as u32));
                c.g[T1][0] = sx(r32(c, T1).wrapping_add(4));
                Addr((voice << 5).wrapping_add(starts)).sw(c, sx(start));
                let seq = (Addr(sync + SV_SYNC_KEY_SEQ).lw(c) as u32).wrapping_add(r32(c, T6));
                let params = Addr(sync + SV_SYNC_VOICE_PARAMS).lw(c) as u32;
                Addr(sync + SV_SYNC_KEY_SEQ).sw(c, sx(seq));
                let voice = voice_at.lh(c) as u32;
                let e_at = (voice << 5).wrapping_add(starts);
                let addr = Addr(e_at).lw(c) as u32;
                let param_at = params.wrapping_add(voice.wrapping_sub(first as u32) << 4);
                Addr(param_at + 0x1a0).sw(c, sx(addr | seq));
                let voice = voice_at.lh(c) as u32;
                let first2 = Addr(state + SV_STATE_VOICE_BASE).lbu(c) as u32;
                let shadow_at = (voice.wrapping_sub(first2) << 3).wrapping_add(r32(c, T5));
                Addr(shadow_at).sw(c, 0);
                let n = Addr(r32(c, S2) + SV_CHAN_VOICE_COUNT).lbu(c);
                let more = (c.g[S1][0] as i64) < (n as i64);
                c.g[A0][0] = sx(e_at);
                c.g[A1][0] = sx(seq);
                c.g[A2][0] = sx(param_at);
                c.g[A3][0] = first;
                c.g[V1][0] = sx(shadow_at);
                c.g[V0][0] = more as u64;
                c.g[T0][0] = sx(r32(c, T0).wrapping_add(2));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_40d8);
                }
            }
            sv_epilogue(c, frame, 2);
            return Ok(());
        }

        // EE voices: start each one.
        c.cycles += 3;
        let n = Addr(r32(c, S2) + SV_CHAN_VOICE_COUNT).lbu(c);
        c.g[T2][0] = n;
        c.g[S1][0] = 0;
        if n != 0 {
            c.cycles += 5;
            c.g[S6][0] = c.g[S4][0];
            c.g[S0][0] = sx(r32(c, S3).wrapping_add(SV_DESC_STRETCH));
            c.g[S4][0] = sx(r32(c, S2).wrapping_add(SV_CHAN_VOICES));
            c.g[A0][0] = Addr(r32(c, S3) + SV_DESC_ARG60).lw(c);
            loop {
                c.cycles += 31;
                let desc = r32(c, S3);
                let state = r32(c, S6);
                let base = r32(c, S5);
                let stretch_at = r32(c, S0);
                let s = sp(c);
                c.g[A2][0] = 0x1;
                let priority = Addr(desc + SV_DESC_PRIORITY).lb(c);
                let arg5c = Addr(desc + SV_DESC_ARG5C).lw(c);
                let voice = Addr(r32(c, S4)).lh(c);
                let index = c.g[S1][0];
                Addr(s + 0x28).sw(c, index);
                c.g[S4][0] = sx(r32(c, S4).wrapping_add(2));
                Addr(s).sw(c, arg5c);
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                let arg60 = c.g[A0][0];
                Addr(s + 0x8).sw(c, arg60);
                Addr(s + 0x20).sw(c, priority);
                let ee_first = Addr(state + SV_STATE_EE_VOICE_BASE).lbu(c) as u32;
                let first = Addr(state + SV_STATE_VOICE_BASE).lbu(c) as u32;
                let off0 = Addr(desc + SV_DESC_OFFSETS).lw(c) as u32;
                Addr(s + 0x10).sw(c, 0);
                Addr(s + 0x18).sw(c, 0);
                // s0 walks the stretch words; the loop and sample offsets
                // of the same voice sit 0x18 and 0x30 bytes before.
                let loop_off =
                    Addr(stretch_at.wrapping_sub(SV_DESC_STRETCH - SV_DESC_LOOPS)).lw(c) as u32;
                let off_i =
                    Addr(stretch_at.wrapping_sub(SV_DESC_STRETCH - SV_DESC_OFFSETS)).lw(c) as u32;
                let stretch = Addr(stretch_at).lw(c);
                let codec = Addr(desc + SV_DESC_CODEC).lbu(c);
                let ee_param = Addr(r32(c, S2) + SV_CHAN_EE_PARAM).lw(c);
                // Registers at the call.
                c.g[V0][0] = sx(loop_off);
                c.g[V1][0] = arg5c;
                c.g[A0][0] = sx((voice as u32).wrapping_sub(first.wrapping_add(ee_first)));
                c.g[A1][0] = codec;
                c.g[A3][0] = sx(base.wrapping_add(off_i));
                c.g[T0][0] = sx(base.wrapping_add(off0).wrapping_add(loop_off));
                c.g[T1][0] = stretch;
                c.g[T3][0] = ee_param;
                c.g[S0][0] = sx(stretch_at.wrapping_add(4));
                call_ptr(c, SV_EE_VOICE_START, 0x003c_41fc)?;
                let n = Addr(r32(c, S2) + SV_CHAN_VOICE_COUNT).lbu(c);
                c.g[T2][0] = n;
                let more = (c.g[S1][0] as i64) < (n as i64);
                c.g[V0][0] = more as u64;
                if !more {
                    break;
                }
                c.g[A0][0] = Addr(r32(c, S3) + SV_DESC_ARG60).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x003c_4190);
                }
            }
        }

        // Channel parameters on the first voice.
        c.cycles += 23;
        let first_voice = Addr(r32(c, S2) + SV_CHAN_VOICES);
        c.g[A0][0] = first_voice.lh(c);
        c.g[S1][0] = 0;
        SV_APPLY_PITCH.call(c, 0x003c_4218)?;
        c.g[A0][0] = first_voice.lh(c);
        SV_APPLY_PAN.call(c, 0x003c_4220)?;
        c.g[A0][0] = first_voice.lh(c);
        c.g[A1][0] = 0;
        SV_APPLY_SEND.call(c, 0x003c_422c)?;
        c.g[A0][0] = first_voice.lh(c);
        c.g[A1][0] = c.g[S7][0];
        SV_APPLY_TIME_STRETCH.call(c, 0x003c_4238)?;
        c.g[A0][0] = first_voice.lh(c);
        c.g[A1][0] = c.g[FP][0];
        SV_APPLY_DISTORTION.call(c, 0x003c_4244)?;
        c.g[A1][0] = Addr(sp(c) + 0x30).lw(c);
        c.g[A0][0] = first_voice.lh(c);
        SV_APPLY_LOWPASS.call(c, 0x003c_4250)?;
        c.g[A0][0] = first_voice.lh(c);
        c.g[A1][0] = Addr(sp(c) + 0x34).lw(c);
        SV_APPLY_HIGHPASS.call(c, 0x003c_425c)?;
        let n = Addr(r32(c, S2) + SV_CHAN_VOICE_COUNT).lbu(c);
        c.g[V0][0] = 0x51_0000;
        if n != 0 {
            // Activate each voice.
            c.cycles += 4;
            c.g[S0][0] = sx(r32(c, S2).wrapping_add(SV_CHAN_VOICES));
            c.g[S3][0] = sx(SV_STATE);
            c.g[V0][0] = Addr(SV_STATE + SV_STATE_EE_VOICE_BASE).lbu(c);
            loop {
                c.cycles += 11;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                let state = r32(c, S3);
                let first = Addr(state + SV_STATE_VOICE_BASE).lbu(c) as u32;
                let voice = Addr(r32(c, S0)).lh(c);
                c.g[V1][0] = voice;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(2));
                let bases = first.wrapping_add(r32(c, V0));
                c.g[A0][0] = sx((voice as u32).wrapping_sub(bases));
                call_ptr(c, SV_EE_VOICE_ACTIVATE, 0x003c_4294)?;
                let n = Addr(r32(c, S2) + SV_CHAN_VOICE_COUNT).lbu(c);
                let more = (c.g[S1][0] as i64) < (n as i64);
                c.g[V0][0] = more as u64;
                if !more {
                    break;
                }
                c.g[V0][0] = Addr(r32(c, S3) + SV_STATE_EE_VOICE_BASE).lbu(c);
                if event_due(c) {
                    deliver_event_here(c, 0x003c_4278);
                }
            }
        }
        sv_epilogue(c, frame, 1);
        Ok(())
    });
}

/// `SND_ChannelStartVoices`' exit: the `lq s0` block (`pre` cycles), then
/// v0 = 0 and the callee-saved registers back.
fn sv_epilogue(c: &mut Cpu, frame: Frame, pre: u64) {
    c.cycles += pre;
    frame.restore(c, S0, 0xd0);
    c.cycles += 12;
    c.g[V0][0] = 0;
    frame.restore(c, S1, 0xc0);
    frame.restore(c, S2, 0xb0);
    frame.restore(c, S3, 0xa0);
    frame.restore(c, S4, 0x90);
    frame.restore(c, S5, 0x80);
    frame.restore(c, S6, 0x70);
    frame.restore(c, S7, 0x60);
    frame.restore(c, FP, 0x50);
    frame.restore_ra(c, 0x40);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// AnimSeqList_Save (0x3147f0)

/// AnimSeqList `int`: number of sequences.
const SEQLIST_COUNT: u32 = 0x0;
/// AnimSeqList: first sequence node.
const SEQLIST_FIRST: u32 = 0x4;
/// AnimSequence: next node in the list.
const SEQLIST_NODE_NEXT: u32 = 0xc8;
/// `AnimSequence_Save(seq, stream)`.
const SEQLIST_SEQUENCE_SAVE: Callee =
    Callee { addr: 0x0031_3dd0, f: ee::s10::AnimSequence_Save };

/// `void AnimSeqList_Save(AnimSeqList* list, cAsStream* s)` (0x3147f0):
/// write the list's count word (`s->Write(list, 4)`, vtable slot +0x8), then
/// save each of its `count` sequences (linked through +0xc8).
pub fn anim_seq_list_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x50);
        c.g[A2][0] = 0x4;
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let stream = c.g[A1][0];
        c.g[S3][0] = stream;
        c.g[A1][0] = c.g[S2][0];
        // s->Write(list, 4).
        let vt = Addr(stream as u32).lw(c);
        c.g[V1][0] = vt;
        let slot = crate::call::VSlot::read(c, vt, 0x8);
        c.g[A0][0] = slot.delta;
        c.g[V0][0] = slot.func;
        c.g[A0][0] = sx((stream as u32).wrapping_add(slot.delta as u32));
        call_ptr(c, slot.func as u32, 0x0031_4830)?;
        let list = r32(c, S2);
        let count = Addr(list + SEQLIST_COUNT).lw(c);
        c.g[V1][0] = count;
        c.g[S0][0] = Addr(list + SEQLIST_FIRST).lw(c);
        if (count as i64) > 0 {
            c.cycles += 1;
            loop {
                c.cycles += 8;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S3][0];
                SEQLIST_SEQUENCE_SAVE.call(c, 0x0031_484c)?;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                let count = Addr(r32(c, S2) + SEQLIST_COUNT).lw(c);
                let more = (c.g[S1][0] as i64) < (count as i64);
                c.g[V0][0] = more as u64;
                c.g[S0][0] = Addr(r32(c, S0).wrapping_add(SEQLIST_NODE_NEXT)).lw(c);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0031_4840);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// RiderBumpSub_HandleInput (0x12e778)

/// Bump sub-state: the rider (`*this`).
const BUMP_RIDER: u32 = 0x0;
/// Pad: button / axis word (bit 12 reset path, 13/14 boost input, bits
/// 21..26 and 15..20 the 6-bit stick axes).
const BUMP_PAD_BUTTONS: u32 = 0x0;
/// Rider: velocity and facing vectors (their dot product picks the lean
/// sign).
const BUMP_RIDER_VELOCITY: u32 = 0x1e0;
const BUMP_RIDER_FACING: u32 = 0x3a0;
/// Rider floats: slew-1 target and rate.
const BUMP_RIDER_SLEW1_TARGET: u32 = 0x204;
const BUMP_RIDER_SLEW1_RATE: u32 = 0x200;
/// Rider: the owner (`RiderBase`) and the animation controller.
const BUMP_RIDER_OWNER: u32 = 0x77c;
const BUMP_RIDER_ANIM_CTRL: u32 = 0x784;
/// `gp`-relative `.lit4`: 1/31 (axis scale), again for the lean, 1/30.
const BUMP_AXIS_SCALE_GP: u32 = 0x77a0;
const BUMP_LEAN_SCALE_GP: u32 = 0x779c;
const BUMP_SLEW1_RATE_GP: u32 = 0x7798;

const BUMP_TRY_ENTER_FINISH_LINE: Callee =
    Callee { addr: 0x0011_6378, f: ee::s00::Rider_TryEnterFinishLine };
const BUMP_CHECK_RESET_PATH: Callee =
    Callee { addr: 0x0011_6120, f: ee::s00::Rider_CheckResetPath };
const BUMP_TRY_START_RAILSLIDE: Callee =
    Callee { addr: 0x0010_6848, f: ee::s00::Rider_TryStartRailslide };
const BUMP_GET_MAIN_STATE: Callee = Callee { addr: 0x0011_fe98, f: ee::s00::Rider_GetMainState };
const BUMP_HANDLE_BOOST_INPUT: Callee =
    Callee { addr: 0x0011_4130, f: ee::s00::Rider_HandleBoostInput };
const BUMP_SET_SLEW0_TARGET: Callee =
    Callee { addr: 0x0011_3e80, f: ee::s00::Rider_SetSlew0Target };
const BUMP_SET_SLEW5_TARGET: Callee =
    Callee { addr: 0x0011_3f38, f: ee::s00::Rider_SetSlew5Target };
const BUMP_SET_SLEW3_AND4_TARGETS: Callee =
    Callee { addr: 0x0011_3f88, f: ee::s00::Rider_SetSlew3And4Targets };
const BUMP_RESET_GRIND_ORIENTATION: Callee =
    Callee { addr: 0x0011_5640, f: ee::s00::Rider_ResetGrindOrientation };
const BUMP_SET_SUB_STATE: Callee = Callee { addr: 0x0011_fec8, f: ee::s00::Rider_SetSubState };
/// `AnimController_IsLayerFinished(ctrl, layer)` (cross-shard).
const BUMP_ANIM_LAYER_DONE: u32 = 0x0031_2ae8;
/// The owner's land-animation helper (`owner + 0x2b0`, cross-shard).
const BUMP_PLAY_LAND_ANIM: u32 = 0x0013_26c8;

/// `void RiderBumpSub_HandleInput(RiderBumpSub* this, Pad* pad)` (0x12e778):
/// the input of a rider recovering from a bump, once per tick.
///
/// - Crossing the finish line, a path reset (pad bit 12) or a rail start
///   end the function.
/// - Airborne (main state 1): no boost, slew 0 to 0. Otherwise boost input
///   (bits 14/13); in main state 4 slew 5 follows the axis in bits 21..26
///   (/31), else slew 0 follows the axis in bits 15..20 (/31), negated when
///   the rider moves backwards (velocity . facing < 0).
/// - Slews 3/4 to 0, slew 1 to 0 at rate 1/30.
/// - When the layer-2 animation has finished: main state 4 plays the land
///   animation and enters sub-state 7; otherwise the grind orientation is
///   reset and sub-state 4 (airborne) or 0 follows.
pub fn rider_bump_sub_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save_fpr(c, 20, 0x30);
        let this = Addr(r32(c, S0));
        let pad = Addr(r32(c, S1));
        c.g[A0][0] = this.off(BUMP_RIDER).lw(c);
        BUMP_TRY_ENTER_FINISH_LINE.call(c, 0x0012_e79c)?;
        if c.g[V0][0] != 0 {
            return bump_exit(c, frame);
        }
        c.cycles += 8;
        let buttons = pad.off(BUMP_PAD_BUTTONS).lw(c) as u32;
        c.g[A2][0] = 0;
        c.g[A0][0] = this.off(BUMP_RIDER).lw(c);
        c.g[A1][0] = sx(((buttons as i32) >> 12) as u32) & 0x1;
        BUMP_CHECK_RESET_PATH.call(c, 0x0012_e7bc)?;
        if c.g[V0][0] != 0 {
            return bump_exit(c, frame);
        }
        c.cycles += 4;
        c.g[A0][0] = this.off(BUMP_RIDER).lw(c);
        BUMP_TRY_START_RAILSLIDE.call(c, 0x0012_e7cc)?;
        if c.g[V0][0] != 0 {
            return bump_exit(c, frame);
        }

        c.cycles += 5;
        c.g[A0][0] = this.off(BUMP_RIDER).lw(c);
        BUMP_GET_MAIN_STATE.call(c, 0x0012_e7dc)?;
        c.g[V1][0] = 0x1;
        if c.g[V0][0] == 1 {
            // Airborne: no boost, no lean.
            c.cycles += 9;
            c.g[A0][0] = this.off(BUMP_RIDER).lw(c);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            BUMP_HANDLE_BOOST_INPUT.call(c, 0x0012_e7f8)?;
            c.f[12] = 0;
            c.g[A0][0] = this.off(BUMP_RIDER).lw(c);
            BUMP_SET_SLEW0_TARGET.call(c, 0x0012_e804)?;
        } else {
            let buttons = pad.off(BUMP_PAD_BUTTONS).lw(c) as u32;
            c.cycles += 11;
            c.g[A0][0] = this.off(BUMP_RIDER).lw(c);
            c.g[A2][0] = sx(((buttons as i32) >> 13) as u32) & 0x1;
            c.g[A1][0] = sx(((buttons as i32) >> 14) as u32) & 0x1;
            BUMP_HANDLE_BOOST_INPUT.call(c, 0x0012_e824)?;
            c.g[A0][0] = this.off(BUMP_RIDER).lw(c);
            BUMP_GET_MAIN_STATE.call(c, 0x0012_e82c)?;
            c.g[V1][0] = 0x4;
            let state4 = c.g[V0][0] == 4;
            let buttons = pad.off(BUMP_PAD_BUTTONS).lw(c) as u32;
            if state4 {
                // Slew 5 follows the axis in bits 21..26.
                c.cycles += 10;
                let gp = r32(c, GP);
                let scale = Addr(gp.wrapping_sub(BUMP_AXIS_SCALE_GP)).read_fpu(c);
                c.f[0] = scale.0;
                c.g[A0][0] = this.off(BUMP_RIDER).lw(c);
                let axis = (((buttons << 5) as i32) >> 26) as u32;
                c.g[V0][0] = sx(axis);
                c.f[12] = axis;
                c.cvt_s_w(12, 12);
                c.mul_s(12, 12, 0);
                BUMP_SET_SLEW5_TARGET.call(c, 0x0012_e858)?;
            } else {
                // Slew 0 follows the axis in bits 15..20, signed by the
                // direction of travel.
                c.cycles += 20;
                let rider = this.off(BUMP_RIDER).lw(c);
                c.g[A0][0] = rider;
                let axis = (((buttons << 11) as i32) >> 26) as u32;
                c.f[3] = 0;
                c.g[V0][0] = sx(axis);
                let rider = rider as u32;
                c.lqc2(3, rider.wrapping_add(BUMP_RIDER_VELOCITY));
                c.lqc2(5, rider.wrapping_add(BUMP_RIDER_FACING));
                c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
                c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
                c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
                c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
                c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
                c.qmfc2(3, 4);
                c.f[1] = c.g[V1][0] as u32;
                c.f[2] = axis;
                c.cvt_s_w(2, 2);
                let gp = r32(c, GP);
                let scale = Addr(gp.wrapping_sub(BUMP_LEAN_SCALE_GP)).read_fpu(c);
                c.f[0] = scale.0;
                c.c_lt_s(1, 3);
                c.mul_s(12, 2, 0);
                if c.fcc() {
                    c.cycles += 1;
                    c.neg_s(12, 12);
                }
                c.cycles += 2;
                BUMP_SET_SLEW0_TARGET.call(c, 0x0012_e8bc)?;
            }
        }

        // Slews 3/4 and 1 to zero.
        c.cycles += 15;
        c.f[20] = 0;
        c.g[A0][0] = this.off(BUMP_RIDER).lw(c);
        c.f[13] = c.f[20];
        c.f[12] = c.f[20];
        BUMP_SET_SLEW3_AND4_TARGETS.call(c, 0x0012_e8d0)?;
        let rider_word = this.off(BUMP_RIDER).lw(c);
        c.g[V1][0] = rider_word;
        let rider = Addr(rider_word as u32);
        c.g[A1][0] = 0x2;
        let gp = r32(c, GP);
        let rate = Addr(gp.wrapping_sub(BUMP_SLEW1_RATE_GP)).read_fpu(c);
        c.f[0] = rate.0;
        let zero = FpuF32(c.f[20]);
        rider.off(BUMP_RIDER_SLEW1_TARGET).write_fpu(c, zero);
        rider.off(BUMP_RIDER_SLEW1_RATE).write_fpu(c, rate);
        let rider = this.off(BUMP_RIDER).lw(c);
        c.g[V0][0] = rider;
        c.g[A0][0] = Addr((rider as u32).wrapping_add(BUMP_RIDER_ANIM_CTRL)).lw(c);
        call_ptr(c, BUMP_ANIM_LAYER_DONE, 0x0012_e8f0)?;
        if c.g[V0][0] == 0 {
            return bump_exit(c, frame);
        }

        // The bump animation has finished: leave the sub-state.
        c.cycles += 5;
        c.g[A0][0] = this.off(BUMP_RIDER).lw(c);
        BUMP_GET_MAIN_STATE.call(c, 0x0012_e900)?;
        c.g[V1][0] = 0x4;
        c.g[A1][0] = 0;
        if c.g[V0][0] == 4 {
            c.cycles += 9;
            let rider = this.off(BUMP_RIDER).lw(c);
            c.g[V0][0] = rider;
            let owner = Addr((rider as u32).wrapping_add(BUMP_RIDER_OWNER)).lw(c) as u32;
            c.g[A0][0] = sx(owner.wrapping_add(0x2b0));
            call_ptr(c, BUMP_PLAY_LAND_ANIM, 0x0012_e91c)?;
            c.g[A0][0] = this.off(BUMP_RIDER).lw(c);
            c.g[A1][0] = 0x7;
            BUMP_SET_SUB_STATE.call(c, 0x0012_e928)?;
            return bump_exit(c, frame);
        }
        c.cycles += 7;
        c.g[A0][0] = this.off(BUMP_RIDER).lw(c);
        BUMP_RESET_GRIND_ORIENTATION.call(c, 0x0012_e938)?;
        c.g[A0][0] = this.off(BUMP_RIDER).lw(c);
        BUMP_GET_MAIN_STATE.call(c, 0x0012_e940)?;
        c.g[V1][0] = 0x1;
        let airborne = c.g[V0][0] == 1;
        c.g[A0][0] = this.off(BUMP_RIDER).lw(c);
        if airborne {
            c.cycles += 4;
            c.g[A1][0] = 0x4;
            BUMP_SET_SUB_STATE.call(c, 0x0012_e954)?;
        } else {
            c.cycles += 3;
            c.g[A1][0] = 0;
            BUMP_SET_SUB_STATE.call(c, 0x0012_e964)?;
        }
        bump_exit(c, frame)
    });
}

/// `RiderBumpSub_HandleInput`'s exit (`lq s0` in each branch, then the
/// shared 5-cycle tail).
fn bump_exit(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    frame.restore(c, S0, 0x20);
    c.cycles += 5;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 20, 0x30);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// WorldPainter_GetWeather28 (0x2ef0a0), GetWeather90 (0x2ee930),
// GetWeather10 (0x2ee4c0)

/// The world painters (`D_004FA370`, 0xf0 bytes each).
const WEATHER_PAINTERS: u32 = 0x004f_a370;
/// Painter: pointer to its weather object pointer.
const WEATHER_HOLDER: u32 = 0x20;

/// The painters' float weather getters: `obj = *painters[index].holder`,
/// call the object's virtual method at v-table `vt_off` (`{delta: i16,
/// fn}`, `this + delta`, through the function table) and return the float
/// it points at (f0).
fn weather_slot_float_getter(c: &mut Cpu, vt_off: u32, ret_addr: u32) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[V1][0] = 0xf0;
        let frame = Frame::open(c, 0x10);
        let index = r32(c, A0);
        let off = abi::mult(c, index, 0xf0);
        c.g[A0][0] = sx(off);
        c.g[V0][0] = 0x50_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(WEATHER_PAINTERS);
        let painter = off.wrapping_add(WEATHER_PAINTERS);
        c.g[A0][0] = sx(painter);
        let holder = Addr(painter + WEATHER_HOLDER).lw(c);
        c.g[V1][0] = holder;
        let obj = Addr(holder as u32).lw(c);
        c.g[A2][0] = obj;
        let vt = Addr((obj as u32).wrapping_add(4)).lw(c);
        c.g[A1][0] = vt;
        let slot = crate::call::VSlot::read(c, vt, vt_off);
        c.g[A0][0] = slot.delta;
        c.g[V0][0] = slot.func;
        c.g[A0][0] = sx((obj as u32).wrapping_add(slot.delta as u32));
        call_ptr(c, slot.func as u32, ret_addr)?;
        c.lwc1(0, r32(c, V0));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float WorldPainter_GetWeather28(int index)` (0x2ef0a0): weather method
/// at v-table +0x178.
pub fn world_painter_get_weather28(c: &mut Cpu) {
    weather_slot_float_getter(c, 0x178, 0x002e_f0d8);
}

/// `float WorldPainter_GetWeather90(int index)` (0x2ee930): weather method
/// at v-table +0x1e0.
pub fn world_painter_get_weather90(c: &mut Cpu) {
    weather_slot_float_getter(c, 0x1e0, 0x002e_e968);
}

/// `float WorldPainter_GetWeather10(int index)` (0x2ee4c0): weather method
/// at v-table +0x160.
pub fn world_painter_get_weather10(c: &mut Cpu) {
    weather_slot_float_getter(c, 0x160, 0x002e_e4f8);
}

// ---------------------------------------------------------------------------
// cCrowdAnim2D_StepFrame (0x2dba08)

/// The step-mode jump table (`0x480000 + 0x75f0`, five entries in .rodata).
const CROWD_STEP_TABLE: u32 = 0x0048_75f0;
/// The five step modes' handlers (the table's entries).
const CROWD_STEP_FORWARD: u32 = 0x002d_ba30;
const CROWD_STEP_BACKWARD: u32 = 0x002d_ba44;
const CROWD_STEP_PINGPONG_UP: u32 = 0x002d_ba54;
const CROWD_STEP_PINGPONG_DOWN: u32 = 0x002d_ba7c;
const CROWD_STEP_LOOP: u32 = 0x002d_baa0;

/// `int cCrowdAnim2D_StepFrame(void* self, int frame, int count, int* mode)`
/// (0x2dba08): the next frame of a 2D crowd animation of `count` frames.
/// Mode 0 plays forward and holds the last frame, 1 backward to frame 0,
/// 2/3 ping-pong (switching `*mode` at the ends), 4 loops; single-frame
/// animations stay put in modes 2-4, unknown modes return `frame`.
pub fn crowd_anim2d_step_frame(c: &mut Cpu) {
    c.cycles += 4;
    let mode_at = Addr(r32(c, A3));
    let mode = mode_at.lw(c);
    c.g[V1][0] = mode;
    c.g[V0][0] = 0x48_0000;
    if mode < 5 {
        c.cycles += 6;
        let entry = ((mode as u32) << 2).wrapping_add(CROWD_STEP_TABLE);
        c.g[V1][0] = sx(entry);
        c.g[V0][0] = sx(CROWD_STEP_TABLE);
        let target = Addr(entry).lw(c);
        c.g[A0][0] = target;
        let count = c.g[A2][0];
        // `slt frame, count` on the full registers.
        fn below(c: &Cpu, count: u64) -> u64 {
            ((c.g[A1][0] as i64) < (count as i64)) as u64
        }
        match target as u32 {
            CROWD_STEP_FORWARD => {
                c.cycles += 5;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                c.g[V1][0] = sx((count as u32).wrapping_sub(1));
                c.g[V0][0] = below(c, count);
                if c.g[V0][0] == 0 {
                    c.g[A1][0] = c.g[V1][0];
                }
            }
            CROWD_STEP_BACKWARD => {
                c.cycles += 4;
                c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
                c.g[V0][0] = (-1i64 < c.g[A1][0] as i64) as u64;
                c.cycles += 1;
                if c.g[V0][0] == 0 {
                    c.g[A1][0] = 0;
                }
            }
            CROWD_STEP_PINGPONG_UP => {
                c.cycles += 3;
                c.g[V0][0] = 0x1;
                if count != 1 {
                    c.cycles += 4;
                    c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                    let inside = below(c, count) != 0;
                    c.g[V0][0] = 0x3;
                    if !inside {
                        // Past the end: turn around.
                        c.cycles += 3;
                        c.g[A1][0] = sx((count as u32).wrapping_sub(2));
                        mode_at.sw(c, 0x3);
                    }
                }
            }
            CROWD_STEP_PINGPONG_DOWN => {
                c.cycles += 3;
                c.g[V0][0] = 0x1;
                if count != 1 {
                    c.cycles += 3;
                    c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
                    let inside = (c.g[A1][0] as i64) >= 0;
                    c.g[V0][0] = 0x2;
                    if !inside {
                        // Before the start: turn around.
                        c.cycles += 3;
                        c.g[A1][0] = 0x1;
                        mode_at.sw(c, 0x2);
                    }
                }
            }
            CROWD_STEP_LOOP => {
                c.cycles += 3;
                c.g[V0][0] = 0x1;
                if count != 1 {
                    c.cycles += 2;
                    c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                    c.g[V0][0] = below(c, count);
                    c.cycles += 1;
                    if c.g[V0][0] == 0 {
                        c.g[A1][0] = 0;
                    }
                }
            }
            other => {
                c.jump_out(other, 0x002d_ba08, 0x002d_bac0);
                return;
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A1][0];
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, Rng, DATA, GAME_GP};

    /// `RiderMetrix_SetSpinAmount`: random and edge-case old/new spin
    /// amounts and totals (including totals that would go negative), the
    /// game's weight constant or an edge case.
    #[test]
    fn rider_metrix_set_spin_amount_matches() {
        check(ee::s00::f_00119898__generated, rider_metrix_set_spin_amount, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x0011_9898, 0);
            c.pc = 0x0011_9898;
            let k = if seed % 5 == 0 { r.edge_f32() } else { 0x3c8e_fa35 };
            c.bus.write32(GAME_GP - SPIN_WEIGHT_GP, k);
            let pick = |r: &mut Rng, i: u32| match i % 3 {
                0 => r.edge_f32(),
                _ => r.f32(400.0),
            };
            c.bus.write32(DATA + SPIN_METRIX_SPIN_AMOUNT, pick(&mut r, seed));
            c.bus.write32(DATA + SPIN_METRIX_ROTATION_TOTAL, pick(&mut r, seed / 3));
            c.f[12] = pick(&mut r, seed / 9);
            c.g[A0][0] = sx(DATA);
        });
    }

    /// `RiderAnim_UpdateSeqType01`: the real track advance, weight fade and
    /// list removal on a random sequence (ended / faded / still running).
    #[test]
    fn rider_anim_seq_type01_update_matches() {
        use crate::difftest::fakes::ptr;
        check(ee::s00::f_00104940__generated, rider_anim_seq_type01_update, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x0010_4940, 0);
            crate::difftest::elf_image(c);
            c.pc = 0x0010_4940;
            let seq = ptr(&mut r);
            let list = ptr(&mut r);
            c.g[A0][0] = sx(ptr(&mut r));
            c.g[A1][0] = sx(list);
            c.g[A2][0] = sx(seq);
            for off in [0x8, 0xc, 0x10, 0x90, 0x94, 0x98, 0x9c, 0xa4, 0xa8] {
                let v = match r.next_u32() % 4 {
                    0 => 0,
                    1 => r.edge_f32(),
                    _ => r.f32(3.0),
                };
                c.bus.write32(seq + off, v);
            }
            c.bus.write32(seq + 0x1c, r.next_u32() & 1);
            c.bus.write32(seq + 0xa0, r.next_u32() & 1);
            let mut at = list + 4;
            for _ in 0..seed % 3 {
                let other = ptr(&mut r) + 0x8;
                c.bus.write32(at, other);
                at = other + 200;
            }
            c.f[12] = if seed % 4 == 0 { r.edge_f32() } else { r.f32(0.05) };
        });
    }

    /// `SND_ChannelStartVoices`: SPU / IOP DSP / EE channels (snd's channel
    /// fixture, `seed % 3`) with 0-4 voices, output mode 2 or not, a random
    /// start descriptor; the real apply/start/activate callees run (the EE
    /// codec table and stretch words are zero, so `SND_EeVoiceStart` takes
    /// its short path).
    #[test]
    fn snd_channel_start_voices_matches() {
        use crate::difftest::fill;
        use crate::game::lib::snd::tests::{channel_flags, setup_channel};
        /// `SND_GetPanGains`' per-speaker byte tables, the speaker layouts
        /// and the pan sine table (as snd's pan fixture fills them).
        const PAN_TABLES: u32 = 0x0050_ac38;
        const PAN_LAYOUTS: u32 = 0x0050_ab20;
        const PAN_SINES: u32 = 0x0044_dd78;
        const DESC: u32 = DATA + 0x5_0000;
        const SAMPLE_BASE: u32 = DATA + 0x5_1000;
        check(ee::s13::f_003c3ee0__generated, snd_channel_start_voices, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x003c_3ee0, 0);
            c.pc = 0x003c_3ee0;
            let flags = channel_flags(&mut r, seed);
            setup_channel(c, &mut r, seed, flags);
            let ch_id = c.g[A0][0];
            fill(c, &mut r, PAN_LAYOUTS, 0x30);
            fill(c, &mut r, PAN_SINES, 0x102);
            for i in 0..8 {
                c.bus.write32(PAN_TABLES + 4 * i, DATA + 0x4_0000 + 0x100 * i);
            }
            fill(c, &mut r, DATA + 0x4_0000, 0x200);
            // The start descriptor: random words, the sample base pointer,
            // no stretch (SND_EeVoiceStart would allocate a stretcher).
            fill(c, &mut r, DESC, 0x30);
            c.bus.write32(DESC + SV_DESC_BASE_PTR, SAMPLE_BASE);
            c.bus.write32(SAMPLE_BASE, r.next_u32());
            for i in 0..6 {
                c.bus.write32(DESC + SV_DESC_STRETCH + 4 * i, 0);
            }
            c.bus.write32(SV_SYNC + SV_SYNC_KEY_SEQ, r.next_u32());
            c.g[A0][0] = sx(DESC);
            c.g[A1][0] = sx(r.next_u32());
            c.g[A2][0] = ch_id;
        });
    }

    /// `AnimSeqList_Save`: a stream whose Write is a fake (some seeds
    /// unwind, at any call), lists of -1..3 sequences (each with 0-1 keys)
    /// saved by the real `AnimSequence_Save`.
    #[test]
    fn anim_seq_list_save_matches() {
        use crate::difftest::fakes::{covered, FAKE};
        use crate::difftest::fill;
        covered(ee::s10::f_003147f0__generated, anim_seq_list_save, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x0031_47f0, 40);
            c.pc = 0x0031_47f0;
            let list = DATA + 0x1000;
            let stream = DATA + 0x2000;
            let vt = DATA + 0x3000;
            c.bus.write32(stream, vt);
            c.bus.write16(vt + 0x8, ((r.next_u32() & 0x3f) * 4) as u16);
            c.bus.write32(vt + 0xc, FAKE);
            let count = (seed % 5) as i32 - 1;
            let mut next = 0;
            for k in (0..count.max(0) as u32).rev() {
                let seq = DATA + 0x4000 + 0x200 * k;
                fill(c, &mut r, seq, 0x34);
                let key = if r.next_u32() & 1 != 0 {
                    let key = seq + 0x100;
                    fill(c, &mut r, key, 4);
                    c.bus.write32(key + 0xc, 0);
                    key
                } else {
                    0
                };
                c.bus.write32(seq + 0xac, key);
                c.bus.write32(seq + SEQLIST_NODE_NEXT, next);
                next = seq;
            }
            c.bus.write32(list + SEQLIST_COUNT, count as u32);
            c.bus.write32(list + SEQLIST_FIRST, next);
            c.g[A0][0] = sx(list);
            c.g[A1][0] = sx(stream);
        });
    }

    /// The `RiderBumpSub_HandleInput` fixture: the prewind input fixture's
    /// rider (snd.rs `prewind_input_fixture`) with a bump pad, every main
    /// state the function branches on and a finished / running layer-2
    /// animation.
    mod bump_input_fixture {
        use super::super::*;
        use crate::difftest::fakes::{fake_call, machine, FAKE};
        use crate::difftest::{elf_image, put_vec, Rng, DATA, GAME_GP};

        const THIS: u32 = DATA + 0x7_0000;
        const PAD: u32 = DATA + 0x7_0100;
        const RIDER: u32 = DATA + 0x7_1000;
        const OWNER: u32 = DATA + 0x7_2000;
        const CTRL: u32 = DATA + 0x7_3000;
        const CTRL_VT: u32 = DATA + 0x7_3100;
        const SEQ_LISTS: u32 = DATA + 0x7_3200;
        const SEQ: u32 = DATA + 0x7_3300;
        const MODEL: u32 = DATA + 0x7_4000;
        const BONES: u32 = DATA + 0x7_4100;
        const LIST: u32 = DATA + 0x7_5000;
        const LIST_TREE: u32 = DATA + 0x7_5400;
        const METRIX: u32 = DATA + 0x7_6000;
        const AUDIO: u32 = DATA + 0x7_7000;
        const BE_LIBRARY: u32 = DATA + 0x7_8000;
        const BE_OBJ: u32 = DATA + 0x7_8100;
        const BE_VT: u32 = DATA + 0x7_8200;
        const ANIM_TABLE: u32 = 0x0044_6990;
        const BE_INTERFACES: u32 = 0x0053_08a8;
        /// `RiderBase_EnterSubState`'s jump table (zeroed: the enter
        /// handlers become fakes, which unwind the call chain).
        const ENTER_SUB_STATE_TABLE: u32 = 0x0045_6bd0;
        /// The controller's "current animation kind" method: answers 0x207
        /// (`PlayAnim` stops there).
        const FAKE_ANIM_KIND: u32 = FAKE + 0x10;

        fn bump_miss(c: &mut Cpu, target: u32) {
            fake_call(c, target);
            if target == FAKE_ANIM_KIND {
                c.g[V0][0] = 0x207;
            }
        }

        /// A 6-bit signed axis: centred, small, negative or random.
        fn bump_axis(r: &mut Rng) -> u32 {
            match r.next_u32() % 4 {
                0 => 0,
                1 => 3,
                2 => 0x3d,
                _ => r.next_u32() & 0x3f,
            }
        }

        pub(super) fn setup(c: &mut Cpu, seed: u32) {
            let mut r = machine(c, seed, 0x0012_e778, 12);
            let r = &mut r;
            c.hooks.miss = bump_miss;
            elf_image(c);
            let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
            for i in 0..14 {
                w(c, ENTER_SUB_STATE_TABLE + 4 * i, 0);
            }
            // The .lit4 scales (the game's or edge cases).
            if seed % 7 == 0 {
                for off in [BUMP_AXIS_SCALE_GP, BUMP_LEAN_SCALE_GP, BUMP_SLEW1_RATE_GP] {
                    let v = r.edge_f32();
                    w(c, GAME_GP - off, v);
                }
            }

            // The back end: the library exists, interface getters are fakes.
            w(c, GAME_GP - 0x1f04, BE_LIBRARY);
            w(c, GAME_GP - 0x1efc, 0);
            w(c, BE_INTERFACES, BE_OBJ);
            w(c, BE_OBJ + 4, BE_VT);
            w(c, BE_VT + 0xc, FAKE);
            c.bus.write8(0x0053_08e2, 10);
            c.bus.write8(0x0053_08e4, 10);
            w(c, GAME_GP + 0x410, AUDIO);

            // The pad: never the reset bit; random boost bits; both axes.
            let mut buttons = r.next_u32() & !0x0000_1000;
            buttons &= !(0x3f << 15 | 0x3f << 21);
            buttons |= bump_axis(r) << 15 | bump_axis(r) << 21;
            w(c, PAD, buttons);
            w(c, PAD + 4, r.next_u32());

            w(c, THIS, RIDER);
            // The rider: racing (finished on every 16th seed); main state
            // 1 (airborne), 4, 2 or 0; no sub-state to exit.
            w(c, RIDER + 0x470, if seed % 16 == 15 { 1.0f32.to_bits() } else { (-1.0f32).to_bits() });
            w(c, RIDER + 0x77c, OWNER);
            w(c, OWNER + 0xde0, [1, 4, 2, 0][((seed / 2) % 4) as usize]);
            w(c, OWNER + 0xde4, 0x20);
            w(c, OWNER + 0x2b8, RIDER);
            w(c, RIDER + 0x328, 0);
            w(c, RIDER + 0x304, 3);
            w(c, RIDER + 0x780, MODEL);
            w(c, MODEL + 0x2c, BONES);
            w(c, RIDER + 0x860, LIST);
            w(c, LIST, LIST_TREE);
            w(c, RIDER + 0x790, METRIX);
            w(c, METRIX + 0x1ac, RIDER);
            for off in [0x220, 0x2a4, 0x2b0] {
                w(c, RIDER + off, r.f32(2.0));
            }
            w(c, RIDER + 0x2dc, if r.next_u32() % 2 == 0 { 0 } else { r.f32(1.0) });
            put_vec(c, RIDER + BUMP_RIDER_VELOCITY, [r.f32(5.0), r.f32(5.0), r.f32(5.0), 0]);
            put_vec(c, RIDER + BUMP_RIDER_FACING, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 0]);
            for off in [0x100, 0x110, 0x120, 0x130] {
                put_vec(c, RIDER + off, [r.f32(1.0), r.f32(1.0), r.f32(1.0), 1.0f32.to_bits()]);
            }

            // The animation controller: layer 2's sequence finished on
            // every other seed.
            w(c, RIDER + BUMP_RIDER_ANIM_CTRL, CTRL);
            w(c, CTRL + 0x50, SEQ_LISTS);
            w(c, CTRL + 0x58, CTRL_VT);
            w(c, CTRL_VT + 0x24, FAKE_ANIM_KIND);
            for k in 0..4 {
                w(c, SEQ_LISTS + 8 * k + 4, SEQ);
            }
            w(c, SEQ + 0xc0, if seed % 2 == 0 { 0 } else { 1 });
            for id in [0x12, 0x13, 0x14, 0x44, 0x45, 0x46] {
                w(c, ANIM_TABLE + id * 0x1c, 0);
            }
            w(c, CTRL + 8, 0x31);

            c.g[A0][0] = sx(THIS);
            c.g[A1][0] = sx(PAD);
            c.pc = 0x0012_e778;
        }
    }

    /// `RiderBumpSub_HandleInput`: finished or not, main states 1 / 4 /
    /// other, forwards and backwards travel, the layer-2 animation finished
    /// (land animation + sub-state 7, or grind reset + sub-state 4/0) or
    /// running; real callees, fakes that unwind.
    #[test]
    fn rider_bump_sub_handle_input_matches() {
        check(ee::s00::f_0012e778__generated, rider_bump_sub_handle_input, 300, &bump_input_fixture::setup);
    }

    /// The weather-getter fixture: painter index 0..3, its weather holder,
    /// object and v-table at random pointers, the slot `{delta, FAKE}`;
    /// the fake (which may unwind) returns a pointer to random data.
    fn weather_getter_253_setup(c: &mut Cpu, seed: u32, entry_pc: u32, vt_off: u32) {
        use crate::difftest::fakes::{ptr, FAKE};
        let mut r = machine(c, seed, entry_pc as u64, 1);
        c.pc = entry_pc;
        let index = seed % 4;
        c.g[A0][0] = index as u64;
        let entry = WEATHER_PAINTERS + index * 0xf0;
        let holder = ptr(&mut r);
        let obj = ptr(&mut r);
        let vt = ptr(&mut r);
        c.bus.write32(entry + WEATHER_HOLDER, holder);
        c.bus.write32(holder, obj);
        c.bus.write32(obj + 4, vt);
        let delta = (r.next_u32() as i16 as i32 & !3) as u32;
        c.bus.write16(vt + vt_off, delta as u16);
        c.bus.write32(vt + vt_off + 4, FAKE);
    }

    /// `WorldPainter_GetWeather28`.
    #[test]
    fn world_painter_get_weather28_matches() {
        crate::difftest::fakes::covered(ee::s09::f_002ef0a0__generated, world_painter_get_weather28, 200, &|c, seed| {
            weather_getter_253_setup(c, seed, 0x002e_f0a0, 0x178)
        });
    }

    /// `WorldPainter_GetWeather90`.
    #[test]
    fn world_painter_get_weather90_matches() {
        crate::difftest::fakes::covered(ee::s09::f_002ee930__generated, world_painter_get_weather90, 200, &|c, seed| {
            weather_getter_253_setup(c, seed, 0x002e_e930, 0x1e0)
        });
    }

    /// `WorldPainter_GetWeather10`.
    #[test]
    fn world_painter_get_weather10_matches() {
        crate::difftest::fakes::covered(ee::s09::f_002ee4c0__generated, world_painter_get_weather10, 200, &|c, seed| {
            weather_getter_253_setup(c, seed, 0x002e_e4c0, 0x160)
        });
    }

    /// `cCrowdAnim2D_StepFrame`: modes 0..4 and out of range, one-frame and
    /// longer animations, frames at / near both ends and random (the real
    /// jump table from the game image).
    #[test]
    fn crowd_anim2d_step_frame_matches() {
        use crate::difftest::elf_image;
        check(ee::s09::f_002dba08__generated, crowd_anim2d_step_frame, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x002d_ba08, 0);
            elf_image(c);
            c.pc = 0x002d_ba08;
            let mode = if seed % 13 == 12 { r.next_u32() } else { seed % 5 };
            c.bus.write32(DATA, mode);
            let count = match (seed / 5) % 4 {
                0 => 1,
                1 => 2,
                2 => r.next_u32() % 40,
                _ => r.next_u32(),
            };
            let frame = match (seed / 20) % 5 {
                0 => 0,
                1 => count.wrapping_sub(1),
                2 => count,
                3 => u32::MAX,
                _ => r.next_u32() % 40,
            };
            c.g[A1][0] = sx(frame);
            c.g[A2][0] = sx(count);
            c.g[A3][0] = sx(DATA);
        });
    }
}

