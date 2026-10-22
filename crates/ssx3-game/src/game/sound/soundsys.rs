//! `src/sound/soundsys.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::ps2f::FpuF32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::abi::r32;
use crate::call::sp;
use crate::call::Flow;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee as ee;
use crate::events;
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::funcs::call_indirect;
use crate::call::Unwound;
use crate::events::deliver_event;
pub(crate) use crate::game::visualfx::boardtrailfx::p;
pub(crate) use crate::game::worldpainter::quadtree::create_with;
pub(crate) use crate::game::fe::festaterewards2::r;
pub(crate) use crate::game::fe::festaterewards2::Ops;
pub(crate) use crate::game::sound::bankmonitor::leave;
pub(crate) use crate::game::world::wscriptcache::DELETE_2 as DELETE;
pub(crate) use crate::game::animation::animsequencer::dtor;
pub(crate) use crate::game::sound::bankmanager::race_type_in_range;
pub(crate) use crate::game::dirtysock::tags2::lw;
pub(crate) use crate::game::be::bestate2::ea;
pub(crate) use crate::game::fe::feasyncfile::gp;
pub(crate) use crate::game::fe::feasyncfile::gp_7 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::lw_at_2 as lw_at;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2;
pub(crate) use crate::game::scripter::ssxscriptengine::sw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::fe::feasyncfile::low_2;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::main::gamestate::gw;
pub(crate) use crate::game::main::gamestate::lw_at_3;
pub(crate) use crate::game::fe::festaterewards::at_4 as at_2;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw_2;
pub(crate) use crate::game::wscript::wscriptman::sw;
pub(crate) use crate::game::fe::feasyncfile::at as at_3;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_3;
pub(crate) use crate::game::fe::feasyncfile::gp_15 as gp_3;
pub(crate) use crate::game::fe::feasyncfile::lw_8 as lw_4;
pub(crate) use crate::game::fe::feasyncfile::sw_6 as sw_2;
pub(crate) use crate::game::fe::feasyncfile::a;
pub(crate) use crate::game::render::graphicsman::RegMem;
pub(crate) use crate::game::fe::fepopup::at as at_4;
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::fe::fepopup::lhu;
pub(crate) use crate::game::fe::fepopup::sh;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::be::beintnetwork::ldl;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0028_7920, ssx_audio_system_get_channel_volume, "SSXAudioSystem_GetChannelVolume"),
    (0x0028_7f00, ssx_audio_system_set_fade_mode, "SSXAudioSystem_SetFadeMode"),
    (0x0028_fea0, midi_banks_base_dtor, "MIDIBanks_BaseDtor"),
    (0x0029_0b58, audio_eval_curve5, "Audio_EvalCurve5"),
    (0x0029_0cc0, ssx_audio_system_play_sound, "SSXAudioSystem_PlaySound"),
    (0x0029_0f58, ssx_audio_system_set_player_trick_score, "SSXAudioSystem_SetPlayerTrickScore"),
    (0x0029_13d8, ssx_audio_system_stop_ambience_loop, "SSXAudioSystem_StopAmbienceLoop"),
    (0x0029_1710, ssx_audio_get_surface_sound_type, "SSXAudioSystem_GetSurfaceSoundType"),
    (0x0029_17b8, ssx_audio_system_get_surface_bank, "SSXAudioSystem_GetSurfaceBank"),
    (0x0029_29d8, ssx_audio_system_restart_all_board_loops, "SSXAudioSystem_RestartAllBoardLoops"),
    (0x0029_2a50, ssx_audio_system_start_rider_board_loops, "SSXAudioSystem_StartRiderBoardLoops"),
    (0x0029_2ae8, ssx_audio_system_kill_rider_timers, "SSXAudioSystem_KillRiderTimers"),
    (0x0029_2b48, ssx_audio_system_stop_board_loops, "SSXAudioSystem_StopBoardLoops"),
    (0x0029_34d0, ssx_audio_get_surface_group, "SSXAudioSystem_GetSurfaceGroup"),
    (0x0029_4880, ssx_audio_system_set_rider_air_flag, "SSXAudioSystem_SetRiderAirFlag"),
    (0x0029_4f48, ssx_audio_system_tick_listener, "SSXAudioSystem_TickListener"),
    (0x0029_7eb8, ssx_audio_system_stop_object_loop, "SSXAudioSystem_StopObjectLoop"),
    (0x0029_80b0, ssxaudiosystem_stop_all_object_loops, "SSXAudioSystem_StopAllObjectLoops"),
    (0x0029_8d00, ssxaudiosystem_play_hit_boost_sound, "SSXAudioSystem_PlayHitBoostSound"),
    (0x0029_9b70, ssx_audio_system_play_rail_change_sound, "SSXAudioSystem_PlayRailChangeSound"),
    (0x0029_b3c0, ssx_audio_system_stop_trick_meter_sound, "SSXAudioSystem_StopTrickMeterSound"),
    (0x0029_b7e0, ssx_audio_system_on_bonus_trick, "SSXAudioSystem_OnBonusTrick"),
    (0x0029_ce28, ssx_audio_system_pause_sfx, "SSXAudioSystem_PauseSfx"),
    (0x0029_ce70, ssx_audio_system_resume_sfx, "SSXAudioSystem_ResumeSfx"),
    (0x0029_d370, ssx_audio_system_start_loop_sfx, "SSXAudioSystem_StartLoopSfx"),
    (0x0029_d678, ssx_audio_system_stop_loop_sfx, "SSXAudioSystem_StopLoopSfx"),
    (0x0029_dbb0, ssx_audio_system_on_mission_abort, "SSXAudioSystem_OnMissionAbort"),
    (0x0029_e578, ssx_audio_system_reset_wipeout_sfx, "SSXAudioSystem_ResetWipeoutSfx"),
    (0x0029_ed18, ssx_audio_system_stop_rider_loops, "SSXAudioSystem_StopRiderLoops"),
    (0x0029_f088, ssx_audio_system_get_current_speech_category, "SSXAudioSystem_GetCurrentSpeechCategory"),
    (0x0029_f0d8, ssx_audio_system_is_dj_speech_on, "SSXAudioSystem_IsDJSpeechOn"),
    (0x0029_f128, audio_is_pa_speech_on, "SSXAudioSystem_IsPASpeechOn"),
    (0x0029_f160, audio_is_arcade_speech_on, "SSXAudioSystem_IsArcadeSpeechOn"),
    (0x0029_f2b0, character_to_speech_mask, "SSXAudioSystem_CharacterToSpeechMask"),
    (0x0029_f5e0, ssx_audio_system_unload_grunt_banks, "SSXAudioSystem_UnloadGruntBanks"),
    (0x002a_1388, ssx_audio_system_rider_say_20b9_callback, "SSXAudioSystem_RiderSay20B9Callback"),
    (0x002a_1778, ssx_audio_system_rider_say_20bd, "SSXAudioSystem_RiderSay20BD"),
    (0x002a_1b58, ssx_audio_system_on_script_rider_speech2, "SSXAudioSystem_OnScriptRiderSpeech2"),
    (0x002a_1b88, ssx_audio_system_on_script_rider_speech01, "SSXAudioSystem_OnScriptRiderSpeech01"),
    (0x002a_1da0, character_to_voice_mask, "SSXAudioSystem_CharacterToVoiceMask"),
    (0x002a_2018, dj_say_20c8, "SSXAudioSystem_DJSay20C8"),
    (0x002a_20d0, dj_say_20c9, "SSXAudioSystem_DJSay20C9"),
    (0x002a_2188, ssx_audio_system_dj_say_20ca, "SSXAudioSystem_DJSay20CA"),
    (0x002a_2260, dj_say_20e7, "SSXAudioSystem_DJSay20E7"),
    (0x002a_2318, dj_say_20e8, "SSXAudioSystem_DJSay20E8"),
    (0x002a_23d0, ssx_audio_system_dj_say_20e9, "SSXAudioSystem_DJSay20E9"),
    (0x002a_24b0, ssx_audio_system_dj_say_20cc, "SSXAudioSystem_DJSay20CC"),
    (0x002a_2568, ssx_audio_system_dj_say20cc_2, "SSXAudioSystem_DJSay20CC_2"),
    (0x002a_2638, dj_say_20ba, "SSXAudioSystem_DJSay20BA"),
    (0x002a_26f0, ssx_audio_system_dj_say20e5, "SSXAudioSystem_DJSay20E5"),
    (0x002a_27d8, ssxaudiosystem_djsay20e6, "SSXAudioSystem_DJSay20E6"),
    (0x002a_2860, ssx_audio_system_dj_say_20cf, "SSXAudioSystem_DJSay20CF"),
    (0x002a_2938, ssx_audio_system_dj_say_2102, "SSXAudioSystem_DJSay2102"),
    (0x002a_2ad0, dj_say_212b, "SSXAudioSystem_DJSay212B"),
    (0x002a_2b88, ssx_audio_system_dj_say_212c, "SSXAudioSystem_DJSay212C"),
    (0x002a_2cf0, ssx_audio_system_dj_say212e, "SSXAudioSystem_DJSay212E"),
    (0x002a_2da0, ssx_audio_system_dj_say212f, "SSXAudioSystem_DJSay212F"),
    (0x002a_3170, ssx_audio_system_play_dj_song_intro, "SSXAudioSystem_PlayDJSongIntro"),
    (0x002a_32b0, ssx_audio_system_pa_say_20c1, "SSXAudioSystem_PASay20C1"),
    (0x002a_3358, ssx_audio_system_pa_say_20c2, "SSXAudioSystem_PASay20C2"),
    (0x002a_3708, ssx_audio_system_pa_say20c5, "SSXAudioSystem_PASay20C5"),
    (0x002a_3b18, ssx_audio_system_arcade_say20a7, "SSXAudioSystem_ArcadeSay20A7"),
    (0x002a_3fd8, ssx_audio_system_is_track_id_group5, "SSXAudioSystem_IsTrackIdGroup5"),
    (0x002a_4040, ssx_audio_system_is_event_mode4, "SSXAudioSystem_IsEventMode4"),
    (0x002a_4078, ssx_audio_system_is_track_group0_event, "SSXAudioSystem_IsTrackGroup0Event"),
    (0x002a_40e8, ssx_audio_system_is_track_group1_event, "SSXAudioSystem_IsTrackGroup1Event"),
    (0x002a_4168, ssx_audio_system_is_track_group2_event, "SSXAudioSystem_IsTrackGroup2Event"),
    (0x002a_41d8, ssx_audio_system_is_track_group3_event, "SSXAudioSystem_IsTrackGroup3Event"),
    (0x002a_4248, ssx_audio_system_is_race_type4_or5, "SSXAudioSystem_IsRaceType4or5"),
    (0x002a_4290, ssx_audio_system_is_race_type4, "SSXAudioSystem_IsRaceType4"),
    (0x002a_4318, ssx_audio_system_is_race_type6_to8, "SSXAudioSystem_IsRaceType6to8"),
    (0x002a_4368, ssx_audio_system_is_race_type9_to11, "SSXAudioSystem_IsRaceType9to11"),
    (0x002a_4550, ssx_audio_system_clear_pending_speech, "SSXAudioSystem_ClearPendingSpeech"),
    (0x002a_4590, ssx_audio_system_reset_race_result_state, "SSXAudioSystem_ResetRaceResultState"),
    (0x002a_45c0, ssx_audio_system_record_race_result, "SSXAudioSystem_RecordRaceResult"),
    (0x002a_4660, ssx_audio_system_store_race_result, "SSXAudioSystem_StoreRaceResult"),
    (0x002a_4718, ssx_audio_system_has_track_changed, "SSXAudioSystem_HasTrackChanged"),
    (0x002a_49e8, ssx_audio_system_start_section_speech_timer, "SSXAudioSystem_StartSectionSpeechTimer"),
    (0x002a_4b68, ssx_audio_system_stop_speech_on_reset, "SSXAudioSystem_StopSpeechOnReset"),
    (0x002a_4ba8, crowd_audio_init_sample_list, "CrowdAudio_InitSampleList"),
    (0x002a_4be0, crowd_audio_delete_sample_list, "CrowdAudio_DeleteSampleList"),
    (0x002a_4ca0, crowd_audio_pick_random_sample, "CrowdAudio_PickRandomSample"),
    (0x002a_4cf8, crowd_audio_free_sample_list, "CrowdAudio_FreeSampleList"),
    (0x002a_4e38, crowd_audio_destroy_instance, "CrowdAudio_DestroyInstance"),
    (0x002a_4e88, crowd_audio_update_instance, "CrowdAudio_UpdateInstance"),
    (0x002a_62f0, crowd_audio_instance_remove_source, "CrowdAudio_InstanceRemoveSource"),
    (0x002a_6430, crowd_audio_instance_remove_all_sources, "CrowdAudio_InstanceRemoveAllSources"),
    (0x002a_67f0, crowd_audio_instance_start_anticipate, "CrowdAudio_InstanceStartAnticipate"),
    (0x002a_6808, crowd_audio_instance_stop_anticipate, "CrowdAudio_InstanceStopAnticipate"),
    (0x002a_6848, crowd_audio_instance_stop_sound, "CrowdAudio_InstanceStopSound"),
    (0x002a_6f38, crowd_audio_unload, "CrowdAudio_Unload"),
    (0x002a_72c0, crowd_audio_decode_source_handle, "CrowdAudio_DecodeSourceHandle"),
    (0x002a_72d8, crowd_audio_remove_source, "CrowdAudio_RemoveSource"),
    (0x002a_7340, crowd_audio_set_source_param, "CrowdAudio_SetSourceParam"),
    (0x002a_73a8, crowd_audio_play_trick, "CrowdAudio_PlayTrick"),
    (0x002a_73d8, crowd_audio_play_fall, "CrowdAudio_PlayFall"),
    (0x002a_7438, crowd_audio_stop_anticipate, "CrowdAudio_StopAnticipate"),
    (0x002a_7468, crowd_audio_push_play_params, "CrowdAudio_PushPlayParams"),
    (0x002a_78e0, ini_parser_get_char, "IniParser_GetChar"),
    (0x002a_79c8, ini_parser_skip_comment, "IniParser_SkipComment"),
    (0x002a_7a20, ini_parser_skip_section_headers, "IniParser_SkipSectionHeaders"),
    (0x002a_7a90, ini_parser_skip_to_eol, "IniParser_SkipToEol"),
    (0x002a_7bf8, ini_parser_find_section, "IniParser_FindSection"),
    (0x002a_7c68, ini_parser_find_section_prefix, "IniParser_FindSectionPrefix"),
    (0x002a_7cf0, ini_parser_read_quoted_string, "IniParser_ReadQuotedString"),
    (0x002a_8070, ini_parser_get_float, "IniParser_GetFloat"),
    (0x002a_80d8, ini_parser_get_int, "IniParser_GetInt"),
    (0x002a_8450, midi_banks_push_play_params, "MIDIBanks_PushPlayParams"),
    (0x002a_8c20, ps2_sound_manager_ctor, "PS2SoundManager_ctor"),
    (0x002a_8c50, ps2_sound_manager_dtor, "PS2SoundManager_dtor"),
    (0x002a_8f50, ps2_sound_manager_shutdown, "PS2SoundManager_Shutdown"),
    (0x002a_8fa0, ps2_sound_manager_create, "PS2SoundManager_Create"),
    (0x002a_9250, ps2_sound_manager_set_spu_mem_base, "PS2SoundManager_SetSpuMemBase"),
    (0x002a_9288, ps2_sound_manager_v07, "PS2SoundManager_v07"),
    (0x002a_92b8, ps2_sound_manager_pop_free_spu_block, "PS2SoundManager_PopFreeSpuBlock"),
    (0x002a_92f8, ps2_sound_manager_free_spu_block, "PS2SoundManager_FreeSpuBlock"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    use super::*;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_LIST_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_LIST_VISIBLE_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_FIND_ROSTER_ENTRY;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_IS_BUDDY_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_IS_BLOCKED_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_IS_TEMP_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_IS_ONLINE_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_IS_SAME_PRODUCT_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_COUNT_MESSAGES_FROM_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_INSTANCE_START;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_ADD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_SKIP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_EMPTY_CROWD_HANDLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_GET_BUFFER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_RIDER_SAY_20B9;
    pub(crate) use crate::game::dirtysock::tags2::callee::CROWD_INSTANCE_PLAY_TRICK;
    pub(crate) use crate::game::dirtysock::tags2::callee::CROWD_INSTANCE_PLAY_FALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::CROWD_INSTANCE_STOP_ANTICIPATE;
    pub(crate) use crate::game::be::beintmission::callee::FIND_MISSION;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_RESET_STATE;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_SET_ALGORITHM_IMMEDIATE;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_SET_ALGORITHM_2 as CHASE_CAM_SET_ALGORITHM;
    pub(crate) use crate::game::be::beintmission::callee::GET_RIDER_PREVIEW;
    pub(crate) use crate::game::be::beintmission::callee::RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::be::beintmission::callee::NET_FE_STATE_CTOR;
    pub(crate) use crate::game::be::beintmission::callee::LOBBY_MENU_REQUEST_JOIN_ROOM;
    pub(crate) use crate::game::be::beintmission::callee::POPUP_CONFIRM_CFG_RESET;
    pub(crate) use crate::game::be::beintmission::callee::REWARD_GALLERY_CHECK_REWARDS_READY;
    pub(crate) use crate::game::be::beintmission::callee::REWARD_GALLERY_POLL_SHAPE_LOAD;
    pub(crate) use crate::game::be::beintmission::callee::BUDDY_MAN_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::be::beintmission::callee::REPLAY_APPLY_CAMERA;
    pub(crate) use crate::game::be::beintmission::callee::RACE_PROGRESS_RESET_ALL;
    pub(crate) use crate::game::be::beintmission::callee::MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_STOP_MAIN_LISTS;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_STOP_AUX_LISTS;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_DEACTIVATE_BINDING;
    pub(crate) use crate::game::be::beintmission::callee::CROWD_AUDIO_STOP_ALL;
    pub(crate) use crate::game::be::beintmission::callee::BANK_MONITOR_STOP_ALL;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_LEVEL_TO_VOLUME;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::be::beintmission::callee::SND_BANK_SET_MEMORY;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_IS_GAME_MUSIC_ENABLED;
    pub(crate) use crate::game::be::beintmission::callee::MUSIC_PLAYER_STOP_SONG;
    pub(crate) use crate::game::be::beintmission::callee::UI_STATE_UPDATE;
    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::be::beintmission::callee::BE_NEW_PLAYER_GET_INBOX;
    pub(crate) use crate::game::be::beintmission::callee::WSCRIPT_MAN_EVENT_QUEUE_POP;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_START_PRERACE_MUSIC;
    pub(crate) use crate::game::be::beintmission::callee::SLOGIN_DELETE;
    pub(crate) use crate::game::be::beintmission::callee::CAMERA_TRIGGER_MAN_UPDATE_MASK1;
    pub(crate) use crate::game::be::beintmission::callee::EA_MEMSET;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_MEM_ALLOC;
    pub(crate) use crate::game::worldpainter::quadtree::callee::BUILTIN_DELETE_2 as BUILTIN_DELETE;
    pub(crate) use crate::game::worldpainter::quadtree::callee::PS2_SOUND_MANAGER_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STREAM_BUFFER_SET_FILE;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STREAM_BUFFER_STOP;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SPEECH_INSTANCE_STOP_STREAM;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SONG_GET_STREAM_STATUS;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SONG_POST_EVENT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SONG_FADE_OUT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SNDSTRM_GET_REQUEST_STATUS;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MEM_CARD_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MEM_CARD_INIT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MEM_CARD_SET_STATE;
    pub(crate) use crate::game::worldpainter::quadtree::callee::CONSOLE_CONFIG_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STRCPY;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MENU_ITEM_MEASURE_ROW;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STATS_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::AI_VFX_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::VFX_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::WORLD_GET_SECTION_SCRIPT_FUNC;
    pub(crate) use crate::game::worldpainter::quadtree::callee::WORLD_OBJ_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MOD_OBJ_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::GET_TRANSFORM_BOUNDS;
    pub(crate) use crate::game::worldpainter::quadtree::callee::INTC_REMOVE_HANDLER;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EMITTER_RENDER;
    pub(crate) use crate::game::worldpainter::quadtree::callee::GRAPHICS_ON_VIF1_DMA_END;
    pub(crate) use crate::game::worldpainter::quadtree::callee::GRAPHICS_MAN_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_ALLOC as EA_MEM_ALLOC_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_SOUND_BASE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR_2 as UI_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_REMOVE_TICK_CALLBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::CW_SCRIPT_CACHE_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_CAMERA_SET_CONTROLLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MENU_ON_LOBBY_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_FORWARD_EVENT_TO_UI;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::LUNO_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_PLAY_CHARACTER_TUNE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_GAME_UTIL_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::CSETUP_MAN_SEND_PACKET;
    pub(crate) use crate::game::dirtysock::tags2::callee::CVOIP_MAN_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_RPC_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_RPC_SET_PARAMS;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_HIGHLIGHT_SLOT_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SAVE_EMIT;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_REQUEST_CARD_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_HANDLE_MUSIC_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_ARCADE_SAY;
    pub(crate) use crate::game::dirtysock::tags2::callee::EATIMER_REMOVE_CALLBACK;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_NAMES;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_ICONS;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SCROLL_ARROWS;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_COUNT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_DISCONNECTED_POPUP_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_LOGIN_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_LOGIN_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_QUICK_PLAY_POPUP_ON_OPTIONS_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_STATS_REFRESH_RESULTS;
    pub(crate) use crate::game::fe::festaterewards2::callee::POPUP_CONFIRM_GET_TEXT_BOX_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::PREVIEW_REWARD_POLL_IMAGE_LOAD;
    pub(crate) use crate::game::fe::festaterewards2::callee::MOUNTAIN_ROOM_UPDATE_EVENT_ICONS;
    pub(crate) use crate::game::fe::festaterewards2::callee::TROPHY_ROOM_LOAD_TROPHY_TEXTURES;
    pub(crate) use crate::game::fe::festaterewards2::callee::SECTION_MAN_FIND_LOGICAL_SECTION;
    pub(crate) use crate::game::fe::festaterewards2::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::APP_MODULE_TRY_EXIT;
    pub(crate) use crate::game::fe::festaterewards2::callee::S_LOGIN_UPDATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::USB_KB_DEFAULT_CONNECT_CB;
    pub(crate) use crate::game::fe::festaterewards2::callee::WORLD_STATE_STREAM_RESET;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_STREAM_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_FIND_NEXT_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_FIND_PREV_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_SNAP_TO_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_SET_STATE_2;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_LIST_ADD_MOVIE;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_LIST_POLL_LOADING;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_ENGINE_UNLOAD_WORLD_SCRIPTS;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_ENGINE_LIST_STOP;
    pub(crate) use crate::game::fe::festaterewards2::callee::CROWD_AUDIO_INSTANCE_STOP_SOUND;
    pub(crate) use crate::game::fe::festaterewards2::callee::SONG_INSTANCE_GET_DUCK_TO_LOOPS;
    pub(crate) use crate::game::fe::festaterewards2::callee::WORLD_TRIGGER_MANAGER_GET_TRIGGER_INFO;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_MAN_FREE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_GET_POOL_BLOCK_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::DL_HEAP_USABLE_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_REPORT_POOL_STATS;
    pub(crate) use crate::game::fe::festaterewards2::callee::PS2_PAD_CREATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::ANIM_OBJECT_V36;
    pub(crate) use crate::game::fe::festaterewards2::callee::DYNAMIC_EMITTER_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_STATE_UPDATE as UI_STATE_UPDATE_2;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SCROLL_SET_WIDE_STRING;
    pub(crate) use crate::game::fe::festaterewards2::callee::TWSCRIPT_CACHE_RELOCATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::EA_MUTEX_LOCK;
    pub(crate) use crate::game::fe::festaterewards2::callee::EA_MUTEX_UNLOCK;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCE_USBKB_SET_ARRANGEMENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCE_USBKB_SET_READ_MODE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEMSET;
    pub(crate) use crate::game::fe::festaterewards2::callee::SPRINTF;
    pub(crate) use crate::game::sound::bankmonitor::callee::GAME_STATE_GOTO_NEXT_2 as GAME_STATE_GOTO_NEXT;
    pub(crate) use crate::game::sound::bankmonitor::callee::PS2_MEMCARD_CREATE;
    pub(crate) use crate::game::sound::bankmonitor::callee::LOBBY_LIST_GET_AT as LOBBY_LIST_GET_AT_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::BUDDY_MAN_CONNECT;
    pub(crate) use crate::game::sound::bankmonitor::callee::STR_CASE_CMP;
    pub(crate) use crate::game::sound::bankmonitor::callee::SLOGIN_MC_GET_PORT_SLOT;
    pub(crate) use crate::game::sound::bankmonitor::callee::MEMCARD_REQUEST_GET_INFO_NOW;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_INT;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_VEC3;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_FLOAT_2 as PATH_DATA_READ_FLOAT;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_LIST_ADD_CHOICE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS;
    pub(crate) use crate::game::sound::bankmonitor::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SSX_AUDIO_SYSTEM_END_SCRIPT_MUSIC;
    pub(crate) use crate::game::sound::bankmonitor::callee::WORLD_PAINTER_SET_SCRIPT_SURFACE_MODE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_ENGINE_GET_INSTANCE as SCRIPT_ENGINE_GET_INSTANCE_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::SND_BANK_MAN_UNLOAD_MEMORY_BANK;
    pub(crate) use crate::game::sound::bankmonitor::callee::MUSIC_PLAYER_GET_SONG_TAG;
    pub(crate) use crate::game::sound::bankmonitor::callee::SPEECH_SYSTEM_STOP_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::BANK_MONITOR_TIMER_KILL;
    pub(crate) use crate::game::sound::bankmonitor::callee::SNDSTRM_SET_PREBUFFER;
    pub(crate) use crate::game::sound::bankmonitor::callee::AUDIO_RAND;
    pub(crate) use crate::game::sound::bankmonitor::callee::SONG_INSTANCE_GET_REQUEST_STATUS;
    pub(crate) use crate::game::sound::bankmonitor::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::sound::bankmonitor::callee::MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_MAN_CALL_TASK_FUNC;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_PROCESS_LIST_REMOVE;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_LIBRARY_GET_INSTANCE_3;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_LIBRARY_GET_INTERFACE as BE_LIBRARY_GET_INTERFACE_3;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_MISSION_INT_GET_COLLECTED_COUNT;
    pub(crate) use crate::game::sound::bankmonitor::callee::ADF_DATA_SOURCE_SET_DATA;
    pub(crate) use crate::game::sound::bankmonitor::callee::EA_FILE_LOAD_DECOMPRESSED;
    pub(crate) use crate::game::sound::bankmonitor::callee::GAME_ANIM_MAP_FIND_HASH;
    pub(crate) use crate::game::sound::bankmonitor::callee::MEMSET_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::CROWD_MAN3D_GROUP_CREATE_MESHES;
    pub(crate) use crate::game::sound::bankmonitor::callee::TEX_MAN_FREE_TEXTURE;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_LIST_COUNT;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_MENU_SET_NAV_BUTTONS;
    pub(crate) use crate::game::sound::bankmonitor::callee::SECTION_ALLOCATOR_DTOR;
    pub(crate) use crate::game::sound::bankmonitor::callee::STREAM_FILE_RELEASE_CHUNK;
    pub(crate) use crate::game::sound::bankmonitor::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_3;
    pub(crate) use crate::game::sound::bankmonitor::callee::RIDER_METRIX_SHOW_HUD_NUMBER;
    pub(crate) use crate::game::sound::bankmonitor::callee::RIDER_METRIX_CLEAR_PEAK_A4;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_GET_EVENT_RECORD_INDEX;
    pub(crate) use crate::game::sound::bankmonitor::callee::BOLTS_FREE_DB;
    pub(crate) use crate::game::world::wscriptcache::callee::SCRIPT_ENGINE_GET_INSTANCE as SCRIPT_ENGINE_GET_INSTANCE_3;
    pub(crate) use crate::game::world::wscriptcache::callee::SCRIPT_SOUND_BANK_MAN_FIND_BANK;
    pub(crate) use crate::game::world::wscriptcache::callee::SSX_AUDIO_SYSTEM_PAUSE;
    pub(crate) use crate::game::world::wscriptcache::callee::BANK_MONITOR_DELETE_INSTANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::BELIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::BELIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::world::wscriptcache::callee::BE_NEW_RACE_INT_GET_TRACK_INFO;
    pub(crate) use crate::game::world::wscriptcache::callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP5;
    pub(crate) use crate::game::world::wscriptcache::callee::CROWD_AUDIO_FREE_SAMPLE_LIST;
    pub(crate) use crate::game::world::wscriptcache::callee::CUT_SCENE_TRANSITION_MAN_BASE_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::CUT_SCENE_TRANSITION_MAN_INIT;
    pub(crate) use crate::game::world::wscriptcache::callee::WSCRIPT_MAN_COLLECT_SET_GET_COUNT;
    pub(crate) use crate::game::world::wscriptcache::callee::WSCRIPT_MAN_COLLECT_SET_ADD_ITEMS;
    pub(crate) use crate::game::world::wscriptcache::callee::MDF_FILE_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::SPRINTF_2;
    pub(crate) use crate::game::world::wscriptcache::callee::INPUT_PREPROC_WRITE_STRING;
    pub(crate) use crate::game::world::wscriptcache::callee::CINSTANCE_NODE_CTOR_V;
    pub(crate) use crate::game::world::wscriptcache::callee::FONT_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_ANIM_APPLY;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_ANIM_ADVANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::SECTION_ALLOCATOR_FREE_ALL;
    pub(crate) use crate::game::world::wscriptcache::callee::C_WORLD_CACHE_STREAMER_DTOR_STUB;
    pub(crate) use crate::game::world::wscriptcache::callee::RCMP_SYSTEM_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::PS2_SONY_CODEC_INTERNAL_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::MPEG2_GET_BITS1;
    pub(crate) use crate::game::world::wscriptcache::callee::MPEG2_FLUSH_BUFFER_3 as MPEG2_FLUSH_BUFFER;
    pub(crate) use crate::game::world::wscriptcache::callee::RIDER_SET_SUB_STATE;
    pub(crate) use crate::game::world::wscriptcache::callee::RIDER_SET_MAIN_STATE;
    pub(crate) use crate::game::world::wscriptcache::callee::BE_SCORE_INT_GET_HIGH_SCORE_TABLE;
    pub(crate) use crate::game::world::wscriptcache::callee::CHASE_CAMERA_ALGORITHM_CTOR_2 as CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR;
    pub(crate) use crate::game::world::wscriptcache::callee::CAMERA_TRIGGER_STACK_CLEAR;
    pub(crate) use crate::game::world::wscriptcache::callee::CAMERA_RESET_CHASE_CONTROLLER_SWITCHES;
    pub(crate) use crate::game::world::wscriptcache::callee::BOLT_FILE_CACHE_SHUTDOWN;
    pub(crate) use crate::game::world::wscriptcache::callee::DELETE;
    pub(crate) use crate::game::world::wscriptcache::callee::SND_BANK_MONITOR_MEM_SIZE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_4 as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_4 as NET_FE_STATE_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHAT_ROOM_REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_ACCOUNT_MEM_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OV_MESSAGE_CENTER_REFRESH_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::OV_MESSAGE_CENTER_UPDATE_COUNT_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_UPDATE_STATES;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_UPDATE_MODE1;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_DISCONNECT_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_CACHE_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_WRITE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_PAUSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_ALLOC_PROCESS;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_INSTANCE_2 as SCRIPT_ENGINE_GET_INSTANCE_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_ACTIVATE_BINDING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_IS_FULL;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_PAUSE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SAY_20D3;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SAY_20D4;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_IS_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_IS_DISABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::OV_SCREEN_PROFILE_SAVE_PDA_REFRESH_CARD_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::OV_SCREEN_PROFILE_SAVE_PDA_RESET_SLOT_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_STATE_GOTO_NEXT_3 as GAME_STATE_GOTO_NEXT_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VID_ENGINE_INSTALL_MEM_HOOKS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VID_ENGINE_PLAYER_INIT_OPEN_PARAMS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_CLEAR_LOGIN;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_CLEAR_MESSAGES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_DISCONNECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_SAVE_EMIT as REPLAY_SAVE_EMIT_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_LIST_STOP_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SSX_AUDIO_SYSTEM_DJ_SAY_20E5;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SPEECH_SYSTEM_CANCEL_PENDING_INSTANCE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MUSIC_PLAYER_GET_SELECTED_SONG_SED_VALUE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BANK_MONITOR_RESET_INSTANCE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_TRANSITION_MAN_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WSCRIPT_MAN_DEACTIVATE_PROCESS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WSCRIPT_MAN_UNLOAD_SECTION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_BX_STRING_COPY_BEFORE_WRITE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SYNC_PROCESS_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ANIM_OBJECT_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MOD_OBJ_CREATE_MODIFIER_BLOCK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RAIL_MODIFIER_LIST_PUSH_FRONT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INTC_TABLE_ADD_HANDLER;
    pub(crate) use crate::game::wscript::wscriptman::callee::SET_FADE_GAIN;
    pub(crate) use crate::game::wscript::wscriptman::callee::SND_BANK_UNLOAD;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_AUDIO_INSTANCE_STOP_SOUND as CROWD_AUDIO_INSTANCE_STOP_SOUND_2;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_INSTANCE_POLL_FREE;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_INSTANCE_SET_PITCH;
    pub(crate) use crate::game::wscript::wscriptman::callee::CPS2_MEMCARD_SET_STATE;
    pub(crate) use crate::game::wscript::wscriptman::callee::AVALANCHE_BAKE_NODE_HISTORY;
    pub(crate) use crate::game::wscript::wscriptman::callee::WSCRIPT_MAN_CALL_FUNC;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_MAN3D_GROUP_FREE_MESHES;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_MAN3D_GROUP_CREATE_MESHES_2;
    pub(crate) use crate::game::wscript::wscriptman::callee::FLEX_RAIL_UPDATE_SPACE_NODE;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_FLAG_MANAGER_GROUP_LOAD;
    pub(crate) use crate::game::wscript::wscriptman::callee::FLOATING_V49;
    pub(crate) use crate::game::wscript::wscriptman::callee::MOD_OBJ_DTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::MODIFIER_LIST_UNLINK;
    pub(crate) use crate::game::wscript::wscriptman::callee::RAIL_MODIFIER_LIST_UNLINK;
    pub(crate) use crate::game::wscript::wscriptman::callee::DISABLE_INTC;
    pub(crate) use crate::game::wscript::wscriptman::callee::REMOVE_INTC_HANDLER;
    pub(crate) use crate::game::wscript::wscriptman::callee::TEX_MAN_LOAD_TEXTURE;
    pub(crate) use crate::game::wscript::wscriptman::callee::LIGHT_MAN_EMPTY_HOOK_A;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_GET_STATE_DATA;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::wscript::wscriptman::callee::CPS2_CHUNK_RESOLVE_CTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::AV_PLAYER_INIT;
    pub(crate) use crate::game::wscript::wscriptman::callee::FOCUS_MAN_ADD_FOCUS_BOX;
    pub(crate) use crate::game::wscript::wscriptman::callee::CAM_CONTROLLER_CLEAR_ALGORITHMS;
    pub(crate) use crate::game::wscript::wscriptman::callee::ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::wscript::wscriptman::callee::APP_MODULE_UPDATE_FADE;
    pub(crate) use crate::game::wscript::wscriptman::callee::DEBUG_MENU_MODULE_UPDATE_CURSOR_TRAIL;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_SETUP_MAN_SEND_QUIT;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_EMPTY_CHILD_HANDLER;
    pub(crate) use crate::game::wscript::wscriptman::callee::NET_FE_STATE_ON_CHILD_EVENT as NET_FE_STATE_ON_CHILD_EVENT_3;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_SET_USER_DATA;
    pub(crate) use crate::game::wscript::wscriptman::callee::EA_FILE_MOUNT_BIG;
    pub(crate) use crate::game::wscript::wscriptman::callee::NET_FE_STATE_CTOR as NET_FE_STATE_CTOR_3;
    pub(crate) use crate::game::wscript::wscriptman::callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::dirtysock::tags2::callee::ON_QUIT_POPUP_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::EMPTY_CHILD_HANDLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_ON_CHILD_EVENT_3 as NET_FE_STATE_ON_CHILD_EVENT_4;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE_2 as NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_CTOR_3 as NET_FE_STATE_CTOR_4;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_DTOR_2 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::STATS_SELECT_CATEGORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_UPDATE_CURSOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_CARD_SET_USE_SAVE_TITLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::TROPHY_ROOM_RELEASE_IMAGES;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_GET_INBOX;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_PUSH_FREE_NODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_END_OPERATION_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_SET_PRESENCE_ADDR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_DESTROY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_OBJ_CAM_SET_OWNER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_OBJ_TABLE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLINK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_FREE_MEMORY_2 as SND_BANK_FREE_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RECORD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_ADD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::STREAM_BUFFER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_INSTANCE_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::PATH_PLAT_REFRESH_STATUS;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_CARD_ASCII_CHAR_TO_SJIS;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_GET_SAVE_NAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_READ_SAVE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_PAGE_SELECT_NEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_ITEM_MEASURE_ROW;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_FREE_HISTORIES;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_BAKE_GROUP;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_EXPORT_HISTORY_FILES;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_LOAD_HISTORY_FILE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SNOW_BOX_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT0;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT1;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT2;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT3;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT4;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT5;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT6;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_MODULE_GET_PLAYER_PAD_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::UIMENU_SET_SELECTION;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_TEXT_GET_FONT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_REWARD_INT_IS_UNLOCKED;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_3 as UI_STATE_CTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR as UI_STATE_DTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_LIST_DIR_ALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_BX_STRING_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE as BUILTIN_DELETE_4;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_LIST_GET_AT_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::SLOGIN_MC_GET_PORT_SLOT_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCE_MC_FORMAT;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY as STRCPY_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPRINTF_2 as SPRINTF_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::RAND_NEXT_MAIN;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SETUP_MAN_SEND_QUIT as C_SETUP_MAN_SEND_QUIT_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF as UI_OBJECT_SHOW_CLAMP_ALPHA_HALF_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_LUNO_TABLE_DTOR;
    pub(crate) use crate::game::render::particle::callee::INI_PARSER_SKIP_TO_EOL;
    pub(crate) use crate::game::render::particle::callee::BANK_MONITOR_INSTANCE_PAUSE;
    pub(crate) use crate::game::render::particle::callee::SONG_INSTANCE_GET_STREAM_HANDLE;
    pub(crate) use crate::game::render::particle::callee::SONG_INSTANCE_GET_MS_TO_NEXT_BEAT;
    pub(crate) use crate::game::render::particle::callee::CPS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_CTOR as MENU_SYS_ITEM_CTOR_2;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_DRAW_ROW_2;
    pub(crate) use crate::game::render::particle::callee::CUT_SCENE_TRANSITION_SAVE_VIEWPORT;
    pub(crate) use crate::game::render::particle::callee::EA_MEM_ALLOC_NAMED_2;
    pub(crate) use crate::game::render::particle::callee::AVA_SPLINE_MODIFIER_CTOR;
    pub(crate) use crate::game::render::particle::callee::POSITION_MODIFIER_CTOR;
    pub(crate) use crate::game::render::particle::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER;
    pub(crate) use crate::game::render::particle::callee::ANIM_OBJECT_BASE_DTOR;
    pub(crate) use crate::game::render::particle::callee::TEX_MAN_LOAD_TEXTURE as TEX_MAN_LOAD_TEXTURE_2;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_RESET;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_PUSH_BACK_2 as UI_LIST_PUSH_BACK;
    pub(crate) use crate::game::render::particle::callee::CPS2_CHUNK_RESOLVE_DTOR_STUB;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_ON_MISSION_COMPLETED;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_AWARD_POINTS;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_SETTLE_SLEW;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_FINISH_PRESS;
    pub(crate) use crate::game::render::particle::callee::RIDER_ADD_BOOST;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_FINISH_BOARD_PRESS;
    pub(crate) use crate::game::render::particle::callee::ALGORITHM_LIST_DTOR;
    pub(crate) use crate::game::render::particle::callee::CHASE_CAMERA_ALGORITHM_CTOR as CHASE_CAMERA_ALGORITHM_CTOR_2;
    pub(crate) use crate::game::render::particle::callee::CFE_STATE_CHAR_SELECT_UPDATE_ATTRIBUTE_BARS;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_CSTR;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_COPY;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_CONCAT_IN_PLACE;
    pub(crate) use crate::game::render::particle::callee::CROWD_MAN3D_GROUP_REMOVE_SECTION;
    pub(crate) use crate::game::render::particle::callee::WORLD_OBJ_SAVE as WORLD_OBJ_SAVE_2;
    pub(crate) use crate::game::render::particle::callee::FLEX_BRIDGE_SPRINGS_SAVE;
    pub(crate) use crate::game::render::particle::callee::BELIBRARY_GET_INSTANCE as BELIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::render::particle::callee::BELIBRARY_GET_INTERFACE as BELIBRARY_GET_INTERFACE_2;
    pub(crate) use crate::game::render::particle::callee::BEREWARD_ON_MISSION_COMPLETED;
    pub(crate) use crate::game::render::particle::callee::BEREWARD_ON_COLLECTIBLE_COLLECTED;
    pub(crate) use crate::game::render::particle::callee::BENEW_PLAYER_INT_GET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::render::particle::callee::BENEW_RACE_INT_IS_STATION_TRACK_BY_INDEX;
    pub(crate) use crate::game::render::particle::callee::SND_LOCK;
    pub(crate) use crate::game::render::particle::callee::SND_UNLOCK;
    pub(crate) use crate::game::render::particle::callee::SND_SPU_MEM_SET_BASE;
    pub(crate) use crate::game::render::particle::callee::LOBBY_OCCUPANT_POP;
    pub(crate) use crate::game::render::particle::callee::LOBBY_OCCUPANT_SELECT;
    pub(crate) use crate::game::render::particle::callee::SNDSTRM_SET_PITCH;
    pub(crate) use crate::game::render::particle::callee::PATH_SET_PAUSED;
    pub(crate) use crate::game::render::particle::callee::PATH_POST_EVENT;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_SEEK;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_READ;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_WRITE;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_MKDIR;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_DELETE;
    pub(crate) use crate::game::render::particle::callee::ISIGNAL_SEMA;
    pub(crate) use crate::game::render::particle::callee::STRLEN;
    pub(crate) use crate::game::render::particle::callee::WORLD_CREATE_DEAD_NODE;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_GET_INSTANCE as SCRIPT_ENGINE_GET_INSTANCE_5;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_BINDING_HAS_PROCESS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::MENU_SYS_ITEM_CTOR as MENU_SYS_ITEM_CTOR_3;
    pub(crate) use crate::game::intersect::worldsphtree::callee::MENU_SYS_ITEM_DRAW_ROW as MENU_SYS_ITEM_DRAW_ROW_3;
    pub(crate) use crate::game::intersect::worldsphtree::callee::MEMCARD_SJIS_CHAR_TO_ASCII;
    pub(crate) use crate::game::intersect::worldsphtree::callee::PS2_MEMCARD_SET_STATE;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WSCRIPT_PROCESS_LIST_CONTAINS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WSCRIPT_PROCESS_LIST_REMOVE as WSCRIPT_PROCESS_LIST_REMOVE_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT as WSCRIPT_PROCESS_LIST_PUSH_FRONT_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::POSITION_MODIFIER_CTOR as POSITION_MODIFIER_CTOR_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER as MOD_OBJ_SET_TRANSFORM_MODIFIER_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SSX_AUDIO_SYSTEM_RESTART_PEAK_MUSIC;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_STOP_PROCESS_SEQ;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_KILL_PROCESS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_GET_PROCESS as SCRIPT_ENGINE_GET_PROCESS_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WORLD_OBJ_CTOR_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SONG_INSTANCE_GET_BEAT_MS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SONG_INSTANCE_GET_POSITION_MS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUILD_BUDDY_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SET_SELECTED_USER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GET_SELECTED_USER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SHOW_BUDDY_ACTION_POPUP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ON_CHILD_EVENT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GOTO_NEXT_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::HANDLE_NET_EVENT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ONLINE_CHARACTER_SELECT_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CHANNEL_GET_TRACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LIST_GET_ENTRY_TYPE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FIND_FREE_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LOAD_BANK_ASYNC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REQUEST_IMAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SEND_PACKET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BANK_MONITOR_RESET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_SAY_20B9;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::STOP_BOARD_LOOPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::START_RIDER_BOARD_LOOPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GET_RACE_MAN;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LEVEL_TO_VOLUME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::AUDIO_SYSTEM_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_END_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_SET_CONTROLLER_INDEX;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_UNLOAD as SND_BANK_UNLOAD_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_STATE_CTOR_2_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_CTOR_2 as UI_LIST_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_MENU_SET_SELECTION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_CARD_BASE_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_CARD_ON_CHILD_EVENT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_HAS_RENDER_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_RESET as UI_LIST_RESET_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_ALLOC_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCARD_INSTALL_CALLBACKS;
    pub(crate) use crate::game::fe::feasyncfile::callee::PLAYBACK_RESTORE_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CHUNK_RESOLVE_HOOK;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_RENDER_FADE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_RAIL_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPLINE_MODIFIER_SET_SPEED;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUTOSAVE_UPDATE_ICON;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_ANIM_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_RIDER_PREVIEWS;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2 as WORLD_OBJ_CTOR_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_DTOR_2 as WORLD_OBJ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_RUN_KILL_SCRIPT_2 as WORLD_OBJ_RUN_KILL_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MOD_OBJ_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_LOCALE_LOAD_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INSTANCE_2 as BE_LIBRARY_GET_INSTANCE_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INTERFACE_2 as BE_LIBRARY_GET_INTERFACE_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_9 as NET_FE_STATE_CTOR_5;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_GET_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_EXPECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_READ_QUOTED_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_TRICK_METER;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MENU_HIDE_ROOM_TABS;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_CHAR_STATS;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_REMOVE_MESSAGE;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_COUNT_GROUP_MESSAGES;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_GET_MESSAGE_COUNT;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_REFRESH_LIST;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_COUNT_TEXT;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_SCROLL_ARROWS;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_CLEAR_SLOTS;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_INSTALL_VIF_VU_HANDLERS;
    pub(crate) use crate::game::main::gamestate::callee::GAME_STATE_LOAD_STATE;
    pub(crate) use crate::game::main::gamestate::callee::VD_TEXTURE_SINK_CTOR;
    pub(crate) use crate::game::main::gamestate::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::main::gamestate::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INSTANCE_3 as BE_LIBRARY_GET_INSTANCE_5;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INTERFACE_3 as BE_LIBRARY_GET_INTERFACE_5;
    pub(crate) use crate::game::main::gamestate::callee::BE_ECON_INT_GET_TOTAL_EARNED;
    pub(crate) use crate::game::main::gamestate::callee::BOLTS_FIND_BOLT_INDEX;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_CMD_QUEUE_AT_2;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SECTION_ALLOCATOR_CTOR;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_CTOR_3;
    pub(crate) use crate::game::main::gamestate::callee::MEMCARD_STATE_BASE_INSTALL_CALLBACKS;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_SHUTDOWN_2 as UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_STOP_3;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_START;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_2;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_RESET_RACE_SOUNDS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_SCRIPT_OBJ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_OBJ_CAM_IS_BOUND_TO_PROCESS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_IS_OBJ_BOUND_TO_PROCESS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_OBJ_CAM_HAS_BINDING;
    pub(crate) use crate::game::main::gamestate::callee::SPEECH_INSTANCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::main::gamestate::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_LANGUAGE_MASK;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_VALIDATE_SELECTION;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_NEXT as MENU_SYS_PAGE_SELECT_NEXT_2;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_PREV;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_HANDLE_INPUT;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_IS_DISABLED as MENU_SYS_ITEM_IS_DISABLED_2;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_GET_INPUT_CODE;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_PAUSE_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_RESUME_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::GAME_ANIM_MAP_UNREGISTER_BANK;
    pub(crate) use crate::game::main::gamestate::callee::ANIM_DATA_BASE_FREE_CHAN_CACHES;
    pub(crate) use crate::game::main::gamestate::callee::ADF_DATA_SOURCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::C_KEYBOARD_POPUP_SET_KEY_ENABLED;
    pub(crate) use crate::game::main::gamestate::callee::RIDER_CLEAR_NEARBY_LIGHTS;
    pub(crate) use crate::game::main::gamestate::callee::SNDSTRM_CLOSE;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_FREE_4 as EA_MEM_FREE_2;
    pub(crate) use crate::game::main::gamestate::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_5;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_OPEN_KEYBOARD_TEXT_ID;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_ON_CHILD_EVENT_3 as NET_FE_STATE_ON_CHILD_EVENT_5;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN_PTR;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_STOP_HANDLE as BANK_MONITOR_STOP_HANDLE_2;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_FIRST_2 as UI_LIST_FIRST;
    pub(crate) use crate::game::main::gamestate::callee::C_GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_BRAGGING_RIGHTS_INIT_PLAYER_ROWS;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_ALLOC_NAMED_3;
    pub(crate) use crate::game::main::gamestate::callee::MEM_MOVE;
    pub(crate) use crate::game::main::gamestate::callee::STR_COPY;
    pub(crate) use crate::game::main::gamestate::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::main::gamestate::callee::STR_N_COPY;
    pub(crate) use crate::game::main::gamestate::callee::SPHERE_TREE_COLLIDE_SPHERE_TREE;
    pub(crate) use crate::game::main::gamestate::callee::SPHERE_TREE_COLLIDE_AABB;
    pub(crate) use crate::game::main::gamestate::callee::SPCH_SHUTDOWN;
    pub(crate) use crate::game::main::gamestate::callee::SPCH_MAKE_ID;
    pub(crate) use crate::game::main::gamestate::callee::MEM_FREE;
    pub(crate) use crate::game::main::gamestate::callee::STR_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_2 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::BX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_INIT_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_CLEAR_EMITTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_BLOCK_POOL_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_MEMMAN_CREATE_SECTION_ALLOCATOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SECTION_TABLE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_4 as UI_LIST_FIRST_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_SPRINGS_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_BUILD_MESH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_UPDATE_VERTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSTANCE_GET_NODE_MATRIX;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_FINISH_CLEAR_FLAG40;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_ANIM_MAP_SET_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::ACTIVE_CAMERA_TRIGGER_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_STACK_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_ERROR_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_CREATED_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_9 as NET_FE_STATE_ON_CHILD_EVENT_6;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_SHOW_ROOM_TAB;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_UPDATE_PLAYERS_IN_ROOM;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_UPDATE_3 as NET_FE_STATE_UPDATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_IS_FLAG_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_EMPTY_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_REQUEST;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_NEXT_SECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_2 as SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CAM_BUILD_MATRIX_FROM_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_CTOR as SCRIPT_BINDING_CTOR_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CTOR as SCRIPT_OBJ_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_MEM_CARD_CLEAR_DIR_CACHE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_RESET_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_INIT_FROM_RIDER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SET_LOCAL_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SEND_CHARACTER_MSG;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES as CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_CAN_SKIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_GET_PLAY_STATUS_2 as SCRIPT_LIST_GET_PLAY_STATUS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_4;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_SCRIPT_SET_OWNER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PATH_PLAT_FIND_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_DRAW_ROW_4;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_CTOR as MENU_SYS_ITEM_CTOR_4;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MATH_EVAL_CUBIC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_DTOR_BODY as UI_LIST_NODE_DTOR_BODY_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_CLOSE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_ADD_VERTEX;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_SCROLL_WINDOW;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_MASK_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_CURSOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_PACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_UNPACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_REWARD_INT_IS_PEAK_GOAL_COMPLETE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VEC3_NEG_DOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_3 as SCRIPT_LIST_IS_PLAYING_SCRIPT_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO_4 as SCRIPT_DATA_GET_INFO_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_INSTANCE_GET_REMAINING_STEPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CONCAT_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CTOR_COPY as CBX_STRING_CTOR_COPY_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_DTOR as CBX_STRING_DTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ATTACHED_DYNAMIC_EMITTER_CREATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_MAN2D_CLEAR_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_STREAM_REWIND_READ;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_ALGORITHM_DEFAULT_FOV;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_CHARACTER as BE_GET_RIDER_CHARACTER_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_PROCESS_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_FLAG_MANAGER_GROUP_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE as WORLD_OBJ_REPLACE_WITH_DEAD_NODE_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_TABLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_FIRST_5 as UI_LIST_FIRST_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_ASCII_TO_WIDE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_TEXT_SET_WIDE_STRING_INTERNAL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BELIBRARY_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_MAN_FREE_ENTRY;
    pub(crate) use crate::game::ai::rider::callee::NET_FE_STATE_ON_CHILD_EVENT as NET_FE_STATE_ON_CHILD_EVENT_7;
    pub(crate) use crate::game::ai::rider::callee::NET_ACCOUNT_MEM_CTOR;
    pub(crate) use crate::game::ai::rider::callee::SHOW_CHAR_STATS;
    pub(crate) use crate::game::ai::rider::callee::START_CARD_SCAN;
    pub(crate) use crate::game::ai::rider::callee::SHOW_CHECKING_CARD_POPUP;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_ACCOUNT_NAME;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_PASSWORD;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_CONFIRM_PASSWORD;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_EMAIL;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_SHARE_INFO;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_INFORM;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_BACKUP_ACCOUNT;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_ALL_FIELDS_FILLED;
    pub(crate) use crate::game::ai::rider::callee::LOGIN_ON_ACCOUNT_MEM_POPUP;
    pub(crate) use crate::game::ai::rider::callee::LOGIN_ON_DISCONNECT_POPUP;
    pub(crate) use crate::game::ai::rider::callee::MESSAGE_CENTER_GET_LIST_INDEX;
    pub(crate) use crate::game::ai::rider::callee::MESSAGE_CENTER_OPEN_VIEW_POPUP;
    pub(crate) use crate::game::ai::rider::callee::LOBBY_MAN_SEND_REQUEST as LOBBY_MAN_SEND_REQUEST_2;
    pub(crate) use crate::game::ai::rider::callee::LOBBY_MAN_GET_LANGUAGE;
    pub(crate) use crate::game::ai::rider::callee::BUDDY_MAN_GET_ROSTER_COUNT as BUDDY_MAN_GET_ROSTER_COUNT_2;
    pub(crate) use crate::game::ai::rider::callee::BUDDY_MAN_GET_ROSTER_ENTRY as BUDDY_MAN_GET_ROSTER_ENTRY_2;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_GET_SURFACE_SOUND_TYPE;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_BASE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_SURFACE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_EDGE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_PLAY_FE_SOUND;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_WIPEOUT_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_WIPEOUT_LOOP2;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_TRICK_METER_SOUND;
    pub(crate) use crate::game::ai::rider::callee::SCRIPT_OBJ_DEACTIVATE;
    pub(crate) use crate::game::ai::rider::callee::REPLAY_FREE_SNAPSHOT;
    pub(crate) use crate::game::ai::rider::callee::CODEC_POP_FREE_FRAME;
    pub(crate) use crate::game::ai::rider::callee::MPEG2_GET_BITS;
    pub(crate) use crate::game::ai::rider::callee::MPEG2_MARKER_BIT;
    pub(crate) use crate::game::ai::rider::callee::BUY_POPUP_INIT_BUY_BOLT;
    pub(crate) use crate::game::ai::rider::callee::RIDER_GET_START_DIST_TO_FINISH;
    pub(crate) use crate::game::ai::rider::callee::BENEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::ai::rider::callee::SCRIPT_ENGINE_GET_PROCESS as SCRIPT_ENGINE_GET_PROCESS_3;
    pub(crate) use crate::game::ai::rider::callee::IN_SEQ_PLAY;
    pub(crate) use crate::game::ai::rider::callee::CROWD_MAN_GROUP_FREE;
    pub(crate) use crate::game::ai::rider::callee::RAIL_MAN_ENABLE_RAIL;
    pub(crate) use crate::game::ai::rider::callee::SYNC_PROCESS_DTOR;
    pub(crate) use crate::game::ai::rider::callee::TEX_MAN_EVICT_TEXTURE;
    pub(crate) use crate::game::ai::rider::callee::VRAM_HEAP_FREE_TEXTURE_BLOCKS;
    pub(crate) use crate::game::ai::rider::callee::GAME_UI_OPEN_SCREEN;
    pub(crate) use crate::game::ai::rider::callee::WORLD_CACHE_RESET;
    pub(crate) use crate::game::ai::rider::callee::WORLD_CACHE_LOAD;
    pub(crate) use crate::game::ai::rider::callee::STREAMER_GET_SECTION_COUNT;
    pub(crate) use crate::game::ai::rider::callee::WSCRIPT_CACHE_ALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_ABORT_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_EMAIL_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_LOGIN_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_12 as NET_FE_STATE_ON_CHILD_EVENT_8;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHALLENGE_POPUP_SET_SETTINGS_LOCKED;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_11 as NET_FE_STATE_CTOR_6;
    pub(crate) use crate::game::fe::feasyncfile::callee::VOIP_MAN_DEACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::VOIP_MAN_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_RESET_RACE_RESULT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_IS_TRACK_GROUP0_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STORE_RACE_RESULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_SKIP_WHITESPACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_GET_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_CHARACTER_2 as BE_GET_RIDER_CHARACTER_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COUNT_COLLECTED_ON_PEAK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COUNT_COMPLETED_ON_PEAK;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_GET_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::SETUP_BUDDY_ACTION_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_4 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_3 as NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID_2 as POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_CARD_STATE_BASE_SHUTDOWN;
    pub(crate) use crate::game::fe::feasyncfile::callee::DEAD_NODE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CROWD_MAN3D_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::CROWD_MAN3D_GROUP_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_TREE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FEMODULE_RIDER_PREVIEW_UNLOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_UPDATE_BOARD_LEAN;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_BUILD_WORLD_MATRIX;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_LOAD_STATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_ON_CARD_SCANNED;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_LOAD_UPDATE_PLAYER_RANKING;
    pub(crate) use crate::game::fe::feasyncfile::callee::LIGHT_MAN_EMPTY_HOOK_D;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_CHAR_PROFILE_EVENT_RECORD;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_SLOT_3 as BE_NEW_PLAYER_INT_GET_SLOT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_SET_ACTIVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_GET_BITS_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_MARKER_BIT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_FLUSH;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_FROM_ASCII_2 as USTR_FROM_ASCII;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_SPRINTF;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_SET_WIDGET_ALIGN;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_SET_WRAPPED_TEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::RIDER_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::DEBUG_MENU_DISPATCH_ITEM_OP;
    pub(crate) use crate::game::bx::bxstring2::callee::WSCRIPT_MISSION_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED_4;
    pub(crate) use crate::game::bx::bxstring2::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT_2 as WSCRIPT_PROCESS_LIST_PUSH_FRONT_3;
    pub(crate) use crate::game::bx::bxstring2::callee::MULTIPARTICLE_MAN_CLEAR;
    pub(crate) use crate::game::bx::bxstring2::callee::MULTIPARTICLE_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::MUSIC_PLAYER_SET_EVENT_STATE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_UPDATE as UI_STATE_UPDATE_3;
    pub(crate) use crate::game::bx::bxstring2::callee::OPTIONS_SET_VIBRATION;
    pub(crate) use crate::game::bx::bxstring2::callee::STR_HASH as STR_HASH_2;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_SCREEN_FIND_OBJECT as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_MENU_FIND_FIRST_SELECTABLE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_MENU_SET_SELECTION as UI_MENU_SET_SELECTION_2;
    pub(crate) use crate::game::bx::bxstring2::callee::STRLEN as STRLEN_2;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_DEQUEUE_SECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_POP_FREE_NODE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_ADD_ITEM_STR_ID;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_FIND_ITEM_BY_VALUE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_SET_SELECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_FE_STATE_GET_POPUP as NET_FE_STATE_GET_POPUP_2;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_FE_STATE_SHOW_POPUP as NET_FE_STATE_SHOW_POPUP_2;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::bx::bxstring2::callee::CBXSTRING_CTOR_COPY;
    pub(crate) use crate::game::bx::bxstring2::callee::CBXSTRING_DTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::bx::bxstring2::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT as SPEECH_SYSTEM_SET_EVENT_CONTEXT_2;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_GET_LANGUAGE_MASK;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_GET_SPEECH_CONTEXT_MASK;
    pub(crate) use crate::game::bx::bxstring2::callee::INI_PARSER_SKIP_WHITESPACE;
    pub(crate) use crate::game::bx::bxstring2::callee::CHUNK_RESOLVE_RELOCATE_DMA;
    pub(crate) use crate::game::bx::bxstring2::callee::FLAG_MANAGER_GROUP_BUILD_MESH;
    pub(crate) use crate::game::bx::bxstring2::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_4;
    pub(crate) use crate::game::bx::bxstring2::callee::BE_GET_CHAR_PROFILE_EVENT_RECORD as BE_GET_CHAR_PROFILE_EVENT_RECORD_2;
    pub(crate) use crate::game::bx::bxstring2::callee::OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::READ_PRIVATE_MESSAGE_SET_TEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_MANAGER;
    pub(crate) use crate::game::bx::bxstring2::callee::OV_SCREEN_PDA_TEMPLATE_CLOSE;
    pub(crate) use crate::game::bx::bxstring2::callee::SUPERPIPE_COMPUTE_DESIRED_EYE;
    pub(crate) use crate::game::bx::bxstring2::callee::SUPERPIPE_COMPUTE_LOOK_AT;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAMERA_RESET_FOR_RIDER;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAMERA_INIT_FROM_RIDER;
    pub(crate) use crate::game::bx::bxstring2::callee::MEMSET as MEMSET_3;
    pub(crate) use crate::game::bx::bxstring2::callee::QSORT;
    pub(crate) use crate::game::bx::bxstring2::callee::CROWD_MAN3D_GROUP_BUILD_MATRICES;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAM_CONTROLLER_RESTORE_DEFAULT;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_LIST_START as SCRIPT_ENGINE_LIST_START_2;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_SET_SKIP_DELAY;
    pub(crate) use crate::game::bx::bxstring2::callee::CUTSCENE_TRANSITION_MAN_FORCE_ON;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_MGR_FIND_STATE as UI_STATE_MGR_FIND_STATE_2;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_SHUTDOWN as UI_STATE_SHUTDOWN_2;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_MGR_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::bx::bxstring2::callee::GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_LEN;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_CPY;
    pub(crate) use crate::game::bx::bxstring2::callee::UITEXT_SET_WIDE_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::MALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FREE2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::VEC3_MAX_COMPONENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_12 as NET_FE_STATE_CTOR_7;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_DTOR_4 as NET_FE_STATE_DTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_3 as NET_FE_STATE_INIT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_SELECTED_USER;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_MANAGER_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST_2 as PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT_2 as PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_FREE_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_PARSE_2 as CAMERA_TRIGGER_MAN_PARSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_MONTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_DAY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_YEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_GENDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_LEGAL;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARD_GALLERY_INIT_PAGES;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARD_GALLERY_REQUEST_SHAPE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OV_STATE_OVERLAY_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPTIONS_HUD_REFRESH_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_SET_FLAG_2 as MESSAGE_CENTER_SET_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_BIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_WRITE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_NEXT_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_EMPTY_STUB;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_CTOR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_ALLOC_BUFFER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_LANGUAGE_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SET_EVENT_CONTEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRACK_TO_CATEGORY;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE_2 as AUDIO_GET_GAME_MODULE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_SOMETHING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_NEW;
    pub(crate) use crate::game::fe::festaterewards::callee::IS_DJ_SPEECH_ON_2;
    pub(crate) use crate::game::fe::festaterewards::callee::IS_ARCADE_SPEECH_ON;
    pub(crate) use crate::game::fe::festaterewards::callee::GET_LANGUAGE_MASK_2;
    pub(crate) use crate::game::fe::festaterewards::callee::SPEECH_STOP_INSTANCE;
    pub(crate) use crate::game::fe::festaterewards::callee::SPEECH_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_POP_FREE_NODE;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_PUSH_FREE_NODE;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_START_SCRIPT;
    pub(crate) use crate::game::fe::festaterewards::callee::MOD_OBJ_RUN_SCRIPT_EVENT;
    pub(crate) use crate::game::fe::festaterewards::callee::REWARD_GALLERY_REGISTER_TEXTURES;
    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_SERVER_TIMEOUT;
    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_OPPONENT_DISCONNECTED;
    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_SHOW_CANCEL;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_LIBRARY_INSTANCE;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_LIBRARY_INTERFACE;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_REWARD_SET_BIT;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_REWARD_GET_FILE;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_ECON_SPEND_MONEY;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_ECON_GET_MONEY;
    pub(crate) use crate::game::fe::festaterewards::callee::TAG_FIELD_COPY_TAG;
    pub(crate) use crate::game::fe::festaterewards::callee::TAG_FIELD_SET_RAW;
    pub(crate) use crate::game::fe::festaterewards::callee::BRAGGING_RIGHTS_UPDATE_ROW_COLORS;
    pub(crate) use crate::game::fe::festaterewards::callee::BENEWPLAYER_IS_SHARING_CHAR_RECORD;
    pub(crate) use crate::game::fe::festaterewards::callee::BECHAR_GET_OVERALL_RATING;
    pub(crate) use crate::game::fe::festaterewards::callee::MESSAGE_IS_FLAG_SET;
    pub(crate) use crate::game::fe::festaterewards::callee::MESSAGE_SET_FLAG;
    pub(crate) use crate::game::fe::festaterewards::callee::MESSAGE_POST;
    pub(crate) use crate::game::fe::festaterewards::callee::TEXFLIP_SET_FRAME;
    pub(crate) use crate::game::fe::festaterewards::callee::BENEWPLAYER_GET_SLOT;
    pub(crate) use crate::game::fe::festaterewards::callee::VFX_PLANTS_GET_SWAY_MATRIX;
    pub(crate) use crate::game::fe::fepopup::callee::C00113f88;
    pub(crate) use crate::game::fe::fepopup::callee::C00115ab0;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fe78;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fe98;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fec8;
    pub(crate) use crate::game::fe::fepopup::callee::C00125108;
    pub(crate) use crate::game::fe::fepopup::callee::C0014a080;
    pub(crate) use crate::game::fe::fepopup::callee::C0014a0e0;
    pub(crate) use crate::game::fe::fepopup::callee::C0014bdb8;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d448;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d988;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d998;
    pub(crate) use crate::game::fe::fepopup::callee::C0014dc80;
    pub(crate) use crate::game::fe::fepopup::callee::C0014dd58;
    pub(crate) use crate::game::fe::fepopup::callee::C00150928;
    pub(crate) use crate::game::fe::fepopup::callee::C00150b48;
    pub(crate) use crate::game::fe::fepopup::callee::C00151c48;
    pub(crate) use crate::game::fe::fepopup::callee::C001535c0;
    pub(crate) use crate::game::fe::fepopup::callee::C001543e0;
    pub(crate) use crate::game::fe::fepopup::callee::C00156988;
    pub(crate) use crate::game::fe::fepopup::callee::C00156a60;
    pub(crate) use crate::game::fe::fepopup::callee::C001577e0;
    pub(crate) use crate::game::fe::fepopup::callee::C00157bf0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158910;
    pub(crate) use crate::game::fe::fepopup::callee::C001589b0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158a50;
    pub(crate) use crate::game::fe::fepopup::callee::C00158af0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158c80;
    pub(crate) use crate::game::fe::fepopup::callee::C00158d58;
    pub(crate) use crate::game::fe::fepopup::callee::C00159170;
    pub(crate) use crate::game::fe::fepopup::callee::C00159cd0;
    pub(crate) use crate::game::fe::fepopup::callee::C0015a2e0;
    pub(crate) use crate::game::fe::fepopup::callee::C0016c6b0;
    pub(crate) use crate::game::fe::fepopup::callee::C0016c718;
    pub(crate) use crate::game::fe::fepopup::callee::C00176890;
    pub(crate) use crate::game::fe::fepopup::callee::C0017f500;
    pub(crate) use crate::game::fe::fepopup::callee::C0019e2b0;
    pub(crate) use crate::game::fe::fepopup::callee::C0019f878;
    pub(crate) use crate::game::fe::fepopup::callee::C001bb9b0;
    pub(crate) use crate::game::fe::fepopup::callee::C001c84a8;
    pub(crate) use crate::game::fe::fepopup::callee::C001c8568;
    pub(crate) use crate::game::fe::fepopup::callee::C001c9b68;
    pub(crate) use crate::game::fe::fepopup::callee::C001cb9b0;
    pub(crate) use crate::game::fe::fepopup::callee::C001cbc48;
    pub(crate) use crate::game::fe::fepopup::callee::C001cbce8;
    pub(crate) use crate::game::fe::fepopup::callee::C001cbfc0;
    pub(crate) use crate::game::fe::fepopup::callee::C001ce560;
    pub(crate) use crate::game::fe::fepopup::callee::C001d32e0;
    pub(crate) use crate::game::fe::fepopup::callee::C001e1a30;
    pub(crate) use crate::game::fe::fepopup::callee::C001e3388;
    pub(crate) use crate::game::fe::fepopup::callee::C001e91a8;
    pub(crate) use crate::game::fe::fepopup::callee::C001e91f8;
    pub(crate) use crate::game::fe::fepopup::callee::C001e9290;
    pub(crate) use crate::game::fe::fepopup::callee::C001f10f8;
    pub(crate) use crate::game::fe::fepopup::callee::C0020e900;
    pub(crate) use crate::game::fe::fepopup::callee::C00224c78;
    pub(crate) use crate::game::fe::fepopup::callee::C00224da0;
    pub(crate) use crate::game::fe::fepopup::callee::C00224df0;
    pub(crate) use crate::game::fe::fepopup::callee::C00225068;
    pub(crate) use crate::game::fe::fepopup::callee::C002259a8;
    pub(crate) use crate::game::fe::fepopup::callee::C00227f80;
    pub(crate) use crate::game::fe::fepopup::callee::C00231f80;
    pub(crate) use crate::game::fe::fepopup::callee::C00238bf8;
    pub(crate) use crate::game::fe::fepopup::callee::C0026d4d8;
    pub(crate) use crate::game::fe::fepopup::callee::C0026db88;
    pub(crate) use crate::game::fe::fepopup::callee::C0026e448;
    pub(crate) use crate::game::fe::fepopup::callee::C0026f4a0;
    pub(crate) use crate::game::fe::fepopup::callee::C002702f8;
    pub(crate) use crate::game::fe::fepopup::callee::C002703f0;
    pub(crate) use crate::game::fe::fepopup::callee::C00275498;
    pub(crate) use crate::game::fe::fepopup::callee::C002758c0;
    pub(crate) use crate::game::fe::fepopup::callee::C00275b98;
    pub(crate) use crate::game::fe::fepopup::callee::C00275cf8;
    pub(crate) use crate::game::fe::fepopup::callee::C00278308;
    pub(crate) use crate::game::fe::fepopup::callee::C00278e50;
    pub(crate) use crate::game::fe::fepopup::callee::C00278f38;
    pub(crate) use crate::game::fe::fepopup::callee::C00278f68;
    pub(crate) use crate::game::fe::fepopup::callee::C00281ed8;
    pub(crate) use crate::game::fe::fepopup::callee::C00283228;
    pub(crate) use crate::game::fe::fepopup::callee::C00283518;
    pub(crate) use crate::game::fe::fepopup::callee::C002837c8;
    pub(crate) use crate::game::fe::fepopup::callee::C0028b180;
    pub(crate) use crate::game::fe::fepopup::callee::C0028bb10;
    pub(crate) use crate::game::fe::fepopup::callee::C0028e8c0;
    pub(crate) use crate::game::fe::fepopup::callee::C002913d8;
    pub(crate) use crate::game::fe::fepopup::callee::C003457c8;
    pub(crate) use crate::game::fe::fepopup::callee::C00358020;
    pub(crate) use crate::game::fe::fepopup::callee::C00362478;
    pub(crate) use crate::game::fe::fepopup::callee::C003625c0;
    pub(crate) use crate::game::fe::fepopup::callee::C00362660;
    pub(crate) use crate::game::fe::fepopup::callee::C003626d8;
    pub(crate) use crate::game::fe::fepopup::callee::C003627a8;
    pub(crate) use crate::game::fe::fepopup::callee::C00364cd0;
    pub(crate) use crate::game::fe::fepopup::callee::C00366ce0;
    pub(crate) use crate::game::fe::fepopup::callee::C00368170;
    pub(crate) use crate::game::fe::fepopup::callee::C003684f0;
    pub(crate) use crate::game::fe::fepopup::callee::C00368970;
    pub(crate) use crate::game::fe::fepopup::callee::C0038f460;
    pub(crate) use crate::game::fe::fepopup::callee::C0038f668;
    pub(crate) use crate::game::fe::fepopup::callee::C0039c870;
    pub(crate) use crate::game::fe::fepopup::callee::C0039ff78;
    pub(crate) use crate::game::fe::fepopup::callee::C003b1300;
    pub(crate) use crate::game::fe::fepopup::callee::C003b2360;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMSET_3 as MEMSET_4;
    pub(crate) use crate::game::dirtysock::tags2::callee::STR_HASH_4 as STR_HASH_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPRINTF_6 as SPRINTF_4;
    pub(crate) use crate::game::dirtysock::tags2::callee::FORMAT_TAGGED;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_RESET_PROFILE_CHARACTERS_2 as BE_RESET_PROFILE_CHARACTERS;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_NEW_PLAYER_INT_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_CHAR_INT_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_BAGT_INT_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_MEM_MAN_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::TAG_FIELD_FIND;
    pub(crate) use crate::game::dirtysock::tags2::callee::TAG_FIELD_GET_NUMBER;
    pub(crate) use crate::game::dirtysock::tags2::callee::CBE_GET_BE;
    pub(crate) use crate::game::dirtysock::tags2::callee::CBE_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_CHAR_ID;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_ALT_CHAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::GM_PICK_OPPONENTS;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_SET_MODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_GET_PRODUCT_CODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_GET_SAVE_TYPE_TAG;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_00152BA8;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_00152948;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_002C2540;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_002C26D0;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_IS_FLAG_SET_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_SET_FLAG_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::STR_INTERN;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_0039E2A0;
    pub(crate) use crate::game::dirtysock::tags2::callee::SNPRINTF;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBYMAN_SEND_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_1;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_4;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FLUSH;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_5;
    pub(crate) use crate::game::dirtysock::tags2::callee::IS_PA_SPEECH_ON;
    pub(crate) use crate::game::dirtysock::tags2::callee::IS_REPLAY_PLAYING;
    pub(crate) use crate::game::dirtysock::tags2::callee::RIDER_123168;
    pub(crate) use crate::game::dirtysock::tags2::callee::IS_RACE_TYPE_4OR5;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_ADD_TIMER_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPEECH_SET_EVENT_CONTEXT as SPEECH_SET_EVENT_CONTEXT_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_LANGUAGE_MASK as GET_LANGUAGE_MASK_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_RIDER_VOICE_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_FINISH_SPEECH_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPEECH_QUEUE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FIND;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_REMOVE;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_DEACTIVATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SETUPMAN_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::TEXT_OBJ;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEASURE_PLAIN;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEASURE_WRAP;
    pub(crate) use crate::game::dirtysock::tags2::callee::ANIM_GET_LAYER_SEQUENCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IS_LOCAL_PLAYER;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_TRANSITION_MAN;
    pub(crate) use crate::game::dirtysock::tags2::callee::TRANSITION_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::EFFECT_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::EFFECT_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::TRANSITION_START;


    /// `Rider_GetMainState(Rider*)`: `*(*(rider + 0x77c) + 0xde0)`.
    pub const RIDER_GET_MAIN_STATE: Callee = Callee {
        addr: 0x0011_fe98,
        f: ee::s00::Rider_GetMainState,
    };


    pub const FIND_INSTANCE: Callee = Callee { addr: 0x002a_d550, f: ee::s08::BankMonitor_FindInstance };

    pub const STOP_SOUND: Callee = Callee { addr: 0x002a_6848, f: ee::s08::CrowdAudio_InstanceStopSound };

    pub const PICK_SAMPLE: Callee = Callee { addr: 0x002a_4ca0, f: ee::s08::CrowdAudio_PickRandomSample };

    pub const PUSH_PLAY_PARAMS: Callee = Callee { addr: 0x002a_8450, f: ee::s08::MIDIBanks_PushPlayParams };

    pub const MIDI_PLAY: Callee = Callee { addr: 0x002a_86b8, f: ee::s08::MIDIBanks_Play };

    pub const SET_PITCH: Callee = Callee { addr: 0x002a_be78, f: ee::s08::BankMonitorInstance_SetPitch };

    pub const PLAY_PUSHED_SOUND: Callee = Callee { addr: 0x0029_06b8, f: ee::s07::Audio_PlayPushedSound };
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod table {
    pub(crate) use crate::game::wscript::wscriptman::table::IS_REPLAY_PLAYING;
    pub(crate) use crate::game::wscript::wscriptman::table::GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::wscript::wscriptman::table::SND_BANK_REMOVE;
    pub(crate) use crate::game::wscript::wscriptman::table::SND_MIDI_RESUME;
    pub(crate) use crate::game::wscript::wscriptman::table::SCE_MC_FLUSH;
    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_DELETE;
    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_VEC_DELETE;
    pub(crate) use crate::game::wscript::wscriptman::table::MEM_FREE;
    pub(crate) use crate::game::wscript::wscriptman::table::WORLD_GET_OBJECT_SCRIPT_FUNC;
    pub(crate) use crate::game::wscript::wscriptman::table::WSCRIPT_PROCESS_LIST_POP_FRONT;
    pub(crate) use crate::game::wscript::wscriptman::table::STR_HASH;
    pub(crate) use crate::game::wscript::wscriptman::table::SERIALIZE_READ_U32;
    pub(crate) use crate::game::wscript::wscriptman::table::SERIALIZE_READ_U32_2;
    pub(crate) use crate::game::wscript::wscriptman::table::BXFILE_EXISTS;
    pub(crate) use crate::game::wscript::wscriptman::table::STREAM_FILE_DESTROY;
    pub(crate) use crate::game::wscript::wscriptman::table::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::table::BXSTRING_ASSIGN;
    pub(crate) use crate::game::fe::feasyncfile::table::BXSTRING_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_MENU_RESET_SCROLL;
    pub(crate) use crate::game::fe::feasyncfile::table::PS2_MEM_CARD_CREATE;
    pub(crate) use crate::game::fe::feasyncfile::table::PATH_STOP_TRACKS;
    pub(crate) use crate::game::fe::feasyncfile::table::EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::table::VOIP_RPC_START;
    pub(crate) use crate::game::fe::feasyncfile::table::VOIP_RPC_SET_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::table::AUDIO_GET_RACE_MAN;
    pub(crate) use crate::game::fe::feasyncfile::table::ASYNC_FILE_GET_RESULT;
    pub(crate) use crate::game::fe::feasyncfile::table::SCRIPT_ENGINE_GET_RESULT_VTABLE;
    pub(crate) use crate::game::fe::feasyncfile::table::RIDER_MODEL_FIND;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_NEW_RACE_SET_GAME_MODE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_NEW_RACE_SET_EVENT_TYPE;
    pub(crate) use crate::game::fe::feasyncfile::table::REPLAY_END_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::table::REPLAY_BEGIN_PLAYBACK;
    pub(crate) use crate::game::fe::feasyncfile::table::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::table::OPERATOR_NEW_TAG;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_TEXT_SET_WIDE_STRING;
    pub(crate) use crate::game::fe::feasyncfile::table::SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::table::EAC_GET_UNPACKED_SIZE;
    pub(crate) use crate::game::fe::feasyncfile::table::EAC_UNPACK;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_SPHERE_TREE_TRIANGLE;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_HULL_TRIANGLE;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_HULL_TRIANGLE2;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_SPHERE_CALC;
    pub(crate) use crate::game::fe::feasyncfile::table::STR_CASECMP;
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod addr {
    pub(crate) use crate::game::bx::bxstring2::addr::CBE_GET_BE;
    pub(crate) use crate::game::bx::bxstring2::addr::CBE_GET_INTERFACE;
    pub(crate) use crate::game::bx::bxstring2::addr::BE_INTERFACE_LOOKUP;
    pub(crate) use crate::game::bx::bxstring2::addr::BANK_MONITOR_TIMER_RESET;
    pub(crate) use crate::game::bx::bxstring2::addr::SPEECH_BANK_FREE_CALLBACK;
    pub(crate) use crate::game::bx::bxstring2::addr::UI_ENGINE_TEARDOWN;
    pub(crate) use crate::game::bx::bxstring2::addr::SPRINTF;
    pub(crate) use crate::game::bx::bxstring2::addr::FMODF;
    pub(crate) use crate::game::bx::bxstring2::addr::MEM_MAN_ALLOC;
    pub(crate) use crate::game::bx::bxstring2::addr::MEM_MAN_FREE;
    pub(crate) use crate::game::bx::bxstring2::addr::MEM_MAN_FREE_NOSIZE;
    pub(crate) use crate::game::bx::bxstring2::addr::MC_MANAGER_GET_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STR_HASH;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_ENGINE_CREATE_SCREEN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_SCREEN_START_THREAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::CLOSE_UNPINNED_POPUPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCASECMP;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCMP;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRLEN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MEMSET;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_BE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_INTERFACE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::CHASE_CAMERA_RESET_FOR_RIDER;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_POP_PAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_PUSH_PAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::FE_NET_SEND;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_DTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::WORLD_GET;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MC_STATUS_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::RIDER_PREVIEW;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::RIDER_PREVIEW_BOLTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_INTERFACE_9;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::HUD_SHOW_NUMBER_DELAYED;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::SPATIAL_INSERT;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::POSITION_MODIFIER_CTOR_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::WORLD_PAINTER_2D;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_OBJECT_BY_HASH;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BANK_MONITOR_ADD;
}

/// `GetHashValue32` (0x317670) and the UI object lookups, which the generator
/// emitted as cross-shard `call_indirect`s.
pub(crate) mod table_callee {
    pub(crate) use crate::game::fe::festaterewards::table_callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::fe::festaterewards::table_callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::fe::festaterewards::table_callee::STRLEN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::festaterewards::table_callee::STRCPY;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_OBJECT_PUSH;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_ADD_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_ENGINE_GET_INTERFACE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_MENU_SET_SELECTED_BY_INDEX;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_FIND;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_GET_STRING4;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BUDDY_PRODUCT_COMPARE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::SSX_AUDIO_GET_RACE_MAN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_REMOVE_SCRIPT_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_SET_SCRIPT_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_SCRIPT_CLEANUP;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_RICH_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::CODEC_DECODE_FRAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET_INTERFACE_FV;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_GET_BASE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BUY_POPUP_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_POSTERS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_TRADING_CARDS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_CONCEPT_ARTS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_VIDEOS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_TOYS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_CHEAT_CHARS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_OPEN_FINISH_SCREEN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::OPERATOR_DELETE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::AI_SET_AI_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_SET_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET_TEXT_FROM_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BENEW_PLAYER_INT_GET_CHAR_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_MODULE_SMALL_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_MODULE_BIG_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::APP_MAN_SET_NEXT_MODULE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UPLOAD_TEXTURE_PIXELS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::MUSIC_SLOT_RELEASE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_STATE_DTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BXSTRING_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BXSTRING_DTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::SECTION_TABLE_CTOR;
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
    0x5788, 0x5740, 0x5744, 0x574c, 0x5750, 0x5754, 0x5758, 0x5760, 0x5768, 0x576c, 0x5770,
    0x5774, 0x577c, 0x5784,
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
            c.lwc1(3, (c.g[V1][0] as u32));
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
        let mut id;
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
        let mut result = 0u64;
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
            0x0029_f2d8 | 0x0029_f2e0 | 0x0029_f2e8 | 0x0029_f2f0 | 0x0029_f2f8
            | 0x0029_f300 | 0x0029_f308 | 0x0029_f310 | 0x0029_f318 | 0x0029_f320 => {
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
            0x002a_1dc8 | 0x002a_1dd0 | 0x002a_1dd8 | 0x002a_1de0 | 0x002a_1de8
            | 0x002a_1df0 | 0x002a_1df8 | 0x002a_1e00 | 0x002a_1e08 | 0x002a_1e10 => {
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
        let mut handle = 0u64;
        c.g[A1][0] = lw_at(c, this, 0x174);
        handle = c.g[A1][0];
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
            c.sw((c.g[SP][0] as u32), 0u64);
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
                        c.sw((c.g[V0][0] as u32), 0u64);
                        c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x1));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
                        c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0u64);
                        c.sw((c.g[V1][0] as u32), 0u64);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                        c.sw((c.g[A0][0] as u32), 0u64);
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
                            c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S7][0] as u32)));
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
                                    c.lq(4, (c.g[A0][0] as u32));
                                    c.g[S5][0] = 0x1;
                                    c.g[A1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0xd4));
                                    c.sq(4, (c.g[SP][0] as u32));
                                    c.g[A1][0] = ((c.g[S5][0] as i64) < (c.g[A1][0] as i64)) as u64;
                                    c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x20));
                                    c.g[FP][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[SP][0] as u32)));
                                    c.sq(4, (c.g[V0][0] as u32).wrapping_add(0x10));
                                    c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x20));
                                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    let t = c.g[A1][0] == 0u64;
                                    c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                                    if t {
                                        break 'b_2a51d4;
                                    } else {
                                        c.cycles += 6;
                                        c.g[V0][0] = 0xc;
                                        c.g[T3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8c));
                                        c.mult(0, 2, 9, 2);
                                        c.g[T4][0] = c.g[S3][0];
                                        c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
                                        c.lwc1(5, (c.g[SP][0] as u32));
                                        'l_2a4fd0: loop {
                                            c.cycles += 4;
                                            c.g[T5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                                            c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[S7][0] as u32)));
                                            c.g[A3][0] = c.g[V1][0];
                                            'l_2a4fe0: loop {
                                                'b_2a5004: {
                                                    c.cycles += 6;
                                                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(0xc));
                                                    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(0xc));
                                                    c.g[S6][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_sub(0xc));
                                                    let t = c.g[A0][0] != 0u64;
                                                    c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x1));
                                                    if t {
                                                        break 'b_2a5004;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = ((c.g[T1][0] as i64) < 15i64) as u64;
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
                                                        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                                        c.lwc1(0, (c.g[A0][0] as u32));
                                                        c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                        c.mtc1(2, 4);
                                                        c.cvt_s_w(4, 4);
                                                        c.mtc1(3, 2);
                                                        c.cvt_s_w(2, 2);
                                                        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
                                                        c.sub_s(0, 0, 5);
                                                        c.lwc1(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                                                        c.div_s(4, 4, 2);
                                                        c.mul_s(0, 0, 4);
                                                        c.add_s(0, 5, 0);
                                                        c.swc1(0, (c.g[SP][0] as u32));
                                                        c.g[V0][0] = c.lw((c.g[A3][0] as u32));
                                                        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                                                        c.sub_s(0, 0, 1);
                                                        c.mul_s(0, 0, 4);
                                                        c.add_s(1, 1, 0);
                                                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
                                                        c.g[V0][0] = c.lw((c.g[A3][0] as u32));
                                                        c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
                                                        c.sub_s(0, 0, 3);
                                                        c.mul_s(0, 0, 4);
                                                        c.add_s(3, 3, 0);
                                                        c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x8));
                                                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                                        c.g[V0][0] = ((c.g[FP][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = sx(((c.g[FP][0] as u32) as i32 >> 2) as u32);
                                                        if t {
                                                            break 'b_2a5088;
                                                        } else {
                                                            c.cycles += 3;
                                                            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                            c.g[FP][0] = c.g[V1][0];
                                                            break 'b_2a5090;
                                                        }
                                                    }
                                                    c.cycles += 2;
                                                    c.g[V0][0] = sx(((c.g[V1][0] as u32) as i32 >> 2) as u32);
                                                    c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                    break 'b_2a5090;
                                                }
                                                'b_2a51c4: {
                                                    c.cycles += 4;
                                                    c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x170));
                                                    c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
                                                    if c.g[V0][0] != 0u64 {
                                                        c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0xd4));
                                                        break 'b_2a51c4;
                                                    } else {
                                                        'b_2a51c0: {
                                                            'b_2a51ac: {
                                                                c.cycles += 57;
                                                                c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x20));
                                                                c.g[A2][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                                                c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[T4][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                c.mtc1(6, 4);
                                                                c.cvt_s_w(4, 4);
                                                                c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                                                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                                                                c.g[V0][0] = c.lw((c.g[A3][0] as u32));
                                                                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[SP][0] as u32)));
                                                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                                                                c.lwc1(2, (c.g[A1][0] as u32).wrapping_add(0x10));
                                                                c.mtc1(4, 3);
                                                                c.cvt_s_w(3, 3);
                                                                c.lwc1(0, (c.g[V0][0] as u32));
                                                                c.g[V1][0] = c.g[A1][0];
                                                                c.sub_s(0, 0, 2);
                                                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                                                                c.div_s(4, 4, 3);
                                                                c.lwc1(1, (c.g[V1][0] as u32));
                                                                c.mul_s(0, 0, 4);
                                                                c.add_s(1, 1, 0);
                                                                c.swc1(1, (c.g[V1][0] as u32));
                                                                c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x20));
                                                                c.g[A0][0] = c.lw((c.g[A3][0] as u32));
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                                                                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[SP][0] as u32)));
                                                                c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x4));
                                                                c.lwc1(2, (c.g[V1][0] as u32).wrapping_add(0x14));
                                                                c.g[V0][0] = c.g[V1][0];
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                                                                c.sub_s(1, 1, 2);
                                                                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                                                                c.mul_s(1, 1, 4);
                                                                c.add_s(0, 0, 1);
                                                                c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
                                                                c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x20));
                                                                c.g[A0][0] = c.lw((c.g[A3][0] as u32));
                                                                c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                                                                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[SP][0] as u32)));
                                                                c.lwc1(1, (c.g[A0][0] as u32).wrapping_add(0x8));
                                                                c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x18));
                                                                c.g[V1][0] = c.g[V0][0];
                                                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                                                                c.sub_s(1, 1, 2);
                                                                c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                                                                c.mul_s(1, 1, 4);
                                                                c.add_s(0, 0, 1);
                                                                c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                                                                c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x20));
                                                                c.g[A1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                                                c.g[A0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                c.g[V1][0] = sx((c.g[T4][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                c.g[V1][0] = c.lw((c.g[V1][0] as u32));
                                                                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                                                                let t = c.g[V0][0] == 0u64;
                                                                c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 2) as u32);
                                                                if t {
                                                                    break 'b_2a51ac;
                                                                } else {
                                                                    c.cycles += 10;
                                                                    c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                    c.sw((c.g[A0][0] as u32), c.g[V0][0]);
                                                                    c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x20));
                                                                    c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                    c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                                                                    break 'b_2a51c0;
                                                                }
                                                            }
                                                            c.cycles += 5;
                                                            c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                            c.g[V1][0] = sx(((c.g[A1][0] as u32) as i32 >> 2) as u32);
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                            c.sw((c.g[A0][0] as u32), c.g[V0][0]);
                                                            break 'b_2a51c0;
                                                        }
                                                        c.cycles += 1;
                                                        c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0xd4));
                                                        break 'b_2a51c4;
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.g[S5][0] = c.g[T5][0];
                                                c.g[V0][0] = ((c.g[S5][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                                let t = c.g[V0][0] != 0u64;
                                                c.lwc1(5, (c.g[SP][0] as u32));
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
                                        c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add((c.g[T2][0] as u32)));
                                        c.g[V0][0] = 0x1;
                                        let t = c.g[A0][0] != c.g[V0][0];
                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x78), c.g[A0][0]);
                                        if t {
                                            break 'b_2a55e8;
                                        } else {
                                            c.cycles += 3;
                                            c.g[A1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x4));
                                            let t = (c.g[A1][0] as i64) < 0;
                                            c.lq(2, (c.g[SP][0] as u32));
                                            if t {
                                                break 'b_2a5a68;
                                            } else {
                                                c.cycles += 10;
                                                c.g[A3][0] = sx((c.g[S6][0] as u32).wrapping_add(0x10));
                                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                c.g[A2][0] = 0x1;
                                                c.sq(2, (c.g[S6][0] as u32).wrapping_add(0x10));
                                                c.sw((c.g[SP][0] as u32).wrapping_add(0x7c), c.g[A3][0]);
                                                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                                                callee::FIND_INSTANCE.call(c, 0x002a5214)?;
                                                if c.g[V0][0] != 0u64 {
                                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x64), c.g[FP][0]);
                                                    break 'b_2a5a68;
                                                } else {
                                                    'b_2a54fc: {
                                                        'b_2a54d0: {
                                                            c.cycles += 167;
                                                            c.g[T0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0xd8));
                                                            c.g[A0][0] = 0x18;
                                                            c.g[S2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                            c.g[A1][0] = 0u64;
                                                            c.sw((c.g[SP][0] as u32).wrapping_add(0x80), c.g[T0][0]);
                                                            c.g[A2][0] = 0x8;
                                                            c.g[S0][0] = c.lw((c.g[S2][0] as u32));
                                                            c.g[S5][0] = c.lw((c.g[S6][0] as u32));
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                            c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                                            c.mult(0, 3, 3, 4);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                            c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1e3));
                                                            c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1dc));
                                                            c.ldl(8, (c.g[S0][0] as u32).wrapping_add(0x1eb));
                                                            c.ldr(8, (c.g[S0][0] as u32).wrapping_add(0x1e4));
                                                            c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1f3));
                                                            c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1ec));
                                                            c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                                                            c.sdr(7, (c.g[V1][0] as u32));
                                                            c.sdl(8, (c.g[V1][0] as u32).wrapping_add(0xf));
                                                            c.sdr(8, (c.g[V1][0] as u32).wrapping_add(0x8));
                                                            c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x17));
                                                            c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0x10));
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                            c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0u64);
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                            c.sw((c.g[V1][0] as u32), 0u64);
                                                            c.g[V0][0] = 0x18;
                                                            c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                                            c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
                                                            c.mult(0, 4, 4, 2);
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                                            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                                                            c.g[V1][0] = c.lb((c.g[A0][0] as u32));
                                                            c.sw((c.g[A3][0] as u32), c.g[V1][0]);
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                            c.sw((c.g[V0][0] as u32), 0u64);
                                                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                                                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
                                                            callee::MEMSET.call(c, 0x002a5304)?;
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[A0][0] = 0x64;
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
                                                            c.g[A1][0] = 0x5a;
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                            c.g[A2][0] = 0x32;
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                            c.g[A3][0] = 0x7f;
                                                            c.sw((c.g[V1][0] as u32), 0u64);
                                                            c.g[T0][0] = 0x18;
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                            c.sw((c.g[V0][0] as u32), 0u64);
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                            c.sw((c.g[V1][0] as u32), 0u64);
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                            c.sw((c.g[V0][0] as u32), 0u64);
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                            c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                            c.sw((c.g[V0][0] as u32), c.g[A0][0]);
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                            c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                            c.sw((c.g[V1][0] as u32), 0u64);
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                            c.sw((c.g[V0][0] as u32), 0u64);
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                            c.sw((c.g[V1][0] as u32), 0u64);
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                            c.sw((c.g[V0][0] as u32), c.g[A3][0]);
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
                                                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x78));
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                            c.sw((c.g[V1][0] as u32), c.g[A0][0]);
                                                            c.g[A0][0] = c.lw((c.g[S2][0] as u32));
                                                            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x80));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                            c.sw((c.g[V0][0] as u32), c.g[A1][0]);
                                                            c.g[V1][0] = c.lw((c.g[S2][0] as u32));
                                                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                            c.sw((c.g[V0][0] as u32), c.g[S5][0]);
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x7c));
                                                            c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                            c.sw((c.g[V1][0] as u32).wrapping_add(0x1fc), c.g[A2][0]);
                                                            c.g[A1][0] = c.lw((c.g[A0][0] as u32));
                                                            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                                                            c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1d8));
                                                            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                            c.sw((c.g[V0][0] as u32), c.g[FP][0]);
                                                            c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                                                            c.g[A3][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
                                                            c.mult(0, 4, 4, 8);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                            c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x20));
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                                            c.g[V0][0] = ((c.g[V0][0] as i64) < 127i64) as u64;
                                                            let t = c.g[V0][0] == 0u64;
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                                            if t {
                                                                break 'b_2a54d0;
                                                            } else {
                                                                c.cycles += 6;
                                                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                                                                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
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
                                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                                        c.g[V1][0] = ((c.g[V1][0] as i64) < 127i64) as u64;
                                                        if c.g[V1][0] == 0u64 {
                                                            c.g[V0][0] = 0x7f;
                                                            break 'b_2a54fc;
                                                        } else {
                                                            c.cycles += 4;
                                                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
                                                            c.g[V0][0] = c.lbu((c.g[V0][0] as u32));
                                                            break 'b_2a54fc;
                                                        }
                                                    }
                                                    c.cycles += 59;
                                                    c.sb((c.g[A0][0] as u32), c.g[V0][0]);
                                                    c.g[A2][0] = 0x1;
                                                    c.g[A3][0] = 0x4a0000;
                                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                    c.g[A1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x17c));
                                                    c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x214));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                    c.sw((c.g[V0][0] as u32), c.g[A1][0]);
                                                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                                                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x24c));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                    c.sw((c.g[V0][0] as u32), c.g[A2][0]);
                                                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                    c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x3700));
                                                    c.ldl(2, (c.g[A2][0] as u32).wrapping_add(0x7));
                                                    c.ldr(2, (c.g[A2][0] as u32));
                                                    c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x67));
                                                    c.sdr(2, (c.g[SP][0] as u32).wrapping_add(0x60));
                                                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                    c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x67));
                                                    c.ldr(7, (c.g[SP][0] as u32).wrapping_add(0x60));
                                                    c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x47));
                                                    c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x40));
                                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                    c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x238));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                    c.sw((c.g[V0][0] as u32), c.g[S7][0]);
                                                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                                                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x254));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                    c.sw((c.g[V0][0] as u32), 0u64);
                                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                    c.g[A1][0] = c.lw((c.g[S7][0] as u32));
                                                    c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                                                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x23c));
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                    c.sw((c.g[V0][0] as u32), c.g[A1][0]);
                                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                    callee::PLAY_PUSHED_SOUND.call(c, 0x002a55e0)?;
                                                    c.sw((c.g[S6][0] as u32).wrapping_add(0x4), c.g[V0][0]);
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
                                            c.g[S6][0] = sx((c.g[S7][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                            c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x8));
                                            if c.g[V1][0] == 0u64 {
                                                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                                                break 'b_2a5a5c;
                                            } else {
                                                'b_2a5a58: {
                                                    c.cycles += 3;
                                                    c.g[A1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x4));
                                                    let t = (c.g[A1][0] as i64) < 0;
                                                    c.g[A2][0] = 0x1;
                                                    if t {
                                                        break 'b_2a5a58;
                                                    } else {
                                                        'b_2a5a38: {
                                                            c.cycles += 13;
                                                            c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x20));
                                                            c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                            c.g[S4][0] = sx((c.g[S6][0] as u32).wrapping_add(0x10));
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                                                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[SP][0] as u32)));
                                                            c.lq(2, (c.g[V1][0] as u32).wrapping_add(0x10));
                                                            c.sq(2, (c.g[S6][0] as u32).wrapping_add(0x10));
                                                            c.g[A0][0] = c.lw((c.g[A3][0] as u32));
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                                                            callee::FIND_INSTANCE.call(c, 0x002a563c)?;
                                                            c.g[A2][0] = c.g[V0][0];
                                                            if c.g[A2][0] != 0u64 {
                                                                c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x20));
                                                                break 'b_2a5a38;
                                                            } else {
                                                                'b_2a594c: {
                                                                    'b_2a5920: {
                                                                        c.cycles += 176;
                                                                        c.g[A2][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0xd8));
                                                                        c.g[T0][0] = 0x18;
                                                                        c.g[S2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                                        c.g[A1][0] = 0u64;
                                                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x84), c.g[A2][0]);
                                                                        c.g[A2][0] = 0x8;
                                                                        c.g[A3][0] = c.lw((c.g[S6][0] as u32));
                                                                        c.g[S0][0] = c.lw((c.g[S2][0] as u32));
                                                                        c.sw((c.g[SP][0] as u32).wrapping_add(0x88), c.g[A3][0]);
                                                                        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
                                                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                                                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
                                                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                                                        c.mult(0, 3, 3, 8);
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                        c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1e3));
                                                                        c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1dc));
                                                                        c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x1eb));
                                                                        c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x1e4));
                                                                        c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1f3));
                                                                        c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1ec));
                                                                        c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                                                                        c.sdr(2, (c.g[V1][0] as u32));
                                                                        c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
                                                                        c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
                                                                        c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
                                                                        c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
                                                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0u64);
                                                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                        c.sw((c.g[V1][0] as u32), 0u64);
                                                                        c.g[V0][0] = 0x18;
                                                                        c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
                                                                        c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
                                                                        c.mult(0, 4, 4, 2);
                                                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                                                                        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                                                                        c.g[V1][0] = c.lb((c.g[A0][0] as u32));
                                                                        c.sw((c.g[A3][0] as u32), c.g[V1][0]);
                                                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                        c.sw((c.g[V0][0] as u32), 0u64);
                                                                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
                                                                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
                                                                        callee::MEMSET.call(c, 0x002a5734)?;
                                                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[A0][0] = 0x64;
                                                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
                                                                        c.g[A1][0] = 0x5a;
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                                        c.g[A2][0] = 0x32;
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                        c.g[A3][0] = 0x7f;
                                                                        c.sw((c.g[V1][0] as u32), 0u64);
                                                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                        c.sw((c.g[V0][0] as u32), 0u64);
                                                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                        c.sw((c.g[V1][0] as u32), 0u64);
                                                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                        c.sw((c.g[V0][0] as u32), 0u64);
                                                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                        c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
                                                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                        c.sw((c.g[V0][0] as u32), c.g[A0][0]);
                                                                        c.g[A0][0] = 0x1;
                                                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                                                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                        c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
                                                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                        c.sw((c.g[V1][0] as u32), 0u64);
                                                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                        c.sw((c.g[V0][0] as u32), 0u64);
                                                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                        c.sw((c.g[V1][0] as u32), 0u64);
                                                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                        c.sw((c.g[V0][0] as u32), c.g[A3][0]);
                                                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                        c.sw((c.g[V1][0] as u32), c.g[A0][0]);
                                                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32));
                                                                        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
                                                                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x84));
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                        c.sw((c.g[V0][0] as u32), c.g[A1][0]);
                                                                        c.g[V1][0] = c.lw((c.g[S2][0] as u32));
                                                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x88));
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                        c.sw((c.g[V0][0] as u32), c.g[A2][0]);
                                                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                                        c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                        c.sw((c.g[V1][0] as u32).wrapping_add(0x1fc), c.g[S4][0]);
                                                                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8c));
                                                                        c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x20));
                                                                        c.g[A3][0] = c.lw((c.g[A0][0] as u32));
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                        c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                        c.g[T0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1d8));
                                                                        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                                                                        c.g[A1][0] = c.lw((c.g[A2][0] as u32));
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                                        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x38));
                                                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                        c.g[A1][0] = 0x18;
                                                                        c.sw((c.g[V1][0] as u32), c.g[A0][0]);
                                                                        c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x1f4));
                                                                        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x1c));
                                                                        c.g[A2][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x38));
                                                                        c.mult(0, 4, 4, 5);
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                                        c.g[A1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x20));
                                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                                                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                                                        c.g[V0][0] = ((c.g[V0][0] as i64) < 127i64) as u64;
                                                                        let t = c.g[V0][0] == 0u64;
                                                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                                                        if t {
                                                                            break 'b_2a5920;
                                                                        } else {
                                                                            c.cycles += 6;
                                                                            c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x1c));
                                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                                                            let t = (c.g[V1][0] as i64) < 0;
                                                                            c.g[V0][0] = 0u64;
                                                                            if t {
                                                                                break 'b_2a594c;
                                                                            } else {
                                                                                break 'b_2a5920;
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 7;
                                                                    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x1c));
                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                                                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                                                    c.g[V1][0] = ((c.g[V1][0] as i64) < 127i64) as u64;
                                                                    if c.g[V1][0] == 0u64 {
                                                                        c.g[V0][0] = 0x7f;
                                                                        break 'b_2a594c;
                                                                    } else {
                                                                        c.cycles += 4;
                                                                        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x1c));
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
                                                                        c.g[V0][0] = c.lbu((c.g[V0][0] as u32));
                                                                        break 'b_2a594c;
                                                                    }
                                                                }
                                                                c.cycles += 59;
                                                                c.sb((c.g[A0][0] as u32), c.g[V0][0]);
                                                                c.g[A3][0] = 0x1;
                                                                c.g[A2][0] = 0x4a0000;
                                                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                                c.g[A1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x17c));
                                                                c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                                                                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x214));
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                c.sw((c.g[V0][0] as u32), c.g[A1][0]);
                                                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                                                                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x24c));
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                c.sw((c.g[V0][0] as u32), c.g[A3][0]);
                                                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                                c.g[A1][0] = c.lw((c.g[S7][0] as u32));
                                                                c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                                                                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x23c));
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                c.sw((c.g[V0][0] as u32), c.g[A1][0]);
                                                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                                c.g[A1][0] = c.lw((c.g[V1][0] as u32));
                                                                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3700));
                                                                c.ldl(8, (c.g[A0][0] as u32).wrapping_add(0x7));
                                                                c.ldr(8, (c.g[A0][0] as u32));
                                                                c.sdl(8, (c.g[SP][0] as u32).wrapping_add(0x67));
                                                                c.sdr(8, (c.g[SP][0] as u32).wrapping_add(0x60));
                                                                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1d8));
                                                                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
                                                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x67));
                                                                c.ldr(5, (c.g[SP][0] as u32).wrapping_add(0x60));
                                                                c.sdl(5, (c.g[A0][0] as u32).wrapping_add(0x47));
                                                                c.sdr(5, (c.g[A0][0] as u32).wrapping_add(0x40));
                                                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                                                                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x238));
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                c.sw((c.g[V0][0] as u32), c.g[S7][0]);
                                                                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                                c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                                                                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x254));
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                c.sw((c.g[V0][0] as u32), 0u64);
                                                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                                callee::PLAY_PUSHED_SOUND.call(c, 0x002a5a30)?;
                                                                c.sw((c.g[S6][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                                                break 'b_2a5a58;
                                                            }
                                                        }
                                                        c.cycles += 8;
                                                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8c));
                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                        c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                        c.g[A0][0] = c.lw((c.g[A1][0] as u32));
                                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                        c.sw((c.g[A2][0] as u32).wrapping_add(0x64), c.g[V1][0]);
                                                        break 'b_2a5a58;
                                                    }
                                                }
                                                c.cycles += 1;
                                                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
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
                                            c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x180));
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0x74), c.g[V0][0]);
                                            let t = c.g[V1][0] == 0u64;
                                            c.sw((c.g[SP][0] as u32).wrapping_add(0x70), c.g[V0][0]);
                                            if t {
                                                break 'b_2a5c38;
                                            } else {
                                                'b_2a5ab4: {
                                                    c.cycles += 14;
                                                    c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                    c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                                                    c.g[A1][0] = c.lw((c.g[S7][0] as u32));
                                                    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x74));
                                                    c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xc));
                                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
                                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                                                    let t = c.g[V0][0] as u32;
                                                    c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                    call_ptr(c, t, 0x002a5aa0)?;
                                                    c.g[S0][0] = c.g[V0][0];
                                                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1));
                                                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                                                    if c.g[V0][0] != 0u64 {
                                                        c.sw((c.g[S7][0] as u32).wrapping_add(0x18c), c.g[S0][0]);
                                                        break 'b_2a5ab4;
                                                    } else {
                                                        break 'b_2a5ab4;
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x180));
                                                c.g[V0][0] = 0x1;
                                                if c.g[V1][0] != c.g[V0][0] {
                                                    c.g[A1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x174));
                                                    break 'b_2a5c3c;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = 0x3;
                                                    if c.g[S0][0] == c.g[V0][0] {
                                                        c.g[A1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x174));
                                                        break 'b_2a5c3c;
                                                    } else {
                                                        c.cycles += 10;
                                                        c.g[A0][0] = c.g[S7][0];
                                                        callee::STOP_SOUND.call(c, 0x002a5ad8)?;
                                                        c.g[A0][0] = 0x54;
                                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                        c.mult(0, 4, 16, 4);
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4c));
                                                        callee::PICK_SAMPLE.call(c, 0x002a5af0)?;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[A2][0] = c.g[V0][0];
                                                        if t {
                                                            break 'b_2a5c38;
                                                        } else {
                                                            'b_2a5bbc: {
                                                                'b_2a5b90: {
                                                                    c.cycles += 32;
                                                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                                    c.g[A1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0xd8));
                                                                    callee::PUSH_PLAY_PARAMS.call(c, 0x002a5b04)?;
                                                                    c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                                    c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x10));
                                                                    c.g[A3][0] = 0x18;
                                                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32));
                                                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                                                                    c.sw((c.g[V1][0] as u32).wrapping_add(0x1fc), c.g[A1][0]);
                                                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x74));
                                                                    c.g[A1][0] = c.lw((c.g[A2][0] as u32));
                                                                    c.mult(0, 4, 30, 4);
                                                                    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                                                                    c.g[A2][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1d8));
                                                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                                    c.g[A0][0] = sx(((c.g[A0][0] as u32) as i32 >> 12) as u32);
                                                                    c.sw((c.g[V0][0] as u32), c.g[A0][0]);
                                                                    c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
                                                                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                                                                    c.g[T0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x38));
                                                                    c.mult(0, 4, 4, 7);
                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                                                    c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x20));
                                                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                                                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                                                    c.g[V0][0] = ((c.g[V0][0] as i64) < 127i64) as u64;
                                                                    let t = c.g[V0][0] == 0u64;
                                                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                                                    if t {
                                                                        break 'b_2a5b90;
                                                                    } else {
                                                                        c.cycles += 6;
                                                                        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                                                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                                                        let t = (c.g[V1][0] as i64) < 0;
                                                                        c.g[V0][0] = 0u64;
                                                                        if t {
                                                                            break 'b_2a5bbc;
                                                                        } else {
                                                                            break 'b_2a5b90;
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 7;
                                                                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                                                                c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                                                                c.g[V1][0] = ((c.g[V1][0] as i64) < 127i64) as u64;
                                                                if c.g[V1][0] == 0u64 {
                                                                    c.g[V0][0] = 0x7f;
                                                                    break 'b_2a5bbc;
                                                                } else {
                                                                    c.cycles += 4;
                                                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x1c));
                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
                                                                    c.g[V0][0] = c.lbu((c.g[V0][0] as u32));
                                                                    break 'b_2a5bbc;
                                                                }
                                                            }
                                                            c.cycles += 29;
                                                            c.sb((c.g[A0][0] as u32), c.g[V0][0]);
                                                            c.g[A2][0] = 0x1;
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                            c.g[A1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x17c));
                                                            c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                                                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x214));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                            c.sw((c.g[V0][0] as u32), c.g[A1][0]);
                                                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                            c.g[A0][0] = c.lw((c.g[V1][0] as u32));
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
                                                            c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x24c));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                                            c.sw((c.g[V0][0] as u32), c.g[A2][0]);
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                            c.g[A1][0] = c.lw((c.g[S7][0] as u32));
                                                            c.g[V1][0] = c.lw((c.g[A0][0] as u32));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
                                                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x23c));
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                            c.sw((c.g[V0][0] as u32), c.g[A1][0]);
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x608));
                                                            callee::MIDI_PLAY.call(c, 0x002a5c28)?;
                                                            let t = (c.g[V0][0] as i64) < 0;
                                                            c.sw((c.g[S7][0] as u32).wrapping_add(0x174), c.g[V0][0]);
                                                            if t {
                                                                break 'b_2a5c98;
                                                            } else {
                                                                c.cycles += 2;
                                                                c.g[V0][0] = 0x2;
                                                                c.sw((c.g[S7][0] as u32).wrapping_add(0x180), c.g[V0][0]);
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
                                            c.g[A0][0] = c.lw((c.g[V0][0] as u32));
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
                                            callee::FIND_INSTANCE.call(c, 0x002a5c54)?;
                                            c.g[S0][0] = c.g[V0][0];
                                            if c.g[S0][0] != 0u64 {
                                                c.g[V1][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x180));
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
                                                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x70));
                                                c.g[A0][0] = c.g[S0][0];
                                                callee::SET_PITCH.call(c, 0x002a5c88)?;
                                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x74));
                                                c.mult(0, 2, 30, 2);
                                                c.g[A0][0] = sx(((c.g[V0][0] as u32) as i32 >> 12) as u32);
                                                break 'b_2a5c94;
                                            }
                                        }
                                        c.cycles += 1;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x64), c.g[A0][0]);
                                        break 'b_2a5c98;
                                    }
                                }
                                c.cycles += 3;
                                c.lq(2, (c.g[SP][0] as u32));
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
    return Ok(());
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
        let mut result = 0u64;
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
        let mut result: u64 = 0;
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
                c.g[V0][0] = result;
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
        let mut last = 0u64;
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
            last = c.g[V0][0];
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
        c.g[V0][0] = c.lb((c.g[V1][0] as u32));
        let quoted = c.g[V0][0] == c.g[A0][0];
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
        if !quoted {
            c.cycles += 2;
            c.g[S3][0] = 0x22;
            loop {
                c.cycles += 8;
                c.g[A0][0] = c.g[S1][0];
                callee::INI_PARSER_GET_CHAR.call(c, 0x002a_7d50)?;
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                c.sb((c.g[V1][0] as u32), c.g[V0][0]);
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lb((c.g[V0][0] as u32));
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
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
        c.lq(17, frame.at(c, 0x40));
        c.lq(18, frame.at(c, 0x30));
        c.lq(19, frame.at(c, 0x20));
        c.lq(20, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        c.sb((c.g[V1][0] as u32), 0u64);
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
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S2][0] as u32)));
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S1][0] as u32)));
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
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
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
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
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
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
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
    c.g[A1][0] = c.lw((c.g[V0][0] as u32));
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
    if c.pc != 0x002a2498 { return Err(crate::call::Unwound); }
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
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
    c.pc = t; return Ok(());
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
    const STOP_ALL: Callee = Callee { addr: 0x002a_7718, f: ee::s08::CrowdAudio_StopAll };
    const FREE_LIST: Callee = Callee { addr: 0x002a_4cf8, f: ee::s08::CrowdAudio_FreeSampleList };
    const DESTROY: Callee =
        Callee { addr: 0x002a_4e38, f: ee::s08::CrowdAudio_DestroyInstance };
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
                c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                c.g[V0][0] = c.ldw_2(A0, 0);
                let t = c.g[V0][0] == 0u64;
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x28));
                if t {
                    break 'b_2a6414;
                } else {
                    'b_2a6390: {
                        c.cycles += 11;
                        c.g[V0][0] = c.ldw_2(S0, 0xd4);
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
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
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
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
    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.stw_2(V0, 0x1fc, 0u64);
    c.g[V1][0] = c.ldw_2(S0, 0x1f4);
    c.g[V0][0] = c.ldw_2(S1, 0x34);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.stw_2(V1, 0, 0u64);
    c.g[A3][0] = c.ldw_2(S0, 0x1f4);
    c.g[A0][0] = c.ldw_2(S0, 0x1f4);
    c.g[T0][0] = c.ldw_2(S1, 0x20);
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
    c.mult(0, 4, 4, 9);
    c.g[V0][0] = c.ldw_2(S1, 0x38);
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
    c.g[V1][0] = c.ldb(A0, 0);
    c.stw_2(A3, 0, c.g[V1][0]);
    c.g[V0][0] = c.ldw_2(S0, 0x1f4);
    c.g[V1][0] = c.ldw_2(S1, 0x3c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.stw_2(V0, 0, 0u64);
    c.g[A0][0] = c.ldw_2(S0, 0x1f4);
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
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
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.g[A3][0] = 0x7f;
    c.stw_2(V1, 0, 0u64);
    c.g[T0][0] = 0x1;
    c.g[V0][0] = c.ldw_2(S0, 0x1f4);
    c.g[V1][0] = c.ldw_2(S1, 0x64);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.stw_2(V0, 0, 0u64);
    c.g[V1][0] = c.ldw_2(S0, 0x1f4);
    c.g[V0][0] = c.ldw_2(S1, 0x68);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.stw_2(V1, 0, 0u64);
    c.g[V0][0] = c.ldw_2(S0, 0x1f4);
    c.g[V1][0] = c.ldw_2(S1, 0x6c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.stw_2(V0, 0, 0u64);
    c.g[V1][0] = c.ldw_2(S0, 0x1f4);
    c.g[V0][0] = c.ldw_2(S1, 0x70);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.stw_2(V1, 0x4, 0u64);
    c.g[V0][0] = c.ldw_2(S0, 0x1f4);
    c.g[V1][0] = c.ldw_2(S1, 0x70);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.stw_2(V0, 0, c.g[A0][0]);
    c.g[V1][0] = c.ldw_2(S0, 0x1f4);
    c.g[V0][0] = c.ldw_2(S1, 0x70);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.stw_2(V1, 0x8, c.g[A1][0]);
    c.g[V0][0] = c.ldw_2(S0, 0x1f4);
    c.g[V1][0] = c.ldw_2(S1, 0x70);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.stw_2(V0, 0xc, c.g[A2][0]);
    c.g[V1][0] = c.ldw_2(S0, 0x1f4);
    c.g[V0][0] = c.ldw_2(S1, 0x74);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.stw_2(V1, 0, 0u64);
    c.g[V0][0] = c.ldw_2(S0, 0x1f4);
    c.g[V1][0] = c.ldw_2(S1, 0x80);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.stw_2(V0, 0, 0u64);
    c.g[V1][0] = c.ldw_2(S0, 0x1f4);
    c.g[V0][0] = c.ldw_2(S1, 0x84);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.stw_2(V1, 0, 0u64);
    c.g[V0][0] = c.ldw_2(S0, 0x1f4);
    c.g[V1][0] = c.ldw_2(S1, 0x78);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.stw_2(V0, 0, c.g[A3][0]);
    c.g[V1][0] = c.ldw_2(S0, 0x1f4);
    c.g[V0][0] = c.ldw_2(S1, 0x7c);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.lq(16, c.pa(SP, 0x20));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
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
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[S4][0] = c.g[A2][0];
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[A1][0] = 0u64;
    c.g[S0][0] = c.lw((c.g[S2][0] as u32));
    c.g[A2][0] = 0x8;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
    c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
    c.mult(0, 3, 3, 9);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1e3));
    c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1dc));
    c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x1eb));
    c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x1e4));
    c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1f3));
    c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1ec));
    c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
    c.sdr(2, (c.g[V1][0] as u32));
    c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
    c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
    c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
    c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0u64);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[V1][0] as u32), 0u64);
    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
    c.mult(0, 4, 4, 9);
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
    c.g[V1][0] = c.lb((c.g[A0][0] as u32));
    c.sw((c.g[A3][0] as u32), c.g[V1][0]);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sw((c.g[V0][0] as u32), 0u64);
    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x002a854c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    c.pc = 0x00416210; call_indirect(c, 0x00416210);
    if c.pc != 0x002a854c { return; }
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[A0][0] = 0x64;
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
    c.g[A1][0] = 0x5a;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[A2][0] = 0x32;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.g[A3][0] = 0x7f;
    c.sw((c.g[V1][0] as u32), 0u64);
    c.g[T0][0] = 0x1;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sw((c.g[V0][0] as u32), 0u64);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[V1][0] as u32), 0u64);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sw((c.g[V0][0] as u32), 0u64);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sw((c.g[V0][0] as u32), c.g[A0][0]);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A1][0]);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[V1][0] as u32), 0u64);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sw((c.g[V0][0] as u32), 0u64);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[V1][0] as u32), 0u64);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sw((c.g[V0][0] as u32), c.g[A3][0]);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.sw((c.g[V1][0] as u32), c.g[T0][0]);
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    c.g[A0][0] = c.lw((c.g[S2][0] as u32));
    c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sw((c.g[V0][0] as u32), c.g[S3][0]);
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[V1][0] = c.lw((c.g[S2][0] as u32));
    c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
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
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[S4][0] = c.g[A2][0];
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[A1][0] = 0u64;
    c.g[S0][0] = c.lw((c.g[S2][0] as u32));
    c.g[A2][0] = 0x8;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
    c.sw((c.g[S0][0] as u32).wrapping_add(0x1f4), c.g[V0][0]);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
    c.mult(0, 3, 3, 9);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1e3));
    c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1dc));
    c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x1eb));
    c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x1e4));
    c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1f3));
    c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1ec));
    c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
    c.sdr(2, (c.g[V1][0] as u32));
    c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
    c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
    c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
    c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0u64);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[V1][0] as u32), 0u64);
    c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
    c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
    c.mult(0, 4, 4, 9);
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
    c.g[V1][0] = c.lb((c.g[A0][0] as u32));
    c.sw((c.g[A3][0] as u32), c.g[V1][0]);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sw((c.g[V0][0] as u32), 0u64);
    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
    c.g[RA][0] = sx(0x00290dbc);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    c.pc = 0x00416210; call_indirect(c, 0x00416210);
    if c.pc != 0x00290dbc { return; }
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[A3][0] = 0x64;
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
    c.g[T0][0] = 0x5a;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[T1][0] = 0x32;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.g[T2][0] = 0x7f;
    c.sw((c.g[V1][0] as u32), 0u64);
    c.g[T3][0] = 0x1;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = 0x5;
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[A2][0] = 0u64;
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[V1][0] as u32), 0u64);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sw((c.g[V0][0] as u32), 0u64);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[V1][0] as u32), 0u64);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[V1][0] as u32), c.g[A3][0]);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[T1][0]);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sw((c.g[V0][0] as u32), 0u64);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[V1][0] as u32), 0u64);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sw((c.g[V0][0] as u32), 0u64);
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.sw((c.g[V1][0] as u32), c.g[T2][0]);
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.sw((c.g[V0][0] as u32), c.g[T3][0]);
    c.g[V1][0] = c.lw((c.g[S2][0] as u32));
    c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.sw((c.g[V0][0] as u32), c.g[S3][0]);
    c.g[V1][0] = c.lw((c.g[S2][0] as u32));
    c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.sw((c.g[V0][0] as u32), c.g[S4][0]);
    c.g[RA][0] = sx(0x00290f18);
    c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x118));
    c.pc = 0x00287968; ee::s07::SSXAudioSystem_GetChannelGainPtr(c);
    if c.pc != 0x00290f18 { return; }
    c.g[A1][0] = c.lw((c.g[S0][0] as u32));
    c.g[A0][0] = c.g[S2][0];
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
    c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x214));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
    c.g[RA][0] = sx(0x00290f38);
    c.sw((c.g[V1][0] as u32), c.g[V0][0]);
    c.pc = 0x002906b8; ee::s07::Audio_PlayPushedSound(c);
    if c.pc != 0x00290f38 { return; }
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
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
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[RA][0] = sx(0x00299b94);
    c.g[S0][0] = c.g[A1][0];
    c.pc = 0x00288ae0; call_indirect(c, 0x00288ae0);
    if c.pc != 0x00299b94 { return; }
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
            c.g[S0][0] = c.lw((c.g[S3][0] as u32));
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
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1e3));
            c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1dc));
            c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x1eb));
            c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x1e4));
            c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1f3));
            c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1ec));
            c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
            c.sdr(2, (c.g[V1][0] as u32));
            c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
            c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
            c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
            c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0u64);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V1][0] as u32), 0u64);
            c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
            c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
            c.mult(0, 4, 4, 9);
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
            c.g[V1][0] = c.lb((c.g[A0][0] as u32));
            c.sw((c.g[A3][0] as u32), c.g[V1][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32), 0u64);
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.g[RA][0] = sx(0x00299c8c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
            c.pc = 0x00416210; call_indirect(c, 0x00416210);
            if c.pc != 0x00299c8c { return; }
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[A3][0] = 0x64;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
            c.g[T0][0] = 0x5a;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[T1][0] = 0x32;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[T2][0] = 0x7f;
            c.sw((c.g[V1][0] as u32), 0u64);
            c.g[T3][0] = 0x1;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = 0x9;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[A2][0] = 0u64;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V1][0] as u32), 0u64);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32), 0u64);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V1][0] as u32), 0u64);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V1][0] as u32), c.g[A3][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[T1][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32), 0u64);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V1][0] as u32), 0u64);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32), 0u64);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.sw((c.g[V1][0] as u32), c.g[T2][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
            c.sw((c.g[V0][0] as u32), c.g[T3][0]);
            c.g[V1][0] = c.lw((c.g[S3][0] as u32));
            c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
            c.sw((c.g[V0][0] as u32), 0u64);
            c.g[V1][0] = c.lw((c.g[S3][0] as u32));
            c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
            c.sw((c.g[V0][0] as u32), c.g[S2][0]);
            c.g[RA][0] = sx(0x00299de8);
            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
            c.pc = 0x00287968; call_indirect(c, 0x00287968);
            if c.pc != 0x00299de8 { return; }
            c.g[A1][0] = c.lw((c.g[S0][0] as u32));
            c.g[A0][0] = c.g[S3][0];
            c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
            c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x214));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
            c.g[RA][0] = sx(0x00299e08);
            c.sw((c.g[V1][0] as u32), c.g[V0][0]);
            c.pc = 0x002906b8; call_indirect(c, 0x002906b8);
            if c.pc != 0x00299e08 { return; }
        }
        c.cycles += 1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    }
    c.cycles += 6;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
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
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x6c74));
    c.g[T1][0] = 0x18;
    if c.g[V0][0] != 0 {
        c.cycles += 149;
        c.g[S0][0] = c.lw((c.g[S3][0] as u32));
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
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.ldl(2, (c.g[S0][0] as u32).wrapping_add(0x1e3));
        c.ldr(2, (c.g[S0][0] as u32).wrapping_add(0x1dc));
        c.ldl(4, (c.g[S0][0] as u32).wrapping_add(0x1eb));
        c.ldr(4, (c.g[S0][0] as u32).wrapping_add(0x1e4));
        c.ldl(7, (c.g[S0][0] as u32).wrapping_add(0x1f3));
        c.ldr(7, (c.g[S0][0] as u32).wrapping_add(0x1ec));
        c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
        c.sdr(2, (c.g[V1][0] as u32));
        c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
        c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
        c.sdl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
        c.sdr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x1fc), 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32), 0u64);
        c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[T0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x20));
        c.g[A3][0] = sx((c.g[A3][0] as u32) << 2);
        c.mult(0, 4, 4, 9);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[T0][0] as u32)));
        c.g[V1][0] = c.lb((c.g[A0][0] as u32));
        c.sw((c.g[A3][0] as u32), c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32), 0u64);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 3);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.g[RA][0] = sx(0x0029d46c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
        c.pc = 0x00416210; call_indirect(c, 0x00416210);
        if c.pc != 0x0029d46c { return; }
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[A3][0] = 0x64;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
        c.g[T0][0] = 0x5a;
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[T1][0] = 0x32;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.g[T2][0] = 0x7f;
        c.sw((c.g[V1][0] as u32), 0u64);
        c.g[T3][0] = 0x1;
        c.g[A1][0] = 0x5;
        c.g[A2][0] = 0u64;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[A0][0] = c.g[S3][0];
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x64));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32), 0u64);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x68));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32), 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6c));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32), 0u64);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32), c.g[A3][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x70));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 4);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32).wrapping_add(0xc), c.g[T1][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x74));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32), 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x80));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32), 0u64);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x84));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32), 0u64);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x78));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sw((c.g[V1][0] as u32), c.g[T2][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1f4));
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x7c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.sw((c.g[V0][0] as u32), c.g[T3][0]);
        c.g[V1][0] = c.lw((c.g[S3][0] as u32));
        c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
        c.sw((c.g[V0][0] as u32), c.g[S2][0]);
        c.g[V1][0] = c.lw((c.g[S3][0] as u32));
        c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1f4));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
        c.sw((c.g[V0][0] as u32), 0u64);
        c.g[RA][0] = sx(0x0029d5c8);
        c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x118));
        c.pc = 0x00287968; call_indirect(c, 0x00287968);
        if c.pc != 0x0029d5c8 { return; }
        c.g[A1][0] = c.lw((c.g[S0][0] as u32));
        c.g[A0][0] = c.g[S3][0];
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1f4));
        c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x214));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A2][0] as u32)));
        c.g[RA][0] = sx(0x0029d5e8);
        c.sw((c.g[V1][0] as u32), c.g[V0][0]);
        c.pc = 0x002906b8; call_indirect(c, 0x002906b8);
        if c.pc != 0x0029d5e8 { return; }
        c.sw((c.g[S3][0] as u32).wrapping_add(0x5fc8), c.g[V0][0]);
    }
    c.cycles += 9;
    c.g[V0][0] = 0x1;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.sw((c.g[S3][0] as u32).wrapping_add(0x5fc4), c.g[V0][0]);
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
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
            if callee::C0028bb10.call(c, 0x0028ffdc).is_err() {
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
        if callee::C002913d8.call(c, 0x00292b5c).is_err() {
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

        let mut out = false;
        callee::IS_PA_SPEECH_ON.call(c, 0x002a_372c)?;
        out = c.g[V0][0] == 0;
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
            c.g[S0][0] = c.lw((c.g[V0][0] as u32));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            let f = c.g[V0][0] as u32;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1d8));
            crate::call::call_ptr(c, f, 0x002a_378c)?;
            c.ldl(6, (c.g[S1][0] as u32).wrapping_add(7));
            c.ldr(6, (c.g[S1][0] as u32));
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::fill;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::*;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::put_words;
    use ssx3_ee as ee;
    use crate::difftest::put_vec;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::events::driven;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::SP as STACK_TOP;
    use crate::difftest::fakes::covered;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::elf_image;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::SP as STACK;
    use ssx3_ee_rt::funcs::GuestFn;
    use crate::difftest::SP;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::compare;
    use crate::difftest::seed_count;
    pub(crate) use crate::game::ai::rider::tests::put_world_2 as put_world;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::mixed;
    pub(crate) use crate::game::sound::bankmanager::tests::track_id;
    pub(crate) use crate::game::dirtysock::tags2::tests::SCRATCH;
    pub(crate) use crate::game::bx::bxstring::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS as SEEDS_3;
    pub(crate) use crate::game::worldpainter::quadtree::tests::pool_allocator;
    pub(crate) use crate::game::worldpainter::quadtree::tests::MEM_HEAP;
    pub(crate) use crate::game::worldpainter::quadtree::tests::run;
    pub(crate) use crate::game::worldpainter::quadtree::tests::heap_init;
    pub(crate) use crate::game::worldpainter::quadtree::tests::alloc_machine;
    pub(crate) use crate::game::fe::fememcard::tests::put_slot;
    pub(crate) use crate::game::dirtysock::tags2::tests::zero;
    pub(crate) use crate::game::fe::festaterewards2::tests::SEEDS as SEEDS_4;
    pub(crate) use crate::game::fe::festaterewards2::tests::w;
    pub(crate) use crate::game::fe::festaterewards2::tests::belib;
    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4 as SEEDS_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::run as run_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::call_machine;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS_6;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::gp_at;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::sound::bankmanager::tests::be_library_2 as be_library;
    pub(crate) use crate::game::sound::bankmanager::tests::race_type_test;
    pub(crate) use crate::game::wscript::wscriptman::tests::load;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::dummy_world;
    pub(crate) use crate::game::dirtysock::tags2::tests::SEEDS_7;
    pub(crate) use crate::game::dirtysock::tags2::tests::dummy_world as dummy_world_2;
    pub(crate) use crate::game::dirtysock::tags2::tests::common;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7 as SEEDS_8;
    pub(crate) use crate::game::render::particle::tests::common as common_2;
    pub(crate) use crate::game::object::railmodifier::tests::rng as rng_2;
    pub(crate) use crate::game::be::bestate2::tests::SEEDS as SEEDS_9;
    pub(crate) use crate::game::render::bezierman::tests::b;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_4 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::world;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool;
    pub(crate) use crate::game::main::ssxapp::tests::strict_check;
    pub(crate) use crate::game::lib::sif::tests::SEEDS as SEEDS_10;
    pub(crate) use crate::game::ai::ai::tests::fake_check;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ4;
    pub(crate) use crate::game::fe::feasyncfile::tests::VT;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_LIBRARY;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_3 as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_4 as globals;
    pub(crate) use crate::game::fe::ovtemplatedialog::tests::ZERO_STUB;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_2 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_5 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::W;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_4 as run_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_4 as heap_init_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base;
    pub(crate) use crate::game::object::effectlink::tests::alloc;
    pub(crate) use crate::game::camera::cameracontroller::tests::put_vtable;
    pub(crate) use crate::game::bx::bxstring::tests::put_str;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_11;
    pub(crate) use crate::game::dirtysock::tags2::tests::world_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_12;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::string_2 as string;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_13;
    pub(crate) use crate::game::main::gamestate::tests::base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_14;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_4;
    pub(crate) use crate::game::fe::festaterewards::tests::w16;
    pub(crate) use crate::game::fe::festaterewards::tests::slot_2 as slot_3;
    pub(crate) use crate::game::fe::festaterewards::tests::globals_3;
    pub(crate) use crate::game::fe::festaterewards::tests::pool_4 as pool_3;
    pub(crate) use crate::game::fe::festaterewards::tests::heap_init_3 as heap_init_5;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_9 as globals_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_7 as pool_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_9 as heap_init_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_5;
    pub(crate) use crate::game::fe::festateloadscreen::tests::r32_seed;
    pub(crate) use crate::game::ai::ai::tests::RIDER;
    pub(crate) use crate::game::sound::bankmanager::tests::SYS;
    pub(crate) use crate::game::ai::ai::tests::wf;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_10 as globals_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_8 as pool_5;
    pub(crate) use crate::game::bx::bxstring2::tests::W_2;
    pub(crate) use crate::game::bx::bxstring2::tests::RM as RM_2;
    pub(crate) use crate::game::bx::bxstring2::tests::RIDERS;
    pub(crate) use crate::game::bx::bxstring2::tests::BE_2 as BE_3;
    pub(crate) use crate::game::bx::bxstring2::tests::DB;
    pub(crate) use crate::game::bx::bxstring2::tests::VTS;
    pub(crate) use crate::game::bx::bxstring2::tests::base_3 as base_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_4 as OBJ2_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_11 as globals_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_9 as pool_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_8 as BE_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_6 as W_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS_15;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_9 as BE_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_7 as WORLD_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::RMAN;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDERS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDER as RIDER_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::check_all;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_8;
    pub(crate) use crate::game::bx::bxstring2::tests::f;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_12 as globals_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::ui;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_12 as heap_init_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_10 as pool_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;
    pub(crate) use crate::game::fe::festaterewards::tests::BE_7;
    pub(crate) use crate::game::fe::festaterewards::tests::W as W_5;
    pub(crate) use crate::game::fe::festaterewards::tests::HOOKS;
    pub(crate) use crate::game::fe::festaterewards::tests::KEEP_F0;
    pub(crate) use crate::game::fe::festaterewards::tests::hook;
    pub(crate) use crate::game::fe::festaterewards::tests::strict;
    pub(crate) use crate::game::fe::festaterewards::tests::base_9 as base_10;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_13 as globals_9;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_11 as pool_9;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_18 as base_11;
    pub(crate) use crate::game::render::graphicsman::tests::same;
    pub(crate) use crate::game::ai::ai::tests::generic;
    pub(crate) use crate::game::ai::ai::tests::globals_8 as globals_10;
    pub(crate) use crate::game::fe::fepopup::tests::BEL;
    pub(crate) use crate::game::fe::fepopup::tests::WORLD_4 as WORLD_3;
    pub(crate) use crate::game::fe::fepopup::tests::AUX;
    pub(crate) use crate::game::fe::fepopup::tests::SYS as SYS_3;
    pub(crate) use crate::game::fe::fepopup::tests::RACE;
    pub(crate) use crate::game::fe::fepopup::tests::RMAN_2;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all_2;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_12;
    pub(crate) use crate::game::dirtysock::tags2::tests::belib_3 as belib_2;


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
                c.g[A1][0] = if seed % 2 == 0 { 0 } else { r.next_u32() as u64 };
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
        check(ssx3_ee::s07::f_00287920, ssx_audio_system_get_channel_volume, 100, &|c, seed| {
            let mut r = Rng(0x1111_0007 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let idx = (seed % 8) as u64;
            c.g[A1][0] = idx;
            c.bus.write32(DATA + (idx as u32) * 0x10 + 0x62bc, r.edge_f32());
        });
    }


    #[test]
    fn crowd_audio_decode_source_handle_matches() {
        check(ssx3_ee::s08::f_002a72c0, crowd_audio_decode_source_handle, 200, &|c, seed| {
            let mut r = Rng(0x1111_000c ^ seed as u64);
            scramble(c, &mut r);
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.g[A2][0] = DATA as u64;
            c.g[A3][0] = (DATA + 4) as u64;
            c.bus.write32(DATA, r.next_u32());
            c.bus.write32(DATA + 4, r.next_u32());
        });
    }


    #[test]
    fn ini_parser_get_char_matches() {
        check(ssx3_ee::s08::f_002a78e0, ini_parser_get_char, 200, &|c, seed| {
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
        });
    }


    #[test]
    fn ps2_sound_manager_ctor_matches() {
        check(ssx3_ee::s08::f_002a8c20, ps2_sound_manager_ctor, 50, &|c, seed| {
            let mut r = Rng(0x1111_000e ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..210 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }


    #[test]
    fn ps2_sound_manager_pop_free_spu_block_matches() {
        check_lohi(ssx3_ee::s08::f_002a92b8, ps2_sound_manager_pop_free_spu_block, 200, &|c, seed| {
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
        });
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
        check(ee::s07::f_00290f58__generated, ssx_audio_system_set_player_trick_score, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x290f58, 6);
            let app = DATA;
            let world = DATA + 0x100;
            let man = DATA + 0x200;
            let racman = DATA + 0x300;
            let view = DATA + 0x400;
            put_world(c, world, man, racman);
            c.bus.write32(man + 0x28, 0); // no replay
            c.bus.write32(man + 0x84, view);
            c.bus.write32(view + 0x10, if seed % 4 == 1 { 2 } else { 1 });
            // An empty player table: FindRider answers 0 for both players.
            c.bus.write32(racman + 0x40, 0);
            c.bus.write32(racman + 0x78, 0);
            c.bus.write32(racman + 0x7c, 0);
            let rider = [0u32, 1, 2, 0xffff_ffff][(seed % 4) as usize];
            c.f[12] = r.f32(10.0);
            c.g[A0][0] = sx(app);
            c.g[A1][0] = sx(rider);
        });
    }


    #[test]
    fn ssx_audio_system_is_track_id_group5_matches() {
        check(ee::s08::f_002a3fd8__generated, ssx_audio_system_is_track_id_group5, SEEDS, &|c, seed| {
            let mut r = rng(0x002a_3fd8, seed);
            scramble(c, &mut r);
            track_id(c, &mut r, seed, 0x11, 5);
        });
    }


    #[test]
    fn ssx_audio_system_reset_wipeout_sfx_matches() {
        check(ee::s08::f_0029e578__generated, ssx_audio_system_reset_wipeout_sfx, SEEDS, &|c, seed| {
            let mut r = rng(0x0029_e578, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x1000);
            let voice = (seed % 8) as u32;
            c.bus.write32(DATA + 0x1000 + 0x870, voice);
            c.bus.write32(DATA + voice * 4 + 0x6080, r.next_u32());
        });
    }


    #[test]
    fn crowd_audio_instance_start_anticipate_matches() {
        check(ee::s08::f_002a67f0__generated, crowd_audio_instance_start_anticipate, SEEDS, &|c, seed| {
            let mut r = rng(0x002a_67f0, seed);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            c.bus.write32(DATA + 0x180, if seed % 3 == 0 { 0 } else { 1 });
        });
    }


    #[test]
    fn ssx_audio_system_set_rider_air_flag_matches() {
        check(ee::s07::f_00294880__generated, ssx_audio_system_set_rider_air_flag, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x29_4880, 0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (r.next_u32() % 12) as u64;
            c.g[A2][0] = r.next_u32() as u64;
        });
    }


    #[test]
    fn ssx_audio_system_tick_listener_matches() {
        check(ee::s08::f_00294f48__generated, ssx_audio_system_tick_listener, 100, &|c, seed| {
            let mut r = machine(c, seed, 0x294f48, 1);
            let this = SCRATCH;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = r.next_u32() as u64;
            // `*(this + 0xc)` is the listener vtable; slot `+8/+0xc` is the
            // tick method, which lands in the fake-call hook.
            let vt = fakes::vtable(c, &mut r, &[8]);
            c.bus.write32(this + 0xc, vt);
        });
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
        check(ee::s08::f_002a73a8__generated, crowd_audio_play_trick, 100, &|c, seed| {
            crowd_setup(c, seed, 0x2a73a8)
        });
    }


    #[test]
    fn crowd_audio_play_fall_matches() {
        check(ee::s08::f_002a73d8__generated, crowd_audio_play_fall, 100, &|c, seed| {
            crowd_setup(c, seed, 0x2a73d8)
        });
    }


    #[test]
    fn crowd_audio_stop_anticipate_matches() {
        check(ee::s08::f_002a7438__generated, crowd_audio_stop_anticipate, 100, &|c, seed| {
            crowd_setup(c, seed, 0x2a7438)
        });
    }


    // ------------------------------------------------------- sound / track slots

    #[test]
    fn ps2_sound_manager_v07_matches() {
        check(ee::s08::f_002a9288__generated, ps2_sound_manager_v07, SEEDS_2, &|c, seed| {
            let mut r = machine(c, seed, 0x2a9288, 1);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 0x30c, r.next_u32());
        });
    }


    #[test]
    fn audio_is_pa_speech_on_matches() {
        check(ssx3_ee::s08::f_0029f128__generated, audio_is_pa_speech_on, SEEDS_3, &|c, seed| {
            let mut r = machine(c, seed, 0x29f128, 0);
            let sys = DATA + 0x1000;
            // Channel 3 volume lives at +0x30 + 0x62bc.
            c.bus.write32(sys + 0x62ec, [0u32, 0x3f80_0000, 0x8000_0000, 0x4120_0000][seed as usize % 4]);
            let _ = r.next_u32();
            c.g[A0][0] = sx(sys);
        });
    }


    #[test]
    fn audio_is_arcade_speech_on_matches() {
        check(ssx3_ee::s08::f_0029f160__generated, audio_is_arcade_speech_on, SEEDS_3, &|c, seed| {
            let mut r = machine(c, seed, 0x29f160, 0);
            let sys = DATA + 0x1000;
            // Channel 0xa volume lives at +0xa0 + 0x62bc.
            c.bus.write32(sys + 0x635c, [0u32, 0xbf80_0000, 0x0000_0001, 0x7f80_0000][seed as usize % 4]);
            let _ = r.next_u32();
            c.g[A0][0] = sx(sys);
        });
    }


    #[test]
    fn crowd_audio_init_sample_list_matches() {
        check(ssx3_ee::s08::f_002a4ba8__generated, crowd_audio_init_sample_list, SEEDS_3, &|c, seed| {
            let mut r = machine(c, seed, 0x2a4ba8, 0);
            let audio = DATA + 0x100;
            fill(c, &mut r, audio, 0x20);
            c.g[A0][0] = sx(audio);
        });
    }


    // ------------------------------------------------------------- audio

    #[test]
    fn ps2_sound_manager_create_matches() {
        check(ee::s08::f_002a8fa0__generated, ps2_sound_manager_create, 40, &|c, seed| {
            alloc_machine(c, seed, 0x2a8f_a0);
        });
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
        check(ssx3_ee::s08::f_002a6808__generated, crowd_audio_instance_stop_anticipate, SEEDS_4, &|c, seed| {
            machine(c, seed, 0x2a68_08, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            w(c, DATA + 0x180, if seed % 2 == 0 { 2 } else { 0 });
        });
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
        race_type_test(ee::s08::f_002a4318__generated, ssx_audio_system_is_race_type6_to8, 0x2a43_18);
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
                c.bus.write32(this + 4, if seed % 2 == 0 { 0 } else { DATA + 0x1_2000 });
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
        check(ee::s08::f_002a79c8__generated, ini_parser_skip_comment, SEEDS_8, &|c, seed| {
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
        });
    }


    #[test]
    fn ps2_sound_manager_free_spu_block_matches() {
        check(ee::s08::f_002a92f8__generated, ps2_sound_manager_free_spu_block, SEEDS_8, &|c, seed| {
            let mut r = common_2(c, seed, 0x2a92f8, 1);
            clear(c, DATA, 0x100);
            clear(c, DATA + 0x200, 0x40);
            c.bus.write32(DATA + 0x200, r.next_u32());
            c.bus.write32(DATA + 0x208, r.next_u32());
            c.bus.write32(DATA + 0x30c, r.next_u32());
            c.bus.write32(DATA + 0x310, r.next_u32());
            c.bus.write32(DATA + 0x31c, if seed % 3 == 0 { 0 } else { r.next_u32() });
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x200);
        });
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
        check(ee::s07::f_00292ae8__generated, ssx_audio_system_kill_rider_timers, 24, &|c, seed| {
            let mut r = machine(c, seed, 0x0029_2ae8, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = 7;
            w(c, DATA + 0x118, DATA + 0x100);
            w(c, DATA + 0x100, DATA + 0x200);
            let vt = vtable(c, &mut r, &[0x20]);
            w(c, DATA + 0x200 + 0xab0, vt);
        });
    }


    #[test]
    fn ssx_audio_system_is_track_group2_event_matches() {
        check(ee::s08::f_002a4168__generated, ssx_audio_system_is_track_group2_event, 64, &|c, seed| {
            machine(c, seed, 0x002a_4168, 4);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 12) as u64;
            b(c, 0x0053_5c10, (seed % 4) as u8);
        });
    }


    #[test]
    fn ssx_audio_system_is_track_group3_event_matches() {
        check(ee::s08::f_002a41d8__generated, ssx_audio_system_is_track_group3_event, 64, &|c, seed| {
            machine(c, seed, 0x002a_41d8, 4);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 12) as u64;
            b(c, 0x0053_5c10, (seed % 4) as u8);
        });
    }


    #[test]
    fn ps2_sound_manager_dtor_matches() {
        check(ee::s08::f_002a8c50__generated, ps2_sound_manager_dtor, SEEDS_9, &|c, seed| {
            machine(c, seed, 0x002a_8c50, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 3) as u64;
            w(c, DATA + 4, if seed % 2 == 0 { 0 } else { DATA + 0x800 });
        });
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
        check(ee::s08::f_002a8070__generated, ini_parser_get_float, 8, &|c, seed| {
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
        });
    }


    #[test]
    fn ini_parser_get_int_matches() {
        check(ee::s08::f_002a80d8__generated, ini_parser_get_int, 8, &|c, seed| {
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
        });
    }


    /// A Rider with a main-state word the (real) `Rider_GetMainState` callee
    /// reads: `rider + 0x77c -> + 0xde0`.
    pub(crate) fn rider_state(c: &mut Cpu, r: &mut Rng, state: u32) -> u32 {
        let rider = DATA + 0x800;
        c.bus.write32(rider + 0x77c, DATA + 0x900);
        c.bus.write32(DATA + 0x900 + 0xde0, state);
        c.bus.write32(rider + 0x438, r.next_u32() % 24);
        rider
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
        strict_check(ssx3_ee::s07::f_002934d0__generated, ssx_audio_get_surface_group, 48, &|c, seed| {
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
        });
    }


    #[test]
    fn ssx_audio_get_surface_sound_type_matches() {
        strict_check(ssx3_ee::s07::f_00291710__generated, ssx_audio_get_surface_sound_type, 64, &|c, seed| {
            let mut r = machine(c, seed, 0x2917_10, 0);
            c.pc = 0x0029_1710;
            put_words(c, SURFACE_SOUND_TABLE, &SURFACE_SOUND_HANDLERS);
            let sys = DATA + 0x100;
            let rider = DATA + 0x800;
            c.g[A0][0] = sx(sys);
            c.g[A1][0] = sx(rider);
            c.bus.write32(rider + 0x77c, DATA + 0x900);
            c.bus.write32(DATA + 0x900 + 0xde0, if seed % 3 == 0 { 4 } else { (seed / 3) % 6 });
            // The surface id (+1 indexes the table): in range for 0..0x13,
            // past the end for 0x14 and above.
            let id = match seed % 5 {
                4 => 0x14 + (seed / 5) % 8,
                _ => (seed / 5) % 0x14,
            };
            c.bus.write32(rider + 0x438, id);
            c.bus.write32(sys + 0x598c, r.next_u32() % 0x14);
        });
    }


    // ------------------------------------------------------ audio mask tables

    #[test]
    fn character_to_speech_mask_matches() {
        fake_check(ssx3_ee::s08::f_0029f2b0__generated, character_to_speech_mask, 40, &|c, seed, r| {
            put_words(
                c,
                0x0048_2e60,
                &[
                    0x0029_f2e8, 0x0029_f2f8, 0x0029_f320, 0x0029_f2e0, 0x0029_f300,
                    0x0029_f318, 0x0029_f2d8, 0x0029_f310, 0x0029_f2f0, 0x0029_f308,
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
        });
    }


    #[test]
    fn character_to_voice_mask_matches() {
        fake_check(ssx3_ee::s08::f_002a1da0__generated, character_to_voice_mask, 41, &|c, seed, r| {
            put_words(
                c,
                0x0048_2fe0,
                &[
                    0x002a_1dd8, 0x002a_1de8, 0x002a_1e10, 0x002a_1dd0, 0x002a_1df0,
                    0x002a_1e08, 0x002a_1dc8, 0x002a_1e00, 0x002a_1de0, 0x002a_1df8,
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
        });
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
        c.bus.write32(VOICE + 0xd4, if seed % 5 == 0 { 0 } else { 1 });
        c.bus.write32(VOICE + 0xd8, r.next_u32() % 4);
        c.bus.write32(VOICE + 0x170, r.next_u32() % 4);
        c.bus.write32(VOICE + 0x174, (r.next_u32() % 5).wrapping_sub(1));
        c.bus.write32(VOICE + 0x178, r.next_u32());
        c.bus.write32(VOICE + 0x17c, r.next_u32() % 0x4000);
        c.bus.write32(VOICE + 0x180, r.next_u32() % 4);
        c.bus.write32(VOICE + 0x18c, INSTANCES + (r.next_u32() % 4) * INSTANCE_SIZE);
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
            c.bus.write32(slot + 0x8, if i == 0 { 0 } else { CHANNEL + 0x100 * i });
            c.bus.write32(slot + 0x20, i);
            let q = [r.f32(50.0), r.f32(50.0), r.f32(50.0), r.f32(1.0)];
            put_vec(c, slot + 0x10, q);
            // The sound the slot carries: three floats read as deltas.
            put_vec(c, SOUND + 0x100 * i, [r.f32(10.0), r.f32(10.0), r.f32(10.0), 0]);
            // The mixer channel: its index and frame counter.
            c.bus.write32(CHANNEL + 0x100 * i + 0x4, r.next_u32() % 0x100);
            c.bus.write32(CHANNEL + 0x100 * i + 0x20, i);
            put_vec(c, CHANNEL + 0x100 * i + 0x10, [0, 0, 0, 0]);
        }

        c.g[A0][0] = sx(VOICE);
    }


    #[test]
    fn crowd_audio_update_instance_matches() {
        check(ee::s08::f_002a4e88__generated, crowd_audio_update_instance, 150, &|c, seed| {
            fixture(c, seed);
        });
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
        check(ssx3_ee::s08::f_002a7c68__generated, ini_parser_find_section_prefix, SEEDS_14, &|c, seed| {
            let mut r = machine(c, seed, 0x2a7c68, 4);
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
        });
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
        check(ssx3_ee::s08::f_002a45c0__generated, ssx_audio_system_record_race_result, 10, &|c, seed| {
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
        });
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
        check(ee::s08::f_002a2018__generated, dj_say_20c8, 30, &|c, seed| {
            dj_setup(c, seed, 0x2a20)
        });
    }


    #[test]
    fn dj_say_20c9_matches() {
        check(ee::s08::f_002a20d0__generated, dj_say_20c9, 30, &|c, seed| {
            dj_setup(c, seed, 0x2a21)
        });
    }


    #[test]
    fn dj_say_20e7_matches() {
        check(ee::s08::f_002a2260__generated, dj_say_20e7, 30, &|c, seed| {
            dj_setup(c, seed, 0x2a22)
        });
    }


    #[test]
    fn dj_say_20e8_matches() {
        check(ee::s08::f_002a2318__generated, dj_say_20e8, 30, &|c, seed| {
            dj_setup(c, seed, 0x2a23)
        });
    }


    #[test]
    fn dj_say_20ba_matches() {
        check(ee::s08::f_002a2638__generated, dj_say_20ba, 30, &|c, seed| {
            dj_setup(c, seed, 0x2a26)
        });
    }


    #[test]
    fn dj_say_212b_matches() {
        check(ee::s08::f_002a2ad0__generated, dj_say_212b, 30, &|c, seed| {
            dj_setup(c, seed, 0x2a2a)
        });
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

    pub(crate) const ARR_2: u32 = DATA + 0x11_0000;

    pub(crate) const SI_2: u32 = DATA + 0x11_1000;


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


    /// A tiny trampoline for setup calls into guest code.
    pub(crate) fn load_2(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = 0x0012_3450;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, 0x0012_3450, "setup call {addr:#x} did not return");
        c.g[V0][0]
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
        diff(ee::s08::f_002a23d0__generated, ssx_audio_system_dj_say_20e9, 40, &|c, seed| {
            let mut r = fe_base(c, seed, 0x2a_23d0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = if seed % 2 == 0 { 0xffff_ffff_ffff_ffff } else { (seed % 4) as u64 };
            w(c, DATA + 0x5560, 0);
            w(c, DATA + 0x5578, 0);
            w(c, GAME_GP.wrapping_add(0x173c), 0x002c_c098);
            w(c, DATA + 0x3000, 0);
        });
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
        strict_2(ssx3_ee::s08::f_002a6f38__generated, crowd_audio_unload, SEEDS, &|c, seed| {
            let mut r = base_11(c, seed, 0x6f38);
            let this = DATA + 0x1000;
            // No instance array and no samples: StopAll and the per-bank
            // free loops all return at once.
            w(c, this + 0x18, 0);
            w(c, this + 0x14, 0);
            c.g[A0][0] = sx(this);
        });
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
        check_all_2(ee::s07::f_0028fea0__generated, midi_banks_base_dtor, 30, &|c, seed| {
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
        });
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
        diff(ee::s08::f_002a3708__generated, ssx_audio_system_pa_say20c5, 30, &|c, seed| {
            let mut r = machine(c, seed, 0x2a_3708, 4);
            belib_2(c);
            let audio = DATA + 0x1_0000;
            let rider = DATA + 0x2_0000;
            c.g[A0][0] = sx(audio);
            c.g[A1][0] = sx(rider);
            // Channel 3's volume: 0.0 on even seeds (speech off), 1.0 otherwise.
            c.bus
                .write32(audio + 0x62bc + 3 * 16, if seed % 2 == 0 { 0 } else { 0x3f80_0000 });
            // The replay/race-type/rider predicates stay false.
            c.bus.write32(audio + 0x6294, 0);
            c.bus.write32(audio + 0x530, 0);
            c.bus.write32(rider + 0x6c0, DATA + 0x3_0000);
            c.bus.write32(DATA + 0x3_0000 + 0x38, 0);
            let _ = r.next_u32();
        });
    }
}
