//! `src/sound/ssxaudio.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::addu;
use crate::abi::r32;
use crate::abi::reg;
use crate::abi::ret;
use crate::abi::set_reg;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::r;
pub(crate) use crate::game::dirtysock::tagsunk::gp_3 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_4;
pub(crate) use crate::game::fe::feasyncfile::gp_15 as gp_6;
pub(crate) use crate::game::fe::feasyncfile::gpa;
pub(crate) use crate::game::fe::feasyncfile::lh;
pub(crate) use crate::game::fe::feasyncfile::low_2;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_3;
pub(crate) use crate::game::fe::feasyncfile::lw_6 as lw_4;
pub(crate) use crate::game::fe::feasyncfile::sw_4 as sw_2;
pub(crate) use crate::game::fe::festateloadhint::oc1009_base_minus;
pub(crate) use crate::game::fe::festateloadhint::oc1009_ea;
pub(crate) use crate::game::fe::festateloadhint::oc1009_gp_minus;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw_2;
pub(crate) use crate::game::fe::festatetrophyroom::Ops;
pub(crate) use crate::game::lib::snd::addiu;
pub(crate) use crate::game::lib::snd::at;
pub(crate) use crate::game::lib::snd::cvt_w;
pub(crate) use crate::game::lib::snd::fpr;
pub(crate) use crate::game::lib::snd::sll;
pub(crate) use crate::game::render::graphicsman::RegMem;
pub(crate) use crate::game::render::lightman::gp_2 as gp;
pub(crate) use crate::game::scripter::ssxscriptengine::gp as gp_5;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::sw_at;
pub(crate) use crate::game::sound::bankmonitor::add32;
pub(crate) use crate::game::sound::bankmonitor::gp as gp_3;
pub(crate) use crate::game::sound::bankmonitor::leave;
pub(crate) use crate::game::sound::bankmonitor::load_gp;
pub(crate) use crate::game::sound::bankmonitor::p;
pub(crate) use crate::game::wscript::wscriptman::gp as gp_4;
pub(crate) use crate::game::wscript::wscriptman::sw;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0028_4bb0,
        ssx_audio_system_create,
        "SSXAudioSystem_Create",
    ),
    (
        0x0028_4c28,
        ssx_audio_system_destroy,
        "SSXAudioSystem_Destroy",
    ),
    (
        0x0028_54a8,
        ssx_audio_system_set_output_mode,
        "SSXAudioSystem_SetOutputMode",
    ),
    (
        0x0028_5930,
        ssx_audio_system_compute_3d_pan,
        "SSXAudioSystem_Compute3DPan",
    ),
    (
        0x0028_5be0,
        ssx_audio_system_set_mode,
        "SSXAudioSystem_SetMode",
    ),
    (
        0x0028_5f48,
        audio_stop_all_sounds,
        "SSXAudioSystem_StopAllSounds",
    ),
    (
        0x0028_6200,
        ssx_audio_system_unload_front_end_audio,
        "SSXAudioSystem_UnloadFrontEndAudio",
    ),
    (
        0x0028_6c00,
        ssx_audio_system_unload_world_banks,
        "SSXAudioSystem_UnloadWorldBanks",
    ),
    (
        0x0028_6ca8,
        ssx_audio_system_load_memory_bank,
        "SSXAudioSystem_LoadMemoryBank",
    ),
    (
        0x0028_6e20,
        ssx_audio_system_enter_game_init,
        "SSXAudioSystem_EnterGameInit",
    ),
    (
        0x0028_7108,
        ssx_audio_system_on_reset_race,
        "SSXAudioSystem_OnResetRace",
    ),
    (
        0x0028_73d8,
        ssx_audio_system_level_to_volume,
        "SSXAudioSystem_LevelToVolume",
    ),
    (
        0x0028_7410,
        ssx_audio_system_set_music_volume,
        "SSXAudioSystem_SetMusicVolume",
    ),
    (
        0x0028_7488,
        ssx_audio_system_set_sfx_volume,
        "SSXAudioSystem_SetSfxVolume",
    ),
    (
        0x0028_7520,
        audio_set_speech_volume,
        "SSXAudioSystem_SetSpeechVolume",
    ),
    (
        0x0028_7558,
        ssx_audio_system_set_music_channel_enabled,
        "SSXAudioSystem_SetMusicChannelEnabled",
    ),
    (
        0x0028_75d0,
        ssx_audio_system_set_sfx_sub_channels_enabled,
        "SSXAudioSystem_SetSfxSubChannelsEnabled",
    ),
    (
        0x0028_7670,
        ssx_audio_system_get_music_volume_level,
        "SSXAudioSystem_GetMusicVolumeLevel",
    ),
    (
        0x0028_76a0,
        ssx_audio_system_get_sfx_volume_level,
        "SSXAudioSystem_GetSfxVolumeLevel",
    ),
    (
        0x0028_76d0,
        ssx_audio_system_get_speech_volume_level,
        "SSXAudioSystem_GetSpeechVolumeLevel",
    ),
    (
        0x0028_7930,
        ssx_audio_system_get_channel_gain,
        "SSXAudioSystem_GetChannelGain",
    ),
    (
        0x0028_7a10,
        ssx_audio_system_set_fade_gain,
        "SSXAudioSystem_SetFadeGain",
    ),
    (
        0x0028_8370,
        ssx_audio_system_get_preset_gain,
        "SSXAudioSystem_GetPresetGain",
    ),
    (
        0x0028_87a8,
        ssx_audio_system_set_gain_target,
        "SSXAudioSystem_SetGainTarget",
    ),
    (
        0x0028_8ce8,
        ssx_audio_system_get_listener_count,
        "SSXAudioSystem_GetListenerCount",
    ),
    (
        0x0028_9650,
        ssx_audio_system_crowd_state_callback,
        "SSXAudioSystem_CrowdStateCallback",
    ),
    (
        0x0028_9830,
        ssx_audio_system_set_duck_gain,
        "SSXAudioSystem_SetDuckGain",
    ),
    (
        0x0028_99e8,
        ssx_audio_system_is_sound_mode2,
        "SSXAudioSystem_IsSoundMode2",
    ),
    (
        0x0028_9b18,
        ssx_audio_system_on_anim_sound_event,
        "SSXAudioSystem_OnAnimSoundEvent",
    ),
    (
        0x0028_9b70,
        ssx_audio_system_pause_game_audio,
        "SSXAudioSystem_PauseGameAudio",
    ),
    (
        0x0028_9d08,
        ssx_audio_system_option_to_output_mode,
        "SSXAudioSystem_OptionToOutputMode",
    ),
    (
        0x0028_9d60,
        ssx_audio_system_output_mode_to_option,
        "SSXAudioSystem_OutputModeToOption",
    ),
    (0x0028_a058, sndloadqueue_ctor, "SndLoadQueue_ctor"),
    (0x0028_a148, snd_load_queue_dtor, "SndLoadQueue_dtor"),
    (0x002a_77f8, ini_parser_ctor, "IniParser_ctor"),
    (0x002a_7ff8, ini_parser_get_string, "IniParser_GetString"),
    (
        0x0028_83b0,
        ssx_audio_system_set_mix_preset,
        "SSXAudioSystem_SetMixPreset",
    ),
    (
        0x0028_7c48,
        ssx_audio_system_update_fade_2,
        "SSXAudioSystem_UpdateFade",
    ),
    (
        0x0028_9688,
        ssx_audio_system_update_speech_ducking,
        "SSXAudioSystem_UpdateSpeechDucking",
    ),
    (
        0x0028_87f8,
        ssx_audio_system_update_mix_ramp,
        "SSXAudioSystem_UpdateMixRamp",
    ),
    (
        0x0028_9c18,
        ssx_audio_system_get_rider_rating_tier,
        "SSXAudioSystem_GetRiderRatingTier",
    ),
    (
        0x0028_8a20,
        ssx_audio_system_get_rider_listener_index,
        "SSXAudioSystem_GetRiderListenerIndex",
    ),
    (
        0x0028_8b40,
        ssx_audio_find_view_in_range,
        "SSXAudioSystem_FindViewInRange",
    ),
    (
        0x0028_8ac0,
        ssx_audio_get_view_pos,
        "SSXAudioSystem_GetViewPos",
    ),
    (
        0x0028_8c08,
        ssx_audio_system_is_any_view_in_range,
        "SSXAudioSystem_IsAnyViewInRange",
    ),
    (
        0x0028_5f80,
        ssx_audio_system_stop_by_callback,
        "SSXAudioSystem_StopByCallback",
    ),
    (
        0x0028_54f8,
        ssx_audio_system_play_stream,
        "SSXAudioSystem_PlayStream",
    ),
    (
        0x0028_5fb0,
        ssx_audio_system_load_front_end_audio,
        "SSXAudioSystem_LoadFrontEndAudio",
    ),
    (
        0x0028_67e8,
        ssx_audio_system_enter_game,
        "SSXAudioSystem_EnterGame",
    ),
    (
        0x0028_71b0,
        ssx_audio_system_reset_race_sounds,
        "SSXAudioSystem_ResetRaceSounds",
    ),
    (
        0x0028_8d18,
        ssx_audio_system_setup_channel_gains,
        "SSXAudioSystem_SetupChannelGains",
    ),
    (
        0x0028_9520,
        ssx_audio_system_load_playlist,
        "SSXAudioSystem_LoadPlaylist",
    ),
    (
        0x0028_9af0,
        ssx_audio_system_rider_init_stub,
        "SSXAudioSystem_RiderInitStub",
    ),
    (0x0028_4c68, ssx_audio_system_ctor, "SSXAudioSystem_ctor"),
    (
        0x0028_62a8,
        ssx_audio_system_load_world_audio,
        "SSXAudioSystem_LoadWorldAudio",
    ),
    (
        0x0028_7700,
        ssx_audio_system_set_channel_volume,
        "SSXAudioSystem_SetChannelVolume",
    ),
    (
        0x0028_7fc8,
        ssx_audio_system_load_volume_config,
        "SSXAudioSystem_LoadVolumeConfig",
    ),
    (
        0x0028_8f60,
        ssx_audio_system_set_debug_mode,
        "SSXAudioSystem_SetDebugMode",
    ),
    (
        0x0028_9470,
        ssx_audio_system_read_playlist_names,
        "SSXAudioSystem_ReadPlaylistNames",
    ),
    (0x0028_5210, ssx_audio_system_dtor, "SSXAudioSystem_dtor"),
    (
        0x0028_61d0,
        ssx_audio_system_set_music_event1,
        "SSXAudioSystem_SetMusicEvent1",
    ),
    (
        0x0028_6a80,
        ssx_audio_system_exit_game,
        "SSXAudioSystem_ExitGame",
    ),
    (
        0x0028_6d18,
        ssx_audio_system_unload_memory_bank,
        "SSXAudioSystem_UnloadMemoryBank",
    ),
    (
        0x0028_70a0,
        ssx_audio_system_on_quick_restart,
        "SSXAudioSystem_OnQuickRestart",
    ),
    (
        0x0028_72a8,
        ssx_audio_system_start_pre_race_music,
        "SSXAudioSystem_StartPreRaceMusic",
    ),
    (
        0x0028_79b0,
        ssx_audio_system_inc_channel_volume,
        "SSXAudioSystem_IncChannelVolume",
    ),
    (
        0x0028_79e0,
        ssx_audio_system_dec_channel_volume,
        "SSXAudioSystem_DecChannelVolume",
    ),
    (
        0x0028_9398,
        ssx_audio_system_prev_debug_mode,
        "SSXAudioSystem_PrevDebugMode",
    ),
    (
        0x0028_93c8,
        ssx_audio_system_next_debug_mode,
        "SSXAudioSystem_NextDebugMode",
    ),
    (
        0x0028_9400,
        ssx_audio_system_schedule_trigger_timer,
        "SSXAudioSystem_ScheduleTriggerTimer",
    ),
    (
        0x0028_9448,
        ssx_audio_system_trigger_timer_callback,
        "SSXAudioSystem_TriggerTimerCallback",
    ),
    (
        0x0028_9680,
        ssx_audio_system_empty_crowd_handler,
        "SSXAudioSystem_EmptyCrowdHandler",
    ),
    (
        0x0028_99f8,
        ssx_audio_system_apply_music_trigger_paint,
        "SSXAudioSystem_ApplyMusicTriggerPaint",
    ),
    (
        0x0028_9af8,
        ssx_audio_system_rider_release,
        "SSXAudioSystem_RiderRelease",
    ),
    (
        0x0028_9dc0,
        ssx_audio_system_get_output_mode_option,
        "SSXAudioSystem_GetOutputModeOption",
    ),
    (
        0x0028_9de0,
        ssx_audio_system_take_boot_movie_flag,
        "SSXAudioSystem_TakeBootMovieFlag",
    ),
    (
        0x0028_9df0,
        snd_load_queue_finish_current,
        "SndLoadQueue_FinishCurrent",
    ),
];

/// gp-relative `.lit4` constants.
pub(crate) mod pan_k {

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

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::be::beintmission::callee::AUDIO_LEVEL_TO_VOLUME;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_SET_CHANNEL_VOLUME;

    pub(crate) use crate::game::be::beintmission::callee::BANK_MONITOR_STOP_ALL;

    pub(crate) use crate::game::be::beintmission::callee::CROWD_AUDIO_STOP_ALL;

    pub(crate) use crate::game::be::beintnetwork::callee::SSX_AUDIO_SYSTEM_GET_APP;
    pub(crate) use crate::game::be::beintnetwork::callee::SSX_AUDIO_SYSTEM_GET_VIEW_COUNT;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::AUDIO_SET_OUTPUT;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::SSX_AUDIO_EMPTY_CROWD_HANDLER;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::SSX_AUDIO_GET_CHANNEL_VOLUME;

    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_LEVEL_TO_VOLUME_2;

    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_CHANNEL_VOLUME_2;

    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_EXPECT;

    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_READ_QUOTED_STRING;

    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_UNLOAD_BANK;

    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_FLUSH;

    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_SET_MODE;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::AUDIO_SYSTEM_CTOR;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GET_CHANNEL_VOLUME;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LEVEL_TO_VOLUME;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SET_CHANNEL_VOLUME;

    pub(crate) use crate::game::world::wscriptcache::callee::SSX_AUDIO_SYSTEM_PAUSE;

    pub(crate) use crate::game::wscript::wscriptman::callee::SET_FADE_GAIN;
}

/// The audio app / view objects: `world + 0x84` is the audio system, its
/// `+0x84` the app, and the app's `+0x10` the view count.
pub(crate) mod audio {
    pub(crate) use crate::game::sound::asyncsys::audio::APP;
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_UpdateFade (0x287c48).

/// The fade state of an `sSaMix7C48` (offsets from the object).
pub(crate) mod fade {

    /// Non-zero while a fade is running.
    pub const FADING: u32 = 0x63c0;

    /// Bits: which channels the fade drives (bit 2 = `chan[4]`/the mix).
    pub const MASK: u32 = 0x63c4;

    /// Per-frame step applied to the current value.
    pub const RATE: u32 = 0x63c8;

    /// Where the ramp stops.
    pub const TARGET: u32 = 0x63cc;

    /// The current fade value.
    pub const CUR: u32 = 0x63d0;

    /// `chan[4].out`, scaled by the fade for the mix levels.
    pub const CHAN4: u32 = 0x6300;

    /// Level written with the fade value before the mix levels.
    pub const MIX_LEVEL: u32 = 0x63f4;

    /// The ten output mix levels (`mix[0..10]`).
    pub const MIX: u32 = 0x636c;
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod addr {
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BANK_MONITOR_ADD;

    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MEM_MAN_ALLOC;
}

/// `GetHashValue32` (0x317670) and the UI object lookups, which the generator
/// emitted as cross-shard `call_indirect`s.
pub(crate) mod table_callee {

    pub(crate) use crate::game::fe::festaterewards::table_callee::OPERATOR_NEW;

    pub(crate) use crate::game::fe::festaterewards::table_callee::STRCPY;
    pub(crate) use crate::game::fe::festaterewards::table_callee::STRLEN;
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod table {

    pub(crate) use crate::game::fe::feasyncfile::table::BUILTIN_DELETE;

    pub(crate) use crate::game::fe::feasyncfile::table::EA_MEM_FREE;
}

// ---------------------------------------------------------------------------
// SSX audio system.

/// `float SSXAudioSystem_LevelToVolume(SSXAudioSystem *self, int level)`
/// (0x2873d8): scales the integer level by the constant at `gp-0x462c`, then
/// clamps the result to `[0, 1]`.
pub fn ssx_audio_system_level_to_volume(c: &mut Cpu) {
    c.cycles += 11;
    let level = FpuF32::from_i32(c.g[A1][0] as u32 as i32);
    let scale = Addr((c.g[GP][0] as u32).wrapping_sub(0x462c)).read_fpu(c);
    let scaled = level * scale;
    c.f[2] = scaled.0;
    c.g[AT][0] = 0x3f80_0000;
    c.f[3] = FpuF32::ONE.0;
    let zero = FpuF32::ZERO;
    c.f[1] = zero.0;
    abi::set_fcc(c, zero.le(scaled));
    if zero.le(scaled) {
        c.cycles += 1;
        c.f[1] = scaled.min(FpuF32::ONE).0;
    }
    c.cycles += 2;
    c.f[0] = c.f[1];
    abi::ret(c);
}

/// `float SSXAudioSystem_GetChannelGain(SSXAudioSystem *self, int channel,
/// int sub)` (0x287930): channel 4 returns a per-sub gain from +0x636c
/// (`sub*4`), everything else a per-channel gain from +0x62c8 (`channel*16`).
pub fn ssx_audio_system_get_channel_gain(c: &mut Cpu) {
    const GAIN: u32 = 0x62c8;
    const SUB_GAIN: u32 = 0x636c;
    c.cycles += 3;
    let base = abi::arg(c, 0);
    if c.g[A1][0] != 4 {
        c.g[V0][0] = sx(abi::arg(c, 1) << 4);
        c.cycles += 3;
        let addr = base.wrapping_add(c.g[V0][0] as u32);
        c.g[V0][0] = sx(addr);
        c.f[0] = Addr(addr).off(GAIN).read_fpu(c).0;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    if (c.g[A2][0] as i64) < 10 {
        c.cycles += 3;
        let off = sx(abi::arg(c, 2) << 2);
        let addr = base.wrapping_add(off as u32);
        c.g[V0][0] = sx(addr);
        c.f[0] = Addr(addr).off(SUB_GAIN).read_fpu(c).0;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    c.g[V0][0] = sx(abi::arg(c, 1) << 4);
    c.cycles += 3;
    let addr = base.wrapping_add(c.g[V0][0] as u32);
    c.g[V0][0] = sx(addr);
    c.f[0] = Addr(addr).off(GAIN).read_fpu(c).0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_Compute3DPan.

pub(crate) const GET_APP: Callee = Callee {
    addr: 0x0028_b1c0,
    f: ee::s07::SSXAudioSystem_GetApp,
};

pub(crate) const GET_GAME_MODULE: Callee = Callee {
    addr: 0x0028_b1c8,
    f: ee::s07::SSXAudioSystem_GetGameModule,
};

/// `float Math_Atan(float)` (function table, cross-shard).
pub(crate) const MATH_ATAN: u32 = 0x0031_c228;

/// The listener offset: +25 up, -100 along z.
pub(crate) const PAN_UP: FpuF32 = FpuF32(0x41c8_0000);

pub(crate) const PAN_BACK: FpuF32 = FpuF32(0xc2c8_0000);

/// A gp-relative float.
pub(crate) fn gp_const(c: &mut Cpu, off: i32) -> FpuF32 {
    let gp = reg(c, GP);
    at(gp, off as u32).read_fpu(c)
}

/// `atan(y / x)` folded into a full angle the way the game does it: if
/// `x < 0`, add pi when `y > 0`, else subtract it; `x == 0` gives 0 (y ==
/// 0), `+pi/2` (y >= 0) or `-pi/2` (constants `zero_pos`/`zero_neg` and
/// `pi_add`/`pi_sub`). The FPU registers and FCC end as the original's
/// (`dst` is the register the angle is built in: f3 or f12).
#[allow(clippy::too_many_arguments)]
pub(crate) fn pan_angle(
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
pub(crate) fn binary_angle(c: &mut Cpu, angle: FpuF32, k: i32) -> u64 {
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
                let k = [
                    pan_k::AZ_LEFT,
                    pan_k::AZ_RIGHT,
                    pan_k::AZ_PI_ADD,
                    pan_k::AZ_PI_SUB,
                ];
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
            let k = [
                pan_k::EL_DOWN,
                pan_k::EL_UP,
                pan_k::EL_PI_ADD,
                pan_k::EL_PI_SUB,
            ];
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
pub(crate) fn pan_exit(c: &mut Cpu, frame: Frame) -> Flow {
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

/// `int SSXAudioSystem_GetListenerCount()` (0x288ce8): the app's view count,
/// or 1 when the world has no audio app.
pub fn ssx_audio_system_get_listener_count(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::SSX_AUDIO_SYSTEM_GET_APP.call(c, 0x0028_8cf8)?;
        let world = c.g[V0][0] as u32;
        let system = Addr(world).off(audio::APP).lw(c);
        c.g[V1][0] = system;
        c.g[V0][0] = 1;
        if system != 0 {
            c.cycles += 2;
            callee::SSX_AUDIO_SYSTEM_GET_VIEW_COUNT.call(c, 0x0028_8d0c)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// One faded channel pair: `fadeCur` is stored at `out`, `gain * fadeCur` at
/// `src + 4` and `gain * fadeCur * tail` at `src + 8`; the mask is left in v0
/// for the next test (as the original does, block by block).
///
/// `out` is the level the channel's fade value is written to; `src` the pair
/// of gains (also where the two scaled values go); `tail` the second factor.
pub(crate) fn scale_channel(c: &mut Cpu, base: u32, out: u32, src: u32, tail: u32) {
    c.cycles += 9;
    c.lwc1(0, base.wrapping_add(fade::CUR));
    c.lwc1(1, base.wrapping_add(src));
    c.lwc1(2, base.wrapping_add(tail));
    c.mul_s(1, 1, 0);
    c.swc1(0, base.wrapping_add(out));
    c.mul_s(2, 1, 2);
    c.swc1(1, base.wrapping_add(src).wrapping_add(4));
    c.swc1(2, base.wrapping_add(src).wrapping_add(8));
    c.g[V0][0] = c.lw(base.wrapping_add(fade::MASK));
}

/// Test mask bit `bit` (+`cost` cycles) and, when set, scale that channel.
/// Every path leaves the mask in v0 (the original reloads it either way).
pub(crate) fn mask_bit(c: &mut Cpu, base: u32, bit: u64, cost: u32, out: u32, src: u32, tail: u32) {
    c.cycles += cost as u64;
    let m = c.lw(base.wrapping_add(fade::MASK));
    if m & bit != 0 {
        scale_channel(c, base, out, src, tail);
    } else {
        c.g[V0][0] = m;
    }
}

/// `void func_00287C48(sSaMix *s)` (0x287c48): advance the audio fade ramp.
///
/// With `fading` set, `cur += rate` is stored and, once the ramp passes
/// `target` (below it for a negative rate, above it for a positive one), the
/// value is clamped to the target and `fading` cleared. Every channel bit of
/// the mask then scales the current value into its gain pair, and bit 2 also
/// rebuilds the ten output mix levels from `chan[2].out` (+0x6300) times the
/// constants at `gp+0x43c..0x460` (`D_004A352C..0x3550`).
pub fn ssx_audio_system_update_fade(c: &mut Cpu) {
    c.cycles += 3;
    let base = abi::arg(c, 0);
    let fading = c.lw(base.wrapping_add(fade::FADING));
    c.g[V0][0] = fading;
    if fading != 0 {
        // `cur += rate`
        c.cycles += 8;
        c.lwc1(0, base.wrapping_add(fade::CUR));
        c.lwc1(1, base.wrapping_add(fade::RATE));
        c.mtc1(0, 2); // f2 = 0.0
        c.add_s(0, 0, 1);
        c.c_lt_s(1, 2); // rate < 0
        let falling = c.fcc();
        c.swc1(0, base.wrapping_add(fade::CUR));
        if falling {
            // a falling ramp stops when cur < target
            c.cycles += 4;
            c.lwc1(1, base.wrapping_add(fade::TARGET));
            c.c_lt_s(0, 1);
        } else {
            // a rising ramp stops when target < cur
            c.cycles += 3;
            c.lwc1(1, base.wrapping_add(fade::TARGET));
            c.c_lt_s(1, 0);
        }
        c.cycles += 2;
        if c.fcc() {
            c.cycles += 3;
            c.swc1(1, base.wrapping_add(fade::CUR));
            c.sw(base.wrapping_add(fade::FADING), 0);
        }
        // Channels 0 and 1 only test their bit (their `set` was inlined away).
        c.cycles += 3;
        c.g[V0][0] = c.lw(base.wrapping_add(fade::MASK));
        c.cycles += 3;
        c.g[V0][0] = c.lw(base.wrapping_add(fade::MASK));

        // Channel 4: the ten mix levels.
        c.cycles += 3;
        c.g[V0][0] &= 4;
        c.lwc1(1, gp(c, 0x43c));
        if c.g[V0][0] != 0 {
            c.cycles += 37;
            c.lwc1(12, base.wrapping_add(fade::CUR));
            c.lwc1(11, base.wrapping_add(fade::CHAN4));
            c.lwc1(0, base.wrapping_add(fade::MIX_LEVEL));
            c.mul_s(11, 11, 12);
            c.lwc1(2, gp(c, 0x440));
            c.lwc1(4, gp(c, 0x444));
            c.mul_s(0, 11, 0);
            c.lwc1(5, gp(c, 0x448));
            c.lwc1(7, gp(c, 0x44c));
            c.lwc1(9, gp(c, 0x450));
            c.lwc1(10, gp(c, 0x454));
            c.mul_s(1, 0, 1);
            c.lwc1(8, gp(c, 0x458));
            c.mul_s(2, 0, 2);
            c.lwc1(6, gp(c, 0x45c));
            c.mul_s(4, 0, 4);
            c.lwc1(3, gp(c, 0x460));
            c.mul_s(5, 0, 5);
            c.mul_s(7, 0, 7);
            c.swc1(1, base.wrapping_add(fade::MIX));
            c.mul_s(3, 0, 3);
            c.swc1(2, base.wrapping_add(fade::MIX).wrapping_add(4));
            c.mul_s(9, 0, 9);
            c.swc1(4, base.wrapping_add(fade::MIX).wrapping_add(8));
            c.mul_s(10, 0, 10);
            c.swc1(5, base.wrapping_add(fade::MIX).wrapping_add(0xc));
            c.mul_s(8, 0, 8);
            c.swc1(7, base.wrapping_add(fade::MIX).wrapping_add(0x10));
            c.mul_s(6, 0, 6);
            c.swc1(9, base.wrapping_add(fade::MIX).wrapping_add(0x14));
            c.swc1(10, base.wrapping_add(fade::MIX).wrapping_add(0x18));
            c.swc1(8, base.wrapping_add(fade::MIX).wrapping_add(0x1c));
            c.swc1(6, base.wrapping_add(fade::MIX).wrapping_add(0x20));
            c.swc1(3, base.wrapping_add(fade::MIX).wrapping_add(0x24));
            c.swc1(12, base.wrapping_add(0x63a4));
            c.swc1(11, base.wrapping_add(fade::CHAN4).wrapping_add(4));
            c.swc1(0, base.wrapping_add(fade::CHAN4).wrapping_add(8));
        }

        // Channel 2 (bit 8, one cycle more: it reloads the mask first).
        mask_bit(c, base, 8, 4, 0x6398, 0x62d0, 0x63e8);
        // Channel 3, 6, 7, 8, 9, 10.
        mask_bit(c, base, 0x80, 3, 0x63a8, 0x6310, 0x63f8);
        mask_bit(c, base, 0x20, 3, 0x63ac, 0x6320, 0x63fc);
        mask_bit(c, base, 0x40, 3, 0x63b0, 0x6330, 0x6400);
        mask_bit(c, base, 0x10, 3, 0x63b4, 0x6340, 0x6404);
        mask_bit(c, base, 0x100, 3, 0x63b8, 0x6350, 0x6408);

        // Channel 11 (bit 0x200): no mask reload, one cycle less.
        c.cycles += 3;
        c.g[V0][0] &= 0x200;
        if c.g[V0][0] != 0 {
            c.cycles += 8;
            c.lwc1(0, base.wrapping_add(fade::CUR));
            c.lwc1(1, base.wrapping_add(0x6360));
            c.lwc1(2, base.wrapping_add(0x640c));
            c.mul_s(1, 1, 0);
            c.swc1(0, base.wrapping_add(0x63bc));
            c.mul_s(2, 1, 2);
            c.swc1(1, base.wrapping_add(0x6364));
            c.swc1(2, base.wrapping_add(0x6368));
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `bool SSXAudioSystem_IsSoundMode2(SSXAudioSystem* self)` (0x2899e8): true
/// when the sound-mode word at `+0x608c` is 2.
pub fn ssx_audio_system_is_sound_mode2(c: &mut Cpu) {
    c.cycles += 4;
    let this = abi::arg(c, 0);
    c.g[V0][0] = c.lw(this.wrapping_add(0x608c));
    c.g[V0][0] = c.g[V0][0] ^ 0x2;
    c.g[V0][0] = (c.g[V0][0] < 1u64) as u64;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Audio system / music player / bank accessors.

/// `void SSXAudioSystem_SetMode(SSXAudioSystem* a, int mode)` (0x285be0): the
/// system mode at +0x6474, stored only when it changed.
pub fn ssx_audio_system_set_mode(c: &mut Cpu) {
    c.cycles += 3;
    let selfp = abi::arg(c, 0);
    let mode = c.g[A1][0];
    c.g[V0][0] = c.lw(selfp.wrapping_add(0x6474));
    if c.g[V0][0] != mode {
        c.sw(selfp.wrapping_add(0x6474), mode);
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Audio.

/// The shared body of the three volume-level getters: read the channel's
/// volume and scale the 0..1 value to the 0..11 menu level.
pub(crate) fn volume_level(c: &mut Cpu, channel: u32, ret: u32) -> Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[A1][0] = channel as u64;
    callee::SSX_AUDIO_GET_CHANNEL_VOLUME.call(c, ret)?;
    // f1 = 11.0, then the usual `(int)(volume * 11.0f)`.
    c.g[AT][0] = 0x4130_0000; // 11.0
    c.mtc1(AT, 1);
    frame.restore_ra(c, 0);
    let level = FpuF32(c.f[0]) * FpuF32(c.f[1]);
    c.f[0] = level.0;
    c.f[1] = level.to_i32() as u32;
    c.mfc1(V0, 1);
    frame.close(c);
    Ok(())
}

/// `int SSXAudioSystem_GetMusicVolumeLevel(SSXAudioSystem *self)` (0x287670).
pub fn ssx_audio_system_get_music_volume_level(c: &mut Cpu) {
    guest(c, |c| volume_level(c, 1, 0x0028_7680))
}

/// `int SSXAudioSystem_GetSfxVolumeLevel(SSXAudioSystem *self)` (0x2876a0).
pub fn ssx_audio_system_get_sfx_volume_level(c: &mut Cpu) {
    guest(c, |c| volume_level(c, 5, 0x0028_76b0))
}

/// `int SSXAudioSystem_GetSpeechVolumeLevel(SSXAudioSystem *self)` (0x2876d0).
pub fn ssx_audio_system_get_speech_volume_level(c: &mut Cpu) {
    guest(c, |c| volume_level(c, 4, 0x0028_76e0))
}

/// `void SSXAudioSystem_CrowdStateCallback(SSXAudioSystem *self, int *state)`
/// (0x289650): when the incoming state is 3 (`idle`) reset the crowd handler.
pub fn ssx_audio_system_crowd_state_callback(c: &mut Cpu) {
    /// `gp + 0x410`: the crowd audio singleton.
    const CROWD: u32 = 0x410;

    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = 3;
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
        let idle = c.g[V0][0] == c.g[V1][0];
        frame.restore_ra(c, 0);
        if idle {
            c.cycles += 3;
            c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(CROWD)).lw(c);
            callee::SSX_AUDIO_EMPTY_CROWD_HANDLER.call(c, 0x0028_9674)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Audio system.

/// `void SSXAudioSystem_StopAllSounds(SSXAudioSystem* self)` (0x285f48):
/// stop the crowd and bank-monitor sources, then the sound-system sources.
pub fn audio_stop_all_sounds(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::CROWD_AUDIO_STOP_ALL.call(c, 0x0028_5f5c)?;
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x118)).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
        callee::BANK_MONITOR_STOP_ALL.call(c, 0x0028_5f6c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SSXAudioSystem_SetSpeechVolume(SSXAudioSystem* self, int level)`
/// (0x287520): convert the level to a volume and apply it to speech
/// channel 4.
pub fn audio_set_speech_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::AUDIO_LEVEL_TO_VOLUME.call(c, 0x0028_7534)?;
        c.g[A0][0] = c.g[S0][0];
        c.mov_s(12, 0);
        c.g[A1][0] = 4;
        callee::AUDIO_SET_CHANNEL_VOLUME.call(c, 0x0028_7544)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Sound.

/// `float SSXAudioSystem_GetPresetGain(SSXAudioSystem* self, int i, int pct)`
/// (0x288370): a non-negative percentage is scaled by 0.01, otherwise the
/// preset's stored gain (custom table at +0x6440 when +0x6410 is set, the
/// default table at +0x63E4 otherwise).
pub fn ssx_audio_system_get_preset_gain(c: &mut Cpu) {
    c.cycles += 2;
    let this = r(c, A0);
    let index = c.g[A1][0];
    if (c.g[A2][0] as i64) < 0 {
        let custom = c.lw_r_2(V0, this.wrapping_add(0x6410));
        c.cycles += 2;
        c.g[V0][0] = sx((index as u32) << 2);
        let slot = sx(this.wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = slot;
        if custom != 0 {
            c.cycles += 3;
            c.ld_f(0, (slot as u32).wrapping_add(0x6440));
        } else {
            c.cycles += 3;
            c.ld_f(0, (slot as u32).wrapping_add(0x63e4));
        }
    } else {
        c.cycles += 5;
        c.mtc1(A2, 1);
        c.cvt_s_w(1, 1);
        c.ld_f(0, gp_2(c, -0x4618));
        c.mul_s(0, 1, 0);
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Audio.

/// `void SSXAudioSystem_Destroy()` (0x284c28): tell the audio interface
/// object (gp_3+0x410) to shut down (virtual slot 1, argument 3), then clear
/// the singleton.
pub fn ssx_audio_system_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        load_gp(c, V0, 0x410);
        let empty = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        if !empty {
            c.cycles += 8;
            c.g[V1][0] = Addr(r32(c, V0)).off(0x118).lw(c);
            c.g[A1][0] = 3;
            c.g[A2][0] = Addr(r32(c, V1)).lw(c);
            c.g[V0][0] = Addr(r32(c, A2)).off(0x1d4).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).off(8).lh(c);
            c.g[V1][0] = Addr(r32(c, V0)).off(0xc).lw(c);
            let func = r32(c, V1);
            c.g[RA][0] = sx(0x0028_4c58);
            c.g[A0][0] = add32(c.g[A2][0], c.g[A0][0]);
            c.pc = func;
            call_indirect(c, func);
            if c.pc != 0x0028_4c58 {
                return Err(crate::call::Unwound);
            }
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        c.sw(gp_3(c, 0x410), 0);
        leave(c, 0x10);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSX audio system: pause latches.

/// `void SSXAudioSystem_PauseGameAudio(SSXAudioSystem* s)` (0x289b70): once
/// per session (`s+0x5fb4`), pause the bank monitor and the music stream.
pub fn ssx_audio_system_pause_game_audio(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x5fb4));
        if c.g[V0][0] == 0 {
            c.cycles += 7;
            callee::SSX_AUDIO_SYSTEM_PAUSE.call(c, 0x0028_9b94)?;
            let stream = r32(c, S0).wrapping_add(0x118);
            c.g[A0][0] = sx(stream);
            call_ptr(c, 0x002b_3a70, 0x0028_9b9c)?;
            c.g[V0][0] = 1;
            c.sw(r32(c, S0).wrapping_add(0x5fb4), c.g[V0][0]);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SSXAudioSystem_SetOutputMode(SSXAudioSystem* sys, int mode)`
/// (0x2854a8): apply `mode` through the audio object at `sys+0x118`'s slot
/// 0x1d8 field, and when that reports 2 remember the mode at +0x62b0.
/// 14 + 6 cycles plus the callee.
pub fn ssx_audio_system_set_output_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        let mode = c.g[A1][0];
        c.g[S0][0] = mode;
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        c.g[S1][0] = c.g[A0][0];
        let obj = c.lw(p(self_, 0x118));
        c.g[V0][0] = obj;
        let vt = c.lw(obj as u32);
        c.g[V1][0] = vt;
        c.g[A0][0] = c.lw(p(vt as u32, 0x1d8));
        call_ptr(c, callee::AUDIO_SET_OUTPUT, 0x0028_54d0)?;
        let result = c.g[V0][0];
        c.g[V1][0] = result;
        c.g[V0][0] = 2;
        if c.g[V1][0] == c.g[V0][0] {
            c.sw(p(self_, 0x62b0), c.g[S0][0]);
        }
        c.cycles += 6;
        frame.restore(c, 16, 0x20);
        c.g[V0][0] = c.g[V1][0];
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Audio.

/// `void SSXAudioSystem_SetGainTarget(sAudioFade* self, int i, int smooth,
/// float target, float time)` (0x2887a8): store the gain target and, when it
/// differs from the current gain, derive the ramp rate from the time (a
/// `smooth` of 0 leaves the rate alone). Equal target zeroes the rate.
pub fn ssx_audio_system_set_gain_target(c: &mut Cpu) {
    c.cycles += 10;
    let this = r32(c, A0);
    c.g[A3][0] = c.g[A0][0];
    // self + i*4 (the per-channel slots).
    let ch = this.wrapping_add(r32(c, A1) << 2);
    c.g[V0][0] = sx(ch);
    c.swc1(12, p(ch, 0x6440)); // target[i]
    c.g[V1][0] = sx(ch);
    c.lwc1(0, p(ch, 0x63e4)); // cur[i]
    c.c_eq_s(12, 0);
    if c.fcc() {
        Addr(p(ch, 0x6414)).sw(c, 0); // rate[i] = 0
    } else {
        c.cycles += 2;
        if c.g[A2][0] != 0 {
            c.cycles += 6;
            c.sub_s(0, 12, 0); // target - cur
            c.lwc1(1, gp_4(c, -0x4614)); // 1/60
            c.div_s(0, 0, 13); // / time
            c.mul_s(0, 0, 1); // * (1/60)
            c.swc1(0, p(ch, 0x6414));
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void SSXAudioSystem_SetDuckGain(void* self, float v)` (0x289830): set the
/// duck gain at `+0x6c5c` through the fade system when it changed.
pub fn ssx_audio_system_set_duck_gain(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_fpr(c, 20, 0x20);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.mov_s(20, 12);
        c.lwc1(0, p(this, 0x6c5c));
        c.c_eq_s(0, 20);
        if !c.fcc() {
            c.cycles += 5;
            c.mtc1(0, 13); // fade time 0.0
            c.g[A1][0] = 0x1;
            callee::SET_FADE_GAIN.call(c, 0x0028_9868)?;
            c.swc1(20, p(this, 0x6c5c));
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Audio.

/// `int SSXAudioSystem_OptionToOutputMode(option)` (0x289d08): map the audio
/// option byte to its output mode.
pub fn ssx_audio_system_option_to_output_mode(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A1][0] = c.g[A1][0] & 0xff;
    c.g[V1][0] = 1;
    let is1 = c.g[A1][0] == c.g[V1][0];
    c.g[V0][0] = ((c.g[A1][0] as i64) < 2i64) as u64;
    let mut to_289d50 = true;
    if is1 {
        c.cycles += 2;
        c.g[V1][0] = 2;
    } else {
        c.cycles += 2;
        let small = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[V1][0];
        if small {
            to_289d50 = false; // break b_289d54 with v0 = 1
        } else {
            c.cycles += 3;
            c.g[V0][0] = 2;
            let is2 = c.g[A1][0] == c.g[V0][0];
            c.g[V0][0] = 3;
            if is2 {
                c.cycles += 2;
                c.g[V1][0] = 0;
            } else {
                c.cycles += 2;
                let is3 = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = c.g[V1][0];
                if is3 {
                    c.cycles += 1;
                    c.g[V1][0] = 3;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    to_289d50 = false;
                }
            }
        }
    }
    if to_289d50 {
        c.cycles += 1;
        c.g[V0][0] = c.g[V1][0];
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Audio.

/// `void SSXAudioSystem_OnAnimSoundEvent(void* a0, void* a1, int type)`
/// (0x289b18): dispatch an animation sound event (0x50/0x51/0x52).
pub fn ssx_audio_system_on_anim_sound_event(c: &mut Cpu) {
    c.cycles += 4;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = 0x50;
    let not_50 = c.g[A2][0] != c.g[V0][0];
    frame.save_ra(c, 0);
    if not_50 {
        c.cycles += 3;
        c.g[V0][0] = 0x51;
        let not_51 = c.g[A2][0] != c.g[V0][0];
        c.g[V0][0] = 0x52;
        if not_51 {
            c.cycles += 2;
            let not_52 = c.g[A2][0] != c.g[V0][0];
            frame.restore_ra(c, 0);
            if !not_52 {
                c.cycles += 3;
                if call_ptr(c, 0x0029_c088, 0x0028_9b64).is_err() {
                    return;
                }
                frame.restore_ra(c, 0);
            }
        } else {
            c.cycles += 4;
            if call_ptr(c, 0x0029_b968, 0x0028_9b4c).is_err() {
                return;
            }
            frame.restore_ra(c, 0);
        }
    } else {
        c.cycles += 4;
        if call_ptr(c, 0x0029_bcf8, 0x0028_9b30).is_err() {
            return;
        }
        frame.restore_ra(c, 0);
    }
    c.cycles += 2;
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `int SSXAudioSystem_OutputModeToOption(int mode)` (0x289d60): map an
/// output-mode enum to its menu option index (1→0, 2→1, 0→2, 3→3).
pub fn ssx_audio_system_output_mode_to_option(c: &mut Cpu) {
    let mode = c.g[A1][0];
    let (option, extra) = if mode == 1 {
        (0u64, 1u64)
    } else if (mode as i64) < 2 {
        if mode == 0 {
            (2, 8)
        } else {
            (0, 7)
        }
    } else if mode == 2 {
        (1, 8)
    } else if mode == 3 {
        (3, 9)
    } else {
        (0, 9)
    };
    c.cycles += 3 + extra + 2;
    c.g[V1][0] = option;
    c.g[V0][0] = c.g[V1][0];
    abi::ret(c);
}

/// `SSXAudioSystem::LoadMemoryBank(self, id, data)` (0x286ca8): load one
/// memory-resident sound bank and mark it in the loaded/sub tables.
pub fn ssx_audio_system_load_memory_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let absent = c.g[A2][0] == 0;
        c.g[S1][0] = sx((c.g[S2][0] as u32) >> 8);
        if !absent {
            c.cycles += 12;
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x118)).lw(c);
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(8));
            c.g[A3][0] = 0x20000;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
            call_ptr(c, 0x0028_be60, 0x0028_6ce0)?;
            c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
            c.g[A1][0] = 1;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = c.g[S2][0] & 0xff;
            c.g[V1][0] = c.g[A0][0];
            Addr(p(c.g[A0][0] as u32, 0x6c70)).sw(c, c.g[A1][0]);
            Addr(p(c.g[V1][0] as u32, 0x6c78)).sw(c, c.g[V0][0]);
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

/// `SSXAudioSystem_Create` (0x284BB0): lazily allocate and construct the sound
/// system singleton, then tick its bank monitor. Returns the singleton.
pub fn ssx_audio_system_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = Addr(gp_5(c, 0x410)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let exists = c.g[V0][0] != 0;
        c.g[S0][0] = c.g[A0][0];
        if !exists {
            c.cycles += 14;
            c.g[A1][0] = 0x48_0000;
            c.g[A2][0] = 0xffff_ffff_8000_0000;
            c.g[A1][0] = low(c, A1).wrapping_add(0x28b0);
            c.g[A0][0] = 0x7780;
            c.g[A2][0] |= 0x400;
            c.g[A3][0] = 0;
            call_ptr(c, addr::MEM_MAN_ALLOC, 0x0028_4be4)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[T0][0] = c.g[S0][0];
            c.g[A1][0] = 1;
            c.g[A2][0] = 0x1c;
            c.g[A3][0] = 3;
            callee::AUDIO_SYSTEM_CTOR.call(c, 0x0028_4bfc)?;
            sw_at(c, GP, 0x410, c.g[V0][0]);
        }
        c.cycles += 9;
        c.g[V0][0] = Addr(gp_5(c, 0x410)).lw(c);
        c.g[V1][0] = lw_at(c, V0, 0x118);
        c.g[A0][0] = Addr(r32(c, V1)).lw(c);
        c.g[A0][0] = low(c, A0).wrapping_add(0x1d8);
        call_ptr(c, addr::BANK_MONITOR_ADD, 0x0028_4c14)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem_SetMusicVolume` (0x287410): set the music volume on the
/// music channels (1, 8 and, when the music is not on the alternative channel,
/// 2).
pub fn ssx_audio_system_set_music_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_fpr(c, 20, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::LEVEL_TO_VOLUME.call(c, 0x0028_7428)?;
        c.mov_s(20, 0);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        c.mov_s(12, 20);
        callee::SET_CHANNEL_VOLUME.call(c, 0x0028_743c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 8;
        c.mov_s(12, 20);
        callee::SET_CHANNEL_VOLUME.call(c, 0x0028_744c)?;
        c.g[V0][0] = lw_at(c, S0, 0x62b8);
        let no_music = c.g[V0][0] == 0;
        c.g[V0][0] = 2;
        if !no_music {
            c.cycles += 3;
            c.g[V1][0] = lw_at(c, S0, 0x608c);
            let alts = c.g[V1][0] == c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            if !alts {
                c.cycles += 3;
                c.mov_s(12, 20);
                c.g[A1][0] = 2;
                callee::SET_CHANNEL_VOLUME.call(c, 0x0028_7470)?;
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem_SetMusicChannelEnabled` (0x287558): turn the second music
/// channel on (remembering the first channel's volume) or mute it.
pub fn ssx_audio_system_set_music_channel_enabled(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let on = c.g[A1][0] != 0;
        c.g[S0][0] = c.g[A0][0];
        let mut enable_path = false;
        if on {
            c.cycles += 4;
            c.g[V1][0] = lw_at(c, S0, 0x608c);
            c.g[V0][0] = 2;
            let is_alt = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 3;
            if !is_alt {
                c.cycles += 2;
                let is_alt2 = c.g[V1][0] == c.g[V0][0];
                if !is_alt2 {
                    c.cycles += 9;
                    c.g[A1][0] = 1;
                    callee::GET_CHANNEL_VOLUME.call(c, 0x0028_758c)?;
                    c.mov_s(12, 0);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 2;
                    callee::SET_CHANNEL_VOLUME.call(c, 0x0028_759c)?;
                    c.g[V0][0] = 1;
                    sw_at(c, S0, 0x62b8, c.g[V0][0]);
                    enable_path = true;
                }
            }
        }
        if !enable_path {
            c.cycles += 5;
            c.mtc1(0, 12);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 2;
            callee::SET_CHANNEL_VOLUME.call(c, 0x0028_75b8)?;
            sw_at(c, S0, 0x62b8, 0);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `func_002A7FF8` (0x2a7ff8): `IniParser_GetString`. Expects an `=`-style
/// token (`0x41aa88(this+0xc)`), then the string `"0x4a3740"` (`IniParser_Expect`),
/// then reads a quoted string and copies it to `a2` (`0x4166f4`); returns 1 on
/// success, 0 otherwise. Signature: `int get_string(IniParser* this, void*,
/// char* out)`.
pub fn ini_parser_get_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        c.g[RA][0] = sx(0x002a_8018);
        c.g[A0][0] = sx(low_2(c, S0).wrapping_add(0xc));
        call_ptr(c, 0x0041_aa88, 0x002a_8018)?;
        let failed = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !failed {
            c.cycles += 6;
            c.g[A1][0] = 0x4a_0000;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x002a_8030);
            c.g[A1][0] = sx(low_2(c, A1).wrapping_add(0x3740));
            callee::INI_PARSER_EXPECT.call(c, 0x002a_8030)?;
            let failed = c.g[V0][0] != 0;
            c.g[V0][0] = 0;
            if !failed {
                c.cycles += 4;
                c.g[RA][0] = sx(0x002a_8040);
                c.g[A0][0] = c.g[S0][0];
                callee::INI_PARSER_READ_QUOTED_STRING.call(c, 0x002a_8040)?;
                let empty = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S1][0];
                if empty {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                } else {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002a_8050);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0041_66f4, 0x002a_8050)?;
                    c.g[V0][0] = 0x1;
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_EnterGameInit(SSXAudioSystem* a)` (0x00286e20): enter
/// game mode (6), reset the world triggers, restart the board loops and store
/// the game module's id; in mode 1 say the "20BB" line.
pub fn ssx_audio_system_enter_game_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        c.g[A1][0] = 6;
        c.sq(S0, sp(c).wrapping_add(0x10));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0028_5be0, 0x0028_6e38)?; // SSXAudioSystem_SetMode
        c.g[A0][0] = c.lw(gpa(c, 0x21e4));
        call_ptr(c, 0x002b_6900, 0x0028_6e40)?; // WorldTriggerManager_ResetPlayedList
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0029_29d8, 0x0028_6e48)?; // SSXAudioSystem_RestartAllBoardLoops
        call_ptr(c, 0x0028_b1c8, 0x0028_6e50)?; // SSXAudioSystem_GetGameModule
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(8));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x582c), c.g[V0][0]);
        call_ptr(c, 0x0014_dc80, 0x0028_6e60)?; // BELibrary_GetInstance
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0028_6e6c)?; // BELibrary_GetInterface
        c.g[V1][0] = sx(0x0053_5c11);
        c.g[V0][0] = 1;
        c.g[V1][0] = c.lb(c.g[V1][0] as u32);
        let one = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 2;
        let run = if one {
            true
        } else {
            c.cycles += 2;
            c.g[V1][0] == c.g[V0][0]
        };
        if run {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002a_39e0, 0x0028_6e90)?; // SSXAudioSystem_PASay20BB
        }
        c.lq(S0, sp(c).wrapping_add(0x10));
        c.cycles += 3;
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        abi::ret(c);
        Ok(())
    })
}

/// `IniParser* IniParser_ctor(IniParser* self, char* text, u32 flags)`
/// (0x2a77f8): measure the text, copy it into a fresh allocation and point the
/// cursor at_2 the copy's start.
pub fn ini_parser_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        Addr(c.g[S0][0] as u32).off(0x4).sw(c, 0);
        c.g[A0][0] = c.g[S1][0];
        Addr(c.g[S0][0] as u32).off(0x8).sw(c, 0);
        call_ptr(c, table_callee::STRLEN, 0x002a_7828)?;
        let size = c.g[V0][0];
        Addr(c.g[S0][0] as u32).sw(c, size);
        if size != 0 {
            c.cycles += 16;
            c.g[A2][0] = c.g[S2][0];
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            c.g[A1][0] = c.g[S1][0];
            c.g[A3][0] = 0;
            call_ptr(c, table_callee::OPERATOR_NEW, 0x002a_7844)?;
            let text = c.g[V0][0];
            c.g[A2][0] = lw_2(c, S0, 0);
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = text;
            Addr(c.g[S0][0] as u32).off(0x4).sw(c, text);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            call_ptr(c, table_callee::STRCPY, 0x002a_785c)?;
            c.g[V1][0] = lw_2(c, S0, 0x4);
            c.g[V0][0] = lw_2(c, S0, 0);
            Addr(c.g[S0][0] as u32).off(0x8).sw(c, c.g[V1][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            Addr(c.g[V1][0] as u32).sb(c, 0);
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x20);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem_SetSfxVolume` (0x287488): convert the level to a volume
/// and apply it to the SFX channels (plus 9/10 when the extra bank exists).
pub fn ssx_audio_system_set_sfx_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_fpr(c, 20, 0x20);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0028_74a0);
        c.g[S0][0] = c.g[A0][0];
        callee::AUDIO_LEVEL_TO_VOLUME_2.call(c, 0x0028_74a0)?;
        c.mov_s(20, 0);
        for (channel, ret) in [
            (5u64, 0x0028_74b4u32),
            (3, 0x0028_74c4),
            (6, 0x0028_74d4),
            (7, 0x0028_74e4),
        ] {
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = channel;
            c.g[RA][0] = sx(ret);
            c.mov_s(12, 20);
            callee::AUDIO_SET_CHANNEL_VOLUME_2.call(c, ret)?;
        }
        c.g[V0][0] = Addr(p(r(c, S0), 0x62b4)).lw(c);
        let no_extra = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[S0][0];
        if !no_extra {
            c.cycles += 7;
            c.g[A1][0] = 9;
            c.g[RA][0] = sx(0x0028_74fc);
            c.mov_s(12, 20);
            callee::AUDIO_SET_CHANNEL_VOLUME_2.call(c, 0x0028_74fc)?;
            c.g[A0][0] = c.g[S0][0];
            c.mov_s(12, 20);
            c.g[RA][0] = sx(0x0028_750c);
            c.g[A1][0] = 0xa;
            callee::AUDIO_SET_CHANNEL_VOLUME_2.call(c, 0x0028_750c)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Audio system.

/// `void SSXAudioSystem_SetSfxSubChannelsEnabled(AudioSystem* self, int on)`
/// (0x2875d0): mute sound-effect channels 9 and 10 (volume 0) when `on` is
/// zero; otherwise restore each to the channel-5 volume and remember the
/// setting at `+0x62b4`.
pub fn ssx_audio_system_set_sfx_sub_channels_enabled(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        let off = c.g[S1][0] == 0;
        frame.save_fpr(c, 20, 0x30);
        if off {
            c.cycles += 10;
            c.mtc1(0, 20);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 9;
            c.mov_s(12, 20);
            call_ptr(c, 0x0028_7700, 0x0028_7640)?; // SetChannelVolume
            c.mov_s(12, 20);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0xa;
            call_ptr(c, 0x0028_7700, 0x0028_7650)?;
            sw(c, p(c.g[S0][0] as u32, 0x62b4), c.g[S1][0]);
        } else {
            c.cycles += 15;
            c.g[A1][0] = 5;
            call_ptr(c, 0x0028_7920, 0x0028_75f8)?; // GetChannelVolume
            c.mov_s(12, 0);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 9;
            call_ptr(c, 0x0028_7700, 0x0028_7608)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 5;
            call_ptr(c, 0x0028_7920, 0x0028_7614)?;
            c.mov_s(12, 0);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0xa;
            call_ptr(c, 0x0028_7700, 0x0028_7624)?;
            sw(c, p(c.g[S0][0] as u32, 0x62b4), c.g[S1][0]);
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

/// `void SndLoadQueue_dtor(this, flags)` (0x28a148): flush the queue and free
/// the four owned buffers.
pub fn snd_load_queue_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0048_3b10;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        sw_2(c, S0, 0x1d4, c.g[V0][0]);
        callee::SND_LOAD_QUEUE_FLUSH.call(c, 0x0028_a170)?;
        c.g[A0][0] = lw_3(c, S0, 0x1c0);
        let first_null = c.g[A0][0] == 0;
        Addr(gp(c, 0x520)).sw(c, 0);
        if !first_null {
            c.cycles += 2;
            call_ptr(c, table::EA_MEM_FREE, 0x0028_a184)?;
        }
        c.cycles += 3;
        c.g[A0][0] = lw_3(c, S0, 0x1b0);
        if c.g[A0][0] == 0 {
            c.g[A0][0] = lw_3(c, S0, 0x1a0);
        } else {
            c.cycles += 3;
            call_ptr(c, table::EA_MEM_FREE, 0x0028_a198)?;
            c.g[A0][0] = lw_3(c, S0, 0x1a0);
        }
        c.cycles += 2;
        if c.g[A0][0] == 0 {
            c.g[A0][0] = lw_3(c, S0, 0x190);
        } else {
            c.cycles += 3;
            call_ptr(c, table::EA_MEM_FREE, 0x0028_a1ac)?;
            c.g[A0][0] = lw_3(c, S0, 0x190);
        }
        c.cycles += 2;
        let third_null = c.g[A0][0] == 0;
        c.g[V0][0] = c.g[S1][0] & 1;
        if !third_null {
            c.cycles += 3;
            call_ptr(c, table::EA_MEM_FREE, 0x0028_a1c0)?;
            c.g[V0][0] = c.g[S1][0] & 1;
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x0028_a1d4)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_UnloadWorldBanks(self)` (0x286c00): finish the
/// current load, unload the world's banks 0..16 (except 13) and free the
/// bank memory.
pub fn ssx_audio_system_unload_world_banks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = 0;
        call_ptr(c, 0x002a_6f38, 0x00286c20)?;
        c.g[S2][0] = 0xd;
        c.g[V0][0] = Addr(p(r32(c, S1), 0x118)).lw(c);
        c.g[A1][0] = 1;
        c.g[A0][0] = Addr(r32(c, V0)).lw(c);
        call_ptr(c, 0x0028_9df0, 0x00286c34)?;
        loop {
            c.cycles += 2;
            let is13 = c.g[S0][0] == c.g[S2][0];
            c.g[A1][0] = c.g[S0][0];
            if !is13 {
                c.cycles += 3;
                c.g[V0][0] = Addr(p(r32(c, S1), 0x118)).lw(c);
                c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                call_ptr(c, 0x0028_bce8, 0x00286c4c)?;
            }
            c.cycles += 4;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
            c.g[V0][0] = ((c.g[S0][0] as i64) < 17) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = 3;
            if !more {
                c.cycles += 18;
                c.g[V0][0] = Addr(p(r32(c, S1), 0x118)).lw(c);
                c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                call_ptr(c, 0x0028_bde0, 0x00286c68)?;
                c.g[V0][0] = Addr(p(r32(c, S1), 0x118)).lw(c);
                c.g[A1][0] = 0xc;
                c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                call_ptr(c, 0x0028_bde0, 0x00286c78)?;
                c.g[V0][0] = Addr(p(r32(c, S1), 0x118)).lw(c);
                c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                call_ptr(c, 0x0028_a230, 0x00286c84)?;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0029_f5e0, 0x00286c8c)?;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0028_6c38);
            }
        }
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_OnResetRace(self)` (0x287108): reset the race sounds
/// and, once the race manager agrees, restart the race music.
pub fn ssx_audio_system_on_reset_race(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0028_71b0, 0x0028711c)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002a_4168, 0x00287124)?;
        let mut proceed = c.g[V0][0] != 0;
        if !proceed {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002a_41d8, 0x00287134)?;
            proceed = c.g[V0][0] != 0;
        }
        if proceed {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002a_4040, 0x00287144)?;
            if c.g[V0][0] == 0 {
                c.cycles += 7;
                c.g[A1][0] = Addr(gp_6(c, -0x848)).lw(c);
                c.g[V0][0] = Addr(p(r32(c, S0), 0x5824)).lw(c);
                c.g[V1][0] = Addr(p(r32(c, A1), 0xc0)).lw(c);
                c.g[A0][0] = Addr(p(r32(c, V1), 0x70)).lw(c);
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 0;
                    call_ptr(c, 0x0029_5028, 0x00287178)?;
                    if c.g[V0][0] == 0 {
                        c.cycles += 8;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0;
                        c.g[A2][0] = 0;
                        c.g[A3][0] = sx(0xffff_ffff);
                        c.g[T0][0] = 0;
                        c.g[T1][0] = 0;
                        call_ptr(c, 0x0029_5628, 0x0028719c)?;
                    }
                }
            }
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Audio system: unload front-end audio / song changed.

/// `void SSXAudioSystem_UnloadFrontEndAudio(self)` (0x286200): stop the
/// front-end music (depending on the two song slots), kill bank timers and
/// unload the front-end bank.
pub fn ssx_audio_system_unload_front_end_audio(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        'end: {
            'second: {
                'first: {
                    c.cycles += 8;
                    c.g[V1][0] = 2;
                    frame.save(c, S0, 0x10);
                    frame.save_ra(c, 0);
                    c.g[S0][0] = c.g[A0][0];
                    c.g[V0][0] = lw_4(c, at_4(c.g[S0][0], 0x608c));
                    let t = c.g[V0][0] == c.g[V1][0];
                    if t {
                        break 'first;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = lw_4(c, at_4(c.g[S0][0], 0x6090));
                    if c.g[V0][0] != c.g[V1][0] {
                        break 'second;
                    }
                    break 'first;
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x0028_6234);
                c.g[A0][0] = sx(at_4(c.g[S0][0], 0x118));
                call_ptr(c, 0x002b_3ac0, 0x0028_6234)?;
                c.g[A1][0] = 0;
                break 'end;
            }
            c.cycles += 5;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 12);
            c.g[RA][0] = sx(0x0028_624c);
            c.g[A0][0] = sx(at_4(c.g[S0][0], 0x118));
            call_ptr(c, 0x002b_3d48, 0x0028_624c)?;
            c.g[A1][0] = 0;
            break 'end;
        }
        c.cycles += 22;
        c.g[RA][0] = sx(0x0028_6258);
        c.g[A0][0] = c.g[S0][0];
        callee::SSX_AUDIO_SYSTEM_SET_MODE.call(c, 0x0028_6258)?;
        c.g[V0][0] = lw_4(c, at_4(c.g[S0][0], 0x118));
        c.g[A0][0] = lw_4(c, c.g[V0][0] as u32);
        c.g[RA][0] = sx(0x0028_6268);
        c.g[A0][0] = sx(at_4(c.g[A0][0], 0x1d8));
        call_ptr(c, 0x002a_dda0, 0x0028_6268)?;
        c.g[V0][0] = lw_4(c, at_4(c.g[S0][0], 0x118));
        c.g[A0][0] = lw_4(c, c.g[V0][0] as u32);
        c.g[V1][0] = lw_4(c, at_4(c.g[A0][0], 0xab0));
        c.g[A0][0] = sx(at_4(c.g[A0][0], 0x1d8));
        c.g[V0][0] = lh(c, at_4(c.g[V1][0], 0x10));
        c.g[A1][0] = lw_4(c, at_4(c.g[V1][0], 0x14));
        let target = c.g[A1][0] as u32;
        c.g[RA][0] = sx(0x0028_6288);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        call_ptr(c, target, 0x0028_6288)?;
        c.g[V0][0] = lw_4(c, at_4(c.g[S0][0], 0x118));
        c.g[A1][0] = 0;
        c.g[RA][0] = sx(0x0028_6298);
        c.g[A0][0] = lw_4(c, c.g[V0][0] as u32);
        callee::SND_BANK_MAN_UNLOAD_BANK.call(c, 0x0028_6298)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SndLoadQueue_ctor(self)` (0x28a058): five 0x18-byte queue nodes with the
/// same vtable, linked into a count-6 ring, then publish the queue in the
/// sound system's global.
pub fn sndloadqueue_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 60;
        let frame = Frame::open(c, 0x40);
        let _this = abi::arg(c, 0);
        c.g[V0][0] = 0x48_0000;
        frame.save(c, 16, 0x30);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3b10));
        frame.save(c, 17, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x10);
        c.g[S1][0] = 0x48_0000;
        frame.save_ra(c, 0);
        c.g[S2][0] = 6;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2820));
        c.sw(p(c.g[S0][0] as u32, 0x18c), c.g[S2][0]);
        c.sw(p(c.g[S0][0] as u32, 4), 0);
        c.g[A1][0] = c.g[S1][0];
        c.sw(p(c.g[S0][0] as u32, 8), 0);
        c.g[A0][0] = 0x18;
        c.sw(p(c.g[S0][0] as u32, 0x194), 0);
        c.g[A2][0] = 0;
        c.sw(p(c.g[S0][0] as u32, 0x198), 0);
        c.g[A3][0] = 0;
        c.g[RA][0] = sx(0x0028_a0b0);
        c.sw(p(c.g[S0][0] as u32, 0x1d4), c.g[V0][0]);
        call_ptr(c, 0x0031_7e30, 0x0028_a0b0)?; // operator new(0x18, ...)
        c.sw(p(c.g[S0][0] as u32, 0x19c), c.g[S2][0]);
        c.g[A1][0] = c.g[S1][0];
        c.sw(p(c.g[S0][0] as u32, 0x1a4), 0);
        c.g[A0][0] = 0x18;
        c.sw(p(c.g[S0][0] as u32, 0x1a8), 0);
        c.g[A2][0] = 0;
        c.sw(p(c.g[S0][0] as u32, 0x190), c.g[V0][0]);
        c.g[RA][0] = sx(0x0028_a0d4);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x0028_a0d4)?;
        c.sw(p(c.g[S0][0] as u32, 0x1ac), c.g[S2][0]);
        c.g[A1][0] = c.g[S1][0];
        c.sw(p(c.g[S0][0] as u32, 0x1b4), 0);
        c.g[A0][0] = 0x18;
        c.sw(p(c.g[S0][0] as u32, 0x1b8), 0);
        c.g[A2][0] = 0;
        c.sw(p(c.g[S0][0] as u32, 0x1a0), c.g[V0][0]);
        c.g[RA][0] = sx(0x0028_a0f8);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x0028_a0f8)?;
        c.sw(p(c.g[S0][0] as u32, 0x1bc), c.g[S2][0]);
        c.g[A1][0] = c.g[S1][0];
        c.sw(p(c.g[S0][0] as u32, 0x1c4), 0);
        c.g[A0][0] = 0x18;
        c.sw(p(c.g[S0][0] as u32, 0x1c8), 0);
        c.g[A2][0] = 0;
        c.sw(p(c.g[S0][0] as u32, 0x1b0), c.g[V0][0]);
        c.g[RA][0] = sx(0x0028_a11c);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x0028_a11c)?;
        c.sw(p(c.g[S0][0] as u32, 0x1c0), c.g[V0][0]);
        c.sw(p(c.g[S0][0] as u32, 0x1cc), 0);
        c.g[V0][0] = c.g[S0][0];
        c.sw(p(c.g[S0][0] as u32, 0x1d0), 0);
        c.sw(gp(c, 0x520), c.g[S0][0]);
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_SetFadeGain(SaMix* self, int mask, float time,
/// float v)` (0x287a10): with `time == 0` set the channels selected by
/// `mask` immediately and, for the master channel (bit 2), recompute the ten
/// mix outputs from its value; otherwise store a fade (mask/target/rate) and
/// mark it running when the rate is non-zero.
pub fn ssx_audio_system_set_fade_gain(c: &mut Cpu) {
    'b_287c3c: {
        c.cycles += 5;
        c.mtc1(0, 2);
        c.c_eq_s(13, 2);
        let t = !c.fcc();
        c.g[V0][0] = c.g[A1][0] & 0x1;
        if t {
            c.cycles += 11;
            c.lwc1(0, c.pa(A0, 0x63d0));
            c.lwc1(1, c.pa(GP, -0x4620));
            c.sub_s(0, 12, 0);
            c.stw_2(A0, 0x63c4, c.g[A1][0]);
            c.swc1(12, c.pa(A0, 0x63cc));
            c.div_s(0, 0, 13);
            c.mul_s(0, 0, 1);
            c.c_eq_s(0, 2);
            let t = c.fcc();
            c.swc1(0, c.pa(A0, 0x63c8));
            if t {
                break 'b_287c3c;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0x1;
                c.stw_2(A0, 0x63c0, c.g[V0][0]);
                break 'b_287c3c;
            }
        } else {
            'b_287a48: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.g[A1][0] & 0x2;
                if t {
                    break 'b_287a48;
                } else {
                    c.cycles += 7;
                    c.lwc1(0, c.pa(A0, 0x62d0));
                    c.lwc1(1, c.pa(A0, 0x63e8));
                    c.mul_s(0, 0, 12);
                    c.swc1(12, c.pa(A0, 0x6398));
                    c.mul_s(1, 0, 1);
                    c.swc1(0, c.pa(A0, 0x62d4));
                    c.swc1(1, c.pa(A0, 0x62d8));
                    break 'b_287a48;
                }
            }
            'b_287a6c: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.g[A1][0] & 0x4;
                if t {
                    break 'b_287a6c;
                } else {
                    c.cycles += 7;
                    c.lwc1(0, c.pa(A0, 0x6310));
                    c.lwc1(1, c.pa(A0, 0x63f8));
                    c.mul_s(0, 0, 12);
                    c.swc1(12, c.pa(A0, 0x63a8));
                    c.mul_s(1, 0, 1);
                    c.swc1(0, c.pa(A0, 0x6314));
                    c.swc1(1, c.pa(A0, 0x6318));
                    break 'b_287a6c;
                }
            }
            'b_287b04: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.lwc1(1, c.pa(GP, 0x43c));
                if t {
                    break 'b_287b04;
                } else {
                    c.cycles += 36;
                    c.lwc1(11, c.pa(A0, 0x6300));
                    c.lwc1(0, c.pa(A0, 0x63f4));
                    c.mul_s(11, 11, 12);
                    c.lwc1(2, c.pa(GP, 0x440));
                    c.lwc1(4, c.pa(GP, 0x444));
                    c.mul_s(0, 11, 0);
                    c.lwc1(5, c.pa(GP, 0x448));
                    c.lwc1(7, c.pa(GP, 0x44c));
                    c.lwc1(9, c.pa(GP, 0x450));
                    c.lwc1(10, c.pa(GP, 0x454));
                    c.mul_s(1, 0, 1);
                    c.lwc1(8, c.pa(GP, 0x458));
                    c.mul_s(2, 0, 2);
                    c.lwc1(6, c.pa(GP, 0x45c));
                    c.mul_s(4, 0, 4);
                    c.lwc1(3, c.pa(GP, 0x460));
                    c.mul_s(5, 0, 5);
                    c.mul_s(7, 0, 7);
                    c.swc1(1, c.pa(A0, 0x636c));
                    c.mul_s(3, 0, 3);
                    c.swc1(2, c.pa(A0, 0x6370));
                    c.mul_s(9, 0, 9);
                    c.swc1(4, c.pa(A0, 0x6374));
                    c.mul_s(10, 0, 10);
                    c.swc1(5, c.pa(A0, 0x6378));
                    c.mul_s(8, 0, 8);
                    c.swc1(7, c.pa(A0, 0x637c));
                    c.mul_s(6, 0, 6);
                    c.swc1(9, c.pa(A0, 0x6380));
                    c.swc1(10, c.pa(A0, 0x6384));
                    c.swc1(8, c.pa(A0, 0x6388));
                    c.swc1(6, c.pa(A0, 0x638c));
                    c.swc1(3, c.pa(A0, 0x6390));
                    c.swc1(12, c.pa(A0, 0x63a4));
                    c.swc1(11, c.pa(A0, 0x6304));
                    c.swc1(0, c.pa(A0, 0x6308));
                    break 'b_287b04;
                }
            }
            'b_287b2c: {
                c.cycles += 3;
                c.g[V0][0] = c.g[A1][0] & 0x8;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.g[A1][0] & 0x80;
                if t {
                    break 'b_287b2c;
                } else {
                    c.cycles += 7;
                    c.lwc1(0, c.pa(A0, 0x62e0));
                    c.lwc1(1, c.pa(A0, 0x63ec));
                    c.mul_s(0, 0, 12);
                    c.swc1(12, c.pa(A0, 0x639c));
                    c.mul_s(1, 0, 1);
                    c.swc1(0, c.pa(A0, 0x62e4));
                    c.swc1(1, c.pa(A0, 0x62e8));
                    break 'b_287b2c;
                }
            }
            'b_287b50: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.g[A1][0] & 0x20;
                if t {
                    break 'b_287b50;
                } else {
                    c.cycles += 7;
                    c.lwc1(0, c.pa(A0, 0x62f0));
                    c.lwc1(1, c.pa(A0, 0x63f0));
                    c.mul_s(0, 0, 12);
                    c.swc1(12, c.pa(A0, 0x63a0));
                    c.mul_s(1, 0, 1);
                    c.swc1(0, c.pa(A0, 0x62f4));
                    c.swc1(1, c.pa(A0, 0x62f8));
                    break 'b_287b50;
                }
            }
            'b_287b74: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.g[A1][0] & 0x40;
                if t {
                    break 'b_287b74;
                } else {
                    c.cycles += 7;
                    c.lwc1(0, c.pa(A0, 0x6320));
                    c.lwc1(1, c.pa(A0, 0x63fc));
                    c.mul_s(0, 0, 12);
                    c.swc1(12, c.pa(A0, 0x63ac));
                    c.mul_s(1, 0, 1);
                    c.swc1(0, c.pa(A0, 0x6324));
                    c.swc1(1, c.pa(A0, 0x6328));
                    break 'b_287b74;
                }
            }
            'b_287b98: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.g[A1][0] & 0x10;
                if t {
                    break 'b_287b98;
                } else {
                    c.cycles += 7;
                    c.lwc1(0, c.pa(A0, 0x6330));
                    c.lwc1(1, c.pa(A0, 0x6400));
                    c.mul_s(0, 0, 12);
                    c.swc1(12, c.pa(A0, 0x63b0));
                    c.mul_s(1, 0, 1);
                    c.swc1(0, c.pa(A0, 0x6334));
                    c.swc1(1, c.pa(A0, 0x6338));
                    break 'b_287b98;
                }
            }
            'b_287bbc: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.g[A1][0] & 0x100;
                if t {
                    break 'b_287bbc;
                } else {
                    c.cycles += 7;
                    c.lwc1(0, c.pa(A0, 0x6340));
                    c.lwc1(1, c.pa(A0, 0x6404));
                    c.mul_s(0, 0, 12);
                    c.swc1(12, c.pa(A0, 0x63b4));
                    c.mul_s(1, 0, 1);
                    c.swc1(0, c.pa(A0, 0x6344));
                    c.swc1(1, c.pa(A0, 0x6348));
                    break 'b_287bbc;
                }
            }
            'b_287be0: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.g[A1][0] & 0x200;
                if t {
                    break 'b_287be0;
                } else {
                    c.cycles += 7;
                    c.lwc1(0, c.pa(A0, 0x6350));
                    c.lwc1(1, c.pa(A0, 0x6408));
                    c.mul_s(0, 0, 12);
                    c.swc1(12, c.pa(A0, 0x63b8));
                    c.mul_s(1, 0, 1);
                    c.swc1(0, c.pa(A0, 0x6354));
                    c.swc1(1, c.pa(A0, 0x6358));
                    break 'b_287be0;
                }
            }
            c.cycles += 2;
            let t = c.g[V0][0] == 0u64;

            if t {
                break 'b_287c3c;
            } else {
                c.cycles += 8;
                c.lwc1(0, c.pa(A0, 0x6360));
                c.lwc1(1, c.pa(A0, 0x640c));
                c.mul_s(0, 0, 12);
                c.swc1(12, c.pa(A0, 0x63bc));
                c.mul_s(1, 0, 1);
                c.swc1(0, c.pa(A0, 0x6364));
                let ra = c.g[RA][0] as u32;
                c.swc1(1, c.pa(A0, 0x6368));
                c.pc = ra;
                return;
            }
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.pc = ra;
    return;
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_SetMixPreset (0x2883b0)

/// SSXAudioSystem fields.
pub(crate) const AUD_CHAN_OUT: u32 = 0x62c8;

// 11 channels of 0x10: +0 out, -4 gain
pub(crate) const AUD_CHAN_GAIN: u32 = 0x62c4;

pub(crate) const AUD_MIX_OUT: u32 = 0x636c;

// 10 floats: channel 4's out times a weight
pub(crate) const AUD_CUR_GAIN: u32 = 0x63e4;

// float per channel
pub(crate) const AUD_FADING: u32 = 0x6410;

pub(crate) const AUD_PRESET: u32 = 0x646c;

/// Preset table: 0x28 bytes each, gains per channel (-1: unchanged) at
/// +0x647c.. and the fade time at +0x64a0.
pub(crate) const AUD_PRESETS: u32 = 0x647c;

pub(crate) const PRESET_SIZE: u64 = 0x28;

pub(crate) const PRESET_FADE: u32 = 0x64a0;

/// The ten mix weights (gp+0x43c..0x460, `D_004A352C`..).
pub(crate) const GP_MIX_WEIGHTS: u32 = 0x43c;

/// The channels a preset sets (5 is not part of a preset).
pub(crate) const PRESET_CHANNELS: [u64; 9] = [1, 2, 3, 4, 6, 7, 8, 9, 10];

pub(crate) const AUDIO_GET_PRESET_GAIN: Callee = Callee {
    addr: 0x0028_8370,
    f: ee::s07::SSXAudioSystem_GetPresetGain,
};

pub(crate) const AUDIO_SET_GAIN_TARGET: Callee = Callee {
    addr: 0x0028_87a8,
    f: ee::s07::SSXAudioSystem_SetGainTarget,
};

/// `void SSXAudioSystem_SetMixPreset(SSXAudioSystem* self, int id)`
/// (0x2883b0): switch to mix preset `id` (negative: 0). Unless it is already
/// current, look up the gain of channels 1-4 and 6-10 (`GetPresetGain`: the
/// preset's gain, or the current/target one where the preset says -1). With
/// no fade time the gains apply at once: each channel's output becomes gain
/// times level, and the ten mix outputs follow channel 4 times their
/// weights. With a fade time every channel gets a gain target and fade
/// rate (`SetGainTarget`), and the system is marked fading.
pub fn ssx_audio_system_set_mix_preset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x100);
        c.g[V0][0] = u64::MAX;
        frame.save(c, S1, 0x90);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
        frame.save(c, S0, 0xa0);
        c.g[S1][0] = c.g[A0][0];
        if c.g[V0][0] == 0 {
            c.g[A1][0] = 0;
        }
        c.g[V0][0] = c.g[A1][0];
        for (reg, off) in [
            (S2, 0x80),
            (S3, 0x70),
            (S4, 0x60),
            (S5, 0x50),
            (S6, 0x40),
            (S7, 0x30),
            (FP, 0x20),
        ] {
            frame.save(c, reg, off);
        }
        frame.save_ra(c, 0x10);
        for i in 0..10 {
            frame.save_fpr(c, 29 - i, 0xf8 - 8 * i as u32);
        }
        let this = Addr(c.g[S1][0] as u32);
        c.g[V1][0] = this.off(AUD_PRESET).lw(c);
        c.g[S0][0] = PRESET_SIZE;
        if c.g[V0][0] != c.g[V1][0] {
            c.cycles += 73;
            this.off(AUD_PRESET).sw(c, c.g[V0][0]);
            c.mult(0, S0, V0, S0); // s0 = id * 0x28
                                   // Gains of the nine channels into f21..f29; the pointers to the
                                   // preset's entries stay in s2-s7, fp and two stack words.
            let ptr_regs = [S2, S3, S4, S5, S6, S7, FP];
            let rets = [
                0x0028_8440u32,
                0x0028_845c,
                0x0028_8478,
                0x0028_8494,
                0x0028_84b0,
                0x0028_84cc,
                0x0028_84e8,
                0x0028_8508,
                0x0028_8528,
            ];
            for (i, &ch) in PRESET_CHANNELS.iter().enumerate() {
                if i > 0 {
                    c.mov_s(20 + i, 0);
                }
                c.g[V0][0] = addu(sx(this.0), (AUD_PRESETS + 4 * i as u32) as u64);
                c.g[A0][0] = c.g[S1][0];
                let entry = addu(c.g[V0][0], c.g[S0][0]);
                if i < 7 {
                    c.g[ptr_regs[i]][0] = entry;
                } else {
                    c.g[V0][0] = entry;
                    let a = frame.at(c, 4 * (i as u32 - 7));
                    c.sw(a, entry);
                }
                c.g[A1][0] = ch;
                c.g[A2][0] = Addr(entry as u32).lw(c);
                AUDIO_GET_PRESET_GAIN.call(c, rets[i])?;
            }
            c.mov_s(29, 0);
            c.g[S0][0] = addu(c.g[S1][0], c.g[S0][0]);
            let fade = Addr(c.g[S0][0] as u32).off(PRESET_FADE).read_fpu(c);
            abi::set_fpr(c, 20, fade);
            c.f[0] = 0;
            abi::set_fcc(c, fade.eq_s(FpuF32(0)));
            c.g[A0][0] = addu(c.g[S1][0], AUD_CHAN_OUT as u64);
            if fade.eq_s(FpuF32(0)) {
                // Apply at once.
                c.cycles += 12;
                for (i, &ch) in PRESET_CHANNELS.iter().enumerate() {
                    let a = this.off(AUD_CUR_GAIN + 4 * ch as u32).0;
                    c.swc1(21 + i, a);
                }
                c.g[V1][0] = addu(c.g[S1][0], AUD_CUR_GAIN as u64);
                c.g[V0][0] = addu(c.g[S1][0], AUD_CHAN_GAIN as u64);
                c.g[A1][0] = 10;
                loop {
                    // out[k] = gain[k] * level[k], k = 0..10
                    c.cycles += 9;
                    let gain = Addr(c.g[V0][0] as u32).read_fpu(c);
                    c.g[A1][0] = addu(c.g[A1][0], (-1i64) as u64);
                    let level = Addr(c.g[V1][0] as u32).read_fpu(c);
                    c.g[V0][0] = addu(c.g[V0][0], 0x10);
                    c.g[V1][0] = addu(c.g[V1][0], 4);
                    let out = gain * level;
                    abi::set_fpr(c, 1, level);
                    abi::set_fpr(c, 0, out);
                    Addr(c.g[A0][0] as u32).write_fpu(c, out);
                    let more = (c.g[A1][0] as i64) >= 0;
                    c.g[A0][0] = addu(c.g[A0][0], 0x10);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0028_8578);
                    }
                }
                c.cycles += 32;
                let gp = Addr(c.g[GP][0] as u32);
                let m = this.off(AUD_CHAN_OUT + 4 * 0x10).read_fpu(c);
                // The weights' registers in the original: f8 f9 f10 f6 f7 f5
                // f4 f2 f1, and f3 (whose product lands in f0).
                let regs = [8usize, 9, 10, 6, 7, 5, 4, 2, 1, 3];
                for (k, &f) in regs.iter().enumerate() {
                    let w = gp.off(GP_MIX_WEIGHTS + 4 * k as u32).read_fpu(c);
                    let out = m * w;
                    this.off(AUD_MIX_OUT + 4 * k as u32).write_fpu(c, out);
                    if f == 3 {
                        abi::set_fpr(c, 3, w);
                        abi::set_fpr(c, 0, out);
                    } else {
                        abi::set_fpr(c, f, out);
                    }
                }
            } else {
                // Fade: a gain target per channel over `fade` seconds.
                c.cycles += 76;
                let rets = [
                    0x0028_863cu32,
                    0x0028_865c,
                    0x0028_867c,
                    0x0028_869c,
                    0x0028_86bc,
                    0x0028_86dc,
                    0x0028_86fc,
                    0x0028_8720,
                    0x0028_8744,
                ];
                for (i, &ch) in PRESET_CHANNELS.iter().enumerate() {
                    let entry = if i < 7 {
                        c.g[ptr_regs[i]][0]
                    } else {
                        let a = frame.at(c, 4 * (i as u32 - 7));
                        c.g[V0][0] = c.lw(a);
                        c.g[V0][0]
                    };
                    let gain = Addr(entry as u32).lw(c);
                    c.mov_s(12, 21 + i);
                    c.mov_s(13, 20);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = ch;
                    // a2 = (gain != -1)
                    c.g[A2][0] = (0u64 < !gain) as u64;
                    AUDIO_SET_GAIN_TARGET.call(c, rets[i])?;
                }
                c.g[V0][0] = 1;
                this.off(AUD_FADING).sw(c, 1);
            }
        }
        c.cycles += 22;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        for (reg, off) in [
            (S2, 0x80),
            (S3, 0x70),
            (S4, 0x60),
            (S5, 0x50),
            (S6, 0x40),
            (S7, 0x30),
            (FP, 0x20),
        ] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0x10);
        for i in 0..10 {
            frame.restore_fpr(c, 29 - i, 0xf8 - 8 * i as u32);
        }
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod audio_fade {

    /// `+0x63c0` a fade runs.
    pub const FADING: u32 = 0x63c0;

    /// `+0x63c4` which mixer channels follow the fade (bit mask).
    pub const MASK: u32 = 0x63c4;

    /// `+0x63c8` fade step per tick.
    pub const RATE: u32 = 0x63c8;

    /// `+0x63cc` fade target level.
    pub const TARGET: u32 = 0x63cc;

    /// `+0x63d0` current fade level.
    pub const CUR: u32 = 0x63d0;

    /// A mixer channel: `+0` volume, `+4` volume * fade, `+8` that * gain.
    pub struct Chan {
        pub bit: u64,
        pub base: u32,
        pub gain: u32,
        /// Where the channel's copy of the fade level lives.
        pub level: u32,
    }

    pub(crate) const fn ch(bit: u64, base: u32, gain: u32, level: u32) -> Chan {
        Chan {
            bit,
            base,
            gain,
            level,
        }
    }

    /// Channels 1, 5 (before the reverb-mix channel 4).
    pub const FIRST: [Chan; 2] = [
        ch(0x1, 0x62d0, 0x63e8, 0x6398),
        ch(0x2, 0x6310, 0x63f8, 0x63a8),
    ];

    /// Channel 4: also scales the ten send levels at `+0x636c` by the
    /// constants at `gp+0x43c..`.
    pub const MIX: Chan = ch(0x4, 0x6300, 0x63f4, 0x63a4);

    pub const MIX_SENDS: u32 = 0x636c;

    pub const GP_MIX_LEVELS: u32 = 0x43c;

    /// FPU registers the ten send levels go through.
    pub const MIX_REGS: [usize; 10] = [1, 2, 4, 5, 7, 9, 10, 8, 6, 3];

    /// Channel 2 (its mask is reloaded first).
    pub const AFTER_MIX: Chan = ch(0x8, 0x62e0, 0x63ec, 0x639c);

    /// Channels 3, 6, 7, 8, 9.
    pub const REST: [Chan; 5] = [
        ch(0x80, 0x62f0, 0x63f0, 0x63a0),
        ch(0x20, 0x6320, 0x63fc, 0x63ac),
        ch(0x40, 0x6330, 0x6400, 0x63b0),
        ch(0x10, 0x6340, 0x6404, 0x63b4),
        ch(0x100, 0x6350, 0x6408, 0x63b8),
    ];

    /// Channel 10 (last: no mask reload).
    pub const LAST: Chan = ch(0x200, 0x6360, 0x640c, 0x63bc);
}

/// One mixer channel follows the fade level: `level = cur`,
/// `vol' = vol * cur`, `out = vol' * gain`.
pub(crate) fn fade_channel(c: &mut Cpu, s: u32, ch: &audio_fade::Chan) {
    c.lwc1(0, s.wrapping_add(audio_fade::CUR));
    c.lwc1(1, s.wrapping_add(ch.base));
    c.lwc1(2, s.wrapping_add(ch.gain));
    c.mul_s(1, 1, 0);
    c.swc1(0, s.wrapping_add(ch.level));
    c.mul_s(2, 1, 2);
    c.swc1(1, s.wrapping_add(ch.base + 4));
    c.swc1(2, s.wrapping_add(ch.base + 8));
}

/// `void SSXAudioSystem::UpdateFade()` (0x287c48): while a fade runs, step
/// the fade level by the rate and stop at the target (from either side);
/// then every channel selected by the mask takes the level (channel 4 also
/// rescales its ten send levels).
pub fn ssx_audio_system_update_fade_2(c: &mut Cpu) {
    use audio_fade::*;
    let s = c.g[A0][0] as u32;
    c.cycles += 3;
    c.g[V0][0] = c.lw(s.wrapping_add(FADING));
    if c.g[V0][0] != 0 {
        // cur += rate; past the target: clamp and stop.
        c.cycles += 8;
        c.lwc1(0, s.wrapping_add(CUR));
        c.lwc1(1, s.wrapping_add(RATE));
        c.mtc1(0, 2);
        c.add_s(0, 0, 1);
        c.c_lt_s(1, 2);
        let falling = c.fcc();
        c.swc1(0, s.wrapping_add(CUR));
        if falling {
            c.cycles += 4;
            c.lwc1(1, s.wrapping_add(TARGET));
            c.c_lt_s(0, 1);
        } else {
            c.cycles += 3;
            c.lwc1(1, s.wrapping_add(TARGET));
            c.c_lt_s(1, 0);
        }
        c.cycles += 2;
        if c.fcc() {
            c.cycles += 3;
            c.swc1(1, s.wrapping_add(CUR));
            c.sw(s.wrapping_add(FADING), 0);
        }
        c.g[V0][0] = c.lw(s.wrapping_add(MASK));

        for ch in FIRST.iter() {
            c.cycles += 3;
            c.g[V0][0] &= ch.bit;
            if c.g[V0][0] != 0 {
                c.cycles += 9;
                fade_channel(c, s, ch);
            }
            c.g[V0][0] = c.lw(s.wrapping_add(MASK));
        }

        c.cycles += 3;
        c.g[V0][0] &= MIX.bit;
        let gp = c.g[GP][0] as u32;
        c.lwc1(MIX_REGS[0], gp.wrapping_add(GP_MIX_LEVELS));
        if c.g[V0][0] != 0 {
            c.cycles += 37;
            c.lwc1(12, s.wrapping_add(CUR));
            c.lwc1(11, s.wrapping_add(MIX.base));
            c.lwc1(0, s.wrapping_add(MIX.gain));
            c.mul_s(11, 11, 12);
            c.mul_s(0, 11, 0);
            for (i, &f) in MIX_REGS.iter().enumerate() {
                let k = gp.wrapping_add(GP_MIX_LEVELS + 4 * i as u32);
                c.lwc1(f, k);
                c.mul_s(f, 0, f);
                c.swc1(f, s.wrapping_add(MIX_SENDS + 4 * i as u32));
            }
            c.swc1(12, s.wrapping_add(MIX.level));
            c.swc1(11, s.wrapping_add(MIX.base + 4));
            c.swc1(0, s.wrapping_add(MIX.base + 8));
        }

        c.cycles += 1;
        c.g[V0][0] = c.lw(s.wrapping_add(MASK));
        for ch in core::iter::once(&AFTER_MIX).chain(REST.iter()) {
            c.cycles += 3;
            c.g[V0][0] &= ch.bit;
            if c.g[V0][0] != 0 {
                c.cycles += 9;
                fade_channel(c, s, ch);
            }
            c.g[V0][0] = c.lw(s.wrapping_add(MASK));
        }

        c.cycles += 3;
        c.g[V0][0] &= LAST.bit;
        if c.g[V0][0] != 0 {
            c.cycles += 8;
            fade_channel(c, s, &LAST);
        }
    }
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------- SSXAudioSystem_UpdateSpeechDucking

/// `SSXAudioSystem_GetSpeechMode(self, 0)` (0x29f088).
pub(crate) const AUDIO_GET_SPEECH_MODE: u32 = 0x0029_f088;

/// `SSXAudioSystem_SetDuckGain(self, float gain)`.
pub(crate) const AUDIO_SET_DUCK_GAIN: Callee = Callee {
    addr: 0x0028_9830,
    f: ee::s07::SSXAudioSystem_SetDuckGain,
};

/// `void SSXAudioSystem_UpdateSpeechDucking(SSXAudioSystem* self)`
/// (0x289688): duck the music under speech. While a speech envelope plays
/// (`+0x6c58`), every other tick its next level is looked up (index
/// `tick * rate / stream rate`; past the end the envelope stops) and turned
/// into a target gain (1 below the first threshold, else 1 or 0 by mode);
/// the gain then ramps there (`+0x6c64` per tick) over the following tick.
/// The gain is applied by `SetDuckGain` on the tick the target is set and
/// while ramping (`+0x6c60 <= 0`).
pub fn ssx_audio_system_update_speech_ducking(c: &mut Cpu) {
    /// SSXAudioSystem fields.
    const TICK: u32 = 0x6864;
    const LEVELS: u32 = 0x6868;
    const LEVEL_COUNT: u32 = 0x6c50;
    const LEVEL_RATE: u32 = 0x6c54;
    const PLAYING: u32 = 0x6c58;
    const STEP_STATE: u32 = 0x6c60;
    const GAIN_STEP: u32 = 0x6c64;
    const GAIN: u32 = 0x6c68;
    const TARGET: u32 = 0x6c6c;
    /// `gp+0x2a74`: the speech stream (its rate at +0x2c).
    const STREAM_GP: u32 = 0x2a74;
    const ONE: u32 = 0x3f80_0000;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = r32(c, S0);
        let at = |o: u32| this.wrapping_add(o);
        c.g[V0][0] = c.lw(at(PLAYING));
        if c.g[V0][0] != 0 {
            c.cycles += 7;
            let tick = c.lw(at(TICK));
            c.g[A0][0] = tick;
            let t = tick as u32;
            let even = sx((((t.wrapping_add(t >> 31) as i32) >> 1) as u32) << 1);
            c.g[V0][0] = even;
            if tick == even {
                c.cycles += 16;
                c.g[V1][0] = c.lw(at(LEVEL_RATE));
                c.lwc1(0, at(TARGET));
                c.mult(0, V1, A0, V1);
                c.sw(at(STEP_STATE), 0);
                c.swc1(0, at(GAIN));
                c.g[A0][0] = c.lw(r32(c, GP).wrapping_add(STREAM_GP));
                c.g[V0][0] = c.lw(at(LEVEL_COUNT));
                c.mtc1(V1, 0);
                c.cvt_s_w(0, 0);
                c.lwc1(1, r32(c, A0).wrapping_add(0x2c));
                c.div_s(0, 0, 1);
                c.cvt_w_s(1, 0);
                c.mfc1(V1, 1);
                let in_range = (c.g[V1][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = in_range as u64;
                c.g[V0][0] = sx(this.wrapping_add(r32(c, V1)));
                if !in_range {
                    // Past the end: stop, back to full gain.
                    c.cycles += 4;
                    c.g[AT][0] = ONE as u64;
                    c.f[0] = ONE;
                    c.sw(at(PLAYING), 0);
                    c.swc1(0, at(TARGET));
                } else {
                    c.cycles += 3;
                    c.g[S1][0] = c.lb(r32(c, V0).wrapping_add(LEVELS));
                    c.g[A0][0] = c.g[S0][0];
                    if (c.g[S1][0] as i64) <= 0 {
                        c.cycles += 4;
                        c.g[AT][0] = ONE as u64;
                        c.f[0] = ONE;
                        c.swc1(0, at(TARGET));
                    } else {
                        c.cycles += 6;
                        c.g[A1][0] = 0;
                        call_ptr(c, AUDIO_GET_SPEECH_MODE, 0x0028_9718)?;
                        c.g[V1][0] = c.g[V0][0];
                        // Thresholds (v0, a0, v1) and gains (f1 above the
                        // first, f0 above the last).
                        if c.g[V1][0] == 2 {
                            c.cycles += 7;
                            c.g[V0][0] = 0;
                            c.g[A0][0] = 0;
                            c.g[AT][0] = ONE as u64;
                            c.f[1] = ONE;
                            c.f[0] = 0;
                            c.g[V1][0] = 0x1e;
                        } else {
                            c.cycles += 6;
                            c.g[AT][0] = ONE as u64;
                            c.f[0] = ONE;
                            c.g[V0][0] = 0x1e;
                            c.g[A0][0] = 0x28;
                            c.g[V1][0] = 0x32;
                            c.f[1] = c.f[0];
                        }
                        c.cycles += 3;
                        let level = c.g[S1][0] as i64;
                        let below0 = level < c.g[V0][0] as i64;
                        c.g[V0][0] = (level < c.g[A0][0] as i64) as u64;
                        if below0 {
                            c.cycles += 4;
                            c.g[AT][0] = ONE as u64;
                            c.f[0] = ONE;
                            c.swc1(0, at(TARGET));
                        } else {
                            c.cycles += 2;
                            if c.g[V0][0] != 0 {
                                c.swc1(1, at(TARGET));
                            } else {
                                c.cycles += 3;
                                let below2 = level < c.g[V1][0] as i64;
                                c.g[V0][0] = below2 as u64;
                                if !below2 {
                                    c.swc1(0, at(TARGET));
                                } else {
                                    c.cycles += 2;
                                    c.swc1(1, at(TARGET));
                                }
                            }
                        }
                    }
                }
                // The ramp step toward the new target.
                c.cycles += 5;
                c.lwc1(0, at(TARGET));
                c.lwc1(1, at(GAIN));
                c.sub_s(0, 0, 1);
                c.swc1(0, at(GAIN_STEP));
            }
            c.g[V0][0] = c.lw(at(TICK));
            c.cycles += 3;
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
            c.sw(at(TICK), c.g[V0][0]);
        }
        c.g[V0][0] = c.lw(at(STEP_STATE));

        c.cycles += 2;
        if (c.g[V0][0] as i64) <= 0 {
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                // Ramping: gain += step.
                c.lwc1(0, at(GAIN));
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.lwc1(1, at(GAIN_STEP));
                c.add_s(0, 0, 1);
                c.mov_s(12, 0);
                c.swc1(0, at(GAIN));
            } else {
                c.cycles += 3;
                c.lwc1(12, at(TARGET));
                c.g[A0][0] = c.g[S0][0];
            }
            c.cycles += 6;
            AUDIO_SET_DUCK_GAIN.call(c, 0x0028_980c)?;
            c.g[V0][0] = c.lw(at(STEP_STATE));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
            c.sw(at(STEP_STATE), c.g[V0][0]);
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

pub(crate) mod mix_ramp {

    /// Non-zero while a ramp is in progress.
    pub const DIRTY: u32 = 0x6410;

    /// Eleven per-channel ramp steps (0: idle).
    pub const DELTA: u32 = 0x6414;

    /// Eleven current levels.
    pub const CUR: u32 = 0x63e4;

    /// Eleven targets (`CUR + 0x5c`).
    pub const TARGET_FROM_CUR: u32 = 0x5c;

    /// Eleven channel pairs `{base, scaled}` (16 bytes apart; `scaled` at
    /// 0x62c8 + 16 i, `base` 4 bytes before it).
    pub const CHAN_SCALED: u32 = 0x62c8;

    /// The master level.
    pub const MASTER: u32 = 0x6308;

    /// Ten outputs: master times the gp table at 0x43c..0x460.
    pub const OUT: u32 = 0x636c;
}

/// `void SSXAudioSystem_UpdateMixRamp(SSXAudioSystem* self)` (0x2887f8):
/// while a mix ramp is running, step each of the eleven channel levels
/// toward its target (clamping and stopping the step on arrival, keeping
/// the ramp running otherwise), rescale each stepping channel by its level,
/// and recompute the ten master outputs from the gp gain table.
pub fn ssx_audio_system_update_mix_ramp(c: &mut Cpu) {
    use mix_ramp::*;
    c.cycles += 3;
    let this = r32(c, A0);
    c.g[V0][0] = c.lw(this + DIRTY);
    c.g[T1][0] = 1;
    if c.g[V0][0] != 0 {
        c.cycles += 7;
        c.sw(this + DIRTY, 0);
        c.mtc1(0, 2); // 0.0
        c.g[A2][0] = sx(this + CHAN_SCALED);
        c.g[V1][0] = sx(this + CUR);
        c.g[A1][0] = sx(this + DELTA);
        c.g[T0][0] = 0;
        c.g[A3][0] = 10;
        loop {
            // Loop state lives in a1 (delta), v1 (cur), a2 (channel), t0
            // (byte offset), a3 (count down).
            let this = r32(c, A0);
            let delta = r32(c, A1);
            let cur = r32(c, V1);
            c.cycles += 5;
            c.lwc1(1, delta);
            c.c_eq_s(1, 2);
            if c.fcc() {
                c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x10));
            } else {
                // cur += delta (f1).
                c.cycles += 8;
                c.lwc1(0, cur);
                c.add_s(1, 0, 1);
                c.swc1(1, cur);
                c.lwc1(0, delta);
                c.c_lt_s(0, 2);
                let falling = c.fcc();
                c.g[V0][0] = sx(this + CUR);
                let arrived;
                if falling {
                    c.cycles += 5;
                    c.lwc1(0, cur + TARGET_FROM_CUR);
                    c.c_le_s(1, 0);
                    arrived = c.fcc();
                    if !arrived {
                        c.cycles += 2;
                    }
                } else {
                    c.cycles += 5;
                    c.lwc1(0, cur + TARGET_FROM_CUR);
                    c.c_le_s(0, 1);
                    arrived = c.fcc();
                    if arrived {
                        c.cycles += 1;
                    }
                }
                if arrived {
                    // Clamp to the target, stop stepping.
                    c.swc1(0, cur);
                    c.cycles += 1;
                    c.sw(delta, 0);
                } else {
                    c.sw(this + DIRTY, c.g[T1][0]);
                }
                // scaled = base * cur.
                c.cycles += 6;
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, T0)));
                let chan = r32(c, A2);
                c.lwc1(0, chan.wrapping_sub(4));
                c.lwc1(1, r32(c, V0));
                c.mul_s(0, 0, 1);
                c.swc1(0, chan);
                c.g[A2][0] = sx(chan.wrapping_add(0x10));
            }
            c.cycles += 5;
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(4));
            c.g[A3][0] = sx(r32(c, A3).wrapping_sub(1));
            let more = (c.g[A3][0] as i64) >= 0;
            c.g[T0][0] = sx(r32(c, T0).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0028_8820);
            }
        }
        // Outputs: master * gain[k].
        c.cycles += 31;
        let gp = r32(c, GP);
        let this = r32(c, A0);
        c.lwc1(8, gp + 0x43c);
        c.lwc1(9, gp + 0x440);
        c.lwc1(10, gp + 0x444);
        c.lwc1(6, gp + 0x448);
        c.lwc1(7, gp + 0x44c);
        c.lwc1(5, gp + 0x450);
        c.lwc1(4, gp + 0x454);
        c.lwc1(2, gp + 0x458);
        c.lwc1(1, gp + 0x45c);
        c.lwc1(0, this + MASTER);
        c.lwc1(3, gp + 0x460);
        c.mul_s(8, 0, 8);
        c.mul_s(9, 0, 9);
        c.mul_s(10, 0, 10);
        c.mul_s(6, 0, 6);
        c.swc1(8, this + OUT);
        c.mul_s(7, 0, 7);
        c.swc1(9, this + OUT + 0x4);
        c.mul_s(5, 0, 5);
        c.swc1(10, this + OUT + 0x8);
        c.mul_s(4, 0, 4);
        c.swc1(6, this + OUT + 0xc);
        c.mul_s(2, 0, 2);
        c.swc1(7, this + OUT + 0x10);
        c.mul_s(1, 0, 1);
        c.swc1(5, this + OUT + 0x14);
        c.mul_s(0, 0, 3);
        c.swc1(4, this + OUT + 0x18);
        c.swc1(2, this + OUT + 0x1c);
        c.swc1(1, this + OUT + 0x20);
        c.swc1(0, this + OUT + 0x24);
    }
    c.cycles += 2;
    ret(c);
}

/// `int SSXAudioSystem_GetRiderRatingTier(self, Rider* rider)` (0x289c18,
/// ssxAudio.cpp): the rider's overall rating tier for speech selection:
/// asks the rider's controller (`rider + 0x6c0`, vtable slot 0x38) for its
/// character, rates it with `BECharInt_GetOverallRating` on the BE library's
/// interface 3, and returns 0 below 4, 1 below 8, else 2.
pub fn ssx_audio_system_get_rider_rating_tier(c: &mut Cpu) {
    const CONTROLLER: u32 = 0x6c0;
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, 0x0014_dc80, 0x0028_9c30)?; // BELibrary_GetInstance
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 3;
        call_ptr(c, 0x0014_dd58, 0x0028_9c3c)?; // BELibrary_GetInterface
        let controller = r32(c, S0).wrapping_add(CONTROLLER);
        let vt = Addr(controller).lw(c);
        c.g[V1][0] = vt;
        c.g[S1][0] = c.g[V0][0];
        c.g[S0][0] = sx(controller);
        let delta = Addr(vt as u32).off(0x38).lh(c);
        let method = Addr(vt as u32).off(0x3c).lw(c);
        c.g[V0][0] = method;
        c.g[A0][0] = sx(controller.wrapping_add(delta as u32));
        call_ptr(c, method as u32, 0x0028_9c58)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0014_8950, 0x0028_9c64)?; // BECharInt_GetOverallRating
        let rating = c.g[V0][0] as i64;
        c.g[A0][0] = c.g[V0][0];
        if rating < 4 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 4;
            let below8 = rating < 8;
            c.g[A0][0] = below8 as u64;
            c.g[V1][0] = 1;
            c.g[V0][0] = if below8 { 1 } else { 2 };
        }
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem_GetListenerCount` (0x288ce8).
pub(crate) const AUDIO_GET_LISTENER_COUNT: Callee = Callee {
    addr: 0x0028_8ce8,
    f: ee::s07::SSXAudioSystem_GetListenerCount,
};

/// `SSXAudioSystem_GetViewPos` (0x288ac0).
pub(crate) const AUDIO_GET_VIEW_POS: Callee = Callee {
    addr: 0x0028_8ac0,
    f: ee::s07::SSXAudioSystem_GetViewPos,
};

/// The loop body of [`ssx_audio_find_view_in_range`] (a resume entry).
pub(crate) const FIND_VIEW_BODY: u32 = 0x0028_8b78;

/// `int SSXAudioSystem_FindViewInRange(void* self, sAudV4* pos, float radius)`
/// (0x288b40): the index of the first listener view whose position lies
/// within `radius * 100` of `pos` (squared 4D distance from VU0), else -1.
/// The loop head (0x288b78) is a resume entry: an event at the back edge
/// can unwind there.
pub fn ssx_audio_find_view_in_range(c: &mut Cpu) {
    let frame = Frame { size: 0x70 };
    guest(c, |c| {
        match c.pc {
            0x0028_8b40 => {
                c.cycles += 14;
                Frame::open(c, 0x70);
                c.g[AT][0] = 0x42c8_0000; // 100.0f
                c.mtc1(AT, 0);
                frame.save_fpr(c, 20, 0x60);
                c.mul_s(20, 12, 0);
                frame.save(c, S0, 0x50);
                frame.save(c, S1, 0x40);
                c.g[S0][0] = 0;
                frame.save(c, S2, 0x30);
                c.g[S1][0] = c.g[A0][0];
                // f20 = (radius * 100)^2
                c.mul_s(20, 20, 20);
                frame.save_ra(c, 0x20);
                c.g[S2][0] = c.g[A1][0];
            }
            FIND_VIEW_BODY => {
                if find_view_in_range_body(c, frame)? {
                    return find_view_in_range_exit(c, frame);
                }
            }
            pc => {
                c.jump_out(pc, 0x0028_8b40, 0x0028_8c04);
                return Ok(());
            }
        }
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            AUDIO_GET_LISTENER_COUNT.call(c, 0x0028_8bd8)?;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[V0][0] = u64::MAX; // -1: none in range
                break;
            }
            c.g[A0][0] = c.g[S1][0];
            if event_due(c) && crate::events::back_edge_unwinds(c, FIND_VIEW_BODY) {
                return Ok(());
            }
            if find_view_in_range_body(c, frame)? {
                break;
            }
        }
        find_view_in_range_exit(c, frame)
    })
}

/// One view of [`ssx_audio_find_view_in_range`] (0x288b78): true (with
/// v0 = index) when view `s0` is in range, else `s0 += 1`.
pub(crate) fn find_view_in_range_body(
    c: &mut Cpu,
    frame: Frame,
) -> Result<bool, crate::call::Unwound> {
    use ssx3_ee_rt::vu0::{W, Y, Z};
    c.cycles += 21;
    c.g[A1][0] = c.g[S0][0];
    AUDIO_GET_VIEW_POS.call(c, 0x0028_8b80)?;
    // d = view_pos - pos, through the stack as the original spills it.
    c.lqc2(3, r32(c, V0));
    c.lqc2(4, r32(c, S2));
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, frame.at(c, 0x10));
    c.lq(V0, frame.at(c, 0x10));
    c.sq(V0, frame.at(c, 0));
    c.lqc2(3, frame.at(c, 0));
    c.lqc2(5, frame.at(c, 0));
    // dot(d, d) in vf4.x (vf6.x = 1.0)
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(V1, 4);
    c.mtc1(V1, 0);
    c.c_lt_s(0, 20);
    c.g[V0][0] = c.g[S0][0];
    if c.fcc() {
        return Ok(true);
    }
    c.cycles += 1;
    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
    Ok(false)
}

/// The epilogue of [`ssx_audio_find_view_in_range`] (0x288be8).
pub(crate) fn find_view_in_range_exit(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 7;
    frame.restore(c, S0, 0x50);
    frame.restore(c, S1, 0x40);
    frame.restore(c, S2, 0x30);
    frame.restore_ra(c, 0x20);
    frame.restore_fpr(c, 20, 0x60);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// 0x00288a20 SSXAudioSystem_GetRiderListenerIndex

/// Rider fields read by [`ssx_audio_system_get_rider_listener_index`].
pub(crate) const RLI_FIXED_FLAG: u32 = 0x874;

pub(crate) const RLI_FIXED_ON: u32 = 0x87c;

pub(crate) const RLI_FIXED_INDEX: u32 = 0x870;

/// The rider's embedded object whose vtable slot 5 (`+0x28`) gives its
/// world position.
pub(crate) const RLI_POS_OBJECT: u32 = 0x6c0;

/// `int SSXAudioSystem_GetRiderListenerIndex(SSXAudioSystem* self, Rider*
/// rider)` (0x288a20): which listener (view) hears this rider. 0 outside the
/// game; the rider's fixed index (`+0x870`) when it has one (`+0x874` and
/// `+0x87c` set); 0 with a single listener; else whether the rider's
/// position is closer to the second view.
pub fn ssx_audio_system_get_rider_listener_index(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        Callee {
            addr: 0x0028_b1b0,
            f: ee::s07::SSXAudioSystem_IsInGame,
        }
        .call(c, 0x0028_8a3c)?;
        let in_game = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        // Ends at the shared "restore s0" block (+1) unless the second-view
        // test ran.
        let mut via_restore = true;
        if in_game {
            let rider = c.g[S0][0] as u32;
            c.cycles += 3;
            c.g[V0][0] = c.lw(rider.wrapping_add(RLI_FIXED_FLAG));
            c.g[V1][0] = 0;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[V0][0] = c.lw(rider.wrapping_add(RLI_FIXED_ON));
                c.g[V1][0] = (0 < c.g[V0][0]) as u64;
            }
            c.cycles += 2;
            if c.g[V1][0] != 0 {
                c.cycles += 2;
                c.g[V0][0] = c.lw(rider.wrapping_add(RLI_FIXED_INDEX));
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                Callee {
                    addr: 0x0028_8ce8,
                    f: ee::s07::SSXAudioSystem_GetListenerCount,
                }
                .call(c, 0x0028_8a70)?;
                c.g[V1][0] = 0x1;
                let rider = c.g[S0][0] as u32;
                c.g[A1][0] = sx(rider.wrapping_add(RLI_POS_OBJECT));
                if c.g[V0][0] == c.g[V1][0] {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                } else {
                    c.cycles += 10;
                    c.g[V1][0] = c.lw(rider.wrapping_add(RLI_POS_OBJECT));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                    let method = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, method, 0x0028_8a90)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[V0][0];
                    Callee {
                        addr: 0x0028_8940,
                        f: ee::s07::SSXAudioSystem_IsCloserToSecondView,
                    }
                    .call(c, 0x0028_8a9c)?;
                    via_restore = false;
                }
            }
        }
        if via_restore {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_IsAnyViewInRange (0x288c08)

/// Entry of `SSXAudioSystem_IsAnyViewInRange`.
pub(crate) const OC305_VIEW_IN_RANGE_ENTRY: u32 = 0x0028_8c08;

/// Loop head (the per-view body), where the function resumes after an
/// unwind at the back-edge event check.
pub(crate) const OC305_VIEW_IN_RANGE_HEAD: u32 = 0x0028_8c48;

/// End of the function (for `jump_out`).
pub(crate) const OC305_VIEW_IN_RANGE_END: u32 = 0x0028_8ce8;

/// Metres to world units (100.0f).
pub(crate) const OC305_VIEW_IN_RANGE_UNITS: u32 = 0x42c8_0000;

/// `SSXAudioSystem_GetViewPos` (0x288ac0).
pub(crate) const OC305_SSX_AUDIO_GET_VIEW_POS: Callee = Callee {
    addr: 0x0028_8ac0,
    f: ee::s07::SSXAudioSystem_GetViewPos,
};

/// `SSXAudioSystem_GetListenerCount` (0x288ce8).
pub(crate) const OC305_SSX_AUDIO_GET_LISTENER_COUNT: Callee = Callee {
    addr: 0x0028_8ce8,
    f: ee::s07::SSXAudioSystem_GetListenerCount,
};

/// `bool SSXAudioSystem_IsAnyViewInRange(SSXAudioSystem* self, const Vec4*
/// pos, float radius)` (0x288c08): whether any listener view lies within
/// `radius` metres (`radius * 100` units, compared squared) of `pos` with
/// the view in front of it (`(view - pos).z > 0`). Frame 0x70 (ra at 0x20,
/// s0..s2 at 0x50/0x40/0x30, f20/f21 at 0x60/0x68; the difference vector
/// spilled at 0x10 and 0x0). Loop state: s0 = view index, s1 = self, s2 =
/// pos, f20 = radius squared, f21 = 0. Dispatcher-shaped: re-entered at
/// [`OC305_VIEW_IN_RANGE_HEAD`] after an unwind at the loop's event check.
pub fn ssx_audio_system_is_any_view_in_range(c: &mut Cpu) {
    use crate::abi::{c_lt_s, set_fpr};
    use crate::ps2f::FpuF32;
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    guest(c, |c| {
        let frame = Frame { size: 0x70 };
        // Entering at the head resumes inside the loop body.
        let mut at_head = match c.pc {
            OC305_VIEW_IN_RANGE_ENTRY => {
                c.cycles += 16;
                let frame = Frame::open(c, 0x70);
                c.g[AT][0] = sx(OC305_VIEW_IN_RANGE_UNITS);
                c.f[0] = OC305_VIEW_IN_RANGE_UNITS;
                frame.save_fpr(c, 20, 0x60);
                let r = crate::abi::farg(c, 0) * FpuF32(c.f[0]);
                set_fpr(c, 20, r);
                frame.save_fpr(c, 21, 0x68);
                frame.save(c, S0, 0x50);
                frame.save(c, S1, 0x40);
                c.g[S0][0] = 0;
                set_fpr(c, 20, r * r);
                frame.save(c, S2, 0x30);
                c.f[21] = 0;
                c.g[S1][0] = c.g[A0][0];
                frame.save_ra(c, 0x20);
                c.g[S2][0] = c.g[A1][0];
                false
            }
            OC305_VIEW_IN_RANGE_HEAD => true,
            pc => {
                c.jump_out(pc, OC305_VIEW_IN_RANGE_ENTRY, OC305_VIEW_IN_RANGE_END);
                return Ok(());
            }
        };
        let mut found = false;
        loop {
            if !at_head {
                // 0x288cb0: more views?
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                OC305_SSX_AUDIO_GET_LISTENER_COUNT.call(c, 0x0028_8cb8)?;
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A0][0] = c.g[S1][0];
                if event_due(c) && crate::events::back_edge_unwinds(c, OC305_VIEW_IN_RANGE_HEAD) {
                    return Ok(());
                }
            }
            at_head = false;
            // 0x288c48: d = view position - pos; |d|^2 through VU0.
            c.cycles += 21;
            c.g[A1][0] = c.g[S0][0];
            OC305_SSX_AUDIO_GET_VIEW_POS.call(c, 0x0028_8c50)?;
            let view = r32(c, V0);
            let pos = r32(c, S2);
            let spill_hi = frame.at(c, 0x10);
            let spill = frame.at(c, 0);
            c.lqc2(3, view);
            c.lqc2(4, pos);
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, spill_hi);
            c.lq(V0, spill_hi);
            c.sq(V0, spill);
            c.lqc2(3, spill);
            c.lqc2(5, spill);
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.qmfc2(V1, 4);
            c.f[0] = c.g[V1][0] as u32;
            let near = c_lt_s(c, FpuF32(c.f[0]), FpuF32(c.f[20]));
            c.lwc1(0, spill + 8);
            if near {
                // 0x288c9c: in front of the listener?
                c.cycles += 4;
                let ahead = c_lt_s(c, FpuF32(c.f[21]), FpuF32(c.f[0]));
                c.g[V0][0] = 1;
                if ahead {
                    found = true;
                    break;
                }
            }
            // 0x288cac
            c.cycles += 1;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
        }
        if !found {
            // 0x288cc4
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        // 0x288cc8
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 21, 0x68);
        frame.restore_fpr(c, 20, 0x60);
        frame.close(c);
        Ok(())
    })
}

// ------------------------------------------------ SSXAudioSystem_GetViewPos

pub(crate) const VIEW_POS_GET_VIEW: Callee = Callee {
    addr: 0x0028_b210,
    f: ee::s07::SSXAudioSystem_GetView,
};

/// View: +0x20 the camera position (a 4-vector).
pub(crate) const VIEW_POS_OFFSET: u32 = 0x20;

/// `sSndVec4* SSXAudioSystem_GetViewPos(void* self, int index)` (0x288ac0):
/// the position of listener view `index` (`SSXAudioSystem_GetView(index)
/// + 0x20`).
pub fn ssx_audio_get_view_pos(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        VIEW_POS_GET_VIEW.call(c, 0x0028_8ad0)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(VIEW_POS_OFFSET));
        frame.close(c);
        Ok(())
    });
}

// ------------------------------------------ SSXAudioSystem_StopByCallback

/// `SSXAudioSystem` +0x118: pointer to the bank-monitor holder pointer.
pub(crate) const SSX_AUDIO_STOP_BY_CALLBACK_HOLDER: u32 = 0x118;

/// Offset of the bank monitor inside the holder's object.
pub(crate) const SSX_AUDIO_STOP_BY_CALLBACK_MONITOR: u32 = 0x1d8;

/// `void SSXAudioSystem_StopByCallback(SSXAudioSystem* s, Pair p)`
/// (0x00285f80): forward the (64-bit) callback pair to the bank monitor
/// `(**(s+0x118)) + 0x1d8` (0x2ad410), stopping the sounds it matches.
pub fn ssx_audio_system_stop_by_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        // The pair is passed through a stack slot (`sd a1; ld a1`).
        let pair = frame.at(c, 0);
        c.sd(pair, c.g[A1][0]);
        c.g[A1][0] = c.ld(pair);
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(SSX_AUDIO_STOP_BY_CALLBACK_HOLDER)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).lw(c);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(SSX_AUDIO_STOP_BY_CALLBACK_MONITOR));
        call_ptr(c, 0x002a_d410, 0x0028_5fa0)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_PlayStream
// ---------------------------------------------------------------------------

/// `SSXAudioSystem` field: the stream array handle (`StreamArray**`).
pub(crate) const PLAY_STREAM_ARRAY: u32 = 0x118;

/// `StreamArray` fields: the play-params stack (`+0x1d8`) and its top index.
pub(crate) const PLAY_STREAM_PARAMS: u32 = 0x1d8;

pub(crate) const PLAY_STREAM_TOP: u32 = 0x1f4;

/// `(x, y)`: `x = top << shift; y = params[field]; x += y` (the address of
/// element `top` of the per-level array at `params + field`), as the
/// compiled code computes it, `x` and `y` left in those registers.
pub(crate) fn play_stream_slot(c: &mut Cpu, x: usize, y: usize, field: u32, shift: u32) -> u32 {
    c.g[x][0] = c.lw(r32(c, S0).wrapping_add(PLAY_STREAM_TOP));
    c.g[y][0] = c.lw(r32(c, S1).wrapping_add(field));
    c.g[x][0] = sx(r32(c, x) << shift);
    c.g[x][0] = sx(r32(c, x).wrapping_add(r32(c, y)));
    r32(c, x)
}

/// `int SSXAudioSystem_PlayStream(SSXAudioSystem* s, int id, const char*
/// name, int a3, int target, int a5, int a6, int engine, float radius,
/// int a8)` (0x2854f8): queues stream `id` on the stream array; if that
/// worked (handle >= 0), pushes a fresh play-params level (defaults: volume
/// 100/90/50, pan 0x7f, priority 1), sets the vanish radius (`radius`, or
/// 100 / the engine default when `radius <= 0`), records `a6` and the
/// target (and which view it is closer to), marks replay-mirrored engine
/// streams in two-player mode, adds a bank-monitor instance with the stream
/// attached, pops the level and returns the handle.
pub fn ssx_audio_system_play_stream(c: &mut Cpu) {
    guest(c, play_stream_body);
}

pub(crate) fn play_stream_body(c: &mut Cpu) -> Flow {
    c.cycles += 28;
    let frame = Frame::open(c, 0xc0);
    frame.save(c, S2, 0x80);
    frame.save(c, S3, 0x70);
    frame.save(c, S4, 0x60);
    frame.save(c, S5, 0x50);
    frame.save(c, S6, 0x40);
    frame.save(c, S7, 0x30);
    frame.save(c, FP, 0x20);
    frame.save_fpr(c, 20, 0xb0);
    frame.save(c, S0, 0xa0);
    frame.save(c, S1, 0x90);
    frame.save_ra(c, 0x10);
    c.g[S2][0] = c.g[A0][0];
    c.g[S3][0] = c.g[T0][0];
    c.g[S5][0] = c.g[T3][0];
    c.g[S6][0] = c.g[A1][0];
    c.g[S7][0] = c.g[A2][0];
    c.g[FP][0] = c.g[A3][0];
    c.mov_s(20, 12);
    // StreamArray_Queue(*s->array, id, a5, name, a3); a6 on the stack.
    c.g[A2][0] = c.g[T1][0];
    c.g[A3][0] = c.g[S7][0];
    c.g[T0][0] = c.g[FP][0];
    c.g[A0][0] = c.lw(r32(c, S2).wrapping_add(PLAY_STREAM_ARRAY));
    let a6 = c.g[T2][0];
    c.sw(sp(c), a6);
    call_ptr(c, 0x002a_ad78, 0x0028_555c)?;
    c.g[S4][0] = c.g[V0][0];
    c.g[T1][0] = 0x18;
    if (c.g[S4][0] as i64) >= 0 {
        play_stream_push(c)?;
    }
    c.cycles += 13;
    frame.restore(c, S0, 0xa0);
    frame.restore(c, S1, 0x90);
    frame.restore(c, S2, 0x80);
    frame.restore(c, S3, 0x70);
    frame.restore(c, S4, 0x60);
    frame.restore(c, S5, 0x50);
    frame.restore(c, S6, 0x40);
    frame.restore(c, S7, 0x30);
    frame.restore(c, FP, 0x20);
    frame.restore_ra(c, 0x10);
    frame.restore_fpr(c, 20, 0xb0);
    frame.close(c);
    Ok(())
}

/// The queued path: push a level, fill it, add the instance, pop.
pub(crate) fn play_stream_push(c: &mut Cpu) -> Flow {
    c.cycles += 126;
    // s0 = *s->array; s1 = &s0->params; top++.
    c.g[V1][0] = c.lw(r32(c, S2).wrapping_add(PLAY_STREAM_ARRAY));
    c.g[A1][0] = 0;
    c.g[A2][0] = 8;
    c.g[S0][0] = c.lw(r32(c, V1));
    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(PLAY_STREAM_TOP));
    c.g[S1][0] = sx(r32(c, S0).wrapping_add(PLAY_STREAM_PARAMS));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
    let top = c.g[V0][0];
    c.sw(r32(c, S0).wrapping_add(PLAY_STREAM_TOP), top);
    // params->entries[top] (24 bytes each, at params[0x20]) = the defaults
    // at params+4.
    c.g[V1][0] = c.lw(r32(c, S0).wrapping_add(PLAY_STREAM_TOP));
    c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x20));
    c.mult(0, V1, V1, T1);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    let s0 = r32(c, S0);
    c.ldl(V0, s0.wrapping_add(0x1e3));
    c.ldr(V0, s0.wrapping_add(0x1dc));
    c.ldl(A0, s0.wrapping_add(0x1eb));
    c.ldr(A0, s0.wrapping_add(0x1e4));
    c.ldl(A3, s0.wrapping_add(0x1f3));
    c.ldr(A3, s0.wrapping_add(0x1ec));
    let e = r32(c, V1);
    c.sdl(V0, e.wrapping_add(7));
    c.sdr(V0, e);
    c.sdl(A0, e.wrapping_add(0xf));
    c.sdr(A0, e.wrapping_add(8));
    c.sdl(A3, e.wrapping_add(0x17));
    c.sdr(A3, e.wrapping_add(0x10));
    // Target word of the level (s0[0x1fc + 4 top]) and [0x34][top] cleared.
    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(PLAY_STREAM_TOP));
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
    c.sw(r32(c, V0).wrapping_add(0x1fc), 0);
    let a = play_stream_slot(c, V1, V0, 0x34, 2);
    c.sw(a, 0);
    // [0x38][top] = the entry's first byte (its volume, sign-extended).
    c.g[A3][0] = c.lw(r32(c, S0).wrapping_add(PLAY_STREAM_TOP));
    c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(PLAY_STREAM_TOP));
    c.g[T0][0] = c.lw(r32(c, S1).wrapping_add(0x20));
    c.g[A3][0] = sx(r32(c, A3) << 2);
    c.mult(0, A0, A0, T1);
    c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x38));
    c.g[A3][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, T0)));
    c.g[V1][0] = c.lb(r32(c, A0));
    let vol = c.g[V1][0];
    c.sw(r32(c, A3), vol);
    let a = play_stream_slot(c, V0, V1, 0x3c, 2);
    c.sw(a, 0);
    // memset(&params[0x40 + 8 top], 0, 8).
    c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(PLAY_STREAM_TOP));
    c.g[A0][0] = sx(r32(c, A0) << 3);
    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x40));
    call_ptr(c, 0x0041_6210, 0x0028_563c)?;
    // The remaining per-level fields.
    c.g[A0][0] = 0x64;
    c.g[A1][0] = 0x5a;
    c.mtc1(0, 0);
    c.g[A2][0] = 0x32;
    c.g[A3][0] = 0x7f;
    c.c_le_s(20, 0);
    c.g[T0][0] = 1;
    let a = play_stream_slot(c, V1, V0, 0x60, 2);
    c.sw(a, 0);
    let a = play_stream_slot(c, V1, V0, 0x64, 2);
    c.sw(a, 0);
    let a = play_stream_slot(c, V0, V1, 0x68, 2);
    c.swc1(0, a);
    let a = play_stream_slot(c, V1, V0, 0x6c, 2);
    c.swc1(0, a);
    let a = play_stream_slot(c, V0, V1, 0x70, 4);
    c.sw(a.wrapping_add(4), 0);
    let a = play_stream_slot(c, V1, V0, 0x70, 4);
    let v = c.g[A0][0];
    c.sw(a, v);
    let a = play_stream_slot(c, V0, V1, 0x70, 4);
    let v = c.g[A1][0];
    c.sw(a.wrapping_add(8), v);
    let a = play_stream_slot(c, V1, V0, 0x70, 4);
    let v = c.g[A2][0];
    c.sw(a.wrapping_add(0xc), v);
    let a = play_stream_slot(c, V0, V1, 0x74, 2);
    c.sw(a, 0);
    let a = play_stream_slot(c, V1, V0, 0x80, 2);
    c.sw(a, 0);
    let a = play_stream_slot(c, V0, V1, 0x84, 2);
    c.sw(a, 0);
    let a = play_stream_slot(c, V1, V0, 0x78, 2);
    let v = c.g[A3][0];
    c.sw(a, v);
    let a = play_stream_slot(c, V0, V1, 0x7c, 2);
    let v = c.g[T0][0];
    c.sw(a, v);

    // SndPlayParams_SetRadiusVanish(params, radius, -1).
    if !c.fcc() {
        c.cycles += 8;
        c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(PLAY_STREAM_ARRAY));
        c.mov_s(12, 20);
    } else {
        c.cycles += 2;
        c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(PLAY_STREAM_ARRAY));
        c.cycles += 3;
        if c.g[S5][0] == 0 {
            c.g[AT][0] = 0x42c8_0000;
            c.mtc1(AT, 12);
        } else {
            c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x465c));
        }
        c.cycles += 6;
    }
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(AT, 13);
    c.g[A0][0] = c.lw(r32(c, V0));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(PLAY_STREAM_PARAMS));
    let fell = c.fcc();
    call_ptr(c, 0x002a_9988, if fell { 0x0028_5790 } else { 0x0028_57b4 })?;
    c.g[V1][0] = c.lw(r32(c, S2).wrapping_add(PLAY_STREAM_ARRAY));

    // params[0x60][top] = a6 (kept on the stack).
    c.cycles += 8;
    c.g[A0][0] = c.lw(r32(c, V1));
    c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(PLAY_STREAM_TOP));
    c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(0x238));
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    c.g[V1][0] = c.lw(sp(c));
    let v = c.g[V1][0];
    c.sw(r32(c, V0), v);
    if c.g[S3][0] != 0 {
        // target word = target; params[0x64][top] = IsCloserToSecondView.
        c.cycles += 16;
        c.g[A2][0] = c.lw(r32(c, S2).wrapping_add(PLAY_STREAM_ARRAY));
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[V1][0] = c.lw(r32(c, A2));
        c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(PLAY_STREAM_TOP));
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        let target = c.g[S3][0];
        c.sw(r32(c, V1).wrapping_add(0x1fc), target);
        c.g[S0][0] = c.lw(r32(c, S2).wrapping_add(PLAY_STREAM_ARRAY));
        PLAY_STREAM_CLOSER_VIEW.call(c, 0x0028_5800)?;
        c.g[A0][0] = c.lw(r32(c, S0));
        c.g[V1][0] = c.lw(r32(c, A0).wrapping_add(PLAY_STREAM_TOP));
        c.g[A1][0] = c.lw(r32(c, A0).wrapping_add(0x23c));
        c.g[V1][0] = sx(r32(c, V1) << 2);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
        let v = c.g[V0][0];
        c.sw(r32(c, V1), v);
    }

    // Engine streams in two-player mode outside replays: mirrored.
    c.cycles += 7;
    call_ptr(c, 0x0014_dc80, 0x0028_5820)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x0014_dd58, 0x0028_582c)?;
    let have = c.g[V0][0] != 0;
    c.g[V0][0] = 0x53_0000;
    'mode: {
        if have {
            c.cycles += 4;
            c.g[A0][0] = 2;
            c.g[V1][0] = c.lb(0x0053_5c11);
            if c.g[V1][0] != c.g[A0][0] {
                c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(PLAY_STREAM_ARRAY));
                break 'mode;
            }
            c.cycles += 2;
            if c.g[S5][0] == 0 {
                c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(PLAY_STREAM_ARRAY));
                break 'mode;
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S2][0];
            PLAY_STREAM_REPLAY_PLAYING.call(c, 0x0028_5854)?;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(PLAY_STREAM_ARRAY));
                break 'mode;
            }
            // params[0x80][top] = 1; params[0x84][top] = a8.
            c.cycles += 16;
            c.g[A0][0] = c.lw(r32(c, S2).wrapping_add(PLAY_STREAM_ARRAY));
            c.g[A1][0] = 1;
            c.g[V1][0] = c.lw(r32(c, A0));
            c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(PLAY_STREAM_TOP));
            c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x258));
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            c.sw(r32(c, V0), 1);
            c.g[V1][0] = c.lw(r32(c, S2).wrapping_add(PLAY_STREAM_ARRAY));
            c.g[A0][0] = c.lw(r32(c, V1));
            c.g[V1][0] = c.lw(sp(c).wrapping_add(0xc0));
            c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(PLAY_STREAM_TOP));
            c.g[A1][0] = c.lw(r32(c, A0).wrapping_add(0x25c));
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
            let v = c.g[V1][0];
            c.sw(r32(c, V0), v);
        }
        c.cycles += 1;
        c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(PLAY_STREAM_ARRAY));
    }

    // BankMonitor_AddInstance(params, handle, 0x7f, 3); attach the stream.
    c.cycles += 9;
    c.g[A1][0] = c.g[S4][0];
    c.g[A2][0] = 0x7f;
    c.g[A3][0] = 3;
    c.g[A0][0] = c.lw(r32(c, V0));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(PLAY_STREAM_PARAMS));
    call_ptr(c, 0x002a_cfe0, 0x0028_58b8)?;
    c.g[S0][0] = c.g[V0][0];
    if c.g[S0][0] != 0 {
        c.cycles += 9;
        c.g[A0][0] = c.lw(r32(c, S2).wrapping_add(PLAY_STREAM_ARRAY));
        c.g[A1][0] = c.g[S6][0];
        call_ptr(c, 0x002a_b188, 0x0028_58d0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S7][0];
        c.g[A3][0] = c.g[FP][0];
        call_ptr(c, 0x002a_bcf8, 0x0028_58e4)?;
    }
    c.g[V1][0] = c.lw(r32(c, S2).wrapping_add(PLAY_STREAM_ARRAY));

    // Pop the level; return the handle.
    c.cycles += 5;
    c.g[V0][0] = c.g[S4][0];
    c.g[A1][0] = c.lw(r32(c, V1));
    c.g[A0][0] = c.lw(r32(c, A1).wrapping_add(PLAY_STREAM_TOP));
    c.g[A0][0] = sx(r32(c, A0).wrapping_sub(1));
    let v = c.g[A0][0];
    c.sw(r32(c, A1).wrapping_add(PLAY_STREAM_TOP), v);
    Ok(())
}

pub(crate) const PLAY_STREAM_CLOSER_VIEW: Callee = Callee {
    addr: 0x0028_8940,
    f: ee::s07::SSXAudioSystem_IsCloserToSecondView,
};

pub(crate) const PLAY_STREAM_REPLAY_PLAYING: Callee = Callee {
    addr: 0x0028_8ae0,
    f: ee::s07::SSXAudioSystem_IsReplayPlaying,
};

/// `void SSXAudioSystem_SetupChannelGains(AudioSystem* self, int mode)` (0x288d18):
/// fill the per-kind volume scaling globals (gp+0x414..0x460: SFX, music,
/// speech, board, ambient, per-character...) for the mix `mode` (0: all 1.0
/// except the character speech 0.6; 1: the tuned in-race mix; others leave
/// them as they are), then re-apply every one of the 11 channel volumes with
/// `SetChannelVolume(self, ch, GetChannelVolume(self, ch))`.
pub fn ssx_audio_system_setup_channel_gains(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x20 };
        'b_288e1c: {
            'b_288e18: {
                'b_288d8c: {
                    c.cycles += 5;
                    Frame::open(c, 0x20);
                    frame.save(c, S0, 0x10);
                    frame.save_ra(c, 0);
                    let t = c.g[A1][0] != 0u64;
                    c.g[S0][0] = c.g[A0][0];
                    if t {
                        break 'b_288d8c;
                    } else {
                        c.cycles += 24;
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 0);
                        c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4610));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x460));
                        c.swc1(1, (c.g[GP][0] as u32).wrapping_add(0x41c));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x414));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x418));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x420));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x424));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x428));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x42c));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x430));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x434));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x438));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x43c));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x440));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x444));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x448));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x44c));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x450));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x454));
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x458));
                        let t = true;
                        c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x45c));
                        if t {
                            break 'b_288e18;
                        } else {
                            break 'b_288d8c;
                        }
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = 0x1;
                let t = c.g[A1][0] != c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                if t {
                    break 'b_288e1c;
                } else {
                    c.cycles += 32;
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x460c));
                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x4608));
                    c.g[AT][0] = 0x3f800000;
                    c.mtc1(1, 6);
                    c.g[AT][0] = 0x3f400000;
                    c.mtc1(1, 2);
                    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4604));
                    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4600));
                    c.g[AT][0] = 0x3f000000;
                    c.mtc1(1, 5);
                    c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x45fc));
                    c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x45f8));
                    c.swc1(3, (c.g[GP][0] as u32).wrapping_add(0x420));
                    c.swc1(4, (c.g[GP][0] as u32).wrapping_add(0x424));
                    c.swc1(5, (c.g[GP][0] as u32).wrapping_add(0x430));
                    c.swc1(2, (c.g[GP][0] as u32).wrapping_add(0x438));
                    c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x44c));
                    c.swc1(7, (c.g[GP][0] as u32).wrapping_add(0x450));
                    c.swc1(1, (c.g[GP][0] as u32).wrapping_add(0x454));
                    c.swc1(6, (c.g[GP][0] as u32).wrapping_add(0x45c));
                    c.swc1(8, (c.g[GP][0] as u32).wrapping_add(0x460));
                    c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x414));
                    c.swc1(6, (c.g[GP][0] as u32).wrapping_add(0x418));
                    c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x41c));
                    c.swc1(2, (c.g[GP][0] as u32).wrapping_add(0x428));
                    c.swc1(0, (c.g[GP][0] as u32).wrapping_add(0x42c));
                    c.swc1(2, (c.g[GP][0] as u32).wrapping_add(0x434));
                    c.swc1(1, (c.g[GP][0] as u32).wrapping_add(0x43c));
                    c.swc1(1, (c.g[GP][0] as u32).wrapping_add(0x440));
                    c.swc1(1, (c.g[GP][0] as u32).wrapping_add(0x444));
                    c.swc1(1, (c.g[GP][0] as u32).wrapping_add(0x448));
                    c.swc1(6, (c.g[GP][0] as u32).wrapping_add(0x458));
                    break 'b_288e18;
                }
            }
            c.cycles += 1;
            c.g[A0][0] = c.g[S0][0];
            break 'b_288e1c;
        }
        c.cycles += 80;
        // Re-apply every channel's set volume: SetChannelVolume(self, ch,
        // GetChannelVolume(self, ch)) for the eleven channels, so the new
        // scaling factors reach the derived per-channel gains.
        for channel in 0..=10u32 {
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = channel as u64;
            call_ptr(c, 0x0028_7920, 0x0028_8e24 + 0x1c * channel)?;
            c.mov_s(12, 0);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = channel as u64;
            call_ptr(c, 0x0028_7700, 0x0028_8e34 + 0x1c * channel)?;
        }
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        return Ok(());
    });
}

/// `void SSXAudioSystem_EnterGame(SSXAudioSystem* this)` (0x2867e8).
///
/// Starts the in-game audio: sets the radio mode and playlist from the back
/// end's reward interface, starts a song (`SelectRandomSong` + `PlayMusic`)
/// unless one is already playing (id 401), clears the rider air flags, fetches
/// the five world-painter queries (music trigger x2, ambience x2, mix x2) into
/// the system (`+0x625c`, `+0x6260`, `+0x5fa0`, `+0x5fa4`, `+0x6c80`,
/// `+0x6c84`), resets the fade/ducking fields, fills the 23 "first time on
/// track" flags at `+0x579c` (when the audio state byte at 0x535c11 is 0) and
/// finally resets the music state, runs `EnterGameInit` and 0x2a4a78.
pub fn ssx_audio_system_enter_game(c: &mut Cpu) {
    guest(c, |c| {
        'b_2868f4: {
            'b_28688c: {
                c.cycles += 36;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[S4][0] = c.g[A0][0];
                call_ptr(c, 0x0014dc80, 0x00286810)?;
                c.g[A1][0] = 0xd;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014dd58, 0x0028681c)?;
                c.g[S2][0] = c.g[V0][0];
                call_ptr(c, 0x0028b1d8, 0x00286824)?;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                call_ptr(c, 0x00123128, 0x0028682c)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = 0u64;
                c.g[A2][0] = c.g[S0][0];
                call_ptr(c, 0x00158750, 0x00286840)?;
                c.g[A2][0] = c.g[S0][0];
                c.g[S1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x00158700, 0x00286854)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S4][0];
                call_ptr(c, 0x0028bf78, 0x00286860)?;
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x0028c2d0, 0x0028686c)?;
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x530));
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x118));
                if t {
                    break 'b_28688c;
                } else {
                    c.cycles += 5;
                    c.g[S5][0] = 0x530000;
                    call_ptr(c, 0x002b49e0, 0x00286880)?;
                    c.g[V1][0] = 0x191;
                    let t = c.g[V0][0] == c.g[V1][0];

                    if t {
                        break 'b_2868f4;
                    } else {
                        break 'b_28688c;
                    }
                }
            }
            'b_2868dc: {
                c.cycles += 12;
                c.g[A0][0] = c.g[S4][0];
                call_ptr(c, 0x0028d488, 0x00286894)?;
                call_ptr(c, 0x0014dc80, 0x0028689c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x002868a8)?;
                c.g[V1][0] = 0x530000;
                c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
                c.g[V0][0] = c.lb((c.g[A0][0] as u32).wrapping_add(0x49));
                let t = c.g[V0][0] != 0u64;
                c.g[S5][0] = c.g[V1][0];
                if t {
                    break 'b_2868dc;
                } else {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0xffffffffffffffff;
                    c.g[T0][0] = 0u64;
                    call_ptr(c, 0x0028cf98, 0x002868d4)?;
                    let t = true;

                    if t {
                        break 'b_2868f4;
                    } else {
                        break 'b_2868dc;
                    }
                }
            }
            c.cycles += 6;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = 0x24;
            c.g[A2][0] = 0u64;
            c.g[A3][0] = 0xffffffffffffffff;
            c.g[T0][0] = 0u64;
            call_ptr(c, 0x0028cf98, 0x002868f4)?;
            break 'b_2868f4;
        }
        'b_286a44: {
            c.cycles += 59;
            c.g[S1][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2178));
            c.g[A0][0] = c.g[S4][0];
            call_ptr(c, 0x002948a0, 0x00286900)?;
            c.g[S2][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2188));
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A1][0] = c.g[S1][0];
            c.g[S3][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2180));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002c0a58, 0x00286924)?;
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S0][0];
            c.sw((c.g[S4][0] as u32).wrapping_add(0x625c), c.g[V0][0]);
            call_ptr(c, 0x002c0a58, 0x00286934)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S2][0];
            c.sw((c.g[S4][0] as u32).wrapping_add(0x6260), c.g[V0][0]);
            call_ptr(c, 0x002c0a58, 0x00286944)?;
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.g[S0][0];
            c.sw((c.g[S4][0] as u32).wrapping_add(0x5fa0), c.g[V0][0]);
            call_ptr(c, 0x002c0a58, 0x00286954)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S3][0];
            c.sw((c.g[S4][0] as u32).wrapping_add(0x5fa4), c.g[V0][0]);
            call_ptr(c, 0x002c0a58, 0x00286964)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S3][0];
            c.sw((c.g[S4][0] as u32).wrapping_add(0x6c80), c.g[V0][0]);
            call_ptr(c, 0x002c0a58, 0x00286974)?;
            c.g[A2][0] = 0xffffffffffffffff;
            c.g[V1][0] = 0x32;
            c.g[A3][0] = 0x1;
            c.g[AT][0] = 0x3f000000;
            c.mtc1(1, 12);
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 13);
            c.g[A0][0] = c.g[S4][0];
            c.sw((c.g[S4][0] as u32).wrapping_add(0x626c), 0u64);
            c.g[A1][0] = 0x3;
            c.sw((c.g[S4][0] as u32).wrapping_add(0x6274), 0u64);
            c.sw((c.g[S4][0] as u32).wrapping_add(0x5fc4), 0u64);
            c.sb((c.g[S4][0] as u32).wrapping_add(0x5fc0), c.g[V1][0]);
            c.sw((c.g[S4][0] as u32).wrapping_add(0x6268), c.g[A2][0]);
            c.sw((c.g[S4][0] as u32).wrapping_add(0x6270), c.g[A3][0]);
            c.sw((c.g[S4][0] as u32).wrapping_add(0x6264), c.g[A2][0]);
            c.sw((c.g[S4][0] as u32).wrapping_add(0x6c84), c.g[V0][0]);
            call_ptr(c, 0x00287f00, 0x002869bc)?;
            c.sw((c.g[S4][0] as u32).wrapping_add(0x6288), 0u64);
            call_ptr(c, 0x0014dc80, 0x002869c4)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014dd58, 0x002869d0)?;
            c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x5bc8));
            c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
            let t = c.g[V0][0] != 0u64;
            c.g[S2][0] = 0u64;
            if t {
                break 'b_286a44;
            } else {
                c.cycles += 2;
                c.g[S1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x579c));
                'l_2869e8: loop {
                    c.cycles += 23;
                    call_ptr(c, 0x0014dc80, 0x002869f0)?;
                    c.g[A1][0] = 0x1;
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0014dd58, 0x002869fc)?;
                    c.g[S0][0] = c.g[V0][0];
                    call_ptr(c, 0x0028b1d8, 0x00286a04)?;
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
                    c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
                    let t = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                    call_ptr(c, t, 0x00286a20)?;
                    c.g[A2][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = c.g[V0][0];
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    call_ptr(c, 0x00145d38, 0x00286a34)?;
                    c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                    c.g[V0][0] = ((c.g[S2][0] as i64) < 23i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x002869e8);
                        }
                        continue 'l_2869e8;
                    } else {
                        break 'b_286a44;
                    }
                }
            }
        }
        c.cycles += 15;
        c.g[A0][0] = c.g[S4][0];
        call_ptr(c, 0x0028df08, 0x00286a4c)?;
        c.g[A0][0] = c.g[S4][0];
        call_ptr(c, 0x00286e20, 0x00286a54)?;
        c.g[A0][0] = c.g[S4][0];
        call_ptr(c, 0x002a4a78, 0x00286a5c)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

/// `void SSXAudioSystem_LoadFrontEndAudio(SSXAudioSystem* this)` (0x285fb0,
/// frame 0xa20; the reference's `cSSXAudio_FrontEndLoad`): sets up the channel
/// gains and audio mode 1, opens the audio ini (`IniParser_ctor`, 0x2a77f8,
/// path from `gp+0x464`). It scans the `GLOBAL` section for its `BASEPATH`
/// (loop 1, `strcpy` of it), then the `FE` section: for each key equal to
/// `MAIN` (`strcasecmp`) it formats `%s%s%s.b` and loads that bank
/// synchronously (`SndBankMan_LoadBank`, slot 0). Then it ducks to the stream
/// (`SetFadeMode(3, 0.5, 1.0)`), sets the channel gain through the music
/// system's vtable entry 2 (`charsel`), sends message 301 (`0x2b49c0`), stores
/// 10 at +0x6290 and the radio mode at +0x6090, resumes the audio and
/// destroys the parser. Both key loops are native loops with the event check
/// at the back edge; a resume at a block head (`c.pc != entry`) falls back to
/// the generated dispatcher.
pub fn ssx_audio_system_load_front_end_audio(c: &mut Cpu) {
    if c.pc != 0x0028_5fb0 {
        return ee::s07::f_00285fb0__generated(c);
    }
    guest(c, |c| {
        c.cycles += 28;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa20));
        c.g[A1][0] = 0u64;
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x9e0));
        c.g[S3][0] = c.g[A0][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa10));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x9c0));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xa00));
        c.g[S0][0] = 0x4a0000;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x9f0));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x9d0));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x9b0), c.g[RA][0]);
        c.g[RA][0] = sx(0x00285fe4);
        c.g[S5][0] = 0x4a0000;
        call_ptr(c, 0x00288d18, 0x00285fe4)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[RA][0] = sx(0x00285ff0);
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x00285be0, 0x00285ff0)?;
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x464));
        c.g[A0][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x00286000);
        c.g[A2][0] = 0x100;
        call_ptr(c, 0x002a77f8, 0x00286000)?;
        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
        c.g[A1][0] = 0x4a0000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3570));
        c.g[A0][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x00286018);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        call_ptr(c, 0x002a7bf8, 0x00286018)?;
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
        'b_286054: {
            if t {
                break 'b_286054;
            }
            c.cycles += 2;
            'l_286044: loop {
                c.cycles += 4;
                c.g[RA][0] = sx(0x0028604c);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x002a7f90, 0x0028604c)?;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                if !t {
                    break 'l_286044;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x00286028);
                }
                c.cycles += 7;
                c.g[RA][0] = sx(0x00286030);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x002a7da0, 0x00286030)?;
                c.g[A1][0] = 0x480000;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x28c0));
                c.g[RA][0] = sx(0x00286044);
                c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x810));
                call_ptr(c, 0x002a7ff8, 0x00286044)?;
            }
        }
        c.cycles += 7;
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3578));
        c.g[A0][0] = c.g[SP][0];
        c.g[S4][0] = sx((c.g[S3][0] as u32).wrapping_add(0x118));
        c.g[RA][0] = sx(0x00286068);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x002a7bf8, 0x00286068)?;
        let t = c.g[V0][0] == 0u64;
        'b_286120: {
            if t {
                break 'b_286120;
            }
            c.cycles += 6;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x890));
            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x810));
            c.g[RA][0] = sx(0x00286080);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x004166f4, 0x00286080)?;
            c.g[S2][0] = c.g[S0][0];
            'l_286110: loop {
                c.cycles += 4;
                c.g[RA][0] = sx(0x00286118);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x002a7f90, 0x00286118)?;
                let t = c.g[V0][0] == 0u64;
                if !t {
                    break 'l_286110;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x00286088);
                }
                c.cycles += 15;
                c.g[RA][0] = sx(0x00286090);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x002a7da0, 0x00286090)?;
                c.g[A1][0] = 0x480000;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x28c0));
                c.g[A0][0] = c.g[SP][0];
                c.g[RA][0] = sx(0x002860a8);
                c.g[A2][0] = c.g[S2][0];
                call_ptr(c, 0x002a7ff8, 0x002860a8)?;
                c.g[V1][0] = 0x4a0000;
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3580));
                c.g[RA][0] = sx(0x002860bc);
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x0041aa88, 0x002860bc)?;
                let t = c.g[V0][0] != 0u64;
                c.g[A1][0] = c.g[S1][0];
                if !t {
                    c.cycles += 19;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x910));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x002860d4);
                    c.g[A2][0] = c.g[S0][0];
                    call_ptr(c, 0x002a7ff8, 0x002860d4)?;
                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x970));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A2][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3588));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3590));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A3][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x002860f8);
                    c.g[T0][0] = c.g[S0][0];
                    call_ptr(c, 0x00417828, 0x002860f8)?;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
                    c.g[A2][0] = c.g[S1][0];
                    c.g[A1][0] = 0u64;
                    c.g[A3][0] = 0u64;
                    c.g[RA][0] = sx(0x00286110);
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    call_ptr(c, 0x0028bc58, 0x00286110)?;
                }
            }
        }
        c.cycles += 43;
        c.g[AT][0] = 0x3f000000;
        c.mtc1(1, 12);
        c.g[A0][0] = c.g[S3][0];
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 13);
        c.g[A1][0] = 0x3;
        c.g[RA][0] = sx(0x00286140);
        c.g[S2][0] = sx((c.g[S5][0] as u32).wrapping_add(0x3598));
        call_ptr(c, 0x00287f00, 0x00286140)?;
        c.g[S1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x5558));
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 0x1;
        c.g[A2][0] = 0u64;
        c.g[S0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = sx(0x0028615c);
        c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[S0][0] as u32));
        call_ptr(c, 0x00287968, 0x0028615c)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
        c.g[A3][0] = c.g[V0][0];
        c.g[A2][0] = 0xc;
        c.g[T0][0] = 0x1;
        c.g[A0][0] = c.g[S0][0];
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x00286178);
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, t, 0x00286178)?;
        c.g[A0][0] = c.g[S4][0];
        c.g[RA][0] = sx(0x00286184);
        c.g[A1][0] = 0x12d;
        call_ptr(c, 0x002b49c0, 0x00286184)?;
        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x608c));
        c.g[V0][0] = 0xa;
        c.g[A0][0] = c.g[S3][0];
        c.sw((c.g[S3][0] as u32).wrapping_add(0x6290), c.g[V0][0]);
        c.g[RA][0] = sx(0x0028619c);
        c.sw((c.g[S3][0] as u32).wrapping_add(0x6090), c.g[V1][0]);
        call_ptr(c, 0x00289bb8, 0x0028619c)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x002861a8);
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x002a7890, 0x002861a8)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa10));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xa00));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x9f0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x9e0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x9d0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x9c0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x9b0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa20));
        c.pc = t;
        return Ok(());
    });
}

/// `void SSXAudioSystem_LoadPlaylist(SSXAudioSystem* this, const char* name)` (0x289520).
///
/// Clears the music system (`this + 0x118`), parses the playlist file's
/// section `name` and adds every `SONG` key to it (`func_002B3EE8`), then
/// stores the playlist's index among the 20 names (`+0x60a4`, 16 bytes each;
/// 20 when not found) at `+0x6238`, finishes the music system, copies its
/// active map (`+0x510 -> +0x518`) and destroys the parser.
pub fn ssx_audio_system_load_playlist(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x8b0));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x880));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x8a0));
        c.g[S2][0] = c.g[A0][0];
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x860));
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x118));
        c.g[S4][0] = c.g[A1][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x890));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x870));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x850), c.g[RA][0]);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002b3eb0, 0x00289550)?;
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x46c));
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = 0x100;
        call_ptr(c, 0x002a77f8, 0x00289560)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = c.g[S4][0];
        call_ptr(c, 0x002a7c68, 0x0028956c)?;
        let not_found = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0x14;
        // 0x2895d8 is the "name search" stage; the song scan runs only when
        // the section was found.
        if !not_found {
            // 0x289574
            c.cycles += 4;
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, 0x002a7a20, 0x0028957c)?;
            c.g[S3][0] = c.g[S0][0];
            // 0x2895c8: loop head; the body (0x289588) runs while !IsEOF.
            loop {
                c.cycles += 4;
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x002a7f90, 0x002895d0)?;
                let eof = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0x14;
                if eof {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x00289588);
                }
                // 0x289588: next key name; `SONG` keys assign the song.
                c.cycles += 9;
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x002a7da0, 0x00289590)?;
                c.g[V1][0] = 0x4a0000;
                c.g[A1][0] = c.g[V0][0];
                c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3608));
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0041aa88, 0x002895a4)?;
                let not_song = c.g[V0][0] != 0u64;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x810));
                if !not_song {
                    // 0x2895ac
                    c.cycles += 7;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A2][0] = c.g[S0][0];
                    call_ptr(c, 0x002a7ff8, 0x002895bc)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, 0x002b3ee8, 0x002895c8)?;
                }
            }
        }
        // 0x2895d8: playlist index = 20 ("not found"), then search the names.
        c.cycles += 4;
        c.g[S0][0] = 0u64;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x6238), c.g[V0][0]);
        c.g[S3][0] = sx((c.g[S2][0] as u32).wrapping_add(0x118));
        loop {
            // 0x2895e8
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 20i64) as u64;
            let done = c.g[V0][0] == 0u64;
            c.g[A0][0] = 0x14;
            if done {
                break;
            }
            // 0x2895f4
            c.cycles += 7;
            c.g[A1][0] = c.g[S4][0];
            c.mult(0, 4, 16, 4);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60a4));
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, 0x004165a8, 0x00289608)?;
            if c.g[V0][0] != 0u64 {
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                if event_due(c) {
                    deliver_event_here(c, 0x002895e8);
                }
                continue;
            }
            // 0x289610: match: remember the index.
            c.cycles += 1;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x6238), c.g[S0][0]);
            break;
        }
        // 0x289614
        c.cycles += 15;
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, 0x002b4060, 0x0028961c)?;
        c.g[V0][0] = c.ld((c.g[S2][0] as u32).wrapping_add(0x510));
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 0x2;
        c.sd((c.g[S2][0] as u32).wrapping_add(0x518), c.g[V0][0]);
        call_ptr(c, 0x002a7890, 0x00289630)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x8a0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x890));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x880));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x870));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x860));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x850));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x8b0));
        c.pc = t;
        return Ok(());
    })
}

/// `void SSXAudioSystem_ResetRaceSounds(SSXAudioSystem* self)` (0x2871b0): clears the rider
/// air flags, sets the fade mode, and per race-manager entry calls the two reset hooks;
/// then fades out the monitor with the stored fade time.
pub fn ssx_audio_system_reset_race_sounds(c: &mut Cpu) {
    guest(c, |c| {
        'b_28725c: {
            c.cycles += 26;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S1][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[RA][0] = sx(0x002871d4);
            c.g[S2][0] = 0u64;
            Callee {
                addr: 0x002948a0,
                f: ee::s07::SSXAudioSystem_ClearRiderAirFlags,
            }
            .call(c, 0x002871d4)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x002871e0);
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0029b3c0, 0x002871e0)?;
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 13);
            c.g[A1][0] = 0x3;
            c.g[AT][0] = 0x3f000000;
            c.mtc1(1, 12);
            c.g[RA][0] = sx(0x002871fc);
            c.g[A0][0] = c.g[S1][0];
            Callee {
                addr: 0x00287f00,
                f: ee::s07::SSXAudioSystem_SetFadeMode,
            }
            .call(c, 0x002871fc)?;
            c.g[RA][0] = sx(0x00287204);
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x21e4));
            call_ptr(c, 0x002b6900, 0x00287204)?;
            c.g[RA][0] = sx(0x0028720c);
            Callee {
                addr: 0x0028b1d8,
                f: ee::s07::SSXAudioSystem_GetRaceMan,
            }
            .call(c, 0x0028720c)?;
            c.g[S3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
            if (c.g[S3][0] as i64) <= 0 {
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x59e4));
                break 'b_28725c;
            } else {
                'l_287218: loop {
                    c.cycles += 16;
                    c.g[RA][0] = sx(0x00287220);
                    c.g[S0][0] = sx((c.g[S2][0] as u32) << 2);
                    Callee {
                        addr: 0x0028b1d8,
                        f: ee::s07::SSXAudioSystem_GetRaceMan,
                    }
                    .call(c, 0x00287220)?;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                    c.g[RA][0] = sx(0x00287234);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    call_ptr(c, 0x00296e20, 0x00287234)?;
                    c.g[RA][0] = sx(0x0028723c);
                    Callee {
                        addr: 0x0028b1d8,
                        f: ee::s07::SSXAudioSystem_GetRaceMan,
                    }
                    .call(c, 0x0028723c)?;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0028724c);
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                    call_ptr(c, 0x00297438, 0x0028724c)?;
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S3][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00287218);
                        }
                        continue 'l_287218;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x59e4));
                        break 'b_28725c;
                    }
                }
            }
        }
        'b_287288: {
            c.cycles += 3;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x6074), 0u64);
            let t = (c.g[V0][0] as i64) <= 0;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x6078), 0u64);
            if t {
                break 'b_287288;
            } else {
                c.cycles += 8;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x118));
                c.g[A2][0] = 0x1;
                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x59e0));
                c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                c.lwc1(12, (c.g[S1][0] as u32).wrapping_add(0x6074));
                c.g[RA][0] = sx(0x00287284);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                call_ptr(c, 0x002ad5f0, 0x00287284)?;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x59e4), 0u64);
                break 'b_287288;
            }
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `void SSXAudioSystem_RiderInitStub(void)` (0x289af0): empty.
pub fn ssx_audio_system_rider_init_stub(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cSSXAudio::cSSXAudio(int, int, eSOUNDDEF_MODE)`: constructs the audio system (bank manager, stream array, controller, vtable patches when `a1` is set), clears the channel/volume state, sets the default channel volume, reads the playlist and volume config and loads the loading-screen bank.
/// Hand port of the generated body (SSXAudioSystem_ctor, 0x00284c68), every register, cycle and
/// call kept; any other `pc` (a resume after an unwound callee) runs the generated body.
pub fn ssx_audio_system_ctor(c: &mut Cpu) {
    if c.pc != 0x00284c68 {
        return ee::s07::f_00284c68__generated(c);
    }
    guest(c, |c| {
        'b_284d00: {
            c.cycles += 16;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x130));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x110));
            c.g[S0][0] = c.g[A2][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x100));
            c.g[S1][0] = c.g[A0][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xf0));
            c.g[S2][0] = c.g[A3][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
            c.g[S3][0] = c.g[A1][0];
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xd0));
            c.g[S4][0] = c.g[T0][0];
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xc0));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0xb0));
            let t = c.g[S3][0] == 0u64;
            c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[RA][0]);
            if t {
                break 'b_284d00;
            } else {
                c.cycles += 22;
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6c98));
                c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6c90));
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x7768));
                c.g[A0][0] = c.g[A2][0];
                c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                c.g[A1][0] = c.g[S4][0];
                c.sw((c.g[S1][0] as u32).wrapping_add(0x7768), c.g[V1][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6e70), c.g[A2][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x118), c.g[V0][0]);
                c.g[RA][0] = sx(0x00284cd4);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x5560), c.g[V0][0]);
                call_ptr(c, 0x002ade88, 0x00284cd4)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7768));
                c.g[A1][0] = 0u64;
                c.g[A2][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00284ce8);
                c.g[A3][0] = c.g[S4][0];
                Callee {
                    addr: 0x0028b9b8,
                    f: ee::s07::SndBankMan_ctor,
                }
                .call(c, 0x00284ce8)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x5560));
                c.g[A1][0] = 0u64;
                c.g[A2][0] = c.g[S0][0];
                c.g[A3][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x00284d00);
                c.g[T0][0] = c.g[S4][0];
                call_ptr(c, 0x002aa490, 0x00284d00)?;
                break 'b_284d00;
            }
        }
        'b_284ea0: {
            c.cycles += 23;
            c.g[A3][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0u64;
            c.g[RA][0] = sx(0x00284d18);
            c.g[T0][0] = c.g[S4][0];
            call_ptr(c, 0x002ab200, 0x00284d18)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x118));
            c.g[A1][0] = 0x480000;
            c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3708));
            c.g[A3][0] = 0x480000;
            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
            c.g[T8][0] = sx((c.g[A3][0] as u32).wrapping_add(0x3648));
            c.g[S2][0] = 0x480000;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1d8));
            c.g[T9][0] = sx((c.g[S2][0] as u32).wrapping_add(0x3728));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x4), c.g[A2][0]);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x118));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            c.sw((c.g[V1][0] as u32).wrapping_add(0xab0), c.g[T8][0]);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x118));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            let t = c.g[S3][0] != 0u64;
            c.sw((c.g[V1][0] as u32).wrapping_add(0x1d4), c.g[T9][0]);
            if t {
                break 'b_284ea0;
            } else {
                c.cycles += 81;
                c.g[V1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[T6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[A1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x3708));
                c.g[V0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                c.sd(c.g[SP][0] as u32, c.g[A1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x118));
                c.g[T5][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x3648));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.g[A1][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
                c.g[A0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x8));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[SP][0]);
                c.g[T4][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x8));
                c.g[T2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x118));
                c.g[T3][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x10));
                c.g[V1][0] = c.lw(c.g[T2][0] as u32);
                c.g[T1][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x18));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
                c.g[T0][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6c90));
                c.g[A3][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x28));
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[A2][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x30));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x38));
                c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[T5][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x28), c.g[T4][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[T3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x38), c.g[T1][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[T0][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x48), c.g[A3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V1][0]);
                c.g[V0][0] = c.lw(c.g[T2][0] as u32);
                c.g[A3][0] = c.lhu((c.g[T8][0] as u32).wrapping_add(0x30));
                c.sw((c.g[V0][0] as u32).wrapping_add(0xab0), c.g[T6][0]);
                c.g[A2][0] = c.lhu((c.g[T8][0] as u32).wrapping_add(0x8));
                c.g[T7][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x118));
                c.g[A1][0] = c.lhu((c.g[T8][0] as u32).wrapping_add(0x10));
                c.g[V1][0] = c.lw(c.g[T7][0] as u32);
                c.g[A0][0] = c.lhu((c.g[T8][0] as u32).wrapping_add(0x18));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6c98));
                c.g[V0][0] = c.lhu((c.g[T8][0] as u32).wrapping_add(0x28));
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                c.g[T6][0] = c.ld((c.g[S2][0] as u32).wrapping_add(0x3728));
                c.g[T0][0] = c.ld((c.g[T9][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[T5][0] = c.ld((c.g[T9][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[T4][0] = c.ld((c.g[T9][0] as u32).wrapping_add(0x18));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[T3][0] = c.ld((c.g[T9][0] as u32).wrapping_add(0x20));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[T2][0] = c.ld((c.g[T9][0] as u32).wrapping_add(0x28));
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[T1][0] = c.ld((c.g[T9][0] as u32).wrapping_add(0x30));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x68), c.g[T0][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x48), c.g[V0][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x28), c.g[A2][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x30), c.g[A1][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x38), c.g[A0][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[T6][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[T5][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x78), c.g[T4][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[T3][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x88), c.g[T2][0]);
                c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[T1][0]);
                c.g[V0][0] = c.lhu((c.g[T9][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lw(c.g[T7][0] as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sw((c.g[A0][0] as u32).wrapping_add(0x1d4), c.g[S0][0]);
                c.sh((c.g[SP][0] as u32).wrapping_add(0x68), c.g[V0][0]);
                break 'b_284ea0;
            }
        }
        c.cycles += 72;
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.g[V1][0] = 0x480000;
        c.g[T0][0] = 0x480000;
        c.g[T1][0] = 0x480000;
        c.g[V0][0] = 0xffffffffffffffff;
        c.g[T2][0] = 0x1;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3688));
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x36d8));
        c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x36b8));
        c.g[T3][0] = 0xc8;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5728), 0u64);
        c.g[A1][0] = 0x480000;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5734), 0u64);
        c.g[A3][0] = 0u64;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5740), 0u64);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2820));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5744), 0u64);
        c.g[A0][0] = 0x320;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x574c), 0u64);
        c.g[A2][0] = 0u64;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5750), 0u64);
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x60a4));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5754), 0u64);
        c.g[S5][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6c70));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5758), 0u64);
        c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6c78));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5768), 0u64);
        c.g[S6][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6030));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x576c), 0u64);
        c.g[S7][0] = sx((c.g[S1][0] as u32).wrapping_add(0x579c));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5770), 0u64);
        c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5560));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5774), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x577c), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5784), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5788), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x578c), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5818), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5820), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5828), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5984), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5988), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5558), c.g[V1][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x571c), c.g[T0][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[T1][0]);
        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x5730));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5794), c.g[T2][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x581c), c.g[V0][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x597c), c.g[T3][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x572c), c.g[V0][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5764), c.g[V0][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5780), c.g[T2][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5790), c.g[T2][0]);
        c.g[RA][0] = sx(0x00284f80);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5798), c.g[V0][0]);
        call_ptr(c, 0x00317e30, 0x00284f80)?;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5980), c.g[V0][0]);
        c.g[T0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6394));
        c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x63e4));
        c.g[V1][0] = 0xffffffffffffffff;
        c.g[V0][0] = 0x1d;
        c.g[V0][0] = 0xffffffffffff0000;
        c.g[V1][0] = 0xffffffffffffffff;
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5fdc), c.g[V1][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5fc0), c.g[V0][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5fc8), c.g[V1][0]);
        c.g[V0][0] = 0x2;
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5fd4), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5fd8), 0u64);
        'l_284fc0: loop {
            c.cycles += 7;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            let t = c.g[V0][0] != c.g[V1][0];
            c.g[A0][0] = 0xffffffffffffffff;
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x00284fc0);
                }
                continue 'l_284fc0;
            } else {
                c.cycles += 37;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.g[A2][0] = 0x1;
                c.g[V1][0] = 0x17;
                c.g[V0][0] = 0x3;
                c.g[A1][0] = 0x3c;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6284), c.g[V1][0]);
                c.mov_s(1, 0);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x63d4), c.g[V0][0]);
                c.g[V1][0] = c.g[A3][0];
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6c54), c.g[A1][0]);
                c.g[V0][0] = c.g[T0][0];
                c.sw((c.g[S1][0] as u32).wrapping_add(0x646c), c.g[A0][0]);
                c.g[A1][0] = 0xa;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6c88), c.g[A2][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x62b0), c.g[S4][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x603c), c.g[A0][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6040), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x608c), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6090), 0u64);
                c.sd((c.g[S1][0] as u32).wrapping_add(0x6098), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6254), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x627c), c.g[A2][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6294), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6298), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x62a4), c.g[A0][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x62a8), c.g[A0][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x62b4), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x62b8), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x63c0), 0u64);
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x63d0));
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x63d8));
                c.sw((c.g[S1][0] as u32).wrapping_add(0x63dc), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6474), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6c58), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6c60), c.g[A2][0]);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6250), 0u64);
                'l_285070: loop {
                    c.cycles += 7;
                    c.swc1(1, c.g[V0][0] as u32);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    c.swc1(1, c.g[V1][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                    let t = (c.g[A1][0] as i64) >= 0;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00285070);
                        }
                        continue 'l_285070;
                    } else {
                        c.cycles += 27;
                        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x4660));
                        c.g[A0][0] = c.g[S1][0];
                        c.g[RA][0] = sx(0x0028509c);
                        c.g[A1][0] = 0u64;
                        Callee {
                            addr: 0x00287700,
                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                        }
                        .call(c, 0x0028509c)?;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x6470), 0u64);
                        c.g[A1][0] = 0u64;
                        c.g[RA][0] = sx(0x002850ac);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x00288f60,
                            f: ee::s07::SSXAudioSystem_SetDebugMode,
                        }
                        .call(c, 0x002850ac)?;
                        c.g[RA][0] = sx(0x002850b4);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x00289470,
                            f: ee::s07::SSXAudioSystem_ReadPlaylistNames,
                        }
                        .call(c, 0x002850b4)?;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x002850c0);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x00289520,
                            f: ee::s07::SSXAudioSystem_LoadPlaylist,
                        }
                        .call(c, 0x002850c0)?;
                        c.g[RA][0] = sx(0x002850c8);
                        c.g[A0][0] = c.g[S1][0];
                        Callee {
                            addr: 0x00287fc8,
                            f: ee::s07::SSXAudioSystem_LoadVolumeConfig,
                        }
                        .call(c, 0x002850c8)?;
                        c.g[RA][0] = sx(0x002850d0);
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x00297f70, 0x002850d0)?;
                        c.g[A0][0] = 0x290000;
                        c.g[RA][0] = sx(0x002850dc);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x69b0));
                        call_ptr(c, 0x003b5a28, 0x002850dc)?;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x5fb0), 0u64);
                        c.g[A1][0] = 0xffffffffffffffff;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x5fb4), 0u64);
                        c.g[V1][0] = c.g[S3][0];
                        c.g[V0][0] = c.g[S5][0];
                        c.g[A0][0] = 0x1;
                        'l_2850f8: loop {
                            c.cycles += 7;
                            c.sw(c.g[V0][0] as u32, 0u64);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1));
                            c.sw(c.g[V1][0] as u32, c.g[A1][0]);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
                            let t = (c.g[A0][0] as i64) >= 0;
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x002850f8);
                                }
                                continue 'l_2850f8;
                            } else {
                                c.cycles += 7;
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x5fe0), 0u64);
                                c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x8));
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x5fe4), 0u64);
                                c.g[A0][0] = 0xffffffffffffffff;
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x5fec), 0u64);
                                c.g[V1][0] = 0x2;
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x5ff0), 0u64);
                                'l_285130: loop {
                                    c.cycles += 7;
                                    c.sw(c.g[V0][0] as u32, c.g[A0][0]);
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                    let t = (c.g[V1][0] as i64) >= 0;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x00285130);
                                        }
                                        continue 'l_285130;
                                    } else {
                                        c.cycles += 5;
                                        c.g[RA][0] = sx(0x00285154);
                                        c.g[A0][0] = c.g[S1][0];
                                        Callee {
                                            addr: 0x002948a0,
                                            f: ee::s07::SSXAudioSystem_ClearRiderAirFlags,
                                        }
                                        .call(c, 0x00285154)?;
                                        c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add(0x58));
                                        c.g[V1][0] = 0x16;
                                        'l_285160: loop {
                                            c.cycles += 7;
                                            c.sw(c.g[V0][0] as u32, 0u64);
                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                                            let t = (c.g[V1][0] as i64) >= 0;
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x00285160);
                                                }
                                                continue 'l_285160;
                                            } else {
                                                c.cycles += 36;
                                                c.g[A0][0] = c.g[S1][0];
                                                c.g[RA][0] = sx(0x00285188);
                                                c.g[A1][0] = 0x1;
                                                call_ptr(c, 0x002a4590, 0x00285188)?;
                                                c.g[S0][0] = 0x4a0000;
                                                c.g[A1][0] = 0u64;
                                                c.g[RA][0] = sx(0x00285198);
                                                c.g[A0][0] = c.g[S1][0];
                                                call_ptr(c, 0x0029ed90, 0x00285198)?;
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x3568));
                                                c.g[RA][0] = sx(0x002851a4);
                                                c.g[A0][0] = c.g[S2][0];
                                                call_ptr(c, 0x002b0ce0, 0x002851a4)?;
                                                c.g[A0][0] = c.g[S1][0];
                                                c.g[A1][0] = 0x4;
                                                c.g[RA][0] = sx(0x002851b4);
                                                c.g[A2][0] = 0u64;
                                                Callee {
                                                    addr: 0x00287968,
                                                    f: ee::s07::SSXAudioSystem_GetChannelGainPtr,
                                                }
                                                .call(c, 0x002851b4)?;
                                                c.g[T0][0] = c.g[V0][0];
                                                c.g[A3][0] = c.g[S0][0];
                                                c.g[T1][0] = 0x1;
                                                c.g[T2][0] = 0xffffffffffffffff;
                                                c.g[A0][0] = c.g[S2][0];
                                                c.g[A1][0] = 0u64;
                                                c.g[RA][0] = sx(0x002851d4);
                                                c.g[A2][0] = 0u64;
                                                call_ptr(c, 0x002b0e60, 0x002851d4)?;
                                                c.g[RA][0] = sx(0x002851dc);
                                                c.g[A0][0] = c.g[S1][0];
                                                Callee { addr: 0x0028f700, f: ee::s07::SSXAudioSystem_LoadLoadingScreenBank }.call(c, 0x002851dc)?;
                                                c.g[V0][0] = c.g[S1][0];
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
                                                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x110));
                                                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x100));
                                                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xf0));
                                                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
                                                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xd0));
                                                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xc0));
                                                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0xb0));
                                                c.g[RA][0] =
                                                    c.ld((c.g[SP][0] as u32).wrapping_add(0xa0));
                                                let t = c.g[RA][0] as u32;
                                                c.g[SP][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0x130));
                                                c.pc = t;
                                                return Ok(());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    });
}

/// `void SSXAudioSystem_LoadWorldAudio(SSXAudioSystem* self)`: parses the audio ini (GLOBAL basepath, then the section named like the world extension) and loads the MAIN, BOARD, SPUBOARD, CROWD, AUX, TRICKY, MOUNTAIN, TRANSPORT and LAND banks, then sets the bank memory, loads the crowd audio and the grunt bank.
/// Hand port of the generated body (SSXAudioSystem_LoadWorldAudio, 0x002862a8), every register, cycle and
/// call kept; any other `pc` (a resume after an unwound callee) runs the generated body.
pub fn ssx_audio_system_load_world_audio(c: &mut Cpu) {
    if c.pc != 0x002862a8 {
        return ee::s07::f_002862a8__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x002862a8;
        loop {
            match pc {
                0x002862a8 => {
                    c.cycles += 43;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xad0));
                    c.g[V0][0] = 0x4a0000;
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xac0));
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x35a0));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xa80));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xa60));
                    c.g[S4][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xab0));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xaa0));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xa90));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xa70));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0xa50));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0xa40));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0xa30), c.g[RA][0]);
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x35a0));
                    c.g[A2][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.sw(c.g[SP][0] as u32, c.g[A1][0]);
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A2][0]);
                    c.g[RA][0] = sx(0x002862f8);
                    c.g[A1][0] = 0x1;
                    Callee {
                        addr: 0x00288d18,
                        f: ee::s07::SSXAudioSystem_SetupChannelGains,
                    }
                    .call(c, 0x002862f8)?;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0u64;
                    c.g[RA][0] = sx(0x0028630c);
                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                    Callee {
                        addr: 0x0028a1e8,
                        f: ee::s07::SndLoadQueue_AllocAsyncBuffer,
                    }
                    .call(c, 0x0028630c)?;
                    c.g[V0][0] = 0x4a0000;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                    c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_add(0x35a8));
                    c.g[A0][0] = c.g[S0][0];
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x5878), 0u64);
                    c.g[A2][0] = 0x100;
                    c.sb((c.g[S4][0] as u32).wrapping_add(0x5838), c.g[V1][0]);
                    c.g[S6][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00286334);
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x464));
                    call_ptr(c, 0x002a77f8, 0x00286334)?;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc4));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3570));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0028634c);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), c.g[V1][0]);
                    call_ptr(c, 0x002a7bf8, 0x0028634c)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc4));
                    if t {
                        pc = 0x002863a0;
                        continue;
                    }
                    pc = 0x00286354;
                    continue;
                }
                0x00286354 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x00286390;
                        continue;
                    }
                    pc = 0x0028635c;
                    continue;
                }
                0x0028635c => {
                    c.cycles += 1;
                    pc = 0x00286360;
                    continue;
                }
                0x00286360 => {
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x00286368);
                    c.g[A0][0] = c.g[S6][0];
                    call_ptr(c, 0x002a7da0, 0x00286368)?;
                    c.g[V1][0] = 0x480000;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x28c0));
                    c.g[RA][0] = sx(0x0028637c);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0041aa88, 0x0028637c)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = c.g[S0][0];
                    if t {
                        pc = 0x00286390;
                        continue;
                    }
                    pc = 0x00286384;
                    continue;
                }
                0x00286384 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x00286390);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    call_ptr(c, 0x002a7ff8, 0x00286390)?;
                    pc = 0x00286390;
                    continue;
                }
                0x00286390 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00286398);
                    c.g[A0][0] = c.g[S6][0];
                    call_ptr(c, 0x002a7f90, 0x00286398)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xc4));
                    if t {
                        if back_edge_unwinds(c, 0x00286360) {
                            return Ok(());
                        }
                        pc = 0x00286360;
                        continue;
                    }
                    pc = 0x002863a0;
                    continue;
                }
                0x002863a0 => {
                    c.cycles += 8;
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x8d0));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xa20), c.g[V1][0]);
                    c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x950));
                    c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x9b0));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc8), c.g[V0][0]);
                    c.g[FP][0] = 0x4a0000;
                    let t = true;
                    c.g[S7][0] = 0x4a0000;
                    if t {
                        pc = 0x002866ec;
                        continue;
                    }
                    pc = 0x002863c0;
                    continue;
                }
                0x002863c0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002863c8);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, 0x004165a8, 0x002863c8)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xa20));
                    if t {
                        pc = 0x002866ec;
                        continue;
                    }
                    pc = 0x002863d0;
                    continue;
                }
                0x002863d0 => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x002863d8);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
                    call_ptr(c, 0x004166f4, 0x002863d8)?;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xa20));
                    c.g[RA][0] = sx(0x002863e4);
                    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x587c));
                    call_ptr(c, 0x004166f4, 0x002863e4)?;
                    let t = true;
                    c.g[S5][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xa20));
                    if t {
                        pc = 0x002866dc;
                        continue;
                    }
                    pc = 0x002863ec;
                    continue;
                }
                0x002863ec => {
                    c.cycles += 1;
                    pc = 0x002863f0;
                    continue;
                }
                0x002863f0 => {
                    c.cycles += 15;
                    c.g[RA][0] = sx(0x002863f8);
                    c.g[A0][0] = c.g[S6][0];
                    call_ptr(c, 0x002a7da0, 0x002863f8)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S6][0];
                    c.g[V0][0] = 0x480000;
                    c.g[A2][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x00286410);
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x28c0));
                    call_ptr(c, 0x002a7ff8, 0x00286410)?;
                    c.g[V1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3580));
                    c.g[RA][0] = sx(0x00286424);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0041aa88, 0x00286424)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x00286460;
                        continue;
                    }
                    pc = 0x0028642c;
                    continue;
                }
                0x0028642c => {
                    c.cycles += 13;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x0028643c);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x002a7ff8, 0x0028643c)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3588));
                    c.g[A2][0] = sx((c.g[S7][0] as u32).wrapping_add(0x3590));
                    c.g[A3][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x00286454);
                    c.g[T0][0] = c.g[S3][0];
                    call_ptr(c, 0x00417828, 0x00286454)?;
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                    let t = true;
                    c.g[A1][0] = 0u64;
                    if t {
                        pc = 0x00286670;
                        continue;
                    }
                    pc = 0x00286460;
                    continue;
                }
                0x00286460 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x35b0));
                    c.g[RA][0] = sx(0x00286470);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0041aa88, 0x00286470)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x002864ac;
                        continue;
                    }
                    pc = 0x00286478;
                    continue;
                }
                0x00286478 => {
                    c.cycles += 13;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x00286488);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x002a7ff8, 0x00286488)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3588));
                    c.g[A2][0] = sx((c.g[S7][0] as u32).wrapping_add(0x3590));
                    c.g[A3][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x002864a0);
                    c.g[T0][0] = c.g[S3][0];
                    call_ptr(c, 0x00417828, 0x002864a0)?;
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                    let t = true;
                    c.g[A1][0] = 0x1;
                    if t {
                        pc = 0x00286670;
                        continue;
                    }
                    pc = 0x002864ac;
                    continue;
                }
                0x002864ac => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x28d0));
                    c.g[RA][0] = sx(0x002864bc);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0041aa88, 0x002864bc)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x002864f8;
                        continue;
                    }
                    pc = 0x002864c4;
                    continue;
                }
                0x002864c4 => {
                    c.cycles += 13;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x002864d4);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x002a7ff8, 0x002864d4)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3588));
                    c.g[A2][0] = sx((c.g[S7][0] as u32).wrapping_add(0x3590));
                    c.g[A3][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x002864ec);
                    c.g[T0][0] = c.g[S3][0];
                    call_ptr(c, 0x00417828, 0x002864ec)?;
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                    let t = true;
                    c.g[A1][0] = 0x10;
                    if t {
                        pc = 0x00286670;
                        continue;
                    }
                    pc = 0x002864f8;
                    continue;
                }
                0x002864f8 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x35b8));
                    c.g[RA][0] = sx(0x00286508);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0041aa88, 0x00286508)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x00286544;
                        continue;
                    }
                    pc = 0x00286510;
                    continue;
                }
                0x00286510 => {
                    c.cycles += 13;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x00286520);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x002a7ff8, 0x00286520)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3588));
                    c.g[A2][0] = sx((c.g[S7][0] as u32).wrapping_add(0x3590));
                    c.g[A3][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x00286538);
                    c.g[T0][0] = c.g[S3][0];
                    call_ptr(c, 0x00417828, 0x00286538)?;
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                    let t = true;
                    c.g[A1][0] = 0x5;
                    if t {
                        pc = 0x00286670;
                        continue;
                    }
                    pc = 0x00286544;
                    continue;
                }
                0x00286544 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x35c0));
                    c.g[RA][0] = sx(0x00286554);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0041aa88, 0x00286554)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x00286590;
                        continue;
                    }
                    pc = 0x0028655c;
                    continue;
                }
                0x0028655c => {
                    c.cycles += 13;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x0028656c);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x002a7ff8, 0x0028656c)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3588));
                    c.g[A2][0] = sx((c.g[S7][0] as u32).wrapping_add(0x3590));
                    c.g[A3][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x00286584);
                    c.g[T0][0] = c.g[S3][0];
                    call_ptr(c, 0x00417828, 0x00286584)?;
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                    let t = true;
                    c.g[A1][0] = 0x6;
                    if t {
                        pc = 0x00286670;
                        continue;
                    }
                    pc = 0x00286590;
                    continue;
                }
                0x00286590 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x35c8));
                    c.g[RA][0] = sx(0x002865a0);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0041aa88, 0x002865a0)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x002865dc;
                        continue;
                    }
                    pc = 0x002865a8;
                    continue;
                }
                0x002865a8 => {
                    c.cycles += 13;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x002865b8);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x002a7ff8, 0x002865b8)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3588));
                    c.g[A2][0] = sx((c.g[S7][0] as u32).wrapping_add(0x3590));
                    c.g[A3][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x002865d0);
                    c.g[T0][0] = c.g[S3][0];
                    call_ptr(c, 0x00417828, 0x002865d0)?;
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                    let t = true;
                    c.g[A1][0] = 0x7;
                    if t {
                        pc = 0x00286670;
                        continue;
                    }
                    pc = 0x002865dc;
                    continue;
                }
                0x002865dc => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x28e0));
                    c.g[RA][0] = sx(0x002865ec);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0041aa88, 0x002865ec)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x00286628;
                        continue;
                    }
                    pc = 0x002865f4;
                    continue;
                }
                0x002865f4 => {
                    c.cycles += 13;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x00286604);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x002a7ff8, 0x00286604)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3588));
                    c.g[A2][0] = sx((c.g[S7][0] as u32).wrapping_add(0x3590));
                    c.g[A3][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x0028661c);
                    c.g[T0][0] = c.g[S3][0];
                    call_ptr(c, 0x00417828, 0x0028661c)?;
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                    let t = true;
                    c.g[A1][0] = 0x2;
                    if t {
                        pc = 0x00286670;
                        continue;
                    }
                    pc = 0x00286628;
                    continue;
                }
                0x00286628 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x28f0));
                    c.g[RA][0] = sx(0x00286638);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0041aa88, 0x00286638)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x00286688;
                        continue;
                    }
                    pc = 0x00286640;
                    continue;
                }
                0x00286640 => {
                    c.cycles += 12;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x00286650);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x002a7ff8, 0x00286650)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3588));
                    c.g[A2][0] = sx((c.g[S7][0] as u32).wrapping_add(0x3590));
                    c.g[A3][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x00286668);
                    c.g[T0][0] = c.g[S3][0];
                    call_ptr(c, 0x00417828, 0x00286668)?;
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                    c.g[A1][0] = 0x4;
                    pc = 0x00286670;
                    continue;
                }
                0x00286670 => {
                    c.cycles += 6;
                    c.g[A2][0] = c.g[S2][0];
                    c.g[A3][0] = 0u64;
                    c.g[RA][0] = sx(0x00286680);
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    Callee {
                        addr: 0x0028bc58,
                        f: ee::s07::SndBankMan_LoadBank,
                    }
                    .call(c, 0x00286680)?;
                    let t = true;

                    if t {
                        pc = 0x002866dc;
                        continue;
                    }
                    pc = 0x00286688;
                    continue;
                }
                0x00286688 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x35d0));
                    c.g[RA][0] = sx(0x00286698);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0041aa88, 0x00286698)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = c.g[S0][0];
                    if t {
                        pc = 0x002866dc;
                        continue;
                    }
                    pc = 0x002866a0;
                    continue;
                }
                0x002866a0 => {
                    c.cycles += 15;
                    c.g[A0][0] = c.g[S6][0];
                    c.g[RA][0] = sx(0x002866ac);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x002a7ff8, 0x002866ac)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x3588));
                    c.g[A2][0] = sx((c.g[S7][0] as u32).wrapping_add(0x3590));
                    c.g[A3][0] = c.g[S5][0];
                    c.g[RA][0] = sx(0x002866c4);
                    c.g[T0][0] = c.g[S3][0];
                    call_ptr(c, 0x00417828, 0x002866c4)?;
                    c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                    c.g[A1][0] = 0xa;
                    c.g[A2][0] = c.g[S2][0];
                    c.g[A3][0] = 0u64;
                    c.g[RA][0] = sx(0x002866dc);
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    Callee {
                        addr: 0x0028bc58,
                        f: ee::s07::SndBankMan_LoadBank,
                    }
                    .call(c, 0x002866dc)?;
                    pc = 0x002866dc;
                    continue;
                }
                0x002866dc => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002866e4);
                    c.g[A0][0] = c.g[S6][0];
                    call_ptr(c, 0x002a7f90, 0x002866e4)?;
                    let t = c.g[V0][0] == 0u64;

                    if t {
                        if back_edge_unwinds(c, 0x002863f0) {
                            return Ok(());
                        }
                        pc = 0x002863f0;
                        continue;
                    }
                    pc = 0x002866ec;
                    continue;
                }
                0x002866ec => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002866f4);
                    c.g[A0][0] = c.g[S6][0];
                    call_ptr(c, 0x002a7b08, 0x002866f4)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.g[V0][0];
                    if t {
                        if back_edge_unwinds(c, 0x002863c0) {
                            return Ok(());
                        }
                        pc = 0x002863c0;
                        continue;
                    }
                    pc = 0x002866fc;
                    continue;
                }
                0x002866fc => {
                    c.cycles += 27;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                    c.g[A2][0] = 0x10000;
                    c.g[A1][0] = 0x3;
                    c.g[A2][0] = c.g[A2][0] | 0x9000;
                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                    c.g[A3][0] = 0u64;
                    c.g[RA][0] = sx(0x0028671c);
                    c.g[T0][0] = 0u64;
                    Callee {
                        addr: 0x0028bda8,
                        f: ee::s07::SndBankMan_SetBankMemory,
                    }
                    .call(c, 0x0028671c)?;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x118));
                    c.g[A2][0] = 0x60000;
                    c.g[A1][0] = 0xc;
                    c.g[A2][0] = c.g[A2][0] | 0xa400;
                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                    c.g[A3][0] = 0u64;
                    c.g[RA][0] = sx(0x0028673c);
                    c.g[T0][0] = 0u64;
                    Callee {
                        addr: 0x0028bda8,
                        f: ee::s07::SndBankMan_SetBankMemory,
                    }
                    .call(c, 0x0028673c)?;
                    c.g[RA][0] = sx(0x00286744);
                    call_ptr(c, 0x0014dc80, 0x00286744)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00286750);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x00286750)?;
                    c.g[V0][0] = 0x530000;
                    c.g[A0][0] = 0x2;
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                    c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
                    let t = c.g[V0][0] != c.g[A0][0];
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x468));
                    if t {
                        pc = 0x00286784;
                        continue;
                    }
                    pc = 0x00286768;
                    continue;
                }
                0x00286768 => {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A3][0] = 0x2;
                    c.g[RA][0] = sx(0x0028677c);
                    c.g[T0][0] = 0x5;
                    call_ptr(c, 0x002a6d78, 0x0028677c)?;
                    let t = true;
                    c.g[A0][0] = c.g[S4][0];
                    if t {
                        pc = 0x0028679c;
                        continue;
                    }
                    pc = 0x00286784;
                    continue;
                }
                0x00286784 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A3][0] = 0x1;
                    c.g[RA][0] = sx(0x00286798);
                    c.g[T0][0] = 0x5;
                    call_ptr(c, 0x002a6d78, 0x00286798)?;
                    c.g[A0][0] = c.g[S4][0];
                    pc = 0x0028679c;
                    continue;
                }
                0x0028679c => {
                    c.cycles += 19;
                    c.g[RA][0] = sx(0x002867a4);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0029f3f8, 0x002867a4)?;
                    c.g[V0][0] = 0x1;
                    c.g[A0][0] = c.g[S6][0];
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x627c), c.g[V0][0]);
                    c.g[RA][0] = sx(0x002867b8);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002a7890, 0x002867b8)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xac0));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xab0));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0xaa0));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xa90));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xa80));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xa70));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xa60));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0xa50));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0xa40));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xa30));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xad0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002862a8, 0x002867e8);
                    return Ok(());
                }
            }
        }
    });
}

/// `void SSXAudioSystem_SetChannelVolume(SSXAudioSystem* self, int channel, float volume)`: clamps the volume to [0, 1], scales it per channel (jump table, constants at gp+0x414..), stores volume and gain in the channel record at +0x62bc and for channel 0/1/4 updates the music player, the master volume or the per-bank gains.
/// Hand port of the generated body (SSXAudioSystem_SetChannelVolume, 0x00287700), every register, cycle and
/// call kept; any other `pc` (a resume after an unwound callee) runs the generated body.
pub fn ssx_audio_system_set_channel_volume(c: &mut Cpu) {
    if c.pc != 0x00287700 {
        return ee::s07::f_00287700__generated(c);
    }
    guest(c, |c| {
        'b_2877e0: {
            'b_2877dc: {
                c.cycles += 17;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S2][0] = c.g[A1][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = sx((c.g[S2][0] as u32) << 4);
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x62bc));
                c.mtc1(0, 1);
                c.g[S1][0] = c.g[A0][0];
                c.min_s(12, 12, 0);
                c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[V0][0] = (c.g[S2][0] < 0xbu64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.max_s(12, 12, 1);
                if t {
                    break 'b_2877dc;
                } else {
                    'b_2877d0: {
                        'b_2877cc: {
                            'b_2877c4: {
                                'b_2877bc: {
                                    'b_2877b4: {
                                        'b_2877ac: {
                                            'b_2877a4: {
                                                'b_28779c: {
                                                    'b_287794: {
                                                        'b_28778c: {
                                                            'b_28776c: {
                                                                c.cycles += 7;
                                                                c.g[V0][0] = 0x480000;
                                                                c.g[V1][0] =
                                                                    sx((c.g[S2][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x2900));
                                                                c.g[V1][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[V0][0] as u32,
                                                                    ));
                                                                c.g[A0][0] =
                                                                    c.lw(c.g[V1][0] as u32);
                                                                let t = c.g[A0][0] as u32;
                                                                match t {
                                                                    0x00287760 => {
                                                                        c.cycles += 3;
                                                                        c.swc1(
                                                                            12,
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(0x4),
                                                                        );
                                                                        let t = true;
                                                                        c.swc1(
                                                                            12,
                                                                            c.g[S0][0] as u32,
                                                                        );
                                                                        if t {
                                                                            break 'b_2877dc;
                                                                        } else {
                                                                            break 'b_28776c;
                                                                        }
                                                                    }
                                                                    0x0028776c => {
                                                                        break 'b_28776c;
                                                                    }
                                                                    0x0028778c => {
                                                                        break 'b_28778c;
                                                                    }
                                                                    0x00287794 => {
                                                                        break 'b_287794;
                                                                    }
                                                                    0x0028779c => {
                                                                        break 'b_28779c;
                                                                    }
                                                                    0x002877a4 => {
                                                                        break 'b_2877a4;
                                                                    }
                                                                    0x002877ac => {
                                                                        break 'b_2877ac;
                                                                    }
                                                                    0x002877b4 => {
                                                                        break 'b_2877b4;
                                                                    }
                                                                    0x002877bc => {
                                                                        break 'b_2877bc;
                                                                    }
                                                                    0x002877c4 => {
                                                                        break 'b_2877c4;
                                                                    }
                                                                    0x002877cc => {
                                                                        break 'b_2877cc;
                                                                    }
                                                                    _ => {
                                                                        c.jump_out(
                                                                            t, 0x00287700,
                                                                            0x00287920,
                                                                        );
                                                                        return Ok(());
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 8;
                                                            c.lwc1(
                                                                0,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_add(0x418),
                                                            );
                                                            c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                                .wrapping_add(0x118));
                                                            c.swc1(12, c.g[S0][0] as u32);
                                                            c.mul_s(0, 12, 0);
                                                            c.g[RA][0] = sx(0x00287784);
                                                            c.swc1(
                                                                0,
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            call_ptr(c, 0x002b4388, 0x00287784)?;
                                                            let t = true;
                                                            c.g[V0][0] =
                                                                sx((c.g[S2][0] as u32) << 2);
                                                            if t {
                                                                break 'b_2877e0;
                                                            } else {
                                                                break 'b_28778c;
                                                            }
                                                        }
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.lwc1(
                                                            0,
                                                            (c.g[GP][0] as u32).wrapping_add(0x420),
                                                        );
                                                        if t {
                                                            break 'b_2877d0;
                                                        } else {
                                                            break 'b_287794;
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.lwc1(
                                                        0,
                                                        (c.g[GP][0] as u32).wrapping_add(0x424),
                                                    );
                                                    if t {
                                                        break 'b_2877d0;
                                                    } else {
                                                        break 'b_28779c;
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = true;
                                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x41c));
                                                if t {
                                                    break 'b_2877d0;
                                                } else {
                                                    break 'b_2877a4;
                                                }
                                            }
                                            c.cycles += 2;
                                            let t = true;
                                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x414));
                                            if t {
                                                break 'b_2877d0;
                                            } else {
                                                break 'b_2877ac;
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = true;
                                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x428));
                                        if t {
                                            break 'b_2877d0;
                                        } else {
                                            break 'b_2877b4;
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = true;
                                    c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x42c));
                                    if t {
                                        break 'b_2877d0;
                                    } else {
                                        break 'b_2877bc;
                                    }
                                }
                                c.cycles += 2;
                                let t = true;
                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x430));
                                if t {
                                    break 'b_2877d0;
                                } else {
                                    break 'b_2877c4;
                                }
                            }
                            c.cycles += 2;
                            let t = true;
                            c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x434));
                            if t {
                                break 'b_2877d0;
                            } else {
                                break 'b_2877cc;
                            }
                        }
                        c.cycles += 1;
                        c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x438));
                        break 'b_2877d0;
                    }
                    c.cycles += 3;
                    c.swc1(12, c.g[S0][0] as u32);
                    c.mul_s(0, 12, 0);
                    c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x4));
                    break 'b_2877dc;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
            break 'b_2877e0;
        }
        'b_287910: {
            'b_28790c: {
                'b_287870: {
                    'b_287860: {
                        c.cycles += 11;
                        c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        c.g[A0][0] = 0x1;
                        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x6394));
                        c.g[V0][0] = c.g[V1][0];
                        c.mul_s(1, 1, 0);
                        c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x8));
                        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x63e4));
                        c.mul_s(1, 1, 0);
                        let t = c.g[S2][0] == c.g[A0][0];
                        c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0xc));
                        if t {
                            break 'b_287860;
                        } else {
                            'b_287838: {
                                'b_287828: {
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[S2][0] as i64) < 2i64) as u64;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V0][0] = 0x4;
                                        break 'b_287828;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[S2][0] == 0u64;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        if t {
                                            break 'b_287838;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                                            if t {
                                                break 'b_287910;
                                            } else {
                                                break 'b_287828;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[S2][0] == c.g[V0][0];
                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x43c));
                                if t {
                                    break 'b_287870;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                    if t {
                                        break 'b_28790c;
                                    } else {
                                        break 'b_287838;
                                    }
                                }
                            }
                            c.cycles += 10;
                            c.g[AT][0] = 0x42fe0000;
                            c.mtc1(1, 0);
                            c.mul_s(0, 1, 0);
                            c.cvt_w_s(1, 0);
                            c.mfc1(4, 1);
                            c.g[A0][0] = sx((c.g[A0][0] as u32) << 24);
                            c.g[RA][0] = sx(0x00287858);
                            c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 24) as u32);
                            call_ptr(c, 0x003b8160, 0x00287858)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_28790c;
                            } else {
                                break 'b_287860;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00287868);
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x118));
                    call_ptr(c, 0x002b4388, 0x00287868)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        break 'b_28790c;
                    } else {
                        break 'b_287870;
                    }
                }
                c.cycles += 39;
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_add(0x440));
                c.mul_s(0, 1, 0);
                c.lwc1(3, (c.g[GP][0] as u32).wrapping_add(0x444));
                c.lwc1(4, (c.g[GP][0] as u32).wrapping_add(0x448));
                c.lwc1(5, (c.g[GP][0] as u32).wrapping_add(0x44c));
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x636c));
                c.lwc1(6, (c.g[GP][0] as u32).wrapping_add(0x450));
                c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.lwc1(7, (c.g[GP][0] as u32).wrapping_add(0x454));
                c.mul_s(1, 1, 2);
                c.lwc1(8, (c.g[GP][0] as u32).wrapping_add(0x458));
                c.lwc1(2, (c.g[GP][0] as u32).wrapping_add(0x45c));
                c.lwc1(9, (c.g[GP][0] as u32).wrapping_add(0x460));
                c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x6370));
                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.mul_s(0, 0, 3);
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x6374));
                c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.mul_s(1, 1, 4);
                c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x6378));
                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.mul_s(0, 0, 5);
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x637c));
                c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.mul_s(1, 1, 6);
                c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x6380));
                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.mul_s(0, 0, 7);
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x6384));
                c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.mul_s(1, 1, 8);
                c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x6388));
                c.lwc1(0, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.mul_s(0, 0, 2);
                c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x638c));
                c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0xc));
                c.mul_s(1, 1, 9);
                c.swc1(1, (c.g[S1][0] as u32).wrapping_add(0x6390));
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                break 'b_28790c;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_287910;
        }
        c.cycles += 4;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `SSXAudioSystem_LoadVolumeConfig(self)` (0x00287fc8): read the audio mix
/// config (an INI file of sections with MUSIC, DJ, PA, CHARACTER, BOARD,
/// COLLISION, AMBIENT, ARCADESFX, ARCADESPEECH levels and a TIME in ms) into
/// up to 25 mix records at `self + 0x64a0` (0x28 bytes each; `+0x6478` is the
/// record count). Every record starts at level 100, time 0.
///
/// The normal entry only, transcribed from the generated body; a resume
/// (`pc` inside the function) runs the generated body.
pub fn ssx_audio_system_load_volume_config(c: &mut Cpu) {
    if c.pc != 0x00287fc8 {
        return ee::s07::f_00287fc8__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x00287fc8;
        loop {
            match pc {
                0x00287fc8 => {
                    c.cycles += 38;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x8d0));
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x470));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x8a0));
                    c.g[A2][0] = 0x100;
                    c.g[S2][0] = c.g[A0][0];
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x880));
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x647c));
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6480));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x814), c.g[V0][0]);
                    c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x64a0));
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6484));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x818), c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x81c), c.g[V0][0]);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6488));
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x648c));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x870));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x860));
                    c.g[S5][0] = sx((c.g[S2][0] as u32).wrapping_add(0x649c));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x850));
                    c.g[S6][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6498));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x840));
                    c.g[S7][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6494));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x820), c.g[A0][0]);
                    c.g[FP][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6490));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x8c0));
                    c.g[A0][0] = c.g[SP][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x8b0));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x890));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x830), c.g[RA][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x824), c.g[V0][0]);
                    c.g[RA][0] = sx(0x00288048);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x6478), 0u64);
                    call_ptr(c, 0x002a77f8, 0x00288048)?;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = 0x64;
                    c.g[V1][0] = c.g[S4][0];
                    c.g[A1][0] = 0x18;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    pc = 0x00288060;
                    continue;
                }
                0x00288060 => {
                    c.cycles += 13;
                    c.sw((c.g[V1][0] as u32).wrapping_sub(0x24), c.g[A0][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
                    c.sw((c.g[V1][0] as u32).wrapping_sub(0x20), c.g[A0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_sub(0x1c), c.g[A0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_sub(0x18), c.g[A0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_sub(0x14), c.g[A0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_sub(0x10), c.g[A0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_sub(0xc), c.g[A0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_sub(0x8), c.g[A0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_sub(0x4), c.g[A0][0]);
                    c.sw(c.g[V1][0] as u32, 0u64);
                    let t = (c.g[A1][0] as i64) >= 0;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x28));
                    if t {
                        if back_edge_unwinds(c, 0x00288060) {
                            return Ok(());
                        }
                        pc = 0x00288060;
                        continue;
                    }
                    pc = 0x00288094;
                    continue;
                }
                0x00288094 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6478));
                    if t {
                        pc = 0x00288318;
                        continue;
                    }
                    pc = 0x0028809c;
                    continue;
                }
                0x0028809c => {
                    c.cycles += 1;
                    pc = 0x002880a0;
                    continue;
                }
                0x002880a0 => {
                    c.cycles += 10;
                    c.g[RA][0] = sx(0x002880a8);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x002a7da0, 0x002880a8)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V0][0] = 0x4a0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x35d8));
                    c.g[RA][0] = sx(0x002880c0);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x002880c0)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x002880e0;
                        continue;
                    }
                    pc = 0x002880c8;
                    continue;
                }
                0x002880c8 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002880d8);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x810));
                    call_ptr(c, 0x002a80d8, 0x002880d8)?;
                    let t = true;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x814));
                    if t {
                        pc = 0x0028819c;
                        continue;
                    }
                    pc = 0x002880e0;
                    continue;
                }
                0x002880e0 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x35e0));
                    c.g[RA][0] = sx(0x002880f0);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x002880f0)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x00288110;
                        continue;
                    }
                    pc = 0x002880f8;
                    continue;
                }
                0x002880f8 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00288108);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x810));
                    call_ptr(c, 0x002a80d8, 0x00288108)?;
                    let t = true;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x818));
                    if t {
                        pc = 0x0028819c;
                        continue;
                    }
                    pc = 0x00288110;
                    continue;
                }
                0x00288110 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x35e8));
                    c.g[RA][0] = sx(0x00288120);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x00288120)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x00288140;
                        continue;
                    }
                    pc = 0x00288128;
                    continue;
                }
                0x00288128 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00288138);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x810));
                    call_ptr(c, 0x002a80d8, 0x00288138)?;
                    let t = true;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x81c));
                    if t {
                        pc = 0x0028819c;
                        continue;
                    }
                    pc = 0x00288140;
                    continue;
                }
                0x00288140 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2930));
                    c.g[RA][0] = sx(0x00288150);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x00288150)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x00288170;
                        continue;
                    }
                    pc = 0x00288158;
                    continue;
                }
                0x00288158 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00288168);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x810));
                    call_ptr(c, 0x002a80d8, 0x00288168)?;
                    let t = true;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x820));
                    if t {
                        pc = 0x0028819c;
                        continue;
                    }
                    pc = 0x00288170;
                    continue;
                }
                0x00288170 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x35b0));
                    c.g[RA][0] = sx(0x00288180);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x00288180)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x002881b4;
                        continue;
                    }
                    pc = 0x00288188;
                    continue;
                }
                0x00288188 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00288198);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x810));
                    call_ptr(c, 0x002a80d8, 0x00288198)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x824));
                    pc = 0x0028819c;
                    continue;
                }
                0x0028819c => {
                    c.cycles += 6;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6478));
                    c.lo[0] = c.g[A0][0];
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x810));
                    c.madd(0, 3, 3, 19);
                    let t = true;
                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                    if t {
                        pc = 0x002882f8;
                        continue;
                    }
                    pc = 0x002881b4;
                    continue;
                }
                0x002881b4 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2940));
                    c.g[RA][0] = sx(0x002881c4);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x002881c4)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x002881f4;
                        continue;
                    }
                    pc = 0x002881cc;
                    continue;
                }
                0x002881cc => {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x002881dc);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x810));
                    call_ptr(c, 0x002a80d8, 0x002881dc)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6478));
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x810));
                    c.mult(0, 4, 3, 19);
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[FP][0] as u32));
                    let t = true;
                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                    if t {
                        pc = 0x002882f8;
                        continue;
                    }
                    pc = 0x002881f4;
                    continue;
                }
                0x002881f4 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x35f0));
                    c.g[RA][0] = sx(0x00288204);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x00288204)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x00288234;
                        continue;
                    }
                    pc = 0x0028820c;
                    continue;
                }
                0x0028820c => {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0028821c);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x810));
                    call_ptr(c, 0x002a80d8, 0x0028821c)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6478));
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x810));
                    c.mult(0, 4, 3, 19);
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S7][0] as u32));
                    let t = true;
                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                    if t {
                        pc = 0x002882f8;
                        continue;
                    }
                    pc = 0x00288234;
                    continue;
                }
                0x00288234 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2950));
                    c.g[RA][0] = sx(0x00288244);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x00288244)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x480000;
                    if t {
                        pc = 0x00288274;
                        continue;
                    }
                    pc = 0x0028824c;
                    continue;
                }
                0x0028824c => {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0028825c);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x810));
                    call_ptr(c, 0x002a80d8, 0x0028825c)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6478));
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x810));
                    c.mult(0, 4, 3, 19);
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S6][0] as u32));
                    let t = true;
                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                    if t {
                        pc = 0x002882f8;
                        continue;
                    }
                    pc = 0x00288274;
                    continue;
                }
                0x00288274 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2960));
                    c.g[RA][0] = sx(0x00288284);
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041aa88, 0x00288284)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x002882b4;
                        continue;
                    }
                    pc = 0x0028828c;
                    continue;
                }
                0x0028828c => {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x0028829c);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x810));
                    call_ptr(c, 0x002a80d8, 0x0028829c)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6478));
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x810));
                    c.mult(0, 4, 3, 19);
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    let t = true;
                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                    if t {
                        pc = 0x002882f8;
                        continue;
                    }
                    pc = 0x002882b4;
                    continue;
                }
                0x002882b4 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x35f8));
                    c.g[RA][0] = sx(0x002882c4);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0041aa88, 0x002882c4)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = c.g[S0][0];
                    if t {
                        pc = 0x002882f8;
                        continue;
                    }
                    pc = 0x002882cc;
                    continue;
                }
                0x002882cc => {
                    c.cycles += 11;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x002882d8);
                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x810));
                    call_ptr(c, 0x002a80d8, 0x002882d8)?;
                    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6478));
                    c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x810));
                    c.cvt_s_w(0, 0);
                    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x461c));
                    c.mult(0, 2, 3, 19);
                    c.mul_s(0, 0, 1);
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                    c.swc1(0, c.g[V1][0] as u32);
                    pc = 0x002882f8;
                    continue;
                }
                0x002882f8 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x00288300);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x002a7f90, 0x00288300)?;
                    let t = c.g[V0][0] == 0u64;

                    if t {
                        if back_edge_unwinds(c, 0x002880a0) {
                            return Ok(());
                        }
                        pc = 0x002880a0;
                        continue;
                    }
                    pc = 0x00288308;
                    continue;
                }
                0x00288308 => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6478));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x6478), c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6478));
                    pc = 0x00288318;
                    continue;
                }
                0x00288318 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 25i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[SP][0];
                    if t {
                        pc = 0x00288338;
                        continue;
                    }
                    pc = 0x00288324;
                    continue;
                }
                0x00288324 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0028832c);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x002a7b08, 0x0028832c)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S3][0] = 0x28;
                    if t {
                        if back_edge_unwinds(c, 0x002882f8) {
                            return Ok(());
                        }
                        pc = 0x002882f8;
                        continue;
                    }
                    pc = 0x00288334;
                    continue;
                }
                0x00288334 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[SP][0];
                    pc = 0x00288338;
                    continue;
                }
                0x00288338 => {
                    c.cycles += 14;
                    c.g[RA][0] = sx(0x00288340);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x002a7890, 0x00288340)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x8c0));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x8b0));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x8a0));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x890));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x880));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x870));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x860));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x850));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x840));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x830));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x8d0));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x00287fc8, 0x00288370);
                    return Ok(());
                }
            }
        }
    });
}

/// `SSXAudioSystem_SetDebugMode(self, mode)` (0x00288f60): store the debug
/// preset (`+0x6470`) and set the ten channel volumes (0.909 or 0 per preset)
/// plus the arcade SFX sub-channels flag; presets 0..=5 through a jump table at
/// `0x482970`, other modes only store the value.
///
/// The normal entry only, transcribed from the generated body.
pub fn ssx_audio_system_set_debug_mode(c: &mut Cpu) {
    if c.pc != 0x00288f60 {
        return ee::s07::f_00288f60__generated(c);
    }
    guest(c, |c| {
        'b_289380: {
            'b_28937c: {
                c.cycles += 9;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                c.g[V0][0] = (c.g[A1][0] < 0x6u64) as u64;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[S0][0] = c.g[A0][0];
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x28));
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x20));
                let t = c.g[V0][0] == 0u64;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x6470), c.g[A1][0]);
                if t {
                    break 'b_28937c;
                } else {
                    'b_2892cc: {
                        'b_289214: {
                            'b_2891f8: {
                                'b_2891d4: {
                                    'b_289150: {
                                        'b_2890cc: {
                                            'b_289024: {
                                                c.cycles += 7;
                                                c.g[V0][0] = 0x480000;
                                                c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                                                c.g[V0][0] =
                                                    sx((c.g[V0][0] as u32).wrapping_add(0x2970));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                let t = c.g[A0][0] as u32;
                                                match t {
                                                    0x00288fa0 => {
                                                        c.cycles += 33;
                                                        c.lwc1(
                                                            20,
                                                            (c.g[GP][0] as u32)
                                                                .wrapping_sub(0x45f4),
                                                        );
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A1][0] = 0x1;
                                                        c.g[RA][0] = sx(0x00288fb4);
                                                        c.mov_s(12, 20);
                                                        Callee { addr: 0x00287700, f: ee::s07::SSXAudioSystem_SetChannelVolume }.call(c, 0x00288fb4)?;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A1][0] = 0x5;
                                                        c.g[RA][0] = sx(0x00288fc4);
                                                        c.mov_s(12, 20);
                                                        Callee { addr: 0x00287700, f: ee::s07::SSXAudioSystem_SetChannelVolume }.call(c, 0x00288fc4)?;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A1][0] = 0x2;
                                                        c.g[RA][0] = sx(0x00288fd4);
                                                        c.mov_s(12, 20);
                                                        Callee { addr: 0x00287700, f: ee::s07::SSXAudioSystem_SetChannelVolume }.call(c, 0x00288fd4)?;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A1][0] = 0x3;
                                                        c.g[RA][0] = sx(0x00288fe4);
                                                        c.mov_s(12, 20);
                                                        Callee { addr: 0x00287700, f: ee::s07::SSXAudioSystem_SetChannelVolume }.call(c, 0x00288fe4)?;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A1][0] = 0x4;
                                                        c.g[RA][0] = sx(0x00288ff4);
                                                        c.mov_s(12, 20);
                                                        Callee { addr: 0x00287700, f: ee::s07::SSXAudioSystem_SetChannelVolume }.call(c, 0x00288ff4)?;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A1][0] = 0x6;
                                                        c.g[RA][0] = sx(0x00289004);
                                                        c.mov_s(12, 20);
                                                        Callee { addr: 0x00287700, f: ee::s07::SSXAudioSystem_SetChannelVolume }.call(c, 0x00289004)?;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A1][0] = 0x7;
                                                        c.g[RA][0] = sx(0x00289014);
                                                        c.mov_s(12, 20);
                                                        Callee { addr: 0x00287700, f: ee::s07::SSXAudioSystem_SetChannelVolume }.call(c, 0x00289014)?;
                                                        c.g[A0][0] = c.g[S0][0];
                                                        c.g[A1][0] = 0x8;
                                                        let t = true;
                                                        c.mov_s(12, 20);
                                                        if t {
                                                            break 'b_2891d4;
                                                        } else {
                                                            break 'b_289024;
                                                        }
                                                    }
                                                    0x00289024 => {
                                                        break 'b_289024;
                                                    }
                                                    0x002890cc => {
                                                        break 'b_2890cc;
                                                    }
                                                    0x00289150 => {
                                                        break 'b_289150;
                                                    }
                                                    0x00289214 => {
                                                        break 'b_289214;
                                                    }
                                                    0x002892cc => {
                                                        break 'b_2892cc;
                                                    }
                                                    _ => {
                                                        c.jump_out(t, 0x00288f60, 0x00289394);
                                                        return Ok(());
                                                    }
                                                }
                                            }
                                            c.cycles += 42;
                                            c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x45f0));
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[A1][0] = 0x1;
                                            c.mtc1(0, 21);
                                            c.g[RA][0] = sx(0x0028903c);
                                            c.mov_s(12, 20);
                                            Callee {
                                                addr: 0x00287700,
                                                f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                            }
                                            .call(c, 0x0028903c)?;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[A1][0] = 0x5;
                                            c.g[RA][0] = sx(0x0028904c);
                                            c.mov_s(12, 20);
                                            Callee {
                                                addr: 0x00287700,
                                                f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                            }
                                            .call(c, 0x0028904c)?;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[A1][0] = 0x2;
                                            c.g[RA][0] = sx(0x0028905c);
                                            c.mov_s(12, 21);
                                            Callee {
                                                addr: 0x00287700,
                                                f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                            }
                                            .call(c, 0x0028905c)?;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[A1][0] = 0x3;
                                            c.g[RA][0] = sx(0x0028906c);
                                            c.mov_s(12, 21);
                                            Callee {
                                                addr: 0x00287700,
                                                f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                            }
                                            .call(c, 0x0028906c)?;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[A1][0] = 0x4;
                                            c.g[RA][0] = sx(0x0028907c);
                                            c.mov_s(12, 21);
                                            Callee {
                                                addr: 0x00287700,
                                                f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                            }
                                            .call(c, 0x0028907c)?;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[A1][0] = 0x6;
                                            c.g[RA][0] = sx(0x0028908c);
                                            c.mov_s(12, 20);
                                            Callee {
                                                addr: 0x00287700,
                                                f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                            }
                                            .call(c, 0x0028908c)?;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[A1][0] = 0x7;
                                            c.g[RA][0] = sx(0x0028909c);
                                            c.mov_s(12, 20);
                                            Callee {
                                                addr: 0x00287700,
                                                f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                            }
                                            .call(c, 0x0028909c)?;
                                            c.mov_s(12, 20);
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[RA][0] = sx(0x002890ac);
                                            c.g[A1][0] = 0x8;
                                            Callee {
                                                addr: 0x00287700,
                                                f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                            }
                                            .call(c, 0x002890ac)?;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[A1][0] = 0x9;
                                            c.g[RA][0] = sx(0x002890bc);
                                            c.mov_s(12, 21);
                                            Callee {
                                                addr: 0x00287700,
                                                f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                            }
                                            .call(c, 0x002890bc)?;
                                            c.g[A0][0] = c.g[S0][0];
                                            c.g[A1][0] = 0xa;
                                            let t = true;
                                            c.mov_s(12, 21);
                                            if t {
                                                break 'b_2891f8;
                                            } else {
                                                break 'b_2890cc;
                                            }
                                        }
                                        c.cycles += 33;
                                        c.mtc1(0, 12);
                                        c.g[A0][0] = c.g[S0][0];
                                        c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x45ec));
                                        c.g[RA][0] = sx(0x002890e0);
                                        c.g[A1][0] = 0x1;
                                        Callee {
                                            addr: 0x00287700,
                                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                        }
                                        .call(c, 0x002890e0)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0x5;
                                        c.g[RA][0] = sx(0x002890f0);
                                        c.mov_s(12, 20);
                                        Callee {
                                            addr: 0x00287700,
                                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                        }
                                        .call(c, 0x002890f0)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0x2;
                                        c.g[RA][0] = sx(0x00289100);
                                        c.mov_s(12, 20);
                                        Callee {
                                            addr: 0x00287700,
                                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                        }
                                        .call(c, 0x00289100)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0x3;
                                        c.g[RA][0] = sx(0x00289110);
                                        c.mov_s(12, 20);
                                        Callee {
                                            addr: 0x00287700,
                                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                        }
                                        .call(c, 0x00289110)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0x4;
                                        c.g[RA][0] = sx(0x00289120);
                                        c.mov_s(12, 20);
                                        Callee {
                                            addr: 0x00287700,
                                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                        }
                                        .call(c, 0x00289120)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0x6;
                                        c.g[RA][0] = sx(0x00289130);
                                        c.mov_s(12, 20);
                                        Callee {
                                            addr: 0x00287700,
                                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                        }
                                        .call(c, 0x00289130)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0x7;
                                        c.g[RA][0] = sx(0x00289140);
                                        c.mov_s(12, 20);
                                        Callee {
                                            addr: 0x00287700,
                                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                        }
                                        .call(c, 0x00289140)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = 0x8;
                                        let t = true;
                                        c.mov_s(12, 20);
                                        if t {
                                            break 'b_2891d4;
                                        } else {
                                            break 'b_289150;
                                        }
                                    }
                                    c.cycles += 33;
                                    c.mtc1(0, 20);
                                    c.g[A0][0] = c.g[S0][0];
                                    c.lwc1(21, (c.g[GP][0] as u32).wrapping_sub(0x45e8));
                                    c.g[A1][0] = 0x1;
                                    c.g[RA][0] = sx(0x00289168);
                                    c.mov_s(12, 20);
                                    Callee {
                                        addr: 0x00287700,
                                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                    }
                                    .call(c, 0x00289168)?;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = 0x5;
                                    c.g[RA][0] = sx(0x00289178);
                                    c.mov_s(12, 21);
                                    Callee {
                                        addr: 0x00287700,
                                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                    }
                                    .call(c, 0x00289178)?;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = 0x2;
                                    c.g[RA][0] = sx(0x00289188);
                                    c.mov_s(12, 20);
                                    Callee {
                                        addr: 0x00287700,
                                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                    }
                                    .call(c, 0x00289188)?;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = 0x3;
                                    c.g[RA][0] = sx(0x00289198);
                                    c.mov_s(12, 20);
                                    Callee {
                                        addr: 0x00287700,
                                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                    }
                                    .call(c, 0x00289198)?;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = 0x4;
                                    c.g[RA][0] = sx(0x002891a8);
                                    c.mov_s(12, 20);
                                    Callee {
                                        addr: 0x00287700,
                                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                    }
                                    .call(c, 0x002891a8)?;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = 0x6;
                                    c.g[RA][0] = sx(0x002891b8);
                                    c.mov_s(12, 21);
                                    Callee {
                                        addr: 0x00287700,
                                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                    }
                                    .call(c, 0x002891b8)?;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = 0x7;
                                    c.g[RA][0] = sx(0x002891c8);
                                    c.mov_s(12, 21);
                                    Callee {
                                        addr: 0x00287700,
                                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                    }
                                    .call(c, 0x002891c8)?;
                                    c.mov_s(12, 21);
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = 0x8;
                                    break 'b_2891d4;
                                }
                                c.cycles += 9;
                                c.g[RA][0] = sx(0x002891dc);
                                Callee {
                                    addr: 0x00287700,
                                    f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                }
                                .call(c, 0x002891dc)?;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A1][0] = 0x9;
                                c.g[RA][0] = sx(0x002891ec);
                                c.mov_s(12, 20);
                                Callee {
                                    addr: 0x00287700,
                                    f: ee::s07::SSXAudioSystem_SetChannelVolume,
                                }
                                .call(c, 0x002891ec)?;
                                c.g[A0][0] = c.g[S0][0];
                                c.g[A1][0] = 0xa;
                                c.mov_s(12, 20);
                                break 'b_2891f8;
                            }
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x00289200);
                            Callee {
                                addr: 0x00287700,
                                f: ee::s07::SSXAudioSystem_SetChannelVolume,
                            }
                            .call(c, 0x00289200)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x0028920c);
                            c.g[A1][0] = 0x1;
                            Callee {
                                addr: 0x002875d0,
                                f: ee::s07::SSXAudioSystem_SetSfxSubChannelsEnabled,
                            }
                            .call(c, 0x0028920c)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                            if t {
                                break 'b_289380;
                            } else {
                                break 'b_289214;
                            }
                        }
                        c.cycles += 46;
                        c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x45e4));
                        c.g[A0][0] = c.g[S0][0];
                        c.mtc1(0, 20);
                        c.g[RA][0] = sx(0x00289228);
                        c.g[A1][0] = 0x1;
                        Callee {
                            addr: 0x00287700,
                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                        }
                        .call(c, 0x00289228)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0x5;
                        c.g[RA][0] = sx(0x00289238);
                        c.mov_s(12, 20);
                        Callee {
                            addr: 0x00287700,
                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                        }
                        .call(c, 0x00289238)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0x2;
                        c.g[RA][0] = sx(0x00289248);
                        c.mov_s(12, 20);
                        Callee {
                            addr: 0x00287700,
                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                        }
                        .call(c, 0x00289248)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0x3;
                        c.g[RA][0] = sx(0x00289258);
                        c.mov_s(12, 20);
                        Callee {
                            addr: 0x00287700,
                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                        }
                        .call(c, 0x00289258)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0x4;
                        c.g[RA][0] = sx(0x00289268);
                        c.mov_s(12, 20);
                        Callee {
                            addr: 0x00287700,
                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                        }
                        .call(c, 0x00289268)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0x6;
                        c.g[RA][0] = sx(0x00289278);
                        c.mov_s(12, 20);
                        Callee {
                            addr: 0x00287700,
                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                        }
                        .call(c, 0x00289278)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0x7;
                        c.g[RA][0] = sx(0x00289288);
                        c.mov_s(12, 20);
                        Callee {
                            addr: 0x00287700,
                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                        }
                        .call(c, 0x00289288)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0x8;
                        c.g[RA][0] = sx(0x00289298);
                        c.mov_s(12, 20);
                        Callee {
                            addr: 0x00287700,
                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                        }
                        .call(c, 0x00289298)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0x9;
                        c.g[RA][0] = sx(0x002892a8);
                        c.mov_s(12, 20);
                        Callee {
                            addr: 0x00287700,
                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                        }
                        .call(c, 0x002892a8)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0xa;
                        c.g[RA][0] = sx(0x002892b8);
                        c.mov_s(12, 20);
                        Callee {
                            addr: 0x00287700,
                            f: ee::s07::SSXAudioSystem_SetChannelVolume,
                        }
                        .call(c, 0x002892b8)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x002892c4);
                        c.g[A1][0] = 0u64;
                        Callee {
                            addr: 0x002875d0,
                            f: ee::s07::SSXAudioSystem_SetSfxSubChannelsEnabled,
                        }
                        .call(c, 0x002892c4)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                        if t {
                            break 'b_289380;
                        } else {
                            break 'b_2892cc;
                        }
                    }
                    c.cycles += 44;
                    c.mtc1(0, 20);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x1;
                    c.g[RA][0] = sx(0x002892e0);
                    c.mov_s(12, 20);
                    Callee {
                        addr: 0x00287700,
                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                    }
                    .call(c, 0x002892e0)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x5;
                    c.g[RA][0] = sx(0x002892f0);
                    c.mov_s(12, 20);
                    Callee {
                        addr: 0x00287700,
                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                    }
                    .call(c, 0x002892f0)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x2;
                    c.g[RA][0] = sx(0x00289300);
                    c.mov_s(12, 20);
                    Callee {
                        addr: 0x00287700,
                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                    }
                    .call(c, 0x00289300)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x3;
                    c.g[RA][0] = sx(0x00289310);
                    c.mov_s(12, 20);
                    Callee {
                        addr: 0x00287700,
                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                    }
                    .call(c, 0x00289310)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x4;
                    c.g[RA][0] = sx(0x00289320);
                    c.mov_s(12, 20);
                    Callee {
                        addr: 0x00287700,
                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                    }
                    .call(c, 0x00289320)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x6;
                    c.g[RA][0] = sx(0x00289330);
                    c.mov_s(12, 20);
                    Callee {
                        addr: 0x00287700,
                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                    }
                    .call(c, 0x00289330)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x7;
                    c.g[RA][0] = sx(0x00289340);
                    c.mov_s(12, 20);
                    Callee {
                        addr: 0x00287700,
                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                    }
                    .call(c, 0x00289340)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x8;
                    c.g[RA][0] = sx(0x00289350);
                    c.mov_s(12, 20);
                    Callee {
                        addr: 0x00287700,
                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                    }
                    .call(c, 0x00289350)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x9;
                    c.g[RA][0] = sx(0x00289360);
                    c.mov_s(12, 20);
                    Callee {
                        addr: 0x00287700,
                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                    }
                    .call(c, 0x00289360)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0xa;
                    c.g[RA][0] = sx(0x00289370);
                    c.mov_s(12, 20);
                    Callee {
                        addr: 0x00287700,
                        f: ee::s07::SSXAudioSystem_SetChannelVolume,
                    }
                    .call(c, 0x00289370)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0028937c);
                    c.g[A1][0] = 0u64;
                    Callee {
                        addr: 0x002875d0,
                        f: ee::s07::SSXAudioSystem_SetSfxSubChannelsEnabled,
                    }
                    .call(c, 0x0028937c)?;
                    break 'b_28937c;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_289380;
        }
        c.cycles += 5;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x28));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `SSXAudioSystem_ReadPlaylistNames(self)` (0x00289470): when the playlist
/// file exists, read up to 20 section names into `self + 0x60a4` (0x14 bytes
/// each; `+0x6234` is the count, 0 when the file is missing).
///
/// The normal entry only, transcribed from the generated body.
pub fn ssx_audio_system_read_playlist_names(c: &mut Cpu) {
    if c.pc != 0x00289470 {
        return ee::s07::f_00289470__generated(c);
    }
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x840));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x830));
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x820));
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x46c));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x810), c.g[RA][0]);
        c.g[RA][0] = sx(0x00289490);
        call_ptr(c, 0x00317f38, 0x00289490)?;
        let missing = c.g[V0][0] == 0u64;
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x46c));
        if missing {
            // no playlist file (0x00289504): count 0
            c.cycles += 2;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x6234), 0u64);
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x830));
        } else {
            // open the file and read the section names (0x00289498)
            c.cycles += 6;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = 0x100;
            c.g[RA][0] = sx(0x002894a8);
            c.g[S1][0] = 0x14;
            call_ptr(c, 0x002a77f8, 0x002894a8)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x6234), 0u64);
            loop {
                // room left? (0x002894d0)
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6234));
                c.g[V0][0] = ((c.g[V0][0] as i64) < 20i64) as u64;
                let full = c.g[V0][0] == 0u64;
                c.g[A0][0] = c.g[SP][0];
                if full {
                    break;
                }
                // next section (0x002894e0)
                c.cycles += 4;
                c.g[RA][0] = sx(0x002894e8);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x002a7b08, 0x002894e8)?;
                if c.g[V0][0] == 0u64 {
                    // no more sections (0x002894f0)
                    c.cycles += 1;
                    c.g[A0][0] = c.g[SP][0];
                    break;
                }
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6234));
                if back_edge_unwinds(c, 0x002894b0) {
                    return Ok(());
                }
                // store the name at +0x60a4 + count * 0x14 (0x002894b0)
                c.cycles += 8;
                c.g[A1][0] = c.g[V0][0];
                c.mult(0, 4, 4, 17);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60a4));
                c.g[RA][0] = sx(0x002894c4);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, 0x004166f4, 0x002894c4)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x6234));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x6234), c.g[V1][0]);
            }
            // close the file (0x002894f4)
            c.cycles += 4;
            c.g[RA][0] = sx(0x002894fc);
            c.g[A1][0] = 0x2;
            call_ptr(c, 0x002a7890, 0x002894fc)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x830));
        }
        // epilogue (0x0028950c)
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x820));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x810));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x840));
        c.pc = t;
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_dtor (0x285210)
// ---------------------------------------------------------------------------

/// `void SSXAudioSystem::~SSXAudioSystem(int flags)` (0x285210): `this = a0`,
/// `flags = a1` (bit 0: delete the object, bit 1: destroy the virtual bases).
/// Resets the vtable words (music system `+0x5558`, speech system `+0x571c`,
/// object `+0xc`), points the sound-system bases' vtables back at the base
/// tables (`BankManager+0x1d8/+0x4`, `+0xab0`, `+0x1d4`; with `flags == 0` it
/// first builds adjusted stack copies of those tables, gcc 2.95's virtual-base
/// `this` fix-up), unloads the loading-screen bank, removes the user-chunk
/// handler, tears down the speech system (`+0x5560`), the big-file sync (`+0x5728`)
/// and the play history (`+0x5980`), runs the controller destructor and, for
/// `flags & 2`, the stream, bank-manager and sound-system destructors, and for
/// `flags & 1` frees the object.
pub fn ssx_audio_system_dtor(c: &mut Cpu) {
    guest(c, |c| {
        // vtable words, then (flags == 0) the stack copies of the base tables
        c.cycles += 36;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x100));
        c.g[V0][0] = 0x480000;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3688));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.g[S1][0] = c.g[A0][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.g[V1][0] = 0x480000;
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.g[A2][0] = 0x480000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x36d8));
        c.sd((c.g[SP][0] as u32).wrapping_add(0xa0), c.g[RA][0]);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x36b8));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x5558), c.g[V0][0]);
        c.g[A0][0] = 0x480000;
        c.sw((c.g[S1][0] as u32).wrapping_add(0xc), c.g[A2][0]);
        c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3708));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x571c), c.g[V1][0]);
        c.g[A2][0] = 0x480000;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x118));
        c.g[T8][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3648));
        c.g[S3][0] = 0x480000;
        c.g[S4][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5560));
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        c.g[T9][0] = sx((c.g[S3][0] as u32).wrapping_add(0x3728));
        c.g[S2][0] = c.g[A1][0];
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A3][0]);
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x118));
        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
        c.sw((c.g[V0][0] as u32).wrapping_add(0xab0), c.g[T8][0]);
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x118));
        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
        let t = c.g[S2][0] != 0u64;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x1d4), c.g[T9][0]);
        if !t {
            c.cycles += 81;
            c.g[A1][0] = c.ld((c.g[A0][0] as u32).wrapping_add(0x3708));
            c.g[T6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[V1][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x8));
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[A0][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x18));
            c.g[V0][0] = c.ld((c.g[A3][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
            c.sd(c.g[SP][0] as u32, c.g[A1][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x18), c.g[A0][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x118));
            c.g[A1][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            c.g[A0][0] = c.lhu((c.g[A3][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
            c.g[T5][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x3648));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[SP][0]);
            c.g[T4][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x8));
            c.g[T2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x118));
            c.g[T3][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = c.lw(c.g[T2][0] as u32);
            c.g[T1][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x18));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1d8));
            c.g[T0][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x20));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6c90));
            c.g[A3][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x28));
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
            c.g[A2][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x30));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = c.ld((c.g[T8][0] as u32).wrapping_add(0x38));
            c.sh((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
            c.sh((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A1][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[T5][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x28), c.g[T4][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[T3][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x38), c.g[T1][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x40), c.g[T0][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x48), c.g[A3][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A2][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x58), c.g[V1][0]);
            c.g[V0][0] = c.lw(c.g[T2][0] as u32);
            c.g[A3][0] = c.lhu((c.g[T8][0] as u32).wrapping_add(0x30));
            c.sw((c.g[V0][0] as u32).wrapping_add(0xab0), c.g[T6][0]);
            c.g[A2][0] = c.lhu((c.g[T8][0] as u32).wrapping_add(0x8));
            c.g[T7][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x118));
            c.g[A1][0] = c.lhu((c.g[T8][0] as u32).wrapping_add(0x10));
            c.g[V1][0] = c.lw(c.g[T7][0] as u32);
            c.g[A0][0] = c.lhu((c.g[T8][0] as u32).wrapping_add(0x18));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6c98));
            c.g[V0][0] = c.lhu((c.g[T8][0] as u32).wrapping_add(0x28));
            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            c.g[T6][0] = c.ld((c.g[S3][0] as u32).wrapping_add(0x3728));
            c.g[T0][0] = c.ld((c.g[T9][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[T5][0] = c.ld((c.g[T9][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[T4][0] = c.ld((c.g[T9][0] as u32).wrapping_add(0x18));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[T3][0] = c.ld((c.g[T9][0] as u32).wrapping_add(0x20));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[T2][0] = c.ld((c.g[T9][0] as u32).wrapping_add(0x28));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[T1][0] = c.ld((c.g[T9][0] as u32).wrapping_add(0x30));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x68), c.g[T0][0]);
            c.sh((c.g[SP][0] as u32).wrapping_add(0x48), c.g[V0][0]);
            c.sh((c.g[SP][0] as u32).wrapping_add(0x28), c.g[A2][0]);
            c.sh((c.g[SP][0] as u32).wrapping_add(0x30), c.g[A1][0]);
            c.sh((c.g[SP][0] as u32).wrapping_add(0x38), c.g[A0][0]);
            c.sh((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A3][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[T6][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[T5][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x78), c.g[T4][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x80), c.g[T3][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x88), c.g[T2][0]);
            c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[T1][0]);
            c.g[V0][0] = c.lhu((c.g[T9][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = c.lw(c.g[T7][0] as u32);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw((c.g[A0][0] as u32).wrapping_add(0x1d4), c.g[S0][0]);
            c.sh((c.g[SP][0] as u32).wrapping_add(0x68), c.g[V0][0]);
        }
        let frame = Frame { size: 0x100 };
        c.cycles += 10;
        c.g[A0][0] = c.g[S1][0];
        Callee {
            addr: 0x0028_f730,
            f: ee::s07::f_0028f730,
        }
        .call(c, 0x0028_53ec)?;
        c.g[A0][0] = 0x0029_0000;
        c.g[A0][0] = oc1009_base_minus(c.g[A0][0] as u32, 0x69b0);
        call_ptr(c, 0x003b_5a60, 0x0028_53f8)?;
        c.g[A0][0] = c.g[S4][0];
        call_ptr(c, 0x002b_0da0, 0x0028_5400)?;
        c.g[A0][0] = c.lw(oc1009_ea(c.g[S1][0], 0x5728));
        if c.g[A0][0] == 0 {
            c.g[A0][0] = c.lw(oc1009_ea(c.g[S1][0], 0x5980));
        } else {
            c.cycles += 3;
            c.g[A1][0] = 0x64;
            call_ptr(c, 0x003d_edc0, 0x0028_5414)?;
            c.g[A0][0] = c.lw(oc1009_ea(c.g[S1][0], 0x5980));
        }
        c.cycles += 2;
        if c.g[A0][0] == 0 {
            c.g[A0][0] = c.g[S1][0];
        } else {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0028_5428)?;
            c.g[A0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        c.g[A1][0] = 0;
        call_ptr(c, 0x002a_b478, 0x0028_5434)?;
        c.g[V0][0] = c.g[S2][0] & 0x2;
        let no_bases = c.g[V0][0] == 0;
        c.g[V0][0] = c.g[S2][0] & 0x1;
        if !no_bases {
            c.cycles += 13;
            c.g[A0][0] = c.lw(oc1009_ea(c.g[S1][0], 0x118));
            c.g[A1][0] = 0;
            call_ptr(c, 0x002a_a648, 0x0028_544c)?;
            c.g[V0][0] = c.lw(oc1009_ea(c.g[S1][0], 0x118));
            c.g[A1][0] = 0;
            c.g[A0][0] = c.lw(c.g[V0][0] as u32);
            Callee {
                addr: 0x0028_bb10,
                f: ee::s07::f_0028bb10,
            }
            .call(c, 0x0028_545c)?;
            c.g[V1][0] = c.lw(oc1009_ea(c.g[S1][0], 0x118));
            c.g[A1][0] = 0;
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            c.g[A0][0] = c.lw(oc1009_ea(c.g[V0][0], 0x1d8));
            call_ptr(c, 0x002a_dee8, 0x0028_5470)?;
            c.g[V0][0] = c.g[S2][0] & 0x1;
        }
        c.cycles += 2;
        let keep = c.g[V0][0] == 0;
        frame.restore(c, S0, 0xf0);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0028_5484)?;
            frame.restore(c, S0, 0xf0);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0xe0);
        frame.restore(c, S2, 0xd0);
        frame.restore(c, S3, 0xc0);
        frame.restore(c, S4, 0xb0);
        frame.restore_ra(c, 0xa0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_OnRiderFinish (0x286ea0)
// ---------------------------------------------------------------------------

/// `void SSXAudioSystem::OnRiderFinish(Rider* rider)` (0x286ea0): `this = a0`,
/// `rider = a1`. Ignored unless the rider is the one the audio system tracks
/// for its player slot (`rider+0x870`). Reports the finish to the sound
/// controller (`gp+0x410`, mode 2, rider id unless a replay is playing),
/// updates the rider's, race and music state, switches the game music (monitor
/// message `0x24` when the finish is in the racing modes 2/3 and the player
/// is ahead, else message `0xa`), then, for a rider that has not finished
/// (`rider+0x480 == 0`), starts the finish speech (`0x2a3708`) or the
/// celebration (`0x2a34d0`) and finally the finish stinger (`0x2a45c0`) in
/// the plain race mode.
#[allow(dead_code)]
pub fn ssx_audio_system_on_rider_finish(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lw(oc1009_ea(c.g[S1][0], 0x870));
        Callee {
            addr: 0x0028_5d98,
            f: ee::s07::f_00285d98,
        }
        .call(c, 0x0028_6ed0)?;
        if c.g[S1][0] != c.g[V0][0] {
            frame.restore(c, S0, 0x60);
            return oc1009_orf_finish(c, frame);
        }
        c.cycles += 4;
        c.g[A0][0] = c.g[S0][0];
        Callee {
            addr: 0x0028_8ae0,
            f: ee::s07::f_00288ae0,
        }
        .call(c, 0x0028_6ee0)?;
        let replay = c.g[V0][0] != 0;
        c.g[A2][0] = 0;
        if !replay {
            c.cycles += 1;
            c.g[A2][0] = c.lw(oc1009_ea(c.g[S1][0], 0x870));
        }
        c.cycles += 29;
        c.g[A0][0] = c.lw(oc1009_ea(c.g[GP][0], 0x410));
        c.g[A1][0] = 2;
        c.g[S4][0] = 1;
        call_ptr(c, 0x002a_73a8, 0x0028_6efc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0029_6e20, 0x0028_6f08)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0029_7438, 0x0028_6f14)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0029_b3c0, 0x0028_6f20)?;
        c.g[V0][0] = c.lw(oc1009_gp_minus(c, 0x848));
        c.g[V1][0] = c.lw(oc1009_ea(c.g[V0][0], 0xc0));
        c.g[S3][0] = c.lw(c.g[V1][0] as u32);
        c.g[S2][0] = c.lw(oc1009_ea(c.g[V1][0], 0x70));
        call_ptr(c, 0x0014_dc80, 0x0028_6f34)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0028_6f40)?;
        c.g[V1][0] = 0x0053_0000;
        c.g[S5][0] = c.g[V1][0];
        c.g[V1][0] = sx(oc1009_ea(c.g[V1][0], 0x5bc8));
        c.g[V0][0] = c.lb(oc1009_ea(c.g[V1][0], 0x48));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(2));
        c.g[V0][0] = (c.g[V0][0] < 2) as u64;
        let not_race_mode = c.g[V0][0] == 0;
        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S2][0] as i64)) as u64;
        if !not_race_mode {
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                Callee {
                    addr: 0x0028_d960,
                    f: ee::s07::f_0028d960,
                }
                .call(c, 0x0028_6f70)?;
                let music_off = c.g[V0][0] == 0;
                c.g[A2][0] = sx(oc1009_ea(c.g[S0][0], 0x118));
                if !music_off {
                    c.cycles += 8;
                    c.g[V1][0] = c.lw(oc1009_ea(c.g[S0][0], 0x5558));
                    c.g[A1][0] = 0x24;
                    c.g[S4][0] = 0;
                    c.g[A0][0] = c.lh(oc1009_ea(c.g[V1][0], 0x20));
                    c.g[V0][0] = c.lw(oc1009_ea(c.g[V1][0], 0x24));
                    let method = c.g[V0][0] as u32;
                    c.g[A0][0] = sx(oc1009_ea(c.g[A2][0], c.g[A0][0] as u32));
                    call_ptr(c, method, 0x0028_6f94)?;
                    c.sw(oc1009_ea(c.g[S0][0], 0x627c), 0);
                }
            }
        }
        c.cycles += 2;
        if c.g[S4][0] != 0 {
            c.cycles += 4;
            Callee {
                addr: 0x0028_d960,
                f: ee::s07::f_0028d960,
            }
            .call(c, 0x0028_6fa8)?;
            let music_off = c.g[V0][0] == 0;
            c.g[A2][0] = sx(oc1009_ea(c.g[S0][0], 0x118));
            if !music_off {
                c.cycles += 6;
                c.g[V1][0] = c.lw(oc1009_ea(c.g[S0][0], 0x5558));
                c.g[A1][0] = 0xa;
                c.g[A0][0] = c.lh(oc1009_ea(c.g[V1][0], 0x20));
                c.g[V0][0] = c.lw(oc1009_ea(c.g[V1][0], 0x24));
                let method = c.g[V0][0] as u32;
                c.g[A0][0] = sx(oc1009_ea(c.g[A2][0], c.g[A0][0] as u32));
                call_ptr(c, method, 0x0028_6fc8)?;
            }
        }
        // the rider's own finish sound
        c.cycles += 8;
        call_ptr(c, 0x0014_dc80, 0x0028_6fd0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0028_6fdc)?;
        c.g[V1][0] = c.lw(oc1009_ea(c.g[S1][0], 0x480));
        let already = c.g[V1][0] != 0;
        c.g[V0][0] = sx(oc1009_ea(c.g[S5][0], 0x5bc8));
        if !already {
            // Which start: the finish speech (two code paths, `a` and `b`, that
            // differ in cycles and return address) or the celebration.
            #[derive(PartialEq)]
            enum Start {
                SpeechA,
                SpeechB,
                Celebrate,
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002a_4078, 0x0028_6ff0)?;
            let tracked = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S1][0];
            let start;
            if tracked {
                start = Start::SpeechA;
            } else {
                c.cycles += 5;
                c.g[V0][0] = sx(oc1009_ea(c.g[S5][0], 0x5bc8));
                c.g[A0][0] = 2;
                c.g[V1][0] = c.lb(oc1009_ea(c.g[V0][0], 0x49));
                let not_mode2 = c.g[V1][0] != c.g[A0][0];
                c.g[V0][0] = ((c.g[S3][0] as i64) < 3) as u64;
                if !not_mode2 {
                    c.cycles += 6;
                    Callee {
                        addr: 0x0028_b1d8,
                        f: ee::s07::f_0028b1d8,
                    }
                    .call(c, 0x0028_7014)?;
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x0012_a250, 0x0028_701c)?;
                    let no_race = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[S1][0];
                    start = if no_race {
                        Start::Celebrate
                    } else {
                        Start::SpeechA
                    };
                } else {
                    c.cycles += 2;
                    let few_players = c.g[V0][0] != 0;
                    c.g[A1][0] = c.g[S1][0];
                    start = if few_players {
                        Start::Celebrate
                    } else {
                        Start::SpeechB
                    };
                }
            }
            match start {
                Start::SpeechA => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x002a_3708, 0x0028_702c)?;
                }
                Start::Celebrate => {
                    c.cycles += 7;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = 0;
                    c.g[A3][0] = u64::MAX;
                    call_ptr(c, 0x002a_34d0, 0x0028_7050)?;
                }
                Start::SpeechB => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x002a_3708, 0x0028_7060)?;
                }
            }
            c.g[V0][0] = sx(oc1009_ea(c.g[S5][0], 0x5bc8));
        }
        c.cycles += 3;
        c.g[V1][0] = c.lb(oc1009_ea(c.g[V0][0], 0x49));
        if c.g[V1][0] != 0 {
            frame.restore(c, S0, 0x60);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002a_45c0, 0x0028_7078)?;
            frame.restore(c, S0, 0x60);
        }
        oc1009_orf_finish(c, frame)
    });
}

/// Epilogue of `OnRiderFinish` (`s0` is restored by the caller).
pub(crate) fn oc1009_orf_finish(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 8;
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore(c, S5, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// Pops one entry of the ring at `self + base` after the empty check failed
/// (`v1 = self + base`, `a1` = the tail index): leaves the entry in `v0`,
/// advances (and wraps) the index at `+0xc`. 10 cycles, +1 on the wrap.
#[inline(always)]
pub(crate) fn oc1010_ring_pop(c: &mut Cpu, s1: u32, base: u32) {
    c.cycles += 10;
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = c.lw(s1.wrapping_add(base));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[V0][0] = c.lw(c.g[V0][0] as u32);
    c.g[V1][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
    let in_range = c.g[V1][0] != 0;
    c.sw(s1.wrapping_add(base).wrapping_add(0xc), c.g[A1][0]);
    if !in_range {
        c.cycles += 1;
        c.sw(s1.wrapping_add(base).wrapping_add(0xc), 0);
    }
}

/// Pops the ring at `self + base` (second to fourth rings of the async
/// system: names, kinds): `-1` when empty, else the entry. Returns it.
#[inline(always)]
pub(crate) fn oc1010_pop_ring(c: &mut Cpu, s1: u32, base: u32) -> u64 {
    c.cycles += 4;
    c.g[V0][0] = c.lw(s1.wrapping_add(base).wrapping_add(0x8));
    c.g[A1][0] = c.lw(s1.wrapping_add(base).wrapping_add(0xc));
    let nonempty = c.g[V0][0] != c.g[A1][0];
    c.g[V1][0] = sx(s1.wrapping_add(base));
    if !nonempty {
        c.cycles += 2;
        return 0xffff_ffff_ffff_ffff;
    }
    oc1010_ring_pop(c, s1, base);
    c.cycles += 1;
    c.g[V0][0]
}

/// `void SndLoadQueue_FinishCurrent(SndLoadQueue* self, int flush)` (0x289df0),
/// the C++ `cAsyncSys_DoCancel`: pops one entry of each of the four request
/// rings (user handle +0x18c, name slot +0x19c, kind +0x1ac, flags +0x1bc),
/// and when a load is in flight (+4) asks the async file layer for its result
/// and notifies through the vtable (+0x1d4, entry at +0x28/+0x2c) that the
/// bank finished (kinds 0 and 2: result 1 means ok; kind 1 uses the kind
/// itself), then clears busy. With `flush` the tails are set to the heads.
/// Finally starts the next queued load.
pub fn snd_load_queue_finish_current(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S4, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S5, 0x10);
        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x18c));
        frame.save(c, S0, 0x60);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save_ra(c, 0);
        let this = c.g[S1][0] as u32;
        c.g[A1][0] = c.lw(this.wrapping_add(0x198));
        c.g[V0][0] = c.lw(this.wrapping_add(0x194));
        let empty = c.g[V0][0] == c.g[A1][0];
        c.g[S4][0] = 0xffff_ffff_ffff_ffff;
        if !empty {
            oc1010_ring_pop(c, this, 0x18c);
            c.cycles += 1;
            c.g[S4][0] = c.g[V0][0];
        }
        c.g[S3][0] = oc1010_pop_ring(c, this, 0x19c);
        c.g[S2][0] = oc1010_pop_ring(c, this, 0x1ac);
        // Fourth ring (flags): popped and dropped.
        c.cycles += 4;
        c.g[V0][0] = c.lw(this.wrapping_add(0x1c4));
        c.g[A0][0] = c.lw(this.wrapping_add(0x1c8));
        let empty = c.g[V0][0] == c.g[A0][0];
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
        if !empty {
            c.cycles += 5;
            c.g[V1][0] = c.lw(this.wrapping_add(0x1bc));
            c.sw(this.wrapping_add(0x1c8), c.g[V0][0]);
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.sw(this.wrapping_add(0x1c8), 0);
            }
        }
        // The in-flight load: finish it according to its kind.
        c.cycles += 3;
        c.g[V0][0] = c.lw(this.wrapping_add(0x4));
        let idle = c.g[V0][0] == 0;
        c.g[V0][0] = 0x1;
        if !idle {
            c.cycles += 2;
            let kind_is_1 = c.g[S2][0] == c.g[V0][0];
            c.g[V0][0] = ((c.g[S2][0] as i64) < 2) as u64;
            // true: kind 0 or 2 arm (copy A); false: kind 1 arm (copy B).
            let arm_a;
            if kind_is_1 {
                arm_a = None;
            } else {
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = 0x2;
                    c.cycles += 2;
                    arm_a = if c.g[S2][0] != c.g[V0][0] {
                        Some(false)
                    } else {
                        Some(true)
                    };
                } else {
                    c.cycles += 2;
                    if c.g[S2][0] == 0 {
                        arm_a = Some(true);
                    } else {
                        c.cycles += 2;
                        arm_a = Some(false);
                    }
                }
            }
            match arm_a {
                Some(true) => {
                    c.cycles += 25;
                    c.g[A0][0] = c.lw(this);
                    call_ptr(c, 0x003d_faf0, 0x0028_9f4c)?;
                    c.g[A0][0] = c.lw(this);
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 0;
                    call_ptr(c, 0x003d_f9d8, 0x0028_9f60)?;
                    c.g[V1][0] = c.lw(this.wrapping_add(0x1d4));
                    c.g[T0][0] = sx((c.g[S3][0] as u32) << 6);
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = 0x1;
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.g[S0][0] ^= 0x1;
                    c.g[T3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                    c.g[A3][0] = 0x2;
                    if c.g[S0][0] == 0 {
                        c.g[A3][0] = c.g[V0][0];
                    }
                    c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
                    c.g[A1][0] = c.g[S4][0];
                    c.g[T0][0] = sx(this.wrapping_add(c.g[T0][0] as u32));
                    c.g[A2][0] = 0xffff_ffff_ffff_ffff;
                    c.g[T1][0] = 0;
                    let target = c.g[T3][0] as u32;
                    c.g[T2][0] = 0;
                    call_ptr(c, target, 0x0028_9fa0)?;
                    c.sw(this.wrapping_add(0x4), 0);
                }
                None => {
                    c.cycles += 23;
                    c.g[A0][0] = c.lw(this);
                    call_ptr(c, 0x003d_faf0, 0x0028_9fb0)?;
                    c.g[A0][0] = c.lw(this);
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 0;
                    call_ptr(c, 0x003d_f9d8, 0x0028_9fc4)?;
                    c.g[V1][0] = c.lw(this.wrapping_add(0x1d4));
                    c.g[T0][0] = sx((c.g[S3][0] as u32) << 6);
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0xc));
                    c.g[S0][0] ^= 0x1;
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
                    c.g[A3][0] = 0x2;
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
                    if c.g[S0][0] == 0 {
                        c.g[A3][0] = c.g[S2][0];
                    }
                    c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
                    c.g[A1][0] = c.g[S4][0];
                    c.g[T0][0] = sx(this.wrapping_add(c.g[T0][0] as u32));
                    c.g[A2][0] = 0xffff_ffff_ffff_ffff;
                    c.g[T1][0] = 0;
                    let target = c.g[V0][0] as u32;
                    c.g[T2][0] = 0;
                    call_ptr(c, target, 0x0028_a000)?;
                    c.sw(this.wrapping_add(0x4), 0);
                }
                Some(false) => {}
            }
        }
        c.cycles += 2;
        if c.g[S5][0] != 0 {
            c.cycles += 8;
            c.g[V0][0] = c.lw(this.wrapping_add(0x194));
            c.g[V1][0] = c.lw(this.wrapping_add(0x1a4));
            c.g[A0][0] = c.lw(this.wrapping_add(0x1b4));
            c.g[A1][0] = c.lw(this.wrapping_add(0x1c4));
            c.sw(this.wrapping_add(0x198), c.g[V0][0]);
            c.sw(this.wrapping_add(0x1a8), c.g[V1][0]);
            c.sw(this.wrapping_add(0x1b8), c.g[A0][0]);
            c.sw(this.wrapping_add(0x1c8), c.g[A1][0]);
        }
        c.cycles += 11;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0028_a298, 0x0028_a034)?;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `SSXAudioSystem::SetMusicEvent1(self)` (0x2861d0, 48 B): calls the audio
/// object's music callback (the function pointer at `self->mpAudio+0x24`)
/// with `self+0x118 + lh(mpAudio+0x20)` as its object and 1 as its argument.
pub fn ssx_audio_system_set_music_event1(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let f = Frame::open(c, 0x10);
        c.g[A1][0] = 1;
        f.save_ra(c, 0);
        let this = c.g[A0][0] as u32;
        c.g[V1][0] = Addr(this).off(0x5558).lw(c);
        c.g[A0][0] = sx(this.wrapping_add(0x118));
        let obj = c.g[V1][0] as u32;
        c.g[A2][0] = Addr(obj).off(0x20).lh(c);
        c.g[V0][0] = Addr(obj).off(0x24).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        call_ptr(c, target, 0x0028_61f4)?;
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// One call of the exit chain: `obj->vt->fn(obj + vt->delta, 3)`, where the
/// caller set A1 = 3. Leaves V0 = the vtable, V1 = the function, A0 = the
/// object argument, as the compiled code does.
pub(crate) fn ssx_exit_call(c: &mut Cpu, obj: u32, ret: u32) -> Flow {
    c.g[V0][0] = Addr(obj).off(4).lw(c);
    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// One later link of the exit chain: `obj = self->load` (into A2), the link
/// `self->clear` is zeroed after the load, and a non-null object gets the
/// [`ssx_exit_call`] (with cycles 3 before the load and 6 on the call path).
pub(crate) fn ssx_exit_link(c: &mut Cpu, this: u32, load: u32, clear: u32, ret: u32) -> Flow {
    c.cycles += 3;
    c.g[A2][0] = Addr(this).off(load).lw(c);
    let empty = c.g[A2][0] == 0;
    Addr(this).off(clear).sw(c, 0);
    if !empty {
        c.cycles += 6;
        c.g[A1][0] = 3;
        let obj = c.g[A2][0] as u32;
        ssx_exit_call(c, obj, ret)?;
    }
    Ok(())
}

/// `SSXAudioSystem::ExitGame(self)` (0x286a80, 380 B): stops the audio
/// subsystems in order (resume, mode, music, loops, speech, object loops,
/// mix preset, timers), calls the speech callback and the speech stop.
pub fn ssx_audio_system_exit_game(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let f = Frame::open(c, 0x20);
        f.save(c, S0, 0x10);
        f.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        call_ptr(c, 0x0028_9bb8, 0x0028_6a94)?;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0028_5be0, 0x0028_6aa0)?;
        c.g[A0][0] = sx(this.wrapping_add(0x118));
        call_ptr(c, 0x002b_3ac0, 0x0028_6aa8)?;
        c.mtc1(0, 12);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0029_d678, 0x0028_6ab4)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002a_4550, 0x0028_6abc)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0029_2b48, 0x0028_6ac4)?;

        // The object chain: the first link has no clear, the rest zero
        // their own link after loading it.
        c.g[A2][0] = Addr(this).off(0x625c).lw(c);
        let empty = c.g[A2][0] == 0;
        c.g[A1][0] = 3;
        if !empty {
            c.cycles += 5;
            let obj = c.g[A2][0] as u32;
            ssx_exit_call(c, obj, 0x0028_6ae4)?;
        }
        ssx_exit_link(c, this, 0x6260, 0x625c, 0x0028_6b08)?;
        ssx_exit_link(c, this, 0x5fa0, 0x6260, 0x0028_6b2c)?;
        ssx_exit_link(c, this, 0x5fa4, 0x5fa0, 0x0028_6b50)?;
        ssx_exit_link(c, this, 0x6c80, 0x5fa4, 0x0028_6b74)?;
        ssx_exit_link(c, this, 0x6c84, 0x6c80, 0x0028_6b98)?;

        c.cycles += 25;
        Addr(this).off(0x6c84).sw(c, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0029_80b0, 0x0028_6ba4)?;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0028_83b0, 0x0028_6bb0)?;

        // mgr = *self->mpMgr; mgr->timers stop with mgr+0x1d8.
        c.g[V0][0] = Addr(this).off(0x118).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
        call_ptr(c, 0x002a_dda0, 0x0028_6bc0)?;

        // The speech callback: (mgr+0x1d8) + lh(cfg+0x10), cfg+0x14.
        c.g[V0][0] = Addr(this).off(0x118).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0xab0).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x10).lh(c);
        c.g[A1][0] = Addr(c.g[V1][0] as u32).off(0x14).lw(c);
        let target = c.g[A1][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        call_ptr(c, target, 0x0028_6be0)?;

        c.g[A0][0] = sx(this.wrapping_add(0x5560));
        c.g[A1][0] = 0;
        call_ptr(c, 0x002b_11b0, 0x0028_6bec)?;

        f.restore(c, S0, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// The probe of the unload chain: `mgr = *(self+0x118)`, `cfg = mgr->0xacc`,
/// `busy = (cfg[off] ^ 1) != 0`, the call's A0 = `mgr + 0x1d8`; not busy takes
/// the next word (`cfg[off_next]`) into A1, busy sets A1 = -1.
pub(crate) fn ssx_unload_probe(c: &mut Cpu, this: u32, off: u32, off_next: u32) {
    c.g[V0][0] = Addr(this).off(0x118).lw(c);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
    c.g[A1][0] = Addr(c.g[V1][0] as u32).off(0xacc).lw(c);
    c.g[V0][0] = Addr(c.g[A1][0] as u32).off(off).lw(c) ^ 1;
    let busy = c.g[V0][0] != 0;
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1d8));
    if busy {
        c.cycles += 1;
        c.g[A1][0] = u64::MAX;
    } else {
        c.cycles += 2;
        c.g[A1][0] = Addr(c.g[A1][0] as u32).off(off_next).lw(c);
    }
}

/// The body of `SSXAudioSystem::UnloadMemoryBank` once the bank's handle
/// matches: bank 0 stops its music with the probe at 0x300, the other banks
/// stop their loops with the probe at 0x360, then the bank's object (the table
/// at `self+0x6c70`) is unloaded and its handle slot cleared.
pub(crate) fn ssx_unload_matched(c: &mut Cpu, this: u32) -> Flow {
    let s2 = sx((c.g[S3][0] as u32) << 2);
    c.cycles += 2;
    if c.g[S3][0] == 0 {
        c.cycles += 7;
        ssx_unload_probe(c, this, 0x300, 0x304);
        c.cycles += 5;
        c.mtc1(0, 12);
        c.g[S2][0] = s2;
        call_ptr(c, 0x002a_d650, 0x0028_6d8c)?;
    } else {
        c.cycles += 10;
        c.mtc1(0, 12);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0029_d678, 0x0028_6da0)?;
        ssx_unload_probe(c, this, 0x360, 0x364);
        c.cycles += 4;
        c.mtc1(0, 12);
        c.g[S2][0] = s2;
        call_ptr(c, 0x002a_d650, 0x0028_6dd4)?;
    }
    c.g[V0][0] = sx(this.wrapping_add(0x6c70));

    // The bank's object slot: unload it and clear both words.
    c.cycles += 4;
    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
    let slot = c.g[S1][0] as u32;
    c.g[V1][0] = Addr(slot).lw(c);
    let empty = c.g[V1][0] == 0;
    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(8));
    if !empty {
        c.cycles += 7;
        c.g[V0][0] = Addr(this).off(0x118).lw(c);
        c.g[RA][0] = sx(0x0028_6df4);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        call_ptr(c, 0x0028_be90, 0x0028_6df4)?;
        c.g[V1][0] = sx(this.wrapping_add(c.g[S2][0] as u32));
        Addr(slot).sw(c, 0);
        c.g[V0][0] = u64::MAX;
        let v = c.g[V0][0];
        let handle = c.g[V1][0] as u32;
        Addr(handle).off(0x6c78).sw(c, v);
    }
    Ok(())
}

/// `SSXAudioSystem::UnloadMemoryBank(self = a0, handle = a1)` (0x286d18,
/// 264 B): `slot = handle >> 8`, and only if `self->slot[slot] == handle & 0xff`
/// (the word at `self + 4*slot + 0x6c78`) the bank is unloaded.
pub fn ssx_audio_system_unload_memory_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let f = Frame::open(c, 0x50);
        f.save(c, S3, 0x10);
        f.save(c, S2, 0x20);
        f.save(c, S1, 0x30);
        f.save(c, S0, 0x40);
        c.g[S3][0] = sx((c.g[A1][0] as u32) >> 8);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
        f.save_ra(c, 0);
        c.g[V0][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
        c.g[A1][0] &= 0xff;
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x6c78).lw(c);
        if c.g[V1][0] == c.g[A1][0] {
            ssx_unload_matched(c, this)?;
            c.cycles += 1;
        }
        f.restore(c, S0, 0x40);

        // Epilogue.
        c.cycles += 6;
        f.restore(c, S1, 0x30);
        f.restore(c, S2, 0x20);
        f.restore(c, S3, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem::OnQuickRestart(self)` (0x2870a0, 104 B): resets the race
/// sounds, and unless the event is in mode 4 and the tracked sound is not
/// playing, tracks a new sound (`TrackSound(self, 0, 0, -1, 0, 0)`).
pub fn ssx_audio_system_on_quick_restart(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let f = Frame::open(c, 0x20);
        f.save(c, S0, 0x10);
        f.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0028_71b0, 0x0028_70b4)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002a_4040, 0x0028_70bc)?;
        if c.g[V0][0] == 0 {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0029_5028, 0x0028_70d4)?;
            if c.g[V0][0] == 0 {
                c.cycles += 8;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                c.g[A3][0] = u64::MAX;
                c.g[T0][0] = 0;
                c.g[T1][0] = 0;
                call_ptr(c, 0x0029_5628, 0x0028_70f8)?;
            }
        }
        f.restore(c, S0, 0x10);

        // Epilogue.
        c.cycles += 3;
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem::StartPreRaceMusic(self)` (0x2872a8, 304 B): reads the
/// audio game module's config bytes (0x535bc8+0x48/0x49/0x4a); for the two
/// valid ranges either plays a random song (no song loaded) or asks the
/// speech system to say the DJ line, then records the played sound ids.
pub fn ssx_audio_system_start_pre_race_music(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let f = Frame::open(c, 0x40);
        f.save(c, S0, 0x30);
        f.save(c, S1, 0x20);
        f.save(c, S2, 0x10);
        f.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        call_ptr(c, 0x0028_b1c8, 0x0028_72c4)?;
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(8).lw(c);
        let v = c.g[V0][0];
        Addr(this).off(0x582c).sw(c, v);
        call_ptr(c, 0x0014_dc80, 0x0028_72d4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0028_72e0)?;

        c.g[V1][0] = 0x0053_0000;
        c.g[S2][0] = c.g[V1][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x48).lb(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(5));
        c.g[V0][0] = (c.g[V0][0] < 2) as u64;

        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(this).off(0x530).lw(c);
            let loaded = c.g[V0][0] != 0;
            c.g[S1][0] = sx(this.wrapping_add(0x118));
            if loaded {
                c.cycles += 8;
                c.g[V1][0] = Addr(this).off(0x5558).lw(c);
                c.g[A1][0] = 0;
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x20).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x24).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0028_7324)?;
                c.g[A0][0] = c.g[S0][0];
            } else {
                c.cycles += 9;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0028_d488, 0x0028_7334)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                c.g[A3][0] = u64::MAX;
                c.g[T0][0] = 0;
                call_ptr(c, 0x0028_cf98, 0x0028_734c)?;
                c.g[A0][0] = c.g[S0][0];
            }

            c.cycles += 5;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0029_5028, 0x0028_735c)?;
            if c.g[V0][0] == 0 {
                let mut skip_tail = false;
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x5bc8));
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x4a).lb(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
                c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                let in_range = c.g[V0][0] != 0;
                c.g[V0][0] = 2;
                if in_range {
                    c.cycles += 3;
                    c.g[V1][0] = Addr(c.g[V1][0] as u32).off(0x49).lb(c);
                    let is_two = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 1;
                    if is_two {
                        c.cycles += 7;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0;
                        c.g[A2][0] = 0;
                        call_ptr(c, 0x002a_26f0, 0x0028_73a0)?;
                        c.g[A0][0] = sx(this.wrapping_add(0x5560));
                        c.g[A1][0] = 0;
                        call_ptr(c, 0x002b_1758, 0x0028_73ac)?;
                    } else {
                        c.cycles += 2;
                        let v = c.g[V0][0];
                        Addr(this).off(0x5788).sw(c, v);
                        skip_tail = true;
                    }
                }
                if !skip_tail {
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                }

                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                let v = c.g[V0][0];
                Addr(this).off(0x5774).sw(c, v);
                call_ptr(c, 0x002b_4908, 0x0028_73bc)?;
                let v = c.g[V0][0];
                Addr(this).off(0x5778).sw(c, v);
            }
        }

        f.restore(c, S0, 0x30);

        // Epilogue.
        c.cycles += 5;
        f.restore(c, S1, 0x20);
        f.restore(c, S2, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_IncChannelVolume(SSXAudioSystem* self, int ch)` (0x2879b0):
/// raise channel `ch`'s volume (record at +0x62bc, 16 bytes per channel) by
/// the 0.0909 step at gp-0x4628 and re-apply it through
/// `SSXAudioSystem_SetChannelVolume`.
pub fn ssx_audio_system_inc_channel_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 4);
        frame.save_ra(c, 0);
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let step = Addr((c.g[GP][0] as u32).wrapping_sub(0x4628)).read_fpu(c);
        c.f[0] = step.0;
        let volume = Addr((c.g[V0][0] as u32).wrapping_add(0x62bc)).read_fpu(c);
        c.f[12] = (volume + step).0;
        Callee {
            addr: 0x0028_7700,
            f: ee::s07::SSXAudioSystem_SetChannelVolume,
        }
        .call(c, 0x0028_79d0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_DecChannelVolume(SSXAudioSystem* self, int ch)` (0x2879e0):
/// the counterpart of [`ssx_audio_system_inc_channel_volume`], lowering the
/// volume by the step at gp-0x4624.
pub fn ssx_audio_system_dec_channel_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 4);
        frame.save_ra(c, 0);
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        let step = Addr((c.g[GP][0] as u32).wrapping_sub(0x4624)).read_fpu(c);
        c.f[0] = step.0;
        let volume = Addr((c.g[V0][0] as u32).wrapping_add(0x62bc)).read_fpu(c);
        c.f[12] = (volume - step).0;
        Callee {
            addr: 0x0028_7700,
            f: ee::s07::SSXAudioSystem_SetChannelVolume,
        }
        .call(c, 0x0028_7a00)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_PrevDebugMode(SSXAudioSystem* self)` (0x289398): step the
/// debug preset (+0x6470) down, wrapping from 0 to 5, and apply it through
/// `SSXAudioSystem_SetDebugMode`.
pub fn ssx_audio_system_prev_debug_mode(c: &mut Cpu) {
    guest(c, |c| {
        let obj = c.g[A0][0] as u32;
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(obj.wrapping_add(0x6470)).lw(c);
        let stepped = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        if !stepped {
            c.cycles += 1;
            c.g[V0][0] = 5;
        }
        c.cycles += 6;
        let preset = c.g[V0][0];
        Addr(obj.wrapping_add(0x6470)).sw(c, preset);
        c.g[RA][0] = sx(0x0028_93bc);
        c.g[A1][0] = Addr(obj.wrapping_add(0x6470)).lw(c);
        Callee {
            addr: 0x0028_8f60,
            f: ee::s07::SSXAudioSystem_SetDebugMode,
        }
        .call(c, 0x0028_93bc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_NextDebugMode(SSXAudioSystem* self)` (0x2893c8): step the
/// debug preset (+0x6470) up, wrapping from 6 to 0, and apply it through
/// `SSXAudioSystem_SetDebugMode`.
pub fn ssx_audio_system_next_debug_mode(c: &mut Cpu) {
    guest(c, |c| {
        let obj = c.g[A0][0] as u32;
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 6;
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(obj.wrapping_add(0x6470)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        let more = c.g[V0][0] != c.g[V1][0];
        let stepped = c.g[V0][0];
        Addr(obj.wrapping_add(0x6470)).sw(c, stepped);
        if !more {
            c.cycles += 1;
            Addr(obj.wrapping_add(0x6470)).sw(c, 0u64);
        }
        c.cycles += 5;
        c.g[RA][0] = sx(0x0028_93f0);
        c.g[A1][0] = Addr(obj.wrapping_add(0x6470)).lw(c);
        Callee {
            addr: 0x0028_8f60,
            f: ee::s07::SSXAudioSystem_SetDebugMode,
        }
        .call(c, 0x0028_93f0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_ScheduleTriggerTimer(SSXAudioSystem* self)` (0x289400):
/// arm the 250-tick trigger timer on the bank monitor (`self->+0x118->*` +0x1d8)
/// with the callback `SSXAudioSystem_TriggerTimerCallback` (the 8-byte pointer at
/// 0x4a3600 in the game image).
pub fn ssx_audio_system_schedule_trigger_timer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 0x004a_0000;
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xfa;
        c.ldl(A2, (c.g[V1][0] as u32).wrapping_add(0x3607));
        c.ldr(A2, (c.g[V1][0] as u32).wrapping_add(0x3600));
        c.g[A3][0] = 0;
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x118)).lw(c);
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        c.g[T2][0] = 0;
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[T3][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
        call_ptr(c, 0x002a_dca0, 0x0028_943c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_TriggerTimerCallback(void)` (0x289448): the timer's
/// callback, `WorldTriggerManager` monitor pass (gp+0x21e4, one, no extra) and
/// result 1.
pub fn ssx_audio_system_trigger_timer_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x21e4)).lw(c);
        frame.save_ra(c, 0);
        c.g[A1][0] = 1;
        c.g[A2][0] = 0;
        call_ptr(c, 0x002b_6950, 0x0028_9460)?;
        c.g[V0][0] = 1;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_EmptyCrowdHandler(void)` (0x289680): an empty
/// callback; it only returns.
pub fn ssx_audio_system_empty_crowd_handler(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void SSXAudioSystem_ApplyMusicTriggerPaint(SSXAudioSystem* self, Rider* rider, Obj* obj)`
/// (0x2899f8). When `FindRider(self, -1)` is `rider` and `obj` is set, `obj`'s
/// vtable method (`+4 -> +0x24`) gives a type code: 0x1f and 0x20 choose a
/// song through `MusicPlayer_GetSongTag` and `SSXAudioSystem_PlayMusic`
/// (0x65 / 0x66 end without playing), 0x21 ends, other codes call the
/// method at `self + 0x5558`'s vtable.
pub fn ssx_audio_system_apply_music_trigger_paint(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        'b_289ad8: {
            c.g[A1][0] = u64::MAX;
            Callee {
                addr: 0x0028_5d98,
                f: ee::s07::SSXAudioSystem_FindRider,
            }
            .call(c, 0x0028_9a20)?;
            if c.g[S0][0] != c.g[V0][0] {
                frame.restore(c, S0, 0x30);
                break 'b_289ad8;
            }
            c.cycles += 2;
            if c.g[S1][0] == 0 {
                frame.restore(c, S0, 0x30);
                break 'b_289ad8;
            }
            'b_289ad4: {
                c.cycles += 9;
                c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x4)).lw(c);
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x20)).lh(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x24)).lw(c);
                let method = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, method, 0x0028_9a44)?;
                c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[V0][0] = u64::MAX;
                let t = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = 0x1f;
                if t {
                    break 'b_289ad4;
                }
                'b_289ab8: {
                    'b_289aa8: {
                        'b_289a80: {
                            c.cycles += 2;
                            let t = c.g[A1][0] != c.g[V0][0];
                            c.g[V0][0] = 0x20;
                            if t {
                                break 'b_289a80;
                            }
                            c.cycles += 5;
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x118));
                            call_ptr(c, 0x002b_49e0, 0x0028_9a64)?;
                            c.g[V1][0] = 0x65;
                            let t = c.g[V0][0] == c.g[V1][0];
                            c.g[A1][0] = 0;
                            if t {
                                break 'b_289ad4;
                            }
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[A2][0] = 0;
                            c.g[A3][0] = 0x1;
                            break 'b_289aa8;
                        }
                        c.cycles += 2;
                        let t = c.g[A1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x21;
                        if t {
                            break 'b_289ab8;
                        }
                        c.cycles += 5;
                        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x118));
                        call_ptr(c, 0x002b_49e0, 0x0028_9a90)?;
                        c.g[V1][0] = 0x66;
                        let t = c.g[V0][0] == c.g[V1][0];
                        c.g[A1][0] = 0;
                        if t {
                            break 'b_289ad4;
                        }
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A2][0] = 0;
                        c.g[A3][0] = 0x2;
                        break 'b_289aa8;
                    }
                    c.cycles += 4;
                    c.g[T0][0] = 0;
                    Callee {
                        addr: 0x0028_cf98,
                        f: ee::s07::SSXAudioSystem_PlayMusic,
                    }
                    .call(c, 0x0028_9ab0)?;
                    frame.restore(c, S0, 0x30);
                    break 'b_289ad8;
                }
                c.cycles += 2;
                let t = c.g[A1][0] == c.g[V0][0];
                c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x118));
                if t {
                    break 'b_289ad4;
                }
                c.cycles += 5;
                c.g[V1][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x5558)).lw(c);
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x20)).lh(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x24)).lw(c);
                let method = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, method, 0x0028_9ad4)?;
                break 'b_289ad4;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
            break 'b_289ad8;
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem_KillRiderTimers` (0x292ae8).
pub(crate) const RIDER_RELEASE_KILL_TIMERS_CALLEE: Callee = Callee {
    addr: 0x0029_2ae8,
    f: ee::s07::SSXAudioSystem_KillRiderTimers,
};

/// `SSXAudioSystem_OutputModeToOption` (0x289d60).
pub(crate) const OUTPUT_MODE_TO_OPTION_CALLEE: Callee = Callee {
    addr: 0x0028_9d60,
    f: ee::s07::SSXAudioSystem_OutputModeToOption,
};

/// `void SSXAudioSystem_RiderRelease(SSXAudioSystem* self)` (0x289af8):
/// stop the rider timers by forwarding to `SSXAudioSystem_KillRiderTimers`.
pub fn ssx_audio_system_rider_release(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        RIDER_RELEASE_KILL_TIMERS_CALLEE.call(c, 0x0028_9b08)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SSXAudioSystem_GetOutputModeOption(SSXAudioSystem* self)` (0x289dc0):
/// the option index of the output mode stored at `self+0x62b0`.
pub fn ssx_audio_system_get_output_mode_option(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(crate::abi::arg(c, 0).wrapping_add(0x62b0)).lw(c);
        OUTPUT_MODE_TO_OPTION_CALLEE.call(c, 0x0028_9dd0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool SSXAudioSystem_TakeBootMovieFlag(SSXAudioSystem* self)` (0x289de0):
/// return the boot-movie flag at `self+0x6c88` and clear it.
pub fn ssx_audio_system_take_boot_movie_flag(c: &mut Cpu) {
    c.cycles += 3;
    let flag = crate::abi::arg(c, 0).wrapping_add(0x6c88);
    c.g[V0][0] = Addr(flag).lw(c);
    Addr(flag).sw(c, 0);
    ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    pub(crate) use crate::game::fe::fereal::tests::oc787_install_fakes;
    pub(crate) use crate::game::fe::uitemplatemap::tests::oc777_install_fakes;
    pub(crate) use crate::game::sound::bankmonitor::tests::oc774_install_fakes;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::tests::oc784_install_fakes;

    use crate::difftest::elf_image;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::fill;
    use crate::difftest::put_vec;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::ai::tests::strict;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::SEEDS_2 as SEEDS;
    pub(crate) use crate::game::be::beintplayer::tests::w8;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::base_6 as base_10;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::base_8 as base;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::SCRATCH;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::gp_at;

    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;

    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_9;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_3;

    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;

    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot_9 as slot_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::string_2 as string;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_3 as OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_2 as OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH as SCRATCH_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_2 as WORLD;

    pub(crate) use crate::game::fe::fepopupmisc::tests::wb;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4;

    pub(crate) use crate::game::fe::festatetrophyroom::tests::belib;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::w;
    pub(crate) use crate::game::main::gameload::tests::fb;
    pub(crate) use crate::game::render::graphicsman::tests::same;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::belib_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::heap_setup;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_4;
    pub(crate) use crate::game::sound::asyncsys::tests::audio_world;
    pub(crate) use crate::game::sound::bankmanager::tests::mixed;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::sound::bankmanager::tests::SYS;
    pub(crate) use crate::game::sound::bankmonitor::tests::POOL_BLOCK;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear;

    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS_3;

    use ssx3_ee as ee;

    #[test]
    fn ssx_audio_system_level_to_volume_matches() {
        check(
            ssx3_ee::s07::f_002873d8,
            ssx_audio_system_level_to_volume,
            300,
            &|c, seed| {
                let mut r = Rng(0x1111_0006 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let scale = match seed % 3 {
                    0 => 1.0f32.to_bits(),
                    1 => 2.0f32.to_bits(),
                    _ => r.edge_f32(),
                };
                c.bus.write32(GAME_GP - 0x462c, scale);
                c.g[A1][0] = match seed % 4 {
                    0 => (-5i32) as u32 as u64,
                    1 => 0,
                    2 => 5,
                    _ => r.next_u32() as u64,
                };
            },
        );
    }

    #[test]
    fn ssx_audio_system_get_channel_gain_matches() {
        check(
            ssx3_ee::s07::f_00287930,
            ssx_audio_system_get_channel_gain,
            300,
            &|c, seed| {
                let mut r = Rng(0x1111_0008 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let a1 = (seed % 5) as u64; // 4 exercises the sub-channel path
                let a2 = match seed % 4 {
                    0 => 3,
                    1 => 12,
                    2 => 0xffff_ffff_0000_0007, // negative i64 (< 10), low word stays small
                    _ => (seed % 6) as u64,
                };
                c.g[A1][0] = a1;
                c.g[A2][0] = a2;
                c.bus
                    .write32(DATA + (a1 as u32) * 0x10 + 0x62c8, r.edge_f32());
                c.bus
                    .write32(DATA + ((a2 as u32) << 2) + 0x636c, r.edge_f32());
            },
        );
    }

    #[test]
    fn ssx_audio_system_compute_3d_pan_matches() {
        check(
            ee::s07::f_00285930__generated,
            ssx_audio_system_compute_3d_pan,
            240,
            &|c, seed| {
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
                c.bus.write32(
                    emitter + 0xa0,
                    if seed % 5 == 0 { r.next_u32() | 1 } else { 0 },
                );
                c.bus.write32(emitter + 0xa4, r.next_u32() & 1);
                c.bus.write32(players + 4 * player, listener);
                let identity = seed % 2 == 0;
                for row in 0..4u32 {
                    for col in 0..4u32 {
                        let v = if identity {
                            if row == col {
                                0x3f80_0000
                            } else {
                                0
                            }
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
            },
        );
    }

    #[test]
    fn ssx_audio_system_get_listener_count_matches() {
        check(
            ee::s07::f_00288ce8__generated,
            ssx_audio_system_get_listener_count,
            SEEDS,
            &|c, seed| {
                let mut r = Rng(0x0028_8ce8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let world = audio_world(c);
                if seed % 2 == 0 {
                    // No audio system: the listener count is the literal 1.
                    c.bus.write32(world + 0x84, 0);
                } else {
                    c.bus.write32(world + 0x84, DATA + 0x2000);
                    c.bus.write32(DATA + 0x2000 + 0x84, DATA + 0x3000);
                    c.bus.write32(DATA + 0x3000 + 0x10, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn ssx_audio_system_is_sound_mode2_matches() {
        check(
            ssx3_ee::s07::f_002899e8__generated,
            ssx_audio_system_is_sound_mode2,
            200,
            &|c, seed| {
                let mut r = Rng(0x2899_e801 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                let v = match seed % 3 {
                    0 => 2,
                    1 => r.next_u32(),
                    _ => 3,
                };
                c.bus.write32(DATA + 0x608c, v);
                c.pc = 0x0028_99e8;
            },
        );
    }

    #[test]
    fn ssx_audio_system_set_mode_matches() {
        check(
            ee::s07::f_00285be0__generated,
            ssx_audio_system_set_mode,
            SEEDS_2,
            &|c, seed| {
                let mut r = rng(0x0028_5be0, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                let old = r.next_u32();
                c.bus.write32(DATA + 0x6474, old);
                c.g[A1][0] = match seed % 3 {
                    0 => old as u64,
                    1 => mixed(&mut r, seed, old),
                    _ => r.next_u32() as u64,
                };
            },
        );
    }

    // ---------------------------------------------------------------- audio

    /// A `SSXAudioSystem` at `SCRATCH` with a per-channel volume at
    /// `+0x62bc + channel*16`.
    pub(crate) fn audio_volume_setup(c: &mut Cpu, seed: u32, salt: u64, channel: u32) {
        let mut r = Rng(salt ^ (seed as u64) << 9);
        crate::difftest::scramble(c, &mut r);
        c.g[GP][0] = crate::difftest::GAME_GP as u64;
        c.g[A0][0] = SCRATCH as u64;
        c.bus.write32(
            SCRATCH + 0x62bc + 16 * channel,
            if seed % 3 == 0 {
                r.edge_f32()
            } else {
                r.f32(1.0)
            },
        );
    }

    #[test]
    fn ssx_audio_system_get_music_volume_level_matches() {
        check(
            ee::s07::f_00287670__generated,
            ssx_audio_system_get_music_volume_level,
            120,
            &|c, seed| audio_volume_setup(c, seed, 0x287670, 1),
        );
    }

    #[test]
    fn ssx_audio_system_get_sfx_volume_level_matches() {
        check(
            ee::s07::f_002876a0__generated,
            ssx_audio_system_get_sfx_volume_level,
            120,
            &|c, seed| audio_volume_setup(c, seed, 0x2876a0, 5),
        );
    }

    #[test]
    fn ssx_audio_system_get_speech_volume_level_matches() {
        check(
            ee::s07::f_002876d0__generated,
            ssx_audio_system_get_speech_volume_level,
            120,
            &|c, seed| audio_volume_setup(c, seed, 0x2876d0, 4),
        );
    }

    #[test]
    fn ssx_audio_system_crowd_state_callback_matches() {
        check(
            ee::s07::f_00289650__generated,
            ssx_audio_system_crowd_state_callback,
            100,
            &|c, seed| {
                let _r = machine(c, seed, 0x289650, 1);
                let state = SCRATCH + 0x200;
                c.g[A0][0] = state as u64;
                c.bus
                    .write32(state, if seed % 2 == 0 { 3 } else { (seed % 5) as u32 });
                c.bus
                    .write32(crate::difftest::GAME_GP + 0x410, SCRATCH + 0x300);
                // D_004A3500
            },
        );
    }

    // ------------------------------------------------------------------
    // Audio.

    #[test]
    fn audio_stop_all_sounds_matches() {
        check(
            ssx3_ee::s07::f_00285f48__generated,
            audio_stop_all_sounds,
            SEEDS,
            &|c, seed| {
                let _r = machine(c, seed, 0x285f48, 1);
                let sys = DATA + 0x100;
                // No crowd instances and no sound-system source list.
                c.bus.write32(sys + 0x14, 0);
                c.bus.write32(sys + 0x118, 0);
                c.g[A0][0] = sx(sys);
            },
        );
    }

    #[test]
    fn audio_set_speech_volume_matches() {
        check(
            ssx3_ee::s07::f_00287520__generated,
            audio_set_speech_volume,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x287520, 2);
                let sys = DATA + 0x100;
                fill(c, &mut r, sys, 0x1000);
                c.bus.write32(GAME_GP.wrapping_sub(0x462c), r.edge_f32());
                c.g[A0][0] = sx(sys);
                c.g[A1][0] = (r.next_u32() % 12) as u64;
            },
        );
    }

    pub(crate) const GP_DB0: u32 = 0x49ea_d8;

    // --------------------------------------------------------------- sound

    #[test]
    fn ssx_audio_system_get_preset_gain_matches() {
        check(
            ssx3_ee::s07::f_00288370__generated,
            ssx_audio_system_get_preset_gain,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2883_70, 0);
                belib(c);
                c.g[A0][0] = DATA as u64;
                let index = seed % 4;
                c.g[A1][0] = index as u64;
                w(c, GP_DB0, fb(0.01));
                for i in 0..4u32 {
                    w(c, DATA + 0x6440 + i * 4, r.edge_f32());
                    w(c, DATA + 0x63e4 + i * 4, r.edge_f32());
                }
                match seed % 3 {
                    0 => c.g[A2][0] = 0,
                    1 => c.g[A2][0] = (r.next_u32() % 1000) as u64,
                    _ => {
                        c.g[A2][0] = sx(0xffff_ffff);
                        w(c, DATA + 0x6410, (seed % 2) as u32);
                    }
                }
            },
        );
    }

    // ------------------------------------------------------------------
    // Audio.

    #[test]
    fn ssx_audio_system_destroy_matches() {
        check(
            ee::s07::f_00284c28__generated,
            ssx_audio_system_destroy,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x284c28, 0);
                let sys = DATA + 0x300;
                let inner = DATA + 0x400;
                let obj = DATA + 0x500;
                let vt = vtable(c, &mut r, &[8]);
                c.bus
                    .write32(GAME_GP + 0x410, if seed % 3 != 0 { sys } else { 0 });
                c.bus.write32(sys + 0x118, inner);
                c.bus.write32(inner, obj);
                c.bus.write32(obj + 0x1d4, vt);
            },
        );
    }

    #[test]
    fn ssx_audio_system_pause_game_audio_matches() {
        check(
            ee::s07::f_00289b70__generated,
            ssx_audio_system_pause_game_audio,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28_9b70, 0);
                let obj = DATA + 0x1000;
                c.bus
                    .write32(obj + 0x5fb4, if seed % 2 == 0 { 1 } else { 0 });
                c.bus.write32(obj + 0x118, DATA + 0x2000);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn ssx_audio_system_set_output_mode_matches() {
        check(
            ee::s07::f_002854a8__generated,
            ssx_audio_system_set_output_mode,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x2854);
                let sys = DATA;
                let obj = DATA + 0x1000;
                c.bus.write32(sys + 0x118, obj);
                c.bus.write32(obj, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000 + 0x1d8, DATA + 0x3000);
                fill(c, &mut r, DATA + 0x3000, 0x20);
                c.bus.write32(sys + 0x62b0, 0xdead);
                c.g[A0][0] = sx(sys);
                c.g[A1][0] = sx(r.next_u32() % 8);
            },
        );
    }

    // ------------------------------------------------------------------
    // Audio.

    #[test]
    fn ssx_audio_system_set_gain_target_matches() {
        check(
            ee::s07::f_002887a8__generated,
            ssx_audio_system_set_gain_target,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2887_a8, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let ch = (seed % 5) as u32;
                let target = r.edge_f32();
                c.g[A0][0] = sx(this);
                c.g[A1][0] = ch as u64;
                c.g[A2][0] = (seed % 2) as u64;
                c.f[12] = target;
                c.f[13] = if seed % 7 == 0 { 0 } else { r.edge_f32() };
                c.bus.write32(gp_at(-0x4614), 0x3c88_8893); // 1/60
                let cur = if seed % 3 == 0 { target } else { r.edge_f32() };
                c.bus.write32(this + 0x63e4 + 4 * ch, cur);
            },
        );
    }

    #[test]
    fn ssx_audio_system_set_duck_gain_matches() {
        check(
            ee::s07::f_00289830__generated,
            ssx_audio_system_set_duck_gain,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2898_30, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let v = r.edge_f32();
                c.g[A0][0] = sx(this);
                c.f[12] = v;
                c.bus.write32(gp_at(-0x4620), 0x3c88_8893);
                let cur = if seed % 2 == 0 { v } else { r.edge_f32() };
                c.bus.write32(this + 0x6c5c, cur);
            },
        );
    }

    #[test]
    fn ssx_audio_system_option_to_output_mode_matches() {
        check(
            ee::s07::f_00289d08__generated,
            ssx_audio_system_option_to_output_mode,
            160,
            &|c, seed| {
                let mut r = Rng(0x289d08 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[A1][0] = (seed % 8) as u64;
            },
        );
    }

    #[test]
    fn ssx_audio_system_on_anim_sound_event_matches() {
        check(
            ssx3_ee::s07::f_00289b18__generated,
            ssx_audio_system_on_anim_sound_event,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x289b18, 3);
                let a0 = DATA + 0x400;
                let a1 = DATA + 0x800;
                // A null sfx table at `+0x874` makes every handler return early.
                c.bus.write32(a1 + 0x874, 0);
                c.g[A0][0] = a0 as u64;
                c.g[A1][0] = a1 as u64;
                c.g[A2][0] = [0x50u64, 0x51, 0x52, 0x53, 0][(seed % 5) as usize];
            },
        );
    }

    #[test]
    fn ssx_audio_system_output_mode_to_option_matches() {
        check(
            ssx3_ee::s07::f_00289d60__generated,
            ssx_audio_system_output_mode_to_option,
            80,
            &|c, seed| {
                fakes::machine(c, seed, 0x289d60, 0);
                c.g[A1][0] = match seed % 7 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => 4,
                    5 => 0x1_0000_0001,
                    _ => u64::MAX,
                };
            },
        );
    }

    #[test]
    fn ssx_audio_system_load_memory_bank_matches() {
        check(
            ee::s07::f_00286ca8__generated,
            ssx_audio_system_load_memory_bank,
            6,
            &|c, seed| {
                let _ = base_2(c, seed, 0x0028_6ca8, 2);
                let id = 0x100 + (seed % 4);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = id as u64;
                c.g[A2][0] = if seed % 2 == 0 { 0 } else { 1 };
                // manager -> bank table: keep the bank header's free link at
                // -1 so the unload path does not walk the test heap.
                let manager = DATA + 0x3200;
                w(c, OBJ + 0x118, DATA + 0x3000);
                w(c, DATA + 0x3000, manager);
                w(c, DATA + 0x3ccc, DATA + 0x4000);
                let handle = (id >> 8) + 8;
                w(c, DATA + 0x4000 + 0x60 * handle + 4, 0xffff_ffff);
            },
        );
    }

    #[test]
    fn ssx_audio_system_create_matches() {
        check(
            ee::s07::f_00284bb0__generated,
            ssx_audio_system_create,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x28_4bb0, 3);
                belib_2(c);
                heap_setup(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                // An existing sound system (the singleton create path allocates
                // and constructs the whole object).
                w(c, GAME_GP + 0x410, DATA + 0x3000);
                w(c, DATA + 0x3000 + 0x118, DATA + 0x3200);
                w(c, DATA + 0x3200, DATA + 0x3400);
                c.pc = 0x0028_4bb0;
            },
        );
    }

    #[test]
    fn ssx_audio_system_set_music_volume_matches() {
        check(
            ee::s07::f_00287410__generated,
            ssx_audio_system_set_music_volume,
            SEEDS_4,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28_7410, 4);
                belib_2(c);
                c.g[A0][0] = (DATA + 0x7000) as u64;
                c.g[A1][0] = (seed % 11) as u64;
                c.f[12] = r.edge_f32();
                w(c, DATA + 0x7000 + 0x62b8, (seed % 2) as u32);
                w(c, DATA + 0x7000 + 0x608c, (seed % 4) as u32);
                c.pc = 0x0028_7410;
            },
        );
    }

    #[test]
    fn ssx_audio_system_set_music_channel_enabled_matches() {
        check(
            ee::s07::f_00287558__generated,
            ssx_audio_system_set_music_channel_enabled,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x28_7558, 3);
                belib_2(c);
                c.g[A0][0] = (DATA + 0x7000) as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(c, DATA + 0x7000 + 0x608c, (seed % 4) as u32);
                c.pc = 0x0028_7558;
            },
        );
    }

    #[test]
    fn ini_parser_get_string_matches() {
        check(
            ee::s08::f_002a7ff8__generated,
            ini_parser_get_string,
            SEEDS_5,
            &|c, seed| {
                machine(c, seed, 0x2a7ff8, 4);
                be_library(c);
                world(c);
                heap_init(c);
                let this = DATA;
                let stream = DATA + 0x400;
                let key = DATA + 0x500;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = 0; // compared case-insensitively against `this+0xc`
                c.g[A2][0] = (DATA + 0x800) as u64;
                // `this+0xc` is the key the match compares against address 0 (so
                // it must be the empty string to pass); `this+8` is the character
                // stream. `ReadQuotedString`'s copy loop has no NUL check, so the
                // stream must always contain a closing quote.
                string(c, key, "");
                w(c, this + 0xc, key);
                w(c, this + 8, stream);
                match seed % 4 {
                    0 => string(c, stream, "\""),
                    1 => string(c, stream, "v\""),
                    2 => string(c, stream, "value\""),
                    _ => string(c, stream, "\"\""),
                }
            },
        );
    }

    #[test]
    fn ssx_audio_system_enter_game_init_matches() {
        check(
            ee::s07::f_00286e20__generated,
            ssx_audio_system_enter_game_init,
            60,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x286e, 3);
                w(
                    c,
                    GAME_GP.wrapping_add(0x21e4),
                    if seed % 3 == 0 { 0 } else { DATA + 0x400 },
                );
                wb(
                    c,
                    0x0053_5c11,
                    match seed % 4 {
                        0 => 1,
                        1 => 2,
                        _ => 0,
                    },
                );
                fill(c, &mut r, DATA, 0x40);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn ini_parser_ctor_matches() {
        check(
            ee::s08::f_002a77f8__generated,
            ini_parser_ctor,
            40,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x2a77f8, 3);
                let obj = DATA + 0x1000;
                let text = DATA + 0x6000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = text as u64;
                c.g[A2][0] = r.next_u32() as u64;
                let n = if seed % 3 == 0 { 0 } else { 1 + seed % 40 } as u32;
                for i in 0..n {
                    w8(c, text + i, b'a' + (seed as u8 % 26));
                }
                w8(c, text + n, 0);
            },
        );
    }

    #[test]
    fn ssx_audio_system_set_sfx_volume_matches() {
        check(
            ee::s07::f_00287488__generated,
            ssx_audio_system_set_sfx_volume,
            SEEDS,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                w(c, this + 0x62b4, (seed & 1) as u32);
                c.g[A0][0] = (seed % 100) as u64;
                let _ = this;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 101) as u64;
            }),
        );
    }

    // ------------------------------------------------------------------ audio

    #[test]
    fn ssx_audio_system_set_sfx_sub_channels_enabled_matches() {
        check(
            ssx3_ee::s07::f_002875d0__generated,
            ssx_audio_system_set_sfx_sub_channels_enabled,
            12,
            &|c, seed| {
                let mut r = base_6(c, seed, 0x0028_75d0);
                let audio = DATA + 0x100;
                clear(c, audio, 0x6400);
                for k in 0..11 {
                    w(c, audio + 0x62bc + 0x10 * k, r.edge_f32());
                }
                c.g[A0][0] = sx(audio);
                c.g[A1][0] = match seed % 3 {
                    0 => 0,
                    1 => 1,
                    _ => r.next_u32() as u64,
                };
            },
        );
    }

    #[test]
    fn snd_load_queue_dtor_matches() {
        check(
            ssx3_ee::s07::f_0028a148__generated,
            snd_load_queue_dtor,
            30,
            &|c, seed| {
                let _r = base_7(c, seed, 0x28a148, 3);
                let this = DATA + 0x1000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed & 1) as u64;
                for (i, off) in [0x1c0u32, 0x1b0, 0x1a0, 0x190].into_iter().enumerate() {
                    let set = (seed >> i) & 1 != 0;
                    w_2(c, this + off, if set { POOL_BLOCK } else { 0 });
                }
                w_2(c, GAME_GP.wrapping_add(0x520), 0x55);
            },
        );
    }

    pub(crate) const BANKMAN: u32 = DATA + 0x15000;

    // ---------------------------------------------------------------- scripts

    #[test]
    fn ssx_audio_system_unload_world_banks_matches() {
        check(
            ee::s07::f_00286c00__generated,
            ssx_audio_system_unload_world_banks,
            4,
            &|c, seed| {
                let _ = base_8(c, seed, 0x286c00, 4);
                c.g[A0][0] = SYS as u64;
                w(c, SYS + 0x400, BANKMAN);
                w(c, SYS + 0x118, SYS + 0x400);
            },
        );
    }

    #[test]
    fn ssx_audio_system_on_reset_race_matches() {
        check(
            ee::s07::f_00287108__generated,
            ssx_audio_system_on_reset_race,
            8,
            &|c, seed| {
                let _ = base_8(c, seed, 0x287108, 4);
                c.g[A0][0] = SYS as u64;
                w(c, SYS + 0x5824, (seed * 3) as u32);
                w(c, WORLD + 0xc0, OBJ2);
                w(c, OBJ2 + 0x70, (seed % 4) as u32);
            },
        );
    }

    // ---------------------------------------------------------------- audio

    #[test]
    fn ssx_audio_system_unload_front_end_audio_matches() {
        check(
            ee::s07::f_00286200__generated,
            ssx_audio_system_unload_front_end_audio,
            SEEDS_6,
            &|c, seed| {
                let _ = base_9(c, seed, 0x2862_00, 8);
                let obj = SCRATCH_2;
                c.g[A0][0] = obj as u64;
                w(
                    c,
                    obj + 0x608c,
                    match seed % 3 {
                        0 => 2,
                        1 => 1,
                        _ => 0,
                    },
                );
                w(c, obj + 0x6090, (seed % 2) as u32);
                let player = obj + 0x2000;
                w(c, obj + 0x118, player);
                w(c, player, obj + 0x2100);
                w(c, obj + 0x2100 + 0xab0, obj + 0x2200);
                slot_4(c, obj + 0x2200, 0x10, 0);
                slot_4(c, obj + 0x2200, 0x14, 1);
            },
        );
    }

    #[test]
    fn sndloadqueue_ctor_matches() {
        strict(
            ee::s07::f_0028a058__generated,
            sndloadqueue_ctor,
            SEEDS,
            &|c, seed| {
                let _r = base_10(c, seed, 0xa058, 0);
                c.g[A0][0] = sx(DATA + 0x1000);
            },
        );
    }

    #[test]
    fn ssx_audio_system_set_fade_gain_matches() {
        same(
            ssx3_ee::s07::f_00287a10__generated,
            ssx_audio_system_set_fade_gain,
            200,
            &|c, seed| {
                let mut r = Rng(0x287a10 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let s = DATA + 0x100;
                c.g[A0][0] = s as u64;
                c.g[A1][0] = (seed % 0x400) as u64;
                c.f[12] = if seed % 3 == 0 { 0 } else { r.f32(2.0) };
                c.f[13] = r.f32(1.0);
                for off in [0x63c4u32, 0x63c8, 0x63cc, 0x63d0] {
                    c.bus.write32(s + off, r.edge_f32());
                }
                for off in [
                    -0x4620i32, 0x43c, 0x440, 0x444, 0x448, 0x44c, 0x450, 0x454, 0x458, 0x45c,
                    0x460,
                ] {
                    c.bus
                        .write32(GAME_GP.wrapping_add(off as u32), r.edge_f32());
                }
            },
        );
    }

    /// SSXAudioSystem_SetMixPreset: same preset (no-op), negative id,
    /// immediate apply (fade 0) and fading, gains of -1 (keep current).
    #[test]
    fn ssx_audio_system_set_mix_preset_matches() {
        check_lohi(
            ee::s07::f_002883b0__generated,
            ssx_audio_system_set_mix_preset,
            240,
            &|c, seed| {
                let mut r = Rng(0x2883_b000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let this = DATA;
                c.g[A0][0] = sx(this);
                for k in 0..0x50 {
                    c.bus.write32(this + 0x62c0 + 4 * k, r.f32(4.0));
                }
                for k in 0..0x30 {
                    c.bus.write32(this + 0x6440 + 4 * k, r.f32(4.0));
                }
                for k in 0..10 {
                    c.bus.write32(GAME_GP + GP_MIX_WEIGHTS + 4 * k, r.f32(2.0));
                }
                c.bus.write32(GAME_GP.wrapping_sub(0x4614), r.f32(2.0));
                c.bus.write32(this + AUD_FADING, r.next_u32() % 2);
                let id = (r.next_u32() % 4) as i32 - if seed % 6 == 0 { 3 } else { 0 };
                c.g[A1][0] = sx(id as u32);
                let cur = if seed % 8 == 0 {
                    id.max(0) as u32
                } else {
                    r.next_u32() % 5
                };
                c.bus.write32(this + AUD_PRESET, cur);
                for p in 0..4u32 {
                    let e = this + AUD_PRESETS + p * PRESET_SIZE as u32;
                    for k in 0..9 {
                        let g = if r.next_u32() % 3 == 0 {
                            u32::MAX
                        } else {
                            r.f32(2.0)
                        };
                        c.bus.write32(e + 4 * k, g);
                    }
                    let fade = if seed % 2 == 0 {
                        0
                    } else {
                        r.f32(3.0) & 0x7fff_ffff
                    };
                    c.bus.write32(e + 0x24, fade);
                }
            },
        );
    }

    use ssx3_ee_rt::Cpu;

    /// SSXAudioSystem_UpdateFade: no fade; rising and falling fades that
    /// do or do not reach the target; random channel masks (all bits).
    #[test]
    fn ssx_audio_system_update_fade_matches() {
        use super::audio_fade::*;
        use crate::difftest::scramble;
        check(
            ee::s07::f_00287c48__generated,
            ssx_audio_system_update_fade_2,
            300,
            &|c, seed| {
                let mut r = Rng(0x287c_48 ^ (seed as u64) << 12);
                scramble(c, &mut r);
                c.pc = 0x0028_7c48;
                c.g[GP][0] = GAME_GP as u64;
                for i in 0..10 {
                    c.bus.write32(GAME_GP + GP_MIX_LEVELS + 4 * i, r.f32(2.0));
                }
                let s = DATA;
                c.g[A0][0] = s as u64;
                for off in (0x62d0..0x6410).step_by(4) {
                    c.bus.write32(s + off, r.f32(2.0));
                }
                c.bus
                    .write32(s + FADING, if seed % 6 == 0 { 0 } else { 1 + seed % 3 });
                c.bus.write32(
                    s + MASK,
                    if seed % 5 == 0 {
                        0x3ff
                    } else {
                        r.next_u32() & 0x3ff
                    },
                );
                c.bus.write32(s + RATE, r.f32(0.5));
                c.bus.write32(s + TARGET, r.f32(1.0));
                c.bus.write32(s + CUR, r.f32(1.0));
            },
        );
    }

    // ---------------------------------------- SSXAudioSystem_UpdateSpeechDucking

    /// `SSXAudioSystem_UpdateSpeechDucking`: playing or not, even and odd
    /// ticks, envelope indices in and past the range, levels <= 0 and across
    /// the thresholds of speech modes 2, 3 and others, ramp states -1/0/1;
    /// the real speech-mode getter, `SetDuckGain` and `SetFadeGain` run.
    #[test]
    fn ssx_audio_system_update_speech_ducking_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA, GAME_GP};
        check(
            ee::s07::f_00289688__generated,
            ssx_audio_system_update_speech_ducking,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0028_9688, 0);
                c.pc = 0x0028_9688;
                let (this, stream) = (DATA + 0x100, DATA + 0x9000);
                let f = |x: f32| x.to_bits();
                c.bus.write32(GAME_GP.wrapping_sub(0x4620), 0x3c88_8893);
                c.bus.write32(GAME_GP + 0x2a74, stream);
                c.bus.write32(
                    stream + 0x2c,
                    if seed % 9 == 0 {
                        r.edge_f32()
                    } else {
                        f(1.0 + (r.next_u32() % 4) as f32)
                    },
                );
                c.bus.write32(this + 0x6c58, (seed % 4 != 0) as u32);
                c.bus.write32(this + 0x6864, r.next_u32() % 20);
                c.bus.write32(this + 0x6c54, 1 + r.next_u32() % 4);
                c.bus.write32(this + 0x6c50, r.next_u32() % 100);
                for i in 0..0x60u32 {
                    let v = (r.next_u32() % 90) as i32 - 10;
                    c.bus.write8(this + 0x6868 + i, v as u8);
                }
                c.bus
                    .write32(this + 0x5738, [2u32, 3, 7][(seed % 3) as usize]);
                c.bus.write32(
                    this + 0x6c60,
                    [0xffff_ffffu32, 0, 1, 0xffff_fffe][((seed / 3) % 4) as usize],
                );
                for off in [0x6c64u32, 0x6c68, 0x6c6c] {
                    c.bus.write32(this + off, r.f32(1.0));
                }
                let cur = if seed % 2 == 0 {
                    c.bus.read32(this + 0x6c6c)
                } else {
                    r.f32(1.0)
                };
                c.bus.write32(this + 0x6c5c, cur);
                for off in [0x63c4u32, 0x63c8, 0x63cc, 0x63d0] {
                    c.bus.write32(this + off, r.f32(1.0));
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    // ------------------------------------------- SSXAudioSystem_UpdateMixRamp

    /// `SSXAudioSystem_UpdateMixRamp`: idle or ramping; per channel an idle
    /// step, a rising or falling step that arrives (clamped) or not, with
    /// edge-case floats mixed in.
    #[test]
    fn ssx_audio_system_update_mix_ramp_matches() {
        use super::mix_ramp::*;
        use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
        let f = |x: f32| x.to_bits();
        check(
            ee::s07::f_002887f8__generated,
            ssx_audio_system_update_mix_ramp,
            300,
            &|c, seed| {
                let mut r = Rng(0x2887_f800 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0028_87f8;
                let this = DATA;
                c.bus.write32(
                    this + DIRTY,
                    if seed % 9 == 0 { 0 } else { r.next_u32() | 1 },
                );
                for i in 0..11u32 {
                    let cur = (r.next_u32() % 200) as f32 * 0.01;
                    let target = (r.next_u32() % 200) as f32 * 0.01;
                    let delta = match r.next_u32() % 6 {
                        0 => 0.0,
                        1 => 0.05,
                        2 => -0.05,
                        3 => 1.0,
                        4 => -1.0,
                        _ => f32::from_bits(r.edge_f32()),
                    };
                    c.bus.write32(this + CUR + 4 * i, f(cur));
                    c.bus
                        .write32(this + CUR + TARGET_FROM_CUR + 4 * i, f(target));
                    c.bus.write32(this + DELTA + 4 * i, f(delta));
                    c.bus.write32(this + CHAN_SCALED - 4 + 0x10 * i, r.f32(2.0));
                    c.bus.write32(this + CHAN_SCALED + 0x10 * i, r.next_u32());
                }
                c.bus.write32(this + MASTER, r.f32(1.0));
                for k in 0..10u32 {
                    c.bus.write32(GAME_GP + 0x43c + 4 * k, r.f32(1.0));
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `SSXAudioSystem_GetRiderRatingTier`: the BE library dummy at gp-0x1f04
    /// (so `BELibrary_GetInstance` returns it), interface table 0x5308a8
    /// indexed by gp-0x1efc whose object's `GetInterface` (+0x8) and the
    /// rider controller's slot 0x38 are fakes (the latter forced to return a
    /// rider 1..7); the real `BECharInt_GetOverallRating` on the rider
    /// tables of its own test (ratings below 4, 4..7 and 8 up).
    #[test]
    fn ssx_audio_system_get_rider_rating_tier_matches() {
        use crate::difftest::fakes::LOG_RESULTS;
        use crate::difftest::{fill, GAME_GP};
        const BELIB: u32 = DATA + 0x1000;
        const IFACE_OBJ: u32 = DATA + 0x1100;
        const IFACE_VT: u32 = DATA + 0x1200;
        const RIDER: u32 = DATA + 0x2000;
        const CTRL_VT: u32 = DATA + 0x3000;
        covered(
            ssx3_ee::s07::f_00289c18__generated,
            ssx_audio_system_get_rider_rating_tier,
            300,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x289c18, 2);
                c.pc = 0x0028_9c18;
                c.bus.write32(GAME_GP - 0x1f04, BELIB);
                let idx = r.next_u32() % 3;
                c.bus.write32(GAME_GP - 0x1efc, idx);
                c.bus.write32(0x0053_08a8 + 4 * idx, IFACE_OBJ);
                c.bus.write32(IFACE_OBJ + 4, IFACE_VT);
                c.bus.write16(IFACE_VT + 0x8, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(IFACE_VT + 0xc, FAKE);
                c.g[A0][0] = sx(DATA + 0x5000);
                c.g[A1][0] = sx(RIDER);
                c.bus.write32(RIDER + 0x6c0, CTRL_VT);
                c.bus.write16(CTRL_VT + 0x38, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(CTRL_VT + 0x3c, FAKE + 0x10);
                c.bus.write32(LOG_RESULTS + 4, 1 + r.next_u32() % 7);
                // BECharInt_GetOverallRating's tables.
                for i in 0..8u32 {
                    c.bus.write32(0x0053_05b0 + 4 * i, r.next_u32() % 8);
                }
                for i in 0..8u32 {
                    let p = 0x0053_5b20 + 0x1c * i;
                    fill(c, &mut r, p, 7);
                    if r.next_u32() % 3 == 0 {
                        c.bus.write32(p + 0xc, 0xffff_ffff);
                    }
                    c.bus.write8(p + 0x11, (r.next_u32() % 10) as u8);
                }
                let scale = [3u8, 7, 15, 31][(seed % 4) as usize];
                for i in 0..0x100u32 {
                    let v = r.next_u32() as u8 % (scale + 1);
                    c.bus.write8(0x0053_5538 + i, v);
                }
            },
        );
    }

    /// The audio world (gp-0x848) with an app whose view list holds 0..3
    /// views (or no list: one listener, view 0 read from low RAM) and a
    /// listener position near or far from them.
    #[test]
    fn ssx_audio_find_view_in_range_matches() {
        use crate::difftest::GAME_GP;
        check(
            ee::s07::f_00288b40__generated,
            ssx_audio_find_view_in_range,
            300,
            &|c, seed| {
                let mut r = Rng(0x288b_40c3 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0028_8b40;
                c.g[GP][0] = sx(GAME_GP);
                let world = DATA;
                let app = DATA + 0x100;
                let list = DATA + 0x200;
                let pos = DATA + 0x400;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, app);
                let has_list = seed % 7 != 0;
                c.bus.write32(app + 0x84, if has_list { list } else { 0 });
                let n = seed % 4;
                c.bus.write32(list + 0x10, n);
                let spread = if seed % 3 == 0 { 1.0 } else { 20.0 };
                for i in 0..3 {
                    let view = DATA + 0x1000 + i * 0x40;
                    c.bus.write32(list + 4 + i * 4, view);
                    put_vec(
                        c,
                        view + 0x20,
                        [r.f32(spread), r.f32(spread), r.f32(spread), r.f32(1.0)],
                    );
                }
                put_vec(
                    c,
                    pos,
                    [r.f32(spread), r.f32(spread), r.f32(spread), r.f32(1.0)],
                );
                c.g[A0][0] = sx(DATA + 0x800);
                c.g[A1][0] = sx(pos);
                c.f[12] = if seed % 11 == 0 {
                    r.edge_f32()
                } else {
                    r.f32(0.2) & 0x7fff_ffff
                };
            },
        );
    }

    /// SSXAudioSystem_GetRiderListenerIndex: outside / inside the game, the
    /// rider's fixed index on or off (flag, on word), one or two listeners;
    /// with two the rider's position getter is a fake (may unwind) and the
    /// real IsCloserToSecondView compares against two random views.
    #[test]
    fn ssx_audio_system_get_rider_listener_index_matches() {
        use crate::difftest::fakes::{machine, vtable};
        use crate::difftest::{check, fill, DATA, GAME_GP};
        const THIS: u32 = DATA + 0x1_0000;
        const RIDER: u32 = DATA + 0x2_0000;
        const WORLD: u32 = DATA + 0x3_0000;
        const APP: u32 = DATA + 0x3_1000;
        const VIEWS: u32 = DATA + 0x3_2000;
        const VIEW: u32 = DATA + 0x3_3000;
        let f = |x: f32| x.to_bits();
        check(
            ee::s07::f_00288a20__generated,
            ssx_audio_system_get_rider_listener_index,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0028_8a20, 1);
                c.pc = 0x0028_8a20;
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = sx(RIDER);
                // In the game (the world's app word) or not.
                c.bus.write32(GAME_GP - 0x848, WORLD);
                c.bus
                    .write32(WORLD + 0x84, if seed % 7 == 0 { 0 } else { APP });
                c.bus.write32(APP + 0x84, VIEWS);
                c.bus.write32(VIEWS + 0x10, 1 + (seed / 2) % 2);
                for i in 0..2u32 {
                    let v = VIEW + 0x100 * i;
                    c.bus.write32(VIEWS + 4 + 4 * i, v);
                    for k in 0..0x40u32 {
                        c.bus
                            .write32(v + 4 * k, f(r.next_u32() as f32 / 1e7 - 200.0));
                    }
                }
                // The fixed listener index.
                let flag = [0u32, 1, 0x8000_0000][(seed % 3) as usize];
                let on = [0u32, 2, 0xffff_ffff][((seed / 3) % 3) as usize];
                c.bus.write32(RIDER + RLI_FIXED_FLAG, flag);
                c.bus.write32(RIDER + RLI_FIXED_ON, on);
                c.bus.write32(RIDER + RLI_FIXED_INDEX, r.next_u32() % 2);
                let vt = vtable(c, &mut r, &[0x28]);
                c.bus.write32(RIDER + RLI_POS_OBJECT, vt);
                fill(c, &mut r, DATA, 0x10);
            },
        );
    }

    /// The audio world for `SSXAudioSystem_IsAnyViewInRange`: the world at
    /// `gp-0x848` (`+0x84` the game module, or none), the module's view list
    /// (`+0x84`: count at `+0x10`, views from `+0x4`), each view's position
    /// at `+0x20` near `pos` or far, in front of it or behind; a radius
    /// argument in f12.
    pub(crate) fn oc305_view_in_range_fixture(c: &mut Cpu, seed: u32, min_views: u32) {
        use crate::difftest::{fill, scramble, Rng, DATA, GAME_GP};
        let mut r = Rng(0x288c_0800 ^ seed as u64);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        let world = DATA;
        let module = DATA + 0x100;
        let views = DATA + 0x200;
        c.bus.write32(GAME_GP - 0x848, world);
        c.bus
            .write32(world + 0x84, if seed % 9 == 0 { 0 } else { module });
        c.bus.write32(module + 0x84, views);
        // At most 3: the view pointers sit at +0x4..+0xc, right below the
        // count at +0x10 (a fourth pointer would overwrite the count).
        let count = min_views + seed / 2 % (4 - min_views);
        c.bus.write32(views + 0x10, count);
        let pos = DATA + 0x1000;
        let mut coord = || f32::from_bits(r.f32(5000.0));
        let base = [coord(), coord(), coord()];
        for (k, v) in base.iter().enumerate() {
            c.bus.write32(pos + 4 * k as u32, v.to_bits());
        }
        c.bus.write32(pos + 12, r.next_u32());
        let radius = 0.5 + f32::from_bits(r.f32(10.0)).abs();
        c.f[12] = radius.to_bits();
        for i in 0..count {
            let view = DATA + 0x2000 + 0x100 * i;
            fill(c, &mut r, view, 0x40);
            c.bus.write32(views + 4 + 4 * i, view);
            let near = r.next_u32() % 2 == 0;
            let reach = radius * 100.0 * if near { 0.5 } else { 1.5 };
            let dz = if r.next_u32() % 3 == 0 {
                -reach * 0.5
            } else {
                reach * 0.5
            };
            let d = [reach * 0.5, -reach * 0.5, dz];
            for k in 0..3 {
                c.bus
                    .write32(view + 0x20 + 4 * k as u32, (base[k] + d[k]).to_bits());
            }
        }
        c.g[A0][0] = sx(DATA + 0x3000);
        c.g[A1][0] = sx(pos);
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = OC305_VIEW_IN_RANGE_ENTRY;
    }

    // `SSXAudioSystem_IsAnyViewInRange`: 0..3 views, near or far, in
    // front or behind, no game module (one view); then events off,
    // delivered, or unwinding at the loop's back edge and re-entering at
    // the head.
    crate::difftest::events::driven!(oc305_gen_view_in_range, ee::s07::f_00288c08__generated);
    crate::difftest::events::driven!(
        oc305_hand_view_in_range,
        ssx_audio_system_is_any_view_in_range
    );

    #[test]
    fn ssx_audio_system_is_any_view_in_range_matches() {
        use crate::difftest::events::{check_events, schedule, Events};
        crate::difftest::check(
            ee::s07::f_00288c08__generated,
            ssx_audio_system_is_any_view_in_range,
            240,
            &|c, seed| oc305_view_in_range_fixture(c, seed, 0),
        );
        check_events(
            oc305_gen_view_in_range,
            oc305_hand_view_in_range,
            150,
            &|c, seed| {
                // At least two views so the back edge is taken.
                oc305_view_in_range_fixture(c, seed, 2);
                let mut r = crate::difftest::Rng(0x288c_0801 ^ seed as u64);
                let mode = schedule(c, &mut r, seed, OC305_VIEW_IN_RANGE_ENTRY);
                if mode != Events::Off {
                    c.next_event = c.cycles + (r.next_u32() % 40) as u64;
                }
                mode
            },
        );
    }

    /// The audio app's listener list with three views; index 0..2.
    #[test]
    fn ssx_audio_get_view_pos_matches() {
        use crate::difftest::GAME_GP;
        check(
            ee::s07::f_00288ac0__generated,
            ssx_audio_get_view_pos,
            60,
            &|c, seed| {
                let mut r = Rng(0x288a_c0d1 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0028_8ac0;
                c.g[GP][0] = sx(GAME_GP);
                let world = DATA;
                let app = DATA + 0x100;
                let list = DATA + 0x200;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, app);
                c.bus.write32(app + 0x84, list);
                c.bus.write32(list + 0x10, 3);
                for i in 0..3 {
                    c.bus
                        .write32(list + 4 + i * 4, DATA + 0x1000 + (r.next_u32() & 0xff0));
                }
                c.g[A0][0] = sx(DATA + 0x800);
                c.g[A1][0] = sx(seed % 3);
            },
        );
    }

    use crate::difftest::fakes::fake;
    use crate::difftest::fakes::fall_back;
    use crate::difftest::fakes::hook;

    pub(crate) use crate::game::object::movenode::tests::oc338_fake;
    pub(crate) use crate::game::object::movenode::tests::OC338_CTRL;
    pub(crate) use crate::game::object::movenode::tests::OC338_MAGIC;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    oc338_fake!(
        oc338_fake_bank_monitor_stop,
        OC338_PREV_BANK_MONITOR_STOP,
        0x002a_d410,
        ee::s08::f_002ad410__generated
    );

    /// `SSXAudioSystem_StopByCallback`: the bank monitor at
    /// `**(s+0x118) + 0x1d8` gets the 64-bit pair (a logging fake).
    #[test]
    fn ssx_audio_system_stop_by_callback_matches() {
        hook(
            addr_of_mut!(ee::s08::OVERRIDE_002AD410),
            oc338_fake_bank_monitor_stop,
            &OC338_PREV_BANK_MONITOR_STOP,
        );
        covered(
            ee::s07::f_00285f80__generated,
            ssx_audio_system_stop_by_callback,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0028_5f80, 1);
                c.pc = 0x0028_5f80;
                c.bus.write32(OC338_CTRL, OC338_MAGIC);
                let this = DATA + 0x5_0000;
                let holder = DATA + 0x5_1000;
                c.bus
                    .write32(this + SSX_AUDIO_STOP_BY_CALLBACK_HOLDER, holder);
                c.bus
                    .write32(holder, DATA + 0x5_2000 + (r.next_u32() & 0xff0));
                c.g[A0][0] = sx(this);
            },
        );
    }

    pub(crate) use crate::game::lib::snd::tests::oc375_machine;
    pub(crate) use crate::game::lib::snd::tests::oc375_script;

    /// SSXAudioSystem_PlayStream: the queue fails or succeeds; radius
    /// positive, zero/negative with and without the engine flag; with or
    /// without a target; the two-player mirror check through each exit; the
    /// bank monitor returning an instance or none. All callees but memset
    /// are fakes.
    #[test]
    fn ssx_audio_system_play_stream_matches() {
        covered(
            ee::s07::f_002854f8__generated,
            ssx_audio_system_play_stream,
            300,
            &|c, seed| {
                let mut r = oc375_machine(c, seed, 0x0028_54f8, 8);
                c.pc = 0x0028_54f8;
                let sys = DATA + 0x100;
                let pp = DATA + 0x200;
                let arr = DATA + 0x1000;
                let params = arr + PLAY_STREAM_PARAMS;
                c.bus.write32(sys + PLAY_STREAM_ARRAY, pp);
                c.bus.write32(pp, arr);
                fill(c, &mut r, arr + 0x1d8, 0x30);
                c.bus.write32(arr + PLAY_STREAM_TOP, r.next_u32() % 3);
                // Per-level arrays.
                for (i, off) in [
                    0x34u32, 0x38, 0x3c, 0x60, 0x64, 0x68, 0x6c, 0x70, 0x74, 0x78, 0x7c, 0x80, 0x84,
                ]
                .into_iter()
                .enumerate()
                {
                    c.bus
                        .write32(params + off, DATA + 0x2000 + 0x100 * i as u32);
                }
                c.bus.write32(params + 0x20, DATA + 0x3000);
                fill(c, &mut r, DATA + 0x3000, 0x40);
                c.g[A0][0] = sx(sys);
                c.g[T0][0] = if seed % 3 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.g[T3][0] = if seed % 2 == 0 { 0 } else { 1 };
                c.f[12] = match seed % 4 {
                    0 => 0,
                    1 => 0xbf80_0000,
                    _ => r.f32(500.0) & 0x7fff_ffff,
                };
                c.bus.write32(GAME_GP - 0x465c, 0x41ef_ff88);
                c.bus.write32(crate::difftest::SP, r.next_u32());
                c.bus.write8(0x0053_5c11, if seed % 5 == 0 { 1 } else { 2 });
                oc375_script(
                    c,
                    0x002a_ad78,
                    if seed % 7 == 0 {
                        0xffff_ffff
                    } else {
                        r.next_u32() % 50
                    },
                );
                oc375_script(c, 0x0014_dd58, (seed / 2 % 4 != 0) as u32);
                oc375_script(c, 0x0028_8ae0, (seed / 3 % 3 == 0) as u32);
                oc375_script(
                    c,
                    0x002a_cfe0,
                    if seed / 5 % 3 == 0 { 0 } else { DATA + 0x5000 },
                );
            },
        );
    }

    pub(crate) use crate::game::fe::fereal::tests::oc787_mail;
    pub(crate) use crate::game::fe::feridermanager::tests::oc777_pat;
    pub(crate) use crate::game::fe::festatecharsetup::tests::oc774_arm;
    pub(crate) use crate::game::fe::festatecharsetup::tests::oc774_force;
    pub(crate) use crate::game::fe::festatecharsetup::tests::oc787_arm;
    pub(crate) use crate::game::fe::festatecharsetup::tests::OC774_AUDIO;
    pub(crate) use crate::game::fe::festatecharsetup::tests::OC774_MOD;
    pub(crate) use crate::game::fe::festatecharsetup::tests::OC774_UI;
    pub(crate) use crate::game::fe::festatecharsetup::tests::OC774_WORLD;
    pub(crate) use crate::game::fe::festatecharsetup::tests::OC787_AUX;

    pub(crate) use crate::game::fe::festatecharsetup::tests::OC787_OBJ;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::oc784_arm;

    pub(crate) use crate::game::fe::festatetrophyroom::tests::OC784_OBJ;
    pub(crate) use crate::game::fe::ovtemplatedialog::tests::oc784_force;
    pub(crate) use crate::game::fe::ovtemplatedialog::tests::oc784_force_zero;

    pub(crate) use crate::game::fe::ovtemplatedialog::tests::OC774_STATE;
    pub(crate) use crate::game::fe::ovtemplatedialog::tests::OC784_FILE;
    pub(crate) use crate::game::fe::uitemplatemap::tests::oc777_gate;

    pub(crate) use crate::game::fe::uitemplatemap::tests::OC777_OBJ;

    pub(crate) use crate::game::sound::bankmonitor::tests::oc768_machine;

    pub(crate) use crate::game::sound::icepick::worldtriggermanager::tests::oc784_count;
    pub(crate) use crate::game::sound::soundsys::tests::oc777_tables;

    // -- SSXAudioSystem_SetupChannelGains --------------------------------

    /// Modes 0, 1 and others (one seed in three each), with the game image in
    /// RAM for the `.lit4` scaling constants and `SetChannelVolume`'s jump
    /// table; the real Get/SetChannelVolume run, the music player update and
    /// the master volume call they make are gated fakes (which unwind for
    /// some seeds).
    #[test]
    fn ssx_audio_system_setup_channel_gains_matches() {
        covered(
            ee::s07::f_00288d18__generated,
            ssx_audio_system_setup_channel_gains,
            45,
            &|c, seed| {
                let mut r = oc768_machine(c, seed, 0x0028_8d18, 2);
                elf_image(c);
                let this = DATA + 0x100;
                for off in (0..0x6400u32).step_by(4) {
                    c.bus.write32(this + off, 0);
                }
                for k in 0..11u32 {
                    let v = r.edge_f32();
                    c.bus.write32(this + 0x62bc + 0x10 * k, v);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = match seed % 3 {
                    0 => 0,
                    1 => 1,
                    _ => 2 + (r.next_u32() % 9) as u64,
                };
            },
        );
    }

    /// `SSXAudioSystem_EnterGame`: a song always playing with id 401 (see the
    /// fixture), the audio state byte (0x535c11) zero or not (the 23 first-time
    /// flags are filled), and the unwinds of its calls. The music functions
    /// (0x28bf78, 0x28c2d0, 0x28d488, 0x28cf98, 0x28df08, 0x2a4a78) are the
    /// real, unported ones, running on a zeroed audio system.
    #[test]
    fn ssx_audio_system_enter_game_matches() {
        oc774_install_fakes();
        fakes::covered(
            ee::s07::f_002867e8__generated,
            ssx_audio_system_enter_game,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x774f, 60);
                oc774_arm(c);
                c.pc = 0x0028_67e8;
                c.g[A0][0] = sx(OC774_AUDIO);
                // A song is always playing (id 401): the song-selection chain
                // (0x28d488 -> MusicPlayer_SelectNextSong 0x2b42b8, unported)
                // loops forever on a zeroed audio system, so the fixture never
                // reaches it. The two "not playing" branches stay untested.
                c.bus.write32(OC774_AUDIO + 0x530, 1);
                c.bus.write32(GAME_GP - 0x848, OC774_WORLD);
                c.bus.write32(OC774_WORLD + 0x84, OC774_MOD);
                c.bus.write32(OC774_MOD + 0x10, OC774_UI);
                c.bus.write32(OC774_UI + 0x8, OC774_STATE);
                c.bus.write8(0x0053_5c11, ((seed / 4) % 2) as u8);
                let _ = r.next_u32();
                oc774_force(c, 0x002b_49e0, 0x191);
            },
        );
    }

    /// Shared fixture of the front-end audio load: the audio system object
    /// (`this`), its music system pointer (+0x118) and stream-system chain,
    /// the object at +0x5558 whose vtable entry 2 is a fake, and the game
    /// tables the channel-gain setup dispatches on.
    pub(crate) fn oc777_audio_fixture(c: &mut Cpu, r: &mut Rng) {
        oc777_tables(c);
        let this = OC777_OBJ;
        let music = OC777_OBJ + 0x1000;
        let bank_man = OC777_OBJ + 0x1100;
        let gain_obj = OC777_OBJ + 0x1200;
        c.bus.write32(this + 0x118, music);
        c.bus.write32(music, bank_man);
        c.bus.write32(bank_man + 0xacc, r.next_u32());
        c.bus.write32(this + 0x5558, gain_obj);
        c.bus.write16(gain_obj + 0x10, (r.next_u32() & 0x7c) as u16);
        c.bus.write32(gain_obj + 0x14, fakes::FAKE + 0x20);
        c.bus.write32(this + 0x608c, r.next_u32());
        c.bus.write32(GAME_GP + 0x464, r.next_u32());
        c.g[A0][0] = sx(this);
    }

    /// `SSXAudioSystem_LoadFrontEndAudio`: the ini parser calls are gated
    /// fakes with scripted results (section found or not, end-of-file after
    /// 0..4 keys, `strcasecmp` equal for some keys); the channel-gain setup,
    /// `SetChannelVolume`, `GetName` and `SndBankMan_LoadBank` run for real
    /// (the parser object is zeroed stack, so `GetName` ends at once).
    #[test]
    fn ssx_audio_system_load_front_end_audio_matches() {
        oc777_install_fakes();
        check(
            ee::s07::f_00285fb0__generated,
            ssx_audio_system_load_front_end_audio,
            72,
            &|c, seed| {
                let mut r = machine(c, seed, 0x777d, 60);
                c.pc = 0x0028_5fb0;
                oc777_gate(c);
                oc777_audio_fixture(c, &mut r);
                oc777_pat(c, 0x002a_7bf8, r.next_u32() & 3, 0);
                oc777_pat(c, 0x002a_7f90, r.next_u32() & 0xf, 0);
                oc777_pat(c, 0x0041_aa88, r.next_u32() & 0xff, 0);
                oc777_pat(c, 0x0028_b830, r.next_u32() & 3, 0);
            },
        );
    }

    /// The first `n` calls of the fake for `addr` return 1.
    pub(crate) fn oc784_first_ones(c: &mut Cpu, addr: u32, n: u32) {
        c.bus.write32(oc784_count(addr), n);
    }

    /// `SSXAudioSystem_LoadPlaylist`: section found or not, how many names
    /// mismatch before the match (or none matches), and the unwinds. The song
    /// scan loop (`IsEOF` false) needs the real `IniParser_ReadToken`
    /// (0x2a7da0, no override slot), so `IsEOF` always reports end of file.
    #[test]
    fn ssx_audio_system_load_playlist_matches() {
        oc784_install_fakes();
        fakes::covered(
            ee::s07::f_00289520__generated,
            ssx_audio_system_load_playlist,
            250,
            &|c, seed| {
                let mut r = machine(c, seed, 0x784d, 10);
                oc784_arm(c);
                c.pc = 0x0028_9520;
                c.g[A0][0] = sx(OC784_OBJ);
                c.g[A1][0] = sx(OC784_FILE);
                c.bus.write32(GAME_GP + 0x46c, r.next_u32());
                c.bus.write32(OC784_OBJ + 0x510, r.next_u32());
                c.bus.write32(OC784_OBJ + 0x514, r.next_u32());
                if seed % 3 == 0 {
                    oc784_force_zero(c, 0x002a_7c68);
                } else {
                    oc784_force(c, 0x002a_7c68, 5);
                }
                oc784_force(c, 0x002a_7f90, 5);
                oc784_first_ones(c, 0x0041_65a8, (seed / 3) % 22);
                if (seed / 66) % 2 == 0 {
                    oc784_force_zero(c, 0x0041_65a8);
                }
            },
        );
    }

    /// Make the fake for `addr` return `v` in `v0` (stored +3).
    pub(crate) fn oc787_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc787_mail(addr), v.wrapping_add(3));
    }

    /// `SSXAudioSystem_ResetRaceSounds`: the race manager has 0..3 entries
    /// (or a negative count), the monitor fade count at `+0x59e4` is positive
    /// or not, and any call may unwind.
    #[test]
    fn ssx_audio_system_reset_race_sounds_matches() {
        oc787_install_fakes();
        fakes::covered(
            ee::s07::f_002871b0__generated,
            ssx_audio_system_reset_race_sounds,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x787f, 12);
                oc787_arm(c);
                c.pc = 0x0028_71b0;
                c.g[A0][0] = sx(OC787_OBJ);
                let raceman = OC787_AUX;
                oc787_force(c, 0x0028_b1d8, raceman);
                c.bus.write32(
                    raceman + 0x78,
                    [0u32, 1, 2, 3, 0xffff_ffff][(seed % 5) as usize],
                );
                fill(c, &mut r, raceman + 0x28, 4);
                c.bus.write32(
                    OC787_OBJ + 0x59e4,
                    [0u32, 1, 5, 0xffff_ffff][((seed / 5) % 4) as usize],
                );
                c.bus.write32(OC787_OBJ + 0x59e0, r.next_u32());
                c.bus.write32(OC787_OBJ + 0x118, OC787_AUX + 0x400);
                c.bus.write32(OC787_AUX + 0x400, OC787_AUX + 0x800);
                c.bus.write32(GAME_GP + 0x21e4, r.next_u32());
            },
        );
    }

    #[test]
    fn ssx_audio_system_rider_init_stub_matches() {
        check(
            ee::s07::f_00289af0__generated,
            ssx_audio_system_rider_init_stub,
            20,
            &|c, seed| {
                let _ = machine(c, seed, 0x7950_0005, 0);
            },
        );
    }

    use crate::difftest::fresh;
    use crate::difftest::put_words;
    use crate::difftest::run_generated;
    use std::collections::BTreeSet;

    /// Gate word of the oc805 fakes: they act only on machines that hold the magic here.
    pub(crate) const OC805_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC805_MAGIC: u32 = 0x0c80_5a5a;

    /// Result queues of the fakes: one 0x80-byte record per callee at `OC805_Q + 0x80 * index`
    /// (word 0: calls made, word 1: code after the queue ran out, words 2..: queued codes).
    pub(crate) const OC805_Q: u32 = DATA + 0x20_1000;

    pub(crate) const OC805_QLEN: u32 = 24;

    /// Every callee of the four functions (their override slots are faked).
    pub(crate) const OC805_CALLEES: &[u32] = &[
        0x0026ada8, 0x0026b310, 0x0026b410, 0x0026ba68, 0x00317e98, 0x003e22f0, 0x00287700,
        0x00287968, 0x00287fc8, 0x00288f60, 0x00289470, 0x00289520, 0x0028b9b8, 0x0028f700,
        0x002948a0, 0x00297f70, 0x0029ed90, 0x002a4590, 0x002aa490, 0x002ab200, 0x002ade88,
        0x002b0ce0, 0x002b0e60, 0x00317e30, 0x003b5a28, 0x0014dc80, 0x0014dd58, 0x00288d18,
        0x0028a1e8, 0x0028bc58, 0x0028bda8, 0x0029f3f8, 0x002a6d78, 0x002a77f8, 0x002a7890,
        0x002a7b08, 0x002a7bf8, 0x002a7da0, 0x002a7f90, 0x002a7ff8, 0x004165a8, 0x004166f4,
        0x00417828, 0x0041aa88, 0x002b4388, 0x003b8160,
    ];

    /// A queue code: 0 = the fake's default pointer, else the forced `v0` plus one.
    pub(crate) fn oc805_code(v: u32) -> u32 {
        v.wrapping_add(1)
    }

    /// Queue the results of the fake for `addr`; `after` is used once the queue ran out.
    pub(crate) fn oc805_queue(c: &mut Cpu, addr: u32, codes: &[u32], after: u32) {
        let i = OC805_CALLEES.iter().position(|&a| a == addr).unwrap() as u32;
        let q = OC805_Q + 0x80 * i;
        c.bus.write32(q, 0);
        c.bus.write32(q + 4, after);
        for (n, &code) in codes.iter().enumerate() {
            c.bus.write32(q + 8 + 4 * n as u32, code);
        }
    }

    /// Random queue: each entry is the forced value `zero` or the default pointer.
    pub(crate) fn oc805_random_queue(
        c: &mut Cpu,
        r: &mut Rng,
        addr: u32,
        len: u32,
        zero_in: u32,
        after: u32,
    ) {
        let codes: Vec<u32> = (0..len)
            .map(|_| {
                if r.next_u32() % zero_in == 0 {
                    oc805_code(0)
                } else {
                    0
                }
            })
            .collect();
        oc805_queue(c, addr, &codes, after);
    }

    pub(crate) fn oc805_unhooked(_c: &mut Cpu) {
        unreachable!("oc805 fake called before it was hooked");
    }

    /// The fake callee for `A`: logs the call like `fakes::fake_call` and returns the queued
    /// result (or the default pointer); machines without the gate word run the old body.
    pub(crate) fn oc805_fake<const A: u32>(c: &mut Cpu) {
        if c.bus.read32(OC805_CTRL) != OC805_MAGIC {
            return fakes::fall_back(c, fakes::prev_cell(oc805_fake::<A>), oc805_unhooked);
        }
        let i = OC805_CALLEES.iter().position(|&a| a == A).unwrap() as u32;
        let q = OC805_Q + 0x80 * i;
        let n = c.bus.read32(q);
        c.bus.write32(q, n + 1);
        let code = if n < OC805_QLEN {
            c.bus.read32(q + 8 + 4 * n)
        } else {
            c.bus.read32(q + 4)
        };
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, A);
        if c.pc == ra && code != 0 {
            c.g[V0][0] = code.wrapping_sub(1) as u64;
        }
    }

    macro_rules! oc805_hooks {
        ($($slot:path => $addr:literal),* $(,)?) => {
            pub(crate) fn oc805_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), oc805_fake::<$addr>, fakes::prev_cell(oc805_fake::<$addr>)); )*
            }
        };
    }

    oc805_hooks! {
        ee::s07::OVERRIDE_0026ADA8 => 0x0026ada8,
        ee::s07::OVERRIDE_0026B310 => 0x0026b310,
        ee::s07::OVERRIDE_0026B410 => 0x0026b410,
        ee::s07::OVERRIDE_0026BA68 => 0x0026ba68,
        ee::s10::OVERRIDE_00317E98 => 0x00317e98,
        ee::s14::OVERRIDE_003E22F0 => 0x003e22f0,
        ee::s07::OVERRIDE_00287700 => 0x00287700,
        ee::s07::OVERRIDE_00287968 => 0x00287968,
        ee::s07::OVERRIDE_00287FC8 => 0x00287fc8,
        ee::s07::OVERRIDE_00288F60 => 0x00288f60,
        ee::s07::OVERRIDE_00289470 => 0x00289470,
        ee::s07::OVERRIDE_00289520 => 0x00289520,
        ee::s07::OVERRIDE_0028B9B8 => 0x0028b9b8,
        ee::s07::OVERRIDE_0028F700 => 0x0028f700,
        ee::s07::OVERRIDE_002948A0 => 0x002948a0,
        ee::s08::OVERRIDE_00297F70 => 0x00297f70,
        ee::s08::OVERRIDE_0029ED90 => 0x0029ed90,
        ee::s08::OVERRIDE_002A4590 => 0x002a4590,
        ee::s08::OVERRIDE_002AA490 => 0x002aa490,
        ee::s08::OVERRIDE_002AB200 => 0x002ab200,
        ee::s08::OVERRIDE_002ADE88 => 0x002ade88,
        ee::s08::OVERRIDE_002B0CE0 => 0x002b0ce0,
        ee::s08::OVERRIDE_002B0E60 => 0x002b0e60,
        ee::s10::OVERRIDE_00317E30 => 0x00317e30,
        ee::s13::OVERRIDE_003B5A28 => 0x003b5a28,
        ee::s01::OVERRIDE_0014DC80 => 0x0014dc80,
        ee::s01::OVERRIDE_0014DD58 => 0x0014dd58,
        ee::s07::OVERRIDE_00288D18 => 0x00288d18,
        ee::s07::OVERRIDE_0028A1E8 => 0x0028a1e8,
        ee::s07::OVERRIDE_0028BC58 => 0x0028bc58,
        ee::s07::OVERRIDE_0028BDA8 => 0x0028bda8,
        ee::s08::OVERRIDE_0029F3F8 => 0x0029f3f8,
        ee::s08::OVERRIDE_002A6D78 => 0x002a6d78,
        ee::s08::OVERRIDE_002A77F8 => 0x002a77f8,
        ee::s08::OVERRIDE_002A7890 => 0x002a7890,
        ee::s08::OVERRIDE_002A7B08 => 0x002a7b08,
        ee::s08::OVERRIDE_002A7BF8 => 0x002a7bf8,
        ee::s08::OVERRIDE_002A7DA0 => 0x002a7da0,
        ee::s08::OVERRIDE_002A7F90 => 0x002a7f90,
        ee::s08::OVERRIDE_002A7FF8 => 0x002a7ff8,
        ee::s15::OVERRIDE_004165A8 => 0x004165a8,
        ee::s15::OVERRIDE_004166F4 => 0x004166f4,
        ee::s15::OVERRIDE_00417828 => 0x00417828,
        ee::s15::OVERRIDE_0041AA88 => 0x0041aa88,
        ee::s08::OVERRIDE_002B4388 => 0x002b4388,
        ee::s13::OVERRIDE_003B8160 => 0x003b8160,
    }

    /// Scrambled machine with the fake hooks, the gate word set and `pc` at the entry.
    pub(crate) fn oc805_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32, entry: u32) -> Rng {
        oc805_install_fakes();
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC805_CTRL, OC805_MAGIC);
        c.pc = entry;
        r
    }

    /// Distinct total cycle counts of the generated body over the seeds (one per path taken).
    pub(crate) fn oc805_paths(
        generated: fn(&mut Cpu),
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
    ) -> usize {
        let mut seen = BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            setup(&mut c, seed);
            let before = c.cycles;
            run_generated(&mut c, generated);
            seen.insert(c.cycles - before);
        }
        seen.len()
    }

    pub(crate) fn ctor_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc805_machine(c, seed, 0x4c68, 26, 0x0028_4c68);
        let this = DATA + 0x100;
        c.g[A0][0] = this as u64;
        // a1: construct the sub-systems (1) or only fix up the existing vtable (0).
        c.g[A1][0] = (seed % 2) as u64;
        c.g[A2][0] = r.next_u32() as u64;
        c.g[A3][0] = r.next_u32() as u64;
        c.g[T0][0] = r.next_u32() as u64;
        // The controller pointers the a1 == 0 path reads: +0x118 -> P -> Q, Q+0x1d8 -> R.
        let (p, q, rr) = (DATA + 0x2_0000, DATA + 0x2_0100, DATA + 0x2_0400);
        c.bus.write32(this + 0x118, p);
        c.bus.write32(p, q);
        c.bus.write32(q + 0x1d8, rr);
    }

    /// `SSXAudioSystem_ctor`: with and without the sub-system construction, with unwinds.
    #[test]
    fn ssx_audio_system_ctor_matches() {
        check(
            ee::s07::f_00284c68__generated,
            ssx_audio_system_ctor,
            200,
            &ctor_setup,
        );
        assert!(oc805_paths(ee::s07::f_00284c68__generated, 200, &ctor_setup) >= 2);
    }

    pub(crate) fn load_world_audio_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc805_machine(c, seed, 0x62a8, 70, 0x0028_62a8);
        let this = DATA + 0x100;
        c.g[A0][0] = this as u64;
        c.bus.write32(this + 0x118, DATA + 0x2_0000);
        // cBENewRaceInterface.cGamePath == 2 or not.
        c.bus
            .write8(0x0053_5bc8 + 0x49, if seed % 3 == 0 { 2 } else { 1 });
        // The ini parser: FindSection, sections (strcmp 0 = the world section), keys
        // (strcasecmp 0 = this key), section ends. Past their queues the section scan stops
        // and the key loops end.
        oc805_random_queue(c, &mut r, 0x002a_7bf8, 2, 3, 0);
        oc805_random_queue(c, &mut r, 0x002a_7b08, 4, 3, oc805_code(0));
        oc805_random_queue(c, &mut r, 0x0041_65a8, 4, 2, 0);
        oc805_random_queue(c, &mut r, 0x0041_aa88, 24, 3, 0);
        // AtSectionEnd: a forced 0 keeps scanning keys, the default pointer (nonzero) ends
        // the section; past the queue it always ends.
        let ends: Vec<u32> = (0..24)
            .map(|_| {
                if r.next_u32() % 4 == 0 {
                    0
                } else {
                    oc805_code(0)
                }
            })
            .collect();
        oc805_queue(c, 0x002a_7f90, &ends, 0);
    }

    /// `SSXAudioSystem_LoadWorldAudio`: the ini scan with random section and key matches.
    #[test]
    fn ssx_audio_system_load_world_audio_matches() {
        check(
            ee::s07::f_002862a8__generated,
            ssx_audio_system_load_world_audio,
            300,
            &load_world_audio_setup,
        );
        assert!(oc805_paths(ee::s07::f_002862a8__generated, 300, &load_world_audio_setup) >= 6);
    }

    pub(crate) fn set_channel_volume_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc805_machine(c, seed, 0x7700, 3, 0x0028_7700);
        let this = DATA + 0x1_0000;
        c.g[A0][0] = this as u64;
        // Channels 0..=10 (the jump table), 11 and 12 (out of range), and -1.
        let ch = seed % 14;
        c.g[A1][0] = if ch == 13 { u64::MAX } else { ch as u64 };
        c.f[12] = if seed % 5 == 0 {
            r.edge_f32()
        } else {
            r.f32(1.5)
        };
        // The jump table at 0x482900 and the per-channel scale constants at gp + 0x414.
        let arms = [
            0x0028_7760u32,
            0x0028_776c,
            0x0028_778c,
            0x0028_7794,
            0x0028_779c,
            0x0028_77a4,
            0x0028_77ac,
            0x0028_77b4,
            0x0028_77bc,
            0x0028_77c4,
            0x0028_77cc,
        ];
        put_words(c, 0x0048_2900, &arms);
        let consts: Vec<u32> = (0..0x14).map(|_| r.f32(2.0)).collect();
        put_words(c, GAME_GP + 0x414, &consts);
        // The channel records (+0x62bc, 16 bytes each) and the two gain tables (+0x6394, +0x63e4).
        let rec: Vec<u32> = (0..0x60).map(|_| r.f32(2.0)).collect();
        put_words(c, this + 0x6280, &rec);
        let gains: Vec<u32> = (0..0x40).map(|_| r.f32(2.0)).collect();
        put_words(c, this + 0x6360, &gains);
    }

    /// `SSXAudioSystem_SetChannelVolume`: every channel arm of the jump table and the range check.
    #[test]
    fn ssx_audio_system_set_channel_volume_matches() {
        check(
            ee::s07::f_00287700__generated,
            ssx_audio_system_set_channel_volume,
            280,
            &set_channel_volume_setup,
        );
        assert!(
            oc805_paths(
                ee::s07::f_00287700__generated,
                280,
                &set_channel_volume_setup
            ) >= 8
        );
    }

    /// Gate word of this module's callee fakes (in the test machine's RAM).
    pub(crate) const OC806_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC806_MAGIC: u32 = 0x4f43_3830;

    /// Per-fake result policy words: `OC806_CTRL + 0x10 + 4 * index`.
    pub(crate) const OC806_POLICY: u32 = OC806_CTRL + 0x10;

    /// The shared salt of the policy hash.
    pub(crate) const OC806_SALT: u32 = OC806_CTRL + 4;

    // Fake indexes (policy slots).
    pub(crate) const FX_NEXT_SECTION: u32 = 2;

    pub(crate) const FX_AT_SECTION_END: u32 = 4;

    pub(crate) const FX_STRCASECMP: u32 = 6;

    pub(crate) const FX_PATH_EXISTS: u32 = 9;

    /// The fake callee: logs the call like `fakes::fake_call` (arguments,
    /// registers, cycles), then, when the machine's policy word for this fake
    /// is `p` in 1..=9, returns 1 with probability `(p - 1) / 8` and 0
    /// otherwise (a pointer-like value when the policy is 0).
    pub(crate) fn oc806_fake_call(c: &mut Cpu, addr: u32, index: u32) {
        let n = c.bus.read32(fakes::LOG_COUNT);
        let ra = c.g[RA][0] as u32;
        let policy = c.bus.read32(OC806_POLICY + 4 * index);
        let salt = c.bus.read32(OC806_SALT);
        fakes::fake_call(c, addr);
        if c.pc == ra && policy != 0 {
            let h = fakes::mix(u64::from(salt) ^ u64::from(n) << 20 ^ u64::from(addr));
            c.g[V0][0] = ((h & 7) < u64::from(policy - 1)) as u64;
        }
    }

    macro_rules! oc806_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr, $index:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC806_CTRL) != OC806_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc806_fake_call(c, $addr, $index);
                }
            )*
            fn oc806_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc806_fakes!(
        (
            oc806_fake_2a77f8,
            OC806_PREV_2A77F8,
            ee::s08::OVERRIDE_002A77F8,
            ee::s08::f_002a77f8__generated,
            0x002a_77f8,
            0
        ),
        (
            oc806_fake_2a7890,
            OC806_PREV_2A7890,
            ee::s08::OVERRIDE_002A7890,
            ee::s08::f_002a7890__generated,
            0x002a_7890,
            1
        ),
        (
            oc806_fake_2a7b08,
            OC806_PREV_2A7B08,
            ee::s08::OVERRIDE_002A7B08,
            ee::s08::f_002a7b08__generated,
            0x002a_7b08,
            2
        ),
        (
            oc806_fake_2a7da0,
            OC806_PREV_2A7DA0,
            ee::s08::OVERRIDE_002A7DA0,
            ee::s08::f_002a7da0__generated,
            0x002a_7da0,
            3
        ),
        (
            oc806_fake_2a7f90,
            OC806_PREV_2A7F90,
            ee::s08::OVERRIDE_002A7F90,
            ee::s08::f_002a7f90__generated,
            0x002a_7f90,
            4
        ),
        (
            oc806_fake_2a80d8,
            OC806_PREV_2A80D8,
            ee::s08::OVERRIDE_002A80D8,
            ee::s08::f_002a80d8__generated,
            0x002a_80d8,
            5
        ),
        (
            oc806_fake_41aa88,
            OC806_PREV_41AA88,
            ee::s15::OVERRIDE_0041AA88,
            ee::s15::f_0041aa88__generated,
            0x0041_aa88,
            6
        ),
        (
            oc806_fake_2875d0,
            OC806_PREV_2875D0,
            ee::s07::OVERRIDE_002875D0,
            ee::s07::f_002875d0__generated,
            0x0028_75d0,
            7
        ),
        (
            oc806_fake_287700,
            OC806_PREV_287700,
            ee::s07::OVERRIDE_00287700,
            ee::s07::f_00287700__generated,
            0x0028_7700,
            8
        ),
        (
            oc806_fake_317f38,
            OC806_PREV_317F38,
            ee::s10::OVERRIDE_00317F38,
            ee::s10::f_00317f38__generated,
            0x0031_7f38,
            9
        ),
        (
            oc806_fake_4166f4,
            OC806_PREV_4166F4,
            ee::s15::OVERRIDE_004166F4,
            ee::s15::f_004166f4__generated,
            0x0041_66f4,
            10
        ),
        (
            oc806_fake_2adf60,
            OC806_PREV_2ADF60,
            ee::s08::OVERRIDE_002ADF60,
            ee::s08::f_002adf60__generated,
            0x002a_df60,
            11
        ),
        (
            oc806_fake_28d8a0,
            OC806_PREV_28D8A0,
            ee::s07::OVERRIDE_0028D8A0,
            ee::s07::f_0028d8a0__generated,
            0x0028_d8a0,
            12
        ),
        (
            oc806_fake_2b42b8,
            OC806_PREV_2B42B8,
            ee::s08::OVERRIDE_002B42B8,
            ee::s08::f_002b42b8__generated,
            0x002b_42b8,
            13
        ),
    );

    /// A scrambled machine with the fakes armed: the object at `DATA + 0x1000`
    /// in `a0`, the fake policies and `pc` at the function's entry.
    pub(crate) fn oc806_machine(
        c: &mut Cpu,
        seed: u32,
        salt: u64,
        calls: u32,
        entry: u32,
        policies: &[(u32, u32)],
    ) {
        oc806_install_fakes();
        fakes::machine(c, seed, salt, calls);
        c.bus.write32(OC806_CTRL, OC806_MAGIC);
        c.bus.write32(OC806_SALT, seed);
        for &(index, policy) in policies {
            c.bus.write32(OC806_POLICY + 4 * index, policy);
        }
        c.g[A0][0] = u64::from(DATA + 0x1000);
        c.pc = entry;
    }

    /// Mix-file parse: keys that match or not (policy of the `strcasecmp`
    /// fake), sections ending early or late, up to 25 mixes, unwinds.
    #[test]
    fn ssx_audio_system_load_volume_config_matches() {
        fakes::covered(
            ee::s07::f_00287fc8__generated,
            ssx_audio_system_load_volume_config,
            300,
            &|c, seed| {
                oc806_machine(
                    c,
                    seed,
                    0x806_0001,
                    150,
                    0x0028_7fc8,
                    &[
                        (FX_STRCASECMP, 1 + (seed / 3) % 9),
                        (FX_AT_SECTION_END, 4 + seed % 6),
                        (FX_NEXT_SECTION, 2 + (seed / 7) % 8),
                    ],
                );
            },
        );
    }

    /// Debug presets 0..=5 (jump table), larger values store only.
    #[test]
    fn ssx_audio_system_set_debug_mode_matches() {
        fakes::covered(
            ee::s07::f_00288f60__generated,
            ssx_audio_system_set_debug_mode,
            240,
            &|c, seed| {
                oc806_machine(c, seed, 0x806_0002, 12, 0x0028_8f60, &[]);
                elf_image(c);
                c.g[A1][0] = match seed % 8 {
                    7 => 100,
                    m => u64::from(m),
                };
            },
        );
    }

    /// Playlist file missing or present, 0..=20 sections.
    #[test]
    fn ssx_audio_system_read_playlist_names_matches() {
        fakes::covered(
            ee::s07::f_00289470__generated,
            ssx_audio_system_read_playlist_names,
            240,
            &|c, seed| {
                oc806_machine(
                    c,
                    seed,
                    0x806_0003,
                    10,
                    0x0028_9470,
                    &[
                        (FX_PATH_EXISTS, if seed % 4 == 0 { 1 } else { 9 }),
                        (FX_NEXT_SECTION, 2 + (seed / 4) % 8),
                    ],
                );
            },
        );
    }

    use crate::game::be::beprepoststate::tests::belib as belib_m2;
    pub(crate) use crate::game::fe::festateloadhint::tests::oc1009_machine;

    use crate::game::sound::soundsys::tests::oc739_gate;
    use crate::game::sound::soundsys::tests::oc739_install;
    use crate::game::sound::soundsys::tests::OC739_CTRL;

    /// `SSXAudioSystem_dtor`: flags 0..3 (vtable fix-up copies with flags 0,
    /// base destructors with bit 1, delete with bit 0), with and without the
    /// big-file sync and the play history.
    #[test]
    fn ssx_audio_system_dtor_matches() {
        check(
            ee::s07::f_00285210__generated,
            ssx_audio_system_dtor,
            160,
            &|c, seed| {
                let mut r = oc1009_machine(c, seed, 0x0028_5210, 12);
                let this = DATA + 0x23_0000;
                let (ptr, bm, ss) = (DATA + 0x23_1000, DATA + 0x23_2000, DATA + 0x23_3000);
                c.bus.write32(this + 0x118, ptr);
                c.bus.write32(ptr, bm);
                c.bus.write32(bm + 0x1d8, ss);
                c.bus.write32(
                    this + 0x5728,
                    if (seed >> 2) & 1 == 1 {
                        r.next_u32() | 1
                    } else {
                        0
                    },
                );
                c.bus.write32(
                    this + 0x5980,
                    if (seed >> 3) & 1 == 1 {
                        r.next_u32() | 1
                    } else {
                        0
                    },
                );
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 4) as u64;
            },
        );
    }

    /// `SndLoadQueue_FinishCurrent`: four rings (size, data, head, tail at
    /// +0x18c/+0x19c/+0x1ac/+0x1bc; empty, wrapping and plain pops), busy or
    /// not, every kind (0, 1, 2, other, empty ring), the async layer's
    /// cancel and result (gated fakes), the vtable notification, with and
    /// without flush; the next load starts through the real `StartNext`.
    #[test]
    fn snd_load_queue_finish_current_matches() {
        oc739_install();
        fakes::covered(
            ee::s07::f_00289df0__generated,
            snd_load_queue_finish_current,
            500,
            &|c, seed| {
                let mut r = machine(c, seed, 0x289df0, 6);
                oc739_gate(c);
                c.bus.write32(OC739_CTRL + 0x20, r.next_u32() % 3);
                c.bus.write32(OC739_CTRL + 0x24, r.next_u32() % 2);
                let this = DATA + 0x100;
                let vt = DATA + 0x2000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 3 == 0 { 1 } else { 0 };
                c.bus.write32(this, DATA + 0x5000);
                c.bus.write32(this + 4, (seed % 4 != 0) as u32);
                let kinds = [0u32, 1, 2, 3, u32::MAX, 1, 0, 2, 1, 2];
                for k in 0..4u32 {
                    let base = this + 0x18c + 0x10 * k;
                    let size = 1 + r.next_u32() % 4;
                    let data = DATA + 0x1000 + 0x40 * k;
                    let head = r.next_u32() % size;
                    let mut tail = r.next_u32() % size;
                    if r.next_u32() % 4 != 0 && head == tail {
                        tail = (tail + 1) % size;
                    }
                    c.bus.write32(base, size);
                    c.bus.write32(base + 4, data);
                    c.bus.write32(base + 8, head);
                    c.bus.write32(base + 0xc, tail);
                    for i in 0..size {
                        let v = if k == 2 {
                            kinds[(r.next_u32() % 10) as usize]
                        } else {
                            r.next_u32() % 4
                        };
                        c.bus.write32(data + 4 * i, v);
                    }
                }
                c.bus.write32(this + 0x1cc, r.next_u32());
                c.bus.write32(this + 0x1d0, r.next_u32());
                c.bus.write32(this + 0x1d4, vt);
                for off in (0..0x40u32).step_by(4) {
                    let lh_slot = matches!(off, 0x10 | 0x18 | 0x20 | 0x28);
                    c.bus
                        .write32(vt + off, if lh_slot { 0 } else { fakes::FAKE });
                }
                c.bus
                    .write16(vt + 0x28, (r.next_u32() as i16 as i32 & !3) as u16);
                c.pc = 0x0028_9df0;
            },
        );
    }

    // Fixture objects live in DATA+0x200000 and up: clear of the fake log
    // (DATA+0x100000) and of the pointers the fakes return (DATA..DATA+0x40000).

    /// Gate word of this module's fakes, and its magic value.
    pub(crate) const OC1017_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC1017_MAGIC: u32 = 0x4f43_1017;

    /// The audio system (`self`) and the objects its fields point to.
    pub(crate) const OC1017_SELF: u32 = DATA + 0x21_0000;

    pub(crate) const OC1017_OBJ: u32 = DATA + 0x23_0000;

    /// Chain links (`LINK + 0x100*i`), their vtables (`VT + 0x100*i`).
    pub(crate) const OC1017_LINK: u32 = DATA + 0x24_0000;

    pub(crate) const OC1017_VT: u32 = DATA + 0x25_0000;

    /// The manager (`self->mpMgr` = `*(self+0x118)`), its config objects.
    pub(crate) const OC1017_MGR: u32 = DATA + 0x26_0000;

    pub(crate) const OC1017_CFG: u32 = DATA + 0x27_0000;

    pub(crate) const OC1017_CFG2: u32 = DATA + 0x28_0000;

    pub(crate) fn oc1017_w(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(addr, v);
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns 0
    /// if the mailbox word at `CTRL + mail` is set (the unused word `0x1fc`
    /// never is).
    pub(crate) fn oc1017_fake_call(c: &mut Cpu, addr: u32, mail: u32) {
        let zero = c.bus.read32(OC1017_CTRL + mail) != 0;
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if zero && c.pc == ra {
            c.g[V0][0] = 0;
        }
    }

    macro_rules! oc1017_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr, $mail:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC1017_CTRL) != OC1017_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc1017_fake_call(c, $addr, $mail);
                }
            )*
            fn oc1017_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc1017_fakes!(
        (
            oc1017_fake_317670,
            OC1017_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670,
            0x100
        ),
        (
            oc1017_fake_317d70,
            OC1017_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70,
            0x1fc
        ),
        (
            oc1017_fake_186a08,
            OC1017_PREV_186A08,
            ee::s02::OVERRIDE_00186A08,
            ee::s02::f_00186a08__generated,
            0x0018_6a08,
            0x1fc
        ),
        (
            oc1017_fake_39f400,
            OC1017_PREV_39F400,
            ee::s13::OVERRIDE_0039F400,
            ee::s13::f_0039f400__generated,
            0x0039_f400,
            0x1fc
        ),
        (
            oc1017_fake_242eb8,
            OC1017_PREV_242EB8,
            ee::s06::OVERRIDE_00242EB8,
            ee::s06::f_00242eb8__generated,
            0x0024_2eb8,
            0x1fc
        ),
        (
            oc1017_fake_289bb8,
            OC1017_PREV_289BB8,
            ee::s07::OVERRIDE_00289BB8,
            ee::s07::f_00289bb8__generated,
            0x0028_9bb8,
            0x1fc
        ),
        (
            oc1017_fake_285be0,
            OC1017_PREV_285BE0,
            ee::s07::OVERRIDE_00285BE0,
            ee::s07::f_00285be0__generated,
            0x0028_5be0,
            0x1fc
        ),
        (
            oc1017_fake_2b3ac0,
            OC1017_PREV_2B3AC0,
            ee::s08::OVERRIDE_002B3AC0,
            ee::s08::f_002b3ac0__generated,
            0x002b_3ac0,
            0x1fc
        ),
        (
            oc1017_fake_29d678,
            OC1017_PREV_29D678,
            ee::s08::OVERRIDE_0029D678,
            ee::s08::f_0029d678__generated,
            0x0029_d678,
            0x1fc
        ),
        (
            oc1017_fake_2a4550,
            OC1017_PREV_2A4550,
            ee::s08::OVERRIDE_002A4550,
            ee::s08::f_002a4550__generated,
            0x002a_4550,
            0x1fc
        ),
        (
            oc1017_fake_292b48,
            OC1017_PREV_292B48,
            ee::s07::OVERRIDE_00292B48,
            ee::s07::f_00292b48__generated,
            0x0029_2b48,
            0x1fc
        ),
        (
            oc1017_fake_2980b0,
            OC1017_PREV_2980B0,
            ee::s08::OVERRIDE_002980B0,
            ee::s08::f_002980b0__generated,
            0x0029_80b0,
            0x1fc
        ),
        (
            oc1017_fake_2883b0,
            OC1017_PREV_2883B0,
            ee::s07::OVERRIDE_002883B0,
            ee::s07::f_002883b0__generated,
            0x0028_83b0,
            0x1fc
        ),
        (
            oc1017_fake_2adda0,
            OC1017_PREV_2ADDA0,
            ee::s08::OVERRIDE_002ADDA0,
            ee::s08::f_002adda0__generated,
            0x002a_dda0,
            0x1fc
        ),
        (
            oc1017_fake_2b11b0,
            OC1017_PREV_2B11B0,
            ee::s08::OVERRIDE_002B11B0,
            ee::s08::f_002b11b0__generated,
            0x002b_11b0,
            0x1fc
        ),
        (
            oc1017_fake_2ad650,
            OC1017_PREV_2AD650,
            ee::s08::OVERRIDE_002AD650,
            ee::s08::f_002ad650__generated,
            0x002a_d650,
            0x1fc
        ),
        (
            oc1017_fake_28be90,
            OC1017_PREV_28BE90,
            ee::s07::OVERRIDE_0028BE90,
            ee::s07::f_0028be90__generated,
            0x0028_be90,
            0x1fc
        ),
        (
            oc1017_fake_2871b0,
            OC1017_PREV_2871B0,
            ee::s07::OVERRIDE_002871B0,
            ee::s07::f_002871b0__generated,
            0x0028_71b0,
            0x1fc
        ),
        (
            oc1017_fake_2a4040,
            OC1017_PREV_2A4040,
            ee::s08::OVERRIDE_002A4040,
            ee::s08::f_002a4040__generated,
            0x002a_4040,
            0x104
        ),
        (
            oc1017_fake_295028,
            OC1017_PREV_295028,
            ee::s08::OVERRIDE_00295028,
            ee::s08::f_00295028__generated,
            0x0029_5028,
            0x108
        ),
        (
            oc1017_fake_295628,
            OC1017_PREV_295628,
            ee::s08::OVERRIDE_00295628,
            ee::s08::f_00295628__generated,
            0x0029_5628,
            0x1fc
        ),
        (
            oc1017_fake_28b1c8,
            OC1017_PREV_28B1C8,
            ee::s07::OVERRIDE_0028B1C8,
            ee::s07::f_0028b1c8__generated,
            0x0028_b1c8,
            0x1fc
        ),
        (
            oc1017_fake_14dc80,
            OC1017_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80,
            0x1fc
        ),
        (
            oc1017_fake_14dd58,
            OC1017_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58,
            0x1fc
        ),
        (
            oc1017_fake_28d488,
            OC1017_PREV_28D488,
            ee::s07::OVERRIDE_0028D488,
            ee::s07::f_0028d488__generated,
            0x0028_d488,
            0x1fc
        ),
        (
            oc1017_fake_28cf98,
            OC1017_PREV_28CF98,
            ee::s07::OVERRIDE_0028CF98,
            ee::s07::f_0028cf98__generated,
            0x0028_cf98,
            0x1fc
        ),
        (
            oc1017_fake_2a26f0,
            OC1017_PREV_2A26F0,
            ee::s08::OVERRIDE_002A26F0,
            ee::s08::f_002a26f0__generated,
            0x002a_26f0,
            0x1fc
        ),
        (
            oc1017_fake_2b1758,
            OC1017_PREV_2B1758,
            ee::s08::OVERRIDE_002B1758,
            ee::s08::f_002b1758__generated,
            0x002b_1758,
            0x1fc
        ),
        (
            oc1017_fake_2b4908,
            OC1017_PREV_2B4908,
            ee::s08::OVERRIDE_002B4908,
            ee::s08::f_002b4908__generated,
            0x002b_4908,
            0x1fc
        ),
    );

    /// One seed's machine: the fakes gated on, the mailboxes cleared.
    pub(crate) fn oc1017_setup(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        oc1017_install_fakes();
        let r = machine(c, seed, salt, 0);
        oc1017_w(c, OC1017_CTRL, OC1017_MAGIC);
        r
    }

    /// `self->mpMgr` (`*(self+0x118)`) with its config objects, and the
    /// chain links (`self+0x625c` .. `self+0x6c84`) of a seed's subset.
    pub(crate) fn oc1017_manager(c: &mut Cpu, r: &mut Rng, seed: u32) {
        oc1017_w(c, OC1017_SELF + 0x118, OC1017_MGR + 0x100);
        oc1017_w(c, OC1017_MGR + 0x100, OC1017_MGR);
        oc1017_w(c, OC1017_MGR + 0xab0, OC1017_CFG2);
        oc1017_w(c, OC1017_MGR + 0xacc, OC1017_CFG);
        c.bus.write16(OC1017_CFG2 + 0x10, r.next_u32() as u16);
        oc1017_w(c, OC1017_CFG2 + 0x14, fakes::FAKE + 0x20 * (seed % 3));
        oc1017_w(c, OC1017_CFG + 0x300, (seed % 2) as u32);
        oc1017_w(c, OC1017_CFG + 0x304, r.next_u32());
        oc1017_w(c, OC1017_CFG + 0x360, ((seed / 2) % 2) as u32);
        oc1017_w(c, OC1017_CFG + 0x364, r.next_u32());
    }

    /// A chain link object `i`: `link+4` is its vtable, `vt+8` a halfword
    /// delta and `vt+0xc` the function (an address the machine's miss hook
    /// answers).
    pub(crate) fn oc1017_link(c: &mut Cpu, r: &mut Rng, i: u32, seed: u32) -> u32 {
        let link = OC1017_LINK + 0x100 * i;
        let vt = OC1017_VT + 0x100 * i;
        oc1017_w(c, link + 4, vt);
        c.bus.write16(vt + 8, r.next_u32() as u16);
        oc1017_w(c, vt + 0xc, fakes::FAKE + 0x10 * ((seed + i) % 4));
        link
    }

    #[test]
    fn ssx_audio_system_set_music_event1_matches() {
        check(
            ee::s07::f_002861d0__generated,
            ssx_audio_system_set_music_event1,
            300,
            &|c, seed| {
                let mut r = oc1017_setup(c, seed, 0x0028_61d0);
                oc1017_w(c, OC1017_SELF + 0x5558, OC1017_OBJ);
                c.bus.write16(OC1017_OBJ + 0x20, r.next_u32() as u16);
                oc1017_w(c, OC1017_OBJ + 0x24, fakes::FAKE + 0x10 * (seed % 3));
                c.g[A0][0] = sx(OC1017_SELF);
            },
        );
    }

    #[test]
    fn ssx_audio_system_exit_game_matches() {
        check(
            ee::s07::f_00286a80__generated,
            ssx_audio_system_exit_game,
            300,
            &|c, seed| {
                let mut r = oc1017_setup(c, seed, 0x0028_6a80);
                oc1017_manager(c, &mut r, seed);
                // Each chain link present on the seed's bits.
                let fields = [0x625c, 0x6260, 0x5fa0, 0x5fa4, 0x6c80, 0x6c84];
                for (i, &off) in fields.iter().enumerate() {
                    if (seed >> i) & 1 == 1 {
                        let link = oc1017_link(c, &mut r, i as u32, seed);
                        oc1017_w(c, OC1017_SELF + off, link);
                    }
                }
                c.g[A0][0] = sx(OC1017_SELF);
            },
        );
    }

    #[test]
    fn ssx_audio_system_unload_memory_bank_matches() {
        check(
            ee::s07::f_00286d18__generated,
            ssx_audio_system_unload_memory_bank,
            300,
            &|c, seed| {
                let mut r = oc1017_setup(c, seed, 0x0028_6d18);
                oc1017_manager(c, &mut r, seed);
                let bank = seed % 3;
                let id = 7u32;
                // The handle slot matches unless seed % 5 == 0; the bank's
                // object slot is empty on even seeds.
                let stored = if seed % 5 == 0 { id ^ 1 } else { id };
                oc1017_w(c, OC1017_SELF + 0x6c78 + 4 * bank, stored);
                let obj_slot = if seed % 2 == 0 { 0 } else { OC1017_OBJ };
                oc1017_w(c, OC1017_SELF + 0x6c70 + 4 * bank, obj_slot);
                c.g[A0][0] = sx(OC1017_SELF);
                c.g[A1][0] = ((bank << 8) | id) as u64;
            },
        );
    }

    #[test]
    fn ssx_audio_system_on_quick_restart_matches() {
        check(
            ee::s07::f_002870a0__generated,
            ssx_audio_system_on_quick_restart,
            300,
            &|c, seed| {
                let _ = oc1017_setup(c, seed, 0x0028_70a0);
                // IsEventMode4 is 0 on seeds % 3 == 0 (the chain goes on);
                // IsTrackedSoundPlaying is 0 on even seeds (TrackSound runs).
                oc1017_w(c, OC1017_CTRL + 0x104, (seed % 3 == 0) as u32);
                oc1017_w(c, OC1017_CTRL + 0x108, (seed % 2 == 0) as u32);
                c.g[A0][0] = sx(OC1017_SELF);
            },
        );
    }

    #[test]
    fn ssx_audio_system_start_pre_race_music_matches() {
        check(
            ee::s07::f_002872a8__generated,
            ssx_audio_system_start_pre_race_music,
            300,
            &|c, seed| {
                let _ = oc1017_setup(c, seed, 0x0028_72a8);
                // The game module's config bytes at 0x535bc8+0x48/0x49/0x4a.
                c.bus
                    .write8(0x0053_5c10, [5u8, 6, 0x20][(seed % 3) as usize]);
                c.bus
                    .write8(0x0053_5c12, [4u8, 5, 9][((seed / 3) % 3) as usize]);
                c.bus
                    .write8(0x0053_5c11, [2u8, 1][((seed / 9) % 2) as usize]);
                oc1017_w(
                    c,
                    OC1017_SELF + 0x530,
                    if (seed / 18) % 2 == 0 { 0 } else { 0x1000 },
                );
                oc1017_w(c, OC1017_SELF + 0x5558, OC1017_LINK);
                c.bus.write16(OC1017_LINK + 0x20, (seed * 37) as u16);
                oc1017_w(c, OC1017_LINK + 0x24, fakes::FAKE + 0x10);
                oc1017_w(c, OC1017_CTRL + 0x108, ((seed / 36) % 2 == 0) as u32);
                c.g[A0][0] = sx(OC1017_SELF);
            },
        );
    }

    /// Gate word of this module's callee fakes, in the test machine's RAM.
    pub(crate) const OC1018_CTRL: u32 = DATA + 0x28_0000;

    pub(crate) const OC1018_MAGIC: u32 = 0x1018_c0de;

    /// Result of the fake `FindRider` (a rider pointer).
    pub(crate) const OC1018_RIDER: u32 = OC1018_CTRL + 0x10;

    /// Result of the fake `MusicPlayer_GetSongTag`.
    pub(crate) const OC1018_TAG: u32 = OC1018_CTRL + 0x14;

    /// The fake `FindRider`: logs the call, returns the rider word when gated on.
    pub(crate) fn oc1018_find_rider(c: &mut Cpu) {
        if c.bus.read32(OC1018_CTRL) != OC1018_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1018_find_rider),
                ee::s07::f_00285d98__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        let rider = c.bus.read32(OC1018_RIDER);
        fakes::fake_call(c, 0x0028_5d98);
        if c.pc == ra {
            c.g[V0][0] = sx(rider);
        }
    }

    /// The fake `MusicPlayer_GetSongTag`: logs, returns the tag word when gated on.
    pub(crate) fn oc1018_get_song_tag(c: &mut Cpu) {
        if c.bus.read32(OC1018_CTRL) != OC1018_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1018_get_song_tag),
                ee::s08::f_002b49e0__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        let tag = c.bus.read32(OC1018_TAG);
        fakes::fake_call(c, 0x002b_49e0);
        if c.pc == ra {
            c.g[V0][0] = sx(tag);
        }
    }

    /// The fake `SSXAudioSystem_PlayMusic`: logs the call only.
    pub(crate) fn oc1018_play_music(c: &mut Cpu) {
        if c.bus.read32(OC1018_CTRL) != OC1018_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1018_play_music),
                ee::s07::f_0028cf98__generated,
            );
        }
        fakes::fake_call(c, 0x0028_cf98);
    }

    /// The fake bank-monitor `AddTimer`: logs the call only.
    pub(crate) fn oc1018_add_timer(c: &mut Cpu) {
        if c.bus.read32(OC1018_CTRL) != OC1018_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1018_add_timer),
                ee::s08::f_002adca0__generated,
            );
        }
        fakes::fake_call(c, 0x002a_dca0);
    }

    /// The fake world-trigger monitor pass: logs the call only.
    pub(crate) fn oc1018_monitor_triggers(c: &mut Cpu) {
        if c.bus.read32(OC1018_CTRL) != OC1018_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc1018_monitor_triggers),
                ee::s08::f_002b6950__generated,
            );
        }
        fakes::fake_call(c, 0x002b_6950);
    }

    /// Hooks the fakes into their override slots (once per process).
    pub(crate) fn oc1018_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00285D98),
            oc1018_find_rider,
            fakes::prev_cell(oc1018_find_rider),
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B49E0),
            oc1018_get_song_tag,
            fakes::prev_cell(oc1018_get_song_tag),
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0028CF98),
            oc1018_play_music,
            fakes::prev_cell(oc1018_play_music),
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002ADCA0),
            oc1018_add_timer,
            fakes::prev_cell(oc1018_add_timer),
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002B6950),
            oc1018_monitor_triggers,
            fakes::prev_cell(oc1018_monitor_triggers),
        );
    }

    /// A scrambled machine with this module's fakes armed, `a0` at the object
    /// `DATA + 0x1000` and `pc` at `entry`.
    pub(crate) fn oc1018_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32, entry: u32) -> Rng {
        oc1018_install_fakes();
        let r = fakes::machine(c, seed, salt, calls);
        c.bus.write32(OC1018_CTRL, OC1018_MAGIC);
        c.g[A0][0] = u64::from(DATA + 0x1000);
        c.pc = entry;
        r
    }

    /// The object's debug preset set to a value that covers the wrap cases.
    pub(crate) fn oc1018_preset_setup(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        oc1018_machine(c, seed, salt, 12, entry);
        elf_image(c);
        let preset = match seed % 9 {
            8 => 100,
            m => m,
        };
        c.bus.write32(DATA + 0x1000 + 0x6470, preset);
    }

    /// The channel-volume fixture of `SSXAudioSystem_SetChannelVolume`, the step
    /// constants from the game image, and the entry `pc`.
    pub(crate) fn oc1018_channel_setup(c: &mut Cpu, seed: u32, entry: u32) {
        set_channel_volume_setup(c, seed);
        elf_image(c);
        c.pc = entry;
    }

    /// `SSXAudioSystem_IncChannelVolume`: every channel and the out-of-range ones.
    #[test]
    fn ssx_audio_system_inc_channel_volume_matches() {
        check(
            ee::s07::f_002879b0__generated,
            ssx_audio_system_inc_channel_volume,
            280,
            &|c, seed| oc1018_channel_setup(c, seed, 0x0028_79b0),
        );
    }

    /// `SSXAudioSystem_DecChannelVolume`: every channel and the out-of-range ones.
    #[test]
    fn ssx_audio_system_dec_channel_volume_matches() {
        check(
            ee::s07::f_002879e0__generated,
            ssx_audio_system_dec_channel_volume,
            280,
            &|c, seed| oc1018_channel_setup(c, seed, 0x0028_79e0),
        );
    }

    /// Presets 0..=8 (wrap from 0, then the store and the apply) and 100.
    #[test]
    fn ssx_audio_system_prev_debug_mode_matches() {
        check(
            ee::s07::f_00289398__generated,
            ssx_audio_system_prev_debug_mode,
            240,
            &|c, seed| oc1018_preset_setup(c, seed, 0x9398, 0x0028_9398),
        );
    }

    /// Presets 0..=8 (wrap from 6) and 100.
    #[test]
    fn ssx_audio_system_next_debug_mode_matches() {
        check(
            ee::s07::f_002893c8__generated,
            ssx_audio_system_next_debug_mode,
            240,
            &|c, seed| oc1018_preset_setup(c, seed, 0x93c8, 0x0028_93c8),
        );
    }

    /// The timer arms with the callback pointer from the game image; the bank
    /// monitor is faked (logs its arguments).
    #[test]
    fn ssx_audio_system_schedule_trigger_timer_matches() {
        check(
            ee::s07::f_00289400__generated,
            ssx_audio_system_schedule_trigger_timer,
            200,
            &|c, seed| {
                oc1018_machine(c, seed, 0x9400, 2, 0x0028_9400);
                elf_image(c);
                c.bus.write32(DATA + 0x1000 + 0x118, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000, DATA + 0x3000);
            },
        );
    }

    /// The callback calls the world-trigger monitor (faked) and returns 1.
    #[test]
    fn ssx_audio_system_trigger_timer_callback_matches() {
        check(
            ee::s07::f_00289448__generated,
            ssx_audio_system_trigger_timer_callback,
            200,
            &|c, seed| {
                oc1018_machine(c, seed, 0x9448, 2, 0x0028_9448);
                c.bus.write32(
                    GAME_GP.wrapping_add(0x21e4),
                    DATA + 0x9000 + 0x40 * (seed % 4),
                );
            },
        );
    }

    /// The empty callback only returns to `ra`.
    #[test]
    fn ssx_audio_system_empty_crowd_handler_matches() {
        check(
            ee::s07::f_00289680__generated,
            ssx_audio_system_empty_crowd_handler,
            50,
            &|c, seed| {
                let mut r = Rng(0x9680 ^ u64::from(seed));
                scramble(c, &mut r);
                c.g[RA][0] = sx(0x0030_0000 + 8 * seed);
            },
        );
    }

    /// The paint: rider match or not, a null or set object, every type code
    /// (-1, 0x1f, 0x20, 0x21, other) and the song-tag results. The rider and the
    /// song tag come from the faked callees; the miss fakes return a pointer to
    /// the type-code word.
    pub(crate) fn oc1018_paint_setup(c: &mut Cpu, seed: u32) {
        oc1018_machine(c, seed, 0x99f8, 4, 0x0028_99f8);
        let rider = DATA + 0x2000;
        c.g[A1][0] = u64::from(rider);
        c.g[A2][0] = if seed % 5 == 0 {
            0
        } else {
            u64::from(DATA + 0x4000)
        };
        c.bus.write32(
            OC1018_RIDER,
            if seed % 3 == 0 { DATA + 0x2400 } else { rider },
        );
        c.bus
            .write32(OC1018_TAG, [0x65, 0x66, 0][(seed % 3) as usize]);
        // The object's vtable: delta at +0x20, method at +0x24 (a miss fake).
        c.bus.write32(DATA + 0x4004, DATA + 0x5000);
        c.bus.write16(DATA + 0x5020, ((seed % 2) * 0x10) as u16);
        c.bus
            .write32(DATA + 0x5024, fakes::FAKE + 0x10 * (seed % 3));
        // The self's table at +0x5558, the other path's method.
        c.bus.write32(DATA + 0x1000 + 0x5558, DATA + 0x6000);
        c.bus.write16(DATA + 0x6020, 0x10);
        c.bus.write32(DATA + 0x6024, fakes::FAKE + 0x30);
        // The miss fakes return this word's address; the code word is read back.
        let code_word = DATA + 0x7000;
        c.bus.write32(
            code_word,
            [u32::MAX, 0x1f, 0x20, 0x21, 0x33][(seed % 5) as usize],
        );
        for n in 0..8 {
            c.bus.write32(fakes::LOG_RESULTS + 4 * n, code_word);
        }
    }

    /// The paint over every type code, rider and song-tag outcome.
    #[test]
    fn ssx_audio_system_apply_music_trigger_paint_matches() {
        check(
            ee::s07::f_002899f8__generated,
            ssx_audio_system_apply_music_trigger_paint,
            300,
            &oc1018_paint_setup,
        );
    }

    #[test]
    fn ssx_audio_system_rider_release_matches() {
        check(
            ee::s07::f_00289af8__generated,
            ssx_audio_system_rider_release,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0028_9af8, 2);
                belib_m2(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 7;
                c.bus.write32(DATA + 0x118, DATA + 0x100);
                c.bus.write32(DATA + 0x100, DATA + 0x200);
                let vt = vtable(c, &mut r, &[0x20]);
                c.bus.write32(DATA + 0x200 + 0xab0, vt);
            },
        );
    }

    #[test]
    fn ssx_audio_system_get_output_mode_option_matches() {
        check(
            ee::s07::f_00289dc0__generated,
            ssx_audio_system_get_output_mode_option,
            100,
            &|c, seed| {
                let _ = machine(c, seed, 0x0028_9dc0, 2);
                belib_m2(c);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x62b0, seed % 5);
            },
        );
    }

    #[test]
    fn ssx_audio_system_take_boot_movie_flag_matches() {
        check(
            ee::s07::f_00289de0__generated,
            ssx_audio_system_take_boot_movie_flag,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0028_9de0, 0);
                c.g[A0][0] = DATA as u64;
                let flag = if seed % 3 == 0 { 0 } else { r.next_u32() };
                c.bus.write32(DATA + 0x6c88, flag);
            },
        );
    }
}
