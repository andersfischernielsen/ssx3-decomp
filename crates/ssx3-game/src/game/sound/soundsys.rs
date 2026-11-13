//! `src/sound/soundsys.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::animation::animsequencer::dtor;
pub(crate) use crate::game::be::beintnetwork::ldl;
pub(crate) use crate::game::be::beprepoststate::ea;
pub(crate) use crate::game::dirtysock::tagsunk::lw;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at;
pub(crate) use crate::game::fe::feasyncfile::gp;
pub(crate) use crate::game::fe::feasyncfile::gp_15 as gp_3;
pub(crate) use crate::game::fe::feasyncfile::gp_7 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::low_2;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_3;
pub(crate) use crate::game::fe::feasyncfile::lw_at_2 as lw_at;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::sw_6 as sw_2;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::fe::fepopup::at as at_4;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lhu;
pub(crate) use crate::game::fe::fepopup::sh;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw_2;
pub(crate) use crate::game::fe::festatetrophyroom::r;
pub(crate) use crate::game::fe::festatetrophyroom::Ops;
pub(crate) use crate::game::lib::snd::add;
pub(crate) use crate::game::lib::snd::shl;
pub(crate) use crate::game::main::gamestate::gw;
pub(crate) use crate::game::main::gamestate::lw_at_3;
pub(crate) use crate::game::render::graphicsman::RegMem;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2;
pub(crate) use crate::game::scripter::ssxscriptengine::sw_at;
pub(crate) use crate::game::sound::bankmanager::race_type_in_range;
pub(crate) use crate::game::sound::bankmonitor::leave;
pub(crate) use crate::game::visualfx::boardtrailfx::p;
pub(crate) use crate::game::worldpainter::quadtree::create_with;
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
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0028_7920,
        ssx_audio_system_get_channel_volume,
        "SSXAudioSystem_GetChannelVolume",
    ),
    (
        0x0028_7f00,
        ssx_audio_system_set_fade_mode,
        "SSXAudioSystem_SetFadeMode",
    ),
    (0x0028_fea0, midi_banks_base_dtor, "MIDIBanks_BaseDtor"),
    (0x0029_0b58, audio_eval_curve5, "Audio_EvalCurve5"),
    (
        0x0029_0cc0,
        ssx_audio_system_play_sound,
        "SSXAudioSystem_PlaySound",
    ),
    (
        0x0029_0f58,
        ssx_audio_system_set_player_trick_score,
        "SSXAudioSystem_SetPlayerTrickScore",
    ),
    (
        0x0029_13d8,
        ssx_audio_system_stop_ambience_loop,
        "SSXAudioSystem_StopAmbienceLoop",
    ),
    (
        0x0029_1710,
        ssx_audio_get_surface_sound_type,
        "SSXAudioSystem_GetSurfaceSoundType",
    ),
    (
        0x0029_17b8,
        ssx_audio_system_get_surface_bank,
        "SSXAudioSystem_GetSurfaceBank",
    ),
    (
        0x0029_29d8,
        ssx_audio_system_restart_all_board_loops,
        "SSXAudioSystem_RestartAllBoardLoops",
    ),
    (
        0x0029_2a50,
        ssx_audio_system_start_rider_board_loops,
        "SSXAudioSystem_StartRiderBoardLoops",
    ),
    (
        0x0029_2ae8,
        ssx_audio_system_kill_rider_timers,
        "SSXAudioSystem_KillRiderTimers",
    ),
    (
        0x0029_2b48,
        ssx_audio_system_stop_board_loops,
        "SSXAudioSystem_StopBoardLoops",
    ),
    (
        0x0029_34d0,
        ssx_audio_get_surface_group,
        "SSXAudioSystem_GetSurfaceGroup",
    ),
    (
        0x0029_4880,
        ssx_audio_system_set_rider_air_flag,
        "SSXAudioSystem_SetRiderAirFlag",
    ),
    (
        0x0029_4f48,
        ssx_audio_system_tick_listener,
        "SSXAudioSystem_TickListener",
    ),
    (
        0x0029_7eb8,
        ssx_audio_system_stop_object_loop,
        "SSXAudioSystem_StopObjectLoop",
    ),
    (
        0x0029_80b0,
        ssxaudiosystem_stop_all_object_loops,
        "SSXAudioSystem_StopAllObjectLoops",
    ),
    (
        0x0029_8d00,
        ssxaudiosystem_play_hit_boost_sound,
        "SSXAudioSystem_PlayHitBoostSound",
    ),
    (
        0x0029_9b70,
        ssx_audio_system_play_rail_change_sound,
        "SSXAudioSystem_PlayRailChangeSound",
    ),
    (
        0x0029_b3c0,
        ssx_audio_system_stop_trick_meter_sound,
        "SSXAudioSystem_StopTrickMeterSound",
    ),
    (
        0x0029_b7e0,
        ssx_audio_system_on_bonus_trick,
        "SSXAudioSystem_OnBonusTrick",
    ),
    (
        0x0029_ce28,
        ssx_audio_system_pause_sfx,
        "SSXAudioSystem_PauseSfx",
    ),
    (
        0x0029_ce70,
        ssx_audio_system_resume_sfx,
        "SSXAudioSystem_ResumeSfx",
    ),
    (
        0x0029_d370,
        ssx_audio_system_start_loop_sfx,
        "SSXAudioSystem_StartLoopSfx",
    ),
    (
        0x0029_d678,
        ssx_audio_system_stop_loop_sfx,
        "SSXAudioSystem_StopLoopSfx",
    ),
    (
        0x0029_dbb0,
        ssx_audio_system_on_mission_abort,
        "SSXAudioSystem_OnMissionAbort",
    ),
    (
        0x0029_e578,
        ssx_audio_system_reset_wipeout_sfx,
        "SSXAudioSystem_ResetWipeoutSfx",
    ),
    (
        0x0029_ed18,
        ssx_audio_system_stop_rider_loops,
        "SSXAudioSystem_StopRiderLoops",
    ),
    (
        0x0029_f088,
        ssx_audio_system_get_current_speech_category,
        "SSXAudioSystem_GetCurrentSpeechCategory",
    ),
    (
        0x0029_f0d8,
        ssx_audio_system_is_dj_speech_on,
        "SSXAudioSystem_IsDJSpeechOn",
    ),
    (
        0x0029_f128,
        audio_is_pa_speech_on,
        "SSXAudioSystem_IsPASpeechOn",
    ),
    (
        0x0029_f160,
        audio_is_arcade_speech_on,
        "SSXAudioSystem_IsArcadeSpeechOn",
    ),
    (
        0x0029_f2b0,
        character_to_speech_mask,
        "SSXAudioSystem_CharacterToSpeechMask",
    ),
    (
        0x0029_f5e0,
        ssx_audio_system_unload_grunt_banks,
        "SSXAudioSystem_UnloadGruntBanks",
    ),
    (
        0x002a_1388,
        ssx_audio_system_rider_say_20b9_callback,
        "SSXAudioSystem_RiderSay20B9Callback",
    ),
    (
        0x002a_1778,
        ssx_audio_system_rider_say_20bd,
        "SSXAudioSystem_RiderSay20BD",
    ),
    (
        0x002a_1b58,
        ssx_audio_system_on_script_rider_speech2,
        "SSXAudioSystem_OnScriptRiderSpeech2",
    ),
    (
        0x002a_1b88,
        ssx_audio_system_on_script_rider_speech01,
        "SSXAudioSystem_OnScriptRiderSpeech01",
    ),
    (
        0x002a_1da0,
        character_to_voice_mask,
        "SSXAudioSystem_CharacterToVoiceMask",
    ),
    (0x002a_2018, dj_say_20c8, "SSXAudioSystem_DJSay20C8"),
    (0x002a_20d0, dj_say_20c9, "SSXAudioSystem_DJSay20C9"),
    (
        0x002a_2188,
        ssx_audio_system_dj_say_20ca,
        "SSXAudioSystem_DJSay20CA",
    ),
    (0x002a_2260, dj_say_20e7, "SSXAudioSystem_DJSay20E7"),
    (0x002a_2318, dj_say_20e8, "SSXAudioSystem_DJSay20E8"),
    (
        0x002a_23d0,
        ssx_audio_system_dj_say_20e9,
        "SSXAudioSystem_DJSay20E9",
    ),
    (
        0x002a_24b0,
        ssx_audio_system_dj_say_20cc,
        "SSXAudioSystem_DJSay20CC",
    ),
    (
        0x002a_2568,
        ssx_audio_system_dj_say20cc_2,
        "SSXAudioSystem_DJSay20CC_2",
    ),
    (0x002a_2638, dj_say_20ba, "SSXAudioSystem_DJSay20BA"),
    (
        0x002a_26f0,
        ssx_audio_system_dj_say20e5,
        "SSXAudioSystem_DJSay20E5",
    ),
    (
        0x002a_27d8,
        ssxaudiosystem_djsay20e6,
        "SSXAudioSystem_DJSay20E6",
    ),
    (
        0x002a_2860,
        ssx_audio_system_dj_say_20cf,
        "SSXAudioSystem_DJSay20CF",
    ),
    (
        0x002a_2938,
        ssx_audio_system_dj_say_2102,
        "SSXAudioSystem_DJSay2102",
    ),
    (0x002a_2ad0, dj_say_212b, "SSXAudioSystem_DJSay212B"),
    (
        0x002a_2b88,
        ssx_audio_system_dj_say_212c,
        "SSXAudioSystem_DJSay212C",
    ),
    (
        0x002a_2cf0,
        ssx_audio_system_dj_say212e,
        "SSXAudioSystem_DJSay212E",
    ),
    (
        0x002a_2da0,
        ssx_audio_system_dj_say212f,
        "SSXAudioSystem_DJSay212F",
    ),
    (
        0x002a_3170,
        ssx_audio_system_play_dj_song_intro,
        "SSXAudioSystem_PlayDJSongIntro",
    ),
    (
        0x002a_32b0,
        ssx_audio_system_pa_say_20c1,
        "SSXAudioSystem_PASay20C1",
    ),
    (
        0x002a_3358,
        ssx_audio_system_pa_say_20c2,
        "SSXAudioSystem_PASay20C2",
    ),
    (
        0x002a_3708,
        ssx_audio_system_pa_say20c5,
        "SSXAudioSystem_PASay20C5",
    ),
    (
        0x002a_3b18,
        ssx_audio_system_arcade_say20a7,
        "SSXAudioSystem_ArcadeSay20A7",
    ),
    (
        0x002a_3fd8,
        ssx_audio_system_is_track_id_group5,
        "SSXAudioSystem_IsTrackIdGroup5",
    ),
    (
        0x002a_4040,
        ssx_audio_system_is_event_mode4,
        "SSXAudioSystem_IsEventMode4",
    ),
    (
        0x002a_4078,
        ssx_audio_system_is_track_group0_event,
        "SSXAudioSystem_IsTrackGroup0Event",
    ),
    (
        0x002a_40e8,
        ssx_audio_system_is_track_group1_event,
        "SSXAudioSystem_IsTrackGroup1Event",
    ),
    (
        0x002a_4168,
        ssx_audio_system_is_track_group2_event,
        "SSXAudioSystem_IsTrackGroup2Event",
    ),
    (
        0x002a_41d8,
        ssx_audio_system_is_track_group3_event,
        "SSXAudioSystem_IsTrackGroup3Event",
    ),
    (
        0x002a_4248,
        ssx_audio_system_is_race_type4_or5,
        "SSXAudioSystem_IsRaceType4or5",
    ),
    (
        0x002a_4290,
        ssx_audio_system_is_race_type4,
        "SSXAudioSystem_IsRaceType4",
    ),
    (
        0x002a_4318,
        ssx_audio_system_is_race_type6_to8,
        "SSXAudioSystem_IsRaceType6to8",
    ),
    (
        0x002a_4368,
        ssx_audio_system_is_race_type9_to11,
        "SSXAudioSystem_IsRaceType9to11",
    ),
    (
        0x002a_4550,
        ssx_audio_system_clear_pending_speech,
        "SSXAudioSystem_ClearPendingSpeech",
    ),
    (
        0x002a_4590,
        ssx_audio_system_reset_race_result_state,
        "SSXAudioSystem_ResetRaceResultState",
    ),
    (
        0x002a_45c0,
        ssx_audio_system_record_race_result,
        "SSXAudioSystem_RecordRaceResult",
    ),
    (
        0x002a_4660,
        ssx_audio_system_store_race_result,
        "SSXAudioSystem_StoreRaceResult",
    ),
    (
        0x002a_4718,
        ssx_audio_system_has_track_changed,
        "SSXAudioSystem_HasTrackChanged",
    ),
    (
        0x002a_49e8,
        ssx_audio_system_start_section_speech_timer,
        "SSXAudioSystem_StartSectionSpeechTimer",
    ),
    (
        0x002a_4b68,
        ssx_audio_system_stop_speech_on_reset,
        "SSXAudioSystem_StopSpeechOnReset",
    ),
    (
        0x002a_4ba8,
        crowd_audio_init_sample_list,
        "CrowdAudio_InitSampleList",
    ),
    (
        0x002a_4be0,
        crowd_audio_delete_sample_list,
        "CrowdAudio_DeleteSampleList",
    ),
    (
        0x002a_4ca0,
        crowd_audio_pick_random_sample,
        "CrowdAudio_PickRandomSample",
    ),
    (
        0x002a_4cf8,
        crowd_audio_free_sample_list,
        "CrowdAudio_FreeSampleList",
    ),
    (
        0x002a_4e38,
        crowd_audio_destroy_instance,
        "CrowdAudio_DestroyInstance",
    ),
    (
        0x002a_4e88,
        crowd_audio_update_instance,
        "CrowdAudio_UpdateInstance",
    ),
    (
        0x002a_62f0,
        crowd_audio_instance_remove_source,
        "CrowdAudio_InstanceRemoveSource",
    ),
    (
        0x002a_6430,
        crowd_audio_instance_remove_all_sources,
        "CrowdAudio_InstanceRemoveAllSources",
    ),
    (
        0x002a_67f0,
        crowd_audio_instance_start_anticipate,
        "CrowdAudio_InstanceStartAnticipate",
    ),
    (
        0x002a_6808,
        crowd_audio_instance_stop_anticipate,
        "CrowdAudio_InstanceStopAnticipate",
    ),
    (
        0x002a_6848,
        crowd_audio_instance_stop_sound,
        "CrowdAudio_InstanceStopSound",
    ),
    (0x002a_6f38, crowd_audio_unload, "CrowdAudio_Unload"),
    (
        0x002a_72c0,
        crowd_audio_decode_source_handle,
        "CrowdAudio_DecodeSourceHandle",
    ),
    (
        0x002a_72d8,
        crowd_audio_remove_source,
        "CrowdAudio_RemoveSource",
    ),
    (
        0x002a_7340,
        crowd_audio_set_source_param,
        "CrowdAudio_SetSourceParam",
    ),
    (0x002a_73a8, crowd_audio_play_trick, "CrowdAudio_PlayTrick"),
    (0x002a_73d8, crowd_audio_play_fall, "CrowdAudio_PlayFall"),
    (
        0x002a_7438,
        crowd_audio_stop_anticipate,
        "CrowdAudio_StopAnticipate",
    ),
    (
        0x002a_7468,
        crowd_audio_push_play_params,
        "CrowdAudio_PushPlayParams",
    ),
    (0x002a_78e0, ini_parser_get_char, "IniParser_GetChar"),
    (
        0x002a_79c8,
        ini_parser_skip_comment,
        "IniParser_SkipComment",
    ),
    (
        0x002a_7a20,
        ini_parser_skip_section_headers,
        "IniParser_SkipSectionHeaders",
    ),
    (0x002a_7a90, ini_parser_skip_to_eol, "IniParser_SkipToEol"),
    (
        0x002a_7bf8,
        ini_parser_find_section,
        "IniParser_FindSection",
    ),
    (
        0x002a_7c68,
        ini_parser_find_section_prefix,
        "IniParser_FindSectionPrefix",
    ),
    (
        0x002a_7cf0,
        ini_parser_read_quoted_string,
        "IniParser_ReadQuotedString",
    ),
    (0x002a_8070, ini_parser_get_float, "IniParser_GetFloat"),
    (0x002a_80d8, ini_parser_get_int, "IniParser_GetInt"),
    (
        0x002a_8450,
        midi_banks_push_play_params,
        "MIDIBanks_PushPlayParams",
    ),
    (0x002a_8c20, ps2_sound_manager_ctor, "PS2SoundManager_ctor"),
    (0x002a_8c50, ps2_sound_manager_dtor, "PS2SoundManager_dtor"),
    (
        0x002a_8f50,
        ps2_sound_manager_shutdown,
        "PS2SoundManager_Shutdown",
    ),
    (
        0x002a_8fa0,
        ps2_sound_manager_create,
        "PS2SoundManager_Create",
    ),
    (
        0x002a_9250,
        ps2_sound_manager_set_spu_mem_base,
        "PS2SoundManager_SetSpuMemBase",
    ),
    (0x002a_9288, ps2_sound_manager_v07, "PS2SoundManager_v07"),
    (
        0x002a_92b8,
        ps2_sound_manager_pop_free_spu_block,
        "PS2SoundManager_PopFreeSpuBlock",
    ),
    (
        0x002a_92f8,
        ps2_sound_manager_free_spu_block,
        "PS2SoundManager_FreeSpuBlock",
    ),
    (
        0x0029_5208,
        ssx_audio_system_update_tracked_sounds,
        "SSXAudioSystem_UpdateTrackedSounds",
    ),
    (
        0x0029_7fa0,
        ssx_audio_system_update_object_loops,
        "SSXAudioSystem_UpdateObjectLoops",
    ),
    (
        0x0029_d290,
        ssx_audio_system_update_race_loop_sfx,
        "SSXAudioSystem_UpdateRaceLoopSfx",
    ),
    (
        0x002a_43b8,
        ssx_audio_system_flush_pending_speech,
        "SSXAudioSystem_FlushPendingSpeech",
    ),
    (
        0x0029_47b0,
        ssx_audio_system_update_airborne_riders,
        "SSXAudioSystem_UpdateAirborneRiders",
    ),
    (
        0x0029_b738,
        ssx_audio_update_uber_speech,
        "SSXAudioSystem_UpdateUberSpeech",
    ),
    (
        0x0029_d610,
        ssx_audio_system_update_loop_sfx,
        "SSXAudioSystem_UpdateLoopSfx",
    ),
    (
        0x0029_3b50,
        ssx_audio_system_calc_surface_loop_remote_group0,
        "SSXAudioSystem_CalcSurfaceLoopRemoteGroup0",
    ),
    (
        0x0029_6088,
        ssx_audio_system_play_wall_hit_sound,
        "SSXAudioSystem_PlayWallHitSound",
    ),
    (0x002a_6d18, crowd_audio_update, "CrowdAudio_Update"),
    (
        0x0029_4678,
        ssx_audio_system_update_rider_air_state,
        "SSXAudioSystem_UpdateRiderAirState",
    ),
    (
        0x002a_77c8,
        crowd_audio_get_instance_reaction,
        "CrowdAudio_GetInstanceReaction",
    ),
    (
        0x0029_3740,
        ssx_audio_system_calc_surface_loop_group1,
        "SSXAudioSystem_CalcSurfaceLoopGroup1",
    ),
    (
        0x0029_5028,
        ssx_audio_system_is_tracked_sound_playing,
        "SSXAudioSystem_IsTrackedSoundPlaying",
    ),
    (
        0x0029_5628,
        ssx_audio_system_track_sound,
        "SSXAudioSystem_TrackSound",
    ),
    (
        0x0029_8d90,
        ssx_audio_system_start_boost_sound,
        "SSXAudioSystem_StartBoostSound",
    ),
    (
        0x0029_9368,
        ssx_audio_system_play_boost_empty_sound,
        "SSXAudioSystem_PlayBoostEmptySound",
    ),
    (
        0x0029_9e28,
        ssx_audio_system_on_race_position_change,
        "SSXAudioSystem_OnRacePositionChange",
    ),
    (
        0x0029_6310,
        ssx_audio_system_play_wipeout_impact,
        "SSXAudioSystem_PlayWipeoutImpact",
    ),
    (
        0x0029_74a0,
        ssx_audio_system_play_object_sound,
        "SSXAudioSystem_PlayObjectSound",
    ),
    (
        0x0029_a530,
        ssx_audio_system_play_grab_trick_sound,
        "SSXAudioSystem_PlayGrabTrickSound",
    ),
    (
        0x0029_2508,
        ssx_audio_system_start_board_edge_loop,
        "SSXAudioSystem_StartBoardEdgeLoop",
    ),
    (
        0x0029_6e80,
        ssx_audio_system_start_wipeout_loop2,
        "SSXAudioSystem_StartWipeoutLoop2",
    ),
    (
        0x0029_7950,
        ssx_audio_system_start_object_loop,
        "SSXAudioSystem_StartObjectLoop",
    ),
    (
        0x0029_f0a0,
        ssx_audio_system_is_speech_volume_on,
        "SSXAudioSystem_IsSpeechVolumeOn",
    ),
    (
        0x002a_0560,
        ssx_audio_system_rider_say_207d,
        "SSXAudioSystem_RiderSay207D",
    ),
    (
        0x002a_5cd8,
        crowd_audio_instance_set_source_param,
        "CrowdAudio_InstanceSetSourceParam",
    ),
    (
        0x002a_77a8,
        crowd_audio_pitch_bend_callback,
        "CrowdAudio_PitchBendCallback",
    ),
    (
        0x0028_8940,
        ssx_audio_is_closer_to_second_view,
        "SSXAudioSystem_IsCloserToSecondView",
    ),
    (
        0x0029_10e0,
        ssx_audio_system_start_ambience_loop,
        "SSXAudioSystem_StartAmbienceLoop",
    ),
    (
        0x0029_6868,
        ssx_audio_system_start_wipeout_loop,
        "SSXAudioSystem_StartWipeoutLoop",
    ),
    (
        0x002a_02d8,
        ssx_audio_speech_wipeout_recover,
        "SSXAudioSystem_SpeechWipeoutRecover",
    ),
    (
        0x002a_68b0,
        crowd_audio_update_pitch_bend,
        "CrowdAudio_UpdatePitchBend",
    ),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_GET_SURFACE_SOUND_TYPE;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_PLAY_FE_SOUND;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_BASE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_EDGE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_SURFACE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_TRICK_METER_SOUND;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_WIPEOUT_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_WIPEOUT_LOOP2;

    pub(crate) use crate::game::be::beintmission::callee::AUDIO_GET_CHANNEL_VOLUME;

    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INTERFACE;

    pub(crate) use crate::game::bx::bxstringctor::callee::INI_PARSER_SKIP_WHITESPACE;

    pub(crate) use crate::game::bx::bxstringctor::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT as SPEECH_SYSTEM_SET_EVENT_CONTEXT_2;
    pub(crate) use crate::game::bx::bxstringctor::callee::SSX_AUDIO_GET_LANGUAGE_MASK;
    pub(crate) use crate::game::bx::bxstringctor::callee::SSX_AUDIO_GET_SPEECH_CONTEXT_MASK;
    pub(crate) use crate::game::bx::bxstringctor::callee::SSX_AUDIO_IS_DJ_SPEECH_ON;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::BANK_MONITOR_ADD_TIMER_2;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::BANK_MONITOR_STOP_HANDLE;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::BE_LIBRARY_GET_INTERFACE_2;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::CROWD_INSTANCE_PLAY_FALL;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::CROWD_INSTANCE_PLAY_TRICK;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::CROWD_INSTANCE_STOP_ANTICIPATE;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::GET_FINISH_SPEECH_MASK;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::GET_LANGUAGE_MASK as GET_LANGUAGE_MASK_3;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::GET_RIDER_VOICE_MASK;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::IS_PA_SPEECH_ON;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::IS_RACE_TYPE_4OR5;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::IS_REPLAY_PLAYING;

    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RAND_NEXT_MAIN;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::RIDER_123168;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::SND_BANK_MAN_UNLOAD_BANK;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::SPEECH_QUEUE;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::SPEECH_SET_EVENT_CONTEXT as SPEECH_SET_EVENT_CONTEXT_2;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::SSX_AUDIO_ARCADE_SAY;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::SSX_AUDIO_RIDER_SAY_20B9;

    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE_2 as AUDIO_GET_GAME_MODULE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_IS_TRACK_GROUP0_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_RESET_RACE_RESULT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_TRICK_METER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STORE_RACE_RESULT;

    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_PAUSE_ALL;

    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE;

    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_SOMETHING;

    pub(crate) use crate::game::fe::feasyncfile::callee::GET_LANGUAGE_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_GET_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_EXPECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_GET_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_NEXT_SECTION;

    pub(crate) use crate::game::fe::feasyncfile::callee::INI_SKIP_WHITESPACE;

    pub(crate) use crate::game::fe::feasyncfile::callee::IS_DJ_SPEECH_ON;

    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SAY_20D3;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SAY_20D4;

    pub(crate) use crate::game::fe::feasyncfile::callee::SET_EVENT_CONTEXT;

    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_NEW;

    pub(crate) use crate::game::fe::feasyncfile::callee::TRACK_TO_CATEGORY;

    pub(crate) use crate::game::fe::fepopup::callee::C0028BB10;

    pub(crate) use crate::game::fe::fepopup::callee::C002913D8;

    pub(crate) use crate::game::fe::festaterewards::callee::GET_LANGUAGE_MASK_2;
    pub(crate) use crate::game::fe::festaterewards::callee::IS_ARCADE_SPEECH_ON;
    pub(crate) use crate::game::fe::festaterewards::callee::IS_DJ_SPEECH_ON_2;

    pub(crate) use crate::game::fe::festaterewards::callee::SPEECH_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::fe::festaterewards::callee::SPEECH_STOP_INSTANCE;

    pub(crate) use crate::game::fe::festatetrophyroom::callee::CROWD_AUDIO_INSTANCE_STOP_SOUND;

    pub(crate) use crate::game::fe::festatetrophyroom::callee::MEMSET;

    pub(crate) use crate::game::intersect::worldsphtree::callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP3;

    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_STOP_HANDLE as BANK_MONITOR_STOP_HANDLE_2;

    pub(crate) use crate::game::main::gamestate::callee::SPCH_MAKE_ID;

    pub(crate) use crate::game::main::gamestate::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT;

    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN_PTR;

    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_LANGUAGE_MASK;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_IS_DJ_SPEECH_ON;

    pub(crate) use crate::game::render::particle::callee::INI_PARSER_SKIP_TO_EOL;

    pub(crate) use crate::game::render::particle::callee::SND_SPU_MEM_SET_BASE;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES as CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES_2;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GET_RACE_MAN;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MUSIC_PLAYER_GET_SELECTED_SONG_SED_VALUE;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_SAY_20B9;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SPEECH_SYSTEM_CANCEL_PENDING_INSTANCE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SSX_AUDIO_SYSTEM_DJ_SAY_20E5;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::START_RIDER_BOARD_LOOPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::STOP_BOARD_LOOPS;

    pub(crate) use crate::game::sound::bankmonitor::callee::MUSIC_PLAYER_GET_SONG_TAG;

    pub(crate) use crate::game::sound::bankmonitor::callee::SPEECH_SYSTEM_STOP_INSTANCE;

    pub(crate) use crate::game::world::wscriptcache::callee::CROWD_AUDIO_FREE_SAMPLE_LIST;

    pub(crate) use crate::game::worldpainter::quadtree::callee::PS2_SOUND_MANAGER_CTOR;

    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_ADD_TIMER;

    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_AUDIO_INSTANCE_STOP_SOUND as CROWD_AUDIO_INSTANCE_STOP_SOUND_2;

    /// `Rider_GetMainState(Rider*)`: `*(*(rider + 0x77c) + 0xde0)`.
    pub const RIDER_GET_MAIN_STATE: Callee = Callee {
        addr: 0x0011_fe98,
        f: ee::s00::Rider_GetMainState,
    };

    pub const FIND_INSTANCE: Callee = Callee {
        addr: 0x002a_d550,
        f: ee::s08::BankMonitor_FindInstance,
    };

    pub const STOP_SOUND: Callee = Callee {
        addr: 0x002a_6848,
        f: ee::s08::CrowdAudio_InstanceStopSound,
    };

    pub const PICK_SAMPLE: Callee = Callee {
        addr: 0x002a_4ca0,
        f: ee::s08::CrowdAudio_PickRandomSample,
    };

    pub const PUSH_PLAY_PARAMS: Callee = Callee {
        addr: 0x002a_8450,
        f: ee::s08::MIDIBanks_PushPlayParams,
    };

    pub const MIDI_PLAY: Callee = Callee {
        addr: 0x002a_86b8,
        f: ee::s08::MIDIBanks_Play,
    };

    pub const SET_PITCH: Callee = Callee {
        addr: 0x002a_be78,
        f: ee::s08::BankMonitorInstance_SetPitch,
    };

    pub const PLAY_PUSHED_SOUND: Callee = Callee {
        addr: 0x0029_06b8,
        f: ee::s07::Audio_PlayPushedSound,
    };
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod table {

    pub(crate) use crate::game::fe::feasyncfile::table::AUDIO_GET_RACE_MAN;

    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_DELETE;

    pub(crate) use crate::game::wscript::wscriptman::table::GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::wscript::wscriptman::table::IS_REPLAY_PLAYING;
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod addr {
    pub(crate) use crate::game::bx::bxstringctor::addr::BANK_MONITOR_TIMER_RESET;
    pub(crate) use crate::game::bx::bxstringctor::addr::BE_INTERFACE_LOOKUP;
    pub(crate) use crate::game::bx::bxstringctor::addr::CBE_GET_BE;
    pub(crate) use crate::game::bx::bxstringctor::addr::CBE_GET_INTERFACE;

    pub(crate) use crate::game::bx::bxstringctor::addr::MEM_MAN_FREE;
    pub(crate) use crate::game::bx::bxstringctor::addr::MEM_MAN_FREE_NOSIZE;
}

/// `GetHashValue32` (0x317670) and the UI object lookups, which the generator
/// emitted as cross-shard `call_indirect`s.
pub(crate) mod table_callee {

    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_GET_MAIN_STATE;

    pub(crate) use crate::game::fe::festaterewards::table_callee::SSX_AUDIO_GET_RACE_MAN;
}

/// `u32 SSXAudioSystem_GetSurfaceBank(int index)` (0x002917b8): 1 when
/// `index > 3` (signed 64-bit compare), else 0x10. `a0` is left as 1 and `v1`
/// holds the compare result.
pub fn ssx_audio_system_get_surface_bank(c: &mut Cpu) {
    c.cycles += 6;
    c.g[A0][0] = 1;
    let greater = 3i64 < c.g[A1][0] as i64;
    c.g[V1][0] = greater as u64;
    c.g[V0][0] = 0x10;
    if greater {
        c.g[V0][0] = c.g[A0][0];
    }
    abi::ret(c);
}

/// Offset of the current speech category word.
pub(crate) const SPEECH_CATEGORY: u32 = 0x5738;

/// `int SSXAudioSystem_GetCurrentSpeechCategory(this, bool clear)`
/// (0x0029f088): returns -1 when `clear` (`a1`) is non-zero, else the current
/// category at +0x5738.
pub fn ssx_audio_system_get_current_speech_category(c: &mut Cpu) {
    c.cycles += 2;
    if abi::arg64(c, 1) != 0 {
        c.cycles += 2;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = Addr(abi::arg(c, 0)).off(SPEECH_CATEGORY).lw(c);
    abi::ret(c);
}

/// Offsets of the pending-speech fields cleared by
/// `SSXAudioSystem_ClearPendingSpeech`.
pub(crate) const PENDING_SPEECH_FIELDS: [u32; 14] = [
    0x5788, 0x5740, 0x5744, 0x574c, 0x5750, 0x5754, 0x5758, 0x5760, 0x5768, 0x576c, 0x5770, 0x5774,
    0x577c, 0x5784,
];

/// `void SSXAudioSystem_ClearPendingSpeech(this)` (0x002a4550): zeroes the
/// fourteen pending-speech bookkeeping words.
pub fn ssx_audio_system_clear_pending_speech(c: &mut Cpu) {
    c.cycles += 15;
    let this = Addr(abi::arg(c, 0));
    for off in PENDING_SPEECH_FIELDS {
        this.off(off).sw(c, 0);
    }
    abi::ret(c);
}

/// `float SSXAudioSystem_GetChannelVolume(SSXAudioSystem *self, int channel)`
/// (0x287920): the volume field at `self + channel*16 + 0x62bc`.
pub fn ssx_audio_system_get_channel_volume(c: &mut Cpu) {
    const VOLUME: u32 = 0x62bc;
    c.cycles += 4;
    let off = sx(abi::arg(c, 1) << 4);
    let addr = abi::arg(c, 0).wrapping_add(off as u32);
    c.g[A1][0] = off;
    c.g[A0][0] = sx(addr);
    c.f[0] = Addr(addr).off(VOLUME).read_fpu(c).0;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Crowd audio.

/// `void CrowdAudio_DecodeSourceHandle(uint handle, int *index, int *source)`
/// (0x2a72c0): split the packed handle into its low byte and its arithmetic
/// `>> 8` (sign-extended) high part.
pub fn crowd_audio_decode_source_handle(c: &mut Cpu) {
    c.cycles += 5;
    let handle = c.g[A1][0];
    let index = handle & 0xff;
    let source = sx(((handle as u32) as i32 >> 8) as u32);
    c.g[V0][0] = index;
    c.g[A1][0] = source;
    Addr(abi::arg(c, 2)).sw(c, index);
    Addr(abi::arg(c, 3)).sw(c, source);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// INI parser.

/// `int IniParser_GetChar(IniParser *self)` (0x2a78e0): read the sign-extended
/// byte at the cursor, then advance the cursor by one.
pub fn ini_parser_get_char(c: &mut Cpu) {
    const CURSOR: u32 = 0x8;
    c.cycles += 5;
    let parser = Addr(abi::arg(c, 0));
    let ptr = parser.off(CURSOR).lw(c);
    c.g[V1][0] = ptr;
    c.g[V0][0] = Addr(ptr as u32).lb(c);
    c.g[V1][0] = sx((ptr as u32).wrapping_add(1));
    parser.off(CURSOR).sw(c, c.g[V1][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// PS2 sound manager and its SPU-block free list.

/// `PS2SoundManager_ctor(PS2SoundManager *self)` (0x2a8c20): install the
/// vtable `0x4835f8`, `-1` sentinels, and zero the state words.
pub fn ps2_sound_manager_ctor(c: &mut Cpu) {
    const VTABLE: u32 = 0x0048_35f8; // 0x480000 + 0x35f8
    c.cycles += 12;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = sx(VTABLE);
    c.g[V1][0] = u64::MAX;
    this.off(0x318).sw(c, u64::MAX);
    this.sw(c, sx(VTABLE));
    c.g[V0][0] = c.g[A0][0];
    this.off(0x4).sw(c, 0);
    this.off(0x8).sw(c, 0);
    this.off(0x30c).sw(c, u64::MAX);
    this.off(0x310).sw(c, u64::MAX);
    this.off(0x31c).sw(c, 0);
    abi::ret(c);
}

/// `void *PS2SoundManager_PopFreeSpuBlock(PS2SoundManager *self)` (0x2a92b8):
/// pop the head block off the free list at +0x310 (blocks 24 bytes each) and
/// return its address, or NULL when the list sentinel `-1` is reached.
pub fn ps2_sound_manager_pop_free_spu_block(c: &mut Cpu) {
    const FREE_HEAD: u32 = 0x310;
    const BLOCK_SIZE: u32 = 0x18;
    c.cycles += 4;
    let sm = Addr(abi::arg(c, 0));
    let head = sm.off(FREE_HEAD).lw(c);
    c.g[A2][0] = c.g[A0][0];
    c.g[V1][0] = 0x18;
    if (head as i64) < 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 9;
    let lo = abi::mult(c, head as u32, BLOCK_SIZE);
    let block = sx(sm.0.wrapping_add(lo.wrapping_add(0xc)));
    c.g[V1][0] = block;
    c.g[V0][0] = block;
    let next = Addr(block as u32).off(0x14).lw(c);
    c.g[A0][0] = next;
    sm.off(FREE_HEAD).sw(c, next);
    Addr(block as u32).off(0x14).sw(c, u64::MAX);
    c.g[A1][0] = u64::MAX;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Audio curve evaluation.

/// `float Audio_EvalCurve5(const float *curve, float x)` (0x290b58): find the
/// segment [curve[4k], curve[4k+4]) containing `x` over the five control
/// points and interpolate `value[k] + (x - t0) * (value[k+1] - value[k]) /
/// (t1 - t0)`; the value words live at +0x14 and the segment index is scaled by
/// 4. Falls back to the first value when `x` is below the first key, and to the
/// last or 0 when above. Back-edge event check at 0x290b68.
pub fn audio_eval_curve5(c: &mut Cpu) {
    const HEAD: u32 = 0x0029_0b68;
    c.cycles += 4;
    let curve = abi::arg(c, 0);
    let x = FpuF32(c.f[12]);
    c.g[A1][0] = 1;
    c.g[V1][0] = sx(curve.wrapping_add(4));
    c.g[A3][0] = 0;
    loop {
        c.cycles += 5;
        c.lwc1(1, (c.g[V1][0] as u32).wrapping_sub(4));
        let f1 = FpuF32(c.f[1]);
        let le = f1.le(x);
        abi::set_fcc(c, le);
        c.g[V0][0] = c.g[A3][0];
        if !le {
            // 'b_290b90 tail
        } else {
            c.cycles += 5;
            c.lwc1(3, c.g[V1][0] as u32);
            let f3 = FpuF32(c.f[3]);
            let lt = x.lt(f3);
            abi::set_fcc(c, lt);
            c.g[A2][0] = sx((c.g[A1][0] as u32) << 2);
            if lt {
                c.cycles += 12;
                let base = curve.wrapping_add(0x14);
                c.g[V0][0] = sx(base);
                let f0 = x - f1;
                c.f[0] = f0.0;
                let va = base.wrapping_add(c.g[A3][0] as u32);
                c.g[V1][0] = sx(va);
                let f3b = f3 - f1;
                c.f[3] = f3b.0;
                let vb = base.wrapping_add(c.g[A2][0] as u32);
                c.g[V0][0] = sx(vb);
                c.lwc1(2, va);
                c.lwc1(1, vb);
                let f1b = FpuF32(c.f[1]) - FpuF32(c.f[2]);
                c.f[1] = f1b.0;
                let f0b = f0 * f1b;
                c.f[0] = f0b.0;
                let q = f0b.div(f3b);
                c.f[0] = q.0;
                let r = FpuF32(c.f[2]) + q;
                c.f[0] = r.0;
                abi::ret(c);
                return;
            }
        }
        c.cycles += 5;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
        let k = c.g[A1][0];
        c.g[V0][0] = ((k as i64) < 5) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
        if more {
            if event_due(c) {
                deliver_event_here(c, HEAD);
            }
            continue;
        }
        break;
    }
    // Past the last segment: compare `x` against the first key.
    c.cycles += 5;
    c.lwc1(0, curve);
    let first = FpuF32(c.f[0]);
    let x_lt_first = x.lt(first);
    abi::set_fcc(c, x_lt_first);
    if !x_lt_first {
        c.lwc1(0, curve.wrapping_add(0x10));
        c.cycles += 3;
        let last = FpuF32(c.f[0]);
        let last_le_x = last.le(x);
        abi::set_fcc(c, last_le_x);
        if last_le_x {
            c.cycles += 2;
            c.lwc1(0, curve.wrapping_add(0x24));
            abi::ret(c);
        } else {
            c.cycles += 3;
            c.mtc1(0, 0); // f0 = 0
            abi::ret(c);
        }
    } else {
        c.cycles += 2;
        c.lwc1(0, curve.wrapping_add(0x14));
        abi::ret(c);
    }
}

// ---------------------------------------------------------------------------
// Audio scoring.

/// `void SSXAudioSystem_SetPlayerTrickScore(SSXAudioSystem* app, int rider,
/// float score)` (0x290f58): store the trick score on the rider's listener
/// (rider 0 at `+0x6074`, rider 1 at `+0x6078`) when it is the active player.
pub fn ssx_audio_system_set_player_trick_score(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        let app = abi::arg(c, 0);
        let rider = abi::arg(c, 1);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(app);
        c.g[S1][0] = sx(rider);
        frame.save_fpr(c, 20, 0x30);
        frame.save_ra(c, 0);
        c.mov_s(20, 12);
        c.g[A1][0] = 0;
        call_ptr(c, 0x0028_5d98, 0x0029_0f80)?;
        if c.g[V0][0] == c.g[S1][0] {
            c.cycles += 2;
            c.swc1(20, p(app, 0x6074));
            audio_score_tail(c, frame);
            return Ok(());
        }
        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0028_8ce8, 0x0029_0f98)?;
        c.g[V1][0] = 2;
        if c.g[V0][0] != c.g[V1][0] {
            frame.restore(c, S0, 0x20);
            audio_score_exit(c, frame);
            return Ok(());
        }
        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0028_5d98, 0x0029_0fb0)?;
        if c.g[V0][0] == c.g[S1][0] {
            c.swc1(20, p(app, 0x6078));
        }
        audio_score_tail(c, frame);
        Ok(())
    })
}

/// 0x290fb8: the `+1` cycle tail and `s0` pop.
pub(crate) fn audio_score_tail(c: &mut Cpu, frame: Frame) {
    c.cycles += 1;
    frame.restore(c, S0, 0x20);
    audio_score_exit(c, frame);
}

/// 0x290fbc: the epilogue (s1, ra and f20 back, `sp += 0x40`).
pub(crate) fn audio_score_exit(c: &mut Cpu, frame: Frame) {
    c.cycles += 5;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 20, 0x30);
    frame.close(c);
}

/// `bool SSXAudioSystem_IsTrackIdGroup5(int self, int id)` (0x2a3fd8): ids
/// 0x11..0x15.
pub fn ssx_audio_system_is_track_id_group5(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = sx(abi::arg(c, 1).wrapping_sub(0x11));
    c.g[V0][0] = (c.g[V0][0] < 0x5) as u64;
    abi::ret(c);
}

/// `void SSXAudioSystem_ResetWipeoutSfx(SSXAudioSystem* a, Wipeout* w)`
/// (0x29e578): clears the wipe-out active word of the voice indexed by
/// `w->+0x870` (4-byte entries at +0x6080).
pub fn ssx_audio_system_reset_wipeout_sfx(c: &mut Cpu) {
    c.cycles += 5;
    let selfp = abi::arg(c, 0);
    let src = abi::arg(c, 1);
    c.g[V0][0] = c.lw(src.wrapping_add(0x870));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[A0][0] = sx(selfp.wrapping_add(c.g[V0][0] as u32));
    let dst = (c.g[A0][0] as u32).wrapping_add(0x6080);
    c.sw(dst, 0);
    abi::ret(c);
}

/// `void CrowdAudio_InstanceStartAnticipate(CrowdAudio* a)` (0x2a67f0): sets
/// the "anticipating" flag at +0x180 the first time it is called and returns
/// 1.
pub fn crowd_audio_instance_start_anticipate(c: &mut Cpu) {
    c.cycles += 3;
    let selfp = abi::arg(c, 0);
    c.g[V0][0] = c.lw(selfp.wrapping_add(0x180));
    let already = c.g[V0][0] != 0;
    c.g[V0][0] = 1;
    if !already {
        c.cycles += 1;
        c.sw(selfp.wrapping_add(0x180), c.g[V0][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void SSXAudioSystem_SetRiderAirFlag(this, int i, int on)` (0x00294880):
/// store `on` in the per-rider air flag at `+0x59e8 + i*4`, for `i < 6`.
pub fn ssx_audio_system_set_rider_air_flag(c: &mut Cpu) {
    c.cycles += 3;
    let in_range = (c.g[A1][0] as i64) < 6;
    c.g[V0][0] = in_range as u64;
    c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
    if in_range {
        c.cycles += 2;
        let at = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = at;
        c.sw((at as u32).wrapping_add(0x59e8), c.g[A2][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void SSXAudioSystem_TickListener(SSXAudioSystem *self)` (0x294f48): call
/// the listener object's virtual tick (`vtable+0xc`) with no arguments.
pub fn ssx_audio_system_tick_listener(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0;
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0xc).lw(c);
        c.g[A3][0] = Addr(c.g[V1][0] as u32).off(8).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0xc).lw(c);
        let method = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A3][0] as u32));
        call_ptr(c, method, 0x0029_4f6c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SSXAudioSystem_OnScriptRiderSpeech2(SSXAudioSystem *self, int msg,
/// void *arg)` (0x2a1b58): a rider speech request (`msg == 2`) is forwarded
/// and acknowledged; anything else is ignored.
pub fn ssx_audio_system_on_script_rider_speech2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 2;
        let speech = c.g[A1][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        if speech {
            c.cycles += 3;
            c.g[A1][0] = c.g[A2][0];
            callee::SSX_AUDIO_RIDER_SAY_20B9.call(c, 0x002a_1b78)?;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_ResetRaceResultState(SSXAudioSystem *self, int flag)`
/// (0x2a4590): clear the race-result announcement state; the two "no result
/// yet" cells (`+0x5800`, `+0x5810`, and `+0x580c` when `flag`) become -1 and
/// the queue position (`+0x5814`) 0x17.
pub fn ssx_audio_system_reset_race_result_state(c: &mut Cpu) {
    c.cycles += 8;
    let this = abi::arg(c, 0);
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    let base = Addr(this);
    base.off(0x57f8).sw(c, 0);
    base.off(0x57fc).sw(c, 0);
    base.off(0x5804).sw(c, 0);
    base.off(0x5808).sw(c, 0);
    base.off(0x5800).sw(c, c.g[V0][0]);
    let flag = c.g[A1][0] == 0;
    base.off(0x5810).sw(c, c.g[V0][0]);
    if !flag {
        c.cycles += 1;
        base.off(0x580c).sw(c, c.g[V0][0]);
    }
    c.cycles += 3;
    c.g[V0][0] = 0x17;
    base.off(0x5814).sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// The shared body of the three crowd-voice dispatchers: pick the voice at
/// `index * 0x190` in the array at `self+0x18` and forward to `play(voice,
/// arg)`; do nothing while no crowd audio object is loaded.
pub(crate) fn crowd_voice(c: &mut Cpu, index_reg: usize, play: Callee, ret: u32) -> Flow {
    /// The pool of 0x190-byte crowd voices.
    const VOICES: u32 = 0x18;
    /// Size of one `sSndVoice190`.
    const VOICE_SIZE: u32 = 0x190;

    c.cycles += 5;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(VOICES).lw(c);
    let no_voices = c.g[V0][0] == 0;
    frame.restore_ra(c, 0);
    if !no_voices {
        c.cycles += 5;
        c.g[A0][0] = VOICE_SIZE as u64;
        c.mult(0, V1, index_reg, A0);
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        play.call(c, ret)?;
        frame.restore_ra(c, 0);
    }
    c.cycles += 2;
    frame.close(c);
    Ok(())
}

/// `void CrowdAudio_PlayTrick(CrowdAudio *self, int arg, int voice)`
/// (0x2a73a8).
pub fn crowd_audio_play_trick(c: &mut Cpu) {
    guest(c, |c| {
        crowd_voice(c, A2, callee::CROWD_INSTANCE_PLAY_TRICK, 0x002a_73cc)
    })
}

/// `void CrowdAudio_PlayFall(CrowdAudio *self, int arg, int voice)`
/// (0x2a73d8).
pub fn crowd_audio_play_fall(c: &mut Cpu) {
    guest(c, |c| {
        crowd_voice(c, A2, callee::CROWD_INSTANCE_PLAY_FALL, 0x002a_73fc)
    })
}

/// `void CrowdAudio_StopAnticipate(CrowdAudio *self, int voice)` (0x2a7438):
/// the voice index is in `a1` here, not `a2`.
pub fn crowd_audio_stop_anticipate(c: &mut Cpu) {
    guest(c, |c| {
        crowd_voice(c, A1, callee::CROWD_INSTANCE_STOP_ANTICIPATE, 0x002a_745c)
    })
}

// ---------------------------------------------------------------------------
// Sound.

/// `PS2SoundManager_v07` (0x2a9288): set the sound-system SPU memory base to
/// the manager's cached value (at `+0x30c`) with `-1` ("use the default"),
/// then clear the pending flag at `+0x31c`.
pub fn ps2_sound_manager_v07(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = u64::MAX;
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x30c).lw(c);
        call_ptr(c, 0x003b_5b20, 0x002a_92a4)?;
        let this = c.g[S0][0] as u32;
        Addr(this).off(0x31c).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool SSXAudioSystem_IsPASpeechOn(SSXAudioSystem* self)` (0x29f128).
pub fn audio_is_pa_speech_on(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = 3;
        call_ptr(c, callee::AUDIO_GET_CHANNEL_VOLUME, 0x0029_f138)?;
        c.mtc1(0, 1);
        c.c_eq_s(0, 1);
        let silent = c.fcc();
        c.g[V0][0] = 0;
        if !silent {
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `bool SSXAudioSystem_IsArcadeSpeechOn(SSXAudioSystem* self)` (0x29f160).
pub fn audio_is_arcade_speech_on(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xa;
        call_ptr(c, callee::AUDIO_GET_CHANNEL_VOLUME, 0x0029_f170)?;
        c.mtc1(0, 1);
        c.c_eq_s(0, 1);
        let silent = c.fcc();
        c.g[V0][0] = 0;
        if !silent {
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void CrowdAudio_InitSampleList(CrowdAudio* self)` (0x2a4ba8): clear the
/// sample count and the five sample slots.
pub fn crowd_audio_init_sample_list(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = c.g[A0][0];
    c.g[A1][0] = 4;
    Addr(c.g[V0][0] as u32).sw(c, 0);
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
    loop {
        c.cycles += 7;
        Addr(c.g[V1][0] as u32).sw(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        let more = (c.g[A1][0] as i64) >= 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(4));
        if !more {
            c.cycles += 2;
            crate::abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x002a_4bb8);
        }
    }
}

/// `void* PS2SoundManager_Create()` (0x2a8fa0): allocate the 0x320-byte sound
/// manager and construct it.
pub fn ps2_sound_manager_create(c: &mut Cpu) {
    guest(c, |c| {
        create_with(c, 0x320, 0x0048_3168, 0, 0x002a_8fc0, |c, ret| {
            callee::PS2_SOUND_MANAGER_CTOR.call(c, ret)
        })
    })
}

/// `void SSXAudioSystem_OnBonusTrick(SSXAudioSystem* audio, Rider* r)`
/// (0x29b7e0): say the bonus-trick line when the rider's trick display is
/// active.
pub fn ssx_audio_system_on_bonus_trick(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(abi::arg(c, 1)).off(0x874).lw(c);
        let inactive = c.g[V0][0] == 0;
        c.g[V1][0] = 0;
        if !inactive {
            c.cycles += 2;
            c.g[V0][0] = Addr(abi::arg(c, 1)).off(0x87c).lw(c);
            c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
        }
        c.cycles += 2;
        let quiet = c.g[V1][0] == 0;
        frame.restore_ra(c, 0);
        if !quiet {
            c.cycles += 3;
            c.g[A2][0] = 8;
            callee::SSX_AUDIO_ARCADE_SAY.call(c, 0x0029_b80c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `int SSXAudioSystem_IsEventMode4(void)` (0x2a4040): whether the BE event
/// mode byte is 4.
pub fn ssx_audio_system_is_event_mode4(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::BE_LIBRARY_GET_INSTANCE_2.call(c, 0x002a_4050)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        callee::BE_LIBRARY_GET_INTERFACE_2.call(c, 0x002a_405c)?;
        c.g[V1][0] = 0x0053_0000;
        frame.restore_ra(c, 0);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x5c10)).lb(c);
        c.g[V0][0] ^= 0x4;
        c.g[V0][0] = (c.g[V0][0] < 1u64) as u64;
        frame.close(c);
        Ok(())
    })
}

/// `int SSXAudioSystem_IsRaceType4(void)` (0x2a4290): whether the BE race type
/// byte is 4.
pub fn ssx_audio_system_is_race_type4(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::BE_LIBRARY_GET_INSTANCE_2.call(c, 0x002a_42a0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        callee::BE_LIBRARY_GET_INTERFACE_2.call(c, 0x002a_42ac)?;
        c.g[V1][0] = 0x0053_0000;
        frame.restore_ra(c, 0);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x5c12)).lb(c);
        c.g[V0][0] ^= 0x4;
        c.g[V0][0] = (c.g[V0][0] < 1u64) as u64;
        frame.close(c);
        Ok(())
    })
}

/// `void CrowdAudio_InstanceStopAnticipate(CrowdAudioInstance* self)`
/// (0x2A6808): stop the instance if it is in the "sound started" state (2),
/// then mark the slot idle.
pub fn crowd_audio_instance_stop_anticipate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 2;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let state = c.lw_r_2(V0, r(c, S0).wrapping_add(0x180));
        if state == c.g[V1][0] {
            c.cycles += 3;
            callee::CROWD_AUDIO_INSTANCE_STOP_SOUND.call(c, 0x002a_6830)?;
        }
        Addr(r(c, S0).wrapping_add(0x180)).sw(c, 0);
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_UnloadGruntBanks(SSXAudioSystem* self)`
/// (0x29f5e0): unload sound banks 14 and 15.
pub fn ssx_audio_system_unload_grunt_banks(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0xe;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S0)).off(0x118).lw(c);
        let bank_man = r32(c, V0);
        c.g[A0][0] = Addr(bank_man).lw(c);
        callee::SND_BANK_MAN_UNLOAD_BANK.call(c, 0x0029_f600)?;
        c.g[V0][0] = Addr(r32(c, S0)).off(0x118).lw(c);
        c.g[A1][0] = 0xf;
        c.g[A0][0] = Addr(r32(c, V0)).lw(c);
        callee::SND_BANK_MAN_UNLOAD_BANK.call(c, 0x0029_f610)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_StopSpeechOnReset(SSXAudioSystem* self)`
/// (0x2a4b68): when the music player at `+0x118` is on song tag 0x191, stop
/// the speech instance at `+0x5560`.
pub fn ssx_audio_system_stop_speech_on_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(p(r32(c, S0), 0x118));
        callee::MUSIC_PLAYER_GET_SONG_TAG.call(c, 0x002a_4b80)?;
        c.g[V1][0] = 0x191;
        if c.g[V0][0] != c.g[V1][0] {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 4;
            c.g[A0][0] = sx(p(r32(c, S0), 0x5560));
            c.g[A1][0] = 0;
            callee::SPEECH_SYSTEM_STOP_INSTANCE.call(c, 0x002a_4b98)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        leave(c, 0x20);
        Ok(())
    })
}

/// `void CrowdAudio_DeleteSampleList(CrowdAudio* self, int flags)` (0x2a4be0):
/// free the crowd sample list and, when `flags & 1`, the object.
pub fn crowd_audio_delete_sample_list(c: &mut Cpu) {
    guest(c, |c| {
        dtor(
            c,
            callee::CROWD_AUDIO_FREE_SAMPLE_LIST,
            0x002a_4bfc,
            0x002a_4c10,
        )
    })
}

/// `void SSXAudioSystem_PauseSfx(self)` (0x29ce28): the first time it is
/// called, flag the audio system paused (`+0x5fb0`, and the bank monitor's
/// `+0x26c`) and pause every bank monitor.
pub fn ssx_audio_system_pause_sfx(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let self_ = c.g[A1][0] as u32;
        c.g[V0][0] = Addr(self_.wrapping_add(0x5fb0)).lw(c);
        let already = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if !already {
            c.cycles += 10;
            c.g[A0][0] = 1;
            c.g[V1][0] = Addr(self_.wrapping_add(0x118)).lw(c);
            Addr(self_.wrapping_add(0x5fb0)).sw(c, c.g[A0][0]);
            let monitor = c.g[V1][0] as u32;
            c.g[V0][0] = Addr(monitor).lw(c);
            Addr((c.g[V0][0] as u32).wrapping_add(0x26c)).sw(c, c.g[A0][0]);
            c.g[V1][0] = Addr(self_.wrapping_add(0x118)).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            let bank = (c.g[A0][0] as u32).wrapping_add(0x1d8);
            c.g[A0][0] = sx(bank);
            callee::BANK_MONITOR_PAUSE_ALL.call(c, 0x0029_ce64)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `int SSXAudioSystem_OnScriptRiderSpeech01(self, type, a, b)` (0x2a1b88):
/// dispatch a scripted rider speech request: type 0 to `..._RiderSay20D3`,
/// type 1 to `..._RiderSay20D4`, anything else is not handled.
pub fn ssx_audio_system_on_script_rider_speech01(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        let not_zero = c.g[A1][0] != 0;
        frame.save_ra(c, 0);
        if !not_zero {
            c.cycles += 5;
            c.g[A1][0] = c.g[A2][0];
            c.g[A2][0] = c.g[A3][0];
            callee::RIDER_SAY_20D3.call(c, 0x002a_1ba0)?;
            c.g[V0][0] = 1;
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = 1;
        let is_one = c.g[A1][0] == c.g[V0][0];
        c.g[V0][0] = 0;
        if !is_one {
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 4;
        c.g[A1][0] = c.g[A2][0];
        c.g[A2][0] = c.g[A3][0];
        callee::RIDER_SAY_20D4.call(c, 0x002a_1bc0)?;
        c.g[V0][0] = 1;
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SSXAudioSystem_IsRaceType4or5(self)` (0x2a4248): true when the current
/// race type (`D_00535C12[0]`, a byte in the game's mode block) is 4 or 5.
/// Both calls are the BE library interface accesses the original makes first.
pub fn ssx_audio_system_is_race_type4_or5(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0014_dc80, 0x002a_4258)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x002a_4264)?;
        c.g[V1][0] = 0x53_0000;
        c.g[V0][0] = 4;
        let mode = (c.g[V1][0] as u32).wrapping_add(0x5c12);
        c.g[V1][0] = Addr(mode).lb(c);
        let type4 = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 5;
        if !type4 {
            c.cycles += 2;
            let type5 = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0;
            if !type5 {
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
        c.cycles += 1;
        c.g[V0][0] = 1;
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Audio.

/// `void SSXAudioSystem_PlayDJSongIntro(SSXAudioSystem* self)` (0x2a3170):
/// cut the DJ speech, mark the intro as pending and remember the selected
/// song's SED value.
pub fn ssx_audio_system_play_dj_song_intro(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 0;
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        callee::SSX_AUDIO_SYSTEM_DJ_SAY_20E5.call(c, 0x002a_318c)?;
        c.g[A0][0] = sx(this.wrapping_add(0x5560));
        c.g[A1][0] = 0;
        callee::SPEECH_SYSTEM_CANCEL_PENDING_INSTANCE.call(c, 0x002a_3198)?;
        c.g[V0][0] = 1;
        c.g[A0][0] = sx(this.wrapping_add(0x118));
        Addr(this).off(0x5774).sw(c, c.g[V0][0]);
        callee::MUSIC_PLAYER_GET_SELECTED_SONG_SED_VALUE.call(c, 0x002a_31a8)?;
        Addr(this).off(0x5778).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool SSXAudioSystem_IsRaceType6to8(void)` (0x2a4318).
pub fn ssx_audio_system_is_race_type6_to8(c: &mut Cpu) {
    guest(c, |c| {
        race_type_in_range(c, 6, 9, [0x002a_4328, 0x002a_4334, 0x002a_4358])
    })
}

/// `bool SSXAudioSystem_IsRaceType9to11(void)` (0x2a4368).
pub fn ssx_audio_system_is_race_type9_to11(c: &mut Cpu) {
    guest(c, |c| {
        race_type_in_range(c, 9, 12, [0x002a_4378, 0x002a_4384, 0x002a_43a8])
    })
}

/// `void PS2SoundManager_Shutdown(PS2SoundManager* self)` (0x2a8f50): stop
/// the SPU streams, shut the driver down and free the owned buffer.
pub fn ps2_sound_manager_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0xffff_ffff_ffff_ffff;
        frame.save(c, S0, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0xffff_ffff_ffff_ffff;
        call_ptr(c, 0x003b_5b20, 0x002a_8f6c)?;
        call_ptr(c, 0x003b_5320, 0x002a_8f74)?;
        c.g[A0][0] = Addr(this).off(4).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x002a_8f88)?;
        }
        Addr(this).off(4).sw(c, 0);
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool SSXAudioSystem_IsDJSpeechOn(void* self)` (0x29f0d8): true when the
/// replay is not playing and the DJ speech channel's volume is non-zero.
pub fn ssx_audio_system_is_dj_speech_on(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, table::IS_REPLAY_PLAYING, 0x0029_f0ec)?;
        let mut on = 0u64;
        if c.g[V0][0] == 0 {
            c.cycles += 8;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x2;
            call_ptr(c, table::GET_CHANNEL_VOLUME, 0x0029_f100)?;
            c.mtc1(0, 1); // 0.0
            c.c_eq_s(0, 1);
            if !c.fcc() {
                c.cycles += 1;
                on = 0x1;
            }
        }
        c.g[V0][0] = on;
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_StartSectionSpeechTimer(void* self)` (0x2a49e8):
/// mark the section speech active and arm the bank monitor's 30 s timer.
pub fn ssx_audio_system_start_section_speech_timer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x10);
        let this = r32(c, A0);
        c.g[V0][0] = 0x1;
        frame.save_ra(c, 0);
        c.g[V1][0] = 0x4a_0000; // the timer constant's page
        Addr(p(this, 0x5818)).sw(c, c.g[V0][0]);
        c.g[A1][0] = 0x7530; // 30000 ms
        c.g[V0][0] = Addr(p(this, 0x118)).lw(c);
        c.g[A3][0] = 0;
        // The 8-byte timer argument (`D_004A36F0`): an unaligned `ld` pair.
        c.ldl(A2, 0x004a_36f7);
        c.ldr(A2, 0x004a_36f0);
        c.g[T0][0] = 0;
        c.g[A0][0] = Addr(r32(c, V0)).lw(c);
        c.g[T1][0] = 0;
        c.g[T2][0] = 0;
        c.g[T3][0] = 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x1d8));
        callee::BANK_MONITOR_ADD_TIMER.call(c, 0x002a_4a2c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CrowdAudio_DestroyInstance(CrowdAudio* self, int flags)`
/// (0x2a4e38): stop the instance's sound, drop its sources, and delete it
/// when `flags & 1`.
pub fn crowd_audio_destroy_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        callee::CROWD_AUDIO_INSTANCE_STOP_SOUND_2.call(c, 0x002a_4e54)?;
        c.mtc1(0, 12); // fade out over 0.0
        c.g[A0][0] = c.g[S1][0];
        callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES.call(c, 0x002a_4e60)?;
        c.g[S0][0] &= 1;
        let owned = c.g[S0][0] != 0;
        frame.restore(c, S0, 0x20);
        if owned {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x002a_4e74)?;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_StopAmbienceLoop(self)` (0x2913d8): stop the ambience
/// handle through the sound-system interface and clear the loop fields.
pub fn ssx_audio_system_stop_ambience_loop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x4a_0000;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.ldl(5, p(r32(c, V0), 0x3687));
        c.ldr(5, p(r32(c, V0), 0x3680));
        c.g[V1][0] = Addr(r32(c, S0)).off(0x118).lw(c);
        c.g[A0][0] = c.lw(r32(c, V1));
        c.g[V0][0] = Addr(r32(c, A0)).off(0xab0).lw(c);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x1d8));
        c.g[V1][0] = Addr(r32(c, V0)).off(0x18).lh(c);
        c.g[A2][0] = Addr(r32(c, V0)).off(0x1c).lw(c);
        let target = r32(c, A2);
        c.g[RA][0] = sx(0x0029_1414);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
        call_ptr(c, target, 0x0029_1414)?;
        Addr(p(r32(c, S0), 0x5834)).sw(c, 0);
        Addr(p(r32(c, S0), 0x5830)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_StopLoopSfx(self)` (0x29d678): stop the current loop
/// SFX handle through the bank monitor and clear the slot.
pub fn ssx_audio_system_stop_loop_sfx(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S0)).off(0x5fc4).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = Addr(r32(c, S0)).off(0x5fc8).lw(c);
            let negative = (c.g[A1][0] as i64) < 0;
            c.g[A2][0] = 1;
            if !negative {
                c.cycles += 6;
                c.g[V0][0] = Addr(r32(c, S0)).off(0x118).lw(c);
                c.g[A0][0] = c.lw(r32(c, V0));
                c.g[RA][0] = sx(0x0029_d6b0);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x1d8));
                callee::BANK_MONITOR_STOP_HANDLE.call(c, 0x0029_d6b0)?;
                c.g[V0][0] = u64::MAX;
                Addr(p(r32(c, S0), 0x5fc8)).sw(c, c.g[V0][0]);
            }
            c.cycles += 2;
            Addr(p(r32(c, S0), 0x5fc4)).sw(c, 0);
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SSXAudioSystem_HasTrackChanged(self)` (0x2a4718): whether the current
/// track id differs from the system's track (and is not the "none" id 0x17).
pub fn ssx_audio_system_has_track_changed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[RA][0] = sx(0x002a_472c);
        call_ptr(c, callee::BE_LIBRARY_GET_INSTANCE, 0x002a_472c)?;
        c.g[A1][0] = 0;
        c.g[RA][0] = sx(0x002a_4738);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x002a_4738)?;
        c.g[RA][0] = sx(0x002a_4740);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_4bc0, 0x002a_4740)?;
        c.g[A0][0] = Addr(r32(c, S0)).off(0x5814).lw(c);
        c.g[V1][0] = c.lw(r32(c, V0));
        let same = c.g[V1][0] == c.g[A0][0];
        c.g[V0][0] = 0x17;
        if same {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 2;
            if c.g[A0][0] != c.g[V0][0] {
                c.g[V0][0] = 1;
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* CrowdAudio_PickRandomSample(list)` (0x2a4ca0): a random entry of
/// the sample list, or 0 when the list is empty.
pub fn crowd_audio_pick_random_sample(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw(r32(c, S0));
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 6;
            c.g[RA][0] = sx(0x002a_4cc4);
            call_ptr(c, callee::RAND_NEXT_MAIN, 0x002a_4cc4)?;
            c.g[V1][0] = c.lw(r32(c, S0));
            c.divu(0, V0, V1);
            c.cycles += 4;
            c.g[A0][0] = c.hi[0];
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
            c.g[V0][0] = Addr(r32(c, A0)).off(4).lw(c);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Sound (s08).

/// `int IniParser_SkipComment(IniParser* self)` (0x2a79c8): 1 at end of input,
/// otherwise skip a `#`/`;` comment line and report whether text followed.
pub fn ini_parser_skip_comment(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, A0)).off(0x8).lw(c);
        c.g[V1][0] = Addr(r32(c, V0)).lb(c);
        let at_end = c.g[V1][0] == 0;
        c.g[V0][0] = b'#' as u64;
        if at_end {
            c.cycles += 1;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 2;
            let hash = c.g[V1][0] == c.g[V0][0];
            if !hash {
                c.cycles += 3;
                c.g[V0][0] = b';' as u64;
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[V0][0] = 0;
                    c.cycles += 3;
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
            }
            c.cycles += 5;
            callee::INI_PARSER_SKIP_TO_EOL.call(c, 0x002a_79fc)?;
            c.g[V1][0] = c.g[V0][0];
            let skipped = c.g[V1][0] != 0;
            c.g[V0][0] = 1;
            if !skipped {
                c.cycles += 2;
                c.g[V0][0] = 0;
            }
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void PS2SoundManager_FreeSpuBlock(PS2SoundManager* self, SpuBlock* e)`
/// (0x2a92f8): push a block back onto the free list and, before the SPU base
/// has been set, hand the block's voice id to the driver.
pub fn ps2_sound_manager_free_spu_block(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = u64::MAX;
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, A1)).off(0x8).lw(c);
        Addr(r32(c, A0)).off(0x30c).sw(c, c.g[V0][0]);
        Addr(r32(c, A1)).off(0x8).sw(c, c.g[V1][0]);
        Addr(r32(c, A1)).off(0x4).sw(c, c.g[V1][0]);
        Addr(r32(c, A1)).off(0xc).sw(c, 0);
        Addr(r32(c, A1)).off(0x10).sw(c, 0);
        c.g[V1][0] = Addr(r32(c, A1)).lw(c);
        c.g[V0][0] = Addr(r32(c, A0)).off(0x310).lw(c);
        Addr(r32(c, A1)).off(0x14).sw(c, c.g[V0][0]);
        Addr(r32(c, A0)).off(0x310).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(r32(c, A0)).off(0x31c).lw(c);
        let spu_base_set = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if !spu_base_set {
            c.cycles += 4;
            c.g[A1][0] = Addr(r32(c, A0)).off(0x30c).lw(c);
            c.g[A0][0] = u64::MAX;
            call_ptr(c, callee::SND_SPU_MEM_SET_BASE, 0x002a_9344)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The pool base setter `func_003B5B20` (SND_Lock; SNDI_SpuMemSetBase;
/// SND_Unlock) that the original calls indirectly. Kept a table call so a
/// replaced callee is reached too.
pub(crate) const SNDI_POOL_SET_BASE: u32 = 0x003b_5b20;

/// The return address the original loads into `ra` for that call.
pub(crate) const POOL_SET_BASE_RET: u32 = 0x002a_927c;

/// `void PS2SoundManager_SetSpuMemBase(PS2SoundManager* this, int index)`
/// (0x2a9250): mark the manager (`this+0x31c = 1`) and hand the pool entry
/// `this[0xc + index*0x18]`'s two words to the SPU base setter.
pub fn ps2_sound_manager_set_spu_mem_base(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[V1][0] = 0x18;
        let frame = Frame::open(c, 0x10);
        let this = c.g[A0][0] as u32;
        let index = c.g[A1][0] as u32;
        let entry_off = abi::mult(c, index, 0x18);
        c.g[A1][0] = sx(entry_off);
        frame.save_ra(c, 0);
        c.g[V0][0] = 1;
        let one = c.g[V0][0];
        Addr(this.wrapping_add(0x31c)).sw(c, one);
        let entry = this.wrapping_add(entry_off.wrapping_add(0xc));
        c.g[A1][0] = sx(entry_off.wrapping_add(0xc));
        c.g[A0][0] = sx(entry);
        let flag = Addr(entry.wrapping_add(8)).lw(c);
        c.g[A1][0] = flag;
        let base = Addr(entry.wrapping_add(4)).lw(c);
        c.g[A0][0] = base;
        call_ptr(c, SNDI_POOL_SET_BASE, POOL_SET_BASE_RET)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SSXAudioSystem_KillRiderTimers(SSXAudioSystem* self, int unused)`
/// (0x292AE8): reset the bank monitor's timer through its vtable slot 4 and
/// then run the monitor's tick reset.
pub fn ssx_audio_system_kill_rider_timers(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw(ea(c.g[S0][0], 0x118));
        c.g[A0][0] = c.lw(c.g[V0][0] as u32);
        c.g[V1][0] = c.lw(ea(c.g[A0][0], 0xab0));
        c.g[A0][0] = sx(ea(c.g[A0][0], 0x1d8));
        c.g[V0][0] = c.lh(ea(c.g[V1][0], 0x20));
        c.g[A2][0] = c.lw(ea(c.g[V1][0], 0x24));
        let method = c.g[A2][0] as u32;
        c.g[A0][0] = sx(ea(c.g[A0][0], c.g[V0][0] as u32));
        call_ptr(c, method, 0x0029_2b18)?;
        c.g[V1][0] = c.lw(ea(c.g[S0][0], 0x118));
        c.g[V0][0] = 0x4a0000;
        c.ldl(5, ea(c.g[V0][0], 0x36af));
        c.ldr(5, ea(c.g[V0][0], 0x36a8));
        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
        c.g[A0][0] = sx(ea(c.g[A0][0], 0x1d8));
        call_ptr(c, addr::BANK_MONITOR_TIMER_RESET, 0x0029_2b34)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool SSXAudioSystem_IsTrackGroup2Event(SSXAudioSystem* self, int id)`
/// (0x2A4168): is `id` in the given track group, or is that group's flag set?
/// `ret0` is the first call's return address; the others follow at +0xC, +0x14
/// and +0x20 (the next instruction of each call in the original).
pub(crate) fn is_track_group_event(c: &mut Cpu, group: u8, ret0: u32) -> crate::call::Flow {
    c.cycles += 15;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    call_ptr(c, addr::CBE_GET_BE, ret0)?;
    c.g[A1][0] = 0;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, addr::CBE_GET_INTERFACE, ret0 + 0xc)?;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, addr::BE_INTERFACE_LOOKUP, ret0 + 0x14)?;
    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
    c.g[A0][0] = c.g[S0][0];
    if group == 2 {
        callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP2.call(c, ret0 + 0x20)?;
    } else {
        callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP3.call(c, ret0 + 0x20)?;
    }
    if c.g[V0][0] != 0 {
        c.g[V0][0] = 1;
    } else {
        c.cycles += 4;
        c.g[V1][0] = 0x530000;
        c.g[V0][0] = c.lb(ea(c.g[V1][0], 0x5c10));
        c.g[V0][0] ^= group as u64;
        c.g[V0][0] = (c.g[V0][0] < 1) as u64;
    }
    c.cycles += 4;
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `bool SSXAudioSystem_IsTrackGroup2Event(...)` (0x2A4168).
pub fn ssx_audio_system_is_track_group2_event(c: &mut Cpu) {
    guest(c, |c| is_track_group_event(c, 2, 0x002a_417c))
}

/// `bool SSXAudioSystem_IsTrackGroup3Event(...)` (0x2A41D8).
pub fn ssx_audio_system_is_track_group3_event(c: &mut Cpu) {
    guest(c, |c| is_track_group_event(c, 3, 0x002a_41ec))
}

/// `void* PS2SoundManager_dtor(PS2SoundManager* self, int flags)` (0x2A8C50):
/// install the vtable, free the owned buffer at `+4`, and delete the object
/// when bit 0 of `flags` is set.
pub fn ps2_sound_manager_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x480000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(ea(c.g[V0][0], 0x35f8));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw(ea(c.g[S0][0], 0), c.g[V0][0]);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = c.lw(ea(c.g[S0][0], 4));
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            call_ptr(c, addr::MEM_MAN_FREE_NOSIZE, 0x002a_8c84)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, addr::MEM_MAN_FREE, 0x002a_8c98)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_ResumeSfx(SSXAudioSystem* self)` (0x29ce70): if a sound
/// was suspended, resume all bank monitors, reset their distances and mark
/// every trigger active again.
pub fn ssx_audio_system_resume_sfx(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x5fb0)).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 15;
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x118)).lw(c);
            Addr(p(c.g[S0][0] as u32, 0x5fb0)).sw(c, 0);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
            call_ptr(c, 0x002a_d300, 0x0029_cea0)?;
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x118)).lw(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            Addr(p(c.g[V1][0] as u32, 0x26c)).sw(c, 0);
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x118)).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
            call_ptr(c, 0x002a_d810, 0x0029_cebc)?;
            c.g[A0][0] = Addr(gp(c, 0x21e4)).lw(c);
            call_ptr(c, 0x002b_6a68, 0x0029_cec4)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CrowdAudio_RemoveSource(CrowdAudio* self, int id)` (0x2a72d8).
pub fn crowd_audio_remove_source(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.sq(S0, p(c.g[SP][0] as u32, 0x20));
        c.swc1(20, p(c.g[SP][0] as u32, 0x30));
        c.g[S0][0] = c.g[A0][0];
        c.sd(p(c.g[SP][0] as u32, 0x10), c.g[RA][0]);
        let negative = (c.g[A1][0] as i64) < 0;
        c.mov_s(20, 12);
        if !negative {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x18)).lw(c);
            let absent = c.g[V0][0] == 0;
            c.g[A2][0] = c.g[SP][0];
            if !absent {
                c.cycles += 10;
                c.g[A3][0] = c.g[SP][0] | 4;
                call_ptr(c, 0x002a_72c0, 0x002a_7308)?;
                c.g[A0][0] = Addr(c.g[SP][0] as u32).lw(c);
                c.g[V0][0] = 0x190;
                c.g[V1][0] = Addr(p(c.g[S0][0] as u32, 0x18)).lw(c);
                c.mov_s(12, 20);
                c.mult(0, 4, 4, 2);
                c.g[A1][0] = Addr(p(c.g[SP][0] as u32, 4)).lw(c);
                c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, 0x002a_62f0, 0x002a_7328)?;
            }
        }
        c.cycles += 5;
        c.lq(S0, p(c.g[SP][0] as u32, 0x20));
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0x10));
        c.lwc1(20, p(c.g[SP][0] as u32, 0x30));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    })
}

/// `void CrowdAudio_SetSourceParam(CrowdAudio* self, int id, int param,
/// float value)` (0x2a7340).
pub fn crowd_audio_set_source_param(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.sq(S0, p(c.g[SP][0] as u32, 0x30));
        c.sq(S1, p(c.g[SP][0] as u32, 0x20));
        c.g[S0][0] = c.g[A0][0];
        c.sd(p(c.g[SP][0] as u32, 0x10), c.g[RA][0]);
        let negative = (c.g[A1][0] as i64) < 0;
        c.g[S1][0] = c.g[A2][0];
        if !negative {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x18)).lw(c);
            let absent = c.g[V0][0] == 0;
            c.g[A2][0] = c.g[SP][0];
            if !absent {
                c.cycles += 10;
                c.g[A3][0] = c.g[SP][0] | 4;
                call_ptr(c, 0x002a_72c0, 0x002a_7370)?;
                c.g[A0][0] = Addr(c.g[SP][0] as u32).lw(c);
                c.g[V0][0] = 0x190;
                c.g[V1][0] = Addr(p(c.g[S0][0] as u32, 0x18)).lw(c);
                c.g[A2][0] = c.g[S1][0];
                c.mult(0, 4, 4, 2);
                c.g[A1][0] = Addr(p(c.g[SP][0] as u32, 4)).lw(c);
                c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, 0x002a_5cd8, 0x002a_7390)?;
            }
        }
        c.cycles += 5;
        c.lq(S0, p(c.g[SP][0] as u32, 0x30));
        c.lq(S1, p(c.g[SP][0] as u32, 0x20));
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0x10));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    })
}

/// `int IniParser_GetFloat(IniParser* self, const char* name, float* out)`
/// (0x2a8070).
pub fn ini_parser_get_float(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        c.g[A0][0] = sx(p(c.g[S0][0] as u32, 0x0c));
        call_ptr(c, 0x0041_aa88, 0x002a_8090)?;
        let mismatch = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !mismatch {
            c.cycles += 6;
            c.g[A1][0] = 0x004a_3740;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002a_7ef0, 0x002a_80a8)?;
            let bad = c.g[V0][0] != 0;
            c.g[V0][0] = 0;
            if !bad {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002a_7e60, 0x002a_80b8)?;
                c.swc1(0, c.g[S1][0] as u32);
                c.g[V0][0] = 1;
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

/// `int IniParser_GetInt(IniParser* self, const char* name, int* out)`
/// (0x2a80d8).
pub fn ini_parser_get_int(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        c.g[A0][0] = sx(p(c.g[S0][0] as u32, 0x0c));
        call_ptr(c, 0x0041_aa88, 0x002a_80f8)?;
        let mismatch = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if !mismatch {
            c.cycles += 6;
            c.g[A1][0] = 0x004a_3740;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002a_7ef0, 0x002a_8110)?;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = 0;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002a_7ea8, 0x002a_8120)?;
                c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                c.g[V0][0] = 1;
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

/// `SSXAudioSystem_GetSurfaceSoundType`'s table (`0x480000 + 0x2aa0`),
/// indexed by the surface type id + 1 (ids < 0x15).
pub(crate) const SURFACE_SOUND_TABLE: u32 = 0x0048_2aa0;

/// `SSXAudioSystem_GetSurfaceGroup`'s table (`0x480000 + 0x2b50`), indexed by
/// the group id (ids < 9).
pub(crate) const SURFACE_GROUP_TABLE: u32 = 0x0048_2b50;

/// `int func_00291710(SSXAudioSystem *sys, Rider *rider)` (0x291710): the
/// surface sound type for the rider's current surface. The rider's main
/// state (`Rider_GetMainState`) picks `+0x438` normally, or the system's
/// `+0x598c` word in state 4; the id + 1 then maps through the table at
/// [`SURFACE_SOUND_TABLE`] (ids above 0x14 give 0).
pub fn ssx_audio_get_surface_sound_type(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        callee::RIDER_GET_MAIN_STATE.call(c, 0x0029_1730)?;
        c.g[V1][0] = 0x4;
        let id;
        if c.g[V0][0] != c.g[V1][0] {
            c.g[V0][0] = Addr(r32(c, S0)).off(0x438).lw(c);
            id = c.g[V0][0];
        } else {
            c.cycles += 1;
            c.g[V0][0] = Addr(r32(c, S1)).off(0x598c).lw(c);
            id = c.g[V0][0];
        }
        // 0x291740: lookup by id + 1.
        c.cycles += 4;
        c.g[V1][0] = sx((id as u32).wrapping_add(0x1));
        c.g[V0][0] = (c.g[V1][0] < 0x15) as u64;
        let out_of_range = c.g[V0][0] == 0;
        c.g[V0][0] = 0x0048_0000;
        if out_of_range {
            c.cycles += 1;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx(SURFACE_SOUND_TABLE);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            match c.g[A0][0] as u32 {
                0x0029_1768 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x1;
                }
                0x0029_1770 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x2;
                }
                0x0029_1778 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x3;
                }
                0x0029_1780 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x7;
                }
                0x0029_1788 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x5;
                }
                0x0029_1790 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x4;
                }
                0x0029_1798 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x8;
                }
                0x0029_17a0 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                }
                target => {
                    c.jump_out(target, 0x0029_1710, 0x0029_17b8);
                    return Ok(());
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int func_002934D0(SSXAudioSystem *sys, int group, Rider *rider)`
/// (0x2934d0): the surface group a rider's current surface belongs to. Group
/// ids below nine are looked up in the table at [`SURFACE_GROUP_TABLE`]; one
/// handler asks `Rider_GetMainState` and returns 2 unless the state is 4.
pub fn ssx_audio_get_surface_group(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = (c.g[A1][0] < 0x9) as u64;
        let out_of_range = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        let result: u64;
        if !out_of_range {
            c.cycles += 7;
            c.g[V0][0] = 0x0048_0000;
            c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
            c.g[V0][0] = sx(SURFACE_GROUP_TABLE);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            match c.g[A0][0] as u32 {
                0x0029_34fc => {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[A2][0];
                    callee::RIDER_GET_MAIN_STATE.call(c, 0x0029_3504)?;
                    c.g[V0][0] ^= 0x4;
                    c.g[V1][0] = 0x2;
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = 0;
                    }
                    result = c.g[V1][0];
                }
                0x0029_3514 => {
                    c.cycles += 2;
                    c.g[V1][0] = 0x1;
                    result = c.g[V1][0];
                }
                0x0029_351c => {
                    c.cycles += 2;
                    c.g[V1][0] = 0x2;
                    result = c.g[V1][0];
                }
                0x0029_3524 => {
                    c.cycles += 1;
                    c.g[V1][0] = 0;
                    result = 0;
                }
                target => {
                    c.jump_out(target, 0x0029_34d0, 0x0029_3538);
                    return Ok(());
                }
            }
        } else {
            // 0x293524: an unknown group id is group 0.
            c.cycles += 1;
            c.g[V1][0] = 0;
            result = 0;
        }
        c.cycles += 4;
        frame.restore_ra(c, 0);
        c.g[V0][0] = result;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Audio character masks: two jump-table lookups in `.rodata`.

/// `u32 SSXAudioSystem_CharacterToSpeechMask(int character, int unused)`
/// (0x29f2b0): one speech bit per character id 0..9 (1,2,4,8,0x10,0x20,0x80,
/// 0x40,0x100,0x200); 0 for an unknown id or a non-null second argument.
pub fn character_to_speech_mask(c: &mut Cpu) {
    let mut pc = c.pc as u32;
    loop {
        match pc {
            0x0029_f2b0 => {
                c.cycles += 2;
                let skip = c.g[A1][0] != 0;
                c.g[V0][0] = (c.g[A0][0] < 0xau64) as u64;
                pc = if skip { 0x0029_f328 } else { 0x0029_f2b8 };
            }
            0x0029_f2b8 => {
                c.cycles += 2;
                let skip = c.g[V0][0] == 0;
                c.g[V0][0] = 0x48_0000;
                pc = if skip { 0x0029_f328 } else { 0x0029_f2c0 };
            }
            0x0029_f2c0 => {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2e60));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                let target = c.g[A0][0] as u32;
                pc = match target {
                    0x0029_f2d8 => 0x0029_f2d8,
                    0x0029_f2e0 => 0x0029_f2e0,
                    0x0029_f2e8 => 0x0029_f2e8,
                    0x0029_f2f0 => 0x0029_f2f0,
                    0x0029_f2f8 => 0x0029_f2f8,
                    0x0029_f300 => 0x0029_f300,
                    0x0029_f308 => 0x0029_f308,
                    0x0029_f310 => 0x0029_f310,
                    0x0029_f318 => 0x0029_f318,
                    0x0029_f320 => 0x0029_f320,
                    _ => {
                        c.jump_out(target, 0x0029_f2b0, 0x0029_f330);
                        return;
                    }
                };
            }
            0x0029_f2d8 | 0x0029_f2e0 | 0x0029_f2e8 | 0x0029_f2f0 | 0x0029_f2f8 | 0x0029_f300
            | 0x0029_f308 | 0x0029_f310 | 0x0029_f318 | 0x0029_f320 => {
                c.cycles += 2;
                let mask = match pc {
                    0x0029_f2d8 => 0x1,
                    0x0029_f2e0 => 0x2,
                    0x0029_f2e8 => 0x4,
                    0x0029_f2f0 => 0x8,
                    0x0029_f2f8 => 0x10,
                    0x0029_f300 => 0x20,
                    0x0029_f308 => 0x80,
                    0x0029_f310 => 0x40,
                    0x0029_f318 => 0x100,
                    _ => 0x200,
                };
                c.g[V0][0] = mask;
                abi::ret(c);
                return;
            }
            0x0029_f328 => {
                c.cycles += 2;
                c.g[V0][0] = 0;
                abi::ret(c);
                return;
            }
            _ => {
                c.jump_out(pc, 0x0029_f2b0, 0x0029_f330);
                return;
            }
        }
    }
}

/// `u32 SSXAudioSystem_CharacterToVoiceMask(int character, int unused)`
/// (0x2a1da0): one voice bit per character id 0..9 (1,2,4,8,0x10,0x20,0x40,
/// 0x80,0x100,0x200); 0x1000 for an unknown id or a non-null third argument.
pub fn character_to_voice_mask(c: &mut Cpu) {
    let mut pc = c.pc as u32;
    loop {
        match pc {
            0x002a_1da0 => {
                c.cycles += 2;
                let skip = c.g[A2][0] != 0;
                c.g[V0][0] = (c.g[A1][0] < 0xau64) as u64;
                pc = if skip { 0x002a_1e18 } else { 0x002a_1da8 };
            }
            0x002a_1da8 => {
                c.cycles += 2;
                let skip = c.g[V0][0] == 0;
                c.g[V0][0] = 0x48_0000;
                pc = if skip { 0x002a_1e18 } else { 0x002a_1db0 };
            }
            0x002a_1db0 => {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2fe0));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                let target = c.g[A0][0] as u32;
                pc = match target {
                    0x002a_1dc8 => 0x002a_1dc8,
                    0x002a_1dd0 => 0x002a_1dd0,
                    0x002a_1dd8 => 0x002a_1dd8,
                    0x002a_1de0 => 0x002a_1de0,
                    0x002a_1de8 => 0x002a_1de8,
                    0x002a_1df0 => 0x002a_1df0,
                    0x002a_1df8 => 0x002a_1df8,
                    0x002a_1e00 => 0x002a_1e00,
                    0x002a_1e08 => 0x002a_1e08,
                    0x002a_1e10 => 0x002a_1e10,
                    _ => {
                        c.jump_out(target, 0x002a_1da0, 0x002a_1e20);
                        return;
                    }
                };
            }
            0x002a_1dc8 | 0x002a_1dd0 | 0x002a_1dd8 | 0x002a_1de0 | 0x002a_1de8 | 0x002a_1df0
            | 0x002a_1df8 | 0x002a_1e00 | 0x002a_1e08 | 0x002a_1e10 => {
                c.cycles += 2;
                let mask = match pc {
                    0x002a_1dc8 => 0x1,
                    0x002a_1dd0 => 0x2,
                    0x002a_1dd8 => 0x4,
                    0x002a_1de0 => 0x8,
                    0x002a_1de8 => 0x10,
                    0x002a_1df0 => 0x20,
                    0x002a_1df8 => 0x40,
                    0x002a_1e00 => 0x80,
                    0x002a_1e08 => 0x100,
                    _ => 0x200,
                };
                c.g[V0][0] = mask;
                abi::ret(c);
                return;
            }
            0x002a_1e18 => {
                c.cycles += 2;
                c.g[V0][0] = 0x1000;
                abi::ret(c);
                return;
            }
            _ => {
                c.jump_out(pc, 0x002a_1da0, 0x002a_1e20);
                return;
            }
        }
    }
}

/// `CrowdAudio_InstanceStopSound(self)` 0x2a6848: clamp the instance volume
/// to 0.25, clear the playing flag and, when a sound is live, stop the bank
/// monitor handle.
pub fn crowd_audio_instance_stop_sound(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        let this = abi::arg(c, 0);
        c.g[AT][0] = 0x3e80_0000;
        c.mtc1(1, 1);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(this);
        c.lwc1(0, p(this, 0x188));
        c.c_lt_s(1, 0);
        if c.fcc() {
            c.swc1(1, p(this, 0x188));
        }
        c.cycles += 3;
        c.g[A1][0] = lw_at(c, this, 0x174);
        let handle = c.g[A1][0];
        Addr(p(this, 0x180)).sw(c, 0);
        if (handle as i64) >= 0 {
            c.cycles += 8;
            c.g[V0][0] = Addr(gp_2(c, 0x608)).lw(c);
            c.g[A2][0] = 2;
            c.mtc1(0, 12);
            c.g[A0][0] = Addr(p(r32(c, V0), 0)).lw(c);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x1d8));
            call_ptr(c, 0x002a_d5f0, 0x002a_6898)?;
            c.g[V0][0] = u64::MAX;
            Addr(p(this, 0x174)).sw(c, c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem_IsTrackGroup0Event(self)` 0x2a4078: true when the BE
/// interface's current track is in group 0, or when the game state is 0 or 5.
pub fn ssx_audio_system_is_track_group0_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg64(c, 0);
        c.g[S0][0] = this;
        call_ptr(c, 0x0014_dc80, 0x002a_408c)?;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dd58, 0x002a_4098)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_4bc0, 0x002a_40a0)?;
        c.g[A1][0] = Addr(r32(c, V0)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002a_40e0, 0x002a_40ac)?;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = 1;
        } else {
            c.cycles += 4;
            c.g[V0][0] = 0x530000;
            c.g[V0][0] = Addr(0x0053_5c10).lb(c);
            let was_zero = c.g[V0][0] == 0;
            c.g[V0][0] ^= 5;
            if was_zero {
                c.cycles += 1;
                c.g[V0][0] = 1;
            } else {
                c.cycles += 2;
                c.g[V0][0] = (c.g[V0][0] < 1) as u64;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `gp + 0x608`: the SND system. (Unlike the game globals this one lives
/// above `gp`.)
#[cfg(test)]
pub(crate) const SND_SYSTEM_GP: u32 = 0x608;

// ---------------------------------------------------------------------------
// The body.

/// `CrowdAudio_UpdateInstance(voice)` 0x2a4e88: advance the voice's motion
/// slots and keep its MIDI player in step with them.
pub fn crowd_audio_update_instance(c: &mut Cpu) {
    guest(c, body);
}

pub(crate) fn body(c: &mut Cpu) -> Flow {
    'b_2a5ca4: {
        c.cycles += 15;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x130));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0xb0));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.g[S7][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0xd0));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0xc0));
        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x90), c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0xd4));
        let t = c.g[V0][0] == 0u64;
        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        if t {
            break 'b_2a5ca4;
        } else {
            c.cycles += 11;
            c.sw(c.g[SP][0] as u32, 0u64);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8c), c.g[V1][0]);
            c.g[T1][0] = 0u64;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
            c.g[T2][0] = 0u64;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
            c.g[V0][0] = 0x2;
            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0u64);
            c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[V1][0] = 0xffffffffffffffff;
            'l_2a4ef0: loop {
                c.cycles += 7;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                let t = c.g[V0][0] != c.g[V1][0];
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8c));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x002a4ef0);
                    }
                    continue 'l_2a4ef0;
                } else {
                    c.cycles += 3;
                    c.g[V1][0] = c.g[S3][0];
                    c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S5][0] = 0x2;
                    'l_2a4f18: loop {
                        c.cycles += 11;
                        c.sw(c.g[V0][0] as u32, 0u64);
                        c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x1));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                        c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0u64);
                        c.sw(c.g[V1][0] as u32, 0u64);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                        c.sw(c.g[A0][0] as u32, 0u64);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                        let t = (c.g[S5][0] as i64) >= 0;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x002a4f18);
                            }
                            continue 'l_2a4f18;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = 0xc;
                            c.mult(0, 2, 9, 2);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
                            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S7][0] as u32));
                            c.g[A3][0] = c.g[V1][0];
                            'l_2a4f58: loop {
                                'b_2a4f78: {
                                    c.cycles += 5;
                                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                                    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0xc));
                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0xc));
                                    let t = c.g[A0][0] != 0u64;
                                    c.g[S6][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                    if t {
                                        break 'b_2a4f78;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = ((c.g[T1][0] as i64) < 15i64) as u64;
                                        if c.g[V0][0] != 0u64 {
                                            c.g[A3][0] = c.g[V1][0];
                                            if event_due(c) {
                                                deliver_event_here(c, 0x002a4f58);
                                            }
                                            continue 'l_2a4f58;
                                        } else {
                                            break 'b_2a4f78;
                                        }
                                    }
                                }
                                'b_2a51d4: {
                                    c.cycles += 16;
                                    c.lq(4, c.g[A0][0] as u32);
                                    c.g[S5][0] = 0x1;
                                    c.g[A1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0xd4));
                                    c.sq(4, c.g[SP][0] as u32);
                                    c.g[A1][0] = ((c.g[S5][0] as i64) < (c.g[A1][0] as i64)) as u64;
                                    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x20));
                                    c.g[FP][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[SP][0] as u32));
                                    c.sq(4, (c.g[V0][0] as u32).wrapping_add(0x10));
                                    c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x20));
                                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V1][0] =
                                        sx((c.g[S3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                    let t = c.g[A1][0] == 0u64;
                                    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                                    if t {
                                        break 'b_2a51d4;
                                    } else {
                                        c.cycles += 6;
                                        c.g[V0][0] = 0xc;
                                        c.g[T3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8c));
                                        c.mult(0, 2, 9, 2);
                                        c.g[T4][0] = c.g[S3][0];
                                        c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
                                        c.lwc1(5, c.g[SP][0] as u32);
                                        'l_2a4fd0: loop {
                                            c.cycles += 4;
                                            c.g[T5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                                            c.g[V1][0] =
                                                sx((c.g[T0][0] as u32)
                                                    .wrapping_add(c.g[S7][0] as u32));
                                            c.g[A3][0] = c.g[V1][0];
                                            'l_2a4fe0: loop {
                                                'b_2a5004: {
                                                    c.cycles += 6;
                                                    c.g[T0][0] =
                                                        sx((c.g[T0][0] as u32).wrapping_add(0xc));
                                                    c.g[V1][0] =
                                                        sx((c.g[A3][0] as u32).wrapping_add(0xc));
                                                    c.g[S6][0] =
                                                        c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                                    c.g[A0][0] =
                                                        c.lw((c.g[V1][0] as u32).wrapping_sub(0xc));
                                                    let t = c.g[A0][0] != 0u64;
                                                    c.g[T1][0] =
                                                        sx((c.g[T1][0] as u32).wrapping_add(0x1));
                                                    if t {
                                                        break 'b_2a5004;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[V0][0] =
                                                            ((c.g[T1][0] as i64) < 15i64) as u64;
                                                        if c.g[V0][0] != 0u64 {
                                                            c.g[A3][0] = c.g[V1][0];
                                                            if event_due(c) {
                                                                deliver_event_here(c, 0x002a4fe0);
                                                            }
                                                            continue 'l_2a4fe0;
                                                        } else {
                                                            break 'b_2a5004;
                                                        }
                                                    }
                                                }
                                                'b_2a5090: {
                                                    'b_2a5088: {
                                                        c.cycles += 30;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0x4));
                                                        c.lwc1(0, c.g[A0][0] as u32);
                                                        c.g[V1][0] = sx((c.g[FP][0] as u32)
                                                            .wrapping_add(c.g[V0][0] as u32));
                                                        c.mtc1(2, 4);
                                                        c.cvt_s_w(4, 4);
                                                        c.mtc1(3, 2);
                                                        c.cvt_s_w(2, 2);
                                                        c.lwc1(
                                                            1,
                                                            (c.g[SP][0] as u32).wrapping_add(0x4),
                                                        );
                                                        c.sub_s(0, 0, 5);
                                                        c.lwc1(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x8),
                                                        );
                                                        c.div_s(4, 4, 2);
                                                        c.mul_s(0, 0, 4);
                                                        c.add_s(0, 5, 0);
                                                        c.swc1(0, c.g[SP][0] as u32);
                                                        c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                                                        c.lwc1(
                                                            0,
                                                            (c.g[V0][0] as u32).wrapping_add(0x4),
                                                        );
                                                        c.sub_s(0, 0, 1);
                                                        c.mul_s(0, 0, 4);
                                                        c.add_s(1, 1, 0);
                                                        c.swc1(
                                                            1,
                                                            (c.g[SP][0] as u32).wrapping_add(0x4),
                                                        );
                                                        c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                                                        c.lwc1(
                                                            0,
                                                            (c.g[V0][0] as u32).wrapping_add(0x8),
                                                        );
                                                        c.sub_s(0, 0, 3);
                                                        c.mul_s(0, 0, 4);
                                                        c.add_s(3, 3, 0);
                                                        c.swc1(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x8),
                                                        );
                                                        c.g[V1][0] = c
                                                            .lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0x4));
                                                        c.g[V0][0] = ((c.g[FP][0] as i64)
                                                            < (c.g[V1][0] as i64))
                                                            as u64;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] =
                                                            sx(((c.g[FP][0] as u32) as i32 >> 2)
                                                                as u32);
                                                        if t {
                                                            break 'b_2a5088;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[T2][0] = sx((c.g[T2][0] as u32)
                                                                .wrapping_add(c.g[V0][0] as u32));
                                                            c.g[FP][0] = c.g[V1][0];
                                                            break 'b_2a5090;
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32
                                                        >> 2)
                                                        as u32);
                                                    c.g[T2][0] = sx((c.g[T2][0] as u32)
                                                        .wrapping_add(c.g[V0][0] as u32));
                                                    break 'b_2a5090;
                                                }
                                                'b_2a51c4: {
                                                    c.cycles += 4;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S7][0] as u32).wrapping_add(0x170)
                                                        );
                                                    c.g[V0][0] =
                                                        ((c.g[V0][0] as i64) < 2i64) as u64;
                                                    if c.g[V0][0] != 0u64 {
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S7][0] as u32)
                                                                .wrapping_add(0xd4));
                                                        break 'b_2a51c4;
                                                    } else {
                                                        'b_2a51c0: {
                                                            'b_2a51ac: {
                                                                c.cycles += 57;
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[S6][0] as u32)
                                                                        .wrapping_add(0x20));
                                                                c.g[A2][0] = c
                                                                    .lw((c.g[A3][0] as u32)
                                                                        .wrapping_add(0x4));
                                                                c.g[V0][0] =
                                                                    sx((c.g[V1][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[T4][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[V0][0] as u32,
                                                                    ));
                                                                c.mtc1(6, 4);
                                                                c.cvt_s_w(4, 4);
                                                                c.g[A0][0] =
                                                                    c.lw(c.g[V0][0] as u32);
                                                                c.g[V1][0] =
                                                                    sx((c.g[V1][0] as u32) << 4);
                                                                c.g[V0][0] =
                                                                    c.lw(c.g[A3][0] as u32);
                                                                c.g[A1][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[SP][0] as u32,
                                                                    ));
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[A2][0] as u32,
                                                                    ));
                                                                c.lwc1(
                                                                    2,
                                                                    (c.g[A1][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                );
                                                                c.mtc1(4, 3);
                                                                c.cvt_s_w(3, 3);
                                                                c.lwc1(0, c.g[V0][0] as u32);
                                                                c.g[V1][0] = c.g[A1][0];
                                                                c.sub_s(0, 0, 2);
                                                                c.g[V1][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x10));
                                                                c.div_s(4, 4, 3);
                                                                c.lwc1(1, c.g[V1][0] as u32);
                                                                c.mul_s(0, 0, 4);
                                                                c.add_s(1, 1, 0);
                                                                c.swc1(1, c.g[V1][0] as u32);
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S6][0] as u32)
                                                                        .wrapping_add(0x20));
                                                                c.g[A0][0] =
                                                                    c.lw(c.g[A3][0] as u32);
                                                                c.g[V0][0] =
                                                                    sx((c.g[V0][0] as u32) << 4);
                                                                c.g[V1][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[SP][0] as u32,
                                                                    ));
                                                                c.lwc1(
                                                                    1,
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x4),
                                                                );
                                                                c.lwc1(
                                                                    2,
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0x14),
                                                                );
                                                                c.g[V0][0] = c.g[V1][0];
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x10));
                                                                c.sub_s(1, 1, 2);
                                                                c.lwc1(
                                                                    0,
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x4),
                                                                );
                                                                c.mul_s(1, 1, 4);
                                                                c.add_s(0, 0, 1);
                                                                c.swc1(
                                                                    0,
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x4),
                                                                );
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[S6][0] as u32)
                                                                        .wrapping_add(0x20));
                                                                c.g[A0][0] =
                                                                    c.lw(c.g[A3][0] as u32);
                                                                c.g[V1][0] =
                                                                    sx((c.g[V1][0] as u32) << 4);
                                                                c.g[V0][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[SP][0] as u32,
                                                                    ));
                                                                c.lwc1(
                                                                    1,
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x8),
                                                                );
                                                                c.lwc1(
                                                                    2,
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x18),
                                                                );
                                                                c.g[V1][0] = c.g[V0][0];
                                                                c.g[V1][0] = sx((c.g[V1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x10));
                                                                c.sub_s(1, 1, 2);
                                                                c.lwc1(
                                                                    0,
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0x8),
                                                                );
                                                                c.mul_s(1, 1, 4);
                                                                c.add_s(0, 0, 1);
                                                                c.swc1(
                                                                    0,
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0x8),
                                                                );
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S6][0] as u32)
                                                                        .wrapping_add(0x20));
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[A3][0] as u32)
                                                                        .wrapping_add(0x4));
                                                                c.g[A0][0] =
                                                                    sx((c.g[V0][0] as u32) << 2);
                                                                c.g[V1][0] = sx((c.g[T4][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[A0][0] as u32,
                                                                    ));
                                                                c.g[V1][0] =
                                                                    c.lw(c.g[V1][0] as u32);
                                                                c.g[V0][0] = ((c.g[V1][0] as i64)
                                                                    < (c.g[A1][0] as i64))
                                                                    as u64;
                                                                let t = c.g[V0][0] == 0u64;
                                                                c.g[V1][0] =
                                                                    sx(((c.g[V1][0] as u32) as i32
                                                                        >> 2)
                                                                        as u32);
                                                                if t {
                                                                    break 'b_2a51ac;
                                                                } else {
                                                                    c.cycles += 10;
                                                                    c.g[A0][0] = sx((c.g[T3][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            c.g[A0][0] as u32,
                                                                        ));
                                                                    c.g[V0][0] =
                                                                        c.lw(c.g[A0][0] as u32);
                                                                    c.g[V0][0] = sx((c.g[V0][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            c.g[V1][0] as u32,
                                                                        ));
                                                                    c.sw(
                                                                        c.g[A0][0] as u32,
                                                                        c.g[V0][0],
                                                                    );
                                                                    c.g[V1][0] = c
                                                                        .lw((c.g[S6][0] as u32)
                                                                            .wrapping_add(0x20));
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[A3][0] as u32)
                                                                            .wrapping_add(0x4));
                                                                    c.g[V1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        << 2);
                                                                    c.g[V1][0] = sx((c.g[S3][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            c.g[V1][0] as u32,
                                                                        ));
                                                                    c.sw(
                                                                        c.g[V1][0] as u32,
                                                                        c.g[V0][0],
                                                                    );
                                                                    break 'b_2a51c0;
                                                                }
                                                            }
                                                            c.cycles += 5;
                                                            c.g[A0][0] = sx((c.g[T3][0] as u32)
                                                                .wrapping_add(c.g[A0][0] as u32));
                                                            c.g[V1][0] = sx(((c.g[A1][0] as u32)
                                                                as i32
                                                                >> 2)
                                                                as u32);
                                                            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(c.g[V1][0] as u32));
                                                            c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                                                            break 'b_2a51c0;
                                                        }
                                                        c.cycles += 1;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[S7][0] as u32)
                                                                .wrapping_add(0xd4));
                                                        break 'b_2a51c4;
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.g[S5][0] = c.g[T5][0];
                                                c.g[V0][0] = ((c.g[S5][0] as i64)
                                                    < (c.g[V0][0] as i64))
                                                    as u64;
                                                let t = c.g[V0][0] != 0u64;
                                                c.lwc1(5, c.g[SP][0] as u32);
                                                if t {
                                                    if event_due(c) {
                                                        deliver_event_here(c, 0x002a4fd0);
                                                    }
                                                    continue 'l_2a4fd0;
                                                } else {
                                                    break 'b_2a51d4;
                                                }
                                            }
                                        }
                                    }
                                }
                                'b_2a5a68: {
                                    'b_2a55e8: {
                                        c.cycles += 5;
                                        c.g[A0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x170));
                                        c.g[FP][0] =
                                            sx((c.g[FP][0] as u32).wrapping_add(c.g[T2][0] as u32));
                                        c.g[V0][0] = 0x1;
                                        let t = c.g[A0][0] != c.g[V0][0];
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x78), c.g[A0][0]);
                                        if t {
                                            break 'b_2a55e8;
                                        } else {
                                            c.cycles += 3;
                                            c.g[A1][0] =
                                                c.lw((c.g[S6][0] as u32).wrapping_add(0x4));
                                            let t = (c.g[A1][0] as i64) < 0;
                                            c.lq(2, c.g[SP][0] as u32);
                                            if t {
                                                break 'b_2a5a68;
                                            } else {
                                                c.cycles += 10;
                                                c.g[A3][0] =
                                                    sx((c.g[S6][0] as u32).wrapping_add(0x10));
                                                c.g[V1][0] =
                                                    c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                c.g[A2][0] = 0x1;
                                                c.sq(2, (c.g[S6][0] as u32).wrapping_add(0x10));
                                                c.sw(
                                                    (c.g[SP][0] as u32).wrapping_add(0x7c),
                                                    c.g[A3][0],
                                                );
                                                c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                                                callee::FIND_INSTANCE.call(c, 0x002a5214)?;
                                                if c.g[V0][0] != 0u64 {
                                                    c.sw(
                                                        (c.g[V0][0] as u32).wrapping_add(0x64),
                                                        c.g[FP][0],
                                                    );
                                                    break 'b_2a5a68;
                                                } else {
                                                    'b_2a54fc: {
                                                        'b_2a54d0: {
                                                            c.cycles += 167;
                                                            c.g[T0][0] = c.lw((c.g[S7][0] as u32)
                                                                .wrapping_add(0xd8));
                                                            c.g[A0][0] = 0x18;
                                                            c.g[S2][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_add(0x608));
                                                            c.g[A1][0] = 0u64;
                                                            c.sw(
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x80),
                                                                c.g[T0][0],
                                                            );
                                                            c.g[A2][0] = 0x8;
                                                            c.g[S0][0] = c.lw(c.g[S2][0] as u32);
                                                            c.g[S5][0] = c.lw(c.g[S6][0] as u32);
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1f4));
                                                            c.g[S1][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1d8));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x1));
                                                            c.sw(
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x1f4),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1f4));
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x20));
                                                            c.mult(0, 3, 3, 4);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(c.g[V0][0] as u32));
                                                            c.ldl(
                                                                7,
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x1e3),
                                                            );
                                                            c.ldr(
                                                                7,
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x1dc),
                                                            );
                                                            c.ldl(
                                                                8,
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x1eb),
                                                            );
                                                            c.ldr(
                                                                8,
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x1e4),
                                                            );
                                                            c.ldl(
                                                                2,
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x1f3),
                                                            );
                                                            c.ldr(
                                                                2,
                                                                (c.g[S0][0] as u32)
                                                                    .wrapping_add(0x1ec),
                                                            );
                                                            c.sdl(
                                                                7,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(7, c.g[V1][0] as u32);
                                                            c.sdl(
                                                                8,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                8,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                2,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x17),
                                                            );
                                                            c.sdr(
                                                                2,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1f4));
                                                            c.g[V0][0] =
                                                                sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(c.g[V0][0] as u32));
                                                            c.sw(
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x1fc),
                                                                0u64,
                                                            );
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1f4));
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x34));
                                                            c.g[V1][0] =
                                                                sx((c.g[V1][0] as u32) << 2);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(c.g[V0][0] as u32));
                                                            c.sw(c.g[V1][0] as u32, 0u64);
                                                            c.g[V0][0] = 0x18;
                                                            c.g[A3][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1f4));
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1f4));
                                                            c.g[T0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x20));
                                                            c.g[A3][0] =
                                                                sx((c.g[A3][0] as u32) << 2);
                                                            c.mult(0, 4, 4, 2);
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x38));
                                                            c.g[A3][0] = sx((c.g[A3][0] as u32)
                                                                .wrapping_add(c.g[V0][0] as u32));
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(c.g[T0][0] as u32));
                                                            c.g[V1][0] = c.lb(c.g[A0][0] as u32);
                                                            c.sw(c.g[A3][0] as u32, c.g[V1][0]);
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1f4));
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x3c));
                                                            c.g[V0][0] =
                                                                sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(c.g[V1][0] as u32));
                                                            c.sw(c.g[V0][0] as u32, 0u64);
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1f4));
                                                            c.g[A0][0] =
                                                                sx((c.g[A0][0] as u32) << 3);
                                                            c.g[A0][0] = sx((c.g[S1][0] as u32)
                                                                .wrapping_add(c.g[A0][0] as u32));
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x40));
                                                            callee::MEMSET.call(c, 0x002a5304)?;
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1f4));
                                                            c.g[A0][0] = 0x64;
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x60));
                                                            c.g[A1][0] = 0x5a;
                                                            c.g[V1][0] =
                                                                sx((c.g[V1][0] as u32) << 2);
                                                            c.g[A2][0] = 0x32;
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(c.g[V0][0] as u32));
                                                            c.g[A3][0] = 0x7f;
                                                            c.sw(c.g[V1][0] as u32, 0u64);
                                                            c.g[T0][0] = 0x18;
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1f4));
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x64));
                                                            c.g[V0][0] =
                                                                sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(c.g[V1][0] as u32));
                                                            c.sw(c.g[V0][0] as u32, 0u64);
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1f4));
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x68));
                                                            c.g[V1][0] =
                                                                sx((c.g[V1][0] as u32) << 2);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(c.g[V0][0] as u32));
                                                            c.sw(c.g[V1][0] as u32, 0u64);
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1f4));
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x6c));
                                                            c.g[V0][0] =
                                                                sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(c.g[V1][0] as u32));
                                                            c.sw(c.g[V0][0] as u32, 0u64);
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1f4));
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x70));
                                                            c.g[V1][0] =
                                                                sx((c.g[V1][0] as u32) << 4);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(c.g[V0][0] as u32));
                                                            c.sw(
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x4),
                                                                0u64,
                                                            );
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1f4));
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x70));
                                                            c.g[V0][0] =
                                                                sx((c.g[V0][0] as u32) << 4);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(c.g[V1][0] as u32));
                                                            c.sw(c.g[V0][0] as u32, c.g[A0][0]);
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1f4));
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x70));
                                                            c.g[V1][0] =
                                                                sx((c.g[V1][0] as u32) << 4);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(c.g[V0][0] as u32));
                                                            c.sw(
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                                c.g[A1][0],
                                                            );
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1f4));
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x70));
                                                            c.g[V0][0] =
                                                                sx((c.g[V0][0] as u32) << 4);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(c.g[V1][0] as u32));
                                                            c.sw(
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0xc),
                                                                c.g[A2][0],
                                                            );
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1f4));
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x74));
                                                            c.g[V1][0] =
                                                                sx((c.g[V1][0] as u32) << 2);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(c.g[V0][0] as u32));
                                                            c.sw(c.g[V1][0] as u32, 0u64);
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1f4));
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x80));
                                                            c.g[V0][0] =
                                                                sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(c.g[V1][0] as u32));
                                                            c.sw(c.g[V0][0] as u32, 0u64);
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1f4));
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x84));
                                                            c.g[V1][0] =
                                                                sx((c.g[V1][0] as u32) << 2);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(c.g[V0][0] as u32));
                                                            c.sw(c.g[V1][0] as u32, 0u64);
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1f4));
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x78));
                                                            c.g[V0][0] =
                                                                sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(c.g[V1][0] as u32));
                                                            c.sw(c.g[V0][0] as u32, c.g[A3][0]);
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1f4));
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x7c));
                                                            c.g[A0][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x78));
                                                            c.g[V1][0] =
                                                                sx((c.g[V1][0] as u32) << 2);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(c.g[V0][0] as u32));
                                                            c.sw(c.g[V1][0] as u32, c.g[A0][0]);
                                                            c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                                                            c.g[V1][0] = c.lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x1f4));
                                                            c.g[A1][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x80));
                                                            c.g[V0][0] =
                                                                sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(c.g[V1][0] as u32));
                                                            c.sw(c.g[V0][0] as u32, c.g[A1][0]);
                                                            c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                                                            c.g[A0][0] = c.lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x1f4));
                                                            c.g[V0][0] =
                                                                sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(c.g[A0][0] as u32));
                                                            c.sw(c.g[V0][0] as u32, c.g[S5][0]);
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_add(0x608));
                                                            c.g[A2][0] = c.lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x7c));
                                                            c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x1f4));
                                                            c.g[V0][0] =
                                                                sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(c.g[V0][0] as u32));
                                                            c.sw(
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x1fc),
                                                                c.g[A2][0],
                                                            );
                                                            c.g[A1][0] = c.lw(c.g[A0][0] as u32);
                                                            c.g[V0][0] = c.lw((c.g[A1][0] as u32)
                                                                .wrapping_add(0x1f4));
                                                            c.g[A2][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x1d8));
                                                            c.g[V1][0] = c.lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0x38));
                                                            c.g[V0][0] =
                                                                sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(c.g[V1][0] as u32));
                                                            c.sw(c.g[V0][0] as u32, c.g[FP][0]);
                                                            c.g[A0][0] = c.lw((c.g[A1][0] as u32)
                                                                .wrapping_add(0x1f4));
                                                            c.g[V1][0] = c.lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0x1c));
                                                            c.g[A3][0] = c.lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0x38));
                                                            c.mult(0, 4, 4, 8);
                                                            c.g[V1][0] =
                                                                sx((c.g[V1][0] as u32) << 2);
                                                            c.g[A1][0] = c.lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0x20));
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(c.g[A3][0] as u32));
                                                            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                                            c.g[V0][0] = ((c.g[V0][0] as i64)
                                                                < 127i64)
                                                                as u64;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(c.g[A1][0] as u32));
                                                            if t {
                                                                break 'b_2a54d0;
                                                            } else {
                                                                c.cycles += 6;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[A2][0] as u32)
                                                                        .wrapping_add(0x1c));
                                                                c.g[V0][0] =
                                                                    sx((c.g[V0][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[A3][0] as u32,
                                                                    ));
                                                                c.g[V1][0] =
                                                                    c.lw(c.g[V0][0] as u32);
                                                                let t = (c.g[V1][0] as i64) < 0;
                                                                c.g[V0][0] = 0u64;
                                                                if t {
                                                                    break 'b_2a54fc;
                                                                } else {
                                                                    break 'b_2a54d0;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 7;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0x1c));
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                            .wrapping_add(c.g[A3][0] as u32));
                                                        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                                        c.g[V1][0] =
                                                            ((c.g[V1][0] as i64) < 127i64) as u64;
                                                        if c.g[V1][0] == 0u64 {
                                                            c.g[V0][0] = 0x7f;
                                                            break 'b_2a54fc;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.g[V0][0] = c.lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0x1c));
                                                            c.g[V0][0] =
                                                                sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(c.g[A3][0] as u32));
                                                            c.g[V0][0] = c.lbu(c.g[V0][0] as u32);
                                                            break 'b_2a54fc;
                                                        }
                                                    }
                                                    c.cycles += 59;
                                                    c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                                                    c.g[A2][0] = 0x1;
                                                    c.g[A3][0] = 0x4a0000;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_add(0x608)
                                                        );
                                                    c.g[A1][0] =
                                                        c.lw(
                                                            (c.g[S7][0] as u32).wrapping_add(0x17c)
                                                        );
                                                    c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x1f4)
                                                        );
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x214)
                                                        );
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    c.sw(c.g[V0][0] as u32, c.g[A1][0]);
                                                    c.g[V1][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_add(0x608)
                                                        );
                                                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[A0][0] as u32).wrapping_add(0x1f4)
                                                        );
                                                    c.g[V1][0] =
                                                        c.lw(
                                                            (c.g[A0][0] as u32).wrapping_add(0x24c)
                                                        );
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add(c.g[V1][0] as u32));
                                                    c.sw(c.g[V0][0] as u32, c.g[A2][0]);
                                                    c.g[V1][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_add(0x608)
                                                        );
                                                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                    c.g[A2][0] =
                                                        sx((c.g[A3][0] as u32)
                                                            .wrapping_add(0x3700));
                                                    c.ldl(2, (c.g[A2][0] as u32).wrapping_add(0x7));
                                                    c.ldr(2, c.g[A2][0] as u32);
                                                    c.sdl(
                                                        2,
                                                        (c.g[SP][0] as u32).wrapping_add(0x67),
                                                    );
                                                    c.sdr(
                                                        2,
                                                        (c.g[SP][0] as u32).wrapping_add(0x60),
                                                    );
                                                    c.g[V1][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[A0][0] as u32).wrapping_add(0x1f4)
                                                        );
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                        .wrapping_add(c.g[V0][0] as u32));
                                                    c.ldl(
                                                        7,
                                                        (c.g[SP][0] as u32).wrapping_add(0x67),
                                                    );
                                                    c.ldr(
                                                        7,
                                                        (c.g[SP][0] as u32).wrapping_add(0x60),
                                                    );
                                                    c.sdl(
                                                        7,
                                                        (c.g[V1][0] as u32).wrapping_add(0x47),
                                                    );
                                                    c.sdr(
                                                        7,
                                                        (c.g[V1][0] as u32).wrapping_add(0x40),
                                                    );
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_add(0x608)
                                                        );
                                                    c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x1f4)
                                                        );
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x238)
                                                        );
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    c.sw(c.g[V0][0] as u32, c.g[S7][0]);
                                                    c.g[V1][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_add(0x608)
                                                        );
                                                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[A0][0] as u32).wrapping_add(0x1f4)
                                                        );
                                                    c.g[V1][0] =
                                                        c.lw(
                                                            (c.g[A0][0] as u32).wrapping_add(0x254)
                                                        );
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add(c.g[V1][0] as u32));
                                                    c.sw(c.g[V0][0] as u32, 0u64);
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_add(0x608)
                                                        );
                                                    c.g[A1][0] = c.lw(c.g[S7][0] as u32);
                                                    c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x1f4)
                                                        );
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x23c)
                                                        );
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    c.sw(c.g[V0][0] as u32, c.g[A1][0]);
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_add(0x608)
                                                        );
                                                    callee::PLAY_PUSHED_SOUND
                                                        .call(c, 0x002a55e0)?;
                                                    c.sw(
                                                        (c.g[S6][0] as u32).wrapping_add(0x4),
                                                        c.g[V0][0],
                                                    );
                                                    break 'b_2a5a68;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    c.g[S5][0] = 0u64;
                                    c.g[V0][0] = 0x30;
                                    'l_2a55f0: loop {
                                        'b_2a5a5c: {
                                            c.cycles += 6;
                                            c.mult(0, 2, 21, 2);
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xe0));
                                            c.g[S6][0] =
                                                sx((c.g[S7][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                            c.g[V1][0] =
                                                c.lw((c.g[S6][0] as u32).wrapping_add(0x8));
                                            if c.g[V1][0] == 0u64 {
                                                c.g[S5][0] =
                                                    sx((c.g[S5][0] as u32).wrapping_add(0x1));
                                                break 'b_2a5a5c;
                                            } else {
                                                'b_2a5a58: {
                                                    c.cycles += 3;
                                                    c.g[A1][0] =
                                                        c.lw((c.g[S6][0] as u32).wrapping_add(0x4));
                                                    let t = (c.g[A1][0] as i64) < 0;
                                                    c.g[A2][0] = 0x1;
                                                    if t {
                                                        break 'b_2a5a58;
                                                    } else {
                                                        'b_2a5a38: {
                                                            c.cycles += 13;
                                                            c.g[V1][0] = c.lw((c.g[S6][0] as u32)
                                                                .wrapping_add(0x20));
                                                            c.g[A3][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_add(0x608));
                                                            c.g[S4][0] = sx((c.g[S6][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.g[V1][0] =
                                                                sx((c.g[V1][0] as u32) << 4);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                                .wrapping_add(c.g[SP][0] as u32));
                                                            c.lq(
                                                                2,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            c.sq(
                                                                2,
                                                                (c.g[S6][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            c.g[A0][0] = c.lw(c.g[A3][0] as u32);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x1d8));
                                                            callee::FIND_INSTANCE
                                                                .call(c, 0x002a563c)?;
                                                            c.g[A2][0] = c.g[V0][0];
                                                            if c.g[A2][0] != 0u64 {
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[S6][0] as u32)
                                                                        .wrapping_add(0x20));
                                                                break 'b_2a5a38;
                                                            } else {
                                                                'b_2a594c: {
                                                                    'b_2a5920: {
                                                                        c.cycles += 176;
                                                                        c.g[A2][0] = c.lw((c.g[S7]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0xd8));
                                                                        c.g[T0][0] = 0x18;
                                                                        c.g[S2][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x608));
                                                                        c.g[A1][0] = 0u64;
                                                                        c.sw(
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x84),
                                                                            c.g[A2][0],
                                                                        );
                                                                        c.g[A2][0] = 0x8;
                                                                        c.g[A3][0] =
                                                                            c.lw(c.g[S6][0] as u32);
                                                                        c.g[S0][0] =
                                                                            c.lw(c.g[S2][0] as u32);
                                                                        c.sw(
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x88),
                                                                            c.g[A3][0],
                                                                        );
                                                                        c.g[S1][0] =
                                                                            sx((c.g[S0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x1d8,
                                                                                ));
                                                                        c.g[V0][0] = c.lw((c.g[S0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1f4));
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(0x1));
                                                                        c.sw(
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x1f4,
                                                                                ),
                                                                            c.g[V0][0],
                                                                        );
                                                                        c.g[V1][0] = c.lw((c.g[S0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1f4));
                                                                        c.g[V0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x20));
                                                                        c.mult(0, 3, 3, 8);
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V0][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.ldl(
                                                                            2,
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x1e3,
                                                                                ),
                                                                        );
                                                                        c.ldr(
                                                                            2,
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x1dc,
                                                                                ),
                                                                        );
                                                                        c.ldl(
                                                                            4,
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x1eb,
                                                                                ),
                                                                        );
                                                                        c.ldr(
                                                                            4,
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x1e4,
                                                                                ),
                                                                        );
                                                                        c.ldl(
                                                                            7,
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x1f3,
                                                                                ),
                                                                        );
                                                                        c.ldr(
                                                                            7,
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x1ec,
                                                                                ),
                                                                        );
                                                                        c.sdl(
                                                                            2,
                                                                            (c.g[V1][0] as u32)
                                                                                .wrapping_add(0x7),
                                                                        );
                                                                        c.sdr(2, c.g[V1][0] as u32);
                                                                        c.sdl(
                                                                            4,
                                                                            (c.g[V1][0] as u32)
                                                                                .wrapping_add(0xf),
                                                                        );
                                                                        c.sdr(
                                                                            4,
                                                                            (c.g[V1][0] as u32)
                                                                                .wrapping_add(0x8),
                                                                        );
                                                                        c.sdl(
                                                                            7,
                                                                            (c.g[V1][0] as u32)
                                                                                .wrapping_add(0x17),
                                                                        );
                                                                        c.sdr(
                                                                            7,
                                                                            (c.g[V1][0] as u32)
                                                                                .wrapping_add(0x10),
                                                                        );
                                                                        c.g[V0][0] = c.lw((c.g[S0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1f4));
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[S0][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V0][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.sw(
                                                                            (c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x1fc,
                                                                                ),
                                                                            0u64,
                                                                        );
                                                                        c.g[V1][0] = c.lw((c.g[S0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1f4));
                                                                        c.g[V0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x34));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                << 2);
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V0][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.sw(
                                                                            c.g[V1][0] as u32,
                                                                            0u64,
                                                                        );
                                                                        c.g[V0][0] = 0x18;
                                                                        c.g[A3][0] = c.lw((c.g[S0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1f4));
                                                                        c.g[A0][0] = c.lw((c.g[S0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1f4));
                                                                        c.g[T0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x20));
                                                                        c.g[A3][0] =
                                                                            sx((c.g[A3][0] as u32)
                                                                                << 2);
                                                                        c.mult(0, 4, 4, 2);
                                                                        c.g[V0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x38));
                                                                        c.g[A3][0] =
                                                                            sx((c.g[A3][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V0][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.g[A0][0] =
                                                                            sx((c.g[A0][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[T0][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.g[V1][0] =
                                                                            c.lb(c.g[A0][0] as u32);
                                                                        c.sw(
                                                                            c.g[A3][0] as u32,
                                                                            c.g[V1][0],
                                                                        );
                                                                        c.g[V0][0] = c.lw((c.g[S0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1f4));
                                                                        c.g[V1][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x3c));
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V1][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.sw(
                                                                            c.g[V0][0] as u32,
                                                                            0u64,
                                                                        );
                                                                        c.g[A0][0] = c.lw((c.g[S0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1f4));
                                                                        c.g[A0][0] =
                                                                            sx((c.g[A0][0] as u32)
                                                                                << 3);
                                                                        c.g[A0][0] =
                                                                            sx((c.g[S1][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[A0][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.g[A0][0] =
                                                                            sx((c.g[A0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x40,
                                                                                ));
                                                                        callee::MEMSET
                                                                            .call(c, 0x002a5734)?;
                                                                        c.g[V1][0] = c.lw((c.g[S0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1f4));
                                                                        c.g[A0][0] = 0x64;
                                                                        c.g[V0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x60));
                                                                        c.g[A1][0] = 0x5a;
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                << 2);
                                                                        c.g[A2][0] = 0x32;
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V0][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.g[A3][0] = 0x7f;
                                                                        c.sw(
                                                                            c.g[V1][0] as u32,
                                                                            0u64,
                                                                        );
                                                                        c.g[V0][0] = c.lw((c.g[S0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1f4));
                                                                        c.g[V1][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x64));
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V1][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.sw(
                                                                            c.g[V0][0] as u32,
                                                                            0u64,
                                                                        );
                                                                        c.g[V1][0] = c.lw((c.g[S0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1f4));
                                                                        c.g[V0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x68));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                << 2);
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V0][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.sw(
                                                                            c.g[V1][0] as u32,
                                                                            0u64,
                                                                        );
                                                                        c.g[V0][0] = c.lw((c.g[S0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1f4));
                                                                        c.g[V1][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x6c));
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V1][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.sw(
                                                                            c.g[V0][0] as u32,
                                                                            0u64,
                                                                        );
                                                                        c.g[V1][0] = c.lw((c.g[S0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1f4));
                                                                        c.g[V0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x70));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                << 4);
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V0][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.sw(
                                                                            (c.g[V1][0] as u32)
                                                                                .wrapping_add(0x4),
                                                                            0u64,
                                                                        );
                                                                        c.g[V0][0] = c.lw((c.g[S0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1f4));
                                                                        c.g[V1][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x70));
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                << 4);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V1][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.sw(
                                                                            c.g[V0][0] as u32,
                                                                            c.g[A0][0],
                                                                        );
                                                                        c.g[A0][0] = 0x1;
                                                                        c.g[V1][0] = c.lw((c.g[S0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1f4));
                                                                        c.g[V0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x70));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                << 4);
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V0][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.sw(
                                                                            (c.g[V1][0] as u32)
                                                                                .wrapping_add(0x8),
                                                                            c.g[A1][0],
                                                                        );
                                                                        c.g[V0][0] = c.lw((c.g[S0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1f4));
                                                                        c.g[V1][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x70));
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                << 4);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V1][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.sw(
                                                                            (c.g[V0][0] as u32)
                                                                                .wrapping_add(0xc),
                                                                            c.g[A2][0],
                                                                        );
                                                                        c.g[V1][0] = c.lw((c.g[S0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1f4));
                                                                        c.g[V0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x74));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                << 2);
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V0][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.sw(
                                                                            c.g[V1][0] as u32,
                                                                            0u64,
                                                                        );
                                                                        c.g[V0][0] = c.lw((c.g[S0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1f4));
                                                                        c.g[V1][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x80));
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V1][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.sw(
                                                                            c.g[V0][0] as u32,
                                                                            0u64,
                                                                        );
                                                                        c.g[V1][0] = c.lw((c.g[S0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1f4));
                                                                        c.g[V0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x84));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                << 2);
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V0][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.sw(
                                                                            c.g[V1][0] as u32,
                                                                            0u64,
                                                                        );
                                                                        c.g[V0][0] = c.lw((c.g[S0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1f4));
                                                                        c.g[V1][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x78));
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V1][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.sw(
                                                                            c.g[V0][0] as u32,
                                                                            c.g[A3][0],
                                                                        );
                                                                        c.g[V1][0] = c.lw((c.g[S0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1f4));
                                                                        c.g[V0][0] = c.lw((c.g[S1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x7c));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                << 2);
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V0][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.sw(
                                                                            c.g[V1][0] as u32,
                                                                            c.g[A0][0],
                                                                        );
                                                                        c.g[A0][0] =
                                                                            c.lw(c.g[S2][0] as u32);
                                                                        c.g[V1][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0x4));
                                                                        c.g[V0][0] = c.lw((c.g[A0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1f4));
                                                                        c.g[A1][0] = c.lw((c.g[SP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x84));
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V1][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.sw(
                                                                            c.g[V0][0] as u32,
                                                                            c.g[A1][0],
                                                                        );
                                                                        c.g[V1][0] =
                                                                            c.lw(c.g[S2][0] as u32);
                                                                        c.g[A0][0] = c
                                                                            .lw((c.g[S2][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        c.g[V0][0] = c.lw((c.g[V1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1f4));
                                                                        c.g[A2][0] = c.lw((c.g[SP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x88));
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[A0][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.sw(
                                                                            c.g[V0][0] as u32,
                                                                            c.g[A2][0],
                                                                        );
                                                                        c.g[A0][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x608));
                                                                        c.g[V1][0] =
                                                                            c.lw(c.g[A0][0] as u32);
                                                                        c.g[V0][0] = c.lw((c.g[V1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1f4));
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                << 2);
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V0][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.sw(
                                                                            (c.g[V1][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x1fc,
                                                                                ),
                                                                            c.g[S4][0],
                                                                        );
                                                                        c.g[V1][0] = c.lw((c.g[SP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x8c));
                                                                        c.g[V0][0] = c.lw((c.g[S6]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x20));
                                                                        c.g[A3][0] =
                                                                            c.lw(c.g[A0][0] as u32);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                << 2);
                                                                        c.g[A2][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V0][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.g[T0][0] =
                                                                            sx((c.g[A3][0] as u32)
                                                                                .wrapping_add(
                                                                                    0x1d8,
                                                                                ));
                                                                        c.g[V0][0] =
                                                                            sx((c.g[S3][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V0][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.g[V1][0] = c.lw((c.g[A3]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1f4));
                                                                        c.g[A0][0] =
                                                                            c.lw(c.g[V0][0] as u32);
                                                                        c.g[A1][0] =
                                                                            c.lw(c.g[A2][0] as u32);
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] = c.lw((c.g[T0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x38));
                                                                        c.g[A0][0] =
                                                                            sx((c.g[A0][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[A1][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[V0][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.g[A1][0] = 0x18;
                                                                        c.sw(
                                                                            c.g[V1][0] as u32,
                                                                            c.g[A0][0],
                                                                        );
                                                                        c.g[A0][0] = c.lw((c.g[A3]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1f4));
                                                                        c.g[V1][0] = c.lw((c.g[T0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        c.g[A2][0] = c.lw((c.g[T0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x38));
                                                                        c.mult(0, 4, 4, 5);
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                << 2);
                                                                        c.g[A1][0] = c.lw((c.g[T0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x20));
                                                                        c.g[V1][0] =
                                                                            sx((c.g[V1][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[A2][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.g[V0][0] =
                                                                            c.lw(c.g[V1][0] as u32);
                                                                        c.g[V0][0] = ((c.g[V0][0]
                                                                            as i64)
                                                                            < 127i64)
                                                                            as u64;
                                                                        let t = c.g[V0][0] == 0u64;
                                                                        c.g[A0][0] =
                                                                            sx((c.g[A0][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[A1][0]
                                                                                        as u32,
                                                                                ));
                                                                        if t {
                                                                            break 'b_2a5920;
                                                                        } else {
                                                                            c.cycles += 6;
                                                                            c.g[V0][0] = c.lw((c.g
                                                                                [T0][0]
                                                                                as u32)
                                                                                .wrapping_add(
                                                                                    0x1c,
                                                                                ));
                                                                            c.g[V0][0] =
                                                                                sx((c.g[V0][0]
                                                                                    as u32)
                                                                                    << 2);
                                                                            c.g[V0][0] =
                                                                                sx((c.g[V0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        c.g[A2][0]
                                                                                            as u32,
                                                                                    ));
                                                                            c.g[V1][0] = c
                                                                                .lw(c.g[V0][0]
                                                                                    as u32);
                                                                            let t = (c.g[V1][0]
                                                                                as i64)
                                                                                < 0;
                                                                            c.g[V0][0] = 0u64;
                                                                            if t {
                                                                                break 'b_2a594c;
                                                                            } else {
                                                                                break 'b_2a5920;
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 7;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[T0][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    c.g[V0][0] = sx((c.g[V0][0]
                                                                        as u32)
                                                                        << 2);
                                                                    c.g[V0][0] = sx((c.g[V0][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            c.g[A2][0] as u32,
                                                                        ));
                                                                    c.g[V1][0] =
                                                                        c.lw(c.g[V0][0] as u32);
                                                                    c.g[V1][0] = ((c.g[V1][0]
                                                                        as i64)
                                                                        < 127i64)
                                                                        as u64;
                                                                    if c.g[V1][0] == 0u64 {
                                                                        c.g[V0][0] = 0x7f;
                                                                        break 'b_2a594c;
                                                                    } else {
                                                                        c.cycles += 4;
                                                                        c.g[V0][0] = c.lw((c.g[T0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[A2][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.g[V0][0] = c
                                                                            .lbu(c.g[V0][0] as u32);
                                                                        break 'b_2a594c;
                                                                    }
                                                                }
                                                                c.cycles += 59;
                                                                c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                                                                c.g[A3][0] = 0x1;
                                                                c.g[A2][0] = 0x4a0000;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_add(0x608));
                                                                c.g[A1][0] = c
                                                                    .lw((c.g[S7][0] as u32)
                                                                        .wrapping_add(0x17c));
                                                                c.g[V1][0] =
                                                                    c.lw(c.g[A0][0] as u32);
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[V1][0] as u32)
                                                                        .wrapping_add(0x1f4));
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[V1][0] as u32)
                                                                        .wrapping_add(0x214));
                                                                c.g[V0][0] =
                                                                    sx((c.g[V0][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[A0][0] as u32,
                                                                    ));
                                                                c.sw(c.g[V0][0] as u32, c.g[A1][0]);
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_add(0x608));
                                                                c.g[A0][0] =
                                                                    c.lw(c.g[V1][0] as u32);
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[A0][0] as u32)
                                                                        .wrapping_add(0x1f4));
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[A0][0] as u32)
                                                                        .wrapping_add(0x24c));
                                                                c.g[V0][0] =
                                                                    sx((c.g[V0][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[V1][0] as u32,
                                                                    ));
                                                                c.sw(c.g[V0][0] as u32, c.g[A3][0]);
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_add(0x608));
                                                                c.g[A1][0] =
                                                                    c.lw(c.g[S7][0] as u32);
                                                                c.g[V1][0] =
                                                                    c.lw(c.g[A0][0] as u32);
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[V1][0] as u32)
                                                                        .wrapping_add(0x1f4));
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[V1][0] as u32)
                                                                        .wrapping_add(0x23c));
                                                                c.g[V0][0] =
                                                                    sx((c.g[V0][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[A0][0] as u32,
                                                                    ));
                                                                c.sw(c.g[V0][0] as u32, c.g[A1][0]);
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_add(0x608));
                                                                c.g[A1][0] =
                                                                    c.lw(c.g[V1][0] as u32);
                                                                c.g[A0][0] = sx((c.g[A2][0]
                                                                    as u32)
                                                                    .wrapping_add(0x3700));
                                                                c.ldl(
                                                                    8,
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x7),
                                                                );
                                                                c.ldr(8, c.g[A0][0] as u32);
                                                                c.sdl(
                                                                    8,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x67),
                                                                );
                                                                c.sdr(
                                                                    8,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x60),
                                                                );
                                                                c.g[A0][0] = sx((c.g[A1][0]
                                                                    as u32)
                                                                    .wrapping_add(0x1d8));
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[A1][0] as u32)
                                                                        .wrapping_add(0x1f4));
                                                                c.g[V0][0] =
                                                                    sx((c.g[V0][0] as u32) << 3);
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[V0][0] as u32,
                                                                    ));
                                                                c.ldl(
                                                                    5,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x67),
                                                                );
                                                                c.ldr(
                                                                    5,
                                                                    (c.g[SP][0] as u32)
                                                                        .wrapping_add(0x60),
                                                                );
                                                                c.sdl(
                                                                    5,
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x47),
                                                                );
                                                                c.sdr(
                                                                    5,
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x40),
                                                                );
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_add(0x608));
                                                                c.g[A0][0] =
                                                                    c.lw(c.g[V1][0] as u32);
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[A0][0] as u32)
                                                                        .wrapping_add(0x1f4));
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[A0][0] as u32)
                                                                        .wrapping_add(0x238));
                                                                c.g[V0][0] =
                                                                    sx((c.g[V0][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[V1][0] as u32,
                                                                    ));
                                                                c.sw(c.g[V0][0] as u32, c.g[S7][0]);
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_add(0x608));
                                                                c.g[A0][0] =
                                                                    c.lw(c.g[V1][0] as u32);
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[A0][0] as u32)
                                                                        .wrapping_add(0x1f4));
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[A0][0] as u32)
                                                                        .wrapping_add(0x254));
                                                                c.g[V0][0] =
                                                                    sx((c.g[V0][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[V1][0] as u32,
                                                                    ));
                                                                c.sw(c.g[V0][0] as u32, 0u64);
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_add(0x608));
                                                                callee::PLAY_PUSHED_SOUND
                                                                    .call(c, 0x002a5a30)?;
                                                                c.sw(
                                                                    (c.g[S6][0] as u32)
                                                                        .wrapping_add(0x4),
                                                                    c.g[V0][0],
                                                                );
                                                                break 'b_2a5a58;
                                                            }
                                                        }
                                                        c.cycles += 8;
                                                        c.g[V1][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x8c));
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                        c.g[A1][0] = sx((c.g[V1][0] as u32)
                                                            .wrapping_add(c.g[V0][0] as u32));
                                                        c.g[V0][0] = sx((c.g[S3][0] as u32)
                                                            .wrapping_add(c.g[V0][0] as u32));
                                                        c.g[A0][0] = c.lw(c.g[A1][0] as u32);
                                                        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                                        c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                            .wrapping_add(c.g[A0][0] as u32));
                                                        c.sw(
                                                            (c.g[A2][0] as u32).wrapping_add(0x64),
                                                            c.g[V1][0],
                                                        );
                                                        break 'b_2a5a58;
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[S5][0] =
                                                    sx((c.g[S5][0] as u32).wrapping_add(0x1));
                                                break 'b_2a5a5c;
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = ((c.g[S5][0] as i64) < 3i64) as u64;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[V0][0] = 0x30;
                                        if t {
                                            if event_due(c) {
                                                deliver_event_here(c, 0x002a55f0);
                                            }
                                            continue 'l_2a55f0;
                                        } else {
                                            break 'b_2a5a68;
                                        }
                                    }
                                }
                                'b_2a5c98: {
                                    'b_2a5c3c: {
                                        'b_2a5c38: {
                                            c.cycles += 5;
                                            c.g[V0][0] = 0x1000;
                                            c.g[V1][0] =
                                                c.lw((c.g[S7][0] as u32).wrapping_add(0x180));
                                            c.sw(
                                                (c.g[SP][0] as u32).wrapping_add(0x74),
                                                c.g[V0][0],
                                            );
                                            let t = c.g[V1][0] == 0u64;
                                            c.sw(
                                                (c.g[SP][0] as u32).wrapping_add(0x70),
                                                c.g[V0][0],
                                            );
                                            if t {
                                                break 'b_2a5c38;
                                            } else {
                                                'b_2a5ab4: {
                                                    c.cycles += 14;
                                                    c.g[T0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_add(0x608)
                                                        );
                                                    c.g[A2][0] =
                                                        sx((c.g[SP][0] as u32).wrapping_add(0x70));
                                                    c.g[A1][0] = c.lw(c.g[S7][0] as u32);
                                                    c.g[A3][0] =
                                                        sx((c.g[SP][0] as u32).wrapping_add(0x74));
                                                    c.g[V1][0] =
                                                        c.lw((c.g[T0][0] as u32).wrapping_add(0xc));
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V1][0] as u32).wrapping_add(0x10));
                                                    c.g[V0][0] = c
                                                        .lw((c.g[V1][0] as u32).wrapping_add(0x14));
                                                    let t = c.g[V0][0] as u32;
                                                    c.g[A0][0] = sx((c.g[T0][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    call_ptr(c, t, 0x002a5aa0)?;
                                                    c.g[S0][0] = c.g[V0][0];
                                                    c.g[V0][0] =
                                                        sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                                                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                                                    if c.g[V0][0] != 0u64 {
                                                        c.sw(
                                                            (c.g[S7][0] as u32).wrapping_add(0x18c),
                                                            c.g[S0][0],
                                                        );
                                                        break 'b_2a5ab4;
                                                    } else {
                                                        break 'b_2a5ab4;
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S7][0] as u32).wrapping_add(0x180));
                                                c.g[V0][0] = 0x1;
                                                if c.g[V1][0] != c.g[V0][0] {
                                                    c.g[A1][0] =
                                                        c.lw(
                                                            (c.g[S7][0] as u32).wrapping_add(0x174)
                                                        );
                                                    break 'b_2a5c3c;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = 0x3;
                                                    if c.g[S0][0] == c.g[V0][0] {
                                                        c.g[A1][0] = c
                                                            .lw((c.g[S7][0] as u32)
                                                                .wrapping_add(0x174));
                                                        break 'b_2a5c3c;
                                                    } else {
                                                        c.cycles += 10;
                                                        c.g[A0][0] = c.g[S7][0];
                                                        callee::STOP_SOUND.call(c, 0x002a5ad8)?;
                                                        c.g[A0][0] = 0x54;
                                                        c.g[V0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_add(0x608));
                                                        c.mult(0, 4, 16, 4);
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_add(c.g[V0][0] as u32));
                                                        c.g[A0][0] =
                                                            sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x4c));
                                                        callee::PICK_SAMPLE.call(c, 0x002a5af0)?;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A2][0] = c.g[V0][0];
                                                        if t {
                                                            break 'b_2a5c38;
                                                        } else {
                                                            'b_2a5bbc: {
                                                                'b_2a5b90: {
                                                                    c.cycles += 32;
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[GP][0] as u32)
                                                                            .wrapping_add(0x608));
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[S7][0] as u32)
                                                                            .wrapping_add(0xd8));
                                                                    callee::PUSH_PLAY_PARAMS
                                                                        .call(c, 0x002a5b04)?;
                                                                    c.g[A2][0] = c
                                                                        .lw((c.g[GP][0] as u32)
                                                                            .wrapping_add(0x608));
                                                                    c.g[A1][0] = sx((c.g[S7][0]
                                                                        as u32)
                                                                        .wrapping_add(0x10));
                                                                    c.g[A3][0] = 0x18;
                                                                    c.g[V1][0] =
                                                                        c.lw(c.g[A2][0] as u32);
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[V1][0] as u32)
                                                                            .wrapping_add(0x1f4));
                                                                    c.g[V0][0] = sx((c.g[V0][0]
                                                                        as u32)
                                                                        << 2);
                                                                    c.g[V1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            c.g[V0][0] as u32,
                                                                        ));
                                                                    c.sw(
                                                                        (c.g[V1][0] as u32)
                                                                            .wrapping_add(0x1fc),
                                                                        c.g[A1][0],
                                                                    );
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[SP][0] as u32)
                                                                            .wrapping_add(0x74));
                                                                    c.g[A1][0] =
                                                                        c.lw(c.g[A2][0] as u32);
                                                                    c.mult(0, 4, 30, 4);
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[A1][0] as u32)
                                                                            .wrapping_add(0x1f4));
                                                                    c.g[A2][0] = sx((c.g[A1][0]
                                                                        as u32)
                                                                        .wrapping_add(0x1d8));
                                                                    c.g[V1][0] = c
                                                                        .lw((c.g[A2][0] as u32)
                                                                            .wrapping_add(0x38));
                                                                    c.g[V0][0] = sx((c.g[V0][0]
                                                                        as u32)
                                                                        << 2);
                                                                    c.g[V0][0] = sx((c.g[V0][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            c.g[V1][0] as u32,
                                                                        ));
                                                                    c.g[A0][0] = sx(((c.g[A0][0]
                                                                        as u32)
                                                                        as i32
                                                                        >> 12)
                                                                        as u32);
                                                                    c.sw(
                                                                        c.g[V0][0] as u32,
                                                                        c.g[A0][0],
                                                                    );
                                                                    c.g[A0][0] = c
                                                                        .lw((c.g[A1][0] as u32)
                                                                            .wrapping_add(0x1f4));
                                                                    c.g[V1][0] = c
                                                                        .lw((c.g[A2][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    c.g[T0][0] = c
                                                                        .lw((c.g[A2][0] as u32)
                                                                            .wrapping_add(0x38));
                                                                    c.mult(0, 4, 4, 7);
                                                                    c.g[V1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        << 2);
                                                                    c.g[A1][0] = c
                                                                        .lw((c.g[A2][0] as u32)
                                                                            .wrapping_add(0x20));
                                                                    c.g[V1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            c.g[T0][0] as u32,
                                                                        ));
                                                                    c.g[V0][0] =
                                                                        c.lw(c.g[V1][0] as u32);
                                                                    c.g[V0][0] = ((c.g[V0][0]
                                                                        as i64)
                                                                        < 127i64)
                                                                        as u64;
                                                                    let t = c.g[V0][0] == 0u64;
                                                                    c.g[A0][0] = sx((c.g[A0][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            c.g[A1][0] as u32,
                                                                        ));
                                                                    if t {
                                                                        break 'b_2a5b90;
                                                                    } else {
                                                                        c.cycles += 6;
                                                                        c.g[V0][0] = c.lw((c.g[A2]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x1c));
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                << 2);
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_add(
                                                                                    c.g[T0][0]
                                                                                        as u32,
                                                                                ));
                                                                        c.g[V1][0] =
                                                                            c.lw(c.g[V0][0] as u32);
                                                                        let t =
                                                                            (c.g[V1][0] as i64) < 0;
                                                                        c.g[V0][0] = 0u64;
                                                                        if t {
                                                                            break 'b_2a5bbc;
                                                                        } else {
                                                                            break 'b_2a5b90;
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 7;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[A2][0] as u32)
                                                                        .wrapping_add(0x1c));
                                                                c.g[V0][0] =
                                                                    sx((c.g[V0][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        c.g[T0][0] as u32,
                                                                    ));
                                                                c.g[V1][0] =
                                                                    c.lw(c.g[V0][0] as u32);
                                                                c.g[V1][0] = ((c.g[V1][0] as i64)
                                                                    < 127i64)
                                                                    as u64;
                                                                if c.g[V1][0] == 0u64 {
                                                                    c.g[V0][0] = 0x7f;
                                                                    break 'b_2a5bbc;
                                                                } else {
                                                                    c.cycles += 4;
                                                                    c.g[V0][0] = c
                                                                        .lw((c.g[A2][0] as u32)
                                                                            .wrapping_add(0x1c));
                                                                    c.g[V0][0] = sx((c.g[V0][0]
                                                                        as u32)
                                                                        << 2);
                                                                    c.g[V0][0] = sx((c.g[V0][0]
                                                                        as u32)
                                                                        .wrapping_add(
                                                                            c.g[T0][0] as u32,
                                                                        ));
                                                                    c.g[V0][0] =
                                                                        c.lbu(c.g[V0][0] as u32);
                                                                    break 'b_2a5bbc;
                                                                }
                                                            }
                                                            c.cycles += 29;
                                                            c.sb(c.g[A0][0] as u32, c.g[V0][0]);
                                                            c.g[A2][0] = 0x1;
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_add(0x608));
                                                            c.g[A1][0] = c.lw((c.g[S7][0] as u32)
                                                                .wrapping_add(0x17c));
                                                            c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x1f4));
                                                            c.g[A0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x214));
                                                            c.g[V0][0] =
                                                                sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(c.g[A0][0] as u32));
                                                            c.sw(c.g[V0][0] as u32, c.g[A1][0]);
                                                            c.g[V1][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_add(0x608));
                                                            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x1f4));
                                                            c.g[V1][0] = c.lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x24c));
                                                            c.g[V0][0] =
                                                                sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(c.g[V1][0] as u32));
                                                            c.sw(c.g[V0][0] as u32, c.g[A2][0]);
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_add(0x608));
                                                            c.g[A1][0] = c.lw(c.g[S7][0] as u32);
                                                            c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x1f4));
                                                            c.g[A0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x23c));
                                                            c.g[V0][0] =
                                                                sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(c.g[A0][0] as u32));
                                                            c.sw(c.g[V0][0] as u32, c.g[A1][0]);
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_add(0x608));
                                                            callee::MIDI_PLAY
                                                                .call(c, 0x002a5c28)?;
                                                            let t = (c.g[V0][0] as i64) < 0;
                                                            c.sw(
                                                                (c.g[S7][0] as u32)
                                                                    .wrapping_add(0x174),
                                                                c.g[V0][0],
                                                            );
                                                            if t {
                                                                break 'b_2a5c98;
                                                            } else {
                                                                c.cycles += 2;
                                                                c.g[V0][0] = 0x2;
                                                                c.sw(
                                                                    (c.g[S7][0] as u32)
                                                                        .wrapping_add(0x180),
                                                                    c.g[V0][0],
                                                                );
                                                                break 'b_2a5c38;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[A1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x174));
                                        break 'b_2a5c3c;
                                    }
                                    c.cycles += 2;
                                    let t = (c.g[A1][0] as i64) < 0;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                    if t {
                                        break 'b_2a5c98;
                                    } else {
                                        'b_2a5c70: {
                                            c.cycles += 7;
                                            c.g[A2][0] = 0x2;
                                            c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                                            callee::FIND_INSTANCE.call(c, 0x002a5c54)?;
                                            c.g[S0][0] = c.g[V0][0];
                                            if c.g[S0][0] != 0u64 {
                                                c.g[V1][0] =
                                                    c.lw((c.g[S7][0] as u32).wrapping_add(0x180));
                                                break 'b_2a5c70;
                                            } else {
                                                c.cycles += 4;
                                                c.g[A0][0] = c.g[S7][0];
                                                callee::STOP_SOUND.call(c, 0x002a5c68)?;
                                                c.sw((c.g[S7][0] as u32).wrapping_add(0x18c), 0u64);
                                                break 'b_2a5c98;
                                            }
                                        }
                                        'b_2a5c94: {
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x2;
                                            let t = c.g[V1][0] != c.g[V0][0];
                                            c.g[A0][0] = c.g[FP][0];
                                            if t {
                                                break 'b_2a5c94;
                                            } else {
                                                c.cycles += 6;
                                                c.g[A1][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x70));
                                                c.g[A0][0] = c.g[S0][0];
                                                callee::SET_PITCH.call(c, 0x002a5c88)?;
                                                c.g[V0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x74));
                                                c.mult(0, 2, 30, 2);
                                                c.g[A0][0] =
                                                    sx(((c.g[V0][0] as u32) as i32 >> 12) as u32);
                                                break 'b_2a5c94;
                                            }
                                        }
                                        c.cycles += 1;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x64), c.g[A0][0]);
                                        break 'b_2a5c98;
                                    }
                                }
                                c.cycles += 3;
                                c.lq(2, c.g[SP][0] as u32);
                                c.sw((c.g[S7][0] as u32).wrapping_add(0x178), c.g[FP][0]);
                                c.sq(2, (c.g[S7][0] as u32).wrapping_add(0x10));
                                break 'b_2a5ca4;
                            }
                        }
                    }
                }
            }
        }
    }
    c.cycles += 12;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x120));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x110));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x100));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0xf0));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0xe0));
    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0xd0));
    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0xc0));
    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0xb0));
    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0xa0));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x90));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x130));
    c.pc = t;
    Ok(())
}

// ---------------------------------------------------------------------------
// Audio.

/// `CrowdAudio::InstanceRemoveAllSources(self, fade)` (0x2a6430): remove the
/// fifteen crowd-audio instance slots.
pub fn crowd_audio_instance_remove_all_sources(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x20));
        frame.save_fpr(c, 20, 0x40);
        c.g[S1][0] = 0;
        frame.save_ra(c, 0);
        c.mov_s(20, 12);
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
            let empty = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S2][0];
            if !empty {
                c.cycles += 3;
                c.g[A1][0] = c.g[S1][0];
                c.mov_s(12, 20);
                call_ptr(c, 0x002a_62f0, 0x002a_6470)?;
            }
            c.cycles += 4;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S1][0] as i64) < 15i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x002a_6458);
                }
            } else {
                c.cycles += 7;
                frame.restore(c, S0, 0x30);
                frame.restore(c, S1, 0x20);
                frame.restore(c, S2, 0x10);
                frame.restore_ra(c, 0);
                frame.restore_fpr(c, 20, 0x40);
                frame.close(c);
                return Ok(());
            }
        }
    })
}

// ---------------------------------------------------------------------------
// Ini parser, script engine, replay and codec.

/// `IniParser_SkipSectionHeaders(self)` (0x2a7a20): skip whitespace, then walk
/// past consecutive `[...]` section headers, ending at the section end.
pub fn ini_parser_skip_section_headers(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x002a_78f8, 0x002a_7a38)?;
        c.g[V1][0] = Addr(p(c.g[S0][0] as u32, 8)).lw(c);
        c.g[A0][0] = 0x5b;
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lb(c);
        let not_header = c.g[V0][0] != c.g[A0][0];
        c.g[S1][0] = 0x5b;
        if !not_header {
            c.cycles += 1;
            loop {
                c.cycles += 8;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002a_7b08, 0x002a_7a58)?;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002a_78f8, 0x002a_7a60)?;
                c.g[V1][0] = Addr(p(c.g[S0][0] as u32, 8)).lw(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lb(c);
                let another = c.g[V0][0] == c.g[S1][0];
                if !another {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002a_7a50);
                }
            }
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002a_7f90, 0x002a_7a78)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_StopTrickMeterSound(AudioSystem* self, void*
/// rider)` (0x29b3c0): stop the trick-meter handle when the rider is gone or
/// finished.
pub fn ssx_audio_system_stop_trick_meter_sound(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        let no_rider = c.g[A1][0] == 0;
        frame.save_ra(c, 0);
        let run;
        if no_rider {
            run = true;
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(at(c, A1, 0x874)).lw(c);
            let gone = c.g[V0][0] == 0;
            c.g[V1][0] = 0;
            if !gone {
                c.cycles += 2;
                c.g[V0][0] = Addr(at(c, A1, 0x87c)).lw(c);
                c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
            }
            c.cycles += 2;
            if c.g[V1][0] == 0 {
                frame.restore(c, S0, 0x20);
                c.cycles += 4;
                frame.restore(c, S1, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            run = true;
        }
        if run {
            c.cycles += 4;
            c.g[A1][0] = Addr(at(c, S0, 0x5fdc)).lw(c);
            c.g[S1][0] = u64::MAX;
            let same = c.g[A1][0] == c.g[S1][0];
            c.g[A2][0] = 1;
            if !same {
                c.cycles += 6;
                c.g[V0][0] = Addr(at(c, S0, 0x118)).lw(c);
                c.mtc1(0, 12);
                c.g[A0][0] = Addr(at(c, V0, 0)).lw(c);
                c.g[A0][0] = sx(at(c, A0, 0x1d8));
                call_ptr(c, 0x002a_d5f0, 0x0029_b418)?;
                Addr(at(c, S0, 0x5fdc)).sw(c, c.g[S1][0]);
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SSXAudioSystem_IsTrackGroup1Event(AudioSystem* self, void* info)`
/// (0x2a40e8): whether the current track is in group 1, or the sound's
/// configured event is the group-1 event flag.
pub fn ssx_audio_system_is_track_group1_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x002a_40fc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x002a_4108)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_4bc0, 0x002a_4110)?;
        c.g[A1][0] = Addr(at(c, V0, 0)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002a_4158, 0x002a_411c)?;
        let group1 = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        let mut result = 1u64;
        if !group1 {
            c.cycles += 5;
            c.g[V0][0] = 0x530000;
            c.g[A0][0] = 1;
            c.g[V1][0] = Addr(at(c, V0, 0x5c10)).lb(c);
            let is_one = c.g[V1][0] == c.g[A0][0];
            c.g[V1][0] = c.g[V1][0] ^ 6;
            if is_one {
                c.cycles += 1;
                result = 1;
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0;
                if c.g[V1][0] == 0 {
                    c.g[V0][0] = c.g[A0][0];
                }
                result = c.g[V0][0];
            }
        }
        c.g[V0][0] = result;
        frame.restore(c, S0, 0x10);
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CrowdAudio_FreeSampleList(SampleList* list)` (0x2a4cf8): free every
/// sample pointer in the list and clear it.
pub fn crowd_audio_free_sample_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(at(c, S2, 0)).lw(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !empty {
            c.cycles += 1;
            c.g[S0][0] = sx(at(c, S2, 4));
            loop {
                c.cycles += 3;
                c.g[A0][0] = Addr(at(c, S0, 0)).lw(c);
                if c.g[A0][0] == 0 {
                    Addr(at(c, S0, 0)).sw(c, 0);
                } else {
                    c.cycles += 3;
                    call_ptr(c, 0x0031_7e98, 0x002a_4d34)?;
                    Addr(at(c, S0, 0)).sw(c, 0);
                }
                c.cycles += 5;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V0][0] = Addr(at(c, S2, 0)).lw(c);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002a_4d20);
                }
            }
        }
        c.cycles += 7;
        Addr(at(c, S2, 0)).sw(c, 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int IniParser_FindSection(IniParser* self, const char* name)` (0x2a7bf8):
/// walk the parser's sections and return whether one matches `name`
/// (case-insensitively). Its loop head is a resume entry after an event.
pub fn ini_parser_find_section(c: &mut Cpu) {
    let frame = Frame { size: 0x30 };
    guest(c, |c| {
        #[derive(Clone, Copy)]
        enum Stage {
            Head,
            Check,
            Get,
            Cmp,
        }
        let mut st = match c.pc {
            0x002a_7bf8 => {
                c.cycles += 7;
                c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
                c.sq(16, frame.at(c, 0x20));
                c.sq(17, frame.at(c, 0x10));
                c.g[S0][0] = c.g[A0][0];
                frame.save_ra(c, 0);
                c.g[S1][0] = c.g[A1][0];
                Stage::Head
            }
            0x002a_7c14 => {
                c.cycles += 1;
                Stage::Cmp
            }
            0x002a_7c18 => Stage::Cmp,
            _ => {
                c.jump_out(c.pc, 0x002a_7bf8, 0x002a_7c68);
                return Ok(());
            }
        };
        let result: u64;
        loop {
            match st {
                Stage::Head => {
                    c.cycles += 1;
                    c.g[V0][0] = Addr(at(c, S0, 8)).lw(c);
                    st = Stage::Check;
                }
                Stage::Check => {
                    c.cycles += 3;
                    c.g[V1][0] = Addr(at(c, V0, 0)).lb(c);
                    let empty = c.g[V1][0] == 0;
                    c.g[V0][0] = 0;
                    if empty {
                        result = 0;
                        break;
                    }
                    st = Stage::Get;
                }
                Stage::Get => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x002a_7b08, 0x002a_7c48)?;
                    if c.g[V0][0] == 0 {
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                        result = 0;
                        break;
                    }
                    c.g[A0][0] = c.g[V0][0];
                    if event_due(c) && back_edge_unwinds(c, 0x002a_7c18) {
                        return Ok(());
                    }
                    st = Stage::Cmp;
                }
                Stage::Cmp => {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0041_aa88, 0x002a_7c20)?;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = Addr(at(c, S0, 8)).lw(c);
                        st = Stage::Check;
                    } else {
                        c.cycles += 2;
                        result = 1;
                        break;
                    }
                }
            }
        }
        c.g[V0][0] = result;
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Audio.

/// `SSXAudioSystem_RestartAllBoardLoops` (0x2929D8): stop every board loop and
/// restart one per rider in the race manager, then clear the "loops running"
/// flag. Dispatcher-shaped: a delivered event resumes at 0x292A00.
pub fn ssx_audio_system_restart_all_board_loops(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x0029_29d8 => {
                    c.cycles += 9;
                    c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x30));
                    c.sq(17, sp(c).wrapping_add(0x10));
                    c.sq(16, sp(c).wrapping_add(0x20));
                    c.g[S1][0] = c.g[A0][0];
                    c.sd(sp(c), c.g[RA][0]);
                    c.g[S0][0] = 0;
                    callee::STOP_BOARD_LOOPS.call(c, 0x0029_29f4)?;
                    pc = 0x0029_2a20;
                }
                0x0029_29fc => {
                    c.cycles += 1;
                    pc = 0x0029_2a00;
                }
                0x0029_2a00 => {
                    c.cycles += 8;
                    callee::GET_RACE_MAN.call(c, 0x0029_2a08)?;
                    c.g[V1][0] = sx(r32(c, S0) << 2);
                    c.g[A0][0] = c.g[S1][0];
                    c.g[V0][0] = low(c, V0).wrapping_add(low(c, V1));
                    c.g[S0][0] = low(c, S0).wrapping_add(1);
                    c.g[A1][0] = Addr(r32(c, V0).wrapping_add(0x28)).lw(c);
                    callee::START_RIDER_BOARD_LOOPS.call(c, 0x0029_2a20)?;
                    pc = 0x0029_2a20;
                }
                0x0029_2a20 => {
                    c.cycles += 6;
                    callee::GET_RACE_MAN.call(c, 0x0029_2a28)?;
                    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x78)).lw(c);
                    c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    let more = c.g[V1][0] != 0;
                    c.g[RA][0] = Addr(sp(c)).ld(c);
                    if more {
                        if c.cycles >= c.next_event {
                            c.pc = 0x0029_2a00;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x0029_2a00;
                    } else {
                        pc = 0x0029_2a38;
                    }
                }
                0x0029_2a38 => {
                    c.cycles += 5;
                    sw_at(c, S1, 0x5830, 0);
                    c.lq(16, sp(c).wrapping_add(0x20));
                    c.lq(17, sp(c).wrapping_add(0x10));
                    let ret = c.g[RA][0] as u32;
                    c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x30));
                    c.pc = ret;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0029_29d8, 0x0029_2a4c);
                    return Ok(());
                }
            }
        }
    })
}

/// `SSXAudioSystem_RiderSay20B9Callback` (0x2A1388): a rider's "say" callback:
/// map the object's rider index to a race rider id and fire the line. Returns
/// 1.
pub fn ssx_audio_system_rider_say_20b9_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = lw_at_2(c, A1, 0x1c);
        let out_of_range = (c.g[S0][0] as i64) < 0;
        c.g[S1][0] = c.g[A0][0];
        let mut have_id = false;
        if !out_of_range {
            c.cycles += 6;
            callee::GET_RACE_MAN.call(c, 0x002a_13ac)?;
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x78)).lw(c);
            c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[V1][0] as i64)) as u64;
            if c.g[V1][0] != 0 {
                c.cycles += 5;
                callee::GET_RACE_MAN.call(c, 0x002a_13c4)?;
                c.g[V1][0] = sx(r32(c, S0) << 2);
                c.g[V0][0] = low(c, V0).wrapping_add(low(c, V1));
                have_id = true;
            }
        }
        if !have_id {
            c.cycles += 2;
            callee::GET_RACE_MAN.call(c, 0x002a_13d8)?;
        }
        c.cycles += 9;
        c.g[A1][0] = Addr(r32(c, V0).wrapping_add(0x28)).lw(c);
        c.g[A0][0] = c.g[S1][0];
        callee::RIDER_SAY_20B9.call(c, 0x002a_13e4)?;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = 1;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Audio and parser helpers.

/// `func_0029ED18` (0x29ed18): `SSXAudioSystem_StopRiderLoops`. Stops the
/// rider's wipeout loops and trick-meter sound, and when the bank monitor
/// count at `+0x59e4` is positive stops the handle at `+0x59e0` on the monitor
/// `*(this+0x118) + 0x1d8` (with the float argument 0 in f12), clearing the
/// count. Signature: `void stop_rider_loops(this)`.
pub fn ssx_audio_system_stop_rider_loops(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0029_ed2c);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0028_b1d8, 0x0029_ed2c)?;
        c.g[A1][0] = lw_r(c, V0, 0x28);
        c.g[RA][0] = sx(0x0029_ed38);
        c.g[A0][0] = c.g[S0][0];
        callee::AUDIO_STOP_WIPEOUT_LOOP.call(c, 0x0029_ed38)?;
        c.g[RA][0] = sx(0x0029_ed40);
        call_ptr(c, 0x0028_b1d8, 0x0029_ed40)?;
        c.g[A1][0] = lw_r(c, V0, 0x28);
        c.g[RA][0] = sx(0x0029_ed4c);
        c.g[A0][0] = c.g[S0][0];
        callee::AUDIO_STOP_WIPEOUT_LOOP2.call(c, 0x0029_ed4c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0029_ed58);
        c.g[A1][0] = 0;
        callee::AUDIO_STOP_TRICK_METER.call(c, 0x0029_ed58)?;
        c.g[V0][0] = lw_r(c, S0, 0x59e4);
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[A2][0] = 0x1;
        if !none {
            c.cycles += 7;
            c.g[V0][0] = lw_r(c, S0, 0x118);
            c.g[A1][0] = lw_r(c, S0, 0x59e0);
            c.g[A0][0] = Addr(low_2(c, V0)).lw(c);
            c.mtc1(0, 12);
            c.g[RA][0] = sx(0x0029_ed7c);
            c.g[A0][0] = sx(low_2(c, A0).wrapping_add(0x1d8));
            callee::BANK_MONITOR_STOP_HANDLE.call(c, 0x0029_ed7c)?;
            sw_r(c, S0, 0x59e4, 0);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `func_002A4C28` (0x2a4c28): `CrowdAudio_AddSampleFile`. Appends a sample
/// file: when fewer than five are registered (`this->+0`), copies/registers
/// `a1` with `a2` (`0x3e1908`), stores the handle at `this+4+4*n`, bumps the
/// count and returns 1 if the slot is non-null. Signature:
/// `int add(CrowdAudio* this, const char* file, int bank)`.
pub fn crowd_audio_add_sample_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.g[A1][0];
        c.g[V0][0] = lw_r(c, S0, 0);
        c.g[V0][0] = ((c.g[V0][0] as i64) < 5) as u64;
        let full = c.g[V0][0] == 0;
        c.g[A1][0] = c.g[A2][0];
        if full {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 13;
            c.g[RA][0] = sx(0x002a_4c54);
            call_ptr(c, 0x003e_1908, 0x002a_4c54)?;
            c.g[V1][0] = lw_r(c, S0, 0);
            c.g[A0][0] = sx(low_2(c, S0).wrapping_add(0x4));
            c.g[V1][0] = sx(low_2(c, V1) << 2);
            c.g[V1][0] = sx(low_2(c, A0).wrapping_add(low_2(c, V1)));
            sw_r(c, V1, 0, c.g[V0][0]);
            c.g[A1][0] = lw_r(c, S0, 0);
            c.g[V0][0] = sx(low_2(c, A1) << 2);
            c.g[A0][0] = sx(low_2(c, A0).wrapping_add(low_2(c, V0)));
            c.g[V1][0] = Addr(low_2(c, A0)).lw(c);
            let stored = c.g[V1][0] != 0;
            c.g[V1][0] = sx(low_2(c, A1).wrapping_add(0x1));
            if stored {
                c.cycles += 2;
                c.g[V0][0] = 0x1;
                sw_r(c, S0, 0, c.g[V1][0]);
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `func_002A7A90` (0x2a7a90): `IniParser_SkipToEol`. Advances the parser
/// (`IniParser_GetChar`) until the character at `this->+8` is a carriage
/// return (0xd), a newline (0xa) or NUL; returns 1 when the buffer ended, 0
/// when a line terminator was consumed. The loop's back edge carries an event
/// check. Signature: `int skip_to_eol(IniParser* this)`.
pub fn ini_parser_skip_to_eol(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = lw_r(c, S0, 0x8);
        c.g[V0][0] = Addr(low_2(c, V1)).lb(c);
        let empty = c.g[V0][0] == 0;
        c.g[S2][0] = 0xd;
        if empty {
            c.cycles += 1;
            c.g[V0][0] = 0x1;
        } else {
            c.cycles += 2;
            c.g[S1][0] = 0xa;
            loop {
                c.cycles += 3;
                c.g[V0][0] = Addr(low_2(c, V1)).lb(c);
                if c.g[V0][0] == c.g[S2][0] {
                    c.g[V0][0] = 0;
                    break;
                }
                c.cycles += 2;
                let newline = c.g[V0][0] == c.g[S1][0];
                c.g[V0][0] = 0;
                if newline {
                    break;
                }
                c.cycles += 6;
                c.g[RA][0] = sx(0x002a_7adc);
                c.g[A0][0] = c.g[S0][0];
                callee::INI_PARSER_GET_CHAR.call(c, 0x002a_7adc)?;
                c.g[V1][0] = lw_r(c, S0, 0x8);
                c.g[V0][0] = Addr(low_2(c, V1)).lb(c);
                let more = c.g[V0][0] != 0;
                if !more {
                    c.cycles += 1;
                    c.g[V0][0] = 0x1;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002a_7ac0);
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

/// `void SSXAudioSystem_StopAllObjectLoops(self)` (0x2980b0): stop the
/// looping handle of every active object slot (`0x30` bytes each, 0x5a0
/// bytes of slots).
pub fn ssxaudiosystem_stop_all_object_loops(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 17, 0x30);
        frame.save(c, 16, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 18, 0x20);
        c.g[S0][0] = c.g[S1][0];
        frame.save(c, 19, 0x10);
        c.g[S2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5a0));
        frame.save_ra(c, 0);
        c.g[S3][0] = 1;
        c.g[V0][0] = lw_at_3(c, p(r32(c, S0), 0x5a00));
        loop {
            c.cycles += 2;
            if c.g[V0][0] != c.g[S3][0] {
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
            } else {
                c.cycles += 9;
                c.g[V0][0] = lw_at_3(c, p(r32(c, S1), 0x118));
                c.g[A2][0] = 1;
                c.g[A1][0] = lw_at_3(c, p(r32(c, S0), 0x5a24));
                c.g[A0][0] = lw_at_3(c, c.g[V0][0] as u32);
                c.mtc1(0, 12);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                callee::BANK_MONITOR_STOP_HANDLE_2.call(c, 0x0029_8104)?;
                c.sw(p(r32(c, S0), 0x5a00), 0);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = lw_at_3(c, p(r32(c, S0), 0x5a00));
            if event_due(c) {
                deliver_event_here(c, 0x0029_80e0);
            }
        }
        c.cycles += 7;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_PlayHitBoostSound(self, rider)` (0x298d00): when the
/// rider has a boost sound, add its timer to the bank monitor.
pub fn ssxaudiosystem_play_hit_boost_sound(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_at_3(c, p(abi::r32(c, A1), 0x874));
        let none = c.g[V0][0] == 0;
        c.g[V1][0] = 0;
        if !none {
            c.cycles += 2;
            c.g[V0][0] = lw_at_3(c, p(abi::r32(c, A1), 0x87c));
            c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
        }
        c.cycles += 2;
        let silent = c.g[V1][0] == 0;
        c.g[S1][0] = 0x4a0000;
        if !silent {
            c.cycles += 17;
            c.g[V0][0] = lw_at_3(c, p(abi::r32(c, A0), 0x118));
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3678));
            c.g[A1][0] = 9;
            c.g[S0][0] = lw_at_3(c, c.g[V0][0] as u32);
            c.g[A2][0] = 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
            call_ptr(
                c,
                callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN_PTR.addr,
                0x0029_8d48,
            )?;
            c.ldl(6, p(c.g[S1][0] as u32, 7));
            c.ldr(6, c.g[S1][0] as u32);
            c.g[A0][0] = c.g[S0][0];
            c.g[A3][0] = c.g[V0][0];
            c.g[A1][0] = 0xfa;
            c.g[T0][0] = 0;
            c.g[T1][0] = 0x72;
            c.g[T2][0] = 0x7f;
            c.g[T3][0] = 0;
            callee::BANK_MONITOR_ADD_TIMER.call(c, 0x0029_8d70)?;
        }
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_DJSay20E6(self)` (0x2a27d8): when DJ speech is on, set
/// the speech event context and play the DJ line through the speech system.
pub fn ssxaudiosystem_djsay20e6(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SSX_AUDIO_SYSTEM_IS_DJ_SPEECH_ON.call(c, 0x002a_27ec)?;
        let off = c.g[V0][0] == 0;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5560));
        if !off {
            c.cycles += 10;
            c.mtc1(0, 12);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x20e6;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0xa;
            c.g[T1][0] = 0;
            c.g[T2][0] = 0;
            callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT.call(c, 0x002a_2814)?;
            if c.g[V0][0] == 0 {
                frame.restore(c, 16, 0x10);
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 12;
            c.g[A0][0] = c.g[S0][0];
            callee::SSX_AUDIO_SYSTEM_GET_LANGUAGE_MASK.call(c, 0x002a_2824)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = 1;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x20e6;
            call_ptr(c, callee::SPCH_MAKE_ID, 0x002a_2838)?;
            c.g[V1][0] = lw_at_3(c, gw(c, 0x173c));
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S0][0];
            c.g[A1][0] = 1;
            call_ptr(c, c.g[V1][0] as u32, 0x002a_284c)?;
        }
        c.cycles += 1;
        frame.restore(c, 16, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// INI parser.

pub(crate) const INI_FIND_ENTRY: u32 = 0x002a_7c68;

pub(crate) const INI_FIND_END: u32 = 0x002a_7cf0;

/// Resumed after an event at the section walk's back edge.
pub(crate) const INI_FIND_SECTION: u32 = 0x002a_7c88;

/// `int IniParser_FindSectionPrefix(IniParser* self, const char* prefix)`
/// (0x2a7c68): walk the sections and return 1 when one is a prefix of the
/// other's name.
pub fn ini_parser_find_section_prefix(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                INI_FIND_ENTRY => {
                    c.cycles += 8;
                    let frame = Frame::open(c, 0x40);
                    frame.save(c, S1, 0x20);
                    frame.save(c, S2, 0x10);
                    c.g[S1][0] = c.g[A1][0];
                    frame.save(c, S0, 0x30);
                    c.g[S2][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.cycles += 1;
                    c.g[V0][0] = Addr(p(r32(c, S2), 8)).lw(c);
                    pc = 0x002a_7cb4;
                }
                0x002a_7cb4 => {
                    c.cycles += 3;
                    c.g[V1][0] = Addr(r32(c, V0)).lb(c);
                    let empty = c.g[V1][0] == 0;
                    c.g[V0][0] = 0;
                    if empty {
                        pc = 0x002a_7cd8;
                    } else {
                        pc = 0x002a_7cc0;
                    }
                }
                0x002a_7cc0 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x002a_7cc8);
                    c.g[A0][0] = c.g[S2][0];
                    callee::INI_PARSER_NEXT_SECTION.call(c, 0x002a_7cc8)?;
                    c.g[S0][0] = c.g[V0][0];
                    if c.g[S0][0] != 0 {
                        c.g[A0][0] = c.g[S1][0];
                        if event_due(c) {
                            c.pc = INI_FIND_SECTION;
                            if c.event() {
                                return Err(crate::call::Unwound);
                            }
                        }
                        pc = INI_FIND_SECTION;
                    } else {
                        c.cycles += 1;
                        c.g[V0][0] = 0;
                        pc = 0x002a_7cd8;
                    }
                }
                INI_FIND_SECTION => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x002a_7c90);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0041_ad80, 0x002a_7c90)?;
                    let found = c.g[V0][0] == c.g[S1][0];
                    c.g[A0][0] = c.g[S0][0];
                    if found {
                        c.cycles += 2;
                        c.g[V0][0] = 0x1;
                        pc = 0x002a_7cd8;
                    } else {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x002a_7ca0);
                        c.g[A1][0] = c.g[S1][0];
                        call_ptr(c, 0x0041_ad80, 0x002a_7ca0)?;
                        if c.g[V0][0] != c.g[S0][0] {
                            c.g[V0][0] = Addr(p(r32(c, S2), 8)).lw(c);
                            pc = 0x002a_7cb4;
                        } else {
                            c.cycles += 2;
                            c.g[V0][0] = 0x1;
                            pc = 0x002a_7cd8;
                        }
                    }
                }
                0x002a_7cd8 => {
                    c.cycles += 6;
                    frame_restore_ini(c);
                    return Ok(());
                }
                other => {
                    c.jump_out(other, INI_FIND_ENTRY, INI_FIND_END);
                    return Ok(());
                }
            }
        }
    })
}

/// The shared epilogue of [`ini_parser_find_section_prefix`].
pub(crate) fn frame_restore_ini(c: &mut Cpu) {
    let frame = Frame { size: 0x40 };
    frame.restore(c, S0, 0x30);
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Crowd audio.

/// `void CrowdAudio_StopAll(CrowdAudio* self)` (0x2a7718): remove every source
/// from the per-instance source lists, then stop all of the crowd's sound
/// through its interface.
pub fn crowd_audio_stop_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(r32(c, S1), 0x14)).lw(c);
        c.g[S0][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 1;
            c.g[S2][0] = 0;
            loop {
                c.cycles += 9;
                c.g[A0][0] = Addr(p(r32(c, S1), 0x18)).lw(c);
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                c.mtc1(0, 12);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S2)));
                callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES_2.call(c, 0x002a_7754)?;
                c.g[V0][0] = Addr(p(r32(c, S1), 0x14)).lw(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(0x190));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002a_7740);
                }
            }
        }
        c.cycles += 16;
        c.g[A0][0] = Addr(r32(c, S1)).lw(c);
        c.g[V0][0] = 0x004a_0000;
        c.ldl(5, p(0x004a_0000, 0x3707));
        c.ldr(5, p(0x004a_0000, 0x3700));
        c.g[V1][0] = Addr(p(r32(c, A0), 0xab0)).lw(c);
        c.g[A0][0] = sx(p(r32(c, A0), 0x1d8));
        c.g[V0][0] = Addr(p(r32(c, V1), 0x18)).lh(c);
        c.g[A2][0] = Addr(p(r32(c, V1), 0x1c)).lw(c);
        let target = r32(c, A2);
        c.g[A0][0] = sx(p(r32(c, A0), r32(c, V0)));
        call_ptr(c, target, 0x002a_778c)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_StartRiderBoardLoops(self, rider)` (0x292a50): cache
/// the rider's main-state and surface type and start the three board loops.
pub fn ssx_audio_system_start_rider_board_loops(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, table_callee::RIDER_GET_MAIN_STATE, 0x0029_2a78)?;
        c.g[S3][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S0][0];
        callee::SSX_AUDIO_GET_SURFACE_SOUND_TYPE.call(c, 0x0029_2a88)?;
        c.g[S1][0] = c.g[V0][0];
        Addr(c.g[S0][0] as u32).off(0x764).sw(c, c.g[S3][0]);
        Addr(c.g[S0][0] as u32).off(0x760).sw(c, c.g[S3][0]);
        c.g[A0][0] = c.g[S2][0];
        Addr(c.g[S0][0] as u32).off(0x768).sw(c, c.g[S1][0]);
        c.g[A1][0] = c.g[S0][0];
        Addr(c.g[S0][0] as u32).off(0x76c).sw(c, c.g[S1][0]);
        callee::SSX_AUDIO_START_BOARD_BASE_LOOP.call(c, 0x0029_2aa8)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        callee::SSX_AUDIO_START_BOARD_SURFACE_LOOP.call(c, 0x0029_2ab8)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        callee::SSX_AUDIO_START_BOARD_EDGE_LOOP.call(c, 0x0029_2ac8)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_OnMissionAbort(self, keep)` (0x29dbb0): if the mission
/// run is live, stop the wipeout/trick sounds, send event 0x27 and clear the
/// mission flags.
pub fn ssx_audio_system_on_mission_abort(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw_2(c, S0, 0x5fd4);
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            if c.g[A1][0] == 0 {
                c.cycles += 2;
                c.g[A1][0] = 0xe;
                callee::SSX_AUDIO_PLAY_FE_SOUND.call(c, 0x0029_dbdc)?;
            }
            c.cycles += 23;
            call_ptr(c, table_callee::SSX_AUDIO_GET_RACE_MAN, 0x0029_dbe4)?;
            c.g[A1][0] = lw_2(c, V0, 0x28);
            c.g[A0][0] = c.g[S0][0];
            callee::SSX_AUDIO_STOP_WIPEOUT_LOOP.call(c, 0x0029_dbf0)?;
            call_ptr(c, table_callee::SSX_AUDIO_GET_RACE_MAN, 0x0029_dbf8)?;
            c.g[A1][0] = lw_2(c, V0, 0x28);
            c.g[A0][0] = c.g[S0][0];
            callee::SSX_AUDIO_STOP_WIPEOUT_LOOP2.call(c, 0x0029_dc04)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            callee::SSX_AUDIO_STOP_TRICK_METER_SOUND.call(c, 0x0029_dc10)?;
            c.g[V1][0] = lw_2(c, S0, 0x5558);
            c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x118));
            c.g[A1][0] = 0x27;
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x20).lh(c);
            c.g[V0][0] = lw_2(c, V1, 0x24);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V0][0] as u32, 0x0029_dc2c)?;
            Addr(c.g[S0][0] as u32).off(0x5fd4).sw(c, 0);
            Addr(c.g[S0][0] as u32).off(0x5fd0).sw(c, 0);
            frame.restore(c, S0, 0x10);
        } else {
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Crowd audio.

/// `int CrowdAudio_AddSource(CrowdAudio* self, void* source)` (0x2a7678): add
/// `source` to the crowd instance at `self+0` (a 0x190-byte source description
/// appended at `+0x1d8`), returning the source bit or -1, then decrement the
/// instance's pending count at `+0x1f4`.
pub fn crowd_audio_add_source(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[T0][0] = lw(c, p(c.g[S0][0] as u32, 0x18));
        if c.g[T0][0] != 0 {
            c.g[V1][0] = lw(c, c.g[S0][0] as u32);
            c.cycles += 14;
            c.g[A3][0] = 0x190;
            c.g[A2][0] = c.g[A1][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1d8));
            c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, 0x1c));
            c.g[A1][0] = c.g[V1][0];
            c.g[A0][0] = lw(c, p(c.g[V1][0] as u32, 0x64));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[A0][0] = lw(c, c.g[V0][0] as u32);
            c.mult(0, 2, 4, 7);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T0][0] as u32));
            call_ptr(c, 0x002a_5d08, 0x002a_76cc)?; // CrowdAudio_InstanceAddSource
            let failed = (c.g[V0][0] as i64) < 0;
            c.g[A1][0] = sx((c.g[V0][0] as u32) << 8);
            if !failed {
                c.cycles += 8;
                c.g[V1][0] = lw(c, c.g[S0][0] as u32);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1d8));
                c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, 0x1c));
                c.g[A0][0] = lw(c, p(c.g[V1][0] as u32, 0x64));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = lw(c, c.g[V0][0] as u32);
                c.g[V0][0] = c.g[A1][0] | c.g[V1][0];
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        }
        c.cycles += 4;
        c.g[A0][0] = lw(c, c.g[S0][0] as u32);
        c.g[V1][0] = lw(c, p(c.g[A0][0] as u32, 0x1f4));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        sw(c, p(c.g[A0][0] as u32, 0x1f4), c.g[V1][0]);
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SSXAudioSystem_RecordRaceResult(self)` (0x2a45c0): if the race is
/// still the current result slot, reset it; then store the track event result
/// for the current rider.
pub fn ssx_audio_system_record_race_result(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        let this = abi::arg(c, 0);
        c.g[S1][0] = sx(this);
        frame.save(c, S2, 0x10);
        c.g[A0][0] = 1;
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_3(c, A1, 0xc0);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        let not_current = c.g[V1][0] != c.g[A0][0];
        c.g[V0][0] = 0x0053_0000;
        if !not_current {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0;
            callee::AUDIO_RESET_RACE_RESULT_STATE.call(c, 0x002a_45fc)?;
            c.g[V0][0] = 0x0053_0000;
        }
        c.cycles += 7;
        c.g[S2][0] = Addr(c.g[V0][0] as u32).off(0x6730).lw(c);
        call_ptr(c, table::AUDIO_GET_RACE_MAN, 0x002a_4608)?;
        c.g[S0][0] = lw_3(c, V0, 0x28);
        c.g[A0][0] = c.g[S1][0];
        callee::AUDIO_IS_TRACK_GROUP0_EVENT.call(c, 0x002a_4614)?;
        let event = c.g[V0][0] != 0;
        if !event {
            c.g[V1][0] = lw_3(c, S0, 0x790);
            c.cycles += 1;
            c.g[A2][0] = 0;
        } else {
            c.cycles += 4;
            c.g[V0][0] = lw_3(c, S0, 0x790);
            c.g[V1][0] = c.g[V0][0];
            c.g[A2][0] = lw_3(c, V0, 0x128);
        }
        c.cycles += 12;
        c.g[T0][0] = lw_3(c, S0, 0x480);
        c.g[A0][0] = c.g[S1][0];
        c.g[A3][0] = lw_3(c, V1, 0x114);
        c.g[A1][0] = c.g[S2][0];
        c.g[T0][0] ^= 1;
        c.g[RA][0] = sx(0x002a_4648);
        c.pc = 0x002a_4660;
        (callee::AUDIO_STORE_RACE_RESULT.f)(c);
        if c.pc != 0x002a_4648 {
            return Err(Unwound);
        }
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `char IniParser_Expect(parser, name)` (0x2a7ef0): consume the expected
/// token, returning 0 on a match, 1 on a mismatch, or the offending char at_3
/// end of input.
pub fn ini_parser_expect(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        let parser = abi::arg(c, 0);
        c.g[S1][0] = sx(parser);
        frame.save(c, S2, 0x20);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[S2][0] = 0;
        callee::INI_SKIP_WHITESPACE.call(c, 0x002a_7f18)?;
        c.g[V1][0] = lw_3(c, S1, 0x8);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lb(c);
        let at_end = c.g[V0][0] == 0;
        c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S2][0] as u32));
        let result: u64;
        if at_end {
            c.cycles += 1;
            result = Addr(c.g[V1][0] as u32).lb(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S3][0] as u32).lb(c);
            let name_end = c.g[V0][0] == 0;
            c.g[S0][0] = c.g[S3][0];
            if name_end {
                c.cycles += 1;
                c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S2][0] as u32));
                result = Addr(c.g[V1][0] as u32).lb(c);
            } else {
                c.cycles += 1;
                'expect: loop {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S1][0];
                    callee::INI_GET_CHAR.call(c, 0x002a_7f40)?;
                    c.g[V1][0] = Addr(c.g[S0][0] as u32).lb(c);
                    if c.g[V0][0] != c.g[V1][0] {
                        c.g[V0][0] = 1;
                        result = 1;
                        break 'expect;
                    }
                    c.cycles += 4;
                    c.g[V1][0] = lw_3(c, S1, 0x8);
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).lb(c);
                    let end = c.g[V0][0] == 0;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
                    if end {
                        c.cycles += 1;
                        c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S2][0] as u32));
                        result = Addr(c.g[V1][0] as u32).lb(c);
                        break 'expect;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).lb(c);
                    let more = c.g[V1][0] != 0;
                    c.g[S0][0] = c.g[V0][0];
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x002a_7f38);
                        }
                        continue 'expect;
                    }
                    c.cycles += 1;
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S2][0] as u32));
                    result = Addr(c.g[V1][0] as u32).lb(c);
                    break 'expect;
                }
            }
        }
        // 0x2a7f70: every path returns the char it loaded (or 1) in v0.
        c.g[V0][0] = result;
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Audio.

/// `void SSXAudioSystem_RiderSay20BD(self, rider)` (0x2a1778): when speech is
/// on and the rider's speech system is ready, post the rider's "20bd" event
/// with the character's voice mask.
pub fn ssx_audio_system_rider_say_20bd(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x0029_f0a0, 0x002a1794)?;
        let speech_off = c.g[V0][0] == 0;
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x5560));
        if !speech_off {
            c.cycles += 5;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002b_0e28, 0x002a17a8)?;
            let not_ready = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !not_ready {
                c.cycles += 10;
                c.mtc1(0, 12);
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x20bd;
                c.g[A3][0] = 0;
                c.g[T0][0] = c.g[S1][0];
                c.g[T1][0] = 0;
                c.g[T2][0] = 1;
                call_ptr(c, 0x002b_1458, 0x002a17d0)?;
                let rejected = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S1][0];
                if !rejected {
                    c.cycles += 12;
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x0029_f2b0, 0x002a17e0)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = 1;
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 0x20bd;
                    call_ptr(c, 0x003d_8008, 0x002a17f4)?;
                    c.g[V1][0] = Addr(gp_3(c, 0x173c)).lw(c);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A2][0] = c.g[S0][0];
                    let post = r32(c, V1);
                    c.g[A1][0] = 1;
                    call_ptr(c, post, 0x002a1808)?;
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

/// `void SSXAudioSystem_DJSay212C(self)` (0x2a2b88): post the DJ's "212c"
/// event with the language mask and the current line, then mark it played.
pub fn ssx_audio_system_dj_say_212c(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        call_ptr(c, 0x0029_f0d8, 0x002a2ba4)?;
        let speech_off = c.g[V0][0] == 0;
        c.g[A0][0] = sx(r32(c, S2).wrapping_add(0x5560));
        if !speech_off {
            c.cycles += 10;
            c.mtc1(0, 12);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x212c;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0xa;
            c.g[T1][0] = 0;
            c.g[T2][0] = 0;
            call_ptr(c, 0x002b_1458, 0x002a2bcc)?;
            let rejected = c.g[V0][0] == 0;
            c.g[V0][0] = 1;
            if !rejected {
                c.cycles += 15;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0029_f198, 0x002a2bdc)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[S1][0] = Addr(p(r32(c, S2), 0x5780)).lw(c);
                c.g[A0][0] = 1;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x212c;
                call_ptr(c, 0x003d_8008, 0x002a2bf4)?;
                c.g[V1][0] = Addr(gp_3(c, 0x173c)).lw(c);
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S0][0];
                c.g[A3][0] = c.g[S1][0];
                let post = r32(c, V1);
                c.g[A1][0] = 2;
                call_ptr(c, post, 0x002a2c0c)?;
                c.g[V0][0] = 1;
            }
            let last = c.g[V0][0];
            c.cycles += 1;
            Addr(p(r32(c, S2), 0x5780)).sw(c, last);
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

/// The shared body of the two player-announcer lines: check the PA speech
/// switch, post the event with the language and rider-voice masks, and play
/// it on the announcer.
pub(crate) fn pa_say(c: &mut Cpu, code: u32, entry: u32) -> Flow {
    c.cycles += 8;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    call_ptr(c, 0x0029_f128, entry + 0x18)?;
    let speech_off = c.g[V0][0] == 0;
    c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x5560));
    let mut skip_one = true;
    if !speech_off {
        c.cycles += 10;
        c.mtc1(0, 12);
        c.g[A1][0] = 0;
        c.g[A2][0] = sx(code);
        c.g[A3][0] = 0;
        c.g[T0][0] = 0xb;
        c.g[T1][0] = 0;
        c.g[T2][0] = 0;
        call_ptr(c, 0x002b_1458, entry + 0x40)?;
        if c.g[V0][0] == 0 {
            skip_one = false;
        } else {
            c.cycles += 18;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0029_f198, entry + 0x50)?;
            c.g[S1][0] = c.g[V0][0];
            call_ptr(c, 0x0028_b1d8, entry + 0x58)?;
            c.g[A1][0] = Addr(p(r32(c, V0), 0x28)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002a_1d48, entry + 0x64)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = 1;
            c.g[A1][0] = 0;
            c.g[A2][0] = sx(code);
            call_ptr(c, 0x003d_8008, entry + 0x78)?;
            c.g[V1][0] = Addr(gp_3(c, 0x173c)).lw(c);
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[A3][0] = c.g[S0][0];
            let post = r32(c, V1);
            c.g[A1][0] = 2;
            call_ptr(c, post, entry + 0x90)?;
        }
    }
    if skip_one {
        c.cycles += 1;
    }
    frame.restore(c, S0, 0x20);
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void SSXAudioSystem_PASay20C1(self)` (0x2a32b0).
pub fn ssx_audio_system_pa_say_20c1(c: &mut Cpu) {
    guest(c, |c| pa_say(c, 0x20c1, 0x002a_32b0))
}

/// `void SSXAudioSystem_PASay20C2(self)` (0x2a3358).
pub fn ssx_audio_system_pa_say_20c2(c: &mut Cpu) {
    guest(c, |c| pa_say(c, 0x20c2, 0x002a_3358))
}

// ---------------------------------------------------------------------------
// `SSXAudioSystem_DJSay212E`/`SSXAudioSystem_DJSay212F` (0x2a2cf0/0x2a2da0): if
// DJ speech is on, push speech context `id` (0x212e/0x212f) and hand the
// language and context masks to the speech system.
pub(crate) fn dj_say(c: &mut Cpu, id: u64, ret_base: u32) -> crate::call::Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x40);
    frame.save(c, S0, 0x30);
    frame.save(c, S2, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save(c, S1, 0x20);
    frame.save_ra(c, 0);
    c.g[S2][0] = c.g[A1][0];
    callee::SSX_AUDIO_IS_DJ_SPEECH_ON.call(c, ret_base)?;
    let off = c.g[V0][0] == 0;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5560));
    if !off {
        c.cycles += 10;
        c.mtc1(0, 12);
        c.g[A1][0] = 0u64;
        c.g[A2][0] = id;
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0xa;
        c.g[T1][0] = 0u64;
        c.g[T2][0] = 0u64;
        callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT_2.call(c, ret_base + 0x28)?;
        if c.g[V0][0] != 0 {
            c.cycles += 18;
            c.g[A0][0] = c.g[S0][0];
            callee::SSX_AUDIO_GET_LANGUAGE_MASK.call(c, ret_base + 0x38)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[S1][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = 0xffff_ffff_ffff_ffff;
            callee::SSX_AUDIO_GET_SPEECH_CONTEXT_MASK.call(c, ret_base + 0x4c)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = 0x1;
            c.g[A1][0] = 0u64;
            c.g[A2][0] = id;
            call_ptr(c, 0x003d_8008, ret_base + 0x60)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x173c));
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[A3][0] = c.g[S0][0];
            let target = c.g[V1][0] as u32;
            c.g[A1][0] = 0x2;
            call_ptr(c, target, ret_base + 0x78)?;
        }
    }
    c.cycles += 1;
    c.lq(16, frame.at(c, 0x30));
    c.cycles += 5;
    c.lq(17, frame.at(c, 0x20));
    c.lq(18, frame.at(c, 0x10));
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `SSXAudioSystem_DJSay212E` (0x2a2cf0): queue DJ speech 0x212e.
pub fn ssx_audio_system_dj_say212e(c: &mut Cpu) {
    guest(c, |c| dj_say(c, 0x212e, 0x002a_2d10))
}

/// `SSXAudioSystem_DJSay212F` (0x2a2da0): queue DJ speech 0x212f.
pub fn ssx_audio_system_dj_say212f(c: &mut Cpu) {
    guest(c, |c| dj_say(c, 0x212f, 0x002a_2dc0))
}

// ---------------------------------------------------------------------------
// `IniParser_ReadQuotedString` (0x2a7cf0): skip whitespace, expect a quote,
// copy characters until the closing quote, expect a quote, terminate the
// string and return its buffer.
pub fn ini_parser_read_quoted_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        frame.save(c, S4, 0x10);
        c.g[S0][0] = 0x4a0000;
        frame.save(c, S3, 0x20);
        c.g[S4][0] = c.g[S0][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = 0u64;
        callee::INI_PARSER_SKIP_WHITESPACE.call(c, 0x002a_7d20)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3728));
        callee::INI_PARSER_EXPECT.call(c, 0x002a_7d2c)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
        c.g[A0][0] = 0x22;
        c.g[V0][0] = c.lb(c.g[V1][0] as u32);
        let quoted = c.g[V0][0] == c.g[A0][0];
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
        if !quoted {
            c.cycles += 2;
            c.g[S3][0] = 0x22;
            loop {
                c.cycles += 8;
                c.g[A0][0] = c.g[S1][0];
                callee::INI_PARSER_GET_CHAR.call(c, 0x002a_7d50)?;
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.sb(c.g[V1][0] as u32, c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lb(c.g[V0][0] as u32);
                let at_quote = c.g[V1][0] == c.g[S3][0];
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                if at_quote {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002a_7d48);
                }
            }
        }
        c.cycles += 14;
        c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x3728));
        c.g[A0][0] = c.g[S1][0];
        callee::INI_PARSER_EXPECT.call(c, 0x002a_7d74)?;
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
        c.lq(16, frame.at(c, 0x50));
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
        c.lq(17, frame.at(c, 0x40));
        c.lq(18, frame.at(c, 0x30));
        c.lq(19, frame.at(c, 0x20));
        c.lq(20, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        c.sb(c.g[V1][0] as u32, 0u64);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = ra;
        Ok(())
    })
}

/// `void SSXAudioSystem_StopObjectLoop(self, objectId, int context)`
/// (0x297eb8): find the loop slot for (object, context), stop its bank
/// handle and clear the slot.
pub fn ssx_audio_system_stop_object_loop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S0, 0x60);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S1][0] = this;
        c.g[S0][0] = this;
        c.g[S2][0] = sx((this as u32).wrapping_add(0x5a0));
        c.g[S3][0] = c.g[A2][0];
        c.g[S4][0] = c.g[A1][0];
        c.g[S5][0] = 0x1;
        c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x5a00)).lw(c);
        loop {
            c.cycles += 2;
            let found = if c.g[V0][0] != c.g[S5][0] {
                false
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x5a04)).lw(c);
                if c.g[V0][0] != c.g[S4][0] {
                    false
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x5a20)).lw(c);
                    if c.g[V0][0] != c.g[S3][0] {
                        false
                    } else {
                        c.cycles += 9;
                        c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 0x118)).lw(c);
                        c.g[A2][0] = 0x1;
                        c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0x5a24)).lw(c);
                        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
                        c.mtc1(0, 12);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                        call_ptr(c, 0x002a_d5f0, 0x0029_7f34)?;
                        c.sw(p(c.g[S0][0] as u32, 0x5a00), 0);
                        true
                    }
                }
            };
            let _ = found;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x30));
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x5a00)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0029_7ef8);
            }
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

/// `void SSXAudioSystem_StoreRaceResult(self, result, int a2, int a3)`
/// (0x2a4660): remember a race result in the audio system's result buffers.
pub fn ssx_audio_system_store_race_result(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = Addr(p(r32(c, GP), 0xffff_f7b8)).lw(c);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0xc0)).lw(c);
        c.g[V0][0] = Addr(p(c.g[A0][0] as u32, 0x98)).lw(c);
        let finished = c.g[V0][0] != 0;
        c.g[S2][0] = c.g[A2][0];
        let mut tail = false;
        if !finished {
            c.cycles += 5;
            c.g[V1][0] = Addr(p(c.g[A0][0] as u32, 0x70)).lw(c);
            c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x5804)).lw(c);
                tail = true;
            }
        }
        if !tail {
            c.cycles += 3;
            c.g[A2][0] = 0x1;
            let no_track = c.g[T0][0] == 0;
            c.sw(p(c.g[S0][0] as u32, 0x57f8), c.g[A2][0]);
            if !no_track {
                c.cycles += 4;
                c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
                c.g[V0][0] = 0x3;
                if c.g[V1][0] == c.g[V0][0] {
                    c.sw(p(c.g[S0][0] as u32, 0x57fc), c.g[A2][0]);
                }
            }
            c.cycles += 10;
            c.sw(p(c.g[S0][0] as u32, 0x5800), c.g[A1][0]);
            call_ptr(c, 0x0028_b1e8, 0x002a_46c8)?;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_dd58, 0x002a_46d4)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_4bc0, 0x002a_46dc)?;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.sw(p(c.g[S0][0] as u32, 0x5810), c.g[V1][0]);
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x5804)).lw(c);
        }
        c.cycles += 11;
        c.g[V1][0] = Addr(p(c.g[S0][0] as u32, 0x5808)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S1][0] as u32));
        c.sw(p(c.g[S0][0] as u32, 0x5804), c.g[V0][0]);
        c.sw(p(c.g[S0][0] as u32, 0x5808), c.g[V1][0]);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = c.g[RA][0] as u32;
        Ok(())
    })
}

/// `void SSXAudioSystem_DJSay20CC(self, int context)` (0x2a24b0): have the DJ
/// say line 0x20cc in the given speech context, if DJ speech is on.
pub fn ssx_audio_system_dj_say_20cc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        call_ptr(c, 0x0029_f0d8, 0x002a_24d0)?;
        let off = c.g[V0][0] == 0;
        c.g[A0][0] = sx((this as u32).wrapping_add(0x5560));
        let mut tail = 1;
        if !off {
            c.cycles += 10;
            c.mtc1(0, 12);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x20cc;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0xa;
            c.g[T1][0] = 0;
            c.g[T2][0] = 0;
            call_ptr(c, 0x002b_1458, 0x002a_24f8)?;
            if c.g[V0][0] == 0 {
                tail = 0;
            } else {
                c.cycles += 19;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0029_f198, 0x002a_2508)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S2][0];
                c.g[A2][0] = 0xffff_ffff_ffff_ffff;
                call_ptr(c, 0x002a_1bd8, 0x002a_251c)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = 0x1;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x20cc;
                call_ptr(c, 0x003d_8008, 0x002a_2530)?;
                c.g[V1][0] = Addr(p(r32(c, GP), 0x173c)).lw(c);
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[A3][0] = c.g[S0][0];
                c.g[A1][0] = 0x3;
                let target = c.g[V1][0] as u32;
                c.g[T0][0] = 0x3;
                call_ptr(c, target, 0x002a_254c)?;
            }
        }
        c.cycles += tail;
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// DJ speech wrappers.

/// The shared body of the six `SSXAudioSystem_DJSayXXXX` entry points: when DJ
/// speech is on, play the speech event with the current language mask and
/// track/category, then hand it to the speech system.
pub(crate) fn dj_say_2(c: &mut Cpu, base: u32, event: u64) -> Flow {
    let r1 = base.wrapping_add(0x18);
    let r2 = base.wrapping_add(0x40);
    let r3 = base.wrapping_add(0x50);
    let r4 = base.wrapping_add(0x58);
    let r5 = base.wrapping_add(0x64);
    let r6 = base.wrapping_add(0x6c);
    let r7 = base.wrapping_add(0x78);
    let r8 = base.wrapping_add(0x8c);
    let r9 = base.wrapping_add(0xa4);
    c.cycles += 8;
    let frame = Frame::open(c, 0x30);
    'outer: {
        'skip: {
            frame.save(c, S0, 0x20);
            frame.save(c, S1, 0x10);
            frame.save_ra(c, 0);
            let this = r(c, A0);
            c.g[S0][0] = sx(this);
            c.g[RA][0] = sx(r1);
            callee::IS_DJ_SPEECH_ON.call(c, r1)?;
            let off = c.g[V0][0] == 0;
            c.g[A0][0] = sx(p(this, 0x5560));
            if off {
                break 'skip;
            }
            c.cycles += 10;
            c.mtc1(0, 12);
            c.g[A1][0] = 0;
            c.g[A2][0] = event;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0xa;
            c.g[T1][0] = 0;
            c.g[RA][0] = sx(r2);
            c.g[T2][0] = 0;
            callee::SET_EVENT_CONTEXT.call(c, r2)?;
            if c.g[V0][0] == 0 {
                frame.restore(c, 16, 0x20);
                break 'outer;
            }
            c.cycles += 23;
            c.g[RA][0] = sx(r3);
            c.g[A0][0] = c.g[S0][0];
            callee::GET_LANGUAGE_MASK.call(c, r3)?;
            c.g[RA][0] = sx(r4);
            c.g[S1][0] = c.g[V0][0];
            call_ptr(c, callee::AUDIO_GET_GAME_MODULE, r4)?;
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(r5);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::BE_GET_INTERFACE, r5)?;
            c.g[RA][0] = sx(r6);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::BE_GET_SOMETHING, r6)?;
            c.g[A1][0] = Addr(r(c, V0)).lw(c);
            c.g[RA][0] = sx(r7);
            c.g[A0][0] = c.g[S0][0];
            callee::TRACK_TO_CATEGORY.call(c, r7)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = 1;
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(r8);
            c.g[A2][0] = event;
            call_ptr(c, callee::SPEECH_NEW, r8)?;
            c.g[V1][0] = Addr(gp(c, 0x173c)).lw(c);
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[A3][0] = c.g[S0][0];
            let target = r(c, V1);
            c.g[RA][0] = sx(r9);
            c.g[A1][0] = 2;
            call_ptr(c, target, r9)?;
            break 'skip;
        }
        c.cycles += 1;
        frame.restore(c, 16, 0x20);
    }
    c.cycles += 4;
    frame_restore_s1(c);
    Ok(())
}

/// The `lq s1; ld ra; addiu sp; jr ra` tail of a 0x30-byte frame.
#[inline(always)]
pub(crate) fn frame_restore_s1(c: &mut Cpu) {
    c.lq(17, p(r(c, SP), 0x10));
    c.g[RA][0] = c.ld(r(c, SP));
    let ra = r(c, RA);
    c.g[SP][0] = sx(r(c, SP).wrapping_add(0x30));
    c.pc = ra;
}

/// `void SSXAudioSystem_DJSay20C8(void* self)` (0x2a2018).
pub fn dj_say_20c8(c: &mut Cpu) {
    guest(c, |c| dj_say_2(c, 0x002a_2018, 0x20c8))
}

/// `void SSXAudioSystem_DJSay20C9(void* self)` (0x2a20d0).
pub fn dj_say_20c9(c: &mut Cpu) {
    guest(c, |c| dj_say_2(c, 0x002a_20d0, 0x20c9))
}

/// `void SSXAudioSystem_DJSay20E7(void* self)` (0x2a2260).
pub fn dj_say_20e7(c: &mut Cpu) {
    guest(c, |c| dj_say_2(c, 0x002a_2260, 0x20e7))
}

/// `void SSXAudioSystem_DJSay20E8(void* self)` (0x2a2318).
pub fn dj_say_20e8(c: &mut Cpu) {
    guest(c, |c| dj_say_2(c, 0x002a_2318, 0x20e8))
}

/// `void SSXAudioSystem_DJSay20BA(void* self)` (0x2a2638).
pub fn dj_say_20ba(c: &mut Cpu) {
    guest(c, |c| dj_say_2(c, 0x002a_2638, 0x20ba))
}

/// `void SSXAudioSystem_DJSay212B(void* self)` (0x2a2ad0).
pub fn dj_say_212b(c: &mut Cpu) {
    guest(c, |c| dj_say_2(c, 0x002a_2ad0, 0x212b))
}

/// `SSXAudioSystem_SetFadeMode(this, mode, f0, f1)` (0x287f00): when the
/// requested mode differs and no fade is pending, look the mode's mask up and
/// start a fade; when it matches and a fade is pending, retarget it; otherwise
/// do nothing.
pub fn ssx_audio_system_set_fade_mode(c: &mut Cpu) {
    'b_287fac: {
        'b_287fa8: {
            'b_287f6c: {
                c.cycles += 12;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.g[V0][0] = 0x3;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S0][0] = c.g[A0][0];
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x38));
                c.g[S1][0] = c.g[A1][0];
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.mov_s(21, 12);
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                let t = c.g[S1][0] != c.g[V0][0];
                c.mov_s(20, 13);
                if t {
                    break 'b_287f6c;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x63dc));
                    if c.g[V0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        break 'b_287fac;
                    } else {
                        c.cycles += 12;
                        c.g[RA][0] = sx(0x00287f44);
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x63d4));
                        c.pc = 0x00284ba0;
                        ee::s07::SSXAudioSystem_GetFadeMask(c);
                        if c.pc != 0x00287f44 {
                            return;
                        }
                        c.g[AT][0] = 0x3f800000;
                        c.mtc1(1, 12);
                        c.g[A1][0] = c.g[V0][0];
                        c.mov_s(13, 21);
                        c.g[RA][0] = sx(0x00287f5c);
                        c.g[A0][0] = c.g[S0][0];
                        c.pc = 0x00287a10;
                        ee::s07::SSXAudioSystem_SetFadeGain(c);
                        if c.pc != 0x00287f5c {
                            return;
                        }
                        c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x63d8));
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x63d4), c.g[S1][0]);
                        let t = true;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x63dc), 0u64);
                        if t {
                            break 'b_287fa8;
                        } else {
                            break 'b_287f6c;
                        }
                    }
                }
            }
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x63dc));
            if c.g[V0][0] != 0u64 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_287fac;
            } else {
                c.cycles += 12;
                c.g[RA][0] = sx(0x00287f80);
                c.g[A0][0] = c.g[S1][0];
                c.pc = 0x00284ba0;
                ee::s07::SSXAudioSystem_GetFadeMask(c);
                if c.pc != 0x00287f80 {
                    return;
                }
                c.g[A1][0] = c.g[V0][0];
                c.mov_s(13, 21);
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x00287f94);
                c.mov_s(12, 20);
                c.pc = 0x00287a10;
                ee::s07::SSXAudioSystem_SetFadeGain(c);
                if c.pc != 0x00287f94 {
                    return;
                }
                c.g[V0][0] = 0x1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x63d4), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x63dc), c.g[V0][0]);
                c.swc1(20, (c.g[S0][0] as u32).wrapping_add(0x63d8));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x63e0), 0u64);
                break 'b_287fa8;
            }
        }
        c.cycles += 1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        break 'b_287fac;
    }
    c.cycles += 6;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
    c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x38));
    c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = t;
    return;
}

/// `void SSXAudioSystem_DJSay20CC_2(self, u32 context)` (0x2a2568): when DJ
/// speech is on, install a 20CC speech event and, if accepted, queue the
/// actual line through the speech system.
pub fn ssx_audio_system_dj_say20cc_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        call_ptr(c, 0x0029_f0d8, 0x002a_2588)?;
        let off = c.g[V0][0] == 0;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5560));
        if off {
            // Speech off: fall through to the epilogue below.
        } else {
            c.cycles += 10;
            c.mtc1(0, 12);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x20cc;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0xa;
            c.g[T1][0] = 0;
            c.g[T2][0] = 0;
            call_ptr(c, 0x002b_1458, 0x002a_25b0)?;
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                frame.restore(c, S0, 0x30);
                c.cycles += 5;
                frame.restore(c, S1, 0x20);
                frame.restore(c, S2, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 25;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0029_f198, 0x002a_25c0)?;
            c.g[S1][0] = c.g[V0][0];
            call_ptr(c, 0x0028_b1e8, 0x002a_25c8)?;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_dd58, 0x002a_25d4)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_4bc0, 0x002a_25dc)?;
            c.g[A1][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S2][0];
            call_ptr(c, 0x002a_1bd8, 0x002a_25ec)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = 0x1;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x20cc;
            call_ptr(c, 0x003d_8008, 0x002a_2600)?;
            c.g[V1][0] = Addr(gp(c, 0x173c)).lw(c);
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[A3][0] = c.g[S0][0];
            c.g[A1][0] = 0x3;
            let target = c.g[V1][0] as u32;
            c.g[T0][0] = 0x3;
            call_ptr(c, target, 0x002a_261c)?;
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// A `gp_4`-relative global address.
#[inline(always)]
pub(crate) fn gp_4(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

// ---------------------------------------------------------------------------
// Audio announcer lines.

/// `void SSXAudioSystem_DJSay20CA(SSXAudioSystem* self)` (0x2a2188): check the
/// DJ speech switch, post event 0x20CA with the language and DJ-voice masks,
/// then hand it to the speech manager with the category.
pub fn ssx_audio_system_dj_say_20ca(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0029_f0d8, 0x002a_21a4)?;
        let speech_off = c.g[V0][0] == 0;
        c.g[A0][0] = sx(p(r32(c, S0), 0x5560));
        let mut rejected = false;
        if !speech_off {
            c.cycles += 10;
            c.mtc1(0, 12);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x20ca;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0xa;
            c.g[T1][0] = 0;
            c.g[T2][0] = 0;
            call_ptr(c, 0x002b_1458, 0x002a_21cc)?;
            if c.g[V0][0] != 0 {
                c.cycles += 29;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0029_f198, 0x002a_21dc)?;
                c.g[S2][0] = c.g[V0][0];
                call_ptr(c, 0x0028_b1d8, 0x002a_21e4)?;
                c.g[A1][0] = Addr(p(r32(c, V0), 0x28)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002a_1d48, 0x002a_21f0)?;
                c.g[S1][0] = c.g[V0][0];
                call_ptr(c, 0x0028_b1e8, 0x002a_21f8)?;
                c.g[A1][0] = 0;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014_dd58, 0x002a_2204)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014_4bc0, 0x002a_220c)?;
                c.g[A1][0] = Addr(r32(c, V0)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002a_1e20, 0x002a_2218)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = 1;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x20ca;
                call_ptr(c, 0x003d_8008, 0x002a_222c)?;
                c.g[V1][0] = Addr(gp_4(c, 0x173c)).lw(c);
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S2][0];
                c.g[A3][0] = c.g[S1][0];
                c.g[T0][0] = c.g[S0][0];
                let post = r32(c, V1);
                c.g[A1][0] = 3;
                call_ptr(c, post, 0x002a_2248)?;
            } else {
                rejected = true;
            }
        }
        if !rejected {
            c.cycles += 1;
        }
        c.lq(S0, frame.at(c, 0x30));
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_DJSay20CF(SSXAudioSystem* self, int key)` (0x2a2860):
/// the same for event 0x20CF, but picks one bit from the language/voice mask
/// pair (or from `key` when it is a mask index) before posting.
pub fn ssx_audio_system_dj_say_20cf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S0, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, 0x0029_f0d8, 0x002a_2884)?;
        let speech_off = c.g[V0][0] == 0;
        c.g[A0][0] = sx(p(r32(c, S1), 0x5560));
        if !speech_off {
            c.cycles += 10;
            c.mtc1(0, 12);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x20cf;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0xa;
            c.g[T1][0] = 0;
            c.g[T2][0] = 0;
            call_ptr(c, 0x002b_1458, 0x002a_28ac)?;
            let rejected = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S1][0];
            if !rejected {
                c.cycles += 5;
                c.g[S3][0] = 0;
                call_ptr(c, 0x0029_f198, 0x002a_28bc)?;
                c.g[S2][0] = 0;
                c.g[S1][0] = c.g[V0][0];
                if (c.g[S0][0] as i64) >= 0 {
                    c.cycles += 3;
                    c.g[V0][0] = 0x3e7;
                    let is_999 = c.g[S0][0] == c.g[V0][0];
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 100i64) as u64;
                    if !is_999 {
                        c.cycles += 2;
                        let high = c.g[V0][0] == 0;
                        c.g[V0][0] = 1;
                        if high {
                            c.cycles += 2;
                            c.g[S0][0] = sx(r32(c, S0).wrapping_sub(0x64));
                            c.g[S2][0] = sx(r32(c, V0) << (r32(c, S0) & 31));
                        } else {
                            c.cycles += 2;
                            c.g[S3][0] = sx(r32(c, V0) << (r32(c, S0) & 31));
                        }
                    }
                }
                c.cycles += 12;
                c.g[A0][0] = 1;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x20cf;
                call_ptr(c, 0x003d_8008, 0x002a_28fc)?;
                c.g[V1][0] = Addr(gp_4(c, 0x173c)).lw(c);
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[T0][0] = c.g[S3][0];
                c.g[T1][0] = c.g[S2][0];
                c.g[A1][0] = 4;
                c.g[A3][0] = 3;
                let post = r32(c, V1);
                call_ptr(c, post, 0x002a_291c)?;
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
    })
}

/// `void SSXAudioSystem_DJSay2102(SSXAudioSystem* self)` (0x2a2938): the same
/// as [`ssx_audio_system_dj_say_20ca`] for event 0x2102.
pub fn ssx_audio_system_dj_say_2102(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0029_f0d8, 0x002a_2954)?;
        let speech_off = c.g[V0][0] == 0;
        c.g[A0][0] = sx(p(r32(c, S0), 0x5560));
        let mut rejected = false;
        if !speech_off {
            c.cycles += 10;
            c.mtc1(0, 12);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x2102;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0xa;
            c.g[T1][0] = 0;
            c.g[T2][0] = 0;
            call_ptr(c, 0x002b_1458, 0x002a_297c)?;
            if c.g[V0][0] != 0 {
                c.cycles += 29;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0029_f198, 0x002a_298c)?;
                c.g[S2][0] = c.g[V0][0];
                call_ptr(c, 0x0028_b1d8, 0x002a_2994)?;
                c.g[A1][0] = Addr(p(r32(c, V0), 0x28)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002a_1d48, 0x002a_29a0)?;
                c.g[S1][0] = c.g[V0][0];
                call_ptr(c, 0x0028_b1e8, 0x002a_29a8)?;
                c.g[A1][0] = 0;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014_dd58, 0x002a_29b4)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014_4bc0, 0x002a_29bc)?;
                c.g[A1][0] = Addr(r32(c, V0)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002a_1e20, 0x002a_29c8)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = 1;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x2102;
                call_ptr(c, 0x003d_8008, 0x002a_29dc)?;
                c.g[V1][0] = Addr(gp_4(c, 0x173c)).lw(c);
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S2][0];
                c.g[A3][0] = c.g[S1][0];
                c.g[T0][0] = c.g[S0][0];
                let post = r32(c, V1);
                c.g[A1][0] = 3;
                call_ptr(c, post, 0x002a_29f8)?;
            } else {
                rejected = true;
            }
        }
        if !rejected {
            c.cycles += 1;
        }
        c.lq(S0, frame.at(c, 0x30));
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_DJSay20E9 (0x00002a23d0).

/// `SSXAudioSystem_DJSay20E9`: make the DJ say the given line (event 0x20e9) unless the DJ is already talking.
pub fn ssx_audio_system_dj_say_20e9(c: &mut Cpu) {
    guest(c, |c| {
        'b_2a249c: {
            'b_2a2498: {
                c.cycles += 10;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.g[S1][0] = c.g[A1][0];
                call_ptr(c, 0x0029f0d8, 0x002a23f0)?;
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5560));
                if t {
                    break 'b_2a2498;
                } else {
                    c.cycles += 10;
                    c.mtc1(0, 12);
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0x20e9;
                    c.g[A3][0] = 0u64;
                    c.g[T0][0] = 0xa;
                    c.g[T1][0] = 0u64;
                    c.g[T2][0] = 0u64;
                    call_ptr(c, 0x002b1458, 0x002a2418)?;
                    if c.g[V0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        break 'b_2a249c;
                    } else {
                        'b_2a2464: {
                            'b_2a2460: {
                                c.cycles += 6;
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x0029f198, 0x002a2428)?;
                                c.g[S2][0] = c.g[V0][0];
                                c.g[V0][0] = 0xffffffffffffffff;
                                let t = c.g[S1][0] != c.g[V0][0];
                                c.g[A0][0] = c.g[S0][0];
                                if t {
                                    break 'b_2a2460;
                                } else {
                                    c.cycles += 10;
                                    call_ptr(c, 0x0028b1e8, 0x002a2440)?;
                                    c.g[A1][0] = 0u64;
                                    c.g[A0][0] = c.g[V0][0];
                                    call_ptr(c, 0x0014dd58, 0x002a244c)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    call_ptr(c, 0x00144bc0, 0x002a2454)?;
                                    c.g[A1][0] = c.lw(c.g[V0][0] as u32);
                                    let t = true;
                                    c.g[A0][0] = c.g[S0][0];
                                    if t {
                                        break 'b_2a2464;
                                    } else {
                                        break 'b_2a2460;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[A1][0] = c.g[S1][0];
                            break 'b_2a2464;
                        }
                        c.cycles += 13;
                        call_ptr(c, 0x002a1e20, 0x002a246c)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A0][0] = 0x1;
                        c.g[A1][0] = 0u64;
                        c.g[A2][0] = 0x20e9;
                        call_ptr(c, 0x003d8008, 0x002a2480)?;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x173c));
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A2][0] = c.g[S2][0];
                        c.g[A3][0] = c.g[S0][0];
                        let t = c.g[V1][0] as u32;
                        c.g[A1][0] = 0x2;
                        c.g[RA][0] = sx(0x002a2498);
                        c.pc = t;
                        ssx3_ee_rt::funcs::call_indirect(c, t);
                        if c.pc != 0x002a2498 {
                            return Err(crate::call::Unwound);
                        }
                        break 'b_2a2498;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_2a249c;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// DJ / arcade speech wrappers.

/// `void SSXAudioSystem_DJSay20E5(void* self, int voice, int lang)` (0x2a26f0):
/// when DJ speech is enabled, stop the instance's speech stream, set the
/// `0x20e5` event context and start the announcement through the speech
/// interface, clearing the "pending" word at `self + 0x5794`.
pub fn ssx_audio_system_dj_say20e5(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S0, 0x60);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        c.g[S1][0] = c.g[A0][0];
        c.g[S4][0] = c.g[A1][0];
        c.g[S5][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        callee::IS_DJ_SPEECH_ON_2.call(c, 0x002a_2720)?;

        let speech = p(r(c, S1), 0x5560);
        let speaking = r(c, V0) != 0;
        c.g[S0][0] = sx(speech);
        if !speaking {
            return epilogue(c, frame);
        }

        c.cycles += 14;
        c.g[A1][0] = 0;
        c.g[A0][0] = sx(speech);
        callee::SPEECH_STOP_INSTANCE.call(c, 0x002a_2734)?;

        c.mtc1(0, 12);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x20e5;
        c.g[A3][0] = 0;
        c.g[T0][0] = 0xa;
        c.g[T1][0] = 0;
        c.g[T2][0] = 0;
        callee::SPEECH_SET_EVENT_CONTEXT.call(c, 0x002a_2758)?;

        let started = r(c, V0) != 0;
        c.g[A0][0] = c.g[S1][0];
        if !started {
            return epilogue(c, frame);
        }

        // `s2` is the voice (1 or 2), `s0` the language index (1 or 2).
        c.cycles += 7;
        c.g[S2][0] = 2;
        callee::GET_LANGUAGE_MASK_2.call(c, 0x002a_2768)?;
        c.g[S3][0] = c.g[V0][0];
        c.g[S0][0] = 1;
        c.g[V0][0] = Addr(p(r(c, S1), 0x5794)).lw(c);
        let pending = c.g[V0][0] == 0;
        if c.g[S4][0] != 0 {
            c.g[S2][0] = c.g[S0][0];
        }
        if !pending {
            c.cycles += 1;
            c.sw(p(r(c, S1), 0x5794), 0);
        }

        c.cycles += 13;
        c.g[V0][0] = 2;
        c.g[A0][0] = 1;
        if c.g[S5][0] != 0 {
            c.g[S0][0] = c.g[V0][0];
        }
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x20e5;
        call_ptr(c, 0x003d_8008, 0x002a_2798)?;

        c.g[V1][0] = Addr(gp(c, 0x173c)).lw(c);
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S2][0];
        c.g[T0][0] = c.g[S0][0];
        let target = r(c, V1);
        c.g[A1][0] = 3;
        call_ptr(c, target, 0x002a_27b4)?;
        epilogue(c, frame)
    })
}

/// The `DJSay20E5` epilogue: restore s0-s5, ra, sp and return.
pub(crate) fn epilogue(c: &mut Cpu, frame: Frame) -> Flow {
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
}

/// `void SSXAudioSystem_ArcadeSay20A7(void* self, void* obj, int arg)`
/// (0x2a3b18): when arcade speech is enabled, the object's animation has not
/// finished (`+0x470 >= 0`), the object is this system's current animation
/// target and the speech system is idle, announce event `0x20a7` and hand it
/// to the speech interface.
pub fn ssx_audio_system_arcade_say20a7(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        c.g[S0][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x40);
        callee::IS_ARCADE_SPEECH_ON.call(c, 0x002a_3b40)?;

        'outer: {
            if r(c, V0) == 0 {
                frame.restore(c, 16, 0x30);
                break 'outer;
            }
            c.cycles += 2;
            if c.g[S0][0] == 0 {
                frame.restore(c, 16, 0x30);
                break 'outer;
            }

            // f20 = 0; fcc = (f20 <= *(*obj + 0x470)), i.e. ">= 0".
            c.cycles += 6;
            c.lwc1(0, p(r(c, S0), 0x470));
            c.mtc1(0, 20);
            c.c_le_s(20, 0);
            let done = c.fcc();
            c.g[V0][0] = 1;
            if !done {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            c.cycles += 2;
            if r(c, V0) != 0 {
                frame.restore(c, 16, 0x30);
                break 'outer;
            }

            // The object must be this system's bound animation object.
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = u64::MAX;
            call_ptr(c, 0x0028_5d98, 0x002a_3b80)?;
            if c.g[S0][0] != c.g[V0][0] {
                frame.restore(c, 16, 0x30);
                break 'outer;
            }

            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0028_8ae0, 0x002a_3b90)?;
            let busy = r(c, V0) != 0;
            frame.restore(c, 16, 0x30);
            if busy {
                break 'outer;
            }

            'context: {
                c.cycles += 11;
                c.g[A0][0] = sx(p(r(c, S1), 0x5560));
                c.mov_s(12, 20);
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x20a7;
                c.g[A3][0] = 0;
                c.g[T0][0] = 0xc;
                c.g[T1][0] = 0;
                c.g[T2][0] = 0;
                callee::SPEECH_SET_EVENT_CONTEXT.call(c, 0x002a_3bbc)?;
                let skip = r(c, V0) == 0;
                c.g[A0][0] = 1;
                if skip {
                    break 'context;
                }
                c.cycles += 8;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x20a7;
                call_ptr(c, 0x003d_8008, 0x002a_3bd0)?;
                c.g[V1][0] = Addr(gp(c, 0x173c)).lw(c);
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S2][0];
                let target = r(c, V1);
                c.g[A1][0] = 1;
                call_ptr(c, target, 0x002a_3be4)?;
            }
            c.cycles += 1;
            frame.restore(c, 16, 0x30);
        }

        c.cycles += 6;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `CrowdAudio_Unload(self)` (0x2a6f38): stop all crowd audio, release the
/// instance array, then free the four sample lists of every bank.
pub fn crowd_audio_unload(c: &mut Cpu) {
    const STOP_ALL: Callee = Callee {
        addr: 0x002a_7718,
        f: ee::s08::CrowdAudio_StopAll,
    };
    const FREE_LIST: Callee = Callee {
        addr: 0x002a_4cf8,
        f: ee::s08::CrowdAudio_FreeSampleList,
    };
    const DESTROY: Callee = Callee {
        addr: 0x002a_4e38,
        f: ee::s08::CrowdAudio_DestroyInstance,
    };
    const MEM_FREE: u32 = 0x0031_7e98;

    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x80);
        let this = abi::arg(c, 0);
        frame.save(c, 17, 0x60);
        frame.save(c, 16, 0x70);
        frame.save(c, 18, 0x50);
        frame.save(c, 19, 0x40);
        frame.save(c, 20, 0x30);
        frame.save(c, 21, 0x20);
        frame.save(c, 22, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        STOP_ALL.call(c, 0x002a_6f64)?;
        c.g[A1][0] = lw(c, p(this, 0x18));
        let no_instances = c.g[A1][0] == 0;
        c.g[V0][0] = 0x190;
        if !no_instances {
            c.cycles += 5;
            c.g[V1][0] = lw(c, (c.g[A1][0] as u32).wrapping_sub(0x10));
            c.mult(0, A0, V1, V0);
            c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            if c.g[A1][0] == c.g[S0][0] {
                c.g[A0][0] = lw(c, p(this, 0x18));
            } else {
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x190));
                loop {
                    c.cycles += 6;
                    c.g[A1][0] = 0;
                    c.g[A0][0] = c.g[S0][0];
                    DESTROY.call(c, 0x002a_6f94)?;
                    c.g[V0][0] = lw(c, p(this, 0x18));
                    let more = c.g[V0][0] != c.g[S0][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x190));
                    if !more {
                        c.cycles += 1;
                        c.g[A0][0] = lw(c, p(this, 0x18));
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002a_6f88);
                    }
                }
            }
            c.cycles += 4;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
            call_ptr(c, MEM_FREE, 0x002a_6fac)?;
            sw_2(c, p(c.g[S1][0] as u32, 0x18), 0);
            sw_2(c, p(c.g[S1][0] as u32, 0x14), 0);
        }
        c.cycles += 7;
        c.g[S6][0] = sx((c.g[S1][0] as u32).wrapping_add(0x168));
        c.g[S5][0] = sx((c.g[S1][0] as u32).wrapping_add(0x34));
        c.g[S4][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4c));
        c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1c));
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x6c));
        c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x68));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x64));
        loop {
            c.cycles += 17;
            sw_2(c, c.g[S1][0] as u32, 0);
            c.g[A0][0] = c.g[S4][0];
            sw_2(c, c.g[S2][0] as u32, 0);
            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x54));
            sw_2(c, c.g[S0][0] as u32, 0);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x54));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x54));
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x54));
            FREE_LIST.call(c, 0x002a_6ff4)?;
            c.g[A0][0] = c.g[S5][0];
            FREE_LIST.call(c, 0x002a_6ffc)?;
            c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x54));
            c.g[A0][0] = c.g[S3][0];
            FREE_LIST.call(c, 0x002a_7008)?;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S6][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x54));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002a_6fd0);
            }
        }
        c.cycles += 10;
        frame.restore(c, 16, 0x70);
        frame.restore(c, 17, 0x60);
        frame.restore(c, 18, 0x50);
        frame.restore(c, 19, 0x40);
        frame.restore(c, 20, 0x30);
        frame.restore(c, 21, 0x20);
        frame.restore(c, 22, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void CrowdAudio_InstanceRemoveSource(SsMgr* self, int idx, float fade)`
/// (0x2a62f0): drop voice `idx` (12-byte entries at `+0x20`): release the
/// reference, free its stream slot when the last reference goes, and when no
/// active voices remain reset the manager (clamped volume, stream id,
/// state).
pub fn crowd_audio_instance_remove_source(c: &mut Cpu) {
    'b_2a6418: {
        'b_2a6414: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(16, c.pa(SP, 0x20));
            c.swc1(20, c.pa(SP, 0x30));
            c.g[S0][0] = c.g[A0][0];
            c.sq(17, c.pa(SP, 0x10));
            c.mov_s(20, 12);
            let t = (c.g[A1][0] as i64) < 0;
            c.std(SP, 0, c.g[RA][0]);
            if t {
                break 'b_2a6414;
            } else {
                c.cycles += 7;
                c.g[V0][0] = 0xc;
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x20));
                c.mult(0, 5, 5, 2);
                c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[V0][0] = c.ldw_2(A0, 0);
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x28));
                if t {
                    break 'b_2a6414;
                } else {
                    'b_2a6390: {
                        c.cycles += 11;
                        c.g[V0][0] = c.ldw_2(S0, 0xd4);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        c.g[S1][0] = c.ldw_2(V1, 0);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                        c.stw_2(S0, 0xd4, c.g[V0][0]);
                        c.stw_2(A0, 0, 0u64);
                        c.stw_2(V1, 0, 0u64);
                        c.g[V0][0] = c.ldw_2(S1, 0x8);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                        let t = c.g[V0][0] != 0u64;
                        c.stw_2(S1, 0x8, c.g[V0][0]);
                        if t {
                            break 'b_2a6390;
                        } else {
                            'b_2a6374: {
                                c.cycles += 3;
                                c.g[A1][0] = c.ldw_2(S1, 0x4);
                                let t = (c.g[A1][0] as i64) < 0;
                                c.g[V0][0] = c.ldw_2(GP, 0x608);
                                if t {
                                    break 'b_2a6374;
                                } else {
                                    c.cycles += 4;
                                    c.g[A2][0] = 0x1;
                                    c.g[A0][0] = c.ldw_2(V0, 0);
                                    c.g[RA][0] = sx(0x002a6374);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                                    c.pc = 0x002ad5f0;
                                    call_indirect(c, 0x002ad5f0);
                                    if c.pc != 0x002a6374 {
                                        return;
                                    }
                                    break 'b_2a6374;
                                }
                            }
                            c.cycles += 7;
                            c.g[V0][0] = 0xffffffffffffffff;
                            c.stw_2(S1, 0, c.g[V0][0]);
                            c.stw_2(S1, 0x4, c.g[V0][0]);
                            c.stw_2(S0, 0x18c, 0u64);
                            c.g[V0][0] = c.ldw_2(S0, 0x170);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                            c.stw_2(S0, 0x170, c.g[V0][0]);
                            break 'b_2a6390;
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.ldw_2(S0, 0xd4);
                    if c.g[V0][0] != 0u64 {
                        c.lq(16, c.pa(SP, 0x20));
                        break 'b_2a6418;
                    } else {
                        'b_2a63b8: {
                            c.cycles += 7;
                            c.lwc1(0, c.pa(S0, 0x188));
                            c.g[AT][0] = 0x3e800000;
                            c.mtc1(1, 1);
                            c.c_lt_s(1, 0);
                            if c.fcc() {
                                c.swc1(1, c.pa(S0, 0x188));
                                break 'b_2a63b8;
                            } else {
                                break 'b_2a63b8;
                            }
                        }
                        'b_2a640c: {
                            c.cycles += 3;
                            c.g[A1][0] = c.ldw_2(S0, 0x174);
                            let t = (c.g[A1][0] as i64) < 0;
                            c.g[V0][0] = 0xffffffffffffffff;
                            if t {
                                break 'b_2a640c;
                            } else {
                                'b_2a63ec: {
                                    c.cycles += 5;
                                    c.mtc1(0, 0);
                                    c.c_eq_s(20, 0);
                                    let t = !c.fcc();
                                    c.g[V0][0] = c.ldw_2(GP, 0x608);
                                    if t {
                                        break 'b_2a63ec;
                                    } else {
                                        c.cycles += 5;
                                        c.mov_s(12, 20);
                                        c.g[A2][0] = 0x2;
                                        c.g[A0][0] = c.ldw_2(V0, 0);
                                        c.g[RA][0] = sx(0x002a63ec);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                                        c.pc = 0x002ad5f0;
                                        call_indirect(c, 0x002ad5f0);
                                        if c.pc != 0x002a63ec {
                                            return;
                                        }
                                        break 'b_2a63ec;
                                    }
                                }
                                c.cycles += 5;
                                c.g[V0][0] = 0xffffffffffffffff;
                                c.g[A0][0] = c.ldw_2(S0, 0x180);
                                c.g[V1][0] = 0x2;
                                let t = c.g[A0][0] != c.g[V1][0];
                                c.stw_2(S0, 0x174, c.g[V0][0]);
                                if t {
                                    break 'b_2a640c;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x1;
                                    c.stw_2(S0, 0x180, c.g[V0][0]);
                                    c.g[V0][0] = 0xffffffffffffffff;
                                    break 'b_2a640c;
                                }
                            }
                        }
                        c.cycles += 2;
                        c.stw_2(S0, 0x178, 0u64);
                        c.stw_2(S0, 0, c.g[V0][0]);
                        break 'b_2a6414;
                    }
                }
            }
        }
        c.cycles += 1;
        c.lq(16, c.pa(SP, 0x20));
        break 'b_2a6418;
    }
    c.cycles += 5;
    c.lq(17, c.pa(SP, 0x10));
    c.g[RA][0] = c.ldd(SP, 0);
    c.lwc1(20, c.pa(SP, 0x30));
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = ra;
    return;
}

/// `void CrowdAudio_PushPlayParams(CrowdAudio* self)` (0x2a7468): append the
/// crowd voice's play parameters to its ring buffers (the current slot is
/// `+0x1f4`, the template at `+0x1d8`), clearing the per-slot fields the
/// mixer starts from.
pub fn crowd_audio_push_play_params(c: &mut Cpu) {
    c.cycles += 132;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
    c.g[T1][0] = 0x18;
    c.sq(17, c.pa(SP, 0x10));
    c.g[A1][0] = 0u64;
    c.std(SP, 0, c.g[RA][0]);
    c.g[A2][0] = 0x8;
    c.sq(16, c.pa(SP, 0x20));
    c.g[S0][0] = c.ldw_2(A0, 0);
    c.g[V0][0] = c.ldw_2(S0, 0x1f4);
    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
    c.stw_2(S0, 0x1f4, c.g[V0][0]);
    c.g[V1][0] = c.ldw_2(S0, 0x1f4);
    c.g[V0][0] = c.ldw_2(S1, 0x20);
    c.mult(0, 3, 3, 9);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.ldl(2, c.pa(S0, 0x1e3));
    c.ldr(2, c.pa(S0, 0x1dc));
    c.ldl(4, c.pa(S0, 0x1eb));
    c.ldr(4, c.pa(S0, 0x1e4));
    c.ldl(7, c.pa(S0, 0x1f3));
    c.ldr(7, c.pa(S0, 0x1ec));
    c.sdl(2, c.pa(V1, 0x7));
    c.sdr(2, c.pa(V1, 0));
    c.sdl(4, c.pa(V1, 0xf));
    c.sdr(4, c.pa(V1, 0x8));
    c.sdl(7, c.pa(V1, 0x17));
    c.sdr(7, c.pa(V1, 0x10));
    c.g[V0][0] = c.ldw_2(S0, 0x1f4);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.stw_2(V0, 0x1fc, 0u64);
    c.g[V1][0] = c.ldw_2(S0, 0x1f4);
    c.g[V0][0] = c.ldw_2(S1, 0x34);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.stw_2(V1, 0, 0u64);
    c.g[A3][0] = c.ldw_2(S0, 0x1f4);
    c.g[A0][0] = c.ldw_2(S0, 0x1f4);
    c.g[T0][0] = c.ldw_2(S1, 0x20);
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
    c.mult(0, 4, 4, 9);
    c.g[V0][0] = c.ldw_2(S1, 0x38);
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
    c.g[V1][0] = c.ldb(A0, 0);
    c.stw_2(A3, 0, c.g[V1][0]);
    c.g[V0][0] = c.ldw_2(S0, 0x1f4);
    c.g[V1][0] = c.ldw_2(S1, 0x3c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.stw_2(V0, 0, 0u64);
    c.g[A0][0] = c.ldw_2(S0, 0x1f4);
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[RA][0] = sx(0x002a754c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    c.pc = 0x00416210;
    call_indirect(c, 0x00416210);
    if c.pc != 0x002a754c {
        return;
    }
    c.g[V1][0] = c.ldw_2(S0, 0x1f4);
    c.g[A0][0] = 0x64;
    c.g[V0][0] = c.ldw_2(S1, 0x60);
    c.g[A1][0] = 0x5a;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[A2][0] = 0x32;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A3][0] = 0x7f;
    c.stw_2(V1, 0, 0u64);
    c.g[T0][0] = 0x1;
    c.g[V0][0] = c.ldw_2(S0, 0x1f4);
    c.g[V1][0] = c.ldw_2(S1, 0x64);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.stw_2(V0, 0, 0u64);
    c.g[V1][0] = c.ldw_2(S0, 0x1f4);
    c.g[V0][0] = c.ldw_2(S1, 0x68);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.stw_2(V1, 0, 0u64);
    c.g[V0][0] = c.ldw_2(S0, 0x1f4);
    c.g[V1][0] = c.ldw_2(S1, 0x6c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.stw_2(V0, 0, 0u64);
    c.g[V1][0] = c.ldw_2(S0, 0x1f4);
    c.g[V0][0] = c.ldw_2(S1, 0x70);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.stw_2(V1, 0x4, 0u64);
    c.g[V0][0] = c.ldw_2(S0, 0x1f4);
    c.g[V1][0] = c.ldw_2(S1, 0x70);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.stw_2(V0, 0, c.g[A0][0]);
    c.g[V1][0] = c.ldw_2(S0, 0x1f4);
    c.g[V0][0] = c.ldw_2(S1, 0x70);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.stw_2(V1, 0x8, c.g[A1][0]);
    c.g[V0][0] = c.ldw_2(S0, 0x1f4);
    c.g[V1][0] = c.ldw_2(S1, 0x70);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.stw_2(V0, 0xc, c.g[A2][0]);
    c.g[V1][0] = c.ldw_2(S0, 0x1f4);
    c.g[V0][0] = c.ldw_2(S1, 0x74);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.stw_2(V1, 0, 0u64);
    c.g[V0][0] = c.ldw_2(S0, 0x1f4);
    c.g[V1][0] = c.ldw_2(S1, 0x80);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.stw_2(V0, 0, 0u64);
    c.g[V1][0] = c.ldw_2(S0, 0x1f4);
    c.g[V0][0] = c.ldw_2(S1, 0x84);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.stw_2(V1, 0, 0u64);
    c.g[V0][0] = c.ldw_2(S0, 0x1f4);
    c.g[V1][0] = c.ldw_2(S1, 0x78);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.stw_2(V0, 0, c.g[A3][0]);
    c.g[V1][0] = c.ldw_2(S0, 0x1f4);
    c.g[V0][0] = c.ldw_2(S1, 0x7c);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.lq(16, c.pa(SP, 0x20));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.lq(17, c.pa(SP, 0x10));
    c.g[RA][0] = c.ldd(SP, 0);
    c.stw_2(V1, 0, c.g[T0][0]);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    c.pc = ra;
    return;
}

// ---------------------------------------------------------------------------
// MIDI play-parameter ring buffer.

/// `MIDIBanks_PushPlayParams(Bank *banks, const PlayParams *p, int channel)`
/// (0x2a8450): append one play-parameter record to the bank's ring buffers.
///
/// `banks->first` (offset 0) is the track object; `+0x1f4` is the write
/// cursor, `+0x1dc..0x1f4` the per-call parameter copies, `+0x1fc` the
/// record queue and `first+0x1d8` an array descriptor whose `+0x20` count
/// wraps the cursor.  The parallel arrays at `+0x34..0x84` hold the
/// per-record fields.
pub fn midi_banks_push_play_params(c: &mut Cpu) {
    c.cycles += 153;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
    c.g[T1][0] = 0x18;
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[S2][0] = c.g[A0][0];
    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[S3][0] = c.g[A1][0];
    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
    c.g[S4][0] = c.g[A2][0];
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[A1][0] = 0u64;
    c.g[S0][0] = c.lw(c.g[S2][0] as u32);
    c.g[A2][0] = 0x8;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
    c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
    c.mult(0, 3, 3, 9);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1e3));
    c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1dc));
    c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x1eb));
    c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x1e4));
    c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1f3));
    c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1ec));
    c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
    c.sdr(2, c.g[V1][0] as u32);
    c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
    c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
    c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
    c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0u64);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sw(c.g[V1][0] as u32, 0u64);
    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
    c.mult(0, 4, 4, 9);
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
    c.g[V1][0] = c.lb(c.g[A0][0] as u32);
    c.sw(c.g[A3][0] as u32, c.g[V1][0]);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sw(c.g[V0][0] as u32, 0u64);
    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[RA][0] = sx(0x002a854c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    c.pc = 0x00416210;
    call_indirect(c, 0x00416210);
    if c.pc != 0x002a854c {
        return;
    }
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[A0][0] = 0x64;
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
    c.g[A1][0] = 0x5a;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[A2][0] = 0x32;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A3][0] = 0x7f;
    c.sw(c.g[V1][0] as u32, 0u64);
    c.g[T0][0] = 0x1;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sw(c.g[V0][0] as u32, 0u64);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sw(c.g[V1][0] as u32, 0u64);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sw(c.g[V0][0] as u32, 0u64);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sw(c.g[V0][0] as u32, c.g[A0][0]);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sw(c.g[V1][0] as u32, 0u64);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sw(c.g[V0][0] as u32, 0u64);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sw(c.g[V1][0] as u32, 0u64);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sw(c.g[V0][0] as u32, c.g[A3][0]);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.sw(c.g[V1][0] as u32, c.g[T0][0]);
    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
    c.g[A0][0] = c.lw(c.g[S2][0] as u32);
    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sw(c.g[V0][0] as u32, c.g[S3][0]);
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[V1][0] = c.lw(c.g[S2][0] as u32);
    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[S4][0]);
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Audio: play a sound.

/// `SSXAudioSystem_PlaySound(self, int bankId, int soundId)` (0x290cc0):
/// queue one sound.  It is the same ring-buffer push as
/// [`midi_banks_push_play_params`], then the bank's channel state gives the
/// gain slot and the sound is played.
pub fn ssx_audio_system_play_sound(c: &mut Cpu) {
    c.cycles += 166;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
    c.g[T1][0] = 0x18;
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[S2][0] = c.g[A0][0];
    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[S3][0] = c.g[A1][0];
    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
    c.g[S4][0] = c.g[A2][0];
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[A1][0] = 0u64;
    c.g[S0][0] = c.lw(c.g[S2][0] as u32);
    c.g[A2][0] = 0x8;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
    c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
    c.mult(0, 3, 3, 9);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1e3));
    c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1dc));
    c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x1eb));
    c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x1e4));
    c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1f3));
    c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1ec));
    c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
    c.sdr(2, c.g[V1][0] as u32);
    c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
    c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
    c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
    c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0u64);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sw(c.g[V1][0] as u32, 0u64);
    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
    c.mult(0, 4, 4, 9);
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
    c.g[V1][0] = c.lb(c.g[A0][0] as u32);
    c.sw(c.g[A3][0] as u32, c.g[V1][0]);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sw(c.g[V0][0] as u32, 0u64);
    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[RA][0] = sx(0x00290dbc);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    c.pc = 0x00416210;
    call_indirect(c, 0x00416210);
    if c.pc != 0x00290dbc {
        return;
    }
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[A3][0] = 0x64;
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
    c.g[T0][0] = 0x5a;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[T1][0] = 0x32;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[T2][0] = 0x7f;
    c.sw(c.g[V1][0] as u32, 0u64);
    c.g[T3][0] = 0x1;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = 0x5;
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[A2][0] = 0u64;
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sw(c.g[V1][0] as u32, 0u64);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sw(c.g[V0][0] as u32, 0u64);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sw(c.g[V1][0] as u32, 0u64);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sw(c.g[V1][0] as u32, c.g[A3][0]);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[T1][0]);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sw(c.g[V0][0] as u32, 0u64);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sw(c.g[V1][0] as u32, 0u64);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sw(c.g[V0][0] as u32, 0u64);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sw(c.g[V1][0] as u32, c.g[T2][0]);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sw(c.g[V0][0] as u32, c.g[T3][0]);
    c.g[V1][0] = c.lw(c.g[S2][0] as u32);
    c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
    c.sw(c.g[V0][0] as u32, c.g[S3][0]);
    c.g[V1][0] = c.lw(c.g[S2][0] as u32);
    c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
    c.sw(c.g[V0][0] as u32, c.g[S4][0]);
    c.g[RA][0] = sx(0x00290f18);
    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x118));
    c.pc = 0x00287968;
    ee::s07::SSXAudioSystem_GetChannelGainPtr(c);
    if c.pc != 0x00290f18 {
        return;
    }
    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
    c.g[A0][0] = c.g[S2][0];
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
    c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x214));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[RA][0] = sx(0x00290f38);
    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
    c.pc = 0x002906b8;
    ee::s07::Audio_PlayPushedSound(c);
    if c.pc != 0x00290f38 {
        return;
    }
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Audio: rail-change sound.

/// `SSXAudioSystem_PlayRailChangeSound(self, Rider *rider)` (0x299b70):
/// while a replay is playing, or before the rider has a rail sound pending,
/// do nothing; otherwise push a play record (sound 9) and start it.
pub fn ssx_audio_system_play_rail_change_sound(c: &mut Cpu) {
    c.cycles += 11;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.g[S3][0] = c.g[A0][0];
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
    c.g[RA][0] = sx(0x00299b94);
    c.g[S0][0] = c.g[A1][0];
    c.pc = 0x00288ae0;
    call_indirect(c, 0x00288ae0);
    if c.pc != 0x00299b94 {
        return;
    }
    if c.g[V0][0] != 0 {
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    } else {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x874));
        c.g[A1][0] = 0u64;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x87c));
            c.g[A1][0] = (0u64 < c.g[V0][0]) as u64;
        }
        c.cycles += 2;
        c.g[T1][0] = 0x18;
        if c.g[A1][0] != 0 {
            c.cycles += 148;
            c.g[S0][0] = c.lw(c.g[S3][0] as u32);
            c.g[A1][0] = 0u64;
            c.g[A2][0] = 0x8;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
            c.g[S2][0] = 0x6c;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
            c.mult(0, 3, 3, 9);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1e3));
            c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1dc));
            c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x1eb));
            c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x1e4));
            c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1f3));
            c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1ec));
            c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
            c.sdr(2, c.g[V1][0] as u32);
            c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
            c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
            c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
            c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0u64);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw(c.g[V1][0] as u32, 0u64);
            c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
            c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
            c.mult(0, 4, 4, 9);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
            c.g[V1][0] = c.lb(c.g[A0][0] as u32);
            c.sw(c.g[A3][0] as u32, c.g[V1][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw(c.g[V0][0] as u32, 0u64);
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            c.g[RA][0] = sx(0x00299c8c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
            c.pc = 0x00416210;
            call_indirect(c, 0x00416210);
            if c.pc != 0x00299c8c {
                return;
            }
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[A3][0] = 0x64;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
            c.g[T0][0] = 0x5a;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[T1][0] = 0x32;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[T2][0] = 0x7f;
            c.sw(c.g[V1][0] as u32, 0u64);
            c.g[T3][0] = 0x1;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = 0x9;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[A2][0] = 0u64;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw(c.g[V1][0] as u32, 0u64);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw(c.g[V0][0] as u32, 0u64);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw(c.g[V1][0] as u32, 0u64);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw(c.g[V1][0] as u32, c.g[A3][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[T1][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw(c.g[V0][0] as u32, 0u64);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw(c.g[V1][0] as u32, 0u64);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw(c.g[V0][0] as u32, 0u64);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw(c.g[V1][0] as u32, c.g[T2][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw(c.g[V0][0] as u32, c.g[T3][0]);
            c.g[V1][0] = c.lw(c.g[S3][0] as u32);
            c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
            c.sw(c.g[V0][0] as u32, 0u64);
            c.g[V1][0] = c.lw(c.g[S3][0] as u32);
            c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
            c.sw(c.g[V0][0] as u32, c.g[S2][0]);
            c.g[RA][0] = sx(0x00299de8);
            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
            c.pc = 0x00287968;
            call_indirect(c, 0x00287968);
            if c.pc != 0x00299de8 {
                return;
            }
            c.g[A1][0] = c.lw(c.g[S0][0] as u32);
            c.g[A0][0] = c.g[S3][0];
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
            c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x214));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
            c.g[RA][0] = sx(0x00299e08);
            c.sw(c.g[V1][0] as u32, c.g[V0][0]);
            c.pc = 0x002906b8;
            call_indirect(c, 0x002906b8);
            if c.pc != 0x00299e08 {
                return;
            }
        }
        c.cycles += 1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    }
    c.cycles += 6;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    abi::ret(c);
}
// ---------------------------------------------------------------------------
// Audio: looping effect.

/// `SSXAudioSystem_StartLoopSfx(self)` (0x29d370): if the looping-effect
/// slot at `self+0x6c74` is armed, push a play record (sound 9) and start it,
/// store the handle at `+0x5fc8`; set the active flag at `+0x5fc4`.
pub fn ssx_audio_system_start_loop_sfx(c: &mut Cpu) {
    c.cycles += 10;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.g[S3][0] = c.g[A0][0];
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x6c74));
    c.g[T1][0] = 0x18;
    if c.g[V0][0] != 0 {
        c.cycles += 149;
        c.g[S0][0] = c.lw(c.g[S3][0] as u32);
        c.g[A1][0] = 0u64;
        c.g[A2][0] = 0x8;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
        c.g[S2][0] = 0x9;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
        c.mult(0, 3, 3, 9);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1e3));
        c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1dc));
        c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x1eb));
        c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x1e4));
        c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1f3));
        c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1ec));
        c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
        c.sdr(2, c.g[V1][0] as u32);
        c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
        c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
        c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
        c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw(c.g[V1][0] as u32, 0u64);
        c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
        c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
        c.mult(0, 4, 4, 9);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
        c.g[V1][0] = c.lb(c.g[A0][0] as u32);
        c.sw(c.g[A3][0] as u32, c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw(c.g[V0][0] as u32, 0u64);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[RA][0] = sx(0x0029d46c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
        c.pc = 0x00416210;
        call_indirect(c, 0x00416210);
        if c.pc != 0x0029d46c {
            return;
        }
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[A3][0] = 0x64;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
        c.g[T0][0] = 0x5a;
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[T1][0] = 0x32;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[T2][0] = 0x7f;
        c.sw(c.g[V1][0] as u32, 0u64);
        c.g[T3][0] = 0x1;
        c.g[A1][0] = 0x5;
        c.g[A2][0] = 0u64;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[A0][0] = c.g[S3][0];
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw(c.g[V1][0] as u32, 0u64);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw(c.g[V0][0] as u32, 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw(c.g[V1][0] as u32, 0u64);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw(c.g[V1][0] as u32, c.g[A3][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[T1][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw(c.g[V0][0] as u32, 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw(c.g[V1][0] as u32, 0u64);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw(c.g[V0][0] as u32, 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw(c.g[V1][0] as u32, c.g[T2][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw(c.g[V0][0] as u32, c.g[T3][0]);
        c.g[V1][0] = c.lw(c.g[S3][0] as u32);
        c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
        c.sw(c.g[V0][0] as u32, c.g[S2][0]);
        c.g[V1][0] = c.lw(c.g[S3][0] as u32);
        c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
        c.sw(c.g[V0][0] as u32, 0u64);
        c.g[RA][0] = sx(0x0029d5c8);
        c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
        c.pc = 0x00287968;
        call_indirect(c, 0x00287968);
        if c.pc != 0x0029d5c8 {
            return;
        }
        c.g[A1][0] = c.lw(c.g[S0][0] as u32);
        c.g[A0][0] = c.g[S3][0];
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
        c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x214));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[RA][0] = sx(0x0029d5e8);
        c.sw(c.g[V1][0] as u32, c.g[V0][0]);
        c.pc = 0x002906b8;
        call_indirect(c, 0x002906b8);
        if c.pc != 0x0029d5e8 {
            return;
        }
        c.sw((c.g[S3][0] as u32).wrapping_add(0x5fc8), c.g[V0][0]);
    }
    c.cycles += 9;
    c.g[V0][0] = 0x1;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.sw((c.g[S3][0] as u32).wrapping_add(0x5fc4), c.g[V0][0]);
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
    abi::ret(c);
}

/// `void func_0028FEA0(cMIDIBanks* self, int flags)` (0x28fea0): destroying
/// destructor of the MIDI bank object: install the base vtables, free the
/// two owned buffers, clear the global, and for `flags & 2` tear down the
/// sound manager, for `flags & 1` free `self`.
pub fn midi_banks_base_dtor(c: &mut Cpu) {
    let frame = Frame::open(c, 0x90);
    'b_28ff9c: {
        c.cycles += 23;
        c.g[V0][0] = 0x480000;
        frame.save(c, S0, 0x80);
        c.g[V0][0] = sx(at_4(c, V0, 0x3a48));
        frame.save(c, S1, 0x70);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x60);
        c.g[A3][0] = 0x480000;
        sw(c, at_4(c, S0, 0xc), c.g[V0][0]);
        c.g[A2][0] = sx(at_4(c, A3, 0x3a60));
        c.g[V1][0] = lw(c, r32(c, S0));
        c.g[A0][0] = 0x480000;
        c.g[A0][0] = sx(at_4(c, A0, 0x3b48));
        c.g[T0][0] = 0x480000;
        c.g[V0][0] = lw(c, at_4(c, V1, 0x1d8));
        c.g[T5][0] = sx(at_4(c, T0, 0x3a80));
        c.g[S1][0] = c.g[A1][0];
        sw(c, at_4(c, V0, 0x4), c.g[A2][0]);
        c.g[V0][0] = lw(c, r32(c, S0));
        sw(c, at_4(c, V0, 0xab0), c.g[A0][0]);
        c.g[V1][0] = lw(c, r32(c, S0));
        let t = c.g[S1][0] != 0u64;
        sw(c, at_4(c, V1, 0x1d4), c.g[T5][0]);
        if t {
            break 'b_28ff9c;
        } else {
            c.cycles += 40;
            c.g[A1][0] = ld(c, at_4(c, A3, 0x3a60));
            c.g[T4][0] = sx(at_4(c, SP, 0x20));
            c.g[V1][0] = ld(c, at_4(c, A2, 0x10));
            c.g[A0][0] = ld(c, at_4(c, A2, 0x18));
            c.g[V0][0] = ld(c, at_4(c, A2, 0x8));
            sd(c, r32(c, SP), c.g[A1][0]);
            sd(c, at_4(c, SP, 0x18), c.g[A0][0]);
            sd(c, at_4(c, SP, 0x10), c.g[V1][0]);
            sd(c, at_4(c, SP, 0x8), c.g[V0][0]);
            c.g[V0][0] = lw(c, r32(c, S0));
            c.g[A0][0] = lhu(c, at_4(c, A2, 0x8));
            c.g[V1][0] = lw(c, at_4(c, V0, 0x1d8));
            c.g[T3][0] = ld(c, at_4(c, T0, 0x3a80));
            sw(c, at_4(c, V1, 0x4), c.g[SP][0]);
            c.g[V1][0] = ld(c, at_4(c, T5, 0x28));
            c.g[T1][0] = lw(c, r32(c, S0));
            c.g[A3][0] = ld(c, at_4(c, T5, 0x8));
            c.g[V0][0] = lw(c, at_4(c, T1, 0x1d8));
            c.g[T2][0] = ld(c, at_4(c, T5, 0x10));
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x10));
            c.g[T0][0] = ld(c, at_4(c, T5, 0x18));
            c.g[V0][0] = sx(r32(c, S0).wrapping_sub(r32(c, V0)));
            c.g[A2][0] = ld(c, at_4(c, T5, 0x20));
            c.g[A1][0] = ld(c, at_4(c, T5, 0x30));
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
            sd(c, at_4(c, SP, 0x28), c.g[A3][0]);
            sd(c, at_4(c, SP, 0x48), c.g[V1][0]);
            sh(c, at_4(c, SP, 0x8), c.g[A0][0]);
            sd(c, at_4(c, SP, 0x20), c.g[T3][0]);
            sd(c, at_4(c, SP, 0x30), c.g[T2][0]);
            sd(c, at_4(c, SP, 0x38), c.g[T0][0]);
            sd(c, at_4(c, SP, 0x40), c.g[A2][0]);
            sd(c, at_4(c, SP, 0x50), c.g[A1][0]);
            sw(c, at_4(c, T1, 0x1d4), c.g[T4][0]);
            c.g[V1][0] = lhu(c, at_4(c, T5, 0x8));
            c.g[V0][0] = lw(c, r32(c, S0));
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x18));
            c.g[V0][0] = sx(r32(c, S0).wrapping_sub(r32(c, V0)));
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            sh(c, at_4(c, SP, 0x28), c.g[V1][0]);
            break 'b_28ff9c;
        }
    }
    'b_28ffb4: {
        c.cycles += 3;
        c.g[A0][0] = lw(c, at_4(c, S0, 0x4));
        if c.g[A0][0] == 0u64 {
            c.g[A0][0] = lw(c, at_4(c, S0, 0x8));
            break 'b_28ffb4;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0028ffb0);
            if call_ptr(c, 0x00317e98, 0x0028ffb0).is_err() {
                return;
            }
            c.g[A0][0] = lw(c, at_4(c, S0, 0x8));
            break 'b_28ffb4;
        }
    }
    'b_28ffc8: {
        c.cycles += 2;
        let t = c.g[A0][0] == 0u64;
        c.g[V0][0] = c.g[S1][0] & 0x2;
        if t {
            break 'b_28ffc8;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0028ffc4);
            if call_ptr(c, 0x00317e98, 0x0028ffc4).is_err() {
                return;
            }
            c.g[V0][0] = c.g[S1][0] & 0x2;
            break 'b_28ffc8;
        }
    }
    'b_28ffec: {
        c.cycles += 2;
        let t = c.g[V0][0] == 0u64;
        sw(c, at_4(c, GP, 0x574), 0u64);
        if t {
            break 'b_28ffec;
        } else {
            c.cycles += 7;
            c.g[A0][0] = lw(c, r32(c, S0));
            c.g[RA][0] = sx(0x0028ffdc);
            c.g[A1][0] = 0u64;
            if callee::C0028BB10.call(c, 0x0028ffdc).is_err() {
                return;
            }
            c.g[V0][0] = lw(c, r32(c, S0));
            c.g[A1][0] = 0u64;
            c.g[RA][0] = sx(0x0028ffec);
            c.g[A0][0] = lw(c, at_4(c, V0, 0x1d8));
            if call_ptr(c, 0x002adee8, 0x0028ffec).is_err() {
                return;
            }
            break 'b_28ffec;
        }
    }
    'b_290004: {
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 0x1;
        if c.g[V0][0] == 0u64 {
            frame.restore(c, S0, 0x80);
            break 'b_290004;
        } else {
            c.cycles += 3;
            c.g[RA][0] = sx(0x00290000);
            c.g[A0][0] = c.g[S0][0];
            if call_ptr(c, 0x00317e50, 0x00290000).is_err() {
                return;
            }
            frame.restore(c, S0, 0x80);
            break 'b_290004;
        }
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x70);
    c.g[RA][0] = ld(c, at_4(c, SP, 0x60));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_4(c, SP, 0x90));
    c.pc = t;
    return;
}

/// `void func_00292B48(void* self)` (0x292b48): stop the board loop sounds:
/// stop the ambience loop, send the six "stop loop" sound keys, and stop the
/// handle at_4 `+0x59e0` when its count is positive.
pub fn ssx_audio_system_stop_board_loops(c: &mut Cpu) {
    let frame = Frame::open(c, 0x20);
    'b_292c50: {
        c.cycles += 59;
        frame.save(c, S0, 0x10);
        sd(c, r32(c, SP), c.g[RA][0]);
        c.g[RA][0] = sx(0x00292b5c);
        c.g[S0][0] = c.g[A0][0];
        if callee::C002913D8.call(c, 0x00292b5c).is_err() {
            return;
        }
        c.g[V1][0] = lw(c, at_4(c, S0, 0x118));
        c.g[V0][0] = 0x4a0000;
        ldl(c, 5, at_4(c, V0, 0x3688));
        c.g[A0][0] = lw(c, r32(c, V1));
        c.g[V0][0] = lw(c, at_4(c, A0, 0xab0));
        c.g[A0][0] = sx(at_4(c, A0, 0x1d8));
        c.g[V1][0] = lh(c, at_4(c, V0, 0x18));
        c.g[A2][0] = lw(c, at_4(c, V0, 0x1c));
        let t = c.g[A2][0] as u32;
        c.g[RA][0] = sx(0x00292b88);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
        if call_ptr(c, t, 0x00292b88).is_err() {
            return;
        }
        c.g[V1][0] = lw(c, at_4(c, S0, 0x118));
        c.g[V0][0] = 0x4a0000;
        ldl(c, 5, at_4(c, V0, 0x3690));
        c.g[A0][0] = lw(c, r32(c, V1));
        c.g[V0][0] = lw(c, at_4(c, A0, 0xab0));
        c.g[A0][0] = sx(at_4(c, A0, 0x1d8));
        c.g[V1][0] = lh(c, at_4(c, V0, 0x18));
        c.g[A2][0] = lw(c, at_4(c, V0, 0x1c));
        let t = c.g[A2][0] as u32;
        c.g[RA][0] = sx(0x00292bb4);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
        if call_ptr(c, t, 0x00292bb4).is_err() {
            return;
        }
        c.g[V1][0] = lw(c, at_4(c, S0, 0x118));
        c.g[V0][0] = 0x4a0000;
        ldl(c, 5, at_4(c, V0, 0x3698));
        c.g[A0][0] = lw(c, r32(c, V1));
        c.g[V0][0] = lw(c, at_4(c, A0, 0xab0));
        c.g[A0][0] = sx(at_4(c, A0, 0x1d8));
        c.g[V1][0] = lh(c, at_4(c, V0, 0x18));
        c.g[A2][0] = lw(c, at_4(c, V0, 0x1c));
        let t = c.g[A2][0] as u32;
        c.g[RA][0] = sx(0x00292be0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
        if call_ptr(c, t, 0x00292be0).is_err() {
            return;
        }
        c.g[V1][0] = lw(c, at_4(c, S0, 0x118));
        c.g[V0][0] = 0x4a0000;
        ldl(c, 5, at_4(c, V0, 0x36a0));
        c.g[A0][0] = lw(c, r32(c, V1));
        c.g[V0][0] = lw(c, at_4(c, A0, 0xab0));
        c.g[A0][0] = sx(at_4(c, A0, 0x1d8));
        c.g[V1][0] = lh(c, at_4(c, V0, 0x18));
        c.g[A2][0] = lw(c, at_4(c, V0, 0x1c));
        let t = c.g[A2][0] as u32;
        c.g[RA][0] = sx(0x00292c0c);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
        if call_ptr(c, t, 0x00292c0c).is_err() {
            return;
        }
        c.g[V1][0] = lw(c, at_4(c, S0, 0x118));
        c.g[V0][0] = 0x4a0000;
        ldl(c, 5, at_4(c, V0, 0x36a8));
        c.g[A0][0] = lw(c, r32(c, V1));
        c.g[RA][0] = sx(0x00292c28);
        c.g[A0][0] = sx(at_4(c, A0, 0x1d8));
        if call_ptr(c, 0x002adde0, 0x00292c28).is_err() {
            return;
        }
        c.g[V0][0] = lw(c, at_4(c, S0, 0x59e4));
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[A2][0] = 0x1;
        if t {
            break 'b_292c50;
        } else {
            c.cycles += 7;
            c.g[V0][0] = lw(c, at_4(c, S0, 0x118));
            c.g[A1][0] = lw(c, at_4(c, S0, 0x59e0));
            c.g[A0][0] = lw(c, r32(c, V0));
            c.mtc1(0, 12);
            c.g[RA][0] = sx(0x00292c4c);
            c.g[A0][0] = sx(at_4(c, A0, 0x1d8));
            if call_ptr(c, 0x002ad5f0, 0x00292c4c).is_err() {
                return;
            }
            sw(c, at_4(c, S0, 0x59e4), 0u64);
            break 'b_292c50;
        }
    }
    c.cycles += 26;
    c.g[V1][0] = lw(c, at_4(c, S0, 0x118));
    c.g[V0][0] = 0x4a0000;
    ldl(c, 5, at_4(c, V0, 0x36b0));
    c.g[A0][0] = lw(c, r32(c, V1));
    c.g[V0][0] = lw(c, at_4(c, A0, 0xab0));
    c.g[A0][0] = sx(at_4(c, A0, 0x1d8));
    c.g[V1][0] = lh(c, at_4(c, V0, 0x18));
    c.g[A2][0] = lw(c, at_4(c, V0, 0x1c));
    let t = c.g[A2][0] as u32;
    c.g[RA][0] = sx(0x00292c7c);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
    if call_ptr(c, t, 0x00292c7c).is_err() {
        return;
    }
    c.g[V1][0] = lw(c, at_4(c, S0, 0x118));
    c.g[V0][0] = 0x4a0000;
    ldl(c, 5, at_4(c, V0, 0x36b8));
    c.g[A0][0] = lw(c, r32(c, V1));
    c.g[V0][0] = lw(c, at_4(c, A0, 0xab0));
    c.g[A0][0] = sx(at_4(c, A0, 0x1d8));
    c.g[V1][0] = lh(c, at_4(c, V0, 0x18));
    c.g[A2][0] = lw(c, at_4(c, V0, 0x1c));
    let t = c.g[A2][0] as u32;
    c.g[RA][0] = sx(0x00292ca8);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
    if call_ptr(c, t, 0x00292ca8).is_err() {
        return;
    }
    frame.restore(c, S0, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_4(c, SP, 0x20));
    c.pc = t;
    return;
}

/// `void SSXAudioSystem_PASay20C5(SSXAudioSystem *self, Rider *rider)`
/// (0x2a3708): when PA speech is on and the rider is in a race, either queue
/// the 1-second bank timer or queue the finish speech (language, voice and
/// finish masks) as event context 0x20c5.
pub fn ssx_audio_system_pa_say20c5(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 19, 0x10);
        c.g[S0][0] = abi::arg64(c, 0);
        frame.save(c, 17, 0x30);
        frame.save(c, 18, 0x20);
        frame.save_ra(c, 0);
        c.g[S3][0] = abi::arg64(c, 1);

        callee::IS_PA_SPEECH_ON.call(c, 0x002a_372c)?;
        let mut out = c.g[V0][0] == 0;
        if !out {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            callee::IS_REPLAY_PLAYING.call(c, 0x002a_373c)?;
            out = c.g[V0][0] != 0;
        }
        if !out {
            c.cycles += 4;
            c.g[A0][0] = c.g[S3][0];
            callee::RIDER_123168.call(c, 0x002a_374c)?;
            out = c.g[V0][0] != 0;
        }
        let mut bank_path = false;
        if !out {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            callee::IS_RACE_TYPE_4OR5.call(c, 0x002a_375c)?;
            let not45 = c.g[V0][0] == 0;
            c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x6c0));
            bank_path = !not45;
        }
        if bank_path {
            c.cycles += 22;
            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x6c0));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x118));
            c.g[S1][0] = 0x004a_0000;
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x36e8));
            c.g[S0][0] = c.lw(c.g[V0][0] as u32);
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            let f = c.g[V0][0] as u32;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
            crate::call::call_ptr(c, f, 0x002a_378c)?;
            c.ldl(6, (c.g[S1][0] as u32).wrapping_add(7));
            c.ldr(6, c.g[S1][0] as u32);
            c.g[A0][0] = c.g[S0][0];
            c.g[T0][0] = c.g[V0][0];
            c.g[A1][0] = 0x3e8;
            c.g[A3][0] = 0;
            c.g[T1][0] = 0;
            c.g[T2][0] = 0;
            c.g[T3][0] = 0;
            callee::BANK_MONITOR_ADD_TIMER_2.call(c, 0x002a_37b4)?;
            frame.restore(c, 16, 0x40);
        } else if !out {
            c.cycles += 11;
            c.mtc1(0, 12); // fourth float argument = 0.0
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5560));
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x20c5;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0xb;
            c.g[T1][0] = 0;
            c.g[T2][0] = 0;
            callee::SPEECH_SET_EVENT_CONTEXT_2.call(c, 0x002a_37e0)?;
            if c.g[V0][0] != 0 {
                c.cycles += 23;
                c.g[A0][0] = c.g[S0][0];
                callee::GET_LANGUAGE_MASK_3.call(c, 0x002a_37f0)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[S2][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S3][0];
                callee::GET_RIDER_VOICE_MASK.call(c, 0x002a_3800)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S3][0];
                callee::GET_FINISH_SPEECH_MASK.call(c, 0x002a_3810)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = 1;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x20c5;
                callee::SPEECH_QUEUE.call(c, 0x002a_3824)?;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x173c));
                c.g[A0][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S2][0];
                c.g[A3][0] = c.g[S1][0];
                c.g[T0][0] = c.g[S0][0];
                let f = c.g[V1][0] as u32;
                c.g[A1][0] = 3;
                crate::call::call_ptr(c, f, 0x002a_3840)?;
                frame.restore(c, 16, 0x40);
            }
        }

        c.cycles += 6;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod tracked {

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

pub(crate) const BANK_MONITOR_FIND_INSTANCE: Callee = Callee {
    addr: 0x002a_d550,
    f: ee::s08::BankMonitor_FindInstance,
};

pub(crate) const BANK_MONITOR_STOP_HANDLE: Callee = Callee {
    addr: 0x002a_d5f0,
    f: ee::s08::BankMonitor_StopHandle,
};

/// Pop one word of the ring (-1 when empty). Registers on entry: `a0` =
/// write index, `a1` = read index (the first pop loads them, with `v1` =
/// write); the last pop does not reload them.
pub(crate) fn tracked_pop(c: &mut Cpu, this: u32, first: bool, last: bool) -> u64 {
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
pub(crate) fn tracked_push(c: &mut Cpu, this: u32, value: u64, first: bool, last: bool) -> bool {
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
pub(crate) fn tracked_record(c: &mut Cpu, this: u32, rec: [u64; 5]) -> crate::call::Flow {
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

pub(crate) mod pending_speech {

    pub const DJ_20E5: u32 = 0x5768;

    pub const DJ_212B: u32 = 0x5784;

    pub const DJ_20BA: u32 = 0x576c;

    pub const DJ_212C: u32 = 0x577c;

    pub const DJ_212D: u32 = 0x5770;

    pub const RIDER_20D3: u32 = 0x5788;

    /// DJ line 0x20cf with its argument at `+0x5778`.
    pub const DJ_20CF: u32 = 0x5774;

    pub const DJ_20CF_ARG: u32 = 0x5778;

    /// DJ chatter (kind 0, 0, 1).
    pub const CHATTER_A: u32 = 0x574c;

    pub const CHATTER_B: u32 = 0x5750;

    pub const CHATTER_C: u32 = 0x5754;

    /// DJ line 0x20cc with its argument at `+0x575c`.
    pub const DJ_20CC: u32 = 0x5758;

    pub const DJ_20CC_ARG: u32 = 0x575c;

    pub const DJ_20E6: u32 = 0x5740;

    /// Rider line 0x20b9 (independent of the others), for the race
    /// manager's rider number `+0x5748` (riders at race `+0x28`).
    pub const RIDER_20B9: u32 = 0x5744;

    pub const RIDER_20B9_INDEX: u32 = 0x5748;

    pub const RACE_RIDERS: u32 = 0x28;
}

pub(crate) mod speech_callee {
    use super::ee;
    use super::Callee;

    macro_rules! callee {
        ($name:ident, $addr:expr, $f:ident) => {
            pub const $name: Callee = Callee {
                addr: $addr,
                f: ee::s08::$f,
            };
        };
    }

    callee!(DJ_SAY_20E5, 0x002a_26f0, SSXAudioSystem_DJSay20E5);

    callee!(DJ_SAY_212B, 0x002a_2ad0, SSXAudioSystem_DJSay212B);

    callee!(DJ_SAY_20BA, 0x002a_2638, SSXAudioSystem_DJSay20BA);

    callee!(DJ_SAY_212C, 0x002a_2b88, SSXAudioSystem_DJSay212C);

    callee!(DJ_SAY_212D, 0x002a_2c30, SSXAudioSystem_DJSay212D);

    callee!(RIDER_SAY_20D3, 0x002a_1138, SSXAudioSystem_RiderSay20D3);

    callee!(DJ_SAY_20CF, 0x002a_2860, SSXAudioSystem_DJSay20CF);

    callee!(PLAY_DJ_CHATTER, 0x002a_2e50, SSXAudioSystem_PlayDJChatter);

    callee!(DJ_SAY_20CC, 0x002a_2568, SSXAudioSystem_DJSay20CC_2);

    callee!(DJ_SAY_20E6, 0x002a_27d8, SSXAudioSystem_DJSay20E6);

    callee!(RIDER_SAY_20B9, 0x002a_1400, SSXAudioSystem_RiderSay20B9);

    /// `SSXAudioSystem_GetRaceMan` (through the function table).
    pub const GET_RACE_MAN: u32 = 0x0028_b1d8;
}

pub(crate) mod object_loop {

    pub const SLOTS: u32 = 0x5a00;

    pub const SLOT_SIZE: u64 = 0x30;

    pub const COUNT: i64 = 30;

    /// Slot: active (`+0`, 1), the sound object (`+4`; its scene node at
    /// `+0xc`, its own position at `+0x40`), the node's bone (`+8`, -1 the
    /// node itself), the emitter position (`+0x10`, a quadword), the loop
    /// handle (`+0x24`).
    pub const ACTIVE: u32 = 0x0;

    pub const OBJECT: u32 = 0x4;

    pub const BONE: u32 = 0x8;

    pub const POSITION: u32 = 0x10;

    pub const HANDLE: u32 = 0x24;

    pub const OBJ_NODE: u32 = 0xc;

    pub const OBJ_POSITION: u32 = 0x40;

    /// Node vtable (`+0xc`) slots: the world matrix (`+0xc0`), "has bones"
    /// (`+0xd0`), a bone's matrix (`+0xe8`); matrices hold the translation
    /// at `+0x30`.
    pub const NODE_VTABLE: u32 = 0xc;

    pub const VT_MATRIX: u32 = 0xc0;

    pub const VT_HAS_BONES: u32 = 0xd0;

    pub const VT_BONE_MATRIX: u32 = 0xe8;

    pub const TRANSLATION: u32 = 0x30;

    /// The audio system's channel table (`+0x118`; the bank monitor at
    /// `*table + 0x1d8`).
    pub const CHANNELS: u32 = 0x118;

    pub const BANK_MONITOR: u32 = 0x1d8;
}

pub(crate) mod object_loop_callee {
    use super::ee;
    use super::Callee;

    pub const STOP_HANDLE: Callee = Callee {
        addr: 0x002a_d5f0,
        f: ee::s08::BankMonitor_StopHandle,
    };
}

/// `void SSXAudioSystem_FlushPendingSpeech(SSXAudioSystem* self)`
/// (0x2a43b8): play the first pending DJ/rider line (in priority order) and
/// clear its request; then, independently, the pending rider line 0x20b9
/// for the race manager's rider (if that rider exists).
pub fn ssx_audio_system_flush_pending_speech(c: &mut Cpu) {
    use pending_speech::*;
    use speech_callee::*;
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        let at = |off: u32| Addr(this.wrapping_add(off));
        let me = c.g[S0][0];
        // Play one line: the call, then clear its request (through s0 as
        // the original does, not a saved copy).
        let say = |c: &mut Cpu, callee: Callee, ret: u32, flag: u32| -> Flow {
            callee.call(c, ret)?;
            let s0 = c.g[S0][0] as u32;
            Addr(s0.wrapping_add(flag)).sw(c, 0);
            Ok(())
        };
        c.g[V0][0] = at(DJ_20E5).lw(c);
        c.g[A1][0] = 0;
        let played = 'played: {
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A2][0] = 0;
                say(c, DJ_SAY_20E5, 0x002a_43dc, DJ_20E5)?;
                break 'played true;
            }
            c.cycles += 3;
            c.g[V0][0] = at(DJ_212B).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = me;
                say(c, DJ_SAY_212B, 0x002a_43f8, DJ_212B)?;
                break 'played true;
            }
            // Lines whose word the previous check loaded (delay slot).
            for (flag, callee, ret) in [
                (DJ_20BA, DJ_SAY_20BA, 0x002a_4410),
                (DJ_212C, DJ_SAY_212C, 0x002a_4428),
                (DJ_212D, DJ_SAY_212D, 0x002a_4440),
            ] {
                c.g[V0][0] = at(flag).lw(c);
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.cycles += 4;
                    c.g[A0][0] = me;
                    say(c, callee, ret, flag)?;
                    break 'played true;
                }
            }
            c.g[V0][0] = at(RIDER_20D3).lw(c);
            c.cycles += 2;
            c.g[A0][0] = me;
            if c.g[V0][0] != 0 {
                c.cycles += 5;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                say(c, RIDER_SAY_20D3, 0x002a_445c, RIDER_20D3)?;
                break 'played true;
            }
            c.cycles += 3;
            c.g[V0][0] = at(DJ_20CF).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 5;
                c.g[A1][0] = at(DJ_20CF_ARG).lw(c);
                c.g[A0][0] = me;
                say(c, DJ_SAY_20CF, 0x002a_447c, DJ_20CF)?;
                break 'played true;
            }
            c.g[V0][0] = at(CHATTER_A).lw(c);
            c.cycles += 2;
            c.g[A0][0] = me;
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A1][0] = 0;
                say(c, PLAY_DJ_CHATTER, 0x002a_4494, CHATTER_A)?;
                break 'played true;
            }
            c.cycles += 3;
            c.g[V0][0] = at(CHATTER_B).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A1][0] = 0;
                say(c, PLAY_DJ_CHATTER, 0x002a_44b0, CHATTER_B)?;
                break 'played true;
            }
            c.g[V0][0] = at(CHATTER_C).lw(c);
            c.cycles += 2;
            c.g[A0][0] = me;
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A1][0] = 1;
                say(c, PLAY_DJ_CHATTER, 0x002a_44c8, CHATTER_C)?;
                break 'played true;
            }
            c.cycles += 3;
            c.g[V0][0] = at(DJ_20CC).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 5;
                c.g[A1][0] = at(DJ_20CC_ARG).lw(c);
                c.g[A0][0] = me;
                say(c, DJ_SAY_20CC, 0x002a_44e8, DJ_20CC)?;
                break 'played true;
            }
            c.g[V0][0] = at(DJ_20E6).lw(c);
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[A0][0] = me;
                say(c, DJ_SAY_20E6, 0x002a_4500, DJ_20E6)?;
                break 'played true;
            }
            false
        };
        // A line played: the block at 0x2a4504.
        if played {
            c.cycles += 1;
        }
        // From here on `self` is s0 as the original reads it after the call.
        let this = c.g[S0][0] as u32;
        let at = |off: u32| Addr(this.wrapping_add(off));
        c.g[V0][0] = at(RIDER_20B9).lw(c);

        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.cycles += 8;
            call_ptr(c, GET_RACE_MAN, 0x002a_4518)?;
            c.g[V1][0] = at(RIDER_20B9_INDEX).lw(c);
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(RACE_RIDERS)).lw(c);
            if c.g[A1][0] != 0 {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                RIDER_SAY_20B9.call(c, 0x002a_4538)?;
            }
            let s0 = c.g[S0][0] as u32;
            Addr(s0.wrapping_add(RIDER_20B9)).sw(c, 0);
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_UpdateObjectLoops(SSXAudioSystem* self)`
/// (0x297fa0): for every active object loop, follow its emitter: a sound
/// object whose scene node is gone stops the loop; otherwise the position
/// is the node's translation (bone -1), the bone's translation (node with
/// bones) or the object's own position.
pub fn ssx_audio_system_update_object_loops(c: &mut Cpu) {
    use object_loop::*;
    use object_loop_callee::*;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[V0][0] = SLOT_SIZE;
        let this = c.g[S2][0] as u32;
        loop {
            // Slot i = s1 at s0 - 0x5a00.
            c.cycles += 6;
            c.g[A0][0] = 1;
            c.mult(0, V1, S1, V0);
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(this));
            let slot = (c.g[S0][0] as u32).wrapping_add(SLOTS);
            let at = |off: u32| Addr(slot.wrapping_add(off));
            c.g[V1][0] = at(ACTIVE).lw(c);
            if c.g[V1][0] == 1 {
                c.cycles += 4;
                c.g[V0][0] = at(OBJECT).lw(c);
                c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(OBJ_NODE)).lw(c);
                let node = c.g[A1][0] as u32;
                if node == 0 {
                    // The node is gone: stop the loop.
                    c.cycles += 9;
                    c.g[V0][0] = Addr(this.wrapping_add(CHANNELS)).lw(c);
                    c.g[A2][0] = 1;
                    c.g[A1][0] = at(HANDLE).lw(c);
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
                    abi::set_fpr(c, 12, FpuF32::ZERO);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(BANK_MONITOR));
                    STOP_HANDLE.call(c, 0x0029_8004)?;
                    at(ACTIVE).sw(c, 0);
                } else {
                    c.g[V1][0] = at(BONE).lw(c);
                    c.cycles += 3;
                    let bone = c.g[V1][0];
                    c.g[V0][0] = u64::MAX;
                    c.g[V1][0] = Addr(node.wrapping_add(NODE_VTABLE)).lw(c);
                    let vt = c.g[V1][0] as u32;
                    // Where the translation is read (`None`: the object's own
                    // position, through v1 = the object).
                    let matrix = if bone == u64::MAX {
                        c.cycles += 6;
                        virtual_call(c, node, vt, VT_MATRIX, 0x0029_8028)?;
                        true
                    } else {
                        c.cycles += 6;
                        virtual_call(c, node, vt, VT_HAS_BONES, 0x0029_8040)?;
                        if c.g[V0][0] == 0 {
                            c.g[V1][0] = at(OBJECT).lw(c);
                            false
                        } else {
                            c.cycles += 9;
                            c.g[V0][0] = at(OBJECT).lw(c);
                            c.g[A1][0] = at(BONE).lw(c);
                            c.g[A2][0] = Addr((c.g[V0][0] as u32).wrapping_add(OBJ_NODE)).lw(c);
                            let node = c.g[A2][0] as u32;
                            c.g[V1][0] = Addr(node.wrapping_add(NODE_VTABLE)).lw(c);
                            let vt = c.g[V1][0] as u32;
                            virtual_call(c, node, vt, VT_BONE_MATRIX, 0x0029_8068)?;
                            true
                        }
                    };
                    let sp0 = sp(c);
                    if matrix {
                        c.lq(V1, (c.g[V0][0] as u32).wrapping_add(TRANSLATION));
                        c.cycles += 3;
                        c.sq(V1, at(POSITION).0);
                        c.sq(V1, sp0);
                    } else {
                        c.cycles += 3;
                        c.lq(V0, (c.g[V1][0] as u32).wrapping_add(OBJ_POSITION));
                        c.sq(V0, at(POSITION).0);
                        c.sq(V0, sp0);
                    }
                }
                c.cycles += 1;
            }
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < COUNT) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V0][0] = SLOT_SIZE;
            if event_due(c) {
                deliver_event_here(c, 0x0029_7fc0);
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// A virtual call `node->vt[slot](node + delta)` (`{s16 delta; fn}` at
/// `vt + slot`), as the original sets it up: a0 = node + delta, v0 = fn.
pub(crate) fn virtual_call(c: &mut Cpu, node: u32, vt: u32, slot: u32, ret: u32) -> Flow {
    c.g[A0][0] = Addr(vt.wrapping_add(slot)).lh(c);
    c.g[V0][0] = Addr(vt.wrapping_add(slot + 4)).lw(c);
    let f = c.g[V0][0] as u32;
    c.g[A0][0] = sx(node.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, f, ret)
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_UpdateRaceLoopSfx (0x29d290)

/// `SSXAudioSystem_GetRaceMan` (cross-shard, through the table).
pub(crate) const SSX_AUDIO_GET_RACE_MAN: u32 = 0x0028_b1d8;

pub(crate) const SSX_AUDIO_START_LOOP_SFX: Callee = Callee {
    addr: 0x0029_d370,
    f: ee::s08::SSXAudioSystem_StartLoopSfx,
};

pub(crate) const SSX_AUDIO_STOP_LOOP_SFX: Callee = Callee {
    addr: 0x0029_d678,
    f: ee::s08::SSXAudioSystem_StopLoopSfx,
};

pub(crate) const BANK_MONITOR_STOP_BY_BANK: Callee = Callee {
    addr: 0x002a_d650,
    f: ee::s08::BankMonitor_StopByBank,
};

/// Race manager: its race state (+0x28) with the loop sound id (+0x430,
/// -1: none) and the race phase (+0x434; below 22 the loop plays).
pub(crate) const RACE_MAN_STATE: u32 = 0x28;

pub(crate) const RACE_LOOP_ID: u32 = 0x430;

pub(crate) const RACE_PHASE: u32 = 0x434;

pub(crate) const RACE_PHASE_LOOPING: i64 = 22;

/// Audio system: the loop id last seen (+0x5fc0, compared by its low byte)
/// and the sound engine pointer (+0x118 -> object with the bank monitor at
/// +0x1d8 and the event table at +0xacc: two events at +0x300/+0x360, each
/// "on" word followed by its bank id).
pub(crate) const AUDIO_LOOP_SEEN: u32 = 0x5fc0;

pub(crate) const AUDIO_ENGINE: u32 = 0x118;

pub(crate) const ENGINE_BANK_MONITOR: u32 = 0x1d8;

pub(crate) const ENGINE_EVENTS: u32 = 0xacc;

/// `gp`-relative fade times (`.lit4`): the loop stop and the two bank stops.
pub(crate) const GP_LOOP_FADE: u32 = 0x4594;

pub(crate) const GP_BANK_A_FADE: u32 = 0x4590;

pub(crate) const GP_BANK_B_FADE: u32 = 0x458c;

/// `BankMonitor_StopByBank(engine.bank_monitor, id, fade)` for the event at
/// `events + on`: id = the bank id after the "on" word if that word is 1,
/// else -1. `fade_gp` is the gp offset of the fade literal; `extra` the
/// cycles charged with the call.
pub(crate) fn stop_event_bank(
    c: &mut Cpu,
    on: u32,
    fade_gp: u32,
    extra: u64,
    ret: u32,
) -> crate::call::Flow {
    let engine = Addr(r32(c, S0)).off(AUDIO_ENGINE).lw(c);
    c.g[V0][0] = engine;
    c.g[V1][0] = Addr(engine as u32).lw(c);
    let object = r32(c, V1);
    c.g[A1][0] = Addr(object).off(ENGINE_EVENTS).lw(c);
    let events = r32(c, A1);
    c.g[V0][0] = Addr(events).off(on).lw(c) ^ 1;
    c.g[A0][0] = sx(object.wrapping_add(ENGINE_BANK_MONITOR));
    if c.g[V0][0] != 0 {
        c.cycles += 1;
        c.g[A1][0] = u64::MAX;
    } else {
        c.cycles += 2;
        c.g[A1][0] = Addr(events).off(on + 4).lw(c);
    }
    c.cycles += extra;
    let gp = r32(c, GP);
    c.lwc1(12, gp.wrapping_sub(fade_gp));
    BANK_MONITOR_STOP_BY_BANK.call(c, ret)
}

/// `void SSXAudioSystem_UpdateRaceLoopSfx(SSXAudioSystem* self)` (0x29d290):
/// follows the race's loop sound id (`raceMan->state+0x430`). When it is
/// set (not -1) and its low byte differs from the one last seen
/// (`self+0x5fc0`, then updated): in the looping phases (`+0x434 < 22`)
/// starts the loop sfx; afterwards stops it (`StopLoopSfx` with a fade) and
/// stops the banks of the two race events (`BankMonitor_StopByBank`).
pub fn ssx_audio_system_update_race_loop_sfx(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, SSX_AUDIO_GET_RACE_MAN, 0x0029_d2a4)?;
        let this = Addr(r32(c, S0));
        c.g[A1][0] = Addr(r32(c, V0)).off(RACE_MAN_STATE).lw(c);
        let race = Addr(r32(c, A1));
        c.g[A0][0] = race.off(RACE_LOOP_ID).lw(c);
        c.g[V0][0] = c.g[A0][0] & 0xff;
        'out: {
            if c.g[A0][0] == u64::MAX {
                c.cycles += 1;
                break 'out;
            }
            c.cycles += 3;
            c.g[V1][0] = this.off(AUDIO_LOOP_SEEN).lbu(c);
            if c.g[V0][0] == c.g[V1][0] {
                break 'out;
            }
            c.cycles += 5;
            this.off(AUDIO_LOOP_SEEN).sw(c, c.g[A0][0]);
            c.g[V0][0] = race.off(RACE_PHASE).lw(c);
            c.g[V0][0] = ((c.g[V0][0] as i64) < RACE_PHASE_LOOPING) as u64;
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                SSX_AUDIO_START_LOOP_SFX.call(c, 0x0029_d2e0)?;
                break 'out;
            }
            c.cycles += 10;
            let gp = r32(c, GP);
            c.lwc1(12, gp.wrapping_sub(GP_LOOP_FADE));
            c.g[A0][0] = c.g[S0][0];
            SSX_AUDIO_STOP_LOOP_SFX.call(c, 0x0029_d2f4)?;
            stop_event_bank(c, 0x300, GP_BANK_A_FADE, 10, 0x0029_d328)?;
            stop_event_bank(c, 0x360, GP_BANK_B_FADE, 3, 0x0029_d35c)?;
            c.cycles += 1;
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod airborne_riders {
    use super::*;

    pub const GET_RACE_MAN: Callee = Callee {
        addr: 0x0028_b1d8,
        f: ee::s07::f_0028b1d8,
    };

    pub const UPDATE_RIDER_AIR_STATE: Callee = Callee {
        addr: 0x0029_4678,
        f: ee::s07::f_00294678,
    };

    /// Audio system: one "airborne" flag word per rider slot (6).
    pub const AIR_FLAGS: u32 = 0x59e8;

    /// Race manager: the rider count and the rider pointers.
    pub const RACE_RIDER_COUNT: u32 = 0x78;

    pub const RACE_RIDERS: u32 = 0x28;

    /// The loop head (event checks).
    pub const LOOP_HEAD: u32 = 0x0029_47d8;
}

pub(crate) mod loop_sfx {
    use super::ee;
    use super::Callee;

    /// SSXAudioSystem: a looping effect is playing, and its SND handle.
    pub const LOOP_ACTIVE: u32 = 0x5fc4;

    pub const LOOP_HANDLE: u32 = 0x5fc8;

    /// `SND_Lock`, `SND_IsHandleDone(handle)`, `SND_Unlock`.
    pub const SND_LOCK: u32 = 0x003b_58a0;

    pub const SND_IS_HANDLE_DONE: u32 = 0x003b_8ab8;

    pub const SND_UNLOCK: u32 = 0x003b_58d8;

    pub const STOP_LOOP_SFX: Callee = Callee {
        addr: 0x0029_d678,
        f: ee::s08::f_0029d678,
    };

    pub const START_LOOP_SFX: Callee = Callee {
        addr: 0x0029_d370,
        f: ee::s08::f_0029d370,
    };
}

// ------------------------------------------- SSXAudioSystem_UpdateUberSpeech

/// `SSXAudioSystem` fields: the rider id the uber-speech state belongs to
/// (+0x5820) and the rider's last seen trick/boost state (+0x581c).
pub(crate) const UBER_RIDER_ID: u32 = 0x5820;

pub(crate) const UBER_LAST_STATE: u32 = 0x581c;

/// Rider fields: player id (+0x870) and the state watched for "uber" (10).
pub(crate) const UBER_RIDER_PLAYER_ID: u32 = 0x870;

pub(crate) const UBER_RIDER_STATE: u32 = 0x2f4;

/// Game module: game mode (+0x214); uber speech only in mode 4.
pub(crate) const UBER_MODULE_MODE: u32 = 0x214;

/// `SSXAudioSystem_ArcadeSay2133` (0x2a3de0).
pub(crate) const UBER_ARCADE_SAY: Callee = Callee {
    addr: 0x002a_3de0,
    f: ee::s08::f_002a3de0,
};

/// `void SSXAudioSystem_UpdateUberSpeech(SSXAudioSystem* self)` (0x29b738),
/// every frame: outside replays, find the followed rider
/// (`FindRider(self, -1)`); when it is the rider tracked at +0x5820 and its
/// state (+0x2f4) just became 10 (uber) in game mode 4, say line 0x2133
/// (`ArcadeSay2133(self, rider, 2)`); a different rider is tracked from now
/// on. The rider's state is remembered at +0x581c. Frame 0x30 (s0, s1).
pub fn ssx_audio_update_uber_speech(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        // SSXAudioSystem_IsReplayPlaying(self)
        call_ptr(c, 0x0028_8ae0, 0x0029_b750)?;
        if c.g[V0][0] == 0 {
            c.cycles += 8;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = u64::MAX;
            // SSXAudioSystem_FindRider(self, -1)
            call_ptr(c, 0x0028_5d98, 0x0029_b764)?;
            c.g[S1][0] = c.g[V0][0];
            let this = r32(c, S0);
            let rider = r32(c, S1);
            let tracked = Addr(this.wrapping_add(UBER_RIDER_ID)).lw(c);
            c.g[V0][0] = tracked;
            let id = Addr(rider.wrapping_add(UBER_RIDER_PLAYER_ID)).lw(c);
            c.g[V1][0] = id;
            if id != tracked {
                // Another rider: track it.
                Addr(this.wrapping_add(UBER_RIDER_ID)).sw(c, id);
                c.cycles += 1;
            } else {
                c.cycles += 4;
                let state = Addr(rider.wrapping_add(UBER_RIDER_STATE)).lw(c);
                c.g[V1][0] = state;
                c.g[V0][0] = 10;
                if state == 10 {
                    c.cycles += 3;
                    let last = Addr(this.wrapping_add(UBER_LAST_STATE)).lw(c);
                    c.g[V0][0] = last;
                    if last != state {
                        c.cycles += 6;
                        // SSXAudioSystem_GetGameModule()
                        call_ptr(c, 0x0028_b1c8, 0x0029_b79c)?;
                        let mode = Addr(r32(c, V0).wrapping_add(UBER_MODULE_MODE)).lw(c);
                        c.g[V1][0] = mode;
                        c.g[V0][0] = 4;
                        if mode == 4 {
                            c.cycles += 6;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = c.g[S1][0];
                            c.g[A2][0] = 2;
                            UBER_ARCADE_SAY.call(c, 0x0029_b7bc)?;
                        }
                    }
                }
            }
            let state = Addr(r32(c, S1).wrapping_add(UBER_RIDER_STATE)).lw(c);
            c.g[V0][0] = state;
            c.cycles += 2;
            Addr(r32(c, S0).wrapping_add(UBER_LAST_STATE)).sw(c, state);
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_UpdateAirborneRiders(SSXAudioSystem* this)`
/// (0x2947b0): for each of the 6 rider slots whose airborne flag is set,
/// update that rider's air sound state if the race has such a rider;
/// otherwise clear the flag.
pub fn ssx_audio_system_update_airborne_riders(c: &mut Cpu) {
    use airborne_riders::*;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        // s0: the slot's flag, s1: the slot, s3: the slot * 4.
        c.g[S0][0] = sx(r32(c, S2).wrapping_add(AIR_FLAGS));
        frame.save(c, S3, 0x10);
        c.g[S1][0] = 0;
        frame.save_ra(c, 0);
        c.g[S3][0] = 0;
        loop {
            'next: {
                'clear: {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                    if c.g[V0][0] == 0 {
                        Addr(r32(c, S0)).sw(c, 0);
                        break 'clear;
                    }
                    c.cycles += 6;
                    GET_RACE_MAN.call(c, 0x0029_47ec)?;
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(RACE_RIDER_COUNT)).lw(c);
                    c.g[V1][0] = ((c.g[S1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if c.g[V1][0] == 0 {
                        Addr(r32(c, S0)).sw(c, 0);
                        break 'clear;
                    }
                    c.cycles += 7;
                    GET_RACE_MAN.call(c, 0x0029_4804)?;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(r32(c, S3)));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(RACE_RIDERS)).lw(c);
                    c.g[A1][0] = c.g[V1][0];
                    if c.g[V1][0] == 0 {
                        c.cycles += 1;
                        Addr(r32(c, S0)).sw(c, 0);
                        break 'clear;
                    }
                    // UpdateRiderAirState(this, rider).
                    c.cycles += 4;
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                    UPDATE_RIDER_AIR_STATE.call(c, 0x0029_4820)?;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
                    break 'next;
                }
                c.cycles += 2;
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 6) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S3][0] = sx(r32(c, S3).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, LOOP_HEAD);
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
    })
}

/// `void SSXAudioSystem_UpdateLoopSfx(SSXAudioSystem *self)` (0x29d610):
/// while a looping effect plays, check its SND handle (under the SND lock);
/// when the sample has ended, stop it (fade 0) and start the next one.
pub fn ssx_audio_system_update_loop_sfx(c: &mut Cpu) {
    use loop_sfx::*;
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S1) + LOOP_ACTIVE).lw(c);
        frame.restore(c, S0, 0x20);
        if c.g[V0][0] != 0 {
            c.cycles += 8;
            call_ptr(c, SND_LOCK, 0x0029_d638)?;
            c.g[A0][0] = Addr(r32(c, S1) + LOOP_HANDLE).lw(c);
            call_ptr(c, SND_IS_HANDLE_DONE, 0x0029_d640)?;
            c.g[S0][0] = (c.g[V0][0] > 0) as u64;
            call_ptr(c, SND_UNLOCK, 0x0029_d648)?;
            let done = c.g[S0][0] != 0;
            frame.restore(c, S0, 0x20);
            if done {
                c.cycles += 6;
                c.mtc1(0, 12);
                c.g[A0][0] = c.g[S1][0];
                STOP_LOOP_SFX.call(c, 0x0029_d65c)?;
                c.g[A0][0] = c.g[S1][0];
                START_LOOP_SFX.call(c, 0x0029_d664)?;
                frame.restore(c, S0, 0x20);
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// CrowdAudio_Update (0x2a6d18)

/// CrowdAudio fields: number of crowd instances, their array (0x190 bytes each).
pub(crate) const CROWD_AUDIO_COUNT: u32 = 0x14;

pub(crate) const CROWD_AUDIO_INSTANCES: u32 = 0x18;

pub(crate) const CROWD_AUDIO_INSTANCE_SIZE: u32 = 0x190;

pub(crate) const CROWD_AUDIO_UPDATE_INSTANCE: Callee = Callee {
    addr: 0x002a_4e88,
    f: ee::s08::f_002a4e88,
};

/// `void CrowdAudio_Update(CrowdAudio *self)` (0x2a6d18): updates every crowd
/// sound instance (`CrowdAudio_UpdateInstance`), re-reading the count after
/// each one.
pub fn crowd_audio_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(CROWD_AUDIO_COUNT)).lw(c);
        c.g[S0][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 1;
            // s0: index, s2: byte offset of the instance.
            c.g[S2][0] = 0;
            loop {
                c.cycles += 8;
                let array = Addr(r32(c, S1).wrapping_add(CROWD_AUDIO_INSTANCES)).lw(c);
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                c.g[A0][0] = sx((array as u32).wrapping_add(r32(c, S2)));
                CROWD_AUDIO_UPDATE_INSTANCE.call(c, 0x002a_6d50)?;
                let count = Addr(r32(c, S1).wrapping_add(CROWD_AUDIO_COUNT)).lw(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (count as i64)) as u64;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(CROWD_AUDIO_INSTANCE_SIZE));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002a_6d40);
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
// SSXAudioSystem_CalcSurfaceLoopRemoteGroup0 (0x293b50)

pub(crate) const SSX_AUDIO_CALC_SURFACE_LOOP_GROUP0: Callee = Callee {
    addr: 0x0029_3538,
    f: ee::s07::f_00293538,
};

/// `void SSXAudioSystem_CalcSurfaceLoopRemoteGroup0(...)` (0x293b50): the
/// remote (other riders') variant of the surface loop group 0 calculation;
/// forwards its arguments to `SSXAudioSystem_CalcSurfaceLoopGroup0`.
pub fn ssx_audio_system_calc_surface_loop_remote_group0(c: &mut Cpu) {
    call::tail_wrapper(
        c,
        None,
        call::Target::Direct(SSX_AUDIO_CALC_SURFACE_LOOP_GROUP0),
        0x0029_3b60,
    );
}

// -------------------------------------------- SSXAudioSystem_PlayWallHitSound

/// `SSXAudioSystem_FindViewInRange` (0x288b40, cross-shard).
pub(crate) const WALLHIT_FIND_VIEW_IN_RANGE: u32 = 0x0028_8b40;

/// `Audio_EvalCurveTable` (0x290c10, cross-shard).
pub(crate) const WALLHIT_EVAL_CURVE_TABLE: u32 = 0x0029_0c10;

/// `SSXAudioSystem_IsTrackedSoundPlaying` (0x295028).
pub(crate) const WALLHIT_IS_TRACKED_SOUND_PLAYING: Callee = Callee {
    addr: 0x0029_5028,
    f: ee::s08::f_00295028,
};

/// `WorldTriggerManager_PlayTriggerSound` (0x2b5f60).
pub(crate) const WALLHIT_PLAY_TRIGGER_SOUND: Callee = Callee {
    addr: 0x002b_5f60,
    f: ee::s08::f_002b5f60,
};

/// `SSXAudioSystem_TrackSound` (0x295628).
pub(crate) const WALLHIT_TRACK_SOUND: Callee = Callee {
    addr: 0x0029_5628,
    f: ee::s08::f_00295628,
};

/// The event's sound table (`{count, _, ids[]}` at +0x10 of it).
pub(crate) const WALLHIT_EVENT_SOUNDS: u32 = 0x8c;

/// The rider's physics object (v-table at +0: position at +0x28,
/// velocity at +0x10).
pub(crate) const WALLHIT_RIDER_BODY: u32 = 0x6c0;

/// `WorldTriggerManager*` at gp+0x21e4.
pub(crate) const WALLHIT_TRIGGER_MANAGER_GP: u32 = 0x21e4;

/// The audio system's player (+0x118, `**` -> clock at +0x260).
pub(crate) const WALLHIT_PLAYER: u32 = 0x118;

/// `void SSXAudioSystem_PlayWallHitSound(SSXAudioSystem* self, Rider* rider,
/// SoundEvent* evt, int idx)` (0x296088): when a view is within 100 m of
/// the rider and sound `idx` of the event is not already tracked, plays it
/// (volume from the rider's speed through curve table 0, clamped 0..127)
/// and tracks it for the next second.
pub fn ssx_audio_system_play_wall_hit_sound(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x60);
        frame.save(c, 16, 0x50);
        frame.save(c, 17, 0x40);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, 18, 0x30);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, 19, 0x20);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, 20, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, S2).wrapping_add(WALLHIT_EVENT_SOUNDS)).lw(c);
        c.g[S3][0] = c.g[A0][0];
        // Ends early (`true`) at the shared exit with its extra cycle, or
        // (`false`) straight at the epilogue.
        let extra_cycle = 'play: {
            if c.g[V0][0] == 0 {
                break 'play true;
            }
            // Any view near the rider's position?
            c.cycles += 13;
            c.g[V1][0] = Addr(r32(c, S0).wrapping_add(WALLHIT_RIDER_BODY)).lw(c);
            c.g[S4][0] = sx(r32(c, S0).wrapping_add(WALLHIT_RIDER_BODY));
            let vt = r32(c, V1);
            let delta = Addr(vt.wrapping_add(0x28)).lh(c);
            c.g[V0][0] = Addr(vt.wrapping_add(0x2c)).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = sx(r32(c, S4).wrapping_add(delta as u32));
            call_ptr(c, target, 0x0029_60d4)?;
            c.g[AT][0] = 0x42c8_0000;
            c.f[12] = 0x42c8_0000;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, WALLHIT_FIND_VIEW_IN_RANGE, 0x0029_60e8)?;
            let none = (c.g[V0][0] as i64) < 0;
            c.g[A0][0] = c.g[S3][0];
            if none {
                break 'play true;
            }
            // Already playing this one?
            c.cycles += 5;
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S1][0];
            WALLHIT_IS_TRACKED_SOUND_PLAYING.call(c, 0x0029_60fc)?;
            if c.g[V0][0] != 0 {
                break 'play false;
            }
            // Volume from the length of the rider's velocity.
            c.cycles += 42;
            c.g[V1][0] = Addr(r32(c, S0).wrapping_add(WALLHIT_RIDER_BODY)).lw(c);
            let vt = r32(c, V1);
            let delta = Addr(vt.wrapping_add(0x10)).lh(c);
            c.g[V0][0] = Addr(vt.wrapping_add(0x14)).lw(c);
            let target = r32(c, V0);
            c.g[A0][0] = sx(r32(c, S4).wrapping_add(delta as u32));
            call_ptr(c, target, 0x0029_6118)?;
            c.lqc2(3, r32(c, V0));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().sqrt(4, 0);
            c.vu0().waitq();
            c.cfc2(3, 22);
            c.mtc1(3, 12);
            c.g[A0][0] = 0;
            call_ptr(c, WALLHIT_EVAL_CURVE_TABLE, 0x0029_6148)?;
            // Clamp the volume to 0..127 and pick the sound: ids[idx], or
            // ids[0] when idx is out of range or its id is 0.
            let sounds = Addr(r32(c, S2).wrapping_add(WALLHIT_EVENT_SOUNDS)).lw(c);
            c.g[V0][0] = sounds;
            c.cvt_w_s(1, 0);
            c.mfc1(T0, 1);
            c.g[A2][0] = 0x7f;
            c.g[V1][0] = u64::MAX;
            c.g[T1][0] = Addr((sounds as u32).wrapping_add(0x10)).lw(c);
            c.g[A3][0] = c.g[S0][0];
            c.g[A1][0] = ((c.g[T0][0] as i64) < 128) as u64;
            c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(WALLHIT_TRIGGER_MANAGER_GP)).lw(c);
            c.g[V0][0] = Addr(r32(c, T1)).lw(c);
            if c.g[A1][0] == 0 {
                c.g[T0][0] = c.g[A2][0];
            }
            c.g[T1][0] = sx(r32(c, T1).wrapping_add(8));
            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[T0][0] as i64)) as u64;
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V1][0] == 0 {
                c.g[T0][0] = 0;
            }
            if c.g[V0][0] == 0 {
                c.g[S1][0] = 0;
            }
            c.g[A1][0] = c.g[S2][0];
            c.g[V0][0] = sx((r32(c, S1) << 2).wrapping_add(r32(c, T1)));
            c.g[V1][0] = Addr(r32(c, V0)).lw(c);
            if c.g[V1][0] != 0 {
                c.g[T1][0] = c.g[V0][0];
            }
            c.g[A2][0] = Addr(r32(c, T1)).lw(c);
            WALLHIT_PLAY_TRIGGER_SOUND.call(c, 0x0029_61a4)?;
            let failed = (c.g[V0][0] as i64) < 0;
            c.g[A2][0] = c.g[V0][0];
            if failed {
                break 'play true;
            }
            // Track it until the player's clock + 1000.
            c.cycles += 9;
            c.g[V1][0] = Addr(r32(c, S3).wrapping_add(WALLHIT_PLAYER)).lw(c);
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[V0][0] = Addr(r32(c, V1)).lw(c);
            c.g[T1][0] = c.g[S1][0];
            c.g[T0][0] = 0;
            c.g[A3][0] = Addr(r32(c, V0).wrapping_add(0x260)).lw(c);
            c.g[A3][0] = sx(r32(c, A3).wrapping_add(1000));
            WALLHIT_TRACK_SOUND.call(c, 0x0029_61d0)?;
            true
        };
        if extra_cycle {
            c.cycles += 1;
        }
        frame.restore(c, 16, 0x50);
        c.cycles += 7;
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod rider_air_state {

    /// Rider: the air-state object (+0x6c0, its vtable first; slot 0x38 =
    /// the rider's audio slot index).
    pub const RIDER_AIR_OBJ: u32 = 0x6c0;

    /// Rider: the animation/physics state (+0x788).
    pub const RIDER_ANIM: u32 = 0x788;

    /// Rider: player id (+0x870), human flags (+0x874, +0x87c).
    pub const RIDER_PLAYER_ID: u32 = 0x870;

    pub const RIDER_HUMAN: u32 = 0x874;

    pub const RIDER_HUMAN2: u32 = 0x87c;

    /// Anim state: air time (+0x98, float), air mode (+0xac: 1 or 3 = airborne).
    pub const ANIM_AIR_TIME: u32 = 0x98;

    pub const ANIM_AIR_MODE: u32 = 0xac;

    /// The air-state object's vtable slot giving the rider's audio slot.
    pub const VT_SLOT: u32 = 0x38;
}

/// `void SSXAudioSystem_UpdateRiderAirState(SSXAudioSystem* self, Rider*
/// rider)` (0x294678): when the rider is no longer airborne (air mode not 1
/// or 3), clear its airborne flag (`SetRiderAirFlag(self, slot, 1)`, the
/// slot from the rider's air-state virtual). Otherwise keep the flag set
/// (`SetRiderAirFlag(self, slot, 0)`), let the rider say line 0x2083, and
/// when the followed rider (`FindRider(self, idx)`, idx 0 in replays, the
/// player id of a human rider, else -1) is this rider and its air time is
/// over 2 s, start the crowd's anticipation for it
/// (`CrowdAudio_StartAnticipate(self, idx)`). Frame 0x50 (s0-s2, f20).
pub fn ssx_audio_system_update_rider_air_state(c: &mut Cpu) {
    use rider_air_state::*;
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x40);
        let rider = r32(c, S0);
        let anim = Addr(rider.wrapping_add(RIDER_ANIM)).lw(c);
        let mode = Addr((anim as u32).wrapping_add(ANIM_AIR_MODE)).lw(c);
        c.g[V1][0] = mode;
        let airborne = if mode == 1 {
            c.cycles += 1;
            true
        } else {
            c.cycles += 3;
            c.g[V0][0] = 3;
            if mode == 3 {
                c.cycles += 1;
            }
            mode == 3
        };
        c.g[A0][0] = airborne as u64;

        c.cycles += 2;
        let vt = Addr(rider.wrapping_add(RIDER_AIR_OBJ)).lw(c);
        c.g[V1][0] = vt;
        if !airborne {
            // Landed: clear the rider's airborne flag.
            c.cycles += 11;
            c.g[A1][0] = sx(rider.wrapping_add(RIDER_AIR_OBJ));
            let delta = Addr((vt as u32).wrapping_add(VT_SLOT)).lh(c);
            let method = Addr((vt as u32).wrapping_add(VT_SLOT + 4)).lw(c);
            c.g[V0][0] = method;
            c.g[A0][0] = abi::addu(c.g[A1][0], delta);
            call_ptr(c, method as u32, 0x0029_46d8)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[V0][0];
            c.g[A2][0] = 1;
            Callee {
                addr: 0x0029_4880,
                f: ee::s07::f_00294880,
            }
            .call(c, 0x0029_46e8)?;
            frame.restore(c, S0, 0x30);
        } else {
            c.cycles += 19;
            c.g[A1][0] = sx(rider.wrapping_add(RIDER_AIR_OBJ));
            c.g[S2][0] = u64::MAX;
            let delta = Addr((vt as u32).wrapping_add(VT_SLOT)).lh(c);
            let method = Addr((vt as u32).wrapping_add(VT_SLOT + 4)).lw(c);
            c.g[V0][0] = method;
            c.g[A0][0] = abi::addu(c.g[A1][0], delta);
            call_ptr(c, method as u32, 0x0029_4708)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = 0;
            Callee {
                addr: 0x0029_4880,
                f: ee::s07::f_00294880,
            }
            .call(c, 0x0029_4718)?;
            // f20 = the rider's air time.
            let anim = Addr(r32(c, S0).wrapping_add(RIDER_ANIM)).lw(c);
            c.g[V0][0] = anim;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            let air_time = Addr((anim as u32).wrapping_add(ANIM_AIR_TIME)).read_fpu(c);
            abi::set_fpr(c, 20, air_time);
            // SSXAudioSystem_RiderSay2083(self, rider)
            call_ptr(c, 0x0029_fcc8, 0x0029_472c)?;
            c.g[A0][0] = c.g[S1][0];
            Callee {
                addr: 0x0028_8ae0,
                f: ee::s07::f_00288ae0,
            }
            .call(c, 0x0029_4734)?;
            if c.g[V0][0] != 0 {
                // Replay: the followed rider is rider 0.
                c.cycles += 2;
                c.g[S2][0] = 0;
            } else {
                let rider = r32(c, S0);
                let human = Addr(rider.wrapping_add(RIDER_HUMAN)).lw(c);
                c.g[V0][0] = human;
                c.cycles += 2;
                c.g[V1][0] = 0;
                if human != 0 {
                    c.cycles += 2;
                    let human2 = Addr(rider.wrapping_add(RIDER_HUMAN2)).lw(c);
                    c.g[V0][0] = human2;
                    c.g[V1][0] = (human2 != 0) as u64;
                }
                c.cycles += 2;
                if c.g[V1][0] != 0 {
                    c.g[S2][0] = Addr(rider.wrapping_add(RIDER_PLAYER_ID)).lw(c);
                }
            }
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S2][0];
            Callee {
                addr: 0x0028_5d98,
                f: ee::s07::f_00285d98,
            }
            .call(c, 0x0029_4768)?;
            let followed = c.g[V0][0] == c.g[S0][0];
            frame.restore(c, S0, 0x30);
            if followed {
                c.cycles += 6;
                c.g[AT][0] = 0x4000_0000;
                let two = FpuF32(0x4000_0000);
                abi::set_fpr(c, 0, two);
                let air_time = FpuF32(c.f[20]);
                let long_air = abi::c_lt_s(c, two, air_time);
                c.g[A0][0] = c.g[S1][0];
                if long_air {
                    c.cycles += 3;
                    c.g[A1][0] = c.g[S2][0];
                    // CrowdAudio_StartAnticipate(self, idx)
                    call_ptr(c, 0x002a_7408, 0x0029_4790)?;
                    frame.restore(c, S0, 0x30);
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

// -- 0x002a77c8 CrowdAudio_GetInstanceReaction -------------------------------

/// Crowd audio: instance count (+0x14) and instance array (+0x18) of
/// 0x190-byte instances whose reaction is at +0x18c.
pub(crate) const CROWD_INSTANCE_COUNT: u32 = 0x14;

pub(crate) const CROWD_INSTANCES: u32 = 0x18;

pub(crate) const CROWD_INSTANCE_SIZE: u32 = 0x190;

pub(crate) const CROWD_INSTANCE_REACTION: u32 = 0x18c;

/// `int CrowdAudio_GetInstanceReaction(CrowdAudio* self, int i)` (0x2a77c8):
/// the reaction of crowd instance `i` (signed `i < count`), else 0. Leaf.
pub fn crowd_audio_get_instance_reaction(c: &mut Cpu) {
    c.cycles += 4;
    let crowd = abi::arg(c, 0);
    let count = Addr(crowd.wrapping_add(CROWD_INSTANCE_COUNT)).lw(c);
    let in_range = (c.g[A1][0] as i64) < (count as i64);
    c.g[V0][0] = in_range as u64;
    c.g[V1][0] = CROWD_INSTANCE_SIZE as u64;
    if in_range {
        c.cycles += 5;
        let base = Addr(crowd.wrapping_add(CROWD_INSTANCES)).lw(c);
        let i = abi::arg(c, 1);
        let off = abi::mult(c, i, CROWD_INSTANCE_SIZE);
        let inst = (base as u32).wrapping_add(off);
        c.g[V1][0] = sx(inst);
        c.g[V0][0] = Addr(inst.wrapping_add(CROWD_INSTANCE_REACTION)).lw(c);
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0;
    }
    ret(c);
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_PlayBoostEmptySound

/// `SSXAudioSystem.bankMonitor` (+0): the monitor whose push queue the
/// sound goes through.
pub(crate) const AUDIO_SYS_MONITOR: u32 = 0x0;

/// `SSXAudioSystem.pushBank` / `.pushSample` (+4 / +8): per-queue-entry
/// word arrays.
pub(crate) const AUDIO_SYS_PUSH_BANK: u32 = 0x4;

pub(crate) const AUDIO_SYS_PUSH_SAMPLE: u32 = 0x8;

/// `SSXAudioSystem.sndRoot` (+0x118): `*root` is the sound bank monitor.
pub(crate) const AUDIO_SYS_SND_ROOT: u32 = 0x118;

/// `Rider.isHuman` / `Rider.isLocal`.
pub(crate) const RIDER_AUDIO_HUMAN: u32 = 0x874;

pub(crate) const RIDER_AUDIO_LOCAL: u32 = 0x87c;

/// The bank monitor's push queue (`monitor + 0x1d8`) and its entry count
/// (`queue + 0x1c`, i.e. `monitor + 0x1f4`).
pub(crate) const MONITOR_PUSH_QUEUE: u32 = 0x1d8;

pub(crate) const MONITOR_PUSH_COUNT: u32 = 0x1f4;

/// One "store `value` into word table `queue + table` at the current queue
/// entry" step of the inlined push: `ri = count << shift; rt = table;
/// ri += rt; *(ri + extra) = value` (the original's register pair).
pub(crate) fn boost_empty_store_entry(
    c: &mut Cpu,
    (ri, rt): (usize, usize),
    table: u32,
    shift: u32,
    extra: u32,
    value: u64,
) {
    let monitor = c.g[S0][0] as u32;
    let queue = c.g[S1][0] as u32;
    c.g[ri][0] = Addr(monitor).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[rt][0] = Addr(queue).off(table).lw(c);
    c.g[ri][0] = sx((c.g[ri][0] as u32) << shift);
    c.g[ri][0] = sx((c.g[ri][0] as u32).wrapping_add(c.g[rt][0] as u32));
    Addr(c.g[ri][0] as u32).off(extra).sw(c, value);
}

/// `void SSXAudioSystem_PlayBoostEmptySound(SSXAudioSystem* sys, Rider*
/// rider)` (0x299368): outside replays, for a human local rider, cancel the
/// pending boost-empty timers and push sound 0x6c of bank 9 (volume 100,
/// pan 90, 50, 127) on the monitor's push queue, then play it
/// (`Audio_PlayPushedSound`).
pub fn ssx_audio_system_play_boost_empty_sound(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A0][0]; // sys
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0]; // rider
                                 // SSXAudioSystem_IsReplayPlaying
        call_ptr(c, 0x0028_8ae0, 0x0029_938c)?;
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            let rider = c.g[S0][0] as u32;
            c.g[V0][0] = Addr(rider).off(RIDER_AUDIO_HUMAN).lw(c);
            c.g[A1][0] = 0;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[V0][0] = Addr(rider).off(RIDER_AUDIO_LOCAL).lw(c);
                c.g[A1][0] = (0u64 < c.g[V0][0]) as u64;
            }
            c.cycles += 2;
            let play = c.g[A1][0] != 0;
            c.g[V0][0] = 0x4a_0000;
            if play {
                c.cycles += 155;
                boost_empty_push_and_play(c)?;
            }
            c.cycles += 1;
        }
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

/// The inlined push of `PlayBoostEmptySound` (v0 = 0x4a0000 on entry).
pub(crate) fn boost_empty_push_and_play(c: &mut Cpu) -> crate::call::Flow {
    let sys = c.g[S3][0] as u32;
    // BankMonitor_KillTimersByCallback(*sys->sndRoot + 0x1d8, callback key
    // at 0x4a36a8).
    c.g[V1][0] = Addr(sys).off(AUDIO_SYS_SND_ROOT).lw(c);
    c.ldl(5, (c.g[V0][0] as u32).wrapping_add(0x36af));
    c.ldr(5, (c.g[V0][0] as u32).wrapping_add(0x36a8));
    c.g[S2][0] = 0x6c; // the sample
    c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(MONITOR_PUSH_QUEUE));
    Callee {
        addr: 0x002a_dde0,
        f: ee::s08::BankMonitor_KillTimersByCallback,
    }
    .call(c, 0x0029_93cc)?;
    audio_push_voice_and_play(c, [0x0029_94a0, 0x0029_95fc, 0x0029_961c])
}

/// The push-and-play sequence the compiler inlined into the
/// `SSXAudioSystem` sound triggers: a new entry on the bank monitor's push
/// queue (`sys->monitor + 0x1d8`) for sample `s2` of bank 9 (volume 100,
/// pan 90, 50, 127), its gain pointer, then `Audio_PlayPushedSound(sys)`.
/// `s3` = sys; `rets` = the return addresses of the memset, gain lookup and
/// play calls. Leaves v0 = the play handle.
pub(crate) fn audio_push_voice_and_play(c: &mut Cpu, rets: [u32; 3]) -> crate::call::Flow {
    // New queue entry: count += 1, copy the 24-byte template into the
    // entry's record.
    c.g[S0][0] = Addr(c.g[S3][0] as u32).off(AUDIO_SYS_MONITOR).lw(c);
    let monitor = c.g[S0][0] as u32;
    c.g[T1][0] = 0x18;
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x8;
    c.g[V0][0] = Addr(monitor).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[S1][0] = sx(monitor.wrapping_add(MONITOR_PUSH_QUEUE));
    let queue = c.g[S1][0] as u32;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    Addr(monitor).off(MONITOR_PUSH_COUNT).sw(c, c.g[V0][0]);
    c.g[V1][0] = Addr(monitor).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[V0][0] = Addr(queue).off(0x20).lw(c);
    c.mult(0, 3, 3, 9);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.ldl(2, monitor.wrapping_add(0x1e3));
    c.ldr(2, monitor.wrapping_add(0x1dc));
    c.ldl(4, monitor.wrapping_add(0x1eb));
    c.ldr(4, monitor.wrapping_add(0x1e4));
    c.ldl(7, monitor.wrapping_add(0x1f3));
    c.ldr(7, monitor.wrapping_add(0x1ec));
    let record = c.g[V1][0] as u32;
    c.sdl(2, record.wrapping_add(0x7));
    c.sdr(2, record);
    c.sdl(4, record.wrapping_add(0xf));
    c.sdr(4, record.wrapping_add(0x8));
    c.sdl(7, record.wrapping_add(0x17));
    c.sdr(7, record.wrapping_add(0x10));
    // monitor->pushHandle[count] = 0 (inline array at +0x1fc).
    c.g[V0][0] = Addr(monitor).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx(monitor.wrapping_add(c.g[V0][0] as u32));
    Addr(c.g[V0][0] as u32).off(0x1fc).sw(c, 0);
    boost_empty_store_entry(c, (V1, V0), 0x34, 2, 0, 0);
    // table 0x38 [count] = the first byte of the entry's record
    // (sign-extended; t1 = 0x18 is the record size).
    c.g[A3][0] = Addr(monitor).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[A0][0] = Addr(monitor).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[T0][0] = Addr(queue).off(0x20).lw(c);
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
    c.mult(0, 4, 4, 9);
    c.g[V0][0] = Addr(queue).off(0x38).lw(c);
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
    c.g[V1][0] = Addr(c.g[A0][0] as u32).lb(c);
    Addr(c.g[A3][0] as u32).sw(c, c.g[V1][0]);
    boost_empty_store_entry(c, (V0, V1), 0x3c, 2, 0, 0);
    // memset(&queue->slots[count] (8 bytes at +0x40), 0, 8)
    c.g[A0][0] = Addr(monitor).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
    c.g[A0][0] = sx(queue.wrapping_add(c.g[A0][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    call_ptr(c, 0x0041_6210, rets[0])?;

    // The entry's parameters.
    c.g[A3][0] = 0x64; // volume
    c.g[T0][0] = 0x5a;
    c.g[T1][0] = 0x32;
    boost_empty_store_entry(c, (V1, V0), 0x60, 2, 0, 0);
    c.g[T2][0] = 0x7f;
    c.g[T3][0] = 0x1;
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = 0x9; // bank / channel kind
    c.g[A2][0] = 0;
    boost_empty_store_entry(c, (V1, V0), 0x64, 2, 0, 0);
    boost_empty_store_entry(c, (V0, V1), 0x68, 2, 0, 0);
    boost_empty_store_entry(c, (V1, V0), 0x6c, 2, 0, 0);
    boost_empty_store_entry(c, (V0, V1), 0x70, 4, 0x4, 0);
    let (a3, t0, t1, t2, t3) = (c.g[A3][0], c.g[T0][0], c.g[T1][0], c.g[T2][0], c.g[T3][0]);
    boost_empty_store_entry(c, (V1, V0), 0x70, 4, 0x0, a3);
    boost_empty_store_entry(c, (V0, V1), 0x70, 4, 0x8, t0);
    boost_empty_store_entry(c, (V1, V0), 0x70, 4, 0xc, t1);
    boost_empty_store_entry(c, (V0, V1), 0x74, 2, 0, 0);
    boost_empty_store_entry(c, (V1, V0), 0x80, 2, 0, 0);
    boost_empty_store_entry(c, (V0, V1), 0x84, 2, 0, 0);
    boost_empty_store_entry(c, (V1, V0), 0x78, 2, 0, t2);
    boost_empty_store_entry(c, (V0, V1), 0x7c, 2, 0, t3);

    // sys->pushBank[count] = 0; sys->pushSample[count] = sample (s2).
    for (field, value) in [
        (AUDIO_SYS_PUSH_BANK, 0),
        (AUDIO_SYS_PUSH_SAMPLE, c.g[S2][0]),
    ] {
        let sys = c.g[S3][0] as u32;
        c.g[V1][0] = Addr(sys).off(AUDIO_SYS_MONITOR).lw(c);
        c.g[A3][0] = Addr(sys).off(field).lw(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(MONITOR_PUSH_COUNT).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
        Addr(c.g[V0][0] as u32).sw(c, value);
    }

    // monitor->pushGain[count] = SSXAudioSystem_GetChannelGainPtr(sys, 9, 0).
    c.g[S0][0] = Addr(c.g[S3][0] as u32).off(AUDIO_SYS_SND_ROOT).lw(c);
    call_ptr(c, 0x0028_7968, rets[1])?;
    c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
    c.g[A0][0] = c.g[S3][0];
    let monitor = c.g[A1][0] as u32;
    c.g[V1][0] = Addr(monitor).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[A2][0] = Addr(monitor).off(0x214).lw(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
    // Audio_PlayPushedSound(sys)
    call_ptr(c, 0x0029_06b8, rets[2])
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_OnRacePositionChange

/// `SSXRaceMan.humanCount` / `.cpuCount`? (`+0x7c`, `+0x80`): the two
/// counts whose sum is the number of racers.
pub(crate) const RACE_MAN_COUNT_A: u32 = 0x7c;

pub(crate) const RACE_MAN_COUNT_B: u32 = 0x80;

/// `GameModule.raceType` (+0x214): 4 is a race.
pub(crate) const GAME_MODULE_RACE_TYPE: u32 = 0x214;

/// The audio settings byte at 0x535c10 (non-zero: check the race type).
pub(crate) const AUDIO_RACE_TYPE_CHECK: u32 = 0x0053_5c10;

/// `void SSXAudioSystem_OnRacePositionChange(SSXAudioSystem* sys, Rider*
/// rider, Rider* passed, Rider* passer, int limit, int position)`
/// (0x299e28): in a race with at least two racers, for the human local
/// rider (or the rider `FindRider(sys, -1)` returns), play the "position
/// change" sting (sample 0x6d) when the rider moved into first place and
/// it is not already tracked or a replay, then have the riders taunt each
/// other (`RiderSay207D(sys, rider, passed)` when `position < limit`, else
/// `RiderSay207D(sys, passer, rider)`).
pub fn ssx_audio_system_on_race_position_change(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        c.g[S3][0] = c.g[A0][0]; // sys
        frame.save(c, S5, 0x40);
        c.g[S4][0] = c.g[A1][0]; // rider
        frame.save(c, S6, 0x30);
        c.g[S5][0] = c.g[T1][0]; // position
        frame.save(c, S7, 0x20);
        c.g[S6][0] = c.g[A3][0]; // passer
        frame.save(c, FP, 0x10);
        c.g[S7][0] = c.g[A2][0]; // passed
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        frame.save_ra(c, 0);
        c.g[FP][0] = c.g[T0][0]; // limit
                                 // BELibrary_GetInterface(BELibrary_GetInstance(), 0)
        call_ptr(c, 0x0014_dc80, 0x0029_9e70)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0029_9e7c)?;
        'out: {
            c.g[V1][0] = 0x53_0000;
            c.g[V0][0] = Addr(AUDIO_RACE_TYPE_CHECK).lb(c);
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S3][0];
                Callee {
                    addr: 0x002a_4290,
                    f: ee::s08::SSXAudioSystem_IsRaceType4,
                }
                .call(c, 0x0029_9e94)?;
                let ok = c.g[V0][0] != 0;
                frame.restore(c, S0, 0x90);
                if !ok {
                    break 'out;
                }
            }
            c.cycles += 6;
            // SSXAudioSystem_GetGameModule
            call_ptr(c, 0x0028_b1c8, 0x0029_9ea4)?;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(GAME_MODULE_RACE_TYPE).lw(c);
            c.g[V0][0] = 0x4;
            let race = c.g[V1][0] == c.g[V0][0];
            frame.restore(c, S0, 0x90);
            if !race {
                break 'out;
            }
            c.cycles += 3;
            let rider = c.g[S4][0] as u32;
            c.g[V0][0] = Addr(rider).off(RIDER_AUDIO_HUMAN).lw(c);
            c.g[V1][0] = 0;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[V0][0] = Addr(rider).off(RIDER_AUDIO_LOCAL).lw(c);
                c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
            }
            c.cycles += 2;
            c.g[A0][0] = c.g[S3][0];
            if c.g[V1][0] == 0 {
                c.cycles += 4;
                c.g[A1][0] = u64::MAX;
                // SSXAudioSystem_FindRider(sys, -1)
                call_ptr(c, 0x0028_5d98, 0x0029_9ed8)?;
                let same = c.g[V0][0] == c.g[S4][0];
                frame.restore(c, S0, 0x90);
                if !same {
                    break 'out;
                }
            }
            c.cycles += 9;
            // SSXAudioSystem_GetRaceMan (twice)
            call_ptr(c, 0x0028_b1d8, 0x0029_9ee8)?;
            c.g[S0][0] = Addr(c.g[V0][0] as u32).off(RACE_MAN_COUNT_A).lw(c);
            call_ptr(c, 0x0028_b1d8, 0x0029_9ef0)?;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(RACE_MAN_COUNT_B).lw(c);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[S0][0] = ((c.g[S0][0] as i64) < 2) as u64;
            if c.g[S0][0] != 0 {
                frame.restore(c, S0, 0x90);
                break 'out;
            }
            /// `position < limit` (slt s5, fp).
            fn ahead(c: &Cpu) -> u64 {
                ((c.g[S5][0] as i64) < (c.g[FP][0] as i64)) as u64
            }
            c.cycles += 2;
            c.g[V0][0] = ahead(c);
            if c.g[S5][0] == 0 {
                c.cycles += 6;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = 0x1;
                c.g[A2][0] = 0;
                Callee {
                    addr: 0x0029_5028,
                    f: ee::s08::SSXAudioSystem_IsTrackedSoundPlaying,
                }
                .call(c, 0x0029_9f1c)?;
                let playing = c.g[V0][0] != 0;
                c.g[V0][0] = ahead(c);
                if !playing {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S3][0];
                    // SSXAudioSystem_IsReplayPlaying
                    call_ptr(c, 0x0028_8ae0, 0x0029_9f2c)?;
                    let replay = c.g[V0][0] != 0;
                    c.g[V0][0] = ahead(c);
                    if !replay {
                        c.cycles += 160;
                        c.g[S2][0] = 0x6d; // the sample
                        audio_push_voice_and_play(c, [0x0029_a00c, 0x0029_a168, 0x0029_a188])?;
                        // SSXAudioSystem_TrackSound(sys, 1, handle,
                        // (*sys->sndRoot)->time + 200, 0, 0)
                        c.g[V1][0] = Addr(c.g[S3][0] as u32).off(AUDIO_SYS_SND_ROOT).lw(c);
                        c.g[A2][0] = c.g[V0][0];
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A1][0] = 0x1;
                        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                        c.g[T0][0] = 0;
                        c.g[T1][0] = 0;
                        c.g[A3][0] = Addr(c.g[V0][0] as u32).off(0x260).lw(c);
                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0xc8));
                        Callee {
                            addr: 0x0029_5628,
                            f: ee::s08::SSXAudioSystem_TrackSound,
                        }
                        .call(c, 0x0029_a1b0)?;
                        c.g[V0][0] = ahead(c);
                    }
                }
            }
            let say = Callee {
                addr: 0x002a_0560,
                f: ee::s08::SSXAudioSystem_RiderSay207D,
            };
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[A0][0] = c.g[S3][0];
                if c.g[S7][0] != 0 {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A2][0] = c.g[S7][0];
                    say.call(c, 0x0029_a1d0)?;
                    frame.restore(c, S0, 0x90);
                    break 'out;
                }
            } else {
                c.cycles += 2;
                c.g[A0][0] = c.g[S3][0];
                if c.g[S6][0] != 0 {
                    c.cycles += 3;
                    c.g[A1][0] = c.g[S6][0];
                    c.g[A2][0] = c.g[S4][0];
                    say.call(c, 0x0029_a1ec)?;
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x90);
        }
        c.cycles += 11;
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_IsTrackedSoundPlaying (0x295028, sound/soundsys.cpp)

/// SSXAudioSystem: the tracked-sound ring `{ int size; int* words; int
/// write; int read; }` (records of 5 words: key, handle, ..., subkey) and
/// the `Audio**` whose `*p + 0x1d8` is the bank monitor.
pub(crate) const TSP_RING_SIZE: u32 = 0x597c;

pub(crate) const TSP_RING_WORDS: u32 = 0x4;

// from the ring
pub(crate) const TSP_RING_WRITE: u32 = 0x5984;

pub(crate) const TSP_RING_READ: u32 = 0x5988;

pub(crate) const TSP_AUDIO: u32 = 0x118;

pub(crate) const TSP_BANK_MONITOR: u32 = 0x1d8;

pub(crate) const TSP_BANK_MONITOR_FIND_INSTANCE: Callee = Callee {
    addr: 0x002a_d550,
    f: ee::s08::BankMonitor_FindInstance,
};

/// Ring word `idx` (the register value the original compares) counted from
/// the read index, or -1 past the queued words or for an empty ring. The
/// caller has charged its prefix and holds write in a3 and read in a1;
/// leaves v0/v1/a0 as the original does. Returns the word (sign-extended).
pub(crate) fn tsp_ring_get(c: &mut Cpu, idx: u64) -> u64 {
    let this = r32(c, S0);
    if c.g[A3][0] == c.g[A1][0] {
        c.cycles += 2;
        return 0xffff_ffff_ffff_ffff;
    }
    c.cycles += 10;
    c.g[A0][0] = Addr(this.wrapping_add(TSP_RING_SIZE)).lw(c);
    c.g[V0][0] = sx(r32(c, A3).wrapping_sub(r32(c, A1)).wrapping_add(r32(c, A0)));
    let (_, queued) = {
        let (n, d) = (r32(c, V0), r32(c, A0));
        abi::div(c, n, d)
    };
    let in_range = (idx as i64) < (sx(queued) as i64);
    c.g[V1][0] = sx(r32(c, A1).wrapping_add(idx as u32));
    c.cycles += 2;
    if !in_range {
        return 0xffff_ffff_ffff_ffff;
    }
    c.cycles += 6;
    let (_, slot) = {
        let (n, d) = (r32(c, V1), r32(c, A0));
        abi::div(c, n, d)
    };
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
        let (_, queued) = {
            let (n, d) = (r32(c, V1), r32(c, V0));
            abi::div(c, n, d)
        };
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
// SSXAudioSystem_TrackSound (0x295628)
// ---------------------------------------------------------------------------

/// The tracked-sound ring of `int`s (5 per tracked sound): capacity, data,
/// head (write index) and tail (read index).
pub(crate) const TS_RING: u32 = 0x597c;

pub(crate) const TS_RING_DATA: u32 = 0x4;

pub(crate) const TS_RING_HEAD: u32 = 0x8;

pub(crate) const TS_RING_TAIL: u32 = 0xc;

/// The audio system's manager pointer; `*mgr + 0x1d8` is the bank monitor.
pub(crate) const TS_MGR: u32 = 0x118;

pub(crate) const TS_BANK_MONITOR: u32 = 0x1d8;

pub(crate) const TS_STOP_HANDLE: Callee = Callee {
    addr: 0x002a_d5f0,
    f: ee::s08::BankMonitor_StopHandle,
};

/// Drop the ring's oldest entry (pops 3..5 of a full ring): `extra` cycles
/// first, then tail = (tail + 1) wrapped at the capacity, unless empty.
pub(crate) fn ts_ring_drop(c: &mut Cpu, ring: u32, extra: u64) {
    c.cycles += extra;
    let head = Addr(ring + TS_RING_HEAD).lw(c);
    let tail = Addr(ring + TS_RING_TAIL).lw(c);
    c.g[A0][0] = tail;
    let next = sx((tail as u32).wrapping_add(1));
    c.g[V0][0] = next;
    if head != tail {
        c.cycles += 5;
        let cap = Addr(ring).lw(c);
        c.g[V1][0] = cap;
        Addr(ring + TS_RING_TAIL).sw(c, next);
        let fits = (next as i64) < (cap as i64);
        c.g[V0][0] = fits as u64;
        if !fits {
            Addr(ring + TS_RING_TAIL).sw(c, 0);
        }
    }
}

/// One push of `value` (`(head + 1) % cap` already in HI): unless full,
/// `data[head] = value`, head advances (wrapping). `first` is the push
/// right after the full check (head in v1, 6 setup cycles); the last push
/// does not reload v0.
pub(crate) fn ts_ring_push(c: &mut Cpu, ring: u32, value: u64, first: bool, last: bool) {
    // (head + 1) % cap.
    if first {
        c.cycles += 6;
        c.g[A1][0] = sx(ring);
        let cap = Addr(ring).lw(c);
        c.g[V0][0] = cap;
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
        let n = r32(c, V1);
        abi::div(c, n, cap as u32);
    } else {
        c.cycles += 5;
        let cap = Addr(ring).lw(c);
        c.g[V1][0] = cap;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        let n = r32(c, V0);
        abi::div(c, n, cap as u32);
    }
    c.cycles += 4;
    let tail = Addr(ring + TS_RING_TAIL).lw(c);
    if first {
        c.g[V1][0] = tail;
    } else {
        c.g[V0][0] = tail;
    }
    let next_head = c.hi[0];
    c.g[A0][0] = next_head;
    if next_head != tail {
        c.cycles += 11;
        let ring_base = r32(c, A1);
        let head = Addr(ring_base + TS_RING_HEAD).lw(c);
        let data = Addr(ring_base + TS_RING_DATA).lw(c);
        let at = sx(((head as u32) << 2).wrapping_add(data as u32));
        Addr(at as u32).sw(c, value);
        let head = Addr(ring + TS_RING_HEAD).lw(c);
        let cap = Addr(ring).lw(c);
        let head = sx((head as u32).wrapping_add(1));
        c.g[V1][0] = head;
        let fits = (head as i64) < (cap as i64);
        c.g[V0][0] = fits as u64;
        Addr(ring + TS_RING_HEAD).sw(c, head);
        if !fits {
            c.cycles += 1;
            Addr(ring + TS_RING_HEAD).sw(c, 0);
        }
        c.cycles += 1;
    }
    if !last {
        c.g[V0][0] = Addr(ring + TS_RING_HEAD).lw(c);
    }
}

/// `void SSXAudioSystem_TrackSound(SSXAudioSystem* self, int evt, int h,
/// int t, int a, int idx)` (0x295628): remembers a playing sound (five
/// ints) in the tracked-sound ring. When the ring is full the oldest entry
/// is dropped first and its sound handle (second int) stopped through the
/// bank monitor (`BankMonitor_StopHandle(bm, handle, 1, 0.0)`).
pub fn ssx_audio_system_track_sound(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S5, 0x10);
        c.g[S4][0] = c.g[T0][0];
        frame.save_ra(c, 0);
        c.g[S5][0] = c.g[T1][0];
        let this = r32(c, S0);
        let ring = this.wrapping_add(TS_RING);
        let head = Addr(ring + TS_RING_HEAD).lw(c);
        let cap = Addr(ring).lw(c);
        c.g[A2][0] = head;
        c.g[A1][0] = cap;
        c.g[V0][0] = sx((head as u32).wrapping_add(1));
        let n = r32(c, V0);
        abi::div(c, n, cap as u32);

        // Full?
        c.cycles += 4;
        let tail = Addr(ring + TS_RING_TAIL).lw(c);
        c.g[A0][0] = tail;
        c.g[V1][0] = c.hi[0];
        if c.hi[0] == tail {
            // Pop 1 (the event).
            c.cycles += 2;
            let next = sx((tail as u32).wrapping_add(1));
            c.g[V0][0] = next;
            if head != tail {
                c.cycles += 3;
                let fits = (next as i64) < (cap as i64);
                c.g[V1][0] = fits as u64;
                Addr(ring + TS_RING_TAIL).sw(c, next);
                if !fits {
                    c.cycles += 1;
                    Addr(ring + TS_RING_TAIL).sw(c, 0);
                }
            }
            // Pop 2: the sound handle (-1 when empty).
            c.cycles += 7;
            let mgr = Addr(this.wrapping_add(TS_MGR)).lw(c);
            c.g[V0][0] = mgr;
            c.g[A2][0] = sx(ring);
            let head = Addr(ring + TS_RING_HEAD).lw(c);
            c.g[A0][0] = head;
            let bank_owner = Addr(mgr as u32).lw(c);
            c.g[V1][0] = bank_owner;
            let tail = Addr(ring + TS_RING_TAIL).lw(c);
            c.g[A1][0] = tail;
            let monitor = sx((bank_owner as u32).wrapping_add(TS_BANK_MONITOR));
            c.g[A3][0] = monitor;
            if head != tail {
                c.cycles += 10;
                let tail = Addr(ring + TS_RING_TAIL).lw(c);
                let next = sx((tail as u32).wrapping_add(1));
                c.g[A1][0] = next;
                let data = Addr(ring + TS_RING_DATA).lw(c);
                c.g[A0][0] = data;
                let cap = Addr(ring).lw(c);
                let at = sx(((tail as u32) << 2).wrapping_add(data as u32));
                let handle = Addr(at as u32).lw(c);
                c.g[V0][0] = handle;
                let fits = (next as i64) < (cap as i64);
                c.g[V1][0] = fits as u64;
                Addr(ring + TS_RING_TAIL).sw(c, next);
                if !fits {
                    c.cycles += 1;
                    Addr(ring + TS_RING_TAIL).sw(c, 0);
                }
                c.cycles += 1;
                c.g[A1][0] = handle;
            } else {
                c.cycles += 2;
                c.g[A1][0] = u64::MAX;
            }
            // BankMonitor_StopHandle(monitor, handle, 1, 0.0f).
            c.cycles += 8;
            abi::set_fpr(c, 12, FpuF32::ZERO);
            c.g[A0][0] = monitor;
            c.g[A2][0] = 1;
            TS_STOP_HANDLE.call(c, 0x0029_5704)?;
            // Pops 3..5.
            ts_ring_drop(c, ring, 0);
            ts_ring_drop(c, ring, 4);
            ts_ring_drop(c, ring, 4);
            c.cycles += 1;
        }
        c.g[V1][0] = Addr(ring + TS_RING_HEAD).lw(c);

        // Push the five values.
        let s1 = c.g[S1][0];
        ts_ring_push(c, ring, s1, true, false);
        let s2 = c.g[S2][0];
        ts_ring_push(c, ring, s2, false, false);
        let s3 = c.g[S3][0];
        ts_ring_push(c, ring, s3, false, false);
        let s4 = c.g[S4][0];
        ts_ring_push(c, ring, s4, false, false);
        let s5 = c.g[S5][0];
        ts_ring_push(c, ring, s5, false, true);
        frame.restore(c, S0, 0x60);
        c.cycles += 8;
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
// SSXAudioSystem_StartBoostSound (0x298d90)
// ---------------------------------------------------------------------------

/// Rider fields: boost active and boost amount (both non-zero to start).
pub(crate) const BOOST_RIDER_ACTIVE: u32 = 0x874;

pub(crate) const BOOST_RIDER_AMOUNT: u32 = 0x87c;

/// Audio system fields: the boost voice handle and the boost start count
/// (the start voice only plays on the first start).
pub(crate) const BOOST_HANDLE: u32 = 0x59e0;

pub(crate) const BOOST_COUNT: u32 = 0x59e4;

/// The boost sounds' timer callback key (8 bytes, `D_004A36A8`).
pub(crate) const BOOST_TIMER_KEY: u32 = 0x004a_36a8;

/// The play-record track (`*self`): write cursor at +0x1f4, the default
/// record at +0x1dc (0x18 bytes), the record arrays' descriptor at +0x1d8.
pub(crate) const BOOST_TRACK_CURSOR: u32 = 0x1f4;

pub(crate) const BOOST_TRACK_ARRAYS: u32 = 0x1d8;

pub(crate) const BOOST_IS_MUTED: u32 = 0x0028_8ae0;

pub(crate) const BOOST_KILL_TIMERS: Callee = Callee {
    addr: 0x002a_dde0,
    f: ee::s08::BankMonitor_KillTimersByCallback,
};

pub(crate) const BOOST_MEMSET: u32 = 0x0041_6210;

pub(crate) const BOOST_GET_CHANNEL_GAIN: u32 = 0x0028_7968;

pub(crate) const BOOST_PLAY_PUSHED: u32 = 0x0029_06b8;

/// `lw(arrays + base_off) + (cursor << shift)` with the cursor re-read, the
/// sum in v0 (`in_v0`) or v1 and the array base in the other; then the
/// store of `value` at `+field`.
pub(crate) fn boost_store(
    c: &mut Cpu,
    base_off: u32,
    shift: u32,
    field: u32,
    value: u64,
    in_v0: bool,
) {
    let track = r32(c, S0);
    let arrays = r32(c, S1);
    let cursor = Addr(track + BOOST_TRACK_CURSOR).lw(c);
    let base = Addr(arrays.wrapping_add(base_off)).lw(c);
    let at = sx(((cursor as u32) << shift).wrapping_add(base as u32));
    let (a, b) = if in_v0 { (V0, V1) } else { (V1, V0) };
    c.g[a][0] = at;
    c.g[b][0] = base;
    Addr((at as u32).wrapping_add(field)).sw(c, value);
}

/// Push a play record for `sound` on the track and play it (the inlined
/// `PushPlayParams` + `PlaySound` of the original); returns with the
/// handle in v0. `s2_sound`: the second copy keeps the sound id in s2 (set
/// first) instead of v1.
pub(crate) fn boost_push_and_play(
    c: &mut Cpu,
    cycles: u64,
    sound: u64,
    s2_sound: bool,
    rets: [u32; 3],
) -> crate::call::Flow {
    c.cycles += cycles;
    let this = r32(c, S3);
    let track = Addr(this).lw(c);
    c.g[S0][0] = track;
    let track = track as u32;
    c.g[A1][0] = 0;
    c.g[A2][0] = 8;
    let cursor = Addr(track + BOOST_TRACK_CURSOR).lw(c);
    c.g[S1][0] = sx(track.wrapping_add(BOOST_TRACK_ARRAYS));
    if s2_sound {
        c.g[S2][0] = sound;
    }
    let arrays = r32(c, S1);
    // cursor++.
    let cursor = sx((cursor as u32).wrapping_add(1));
    c.g[V0][0] = cursor;
    Addr(track + BOOST_TRACK_CURSOR).sw(c, cursor);
    // records[cursor] = the default record (0x18 bytes).
    let cursor = Addr(track + BOOST_TRACK_CURSOR).lw(c);
    let records = Addr(arrays.wrapping_add(0x20)).lw(c);
    c.g[V0][0] = records;
    let stride = r32(c, T1);
    let row = abi::mult(c, cursor as u32, stride);
    let dst = sx(row.wrapping_add(records as u32));
    c.g[V1][0] = dst;
    let w0 = Addr(track + 0x1dc).read_u64_unaligned(c);
    let w1 = Addr(track + 0x1e4).read_u64_unaligned(c);
    let w2 = Addr(track + 0x1ec).read_u64_unaligned(c);
    c.g[V0][0] = w0;
    c.g[A0][0] = w1;
    c.g[A3][0] = w2;
    Addr(dst as u32).write_u64_unaligned(c, w0);
    Addr((dst as u32).wrapping_add(8)).write_u64_unaligned(c, w1);
    Addr((dst as u32).wrapping_add(0x10)).write_u64_unaligned(c, w2);
    // track->flags[cursor] = 0.
    let cursor = Addr(track + BOOST_TRACK_CURSOR).lw(c);
    let at = sx(((cursor as u32) << 2).wrapping_add(track));
    c.g[V0][0] = at;
    Addr((at as u32).wrapping_add(0x1fc)).sw(c, 0);
    boost_store(c, 0x34, 2, 0, 0, false);
    // priorities[cursor] = (s8) records[cursor].first byte.
    let cursor = Addr(track + BOOST_TRACK_CURSOR).lw(c);
    let cursor2 = Addr(track + BOOST_TRACK_CURSOR).lw(c);
    let records = Addr(arrays.wrapping_add(0x20)).lw(c);
    c.g[T0][0] = records;
    let stride = r32(c, T1);
    let row = abi::mult(c, cursor2 as u32, stride);
    let base = Addr(arrays.wrapping_add(0x38)).lw(c);
    c.g[V0][0] = base;
    let dst = sx(((cursor as u32) << 2).wrapping_add(base as u32));
    c.g[A3][0] = dst;
    let src = sx(row.wrapping_add(records as u32));
    c.g[A0][0] = src;
    let first = Addr(src as u32).lb(c);
    c.g[V1][0] = first;
    Addr(dst as u32).sw(c, first);
    boost_store(c, 0x3c, 2, 0, 0, true);
    // memset(&arrays->params[cursor], 0, 8).
    let cursor = Addr(track + BOOST_TRACK_CURSOR).lw(c);
    c.g[A0][0] = sx((((cursor as u32) << 3).wrapping_add(arrays)).wrapping_add(0x40));
    call_ptr(c, BOOST_MEMSET, rets[0])?;
    // The rest of the record: volume 100, pan 90, 50, pitch 127, 1.
    let track = r32(c, S0);
    let arrays = r32(c, S1);
    let _ = (track, arrays);
    c.g[A3][0] = 0x64;
    c.g[T0][0] = 0x5a;
    c.g[T1][0] = 0x32;
    c.g[T2][0] = 0x7f;
    c.g[T3][0] = 1;
    boost_store(c, 0x60, 2, 0, 0, false);
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = 9;
    c.g[A2][0] = 0;
    boost_store(c, 0x64, 2, 0, 0, false);
    boost_store(c, 0x68, 2, 0, 0, true);
    boost_store(c, 0x6c, 2, 0, 0, false);
    boost_store(c, 0x70, 4, 0x4, 0, true);
    let v = c.g[A3][0];
    boost_store(c, 0x70, 4, 0x0, v, false);
    let v = c.g[T0][0];
    boost_store(c, 0x70, 4, 0x8, v, true);
    let v = c.g[T1][0];
    boost_store(c, 0x70, 4, 0xc, v, false);
    boost_store(c, 0x74, 2, 0, 0, true);
    boost_store(c, 0x80, 2, 0, 0, false);
    boost_store(c, 0x84, 2, 0, 0, true);
    let v = c.g[T2][0];
    boost_store(c, 0x78, 2, 0, v, false);
    let v = c.g[T3][0];
    boost_store(c, 0x7c, 2, 0, v, true);
    // self->handles[cursor] = 0; self->sounds[cursor] = sound.
    let this = r32(c, S3);
    for (field, last) in [(0x4u32, false), (0x8, true)] {
        let track = Addr(this).lw(c);
        c.g[V1][0] = track;
        let table = Addr(this + field).lw(c);
        c.g[A3][0] = table;
        let cursor = Addr((track as u32).wrapping_add(BOOST_TRACK_CURSOR)).lw(c);
        let at = sx(((cursor as u32) << 2).wrapping_add(table as u32));
        c.g[V0][0] = at;
        if last {
            let value = if s2_sound {
                c.g[S2][0]
            } else {
                c.g[V1][0] = sound;
                sound
            };
            Addr(at as u32).sw(c, value);
        } else {
            Addr(at as u32).sw(c, 0);
        }
    }
    // gains[cursor] = SSXAudioSystem_GetChannelGainPtr(...); play it.
    c.g[S0][0] = Addr(this + 0x118).lw(c);
    call_ptr(c, BOOST_GET_CHANNEL_GAIN, rets[1])?;
    let owner = Addr(r32(c, S0)).lw(c);
    c.g[A1][0] = owner;
    c.g[A0][0] = c.g[S3][0];
    let cursor = Addr((owner as u32).wrapping_add(BOOST_TRACK_CURSOR)).lw(c);
    let gains = Addr((owner as u32).wrapping_add(0x214)).lw(c);
    c.g[A2][0] = gains;
    let at = sx(((cursor as u32) << 2).wrapping_add(gains as u32));
    c.g[V1][0] = at;
    let gain = c.g[V0][0];
    Addr(at as u32).sw(c, gain);
    call_ptr(c, BOOST_PLAY_PUSHED, rets[2])
}

/// `void SSXAudioSystem_StartBoostSound(SSXAudioSystem* self, Rider* rider,
/// int withVoice)` (0x298d90): unless the audio is muted (0x288ae0) or the
/// rider's boost is off, kill the boost timers, play the boost start sound
/// (0x7a, only on the first start; its handle kept at `+0x59e0`), the
/// boost voice (0x4a) when asked, and count the start.
pub fn ssx_audio_system_start_boost_sound(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        call_ptr(c, BOOST_IS_MUTED, 0x0029_8db8)?;
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            let rider = r32(c, S0);
            let active = Addr(rider + BOOST_RIDER_ACTIVE).lw(c);
            c.g[V0][0] = active;
            c.g[A1][0] = 0;
            if active != 0 {
                c.cycles += 2;
                let amount = Addr(rider + BOOST_RIDER_AMOUNT).lw(c);
                c.g[V0][0] = amount;
                c.g[A1][0] = (0 < amount) as u64;
            }
            c.cycles += 2;
            c.g[V0][0] = 0x004a_0000;
            if c.g[A1][0] != 0 {
                c.cycles += 9;
                let this = r32(c, S3);
                let mgr = Addr(this + 0x118).lw(c);
                c.g[V1][0] = mgr;
                c.g[A1][0] = Addr(BOOST_TIMER_KEY).read_u64_unaligned(c);
                let owner = Addr(mgr as u32).lw(c);
                c.g[A0][0] = sx((owner as u32).wrapping_add(0x1d8));
                BOOST_KILL_TIMERS.call(c, 0x0029_8df4)?;
                let count = Addr(r32(c, S3) + BOOST_COUNT).lw(c);
                c.g[V0][0] = count;
                c.g[T1][0] = 0x18;
                if count == 0 {
                    boost_push_and_play(
                        c,
                        149,
                        0x7a,
                        false,
                        [0x0029_8ed0, 0x0029_9030, 0x0029_9050],
                    )?;
                    let handle = c.g[V0][0];
                    Addr(r32(c, S3) + BOOST_HANDLE).sw(c, handle);
                }
                c.cycles += 2;
                c.g[T1][0] = 0x18;
                if c.g[S2][0] != 0 {
                    boost_push_and_play(
                        c,
                        148,
                        0x4a,
                        true,
                        [0x0029_9130, 0x0029_928c, 0x0029_92ac],
                    )?;
                }
                c.cycles += 3;
                let this = r32(c, S3);
                let count = sx((Addr(this + BOOST_COUNT).lw(c) as u32).wrapping_add(1));
                c.g[V0][0] = count;
                Addr(this + BOOST_COUNT).sw(c, count);
            }
            c.cycles += 1;
        }
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
// SSXAudioSystem_CalcSurfaceLoopGroup1.

/// The group-1 pitch and volume curves (`.rodata`, five points each, copied
/// to the stack).
#[cfg(test)]
pub(crate) const SURFACE_LOOP1_PITCH_CURVE: u32 = 0x0048_2bc8;

pub(crate) const SURFACE_LOOP1_VOLUME_CURVE: u32 = 0x0048_2bf0;

/// The smoothed loop level per surface (`SSXAudioSystem + 0x59b8`).
pub(crate) const SURFACE_LOOP1_LEVELS: u32 = 0x59b8;

/// Copy one unaligned doubleword (`ldl/ldr` into GPR `reg`, `sdl/sdr`).
pub(crate) fn surface_loop1_copy_dword(c: &mut Cpu, reg: usize, src: u32, dst: u32) {
    c.ldl(reg, src.wrapping_add(7));
    c.ldr(reg, src);
    c.sdl(reg, dst.wrapping_add(7));
    c.sdr(reg, dst);
}

/// `void SSXAudioSystem_CalcSurfaceLoopGroup1(SSXAudioSystem* a, Rider* r,
/// int* pitch, int* volume)` (0x293740): the board-surface loop of group 1.
/// The level is the rider's speed factor (`+0x758`) times
/// `1 - (+0x750 + +0x754)`, boosted (`* +0x75c * 10`) when `+0x75c` is
/// under the threshold `gp-0x45c0`. The surface index comes from the
/// rider's virtual (`+0x6c0`, slot 0x38); a jump of more than the
/// threshold from that surface's previous level moves a quarter of the way
/// instead. The stored level is evaluated on the pitch and volume curves
/// (`Audio_EvalCurve5`): `pitch = int(curve)`, `volume = int(curve * 4096)`.
pub fn ssx_audio_system_calc_surface_loop_group1(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0xb0);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 0); // 1.0
        frame.save(c, S0, 0x90);
        frame.save(c, S1, 0x80);
        c.g[S0][0] = c.g[A0][0]; // audio system
        frame.save(c, S2, 0x70);
        c.g[S1][0] = c.g[A2][0]; // pitch out
        frame.save_fpr(c, 21, 0xa8);
        c.g[S2][0] = c.g[A3][0]; // volume out
        frame.save_ra(c, 0x60);
        frame.save_fpr(c, 20, 0xa0);
        let rider = c.g[A1][0] as u32;
        c.lwc1(21, (c.g[GP][0] as u32).wrapping_sub(0x45c0)); // threshold
        c.lwc1(1, rider.wrapping_add(0x754));
        c.lwc1(2, rider.wrapping_add(0x750));
        c.lwc1(12, rider.wrapping_add(0x75c));
        c.add_s(1, 1, 2);
        c.lwc1(20, rider.wrapping_add(0x758));
        c.c_lt_s(12, 21);
        c.sub_s(0, 0, 1);
        let boost = c.fcc();
        c.mul_s(20, 20, 0); // level
        if boost {
            c.cycles += 4;
            c.mul_s(1, 20, 12);
            c.g[AT][0] = 0x4120_0000;
            c.mtc1(AT, 0); // 10.0
            c.mul_s(20, 1, 0);
        }

        // Surface index from the rider's virtual.
        c.cycles += 18;
        c.g[V1][0] = Addr(rider.wrapping_add(0x6c0)).lw(c);
        c.g[A1][0] = sx(rider.wrapping_add(0x6c0));
        let vt = c.g[V1][0];
        let slot = VSlot::read(c, vt, 0x38);
        c.g[A0][0] = slot.delta;
        c.g[V0][0] = slot.func;
        c.g[A0][0] = slot.this(c.g[A1][0]);
        call_ptr(c, slot.func as u32, 0x0029_37c0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(SURFACE_LOOP1_LEVELS));
        c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.lwc1(2, c.g[V0][0] as u32); // previous level
        c.sub_s(1, 2, 20);
        c.abs_s(0, 1);
        c.c_lt_s(21, 0);
        let jump = c.fcc();
        c.g[V1][0] = 0x0048_0000;
        if jump {
            c.cycles += 3;
            c.c_lt_s(20, 2);
            if c.fcc() {
                // Falling: move down a quarter of the way.
                c.cycles += 5;
                c.g[AT][0] = 0x3e80_0000;
                c.mtc1(AT, 0);
                c.mul_s(0, 1, 0);
                c.sub_s(20, 2, 0);
            } else {
                c.cycles += 4;
                c.c_lt_s(2, 20);
                c.g[V1][0] = 0x0048_0000;
                if c.fcc() {
                    // Rising: move up a quarter of the way.
                    c.cycles += 5;
                    c.sub_s(0, 20, 2);
                    c.g[AT][0] = 0x3e80_0000;
                    c.mtc1(AT, 1);
                    c.mul_s(0, 0, 1);
                    c.add_s(20, 2, 0);
                }
            }
        }

        // Store the level, copy both curves to the stack, evaluate them.
        c.cycles += 68;
        c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[T0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2bc8));
        let pitch_src = c.g[T0][0] as u32;
        let base = sp(c);
        for (i, reg) in [4usize, 6, 7, 4, 6].into_iter().enumerate() {
            let o = 8 * i as u32;
            surface_loop1_copy_dword(c, reg, pitch_src + o, base + o);
        }
        c.swc1(20, c.g[V0][0] as u32);
        c.g[A1][0] = 0x0048_0000;
        c.g[S0][0] = sx(base.wrapping_add(0x30));
        c.g[A0][0] = c.g[SP][0];
        c.g[A3][0] = sx(SURFACE_LOOP1_VOLUME_CURVE);
        for (i, reg) in [2usize, 3, 6, 2, 3].into_iter().enumerate() {
            let o = 8 * i as u32;
            surface_loop1_copy_dword(c, reg, SURFACE_LOOP1_VOLUME_CURVE + o, base + 0x30 + o);
        }
        c.mov_s(12, 20);
        Callee {
            addr: 0x0029_0b58,
            f: ee::s07::Audio_EvalCurve5,
        }
        .call(c, 0x0029_38fc)?;
        c.cvt_w_s(1, 0);
        c.swc1(1, c.g[S1][0] as u32);
        c.g[A0][0] = c.g[S0][0];
        c.mov_s(12, 20);
        Callee {
            addr: 0x0029_0b58,
            f: ee::s07::Audio_EvalCurve5,
        }
        .call(c, 0x0029_3910)?;
        c.g[AT][0] = 0x4580_0000;
        c.mtc1(AT, 1); // 4096.0
        c.mul_s(0, 0, 1);
        c.cvt_w_s(1, 0);
        c.swc1(1, c.g[S2][0] as u32);
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore_ra(c, 0x60);
        frame.restore_fpr(c, 21, 0xa8);
        frame.restore_fpr(c, 20, 0xa0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod wipeout_rider {

    /// Sound source position handed to the pushed sound (`&rider->...`).
    pub const SOURCE: u32 = 0x110;

    /// Rider state; 2 and 3 pick the fixed impact sample.
    pub const STATE: u32 = 0x438;

    /// The rider's controller object (embedded; vptr at +0).
    pub const CONTROLLER: u32 = 0x6c0;

    /// Impact speed (`float`), the input of the volume curve.
    pub const SPEED: u32 = 0x770;

    /// Listener index, is-human, is-local.
    pub const LISTENER: u32 = 0x870;

    pub const HUMAN: u32 = 0x874;

    pub const LOCAL: u32 = 0x87c;

    /// Controller vtable slots: a flag (true: check the view range) and the
    /// position.
    pub const VT_FLAG: u32 = 0x48;

    pub const VT_POSITION: u32 = 0x28;
}

pub(crate) mod wipeout_queue {

    pub const RECORDS: u32 = 0x20;

    pub const RECORD_SIZE: u64 = 0x18;

    pub const VOLUME: u32 = 0x38;

    pub const SLOTS: u32 = 0x40;

    pub const PARAMS: u32 = 0x70;

    /// Inline per-entry arrays of the monitor itself.
    pub const MON_SOURCE: u32 = 0x1fc;

    pub const MON_GAIN: u32 = 0x214;

    pub const MON_LISTENER: u32 = 0x23c;

    pub const MON_REPLAY_FLAG: u32 = 0x258;

    pub const MON_REPLAY_LISTENER: u32 = 0x25c;
}

/// Game-mode byte: 2 enables the replay check.
pub(crate) const WIPEOUT_MODE_BYTE: u32 = 0x0053_5c11;

/// `lw base+off` on whole registers.
#[inline(always)]
pub(crate) fn pwi_lw(c: &mut Cpu, base: u64, off: u32) -> u64 {
    Addr((base as u32).wrapping_add(off)).lw(c)
}

/// `ri = count << shift` (count from `addr`), as the original's
/// `lw; sll; addu table` triple; returns the element address.
#[inline(always)]
pub(crate) fn pwi_index(
    c: &mut Cpu,
    ri: usize,
    rt: usize,
    count_at: u64,
    table: u64,
    shift: u32,
) -> u32 {
    c.g[ri][0] = pwi_lw(c, count_at, 0);
    c.g[rt][0] = table;
    c.g[ri][0] = sx((c.g[ri][0] as u32) << shift);
    c.g[ri][0] = sx((c.g[ri][0] as u32).wrapping_add(c.g[rt][0] as u32));
    c.g[ri][0] as u32
}

/// `void SSXAudioSystem_PlayWipeoutImpact(SSXAudioSystem* sys, Rider* rider)`
/// (0x296310): the impact sound of a wipeout. Unless the rider's controller
/// flag is clear and the rider is local, the rider must be within 100 units
/// of a view (`FindViewInRange`). The volume comes from a five-key curve over
/// the impact speed (clamped to 0..127); the sample is fixed (bank 10,
/// sample 9) in rider states 2/3, else a random one of 0x30..0x33. A new
/// entry goes on the bank monitor's push queue (volume 100, pan 90, 50,
/// 127), with its gain (channel kind 5), the rider as source and listener,
/// and for a human local rider outside replays (mode byte 2) the replay
/// listener; then `Audio_PlayPushedSound(sys)`.
pub fn ssx_audio_system_play_wipeout_impact(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S0, 0xc0);
        frame.save(c, S3, 0x90);
        frame.save(c, S5, 0x70);
        frame.save(c, S1, 0xb0);
        frame.save(c, S2, 0xa0);
        frame.save(c, S4, 0x80);
        frame.save(c, S6, 0x60);
        frame.save(c, S7, 0x50);
        frame.save(c, FP, 0x40);
        frame.save_ra(c, 0x30);
        c.g[S3][0] = c.g[A0][0]; // sys
        c.g[S5][0] = c.g[A1][0]; // rider
        c.g[S0][0] = sx(r32(c, S5).wrapping_add(wipeout_rider::CONTROLLER));
        if pwi_in_range(c)? {
            pwi_volume_and_sample(c)?;
            pwi_replay_check(c)?;
            pwi_push_entry(c)?;
            pwi_finish_entry(c)?;
            c.cycles += 2;
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x0029_06b8, 0x0029_6838)?; // Audio_PlayPushedSound
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xc0);
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
        Ok(())
    })
}

/// The range gate: false when no view is within 100 units of the rider.
/// Leaves v0 = 0x480000 when it passes.
pub(crate) fn pwi_in_range(c: &mut Cpu) -> Result<bool, crate::call::Unwound> {
    use wipeout_rider::*;
    let controller = c.g[S0][0];
    // controller->flag()
    let vt = pwi_lw(c, c.g[S5][0], CONTROLLER);
    c.g[V1][0] = vt;
    let m = VSlot::read(c, vt, VT_FLAG);
    c.g[V0][0] = m.func;
    c.g[A0][0] = m.this(controller);
    call_ptr(c, m.func as u32, 0x0029_635c)?;
    if c.g[V0][0] != 0 {
        c.g[V1][0] = pwi_lw(c, c.g[S5][0], CONTROLLER);
    } else {
        c.cycles += 3;
        let local = pwi_lw(c, c.g[S5][0], LOCAL) != 0;
        c.g[V0][0] = 0x48_0000;
        if local {
            return Ok(true);
        }
        c.cycles += 1;
        c.g[V1][0] = pwi_lw(c, c.g[S5][0], CONTROLLER);
    }
    // FindViewInRange(sys, controller->position(), 100.0)
    c.cycles += 11;
    let vt = c.g[V1][0];
    let m = VSlot::read(c, vt, VT_POSITION);
    c.g[V0][0] = m.func;
    c.g[A0][0] = m.this(c.g[S0][0]);
    call_ptr(c, m.func as u32, 0x0029_6384)?;
    c.g[AT][0] = 0x42c8_0000;
    c.f[12] = 0x42c8_0000; // 100.0
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, 0x0028_8b40, 0x0029_6398)?;
    let out = (c.g[V0][0] as i64) < 0;
    c.g[V0][0] = 0x48_0000;
    Ok(!out)
}

/// Volume (s4) from the speed curve, sample bank / sample (s7 / s6).
pub(crate) fn pwi_volume_and_sample(c: &mut Cpu) -> crate::call::Flow {
    use wipeout_rider::*;
    c.cycles += 36;
    let speed = pwi_lw(c, c.g[S5][0], SPEED);
    c.f[12] = speed as u32;
    // The curve at 0x482da0 (five keys, five values), by value on the
    // stack (five doublewords).
    c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2da0));
    let src = c.g[A3][0] as u32;
    let dst = sp(c);
    for (i, r) in [V1, A1, A2, V1, A1].into_iter().enumerate() {
        let off = 8 * i as u32;
        c.g[r][0] = c.ld(src.wrapping_add(off));
        c.sd(dst.wrapping_add(off), c.g[r][0]);
    }
    c.g[A0][0] = sx(dst);
    call_ptr(c, 0x0029_0b58, 0x0029_6400)?; // Audio_EvalCurve5
    c.g[S7][0] = 0;
    c.cvt_w_s(1, 0);
    c.mfc1(S4, 1);
    c.g[A0][0] = 0x7f;
    c.g[V1][0] = u64::MAX;
    let state = pwi_lw(c, c.g[S5][0], STATE);
    c.g[A2][0] = state;
    c.g[A1][0] = 2;
    c.g[V0][0] = ((c.g[S4][0] as i64) < 128) as u64;
    if c.g[V0][0] == 0 {
        c.g[S4][0] = 0x7f;
    }
    c.g[V1][0] = (-1i64 < c.g[S4][0] as i64) as u64;
    if c.g[V1][0] == 0 {
        c.g[S4][0] = 0;
    }
    let fixed = if state == 2 {
        true
    } else {
        c.cycles += 3;
        c.g[V0][0] = 3;
        if state != 3 {
            c.g[V0][0] = pwi_lw(c, c.g[S3][0], 0x118);
            false
        } else {
            true
        }
    };
    if fixed {
        c.cycles += 3;
        c.g[S7][0] = 0xa;
        c.g[S6][0] = 0x9;
        return Ok(());
    }
    // Audio_Rand(monitor->queue head) -> 0x30 + rand15 * 3 / 0x7fff
    c.cycles += 8;
    c.g[V1][0] = pwi_lw(c, c.g[V0][0], 0);
    c.g[A0][0] = pwi_lw(c, c.g[V1][0], 0x1d8);
    Callee {
        addr: 0x002a_df60,
        f: ee::s08::Audio_Rand,
    }
    .call(c, 0x0029_6454)?;
    c.g[V0][0] &= 0x7fff;
    c.g[A0][0] = 0x7fff;
    c.g[V1][0] = sx((c.g[V0][0] as u32) << 1);
    c.cycles += 4;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let (q, _) = crate::abi::divu(c, c.g[V1][0] as u32, 0x7fff);
    c.g[V1][0] = sx(q);
    c.g[S6][0] = sx(q.wrapping_add(0x30));
    Ok(())
}

/// fp = human local rider in mode 2 and no replay playing; s0 = monitor.
pub(crate) fn pwi_replay_check(c: &mut Cpu) -> crate::call::Flow {
    use wipeout_rider::*;
    c.cycles += 4;
    c.g[V0][0] = pwi_lw(c, c.g[S5][0], HUMAN);
    c.g[FP][0] = 0;
    c.g[V1][0] = 0;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = pwi_lw(c, c.g[S5][0], LOCAL);
        c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
    }
    c.cycles += 2;
    if c.g[V1][0] != 0 {
        c.cycles += 10;
        call_ptr(c, 0x0014_dc80, 0x0029_64a0)?; // BELibrary_GetInstance
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0029_64ac)?; // BELibrary_GetInterface
        c.g[V1][0] = 0x53_0000;
        c.g[A0][0] = 2;
        c.g[V0][0] = Addr(WIPEOUT_MODE_BYTE).lb(c);
        if c.g[V0][0] == 2 {
            c.cycles += 4;
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x0028_8ae0, 0x0029_64c8)?; // IsReplayPlaying
            c.g[FP][0] = (c.g[V0][0] < 1) as u64;
        }
    }
    c.g[S0][0] = pwi_lw(c, c.g[S3][0], 0);
    Ok(())
}

/// A new push-queue entry on the monitor (s0; s1 = its queue): count += 1,
/// copy the record template, clear the entry, the default parameters, and
/// the sample (s7 / s6) on the system's push tables.
pub(crate) fn pwi_push_entry(c: &mut Cpu) -> crate::call::Flow {
    use crate::game::sound::soundsys::boost_empty_store_entry as store;
    use wipeout_queue::*;
    c.cycles += 153;
    let monitor = c.g[S0][0];
    let count_at = sx((monitor as u32).wrapping_add(0x1f4));
    c.g[S2][0] = RECORD_SIZE;
    c.g[A1][0] = 0;
    c.g[A2][0] = 8;
    c.g[V0][0] = pwi_lw(c, monitor, 0x1f4);
    c.g[S1][0] = sx((monitor as u32).wrapping_add(0x1d8));
    let queue = c.g[S1][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    Addr(count_at as u32).sw(c, c.g[V0][0]);
    c.g[V1][0] = pwi_lw(c, count_at, 0);
    c.g[V0][0] = pwi_lw(c, queue, RECORDS);
    c.mult(0, V1, V1, S2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    // Record template (monitor + 0x1dc, 24 bytes, unaligned).
    let m = monitor as u32;
    c.ldl(2, m.wrapping_add(0x1e3));
    c.ldr(2, m.wrapping_add(0x1dc));
    c.ldl(4, m.wrapping_add(0x1eb));
    c.ldr(4, m.wrapping_add(0x1e4));
    c.ldl(7, m.wrapping_add(0x1f3));
    c.ldr(7, m.wrapping_add(0x1ec));
    let record = c.g[V1][0] as u32;
    c.sdl(2, record.wrapping_add(0x7));
    c.sdr(2, record);
    c.sdl(4, record.wrapping_add(0xf));
    c.sdr(4, record.wrapping_add(0x8));
    c.sdl(7, record.wrapping_add(0x17));
    c.sdr(7, record.wrapping_add(0x10));
    // monitor->source[count] = 0 (inline).
    c.g[V0][0] = pwi_lw(c, count_at, 0);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx(m.wrapping_add(c.g[V0][0] as u32));
    Addr(c.g[V0][0] as u32).off(MON_SOURCE).sw(c, 0);
    store(c, (V1, V0), 0x34, 2, 0, 0);
    // volume[count] = the record's first byte (sign-extended).
    c.g[A3][0] = pwi_lw(c, count_at, 0);
    c.g[A0][0] = pwi_lw(c, count_at, 0);
    c.g[T0][0] = pwi_lw(c, queue, RECORDS);
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
    c.mult(0, A0, A0, S2);
    c.g[V0][0] = pwi_lw(c, queue, VOLUME);
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
    c.g[V1][0] = Addr(c.g[A0][0] as u32).lb(c);
    Addr(c.g[A3][0] as u32).sw(c, c.g[V1][0]);
    store(c, (V0, V1), 0x3c, 2, 0, 0);
    // memset(&queue->slots[count], 0, 8)
    c.g[A0][0] = pwi_lw(c, count_at, 0);
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
    c.g[A0][0] = sx((queue as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(SLOTS));
    call_ptr(c, 0x0041_6210, 0x0029_65a0)?;

    // Parameters: volume 100, pan 90, 50, 127, 1.
    c.g[A0][0] = 0x64;
    c.g[A1][0] = 0x5a;
    c.g[A2][0] = 0x32;
    c.g[A3][0] = 0x7f;
    c.g[T0][0] = 0x1;
    store(c, (V1, V0), 0x60, 2, 0, 0);
    store(c, (V0, V1), 0x64, 2, 0, 0);
    store(c, (V1, V0), 0x68, 2, 0, 0);
    store(c, (V0, V1), 0x6c, 2, 0, 0);
    store(c, (V1, V0), PARAMS, 4, 0x4, 0);
    let (a0, a1, a2, a3, t0) = (c.g[A0][0], c.g[A1][0], c.g[A2][0], c.g[A3][0], c.g[T0][0]);
    store(c, (V0, V1), PARAMS, 4, 0x0, a0);
    store(c, (V1, V0), PARAMS, 4, 0x8, a1);
    store(c, (V0, V1), PARAMS, 4, 0xc, a2);
    store(c, (V1, V0), 0x74, 2, 0, 0);
    store(c, (V0, V1), 0x80, 2, 0, 0);
    store(c, (V1, V0), 0x84, 2, 0, 0);
    store(c, (V0, V1), 0x78, 2, 0, a3);
    store(c, (V1, V0), 0x7c, 2, 0, t0);

    // sys->pushBank[count] = s7; sys->pushSample[count] = s6.
    let sys = c.g[S3][0];
    c.g[A0][0] = pwi_lw(c, sys, 0);
    c.g[V1][0] = pwi_lw(c, sys, 0x4);
    let at = pwi_index(
        c,
        V0,
        V1,
        sx((c.g[A0][0] as u32).wrapping_add(0x1f4)),
        c.g[V1][0],
        2,
    );
    Addr(at).sw(c, c.g[S7][0]);
    c.g[V1][0] = pwi_lw(c, sys, 0);
    c.g[A0][0] = pwi_lw(c, sys, 0x8);
    let at = pwi_index(
        c,
        V0,
        A0,
        sx((c.g[V1][0] as u32).wrapping_add(0x1f4)),
        c.g[A0][0],
        2,
    );
    Addr(at).sw(c, c.g[S6][0]);

    // root monitor: volume[count] = s4, then the record's volume byte =
    // clamp(volume[count], 0, 127).
    c.g[V1][0] = pwi_lw(c, sys, 0x118);
    c.g[A1][0] = pwi_lw(c, c.g[V1][0], 0);
    c.g[V0][0] = pwi_lw(c, c.g[A1][0], 0x1f4);
    c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1d8));
    c.g[V1][0] = pwi_lw(c, c.g[A2][0], VOLUME);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    Addr(c.g[V0][0] as u32).sw(c, c.g[S4][0]);
    c.g[A0][0] = pwi_lw(c, c.g[A1][0], 0x1f4);
    c.g[V1][0] = pwi_lw(c, c.g[A2][0], 0x1c);
    c.g[A3][0] = pwi_lw(c, c.g[A2][0], VOLUME);
    c.mult(0, A0, A0, S2);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[A1][0] = pwi_lw(c, c.g[A2][0], RECORDS);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
    c.g[V0][0] = pwi_lw(c, c.g[V1][0], 0);
    c.g[V0][0] = ((c.g[V0][0] as i64) < 127) as u64;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    // The current volume word: queue->volume[queue->count].
    let vol_at = |c: &mut Cpu| -> u64 {
        let n = pwi_lw(c, c.g[A2][0], 0x1c);
        sx(((n as u32) << 2).wrapping_add(c.g[A3][0] as u32))
    };
    let mut clamp_top = true;
    if c.g[V0][0] != 0 {
        c.cycles += 6;
        c.g[V0][0] = vol_at(c);
        c.g[V1][0] = pwi_lw(c, c.g[V0][0], 0);
        c.g[V0][0] = 0;
        clamp_top = (c.g[V1][0] as i64) >= 0;
    }
    if clamp_top {
        c.cycles += 7;
        c.g[V0][0] = vol_at(c);
        c.g[V1][0] = pwi_lw(c, c.g[V0][0], 0);
        c.g[V1][0] = ((c.g[V1][0] as i64) < 127) as u64;
        if c.g[V1][0] == 0 {
            c.g[V0][0] = 0x7f;
        } else {
            c.cycles += 4;
            c.g[V0][0] = vol_at(c);
            c.g[V0][0] = Addr(c.g[V0][0] as u32).lbu(c);
        }
    }
    Ok(())
}

/// Store the volume byte, then the entry's gain, source, listener and (fp)
/// replay listener.
pub(crate) fn pwi_finish_entry(c: &mut Cpu) -> crate::call::Flow {
    use wipeout_queue::*;
    c.cycles += 30;
    Addr(c.g[A0][0] as u32).sb(c, c.g[V0][0]);
    // monitor->gain[count] = SSXAudioSystem_GetChannelGainPtr(sys, 5, 0)
    c.g[A2][0] = 0;
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = 0x5;
    c.g[S0][0] = pwi_lw(c, c.g[S3][0], 0x118);
    call_ptr(c, 0x0028_7968, 0x0029_6790)?;
    c.g[A3][0] = pwi_lw(c, c.g[S0][0], 0);
    c.g[T0][0] = sx(r32(c, S5).wrapping_add(wipeout_rider::SOURCE));
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = c.g[S5][0];
    c.g[V1][0] = pwi_lw(c, c.g[A3][0], 0x1f4);
    c.g[A2][0] = pwi_lw(c, c.g[A3][0], MON_GAIN);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
    // monitor->source[count] = &rider->source (inline array).
    c.g[A2][0] = pwi_lw(c, c.g[S3][0], 0x118);
    c.g[V1][0] = pwi_lw(c, c.g[A2][0], 0);
    c.g[V0][0] = pwi_lw(c, c.g[V1][0], 0x1f4);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    Addr(c.g[V1][0] as u32).off(MON_SOURCE).sw(c, c.g[T0][0]);
    // monitor->listener[count] = SSXAudioSystem_GetRiderListenerIndex(sys, rider)
    c.g[S0][0] = pwi_lw(c, c.g[S3][0], 0x118);
    call_ptr(c, 0x0028_8a20, 0x0029_67d4)?;
    c.g[A0][0] = pwi_lw(c, c.g[S0][0], 0);
    c.g[V1][0] = pwi_lw(c, c.g[A0][0], 0x1f4);
    c.g[A1][0] = pwi_lw(c, c.g[A0][0], MON_LISTENER);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
    Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
    if c.g[FP][0] != 0 {
        // Live play of the local human: flag and listener for the replay.
        c.cycles += 16;
        for (table, listener) in [(MON_REPLAY_FLAG, false), (MON_REPLAY_LISTENER, true)] {
            c.g[A0][0] = pwi_lw(c, c.g[S3][0], 0x118);
            c.g[A1][0] = if listener {
                pwi_lw(c, c.g[S5][0], wipeout_rider::LISTENER)
            } else {
                1
            };
            c.g[V1][0] = pwi_lw(c, c.g[A0][0], 0);
            c.g[V0][0] = pwi_lw(c, c.g[V1][0], 0x1f4);
            c.g[A0][0] = pwi_lw(c, c.g[V1][0], table);
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            Addr(c.g[V0][0] as u32).sw(c, c.g[A1][0]);
        }
    }
    Ok(())
}

/// One field of the voice record a play request pushes: reads the write
/// cursor (`bank + 0x1f4`, s0) into `idx_reg` and the array pointer at
/// `params + table` (s1) into `base_reg`; returns (and leaves in `idx_reg`)
/// `array + (cursor << shift)`.
pub(crate) fn grab_trick_voice_slot(
    c: &mut Cpu,
    idx_reg: usize,
    base_reg: usize,
    table: u32,
    shift: u32,
) -> u32 {
    let bank = c.g[S0][0] as u32;
    let params = c.g[S1][0] as u32;
    c.g[idx_reg][0] = c.lw(bank.wrapping_add(0x1f4));
    c.g[base_reg][0] = c.lw(params.wrapping_add(table));
    let at = ((c.g[idx_reg][0] as u32) << shift).wrapping_add(c.g[base_reg][0] as u32);
    c.g[idx_reg][0] = sx(at);
    at
}

/// `void SSXAudioSystem::PlayGrabTrickSound(Rider* rider)` (0x29a530): when
/// the rider has a grab sound pending (+0x874 and +0x87c > 0), pushes a play
/// record for sound 0x77 on channel 9 into the MIDI bank (`*this`): bumps
/// the cursor (+0x1f4), copies the 24-byte template (+0x1dc) into the
/// record array, clears/sets the record's fields (volume 0x64, pan 0x5a,
/// 0x32, 0x7f, 1), stores the channel gain pointer
/// (`GetChannelGainPtr(this, 9, 0)`) and plays it (`Audio_PlayPushedSound`).
pub fn ssx_audio_system_play_grab_trick_sound(c: &mut Cpu) {
    const PENDING: u32 = 0x874;
    const PENDING_COUNT: u32 = 0x87c;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = 0;
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        let rider = c.g[A1][0] as u32;
        let pending = c.lw(rider.wrapping_add(PENDING));
        c.g[V0][0] = pending;
        c.g[S3][0] = c.g[A0][0];
        if pending != 0 {
            c.cycles += 2;
            let count = c.lw(rider.wrapping_add(PENDING_COUNT));
            c.g[V0][0] = count;
            c.g[V1][0] = (0 < count) as u64;
        }
        c.cycles += 2;
        c.g[T1][0] = 0x18;
        if c.g[V1][0] != 0 {
            c.cycles += 148;
            let me = c.g[S3][0] as u32;
            let bank = c.lw(me) as u32;
            c.g[S0][0] = sx(bank);
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x8;
            let cursor = c.lw(bank.wrapping_add(0x1f4));
            c.g[S1][0] = sx(bank.wrapping_add(0x1d8));
            c.g[S2][0] = 0x77;
            let cursor = sx((cursor as u32).wrapping_add(1));
            c.g[V0][0] = cursor;
            c.sw(bank.wrapping_add(0x1f4), cursor);
            // The record (cursor * 0x18 into params+0x20) gets the template.
            let params = c.g[S1][0] as u32;
            c.g[V1][0] = c.lw(bank.wrapping_add(0x1f4));
            c.g[V0][0] = c.lw(params.wrapping_add(0x20));
            c.mult(0, V1, V1, T1);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.ldl(V0, bank.wrapping_add(0x1e3));
            c.ldr(V0, bank.wrapping_add(0x1dc));
            c.ldl(A0, bank.wrapping_add(0x1eb));
            c.ldr(A0, bank.wrapping_add(0x1e4));
            c.ldl(A3, bank.wrapping_add(0x1f3));
            c.ldr(A3, bank.wrapping_add(0x1ec));
            let record = c.g[V1][0] as u32;
            c.sdl(V0, record.wrapping_add(0x7));
            c.sdr(V0, record);
            c.sdl(A0, record.wrapping_add(0xf));
            c.sdr(A0, record.wrapping_add(0x8));
            c.sdl(A3, record.wrapping_add(0x17));
            c.sdr(A3, record.wrapping_add(0x10));
            let cur = c.lw(bank.wrapping_add(0x1f4));
            let at = sx(((cur as u32) << 2).wrapping_add(bank));
            c.g[V0][0] = at;
            c.sw((at as u32).wrapping_add(0x1fc), 0);
            let at = grab_trick_voice_slot(c, V1, V0, 0x34, 2);
            c.sw(at, 0);
            // field 0x38[cursor] = (s8) record_bytes[cursor * 0x18]
            c.g[A3][0] = c.lw(bank.wrapping_add(0x1f4));
            c.g[A0][0] = c.lw(bank.wrapping_add(0x1f4));
            c.g[T0][0] = c.lw(params.wrapping_add(0x20));
            c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
            c.mult(0, A0, A0, T1);
            c.g[V0][0] = c.lw(params.wrapping_add(0x38));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
            let byte = c.lb(c.g[A0][0] as u32);
            c.g[V1][0] = byte;
            c.sw(c.g[A3][0] as u32, byte);
            let at = grab_trick_voice_slot(c, V0, V1, 0x3c, 2);
            c.sw(at, 0);
            // memset(&params->names[cursor] (+0x40, 8 bytes each), 0, 8)
            let cur = c.lw(bank.wrapping_add(0x1f4));
            c.g[A0][0] = sx(params.wrapping_add((cur as u32) << 3).wrapping_add(0x40));
            call_ptr(c, 0x0041_6210, 0x0029_a63c)?;
            let at = grab_trick_voice_slot(c, V1, V0, 0x60, 2);
            c.g[A3][0] = 0x64;
            c.g[T0][0] = 0x5a;
            c.g[T1][0] = 0x32;
            c.g[T2][0] = 0x7f;
            c.sw(at, 0);
            c.g[T3][0] = 0x1;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = 0x9;
            c.g[A2][0] = 0;
            let at = grab_trick_voice_slot(c, V1, V0, 0x64, 2);
            c.sw(at, 0);
            let at = grab_trick_voice_slot(c, V0, V1, 0x68, 2);
            c.sw(at, 0);
            let at = grab_trick_voice_slot(c, V1, V0, 0x6c, 2);
            c.sw(at, 0);
            // The 16-byte envelope at +0x70: 0, 0x64, 0x5a, 0x32.
            let at = grab_trick_voice_slot(c, V0, V1, 0x70, 4);
            c.sw(at.wrapping_add(0x4), 0);
            let at = grab_trick_voice_slot(c, V1, V0, 0x70, 4);
            c.sw(at, c.g[A3][0]);
            let at = grab_trick_voice_slot(c, V0, V1, 0x70, 4);
            c.sw(at.wrapping_add(0x8), c.g[T0][0]);
            let at = grab_trick_voice_slot(c, V1, V0, 0x70, 4);
            c.sw(at.wrapping_add(0xc), c.g[T1][0]);
            let at = grab_trick_voice_slot(c, V0, V1, 0x74, 2);
            c.sw(at, 0);
            let at = grab_trick_voice_slot(c, V1, V0, 0x80, 2);
            c.sw(at, 0);
            let at = grab_trick_voice_slot(c, V0, V1, 0x84, 2);
            c.sw(at, 0);
            let at = grab_trick_voice_slot(c, V1, V0, 0x78, 2);
            c.sw(at, c.g[T2][0]);
            let at = grab_trick_voice_slot(c, V0, V1, 0x7c, 2);
            c.sw(at, c.g[T3][0]);
            // this->+4[cursor] = 0, this->+8[cursor] = 0x77
            for (table, value) in [(0x4u32, 0u64), (0x8, 0x77)] {
                let me = c.g[S3][0] as u32;
                c.g[V1][0] = c.lw(me);
                c.g[A3][0] = c.lw(me.wrapping_add(table));
                let cur = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                let at = sx(((cur as u32) << 2).wrapping_add(c.g[A3][0] as u32));
                c.g[V0][0] = at;
                let value = if table == 0x8 { c.g[S2][0] } else { value };
                c.sw(at as u32, value);
            }
            // gain = GetChannelGainPtr(this, 9, 0); channels->gain[cursor] = gain
            let me = c.g[S3][0] as u32;
            c.g[S0][0] = c.lw(me.wrapping_add(0x118));
            call_ptr(c, 0x0028_7968, 0x0029_a798)?;
            let channels = c.lw(c.g[S0][0] as u32);
            c.g[A1][0] = channels;
            c.g[A0][0] = c.g[S3][0];
            let cur = c.lw((channels as u32).wrapping_add(0x1f4));
            c.g[A2][0] = c.lw((channels as u32).wrapping_add(0x214));
            let at = sx(((cur as u32) << 2).wrapping_add(c.g[A2][0] as u32));
            c.g[V1][0] = at;
            c.sw(at as u32, c.g[V0][0]);
            call_ptr(c, 0x0029_06b8, 0x0029_a7b8)?; // Audio_PlayPushedSound
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// 0x002974a0 SSXAudioSystem_PlayObjectSound

/// Offset of the object-sound "suppressed" word in the audio system (a
/// non-negative value means object sounds are off).
pub(crate) const POS_SUPPRESS: u32 = 0x62a8;

/// `void SSXAudioSystem_PlayObjectSound(SSXAudioSystem* self, WorldObject*
/// obj, int sound)` (0x2974a0): play a world object's one-shot sound at the
/// object's position (`obj+0x40`). Sound 0 is nothing; sound 0x66 only plays
/// when the current track's flag word is set; nothing plays while
/// `self+0x62a8` is non-negative or the object already has a tracked sound.
/// The sound number picks the bank: <100 bank 2, <150 bank 8 (minus 100),
/// <200 bank 9 (minus 150), else bank 4 (minus 200). The play record is
/// pushed inline (the `PushPlayParams` sequence), then the 3D position,
/// radius 300 / no vanish, listener and priority 0x50 are set, the sound is
/// started and tracked against the object.
pub fn ssx_audio_system_play_object_sound(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save(c, S3, 0x50);
        frame.save(c, S6, 0x20);
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        c.g[S0][0] = c.g[A2][0];
        c.g[S3][0] = c.g[A0][0];
        c.g[S6][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        let played = c.g[S0][0] == 0 || play_object_sound_body(c)?;
        if played {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x80);
        c.cycles += 9;
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// The body of [`ssx_audio_system_play_object_sound`] for a non-zero sound
/// (in s0; self in s3, object in s6). `Ok(true)` when it ran to the end (the
/// sound was played), `Ok(false)` on an early exit.
pub(crate) fn play_object_sound_body(c: &mut Cpu) -> Result<bool, crate::call::Unwound> {
    c.cycles += 3;
    c.g[V0][0] = 0x66;
    if c.g[S0][0] == 0x66 {
        // Sound 0x66: only when the current track's flag is set.
        c.cycles += 10;
        call_ptr(c, 0x0014_dc80, 0x0029_74e8)?; // cBE_getBE
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0029_74f4)?; // BELibrary_GetInterface(be, 0)
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_4bc0, 0x0029_74fc)?; // the current track
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        if c.g[V1][0] == 0 {
            return Ok(false);
        }
        c.cycles += 1;
    }
    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(POS_SUPPRESS));
    c.cycles += 2;
    if (c.g[V0][0] as i64) >= 0 {
        return Ok(false);
    }
    c.cycles += 6;
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = c.g[S6][0];
    c.g[A2][0] = 0;
    Callee {
        addr: 0x0029_5028,
        f: ee::s08::SSXAudioSystem_IsTrackedSoundPlaying,
    }
    .call(c, 0x0029_7524)?;
    if c.g[V0][0] != 0 {
        return Ok(false);
    }
    // The object's position, copied to the frame (the 3D source).
    c.cycles += 5;
    c.lq(2, (c.g[S6][0] as u32).wrapping_add(0x40));
    c.g[S5][0] = 0x2;
    let n = c.g[S0][0] as i64;
    c.g[V1][0] = (n < 100) as u64;
    c.sq(2, c.g[SP][0] as u32);
    // Bank (s5) and sound within the bank (s4).
    if n < 100 {
        c.cycles += 2;
        c.g[S4][0] = c.g[S0][0];
    } else {
        c.cycles += 3;
        c.g[V0][0] = (n < 150) as u64;
        c.g[S4][0] = sx((n as u32).wrapping_sub(100));
        if n < 150 {
            c.cycles += 2;
            c.g[S5][0] = 0x8;
        } else {
            c.cycles += 3;
            c.g[V0][0] = (n < 200) as u64;
            c.g[S4][0] = sx((n as u32).wrapping_sub(150));
            if n < 200 {
                c.cycles += 2;
                c.g[S5][0] = 0x9;
            } else {
                c.cycles += 2;
                c.g[S4][0] = sx((n as u32).wrapping_sub(200));
                c.g[S5][0] = 0x4;
            }
        }
    }
    // Push a play record (the inlined PushPlayParams), bank s5, sound s4;
    // then the channel volume clamped to 0..=127 into the record's first byte.
    c.cycles += 154;
    c.g[S0][0] = c.lw(c.g[S3][0] as u32);
    c.g[S2][0] = 0x18;
    c.g[A1][0] = 0u64;
    c.g[A2][0] = 0x8;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
    c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
    c.mult(0, 3, 3, 18);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1e3));
    c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1dc));
    c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x1eb));
    c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x1e4));
    c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1f3));
    c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1ec));
    c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
    c.sdr(2, c.g[V1][0] as u32);
    c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
    c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
    c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
    c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0u64);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sw(c.g[V1][0] as u32, 0u64);
    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
    c.mult(0, 4, 4, 18);
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
    c.g[V1][0] = c.lb(c.g[A0][0] as u32);
    c.sw(c.g[A3][0] as u32, c.g[V1][0]);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sw(c.g[V0][0] as u32, 0u64);
    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    call_ptr(c, 0x00416210, 0x0029764c)?;
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[A0][0] = 0x64;
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
    c.g[A1][0] = 0x5a;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[A2][0] = 0x32;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A3][0] = 0x7f;
    c.sw(c.g[V1][0] as u32, 0u64);
    c.g[T0][0] = 0x1;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sw(c.g[V0][0] as u32, 0u64);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sw(c.g[V1][0] as u32, 0u64);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sw(c.g[V0][0] as u32, 0u64);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sw(c.g[V0][0] as u32, c.g[A0][0]);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sw(c.g[V1][0] as u32, 0u64);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sw(c.g[V0][0] as u32, 0u64);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sw(c.g[V1][0] as u32, 0u64);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sw(c.g[V0][0] as u32, c.g[A3][0]);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sw(c.g[V1][0] as u32, c.g[T0][0]);
    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sw(c.g[V0][0] as u32, c.g[S5][0]);
    c.g[V1][0] = c.lw(c.g[S3][0] as u32);
    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.sw(c.g[V0][0] as u32, c.g[S4][0]);
    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
    c.g[A1][0] = c.lw(c.g[V1][0] as u32);
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
    c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1d8));
    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sw(c.g[V0][0] as u32, c.g[A3][0]);
    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
    c.g[A3][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
    c.mult(0, 4, 4, 18);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x20));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
    c.g[V0][0] = ((c.g[V0][0] as i64) < 127i64) as u64;
    let at_least_max = c.g[V0][0] == 0u64;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    let mut negative = false;
    if !at_least_max {
        c.cycles += 6;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        negative = (c.g[V1][0] as i64) < 0;
        c.g[V0][0] = 0;
    }
    if !negative {
        c.cycles += 7;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        c.g[V1][0] = ((c.g[V1][0] as i64) < 127) as u64;
        if c.g[V1][0] == 0 {
            c.g[V0][0] = 0x7f;
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A3][0] as u32));
            c.g[V0][0] = c.lbu(c.g[V0][0] as u32);
        }
    }
    // Channel gain, 3D source (position on the frame), radius 300 / no
    // vanish, listener, priority 0x50; play and track it against the object.
    c.cycles += 64;
    c.sb(c.g[A0][0] as u32, c.g[V0][0]);
    c.g[A2][0] = 0u64;
    c.g[A1][0] = 0x5;
    c.g[A0][0] = c.g[S3][0];
    c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
    call_ptr(c, 0x00287968, 0x0029783c)?;
    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
    c.g[A2][0] = 0x1;
    c.g[AT][0] = 0x43960000;
    c.mtc1(1, 12);
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
    c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x214));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[AT][0] = 0xffffffffbf800000;
    c.mtc1(1, 13);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
    c.g[V1][0] = c.lw(c.g[A0][0] as u32);
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sw((c.g[V1][0] as u32).wrapping_add(0x1fc), c.g[SP][0]);
    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
    c.g[V1][0] = c.lw(c.g[A0][0] as u32);
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x20c));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    c.sw(c.g[V0][0] as u32, c.g[A2][0]);
    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
    Callee {
        addr: 0x002a9988,
        f: ee::s08::SndPlayParams_SetRadiusVanish,
    }
    .call(c, 0x002978ac)?;
    c.g[A1][0] = c.g[SP][0];
    c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, 0x00288940, 0x002978bc)?;
    c.g[A1][0] = c.lw(c.g[S0][0] as u32);
    c.g[A0][0] = c.g[S3][0];
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
    c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x23c));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
    c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
    c.g[A1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x250));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = 0x50;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.sw(c.g[V0][0] as u32, c.g[V1][0]);
    call_ptr(c, 0x002906b8, 0x002978fc)?;
    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
    c.g[A2][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = c.g[S6][0];
    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
    c.g[T0][0] = 0u64;
    c.g[T1][0] = 0u64;
    c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x260));
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x32));
    Callee {
        addr: 0x00295628,
        f: ee::s08::SSXAudioSystem_TrackSound,
    }
    .call(c, 0x00297924)?;
    Ok(true)
}

pub(crate) mod rs207d {

    /// `float` along-track progress; speech only while it is negative.
    pub const PROGRESS: u32 = 0x470;

    /// `float` heading angle (radians) of the passed rider.
    pub const HEADING: u32 = 0x4cc;

    /// The rider's embedded scripting object (its vtable pointer).
    pub const OBJ: u32 = 0x6c0;

    /// "Local" flag (0: not the local player).
    pub const LOCAL: u32 = 0x87c;

    /// Object vtable slots (`{i16 delta; pad; fn}`): direction vector,
    /// position, "is AI".
    pub const VT_DIR: u32 = 0x10;

    pub const VT_POS: u32 = 0x28;

    pub const VT_IS_AI: u32 = 0x48;

    /// Audio system: the speech system and the last pass time.
    pub const SPEECH: u32 = 0x5560;

    pub const LAST_PASS: u32 = 0x582c;

    /// Race manager: the race time.
    pub const RACE_TIME: u32 = 0x8;

    /// gp: speech settings (`+0x550` speech mode), the post hook, the
    /// listener radius and the three distance limits.
    pub const SPEECH_SETTINGS_GP: u32 = 0x84c;

    pub const SPEECH_MODE: u32 = 0x550;

    pub const POST_HOOK_GP: u32 = 0x173c;

    pub const RADIUS_GP: u32 = 0x5e8;

    pub const MIN_GAP_GP: u32 = 0x456c;

    pub const MAX_GAP_GP: u32 = 0x4568;

    pub const FAR_GAP_GP: u32 = 0x4564;
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_StartObjectLoop

/// `SSXAudioSystem` word that gates object loops: they start only while it
/// is negative.
pub(crate) const SOL_GATE: u32 = 0x62a8;

/// `SSXAudioSystem` object-loop slots: 30 records of 0x30 bytes.
pub(crate) const SOL_SLOTS: u32 = 0x5a00;

pub(crate) const SOL_SLOT_SIZE: u32 = 0x30;

pub(crate) const SOL_SLOT_COUNT: u32 = 30;

/// Slot fields: in use (1 = playing), the object, the bone (-1: the whole
/// object), the 3D position the push entry points at, the sound id and the
/// bank-monitor handle.
pub(crate) const SOL_ACTIVE: u32 = 0x0;

pub(crate) const SOL_OBJECT: u32 = 0x4;

pub(crate) const SOL_BONE: u32 = 0x8;

pub(crate) const SOL_POSITION: u32 = 0x10;

pub(crate) const SOL_SOUND: u32 = 0x20;

pub(crate) const SOL_HANDLE: u32 = 0x24;

/// The object's scene node (vtable at node + 0xc) and its fallback position.
pub(crate) const SOL_OBJ_NODE: u32 = 0xc;

pub(crate) const SOL_OBJ_POSITION: u32 = 0x40;

/// Node vtable slots ({delta: i16, pad, fn}): the node's transform, the
/// bone lookup, the bone's transform (all return a matrix; its
/// translation row is at +0x30).
pub(crate) const SOL_VT_TRANSFORM: u32 = 0xc0;

pub(crate) const SOL_VT_HAS_BONE: u32 = 0xd0;

pub(crate) const SOL_VT_BONE_TRANSFORM: u32 = 0xe8;

/// Sound ids with a track check: 0x66 only plays on tracks whose record
/// says so; 201 gets the long radius on track kind 8.
pub(crate) const SOL_TRACK_SOUND: u64 = 0x66;

pub(crate) const SOL_RADIUS_SOUND: u64 = 0xc9;

/// Push-queue tables of the bank monitor (`*sys->sndRoot`): the gain
/// pointers and the priorities, indexed by the push count.
pub(crate) const SOL_MONITOR_GAINS: u32 = 0x214;

pub(crate) const SOL_MONITOR_PRIORITIES: u32 = 0x250;

/// The push entry's position pointer array (`monitor + 0x1fc + 4 * count`).
pub(crate) const SOL_MONITOR_POSITIONS: u32 = 0x1fc;

pub(crate) const SOL_STOP_HANDLE: Callee = Callee {
    addr: 0x002a_d5f0,
    f: ee::s08::BankMonitor_StopHandle,
};

pub(crate) const SOL_SET_RADIUS_VANISH: Callee = Callee {
    addr: 0x002a_9988,
    f: ee::s08::SndPlayParams_SetRadiusVanish,
};

/// `cBE_getBE`, `SSXAudioSystem_GetBELibrary`, `BELibrary_GetInterface`,
/// the current track record (`0x43d950 + 100 * track`), `memset`,
/// `SSXAudioSystem_GetChannelGainPtr`, `Audio_PlayPushedSound` (called
/// through the function table).
pub(crate) const SOL_GET_BE: u32 = 0x0014_dc80;

pub(crate) const SOL_GET_BE_LIBRARY: u32 = 0x0028_b1e8;

pub(crate) const SOL_GET_INTERFACE: u32 = 0x0014_dd58;

pub(crate) const SOL_TRACK_RECORD: u32 = 0x0014_4bc0;

pub(crate) const SOL_MEMSET: u32 = 0x0041_6210;

pub(crate) const SOL_GET_CHANNEL_GAIN: u32 = 0x0028_7968;

pub(crate) const SOL_PLAY_PUSHED: u32 = 0x0029_06b8;

/// `void SSXAudioSystem_StartObjectLoop(SSXAudioSystem* sys, Object* obj,
/// int sound, int bone)` (0x297950): start a looping 3D sound attached to
/// an object (or one of its bones). Sound 0 does nothing; sound 0x66 only
/// on tracks whose record allows it; nothing while the system's loop gate
/// (`+0x62a8`) is non-negative or the object has no scene node. A loop of
/// the same sound already playing on the object is stopped first. The loop
/// takes a free slot of 30 (none free: nothing), whose position comes from
/// the node's (or bone's) transform or the object's position; the sound id
/// selects bank 2 (< 100), 8 (< 150), 9 (< 200) or 4 with the sample index
/// relative to the bank's base. It is pushed on the bank monitor's queue
/// (volume 100, pan 90, 50, 127; radius 300, or 400 with priority 50 for
/// sound 201 on track kind 8, otherwise priority 80), played, and the
/// handle kept in the slot.
pub fn ssx_audio_system_start_object_loop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        c.g[S1][0] = c.g[A1][0]; // obj
        frame.save(c, S3, 0x60);
        c.g[S2][0] = c.g[A2][0]; // sound
        frame.save(c, S5, 0x40);
        c.g[S3][0] = c.g[A0][0]; // sys
        frame.save(c, S0, 0x90);
        c.g[S5][0] = c.g[A3][0]; // bone
        frame.save(c, S4, 0x50);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save_ra(c, 0x10);
        frame.save_fpr(c, 20, 0xa0);
        start_object_loop_body(c, frame)?;
        c.cycles += 11;
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore_ra(c, 0x10);
        frame.restore_fpr(c, 20, 0xa0);
        frame.close(c);
        Ok(())
    })
}

/// Everything between prologue and epilogue; every way out restores s0
/// (the original reloads it in branch delay slots, or at 0x297e84).
pub(crate) fn start_object_loop_body(c: &mut Cpu, frame: Frame) -> Flow {
    if !start_object_loop_allowed(c, frame)? {
        return Ok(());
    }
    if !start_object_loop_claim_slot(c, frame)? {
        return Ok(());
    }
    start_object_loop_position(c)?;
    start_object_loop_bank(c);
    start_object_loop_radius(c)?;
    start_object_loop_push_and_play(c)?;
    c.cycles += 1;
    frame.restore(c, S0, 0x90);
    Ok(())
}

/// The early outs: sound 0, sound 0x66 on a track whose record word is 0,
/// a non-negative gate, an object without a scene node. Leaves a1 = 1.
pub(crate) fn start_object_loop_allowed(c: &mut Cpu, frame: Frame) -> Result<bool, Unwound> {
    if c.g[S2][0] == 0 {
        c.cycles += 1;
        frame.restore(c, S0, 0x90);
        return Ok(false);
    }
    c.cycles += 3;
    c.g[V0][0] = SOL_TRACK_SOUND;
    if c.g[S2][0] != c.g[V0][0] {
        c.g[V0][0] = Addr(r32(c, S3)).off(SOL_GATE).lw(c);
    } else {
        c.cycles += 10;
        call_ptr(c, SOL_GET_BE, 0x0029_79a4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, SOL_GET_INTERFACE, 0x0029_79b0)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, SOL_TRACK_RECORD, 0x0029_79b8)?;
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        frame.restore(c, S0, 0x90);
        if c.g[V1][0] == 0 {
            return Ok(false);
        }
        c.cycles += 1;
        c.g[V0][0] = Addr(r32(c, S3)).off(SOL_GATE).lw(c);
    }
    c.cycles += 2;
    frame.restore(c, S0, 0x90);
    if (c.g[V0][0] as i64) >= 0 {
        return Ok(false);
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, S1)).off(SOL_OBJ_NODE).lw(c);
    c.g[A1][0] = 1;
    Ok(c.g[V0][0] != 0)
}

/// Stop the first slot playing this sound on this object, then take the
/// first free slot (s4 = its index). False (s0 restored) when all 30 are
/// in use.
pub(crate) fn start_object_loop_claim_slot(c: &mut Cpu, frame: Frame) -> Result<bool, Unwound> {
    c.cycles += 3;
    c.g[S0][0] = c.g[S3][0];
    c.g[A0][0] = sx(r32(c, S3).wrapping_add(SOL_SLOT_COUNT * SOL_SLOT_SIZE));
    c.g[V1][0] = c.g[S0][0];
    loop {
        // 0x2979e8: slot at v1 (= s0).
        c.cycles += 3;
        let slot = Addr(r32(c, V1)).off(SOL_SLOTS);
        c.g[V0][0] = slot.off(SOL_ACTIVE).lw(c);
        if c.g[V0][0] == c.g[A1][0] {
            c.cycles += 3;
            c.g[V0][0] = slot.off(SOL_OBJECT).lw(c);
            if c.g[V0][0] == c.g[S1][0] {
                c.cycles += 3;
                c.g[V0][0] = slot.off(SOL_SOUND).lw(c);
                if c.g[V0][0] == c.g[S2][0] {
                    c.g[V0][0] = Addr(r32(c, S3)).off(AUDIO_SYS_SND_ROOT).lw(c);
                    start_object_loop_stop_slot(c)?;
                    break;
                }
                c.cycles += 1;
            }
        }
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(SOL_SLOT_SIZE));
        // 0x297a10
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[A0][0] as i64)) as u64;
        c.g[V1][0] = c.g[S0][0];
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0029_79e8);
        }
    }

    // 0x297a1c: the first free slot.
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, S3)).off(SOL_SLOTS + SOL_ACTIVE).lw(c);
    c.g[S4][0] = 0;
    if c.g[V0][0] == 0 {
        c.cycles += 3;
        c.g[V0][0] = 1;
        Addr(r32(c, S3))
            .off(SOL_SLOTS + SOL_ACTIVE)
            .sw(c, c.g[V0][0]);
    } else {
        c.cycles += 1;
        c.g[S4][0] = sx(r32(c, S4).wrapping_add(1));
        loop {
            // 0x297a5c
            c.cycles += 3;
            let more = (c.g[S4][0] as i64) < SOL_SLOT_COUNT as i64;
            c.g[V0][0] = SOL_SLOT_SIZE as u64;
            if !more {
                break;
            }
            c.cycles += 5;
            c.mult(0, V1, S4, V0);
            c.g[A0][0] = sx(r32(c, V1).wrapping_add(r32(c, S3)));
            c.g[V1][0] = Addr(r32(c, A0)).off(SOL_SLOTS + SOL_ACTIVE).lw(c);
            if c.g[V1][0] == 0 {
                c.cycles += 2;
                c.g[V0][0] = 1;
                Addr(r32(c, A0))
                    .off(SOL_SLOTS + SOL_ACTIVE)
                    .sw(c, c.g[V0][0]);
                break;
            }
            c.g[S4][0] = sx(r32(c, S4).wrapping_add(1));
            if event_due(c) {
                deliver_event_here(c, 0x0029_7a5c);
            }
        }
    }

    // 0x297a84
    c.cycles += 3;
    let full = c.g[S4][0] == SOL_SLOT_COUNT as u64;
    c.g[V0][0] = SOL_SLOT_SIZE as u64;
    if full {
        c.cycles += 1;
        frame.restore(c, S0, 0x90);
        return Ok(false);
    }
    Ok(true)
}

/// 0x297a34: `BankMonitor_StopHandle(&(*sys->sndRoot)->monitor,
/// slot->handle, 0.0f, 1)` and free the slot (s0 = slot - 0x5a00, v0 =
/// sys->sndRoot).
pub(crate) fn start_object_loop_stop_slot(c: &mut Cpu) -> Flow {
    c.cycles += 8;
    c.g[A2][0] = 1;
    c.g[A1][0] = Addr(r32(c, S0)).off(SOL_SLOTS + SOL_HANDLE).lw(c);
    c.g[A0][0] = Addr(r32(c, V0)).lw(c);
    c.f[12] = 0; // fade time 0.0
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(MONITOR_PUSH_QUEUE));
    SOL_STOP_HANDLE.call(c, 0x0029_7a4c)?;
    Addr(r32(c, S0)).off(SOL_SLOTS + SOL_ACTIVE).sw(c, 0);
    if event_due(c) {
        deliver_event_here(c, 0x0029_7a1c);
    }
    Ok(())
}

/// `node->vtable[slot](node + delta, ...)` with the node (`obj->node`) in
/// `node_reg`; returns to `ret`.
pub(crate) fn start_object_loop_node_call(
    c: &mut Cpu,
    node_reg: usize,
    slot: u32,
    ret: u32,
) -> Flow {
    c.g[node_reg][0] = Addr(r32(c, S1)).off(SOL_OBJ_NODE).lw(c);
    c.g[V1][0] = Addr(r32(c, node_reg)).off(0xc).lw(c);
    c.g[A0][0] = Addr(r32(c, V1)).off(slot).lh(c);
    c.g[V0][0] = Addr(r32(c, V1)).off(slot + 4).lw(c);
    let target = r32(c, V0);
    c.g[A0][0] = sx(r32(c, node_reg).wrapping_add(r32(c, A0)));
    call_ptr(c, target, ret)
}

/// Fill the slot (s0 = sys + 0x30 * s4): the object, and the position
/// (also kept at sp+0): the node's translation for bone -1, else the
/// bone's translation, or the object's position when the node has no such
/// bone.
pub(crate) fn start_object_loop_position(c: &mut Cpu) -> Flow {
    c.cycles += 5;
    c.g[V1][0] = u64::MAX;
    c.mult(0, A0, S4, V0);
    c.g[S0][0] = sx(r32(c, A0).wrapping_add(r32(c, S3)));
    let whole_object = c.g[S5][0] == c.g[V1][0];
    Addr(r32(c, S0))
        .off(SOL_SLOTS + SOL_OBJECT)
        .sw(c, c.g[S1][0]);
    let reg = if whole_object {
        c.cycles += 8;
        start_object_loop_node_call(c, A1, SOL_VT_TRANSFORM, 0x0029_7abc)?;
        c.lq(V1, r32(c, V0).wrapping_add(0x30));
        V1
    } else {
        c.cycles += 8;
        start_object_loop_node_call(c, A1, SOL_VT_HAS_BONE, 0x0029_7adc)?;
        let has_bone = c.g[V0][0] != 0;
        c.g[A1][0] = c.g[S5][0];
        if has_bone {
            c.cycles += 7;
            start_object_loop_node_call(c, A2, SOL_VT_BONE_TRANSFORM, 0x0029_7afc)?;
            c.lq(V1, r32(c, V0).wrapping_add(0x30));
            V1
        } else {
            c.cycles += 3;
            c.lq(V0, r32(c, S1).wrapping_add(SOL_OBJ_POSITION));
            c.sq(V0, r32(c, S0).wrapping_add(SOL_SLOTS + SOL_POSITION));
            c.sq(V0, sp(c));
            return Ok(());
        }
    };
    c.cycles += 3;
    c.sq(reg, r32(c, S0).wrapping_add(SOL_SLOTS + SOL_POSITION));
    c.sq(reg, sp(c));
    Ok(())
}

/// Keep the bone in the slot; the bank (s6) and sample index (s5) from the
/// sound id: < 100 bank 2, < 150 bank 8, < 200 bank 9, else bank 4.
pub(crate) fn start_object_loop_bank(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = SOL_SLOT_SIZE as u64;
    c.g[V1][0] = ((c.g[S2][0] as i64) < 100) as u64;
    c.mult(0, A0, S4, V0);
    c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, S3)));
    Addr(r32(c, V0)).off(SOL_SLOTS + SOL_BONE).sw(c, c.g[S5][0]);
    let sound = r32(c, S2);
    if c.g[V1][0] != 0 {
        c.cycles += 3;
        c.g[S6][0] = 2;
        c.g[S5][0] = c.g[S2][0];
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = ((c.g[S2][0] as i64) < 150) as u64;
    c.g[S6][0] = 8;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[S5][0] = sx(sound.wrapping_sub(100));
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = ((c.g[S2][0] as i64) < 200) as u64;
    c.g[S6][0] = 9;
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[S5][0] = sx(sound.wrapping_sub(150));
        return;
    }
    c.cycles += 2;
    c.g[S6][0] = 4;
    c.g[S5][0] = sx(sound.wrapping_sub(200));
}

/// Keep the sound id in the slot; radius (f20) 300 and priority (s7) 80,
/// or 400 and 50 for sound 201 on a track whose record word is 8. Leaves
/// s0 = sys->monitor.
pub(crate) fn start_object_loop_radius(c: &mut Cpu) -> Flow {
    c.cycles += 9;
    c.g[V0][0] = SOL_SLOT_SIZE as u64;
    c.g[AT][0] = 0x4396_0000; // 300.0f
    c.mtc1(AT, 20);
    c.mult(0, V1, S4, V0);
    c.g[S7][0] = 0x50;
    c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, S3)));
    c.g[V1][0] = SOL_RADIUS_SOUND;
    Addr(r32(c, V0))
        .off(SOL_SLOTS + SOL_SOUND)
        .sw(c, c.g[S2][0]);
    if c.g[S2][0] == c.g[V1][0] {
        c.cycles += 11;
        call_ptr(c, SOL_GET_BE_LIBRARY, 0x0029_7b98)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, SOL_GET_INTERFACE, 0x0029_7ba4)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, SOL_TRACK_RECORD, 0x0029_7bac)?;
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        c.g[V0][0] = 8;
        if c.g[V1][0] != c.g[V0][0] {
            c.g[S0][0] = Addr(r32(c, S3)).off(AUDIO_SYS_MONITOR).lw(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[AT][0] = 0x43c8_0000; // 400.0f
        c.mtc1(AT, 20);
        c.g[S7][0] = 0x32;
    }
    c.cycles += 1;
    c.g[S0][0] = Addr(r32(c, S3)).off(AUDIO_SYS_MONITOR).lw(c);
    Ok(())
}

/// 0x297bcc: push a new entry on the monitor's queue (s0 = monitor, s1 =
/// its queue at +0x1d8): count += 1, copy the 24-byte template into the
/// entry's record, clear its fields, volume 100 / pan 90, 50 / 127 / 1,
/// bank s6 and sample s5 in the system's push tables, the channel gain
/// pointer, the slot's position, radius f20 (vanish -1), priority s7; then
/// `Audio_PlayPushedSound(sys)` and keep the handle in the slot.
pub(crate) fn start_object_loop_push_and_play(c: &mut Cpu) -> Flow {
    c.cycles += 174;
    let monitor = r32(c, S0);
    c.g[T1][0] = 0x18; // record size
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x8;
    c.g[V0][0] = Addr(monitor).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[S1][0] = sx(monitor.wrapping_add(MONITOR_PUSH_QUEUE));
    let queue = r32(c, S1);
    c.g[S2][0] = SOL_SLOT_SIZE as u64;
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
    Addr(monitor).off(MONITOR_PUSH_COUNT).sw(c, c.g[V0][0]);
    c.g[V1][0] = Addr(monitor).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[V0][0] = Addr(queue).off(0x20).lw(c);
    c.mult(0, V1, V1, T1);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    c.ldl(A0, monitor.wrapping_add(0x1e3));
    c.ldr(A0, monitor.wrapping_add(0x1dc));
    c.ldl(A3, monitor.wrapping_add(0x1eb));
    c.ldr(A3, monitor.wrapping_add(0x1e4));
    c.ldl(T0, monitor.wrapping_add(0x1f3));
    c.ldr(T0, monitor.wrapping_add(0x1ec));
    let record = r32(c, V1);
    c.sdl(A0, record.wrapping_add(0x7));
    c.sdr(A0, record);
    c.sdl(A3, record.wrapping_add(0xf));
    c.sdr(A3, record.wrapping_add(0x8));
    c.sdl(T0, record.wrapping_add(0x17));
    c.sdr(T0, record.wrapping_add(0x10));
    // monitor->positions[count] = 0 (set below).
    c.g[V0][0] = Addr(monitor).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = sx(monitor.wrapping_add(r32(c, V0)));
    Addr(r32(c, V0)).off(SOL_MONITOR_POSITIONS).sw(c, 0);
    boost_empty_store_entry(c, (V1, V0), 0x34, 2, 0, 0);
    // table 0x38 [count] = the first byte of the entry's record.
    c.g[A3][0] = Addr(monitor).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[A0][0] = Addr(monitor).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[T0][0] = Addr(queue).off(0x20).lw(c);
    c.g[A3][0] = sx(r32(c, A3) << 2);
    c.mult(0, A0, A0, T1);
    c.g[V0][0] = Addr(queue).off(0x38).lw(c);
    c.g[A3][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, T0)));
    c.g[V1][0] = Addr(r32(c, A0)).lb(c);
    Addr(r32(c, A3)).sw(c, c.g[V1][0]);
    boost_empty_store_entry(c, (V0, V1), 0x3c, 2, 0, 0);
    // memset(&queue->slots[count] (8 bytes at +0x40), 0, 8)
    c.g[A0][0] = Addr(monitor).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[A0][0] = sx(r32(c, A0) << 3);
    c.g[A0][0] = sx(queue.wrapping_add(r32(c, A0)));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x40));
    call_ptr(c, SOL_MEMSET, 0x0029_7ca0)?;

    // The entry's parameters.
    c.g[A3][0] = 0x64; // volume
    c.g[T0][0] = 0x5a;
    c.g[T1][0] = 0x32;
    boost_empty_store_entry(c, (V1, V0), 0x60, 2, 0, 0);
    c.g[T2][0] = 0x7f;
    c.g[T3][0] = 0x1;
    c.g[A1][0] = 0x5; // channel kind of GetChannelGainPtr
    c.g[A2][0] = 0;
    c.g[A0][0] = c.g[S3][0];
    boost_empty_store_entry(c, (V1, V0), 0x64, 2, 0, 0);
    boost_empty_store_entry(c, (V0, V1), 0x68, 2, 0, 0);
    boost_empty_store_entry(c, (V1, V0), 0x6c, 2, 0, 0);
    boost_empty_store_entry(c, (V0, V1), 0x70, 4, 0x4, 0);
    let (a3, t0, t1, t2, t3) = (c.g[A3][0], c.g[T0][0], c.g[T1][0], c.g[T2][0], c.g[T3][0]);
    boost_empty_store_entry(c, (V1, V0), 0x70, 4, 0x0, a3);
    boost_empty_store_entry(c, (V0, V1), 0x70, 4, 0x8, t0);
    boost_empty_store_entry(c, (V1, V0), 0x70, 4, 0xc, t1);
    boost_empty_store_entry(c, (V0, V1), 0x74, 2, 0, 0);
    boost_empty_store_entry(c, (V1, V0), 0x80, 2, 0, 0);
    boost_empty_store_entry(c, (V0, V1), 0x84, 2, 0, 0);
    boost_empty_store_entry(c, (V1, V0), 0x78, 2, 0, t2);
    boost_empty_store_entry(c, (V0, V1), 0x7c, 2, 0, t3);

    // sys->pushBank[count] = bank (s6); sys->pushSample[count] = sample (s5).
    for (field, reg) in [(AUDIO_SYS_PUSH_BANK, S6), (AUDIO_SYS_PUSH_SAMPLE, S5)] {
        let sys = r32(c, S3);
        c.g[V1][0] = Addr(sys).off(AUDIO_SYS_MONITOR).lw(c);
        c.g[A3][0] = Addr(sys).off(field).lw(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(MONITOR_PUSH_COUNT).lw(c);
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A3)));
        Addr(r32(c, V0)).sw(c, c.g[reg][0]);
    }

    // gains[count] = SSXAudioSystem_GetChannelGainPtr(sys, 5, 0);
    // positions[count] = &slot->position.
    c.g[S0][0] = Addr(r32(c, S3)).off(AUDIO_SYS_SND_ROOT).lw(c);
    call_ptr(c, SOL_GET_CHANNEL_GAIN, 0x0029_7dfc)?;
    c.mult(0, V1, S4, S2);
    c.g[A0][0] = Addr(r32(c, S0)).lw(c);
    c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0f
    c.mtc1(AT, 13);
    c.mov_s(12, 20);
    let root_monitor = r32(c, A0);
    c.g[A1][0] = Addr(root_monitor).off(SOL_MONITOR_GAINS).lw(c);
    c.g[A2][0] = sx(r32(c, V1).wrapping_add(r32(c, S3)));
    c.g[V1][0] = Addr(root_monitor).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[A2][0] = sx(r32(c, A2).wrapping_add(SOL_SLOTS + SOL_POSITION));
    c.g[V1][0] = sx(r32(c, V1) << 2);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
    Addr(r32(c, V1)).sw(c, c.g[V0][0]);
    c.g[A0][0] = Addr(r32(c, S3)).off(AUDIO_SYS_SND_ROOT).lw(c);
    c.g[V1][0] = Addr(r32(c, A0)).lw(c);
    c.g[V0][0] = Addr(r32(c, V1)).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    Addr(r32(c, V1))
        .off(SOL_MONITOR_POSITIONS)
        .sw(c, c.g[A2][0]);

    // SndPlayParams_SetRadiusVanish(&monitor->queue, radius, -1.0f)
    c.g[V0][0] = Addr(r32(c, S3)).off(AUDIO_SYS_SND_ROOT).lw(c);
    c.g[A0][0] = Addr(r32(c, V0)).lw(c);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(MONITOR_PUSH_QUEUE));
    SOL_SET_RADIUS_VANISH.call(c, 0x0029_7e54)?;

    // priorities[count] = s7; handle = Audio_PlayPushedSound(sys).
    c.g[A1][0] = Addr(r32(c, S3)).off(AUDIO_SYS_SND_ROOT).lw(c);
    c.g[A0][0] = c.g[S3][0];
    c.g[V1][0] = Addr(r32(c, A1)).lw(c);
    c.g[V0][0] = Addr(r32(c, V1)).off(MONITOR_PUSH_COUNT).lw(c);
    c.g[A1][0] = Addr(r32(c, V1)).off(SOL_MONITOR_PRIORITIES).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
    Addr(r32(c, V0)).sw(c, c.g[S7][0]);
    call_ptr(c, SOL_PLAY_PUSHED, 0x0029_7e78)?;
    c.mult(0, V1, S4, S2);
    c.g[S2][0] = sx(r32(c, V1).wrapping_add(r32(c, S3)));
    Addr(r32(c, S2))
        .off(SOL_SLOTS + SOL_HANDLE)
        .sw(c, c.g[V0][0]);
    Ok(())
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_StartWipeoutLoop2 (0x296e80).

/// `rider->audio` (rider+0x80): the rider's audio state.
pub(crate) const WL2_RIDER_AUDIO: u32 = 0x80;

/// Rider audio: embedded object (vtable at +0) asked whether the rider is
/// audible (slot 0x48) and for its position (slot 0x28).
pub(crate) const WL2_AUDIO_OBJ: u32 = 0x6c0;

pub(crate) const WL2_SLOT_IS_LOCAL: u32 = 0x48;

pub(crate) const WL2_SLOT_GET_POS: u32 = 0x28;

/// Rider audio: wipeout intensity (float) fed to the volume curve.
pub(crate) const WL2_INTENSITY: u32 = 0x770;

/// Rider audio: the wipeout-loop-2 voice handle (< 0: not playing).
pub(crate) const WL2_HANDLE: u32 = 0x778;

/// Rider audio: listener index, "has a listener", "listener valid".
pub(crate) const WL2_LISTENER_INDEX: u32 = 0x870;

pub(crate) const WL2_HAS_LISTENER: u32 = 0x874;

pub(crate) const WL2_LISTENER_VALID: u32 = 0x87c;

/// Rider audio: the sound source position (record source pointer).
pub(crate) const WL2_SOURCE_POS: u32 = 0x110;

/// SSXAudioSystem: the channel holder whose first word is the bank monitor
/// the voice record is completed in.
pub(crate) const WL2_CHANNEL: u32 = 0x118;

/// Bank monitor: record write cursor, record descriptor, parameter block,
/// per-record source pointers.
pub(crate) const WL2_MON_CURSOR: u32 = 0x1f4;

pub(crate) const WL2_MON_DESC: u32 = 0x1d8;

pub(crate) const WL2_MON_PARAMS: u32 = 0x1dc;

pub(crate) const WL2_MON_SOURCE: u32 = 0x1fc;

/// Rodata: the five-point volume curve, the 8-byte record default, and the
/// speech mode byte (2: replay check needed).
pub(crate) const WL2_VOLUME_CURVE: u32 = 0x0048_2da0;

pub(crate) const WL2_RECORD_DEFAULT: u32 = 0x004a_36b8;

pub(crate) const WL2_SPEECH_MODE: u32 = 0x0053_5c11;

/// Callees (all reached through the function table).
pub(crate) const WL2_EVAL_CURVE5: u32 = 0x0029_0b58;

pub(crate) const WL2_GET_BE: u32 = 0x0014_dc80;

pub(crate) const WL2_GET_INTERFACE: u32 = 0x0014_dd58;

pub(crate) const WL2_IS_REPLAY_PLAYING: u32 = 0x0028_8ae0;

pub(crate) const WL2_FIND_VIEW_IN_RANGE: u32 = 0x0028_8b40;

pub(crate) const WL2_MEMSET: u32 = 0x0041_6210;

pub(crate) const WL2_GET_CHANNEL_GAIN_PTR: u32 = 0x0028_7968;

pub(crate) const WL2_GET_RIDER_LISTENER_INDEX: u32 = 0x0028_8a20;

pub(crate) const WL2_PLAY_PUSHED_SOUND: u32 = 0x0029_06b8;

/// `reg + off` as an address.
#[inline(always)]
pub(crate) fn wl2_ea(c: &Cpu, r: usize, off: u32) -> u32 {
    (c.g[r][0] as u32).wrapping_add(off)
}

/// `lw rd, off(rb)`.
#[inline(always)]
pub(crate) fn wl2_lw(c: &mut Cpu, rd: usize, rb: usize, off: u32) {
    let a = wl2_ea(c, rb, off);
    c.g[rd][0] = c.lw(a);
}

/// `sll rd, rd, shift; addu rd, rd, rb` (an array element address).
#[inline(always)]
pub(crate) fn wl2_index(c: &mut Cpu, rd: usize, shift: u32, rb: usize) {
    c.g[rd][0] = sx(((c.g[rd][0] as u32) << shift).wrapping_add(c.g[rb][0] as u32));
}

/// Store `val` into element `cursor` of the monitor's (s0) record array at
/// descriptor (s1) field `field` (element size `1 << shift`, at `+off`),
/// with the original's register pair `ri`/`rb`.
pub(crate) fn wl2_record_store(
    c: &mut Cpu,
    ri: usize,
    rb: usize,
    field: u32,
    shift: u32,
    off: u32,
    val: u64,
) {
    wl2_lw(c, ri, S0, WL2_MON_CURSOR);
    wl2_lw(c, rb, S1, field);
    wl2_index(c, ri, shift, rb);
    let a = wl2_ea(c, ri, off);
    c.sw(a, val);
}

/// Unaligned doubleword copy through `regs` (`ldl/ldr` all, then
/// `sdl/sdr` all).
pub(crate) fn wl2_copy_dwords(c: &mut Cpu, regs: &[usize], src: u32, dst: u32) {
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

/// `void SSXAudioSystem_StartWipeoutLoop2(SSXAudioSystem *audio, Rider
/// *rider)` (0x296e80): start the rider's second wipeout loop unless it is
/// already playing. A rider that is not the local one (virtual slot 0x48)
/// and has no valid listener must be within 100 units of a view. The volume
/// comes from the five-point curve over the wipeout intensity (clamped to
/// 0..127); a voice record is pushed on the audio system's bank monitor
/// (gain, pan, source, listener, replay flag) and played, and the handle is
/// stored in the rider's audio state.
pub fn ssx_audio_system_start_wipeout_loop2(c: &mut Cpu) {
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
        wl2_lw(c, A1, S5, WL2_RIDER_AUDIO);
        wl2_lw(c, V0, A1, WL2_HANDLE);
        c.g[S3][0] = c.g[A0][0];
        if (c.g[V0][0] as i64) >= 0 {
            // Already playing.
            c.cycles += 1;
            frame.restore(c, S0, 0xb0);
        } else if wl2_should_start(c, frame)? {
            wl2_start(c)?;
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
pub(crate) fn wl2_should_start(c: &mut Cpu, frame: Frame) -> Result<bool, crate::call::Unwound> {
    c.cycles += 8;
    wl2_lw(c, V1, A1, WL2_AUDIO_OBJ);
    c.g[A1][0] = sx(wl2_ea(c, A1, WL2_AUDIO_OBJ));
    c.g[A0][0] = c.lh(wl2_ea(c, V1, WL2_SLOT_IS_LOCAL));
    wl2_lw(c, V0, V1, WL2_SLOT_IS_LOCAL + 4);
    let method = c.g[V0][0] as u32;
    c.g[A0][0] = sx(wl2_ea(c, A1, c.g[A0][0] as u32));
    call_ptr(c, method, 0x0029_6ed0)?;
    let check_views = if c.g[V0][0] != 0 {
        wl2_lw(c, A0, S5, WL2_RIDER_AUDIO);
        true
    } else {
        c.cycles += 4;
        wl2_lw(c, V0, S5, WL2_RIDER_AUDIO);
        wl2_lw(c, V1, V0, WL2_LISTENER_VALID);
        if c.g[V1][0] != 0 {
            wl2_lw(c, V1, S5, WL2_RIDER_AUDIO);
            false
        } else {
            c.cycles += 1;
            wl2_lw(c, A0, S5, WL2_RIDER_AUDIO);
            true
        }
    };
    if !check_views {
        return Ok(true);
    }
    c.cycles += 13;
    wl2_lw(c, V1, A0, WL2_AUDIO_OBJ);
    c.g[A0][0] = sx(wl2_ea(c, A0, WL2_AUDIO_OBJ));
    c.g[A1][0] = c.lh(wl2_ea(c, V1, WL2_SLOT_GET_POS));
    wl2_lw(c, V0, V1, WL2_SLOT_GET_POS + 4);
    let method = c.g[V0][0] as u32;
    c.g[A0][0] = sx(wl2_ea(c, A0, c.g[A1][0] as u32));
    call_ptr(c, method, 0x0029_6f04)?;
    // FindViewInRange(audio, pos, 100.0f)
    c.g[AT][0] = 0x42c8_0000;
    c.mtc1(AT, 12);
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, WL2_FIND_VIEW_IN_RANGE, 0x0029_6f18)?;
    let none = (c.g[V0][0] as i64) < 0;
    frame.restore(c, S0, 0xb0);
    if none {
        return Ok(false);
    }
    c.cycles += 1;
    wl2_lw(c, V1, S5, WL2_RIDER_AUDIO);
    Ok(true)
}

/// Start the loop: volume from the curve, the replay flag, the voice
/// record, play it and keep the handle.
pub(crate) fn wl2_start(c: &mut Cpu) -> crate::call::Flow {
    // Volume = EvalCurve5(curve copied to the stack, intensity).
    c.cycles += 38;
    c.g[V0][0] = 0x48_0000;
    c.g[A3][0] = sx(WL2_VOLUME_CURVE);
    let curve = sp(c);
    wl2_copy_dwords(c, &[A0, A1, A2], WL2_VOLUME_CURVE, curve);
    wl2_copy_dwords(c, &[A0, A1], WL2_VOLUME_CURVE + 0x18, curve + 0x18);
    c.g[S6][0] = 0;
    c.g[A0][0] = c.g[SP][0];
    let intensity = wl2_ea(c, V1, WL2_INTENSITY);
    c.lwc1(12, intensity);
    call_ptr(c, WL2_EVAL_CURVE5, 0x0029_6f8c)?;
    c.cvt_w_s(1, 0);
    c.mfc1(S4, 1);
    wl2_lw(c, A2, S5, WL2_RIDER_AUDIO);
    c.g[A0][0] = 0x7f;
    c.g[V1][0] = u64::MAX;
    wl2_lw(c, A1, A2, WL2_HAS_LISTENER);
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
        wl2_lw(c, V0, A2, WL2_LISTENER_VALID);
        c.g[A3][0] = (c.g[V0][0] != 0) as u64;
    }
    wl2_replay_flag(c)?;
    wl2_push_record(c)?;
    wl2_finish_record(c)?;
    // handle = PlayPushedSound(audio)
    c.cycles += 4;
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, WL2_PLAY_PUSHED_SOUND, 0x0029_73fc)?;
    wl2_lw(c, V1, S5, WL2_RIDER_AUDIO);
    let a = wl2_ea(c, V1, WL2_HANDLE);
    c.sw(a, c.g[V0][0]);
    Ok(())
}

/// s6 = "a replay is playing" for a rider with a valid listener in speech
/// mode 2 (the BE library is fetched first); then s0 = the bank monitor.
pub(crate) fn wl2_replay_flag(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 2;
    if c.g[A3][0] != 0 {
        c.cycles += 10;
        call_ptr(c, WL2_GET_BE, 0x0029_6fd4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, WL2_GET_INTERFACE, 0x0029_6fe0)?;
        c.g[V1][0] = 0x53_0000;
        c.g[A0][0] = 2;
        c.g[V0][0] = c.lb(WL2_SPEECH_MODE);
        if c.g[V0][0] == c.g[A0][0] {
            c.cycles += 4;
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, WL2_IS_REPLAY_PLAYING, 0x0029_6ffc)?;
            c.g[S6][0] = (c.g[V0][0] < 1) as u64;
        }
    }
    wl2_lw(c, S0, S3, 0);
    Ok(())
}

/// Push a voice record on the bank monitor in s0 (advance the cursor, copy
/// the parameter block, clear/default the per-record fields), then write
/// the gain (s4) into the channel monitor and its pan byte (clamped).
pub(crate) fn wl2_push_record(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 154;
    c.g[S2][0] = 0x18;
    c.g[A1][0] = 0;
    c.g[A2][0] = 8;
    // ++cursor
    wl2_lw(c, V0, S0, WL2_MON_CURSOR);
    c.g[S1][0] = sx(wl2_ea(c, S0, WL2_MON_DESC));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    let a = wl2_ea(c, S0, WL2_MON_CURSOR);
    c.sw(a, c.g[V0][0]);
    // records[cursor] (0x18 bytes) = params
    wl2_lw(c, V1, S0, WL2_MON_CURSOR);
    wl2_lw(c, V0, S1, 0x20);
    c.mult(0, V1, V1, S2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let (params, rec) = (wl2_ea(c, S0, WL2_MON_PARAMS), c.g[V1][0] as u32);
    wl2_copy_dwords(c, &[V0, A0, A3], params, rec);
    // source[cursor] = 0
    wl2_lw(c, V0, S0, WL2_MON_CURSOR);
    wl2_index(c, V0, 2, S0);
    let a = wl2_ea(c, V0, WL2_MON_SOURCE);
    c.sw(a, 0);
    wl2_record_store(c, V1, V0, 0x34, 2, 0, 0);
    // field38[cursor] = (s8) records[cursor].byte0
    wl2_lw(c, A3, S0, WL2_MON_CURSOR);
    wl2_lw(c, A0, S0, WL2_MON_CURSOR);
    wl2_lw(c, T0, S1, 0x20);
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
    c.mult(0, A0, A0, S2);
    wl2_lw(c, V0, S1, 0x38);
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
    c.g[V1][0] = c.lb(c.g[A0][0] as u32);
    c.sw(c.g[A3][0] as u32, c.g[V1][0]);
    wl2_record_store(c, V0, V1, 0x3c, 2, 0, 0);
    // memset(&desc->field40[cursor], 0, 8)
    wl2_lw(c, A0, S0, WL2_MON_CURSOR);
    wl2_index(c, A0, 3, S1);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    call_ptr(c, WL2_MEMSET, 0x0029_70d4)?;
    // Per-record defaults.
    wl2_lw(c, V1, S0, WL2_MON_CURSOR);
    c.g[A0][0] = 0x64;
    wl2_lw(c, V0, S1, 0x60);
    c.g[A1][0] = 0x5a;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[A2][0] = 0x32;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A3][0] = 0x7f;
    c.sw(c.g[V1][0] as u32, 0);
    c.g[T0][0] = 1;
    wl2_record_store(c, V0, V1, 0x64, 2, 0, 0);
    wl2_record_store(c, V1, V0, 0x68, 2, 0, 0);
    wl2_record_store(c, V0, V1, 0x6c, 2, 0, 0);
    wl2_record_store(c, V1, V0, 0x70, 4, 4, 0);
    let (a0, a1, a2, a3, t0) = (c.g[A0][0], c.g[A1][0], c.g[A2][0], c.g[A3][0], c.g[T0][0]);
    wl2_record_store(c, V0, V1, 0x70, 4, 0, a0);
    wl2_record_store(c, V1, V0, 0x70, 4, 8, a1);
    wl2_record_store(c, V0, V1, 0x70, 4, 0xc, a2);
    wl2_record_store(c, V1, V0, 0x74, 2, 0, 0);
    wl2_record_store(c, V0, V1, 0x80, 2, 0, 0);
    wl2_record_store(c, V1, V0, 0x84, 2, 0, 0);
    wl2_record_store(c, V0, V1, 0x78, 2, 0, a3);
    wl2_record_store(c, V1, V0, 0x7c, 2, 0, t0);
    // audio->array4[cursor] = 0; audio->array8[cursor] = 0x34
    wl2_lw(c, A0, S3, 0);
    wl2_lw(c, V1, S3, 4);
    wl2_lw(c, V0, A0, WL2_MON_CURSOR);
    wl2_index(c, V0, 2, V1);
    c.sw(c.g[V0][0] as u32, 0);
    wl2_lw(c, V1, S3, 0);
    wl2_lw(c, A0, S3, 8);
    wl2_lw(c, V0, V1, WL2_MON_CURSOR);
    c.g[V1][0] = 0x34;
    wl2_index(c, V0, 2, A0);
    c.sw(c.g[V0][0] as u32, c.g[V1][0]);
    // Channel monitor: gain[cursor] = volume.
    wl2_lw(c, A0, S3, WL2_CHANNEL);
    wl2_lw(c, A1, A0, 0);
    wl2_lw(c, V0, A1, WL2_MON_CURSOR);
    c.g[A2][0] = sx(wl2_ea(c, A1, WL2_MON_DESC));
    wl2_lw(c, V1, A2, 0x38);
    wl2_index(c, V0, 2, V1);
    c.sw(c.g[V0][0] as u32, c.g[S4][0]);
    // Pan byte of the channel record = clamp(gain[desc.cursor], 0, 127).
    wl2_lw(c, A0, A1, WL2_MON_CURSOR);
    wl2_lw(c, V1, A2, 0x1c);
    wl2_lw(c, A3, A2, 0x38);
    c.mult(0, A0, A0, S2);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    wl2_lw(c, A1, A2, 0x20);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A3][0] as u32));
    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
    c.g[V0][0] = ((c.g[V0][0] as i64) < 127) as u64;
    let at_most = c.g[V0][0] == 0;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    let mut negative = false;
    if !at_most {
        c.cycles += 6;
        wl2_lw(c, V0, A2, 0x1c);
        wl2_index(c, V0, 2, A3);
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        negative = (c.g[V1][0] as i64) < 0;
        c.g[V0][0] = 0;
    }
    if !negative {
        c.cycles += 7;
        wl2_lw(c, V0, A2, 0x1c);
        wl2_index(c, V0, 2, A3);
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        c.g[V1][0] = ((c.g[V1][0] as i64) < 127) as u64;
        if c.g[V1][0] == 0 {
            c.g[V0][0] = 0x7f;
        } else {
            c.cycles += 4;
            wl2_lw(c, V0, A2, 0x1c);
            wl2_index(c, V0, 2, A3);
            c.g[V0][0] = c.lbu(c.g[V0][0] as u32);
        }
    }
    Ok(())
}

/// Store the pan byte, then fill the channel monitor's record: gain slot
/// pointer, source position, listener index, rider, default, and the
/// replay flag with the rider's listener index.
pub(crate) fn wl2_finish_record(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 23;
    c.sb(c.g[A0][0] as u32, c.g[V0][0]);
    // gainPtr[cursor] = GetChannelGainPtr(audio, 5, 0)
    c.g[A1][0] = 5;
    c.g[A0][0] = c.g[S3][0];
    c.g[A2][0] = 0;
    wl2_lw(c, S0, S3, WL2_CHANNEL);
    call_ptr(c, WL2_GET_CHANNEL_GAIN_PTR, 0x0029_72c8)?;
    wl2_lw(c, A0, S0, 0);
    c.g[A2][0] = 0;
    wl2_lw(c, V1, A0, WL2_MON_CURSOR);
    wl2_lw(c, A1, A0, 0x214);
    wl2_index(c, V1, 2, A1);
    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
    // source[cursor] = &rider->audio->sourcePos
    wl2_lw(c, V0, S3, WL2_CHANNEL);
    wl2_lw(c, A0, S5, WL2_RIDER_AUDIO);
    wl2_lw(c, V1, V0, 0);
    c.g[A0][0] = sx(wl2_ea(c, A0, WL2_SOURCE_POS));
    wl2_lw(c, V0, V1, WL2_MON_CURSOR);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let a = wl2_ea(c, V1, WL2_MON_SOURCE);
    c.sw(a, c.g[A0][0]);
    let no_rider = c.g[S5][0] == 0;
    wl2_lw(c, S0, S3, WL2_CHANNEL);
    if !no_rider {
        c.cycles += 1;
        wl2_lw(c, A2, S5, WL2_RIDER_AUDIO);
    }
    // listener[cursor] = GetRiderListenerIndex(audio, rider->audio)
    c.cycles += 40;
    c.g[A1][0] = c.g[A2][0];
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, WL2_GET_RIDER_LISTENER_INDEX, 0x0029_731c)?;
    wl2_lw(c, A0, S0, 0);
    c.g[A2][0] = 0x4a_0000;
    wl2_lw(c, V1, A0, WL2_MON_CURSOR);
    wl2_lw(c, A1, A0, 0x23c);
    wl2_index(c, V1, 2, A1);
    c.sw(c.g[V1][0] as u32, c.g[V0][0]);
    // rider[cursor] = rider
    wl2_lw(c, A0, S3, WL2_CHANNEL);
    wl2_lw(c, V1, A0, 0);
    wl2_lw(c, V0, V1, WL2_MON_CURSOR);
    wl2_lw(c, A0, V1, 0x238);
    wl2_index(c, V0, 2, A0);
    c.sw(c.g[V0][0] as u32, c.g[S5][0]);
    // desc.field40[cursor] (8 bytes) = default (through the stack)
    wl2_lw(c, V1, S3, WL2_CHANNEL);
    wl2_lw(c, A0, V1, 0);
    c.g[A3][0] = sx(wl2_ea(c, A2, 0x36b8));
    let tmp = sp(c).wrapping_add(0x30);
    wl2_copy_dwords(c, &[V0], WL2_RECORD_DEFAULT, tmp);
    c.g[V1][0] = sx(wl2_ea(c, A0, WL2_MON_DESC));
    wl2_lw(c, V0, A0, WL2_MON_CURSOR);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let dst = wl2_ea(c, V1, 0x40);
    wl2_copy_dwords(c, &[T0], tmp, dst);
    // replay[cursor] = 0
    wl2_lw(c, A0, S3, WL2_CHANNEL);
    wl2_lw(c, V1, A0, 0);
    wl2_lw(c, V0, V1, WL2_MON_CURSOR);
    wl2_lw(c, A0, V1, 0x254);
    wl2_index(c, V0, 2, A0);
    let replaying = c.g[S6][0] != 0;
    c.sw(c.g[V0][0] as u32, 0);
    if replaying {
        // replay[cursor] = 1; replayListener[cursor] = rider->audio->listenerIndex
        c.cycles += 17;
        wl2_lw(c, A0, S3, WL2_CHANNEL);
        c.g[A1][0] = 1;
        wl2_lw(c, V1, A0, 0);
        wl2_lw(c, V0, V1, WL2_MON_CURSOR);
        wl2_lw(c, A0, V1, 0x258);
        wl2_index(c, V0, 2, A0);
        c.sw(c.g[V0][0] as u32, c.g[A1][0]);
        wl2_lw(c, V1, S3, WL2_CHANNEL);
        wl2_lw(c, A0, S5, WL2_RIDER_AUDIO);
        wl2_lw(c, A1, V1, 0);
        wl2_lw(c, A2, A0, WL2_LISTENER_INDEX);
        wl2_lw(c, V0, A1, WL2_MON_CURSOR);
        wl2_lw(c, V1, A1, 0x25c);
        wl2_index(c, V0, 2, V1);
        c.sw(c.g[V0][0] as u32, c.g[A2][0]);
    }
    Ok(())
}

/// `bool SSXAudioSystem::IsSpeechVolumeOn()` (0x29f0a0): whether the speech
/// channel's volume (`GetChannelVolume(4)`) is not zero.
pub fn ssx_audio_system_is_speech_volume_on(c: &mut Cpu) {
    const GET_CHANNEL_VOLUME: u32 = 0x0028_7920;
    const SPEECH_CHANNEL: u64 = 4;
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = SPEECH_CHANNEL;
        call_ptr(c, GET_CHANNEL_VOLUME, 0x0029_f0b0)?;
        c.f[1] = 0;
        let silent = FpuF32(c.f[0]).eq_s(FpuF32::ZERO);
        abi::set_fcc(c, silent);
        c.g[V0][0] = 0;
        if !silent {
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) const RS207D_IS_SPEECH_VOLUME_ON: Callee = Callee {
    addr: 0x0029_f0a0,
    f: ee::s08::SSXAudioSystem_IsSpeechVolumeOn,
};

pub(crate) const RS207D_IS_INSTANCE_READY: Callee = Callee {
    addr: 0x002b_0e28,
    f: ee::s08::SpeechSystem_IsInstanceReady,
};

pub(crate) const RS207D_GET_RIDER_SPEECH_MASK: Callee = Callee {
    addr: 0x0029_f330,
    f: ee::s08::SSXAudioSystem_GetRiderSpeechMask,
};

pub(crate) const RS207D_GET_RIDER_VOICE_MASK: Callee = Callee {
    addr: 0x002a_1d48,
    f: ee::s08::SSXAudioSystem_GetRiderVoiceMask,
};

pub(crate) const RS207D_GET_RIDER_RIVALRY: Callee = Callee {
    addr: 0x002a_1820,
    f: ee::s08::SSXAudioSystem_GetRiderRivalry,
};

pub(crate) const RS207D_SET_EVENT_CONTEXT: Callee = Callee {
    addr: 0x002b_1458,
    f: ee::s08::SpeechSystem_SetEventContext,
};

/// `jalr` to the scripting-object method at `slot` of the rider whose
/// object base (`rider + 0x6c0`) is `obj`, vtable `vt`: `a0 = obj + delta`.
pub(crate) fn rs207d_obj_call(
    c: &mut Cpu,
    obj: u64,
    vt: u64,
    slot: u32,
    ret_to: u32,
) -> crate::call::Flow {
    c.g[A0][0] = Addr((vt as u32).wrapping_add(slot)).lh(c);
    c.g[V0][0] = Addr((vt as u32).wrapping_add(slot + 4)).lw(c);
    let f = r32(c, V0);
    c.g[A0][0] = sx((obj as u32).wrapping_add(r32(c, A0)));
    call_ptr(c, f, ret_to)
}

/// `dot(sp[0..4], *v)` in vf4.x (VU0 macro ops, vf6.x = 1.0).
pub(crate) fn rs207d_dot(c: &mut Cpu, v: u32) {
    use ssx3_ee_rt::vu0::{Src, W, Y, Z};
    c.lqc2(3, sp(c));
    c.lqc2(5, v);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
}

/// Post one speech event: register its context (`SetEventContext(speech,
/// 0, code, rider, Rider_GetCharacter(rider), 0, f20, 0)`, the rider in
/// s1/s0 as `rider_reg`) and, when accepted, call the post hook
/// (`gp+0x173c`) with `SPCH_MakeId(1, 0, code)` and the arguments
/// `hook_args` sets (`before_id` runs before `SPCH_MakeId`). Returns
/// whether the context was accepted.
pub(crate) fn rs207d_post(
    c: &mut Cpu,
    rider_reg: usize,
    code: u64,
    rets: [u32; 4],
    accept_cycles: u64,
    before_id: impl FnOnce(&mut Cpu),
    hook_args: impl FnOnce(&mut Cpu),
) -> Result<bool, crate::call::Unwound> {
    c.g[A0][0] = c.g[rider_reg][0];
    call_ptr(c, 0x0012_3128, rets[0])?; // Rider_GetCharacter
    c.g[A3][0] = c.g[rider_reg][0];
    c.g[T0][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = 0;
    c.g[A2][0] = code;
    c.mov_s(12, 20);
    c.g[T1][0] = 0;
    c.g[T2][0] = 0;
    RS207D_SET_EVENT_CONTEXT.call(c, rets[1])?;
    let accepted = c.g[V0][0] != 0;
    c.g[A0][0] = 1;
    if !accepted {
        return Ok(false);
    }
    c.cycles += accept_cycles;
    c.g[A1][0] = 0;
    c.g[A2][0] = code;
    before_id(c);
    call_ptr(c, 0x003d_8008, rets[2])?; // SPCH_MakeId(1, 0, code)
    c.g[V1][0] = Addr(r32(c, GP).wrapping_add(rs207d::POST_HOOK_GP)).lw(c);
    c.g[A0][0] = c.g[V0][0];
    hook_args(c);
    let hook = r32(c, V1);
    call_ptr(c, hook, rets[3])?;
    Ok(true)
}

/// `void SSXAudioSystem_RiderSay207D(SSXAudioSystem* sys, Rider* a, Rider*
/// b)` (0x2a0560): the "passed you" speech when rider `a` overtakes `b`.
/// Only with speech on (mode 2), outside replays, both riders' character
/// info clear, at least 10 s (600 frames at 60 Hz, `tick`-scaled) since
/// the race-start stamp (`sys+0x582c`), both still behind the start line
/// (progress < 0), one of them the followed rider unless both are
/// human/local, the speech system ready, both heard by the same
/// listener view, and their gap along `b`'s heading between the near and
/// far limits. Then, by rivalry (2: rival lines 0x2081/0x207f; 0/1: pass
/// lines 0x2080/0x207d), posts a line for each rider with its speech mask
/// and the gap class (1, or 2 beyond the "far" limit).
pub fn ssx_audio_system_rider_say_207d(c: &mut Cpu) {
    let frame = Frame { size: 0xc0 };
    guest(c, |c| {
        c.cycles += 17;
        Frame::open(c, 0xc0);
        frame.save(c, S3, 0x70);
        frame.save(c, S0, 0xa0);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x90);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x80);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        frame.save(c, S6, 0x40);
        frame.save(c, S7, 0x30);
        frame.save_ra(c, 0x20);
        frame.save_fpr(c, 20, 0xb0);
        RS207D_IS_SPEECH_VOLUME_ON.call(c, 0x002a_059c)?;
        // false: the plain exit; true: the exit through 0x2a09fc (+1 cycle).
        let late_exit = rs207d_body(c)?;
        if late_exit {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0xa0);
        c.cycles += 11;
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 20, 0xb0);
        frame.close(c);
        Ok(())
    })
}

/// The body of [`ssx_audio_system_rider_say_207d`] after the volume check:
/// `Ok(true)` when it leaves through the 0x2a09fc exit block.
pub(crate) fn rs207d_body(c: &mut Cpu) -> Result<bool, crate::call::Unwound> {
    use rs207d::*;
    let gp = r32(c, GP);
    let volume_off = c.g[V0][0] == 0;
    c.g[V1][0] = Addr(gp.wrapping_sub(SPEECH_SETTINGS_GP)).lw(c);
    if volume_off {
        return Ok(true);
    }
    c.cycles += 4;
    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(SPEECH_MODE)).lw(c) ^ 2;
    if c.g[V0][0] != 0 {
        return Ok(false);
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, 0x0028_8ae0, 0x002a_05bc)?; // SSXAudioSystem_IsReplayPlaying
    if c.g[V0][0] != 0 {
        return Ok(false);
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x0012_3168, 0x002a_05cc)?; // Rider_GetPlayerCharacterInfo
    if c.g[V0][0] != 0 {
        return Ok(false);
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0012_3168, 0x002a_05dc)?;
    if c.g[V0][0] != 0 {
        return Ok(false);
    }
    c.cycles += 9;
    call_ptr(c, 0x0028_b1c8, 0x002a_05ec)?; // SSXAudioSystem_GetGameModule
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0xc)).lw(c);
    c.g[A0][0] = Addr(r32(c, S3).wrapping_add(LAST_PASS)).lw(c);
    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(RACE_TIME)).lw(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, A0)));
    // The game module's frame counter counts ticks; `+0x582c` is its value
    // when the race started (EnterGameInit / race start): no pass lines in
    // the first 600 60 Hz frames (10 s), the same wall time at any rate.
    let wait = crate::tick::ticks_from_frames60(c, 600) as i64;
    c.g[V0][0] = ((c.g[V0][0] as i64) < wait) as u64;
    if c.g[V0][0] != 0 {
        return Ok(false);
    }
    // Both riders still behind the line (0 <= progress: no speech).
    for (rider_reg, extra) in [(S1, 6u64), (S0, 5)] {
        c.cycles += extra;
        c.lwc1(0, r32(c, rider_reg).wrapping_add(PROGRESS));
        if extra == 6 {
            c.mtc1(0, 1);
        }
        c.c_le_s(1, 0);
        c.g[V0][0] = 1;
        if !c.fcc() {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            return Ok(false);
        }
    }
    // a: "AI or not local" (s4); its object base in s6.
    c.cycles += 10;
    c.g[V1][0] = Addr(r32(c, S1).wrapping_add(OBJ)).lw(c);
    c.g[A1][0] = sx(r32(c, S1).wrapping_add(OBJ));
    c.g[S6][0] = c.g[A1][0];
    c.g[S4][0] = 0;
    let (obj, vt) = (c.g[A1][0], c.g[V1][0]);
    rs207d_obj_call(c, obj, vt, VT_IS_AI, 0x002a_066c)?;
    let mut set = c.g[V0][0] != 0;
    if !set {
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(LOCAL)).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            set = true;
        }
    }
    if set {
        c.g[S4][0] = 1;
        c.cycles += 1;
    }
    c.g[V1][0] = Addr(r32(c, S0).wrapping_add(OBJ)).lw(c);
    // b: the same (s2); its object base in s5.
    c.cycles += 9;
    c.g[A1][0] = sx(r32(c, S0).wrapping_add(OBJ));
    c.g[S5][0] = c.g[A1][0];
    c.g[S2][0] = 0;
    let (obj, vt) = (c.g[A1][0], c.g[V1][0]);
    rs207d_obj_call(c, obj, vt, VT_IS_AI, 0x002a_06a4)?;
    let mut set = c.g[V0][0] != 0;
    if !set {
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(LOCAL)).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            set = true;
        }
    }
    if set {
        c.g[S2][0] = 1;
        c.cycles += 1;
    }
    c.g[A0][0] = c.g[S3][0];
    // The followed rider must be a or b unless both are AI / not local.
    c.cycles += 4;
    c.g[A1][0] = u64::MAX;
    call_ptr(c, 0x0028_5d98, 0x002a_06c8)?; // SSXAudioSystem_FindRider(sys, -1)
    let a_flag = c.g[S4][0] != 0;
    c.g[S4][0] = sx(r32(c, S3).wrapping_add(SPEECH));
    let mut tail = true;
    if a_flag {
        c.cycles += 2;
        if c.g[S2][0] != 0 {
            c.cycles += 2;
            if c.g[S1][0] == c.g[V0][0] {
                tail = false;
            } else {
                c.cycles += 2;
                if c.g[S0][0] != c.g[V0][0] {
                    return Ok(false);
                }
                c.cycles += 1;
                c.g[S4][0] = sx(r32(c, S3).wrapping_add(SPEECH));
            }
        }
    }
    if tail {
        c.cycles += 1;
    }
    c.g[A1][0] = 0;
    c.cycles += 4;
    c.g[A0][0] = c.g[S4][0];
    RS207D_IS_INSTANCE_READY.call(c, 0x002a_06f8)?;
    if c.g[V0][0] == 0 {
        return Ok(false);
    }
    // Both riders must be heard by the same listener view.
    c.cycles += 12;
    c.g[V1][0] = Addr(r32(c, S1).wrapping_add(OBJ)).lw(c);
    let (obj, vt) = (c.g[S6][0], c.g[V1][0]);
    rs207d_obj_call(c, obj, vt, VT_POS, 0x002a_0714)?;
    c.lwc1(12, gp.wrapping_add(RADIUS_GP));
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, 0x0028_8b40, 0x002a_0724)?; // SSXAudioSystem_FindViewInRange
    c.g[S2][0] = c.g[V0][0];
    if (c.g[S2][0] as i64) < 0 {
        return Ok(false);
    }
    c.cycles += 11;
    c.g[V1][0] = Addr(r32(c, S0).wrapping_add(OBJ)).lw(c);
    let (obj, vt) = (c.g[S5][0], c.g[V1][0]);
    rs207d_obj_call(c, obj, vt, VT_POS, 0x002a_0744)?;
    c.lwc1(12, gp.wrapping_add(RADIUS_GP));
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, 0x0028_8b40, 0x002a_0754)?;
    if c.g[S2][0] != c.g[V0][0] {
        return Ok(false);
    }
    // The gap along b's heading: dot((cos, sin, 0, 0), dirA - dirB).
    c.cycles += 44;
    c.lwc1(12, r32(c, S0).wrapping_add(HEADING));
    c.g[A0][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A1][0] = sx(sp(c).wrapping_add(0x14));
    call_ptr(c, 0x0031_be50, 0x002a_076c)?; // Math_SinCos(heading, &sin, &cos)
    let s = sp(c);
    c.lwc1(0, s.wrapping_add(0x14));
    c.lwc1(1, s.wrapping_add(0x10));
    c.swc1(0, s);
    c.swc1(1, s.wrapping_add(0x4));
    c.sw(s.wrapping_add(0x8), 0);
    c.sw(s.wrapping_add(0xc), 0);
    c.g[V1][0] = Addr(r32(c, S1).wrapping_add(OBJ)).lw(c);
    let (obj, vt) = (c.g[S6][0], c.g[V1][0]);
    rs207d_obj_call(c, obj, vt, VT_DIR, 0x002a_0798)?;
    c.g[V1][0] = Addr(r32(c, S0).wrapping_add(OBJ)).lw(c);
    let v = r32(c, V0);
    rs207d_dot(c, v);
    c.qmfc2(A0, 4);
    c.mtc1(A0, 20);
    let (obj, vt) = (c.g[S5][0], c.g[V1][0]);
    rs207d_obj_call(c, obj, vt, VT_DIR, 0x002a_07d0)?;
    let v = r32(c, V0);
    rs207d_dot(c, v);
    c.qmfc2(V1, 4);
    c.mtc1(V1, 0);
    c.lwc1(1, gp.wrapping_sub(MIN_GAP_GP));
    c.sub_s(20, 20, 0);
    c.c_lt_s(1, 20);
    if !c.fcc() {
        return Ok(false);
    }
    c.cycles += 5;
    c.lwc1(0, gp.wrapping_sub(MAX_GAP_GP));
    c.c_lt_s(20, 0);
    let too_far = !c.fcc();
    c.g[A0][0] = c.g[S1][0];
    if too_far {
        return Ok(true);
    }
    // Speech masks (s6: a, s7: b), b's voice mask (s2), gap class (s5).
    c.cycles += 14;
    c.g[S5][0] = 1;
    RS207D_GET_RIDER_SPEECH_MASK.call(c, 0x002a_0828)?;
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = c.g[S0][0];
    c.g[S6][0] = c.g[V0][0];
    RS207D_GET_RIDER_VOICE_MASK.call(c, 0x002a_0838)?;
    c.g[S2][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S0][0];
    RS207D_GET_RIDER_SPEECH_MASK.call(c, 0x002a_0844)?;
    c.lwc1(0, gp.wrapping_sub(FAR_GAP_GP));
    c.c_le_s(0, 20);
    let near = !c.fcc();
    c.g[S7][0] = c.g[V0][0];
    if !near {
        c.cycles += 1;
        c.g[S5][0] = 2;
    }
    // Rivalry: 2 the rival lines, 0/1 the pass lines.
    c.cycles += 8;
    c.g[A0][0] = c.g[S3][0];
    c.g[A1][0] = c.g[S1][0];
    c.g[A2][0] = c.g[S0][0];
    RS207D_GET_RIDER_RIVALRY.call(c, 0x002a_086c)?;
    c.g[V1][0] = c.g[V0][0];
    c.g[V0][0] = 2;
    let rival = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = (c.g[V1][0] < 2) as u64;
    if rival {
        // a: "rival passed" (0x2081), b: "passed by rival" (0x207f).
        c.cycles += 14;
        c.mtc1(0, 20);
        let rets = [0x002a_0888, 0x002a_08ac, 0x002a_08c0, 0x002a_08d8];
        rs207d_post(
            c,
            S1,
            0x2081,
            rets,
            9,
            |_| {},
            |c| {
                c.g[A2][0] = c.g[S6][0];
                c.g[A1][0] = 2;
                c.g[A3][0] = c.g[S5][0];
            },
        )?;
        c.cycles += 13;
        let rets = [0x002a_08e0, 0x002a_0904, 0x002a_0918, 0x002a_0930];
        let posted = rs207d_post(
            c,
            S0,
            0x207f,
            rets,
            11,
            |_| {},
            |c| {
                c.g[A2][0] = c.g[S7][0];
                c.g[A3][0] = c.g[S5][0];
                c.g[A1][0] = 2;
            },
        )?;
        return Ok(!posted);
    }
    c.cycles += 2;
    if c.g[V0][0] == 0 {
        return Ok(false);
    }
    // a: "passed" (0x2080, with b's voice mask | 0x1000), b: "got
    // passed" (0x207d).
    c.cycles += 14;
    c.mtc1(0, 20);
    let rets = [0x002a_094c, 0x002a_0970, 0x002a_0988, 0x002a_09a4];
    let before = |c: &mut Cpu| c.g[S2][0] |= 0x1000;
    rs207d_post(c, S1, 0x2080, rets, 11, before, |c| {
        c.g[A2][0] = c.g[S6][0];
        c.g[T0][0] = c.g[S2][0];
        c.g[A1][0] = 3;
        c.g[A3][0] = c.g[S5][0];
    })?;
    c.cycles += 13;
    let rets = [0x002a_09ac, 0x002a_09d0, 0x002a_09e4, 0x002a_09fc];
    rs207d_post(
        c,
        S0,
        0x207d,
        rets,
        9,
        |_| {},
        |c| {
            c.g[A2][0] = c.g[S7][0];
            c.g[A3][0] = c.g[S5][0];
            c.g[A1][0] = 2;
        },
    )?;
    Ok(true)
}

// ---------------------------------------------------------------------------
// 0x002a77a8 CrowdAudio_PitchBendCallback

pub(crate) const CAPB_UPDATE_PITCH_BEND: Callee = Callee {
    addr: 0x002a_68b0,
    f: ee::s08::CrowdAudio_UpdatePitchBend,
};

/// `void CrowdAudio_PitchBendCallback(void* ctx, BankMonitorInstance*
/// voice, float dt)` (0x2a77a8): the bank monitor's per-voice callback:
/// `CrowdAudio_UpdatePitchBend(voice->owner (+0x80), voice, dt)`.
pub fn crowd_audio_pitch_bend_callback(c: &mut Cpu) {
    let frame = Frame { size: 0x10 };
    guest(c, |c| {
        c.cycles += 7;
        Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(r32(c, A1)).off(0x80).lw(c);
        CAPB_UPDATE_PITCH_BEND.call(c, 0x002a_77b8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// CrowdAudio_InstanceSetSourceParam (0x2a5cd8).

/// Crowd audio instance: 12-byte sources from `+0x20` (`+0` active, `+4`
/// the parameter).
pub(crate) const CROWD_SOURCE_SIZE: u32 = 0xc;

pub(crate) const CROWD_SOURCE_ACTIVE: u32 = 0x20;

pub(crate) const CROWD_SOURCE_PARAM: u32 = 0x24;

/// `void CrowdAudio_InstanceSetSourceParam(Instance* inst, int source, int
/// value)` (0x2a5cd8): set an active source's parameter (negative source
/// indices are ignored).
pub fn crowd_audio_instance_set_source_param(c: &mut Cpu) {
    c.cycles += 2;
    c.g[A3][0] = c.g[A0][0];
    if (c.g[A1][0] as i64) >= 0 {
        c.cycles += 6;
        c.g[V0][0] = CROWD_SOURCE_SIZE as u64;
        c.mult(0, A0, A1, V0);
        let source = Addr(r32(c, A3).wrapping_add(r32(c, A0)));
        c.g[V1][0] = sx(source.0);
        c.g[V0][0] = source.off(CROWD_SOURCE_ACTIVE).lw(c);
        let active = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[V1][0];
        if active {
            c.cycles += 1;
            source.off(CROWD_SOURCE_PARAM).sw(c, c.g[A2][0]);
        }
    }
    c.cycles += 2;
    ret(c);
}

// ---------------------------------------------------------------------------
// SSXAudioSystem_StartBoardEdgeLoop (0x292508)

/// `SSXAudioSystem` fields: the bank monitor (`+0`), the push tables of
/// bank and sample per queue entry (`+4`, `+8`), the sound root (`+0x118`,
/// `*root` is the monitor too).
pub(crate) const OC300_SYS_MONITOR: u32 = 0x0;

pub(crate) const OC300_SYS_PUSH_BANK: u32 = 0x4;

pub(crate) const OC300_SYS_PUSH_SAMPLE: u32 = 0x8;

pub(crate) const OC300_SYS_SND_ROOT: u32 = 0x118;

/// Rider audio fields: human (`+0x874`), local (`+0x87c`), listener index
/// (`+0x870`); the rider's sound position at `+0x110`.
pub(crate) const OC300_RIDER_HUMAN: u32 = 0x874;

pub(crate) const OC300_RIDER_LOCAL: u32 = 0x87c;

pub(crate) const OC300_RIDER_LISTENER: u32 = 0x870;

pub(crate) const OC300_RIDER_SOUND_POS: u32 = 0x110;

/// Bank monitor: the push queue (`+0x1d8`) and its entry count (`+0x1f4`,
/// = queue `+0x1c`); the 24-byte entry template at `+0x1dc`; per-entry
/// handles inline at `+0x1fc`.
pub(crate) const OC300_MON_QUEUE: u32 = 0x1d8;

pub(crate) const OC300_MON_COUNT: u32 = 0x1f4;

pub(crate) const OC300_MON_HANDLES: u32 = 0x1fc;

/// The game-mode byte (2: replays possible).
pub(crate) const OC300_MODE_BYTE: u32 = 0x0053_5c11;

/// The 8-byte callback key copied into the entry's slot (0x4a36a0).
pub(crate) const OC300_EDGE_KEY: u32 = 0x004a_36a0;

/// `SSXAudioSystem_IsReplayPlaying` (0x288ae0).
pub(crate) const OC300_IS_REPLAY_PLAYING: Callee = Callee {
    addr: 0x0028_8ae0,
    f: ee::s07::SSXAudioSystem_IsReplayPlaying,
};

/// `SSXAudioSystem_GetSurfaceBank` (0x2917b8).
pub(crate) const OC300_GET_SURFACE_BANK: Callee = Callee {
    addr: 0x0029_17b8,
    f: ee::s07::SSXAudioSystem_GetSurfaceBank,
};

/// `SSXAudioSystem_GetChannelGainPtr` (0x287968).
pub(crate) const OC300_GET_CHANNEL_GAIN_PTR: Callee = Callee {
    addr: 0x0028_7968,
    f: ee::s07::SSXAudioSystem_GetChannelGainPtr,
};

/// `SSXAudioSystem_GetRiderListenerIndex` (0x288a20).
pub(crate) const OC300_GET_RIDER_LISTENER_INDEX: Callee = Callee {
    addr: 0x0028_8a20,
    f: ee::s07::SSXAudioSystem_GetRiderListenerIndex,
};

/// `Audio_PlayPushedSound` (0x2906b8).
pub(crate) const OC300_PLAY_PUSHED_SOUND: Callee = Callee {
    addr: 0x0029_06b8,
    f: ee::s07::Audio_PlayPushedSound,
};

/// `table[count] = value` on the monitor's push queue: `cnt = monitor->count;
/// tab = queue[off]; *(tab + (cnt << shift) + extra) = value`, in the
/// registers the original uses.
#[allow(clippy::too_many_arguments)]
pub(crate) fn oc300_edge_store(
    c: &mut Cpu,
    (cnt, tab): (usize, usize),
    monitor: u32,
    queue: u32,
    off: u32,
    shift: u32,
    extra: u32,
    value: u64,
) {
    c.g[cnt][0] = Addr(monitor).off(OC300_MON_COUNT).lw(c);
    c.g[tab][0] = Addr(queue).off(off).lw(c);
    c.g[cnt][0] = sx((c.g[cnt][0] as u32) << shift);
    c.g[cnt][0] = sx((c.g[cnt][0] as u32).wrapping_add(c.g[tab][0] as u32));
    Addr(c.g[cnt][0] as u32).off(extra).sw(c, value);
}

/// `*(lw(lw(sys->sndRoot) + off) + count * 4) = value` (the root monitor's
/// table `off` at the current entry), in the registers the original uses:
/// `root`, `mon`, `cnt`, `tab`.
pub(crate) fn oc300_edge_root_store(c: &mut Cpu, regs: [usize; 4], off: u32, value: u64) {
    let [root, mon, cnt, tab] = regs;
    c.g[root][0] = Addr(r32(c, S5)).off(OC300_SYS_SND_ROOT).lw(c);
    c.g[mon][0] = Addr(c.g[root][0] as u32).lw(c);
    c.g[cnt][0] = Addr(c.g[mon][0] as u32).off(OC300_MON_COUNT).lw(c);
    c.g[tab][0] = Addr(c.g[mon][0] as u32).off(off).lw(c);
    c.g[cnt][0] = sx((c.g[cnt][0] as u32) << 2);
    c.g[cnt][0] = sx((c.g[cnt][0] as u32).wrapping_add(c.g[tab][0] as u32));
    Addr(c.g[cnt][0] as u32).sw(c, value);
}

/// `void SSXAudioSystem_StartBoardEdgeLoop(SSXAudioSystem* sys, Rider*
/// rider, int surface)` (0x292508): for a human local rider, push the
/// board-edge loop on the bank monitor's push queue (bank from
/// `GetSurfaceBank(sys, surface)`, sample `surface * 8 + 3`; volume 100,
/// pan 90, 50, 127), clamp its volume to 0..127, attach the channel-6 gain,
/// the rider's sound position, callback key and listener index, set radius
/// 100 / vanish -1; during a replay (mode byte 2) mark it as a replay sound
/// of the rider's listener. Then `Audio_PlayPushedSound(sys)`.
pub fn ssx_audio_system_start_board_edge_loop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xa0);
        c.g[V1][0] = 0;
        frame.save(c, S2, 0x70);
        frame.save(c, S5, 0x40);
        c.g[S2][0] = c.g[A2][0]; // surface
        frame.save(c, S6, 0x30);
        frame.save(c, S0, 0x90);
        c.g[S6][0] = c.g[A1][0]; // rider
        frame.save(c, S1, 0x80);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S7, 0x20);
        frame.save_ra(c, 0x10);
        let rider = r32(c, S6);
        c.g[V0][0] = Addr(rider).off(OC300_RIDER_HUMAN).lw(c);
        c.g[S5][0] = c.g[A0][0]; // sys
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = Addr(rider).off(OC300_RIDER_LOCAL).lw(c);
            c.g[V1][0] = (c.g[V0][0] != 0) as u64;
        }
        c.cycles += 2;
        let play = c.g[V1][0] != 0;
        frame.restore(c, S0, 0x90);
        if play {
            oc300_edge_push_and_play(c)?;
            frame.restore(c, S0, 0x90);
        }
        c.cycles += 10;
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// The push-and-play body of `StartBoardEdgeLoop` (s5 = sys, s6 = rider,
/// s2 = surface).
pub(crate) fn oc300_edge_push_and_play(c: &mut Cpu) -> crate::call::Flow {
    // In a replay-capable mode, remember whether no replay is playing.
    c.cycles += 10;
    c.g[S7][0] = 0;
    call_ptr(c, 0x0014_dc80, 0x0029_2560)?; // BELibrary_GetInstance
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x0014_dd58, 0x0029_256c)?; // BELibrary_GetInterface(be, 0)
    c.g[V1][0] = 0x53_0000;
    c.g[A0][0] = 2;
    c.g[V0][0] = Addr(OC300_MODE_BYTE).lb(c);
    if c.g[V0][0] != c.g[A0][0] {
        c.g[A0][0] = c.g[S5][0];
    } else {
        c.cycles += 4;
        c.g[A0][0] = c.g[S5][0];
        OC300_IS_REPLAY_PLAYING.call(c, 0x0029_2588)?;
        c.g[S7][0] = (c.g[V0][0] < 1) as u64;
        c.g[A0][0] = c.g[S5][0];
    }

    // bank = GetSurfaceBank(sys, surface).
    c.cycles += 159;
    c.g[A1][0] = c.g[S2][0];
    OC300_GET_SURFACE_BANK.call(c, 0x0029_2598)?;
    // New queue entry: count += 1, copy the 24-byte template into the
    // entry's record (record size 0x18 in s3).
    c.g[S3][0] = 0x18;
    c.g[S0][0] = Addr(r32(c, S5)).off(OC300_SYS_MONITOR).lw(c);
    c.g[S4][0] = c.g[V0][0]; // bank
    c.g[S2][0] = sx(r32(c, S2) << 3);
    c.g[A1][0] = 0;
    let monitor = r32(c, S0);
    c.g[V0][0] = Addr(monitor).off(OC300_MON_COUNT).lw(c);
    c.g[S1][0] = sx(monitor.wrapping_add(OC300_MON_QUEUE));
    let queue = r32(c, S1);
    c.g[S2][0] = sx(r32(c, S2).wrapping_add(3)); // sample = surface * 8 + 3
    c.g[A2][0] = 0x8;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    Addr(monitor).off(OC300_MON_COUNT).sw(c, c.g[V0][0]);
    c.g[V1][0] = Addr(monitor).off(OC300_MON_COUNT).lw(c);
    c.g[V0][0] = Addr(queue).off(0x20).lw(c);
    c.mult(0, 3, 3, 19);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.ldl(2, monitor.wrapping_add(0x1e3));
    c.ldr(2, monitor.wrapping_add(0x1dc));
    c.ldl(4, monitor.wrapping_add(0x1eb));
    c.ldr(4, monitor.wrapping_add(0x1e4));
    c.ldl(7, monitor.wrapping_add(0x1f3));
    c.ldr(7, monitor.wrapping_add(0x1ec));
    let record = r32(c, V1);
    c.sdl(2, record.wrapping_add(0x7));
    c.sdr(2, record);
    c.sdl(4, record.wrapping_add(0xf));
    c.sdr(4, record.wrapping_add(0x8));
    c.sdl(7, record.wrapping_add(0x17));
    c.sdr(7, record.wrapping_add(0x10));
    // monitor->handles[count] = 0.
    c.g[V0][0] = Addr(monitor).off(OC300_MON_COUNT).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx(monitor.wrapping_add(c.g[V0][0] as u32));
    Addr(r32(c, V0)).off(OC300_MON_HANDLES).sw(c, 0);
    oc300_edge_store(c, (V1, V0), monitor, queue, 0x34, 2, 0, 0);
    // table 0x38 [count] = the entry record's first byte (sign-extended).
    c.g[A3][0] = Addr(monitor).off(OC300_MON_COUNT).lw(c);
    c.g[A0][0] = Addr(monitor).off(OC300_MON_COUNT).lw(c);
    c.g[T0][0] = Addr(queue).off(0x20).lw(c);
    c.g[A3][0] = sx(r32(c, A3) << 2);
    c.mult(0, 4, 4, 19);
    c.g[V0][0] = Addr(queue).off(0x38).lw(c);
    c.g[A3][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, T0)));
    c.g[V1][0] = Addr(r32(c, A0)).lb(c);
    Addr(r32(c, A3)).sw(c, c.g[V1][0]);
    oc300_edge_store(c, (V0, V1), monitor, queue, 0x3c, 2, 0, 0);
    // memset(&queue->slots[count] (8 bytes at +0x40), 0, 8).
    c.g[A0][0] = Addr(monitor).off(OC300_MON_COUNT).lw(c);
    c.g[A0][0] = sx(r32(c, A0) << 3);
    c.g[A0][0] = sx(queue.wrapping_add(r32(c, A0)));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x40));
    call_ptr(c, 0x0041_6210, 0x0029_2678)?;

    // The entry's parameters (volume 100, pan 90/50/127, flag 1).
    let monitor = r32(c, S0);
    let queue = r32(c, S1);
    c.g[A0][0] = 0x64;
    c.g[A1][0] = 0x5a;
    c.g[A2][0] = 0x32;
    c.g[A3][0] = 0x7f;
    oc300_edge_store(c, (V1, V0), monitor, queue, 0x60, 2, 0, 0);
    c.g[T0][0] = 0x1;
    oc300_edge_store(c, (V0, V1), monitor, queue, 0x64, 2, 0, 0);
    oc300_edge_store(c, (V1, V0), monitor, queue, 0x68, 2, 0, 0);
    oc300_edge_store(c, (V0, V1), monitor, queue, 0x6c, 2, 0, 0);
    oc300_edge_store(c, (V1, V0), monitor, queue, 0x70, 4, 0x4, 0);
    oc300_edge_store(c, (V0, V1), monitor, queue, 0x70, 4, 0x0, 0x64);
    oc300_edge_store(c, (V1, V0), monitor, queue, 0x70, 4, 0x8, 0x5a);
    oc300_edge_store(c, (V0, V1), monitor, queue, 0x70, 4, 0xc, 0x32);
    oc300_edge_store(c, (V1, V0), monitor, queue, 0x74, 2, 0, 0);
    oc300_edge_store(c, (V0, V1), monitor, queue, 0x80, 2, 0, 0);
    oc300_edge_store(c, (V1, V0), monitor, queue, 0x84, 2, 0, 0);
    oc300_edge_store(c, (V0, V1), monitor, queue, 0x78, 2, 0, 0x7f);
    oc300_edge_store(c, (V1, V0), monitor, queue, 0x7c, 2, 0, 1);

    // sys->pushBank[count] = bank (s4); sys->pushSample[count] = sample (s2).
    let sys = r32(c, S5);
    c.g[A0][0] = Addr(sys).off(OC300_SYS_MONITOR).lw(c);
    c.g[V1][0] = Addr(sys).off(OC300_SYS_PUSH_BANK).lw(c);
    c.g[V0][0] = Addr(r32(c, A0)).off(OC300_MON_COUNT).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    Addr(r32(c, V0)).sw(c, c.g[S4][0]);
    c.g[V1][0] = Addr(sys).off(OC300_SYS_MONITOR).lw(c);
    c.g[A0][0] = Addr(sys).off(OC300_SYS_PUSH_SAMPLE).lw(c);
    c.g[V0][0] = Addr(r32(c, V1)).off(OC300_MON_COUNT).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
    Addr(r32(c, V0)).sw(c, c.g[S2][0]);

    // vals[count] (root queue table 0x38) := 0, then the entry record's
    // first byte := clamp(vals[count], 0, 127).
    c.g[V1][0] = Addr(sys).off(OC300_SYS_SND_ROOT).lw(c);
    c.g[A1][0] = Addr(r32(c, V1)).lw(c);
    let root_mon = r32(c, A1);
    c.g[V0][0] = Addr(root_mon).off(OC300_MON_COUNT).lw(c);
    c.g[A2][0] = sx(root_mon.wrapping_add(OC300_MON_QUEUE));
    let root_queue = r32(c, A2);
    c.g[V1][0] = Addr(root_queue).off(0x38).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    Addr(r32(c, V0)).sw(c, 0);
    c.g[A0][0] = Addr(root_mon).off(OC300_MON_COUNT).lw(c);
    c.g[V1][0] = Addr(root_queue).off(0x1c).lw(c);
    c.g[A3][0] = Addr(root_queue).off(0x38).lw(c);
    c.mult(0, 4, 4, 19);
    c.g[V1][0] = sx(r32(c, V1) << 2);
    c.g[A1][0] = Addr(root_queue).off(0x20).lw(c);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A3)));
    c.g[V0][0] = Addr(r32(c, V1)).lw(c);
    c.g[V0][0] = ((c.g[V0][0] as i64) < 127) as u64;
    let below_max = c.g[V0][0] != 0;
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
    // The current volume `vals[cur]` (queue + 0x1c is the count).
    let volume_at = |c: &mut Cpu| {
        c.g[V0][0] = Addr(root_queue).off(0x1c).lw(c);
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A3)));
    };
    let mut negative = false;
    if below_max {
        c.cycles += 6;
        volume_at(c);
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        negative = (c.g[V1][0] as i64) < 0;
        c.g[V0][0] = 0;
    }
    if !negative {
        c.cycles += 7;
        volume_at(c);
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        c.g[V1][0] = ((c.g[V1][0] as i64) < 127) as u64;
        if c.g[V1][0] == 0 {
            c.g[V0][0] = 0x7f;
        } else {
            c.cycles += 4;
            volume_at(c);
            c.g[V0][0] = Addr(r32(c, V0)).lbu(c);
        }
    }
    c.cycles += 69;
    Addr(r32(c, A0)).sb(c, c.g[V0][0]);

    // root->gains[count] = GetChannelGainPtr(sys, 6, 0).
    c.g[A2][0] = 0;
    c.g[A0][0] = c.g[S5][0];
    c.g[A1][0] = 0x6;
    c.g[S0][0] = Addr(r32(c, S5)).off(OC300_SYS_SND_ROOT).lw(c);
    OC300_GET_CHANNEL_GAIN_PTR.call(c, 0x0029_2868)?;
    c.g[A3][0] = Addr(r32(c, S0)).lw(c);
    c.g[T0][0] = sx(r32(c, S6).wrapping_add(OC300_RIDER_SOUND_POS));
    c.g[T1][0] = 0x4a_0000;
    c.g[A0][0] = c.g[S5][0];
    let mon = r32(c, A3);
    c.g[V1][0] = Addr(mon).off(OC300_MON_COUNT).lw(c);
    c.g[A1][0] = c.g[S6][0];
    c.g[A2][0] = Addr(mon).off(0x214).lw(c);
    c.g[V1][0] = sx(r32(c, V1) << 2);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A2)));
    Addr(r32(c, V1)).sw(c, c.g[V0][0]);
    // root->handles[count] = &rider->soundPos.
    c.g[A2][0] = Addr(r32(c, S5)).off(OC300_SYS_SND_ROOT).lw(c);
    c.g[V1][0] = Addr(r32(c, A2)).lw(c);
    c.g[V0][0] = Addr(r32(c, V1)).off(OC300_MON_COUNT).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    Addr(r32(c, V1)).off(OC300_MON_HANDLES).sw(c, c.g[T0][0]);
    // The callback key (8 bytes at 0x4a36a0, through the stack) into the
    // entry's slot (queue + 0x40 + count * 8).
    c.g[V0][0] = Addr(r32(c, S5)).off(OC300_SYS_SND_ROOT).lw(c);
    c.g[A2][0] = Addr(r32(c, V0)).lw(c);
    c.g[T0][0] = sx(OC300_EDGE_KEY);
    c.ldl(2, OC300_EDGE_KEY.wrapping_add(0x7));
    c.ldr(2, OC300_EDGE_KEY);
    let stack = sp(c);
    c.sdl(2, stack.wrapping_add(0x7));
    c.sdr(2, stack);
    let mon = r32(c, A2);
    c.g[V1][0] = sx(mon.wrapping_add(OC300_MON_QUEUE));
    c.g[V0][0] = Addr(mon).off(OC300_MON_COUNT).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 3);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    c.ldl(2, stack.wrapping_add(0x7));
    c.ldr(2, stack);
    let slot = r32(c, V1);
    c.sdl(2, slot.wrapping_add(0x47));
    c.sdr(2, slot.wrapping_add(0x40));
    // root table 0x254 [count] = 0; root table 0x238 [count] = rider.
    oc300_edge_root_store(c, [A2, V1, V0, A2], 0x254, 0);
    let rider = c.g[S6][0];
    oc300_edge_root_store(c, [V1, A2, V0, V1], 0x238, rider);
    // root table 0x23c [count] = GetRiderListenerIndex(sys, rider).
    c.g[S0][0] = Addr(r32(c, S5)).off(OC300_SYS_SND_ROOT).lw(c);
    OC300_GET_RIDER_LISTENER_INDEX.call(c, 0x0029_2924)?;
    c.g[A0][0] = Addr(r32(c, S0)).lw(c);
    c.g[AT][0] = 0x42c8_0000;
    c.mtc1(1, 12); // radius 100.0
    let mon = r32(c, A0);
    c.g[V1][0] = Addr(mon).off(OC300_MON_COUNT).lw(c);
    c.g[A1][0] = Addr(mon).off(0x23c).lw(c);
    c.g[V1][0] = sx(r32(c, V1) << 2);
    c.g[AT][0] = sx(0xbf80_0000);
    c.mtc1(1, 13); // vanish -1.0
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
    Addr(r32(c, V1)).sw(c, c.g[V0][0]);
    // SndPlayParams_SetRadiusVanish(&root->queue, 100.0, -1.0).
    c.g[V0][0] = Addr(r32(c, S5)).off(OC300_SYS_SND_ROOT).lw(c);
    c.g[A0][0] = Addr(r32(c, V0)).lw(c);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(OC300_MON_QUEUE));
    call_ptr(c, 0x002a_9988, 0x0029_295c)?;
    let replay = c.g[S7][0] != 0;
    c.g[A1][0] = 1;
    if replay {
        // root table 0x258 [count] = 1; root table 0x25c [count] =
        // rider->listener.
        c.cycles += 15;
        oc300_edge_root_store(c, [A0, V1, V0, A0], 0x258, 1);
        c.g[A0][0] = Addr(r32(c, S5)).off(OC300_SYS_SND_ROOT).lw(c);
        c.g[A1][0] = Addr(r32(c, S6)).off(OC300_RIDER_LISTENER).lw(c);
        c.g[V1][0] = Addr(r32(c, A0)).lw(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(OC300_MON_COUNT).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).off(0x25c).lw(c);
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
        Addr(r32(c, V0)).sw(c, c.g[A1][0]);
    }
    c.cycles += 3;
    c.g[A0][0] = c.g[S5][0];
    OC300_PLAY_PUSHED_SOUND.call(c, 0x0029_29a8)
}

pub(crate) mod closer_view {
    use crate::call::Callee;
    use ssx3_ee as ee;

    pub const GET_LISTENER_COUNT: Callee = Callee {
        addr: 0x0028_8ce8,
        f: ee::s07::SSXAudioSystem_GetListenerCount,
    };

    pub const GET_VIEW_POS: Callee = Callee {
        addr: 0x0028_8ac0,
        f: ee::s07::SSXAudioSystem_GetViewPos,
    };
}

pub(crate) mod wipeout_recover {
    use crate::call::Callee;
    use ssx3_ee as ee;

    /// Rider: +0x470 course progress (< 0 before the finish), +0x6c0 its
    /// scripting object (vtable `+0x28` position, `+0x48` "is AI"), +0x87c
    /// the local-human flag.
    pub const RIDER_PROGRESS: u32 = 0x470;

    pub const RIDER_OBJ: u32 = 0x6c0;

    pub const RIDER_LOCAL: u32 = 0x87c;

    pub const VT_POSITION: u32 = 0x28;

    pub const VT_IS_AI: u32 = 0x48;

    /// SSXAudioSystem: +0x5560 the speech system.
    pub const SYS_SPEECH: u32 = 0x5560;

    /// `gp+0x5e8`: the hearing radius (/100) for the listener check;
    /// `gp+0x173c`: the speech post hook.
    pub const GP_RADIUS: u32 = 0x5e8;

    pub const GP_POST_HOOK: u32 = 0x173c;

    /// The recover lines by track category (1, 2, anything else).
    pub const LINE_CAT1: u64 = 0x2088;

    pub const LINE_CAT2: u64 = 0x20a0;

    pub const LINE_OTHER: u64 = 0x20a1;

    pub const IS_SPEECH_VOLUME_ON: Callee = Callee {
        addr: 0x0029_f0a0,
        f: ee::s08::SSXAudioSystem_IsSpeechVolumeOn,
    };

    pub const IS_INSTANCE_READY: Callee = Callee {
        addr: 0x002b_0e28,
        f: ee::s08::SpeechSystem_IsInstanceReady,
    };

    pub const GET_RIDER_SPEECH_MASK: Callee = Callee {
        addr: 0x0029_f330,
        f: ee::s08::SSXAudioSystem_GetRiderSpeechMask,
    };

    pub const IS_TRACK_GROUP5: Callee = Callee {
        addr: 0x002a_3f90,
        f: ee::s08::SSXAudioSystem_IsTrackGroup5,
    };

    pub const IS_EVENT_MODE4: Callee = Callee {
        addr: 0x002a_4040,
        f: ee::s08::SSXAudioSystem_IsEventMode4,
    };

    pub const TRACK_TO_CATEGORY: Callee = Callee {
        addr: 0x002a_1e20,
        f: ee::s08::SSXAudioSystem_TrackToCategory,
    };

    pub const SET_EVENT_CONTEXT: Callee = Callee {
        addr: 0x002b_1458,
        f: ee::s08::SpeechSystem_SetEventContext,
    };

    // Cross-shard direct calls (emitted as `call_indirect`).
    pub const IS_REPLAY_PLAYING: u32 = 0x0028_8ae0;

    pub const FIND_RIDER: u32 = 0x0028_5d98;

    pub const FIND_VIEW_IN_RANGE: u32 = 0x0028_8b40;

    pub const GET_BE_LIBRARY: u32 = 0x0028_b1e8;

    pub const BE_GET_INTERFACE: u32 = 0x0014_dd58;

    pub const RACE_GET_TRACK: u32 = 0x0014_4bc0;

    pub const RIDER_GET_CHARACTER: u32 = 0x0012_3128;

    pub const SPCH_MAKE_ID: u32 = 0x003d_8008;
}

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
pub(crate) fn wol_ea(c: &Cpu, r: usize, off: u32) -> u32 {
    (c.g[r][0] as u32).wrapping_add(off)
}

/// `lw rd, off(rb)`.
#[inline(always)]
pub(crate) fn wol_lw(c: &mut Cpu, rd: usize, rb: usize, off: u32) {
    let a = wol_ea(c, rb, off);
    c.g[rd][0] = c.lw(a);
}

/// `sll rd, rd, shift; addu rd, rd, rb` (an array element address).
#[inline(always)]
pub(crate) fn wol_index(c: &mut Cpu, rd: usize, shift: u32, rb: usize) {
    c.g[rd][0] = sx(((c.g[rd][0] as u32) << shift).wrapping_add(c.g[rb][0] as u32));
}

/// Store `val` into element `cursor` of the monitor's (s0) record array at
/// descriptor (s1) field `field` (element size `1 << shift`, at `+off`),
/// with the original's register pair `ri`/`rb`.
pub(crate) fn wol_record_store(
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
pub(crate) fn wol_copy_dwords(c: &mut Cpu, regs: &[usize], src: u32, dst: u32) {
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
pub(crate) fn wol_should_start(c: &mut Cpu, frame: Frame) -> Result<bool, crate::call::Unwound> {
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
pub(crate) fn wol_start(c: &mut Cpu) -> crate::call::Flow {
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
pub(crate) fn wol_replay_flag(c: &mut Cpu) -> crate::call::Flow {
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
pub(crate) fn wol_push_record(c: &mut Cpu) -> crate::call::Flow {
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
pub(crate) fn wol_finish_record(c: &mut Cpu) -> crate::call::Flow {
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

/// `bool SSXAudioSystem_IsCloserToSecondView(void* self, sSndVec4* pos)`
/// (0x288940): in split screen (two or more listeners), whether `pos` is
/// at least as close to the second view as to the first (squared 4D
/// distances from VU0: `!(d0 < d1)`); false with a single listener.
pub fn ssx_audio_is_closer_to_second_view(c: &mut Cpu) {
    use closer_view as k;
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        c.g[S1][0] = c.g[A1][0];
        k::GET_LISTENER_COUNT.call(c, 0x0028_895c)?;
        let single = (c.g[V0][0] as i64) < 2;
        c.g[V0][0] = 0;
        if !single {
            c.cycles += 41;
            // d0 = view0 - pos at sp+0x10, then copied to sp+0
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            k::GET_VIEW_POS.call(c, 0x0028_8974)?;
            c.lqc2(3, r32(c, V0));
            c.lqc2(4, r32(c, S1));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, frame.at(c, 0x10));
            c.g[A0][0] = c.g[S0][0];
            c.lq(V0, frame.at(c, 0x10));
            c.g[A1][0] = 1;
            c.sq(V0, frame.at(c, 0));
            // d1 = view1 - pos at sp+0x20, then copied to sp+0x10
            k::GET_VIEW_POS.call(c, 0x0028_8998)?;
            c.lqc2(3, r32(c, V0));
            c.lqc2(4, r32(c, S1));
            c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, frame.at(c, 0x20));
            c.lq(V1, frame.at(c, 0x20));
            c.g[V0][0] = 0;
            c.sq(V1, frame.at(c, 0x10));
            // f1 = dot(d0, d0)
            closer_view_dot(c, frame.at(c, 0), A0);
            c.mtc1(A0, 1);
            // f0 = dot(d1, d1)
            closer_view_dot(c, frame.at(c, 0x10), V1);
            c.mtc1(V1, 0);
            c.c_lt_s(1, 0);
            if !c.fcc() {
                c.g[V0][0] = 1;
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    });

    /// `dot(d, d)` of the vector at `at` into vf4.x (vf6.x = 1.0), moved
    /// to GPR `gpr`.
    fn closer_view_dot(c: &mut Cpu, at: u32, gpr: usize) {
        c.lqc2(3, at);
        c.lqc2(5, at);
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.qmfc2(gpr, 4);
    }
}

/// `void SSXAudioSystem_SpeechWipeoutRecover(SSXAudioSystem* sys, Rider*
/// rider)` (0x2a02d8): the "back on the board" speech after a wipeout.
/// Only with speech volume on, outside replays, before the finish
/// (progress < 0), for the followed rider when the rider is an AI or not a
/// local human, with the speech instance ready and the rider heard by a
/// listener view (within `gp+0x5e8 * 100`). The line depends on the
/// track's category (1: 0x2088, 2: 0x20a0, else 0x20a1); when its event
/// context is accepted it is posted through the hook with the rider's
/// speech mask and priority 1 (track group 5 or event mode 4) or 3.
pub fn ssx_audio_speech_wipeout_recover(c: &mut Cpu) {
    use wipeout_recover as k;
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S0, 0x60);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        k::IS_SPEECH_VOLUME_ON.call(c, 0x002a_0304)?;
        // true: out through 0x2a053c (one more cycle)
        if wipeout_recover_body(c)? {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x60);
        c.cycles += 8;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });

    /// Everything after the volume check (s1 = sys, s0 = rider): `Ok(true)`
    /// when it leaves through 0x2a053c.
    fn wipeout_recover_body(c: &mut Cpu) -> Result<bool, crate::call::Unwound> {
        use wipeout_recover as k;
        if c.g[V0][0] == 0 {
            return Ok(false);
        }
        c.cycles += 4;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, k::IS_REPLAY_PLAYING, 0x002a_0314)?;
        if c.g[V0][0] != 0 {
            return Ok(false);
        }
        // finished (progress >= 0): nothing to say
        c.cycles += 6;
        c.lwc1(1, r32(c, S0).wrapping_add(k::RIDER_PROGRESS));
        c.mtc1(0, 0);
        c.c_le_s(0, 1);
        c.g[V0][0] = 1;
        if !c.fcc() {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            return Ok(false);
        }
        // s2 = the rider is an AI or not a local human: then it must be
        // the followed rider
        c.cycles += 10;
        let obj = r32(c, S0).wrapping_add(k::RIDER_OBJ);
        c.g[V1][0] = Addr(obj).lw(c);
        c.g[A1][0] = sx(obj);
        c.g[S5][0] = c.g[A1][0];
        c.g[S2][0] = 0;
        wipeout_recover_obj_call(c, k::VT_IS_AI, 0x002a_0360)?;
        if c.g[V0][0] != 0 {
            c.g[S2][0] = 1;
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(k::RIDER_LOCAL)).lw(c);
            if c.g[V0][0] == 0 {
                c.g[S2][0] = 1;
            }
        }
        c.cycles += 2;
        c.g[A0][0] = c.g[S1][0];
        if c.g[S2][0] != 0 {
            c.cycles += 4;
            c.g[A1][0] = u64::MAX;
            call_ptr(c, k::FIND_RIDER, 0x002a_0384)?;
            if c.g[V0][0] != c.g[S0][0] {
                return Ok(false);
            }
        }
        // s2 = the speech system
        c.cycles += 6;
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(k::SYS_SPEECH));
        c.g[A1][0] = 0;
        c.g[S2][0] = c.g[A0][0];
        k::IS_INSTANCE_READY.call(c, 0x002a_039c)?;
        if c.g[V0][0] == 0 {
            return Ok(false);
        }
        // s3 = speech mask, s4 = priority
        c.cycles += 7;
        c.g[A0][0] = c.g[S0][0];
        k::GET_RIDER_SPEECH_MASK.call(c, 0x002a_03ac)?;
        c.g[S3][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        k::IS_TRACK_GROUP5.call(c, 0x002a_03b8)?;
        c.g[S4][0] = 1;
        if c.g[V0][0] == 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            k::IS_EVENT_MODE4.call(c, 0x002a_03c8)?;
            c.g[S4][0] = 1;
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[S4][0] = 3;
            }
        }
        // heard by a listener view?
        c.cycles += 11;
        c.g[V1][0] = Addr(r32(c, S0).wrapping_add(k::RIDER_OBJ)).lw(c);
        wipeout_recover_obj_call(c, k::VT_POSITION, 0x002a_03e8)?;
        c.lwc1(12, r32(c, GP).wrapping_add(k::GP_RADIUS));
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, k::FIND_VIEW_IN_RANGE, 0x002a_03f8)?;
        if (c.g[V0][0] as i64) < 0 {
            return Ok(false);
        }
        // the track's category picks the line
        c.cycles += 14;
        call_ptr(c, k::GET_BE_LIBRARY, 0x002a_0408)?;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, k::BE_GET_INTERFACE, 0x002a_0414)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, k::RACE_GET_TRACK, 0x002a_041c)?;
        c.g[A1][0] = Addr(r32(c, V0)).lw(c);
        c.g[A0][0] = c.g[S1][0];
        k::TRACK_TO_CATEGORY.call(c, 0x002a_0428)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = 2;
        if c.g[V1][0] == 1 {
            c.cycles += 13;
            if !wipeout_recover_context(c, k::LINE_CAT1, [0x002a_0440, 0x002a_0464])? {
                return Ok(true);
            }
            c.cycles += 3;
            c.g[A1][0] = 0;
            c.g[A2][0] = k::LINE_CAT1;
            c.cycles += 10;
            wipeout_recover_post(c, [0x002a_04c4, 0x002a_04dc])?;
            return Ok(false);
        }
        c.cycles += 2;
        if c.g[V1][0] == c.g[V0][0] {
            c.cycles += 13;
            if !wipeout_recover_context(c, k::LINE_CAT2, [0x002a_0488, 0x002a_04ac])? {
                return Ok(true);
            }
            c.cycles += 2;
            c.g[A1][0] = 0;
            c.g[A2][0] = k::LINE_CAT2;
            c.cycles += 10;
            wipeout_recover_post(c, [0x002a_04c4, 0x002a_04dc])?;
            return Ok(false);
        }
        c.cycles += 13;
        if !wipeout_recover_context(c, k::LINE_OTHER, [0x002a_04ec, 0x002a_0510])? {
            return Ok(true);
        }
        c.cycles += 9;
        c.g[A1][0] = 0;
        c.g[A2][0] = k::LINE_OTHER;
        wipeout_recover_post(c, [0x002a_0524, 0x002a_053c])?;
        Ok(true)
    }

    /// `jalr` to the method at `slot` of the rider's scripting object
    /// (`s5`, vtable in v1): `a0 = obj + delta`.
    fn wipeout_recover_obj_call(c: &mut Cpu, slot: u32, ret_to: u32) -> Flow {
        let vt = r32(c, V1);
        c.g[A0][0] = Addr(vt.wrapping_add(slot)).lh(c);
        c.g[V0][0] = Addr(vt.wrapping_add(slot + 4)).lw(c);
        let f = r32(c, V0);
        c.g[A0][0] = sx(r32(c, S5).wrapping_add(r32(c, A0)));
        call_ptr(c, f, ret_to)
    }

    /// `SetEventContext(speech, 0, line, rider, Rider_GetCharacter(rider),
    /// 0, 0.0, 0)`: whether it was accepted (a0 = 1 after).
    fn wipeout_recover_context(
        c: &mut Cpu,
        line: u64,
        rets: [u32; 2],
    ) -> Result<bool, crate::call::Unwound> {
        use wipeout_recover as k;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, k::RIDER_GET_CHARACTER, rets[0])?;
        c.mtc1(0, 12);
        c.g[A0][0] = c.g[S2][0];
        c.g[A3][0] = c.g[S0][0];
        c.g[T0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = line;
        c.g[T1][0] = 0;
        c.g[T2][0] = 0;
        k::SET_EVENT_CONTEXT.call(c, rets[1])?;
        let accepted = c.g[V0][0] != 0;
        c.g[A0][0] = 1;
        Ok(accepted)
    }

    /// Post the line (a0 = 1, a1 = 0, a2 = line already set): the hook
    /// `gp+0x173c` with `(SPCH_MakeId(1, 0, line), 2, mask, priority)`.
    fn wipeout_recover_post(c: &mut Cpu, rets: [u32; 2]) -> Flow {
        use wipeout_recover as k;
        call_ptr(c, k::SPCH_MAKE_ID, rets[0])?;
        c.g[V1][0] = Addr(r32(c, GP).wrapping_add(k::GP_POST_HOOK)).lw(c);
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S4][0];
        let hook = r32(c, V1);
        c.g[A1][0] = 2;
        call_ptr(c, hook, rets[1])
    }
}

// ---------------------------------------------------------------------------
// 0x002910e0 SSXAudioSystem_StartAmbienceLoop

/// `SSXAudioSystem.ambienceGain` (+0x5830, the gain word the loop plays
/// with) and `.ambienceStarted` (+0x5834).
pub(crate) const AMBIENCE_LOOP_GAIN: u32 = 0x5830;

pub(crate) const AMBIENCE_LOOP_STARTED: u32 = 0x5834;

/// The 8-byte key the ambience loop is pushed with (`.data` 0x4a3680).
pub(crate) const AMBIENCE_LOOP_KEY: u32 = 0x004a_3680;

/// `void SSXAudioSystem_StartAmbienceLoop(SSXAudioSystem* sys)` (0x2910e0):
/// once: push sample 0x20 of bank 0 on the bank monitor's queue (pan 100,
/// 90, 50, priority 127, the system's ambience gain word, the ambience
/// key), play it (`Audio_PlayPushedSound`) and mark it started.
pub fn ssx_audio_system_start_ambience_loop(c: &mut Cpu) {
    use crate::game::sound::soundsys::boost_empty_store_entry as ambience_store;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S4, 0x20);
        frame.save(c, S0, 0x60);
        c.g[S4][0] = c.g[A0][0]; // sys
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save_ra(c, 0x10);
        let sys = c.g[S4][0] as u32;
        c.g[V0][0] = c.lw(sys.wrapping_add(AMBIENCE_LOOP_STARTED));
        frame.restore(c, S0, 0x60);
        if c.g[V0][0] == 0 {
            c.cycles += 172;
            // New queue entry: count += 1, the 24-byte template record.
            c.g[S0][0] = c.lw(sys); // monitor
            let monitor = c.g[S0][0] as u32;
            c.g[T1][0] = 0x18;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0x8;
            c.g[V0][0] = c.lw(monitor.wrapping_add(0x1f4));
            c.g[S1][0] = sx(monitor.wrapping_add(0x1d8)); // queue
            let queue = c.g[S1][0] as u32;
            c.g[S2][0] = 0x1;
            c.g[S3][0] = 0x20; // sample
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            c.sw(monitor.wrapping_add(0x1f4), c.g[V0][0]);
            c.g[V1][0] = c.lw(monitor.wrapping_add(0x1f4));
            c.g[V0][0] = c.lw(queue.wrapping_add(0x20));
            c.mult(0, V1, V1, T1);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.ldl(2, monitor.wrapping_add(0x1e3));
            c.ldr(2, monitor.wrapping_add(0x1dc));
            c.ldl(4, monitor.wrapping_add(0x1eb));
            c.ldr(4, monitor.wrapping_add(0x1e4));
            c.ldl(7, monitor.wrapping_add(0x1f3));
            c.ldr(7, monitor.wrapping_add(0x1ec));
            let record = c.g[V1][0] as u32;
            c.sdl(2, record.wrapping_add(0x7));
            c.sdr(2, record);
            c.sdl(4, record.wrapping_add(0xf));
            c.sdr(4, record.wrapping_add(0x8));
            c.sdl(7, record.wrapping_add(0x17));
            c.sdr(7, record.wrapping_add(0x10));
            c.g[V0][0] = c.lw(monitor.wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx(monitor.wrapping_add(c.g[V0][0] as u32));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0);
            ambience_store(c, (V1, V0), 0x34, 2, 0, 0);
            c.g[A3][0] = c.lw(monitor.wrapping_add(0x1f4));
            c.g[A0][0] = c.lw(monitor.wrapping_add(0x1f4));
            c.g[T0][0] = c.lw(queue.wrapping_add(0x20));
            c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
            c.mult(0, A0, A0, T1);
            c.g[V0][0] = c.lw(queue.wrapping_add(0x38));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[T0][0] as u32));
            c.g[V1][0] = c.lb(c.g[A0][0] as u32);
            c.sw(c.g[A3][0] as u32, c.g[V1][0]);
            ambience_store(c, (V0, V1), 0x3c, 2, 0, 0);
            // memset(&queue.slots[count], 0, 8)
            c.g[A0][0] = c.lw(monitor.wrapping_add(0x1f4));
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
            c.g[A0][0] = sx(queue.wrapping_add(c.g[A0][0] as u32));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
            call_ptr(c, 0x0041_6210, 0x0029_11e8)?;

            // The entry's parameters.
            c.g[A1][0] = 0x64;
            c.g[A2][0] = 0x5a;
            c.g[A3][0] = 0x32;
            c.g[T0][0] = 0x7f;
            c.g[T1][0] = sx(sys.wrapping_add(AMBIENCE_LOOP_GAIN));
            c.g[T2][0] = 0x4a_0000;
            c.g[A0][0] = c.g[S4][0];
            ambience_store(c, (V1, V0), 0x60, 2, 0, 0);
            ambience_store(c, (V0, V1), 0x64, 2, 0, 0);
            ambience_store(c, (V1, V0), 0x68, 2, 0, 0);
            ambience_store(c, (V0, V1), 0x6c, 2, 0, 0);
            ambience_store(c, (V1, V0), 0x70, 4, 0x4, 0);
            ambience_store(c, (V0, V1), 0x70, 4, 0x0, 0x64);
            ambience_store(c, (V1, V0), 0x70, 4, 0x8, 0x5a);
            ambience_store(c, (V0, V1), 0x70, 4, 0xc, 0x32);
            ambience_store(c, (V1, V0), 0x74, 2, 0, 0);
            ambience_store(c, (V0, V1), 0x80, 2, 0, 0);
            ambience_store(c, (V1, V0), 0x84, 2, 0, 0);
            ambience_store(c, (V0, V1), 0x78, 2, 0, 0x7f);
            ambience_store(c, (V1, V0), 0x7c, 2, 0, 0x1);

            // sys->pushBank[count] = 0; sys->pushSample[count] = 0x20.
            c.g[A1][0] = c.lw(sys);
            c.g[V1][0] = c.lw(sys.wrapping_add(0x4));
            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw(c.g[V0][0] as u32, 0);
            c.g[V1][0] = c.lw(sys);
            c.g[A1][0] = c.lw(sys.wrapping_add(0x8));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
            c.sw(c.g[V0][0] as u32, c.g[S3][0]);
            // Through (*sys->sndRoot): gain[count] = &sys->ambienceGain,
            // table 0x7c[count] = 0, slots[count] = the ambience key.
            for (table, value) in [(0x214u32, c.g[T1][0]), (0x254, 0)] {
                c.g[V1][0] = c.lw(sys.wrapping_add(0x118));
                c.g[A1][0] = c.lw(c.g[V1][0] as u32);
                let mon = c.g[A1][0] as u32;
                c.g[V0][0] = c.lw(mon.wrapping_add(0x1f4));
                c.g[V1][0] = c.lw(mon.wrapping_add(table));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sw(c.g[V0][0] as u32, value);
            }
            c.g[V1][0] = c.lw(sys.wrapping_add(0x118));
            c.g[A1][0] = c.lw(c.g[V1][0] as u32);
            let mon = c.g[A1][0] as u32;
            c.g[A3][0] = sx(AMBIENCE_LOOP_KEY);
            let tmp = sp(c);
            c.ldl(2, AMBIENCE_LOOP_KEY.wrapping_add(0x7));
            c.ldr(2, AMBIENCE_LOOP_KEY);
            c.sdl(2, tmp.wrapping_add(0x7));
            c.sdr(2, tmp);
            c.g[V1][0] = sx(mon.wrapping_add(0x1d8));
            c.g[V0][0] = c.lw(mon.wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            let slot = c.g[V1][0] as u32;
            c.ldl(8, tmp.wrapping_add(0x7));
            c.ldr(8, tmp);
            c.sdl(8, slot.wrapping_add(0x47));
            c.sdr(8, slot.wrapping_add(0x40));
            Callee {
                addr: 0x0029_06b8,
                f: ee::s07::Audio_PlayPushedSound,
            }
            .call(c, 0x0029_13b4)?;
            c.sw(sys.wrapping_add(AMBIENCE_LOOP_STARTED), c.g[S2][0]);
            frame.restore(c, S0, 0x60);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SND_FireEvents (0x3b5f98)
// ---------------------------------------------------------------------------

/// The sound library's event-handler table (`D_0050A8E8`).
pub(crate) const SND_EVENT_TABLE: u32 = 0x0050_a8e8;

// ---------------------------------------------------------------------------
// CrowdAudio_UpdatePitchBend (0x2a68b0)
// ---------------------------------------------------------------------------

/// Sound handle of a bank-monitor instance (negative: not playing).
pub(crate) const CROWD_VOICE_HANDLE: u32 = 0x4;

/// CrowdAudio: index of the active pitch-bend curve, and its time left.
pub(crate) const CROWD_BEND_CURVE: u32 = 0x184;

pub(crate) const CROWD_BEND_TIME: u32 = 0x188;

/// gp_m2-relative pointer to the crowd sound table (`D_004A36F8`), 0x54-byte
/// entries; the bend curve fields start 0x1c into an entry.
pub(crate) const CROWD_TABLE_GP: u32 = 0x608;

pub(crate) const CROWD_ENTRY_SIZE: u32 = 0x54;

pub(crate) const CROWD_ENTRY_BEND: u32 = 0x1c;

/// Curve fields (relative to the bend start): ramp-down length, ramp-up
/// length, bend depth.
pub(crate) const BEND_FALL: u32 = 0x48;

pub(crate) const BEND_RISE: u32 = 0x4c;

pub(crate) const BEND_DEPTH: u32 = 0x50;

/// `void CrowdAudio_UpdatePitchBend(CrowdAudio* self, BankMonitorInstance*
/// voice, float dt)` (0x2a68b0): a voice that is not playing is reset;
/// otherwise, while a pitch bend is running, counts its time down by `dt`
/// and sets the voice's pitch bend (0x40 = centre) from the curve: rising
/// while the time left is under the rise length, then falling.
pub fn crowd_audio_update_pitch_bend(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let handle = Addr(r32(c, S1)).off(CROWD_VOICE_HANDLE).lw(c);
        c.g[V0][0] = handle;
        c.g[A1][0] = c.g[A0][0];
        if (handle as i64) < 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            Callee {
                addr: 0x002a_bb38,
                f: ee::s08::f_002abb38,
            }
            .call(c, 0x002a_68d8)?;
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 6;
            let this = Addr(r32(c, A1));
            let time = this.off(CROWD_BEND_TIME).read_fpu(c);
            c.f[1] = time.0;
            c.f[0] = 0;
            let idle = time.le(FpuF32::ZERO);
            abi::set_fcc(c, idle);
            frame.restore(c, S0, 0x20);
            if !idle {
                c.cycles += 13;
                let curve = this.off(CROWD_BEND_CURVE).lw(c);
                c.g[V1][0] = CROWD_ENTRY_SIZE as u64;
                let left = time - FpuF32(c.f[12]);
                c.f[12] = left.0;
                let table = Addr(c.g[GP][0] as u32).off(CROWD_TABLE_GP).lw(c) as u32;
                let offset =
                    abi::mult(c, curve as u32, CROWD_ENTRY_SIZE).wrapping_add(CROWD_ENTRY_BEND);
                this.off(CROWD_BEND_TIME).write_fpu(c, left);
                c.g[V0][0] = sx(offset);
                let bend = Addr(table.wrapping_add(offset));
                c.g[A0][0] = sx(bend.0);
                let rise = bend.off(BEND_RISE).read_fpu(c);
                c.f[1] = rise.0;
                let falling = abi::c_lt_s(c, rise, left);
                c.g[S0][0] = 0x40;
                let scaled = if falling {
                    // Past the rise: depth * (fall - (left - rise)) / fall.
                    c.cycles += 8;
                    let over = left - rise;
                    let fall = bend.off(BEND_FALL).read_fpu(c);
                    let depth = bend.off(BEND_DEPTH).read_fpu(c);
                    let remaining = fall - over;
                    c.f[1] = remaining.0;
                    c.f[2] = fall.0;
                    Some((depth * remaining).div(fall))
                } else {
                    c.cycles += 3;
                    if abi::c_lt_s(c, FpuF32::ZERO, left) {
                        // Rising: depth * left / rise.
                        c.cycles += 3;
                        let depth = bend.off(BEND_DEPTH).read_fpu(c);
                        Some((depth * left).div(rise))
                    } else {
                        None
                    }
                };
                if let Some(scaled) = scaled {
                    c.cycles += 3;
                    c.f[0] = scaled.0;
                    let whole = scaled.to_i32() as u32;
                    c.f[1] = whole;
                    c.g[V0][0] = sx(whole);
                    c.g[S0][0] = sx(whole.wrapping_add(0x40));
                }
                c.cycles += 8;
                call_ptr(c, 0x003b_58a0, 0x002a_6978)?; // SND_Lock
                c.g[A0][0] = Addr(r32(c, S1)).off(CROWD_VOICE_HANDLE).lw(c);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003b_c6b8, 0x002a_6984)?; // SND_SetPitchBend
                call_ptr(c, 0x003b_58d8, 0x002a_698c)?; // SND_Unlock
                frame.restore(c, S0, 0x20);
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;

    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;

    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fakes::LOG_UNWIND_AT;

    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::ai::tests::fake_check;
    pub(crate) use crate::game::ai::ai::tests::generic;

    pub(crate) use crate::game::ai::ai::tests::wf;
    pub(crate) use crate::game::ai::ai::tests::RIDER;
    pub(crate) use crate::game::ai::rider::tests::put_world_2 as put_world;
    pub(crate) use crate::game::be::beprepoststate::tests::SEEDS as SEEDS_9;
    pub(crate) use crate::game::bx::bxstring::tests::put_str;
    pub(crate) use crate::game::bx::bxstring::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::bx::bxstringctor::tests::base_3 as base_6;
    pub(crate) use crate::game::render::particle::tests::f;

    pub(crate) use crate::game::camera::cameracontroller::tests::put_vtable;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::belib_3 as belib_2;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::common;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::world_2;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::zero;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::SCRATCH;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::SEEDS_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_18 as base_11;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_4 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::call_machine;
    pub(crate) use crate::game::fe::feasyncfile::tests::check_all;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;

    pub(crate) use crate::game::fe::feasyncfile::tests::globals_4 as globals;

    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init_4;

    pub(crate) use crate::game::fe::feasyncfile::tests::pool;

    pub(crate) use crate::game::fe::feasyncfile::tests::slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::string_2 as string;

    pub(crate) use crate::game::fe::feasyncfile::tests::w_3 as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::world;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_4 as OBJ2_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ_3;

    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_14;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS_15;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_12;
    pub(crate) use crate::game::fe::feasyncfile::tests::VT;

    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_2 as WORLD;

    pub(crate) use crate::game::fe::fememcard::tests::put_slot;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_12;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all_2;

    pub(crate) use crate::game::fe::festateloadscreen::tests::r32_seed;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4;
    pub(crate) use crate::game::fe::festaterewards::tests::base_9 as base_10;

    pub(crate) use crate::game::fe::festaterewards::tests::slot_2 as slot_3;
    pub(crate) use crate::game::fe::festaterewards::tests::strict;

    pub(crate) use crate::game::fe::festaterewards::tests::W as W_5;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::belib;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::w;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::SEEDS as SEEDS_4;
    pub(crate) use crate::game::fe::ovtemplatedialog::tests::ZERO_STUB;

    pub(crate) use crate::game::main::gamemode::tests::SEEDS as SEEDS_3;
    pub(crate) use crate::game::main::gamestate::tests::base_3;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_13;
    pub(crate) use crate::game::main::ssxapp::tests::strict_check;
    pub(crate) use crate::game::object::effectlink::tests::alloc;
    pub(crate) use crate::game::object::railmodifier::tests::rng as rng_2;
    pub(crate) use crate::game::render::bezierman::tests::b;
    pub(crate) use crate::game::render::graphicsman::tests::same;
    pub(crate) use crate::game::render::particle::tests::common as common_2;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_8;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::gp_at;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_11;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS_6;

    pub(crate) use crate::game::sound::bankmanager::tests::race_type_test;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::track_id;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS;
    pub(crate) use crate::game::sound::bankmanager::tests::SYS;
    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4 as SEEDS_5;
    pub(crate) use crate::game::worldpainter::quadtree::tests::alloc_machine;

    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::dummy_world;

    use ssx3_ee as ee;

    #[test]
    fn ssx_audio_system_get_surface_bank_matches() {
        check(
            ssx3_ee::s07::f_002917b8__generated,
            ssx_audio_system_get_surface_bank,
            300,
            &|c, seed| {
                let mut r = Rng(0x6789_abcd ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = match seed % 5 {
                    0 => 0,
                    1 => 3,
                    2 => 4,
                    3 => 0xffff_ffff,
                    _ => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                };
            },
        );
    }

    #[test]
    fn ssx_audio_system_get_current_speech_category_matches() {
        check(
            ssx3_ee::s08::f_0029f088__generated,
            ssx_audio_system_get_current_speech_category,
            300,
            &|c, seed| {
                let mut r = Rng(0x789a_bcde ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                c.bus.write32(DATA + SPEECH_CATEGORY, r.next_u32());
            },
        );
    }

    #[test]
    fn ssx_audio_system_clear_pending_speech_matches() {
        check(
            ssx3_ee::s08::f_002a4550__generated,
            ssx_audio_system_clear_pending_speech,
            300,
            &|c, seed| {
                let mut r = Rng(0x89ab_cdef ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for off in PENDING_SPEECH_FIELDS {
                    c.bus.write32(DATA + off, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn ssx_audio_system_get_channel_volume_matches() {
        check(
            ssx3_ee::s07::f_00287920,
            ssx_audio_system_get_channel_volume,
            100,
            &|c, seed| {
                let mut r = Rng(0x1111_0007 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let idx = (seed % 8) as u64;
                c.g[A1][0] = idx;
                c.bus
                    .write32(DATA + (idx as u32) * 0x10 + 0x62bc, r.edge_f32());
            },
        );
    }

    #[test]
    fn crowd_audio_decode_source_handle_matches() {
        check(
            ssx3_ee::s08::f_002a72c0,
            crowd_audio_decode_source_handle,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_000c ^ seed as u64);
                scramble(c, &mut r);
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.g[A2][0] = DATA as u64;
                c.g[A3][0] = (DATA + 4) as u64;
                c.bus.write32(DATA, r.next_u32());
                c.bus.write32(DATA + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn ini_parser_get_char_matches() {
        check(
            ssx3_ee::s08::f_002a78e0,
            ini_parser_get_char,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_000d ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x8, DATA + 0x100);
                let b = if seed % 2 == 0 {
                    (r.next_u32() | 0x80) as u8
                } else {
                    r.next_u32() as u8
                };
                c.bus.write8(DATA + 0x100, b);
            },
        );
    }

    #[test]
    fn ps2_sound_manager_ctor_matches() {
        check(
            ssx3_ee::s08::f_002a8c20,
            ps2_sound_manager_ctor,
            50,
            &|c, seed| {
                let mut r = Rng(0x1111_000e ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..210 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn ps2_sound_manager_pop_free_spu_block_matches() {
        check_lohi(
            ssx3_ee::s08::f_002a92b8,
            ps2_sound_manager_pop_free_spu_block,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_000f ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let head: u32 = match seed % 4 {
                    0 => 0,
                    1 => 1,
                    2 => 0x8000_0000, // negative after lw sign extension
                    _ => 2,
                };
                c.bus.write32(DATA + 0x310, head);
                let block = DATA.wrapping_add(head.wrapping_mul(0x18).wrapping_add(0xc));
                c.bus.write32(block + 0x14, r.next_u32());
            },
        );
    }

    #[test]
    fn audio_eval_curve5_matches() {
        check_events(
            ssx3_ee::s07::f_00290b58,
            audio_eval_curve5,
            120,
            &|c, seed| {
                let mut r = Rng(0x290b_5800 ^ seed as u64);
                scramble(c, &mut r);
                let curve = DATA;
                c.g[A0][0] = curve as u64;
                for i in 0..0x40 / 4 {
                    c.bus.write32(curve + 4 * i, r.edge_f32());
                }
                c.f[12] = r.edge_f32();
                let mode = schedule(c, &mut r, seed, 0x0029_0b58);
                if mode != Events::Off {
                    c.next_event = c.cycles;
                }
                mode
            },
        );
    }

    #[test]
    fn ssx_audio_system_set_player_trick_score_matches() {
        check(
            ee::s07::f_00290f58__generated,
            ssx_audio_system_set_player_trick_score,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x290f58, 6);
                let app = DATA;
                let world = DATA + 0x100;
                let man = DATA + 0x200;
                let racman = DATA + 0x300;
                let view = DATA + 0x400;
                put_world(c, world, man, racman);
                c.bus.write32(man + 0x28, 0); // no replay
                c.bus.write32(man + 0x84, view);
                c.bus
                    .write32(view + 0x10, if seed % 4 == 1 { 2 } else { 1 });
                // An empty player table: FindRider answers 0 for both players.
                c.bus.write32(racman + 0x40, 0);
                c.bus.write32(racman + 0x78, 0);
                c.bus.write32(racman + 0x7c, 0);
                let rider = [0u32, 1, 2, 0xffff_ffff][(seed % 4) as usize];
                c.f[12] = r.f32(10.0);
                c.g[A0][0] = sx(app);
                c.g[A1][0] = sx(rider);
            },
        );
    }

    #[test]
    fn ssx_audio_system_is_track_id_group5_matches() {
        check(
            ee::s08::f_002a3fd8__generated,
            ssx_audio_system_is_track_id_group5,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x002a_3fd8, seed);
                scramble(c, &mut r);
                track_id(c, &mut r, seed, 0x11, 5);
            },
        );
    }

    #[test]
    fn ssx_audio_system_reset_wipeout_sfx_matches() {
        check(
            ee::s08::f_0029e578__generated,
            ssx_audio_system_reset_wipeout_sfx,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0029_e578, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x1000);
                let voice = (seed % 8) as u32;
                c.bus.write32(DATA + 0x1000 + 0x870, voice);
                c.bus.write32(DATA + voice * 4 + 0x6080, r.next_u32());
            },
        );
    }

    #[test]
    fn crowd_audio_instance_start_anticipate_matches() {
        check(
            ee::s08::f_002a67f0__generated,
            crowd_audio_instance_start_anticipate,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x002a_67f0, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.bus
                    .write32(DATA + 0x180, if seed % 3 == 0 { 0 } else { 1 });
            },
        );
    }

    #[test]
    fn ssx_audio_system_set_rider_air_flag_matches() {
        check(
            ee::s07::f_00294880__generated,
            ssx_audio_system_set_rider_air_flag,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x29_4880, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (r.next_u32() % 12) as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn ssx_audio_system_tick_listener_matches() {
        check(
            ee::s08::f_00294f48__generated,
            ssx_audio_system_tick_listener,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x294f48, 1);
                let this = SCRATCH;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                // `*(this + 0xc)` is the listener vtable; slot `+8/+0xc` is the
                // tick method, which lands in the fake-call hook.
                let vt = fakes::vtable(c, &mut r, &[8]);
                c.bus.write32(this + 0xc, vt);
            },
        );
    }

    #[test]
    fn ssx_audio_system_on_script_rider_speech2_matches() {
        check(
            ee::s08::f_002a1b58__generated,
            ssx_audio_system_on_script_rider_speech2,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a1b58, 2);
                c.g[A0][0] = SCRATCH as u64;
                c.g[A1][0] = (seed % 3) as u64; // 2 = a rider speech request
                c.g[A2][0] = (SCRATCH + 0x300) as u64;
                // Speech channel 4's volume: a plain 0.5 keeps the volume
                // query on its "not muted" early-out.
                c.bus.write32(SCRATCH + 0x62fc, 0.5f32.to_bits());
                c.bus.write32(SCRATCH + 0x300, r.next_u32());
            },
        );
    }

    #[test]
    fn ssx_audio_system_reset_race_result_state_matches() {
        check(
            ee::s08::f_002a4590__generated,
            ssx_audio_system_reset_race_result_state,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a4590, 0);
                c.g[A0][0] = SCRATCH as u64;
                c.g[A1][0] = (seed % 2) as u64;
                crate::difftest::fill(c, &mut r, SCRATCH + 0x57e0, 0x10);
            },
        );
    }

    /// A crowd-audio object at `SCRATCH` with four voices at `+0x18`. Every
    /// voice has no pending sound (`+0x174 == -1`) and no sound to stop
    /// (`+0xd4 == 0`), so the instance helpers return immediately. Both the
    /// play form (index in `a2`) and the stop form (index in `a1`) get a small
    /// in-range index.
    pub(crate) fn crowd_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 4);
        c.g[A0][0] = SCRATCH as u64;
        c.g[A1][0] = (seed % 4) as u64;
        c.g[A2][0] = ((seed / 4) % 4) as u64;
        let voices = SCRATCH + 0x1000;
        c.bus.write32(SCRATCH + 0x18, voices);
        for i in 0..4u32 {
            let voice = voices + 0x190 * i;
            c.bus.write32(voice + 0x174, 0xffff_ffff);
            c.bus.write32(voice + 0xd4, 0);
            c.bus.write32(voice + 0x188, r.next_u32() & 0x3f80_0000);
        }
    }

    #[test]
    fn crowd_audio_play_trick_matches() {
        check(
            ee::s08::f_002a73a8__generated,
            crowd_audio_play_trick,
            100,
            &|c, seed| crowd_setup(c, seed, 0x2a73a8),
        );
    }

    #[test]
    fn crowd_audio_play_fall_matches() {
        check(
            ee::s08::f_002a73d8__generated,
            crowd_audio_play_fall,
            100,
            &|c, seed| crowd_setup(c, seed, 0x2a73d8),
        );
    }

    #[test]
    fn crowd_audio_stop_anticipate_matches() {
        check(
            ee::s08::f_002a7438__generated,
            crowd_audio_stop_anticipate,
            100,
            &|c, seed| crowd_setup(c, seed, 0x2a7438),
        );
    }

    // ------------------------------------------------------- sound / track slots

    #[test]
    fn ps2_sound_manager_v07_matches() {
        check(
            ee::s08::f_002a9288__generated,
            ps2_sound_manager_v07,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a9288, 1);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x30c, r.next_u32());
            },
        );
    }

    #[test]
    fn audio_is_pa_speech_on_matches() {
        check(
            ssx3_ee::s08::f_0029f128__generated,
            audio_is_pa_speech_on,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x29f128, 0);
                let sys = DATA + 0x1000;
                // Channel 3 volume lives at +0x30 + 0x62bc.
                c.bus.write32(
                    sys + 0x62ec,
                    [0u32, 0x3f80_0000, 0x8000_0000, 0x4120_0000][seed as usize % 4],
                );
                let _ = r.next_u32();
                c.g[A0][0] = sx(sys);
            },
        );
    }

    #[test]
    fn audio_is_arcade_speech_on_matches() {
        check(
            ssx3_ee::s08::f_0029f160__generated,
            audio_is_arcade_speech_on,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x29f160, 0);
                let sys = DATA + 0x1000;
                // Channel 0xa volume lives at +0xa0 + 0x62bc.
                c.bus.write32(
                    sys + 0x635c,
                    [0u32, 0xbf80_0000, 0x0000_0001, 0x7f80_0000][seed as usize % 4],
                );
                let _ = r.next_u32();
                c.g[A0][0] = sx(sys);
            },
        );
    }

    #[test]
    fn crowd_audio_init_sample_list_matches() {
        check(
            ssx3_ee::s08::f_002a4ba8__generated,
            crowd_audio_init_sample_list,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a4ba8, 0);
                let audio = DATA + 0x100;
                fill(c, &mut r, audio, 0x20);
                c.g[A0][0] = sx(audio);
            },
        );
    }

    // ------------------------------------------------------------- audio

    #[test]
    fn ps2_sound_manager_create_matches() {
        check(
            ee::s08::f_002a8fa0__generated,
            ps2_sound_manager_create,
            40,
            &|c, seed| {
                alloc_machine(c, seed, 0x2a8f_a0);
            },
        );
    }

    #[test]
    fn ssx_audio_system_on_bonus_trick_matches() {
        check(
            ee::s08::f_0029b7e0__generated,
            ssx_audio_system_on_bonus_trick,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x29b7_e0, 0);
                let audio = DATA + 0x400;
                let rider = DATA + 0x600;
                zero(c, audio, 0x100);
                zero(c, rider, 0x900);
                c.g[A0][0] = sx(audio);
                c.g[A1][0] = sx(rider);
                c.bus
                    .write32(rider + 0x874, if seed % 3 == 0 { 0 } else { 1 });
                c.bus
                    .write32(rider + 0x87c, if seed % 2 == 0 { 0 } else { 1 });
            },
        );
    }

    #[test]
    fn ssx_audio_system_is_event_mode4_matches() {
        check(
            ee::s08::f_002a4040__generated,
            ssx_audio_system_is_event_mode4,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a40_40, 0);
                let idx = seed % 4;
                zero(c, DATA + 0x400, 0x400);
                c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x400);
                c.bus.write32(GAME_GP.wrapping_sub(0x1efc), idx);
                c.bus.write32(0x0053_08a8 + 4 * idx, DATA + 0x600);
                c.bus.write32(DATA + 0x600 + 4, DATA + 0x700);
                put_slot(c, &mut r, DATA + 0x700, 8);
                c.bus.write8(0x0053_5c10, (seed % 7) as u8);
            },
        );
    }

    #[test]
    fn ssx_audio_system_is_race_type4_matches() {
        check(
            ee::s08::f_002a4290__generated,
            ssx_audio_system_is_race_type4,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a42_90, 0);
                let idx = seed % 4;
                zero(c, DATA + 0x400, 0x400);
                c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x400);
                c.bus.write32(GAME_GP.wrapping_sub(0x1efc), idx);
                c.bus.write32(0x0053_08a8 + 4 * idx, DATA + 0x600);
                c.bus.write32(DATA + 0x600 + 4, DATA + 0x700);
                put_slot(c, &mut r, DATA + 0x700, 8);
                c.bus.write8(0x0053_5c12, (seed % 7) as u8);
            },
        );
    }

    #[test]
    fn crowd_audio_instance_stop_anticipate_matches() {
        check(
            ssx3_ee::s08::f_002a6808__generated,
            crowd_audio_instance_stop_anticipate,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x2a68_08, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
                w(c, DATA + 0x180, if seed % 2 == 0 { 2 } else { 0 });
            },
        );
    }

    #[test]
    fn ssx_audio_system_unload_grunt_banks_matches() {
        check(
            ee::s08::f_0029f5e0__generated,
            ssx_audio_system_unload_grunt_banks,
            SEEDS_5,
            &|c, seed| {
                let _ = machine(c, seed, 0x29f5e0, 0);
                let self_at = DATA + 0x200;
                let bank_man = DATA + 0x300;
                let banks = DATA + 0x400;
                c.g[A0][0] = self_at as u64;
                c.bus.write32(self_at + 0x118, bank_man);
                c.bus.write32(bank_man, banks);
                c.bus.write32(banks + 0xacc, (seed % 3) as u32);
            },
        );
    }

    #[test]
    fn ssx_audio_system_stop_speech_on_reset_matches() {
        check(
            ee::s08::f_002a4b68__generated,
            ssx_audio_system_stop_speech_on_reset,
            SEEDS_5,
            &|c, seed| {
                let _ = machine(c, seed, 0x2a4b68, 0);
                let self_at = DATA + 0x200;
                let player = self_at + 0x118;
                c.g[A0][0] = self_at as u64;
                let tagged = seed % 2 == 0;
                c.bus.write32(player + 0x418, tagged as u32);
                if tagged {
                    let song = DATA + 0x400;
                    c.bus.write32(player + 0x408, song);
                    c.bus.write32(song + 0x58, 0x191);
                }
            },
        );
    }

    #[test]
    fn crowd_audio_delete_sample_list_matches() {
        check(
            ee::s08::f_002a4be0__generated,
            crowd_audio_delete_sample_list,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_4be0, 0);
                let obj = DATA + 0x1000;
                let count = match seed % 6 {
                    0 => -1i32,
                    1 => 0,
                    n => n as i32 - 2,
                };
                c.bus.write32(obj, count as u32);
                for i in 0..5u32 {
                    c.bus.write32(obj + 4 + 4 * i, 0);
                }
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = (seed % 4 == 0) as u64;
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn ssx_audio_system_pause_sfx_matches() {
        check(
            ee::s08::f_0029ce28__generated,
            ssx_audio_system_pause_sfx,
            60,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x29ce_28, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                let monitor = DATA + 0x2000;
                c.bus.write32(this + 0x118, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000, monitor);
                c.bus
                    .write32(this + 0x5fb0, if seed % 3 == 0 { 1 } else { 0 });
                c.bus.write32(monitor + 0x26c, r.next_u32());
                c.bus.write32(monitor + 0x1d8, r.next_u32());
            },
        );
    }

    #[test]
    fn ssx_audio_system_on_script_rider_speech01_matches() {
        check(
            ee::s08::f_002a1b88__generated,
            ssx_audio_system_on_script_rider_speech01,
            60,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x2a1b_88, 2);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(match seed % 4 {
                    0 => 0,
                    1 => 1,
                    _ => 2 + seed % 0x10,
                });
                c.g[A2][0] = sx(r.next_u32() & 0xffff);
                c.g[A3][0] = sx(r.next_u32() & 0xffff);
                c.bus.write32(this + 0x118, DATA + 0x1000);
            },
        );
    }

    #[test]
    fn ssx_audio_system_is_race_type4_or5_matches() {
        check(
            ee::s08::f_002a4248__generated,
            ssx_audio_system_is_race_type4_or5,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a42_48, 2);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(seed % 2);
                // The BE library singleton (a zeroed dummy) and its interface
                // slot, whose method is a fake.
                c.bus.write32(GAME_GP - 0x1f04, DATA + 0x600);
                c.bus.write32(GAME_GP - 0x1efc, 0);
                let iface = DATA + 0x400;
                c.bus.write32(0x0053_08a8, iface);
                c.bus.write32(iface + 4, DATA + 0x800);
                slot(c, &mut r, DATA + 0x800, 8, FAKE + 0x10);
                c.bus.write8(
                    0x0053_5c12,
                    match seed % 3 {
                        0 => 4,
                        1 => 5,
                        _ => 7,
                    },
                );
            },
        );
    }

    // ------------------------------------------------------------------
    // Audio.

    #[test]
    fn ssx_audio_system_play_dj_song_intro_matches() {
        check(
            ee::s08::f_002a3170__generated,
            ssx_audio_system_play_dj_song_intro,
            SEEDS_6,
            &|c, seed| {
                let _ = machine(c, seed, 0x2a31_70, 2);
                let this = DATA + 0x100;
                clear(c, this, 0x1620);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn ssx_audio_system_is_race_type6_to8_matches() {
        race_type_test(
            ee::s08::f_002a4318__generated,
            ssx_audio_system_is_race_type6_to8,
            0x2a43_18,
        );
    }

    #[test]
    fn ssx_audio_system_is_race_type9_to11_matches() {
        race_type_test(
            ee::s08::f_002a4368__generated,
            ssx_audio_system_is_race_type9_to11,
            0x2a43_68,
        );
    }

    #[test]
    fn ps2_sound_manager_shutdown_matches() {
        check(
            ee::s08::f_002a8f50__generated,
            ps2_sound_manager_shutdown,
            SEEDS_6,
            &|c, seed| {
                let _ = machine(c, seed, 0x2a8f_50, 2);
                let this = DATA + 0x100;
                clear(c, this, 0x40);
                c.g[A0][0] = sx(this);
                c.bus
                    .write32(this + 4, if seed % 2 == 0 { 0 } else { DATA + 0x1_2000 });
            },
        );
    }

    #[test]
    fn ssx_audio_system_is_dj_speech_on_matches() {
        check(
            ee::s08::f_0029f0d8__generated,
            ssx_audio_system_is_dj_speech_on,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x29f0_d8, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x100);
                let module = dummy_world(c, DATA + 0x2000);
                match seed % 3 {
                    0 => c.bus.write32(module + 0x28, 0),
                    1 => {
                        c.bus.write32(module + 0x28, DATA + 0x300);
                        c.bus.write32(DATA + 0x300, 5);
                    }
                    _ => {
                        c.bus.write32(module + 0x28, DATA + 0x304);
                        c.bus.write32(DATA + 0x304, 20);
                    }
                }
                c.g[A0][0] = sx(this);
                let vol = if seed % 4 == 0 { 0 } else { r.edge_f32() };
                c.bus.write32(this + 0x62bc + 2 * 16, vol);
            },
        );
    }

    #[test]
    fn ssx_audio_system_start_section_speech_timer_matches() {
        check(
            ee::s08::f_002a49e8__generated,
            ssx_audio_system_start_section_speech_timer,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2a49_e8, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let mon = DATA + 0x2000;
                clear(c, this, 0x40);
                clear(c, mon, 0x800);
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x118, mon);
                put_words(c, 0x004a_36f0, &[0xffff_0000, 0x002a_4a68]);
            },
        );
    }

    #[test]
    fn crowd_audio_destroy_instance_matches() {
        check(
            ee::s08::f_002a4e38__generated,
            crowd_audio_destroy_instance,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a4e_38, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                let mon = DATA + 0x6000;
                clear(c, this, 0x200);
                clear(c, mon, 0x200);
                c.bus.write32(mon, mon);
                c.bus.write32(gp_at(0x608), mon);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
                let src = if seed % 3 == 0 { u32::MAX } else { seed % 4 };
                c.bus.write32(this + 0x174, src);
                c.bus.write32(this + 0x188, r.edge_f32());
            },
        );
    }

    #[test]
    fn ssx_audio_system_stop_ambience_loop_matches() {
        check(
            ee::s07::f_002913d8__generated,
            ssx_audio_system_stop_ambience_loop,
            SEEDS_7,
            &|c, seed| {
                let _ = common(c, seed, 0x2913d8, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x6000);
                let sys = DATA + 0x2000;
                let vt = DATA + 0x3000;
                clear(c, sys, 0x1000);
                clear(c, vt, 0x200);
                c.bus.write32(this + 0x118, sys);
                c.bus.write32(sys, DATA + 0x4000);
                c.bus.write32(sys + 0xab0, vt);
                c.bus.write16(vt + 0x18, 0x1d8);
                c.bus.write32(vt + 0x1c, FAKE);
                put_words(c, 0x004a_3680, &[0x1234_5678, 0x9abc_def0]);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn ssx_audio_system_stop_loop_sfx_matches() {
        check(
            ee::s08::f_0029d678__generated,
            ssx_audio_system_stop_loop_sfx,
            SEEDS_7,
            &|c, seed| {
                let _ = common(c, seed, 0x29d678, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x6100);
                let sys = DATA + 0x3000;
                clear(c, sys, 0x200);
                c.bus.write32(sys, sys);
                c.bus.write32(this + 0x118, sys);
                c.bus
                    .write32(this + 0x5fc4, if seed % 3 == 0 { 0 } else { 1 });
                c.bus
                    .write32(this + 0x5fc8, if seed % 4 == 0 { u32::MAX } else { 3 });
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn ssx_audio_system_has_track_changed_matches() {
        check(
            ee::s08::f_002a4718__generated,
            ssx_audio_system_has_track_changed,
            SEEDS_7,
            &|c, seed| {
                let _ = common(c, seed, 0x2a4718, 3);
                let this = DATA + 0x100;
                clear(c, this, 0x6000);
                let iface = DATA + 0x2000;
                clear(c, iface, 0x100);
                let vt = DATA + 0x3000;
                clear(c, vt, 0x100);
                c.bus.write32(gp_at(-0x1efc), 0);
                c.bus.write32(0x0053_08a8, iface);
                c.bus.write32(iface + 4, vt);
                c.bus.write16(vt + 8, 0x10);
                c.bus.write32(vt + 0xc, FAKE);
                c.bus
                    .write32(this + 0x5814, [0x17u32, 0, 5, 0x17][(seed % 4) as usize]);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn crowd_audio_pick_random_sample_matches() {
        check_lohi(
            ee::s08::f_002a4ca0__generated,
            crowd_audio_pick_random_sample,
            SEEDS_7,
            &|c, seed| {
                let mut r = common(c, seed, 0x2a4ca0, 1);
                let list = DATA + 0x100;
                clear(c, list, 0x40);
                let n = (seed % 5) as u32;
                c.bus.write32(list, n);
                for i in 0..n {
                    c.bus.write32(list + 4 + 4 * i, r.next_u32());
                }
                c.g[A0][0] = sx(list);
            },
        );
    }

    // ------------------------------------------------------------------
    // Sound.

    #[test]
    fn ini_parser_skip_comment_matches() {
        check(
            ee::s08::f_002a79c8__generated,
            ini_parser_skip_comment,
            SEEDS_8,
            &|c, seed| {
                let _ = common_2(c, seed, 0x2a79c8, 1);
                let text = DATA + 0x1000;
                clear(c, text, 0x40);
                match seed % 4 {
                    0 => {}
                    1 => {
                        c.bus.write8(text, b'#');
                        c.bus.write8(text + 1, b'a');
                        c.bus.write8(text + 2, b'\n');
                    }
                    2 => {
                        c.bus.write8(text, b';');
                        c.bus.write8(text + 1, b'b');
                        c.bus.write8(text + 2, b'\r');
                    }
                    _ => {
                        c.bus.write8(text, b'x');
                    }
                }
                c.bus.write32(DATA + 8, text);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn ps2_sound_manager_free_spu_block_matches() {
        check(
            ee::s08::f_002a92f8__generated,
            ps2_sound_manager_free_spu_block,
            SEEDS_8,
            &|c, seed| {
                let mut r = common_2(c, seed, 0x2a92f8, 1);
                clear(c, DATA, 0x100);
                clear(c, DATA + 0x200, 0x40);
                c.bus.write32(DATA + 0x200, r.next_u32());
                c.bus.write32(DATA + 0x208, r.next_u32());
                c.bus.write32(DATA + 0x30c, r.next_u32());
                c.bus.write32(DATA + 0x310, r.next_u32());
                c.bus
                    .write32(DATA + 0x31c, if seed % 3 == 0 { 0 } else { r.next_u32() });
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x200);
            },
        );
    }

    #[test]
    fn ps2_sound_manager_set_spu_mem_base_matches() {
        check_lohi(
            ee::s08::f_002a9250__generated,
            ps2_sound_manager_set_spu_mem_base,
            200,
            &|c, seed| {
                let mut r = rng_2(0x002a_9250, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 4) as u64;
                // The pool entries at +0xc, stride 0x18: unk4 (the new SPU
                // base) and unk8 feed the setter; keep them small so its
                // range check takes the short path.
                for i in 0..4u32 {
                    c.bus.write32(DATA + 0xc + i * 0x18 + 4, (seed + i) % 0x200);
                    c.bus
                        .write32(DATA + 0xc + i * 0x18 + 8, r.next_u32() & 0xffff);
                }
                // SND_Lock / SND_Unlock call their hook through these
                // pointers; point both at a leaf that only returns.
                c.bus.write32(0x0044_c46c, 0x0036_0768);
                c.bus.write32(0x0044_c470, 0x0036_0768);
            },
        );
    }

    #[test]
    fn ssx_audio_system_kill_rider_timers_matches() {
        check(
            ee::s07::f_00292ae8__generated,
            ssx_audio_system_kill_rider_timers,
            24,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0029_2ae8, 2);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 7;
                w(c, DATA + 0x118, DATA + 0x100);
                w(c, DATA + 0x100, DATA + 0x200);
                let vt = vtable(c, &mut r, &[0x20]);
                w(c, DATA + 0x200 + 0xab0, vt);
            },
        );
    }

    #[test]
    fn ssx_audio_system_is_track_group2_event_matches() {
        check(
            ee::s08::f_002a4168__generated,
            ssx_audio_system_is_track_group2_event,
            64,
            &|c, seed| {
                machine(c, seed, 0x002a_4168, 4);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 12) as u64;
                b(c, 0x0053_5c10, (seed % 4) as u8);
            },
        );
    }

    #[test]
    fn ssx_audio_system_is_track_group3_event_matches() {
        check(
            ee::s08::f_002a41d8__generated,
            ssx_audio_system_is_track_group3_event,
            64,
            &|c, seed| {
                machine(c, seed, 0x002a_41d8, 4);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 12) as u64;
                b(c, 0x0053_5c10, (seed % 4) as u8);
            },
        );
    }

    #[test]
    fn ps2_sound_manager_dtor_matches() {
        check(
            ee::s08::f_002a8c50__generated,
            ps2_sound_manager_dtor,
            SEEDS_9,
            &|c, seed| {
                machine(c, seed, 0x002a_8c50, 2);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 3) as u64;
                w(c, DATA + 4, if seed % 2 == 0 { 0 } else { DATA + 0x800 });
            },
        );
    }

    #[test]
    fn ssx_audio_system_resume_sfx_matches() {
        check(
            ee::s08::f_0029ce70__generated,
            ssx_audio_system_resume_sfx,
            8,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0029_ce70, 3);
                be_library_2(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.bus
                    .write32(DATA + 0x5fb0, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(DATA + 0x118, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000 + 0x26c, r.next_u32());
                c.bus.write32(GAME_GP + 0x21e4, DATA + 0x3000);
            },
        );
    }

    #[test]
    fn crowd_audio_remove_source_matches() {
        check(
            ee::s08::f_002a72d8__generated,
            crowd_audio_remove_source,
            12,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002a_72d8, 2);
                be_library_2(c);
                world(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = if seed % 3 == 0 {
                    0xffff_ffffu64
                } else {
                    (r.next_u32() & 0x7fff) as u64
                };
                c.bus
                    .write32(DATA + 0x18, if seed % 4 == 0 { 0 } else { DATA + 0x2000 });
                c.f[12] = r.next_u32();
            },
        );
    }

    #[test]
    fn crowd_audio_set_source_param_matches() {
        check(
            ee::s08::f_002a7340__generated,
            crowd_audio_set_source_param,
            12,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002a_7340, 2);
                be_library_2(c);
                world(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = if seed % 3 == 0 {
                    0xffff_ffffu64
                } else {
                    (r.next_u32() & 0x7fff) as u64
                };
                c.g[A2][0] = (r.next_u32() % 8) as u64;
                c.bus
                    .write32(DATA + 0x18, if seed % 4 == 0 { 0 } else { DATA + 0x2000 });
                c.f[12] = r.next_u32();
            },
        );
    }

    #[test]
    fn ini_parser_get_float_matches() {
        check(
            ee::s08::f_002a8070__generated,
            ini_parser_get_float,
            8,
            &|c, seed| {
                machine(c, seed, 0x002a_8070, 2);
                be_library_2(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (if seed % 2 == 0 {
                    DATA + 0x1200
                } else {
                    DATA + 0x1100
                }) as u64;
                c.g[A2][0] = (DATA + 0x1300) as u64;
                c.bus.write32(DATA + 0x1200, 0x79); // "y\0"
                c.bus.write32(DATA + 0x1100, 0); // empty name
                c.bus.write32(DATA + 0x8, DATA + 0x1400); // empty input buffer
                c.bus.write32(DATA + 0x1400, 0);
            },
        );
    }

    #[test]
    fn ini_parser_get_int_matches() {
        check(
            ee::s08::f_002a80d8__generated,
            ini_parser_get_int,
            8,
            &|c, seed| {
                machine(c, seed, 0x002a_80d8, 2);
                be_library_2(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (if seed % 2 == 0 {
                    DATA + 0x1200
                } else {
                    DATA + 0x1100
                }) as u64;
                c.g[A2][0] = (DATA + 0x1300) as u64;
                c.bus.write32(DATA + 0x1200, 0x79);
                c.bus.write32(DATA + 0x1100, 0);
                c.bus.write32(DATA + 0x8, DATA + 0x1400);
                c.bus.write32(DATA + 0x1400, 0);
            },
        );
    }

    /// The shipped surface sound-type table at [`SURFACE_SOUND_TABLE`].
    pub(crate) const SURFACE_SOUND_HANDLERS: [u32; 21] = [
        0x0029_17a0,
        0x0029_17a0,
        0x0029_1770,
        0x0029_1768,
        0x0029_1768,
        0x0029_1778,
        0x0029_17a0,
        0x0029_17a0,
        0x0029_1780,
        0x0029_1780,
        0x0029_1788,
        0x0029_1790,
        0x0029_1798,
        0x0029_17a0,
        0x0029_1768,
        0x0029_17a0,
        0x0029_17a0,
        0x0029_17a0,
        0x0029_17a0,
        0x0029_1780,
        0x0029_17a0,
    ];

    /// The shipped surface group table at [`SURFACE_GROUP_TABLE`].
    pub(crate) const SURFACE_GROUP_HANDLERS: [u32; 9] = [
        0x0029_3524,
        0x0029_3514,
        0x0029_3514,
        0x0029_34fc,
        0x0029_351c,
        0x0029_351c,
        0x0029_351c,
        0x0029_351c,
        0x0029_351c,
    ];

    #[test]
    fn ssx_audio_get_surface_group_matches() {
        strict_check(
            ssx3_ee::s07::f_002934d0__generated,
            ssx_audio_get_surface_group,
            48,
            &|c, seed| {
                machine(c, seed, 0x2934_d0, 0);
                c.pc = 0x0029_34d0;
                put_words(c, SURFACE_GROUP_TABLE, &SURFACE_GROUP_HANDLERS);
                let sys = DATA + 0x100;
                let rider = DATA + 0x800;
                c.g[A0][0] = sx(sys);
                c.g[A2][0] = sx(rider);
                // Group ids below nine dispatch; nine and above return 0.
                c.g[A1][0] = sx(match seed % 11 {
                    9 => 9,
                    10 => 0x100 + seed % 8,
                    _ => seed % 9,
                });
                // Rider_GetMainState(rider) = *(*(rider + 0x77c) + 0xde0).
                c.bus.write32(rider + 0x77c, DATA + 0x900);
                c.bus.write32(DATA + 0x900 + 0xde0, (seed / 11) % 6);
            },
        );
    }

    #[test]
    fn ssx_audio_get_surface_sound_type_matches() {
        strict_check(
            ssx3_ee::s07::f_00291710__generated,
            ssx_audio_get_surface_sound_type,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2917_10, 0);
                c.pc = 0x0029_1710;
                put_words(c, SURFACE_SOUND_TABLE, &SURFACE_SOUND_HANDLERS);
                let sys = DATA + 0x100;
                let rider = DATA + 0x800;
                c.g[A0][0] = sx(sys);
                c.g[A1][0] = sx(rider);
                c.bus.write32(rider + 0x77c, DATA + 0x900);
                c.bus.write32(
                    DATA + 0x900 + 0xde0,
                    if seed % 3 == 0 { 4 } else { (seed / 3) % 6 },
                );
                // The surface id (+1 indexes the table): in range for 0..0x13,
                // past the end for 0x14 and above.
                let id = match seed % 5 {
                    4 => 0x14 + (seed / 5) % 8,
                    _ => (seed / 5) % 0x14,
                };
                c.bus.write32(rider + 0x438, id);
                c.bus.write32(sys + 0x598c, r.next_u32() % 0x14);
            },
        );
    }

    // ------------------------------------------------------ audio mask tables

    #[test]
    fn character_to_speech_mask_matches() {
        fake_check(
            ssx3_ee::s08::f_0029f2b0__generated,
            character_to_speech_mask,
            40,
            &|c, seed, r| {
                put_words(
                    c,
                    0x0048_2e60,
                    &[
                        0x0029_f2e8,
                        0x0029_f2f8,
                        0x0029_f320,
                        0x0029_f2e0,
                        0x0029_f300,
                        0x0029_f318,
                        0x0029_f2d8,
                        0x0029_f310,
                        0x0029_f2f0,
                        0x0029_f308,
                    ],
                );
                let idx = (r.next_u32() % 10) as u32;
                match seed % 5 {
                    0 => {
                        c.g[A0][0] = idx as u64;
                        c.g[A1][0] = (r.next_u32() | 1) as u64;
                    }
                    1 => {
                        c.g[A0][0] = (10 + r.next_u32() % 10) as u64;
                        c.g[A1][0] = 0;
                    }
                    2 => {
                        c.g[A0][0] = idx as u64;
                        c.g[A1][0] = 0;
                        c.bus.write32(0x0048_2e60 + 4 * idx, 0xdead_beef);
                    }
                    _ => {
                        c.g[A0][0] = idx as u64;
                        c.g[A1][0] = 0;
                    }
                }
            },
        );
    }

    #[test]
    fn character_to_voice_mask_matches() {
        fake_check(
            ssx3_ee::s08::f_002a1da0__generated,
            character_to_voice_mask,
            41,
            &|c, seed, r| {
                put_words(
                    c,
                    0x0048_2fe0,
                    &[
                        0x002a_1dd8,
                        0x002a_1de8,
                        0x002a_1e10,
                        0x002a_1dd0,
                        0x002a_1df0,
                        0x002a_1e08,
                        0x002a_1dc8,
                        0x002a_1e00,
                        0x002a_1de0,
                        0x002a_1df8,
                    ],
                );
                let idx = (r.next_u32() % 10) as u32;
                match seed % 5 {
                    0 => {
                        c.g[A1][0] = idx as u64;
                        c.g[A2][0] = (r.next_u32() | 1) as u64;
                    }
                    1 => {
                        c.g[A1][0] = (10 + r.next_u32() % 10) as u64;
                        c.g[A2][0] = 0;
                    }
                    2 => {
                        c.g[A1][0] = idx as u64;
                        c.g[A2][0] = 0;
                        c.bus.write32(0x0048_2fe0 + 4 * idx, 0xdead_beef);
                    }
                    _ => {
                        c.g[A1][0] = idx as u64;
                        c.g[A2][0] = 0;
                    }
                }
            },
        );
    }

    #[test]
    fn crowd_audio_instance_stop_sound_matches() {
        check(
            ee::s08::f_002a6848__generated,
            crowd_audio_instance_stop_sound,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x2a6848, 0);
                globals(c);
                c.g[A0][0] = OBJ as u64;
                let vol = if seed % 2 == 0 {
                    0x3e80_0000
                } else {
                    0x3f40_0000
                };
                w_2(c, OBJ + 0x188, vol);
                w_2(c, OBJ + 0x174, if seed % 3 == 0 { 0xffff_ffff } else { 3 });
                w_2(c, GAME_GP + 0x608, OBJ2);
                w_2(c, OBJ2, OBJ3);
            },
        );
    }

    #[test]
    fn ssx_audio_system_is_track_group0_event_matches() {
        check(
            ee::s08::f_002a4078__generated,
            ssx_audio_system_is_track_group0_event,
            40,
            &|c, seed| {
                fakes::machine(c, seed, 0x2a4078, 0);
                globals(c);
                c.g[A0][0] = OBJ as u64;
                // `cBE_getInterface` really indexes the fixed global array at
                // 0x5308a8 with `*(gp_2 - 0x1efc)`; point slot_2 0 at our object.
                w_2(c, GAME_GP - 0x1efc, 0);
                w_2(c, 0x0053_08a8, OBJ2);
                w_2(c, OBJ2 + 4, VT);
                slot_2(c, VT, 8, 0, ZERO_STUB);
                w_2(c, 0x0053_5c08, OBJ3.wrapping_sub(0x0044_26b0));
                w_2(
                    c,
                    OBJ3,
                    match seed % 3 {
                        0 => 0,
                        1 => 3,
                        _ => 6,
                    },
                );
                w_2(
                    c,
                    0x0053_5c10,
                    match seed % 4 {
                        0 => 0,
                        1 => 5,
                        2 => 3,
                        _ => 7,
                    },
                );
            },
        );
    }

    /// Everything the function touches lives in the test data area.
    pub(crate) const VOICE: u32 = DATA + 0x3_0000;

    pub(crate) const SND: u32 = DATA + 0x1_0000;

    pub(crate) const MIXER: u32 = DATA + 0x2_0000;

    pub(crate) const RING: u32 = DATA + 0x4_0000;

    pub(crate) const TABLES: u32 = DATA + 0x5_0000;

    pub(crate) const INTERFACE: u32 = DATA + 0x6_0000;

    pub(crate) const SOUND: u32 = DATA + 0x7_0000;

    pub(crate) const CHANNEL: u32 = DATA + 0x7_2000;

    /// `BankMonitor::instances` inside the SND system (`+0x1d8 + 0x8ec`).
    pub(crate) const INSTANCES: u32 = SND + 0xac4;

    /// One bank-monitor instance is 0xc0 bytes.
    pub(crate) const INSTANCE_SIZE: u32 = 0xc0;

    /// A voice, the SND system, its mixer and the objects the motion slots
    /// point at, with the state fields chosen by `seed` so the update
    /// follows different paths (idle voice, motion slots with and without a
    /// sound, each player state).
    pub(crate) fn fixture(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x1042_4e88, 8);
        c.bus.write32(GAME_GP + SND_SYSTEM_GP, SND);

        // The SND system: mixer, per-frame word arrays, playback interface,
        // and the bank monitor (all 64 instances free unless written below).
        c.bus.write32(SND, MIXER);
        c.bus.write32(SND + 0x4, TABLES);
        c.bus.write32(SND + 0x8, TABLES + 0x200);
        c.bus.write32(SND + 0xc, INTERFACE);
        c.bus.write16(INTERFACE + 0x10, 0);
        c.bus.write32(INTERFACE + 0x14, FAKE);

        // The mixer's ring: `count` entries at RING, the template copied in
        // front of them, and the per-entry word tables.
        let count = seed % 4;
        c.bus.write32(MIXER + 0x1f4, count);
        c.bus.write32(MIXER + 0x1f8, RING);
        fill(c, &mut r, MIXER + 0x1dc, 6); // 24-byte template
        c.bus.write32(MIXER + 0x20c, TABLES + 0x400);
        c.bus.write32(MIXER + 0x210, TABLES + 0x600);
        c.bus.write32(MIXER + 0x214, TABLES + 0x800);
        c.bus.write32(MIXER + 0x238, TABLES + 0xa00);
        c.bus.write32(MIXER + 0x23c, TABLES + 0xc00);
        c.bus.write32(MIXER + 0x240, TABLES + 0x1100);
        c.bus.write32(MIXER + 0x244, TABLES + 0x1200);
        c.bus.write32(MIXER + 0x248, TABLES + 0x1300);
        c.bus.write32(MIXER + 0x24c, TABLES + 0xe00);
        c.bus.write32(MIXER + 0x250, TABLES + 0x1400);
        c.bus.write32(MIXER + 0x254, TABLES + 0x1000);
        c.bus.write32(MIXER + 0x258, TABLES + 0x1500);
        c.bus.write32(MIXER + 0x25c, TABLES + 0x1600);
        for i in 0..0x1800 / 4 {
            c.bus.write32(TABLES + i * 4, r.next_u32() % 0x4000);
        }
        fill(c, &mut r, RING, 0x40);

        // The voice.
        fill(c, &mut r, VOICE, 0x60);
        c.bus.write32(VOICE + 0x0, r.next_u32() % 0x1000);
        c.bus
            .write32(VOICE + 0xd4, if seed % 5 == 0 { 0 } else { 1 });
        c.bus.write32(VOICE + 0xd8, r.next_u32() % 4);
        c.bus.write32(VOICE + 0x170, r.next_u32() % 4);
        c.bus
            .write32(VOICE + 0x174, (r.next_u32() % 5).wrapping_sub(1));
        c.bus.write32(VOICE + 0x178, r.next_u32());
        c.bus.write32(VOICE + 0x17c, r.next_u32() % 0x4000);
        c.bus.write32(VOICE + 0x180, r.next_u32() % 4);
        c.bus.write32(
            VOICE + 0x18c,
            INSTANCES + (r.next_u32() % 4) * INSTANCE_SIZE,
        );
        for i in 0..4 {
            let q = [r.f32(1.0), r.f32(2.0), r.f32(3.0), r.f32(4.0)];
            put_vec(c, VOICE + 0x10 + 16 * i, q);
        }

        // The three motion slots: one idle, one with a sound, one with a
        // sound whose counter has run out.
        for i in 0..3 {
            let slot = VOICE + 0xe0 + 0x30 * i;
            fill(c, &mut r, slot, 0xc);
            c.bus.write32(slot, SOUND + 0x100 * i);
            c.bus.write32(
                slot + 0x4,
                match (seed + i) % 3 {
                    0 => 0xffff_ffff, // before the first step
                    1 => r.next_u32() % 0x100,
                    _ => r.next_u32() % 8,
                },
            );
            c.bus
                .write32(slot + 0x8, if i == 0 { 0 } else { CHANNEL + 0x100 * i });
            c.bus.write32(slot + 0x20, i);
            let q = [r.f32(50.0), r.f32(50.0), r.f32(50.0), r.f32(1.0)];
            put_vec(c, slot + 0x10, q);
            // The sound the slot carries: three floats read as deltas.
            put_vec(
                c,
                SOUND + 0x100 * i,
                [r.f32(10.0), r.f32(10.0), r.f32(10.0), 0],
            );
            // The mixer channel: its index and frame counter.
            c.bus
                .write32(CHANNEL + 0x100 * i + 0x4, r.next_u32() % 0x100);
            c.bus.write32(CHANNEL + 0x100 * i + 0x20, i);
            put_vec(c, CHANNEL + 0x100 * i + 0x10, [0, 0, 0, 0]);
        }

        c.g[A0][0] = sx(VOICE);
    }

    #[test]
    fn crowd_audio_update_instance_matches() {
        check(
            ee::s08::f_002a4e88__generated,
            crowd_audio_update_instance,
            150,
            &|c, seed| {
                fixture(c, seed);
            },
        );
    }

    // ---------------------------------------------------------------- audio

    #[test]
    fn crowd_audio_instance_remove_all_sources_matches() {
        check(
            ee::s08::f_002a6430__generated,
            crowd_audio_instance_remove_all_sources,
            6,
            &|c, seed| {
                let _ = base(c, seed, 0x002a_6430, 2);
                c.g[A0][0] = OBJ_2 as u64;
                c.f[12] = if seed % 2 == 0 { 0x3f80_0000 } else { 0 };
                let filled = seed % 4;
                for i in 0..filled {
                    w(c, OBJ_2 + 0x20 + i * 0xc, 1);
                }
                w(c, OBJ_2 + 0xd4, 0);
                w(c, OBJ_2 + 0xd8, 0);
            },
        );
    }

    // -------------------------------------------------- parser, replay, codec

    #[test]
    fn ini_parser_skip_section_headers_matches() {
        check(
            ee::s08::f_002a7a20__generated,
            ini_parser_skip_section_headers,
            6,
            &|c, seed| {
                let _ = base(c, seed, 0x002a_7a20, 4);
                c.g[A0][0] = OBJ_2 as u64;
                w(c, OBJ_2 + 8, DATA + 0x4000);
                let text: &[u8] = if seed % 2 == 0 {
                    b"[a]\n[b]\n\0"
                } else {
                    b"key=1\n\0"
                };
                c.bus.ram[DATA as usize + 0x4000..DATA as usize + 0x4000 + text.len()]
                    .copy_from_slice(text);
            },
        );
    }

    #[test]
    fn ssx_audio_system_stop_trick_meter_sound_matches() {
        check(
            ee::s08::f_0029b3c0__generated,
            ssx_audio_system_stop_trick_meter_sound,
            50,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x29b3_c0);
                c.g[A0][0] = sx(DATA);
                let rider = if seed % 3 == 0 { 0 } else { DATA + 0x1000 };
                c.g[A1][0] = sx(rider);
                if rider != 0 {
                    c.bus
                        .write32(rider + 0x874, if seed % 2 == 0 { 0 } else { 1 });
                    c.bus.write32(rider + 0x87c, (seed % 2) as u32);
                }
                c.bus
                    .write32(DATA + 0x5fdc, if seed % 4 == 0 { u32::MAX } else { 0x1234 });
                let p = DATA + 0x2000;
                c.bus.write32(DATA + 0x118, p);
                c.bus.write32(p, DATA + 0x2100);
                fill(c, &mut r, DATA + 0x2100, 0x100);
            },
        );
    }

    #[test]
    fn ssx_audio_system_is_track_group1_event_matches() {
        check(
            ee::s08::f_002a40e8__generated,
            ssx_audio_system_is_track_group1_event,
            60,
            &|c, seed| {
                base_2(c, seed, 0x2a40_e8);
                let iface = DATA + 0x3000;
                let vt = DATA + 0x3200;
                c.bus.write32(0x5308_a8, iface);
                c.bus.write32(iface + 4, vt);
                put_vtable(c, vt, &[8]);
                c.g[A0][0] = sx(DATA);
                let index = seed % 3;
                c.bus.write32(0x535c_08, index);
                let track_id = if seed % 2 == 0 { 6 } else { 2 };
                c.bus.write32(0x43d9_50 + index * 0x64, track_id);
                c.bus.write8(0x535c_10, if seed % 3 == 0 { 1 } else { 6 });
            },
        );
    }

    #[test]
    fn crowd_audio_free_sample_list_matches() {
        check(
            ee::s08::f_002a4cf8__generated,
            crowd_audio_free_sample_list,
            50,
            &|c, seed| {
                alloc_base(c, seed, 0x2a4c_f8);
                let count = seed % 4;
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA, count);
                for i in 0..count {
                    let p = if (seed + i) % 2 == 0 {
                        0
                    } else {
                        alloc(c, 0x20)
                    };
                    c.bus.write32(DATA + 4 + i * 4, p);
                }
            },
        );
    }

    #[test]
    fn ini_parser_find_section_matches() {
        check(
            ee::s08::f_002a7bf8__generated,
            ini_parser_find_section,
            40,
            &|c, seed| {
                base_2(c, seed, 0x2a7b_f8);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x1000);
                put_str(c, DATA + 0x1000, b"section\0");
                // An empty first line makes the parser report "not found"
                // immediately, without walking the ini stream.
                put_str(c, DATA + 0x1100, b"\0");
                c.bus.write32(DATA + 8, DATA + 0x1100);
                c.pc = 0x002a_7bf8;
            },
        );
    }

    // --------------------------------------------------------------- audio

    #[test]
    fn ssx_audio_system_rider_say_20b9_callback_matches() {
        check(
            ee::s08::f_002a1388__generated,
            ssx_audio_system_rider_say_20b9_callback,
            SEEDS_11,
            &|c, seed| {
                machine(c, seed, 0x2a_1388, 2);
                world_2(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.g[A1][0] = (DATA + 0x2400) as u64;
                w(c, DATA + 0x2400 + 0x1c, (seed % 5) as u32);
                w(c, DATA + 0x1200 + 0x78, (seed % 4) as u32);
                for i in 0..4u32 {
                    w(c, DATA + 0x1200 + 0x28 + 4 * i, 100 + i);
                    w(c, DATA + 0x1400 + 0x40 * i + 0x78, 0);
                }
                c.pc = 0x002a_1388;
            },
        );
    }

    #[test]
    fn ssx_audio_system_restart_all_board_loops_matches() {
        check(
            ee::s07::f_002929d8__generated,
            ssx_audio_system_restart_all_board_loops,
            12,
            &|c, seed| {
                machine(c, seed, 0x29_29d8, 3);
                world_2(c);
                c.g[A0][0] = (DATA + 0x20_0000) as u64;
                w(c, DATA + 0x1200 + 0x78, (seed % 3) as u32);
                for i in 0..3u32 {
                    w(c, DATA + 0x1200 + 0x28 + 4 * i, 100 + i);
                }
                c.pc = 0x0029_29d8;
            },
        );
    }

    #[test]
    fn ssx_audio_system_stop_rider_loops_matches() {
        check(
            ee::s08::f_0029ed18__generated,
            ssx_audio_system_stop_rider_loops,
            SEEDS_12,
            &|c, seed| {
                machine(c, seed, 0x29ed18, 6);
                be_library_3(c);
                world_3(c);
                heap_init_4(c);
                let this = DATA + 0x4000;
                c.g[A0][0] = this as u64;
                w(c, this + 0x118, DATA + 0x800);
                w(c, this + 0x800, DATA + 0x900);
                w(c, this + 0x59e0, seed % 8);
                w(
                    c,
                    this + 0x59e4,
                    if seed % 3 == 0 { 0 } else { 1 + seed % 3 },
                );
            },
        );
    }

    #[test]
    fn ini_parser_skip_to_eol_matches() {
        check(
            ee::s08::f_002a7a90__generated,
            ini_parser_skip_to_eol,
            SEEDS_12,
            &|c, seed| {
                machine(c, seed, 0x2a7a90, 2);
                be_library_3(c);
                world_3(c);
                let this = DATA;
                let text = DATA + 0x400;
                c.g[A0][0] = this as u64;
                w(c, this + 8, text);
                let s = ["abc\ndef", "abcdef", "\r\nrest", "", "no terminator here"]
                    [(seed % 5) as usize];
                string(c, text, s);
            },
        );
    }

    #[test]
    fn ssxaudiosystem_stop_all_object_loops_matches() {
        check(
            ee::s08::f_002980b0__generated,
            ssxaudiosystem_stop_all_object_loops,
            40,
            &|c, seed| {
                base_3(c, seed, 0x980b0, 0);
                let s = DATA + 0x100;
                let chans = DATA + 0x800;
                let gain = DATA + 0x840;
                c.g[A0][0] = sx(s);
                c.bus.write32(s + 0x118, chans);
                c.bus.write32(chans, gain);
                c.bus.write32(s + 0x5a24, seed);
                for i in 0..48u32 {
                    c.bus.write32(
                        s + 0x5a00 + 0x30 * i,
                        if (i + seed) % 4 == 0 { 1 } else { 0 },
                    );
                }
            },
        );
    }

    #[test]
    fn ssxaudiosystem_play_hit_boost_sound_matches() {
        check(
            ee::s08::f_00298d00__generated,
            ssxaudiosystem_play_hit_boost_sound,
            SEEDS_13,
            &|c, seed| {
                base_3(c, seed, 0x98d00, 0);
                let audio = DATA + 0x100;
                let rider = DATA + 0x200;
                let chans = DATA + 0x800;
                c.g[A0][0] = sx(audio);
                c.g[A1][0] = sx(rider);
                c.bus.write32(audio + 0x118, chans);
                c.bus.write32(chans, DATA + 0x840);
                c.bus
                    .write32(rider + 0x874, if seed % 3 == 0 { 0 } else { 1 });
                c.bus
                    .write32(rider + 0x87c, if seed % 2 == 0 { 0 } else { seed % 5 });
            },
        );
    }

    #[test]
    fn ssxaudiosystem_djsay20e6_matches() {
        check(
            ee::s08::f_002a27d8__generated,
            ssxaudiosystem_djsay20e6,
            SEEDS_13,
            &|c, seed| {
                base_3(c, seed, 0xa27d8, 0);
                let audio = DATA + 0x100;
                c.g[A0][0] = sx(audio);
                c.bus.write32(GAME_GP + 0x173c, FAKE);
                c.bus.write32(audio + 0x5560, 0);
                c.bus.write32(audio + 0x118, DATA + 0x800);
            },
        );
    }

    // ------------------------------------------------------------------
    // INI parser.

    #[test]
    fn ini_parser_find_section_prefix_matches() {
        check(
            ssx3_ee::s08::f_002a7c68__generated,
            ini_parser_find_section_prefix,
            SEEDS_14,
            &|c, seed| {
                let _r = machine(c, seed, 0x2a7c68, 4);
                be_library_4(c);
                world_4(c);
                let parser = DATA + 0x100;
                let prefix = DATA + 0x200;
                c.g[A0][0] = parser as u64;
                c.g[A1][0] = prefix as u64;
                c.pc = 0x002a_7c68;
                string(c, prefix, if seed % 2 == 0 { "abc" } else { "abd" });
                match seed % 3 {
                    0 => w(c, parser + 8, 0),
                    _ => {
                        let sect = DATA + 0x300;
                        string(c, sect, if seed % 2 == 0 { "abcdef" } else { "xyz" });
                        w(c, parser + 8, sect);
                    }
                }
            },
        );
    }

    #[test]
    fn ssx_audio_system_start_rider_board_loops_matches() {
        check(
            ee::s07::f_00292a50__generated,
            ssx_audio_system_start_rider_board_loops,
            16,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x292a50, 4);
                let sys = DATA + 0x1000;
                let rider = DATA + 0x2000;
                c.g[A0][0] = sys as u64;
                c.g[A1][0] = rider as u64;
                w(c, rider + 0x760, r.next_u32());
                w(c, rider + 0x764, r.next_u32());
                w(c, rider + 0x768, r.next_u32());
                w(c, rider + 0x76c, r.next_u32());
            },
        );
    }

    #[test]
    fn ssx_audio_system_on_mission_abort_matches() {
        check(
            ee::s08::f_0029dbb0__generated,
            ssx_audio_system_on_mission_abort,
            24,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x29dbb0, 4);
                let sys = DATA + 0x1000;
                let vt = DATA + 0x2_0000;
                c.g[A0][0] = sys as u64;
                c.g[A1][0] = if seed % 2 == 0 { 0 } else { 1 };
                w(c, sys + 0x5fd4, if seed % 3 == 0 { 0 } else { 1 });
                w(c, sys + 0x5fd0, r.next_u32());
                w(c, sys + 0x5558, vt);
                slot_3(c, vt, 0x20, 0, FAKE);
            },
        );
    }

    #[test]
    fn ssx_audio_system_record_race_result_matches() {
        check(
            ssx3_ee::s08::f_002a45c0__generated,
            ssx_audio_system_record_race_result,
            10,
            &|c, seed| {
                let _r = base_5(c, seed, 0x2a45c0, 3);
                let this = DATA + 0x1000;
                let world = DATA + 0x8_0000;
                c.g[A0][0] = this as u64;
                w_3(c, world + 0xc0, DATA + 0x7000);
                w_3(c, DATA + 0x7000, if seed % 2 == 0 { 1 } else { 0 });
                w_3(c, 0x0053_6730, r32_seed(seed));
                let race = world + 0x2000;
                let rider = DATA + 0x5000;
                w_3(c, race + 0x28, rider);
                w_3(c, rider + 0x790, DATA + 0x6000);
                w_3(c, DATA + 0x6000 + 0x128, seed);
                w_3(c, DATA + 0x6000 + 0x114, seed.wrapping_mul(3));
                w_3(c, rider + 0x480, seed & 1);
            },
        );
    }

    pub(crate) const ARR: u32 = DATA + 0x13000;

    pub(crate) const SI: u32 = DATA + 0x13100;

    // ---------------------------------------------------------------- audio

    /// The audio system with speech channels on (a channel's volume is 0,
    /// not 1.0), a ready speech instance and a free event slot.
    pub(crate) fn audio_system(c: &mut Cpu, seed: u32) {
        w(c, SYS + 0x5560 + 4, ARR);
        w(c, ARR, SI);
        w(c, SI + 4, 0);
        w(c, GAME_GP.wrapping_add(0x6c4), 0);
        w(c, GAME_GP.wrapping_add(0x173c), 0);
        if seed % 4 == 0 {
            // speech off (channel volume exactly 1.0) for one channel
            for ch in 0..4u32 {
                wf(c, SYS + 0x62bc + 0x10 * ch, 1.0);
            }
        }
        if seed % 3 == 0 {
            w(c, ARR, 0);
        }
        w(c, SYS + 0x5780, 0);
    }

    #[test]
    fn ssx_audio_system_rider_say_20bd_matches() {
        check(
            ee::s08::f_002a1778__generated,
            ssx_audio_system_rider_say_20bd,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x2a1778, 3);
                audio_system(c, seed);
                c.g[A0][0] = SYS as u64;
                c.g[A1][0] = RIDER as u64;
            },
        );
    }

    #[test]
    fn ssx_audio_system_dj_say_212c_matches() {
        check(
            ee::s08::f_002a2b88__generated,
            ssx_audio_system_dj_say_212c,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x2a2b88, 4);
                audio_system(c, seed);
                c.g[A0][0] = SYS as u64;
                w(c, SYS + 0x5780, (seed % 2) as u32);
            },
        );
    }

    #[test]
    fn ssx_audio_system_pa_say_20c1_matches() {
        check(
            ee::s08::f_002a32b0__generated,
            ssx_audio_system_pa_say_20c1,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x2a32b0, 4);
                audio_system(c, seed);
                c.g[A0][0] = SYS as u64;
                w(c, SYS + 0x6290, RIDER);
            },
        );
    }

    #[test]
    fn ssx_audio_system_pa_say_20c2_matches() {
        check(
            ee::s08::f_002a3358__generated,
            ssx_audio_system_pa_say_20c2,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x2a3358, 4);
                audio_system(c, seed);
                c.g[A0][0] = SYS as u64;
                w(c, SYS + 0x6290, RIDER);
            },
        );
    }

    #[test]
    fn ssx_audio_system_dj_say212e_matches() {
        check(
            ee::s08::f_002a2cf0__generated,
            ssx_audio_system_dj_say212e,
            40,
            &|c, seed| {
                let mut r = base_6(c, seed, 0x2a2c);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 4) as u64;
                fill(c, &mut r, DATA, 0x100);
                c.bus.write32(gp_at(0x173c), DATA + 0x2000);
                c.bus.write32(DATA + 0x2000, FAKE);
            },
        );
    }

    #[test]
    fn ssx_audio_system_dj_say212f_matches() {
        check(
            ee::s08::f_002a2da0__generated,
            ssx_audio_system_dj_say212f,
            40,
            &|c, seed| {
                let mut r = base_6(c, seed, 0x2a2d);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 4) as u64;
                fill(c, &mut r, DATA, 0x100);
                c.bus.write32(gp_at(0x173c), DATA + 0x2000);
                c.bus.write32(DATA + 0x2000, FAKE);
            },
        );
    }

    #[test]
    fn ini_parser_read_quoted_string_matches() {
        check(
            ee::s08::f_002a7cf0__generated,
            ini_parser_read_quoted_string,
            80,
            &|c, seed| {
                let mut r = base_6(c, seed, 0x2a7c);
                let text = DATA + 0x1000;
                c.g[A0][0] = sx(DATA);
                let s: &[u8] = match seed % 3 {
                    0 => b"\"ab\"",
                    1 => b"\"\"",
                    _ => b"\"hello world\"",
                };
                for (i, b) in s.iter().enumerate() {
                    c.bus.write8(text + i as u32, *b);
                }
                c.bus.write8(text + s.len() as u32, 0);
                c.bus.write32(DATA + 8, text);
                fill(c, &mut r, DATA + 0xc, 0x40);
            },
        );
    }

    #[test]
    fn ssx_audio_system_stop_object_loop_matches() {
        check(
            ee::s08::f_00297eb8__generated,
            ssx_audio_system_stop_object_loop,
            6,
            &|c, seed| {
                let _ = base(c, seed, 0x0029_7eb8, 3);
                c.g[A0][0] = OBJ_3 as u64;
                c.g[A1][0] = (seed % 5) as u64;
                c.g[A2][0] = (seed % 3) as u64;
                for slot in 0..4u32 {
                    let s = OBJ_3 + slot * 0x30;
                    let hit = slot == seed % 4;
                    w(c, s + 0x5a00, if hit { 1 } else { 0 });
                    w(c, s + 0x5a04, if hit { seed % 5 } else { 99 });
                    w(c, s + 0x5a20, if hit { seed % 3 } else { 99 });
                    w(c, s + 0x5a24, 0x1234 + slot);
                }
                w(c, OBJ_3 + 0x118, OBJ2_2);
                w(c, OBJ2_2, OBJ3_2);
            },
        );
    }

    #[test]
    fn ssx_audio_system_store_race_result_matches() {
        check(
            ee::s08::f_002a4660__generated,
            ssx_audio_system_store_race_result,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x002a_4660, 3);
                c.g[A0][0] = OBJ_3 as u64;
                c.g[A1][0] = (seed % 5) as u64;
                c.g[A2][0] = (seed % 7) as u64;
                c.g[A3][0] = (seed % 9) as u64;
                c.g[T0][0] = (seed & 1) as u64;
                let game = WORLD + 0x1000;
                w(c, game + 0xc0, OBJ2_2);
                w(c, OBJ2_2 + 0x98, (seed & 2) as u32);
                w(c, OBJ2_2 + 0x70, 5);
                w(c, OBJ2_2, (seed % 4) as u32);
                w(c, OBJ_3 + 0x5804, 10);
                w(c, OBJ_3 + 0x5808, 20);
            },
        );
    }

    #[test]
    fn ssx_audio_system_dj_say_20cc_matches() {
        check(
            ee::s08::f_002a24b0__generated,
            ssx_audio_system_dj_say_20cc,
            4,
            &|c, seed| {
                let _ = base(c, seed, 0x002a_24b0, 3);
                c.g[A0][0] = OBJ_3 as u64;
                c.g[A1][0] = (seed % 5) as u64;
                w(c, GAME_GP.wrapping_add(0x173c), FAKE);
                w(c, OBJ_3 + 0x5560, 0);
                w(c, OBJ_3 + 0x5568, 0);
            },
        );
    }

    // ------------------------------------------------------------ audio DJ

    pub(crate) fn dj_setup(c: &mut Cpu, seed: u32, salt: u64) {
        base_7(c, seed, salt, 0);
        c.g[A0][0] = DATA as u64;
    }

    #[test]
    fn dj_say_20c8_matches() {
        check(
            ee::s08::f_002a2018__generated,
            dj_say_20c8,
            30,
            &|c, seed| dj_setup(c, seed, 0x2a20),
        );
    }

    #[test]
    fn dj_say_20c9_matches() {
        check(
            ee::s08::f_002a20d0__generated,
            dj_say_20c9,
            30,
            &|c, seed| dj_setup(c, seed, 0x2a21),
        );
    }

    #[test]
    fn dj_say_20e7_matches() {
        check(
            ee::s08::f_002a2260__generated,
            dj_say_20e7,
            30,
            &|c, seed| dj_setup(c, seed, 0x2a22),
        );
    }

    #[test]
    fn dj_say_20e8_matches() {
        check(
            ee::s08::f_002a2318__generated,
            dj_say_20e8,
            30,
            &|c, seed| dj_setup(c, seed, 0x2a23),
        );
    }

    #[test]
    fn dj_say_20ba_matches() {
        check(
            ee::s08::f_002a2638__generated,
            dj_say_20ba,
            30,
            &|c, seed| dj_setup(c, seed, 0x2a26),
        );
    }

    #[test]
    fn dj_say_212b_matches() {
        check(
            ee::s08::f_002a2ad0__generated,
            dj_say_212b,
            30,
            &|c, seed| dj_setup(c, seed, 0x2a2a),
        );
    }

    #[test]
    fn ssx_audio_system_set_fade_mode_matches() {
        check_all(
            ee::s07::f_00287f00__generated,
            ssx_audio_system_set_fade_mode,
            SEEDS_15,
            &|c, seed| {
                let mut r = machine(c, seed, 0x287f, 3);
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = match seed % 4 {
                    0 => 3,
                    n => n as u64,
                };
                c.f[12] = r.f32(1.0);
                c.f[13] = r.f32(2.0);
                c.bus.write32(this + 0x63d4, (seed % 4) as u32);
                c.bus.write32(this + 0x63d8, r.f32(1.0));
                c.bus.write32(this + 0x63dc, (seed % 2) as u32);
                c.bus.write32(this + 0x63e0, 0);
            },
        );
    }

    #[test]
    fn ssx_audio_system_dj_say20cc_2_matches() {
        check(
            ee::s08::f_002a2568__generated,
            ssx_audio_system_dj_say20cc_2,
            60,
            &|c, seed| {
                let _r = base_8(c, seed, 0x2a25);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed + 1) as u64;
                // Channel 2's volume: off for every other seed.
                c.bus
                    .write32(DATA + 0x62bc + 0x20, if seed % 2 == 0 { 0 } else { f(0.5) });
            },
        );
    }

    pub(crate) const SYS_2: u32 = DATA + 0x10_0000;

    /// The BELibrary singleton (a zeroed dummy, so nothing reads the disc)
    /// and the world (`gp_4 - 0x848`), game module and race manager.
    pub(crate) fn globals_7(c: &mut Cpu) {
        w(c, GAME_GP.wrapping_sub(0x1f04), BE);
        w(c, GAME_GP.wrapping_sub(0x848), WORLD);
        let game = WORLD + 0x1000;
        w(c, WORLD + 0x84, game);
        let race = WORLD + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
    }

    /// The engine's size-class pool_7 allocator.
    pub(crate) fn pool_7(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w(c, GAME_GP.wrapping_add(0xdd8), 1);
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let p = DATA + 0x40_0000 + 0x4_0000 * i;
            let base_9 = DATA + 0x80_0000 + 0x4000 * i;
            for n in 0..8u32 {
                let node = base_9 + 0x400 * n;
                w(c, node, if n == 7 { 0 } else { node + 0x400 });
            }
            w(c, p + head, base_9);
            w(c, GAME_GP + slot, p);
        }
    }

    pub(crate) fn base_9(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        alloc_ready(c);
        globals_7(c);
        pool_7(c);
        r
    }

    #[test]
    fn ssx_audio_system_dj_say_20ca_matches() {
        check(
            ee::s08::f_002a2188__generated,
            ssx_audio_system_dj_say_20ca,
            12,
            &|c, seed| {
                let _ = base_9(c, seed, 0x2a2188, 3);
                audio_system(c, seed);
                c.g[A0][0] = SYS_2 as u64;
            },
        );
    }

    #[test]
    fn ssx_audio_system_dj_say_20cf_matches() {
        check(
            ee::s08::f_002a2860__generated,
            ssx_audio_system_dj_say_20cf,
            12,
            &|c, seed| {
                let _ = base_9(c, seed, 0x2a2860, 3);
                audio_system(c, seed);
                c.g[A0][0] = SYS_2 as u64;
                c.g[A1][0] = (seed % 200) as u64;
            },
        );
    }

    #[test]
    fn ssx_audio_system_dj_say_2102_matches() {
        check(
            ee::s08::f_002a2938__generated,
            ssx_audio_system_dj_say_2102,
            12,
            &|c, seed| {
                let _ = base_9(c, seed, 0x2a2938, 3);
                audio_system(c, seed);
                c.g[A0][0] = SYS_2 as u64;
            },
        );
    }

    #[test]
    fn dj_say_20e9_matches() {
        diff(
            ee::s08::f_002a23d0__generated,
            ssx_audio_system_dj_say_20e9,
            40,
            &|c, seed| {
                let _r = fe_base(c, seed, 0x2a_23d0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 2 == 0 {
                    0xffff_ffff_ffff_ffff
                } else {
                    (seed % 4) as u64
                };
                w(c, DATA + 0x5560, 0);
                w(c, DATA + 0x5578, 0);
                w(c, GAME_GP.wrapping_add(0x173c), 0x002c_c098);
                w(c, DATA + 0x3000, 0);
            },
        );
    }

    // ------------------------------------------------------------ DJ speech

    #[test]
    fn ssx_audio_system_dj_say20e5_matches() {
        strict(
            ee::s08::f_002a26f0__generated,
            ssx_audio_system_dj_say20e5,
            120,
            &|c, seed| {
                let mut r = base_10(c, seed, 0x2a26, 4);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.g[A2][0] = ((seed / 2) % 2) as u64;
                // IsDJSpeechOn is "speech enabled" when the 0x2 type's float at
                // self+0x62dc is non-zero (and the replay query is quiet).
                w(
                    c,
                    DATA + 0x62dc,
                    if seed % 3 == 0 { 0 } else { r.edge_f32() | 1 },
                );
                w(c, W_5 + 0x1000 + 0x28, 0);
                // The pending word clear.
                w(c, DATA + 0x5794, if seed % 2 == 0 { 0 } else { 7 });
                // The speech object at self+0x5560: an entry matching the
                // 0x20e5 context makes SetEventContext return 0.
                let speech = DATA + 0x5560;
                w(c, speech + 0x18, (seed % 2) as u32);
                w(c, speech + 0x20, 0);
                w(
                    c,
                    speech + 0x24,
                    if seed % 3 == 0 { r.next_u32() } else { 0x20e5 },
                );
                w(c, GAME_GP + 0x173c, FAKE);
            },
        );
    }

    // --------------------------------------------------------- arcade speech

    #[test]
    fn ssx_audio_system_arcade_say20a7_matches() {
        strict(
            ee::s08::f_002a3b18__generated,
            ssx_audio_system_arcade_say20a7,
            150,
            &|c, seed| {
                let mut r = base_10(c, seed, 0x2a3b, 6);
                let obj = DATA + 0x800;
                let v = seed % 6;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (if v == 1 { 0 } else { obj }) as u64;
                c.g[A2][0] = r.next_u32() as u64;
                // The animation object: +0x470 is the animation end time
                // (non-negative means the animation has finished).
                w(
                    c,
                    obj + 0x470,
                    if v == 2 { 0x3f80_0000 } else { 0xbf80_0000 },
                );
                // IsArcadeSpeechOn: the 0x20a7 type's float at +0x635c.
                w(c, DATA + 0x635c, if v == 0 { 0 } else { 0x3f80_0000 });
                // IsReplayPlaying: non-zero when world->game->+0x28->+0 is
                // a small index.
                let busy = DATA + 0x5000;
                w(c, W_5 + 0x1000 + 0x28, if v == 5 { busy } else { 0 });
                w(c, busy, 5);
                // 0x285d98 returns the bound object: either race->+0x28 or
                // race->+0x40->+0x18.
                let q = DATA + 0x5100;
                w(c, W_5 + 0x2000 + 0x28, if v == 3 { 0 } else { obj });
                w(c, W_5 + 0x2000 + 0x40, if v == 5 { q } else { 0 });
                w(c, q + 0x18, obj);
                w(c, GAME_GP + 0x173c, FAKE);
                // A matching speech entry makes SetEventContext return 0.
                if seed % 4 == 1 {
                    w(c, DATA + 0x5560 + 0x18, 1);
                    w(c, DATA + 0x5560 + 0x20, 0);
                    w(c, DATA + 0x5560 + 0x24, 0x20a7);
                }
            },
        );
    }

    #[test]
    fn crowd_audio_unload_matches() {
        strict_2(
            ssx3_ee::s08::f_002a6f38__generated,
            crowd_audio_unload,
            SEEDS,
            &|c, seed| {
                let _r = base_11(c, seed, 0x6f38);
                let this = DATA + 0x1000;
                // No instance array and no samples: StopAll and the per-bank
                // free loops all return at once.
                w(c, this + 0x18, 0);
                w(c, this + 0x14, 0);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn crowd_audio_instance_remove_source_matches() {
        same(
            ssx3_ee::s08::f_002a62f0__generated,
            crowd_audio_instance_remove_source,
            200,
            &|c, seed| {
                let mut r = Rng(0x2a62f0 ^ seed as u64);
                generic(c, seed, 0x60);
                let s = DATA + 0x100;
                let idx = (seed % 4) as u64;
                c.g[A0][0] = s as u64;
                c.g[A1][0] = if seed % 8 == 0 { 8 } else { idx };
                c.f[12] = if seed % 2 == 0 { 0 } else { r.f32(1.0) };
                for off in [0xd4u32, 0x170, 0x174, 0x178, 0x180, 0x188, 0x18c] {
                    c.bus.write32(s + off, (r.next_u32() % 4) as u32);
                }
                c.bus.write32(s + 0x188, r.f32(1.0));
                let ent = s + 0x20 + (idx as u32) * 0xc;
                c.bus.write32(ent, if seed % 9 == 0 { 0 } else { 1 });
                let voice = DATA + 0x500 + (seed % 4) * 0x100;
                c.bus.write32(ent + 4, voice);
                c.bus
                    .write32(voice + 8, if seed % 3 == 0 { 0 } else { 1 + seed % 3 });
                c.bus.write32(voice + 4, (seed % 4) as u32);
                c.bus.write32(voice, 0);
                // The audio system singleton.
                let sys = DATA + 0x740;
                c.bus.write32(GAME_GP + 0x608, sys);
                c.bus.write32(sys, DATA + 0x780);
                for off in [0u32, 4, 8, 0xc, 0x10] {
                    c.bus.write32(DATA + 0x780 + off, r.next_u32() % 8);
                }
            },
        );
    }

    #[test]
    fn crowd_audio_push_play_params_matches() {
        same(
            ssx3_ee::s08::f_002a7468__generated,
            crowd_audio_push_play_params,
            200,
            &|c, seed| {
                let mut r = Rng(0x2a7468 ^ seed as u64);
                generic(c, seed, 0x20);
                let self_at = DATA + 0x100;
                let s = DATA + 0x200;
                c.g[A0][0] = self_at as u64;
                c.bus.write32(self_at, s);
                for i in 0..0x100u32 / 4 {
                    c.bus.write32(s + 0x1d8 + 4 * i, r.next_u32() % 4);
                }
                c.bus.write32(s + 0x1f4, (seed % 16) as u32);
                // Ring buffers the template points at.
                for k in 0..8u32 {
                    c.bus.write32(s + 0x1f8 + 4 * k, DATA + 0x4000 + 0x200 * k);
                    c.bus.write32(s + 0x218 + 4 * k, DATA + 0x4400 + 0x200 * k);
                }
                for (i, off) in [
                    0x238u32, 0x23c, 0x240, 0x244, 0x248, 0x24c, 0x250, 0x254, 0x258, 0x25c,
                ]
                .iter()
                .enumerate()
                {
                    c.bus.write32(s + *off, DATA + 0x5000 + 0x200 * i as u32);
                }
            },
        );
    }

    /// The MIDI bank structures `PushPlayParams` writes: `*banks` is the
    /// track object, whose array descriptors at `+0x1d8` point at the record
    /// arrays.  The write cursor at `+0x1f4` (descriptor `+0x1c`) stays small.
    pub(crate) fn midi_bank(c: &mut Cpu, r: &mut Rng, seed: u32) {
        c.bus.write32(DATA + 0x100, DATA + 0x200);
        c.bus.write32(DATA + 0x104, DATA + 0x2400);
        c.bus.write32(DATA + 0x108, DATA + 0x2500);
        c.bus.write32(DATA + 0x218, DATA + 0x600); // banks+0x118: channel state
        c.bus.write32(DATA + 0x600, DATA + 0x700);
        for (base, arr) in [(DATA + 0x200, DATA + 0x2000), (DATA + 0x700, DATA + 0x4000)] {
            for (i, off) in [
                0x20u32, 0x22, 0x24, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x3c, 0x60, 0x64, 0x68, 0x6c,
                0x70, 0x74, 0x78, 0x7c, 0x80, 0x84,
            ]
            .iter()
            .enumerate()
            {
                c.bus.write32(base + 0x1d8 + off, arr + 0x400 * i as u32);
            }
            c.bus.write32(base + 0x1f4, seed % 4); // write cursor
            for k in 0..0x30u32 {
                c.bus.write32(base + 0x1dc + 4 * k, r.next_u32());
            }
            for k in 0..0x100u32 {
                c.bus.write8(arr + k, (r.next_u32() & 0x7f) as u8);
            }
        }
        for k in 0..0x20u32 {
            c.bus.write32(DATA + 0x900 + 4 * k, r.next_u32());
        }
    }

    #[test]
    fn push_play_params_matches() {
        check(
            ssx3_ee::s08::f_002a8450__generated,
            midi_banks_push_play_params,
            200,
            &|c, seed| {
                let mut r = Rng(0x002a_8450 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x900) as u64;
                c.g[A2][0] = (seed % 16) as u64;
                midi_bank(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn play_sound_matches() {
        check(
            ssx3_ee::s07::f_00290cc0__generated,
            ssx_audio_system_play_sound,
            200,
            &|c, seed| {
                let mut r = Rng(0x0029_0cc0 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = (seed % 16) as u64;
                midi_bank(c, &mut r, seed);
            },
        );
    }

    #[test]
    fn play_rail_change_sound_matches() {
        check(
            ssx3_ee::s08::f_00299b70__generated,
            ssx_audio_system_play_rail_change_sound,
            200,
            &|c, seed| {
                let mut r = Rng(0x0029_9b70 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x3000) as u64; // rider
                midi_bank(c, &mut r, seed);
                // The rider's rail-sound state.
                c.bus.write32(DATA + 0x3000 + 0x874, (seed % 3) as u32);
                c.bus.write32(DATA + 0x3000 + 0x87c, (seed % 5) as u32);
            },
        );
    }

    #[test]
    fn start_loop_sfx_matches() {
        check(
            ssx3_ee::s08::f_0029d370__generated,
            ssx_audio_system_start_loop_sfx,
            200,
            &|c, seed| {
                let mut r = Rng(0x0029_d370 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = (DATA + 0x100) as u64;
                midi_bank(c, &mut r, seed);
                // The loop-effect handle slot: sometimes armed.
                c.bus.write32(DATA + 0x100 + 0x6c74, (seed % 3) as u32);
            },
        );
    }

    pub(crate) const BANKS: u32 = DATA + 0x1_0000;

    pub(crate) const BANK_OWNER: u32 = DATA + 0x1_1000;

    #[test]
    fn midi_banks_base_dtor_matches() {
        check_all_2(
            ee::s07::f_0028fea0__generated,
            midi_banks_base_dtor,
            30,
            &|c, seed| {
                let mut r = base_12(c, seed, 0x28fe_a0, 8);
                c.g[A0][0] = sx(BANKS);
                c.g[A1][0] = sx(seed % 4); // destructor flags
                c.bus
                    .write32(BANKS, if seed % 2 == 0 { 0 } else { BANK_OWNER });
                c.bus
                    .write32(BANKS + 0x4, if seed % 3 == 0 { 0 } else { DATA + 0x1_2000 });
                c.bus
                    .write32(BANKS + 0x8, if seed % 5 == 0 { 0 } else { DATA + 0x1_3000 });
                c.bus.write32(BANK_OWNER + 0x1d8, DATA + 0x1_4000);
                c.bus.write32(BANK_OWNER + 0x1d4, DATA + 0x1_5000);
                let _ = &mut r;
            },
        );
    }

    pub(crate) const AUDIO: u32 = DATA + 0x1_0000;

    pub(crate) const AUDIO_OWNER: u32 = DATA + 0x1_1000;

    #[test]
    fn ssx_audio_system_stop_board_loops_matches() {
        check_all_2(
            ee::s07::f_00292b48__generated,
            ssx_audio_system_stop_board_loops,
            20,
            &|c, seed| {
                let _ = base_12(c, seed, 0x292b_48, 12);
                c.g[A0][0] = sx(AUDIO);
                c.bus.write32(AUDIO + 0x118, DATA + 0x1_2000);
                c.bus.write32(DATA + 0x1_2000, AUDIO_OWNER);
                c.bus.write32(AUDIO + 0x59e0, seed % 0x100);
                c.bus.write32(
                    AUDIO + 0x59e4,
                    if seed % 2 == 0 { 0 } else { (seed % 4) as u32 },
                );
            },
        );
    }

    #[test]
    fn ssx_audio_system_pa_say20c5_matches() {
        diff(
            ee::s08::f_002a3708__generated,
            ssx_audio_system_pa_say20c5,
            30,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_3708, 4);
                belib_2(c);
                let audio = DATA + 0x1_0000;
                let rider = DATA + 0x2_0000;
                c.g[A0][0] = sx(audio);
                c.g[A1][0] = sx(rider);
                // Channel 3's volume: 0.0 on even seeds (speech off), 1.0 otherwise.
                c.bus.write32(
                    audio + 0x62bc + 3 * 16,
                    if seed % 2 == 0 { 0 } else { 0x3f80_0000 },
                );
                // The replay/race-type/rider predicates stay false.
                c.bus.write32(audio + 0x6294, 0);
                c.bus.write32(audio + 0x530, 0);
                c.bus.write32(rider + 0x6c0, DATA + 0x3_0000);
                c.bus.write32(DATA + 0x3_0000 + 0x38, 0);
                let _ = r.next_u32();
            },
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

    /// SSXAudioSystem_FlushPendingSpeech: no request, each single request,
    /// several at once (the first in priority order plays), and the rider
    /// line 0x20b9 with its rider present or not. The real speech callees
    /// run on the zeroed audio system of the DJ-line tests.
    #[test]
    fn ssx_audio_system_flush_pending_speech_matches() {
        use crate::difftest::{check, DATA};
        use crate::game::fe::feasyncfile::tests::W_6;
        use crate::game::sound::soundsys::tests::dj_setup;
        const FLAGS: [u32; 13] = [
            0x5768, 0x5784, 0x576c, 0x577c, 0x5770, 0x5788, 0x5774, 0x574c, 0x5750, 0x5754, 0x5758,
            0x5740, 0,
        ];
        check(
            ee::s08::f_002a43b8__generated,
            ssx_audio_system_flush_pending_speech,
            150,
            &|c, seed| {
                dj_setup(c, seed, 0x2a_43b8);
                c.pc = 0x002a_43b8;
                let this = DATA;
                let first = FLAGS[(seed % 13) as usize];
                if first != 0 {
                    c.bus.write32(this + first, 1 + seed);
                    if seed % 3 == 0 {
                        // A lower-priority request too.
                        c.bus.write32(this + 0x5740, 1);
                    }
                }
                c.bus.write32(this + 0x5778, seed % 7);
                c.bus.write32(this + 0x575c, seed % 5);
                c.bus.write32(this + 0x5744, (seed % 4 != 0) as u32);
                let rider = seed % 4;
                c.bus.write32(this + 0x5748, rider);
                let race = W_6 + 0x2000;
                c.bus.write32(
                    race + 0x28 + 4 * rider,
                    if seed % 2 == 0 { 0 } else { DATA + 0x2_0000 },
                );
            },
        );
    }

    /// SSXAudioSystem_UpdateObjectLoops: 30 slots, active or not; a gone
    /// node (the loop stops through the real BankMonitor_StopHandle on the
    /// StopAllObjectLoops fixture), the node's matrix (bone -1), a bone
    /// matrix, or no bones (a real leaf returning 0) and the object's own
    /// position. The node methods are fakes (some unwind).
    #[test]
    fn ssx_audio_system_update_object_loops_matches() {
        use crate::difftest::fakes::{FAKE, LOG_RESULTS};
        use crate::difftest::{check, fill, DATA};
        use crate::game::main::gamestate::tests::base_3;
        const RETURN_ZERO: u32 = 0x0014_06c0;
        check(
            ee::s08::f_00297fa0__generated,
            ssx_audio_system_update_object_loops,
            120,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x29_7fa0, 8);
                c.pc = 0x0029_7fa0;
                let s = DATA + 0x100;
                let chans = DATA + 0x800;
                let gain = DATA + 0x840;
                c.g[A0][0] = sx(s);
                c.bus.write32(s + 0x118, chans);
                c.bus.write32(chans, gain);
                // Two node vtables: with bones (fake) and without (0).
                let (vt_bones, vt_flat) = (DATA + 0x6800, DATA + 0x6900);
                for (vt, has_bones) in [(vt_bones, FAKE + 0x10), (vt_flat, RETURN_ZERO)] {
                    for (off, f) in [(0xc0, FAKE), (0xd0, has_bones), (0xe8, FAKE + 0x20)] {
                        c.bus
                            .write16(vt + off, (r.next_u32() as i16 as i32 & !3) as u16);
                        c.bus.write32(vt + off + 4, f);
                    }
                }
                // Fake matrices with a translation.
                for k in 0..4u32 {
                    let m = DATA + 0x7000 + 0x100 * k;
                    c.bus.write32(LOG_RESULTS + 4 * k, m);
                    fill(c, &mut r, m + 0x30, 4);
                }
                for i in 0..30u32 {
                    let slot = s + 0x5a00 + 0x30 * i;
                    let obj = DATA + 0x2000 + 0x80 * i;
                    let node = DATA + 0x4000 + 0x40 * i;
                    let active = if (i + seed) % 3 == 0 {
                        1
                    } else {
                        r.next_u32() % 3
                    };
                    c.bus.write32(slot, active);
                    c.bus.write32(slot + 4, obj);
                    c.bus.write32(
                        slot + 8,
                        if r.next_u32() % 2 == 0 {
                            u32::MAX
                        } else {
                            r.next_u32() % 20
                        },
                    );
                    c.bus.write32(slot + 0x24, r.next_u32() % 8);
                    c.bus
                        .write32(obj + 0xc, if r.next_u32() % 5 == 0 { 0 } else { node });
                    fill(c, &mut r, obj + 0x40, 4);
                    c.bus.write32(
                        node + 0xc,
                        if r.next_u32() % 2 == 0 {
                            vt_bones
                        } else {
                            vt_flat
                        },
                    );
                }
            },
        );
    }

    /// SSXAudioSystem_UpdateRaceLoopSfx: the world at `gp-0x848`
    /// (`feasyncfile::tests::base_2`) with its race manager's state holding
    /// a loop id that is -1, the same (low byte) as the one seen, or new;
    /// phases below and from 22. The real StartLoopSfx (no loop bank:
    /// `+0x6c74 = 0`), StopLoopSfx (nothing playing: `+0x5fc4 = 0`) and
    /// BankMonitor_StopByBank (63 idle instances, the events on or off)
    /// run.
    #[test]
    fn ssx_audio_system_update_race_loop_sfx_matches() {
        use crate::game::fe::feasyncfile::tests::base_2;
        check(
            ee::s08::f_0029d290__generated,
            ssx_audio_system_update_race_loop_sfx,
            300,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x0029_d290);
                let gp = crate::difftest::GAME_GP;
                let app = c.bus.read32(gp - 0x848);
                let man = c.bus.read32(app + 0x84);
                let race_man = DATA + 0x10000;
                let race = DATA + 0x10100;
                let this = DATA + 0x11000;
                let engine_ptr = DATA + 0x18000;
                let object = DATA + 0x18100;
                let events = DATA + 0x19000;
                let instances = DATA + 0x1a000;
                c.bus.write32(man + 0xc, race_man);
                c.bus.write32(race_man + RACE_MAN_STATE, race);
                for off in [GP_LOOP_FADE, GP_BANK_A_FADE, GP_BANK_B_FADE] {
                    let v = r.f32(8.0);
                    c.bus.write32(gp - off, v);
                }
                let seen = r.next_u32();
                c.bus.write32(this + AUDIO_LOOP_SEEN, seen);
                let id = match seed % 4 {
                    0 => u32::MAX,
                    1 => (r.next_u32() & !0xff) | (seen & 0xff),
                    _ => r.next_u32() | 1,
                };
                c.bus.write32(race + RACE_LOOP_ID, id);
                let phase = if seed % 2 == 0 {
                    r.next_u32() % 22
                } else {
                    22 + r.next_u32() % 10
                };
                c.bus.write32(race + RACE_PHASE, phase);
                c.bus.write32(this + 0x6c74, 0);
                c.bus.write32(this + 0x5fc4, 0);
                c.bus.write32(this + AUDIO_ENGINE, engine_ptr);
                c.bus.write32(engine_ptr, object);
                c.bus.write32(object + ENGINE_EVENTS, events);
                for on in [0x300u32, 0x360] {
                    let state = if r.next_u32() % 2 == 0 {
                        1
                    } else {
                        r.next_u32() % 4
                    };
                    c.bus.write32(events + on, state);
                    c.bus.write32(events + on + 4, r.next_u32() % 3);
                }
                c.bus
                    .write32(object + ENGINE_BANK_MONITOR + 0x8ec, instances);
                for i in 0..64u32 {
                    let inst = instances + i * 0xc0;
                    c.bus.write32(inst, 0);
                    c.bus.write32(inst + 0xc, r.next_u32() % 3);
                }
                c.g[A0][0] = sx(this);
                c.pc = 0x0029_d290;
            },
        );
    }

    /// SSXAudioSystem_UpdateUberSpeech: the real IsReplayPlaying (no game
    /// module, no replay, replay state 0 / playing), the real FindRider over
    /// up to three race-manager riders, the tracked id matching or not, the
    /// rider state 10 or not, the last state 10 or not, game mode 4 or not,
    /// and the real ArcadeSay2133 on its short paths (arcade speech gain 0,
    /// or the rider's air value not negative).
    #[test]
    fn ssx_audio_update_uber_speech_matches() {
        const SYS: u32 = DATA;
        const GAME: u32 = DATA + 0x1_0000;
        const MODULE: u32 = DATA + 0x1_0100;
        const RACE_MAN: u32 = DATA + 0x1_0400;
        const REPLAY: u32 = DATA + 0x1_0800;
        const RIDERS: u32 = DATA + 0x2_0000;
        check(
            ee::s08::f_0029b738__generated,
            ssx_audio_update_uber_speech,
            300,
            &|c, seed| {
                let mut r = Rng(0x29_b738 ^ (seed as u64) << 20);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.pc = 0x0029_b738;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), GAME);
                c.bus
                    .write32(GAME + 0x84, if seed % 17 == 0 { 0 } else { MODULE });
                c.bus.write32(MODULE + 0xc, RACE_MAN);
                c.bus
                    .write32(MODULE + 0x28, if seed % 4 == 0 { REPLAY } else { 0 });
                c.bus
                    .write32(REPLAY, [0, 0, 3, 12][(r.next_u32() % 4) as usize]);
                c.bus.write32(REPLAY + 0x18, RIDERS);
                c.bus.write32(RACE_MAN + 0x40, REPLAY);
                c.bus.write32(
                    MODULE + UBER_MODULE_MODE,
                    [4, 4, 0, 2][(r.next_u32() % 4) as usize],
                );
                c.bus.write32(RACE_MAN + 0x7c, r.next_u32() % 4);
                c.bus.write32(RACE_MAN + 0x78, r.next_u32() % 4);
                let id = r.next_u32() % 2;
                for i in 0..3 {
                    let rd = RIDERS + 0x1000 * i;
                    c.bus.write32(RACE_MAN + 0x28 + 4 * i, rd);
                    c.bus.write32(rd + 0x874, r.next_u32() % 2);
                    c.bus.write32(rd + 0x87c, r.next_u32() % 2);
                    c.bus.write32(rd + 0xec, (r.next_u32() % 8).wrapping_sub(1));
                    c.bus.write32(
                        rd + UBER_RIDER_PLAYER_ID,
                        if r.next_u32() % 4 == 0 { 1 - id } else { id },
                    );
                    c.bus.write32(
                        rd + UBER_RIDER_STATE,
                        if r.next_u32() % 3 == 0 { 3 } else { 10 },
                    );
                    // Air value: not negative, so ArcadeSay2133 stops early.
                    c.bus.write32(rd + 0x470, r.f32(4.0) & 0x7fff_ffff);
                }
                c.bus.write32(SYS + UBER_RIDER_ID, id);
                c.bus.write32(
                    SYS + UBER_LAST_STATE,
                    if r.next_u32() % 3 == 0 { 10 } else { 0 },
                );
                // Arcade speech gain (channel 10): off or on.
                c.bus.write32(
                    SYS + 0x635c,
                    if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        0x3f80_0000
                    },
                );
                c.g[A0][0] = sx(SYS);
            },
        );
    }

    // -- 0x002947b0 SSXAudioSystem_UpdateAirborneRiders ------------------------

    /// `SSXAudioSystem_UpdateAirborneRiders` against the generated body: the
    /// six airborne flags set or clear, rider counts from none to more than
    /// six (and negative), empty rider slots. The real `UpdateRiderAirState`
    /// runs its "landed" path (animation state neither 1 nor 3): its rider
    /// virtual is a fake (some unwind, some forced to a slot index so
    /// `SetRiderAirFlag` sets a later flag).
    #[test]
    fn ssx_audio_system_update_airborne_riders_matches() {
        use super::airborne_riders::*;
        use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
        use crate::difftest::{DATA, GAME_GP};
        const THIS: u32 = DATA + 0x1_0000;
        const WORLD: u32 = DATA + 0x2_0000;
        const GAME: u32 = DATA + 0x2_0100;
        const RACE: u32 = DATA + 0x2_0200;
        const RIDERS: u32 = DATA + 0x3_0000;
        covered(
            ee::s07::f_002947b0__generated,
            ssx_audio_system_update_airborne_riders,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0029_47b0, 6);
                c.pc = 0x0029_47b0;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                w(c, GAME_GP - 0x848, WORLD);
                w(c, WORLD + 0x84, GAME);
                w(c, GAME + 0xc, RACE);
                let count = match r.next_u32() % 9 {
                    8 => 0xffff_ffff,
                    n => n,
                };
                w(c, RACE + RACE_RIDER_COUNT, count);
                for i in 0..6u32 {
                    w(
                        c,
                        THIS + AIR_FLAGS + 4 * i,
                        if r.next_u32() % 3 == 0 {
                            0
                        } else {
                            r.next_u32() | 1
                        },
                    );
                    let rider = RIDERS + 0x1000 * i;
                    let anim = rider + 0x800;
                    w(
                        c,
                        RACE + RACE_RIDERS + 4 * i,
                        if r.next_u32() % 5 == 0 { 0 } else { rider },
                    );
                    w(c, rider + 0x788, anim);
                    w(c, anim + 0xac, [0, 2, 4][(r.next_u32() % 3) as usize]);
                    // The rider's air-state object's vtable (slot 0x38).
                    let vt = rider + 0xc00;
                    c.bus.write16(vt + 0x38, (r.next_u32() as i16 & !3) as u16);
                    w(c, vt + 0x3c, FAKE + 0x10 * i);
                    w(c, rider + 0x6c0, vt);
                }
                // Fake results: a pointer (no flag) or a slot index 1..5.
                for n in 0..8u32 {
                    let v = if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        1 + r.next_u32() % 5
                    };
                    w(c, LOG_RESULTS + 4 * n, v);
                }
                c.g[A0][0] = sx(THIS);
            },
        );
    }

    /// A looping effect off, or on with a handle that is live in the SND
    /// channel table or negative (done). SND_Lock/SND_Unlock go
    /// through the lock hooks, which are fakes (some seeds unwind there); the
    /// next effect's StartLoopSfx returns at once (no loop list, +0x6c74 = 0).
    #[test]
    fn ssx_audio_system_update_loop_sfx_matches() {
        use crate::difftest::fakes::{covered, FAKE, LOG_RESULTS};
        use loop_sfx::*;
        /// SND state (0x50a8e8): channel count (+0x17e), channel table (+0x1f0).
        const SNDS: u32 = 0x0051_0000 - 0x5718;
        const LOCK_HOOK: u32 = 0x0045_0000 - 0x3b94;
        covered(
            ssx3_ee::s08::f_0029d610__generated,
            ssx_audio_system_update_loop_sfx,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x29d610, 2);
                c.pc = 0x0029_d610;
                for i in 0..2 {
                    c.bus.write32(LOG_RESULTS + 4 * i, 0);
                }
                c.bus.write32(LOCK_HOOK, FAKE + 0x100);
                c.bus.write32(LOCK_HOOK + 4, FAKE + 0x110);
                let channels = DATA + 0x8000;
                c.bus.write16(SNDS + 0x17e, 8);
                c.bus.write32(SNDS + 0x1f0, channels);
                c.bus.write8(SNDS + 0x176, (r.next_u32() % 4) as u8);
                let sys = DATA;
                let handle = r.next_u32() & 0x7fff_ff00 | r.next_u32() % 8;
                // The handle's channel is live (a stale non-negative handle
                // would make StopLoopSfx stop it through a bank monitor).
                let ch = channels + 0x8c * (handle & 0xff);
                c.bus.write8(ch + 0x69, 1);
                c.bus.write32(ch, handle);
                c.bus.write32(
                    sys + LOOP_ACTIVE,
                    if seed % 6 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(
                    sys + LOOP_HANDLE,
                    if seed % 5 == 0 {
                        0x8000_0000 | r.next_u32()
                    } else {
                        handle
                    },
                );
                c.bus.write32(sys + 0x6c74, 0);
                c.g[A0][0] = sys as u64;
            },
        );
    }

    /// 0..3 instances (sometimes a negative count); the instances are
    /// inactive (+0xd4 = 0), so the real `CrowdAudio_UpdateInstance` takes
    /// its short path.
    #[test]
    fn crowd_audio_update_matches() {
        check(
            ee::s08::f_002a6d18__generated,
            crowd_audio_update,
            200,
            &|c, seed| {
                let mut r = Rng(0x2a6d_1800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x002a_6d18;
                c.g[GP][0] = GAME_GP as u64;
                let count: i32 = match seed % 5 {
                    0 => 0,
                    1 => -(r.next_u32() as i32 & 0xff) - 1,
                    n => n as i32 - 1,
                };
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + CROWD_AUDIO_COUNT, count as u32);
                let array = DATA + 0x1000;
                c.bus.write32(DATA + CROWD_AUDIO_INSTANCES, array);
                for i in 0..4 {
                    c.bus
                        .write32(array + i * CROWD_AUDIO_INSTANCE_SIZE + 0xd4, 0);
                }
            },
        );
    }

    /// The audio system at DATA, the rider at DATA+0x10000 with its view
    /// object's vtable call faked (forced to a small surface index, sometimes
    /// unwinding); the curve tables and constants come from the ELF image.
    #[test]
    fn ssx_audio_system_calc_surface_loop_remote_group0_matches() {
        use crate::difftest::elf_image;
        use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
        covered(
            ee::s07::f_00293b50__generated,
            ssx_audio_system_calc_surface_loop_remote_group0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x293b50, 1);
                elf_image(c);
                c.pc = 0x0029_3b50;
                let system = DATA;
                let rider = DATA + 0x10000;
                let vt = DATA + 0x20000;
                let out = DATA + 0x30000;
                c.g[A0][0] = system as u64;
                c.g[A1][0] = rider as u64;
                c.g[A2][0] = out as u64;
                c.g[A3][0] = (out + 4) as u64;
                c.bus.write32(rider + 0x6c0, vt);
                c.bus.write16(vt + 0x38, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(vt + 0x3c, FAKE);
                for off in [0x750, 0x754, 0x758, 0x75c] {
                    c.bus.write32(
                        rider + off,
                        ((r.next_u32() % 2000) as f32 / 1000.0).to_bits(),
                    );
                }
                c.bus.write32(LOG_RESULTS, 1 + seed % 4);
                for i in 0..8 {
                    c.bus.write32(
                        system + 0x59b8 + 4 * i,
                        ((r.next_u32() % 2000) as f32 / 1000.0).to_bits(),
                    );
                }
            },
        );
    }

    /// `SSXAudioSystem_PlayWallHitSound`: no sound table; no view in range
    /// (no views, or views far away; the real `FindViewInRange`); the sound
    /// already tracked (a ring record `{evt, idx, 0}`; the real
    /// `IsTrackedSoundPlaying`); otherwise the real curve table on the
    /// rider's speed, the id pick (idx in / out of range, zero ids) and the
    /// real `PlayTriggerSound`, which fails (trigger type 3). The rider's
    /// position / velocity methods are fakes (which may unwind).
    /// Not reached: a successful `PlayTriggerSound` and `TrackSound`.
    #[test]
    fn ssx_audio_system_play_wall_hit_sound_matches() {
        use crate::difftest::fakes::{vtable, LOG_RESULTS};
        crate::difftest::check_lohi(
            ssx3_ee::s08::f_00296088__generated,
            ssx_audio_system_play_wall_hit_sound,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0029_6088, 2);
                elf_image(c);
                c.pc = 0x0029_6088;
                let base = DATA + 0x20_0000;
                let (app, module, viewman) = (base, base + 0x100, base + 0x200);
                let (rider, pos, vel) = (base + 0x1000, base + 0x300, base + 0x340);
                let (evt, sounds, table) = (base + 0x2000, base + 0x2100, base + 0x2200);
                let (wtm, wtable) = (base + 0x3000, base + 0x3400);
                let (sys, ring) = (base + 0x1_0000, base + 0x2_0000);
                // Views.
                c.bus.write32(GAME_GP - 0x848, app);
                c.bus.write32(app + 0x84, module);
                c.bus.write32(module + 0x84, viewman);
                let views = if seed % 7 == 3 {
                    0
                } else {
                    1 + r.next_u32() % 2
                };
                c.bus.write32(viewman + 0x10, views);
                let far = seed % 7 == 5;
                for i in 0..2 {
                    let view = base + 0x400 + 0x40 * i;
                    c.bus.write32(viewman + 4 + 4 * i, view);
                    for k in 0..4 {
                        let v = if far {
                            2.0e5f32.to_bits()
                        } else {
                            r.f32(3000.0)
                        };
                        c.bus.write32(view + 0x20 + 4 * k, v);
                    }
                }
                // The rider's position and velocity methods (fakes).
                let vt = vtable(c, &mut r, &[0x28, 0x10]);
                c.bus.write32(rider + WALLHIT_RIDER_BODY, vt);
                for k in 0..4 {
                    c.bus.write32(pos + 4 * k, r.f32(3000.0));
                    c.bus.write32(vel + 4 * k, r.f32(60.0));
                }
                c.bus.write32(LOG_RESULTS, pos);
                c.bus.write32(LOG_RESULTS + 4, vel);
                // The event's sound table: count 1..4, ids 0..7 (some 0).
                c.bus.write32(
                    evt + WALLHIT_EVENT_SOUNDS,
                    if seed % 9 == 0 { 0 } else { sounds },
                );
                c.bus.write32(sounds + 0x10, table);
                c.bus.write32(table, 1 + r.next_u32() % 4);
                for k in 0..5 {
                    let id = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        r.next_u32() % 8
                    };
                    c.bus.write32(table + 8 + 4 * k, id);
                }
                let idx = (r.next_u32() % 7).wrapping_sub(1);
                // Trigger types: all 3 (PlayTriggerSound fails at once).
                c.bus.write32(GAME_GP + WALLHIT_TRIGGER_MANAGER_GP, wtm);
                c.bus.write32(wtm + 0x33c, wtable);
                for id in 0..8 {
                    c.bus.write32(wtable + 8 * id, 3);
                }
                // The tracked-sound ring: empty, or one record for this
                // event and index.
                let cap = 40u32;
                let tail = r.next_u32() % cap;
                c.bus.write32(sys + 0x597c, cap);
                c.bus.write32(sys + 0x5980, ring);
                c.bus.write32(sys + 0x5988, tail);
                let tracked = seed % 5 == 2;
                c.bus
                    .write32(sys + 0x5984, if tracked { (tail + 5) % cap } else { tail });
                let record = [evt, idx, 0, r.next_u32(), r.next_u32()];
                for (k, v) in record.into_iter().enumerate() {
                    c.bus.write32(ring + 4 * ((tail + k as u32) % cap), v);
                }
                c.g[A0][0] = sx(sys);
                c.g[A1][0] = sx(rider);
                c.g[A2][0] = sx(evt);
                c.g[A3][0] = sx(idx);
            },
        );
    }

    // -- 0x00294678 SSXAudioSystem_UpdateRiderAirState ------------------------

    /// `SSXAudioSystem_UpdateRiderAirState` against the generated body: air
    /// mode 1/3 (airborne) or other (landed); the rider's slot virtual is a
    /// fake (some unwind, some forced to a slot index); the real
    /// `RiderSay2083` stops at once (speech channel volume 0); the real
    /// `IsReplayPlaying` (replay states none/0/3/12) and `FindRider` over up
    /// to three race riders (which may or may not be this rider); human flags
    /// and player ids; air time below or above 2.0; the real
    /// `CrowdAudio_StartAnticipate` with or without a crowd array.
    #[test]
    fn ssx_audio_system_update_rider_air_state_matches() {
        use super::rider_air_state::*;
        use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
        use crate::difftest::{DATA, GAME_GP};
        const THIS: u32 = DATA + 0x1_0000;
        const WORLD: u32 = DATA + 0x2_0000;
        const MODULE: u32 = DATA + 0x2_0100;
        const RACE_MAN: u32 = DATA + 0x2_0400;
        const REPLAY: u32 = DATA + 0x2_0800;
        const CROWD: u32 = DATA + 0x4_0000;
        const RIDERS: u32 = DATA + 0x5_0000;
        covered(
            ee::s07::f_00294678__generated,
            ssx_audio_system_update_rider_air_state,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0029_4678, 1);
                c.pc = 0x0029_4678;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                w(c, GAME_GP - 0x848, WORLD);
                w(c, WORLD + 0x84, MODULE);
                w(c, MODULE + 0xc, RACE_MAN);
                w(
                    c,
                    MODULE + 0x28,
                    if r.next_u32() % 3 == 0 { REPLAY } else { 0 },
                );
                w(c, REPLAY, [0, 0, 3, 12][(r.next_u32() % 4) as usize]);
                w(c, RACE_MAN + 0x40, REPLAY);
                w(c, RACE_MAN + 0x7c, r.next_u32() % 4);
                w(c, RACE_MAN + 0x78, r.next_u32() % 4);
                for i in 0..3u32 {
                    let rd = RIDERS + 0x1000 * i;
                    let anim = rd + 0x800;
                    w(c, RACE_MAN + 0x28 + 4 * i, rd);
                    w(c, rd + RIDER_ANIM, anim);
                    w(
                        c,
                        anim + ANIM_AIR_MODE,
                        [1, 3, 1, 3, 0, 2][(r.next_u32() % 6) as usize],
                    );
                    w(c, anim + ANIM_AIR_TIME, r.f32(4.0));
                    w(c, rd + RIDER_HUMAN, r.next_u32() % 2);
                    w(c, rd + RIDER_HUMAN2, r.next_u32() % 2);
                    w(c, rd + RIDER_PLAYER_ID, (r.next_u32() % 4).wrapping_sub(1));
                    w(c, rd + 0xec, (r.next_u32() % 8).wrapping_sub(1));
                    let vt = rd + 0xc00;
                    c.bus
                        .write16(vt + VT_SLOT, (r.next_u32() as i16 & !3) as u16);
                    w(c, vt + VT_SLOT + 4, FAKE);
                    w(c, rd + RIDER_AIR_OBJ, vt);
                }
                let k = r.next_u32() % 3;
                w(c, REPLAY + 0x18, RIDERS + 0x1000 * (r.next_u32() % 3));
                // Speech channel (4) volume 0: RiderSay2083 returns at once.
                w(c, THIS + 0x62bc + 0x40, 0);
                w(
                    c,
                    THIS + 0x18,
                    if r.next_u32() % 4 == 0 { 0 } else { CROWD },
                );
                // Fake slot results: a pointer (no flag) or a slot index 1..5.
                let v = if r.next_u32() % 2 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 5
                };
                w(c, LOG_RESULTS, v);
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = sx(RIDERS + 0x1000 * k);
            },
        );
    }

    // -- 0x002a77c8 CrowdAudio_GetInstanceReaction -----------------------------

    /// `CrowdAudio_GetInstanceReaction` against the generated body: indices
    /// below, at and above the count, negative indices (and upper bits in
    /// a1 that the 64-bit compare sees); LO/HI compared too.
    #[test]
    fn crowd_audio_get_instance_reaction_matches() {
        use crate::difftest::{check_lohi, fill, scramble, Rng, DATA};
        const CROWD: u32 = DATA;
        const INSTANCES: u32 = DATA + 0x1000;
        check_lohi(
            ee::s08::f_002a77c8__generated,
            crowd_audio_get_instance_reaction,
            300,
            &|c, seed| {
                let mut r = Rng(0x002a_77c8 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x002a_77c8;
                let count = r.next_u32() % 8;
                c.bus.write32(CROWD + CROWD_INSTANCE_COUNT, count);
                c.bus.write32(CROWD + CROWD_INSTANCES, INSTANCES);
                for i in 0..8 {
                    fill(
                        c,
                        &mut r,
                        INSTANCES + CROWD_INSTANCE_SIZE * i + CROWD_INSTANCE_REACTION,
                        1,
                    );
                }
                c.g[A0][0] = sx(CROWD);
                c.g[A1][0] = match seed % 4 {
                    0 | 1 => (r.next_u32() % 8) as u64,
                    2 => sx((r.next_u32() % 4).wrapping_neg()),
                    _ => (r.next_u32() % 8) as u64 | (r.next_u32() as u64) << 32,
                };
            },
        );
    }

    // ------------------------------------------------------ SSXAudioSystem

    /// Audio-system test world: the audio system, its bank monitor (push
    /// queue tables zeroed), a rider, the game chain for
    /// `SSXAudioSystem_IsReplayPlaying`, fake SND lock hooks.
    pub(crate) mod audio {
        use super::*;
        use crate::difftest::fakes::FAKE;
        use crate::difftest::Rng;

        pub const SYS: u32 = DATA + 0x20_0000;
        pub const MONITOR: u32 = DATA + 0x21_0000;
        pub const ROOT: u32 = DATA + 0x22_0000;
        pub const TABLES: u32 = DATA + 0x23_0000;
        pub const RIDER: u32 = DATA + 0x24_0000;
        pub const GAME: u32 = DATA + 0x25_0000;
        pub const MODULE: u32 = DATA + 0x25_1000;
        pub const REPLAY: u32 = DATA + 0x25_2000;

        /// The machine (fakes, no unwinds), the ELF image, and the world;
        /// `count` entries already in the push queue (0..=2).
        pub fn world(c: &mut Cpu, seed: u32, salt: u64, entry: u32) -> Rng {
            let mut r = machine(c, seed, salt, 0);
            elf_image(c);
            c.pc = entry;
            c.bus.write32(GAME_GP - 0x1f04, DATA + 0x26_0000); // BELibrary
            c.bus.write32(0x0045_0000 - 0x3b94, FAKE); // SND_Lock hook
            c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10); // SND_Unlock hook
                                                              // Replay state: not in game, no replay, or replay states 0/5/12/20.
            c.bus.write32(GAME_GP - 0x848, GAME);
            let in_game = seed % 7 != 0;
            c.bus.write32(GAME + 0x84, if in_game { MODULE } else { 0 });
            c.bus
                .write32(MODULE + 0x28, if seed % 5 == 1 { 0 } else { REPLAY });
            let replay = [0u32, 0, 5, 12, 20][(seed / 3 % 5) as usize];
            c.bus.write32(REPLAY, replay);
            // The system and its monitor.
            c.bus.write32(SYS + AUDIO_SYS_MONITOR, MONITOR);
            c.bus.write32(SYS + AUDIO_SYS_PUSH_BANK, TABLES + 0x1000);
            c.bus.write32(SYS + AUDIO_SYS_PUSH_SAMPLE, TABLES + 0x1100);
            c.bus.write32(SYS + AUDIO_SYS_SND_ROOT, ROOT);
            c.bus.write32(ROOT, MONITOR);
            for i in 0..0x400 {
                c.bus.write32(MONITOR + 4 * i, 0);
            }
            c.bus.write32(MONITOR + MONITOR_PUSH_COUNT, seed / 2 % 3);
            fill(c, &mut r, MONITOR + 0x1dc, 6); // 24-byte entry template
                                                 // Queue tables (queue + 0x20 .. + 0x84), zeroed.
            for (i, off) in [0x20u32, 0x34, 0x38, 0x3c]
                .into_iter()
                .chain((0x60..=0x84).step_by(4))
                .enumerate()
            {
                c.bus.write32(
                    MONITOR + MONITOR_PUSH_QUEUE + off,
                    TABLES + 0x100 * i as u32,
                );
            }
            for i in 0..0x1200 / 4 {
                c.bus.write32(TABLES + 4 * i, 0);
            }
            // Bank-loaded table: zero (no bank: SND_Play fails at once).
            c.bus.write32(MONITOR + 0xacc, TABLES + 0x1180);
            // Channel gains (SSXAudioSystem_GetChannelGainPtr).
            fill(c, &mut r, SYS + 0x6300, 0x40);
            c.bus.write32(SYS + 0x6358, r.f32(1.0));
            // The rider.
            c.bus
                .write32(RIDER + RIDER_AUDIO_HUMAN, (seed % 4 != 1) as u32);
            c.bus
                .write32(RIDER + RIDER_AUDIO_LOCAL, if seed % 6 == 2 { 0 } else { 1 });
            c.g[A0][0] = sx(SYS);
            c.g[A1][0] = sx(RIDER);
            r
        }
    }

    /// `SSXAudioSystem_PlayBoostEmptySound`: in and out of replays, human
    /// and local or not; the play path runs the real timer kill, memset,
    /// gain lookup and `Audio_PlayPushedSound` (no bank loaded: `SND_Play`
    /// fails and the pushed entry is dropped again).
    #[test]
    fn ssx_audio_system_play_boost_empty_sound_matches() {
        check(
            ee::s08::f_00299368__generated,
            ssx_audio_system_play_boost_empty_sound,
            120,
            &|c, seed| {
                audio::world(c, seed, 0x29_9368, 0x0029_9368);
            },
        );
    }

    /// `SSXAudioSystem_OnRacePositionChange`: race-type check byte on/off
    /// (the real `IsRaceType4`), no game module / not a race, two racers
    /// or fewer, a human local rider or not (the real `FindRider`), first
    /// place or not (the sting: real tracked-sound ring, empty, the real
    /// push queue and `Audio_PlayPushedSound` with no bank, `TrackSound`),
    /// position ahead of or behind the limit with and without the other
    /// rider (the real `RiderSay207D`, mostly on its speech-off early out).
    #[test]
    fn ssx_audio_system_on_race_position_change_matches() {
        use crate::difftest::fakes::FAKE;
        check(
            ee::s08::f_00299e28__generated,
            ssx_audio_system_on_race_position_change,
            200,
            &|c, seed| {
                let mut r = audio::world(c, seed, 0x29_9e28, 0x0029_9e28);
                // BELibrary_GetInterface: interface table entry 0 with a
                // fake method.
                let iface = DATA + 0x26_1000;
                let vt = DATA + 0x26_1100;
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.bus.write32(0x0053_08a8, iface);
                c.bus.write32(iface + 4, vt);
                c.bus.write16(vt + 8, 0);
                c.bus.write32(vt + 0xc, FAKE + 0x40);
                c.bus.write8(AUDIO_RACE_TYPE_CHECK, (seed % 3 == 0) as u8);
                c.bus.write8(0x0053_5c12, if seed % 9 == 0 { 2 } else { 4 });
                c.bus.write32(
                    audio::MODULE + GAME_MODULE_RACE_TYPE,
                    if seed % 11 == 5 { 2 } else { 4 },
                );
                let race_man = DATA + 0x25_3000;
                c.bus.write32(audio::MODULE + 0xc, race_man);
                c.bus.write32(race_man + RACE_MAN_COUNT_A, 1 + seed % 2);
                c.bus.write32(
                    race_man + RACE_MAN_COUNT_B,
                    if seed % 13 == 4 { 0 } else { 2 },
                );
                // Tracked-sound ring: capacity 8, empty.
                c.bus.write32(audio::SYS + 0x597c, 8);
                c.bus.write32(audio::SYS + 0x5984, 3);
                c.bus.write32(audio::SYS + 0x5988, 3);
                // (*sndRoot)->time.
                c.bus.write32(audio::MONITOR + 0x260, r.next_u32() % 10000);
                // RiderSay207D: speech mode 2 (its early out) mostly.
                let speech = DATA + 0x26_2000;
                c.bus.write32(GAME_GP - 0x84c, speech);
                c.bus
                    .write32(speech + 0x550, if seed % 10 == 7 { 0 } else { 2 });
                // Arguments: passed / passer riders, limit, position.
                let other = DATA + 0x24_1000;
                c.g[A2][0] = if seed % 4 == 0 { 0 } else { sx(other) };
                c.g[A3][0] = if seed % 5 == 0 { 0 } else { sx(other + 0x1000) };
                c.g[T0][0] = (r.next_u32() % 4) as u64;
                c.g[T1][0] = if seed % 2 == 0 {
                    0
                } else {
                    (r.next_u32() % 4) as u64
                };
            },
        );
    }

    /// SSXAudioSystem_IsTrackedSoundPlaying: rings of several sizes (also 0
    /// and empty), records whose key / subkey match or not, handles 0, -1
    /// and small ids with or without a live bank monitor instance (fixture
    /// as in `ssx_audio_system_update_tracked_sounds_matches`).
    #[test]
    fn ssx_audio_system_is_tracked_sound_playing_matches() {
        check(
            ee::s08::f_00295028__generated,
            ssx_audio_system_is_tracked_sound_playing,
            300,
            &|c, seed| {
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
                    let queued = if seed % 7 == 0 {
                        0
                    } else {
                        r.next_u32() % size
                    };
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
                    let at = if size == 0 {
                        buf + 4 * k
                    } else {
                        buf + 4 * ((read + k) % size)
                    };
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
            },
        );
    }

    /// SSXAudioSystem_TrackSound: ring capacities 0..9 (0: the division's
    /// hardware result), random head/tail or a full ring (capacity 1-2: the
    /// handle pop finds it empty), indices past the end. The real
    /// BankMonitor_StopHandle finds no instance (all 64 free).
    #[test]
    fn ssx_audio_system_track_sound_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, fill, DATA};
        const THIS: u32 = DATA + 0x20_0000;
        const MGR: u32 = DATA + 0x21_0000;
        const BANK_OWNER: u32 = DATA + 0x21_0100;
        const INSTANCES: u32 = DATA + 0x22_0000;
        const RING_DATA: u32 = DATA + 0x21_8000;
        check(
            ssx3_ee::s08::f_00295628__generated,
            ssx_audio_system_track_sound,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x29_5628, 0);
                c.pc = 0x0029_5628;
                c.g[A0][0] = sx(THIS);
                c.bus.write32(THIS + TS_MGR, MGR);
                c.bus.write32(MGR, BANK_OWNER);
                c.bus
                    .write32(BANK_OWNER + TS_BANK_MONITOR + 0x8ec, INSTANCES);
                let ring = THIS + TS_RING;
                let cap = r.next_u32() % 10;
                c.bus.write32(ring, cap);
                c.bus.write32(ring + TS_RING_DATA, RING_DATA);
                fill(c, &mut r, RING_DATA, 16);
                let m = cap.max(1);
                let head = r.next_u32() % m;
                let tail = match seed % 4 {
                    0 | 1 => (head + 1) % m,
                    2 => r.next_u32() % m,
                    _ => r.next_u32() % 16,
                };
                c.bus.write32(
                    ring + TS_RING_HEAD,
                    if seed % 9 == 8 {
                        cap + r.next_u32() % 3
                    } else {
                        head
                    },
                );
                c.bus.write32(ring + TS_RING_TAIL, tail);
            },
        );
    }

    /// SSXAudioSystem_StartBoostSound: the MIDI-bank fixture of the
    /// PlaySound tests (real PushPlayParams-style records, real
    /// GetChannelGainPtr / Audio_PlayPushedSound), rider boost on or off,
    /// first start or not (start sound), voice flag on or off. The timer key
    /// matches no timer (the real KillTimersByCallback only scans).
    #[test]
    fn ssx_audio_system_start_boost_sound_matches() {
        use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
        use crate::game::sound::soundsys::tests::midi_bank;
        check(
            ssx3_ee::s08::f_00298d90__generated,
            ssx_audio_system_start_boost_sound,
            200,
            &|c, seed| {
                let mut r = Rng(0x0029_8d90 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0029_8d90;
                c.g[GP][0] = GAME_GP as u64;
                let this = DATA + 0x100;
                let rider = DATA + 0x3000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(rider);
                c.g[A2][0] = if seed % 2 == 0 {
                    0
                } else {
                    sx(1 + r.next_u32() % 4)
                };
                midi_bank(c, &mut r, seed);
                c.bus.write32(rider + BOOST_RIDER_ACTIVE, (seed / 2) % 3);
                c.bus.write32(
                    rider + BOOST_RIDER_AMOUNT,
                    [0, 1, 0x8000_0000, 7][(seed as usize / 6) % 4],
                );
                c.bus.write32(
                    this + BOOST_COUNT,
                    if seed % 5 < 2 { 0 } else { r.next_u32() % 9 },
                );
                c.bus.write32(BOOST_TIMER_KEY, 0x5a5a_a5a5);
                c.bus.write32(BOOST_TIMER_KEY + 4, 0x7e7e_e7e7);
            },
        );
    }

    /// SSXAudioSystem_CalcSurfaceLoopGroup1: rider factors below / above the
    /// threshold, a surface getter that is a fake virtual (forced index
    /// 0..7 via the fake's result; unwinds on some seeds), previous levels
    /// far below / above / near the new level; the real Audio_EvalCurve5 on
    /// sorted five-point curves.
    #[test]
    fn ssx_audio_system_calc_surface_loop_group1_matches() {
        use crate::difftest::fakes::{machine, vtable, LOG_RESULTS};
        const AUDIO: u32 = DATA;
        const RIDER: u32 = DATA + 0x8000;
        const OUT: u32 = DATA + 0xa000;
        check(
            ee::s07::f_00293740__generated,
            ssx_audio_system_calc_surface_loop_group1,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x29_3740, 1);
                c.pc = 0x0029_3740;
                c.bus.write32(GAME_GP - 0x45c0, 0.3f32.to_bits());
                // Curves: x ascending 0..1, then y values.
                for (base, k) in [
                    (SURFACE_LOOP1_PITCH_CURVE, 1.0f32),
                    (SURFACE_LOOP1_VOLUME_CURVE, 0.5),
                ] {
                    for i in 0..5u32 {
                        c.bus.write32(base + 4 * i, (i as f32 * 0.25).to_bits());
                        c.bus
                            .write32(base + 0x14 + 4 * i, (k * (i * i) as f32 + 0.1).to_bits());
                    }
                }
                let f = |r: &mut Rng, lo: f32, hi: f32| {
                    lo + (r.next_u32() % 1000) as f32 / 1000.0 * (hi - lo)
                };
                c.bus.write32(RIDER + 0x750, f(&mut r, 0.0, 0.4).to_bits());
                c.bus.write32(RIDER + 0x754, f(&mut r, 0.0, 0.4).to_bits());
                c.bus.write32(RIDER + 0x758, f(&mut r, 0.0, 1.0).to_bits());
                c.bus.write32(RIDER + 0x75c, f(&mut r, 0.0, 0.6).to_bits());
                let vt = vtable(c, &mut r, &[0x38]);
                c.bus.write32(RIDER + 0x6c0, vt);
                let index = r.next_u32() % 8;
                // The fake's forced result (zero would mean "a pointer", so the
                // index is 1..8).
                c.bus.write32(LOG_RESULTS, index + 1);
                for i in 0..10u32 {
                    let prev = match (seed + i) % 3 {
                        0 => f(&mut r, -2.0, -1.0),
                        1 => f(&mut r, 1.5, 3.0),
                        _ => f(&mut r, 0.0, 1.0),
                    };
                    c.bus
                        .write32(AUDIO + SURFACE_LOOP1_LEVELS + 4 * i, prev.to_bits());
                }
                c.g[A0][0] = sx(AUDIO);
                c.g[A1][0] = sx(RIDER);
                c.g[A2][0] = sx(OUT);
                c.g[A3][0] = sx(OUT + 4);
            },
        );
    }

    // audio is defined in this module

    /// Views for `FindViewInRange` / `GetListenerCount`.
    pub(crate) const PWI_VIEWS: u32 = DATA + 0x28_0000;

    pub(crate) const PWI_VIEW: u32 = DATA + 0x28_1000;

    /// The controller's position (the fakes' forced result).
    pub(crate) const PWI_POS: u32 = DATA + 0x28_2000;

    /// `SSXAudioSystem_PlayWipeoutImpact` on the shared audio world: the
    /// controller flag from a fake (non-zero) or the real zero leaf, local or
    /// not; no view / a far view (early out) or a view at the rider; the
    /// real curve or one spanning negative to >127 volumes; rider states 2/3
    /// (fixed sample) or the real `Audio_Rand`; human/local and mode byte 2
    /// (real `IsReplayPlaying`, in and out of replays); the real push,
    /// gain, listener lookup and `Audio_PlayPushedSound` (no bank). Some
    /// seeds unwind at a fake call.
    #[test]
    fn ssx_audio_system_play_wipeout_impact_matches() {
        check_lohi(
            ee::s08::f_00296310__generated,
            ssx_audio_system_play_wipeout_impact,
            150,
            &|c, seed| {
                let mut r = audio::world(c, seed, 0x29_6310, 0x0029_6310);
                let rider = audio::RIDER;
                // Controller vtable: position (0x28) and flag (0x48) fakes.
                let vt = vtable(c, &mut r, &[0x28, 0x48]);
                let flag_is_zero = seed % 4 == 3;
                if flag_is_zero {
                    c.bus.write32(vt + 0x4c, 0x0014_06c0); // returns 0
                }
                c.bus.write32(rider + wipeout_rider::CONTROLLER, vt);
                c.bus.write32(LOG_RESULTS, PWI_POS);
                c.bus.write32(LOG_RESULTS + 4, PWI_POS);
                // Views: one, at the rider or far away; none sometimes.
                let count = if !flag_is_zero && seed % 9 == 4 { 0 } else { 1 };
                c.bus.write32(audio::MODULE + 0x84, PWI_VIEWS);
                c.bus.write32(PWI_VIEWS + 0x10, count);
                c.bus.write32(PWI_VIEWS + 4, PWI_VIEW);
                let pos = [r.f32(50.0), r.f32(50.0), r.f32(50.0), 1.0f32.to_bits()];
                put_vec(c, PWI_POS, pos);
                let far = seed % 8 == 5;
                let view = if far {
                    [1.0e4f32.to_bits(), pos[1], pos[2], pos[3]]
                } else {
                    pos
                };
                put_vec(c, PWI_VIEW + 0x20, view);
                // Volume curve: the game's, or keys 0..40 with values -50..300.
                if seed % 3 != 0 {
                    for (i, v) in [
                        0.0f32, 10.0, 20.0, 30.0, 40.0, -50.0, 0.0, 100.0, 200.0, 300.0,
                    ]
                    .into_iter()
                    .enumerate()
                    {
                        c.bus.write32(0x0048_2da0 + 4 * i as u32, v.to_bits());
                    }
                }
                let speed = match seed % 6 {
                    0 => r.edge_f32(),
                    _ => r.f32(45.0),
                };
                c.bus.write32(rider + wipeout_rider::SPEED, speed);
                c.bus.write32(
                    rider + wipeout_rider::STATE,
                    [2, 3, 0, 1, 7][(seed / 2 % 5) as usize],
                );
                c.bus
                    .write32(rider + wipeout_rider::LISTENER, r.next_u32() % 4);
                c.bus
                    .write8(WIPEOUT_MODE_BYTE, if seed % 5 < 3 { 2 } else { 1 });
                if seed % 10 == 7 {
                    c.bus.write32(LOG_UNWIND_AT, r.next_u32() % 4);
                }
            },
        );
    }

    /// `SSXAudioSystem_PlayGrabTrickSound`: the MIDI bank fixture of the
    /// rail-change sibling (`midi_bank`: record arrays, cursor 0..3, the
    /// channel state at this+0x118); the rider's pending flag (+0x874) and
    /// count (+0x87c) cover "nothing pending", "count 0" and "play" (memset,
    /// GetChannelGainPtr and Audio_PlayPushedSound run for real).
    #[test]
    fn ssx_audio_system_play_grab_trick_sound_matches() {
        use crate::difftest::{check, scramble, Rng};
        use crate::game::sound::soundsys::tests::midi_bank;
        check(
            ee::s08::f_0029a530__generated,
            ssx_audio_system_play_grab_trick_sound,
            200,
            &|c, seed| {
                let mut r = Rng(0x0029_a530 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0029_a530;
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x3000) as u64; // rider
                midi_bank(c, &mut r, seed);
                c.bus.write32(DATA + 0x3000 + 0x874, (seed % 3) as u32);
                c.bus.write32(DATA + 0x3000 + 0x87c, (seed % 5) as u32);
            },
        );
    }

    /// SSXAudioSystem_PlayObjectSound: sound 0, sound 0x66 (the back end's
    /// interface getter is a fake that may unwind; the current track's flag
    /// word 0 or set), suppressed or not, the object already tracked (one
    /// queued record with its key and handle 0) or not, and every bank range.
    /// The voice records are the MIDI-bank fixture of the PlaySound tests
    /// (real GetChannelGainPtr / Audio_PlayPushedSound); one listener (the
    /// real IsCloserToSecondView returns at once); TrackSound finds no
    /// playing instance.
    #[test]
    fn ssx_audio_system_play_object_sound_matches() {
        use crate::difftest::fakes::{machine, vtable};
        use crate::difftest::{check, fill, DATA, GAME_GP};
        use crate::game::sound::soundsys::tests::midi_bank;
        const THIS: u32 = DATA + 0x100;
        const OBJ: u32 = DATA + 0x4_8000;
        const WORLD: u32 = DATA + 0x4_4000;
        const INSTANCES: u32 = DATA + 0x4_0000;
        const RING_DATA: u32 = DATA + 0x5_0000;
        const BE_LIBRARY: u32 = DATA + 0x5_8000;
        const BE_OBJ: u32 = DATA + 0x5_8100;
        const BE_INTERFACES: u32 = 0x0053_08a8;
        const TRACK_FLAG: u32 = 0x0043_d950;
        const RING: u32 = 0x597c;
        check(
            ee::s08::f_002974a0__generated,
            ssx_audio_system_play_object_sound,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0029_74a0, 1);
                c.pc = 0x0029_74a0;
                midi_bank(c, &mut r, seed);
                let sound = [
                    0u32, 0x66, 0x66, 5, 99, 100, 120, 149, 150, 170, 199, 200, 230,
                ][(seed % 13) as usize];
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = sx(OBJ);
                c.g[A2][0] = sound as u64;
                fill(c, &mut r, OBJ + 0x40, 4);
                // Object sounds on (negative) or suppressed.
                let suppress = match (seed / 13) % 5 {
                    0 => 0,
                    1 => r.next_u32() & 0x7fff_ffff,
                    _ => 0x8000_0000 | r.next_u32(),
                };
                c.bus.write32(THIS + POS_SUPPRESS, suppress);
                // Sound 0x66: the back end and the current track's flag.
                c.bus.write32(GAME_GP - 0x1f04, BE_LIBRARY);
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.bus.write32(BE_INTERFACES, BE_OBJ);
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(BE_OBJ + 4, vt);
                c.bus
                    .write32(TRACK_FLAG, if seed % 26 == 1 { 0 } else { 1 });
                // One listener: IsCloserToSecondView returns 0 at once.
                c.bus.write32(GAME_GP - 0x848, WORLD);
                // The tracked-sound ring (10 words): empty, or one record
                // for this object (key, handle 0, ..., subkey 0).
                c.bus.write32(THIS + RING, 10);
                c.bus.write32(THIS + RING + 4, RING_DATA);
                let read = r.next_u32() % 10;
                let tracked = seed % 7 == 3;
                let write = if tracked { (read + 5) % 10 } else { read };
                c.bus.write32(THIS + RING + 8, write);
                c.bus.write32(THIS + RING + 0xc, read);
                for k in 0..10 {
                    c.bus.write32(RING_DATA + 4 * k, r.next_u32());
                }
                if tracked {
                    for k in 0..5 {
                        let w = if k == 0 { OBJ } else { 0 };
                        c.bus.write32(RING_DATA + 4 * ((read + k) % 10), w);
                    }
                }
                // The bank monitor's instances (all free).
                c.bus.write32(DATA + 0x700 + 0x1d8 + 0x8ec, INSTANCES);
            },
        );
    }

    pub(crate) use crate::game::ealib::seg_2dcae8::strchr;

    /// `SSXAudioSystem_StartObjectLoop` on the audio world fixture of the
    /// soundsys tests (real memset, GetChannelGainPtr, SetRadiusVanish and
    /// `Audio_PlayPushedSound` with no bank loaded; the real
    /// `BankMonitor_StopHandle` finds no instance). Sound ids 0, 0x66,
    /// 201, every bank range and a negative one; track record word 0, 3
    /// or 8 (the real cBE_getBE / GetBELibrary and track-record getter, a
    /// fake GetInterface method); the gate non-negative; no scene node;
    /// slots all free, all in use, mixed, with or without a slot already
    /// playing this sound on this object; bone -1 (node transform) or a
    /// bone the node has (fake) or lacks (a real leaf returning 0: the
    /// object's position).
    #[test]
    fn ssx_audio_system_start_object_loop_matches() {
        use crate::difftest::fakes::{vtable, FAKE};
        use crate::difftest::{check_lohi, fill, DATA, GAME_GP};
        use crate::game::sound::soundsys::tests::audio;
        const OBJ: u32 = DATA + 0x2a_0000;
        const OTHER_OBJ: u32 = DATA + 0x2a_0800;
        const NODE: u32 = DATA + 0x2a_1000;
        const INSTANCES: u32 = DATA + 0x2b_0000;
        const GAINS: u32 = DATA + 0x2c_0000;
        const PRIORITIES: u32 = DATA + 0x2c_1000;
        const TRACK_RECORD: u32 = 0x0043_d950;
        const RETURNS_ZERO: u32 = 0x0014_06c0;
        const SOUNDS: [i32; 13] = [0, 0x66, 0x66, 5, 99, 100, 149, 150, 199, 200, 201, 250, -3];
        check_lohi(
            ee::s08::f_00297950__generated,
            ssx_audio_system_start_object_loop,
            260,
            &|c, seed| {
                let mut r = audio::world(c, seed, 0x29_7950, 0x0029_7950);
                let sys = audio::SYS;
                // BELibrary_GetInterface: interface 0 with a fake method;
                // the current track (0) and its record word.
                let iface = DATA + 0x26_1000;
                let ivt = DATA + 0x26_1100;
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.bus.write32(0x0053_08a8, iface);
                c.bus.write32(iface + 4, ivt);
                c.bus.write16(ivt + 8, 0);
                c.bus.write32(ivt + 0xc, FAKE + 0x40);
                c.bus.write32(0x0053_5c08, 0);
                c.bus
                    .write32(TRACK_RECORD, [0u32, 8, 3, 8][(r.next_u32() % 4) as usize]);
                // The monitor's gain / priority tables and instances (free).
                c.bus.write32(audio::MONITOR + SOL_MONITOR_GAINS, GAINS);
                c.bus
                    .write32(audio::MONITOR + SOL_MONITOR_PRIORITIES, PRIORITIES);
                c.bus
                    .write32(audio::MONITOR + MONITOR_PUSH_QUEUE + 0x8ec, INSTANCES);
                // The gate.
                let gate = if seed % 13 == 6 {
                    r.next_u32() & 0x7fff_ffff
                } else {
                    0x8000_0000 | r.next_u32()
                };
                c.bus.write32(sys + SOL_GATE, gate);
                // The object, its node and the node's vtable.
                let sound = SOUNDS[(r.next_u32() % SOUNDS.len() as u32) as usize];
                let node = if seed % 17 == 9 { 0 } else { NODE };
                c.bus.write32(OBJ + SOL_OBJ_NODE, node);
                fill(c, &mut r, OBJ + SOL_OBJ_POSITION, 4);
                let nvt = vtable(
                    c,
                    &mut r,
                    &[SOL_VT_TRANSFORM, SOL_VT_HAS_BONE, SOL_VT_BONE_TRANSFORM],
                );
                c.bus.write32(NODE + 0xc, nvt);
                if r.next_u32() % 3 == 0 {
                    c.bus.write32(nvt + SOL_VT_HAS_BONE + 4, RETURNS_ZERO);
                }
                let bone = if r.next_u32() % 3 == 0 {
                    -1i32
                } else {
                    (r.next_u32() % 4) as i32
                };
                // The slots.
                let mode = r.next_u32() % 4;
                let same_sound_odds = if r.next_u32() % 2 == 0 { 3 } else { 0 };
                for k in 0..SOL_SLOT_COUNT {
                    let slot = sys + SOL_SLOTS + k * SOL_SLOT_SIZE;
                    let active = match mode {
                        0 => 0,
                        1 => 1 + r.next_u32() % 2,
                        _ => r.next_u32() % 3,
                    };
                    let object = if r.next_u32() % 2 == 0 {
                        OBJ
                    } else {
                        OTHER_OBJ
                    };
                    let id = if same_sound_odds != 0 && r.next_u32() % same_sound_odds == 0 {
                        sound as u32
                    } else {
                        0x1000 + r.next_u32() % 300
                    };
                    c.bus.write32(slot + SOL_ACTIVE, active);
                    c.bus.write32(slot + SOL_OBJECT, object);
                    c.bus.write32(slot + SOL_SOUND, id);
                    c.bus.write32(slot + SOL_HANDLE, r.next_u32() % 0x100);
                }
                c.g[A0][0] = sx(sys);
                c.g[A1][0] = sx(OBJ);
                c.g[A2][0] = sx(sound as u32);
                c.g[A3][0] = sx(bone as u32);
            },
        );
    }

    /// SSXAudioSystem_StartWipeoutLoop2 on the MIDI-bank fixture of the
    /// PlaySound tests (real record arrays, real GetChannelGainPtr,
    /// GetRiderListenerIndex and PlayPushedSound), a world with one view
    /// (real FindViewInRange: the view near or far from the rider's position
    /// from a fake slot-0x28 method), the local-rider method a fake (true)
    /// or a real `return 0` leaf, listener flags on/off, speech mode 2 or
    /// not, replay playing or not, intensities below/inside/above the curve
    /// (volume clamped at 0 and 127), and the loop already playing.
    #[test]
    fn ssx_audio_system_start_wipeout_loop2_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE, LOG_RESULTS};
        use crate::difftest::{put_vec, put_words, DATA, GAME_GP};
        use crate::game::sound::soundsys::tests::midi_bank;
        const AUDIO: u32 = DATA + 0x100; // midi_bank's SSXAudioSystem
        const RIDER: u32 = DATA + 0x2_0000;
        const RIDER_AUDIO: u32 = DATA + 0x2_1000;
        const VTABLE: u32 = DATA + 0x2_2000;
        const POS: u32 = DATA + 0x2_3000;
        const WORLD: u32 = DATA + 0x2_4000;
        const GAME_MODULE: u32 = DATA + 0x2_5000;
        const VIEW_LIST: u32 = DATA + 0x2_6000;
        const VIEW: u32 = DATA + 0x2_7000;
        const REPLAY: u32 = DATA + 0x2_8000;
        const BE_LIB: u32 = DATA + 0x2_9000;
        const BE_IFACE: u32 = DATA + 0x2_a000;
        const BE_VTABLE: u32 = DATA + 0x2_b000;
        const RETURN_ZERO: u32 = 0x0014_06c0;
        let coord =
            |r: &mut crate::difftest::Rng| ((r.next_u32() % 40_000) as f32 - 20_000.0).to_bits();
        covered(
            ee::s08::f_00296e80__generated,
            ssx_audio_system_start_wipeout_loop2,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x29_6e80, 2);
                c.pc = 0x0029_6e80;
                midi_bank(c, &mut r, seed);
                c.g[A0][0] = sx(AUDIO);
                c.g[A1][0] = sx(RIDER);
                c.bus.write32(RIDER + WL2_RIDER_AUDIO, RIDER_AUDIO);
                // The loop handle: already playing on some seeds.
                let handle = if seed % 6 == 0 { seed } else { u32::MAX };
                c.bus.write32(RIDER_AUDIO + WL2_HANDLE, handle);
                // Audio object vtable: slot 0x48 a fake (true) or a real
                // `return 0`; slot 0x28 a fake returning the position.
                c.bus.write32(RIDER_AUDIO + WL2_AUDIO_OBJ, VTABLE);
                c.bus
                    .write16(VTABLE + WL2_SLOT_IS_LOCAL, (r.next_u32() & 0x30) as u16);
                let is_local = if seed % 2 == 0 { FAKE } else { RETURN_ZERO };
                c.bus.write32(VTABLE + WL2_SLOT_IS_LOCAL + 4, is_local);
                c.bus
                    .write16(VTABLE + WL2_SLOT_GET_POS, (r.next_u32() & 0x30) as u16);
                c.bus.write32(VTABLE + WL2_SLOT_GET_POS + 4, FAKE + 0x10);
                c.bus.write32(LOG_RESULTS, POS);
                c.bus.write32(LOG_RESULTS + 4, POS);
                let p = [coord(&mut r), coord(&mut r), coord(&mut r), 0];
                put_vec(c, POS, p);
                // Intensity -5..45 over the curve's 0..40 range.
                let intensity = ((r.next_u32() % 500) as f32 / 10.0 - 5.0).to_bits();
                c.bus.write32(RIDER_AUDIO + WL2_INTENSITY, intensity);
                c.bus
                    .write32(RIDER_AUDIO + WL2_LISTENER_INDEX, r.next_u32() % 4);
                c.bus
                    .write32(RIDER_AUDIO + WL2_HAS_LISTENER, (seed % 3 != 0) as u32);
                let valid = if (seed / 3) % 3 != 0 {
                    1 + r.next_u32() % 3
                } else {
                    0
                };
                c.bus.write32(RIDER_AUDIO + WL2_LISTENER_VALID, valid);
                // The volume curve (times, then values) and the record default.
                let curve = [
                    0.0f32, 10.0, 20.0, 30.0, 40.0, -40.0, 30.0, 90.0, 127.5, 300.0,
                ];
                put_words(c, WL2_VOLUME_CURVE, &curve.map(f32::to_bits));
                put_words(c, WL2_RECORD_DEFAULT, &[r.next_u32(), r.next_u32()]);
                c.bus
                    .write8(WL2_SPEECH_MODE, if seed % 4 == 3 { 1 } else { 2 });
                // World -> game module -> one view near or far.
                c.bus.write32(GAME_GP - 0x848, WORLD);
                c.bus.write32(WORLD + 0x84, GAME_MODULE);
                c.bus.write32(GAME_MODULE + 0x84, VIEW_LIST);
                c.bus.write32(VIEW_LIST + 0x10, 1);
                c.bus.write32(VIEW_LIST + 4, VIEW);
                let near = seed % 5 < 2;
                let v = if near {
                    let d = |x: u32| (f32::from_bits(x) + 50.0).to_bits();
                    [d(p[0]), d(p[1]), p[2], 0]
                } else {
                    [coord(&mut r), coord(&mut r), coord(&mut r), 0]
                };
                put_vec(c, VIEW + 0x20, v);
                // Replay state (GameModule+0x28 -> replay mode word).
                let replay = if seed % 7 < 3 { 0 } else { REPLAY };
                c.bus.write32(GAME_MODULE + 0x28, replay);
                c.bus.write32(REPLAY, r.next_u32() % 14);
                // BELibrary singleton, interface 0 with a real `return 0` slot.
                c.bus.write32(GAME_GP - 0x1f04, BE_LIB);
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.bus.write32(0x0053_08a8, BE_IFACE);
                c.bus.write32(BE_IFACE + 4, BE_VTABLE);
                c.bus.write16(BE_VTABLE + 8, 0);
                c.bus.write32(BE_VTABLE + 0xc, RETURN_ZERO);
            },
        );
    }

    /// `SSXAudioSystem_IsSpeechVolumeOn` with the real
    /// `SSXAudioSystem_GetChannelVolume`: the speech volume (+0x62fc) is
    /// +-0, a denormal (compares equal to 0) or a random float.
    #[test]
    fn ssx_audio_system_is_speech_volume_on_matches() {
        check(
            ee::s08::f_0029f0a0__generated,
            ssx_audio_system_is_speech_volume_on,
            200,
            &|c, seed| {
                let mut r = Rng(0x29f0a0 ^ (seed as u64) << 16);
                scramble(c, &mut r);
                c.pc = 0x0029_f0a0;
                let volume = match seed % 4 {
                    0 => (r.next_u32() & 1) << 31,
                    1 => r.next_u32() & 0x807f_ffff,
                    _ => r.edge_f32(),
                };
                c.bus.write32(DATA + 0x40 + 0x62bc, volume);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    /// The world of [`super::ssx_audio_system_rider_say_207d`]: the audio
    /// system, riders a / b / c (c only as the followed rider), their
    /// scripting-object vtables (real leaves: `RiderScriptObj_GetSelf`
    /// returns the object + delta, 0x1406c0 returns 0), the speech
    /// settings, the game module with one listener view, the race manager,
    /// the speech system and its event contexts, and the BE singletons.
    pub(crate) fn rs207d_world(c: &mut ssx3_ee_rt::Cpu, seed: u32) -> Rng {
        const SYS: u32 = DATA + 0x20_0000;
        const RIDER_A: u32 = DATA + 0x21_0000;
        const RIDER_B: u32 = DATA + 0x21_1000;
        const RIDER_C: u32 = DATA + 0x21_2000;
        const VTABLES: u32 = DATA + 0x21_4000;
        const SETTINGS: u32 = DATA + 0x21_5000;
        const GAME: u32 = DATA + 0x21_6000;
        const MODULE: u32 = DATA + 0x21_7000;
        const VIEWS: u32 = DATA + 0x21_8000;
        const VIEW: u32 = DATA + 0x21_8100;
        const RACE_MAN: u32 = DATA + 0x21_9000;
        const STREAMS: u32 = DATA + 0x21_a000;
        const STREAM: u32 = DATA + 0x21_a100;
        const REPLAY: u32 = DATA + 0x21_a200;
        const BE_DUMMY: u32 = DATA + 0x21_b000;
        const IFACE: u32 = DATA + 0x21_b100;
        const IFACE_VT: u32 = DATA + 0x21_b200;
        const GET_SELF: u32 = 0x0014_0ce0;
        const RETURN_0: u32 = 0x0014_06c0;
        const F_ONE: u32 = 0x3f80_0000;

        let mut r = machine(c, seed, 0x2a_0560, 12);
        elf_image(c);
        c.pc = 0x002a_0560;
        let w = |c: &mut ssx3_ee_rt::Cpu, a: u32, v: u32| c.bus.write32(a, v);
        let gp = |off: i32| (GAME_GP as i32 + off) as u32;
        // BE library: instance, interface 0 with a fake method, singletons.
        w(c, gp(-0x1f04), BE_DUMMY);
        w(c, gp(-0x1efc), 0);
        w(c, 0x0053_08a8, IFACE);
        w(c, IFACE + 4, IFACE_VT);
        w(c, IFACE_VT + 8, 0);
        w(c, IFACE_VT + 0xc, FAKE + 0x40);
        for (k, off) in [0x1f3ci32, 0x1f30, 0x1f2c, 0x1ea4].into_iter().enumerate() {
            w(c, gp(-off), BE_DUMMY + 0x400 + 0x40 * k as u32);
        }
        // Player slots 0/1/2 -> race records; a character-info byte set for
        // a (seed % 17 == 3) or b (== 4); random rivalry profiles.
        for p in 0..3u32 {
            w(c, 0x0053_05b0 + 4 * p, p);
            let rec = 0x0053_5b20 + 0x1c * p;
            w(
                c,
                rec + 0xc,
                if r.next_u32() % 3 == 0 {
                    u32::MAX
                } else {
                    r.next_u32() % 3
                },
            );
            w(c, rec + 0x10, r.next_u32());
            c.bus.write8(rec + 0x11, (r.next_u32() % 4) as u8);
        }
        c.bus.write8(0x0053_4fe0 + 0x12, (seed % 17 == 3) as u8);
        c.bus
            .write8(0x0053_4fe0 + 0x1c + 0x12, (seed % 17 == 4) as u8);
        // Speech volume (channel 4) on unless seed % 9 == 3; speech mode 2.
        w(
            c,
            SYS + 0x62bc + 0x40,
            if seed % 9 == 3 { 0 } else { F_ONE },
        );
        w(c, gp(-0x84c), SETTINGS);
        w(c, SETTINGS + 0x550, if seed % 10 == 7 { 0 } else { 2 });
        // Game module, replay (playing for seed % 13 == 5), listener view.
        w(c, gp(-0x848), GAME);
        w(c, GAME + 0x84, MODULE);
        w(c, MODULE + 0x28, if seed % 13 == 5 { REPLAY } else { 0 });
        w(c, REPLAY, 3);
        w(c, MODULE + 0xc, RACE_MAN);
        w(c, MODULE + 0x84, VIEWS);
        w(c, VIEWS + 0x10, 1);
        w(c, VIEWS + 4, VIEW);
        put_vec(c, VIEW + 0x20, [0; 4]);
        // Race manager: time vs. the last pass, the followed-rider list,
        // rider count 2 (rival lines) or 3 (BE rivalry).
        w(c, RACE_MAN + 8, 1000);
        w(c, SYS + 0x582c, if seed % 19 == 5 { 900 } else { 0 });
        w(c, RACE_MAN + 0x78, if seed % 3 == 0 { 3 } else { 2 });
        w(c, RACE_MAN + 0x7c, 2);
        let followed = [RIDER_A, RIDER_B, RIDER_C, RIDER_A][(seed / 3 % 4) as usize];
        w(c, RACE_MAN + 0x28, followed);
        w(c, RACE_MAN + 0x2c, RIDER_C);
        // Speech system: an instance stream unless seed % 12 == 6, event
        // contexts free, or a's / b's line already live (rejected).
        let speech = SYS + 0x5560;
        w(c, speech + 4, STREAMS);
        w(c, STREAMS, if seed % 12 == 6 { 0 } else { STREAM });
        w(c, STREAM + 4, 0);
        w(c, gp(0x6c4), 0);
        let live: &[u32] = match seed % 5 {
            1 => &[0x2081, 0x2080],
            2 => &[0x207f, 0x207d],
            _ => &[],
        };
        for (i, &code) in live.iter().enumerate() {
            let slot = speech + 0x18 + 0x20 * i as u32;
            w(c, slot, 1);
            w(c, slot + 4, 50);
            w(c, slot + 8, 0);
            w(c, slot + 0xc, code);
        }
        w(c, gp(0x173c), FAKE + 0x80);
        // The riders.
        let gap = [500.0f32, 1000.0, 1500.0, 2000.0][(seed / 2 % 4) as usize];
        for (i, rider) in [RIDER_A, RIDER_B, RIDER_C].into_iter().enumerate() {
            let vt = VTABLES + 0x100 * i as u32;
            let bits = seed >> (2 * i);
            let progress = match (i, seed % 11) {
                (0, 1) => 1.0f32,
                (1, 2) => 0.0,
                _ => -1.0,
            };
            w(c, rider + 0x470, progress.to_bits());
            let heading = if seed % 7 == 0 { r.f32(3.0) } else { 0 };
            w(c, rider + 0x4cc, heading);
            w(c, rider + 0x6c0, vt);
            w(c, rider + 0x86c, i as u32);
            w(c, rider + 0x874, bits & 1);
            w(c, rider + 0x87c, (bits >> 1) & 1);
            w(c, rider + 0xec, r.next_u32() % 4);
            // Slots: direction (+0x7d0), position (+0x7c0), rival player
            // index (0), "is AI" (0 or non-zero).
            c.bus.write16(vt + 0x10, 0x110);
            w(c, vt + 0x14, GET_SELF);
            c.bus.write16(vt + 0x28, 0x100);
            w(c, vt + 0x2c, GET_SELF);
            c.bus.write16(vt + 0x38, 0);
            w(c, vt + 0x3c, RETURN_0);
            c.bus.write16(vt + 0x48, 0);
            w(
                c,
                vt + 0x4c,
                if (bits >> 4) & 1 == 0 {
                    RETURN_0
                } else {
                    GET_SELF
                },
            );
            // Heard by the view unless seed % 23 puts one far away.
            let far = (i == 0 && seed % 23 == 1) || (i == 1 && seed % 23 == 2);
            let x = if far { 5000.0f32 } else { 1.0 + i as f32 };
            put_vec(
                c,
                rider + 0x7c0,
                [x.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits(), 0],
            );
            let d = if i == 0 { gap } else { 0.0 };
            put_vec(c, rider + 0x7d0, [d.to_bits(), d.to_bits(), 0, 0]);
        }
        c.g[A0][0] = SYS as u64;
        c.g[A1][0] = RIDER_A as u64;
        c.g[A2][0] = RIDER_B as u64;
        r
    }

    crate::difftest::events::driven!(strchr_gen, ee::s15::f_00416404__generated);

    crate::difftest::events::driven!(strchr_hand, strchr);

    /// `strchr`: strings of 0..~6000 bytes at every alignment (byte, 8-byte
    /// and 16-byte paths), searching for a byte of the string (early, late,
    /// in the first block), a byte not in it, or the NUL itself (`ch` with
    /// junk above the low byte); events delivered (or unwinding) inside the
    /// loops.
    #[test]
    fn strchr_matches() {
        use crate::difftest::events::{check_events, schedule};
        use crate::difftest::{fill, scramble};
        check_events(strchr_gen, strchr_hand, 600, &|c, seed| {
            let mut r = Rng(0x7374_7263_6872 ^ (seed as u64) << 20);
            scramble(c, &mut r);
            fill(c, &mut r, DATA, 0x2000);
            let s = DATA
                + 0x10 * (r.next_u32() % 32)
                + match r.next_u32() % 3 {
                    0 => 0,
                    1 => 8,
                    _ => r.next_u32() % 16,
                };
            let len = match r.next_u32() % 6 {
                0 => r.next_u32() % 8,
                1 => r.next_u32() % 40,
                2 => [7, 8, 9, 15, 16, 17, 31, 32, 33][(r.next_u32() % 9) as usize],
                3 => r.next_u32() % 300,
                4 => r.next_u32() % 2000,
                _ => 3000 + r.next_u32() % 3000,
            };
            // Bytes 1..=0xfe; 0xff never appears (the "absent" search).
            for i in 0..len {
                let b = match r.next_u32() % 6 {
                    0 => [1u8, 0x7f, 0x80, 0xfe, 2][(r.next_u32() % 5) as usize],
                    _ => (r.next_u32() as u8).clamp(1, 0xfe),
                };
                c.bus.write8(s + i, b);
            }
            c.bus.write8(s + len, 0);
            let ch = match r.next_u32() % 5 {
                0 => 0xff,
                1 => 0,
                _ if len == 0 => 0x41,
                2 => c.bus.read8(s + r.next_u32() % len.min(20)) as u32,
                _ => c.bus.read8(s + r.next_u32() % len) as u32,
            };
            c.g[A0][0] = sx(s);
            c.g[A1][0] = (r.next_u32() as u64) << 32 | (r.next_u32() & 0xffff_ff00 | ch) as u64;
            schedule(c, &mut r, seed, 0x0041_6404)
        });
    }

    /// `CrowdAudio_PitchBendCallback`: the real `CrowdAudio_UpdatePitchBend`
    /// on the voice's owner (the fixture of its own test): not playing,
    /// idle, rising and falling bends; the SND lock hooks are fakes (some
    /// unwind).
    #[test]
    fn crowd_audio_pitch_bend_callback_matches() {
        use crate::difftest::fill;
        const SELF: u32 = DATA + 0x100;
        const VOICE: u32 = DATA + 0x400;
        const TABLE: u32 = DATA + 0x800;
        check(
            ee::s08::f_002a77a8__generated,
            crowd_audio_pitch_bend_callback,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_77a8, 4);
                c.pc = 0x002a_77a8;
                fill(c, &mut r, SELF, 0x70);
                fill(c, &mut r, VOICE, 0x40);
                fill(c, &mut r, TABLE, 0x60);
                c.bus.write32(VOICE + 0x80, SELF);
                c.bus.write32(GAME_GP + 0x608, TABLE);
                c.bus.write32(0x0044_c46c, FAKE);
                c.bus.write32(0x0044_c470, FAKE + 0x10);
                c.bus.write16(0x0050_a8e8 + 0x17e, 0);
                let playing = seed % 6 != 0;
                c.bus.write32(
                    VOICE + 4,
                    if playing {
                        r.next_u32() & 0x7fff_ffff
                    } else {
                        0x8000_0000 | r.next_u32()
                    },
                );
                c.bus.write32(VOICE + 0x54, 1);
                let curve = r.next_u32() % 4;
                c.bus.write32(SELF + 0x184, curve);
                let bend = TABLE + curve * 0x54 + 0x1c;
                let fall = 0.5 + (r.next_u32() % 100) as f32 / 50.0;
                let rise = 0.25 + (r.next_u32() % 100) as f32 / 50.0;
                c.bus.write32(bend + 0x48, fall.to_bits());
                c.bus.write32(bend + 0x4c, rise.to_bits());
                c.bus.write32(bend + 0x50, r.f32(64.0));
                let time = match seed % 5 {
                    0 => -((r.next_u32() % 3) as f32),
                    _ => (r.next_u32() % 400) as f32 / 50.0,
                };
                c.bus.write32(SELF + 0x188, time.to_bits());
                c.f[12] = ((r.next_u32() % 100) as f32 / 60.0).to_bits();
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = sx(VOICE);
            },
        );
    }

    /// `SSXAudioSystem_RiderSay207D`: every early out (speech volume, mode,
    /// replay, character info, pass interval, progress, followed rider,
    /// speech instance, listener views, gap limits), the rival and pass
    /// lines with accepted and rejected contexts; the post hook and the BE
    /// interface method are fakes (some unwind).
    #[test]
    fn ssx_audio_system_rider_say_207d_matches() {
        check(
            ee::s08::f_002a0560__generated,
            ssx_audio_system_rider_say_207d,
            300,
            &|c, seed| {
                rs207d_world(c, seed);
            },
        );
    }

    /// Tick rate: RiderSay207D's "no pass lines yet" wait (race frame
    /// counter minus the race-start stamp at `sys+0x582c`) is 600 ticks at
    /// 60 Hz, 1200 at 120 and 2400 at 240: the same 10 s. Seed 1 passes
    /// every check before the wait and leaves at rider a's progress check
    /// right after it, so a run that passes the wait costs more cycles.
    #[test]
    fn ssx_audio_system_rider_say_207d_pass_wait_follows_the_rate() {
        const SYS: u32 = DATA + 0x20_0000;
        const RACE_MAN: u32 = DATA + 0x21_9000;
        let run = |hz: u32, elapsed: u32| -> u64 {
            let mut c = crate::difftest::fresh();
            rs207d_world(&mut c, 1);
            crate::tick::set_for_test(&mut c, hz);
            c.bus.write32(RACE_MAN + 8, 5000 + elapsed);
            c.bus.write32(SYS + 0x582c, 5000);
            let start = c.cycles;
            ssx_audio_system_rider_say_207d(&mut c);
            c.cycles - start
        };
        let blocked = run(60, 599);
        let passed = run(60, 600);
        assert_ne!(blocked, passed);
        for (hz, wait) in [(120, 1200), (240, 2400)] {
            assert_eq!(run(hz, 600), blocked, "{hz} Hz, 600 ticks");
            assert_eq!(run(hz, wait - 1), blocked, "{hz} Hz, {} ticks", wait - 1);
            assert_eq!(run(hz, wait), passed, "{hz} Hz, {wait} ticks");
        }
    }

    /// CrowdAudio_InstanceSetSourceParam: negative, zero and positive
    /// source indices; active and inactive sources.
    #[test]
    fn crowd_audio_instance_set_source_param_matches() {
        check(
            ee::s08::f_002a5cd8__generated,
            crowd_audio_instance_set_source_param,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x2a5cd8, 0);
                let inst = DATA + 0x100;
                fill(c, &mut r, inst, 0x40);
                let source: i32 = match seed % 4 {
                    0 => -((r.next_u32() % 5) as i32) - 1,
                    1 => 0,
                    _ => (r.next_u32() % 8) as i32,
                };
                if source >= 0 && seed % 3 == 0 {
                    c.bus.write32(inst + 0x20 + 0xc * source as u32, 0);
                }
                c.g[A0][0] = sx(inst);
                c.g[A1][0] = source as i64 as u64;
                c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.pc = 0x002a_5cd8;
            },
        );
    }

    /// `SSXAudioSystem_StartBoardEdgeLoop` on the shared audio world: human
    /// and local or not (early out), mode byte 2 (the real
    /// `IsReplayPlaying`, in and out of replays: the replay listener
    /// fields) or not, surfaces 0..5 (both banks of the real
    /// `GetSurfaceBank`), the real push, memset, gain, listener lookup,
    /// `SetRadiusVanish` and `Audio_PlayPushedSound` (no bank). The
    /// `BELibrary_GetInterface` method is a fake; some seeds unwind there or
    /// in the SND lock hooks. (The volume clamp always sees the 0 it just
    /// stored, so only its in-range path runs.)
    #[test]
    fn ssx_audio_system_start_board_edge_loop_matches() {
        use crate::difftest::check_lohi;
        use crate::difftest::fakes::{FAKE, LOG_UNWIND_AT};
        use crate::game::sound::soundsys::tests::audio;
        check_lohi(
            ee::s07::f_00292508__generated,
            ssx_audio_system_start_board_edge_loop,
            150,
            &|c, seed| {
                let mut r = audio::world(c, seed, 0x29_2508, 0x0029_2508);
                // BELibrary_GetInterface: interface table entry 0 with a
                // fake method.
                let iface = DATA + 0x26_1000;
                let vt = DATA + 0x26_1100;
                c.bus.write32(GAME_GP - 0x1efc, 0);
                c.bus.write32(0x0053_08a8, iface);
                c.bus.write32(iface + 4, vt);
                c.bus.write16(vt + 8, 0);
                c.bus.write32(vt + 0xc, FAKE + 0x40);
                c.bus
                    .write8(OC300_MODE_BYTE, if seed % 5 < 3 { 2 } else { 1 });
                c.bus
                    .write32(audio::RIDER + OC300_RIDER_LISTENER, r.next_u32() % 4);
                c.g[A2][0] = match seed % 7 {
                    6 => (r.next_u32() % 6) as u64 | (r.next_u32() as u64) << 32,
                    k => k as u64 % 6,
                };
                if seed % 10 == 7 {
                    c.bus.write32(LOG_UNWIND_AT, r.next_u32() % 3);
                }
            },
        );
    }

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
                c.bus
                    .write32(WOL_T_RIDER + WOL_RIDER_AUDIO, WOL_T_RIDER_AUDIO);
                // The loop handle: already playing on some seeds.
                let handle = if seed % 6 == 0 { seed } else { u32::MAX };
                c.bus.write32(WOL_T_RIDER_AUDIO + WOL_HANDLE, handle);
                // Audio object vtable: slot 0x48 a fake (true) or a real
                // `return 0`; slot 0x28 a fake returning the position.
                c.bus
                    .write32(WOL_T_RIDER_AUDIO + WOL_AUDIO_OBJ, WOL_T_VTABLE);
                c.bus.write16(
                    WOL_T_VTABLE + WOL_SLOT_IS_LOCAL,
                    (r.next_u32() & 0x30) as u16,
                );
                let is_local = if seed % 2 == 0 {
                    FAKE
                } else {
                    WOL_T_RETURN_ZERO
                };
                c.bus
                    .write32(WOL_T_VTABLE + WOL_SLOT_IS_LOCAL + 4, is_local);
                c.bus.write16(
                    WOL_T_VTABLE + WOL_SLOT_GET_POS,
                    (r.next_u32() & 0x30) as u16,
                );
                c.bus
                    .write32(WOL_T_VTABLE + WOL_SLOT_GET_POS + 4, FAKE + 0x10);
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

    /// A world with the audio app and its listener list (+0x84): no list
    /// (one listener), 0..3 listeners, and two views at positions near or
    /// far from `pos` (so either view can be the closer one, and equal
    /// distances happen).
    #[test]
    fn ssx_audio_is_closer_to_second_view_matches() {
        use crate::difftest::{put_vec, GAME_GP};
        check(
            ee::s07::f_00288940__generated,
            ssx_audio_is_closer_to_second_view,
            300,
            &|c, seed| {
                let mut r = Rng(0x2889_4011 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0028_8940;
                c.g[GP][0] = sx(GAME_GP);
                let world = DATA;
                let app = DATA + 0x100;
                let list = DATA + 0x200;
                let pos = DATA + 0x400;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, app);
                let has_list = seed % 7 != 0;
                c.bus.write32(app + 0x84, if has_list { list } else { 0 });
                c.bus.write32(list + 0x10, seed % 4);
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
                if seed % 5 == 0 {
                    // both views at the same place: equal distances
                    for i in 0..4 {
                        let w = c.bus.read32(DATA + 0x1020 + 4 * i);
                        c.bus.write32(DATA + 0x1060 + 4 * i, w);
                    }
                }
                put_vec(
                    c,
                    pos,
                    [r.f32(spread), r.f32(spread), r.f32(spread), r.f32(1.0)],
                );
                c.g[A0][0] = sx(DATA + 0x800);
                c.g[A1][0] = sx(pos);
            },
        );
    }

    /// The world of `SSXAudioSystem_RiderSay207D`'s test (audio system,
    /// riders, scripting vtables, speech settings, listener view, race
    /// manager, speech system, BE singletons; fakes for the post hook and
    /// the BE interface method, some unwinding) with rider a recovering:
    /// speech volume off, replay, progress after the finish (1.0, 0.0), AI
    /// or local-human combinations against the followed rider, speech
    /// instance not ready, heard or not (far rider, tiny radius), track
    /// group/event mode priorities, all three track categories, and lines
    /// already live (contexts rejected).
    #[test]
    fn ssx_audio_speech_wipeout_recover_matches() {
        use crate::difftest::GAME_GP;
        use crate::game::sound::soundsys::tests::rs207d_world;
        const SYS: u32 = DATA + 0x20_0000;
        const RIDER_A: u32 = DATA + 0x21_0000;
        check(
            ee::s08::f_002a02d8__generated,
            ssx_audio_speech_wipeout_recover,
            300,
            &|c, seed| {
                let mut r = rs207d_world(c, seed);
                c.pc = 0x002a_02d8;
                let progress = match seed % 11 {
                    1 => 1.0f32,
                    8 => 0.0,
                    _ => -1.0 - (r.next_u32() % 100) as f32,
                };
                c.bus.write32(RIDER_A + 0x470, progress.to_bits());
                let radius = if seed % 17 == 9 { 0.0001f32 } else { 10.0 };
                c.bus.write32(GAME_GP + 0x5e8, radius.to_bits());
                // BENewRaceInt_GetTrack: race record 0's track id at
                // 0x43d950 (categories 1, 2, 4 and "none").
                c.bus.write32(0x0053_5c08, 0);
                c.bus
                    .write32(0x0043_d950, [0u32, 2, 4, 0x20][(seed / 7 % 4) as usize]);
                c.bus.write8(0x0053_5c10, if seed % 3 == 1 { 4 } else { 0 });
                if seed % 5 == 1 {
                    let speech = SYS + 0x5560;
                    for (i, code) in [0x2088u32, 0x20a0, 0x20a1].into_iter().enumerate() {
                        let slot = speech + 0x18 + 0x20 * i as u32;
                        c.bus.write32(slot, 1);
                        c.bus.write32(slot + 4, 50);
                        c.bus.write32(slot + 8, 0);
                        c.bus.write32(slot + 0xc, code);
                    }
                }
                c.g[A0][0] = SYS as u64;
                c.g[A1][0] = RIDER_A as u64;
            },
        );
    }

    /// `SSXAudioSystem_StartAmbienceLoop`: already started, or the push
    /// (0..2 entries queued before) and the real `Audio_PlayPushedSound`
    /// with no bank loaded, on the audio test world.
    #[test]
    fn ssx_audio_system_start_ambience_loop_matches() {
        use crate::difftest::check;
        use crate::game::sound::soundsys::tests::audio;
        check(
            ee::s07::f_002910e0__generated,
            ssx_audio_system_start_ambience_loop,
            120,
            &|c, seed| {
                let mut r = audio::world(c, seed, 0x29_10e0, 0x0029_10e0);
                c.bus.write32(
                    audio::SYS + AMBIENCE_LOOP_STARTED,
                    if seed % 4 == 0 {
                        1 + r.next_u32() % 3
                    } else {
                        0
                    },
                );
                c.bus.write32(audio::SYS + AMBIENCE_LOOP_GAIN, r.f32(1.0));
            },
        );
    }

    use crate::difftest::fakes::covered;

    use ssx3_ee_rt::cpu::sx;
    use ssx3_ee_rt::Cpu;

    /// `CrowdAudio_UpdatePitchBend`: a voice with a negative handle (reset;
    /// its fade-out already running so it returns at_m2 once) or a playing one;
    /// bend time left <= 0 (idle) or running, with dt so that the time left
    /// lands in the falling part, the rising part or at_m2/below zero. The
    /// crowd table (gp_m2+0x608) has 4 entries of random curves. SND_Lock and
    /// SND_Unlock run for real and call their callbacks (fakes);
    /// SND_SetPitchBend runs for real with no channels (handle not found).
    #[test]
    fn crowd_audio_update_pitch_bend_matches() {
        use crate::difftest::GAME_GP;
        const SELF: u32 = DATA + 0x100;
        const VOICE: u32 = DATA + 0x400;
        const TABLE: u32 = DATA + 0x800;
        covered(
            ee::s08::f_002a68b0__generated,
            crowd_audio_update_pitch_bend,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_68b0, 4);
                c.pc = 0x002a_68b0;
                fill(c, &mut r, SELF, 0x70);
                fill(c, &mut r, VOICE, 0x10);
                fill(c, &mut r, TABLE, 0x60);
                c.bus.write32(GAME_GP + CROWD_TABLE_GP, TABLE);
                // Lock/unlock callbacks; no sound channels.
                c.bus.write32(0x0044_c46c, FAKE);
                c.bus.write32(0x0044_c470, FAKE + 0x10);
                c.bus.write16(SND_EVENT_TABLE + 0x17e, 0);
                let playing = seed % 6 != 0;
                c.bus.write32(
                    VOICE + CROWD_VOICE_HANDLE,
                    if playing {
                        r.next_u32() & 0x7fff_ffff
                    } else {
                        0x8000_0000 | r.next_u32()
                    },
                );
                c.bus.write32(VOICE + 0x54, 1); // fade-out already running
                let curve = r.next_u32() % 4;
                c.bus.write32(SELF + CROWD_BEND_CURVE, curve);
                let bend = TABLE + curve * CROWD_ENTRY_SIZE + CROWD_ENTRY_BEND;
                let fall = 0.5 + (r.next_u32() % 100) as f32 / 50.0;
                let rise = 0.25 + (r.next_u32() % 100) as f32 / 50.0;
                c.bus.write32(bend + BEND_FALL, fall.to_bits());
                c.bus.write32(bend + BEND_RISE, rise.to_bits());
                c.bus.write32(bend + BEND_DEPTH, r.f32(64.0));
                let time = match seed % 5 {
                    0 => -((r.next_u32() % 3) as f32),
                    _ => (r.next_u32() % 400) as f32 / 50.0,
                };
                c.bus.write32(SELF + CROWD_BEND_TIME, time.to_bits());
                // dt: the whole time (ends exactly at_m2 0), an edge value, or a frame-ish step.
                c.f[12] = match (seed / 5) % 4 {
                    0 => time.to_bits(),
                    1 => r.edge_f32(),
                    _ => ((r.next_u32() % 100) as f32 / 60.0).to_bits(),
                };
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(VOICE);
            },
        );
    }
}
