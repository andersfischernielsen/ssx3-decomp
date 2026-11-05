//! Staging wave 203 (oc221): hand-written replacements for the hottest
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

use crate::abi::set_fpr;
use crate::guest::Addr;
use crate::ps2f::FpuF32;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003c_38e0, snd_channel_apply_pan, "SND_ChannelApplyPan"),
    (0x0037_dd20, ps2_graphics_man_v92, "PS2GraphicsMan_v92"),
    (
        0x0029_5208,
        ssx_audio_system_update_tracked_sounds,
        "SSXAudioSystem_UpdateTrackedSounds",
    ),
    (
        0x002f_2088,
        attached_emitter_fx_render,
        "AttachedEmitterFX_Render",
    ),
];

// ---------------------------------------------------------------------------
// SND_ChannelApplyPan (0x3c38e0)

/// Sound driver globals `SND_ChannelApplyPan` reads.
mod pan {
    /// The driver state block (`snd::STATE`).
    pub const STATE: u32 = 0x0050_a8e8;
    /// `u8`: speaker count.
    pub const SPEAKERS: u32 = 0x47;
    /// `u8`: auxiliary sends.
    pub const AUX_SENDS: u32 = 0x48;
    /// `SndChannel*`: the channel array.
    pub const CHANNELS: u32 = 0x1f0;
    /// The second global block (`snd::SYNC`).
    pub const SYNC: u32 = 0x0050_ad00;
    /// `float[voice][8]`: per-voice output gains (`SYNC + 0xe98`).
    pub const GAINS: u32 = 0x0050_bb98;
    /// `{ s16 angle; s16 level; }[layout][6]`: the speaker layouts (0x18
    /// bytes each): per voice slot, a pan angle offset and a level.
    pub const LAYOUTS: u32 = 0x0050_ab20;
    /// The `level` halves of `LAYOUTS`.
    pub const LAYOUT_LEVELS: u32 = 0x0050_ab22;
    /// Bytes per layout.
    pub const LAYOUT_SIZE: u64 = 0x18;
    /// `SndChannel` size.
    pub const CHAN_SIZE: u64 = 0x8c;
    /// `s16[]`: the channel's voice numbers.
    pub const VOICES: u32 = 0x04;
    /// `u16`: the channel's pan angle.
    pub const ANGLE: u32 = 0x1c;
    /// `u8`: number of voices.
    pub const VOICE_COUNT: u32 = 0x23;
    /// `u16`: flags.
    pub const FLAGS: u32 = 0x24;
    /// `u16[]`: per-voice pan angles.
    pub const VOICE_ANGLES: u32 = 0x50;
    /// Flag: SPU voices (a centre-rear split in 5-speaker mode).
    pub const F_SPU: u64 = 0x8;
    /// Flag: pan by the smooth lookup table (instead of the gain tables).
    pub const F_SMOOTH: u64 = 0x40;
    /// `1/sqrt(2)`: the mono gain of both sides.
    pub const HALF_POWER: u32 = 0x3f35_04f4;
    /// `32767.0`.
    pub const FULL_SCALE: u32 = 0x46ff_fe00;
    /// `1.0`.
    pub const ONE: u32 = 0x3f80_0000;
}

const SND_CHANNEL_APPLY_VOLUME: Callee = Callee {
    addr: 0x003c_2a50,
    f: ee::s13::SND_ChannelApplyVolume,
};
const SND_PAN_LOOKUP: Callee = Callee {
    addr: 0x003c_3728,
    f: ee::s13::SND_PanLookup,
};
const SND_PAN_FIX_QUADRANT: Callee = Callee {
    addr: 0x003c_3780,
    f: ee::s13::SND_PanFixQuadrant,
};
const SND_GET_PAN_GAINS: Callee = Callee {
    addr: 0x003b_c4c0,
    f: ee::s13::SND_GetPanGains,
};

/// `addu`-style 32-bit add of a register and a value, sign-extended.
#[inline(always)]
fn add(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}

/// `sll r, 5` / `sll r, 2` / ...: 32-bit shift, sign-extended.
#[inline(always)]
fn shl(a: u64, n: u32) -> u64 {
    sx((a as u32) << n)
}

/// `void SND_ChannelApplyPan(int channel)` (0x3c38e0): spread a channel's
/// voices over the speakers. One speaker: every voice gets `1/sqrt 2` on
/// both sides. One voice: the channel's pan angle through the smooth lookup
/// (flag 0x40; result also kept as 16-bit levels) or the gain tables, with a
/// centre-rear split on 5 speakers (SPU flag). Several voices: pick a
/// speaker layout from the speaker/voice counts, pan each voice by its own
/// angle plus the layout's offset, scale by the layout's level, flip
/// out-of-range smooth pans, and route a 6th voice (LFE) to the aux send.
/// Then `SND_ChannelApplyVolume(channel)`.
pub fn snd_channel_apply_pan(c: &mut Cpu) {
    use pan::*;
    guest(c, |c| {
        // Prologue: the channel, its gain block and the speaker count.
        c.cycles += 28;
        let frame = Frame::open(c, 0xd0);
        let index_at = frame.at(c, 0x20);
        c.sw(index_at, c.g[A0][0]);
        c.g[A0][0] = CHAN_SIZE;
        frame.save(c, S0, 0xc0);
        c.g[V0][0] = c.lw(index_at);
        frame.save(c, S2, 0xa0);
        c.mult(0, A0, V0, A0);
        frame.save(c, S1, 0xb0);
        frame.save(c, S3, 0x90);
        frame.save(c, S4, 0x80);
        frame.save(c, S5, 0x70);
        frame.save(c, S6, 0x60);
        frame.save(c, S7, 0x50);
        frame.save(c, FP, 0x40);
        frame.save_ra(c, 0x30);
        c.g[A3][0] = sx(STATE);
        c.g[A2][0] = sx(GAINS);
        c.g[V0][0] = c.lw(index_at);
        c.g[A1][0] = c.lw(STATE + CHANNELS);
        c.g[V1][0] = shl(c.g[V0][0], 5);
        c.g[T0][0] = c.lbu(STATE + SPEAKERS);
        let ch = (c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32);
        c.g[S2][0] = sx(ch);
        c.g[S0][0] = add(c.g[V1][0], GAINS as u64);
        c.g[A0][0] = 1;
        c.g[A1][0] = 0;
        let speakers = c.g[T0][0];

        if speakers == 1 {
            // Mono: both sides of every voice at half power.
            c.cycles += 3;
            c.g[V0][0] = c.lbu(ch + VOICE_COUNT);
            c.g[S0][0] = 0;
            if c.g[V0][0] != 0 {
                c.cycles += 7;
                c.g[A1][0] = sx(GAINS);
                c.g[AT][0] = HALF_POWER as u64;
                c.f[0] = HALF_POWER;
                c.g[A2][0] = sx(GAINS + 4);
                c.g[A0][0] = sx(ch + VOICES);
                loop {
                    c.cycles += 11;
                    let voice = c.lh(r32(c, A0));
                    c.g[S0][0] = add(c.g[S0][0], 1);
                    c.g[V0][0] = shl(voice, 5);
                    c.g[V1][0] = add(c.g[V0][0], (GAINS + 4) as u64);
                    c.swc1(0, r32(c, V1));
                    c.g[V0][0] = add(c.g[V0][0], GAINS as u64);
                    c.swc1(0, r32(c, V0));
                    let count = c.lbu(ch + VOICE_COUNT);
                    c.g[V1][0] = ((c.g[S0][0] as i64) < (count as i64)) as u64;
                    c.g[A0][0] = add(c.g[A0][0], 2);
                    if c.g[V1][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_3978);
                    }
                }
            }
            // A sixth voice (LFE) gets nothing on the front pair.
            c.cycles += 4;
            c.g[V1][0] = c.lbu(ch + VOICE_COUNT);
            c.g[V0][0] = 6;
            c.g[A0][0] = 0x51_0000;
            if c.g[V1][0] == 6 {
                c.cycles += 9;
                let lfe = shl(c.lh(ch + VOICES + 10), 5);
                c.g[A0][0] = sx(SYNC);
                c.g[V0][0] = add(lfe, SYNC as u64);
                c.sw((c.g[V0][0] as u32).wrapping_add(0xe98), 0);
                c.g[V1][0] = shl(c.lh(ch + VOICES + 10), 5);
                c.g[A0][0] = add(c.g[A0][0], c.g[V1][0]);
                c.sw((c.g[A0][0] as u32).wrapping_add(0xe9c), 0);
            }
            c.cycles += 4;
            c.g[A0][0] = c.lw(index_at_now(c, frame));
            SND_CHANNEL_APPLY_VOLUME.call(c, 0x003c_39e0)?;
            frame.restore(c, S0, 0xc0);
            pan_epilogue(c, frame);
            return Ok(());
        }

        c.cycles += 4;
        let count = c.lbu(ch + VOICE_COUNT);
        c.g[V0][0] = count;
        c.g[V1][0] = count & 0xff;
        c.g[A2][0] = count;
        if count == 1 {
            pan_one_voice(c, frame, ch)?;
        } else {
            pan_voices(c, frame, ch, speakers)?;
        }
        c.cycles += 3;
        c.g[A0][0] = c.lw(index_at_now(c, frame));
        SND_CHANNEL_APPLY_VOLUME.call(c, 0x003c_3e74)?;
        frame.restore(c, S0, 0xc0);
        pan_epilogue(c, frame);
        Ok(())
    })
}

/// `sp + 0x20`, where the channel index was saved.
#[inline(always)]
fn index_at_now(c: &Cpu, frame: Frame) -> u32 {
    frame.at(c, 0x20)
}

/// Restore s1-s7, fp and ra and return.
fn pan_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 11;
    frame.restore(c, S1, 0xb0);
    frame.restore(c, S2, 0xa0);
    frame.restore(c, S3, 0x90);
    frame.restore(c, S4, 0x80);
    frame.restore(c, S5, 0x70);
    frame.restore(c, S6, 0x60);
    frame.restore(c, S7, 0x50);
    frame.restore(c, FP, 0x40);
    frame.restore_ra(c, 0x30);
    frame.close(c);
}

/// One voice: the channel's angle into its gain block (s0).
fn pan_one_voice(c: &mut Cpu, frame: Frame, ch: u32) -> crate::call::Flow {
    use pan::*;
    let gains = c.g[S0][0] as u32;
    c.cycles += 3;
    c.g[V0][0] = c.lbu(STATE + AUX_SENDS);
    if c.g[V0][0] != 0 {
        // Nothing on the first aux send.
        c.cycles += 5;
        c.g[V0][0] = shl(c.lbu(STATE + SPEAKERS), 2);
        c.g[V0][0] = add(c.g[S0][0], c.g[V0][0]);
        c.sw(r32(c, V0), 0);
    }
    c.g[V0][0] = c.lhu(ch + FLAGS);
    c.cycles += 3;
    c.g[V0][0] &= F_SMOOTH;
    c.g[A0][0] = c.lhu(ch + ANGLE);
    if c.g[V0][0] != 0 {
        c.cycles += 22;
        c.g[A1][0] = sx(gains);
        SND_PAN_LOOKUP.call(c, 0x003c_3a2c)?;
        c.g[A0][0] = c.lhu(ch + ANGLE);
        c.g[A1][0] = sx(gains + 4);
        c.g[A3][0] = c.lh(gains + 0x1c);
        c.g[A2][0] = sx(gains);
        c.g[T0][0] = c.lh(gains + 0x1e);
        SND_PAN_FIX_QUADRANT.call(c, 0x003c_3a44)?;
        // Keep the pair as 16-bit levels too.
        c.g[AT][0] = FULL_SCALE as u64;
        let scale = FpuF32(FULL_SCALE);
        let right = Addr(gains + 4).read_fpu(c) * scale;
        let left = Addr(gains).read_fpu(c) * scale;
        set_fpr(c, 1, left);
        let right_w = right.to_i32() as u32;
        let left_w = left.to_i32() as u32;
        c.f[2] = right_w;
        c.f[0] = left_w;
        c.g[V0][0] = sx(right_w);
        c.g[V1][0] = sx(left_w);
        c.sh(gains + 0x1c, c.g[V0][0]);
        c.sh(gains + 0x1e, c.g[V1][0]);
    } else {
        c.cycles += 2;
        c.g[A1][0] = sx(gains);
        SND_GET_PAN_GAINS.call(c, 0x003c_3a84)?;
    }
    c.cycles += 6;
    c.g[A0][0] = 5;
    c.g[V0][0] = sx(STATE);
    c.g[V1][0] = c.lbu(STATE + SPEAKERS);
    if c.g[V1][0] == 5 {
        c.cycles += 4;
        c.g[V0][0] = c.lhu(ch + FLAGS) & F_SPU;
        c.g[A1][0] = c.g[SP][0];
        if c.g[V0][0] != 0 {
            // SPU voices on 5 speakers: the opposite angle's front/rear
            // gains go to the centre and rear slots.
            c.cycles += 9;
            let angle = sx((c.lhu(ch + ANGLE) as u32).wrapping_sub(0x8000));
            c.g[A0][0] = angle & 0xffff;
            SND_GET_PAN_GAINS.call(c, 0x003c_3abc)?;
            let tmp = frame.at(c, 0);
            c.lwc1(0, tmp + 8);
            c.lwc1(1, tmp + 0xc);
            c.swc1(0, gains + 0x10);
            c.swc1(1, gains + 4);
        }
    }
    Ok(())
}

/// Where a layout decision continues (the original's join points).
enum LayoutJoin {
    /// `a1 = 0` if v0 is non-zero, then reload the voice count.
    ZeroIfV0,
    /// Reload the voice count.
    Reload,
    /// v0 already holds the voice count.
    Loaded,
}

/// Several voices: choose a speaker layout (a1), then pan every voice.
fn pan_voices(c: &mut Cpu, frame: Frame, ch: u32, speakers: u64) -> crate::call::Flow {
    use pan::*;
    // Layout by speaker/voice counts (registers: a0 = 1, a1 = 0,
    // v1 = a2 = voice count).
    let count = c.g[A2][0];
    c.cycles += 3;
    c.g[V0][0] = 5;
    let mut join = None;
    if speakers == 5 {
        c.cycles += 3;
        c.g[V0][0] = c.lbu(STATE + AUX_SENDS);
        if c.g[V0][0] == 0 {
            // 5 speakers without aux sends: layout 1 for 6 voices.
            c.cycles += 3;
            c.g[V0][0] = c.g[V1][0] ^ 6;
            c.g[A1][0] = c.g[A0][0];
            join = Some(LayoutJoin::ZeroIfV0);
        }
    }
    let join = match join {
        Some(j) => j,
        None => {
            c.g[A0][0] = 0x51_0000;
            c.cycles += 5;
            c.g[V1][0] = 4;
            c.g[V0][0] = sx(STATE);
            c.g[A0][0] = c.lbu(STATE + SPEAKERS);
            c.g[V0][0] = 2;
            if c.g[A0][0] == 4 {
                // Quad: layout 2 for 5-6 voices.
                c.cycles += 7;
                c.g[V0][0] = sx((count as u32).wrapping_sub(5)) & 0xff;
                c.g[V1][0] = (1 < c.g[V0][0]) as u64;
                c.g[A1][0] = if c.g[V1][0] != 0 { 0 } else { 2 };
                LayoutJoin::Reload
            } else {
                c.cycles += 2;
                if c.g[A0][0] != 2 {
                    c.g[V0][0] = c.lbu(ch + VOICE_COUNT);
                    LayoutJoin::Loaded
                } else {
                    // Stereo.
                    c.cycles += 4;
                    let smooth = c.lhu(ch + FLAGS) & F_SMOOTH != 0;
                    c.g[V0][0] = count & 0xff;
                    if smooth {
                        // Layout 1 for 6 voices.
                        c.cycles += 3;
                        c.g[V0][0] ^= 6;
                        c.g[A1][0] = (c.g[V0][0] < 1) as u64;
                        LayoutJoin::Reload
                    } else {
                        // Layout 3 for 5-6 voices, 5 for 4.
                        c.cycles += 5;
                        c.g[V0][0] = sx((count as u32).wrapping_sub(5)) & 0xff;
                        c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                        c.g[A1][0] = 3;
                        if c.g[V0][0] != 0 {
                            LayoutJoin::Reload
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = (count & 0xff) ^ 4;
                            c.g[A1][0] = 5;
                            LayoutJoin::ZeroIfV0
                        }
                    }
                }
            }
        }
    };
    if let LayoutJoin::ZeroIfV0 = join {
        c.cycles += 1;
        if c.g[V0][0] != 0 {
            c.g[A1][0] = 0;
        }
    }
    if !matches!(join, LayoutJoin::Loaded) {
        c.cycles += 1;
        c.g[V0][0] = c.lbu(ch + VOICE_COUNT);
    }
    c.cycles += 2;
    c.g[S0][0] = 0;
    if c.g[V0][0] == 0 {
        return Ok(());
    }

    // Per voice: s0 index, s7 the layout's byte offset, fp the voice angles.
    c.cycles += 8;
    c.g[V0][0] = LAYOUT_SIZE;
    c.g[A2][0] = sx(ch + VOICES);
    c.mult(0, S7, A1, V0);
    let voices_at = frame.at(c, 0x24);
    c.sw(voices_at, c.g[A2][0]);
    c.g[FP][0] = sx(ch + VOICE_ANGLES);
    let layout_at = frame.at(c, 0x28);
    c.sw(layout_at, c.g[S7][0]);
    c.g[V0][0] = 5;
    loop {
        pan_voice(c, frame, ch)?;
        // Next voice.
        c.cycles += 5;
        let count = c.lbu(ch + VOICE_COUNT);
        c.g[S0][0] = c.g[S1][0];
        let more = (c.g[S0][0] as i64) < (count as i64);
        c.g[V0][0] = 5;
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, 0x003c_3b98);
        }
    }
}

/// Where a voice's panning continues before the level scaling.
enum VoiceJoin {
    /// `a2 = 0x510000`, then as `State`.
    Hi,
    /// `a2 = STATE`.
    State,
    /// a2 already holds `STATE`.
    Done,
}

/// Pan voice s0 of a multi-voice channel.
fn pan_voice(c: &mut Cpu, frame: Frame, ch: u32) -> crate::call::Flow {
    use pan::*;
    let i = c.g[S0][0];
    let join = 'pan: {
        c.cycles += 2;
        if i == 5 {
            // The sixth voice (LFE): all to the first aux send if any.
            c.cycles += 5;
            c.g[A0][0] = 0x51_0000;
            c.g[V1][0] = sx(STATE);
            c.g[V0][0] = c.lbu(STATE + AUX_SENDS);
            c.g[A1][0] = 0;
            if c.g[V0][0] != 0 {
                c.cycles += 5;
                c.g[V0][0] = c.lbu(STATE + SPEAKERS);
                c.g[S3][0] = shl(i, 1);
                c.g[S5][0] = sx(ch + VOICES);
                c.g[S1][0] = add(i, 1);
                let voice_at = add(c.g[S5][0], c.g[S3][0]) as u32;
                if c.g[V0][0] != 0 {
                    c.cycles += 4;
                    c.g[A3][0] = c.g[V0][0];
                    c.g[A2][0] = sx(voice_at);
                    c.g[V0][0] = 0x51_0000;
                    c.g[T0][0] = sx(GAINS);
                    loop {
                        c.cycles += 9;
                        let voice = c.lh(voice_at);
                        c.g[V1][0] = shl(c.g[A1][0], 2);
                        c.g[A1][0] = add(c.g[A1][0], 1);
                        c.g[V0][0] = shl(voice, 5);
                        c.g[A0][0] = ((c.g[A1][0] as i64) < (c.g[A3][0] as i64)) as u64;
                        c.g[V1][0] = add(add(c.g[V1][0], c.g[V0][0]), GAINS as u64);
                        c.sw(r32(c, V1), 0);
                        if c.g[A0][0] == 0 {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003c_3bd8);
                        }
                    }
                }
                c.cycles += 12;
                c.g[A0][0] = sx(voice_at);
                c.g[V1][0] = shl(c.g[A1][0], 2);
                let voice = c.lh(voice_at);
                c.g[AT][0] = ONE as u64;
                c.f[0] = ONE;
                c.g[V0][0] = shl(voice, 5);
                c.g[A0][0] = sx(GAINS);
                c.g[V1][0] = add(add(c.g[V1][0], c.g[V0][0]), GAINS as u64);
                c.swc1(0, r32(c, V1));
                break 'pan VoiceJoin::Hi;
            }
            c.cycles += 1;
        }
        let flags = c.lhu(ch + FLAGS);
        c.cycles += 3;
        c.g[V0][0] = shl(i, 2);
        let voices = c.lw(frame.at(c, 0x24));
        if flags & F_SMOOTH != 0 {
            // Smooth: angle + layout offset through the lookup table.
            c.cycles += 23;
            c.g[A0][0] = voices;
            c.g[V1][0] = shl(i, 1);
            c.g[V0][0] = add(c.g[V0][0], c.g[S7][0]);
            c.g[A1][0] = add(c.g[FP][0], c.g[V1][0]);
            c.g[A3][0] = add(voices, c.g[V1][0]);
            c.g[V0][0] = add(c.g[V0][0], LAYOUTS as u64);
            let angle = c.lhu(r32(c, A1));
            let offset = c.lhu(r32(c, V0));
            c.g[S3][0] = c.g[V1][0];
            let voice = c.lh(r32(c, A3));
            c.g[S5][0] = sx(ch + VOICES);
            c.g[S1][0] = add(i, 1);
            c.g[A2][0] = sx(GAINS);
            c.g[A0][0] = add(angle, offset) & 0xffff;
            c.g[A1][0] = add(shl(voice, 5), GAINS as u64);
            SND_PAN_LOOKUP.call(c, 0x003c_3c90)?;
            c.g[A2][0] = 0x51_0000;
            break 'pan VoiceJoin::State;
        }
        // Gain tables: angle + layout offset.
        c.cycles += 25;
        c.g[A0][0] = voices;
        c.g[A3][0] = shl(i, 1);
        c.g[V1][0] = add(voices, c.g[A3][0]);
        c.g[V0][0] = add(add(c.g[V0][0], c.g[S7][0]), LAYOUTS as u64);
        let voice = c.lh(r32(c, V1));
        c.g[S3][0] = c.g[A3][0];
        c.g[V1][0] = c.lh(r32(c, V0));
        c.g[A2][0] = add(c.g[FP][0], c.g[A3][0]);
        c.g[V0][0] = 0x51_0000;
        let angle = c.lhu(r32(c, A2));
        c.g[S6][0] = sx(GAINS);
        c.g[A0][0] = add(angle, c.g[V1][0]);
        c.g[A1][0] = add(shl(voice, 5), GAINS as u64);
        SND_GET_PAN_GAINS.call(c, 0x003c_3ce0)?;
        c.g[S5][0] = sx(ch + VOICES);
        let count = c.lbu(ch + VOICE_COUNT);
        c.g[V1][0] = 0x51_0000;
        c.g[S4][0] = sx(STATE);
        c.g[V0][0] = (count < 5) as u64;
        c.g[S1][0] = add(i, 1);
        if c.g[V0][0] != 0 {
            break 'pan VoiceJoin::Hi;
        }
        c.cycles += 3;
        c.g[V0][0] = 1;
        c.g[A2][0] = 0x51_0000;
        if i != 1 {
            break 'pan VoiceJoin::State;
        }
        c.cycles += 4;
        c.g[V0][0] = c.lbu(STATE + SPEAKERS);
        c.g[A0][0] = 5;
        c.g[A2][0] = sx(STATE);
        if c.g[V0][0] != 5 {
            break 'pan VoiceJoin::Done;
        }
        c.cycles += 4;
        c.g[V0][0] = c.lhu(ch + FLAGS) & F_SPU;
        c.g[A0][0] = 0x8000;
        if c.g[V0][0] == 0 {
            break 'pan VoiceJoin::Hi;
        }
        // SPU voices on 5 speakers: the second voice's centre and rear
        // slots get the opposite angle's gains.
        c.cycles += 12;
        c.g[A1][0] = c.g[SP][0];
        SND_GET_PAN_GAINS.call(c, 0x003c_3d30)?;
        let tmp = frame.at(c, 0);
        let second = c.lh(ch + VOICES + 2);
        c.lwc1(0, tmp + 8);
        c.g[V1][0] = add(c.g[S6][0], shl(second, 5));
        c.lwc1(1, tmp + 0xc);
        c.swc1(0, r32(c, V1).wrapping_add(0x10));
        let second = c.lh(ch + VOICES + 2);
        c.g[V0][0] = add(c.g[S6][0], shl(second, 5));
        c.swc1(1, r32(c, V0).wrapping_add(4));
        VoiceJoin::Hi
    };
    match join {
        VoiceJoin::Hi => {
            c.cycles += 2;
            c.g[A2][0] = sx(STATE);
        }
        VoiceJoin::State => {
            c.cycles += 1;
            c.g[A2][0] = sx(STATE);
        }
        VoiceJoin::Done => {}
    }

    // Scale the voice's outputs by the layout's level / 32767.
    c.cycles += 5;
    let outputs = add(c.lbu(STATE + SPEAKERS), c.lbu(STATE + AUX_SENDS));
    c.g[V1][0] = outputs;
    c.g[V0][0] = c.lbu(STATE + AUX_SENDS);
    c.g[A1][0] = 0;
    if outputs != 0 {
        c.cycles += 21;
        c.g[A0][0] = c.lw(frame.at(c, 0x28));
        let level_at = add(add(shl(i, 2), c.g[A0][0]), LAYOUT_LEVELS as u64);
        let level = c.lh(level_at as u32);
        c.g[V0][0] = level;
        c.g[A3][0] = add(c.g[S5][0], c.g[S3][0]);
        c.g[A0][0] = c.lbu(STATE + SPEAKERS);
        c.g[V1][0] = c.lbu(STATE + AUX_SENDS);
        let level = FpuF32::from_i32(level as i32);
        set_fpr(c, 1, level);
        c.g[AT][0] = FULL_SCALE as u64;
        let scale = FpuF32(FULL_SCALE);
        c.f[2] = FULL_SCALE;
        c.g[T0][0] = sx(GAINS);
        c.g[A2][0] = add(c.g[A0][0], c.g[V1][0]);
        let voice_at = r32(c, A3);
        loop {
            c.cycles += 12;
            let voice = c.lh(voice_at);
            c.g[V0][0] = shl(c.g[A1][0], 2);
            c.g[A1][0] = add(c.g[A1][0], 1);
            c.g[V1][0] = shl(voice, 5);
            c.g[A0][0] = ((c.g[A1][0] as i64) < (c.g[A2][0] as i64)) as u64;
            c.g[V0][0] = add(add(c.g[V0][0], c.g[V1][0]), GAINS as u64);
            let gain = Addr(r32(c, V0));
            let g = (gain.read_fpu(c) * level).div(scale);
            set_fpr(c, 0, g);
            gain.write_fpu(c, g);
            if c.g[A0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003c_3dc8);
            }
        }
    }

    // Smooth pans: a summed angle in the back half flips the first gain.
    c.cycles += 4;
    let smooth = c.lhu(ch + FLAGS) & F_SMOOTH != 0;
    c.g[V0][0] = 0;
    c.g[V1][0] = add(c.g[FP][0], c.g[S3][0]);
    if smooth {
        c.cycles += 12;
        c.g[A0][0] = add(add(shl(i, 2), c.g[S7][0]), LAYOUTS as u64);
        let angle = c.lhu(r32(c, V1));
        let offset = c.lh(r32(c, A0));
        c.g[V1][0] = offset;
        let back = sx((angle as u32)
            .wrapping_add(offset as u32)
            .wrapping_sub(0x4001))
            < 0x7fff;
        c.g[V0][0] = back as u64;
        c.g[V1][0] = add(c.g[S5][0], c.g[S3][0]);
        if back {
            c.cycles += 8;
            c.g[A0][0] = sx(GAINS);
            let voice = c.lh(r32(c, V1));
            c.g[V0][0] = add(shl(voice, 5), GAINS as u64);
            let gain = Addr(r32(c, V0));
            let g = -gain.read_fpu(c);
            set_fpr(c, 0, g);
            gain.write_fpu(c, g);
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// PS2GraphicsMan_v92 (0x37dd20)

/// `PS2GraphicsMan` fields the screen projection reads.
mod gfx {
    /// `int`: the clip matrices are up to date.
    pub const CLIP_VALID: u32 = 0x6b90;
    /// `u32`: screen width.
    pub const WIDTH: u32 = 0x5a30;
    /// `u32`: screen height.
    pub const HEIGHT: u32 = 0x5a34;
    /// `2048.0`: the GS drawing-area centre.
    pub const GS_CENTRE: u32 = 0x4500_0000;
    /// The depth clamp (24-bit Z).
    pub const Z_MAX: u64 = 0x00ff_ffff;
}

const UPDATE_VIEW_MATRICES: Callee = Callee {
    addr: 0x0037_d968,
    f: ee::s12::PS2GraphicsMan_UpdateViewMatrices,
};
const PROJECT_POINT: Callee = Callee {
    addr: 0x0037_dee0,
    f: ee::s12::PS2GraphicsMan_ProjectPoint,
};

/// `(float)half` for a `u32 >> 1` as gcc converts it (`cvt.s.w` when the
/// value is non-negative, else halve-convert-double through v0); the
/// halved value is never negative, so the second form never runs.
fn half_to_float(c: &mut Cpu, half: u64, whole: u64) -> FpuF32 {
    if (half as i64) >= 0 {
        c.cycles += 4;
        FpuF32::from_i32(half as i32)
    } else {
        c.cycles += 6;
        let v = (half & 1) | sx((whole as u32) >> 2);
        c.g[V0][0] = v;
        let f = FpuF32::from_i32(v as i32);
        f + f
    }
}

/// `Vec4 PS2GraphicsMan_v92(PS2GraphicsMan* gm, const Vec4* p)`
/// (0x37dd20, result through the hidden pointer in a0): project `p` to the
/// screen. Refresh the view matrices if stale, project to GS coordinates,
/// move x and y from the 2048-centred GS area to pixels (whole numbers),
/// and clamp z to the 24-bit depth range.
pub fn ps2_graphics_man_v92(c: &mut Cpu) {
    use gfx::*;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        let gm = r32(c, S0);
        c.g[V0][0] = c.lw(gm + CLIP_VALID);
        c.g[S2][0] = c.g[A0][0];
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            UPDATE_VIEW_MATRICES.call(c, 0x0037_dd50)?;
        }

        // Project into a vector on the stack.
        c.cycles += 9;
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        PROJECT_POINT.call(c, 0x0037_dd60)?;
        let v = sp(c);
        let width = c.lw(gm + WIDTH);
        c.g[V1][0] = width;
        c.g[A0][0] = c.g[SP][0];
        let half_w = sx((width as u32) >> 1);
        c.g[V0][0] = half_w;
        c.g[A3][0] = c.g[SP][0];
        let half_w = half_to_float(c, half_w, width);
        set_fpr(c, 2, half_w);

        // x -= 2048 - width / 2, truncated.
        c.cycles += 14;
        c.g[AT][0] = GS_CENTRE as u64;
        c.g[A1][0] = sx(v + 4);
        let left = FpuF32(GS_CENTRE) - half_w;
        let x = Addr(v).read_fpu(c) - left;
        set_fpr(c, 1, x);
        let height = c.lw(gm + HEIGHT);
        c.g[V1][0] = height;
        let half_h = sx((height as u32) >> 1);
        c.g[A2][0] = half_h;
        let xi = x.to_i32();
        c.g[V0][0] = sx(xi as u32);
        let x = FpuF32::from_i32(xi);
        set_fpr(c, 0, x);
        Addr(v).write_fpu(c, x);
        let half_h = half_to_float(c, half_h, height);
        set_fpr(c, 2, half_h);

        // y -= 2048 - height / 2, truncated; z as an integer.
        c.cycles += 18;
        c.g[AT][0] = GS_CENTRE as u64;
        c.g[A0][0] = 0xff_0000;
        c.g[A2][0] = sx(v + 8);
        let top = FpuF32(GS_CENTRE) - half_h;
        let y = Addr(v + 4).read_fpu(c) - top;
        let yi = y.to_i32();
        let y = FpuF32::from_i32(yi);
        Addr(v + 4).write_fpu(c, y);
        let z = Addr(v + 8).read_fpu(c);
        set_fpr(c, 1, z);
        let zi = z.to_i32();
        c.f[0] = zi as u32;
        c.g[V0][0] = sx(zi as u32);
        c.g[A0][0] = Z_MAX;
        let depth = if zi < 0 {
            c.cycles += 1;
            0
        } else {
            c.cycles += 4;
            let over = Z_MAX < sx(zi as u32);
            c.g[V0][0] = over as u64;
            if over {
                Z_MAX
            } else {
                sx(zi as u32)
            }
        };
        c.g[V1][0] = depth;

        // Store z, copy the vector out, return it.
        c.cycles += 12;
        let z = FpuF32::from_i32(depth as i32);
        set_fpr(c, 0, z);
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        Addr(v + 8).write_fpu(c, z);
        frame.restore_ra(c, 0x10);
        c.lq(V1, v);
        c.sq(V1, r32(c, S2));
        frame.restore(c, S2, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_UpdateTrackedSounds (0x295208)

/// The tracked-sound ring of `SSXAudioSystem`: five-word records
/// `{ key, handle, expire tick (-1: never), stop when expired, extra }`.
mod tracked {
    /// `{ int count; int* buf; int write; int read; }`.
    pub const RING: u32 = 0x597c;
    /// `int`: ring size in words.
    pub const COUNT: u32 = 0x597c;
    /// `int*`: the words.
    pub const BUF: u32 = 0x5980;
    /// `int`: write index.
    pub const WRITE: u32 = 0x5984;
    /// `int`: read index.
    pub const READ: u32 = 0x5988;
    /// `Audio**`: the sound system (`*p`: tick at +0x260, bank monitor at
    /// +0x1d8).
    pub const AUDIO: u32 = 0x118;
    /// Audio: the current tick.
    pub const TICK: u32 = 0x260;
    /// Audio: the bank monitor.
    pub const BANK_MONITOR: u32 = 0x1d8;
    /// Words per record.
    pub const RECORD: u64 = 5;
}

const BANK_MONITOR_FIND_INSTANCE: Callee = Callee {
    addr: 0x002a_d550,
    f: ee::s08::BankMonitor_FindInstance,
};
const BANK_MONITOR_STOP_HANDLE: Callee = Callee {
    addr: 0x002a_d5f0,
    f: ee::s08::BankMonitor_StopHandle,
};

/// Pop one word of the ring (-1 when empty). Registers on entry: `a0` =
/// write index, `a1` = read index (the first pop loads them, with `v1` =
/// write); the last pop does not reload them.
fn tracked_pop(c: &mut Cpu, this: u32, first: bool, last: bool) -> u64 {
    use tracked::*;
    let ring = this + RING;
    let has_word = if first {
        c.cycles += 4;
        c.g[V0][0] = c.lw(this + READ);
        c.g[A0][0] = c.g[V1][0];
        c.g[A1][0] = c.g[V0][0];
        c.g[V1][0] != c.g[V0][0]
    } else {
        c.cycles += 2;
        let has_word = c.g[A0][0] != c.g[A1][0];
        if has_word {
            c.g[V0][0] = c.lw(ring + 0xc);
        }
        has_word
    };
    if !has_word {
        c.cycles += 2;
        return u64::MAX;
    }
    c.cycles += if first { 10 } else { 9 };
    if first {
        c.g[V0][0] = c.lw(ring + 0xc);
    }
    c.g[A1][0] = add(c.g[A1][0], 1);
    c.g[A0][0] = c.lw(ring + 4);
    c.g[V0][0] = shl(c.g[V0][0], 2);
    c.g[V1][0] = c.lw(this + COUNT);
    c.g[V0][0] = add(c.g[V0][0], c.g[A0][0]);
    c.g[V0][0] = c.lw(r32(c, V0));
    c.g[V1][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
    c.sw(this + READ, c.g[A1][0]);
    if c.g[V1][0] == 0 {
        c.cycles += 1;
        c.sw(this + READ, 0);
    }
    if last {
        c.cycles += 1;
    } else {
        c.cycles += 3;
        c.g[A0][0] = c.lw(this + WRITE);
        c.g[A1][0] = c.lw(this + READ);
    }
    c.g[V0][0]
}

/// Push one word back into the ring unless it is full; false when full
/// (the word is dropped). On entry (after the first) v0 = write index.
fn tracked_push(c: &mut Cpu, this: u32, value: u64, first: bool, last: bool) -> bool {
    use tracked::*;
    if first {
        c.cycles += 6;
        c.g[V0][0] = c.lw(this + WRITE);
    } else {
        c.cycles += 5;
    }
    c.g[V1][0] = c.lw(this + COUNT);
    c.g[V0][0] = add(c.g[V0][0], 1);
    c.div(0, V0, V1);
    c.cycles += 4;
    c.g[V0][0] = c.lw(this + READ);
    c.g[A0][0] = c.hi[0];
    if c.g[A0][0] == c.g[V0][0] {
        if !last {
            c.g[V0][0] = c.lw(this + WRITE);
        }
        return false;
    }
    c.cycles += 11;
    let write = c.lw(this + WRITE);
    let buf = c.lw(this + BUF);
    c.g[V0][0] = add(shl(write, 2), buf);
    c.sw(r32(c, V0), value);
    c.g[V1][0] = add(c.lw(this + WRITE), 1);
    let count = c.lw(this + COUNT);
    c.g[V0][0] = ((c.g[V1][0] as i64) < (count as i64)) as u64;
    c.sw(this + WRITE, c.g[V1][0]);
    let wrapped = c.g[V0][0] == 0;
    if last {
        if wrapped {
            c.cycles += 2;
            c.sw(this + WRITE, 0);
        }
    } else {
        if wrapped {
            c.cycles += 1;
            c.sw(this + WRITE, 0);
        }
        c.cycles += 1;
        c.g[V0][0] = c.lw(this + WRITE);
    }
    true
}

/// `void SSXAudioSystem_UpdateTrackedSounds(SSXAudioSystem* this)`
/// (0x295208): go once through the records queued in the tracked-sound
/// ring. A record whose sound has gone (negative handle, or no bank-monitor
/// instance) is dropped; a live one is queued again until its expiry tick,
/// then dropped, stopping its sound first if the record asks for it.
pub fn ssx_audio_system_update_tracked_sounds(c: &mut Cpu) {
    use tracked::*;
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x90);
        c.g[A1][0] = RECORD;
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save(c, S6, 0x20);
        frame.save(c, S7, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, S0);

        // Records queued: ((write - read + count) % count) / 5.
        c.g[A0][0] = c.lw(this + READ);
        let write = c.lw(this + WRITE);
        c.g[V0][0] = c.lw(this + COUNT);
        c.g[V1][0] = add(
            sx((write as u32).wrapping_sub(c.g[A0][0] as u32)),
            c.g[V0][0],
        );
        c.cycles += 6;
        c.div(0, V1, V0);
        c.g[A0][0] = c.hi[0];
        c.div(0, A0, A1);
        c.g[A0][0] = c.lo[0];
        c.g[S7][0] = c.g[A0][0];
        if (c.g[A0][0] as i64) > 0 {
            c.cycles += 3;
            c.g[S1][0] = sx(this + RING);
            c.g[V1][0] = c.lw(this + WRITE);
            loop {
                let key = tracked_pop(c, this, true, false);
                c.g[S4][0] = key;
                let handle = tracked_pop(c, this, false, false);
                c.g[S2][0] = handle;
                let expire = tracked_pop(c, this, false, false);
                c.g[S3][0] = expire;
                let stop = tracked_pop(c, this, false, false);
                c.g[S5][0] = stop;
                let extra = tracked_pop(c, this, false, true);
                c.g[S6][0] = extra;
                tracked_record(c, this, [key, handle, expire, stop, extra])?;
                c.cycles += 2;
                if c.g[S7][0] == 0 {
                    break;
                }
                c.g[V1][0] = c.lw(this + WRITE);
                if event_due(c) {
                    deliver_event_here(c, 0x0029_5278);
                }
            }
        }
        c.cycles += 11;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore(c, S7, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// One popped record: requeue, drop or stop it; counts it off (s7).
fn tracked_record(c: &mut Cpu, this: u32, rec: [u64; 5]) -> crate::call::Flow {
    use tracked::*;
    let [_, handle, expire, stop, _] = rec;
    let count_off = |c: &mut Cpu| c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_sub(1));
    c.cycles += 2;
    if (handle as i64) < 0 {
        count_off(c);
        return Ok(());
    }
    c.cycles += 2;
    c.g[A1][0] = handle;
    if handle != 0 {
        // Gone from the bank monitor: drop it.
        c.cycles += 7;
        let audio = c.lw(this + AUDIO);
        c.g[V0][0] = audio;
        c.g[A2][0] = 1;
        c.g[A0][0] = add(c.lw(audio as u32), BANK_MONITOR as u64);
        BANK_MONITOR_FIND_INSTANCE.call(c, 0x0029_53f4)?;
        if c.g[V0][0] == 0 {
            count_off(c);
            return Ok(());
        }
    }
    c.cycles += 6;
    let audio = c.lw(this + AUDIO);
    let sys = c.lw(audio as u32);
    c.g[V1][0] = sys;
    let tick = c.lw((sys as u32).wrapping_add(TICK));
    c.g[V0][0] = ((tick as i64) < (expire as i64)) as u64;
    c.g[A0][0] = add(sys, BANK_MONITOR as u64);
    let keep = if c.g[V0][0] != 0 {
        true
    } else {
        c.cycles += 3;
        c.g[V0][0] = u64::MAX;
        expire == u64::MAX
    };
    if keep {
        // Queue the record again (words that do not fit are dropped).
        for (i, word) in rec.into_iter().enumerate() {
            let last = i == 4;
            let stored = tracked_push(c, this, word, i == 0, last);
            if last && !stored {
                count_off(c);
                return Ok(());
            }
        }
    } else {
        // Expired: stop the sound if asked.
        c.cycles += 2;
        c.g[A1][0] = handle;
        if stop != 0 {
            c.cycles += 3;
            c.f[12] = 0;
            c.g[A2][0] = 1;
            BANK_MONITOR_STOP_HANDLE.call(c, 0x0029_55f0)?;
        }
    }
    c.cycles += 1;
    count_off(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// AttachedEmitterFX_Render (0x2f2088)

/// `AttachedEmitterFX` fields.
mod attached_fx {
    /// `DynamicEmitter*[10]`: the attached emitters.
    pub const ITEMS: u32 = 0x6c;
    /// Slots in `ITEMS`.
    pub const ITEM_COUNT: u64 = 10;
    /// `Rider*`: the rider the effect is attached to (visibility).
    pub const RIDER: u32 = 0x98;
    /// `RiderInfo*`: the owner's render info (draw level, visibility).
    pub const INFO: u32 = 0x9c;
    /// Rider: drawn this frame.
    pub const RIDER_VISIBLE: u32 = 0xb18;
    /// RiderInfo: draw level (negative: none).
    pub const INFO_LEVEL: u32 = 0xc2c;
    /// RiderInfo: drawn this frame.
    pub const INFO_VISIBLE: u32 = 0xcc8;
    /// The draw level without an owner.
    pub const DEFAULT_LEVEL: u64 = 7;
    /// The draw level of an owner without one.
    pub const NO_LEVEL: u64 = 0x16;
}

/// `void AttachedEmitterFX_Render(AttachedEmitterFX* fx)` (0x2f2088): draw
/// each attached emitter (`DynamicEmitter_Render(emitter, level)`) when the
/// rider or its render info is visible; the level is the info's level + 1
/// (0x16 when it has none, 7 without info).
pub fn attached_emitter_fx_render(c: &mut Cpu) {
    use attached_fx::*;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        let fx = r32(c, S1);
        c.g[S0][0] = sx(fx + ITEMS);
        frame.save_ra(c, 0);
        c.g[S2][0] = ITEM_COUNT - 1;
        loop {
            c.cycles += 4;
            let slot = r32(c, S0);
            let emitter = c.lw(slot);
            c.g[V0][0] = emitter;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            c.g[A1][0] = DEFAULT_LEVEL;
            if emitter != 0 {
                // Draw level.
                c.cycles += 3;
                let info = c.lw(fx + INFO);
                c.g[V0][0] = info;
                c.g[A0][0] = info;
                if info != 0 {
                    c.cycles += 5;
                    let level = c.lw((info as u32).wrapping_add(INFO_LEVEL));
                    c.g[V1][0] = ((level as i64) < 0) as u64;
                    c.g[V0][0] = add(level, 1);
                    c.g[A1][0] = if c.g[V1][0] == 0 {
                        c.g[V0][0]
                    } else {
                        NO_LEVEL
                    };
                }
                // Visible: the rider, else the info.
                c.cycles += 3;
                let rider = c.lw(fx + RIDER);
                c.g[V0][0] = rider;
                let mut visible = false;
                if rider != 0 {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((rider as u32).wrapping_add(RIDER_VISIBLE));
                    visible = c.g[V0][0] != 0;
                }
                if !visible {
                    c.cycles += 2;
                    if info != 0 {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((info as u32).wrapping_add(INFO_VISIBLE));
                        visible = c.g[V0][0] != 0;
                    }
                }
                if visible {
                    c.cycles += 2;
                    c.g[A0][0] = c.lw(slot);
                    // DynamicEmitter_Render (cross-shard: call_indirect).
                    call_ptr(c, 0x0037_1380, 0x002f_210c)?;
                }
            }
            c.cycles += 2;
            let more = (c.g[S2][0] as i64) >= 0;
            c.g[S0][0] = add(c.g[S0][0], 4);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002f_20a8);
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fill, scramble, Rng, DATA};

    /// `SND_PanLookup`'s sine table (`0x450000 - 0x2288`).
    const PAN_SINES: u32 = 0x0044_dd78;
    /// `SND_GetPanGains`' per-speaker byte tables (pointers).
    const PAN_TABLES: u32 = 0x0050_ac38;

    /// A channel (snd's fixture) with 1-6 speakers, 0-2 aux sends, 0-7
    /// voices, random smooth/SPU flags, angles, layouts and pan tables.
    fn apply_pan_setup(c: &mut Cpu, seed: u32) {
        use crate::game::lib::snd::tests::{channel_flags, setup_channel};
        let mut r = Rng(0x38e0_0001 ^ (seed as u64) << 8);
        scramble(c, &mut r);
        c.pc = 0x003c_38e0;
        let flags = channel_flags(&mut r, seed);
        setup_channel(c, &mut r, seed, flags);
        let ch = DATA + (c.g[A0][0] as u32) * 0x8c;
        let speakers = [1u8, 2, 4, 5, 3, 6][(seed % 6) as usize];
        c.bus.write8(pan::STATE + pan::SPEAKERS, speakers);
        c.bus
            .write8(pan::STATE + pan::AUX_SENDS, ((seed / 6) % 3) as u8);
        c.bus.write8(ch + pan::VOICE_COUNT, ((seed / 18) % 8) as u8);
        let mut f = flags & !0x48;
        if r.next_u32() & 1 != 0 {
            f |= 0x40;
        }
        if r.next_u32() & 1 != 0 {
            f |= 0x8;
        }
        c.bus.write16(ch + pan::FLAGS, f);
        for i in 0..8 {
            c.bus
                .write16(ch + pan::VOICES + 2 * i, (6 + r.next_u32() % 18) as u16);
            c.bus
                .write16(ch + pan::VOICE_ANGLES + 2 * i, r.next_u32() as u16);
        }
        c.bus.write16(ch + pan::ANGLE, r.next_u32() as u16);
        fill(c, &mut r, pan::LAYOUTS, 0x30);
        fill(c, &mut r, PAN_SINES, 0x102);
        for i in 0..8 {
            c.bus
                .write32(PAN_TABLES + 4 * i, DATA + 0x4_0000 + 0x100 * i);
        }
        fill(c, &mut r, DATA + 0x4_0000, 0x200);
    }

    /// `SND_ChannelApplyPan`: every speaker layout, the mono, one-voice
    /// and LFE paths, smooth and table pans, the SPU centre/rear split.
    #[test]
    fn snd_channel_apply_pan_matches() {
        check(
            ee::s13::f_003c38e0__generated,
            snd_channel_apply_pan,
            300,
            &apply_pan_setup,
        );
    }

    /// `SSXAudioSystem_UpdateTrackedSounds`: rings of several sizes (also
    /// 0, read walking up to write 0: empty pops), records with negative,
    /// zero and live/gone handles, never/expired/pending ticks, stop flags,
    /// nearly full rings (dropped requeues).
    #[test]
    fn ssx_audio_system_update_tracked_sounds_matches() {
        use crate::difftest::fakes::machine;
        check(
            ee::s08::f_00295208__generated,
            ssx_audio_system_update_tracked_sounds,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x29_5208, 0);
                c.pc = 0x0029_5208;
                let this = DATA;
                let buf = DATA + 0x1_0000;
                let audio = DATA + 0x2_0000;
                let sys = DATA + 0x2_1000;
                let bm = sys + tracked::BANK_MONITOR;
                let insts = DATA + 0x3_0000;
                let count = [0u32, 10, 12, 23, 40, 9][(seed % 6) as usize];
                let (read, write) = if count == 0 {
                    (0u32.wrapping_sub(5 + r.next_u32() % 11), 0)
                } else {
                    let read = r.next_u32() % count;
                    let pending = if seed % 5 == 0 {
                        count - 1
                    } else {
                        r.next_u32() % count
                    };
                    (read, (read + pending) % count)
                };
                c.bus.write32(this + tracked::COUNT, count);
                c.bus.write32(this + tracked::BUF, buf);
                c.bus.write32(this + tracked::WRITE, write);
                c.bus.write32(this + tracked::READ, read);
                let tick = r.next_u32() % 1000;
                let n = count.max(16);
                for k in 0..n {
                    let field = k % 5;
                    let w = match field {
                        1 => [u32::MAX, 0, 1, 2, 3][(r.next_u32() % 5) as usize],
                        2 => match r.next_u32() % 3 {
                            0 => u32::MAX,
                            1 => tick.wrapping_sub(r.next_u32() % 5),
                            _ => tick + 1 + r.next_u32() % 5,
                        },
                        3 => r.next_u32() % 2,
                        _ => r.next_u32(),
                    };
                    let at = if count == 0 {
                        buf.wrapping_add(read.wrapping_add(k).wrapping_mul(4))
                    } else {
                        buf + 4 * ((read + k) % count)
                    };
                    c.bus.write32(at, w);
                }
                // A stop flag only with handle 0 / -1: stopping a live
                // instance would reach the sound driver (FadeOut).
                for k in 0..n {
                    if k % 5 == 3 {
                        let idx = |k: u32| {
                            if count == 0 {
                                buf.wrapping_add(read.wrapping_add(k).wrapping_mul(4))
                            } else {
                                buf + 4 * ((read + k) % count)
                            }
                        };
                        let handle = c.bus.read32(idx(k - 2));
                        if handle != 0 && handle != u32::MAX {
                            c.bus.write32(idx(k), 0);
                        }
                    }
                }
                c.bus.write32(this + tracked::AUDIO, audio);
                c.bus.write32(audio, sys);
                c.bus.write32(sys + tracked::TICK, tick);
                c.bus.write32(bm + 0x8ec, insts);
                // Live instances (state 1, not stopping) for some handles.
                for id in 1..=3u32 {
                    if r.next_u32() % 4 != 0 {
                        let inst = insts + 0xc0 * (id * 10 + r.next_u32() % 10);
                        c.bus.write32(inst, 1);
                        c.bus.write32(inst + 4, id);
                        c.bus.write32(inst + 0x9c, 0);
                    }
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `AttachedEmitterFX_Render`: empty and filled slots, with/without
    /// rider and info, negative/positive levels, every visibility case.
    /// The emitters have no particles (+0x174 = 0): `DynamicEmitter_Render`
    /// returns at once.
    #[test]
    fn attached_emitter_fx_render_matches() {
        check(
            ee::s09::f_002f2088__generated,
            attached_emitter_fx_render,
            300,
            &|c, seed| {
                let mut r = Rng(0x2f20_8801 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x002f_2088;
                let fx = DATA;
                for i in 0..10u32 {
                    let e = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        DATA + 0x1000 + 0x200 * i
                    };
                    c.bus.write32(fx + attached_fx::ITEMS + 4 * i, e);
                }
                let rider = DATA + 0x5000;
                let info = DATA + 0x8000;
                let has_rider = seed % 3 != 0;
                let has_info = (seed / 3) % 3 != 0;
                c.bus
                    .write32(fx + attached_fx::RIDER, if has_rider { rider } else { 0 });
                c.bus
                    .write32(fx + attached_fx::INFO, if has_info { info } else { 0 });
                c.bus
                    .write32(rider + attached_fx::RIDER_VISIBLE, r.next_u32() % 2);
                c.bus
                    .write32(info + attached_fx::INFO_VISIBLE, r.next_u32() % 2);
                let level = if r.next_u32() % 3 == 0 {
                    r.next_u32() | 0x8000_0000
                } else {
                    r.next_u32() % 32
                };
                c.bus.write32(info + attached_fx::INFO_LEVEL, level);
                c.g[A0][0] = sx(fx);
            },
        );
    }

    /// `PS2GraphicsMan_v92`: stale clip matrices (UpdateViewMatrices runs)
    /// or fresh ones, screen sizes with the top bit set, depths below 0,
    /// inside and above the 24-bit range.
    #[test]
    fn ps2_graphics_man_v92_matches() {
        use crate::game::render::particle::tests::put_defer;
        check(
            ee::s12::f_0037dd20__generated,
            ps2_graphics_man_v92,
            300,
            &|c, seed| {
                let mut r = Rng(0x37dd_2001 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0037_dd20;
                let gm = DATA;
                let stale = seed % 3 == 0;
                c.bus.write32(
                    gm + gfx::CLIP_VALID,
                    if stale { 0 } else { 1 + r.next_u32() % 9 },
                );
                if stale {
                    let cam = DATA + 0x1_0000;
                    let defer = put_defer(c);
                    for i in 0..16u32 {
                        c.bus.write32(cam + 4 * i, r.f32(2.0));
                        c.bus.write32(gm + 0x6af0 + 4 * i, r.f32(2.0));
                        c.bus.write32(gm + 0x57c0 + 4 * i, r.f32(2.0));
                        c.bus.write32(gm + 0x5860 + 4 * i, r.f32(2.0));
                    }
                    c.bus.write32(gm + 0x13e4, cam);
                    c.bus.write32(gm + 0x18f0, defer);
                }
                // Clip matrix, GS scale and offset (ProjectPoint).
                for i in 0..16u32 {
                    c.bus.write32(gm + 0x58a0 + 4 * i, r.f32(2.0));
                }
                for i in 0..4u32 {
                    c.bus.write32(gm + 0x58e0 + 4 * i, r.f32(300.0));
                    c.bus.write32(gm + 0x58f0 + 4 * i, r.f32(4096.0));
                }
                let depth = [-1.0e3f32, 1.0e6, 3.0e7][(seed / 3 % 3) as usize];
                c.bus.write32(
                    gm + 0x58f8,
                    (depth + (r.next_u32() % 1000) as f32).to_bits(),
                );
                let (w, h) = match seed % 4 {
                    0 => (640, 448),
                    1 => (512, 224),
                    2 => (r.next_u32() | 0x8000_0000, r.next_u32() | 0x8000_0000),
                    _ => (r.next_u32() % 2048, r.next_u32() % 2048),
                };
                c.bus.write32(gm + gfx::WIDTH, w);
                c.bus.write32(gm + gfx::HEIGHT, h);
                let p = DATA + 0x9000;
                for i in 0..3u32 {
                    c.bus.write32(p + 4 * i, r.f32(100.0));
                }
                c.bus.write32(p + 12, (1.0f32).to_bits());
                c.g[A0][0] = sx(DATA + 0xa000);
                c.g[A1][0] = sx(gm);
                c.g[A2][0] = sx(p);
            },
        );
    }
}
