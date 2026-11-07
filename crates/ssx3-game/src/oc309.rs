//! Staging wave 212 (oc309): hand-written replacements for the hottest
//! still-generated functions of a race replay (calls per frame times size).
//! Each reproduces the generated body's registers, cycles, memory and call
//! order.

#![allow(unused_imports)]

use crate::abi;
use crate::abi::{r32, ret};
use crate::guest::Addr;
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003d_fcd8, stream_file_alloc_request, "StreamFile_AllocRequest"),
    (0x002b_04d8, speech_instance_play_event, "SpeechInstance_PlayEvent"),
    (0x003c_8d00, snd_ee_voice_set_lowpass, "SND_EeVoiceSetLowpass"),
    (0x0030_98b0, wscript_man_post_air_event, "WScriptMan_PostAirEvent"),
    (0x0031_d660, c_csi_cubic_spline_alloc, "cCSICubicSpline_Alloc"),
    (0x002a_3fe8, ssx_audio_system_is_track_group4, "SSXAudioSystem_IsTrackGroup4"),
    (0x002f_c2c0, world_obj_factory_dead_node, "WorldObjFactory_DeadNode"),
    (0x003c_3010, snd_channel_apply_time_stretch, "SND_ChannelApplyTimeStretch"),
];

// ---------------------------------------------------------------------------
// StreamFile_AllocRequest (0x3dfcd8)

/// The stream-file request serial counter (`0x520000 - 0x63e8`); bumped by
/// 0x100 per request, never 0, kept in the request's tag word above the
/// low byte.
pub(crate) const STREAM_FILE_REQUEST_SERIAL: u32 = 0x0051_9c18;
/// `StreamFile` fields: the mutex, and the head of the free request list.
pub(crate) const STREAM_FILE_MUTEX: u32 = 0x4;
pub(crate) const STREAM_FILE_FREE_REQUESTS: u32 = 0x68;
/// Request field: the next request in the free list.
pub(crate) const STREAM_FILE_REQUEST_NEXT: u32 = 0xc;

/// `StreamFileRequest* StreamFile_AllocRequest(StreamFile* sf)` (0x3dfcd8):
/// under the stream file's mutex, pops a request off the free list, gives it
/// the next serial (`serial += 0x100`, skipping 0) in its tag word (keeping
/// the low byte); returns it, or null when the list is empty.
pub fn stream_file_alloc_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(STREAM_FILE_MUTEX));
        Callee {
            addr: 0x003e_5700,
            f: ee::s14::EAMutex_Lock,
        }
        .call(c, 0x003d_fcf4)?;
        let sf = r32(c, S0);
        c.g[V0][0] = Addr(sf + STREAM_FILE_FREE_REQUESTS).lw(c);
        c.g[S1][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 8;
            let req = c.g[V0][0];
            c.g[S1][0] = req;
            c.g[A0][0] = 0x52_0000;
            let serial = Addr(STREAM_FILE_REQUEST_SERIAL).lw(c);
            let next = Addr((req as u32).wrapping_add(STREAM_FILE_REQUEST_NEXT)).lw(c);
            c.g[V1][0] = next;
            let serial = sx((serial as u32).wrapping_add(0x100));
            c.g[V0][0] = serial;
            Addr(sf + STREAM_FILE_FREE_REQUESTS).sw(c, next);
            Addr(STREAM_FILE_REQUEST_SERIAL).sw(c, serial);
            if serial == 0 {
                c.cycles += 2;
                c.g[V0][0] = 0x100;
                Addr(STREAM_FILE_REQUEST_SERIAL).sw(c, 0x100);
            }
            c.cycles += 4;
            let serial = Addr(STREAM_FILE_REQUEST_SERIAL).lw(c);
            c.g[V1][0] = serial;
            let tag = Addr(req as u32).lbu(c) | serial;
            c.g[V0][0] = tag;
            Addr(req as u32).sw(c, tag);
        }
        c.cycles += 8;
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(STREAM_FILE_MUTEX));
        Callee {
            addr: 0x003e_5760,
            f: ee::s14::EAMutex_Unlock,
        }
        .call(c, 0x003d_fd40)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SpeechInstance_PlayEvent (0x2b04d8)

/// The speech system (`*(gp + 0x6c4)`): the stream array (+0), the
/// instance table (+4), per-language path prefixes (0x14 bytes each from
/// +0x158) and the speech player object (+0x1bc vtable).
pub(crate) const SPEECH_PLAY_SYSTEM_GP: u32 = 0x6c4;
pub(crate) const SPEECH_PLAY_LANG_PREFIXES: u64 = 0x158;
pub(crate) const SPEECH_PLAY_PLAYER_VTABLE: u32 = 0x1bc;
/// The bank monitor: `*(*system) + 0x1d8` (the stream array's first word).
pub(crate) const SPEECH_PLAY_MONITOR: u32 = 0x1d8;
/// A speech instance: its id (+0), stream slot (+4), bank count (+0xf0)
/// and bank pointers (+0xf4), and the play parameters (+0x8c8..+0x8fc).
pub(crate) const SPEECH_PLAY_STREAM: u32 = 0x4;
pub(crate) const SPEECH_PLAY_BANK_COUNT: u32 = 0xf0;
pub(crate) const SPEECH_PLAY_BANKS: u32 = 0xf4;
/// An event: id (+0), argument (+4), context key (u16 +0xc).
pub(crate) const SPEECH_PLAY_EVENT_ARG: u32 = 0x4;
pub(crate) const SPEECH_PLAY_EVENT_KEY: u32 = 0xc;
/// `"%s%s%s.dat"` and `"data/"`; the two timer parameter blocks.
pub(crate) const SPEECH_PLAY_FORMAT: u64 = 0x0048_32f8;
pub(crate) const SPEECH_PLAY_DATA_DIR: u64 = 0x004a_3590;
pub(crate) const SPEECH_PLAY_TIMER_PLAYING: u32 = 0x004a_37d8;
pub(crate) const SPEECH_PLAY_TIMER_IDLE: u32 = 0x004a_37e0;

const SPEECH_PLAY_HAS_ID: Callee = Callee {
    addr: 0x002a_fa58,
    f: ee::s08::SpeechBank_HasId,
};

/// The speech system pointer (`lw gp+0x6c4`).
fn speech_play_system(c: &mut Cpu) -> u64 {
    let gp = r32(c, GP);
    Addr(gp.wrapping_add(SPEECH_PLAY_SYSTEM_GP)).lw(c)
}

/// The player's start call (vtable slot 8) with the instance's play
/// parameters; `t1` is the timer argument (-1, or the instance's +0x8c8).
/// The registers the two call sites leave differ only in `t1`.
fn speech_play_start(c: &mut Cpu, system: u64, t1: Option<u64>, ret: u32) -> crate::call::Flow {
    let inst = r32(c, S2);
    let event = r32(c, S4);
    let params = sp(c);
    c.g[A2][0] = sx(params.wrapping_add(0x10));
    let gain = Addr(inst.wrapping_add(0x8fc)).lw(c);
    c.g[V0][0] = gain;
    let vt = Addr((system as u32).wrapping_add(SPEECH_PLAY_PLAYER_VTABLE)).lw(c);
    c.g[V1][0] = vt;
    c.g[A3][0] = Addr(event.wrapping_add(SPEECH_PLAY_EVENT_ARG)).lw(c);
    Addr(params).sw(c, gain);
    let slot = crate::call::VSlot::read(c, vt, 8);
    c.g[A0][0] = slot.this(system);
    c.g[A1][0] = Addr(inst.wrapping_add(SPEECH_PLAY_STREAM)).lw(c);
    c.g[V0][0] = slot.func;
    c.g[T0][0] = Addr(inst.wrapping_add(0x8cc)).lw(c);
    c.g[T1][0] = match t1 {
        Some(v) => v,
        None => Addr(inst.wrapping_add(0x8c8)).lw(c),
    };
    c.g[T2][0] = Addr(inst.wrapping_add(0x8d0)).lw(c);
    c.lwc1(12, inst.wrapping_add(0x8f4));
    c.g[T3][0] = Addr(inst.wrapping_add(0x8f8)).lw(c);
    call_ptr(c, slot.func as u32, ret)
}

/// `int SpeechInstance_PlayEvent(SpeechInstance* self, SpeechEvent* ev)`
/// (0x2b04d8): finds the bank holding the event id (the bank at index `id`
/// first, else the last of the instance's banks whose id matches), applies
/// the event context (`SpeechSystem_ApplyEventContext`); with neither,
/// returns -1. Otherwise it builds the bank's stream path (`sprintf(buf,
/// "%s%s%s.dat", "data/", lang prefix, bank name)`, the language from the
/// name's suffix), asks the player for a stream handle, and:
/// - stream slot idle: sets the stream's volume scale pointer (when the
///   player gives one) and returns the player's start call (slot 8) with
///   the instance's timer argument;
/// - stream busy: starts with timer -1 and, on success, queues a bank
///   monitor timer for the queued start (playing: time remaining as the
///   delay, else 200), returning the start result.
///
/// Frame 0x170 (s0..s4; the path buffer at sp+0x10).
pub fn speech_instance_play_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x170);
        frame.save(c, S1, 0x150);
        frame.save(c, S2, 0x140);
        frame.save(c, S3, 0x130);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x120);
        c.g[S1][0] = sx(r32(c, S2).wrapping_add(SPEECH_PLAY_BANKS));
        frame.save(c, S0, 0x160);
        c.g[S4][0] = c.g[A1][0];
        frame.save_ra(c, 0x110);
        c.g[S3][0] = u64::MAX;
        // The bank at index `id` first.
        let id = Addr(r32(c, S4)).lw(c);
        c.g[A1][0] = id;
        let at = r32(c, S1).wrapping_add((id as u32) << 2);
        c.g[V0][0] = sx(at);
        c.g[A0][0] = Addr(at).lw(c);
        SPEECH_PLAY_HAS_ID.call(c, 0x002b_0518)?;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[S3][0] = Addr(r32(c, S4)).lw(c);
        } else {
            // Else scan every bank; the last match wins.
            c.g[V0][0] = Addr(r32(c, S2).wrapping_add(SPEECH_PLAY_BANK_COUNT)).lw(c);
            c.cycles += 2;
            c.g[S0][0] = 0;
            if (c.g[V0][0] as i64) > 0 {
                c.cycles += 2;
                c.g[A0][0] = Addr(r32(c, S1)).lw(c);
                loop {
                    c.cycles += 9;
                    c.g[A1][0] = Addr(r32(c, S4)).lw(c);
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
                    SPEECH_PLAY_HAS_ID.call(c, 0x002b_0544)?;
                    if c.g[V0][0] != 0 {
                        c.g[S3][0] = c.g[S0][0];
                    }
                    let count = Addr(r32(c, S2).wrapping_add(SPEECH_PLAY_BANK_COUNT)).lw(c);
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (count as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.g[A0][0] = Addr(r32(c, S1)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x002b_0538);
                    }
                }
            }
        }
        c.cycles += 2;
        c.g[A0][0] = speech_play_system(c);
        let mut found = (c.g[S3][0] as i64) >= 0;
        if found {
            c.cycles += 5;
            let inst = r32(c, S2);
            c.g[A1][0] = Addr(inst).lw(c);
            c.g[A2][0] = Addr(r32(c, S4).wrapping_add(SPEECH_PLAY_EVENT_KEY)).lhu(c);
            Callee {
                addr: 0x002b_1520,
                f: ee::s08::SpeechSystem_ApplyEventContext,
            }
            .call(c, 0x002b_0570)?;
            found = c.g[V0][0] != 0;
            c.g[V0][0] = sx(r32(c, S3) << 2);
        }
        if !found {
            c.cycles += 1;
            c.g[V0][0] = u64::MAX;
            c.cycles += 1;
            frame.restore(c, S0, 0x160);
        } else {
            // The stream path: "data/" + language prefix + bank name + ".dat".
            c.cycles += 40;
            let bank_slot = r32(c, S2)
                .wrapping_add(SPEECH_PLAY_BANKS)
                .wrapping_add(r32(c, V0));
            c.g[S0][0] = sx(bank_slot);
            let bank = Addr(bank_slot).lw(c);
            c.g[A0][0] = sx((bank as u32).wrapping_add(4));
            Callee {
                addr: 0x002b_03b8,
                f: ee::s08::Speech_GetLanguageFromSuffix,
            }
            .call(c, 0x002b_058c)?;
            c.g[V1][0] = 0x14;
            let system = speech_play_system(c);
            c.mult(0, V0, V0, V1);
            let bank = Addr(r32(c, S0)).lw(c);
            c.g[A1][0] = SPEECH_PLAY_FORMAT;
            c.g[A2][0] = SPEECH_PLAY_DATA_DIR;
            c.g[A0][0] = sx(sp(c).wrapping_add(0x10));
            c.g[T0][0] = sx((bank as u32).wrapping_add(4));
            let prefix = sx(r32(c, V0).wrapping_add(SPEECH_PLAY_LANG_PREFIXES as u32));
            c.g[V0][0] = prefix;
            c.g[A3][0] = sx((system as u32).wrapping_add(prefix as u32));
            call_ptr(c, 0x0041_7828, 0x002b_05c0)?; // sprintf
            // The player's stream handle for the path.
            let system = speech_play_system(c);
            c.g[A3][0] = system;
            let bank = Addr(r32(c, S0)).lw(c);
            let vt = Addr((system as u32).wrapping_add(SPEECH_PLAY_PLAYER_VTABLE)).lw(c);
            c.g[V1][0] = vt;
            c.g[A2][0] = Addr((bank as u32).wrapping_add(0x40)).lw(c);
            let slot = crate::call::VSlot::read(c, vt, 0x10);
            c.g[V0][0] = slot.func;
            c.g[A1][0] = Addr(r32(c, S2)).lw(c);
            c.g[A0][0] = slot.this(system);
            call_ptr(c, slot.func as u32, 0x002b_05e4)?;
            c.g[S1][0] = c.g[V0][0];
            // Time left on the instance's stream slot, and its state.
            c.g[A1][0] = Addr(r32(c, S2).wrapping_add(SPEECH_PLAY_STREAM)).lw(c);
            let system = speech_play_system(c);
            c.g[V0][0] = system;
            c.g[A0][0] = Addr(system as u32).lw(c);
            Callee {
                addr: 0x002a_ae78,
                f: ee::s08::StreamArray_GetTimeRemaining,
            }
            .call(c, 0x002b_05f8)?;
            c.f[1] = FpuF32(c.f[0]).to_i32() as u32;
            c.g[S3][0] = sx(c.f[1]);
            c.g[A1][0] = Addr(r32(c, S2).wrapping_add(SPEECH_PLAY_STREAM)).lw(c);
            let system = speech_play_system(c);
            c.g[V0][0] = system;
            c.g[A0][0] = Addr(system as u32).lw(c);
            Callee {
                addr: 0x002a_ae08,
                f: ee::s08::StreamArray_GetState,
            }
            .call(c, 0x002b_0610)?;
            let busy = c.g[V0][0] != 0;
            c.g[V0][0] = speech_play_system(c);
            if busy {
                c.cycles += 5;
                c.g[A1][0] = Addr(r32(c, S2).wrapping_add(SPEECH_PLAY_STREAM)).lw(c);
                c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                Callee {
                    addr: 0x002a_b028,
                    f: ee::s08::StreamArray_IsPlaying,
                }
                .call(c, 0x002b_0624)?;
                let playing = c.g[V0][0] != 0;
                let system = speech_play_system(c);
                c.g[T0][0] = system;
                c.cycles += 18;
                let ret = if playing { 0x002b_0668 } else { 0x002b_06d0 };
                speech_play_start(c, system, Some(u64::MAX), ret)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[V0][0] = 0x4a_0000;
                if (c.g[S0][0] as i64) >= 0 {
                    // Queue the bank-monitor timer for the start.
                    let system = speech_play_system(c);
                    c.g[V1][0] = system;
                    if playing {
                        c.cycles += 8;
                        c.g[A2][0] = Addr(SPEECH_PLAY_TIMER_PLAYING).read_u64_unaligned(c);
                        c.g[A1][0] = c.g[S3][0];
                    } else {
                        c.cycles += 7;
                        c.g[A2][0] = Addr(SPEECH_PLAY_TIMER_IDLE).read_u64_unaligned(c);
                        c.g[A1][0] = 0xc8;
                    }
                    c.g[V0][0] = Addr(system as u32).lw(c);
                    c.g[A3][0] = c.g[S2][0];
                    c.g[T1][0] = c.g[S1][0];
                    c.cycles += 6;
                    c.g[T0][0] = c.g[S0][0];
                    let streams = Addr(r32(c, V0)).lw(c);
                    c.g[T2][0] = 0;
                    c.g[T3][0] = 0;
                    c.g[A0][0] = sx((streams as u32).wrapping_add(SPEECH_PLAY_MONITOR));
                    Callee {
                        addr: 0x002a_dca0,
                        f: ee::s08::BankMonitor_AddTimer,
                    }
                    .call(c, 0x002b_0710)?;
                }
                c.cycles += 2;
                c.g[V0][0] = c.g[S0][0];
                c.cycles += 1;
                frame.restore(c, S0, 0x160);
            } else {
                // Idle slot: the volume scale pointer, then start.
                c.cycles += 12;
                let system = speech_play_system(c);
                c.g[T0][0] = system;
                c.g[A2][0] = c.g[S1][0];
                let inst = r32(c, S2);
                c.g[A1][0] = Addr(inst).lw(c);
                let vt = Addr((system as u32).wrapping_add(SPEECH_PLAY_PLAYER_VTABLE)).lw(c);
                c.g[V1][0] = vt;
                c.g[A3][0] = Addr(inst.wrapping_add(0x8d0)).lw(c);
                let slot = crate::call::VSlot::read(c, vt, 0x18);
                c.g[V0][0] = slot.func;
                c.g[A0][0] = slot.this(system);
                call_ptr(c, slot.func as u32, 0x002b_073c)?;
                c.g[A2][0] = c.g[V0][0];
                c.g[V0][0] = speech_play_system(c);
                if c.g[A2][0] != 0 {
                    c.cycles += 3;
                    c.g[A1][0] = Addr(r32(c, S2).wrapping_add(SPEECH_PLAY_STREAM)).lw(c);
                    c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                    Callee {
                        addr: 0x002a_b0d0,
                        f: ee::s08::StreamArray_SetVolumeScalePtr,
                    }
                    .call(c, 0x002b_0754)?;
                }
                c.cycles += 18;
                let system = speech_play_system(c);
                speech_play_start(c, system, None, 0x002b_0794)?;
                frame.restore(c, S0, 0x160);
            }
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x150);
        frame.restore(c, S2, 0x140);
        frame.restore(c, S3, 0x130);
        frame.restore(c, S4, 0x120);
        frame.restore_ra(c, 0x110);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_EeVoiceSetLowpass (0x3c8d00)

/// The EE voice table pointer (`SND mixer 0x515b40 + 0x1dc`).
pub(crate) const LOWPASS_VOICES: u32 = 0x0051_5d1c;
/// A voice entry is 0x60 bytes: DSP chain head at +0x40, its lowpass DSP
/// node at +0x58.
pub(crate) const LOWPASS_VOICE_SIZE: u64 = 0x60;
pub(crate) const LOWPASS_VOICE_DSP_CHAIN: u32 = 0x40;
pub(crate) const LOWPASS_VOICE_FILTER: u32 = 0x58;
/// The SND driver state and its output sample rate (u16 at +0x2a).
pub(crate) const LOWPASS_SND_STATE: u32 = 0x0050_a8e8;
pub(crate) const LOWPASS_STATE_RATE: u32 = 0x2a;

const LOWPASS_INIT: Callee = Callee {
    addr: 0x003c_a870,
    f: ee::s14::SND_DspLowpass_Init,
};
const LOWPASS_INSERT_SORTED: Callee = Callee {
    addr: 0x003c_8f48,
    f: ee::s14::SND_DspInsertSorted,
};
const LOWPASS_SET_PARAMS: Callee = Callee {
    addr: 0x003c_a8a0,
    f: ee::s14::SND_DspLowpass_SetParams,
};
const LOWPASS_DSP_REMOVE: Callee = Callee {
    addr: 0x003c_8fb0,
    f: ee::s14::SND_DspRemove,
};

/// `void SND_EeVoiceSetLowpass(int voice, float cutoff)` (0x3c8d00): a
/// cutoff below 1.0 (a fraction of the output rate) gives the voice a
/// lowpass DSP (created on first use: `SND_MemAlloc(0x28)`, priority 0x28,
/// `SND_DspLowpass_Init`, inserted into the voice's DSP chain) and sets it
/// to `round(cutoff * rate) << 7` over `rate << 8` (gain 0x100); a cutoff
/// of 1.0 or more removes and frees the voice's lowpass DSP, if any.
/// Frame 0x40 (s0 = voice entry, f20 = cutoff); the entry offset uses
/// `mult`.
pub fn snd_ee_voice_set_lowpass(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = LOWPASS_VOICE_SIZE;
        c.mult(0, A0, A0, V0);
        frame.save_fpr(c, 20, 0x30);
        c.g[AT][0] = 0x3f80_0000;
        c.f[0] = FpuF32::ONE.0;
        c.f[20] = c.f[12];
        c.g[V1][0] = 0x51_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = Addr(LOWPASS_VOICES).lw(c);
        let below_one = FpuF32(c.f[20]).lt(FpuF32::ONE);
        abi::set_fcc(c, below_one);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
        if below_one {
            c.cycles += 3;
            let voice = r32(c, S0);
            c.g[V0][0] = Addr(voice.wrapping_add(LOWPASS_VOICE_FILTER)).lw(c);
            c.g[V1][0] = 0x51_0000;
            if c.g[V0][0] == 0 {
                // Create the lowpass DSP and insert it into the chain.
                c.cycles += 15;
                c.g[A0][0] = 0x28;
                call_ptr(c, 0x003b_5c60, 0x003c_8d4c)?; // SND_MemAlloc
                let filter = Addr(r32(c, S0).wrapping_add(LOWPASS_VOICE_FILTER));
                let dsp = c.g[V0][0];
                filter.sw(c, dsp);
                c.g[A0][0] = 0x28;
                Addr((dsp as u32).wrapping_add(4)).sw(c, 0);
                c.g[V0][0] = filter.lw(c);
                Addr(r32(c, V0).wrapping_add(0x18)).sh(c, 0x28);
                c.g[V1][0] = filter.lw(c);
                Addr(r32(c, V1).wrapping_add(0x1a)).sb(c, 0);
                c.g[A0][0] = filter.lw(c);
                LOWPASS_INIT.call(c, 0x003c_8d70)?;
                let voice = r32(c, S0);
                c.g[A1][0] = Addr(voice.wrapping_add(LOWPASS_VOICE_FILTER)).lw(c);
                c.g[A0][0] = sx(voice.wrapping_add(LOWPASS_VOICE_DSP_CHAIN));
                LOWPASS_INSERT_SORTED.call(c, 0x003c_8d7c)?;
                c.g[V1][0] = 0x51_0000;
            }
            // params = { round(cutoff * rate) << 7, rate << 8, 0x100 }.
            c.cycles += 14;
            c.f[1] = 0;
            c.g[V1][0] = sx(LOWPASS_SND_STATE);
            c.g[AT][0] = 0x3f00_0000;
            c.f[2] = 0x3f00_0000;
            let rate = Addr(LOWPASS_SND_STATE + LOWPASS_STATE_RATE).lhu(c);
            c.g[A0][0] = rate;
            let rate_f = FpuF32::from_i32(rate as i32);
            c.f[0] = rate_f.0;
            c.g[V0][0] = sx((rate as u32) << 8);
            let scaled = FpuF32(c.f[20]) * rate_f;
            c.f[12] = scaled.0;
            let negative = scaled.lt(FpuF32::ZERO);
            abi::set_fcc(c, negative);
            let params = sp(c);
            let rate_word = c.g[V0][0];
            Addr(params + 4).sw(c, rate_word);
            if negative {
                c.cycles += 2;
                c.g[AT][0] = 0xffff_ffff_bf00_0000;
                c.f[2] = 0xbf00_0000;
            }
            c.cycles += 12;
            let rounded = FpuF32(c.f[2]) + scaled;
            c.f[2] = rounded.0;
            c.f[4] = rounded.to_i32() as u32;
            let cut = sx(c.f[4] << 7);
            c.g[V0][0] = cut;
            c.g[V1][0] = 0x100;
            Addr(params).sw(c, cut);
            c.g[A1][0] = sx(params);
            Addr(params + 8).sw(c, 0x100);
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(LOWPASS_VOICE_FILTER)).lw(c);
            LOWPASS_SET_PARAMS.call(c, 0x003c_8de8)?;
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            let voice = r32(c, S0);
            c.g[A1][0] = Addr(voice.wrapping_add(LOWPASS_VOICE_FILTER)).lw(c);
            if c.g[A1][0] != 0 {
                // Remove the lowpass DSP from the chain and free it.
                c.cycles += 6;
                c.g[A0][0] = sx(voice.wrapping_add(LOWPASS_VOICE_DSP_CHAIN));
                LOWPASS_DSP_REMOVE.call(c, 0x003c_8e04)?;
                c.g[A0][0] = Addr(r32(c, S0).wrapping_add(LOWPASS_VOICE_FILTER)).lw(c);
                call_ptr(c, 0x003b_5e98, 0x003c_8e0c)?; // SND_MemFree
                Addr(r32(c, S0).wrapping_add(LOWPASS_VOICE_FILTER)).sw(c, 0);
            }
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WScriptMan_PostAirEvent (0x3098b0)

/// The script manager's op list: count at +0x40, then 12-byte ops from
/// +0x44 (`{ int op; int a; int b; }`).
pub(crate) const POST_AIR_OP_COUNT: u32 = 0x40;
pub(crate) const POST_AIR_OP_SIZE: u64 = 0xc;
/// The "air" op code.
pub(crate) const POST_AIR_OP: u64 = 0x7_0002;

/// `void WScriptMan_PostAirEvent(WScriptMan* self)` (0x3098b0): appends
/// op 0x70002 to the manager's op list. The 12-byte op is built in a stack
/// temporary whose other two words are never written (stack leftovers are
/// copied along), copied once on the stack, then into the list with
/// unaligned doubleword moves; the count is bumped. Leaf, frame 0x20.
pub fn wscript_man_post_air_event(c: &mut Cpu) {
    c.cycles += 25;
    let man = r32(c, A0);
    c.g[A1][0] = Addr(man.wrapping_add(POST_AIR_OP_COUNT)).lw(c);
    c.g[V1][0] = POST_AIR_OP_SIZE;
    let frame = Frame::open(c, 0x20);
    c.mult(0, V1, A1, V1);
    c.g[V0][0] = POST_AIR_OP;
    let tmp = sp(c);
    Addr(tmp).sw(c, POST_AIR_OP);
    c.g[A2][0] = sx(man.wrapping_add(POST_AIR_OP_COUNT));
    // The op temporary, copied to sp+0x10.
    let head = Addr(tmp).read_u64_unaligned(c);
    let tail = Addr(tmp + 8).lw(c);
    c.g[V0][0] = head;
    c.g[A3][0] = tail;
    Addr(tmp + 0x10).write_u64_unaligned(c, head);
    Addr(tmp + 0x18).sw(c, tail);
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
    let slot = r32(c, V1).wrapping_add(r32(c, A2));
    c.g[V1][0] = sx(slot);
    // ... and into the list slot.
    let head = Addr(tmp + 0x10).read_u64_unaligned(c);
    let tail = Addr(tmp + 0x18).lw(c);
    c.g[V0][0] = head;
    c.g[A2][0] = tail;
    Addr(slot.wrapping_add(4)).write_u64_unaligned(c, head);
    Addr(slot.wrapping_add(0xc)).sw(c, tail);
    let count = c.g[A1][0];
    Addr(man.wrapping_add(POST_AIR_OP_COUNT)).sw(c, count);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// cCSICubicSpline_Alloc (0x31d660)

/// `cCSICubicSpline`: the flags word at +0 (bits 11..18: segment count,
/// bit 1: allocated, bits 3..10 cleared on alloc), the segment array at +8.
pub(crate) const SPLINE_ALLOC_DATA: u32 = 0x8;
/// The allocation name (`0x490000 - 0x24c8`).
pub(crate) const SPLINE_ALLOC_NAME: u64 = 0x0048_db38;

/// `void cCSICubicSpline_Alloc(cCSICubicSpline* self, int points)`
/// (0x31d660): frees the old segment array (`cCSICubicSpline_Free`), stores
/// `points - 1` as the 8-bit segment count in the flags, allocates
/// `count * 0x14` bytes (`operator_new_named`) and marks the spline
/// allocated (bit 1 set, bits 3..10 cleared). Frame 0x30 (s0 = self, s1 =
/// points).
pub fn c_csi_cubic_spline_alloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(SPLINE_ALLOC_DATA)).lw(c);
        c.g[S1][0] = c.g[A1][0];
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            Callee {
                addr: 0x0031_d790,
                f: ee::s10::cCSICubicSpline_Free,
            }
            .call(c, 0x0031_d688)?;
        }
        c.cycles += 30;
        let spline = r32(c, S0);
        let flags = Addr(spline).lw(c);
        let count = sx(r32(c, S1).wrapping_sub(1)) & 0xff;
        c.g[V0][0] = 0xffff_ffff_fff8_07ff;
        let count_bits = sx((count as u32) << 11);
        c.g[V1][0] = count_bits;
        let flags = (flags & 0xffff_ffff_fff8_07ff) | count_bits;
        c.g[A2][0] = flags;
        c.g[A3][0] = 0x14;
        Addr(spline).sw(c, flags);
        c.g[A0][0] = sx((flags as u32) >> 11) & 0xff;
        c.g[A1][0] = SPLINE_ALLOC_NAME;
        c.mult(0, 0, A0, A3);
        c.g[A2][0] = 0;
        c.g[A0][0] = c.lo[0];
        c.g[A3][0] = 0;
        Callee {
            addr: 0x0031_7e30,
            f: ee::s10::operator_new_named,
        }
        .call(c, 0x0031_d6d4)?;
        let spline = r32(c, S0);
        c.g[A0][0] = 0xffff_ffff_ffff_f807;
        let data = c.g[V0][0];
        let flags = (Addr(spline).lw(c) | 2) & 0xffff_ffff_ffff_f807;
        Addr(spline.wrapping_add(SPLINE_ALLOC_DATA)).sw(c, data);
        c.g[V1][0] = flags;
        Addr(spline).sw(c, flags);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_IsTrackGroup4 (0x2a3fe8)

/// The current track index (`BENewRaceInt`, 0x535c08) and the track info
/// table it indexes (0x64-byte records from 0x43d950, track id first).
pub(crate) const TRACK_GROUP4_TRACK: u32 = 0x0053_5c08;
pub(crate) const TRACK_GROUP4_INFOS: u32 = 0x0043_d950;

/// `bool SSXAudioSystem_IsTrackGroup4(SSXAudioSystem* self)` (0x2a3fe8):
/// whether the current race's track id (`BENewRaceInt_GetTrackInfo` of the
/// BE library's interface 0) is in the group 0xe..0x10
/// (`SSXAudioSystem_IsTrackIdGroup4`). Frame 0x20 (s0 = self).
pub fn ssx_audio_system_is_track_group4(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x002a_3ffc)?; // BELibrary_GetInstance
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dd58, 0x002a_4008)?; // BELibrary_GetInterface
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_4bc0, 0x002a_4010)?; // BENewRaceInt_GetTrackInfo
        c.g[A1][0] = Addr(r32(c, V0)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x002a_4030,
            f: ee::s08::SSXAudioSystem_IsTrackIdGroup4,
        }
        .call(c, 0x002a_401c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// WorldObjFactory_DeadNode (0x2fc2c0)

/// The event's kind (+4): 0 kill (replace with a `DeadNode`), 1 hide
/// (flags only), 3 restore (replace with a `RestoreNode`).
pub(crate) const DEAD_NODE_EVENT_KIND: u32 = 0x4;
/// The world node: flags at +8, its current object at +0xc.
pub(crate) const DEAD_NODE_FLAGS: u32 = 0x8;
pub(crate) const DEAD_NODE_OBJECT: u32 = 0xc;
/// The object's vtable (+0xc) and type (`s16` at +0x10).
pub(crate) const DEAD_NODE_OBJ_VTABLE: u32 = 0xc;
pub(crate) const DEAD_NODE_OBJ_TYPE: u32 = 0x10;
/// Object types left alone: a dead node (6), a restore node (0x13).
pub(crate) const DEAD_NODE_TYPE_DEAD: u64 = 0x6;
pub(crate) const DEAD_NODE_TYPE_RESTORE: u64 = 0x13;
/// The allocation names of the two replacement nodes.
pub(crate) const DEAD_NODE_NAME: u64 = 0x0048_96b8;
pub(crate) const DEAD_NODE_RESTORE_NAME: u64 = 0x0048_96c8;

/// Delete the node's current object: `obj->vtable[8](obj + delta, 3)`.
fn dead_node_delete_object(c: &mut Cpu, obj: u64, ret: u32) -> crate::call::Flow {
    c.cycles += 6;
    let vt = Addr((obj as u32).wrapping_add(DEAD_NODE_OBJ_VTABLE)).lw(c);
    c.g[V0][0] = vt;
    c.g[A1][0] = 3;
    let slot = crate::call::VSlot::read(c, vt, 8);
    c.g[V1][0] = slot.func;
    c.g[A0][0] = sx((obj as u32).wrapping_add(slot.delta as u32));
    call_ptr(c, slot.func as u32, ret)
}

/// `EA_MemAllocNamed(0x1c, name, 0x20000000, 0)`, then `ctor(block, node)`.
fn dead_node_replace(c: &mut Cpu, name: u64, alloc_ret: u32, ctor: u32, ret: u32) -> crate::call::Flow {
    c.g[A1][0] = name;
    c.g[A0][0] = 0x1c;
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    Callee {
        addr: 0x0031_7d70,
        f: ee::s10::EA_MemAllocNamed,
    }
    .call(c, alloc_ret)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S0][0];
    call_ptr(c, ctor, ret)
}

/// `void WorldObjFactory_DeadNode(WorldObjFactory* node, Event* ev)`
/// (0x2fc2c0): applies a dead-node script event to a world node. Kind 1
/// deletes the node's object (unless it already is a dead node) and folds
/// the flags (`f &= 0xffff0300; f |= f >> 16 (arith) | 2`); kind 0 deletes
/// it and installs a new `DeadNode`; kind 3 deletes it (unless it is a
/// dead or restore node) and installs a `RestoreNode`. Frame 0x20 (s0 =
/// node).
pub fn world_obj_factory_dead_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let kind = Addr(r32(c, A1).wrapping_add(DEAD_NODE_EVENT_KIND)).lw(c);
        c.g[V1][0] = kind;
        c.g[S0][0] = c.g[A0][0];
        let node = r32(c, S0);
        match kind {
            1 => {
                c.cycles += 3;
                let obj = Addr(node.wrapping_add(DEAD_NODE_OBJECT)).lw(c);
                c.g[A2][0] = obj;
                c.g[V0][0] = DEAD_NODE_TYPE_DEAD;
                if obj != 0 {
                    c.cycles += 3;
                    c.g[V1][0] = Addr((obj as u32).wrapping_add(DEAD_NODE_OBJ_TYPE)).lh(c);
                    if c.g[V1][0] == DEAD_NODE_TYPE_DEAD {
                        frame.restore(c, S0, 0x10);
                        return dead_node_leave(c, frame);
                    }
                    dead_node_delete_object(c, obj, 0x002f_c338)?;
                }
                c.cycles += 9;
                let node = r32(c, S0);
                let kept = Addr(node.wrapping_add(DEAD_NODE_FLAGS)).lw(c) & 0xffff_ffff_ffff_0300;
                c.g[V1][0] = 0xffff_ffff_ffff_0300;
                let high = sx(((kept as u32) as i32 >> 16) as u32);
                c.g[A0][0] = high;
                let flags = kept | high | 2;
                c.g[V0][0] = flags;
                Addr(node.wrapping_add(DEAD_NODE_FLAGS)).sw(c, flags);
                c.cycles += 1;
            }
            0 => {
                c.cycles += 3 + 2 + 2;
                let obj = Addr(node.wrapping_add(DEAD_NODE_OBJECT)).lw(c);
                c.g[A2][0] = obj;
                c.g[V0][0] = DEAD_NODE_TYPE_DEAD;
                if obj != 0 {
                    c.cycles += 3;
                    c.g[V1][0] = Addr((obj as u32).wrapping_add(DEAD_NODE_OBJ_TYPE)).lh(c);
                    if c.g[V1][0] == DEAD_NODE_TYPE_DEAD {
                        frame.restore(c, S0, 0x10);
                        return dead_node_leave(c, frame);
                    }
                    dead_node_delete_object(c, obj, 0x002f_c388)?;
                }
                c.cycles += 11;
                dead_node_replace(c, DEAD_NODE_NAME, 0x002f_c3a0, 0x0035_06d8, 0x002f_c3ac)?;
            }
            3 => {
                c.cycles += 3 + 2 + 2;
                let obj = Addr(node.wrapping_add(DEAD_NODE_OBJECT)).lw(c);
                c.g[A2][0] = obj;
                c.g[V0][0] = DEAD_NODE_TYPE_DEAD;
                if obj != 0 {
                    c.cycles += 3;
                    c.g[V1][0] = Addr((obj as u32).wrapping_add(DEAD_NODE_OBJ_TYPE)).lh(c);
                    c.g[V0][0] = DEAD_NODE_TYPE_RESTORE;
                    if c.g[V1][0] == DEAD_NODE_TYPE_DEAD {
                        c.cycles += 1;
                        frame.restore(c, S0, 0x10);
                        return dead_node_leave(c, frame);
                    }
                    c.cycles += 2;
                    if c.g[V1][0] == DEAD_NODE_TYPE_RESTORE {
                        frame.restore(c, S0, 0x10);
                        return dead_node_leave(c, frame);
                    }
                    dead_node_delete_object(c, obj, 0x002f_c3e8)?;
                }
                c.cycles += 9;
                dead_node_replace(
                    c,
                    DEAD_NODE_RESTORE_NAME,
                    0x002f_c400,
                    0x0035_0f60,
                    0x002f_c40c,
                )?;
                c.cycles += 1;
            }
            _ => {
                // Any other kind: nothing to do.
                c.cycles += 3 + 2 + 2;
                c.g[V0][0] = if (kind as i64) < 2 { 1 } else { 3 };
            }
        }
        frame.restore(c, S0, 0x10);
        dead_node_leave(c, frame)
    })
}

/// The shared epilogue (`ld ra; addiu sp, 0x20; jr ra`, 3 cycles).
fn dead_node_leave(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// SND_ChannelApplyTimeStretch (0x3c3010)

/// The SND driver state: the channel table pointer (+0x1f0) and the two
/// voice-number bases (bytes +0x2f, +0x30) subtracted from a channel's
/// voice numbers.
pub(crate) const STRETCH_SND_STATE: u32 = 0x0050_a8e8;
pub(crate) const STRETCH_STATE_CHANNELS: u32 = 0x1f0;
pub(crate) const STRETCH_STATE_BASE_A: u32 = 0x2f;
pub(crate) const STRETCH_STATE_BASE_B: u32 = 0x30;
/// A channel (0x8c bytes): voice numbers (`s16`) from +4, voice count
/// (byte +0x23), flags (u16 +0x24; bit 2: EE voices).
pub(crate) const STRETCH_CHANNEL_SIZE: u64 = 0x8c;
pub(crate) const STRETCH_CHANNEL_VOICES: u32 = 0x4;
pub(crate) const STRETCH_CHANNEL_COUNT: u32 = 0x23;
pub(crate) const STRETCH_CHANNEL_FLAGS: u32 = 0x24;

/// `int SND_ChannelApplyTimeStretch(int channel, int ratio)` (0x3c3010):
/// for a channel of EE voices (flag 4), sets the time-stretch ratio of each
/// of its voices (`SND_EeVoiceSetTimeStretch(voice - (base_a + base_b),
/// ratio)`); returns 0. Frame 0x60 (s0 = voice cursor, s1 = index, s2 =
/// channel, s3 = driver state, s4 = ratio); the channel offset uses `mult`.
pub fn snd_channel_apply_time_stretch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = STRETCH_CHANNEL_SIZE;
        frame.save(c, S2, 0x30);
        c.mult(0, A0, A0, V0);
        frame.save(c, S4, 0x10);
        frame.save(c, S0, 0x50);
        c.g[V1][0] = sx(STRETCH_SND_STATE);
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0);
        c.g[A2][0] = Addr(STRETCH_SND_STATE + STRETCH_STATE_CHANNELS).lw(c);
        c.g[S2][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
        let channel = r32(c, S2);
        c.g[V0][0] = Addr(channel.wrapping_add(STRETCH_CHANNEL_FLAGS)).lhu(c) & 4;
        c.g[S4][0] = c.g[A1][0];
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(channel.wrapping_add(STRETCH_CHANNEL_COUNT)).lbu(c);
            c.g[S1][0] = 0;
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[S3][0] = c.g[V1][0];
                c.g[S0][0] = sx(channel.wrapping_add(STRETCH_CHANNEL_VOICES));
                c.g[V0][0] = Addr(r32(c, S3).wrapping_add(STRETCH_STATE_BASE_A)).lbu(c);
                loop {
                    c.cycles += 12;
                    c.g[A1][0] = c.g[S4][0];
                    let state = r32(c, S3);
                    let base_b = Addr(state.wrapping_add(STRETCH_STATE_BASE_B)).lbu(c);
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                    let cursor = r32(c, S0);
                    c.g[V1][0] = Addr(cursor).lh(c);
                    let base = (base_b as u32).wrapping_add(r32(c, V0));
                    c.g[S0][0] = sx(cursor.wrapping_add(2));
                    c.g[A0][0] = sx(r32(c, V1).wrapping_sub(base));
                    call_ptr(c, 0x003c_8b40, 0x003c_3090)?; // SND_EeVoiceSetTimeStretch
                    let count = Addr(r32(c, S2).wrapping_add(STRETCH_CHANNEL_COUNT)).lbu(c);
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (count as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.g[V0][0] = Addr(r32(c, S3).wrapping_add(STRETCH_STATE_BASE_A)).lbu(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x003c_3070);
                    }
                }
            }
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x50);
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, machine, LOG_RESULTS};
    use crate::difftest::{fill, DATA};

    /// `StreamFile_AllocRequest`: the real mutex lock/unlock (the fake
    /// GetThreadId returns 0x41: the owner, so the fast path, or another
    /// thread, so the semaphore path through fakes); an empty or non-empty
    /// free list; a serial that wraps to 0.
    #[test]
    fn stream_file_alloc_request_matches() {
        covered(
            ee::s14::f_003dfcd8__generated,
            stream_file_alloc_request,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x003d_fcd8, 4);
                c.pc = 0x003d_fcd8;
                let sf = DATA + 0x400 + (r.next_u32() % 0x40) * 4;
                fill(c, &mut r, sf, 0x20);
                let mutex = sf + STREAM_FILE_MUTEX;
                c.bus.write32(mutex + 4, 0x40 + seed % 3);
                c.bus.write32(mutex + 8, 1 + seed % 2);
                c.bus.write32(mutex, r.next_u32() & 0xff);
                for i in 0..4 {
                    c.bus.write32(LOG_RESULTS + 4 * i, 0x41);
                }
                let req = DATA + 0x2000 + (r.next_u32() % 0x40) * 4;
                fill(c, &mut r, req, 8);
                let head = if seed % 4 == 0 { 0 } else { req };
                c.bus.write32(sf + STREAM_FILE_FREE_REQUESTS, head);
                let serial = if seed % 5 == 1 {
                    0xffff_ff00
                } else {
                    r.next_u32()
                };
                c.bus.write32(STREAM_FILE_REQUEST_SERIAL, serial);
                c.g[A0][0] = sx(sf);
            },
        );
    }

    /// `SND_EeVoiceSetLowpass`: cutoffs below 1.0 (positive and negative,
    /// so both rounding signs) with the voice's lowpass DSP present or
    /// created by the real `SND_MemAlloc` from an empty heap and inserted
    /// into a 0-3 node chain; cutoffs of 1.0 or more with a DSP to remove
    /// (the real `SND_DspRemove`, `SND_MemFree` on an empty heap) or none.
    #[test]
    fn snd_ee_voice_set_lowpass_matches() {
        const VOICES: u32 = DATA + 0x20_0000;
        const NODES: u32 = DATA + 0x20_1000;
        const HEAP: u32 = DATA + 0x20_2000;
        crate::difftest::check_lohi(
            ee::s14::f_003c8d00__generated,
            snd_ee_voice_set_lowpass,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c_8d00, 0);
                c.pc = 0x003c_8d00;
                let index = seed % 8;
                c.g[A0][0] = index as u64;
                c.bus.write32(LOWPASS_VOICES, VOICES);
                fill(c, &mut r, VOICES, 0x60 * 8 / 4);
                let voice = VOICES + 0x60 * index;
                // The DSP chain: 0-3 nodes {next +8, priority +0x18}; the
                // lowpass DSP (when present) is one of them or not.
                let n = seed / 8 % 4;
                for i in 0..n {
                    let node = NODES + 0x40 * i;
                    fill(c, &mut r, node, 0x10);
                    c.bus
                        .write32(node + 8, if i + 1 == n { 0 } else { node + 0x40 });
                    c.bus
                        .write16(node + 0x18, (0x10 + 0x20 * i + r.next_u32() % 0x20) as u16);
                }
                c.bus.write32(
                    voice + LOWPASS_VOICE_DSP_CHAIN,
                    if n == 0 { 0 } else { NODES },
                );
                let filter = match seed % 3 {
                    0 => 0,
                    1 if n > 0 => NODES + 0x40 * (r.next_u32() % n),
                    _ => NODES + 0x200,
                };
                c.bus.write32(voice + LOWPASS_VOICE_FILTER, filter);
                // The driver heap: empty, room for the block.
                for state in [0x0051_5b40u32, LOWPASS_SND_STATE] {
                    c.bus.write32(state + 0x234, HEAP);
                }
                c.bus.write32(HEAP, DATA + 0x20_4000);
                c.bus.write32(HEAP + 0x4, DATA + 0x20_3000);
                c.bus.write32(HEAP + 0xc, 0x8_0000);
                c.bus.write32(HEAP + 0x10, 0x100);
                c.bus.write32(HEAP + 0x14, 0);
                c.bus.write16(
                    LOWPASS_SND_STATE + LOWPASS_STATE_RATE,
                    [48000u32, 44100, 24000, r.next_u32() % 0x1_0000][(seed / 3 % 4) as usize] as u16,
                );
                c.f[12] = match seed / 5 % 6 {
                    0 => 0x3f80_0000,
                    1 => r.edge_f32(),
                    2 => r.f32(1.0) | 0x8000_0000,
                    3 => 0x4000_0000 + (r.next_u32() & 0xff_ffff),
                    _ => r.f32(1.0) & 0x7fff_ffff,
                };
            },
        );
    }

    /// `WScriptMan_PostAirEvent`: random op counts (slots at every
    /// alignment mod 8, as the 12-byte stride gives) and random stack
    /// leftovers in the op temporary.
    #[test]
    fn wscript_man_post_air_event_matches() {
        crate::difftest::check_lohi(
            ee::s10::f_003098b0__generated,
            wscript_man_post_air_event,
            300,
            &|c, seed| {
                let mut r = crate::difftest::Rng(0x30_98b0 ^ (seed as u64) << 12);
                crate::difftest::scramble(c, &mut r);
                c.pc = 0x0030_98b0;
                let man = DATA + 0x100 + (r.next_u32() % 0x40) * 4;
                c.g[A0][0] = sx(man);
                let count = seed % 64;
                c.bus.write32(man + POST_AIR_OP_COUNT, count);
                fill(c, &mut r, man + 0x44, 0x100);
                fill(c, &mut r, crate::difftest::SP - 0x40, 0x10);
            },
        );
    }

    /// `cCSICubicSpline_Alloc`: no old array, or an old array from the real
    /// heap (freed by the real `cCSICubicSpline_Free`); point counts 0..300
    /// (the 8-bit count wraps), random flags; the real `operator_new_named`
    /// over a test arena.
    #[test]
    fn c_csi_cubic_spline_alloc_matches() {
        use crate::game::be::beintreward::tests::world;
        use crate::game::bx::bxstring::tests::{alloc, heap};
        use crate::game::bx::bxstring2::tests::be_library;
        crate::difftest::check_lohi(
            ee::s10::f_0031d660__generated,
            c_csi_cubic_spline_alloc,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x31_d660, 0);
                be_library(c);
                world(c);
                heap(c);
                let old = if seed % 3 == 0 {
                    0
                } else {
                    alloc(c, 0x14 * (1 + r.next_u32() % 16))
                };
                let spline = DATA + 0x100 + (r.next_u32() % 0x40) * 4;
                fill(c, &mut r, spline, 8);
                c.bus.write32(spline + SPLINE_ALLOC_DATA, old);
                c.g[A0][0] = sx(spline);
                c.g[A1][0] = match seed % 5 {
                    0 => 0,
                    1 => 1,
                    _ => (r.next_u32() % 300) as u64,
                };
                c.pc = 0x0031_d660;
            },
        );
    }

    /// `SSXAudioSystem_IsTrackGroup4`: the real BE library lookups with the
    /// interface's vtable slot on a fake (unwinding on some seeds), track
    /// ids around the 0xe..0x10 group.
    #[test]
    fn ssx_audio_system_is_track_group4_matches() {
        use crate::difftest::fakes::FAKE;
        use crate::difftest::GAME_GP;
        covered(
            ee::s08::f_002a3fe8__generated,
            ssx_audio_system_is_track_group4,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_3fe8, 1);
                c.pc = 0x002a_3fe8;
                let idx = seed % 4;
                fill(c, &mut r, DATA + 0x400, 0x40);
                c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x400);
                c.bus.write32(GAME_GP.wrapping_sub(0x1efc), idx);
                c.bus.write32(0x0053_08a8 + 4 * idx, DATA + 0x600);
                c.bus.write32(DATA + 0x600 + 4, DATA + 0x700);
                c.bus.write16(DATA + 0x700 + 8, (r.next_u32() as u16) & 0xfc);
                c.bus.write32(DATA + 0x700 + 0xc, FAKE);
                let track = r.next_u32() % 12;
                c.bus.write32(TRACK_GROUP4_TRACK, track);
                c.bus.write32(
                    TRACK_GROUP4_INFOS + 0x64 * track,
                    0xb + seed % 8,
                );
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }

    /// `WorldObjFactory_DeadNode`: every event kind (0, 1, 3, others on both
    /// sides of 2), no object or an object of type 6, 0x13 or other (its
    /// delete through a fake vtable slot), random node flags; the real
    /// `EA_MemAllocNamed` and node constructors as in
    /// `world_obj_replace_with_dead_node_matches`.
    #[test]
    fn world_obj_factory_dead_node_matches() {
        use crate::difftest::check;
        use crate::difftest::fakes::vtable;
        use crate::game::fe::ovtemplatepausemenu::tests::setup;
        const NODE: u32 = DATA + 0x800;
        const OBJ: u32 = DATA + 0xa00;
        const EVENT: u32 = DATA + 0xc00;
        check(
            ee::s10::f_002fc2c0__generated,
            world_obj_factory_dead_node,
            240,
            &|c, seed| {
                let mut r = setup(c, seed, 0x2f_c2c0);
                c.pc = 0x002f_c2c0;
                let kind: u32 = match seed % 7 {
                    0 | 1 => 0,
                    2 | 3 => 1,
                    4 | 5 => 3,
                    _ => [2, 0xffff_ffff, 4, 0x80][(seed / 7 % 4) as usize],
                };
                c.bus.write32(EVENT + DEAD_NODE_EVENT_KIND, kind);
                fill(c, &mut r, NODE, 8);
                let obj = if seed / 7 % 4 == 0 { 0 } else { OBJ };
                c.bus.write32(NODE + DEAD_NODE_OBJECT, obj);
                fill(c, &mut r, OBJ, 8);
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(OBJ + DEAD_NODE_OBJ_VTABLE, vt);
                let ty = match seed / 3 % 4 {
                    0 => 6,
                    1 => 0x13,
                    _ => r.next_u32() as u16 & 0x1f,
                };
                c.bus.write16(OBJ + DEAD_NODE_OBJ_TYPE, ty);
                c.g[A0][0] = sx(NODE);
                c.g[A1][0] = sx(EVENT);
            },
        );
    }

    /// `SND_ChannelApplyTimeStretch`: channels with and without the EE-voice
    /// flag, 0-6 voices whose numbers map (after the two bases) onto voices
    /// 0-7 of a zeroed voice table (no stretch DSP: the real
    /// `SND_EeVoiceSetTimeStretch` returns at once).
    #[test]
    fn snd_channel_apply_time_stretch_matches() {
        const CHANNELS: u32 = DATA + 0x1000;
        const VOICES: u32 = DATA + 0x20_0000;
        crate::difftest::check_lohi(
            ee::s13::f_003c3010__generated,
            snd_channel_apply_time_stretch,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c_3010, 0);
                c.pc = 0x003c_3010;
                let index = seed % 8;
                c.g[A0][0] = index as u64;
                c.g[A1][0] = sx(r.next_u32());
                c.bus.write32(STRETCH_SND_STATE + STRETCH_STATE_CHANNELS, CHANNELS);
                fill(c, &mut r, CHANNELS, (0x8c * 8) / 4);
                let base_a = r.next_u32() % 0x40;
                let base_b = r.next_u32() % 0x40;
                c.bus.write8(STRETCH_SND_STATE + STRETCH_STATE_BASE_A, base_a as u8);
                c.bus.write8(STRETCH_SND_STATE + STRETCH_STATE_BASE_B, base_b as u8);
                let ch = CHANNELS + 0x8c * index;
                let flags = r.next_u32() as u16;
                c.bus.write16(
                    ch + STRETCH_CHANNEL_FLAGS,
                    if seed % 4 == 0 { flags & !4 } else { flags | 4 },
                );
                let count = seed / 4 % 7;
                c.bus.write8(ch + STRETCH_CHANNEL_COUNT, count as u8);
                for i in 0..count {
                    c.bus.write16(
                        ch + STRETCH_CHANNEL_VOICES + 2 * i,
                        (base_a + base_b + r.next_u32() % 8) as u16,
                    );
                }
                c.bus.write32(0x0051_5d1c, VOICES);
                for i in 0..0x60 * 8 / 4 {
                    c.bus.write32(VOICES + 4 * i, 0);
                }
            },
        );
    }

    /// `SpeechInstance_PlayEvent`: the event id's own bank matching or a
    /// scan over 0-8 banks (some matching), no event context or a matching
    /// one (the real `SpeechSystem_ApplyEventContext`), the real path
    /// `sprintf` over the game image, the player's vtable slots on fakes
    /// (some failing, some unwinding), and the real stream array calls over
    /// a stream slot that is empty, idle, busy or playing (the SND lock
    /// hooks on fakes, the SNDSTRM stream and its current request in test
    /// RAM).
    #[test]
    fn speech_instance_play_event_matches() {
        use crate::difftest::elf_image;
        use crate::difftest::fakes::FAKE;
        use crate::difftest::GAME_GP;
        const SYSTEM: u32 = DATA + 0x1000;
        const INSTANCES: u32 = DATA + 0x1800;
        const STREAMS: u32 = DATA + 0x3000;
        const SLOTS: u32 = DATA + 0x3100;
        const BUFFER: u32 = DATA + 0x3800;
        const INST: u32 = DATA + 0x4000;
        const BANKS: u32 = DATA + 0x5000;
        const EVENT: u32 = DATA + 0x6000;
        const STREAM: u32 = DATA + 0x7000;
        const REQUEST: u32 = DATA + 0x7400;
        const MONITOR: u32 = DATA + 0xa000;
        covered(
            ee::s08::f_002b04d8__generated,
            speech_instance_play_event,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2b_04d8, 8);
                elf_image(c);
                c.pc = 0x002b_04d8;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                // SND_Lock / SND_Unlock through fakes.
                w(c, 0x0044_c46c, FAKE + 0x100);
                w(c, 0x0044_c470, FAKE + 0x110);
                // The speech system.
                w(c, GAME_GP + SPEECH_PLAY_SYSTEM_GP, SYSTEM);
                for i in 0..0x200 {
                    w(c, SYSTEM + 4 * i, 0);
                }
                w(c, SYSTEM, STREAMS);
                w(c, SYSTEM + 4, INSTANCES);
                for k in 0..5u32 {
                    let at = SYSTEM + SPEECH_PLAY_LANG_PREFIXES as u32 + 0x14 * k;
                    for (i, b) in b"lang0/\0".iter().enumerate() {
                        c.bus.write8(at + i as u32, if i == 4 { b'0' + k as u8 } else { *b });
                    }
                }
                // The player's vtable: slot 8 (start) on a fake or on a real
                // getter returning `*(system + 0x414)` (to reach a failed,
                // negative start), slot 0x10 (stream handle) on a fake, slot
                // 0x18 (volume scale pointer) on a fake or on a real leaf
                // returning 0.
                const VT: u32 = DATA + 0x9000;
                c.bus.write16(VT + 8, 0x400);
                w(c, VT + 0xc, if seed % 2 == 0 { 0x0016_94b0 } else { FAKE });
                c.bus.write16(VT + 0x10, (r.next_u32() & 0xfc) as u16);
                w(c, VT + 0x14, FAKE + 0x10);
                c.bus.write16(VT + 0x18, (r.next_u32() & 0xfc) as u16);
                w(c, VT + 0x1c, if seed % 4 == 1 { 0x0014_06c0 } else { FAKE + 0x20 });
                w(c, SYSTEM + SPEECH_PLAY_PLAYER_VTABLE, VT);
                w(
                    c,
                    SYSTEM + 0x414,
                    if seed % 3 == 1 { 0x8000_0000 | r.next_u32() } else { r.next_u32() & 0xffff },
                );
                // The instance and its banks.
                let id = seed % 4;
                for i in 0..4 {
                    w(c, INSTANCES + 4 * i, INST);
                }
                fill(c, &mut r, INST, 0x240);
                w(c, INST, id);
                let slot = r.next_u32() % 4;
                w(c, INST + SPEECH_PLAY_STREAM, slot);
                let count = match seed / 4 % 5 {
                    0 => 0,
                    1 => 0xffff_ffff,
                    _ => 1 + r.next_u32() % 8,
                };
                w(c, INST + SPEECH_PLAY_BANK_COUNT, count);
                let names: [&[u8]; 6] = [
                    b"bank_eng", b"bank_fre", b"bnk_ger", b"x_ita", b"bank", b"s_spa",
                ];
                for k in 0..8u32 {
                    let bank = BANKS + 0x80 * k;
                    w(c, INST + SPEECH_PLAY_BANKS + 4 * k, bank);
                    w(c, bank, r.next_u32() % 10);
                    let name = names[(r.next_u32() % 6) as usize];
                    for (i, b) in name.iter().chain(b"\0").enumerate() {
                        c.bus.write8(bank + 4 + i as u32, *b);
                    }
                    w(c, bank + 0x40, r.next_u32());
                }
                // The event.
                fill(c, &mut r, EVENT, 8);
                let event_id = r.next_u32() % 8;
                w(c, EVENT, event_id);
                if seed % 3 == 0 {
                    w(c, BANKS + 0x80 * event_id, event_id);
                }
                let key = r.next_u32() as u16;
                c.bus.write16(EVENT + SPEECH_PLAY_EVENT_KEY, key);
                // An event context (10 slots of 0x20 from +0x18).
                if seed % 5 != 0 {
                    let ctx = SYSTEM + 0x20 * (r.next_u32() % 10);
                    w(c, ctx + 0x18, 1);
                    w(c, ctx + 0x20, id);
                    w(c, ctx + 0x24, key as u32);
                    w(c, ctx + 0x2c, r.next_u32());
                    w(c, ctx + 0x30, r.f32(2.0));
                    w(c, ctx + 0x34, if seed % 2 == 0 { 0 } else { r.next_u32() });
                }
                // The stream array, its slot's buffer, the SNDSTRM stream.
                for i in 0..0x400 {
                    w(c, MONITOR + 4 * i, 0);
                }
                w(c, STREAMS, MONITOR);
                w(c, STREAMS + 8, SLOTS);
                for i in 0..4 {
                    w(c, SLOTS + 4 * i, if seed / 2 % 7 == 0 { 0 } else { BUFFER });
                }
                let handle = r.next_u32() % 8;
                w(c, BUFFER + 8, handle);
                c.bus.write8(0x0050_a92e, 8);
                for i in 0..8 {
                    w(c, 0x0050_a0a0 + 4 * i, STREAM);
                }
                for i in 0..0x80 {
                    w(c, STREAM + 4 * i, 0);
                }
                w(c, STREAM + 0x124, if seed / 3 % 3 == 0 { 0 } else { 1 + r.next_u32() % 4 });
                w(c, STREAM + 0x11c, REQUEST);
                c.bus.write16(STREAM + 0x18, r.next_u32() as u16);
                fill(c, &mut r, REQUEST, 0x10);
                w(c, REQUEST, 0);
                w(c, REQUEST + 0xc, (r.next_u32() & 0x7fff_ff00) | handle);
                c.bus.write8(REQUEST + 0x24, (seed / 7 % 2) as u8);
                c.g[A0][0] = sx(INST);
                c.g[A1][0] = sx(EVENT);
            },
        );
    }
}
