//! Work package E2: `SSXAudioSystem` and the bank monitors
//! (`docs/investigation/phase2-candidates-3.md`).
//!
//! The main-thread side of SSX 3's audio, above the EA sound driver:
//!
//! - `SSXAudioSystem_Update`, the per-frame entry: screen-tint flash sound,
//!   banks and streams, the pause fade of streams, fades and mix ramps, and
//!   in a race the music, tracked sounds, object loops, boost meter,
//!   countdown beeps and loop effects;
//! - the board loops of a rider (base, surface, edge; local and remote
//!   riders): sound type changes restart the loop, the loop parameters come
//!   from 5-point curves over the rider's speed (`Audio_EvalCurve5`), and
//!   replays or a paused game mute them;
//! - `FindRider` (the player's rider, the first human or a rider by id),
//!   world-paint mix presets, music fade-in, the race countdown beeps, the
//!   speech instance and song instance updates;
//! - `BankMonitorInstance` (one playing sound effect): done test, 3D update
//!   (distance falloff, doppler, pan), pan and volume forwarding to an SND
//!   handle, a MIDI player or a stream buffer; and the `BankMonitor` timers
//!   (delayed callbacks through gcc 2.9x pointers to member functions).
//!
//! Every replacement charges the EE cycles block by block along the path it
//! takes, makes the same calls with the same registers (the call protocol
//! of [`crate::call`]), checks events at the same back edges (the shapes in
//! [`crate::events`]) and leaves the same incidental register values (lines
//! marked "leftover"). The views below repeat offsets of `audio.rs` (D7)
//! where they overlap: its view modules are private.

use crate::abi::{self, addu, reg, set_reg};
use crate::call::{call_ptr, guest, Callee, Flow, Frame, Unwound, VSlot};
use crate::events::{back_edge_unwinds, Resume};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this package: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0028_5bf8, ssx_audio_update, "SSXAudioSystem_Update"),
    (0x002a_c868, bank_monitor_instance_update_3d, "BankMonitorInstance_Update3D"),
    (0x0029_2ef8, ssx_audio_update_board_surface_loop_remote, "SSXAudioSystem_UpdateBoardSurfaceLoopRemote"),
    (0x002a_ba40, bank_monitor_instance_is_done, "BankMonitorInstance_IsDone"),
    (0x0028_5d98, ssx_audio_find_rider, "SSXAudioSystem_FindRider"),
    (0x0029_3538, ssx_audio_calc_surface_loop_group0, "SSXAudioSystem_CalcSurfaceLoopGroup0"),
    (0x002b_21e0, song_instance_update, "SongInstance_Update"),
    (0x0028_98a8, ssx_audio_update_rider_world_paint, "SSXAudioSystem_UpdateRiderWorldPaint"),
    (0x0028_9c98, ssx_audio_apply_mix_paint, "SSXAudioSystem_ApplyMixPaint"),
    (0x0029_3350, ssx_audio_calc_surface_loop_params_remote, "SSXAudioSystem_CalcSurfaceLoopParamsRemote"),
    (0x002a_fc88, speech_instance_update, "SpeechInstance_Update"),
    (0x0029_92d8, ssx_audio_stop_boost_sound, "SSXAudioSystem_StopBoostSound"),
    (0x002a_bf60, bank_monitor_instance_set_volume, "BankMonitorInstance_SetVolume"),
    (0x0028_f000, ssx_audio_update_music_fade_in, "SSXAudioSystem_UpdateMusicFadeIn"),
    (0x0029_3168, ssx_audio_calc_board_base_loop_params, "SSXAudioSystem_CalcBoardBaseLoopParams"),
    (0x0029_3290, ssx_audio_calc_surface_loop_params, "SSXAudioSystem_CalcSurfaceLoopParams"),
    (0x0029_3410, ssx_audio_calc_edge_loop_params, "SSXAudioSystem_CalcEdgeLoopParams"),
    (0x0029_3bb0, ssx_audio_calc_edge_loop_group0, "SSXAudioSystem_CalcEdgeLoopGroup0"),
    (0x0029_cca8, ssx_audio_update_countdown_beeps, "SSXAudioSystem_UpdateCountdownBeeps"),
    (0x002a_b828, bank_monitor_timer_update, "BankMonitor_TimerUpdate"),
    (0x002a_bd90, bank_monitor_instance_set_pan, "BankMonitorInstance_SetPan"),
    (0x0029_2cb8, ssx_audio_update_board_base_loop, "SSXAudioSystem_UpdateBoardBaseLoop"),
    (0x0029_2da0, ssx_audio_update_board_surface_loop, "SSXAudioSystem_UpdateBoardSurfaceLoop"),
    (0x0029_3018, ssx_audio_update_board_edge_loop, "SSXAudioSystem_UpdateBoardEdgeLoop"),
    (0x002a_dca0, bank_monitor_add_timer, "BankMonitor_AddTimer"),
];

// ---------------------------------------------------------------------------
// Callees.

/// Direct calls (`jal` into the same shard: through the generated function,
/// which goes through its override slot when it has one).
mod callee {
    use super::{ee, Callee};

    pub const FIND_FREE_TIMER: Callee = Callee { addr: 0x002a_dc30, f: ee::s08::BankMonitor_FindFreeTimer };
    pub const TIMER_SET: Callee = Callee { addr: 0x002a_d888, f: ee::s08::BankMonitor_TimerSet };
    pub const TIMER_KILL: Callee = Callee { addr: 0x002a_d940, f: ee::s08::BankMonitor_TimerKill };
    pub const STOP_HANDLE: Callee = Callee { addr: 0x002a_d5f0, f: ee::s08::BankMonitor_StopHandle };
    pub const CALC_DOPPLER_PITCH: Callee = Callee { addr: 0x002a_bca0, f: ee::s08::BankMonitor_CalcDopplerPitch };
    pub const BMI_UPDATE_RANGE: Callee = Callee { addr: 0x002a_c7f0, f: ee::s08::BankMonitorInstance_UpdateRange };
    pub const BMI_SET_PAN: Callee = Callee { addr: 0x002a_bd90, f: ee::s08::BankMonitorInstance_SetPan };
    pub const BMI_SET_PITCH: Callee = Callee { addr: 0x002a_be78, f: ee::s08::BankMonitorInstance_SetPitch };
    pub const BMI_SET_VOLUME: Callee = Callee { addr: 0x002a_bf60, f: ee::s08::BankMonitorInstance_SetVolume };
    pub const BMI_GET_FALLOFF_VOLUME: Callee =
        Callee { addr: 0x002a_bc80, f: ee::s08::BankMonitorInstance_GetFalloffVolume };
    pub const BMI_IS_DONE: Callee = Callee { addr: 0x002a_ba40, f: ee::s08::BankMonitorInstance_IsDone };
    pub const STREAM_BUFFER_SET_PAN: Callee = Callee { addr: 0x002a_9ed8, f: ee::s08::StreamBuffer_SetPan };
    pub const STREAM_BUFFER_SET_VOLUME: Callee = Callee { addr: 0x002a_9f30, f: ee::s08::f_002a9f30 };
    pub const STREAM_BUFFER_STOP: Callee = Callee { addr: 0x002a_a408, f: ee::s08::StreamBuffer_Stop };
    pub const SPEECH_INSTANCE_POLL: Callee = Callee { addr: 0x002b_0290, f: ee::s08::f_002b0290 };
    pub const MUSIC_IS_SONG_DATA_LOADED: Callee =
        Callee { addr: 0x002b_4aa0, f: ee::s08::MusicPlayer_IsSongDataLoaded };
    pub const MUSIC_GET_SONG_DATA_BUFFER: Callee =
        Callee { addr: 0x002b_4ae8, f: ee::s08::MusicPlayer_GetSongDataBuffer };
    pub const SONG_POST_EVENT: Callee = Callee { addr: 0x002b_20c8, f: ee::s08::SongInstance_PostEvent };
    pub const SONG_SET_VOLUME: Callee = Callee { addr: 0x002b_2160, f: ee::s08::SongInstance_SetVolume };
    pub const SONG_GET_STREAM_HANDLE: Callee =
        Callee { addr: 0x002b_2488, f: ee::s08::SongInstance_GetStreamHandle };
    pub const ADD_TIMER: Callee = Callee { addr: 0x002a_dca0, f: ee::s08::BankMonitor_AddTimer };

    pub const GET_INSTANCE: Callee = Callee { addr: 0x0028_b180, f: ee::s07::SSXAudioSystem_GetInstance };
    pub const GET_APP: Callee = Callee { addr: 0x0028_b1c0, f: ee::s07::SSXAudioSystem_GetApp };
    pub const GET_GAME_MODULE: Callee = Callee { addr: 0x0028_b1c8, f: ee::s07::SSXAudioSystem_GetGameModule };
    pub const GET_RACE_MAN: Callee = Callee { addr: 0x0028_b1d8, f: ee::s07::SSXAudioSystem_GetRaceMan };
    pub const IS_REPLAY_PLAYING: Callee =
        Callee { addr: 0x0028_8ae0, f: ee::s07::SSXAudioSystem_IsReplayPlaying };
    pub const FIND_RIDER: Callee = Callee { addr: 0x0028_5d98, f: ee::s07::SSXAudioSystem_FindRider };
    pub const SET_MIX_PRESET: Callee = Callee { addr: 0x0028_83b0, f: ee::s07::SSXAudioSystem_SetMixPreset };
    pub const APPLY_MIX_PAINT: Callee = Callee { addr: 0x0028_9c98, f: ee::s07::SSXAudioSystem_ApplyMixPaint };
    pub const UPDATE_FADE: Callee = Callee { addr: 0x0028_7c48, f: ee::s07::SSXAudioSystem_UpdateFade };
    pub const UPDATE_MIX_RAMP: Callee = Callee { addr: 0x0028_87f8, f: ee::s07::SSXAudioSystem_UpdateMixRamp };
    pub const UPDATE_MUSIC_FADE_IN: Callee =
        Callee { addr: 0x0028_f000, f: ee::s07::SSXAudioSystem_UpdateMusicFadeIn };
    pub const UPDATE_PEAK_MUSIC: Callee = Callee { addr: 0x0028_e100, f: ee::s07::SSXAudioSystem_UpdatePeakMusic };
    pub const RESUME_GAME_AUDIO: Callee =
        Callee { addr: 0x0028_9bb8, f: ee::s07::SSXAudioSystem_ResumeGameAudio };
    pub const START_FADE: Callee = Callee { addr: 0x0028_7f00, f: ee::s07::f_00287f00 };
    pub const TINT_FLASH_SOUND: Callee = Callee { addr: 0x0029_1438, f: ee::s07::f_00291438 };
    pub const UPDATE_STATE_FLAGS: Callee = Callee { addr: 0x0028_c8c8, f: ee::s07::f_0028c8c8 };
    pub const UPDATE_RACE_MUSIC_A: Callee = Callee { addr: 0x0029_47b0, f: ee::s07::f_002947b0 };
    pub const UPDATE_RACE_MUSIC_B: Callee = Callee { addr: 0x0029_48a0, f: ee::s07::f_002948a0 };
    pub const UPDATE_END: Callee = Callee { addr: 0x0028_9688, f: ee::s07::f_00289688 };
    pub const RIDER_PAINT_CHANGED: Callee = Callee { addr: 0x0028_d960, f: ee::s07::f_0028d960 };
    pub const RIDER_PAINT_SET: Callee = Callee { addr: 0x0028_d988, f: ee::s07::f_0028d988 };
    pub const RIDER_PAINT2_CHANGED: Callee = Callee { addr: 0x0028_99e8, f: ee::s07::f_002899e8 };
    pub const RIDER_PAINT2_SET: Callee = Callee { addr: 0x0028_99f8, f: ee::s07::f_002899f8 };
    pub const RIDER_MUSIC_STATE: Callee = Callee { addr: 0x0028_9c18, f: ee::s07::f_00289c18 };
    pub const REPLAY_IS_PAUSED: Callee = Callee { addr: 0x0027_0280, f: ee::s07::f_00270280 };
    pub const SURFACE_GROUP: Callee = Callee { addr: 0x0029_34d0, f: ee::s07::f_002934d0 };
    pub const GET_SURFACE_SOUND_TYPE: Callee =
        Callee { addr: 0x0029_1710, f: ee::s07::SSXAudioSystem_GetSurfaceSoundType };
    pub const START_BOARD_SURFACE_LOOP: Callee =
        Callee { addr: 0x0029_1c88, f: ee::s07::SSXAudioSystem_StartBoardSurfaceLoop };
    pub const START_BOARD_EDGE_LOOP: Callee =
        Callee { addr: 0x0029_2508, f: ee::s07::SSXAudioSystem_StartBoardEdgeLoop };
    pub const CALC_BOARD_BASE_LOOP_PARAMS: Callee =
        Callee { addr: 0x0029_3168, f: ee::s07::SSXAudioSystem_CalcBoardBaseLoopParams };
    pub const CALC_SURFACE_LOOP_PARAMS: Callee =
        Callee { addr: 0x0029_3290, f: ee::s07::SSXAudioSystem_CalcSurfaceLoopParams };
    pub const CALC_SURFACE_LOOP_PARAMS_REMOTE: Callee =
        Callee { addr: 0x0029_3350, f: ee::s07::SSXAudioSystem_CalcSurfaceLoopParamsRemote };
    pub const CALC_EDGE_LOOP_PARAMS: Callee =
        Callee { addr: 0x0029_3410, f: ee::s07::SSXAudioSystem_CalcEdgeLoopParams };
    pub const SURFACE_GROUPS: [Callee; 3] = [
        Callee { addr: 0x0029_3538, f: ee::s07::SSXAudioSystem_CalcSurfaceLoopGroup0 },
        Callee { addr: 0x0029_3740, f: ee::s07::SSXAudioSystem_CalcSurfaceLoopGroup1 },
        Callee { addr: 0x0029_3948, f: ee::s07::SSXAudioSystem_CalcSurfaceLoopGroup2 },
    ];
    pub const SURFACE_REMOTE_GROUPS: [Callee; 3] = [
        Callee { addr: 0x0029_3b50, f: ee::s07::SSXAudioSystem_CalcSurfaceLoopRemoteGroup0 },
        Callee { addr: 0x0029_3b70, f: ee::s07::SSXAudioSystem_CalcSurfaceLoopRemoteGroup1 },
        Callee { addr: 0x0029_3b90, f: ee::s07::SSXAudioSystem_CalcSurfaceLoopRemoteGroup2 },
    ];
    pub const EDGE_GROUPS: [Callee; 3] = [
        Callee { addr: 0x0029_3bb0, f: ee::s07::SSXAudioSystem_CalcEdgeLoopGroup0 },
        Callee { addr: 0x0029_3dc0, f: ee::s07::SSXAudioSystem_CalcEdgeLoopGroup1 },
        Callee { addr: 0x0029_3fc8, f: ee::s07::SSXAudioSystem_CalcEdgeLoopGroup2 },
    ];
    pub const EVAL_CURVE5: Callee = Callee { addr: 0x0029_0b58, f: ee::s07::Audio_EvalCurve5 };
}

/// Calls the compiler emitted as `call_indirect` (cross-shard `jal`s,
/// resolved through the function table).
mod table {
    pub const SND_LOCK: u32 = 0x003b_58a0;
    pub const SND_UNLOCK: u32 = 0x003b_58d8;
    pub const SND_IS_HANDLE_DONE: u32 = 0x003b_8ab8;
    pub const SND_SET_PAN: u32 = 0x003b_8c00;
    pub const SND_SET_VOLUME: u32 = 0x003b_85f0;
    pub const SND_STOP: u32 = 0x003b_89e0;
    pub const SNDMIDI_GET_STATUS: u32 = 0x003b_9fc8;
    pub const SNDMIDI_SET_PAN: u32 = 0x003b_9c20;
    pub const SNDMIDI_SET_VOLUME: u32 = 0x003b_9e48;
    pub const SNDMIDI_STOP: u32 = 0x003b_9920;
    pub const SNDSTRM_GET_REQUEST_STATUS: u32 = 0x003b_7c40;
    pub const SNDSTRM_SET_VOLUME: u32 = 0x003b_7e70;
    pub const PATH_ADD_MAP: u32 = 0x003d_2350;
    pub const PATH_ADD_TRACK: u32 = 0x003d_25f0;
    pub const SPRINTF: u32 = 0x0041_7828;
    pub const FIXUNSSFSI: u32 = 0x0041_39f8;
    pub const SPEECH_STREAM_DONE: u32 = 0x003d_7ec8;
    pub const SPEECH_STREAM_START: u32 = 0x003d_8040;

    pub const RIDER_GET_MAIN_STATE: u32 = 0x0011_fe98;
    pub const TINT_POLL_FLASH_SOUND: u32 = 0x0039_0ef8;
    pub const UPDATE_BANKS_AND_STREAMS: u32 = 0x002a_b958;
    pub const STREAM_ARRAY_IS_PLAYING: u32 = 0x002a_b028;
    pub const BMI_RESET: u32 = 0x002a_bb38;
    pub const BMI_SET_PITCH: u32 = 0x002a_be78;
    pub const IS_REPLAY_PLAYING: u32 = 0x0028_8ae0;
    pub const GET_RACE_MAN: u32 = 0x0028_b1d8;
    pub const GET_CHANNEL_GAIN_PTR: u32 = 0x0028_7968;
    pub const UPDATE_TRACKED_SOUNDS: u32 = 0x0029_5208;
    pub const UPDATE_OBJECT_LOOPS: u32 = 0x0029_7fa0;
    pub const UPDATE_RACE_SFX_A: u32 = 0x0029_d290;
    pub const UPDATE_BOOST_METER_SOUNDS: u32 = 0x0029_ab40;
    pub const UPDATE_RACE_SFX_B: u32 = 0x0029_b738;
    pub const UPDATE_COUNTDOWN_BEEPS: u32 = 0x0029_cca8;
    pub const UPDATE_LOOP_SFX: u32 = 0x0029_d610;
    pub const MUSIC_GET_STATE: u32 = 0x002b_49e0;
    pub const MUSIC_SET_VOLUME: u32 = 0x002b_3c28;
}

// ---------------------------------------------------------------------------
// Views.

/// `SSXAudioSystem` fields.
mod sys {
    /// `Snd**`: the sound root (`*root` is the bank / stream object).
    pub const ROOT: u32 = 0x118;
    /// `snd` (`*root`) + this: the bank monitor.
    pub const SND_BANK_MONITOR: u32 = 0x1d8;
    /// Boost sound: its handle and the frames left before it is stopped.
    pub const BOOST_HANDLE: u32 = 0x59e0;
    pub const BOOST_STOP_DELAY: u32 = 0x59e4;
    /// Per-player smoothed edge speed (floats, indexed by player).
    pub const EDGE_SPEED: u32 = 0x5990;
    /// Per-player smoothed surface speed.
    pub const SURFACE_SPEED: u32 = 0x59b8;
    /// Music state (2 = fading in) and the fade-in frame counter.
    pub const MUSIC_FADE_STATE: u32 = 0x6288;
    pub const MUSIC_FADE_FRAMES: u32 = 0x628c;
    /// Per-player world-paint objects (three sets).
    pub const PAINT_A: u32 = 0x625c;
    pub const PAINT_B: u32 = 0x6c80;
    pub const PAINT_C: u32 = 0x5fa0;
    /// Music already started (skips the fade-in).
    pub const MUSIC_STARTED: u32 = 0x5fb4;
    /// The last countdown second a beep was queued for.
    pub const COUNTDOWN_LAST: u32 = 0x607c;
    /// Stream-pause fade state (3 = done) and the "streams were playing"
    /// latch.
    pub const PAUSE_FADE_STATE: u32 = 0x63d4;
    pub const STREAMS_PLAYING: u32 = 0x63e0;
    /// Game mode (6 = race).
    pub const MODE: u32 = 0x6474;
}

/// Rider fields the audio code reads.
mod rider {
    /// Rank / distance key the player search minimises.
    pub const RANK: u32 = 0xec;
    /// Floats: speed components and speed.
    pub const SPEED_A: u32 = 0x750;
    pub const SPEED_B: u32 = 0x754;
    pub const SPEED_C: u32 = 0x758;
    pub const SPEED: u32 = 0x75c;
    /// The main state the audio last saw.
    pub const AUDIO_STATE: u32 = 0x760;
    /// Current surface and edge sound types.
    pub const SURFACE_SOUND: u32 = 0x768;
    pub const EDGE_SOUND: u32 = 0x76c;
    /// The sub-object with the player-index method (vptr at +0).
    pub const CONTROLLER: u32 = 0x6c0;
    /// Player id, human flag, local flag.
    pub const ID: u32 = 0x870;
    pub const HUMAN: u32 = 0x874;
    pub const LOCAL: u32 = 0x87c;
    /// Air time (float; > 0 while airborne).
    pub const AIR: u32 = 0x470;
}

/// `BankMonitorInstance` (0xc0 bytes): one playing sound effect.
mod bmi {
    /// What plays: 0 free, 1 an SND handle, 2 a MIDI player, 3 a stream
    /// buffer.
    pub const STATE: u32 = 0x00;
    /// The SND / MIDI handle.
    pub const HANDLE: u32 = 0x04;
    pub const FLAGS: u32 = 0x08;
    /// The stream buffer (state 3).
    pub const STREAM: u32 = 0x0c;
    /// Releasing: the voice is let run out (no volume updates; the
    /// instance is cleared once it is done).
    pub const RELEASING: u32 = 0x54;
    /// Last listener distance (float).
    pub const DISTANCE: u32 = 0x5c;
    /// `float*` extra gain, or null.
    pub const GAIN_PTR: u32 = 0x60;
    /// Base volume (0..127).
    pub const VOLUME: u32 = 0x64;
    /// Positional (3D) flag.
    pub const POSITIONAL: u32 = 0x68;
    /// Falloff radius (float; 0 = default 30).
    pub const RADIUS: u32 = 0x88;
    /// Channel gain (float).
    pub const GAIN: u32 = 0x90;
    /// Non-zero: no range update / falloff (fixed volume).
    pub const FIXED: u32 = 0x94;
    /// Volume scale (0x7f = unity).
    pub const VOLUME_SCALE: u32 = 0x98;
    /// Last volume and pan sent.
    pub const LAST_VOLUME: u32 = 0xb4;
    pub const LAST_PAN: u32 = 0xb8;
    /// The owning rider (board loops).
    pub const RIDER: u32 = 0x80;
}

/// `BankMonitor` (the manager at `snd + 0x1d8`).
mod bank_monitor {
    /// The listener interface (vptr; slot +0x30 measures a sound), the
    /// doppler flag and the 3D flag.
    pub const LISTENER: u32 = 0x8d8;
    pub const DOPPLER: u32 = 0x8e0;
    pub const ENABLE_3D: u32 = 0x8e4;
    /// Active timer count and the timer array (0x2c bytes each).
    pub const TIMER_COUNT: u32 = 0x90;
    pub const TIMERS: u32 = 0x98;
    pub const TIMER_SIZE: u32 = 0x2c;
}

/// `BankMonitor` timer (0x2c bytes).
mod timer {
    pub const ACTIVE: u32 = 0x00;
    pub const PERIOD: u32 = 0x04;
    /// Milliseconds left (counts down by 10 per update).
    pub const LEFT: u32 = 0x08;
    /// The callback: a gcc 2.9x pointer to member function
    /// `{ short delta; short index; union { short vptr_offset; void *fn; } }`.
    pub const PMF: u32 = 0x0c;
}

/// gp-relative globals.
mod gp {
    /// Pause / menu flags (bit 0: paused, bit 1: frontend).
    pub const PAUSE_FLAGS: u32 = 0x6a0;
    /// The game object (`+0x84` the game module, `+0xc0` the countdown).
    pub const GAME: u32 = 0x848;
    /// The music player (`gp + 0x6fc`) and the speech listener (`gp + 0x6c4`).
    pub const MUSIC_PLAYER: u32 = 0x6fc;
    pub const SPEECH_LISTENER: u32 = 0x6c4;
}

// ---------------------------------------------------------------------------
// Small helpers.

/// `lw base+off` (CPU path, sign-extended).
#[inline(always)]
fn lw_at(c: &mut Cpu, base: u64, off: u32) -> u64 {
    Addr((base as u32).wrapping_add(off)).lw(c)
}

/// `sw v, off(base)`.
#[inline(always)]
fn sw_at(c: &mut Cpu, base: u64, off: u32, v: u64) {
    Addr((base as u32).wrapping_add(off)).sw(c, v)
}

/// `lw` at `gp + off` (negative offsets as `wrapping_sub`).
#[inline(always)]
fn gp_lw(c: &mut Cpu, off: i32) -> u64 {
    Addr((reg(c, GP) as u32).wrapping_add(off as u32)).lw(c)
}

/// `lwc1` at `gp + off` (a float constant).
#[inline(always)]
fn gp_float(c: &mut Cpu, off: i32) -> FpuF32 {
    Addr((reg(c, GP) as u32).wrapping_add(off as u32)).read_fpu(c)
}

/// `lwc1` at `base + off`.
#[inline(always)]
fn lwc1_at(c: &mut Cpu, base: u64, off: u32) -> FpuF32 {
    Addr((base as u32).wrapping_add(off)).read_fpu(c)
}

/// `slt`/`slti` on whole registers.
#[inline(always)]
fn slt(a: u64, b: u64) -> u64 {
    ((a as i64) < (b as i64)) as u64
}

/// `addiu` on a register (32-bit, sign-extended).
#[inline(always)]
fn addiu(a: u64, imm: i32) -> u64 {
    sx((a as u32).wrapping_add(imm as u32))
}

/// An FPU register as a value.
#[inline(always)]
fn fr(c: &Cpu, r: usize) -> FpuF32 {
    FpuF32(c.f[r])
}

/// `cvt.w.s` then `mfc1`: the truncated integer of `v`, sign-extended
/// (the word is left in FPU register `tmp`).
#[inline]
fn to_int_via(c: &mut Cpu, tmp: usize, v: FpuF32) -> u64 {
    let w = FpuF32(v.to_i32() as u32);
    abi::set_fpr(c, tmp, w);
    abi::mfc1(w)
}

/// Save callee-saved GPRs (`sq`) at the given frame offsets.
fn save_regs(c: &mut Cpu, frame: Frame, regs: &[(usize, u32)]) {
    for &(r, off) in regs {
        frame.save(c, r, off);
    }
}

/// Restore callee-saved GPRs (`lq`).
fn restore_regs(c: &mut Cpu, frame: Frame, regs: &[(usize, u32)]) {
    for &(r, off) in regs {
        frame.restore(c, r, off);
    }
}

/// A virtual call through slot `off` of the vtable at `vt` on the object
/// `obj`: `a0 = obj + delta`, the function left in `fn_reg`, returning to
/// `ret`.
fn virtual_call(c: &mut Cpu, obj: u64, vt: u64, off: u32, fn_reg: usize, ret: u32) -> Flow {
    let m = VSlot::read(c, vt, off);
    set_reg(c, fn_reg, m.func);
    set_reg(c, A0, m.this(obj));
    call_ptr(c, m.func as u32, ret)
}

/// Copy a 5-point curve (40 bytes, five `ldl/ldr` + `sdl/sdr` doubleword
/// pairs) from `src` to `dst`. The doublewords pass through `regs[0..3]`
/// (first three) and `regs[0..2]` (last two), which keep the last values.
fn copy_curve(c: &mut Cpu, src: u32, dst: u32, regs: [usize; 3]) {
    let mut d = [0u64; 5];
    for (i, w) in d.iter_mut().enumerate() {
        *w = Addr(src + 8 * i as u32).read_u64_unaligned(c);
    }
    for (i, w) in d.iter().enumerate() {
        Addr(dst + 8 * i as u32).write_u64_unaligned(c, *w);
    }
    set_reg(c, regs[2], d[2]);
    set_reg(c, regs[0], d[3]);
    set_reg(c, regs[1], d[4]);
}

// ---------------------------------------------------------------------------
// BankMonitor timers.

/// `BankMonitor_AddTimer(bm, delay, pmf, a3, t0, t1, t2, t3)` 0x2adca0:
/// start a timer: count it, take a free slot (`BankMonitor_FindFreeTimer`,
/// which also sees the count) and set it up with the arguments
/// (`BankMonitor_TimerSet`; `pmf` is the 8-byte pointer to member function
/// passed in a2, spilled to the frame and reloaded). The count is dropped
/// again afterwards. Returns the slot, or a negative value when none was
/// free.
pub fn bank_monitor_add_timer(c: &mut Cpu) {
    const SAVED: [(usize, u32); 8] =
        [(S0, 0x90), (S1, 0x80), (S2, 0x70), (S3, 0x60), (S4, 0x50), (S5, 0x40), (S6, 0x30), (S7, 0x20)];
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0xa0);
        save_regs(c, frame, &SAVED);
        frame.save_ra(c, 0x10);
        let bm = reg(c, A0);
        let (delay, pmf) = (reg(c, A1), reg(c, A2));
        let (a3, t0, t1, t2, t3) = (reg(c, A3), reg(c, T0), reg(c, T1), reg(c, T2), reg(c, T3));
        set_reg(c, S0, bm);
        set_reg(c, S2, t2);
        set_reg(c, S3, t3);
        set_reg(c, S4, t1);
        set_reg(c, S5, t0);
        set_reg(c, S6, a3);
        set_reg(c, S7, delay);
        Addr(frame.at(c, 0)).sd(c, pmf);
        let count = addiu(lw_at(c, bm, bank_monitor::TIMER_COUNT), 1);
        set_reg(c, V0, count);
        sw_at(c, bm, bank_monitor::TIMER_COUNT, count);
        let count = lw_at(c, bm, bank_monitor::TIMER_COUNT);
        set_reg(c, A1, count);
        callee::FIND_FREE_TIMER.call(c, 0x002a_dd00)?;
        let slot = reg(c, V0);
        set_reg(c, S1, slot);
        set_reg(c, A1, delay);
        if (slot as i64) >= 0 {
            c.cycles += 11;
            let pmf = Addr(frame.at(c, 0)).ld(c);
            set_reg(c, A2, pmf);
            let off = sx(abi::mult(c, slot as u32, bank_monitor::TIMER_SIZE));
            set_reg(c, A3, a3);
            set_reg(c, T0, t0);
            set_reg(c, T1, t1);
            set_reg(c, T2, t2);
            set_reg(c, T3, t3);
            set_reg(c, A0, addu(bm, addiu(off, bank_monitor::TIMERS as i32)));
            callee::TIMER_SET.call(c, 0x002a_dd38)?;
        }
        c.cycles += 15;
        let count = addiu(lw_at(c, bm, bank_monitor::TIMER_COUNT), -1);
        set_reg(c, V1, count); // leftover
        set_reg(c, V0, slot);
        sw_at(c, bm, bank_monitor::TIMER_COUNT, count);
        restore_regs(c, frame, &SAVED);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `BankMonitor_TimerUpdate(timer, owner)` 0x2ab828: count an active
/// timer down by 10 ms; once it is below 5, call its callback (a pointer to
/// member function of `owner`, with the timer as argument). A non-zero
/// result kills the timer, zero re-arms it with its period.
pub fn bank_monitor_timer_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let t = reg(c, A0);
        set_reg(c, S0, t);
        let active = lw_at(c, t, timer::ACTIVE);
        set_reg(c, V0, active);
        if active != 0 {
            c.cycles += 5;
            let left = addiu(lw_at(c, t, timer::LEFT), -10);
            set_reg(c, V0, left);
            let due = slt(left, 5);
            set_reg(c, V1, due);
            sw_at(c, t, timer::LEFT, left);
            if due != 0 {
                pmf_timer_call(c, t)?;
                if reg(c, V0) == 0 {
                    // Re-arm.
                    let left = lw_at(c, t, timer::LEFT);
                    c.cycles += 3;
                    let period = lw_at(c, t, timer::PERIOD);
                    set_reg(c, V1, period);
                    let left = addu(left, period);
                    set_reg(c, V0, left);
                    sw_at(c, t, timer::LEFT, left);
                } else {
                    c.cycles += 4;
                    set_reg(c, A0, t);
                    callee::TIMER_KILL.call(c, 0x002a_b8c4)?;
                    frame.restore(c, S0, 0x10);
                    c.cycles += 3;
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
            }
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The timer callback of [`bank_monitor_timer_update`]: the pointer to
/// member function at `timer + 0xc` called on the owner (a1) with the timer.
/// A negative index is a plain function; otherwise the function is vtable
/// entry `index - 1` (`ld` into t0: `{ short delta; short; void *fn; }`) of
/// the vtable at `owner + vptr_offset`, and both deltas apply.
fn pmf_timer_call(c: &mut Cpu, t: u64) -> Flow {
    let owner = reg(c, A1);
    let pmf = (t as u32).wrapping_add(timer::PMF);
    c.cycles += 3;
    let index = Addr(pmf + 2).lh(c);
    set_reg(c, A2, index);
    let scaled = sx((index as u32) << 3);
    set_reg(c, V1, scaled);
    let virtual_ = (index as i64) >= 0;
    let func = if virtual_ {
        c.cycles += 8;
        let vptr_offset = Addr(pmf + 4).lh(c);
        let vptr_at = addu(owner, vptr_offset);
        set_reg(c, V0, vptr_at);
        let vt = lw_at(c, vptr_at, 0);
        set_reg(c, A0, vt);
        let entry = addu(scaled, vt);
        set_reg(c, V1, entry);
        let slot = c.ld((entry as u32).wrapping_sub(8));
        set_reg(c, T0, slot);
        ((slot as i64) >> 32) as u64
    } else {
        c.cycles += 2;
        Addr(pmf + 4).lw(c)
    };
    set_reg(c, A3, func);
    let delta = Addr(pmf).lh(c);
    set_reg(c, V1, delta);
    c.cycles += 2;
    let low = reg(c, T0) & 0xffff;
    set_reg(c, V0, low);
    let this_delta = if virtual_ {
        c.cycles += 4;
        let vdelta = sx((((low as u32) << 16) as i32 >> 16) as u32);
        addu(vdelta, delta)
    } else {
        c.cycles += 1;
        delta
    };
    set_reg(c, V0, this_delta);
    c.cycles += 5;
    set_reg(c, A0, addu(owner, this_delta));
    set_reg(c, A1, t);
    call_ptr(c, func as u32, 0x002a_b8b4)
}

// ---------------------------------------------------------------------------
// The per-frame update.

/// `SSXAudioSystem_Update(dt f12, sys)` 0x285bf8: the audio system's
/// per-frame update.
///
/// - the screen-tint flash sound (`VfxScreenTint_PollFlashSound`; when it
///   fires, start it on the global audio system with `dt = 0`);
/// - banks and streams (`SSXAudioSystem_UpdateBanksAndStreams`);
/// - unless the stream-pause fade is done (state 3): latch whether the
///   streams play; once they stopped after playing, start the fade
///   (`f_00287f00(sys, 3, 0.5, 1.0)`);
/// - fades and mix ramps; in game: the state flags and, in a race (mode 6),
///   the music fade-in (until the music started), tracked sounds, the race
///   music (mode 4 of the game module or not), object loops, peak music,
///   the race effects, boost meter, countdown beeps and loop effects, and
///   resuming game audio when the music started and the frontend flag is
///   clear;
/// - the end-of-frame update (`f_00289688`).
///
/// `dt` is kept in f20 across the calls.
pub fn ssx_audio_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_fpr(c, 20, 0x20);
        let sys = reg(c, A0);
        set_reg(c, S0, sys);
        frame.save_ra(c, 0);
        c.f[20] = c.f[12];
        call_ptr(c, table::TINT_POLL_FLASH_SOUND, 0x0028_5c14)?;
        let flash = reg(c, V0) != 0;
        c.f[12] = c.f[20];
        if flash {
            c.cycles += 6;
            callee::GET_INSTANCE.call(c, 0x0028_5c24)?;
            c.f[12] = 0;
            set_reg(c, A0, reg(c, V0));
            callee::TINT_FLASH_SOUND.call(c, 0x0028_5c30)?;
            c.f[12] = c.f[20];
        }

        // Banks and streams; the stream-pause fade.
        c.cycles += 6;
        set_reg(c, A0, sys);
        call_ptr(c, table::UPDATE_BANKS_AND_STREAMS, 0x0028_5c3c)?;
        let fade_state = lw_at(c, sys, sys::PAUSE_FADE_STATE);
        set_reg(c, A1, fade_state);
        set_reg(c, V0, 3);
        if fade_state != 3 {
            c.cycles += 3;
            let was_playing = lw_at(c, sys, sys::STREAMS_PLAYING);
            set_reg(c, V0, was_playing);
            c.cycles += 4;
            let root = lw_at(c, sys, sys::ROOT);
            set_reg(c, A0, root);
            if was_playing == 0 {
                call_ptr(c, table::STREAM_ARRAY_IS_PLAYING, 0x0028_5c60)?;
                let playing = reg(c, V0) != 0;
                set_reg(c, V0, 1);
                if playing {
                    c.cycles += 2;
                    sw_at(c, sys, sys::STREAMS_PLAYING, 1);
                }
            } else {
                call_ptr(c, table::STREAM_ARRAY_IS_PLAYING, 0x0028_5c78)?;
                let playing = reg(c, V0) != 0;
                set_reg(c, A0, sys);
                if !playing {
                    c.cycles += 6;
                    c.f[12] = 0x3f00_0000;
                    c.f[13] = 0x3f80_0000;
                    set_reg(c, AT, 0x3f80_0000); // leftover
                    set_reg(c, A1, 3);
                    callee::START_FADE.call(c, 0x0028_5c98)?;
                }
            }
        }

        // Fades, mix ramps and the race.
        c.cycles += 9;
        set_reg(c, A0, sys);
        callee::UPDATE_FADE.call(c, 0x0028_5ca0)?;
        set_reg(c, A0, sys);
        callee::UPDATE_MIX_RAMP.call(c, 0x0028_5ca8)?;
        callee::GET_APP.call(c, 0x0028_5cb0)?;
        let module = lw_at(c, reg(c, V0), 0x84);
        set_reg(c, V1, module);
        if module != 0 {
            c.cycles += 6;
            set_reg(c, A0, sys);
            callee::UPDATE_STATE_FLAGS.call(c, 0x0028_5cc4)?;
            let mode = lw_at(c, sys, sys::MODE);
            set_reg(c, V1, mode);
            set_reg(c, V0, 6);
            if mode == 6 {
                race_update(c, sys)?;
            }
        }
        c.cycles += 7;
        set_reg(c, A0, sys);
        callee::UPDATE_END.call(c, 0x0028_5d80)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// The race part of [`ssx_audio_update`] (mode 6).
fn race_update(c: &mut Cpu, sys: u64) -> Flow {
    c.cycles += 3;
    let started = lw_at(c, sys, sys::MUSIC_STARTED);
    set_reg(c, V0, started);
    if started == 0 {
        c.cycles += 2;
        set_reg(c, A0, sys);
        callee::UPDATE_MUSIC_FADE_IN.call(c, 0x0028_5ce8)?;
    }
    c.cycles += 8;
    set_reg(c, A0, sys);
    call_ptr(c, table::UPDATE_TRACKED_SOUNDS, 0x0028_5cf0)?;
    callee::GET_GAME_MODULE.call(c, 0x0028_5cf8)?;
    let game_mode = lw_at(c, reg(c, V0), 0x214);
    set_reg(c, V1, game_mode);
    set_reg(c, V0, 4);
    set_reg(c, A0, sys);
    if game_mode == 4 {
        c.cycles += 4;
        callee::UPDATE_RACE_MUSIC_A.call(c, 0x0028_5d10)?;
    } else {
        c.cycles += 2;
        callee::UPDATE_RACE_MUSIC_B.call(c, 0x0028_5d20)?;
    }
    c.cycles += 17;
    set_reg(c, A0, sys);
    call_ptr(c, table::UPDATE_OBJECT_LOOPS, 0x0028_5d28)?;
    set_reg(c, A0, sys);
    callee::UPDATE_PEAK_MUSIC.call(c, 0x0028_5d30)?;
    for (target, ret) in [
        (table::UPDATE_RACE_SFX_A, 0x0028_5d38),
        (table::UPDATE_BOOST_METER_SOUNDS, 0x0028_5d40),
        (table::UPDATE_RACE_SFX_B, 0x0028_5d48),
        (table::UPDATE_COUNTDOWN_BEEPS, 0x0028_5d50),
        (table::UPDATE_LOOP_SFX, 0x0028_5d58),
    ] {
        set_reg(c, A0, sys);
        call_ptr(c, target, ret)?;
    }
    let started = lw_at(c, sys, sys::MUSIC_STARTED);
    let flags = gp_lw(c, -(gp::PAUSE_FLAGS as i32));
    set_reg(c, V0, flags);
    if started != 0 {
        c.cycles += 3;
        let frontend = flags & 2;
        set_reg(c, V0, frontend);
        if frontend == 0 {
            c.cycles += 2;
            set_reg(c, A0, sys);
            callee::RESUME_GAME_AUDIO.call(c, 0x0028_5d78)?;
        }
    }
    Ok(())
}

/// `SSXAudioSystem_ApplyMixPaint(sys, rider, paint)` 0x289c98: when
/// `rider` is the player's rider (`FindRider(sys, -1)`) and `paint` is set,
/// switch to the mix preset the paint names (`*paint->preset()`, a virtual
/// call, slot +0x18 of the vtable at `paint + 4`).
pub fn ssx_audio_apply_mix_paint(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        save_regs(c, frame, &[(S0, 0x30), (S2, 0x10), (S1, 0x20)]);
        frame.save_ra(c, 0);
        let (sys, rider, paint) = (reg(c, A0), reg(c, A1), reg(c, A2));
        set_reg(c, S0, rider);
        set_reg(c, S2, sys);
        set_reg(c, S1, paint);
        set_reg(c, A1, u64::MAX);
        callee::FIND_RIDER.call(c, 0x0028_9cc0)?;
        frame.restore(c, S0, 0x30);
        if reg(c, V0) == rider {
            c.cycles += 2;
            if paint == 0 {
                frame.restore(c, S1, 0x20);
                return Ok(finish_apply_mix_paint(c, frame));
            }
            c.cycles += 9;
            let vt = lw_at(c, paint, 4);
            set_reg(c, V1, vt);
            virtual_call(c, paint, vt, 0x18, V0, 0x0028_9ce4)?;
            let preset = lw_at(c, reg(c, V0), 0);
            set_reg(c, A1, preset);
            set_reg(c, A0, sys);
            callee::SET_MIX_PRESET.call(c, 0x0028_9cf0)?;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 1;
        frame.restore(c, S1, 0x20);
        Ok(finish_apply_mix_paint(c, frame))
    })
}

fn finish_apply_mix_paint(c: &mut Cpu, frame: Frame) {
    c.cycles += 4;
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Board loops.

/// The shared shape of `SSXAudioSystem_CalcSurfaceLoopParams`,
/// `..ParamsRemote` and `CalcEdgeLoopParams` (`(sys, soundType, rider,
/// &volume, &pitch t0)`): reset the outputs to volume 0 / pitch 0x1000,
/// look up the sound group of the sound type (`f_002934d0`: 0, 1 or 2, or
/// another value for none) and let that group's function compute them
/// (`group(sys, rider, &volume, &pitch)`). Return addresses are at fixed
/// offsets from `start`.
fn calc_loop_params(c: &mut Cpu, start: u32, groups: [Callee; 3]) {
    const SAVED: [(usize, u32); 4] = [(S2, 0x20), (S0, 0x40), (S1, 0x30), (S3, 0x10)];
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x50);
        set_reg(c, V0, 0x1000);
        save_regs(c, frame, &SAVED);
        frame.save_ra(c, 0);
        let (sys, rider, volume, pitch) = (reg(c, A0), reg(c, A2), reg(c, A3), reg(c, T0));
        set_reg(c, S2, volume);
        set_reg(c, S0, sys);
        set_reg(c, S1, rider);
        set_reg(c, S3, pitch);
        sw_at(c, volume, 0, 0);
        sw_at(c, pitch, 0, 0x1000);
        callee::SURFACE_GROUP.call(c, start + 0x38)?;
        let group = reg(c, V0);
        set_reg(c, V1, group);
        set_reg(c, V0, 1);
        let call_group = |c: &mut Cpu, g: usize, ret: u32| -> Flow {
            set_reg(c, A0, sys);
            set_reg(c, A1, rider);
            set_reg(c, A2, volume);
            set_reg(c, A3, pitch);
            groups[g].call(c, ret)
        };
        if group == 0 {
            c.cycles += 7;
            call_group(c, 0, start + 0x58)?;
        } else {
            c.cycles += 2;
            set_reg(c, V0, 2);
            if group == 1 {
                c.cycles += 7;
                call_group(c, 1, start + 0x7c)?;
            } else {
                c.cycles += 2;
                if group == 2 {
                    c.cycles += 6;
                    call_group(c, 2, start + 0xa0)?;
                }
            }
        }
        c.cycles += 6;
        restore_regs(c, frame, &SAVED);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem_CalcSurfaceLoopParams` 0x293290 ([`calc_loop_params`]
/// with the surface groups).
pub fn ssx_audio_calc_surface_loop_params(c: &mut Cpu) {
    calc_loop_params(c, 0x0029_3290, callee::SURFACE_GROUPS)
}

/// `SSXAudioSystem_CalcSurfaceLoopParamsRemote` 0x293350
/// ([`calc_loop_params`] with the remote-rider surface groups).
pub fn ssx_audio_calc_surface_loop_params_remote(c: &mut Cpu) {
    calc_loop_params(c, 0x0029_3350, callee::SURFACE_REMOTE_GROUPS)
}

/// `SSXAudioSystem_CalcEdgeLoopParams` 0x293410 ([`calc_loop_params`] with
/// the edge groups).
pub fn ssx_audio_calc_edge_loop_params(c: &mut Cpu) {
    calc_loop_params(c, 0x0029_3410, callee::EDGE_GROUPS)
}

/// The tail of the board-loop updates: mute (volume 0 at `sp+0`) while
/// the game is paused (gp flags bit 0) or the replay is paused
/// (`f_00270280(gameModule->replay)`), then store the volume in the
/// instance and send the pitch (`sp+4`) with `BankMonitorInstance_SetPitch`.
/// `flags` is the gp word the caller already loaded into v0; `rets` are the
/// return addresses of `GetGameModule`, `f_00270280` and `SetPitch`, and
/// `extra` the cycles of the store block (11 or 12).
fn board_loop_finish(c: &mut Cpu, frame: Frame, inst: u64, flags: u64, rets: [u32; 3], extra: u64) -> Flow {
    let paused = flags & 1;
    set_reg(c, V0, paused);
    set_reg(c, S0, 0);
    let mut mute = true;
    if paused == 0 {
        c.cycles += 6;
        callee::GET_GAME_MODULE.call(c, rets[0])?;
        let replay = lw_at(c, reg(c, V0), 0x28);
        set_reg(c, A0, replay);
        callee::REPLAY_IS_PAUSED.call(c, rets[1])?;
        mute = reg(c, V0) != 0;
    }
    if mute {
        c.cycles += 1;
        set_reg(c, S0, 1);
    }
    c.cycles += 2;
    if mute {
        Addr(frame.at(c, 0)).sw(c, 0);
    }
    c.cycles += extra;
    let volume = Addr(frame.at(c, 0)).lw(c);
    set_reg(c, V0, volume);
    set_reg(c, A0, inst);
    let pitch = Addr(frame.at(c, 4)).lw(c);
    set_reg(c, A1, pitch);
    sw_at(c, inst, bmi::VOLUME, volume);
    call_ptr(c, table::BMI_SET_PITCH, rets[2])
}

/// `Rider_GetMainState(rider)` returning to `ret`; zeroes the volume at
/// `sp+0` when the rider is in main state 2 (the original's check after the
/// loop parameters). 5 cycles.
fn mute_in_state_2(c: &mut Cpu, frame: Frame, rider: u64, ret: u32) -> Flow {
    c.cycles += 5;
    set_reg(c, A0, rider);
    call_ptr(c, table::RIDER_GET_MAIN_STATE, ret)?;
    set_reg(c, V1, 2);
    if reg(c, V0) == 2 {
        Addr(frame.at(c, 0)).sw(c, 0);
    }
    Ok(())
}

/// `SSXAudioSystem_UpdateBoardBaseLoop(sys, inst)` 0x292cb8: the per-frame
/// callback of a rider's board base loop. Unless the rider is in main
/// state 0 the volume and pitch come from the speed curves
/// (`CalcBoardBaseLoopParams`); otherwise they stay volume 0 / pitch 0x40.
/// Main state 2, an air-time field >= 0 and paused / replay-paused mute
/// it.
pub fn ssx_audio_update_board_base_loop(c: &mut Cpu) {
    const SAVED: [(usize, u32); 3] = [(S0, 0x40), (S1, 0x30), (S2, 0x20)];
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        set_reg(c, V0, 0x40);
        save_regs(c, frame, &SAVED);
        frame.save_ra(c, 0x10);
        let (sys, inst) = (reg(c, A0), reg(c, A1));
        set_reg(c, S1, sys);
        set_reg(c, S2, inst);
        let rider = lw_at(c, inst, bmi::RIDER);
        set_reg(c, S0, rider);
        Addr(frame.at(c, 4)).sw(c, 0x40);
        Addr(frame.at(c, 0)).sw(c, 0);
        set_reg(c, A0, rider);
        call_ptr(c, table::RIDER_GET_MAIN_STATE, 0x0029_2cec)?;
        let state = reg(c, V0);
        set_reg(c, A0, sys);
        if state == 0 {
            c.cycles += 4;
            set_reg(c, A1, rider);
            set_reg(c, A2, sx(frame.at(c, 0)));
            set_reg(c, A3, reg(c, SP) | 4);
            callee::CALC_BOARD_BASE_LOOP_PARAMS.call(c, 0x0029_2d04)?;
        }
        mute_in_state_2(c, frame, rider, 0x0029_2d0c)?;
        // Airborne.
        c.cycles += 6;
        let air = lwc1_at(c, rider, rider::AIR);
        abi::set_fpr(c, 1, air);
        c.f[0] = 0;
        let grounded = {
            let le = FpuF32::ZERO.le(air);
            abi::set_fcc(c, le);
            le
        };
        set_reg(c, V0, 1);
        if !grounded {
            c.cycles += 1;
            set_reg(c, V0, 0);
        }
        c.cycles += 2;
        if reg(c, V0) != 0 {
            Addr(frame.at(c, 0)).sw(c, 0);
        }
        c.cycles += 4;
        let flags = gp_lw(c, -(gp::PAUSE_FLAGS as i32));
        board_loop_finish(c, frame, inst, flags, [0x0029_2d54, 0x0029_2d5c, 0x0029_2d84], 11)?;
        restore_regs(c, frame, &SAVED);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `GetSurfaceSoundType` then the loop parameters
/// (`params(sys, type, rider, &volume, &pitch)`). `cyc` cycles first.
fn loop_params(c: &mut Cpu, frame: Frame, sys: u64, rider: u64, params: Callee, cyc: u64, rets: [u32; 2]) -> Flow {
    c.cycles += cyc;
    set_reg(c, A1, rider);
    callee::GET_SURFACE_SOUND_TYPE.call(c, rets[0])?;
    set_reg(c, A0, sys);
    set_reg(c, A1, reg(c, V0));
    set_reg(c, A2, rider);
    set_reg(c, A3, sx(frame.at(c, 0)));
    set_reg(c, T0, reg(c, SP) | 4);
    params.call(c, rets[1])
}

/// The "special surface" step of the surface / edge loops (main state 1 or
/// 5): a sound type in group 2 is replaced by type 9.
fn special_surface(c: &mut Cpu, sys: u64, rider: u64, field: u32, ret: u32) -> Flow {
    c.cycles += 7;
    let kind = lw_at(c, rider, field);
    set_reg(c, A1, kind);
    set_reg(c, A0, sys);
    set_reg(c, A2, rider);
    callee::SURFACE_GROUP.call(c, ret)?;
    set_reg(c, V1, 2);
    let group2 = reg(c, V0) == 2;
    set_reg(c, V0, 9);
    if group2 {
        c.cycles += 2;
        sw_at(c, rider, field, 9);
    }
    Ok(())
}

/// `SSXAudioSystem_UpdateBoardSurfaceLoop(sys, inst)` 0x292da0: the
/// per-frame callback of a rider's board surface loop. Remembers the
/// rider's main state; in states 1 and 5 maps group-2 surfaces to type 9,
/// otherwise restarts the loop on a sound type change or computes volume
/// and pitch from the surface curves. Main state 2 and paused /
/// replay-paused mute it.
pub fn ssx_audio_update_board_surface_loop(c: &mut Cpu) {
    const SAVED: [(usize, u32); 4] = [(S2, 0x30), (S3, 0x20), (S0, 0x50), (S1, 0x40)];
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        save_regs(c, frame, &SAVED);
        frame.save_ra(c, 0x10);
        let (sys, inst) = (reg(c, A0), reg(c, A1));
        set_reg(c, S2, sys);
        set_reg(c, S3, inst);
        let rider = lw_at(c, inst, bmi::RIDER);
        set_reg(c, S1, rider);
        set_reg(c, A0, rider);
        call_ptr(c, table::RIDER_GET_MAIN_STATE, 0x0029_2dcc)?;
        let state = reg(c, V0);
        set_reg(c, A0, state);
        let seen = lw_at(c, rider, rider::AUDIO_STATE);
        set_reg(c, V0, seen);
        if state != seen {
            sw_at(c, rider, rider::AUDIO_STATE, state);
        }
        c.cycles += 5;
        set_reg(c, V1, 0x40);
        Addr(frame.at(c, 0)).sw(c, 0);
        set_reg(c, V0, 1);
        Addr(frame.at(c, 4)).sw(c, 0x40);
        let special = state == 1 || {
            c.cycles += 3;
            set_reg(c, V0, 5);
            set_reg(c, A0, sys);
            state == 5
        };
        if special {
            special_surface(c, sys, rider, rider::SURFACE_SOUND, 0x0029_2e0c)?;
        } else {
            // The sound type (kept in s0).
            c.cycles += 6;
            set_reg(c, A1, rider);
            callee::GET_SURFACE_SOUND_TYPE.call(c, 0x0029_2e28)?;
            let kind = reg(c, V0);
            set_reg(c, S0, kind);
            let current = lw_at(c, rider, rider::SURFACE_SOUND);
            set_reg(c, V0, current);
            set_reg(c, A0, sys);
            if kind == current {
                loop_params(c, frame, sys, rider, callee::CALC_SURFACE_LOOP_PARAMS, 8, [0x0029_2e64, 0x0029_2e7c])?;
            } else {
                c.cycles += 9;
                sw_at(c, rider, rider::SURFACE_SOUND, kind);
                set_reg(c, A0, inst);
                call_ptr(c, table::BMI_RESET, 0x0029_2e44)?;
                set_reg(c, A0, sys);
                set_reg(c, A2, kind);
                set_reg(c, A1, rider);
                callee::START_BOARD_SURFACE_LOOP.call(c, 0x0029_2e54)?;
            }
        }
        mute_in_state_2(c, frame, rider, 0x0029_2e84)?;
        c.cycles += 4;
        let flags = gp_lw(c, -(gp::PAUSE_FLAGS as i32));
        board_loop_finish(c, frame, inst, flags, [0x0029_2ea8, 0x0029_2eb0, 0x0029_2ed8], 12)?;
        restore_regs(c, frame, &SAVED);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem_UpdateBoardSurfaceLoopRemote(sys, inst)` 0x292ef8: the
/// surface loop of a remote rider: in main states 0 and 4 it restarts the
/// loop on a sound type change and computes volume and pitch from the
/// remote surface curves; paused / replay-paused mute it.
pub fn ssx_audio_update_board_surface_loop_remote(c: &mut Cpu) {
    const SAVED: [(usize, u32); 3] = [(S1, 0x30), (S2, 0x20), (S0, 0x40)];
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        save_regs(c, frame, &SAVED);
        frame.save_ra(c, 0x10);
        let (sys, inst) = (reg(c, A0), reg(c, A1));
        set_reg(c, S1, sys);
        set_reg(c, S2, inst);
        let rider = lw_at(c, inst, bmi::RIDER);
        set_reg(c, S0, rider);
        set_reg(c, A0, rider);
        call_ptr(c, table::RIDER_GET_MAIN_STATE, 0x0029_2f20)?;
        let state = reg(c, V0);
        set_reg(c, V1, state);
        let seen = lw_at(c, rider, rider::AUDIO_STATE);
        set_reg(c, V0, seen);
        if state != seen {
            sw_at(c, rider, rider::AUDIO_STATE, state);
        }
        c.cycles += 4;
        set_reg(c, V0, 0x40);
        Addr(frame.at(c, 0)).sw(c, 0);
        Addr(frame.at(c, 4)).sw(c, 0x40);
        let riding = state == 0 || {
            c.cycles += 3;
            set_reg(c, V0, 4);
            state == 4
        };
        let flags = if riding {
            set_reg(c, A0, sys);
            c.cycles += 7;
            set_reg(c, A1, rider);
            callee::GET_SURFACE_SOUND_TYPE.call(c, 0x0029_2f58)?;
            let kind = reg(c, V0);
            set_reg(c, V1, kind);
            let current = lw_at(c, rider, rider::SURFACE_SOUND);
            set_reg(c, V0, current);
            if kind != current {
                c.cycles += 11;
                sw_at(c, rider, rider::SURFACE_SOUND, kind);
                set_reg(c, A0, inst);
                call_ptr(c, table::BMI_RESET, 0x0029_2f74)?;
                set_reg(c, A0, sys);
                set_reg(c, A1, rider);
                callee::GET_SURFACE_SOUND_TYPE.call(c, 0x0029_2f80)?;
                set_reg(c, A2, reg(c, V0));
                set_reg(c, A0, sys);
                set_reg(c, A1, rider);
                callee::START_BOARD_SURFACE_LOOP.call(c, 0x0029_2f90)?;
            }
            set_reg(c, A0, sys);
            loop_params(
                c,
                frame,
                sys,
                rider,
                callee::CALC_SURFACE_LOOP_PARAMS_REMOTE,
                9,
                [0x0029_2f9c, 0x0029_2fb4],
            )?;
            gp_lw(c, -(gp::PAUSE_FLAGS as i32))
        } else {
            gp_lw(c, -(gp::PAUSE_FLAGS as i32))
        };
        c.cycles += 3;
        board_loop_finish(c, frame, inst, flags, [0x0029_2fcc, 0x0029_2fd4, 0x0029_2ffc], 11)?;
        restore_regs(c, frame, &SAVED);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem_UpdateBoardEdgeLoop(sys, inst)` 0x293018: the
/// per-frame callback of a rider's board edge loop: like the surface loop
/// (states 1 and 5 map group-2 edge types to 9; otherwise restart on a
/// change or compute the edge parameters), without remembering the main
/// state.
pub fn ssx_audio_update_board_edge_loop(c: &mut Cpu) {
    const SAVED: [(usize, u32); 3] = [(S1, 0x30), (S2, 0x20), (S0, 0x40)];
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x50);
        set_reg(c, V0, 0x40);
        save_regs(c, frame, &SAVED);
        frame.save_ra(c, 0x10);
        let (sys, inst) = (reg(c, A0), reg(c, A1));
        set_reg(c, S1, sys);
        set_reg(c, S2, inst);
        let rider = lw_at(c, inst, bmi::RIDER);
        set_reg(c, S0, rider);
        Addr(frame.at(c, 4)).sw(c, 0x40);
        Addr(frame.at(c, 0)).sw(c, 0);
        set_reg(c, A0, rider);
        call_ptr(c, table::RIDER_GET_MAIN_STATE, 0x0029_304c)?;
        let state = reg(c, V0);
        set_reg(c, V1, state);
        set_reg(c, V0, 5);
        let special = state == 1 || {
            c.cycles += 2;
            set_reg(c, A0, sys);
            state == 5
        };
        if special {
            special_surface(c, sys, rider, rider::EDGE_SOUND, 0x0029_3074)?;
        } else {
            let changed = edge_type_changed(c, sys, inst, rider)?;
            if !changed {
                loop_params(c, frame, sys, rider, callee::CALC_EDGE_LOOP_PARAMS, 8, [0x0029_30d8, 0x0029_30f0])?;
            }
        }
        mute_in_state_2(c, frame, rider, 0x0029_30f8)?;
        c.cycles += 4;
        let flags = gp_lw(c, -(gp::PAUSE_FLAGS as i32));
        board_loop_finish(c, frame, inst, flags, [0x0029_311c, 0x0029_3124, 0x0029_314c], 11)?;
        restore_regs(c, frame, &SAVED);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// The sound-type step of the edge loop: when the rider's edge sound type
/// changed, store it, reset the instance and restart the edge loop with it
/// (true); the new type is kept in v1 across the compare.
fn edge_type_changed(c: &mut Cpu, sys: u64, inst: u64, rider: u64) -> Result<bool, Unwound> {
    c.cycles += 6;
    set_reg(c, A1, rider);
    callee::GET_SURFACE_SOUND_TYPE.call(c, 0x0029_3090)?;
    let kind = reg(c, V0);
    set_reg(c, V1, kind);
    let current = lw_at(c, rider, rider::EDGE_SOUND);
    set_reg(c, V0, current);
    set_reg(c, A0, sys);
    if kind == current {
        return Ok(false);
    }
    c.cycles += 12;
    sw_at(c, rider, rider::EDGE_SOUND, kind);
    set_reg(c, A0, inst);
    call_ptr(c, table::BMI_RESET, 0x0029_30ac)?;
    set_reg(c, A0, sys);
    set_reg(c, A1, rider);
    callee::GET_SURFACE_SOUND_TYPE.call(c, 0x0029_30b8)?;
    set_reg(c, A0, sys);
    set_reg(c, A2, reg(c, V0));
    set_reg(c, A1, rider);
    callee::START_BOARD_EDGE_LOOP.call(c, 0x0029_30c8)?;
    Ok(true)
}

// ---------------------------------------------------------------------------
// Boost and speech.

/// `SSXAudioSystem_StopBoostSound(sys, rider)` 0x2992d8: outside replays,
/// for a human local rider, count the boost sound's stop delay down; when
/// it reaches exactly 0 stop the boost handle (`BankMonitor_StopHandle(bm,
/// handle, 1)`, f12 = 0). The delay is never left negative.
pub fn ssx_audio_stop_boost_sound(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        save_regs(c, frame, &[(S0, 0x20), (S1, 0x10)]);
        frame.save_ra(c, 0);
        let (sys, rider) = (reg(c, A0), reg(c, A1));
        set_reg(c, S0, sys);
        set_reg(c, S1, rider);
        call_ptr(c, table::IS_REPLAY_PLAYING, 0x0029_92f4)?;
        if reg(c, V0) == 0 {
            c.cycles += 3;
            let human = lw_at(c, rider, rider::HUMAN);
            set_reg(c, V0, human);
            set_reg(c, A1, 0);
            if human != 0 {
                c.cycles += 2;
                let local = lw_at(c, rider, rider::LOCAL);
                set_reg(c, V0, local);
                set_reg(c, A1, (local != 0) as u64);
            }
            c.cycles += 2;
            if reg(c, A1) != 0 {
                c.cycles += 4;
                let left = addiu(lw_at(c, sys, sys::BOOST_STOP_DELAY), -1);
                set_reg(c, V0, left);
                sw_at(c, sys, sys::BOOST_STOP_DELAY, left);
                if (left as i64) <= 0 {
                    c.cycles += 2;
                    if left == 0 {
                        c.cycles += 8;
                        let root = lw_at(c, sys, sys::ROOT);
                        set_reg(c, V0, root);
                        set_reg(c, A2, 1);
                        let handle = lw_at(c, sys, sys::BOOST_HANDLE);
                        set_reg(c, A1, handle);
                        let snd = lw_at(c, root, 0);
                        c.f[12] = 0;
                        set_reg(c, A0, addiu(snd, sys::SND_BANK_MONITOR as i32));
                        callee::STOP_HANDLE.call(c, 0x0029_934c)?;
                    }
                    sw_at(c, sys, sys::BOOST_STOP_DELAY, 0);
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

/// `SpeechInstance_Update(inst, stream)` 0x2afc88: while the speech
/// instance is active (`f_002b0290`), start its stream once
/// (`f_003d8040`, flag at +0x900) and, once started, notify the speech
/// listener (virtual call, slot +0x20 of the vtable at `listener + 0x1bc`)
/// and clear the flag when the stream reports done (`f_003d7ec8`). An
/// inactive instance sets the flag.
pub fn speech_instance_update(c: &mut Cpu) {
    const STARTED: u32 = 0x900;
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        save_regs(c, frame, &[(S0, 0x20), (S1, 0x10)]);
        frame.save_ra(c, 0);
        let (inst, stream) = (reg(c, A0), reg(c, A1));
        set_reg(c, S0, inst);
        set_reg(c, S1, stream);
        callee::SPEECH_INSTANCE_POLL.call(c, 0x002a_fca4)?;
        let active = reg(c, V0) != 0;
        set_reg(c, V0, 1);
        if !active {
            c.cycles += 1;
            sw_at(c, inst, STARTED, 1);
        } else {
            c.cycles += 3;
            let started = lw_at(c, inst, STARTED);
            set_reg(c, V0, started);
            let listener = gp_lw(c, gp::SPEECH_LISTENER as i32);
            set_reg(c, A1, listener);
            if started == 0 {
                c.cycles += 5;
                set_reg(c, A0, stream);
                call_ptr(c, table::SPEECH_STREAM_START, 0x002a_fcec)?;
                set_reg(c, V1, 1);
                sw_at(c, inst, STARTED, 1);
            } else {
                c.cycles += 9;
                let vt = lw_at(c, listener, 0x1bc);
                set_reg(c, V0, vt);
                virtual_call(c, listener, vt, 0x20, V1, 0x002a_fccc)?;
                set_reg(c, A0, stream);
                call_ptr(c, table::SPEECH_STREAM_DONE, 0x002a_fcd4)?;
                if reg(c, V0) != 0 {
                    sw_at(c, inst, STARTED, 0);
                } else {
                    c.cycles += 2;
                    frame.restore(c, S0, 0x20);
                    return Ok(finish_speech(c, frame));
                }
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x20);
        Ok(finish_speech(c, frame))
    })
}

fn finish_speech(c: &mut Cpu, frame: Frame) {
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// BankMonitorInstance.

/// `BankMonitorInstance_IsDone(inst)` 0x2aba40: has the instance's voice
/// finished? SND handle (state 1): `SND_IsHandleDone != 0`; MIDI player
/// (2): status 2 or an error (< 0); stream buffer (3): request status 3, or
/// an error; any other state counts as done. The driver is queried under
/// `SND_Lock`.
pub fn bank_monitor_instance_is_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        set_reg(c, V0, 2);
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0x10);
        let inst = reg(c, A0);
        set_reg(c, S1, inst);
        frame.save(c, S0, 0x30);
        let state = lw_at(c, inst, bmi::STATE);
        set_reg(c, S0, state);
        if state == 2 {
            // A MIDI player.
            c.cycles += 8;
            call_ptr(c, table::SND_LOCK, 0x002a_babc)?;
            let h = lw_at(c, inst, bmi::HANDLE);
            set_reg(c, A0, h);
            call_ptr(c, table::SNDMIDI_GET_STATUS, 0x002a_bac4)?;
            let status = reg(c, V0);
            set_reg(c, S1, status);
            call_ptr(c, table::SND_UNLOCK, 0x002a_bacc)?;
            set_reg(c, V0, 0);
            if status != state {
                c.cycles += 2;
                if (status as i64) >= 0 {
                    frame.restore(c, S0, 0x30);
                    return Ok(is_done_exit(c, frame, 1));
                }
            }
            c.cycles += 2;
            set_reg(c, V0, 1);
            return Ok(is_done_exit(c, frame, 2));
        }
        c.cycles += 3;
        set_reg(c, V0, 1);
        if (state as i64) < 3 {
            c.cycles += 2;
            frame.restore(c, S0, 0x30);
            if state != 1 {
                c.cycles += 2;
                frame.restore(c, S1, 0x20);
                return Ok(is_done_exit(c, frame, 0));
            }
            // An SND handle (s0 is already the caller's again).
            c.cycles += 8;
            call_ptr(c, table::SND_LOCK, 0x002a_ba9c)?;
            let h = lw_at(c, inst, bmi::HANDLE);
            set_reg(c, A0, h);
            call_ptr(c, table::SND_IS_HANDLE_DONE, 0x002a_baa4)?;
            let done = (reg(c, V0) != 0) as u64;
            set_reg(c, S0, done);
            call_ptr(c, table::SND_UNLOCK, 0x002a_baac)?;
            set_reg(c, V0, done);
            return Ok(is_done_exit(c, frame, 2));
        }
        c.cycles += 3;
        set_reg(c, V0, 1);
        if state != 3 {
            c.cycles += 2;
            frame.restore(c, S0, 0x30);
            return Ok(is_done_exit(c, frame, 1));
        }
        // A stream buffer: its request status lands at sp+0.
        c.cycles += 7;
        call_ptr(c, table::SND_LOCK, 0x002a_baec)?;
        let h = lw_at(c, inst, bmi::HANDLE);
        set_reg(c, A0, h);
        set_reg(c, A1, reg(c, SP));
        call_ptr(c, table::SNDSTRM_GET_REQUEST_STATUS, 0x002a_baf8)?;
        if (reg(c, V0) as i64) < 0 {
            c.cycles += 3;
            call_ptr(c, table::SND_UNLOCK, 0x002a_bb20)?;
            set_reg(c, V0, 1);
        } else {
            c.cycles += 6;
            call_ptr(c, table::SND_UNLOCK, 0x002a_bb08)?;
            let status = Addr(frame.at(c, 0)).lw(c);
            set_reg(c, V0, ((status ^ 3) < 1) as u64);
        }
        Ok(is_done_exit(c, frame, 2))
    })
}

/// The exits of [`bank_monitor_instance_is_done`]: `restores` is how many of
/// `lq s0` (1 cycle) and `lq s1` (1 cycle) are still to come (2, 1 or 0),
/// then 3 cycles for `ra` and the return.
fn is_done_exit(c: &mut Cpu, frame: Frame, restores: u32) {
    if restores >= 2 {
        c.cycles += 1;
        frame.restore(c, S0, 0x30);
    }
    if restores >= 1 {
        c.cycles += 1;
        frame.restore(c, S1, 0x20);
    }
    c.cycles += 3;
    frame.restore_ra(c, 0x10);
    frame.close(c);
}

/// `BankMonitorInstance_SetPan(inst, pan, spread)` 0x2abd90: when the pan
/// moved by more than 0x1ff from the last one sent, remember it and pass
/// pan (and spread) on to whatever plays: the SND handle or MIDI player
/// (under `SND_Lock`), or the stream buffer.
pub fn bank_monitor_instance_set_pan(c: &mut Cpu) {
    const SAVED: [(usize, u32); 3] = [(S0, 0x30), (S1, 0x20), (S2, 0x10)];
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        save_regs(c, frame, &SAVED);
        frame.save_ra(c, 0);
        let (inst, pan, spread) = (reg(c, A0), reg(c, A1), reg(c, A2));
        set_reg(c, S0, inst);
        set_reg(c, S1, pan);
        // |pan - last| <= 0x1ff, as one unsigned compare.
        let last = lw_at(c, inst, bmi::LAST_PAN);
        let moved = addiu(sx((pan as u32).wrapping_sub(last as u32)), 0x1ff);
        let small = (moved < 0x3ff) as u64;
        set_reg(c, V0, small);
        set_reg(c, S2, spread);
        if small != 0 {
            c.cycles += 1;
        } else {
            c.cycles += 4;
            let state = lw_at(c, inst, bmi::STATE);
            set_reg(c, V1, state);
            set_reg(c, V0, 2);
            sw_at(c, inst, bmi::LAST_PAN, pan);
            if state == 2 {
                c.cycles += 9;
                call_ptr(c, table::SND_LOCK, 0x002a_be34)?;
                let h = lw_at(c, inst, bmi::HANDLE);
                set_reg(c, A0, h);
                set_reg(c, A1, pan);
                call_ptr(c, table::SNDMIDI_SET_PAN, 0x002a_be40)?;
                call_ptr(c, table::SND_UNLOCK, 0x002a_be48)?;
            } else {
                c.cycles += 3;
                set_reg(c, V0, 1);
                if (state as i64) < 3 {
                    c.cycles += 2;
                    if state == 1 {
                        c.cycles += 10;
                        call_ptr(c, table::SND_LOCK, 0x002a_be0c)?;
                        let h = lw_at(c, inst, bmi::HANDLE);
                        set_reg(c, A0, h);
                        set_reg(c, A1, pan);
                        set_reg(c, A2, spread);
                        call_ptr(c, table::SND_SET_PAN, 0x002a_be1c)?;
                        call_ptr(c, table::SND_UNLOCK, 0x002a_be24)?;
                    } else {
                        c.cycles += 2;
                    }
                } else {
                    c.cycles += 3;
                    set_reg(c, V0, 3);
                    if state == 3 {
                        let stream = lw_at(c, inst, bmi::STREAM);
                        set_reg(c, A0, stream);
                        c.cycles += 3;
                        set_reg(c, A1, pan);
                        set_reg(c, A2, spread);
                        callee::STREAM_BUFFER_SET_PAN.call(c, 0x002a_be5c)?;
                        c.cycles += 1;
                    } else {
                        c.cycles += 2;
                    }
                }
            }
        }
        c.cycles += 5;
        restore_regs(c, frame, &SAVED);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The volume step of [`bank_monitor_instance_set_volume`]: scale the
/// volume (s0) by the channel gain when the gain is >= 0, clamp it to
/// 0..127 (the -1 for the clamp is left in `minus_one_reg`) and compare it
/// with the last volume sent. `None` when it moved by at most 4 (nothing is
/// sent). `cyc` is the cost of the gain test block (5, or 6 when it loads
/// the gain itself).
fn scaled_volume(c: &mut Cpu, inst: u64, cyc: u64, minus_one_reg: usize) -> Option<u64> {
    c.cycles += cyc;
    let gain = fr(c, 1);
    c.f[0] = 0;
    let scale = FpuF32::ZERO.le(gain);
    abi::set_fcc(c, scale);
    let mut vol = reg(c, S0);
    set_reg(c, V0, slt(vol, 128));
    if scale {
        c.cycles += 6;
        let v = gain * FpuF32::from_i32(vol as i32);
        abi::set_fpr(c, 0, v);
        vol = to_int_via(c, 1, v);
        set_reg(c, S0, vol);
        set_reg(c, V0, slt(vol, 128));
    }
    c.cycles += 7;
    set_reg(c, V1, 0x7f);
    if reg(c, V0) == 0 {
        vol = 0x7f;
    }
    set_reg(c, minus_one_reg, u64::MAX);
    let positive = slt(u64::MAX, vol);
    set_reg(c, V0, positive);
    let last = lw_at(c, inst, bmi::LAST_VOLUME);
    set_reg(c, V1, last);
    if positive == 0 {
        vol = 0;
    }
    set_reg(c, S0, vol);
    if (last as i64) >= 0 {
        c.cycles += 5;
        let moved = addiu(sx((vol as u32).wrapping_sub(last as u32)), 4);
        let small = (moved < 9) as u64;
        set_reg(c, V0, small);
        if small != 0 {
            return None;
        }
    }
    Some(vol)
}

/// `BankMonitorInstance_SetVolume(inst, volume)` 0x2abf60: scale the volume
/// by the instance's channel gain, clamp it to 0..127 and, when it moved
/// by more than 4 from the last one sent, remember it and pass it on to
/// whatever plays: the SND handle or MIDI player (under `SND_Lock`) or the
/// stream buffer. A volume argument of -1 stops the voice instead.
pub fn bank_monitor_instance_set_volume(c: &mut Cpu) {
    const SAVED: [(usize, u32); 4] = [(S0, 0x40), (S1, 0x30), (S3, 0x10), (S2, 0x20)];
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        set_reg(c, V0, 2);
        save_regs(c, frame, &SAVED);
        frame.save_ra(c, 0);
        let (inst, requested) = (reg(c, A0), reg(c, A1));
        set_reg(c, S0, requested);
        set_reg(c, S1, inst);
        let state = lw_at(c, inst, bmi::STATE);
        set_reg(c, V1, state);
        set_reg(c, S3, requested);
        let stop = requested == u64::MAX;
        if state == 2 {
            // A MIDI player.
            let gain = lwc1_at(c, inst, bmi::GAIN);
            abi::set_fpr(c, 1, gain);
            if let Some(vol) = scaled_volume(c, inst, 6, S2) {
                c.cycles += 4;
                sw_at(c, inst, bmi::LAST_VOLUME, vol);
                call_ptr(c, table::SND_LOCK, 0x002a_c0b4)?;
                let h = lw_at(c, inst, bmi::HANDLE);
                set_reg(c, A0, h);
                if stop {
                    c.cycles += 4;
                    call_ptr(c, table::SNDMIDI_STOP, 0x002a_c0c4)?;
                } else {
                    c.cycles += 2;
                    set_reg(c, A1, vol);
                    call_ptr(c, table::SNDMIDI_SET_VOLUME, 0x002a_c0d4)?;
                }
                c.cycles += 4;
                call_ptr(c, table::SND_UNLOCK, 0x002a_c0dc)?;
            }
        } else {
            c.cycles += 3;
            set_reg(c, V0, 1);
            if (state as i64) < 3 {
                c.cycles += 2;
                if state == 1 {
                    // An SND handle.
                    let gain = lwc1_at(c, inst, bmi::GAIN);
                    abi::set_fpr(c, 1, gain);
                    if let Some(vol) = scaled_volume(c, inst, 5, S2) {
                        c.cycles += 4;
                        sw_at(c, inst, bmi::LAST_VOLUME, vol);
                        call_ptr(c, table::SND_LOCK, 0x002a_c024)?;
                        let h = lw_at(c, inst, bmi::HANDLE);
                        set_reg(c, A0, h);
                        c.cycles += 4;
                        if stop {
                            call_ptr(c, table::SND_STOP, 0x002a_c034)?;
                        } else {
                            set_reg(c, A1, vol);
                            call_ptr(c, table::SND_SET_VOLUME, 0x002a_c044)?;
                        }
                        c.cycles += 4;
                        call_ptr(c, table::SND_UNLOCK, 0x002a_c0dc)?;
                    }
                } else {
                    c.cycles += 2;
                }
            } else {
                c.cycles += 3;
                set_reg(c, V0, 3);
                if state == 3 {
                    // A stream buffer.
                    let gain = lwc1_at(c, inst, bmi::GAIN);
                    abi::set_fpr(c, 1, gain);
                    if let Some(vol) = scaled_volume(c, inst, 5, A0) {
                        c.cycles += 2;
                        sw_at(c, inst, bmi::LAST_VOLUME, vol);
                        c.cycles += 4;
                        let stream = lw_at(c, inst, bmi::STREAM);
                        set_reg(c, A0, stream);
                        if stop {
                            callee::STREAM_BUFFER_STOP.call(c, 0x002a_c150)?;
                        } else {
                            set_reg(c, A1, vol);
                            callee::STREAM_BUFFER_SET_VOLUME.call(c, 0x002a_c164)?;
                        }
                    }
                } else {
                    c.cycles += 2;
                }
            }
        }
        c.cycles += 6;
        restore_regs(c, frame, &SAVED);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// FindRider.

/// Addresses of `SSXAudioSystem_FindRider`.
mod find_rider {
    pub const START: u32 = 0x0028_5d98;
    pub const END: u32 = 0x0028_5f48;
    /// Loop heads (resume points after an unwind).
    pub const PLAYER_BODY: u32 = 0x0028_5df8;
    pub const HUMAN_BODY: u32 = 0x0028_5e88;
    pub const FOUND: u32 = 0x0028_5edc;
    pub const ID_BODY: u32 = 0x0028_5ee8;
}

/// Stages of `SSXAudioSystem_FindRider`. Loop state lives in registers:
/// s0 the rider index, s1 the id / best rank / 1, s2 the best index, s3 1,
/// v1 the rider found.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Find {
    /// Replay: the replay's rider.
    Replay,
    /// Pick the search by id (v0 = -1 here).
    ByMode,
    /// Search for the best-ranked human local rider (id -1).
    PlayerBody,
    PlayerCheck,
    /// id -2: the first human local rider.
    HumanBody,
    HumanCheck,
    /// Return v1.
    Found,
    /// Other ids: the rider with that player id.
    IdBody,
    IdCheck,
    /// Not found: rider 0.
    First,
    /// v0 = race manager entry; return its rider.
    RiderAt,
    Epilogue,
}

type FindStep = Result<Resume<Find>, Unwound>;

/// `SSXAudioSystem_FindRider(sys, id)` 0x285d98: the rider the audio
/// follows. In a replay, the replay's rider (or null). Otherwise among the
/// race manager's riders: `id == -1` the human local rider with the lowest
/// rank value (+0xec; best index starts at 0, so rider 0 when none),
/// `id == -2` the first human local rider, else the rider whose player id
/// is `id`; rider 0 when none matches.
///
/// The original is a `match pc` dispatcher whose three loops return on an
/// unwind request, so the loop heads are entry points too.
pub fn ssx_audio_find_rider(c: &mut Cpu) {
    let mut st = match c.pc {
        find_rider::START => match find_entry(c) {
            Ok(Resume::Go(s)) => s,
            _ => return,
        },
        find_rider::PLAYER_BODY => Find::PlayerBody,
        find_rider::HUMAN_BODY => Find::HumanBody,
        find_rider::FOUND => Find::Found,
        find_rider::ID_BODY => Find::IdBody,
        pc => {
            c.jump_out(pc, find_rider::START, find_rider::END);
            return;
        }
    };
    loop {
        match find_stage(c, st) {
            Ok(Resume::Go(s)) => st = s,
            _ => return,
        }
    }
}

/// The prologue and the replay test.
fn find_entry(c: &mut Cpu) -> FindStep {
    c.cycles += 10;
    let frame = Frame::open(c, 0x50);
    save_regs(c, frame, &[(S1, 0x30), (S0, 0x40), (S2, 0x20), (S3, 0x10)]);
    frame.save_ra(c, 0);
    set_reg(c, S1, reg(c, A1));
    callee::IS_REPLAY_PLAYING.call(c, 0x0028_5db8)?;
    if reg(c, V0) == 0 {
        set_reg(c, V0, u64::MAX);
        return Ok(Resume::Go(Find::ByMode));
    }
    Ok(Resume::Go(Find::Replay))
}

/// The race manager (`GetRaceMan`) returning to `ret`.
fn race_man(c: &mut Cpu, ret: u32) -> Result<u64, Unwound> {
    callee::GET_RACE_MAN.call(c, ret)?;
    Ok(reg(c, V0))
}

/// `rider = raceMan->riders[s0]` (left in v1, the entry in v0).
fn rider_at_index(c: &mut Cpu, rm: u64) -> u64 {
    let entry = addu(rm, sx((reg(c, S0) as u32) << 2));
    set_reg(c, V0, entry);
    let rider = lw_at(c, entry, 0x28);
    set_reg(c, V1, rider);
    rider
}

/// The "human local rider" test: a0 = `human && local ? flag : 0`.
fn human_local(c: &mut Cpu, rider: u64, flag: u64) -> bool {
    let human = lw_at(c, rider, rider::HUMAN);
    set_reg(c, V0, human);
    set_reg(c, A0, 0);
    if human != 0 {
        c.cycles += 3;
        let local = lw_at(c, rider, rider::LOCAL);
        set_reg(c, V0, local);
        set_reg(c, A0, if local == 0 { 0 } else { flag });
    }
    reg(c, A0) != 0
}

/// `s0 += 1`.
fn next_index(c: &mut Cpu) {
    set_reg(c, S0, addiu(reg(c, S0), 1));
}

/// `s0 < raceMan->field` (count of riders at +0x7c or +0x78).
fn more_riders(c: &mut Cpu, rm: u64, field: u32) -> bool {
    let n = lw_at(c, rm, field);
    let more = slt(reg(c, S0), n);
    set_reg(c, V1, more);
    more != 0
}

/// A back edge to `head`: event check, then continue there.
fn back_edge(c: &mut Cpu, head: u32, next: Find) -> FindStep {
    if back_edge_unwinds(c, head) {
        return Ok(Resume::Unwind);
    }
    Ok(Resume::Go(next))
}

fn find_stage(c: &mut Cpu, st: Find) -> FindStep {
    use Find::*;
    let go = |s| Ok(Resume::Go(s));
    match st {
        Replay => {
            c.cycles += 5;
            let rm = race_man(c, 0x0028_5dc8)?;
            let replay = lw_at(c, rm, 0x40);
            set_reg(c, V1, replay);
            set_reg(c, V0, 0);
            if replay != 0 {
                c.cycles += 2;
                let rider = lw_at(c, replay, 0x18);
                set_reg(c, V0, rider);
            }
            go(Epilogue)
        }
        ByMode => {
            c.cycles += 2;
            let id = reg(c, S1);
            let v0 = reg(c, V0);
            set_reg(c, V0, (-2i64) as u64);
            if id != v0 {
                c.cycles += 2;
                set_reg(c, S0, 0);
                if id != (-2i64) as u64 {
                    return go(IdCheck);
                }
                c.cycles += 2;
                set_reg(c, S1, 1);
                return go(HumanCheck);
            }
            c.cycles += 5;
            set_reg(c, S2, 0);
            set_reg(c, S1, 30000);
            set_reg(c, S0, 0);
            set_reg(c, S3, 1);
            go(PlayerCheck)
        }
        PlayerBody => {
            c.cycles += 8;
            let rm = race_man(c, 0x0028_5e00)?;
            let rider = rider_at_index(c, rm);
            let candidate = human_local(c, rider, reg(c, S3));
            c.cycles += 2;
            if candidate {
                c.cycles += 4;
                let rank = lw_at(c, rider, rider::RANK);
                set_reg(c, V1, rank);
                let better = slt(rank, reg(c, S1));
                set_reg(c, V0, better);
                if better != 0 {
                    c.cycles += 3;
                    set_reg(c, S1, rank);
                    set_reg(c, S2, reg(c, S0));
                }
            }
            next_index(c);
            go(PlayerCheck)
        }
        PlayerCheck => {
            c.cycles += 6;
            let rm = race_man(c, 0x0028_5e50)?;
            if more_riders(c, rm, 0x7c) {
                return back_edge(c, find_rider::PLAYER_BODY, PlayerBody);
            }
            c.cycles += 5;
            let rm = race_man(c, 0x0028_5e68)?;
            let off = sx((reg(c, S2) as u32) << 2);
            set_reg(c, V1, off);
            set_reg(c, V0, addu(rm, off));
            go(RiderAt)
        }
        HumanBody => {
            c.cycles += 8;
            let rm = race_man(c, 0x0028_5e90)?;
            let rider = rider_at_index(c, rm);
            let found = human_local(c, rider, reg(c, S1));
            c.cycles += 2;
            next_index(c);
            if found {
                return go(Found);
            }
            go(HumanCheck)
        }
        HumanCheck => {
            c.cycles += 6;
            let rm = race_man(c, 0x0028_5ec4)?;
            if more_riders(c, rm, 0x78) {
                return back_edge(c, find_rider::HUMAN_BODY, HumanBody);
            }
            c.cycles += 2;
            go(First)
        }
        Found => {
            c.cycles += 2;
            set_reg(c, V0, reg(c, V1));
            go(Epilogue)
        }
        IdBody => {
            c.cycles += 8;
            let rm = race_man(c, 0x0028_5ef0)?;
            let rider = rider_at_index(c, rm);
            let id = lw_at(c, rider, rider::ID);
            set_reg(c, V0, id);
            next_index(c);
            if id == reg(c, S1) {
                return back_edge(c, find_rider::FOUND, Found);
            }
            go(IdCheck)
        }
        IdCheck => {
            c.cycles += 6;
            let rm = race_man(c, 0x0028_5f10)?;
            if more_riders(c, rm, 0x78) {
                return back_edge(c, find_rider::ID_BODY, IdBody);
            }
            go(First)
        }
        First => {
            c.cycles += 2;
            race_man(c, 0x0028_5f28)?;
            go(RiderAt)
        }
        RiderAt => {
            c.cycles += 1;
            let rider = lw_at(c, reg(c, V0), 0x28);
            set_reg(c, V0, rider);
            go(Epilogue)
        }
        Epilogue => {
            c.cycles += 7;
            let frame = Frame { size: 0x50 };
            restore_regs(c, frame, &[(S0, 0x40), (S1, 0x30), (S2, 0x20), (S3, 0x10)]);
            frame.restore_ra(c, 0);
            frame.close(c);
            Ok(Resume::Done)
        }
    }
}

// ---------------------------------------------------------------------------
// Loop parameters from curves.

/// `.rodata` addresses of the 5-point curves (x, y pairs).
mod curves {
    pub const BASE_VOLUME: u32 = 0x0048_2af8;
    pub const BASE_PITCH: u32 = 0x0048_2b20;
    pub const SURFACE0_VOLUME: u32 = 0x0048_2b78;
    pub const SURFACE0_PITCH: u32 = 0x0048_2ba0;
    pub const EDGE0_VOLUME: u32 = 0x0048_2c68;
    pub const EDGE0_PITCH: u32 = 0x0048_2c90;
}

/// The two curve evaluations the loop-parameter functions end with: the
/// volume curve (copied to `sp+0`) at `x_volume` stored as an int at
/// `volume`, then the pitch curve (at `sp+0x30`, pointer kept in s0) at
/// `x_pitch` times 4096 as an int at `pitch`. The curve copies happen
/// before (`copy_curve`); `rets` are the two return addresses.
fn eval_curves(c: &mut Cpu, x_volume: usize, x_pitch: usize, volume: u64, rets: [u32; 2]) -> Result<FpuF32, Unwound> {
    c.f[12] = c.f[x_volume];
    callee::EVAL_CURVE5.call(c, rets[0])?;
    let v = to_int_via(c, 1, fr(c, 0));
    Addr(volume as u32).sw(c, v);
    set_reg(c, A0, reg(c, S0));
    c.f[12] = c.f[x_pitch];
    callee::EVAL_CURVE5.call(c, rets[1])?;
    let unity = FpuF32(0x4580_0000);
    set_reg(c, AT, 0x4580_0000); // leftover
    Ok(unity)
}

/// `SSXAudioSystem_CalcBoardBaseLoopParams(sys, rider, &volume, &pitch)`
/// 0x293168: the board base loop's volume and pitch from two curves over
/// the rider's speed (+0x75c).
pub fn ssx_audio_calc_board_base_loop_params(c: &mut Cpu) {
    const SAVED: [(usize, u32); 3] = [(S0, 0x90), (S1, 0x80), (S2, 0x70)];
    guest(c, |c| {
        c.cycles += 74;
        let frame = Frame::open(c, 0xb0);
        save_regs(c, frame, &SAVED);
        frame.save_ra(c, 0x60);
        frame.save_fpr(c, 20, 0xa0);
        let (rider, volume, pitch) = (reg(c, A1), reg(c, A2), reg(c, A3));
        set_reg(c, S0, sx(frame.at(c, 0x30)));
        set_reg(c, S1, volume);
        set_reg(c, S2, pitch);
        let speed = lwc1_at(c, rider, rider::SPEED);
        abi::set_fpr(c, 20, speed);
        set_reg(c, T1, curves::BASE_VOLUME as u64);
        copy_curve(c, curves::BASE_VOLUME, frame.at(c, 0), [A0, A1, T0]);
        set_reg(c, A0, reg(c, SP));
        set_reg(c, V1, 0x0048_0000);
        set_reg(c, A3, curves::BASE_PITCH as u64);
        copy_curve(c, curves::BASE_PITCH, frame.at(c, 0x30), [V0, A1, A2]);
        let unity = eval_curves(c, 20, 20, volume, [0x0029_324c, 0x0029_3260])?;
        abi::set_fpr(c, 1, unity);
        let p = fr(c, 0) * unity;
        abi::set_fpr(c, 0, p);
        let w = to_int_via(c, 1, p);
        Addr(pitch as u32).sw(c, w);
        restore_regs(c, frame, &SAVED);
        frame.restore_ra(c, 0x60);
        frame.restore_fpr(c, 20, 0xa0);
        frame.close(c);
        Ok(())
    })
}

/// The player index of a rider: virtual call, slot +0x38 of its controller
/// (`rider + 0x6c0`), returning to `ret`.
fn player_index(c: &mut Cpu, rider: u64, ret: u32) -> Result<u64, Unwound> {
    let ctl = addiu(rider, rider::CONTROLLER as i32);
    let vt = lw_at(c, rider, rider::CONTROLLER);
    set_reg(c, V1, vt);
    set_reg(c, A1, ctl);
    virtual_call(c, ctl, vt, 0x38, V0, ret)?;
    Ok(reg(c, V0))
}

/// Ease the per-player smoothed value `prev` toward `target` when they are
/// further apart than `limit`: up by `up * (prev - target)` / down by
/// `down * (target - prev)` from `prev`... exactly as the originals do it
/// (both blocks are shared by the surface and edge groups). `diff` is the
/// FPU register holding `prev - target`; `v_leftover` the register and
/// value the original rewrites in the delay slots.
fn ease(c: &mut Cpu, target: FpuF32, prev: FpuF32, up: u32, down: u32, diff: usize, leftover: (usize, u64)) -> FpuF32 {
    let d = fr(c, diff);
    c.cycles += 3;
    if abi::c_lt_s(c, target, prev) {
        c.cycles += 5;
        let k = FpuF32(up);
        set_reg(c, AT, up as u64);
        abi::set_fpr(c, 0, k);
        let step = d * k;
        abi::set_fpr(c, 0, step);
        return prev - step;
    }
    c.cycles += 4;
    let below = abi::c_lt_s(c, prev, target);
    set_reg(c, leftover.0, leftover.1);
    if below {
        c.cycles += 5;
        let gap = target - prev;
        let k = FpuF32(down);
        set_reg(c, AT, down as u64);
        // The scale goes to f1 here (f0 holds the gap).
        abi::set_fpr(c, 1, k);
        let step = gap * k;
        abi::set_fpr(c, 0, step);
        return prev + step;
    }
    target
}

/// `SSXAudioSystem_CalcSurfaceLoopGroup0(sys, rider, &volume, &pitch)`
/// 0x293538: surface group 0. The target value is `speedC * (1 - (speedB
/// + speedA))`, times `speed * 10` at low speed (`speed < limit`, gp
/// constant); the per-player smoothed value (`sys + 0x59b8`) eases toward it
/// by 1/4 of the gap once they are more than `limit` apart; volume and
/// pitch come from the group's two curves over it.
pub fn ssx_audio_calc_surface_loop_group0(c: &mut Cpu) {
    const SAVED: [(usize, u32); 3] = [(S0, 0x90), (S1, 0x80), (S2, 0x70)];
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0xb0);
        set_reg(c, AT, 0x3f80_0000);
        save_regs(c, frame, &SAVED);
        frame.save_fpr(c, 21, 0xa8);
        frame.save_ra(c, 0x60);
        frame.save_fpr(c, 20, 0xa0);
        let (sys, rider, volume, pitch) = (reg(c, A0), reg(c, A1), reg(c, A2), reg(c, A3));
        set_reg(c, S0, sys);
        set_reg(c, S1, volume);
        set_reg(c, S2, pitch);
        let limit = gp_float(c, -0x45c4);
        abi::set_fpr(c, 21, limit);
        let b = lwc1_at(c, rider, rider::SPEED_B);
        let a = lwc1_at(c, rider, rider::SPEED_A);
        abi::set_fpr(c, 2, a);
        let speed = lwc1_at(c, rider, rider::SPEED);
        abi::set_fpr(c, 12, speed);
        let ab = b + a;
        abi::set_fpr(c, 1, ab);
        let cc = lwc1_at(c, rider, rider::SPEED_C);
        let slow = abi::c_lt_s(c, speed, limit);
        let k = FpuF32::ONE - ab;
        abi::set_fpr(c, 0, k);
        let mut target = cc * k;
        if slow {
            c.cycles += 4;
            let t = target * speed;
            abi::set_fpr(c, 1, t);
            set_reg(c, AT, 0x4120_0000);
            abi::set_fpr(c, 0, FpuF32(0x4120_0000));
            target = t * FpuF32(0x4120_0000);
        }
        abi::set_fpr(c, 20, target);
        c.cycles += 18;
        let index = player_index(c, rider, 0x0029_35b8)?;
        set_reg(c, A0, index);
        let table = addiu(sys, sys::SURFACE_SPEED as i32);
        let off = sx((index as u32) << 2);
        set_reg(c, V1, off);
        set_reg(c, A1, table);
        let slot = addu(table, off);
        set_reg(c, V0, slot);
        let prev = lwc1_at(c, slot, 0);
        abi::set_fpr(c, 2, prev);
        let d = prev - target;
        abi::set_fpr(c, 1, d);
        abi::set_fpr(c, 0, d.abs());
        let far = abi::c_lt_s(c, limit, d.abs());
        set_reg(c, V1, 0x0048_0000);
        if far {
            target = ease(c, target, prev, 0x3e80_0000, 0x3e80_0000, 1, (V1, 0x0048_0000));
            abi::set_fpr(c, 20, target);
        }
        c.cycles += 68;
        let slot = addu(table, sx((index as u32) << 2));
        set_reg(c, V0, slot);
        set_reg(c, T0, curves::SURFACE0_VOLUME as u64);
        copy_curve(c, curves::SURFACE0_VOLUME, frame.at(c, 0), [A0, A2, A3]);
        Addr(slot as u32).write_fpu(c, target);
        set_reg(c, A1, 0x0048_0000);
        set_reg(c, S0, sx(frame.at(c, 0x30)));
        set_reg(c, A0, reg(c, SP));
        set_reg(c, A3, curves::SURFACE0_PITCH as u64);
        copy_curve(c, curves::SURFACE0_PITCH, frame.at(c, 0x30), [V0, V1, A2]);
        let unity = eval_curves(c, 20, 20, volume, [0x0029_36f4, 0x0029_3708])?;
        abi::set_fpr(c, 1, unity);
        let p = fr(c, 0) * unity;
        abi::set_fpr(c, 0, p);
        let w = to_int_via(c, 1, p);
        Addr(pitch as u32).sw(c, w);
        restore_regs(c, frame, &SAVED);
        frame.restore_ra(c, 0x60);
        frame.restore_fpr(c, 21, 0xa8);
        frame.restore_fpr(c, 20, 0xa0);
        frame.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem_CalcEdgeLoopGroup0(sys, rider, &volume, &pitch)`
/// 0x293bb0: edge group 0. The per-player smoothed edge value
/// (`sys + 0x5990`) eases toward `speedB + speedA` (up by half, down by an
/// eighth of the gap once they are more than a gp limit apart); the volume
/// curve runs over it, the pitch curve over `speedC`. Below a gp speed
/// limit the volume is scaled by `speed * 10`.
pub fn ssx_audio_calc_edge_loop_group0(c: &mut Cpu) {
    const SAVED: [(usize, u32); 4] = [(S0, 0xa0), (S1, 0x90), (S2, 0x80), (S3, 0x70)];
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0xc0);
        save_regs(c, frame, &SAVED);
        frame.save_fpr(c, 20, 0xb0);
        frame.save_ra(c, 0x60);
        frame.save_fpr(c, 21, 0xb8);
        let (sys, rider, volume, pitch) = (reg(c, A0), reg(c, A1), reg(c, A2), reg(c, A3));
        set_reg(c, S0, sys);
        set_reg(c, S1, rider);
        set_reg(c, S2, volume);
        set_reg(c, S3, pitch);
        let speed_c = lwc1_at(c, rider, rider::SPEED_C);
        abi::set_fpr(c, 21, speed_c);
        let b = lwc1_at(c, rider, rider::SPEED_B);
        abi::set_fpr(c, 1, b);
        let a = lwc1_at(c, rider, rider::SPEED_A);
        abi::set_fpr(c, 0, a);
        let mut target = b + a;
        abi::set_fpr(c, 20, target);
        let index = player_index(c, rider, 0x0029_3c08)?;
        set_reg(c, V1, index);
        let limit = gp_float(c, -0x45b8);
        abi::set_fpr(c, 1, limit);
        let off = sx((index as u32) << 2);
        let slot = addu(sys, off);
        set_reg(c, V0, slot);
        let prev = lwc1_at(c, slot, sys::EDGE_SPEED);
        abi::set_fpr(c, 2, prev);
        let d = prev - target;
        abi::set_fpr(c, 3, d);
        abi::set_fpr(c, 0, d.abs());
        let far = abi::c_lt_s(c, limit, d.abs());
        set_reg(c, V0, off);
        if far {
            target = ease(c, target, prev, 0x3f00_0000, 0x3e00_0000, 3, (V0, off));
            abi::set_fpr(c, 20, target);
        }
        c.cycles += 66;
        set_reg(c, A1, 0x0048_0000);
        set_reg(c, V1, 0x0048_0000);
        let slot = addu(sys, off);
        set_reg(c, V0, slot);
        set_reg(c, T0, curves::EDGE0_VOLUME as u64);
        copy_curve(c, curves::EDGE0_VOLUME, frame.at(c, 0), [A0, A2, A3]);
        set_reg(c, S0, sx(frame.at(c, 0x30)));
        Addr((slot as u32).wrapping_add(sys::EDGE_SPEED)).write_fpu(c, target);
        set_reg(c, A0, reg(c, SP));
        set_reg(c, A3, curves::EDGE0_PITCH as u64);
        copy_curve(c, curves::EDGE0_PITCH, frame.at(c, 0x30), [V0, V1, A2]);
        let unity = eval_curves(c, 20, 21, volume, [0x0029_3d40, 0x0029_3d54])?;
        abi::set_fpr(c, 1, unity);
        let slow_limit = gp_float(c, -0x45b4);
        abi::set_fpr(c, 2, slow_limit);
        let p = fr(c, 0) * unity;
        abi::set_fpr(c, 0, p);
        let w = to_int_via(c, 1, p);
        Addr(pitch as u32).sw(c, w);
        let speed = lwc1_at(c, rider, rider::SPEED);
        abi::set_fpr(c, 3, speed);
        if abi::c_lt_s(c, speed, slow_limit) {
            c.cycles += 8;
            let v = FpuF32::from_i32(Addr(volume as u32).read_u32(c) as i32);
            set_reg(c, AT, 0x4120_0000);
            abi::set_fpr(c, 1, FpuF32(0x4120_0000));
            let v = v * speed * FpuF32(0x4120_0000);
            abi::set_fpr(c, 0, v);
            let w = to_int_via(c, 1, v);
            Addr(volume as u32).sw(c, w);
        }
        c.cycles += 8;
        restore_regs(c, frame, &SAVED);
        frame.restore_ra(c, 0x60);
        frame.restore_fpr(c, 21, 0xb8);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BankMonitorInstance_Update3D.

/// `BankMonitorInstance_Update3D(dt f12, inst, bm)` 0x2ac868: the
/// per-frame update of a playing instance.
///
/// A stream whose request is not running (status <= 0, request status not
/// 2) is left alone. With 3D enabled and a positional instance: measure it
/// against the listener (virtual call, slot +0x30: squared distance in f0,
/// pan at `sp+0x18`, spread at `sp+0x1a`); unless the volume is fixed,
/// update the range (`UpdateRange`; when that says "out of range" only the
/// distance is stored); doppler pitch from the distance change and `dt`
/// (when enabled and the last distance is known, else 1); send pan and
/// pitch; volume from the distance falloff (radius 30 when 0), scaled by
/// `volume scale / 127`; store the distance. Otherwise the base volume.
/// Then, unless releasing, the volume (times `*gainPtr` when set) is sent;
/// a releasing instance is cleared once its voice is done.
pub fn bank_monitor_instance_update_3d(c: &mut Cpu) {
    const SAVED: [(usize, u32); 4] = [(S1, 0x60), (S3, 0x40), (S0, 0x70), (S2, 0x50)];
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x90);
        save_regs(c, frame, &SAVED);
        frame.save_fpr(c, 20, 0x80);
        frame.save_ra(c, 0x30);
        frame.save_fpr(c, 21, 0x88);
        let (inst, bm) = (reg(c, A0), reg(c, A1));
        set_reg(c, S1, inst);
        set_reg(c, S3, bm);
        let state = lw_at(c, inst, bmi::STATE);
        set_reg(c, V1, state);
        c.f[20] = c.f[12];
        if state == 0 {
            return Ok(update_3d_exit(c, frame, true));
        }
        c.cycles += 3;
        set_reg(c, V0, 3);
        set_reg(c, S2, 0);
        if state == 3 {
            c.cycles += 9;
            call_ptr(c, table::SND_LOCK, 0x002a_c8b0)?;
            let h = lw_at(c, inst, bmi::HANDLE);
            set_reg(c, A0, h);
            set_reg(c, A1, reg(c, SP));
            call_ptr(c, table::SNDSTRM_GET_REQUEST_STATUS, 0x002a_c8bc)?;
            let status = reg(c, V0);
            set_reg(c, S0, status);
            call_ptr(c, table::SND_UNLOCK, 0x002a_c8c4)?;
            if (status as i64) <= 0 {
                c.cycles += 4;
                let request = Addr(frame.at(c, 0)).lw(c);
                set_reg(c, V1, request);
                set_reg(c, V0, 2);
                frame.restore(c, S0, 0x70);
                if request != 2 {
                    return Ok(update_3d_exit(c, frame, false));
                }
                c.cycles += 1;
            }
        } else {
            c.cycles += 1;
        }
        let enabled = lw_at(c, bm, bank_monitor::ENABLE_3D);
        set_reg(c, V0, enabled);
        c.cycles += 2;
        let mut volume;
        if enabled == 0 {
            volume = lw_at(c, inst, bmi::VOLUME);
        } else {
            c.cycles += 3;
            let positional = lw_at(c, inst, bmi::POSITIONAL);
            set_reg(c, V0, positional);
            set_reg(c, A1, inst);
            if positional == 0 {
                c.cycles += 1;
                volume = lw_at(c, inst, bmi::VOLUME);
            } else {
                volume = positional_volume(c, frame, inst, bm)?;
            }
        }
        set_reg(c, S2, volume);
        c.cycles += 3;
        let releasing = lw_at(c, inst, bmi::RELEASING);
        set_reg(c, V0, releasing);
        if releasing == 0 {
            c.cycles += 3;
            let gain = lw_at(c, inst, bmi::GAIN_PTR);
            set_reg(c, V0, gain);
            set_reg(c, A0, inst);
            if gain != 0 {
                c.cycles += 6;
                let g = lwc1_at(c, gain, 0);
                let v = FpuF32::from_i32(volume as i32) * g;
                abi::set_fpr(c, 0, v);
                volume = to_int_via(c, 1, v);
                set_reg(c, S2, volume);
            }
            c.cycles += 4;
            set_reg(c, A1, volume);
            callee::BMI_SET_VOLUME.call(c, 0x002a_ca80)?;
            frame.restore(c, S0, 0x70);
            return Ok(update_3d_exit(c, frame, false));
        }
        c.cycles += 4;
        set_reg(c, A0, inst);
        callee::BMI_IS_DONE.call(c, 0x002a_ca90)?;
        let done = reg(c, V0) != 0;
        set_reg(c, V0, u64::MAX);
        if done {
            c.cycles += 3;
            sw_at(c, inst, bmi::FLAGS, 0);
            sw_at(c, inst, bmi::HANDLE, u64::MAX);
            sw_at(c, inst, bmi::RELEASING, 0);
        }
        Ok(update_3d_exit(c, frame, true))
    })
}

/// The exits of [`bank_monitor_instance_update_3d`] (`restore_s0`: the 1
/// cycle `lq s0` exit; otherwise s0 is already restored).
fn update_3d_exit(c: &mut Cpu, frame: Frame, restore_s0: bool) {
    if restore_s0 {
        c.cycles += 1;
        frame.restore(c, S0, 0x70);
    }
    c.cycles += 8;
    restore_regs(c, frame, &[(S1, 0x60), (S2, 0x50), (S3, 0x40)]);
    frame.restore_ra(c, 0x30);
    frame.restore_fpr(c, 21, 0x88);
    frame.restore_fpr(c, 20, 0x80);
    frame.close(c);
}

/// The positional part of [`bank_monitor_instance_update_3d`]: returns the
/// volume (0..127) and stores the distance.
fn positional_volume(c: &mut Cpu, frame: Frame, inst: u64, bm: u64) -> Result<u64, Unwound> {
    c.cycles += 10;
    let vt = lw_at(c, bm, bank_monitor::LISTENER);
    set_reg(c, V1, vt);
    set_reg(c, A2, sx(frame.at(c, 0x10)));
    virtual_call(c, bm, vt, 0x30, V0, 0x002a_c90c)?;
    let fixed = lw_at(c, inst, bmi::FIXED);
    set_reg(c, V0, fixed);
    let dist = fr(c, 0).sqrt();
    abi::set_fpr(c, 21, dist);
    let mut volume = reg(c, S2);
    let in_range = fixed != 0 || {
        c.cycles += 6;
        set_reg(c, A0, inst);
        set_reg(c, A1, bm);
        c.f[12] = dist.0;
        callee::BMI_UPDATE_RANGE.call(c, 0x002a_c92c)?;
        let ok = reg(c, V0) != 0;
        set_reg(c, A0, slt(volume, 128));
        if ok {
            c.cycles += 1;
        }
        ok
    };
    if in_range {
        let last = lwc1_at(c, inst, bmi::DISTANCE);
        abi::set_fpr(c, 12, last);
        // Doppler.
        c.cycles += 5;
        set_reg(c, AT, 0xffff_ffff_bf80_0000);
        abi::set_fpr(c, 0, FpuF32(0xbf80_0000));
        let unknown = { let e = last.eq_s(FpuF32(0xbf80_0000)); abi::set_fcc(c, e); e };
        let mut doppler = false;
        if !unknown {
            c.cycles += 3;
            let on = lw_at(c, bm, bank_monitor::DOPPLER);
            set_reg(c, V0, on);
            c.f[13] = c.f[20];
            if on != 0 {
                c.cycles += 5;
                abi::set_fpr(c, 12, last - dist);
                set_reg(c, A0, inst);
                callee::CALC_DOPPLER_PITCH.call(c, 0x002a_c964)?;
                c.f[20] = c.f[0];
                doppler = true;
            }
        }
        if !doppler {
            c.cycles += 2;
            set_reg(c, AT, 0x3f80_0000);
            c.f[20] = 0x3f80_0000;
        }
        // Pan and pitch.
        c.cycles += 14;
        let pan = Addr(frame.at(c, 0x18)).lhu(c);
        set_reg(c, A1, pan);
        set_reg(c, A0, inst);
        let spread = Addr(frame.at(c, 0x1a)).lh(c);
        set_reg(c, A2, spread);
        callee::BMI_SET_PAN.call(c, 0x002a_c984)?;
        set_reg(c, AT, 0x4580_0000);
        abi::set_fpr(c, 0, FpuF32(0x4580_0000));
        let p = fr(c, 20) * FpuF32(0x4580_0000);
        abi::set_fpr(c, 0, p);
        let p = to_int_via(c, 1, p);
        set_reg(c, A1, p);
        set_reg(c, A0, inst);
        callee::BMI_SET_PITCH.call(c, 0x002a_c9a0)?;
        let fixed = lw_at(c, inst, bmi::FIXED);
        set_reg(c, V0, fixed);
        if fixed != 0 {
            c.cycles += 2;
            volume = lw_at(c, inst, bmi::VOLUME);
        } else {
            // Falloff.
            let radius = lwc1_at(c, inst, bmi::RADIUS);
            abi::set_fpr(c, 13, radius);
            c.cycles += 5;
            c.f[0] = 0;
            let zero = { let e = radius.eq_s(FpuF32::ZERO); abi::set_fcc(c, e); e };
            set_reg(c, A0, inst);
            if zero {
                c.cycles += 4;
                set_reg(c, AT, 0x41f0_0000);
                c.f[13] = 0x41f0_0000;
            } else {
                c.cycles += 1;
            }
            let base = lw_at(c, inst, bmi::VOLUME);
            set_reg(c, A1, base);
            c.cycles += 6;
            c.f[12] = dist.0;
            callee::BMI_GET_FALLOFF_VOLUME.call(c, 0x002a_c9e4)?;
            let scale = lw_at(c, inst, bmi::VOLUME_SCALE);
            set_reg(c, V1, scale);
            set_reg(c, A1, 0x7f);
            volume = reg(c, V0);
            if scale != 0x7f {
                c.cycles += 12;
                let n = sx(((volume as u32) << 7).wrapping_sub(volume as u32));
                set_reg(c, A0, u64::MAX);
                let (q, _) = abi::div(c, n as u32, scale as u32);
                volume = sx(q);
                set_reg(c, V0, volume);
                let below = slt(volume, 128);
                set_reg(c, V1, below);
                if below == 0 {
                    volume = 0x7f;
                }
                let positive = slt(u64::MAX, volume);
                set_reg(c, A0, positive);
                if positive == 0 {
                    volume = 0;
                }
            }
        }
        c.cycles += 1;
        set_reg(c, A0, slt(volume, 128));
    }
    // Clamp and store the distance.
    c.cycles += 7;
    set_reg(c, V1, 0x7f);
    if reg(c, A0) == 0 {
        volume = 0x7f;
    }
    let positive = slt(u64::MAX, volume);
    set_reg(c, V0, positive);
    Addr((inst as u32).wrapping_add(bmi::DISTANCE)).write_fpu(c, dist);
    if positive == 0 {
        volume = 0;
    }
    Ok(volume)
}

// ---------------------------------------------------------------------------
// Music, speech, countdown.

/// `SongInstance_Update(song)` 0x2b21e0: once the music player's song data
/// is loaded, register the song's stream map and tracks (`PATH_AddMap`,
/// `PATH_AddTrack` for the main track and, when the song has one, the
/// second track; names built with `sprintf` in the frame) and post a
/// pending event. Afterwards: the song's volume, and when playing and
/// enabled, the stream volume `level * gain (* fade) * volume` clamped to
/// 0..127 (`SNDSTRM_SetVolume`, under `SND_Lock`).
pub fn song_instance_update(c: &mut Cpu) {
    const SAVED: [(usize, u32); 4] = [(S0, 0x140), (S1, 0x130), (S2, 0x120), (S3, 0x110)];
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x160);
        save_regs(c, frame, &SAVED);
        frame.save_ra(c, 0x100);
        frame.save_fpr(c, 20, 0x150);
        let song = reg(c, A0);
        set_reg(c, S0, song);
        let registered = lw_at(c, song, 0x2c);
        set_reg(c, V0, registered);
        if registered == 0 {
            c.cycles += 4;
            let mp = gp_lw(c, gp::MUSIC_PLAYER as i32);
            set_reg(c, A0, mp);
            callee::MUSIC_IS_SONG_DATA_LOADED.call(c, 0x002b_2214)?;
            let loaded = reg(c, V0) != 0;
            let mp = gp_lw(c, gp::MUSIC_PLAYER as i32);
            set_reg(c, A0, mp);
            if loaded {
                register_song(c, frame, song)?;
            }
            return Ok(song_exit(c, frame, true));
        }
        let data = lw_at(c, song, 0);
        c.cycles += 6;
        set_reg(c, A0, song);
        let vol = lw_at(c, data, 0x9c);
        set_reg(c, A1, vol);
        callee::SONG_SET_VOLUME.call(c, 0x002b_2304)?;
        let paused = lw_at(c, song, 0x48);
        set_reg(c, V0, paused);
        if paused != 0 {
            frame.restore(c, S0, 0x140);
            return Ok(song_exit(c, frame, false));
        }
        c.cycles += 3;
        let enabled = lw_at(c, song, 0x54);
        set_reg(c, V0, enabled);
        if enabled == 0 {
            frame.restore(c, S0, 0x140);
            return Ok(song_exit(c, frame, false));
        }
        c.cycles += 3;
        let fade = lw_at(c, song, 0x3c);
        set_reg(c, A0, fade);
        let level = if fade == 0 {
            let level = Addr((song as u32).wrapping_add(0x40)).lb(c);
            set_reg(c, V1, level);
            c.cycles += 8;
            let k = gp_float(c, -0x454c);
            abi::set_fpr(c, 2, k);
            let l = FpuF32::from_i32(level as i32);
            let data = lw_at(c, song, 0);
            set_reg(c, V0, data);
            let v = FpuF32::from_i32(lwc1_at(c, data, 0x88).0 as i32);
            abi::set_fpr(c, 1, v);
            let x = l * k;
            abi::set_fpr(c, 0, x);
            x * v
        } else {
            c.cycles += 12;
            let level = Addr((song as u32).wrapping_add(0x40)).lb(c);
            set_reg(c, V0, level);
            let k = gp_float(c, -0x4550);
            abi::set_fpr(c, 3, k);
            let l = FpuF32::from_i32(level as i32);
            let data = lw_at(c, song, 0);
            set_reg(c, V1, data);
            let f = lwc1_at(c, fade, 0);
            abi::set_fpr(c, 2, f);
            let v = FpuF32::from_i32(lwc1_at(c, data, 0x88).0 as i32);
            abi::set_fpr(c, 1, v);
            let x = l * k * v;
            abi::set_fpr(c, 0, x);
            x * f
        };
        c.cycles += 7;
        let master = gp_float(c, 0x418);
        abi::set_fpr(c, 0, master);
        let level = level * master;
        abi::set_fpr(c, 20, level);
        call_ptr(c, table::SND_LOCK, 0x002b_2384)?;
        set_reg(c, A0, song);
        callee::SONG_GET_STREAM_HANDLE.call(c, 0x002b_238c)?;
        let handle = reg(c, V0);
        set_reg(c, A0, handle);
        if (handle as i64) >= 0 {
            c.cycles += 10;
            set_reg(c, AT, 0x42fe_0000);
            c.f[1] = 0;
            let v = fr(c, 20).min(FpuF32(0x42fe_0000)).max(FpuF32::ZERO);
            abi::set_fpr(c, 0, v);
            let v = to_int_via(c, 1, v);
            set_reg(c, A1, sx((((v as u32) << 24) as i32 >> 24) as u32));
            call_ptr(c, table::SNDSTRM_SET_VOLUME, 0x002b_23bc)?;
        }
        c.cycles += 2;
        call_ptr(c, table::SND_UNLOCK, 0x002b_23c4)?;
        Ok(song_exit(c, frame, true))
    })
}

/// The registration step of [`song_instance_update`].
fn register_song(c: &mut Cpu, frame: Frame, song: u64) -> Flow {
    c.cycles += 31;
    set_reg(c, V0, 1);
    sw_at(c, song, 0x2c, 1);
    let name = addiu(song, 0x60);
    set_reg(c, S1, name);
    callee::MUSIC_GET_SONG_DATA_BUFFER.call(c, 0x002b_222c)?;
    let handle_out = addiu(song, 4);
    set_reg(c, S2, handle_out);
    set_reg(c, A0, reg(c, V0));
    call_ptr(c, table::PATH_ADD_MAP, 0x002b_2238)?;
    let data = lw_at(c, song, 0);
    set_reg(c, V0, 0x004a_0000);
    let fmt = 0x004a_36e0u64;
    set_reg(c, S3, fmt);
    set_reg(c, A0, reg(c, SP));
    set_reg(c, A3, addiu(data, 0x20));
    set_reg(c, A1, fmt);
    set_reg(c, A2, name);
    call_ptr(c, table::SPRINTF, 0x002b_2258)?;
    add_track(c, song, 1, 0x24, [0x1f4, 2, 0x96, 0x37000], 0x002b_2284)?;
    let data = lw_at(c, song, 0);
    set_reg(c, A3, data);
    sw_at(c, song, 0x1c, reg(c, V0));
    let tracks = lw_at(c, data, 0x40);
    set_reg(c, V0, tracks);
    set_reg(c, A0, reg(c, SP));
    if (tracks as i64) > 0 {
        c.cycles += 16;
        set_reg(c, A1, fmt);
        set_reg(c, A2, name);
        set_reg(c, A3, addiu(data, 0x44));
        call_ptr(c, table::SPRINTF, 0x002b_22a8)?;
        add_track(c, song, 2, 0x28, [0xa, 1, 0, 0x6a400], 0x002b_22d4)?;
        sw_at(c, song, 0x20, reg(c, V0));
    }
    let _ = frame;
    c.cycles += 4;
    let event = lw_at(c, song, 0x30);
    set_reg(c, A1, event);
    set_reg(c, S1, u64::MAX);
    set_reg(c, A0, song);
    if event != u64::MAX {
        c.cycles += 4;
        set_reg(c, A2, 1);
        callee::SONG_POST_EVENT.call(c, 0x002b_22f0)?;
        sw_at(c, song, 0x30, u64::MAX);
    }
    Ok(())
}

/// `PATH_AddTrack(0x11000000 | n, name = sp, a2, &song->handle, t0, t1,
/// song->field, t3)`.
fn add_track(c: &mut Cpu, song: u64, n: u64, field: u32, args: [u64; 4], ret: u32) -> Flow {
    let v = lw_at(c, song, field);
    set_reg(c, T2, v);
    set_reg(c, A3, reg(c, S2));
    set_reg(c, A0, 0x1100_0000 | n);
    set_reg(c, A1, reg(c, SP));
    set_reg(c, A2, args[0]);
    set_reg(c, T0, args[1]);
    set_reg(c, T1, args[2]);
    set_reg(c, T3, args[3]);
    call_ptr(c, table::PATH_ADD_TRACK, ret)
}

fn song_exit(c: &mut Cpu, frame: Frame, restore_s0: bool) {
    if restore_s0 {
        c.cycles += 1;
        frame.restore(c, S0, 0x140);
    }
    c.cycles += 7;
    restore_regs(c, frame, &[(S1, 0x130), (S2, 0x120), (S3, 0x110)]);
    frame.restore_ra(c, 0x100);
    frame.restore_fpr(c, 20, 0x150);
    frame.close(c);
}

/// `SSXAudioSystem_UpdateRiderWorldPaint(sys, rider)` 0x2898a8: for player
/// 0 or 1, look up the world paint at the rider's position (virtual call,
/// slot +0x28 of the controller; the position quadword is copied to
/// `sp+0`). When the paint grid changed (`f_0028d960`), sample the two
/// per-player paint layers at (x, z) with a gp radius (virtual call, slot
/// +0x10 of each layer at `layer + 4`) and apply them (`f_0028d988`,
/// `ApplyMixPaint`); then the third layer likewise (`f_002899e8` /
/// `f_002899f8`).
pub fn ssx_audio_update_rider_world_paint(c: &mut Cpu) {
    const SAVED: [(usize, u32); 4] = [(S1, 0x40), (S2, 0x30), (S0, 0x50), (S3, 0x20)];
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x70);
        save_regs(c, frame, &SAVED);
        frame.save_ra(c, 0x10);
        frame.save_fpr(c, 20, 0x60);
        let (sys, rider) = (reg(c, A0), reg(c, A1));
        set_reg(c, S2, rider);
        let player = lw_at(c, rider, rider::ID);
        set_reg(c, S3, player);
        let ok = (player < 2) as u64;
        set_reg(c, V0, ok);
        set_reg(c, S1, sys);
        if ok != 0 {
            c.cycles += 12;
            let ctl = addiu(rider, rider::CONTROLLER as i32);
            let vt = lw_at(c, rider, rider::CONTROLLER);
            set_reg(c, V1, vt);
            set_reg(c, A1, ctl);
            virtual_call(c, ctl, vt, 0x28, V0, 0x0028_98f0)?;
            let pos = reg(c, V0) as u32;
            c.lq(V1, pos);
            set_reg(c, A0, sys);
            let sp = abi::r32(c, SP);
            c.sq(V1, sp);
            callee::RIDER_PAINT_CHANGED.call(c, 0x0028_9900)?;
            let changed = reg(c, V0) != 0;
            let slot = sx((player as u32) << 2);
            set_reg(c, S0, slot);
            if changed {
                c.cycles += 29;
                let radius = gp_float(c, -0x45e0);
                abi::set_fpr(c, 20, radius);
                let layer = lw_at(c, addu(sys, slot), sys::PAINT_A);
                set_reg(c, V0, addu(sys, slot));
                sample_paint(c, layer, 20, 0x0028_9934)?;
                set_reg(c, A2, reg(c, V0));
                set_reg(c, A1, rider);
                set_reg(c, A0, sys);
                callee::RIDER_PAINT_SET.call(c, 0x0028_9944)?;
                let base = addu(sys, slot);
                set_reg(c, S0, base);
                let layer = lw_at(c, base, sys::PAINT_B);
                sample_paint(c, layer, 20, 0x0028_996c)?;
                set_reg(c, A2, reg(c, V0));
                set_reg(c, A0, sys);
                set_reg(c, A1, rider);
                callee::APPLY_MIX_PAINT.call(c, 0x0028_997c)?;
            }
            c.cycles += 4;
            set_reg(c, A0, sys);
            callee::RIDER_PAINT2_CHANGED.call(c, 0x0028_9984)?;
            let changed = reg(c, V0) != 0;
            let slot = sx((player as u32) << 2);
            set_reg(c, V0, slot);
            if changed {
                c.cycles += 14;
                let base = addu(sys, slot);
                set_reg(c, V0, base);
                let layer = lw_at(c, base, sys::PAINT_C);
                let r = gp_float(c, -0x45dc);
                c.f[14] = r.0;
                sample_paint(c, layer, 14, 0x0028_99b4)?;
                set_reg(c, A0, sys);
                set_reg(c, A1, rider);
                set_reg(c, A2, reg(c, V0));
                callee::RIDER_PAINT2_SET.call(c, 0x0028_99c4)?;
            }
        }
        c.cycles += 8;
        restore_regs(c, frame, &SAVED);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

/// Sample a paint layer at the copied position: `f12 = x`, `f13 = sp+4`,
/// `f14 = radius` (from FPU register `radius`), virtual call slot +0x10 of
/// the vtable at `layer + 4`.
fn sample_paint(c: &mut Cpu, layer: u64, radius: usize, ret: u32) -> Flow {
    set_reg(c, A1, layer);
    let sp = abi::r32(c, SP);
    c.f[12] = Addr(sp).read_u32(c);
    c.f[14] = c.f[radius];
    c.f[13] = Addr(sp + 4).read_u32(c);
    let vt = lw_at(c, layer, 4);
    set_reg(c, V1, vt);
    virtual_call(c, layer, vt, 0x10, V0, ret)
}

/// `SSXAudioSystem_UpdateMusicFadeIn(sys)` 0x28f000: while the music player
/// is in states 1..3 hold it at full volume; otherwise fade it in over
/// 800 frames (1000 when the player's music state is 1, 1200 when 2),
/// counting frames at `sys + 0x628c` (state 2 at `+0x6288` starts the
/// count), at `127 * clamp(frames / length, 0, 1)`.
///
/// Tick rate: the count advances once per tick, so the length (60 Hz frames)
/// is converted with `tick::ticks_from_frames60` (13-20 s at every rate).
pub fn ssx_audio_update_music_fade_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        save_regs(c, frame, &[(S0, 0x20), (S1, 0x10)]);
        frame.save_ra(c, 0);
        let sys = reg(c, A0);
        set_reg(c, S0, sys);
        let music = addiu(sys, sys::ROOT as i32);
        set_reg(c, S1, music);
        set_reg(c, A0, music);
        call_ptr(c, table::MUSIC_GET_STATE, 0x0028_f020)?;
        let busy = (addiu(reg(c, V0), -1) < 3) as u64;
        set_reg(c, V0, busy);
        set_reg(c, A0, music);
        if busy != 0 {
            c.cycles += 4;
            set_reg(c, A1, 0x7f);
            call_ptr(c, table::MUSIC_SET_VOLUME, 0x0028_f038)?;
        } else {
            c.cycles += 4;
            let state = lw_at(c, sys, sys::MUSIC_FADE_STATE);
            set_reg(c, V0, state);
            set_reg(c, V1, 2);
            set_reg(c, A0, sys);
            if state != 2 {
                c.cycles += 2;
                sw_at(c, sys, sys::MUSIC_FADE_STATE, 2);
                sw_at(c, sys, sys::MUSIC_FADE_FRAMES, 0);
            }
            c.cycles += 10;
            set_reg(c, A1, u64::MAX);
            callee::FIND_RIDER.call(c, 0x0028_f060)?;
            set_reg(c, S1, 0x320);
            set_reg(c, A1, reg(c, V0));
            set_reg(c, A0, sys);
            callee::RIDER_MUSIC_STATE.call(c, 0x0028_f070)?;
            let m = reg(c, V0);
            set_reg(c, V0, 1);
            set_reg(c, V1, m ^ 2);
            c.cycles += 2;
            if m == 1 {
                set_reg(c, S1, 0x3e8);
            } else {
                set_reg(c, V0, 0x4b0);
                if m ^ 2 == 0 {
                    set_reg(c, S1, 0x4b0);
                }
            }
            c.cycles += 9;
            // The fade length is in 60 Hz frames: the same wall-clock fade
            // at any rate (exact at 60; `s1` is restored below).
            let length = crate::tick::ticks_from_frames60(c, reg(c, S1) as u32) as u64;
            set_reg(c, S1, length);
            let frames = lw_at(c, sys, sys::MUSIC_FADE_FRAMES);
            set_reg(c, V1, frames);
            let fl = FpuF32::from_i32(length as i32);
            abi::set_fpr(c, 0, fl);
            let ff = FpuF32::from_i32(frames as i32);
            abi::set_fpr(c, 1, ff);
            let ratio = ff.div(fl);
            abi::set_fpr(c, 3, ratio);
            set_reg(c, V0, addiu(frames, 1));
            if slt(frames, length) != 0 {
                c.cycles += 2;
                sw_at(c, sys, sys::MUSIC_FADE_FRAMES, addiu(frames, 1));
            } else {
                c.cycles += 1;
                sw_at(c, sys, sys::MUSIC_FADE_FRAMES, length);
            }
            c.cycles += 13;
            abi::set_fpr(c, 0, FpuF32::ONE);
            c.f[2] = 0;
            let r = ratio.min(FpuF32::ONE);
            set_reg(c, AT, 0x42fe_0000);
            let r = r.max(FpuF32::ZERO);
            abi::set_fpr(c, 3, r);
            let v = r * FpuF32(0x42fe_0000);
            abi::set_fpr(c, 1, v);
            let v = to_int_via(c, 0, v);
            set_reg(c, A1, v);
            set_reg(c, A0, music);
            call_ptr(c, table::MUSIC_SET_VOLUME, 0x0028_f0f0)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem_UpdateCountdownBeeps(sys)` 0x29cca8: outside replays,
/// while the race countdown runs (the game's countdown object is set and
/// active, and bit 9 of the flags at 0x5308d0 is clear), compute the
/// seconds left (`(fixunssfsi(countdown->time * k2) - raceTime * k1) %
/// 3600`); on a change to 10, 8, 6 or 1..4 seconds queue a beep timer
/// (`BankMonitor_AddTimer(bm, 450, pmf, gainPtr(9, 0), 0, 0x4e, 0x7f, 0)`),
/// and remember the second.
pub fn ssx_audio_update_countdown_beeps(c: &mut Cpu) {
    const SAVED: [(usize, u32); 4] = [(S3, 0x10), (S0, 0x40), (S1, 0x30), (S2, 0x20)];
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        save_regs(c, frame, &SAVED);
        frame.save_ra(c, 0);
        let sys = reg(c, A0);
        set_reg(c, S3, sys);
        call_ptr(c, table::IS_REPLAY_PLAYING, 0x0029_ccc8)?;
        let replay = reg(c, V0) != 0;
        frame.restore(c, S0, 0x40);
        if !replay {
            countdown(c, frame, sys)?;
        }
        c.cycles += 6;
        restore_regs(c, frame, &[(S1, 0x30), (S2, 0x20), (S3, 0x10)]);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The countdown part of [`ssx_audio_update_countdown_beeps`]; ends with s0
/// restored.
fn countdown(c: &mut Cpu, frame: Frame, sys: u64) -> Flow {
    c.cycles += 4;
    let game = gp_lw(c, -(gp::GAME as i32));
    set_reg(c, V0, game);
    let cd = lw_at(c, game, 0xc0);
    set_reg(c, S0, cd);
    if cd == 0 {
        frame.restore(c, S0, 0x40);
        return Ok(());
    }
    c.cycles += 3;
    let active = lw_at(c, cd, 0x88);
    set_reg(c, V0, active);
    set_reg(c, V1, 0);
    if active != 0 {
        c.cycles += 5;
        set_reg(c, V1, 0x0053_0000);
        let flags = Addr(0x0053_08d0).lw(c);
        let bit = sx(((flags as u32) as i32 >> 9) as u32) & 1;
        set_reg(c, V0, bit);
        set_reg(c, V1, bit ^ 1);
    }
    c.cycles += 2;
    if reg(c, V1) == 0 {
        frame.restore(c, S0, 0x40);
        return Ok(());
    }
    c.cycles += 11;
    call_ptr(c, table::GET_RACE_MAN, 0x0029_cd10)?;
    let race_time = FpuF32::from_i32(lwc1_at(c, reg(c, V0), 0xc).0 as i32);
    let k1 = gp_float(c, -0x459c);
    abi::set_fpr(c, 1, k1);
    let time = lw_at(c, cd, 0x78);
    set_reg(c, A0, time);
    let elapsed = race_time * k1;
    abi::set_fpr(c, 0, elapsed);
    let elapsed = to_int_via(c, 1, elapsed);
    set_reg(c, S0, elapsed);
    let half = sx((time as u32) >> 1);
    set_reg(c, V1, half);
    // `(float)(unsigned)time`.
    let t = if (time as i64) >= 0 {
        c.cycles += 4;
        FpuF32::from_i32(time as i32)
    } else {
        c.cycles += 5;
        let v = (time & 1) | half;
        set_reg(c, V0, v);
        let h = FpuF32::from_i32(v as i32);
        h + h
    };
    abi::set_fpr(c, 0, t);
    c.cycles += 8;
    let k2 = gp_float(c, -0x4598);
    abi::set_fpr(c, 12, t * k2);
    call_ptr(c, table::FIXUNSSFSI, 0x0029_cd64)?;
    set_reg(c, V1, 0xe10);
    let n = sx((reg(c, V0) as u32).wrapping_sub(elapsed as u32));
    set_reg(c, V0, n);
    abi::div(c, n as u32, 0xe10);
    c.cycles += 3;
    let seconds = c.hi[0];
    set_reg(c, S2, seconds);
    if (seconds as i64) <= 0 {
        frame.restore(c, S0, 0x40);
        return Ok(());
    }
    c.cycles += 3;
    let last = lw_at(c, sys, sys::COUNTDOWN_LAST);
    set_reg(c, V0, slt(seconds, 11));
    let mut beep = false;
    if seconds != last {
        c.cycles += 2;
        if slt(seconds, 11) != 0 {
            set_reg(c, V0, 10);
            c.cycles += 2;
            beep = seconds == 10;
            if !beep {
                set_reg(c, V0, 8);
                c.cycles += 2;
                beep = seconds == 8;
            }
            if !beep {
                set_reg(c, V0, 6);
                c.cycles += 2;
                beep = seconds == 6;
            }
            if !beep {
                set_reg(c, V0, slt(seconds, 5));
                c.cycles += 2;
                if slt(seconds, 5) == 0 {
                    sw_at(c, sys, sys::COUNTDOWN_LAST, seconds);
                    c.cycles += 1;
                    frame.restore(c, S0, 0x40);
                    return Ok(());
                }
                beep = true;
            }
        } else {
            set_reg(c, V0, 10);
        }
    }
    if beep {
        c.cycles += 19;
        let root = lw_at(c, sys, sys::ROOT);
        set_reg(c, V0, root);
        let pmf_at = 0x004a_3678u32;
        set_reg(c, S1, pmf_at as u64);
        set_reg(c, A0, sys);
        let snd = lw_at(c, root, 0);
        set_reg(c, A1, 9);
        set_reg(c, A2, 0);
        set_reg(c, S0, addiu(snd, sys::SND_BANK_MONITOR as i32));
        call_ptr(c, table::GET_CHANNEL_GAIN_PTR, 0x0029_cddc)?;
        let pmf = Addr(pmf_at).read_u64_unaligned(c);
        set_reg(c, A2, pmf);
        set_reg(c, A0, reg(c, S0));
        set_reg(c, A3, reg(c, V0));
        set_reg(c, A1, 0x1c2);
        set_reg(c, T0, 0);
        set_reg(c, T1, 0x4e);
        set_reg(c, T2, 0x7f);
        set_reg(c, T3, 0);
        callee::ADD_TIMER.call(c, 0x0029_ce04)?;
    }
    c.cycles += 1;
    sw_at(c, sys, sys::COUNTDOWN_LAST, seconds);
    c.cycles += 1;
    frame.restore(c, S0, 0x40);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
    use crate::difftest::{check, check_lohi, elf_image, fill, Rng, DATA, GAME_GP};
    use ssx3_ee::{s07, s08};

    const SEEDS: u32 = 150;
    /// The `SND_Lock` / `SND_Unlock` hooks.
    const LOCK_HOOK: u32 = 0x0044_c46c;

    // Test world layout.
    const SYS: u32 = DATA;
    const RIDER: u32 = DATA + 0x1_0000;
    const INST: u32 = DATA + 0x2_0000;
    const STATE_OBJ: u32 = DATA + 0x2_1000;
    const ROOT: u32 = DATA + 0x2_2000;
    const SND: u32 = DATA + 0x2_3000;
    const BM: u32 = SND + sys::SND_BANK_MONITOR;
    const VTABLES: u32 = DATA + 0x2_8000;
    const GAME: u32 = DATA + 0x3_0000;
    const MODULE: u32 = DATA + 0x3_1000;
    const REPLAY: u32 = DATA + 0x3_2000;
    const RACE_MAN: u32 = DATA + 0x3_3000;
    const RIDERS: u32 = DATA + 0x4_0000;

    /// A whole register: mostly a sign-extended word, sometimes garbage above.
    fn r64(r: &mut Rng, v: u32) -> u64 {
        if r.next_u32() % 4 == 0 {
            (r.next_u32() as u64) << 32 | v as u64
        } else {
            sx(v)
        }
    }

    /// Pick one of `vals`.
    fn pick(r: &mut Rng, vals: &[u32]) -> u32 {
        vals[(r.next_u32() as usize) % vals.len()]
    }

    /// A small float in [-range, range), or an edge case.
    fn float(r: &mut Rng, range: f32) -> u32 {
        if r.next_u32() % 6 == 0 {
            r.edge_f32()
        } else {
            r.f32(range)
        }
    }

    /// The machine with fakes, the game image (curves, jump tables, gp
    /// constants) and fake lock hooks; forced fake results `1..=3` (player
    /// indices, statuses) for the first calls.
    fn base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        elf_image(c);
        // Pre-initialise the lazily built BE singleton (`gp - 0x1f04`): with
        // it zero the generated `f_0014dc80` would build one through the
        // file system and run away on test data.
        c.bus.write32(GAME_GP - 0x1f04, DATA + 0x6_0000);
        c.bus.write32(GAME_GP - 0x1efc, 0);
        c.bus.write32(LOCK_HOOK, FAKE + 0x100);
        c.bus.write32(LOCK_HOOK + 4, FAKE + 0x110);
        for i in 0..8 {
            let v = if r.next_u32() % 2 == 0 { 0 } else { 1 + r.next_u32() % 3 };
            c.bus.write32(LOG_RESULTS + 4 * i, v);
        }
        r
    }

    /// The audio system, a rider, an instance of it, the sound root and the
    /// game chain, all with randomised fields that steer the branches.
    fn world(c: &mut Cpu, r: &mut Rng) {
        // Game chain: gp-0x848 -> game; +0x84 module; +0x28 replay; +0xc race man.
        c.bus.write32(GAME_GP - gp::GAME, GAME);
        c.bus.write32(GAME + 0x84, if r.next_u32() % 6 == 0 { 0 } else { MODULE });
        c.bus.write32(MODULE + 0x28, if r.next_u32() % 3 == 0 { 0 } else { REPLAY });
        c.bus.write32(MODULE + 0xc, RACE_MAN);
        c.bus.write32(MODULE + 0x214, pick(r, &[0, 4, 4, 2]));
        c.bus.write32(REPLAY, pick(r, &[0, 1, 5, 12, 20]));
        c.bus.write32(GAME_GP - gp::PAUSE_FLAGS, r.next_u32() % 4);
        // The rider.
        c.bus.write32(RIDER + 0x77c, STATE_OBJ);
        c.bus.write32(STATE_OBJ + 0xde0, pick(r, &[0, 0, 1, 2, 4, 5, 3]));
        c.bus.write32(RIDER + 0x438, (r.next_u32() % 24).wrapping_sub(2));
        c.bus.write32(SYS + 0x598c, (r.next_u32() % 24).wrapping_sub(2));
        c.bus.write32(RIDER + rider::AUDIO_STATE, r.next_u32() % 3);
        c.bus.write32(RIDER + rider::SURFACE_SOUND, (r.next_u32() % 24).wrapping_sub(2));
        c.bus.write32(RIDER + rider::EDGE_SOUND, (r.next_u32() % 24).wrapping_sub(2));
        for off in [rider::SPEED_A, rider::SPEED_B, rider::SPEED_C] {
            c.bus.write32(RIDER + off, float(r, 1.0));
        }
        c.bus.write32(RIDER + rider::SPEED, float(r, 60.0));
        c.bus.write32(RIDER + rider::AIR, float(r, 2.0));
        c.bus.write32(RIDER + rider::ID, pick(r, &[0, 1, 2, 5]));
        // Controller vtable: slots 0x28 (position) and 0x38 (player index) are fakes.
        c.bus.write32(RIDER + rider::CONTROLLER, VTABLES);
        for (i, off) in [0x28u32, 0x38].into_iter().enumerate() {
            c.bus.write16(VTABLES + off, (r.next_u32() % 4 * 4) as u16);
            c.bus.write32(VTABLES + off + 4, FAKE + 0x200 + 0x10 * i as u32);
        }
        // Per-player smoothed speeds.
        for i in 0..4 {
            c.bus.write32(SYS + sys::SURFACE_SPEED + 4 * i, float(r, 1.0));
            c.bus.write32(SYS + sys::EDGE_SPEED + 4 * i, float(r, 2.0));
        }
        // The instance.
        c.bus.write32(INST + bmi::RIDER, RIDER);
        c.bus.write32(INST + bmi::STATE, r.next_u32() % 5);
        // Sound root.
        c.bus.write32(SYS + sys::ROOT, ROOT);
        c.bus.write32(ROOT, SND);
    }

    // --- timers ------------------------------------------------------------

    #[test]
    fn add_timer() {
        check_lohi(s08::f_002adca0, bank_monitor_add_timer, SEEDS, &|c, seed| {
            let mut r = base(c, seed, 1, 0);
            let bm = BM;
            c.bus.write32(bm + bank_monitor::TIMER_COUNT, r.next_u32() % 4);
            let all_busy = r.next_u32() % 4 == 0;
            for i in 0..48 {
                let t = bm + bank_monitor::TIMERS + i * bank_monitor::TIMER_SIZE;
                c.bus.write32(t, (all_busy || r.next_u32() % 2 == 0) as u32);
            }
            c.g[A0][0] = sx(bm);
            let d = r.next_u32() % 1000;
            c.g[A1][0] = r64(&mut r, d);
            c.g[A2][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
        });
    }

    #[test]
    fn timer_update() {
        covered(s08::f_002ab828, bank_monitor_timer_update, 150, &|c, seed| {
            let mut r = base(c, seed, 2, 2);
            let t = INST;
            let owner = SYS;
            c.bus.write32(t + timer::ACTIVE, (r.next_u32() % 4 != 0) as u32);
            c.bus.write32(t + timer::PERIOD, r.next_u32() % 100);
            c.bus.write32(t + timer::LEFT, (r.next_u32() % 30).wrapping_sub(5));
            // A plain function (a fake, or GetGameModule returning 0 here) or
            // a virtual one.
            let delta = (r.next_u32() % 8 * 4) as u16;
            c.bus.write16(t + timer::PMF, delta);
            if r.next_u32() % 2 == 0 {
                c.bus.write16(t + timer::PMF + 2, 0xffff);
                let f = if r.next_u32() % 2 == 0 { FAKE + 0x300 } else { 0x0028_b1c8 };
                c.bus.write32(t + timer::PMF + 4, f);
                c.bus.write32(GAME_GP - gp::GAME, 0);
            } else {
                let index = 1 + r.next_u32() % 3;
                let vptr_off = r.next_u32() % 16 * 4;
                c.bus.write16(t + timer::PMF + 2, index as u16);
                c.bus.write16(t + timer::PMF + 4, vptr_off as u16);
                c.bus.write32(owner + vptr_off, VTABLES);
                let entry = VTABLES + (index - 1) * 8;
                c.bus.write32(entry, r.next_u32());
                c.bus.write32(entry + 4, FAKE + 0x310);
            }
            c.g[A0][0] = sx(t);
            c.g[A1][0] = sx(owner);
        });
    }

    // --- BankMonitorInstance --------------------------------------------------

    fn instance(c: &mut Cpu, r: &mut Rng) {
        fill(c, r, INST, 0x30);
        c.bus.write32(INST + bmi::STATE, r.next_u32() % 6);
        c.bus.write32(INST + bmi::HANDLE, r.next_u32() % 8);
        c.bus.write32(INST + bmi::STREAM, DATA + 0x5_0000);
        c.g[A0][0] = sx(INST);
    }

    #[test]
    fn is_done() {
        covered(s08::f_002aba40, bank_monitor_instance_is_done, 150, &|c, seed| {
            let mut r = base(c, seed, 3, 3);
            instance(c, &mut r);
            for i in 0..8 {
                let v = pick(&mut r, &[0, 1, 2, 3, 0xffff_ffff]);
                c.bus.write32(LOG_RESULTS + 4 * i, v);
            }
        });
    }

    #[test]
    fn set_pan() {
        covered(s08::f_002abd90, bank_monitor_instance_set_pan, 150, &|c, seed| {
            let mut r = base(c, seed, 4, 3);
            instance(c, &mut r);
            let last = r.next_u32() % 0x800;
            c.bus.write32(INST + bmi::LAST_PAN, last);
            let pan = if r.next_u32() % 3 == 0 { last.wrapping_add(r.next_u32() % 0x400).wrapping_sub(0x200) } else { r.next_u32() % 0x1000 };
            c.g[A1][0] = r64(&mut r, pan);
            let sp = r.next_u32() % 0x8000;
            c.g[A2][0] = r64(&mut r, sp);
        });
    }

    #[test]
    fn set_volume() {
        covered(s08::f_002abf60, bank_monitor_instance_set_volume, 120, &|c, seed| {
            let mut r = base(c, seed, 5, 3);
            instance(c, &mut r);
            c.bus.write32(INST + bmi::GAIN, if r.next_u32() % 4 == 0 { r.edge_f32() } else { r.f32(2.0) });
            let last = pick(&mut r, &[0xffff_ffff, 0, 64, 127]);
            c.bus.write32(INST + bmi::LAST_VOLUME, last);
            let v = match r.next_u32() % 4 {
                0 => 0xffff_ffff,
                1 => r.next_u32(),
                _ => (last.wrapping_add(r.next_u32() % 16)).wrapping_sub(8),
            };
            c.g[A1][0] = if v == u32::MAX && r.next_u32() % 2 == 0 { u64::MAX >> 1 } else { r64(&mut r, v) };
        });
    }

    // --- shared world extensions ------------------------------------------

    /// Test addresses for the music player, speech listener, song / song data,
    /// a world-paint layer and the race countdown.
    const MP: u32 = DATA + 0x2_4000;
    const SPK: u32 = DATA + 0x2_5000;
    const SONG: u32 = DATA + 0x2_6000;
    const SONG_DATA: u32 = DATA + 0x2_7000;
    const LAYER: u32 = DATA + 0x2_9000;
    const CD: u32 = DATA + 0x3_4000;

    /// `gp`-relative audio globals (music player, speech listener) and the
    /// vtable slots the virtual calls in these functions use (player index
    /// 0x38 and position 0x28 are set by [`world`]).
    fn audio_globals(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(GAME_GP + gp::MUSIC_PLAYER, MP);
        c.bus.write32(GAME_GP + gp::SPEECH_LISTENER, SPK);
        c.bus.write32(MP + 0x408, if r.next_u32() % 3 == 0 { 0 } else { MP + 0x1000 });
        c.bus.write32(MP + 0x418, r.next_u32() % 2);
        c.bus.write32(MP + 0x58, r.next_u32() % 4);
        c.bus.write32(MP + 0x5434, r.next_u32());
        c.bus.write32(MP + 0x5438, if r.next_u32() % 2 == 0 { 0 } else { LAYER });
        c.bus.write32(SPK + 0x1bc, VTABLES);
        for (i, off) in [0x10u32, 0x18, 0x20, 0x30].into_iter().enumerate() {
            c.bus.write16(VTABLES + off, (r.next_u32() % 4 * 4) as u16);
            c.bus.write32(VTABLES + off + 4, FAKE + 0x400 + 0x10 * i as u32);
        }
    }

    /// The game → module → race manager chain plus a small rider table, so
    /// `FindRider` (both its replay and its search paths) can be steered.
    fn find_rider_world(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(GAME_GP - gp::GAME, GAME);
        c.bus.write32(GAME + 0x84, MODULE);
        c.bus.write32(MODULE + 0xc, RACE_MAN);
        let replay = r.next_u32() % 3 != 0;
        c.bus.write32(MODULE + 0x28, if replay { REPLAY } else { 0 });
        c.bus.write32(REPLAY, pick(r, &[0, 1, 2, 5, 9, 12]));
        c.bus.write32(RACE_MAN + 0x40, if r.next_u32() % 4 == 0 { 0 } else { REPLAY });
        c.bus.write32(REPLAY + 0x18, RIDERS);
        c.bus.write32(RACE_MAN + 0x7c, r.next_u32() % 5);
        c.bus.write32(RACE_MAN + 0x78, r.next_u32() % 5);
        for i in 0..4 {
            let rd = RIDERS + i * 0x1000;
            c.bus.write32(RACE_MAN + 0x28 + 4 * i, rd);
            c.bus.write32(rd + rider::HUMAN, if r.next_u32() % 2 == 0 { 0 } else { 1 });
            c.bus.write32(rd + rider::LOCAL, if r.next_u32() % 2 == 0 { 0 } else { 1 });
            c.bus.write32(rd + rider::RANK, (r.next_u32() % 8).wrapping_sub(1));
            c.bus.write32(rd + rider::ID, r.next_u32() % 4);
        }
    }

    // --- loop parameters from curves ---------------------------------------

    fn calc_params(c: &mut Cpu, r: &mut Rng) {
        c.g[A0][0] = sx(SYS);
        c.g[A1][0] = { let v = r.next_u32() % 20; r64(r, v) };
        c.g[A2][0] = sx(RIDER);
        c.g[A3][0] = sx(DATA + 0x10);
        c.g[T0][0] = sx(DATA + 0x14);
    }

    #[test]
    fn calc_surface_loop_params() {
        check(ssx3_ee::s07::f_00293290, ssx_audio_calc_surface_loop_params, 150, &|c, seed| {
            let mut r = base(c, seed, 0x20, 4);
            world(c, &mut r);
            audio_globals(c, &mut r);
            calc_params(c, &mut r);
            c.pc = 0x0029_3290;
        });
    }

    #[test]
    fn calc_surface_loop_params_remote() {
        check(ssx3_ee::s07::f_00293350, ssx_audio_calc_surface_loop_params_remote, 150, &|c, seed| {
            let mut r = base(c, seed, 0x21, 4);
            world(c, &mut r);
            audio_globals(c, &mut r);
            calc_params(c, &mut r);
        });
    }

    #[test]
    fn calc_edge_loop_params() {
        check(ssx3_ee::s07::f_00293410, ssx_audio_calc_edge_loop_params, 150, &|c, seed| {
            let mut r = base(c, seed, 0x22, 4);
            world(c, &mut r);
            audio_globals(c, &mut r);
            calc_params(c, &mut r);
        });
    }

    #[test]
    fn calc_board_base_loop_params() {
        check(ssx3_ee::s07::f_00293168, ssx_audio_calc_board_base_loop_params, 150, &|c, seed| {
            let mut r = base(c, seed, 0x23, 0);
            world(c, &mut r);
            audio_globals(c, &mut r);
            c.g[A0][0] = sx(SYS);
            c.g[A1][0] = sx(RIDER);
            c.g[A2][0] = sx(DATA + 0x10);
            c.g[A3][0] = sx(DATA + 0x14);
            c.pc = 0x0029_3168;
        });
    }

    #[test]
    fn calc_surface_loop_group0() {
        covered(ssx3_ee::s07::f_00293538, ssx_audio_calc_surface_loop_group0, 150, &|c, seed| {
            let mut r = base(c, seed, 0x24, 2);
            world(c, &mut r);
            audio_globals(c, &mut r);
            c.g[A0][0] = sx(SYS);
            c.g[A1][0] = sx(RIDER);
            c.g[A2][0] = sx(DATA + 0x10);
            c.g[A3][0] = sx(DATA + 0x14);
        });
    }

    #[test]
    fn calc_edge_loop_group0() {
        covered(ssx3_ee::s07::f_00293bb0, ssx_audio_calc_edge_loop_group0, 150, &|c, seed| {
            let mut r = base(c, seed, 0x25, 2);
            world(c, &mut r);
            audio_globals(c, &mut r);
            c.g[A0][0] = sx(SYS);
            c.g[A1][0] = sx(RIDER);
            c.g[A2][0] = sx(DATA + 0x10);
            c.g[A3][0] = sx(DATA + 0x14);
        });
    }

    // --- board loops -------------------------------------------------------

    #[test]
    fn board_base_loop() {
        check(ssx3_ee::s07::f_00292cb8, ssx_audio_update_board_base_loop, 150, &|c, seed| {
            let mut r = base(c, seed, 0x26, 3);
            world(c, &mut r);
            audio_globals(c, &mut r);
            c.g[A0][0] = sx(SYS);
            c.g[A1][0] = sx(INST);
        });
    }

    #[test]
    fn board_surface_loop() {
        check(ssx3_ee::s07::f_00292da0, ssx_audio_update_board_surface_loop, 150, &|c, seed| {
            let mut r = base(c, seed, 0x27, 4);
            world(c, &mut r);
            audio_globals(c, &mut r);
            c.g[A0][0] = sx(SYS);
            c.g[A1][0] = sx(INST);
        });
    }

    #[test]
    fn board_surface_loop_remote() {
        check(ssx3_ee::s07::f_00292ef8, ssx_audio_update_board_surface_loop_remote, 150, &|c, seed| {
            let mut r = base(c, seed, 0x28, 4);
            world(c, &mut r);
            audio_globals(c, &mut r);
            c.g[A0][0] = sx(SYS);
            c.g[A1][0] = sx(INST);
        });
    }

    #[test]
    fn board_edge_loop() {
        check(ssx3_ee::s07::f_00293018, ssx_audio_update_board_edge_loop, 150, &|c, seed| {
            let mut r = base(c, seed, 0x29, 4);
            world(c, &mut r);
            audio_globals(c, &mut r);
            c.g[A0][0] = sx(SYS);
            c.g[A1][0] = sx(INST);
            c.pc = 0x0029_3018;
        });
    }

    // --- FindRider / paint -------------------------------------------------

    #[test]
    fn find_rider() {
        check(ssx3_ee::s07::f_00285d98, ssx_audio_find_rider, 150, &|c, seed| {
            let mut r = base(c, seed, 0x2a, 0);
            find_rider_world(c, &mut r);
            c.g[A0][0] = sx(SYS);
            c.g[A1][0] = { let v = pick(&mut r, &[0xffff_ffff, 0xffff_fffe, 0, 1, 2, 3, 7]); r64(&mut r, v) };
            c.pc = 0x0028_5d98;
        });
    }

    #[test]
    fn apply_mix_paint() {
        covered(ssx3_ee::s07::f_00289c98, ssx_audio_apply_mix_paint, 150, &|c, seed| {
            let mut r = base(c, seed, 0x2b, 2);
            find_rider_world(c, &mut r);
            audio_globals(c, &mut r);
            c.bus.write32(LAYER + 4, VTABLES);
            c.g[A0][0] = sx(SYS);
            c.g[A1][0] = sx(RIDERS);
            let paint = if r.next_u32() % 3 == 0 { 0 } else { LAYER };
            c.g[A2][0] = r64(&mut r, paint);
        });
    }

    #[test]
    fn update_rider_world_paint() {
        covered(ssx3_ee::s07::f_002898a8, ssx_audio_update_rider_world_paint, 150, &|c, seed| {
            let mut r = base(c, seed, 0x2c, 4);
            world(c, &mut r);
            audio_globals(c, &mut r);
            c.bus.write32(RIDER + rider::ID, r.next_u32() % 4);
            for i in 0..2 {
                let base = SYS + 4 * i;
                for off in [sys::PAINT_A, sys::PAINT_B, sys::PAINT_C] {
                    c.bus.write32(base + off, if r.next_u32() % 4 == 0 { 0 } else { LAYER });
                }
            }
            c.bus.write32(LAYER + 4, VTABLES);
            c.bus.write32(SYS + 0x608c, pick(&mut r, &[0, 1, 2, 3, 4]));
            c.g[A0][0] = sx(SYS);
            c.g[A1][0] = sx(RIDER);
        });
    }

    // --- bank monitor instance 3D -----------------------------------------

    #[test]
    fn update_3d() {
        covered(ssx3_ee::s08::f_002ac868, bank_monitor_instance_update_3d, 120, &|c, seed| {
            let mut r = base(c, seed, 0x2d, 5);
            world(c, &mut r);
            audio_globals(c, &mut r);
            fill(c, &mut r, INST, 0x30);
            c.bus.write32(INST + bmi::RIDER, RIDER);
            c.bus.write32(INST + bmi::STATE, r.next_u32() % 5);
            c.bus.write32(INST + bmi::HANDLE, r.next_u32());
            c.bus.write32(INST + bmi::STREAM, DATA + 0x5_0000);
            c.bus.write32(INST + bmi::DISTANCE, float(&mut r, 50.0));
            c.bus.write32(INST + bmi::GAIN_PTR, if r.next_u32() % 3 == 0 { 0 } else { DATA + 0x10 });
            c.bus.write32(INST + bmi::RADIUS, float(&mut r, 40.0));
            c.bus.write32(INST + bmi::VOLUME_SCALE, pick(&mut r, &[0x7f, 0x40, 0, 200]));
            c.bus.write32(INST + bmi::FIXED, r.next_u32() % 2);
            c.bus.write32(INST + bmi::POSITIONAL, r.next_u32() % 2);
            c.bus.write32(BM + bank_monitor::LISTENER, VTABLES);
            c.bus.write32(BM + bank_monitor::DOPPLER, r.next_u32() % 2);
            c.bus.write32(BM + bank_monitor::ENABLE_3D, r.next_u32() % 2);
            c.f[12] = float(&mut r, 0.1);
            c.g[A0][0] = sx(INST);
            c.g[A1][0] = sx(BM);
        });
    }

    // --- per-frame update --------------------------------------------------

    #[test]
    fn update() {
        covered(ssx3_ee::s07::f_00285bf8, ssx_audio_update, 150, &|c, seed| {
            let mut r = base(c, seed, 0x2e, 6);
            world(c, &mut r);
            audio_globals(c, &mut r);
            c.bus.write32(SYS + sys::PAUSE_FADE_STATE, pick(&mut r, &[0, 1, 2, 3]));
            c.bus.write32(SYS + sys::STREAMS_PLAYING, r.next_u32() % 2);
            c.bus.write32(SYS + sys::MODE, pick(&mut r, &[0, 6, 6, 2]));
            c.bus.write32(SYS + sys::MUSIC_STARTED, r.next_u32() % 2);
            c.bus.write32(SYS + sys::MUSIC_FADE_STATE, r.next_u32() % 3);
            c.bus.write32(SYS + sys::MUSIC_FADE_FRAMES, r.next_u32() % 1200);
            c.bus.write32(SYS + sys::BOOST_STOP_DELAY, r.next_u32() % 3);
            c.bus.write32(RACE_MAN + 0xc, r.f32(100.0));
            c.f[12] = float(&mut r, 0.05);
            c.g[A0][0] = sx(SYS);
        });
    }

    // --- boost / speech / song --------------------------------------------

    #[test]
    fn stop_boost_sound() {
        check(ssx3_ee::s08::f_002992d8, ssx_audio_stop_boost_sound, 150, &|c, seed| {
            let mut r = base(c, seed, 0x2f, 3);
            find_rider_world(c, &mut r);
            c.bus.write32(RIDER + rider::HUMAN, r.next_u32() % 2);
            c.bus.write32(RIDER + rider::LOCAL, r.next_u32() % 2);
            c.bus.write32(SYS + sys::ROOT, ROOT);
            c.bus.write32(ROOT, SND);
            c.bus.write32(SYS + sys::BOOST_HANDLE, r.next_u32());
            // Covers the exactly-zero stop and the "never negative" clamp.
            c.bus.write32(SYS + sys::BOOST_STOP_DELAY, (r.next_u32() % 3).wrapping_sub(1));
            c.g[A0][0] = sx(SYS);
            c.g[A1][0] = sx(RIDER);
        });
    }

    #[test]
    fn speech_instance_update_matches() {
        check(ssx3_ee::s08::f_002afc88, speech_instance_update, 150, &|c, seed| {
            let mut r = base(c, seed, 0x30, 2);
            audio_globals(c, &mut r);
            c.bus.write32(INST + 0x4, r.next_u32());
            c.bus.write32(INST + 0x900, r.next_u32() % 3);
            c.g[A0][0] = sx(INST);
            c.g[A1][0] = sx(DATA + 0x5_0000);
        });
    }

    #[test]
    fn song_instance_update_matches() {
        check(ssx3_ee::s08::f_002b21e0, song_instance_update, 150, &|c, seed| {
            let mut r = base(c, seed, 0x31, 3);
            audio_globals(c, &mut r);
            c.bus.write32(SONG + 0x2c, r.next_u32() % 2);
            c.bus.write32(SONG + 0, SONG_DATA);
            c.bus.write32(SONG + 0x48, r.next_u32() % 2);
            c.bus.write32(SONG + 0x54, r.next_u32() % 2);
            c.bus.write32(SONG + 0x3c, if r.next_u32() % 3 == 0 { 0 } else { SONG_DATA + 0x100 });
            c.bus.write32(SONG + 0x40, pick(&mut r, &[0x40, 0x3f, 0x7f]));
            c.bus.write32(SONG + 0x30, pick(&mut r, &[0xffff_ffff, 0, 3]));
            c.bus.write32(SONG + 0x1c, r.next_u32());
            c.bus.write32(SONG + 0x20, r.next_u32());
            c.bus.write32(SONG_DATA + 0x9c, r.next_u32() % 128);
            c.bus.write32(SONG_DATA + 0x88, float(&mut r, 1.0));
            c.bus.write32(SONG_DATA + 0x40, r.next_u32() % 2);
            fill(c, &mut r, SONG_DATA + 0x20, 0x10);
            // Force the "song data loaded" answer (`MusicPlayer_IsSongDataLoaded`
            // returns 1 when a1 is 0).
            let arg = if seed % 2 == 0 { 0 } else { let v = r.next_u32(); r64(&mut r, v) };
            c.g[A1][0] = arg;
            c.g[A0][0] = sx(SONG);
        });
    }

    #[test]
    fn update_music_fade_in() {
        check(ssx3_ee::s07::f_0028f000, ssx_audio_update_music_fade_in, 150, &|c, seed| {
            let mut r = base(c, seed, 0x32, 2);
            find_rider_world(c, &mut r);
            audio_globals(c, &mut r);
            // `sys + 0x118` is the embedded music player.
            c.bus.write32(SYS + sys::ROOT + 0x408, if r.next_u32() % 3 == 0 { 0 } else { MP });
            c.bus.write32(SYS + sys::ROOT + 0x418, r.next_u32() % 2);
            c.bus.write32(SYS + sys::ROOT + 0x58, r.next_u32() % 4);
            c.bus.write32(SYS + sys::MUSIC_FADE_STATE, pick(&mut r, &[0, 2, 2]));
            c.bus.write32(SYS + sys::MUSIC_FADE_FRAMES, pick(&mut r, &[0, 5, 799, 800, 1200, 1600]));
            c.g[A0][0] = sx(SYS);
        });
    }

    /// `SSXAudioSystem_UpdateMusicFadeIn` on the differential fixture of
    /// `seed` at rate `hz`: `updates` updates from fade count `start` (fade
    /// state 2); the count it leaves, or `None` if an update did not return.
    fn fade_count(hz: u32, seed: u32, start: u32, updates: u32) -> Option<u32> {
        let mut c = crate::difftest::fresh();
        let c = &mut *c;
        let mut r = base(c, seed, 0x32, 2);
        find_rider_world(c, &mut r);
        audio_globals(c, &mut r);
        c.bus.write32(SYS + sys::ROOT + 0x408, if r.next_u32() % 3 == 0 { 0 } else { MP });
        c.bus.write32(SYS + sys::ROOT + 0x418, r.next_u32() % 2);
        c.bus.write32(SYS + sys::ROOT + 0x58, r.next_u32() % 4);
        c.bus.write32(SYS + sys::MUSIC_FADE_STATE, 2);
        c.bus.write32(SYS + sys::MUSIC_FADE_FRAMES, start);
        crate::tick::set_for_test(c, hz);
        for _ in 0..updates {
            c.g[A0][0] = sx(SYS);
            c.g[RA][0] = sx(crate::difftest::RA);
            c.g[SP][0] = sx(crate::difftest::SP);
            ssx_audio_update_music_fade_in(c);
            if c.pc != crate::difftest::RA {
                return None;
            }
        }
        Some(c.bus.read32(SYS + sys::MUSIC_FADE_FRAMES))
    }

    /// The music fades in over the same wall-clock time at every rate: the
    /// count stops at the length (800/1000/1200 frames at 60, r/60 times as
    /// many ticks at r), and r/60 updates at r advance it as far (in time) as
    /// one at 60, so the volume `127 * count / length` is the same.
    #[test]
    fn update_music_fade_in_follows_the_rate() {
        // A fixture that takes the fade path (not busy, no unwind).
        let seed = (0..200u32)
            .find(|&s| fade_count(60, s, 5, 1) == Some(6) && fade_count(60, s, 100, 2) == Some(102))
            .expect("no fixture reaches the fade path");
        let length = fade_count(60, seed, 100_000, 1).unwrap();
        assert!([800, 1000, 1200].contains(&length), "length {length}");
        for hz in [120u32, 240] {
            let q = hz / 60;
            assert_eq!(fade_count(hz, seed, 100_000, 1), Some(length * q), "{hz} Hz: length");
            assert_eq!(fade_count(hz, seed, length * q - 1, 2), Some(length * q), "{hz} Hz: stops");
            for t in [0u32, 100, length / 2] {
                let at60 = fade_count(60, seed, t, 1).unwrap();
                assert_eq!(fade_count(hz, seed, t * q, q), Some(at60 * q), "{hz} Hz: from {t}");
            }
        }
    }

    // --- countdown ---------------------------------------------------------

    #[test]
    fn update_countdown_beeps() {
        check(ssx3_ee::s08::f_0029cca8, ssx_audio_update_countdown_beeps, 150, &|c, seed| {
            let mut r = base(c, seed, 0x33, 4);
            find_rider_world(c, &mut r);
            audio_globals(c, &mut r);
            c.bus.write32(SYS + sys::ROOT, ROOT);
            c.bus.write32(ROOT, SND);
            c.bus.write32(GAME + 0xc0, if r.next_u32() % 5 == 0 { 0 } else { CD });
            c.bus.write32(CD + 0x88, r.next_u32() % 2);
            c.bus.write32(CD + 0x78, r.next_u32() % 40);
            c.bus.write32(RACE_MAN + 0xc, r.f32(30.0));
            c.bus.write32(0x0053_08d0, pick(&mut r, &[0, 0x200, 0x400, 0x800]));
            c.bus.write32(SYS + sys::COUNTDOWN_LAST, pick(&mut r, &[0, 1, 4, 6, 8, 10, 12]));
            c.g[A0][0] = sx(SYS);
        });
    }
}
