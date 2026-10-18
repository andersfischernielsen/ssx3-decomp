//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for sound DSP (channels, UTK, EAXA, resampling, 3D pan), each with a differential test against the generated
//! body.
//!
//! - `SND_ChannelApplyPitch` / `SND_ChannelApplyVolume`: push a channel's
//!   pitch / volume to its voices (SPU voice parameter blocks, the IOP DSP
//!   block, or the EE software voices through `SND_EeVoiceSet*`).
//! - `SND_UtkLpcFilter`, `SND_UtkDecodeExcitation`: the MicroTalk (UTK)
//!   speech decoder's 12-tap synthesis filter and excitation decoder.
//! - `SND_EaxaDecodeBlocks`: EA-XA ADPCM block decoding (2-tap predictor).
//! - `SSXAudioSystem_Compute3DPan`: listener-relative azimuth/elevation and
//!   squared distance of an emitter.
//!
//! `SND_DspResample_Process` (0x3cb540) is not replaced: its inner loop is a
//! hand-scheduled MMI + VU0 kernel that keeps four sample pointers in the
//! 32-bit lanes of 128-bit GPRs (`pextlw`/`pinteh`/`qfsrv` under `mtsab`,
//! `lw` through `t6`/`t7`/`s0`/`s1` and their upper lanes via `>> 32`) and
//! leaves every lane of t0..t9, s0..s7 and vf2..vf13 changed; reproducing
//! that register state exactly would mean re-emitting the instruction
//! stream, not a typed rewrite.

use crate::abi::{self, addu, r32, reg, set_reg};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due, Resume};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003c_25e0, snd_channel_apply_pitch, "SND_ChannelApplyPitch"),
    (0x003c_2a50, snd_channel_apply_volume, "SND_ChannelApplyVolume"),
    (0x003c_e410, snd_utk_lpc_filter, "SND_UtkLpcFilter"),
    (0x003c_d2b0, snd_utk_decode_excitation, "SND_UtkDecodeExcitation"),
    (0x003c_ca08, snd_eaxa_decode_blocks, "SND_EaxaDecodeBlocks"),
    (0x0028_5930, ssx_audio_system_compute_3d_pan, "SSXAudioSystem_Compute3DPan"),
];

// ---------------------------------------------------------------------------
// Sound driver globals.

/// `lui 0x51`: the high half the driver builds its global addresses from.
const STATE_HI: u64 = 0x0051_0000;
/// The driver state block (`0x510000 - 0x5718`).
const STATE: u32 = 0x0050_a8e8;
/// The driver's second global block (`0x510000 - 0x5300`).
const SYNC: u32 = 0x0050_ad00;

/// Fields of the driver state block.
mod state {
    /// `u16`: sample rate the EE voices' pitch is relative to.
    pub const EE_RATE: u32 = 0x2a;
    /// `u16`: sample rate the IOP DSP voices' pitch is relative to.
    pub const IOP_RATE: u32 = 0x2c;
    /// `u8`: first EE voice number (subtracted, after `VOICE_BASE`).
    pub const EE_VOICE_BASE: u32 = 0x2f;
    /// `u8`: first voice number of the IOP block.
    pub const VOICE_BASE: u32 = 0x30;
    /// `u8`: speaker count (4 = quad).
    pub const SPEAKERS: u32 = 0x47;
    /// `u8`: auxiliary sends (outputs = speakers + sends).
    pub const AUX_SENDS: u32 = 0x48;
    /// `u8`: SPU output mode; 2 drives a second voice block (+0x18).
    pub const OUTPUT_MODE: u32 = 0x49;
    /// `SndChannel*`: the channel array.
    pub const CHANNELS: u32 = 0x1f0;
}

/// Fields of the second global block.
mod sync {
    /// `u8*`: the voice parameter blocks (SPU: 8 bytes per voice; IOP
    /// DSP: 16 bytes per voice at +0x1a4).
    pub const VOICE_PARAMS: u32 = 0xcb0;
    /// `float[voice][8]`: per-voice output gains (32 bytes per voice).
    pub const GAINS: u32 = 0xe98;
}

/// `SndChannel` (0x8c bytes).
mod chan {
    pub const SIZE: u32 = 0x8c;
    /// `s16[]`: the channel's voice numbers.
    pub const VOICES: u32 = 0x04;
    /// `u16`: pitch (4.12 relative to the output rate).
    pub const PITCH: u32 = 0x20;
    /// `u8`: number of voices.
    pub const VOICE_COUNT: u32 = 0x23;
    /// `u16`: flags.
    pub const FLAGS: u32 = 0x24;
    /// `float`: final volume.
    pub const VOLUME: u32 = 0x48;
    /// `s8`: pan.
    pub const PAN: u32 = 0x5f;
    /// `u16`: pitch scale.
    pub const PITCH_SCALE: u32 = 0x8a;
    /// Flag: the voices are SPU voices.
    pub const SPU: u64 = 0x8;
    /// Flag: the voices live in the IOP DSP block.
    pub const IOP: u64 = 0x100;
}

/// `SND_EeVoiceSetPitch(voice, rate)` (function table, cross-shard).
const EE_VOICE_SET_PITCH: u32 = 0x003c_8a88;
/// `SND_EeVoiceSetVolume(voice, output, gain)` (function table).
const EE_VOICE_SET_VOLUME: u32 = 0x003c_8560;
/// 127.0, the 7-bit volume scale.
const VOL_SCALE: FpuF32 = FpuF32(0x42fe_0000);
/// 1/127.
const INV_127: FpuF32 = FpuF32(0x3c01_0204);

// ---------------------------------------------------------------------------
// Register arithmetic as the compiled code does it.

/// `addiu`.
#[inline(always)]
fn addiu(a: u64, imm: i32) -> u64 {
    sx((a as u32).wrapping_add(imm as u32))
}

/// `subu`.
#[inline(always)]
fn subu(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_sub(b as u32))
}

/// `sll`.
#[inline(always)]
fn sll(v: u64, n: u32) -> u64 {
    sx((v as u32) << n)
}

/// `slt` on whole registers.
#[inline(always)]
fn slt(a: u64, b: u64) -> u64 {
    ((a as i64) < (b as i64)) as u64
}

/// The address `base + off` (32-bit).
#[inline(always)]
fn at(base: u64, off: u32) -> Addr {
    Addr((base as u32).wrapping_add(off))
}

/// An FPU register.
#[inline(always)]
fn fpr(c: &Cpu, r: usize) -> FpuF32 {
    FpuF32(c.f[r])
}

/// `cvt.w.s` then `mfc1`: the integer bits.
#[inline(always)]
fn cvt_w(v: FpuF32) -> u32 {
    v.to_i32() as u32
}

/// The channel `index` (`STATE->channels + index * 0x8c`); the product is
/// a `mult` (LO/HI set) whose low word lands in a0.
fn channel_at(c: &mut Cpu) -> u64 {
    let index = r32(c, A0);
    let offset = abi::mult(c, index, chan::SIZE);
    set_reg(c, A0, sx(offset));
    offset as u64
}

// ---------------------------------------------------------------------------
// SND_ChannelApplyPitch.

/// `int SND_ChannelApplyPitch(int channel)` (0x3c25e0): push the channel's
/// pitch to its voices. Returns 0.
///
/// ```text
/// SPU voices:  rate = min(pitch * 4096 / 48000 * scale >> 12, 0x3fff)
///              params[v].pitch (+0x24) = rate   (and params[v + 0x18] in mode 2)
/// IOP voices:  rate = pitch * 4096 / iop_rate * scale >> 12
///              params[(v - base) * 2].dsp_pitch (+0x1a4) = rate
/// EE voices:   rate = pitch * 4096 / ee_rate * scale >> 8
///              SND_EeVoiceSetPitch(v - base - ee_base, rate)
/// ```
///
/// Frame 0x60 (s0 +0x50, s1 +0x40, s2 +0x30, s3 +0x20, s4 +0x10, ra +0).
pub fn snd_channel_apply_pitch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x60);
        set_reg(c, V0, chan::SIZE as u64);
        frame.save(c, S2, 0x30);
        set_reg(c, V1, STATE_HI);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        let offset = channel_at(c);
        set_reg(c, A3, sx(STATE));
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        let channels = Addr(STATE).off(state::CHANNELS).lw(c);
        let ch = addu(channels, sx(offset as u32));
        set_reg(c, S2, ch);
        let flags = at(ch, chan::FLAGS).lhu(c);
        set_reg(c, V1, flags);
        set_reg(c, V0, flags & chan::IOP);
        if flags & chan::SPU != 0 {
            return pitch_spu(c, frame, ch);
        }
        c.cycles += 2;
        set_reg(c, S1, 0);
        if flags & chan::IOP != 0 {
            pitch_iop(c, frame, ch)
        } else {
            pitch_ee(c, frame, ch)
        }
    })
}

/// `pitch << 12 / rate_base * scale` as the original computes it (`div`
/// then `mult`, LO/HI left by the `mult`); leaves the scale in a0 and the
/// product in v0, returns the product's low word.
fn pitch_ratio(c: &mut Cpu, ch: u64, rate_base: u64) -> u32 {
    let pitch = at(ch, chan::PITCH).lhu(c);
    let scaled = sll(pitch, 12);
    let (ratio, _) = abi::div(c, scaled as u32, rate_base as u32);
    let scale = at(ch, chan::PITCH_SCALE).lhu(c);
    set_reg(c, A0, scale);
    let product = abi::mult(c, ratio, scale as u32);
    set_reg(c, V0, sx(product));
    product
}

/// SPU voices: the rate goes into each voice's parameter block.
fn pitch_spu(c: &mut Cpu, frame: Frame, ch: u64) -> Flow {
    c.cycles += 4 + 12;
    set_reg(c, A2, 0x3fff);
    let product = pitch_ratio(c, ch, 0xbb80);
    let count = at(ch, chan::VOICE_COUNT).lbu(c);
    set_reg(c, A1, count);
    set_reg(c, S1, 0);
    let raw = sx(product >> 12);
    let in_range = (raw < 0x4000) as u64;
    set_reg(c, V1, in_range);
    set_reg(c, S3, if in_range != 0 { raw } else { 0x3fff });
    if count == 0 {
        return pitch_exit_early(c, frame);
    }
    c.cycles += 5;
    set_reg(c, V0, STATE_HI);
    set_reg(c, A2, 2);
    set_reg(c, A1, sx(SYNC));
    set_reg(c, A0, addiu(ch, chan::VOICES as i32));
    loop {
        // Loop head 0x3c2678 (a0 the voice slot, s1 the voices done).
        c.cycles += 8;
        let slot = reg(c, A0);
        let rate = reg(c, S3);
        let voice = at(slot, 0).lh(c);
        let entry = spu_voice_params(c, voice);
        at(entry, 0x24).sh(c, rate);
        let mode = Addr(STATE).off(state::OUTPUT_MODE).lbu(c);
        if mode == 2 {
            c.cycles += 7;
            let voice = at(slot, 0).lh(c);
            let entry = spu_voice_params(c, addiu(voice, 0x18));
            at(entry, 0x24).sh(c, rate);
        }
        let count = at(ch, chan::VOICE_COUNT).lbu(c);
        c.cycles += 4;
        let done = addiu(reg(c, S1), 1);
        set_reg(c, S1, done);
        let more = slt(done, count);
        set_reg(c, V0, more);
        set_reg(c, A0, addiu(slot, 2));
        if more == 0 {
            c.cycles += 2;
            frame.restore(c, S0, 0x50);
            return pitch_exit(c, frame);
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_2678);
        }
    }
}

/// `params + voice * 8` (left in v1).
fn spu_voice_params(c: &mut Cpu, voice: u64) -> u64 {
    let params = Addr(SYNC).off(sync::VOICE_PARAMS).lw(c);
    let entry = addu(params, sll(voice, 3));
    set_reg(c, V1, entry);
    entry
}

/// IOP DSP voices: the rate goes into the DSP block at +0x1a4.
fn pitch_iop(c: &mut Cpu, frame: Frame, ch: u64) -> Flow {
    c.cycles += 6 + 6;
    let rate_base = Addr(STATE).off(state::IOP_RATE).lhu(c);
    let product = pitch_ratio(c, ch, rate_base);
    let count = at(ch, chan::VOICE_COUNT).lbu(c);
    set_reg(c, V1, count);
    set_reg(c, S3, sx(product >> 12));
    if count == 0 {
        return pitch_exit_early(c, frame);
    }
    c.cycles += 3;
    set_reg(c, V0, STATE_HI);
    set_reg(c, A1, addiu(ch, chan::VOICES as i32));
    set_reg(c, A2, sx(SYNC));
    loop {
        // Loop head 0x3c2710 (a1 the voice slot, s1 the voices done).
        c.cycles += 12;
        let slot = reg(c, A1);
        let rate = reg(c, S3);
        let voice = at(slot, 0).lh(c);
        let done = addiu(reg(c, S1), 1);
        set_reg(c, S1, done);
        let base = Addr(STATE).off(state::VOICE_BASE).lbu(c);
        set_reg(c, A0, base);
        let params = Addr(SYNC).off(sync::VOICE_PARAMS).lw(c);
        let entry = addu(params, sll(subu(voice, base), 4));
        set_reg(c, V1, entry);
        at(entry, 0x1a4).sh(c, rate);
        let count = at(ch, chan::VOICE_COUNT).lbu(c);
        let more = slt(done, count);
        set_reg(c, V0, more);
        set_reg(c, A1, addiu(slot, 2));
        if more == 0 {
            c.cycles += 2;
            frame.restore(c, S0, 0x50);
            return pitch_exit(c, frame);
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_2710);
        }
    }
}

/// EE software voices: `SND_EeVoiceSetPitch(voice, rate)` per voice.
fn pitch_ee(c: &mut Cpu, frame: Frame, ch: u64) -> Flow {
    c.cycles += 6 + 6;
    let rate_base = Addr(STATE).off(state::EE_RATE).lhu(c);
    let product = pitch_ratio(c, ch, rate_base);
    let count = at(ch, chan::VOICE_COUNT).lbu(c);
    set_reg(c, V1, count);
    set_reg(c, S3, sx(product >> 8));
    if count == 0 {
        return pitch_exit_early(c, frame);
    }
    c.cycles += 4;
    set_reg(c, S4, sx(STATE));
    set_reg(c, S0, addiu(ch, chan::VOICES as i32));
    let ee_base = Addr(STATE).off(state::EE_VOICE_BASE).lbu(c);
    set_reg(c, V0, ee_base);
    loop {
        // Loop head 0x3c2788 (s0 the voice slot, s1 the voices done, v0
        // the EE voice base).
        c.cycles += 12;
        let rate = reg(c, S3);
        set_reg(c, A1, rate);
        let base = Addr(STATE).off(state::VOICE_BASE).lbu(c);
        let done = addiu(reg(c, S1), 1);
        set_reg(c, S1, done);
        let slot = reg(c, S0);
        let voice = at(slot, 0).lh(c);
        set_reg(c, V1, voice);
        let ee_base = reg(c, V0);
        set_reg(c, S0, addiu(slot, 2));
        set_reg(c, A0, subu(voice, addu(base, ee_base)));
        call_ptr(c, EE_VOICE_SET_PITCH, 0x003c_27a8)?;
        let count = at(ch, chan::VOICE_COUNT).lbu(c);
        let more = slt(reg(c, S1), count);
        set_reg(c, V0, more);
        if more == 0 {
            return pitch_exit_early(c, frame);
        }
        let ee_base = Addr(STATE).off(state::EE_VOICE_BASE).lbu(c);
        set_reg(c, V0, ee_base);
        if event_due(c) {
            deliver_event_here(c, 0x003c_2788);
        }
    }
}

/// The early exit (`lq s0` in its own block).
fn pitch_exit_early(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    frame.restore(c, S0, 0x50);
    pitch_exit(c, frame)
}

/// The epilogue: return 0.
fn pitch_exit(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 8;
    set_reg(c, V0, 0);
    frame.restore(c, S1, 0x40);
    frame.restore(c, S2, 0x30);
    frame.restore(c, S3, 0x20);
    frame.restore(c, S4, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// SND_ChannelApplyVolume.

/// Callee-saved GPRs of `SND_ChannelApplyVolume`'s frame (s0 is restored
/// separately, in the exit blocks).
const VOLUME_SAVES: [(usize, u32); 7] =
    [(S2, 0x70), (S3, 0x60), (S4, 0x50), (S5, 0x40), (S6, 0x30), (S7, 0x20), (FP, 0x10)];

/// `void SND_ChannelApplyVolume(int channel)` (0x3c2a50): push the
/// channel's volume to its voices.
///
/// ```text
/// SPU voices:  per voice, two (four in output mode 2) 7-bit levels
///              params[v].level{L,R} = (int)(gain[v][col] * volume * 127)
/// IOP voices:  g = volume * pan * (1/127)
///              dsp[(v - base) * 16 + k].level (+0x1a6) = (int)(g * gain[v][k] * 127)
///              for k < speakers + sends
/// EE voices:   SND_EeVoiceSetVolume(v - base - ee_base, k, g * gain[v][k])
/// ```
///
/// Frame 0xb0 (s0 +0x90, s1 +0x80 .. fp +0x10, ra +0, f20 +0xa0).
pub fn snd_channel_apply_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0xb0);
        set_reg(c, V0, chan::SIZE as u64);
        frame.save(c, S1, 0x80);
        frame.save(c, S0, 0x90);
        let offset = channel_at(c);
        set_reg(c, A3, STATE_HI);
        set_reg(c, A1, sx(STATE));
        for (r, off) in VOLUME_SAVES {
            frame.save(c, r, off);
        }
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0xa0);
        let channels = Addr(STATE).off(state::CHANNELS).lw(c);
        let ch = addu(channels, sx(offset as u32));
        set_reg(c, S1, ch);
        let flags = at(ch, chan::FLAGS).lhu(c);
        set_reg(c, V1, flags);
        set_reg(c, V0, flags & chan::IOP);
        if flags & chan::SPU != 0 {
            return volume_spu(c, frame, ch);
        }
        c.cycles += 2;
        set_reg(c, A2, 0);
        // The send gain shared by the IOP and EE paths.
        c.cycles += 14;
        let pan = at(ch, chan::PAN).lb(c);
        let volume = at(ch, chan::VOLUME).read_fpu(c);
        let pan_f = FpuF32::from_i32(pan as u32 as i32);
        let sends = Addr(STATE).off(state::AUX_SENDS).lbu(c);
        set_reg(c, A0, sends);
        let speakers = Addr(STATE).off(state::SPEAKERS).lbu(c);
        set_reg(c, V1, speakers);
        set_reg(c, AT, INV_127.0 as u64);
        let panned = volume * pan_f;
        let count = at(ch, chan::VOICE_COUNT).lbu(c);
        set_reg(c, V0, count);
        set_reg(c, S4, addu(speakers, sends));
        let gain = panned * INV_127;
        abi::set_fpr(c, 0, panned);
        abi::set_fpr(c, 1, pan_f);
        abi::set_fpr(c, 2, INV_127);
        abi::set_fpr(c, 20, gain);
        if count == 0 {
            return volume_exit_early(c, frame);
        }
        if flags & chan::IOP != 0 {
            volume_iop(c, frame, ch)
        } else {
            volume_ee(c, frame, ch)
        }
    })
}

/// SPU voices: 7-bit levels into the voice parameter blocks.
fn volume_spu(c: &mut Cpu, frame: Frame, ch: u64) -> Flow {
    c.cycles += 3;
    let count = at(ch, chan::VOICE_COUNT).lbu(c);
    set_reg(c, V0, count);
    set_reg(c, A2, 0);
    if count == 0 {
        return volume_exit_early(c, frame);
    }
    c.cycles += 11;
    set_reg(c, V0, STATE_HI);
    let state_reg = reg(c, A1);
    set_reg(c, T4, state_reg);
    set_reg(c, A3, sx(SYNC));
    set_reg(c, AT, VOL_SCALE.0 as u64);
    abi::set_fpr(c, 2, VOL_SCALE);
    set_reg(c, T0, sx(SYNC + sync::GAINS + 4));
    set_reg(c, T3, sx(SYNC + sync::GAINS));
    set_reg(c, T2, sx(SYNC + sync::GAINS + 0xc));
    set_reg(c, T1, sx(SYNC + sync::GAINS + 8));
    set_reg(c, A1, addiu(ch, chan::VOICES as i32));
    loop {
        // Loop head 0x3c2ae0 (a1 the voice slot, a2 the voices done).
        c.cycles += 4;
        let mode = Addr(STATE).off(state::OUTPUT_MODE).lbu(c);
        if mode != 2 {
            c.cycles += 21 + 11;
            spu_level(c, ch, 0x4, 0, 0x26);
            spu_level(c, ch, 0x0, 0, 0x27);
        } else {
            let speakers = Addr(STATE).off(state::SPEAKERS).lbu(c);
            c.cycles += 3;
            if speakers != 4 {
                c.cycles += 44 + 3 + 11;
                spu_level(c, ch, 0x10, 0, 0x26);
                spu_level(c, ch, 0x4, 0, 0x27);
                spu_level(c, ch, 0xc, 0x18, 0x26);
                spu_level(c, ch, 0x8, 0x18, 0x27);
            } else {
                c.cycles += 45 + 3 + 11;
                spu_level(c, ch, 0xc, 0, 0x26);
                spu_level(c, ch, 0x0, 0, 0x27);
                spu_level(c, ch, 0x8, 0x18, 0x26);
                spu_level(c, ch, 0x4, 0x18, 0x27);
            }
        }
        let count = at(ch, chan::VOICE_COUNT).lbu(c);
        let done = addiu(reg(c, A2), 1);
        set_reg(c, A2, done);
        let more = slt(done, count);
        set_reg(c, V0, more);
        let slot = reg(c, A1);
        set_reg(c, A1, addiu(slot, 2));
        if more == 0 {
            c.cycles += 2;
            frame.restore(c, S0, 0x90);
            return volume_exit(c, frame);
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_2ae0);
        }
    }
}

/// One SPU level: `params[voice + upper].byte = (int)(gain[voice][column] *
/// volume * 127)` (registers as the original leaves them: v1 the entry
/// offset, a0 the entry, v0 the level, f0 the float level, f1 its bits).
fn spu_level(c: &mut Cpu, ch: u64, column: u32, upper: i32, byte: u32) {
    let slot = reg(c, A1);
    let voice = at(slot, 0).lh(c);
    let volume = at(ch, chan::VOLUME).read_fpu(c);
    let params = Addr(SYNC).off(sync::VOICE_PARAMS).lw(c);
    let gain = Addr(SYNC + sync::GAINS + column).off((voice as u32) << 5).read_fpu(c);
    let entry_off = sll(addiu(voice, upper), 3);
    set_reg(c, V1, entry_off);
    let entry = addu(params, entry_off);
    set_reg(c, A0, entry);
    let level = (gain * volume) * fpr(c, 2);
    let bits = cvt_w(level);
    abi::set_fpr(c, 0, level);
    c.f[1] = bits;
    set_reg(c, V0, sx(bits));
    at(entry, byte).sb(c, sx(bits));
}

/// IOP DSP voices: a level per output into the DSP block (+0x1a6).
fn volume_iop(c: &mut Cpu, frame: Frame, ch: u64) -> Flow {
    c.cycles += 5;
    let hi = reg(c, A3);
    set_reg(c, T5, hi);
    set_reg(c, T4, STATE_HI);
    set_reg(c, T3, addiu(ch, chan::VOICES as i32));
    set_reg(c, S0, 0);
    loop {
        // Loop head 0x3c2d48 (a2 the voice index).
        c.cycles += 2;
        let i = reg(c, A2);
        set_reg(c, S5, addiu(i, 1));
        if reg(c, S4) != 0 {
            c.cycles += 8;
            set_reg(c, V0, sll(i, 1));
            set_reg(c, T0, sx(SYNC));
            set_reg(c, AT, VOL_SCALE.0 as u64);
            abi::set_fpr(c, 1, VOL_SCALE);
            let slots = reg(c, T3);
            set_reg(c, A3, addu(slots, sll(i, 1)));
            set_reg(c, T2, sx(SYNC + sync::GAINS));
            let hi = reg(c, T5);
            set_reg(c, T1, addiu(hi, -0x5718));
            loop {
                // Loop head 0x3c2d70 (a3 the voice slot, s0 the output).
                c.cycles += 20;
                let k = reg(c, S0);
                let slot = reg(c, A3);
                let outputs = reg(c, S4);
                let voice = at(slot, 0).lh(c);
                let base = Addr(STATE).off(state::VOICE_BASE).lbu(c);
                set_reg(c, A1, base);
                let voice_off = sll(voice, 5);
                set_reg(c, A0, voice_off);
                let params = Addr(SYNC).off(sync::VOICE_PARAMS).lw(c);
                let gain_at = addu(addu(sll(k, 2), voice_off), sx(SYNC + sync::GAINS));
                let gain = at(gain_at, 0).read_fpu(c);
                let dst = addu(params, addu(k, sll(subu(voice, base), 4)));
                set_reg(c, A2, dst);
                let next = addiu(k, 1);
                set_reg(c, S0, next);
                let level = (fpr(c, 20) * gain) * fpr(c, 1);
                let more = slt(next, outputs);
                set_reg(c, V1, more);
                let bits = cvt_w(level);
                abi::set_fpr(c, 0, level);
                c.f[2] = bits;
                set_reg(c, V0, sx(bits));
                at(dst, 0x1a6).sb(c, sx(bits));
                if more == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003c_2d70);
                }
            }
        }
        c.cycles += 5;
        let count = at(ch, chan::VOICE_COUNT).lbu(c);
        let next = reg(c, S5);
        set_reg(c, A2, next);
        let more = slt(next, count);
        set_reg(c, V0, more);
        set_reg(c, S0, 0);
        if more == 0 {
            c.cycles += 2;
            frame.restore(c, S0, 0x90);
            return volume_exit(c, frame);
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_2d48);
        }
    }
}

/// EE software voices: `SND_EeVoiceSetVolume(voice, k, g * gain[v][k])` per
/// voice and output.
fn volume_ee(c: &mut Cpu, frame: Frame, ch: u64) -> Flow {
    c.cycles += 3;
    set_reg(c, S7, addiu(ch, chan::VOICES as i32));
    set_reg(c, FP, STATE_HI);
    set_reg(c, S0, 0);
    loop {
        // Loop head 0x3c2e20 (a2 the voice index).
        c.cycles += 2;
        let i = reg(c, A2);
        set_reg(c, S5, addiu(i, 1));
        if reg(c, S4) != 0 {
            c.cycles += 6;
            set_reg(c, V0, sll(i, 1));
            set_reg(c, V1, STATE_HI);
            set_reg(c, S2, sx(STATE));
            let slots = reg(c, S7);
            let slot = addu(slots, sll(i, 1));
            set_reg(c, S3, slot);
            let hi = reg(c, FP);
            set_reg(c, S6, addiu(hi, -0x4468));
            let voice = at(slot, 0).lh(c);
            set_reg(c, A3, voice);
            loop {
                // Loop head 0x3c2e40 (a3 the voice, s0 the output).
                c.cycles += 16;
                let k = reg(c, S0);
                let voice = reg(c, A3);
                let gains = reg(c, S6);
                let base = Addr(STATE).off(state::VOICE_BASE).lbu(c);
                set_reg(c, A1, k);
                let voice_off = sll(voice, 5);
                set_reg(c, V1, voice_off);
                let ee_base = Addr(STATE).off(state::EE_VOICE_BASE).lbu(c);
                set_reg(c, A2, ee_base);
                let gain_at = addu(addu(sll(k, 2), voice_off), gains);
                set_reg(c, V0, gain_at);
                set_reg(c, S0, addiu(k, 1));
                let gain = at(gain_at, 0).read_fpu(c);
                set_reg(c, A0, subu(voice, addu(base, ee_base)));
                let send = fpr(c, 20) * gain;
                abi::set_fpr(c, 12, send);
                call_ptr(c, EE_VOICE_SET_VOLUME, 0x003c_2e74)?;
                let more = slt(reg(c, S0), reg(c, S4));
                set_reg(c, V0, more);
                if more == 0 {
                    break;
                }
                let slot = reg(c, S3);
                let voice = at(slot, 0).lh(c);
                set_reg(c, A3, voice);
                if event_due(c) {
                    deliver_event_here(c, 0x003c_2e40);
                }
            }
        }
        c.cycles += 5;
        let count = at(ch, chan::VOICE_COUNT).lbu(c);
        let next = reg(c, S5);
        set_reg(c, A2, next);
        let more = slt(next, count);
        set_reg(c, V0, more);
        set_reg(c, S0, 0);
        if more == 0 {
            return volume_exit_early(c, frame);
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_2e20);
        }
    }
}

/// The early exit (`lq s0` in its own block).
fn volume_exit_early(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    frame.restore(c, S0, 0x90);
    volume_exit(c, frame)
}

/// The epilogue.
fn volume_exit(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 12;
    frame.restore(c, S1, 0x80);
    for (r, off) in VOLUME_SAVES {
        frame.restore(c, r, off);
    }
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 20, 0xa0);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// UTK synthesis filter.

/// `SND_UtkLpcFilter(int blocks, float a[12], float hist[12], float* x)`
/// (0x3ce410): the 12-tap all-pole synthesis filter, in place over
/// `blocks` (at least one) blocks of 12 samples:
///
/// ```text
/// y[n] = x[n] + sum_k a[k] * y[n - 1 - k]
/// ```
///
/// The history lives in f12..f23 as a ring (sample j of a block reads tap
/// k from register `12 + (k - j) mod 12` and writes `23 - j`); each sum is
/// accumulated with ADDA/MADDA in the original order (tap 0 first in a
/// block's first sample, taps 1..11 then 0 in the others). The final
/// history is stored back to `hist`. f0..f23, f30, f31 (zero) and ACC are
/// left as the original leaves them (it saves none of f20..f31).
pub fn snd_utk_lpc_filter(c: &mut Cpu) {
    c.cycles += 25;
    let coefs = Addr(r32(c, A1));
    let history = Addr(r32(c, A2));
    let mut a = [FpuF32::ZERO; 12];
    for (k, tap) in a.iter_mut().enumerate() {
        *tap = coefs.off(4 * k as u32).read_fpu(c);
    }
    let zero = FpuF32::ZERO;
    let mut h = [FpuF32::ZERO; 12];
    for (k, y) in h.iter_mut().enumerate() {
        *y = history.off(4 * k as u32).read_fpu(c);
    }
    for k in 0..12 {
        c.f[k] = a[k].0;
    }
    c.f[31] = zero.0;
    loop {
        // Loop head 0x3ce474 (a0 blocks left, a3 the block).
        c.cycles += 196;
        let x_at = Addr(r32(c, A3));
        let left = addiu(reg(c, A0), -1);
        set_reg(c, A0, left);
        let mut x = FpuF32::ZERO;
        let mut acc = FpuF32::ZERO;
        for j in 0..12 {
            x = x_at.off(4 * j as u32).read_fpu(c);
            acc = x + zero;
            for t in 0..12 {
                let k = if j == 0 { t } else { (t + 1) % 12 };
                acc = FpuF32::madd(acc, a[k], h[(k + 12 - j) % 12]);
            }
            let y = FpuF32::madd(acc, zero, zero);
            h[11 - j] = y;
            x_at.off(4 * j as u32).write_fpu(c, y);
        }
        set_reg(c, A3, addiu(x_at.0 as u64, 0x30));
        for k in 0..12 {
            c.f[12 + k] = h[k].0;
        }
        c.f[30] = x.0;
        c.facc = acc.0;
        if (left as i64) <= 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_e474);
            for k in 0..12 {
                a[k] = fpr(c, k);
                h[k] = fpr(c, 12 + k);
            }
        }
    }
    c.cycles += 14;
    for (k, y) in h.iter().enumerate() {
        history.off(4 * k as u32).write_fpu(c, *y);
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// UTK excitation.

/// `SND_UtkReadBits(BitReader*, n)` / `SND_UtkSkipBits(BitReader*, n)`.
const UTK_READ_BITS: Callee = Callee { addr: 0x003c_d1f8, f: ee::s14::SND_UtkReadBits };
const UTK_SKIP_BITS: Callee = Callee { addr: 0x003c_d260, f: ee::s14::SND_UtkSkipBits };

/// Addresses of `SND_UtkDecodeExcitation`.
mod exc {
    pub const START: u32 = 0x003c_d2b0;
    pub const END: u32 = 0x003c_d514;
    /// Loop head: decode the next symbol (v0 the peeked byte).
    pub const SYMBOL: u32 = 0x003c_d310;
    /// Loop head: write a run of zeros.
    pub const ZERO_RUN: u32 = 0x003c_d3d0;
    /// Loop head: one more bit of an escape value's unary magnitude.
    pub const ESCAPE: u32 = 0x003c_d400;
    /// Loop head: one pulse-mode sample.
    pub const PULSE: u32 = 0x003c_d488;
    /// `u8[state][256]`: the Huffman symbol of a peeked byte.
    pub const SYMBOLS: u32 = 0x0044_e9e8;
    /// `{ int next_state; int bits; float value; }[]` per symbol.
    pub const CODES: u32 = 0x0044_ebe8;
    /// `&CODES[0].value`.
    pub const CODE_VALUES: u32 = 0x0044_ebf0;
    /// Samples per excitation frame (the loops stop at 108).
    pub const SAMPLES: u64 = 108;
}

/// `SND_UtkDecodeExcitation`'s frame: 0xb0 (s0 +0x90, s1 +0x80, s2 +0x70,
/// s3 +0x60, s4 +0x50, s5 +0x40, s6 +0x30, s7 +0x20, fp +0x10, ra +0, f21
/// +0xa8, f20 +0xa0).
const EXC_FRAME: Frame = Frame { size: 0xb0 };

/// Stages of `SND_UtkDecodeExcitation` (blocks of the original; the state
/// is in the registers throughout).
#[derive(Clone, Copy, PartialEq, Debug)]
enum Exc {
    Symbol,
    ZeroRunStart,
    ZeroRun,
    EscapeStart,
    EscapeMore,
    EscapeRead,
    Next,
    Tail,
    Pulse,
    Exit,
}

/// `void SND_UtkDecodeExcitation(BitReader* br, int huffman, float* out,
/// int stride)` (0x3cd2b0): decode the 108-sample excitation of a UTK
/// subframe into every `stride`-th sample of `out`.
///
/// ```text
/// huffman:  peek a byte; sym = SYMBOLS[state][byte]; skip CODES[sym].bits;
///           state = CODES[sym].next_state
///           sym >= 4: one sample CODES[sym].value
///           sym 2, 3: a run of (6 bits) + 7 zeros (clipped to the frame)
///           sym 0, 1: escape: magnitude 7 + count of 1 bits, then a sign
///                     bit (1 = positive)
/// pulse:    per sample, the low 2 bits: 1 -> -2.0, 3 -> +2.0 (2 bits),
///           otherwise 0.0 (1 bit)
/// ```
///
/// The original is a `match pc` dispatcher whose loops return on an unwind
/// request, so the loop heads are entry points too (the state is in s0..s7,
/// fp and the temporaries the original keeps there).
pub fn snd_utk_decode_excitation(c: &mut Cpu) {
    let mut stage = match c.pc {
        exc::START => exc_enter(c),
        exc::SYMBOL => Exc::Symbol,
        exc::ZERO_RUN => Exc::ZeroRun,
        exc::ESCAPE => Exc::EscapeMore,
        exc::PULSE => Exc::Pulse,
        pc => {
            c.jump_out(pc, exc::START, exc::END);
            return;
        }
    };
    loop {
        let next = match stage {
            Exc::Symbol => exc_symbol(c),
            Exc::ZeroRunStart => exc_zero_run_start(c),
            Exc::ZeroRun => exc_zero_run(c),
            Exc::EscapeStart => {
                c.cycles += 3;
                let pos = reg(c, S2);
                set_reg(c, S1, sll(pos, 2));
                let stride = reg(c, S3);
                set_reg(c, S2, addu(pos, stride));
                Resume::Go(Exc::EscapeRead)
            }
            Exc::EscapeMore => {
                c.cycles += 1;
                let n = reg(c, S0);
                set_reg(c, S0, addiu(n, 1));
                Resume::Go(Exc::EscapeRead)
            }
            Exc::EscapeRead => exc_escape_read(c),
            Exc::Next => {
                c.cycles += 1;
                let pos = reg(c, S2);
                set_reg(c, V0, slt(pos, exc::SAMPLES));
                Resume::Go(Exc::Tail)
            }
            Exc::Tail => exc_tail(c),
            Exc::Pulse => exc_pulse(c),
            Exc::Exit => {
                exc_exit(c);
                return;
            }
        };
        match next {
            Resume::Go(s) => stage = s,
            Resume::Done | Resume::Unwind => return,
        }
    }
}

/// The prologue and the mode set-up.
fn exc_enter(c: &mut Cpu) -> Exc {
    let frame = EXC_FRAME;
    c.cycles += 17;
    Frame::open(c, frame.size);
    frame.save(c, S3, 0x60);
    frame.save(c, S4, 0x50);
    let stride = reg(c, A3);
    set_reg(c, S3, stride);
    frame.save(c, S5, 0x40);
    let reader = reg(c, A0);
    set_reg(c, S4, reader);
    frame.save(c, S0, 0x90);
    let out = reg(c, A2);
    set_reg(c, S5, out);
    frame.save(c, S1, 0x80);
    frame.save(c, S2, 0x70);
    frame.save(c, S6, 0x30);
    frame.save(c, S7, 0x20);
    frame.save(c, FP, 0x10);
    frame.save_ra(c, 0);
    frame.save_fpr(c, 21, 0xa8);
    frame.save_fpr(c, 20, 0xa0);
    if reg(c, A1) == 0 {
        // Pulse mode: s1 the output, s5 the byte stride, f21/f20 = -2/+2.
        c.cycles += 10;
        set_reg(c, S1, out);
        set_reg(c, AT, 0x4000_0000);
        c.f[21] = 0xc000_0000;
        c.f[20] = 0x4000_0000;
        set_reg(c, S0, 0);
        set_reg(c, S7, 1);
        set_reg(c, S6, 3);
        set_reg(c, S5, sll(stride, 2));
        Exc::Pulse
    } else {
        // Huffman mode: fp the state, s7 the code table, s2 the position.
        c.cycles += 7;
        set_reg(c, FP, 0);
        set_reg(c, S7, sx(exc::CODES));
        set_reg(c, S2, 0);
        set_reg(c, S6, 1);
        let peek = at(reader, 4).lbu(c);
        set_reg(c, V0, peek);
        Exc::Symbol
    }
}

/// Decode one symbol (loop head 0x3cd310).
fn exc_symbol(c: &mut Cpu) -> Resume<Exc> {
    c.cycles += 17;
    let state = reg(c, FP);
    let peek = reg(c, V0);
    let reader = reg(c, S4);
    set_reg(c, A0, reader);
    let sym_at = addu(addu(peek, sll(state, 8)), sx(exc::SYMBOLS));
    let sym = at(sym_at, 0).lbu(c);
    set_reg(c, S1, sym);
    let entry_off = sx(abi::mult(c, sym as u32, 12));
    set_reg(c, S0, entry_off);
    let codes = reg(c, S7);
    let entry = addu(codes, entry_off);
    set_reg(c, V0, entry);
    set_reg(c, V1, entry);
    let bits = at(entry, 4).lw(c);
    set_reg(c, A1, bits);
    let next_state = at(entry, 0).lw(c);
    set_reg(c, FP, next_state);
    if UTK_SKIP_BITS.call(c, 0x003c_d348).is_err() {
        return Resume::Unwind;
    }
    let sym = reg(c, S1);
    set_reg(c, V0, slt(sym, 2));
    if slt(sym, 4) == 0 {
        // A value: out[pos] = CODES[sym].value.
        c.cycles += 8 + 1;
        let entry_off = reg(c, S0);
        let value_at = addu(entry_off, sx(exc::CODE_VALUES));
        set_reg(c, V0, value_at);
        let pos = reg(c, S2);
        let out = reg(c, S5);
        let out_at = addu(sll(pos, 2), out);
        set_reg(c, V1, out_at);
        let value = at(value_at, 0).read_fpu(c);
        abi::set_fpr(c, 0, value);
        let stride = reg(c, S3);
        set_reg(c, S2, addu(pos, stride));
        at(out_at, 0).write_fpu(c, value);
        return Resume::Go(Exc::Next);
    }
    c.cycles += 2;
    set_reg(c, S0, 7);
    Resume::Go(if sym < 2 { Exc::EscapeStart } else { Exc::ZeroRunStart })
}

/// A run of zeros: read its length, clip it to the frame.
fn exc_zero_run_start(c: &mut Cpu) -> Resume<Exc> {
    c.cycles += 9;
    let reader = reg(c, S4);
    set_reg(c, A0, reader);
    set_reg(c, A1, 6);
    if UTK_READ_BITS.call(c, 0x003c_d388).is_err() {
        return Resume::Unwind;
    }
    let run = addiu(reg(c, V0), 7);
    set_reg(c, A0, run);
    let stride = reg(c, S3);
    let pos = reg(c, S2);
    let span = sx(abi::mult(c, stride as u32, run as u32));
    set_reg(c, V1, span);
    let fits = slt(addu(span, pos), 109);
    set_reg(c, V0, 0x6c);
    let mut count = run;
    if fits == 0 {
        c.cycles += 2 + 4;
        let left = subu(0x6c, pos);
        let (q, _) = abi::div(c, left as u32, stride as u32);
        count = sx(q);
        set_reg(c, V0, count);
        set_reg(c, A0, count);
    }
    c.cycles += 2;
    set_reg(c, S0, 0);
    if (count as i64) <= 0 {
        return Resume::Go(Exc::Next);
    }
    c.cycles += 4;
    let first = sll(pos, 2);
    set_reg(c, V0, first);
    set_reg(c, A1, sll(stride, 2));
    let out = reg(c, S5);
    set_reg(c, V1, addu(first, out));
    Resume::Go(Exc::ZeroRun)
}

/// One zero of a run (loop head 0x3cd3d0: v1 the output, a1 the byte
/// stride, a0 the run, s0 the count).
fn exc_zero_run(c: &mut Cpu) -> Resume<Exc> {
    c.cycles += 7;
    let out = reg(c, V1);
    at(out, 0).sw(c, 0);
    let n = addiu(reg(c, S0), 1);
    set_reg(c, S0, n);
    let step = reg(c, A1);
    set_reg(c, V1, addu(out, step));
    let more = slt(n, reg(c, A0));
    set_reg(c, V0, more);
    let pos = addu(reg(c, S2), reg(c, S3));
    set_reg(c, S2, pos);
    if more != 0 {
        if back_edge_unwinds(c, exc::ZERO_RUN) {
            return Resume::Unwind;
        }
        return Resume::Go(Exc::ZeroRun);
    }
    c.cycles += 2;
    set_reg(c, V0, slt(pos, exc::SAMPLES));
    Resume::Go(Exc::Tail)
}

/// An escape value: count 1 bits onto the magnitude in s0, then the sign.
fn exc_escape_read(c: &mut Cpu) -> Resume<Exc> {
    c.cycles += 5;
    let reader = reg(c, S4);
    set_reg(c, A0, reader);
    set_reg(c, A1, 1);
    if UTK_READ_BITS.call(c, 0x003c_d410).is_err() {
        return Resume::Unwind;
    }
    let bit = reg(c, V0);
    set_reg(c, A0, reader);
    if bit == reg(c, S6) {
        if back_edge_unwinds(c, exc::ESCAPE) {
            return Resume::Unwind;
        }
        return Resume::Go(Exc::EscapeMore);
    }
    c.cycles += 4;
    set_reg(c, A1, 1);
    if UTK_READ_BITS.call(c, 0x003c_d420).is_err() {
        return Resume::Unwind;
    }
    let sign = reg(c, V0);
    let magnitude = reg(c, S0);
    let negated = subu(0, magnitude);
    set_reg(c, V0, negated);
    let slot = reg(c, S1);
    let out = reg(c, S5);
    let out_at = addu(slot, out);
    if sign != reg(c, S6) {
        c.cycles += 3 + 1;
        set_reg(c, V1, out_at);
        let value = FpuF32::from_i32(negated as u32 as i32);
        abi::set_fpr(c, 0, value);
        at(out_at, 0).write_fpu(c, value);
    } else {
        c.cycles += 5;
        let value = FpuF32::from_i32(magnitude as u32 as i32);
        abi::set_fpr(c, 0, value);
        set_reg(c, V0, out_at);
        at(out_at, 0).write_fpu(c, value);
    }
    Resume::Go(Exc::Next)
}

/// The Huffman loop's back edge (v0: more samples to go).
fn exc_tail(c: &mut Cpu) -> Resume<Exc> {
    c.cycles += 2;
    if reg(c, V0) != 0 {
        let reader = reg(c, S4);
        let peek = at(reader, 4).lbu(c);
        set_reg(c, V0, peek);
        if back_edge_unwinds(c, exc::SYMBOL) {
            return Resume::Unwind;
        }
        return Resume::Go(Exc::Symbol);
    }
    c.cycles += 2;
    EXC_FRAME.restore(c, S0, 0x90);
    Resume::Go(Exc::Exit)
}

/// One pulse-mode sample (loop head 0x3cd488: s1 the output, s0 the
/// position).
fn exc_pulse(c: &mut Cpu) -> Resume<Exc> {
    c.cycles += 4;
    let reader = reg(c, S4);
    let bits = at(reader, 4).lw(c);
    set_reg(c, V0, bits);
    let code = bits & 3;
    set_reg(c, S2, code);
    let out = reg(c, S1);
    let ret = if code == reg(c, S7) {
        c.cycles += 2 + 5;
        let minus_two = fpr(c, 21);
        at(out, 0).write_fpu(c, minus_two);
        set_reg(c, A1, 2);
        0x003c_d4b8
    } else if code != reg(c, S6) {
        c.cycles += 2 + 4;
        at(out, 0).sw(c, 0);
        set_reg(c, A1, 1);
        0x003c_d4cc
    } else {
        c.cycles += 2 + 1 + 5;
        let two = fpr(c, 20);
        at(out, 0).write_fpu(c, two);
        set_reg(c, A1, 2);
        0x003c_d4b8
    };
    set_reg(c, A0, reader);
    if UTK_SKIP_BITS.call(c, ret).is_err() {
        return Resume::Unwind;
    }
    let pos = addu(reg(c, S0), reg(c, S3));
    set_reg(c, S0, pos);
    c.cycles += 3;
    let more = slt(pos, exc::SAMPLES);
    set_reg(c, V0, more);
    let next = addu(reg(c, S1), reg(c, S5));
    set_reg(c, S1, next);
    if more != 0 {
        if back_edge_unwinds(c, exc::PULSE) {
            return Resume::Unwind;
        }
        return Resume::Go(Exc::Pulse);
    }
    c.cycles += 1;
    EXC_FRAME.restore(c, S0, 0x90);
    Resume::Go(Exc::Exit)
}

/// The epilogue (s0 already restored).
fn exc_exit(c: &mut Cpu) {
    let frame = EXC_FRAME;
    c.cycles += 13;
    frame.restore(c, S1, 0x80);
    frame.restore(c, S2, 0x70);
    frame.restore(c, S3, 0x60);
    frame.restore(c, S4, 0x50);
    frame.restore(c, S5, 0x40);
    frame.restore(c, S6, 0x30);
    frame.restore(c, S7, 0x20);
    frame.restore(c, FP, 0x10);
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 21, 0xa8);
    frame.restore_fpr(c, 20, 0xa0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// EA-XA.

/// `SND_EaxaDecodeRawBlock(EaxaState*)`: an uncompressed (0xee) block.
const EAXA_DECODE_RAW_BLOCK: Callee = Callee { addr: 0x003c_c938, f: ee::s14::SND_EaxaDecodeRawBlock };
/// The predictor coefficient table (`float c0[4..]`, `c1 = c0 + 4`;
/// `0x450000 - 0x1760`).
const EAXA_COEFS: u32 = 0x0044_e8a0;
/// Samples per block.
const EAXA_BLOCK_SAMPLES: i32 = 0x1c;

/// `EaxaState` fields.
mod eaxa {
    /// `int`: samples left to decode.
    pub const LEFT: u32 = 0x0;
    /// `float`: the newest decoded sample.
    pub const HIST1: u32 = 0x4;
    /// `float`: the one before.
    pub const HIST2: u32 = 0x8;
    /// `u8*`: the input.
    pub const INPUT: u32 = 0xc;
    /// `float*`: the output.
    pub const OUTPUT: u32 = 0x10;
}

/// `void SND_EaxaDecodeBlocks(EaxaState* s)` (0x3cca08): decode 28-sample
/// blocks while `s->left > 0`. A block is a header byte (high nibble: the
/// predictor, low nibble: the shift) and 14 bytes of nibbles, high first:
///
/// ```text
/// y = (nibble << 28 >> (shift + 16)) + h1 * c0[p] + h2 * c1[p]
/// ```
///
/// (ADDA/MADDA/MADD: the nibble plus 0, then the two products, each
/// rounded). A 0xee header is a raw block (`SND_EaxaDecodeRawBlock`, with
/// a0..t4 kept in a 0x30 stack frame across the call). The state is written
/// back after every block. Frame 0x10 holding `ra` (`sw`, reloaded with
/// `lq`).
pub fn snd_eaxa_decode_blocks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let st = reg(c, A0);
        set_reg(c, V0, sx(EAXA_COEFS));
        set_reg(c, V1, st);
        set_reg(c, A1, sx(EAXA_COEFS));
        Frame::open(c, 0x10);
        let ra = reg(c, RA);
        Addr(sp(c)).sw(c, ra);
        let left = at(st, eaxa::LEFT).lw(c);
        set_reg(c, A2, left);
        c.f[10] = at(st, eaxa::HIST1).read_fpu(c).0;
        c.f[11] = at(st, eaxa::HIST2).read_fpu(c).0;
        let input = at(st, eaxa::INPUT).lw(c);
        set_reg(c, A3, input);
        let output = at(st, eaxa::OUTPUT).lw(c);
        set_reg(c, T0, output);
        loop {
            // Loop head 0x3cca38 (a2 left, a3 input, t0 output, f10/f11
            // the history).
            c.cycles += 2;
            let st = reg(c, A0);
            let left = reg(c, A2);
            if (left as i64) <= 0 {
                c.cycles += 9;
                at(st, eaxa::LEFT).sw(c, left);
                let (h1, h2) = (fpr(c, 10), fpr(c, 11));
                at(st, eaxa::HIST1).write_fpu(c, h1);
                at(st, eaxa::HIST2).write_fpu(c, h2);
                let (input, output) = (reg(c, A3), reg(c, T0));
                at(st, eaxa::INPUT).sw(c, input);
                at(st, eaxa::OUTPUT).sw(c, output);
                let frame_at = sp(c);
                c.lq(RA, frame_at);
                set_reg(c, SP, addiu(frame_at as u64, 0x10));
                c.pc = reg(c, RA) as u32;
                return Ok(());
            }
            c.cycles += 6;
            let input = reg(c, A3);
            let header = at(input, 0).lbu(c);
            set_reg(c, T1, header);
            set_reg(c, A2, addiu(left, -EAXA_BLOCK_SAMPLES));
            set_reg(c, T5, 0xee);
            let header2 = at(input, 0).lbu(c);
            set_reg(c, T6, header2);
            if header2 != 0xee {
                eaxa_decode_block(c);
            } else {
                eaxa_raw_block(c)?;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_ca38);
            }
        }
    })
}

/// The nibbles of input byte `p` as the original shifts them (t4 the high
/// sample, t3 the low one; returned converted to float).
fn eaxa_nibbles(c: &mut Cpu, input: u64, p: u32, hi_shift: u64, lo_shift: u64) -> (FpuF32, FpuF32) {
    let b = at(input, p).lb(c);
    let hi = sx(((b as u32) as i32 >> 4) as u32);
    let lo = sx((b as u32) << 28);
    let hi = sx((hi as u32) << ((hi_shift as u32) & 31));
    let lo = sx(((lo as u32) as i32 >> ((lo_shift as u32) & 31)) as u32);
    set_reg(c, T4, hi);
    set_reg(c, T3, lo);
    (FpuF32::from_i32(hi as u32 as i32), FpuF32::from_i32(lo as u32 as i32))
}

/// One compressed block (256 cycles).
fn eaxa_decode_block(c: &mut Cpu) {
    c.cycles += 256;
    let st = reg(c, A0);
    let input = reg(c, A3);
    let out = reg(c, T0);
    let header = reg(c, T1);
    let predictor = sx((header as u32) >> 4);
    let shift = header & 0xf;
    let coefs = addu(sll(predictor, 2), reg(c, A1));
    let c0 = at(coefs, 0).read_fpu(c);
    let c1 = at(coefs, 0x10).read_fpu(c);
    let hi_shift = subu(0xc, shift);
    set_reg(c, T2, hi_shift);
    let lo_shift = addiu(shift, 0x10);
    set_reg(c, T1, lo_shift);
    let zero = FpuF32::ZERO;
    // The nibble pair of byte p + 1 is in (f2, f3) for even p, (f4, f5)
    // for odd p.
    let mut pairs = [eaxa_nibbles(c, input, 1, hi_shift, lo_shift), (fpr(c, 4), fpr(c, 5))];
    let mut h1 = fpr(c, 10);
    let mut h2 = fpr(c, 11);
    let mut acc = FpuF32::ZERO;
    for p in 0..14u32 {
        let (hi, lo) = pairs[(p % 2) as usize];
        acc = FpuF32::madd(hi + zero, h1, c0);
        h2 = FpuF32::madd(acc, h2, c1);
        let next = if p < 13 { Some(eaxa_nibbles(c, input, p + 2, hi_shift, lo_shift)) } else { None };
        at(out, 8 * p).write_fpu(c, h2);
        acc = FpuF32::madd(lo + zero, h2, c0);
        h1 = FpuF32::madd(acc, h1, c1);
        if let Some(pair) = next {
            pairs[((p + 1) % 2) as usize] = pair;
        }
        at(out, 8 * p + 4).write_fpu(c, h1);
    }
    c.f[0] = c0.0;
    c.f[1] = c1.0;
    c.f[2] = pairs[0].0 .0;
    c.f[3] = pairs[0].1 .0;
    c.f[4] = pairs[1].0 .0;
    c.f[5] = pairs[1].1 .0;
    c.f[10] = h1.0;
    c.f[11] = h2.0;
    c.f[31] = zero.0;
    c.facc = acc.0;
    let input = addiu(input, 0xf);
    set_reg(c, A3, input);
    let out = addiu(out, 0x70);
    set_reg(c, T0, out);
    at(st, eaxa::HIST1).write_fpu(c, h1);
    at(st, eaxa::HIST2).write_fpu(c, h2);
    at(st, eaxa::INPUT).sw(c, input);
    at(st, eaxa::OUTPUT).sw(c, out);
}

/// Registers `SND_EaxaDecodeBlocks` keeps in a 0x30 frame across the raw
/// block call.
const EAXA_RAW_SAVES: [(u32, usize); 9] =
    [(0x20, A0), (0x1c, A1), (0x18, A2), (0x14, A3), (0x10, T0), (0xc, T1), (0x8, T2), (0x4, T3), (0x0, T4)];

/// A raw block: `SND_EaxaDecodeRawBlock(s)`, then reload the state.
fn eaxa_raw_block(c: &mut Cpu) -> Flow {
    c.cycles += 28;
    let frame = Frame::open(c, 0x30);
    for (off, r) in EAXA_RAW_SAVES {
        let v = reg(c, r);
        Addr(frame.at(c, off)).sw(c, v);
    }
    EAXA_DECODE_RAW_BLOCK.call(c, 0x003c_ca88)?;
    for (off, r) in EAXA_RAW_SAVES.iter().rev() {
        let v = Addr(frame.at(c, *off)).lw(c);
        set_reg(c, *r, v);
    }
    let top = addiu(reg(c, SP), 0x30);
    set_reg(c, SP, top);
    let st = reg(c, A0);
    c.f[10] = at(st, eaxa::HIST1).read_fpu(c).0;
    c.f[11] = at(st, eaxa::HIST2).read_fpu(c).0;
    let input = at(st, eaxa::INPUT).lw(c);
    set_reg(c, A3, input);
    let output = at(st, eaxa::OUTPUT).lw(c);
    set_reg(c, T0, output);
    Ok(())
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_Compute3DPan.

const GET_APP: Callee = Callee { addr: 0x0028_b1c0, f: ee::s07::SSXAudioSystem_GetApp };
const GET_GAME_MODULE: Callee = Callee { addr: 0x0028_b1c8, f: ee::s07::SSXAudioSystem_GetGameModule };
/// `float Math_Atan(float)` (function table, cross-shard).
const MATH_ATAN: u32 = 0x0031_c228;

/// gp-relative `.lit4` constants.
mod pan_k {
    /// 0.01: world units to audio units.
    pub const SCALE: i32 = -0x4658;
    /// -pi/2, +pi/2 (azimuth straight left/right).
    pub const AZ_LEFT: i32 = -0x4654;
    pub const AZ_RIGHT: i32 = -0x4650;
    /// pi (azimuth behind).
    pub const AZ_PI_ADD: i32 = -0x464c;
    pub const AZ_PI_SUB: i32 = -0x4648;
    /// 65536 / 2pi: radians to binary angle.
    pub const AZ_TO_ANGLE: i32 = -0x4644;
    /// -pi/2, +pi/2 (straight down/up).
    pub const EL_DOWN: i32 = -0x4640;
    pub const EL_UP: i32 = -0x463c;
    pub const EL_PI_ADD: i32 = -0x4638;
    pub const EL_PI_SUB: i32 = -0x4634;
    pub const EL_TO_ANGLE: i32 = -0x4630;
}

/// The listener offset: +25 up, -100 along z.
const PAN_UP: FpuF32 = FpuF32(0x41c8_0000);
const PAN_BACK: FpuF32 = FpuF32(0xc2c8_0000);

/// A gp-relative float.
fn gp_const(c: &mut Cpu, off: i32) -> FpuF32 {
    let gp = reg(c, GP);
    at(gp, off as u32).read_fpu(c)
}

/// `atan(y / x)` folded into a full angle the way the game does it: if
/// `x < 0`, add pi when `y > 0`, else subtract it; `x == 0` gives 0 (y ==
/// 0), `+pi/2` (y >= 0) or `-pi/2` (constants `zero_pos`/`zero_neg` and
/// `pi_add`/`pi_sub`). The FPU registers and FCC end as the original's
/// (`dst` is the register the angle is built in: f3 or f12).
#[allow(clippy::too_many_arguments)]
fn pan_angle(
    c: &mut Cpu,
    y: FpuF32,
    x: FpuF32,
    dst: usize,
    k: [i32; 4],
    ret: u32,
) -> Result<FpuF32, crate::call::Unwound> {
    let [zero_neg, zero_pos, pi_add, pi_sub] = k;
    let zero = FpuF32::ZERO;
    let x_zero = x.eq_s(zero);
    abi::set_fcc(c, x_zero);
    let angle = if x_zero {
        c.cycles += 4;
        let y_zero = y.eq_s(zero);
        abi::set_fcc(c, y_zero);
        if y_zero {
            y
        } else {
            c.cycles += 4;
            let up = zero.le(y);
            abi::set_fcc(c, up);
            let down = gp_const(c, zero_neg);
            if up {
                c.cycles += 3;
                gp_const(c, zero_pos)
            } else {
                down
            }
        }
    } else {
        c.cycles += 7;
        abi::set_fpr(c, 12, y.div(x));
        call_ptr(c, MATH_ATAN, ret)?;
        let behind = x.lt(zero);
        abi::set_fcc(c, behind);
        let mut a = fpr(c, 0);
        if behind {
            c.cycles += 3;
            let positive = zero.lt(y);
            abi::set_fcc(c, positive);
            if positive {
                c.cycles += 3;
                let pi = gp_const(c, pi_add);
                abi::set_fpr(c, 0, pi);
                a = a + pi;
            } else {
                c.cycles += 2;
                let pi = gp_const(c, pi_sub);
                abi::set_fpr(c, 0, pi);
                a = a - pi;
            }
        }
        a
    };
    abi::set_fpr(c, dst, angle);
    Ok(angle)
}

/// `angle * to_binary` as a 16-bit binary angle (`cvt.w.s`; f0 the float,
/// f1 its bits, v0 the value).
fn binary_angle(c: &mut Cpu, angle: FpuF32, k: i32) -> u64 {
    let scale = gp_const(c, k);
    let scaled = angle * scale;
    abi::set_fpr(c, 0, scaled);
    let bits = cvt_w(scaled);
    c.f[1] = bits;
    set_reg(c, V0, sx(bits));
    sx(bits)
}

/// `float SSXAudioSystem_Compute3DPan(SSXAudioSystem* this, Emitter* e,
/// Pan3D* out)` (0x285930): the emitter's position relative to its
/// player's listener and the squared distance.
///
/// ```text
/// if (!app->gameModule) return 1.0
/// p   = listener.matrix (+0x40) * (e->pos, 1)        (VU0 MULA/MADDA/MADD)
/// r   = ((p.x, p.y + 25, p.z - 100) * 0.01)          (stored at sp+0..8)
/// if this->panEnabled (+0x62b0):
///     out->azimuth (+8)    = e->fixedSide (+0xa0) ? (e->right (+0xa4) ? 0x4000 : 0xc000)
///                                                 : angle(r.x, r.z) * 65536/2pi
///     out->elevation (+0xa) = angle(r.y, sqrt(r.x² + r.z²)) * 65536/2pi
/// else out->azimuth = out->elevation = 0
/// return r.x² + r.y² + r.z²
/// ```
///
/// Frame 0xa0 (s0 +0x70, s1 +0x60, s2 +0x50, ra +0x40, f22 +0x90, f21
/// +0x88, f20 +0x80); the transformed point also passes through v0 (all
/// 128 bits, `lq`/`sq` at sp+0x30 / sp+0x10).
pub fn ssx_audio_system_compute_3d_pan(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        let emitter = reg(c, A1);
        set_reg(c, S0, emitter);
        frame.save(c, S2, 0x50);
        let out = reg(c, A2);
        set_reg(c, S1, out);
        frame.save_ra(c, 0x40);
        let this = reg(c, A0);
        set_reg(c, S2, this);
        frame.save_fpr(c, 22, 0x90);
        frame.save_fpr(c, 21, 0x88);
        frame.save_fpr(c, 20, 0x80);
        GET_APP.call(c, 0x0028_5960)?;
        let app = reg(c, V0);
        let module = at(app, 0x84).lw(c);
        set_reg(c, V1, module);
        if module == 0 {
            c.cycles += 4;
            set_reg(c, AT, FpuF32::ONE.0 as u64);
            abi::set_fret(c, FpuF32::ONE);
            frame.restore(c, S0, 0x70);
            return pan_exit(c, frame);
        }
        c.cycles += 48;
        GET_GAME_MODULE.call(c, 0x0028_5984)?;
        let module = reg(c, V0);
        let player = at(emitter, 0x84).lw(c);
        let players = at(module, 0x84).lw(c);
        let pos_at = at(emitter, 0x68).lw(c);
        let slot = addu(players, sll(player, 2));
        set_reg(c, A0, slot);
        set_reg(c, AT, FpuF32::ONE.0 as u64);
        let z = at(pos_at, 8).read_fpu(c);
        let x = at(pos_at, 0).read_fpu(c);
        let y = at(pos_at, 4).read_fpu(c);
        let listener = at(slot, 4).lw(c);
        let sp0 = Addr(sp(c));
        sp0.off(0x20).write_fpu(c, x);
        sp0.off(0x24).write_fpu(c, y);
        sp0.off(0x28).write_fpu(c, z);
        sp0.off(0x2c).write_fpu(c, FpuF32::ONE);
        let matrix = addiu(listener, 0x40);
        set_reg(c, V1, matrix);
        // p = M * (x, y, z, 1) on VU0, as the original issues it (its
        // registers and flags are visible to later code).
        c.lqc2(8, sp0.off(0x20).0);
        for i in 0..4u32 {
            c.lqc2(4 + i as usize, (matrix as u32).wrapping_add(0x10 * i));
        }
        c.vu0().mula(0b1111, 4, Src::Bc(8, X));
        c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
        c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
        c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
        c.sqc2(12, sp0.off(0x30).0);
        c.lq(V0, sp0.off(0x30).0);
        set_reg(c, AT, PAN_UP.0 as u64);
        c.sq(V0, sp0.off(0x10).0);
        // The listener-relative position in audio units.
        let scale = gp_const(c, pan_k::SCALE);
        let ly = sp0.off(0x14).read_fpu(c);
        let lz = sp0.off(0x18).read_fpu(c);
        let lx = sp0.off(0x10).read_fpu(c);
        let raised = ly + PAN_UP;
        let pulled = lz + PAN_BACK;
        let enabled = at(this, 0x62b0).lw(c);
        set_reg(c, V0, enabled);
        let rx = lx * scale;
        let ry = raised * scale;
        let rz = pulled * scale;
        abi::set_fpr(c, 0, ry);
        abi::set_fpr(c, 1, pulled);
        abi::set_fpr(c, 2, scale);
        abi::set_fpr(c, 3, lx);
        abi::set_fpr(c, 4, PAN_UP);
        abi::set_fpr(c, 5, PAN_BACK);
        abi::set_fpr(c, 20, rz);
        abi::set_fpr(c, 21, rx);
        sp0.write_fpu(c, rx);
        sp0.off(4).write_fpu(c, ry);
        sp0.off(8).write_fpu(c, rz);
        if enabled == 0 {
            c.cycles += 3;
            at(out, 0xa).sh(c, 0);
            at(out, 8).sh(c, 0);
        } else {
            // Azimuth.
            c.cycles += 3;
            let fixed = at(emitter, 0xa0).lw(c);
            set_reg(c, V0, fixed);
            let azimuth = if fixed != 0 {
                c.cycles += 3;
                let right = at(emitter, 0xa4).lw(c);
                if right != 0 {
                    0x4000
                } else {
                    c.cycles += 2;
                    0xc000
                }
            } else {
                c.cycles += 4;
                abi::set_fpr(c, 22, FpuF32::ZERO);
                let k = [pan_k::AZ_LEFT, pan_k::AZ_RIGHT, pan_k::AZ_PI_ADD, pan_k::AZ_PI_SUB];
                let angle = pan_angle(c, rx, rz, 3, k, 0x0028_5ab0)?;
                c.cycles += 4;
                binary_angle(c, angle, pan_k::AZ_TO_ANGLE)
            };
            set_reg(c, V0, azimuth);
            // Elevation.
            c.cycles += 12;
            at(out, 8).sh(c, azimuth);
            let dx = sp0.read_fpu(c);
            let dz = sp0.off(8).read_fpu(c);
            let xx = dx * dx;
            let dy = sp0.off(4).read_fpu(c);
            abi::set_fpr(c, 21, dy);
            let zz = dz * dz;
            abi::set_fpr(c, 1, zz);
            abi::set_fpr(c, 22, FpuF32::ZERO);
            let horiz2 = xx + zz;
            abi::set_fpr(c, 0, horiz2);
            let horiz = horiz2.sqrt();
            abi::set_fpr(c, 20, horiz);
            let k = [pan_k::EL_DOWN, pan_k::EL_UP, pan_k::EL_PI_ADD, pan_k::EL_PI_SUB];
            let angle = pan_angle(c, dy, horiz, 12, k, 0x0028_5b58)?;
            c.cycles += 5;
            let elevation = binary_angle(c, angle, pan_k::EL_TO_ANGLE);
            at(out, 0xa).sh(c, elevation);
        }
        // Squared distance.
        c.cycles += 9;
        let dx = sp0.read_fpu(c);
        let dy = sp0.off(4).read_fpu(c);
        let xx = dx * dx;
        let dz = sp0.off(8).read_fpu(c);
        let yy = dy * dy;
        let zz = dz * dz;
        abi::set_fpr(c, 2, yy);
        abi::set_fpr(c, 1, zz);
        abi::set_fret(c, (xx + yy) + zz);
        frame.restore(c, S0, 0x70);
        pan_exit(c, frame)
    })
}

/// The epilogue (s0 already restored).
fn pan_exit(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 8;
    frame.restore(c, S1, 0x60);
    frame.restore(c, S2, 0x50);
    frame.restore_ra(c, 0x40);
    frame.restore_fpr(c, 22, 0x90);
    frame.restore_fpr(c, 21, 0x88);
    frame.restore_fpr(c, 20, 0x80);
    frame.close(c);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, elf_image, fill, scramble, Rng, DATA, GAME_GP};

    /// `SND_EeVoiceSetPitch`/`SetVolume`'s voice array pointer (`0x515b40 +
    /// 0x1dc`).
    const EE_VOICES_PTR: u32 = 0x0051_5d1c;

    /// A channel with `flags` and the driver globals the channel functions
    /// read (voice numbers 6..23, so every voice index stays positive).
    fn setup_channel(c: &mut Cpu, r: &mut Rng, seed: u32, flags: u16) {
        let index = r.next_u32() % 3;
        c.g[A0][0] = index as u64;
        c.bus.write32(STATE + state::CHANNELS, DATA);
        let ch = DATA + index * chan::SIZE;
        c.bus.write16(ch + chan::FLAGS, flags);
        c.bus.write16(ch + chan::PITCH, r.next_u32() as u16);
        c.bus.write16(ch + chan::PITCH_SCALE, r.next_u32() as u16);
        c.bus.write8(ch + chan::VOICE_COUNT, ((seed / 12) % 5) as u8);
        for i in 0..4 {
            c.bus.write16(ch + chan::VOICES + 2 * i, (6 + r.next_u32() % 18) as u16);
        }
        let volume = if seed % 9 == 0 { r.edge_f32() } else { r.f32(2.0) };
        c.bus.write32(ch + chan::VOLUME, volume);
        c.bus.write8(ch + chan::PAN, r.next_u32() as u8);
        let mode = if seed % 2 == 0 { 2 } else { r.next_u32() as u8 | 1 };
        c.bus.write8(STATE + state::OUTPUT_MODE, mode);
        c.bus.write8(STATE + state::SPEAKERS, if seed % 4 < 2 { 4 } else { 2 });
        c.bus.write8(STATE + state::AUX_SENDS, (r.next_u32() % 3) as u8);
        let rate = |r: &mut Rng| if seed % 7 == 0 { 0 } else { r.next_u32() as u16 };
        let (iop, ee_rate) = (rate(r), rate(r));
        c.bus.write16(STATE + state::IOP_RATE, iop);
        c.bus.write16(STATE + state::EE_RATE, ee_rate);
        c.bus.write8(STATE + state::VOICE_BASE, (r.next_u32() % 4) as u8);
        c.bus.write8(STATE + state::EE_VOICE_BASE, (r.next_u32() % 3) as u8);
        c.bus.write32(SYNC + sync::VOICE_PARAMS, DATA + 0x1_0000);
        for i in 0..32 * 8 {
            c.bus.write32(SYNC + sync::GAINS + 4 * i, r.f32(1.0));
        }
        // EE voices with a resampler (SND_EeVoiceSetPitch takes the
        // SetRate path).
        c.bus.write32(EE_VOICES_PTR, DATA + 0x2_0000);
        for v in 0..32 {
            c.bus.write32(DATA + 0x2_0000 + v * 0x60 + 0x50, DATA + 0x3_0000 + v * 0x40);
        }
    }

    /// Flags selecting the SPU / IOP / EE path by `seed % 3`, other bits
    /// random.
    fn channel_flags(r: &mut Rng, seed: u32) -> u16 {
        let other = r.next_u32() as u16 & !0x108;
        match seed % 3 {
            0 => other | 0x8 | (r.next_u32() as u16 & 0x100),
            1 => other | 0x100,
            _ => other,
        }
    }

    #[test]
    fn snd_channel_apply_pitch_matches() {
        check(ee::s13::f_003c25e0__generated, snd_channel_apply_pitch, 240, &|c, seed| {
            let mut r = Rng(0x7a11_0001 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            let flags = channel_flags(&mut r, seed);
            setup_channel(c, &mut r, seed, flags);
        });
    }

    #[test]
    fn snd_channel_apply_volume_matches() {
        check(ee::s13::f_003c2a50__generated, snd_channel_apply_volume, 240, &|c, seed| {
            let mut r = Rng(0x7a11_0002 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            let flags = channel_flags(&mut r, seed);
            setup_channel(c, &mut r, seed, flags);
        });
    }

    #[test]
    fn snd_utk_lpc_filter_matches() {
        check(ee::s14::f_003ce410__generated, snd_utk_lpc_filter, 200, &|c, seed| {
            let mut r = Rng(0x7a11_0003 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            let blocks = (seed % 4) as i32 - (seed % 9 == 0) as i32;
            c.g[A0][0] = sx(blocks as u32);
            c.g[A1][0] = sx(DATA);
            c.g[A2][0] = sx(DATA + 0x100);
            c.g[A3][0] = sx(DATA + 0x200);
            let edge = seed % 5 == 0;
            for i in 0..12 {
                let v = if edge { r.edge_f32() } else { r.f32(0.5) };
                c.bus.write32(DATA + 4 * i, v);
                let h = if edge { r.edge_f32() } else { r.f32(1000.0) };
                c.bus.write32(DATA + 0x100 + 4 * i, h);
            }
            for i in 0..48 {
                let x = if edge { r.edge_f32() } else { r.f32(1000.0) };
                c.bus.write32(DATA + 0x200 + 4 * i, x);
            }
        });
    }

    #[test]
    fn snd_utk_decode_excitation_matches() {
        check(ee::s14::f_003cd2b0__generated, snd_utk_decode_excitation, 200, &|c, seed| {
            let mut r = Rng(0x7a11_0004 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            elf_image(c);
            c.pc = exc::START;
            let reader = DATA;
            let input = DATA + 0x100;
            fill(c, &mut r, input, 0x100);
            c.bus.write32(reader, input);
            let buf = r.next_u32();
            c.bus.write32(reader + 4, buf);
            c.bus.write32(reader + 8, 8 + r.next_u32() % 24);
            c.g[A0][0] = sx(reader);
            c.g[A1][0] = if seed % 3 == 0 { 0 } else { r.next_u32() as u64 | 1 };
            c.g[A2][0] = sx(DATA + 0x1000);
            c.g[A3][0] = 1 + (seed % 3) as u64;
        });
    }

    #[test]
    fn snd_eaxa_decode_blocks_matches() {
        check(ee::s14::f_003cca08__generated, snd_eaxa_decode_blocks, 240, &|c, seed| {
            let mut r = Rng(0x7a11_0005 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            for i in 0..20 {
                c.bus.write32(EAXA_COEFS + 4 * i, r.f32(2.0));
            }
            let st = DATA;
            c.g[A0][0] = sx(st);
            let blocks = (seed % 4) as i32;
            let left = blocks * EAXA_BLOCK_SAMPLES - 3 * (seed % 5 == 0) as i32;
            c.bus.write32(st + eaxa::LEFT, left as u32);
            let (h1, h2) = if seed % 7 == 0 { (r.edge_f32(), r.edge_f32()) } else { (r.f32(30000.0), r.f32(30000.0)) };
            c.bus.write32(st + eaxa::HIST1, h1);
            c.bus.write32(st + eaxa::HIST2, h2);
            let input = DATA + 0x100;
            fill(c, &mut r, input, 0x100);
            c.bus.write32(st + eaxa::INPUT, input);
            c.bus.write32(st + eaxa::OUTPUT, DATA + 0x1000);
            let mut p = input;
            for b in 0..4 {
                if (seed / 4 + b) % 3 == 0 {
                    c.bus.write8(p, 0xee);
                    p += 61;
                } else {
                    let h = r.next_u32() as u8;
                    c.bus.write8(p, if h == 0xee { 0x12 } else { h });
                    p += 15;
                }
            }
        });
    }

    #[test]
    fn ssx_audio_system_compute_3d_pan_matches() {
        check(ee::s07::f_00285930__generated, ssx_audio_system_compute_3d_pan, 240, &|c, seed| {
            let mut r = Rng(0x7a11_0006 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            elf_image(c);
            c.g[GP][0] = GAME_GP as u64;
            let app = DATA;
            let module = if seed % 11 == 0 { 0 } else { DATA + 0x100 };
            let players = DATA + 0x200;
            let emitter = DATA + 0x300;
            let pos = DATA + 0x400;
            let listener = DATA + 0x500;
            let this = DATA + 0x1000;
            let out = DATA + 0x8000;
            c.bus.write32(GAME_GP - 0x848, app);
            c.bus.write32(app + 0x84, module);
            c.bus.write32(DATA + 0x100 + 0x84, players);
            let player = r.next_u32() % 4;
            c.bus.write32(emitter + 0x84, player);
            c.bus.write32(emitter + 0x68, pos);
            c.bus.write32(emitter + 0xa0, if seed % 5 == 0 { r.next_u32() | 1 } else { 0 });
            c.bus.write32(emitter + 0xa4, r.next_u32() & 1);
            c.bus.write32(players + 4 * player, listener);
            let identity = seed % 2 == 0;
            for row in 0..4u32 {
                for col in 0..4u32 {
                    let v = if identity {
                        if row == col { 0x3f80_0000 } else { 0 }
                    } else {
                        r.f32(2.0)
                    };
                    c.bus.write32(listener + 0x40 + 16 * row + 4 * col, v);
                }
            }
            let mut x = r.f32(500.0);
            let mut y = r.f32(500.0);
            let mut z = r.f32(500.0);
            if seed % 4 == 0 {
                z = 100.0f32.to_bits();
            }
            if seed % 8 == 0 {
                x = if seed % 16 == 0 { 0 } else { 0x8000_0000 };
            }
            if seed % 16 == 0 || seed % 6 == 2 {
                y = (-25.0f32).to_bits();
            }
            c.bus.write32(pos, x);
            c.bus.write32(pos + 4, y);
            c.bus.write32(pos + 8, z);
            c.bus.write32(this + 0x62b0, (seed % 6 != 0) as u32);
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(emitter);
            c.g[A2][0] = sx(out);
        });
    }
}
