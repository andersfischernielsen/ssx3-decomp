//! Work package D7: the audio system (`docs/investigation/phase2-candidates-2.md`,
//! names from `docs/investigation/naming.md` pass 3).
//!
//! Two layers of the game's audio:
//!
//! - the EA sound driver (`SND_*`), which runs on the sound EE thread
//!   (`SND_EeThread` → [`snd_service`] → `SND_Tick`) and on game threads
//!   behind `SND_Lock`: per-handle pitch and pan, the channel pitch cache,
//!   the iterator over the voice channels of a handle, and the recursive
//!   mutex behind the default lock hooks;
//! - SSX 3's `SSXAudioSystem` on the main thread: the per-frame update of
//!   banks, streams, the 64 `BankMonitor` instances (one per playing sound
//!   effect) and the 48 timers, distance falloff, music and a few getters.
//!
//! Every replacement charges the EE cycles block by block along the path it
//! takes, makes the same calls with the same registers (the call protocol
//! of [`crate::call`]), checks events at the same back edges (the shapes in
//! [`crate::events`]) and leaves the same incidental register values (lines
//! marked "leftover"). The driver's state block and channel layout come
//! from [`crate::sound::snd`] (package A5).

use crate::abi::{self, addu, mult, reg, set_reg};
use crate::call::{call_ptr, guest, tail_wrapper, Callee, Flow, Frame, Target, Unwound, VSlot};
use crate::events::{back_edge_unwinds, deliver_event, event_due, Resume};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use crate::sound::snd;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this package: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0028_b1c8, ssx_audio_get_game_module, "SSXAudioSystem_GetGameModule"),
    (0x0028_b1d8, ssx_audio_get_race_man, "SSXAudioSystem_GetRaceMan"),
    (0x0028_7968, ssx_audio_get_channel_gain_ptr, "SSXAudioSystem_GetChannelGainPtr"),
    (0x0028_8ae0, ssx_audio_is_replay_playing, "SSXAudioSystem_IsReplayPlaying"),
    (0x002a_bc18, bank_monitor_calc_falloff_volume, "BankMonitor_CalcFalloffVolume"),
    (0x002a_bc80, bank_monitor_instance_get_falloff_volume, "BankMonitorInstance_GetFalloffVolume"),
    (0x002a_bb80, bank_monitor_instance_poll_free, "BankMonitorInstance_PollFree"),
    (0x002a_d550, bank_monitor_find_instance, "BankMonitor_FindInstance"),
    (0x002a_be78, bank_monitor_instance_set_pitch, "BankMonitorInstance_SetPitch"),
    (0x002a_d998, bank_monitor_snd_tick_callback, "BankMonitor_SndTickCallback"),
    (0x002b_4388, music_player_update, "MusicPlayer_Update"),
    (0x002b_2488, song_instance_get_stream_handle, "SongInstance_GetStreamHandle"),
    (0x002a_b958, ssx_audio_update_banks_and_streams, "SSXAudioSystem_UpdateBanksAndStreams"),
    (0x002a_b7a0, ssx_audio_update_bank_monitor, "SSXAudioSystem_UpdateBankMonitor"),
    (0x002a_b6b0, ssx_audio_update_bank_monitor_instance, "SSXAudioSystem_UpdateBankMonitorInstance"),
    (0x002a_b8e8, ssx_audio_update_timers, "SSXAudioSystem_UpdateTimers"),
    (0x003b_aa00, snd_next_handle_channel, "SND_NextHandleChannel"),
    (0x003b_a938, snd_channel_calc_pitch, "SND_ChannelCalcPitch"),
    (0x003b_8790, snd_set_pitch, "SND_SetPitch"),
    (0x003b_8c00, snd_set_pan, "SND_SetPan"),
    (0x003b_5860, snd_default_lock, "SND_DefaultLock"),
    (0x003b_5880, snd_default_unlock, "SND_DefaultUnlock"),
    (0x003c_3380, snd_mutex_lock, "SND_MutexLock"),
    (0x003c_33e0, snd_mutex_unlock, "SND_MutexUnlock"),
    (0x003b_5540, snd_service, "SND_Service"),
];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const SND_HANDLE_TO_CHANNEL: Callee = Callee { addr: 0x003b_a6e8, f: ee::s13::SND_HandleToChannel };
    pub const SND_CHANNEL_CALC_PITCH: Callee = Callee { addr: 0x003b_a938, f: ee::s13::SND_ChannelCalcPitch };
    pub const SND_CHANNEL_APPLY_PITCH: Callee = Callee { addr: 0x003c_25e0, f: ee::s13::SND_ChannelApplyPitch };
    pub const SND_CHANNEL_APPLY_PAN: Callee = Callee { addr: 0x003c_38e0, f: ee::s13::SND_ChannelApplyPan };
    pub const SND_NEXT_HANDLE_CHANNEL: Callee = Callee { addr: 0x003b_aa00, f: ee::s13::SND_NextHandleChannel };
    pub const SND_CENTS_TO_PITCH: Callee = Callee { addr: 0x003b_a888, f: ee::s13::SND_CentsToPitch };
    pub const SND_MUTEX_LOCK: Callee = Callee { addr: 0x003c_3380, f: ee::s13::SND_MutexLock };
    pub const SND_MUTEX_UNLOCK: Callee = Callee { addr: 0x003c_33e0, f: ee::s13::SND_MutexUnlock };
    pub const SND_TICK: Callee = Callee { addr: 0x003b_55f0, f: ee::s13::SND_Tick };

    pub const CALC_FALLOFF_VOLUME: Callee =
        Callee { addr: 0x002a_bc18, f: ee::s08::BankMonitor_CalcFalloffVolume };
    pub const CROWD_AUDIO_UPDATE: Callee = Callee { addr: 0x002a_6d18, f: ee::s08::CrowdAudio_Update };
    pub const MUSIC_PLAYER_UPDATE: Callee = Callee { addr: 0x002b_4388, f: ee::s08::MusicPlayer_Update };
    pub const SPEECH_SYSTEM_UPDATE: Callee = Callee { addr: 0x002b_0c78, f: ee::s08::SpeechSystem_Update };
    pub const STREAM_ARRAY_UPDATE_ALL: Callee = Callee { addr: 0x002a_ac28, f: ee::s08::StreamArray_UpdateAll };
    pub const UPDATE_BANK_MONITOR_INSTANCE: Callee =
        Callee { addr: 0x002a_b6b0, f: ee::s08::SSXAudioSystem_UpdateBankMonitorInstance };
    pub const BMI_POLL_FREE: Callee = Callee { addr: 0x002a_bb80, f: ee::s08::BankMonitorInstance_PollFree };
    pub const BMI_UPDATE_3D: Callee = Callee { addr: 0x002a_c868, f: ee::s08::BankMonitorInstance_Update3D };
    pub const BMI_IS_DONE: Callee = Callee { addr: 0x002a_ba40, f: ee::s08::BankMonitorInstance_IsDone };
    pub const BMI_RESET: Callee = Callee { addr: 0x002a_bb38, f: ee::s08::BankMonitorInstance_Reset };
    pub const BANK_MONITOR_TICK: Callee = Callee { addr: 0x002a_dc10, f: ee::s08::BankMonitor_Tick };
    pub const BANK_MONITOR_TIMER_UPDATE: Callee =
        Callee { addr: 0x002a_b828, f: ee::s08::BankMonitor_TimerUpdate };
    pub const STREAM_BUFFER_SET_PITCH: Callee = Callee { addr: 0x002a_9f38, f: ee::s08::StreamBuffer_SetPitch };
    pub const SONG_INSTANCE_UPDATE: Callee = Callee { addr: 0x002b_21e0, f: ee::s08::SongInstance_Update };
    pub const MUSIC_PLAYER_ATTACH_LOOP_BANK: Callee =
        Callee { addr: 0x002b_4620, f: ee::s08::MusicPlayer_AttachLoopBank };
    pub const PATH_GET_TRACK_DATA: Callee = Callee { addr: 0x002a_dfb0, f: ee::s08::PATH_GetTrackData };

    pub const IS_IN_GAME: Callee = Callee { addr: 0x0028_b1b0, f: ee::s07::SSXAudioSystem_IsInGame };
    pub const GET_GAME_MODULE: Callee = Callee { addr: 0x0028_b1c8, f: ee::s07::SSXAudioSystem_GetGameModule };
}

/// Cross-shard calls the compiler emitted as `call_indirect` (function table).
mod table_callee {
    /// Kernel syscall wrappers.
    pub const GET_THREAD_ID: u32 = 0x0042_3c90;
    pub const WAIT_SEMA: u32 = 0x0042_3de0;
    pub const SIGNAL_SEMA: u32 = 0x0042_3dc0;
    pub const SND_LOCK: u32 = 0x003b_58a0;
    pub const SND_UNLOCK: u32 = 0x003b_58d8;
    pub const SND_SET_PITCH: u32 = 0x003b_8790;
    pub const SNDMIDI_SET_PITCH: u32 = 0x003b_9b68;
    pub const SND_LOAD_QUEUE_UPDATE: u32 = 0x0028_aaf8;
    pub const PATH_GET_TRACK_STREAM: u32 = 0x003d_3030;
}

// ---------------------------------------------------------------------------
// Views.

/// More fields of the sound driver's state block ([`snd::STATE`]), as
/// offsets from it.
mod snd_state {
    /// `s8`: the initialised flag (= [`super::snd::INITIALIZED`]).
    pub const INITIALIZED: u32 = 0x174;
    /// `s16`: number of voice channels.
    pub const CHANNEL_COUNT: u32 = 0x17e;
    /// `u32`: driver ticks run so far (`SND_Tick` increments it).
    pub const TICKS: u32 = 0x180;
    /// `SndChannel*`: the channel array.
    pub const CHANNELS: u32 = 0x1f0;
    /// `u32`: `SND_Service` calls since the rate last changed.
    pub const SERVICE_CALLS: u32 = 0x330;
    /// `s16`: the service rate (calls per second) the counters are for.
    pub const SERVICE_RATE: u32 = 0x334;
}

/// Voice channel fields (0x8c bytes; more in [`snd`]).
mod chan {
    /// Final pan (`s16`): patch pan + the handle's pan offset.
    pub const PAN: u32 = 0x1c;
    /// Stereo spread (`s16`, -0x4000..0x3fff).
    pub const SPREAD: u32 = 0x1e;
    /// Voice group (`u8`): channels of one multi-voice handle share it; 0
    /// for a single-voice handle.
    pub const GROUP: u32 = 0x26;
    /// The patch's own pan (`s16`).
    pub const PATCH_PAN: u32 = 0x4c;
    /// Pitch-bend wheel position (`s8`, centre 0x40).
    pub const BEND: u32 = 0x64;
    /// Modulation controller index (`u8`).
    pub const MOD_INDEX: u32 = 0x68;
    /// Allocated flag (`s8`, 1 when allocated).
    pub const ALLOCATED: u32 = 0x69;
    /// Bend remap table (`s8[]`), or null.
    pub const BEND_TABLE: u32 = 0x74;
    /// Modulation table (`s8[]`), or null.
    pub const MOD_TABLE: u32 = 0x7c;
    /// Modulation depth in cents (`s16`).
    pub const MOD_DEPTH: u32 = 0x80;
    /// Bend range in cents (`s16`).
    pub const BEND_RANGE: u32 = 0x82;
    /// The note's pitch in cents (`s16`).
    pub const CENTS: u32 = 0x84;
    /// Cached pitch for `CENTS` + bend + modulation (`u16`, 0 = stale).
    pub const PITCH_CACHE: u32 = 0x86;
    /// The handle's requested pitch scale (`u16`, 0x1000 = unity).
    pub const REQUESTED_PITCH: u32 = 0x88;
    /// Final pitch (`u16`): cache * requested >> 12.
    pub const FINAL_PITCH: u32 = 0x8a;
}

/// The driver's recursive mutex (`SND_MutexLock`), addressed as
/// `0x510000 - 0x5300 + off`.
mod snd_mutex {
    /// Base the code addresses the fields from.
    pub const BASE: u32 = 0x0050_ad00;
    /// The semaphore id.
    pub const SEMA: u32 = 0x1b10;
    /// Owning thread id, -1 when free.
    pub const OWNER: u32 = 0x1b14;
    /// Nesting count.
    pub const COUNT: u32 = 0x1b18;
}

/// `BankMonitorInstance` (0xc0 bytes): one playing sound effect.
mod bmi {
    /// Size of one instance.
    pub const SIZE: u32 = 0xc0;
    /// What plays (`int`): 0 free, 1 an SND handle, 2 a MIDI player,
    /// 3 a stream buffer.
    pub const STATE: u32 = 0x00;
    /// The sound id it was started for / the SND or MIDI handle.
    pub const ID: u32 = 0x04;
    /// The stream buffer (state 3).
    pub const STREAM: u32 = 0x0c;
    /// Stopping flag: reset once the voice is done.
    pub const STOPPING: u32 = 0x9c;
    /// Per-frame callback: a gcc 2.9x pointer to member function of
    /// `SSXAudioSystem` (delta +0xa8, index +0xaa, function / vptr offset
    /// +0xac).
    pub const CALLBACK: u32 = 0xa8;
    /// The last pitch sent (`int`, 0..0x3fff).
    pub const PITCH: u32 = 0xb0;
}

/// `BankMonitor` (the manager of the instances).
mod bank_monitor {
    /// `BankMonitorInstance*`: the 64 instances.
    pub const INSTANCES: u32 = 0x8ec;
    pub const INSTANCE_COUNT: i64 = 64;
}

/// `SSXAudioSystem` and its sound root (`*(sys + ROOT)`, itself a pointer to
/// the bank / stream object `snd`).
mod ssx_audio {
    /// `Snd**`: the sound root.
    pub const ROOT: u32 = 0x118;
    /// The speech system.
    pub const SPEECH: u32 = 0x5560;
    /// Channel gains: one vec4 per kind, and ten per-index gains for kind 4.
    pub const GAINS: u32 = 0x62c8;
    pub const INDEXED_GAINS: u32 = 0x636c;
    /// `snd` (`*root`) fields.
    pub const SND_BANK_MONITOR: u32 = 0x1d8;
    pub const SND_TIMERS_DISABLED: u32 = 0x26c;
    pub const SND_TIMERS: u32 = 0x270;
    pub const TIMER_SIZE: u32 = 0x2c;
    pub const SND_MANAGER: u32 = 0xab0;
    pub const SND_PAUSED: u32 = 0xac0;
    pub const SND_INSTANCES: u32 = 0xac4;
    /// gp offset of the global audio system (`BankMonitor_SndTickCallback`).
    pub const GLOBAL_GP: u32 = 0x688;
    /// The tick callback's object inside it (vtable at +0).
    pub const TICK_LISTENER: u32 = 0x8d8;
    /// gp offset of the game pointer holding the game module (+0x84).
    pub const GAME_GP: u32 = 0x848;
}

// ---------------------------------------------------------------------------
// Small helpers.

/// `lw` through the CPU path (sign-extended).
#[inline(always)]
fn lw(c: &mut Cpu, a: u64) -> u64 {
    Addr(a as u32).lw(c)
}

/// `lw base+off`.
#[inline(always)]
fn lw_at(c: &mut Cpu, base: u64, off: u32) -> u64 {
    Addr((base as u32).wrapping_add(off)).lw(c)
}

/// `swc1 freg, off(sp)` (an FPU callee-saved register in the frame).
#[inline]
fn save_fpr(c: &mut Cpu, frame: Frame, freg: usize, off: u32) {
    let a = frame.at(c, off);
    c.swc1(freg, a);
}

/// `lwc1 freg, off(sp)`.
#[inline]
fn restore_fpr(c: &mut Cpu, frame: Frame, freg: usize, off: u32) {
    let a = frame.at(c, off);
    c.lwc1(freg, a);
}

/// `slt`/`slti` on whole registers.
#[inline(always)]
fn slt(a: u64, b: u64) -> u64 {
    ((a as i64) < (b as i64)) as u64
}

/// `sra` of the low word.
#[inline(always)]
fn sra(v: u64, n: u32) -> u64 {
    sx(((v as u32) as i32 >> n) as u32)
}

// ---------------------------------------------------------------------------
// SSXAudioSystem getters.

/// `SSXAudioSystem_GetGameModule()` 0x28b1c8: the game module
/// (`game->module`, `game = *(gp-0x848)`). 3 cycles.
pub fn ssx_audio_get_game_module(c: &mut Cpu) {
    c.cycles += 3;
    let game = lw(c, (reg(c, GP) as u32).wrapping_sub(ssx_audio::GAME_GP) as u64);
    set_reg(c, V1, game); // leftover
    let module = lw_at(c, game, 0x84);
    set_reg(c, V0, module);
    abi::ret(c);
}

/// `SSXAudioSystem_GetRaceMan()` 0x28b1d8: the race manager of the game
/// module (`module+0xc`). 4 cycles.
pub fn ssx_audio_get_race_man(c: &mut Cpu) {
    c.cycles += 4;
    let game = lw(c, (reg(c, GP) as u32).wrapping_sub(ssx_audio::GAME_GP) as u64);
    let module = lw_at(c, game, 0x84);
    set_reg(c, V1, module); // leftover
    let race_man = lw_at(c, module, 0xc);
    set_reg(c, V0, race_man);
    abi::ret(c);
}

/// `SSXAudioSystem_GetChannelGainPtr(sys, kind, idx)` 0x287968: the gain
/// slot for a channel kind: `&sys->indexedGains[idx]` for kind 4 with
/// `idx < 10` (signed), else `&sys->gains[kind]` (16 bytes each).
pub fn ssx_audio_get_channel_gain_ptr(c: &mut Cpu) {
    let (sys, kind, idx) = (reg(c, A0), reg(c, A1), reg(c, A2));
    c.cycles += 3;
    if kind == 4 {
        c.cycles += 3;
        if (idx as i64) < 10 {
            c.cycles += 3;
            let off = sx(((idx as u32) << 2).wrapping_add(ssx_audio::INDEXED_GAINS));
            set_reg(c, V0, addu(sys, off));
            abi::ret(c);
            return;
        }
        c.cycles += 1;
    }
    c.cycles += 3;
    let v0 = addu(sx((kind as u32) << 4), sys);
    set_reg(c, V0, sx((v0 as u32).wrapping_add(ssx_audio::GAINS)));
    abi::ret(c);
}

/// `SSXAudioSystem_IsReplayPlaying()` 0x288ae0: in game, the game module
/// has a replay object, and its state is a playing one (`state != 0 &&
/// state < 10`, signed). `GetGameModule` is called twice, as in the
/// original.
pub fn ssx_audio_is_replay_playing(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::IS_IN_GAME.call(c, 0x0028_8af0)?;
        let in_game = reg(c, V0) != 0;
        set_reg(c, V0, 0);
        if in_game {
            c.cycles += 5;
            callee::GET_GAME_MODULE.call(c, 0x0028_8b00)?;
            let replay = lw_at(c, reg(c, V0), 0x28);
            set_reg(c, V1, replay);
            set_reg(c, V0, 0);
            if replay != 0 {
                c.cycles += 6;
                callee::GET_GAME_MODULE.call(c, 0x0028_8b14)?;
                let replay = lw_at(c, reg(c, V0), 0x28);
                let state = lw(c, replay);
                set_reg(c, V1, state);
                let mut playing = 0;
                if state != 0 {
                    c.cycles += 1;
                    playing = slt(state, 10);
                }
                set_reg(c, A0, playing); // leftover
                c.cycles += 2;
                set_reg(c, V0, 1);
                if playing == 0 {
                    c.cycles += 1;
                    set_reg(c, V0, 0);
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BankMonitor.

/// `BankMonitor_CalcFalloffVolume(dist f12, radius f13, vol a0)` 0x2abc18:
/// the volume of a 3D sound at `dist` from the listener. Within half a
/// unit it is full; beyond that it falls off quadratically to 0 at
/// `radius`: `(int)(((r - d) / r)^2 * vol)` with `d = max(dist - 0.5, 0)`,
/// and 0 when `d >= r` or `r <= 0`.
pub fn bank_monitor_calc_falloff_volume(c: &mut Cpu) {
    let radius = abi::farg(c, 1);
    let vol = abi::arg(c, 0);
    c.cycles += 8;
    let half = FpuF32(0x3f00_0000);
    set_reg(c, AT, half.0 as u64); // leftover
    let zero = FpuF32::ZERO;
    abi::set_fpr(c, 0, half);
    abi::set_fpr(c, 1, zero);
    let mut d = abi::farg(c, 0) - half;
    if abi::c_lt_s(c, d, zero) {
        d = zero;
    }
    abi::set_fpr(c, 12, d); // leftover
    c.cycles += 4;
    set_reg(c, V0, 0);
    if abi::c_lt_s(c, d, radius) {
        c.cycles += 3;
        if abi::c_lt_s(c, zero, radius) {
            c.cycles += 8;
            let fvol = FpuF32::from_i32(vol as i32);
            let t = (radius - d).div(radius);
            let v = t * t * fvol;
            abi::set_fpr(c, 0, v); // leftover
            let iv = v.to_i32() as u32;
            abi::set_fpr(c, 1, FpuF32(iv)); // leftover
            set_reg(c, V0, sx(iv));
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `BankMonitorInstance_GetFalloffVolume(dist, radius, inst, vol)` 0x2abc80:
/// forwards to [`bank_monitor_calc_falloff_volume`] with `vol` (a1) as a0.
pub fn bank_monitor_instance_get_falloff_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        set_reg(c, A0, reg(c, A1));
        callee::CALC_FALLOFF_VOLUME.call(c, 0x002a_bc90)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BankMonitorInstance_PollFree(inst)` 0x2abb80: is the instance free? A
/// stopping instance whose voice is done is reset here (and counts as
/// free).
pub fn bank_monitor_instance_poll_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let inst = reg(c, A0);
        set_reg(c, S0, inst);
        let state = lw_at(c, inst, bmi::STATE);
        if state == 0 {
            c.cycles += 2;
            set_reg(c, V0, 1);
        } else {
            let stopping = lw_at(c, inst, bmi::STOPPING);
            c.cycles += 2;
            set_reg(c, V0, 0);
            if stopping != 0 {
                c.cycles += 4;
                set_reg(c, A0, inst);
                callee::BMI_IS_DONE.call(c, 0x002a_bbb4)?;
                let done = reg(c, V0) != 0;
                set_reg(c, V0, 0);
                if done {
                    c.cycles += 3;
                    set_reg(c, A0, inst);
                    callee::BMI_RESET.call(c, 0x002a_bbc4)?;
                    set_reg(c, V0, 1);
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

/// `BankMonitor_FindInstance(bm, id, kind)` 0x2ad550: the first busy
/// instance playing `kind`/`id`, or null. Every instance up to the match is
/// polled first ([`bank_monitor_instance_poll_free`]), which resets
/// finished ones on the way.
pub fn bank_monitor_find_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        let (bm, id, kind) = (reg(c, A0), reg(c, A1), reg(c, A2));
        set_reg(c, S1, bm);
        set_reg(c, S3, kind);
        set_reg(c, S4, id);
        let mut offset = 0u64; // s0
        let mut polled = 0u64; // s2
        set_reg(c, S0, offset);
        set_reg(c, S2, polled);
        let found = loop {
            c.cycles += 5;
            let inst = addu(lw_at(c, bm, bank_monitor::INSTANCES), offset);
            set_reg(c, A0, inst);
            callee::BMI_POLL_FREE.call(c, 0x002a_d58c)?;
            let free = reg(c, V0) != 0;
            polled = sx((polled as u32).wrapping_add(1));
            set_reg(c, S2, polled);
            let mut checked_id = false;
            if !free {
                c.cycles += 5;
                let base = lw_at(c, bm, bank_monitor::INSTANCES);
                set_reg(c, V0, base);
                let inst = addu(offset, base);
                set_reg(c, A0, inst);
                let k = lw_at(c, inst, bmi::STATE);
                set_reg(c, V1, k);
                if k == kind {
                    c.cycles += 3;
                    let i = lw_at(c, inst, bmi::ID);
                    set_reg(c, V0, slt(polled, bank_monitor::INSTANCE_COUNT as u64));
                    if i == id {
                        c.cycles += 2;
                        break inst;
                    }
                    checked_id = true;
                }
            }
            if !checked_id {
                c.cycles += 1;
                set_reg(c, V0, slt(polled, bank_monitor::INSTANCE_COUNT as u64));
            }
            c.cycles += 2;
            let more = reg(c, V0) != 0;
            offset = sx((offset as u32).wrapping_add(bmi::SIZE));
            set_reg(c, S0, offset);
            if !more {
                c.cycles += 1;
                break 0;
            }
            if event_due(c) {
                deliver_event(c, 0x002a_d580);
            }
        };
        set_reg(c, V0, found);
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BankMonitorInstance_SetPitch(inst, pitch)` 0x2abe78: clamp the pitch to
/// 0..0x3fff and, when it moved by more than 0xff from the last one sent,
/// remember it and pass it on to whatever plays: the SND handle or MIDI
/// player (under `SND_Lock`), or the stream buffer.
pub fn bank_monitor_instance_set_pitch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        let requested = reg(c, A1);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        let mut pitch = if slt(requested, 0x4000) != 0 { requested } else { 0x3fff };
        let inst = reg(c, A0);
        set_reg(c, S0, inst);
        let positive = slt(u64::MAX, pitch);
        let last = lw_at(c, inst, bmi::PITCH);
        if positive == 0 {
            pitch = 0;
        }
        set_reg(c, S1, pitch);
        set_reg(c, V1, positive); // leftover
        // |pitch - last| <= 0xff, as one unsigned compare.
        let moved = sx((pitch as u32).wrapping_sub(last as u32).wrapping_add(0xff));
        let small = moved < 0x1ff;
        set_reg(c, V0, small as u64);
        if !small {
            c.cycles += 4;
            let state = lw_at(c, inst, bmi::STATE);
            set_reg(c, V1, state);
            set_reg(c, V0, 2);
            Addr((inst as u32).wrapping_add(bmi::PITCH)).sw(c, pitch);
            if state == 2 {
                // A MIDI player.
                c.cycles += 9;
                call_ptr(c, table_callee::SND_LOCK, 0x002a_bf28)?;
                let v = lw_at(c, inst, bmi::ID);
                set_reg(c, A0, v);
                set_reg(c, A1, pitch);
                call_ptr(c, table_callee::SNDMIDI_SET_PITCH, 0x002a_bf34)?;
                call_ptr(c, table_callee::SND_UNLOCK, 0x002a_bf3c)?;
            } else {
                c.cycles += 3;
                let below_3 = slt(state, 3) != 0;
                set_reg(c, V0, 1);
                if below_3 {
                    c.cycles += 2;
                    if state == 1 {
                        // An SND voice handle.
                        c.cycles += 9;
                        call_ptr(c, table_callee::SND_LOCK, 0x002a_bf04)?;
                        let v = lw_at(c, inst, bmi::ID);
                        set_reg(c, A0, v);
                        set_reg(c, A1, pitch);
                        call_ptr(c, table_callee::SND_SET_PITCH, 0x002a_bf10)?;
                        call_ptr(c, table_callee::SND_UNLOCK, 0x002a_bf18)?;
                    } else {
                        c.cycles += 2;
                    }
                } else {
                    c.cycles += 3;
                    set_reg(c, V0, 3);
                    if state == 3 {
                        // A stream buffer.
                        let v = lw_at(c, inst, bmi::STREAM);
                        set_reg(c, A0, v);
                        c.cycles += 3;
                        set_reg(c, A1, pitch);
                        callee::STREAM_BUFFER_SET_PITCH.call(c, 0x002a_bf4c)?;
                    } else {
                        c.cycles += 2;
                    }
                }
            }
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BankMonitor_SndTickCallback()` 0x2ad998: the sound thread's tick hook
/// into the audio system: a virtual call (slot +8) on its tick listener.
pub fn bank_monitor_snd_tick_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        let sys = lw(c, (reg(c, GP) as u32).wrapping_add(ssx_audio::GLOBAL_GP) as u64);
        set_reg(c, A1, sys);
        frame.save_ra(c, 0);
        let vt = lw_at(c, sys, ssx_audio::TICK_LISTENER);
        set_reg(c, V0, vt);
        let m = VSlot::read(c, vt, 8);
        set_reg(c, V1, m.func);
        set_reg(c, A0, m.this(sys));
        call_ptr(c, m.func as u32, 0x002a_d9b8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Music.

/// `MusicPlayer_Update(mp)` 0x2b4388: update the current song (`dt` passes
/// through in f12) and attach its loop bank once one is pending.
pub fn music_player_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let mp = reg(c, A0);
        set_reg(c, S0, mp);
        let song = lw_at(c, mp, 0x408);
        set_reg(c, A0, song);
        if song != 0 {
            c.cycles += 6;
            set_reg(c, A1, 0);
            callee::SONG_INSTANCE_UPDATE.call(c, 0x002b_43ac)?;
            let song = lw_at(c, mp, 0x408);
            set_reg(c, V0, song);
            let pending = lw_at(c, song, 0x50);
            set_reg(c, V1, pending);
            if pending != 0 {
                c.cycles += 3;
                set_reg(c, A0, mp);
                callee::MUSIC_PLAYER_ATTACH_LOOP_BANK.call(c, 0x002b_43c4)?;
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SongInstance_GetStreamHandle()` 0x2b2488: the music track's stream
/// handle (`*PATH_GetTrackData(PATH_GetTrackStream(0x11000001))`), or -1.
pub fn song_instance_get_stream_handle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        set_reg(c, A0, 0x1100_0001);
        call_ptr(c, table_callee::PATH_GET_TRACK_STREAM, 0x002b_249c)?;
        let stream = reg(c, V0);
        if (stream as i64) < 0 {
            set_reg(c, V0, u64::MAX);
        } else {
            c.cycles += 3;
            set_reg(c, A0, stream);
            callee::PATH_GET_TRACK_DATA.call(c, 0x002b_24ac)?;
            let data = reg(c, V0);
            let v = lw(c, data);
            set_reg(c, V0, v);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// The SSXAudioSystem per-frame update.

/// `SSXAudioSystem_UpdateBanksAndStreams(dt f12, sys)` 0x2ab958: the load
/// queue, the crowd, the bank-monitor manager (a virtual call, slot +0x28,
/// with `dt`), music, speech and the stream array, in that order. `dt` is
/// kept in f20 for the virtual call. One block, 34 cycles up front.
pub fn ssx_audio_update_banks_and_streams(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let sys = reg(c, A0);
        set_reg(c, S0, sys);
        frame.save_ra(c, 0);
        let root = addu(sys, ssx_audio::ROOT as u64);
        set_reg(c, S1, root);
        save_fpr(c, frame, 20, 0x30);
        c.f[20] = c.f[12];
        let r = lw_at(c, sys, ssx_audio::ROOT);
        set_reg(c, V0, r);
        let v = lw(c, r);
        set_reg(c, A0, v);
        call_ptr(c, table_callee::SND_LOAD_QUEUE_UPDATE, 0x002a_b984)?;
        set_reg(c, A0, sys);
        callee::CROWD_AUDIO_UPDATE.call(c, 0x002a_b98c)?;
        let r = lw_at(c, sys, ssx_audio::ROOT);
        set_reg(c, V0, r);
        c.f[12] = c.f[20];
        let snd = lw(c, r);
        let vt = lw_at(c, snd, ssx_audio::SND_MANAGER);
        set_reg(c, V1, vt);
        let m = VSlot::read(c, vt, 0x28);
        set_reg(c, V0, m.delta);
        set_reg(c, A1, m.func);
        set_reg(c, A0, m.this(addu(snd, ssx_audio::SND_BANK_MONITOR as u64)));
        call_ptr(c, m.func as u32, 0x002a_b9b0)?;
        set_reg(c, A0, root);
        callee::MUSIC_PLAYER_UPDATE.call(c, 0x002a_b9b8)?;
        set_reg(c, A0, addu(sys, ssx_audio::SPEECH as u64));
        callee::SPEECH_SYSTEM_UPDATE.call(c, 0x002a_b9c0)?;
        let v = lw_at(c, sys, ssx_audio::ROOT);
        set_reg(c, A0, v);
        callee::STREAM_ARRAY_UPDATE_ALL.call(c, 0x002a_b9c8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        restore_fpr(c, frame, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem_UpdateBankMonitor(dt f12, sys)` 0x2ab7a0: update all 64
/// bank-monitor instances, with `dt = 0` while the audio is paused.
pub fn ssx_audio_update_bank_monitor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x20);
        save_fpr(c, frame, 20, 0x40);
        let sys = reg(c, A0);
        set_reg(c, S1, sys);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        let root = lw_at(c, sys, ssx_audio::ROOT);
        set_reg(c, V0, root);
        let snd = lw(c, root);
        set_reg(c, V1, snd);
        let paused = lw_at(c, snd, ssx_audio::SND_PAUSED);
        set_reg(c, A0, paused);
        c.f[20] = c.f[12];
        if paused != 0 {
            c.cycles += 1;
            c.f[20] = 0;
        }
        c.cycles += 3;
        let mut offset = 0u64; // s2
        let mut left = 0x3fu64; // s0
        set_reg(c, S2, offset);
        set_reg(c, S0, left);
        loop {
            c.cycles += 10;
            let root = lw_at(c, sys, ssx_audio::ROOT);
            set_reg(c, V0, root);
            set_reg(c, A1, sys);
            c.f[12] = c.f[20];
            left = sx((left as u32).wrapping_sub(1));
            set_reg(c, S0, left);
            let snd = lw(c, root);
            set_reg(c, V1, snd);
            let base = lw_at(c, snd, ssx_audio::SND_INSTANCES);
            set_reg(c, A0, addu(base, offset));
            callee::UPDATE_BANK_MONITOR_INSTANCE.call(c, 0x002a_b800)?;
            offset = sx((offset as u32).wrapping_add(bmi::SIZE));
            set_reg(c, S2, offset);
            if (left as i64) < 0 {
                break;
            }
            if event_due(c) {
                deliver_event(c, 0x002a_b7e0);
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        restore_fpr(c, frame, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem_UpdateBankMonitorInstance(dt f12, inst, sys)` 0x2ab6b0:
/// for a busy instance and `dt > 0`, run its per-frame callback (a pointer
/// to a member function of `SSXAudioSystem`, if set) and then its 3D update
/// against the listener (`snd + 0x1d8`, or null).
pub fn ssx_audio_update_bank_monitor_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        let inst = reg(c, A0);
        set_reg(c, S0, inst);
        save_fpr(c, frame, 20, 0x40);
        let sys = reg(c, A1);
        set_reg(c, S1, sys);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.f[20] = c.f[12];
        callee::BMI_POLL_FREE.call(c, 0x002a_b6d8)?;
        let dt = FpuF32(c.f[20]);
        if reg(c, V0) == 0 && {
            c.cycles += 5;
            abi::set_fpr(c, 0, FpuF32::ZERO);
            abi::c_lt_s(c, FpuF32::ZERO, dt)
        } {
            c.cycles += 3;
            let index = Addr((inst as u32).wrapping_add(bmi::CALLBACK + 2)).lh(c);
            set_reg(c, A1, index);
            if index != 0 {
                pmf_call(c, inst, sys, index)?;
            }
            c.cycles += 2;
            let mut listener = 0;
            if sys != 0 {
                c.cycles += 4;
                let root = lw_at(c, sys, ssx_audio::ROOT);
                let snd = lw(c, root);
                set_reg(c, V0, snd);
                if snd != 0 {
                    listener = addu(snd, ssx_audio::SND_BANK_MONITOR as u64);
                }
            }
            set_reg(c, A1, listener);
            c.cycles += 4;
            set_reg(c, A0, inst);
            c.f[12] = c.f[20];
            callee::BMI_UPDATE_3D.call(c, 0x002a_b780)?;
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 6;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        restore_fpr(c, frame, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// The instance callback: a gcc 2.9x pointer to member function
/// `{ short delta; short index; union { short vptr_offset; void *fn; } }`
/// at `inst + 0xa8`, called on `sys` with `(dt, inst)`. `index < 0` is a
/// plain function; otherwise the function is vtable entry `index - 1`
/// (8 bytes: `{ short delta; short; void *fn; }`, read with one `ld` into
/// s2) of the vtable at `sys + vptr_offset`, and both deltas apply.
fn pmf_call(c: &mut Cpu, inst: u64, sys: u64, index: u64) -> Flow {
    let pmf = (inst as u32).wrapping_add(bmi::CALLBACK);
    c.cycles += 2;
    let scaled = sx((index as u32) << 3);
    set_reg(c, V1, scaled);
    let virtual_ = (index as i64) >= 0;
    let (func, delta) = if virtual_ {
        c.cycles += 8;
        let vptr_offset = Addr(pmf + 4).lh(c);
        let vptr_at = addu(sys, vptr_offset);
        set_reg(c, V0, vptr_at);
        let vt = lw(c, vptr_at);
        set_reg(c, A0, vt);
        let entry = sx((scaled as u32).wrapping_add(vt as u32));
        let slot = c.ld((entry as u32).wrapping_sub(8));
        set_reg(c, S2, slot);
        let func = ((slot as i64) >> 32) as u64;
        let delta = Addr(pmf).lh(c);
        (func, delta)
    } else {
        c.cycles += 2;
        let func = Addr(pmf + 4).lw(c);
        let delta = Addr(pmf).lh(c);
        (func, delta)
    };
    set_reg(c, A2, func);
    set_reg(c, V1, delta);
    c.cycles += 2;
    set_reg(c, V0, reg(c, S2) & 0xffff);
    let this_delta = if virtual_ {
        c.cycles += 4;
        let vdelta = sra(sx((reg(c, V0) as u32) << 16), 16);
        addu(vdelta, delta)
    } else {
        c.cycles += 1;
        delta
    };
    set_reg(c, V0, this_delta);
    c.cycles += 4;
    set_reg(c, A0, addu(sys, this_delta));
    set_reg(c, A1, inst);
    c.f[12] = c.f[20];
    call_ptr(c, func as u32, 0x002a_b75c)
}

/// `SSXAudioSystem_UpdateTimers(sys)` 0x2ab8e8: unless timers are disabled,
/// tick the bank monitor and update the 48 timers.
pub fn ssx_audio_update_timers(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        let sys = reg(c, A0);
        set_reg(c, S1, sys);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        let root = lw_at(c, sys, ssx_audio::ROOT);
        let snd = lw(c, root);
        set_reg(c, V1, snd);
        let disabled = lw_at(c, snd, ssx_audio::SND_TIMERS_DISABLED);
        set_reg(c, V0, disabled);
        set_reg(c, A0, addu(snd, ssx_audio::SND_BANK_MONITOR as u64));
        if disabled == 0 {
            c.cycles += 3;
            let mut offset = ssx_audio::SND_TIMERS as u64; // s2
            set_reg(c, S2, offset);
            callee::BANK_MONITOR_TICK.call(c, 0x002a_b91c)?;
            let mut left = 0x2fu64; // s0
            set_reg(c, S0, left);
            loop {
                c.cycles += 8;
                let root = lw_at(c, sys, ssx_audio::ROOT);
                set_reg(c, V0, root);
                set_reg(c, A1, sys);
                left = sx((left as u32).wrapping_sub(1));
                set_reg(c, S0, left);
                let snd = lw(c, root);
                set_reg(c, A0, addu(snd, offset));
                callee::BANK_MONITOR_TIMER_UPDATE.call(c, 0x002a_b938)?;
                offset = sx((offset as u32).wrapping_add(ssx_audio::TIMER_SIZE));
                set_reg(c, S2, offset);
                if (left as i64) < 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event(c, 0x002a_b920);
                }
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

// ---------------------------------------------------------------------------
// SND: channels of a handle, pitch and pan.

/// The address of channel `index` (`channels + index * 0x8c`, through
/// `mult`, so LO/HI hold the product).
#[inline]
fn channel_addr(c: &mut Cpu, table: u64, index: u64) -> (u64, u64) {
    let off = sx(mult(c, index as u32, snd::CHANNEL_SIZE));
    (off, addu(table, off))
}

/// `SND_NextHandleChannel(ch, &it)` 0x3baa00: step the cursor `*it` (start
/// at -1) to the next voice channel of the handle whose first channel is
/// `ch`; 0 when there are no more. A single-voice handle (group 0) yields
/// `ch` once. Otherwise channels of the same group that are allocated and
/// hold a handle (word +0 >= 0) are yielded in index order; the cursor is
/// stored on every step.
pub fn snd_next_handle_channel(c: &mut Cpu) {
    let ch = reg(c, A0);
    let it = Addr(abi::arg(c, 1));
    c.cycles += 10;
    set_reg(c, T0, ch);
    let (off, _) = channel_addr(c, 0, ch);
    set_reg(c, V1, off);
    let state = sx(snd::STATE.0);
    set_reg(c, A2, state);
    let table = lw_at(c, state, snd_state::CHANNELS);
    let first = addu(table, off);
    set_reg(c, A0, first);
    let group = Addr((first as u32).wrapping_add(chan::GROUP)).lbu(c);
    set_reg(c, A3, group);
    let cursor = it.lw(c);
    set_reg(c, V0, cursor);
    let yielded = 'r: {
        if group == 0 {
            c.cycles += 2;
            set_reg(c, V0, 0);
            if (cursor as i64) >= 0 {
                break 'r false;
            }
            c.cycles += 1;
            it.sw(c, ch);
            break 'r true;
        }
        c.cycles += 6;
        let next = sx((cursor as u32).wrapping_add(1));
        it.sw(c, next);
        let count = Addr(snd::STATE.0 + snd_state::CHANNEL_COUNT).lh(c);
        set_reg(c, V1, count);
        let more = slt(next, count);
        set_reg(c, V0, more);
        set_reg(c, T0, state);
        if more == 0 {
            c.cycles += 1;
            set_reg(c, V0, 0);
            break 'r false;
        }
        c.cycles += 4;
        set_reg(c, T2, snd::CHANNEL_SIZE as u64);
        set_reg(c, T1, 1);
        let v = it.lw(c);
        set_reg(c, V1, v);
        loop {
            c.cycles += 7;
            let table = lw_at(c, state, snd_state::CHANNELS);
            let index = reg(c, V1);
            set_reg(c, A2, index);
            let (_, channel) = channel_addr(c, table, index);
            set_reg(c, A0, channel);
            let g = Addr((channel as u32).wrapping_add(chan::GROUP)).lbu(c);
            set_reg(c, V0, g);
            let after = sx((index as u32).wrapping_add(1));
            set_reg(c, V1, after);
            let mut stored = false;
            if g == group {
                c.cycles += 3;
                let allocated = Addr((channel as u32).wrapping_add(chan::ALLOCATED)).lb(c);
                set_reg(c, V0, allocated);
                if allocated != 1 {
                    it.sw(c, after);
                    stored = true;
                } else {
                    c.cycles += 3;
                    let handle = lw_at(c, channel, snd::CH_HANDLE);
                    set_reg(c, V0, handle);
                    if (handle as i64) >= 0 {
                        break 'r true;
                    }
                }
            }
            if !stored {
                c.cycles += 1;
                it.sw(c, after);
            }
            c.cycles += 4;
            let count = Addr(snd::STATE.0 + snd_state::CHANNEL_COUNT).lh(c);
            set_reg(c, V0, count);
            let more = slt(after, count);
            set_reg(c, V1, more);
            if more == 0 {
                c.cycles += 2;
                set_reg(c, V0, 0);
                abi::ret(c);
                return;
            }
            let v = it.lw(c);
            set_reg(c, V1, v);
            if event_due(c) {
                deliver_event(c, 0x003b_aa50);
            }
        }
    };
    c.cycles += 2;
    if yielded {
        set_reg(c, V0, 1);
    }
    abi::ret(c);
}

/// `SND_ChannelCalcPitch(index)` 0x3ba938: recompute the channel's final
/// pitch. The base pitch is cached: when stale (0) it is rebuilt from the
/// note's cents plus the pitch bend (`(bend - 0x40) * range >> 6`, the bend
/// remapped through a table if there is one) plus modulation
/// (`(table[mod] - 0x40) * depth >> 6`), through `SND_CentsToPitch`. The
/// final pitch is `cache * requested >> 12`.
pub fn snd_channel_calc_pitch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        set_reg(c, V1, snd::CHANNEL_SIZE as u64);
        set_reg(c, A1, snd::STATE_HI); // leftover
        let off = sx(mult(c, abi::arg(c, 0), snd::CHANNEL_SIZE));
        set_reg(c, A0, off);
        let frame = Frame::open(c, 0x20);
        let table = snd::CHANNELS.lw(c);
        set_reg(c, V0, table);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let ch = addu(table, off);
        set_reg(c, S0, ch);
        let field = |o: u32| Addr((ch as u32).wrapping_add(o));
        let cached = field(chan::PITCH_CACHE).lhu(c);
        set_reg(c, V1, cached);
        if cached != 0 {
            let p = field(chan::PITCH_CACHE).lhu(c);
            set_reg(c, V0, p);
        } else {
            c.cycles += 4;
            let range = field(chan::BEND_RANGE).lh(c);
            set_reg(c, V0, range);
            let mut cents = field(chan::CENTS).lh(c);
            set_reg(c, A1, cents);
            let range_u = field(chan::BEND_RANGE).lhu(c);
            set_reg(c, V1, range_u);
            if range != 0 {
                c.cycles += 3;
                let remap = lw_at(c, ch, chan::BEND_TABLE);
                set_reg(c, A0, remap);
                let mut bend = field(chan::BEND).lb(c);
                if remap != 0 {
                    c.cycles += 2;
                    bend = Addr((remap as u32).wrapping_add(bend as u32)).lb(c);
                }
                c.cycles += 6;
                let range = sra(sx((range_u as u32) << 16), 16);
                set_reg(c, V1, range);
                let delta = sx(mult(c, (bend as u32).wrapping_sub(0x40), range as u32));
                let delta = sra(delta, 6);
                set_reg(c, V0, delta);
                cents = addu(cents, delta);
                set_reg(c, A1, cents);
            }
            c.cycles += 3;
            let mod_table = lw_at(c, ch, chan::MOD_TABLE);
            set_reg(c, V0, mod_table);
            if mod_table != 0 {
                c.cycles += 8;
                let index = field(chan::MOD_INDEX).lbu(c);
                let depth = field(chan::MOD_DEPTH).lh(c);
                set_reg(c, A0, depth);
                let at = addu(mod_table, index);
                set_reg(c, V1, at);
                let m = Addr(at as u32).lb(c);
                let delta = sra(sx(mult(c, (m as u32).wrapping_sub(0x40), depth as u32)), 6);
                set_reg(c, V0, delta);
                cents = addu(cents, delta);
                set_reg(c, A1, cents);
            }
            c.cycles += 4;
            set_reg(c, A0, cents);
            callee::SND_CENTS_TO_PITCH.call(c, 0x003b_a9d4)?;
            let p = reg(c, V0);
            field(chan::PITCH_CACHE).sh(c, p);
            let p = field(chan::PITCH_CACHE).lhu(c);
            set_reg(c, V0, p);
        }
        c.cycles += 8;
        let requested = field(chan::REQUESTED_PITCH).lhu(c);
        set_reg(c, V1, requested);
        let product = sx(mult(c, reg(c, V0) as u32, requested as u32));
        let pitch = sra(product, 12);
        set_reg(c, V0, pitch);
        field(chan::FINAL_PITCH).sh(c, pitch);
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// Addresses of `SND_SetPitch`.
mod set_pitch {
    pub const START: u32 = 0x003b_8790;
    pub const END: u32 = 0x003b_8834;
    /// Loop head (the per-channel body), the resume point after an unwind.
    pub const HEAD: u32 = 0x003b_87d0;
}

/// Stages of `SND_SetPitch` / `SND_SetPan` (loop over the channels of a
/// handle with the cursor at `sp+0`).
#[derive(Clone, Copy, PartialEq, Debug)]
enum ChannelLoop {
    /// The per-channel body (the loop head; resume point).
    Body,
    /// `SND_NextHandleChannel(ch, sp)`.
    Next,
    /// Return the handle's first channel (s0 / s1).
    Exit,
    /// Return v0 as it is.
    Epilogue,
}

/// `Resume` for a stage that ended in an unwinding call.
impl From<Unwound> for Resume<ChannelLoop> {
    fn from(_: Unwound) -> Self {
        Resume::Unwind
    }
}

/// Run `stage`, mapping a call that unwound to [`Resume::Unwind`].
#[inline(always)]
fn stage(f: impl FnOnce() -> Result<Resume<ChannelLoop>, Unwound>) -> Resume<ChannelLoop> {
    f().unwrap_or_else(Resume::from)
}

/// The "next channel" step shared by SetPitch and SetPan: `cycles 5`,
/// `SND_NextHandleChannel(first, sp)` returning to `ret`; true when there
/// is another channel.
fn next_channel(c: &mut Cpu, first_reg: usize, ret: u32) -> Result<bool, Unwound> {
    c.cycles += 5;
    set_reg(c, A0, reg(c, first_reg));
    set_reg(c, A1, reg(c, SP));
    callee::SND_NEXT_HANDLE_CHANNEL.call(c, ret)?;
    Ok(reg(c, V0) != 0)
}

/// `SND_SetPitch(handle, pitch)` 0x3b8790: set the requested pitch scale of
/// every voice channel of a handle and recompute and apply their pitch.
/// Returns the handle's first channel (or the error from
/// `SND_HandleToChannel`), but 0 at once when a channel already has this
/// pitch (the remaining channels are not visited).
///
/// The original is a `match pc` dispatcher whose loop returns on an unwind
/// request, so the loop head is an entry point too (registers: s0 first
/// channel, s1 pitch, s2 0x8c, s3 the state block, a1 the cursor).
pub fn snd_set_pitch(c: &mut Cpu) {
    let mut st = match c.pc {
        set_pitch::START => match stage(|| set_pitch_entry(c)) {
            Resume::Go(s) => s,
            _ => return,
        },
        set_pitch::HEAD => ChannelLoop::Body,
        pc => {
            c.jump_out(pc, set_pitch::START, set_pitch::END);
            return;
        }
    };
    loop {
        let r = match st {
            ChannelLoop::Body => stage(|| set_pitch_body(c)),
            ChannelLoop::Next => stage(|| {
                let more = next_channel(c, S0, 0x003b_880c)?;
                if !more {
                    return Ok(Resume::Go(ChannelLoop::Exit));
                }
                let v = lw(c, reg(c, SP));
                set_reg(c, A1, v);
                if back_edge_unwinds(c, set_pitch::HEAD) {
                    return Ok(Resume::Unwind);
                }
                Ok(Resume::Go(ChannelLoop::Body))
            }),
            ChannelLoop::Exit => {
                c.cycles += 1;
                set_reg(c, V0, reg(c, S0));
                Resume::Go(ChannelLoop::Epilogue)
            }
            ChannelLoop::Epilogue => {
                c.cycles += 7;
                let frame = Frame { size: 0x60 };
                frame.restore(c, S0, 0x50);
                frame.restore(c, S1, 0x40);
                frame.restore(c, S2, 0x30);
                frame.restore(c, S3, 0x20);
                frame.restore_ra(c, 0x10);
                frame.close(c);
                Resume::Done
            }
        };
        match r {
            Resume::Go(s) => st = s,
            Resume::Done | Resume::Unwind => return,
        }
    }
}

/// `SND_SetPitch` up to the loop: the frame, `SND_HandleToChannel`, the
/// cursor.
fn set_pitch_entry(c: &mut Cpu) -> Result<Resume<ChannelLoop>, Unwound> {
    c.cycles += 11;
    let frame = Frame::open(c, 0x60);
    frame.save(c, S1, 0x40);
    frame.save(c, S0, 0x50);
    frame.save(c, S2, 0x30);
    frame.save(c, S3, 0x20);
    frame.save_ra(c, 0x10);
    set_reg(c, S1, reg(c, A1));
    callee::SND_HANDLE_TO_CHANNEL.call(c, 0x003b_87b0)?;
    let first = reg(c, V0);
    set_reg(c, S0, first);
    set_reg(c, V1, u64::MAX);
    if (first as i64) < 0 {
        return Ok(Resume::Go(ChannelLoop::Exit));
    }
    c.cycles += 5;
    set_reg(c, V0, snd::STATE_HI);
    set_reg(c, S3, sx(snd::STATE.0));
    Addr(abi::r32(c, SP)).sw(c, u64::MAX);
    set_reg(c, S2, snd::CHANNEL_SIZE as u64);
    Ok(Resume::Go(ChannelLoop::Next))
}

/// The per-channel body of `SND_SetPitch` (loop head 0x3b87d0).
fn set_pitch_body(c: &mut Cpu) -> Result<Resume<ChannelLoop>, Unwound> {
    c.cycles += 6;
    let table = lw_at(c, reg(c, S3), snd_state::CHANNELS);
    set_reg(c, A0, table);
    let index = reg(c, A1);
    let off = sx(mult(c, index as u32, reg(c, S2) as u32));
    let ch = addu(table, off);
    set_reg(c, V0, ch);
    let current = Addr((ch as u32).wrapping_add(chan::REQUESTED_PITCH)).lhu(c);
    set_reg(c, V1, current);
    set_reg(c, A0, index);
    let pitch = reg(c, S1);
    if current == pitch {
        c.cycles += 2;
        set_reg(c, V0, 0);
        return Ok(Resume::Go(ChannelLoop::Epilogue));
    }
    c.cycles += 4;
    Addr((ch as u32).wrapping_add(chan::REQUESTED_PITCH)).sh(c, pitch);
    callee::SND_CHANNEL_CALC_PITCH.call(c, 0x003b_87f8)?;
    let v = lw(c, reg(c, SP));
    set_reg(c, A0, v);
    callee::SND_CHANNEL_APPLY_PITCH.call(c, 0x003b_8800)?;
    Ok(Resume::Go(ChannelLoop::Next))
}

/// Addresses of `SND_SetPan`.
mod set_pan {
    pub const START: u32 = 0x003b_8c00;
    pub const END: u32 = 0x003b_8cc8;
    /// Loop head (the per-channel body), the resume point after an unwind.
    pub const HEAD: u32 = 0x003b_8c70;
}

/// `SND_SetPan(handle, pan, spread)` 0x3b8c00: clamp the stereo spread to
/// -0x4000..0x3fff and, for every voice channel of the handle, set the
/// spread and the pan (the patch's pan plus `pan`) and apply them. Returns
/// the handle's first channel or the error from `SND_HandleToChannel`.
///
/// A `match pc` dispatcher like [`snd_set_pitch`]; the loop head is an
/// entry point (registers: s0 spread, s1 first channel, s2 pan, s3 0x8c,
/// s4 the state block, a0 the cursor).
pub fn snd_set_pan(c: &mut Cpu) {
    let mut st = match c.pc {
        set_pan::START => match stage(|| set_pan_entry(c)) {
            Resume::Go(s) => s,
            _ => return,
        },
        set_pan::HEAD => ChannelLoop::Body,
        pc => {
            c.jump_out(pc, set_pan::START, set_pan::END);
            return;
        }
    };
    loop {
        let r = match st {
            ChannelLoop::Body => stage(|| {
                c.cycles += 8;
                let table = lw_at(c, reg(c, S4), snd_state::CHANNELS);
                let index = reg(c, A0);
                let off = sx(mult(c, index as u32, reg(c, S3) as u32));
                set_reg(c, A1, off);
                let ch = addu(table, off);
                set_reg(c, V1, ch);
                let patch_pan = Addr((ch as u32).wrapping_add(chan::PATCH_PAN)).lhu(c);
                Addr((ch as u32).wrapping_add(chan::SPREAD)).sh(c, reg(c, S0));
                let pan = addu(patch_pan, reg(c, S2));
                set_reg(c, V0, pan);
                Addr((ch as u32).wrapping_add(chan::PAN)).sh(c, pan);
                callee::SND_CHANNEL_APPLY_PAN.call(c, 0x003b_8c90)?;
                Ok(Resume::Go(ChannelLoop::Next))
            }),
            ChannelLoop::Next => stage(|| {
                let more = next_channel(c, S1, 0x003b_8c9c)?;
                let v = lw(c, reg(c, SP));
                set_reg(c, A0, v);
                if !more {
                    return Ok(Resume::Go(ChannelLoop::Exit));
                }
                if back_edge_unwinds(c, set_pan::HEAD) {
                    return Ok(Resume::Unwind);
                }
                Ok(Resume::Go(ChannelLoop::Body))
            }),
            ChannelLoop::Exit | ChannelLoop::Epilogue => {
                c.cycles += 9;
                set_reg(c, V0, reg(c, S1));
                let frame = Frame { size: 0x70 };
                frame.restore(c, S0, 0x60);
                frame.restore(c, S1, 0x50);
                frame.restore(c, S2, 0x40);
                frame.restore(c, S3, 0x30);
                frame.restore(c, S4, 0x20);
                frame.restore_ra(c, 0x10);
                frame.close(c);
                Resume::Done
            }
        };
        match r {
            Resume::Go(s) => st = s,
            Resume::Done | Resume::Unwind => return,
        }
    }
}

/// `SND_SetPan` up to the loop: the frame, the spread clamp,
/// `SND_HandleToChannel`, the cursor.
fn set_pan_entry(c: &mut Cpu) -> Result<Resume<ChannelLoop>, Unwound> {
    c.cycles += 11;
    let frame = Frame::open(c, 0x70);
    frame.save(c, S0, 0x60);
    frame.save(c, S2, 0x40);
    let mut spread = reg(c, A2);
    frame.save(c, S1, 0x50);
    set_reg(c, S2, reg(c, A1));
    frame.save(c, S3, 0x30);
    let below_max = slt(spread, 0x4000);
    set_reg(c, V0, below_max);
    frame.save(c, S4, 0x20);
    frame.save_ra(c, 0x10);
    if below_max == 0 {
        c.cycles += 2;
        spread = 0x3fff;
    } else {
        c.cycles += 4;
        let above_min = slt(-0x4001i64 as u64, spread);
        set_reg(c, V0, above_min);
        set_reg(c, V1, -0x4000i64 as u64);
        if above_min == 0 {
            spread = -0x4000i64 as u64;
        }
    }
    set_reg(c, S0, spread);
    c.cycles += 5;
    callee::SND_HANDLE_TO_CHANNEL.call(c, 0x003b_8c4c)?;
    let first = reg(c, V0);
    set_reg(c, S1, first);
    set_reg(c, V1, u64::MAX);
    if (first as i64) < 0 {
        return Ok(Resume::Go(ChannelLoop::Exit));
    }
    c.cycles += 5;
    set_reg(c, V0, snd::STATE_HI);
    set_reg(c, S4, sx(snd::STATE.0));
    Addr(abi::r32(c, SP)).sw(c, u64::MAX);
    set_reg(c, S3, snd::CHANNEL_SIZE as u64);
    Ok(Resume::Go(ChannelLoop::Next))
}

// ---------------------------------------------------------------------------
// SND: the driver mutex and the sound thread's service loop.

/// `SND_DefaultLock()` 0x3b5860: the default `SND_Lock` hook, the driver's
/// recursive mutex.
pub fn snd_default_lock(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(callee::SND_MUTEX_LOCK), 0x003b_5870)
}

/// `SND_DefaultUnlock()` 0x3b5880: the default `SND_Unlock` hook.
pub fn snd_default_unlock(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(callee::SND_MUTEX_UNLOCK), 0x003b_5890)
}

/// The mutex field `off` (from [`snd_mutex::BASE`]).
#[inline]
fn mutex_field(off: u32) -> Addr {
    Addr(snd_mutex::BASE + off)
}

/// `SND_MutexLock()` 0x3c3380: a recursive mutex on a semaphore. A thread
/// that doesn't own it waits on the semaphore (`WaitSema` may switch
/// threads) and takes ownership; then the nesting count goes up.
pub fn snd_mutex_lock(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, table_callee::GET_THREAD_ID, 0x003c_3398)?;
        let tid = reg(c, V0);
        set_reg(c, S1, tid);
        set_reg(c, V0, snd::STATE_HI);
        set_reg(c, S0, sx(snd_mutex::BASE));
        let owner = mutex_field(snd_mutex::OWNER).lw(c);
        set_reg(c, V1, owner);
        if owner != tid {
            c.cycles += 4;
            let v = mutex_field(snd_mutex::SEMA).lw(c);
            set_reg(c, A0, v);
            call_ptr(c, table_callee::WAIT_SEMA, 0x003c_33bc)?;
            mutex_field(snd_mutex::OWNER).sw(c, tid);
        }
        c.cycles += 8;
        let count = sx((mutex_field(snd_mutex::COUNT).lw(c) as u32).wrapping_add(1));
        set_reg(c, V0, count);
        mutex_field(snd_mutex::COUNT).sw(c, count);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SND_MutexUnlock()` 0x3c33e0: the owner decrements the nesting count and
/// gives up ownership at 0. Then, as a separate test, whenever the count is
/// below 1 the semaphore is signalled, even when the caller wasn't the
/// owner (a quirk of the original).
pub fn snd_mutex_unlock(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, table_callee::GET_THREAD_ID, 0x003c_33f0)?;
        let tid = reg(c, V0);
        set_reg(c, A1, snd::STATE_HI);
        set_reg(c, A0, sx(snd_mutex::BASE));
        let owner = mutex_field(snd_mutex::OWNER).lw(c);
        set_reg(c, V1, owner);
        if owner == tid {
            c.cycles += 6;
            let count = sx((mutex_field(snd_mutex::COUNT).lw(c) as u32).wrapping_sub(1));
            set_reg(c, V0, count);
            mutex_field(snd_mutex::COUNT).sw(c, count);
            let count = mutex_field(snd_mutex::COUNT).lw(c);
            set_reg(c, V1, count);
            if count == 0 {
                c.cycles += 3;
                set_reg(c, V0, u64::MAX);
                mutex_field(snd_mutex::OWNER).sw(c, u64::MAX);
            }
        }
        c.cycles += 3;
        let count = mutex_field(snd_mutex::COUNT).lw(c);
        set_reg(c, V0, count);
        frame.restore_ra(c, 0);
        if (count as i64) <= 0 {
            c.cycles += 4;
            let v = mutex_field(snd_mutex::SEMA).lw(c);
            set_reg(c, A0, v);
            call_ptr(c, table_callee::SIGNAL_SEMA, 0x003c_3444)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `SND_Service(rate)` 0x3b5540: the sound thread's service call, made
/// `rate` times a second. It keeps the driver at 100 ticks a second: after
/// counting this call it runs `SND_Tick` until the tick counter exceeds
/// `calls * 100 / rate` (computed once, `divu`). A new rate restarts both
/// counters. Nothing happens before the driver is initialised.
pub fn snd_service(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        set_reg(c, V1, snd::STATE_HI);
        frame.save(c, S0, 0x20);
        let state = sx(snd::STATE.0);
        set_reg(c, A1, state);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let field = |o: u32| Addr(snd::STATE.0 + o);
        let initialized = field(snd_state::INITIALIZED).lb(c);
        set_reg(c, V0, initialized);
        let rate = reg(c, A0);
        set_reg(c, A2, rate);
        if initialized == 0 {
            c.cycles += 1;
        } else {
            c.cycles += 5;
            let old = field(snd_state::SERVICE_RATE).lh(c);
            set_reg(c, V0, old);
            if rate != old {
                c.cycles += 3;
                field(snd_state::SERVICE_RATE).sh(c, rate);
                field(snd_state::TICKS).sw(c, 0);
                field(snd_state::SERVICE_CALLS).sw(c, 0);
            }
            c.cycles += 6;
            let calls = sx((field(snd_state::SERVICE_CALLS).lw(c) as u32).wrapping_add(1));
            set_reg(c, A0, 100);
            field(snd_state::SERVICE_CALLS).sw(c, calls);
            c.cycles += 8;
            let calls = field(snd_state::SERVICE_CALLS).lw(c);
            let ticks = field(snd_state::TICKS).lw(c);
            let scaled = sx(mult(c, calls as u32, 100));
            set_reg(c, V0, scaled);
            abi::divu(c, scaled as u32, rate as u32);
            let due = c.lo[0];
            set_reg(c, S0, due);
            let ahead = due < ticks;
            set_reg(c, V1, ahead as u64);
            if !ahead {
                c.cycles += 1;
                set_reg(c, S1, state);
                loop {
                    c.cycles += 6;
                    callee::SND_TICK.call(c, 0x003b_55c8)?;
                    let ticks = field(snd_state::TICKS).lw(c);
                    let done = due < ticks;
                    set_reg(c, V0, done as u64);
                    if done {
                        break;
                    }
                    if event_due(c) {
                        deliver_event(c, 0x003b_55c0);
                    }
                }
                c.cycles += 1;
            }
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes::{covered, machine, ptr, vtable, FAKE, LOG_RESULTS};
    use crate::difftest::{check, check_lohi, check_paths, fill, require_paths, Rng, DATA, GAME_GP};
    use ssx3_ee::{s07, s08, s13};

    const SEEDS: u32 = 300;
    /// The test channel table.
    const CHANNELS: u32 = DATA + 0x10_0000;
    /// The `SND_Lock` / `SND_Unlock` hooks (fakes in the tests).
    const LOCK_HOOK: u32 = 0x0044_c46c;

    /// [`reg64`] with the value computed first.
    macro_rules! r64 {
        ($r:expr, $v:expr) => {{
            let v = $v;
            reg64($r, v)
        }};
    }

    /// A whole register: mostly a sign-extended word, sometimes garbage above.
    fn reg64(r: &mut Rng, v: u32) -> u64 {
        if r.next_u32() % 4 == 0 {
            (r.next_u32() as u64) << 32 | v as u64
        } else {
            sx(v)
        }
    }

    /// A channel table of `n` channels: random groups, flags, handles and
    /// pitch fields; returns `n`.
    fn channels(c: &mut Cpu, r: &mut Rng, n: u32) -> u32 {
        c.bus.write32(snd::CHANNELS.0, CHANNELS);
        c.bus.write16(snd::CHANNEL_COUNT.0, n as u16);
        for i in 0..n {
            let ch = CHANNELS + i * snd::CHANNEL_SIZE;
            c.bus.write8(ch + chan::GROUP, (r.next_u32() % 3) as u8);
            let alloc = if r.next_u32() % 4 != 0 { 1 } else { r.next_u32() as u8 };
            c.bus.write8(ch + chan::ALLOCATED, alloc);
            let h = if r.next_u32() % 5 != 0 { (r.next_u32() << 8 | i) & 0x7fff_ffff } else { r.next_u32() };
            c.bus.write32(ch, h);
            c.bus.write16(ch + chan::PITCH_CACHE, if r.next_u32() % 4 == 0 { 0 } else { r.next_u32() as u16 });
            c.bus.write16(ch + chan::REQUESTED_PITCH, (r.next_u32() % 4) as u16 * 0x800);
            c.bus.write16(ch + chan::CENTS, (r.next_u32() % 0x3000) as u16);
            c.bus.write16(ch + chan::PATCH_PAN, r.next_u32() as u16);
        }
        n
    }

    /// A handle: mostly the handle of one of the channels.
    fn handle(c: &mut Cpu, r: &mut Rng, n: u32) -> u32 {
        if n > 0 && r.next_u32() % 5 != 0 {
            c.bus.read32(CHANNELS + (r.next_u32() % n) * snd::CHANNEL_SIZE)
        } else {
            r.next_u32()
        }
    }

    // --- getters ---------------------------------------------------------

    fn game_chain(c: &mut Cpu, r: &mut Rng) -> u32 {
        let game = ptr(r);
        let module = ptr(r) + 0x10_0000;
        c.bus.write32(GAME_GP - ssx_audio::GAME_GP, game);
        c.bus.write32(game + 0x84, if r.next_u32() % 8 == 0 { 0 } else { module });
        c.bus.write32(module + 0xc, r.next_u32());
        module
    }

    #[test]
    fn get_game_module_and_race_man() {
        for (g, h) in [
            (s07::f_0028b1c8__generated as fn(&mut Cpu), ssx_audio_get_game_module as fn(&mut Cpu)),
            (s07::f_0028b1d8__generated, ssx_audio_get_race_man),
        ] {
            check(g, h, SEEDS, &|c, seed| {
                let mut r = machine(c, seed, 1, 0);
                game_chain(c, &mut r);
            });
        }
    }

    #[test]
    fn get_channel_gain_ptr() {
        check_paths(s07::f_00287968__generated, ssx_audio_get_channel_gain_ptr, SEEDS, &[6, 9, 10], &|c, seed| {
            let mut r = machine(c, seed, 2, 0);
            c.g[A0][0] = r64!(&mut r, r.next_u32());
            let k = if r.next_u32() % 2 == 0 { 4 } else { r.next_u32() % 8 };
            c.g[A1][0] = r64!(&mut r, k);
            let i = if r.next_u32() % 2 == 0 { r.next_u32() % 14 } else { r.next_u32() };
            c.g[A2][0] = r64!(&mut r, i);
        });
    }

    #[test]
    fn is_replay_playing() {
        check_paths(s07::f_00288ae0__generated, ssx_audio_is_replay_playing, SEEDS, &[13, 21, 33, 34], &|c, seed| {
            let mut r = machine(c, seed, 3, 0);
            let module = game_chain(c, &mut r);
            let replay = ptr(&mut r) + 0x20_0000;
            c.bus.write32(module + 0x28, if r.next_u32() % 4 == 0 { 0 } else { replay });
            let s = match r.next_u32() % 3 {
                0 => 0,
                1 => r.next_u32(),
                _ => (r.next_u32() % 14).wrapping_sub(2),
            };
            c.bus.write32(replay, s);
        });
    }

    // --- BankMonitor -----------------------------------------------------

    fn falloff_args(c: &mut Cpu, r: &mut Rng) {
        c.f[12] = if r.next_u32() % 3 == 0 { r.edge_f32() } else { r.f32(50.0) };
        c.f[13] = if r.next_u32() % 3 == 0 { r.edge_f32() } else { r.f32(60.0) };
        let v = if r.next_u32() % 3 == 0 { r.next_u32() } else { r.next_u32() % 128 };
        c.g[A0][0] = r64!(r, v);
        c.g[A1][0] = r64!(r, v ^ r.next_u32() % 3);
    }

    // The `0 < radius` test failing after `d < radius` (17 cycles) is
    // unreachable: `d` is never below 0 in the compare order.
    #[test]
    fn calc_falloff_volume() {
        check_paths(s08::f_002abc18__generated, bank_monitor_calc_falloff_volume, 600, &[14, 25], &|c, seed| {
            let mut r = machine(c, seed, 4, 0);
            falloff_args(c, &mut r);
        });
    }

    #[test]
    fn get_falloff_volume() {
        check_paths(s08::f_002abc80__generated, bank_monitor_instance_get_falloff_volume, 600, &[21, 32], &|c, seed| {
            let mut r = machine(c, seed, 5, 0);
            falloff_args(c, &mut r);
        });
    }

    /// Fake lock hooks, so `SND_Lock`/`SND_Unlock` reach the fakes.
    fn lock_hooks(c: &mut Cpu) {
        c.bus.write32(LOCK_HOOK, FAKE + 0x100);
        c.bus.write32(LOCK_HOOK + 4, FAKE + 0x110);
    }

    /// A bank-monitor instance with a random state.
    fn instance(c: &mut Cpu, r: &mut Rng, inst: u32, busy: u32) {
        fill(c, r, inst, bmi::SIZE / 4);
        let s = if r.next_u32() % busy == 0 { r.next_u32() % 5 } else { 0 };
        c.bus.write32(inst, s);
        c.bus.write32(inst + bmi::ID, r.next_u32() % 4);
        c.bus.write32(inst + bmi::STOPPING, if r.next_u32() % 2 == 0 { 0 } else { 1 });
        c.bus.write32(inst + bmi::STREAM, 0);
        c.bus.write32(inst + bmi::CALLBACK, 0);
    }

    #[test]
    fn poll_free() {
        covered(s08::f_002abb80__generated, bank_monitor_instance_poll_free, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 6, 4);
            lock_hooks(c);
            let inst = ptr(&mut r);
            instance(c, &mut r, inst, 1);
            c.g[A0][0] = r64!(&mut r, inst);
        });
    }

    driven!(gen_find_instance, s08::f_002ad550__generated);
    driven!(hand_find_instance, bank_monitor_find_instance);

    fn find_instance_setup(c: &mut Cpu, seed: u32, r: &mut Rng) {
        lock_hooks(c);
        let bm = ptr(r);
        let insts = DATA + 0x20_0000;
        c.bus.write32(bm + bank_monitor::INSTANCES, insts);
        for i in 0..64 {
            instance(c, r, insts + i * bmi::SIZE, 3);
            if r.next_u32() % 4 != 0 {
                c.bus.write32(insts + i * bmi::SIZE + bmi::STOPPING, 0);
            }
        }
        c.g[A0][0] = sx(bm);
        c.g[A1][0] = r64!(r, r.next_u32() % 4);
        c.g[A2][0] = r64!(r, 1 + r.next_u32() % 3);
        let _ = seed;
    }

    #[test]
    fn find_instance() {
        check(s08::f_002ad550__generated, bank_monitor_find_instance, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 7, 0);
            find_instance_setup(c, seed, &mut r);
        });
        check_events(gen_find_instance, hand_find_instance, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 8, 0);
            find_instance_setup(c, seed, &mut r);
            schedule(c, &mut r, seed, 0x002a_d550)
        });
    }

    #[test]
    fn bmi_set_pitch() {
        covered(s08::f_002abe78__generated, bank_monitor_instance_set_pitch, 600, &|c, seed| {
            let mut r = machine(c, seed, 9, 4);
            lock_hooks(c);
            let inst = ptr(&mut r);
            instance(c, &mut r, inst, 1);
            let last = r.next_u32() % 0x4000;
            c.bus.write32(inst + bmi::PITCH, last);
            let p = match r.next_u32() % 4 {
                0 => r.next_u32(),
                1 => last.wrapping_add(r.next_u32() % 0x200).wrapping_sub(0x100),
                _ => r.next_u32() % 0x5000,
            };
            c.g[A0][0] = sx(inst);
            c.g[A1][0] = r64!(&mut r, p);
        });
    }

    #[test]
    fn snd_tick_callback() {
        covered(s08::f_002ad998__generated, bank_monitor_snd_tick_callback, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 10, 1);
            let sys = ptr(&mut r);
            c.bus.write32(GAME_GP + ssx_audio::GLOBAL_GP, sys);
            let vt = vtable(c, &mut r, &[8]);
            c.bus.write32(sys + ssx_audio::TICK_LISTENER, vt);
        });
    }

    // --- music -----------------------------------------------------------

    #[test]
    fn music_player_update_matches() {
        check(s08::f_002b4388__generated, music_player_update, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 11, 0);
            let mp = ptr(&mut r);
            let song = DATA + 0x30_0000;
            c.bus.write32(mp + 0x408, if r.next_u32() % 3 == 0 { 0 } else { song });
            c.bus.write32(song + 0x50, r.next_u32() % 2);
            c.g[A0][0] = sx(mp);
        });
        require_paths(s08::f_002b4388__generated, SEEDS, &[10], &|c, seed| {
            let mut r = machine(c, seed, 11, 0);
            let mp = ptr(&mut r);
            c.bus.write32(mp + 0x408, 0);
            c.g[A0][0] = sx(mp);
        });
    }

    #[test]
    fn get_stream_handle() {
        check(s08::f_002b2488__generated, song_instance_get_stream_handle, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 12, 0);
        });
    }

    // --- the per-frame update -------------------------------------------

    /// sys → root → snd, with the bank-monitor manager's vtable (slot 0x28 a
    /// fake), instances and timers.
    fn audio_system(c: &mut Cpu, r: &mut Rng, busy: u32) -> u32 {
        lock_hooks(c);
        let sys = DATA + 0x40_0000;
        let root = DATA + 0x50_0000;
        let snd = DATA + 0x60_0000;
        c.bus.write32(sys + ssx_audio::ROOT, root);
        c.bus.write32(root, snd);
        let vt = vtable(c, r, &[0x28]);
        c.bus.write32(snd + ssx_audio::SND_MANAGER, vt);
        c.bus.write32(snd + ssx_audio::SND_PAUSED, r.next_u32() % 2);
        c.bus.write32(snd + ssx_audio::SND_TIMERS_DISABLED, if r.next_u32() % 3 == 0 { 1 } else { 0 });
        let insts = DATA + 0x70_0000;
        c.bus.write32(snd + ssx_audio::SND_INSTANCES, insts);
        for i in 0..64 {
            instance(c, r, insts + i * bmi::SIZE, busy);
            c.bus.write32(insts + i * bmi::SIZE + bmi::STOPPING, 0);
        }
        c.f[12] = if r.next_u32() % 3 == 0 { r.edge_f32() } else { r.f32(0.05) };
        sys
    }

    #[test]
    fn update_banks_and_streams() {
        covered(s08::f_002ab958__generated, ssx_audio_update_banks_and_streams, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 13, 2);
            let sys = audio_system(c, &mut r, 1);
            c.g[A0][0] = sx(sys);
        });
    }

    driven!(gen_update_bank_monitor, s08::f_002ab7a0__generated);
    driven!(hand_update_bank_monitor, ssx_audio_update_bank_monitor);

    #[test]
    fn update_bank_monitor() {
        check(s08::f_002ab7a0__generated, ssx_audio_update_bank_monitor, 100, &|c, seed| {
            let mut r = machine(c, seed, 14, 0);
            let sys = audio_system(c, &mut r, 8);
            c.g[A0][0] = sx(sys);
        });
        check_events(gen_update_bank_monitor, hand_update_bank_monitor, 200, &|c, seed| {
            let mut r = machine(c, seed, 15, 0);
            let sys = audio_system(c, &mut r, 8);
            c.g[A0][0] = sx(sys);
            schedule(c, &mut r, seed, 0x002a_b7a0)
        });
    }

    #[test]
    fn update_bank_monitor_instance() {
        covered(s08::f_002ab6b0__generated, ssx_audio_update_bank_monitor_instance, 600, &|c, seed| {
            let mut r = machine(c, seed, 16, 3);
            let sys = audio_system(c, &mut r, 1);
            let inst = DATA + 0x70_0000;
            // The callback: none, a plain function or a virtual one.
            let delta = (r.next_u32() % 64) as u16 & !3;
            c.bus.write16(inst + bmi::CALLBACK, delta);
            match r.next_u32() % 3 {
                0 => c.bus.write16(inst + bmi::CALLBACK + 2, 0),
                1 => {
                    c.bus.write16(inst + bmi::CALLBACK + 2, 0xffff);
                    c.bus.write32(inst + bmi::CALLBACK + 4, FAKE + 0x200);
                }
                _ => {
                    let index = 1 + r.next_u32() % 4;
                    let vptr_off = (r.next_u32() % 32) * 4;
                    c.bus.write16(inst + bmi::CALLBACK + 2, index as u16);
                    c.bus.write16(inst + bmi::CALLBACK + 4, vptr_off as u16);
                    let vt = DATA + 0x80_0000;
                    c.bus.write32(sys + vptr_off, vt);
                    let entry = vt + (index - 1) * 8;
                    c.bus.write32(entry, r.next_u32());
                    c.bus.write32(entry + 4, FAKE + 0x300);
                }
            }
            c.g[A0][0] = sx(inst);
            c.g[A1][0] = if r.next_u32() % 8 == 0 { 0 } else { sx(sys) };
        });
    }

    driven!(gen_update_timers, s08::f_002ab8e8__generated);
    driven!(hand_update_timers, ssx_audio_update_timers);

    #[test]
    fn update_timers() {
        check(s08::f_002ab8e8__generated, ssx_audio_update_timers, 100, &|c, seed| {
            let mut r = machine(c, seed, 17, 0);
            let sys = audio_system(c, &mut r, 1);
            c.g[A0][0] = sx(sys);
        });
        check_events(gen_update_timers, hand_update_timers, 200, &|c, seed| {
            let mut r = machine(c, seed, 18, 0);
            let sys = audio_system(c, &mut r, 1);
            c.bus.write32(DATA + 0x60_0000 + ssx_audio::SND_TIMERS_DISABLED, 0);
            c.g[A0][0] = sx(sys);
            schedule(c, &mut r, seed, 0x002a_b8e8)
        });
    }

    // --- SND channels ------------------------------------------------------

    fn next_channel_setup(c: &mut Cpu, r: &mut Rng) {
        let n = 1 + r.next_u32() % 24;
        let n = channels(c, r, n);
        let it = DATA + 0x10;
        let cur = if r.next_u32() % 2 == 0 { u32::MAX } else { (r.next_u32() % (n + 3)).wrapping_sub(1) };
        c.bus.write32(it, cur);
        c.g[A0][0] = r64!(r, r.next_u32() % n);
        c.g[A1][0] = sx(it);
    }

    driven!(gen_next, s13::f_003baa00__generated);
    driven!(hand_next, snd_next_handle_channel);

    #[test]
    fn next_handle_channel() {
        check_lohi(s13::f_003baa00__generated, snd_next_handle_channel, 600, &|c, seed| {
            let mut r = machine(c, seed, 19, 0);
            next_channel_setup(c, &mut r);
        });
        require_paths(s13::f_003baa00__generated, 600, &[14, 15, 19], &|c, seed| {
            let mut r = machine(c, seed, 19, 0);
            next_channel_setup(c, &mut r);
        });
        check_events(gen_next, hand_next, 600, &|c, seed| {
            let mut r = machine(c, seed, 20, 0);
            next_channel_setup(c, &mut r);
            schedule(c, &mut r, seed, 0x003b_aa00)
        });
    }

    #[test]
    fn channel_calc_pitch() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 21, 0);
            let n = channels(c, &mut r, 8);
            let i = r.next_u32() % n;
            let ch = CHANNELS + i * snd::CHANNEL_SIZE;
            let tables = DATA + 0x20_0000;
            fill(&mut *c, &mut r, tables, 0x80);
            c.bus.write16(ch + chan::BEND_RANGE, if r.next_u32() % 3 == 0 { 0 } else { r.next_u32() as u16 });
            c.bus.write8(ch + chan::BEND, r.next_u32() as u8);
            c.bus.write32(ch + chan::BEND_TABLE, if r.next_u32() % 2 == 0 { 0 } else { tables + 0x80 });
            c.bus.write32(ch + chan::MOD_TABLE, if r.next_u32() % 2 == 0 { 0 } else { tables });
            c.bus.write8(ch + chan::MOD_INDEX, r.next_u32() as u8);
            c.bus.write16(ch + chan::MOD_DEPTH, r.next_u32() as u16);
            c.bus.write16(ch + chan::REQUESTED_PITCH, r.next_u32() as u16);
            c.g[A0][0] = r64!(&mut r, i);
        };
        check_lohi(s13::f_003ba938__generated, snd_channel_calc_pitch, 600, &setup);
        require_paths(s13::f_003ba938__generated, 600, &[19], &setup);
    }

    fn set_pitch_setup(c: &mut Cpu, r: &mut Rng) {
        lock_hooks(c);
        let n = r.next_u32() % 16;
        let n = channels(c, r, n);
        let h = handle(c, r, n);
        c.g[A0][0] = r64!(r, h);
        let p = if r.next_u32() % 2 == 0 { (r.next_u32() % 4) * 0x800 } else { r.next_u32() & 0xffff };
        c.g[A1][0] = if r.next_u32() % 8 == 0 { r.next_u32() as u64 | (r.next_u32() as u64) << 32 } else { p as u64 };
    }

    driven!(gen_set_pitch, s13::f_003b8790__generated);
    driven!(hand_set_pitch, snd_set_pitch);

    #[test]
    fn set_pitch() {
        check_lohi(s13::f_003b8790__generated, snd_set_pitch, 400, &|c, seed| {
            let mut r = machine(c, seed, 22, 0);
            c.pc = 0x003b_8790;
            set_pitch_setup(c, &mut r);
        });
        check_events(gen_set_pitch, hand_set_pitch, 600, &|c, seed| {
            let mut r = machine(c, seed, 23, 0);
            set_pitch_setup(c, &mut r);
            schedule(c, &mut r, seed, 0x003b_8790)
        });
    }

    fn set_pan_setup(c: &mut Cpu, r: &mut Rng) {
        let n = r.next_u32() % 16;
        let n = channels(c, r, n);
        let h = handle(c, r, n);
        c.g[A0][0] = r64!(r, h);
        c.g[A1][0] = r64!(r, (r.next_u32() % 0x100).wrapping_sub(0x80));
        let s = match r.next_u32() % 3 {
            0 => r.next_u32(),
            1 => (r.next_u32() % 16).wrapping_add(0x3ff8),
            _ => (r.next_u32() % 16).wrapping_sub(0x4008),
        };
        c.g[A2][0] = r64!(r, s);
    }

    driven!(gen_set_pan, s13::f_003b8c00__generated);
    driven!(hand_set_pan, snd_set_pan);

    #[test]
    fn set_pan() {
        check_lohi(s13::f_003b8c00__generated, snd_set_pan, 400, &|c, seed| {
            let mut r = machine(c, seed, 24, 0);
            c.pc = 0x003b_8c00;
            set_pan_setup(c, &mut r);
        });
        check_events(gen_set_pan, hand_set_pan, 600, &|c, seed| {
            let mut r = machine(c, seed, 25, 0);
            set_pan_setup(c, &mut r);
            schedule(c, &mut r, seed, 0x003b_8c00)
        });
    }

    // --- SND mutex and service ---------------------------------------------

    fn mutex_setup(c: &mut Cpu, r: &mut Rng) {
        let tid = 1 + r.next_u32() % 4;
        let owner = match r.next_u32() % 3 {
            0 => tid,
            1 => u32::MAX,
            _ => 1 + r.next_u32() % 4,
        };
        c.bus.write32(LOG_RESULTS, tid);
        c.bus.write32(snd_mutex::BASE + snd_mutex::OWNER, owner);
        c.bus.write32(snd_mutex::BASE + snd_mutex::COUNT, (r.next_u32() % 4).wrapping_sub(1));
        c.bus.write32(snd_mutex::BASE + snd_mutex::SEMA, r.next_u32() % 8);
    }

    #[test]
    fn mutex_lock_unlock() {
        for (g, h) in [
            (s13::f_003c3380__generated as fn(&mut Cpu), snd_mutex_lock as fn(&mut Cpu)),
            (s13::f_003c33e0__generated, snd_mutex_unlock),
            (s13::f_003b5860__generated, snd_default_lock),
            (s13::f_003b5880__generated, snd_default_unlock),
        ] {
            covered(g, h, 400, &|c, seed| {
                let mut r = machine(c, seed, 26, 2);
                mutex_setup(c, &mut r);
            });
        }
    }

    fn service_setup(c: &mut Cpu, r: &mut Rng) {
        let base = snd::STATE.0;
        let init = r.next_u32() % 5 != 0;
        c.bus.write8(base + snd_state::INITIALIZED, init as u8);
        c.bus.write16(base + snd_state::CHANNEL_COUNT, 0);
        let rate = match r.next_u32() % 4 {
            0 => 60,
            1 => 50,
            2 => 1 + r.next_u32() % 200,
            _ => r.next_u32() | 0x100,
        };
        let rate = if !init && r.next_u32() % 2 == 0 { 0 } else { rate };
        let old = if r.next_u32() % 2 == 0 { rate } else { r.next_u32() % 100 };
        c.bus.write16(base + snd_state::SERVICE_RATE, old as u16);
        let calls = r.next_u32() % 12;
        c.bus.write32(base + snd_state::SERVICE_CALLS, calls);
        let due = (calls + 1) * 100 / rate.max(1);
        c.bus.write32(base + snd_state::TICKS, due.saturating_sub(r.next_u32() % 4) + r.next_u32() % 2);
        // Tick callbacks (fakes).
        let cbs = r.next_u32() % 3;
        c.bus.write8(base + 0x177, cbs as u8);
        for i in 0..cbs {
            c.bus.write32(base + 0x184 + 4 * i, FAKE + 0x400 + 0x10 * i);
        }
        c.g[A0][0] = r64!(r, rate);
    }

    driven!(gen_service, s13::f_003b5540__generated);
    driven!(hand_service, snd_service);

    #[test]
    fn service() {
        covered(s13::f_003b5540__generated, snd_service, 600, &|c, seed| {
            let mut r = machine(c, seed, 27, 6);
            service_setup(c, &mut r);
        });
        check_lohi(s13::f_003b5540__generated, snd_service, 300, &|c, seed| {
            let mut r = machine(c, seed, 28, 0);
            service_setup(c, &mut r);
        });
        check_events(gen_service, hand_service, 600, &|c, seed| {
            let mut r = machine(c, seed, 29, 0);
            service_setup(c, &mut r);
            schedule(c, &mut r, seed, 0x003b_5540)
        });
    }

}
