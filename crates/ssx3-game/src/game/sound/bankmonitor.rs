//! `src/sound/bankmonitor.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::tail_wrapper;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Target;
use crate::call::Unwound;
use crate::call::VSlot;
use crate::events;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::bestate2::ea;
pub(crate) use crate::game::bx::bxstring2::frame_less_return;
pub(crate) use crate::game::dirtysock::tags2::arg;
pub(crate) use crate::game::dirtysock::tags2::lw;
pub(crate) use crate::game::fe::feasyncfile::a;
pub(crate) use crate::game::fe::feasyncfile::at as at_2;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_3;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_2;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_3;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::lw_6 as lw_3;
pub(crate) use crate::game::fe::feasyncfile::lw_8 as lw_4;
pub(crate) use crate::game::fe::feasyncfile::lw_at_2 as lw_at;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_3;
pub(crate) use crate::game::fe::feasyncfile::shl32;
pub(crate) use crate::game::fe::feasyncfile::sw_6 as sw_2;
pub(crate) use crate::game::fe::fepopup::lb;
pub(crate) use crate::game::fe::festateload::at;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p_2;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festaterewards2::r;
pub(crate) use crate::game::fe::festaterewards2::Ops;
pub(crate) use crate::game::main::gamemode::low as low_2;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::sw_r;
pub(crate) use crate::game::main::gamestate::lw_at_3;
#[cfg(test)]
pub(crate) use crate::game::render::bezierman::f;
pub(crate) use crate::game::replay::replay2::destroy_if_flag;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::forward;
pub(crate) use crate::game::visualfx::worldlightman::deleting_dtor;
pub(crate) use crate::game::visualfx::worldlightman::DELETE;
pub(crate) use crate::game::world::wscriptcache::DELETE_2;
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
    (0x002a_7890, ini_parser_dtor, "IniParser_dtor"),
    (
        0x002a_7f90,
        ini_parser_at_section_end,
        "IniParser_AtSectionEnd",
    ),
    (0x002a_d2a8, bank_monitor_pause_all, "BankMonitor_PauseAll"),
    (
        0x002a_d300,
        bank_monitor_resume_all,
        "BankMonitor_ResumeAll",
    ),
    (
        0x002a_d398,
        bank_monitor_reset_instance,
        "BankMonitor_ResetInstance",
    ),
    (0x002a_d3c0, bank_monitor_stop_all, "BankMonitor_StopAll"),
    (
        0x002a_d410,
        bank_monitor_stop_by_callback,
        "BankMonitor_StopByCallback",
    ),
    (
        0x002a_d4d8,
        bank_monitor_stop_by_owner,
        "BankMonitor_StopByOwner",
    ),
    (
        0x002a_d650,
        bank_monitor_stop_by_bank,
        "BankMonitor_StopByBank",
    ),
    (
        0x002a_d810,
        bank_monitor_reset_distances,
        "BankMonitor_ResetDistances",
    ),
    (
        0x002a_d848,
        bank_monitor_timer_init,
        "BankMonitor_TimerInit",
    ),
    (
        0x002a_d860,
        bank_monitor_timer_dtor,
        "BankMonitor_TimerDtor",
    ),
    (
        0x002a_d930,
        bank_monitor_timer_is_free,
        "BankMonitor_TimerIsFree",
    ),
    (
        0x002a_d970,
        bank_monitor_timer_set_period,
        "BankMonitor_TimerSetPeriod",
    ),
    (
        0x002a_d980,
        bank_monitor_timer_enable,
        "BankMonitor_TimerEnable",
    ),
    (
        0x002a_d990,
        bank_monitor_timer_disable,
        "BankMonitor_TimerDisable",
    ),
    (0x002a_dc10, bank_monitor_tick, "BankMonitor_Tick"),
    (
        0x002a_dda0,
        bank_monitor_kill_all_timers,
        "BankMonitor_KillAllTimers",
    ),
    (0x002a_df60, audio_rand, "Audio_Rand"),
    (0x002a_dfb0, path_get_track_data, "PATH_GetTrackData"),
    (0x002a_e100, path_plat_shutdown, "PATH_PlatShutdown"),
    (0x002a_e138, path_plat_track_init, "PATH_PlatTrackInit"),
    (
        0x002a_e188,
        path_plat_remove_track_slot,
        "PATH_PlatRemoveTrackSlot",
    ),
    (
        0x002a_e1b8,
        path_plat_find_free_track_slot,
        "PATH_PlatFindFreeTrackSlot",
    ),
    (
        0x002a_e1f0,
        path_plat_bank_slot_init,
        "PATH_PlatBankSlotInit",
    ),
    (
        0x002a_e230,
        path_plat_find_bank_slot,
        "PATH_PlatFindBankSlot",
    ),
    (0x002a_e298, path_plat_add_bank, "PATH_PlatAddBank"),
    (0x002a_e328, path_plat_remove_bank, "PATH_PlatRemoveBank"),
    (
        0x002a_e3a8,
        path_plat_remove_oldest_bank,
        "PATH_PlatRemoveOldestBank",
    ),
    (
        0x002a_e478,
        path_plat_calc_track_mem_size,
        "PATH_PlatCalcTrackMemSize",
    ),
    (
        0x002a_e768,
        path_plat_set_track_file,
        "PATH_PlatSetTrackFile",
    ),
    (
        0x002a_e9f8,
        path_plat_clear_prebuffer,
        "PATH_PlatClearPrebuffer",
    ),
    (
        0x002a_ec28,
        path_plat_mark_track_dirty,
        "PATH_PlatMarkTrackDirty",
    ),
    (
        0x002a_efa8,
        path_plat_refresh_status,
        "PATH_PlatRefreshStatus",
    ),
    (0x002a_f2f0, path_plat_get_status, "PATH_PlatGetStatus"),
    (
        0x002a_f428,
        path_plat_get_track_volume,
        "PATH_PlatGetTrackVolume",
    ),
    (
        0x002a_f4b0,
        path_plat_get_track_aux_send,
        "PATH_PlatGetTrackAuxSend",
    ),
    (
        0x002a_f580,
        path_plat_set_track_level,
        "PATH_PlatSetTrackLevel",
    ),
    (0x002a_f608, path_plat_fade_volume, "PATH_PlatFadeVolume"),
    (0x002a_f7a0, path_plat_set_paused, "PATH_PlatSetPaused"),
    (0x002a_f8e0, speech_bank_ctor, "SpeechBank_ctor"),
    (0x002a_f900, speech_bank_dtor, "SpeechBank_dtor"),
    (0x002a_fa58, speech_bank_has_id, "SpeechBank_HasId"),
    (
        0x002b_0320,
        speech_instance_stop_stream,
        "SpeechInstance_StopStream",
    ),
    (
        0x002b_0348,
        speech_instance_add_bank,
        "SpeechInstance_AddBank",
    ),
    (0x002b_0da0, speechsystem_shutdown, "SpeechSystem_Shutdown"),
    (0x002b_0e60, spch_create_instance, "SPCH_CreateInstance"),
    (
        0x002b_11b0,
        speech_system_stop_instance,
        "SpeechSystem_StopInstance",
    ),
    (
        0x002b_1220,
        speech_system_get_instance_stream_slot,
        "SpeechSystem_GetInstanceStreamSlot",
    ),
    (
        0x002b_1458,
        speech_system_set_event_context,
        "SpeechSystem_SetEventContext",
    ),
    (
        0x002b_1720,
        speech_system_tick_event_contexts,
        "SpeechSystem_TickEventContexts",
    ),
    (
        0x002b_1a98,
        csong_configuration_ctor,
        "cSongConfiguration_ctor",
    ),
    (
        0x002b_1b68,
        c_song_configuration_dtor,
        "cSongConfiguration_dtor",
    ),
    (0x002b_1f78, song_instance_dtor, "SongInstance_dtor"),
    (0x002b_2018, song_instance_pause, "SongInstance_Pause"),
    (0x002b_2070, song_instance_resume, "SongInstance_Resume"),
    (
        0x002b_20c8,
        song_instance_post_event,
        "SongInstance_PostEvent",
    ),
    (
        0x002b_2120,
        song_instance_path_cmd_a,
        "SongInstance_PathCmdA",
    ),
    (0x002b_2418, song_instance_fade_out, "SongInstance_FadeOut"),
    (
        0x002b_24c0,
        song_instance_pick_random_loop,
        "SongInstance_PickRandomLoop",
    ),
    (
        0x002b_25c8,
        song_instance_get_request_status,
        "SongInstance_GetRequestStatus",
    ),
    (
        0x002b_2600,
        song_instance_get_position_ms,
        "SongInstance_GetPositionMs",
    ),
    (
        0x002b_2718,
        song_instance_get_ms_to_next_beat,
        "SongInstance_GetMsToNextBeat",
    ),
    (
        0x002b_2850,
        song_instance_apply_delay_params,
        "SongInstance_ApplyDelayParams",
    ),
    (0x002b_3a70, music_player_pause, "MusicPlayer_Pause"),
    (0x002b_3a98, music_player_resume, "MusicPlayer_Resume"),
    (0x002b_3ac0, music_player_stop_song, "MusicPlayer_StopSong"),
    (
        0x002b_3b88,
        music_player_post_event,
        "MusicPlayer_PostEvent",
    ),
    (
        0x002b_3bc0,
        music_player_set_event_state,
        "MusicPlayer_SetEventState",
    ),
    (
        0x002b_3c98,
        music_player_set_song_volume,
        "MusicPlayer_SetSongVolume",
    ),
    (
        0x002b_3cc0,
        music_player_set_song_field54,
        "MusicPlayer_SetSongField54",
    ),
    (0x002b_3d48, music_player_fade_out, "MusicPlayer_FadeOut"),
    (
        0x002b_3eb0,
        music_player_clear_playlist,
        "MusicPlayer_ClearPlaylist",
    ),
    (
        0x002b_3ee8,
        music_player_enable_song_by_name,
        "MusicPlayer_EnableSongByName",
    ),
    (
        0x002b_3fe8,
        music_player_enable_song,
        "MusicPlayer_EnableSong",
    ),
    (
        0x002b_40f0,
        music_player_get_song_title,
        "MusicPlayer_GetSongTitle",
    ),
    (
        0x002b_4120,
        music_player_get_song_artist,
        "MusicPlayer_GetSongArtist",
    ),
    (
        0x002b_4150,
        music_player_get_song_album,
        "MusicPlayer_GetSongAlbum",
    ),
    (
        0x002b_43d8,
        music_player_load_loop_bank,
        "MusicPlayer_LoadLoopBank",
    ),
    (
        0x002b_46d0,
        music_player_set_song_field50,
        "MusicPlayer_SetSongField50",
    ),
    (
        0x002b_46e8,
        music_player_are_loops_active,
        "MusicPlayer_AreLoopsActive",
    ),
    (
        0x002b_4868,
        song_instance_get_duck_to_loops,
        "SongInstance_GetDuckToLoops",
    ),
    (
        0x002b_4878,
        music_player_get_duck_to_loops,
        "MusicPlayer_GetDuckToLoops",
    ),
    (
        0x002b_48b8,
        song_instance_get_sed_value,
        "SongInstance_GetSedValue",
    ),
    (
        0x002b_4908,
        music_player_get_selected_song_sed_value,
        "MusicPlayer_GetSelectedSongSedValue",
    ),
    (
        0x002b_4920,
        music_player_get_ms_to_next_beat,
        "MusicPlayer_GetMsToNextBeat",
    ),
    (
        0x002b_4978,
        music_player_play_preview,
        "MusicPlayer_PlayPreview",
    ),
    (
        0x002b_49c0,
        music_player_set_song_tag,
        "MusicPlayer_SetSongTag",
    ),
    (
        0x002b_49e0,
        music_player_get_song_tag,
        "MusicPlayer_GetSongTag",
    ),
    (
        0x002b_4a08,
        music_player_set_song_field5c,
        "MusicPlayer_SetSongField5c",
    ),
    (
        0x002b_4a20,
        music_player_load_song_data,
        "MusicPlayer_LoadSongData",
    ),
    (
        0x002b_4aa0,
        music_player_is_song_data_loaded,
        "MusicPlayer_IsSongDataLoaded",
    ),
    (
        0x002b_4af0,
        music_player_is_current_song_enabled,
        "MusicPlayer_IsCurrentSongEnabled",
    ),
    (
        0x002b_9ce0,
        bank_monitor_set_param_callback,
        "BankMonitor_SetParamCallback",
    ),
    (
        0x003d_b040,
        spch_notify_part_callback,
        "SPCH_NotifyPartCallback",
    ),
    (0x002a_f6c0, path_plat_set_volume, "PATH_PlatSetVolume"),
    (
        0x002b_0290,
        speech_instance_is_stream_ready,
        "SpeechInstance_IsStreamReady",
    ),
    (0x002b_0c78, speech_system_update, "SpeechSystem_Update"),
    (
        0x002b_2160,
        song_instance_set_volume,
        "SongInstance_SetVolume",
    ),
    (0x003d_1e80, path_service_queues, "PATH_ServiceQueues"),
    (0x003d_5290, path_find_track, "PATH_FindTrack"),
    (
        0x002a_cfe0,
        bank_monitor_add_instance,
        "BankMonitor_AddInstance",
    ),
    (
        0x002a_eaa8,
        path_plat_get_queued_entry_value,
        "PATH_PlatGetQueuedEntryValue",
    ),
    (
        0x002b_23e8,
        song_instance_set_volume_percent,
        "SongInstance_SetVolumePercent",
    ),
    (0x002b_3c28, music_player_path_cmd_a, "MusicPlayer_PathCmdA"),
    (
        0x002b_3d10,
        music_player_set_volume_percent,
        "MusicPlayer_SetVolumePercent",
    ),
    (0x003d_54e8, path_update_tick42, "PATH_UpdateTick42"),
    (
        0x002a_ebd0,
        path_plat_get_track_state,
        "PATH_PlatGetTrackState",
    ),
    (
        0x003d_4950,
        path_track_get_queued_count,
        "PATH_TrackGetQueuedCount",
    ),
    (
        0x002a_cf60,
        bank_monitor_find_free_instance,
        "BankMonitor_FindFreeInstance",
    ),
    (0x002a_e7a8, path_plat_queue, "PATH_PlatQueue"),
    (0x003d_37a0, path_select_branch, "PATH_SelectBranch"),
    (0x003d_54c8, path_update_tick20, "PATH_UpdateTick20"),
    (
        0x002a_d5f0,
        bank_monitor_stop_handle,
        "BankMonitor_StopHandle",
    ),
    (
        0x002b_04d8,
        speech_instance_play_event,
        "SpeechInstance_PlayEvent",
    ),
];

/// Same-shard direct callees used above.
pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::ai::ai::callee::EA_FILE_FREE_FILE_HANDLE;
    pub(crate) use crate::game::ai::ai::callee::EA_FILE_FREE_REQUEST;
    pub(crate) use crate::game::ai::ai::callee::RIDER_ANIM_SEQ_END_TYPE00;
    pub(crate) use crate::game::ai::ai::callee::RIDER_ANIM_SEQ_END_TYPE01;
    pub(crate) use crate::game::ai::ai::callee::RIDER_ANIM_SEQ_END_TYPE02;
    pub(crate) use crate::game::ai::ai::callee::RIDER_ANIM_SEQ_END_TYPE03;
    pub(crate) use crate::game::ai::ai::callee::RIDER_ANIM_SEQ_END_TYPE04;
    pub(crate) use crate::game::ai::ai::callee::RIDER_ANIM_SEQ_END_TYPE05;
    pub(crate) use crate::game::ai::ai::callee::RIDER_ANIM_SEQ_END_TYPE06;
    pub(crate) use crate::game::ai::ai::callee::RIDER_ANIM_SEQ_END_TYPE07;
    pub(crate) use crate::game::ai::ai::callee::RIDER_ANIM_SEQ_END_TYPE08;
    pub(crate) use crate::game::ai::ai::callee::RIDER_ANIM_SEQ_END_TYPE09;
    pub(crate) use crate::game::ai::ai::callee::RIDER_ANIM_SEQ_END_TYPE0A;
    pub(crate) use crate::game::ai::ai::callee::RIDER_WIPEOUT_UPDATE_ANIM_SPEED;
    pub(crate) use crate::game::ai::ai::callee::SPCH_FIND_BANK_TYPE;
    pub(crate) use crate::game::be::beintoption::callee::BE_OPTION_INT_GET_MAX_HANDICAP;
    pub(crate) use crate::game::be::beintoption::callee::C_WORLD_SOUND_RELOCATE_EMITTERS;
    pub(crate) use crate::game::be::beintoption::callee::INSEQ_GET_INSTANCE;
    pub(crate) use crate::game::be::beintoption::callee::INSEQ_INSTANCE_STOP;
    pub(crate) use crate::game::be::beintoption::callee::IRRADIANCE_REF_GET_ENTRY as IRRADIANCE_REF_GET_ENTRY_2;
    pub(crate) use crate::game::be::beintoption::callee::REPLAY_FIND_SNAPSHOT;
    pub(crate) use crate::game::be::beintoption::callee::SCRIPT_ENGINE_CLEAR_PROCESS;
    pub(crate) use crate::game::be::beintoption::callee::STREAM_ARRAY_STOP;
    pub(crate) use crate::game::be::beintoption::callee::USTR_FROM_ASCII_2;
    pub(crate) use crate::game::be::beintoption::callee::WORLD_PAINTER_GET_IRRADIANCE_NAME2;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_BAKE_GROUP;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_EXPORT_HISTORY_FILES;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_FREE_HISTORIES;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_LOAD_HISTORY_FILE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_INSTANCE_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_INSTANCE_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE as BE_LIBRARY_GET_INSTANCE_4;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE as BE_LIBRARY_GET_INTERFACE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE_2 as BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_REWARD_INT_IS_UNLOCKED;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOLT_FILE_INDEX_MANAGER_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_DESTROY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_SET_PRESENCE_ADDR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE_2 as BUILTIN_DELETE_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAMERA_TRIGGER_STACK_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::CSETUP_MAN_SEND_PACKET;
    pub(crate) use crate::game::dirtysock::tags2::callee::CVOIP_MAN_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::CW_SCRIPT_CACHE_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_BX_STRING_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_LUNO_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_GET_SAVE_NAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_LIST_DIR_ALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_READ_SAVE;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SETUP_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SETUP_MAN_SEND_QUIT;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SHARED_LOGIN_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_VOIP_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::EATIMER_REMOVE_CALLBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEMSET;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_ALLOC as EA_MEM_ALLOC_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE as EA_MEM_FREE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::EMPTY_CHILD_HANDLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::EVENT_SELECT_ON_WIDGET_NOTIFY;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_MODULE_GET_PLAYER_PAD_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_CAMERA_SET_CONTROLLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_STATE_GOTO_NEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_CHANNEL_SET_LISTENER;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_INSTANCE_PAUSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_INSTANCE_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_UPDATE_CURSOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOAD_INDICATOR_CLEAR_BUSY;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_LIST_GET_AT_3 as LOBBY_LIST_GET_AT_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_FORWARD_EVENT_TO_UI;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_SEND_LOCK_CHAL;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MENU_ON_LOBBY_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LUNO_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_END_OPERATION_2 as MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_REQUEST_CARD_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_CLOSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_FLUSH;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_READ;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_WRITE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_ALLOC_NAMED_ALIGNED;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_CARD_ASCII_CHAR_TO_SJIS;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_CARD_SET_USE_SAVE_TITLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT0;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT1;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT2;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT3;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT4;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT5;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT6;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_ITEM_MEASURE_ROW;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_PAGE_SELECT_NEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_GET_INBOX;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_CTOR_3 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_DTOR_2 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_ON_CHILD_EVENT_3 as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE_2 as NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_GAME_UTIL_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::ON_QUIT_POPUP_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::dirtysock::tags2::callee::PATH_PLAT_REFRESH_STATUS;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_ADD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_HIGHLIGHT_SLOT_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RECORD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SAVE_EMIT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCE_MC_FORMAT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_CMD_QUEUE_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_CHOOSE_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_OBJ_TABLE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_PLAY_STATUS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_OBJ_CAM_SET_OWNER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_GET_WORLD_SECTION_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_PUSH_FREE_NODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SLOGIN_MC_GET_PORT_SLOT_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_FREE_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_FREE_MEMORY_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_GET_BUFFER_SIZE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_RELEASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLINK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_REMOVE_TICK_CALLBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SNOW_BOX_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPRINTF_2 as SPRINTF_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_ARCADE_SAY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_HANDLE_MUSIC_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_PLAY_CHARACTER_TUNE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::STATS_SELECT_CATEGORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY;
    pub(crate) use crate::game::dirtysock::tags2::callee::STREAM_BUFFER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::TROPHY_ROOM_RELEASE_IMAGES;
    pub(crate) use crate::game::dirtysock::tags2::callee::UIMENU_SET_SELECTION;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_SOUND_BASE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_3 as UI_STATE_CTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_TEXT_GET_FONT;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_RPC_SET_PARAMS;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_RPC_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::WSCRIPT_MAN_DELETE_ALL_PROCESSES;
    pub(crate) use crate::game::dirtysock::tags2::callee::WSCRIPT_MAN_SPAWN_MISSIONS;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_PUSH_2 as ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ANIM_CONTROLLER_IS_LAYER_ANIM_FINISHED;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_TRY_EXIT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATOI;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATTACHED_EMITTER_FX_DESTROY_EMITTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_LEVEL_TO_VOLUME_2 as AUDIO_LEVEL_TO_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_CHANNEL_VOLUME_2 as AUDIO_SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_DESTROY_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_RESET_ONE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_DECODE_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_IS_AUDIO_DONE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_IS_FRAME_DUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_INSTANCE_RESET as BANK_MONITOR_INSTANCE_RESET_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_KILL_ALL_TIMERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_KILL_TIMERS_BY_CALLBACK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BENEW_PLAYER_INT_GET_SLOT_2 as BENEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_BACKUP_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COPY_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_CHAR_DISPLAY_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE as BE_GET_INTERFACE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE_2 as BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_PROFILE_5 as BE_GET_RIDER_PROFILE_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_SOMETHING;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INSTANCE_4 as BE_LIBRARY_GET_INSTANCE_6;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INTERFACE_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_MISSION_INT_GET_SESSION_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_HUMAN_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_RACE_INT_GET_SPECIAL_EVENT_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_RACE_INT_GET_TRACK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_RESET_PROFILE_CHARACTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_CHAR_PROFILE_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_PHASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_FILE_ACQUIRE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_GET_NEXT_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_GET_PREV_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_NODE_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_API_DESTROY;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUILD_ITEM_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUILTIN_DELETE_6 as BUILTIN_DELETE_7;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_ALLOC_BUFFER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_CTOR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_FREE_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_PARSE_2 as CAMERA_TRIGGER_MAN_PARSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBXSTRING_ASSIGN;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFE_STATE_LOGIN_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFE_STATE_NET_CONFIG_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHAT_ROOM_SELECT_PLAYER_BY_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::CLEAR_MUSIC_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_DAY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_GENDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_LEGAL;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_MONTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_YEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_BX_STRING_ASSIGN_CSTR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_BX_STRING_CTOR_CSTR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_COMM_SYSTEM_CHANNEL_RECV;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_COMM_SYSTEM_CHANNEL_SEND;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_CUT_SCENE_TRANSITION_MAN_SAVE_VIEWPORT;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_POPUP_CONFIRM_EMPTY_INIT_HOOK;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_BUY_SELECTED_ITEM;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_REFRESH_ITEM_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_REMOVE_LIST_ITEM;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_UPDATE_MONEY_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_LOAD_CONFIG_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_SET_LIST_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_SHOW_CARD_ERROR_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_MESSENGER_CLEAR_NAME_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_QUICK_PLAY_POPUP_SET_OPTIONS_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_QUICK_PLAY_POPUP_SET_OPTIONS_HINT_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_SSX_WORLD_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_OPTIONS_DEVICE_SELECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_TROPHY_ROOM_SHOW_TROPHY_IMAGE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_TROPHY_ROOM_SHOW_TROPHY_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_USE_EXISTING_ACCOUNT_CHECK_EMAIL;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_SETUP_MAN_ALLOC_PACKET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_SETUP_MAN_FREE_PACKET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_TRANSITION_MAN_START_CAPTURED;
    pub(crate) use crate::game::fe::feasyncfile::callee::DATA_INSTANCE_FREE_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::DEBUG_MENU_DISPATCH_ITEM_OP;
    pub(crate) use crate::game::fe::feasyncfile::callee::EAFILE_GET_PATH_SIZE_CHECKED;
    pub(crate) use crate::game::fe::feasyncfile::callee::EAFILE_LOAD_FILE_INTO;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_ALLOC_NAMED_5 as EA_MEM_ALLOC_NAMED_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_FREE_4 as EA_MEM_FREE_5;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_FREE_5 as EA_MEM_FREE_6;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::FEMODULE_GET_RIDER_PREVIEW_2 as FEMODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::FEMODULE_RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ASYNC_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_FINISH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_START_PROFILE_SEARCH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_PLAYER_PAD_MASK_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_RELEASE_BOLT_FILES;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE as FE_MODULE_RIDER_PREVIEW_SET_VISIBLE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_UNLOAD_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FREE2;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_ADVANCE_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_QUIT_TO_LOAD_SCREEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_RAND_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_IS_LOADING;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_LANGUAGE_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::GRAPHICS_MAN_FREE_ALL_TEXTURES;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUTSYNC_IS_READY;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_BINDING_EVAL_FLOAT;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_EMPTY_STUB;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_NEXT_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_WRITE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_EVENT_TRACK_FIRE_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_EVENT_TRACK_NEXT_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_INSTANCE_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_CHANNEL_GET_LENGTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_SPEED;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_USER_DATA_2 as IN_SEQ_INSTANCE_SET_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_START_SEQUENCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::ISEQ_HEAP_ALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::ISEQ_SPAN_TRACK_ACTIVATE_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::ISEQ_SPAN_TRACK_NEXT_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_FIND_ENABLED_FROM_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_IS_KEY_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_POPUP_FIND_ENABLED_IN_COLUMN_FROM_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_POPUP_IS_KEY_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_AGREEMENT_TERMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_INDICATOR_ADVANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_INDICATOR_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_DESTROY_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_FLUSH_ROSTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_GET_RECORD;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_CONNECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_RELEASE_SNAPSHOT_LISTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_DECLINE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_TABLE_FIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_CALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_PASS_THROUGH;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_AS_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR_2 as LUNO_VM_REGISTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_FORMAT_SLOT_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_GET as MC_MANAGER_GET_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_GET_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MDF_FILE_CREATE_MODEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCPY_2 as MEMCPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC_NAMED_128;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_SET_ACTIVE_2 as MENU_SYS_SET_ACTIVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_SET_FLAG_2 as MESSAGE_CENTER_SET_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_EXTENSION_AND_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_GET_BITS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_MARKER_BIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_FADE_OUT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_PLAY_SONG;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_RESUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_STOP_SONG_2 as MUSIC_PLAYER_STOP_SONG;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_GOTO_NEXT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_OPEN_KEYBOARD_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_ACCOUNT_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_DTOR_4 as NET_FE_STATE_DTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_DTOR_5 as NET_FE_STATE_DTOR_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP as NET_FE_STATE_GET_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_3 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_SELECTED_USER;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GOTO_NEXT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_3 as NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_13 as NET_FE_STATE_ON_CHILD_EVENT_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_2 as NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_5 as NET_FE_STATE_SHOW_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_GAME_UTIL_ADVERT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_GAME_UTIL_CREATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED_4 as OPERATOR_NEW_NAMED_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPTIONS_HUD_REFRESH_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE_CALLEE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OV_STATE_OVERLAY_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_DATA_FIND_MARKER;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_FIND_BANK_SLOT as PATH_PLAT_FIND_BANK_SLOT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_RESUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SET_RUNNING;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SUSPEND;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_SET_OPTION_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST_2 as PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT_2 as PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_GRID_MESH_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::REFRESH_ITEM_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_COUNT_SAVE_SNAPSHOTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARDS_FILE_PARSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARD_GALLERY_INIT_PAGES;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARD_GALLERY_REQUEST_SHAPE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_ADD_WIPEOUT_RECOVERY_BOOST;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_BASE_RESET_FX;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_CHECK_RESET_PATH;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_GET_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_GET_START_POSE_ANGLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_IS_LOADED;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_METRIX_ON_RIDER_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_METRIX_SHOW_HUD_MSG_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_PLACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SCRIPT_OBJ_MATCHES_RIDER_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_BIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_4 as SCRIPT_CMD_QUEUE_AT_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_ADD_SCRIPT_OBJ;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_CHOOSE_SCRIPT as SCRIPT_ENGINE_CHOOSE_SCRIPT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_CLEAR_SKIP_DELAY_ON_EXIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE_2 as SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_TRANSITION_TARGET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_POP_FREE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_PUSH_FREE_NODE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_BINDING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_START_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_TICK_PROCESS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_3 as SCRIPT_LIST_IS_MOVIE_ENTRY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_SET_PAUSED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_RIDER_GET_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ADD_CAM;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_SET_TRANSITION;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SET_CATEGORY_TITLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SET_EVENT_CONTEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SHOW_BUY_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SHOW_EQUIP_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_IS_LOADED_WITH_NAME as SND_BANK_IS_LOADED_WITH_NAME_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_LOAD as SND_BANK_LOAD_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_UNLOAD_BANK_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK as SND_LOAD_QUEUE_CANCEL_BANK_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK_2 as SND_LOAD_QUEUE_CANCEL_BANK_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_PICK_RANDOM_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPCH_MAKE_ID_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_NEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_SYSTEM_IS_INSTANCE_READY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPLINE_GET_PATH_LENGTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_CONFIGURE_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_UPDATE_EMITTER_ENABLES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_6 as SPRINTF_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_7 as SPRINTF_5;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_CHARACTER_TO_SPEECH_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_FADE_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_IS_SPEECH_VOLUME_ON;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_SET_FADE_GAIN;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_SET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::START_SCREEN_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCASECMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::STR_HASH as STR_HASH_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::STR_HASH_5 as STR_HASH_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SYNC_PROCESS_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TAG_FIELD_GET_BINARY;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_HIDE_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRACK_TO_CATEGORY;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_DRAW_RECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_BASE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_ITEM_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_CREATE_SCREEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_GET_ACTIVE_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UISCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UISCREEN_START_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UITEXTSCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::feasyncfile::callee::UITEXTSCROLL_SET_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ENGINE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_3 as UI_LIST_DELETE_ALL_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_6 as UI_LIST_FIRST_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_4 as UI_LIST_NODE_DTOR_BODY_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_MENU_SET_SELECTION_3 as UI_MENU_SET_SELECTION_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_3 as UI_OBJECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_LOAD as UI_OBJECT_LOAD_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_5 as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_GET_OBJECT_BY_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LAUNCH_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LOOKUP_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_3 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_SET_OBJECT_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCROLL_SET_PARTS_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_CTOR_2_4 as UI_STATE_CTOR_2_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_ALL_POPUPS_2 as UI_STATE_MGR_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_TOP_STATES as UI_STATE_MGR_CLOSE_TOP_STATES_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_FIND_STATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_SWITCH_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXTURE_SET_FIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_GET_ID_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_GET_OWNED_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_SET_TEXT_ID_2 as UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_CPY_2 as USTR_CPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_LEN_2 as USTR_LEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_VSPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_LEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_SPRINTF_ASCII_FMT;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_TO_ASCII;
    pub(crate) use crate::game::fe::feasyncfile::callee::VEC3_MAX_COMPONENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_IS_HULL_FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_SOUND_GET_TRIGGER_IDS;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_CREATE_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_FINALIZE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_IS_FLAG4;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_MANAGER_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_CTOR;
    pub(crate) use crate::game::fe::festateload::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::fe::festateload::callee::BE_LIBRARY_GET_SAVE_DATA_SIZE;
    pub(crate) use crate::game::fe::festateload::callee::BOLT_FILE_CACHE_INIT;
    pub(crate) use crate::game::fe::festateload::callee::CHAR_SELECT_UPDATE_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::festateload::callee::CHAR_SETUP_UPDATE_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::festateload::callee::EA_MEM_FREE;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_SHUTDOWN;
    pub(crate) use crate::game::fe::festateload::callee::EVENT_SELECT_ON_UPDATE;
    pub(crate) use crate::game::fe::festateload::callee::I_SIGNAL_SEMA;
    pub(crate) use crate::game::fe::festateload::callee::MEM_CARD_BASE_DTOR;
    pub(crate) use crate::game::fe::festateload::callee::MENU_SYS_ITEM_ON_FOCUS;
    pub(crate) use crate::game::fe::festateload::callee::MENU_SYS_PAGE_VALIDATE_SELECTION;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_FORWARD_V42;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_FORWARD_V43;
    pub(crate) use crate::game::fe::festateload::callee::MODIFIER_BLOCK_HANDLE_EVENT;
    pub(crate) use crate::game::fe::festateload::callee::RIDER_METRIX_SET_SPIN_AMOUNT;
    pub(crate) use crate::game::fe::festateload::callee::RIDER_PREVIEW_APPLY_TRANSFORM;
    pub(crate) use crate::game::fe::festateload::callee::SONG_INSTANCE_PAUSE;
    pub(crate) use crate::game::fe::festateload::callee::SONG_INSTANCE_RESUME;
    pub(crate) use crate::game::fe::festateload::callee::STRLEN_COPY;
    pub(crate) use crate::game::fe::festateload::callee::UI_BASE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festateload::callee::UI_BASE_ON_SHUTDOWN;
    pub(crate) use crate::game::fe::festateload::callee::UI_STATE_V13;
    pub(crate) use crate::game::fe::festateload::callee::WORLD_GET_OBJECT_SCRIPT_FUNC;
    pub(crate) use crate::game::fe::festateload::callee::WORLD_TRIGGER_MANAGER_DTOR;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_CACHE_SET_DATA;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_MAN_RESET_COMMAND_QUEUE;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_PROCESS_GOTO_TASK;
    pub(crate) use crate::game::fe::festateload::callee::WSCRIPT_PROCESS_JUMP_TO_TASK;
    pub(crate) use crate::game::fe::festaterewards2::callee::ANIM_OBJECT_V36;
    pub(crate) use crate::game::fe::festaterewards2::callee::APP_MODULE_TRY_EXIT;
    pub(crate) use crate::game::fe::festaterewards2::callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY;
    pub(crate) use crate::game::fe::festaterewards2::callee::CROWD_AUDIO_INSTANCE_STOP_SOUND;
    pub(crate) use crate::game::fe::festaterewards2::callee::DL_HEAP_USABLE_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::DYNAMIC_EMITTER_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::EA_MUTEX_LOCK;
    pub(crate) use crate::game::fe::festaterewards2::callee::EA_MUTEX_UNLOCK;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_LOGIN_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_LOGIN_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_STATS_REFRESH_RESULTS;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEMSET;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_GET_POOL_BLOCK_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_MAN_FREE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_REPORT_POOL_STATS;
    pub(crate) use crate::game::fe::festaterewards2::callee::MOUNTAIN_ROOM_UPDATE_EVENT_ICONS;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_DISCONNECTED_POPUP_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_QUICK_PLAY_POPUP_ON_OPTIONS_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::POPUP_CONFIRM_GET_TEXT_BOX_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::PREVIEW_REWARD_POLL_IMAGE_LOAD;
    pub(crate) use crate::game::fe::festaterewards2::callee::PS2_PAD_CREATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_APPLY_CAMERA;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_FIND_NEXT_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_FIND_PREV_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_SET_STATE_2;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_SNAP_TO_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_STREAM_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SCROLL_ARROWS;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_COUNT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_ICONS;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_NAMES;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCE_USBKB_SET_ARRANGEMENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCE_USBKB_SET_READ_MODE;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_ENGINE_LIST_STOP;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_ENGINE_UNLOAD_WORLD_SCRIPTS;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_LIST_ADD_MOVIE;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_LIST_POLL_LOADING;
    pub(crate) use crate::game::fe::festaterewards2::callee::SECTION_MAN_FIND_LOGICAL_SECTION;
    pub(crate) use crate::game::fe::festaterewards2::callee::SONG_INSTANCE_GET_DUCK_TO_LOOPS;
    pub(crate) use crate::game::fe::festaterewards2::callee::SPRINTF;
    pub(crate) use crate::game::fe::festaterewards2::callee::S_LOGIN_UPDATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::TROPHY_ROOM_LOAD_TROPHY_TEXTURES;
    pub(crate) use crate::game::fe::festaterewards2::callee::TWSCRIPT_CACHE_RELOCATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_STATE_UPDATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SCROLL_SET_WIDE_STRING;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::fe::festaterewards2::callee::USB_KB_DEFAULT_CONNECT_CB;
    pub(crate) use crate::game::fe::festaterewards2::callee::WORLD_STATE_STREAM_RESET;
    pub(crate) use crate::game::fe::festaterewards2::callee::WORLD_TRIGGER_MANAGER_GET_TRIGGER_INFO;
    pub(crate) use crate::game::intersect::worldsphtree::callee::MEMCARD_SJIS_CHAR_TO_ASCII;
    pub(crate) use crate::game::intersect::worldsphtree::callee::MENU_SYS_ITEM_CTOR as MENU_SYS_ITEM_CTOR_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::MENU_SYS_ITEM_DRAW_ROW as MENU_SYS_ITEM_DRAW_ROW_3;
    pub(crate) use crate::game::intersect::worldsphtree::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER as MOD_OBJ_SET_TRANSFORM_MODIFIER_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::POSITION_MODIFIER_CTOR as POSITION_MODIFIER_CTOR_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::PS2_MEMCARD_SET_STATE;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_BINDING_HAS_PROCESS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_GET_INSTANCE as SCRIPT_ENGINE_GET_INSTANCE_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_GET_PROCESS as SCRIPT_ENGINE_GET_PROCESS_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_KILL_PROCESS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_STOP_PROCESS_SEQ;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SONG_INSTANCE_GET_BEAT_MS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SONG_INSTANCE_GET_POSITION_MS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP3;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SSX_AUDIO_SYSTEM_RESTART_PEAK_MUSIC;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WORLD_OBJ_CTOR_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WSCRIPT_PROCESS_LIST_CONTAINS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT as WSCRIPT_PROCESS_LIST_PUSH_FRONT_2;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WSCRIPT_PROCESS_LIST_REMOVE as WSCRIPT_PROCESS_LIST_REMOVE_2;
    pub(crate) use crate::game::main::gamestate::callee::ADF_DATA_SOURCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::ANIM_DATA_BASE_FREE_CHAN_CACHES;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_STOP_HANDLE as BANK_MONITOR_STOP_HANDLE_2;
    pub(crate) use crate::game::main::gamestate::callee::BE_ECON_INT_GET_TOTAL_EARNED;
    pub(crate) use crate::game::main::gamestate::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INSTANCE_3 as BE_LIBRARY_GET_INSTANCE_5;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INTERFACE_3;
    pub(crate) use crate::game::main::gamestate::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::main::gamestate::callee::BOLTS_FIND_BOLT_INDEX;
    pub(crate) use crate::game::main::gamestate::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_6;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_BRAGGING_RIGHTS_INIT_PLAYER_ROWS;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_CHAR_STATS;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::main::gamestate::callee::C_GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::main::gamestate::callee::C_KEYBOARD_POPUP_SET_KEY_ENABLED;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_ALLOC_NAMED_3 as EA_MEM_ALLOC_NAMED_2;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_FREE_4;
    pub(crate) use crate::game::main::gamestate::callee::GAME_ANIM_MAP_UNREGISTER_BANK;
    pub(crate) use crate::game::main::gamestate::callee::GAME_STATE_LOAD_STATE;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_CLEAR_SLOTS;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_INSTALL_VIF_VU_HANDLERS;
    pub(crate) use crate::game::main::gamestate::callee::MEMCARD_STATE_BASE_INSTALL_CALLBACKS;
    pub(crate) use crate::game::main::gamestate::callee::MEM_FREE;
    pub(crate) use crate::game::main::gamestate::callee::MEM_MOVE;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_GET_INPUT_CODE;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_HANDLE_INPUT;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_IS_DISABLED;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_NEXT as MENU_SYS_PAGE_SELECT_NEXT_2;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_PREV;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_VALIDATE_SELECTION as MENU_SYS_PAGE_VALIDATE_SELECTION_2;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_COUNT_GROUP_MESSAGES;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_GET_MESSAGE_COUNT;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_REMOVE_MESSAGE;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_ON_CHILD_EVENT_3;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_OPEN_KEYBOARD_TEXT_ID;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_REFRESH_LIST;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_COUNT_TEXT;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_SCROLL_ARROWS;
    pub(crate) use crate::game::main::gamestate::callee::RIDER_CLEAR_NEARBY_LIGHTS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_PAUSE_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_RESUME_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_CMD_QUEUE_AT_2;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_SCRIPT_OBJ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_IS_OBJ_BOUND_TO_PROCESS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_START;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_STOP_3;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_OBJ_CAM_HAS_BINDING;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_OBJ_CAM_IS_BOUND_TO_PROCESS;
    pub(crate) use crate::game::main::gamestate::callee::SECTION_ALLOCATOR_CTOR;
    pub(crate) use crate::game::main::gamestate::callee::SNDSTRM_CLOSE;
    pub(crate) use crate::game::main::gamestate::callee::SPCH_MAKE_ID;
    pub(crate) use crate::game::main::gamestate::callee::SPCH_SHUTDOWN;
    pub(crate) use crate::game::main::gamestate::callee::SPEECH_INSTANCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::main::gamestate::callee::SPHERE_TREE_COLLIDE_AABB;
    pub(crate) use crate::game::main::gamestate::callee::SPHERE_TREE_COLLIDE_SPHERE_TREE;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN_PTR;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_2;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_LANGUAGE_MASK;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_RESET_RACE_SOUNDS;
    pub(crate) use crate::game::main::gamestate::callee::STR_COPY;
    pub(crate) use crate::game::main::gamestate::callee::STR_HASH;
    pub(crate) use crate::game::main::gamestate::callee::STR_N_COPY;
    pub(crate) use crate::game::main::gamestate::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_FIRST_2;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_CTOR_3;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_SHUTDOWN_2 as UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::main::gamestate::callee::VD_TEXTURE_SINK_CTOR;
    pub(crate) use crate::game::render::bezierman::callee::CULL_AGAINST_LIGHTS;
    pub(crate) use crate::game::render::bezierman::callee::PROJECT_RIDER;
    pub(crate) use crate::game::render::bezierman::callee::RIDER_GET_DIST_TO_FINISH;
    pub(crate) use crate::game::render::bezierman::callee::RIDER_SPHERE_TREE_CTOR;
    pub(crate) use crate::game::render::bezierman::callee::RIDER_SPHERE_TREE_GET_BOUNDS;
    pub(crate) use crate::game::render::bezierman::callee::SND_LOCK;
    pub(crate) use crate::game::render::bezierman::callee::SND_STATUS;
    pub(crate) use crate::game::render::bezierman::callee::SND_STREAM_INFO;
    pub(crate) use crate::game::render::bezierman::callee::SND_STREAM_POSITION;
    pub(crate) use crate::game::render::bezierman::callee::SND_SUB_STATE;
    pub(crate) use crate::game::render::bezierman::callee::SND_SUB_STATUS;
    pub(crate) use crate::game::render::bezierman::callee::SND_SUB_TIME;
    pub(crate) use crate::game::render::bezierman::callee::SND_UNLOCK;
    pub(crate) use crate::game::render::particle::callee::ALGORITHM_LIST_DTOR;
    pub(crate) use crate::game::render::particle::callee::ANIM_OBJECT_BASE_DTOR;
    pub(crate) use crate::game::render::particle::callee::AVA_SPLINE_MODIFIER_CTOR;
    pub(crate) use crate::game::render::particle::callee::BANK_MONITOR_INSTANCE_PAUSE;
    pub(crate) use crate::game::render::particle::callee::BELIBRARY_GET_INSTANCE as BELIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::render::particle::callee::BELIBRARY_GET_INTERFACE as BELIBRARY_GET_INTERFACE_2;
    pub(crate) use crate::game::render::particle::callee::BENEW_PLAYER_INT_GET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::render::particle::callee::BENEW_RACE_INT_IS_STATION_TRACK_BY_INDEX;
    pub(crate) use crate::game::render::particle::callee::BEREWARD_ON_COLLECTIBLE_COLLECTED;
    pub(crate) use crate::game::render::particle::callee::BEREWARD_ON_MISSION_COMPLETED;
    pub(crate) use crate::game::render::particle::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_5;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_COPY;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_ASSIGN_CSTR;
    pub(crate) use crate::game::render::particle::callee::BXSTRING_CONCAT_IN_PLACE;
    pub(crate) use crate::game::render::particle::callee::CFE_STATE_CHAR_SELECT_UPDATE_ATTRIBUTE_BARS;
    pub(crate) use crate::game::render::particle::callee::CHASE_CAMERA_ALGORITHM_CTOR as CHASE_CAMERA_ALGORITHM_CTOR_2;
    pub(crate) use crate::game::render::particle::callee::CHASE_CAM_SET_ALGORITHM_IMMEDIATE;
    pub(crate) use crate::game::render::particle::callee::CPS2_CHUNK_RESOLVE_DTOR_STUB;
    pub(crate) use crate::game::render::particle::callee::CPS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::render::particle::callee::CROWD_MAN3D_GROUP_REMOVE_SECTION;
    pub(crate) use crate::game::render::particle::callee::CUT_SCENE_TRANSITION_SAVE_VIEWPORT;
    pub(crate) use crate::game::render::particle::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::render::particle::callee::FLEX_BRIDGE_SPRINGS_SAVE;
    pub(crate) use crate::game::render::particle::callee::INI_PARSER_SKIP_TO_EOL;
    pub(crate) use crate::game::render::particle::callee::ISIGNAL_SEMA;
    pub(crate) use crate::game::render::particle::callee::LOBBY_OCCUPANT_POP;
    pub(crate) use crate::game::render::particle::callee::LOBBY_OCCUPANT_SELECT;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::render::particle::callee::MENU_SYS_ITEM_DRAW_ROW_2;
    pub(crate) use crate::game::render::particle::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER;
    pub(crate) use crate::game::render::particle::callee::PATH_POST_EVENT;
    pub(crate) use crate::game::render::particle::callee::PATH_SET_PAUSED;
    pub(crate) use crate::game::render::particle::callee::POSITION_MODIFIER_CTOR;
    pub(crate) use crate::game::render::particle::callee::RIDER_ADD_BOOST;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_FINISH_PRESS;
    pub(crate) use crate::game::render::particle::callee::RIDER_BOARD_PRESS_SUB_SETTLE_SLEW;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_AWARD_POINTS;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_FINISH_BOARD_PRESS;
    pub(crate) use crate::game::render::particle::callee::RIDER_METRIX_ON_MISSION_COMPLETED;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_DELETE;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_MKDIR;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_READ;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_SEEK;
    pub(crate) use crate::game::render::particle::callee::SCE_MC_WRITE;
    pub(crate) use crate::game::render::particle::callee::SNDSTRM_SET_PITCH;
    pub(crate) use crate::game::render::particle::callee::SND_SPU_MEM_SET_BASE;
    pub(crate) use crate::game::render::particle::callee::SONG_INSTANCE_GET_MS_TO_NEXT_BEAT;
    pub(crate) use crate::game::render::particle::callee::SONG_INSTANCE_GET_STREAM_HANDLE;
    pub(crate) use crate::game::render::particle::callee::STRLEN;
    pub(crate) use crate::game::render::particle::callee::TEX_MAN_LOAD_TEXTURE;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_PUSH_BACK_2;
    pub(crate) use crate::game::render::particle::callee::UI_LIST_RESET;
    pub(crate) use crate::game::render::particle::callee::WORLD_CREATE_DEAD_NODE;
    pub(crate) use crate::game::render::particle::callee::WORLD_OBJ_SAVE as WORLD_OBJ_SAVE_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ANIM_OBJECT_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ATTACHED_DYNAMIC_EMITTER_CREATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::AUDIO_SYSTEM_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BANK_MONITOR_RESET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BANK_MONITOR_RESET_INSTANCE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BELIBRARY_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_CHARACTER as BE_GET_RIDER_CHARACTER_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_REWARD_INT_IS_PEAK_GOAL_COMPLETE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_CLEAR_LOGIN;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_CLEAR_MESSAGES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_DISCONNECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUILD_BUDDY_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_ALGORITHM_DEFAULT_FOV;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CONCAT_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CHANNEL_GET_TRACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_MAN2D_CLEAR_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_BX_STRING_COPY_BEFORE_WRITE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_FLAG_MANAGER_GROUP_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_TRANSITION_MAN_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_TABLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FIND_FREE_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_PROCESS_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_STATE_GOTO_NEXT_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GET_RACE_MAN;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GET_SELECTED_USER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GOTO_NEXT_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::HANDLE_NET_EVENT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_PACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_UNPACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INTC_TABLE_ADD_HANDLER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_INSTANCE_GET_REMAINING_STEPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_MASK_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_CURSOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_SCROLL_WINDOW;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LEVEL_TO_VOLUME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_ADD_VERTEX;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_CLOSE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LIST_GET_ENTRY_TYPE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LOAD_BANK_ASYNC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MATH_EVAL_CUBIC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_CARD_BASE_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_CARD_ON_CHILD_EVENT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_MAN_FREE_ENTRY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_CTOR as MENU_SYS_ITEM_CTOR_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_DRAW_ROW_4;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MOD_OBJ_CREATE_MODIFIER_BLOCK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MUSIC_PLAYER_GET_SELECTED_SONG_SED_VALUE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ONLINE_CHARACTER_SELECT_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ON_CHILD_EVENT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::OV_SCREEN_PROFILE_SAVE_PDA_REFRESH_CARD_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::OV_SCREEN_PROFILE_SAVE_PDA_RESET_SLOT_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PATH_PLAT_FIND_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RAIL_MODIFIER_LIST_PUSH_FRONT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_END_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_SAVE_EMIT as REPLAY_SAVE_EMIT_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_SET_CONTROLLER_INDEX;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_STREAM_REWIND_READ;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REQUEST_IMAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_HAS_RENDER_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_SAY_20B9;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_4 as SCRIPT_BINDING_CTOR_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO_4 as SCRIPT_DATA_GET_INFO_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_LIST_STOP_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_CAN_SKIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_GET_PLAY_STATUS_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_3 as SCRIPT_LIST_IS_PLAYING_SCRIPT_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR_3 as SCRIPT_OBJ_CTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SEND_PACKET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SEND_CHARACTER_MSG;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SET_LOCAL_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SET_SELECTED_USER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SHOW_BUDDY_ACTION_POPUP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SPEECH_SYSTEM_CANCEL_PENDING_INSTANCE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SSX_AUDIO_SYSTEM_DJ_SAY_20E5;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::START_RIDER_BOARD_LOOPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::STOP_BOARD_LOOPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SYNC_PROCESS_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_SCRIPT_SET_OWNER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_ASCII_TO_WIDE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_CTOR_2 as UI_LIST_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_FIRST_5 as UI_LIST_FIRST_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_DTOR_BODY as UI_LIST_NODE_DTOR_BODY_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_MENU_SET_SELECTION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_SCREEN_FIND_OBJECT_3 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_STATE_CTOR_2_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_TEXT_SET_WIDE_STRING_INTERNAL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VEC3_NEG_DOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VID_ENGINE_INSTALL_MEM_HOOKS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VID_ENGINE_PLAYER_INIT_OPEN_PARAMS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WSCRIPT_MAN_DEACTIVATE_PROCESS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WSCRIPT_MAN_UNLOAD_SECTION;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::CBXSTRING_ASSIGN_CSTR;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::C_WORLD_SOUND_UNLINK_EMITTERS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::EA_MEM_COPY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::EVENT_SELECT_HANDLE_INPUT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::GRAPHICS_MAN_EMPTY_HOOK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::IRRADIANCE_REF_GET_ENTRY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::LOBBY_MAN_REQUEST_STATS_PAGE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::MC_MANAGER_GET_INSTANCE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::MENU_SYS_GET_INPUT_CODE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::PATH_DATA_READ_FLOAT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::PATH_SET_TRACK_CONTROL_VALUE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RAND_NEXT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RAND_NEXT_MAIN;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RIDER_METRIX_BANK_COMBO;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_DUMMY_OBJ_READ_FIELDS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_ENGINE_PUSH_FREE_NODE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_OBJ_DEACTIVATE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SSX_APP_BOOT_LOAD;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STATS_PROCESS_RENDER_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STATS_PROCESS_UPDATE_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STREAM_BUFFER_GET_STATUS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STRLEN_AND_STRCPY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::TEX_MAN_SET_ALLOC_FLAGS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_NODE_UNLINK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_PUSH_BACK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_STATE_MGR_RENDER_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::USTR_FROM_ASCII;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_RESET_SECTIONS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_TRIGGER_GET_INFO;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_TRIGGER_GET_NAME;
    pub(crate) use crate::game::world::wscriptcache::callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR;
    pub(crate) use crate::game::world::wscriptcache::callee::BANK_MONITOR_DELETE_INSTANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::BELIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::BELIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::world::wscriptcache::callee::BE_NEW_RACE_INT_GET_TRACK_INFO;
    pub(crate) use crate::game::world::wscriptcache::callee::BE_SCORE_INT_GET_HIGH_SCORE_TABLE;
    pub(crate) use crate::game::world::wscriptcache::callee::BOLT_FILE_CACHE_SHUTDOWN;
    pub(crate) use crate::game::world::wscriptcache::callee::CAMERA_RESET_CHASE_CONTROLLER_SWITCHES;
    pub(crate) use crate::game::world::wscriptcache::callee::CAMERA_TRIGGER_STACK_CLEAR as CAMERA_TRIGGER_STACK_CLEAR_2;
    pub(crate) use crate::game::world::wscriptcache::callee::CHASE_CAMERA_ALGORITHM_CTOR_2 as CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::CINSTANCE_NODE_CTOR_V;
    pub(crate) use crate::game::world::wscriptcache::callee::CROWD_AUDIO_FREE_SAMPLE_LIST;
    pub(crate) use crate::game::world::wscriptcache::callee::CUT_SCENE_TRANSITION_MAN_BASE_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::CUT_SCENE_TRANSITION_MAN_INIT;
    pub(crate) use crate::game::world::wscriptcache::callee::C_WORLD_CACHE_STREAMER_DTOR_STUB;
    pub(crate) use crate::game::world::wscriptcache::callee::DELETE;
    pub(crate) use crate::game::world::wscriptcache::callee::EA_MEM_FREE_2 as EA_MEM_FREE_3;
    pub(crate) use crate::game::world::wscriptcache::callee::FONT_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::INPUT_PREPROC_WRITE_STRING;
    pub(crate) use crate::game::world::wscriptcache::callee::MDF_FILE_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::MPEG2_FLUSH_BUFFER_3 as MPEG2_FLUSH_BUFFER;
    pub(crate) use crate::game::world::wscriptcache::callee::MPEG2_GET_BITS1;
    pub(crate) use crate::game::world::wscriptcache::callee::PS2_SONY_CODEC_INTERNAL_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::RCMP_SYSTEM_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::RIDER_SET_MAIN_STATE;
    pub(crate) use crate::game::world::wscriptcache::callee::RIDER_SET_SUB_STATE;
    pub(crate) use crate::game::world::wscriptcache::callee::SCRIPT_SOUND_BANK_MAN_FIND_BANK;
    pub(crate) use crate::game::world::wscriptcache::callee::SECTION_ALLOCATOR_FREE_ALL;
    pub(crate) use crate::game::world::wscriptcache::callee::SND_BANK_MONITOR_MEM_SIZE;
    pub(crate) use crate::game::world::wscriptcache::callee::SPRINTF_2;
    pub(crate) use crate::game::world::wscriptcache::callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP5;
    pub(crate) use crate::game::world::wscriptcache::callee::SSX_AUDIO_SYSTEM_PAUSE;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_ANIM_ADVANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_ANIM_APPLY;
    pub(crate) use crate::game::world::wscriptcache::callee::WSCRIPT_MAN_COLLECT_SET_ADD_ITEMS;
    pub(crate) use crate::game::world::wscriptcache::callee::WSCRIPT_MAN_COLLECT_SET_GET_COUNT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::AI_VFX_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::BUILTIN_DELETE_2;
    pub(crate) use crate::game::worldpainter::quadtree::callee::CONSOLE_CONFIG_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_MEM_ALLOC;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EMITTER_RENDER;
    pub(crate) use crate::game::worldpainter::quadtree::callee::GET_TRANSFORM_BOUNDS;
    pub(crate) use crate::game::worldpainter::quadtree::callee::GRAPHICS_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::GRAPHICS_ON_VIF1_DMA_END;
    pub(crate) use crate::game::worldpainter::quadtree::callee::INTC_REMOVE_HANDLER;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MEM_CARD_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MEM_CARD_INIT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MEM_CARD_SET_STATE;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MENU_ITEM_MEASURE_ROW;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MOD_OBJ_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::PS2_SOUND_MANAGER_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SNDSTRM_GET_REQUEST_STATUS;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SONG_FADE_OUT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SONG_GET_STREAM_STATUS;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SONG_POST_EVENT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SPEECH_INSTANCE_STOP_STREAM;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STATS_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STRCPY as STRCPY_2;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STREAM_BUFFER_SET_FILE;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STREAM_BUFFER_STOP;
    pub(crate) use crate::game::worldpainter::quadtree::callee::VFX_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::WORLD_GET_SECTION_SCRIPT_FUNC;
    pub(crate) use crate::game::worldpainter::quadtree::callee::WORLD_OBJ_CTOR;

    pub const GAME_STATE_GOTO_NEXT_2: Callee = Callee {
        addr: 0x0023_3aa0,
        f: ee::s06::f_00233aa0,
    };

    pub const PS2_MEMCARD_CREATE: Callee = Callee {
        addr: 0x002c_22c8,
        f: ee::s08::f_002c22c8,
    };

    pub const LOBBY_LIST_GET_AT: Callee = Callee {
        addr: 0x003e_e560,
        f: ee::s14::f_003ee560,
    };

    pub const BUDDY_MAN_CONNECT: Callee = Callee {
        addr: 0x0026_1248,
        f: ee::s06::f_00261248,
    };

    pub const STR_CASE_CMP: Callee = Callee {
        addr: 0x0041_aa88,
        f: ee::s15::f_0041aa88,
    };

    pub const SLOGIN_MC_GET_PORT_SLOT: Callee = Callee {
        addr: 0x003f_a1d0,
        f: ee::s14::f_003fa1d0,
    };

    pub const MEMCARD_REQUEST_GET_INFO_NOW: Callee = Callee {
        addr: 0x002c_67e8,
        f: ee::s08::f_002c67e8,
    };

    pub const PATH_DATA_READ_INT: Callee = Callee {
        addr: 0x0026_ba68,
        f: ee::s07::f_0026ba68,
    };

    pub const PATH_DATA_READ_VEC3: Callee = Callee {
        addr: 0x0026_ba48,
        f: ee::s07::f_0026ba48,
    };

    pub const PATH_DATA_READ_FLOAT_2: Callee = Callee {
        addr: 0x0026_ba88,
        f: ee::s07::f_0026ba88,
    };

    pub const SCRIPT_LIST_ADD_CHOICE: Callee = Callee {
        addr: 0x0027_5ed0,
        f: ee::s07::f_00275ed0,
    };

    pub const SCRIPT_ENGINE_POLL_ASYNC_LOADS: Callee = Callee {
        addr: 0x0027_8918,
        f: ee::s07::f_00278918,
    };

    pub const SSX_AUDIO_SYSTEM_GET_INSTANCE: Callee = Callee {
        addr: 0x0028_b180,
        f: ee::s07::f_0028b180,
    };

    pub const SSX_AUDIO_SYSTEM_END_SCRIPT_MUSIC: Callee = Callee {
        addr: 0x0028_e888,
        f: ee::s07::f_0028e888,
    };

    pub const WORLD_PAINTER_SET_SCRIPT_SURFACE_MODE: Callee = Callee {
        addr: 0x002e_f368,
        f: ee::s09::f_002ef368,
    };

    pub const SCRIPT_BINDING_CTOR: Callee = Callee {
        addr: 0x0028_2f60,
        f: ee::s07::f_00282f60,
    };

    pub const SCRIPT_ENGINE_GET_INSTANCE: Callee = Callee {
        addr: 0x0027_d2e8,
        f: ee::s07::f_0027d2e8,
    };

    pub const SND_BANK_MAN_UNLOAD_MEMORY_BANK: Callee = Callee {
        addr: 0x0028_be90,
        f: ee::s07::f_0028be90,
    };

    pub const MUSIC_PLAYER_GET_SONG_TAG: Callee = Callee {
        addr: 0x002b_49e0,
        f: ee::s08::f_002b49e0,
    };

    pub const SPEECH_SYSTEM_STOP_INSTANCE: Callee = Callee {
        addr: 0x002b_11b0,
        f: ee::s08::f_002b11b0,
    };

    pub const BANK_MONITOR_TIMER_KILL: Callee = Callee {
        addr: 0x002a_d940,
        f: ee::s08::f_002ad940,
    };

    pub const SNDSTRM_SET_PREBUFFER: Callee = Callee {
        addr: 0x003b_7d80,
        f: ee::s13::f_003b7d80,
    };

    pub const AUDIO_RAND: Callee = Callee {
        addr: 0x002a_df60,
        f: ee::s08::f_002adf60,
    };

    pub const SONG_INSTANCE_GET_REQUEST_STATUS: Callee = Callee {
        addr: 0x002b_25c8,
        f: ee::s08::f_002b25c8,
    };

    pub const USTR_TO_ASCII: Callee = Callee {
        addr: 0x002c_2580,
        f: ee::s08::f_002c2580,
    };

    pub const MENU_SYS_ITEM_DRAW_ROW: Callee = Callee {
        addr: 0x002c_a988,
        f: ee::s09::f_002ca988,
    };

    pub const WSCRIPT_MAN_CALL_TASK_FUNC: Callee = Callee {
        addr: 0x0030_ada8,
        f: ee::s10::f_0030ada8,
    };

    pub const WSCRIPT_PROCESS_LIST_REMOVE: Callee = Callee {
        addr: 0x0030_b2d0,
        f: ee::s10::f_0030b2d0,
    };

    pub const WSCRIPT_PROCESS_LIST_PUSH_FRONT: Callee = Callee {
        addr: 0x0030_b260,
        f: ee::s10::f_0030b260,
    };

    pub const BE_LIBRARY_GET_INSTANCE_3: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::f_0014dc80,
    };

    pub const BE_MISSION_INT_GET_COLLECTED_COUNT: Callee = Callee {
        addr: 0x0015_3498,
        f: ee::s01::f_00153498,
    };

    pub const ADF_DATA_SOURCE_SET_DATA: Callee = Callee {
        addr: 0x0031_1290,
        f: ee::s10::f_00311290,
    };

    pub const EA_FILE_LOAD_DECOMPRESSED: Callee = Callee {
        addr: 0x003e_22f0,
        f: ee::s14::f_003e22f0,
    };

    pub const GAME_ANIM_MAP_FIND_HASH: Callee = Callee {
        addr: 0x0031_4e88,
        f: ee::s10::f_00314e88,
    };

    pub const MEMSET_2: Callee = Callee {
        addr: 0x0041_6210,
        f: ee::s15::f_00416210,
    };

    pub const CROWD_MAN3D_GROUP_CREATE_MESHES: Callee = Callee {
        addr: 0x0034_43a8,
        f: ee::s11::f_003443a8,
    };

    pub const TEX_MAN_FREE_TEXTURE: Callee = Callee {
        addr: 0x0036_7bc0,
        f: ee::s12::f_00367bc0,
    };

    pub const UI_LIST_COUNT: Callee = Callee {
        addr: 0x0039_79c0,
        f: ee::s13::f_003979c0,
    };

    pub const UI_MENU_SET_NAV_BUTTONS: Callee = Callee {
        addr: 0x0039_ab00,
        f: ee::s13::f_0039ab00,
    };

    pub const SECTION_ALLOCATOR_DTOR: Callee = Callee {
        addr: 0x003a_77d0,
        f: ee::s13::f_003a77d0,
    };

    pub const STREAM_FILE_RELEASE_CHUNK: Callee = Callee {
        addr: 0x003e_13e8,
        f: ee::s14::f_003e13e8,
    };

    pub const BUILTIN_DELETE_4: Callee = Callee {
        addr: 0x0031_7e50,
        f: ee::s10::f_00317e50,
    };

    pub const RIDER_METRIX_SHOW_HUD_NUMBER: Callee = Callee {
        addr: 0x0011_7b88,
        f: ee::s00::f_00117b88,
    };

    pub const RIDER_METRIX_CLEAR_PEAK_A4: Callee = Callee {
        addr: 0x0011_76f8,
        f: ee::s00::f_001176f8,
    };

    pub const BE_GET_EVENT_RECORD_INDEX: Callee = Callee {
        addr: 0x0014_ab68,
        f: ee::s01::f_0014ab68,
    };

    pub const BOLTS_FREE_DB: Callee = Callee {
        addr: 0x0014_d240,
        f: ee::s01::f_0014d240,
    };
}

/// The monitor slots `PATH_PlatRefreshStatus` walks (`D_004D3E98`, 24
/// pointers) and the fields of one slot (`sMonSlot`) and its sub-streams.
pub(crate) mod monitor {

    pub const TABLE: u32 = 0x004d_3e98;

    pub const SLOTS: u64 = 24;

    pub const H: u32 = 0x0;

    pub const SUB: u32 = 0x4;

    pub const STATUS: u32 = 0xc;

    pub const F48: u32 = 0x48;

    pub const F4C: u32 = 0x4c;

    pub const F50: u32 = 0x50;

    pub const F54: u32 = 0x54;

    pub const BUSY: u32 = 0x58;

    pub const F5C: u32 = 0x5c;

    pub const F60: u32 = 0x60;

    pub const IS_MEM: u32 = 0x64;

    pub const DIRTY: u32 = 0x65;

    pub const COUNT: u32 = 0x66;

    pub const SUBS: u32 = 0x6c;

    /// `sMonSub`: size, id, pos.
    pub const SUB_SIZE: u32 = 0x0;

    pub const SUB_ID: u32 = 0x4;

    pub const SUB_POS: u32 = 0x8;

    pub const SUB_STRIDE: u32 = 0xc;

    /// `sStrmState` on the stack: state, a, b, c.
    pub const ST_STATE: u32 = 0x10;

    pub const ST_A: u32 = 0x14;

    pub const ST_B: u32 = 0x18;

    pub const ST_C: u32 = 0x1c;

    /// `sStrmHdr.rate` and `sStrmPos.samples` within the stack block.
    pub const HDR_RATE: u32 = 0x20;

    pub const POS_SAMPLES: u32 = 0xa0;
}

/// Cross-shard calls the generator emitted as `call_indirect(c, addr)`: they
/// resolve through the function table like the original's `jal`.
pub(crate) mod table_callee {
    pub(crate) use crate::game::be::bedbreward::table_callee::EA_MEMSET;
    pub(crate) use crate::game::be::bedbreward::table_callee::OPERATOR_DELETE;
    pub(crate) use crate::game::be::bedbreward::table_callee::WORLD_CREATE_DEAD_NODE;
}

/// `gp`-relative globals used here.
pub(crate) mod g {
    pub(crate) use crate::game::fe::festateload::g::BE_OPTION_SINGLETON;
    pub(crate) use crate::game::fe::festateload::g::BOLT_FILE_CACHE;
    pub(crate) use crate::game::fe::festateload::g::FE_BASE_BACK;
    pub(crate) use crate::game::fe::festateload::g::SAVE_BUFFER;
    pub(crate) use crate::game::fe::festateload::g::SAVE_BUFFER_END;
    pub(crate) use crate::game::fe::festateload::g::WORLD_TRIGGER_MANAGER;
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod table {
    pub(crate) use crate::game::fe::feasyncfile::table::ASYNC_FILE_GET_RESULT;
    pub(crate) use crate::game::fe::feasyncfile::table::AUDIO_GET_RACE_MAN;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_NEW_RACE_SET_EVENT_TYPE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_NEW_RACE_SET_GAME_MODE;
    pub(crate) use crate::game::fe::feasyncfile::table::BXSTRING_ASSIGN;
    pub(crate) use crate::game::fe::feasyncfile::table::BXSTRING_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_HULL_TRIANGLE;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_HULL_TRIANGLE2;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_SPHERE_CALC;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_SPHERE_TREE_TRIANGLE;
    pub(crate) use crate::game::fe::feasyncfile::table::EAC_GET_UNPACKED_SIZE;
    pub(crate) use crate::game::fe::feasyncfile::table::EAC_UNPACK;
    pub(crate) use crate::game::fe::feasyncfile::table::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::table::EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::table::OPERATOR_NEW_TAG;
    pub(crate) use crate::game::fe::feasyncfile::table::PATH_STOP_TRACKS;
    pub(crate) use crate::game::fe::feasyncfile::table::PS2_MEM_CARD_CREATE;
    pub(crate) use crate::game::fe::feasyncfile::table::REPLAY_BEGIN_PLAYBACK;
    pub(crate) use crate::game::fe::feasyncfile::table::REPLAY_END_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::table::RIDER_MODEL_FIND;
    pub(crate) use crate::game::fe::feasyncfile::table::SCRIPT_ENGINE_GET_RESULT_VTABLE;
    pub(crate) use crate::game::fe::feasyncfile::table::SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::table::STR_CASECMP;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_MENU_RESET_SCROLL;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_TEXT_SET_WIDE_STRING;
    pub(crate) use crate::game::fe::feasyncfile::table::VOIP_RPC_SET_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::table::VOIP_RPC_START;
    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_DELETE;
    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_VEC_DELETE;
    pub(crate) use crate::game::wscript::wscriptman::table::BXFILE_EXISTS;
    pub(crate) use crate::game::wscript::wscriptman::table::GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::wscript::wscriptman::table::IS_REPLAY_PLAYING;
    pub(crate) use crate::game::wscript::wscriptman::table::MEM_FREE;
    pub(crate) use crate::game::wscript::wscriptman::table::SCE_MC_FLUSH;
    pub(crate) use crate::game::wscript::wscriptman::table::SERIALIZE_READ_U32;
    pub(crate) use crate::game::wscript::wscriptman::table::SERIALIZE_READ_U32_2;
    pub(crate) use crate::game::wscript::wscriptman::table::SND_BANK_REMOVE;
    pub(crate) use crate::game::wscript::wscriptman::table::SND_MIDI_RESUME;
    pub(crate) use crate::game::wscript::wscriptman::table::STREAM_FILE_DESTROY;
    pub(crate) use crate::game::wscript::wscriptman::table::STR_HASH;
    pub(crate) use crate::game::wscript::wscriptman::table::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::wscript::wscriptman::table::WORLD_GET_OBJECT_SCRIPT_FUNC;
    pub(crate) use crate::game::wscript::wscriptman::table::WSCRIPT_PROCESS_LIST_POP_FRONT;
}

/// Addresses the generator emitted as `call_indirect` (outside the shard's
/// own direct-call set, so no readable alias is needed here).
pub(crate) mod addr {
    pub(crate) use crate::game::bx::bxstring2::addr::BANK_MONITOR_TIMER_RESET;
    pub(crate) use crate::game::bx::bxstring2::addr::BE_INTERFACE_LOOKUP;
    pub(crate) use crate::game::bx::bxstring2::addr::CBE_GET_BE;
    pub(crate) use crate::game::bx::bxstring2::addr::CBE_GET_INTERFACE;
    pub(crate) use crate::game::bx::bxstring2::addr::FMODF;
    pub(crate) use crate::game::bx::bxstring2::addr::MC_MANAGER_GET_INFO;
    pub(crate) use crate::game::bx::bxstring2::addr::MEM_MAN_ALLOC;
    pub(crate) use crate::game::bx::bxstring2::addr::MEM_MAN_FREE;
    pub(crate) use crate::game::bx::bxstring2::addr::MEM_MAN_FREE_NOSIZE;
    pub(crate) use crate::game::bx::bxstring2::addr::SPEECH_BANK_FREE_CALLBACK;
    pub(crate) use crate::game::bx::bxstring2::addr::SPRINTF;
    pub(crate) use crate::game::bx::bxstring2::addr::UI_ENGINE_TEARDOWN;
    pub(crate) use crate::game::fe::feasyncfile::addr::__BUILTIN_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::addr::__FIXUNSSFSI;
    pub(crate) use crate::game::fe::feasyncfile::addr::C_BX_STRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::addr::EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::addr::FONT_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BANK_MONITOR_ADD;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_BE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_INTERFACE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_INTERFACE_9;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::CHASE_CAMERA_RESET_FOR_RIDER;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::CLOSE_UNPINNED_POPUPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::FE_NET_SEND;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::HUD_SHOW_NUMBER_DELAYED;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MC_STATUS_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MEMSET;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_DTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_POP_PAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_PUSH_PAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::POSITION_MODIFIER_CTOR_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::RIDER_PREVIEW;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::RIDER_PREVIEW_BOLTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::SPATIAL_INSERT;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCASECMP;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCMP;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRLEN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STR_HASH;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_ENGINE_CREATE_SCREEN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_OBJECT_BY_HASH;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_SCREEN_START_THREAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::WORLD_GET;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::WORLD_PAINTER_2D;
}

// ---------------------------------------------------------------------------
// Bank monitor

/// `void BankMonitor_TimerSetPeriod(BankMonitorTimer* t, int period)`
/// (0x2ad970): store the period at +4 and +8 of the timer.
pub fn bank_monitor_timer_set_period(c: &mut Cpu) {
    c.cycles += 3;
    let timer = Addr(abi::arg(c, 0));
    let period = c.g[A1][0];
    timer.off(0x8).sw(c, period);
    timer.off(0x4).sw(c, period);
    abi::ret(c);
}

/// `void BankMonitor_Tick(BankMonitor* m)` (0x2adc10): if the timer at +0x94
/// is inactive, advance the counter at +0x88.
pub fn bank_monitor_tick(c: &mut Cpu) {
    c.cycles += 3;
    let bank = Addr(abi::arg(c, 0));
    let active = bank.off(0x94).lw(c);
    c.g[V0][0] = active;
    if active == 0 {
        c.cycles += 3;
        let count = (bank.off(0x88).lw(c) as u32).wrapping_add(1);
        c.g[V0][0] = sx(count);
        bank.off(0x88).sw(c, sx(count));
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Path / track table

/// The base of the track-data pointer table.
pub(crate) const TRACK_TABLE: u32 = 0x004d_3e98;

/// `TrackData* PATH_GetTrackData(int index)` (0x2adfb0): fetch the pointer
/// from `TRACK_TABLE[index]`.
pub fn path_get_track_data(c: &mut Cpu) {
    c.cycles += 6;
    let index = abi::arg(c, 0) as u32;
    let slot = TRACK_TABLE.wrapping_add(index.wrapping_shl(2));
    c.g[A0][0] = sx(slot);
    c.g[V0][0] = Addr(slot).lw(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Music player

/// `void MusicPlayer_SetSongVolume(MusicPlayer* mp, s8 volume)` (0x2b3c98):
/// store the byte volume into the current song instance at +0x40.
pub fn music_player_set_song_volume(c: &mut Cpu) {
    c.cycles += 4;
    let mp = Addr(abi::arg(c, 0));
    let volume = sx(abi::arg(c, 1) as u8 as i8 as u32);
    let song = mp.off(0x418).lw(c);
    c.g[A1][0] = volume;
    c.g[V0][0] = song;
    if song != 0 {
        c.cycles += 3;
        let instance = mp.off(0x408).lw(c);
        c.g[A0][0] = instance;
        if instance != 0 {
            Addr(instance as u32).off(0x40).sb(c, volume);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// The word holding the currently selected song index.
pub(crate) const CUR_SONG: u32 = 0x3f0;

/// Shared body of the `MusicPlayer_GetSong*` accessors: resolve the song
/// index (negative argument means "current song", a negative current song
/// returns null) and return `this + table + index*100`.
pub(crate) fn music_player_get_song_part(c: &mut Cpu, table: u32) {
    c.cycles += 2;
    let arg = abi::arg64(c, 1);
    c.g[V0][0] = 0x64;
    if (arg as i64) >= 0 {
        c.cycles += 1;
        c.mult(0, V0, A1, V0);
    } else {
        c.cycles += 3;
        let current = Addr(abi::arg(c, 0)).off(CUR_SONG).lw(c);
        c.g[A1][0] = current;
        c.mult(0, V0, A1, V0);
        if (current as i64) < 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
    }
    c.cycles += 3;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(table));
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    abi::ret(c);
}

/// `char* MusicPlayer_GetSongTitle(MusicPlayer* mp, int song)` (0x2b40f0).
pub fn music_player_get_song_title(c: &mut Cpu) {
    music_player_get_song_part(c, 0x934);
}

/// `char* MusicPlayer_GetSongArtist(MusicPlayer* mp, int song)` (0x2b4120).
pub fn music_player_get_song_artist(c: &mut Cpu) {
    music_player_get_song_part(c, 0x2234);
}

/// `char* MusicPlayer_GetSongAlbum(MusicPlayer* mp, int song)` (0x2b4150).
pub fn music_player_get_song_album(c: &mut Cpu) {
    music_player_get_song_part(c, 0x3b34);
}

// ---------------------------------------------------------------------------
// Song / world trigger instances

/// `int SongInstance_GetDuckToLoops(SongInstance* s)` (0x2b4868): read the
/// loop count at +0x94 of the owning song.
pub fn song_instance_get_duck_to_loops(c: &mut Cpu) {
    c.cycles += 3;
    let owner = Addr(abi::arg(c, 0)).lw(c);
    c.g[V1][0] = owner;
    c.g[V0][0] = Addr(owner as u32).off(0x94).lw(c);
    abi::ret(c);
}

/// `void BankMonitor_TimerInit(BankMonitor_Timer *self)` (0x2ad848): clear the
/// active flag and set the period sentinel `-1`. Returns `self` in v0.
pub fn bank_monitor_timer_init(c: &mut Cpu) {
    c.cycles += 5;
    let this = Addr(abi::arg(c, 0));
    this.sw(c, 0);
    this.off(0x4).sw(c, u64::MAX);
    c.g[V0][0] = c.g[A0][0];
    abi::ret(c);
}

/// `bool BankMonitor_TimerIsFree(BankMonitor_Timer *self)` (0x2ad930): true
/// when the period field at +0x4 is the `-1` sentinel.
pub fn bank_monitor_timer_is_free(c: &mut Cpu) {
    c.cycles += 4;
    let v = !(0u64 | Addr(abi::arg(c, 0)).off(0x4).lw(c));
    c.g[V0][0] = (v < 1) as u64;
    abi::ret(c);
}

/// `void BankMonitor_TimerEnable(BankMonitorTimer *t)` (0x2ad980): set the
/// timer's enable flag.
pub fn bank_monitor_timer_enable(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x1;
    Addr(abi::arg(c, 0)).sw(c, 0x1);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Path platform table.

/// The base of the path platform pointer table.
pub(crate) const PLATFORM_TABLE: u32 = 0x004d_3e98;

/// `bool PATH_PlatMarkTrackDirty(int index)` (0x2aec28): set the dirty byte of
/// the platform at `index`, or report an empty slot (-8).
pub fn path_plat_mark_track_dirty(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = (PLATFORM_TABLE & 0xffff_0000) as u64;
    c.g[A0][0] = sx(abi::arg(c, 0) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(PLATFORM_TABLE & 0xffff));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.g[V1][0] = 0x1;
    if c.g[A0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0xffff_ffff_ffff_fff8;
        abi::ret(c);
    } else {
        c.cycles += 3;
        c.g[V0][0] = 0x1;
        Addr((c.g[A0][0] as u32).wrapping_add(0x65)).sb(c, 0x1);
        abi::ret(c);
    }
}

/// `int PATH_PlatGetTrackAuxSend(int index)` (0x2af4b0): the aux-send word of
/// the platform at `index`, or -8 for an empty slot.
pub fn path_plat_get_track_aux_send(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = (PLATFORM_TABLE & 0xffff_0000) as u64;
    c.g[A0][0] = sx(abi::arg(c, 0) << 2);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(PLATFORM_TABLE & 0xffff));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.g[V0][0] = 0xffff_ffff_ffff_fff8;
    if c.g[A0][0] == 0 {
        c.cycles += 2;
        abi::ret(c);
    } else {
        c.cycles += 2;
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x10)).lw(c);
        abi::ret(c);
    }
}

// ---------------------------------------------------------------------------
// Speech / music / world trigger.

/// `SpeechBank* SpeechBank_ctor(SpeechBank *self)` (0x2af8e0): clear the bank
/// and return `self`.
pub fn speech_bank_ctor(c: &mut Cpu) {
    c.cycles += 7;
    let a0 = abi::arg64(c, 0);
    let this = Addr(a0 as u32);
    c.g[V0][0] = u64::MAX;
    this.off(0x48).sw(c, 0);
    this.sw(c, u64::MAX);
    c.g[V0][0] = a0;
    this.off(0x44).sw(c, 0);
    this.off(0x4).sb(c, 0);
    abi::ret(c);
}

/// `void* SpeechSystem_GetInstanceStreamSlot(SpeechSystem *sys, int slot)`
/// (0x2b1220): the stream at `sys->slots[slot] + 4`, or -1.
pub fn speech_system_get_instance_stream_slot(c: &mut Cpu) {
    c.cycles += 6;
    let sys = Addr(abi::arg(c, 0));
    c.g[V0][0] = sys.off(0x4).lw(c);
    c.g[A1][0] = sx(abi::arg(c, 1) << 2);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A1][0] = Addr(c.g[A1][0] as u32).lw(c);
    c.g[V0][0] = u64::MAX;
    if c.g[A1][0] == 0 {
        c.cycles += 2;
        abi::ret(c);
    } else {
        c.cycles += 2;
        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x4)).lw(c);
        abi::ret(c);
    }
}

/// The speech system's event contexts: 10 slots of 0x20 bytes from
/// `sys + 0x18`, each remembering who/where a speech event came from for a
/// few seconds so a line that starts later can still be attributed.
pub(crate) mod speech_ctx {
    /// The first slot.
    pub const SLOTS: u32 = 0x18;
    pub const SLOT_SIZE: u32 = 0x20;
    pub const COUNT: u32 = 10;
    /// Slot fields: in use, ticks left, the two keys, then the context
    /// (two words, a float, a word).
    pub const USED: u32 = 0x0;
    pub const LIFE: u32 = 0x4;
    pub const KEY_A: u32 = 0x8;
    pub const KEY_B: u32 = 0xc;
    pub const ARG_A3: u32 = 0x10;
    pub const ARG_T0: u32 = 0x14;
    pub const ARG_F12: u32 = 0x18;
    pub const ARG_T1: u32 = 0x1c;
    /// A context's lifetime in 60 Hz frames (3 s).
    pub const LIFETIME_FRAMES: u32 = 0xb4;
}

/// `bool SpeechSystem_SetEventContext(SpeechSystem *sys, int keyA, int keyB,
/// int a3, int t0, int t1, float f12, bool refresh)` (0x2b1458): look for a
/// live context with both keys; if there is one, renew its lifetime when
/// `refresh` (t2) is set and return `refresh`; otherwise fill the first free
/// slot (lifetime 3 s, the keys and the context) and return 1, or 0 when all
/// ten are in use.
///
/// Tick rate: the lifetime counts ticks (`SpeechSystem_TickEventContexts`),
/// so the 180 frames are `tick::ticks_from_frames60(180)` (exact at 60).
pub fn speech_system_set_event_context(c: &mut Cpu) {
    use speech_ctx::*;
    c.cycles += 8;
    let sys = c.g[A0][0];
    c.g[T3][0] = sys;
    c.g[T4][0] = c.g[A1][0];
    c.g[T5][0] = c.g[A2][0];
    let (key_a, key_b) = (c.g[T4][0], c.g[T5][0]);
    c.g[V1][0] = sx((sys as u32).wrapping_add(SLOTS));
    // a0: the first free slot (-1: none yet); a2: the slot index; a1: the
    // slot's lifetime field.
    c.g[A0][0] = u64::MAX;
    c.g[A2][0] = 0;
    let life = crate::tick::ticks_from_frames60(c, LIFETIME_FRAMES);
    c.g[T6][0] = life as u64;
    c.g[A1][0] = sx((sys as u32).wrapping_add(SLOTS + LIFE));
    loop {
        c.cycles += 3;
        let at = c.g[A1][0] as u32;
        c.g[V0][0] = c.lw(at.wrapping_sub(LIFE));
        let free = c.g[V0][0] == 0;
        c.g[V0][0] = !c.g[A0][0];
        if free {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.g[A0][0] = c.g[A2][0];
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw(at.wrapping_add(KEY_A - LIFE));
            if c.g[V0][0] == key_a {
                c.cycles += 3;
                c.g[V0][0] = c.lw(at.wrapping_add(KEY_B - LIFE));
                if c.g[V0][0] == key_b {
                    // The same event: renew it (or just report it).
                    c.cycles += 4;
                    if c.g[T2][0] == 0 {
                        c.g[V0][0] = 0;
                    } else {
                        c.g[V0][0] = 1;
                        c.sw(at, life as u64);
                    }
                    abi::ret(c);
                    return;
                }
            }
        }
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
        c.cycles += 3;
        let more = (c.g[A2][0] as i64) < COUNT as i64;
        c.g[V0][0] = more as u64;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(SLOT_SIZE));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x002b_1478);
        }
    }
    c.cycles += 3;
    c.g[V0][0] = u64::MAX;
    let none = c.g[A0][0] == u64::MAX;
    let off = (c.g[A0][0] as u32) << 5;
    c.g[A0][0] = sx(off);
    if none {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    // Fill the free slot.
    c.cycles += 17;
    let slot = (c.g[T3][0] as u32).wrapping_add(off);
    c.g[A1][0] = 1;
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(off));
    c.sw(c.g[V1][0] as u32, 1);
    c.sw(slot + SLOTS + LIFE, life as u64);
    c.sw(slot + SLOTS + KEY_A, c.g[T4][0]);
    c.sw(slot + SLOTS + KEY_B, c.g[T5][0]);
    c.sw(slot + SLOTS + ARG_A3, c.g[A3][0]);
    c.sw(slot + SLOTS + ARG_T0, c.g[T0][0]);
    c.swc1(12, slot + SLOTS + ARG_F12);
    c.sw(slot + SLOTS + ARG_T1, c.g[T1][0]);
    c.g[A2][0] = sx(slot);
    c.g[V1][0] = sx(slot);
    c.g[A0][0] = sx(slot);
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `void SpeechSystem_TickEventContexts(SpeechSystem *sys)` (0x2b1720): count
/// down the lifetime of each live event context, freeing it at zero.
///
/// Tick rate: counts ticks; `SpeechSystem_SetEventContext` converts the
/// lifetime.
pub fn speech_system_tick_event_contexts(c: &mut Cpu) {
    use speech_ctx::*;
    c.cycles += 2;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(SLOTS + LIFE));
    c.g[V1][0] = (COUNT - 1) as u64;
    loop {
        c.cycles += 3;
        let at = c.g[A0][0] as u32;
        c.g[V0][0] = c.lw(at.wrapping_sub(LIFE));
        let used = c.g[V0][0] != 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        if used {
            c.cycles += 4;
            c.g[V0][0] = sx((c.lw(at) as u32).wrapping_sub(1));
            c.sw(at, c.g[V0][0]);
            if (c.g[V0][0] as i64) <= 0 {
                c.cycles += 1;
                c.sw(at.wrapping_sub(LIFE), 0);
            }
        }
        c.cycles += 2;
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[A0][0] = sx(at.wrapping_add(SLOT_SIZE));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x002b_1728);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void SongInstance_ApplyDelayParams(SongInstance *player, SongInstance *a, int b)`
/// (0x2b2850): copy three delay-parameter fields of `a`/`b` into the
/// player's DSP delay table.
pub fn song_instance_apply_delay_params(c: &mut Cpu) {
    c.cycles += 28;
    let p = abi::arg(c, 0);
    let q = abi::arg(c, 1);
    // First delay row: +0x4.
    let v0 = Addr(q).lw(c);
    let v1 = Addr(p).lw(c);
    let a2 = Addr(v0 as u32).lw(c);
    let a3 = Addr((v1 as u32).wrapping_add(0x78)).lw(c);
    let v0 = Addr((a2 as u32).wrapping_add(0x1f4)).lw(c);
    let v1 = Addr((a2 as u32).wrapping_add(0x248)).lw(c);
    let v0 = sx((v0 as u32) << 4);
    let v0 = sx((v0 as u32).wrapping_add(v1 as u32));
    Addr((v0 as u32).wrapping_add(0x4)).sw(c, a3);
    // Second delay row: +0x8.
    let v1 = Addr(q).lw(c);
    let a2 = Addr(p).lw(c);
    let a3 = Addr(v1 as u32).lw(c);
    let t0 = Addr((a2 as u32).wrapping_add(0x7c)).lw(c);
    let v0 = Addr((a3 as u32).wrapping_add(0x1f4)).lw(c);
    let v1 = Addr((a3 as u32).wrapping_add(0x248)).lw(c);
    let v0 = sx((v0 as u32) << 4);
    let v0 = sx((v0 as u32).wrapping_add(v1 as u32));
    Addr((v0 as u32).wrapping_add(0x8)).sw(c, t0);
    // Third delay row: +0x0.
    let v1 = Addr(q).lw(c);
    let a1 = Addr(p).lw(c);
    let a0 = Addr(v1 as u32).lw(c);
    let a2 = Addr((a1 as u32).wrapping_add(0x80)).lw(c);
    let v0 = Addr((a0 as u32).wrapping_add(0x1f4)).lw(c);
    let v1 = Addr((a0 as u32).wrapping_add(0x248)).lw(c);
    let v0 = sx((v0 as u32) << 4);
    let v0 = sx((v0 as u32).wrapping_add(v1 as u32));
    Addr(v0 as u32).sw(c, a2);
    c.g[V0][0] = v0;
    c.g[V1][0] = v1;
    c.g[A0][0] = a0;
    c.g[A1][0] = a1;
    c.g[A2][0] = a2;
    c.g[A3][0] = a3;
    c.g[T0][0] = t0;
    abi::ret(c);
}

/// `int SongInstance_GetSedValue(SongInstance *s)` (0x2b48b8): read the sed
/// value at +0x98 of the owning song.
pub fn song_instance_get_sed_value(c: &mut Cpu) {
    c.cycles += 3;
    let owner = Addr(abi::arg(c, 0)).lw(c);
    c.g[V1][0] = owner;
    c.g[V0][0] = Addr((owner as u32).wrapping_add(0x98)).lw(c);
    abi::ret(c);
}

/// `int MusicPlayer_GetSelectedSongSedValue(MusicPlayer *mp)` (0x2b4908):
/// the sed value of the currently selected song.
pub fn music_player_get_selected_song_sed_value(c: &mut Cpu) {
    c.cycles += 6;
    let mp = Addr(abi::arg(c, 0));
    c.g[V0][0] = mp.off(0x3f0).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[A0][0] = sx(mp.0.wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x4).lw(c);
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x98)).lw(c);
    abi::ret(c);
}

/// `bool MusicPlayer_IsCurrentSongEnabled(MusicPlayer *mp)` (0x2b4af0):
/// whether the bit for the current song is set in the 64-bit enable mask at
/// +0x3f8.
pub fn music_player_is_current_song_enabled(c: &mut Cpu) {
    c.cycles += 5;
    let mp = Addr(abi::arg(c, 0));
    c.g[V1][0] = mp.off(0x3f0).lw(c);
    c.g[V0][0] = mp.off(0x3f8).ld(c);
    c.g[V0][0] = c.g[V0][0] >> (c.g[V1][0] & 63);
    c.g[V0][0] = c.g[V0][0] & 0x1;
    abi::ret(c);
}

/// `void BankMonitor_SetParamCallback(BankMonitor* m, void* cb, ...)`
/// (0x2b9ce0): store `cb` (a1) unaligned into the bank's parameter slot at
/// `m + (*(m+0x1c) << 3) + 0x40`.
pub fn bank_monitor_set_param_callback(c: &mut Cpu) {
    c.cycles += 11;
    let sp = (c.g[SP][0] as u32).wrapping_sub(0x10);
    c.g[SP][0] = sx(sp);
    let cb = c.g[A1][0];
    Addr(sp).sd(c, cb);
    c.g[V0][0] = Addr(abi::arg(c, 0)).off(0x1c).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 3);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    // ldl/ldr reload the callback value out of the frame.
    c.g[V0][0] = cb;
    Addr(c.g[A0][0] as u32).off(0x40).write_u64_unaligned(c, cb);
    c.g[SP][0] = sx(sp.wrapping_add(0x10));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// PATH_PlatRefreshStatus.

/// `void PATH_PlatRefreshStatus(int reset)` (0x2aefa8): the per-frame refresh
/// of the 24 sound-bank monitor slots: a `reset` marks every slot for
/// restart; otherwise each slot with a stream (`h >= 0`) updates its
/// in-memory or streaming state and its `f48`/`f4C`/`f60`/`dirty` fields.
pub fn path_plat_refresh_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x170);
        let frame_sp = sp(c);
        frame.save(c, S1, 0x150);
        frame.save(c, S7, 0xf0);
        c.g[S1][0] = 0;
        frame.save(c, FP, 0xe0);
        c.g[S7][0] = u64::MAX; // -1
        frame.save(c, S0, 0x160);
        c.g[FP][0] = 1;
        frame.save(c, S2, 0x140);
        frame.save(c, S3, 0x130);
        frame.save(c, S4, 0x120);
        frame.save(c, S5, 0x110);
        frame.save(c, S6, 0x100);
        frame.save_ra(c, 0xd0);
        c.g[RA][0] = sx(0x002a_efe8);
        c.sw(frame_sp.wrapping_add(0xc0), c.g[A0][0]); // reset
        call_ptr(c, callee::SND_LOCK, 0x002a_efe8)?;
        c.g[V0][0] = 0x4d_0000;
        loop {
            // m = D_004D3E98[i]
            c.cycles += 6;
            c.g[V1][0] = sx(r32(c, V0).wrapping_add(0x3e98));
            c.g[V0][0] = sx(r32(c, S1) << 2);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
            c.g[S0][0] = c.lw(r32(c, V0));
            let null = c.g[S0][0] == 0;
            c.g[S6][0] = sx(r32(c, S1).wrapping_add(1));
            if !null {
                c.cycles += 3;
                c.g[V1][0] = c.lw(frame_sp.wrapping_add(0xc0));
                let restart = c.g[V1][0] != 0;
                c.g[V0][0] = 1;
                if restart {
                    c.cycles += 3;
                    let m = r32(c, S0);
                    c.sw(m.wrapping_add(monitor::F60), 0);
                    c.sb(m.wrapping_add(monitor::DIRTY), c.g[V0][0]);
                } else {
                    let m = r32(c, S0);
                    // The negative-sub path stores `f48` itself and leaves by
                    // `b_2af29c`, skipping the shared tail's store and cycle.
                    let mut stored48 = false;
                    'body: {
                        // The slot's working values: f60, f48, f4C.
                        c.cycles += 6;
                        c.g[A0][0] = c.lw(m.wrapping_add(monitor::H));
                        c.g[S5][0] = sx(0xffff_fff8); // -8
                        c.g[S4][0] = 0;
                        c.g[S2][0] = 0;
                        let no_stream = (c.g[A0][0] as i64) < 0;
                        c.sw(m.wrapping_add(monitor::BUSY), c.g[FP][0]);
                        if no_stream {
                            break 'body;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = Addr(m.wrapping_add(monitor::IS_MEM)).lb(c);
                        let in_memory = c.g[V0][0] != 0;
                        if !in_memory {
                            // Streaming state machine: stream info, then each
                            // sub-stream's state, then the slot status.
                            c.cycles += 9;
                            c.g[RA][0] = sx(0x002a_f134);
                            c.g[A1][0] = c.g[SP][0];
                            call_ptr(c, callee::SND_STREAM_INFO, 0x002a_f134)?;
                            c.sw(m.wrapping_add(monitor::F50), 0);
                            c.g[S5][0] = c.g[V0][0]; // f60
                            c.g[V1][0] = Addr(m.wrapping_add(monitor::COUNT)).lb(c);
                            c.g[V0][0] = c.lw(frame_sp);
                            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            if c.g[V0][0] == 0 {
                                c.sw(m.wrapping_add(monitor::BUSY), 0);
                            }
                            c.cycles += 3;
                            c.g[V0][0] = Addr(m.wrapping_add(monitor::COUNT)).lb(c);
                            let none = (c.g[V0][0] as i64) <= 0;
                            c.g[S3][0] = 0; // j
                            if !none {
                                c.cycles += 1;
                                c.g[S1][0] = 0; // j * 0xc
                                loop {
                                    'sub: {
                                        c.cycles += 8;
                                        c.g[V0][0] = c.lw(m.wrapping_add(monitor::SUBS));
                                        c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
                                        c.sw(r32(c, V0).wrapping_add(monitor::SUB_POS), 0);
                                        c.g[V1][0] = c.lw(m.wrapping_add(monitor::SUBS));
                                        c.g[V1][0] = sx(r32(c, S1).wrapping_add(r32(c, V1)));
                                        c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(monitor::SUB_ID));
                                        if (c.g[A0][0] as i64) < 0 {
                                            c.g[V0][0] = Addr(m.wrapping_add(monitor::COUNT)).lb(c);
                                            break 'sub;
                                        }
                                        c.cycles += 5;
                                        c.g[RA][0] = sx(0x002a_f188);
                                        c.g[A1][0] = sx(frame_sp.wrapping_add(monitor::ST_STATE));
                                        call_ptr(c, callee::SND_SUB_STATE, 0x002a_f188)?;
                                        let state = c.lw(frame_sp.wrapping_add(monitor::ST_STATE));
                                        c.g[V1][0] = state;
                                        // `switch (state)`: 1 plays the
                                        // sub-stream to its end, 0 and 2
                                        // advance it by st.a/st.b, anything
                                        // else drops it.
                                        let mut store = false;
                                        if state == 1 {
                                            c.cycles += 18;
                                            c.g[V0][0] = c.lw(m.wrapping_add(monitor::SUBS));
                                            c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
                                            c.g[V1][0] = c.lw(r32(c, V0));
                                            c.sw(
                                                r32(c, V0).wrapping_add(monitor::SUB_POS),
                                                c.g[V1][0],
                                            );
                                            c.g[A1][0] = c.lw(m.wrapping_add(monitor::SUBS));
                                            c.g[V1][0] = c.lw(m.wrapping_add(monitor::F54));
                                            c.g[A1][0] = sx(r32(c, S1).wrapping_add(r32(c, A1)));
                                            c.g[A0][0] = c.lw(m.wrapping_add(monitor::F50));
                                            c.g[V0][0] = c.lw(r32(c, A1));
                                            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                                            c.sw(m.wrapping_add(monitor::F54), c.g[V1][0]);
                                            c.g[V0][0] = c.lw(frame_sp.wrapping_add(monitor::ST_C));
                                            c.sw(m.wrapping_add(monitor::BUSY), 0);
                                            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
                                            c.sw(m.wrapping_add(monitor::F50), c.g[A0][0]);
                                            c.g[V0][0] =
                                                c.lw(r32(c, A1).wrapping_add(monitor::SUB_ID));
                                            c.sw(m.wrapping_add(monitor::F5C), c.g[V0][0]);
                                        } else {
                                            c.cycles += 2;
                                            if state == 0 || state == 2 {
                                                c.cycles += 2;
                                                c.g[S4][0] =
                                                    c.lw(frame_sp.wrapping_add(monitor::ST_A));
                                                store = true;
                                            } else {
                                                // b_2af1ac's exit, the else,
                                                // then the default block.
                                                c.cycles += 2;
                                                c.cycles += 2;
                                                c.g[V0][0] = c.lw(m.wrapping_add(monitor::SUBS));
                                                c.cycles += 2;
                                                c.g[V0][0] =
                                                    sx(r32(c, S1).wrapping_add(r32(c, V0)));
                                                c.sw(
                                                    r32(c, V0).wrapping_add(monitor::SUB_ID),
                                                    c.g[S7][0],
                                                );
                                            }
                                        }
                                        if store {
                                            // st.b, falling back to the
                                            // sub-stream's size when both
                                            // are zero.
                                            c.cycles += 2;
                                            c.g[S2][0] = c.lw(frame_sp.wrapping_add(monitor::ST_B));
                                            if c.g[S4][0] != 0 {
                                                c.cycles += 1;
                                                c.g[V0][0] = c.lw(m.wrapping_add(monitor::SUBS));
                                            } else {
                                                c.cycles += 2;
                                                c.g[V0][0] = c.lw(m.wrapping_add(monitor::SUBS));
                                                if c.g[S2][0] == 0 {
                                                    c.cycles += 3;
                                                    c.g[S4][0] = c.g[S2][0];
                                                    c.g[V0][0] =
                                                        sx(r32(c, S1).wrapping_add(r32(c, V0)));
                                                    c.g[S2][0] = c.lw(r32(c, V0));
                                                    c.cycles += 1;
                                                    c.g[V0][0] =
                                                        c.lw(m.wrapping_add(monitor::SUBS));
                                                }
                                            }
                                            c.cycles += 14;
                                            c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
                                            c.sw(
                                                r32(c, V0).wrapping_add(monitor::SUB_POS),
                                                c.g[S2][0],
                                            );
                                            c.g[V1][0] = c.lw(m.wrapping_add(monitor::F54));
                                            c.g[A0][0] = c.lw(m.wrapping_add(monitor::F50));
                                            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, S2)));
                                            c.sw(m.wrapping_add(monitor::F54), c.g[V1][0]);
                                            c.g[V0][0] = c.lw(frame_sp.wrapping_add(monitor::ST_C));
                                            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
                                            c.sw(m.wrapping_add(monitor::F50), c.g[A0][0]);
                                            c.g[V0][0] = c.lw(frame_sp.wrapping_add(monitor::ST_C));
                                            c.g[V1][0] = c.lw(frame_sp.wrapping_add(monitor::ST_B));
                                            c.g[V0][0] =
                                                ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                                            if c.g[V0][0] != 0 {
                                                c.sw(m.wrapping_add(monitor::BUSY), 0);
                                            }
                                            c.cycles += 5;
                                            c.g[V0][0] = c.lw(m.wrapping_add(monitor::SUBS));
                                            c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
                                            c.g[V1][0] =
                                                c.lw(r32(c, V0).wrapping_add(monitor::SUB_ID));
                                            c.sw(m.wrapping_add(monitor::F5C), c.g[V1][0]);
                                        }
                                        // Every path but the dropped
                                        // sub-stream reloads the sub count
                                        // here (the switch's exit).
                                        c.cycles += 1;
                                        c.g[V0][0] = Addr(m.wrapping_add(monitor::COUNT)).lb(c);
                                    }
                                    // The sub-stream loop's tail.
                                    c.cycles += 4;
                                    c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
                                    c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                    let more = c.g[V0][0] != 0;
                                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(monitor::SUB_STRIDE));
                                    if !more {
                                        break;
                                    }
                                    if event_due(c) {
                                        deliver_event_here(c, 0x002a_f160);
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[RA][0] = sx(0x002a_f294);
                            c.g[A0][0] = c.lw(m);
                            call_ptr(c, callee::SND_STATUS, 0x002a_f294)?;
                            c.sw(m.wrapping_add(monitor::STATUS), c.g[V0][0]);
                            break 'body;
                        }

                        // In-memory stream: skip a negative sub or a sub
                        // stream that is gone.
                        c.cycles += 3;
                        c.g[A1][0] = c.lw(m.wrapping_add(monitor::SUB));
                        if (c.g[A1][0] as i64) < 0 {
                            c.sw(m.wrapping_add(monitor::F48), c.g[S4][0]);
                            stored48 = true;
                            break 'body;
                        }
                        c.cycles += 4;
                        c.g[V0][0] = c.lw(m.wrapping_add(monitor::SUBS));
                        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(monitor::SUB_ID));
                        let gone = (c.g[V1][0] as i64) < 0;
                        c.g[A2][0] = sx(frame_sp.wrapping_add(0x20)); // header
                        if gone {
                            break 'body;
                        }
                        c.cycles += 6;
                        c.g[A3][0] = sx(frame_sp.wrapping_add(0x30)); // buffer
                        c.g[RA][0] = sx(0x002a_f06c);
                        c.g[T0][0] = sx(frame_sp.wrapping_add(monitor::POS_SAMPLES));
                        call_ptr(c, callee::SND_STREAM_POSITION, 0x002a_f06c)?;
                        c.g[S5][0] = c.g[V0][0];
                        let failed = (c.g[S5][0] as i64) < 0;
                        c.g[V0][0] = c.lw(m.wrapping_add(monitor::SUBS));
                        if !failed {
                            c.cycles += 5;
                            c.g[RA][0] = sx(0x002a_f080);
                            c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(monitor::SUB_ID));
                            call_ptr(c, callee::SND_SUB_TIME, 0x002a_f080)?;
                            c.g[S2][0] = c.g[V0][0];
                            let ok = (c.g[S2][0] as i64) >= 0;
                            c.g[V0][0] = 0xa;
                            if !ok {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw(m.wrapping_add(monitor::SUBS));
                                c.g[S2][0] = 0;
                                c.sw(r32(c, V0).wrapping_add(monitor::SUB_ID), c.g[S7][0]);
                                break 'body;
                            }
                            // f4C = time * 10; f48 = ms(samples / rate) - f4C.
                            c.cycles += 3;
                            c.g[A0][0] = c.lw(frame_sp.wrapping_add(monitor::POS_SAMPLES));
                            let negative = (c.g[A0][0] as i64) < 0;
                            c.mult(0, S2, S2, V0);
                            if negative {
                                c.cycles += 7;
                                c.g[V1][0] = sx(r32(c, A0) >> 1);
                                c.g[V0][0] = c.g[A0][0] & 0x1;
                                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                c.mtc1(2, 2);
                                c.cvt_s_w(2, 2);
                                c.add_s(2, 2, 2);
                            } else {
                                c.cycles += 4;
                                c.mtc1(4, 2);
                                c.cvt_s_w(2, 2);
                            }
                            c.g[V0][0] = Addr(frame_sp.wrapping_add(monitor::HDR_RATE)).lhu(c);
                            c.cycles += 10;
                            c.g[AT][0] = 0x447a_0000;
                            c.mtc1(1, 1); // f1 = 1000.0
                            c.mtc1(2, 0); // f0 = the rate
                            c.cvt_s_w(0, 0);
                            c.div_s(0, 2, 0);
                            c.mul_s(0, 0, 1);
                            c.cvt_w_s(1, 0);
                            c.mfc1(2, 1);
                            c.g[S4][0] = sx(r32(c, V0).wrapping_sub(r32(c, S2)));
                            c.g[V0][0] = c.lw(m.wrapping_add(monitor::SUBS));
                        }
                        // The sub-stream's position, size, status and id.
                        c.cycles += 12;
                        c.g[S6][0] = sx(r32(c, S1).wrapping_add(1));
                        c.sw(r32(c, V0).wrapping_add(monitor::SUB_POS), c.g[S2][0]);
                        c.g[V1][0] = c.lw(m.wrapping_add(monitor::SUBS));
                        c.g[V0][0] = c.lw(r32(c, V1));
                        c.sw(m.wrapping_add(monitor::F50), c.g[V0][0]);
                        c.g[RA][0] = sx(0x002a_f118);
                        c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(monitor::SUB_ID));
                        call_ptr(c, callee::SND_SUB_STATUS, 0x002a_f118)?;
                        c.sw(m.wrapping_add(monitor::STATUS), c.g[V0][0]);
                        c.g[V0][0] = c.lw(m.wrapping_add(monitor::SUBS));
                        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(monitor::SUB_ID));
                        c.sw(m.wrapping_add(monitor::F5C), c.g[V1][0]);
                        break 'body;
                    }
                    // done: the slot's working values and its dirty flag.
                    if !stored48 {
                        c.cycles += 1;
                        c.sw(m.wrapping_add(monitor::F48), c.g[S4][0]);
                    }
                    c.cycles += 3;
                    c.sw(m.wrapping_add(monitor::F4C), c.g[S2][0]);
                    c.sw(m.wrapping_add(monitor::F60), c.g[S5][0]);
                    c.sb(m.wrapping_add(monitor::DIRTY), 0);
                }
            }
            // The next slot.
            c.cycles += 4;
            c.g[S1][0] = c.g[S6][0];
            c.g[V0][0] = ((c.g[S1][0] as i64) < monitor::SLOTS as i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = 0x4d_0000;
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002a_eff0);
            }
        }
        c.cycles += 14;
        c.g[RA][0] = sx(0x002a_f2c0);
        call_ptr(c, callee::SND_UNLOCK, 0x002a_f2c0)?;
        frame.restore(c, S0, 0x160);
        frame.restore(c, S1, 0x150);
        frame.restore(c, S2, 0x140);
        frame.restore(c, S3, 0x130);
        frame.restore(c, S4, 0x120);
        frame.restore(c, S5, 0x110);
        frame.restore(c, S6, 0x100);
        frame.restore(c, S7, 0xf0);
        frame.restore(c, FP, 0xe0);
        frame.restore_ra(c, 0xd0);
        frame.close(c);
        Ok(())
    })
}

/// `bool SpeechBank_HasId(SpeechBank* b, int id)` (0x2afa58): true when the
/// bank's own id (its first word) equals `id`.
pub fn speech_bank_has_id(c: &mut Cpu) {
    c.cycles += 4;
    let selfp = abi::arg(c, 0);
    let id = c.g[A1][0];
    c.g[V1][0] = c.lw(selfp);
    c.g[V0][0] = id ^ c.g[V1][0];
    c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
    abi::ret(c);
}

/// `void MusicPlayer_SetSongField54(MusicPlayer* p, int v)` (0x2b3cc0): stores
/// `v` at +0x54 of the current song (`+0x408`), if there is one.
pub fn music_player_set_song_field54(c: &mut Cpu) {
    c.cycles += 3;
    let selfp = abi::arg(c, 0);
    let v = c.g[A1][0];
    c.g[V0][0] = c.lw(selfp.wrapping_add(0x408));
    if c.g[V0][0] != 0 {
        let song = c.g[V0][0] as u32;
        c.sw(song.wrapping_add(0x54), v);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void MusicPlayer_SetSongField5c(MusicPlayer* p, int v)` (0x2b4a08): stores
/// `v` at +0x5c of the current song, if there is one.
pub fn music_player_set_song_field5c(c: &mut Cpu) {
    c.cycles += 3;
    let selfp = abi::arg(c, 0);
    let v = c.g[A1][0];
    c.g[V0][0] = c.lw(selfp.wrapping_add(0x408));
    if c.g[V0][0] != 0 {
        let song = c.g[V0][0] as u32;
        c.sw(song.wrapping_add(0x5c), v);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int MusicPlayer_SetSongField50(MusicPlayer* p)` (0x2b46d0): sets the
/// current song's +0x50 word to 1 (when there is a song) and returns 1.
pub fn music_player_set_song_field50(c: &mut Cpu) {
    c.cycles += 3;
    let selfp = abi::arg(c, 0);
    c.g[V1][0] = c.lw(selfp.wrapping_add(0x408));
    let none = c.g[V1][0] == 0;
    c.g[V0][0] = 1;
    if !none {
        c.cycles += 1;
        let song = c.g[V1][0] as u32;
        c.sw(song.wrapping_add(0x50), c.g[V0][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int MusicPlayer_AreLoopsActive(this)` (0x002b46e8): the flag at `+0x4c` of
/// the active stream, or 0 when there is none.
pub fn music_player_are_loops_active(c: &mut Cpu) {
    c.cycles += 3;
    let stream = Addr((c.g[A0][0] as u32).wrapping_add(0x408)).lw(c);
    c.g[V0][0] = stream;
    c.cycles += 2;
    c.g[V0][0] = if stream == 0 {
        0
    } else {
        Addr((stream as u32).wrapping_add(0x4c)).lw(c)
    };
    abi::ret(c);
}

/// `void MusicPlayer_SetSongTag(this, int tag)` (0x002b49c0): store `tag` at
/// `+0x58` of the current song, when the player has one.
pub fn music_player_set_song_tag(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x418)).lw(c);
    let has_song = c.g[V0][0] != 0;
    if has_song {
        c.cycles += 3;
        let song = Addr((c.g[A0][0] as u32).wrapping_add(0x408)).lw(c);
        c.g[A0][0] = song;
        if song != 0 {
            c.sw((song as u32).wrapping_add(0x58), c.g[A1][0]);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `unsigned int Audio_Rand(Rng *unused)` (0x002adf60): the global sound RNG's
/// next value (`Rand_NextMain`).
pub fn audio_rand(c: &mut Cpu) {
    forward(c, 7, 0x10, |_| {}, callee::RAND_NEXT_MAIN, 0x002a_df70)
}

/// `void SongInstance_PathCmdA(void)` (0x002b2120): set PATH control value
/// `0x11000001`.
pub fn song_instance_path_cmd_a(c: &mut Cpu) {
    forward(
        c,
        8,
        0x10,
        |c| c.g[A0][0] = sx(0x1100_0000u32 | 1),
        callee::PATH_SET_TRACK_CONTROL_VALUE,
        0x002b_2134,
    )
}

/// `void BankMonitor_TimerDtor(BankMonitor_Timer *this, int flags)`
/// (0x2ad860).
pub fn bank_monitor_timer_dtor(c: &mut Cpu) {
    guest(c, |c| destroy_if_flag(c, 0x002a_d878))
}

/// `void SpeechInstance_StopStream(SpeechInstance *this)` (0x2b0320): stop the
/// stream array's element this instance refers to (`+0x4`).
pub fn speech_instance_stop_stream(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        // The global holds a pointer to the owner whose first word is the
        // stream-array singleton (`D_004A37B4`, `gp + 0x6c4`).
        let owner = Addr((c.g[GP][0] as u32).wrapping_add(0x6c4)).lw(c);
        c.g[V0][0] = owner;
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(4)).lw(c);
        c.g[A0][0] = Addr(owner as u32).lw(c);
        callee::STREAM_ARRAY_STOP.call(c, 0x002b_0338)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BankMonitor_ResetInstance(self, int instance)` (0x2ad398): reset the
/// 0xc0-byte bank monitor instance at `self + instance*0xc0`.
pub fn bank_monitor_reset_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0xc0;
        frame.save_ra(c, 0);
        c.mult(0, A1, A1, V0);
        let base = Addr(arg(c, 0).wrapping_add(0x8ec)).lw(c);
        c.g[A0][0] = sx((base as u32).wrapping_add(c.g[A1][0] as u32));
        callee::BANK_MONITOR_INSTANCE_RESET.call(c, 0x002a_d3b4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Music player.

/// `void MusicPlayer_Pause(MusicPlayer* self)` (0x2b3a70): pause the current
/// song instance (`self->song` at +0x408), if one is playing.
pub fn music_player_pause(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(at(abi::arg(c, 0), 0x408)).lw(c);
        let song = c.g[A0][0];
        frame.restore_ra(c, 0);
        if song != 0 {
            c.cycles += 3;
            callee::SONG_INSTANCE_PAUSE.call(c, 0x002b_3a8c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `void MusicPlayer_Resume(MusicPlayer* self)` (0x2b3a98): resume the current
/// song instance, if one is loaded.
pub fn music_player_resume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(at(abi::arg(c, 0), 0x408)).lw(c);
        let song = c.g[A0][0];
        frame.restore_ra(c, 0);
        if song != 0 {
            c.cycles += 3;
            callee::SONG_INSTANCE_RESUME.call(c, 0x002b_3ab4)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// `int MusicPlayer_GetSongTag(MusicPlayer* self)` (0x2b49e0): the tag of the
/// current song (+0x58), or -1 while the player holds no tagged song
/// (+0x418 flag).
pub fn music_player_get_song_tag(c: &mut Cpu) {
    c.cycles += 3;
    let self_ = abi::arg(c, 0);
    c.g[V0][0] = Addr(at(self_, 0x418)).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        c.g[A0][0] = Addr(at(self_, 0x408)).lw(c);
        let song = c.g[A0][0];
        if song != 0 {
            c.cycles += 2;
            c.g[V0][0] = Addr(at(song as u32, 0x58)).lw(c);
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = u64::MAX;
    abi::ret(c);
}

/// `cSongConfiguration_dtor` (0x2b1b68): vtable at `+0xa8`.
pub fn c_song_configuration_dtor(c: &mut Cpu) {
    deleting_dtor(c, 0xa8, 0x0048_3980, 0x002b_1b8c);
}

// ---------------------------------------------------------------------------
// Track slots.

/// `PATH_PlatRemoveTrackSlot(int slot)` (0x2ae188): clear every one of the 24
/// entries in the track-slot table at 0x4d3e98 that equals `slot`.
pub fn path_plat_remove_track_slot(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0x004d_0000;
    c.g[A1][0] = 0x17;
    c.g[V1][0] = sx(0x004d_0000u32.wrapping_add(0x3e98));
    let slot = c.g[A0][0];
    loop {
        c.cycles += 3;
        let entry = c.g[V1][0] as u32;
        c.g[V0][0] = Addr(entry).lw(c);
        let same = c.g[V0][0] == slot;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        if same {
            c.cycles += 1;
            Addr(entry).sw(c, 0);
        }
        c.cycles += 2;
        let more = (c.g[A1][0] as i64) >= 0;
        c.g[V1][0] = sx(entry.wrapping_add(4));
        if !more {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        if events::event_due(c) {
            events::deliver_event_here(c, 0x002a_e198);
        }
    }
}

/// `void SpeechSystem_StopInstance(SpeechSystem* s, int i)` (0x2b11b0): stop
/// speech instance `i` if it exists. Leaves the instance list base in `v0`.
pub fn speech_system_stop_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        let i = r32(c, A1) << 2;
        frame.save_ra(c, 0);
        let base = Addr(abi::arg(c, 0)).off(4).lw(c);
        c.g[V0][0] = base;
        c.g[A1][0] = sx(i.wrapping_add(base as u32));
        c.g[A0][0] = Addr(i.wrapping_add(base as u32)).lw(c);
        let empty = c.g[A0][0] == 0;
        frame.restore_ra(c, 0);
        if !empty {
            c.cycles += 3;
            callee::SPEECH_INSTANCE_STOP_STREAM.call(c, 0x002b_11d8)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void BankMonitor_ResetDistances(BankMonitor* bm)` (0x2ad810): set the
/// distance of all 64 voices to -1.0.
pub fn bank_monitor_reset_distances(c: &mut Cpu) {
    const HEAD: u32 = 0x002a_d820;
    c.cycles += 4;
    // `lui at, 0xbf80; mtc1 at, f0`
    c.g[AT][0] = 0xffff_ffff_bf80_0000;
    c.mtc1(AT, 0);
    c.g[A1][0] = 0;
    c.g[V1][0] = 0x3f;
    loop {
        c.cycles += 7;
        let voices = Addr(abi::arg(c, 0)).off(0x8ec).lw(c);
        c.g[V0][0] = voices;
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
        c.g[V0][0] = sx(r32(c, A1).wrapping_add(voices as u32));
        c.swc1(0, r32(c, V0).wrapping_add(0x5c));
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0xc0));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, HEAD);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void* SongInstance_GetRequestStatus(SongInstance* s, sRequestStatus* v)`
/// (0x2b25c8): fill in the song's stream status, then ask the stream driver
/// for its request status.
pub fn song_instance_get_request_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        let v = c.g[A1][0];
        c.g[S0][0] = v;
        frame.save_ra(c, 0x10);
        c.g[A1][0] = sx(frame.at(c, 0));
        callee::SONG_GET_STREAM_STATUS.call(c, 0x002b_25e0)?;
        c.g[A0][0] = Addr(frame.at(c, 4)).lw(c);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, callee::SNDSTRM_GET_REQUEST_STATUS, 0x002b_25ec)?;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void MusicPlayer_PostEvent(MusicPlayer* mp, int event)` (0x2b3b88): post
/// to the current song, if the player has one.
pub fn music_player_post_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let mp = abi::arg(c, 0);
        c.g[V0][0] = Addr(mp).off(0x418).lw(c);
        let playing = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if playing {
            c.cycles += 3;
            c.g[A0][0] = Addr(mp).off(0x408).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                c.g[A2][0] = 0;
                callee::SONG_POST_EVENT.call(c, 0x002b_3bb0)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void MusicPlayer_FadeOut(MusicPlayer* mp, float v)` (0x2b3d48): fade the
/// current song out, if the player has one.
pub fn music_player_fade_out(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let mp = abi::arg(c, 0);
        c.g[V0][0] = Addr(mp).off(0x418).lw(c);
        let playing = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if playing {
            c.cycles += 3;
            c.g[A0][0] = Addr(mp).off(0x408).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                callee::SONG_FADE_OUT.call(c, 0x002b_3d70)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void MusicPlayer_ClearPlaylist(MusicPlayer* mp)` (0x2b3eb0): clear the
/// playlist pointers, the current index and the five pending entries.
pub fn music_player_clear_playlist(c: &mut Cpu) {
    const HEAD: u32 = 0x002b_3ec0;
    c.cycles += 4;
    let mp = abi::arg(c, 0);
    Addr(mp).off(0x3f8).sd(c, 0);
    c.g[V1][0] = 4;
    Addr(mp).off(0x3e8).sw(c, 0);
    c.g[V0][0] = sx(mp.wrapping_add(0x930));
    loop {
        c.cycles += 7;
        Addr(r32(c, V0)).sw(c, 0);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, HEAD);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Bank monitor.

/// `void PATH_PlatShutdown(void)` (0x2ae100): remove the platform tick
/// callback, stop the timer and clear both globals.
pub fn path_plat_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x6a8)).lw(c);
        let absent = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        if !absent {
            c.cycles += 7;
            c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x6ac)).lw(c);
            callee::SND_REMOVE_TICK_CALLBACK.call(c, 0x002a_e118)?;
            c.g[A0][0] = 0x002b_0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fe0));
            callee::EATIMER_REMOVE_CALLBACK.call(c, 0x002a_e124)?;
            Addr((c.g[GP][0] as u32).wrapping_add(0x6a8)).sw(c, 0);
            Addr((c.g[GP][0] as u32).wrapping_add(0x6ac)).sw(c, 0);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int PATH_PlatFindFreeTrackSlot(void)` (0x2ae1b8): index of the first free
/// of 24 track slots, or -1.
pub fn path_plat_find_free_track_slot(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0x004d_0000;
    c.g[V1][0] = 0;
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3e98));
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = c.g[V1][0];
            abi::ret(c);
            return;
        }
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V1][0] as i64) < 24i64) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
        if !more {
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x002a_e1c8);
        }
    }
}

/// `void PATH_PlatBankSlotInit(int* self)` (0x2AE1F0): clear the 0x28-byte bank
/// slot and mark its three ids -1.
pub fn path_plat_bank_slot_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0x28;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MEMSET.call(c, 0x002a_e20c)?;
        c.g[V1][0] = u64::MAX;
        frame.restore_ra(c, 0);
        Addr(r(c, S0)).sw(c, u64::MAX);
        Addr(r(c, S0).wrapping_add(8)).sw(c, u64::MAX);
        Addr(r(c, S0).wrapping_add(4)).sw(c, u64::MAX);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int MusicPlayer_GetDuckToLoops(MusicPlayer* self)` (0x2B4878): how far the
/// music should duck under the loops; 1 while no song is playing (+0x418
/// clear) or the instance is missing (+0x408 clear), else the song's own value.
pub fn music_player_get_duck_to_loops(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = r(c, A0);
        let playing = c.lw_r_2(V0, this.wrapping_add(0x418));
        let idle = playing == 0;
        c.g[V0][0] = 1;
        if idle {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 3;
            let song = c.lw_r_2(A0, this.wrapping_add(0x408));
            frame.restore_ra(c, 0);
            if song != 0 {
                c.cycles += 4;
                callee::SONG_INSTANCE_GET_DUCK_TO_LOOPS.call(c, 0x002b_48a0)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Helpers.

/// `a + off` (32-bit address arithmetic).
#[inline(always)]
pub(crate) fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
pub(crate) fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `lw reg, gp+off`.
#[inline(always)]
pub(crate) fn load_gp(c: &mut Cpu, reg: usize, off: i32) {
    let a = gp(c, off);
    c.g[reg][0] = Addr(a).lw(c);
}

/// `addu` of two register values (32-bit, sign-extended).
#[inline(always)]
pub(crate) fn add32(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}

/// The common epilogue tail: `addiu sp, sp, size; jr ra`.
#[inline(always)]
pub(crate) fn leave(c: &mut Cpu, size: u32) {
    let ra = r32(c, RA);
    c.g[SP][0] = sx(sp(c).wrapping_add(size));
    c.pc = ra;
}

/// `void BankMonitor_KillAllTimers(BankMonitor* self)` (0x2adda0): kill the
/// 48 bank timers at `self+0x98`, stride 0x2c.
pub fn bank_monitor_kill_all_timers(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(p(abi::arg(c, 0), 0x98));
        frame.save_ra(c, 0);
        c.g[S1][0] = 0x2f;
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            callee::BANK_MONITOR_TIMER_KILL.call(c, 0x002a_ddc4)?;
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x2c));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002a_ddb8);
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

/// `void PATH_PlatSetTrackFile(int index, u32 a, u32 b)` (0x2ae768): store
/// `a`/`b` at `+0x28`/`+0x2c` of platform `index` (0..24) when it is not
/// busy (`+0x64` == 0).
pub fn path_plat_set_track_file(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = (c.g[A0][0] < 0x18u64) as u64;
    c.g[V0][0] = 0x004d_0000;
    if c.g[A0][0] >= 0x18u64 {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 6;
    c.g[V1][0] = sx(abi::arg(c, 0) << 2);
    c.g[V0][0] = sx(0x004d_0000u32.wrapping_add(0x3e98));
    c.g[V1][0] = add32(c.g[V1][0], c.g[V0][0]);
    c.g[V1][0] = Addr(r32(c, V1)).lw(c);
    if c.g[V1][0] == 0 {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, V1)).off(0x64).lb(c);
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.sw(p(r32(c, V1), 0x2c), c.g[A2][0]);
    c.sw(p(r32(c, V1), 0x28), c.g[A1][0]);
    c.cycles += 2;
    abi::ret(c);
}

/// `void PATH_PlatClearPrebuffer(int index)` (0x2ae9f8): drop the streaming
/// prebuffer request of platform `index` when it is not busy.
pub fn path_plat_clear_prebuffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x004d_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(0x004d_0000u32.wrapping_add(0x3e98));
        c.g[A0][0] = sx(abi::arg(c, 0) << 2);
        c.g[A0][0] = add32(c.g[A0][0], c.g[V0][0]);
        c.g[A0][0] = Addr(r32(c, A0)).lw(c);
        c.g[V0][0] = Addr(r32(c, A0)).off(0x64).lb(c);
        let busy = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if !busy {
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, A0)).off(0x6c).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).off(4).lw(c);
            callee::SNDSTRM_SET_PREBUFFER.call(c, 0x002a_ea2c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        leave(c, 0x10);
        Ok(())
    })
}

/// `u32 SongInstance_PickRandomLoop(SongInstance* self)` (0x2b24c0):
/// `Audio_Rand() % self->song->loop_count` (0 when there is no song).
pub fn song_instance_pick_random_loop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        load_gp(c, A0, 0x68c);
        callee::AUDIO_RAND.call(c, 0x002b_24d8)?;
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        c.g[V1][0] = Addr(r32(c, A0)).off(0x40).lw(c);
        c.divu(0, V0, V1);
        c.cycles += 5;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        c.g[V0][0] = c.hi[0];
        leave(c, 0x20);
        Ok(())
    })
}

/// `float SongInstance_GetPositionMs(SongInstance* self)` (0x2b2600): the
/// playback position when the request status is the "playing" kind (2),
/// else 0.
pub fn song_instance_get_position_ms(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        callee::SONG_INSTANCE_GET_REQUEST_STATUS.call(c, 0x002b_2610)?;
        c.g[V1][0] = Addr(sp(c)).lw(c);
        c.g[V0][0] = 2;
        let playing = c.g[V1][0] == c.g[V0][0];
        frame.restore_ra(c, 0x10);
        if playing {
            c.cycles += 3;
            c.g[V0][0] = Addr(sp(c)).off(4).lw(c);
            c.mtc1(2, 0);
            c.cvt_s_w(0, 0);
        } else {
            c.cycles += 3;
            c.mtc1(0, 0);
        }
        c.cycles += 2;
        leave(c, 0x20);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Music player.

/// `void MusicPlayer_PlayPreview(MusicPlayer* p, int slot)` (0x2b4978): when
/// the slot's stream (`p[slot+1]->+0xa0`) is loaded, tell the music stream
/// (`p+0x5440`) about it through its slot-3 method.
pub fn music_player_play_preview(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let slot = abi::arg(c, 1);
        c.g[A1][0] = sx(slot << 2);
        c.g[A1][0] = sx(r32(c, A2).wrapping_add(r32(c, A1)));
        c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(4));
        c.g[A1][0] = c.lw(r32(c, V0).wrapping_add(0xa0));
        let unloaded = (c.g[A1][0] as i64) < 0;
        if !unloaded {
            c.cycles += 6;
            c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0x5440));
            let vt = r32(c, V0);
            c.g[A0][0] = c.lh(vt.wrapping_add(0x18));
            c.g[V1][0] = c.lw(vt.wrapping_add(0x1c));
            let method = r32(c, V1);
            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            call_ptr(c, method, 0x002b_49b0)?;
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int PATH_PlatCalcTrackMemSize(int n, int m)` (0x2ae478): the memory a track
/// needs: `n * 0xc + 0x70`, plus the stream size for `m` slots, rounded up to
/// a 16-byte boundary.
pub fn path_plat_calc_track_mem_size(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xc;
        c.mult(0, V0, A0, V0);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(r32(c, V0).wrapping_add(0x70));
        if (c.g[A1][0] as i64) > 0 {
            c.cycles += 3;
            call_ptr(c, callee::SND_BANK_MONITOR_MEM_SIZE, 0x002a_e49c)?;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
        }
        c.cycles += 7;
        c.g[V1][0] = c.g[S0][0] & 0xf;
        c.g[V0][0] = sx(r32(c, S0).wrapping_add(0x10));
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, V1)));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int MusicPlayer_IsSongDataLoaded(self, force)` (0x2b4aa0): true when the
/// song's data has been loaded (`self+0x5438` is the async file, whose result
/// `0x3df980` returns: 1 means loaded).
pub fn music_player_is_song_data_loaded(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        let forced = c.g[A1][0] != 0;
        frame.save_ra(c, 0);
        if forced {
            c.cycles += 2;
            c.g[V0][0] = 1;
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        let self_ = abi::r32(c, A0);
        c.cycles += 3;
        c.g[A0][0] = Addr(self_.wrapping_add(0x5438)).lw(c);
        if c.g[A0][0] == 0 {
            c.g[V0][0] = 0;
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 6;
        call_ptr(c, 0x003d_f980, 0x002b_4ac8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V1][0] = 1;
        let loaded = c.g[A0][0] == c.g[V1][0];
        c.g[V0][0] = 1;
        if !loaded {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BankMonitor_StopAll(BankMonitor* self)` (0x2ad3c0): reset all 64
/// bank-monitor instances.
pub fn bank_monitor_stop_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = 0;
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = 0;
        loop {
            c.cycles += 6;
            c.g[A0][0] = c.g[S1][0];
            callee::BANK_MONITOR_RESET_INSTANCE.call(c, 0x002a_d3e8)?;
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
            c.g[V0][0] = ((c.g[S0][0] as i64) < 64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S0][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002a_d3e0);
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

/// `void PATH_PlatTrackInit(PATH_PlatTrack* self)` (0x2ae138): zero the
/// 0x70-byte track and mark it empty.
pub fn path_plat_track_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0x70;
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0041_6210, 0x002a_e154)?;
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        c.g[V0][0] = 1;
        Addr(this).off(0x65).sb(c, c.g[V0][0]);
        Addr(this).off(8).sw(c, c.g[V1][0]);
        Addr(this).sw(c, c.g[V1][0]);
        Addr(this).off(4).sw(c, c.g[V1][0]);
        Addr(this).off(0x66).sb(c, c.g[V1][0]);
        Addr(this).off(0x5c).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void IniParser_dtor(IniParser* self, int flags)` (0x2a7890): free the
/// owned buffer and, when `flags & 1`, the parser itself.
pub fn ini_parser_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = Addr(this).off(4).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 2;
            call_ptr(c, table::BUILTIN_VEC_DELETE, 0x002a_78b8)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x002a_78cc)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BankMonitor_ResumeAll(self)` (0x2ad300): resume all 64 bank monitor
/// instances and clear the pending count.
pub fn bank_monitor_resume_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = 0x3f;
        frame.save(c, S1, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        loop {
            c.cycles += 6;
            c.g[A0][0] = Addr(r32(c, S2)).off(0x8ec).lw(c);
            c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
            c.g[RA][0] = sx(0x002a_d330);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
            callee::BANK_MONITOR_INSTANCE_RESUME.call(c, 0x002a_d330)?;
            let more = (c.g[S0][0] as i64) >= 0;
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(0xc0));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002a_d320);
            }
        }
        c.cycles += 7;
        Addr(p(r32(c, S2), 0x8e8)).sw(c, 0);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int PATH_PlatGetTrackVolume(idx)` (0x2af428): the track handle's volume,
/// or -8 when the slot is empty.
pub fn path_plat_get_track_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x4d_0000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3e98));
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[S0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
        c.g[V0][0] = c.lw(r32(c, S0));
        if c.g[V0][0] == 0 {
            c.g[V0][0] = u64::MAX.wrapping_sub(7);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, V0)).off(0x65).lb(c);
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[RA][0] = sx(0x002a_f464);
                c.g[A0][0] = 0;
                callee::PATH_PLAT_REFRESH_STATUS.call(c, 0x002a_f464)?;
                c.g[V1][0] = c.lw(r32(c, S0));
            } else {
                c.g[V1][0] = c.lw(r32(c, S0));
            }
            c.cycles += 1;
            c.g[V0][0] = Addr(r32(c, V1)).off(0xc).lw(c);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BankMonitor_PauseAll(BankMonitor* self)` (0x2ad2a8): pause all 64
/// instrument slots.
pub fn bank_monitor_pause_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 1;
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0x3f;
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        Addr(r32(c, S2)).off(0x8e8).sw(c, c.g[V0][0]);
        loop {
            c.cycles += 6;
            c.g[A0][0] = Addr(r32(c, S2)).off(0x8ec).lw(c);
            c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
            c.g[RA][0] = sx(0x002a_d2e0);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
            callee::BANK_MONITOR_INSTANCE_PAUSE.call(c, 0x002a_d2e0)?;
            let more = (c.g[S0][0] as i64) >= 0;
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(0xc0));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002a_d2d0);
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

/// `void SongInstance_Pause(SongInstance* self)` (0x2b2018): stop the song's
/// stream (after the driver lock dance), if it has one.
pub fn song_instance_pause(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x002b_202c);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::SND_LOCK, 0x002b_202c)?;
        c.g[RA][0] = sx(0x002b_2034);
        c.g[A0][0] = c.g[S0][0];
        callee::SONG_INSTANCE_GET_STREAM_HANDLE.call(c, 0x002b_2034)?;
        if (c.g[V0][0] as i64) < 0 {
            // no stream: only the unlock/final notification below
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, callee::SNDSTRM_SET_PITCH, 0x002b_2048)?;
        }
        c.cycles += 10;
        call_ptr(c, callee::SND_UNLOCK, 0x002b_2050)?;
        c.g[A0][0] = 0x1100_0000;
        c.g[A1][0] = 1;
        c.g[A0][0] |= 0x2;
        call_ptr(c, callee::PATH_SET_PAUSED, 0x002b_2060)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SongInstance_Resume(SongInstance* self)` (0x2b2070): the reverse of
/// [`song_instance_pause`], restoring the pitch and unpausing the path.
pub fn song_instance_resume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x002b_2084);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::SND_LOCK, 0x002b_2084)?;
        c.g[RA][0] = sx(0x002b_208c);
        c.g[A0][0] = c.g[S0][0];
        callee::SONG_INSTANCE_GET_STREAM_HANDLE.call(c, 0x002b_208c)?;
        if (c.g[V0][0] as i64) < 0 {
            // no stream
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x1000;
            call_ptr(c, callee::SNDSTRM_SET_PITCH, 0x002b_20a0)?;
        }
        c.cycles += 10;
        call_ptr(c, callee::SND_UNLOCK, 0x002b_20a8)?;
        c.g[A0][0] = 0x1100_0000;
        c.g[A1][0] = 0;
        c.g[A0][0] |= 0x2;
        call_ptr(c, callee::PATH_SET_PAUSED, 0x002b_20b8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SongInstance_PostEvent(SongInstance* self, int code, int force)`
/// (0x2b20c8): queue a music event for the currently playing path, or record
/// its code until the path starts.
pub fn song_instance_post_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[V0][0] = Addr(this).off(0x5c).lw(c);
        let pending;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            if r32(c, A2) == 0 {
                frame.restore_ra(c, 0);
                c.cycles += 2;
                frame.close(c);
                return Ok(());
            }
            c.cycles += 1;
            pending = Addr(this).off(0x2c).lw(c);
        } else {
            pending = Addr(this).off(0x2c).lw(c);
        }
        c.cycles += 2;
        if pending == 0 {
            c.g[V1][0] = Addr(this).off(0x30).lw(c);
            c.cycles += 3;
            c.g[V0][0] = u64::MAX;
            if c.g[V1][0] == c.g[V0][0] {
                Addr(r32(c, A0)).off(0x30).sw(c, c.g[A1][0]);
            }
            c.cycles += 1;
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
        } else {
            c.cycles += 6;
            c.g[A1][0] = sx(r32(c, A1) << 16);
            c.g[A0][0] = u64::MAX;
            c.g[A1][0] = sx((r32(c, A1) as i32 >> 16) as u32);
            call_ptr(c, callee::PATH_POST_EVENT, 0x002b_2100)?;
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
        }
        Ok(())
    })
}

/// `float MusicPlayer_GetMsToNextBeat(MusicPlayer* self)` (0x2b4920): the
/// millisecond distance to the next beat of the playing song, 0 when idle.
pub fn music_player_get_ms_to_next_beat(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x20);
        c.g[V0][0] = Addr(r32(c, S0)).off(0x418).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            c.mtc1(0, 0);
        } else {
            c.cycles += 8;
            call_ptr(c, callee::SND_LOCK, 0x002b_4948)?;
            c.g[A0][0] = Addr(r32(c, S0)).off(0x408).lw(c);
            callee::SONG_INSTANCE_GET_MS_TO_NEXT_BEAT.call(c, 0x002b_4950)?;
            c.mov_s(20, 0);
            call_ptr(c, callee::SND_UNLOCK, 0x002b_4958)?;
            c.mov_s(0, 20);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `int PATH_PlatFindBankSlot(int bank, int id)` (0x2AE230): index of the
/// first of the 24 bank slots (0x4D3EF8, 0x28 bytes each) that matches, or -8.
pub fn path_plat_find_bank_slot(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0x4d0000;
    c.g[V1][0] = 0;
    c.g[A2][0] = sx(0x4d_3ef8);
    loop {
        let found;
        c.cycles += 2;
        let bank_non_negative = (c.g[A0][0] as i64) >= 0;
        c.g[V0][0] = c.lw(ea(c.g[A2][0], 8));
        if bank_non_negative {
            c.cycles += 2;
            if c.g[V0][0] != c.g[A0][0] {
                c.g[V1][0] = sx(ea(c.g[V1][0], 1));
                found = false;
            } else {
                c.cycles += 2;
                if (c.g[A1][0] as i64) < 0 {
                    found = true;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                    if c.g[V0][0] == c.g[A1][0] {
                        found = true;
                    } else {
                        c.cycles += 1;
                        c.g[V1][0] = sx(ea(c.g[V1][0], 1));
                        found = false;
                    }
                }
            }
        } else {
            c.cycles += 2;
            if (c.g[V0][0] as i64) >= 0 {
                c.g[V1][0] = sx(ea(c.g[V1][0], 1));
                found = false;
            } else {
                found = true;
            }
        }
        if found {
            c.cycles += 2;
            c.g[V0][0] = c.g[V1][0];
            abi::ret(c);
            return;
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V1][0] as i64) < 24) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A2][0] = sx(ea(c.g[A2][0], 0x28));
        if !more {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_fff8;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x002a_e240);
        }
    }
}

/// `void SpeechBank_dtor(SpeechBank* self, int flags)` (0x2AF900): release the
/// voice block at `+0`, free the stream at `+0x44` when the flag at `+0x48` is
/// set, and delete the object when bit 0 of `flags` is set.
pub fn speech_bank_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
        call_ptr(c, addr::SPEECH_BANK_FREE_CALLBACK, 0x002a_f920)?;
        c.g[V0][0] = c.lw(ea(c.g[S0][0], 0x48));
        let no_stream = c.g[V0][0] == 0;
        c.g[V0][0] = c.g[S1][0] & 1;
        if !no_stream {
            c.cycles += 3;
            c.g[A0][0] = c.lw(ea(c.g[S0][0], 0x44));
            call_ptr(c, addr::UI_ENGINE_TEARDOWN, 0x002a_f934)?;
            c.g[V0][0] = c.g[S1][0] & 1;
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, addr::MEM_MAN_FREE, 0x002a_f948)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `float SongInstance_GetMsToNextBeat(SongInstance* self)` (0x2B2718): the
/// beat length minus the position modulo it, i.e. the time to the next beat.
pub fn song_instance_get_ms_to_next_beat(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save_fpr(c, 21, 0x28);
        frame.save_fpr(c, 20, 0x20);
        callee::SONG_INSTANCE_GET_BEAT_MS.call(c, 0x002b_2734)?;
        c.mov_s(21, 0);
        c.g[A0][0] = c.g[S0][0];
        callee::SONG_INSTANCE_GET_POSITION_MS.call(c, 0x002b_2740)?;
        c.g[A0][0] = c.g[S0][0];
        c.mov_s(20, 0);
        callee::SONG_INSTANCE_GET_BEAT_MS.call(c, 0x002b_274c)?;
        c.mov_s(12, 20);
        c.mov_s(13, 0);
        call_ptr(c, addr::FMODF, 0x002b_2758)?;
        c.sub_s(0, 21, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x28);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void MusicPlayer_SetEventState(MusicPlayer* self, int event)` (0x2b3bc0):
/// notify the current sequence object when the event id changes.
pub fn music_player_set_event_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = 0;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x418)).lw(c);
        let no_seq = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        let mut equal_case = false;
        if !no_seq {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x41c)).lw(c);
            if c.g[S1][0] != c.g[V0][0] {
                c.cycles += 7;
                c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x5440)).lw(c);
                c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x18)).lh(c);
                c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x1c)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x002b_3c04)?;
                Addr(p(c.g[S0][0] as u32, 0x41c)).sw(c, c.g[S1][0]);
                c.g[V1][0] = 1;
            } else {
                equal_case = true;
            }
        }
        if !equal_case {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 5;
        c.g[V0][0] = c.g[V1][0];
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BankMonitor_TimerDisable(BankMonitor* m)` (0x2ad990): clear the timer
/// word.
pub fn bank_monitor_timer_disable(c: &mut Cpu) {
    c.cycles += 2;
    let m = abi::arg(c, 0);
    Addr(m).sw(c, 0);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// SPCH_NotifyPartCallback: walk a bank's part list, find the instrument that
// matches, and hand it to the bank's callback.

/// `void SPCH_NotifyPartCallback(void* self, SPCH_Bank* bank, int index, void* callbackTable)`
/// (0x3db040): the bank descriptor at `bank` lists `bank[7] & 0xf` parts
/// starting at `bank + ((bank[6] << 1) + 3 & 0x3fc) + 0xc`; for each part whose
/// instrument matches, call the bank callback (`callbackTable[instrument]`)
/// with the part's data. The port accepts all `__entries` resume labels and
/// delivers loop events at the same heads.
pub fn spch_notify_part_callback(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc as u32;
        loop {
            match pc {
                0x003d_b040 => {
                    c.cycles += 18;
                    let frame = Frame::open(c, 0xa0);
                    c.g[V0][0] = 0x45_0000;
                    frame.save(c, 22, 0x80);
                    frame.save(c, 21, 0x70);
                    c.g[S6][0] = c.g[V0][0];
                    frame.save(c, 19, 0x50);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x70));
                    frame.save(c, 17, 0x30);
                    c.g[S3][0] = c.g[A0][0];
                    frame.save(c, 16, 0x20);
                    c.g[S1][0] = c.g[A2][0];
                    frame.save(c, 31, 0x90);
                    c.g[S0][0] = c.g[A1][0];
                    frame.save(c, 20, 0x60);
                    frame.save(c, 18, 0x40);
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x8).lw(c);
                    c.g[S5][0] = c.g[A3][0];
                    pc = if c.g[V1][0] == 0 {
                        0x003d_b1dc
                    } else {
                        0x003d_b088
                    };
                }
                0x003d_b088 => {
                    c.cycles += 17;
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    callee::SPCH_FIND_BANK_TYPE.call(c, 0x003d_b090)?;
                    c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x6).lbu(c);
                    c.g[V0][0] = sx((c.g[S1][0] as u32) << 1);
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x7).lbu(c);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0xc).lhu(c);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3));
                    c.g[S2][0] = c.g[A1][0] & 0xf;
                    c.g[V1][0] = c.g[V1][0] & 0x3fc;
                    c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0xc));
                    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[S4][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    let done = c.g[S2][0] == 0;
                    c.g[A2][0] = 0;
                    pc = if done { 0x003d_b1dc } else { 0x003d_b0cc };
                }
                0x003d_b0cc => {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[A2][0] as u32) << 1);
                    pc = 0x003d_b0d0;
                }
                0x003d_b0d0 => {
                    c.cycles += 11;
                    c.g[S0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x2).lbu(c);
                    c.g[A1][0] = Addr(c.g[V0][0] as u32).lbu(c);
                    c.g[T1][0] = Addr(c.g[V0][0] as u32).off(0x1).lbu(c);
                    c.g[V1][0] = (c.g[A0][0] < 0x6u64) as u64;
                    let sp0 = c.g[SP][0] as u32;
                    Addr(sp0).sw(c, c.g[A1][0]);
                    Addr(sp0.wrapping_add(0x4)).sw(c, c.g[T1][0]);
                    let skip = c.g[V1][0] == 0;
                    Addr(sp0.wrapping_add(0x8)).sw(c, c.g[A0][0]);
                    pc = if skip { 0x003d_b1cc } else { 0x003d_b0fc };
                }
                0x003d_b0fc => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x49_0000;
                    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5ba0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let target = Addr(c.g[V1][0] as u32).lw(c) as u32;
                    c.g[A0][0] = sx(target);
                    pc = match target {
                        0x003d_b118 => 0x003d_b118,
                        0x003d_b1cc => 0x003d_b1cc,
                        _ => {
                            c.jump_out(target, 0x003d_b040, 0x003d_b204);
                            return Ok(());
                        }
                    };
                }
                0x003d_b118 => {
                    c.cycles += 6;
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x4).lbu(c);
                    c.g[S0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
                    c.g[A0][0] = 0;
                    c.g[T2][0] = sx((c.g[V0][0] as u32) >> 2);
                    let none = c.g[T2][0] == 0;
                    c.g[T0][0] = 0;
                    pc = if none { 0x003d_b1a8 } else { 0x003d_b130 };
                }
                0x003d_b130 => {
                    c.cycles += 6;
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x8).lbu(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = Addr(c.g[A2][0] as u32).off(0x2).lbu(c);
                    let found = c.g[V1][0] == c.g[T1][0];
                    pc = if found {
                        c.g[T0][0] = 1;
                        0x003d_b1a8
                    } else {
                        0x003d_b148
                    };
                }
                0x003d_b148 => {
                    c.cycles += 2;
                    c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    pc = 0x003d_b150;
                }
                0x003d_b150 => {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x8).lbu(c);
                    let hit = c.g[V0][0] == c.g[T1][0];
                    c.g[A0][0] = 0;
                    pc = if hit { 0x003d_b178 } else { 0x003d_b15c };
                }
                0x003d_b15c => {
                    c.cycles += 1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    pc = 0x003d_b160;
                }
                0x003d_b160 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 4i64) as u64;
                    let out = c.g[V0][0] == 0;
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    pc = if out { 0x003d_b17c } else { 0x003d_b16c };
                }
                0x003d_b16c => {
                    c.cycles += 3;
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x8).lbu(c);
                    let more = c.g[V1][0] != c.g[T1][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    if more {
                        if back_edge_unwinds(c, 0x003d_b160) {
                            return Ok(());
                        }
                        pc = 0x003d_b160;
                    } else {
                        pc = 0x003d_b178;
                    }
                }
                0x003d_b178 => {
                    c.cycles += 1;
                    c.g[T0][0] = 1;
                    pc = 0x003d_b17c;
                }
                0x003d_b17c => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A3][0] as i64) < (c.g[T2][0] as i64)) as u64;
                    let out = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[A3][0];
                    pc = if out { 0x003d_b1a8 } else { 0x003d_b188 };
                }
                0x003d_b188 => {
                    c.cycles += 7;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A3][0] as u32));
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x8).lbu(c);
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x2).lbu(c);
                    let more = c.g[V0][0] != c.g[T1][0];
                    c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                    if more {
                        if back_edge_unwinds(c, 0x003d_b150) {
                            return Ok(());
                        }
                        pc = 0x003d_b150;
                    } else {
                        pc = 0x003d_b1a4;
                    }
                }
                0x003d_b1a4 => {
                    c.cycles += 1;
                    c.g[T0][0] = 1;
                    pc = 0x003d_b1a8;
                }
                0x003d_b1a8 => {
                    c.cycles += 2;
                    let skip = c.g[T0][0] == 0;
                    c.g[V0][0] = sx((c.g[T1][0] as u32) << 2);
                    pc = if skip { 0x003d_b1cc } else { 0x003d_b1b0 };
                }
                0x003d_b1b0 => {
                    c.cycles += 7;
                    c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x70));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[T0][0] = Addr(c.g[V1][0] as u32).off(0x8).lw(c);
                    c.g[A2][0] = Addr(c.g[V0][0] as u32).lw(c);
                    c.g[A0][0] = c.g[S3][0];
                    let target = c.g[T0][0] as u32;
                    let sp0 = c.g[SP][0] as u32;
                    c.g[A3][0] = Addr(sp0).off(0x10).lw(c);
                    call_ptr(c, target, 0x003d_b1cc)?;
                    pc = 0x003d_b1cc;
                }
                0x003d_b1cc => {
                    c.cycles += 4;
                    c.g[A2][0] = c.g[S0][0];
                    c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[S2][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[V0][0] = sx((c.g[A2][0] as u32) << 1);
                    if more {
                        if back_edge_unwinds(c, 0x003d_b0d0) {
                            return Ok(());
                        }
                        pc = 0x003d_b0d0;
                    } else {
                        pc = 0x003d_b1dc;
                    }
                }
                0x003d_b1dc => {
                    c.cycles += 10;
                    let frame = Frame { size: 0xa0 };
                    frame.restore(c, 31, 0x90);
                    frame.restore(c, 22, 0x80);
                    frame.restore(c, 21, 0x70);
                    frame.restore(c, 20, 0x60);
                    frame.restore(c, 19, 0x50);
                    frame.restore(c, 18, 0x40);
                    frame.restore(c, 17, 0x30);
                    frame.restore(c, 16, 0x20);
                    let ra = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
                    c.pc = ra;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003d_b040, 0x003d_b204);
                    return Ok(());
                }
            }
        }
    })
}

/// `IniParser_AtSectionEnd(parser)` 0x2a7f90: at the end of a section when the
/// cursor has not reached the limit; true when the next character is NUL or
/// `[`.
pub fn ini_parser_at_section_end(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        c.g[V0][0] = lw_at(c, this, 8);
        c.g[V1][0] = lw_at(c, this, 4);
        c.g[A0][0] = Addr(this).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, V1)));
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 1;
        } else {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002a_78f8, 0x002a_7fc4)?;
            c.g[V1][0] = Addr(p(r32(c, S0), 8)).lw(c);
            c.g[A0][0] = Addr(r32(c, V1)).lb(c);
            let zero = c.g[A0][0] == 0;
            c.g[A1][0] = 0;
            if zero {
                c.cycles += 2;
                c.g[A1][0] = 1;
                c.g[V0][0] = 1;
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0x5b;
                let not_bracket = c.g[A0][0] != c.g[V0][0];
                c.g[V0][0] = c.g[A1][0];
                if !not_bracket {
                    c.cycles += 2;
                    c.g[A1][0] = 1;
                    c.g[V0][0] = 1;
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

/// `SpeechInstance::AddBank(self, a, b, c)` (0x2b0348): load a speech bank,
/// append its id and bump the global bank counter.
pub fn speech_instance_add_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        c.g[A1][0] = c.g[A2][0];
        c.g[A2][0] = c.g[A3][0];
        call_ptr(c, 0x002a_f960, 0x002b_0368)?;
        c.g[V1][0] = Addr(p(c.g[S0][0] as u32, 0xf0)).lw(c);
        c.g[A0][0] = sx(p(c.g[S0][0] as u32, 0xf4));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
        c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0xf0)).lw(c);
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
        let empty = c.g[V1][0] == 0;
        c.g[V0][0] = Addr(gp(c, 0x6c8)).lw(c);
        if !empty {
            c.cycles += 4;
            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            Addr(p(c.g[S0][0] as u32, 0xf0)).sw(c, c.g[V1][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            Addr(gp(c, 0x6c8)).sw(c, c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SongInstance::FadeOut(self, seconds)` (0x2b2418): fade the song's stream
/// over `seconds` (locked against the sound thread) and mark the instance
/// faded.
pub fn song_instance_fade_out(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x10);
        frame.save_fpr(c, 20, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.mov_s(20, 12);
        call_ptr(c, 0x003b_58a0, 0x002b_2434)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002b_2488, 0x002b_243c)?;
        let no_stream = (c.g[V0][0] as i64) < 0;
        c.g[A0][0] = c.g[V0][0];
        if !no_stream {
            c.cycles += 7;
            c.g[AT][0] = 0x447a0000;
            c.mtc1(AT, 0);
            c.mul_s(0, 20, 0);
            c.cvt_w_s(1, 0);
            c.mfc1(A1, 1);
            c.g[A2][0] = 0;
            call_ptr(c, 0x003b_7eb0, 0x002b_2460)?;
        }
        c.cycles += 9;
        call_ptr(c, 0x003b_58d8, 0x002b_2468)?;
        c.g[V0][0] = 1;
        c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
        Addr(p(c.g[S0][0] as u32, 0x48)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_fpr(c, 20, 0x20);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    })
}

/// `int PATH_PlatStopTrackSound(int idx)` (0x2af838): stop the platform's
/// track sound through the sound system and mark the platform stopped.
pub fn path_plat_stop_track_sound(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x004d_3e98;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
        sqsp(c, 16, 0x10);
        c.g[A0][0] = abi::addu(c.g[A0][0], c.g[V0][0]);
        c.g[S0][0] = Addr(c.g[A0][0] as u32).lw(c);
        call::call_ptr(c, 0x003b_58a0, 0x002a_f85c)?;
        c.g[V0][0] = Addr(p_2(r32(c, S0), 0x64)).lb(c);
        let stopped = c.g[V0][0] == 0;
        if !stopped {
            c.cycles += 5;
            c.g[V0][0] = Addr(p_2(r32(c, S0), 0x6c)).lw(c);
            c.g[A0][0] = Addr(p_2(c.g[V0][0] as u32, 4)).lw(c);
            call::call_ptr(c, 0x003b_89e0, 0x002a_f874)?;
        } else {
            c.cycles += 2;
            c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
            call::call_ptr(c, 0x003b_78f8, 0x002a_f884)?;
        }
        c.cycles += 9;
        call::call_ptr(c, 0x003b_58d8, 0x002a_f88c)?;
        c.g[V1][0] = 1;
        c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
        Addr(p_2(r32(c, S0), 0x65)).sb(c, c.g[V1][0]);
        c.g[V0][0] = 0;
        lqsp(c, 16, 0x10);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Sound banks, AI and camera.

/// `BankMonitor_StopByOwner` (0x2AD4D8): walk the 64 bank monitor instances and
/// reset the ones owned by `owner`.
pub fn bank_monitor_stop_by_owner(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = 0x3f;
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A0][0];
        c.g[V0][0] = lw_at_2(c, S3, 0x8ec);
        loop {
            c.cycles += 6;
            c.g[S0][0] = low(c, S0).wrapping_sub(1);
            c.g[V0][0] = low(c, S1).wrapping_add(low(c, V0));
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x80)).lw(c);
            c.g[A0][0] = c.g[V0][0];
            let other_owner = c.g[V1][0] != c.g[S2][0];
            c.g[S1][0] = low(c, S1).wrapping_add(0xc0);
            if !other_owner {
                c.cycles += 2;
                callee::BANK_MONITOR_RESET.call(c, 0x002a_d528)?;
            }
            c.cycles += 2;
            if (c.g[S0][0] as i64) >= 0 {
                c.g[V0][0] = lw_at_2(c, S3, 0x8ec);
                if event_due(c) {
                    deliver_event_here(c, 0x002a_d508);
                }
                continue;
            }
            c.cycles += 7;
            frame.restore(c, S0, 0x40);
            frame.restore(c, S1, 0x30);
            frame.restore(c, S2, 0x20);
            frame.restore(c, S3, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            break;
        }
        Ok(())
    })
}

/// `void MusicPlayer_EnableSong(self, idx)` (0x2b3fe8): while fewer than 64
/// songs are enabled, set the `idx`-th bit of the 256-bit mask at `+0x3f8`,
/// bump the count and add one to each of the five per-channel totals whose
/// grid entry for that song is non-zero.
pub fn music_player_enable_song(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A2][0] = c.g[A0][0];
    c.g[A0][0] = lw_r(c, A2, 0x3e8);
    c.g[V0][0] = ((c.g[A0][0] as i64) < 64) as u64;
    let full = c.g[V0][0] == 0;
    c.g[A3][0] = c.g[A1][0];
    if !full {
        c.cycles += 13;
        c.g[A1][0] = 0x14;
        c.g[V1][0] = Addr(low_2(c, A2).wrapping_add(0x3f8)).ld(c);
        c.mult(0, A1, A3, A1); // lo = idx * sizeof(grid row)
        c.g[V0][0] = 0x1;
        c.g[V0][0] = c.g[V0][0] << (c.g[A3][0] & 63);
        c.g[A0][0] = sx(low_2(c, A0).wrapping_add(1));
        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
        sw_r(c, A2, 0x3e8, c.g[A0][0]);
        Addr(low_2(c, A2).wrapping_add(0x3f8)).sd(c, c.g[V1][0]);
        c.g[V0][0] = sx(low_2(c, A2).wrapping_add(0x420));
        c.g[A1][0] = sx(low_2(c, A1).wrapping_add(low_2(c, V0)));
        c.g[A0][0] = sx(low_2(c, A2).wrapping_add(0x920));
        c.g[V1][0] = 0x4;
        loop {
            c.cycles += 3;
            c.g[V0][0] = Addr(low_2(c, A1)).lw(c);
            if c.g[V0][0] == 0 {
                c.g[A0][0] = sx(low_2(c, A0).wrapping_add(4));
            } else {
                c.cycles += 4;
                c.g[V0][0] = Addr(low_2(c, A0)).lw(c);
                c.g[V0][0] = sx(low_2(c, V0).wrapping_add(1));
                sw_r(c, A0, 0, c.g[V0][0]);
                c.g[A0][0] = sx(low_2(c, A0).wrapping_add(4));
            }
            c.cycles += 3;
            c.g[V1][0] = sx(low_2(c, V1).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[A1][0] = sx(low_2(c, A1).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002b_4030);
            }
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void PATH_PlatRemoveBank(int slot)` (0x002ae328): find the platform's
/// bank slot; when it is live, remove the SND bank and clear the slot's
/// fields; returns 0 (or -1 when the slot is not usable).
pub fn path_plat_remove_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S1][0] = sx(0xffff_ffff);
        call_ptr(c, 0x002a_e230, 0x002a_e340)?; // PATH_PlatFindBankSlot
        c.g[A0][0] = c.g[V0][0];
        let live = (c.g[A0][0] as i64) >= 0;
        c.g[V1][0] = 0x28;
        if live {
            c.cycles += 7;
            c.g[V0][0] = sx(0x004d_3ef8);
            c.mult(0, 3, 4, 3);
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(4));
            let empty = (c.g[A0][0] as i64) < 0;
            c.g[V0][0] = 0;
            if !empty {
                c.cycles += 9;
                c.g[A0][0] = sx(0xffff_ffff);
                call_ptr(c, 0x003b_6300, 0x002a_e378)?; // SND_BankRemove
                c.sw((c.g[S0][0] as u32).wrapping_add(0x24), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0);
                c.sw((c.g[S0][0] as u32).wrapping_add(4), c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(8), c.g[S1][0]);
                c.sw(c.g[S0][0] as u32, c.g[S1][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), 0);
                c.g[V0][0] = 0;
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = sx(0xffff_ffff);
        }
        c.cycles += 5;
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

/// `int PATH_PlatGetStatus(int plat, int* out_a, int* out_b)` (0x002af2f0):
/// look the platform up in the bank table, refresh its status if requested,
/// copy the two status words out and return the bank's main status.
pub fn path_plat_get_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
        c.g[V0][0] = sx(0x004d_3e98);
        c.sq(S1, sp(c).wrapping_add(0x20));
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
        c.sq(S2, sp(c).wrapping_add(0x10));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sd(sp(c), c.g[RA][0]);
        c.sq(S0, sp(c).wrapping_add(0x30));
        c.g[S1][0] = c.g[A1][0];
        c.g[S0][0] = c.lw(c.g[A0][0] as u32);
        c.g[S2][0] = c.g[A2][0];
        let absent = c.g[S0][0] == 0;
        c.g[V0][0] = sx(0xffff_fff8);
        if !absent {
            c.cycles += 3;
            c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x65));
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[A0][0] = 0;
                call_ptr(c, 0x002a_efa8, 0x002a_f33c)?; // PATH_PlatRefreshStatus
            }
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
            c.cycles += 6;
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
            c.sw(c.g[S2][0] as u32, c.g[V1][0]);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x60));
            if c.g[V0][0] == 0 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5c));
            }
        }
        c.cycles += 6;
        c.lq(S0, sp(c).wrapping_add(0x30));
        c.lq(S1, sp(c).wrapping_add(0x20));
        c.lq(S2, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        abi::ret(c);
        Ok(())
    })
}

/// `void MusicPlayer_LoadSongData(MusicPlayer* m, void* file, int mode)`
/// (0x002b4a20): cancel any pending load, then either load the buffer into
/// the async file handle (mode 0) or into the engine file (mode != 0).
pub fn music_player_load_song_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
        c.sq(S0, sp(c).wrapping_add(0x30));
        c.sq(S1, sp(c).wrapping_add(0x20));
        c.g[S0][0] = c.g[A0][0];
        c.sq(S2, sp(c).wrapping_add(0x10));
        c.g[S1][0] = c.g[A2][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5438));
        let none = c.g[A0][0] == 0;
        c.g[S2][0] = c.g[A1][0];
        if !none {
            c.cycles += 6;
            call_ptr(c, 0x003d_faf0, 0x002b_4a50)?; // AsyncFile_Cancel
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5438));
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x003d_f9d8, 0x002b_4a60)?; // AsyncFile_GetResult
        }
        c.cycles += 2;
        let direct = c.g[S1][0] != 0;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x5434));
        if !direct {
            c.cycles += 5;
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = 0x6400;
            call_ptr(c, 0x003d_f748, 0x002b_4a74)?; // AsyncFile_LoadInto
            c.sw((c.g[S0][0] as u32).wrapping_add(0x5438), c.g[V0][0]);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = 0x6400;
            call_ptr(c, 0x003e_1b68, 0x002b_4a88)?; // EAFile_LoadFileIntoEx
        }
        c.cycles += 6;
        c.lq(S0, sp(c).wrapping_add(0x30));
        c.lq(S1, sp(c).wrapping_add(0x20));
        c.lq(S2, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        abi::ret(c);
        Ok(())
    })
}

/// `void SpeechSystem_Shutdown(self)` (0x2b0da0): destroy both speech
/// instances, shut the speech system down and free its buffer.
pub fn speechsystem_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 17, 0x20);
        frame.save(c, 18, 0x10);
        c.g[S1][0] = 0;
        frame.save(c, 16, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        loop {
            c.cycles += 6;
            c.g[V0][0] = lw_at_3(c, p(r32(c, S2), 4));
            c.g[S0][0] = sx((c.g[S1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = lw_at_3(c, c.g[V0][0] as u32);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                c.g[A1][0] = 3;
                callee::SPEECH_INSTANCE_DTOR.call(c, 0x002b_0de0)?;
            }
            c.g[V0][0] = lw_at_3(c, p(r32(c, S2), 4));
            c.cycles += 5;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            c.g[V1][0] = ((c.g[S1][0] as i64) < 2) as u64;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            let more = c.g[V1][0] != 0;
            c.sw(c.g[V0][0] as u32, 0);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002b_0dc0);
            }
        }
        c.cycles += 11;
        call_ptr(c, callee::SPCH_SHUTDOWN, 0x002b_0e00)?;
        c.g[A0][0] = lw_at_3(c, p(r32(c, S2), 8));
        call_ptr(c, callee::MEM_FREE, 0x002b_0e08)?;
        c.sw(p(r32(c, S2), 8), 0);
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Audio.

/// `int PATH_PlatSetTrackLevel(int track, int level)` (0x2af580): lock the
/// sound system, set the level on the track's sound, and return the previous
/// lock depth.
pub fn path_plat_set_track_level(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x4d_0000;
        frame.save(c, S1, 0x20);
        c.g[A0][0] = sx(r32(c, A0) << 2);
        frame.save(c, S2, 0x10);
        c.g[V0][0] = sx(0x004d_3e98);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(p(r32(c, A0), r32(c, V0)));
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A1][0];
        c.g[S0][0] = Addr(r32(c, A0)).lw(c);
        let null = c.g[S0][0] == 0;
        c.g[S1][0] = 0xffff_ffff_ffff_fff8;
        if !null {
            c.cycles += 5;
            c.g[RA][0] = sx(0x002a_f5bc);
            call_ptr(c, 0x003b_58a0, 0x002a_f5bc)?;
            c.g[V0][0] = Addr(p(r32(c, S0), 0x64)).lb(c);
            let streamed = c.g[V0][0] == 0;
            c.g[A0][0] = Addr(r32(c, S0)).lw(c);
            if streamed {
                c.cycles += 2;
                c.g[RA][0] = sx(0x002a_f5e0);
                c.g[A1][0] = c.g[S2][0];
                call_ptr(c, 0x003b_8010, 0x002a_f5e0)?;
            } else {
                c.cycles += 4;
                c.g[RA][0] = sx(0x002a_f5d0);
                c.g[A1][0] = c.g[S2][0];
                call_ptr(c, 0x003b_8700, 0x002a_f5d0)?;
            }
            c.cycles += 3;
            c.g[RA][0] = sx(0x002a_f5e8);
            c.g[S1][0] = c.g[V0][0];
            call_ptr(c, 0x003b_58d8, 0x002a_f5e8)?;
            Addr(p(r32(c, S0), 0x14)).sw(c, c.g[S2][0]);
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Bank monitor.

/// `int PATH_PlatAddBank(int a, void* unused, int b, int c)` (0x2ae298): take a
/// free bank-monitor slot for bank `b` (trying `b` then any bank) and store
/// `{b, c, a}`, or return -13 when the table is full.
pub fn path_plat_add_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[S2][0];
        callee::PATH_PLAT_FIND_BANK_SLOT.call(c, 0x002a_e2c0)?;
        c.g[V1][0] = 0x28;
        let mut found = (c.g[V0][0] as i64) >= 0;
        if !found {
            c.cycles += 5;
            c.g[A0][0] = u64::MAX;
            c.g[A1][0] = u64::MAX;
            callee::PATH_PLAT_FIND_BANK_SLOT.call(c, 0x002a_e2d4)?;
            c.g[V1][0] = 0x28;
            found = (c.g[V0][0] as i64) >= 0;
        }
        if found {
            c.cycles += 11;
            c.g[A0][0] = 0x004d_0000;
            c.mult(0, V1, V0, V1);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x3ef8));
            c.g[A1][0] = 1;
            c.g[V0][0] = c.g[S1][0];
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
            Addr(p(r32(c, V1), 0x1c)).sw(c, c.g[A1][0]);
            Addr(p(r32(c, V1), 8)).sw(c, c.g[S0][0]);
            Addr(r32(c, V1)).sw(c, c.g[S2][0]);
            Addr(p(r32(c, V1), 4)).sw(c, c.g[S1][0]);
        } else {
            c.cycles += 1;
            c.g[V0][0] = sx(0xffff_fff3);
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

/// `PATH_PlatSetPaused` (0x2af7a0): pause/resume a platform path (index in
/// `a0`) and remember the old flag. `a1` selects pause (stash the current
/// flag and set the pause byte) or resume.
pub fn path_plat_set_paused(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x004d_0000;
        frame.save(c, S1, 0x10);
        c.g[A0][0] = shl32(c.g[A0][0], 2);
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(0x004d_0000u32.wrapping_add(0x3e98));
        frame.save(c, S0, 0x20);
        c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, V0)));
        let pause = c.g[A1][0] != 0;
        c.g[S0][0] = Addr(r(c, A0)).lw(c);
        if pause {
            c.cycles += 4;
            c.g[V0][0] = Addr(p(r(c, S0), 0x3c)).lhu(c);
            c.g[S1][0] = 0;
            Addr(p(r(c, S0), 0x18)).sw(c, c.g[V0][0]);
        } else {
            c.cycles += 1;
            c.g[S1][0] = Addr(p(r(c, S0), 0x18)).lhu(c);
        }
        c.cycles += 5;
        c.g[RA][0] = sx(0x002a_f7e4);
        call_ptr(c, callee::PATH_PLAT_RESUME, 0x002a_f7e4)?;
        c.g[V0][0] = Addr(p(r(c, S0), 0x64)).lb(c);
        let active = c.g[V0][0] == 0;
        c.g[A1][0] = c.g[S1][0];
        if active {
            c.cycles += 3;
            c.g[A0][0] = Addr(r(c, S0)).lw(c);
            c.g[RA][0] = sx(0x002a_f810);
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, callee::PATH_PLAT_SET_RUNNING, 0x002a_f810)?;
        } else {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(r(c, S0), 0x6c)).lw(c);
            c.g[RA][0] = sx(0x002a_f7fc);
            c.g[A0][0] = Addr(p(r(c, V0), 4)).lw(c);
            call_ptr(c, callee::PATH_PLAT_STOP, 0x002a_f7fc)?;
        }
        c.cycles += 10;
        c.g[RA][0] = sx(0x002a_f818);
        call_ptr(c, callee::PATH_PLAT_SUSPEND, 0x002a_f818)?;
        c.g[V1][0] = 1;
        frame.restore(c, S1, 0x10);
        Addr(p(r(c, S0), 0x65)).sb(c, c.g[V1][0]);
        c.g[V0][0] = 0;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MusicPlayer_LoadLoopBank` (0x2b43d8): pick a random loop of the current
/// song, build its bank name and queue the bank load asynchronously.
pub fn music_player_load_loop_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x130);
        frame.save(c, S0, 0x120);
        frame.save(c, S1, 0x110);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0x100);
        let song = Addr(p(this, 0x408)).lw(c);
        c.g[A0][0] = song;
        if song != 0 {
            c.cycles += 4;
            c.g[V1][0] = Addr(r(c, A0)).lw(c);
            c.g[V0][0] = Addr(p(r(c, V1), 0x40)).lw(c);
            let no_loop = c.g[V0][0] == 0;
            c.g[S1][0] = Addr(p(r(c, A0), 0x34)).lw(c);
            if !no_loop {
                c.cycles += 21;
                c.g[RA][0] = sx(0x002b_4410);
                callee::SONG_INSTANCE_PICK_RANDOM_LOOP.call(c, 0x002b_4410)?;
                c.g[A2][0] = Addr(p(r(c, S0), 0x408)).lw(c);
                c.g[V0][0] = shl32(c.g[V0][0], 5);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(0x44));
                c.g[A1][0] = 0x004a_0000;
                c.g[A3][0] = Addr(r(c, A2)).lw(c);
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = sx(0x004a_0000u32.wrapping_add(0x36e0));
                c.g[A2][0] = sx(r(c, A2).wrapping_add(0x60));
                c.g[RA][0] = sx(0x002b_4438);
                c.g[A3][0] = sx(r(c, A3).wrapping_add(r(c, V0)));
                call_ptr(c, callee::SPRINTF_4, 0x002b_4438)?;
                c.g[V1][0] = Addr(r(c, S0)).lw(c);
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.g[SP][0];
                c.g[A3][0] = 0x100;
                c.g[RA][0] = sx(0x002b_4450);
                c.g[A0][0] = Addr(r(c, V1)).lw(c);
                call_ptr(c, callee::SND_BANK_LOAD_2, 0x002b_4450)?;
                c.g[V1][0] = Addr(p(r(c, S0), 0x408)).lw(c);
                c.g[V0][0] = 1;
                Addr(p(r(c, V1), 0x38)).sw(c, c.g[V0][0]);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x120);
        } else {
            frame.restore(c, S0, 0x120);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Sound platform.

/// `int PATH_PlatSetAuxSend(int platform, int send, int level)` (0x2af4e0):
/// set the auxiliary send on the platform's sound stream (or its channel when
/// the platform has a direct voice), returning the previous value.
pub fn path_plat_set_aux_send(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x004d_0000;
        frame.save(c, S3, 0x30);
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
        frame.save(c, S2, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3e98));
        frame.save(c, S1, 0x10);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A2][0];
        c.g[S0][0] = lw(c, c.g[A0][0] as u32);
        let absent = c.g[S0][0] == 0;
        c.g[S1][0] = 0xffff_ffff_ffff_fff8;
        if !absent {
            c.cycles += 5;
            call_ptr(c, 0x003b_58a0, 0x002a_f524)?; // SND_Lock
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x64)).lb(c);
            let direct = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S3][0];
            if direct {
                c.cycles += 5;
                c.g[A0][0] = lw(c, c.g[S0][0] as u32);
                c.g[A2][0] = c.g[S2][0];
                call_ptr(c, 0x003b_88c8, 0x002a_f53c)?; // SND_SetAuxSend
            } else {
                c.cycles += 3;
                c.g[A0][0] = lw(c, c.g[S0][0] as u32);
                c.g[A2][0] = c.g[S2][0];
                call_ptr(c, 0x003b_8050, 0x002a_f550)?; // SNDSTRM_SetAuxSend
            }
            c.cycles += 3;
            c.g[S1][0] = c.g[V0][0];
            call_ptr(c, 0x003b_58d8, 0x002a_f558)?; // SND_Unlock
            sw(c, p(c.g[S0][0] as u32, 0x10), c.g[S2][0]);
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `SongInstance* SongInstance_dtor(this, flags)` (0x2b1f78): cancel the
/// queued bank, stop the tracks and free the two owned buffers.
pub fn song_instance_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        c.g[V1][0] = Addr(gp_2(c, 0x6fc)).lw(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.g[A1][0] = lw_2(c, S0, 0x34);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        call_ptr(c, 0x0028_a728, 0x002b_1fa4)?;
        c.g[A0][0] = 0x1100_0000;
        c.g[A0][0] |= 1;
        call_ptr(c, table::PATH_STOP_TRACKS, 0x002b_1fb0)?;
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = lw_2(c, V1, 0x40);
        let no_tracks = (c.g[V0][0] as i64) <= 0;
        c.g[A0][0] = 0x1100_0000;
        if !no_tracks {
            c.cycles += 2;
            c.g[A0][0] |= 2;
            call_ptr(c, table::PATH_STOP_TRACKS, 0x002b_1fc8)?;
        }
        c.cycles += 3;
        c.g[A0][0] = lw_2(c, S0, 0x24);
        if c.g[A0][0] == 0 {
            c.g[A0][0] = lw_2(c, S0, 0x28);
        } else {
            c.cycles += 3;
            call_ptr(c, table::EA_MEM_FREE, 0x002b_1fdc)?;
            c.g[A0][0] = lw_2(c, S0, 0x28);
        }
        c.cycles += 2;
        let second_null = c.g[A0][0] == 0;
        c.g[V0][0] = c.g[S1][0] & 1;
        if !second_null {
            c.cycles += 3;
            call_ptr(c, table::EA_MEM_FREE, 0x002b_1ff0)?;
            c.g[V0][0] = c.g[S1][0] & 1;
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x002b_2004)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Speech instance creation.

/// `SpeechInstance* SPCH_CreateInstance(owner, id, a2, a3, t0, t1, t2)`
/// (0x2b0e60): allocate a 0x904-byte speech instance, run its constructor and
/// store it in the owner's instance table.
pub fn spch_create_instance(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x80);
        c.cycles += 43;
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S2, 0x50);
        c.g[S0][0] = sx((c.g[S1][0] as u32) << 2);
        frame.save(c, S3, 0x40);
        c.g[A1][0] = sx(0x0048_0000);
        frame.save(c, S4, 0x30);
        c.g[S2][0] = c.g[T0][0];
        frame.save(c, S5, 0x20);
        c.g[S5][0] = c.g[A2][0];
        frame.save(c, S6, 0x10);
        c.g[S6][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[T1][0];
        c.g[S4][0] = c.g[T2][0];
        c.g[V0][0] = lw_3(c, at_3(c.g[A0][0], 4));
        c.g[A1][0] = sx(0x0048_33b8);
        c.g[A0][0] = 0x904;
        c.g[A2][0] = 0;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[RA][0] = sx(0x002b_0ec0);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x002b_0ec0)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S5][0];
        c.g[A3][0] = c.g[S6][0];
        c.g[T0][0] = c.g[S2][0];
        c.g[T1][0] = c.g[S3][0];
        c.g[T2][0] = c.g[S4][0];
        c.g[RA][0] = sx(0x002b_0ee0);
        c.g[A0][0] = c.g[V0][0];
        callee::SPEECH_INSTANCE_CTOR.call(c, 0x002b_0ee0)?;
        Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Song instance allocation.

/// `SongInstance* SongInstance_Create(...)` (0x2b1b98): allocate a 0xc4-byte
/// song instance from the named pool and construct it from the caller's nine
/// arguments (the ninth is passed on the stack).
pub fn song_instance_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 46;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S3, 0x60);
        let a0 = r(c, A0);
        let a1 = r(c, A1);
        let a2 = r(c, A2);
        let a3 = r(c, A3);
        let t0 = r(c, T0);
        let t1 = r(c, T1);
        let t2 = r(c, T2);
        let t3 = r(c, T3);
        frame.save(c, S0, 0x90);
        // The low byte of t0, sign-extended.
        c.g[S0][0] = sx(((t0 as u32) << 24) as i32 as u32);
        frame.save(c, S2, 0x70);
        frame.save(c, S4, 0x50);
        c.g[S2][0] = sx(a0);
        frame.save(c, S5, 0x40);
        c.g[S4][0] = sx(a2);
        frame.save(c, S1, 0x80);
        c.g[S5][0] = sx(a3);
        frame.save(c, S7, 0x20);
        frame.save(c, S6, 0x30);
        c.g[S6][0] = sx(t1);
        c.g[S7][0] = sx(t2);
        c.g[S3][0] = sx(a1);
        frame.save_ra(c, 0x10);
        c.g[S1][0] = sx(t3);
        c.g[S0][0] = sx((r(c, S0) as i32 >> 24) as u32);

        c.g[A1][0] = sx(0x0048_33e0);
        c.g[A0][0] = 0xc4;
        c.g[A2][0] = 0;
        c.g[RA][0] = sx(0x002b_1bfc);
        c.g[A3][0] = 0;
        call_ptr(c, callee::MALLOC, 0x002b_1bfc)?;

        c.sw(r(c, SP), c.g[S1][0]);
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S4][0];
        c.g[T0][0] = c.g[S5][0];
        c.g[T1][0] = c.g[S0][0];
        c.g[T2][0] = c.g[S6][0];
        c.g[T3][0] = c.g[S7][0];
        c.g[RA][0] = sx(0x002b_1c24);
        c.g[A0][0] = c.g[V0][0];
        callee::SONG_INSTANCE_CTOR.call(c, 0x002b_1c24)?;

        frame.restore(c, 16, 0x90);
        frame.restore(c, 17, 0x80);
        frame.restore(c, 18, 0x70);
        frame.restore(c, 19, 0x60);
        frame.restore(c, 20, 0x50);
        frame.restore(c, 21, 0x40);
        frame.restore(c, 22, 0x30);
        frame.restore(c, 23, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PATH platform.

/// `void PATH_PlatInit(PATH_Plat* p, void* a, void* b)` (0x2ae048): clear the
/// 24 platform path records (`+0x18`, `+8`, `+4`, `0` set to -1), the `+8`
/// field of the parallel table to -1, publish both tables, install the tick
/// callback and create the PATH sound streams.
pub fn path_plat_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx(r(c, SP).wrapping_sub(0x10));
        c.sd(r(c, SP), c.g[RA][0]);
        // v0 = 0x4d3ef8 (record table), v1 = 0x4d3e98 (parallel table).
        c.g[V0][0] = sx(0x004d_3ef8);
        c.g[V1][0] = sx(0x004d_3e98);
        c.g[A3][0] = c.g[A0][0];
        c.g[A2][0] = u64::MAX;
        c.g[A0][0] = 0x17;
        loop {
            c.cycles += 9;
            c.sw(r(c, V1), 0);
            c.g[A0][0] = sx(r(c, A0).wrapping_sub(1));
            c.sw(p(r(c, V0), 0x8), c.g[A2][0]);
            c.g[V1][0] = sx(r(c, V1).wrapping_add(4));
            c.sw(p(r(c, V0), 0x4), c.g[A2][0]);
            c.sw(r(c, V0), c.g[A2][0]);
            c.sw(p(r(c, V0), 0x18), 0);
            let more = (c.g[A0][0] as i64) >= 0;
            c.g[V0][0] = sx(r(c, V0).wrapping_add(0x28));
            if !more {
                c.cycles += 27;
                c.g[A0][0] = c.g[A1][0];
                c.sw(gp_3(c, 0x6a8), c.g[A3][0]);
                c.g[RA][0] = sx(0x002a_e0a4);
                c.sw(gp_3(c, 0x6ac), c.g[A1][0]);
                call_ptr(c, 0x003b_5910, 0x002a_e0a4)?;
                c.g[A0][0] = sx(0x002a_dfe0);
                c.g[A1][0] = 0x5;
                c.g[RA][0] = sx(0x002a_e0b8);
                c.g[A2][0] = 0x64;
                call_ptr(c, 0x003e_57f8, 0x002a_e0b8)?;
                c.g[A2][0] = sx(0x003e_6070);
                c.g[A1][0] = sx(0x0041_75c8);
                c.g[A0][0] = sx(0x0025_2fa0);
                c.g[V1][0] = sx(0x0025_26b8);
                c.g[V0][0] = sx(0x002a_e000);
                c.g[RA][0] = c.ld(r(c, SP));
                c.sw(gp_3(c, 0x690), c.g[A2][0]);
                c.sw(gp_3(c, 0x694), c.g[A1][0]);
                c.sw(gp_3(c, 0x69c), c.g[A0][0]);
                c.sw(gp_3(c, 0x6a0), c.g[V1][0]);
                c.sw(gp_3(c, 0x6a4), c.g[V0][0]);
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002a_e070);
            }
        }
        frame_less_return(c, 0x10);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PATH platform volume.

/// `int PATH_PlatFadeVolume(int idx, int target, int time, int flags)`
/// (0x2af608): lock the platform path, write the target volume and either
/// fade or set it, then unlock; returns the unlock result.
pub fn path_plat_fade_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = sx(0x004d_3e98);
        frame.save(c, S1, 0x30);
        c.g[A0][0] = sx((r(c, A0) as u32) << 2);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, V0)));
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        c.g[S1][0] = c.g[A2][0];
        c.g[S0][0] = Addr(r(c, A0)).lw(c);
        c.g[A3][0] = sx((r(c, A3) as u32) << 24);
        let immediate = c.g[A3][0] != 0;
        c.g[S2][0] = 0;
        'outer: {
            if immediate {
                break 'outer;
            }
            'done: {
                'stream: {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x002a_f650);
                    call_ptr(c, 0x003b_58a0, 0x002a_f650)?;
                    c.g[V0][0] = Addr(p(r(c, S0), 0x64)).lb(c);
                    let plain = c.g[V0][0] == 0;
                    Addr(p(r(c, S0), 8)).sw(c, c.g[S3][0]);
                    if plain {
                        break 'stream;
                    }
                    'fade: {
                        c.cycles += 5;
                        c.g[V0][0] = 0xa;
                        c.g[V1][0] = Addr(p(r(c, S0), 0x6c)).lw(c);
                        c.div(0, 17, 2);
                        let _ = c.g[V0][0] == 0;
                    }
                    c.cycles += 6;
                    c.g[A0][0] = Addr(p(r(c, V1), 4)).lw(c);
                    c.g[A1][0] = c.lo[0];
                    c.g[RA][0] = sx(0x002a_f680);
                    c.g[A2][0] = c.g[S3][0];
                    call_ptr(c, 0x003b_8530, 0x002a_f680)?;
                    break 'done;
                }
                c.cycles += 4;
                c.g[A0][0] = Addr(r(c, S0)).lw(c);
                c.g[A1][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x002a_f698);
                c.g[A2][0] = c.g[S3][0];
                call_ptr(c, 0x003b_7eb0, 0x002a_f698)?;
            }
            c.cycles += 2;
            c.g[RA][0] = sx(0x002a_f6a0);
            c.g[S2][0] = c.g[V0][0];
            call_ptr(c, 0x003b_58d8, 0x002a_f6a0)?;
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MusicPlayer_StopSong(this)` (0x2b3ac0): kill the three bank-monitor
/// timers, cancel and unload the current song's bank, resume the player,
/// release the song instance and clear the song pointer.
pub fn music_player_stop_song(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = abi::arg64(c, 0);

        // Three `ldl`/`ldr` pairs load the callbacks at 0x4a3838, 0x4a3840 and
        // 0x4a3848 into a1 (`gpr 5`).
        for base in [0x004a_3838u32, 0x004a_3840, 0x004a_3848] {
            c.g[V1][0] = Addr(this).lw(c);
            c.g[V0][0] = 0x004a_0000;
            c.ldl(5, base.wrapping_add(7));
            c.ldr(5, base);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
            let ret = match base {
                0x004a_3838 => 0x002b_3aec,
                0x004a_3840 => 0x002b_3b08,
                _ => 0x002b_3b24,
            };
            callee::BANK_MONITOR_KILL_TIMERS_BY_CALLBACK.call(c, ret)?;
        }

        c.g[V1][0] = Addr(this).off(0x408).lw(c);
        if c.g[V1][0] != 0 {
            c.cycles += 9;
            c.g[V0][0] = Addr(this).lw(c);
            c.g[A1][0] = Addr(c.g[V1][0] as u32).off(0x34).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
            callee::SND_LOAD_QUEUE_CANCEL_BANK_3.call(c, 0x002b_3b40)?;
            c.g[V0][0] = Addr(this).lw(c);
            c.g[V1][0] = Addr(this).off(0x408).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[A1][0] = Addr(c.g[V1][0] as u32).off(0x34).lw(c);
            callee::SND_BANK_MAN_UNLOAD_BANK_2.call(c, 0x002b_3b54)?;
        }

        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        callee::MUSIC_PLAYER_RESUME.call(c, 0x002b_3b5c)?;
        c.g[A0][0] = Addr(this).off(0x408).lw(c);
        let no_song = c.g[A0][0] == 0;
        Addr(this).off(0x418).sw(c, 0);
        if !no_song {
            c.cycles += 2;
            c.g[A1][0] = 3;
            callee::SONG_INSTANCE_DTOR.call(c, 0x002b_3b70)?;
        }

        c.cycles += 5;
        Addr(this).off(0x408).sw(c, 0);
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BankMonitor_StopByCallback(this, key)` (0x2ad410): reset every 0xc0-byte
/// bank-monitor instance whose key matches.
pub fn bank_monitor_stop_by_callback(c: &mut Cpu) {
    c.cycles += 14;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.g[S0][0] = c.g[A0][0];
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.g[S1][0] = 0x3f;
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.g[S2][0] = 0u64;
    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[S3][0] = 0u64;
    c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
    c.g[S4][0] = 0xffffffffffffffff;
    c.sd((c.g[SP][0] as u32), c.g[A1][0]);
    'l_2ad448: loop {
        'b_2ad49c: {
            c.cycles += 12;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8ec));
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
            c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
            c.ldr(3, (c.g[SP][0] as u32));
            c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x17));
            c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[V0][0] as u32)));
            c.g[A1][0] = 0u64;
            c.g[V1][0] = c.lh((c.g[SP][0] as u32).wrapping_add(0x12));
            c.g[V0][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0xaa));
            let t = c.g[V1][0] != c.g[V0][0];
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc0));
            if t {
                break 'b_2ad49c;
            } else {
                'b_2ad48c: {
                    c.cycles += 2;
                    let t = c.g[V1][0] == c.g[S4][0];
                    c.g[V1][0] = c.lh((c.g[SP][0] as u32).wrapping_add(0x14));
                    if t {
                        break 'b_2ad48c;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lh((c.g[A0][0] as u32).wrapping_add(0xac));
                        if c.g[V1][0] == c.g[V0][0] {
                            c.g[A1][0] = 0x1;
                            break 'b_2ad49c;
                        } else {
                            break 'b_2ad48c;
                        }
                    }
                }
                c.cycles += 4;
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xac));
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                if c.g[V0][0] == c.g[V1][0] {
                    c.g[A1][0] = 0x1;
                    break 'b_2ad49c;
                } else {
                    break 'b_2ad49c;
                }
            }
        }
        'b_2ad4b0: {
            c.cycles += 2;
            let t = c.g[A1][0] == 0u64;

            if t {
                break 'b_2ad4b0;
            } else {
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8ec));
                c.g[RA][0] = sx(0x002ad4b0);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S3][0] as u32)));
                c.pc = 0x002abb38;
                ee::s08::BankMonitorInstance_Reset(c);
                if c.pc != 0x002ad4b0 {
                    return;
                }
                break 'b_2ad4b0;
            }
        }
        c.cycles += 2;
        let t = (c.g[S1][0] as i64) >= 0;
        c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0xc0));
        if t {
            if c.cycles >= c.next_event {
                c.pc = 0x002ad448;
                c.event();
            }
            continue 'l_2ad448;
        } else {
            c.cycles += 8;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
            let t = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
            c.pc = t;
            return;
        }
    }
}

/// `cSongConfiguration_ctor(cSongConfiguration *self)` (0x2b1a98): zero the
/// configuration, seed the limits (0x5A, 0x32, 0xFFFF, -1, 60.0), and copy
/// the ten 32-byte default name buffers from the game string table.
pub fn csong_configuration_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 42;
        c.g[V0][0] = 0x480000;
        c.g[AT][0] = 0x42f00000;
        c.mtc1(1, 0);
        c.g[T6][0] = c.g[A0][0];
        c.g[V1][0] = 0x64;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3980));
        c.g[T3][0] = 0x4;
        c.g[T4][0] = 0x1;
        c.g[T5][0] = 0xffffffffffffffff;
        c.g[A1][0] = 0x2;
        c.g[A2][0] = 0x8;
        c.g[A3][0] = 0x10;
        c.g[T0][0] = 0x5a;
        c.g[T1][0] = 0x32;
        c.g[T2][0] = 0xffff;
        c.sw((c.g[T6][0] as u32).wrapping_add(0x68), c.g[A1][0]);
        c.g[A0][0] = 0x4a0000;
        c.sw((c.g[T6][0] as u32).wrapping_add(0x70), c.g[A2][0]);
        c.g[T7][0] = c.g[A0][0];
        c.sw((c.g[T6][0] as u32).wrapping_add(0xa8), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[T6][0] as u32).wrapping_add(0x44));
        c.sw((c.g[T6][0] as u32).wrapping_add(0x6c), c.g[T3][0]);
        c.g[A2][0] = 0u64;
        c.sw((c.g[T6][0] as u32).wrapping_add(0x74), c.g[A3][0]);
        c.sw((c.g[T6][0] as u32).wrapping_add(0x7c), c.g[T0][0]);
        c.sw((c.g[T6][0] as u32).wrapping_add(0x84), c.g[T1][0]);
        c.sw((c.g[T6][0] as u32).wrapping_add(0x8c), c.g[V1][0]);
        c.swc1(0, (c.g[T6][0] as u32).wrapping_add(0x90));
        c.sw((c.g[T6][0] as u32).wrapping_add(0x9c), c.g[T2][0]);
        c.sw((c.g[T6][0] as u32).wrapping_add(0xa0), c.g[T5][0]);
        c.sw((c.g[T6][0] as u32).wrapping_add(0xa4), c.g[T4][0]);
        c.sw((c.g[T6][0] as u32).wrapping_add(0x40), 0u64);
        c.sw((c.g[T6][0] as u32).wrapping_add(0x64), c.g[T3][0]);
        c.sw((c.g[T6][0] as u32).wrapping_add(0x78), 0u64);
        c.sw((c.g[T6][0] as u32).wrapping_add(0x80), c.g[V1][0]);
        c.sw((c.g[T6][0] as u32).wrapping_add(0x88), c.g[V1][0]);
        c.sw((c.g[T6][0] as u32).wrapping_add(0x94), c.g[T4][0]);
        c.sw((c.g[T6][0] as u32).wrapping_add(0x98), c.g[T5][0]);
        c.g[V0][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x35a8));
        c.sb((c.g[T6][0] as u32), c.g[V0][0]);
        c.g[V1][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x35a8));
        c.sb((c.g[T6][0] as u32).wrapping_add(0x20), c.g[V1][0]);
        'l_2b1b40: loop {
            c.cycles += 7;
            c.g[V0][0] = c.lbu((c.g[T7][0] as u32).wrapping_add(0x35a8));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
            c.sb((c.g[A1][0] as u32), c.g[V0][0]);
            let t = (c.g[A2][0] as i64) >= 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002b1b40);
                }
                continue 'l_2b1b40;
            } else {
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = c.g[T6][0];
                c.pc = t;
                return Ok(());
            }
        }
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Music banks and menu items.

/// `int PATH_PlatRemoveOldestBank(self)` (0x2ae3a8): scan the 24 bank slots
/// for the oldest (smallest positive age), remove that bank, and return 1
/// when one was found. A resumable loop: the back edge at 0x2ae3f0 is a legal
/// re-entry point.
pub fn path_plat_remove_oldest_bank(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        // The frame is opened at the entry arm and reused by the resume arms.
        let mut frame = Frame { size: 0x70 };
        loop {
            match pc {
                0x002a_e3a8 => {
                    c.cycles += 17;
                    frame = Frame::open(c, 0x70);
                    frame.save(c, S1, 0x50);
                    frame.save(c, S0, 0x60);
                    frame.save(c, S2, 0x40);
                    frame.save(c, S3, 0x30);
                    frame.save(c, S4, 0x20);
                    frame.save(c, S5, 0x10);
                    frame.save_ra(c, 0);
                    c.g[S1][0] = 0x7ff_ffff;
                    c.g[S2][0] = c.g[A0][0];
                    c.g[S3][0] = u64::MAX;
                    c.g[S0][0] = 0;
                    c.g[S5][0] = 0x004d_3ef8;
                    c.g[S4][0] = 0x28;
                    pc = 0x002a_e414;
                }
                0x002a_e3ec => {
                    c.cycles += 1;
                    pc = 0x002a_e3f0;
                }
                0x002a_e3f0 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[V1][0] = Addr(p_3(c.g[V0][0], 0x24)).lw(c);
                    let negative = (c.g[V1][0] as i64) < 0;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    if negative {
                        pc = 0x002a_e410;
                    } else {
                        pc = 0x002a_e400;
                    }
                }
                0x002a_e400 => {
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                        pc = 0x002a_e414;
                    } else {
                        pc = 0x002a_e408;
                    }
                }
                0x002a_e408 => {
                    c.cycles += 2;
                    c.g[S1][0] = c.g[V1][0];
                    c.g[S3][0] = c.g[S0][0];
                    pc = 0x002a_e410;
                }
                0x002a_e410 => {
                    c.cycles += 1;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    pc = 0x002a_e414;
                }
                0x002a_e414 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < 24) as u64;
                    let done = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[S2][0];
                    if done {
                        pc = 0x002a_e430;
                    } else {
                        pc = 0x002a_e420;
                    }
                }
                0x002a_e420 => {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S0][0];
                    callee::PATH_PLAT_FIND_BANK_SLOT_2.call(c, 0x002a_e428)?;
                    let found = (c.g[V0][0] as i64) >= 0;
                    abi::mult(c, c.g[A2][0] as u32, c.g[S4][0] as u32);
                    if found {
                        if c.cycles >= c.next_event {
                            c.pc = 0x002a_e3f0;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x002a_e3f0;
                    } else {
                        pc = 0x002a_e430;
                    }
                }
                0x002a_e430 => {
                    c.cycles += 3;
                    c.g[V0][0] = u64::MAX;
                    let none = c.g[S3][0] == c.g[V0][0];
                    c.g[A0][0] = c.g[S2][0];
                    if none {
                        pc = 0x002a_e44c;
                    } else {
                        pc = 0x002a_e43c;
                    }
                }
                0x002a_e43c => {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S3][0];
                    callee::PATH_PLAT_REMOVE_BANK.call(c, 0x002a_e444)?;
                    c.lq(16, frame.at(c, 0x60));
                    pc = 0x002a_e454;
                }
                0x002a_e44c => {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    c.lq(16, frame.at(c, 0x60));
                    pc = 0x002a_e454;
                }
                0x002a_e454 => {
                    c.cycles += 8;
                    frame.restore(c, S1, 0x50);
                    frame.restore(c, S2, 0x40);
                    frame.restore(c, S3, 0x30);
                    frame.restore(c, S4, 0x20);
                    frame.restore(c, S5, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x002a_e3a8, 0x002a_e474);
                    return Ok(());
                }
            }
        }
    });
}

// ---------------------------------------------------------------------------
// BankMonitor_StopByBank (0x00002ad650).

/// `BankMonitor_StopByBank`: walk the 32 sound bank slots and fade/stop the instances that belong to the given bank.
pub fn bank_monitor_stop_by_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x68));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S0][0] = 0u64;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S1][0] = c.g[A0][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S2][0] = 0x3f;
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S3][0] = c.g[A1][0];
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S4][0] = 0x1;
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.mov_s(20, 12);
        c.mtc1(0, 21);
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8ec));
        'l_2ad698: loop {
            'b_2ad700: {
                c.cycles += 5;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                call_ptr(c, 0x002abb80, 0x002ad6a4)?;
                if c.g[V0][0] != 0u64 {
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc0));
                    break 'b_2ad700;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8ec));
                    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[V1][0] = c.lw((c.g[A3][0] as u32));
                    if c.g[V1][0] != c.g[S4][0] {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc0));
                        break 'b_2ad700;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0xc));
                        c.c_eq_s(20, 21);
                        if c.g[V0][0] != c.g[S3][0] {
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc0));
                            break 'b_2ad700;
                        } else {
                            'b_2ad6f4: {
                                c.cycles += 5;
                                c.g[A0][0] = c.g[A3][0];
                                c.mov_s(12, 20);
                                c.g[A1][0] = 0xffffffffffffffff;
                                let t = c.fcc();
                                c.g[A2][0] = 0x1;
                                if t {
                                    break 'b_2ad6f4;
                                } else {
                                    c.cycles += 4;
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc0));
                                    call_ptr(c, 0x002acb80, 0x002ad6ec)?;
                                    let t = true;

                                    if t {
                                        break 'b_2ad700;
                                    } else {
                                        break 'b_2ad6f4;
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[A0][0] = c.g[A3][0];
                            call_ptr(c, 0x002abb38, 0x002ad6fc)?;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc0));
                            break 'b_2ad700;
                        }
                    }
                }
            }
            c.cycles += 2;
            if (c.g[S2][0] as i64) >= 0 {
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8ec));
                if event_due(c) {
                    deliver_event_here(c, 0x002ad698);
                }
                continue 'l_2ad698;
            } else {
                c.cycles += 10;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x68));
                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.pc = t;
                return Ok(());
            }
        }
        Ok(())
    });
}

/// `MusicPlayer_EnableSongByName(self, name)` (0x2b3ee8): find a song by
/// name (`strstr` of the name in each entry) and mark it enabled in the
/// 64-bit enable mask, bumping the per-part counters; gives up when the
/// entry table is full (64 songs).
pub fn music_player_enable_song_by_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x80);
        let this = abi::arg(c, 0);
        frame.save(c, 16, 0x70);
        frame.save(c, 21, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x60);
        frame.save(c, 18, 0x50);
        frame.save(c, 19, 0x40);
        frame.save(c, 20, 0x30);
        frame.save(c, 22, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, p(this, 0x3e8));
        c.g[V0][0] = ((c.g[V0][0] as i64) < 64) as u64;
        let full = c.g[V0][0] == 0;
        c.g[S5][0] = c.g[A1][0];
        let mut matched = false;
        if !full {
            c.cycles += 3;
            c.g[V0][0] = lw(c, p(this, 0x3e4));
            c.g[S1][0] = 0;
            if (c.g[V0][0] as i64) > 0 {
                c.cycles += 5;
                c.g[S6][0] = 1;
                c.g[S2][0] = sx(this.wrapping_add(0x124));
                c.g[S4][0] = sx(this.wrapping_add(0x420));
                c.g[S3][0] = 0;
                'songs: loop {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S5][0];
                    call_ptr(c, 0x0041_ad80, 0x002b_3f4c)?; // strstr
                    let found = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S6][0] << (c.g[S1][0] & 63);
                    if found {
                        matched = true;
                        c.cycles += 9;
                        c.g[V0][0] = c.ld(p(this, 0x3f8));
                        c.g[V1][0] = lw(c, p(this, 0x3e8));
                        c.g[A2][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S4][0] as u32));
                        c.g[V0][0] |= c.g[A0][0];
                        c.g[A1][0] = sx(this.wrapping_add(0x920));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                        c.sd(p(this, 0x3f8), c.g[V0][0]);
                        sw_2(c, p(this, 0x3e8), c.g[V1][0]);
                        c.g[V1][0] = 4;
                        loop {
                            c.cycles += 3;
                            c.g[V0][0] = lw(c, c.g[A2][0] as u32);
                            if c.g[V0][0] == 0 {
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = lw(c, c.g[A1][0] as u32);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                                sw_2(c, c.g[A1][0] as u32, c.g[V0][0]);
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
                            }
                            c.cycles += 3;
                            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                            let more = (c.g[V1][0] as i64) >= 0;
                            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(4));
                            if !more {
                                c.cycles += 2;
                                break 'songs;
                            }
                            if event_due(c) {
                                deliver_event_here(c, 0x002b_3f78);
                            }
                        }
                        matched = true;
                        break 'songs;
                    }
                    c.cycles += 6;
                    c.g[V0][0] = lw(c, p(this, 0x3e4));
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0xb));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x14));
                    if !more {
                        break 'songs;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002b_3f40);
                    }
                }
            }
        }
        if matched {
            c.lq(16, frame.at(c, 0x70));
        } else {
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x70));
        }
        c.cycles += 9;
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

pub(crate) mod speech_stream_ready {
    use super::*;

    pub const STREAM_ARRAY_GET_STATE: Callee = Callee {
        addr: 0x002a_ae08,
        f: ee::s08::f_002aae08,
    };

    pub const STREAM_ARRAY_IS_PLAYING: Callee = Callee {
        addr: 0x002a_b028,
        f: ee::s08::f_002ab028,
    };

    pub const STREAM_ARRAY_GET_TIME_REMAINING: Callee = Callee {
        addr: 0x002a_ae78,
        f: ee::s08::f_002aae78,
    };

    /// `gp+0x6c4`: the speech system (its first word is the stream array).
    pub const SPEECH_GP: u32 = 0x6c4;

    /// Speech instance: its stream slot.
    pub const STREAM_SLOT: u32 = 0x4;

    /// A playing stream counts as ready when less than this much is left.
    pub const READY_REMAINING: i64 = 500;
}

pub(crate) mod song_set_volume {
    use super::*;

    /// `SND_Lock()` / `SND_Unlock()` (table calls).
    pub const SND_LOCK: u32 = 0x003b_58a0;

    pub const SND_UNLOCK: u32 = 0x003b_58d8;

    pub const GET_STREAM_HANDLE: Callee = Callee {
        addr: 0x002b_2488,
        f: ee::s08::f_002b2488,
    };

    /// `SNDSTRM_SetLowpass(handle, value)` (table call): the song's volume
    /// goes through the stream's low-pass parameter.
    pub const SNDSTRM_SET_LOWPASS: u32 = 0x003b_80a0;

    /// Song instance: the current volume.
    pub const VOLUME: u32 = 0x44;

    pub const MAX_VOLUME: u64 = 0xffff;
}

pub(crate) mod path {

    pub const SLOTS: u32 = 0x0051_7610;

    pub const SLOT_SIZE: u32 = 0x928;

    pub const SLOT_COUNT: u64 = 4;

    /// Which id classes the file holds (top nibble: kind, next nibble: group).
    pub const MASK: u32 = 0x30;

    /// Non-zero when the slot is loaded.
    pub const LOADED: u32 = 0x34;

    /// The 24 tracks, one per bit of the id's low 24 bits.
    pub const TRACKS: u32 = 0x58;

    pub const TRACK_BITS: u64 = 24;
}

/// `int SpeechInstance_IsStreamReady(SpeechInstance* this)` (0x2b0290): 1
/// when the instance's stream slot is idle (state 0); for a slot in state 1,
/// 1 when it is playing with less than 500 left; 0 otherwise.
pub fn speech_instance_is_stream_ready(c: &mut Cpu) {
    use speech_stream_ready::*;
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        let speech = Addr(r32(c, GP).wrapping_add(SPEECH_GP));
        c.g[V0][0] = speech.lw(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(STREAM_SLOT)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0)).lw(c);
        STREAM_ARRAY_GET_STATE.call(c, 0x002b_02b4)?;
        c.g[S1][0] = c.g[V0][0]; // the state
        'done: {
            c.g[V0][0] = 1;
            if c.g[S1][0] == 0 {
                break 'done;
            }
            c.cycles += 3;
            c.g[V0][0] = 1;
            let not_one = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 0;
            if not_one {
                break 'done;
            }
            c.cycles += 6;
            c.g[V0][0] = speech.lw(c);
            c.g[A1][0] = Addr(r32(c, S0).wrapping_add(STREAM_SLOT)).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).lw(c);
            STREAM_ARRAY_IS_PLAYING.call(c, 0x002b_02dc)?;
            let playing = c.g[V0][0] != 0;
            c.g[V0][0] = speech.lw(c);
            if !playing {
                c.cycles += 1;
                c.g[V0][0] = 0;
                break 'done;
            }
            c.cycles += 9;
            c.g[A1][0] = Addr(r32(c, S0).wrapping_add(STREAM_SLOT)).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).lw(c);
            STREAM_ARRAY_GET_TIME_REMAINING.call(c, 0x002b_02f0)?;
            // (int)remaining < 500 ? state : 0.
            c.cvt_w_s(1, 0);
            c.mfc1(V1, 1);
            c.g[V0][0] = 0;
            c.g[V1][0] = ((c.g[V1][0] as i64) < READY_REMAINING) as u64;
            if c.g[V1][0] != 0 {
                c.g[V0][0] = c.g[S1][0];
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

/// `void SongInstance_SetVolume(SongInstance* this, int volume)` (0x2b2160):
/// clamp the volume to 0..0xffff; if it changed, under the SND lock, store
/// it and pass it to the song's stream (when the song has one).
pub fn song_instance_set_volume(c: &mut Cpu) {
    use song_set_volume::*;
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = MAX_VOLUME;
        frame.save(c, S0, 0x20);
        c.g[V1][0] = u64::MAX;
        c.g[S0][0] = c.g[V0][0];
        frame.save(c, S1, 0x10);
        c.g[V0][0] = ((MAX_VOLUME as i64) < (c.g[A1][0] as i64)) as u64;
        frame.save_ra(c, 0);
        if c.g[V0][0] == 0 {
            c.g[S0][0] = c.g[A1][0];
        }
        c.g[S1][0] = c.g[A0][0];
        c.g[V1][0] = (-1 < (c.g[S0][0] as i64)) as u64;
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(VOLUME)).lw(c);
        if c.g[V1][0] == 0 {
            c.g[S0][0] = 0;
        }
        // s0: the clamped volume.
        if c.g[S0][0] != c.g[V0][0] {
            c.cycles += 6;
            call_ptr(c, SND_LOCK, 0x002b_21a4)?;
            c.g[A0][0] = c.g[S1][0];
            GET_STREAM_HANDLE.call(c, 0x002b_21ac)?;
            let no_stream = (c.g[V0][0] as i64) < 0;
            c.g[A0][0] = c.g[V0][0];
            if !no_stream {
                c.cycles += 3;
                Addr(r32(c, S1).wrapping_add(VOLUME)).sw(c, c.g[S0][0]);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, SNDSTRM_SET_LOWPASS, 0x002b_21c0)?;
            }
            c.cycles += 3;
            call_ptr(c, SND_UNLOCK, 0x002b_21c8)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int PATH_PlatSetVolume(int path, int volume)` (0x2af6c0, bankmonitor.cpp):
/// sets the volume of a playing path-music stream. `paths[path]` (the table
/// at 0x4d3e98) is the stream record: +0x0 stream handle, +0x8 pending
/// volume (-1: none), +0xc current volume, +0x64 "is a sound, not a stream"
/// flag (its channel handle at `*(+0x6c) + 4`), +0x65 status dirty flag.
/// Returns 0 when the audio system is off (`+0x6280` of the audio system),
/// -8 when a different volume change is still pending; else clears the
/// pending volume and, if the volume changed and both are non-negative,
/// sets it under `SND_Lock` and records it.
pub fn path_plat_set_volume(c: &mut Cpu) {
    const PATHS: u32 = 0x004d_3e98;
    const HANDLE: u32 = 0x0;
    const PENDING: u32 = 0x8;
    const CURRENT: u32 = 0xc;
    const IS_SOUND: u32 = 0x64;
    const DIRTY: u32 = 0x65;
    const SOUND: u32 = 0x6c;
    const AUDIO_ENABLED: u32 = 0x6280;
    /// Where the generated body leaves the function.
    enum Exit {
        /// 0x2af784: `v0 = s2` first.
        Result,
        /// 0x2af788: restore s0, then the rest.
        RestoreS0,
        /// 0x2af78c: s0 already restored.
        Rest,
    }
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 17, 0x20);
        c.g[A0][0] = sx(r32(c, A0) << 2);
        frame.save(c, 18, 0x10);
        c.g[V0][0] = sx(PATHS);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(PATHS));
        frame.save(c, 16, 0x30);
        c.g[S1][0] = c.g[A1][0];
        c.g[S0][0] = Addr(r32(c, A0)).lw(c);
        c.g[S2][0] = 0;
        call_ptr(c, 0x0028_b180, 0x002a_f6f4)?; // SSXAudioSystem_GetInstance
        c.g[V1][0] = Addr(r32(c, V0)).off(AUDIO_ENABLED).lw(c);
        c.g[V0][0] = 0;
        let exit = if c.g[V1][0] == 0 {
            Exit::RestoreS0
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S0)).off(DIRTY).lb(c);
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[A0][0] = 0;
                Callee {
                    addr: 0x002a_efa8,
                    f: ee::s08::f_002aefa8,
                }
                .call(c, 0x002a_f714)?;
            }
            let path = Addr(r32(c, S0));
            let pending = path.off(PENDING).lw(c);
            c.g[V0][0] = pending;
            c.cycles += 2;
            let current = path.off(CURRENT).lw(c);
            c.g[V1][0] = current;
            let busy = (pending as i64) >= 0 && {
                c.cycles += 2;
                pending != current
            };
            if busy {
                c.g[S2][0] = (-8i64) as u64;
                Exit::Result
            } else {
                c.cycles += 3;
                c.g[V0][0] = u64::MAX;
                path.off(PENDING).sw(c, u64::MAX);
                if current == c.g[S1][0] {
                    Exit::Result
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S2][0];
                    if (c.g[S1][0] as i64) < 0 {
                        Exit::RestoreS0
                    } else {
                        c.cycles += 2;
                        if (current as i64) < 0 {
                            frame.restore(c, 16, 0x30);
                            Exit::Rest
                        } else {
                            c.cycles += 5;
                            call_ptr(c, 0x003b_58a0, 0x002a_f74c)?; // SND_Lock
                            c.g[V0][0] = Addr(r32(c, S0)).off(IS_SOUND).lb(c);
                            c.g[A1][0] = c.g[S1][0];
                            if c.g[V0][0] != 0 {
                                c.cycles += 5;
                                let sound = Addr(r32(c, S0)).off(SOUND).lw(c);
                                c.g[V0][0] = sound;
                                c.g[A0][0] = Addr(sound as u32).off(4).lw(c);
                                call_ptr(c, 0x003b_85f0, 0x002a_f764)?; // SND_SetVolume
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = Addr(r32(c, S0)).off(HANDLE).lw(c);
                                c.g[A1][0] = c.g[S1][0];
                                call_ptr(c, 0x003b_7e70, 0x002a_f778)?; // SNDSTRM_SetVolume
                            }
                            c.cycles += 3;
                            call_ptr(c, 0x003b_58d8, 0x002a_f780)?; // SND_Unlock
                            let volume = c.g[S1][0];
                            Addr(r32(c, S0)).off(CURRENT).sw(c, volume);
                            Exit::Result
                        }
                    }
                }
            }
        };
        if let Exit::Result = exit {
            c.cycles += 1;
            c.g[V0][0] = c.g[S2][0];
        }
        if !matches!(exit, Exit::Rest) {
            c.cycles += 1;
            frame.restore(c, 16, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SpeechSystem_Update(SpeechSystem* self)` (0x2b0c78,
/// bankmonitor.cpp): updates both speech instances (`self->instances[i]`,
/// the array at +0x4; empty slots skipped) with `SpeechInstance_Update(inst,
/// i)`, then ticks the event contexts (`SpeechSystem_TickEventContexts`).
pub fn speech_system_update(c: &mut Cpu) {
    const INSTANCES: u32 = 0x4;
    const INSTANCE_UPDATE: Callee = Callee {
        addr: 0x002a_fc88,
        f: ee::s08::f_002afc88,
    };
    const TICK_EVENT_CONTEXTS: Callee = Callee {
        addr: 0x002b_1720,
        f: ee::s08::f_002b1720,
    };
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = 0;
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V1][0] = Addr(r32(c, S1)).off(INSTANCES).lw(c);
        loop {
            c.cycles += 5;
            let at = (r32(c, S0) << 2).wrapping_add(r32(c, V1));
            c.g[V0][0] = sx(at);
            c.g[A0][0] = Addr(at).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                c.g[A1][0] = c.g[S0][0];
                INSTANCE_UPDATE.call(c, 0x002b_0cb4)?;
            }
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
            c.cycles += 3;
            let more = (c.g[S0][0] as i64) < 2;
            c.g[V0][0] = more as u64;
            if !more {
                break;
            }
            c.g[V1][0] = Addr(r32(c, S1)).off(INSTANCES).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x002b_0c98);
            }
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        TICK_EVENT_CONTEXTS.call(c, 0x002b_0ccc)?;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `PATHTrack *PATH_FindTrack(u32 id)` (0x3d5290): the track named by `id`:
/// the first loaded slot whose class mask shares the id's top nibble and its
/// group nibble, then the track of the lowest set bit among the id's low 24
/// bits. 0 when no slot has it.
pub fn path_find_track(c: &mut Cpu) {
    use path::*;
    c.cycles += 14;
    let id = c.g[A0][0];
    c.g[T3][0] = sx(SLOTS + TRACKS);
    c.g[T2][0] = id & 0xffff_ffff_f000_0000;
    c.g[T1][0] = id & 0x0f00_0000;
    c.g[A2][0] = 0x00ff_ffff;
    c.g[T0][0] = 0;
    c.g[A0][0] = id & 0x00ff_ffff;
    c.g[A1][0] = sx(SLOTS + LOADED);
    c.g[A3][0] = 0;
    loop {
        // a1: this slot's LOADED word; a3: the slot's offset.
        c.cycles += 3;
        let loaded = Addr(c.g[A1][0] as u32).lw(c);
        c.g[V0][0] = loaded;
        c.g[V1][0] = Addr((c.g[A1][0] as u32).wrapping_sub(LOADED - MASK)).lw(c);
        let mut skip_cycle = true;
        if loaded != 0 {
            c.cycles += 3;
            let mask = c.g[V1][0];
            let kind = mask & c.g[T2][0];
            c.g[V0][0] = mask & c.g[T1][0];
            if kind != 0 {
                c.cycles += 2;
                if c.g[V0][0] == 0 {
                    skip_cycle = false;
                } else {
                    // The track of the id's lowest set bit.
                    c.cycles += 2;
                    c.g[A2][0] = 0;
                    c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[T3][0] as u32));
                    loop {
                        c.cycles += 4;
                        let bit = ((c.g[A0][0] as u32) >> ((c.g[A2][0] as u32) & 31)) & 1;
                        c.g[V0][0] = bit as u64;
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                        if bit != 0 {
                            c.cycles += 2;
                            c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                            ret(c);
                            return;
                        }
                        c.cycles += 3;
                        let more = c.g[A2][0] < TRACK_BITS;
                        c.g[V0][0] = more as u64;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003d_52f0);
                        }
                    }
                }
            }
        }
        if skip_cycle {
            c.cycles += 1;
        }
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
        c.cycles += 4;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(SLOT_SIZE));
        let more = c.g[T0][0] < SLOT_COUNT;
        c.g[V0][0] = more as u64;
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(SLOT_SIZE));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003d_52c8);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    ret(c);
}

// ---------------------------------------------------------------------------
// PATH_ServiceQueues (0x3d1e80)

/// `PATH_RemoveEvent` (0x3d1ba8) and `PATH_ServiceQueue` (0x3d1d28).
pub(crate) const PATH_REMOVE_EVENT: Callee = Callee {
    addr: 0x003d_1ba8,
    f: ee::s14::PATH_RemoveEvent,
};

pub(crate) const PATH_SERVICE_QUEUE: Callee = Callee {
    addr: 0x003d_1d28,
    f: ee::s14::PATH_ServiceQueue,
};

/// The PATH state pointer and the PATH clock (ms), both 0x450000-relative.
pub(crate) const PATH_STATE_OFF: u32 = 0xbe0;

pub(crate) const PATH_TIME_OFF: u32 = 0xbc8;

/// PATH state: the event queue heads (16) and their count.
pub(crate) const PATH_QUEUES: u32 = 0xdc;

pub(crate) const PATH_QUEUE_COUNT: u32 = 0x91c;

pub(crate) const PATH_MAX_QUEUES: i64 = 16;

/// Queue head: start time, lifetime (0 = forever), packed action indices
/// (bits 16..21 = end, 22..27 = next).
pub(crate) const QUEUE_START: u32 = 0x0;

pub(crate) const QUEUE_LIFETIME: u32 = 0x4;

pub(crate) const QUEUE_INDICES: u32 = 0xc;

/// Resume points (dispatcher loop heads).
pub(crate) const SERVICE_QUEUES_ENTRY: u32 = 0x003d_1e80;

pub(crate) const SERVICE_QUEUES_NEXT: u32 = 0x003d_1f1c;

pub(crate) const SERVICE_QUEUES_QUEUE: u32 = 0x003d_1ec0;

/// The stages of PATH_ServiceQueues' loop (the original's block heads).
#[derive(Clone, Copy)]
pub(crate) enum ServiceStage {
    /// 0x3d1f20: past the last queue slot? (s0 = index)
    Check,
    /// 0x3d1ec0: one live queue (a0 = head, v0 = its packed indices).
    Queue,
    /// 0x3d1f1c: index + 1.
    Next,
}

/// `int PATH_ServiceQueues()` (0x3d1e80): runs every PATH event queue: a
/// queue with actions left (and not expired) is serviced, one that is done
/// or expired is removed (the later ones shift down); slots past the queue
/// count are cleared. Returns the number of actions run. Dispatcher-shaped:
/// an unwinding event at a back edge returns with `pc` at the loop head
/// (0x3d1f1c or 0x3d1ec0), where the trampoline re-enters it with the loop
/// state in s0..s5 (and a0/v0).
pub fn path_service_queues(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x70 };
        let mut stage = match c.pc {
            SERVICE_QUEUES_ENTRY => {
                c.cycles += 15;
                Frame::open(c, 0x70);
                frame.save(c, S0, 0x60);
                frame.save(c, S1, 0x50);
                c.g[S0][0] = 0; // queue index
                frame.save(c, S2, 0x40);
                c.g[S1][0] = 0; // actions run
                frame.save(c, S3, 0x30);
                c.g[S2][0] = 1;
                frame.save(c, S4, 0x20);
                c.g[S3][0] = 0x45_0000;
                frame.save(c, S5, 0x10);
                c.g[S4][0] = 3;
                frame.save_ra(c, 0);
                c.g[S5][0] = 0x45_0000;
                ServiceStage::Check
            }
            SERVICE_QUEUES_NEXT => ServiceStage::Next,
            SERVICE_QUEUES_QUEUE => ServiceStage::Queue,
            pc => {
                c.jump_out(pc, SERVICE_QUEUES_ENTRY, 0x003d_1f80);
                return Ok(());
            }
        };
        loop {
            match stage {
                ServiceStage::Check => {
                    c.cycles += 3;
                    let index = c.g[S0][0] as i64;
                    c.g[V0][0] = (index < PATH_MAX_QUEUES) as u64;
                    let state = Addr(r32(c, S5).wrapping_sub(PATH_STATE_OFF)).lw(c);
                    c.g[V1][0] = state;
                    if index >= PATH_MAX_QUEUES {
                        break;
                    }
                    c.cycles += 4;
                    let count = Addr((state as u32).wrapping_add(PATH_QUEUE_COUNT)).lw(c);
                    let live = index < count as i64;
                    let slot = (state as u32).wrapping_add((index as u32) << 2);
                    c.g[V0][0] = sx(slot);
                    if !live {
                        // A slot past the count: clear it.
                        c.cycles += 3;
                        Addr(slot).off(PATH_QUEUES).sw(c, 0);
                        if crate::events::back_edge_unwinds(c, SERVICE_QUEUES_NEXT) {
                            return Err(crate::call::Unwound);
                        }
                        stage = ServiceStage::Next;
                        continue;
                    }
                    c.cycles += 4;
                    let head = Addr(slot).off(PATH_QUEUES).lw(c);
                    c.g[A0][0] = head;
                    if head == 0 {
                        break;
                    }
                    c.g[V0][0] = Addr(head as u32).off(QUEUE_INDICES).lw(c);
                    if crate::events::back_edge_unwinds(c, SERVICE_QUEUES_QUEUE) {
                        return Err(crate::call::Unwound);
                    }
                    stage = ServiceStage::Queue;
                }
                ServiceStage::Queue => {
                    // Remove (a1 != 0) when no actions are left, or when the
                    // lifetime ran out; else service.
                    c.cycles += 9;
                    let head = Addr(r32(c, A0));
                    let mut remove = c.g[S4][0];
                    let lifetime = head.off(QUEUE_LIFETIME).lw(c);
                    c.g[A2][0] = lifetime;
                    let indices = r32(c, V0);
                    let end = sx(indices >> 16) & 0x3f;
                    let next = sx(indices >> 22) & 0x3f;
                    c.g[V1][0] = end;
                    let pending = next < end;
                    c.g[V0][0] = pending as u64;
                    if pending {
                        remove = 0;
                    }
                    if lifetime != 0 {
                        c.cycles += 5;
                        let start = head.off(QUEUE_START).lw(c);
                        let now = Addr(r32(c, S3).wrapping_sub(PATH_TIME_OFF)).lw(c);
                        c.g[V1][0] = now;
                        let expiry = sx((start as u32).wrapping_add(lifetime as u32));
                        let expired = expiry < now;
                        c.g[V0][0] = expired as u64;
                        if expired {
                            remove = c.g[S2][0];
                        }
                    }
                    c.g[A1][0] = remove;
                    c.cycles += 2;
                    if remove != 0 {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S0][0];
                        PATH_REMOVE_EVENT.call(c, 0x003d_1f08)?;
                        // The later queues moved down: stay on this index.
                        c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S0][0];
                        PATH_SERVICE_QUEUE.call(c, 0x003d_1f18)?;
                        c.g[S1][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
                    }
                    stage = ServiceStage::Next;
                }
                ServiceStage::Next => {
                    c.cycles += 1;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    stage = ServiceStage::Check;
                }
            }
        }
        c.cycles += 10;
        c.g[V0][0] = c.g[S1][0];
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

// -- 0x003d54e8 PATH_UpdateTick42 --------------------------------------------

/// `void PATH_UpdateTick42(void)` (0x3d54e8): the music-path timer tick,
/// `PATH_Update('B')`. Frame 0x10 (ra).
pub fn path_update_tick42(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x42;
        Callee {
            addr: 0x003d_5128,
            f: ee::s14::f_003d5128,
        }
        .call(c, 0x003d_54f8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// MusicPlayer_PathCmdA (0x2b3c28, bankmonitor.cpp)

/// MusicPlayer fields: the current song instance, the "playing" flag.
pub(crate) const MP_SONG: u32 = 0x408;

pub(crate) const MP_ACTIVE: u32 = 0x418;

/// `SongInstance_PathCmdA(SongInstance*)`.
pub(crate) const MP_SONG_INSTANCE_PATH_CMD_A: Callee = Callee {
    addr: 0x002b_2120,
    f: ee::s08::SongInstance_PathCmdA,
};

/// `void MusicPlayer_PathCmdA(MusicPlayer* self)` (0x2b3c28): when the
/// player is active and has a song instance, forwards PATH command A to it.
pub fn music_player_path_cmd_a(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[V0][0] = Addr(this.wrapping_add(MP_ACTIVE)).lw(c);
        frame.restore_ra(c, 0);
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = Addr(this.wrapping_add(MP_SONG)).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                MP_SONG_INSTANCE_PATH_CMD_A.call(c, 0x002b_3c50)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PATH_PlatGetQueuedEntryValue (0x2aeaa8, bankmonitor.cpp)

/// The 24 sound-bank monitor slots (`BankMonitor*[24]`).
pub(crate) const PQ_MON_TABLE: u32 = 0x004d_3e98;

/// BankMonitor fields: whole-bank value, "status stale" flag, sub-entry
/// count (`s8`), sub-entry array (12-byte `{?, handle, value}`).
pub(crate) const PQ_MON_VALUE: u32 = 0x54;

pub(crate) const PQ_MON_STALE: u32 = 0x65;

pub(crate) const PQ_MON_SUB_COUNT: u32 = 0x66;

pub(crate) const PQ_MON_SUBS: u32 = 0x6c;

pub(crate) const PQ_SUB_SIZE: u32 = 0xc;

pub(crate) const PQ_SUB_HANDLE: u32 = 0x4;

pub(crate) const PQ_SUB_VALUE: u32 = 0x8;

/// `PATH_PlatRefreshStatus(int reset)`.
pub(crate) const PQ_PATH_PLAT_REFRESH_STATUS: Callee = Callee {
    addr: 0x002a_efa8,
    f: ee::s08::PATH_PlatRefreshStatus,
};

/// `int PATH_PlatGetQueuedEntryValue(int slot, int sub)` (0x2aeaa8): looks
/// up monitor `slot` (refreshing every monitor first when its status is
/// stale); `sub < 0` returns the monitor's own value, otherwise sub-entry
/// `sub`'s value, or -8 when `sub` is out of range or the entry has no
/// handle.
pub fn path_plat_get_queued_entry_value(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        let index = (c.g[A0][0] as u32) << 2;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(PQ_MON_TABLE);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = sx(index.wrapping_add(PQ_MON_TABLE));
        c.g[S0][0] = Addr(index.wrapping_add(PQ_MON_TABLE)).lw(c);
        c.g[S1][0] = c.g[A1][0];
        if c.g[S0][0] != 0 {
            c.cycles += 3;
            let mon = r32(c, S0);
            c.g[V0][0] = Addr(mon.wrapping_add(PQ_MON_STALE)).lb(c);
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[A0][0] = 0;
                PQ_PATH_PLAT_REFRESH_STATUS.call(c, 0x002a_eae8)?;
            }
        }
        c.cycles += 2;
        let mon = r32(c, S0);
        let sub = c.g[S1][0];
        if (sub as i64) >= 0 {
            c.g[V0][0] = Addr(mon.wrapping_add(PQ_MON_SUB_COUNT)).lb(c);
            c.cycles += 3;
            let in_range = ((sub as i64) < (c.g[V0][0] as i64)) as u64;
            c.g[V0][0] = PQ_SUB_SIZE as u64;
            if in_range == 0 {
                c.cycles += 1;
                c.g[V0][0] = sx(-8i32 as u32);
            } else {
                c.cycles += 6;
                c.g[A0][0] = Addr(mon.wrapping_add(PQ_MON_SUBS)).lw(c);
                let off = crate::abi::mult(c, sub as u32, PQ_SUB_SIZE);
                let entry = off.wrapping_add(c.g[A0][0] as u32);
                c.g[V0][0] = sx(entry);
                c.g[V1][0] = Addr(entry.wrapping_add(PQ_SUB_HANDLE)).lw(c);
                if (c.g[V1][0] as i64) < 0 {
                    c.g[V0][0] = sx(-8i32 as u32);
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = Addr(entry.wrapping_add(PQ_SUB_VALUE)).lw(c);
                }
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = Addr(mon.wrapping_add(PQ_MON_VALUE)).lw(c);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// MusicPlayer_SetVolumePercent (0x2b3d10, bankmonitor.cpp)

/// `SongInstance_SetVolumePercent(SongInstance*, s8 percent)`.
pub(crate) const MP_SONG_INSTANCE_SET_VOLUME_PERCENT: Callee = Callee {
    addr: 0x002b_23e8,
    f: ee::s08::SongInstance_SetVolumePercent,
};

/// `void MusicPlayer_SetVolumePercent(MusicPlayer* self, s8 percent)`
/// (0x2b3d10): when the player is active and has a song instance, sets the
/// song's PATH volume percentage.
pub fn music_player_set_volume_percent(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[V0][0] = Addr(this.wrapping_add(MP_ACTIVE)).lw(c);
        c.g[A1][0] = sx(c.g[A1][0] as u8 as i8 as i32 as u32);
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = Addr(this.wrapping_add(MP_SONG)).lw(c);
            frame.restore_ra(c, 0);
            if c.g[A0][0] == 0 {
                c.cycles += 2;
                frame.close(c);
                return Ok(());
            }
            c.cycles += 2;
            MP_SONG_INSTANCE_SET_VOLUME_PERCENT.call(c, 0x002b_3d3c)?;
        }
        c.cycles += 1;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BankMonitor_AddInstance (0x2acfe0, bankmonitor.cpp)

/// BankMonitor fields: current sound type, per-type tables (inline arrays
/// at 0x24 and 0x40, pointers to per-type arrays elsewhere), the owner's
/// vtable, the busy counter and the instance array (64 x 0xc0 bytes).
pub(crate) const OC264_BM_KIND: u32 = 0x1c;

pub(crate) const OC264_BM_SOURCES: u32 = 0x24;

pub(crate) const OC264_BM_TABLE_FADE: u32 = 0x34;

pub(crate) const OC264_BM_TABLE_SLOT: u32 = 0x3c;

pub(crate) const OC264_BM_PARAMS: u32 = 0x40;

pub(crate) const OC264_BM_TABLE_80: u32 = 0x60;

pub(crate) const OC264_BM_TABLE_84: u32 = 0x64;

pub(crate) const OC264_BM_TABLE_88: u32 = 0x68;

pub(crate) const OC264_BM_TABLE_8C: u32 = 0x6c;

pub(crate) const OC264_BM_TABLE_94: u32 = 0x74;

pub(crate) const OC264_BM_TABLE_98: u32 = 0x78;

pub(crate) const OC264_BM_TABLE_9C: u32 = 0x7c;

pub(crate) const OC264_BM_TABLE_A0: u32 = 0x80;

pub(crate) const OC264_BM_TABLE_A4: u32 = 0x84;

pub(crate) const OC264_BM_ADD_COUNT: u32 = 0x8dc;

pub(crate) const OC264_BM_INSTANCES: u32 = 0x8ec;

pub(crate) const OC264_BM_INSTANCE_SIZE: u64 = 0xc0;

/// BankMonitorInstance fields written here.
pub(crate) const OC264_BMI_TYPE: u32 = 0x0;

pub(crate) const OC264_BMI_ID: u32 = 0x4;

pub(crate) const OC264_BMI_ACTIVE: u32 = 0x8;

pub(crate) const OC264_BMI_FLAGS: u32 = 0x58;

pub(crate) const OC264_BMI_SLOT: u32 = 0x60;

pub(crate) const OC264_BMI_VOLUME: u32 = 0x64;

pub(crate) const OC264_BMI_SOURCE: u32 = 0x68;

pub(crate) const OC264_BMI_SOURCE_COPY: u32 = 0x70;

pub(crate) const OC264_BMI_PARAMS: u32 = 0xa8;

/// "no free instance" message (`Audio_DebugPrintf` format string).
pub(crate) const OC264_BM_FULL_MSG: u32 = 0x0048_3250;

pub(crate) const OC264_SND_STOP: u32 = 0x003b_89e0;

pub(crate) const OC264_SNDMIDI_STOP: u32 = 0x003b_9920;

pub(crate) const OC264_BM_FIND_FREE_INSTANCE: Callee = Callee {
    addr: 0x002a_cf60,
    f: ee::s08::BankMonitor_FindFreeInstance,
};

pub(crate) const OC264_BMI_RESET: Callee = Callee {
    addr: 0x002a_bb38,
    f: ee::s08::BankMonitorInstance_Reset,
};

pub(crate) const OC264_BMI_UPDATE_DISTANCE: Callee = Callee {
    addr: 0x002a_bd48,
    f: ee::s08::BankMonitorInstance_UpdateDistance,
};

pub(crate) const OC264_AUDIO_DEBUG_PRINTF: Callee = Callee {
    addr: 0x002b_9100,
    f: ee::s08::Audio_DebugPrintf,
};

/// `&table[self->kind]` for the per-type table pointer at `self + field`
/// (the pointer is loaded into `base_reg`, the slot address into v0).
pub(crate) fn oc264_bm_kind_slot(c: &mut Cpu, this: u32, base_reg: usize, field: u32) -> u32 {
    let kind = Addr(this.wrapping_add(OC264_BM_KIND)).lw(c) as u32;
    let base = Addr(this.wrapping_add(field)).lw(c);
    c.g[base_reg][0] = base;
    let slot = (kind << 2).wrapping_add(base as u32);
    c.g[V0][0] = sx(slot);
    slot
}

/// `table[self->kind]` for the per-type table pointer at `self + field`.
pub(crate) fn oc264_bm_kind_word(c: &mut Cpu, this: u32, base_reg: usize, field: u32) -> u64 {
    let slot = oc264_bm_kind_slot(c, this, base_reg, field);
    Addr(slot).lw(c)
}

/// `&self->sources[self->kind]` (inline array at 0x24, base in a3).
pub(crate) fn oc264_bm_source_slot(c: &mut Cpu, this: u32) -> u32 {
    let kind = Addr(this.wrapping_add(OC264_BM_KIND)).lw(c) as u32;
    let slot = r32(c, A3).wrapping_add(kind << 2);
    c.g[V0][0] = sx(slot);
    slot
}

/// `BankMonitorInstance* BankMonitor_AddInstance(BankMonitor* self, int id,
/// int volume, int type)` (0x2acfe0): takes a free instance slot
/// (`BankMonitor_FindFreeInstance`), resets it and fills it from the
/// monitor's per-sound-type tables (slot, source position, params, fades,
/// pitch/volume curves), then measures its distance when it has a source.
/// With no free slot it prints a warning and stops the sound `id` that was
/// to be tracked (`SND_Stop` for type 1, `SNDMIDI_Stop` for type 2).
/// Returns the instance or 0.
pub fn bank_monitor_add_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, S4, 0x30);
        c.g[S3][0] = c.g[A1][0];
        frame.save_ra(c, 0x20);
        c.g[S4][0] = c.g[A2][0];
        c.g[S1][0] = 0;
        let this = r32(c, S0);
        let count = Addr(this.wrapping_add(OC264_BM_ADD_COUNT));
        let n = sx((count.lw(c) as u32).wrapping_add(1));
        c.g[V0][0] = n;
        count.sw(c, n);
        c.g[A1][0] = count.lw(c);
        OC264_BM_FIND_FREE_INSTANCE.call(c, 0x002a_d028)?;
        c.g[A1][0] = c.g[V0][0];
        let none = (c.g[A1][0] as i64) < 0;
        c.g[V0][0] = OC264_BM_INSTANCE_SIZE;
        if !none {
            c.cycles += 66;
            c.g[V1][0] = Addr(this.wrapping_add(OC264_BM_INSTANCES)).lw(c);
            c.mult(0, V0, A1, V0);
            c.g[S1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[A0][0] = c.g[S1][0];
            OC264_BMI_RESET.call(c, 0x002a_d048)?;
            let inst = r32(c, S1);
            let at = |off: u32| Addr(inst.wrapping_add(off));
            let (kind, id, volume) = (c.g[S2][0], c.g[S3][0], c.g[S4][0]);
            at(OC264_BMI_TYPE).sw(c, kind);
            c.g[A1][0] = 0xffff_ffff_0000_0000;
            at(OC264_BMI_ID).sw(c, id);
            c.g[A0][0] &= c.g[A1][0];
            c.g[A3][0] = sx(this.wrapping_add(OC264_BM_SOURCES));
            c.g[A2][0] = 0x0000_0000_ffff_ffff;

            // Bank slot and volume.
            let slot = oc264_bm_kind_word(c, this, V1, OC264_BM_TABLE_SLOT);
            c.g[A1][0] = slot;
            at(OC264_BMI_VOLUME).sw(c, volume);
            at(OC264_BMI_SLOT).sw(c, slot);
            // Source position pointer.
            let p = oc264_bm_source_slot(c, this);
            let source = Addr(p).lw(c);
            c.g[V1][0] = source;
            at(OC264_BMI_SOURCE).sw(c, source);
            // The type's 8-byte parameter pair, packed into a doubleword and
            // stored unaligned through the stack.
            let kind_ix = Addr(this.wrapping_add(OC264_BM_KIND)).lw(c) as u32;
            let pair = this.wrapping_add(kind_ix << 3);
            c.g[V0][0] = sx(pair);
            c.g[A1][0] = Addr(pair.wrapping_add(OC264_BM_PARAMS)).lwu(c);
            c.g[V1][0] = Addr(pair.wrapping_add(OC264_BM_PARAMS + 4)).lwu(c);
            c.g[A0][0] |= c.g[A1][0];
            c.g[V1][0] <<= 32;
            c.g[A0][0] &= c.g[A2][0];
            c.g[A0][0] |= c.g[V1][0];
            c.g[V0][0] = (c.g[A0][0] << 32) >> 32;
            c.g[A0][0] >>= 32;
            let (lo, hi) = (c.g[V0][0], c.g[A0][0]);
            Addr(frame.at(c, 0x10)).sw(c, lo);
            Addr(frame.at(c, 0x14)).sw(c, hi);
            let s = sp(c);
            c.ldl(V0, s.wrapping_add(0x17));
            c.ldr(V0, s.wrapping_add(0x10));
            c.sdl(V0, s.wrapping_add(0x7));
            c.sdr(V0, s);
            c.ldl(V0, s.wrapping_add(0x7));
            c.ldr(V0, s);
            c.sdl(V0, inst.wrapping_add(OC264_BMI_PARAMS + 7));
            c.sdr(V0, inst.wrapping_add(OC264_BMI_PARAMS));
            let w = oc264_bm_kind_word(c, this, A0, OC264_BM_TABLE_80);
            c.g[V1][0] = w;
            at(0x80).sw(c, w);
            let w = oc264_bm_kind_word(c, this, A0, OC264_BM_TABLE_84);
            c.g[V1][0] = w;
            at(0x84).sw(c, w);
            // A fade table entry replaces the source with a private copy.
            let fade = oc264_bm_kind_word(c, this, A0, OC264_BM_TABLE_FADE);
            c.g[V1][0] = fade;
            c.g[A0][0] = sx(inst.wrapping_add(OC264_BMI_SOURCE_COPY));
            if fade != 0 {
                c.cycles += 4;
                c.g[V0][0] = at(OC264_BMI_SOURCE).lw(c);
                let source = r32(c, V0);
                c.lq(V1, source);
                let copy = c.g[A0][0];
                at(OC264_BMI_SOURCE).sw(c, copy);
                c.sq(V1, inst.wrapping_add(OC264_BMI_SOURCE_COPY));
            }

            c.cycles += 31;
            let p = oc264_bm_kind_slot(c, this, V1, OC264_BM_TABLE_88);
            c.g[A1][0] = at(OC264_BMI_FLAGS).lw(c);
            c.lwc1(0, p);
            c.swc1(0, inst.wrapping_add(0x88));
            let p = oc264_bm_kind_slot(c, this, V1, OC264_BM_TABLE_8C);
            c.lwc1(0, p);
            c.swc1(0, inst.wrapping_add(0x8c));
            let w = oc264_bm_kind_word(c, this, A0, OC264_BM_TABLE_94);
            c.g[V1][0] = w;
            at(0x94).sw(c, w);
            let w = oc264_bm_kind_word(c, this, A0, OC264_BM_TABLE_98);
            c.g[V1][0] = w;
            at(0x98).sw(c, w);
            let p = oc264_bm_kind_slot(c, this, V1, OC264_BM_TABLE_9C);
            let flagged = c.g[A1][0] != 0;
            let w = Addr(p).lw(c);
            c.g[V0][0] = w;
            if !flagged {
                c.cycles += 1;
                at(0x9c).sw(c, w);
            }

            c.cycles += 21;
            c.g[AT][0] = 0xffff_ffff_bf80_0000; // -1.0f
            c.mtc1(AT, 0);
            c.swc1(0, inst.wrapping_add(0x90));
            let w = oc264_bm_kind_word(c, this, A0, OC264_BM_TABLE_A0);
            c.g[V1][0] = w;
            at(0xa0).sw(c, w);
            let w = oc264_bm_kind_word(c, this, A0, OC264_BM_TABLE_A4);
            c.g[V1][0] = w;
            at(0xa4).sw(c, w);
            let p = oc264_bm_source_slot(c, this);
            c.g[V1][0] = Addr(p).lw(c);
            c.g[A0][0] = c.g[S1][0];
            if c.g[V1][0] != 0 {
                c.cycles += 2;
                c.g[A1][0] = c.g[S0][0];
                OC264_BMI_UPDATE_DISTANCE.call(c, 0x002a_d228)?;
            }
            c.cycles += 4;
            // active = id >= 0.
            let active = sx((!c.g[S3][0] as u32) >> 31);
            c.g[V0][0] = active;
            Addr(r32(c, S1).wrapping_add(OC264_BMI_ACTIVE)).sw(c, active);
        } else {
            c.cycles += 6;
            c.g[A0][0] = sx(OC264_BM_FULL_MSG);
            OC264_AUDIO_DEBUG_PRINTF.call(c, 0x002a_d244)?;
            let kind = c.g[S2][0];
            c.g[V0][0] = 2;
            if kind == 1 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S3][0];
                call_ptr(c, OC264_SND_STOP, 0x002a_d268)?;
            } else {
                c.cycles += 2;
                if kind == 2 {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, OC264_SNDMIDI_STOP, 0x002a_d278)?;
                } else {
                    c.cycles += 2;
                }
            }
        }
        c.cycles += 12;
        let count = Addr(r32(c, S0).wrapping_add(OC264_BM_ADD_COUNT));
        c.g[V1][0] = count.lw(c);
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S1, 0x60);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
        frame.restore(c, S2, 0x50);
        let left = c.g[V1][0];
        count.sw(c, left);
        frame.restore(c, S0, 0x70);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void SongInstance_SetVolumePercent(SongInstance* self, s8 percent)`
/// (0x2b23e8): when the song is playing (`+0x54` non-zero), set the music
/// tracks' volume through `PATH_SetVolumePercent(0x11000002, percent)`.
pub fn song_instance_set_volume_percent(c: &mut Cpu) {
    const PLAYING: u32 = 0x54;
    const MUSIC_TRACK_MASK: u64 = 0x1100_0002;
    const PATH_SET_VOLUME_PERCENT: Callee = Callee {
        addr: 0x003d_5508,
        f: ee::s14::f_003d5508,
    };
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(PLAYING).lw(c);
        c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = MUSIC_TRACK_MASK;
            PATH_SET_VOLUME_PERCENT.call(c, 0x002b_240c)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PATH_TrackGetQueuedCount

/// `PathTrack.flags`: bits 14..18 hold the number of queued entries.
pub(crate) const PATH_TRACK_FLAGS: u32 = 0x20;

/// `PathTrack.slot`: the PATH platform slot of the track.
pub(crate) const PATH_TRACK_SLOT: u32 = 0x54;

/// The queued-entry count field of a PATH track's flags word.
pub(crate) fn path_track_queued_slots(flags: u64) -> u64 {
    sx((flags as u32) >> 14) & 0x1f
}

/// `int PATH_TrackGetQueuedCount(PathTrack* t)` (0x3d4950): sum the
/// non-negative values of the track's queued entries
/// (`PATH_PlatGetQueuedEntryValue(t->slot, i)` for each of the
/// `(t->flags >> 14) & 31` entries; negative ones are errors and skipped).
pub fn path_track_get_queued_count(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0; // the sum
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0]; // the track
        let flags = Addr(c.g[S2][0] as u32).off(PATH_TRACK_FLAGS).lw(c);
        c.g[V0][0] = path_track_queued_slots(flags);
        c.g[S1][0] = 0; // the entry index
        if c.g[V0][0] != 0 {
            loop {
                c.cycles += 12;
                c.g[A0][0] = Addr(c.g[S2][0] as u32).off(PATH_TRACK_SLOT).lw(c);
                c.g[A1][0] = c.g[S1][0];
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                // PATH_PlatGetQueuedEntryValue
                call_ptr(c, 0x002a_eaa8, 0x003d_4990)?;
                let value = c.g[V0][0];
                let flags = Addr(c.g[S2][0] as u32).off(PATH_TRACK_FLAGS).lw(c);
                let negative = (value as i64) < 0;
                c.g[A0][0] = negative as u64;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(value as u32));
                c.g[V1][0] = (c.g[S1][0] < path_track_queued_slots(flags)) as u64;
                let more = c.g[V1][0] != 0;
                if !negative {
                    c.g[S0][0] = c.g[V0][0];
                }
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_4980);
                }
            }
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

// ---------------------------------------------------------------------------
// PATH_PlatGetTrackState.

/// `PATH_PlatTrack* D_004D3E98[]`: the platform track table.
pub(crate) const PATH_PLAT_TRACKS: u32 = 0x004d_3e98;

/// `PATH_PlatTrack::dirty` (s8): the status needs a refresh.
pub(crate) const PATH_PLAT_TRACK_DIRTY: u32 = 0x65;

/// `PATH_PlatTrack::state`.
pub(crate) const PATH_PLAT_TRACK_STATE: u32 = 0x58;

/// `int PATH_PlatGetTrackState(int idx)` (0x2aebd0): the state word of
/// platform track `idx`, refreshing the status first
/// (`PATH_PlatRefreshStatus(0)`) when the track is marked dirty; -8 for an
/// empty slot.
pub fn path_plat_get_track_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(PATH_PLAT_TRACKS));
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        if c.g[V0][0] == 0 {
            c.g[V0][0] = sx(-8i32 as u32);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(PATH_PLAT_TRACK_DIRTY)).lb(c);
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[A0][0] = 0;
                Callee {
                    addr: 0x002a_efa8,
                    f: ee::s08::f_002aefa8,
                }
                .call(c, 0x002a_ec0c)?;
            }
            // Re-read the slot (the refresh may replace the track).
            c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.cycles += 1;
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(PATH_PLAT_TRACK_STATE)).lw(c);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod path_f {

    /// `PATH_gNetwork` (the loaded path network), `lw 0x44f420`.
    pub const NETWORK: u32 = 0x0044_f420;

    /// Network: base of the packed node records.
    pub const NODE_BASE: u32 = 0x34;

    /// Network: node index -> record word offset (`i16[]`).
    pub const NODE_INDEX: u32 = 0x38;

    /// Network: per-track trackers (`slot` from the node header, + 0x58).
    pub const TRACKERS: u32 = 0x58;

    /// Tracker: the node it is queued on (`i16`).
    pub const TRACKER_NODE: u32 = 0x2c;

    /// Tracker: flags; low nibble = signed lane / remaining count.
    pub const TRACKER_LANE: u32 = 0x20;

    /// Node record: second word (branch count in bits 12-16).
    pub const NODE_WORD1: u32 = 0x4;

    /// Node record: the branch list (`{i8 lo, i8 hi, i16 target}`).
    pub const BRANCHES: u32 = 0x10;
}

pub(crate) mod plat_queue_mon {

    /// Stream handle (`SNDSTRM_*` handle; the bank voice in memory mode).
    pub const HANDLE: u32 = 0x0;

    pub const POS: u32 = 0x4;

    /// Optional file-offset base for `SNDSTRM_QueueFileOffset`, and its limit.
    pub const OFFSET_BASE: u32 = 0x28;

    pub const OFFSET_LIMIT: u32 = 0x2c;

    pub const VOLUME: u32 = 0x30;

    pub const IS_MEM: u32 = 0x64;

    pub const DIRTY: u32 = 0x65;

    pub const COUNT: u32 = 0x66;

    pub const SUBS: u32 = 0x6c;
}

pub(crate) mod plat_queue_callee {
    use crate::call::Callee;
    use ssx3_ee as ee;

    pub const SET_VOLUME: Callee = Callee {
        addr: 0x002a_f6c0,
        f: ee::s08::PATH_PlatSetVolume,
    };

    pub const FIND_BANK_SLOT: Callee = Callee {
        addr: 0x002a_e230,
        f: ee::s08::PATH_PlatFindBankSlot,
    };

    pub const GET_TIME_MS: Callee = Callee {
        addr: 0x002a_f370,
        f: ee::s08::PATH_PlatGetTimeMs,
    };

    pub const SND_LOCK: u32 = 0x003b_58a0;

    pub const SND_PLAY: u32 = 0x003b_6820;

    pub const SND_UNLOCK: u32 = 0x003b_58d8;

    pub const SNDSTRM_STOP: u32 = 0x003b_78f8;

    pub const SNDSTRM_QUEUE_FILE_OFFSET: u32 = 0x003b_7e10;

    pub const SNDSTRM_QUEUE: u32 = 0x003b_78d8;
}

/// `int PATH_SelectBranch(int node, int lane)` (0x3d37a0), EA path library:
/// pick the branch out of path node `node` whose lane range `[lo, hi]`
/// contains `lane`, else the branch whose range is closest (-1 for a node
/// without branches). If the node's tracker is queued on this node, its
/// own lane (low nibble of +0x20, sign-extended) replaces `lane`, and with a
/// positive node count it is consumed (decremented; at -1 the tracker is
/// released by setting its node to -1). A leaf; reproduces every scratch
/// register of the original.
pub fn path_select_branch(c: &mut Cpu) {
    use path_f::*;
    let sext4 = |v: u32| sx(((v << 28) as i32 >> 28) as u32);
    c.cycles += 19;
    c.g[V0][0] = 0x45_0000;
    c.g[T2][0] = c.g[A1][0]; // lane
    c.g[A2][0] = Addr(NETWORK).lw(c);
    c.g[V1][0] = sx(r32(c, A0) << 1);
    c.g[T0][0] = u64::MAX; // result: -1
    let net = r32(c, A2);
    c.g[A1][0] = Addr(net.wrapping_add(NODE_INDEX)).lw(c);
    c.g[A3][0] = Addr(net.wrapping_add(NODE_BASE)).lw(c);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
    c.g[V0][0] = Addr(r32(c, V1)).lh(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[A3][0] = sx(r32(c, A3).wrapping_add(r32(c, V0))); // node record
    c.g[V1][0] = Addr(r32(c, A3)).lw(c); // header
    c.g[V0][0] = sx(r32(c, V1) >> 14) & 0x7c;
    c.g[A2][0] = sx(net.wrapping_add(r32(c, V0)));
    c.g[A1][0] = Addr(r32(c, A2).wrapping_add(TRACKERS)).lw(c); // tracker
    c.g[V0][0] = Addr(r32(c, A1).wrapping_add(TRACKER_NODE)).lh(c);
    c.g[T1][0] = sx(r32(c, A3).wrapping_add(BRANCHES));
    if c.g[V0][0] == c.g[A0][0] {
        // The tracker sits on this node: its lane decides.
        c.cycles += 6;
        let tracker = r32(c, A1);
        c.g[A0][0] = Addr(tracker.wrapping_add(TRACKER_LANE)).lw(c);
        c.g[V1][0] = sx(((r32(c, V1) as i32) >> 28) as u32); // node count
        c.g[V0][0] = sext4(r32(c, A0));
        c.g[T2][0] = c.g[V0][0] & 0x7f;
        if c.g[V1][0] as i64 > 0 {
            c.cycles += 9;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
            c.g[V1][0] = c.g[A0][0] & 0xffff_ffff_ffff_fff0;
            c.g[A0][0] = c.g[V0][0] & 0xf;
            c.g[V1][0] |= c.g[A0][0];
            c.g[V0][0] = sext4(r32(c, V0));
            let lane = c.g[V1][0];
            Addr(tracker.wrapping_add(TRACKER_LANE)).sw(c, lane);
            if c.g[V0][0] == c.g[T0][0] {
                c.cycles += 1;
                let t0 = c.g[T0][0];
                Addr(tracker.wrapping_add(TRACKER_NODE)).sh(c, t0);
            }
        }
    }
    // First branch whose [lo, hi] contains the lane.
    c.cycles += 3;
    c.g[A0][0] = 0;
    c.g[A3][0] = Addr(r32(c, A3).wrapping_add(NODE_WORD1)).lw(c);
    loop {
        c.cycles += 5;
        c.g[V0][0] = sx(r32(c, A3) >> 12);
        c.g[A1][0] = c.g[V0][0] & 0x1f; // branch count
        let inside = c.g[A0][0] < c.g[A1][0];
        c.g[V1][0] = sx(r32(c, A0) << 2);
        if !inside {
            break;
        }
        c.cycles += 5;
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, T1)));
        let entry = r32(c, V1);
        c.g[V0][0] = Addr(entry).lb(c);
        c.g[V0][0] = ((c.g[T2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            c.cycles += 4;
            c.g[V0][0] = Addr(entry.wrapping_add(1)).lb(c);
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[T2][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[T0][0] = Addr(entry.wrapping_add(2)).lh(c);
                break;
            }
        }
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
        if event_due(c) {
            deliver_event_here(c, 0x003d_3838);
        }
    }
    c.cycles += 2;
    if (c.g[T0][0] as i64) < 0 {
        // None contains it: the branch with the nearest end.
        c.cycles += 2;
        c.g[T3][0] = c.g[T1][0];
        if c.g[A1][0] as i64 <= 0 {
            c.cycles += 2;
            c.g[T0][0] = u64::MAX;
        } else {
            c.cycles += 2;
            c.g[T5][0] = 0x7f; // best distance
            if c.g[A1][0] != 0 {
                c.cycles += 3;
                c.g[T1][0] = c.g[A1][0];
                c.g[T4][0] = u64::MAX;
                c.g[T0][0] = c.g[T3][0];
                loop {
                    c.cycles += 16;
                    let entry = r32(c, T0);
                    c.g[A1][0] = Addr(entry).lb(c);
                    c.g[A0][0] = Addr(entry.wrapping_add(1)).lb(c);
                    c.g[A1][0] = sx(r32(c, A1).wrapping_sub(r32(c, T2)));
                    c.g[A0][0] = sx(r32(c, A0).wrapping_sub(r32(c, T2)));
                    c.g[V0][0] = ((c.g[T4][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    c.g[V1][0] = ((c.g[T4][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    c.g[A3][0] = sx(0u32.wrapping_sub(r32(c, A1)));
                    c.g[A2][0] = sx(0u32.wrapping_sub(r32(c, A0)));
                    if c.g[V0][0] == 0 {
                        c.g[A1][0] = c.g[A3][0];
                    }
                    if c.g[V1][0] == 0 {
                        c.g[A0][0] = c.g[A2][0];
                    }
                    c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    if c.g[V0][0] == 0 {
                        c.g[A1][0] = c.g[A0][0];
                    }
                    c.g[V0][0] = c.g[A1][0]; // distance
                    c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[T5][0] as i64)) as u64;
                    c.g[T1][0] = sx(r32(c, T1).wrapping_sub(1));
                    if c.g[V1][0] != 0 {
                        c.cycles += 2;
                        c.g[T5][0] = c.g[V0][0];
                        c.g[T3][0] = c.g[T0][0];
                    }
                    c.cycles += 2;
                    let more = c.g[T1][0] != 0;
                    c.g[T0][0] = sx(r32(c, T0).wrapping_add(4));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003d_38a0);
                    }
                }
            }
            c.cycles += 1;
            c.g[T0][0] = Addr(r32(c, T3).wrapping_add(2)).lhu(c);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[T0][0];
    ret(c);
}

// ---------------------------------------------------------------------------
// BankMonitor_FindFreeInstance (0x2acf60)

/// BankMonitor: the instance array (64 entries of 0xc0 bytes).
pub(crate) const BANK_MONITOR_INSTANCES: u32 = 0x8ec;

pub(crate) const BANK_MONITOR_INSTANCE_SIZE: u32 = 0xc0;

pub(crate) const BANK_MONITOR_INSTANCE_COUNT: i64 = 64;

pub(crate) const BANK_MONITOR_POLL_FREE: Callee = Callee {
    addr: 0x002a_bb80,
    f: ee::s08::BankMonitorInstance_PollFree,
};

/// `int BankMonitor::FindFreeInstance(BankMonitor* this, int n)`
/// (0x2acf60): the index of the n-th free sound-bank instance (polling each
/// with `BankMonitorInstance_PollFree`, which also recycles finished ones),
/// or -1. The loop state lives in s0 (n left), s1 (index), s2 (byte
/// offset), s3 (this), as in the original.
pub fn bank_monitor_find_free_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A0][0];
        loop {
            c.cycles += 5;
            let this = r32(c, S3);
            let base = Addr(this.wrapping_add(BANK_MONITOR_INSTANCES)).lw(c) as u32;
            c.g[A0][0] = sx(base.wrapping_add(r32(c, S2)));
            BANK_MONITOR_POLL_FREE.call(c, 0x002a_cf94)?;
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
                if c.g[S0][0] == 0 {
                    // Found the n-th free one.
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S1][0];
                    break;
                }
            }
            c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < BANK_MONITOR_INSTANCE_COUNT) as u64;
            c.g[S2][0] = sx(r32(c, S2).wrapping_add(BANK_MONITOR_INSTANCE_SIZE));
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[V0][0] = u64::MAX;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002a_cf88);
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
// PATH_UpdateTick20 (0x3d54c8)

/// `void PATH_UpdateTick20(void)` (0x3d54c8): the music-path timer tick,
/// `PATH_Update(0x20)`. Frame 0x10 (ra).
pub fn path_update_tick20(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x20;
        Callee {
            addr: 0x003d_5128,
            f: ee::s14::PATH_Update,
        }
        .call(c, 0x003d_54d8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PATH_PlatQueue (0x2ae7a8)

/// The track monitor table (`D_004D3E98`, 24 pointers) and the bank-slot
/// table (`D_004D3EF8`, 24 entries of 0x28 bytes) of `PATH_PlatQueue`.
pub(crate) const PLAT_QUEUE_TRACKS: u32 = 0x004d_3e98;

pub(crate) const PLAT_QUEUE_BANK_SLOTS: u32 = 0x004d_3ef8;

/// `int PATH_PlatQueue(int track, a1, char* name, u32 sectorInfo, int tag,
/// signed char volume, int pos, int restart)` (0x2ae7a8): queue a track on
/// its monitor slot. A non-negative `volume` (t1) is stored and applied
/// first. In-memory slots look up the bank slot, play the patch under the
/// sound lock and stamp the bank slot's time; streaming slots optionally stop
/// the stream (`restart`, t3), clear the sub-stream ids, then queue the file
/// (at a file offset when the slot has one and the position is below its
/// limit). On success the sub-stream list is shifted up by one and the new
/// handle and `tag` (t0) go in front; the slot is marked dirty. Returns the
/// handle (or -1 when no bank slot exists).
pub fn path_plat_queue(c: &mut Cpu) {
    use plat_queue_callee as callee;
    use plat_queue_mon as mon;
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = 0x4d_0000;
        frame.save(c, S2, 0x70);
        c.g[V0][0] = sx(PLAT_QUEUE_TRACKS);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x80);
        frame.save(c, S3, 0x60);
        c.g[V1][0] = sx(r32(c, S2) << 2);
        frame.save(c, S4, 0x50);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        frame.save(c, S5, 0x40);
        c.g[T1][0] = sx(r32(c, T1) << 24);
        frame.save(c, S6, 0x30);
        c.g[T1][0] = sx(((r32(c, T1) as i32) >> 24) as u32);
        frame.save(c, S7, 0x20);
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, FP, 0x10);
        c.g[S7][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S0, 0x90);
        c.g[FP][0] = c.g[T0][0]; // tag
        c.g[S0][0] = c.lw(r32(c, V1)); // the monitor slot
        c.g[S1][0] = c.g[T2][0]; // pos
        c.g[S5][0] = c.g[T3][0]; // restart
        c.g[S4][0] = u64::MAX; // bank: -1
        if (c.g[T1][0] as i64) >= 0 {
            c.cycles += 3;
            c.g[A1][0] = c.g[T1][0];
            c.sb(r32(c, S0).wrapping_add(mon::VOLUME), c.g[T1][0]);
            callee::SET_VOLUME.call(c, 0x002a_e820)?;
        }
        'out: {
            c.cycles += 3;
            c.g[V0][0] = c.lb(r32(c, S0).wrapping_add(mon::IS_MEM));
            let in_memory = c.g[V0][0] != 0;
            c.g[V0][0] = ((c.g[S4][0] as i64) < (c.g[S1][0] as i64)) as u64;
            if in_memory {
                // Bank mode: find the bank slot and play the patch.
                c.cycles += 7;
                c.g[A0][0] = c.g[S2][0];
                if c.g[V0][0] == 0 {
                    c.g[S1][0] = 0; // pos < 0 -> 0
                }
                c.g[A1][0] = sx(r32(c, S3) >> 16);
                callee::FIND_BANK_SLOT.call(c, 0x002a_e83c)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[V1][0] = 0x28;
                if (c.g[S2][0] as i64) >= 0 {
                    c.cycles += 5;
                    c.g[V0][0] = 0x4d_0000;
                    c.mult(0, V1, S2, V1);
                    c.g[V0][0] = sx(PLAT_QUEUE_BANK_SLOTS);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                    c.g[S4][0] = c.lw(r32(c, V0).wrapping_add(0x4));
                }
                c.cycles += 2;
                c.g[V0][0] = u64::MAX;
                if (c.g[S4][0] as i64) < 0 {
                    break 'out; // no bank: return -1
                }
                c.cycles += 26;
                c.g[V0][0] = c.g[S3][0] & 0xffff;
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S1)));
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                c.sw(r32(c, S0).wrapping_add(mon::POS), c.g[V0][0]);
                call_ptr(c, callee::SND_LOCK, 0x002a_e878)?;
                c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(mon::POS));
                c.g[A0][0] = c.g[S4][0];
                c.g[A2][0] = sx(r32(c, S0).wrapping_add(mon::VOLUME));
                call_ptr(c, callee::SND_PLAY, 0x002a_e888)?;
                c.g[S1][0] = c.g[V0][0];
                call_ptr(c, callee::SND_UNLOCK, 0x002a_e890)?;
                callee::GET_TIME_MS.call(c, 0x002a_e898)?;
                c.g[A0][0] = 0x28;
                c.g[V1][0] = 0x4d_0000;
                c.mult(0, A0, S2, A0);
                c.g[V1][0] = sx(PLAT_QUEUE_BANK_SLOTS);
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
                c.sw(r32(c, V1).wrapping_add(0x24), c.g[V0][0]); // slot time
                let m = r32(c, S0);
                c.sw(m.wrapping_add(mon::HANDLE), c.g[S4][0]);
                c.g[V0][0] = c.lw(m.wrapping_add(mon::SUBS));
                c.sw(r32(c, V0).wrapping_add(0x4), c.g[S1][0]);
                c.g[V0][0] = 1;
                c.g[V1][0] = c.lw(m.wrapping_add(mon::SUBS));
                c.sw(r32(c, V1), c.g[FP][0]);
            } else {
                // Stream mode.
                c.cycles += 2;
                let restart = c.g[S5][0] != 0;
                c.g[S1][0] = sx(r32(c, S3) << 7);
                if restart {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(mon::HANDLE));
                    call_ptr(c, callee::SNDSTRM_STOP, 0x002a_e8dc)?;
                    c.g[V1][0] = c.lb(r32(c, S0).wrapping_add(mon::COUNT));
                    c.g[A0][0] = sx(r32(c, V1).wrapping_sub(1));
                    c.g[V0][0] = 0xc;
                    if (c.g[A0][0] as i64) > 0 {
                        // Clear the ids of sub-streams 1..count (from the top).
                        c.cycles += 3;
                        c.g[A2][0] = u64::MAX;
                        c.mult(0, A1, A0, V0);
                        loop {
                            c.cycles += 9;
                            let m = r32(c, S0);
                            c.g[V1][0] = c.lw(m.wrapping_add(mon::SUBS));
                            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(1));
                            c.g[V1][0] = sx(r32(c, A1).wrapping_add(r32(c, V1)));
                            c.sw(r32(c, V1).wrapping_sub(0x8), c.g[A2][0]);
                            c.g[V0][0] = c.lw(m.wrapping_add(mon::SUBS));
                            c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                            c.sw(r32(c, V0).wrapping_sub(0xc), c.g[A2][0]);
                            let more = (c.g[A0][0] as i64) > 0;
                            c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0xc));
                            if !more {
                                break;
                            }
                            if event_due(c) {
                                deliver_event_here(c, 0x002a_e8f8);
                            }
                        }
                    }
                }
                // Queue the file: at an offset when the slot has one and the
                // position is inside it.
                c.cycles += 3;
                let m = r32(c, S0);
                c.g[A2][0] = c.lw(m.wrapping_add(mon::OFFSET_BASE));
                let mut queued = false;
                if c.g[A2][0] == 0 {
                    c.g[A0][0] = c.lw(m.wrapping_add(mon::HANDLE));
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(m.wrapping_add(mon::OFFSET_LIMIT));
                    c.g[V0][0] = (c.g[S1][0] < c.g[V0][0]) as u64;
                    c.g[A3][0] = c.g[S1][0];
                    if c.g[V0][0] != 0 {
                        c.cycles += 5;
                        c.g[A0][0] = c.lw(m.wrapping_add(mon::HANDLE));
                        c.g[A1][0] = c.g[S6][0];
                        call_ptr(c, callee::SNDSTRM_QUEUE_FILE_OFFSET, 0x002a_e944)?;
                        c.g[S1][0] = c.g[V0][0];
                        queued = true;
                    } else {
                        c.cycles += 1;
                        c.g[A0][0] = c.lw(m.wrapping_add(mon::HANDLE));
                    }
                }
                if !queued {
                    c.cycles += 5;
                    c.g[A3][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S6][0];
                    c.g[A2][0] = c.g[S7][0];
                    call_ptr(c, callee::SNDSTRM_QUEUE, 0x002a_e960)?;
                    c.g[S1][0] = c.g[V0][0];
                }
                c.cycles += 2;
                c.g[V0][0] = 1;
                if (c.g[S1][0] as i64) >= 0 {
                    c.cycles += 4;
                    let m = r32(c, S0);
                    c.g[V0][0] = c.lb(m.wrapping_add(mon::COUNT));
                    c.g[A0][0] = sx(r32(c, V0).wrapping_sub(1));
                    c.g[V0][0] = 0xc;
                    if (c.g[A0][0] as i64) > 0 {
                        // Shift the 12-byte sub-stream records up by one.
                        c.cycles += 1;
                        c.mult(0, V1, A0, V0);
                        loop {
                            c.cycles += 11;
                            let m = r32(c, S0);
                            c.g[V0][0] = c.lw(m.wrapping_add(mon::SUBS));
                            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(1));
                            c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                            let rec = r32(c, V0);
                            c.ldl(A1, rec.wrapping_sub(0x5));
                            c.ldr(A1, rec.wrapping_sub(0xc));
                            c.g[A2][0] = c.lw(rec.wrapping_sub(0x4));
                            c.sdl(A1, rec.wrapping_add(0x7));
                            c.sdr(A1, rec);
                            c.sw(rec.wrapping_add(0x8), c.g[A2][0]);
                            let more = (c.g[A0][0] as i64) > 0;
                            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(0xc));
                            if !more {
                                break;
                            }
                            if event_due(c) {
                                deliver_event_here(c, 0x002a_e980);
                            }
                        }
                    }
                    c.cycles += 5;
                    let m = r32(c, S0);
                    c.g[V0][0] = c.lw(m.wrapping_add(mon::SUBS));
                    c.sw(r32(c, V0).wrapping_add(0x4), c.g[S1][0]);
                    c.g[V1][0] = c.lw(m.wrapping_add(mon::SUBS));
                    c.sw(r32(c, V1), c.g[FP][0]);
                    c.g[V0][0] = 1;
                }
            }
            // Mark the slot dirty and return the handle.
            c.cycles += 2;
            c.sb(r32(c, S0).wrapping_add(mon::DIRTY), c.g[V0][0]);
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 12;
        frame.restore(c, S0, 0x90);
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

pub(crate) const SPEECH_PLAY_HAS_ID: Callee = Callee {
    addr: 0x002a_fa58,
    f: ee::s08::SpeechBank_HasId,
};

/// The speech system pointer (`lw gp+0x6c4`).
pub(crate) fn speech_play_system(c: &mut Cpu) -> u64 {
    let gp = r32(c, GP);
    Addr(gp.wrapping_add(SPEECH_PLAY_SYSTEM_GP)).lw(c)
}

/// The player's start call (vtable slot 8) with the instance's play
/// parameters; `t1` is the timer argument (-1, or the instance's +0x8c8).
/// The registers the two call sites leave differ only in `t1`.
pub(crate) fn speech_play_start(
    c: &mut Cpu,
    system: u64,
    t1: Option<u64>,
    ret: u32,
) -> crate::call::Flow {
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
// 0x002ad5f0 BankMonitor_StopHandle

/// `void BankMonitor_StopHandle(BankMonitor* bm, int id, int kind, float
/// fade)` (0x2ad5f0): find the playing instance `(kind, id)`; fade it out
/// over `fade` (`BankMonitorInstance_FadeVolume(inst, -1, 1, fade)`) or,
/// for no fade, stop it at once (`BankMonitorInstance_Reset`).
pub fn bank_monitor_stop_handle(c: &mut Cpu) {
    use crate::ps2f::FpuF32;
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save_fpr(c, 20, 0x10);
        frame.save_ra(c, 0);
        c.f[20] = c.f[12]; // fade
        Callee {
            addr: 0x002a_d550,
            f: ee::s08::BankMonitor_FindInstance,
        }
        .call(c, 0x002a_d604)?;
        frame.restore_ra(c, 0);
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            c.f[0] = 0;
            let no_fade = FpuF32(c.f[20]).eq_s(FpuF32(c.f[0]));
            crate::abi::set_fcc(c, no_fade);
            c.g[A0][0] = c.g[V0][0];
            if !no_fade {
                c.cycles += 6;
                c.f[12] = c.f[20];
                c.g[A1][0] = u64::MAX;
                c.g[A2][0] = 0x1;
                Callee {
                    addr: 0x002a_cb80,
                    f: ee::s08::BankMonitorInstance_FadeVolume,
                }
                .call(c, 0x002a_d630)?;
            } else {
                c.cycles += 3;
                c.g[A0][0] = c.g[V0][0];
                Callee {
                    addr: 0x002a_bb38,
                    f: ee::s08::BankMonitorInstance_Reset,
                }
                .call(c, 0x002a_d640)?;
            }
            frame.restore_ra(c, 0);
        }
        c.cycles += 3;
        frame.restore_fpr(c, 20, 0x10);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::compare;
    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fakes::*;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::seed_count;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::SP as STACK_TOP;
    use crate::difftest::SP as STACK;
    pub(crate) use crate::game::ai::ai::tests::fake_check;
    pub(crate) use crate::game::ai::ai::tests::leaf;
    pub(crate) use crate::game::ai::ai::tests::leaf_setup;
    pub(crate) use crate::game::ai::ai::tests::rng as rng_2;
    pub(crate) use crate::game::ai::ai::tests::uf;
    pub(crate) use crate::game::ai::ai::tests::CHILD;
    pub(crate) use crate::game::be::bedbprofile::tests::strict;
    pub(crate) use crate::game::be::bestate2::tests::SEEDS as SEEDS_8;
    pub(crate) use crate::game::bx::bxstring::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::bx::bxstring2::tests::game;
    pub(crate) use crate::game::bx::bxstring2::tests::game_globals;
    pub(crate) use crate::game::bx::bxstring2::tests::SCRATCH;
    pub(crate) use crate::game::dirtysock::tags2::tests::clear as clear_4;
    pub(crate) use crate::game::dirtysock::tags2::tests::common;
    pub(crate) use crate::game::dirtysock::tags2::tests::dummy_world;
    pub(crate) use crate::game::dirtysock::tags2::tests::gp_at;
    pub(crate) use crate::game::dirtysock::tags2::tests::SEEDS_7 as SEEDS_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_3 as alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::b;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_9;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_18 as base_10;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_4 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::call_machine;
    pub(crate) use crate::game::fe::feasyncfile::tests::check_all;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::eafile_pools;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals as game_globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_13 as globals_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_5 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_9 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_10 as heap_init_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_12 as heap_init_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_8 as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_9 as heap_init_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_10 as pool_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_11 as pool_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_2 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_3 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_7 as pool_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_10 as pool_allocator_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_3 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_9 as pool_allocator_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::put_str_4 as put_str;
    pub(crate) use crate::game::fe::feasyncfile::tests::run;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_7 as run_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_8 as run_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_4 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_8 as slot_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_9 as slot_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::ui;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_3 as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_4 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_4 as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_6 as BE_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_8 as BE_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_9 as BE_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::IFACE;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ as OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ as OBJ_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDER;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDERS_2 as RIDERS;
    pub(crate) use crate::game::fe::feasyncfile::tests::RMAN;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM_3 as RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH as SCRATCH_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_12;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS_15;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_2 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_7 as WORLD_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_3 as W;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_4 as W_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W_3;
    pub(crate) use crate::game::fe::fepopup3::tests::wb;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::fe::festaterewards2::tests::belib;
    pub(crate) use crate::game::fe::festaterewards2::tests::w;
    pub(crate) use crate::game::fe::festaterewards2::tests::SEEDS as SEEDS_3;
    pub(crate) use crate::game::lib::sif::tests::SEEDS as SEEDS_9;
    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_2;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_10;
    pub(crate) use crate::game::main::gamestate::tests::base_3 as base_4;
    pub(crate) use crate::game::main::gamestate::tests::rand_setup;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_11;
    pub(crate) use crate::game::object::animnode::tests::fb;
    pub(crate) use crate::game::object::animnode::tests::heap_block;
    pub(crate) use crate::game::render::particle::tests::clear as clear_5;
    pub(crate) use crate::game::render::particle::tests::common as common_2;
    pub(crate) use crate::game::render::particle::tests::SEEDS_7;
    pub(crate) use crate::game::replay::replay2::tests::dtor_setup;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear as clear_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS_5;
    pub(crate) use crate::game::sound::bankmanager::tests::mixed;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS;
    pub(crate) use crate::game::visualfx::worldlightman::tests::alloc_setup;
    pub(crate) use crate::game::visualfx::worldlightman::tests::dtor_case;
    pub(crate) use crate::game::visualfx::worldlightman::tests::MEM_MUTEX;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3;
    pub(crate) use crate::game::wscript::wscriptman::tests::load;
    use ssx3_ee as ee;
    use ssx3_ee::s00;
    use ssx3_ee::s01;
    use ssx3_ee::s02;
    use ssx3_ee::s03;
    use ssx3_ee::s08;
    use ssx3_ee::s09;
    use ssx3_ee::s10;
    use ssx3_ee::s11;
    use ssx3_ee::s12;
    use ssx3_ee::s13;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn bank_monitor_timer_set_period_matches() {
        check(
            ssx3_ee::s08::f_002ad970__generated,
            bank_monitor_timer_set_period,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = r.next_u32() as u64 | (r.next_u32() as u64) << 32;
            },
        );
    }

    #[test]
    fn bank_monitor_tick_matches() {
        check(
            ssx3_ee::s08::f_002adc10__generated,
            bank_monitor_tick,
            200,
            &|c, seed| {
                let mut r = Rng(0x2222 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let active = if seed % 3 == 0 { 0 } else { r.next_u32() };
                c.bus.write32(DATA + 0x94, active);
                c.bus.write32(DATA + 0x88, r.next_u32());
            },
        );
    }

    #[test]
    fn path_get_track_data_matches() {
        check(
            ssx3_ee::s08::f_002adfb0__generated,
            path_get_track_data,
            200,
            &|c, seed| {
                let mut r = Rng(0x3333 ^ seed as u64);
                scramble(c, &mut r);
                for k in 0..8u32 {
                    c.bus.write32(TRACK_TABLE + 4 * k, r.next_u32());
                }
                c.g[A0][0] = (r.next_u32() % 8) as u64;
            },
        );
    }

    #[test]
    fn music_player_set_song_volume_matches() {
        check(
            ssx3_ee::s08::f_002b3c98__generated,
            music_player_set_song_volume,
            200,
            &|c, seed| {
                let mut r = Rng(0x4444 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = r.next_u32() as u64 | (r.next_u32() as u64) << 32;
                c.bus
                    .write32(DATA + 0x418, if seed % 4 == 0 { 0 } else { DATA + 0x300 });
                c.bus
                    .write32(DATA + 0x408, if seed % 5 == 0 { 0 } else { DATA + 0x400 });
            },
        );
    }

    /// Shared setup for the three `MusicPlayer_GetSong*` accessors: cover the
    /// direct-index, current-song and null paths.
    pub(crate) fn song_getter_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x5555 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = DATA as u64;
        match seed % 3 {
            0 => {
                c.g[A1][0] = (r.next_u32() % 10) as u64;
                c.bus.write32(DATA + CUR_SONG, r.next_u32());
            }
            _ => {
                c.g[A1][0] = 0xffff_ffff_ffff_ffff; // negative -> use current song
                let current = if seed % 2 == 0 {
                    (r.next_u32() % 10) as u32
                } else {
                    r.next_u32() | 0x8000_0000
                };
                c.bus.write32(DATA + CUR_SONG, current);
            }
        }
    }

    #[test]
    fn music_player_get_song_title_matches() {
        check_lohi(
            ssx3_ee::s08::f_002b40f0__generated,
            music_player_get_song_title,
            200,
            &song_getter_setup,
        );
    }

    #[test]
    fn music_player_get_song_artist_matches() {
        check_lohi(
            ssx3_ee::s08::f_002b4120__generated,
            music_player_get_song_artist,
            200,
            &song_getter_setup,
        );
    }

    #[test]
    fn music_player_get_song_album_matches() {
        check_lohi(
            ssx3_ee::s08::f_002b4150__generated,
            music_player_get_song_album,
            200,
            &song_getter_setup,
        );
    }

    #[test]
    fn song_instance_get_duck_to_loops_matches() {
        check(
            ssx3_ee::s08::f_002b4868__generated,
            song_instance_get_duck_to_loops,
            200,
            &|c, seed| {
                let mut r = Rng(0x6666 ^ seed as u64);
                scramble(c, &mut r);
                let owner = DATA + 0x100;
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA, owner);
                c.bus.write32(owner + 0x94, r.next_u32());
            },
        );
    }

    #[test]
    fn bank_monitor_timer_init_matches() {
        check(
            ssx3_ee::s08::f_002ad848,
            bank_monitor_timer_init,
            50,
            &|c, seed| {
                let mut r = Rng(0x1111_0013 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA, r.next_u32());
                c.bus.write32(DATA + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn bank_monitor_timer_is_free_matches() {
        check(
            ssx3_ee::s08::f_002ad930,
            bank_monitor_timer_is_free,
            100,
            &|c, seed| {
                let mut r = Rng(0x1111_0014 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(
                    DATA + 4,
                    match seed % 3 {
                        0 => 0xffff_ffff,
                        1 => 0,
                        _ => r.next_u32(),
                    },
                );
            },
        );
    }

    #[test]
    fn bank_monitor_timer_enable_matches() {
        check(
            ssx3_ee::s08::f_002ad980,
            bank_monitor_timer_enable,
            64,
            &|c, seed| {
                let mut r = Rng(0x2ad9 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn path_plat_mark_track_dirty_matches() {
        check(
            ssx3_ee::s08::f_002aec28,
            path_plat_mark_track_dirty,
            300,
            &|c, seed| {
                let mut r = Rng(0x2aec ^ seed as u64);
                scramble(c, &mut r);
                let index = (seed % 8) as u32;
                c.g[A0][0] = index as u64;
                let plat = if seed % 3 == 0 { 0 } else { DATA + 0x400 };
                c.bus.write32(PLATFORM_TABLE + index * 4, plat);
                c.bus.write8(DATA + 0x465, 0);
            },
        );
    }

    #[test]
    fn path_plat_get_track_aux_send_matches() {
        check(
            ssx3_ee::s08::f_002af4b0,
            path_plat_get_track_aux_send,
            300,
            &|c, seed| {
                let mut r = Rng(0x2af4 ^ seed as u64);
                scramble(c, &mut r);
                let index = (seed % 8) as u32;
                c.g[A0][0] = index as u64;
                let plat = if seed % 3 == 0 { 0 } else { DATA + 0x400 };
                c.bus.write32(PLATFORM_TABLE + index * 4, plat);
                if plat != 0 {
                    c.bus.write32(plat + 0x10, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn speech_bank_ctor_matches() {
        check(
            ssx3_ee::s08::f_002af8e0,
            speech_bank_ctor,
            64,
            &|c, seed| {
                let mut r = Rng(0x2af8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn speech_system_get_instance_stream_slot_matches() {
        check(
            ssx3_ee::s08::f_002b1220,
            speech_system_get_instance_stream_slot,
            200,
            &|c, seed| {
                let mut r = Rng(0x2b12 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let slot = (r.next_u32() % 8) as u32;
                c.g[A1][0] = slot as u64;
                let stream = if seed % 3 == 0 { 0 } else { DATA + 0x400 };
                c.bus.write32(DATA + 4, DATA + 0x200);
                c.bus.write32(DATA + 0x200 + slot * 4, stream);
                if stream != 0 {
                    c.bus.write32(stream + 4, r.next_u32());
                }
            },
        );
    }

    /// Ten event-context slots at `DATA + 0x18` with keys from a small set
    /// (so lookups hit), some free, some about to expire.
    fn speech_contexts(c: &mut Cpu, r: &mut Rng, all_used: bool) {
        for i in 0..10u32 {
            let s = DATA + 0x18 + 0x20 * i;
            let used = all_used || r.next_u32() % 3 != 0;
            c.bus
                .write32(s, if used { 1 + r.next_u32() % 3 } else { 0 });
            c.bus.write32(
                s + 4,
                [0u32, 1, 2, 0xffff_ffff, 179, 0x8000_0000][(r.next_u32() % 6) as usize],
            );
            c.bus.write32(s + 8, r.next_u32() % 3);
            c.bus.write32(s + 0xc, r.next_u32() % 3);
            for off in [0x10u32, 0x14, 0x18, 0x1c] {
                c.bus.write32(s + off, r.next_u32());
            }
        }
    }

    #[test]
    fn speech_system_set_event_context_matches() {
        check(
            ee::s08::f_002b1458__generated,
            speech_system_set_event_context,
            300,
            &|c, seed| {
                let mut r = rng(0x002b_1458, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                speech_contexts(c, &mut r, seed % 5 == 0);
                let key = |r: &mut Rng| match r.next_u32() % 5 {
                    0 => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                    _ => (r.next_u32() % 3) as u64,
                };
                c.g[A1][0] = key(&mut r);
                c.g[A2][0] = key(&mut r);
                c.g[T2][0] = if seed % 2 == 0 {
                    0
                } else {
                    (r.next_u32() as u64) << 32 | 1
                };
                c.f[12] = r.edge_f32();
            },
        );
    }

    #[test]
    fn speech_system_tick_event_contexts_matches() {
        check(
            ee::s08::f_002b1720__generated,
            speech_system_tick_event_contexts,
            300,
            &|c, seed| {
                let mut r = rng(0x002b_1720, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                speech_contexts(c, &mut r, false);
            },
        );
    }

    /// Ticks until a context set at rate `hz` is freed by the per-tick
    /// countdown.
    fn speech_context_ticks(hz: u32) -> u32 {
        let mut c = crate::difftest::fresh();
        crate::tick::set_for_test(&mut c, hz);
        for i in 0..10u32 {
            c.bus.write32(DATA + 0x18 + 0x20 * i, 0);
        }
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = 1;
        c.g[A2][0] = 2;
        c.g[T2][0] = 1;
        c.g[RA][0] = sx(TEST_RA);
        speech_system_set_event_context(&mut c);
        assert_eq!(c.g[V0][0], 1, "context stored");
        for n in 1..=10_000 {
            c.g[A0][0] = sx(DATA);
            c.g[RA][0] = sx(TEST_RA);
            speech_system_tick_event_contexts(&mut c);
            if c.bus.read32(DATA + 0x18) == 0 {
                return n;
            }
        }
        panic!("{hz} Hz: context never freed");
    }

    /// A speech event context lives 3 s at every rate.
    #[test]
    fn speech_system_event_context_lifetime_follows_the_rate() {
        assert_eq!(speech_context_ticks(60), 180);
        assert_eq!(speech_context_ticks(120), 360);
        assert_eq!(speech_context_ticks(240), 720);
    }

    #[test]
    fn song_instance_apply_delay_params_matches() {
        check(
            ssx3_ee::s08::f_002b2850,
            song_instance_apply_delay_params,
            100,
            &|c, seed| {
                let mut r = Rng(0x2b28 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                c.bus.write32(DATA + 0x100, DATA + 0x1000);
                c.bus.write32(DATA, DATA + 0x2000);
                c.bus.write32(DATA + 0x1000, DATA + 0x3000);
                c.bus.write32(DATA + 0x2078, r.next_u32());
                c.bus.write32(DATA + 0x31f4, 0);
                c.bus.write32(DATA + 0x3248, 0);
                c.bus.write32(DATA + 0x207c, r.next_u32());
                c.bus.write32(DATA + 0x2080, r.next_u32() & 0x3ff);
            },
        );
    }

    #[test]
    fn song_instance_get_sed_value_matches() {
        check(
            ssx3_ee::s08::f_002b48b8,
            song_instance_get_sed_value,
            100,
            &|c, seed| {
                let mut r = Rng(0x2b48 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA, DATA + 0x400);
                c.bus.write32(DATA + 0x498, r.next_u32());
            },
        );
    }

    #[test]
    fn music_player_get_selected_song_sed_value_matches() {
        check(
            ssx3_ee::s08::f_002b4908,
            music_player_get_selected_song_sed_value,
            100,
            &|c, seed| {
                let mut r = Rng(0x2b49 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let idx = (r.next_u32() % 8) as u32;
                c.bus.write32(DATA + 0x3f0, idx);
                c.bus.write32(DATA + idx * 4 + 4, DATA + 0x400);
                c.bus.write32(DATA + 0x498, r.next_u32());
            },
        );
    }

    #[test]
    fn music_player_is_current_song_enabled_matches() {
        check(
            ssx3_ee::s08::f_002b4af0,
            music_player_is_current_song_enabled,
            200,
            &|c, seed| {
                let mut r = Rng(0x2b4a ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(
                    DATA + 0x3f0,
                    if seed % 2 == 0 {
                        r.next_u32() % 64
                    } else {
                        r.next_u32()
                    },
                );
                c.bus.write32(DATA + 0x3f8, r.next_u32());
                c.bus.write32(DATA + 0x3fc, r.next_u32());
            },
        );
    }

    #[test]
    fn bank_monitor_set_param_callback_matches() {
        check(
            ssx3_ee::s08::f_002b9ce0,
            bank_monitor_set_param_callback,
            200,
            &|c, seed| {
                let mut r = Rng(0x9ce0 ^ seed as u64);
                scramble(c, &mut r);
                let base = DATA + (seed % 8);
                c.g[A0][0] = base as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.bus.write32(base + 0x1c, r.next_u32() % 8);
            },
        );
    }

    // PATH_PlatRefreshStatus ---------------------------------------------

    pub(crate) const MON_TABLE: u32 = 0x004d_3e98;

    pub(crate) const MON: u32 = DATA + 0x7000;

    pub(crate) const MON_SUBS: u32 = DATA + 0x7800;

    #[test]
    fn path_plat_refresh_status_matches() {
        covered(
            ssx3_ee::s08::f_002aefa8__generated,
            path_plat_refresh_status,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2aefa8, 1);
                c.g[A0][0] = (seed % 3 == 0) as u64; // reset
                for i in 0..24 {
                    let m = if i == (seed % 24) as u32 { MON } else { 0 };
                    c.bus.write32(MON_TABLE + 4 * i, m);
                }
                let m = MON;
                // One sub-stream on the slot's list.
                c.bus.write32(m + 0x6c, MON_SUBS);
                c.bus.write32(MON_SUBS, 0x40);
                c.bus.write32(MON_SUBS + 8, 0);
                match seed % 6 {
                    0 => c.bus.write32(m + 0x0, u32::MAX), // no stream
                    1 => {
                        // In memory, but the slot's sub index is negative.
                        c.bus.write8(m + 0x64, 1);
                        c.bus.write32(m + 0x4, u32::MAX);
                    }
                    2 => {
                        // Streaming with no sub-streams.
                        c.bus.write8(m + 0x64, 0);
                        c.bus.write8(m + 0x66, 0);
                    }
                    3 => {
                        // In memory, but its sub-stream is gone.
                        c.bus.write8(m + 0x64, 1);
                        c.bus.write32(MON_SUBS + 4, u32::MAX);
                    }
                    4 => {
                        // In memory with a live sub-stream.
                        c.bus.write8(m + 0x64, 1);
                        c.bus.write32(MON_SUBS + 4, 1);
                        c.bus.write32(m + 0x54, 0);
                        c.bus.write32(m + 0x50, 0);
                    }
                    _ => {
                        // Streaming with one live sub-stream.
                        c.bus.write8(m + 0x64, 0);
                        c.bus.write8(m + 0x66, 1);
                        c.bus.write32(MON_SUBS + 4, 1);
                    }
                }
            },
        );
    }

    #[test]
    fn speech_bank_has_id_matches() {
        check(
            ee::s08::f_002afa58__generated,
            speech_bank_has_id,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x002a_fa58, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                let id = r.next_u32();
                c.bus.write32(DATA, id);
                c.g[A1][0] = match seed % 3 {
                    0 => mixed(&mut r, seed, id),
                    1 => id as u64,
                    _ => r.next_u32() as u64,
                };
            },
        );
    }

    #[test]
    fn music_player_set_song_field54_matches() {
        check(
            ee::s08::f_002b3cc0__generated,
            music_player_set_song_field54,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x002b_3cc0, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                if seed % 3 == 0 {
                    c.bus.write32(DATA + 0x408, 0);
                } else {
                    c.bus.write32(DATA + 0x408, DATA + 0x2000);
                    c.bus.write32(DATA + 0x2000 + 0x54, r.next_u32());
                }
                let v = r.next_u32();
                c.g[A1][0] = mixed(&mut r, seed, v);
            },
        );
    }

    #[test]
    fn music_player_set_song_field5c_matches() {
        check(
            ee::s08::f_002b4a08__generated,
            music_player_set_song_field5c,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x002b_4a08, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                if seed % 3 == 0 {
                    c.bus.write32(DATA + 0x408, 0);
                } else {
                    c.bus.write32(DATA + 0x408, DATA + 0x2000);
                    c.bus.write32(DATA + 0x2000 + 0x5c, r.next_u32());
                }
                let v = r.next_u32();
                c.g[A1][0] = mixed(&mut r, seed, v);
            },
        );
    }

    #[test]
    fn music_player_set_song_field50_matches() {
        check(
            ee::s08::f_002b46d0__generated,
            music_player_set_song_field50,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x002b_46d0, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                if seed % 3 == 0 {
                    c.bus.write32(DATA + 0x408, 0);
                } else {
                    c.bus.write32(DATA + 0x408, DATA + 0x2000);
                    c.bus.write32(DATA + 0x2000 + 0x50, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn audio_rand_matches() {
        check(
            ee::s08::f_002adf60__generated,
            audio_rand,
            200,
            &|c, seed| {
                rand_setup(c, seed, 0x004f_f018);
            },
        );
    }

    #[test]
    fn song_instance_path_cmd_a_matches() {
        check(
            ee::s08::f_002b2120__generated,
            song_instance_path_cmd_a,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2b_2120, 0);
                c.g[A0][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn music_player_are_loops_active_matches() {
        check(
            ee::s08::f_002b46e8__generated,
            music_player_are_loops_active,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2b_46e8, 0);
                c.g[A0][0] = DATA as u64;
                if seed % 2 == 0 {
                    c.bus.write32(DATA + 0x408, 0);
                } else {
                    c.bus.write32(DATA + 0x408, DATA + 0x500);
                    c.bus.write32(DATA + 0x500 + 0x4c, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn music_player_set_song_tag_matches() {
        check(
            ee::s08::f_002b49c0__generated,
            music_player_set_song_tag,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2b_49c0, 0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = r.next_u32() as u64;
                match seed % 3 {
                    0 => c.bus.write32(DATA + 0x418, 0),
                    1 => {
                        c.bus.write32(DATA + 0x418, 1);
                        c.bus.write32(DATA + 0x408, 0);
                    }
                    _ => {
                        c.bus.write32(DATA + 0x418, 1);
                        c.bus.write32(DATA + 0x408, DATA + 0x500);
                    }
                }
            },
        );
    }

    #[test]
    fn bank_monitor_timer_dtor_matches() {
        check(
            ee::s08::f_002ad860__generated,
            bank_monitor_timer_dtor,
            60,
            &|c, seed| dtor_setup(c, seed, 0x2ad860),
        );
    }

    #[test]
    fn speech_instance_stop_stream_matches() {
        check(
            ee::s08::f_002b0320__generated,
            speech_instance_stop_stream,
            100,
            &|c, seed| {
                let _r = game(c, seed, 0x2b0320, 1);
                let instance = SCRATCH + 0x3000;
                let owner = SCRATCH + 0x4000;
                let array = SCRATCH + 0x5000;
                let elements = SCRATCH + 0x6000;
                c.g[A0][0] = instance as u64;
                c.bus.write32(difftest_gp() + 0x6c4, owner); // D_004A37B4
                c.bus.write32(owner, array);
                c.bus.write32(instance + 4, (seed % 8) as u32);
                c.bus.write32(array + 8, elements);
                // No element is live, so the array only reads them.
                for i in 0..8u32 {
                    c.bus.write32(elements + 4 * i, 0);
                }
            },
        );
    }

    /// The `gp` value the tests run with (`machine` sets it).
    pub(crate) fn difftest_gp() -> u32 {
        crate::difftest::GAME_GP
    }

    #[test]
    fn bank_monitor_reset_instance_matches() {
        check_lohi(
            ssx3_ee::s08::f_002ad398__generated,
            bank_monitor_reset_instance,
            100,
            &|c, seed| {
                machine(c, seed, 0x41, 0);
                let monitor = DATA + 0x4000;
                let instances = DATA + 0x5000;
                c.bus.write32(monitor + 0x8ec, instances);
                for i in 0..4u32 {
                    // FadeOut returns at once when the instance is fading.
                    c.bus.write32(instances + i * 0xc0 + 0x54, 1);
                }
                c.g[A0][0] = sx(monitor);
                c.g[A1][0] = (seed % 4) as u64;
            },
        );
    }

    /// Scratch for the callees' own data.
    pub(crate) const SONG: u32 = DATA + 0x2000;

    // -----------------------------------------------------------------
    // Music player.

    #[test]
    fn music_player_pause_matches() {
        check(
            s08::f_002b3a70__generated,
            music_player_pause,
            150,
            &|c, seed| {
                machine(c, seed, 0x2b3a70, 1);
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                clear(c, SONG, 0x100);
                c.g[A0][0] = sx(DATA + 0x400);
                // No song / a song with no stream: both early-outs of the callee.
                let song = if seed % 3 == 0 { 0 } else { SONG };
                c.bus.write32(DATA + 0x400 + 0x408, song);
            },
        );
    }

    #[test]
    fn music_player_resume_matches() {
        check(
            s08::f_002b3a98__generated,
            music_player_resume,
            150,
            &|c, seed| {
                machine(c, seed, 0x2b3a98, 1);
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                clear(c, SONG, 0x100);
                c.g[A0][0] = sx(DATA + 0x400);
                let song = if seed % 3 == 0 { 0 } else { SONG };
                c.bus.write32(DATA + 0x400 + 0x408, song);
            },
        );
    }

    #[test]
    fn music_player_get_song_tag_matches() {
        check(
            s08::f_002b49e0__generated,
            music_player_get_song_tag,
            200,
            &|c, seed| {
                let mut r = Rng(0x49e0_1234 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(OBJ);
                clear(c, OBJ, 0x40);
                let tag_flag = if seed % 3 == 0 { 0 } else { 1 };
                let song = if seed % 5 == 0 { 0 } else { CHILD };
                c.bus.write32(OBJ + 0x418, tag_flag);
                c.bus.write32(OBJ + 0x408, song);
                c.bus.write32(CHILD + 0x58, r.next_u32());
            },
        );
    }

    #[test]
    fn path_plat_remove_track_slot_matches() {
        check(
            ee::s08::f_002ae188__generated,
            path_plat_remove_track_slot,
            SEEDS_2,
            &|c, seed| {
                let mut r = Rng(0x002a_e188 ^ (seed as u64) << 3);
                scramble(c, &mut r);
                // The 24-entry table; some entries equal the slot to be cleared
                // (duplicates included so several matches happen per scan).
                let target = 0x7700_0000u32 | (seed % 24);
                for i in 0..24u32 {
                    let v = if (i + seed) % 4 == 0 {
                        target
                    } else {
                        r.next_u32()
                    };
                    c.bus.write32(0x004d_3e98 + 4 * i, v);
                }
                c.g[A0][0] = if seed % 3 == 0 {
                    r.next_u32() as u64
                } else {
                    target as u64
                };
            },
        );
    }

    // ------------------------------------------------------- world triggers

    #[test]
    fn c_song_configuration_dtor_matches() {
        dtor_case(
            ee::s08::f_002b1b68__generated,
            c_song_configuration_dtor,
            0x2b1b68,
        );
    }

    #[test]
    fn bank_monitor_reset_distances_matches() {
        check(
            ee::s08::f_002ad810__generated,
            bank_monitor_reset_distances,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ad8_10, 0);
                let voices = DATA + 0x1000;
                c.g[A0][0] = sx(DATA + 0x100);
                c.bus.write32(DATA + 0x100 + 0x8ec, voices);
                fill(c, &mut r, voices, 64 * 0x30);
            },
        );
    }

    #[test]
    fn speech_system_stop_instance_matches() {
        check(
            ee::s08::f_002b11b0__generated,
            speech_system_stop_instance,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2b11_b0, 0);
                let sys = DATA + 0x100;
                c.g[A0][0] = sx(sys);
                c.g[A1][0] = sx(seed % 16);
                c.bus.write32(sys + 4, DATA + 0x200);
                let slot = DATA + 0x200 + 4 * (seed % 16);
                let entry = if seed % 3 == 0 { 0 } else { DATA + 0x360 };
                c.bus.write32(slot, entry);
                fill(c, &mut r, DATA + 0x360, 8);
            },
        );
    }

    #[test]
    fn song_instance_get_request_status_matches() {
        check(
            ee::s08::f_002b25c8__generated,
            song_instance_get_request_status,
            80,
            &|c, seed| {
                let _r = machine(c, seed, 0x2b25_c8, 0);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(DATA + 0x400);
                // The stream id the wrapper reloads from its frame (`lw(sp + 4)`):
                // negative means "no stream" to SNDSTRM_GetRequestStatus.
                c.bus.write32(STACK_TOP - 0x30 + 4, 0xffff_ffff);
                let _ = seed;
            },
        );
    }

    #[test]
    fn music_player_post_event_matches() {
        check(
            ee::s08::f_002b3b88__generated,
            music_player_post_event,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2b3b_88, 0);
                let mp = DATA + 0x100;
                let song = DATA + 0x300;
                c.g[A0][0] = sx(mp);
                c.g[A1][0] = sx(seed * 3);
                let has_song = seed % 3 != 0;
                let active = seed % 2 == 0;
                c.bus.write32(mp + 0x418, if active { 1 } else { 0 });
                c.bus.write32(mp + 0x408, if has_song { song } else { 0 });
                // A song with no pending events and no queue: the post returns
                // after its first test.
                for i in 0..8u32 {
                    c.bus.write32(song + 4 * i, 0);
                }
            },
        );
    }

    #[test]
    fn music_player_fade_out_matches() {
        check(
            ee::s08::f_002b3d48__generated,
            music_player_fade_out,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2b3d_48, 0);
                let mp = DATA + 0x100;
                let song = DATA + 0x300;
                c.g[A0][0] = sx(mp);
                c.g[A1][0] = sx(r.edge_f32());
                let has_song = seed % 3 != 0;
                let active = seed % 2 == 0;
                c.bus.write32(mp + 0x418, if active { 1 } else { 0 });
                c.bus.write32(mp + 0x408, if has_song { song } else { 0 });
                for i in 0..8u32 {
                    c.bus.write32(song + 4 * i, 0);
                }
            },
        );
    }

    #[test]
    fn music_player_clear_playlist_matches() {
        check(
            ee::s08::f_002b3eb0__generated,
            music_player_clear_playlist,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2b3e_b0, 0);
                let mp = DATA + 0x100;
                c.g[A0][0] = sx(mp);
                fill(c, &mut r, mp + 0x3e0, 0x160);
            },
        );
    }

    // ----------------------------------------------------------- bank monitor

    #[test]
    fn path_plat_shutdown_matches() {
        check(
            ee::s08::f_002ae100__generated,
            path_plat_shutdown,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x2ae1_00, 0);
                let present = if seed % 2 == 0 { 0 } else { 1 };
                c.bus.write32(GAME_GP.wrapping_add(0x6a8), present);
                c.bus.write32(GAME_GP.wrapping_add(0x6ac), present);
            },
        );
    }

    #[test]
    fn path_plat_find_free_track_slot_matches() {
        check(
            ee::s08::f_002ae1b8__generated,
            path_plat_find_free_track_slot,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x2ae1_b8, 0);
                let used = seed % 26;
                for i in 0..24u32 {
                    c.bus.write32(0x004d_3e98 + 4 * i, (i < used) as u32);
                }
            },
        );
    }

    #[test]
    fn path_plat_bank_slot_init_matches() {
        check(
            ssx3_ee::s08::f_002ae1f0__generated,
            path_plat_bank_slot_init,
            SEEDS_3,
            &|c, seed| {
                machine(c, seed, 0x2ae1_f0, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn music_player_get_duck_to_loops_matches() {
        check(
            ssx3_ee::s08::f_002b4878__generated,
            music_player_get_duck_to_loops,
            SEEDS_3,
            &|c, seed| {
                machine(c, seed, 0x2b48_78, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
                match seed % 3 {
                    0 => {}
                    1 => {
                        w(c, DATA + 0x418, 1);
                    }
                    _ => {
                        w(c, DATA + 0x418, 1);
                        w(c, DATA + 0x408, DATA + 0x1000);
                    }
                }
            },
        );
    }

    pub(crate) const SEEDS_4: u32 = 200;

    #[test]
    fn bank_monitor_kill_all_timers_matches() {
        check(
            ee::s08::f_002adda0__generated,
            bank_monitor_kill_all_timers,
            SEEDS_4,
            &|c, seed| {
                let _ = machine(c, seed, 0x2adda0, 0);
                let self_at = DATA + 0x200;
                c.g[A0][0] = self_at as u64;
                for k in 0..4u32 {
                    c.bus.write32(self_at + 0x98 + 0x2c * k, seed + k);
                }
            },
        );
    }

    #[test]
    fn path_plat_set_track_file_matches() {
        check(
            ee::s08::f_002ae768__generated,
            path_plat_set_track_file,
            SEEDS_4,
            &|c, seed| {
                let _ = machine(c, seed, 0x2ae768, 0);
                let plat = DATA + 0x300;
                for k in 0..4u32 {
                    let entry = if (seed / 4 + k) % 4 == 0 {
                        0
                    } else {
                        plat + 0x80 * k
                    };
                    c.bus.write32(0x004d_3e98 + 4 * k, entry);
                    c.bus
                        .write32(plat + 0x80 * k + 0x64, ((seed + k) % 3) as u32 & 1);
                }
                let idx = if seed % 6 == 5 {
                    0x18u64
                } else {
                    (seed % 4) as u64
                };
                c.g[A0][0] = idx;
                c.g[A1][0] = (seed % 1000) as u64;
                c.g[A2][0] = (0x1000 + seed) as u64;
            },
        );
    }

    #[test]
    fn path_plat_clear_prebuffer_matches() {
        check(
            ee::s08::f_002ae9f8__generated,
            path_plat_clear_prebuffer,
            SEEDS_4,
            &|c, seed| {
                let _ = machine(c, seed, 0x2ae9f8, 0);
                let plat = DATA + 0x300;
                let idx = seed % 4;
                c.g[A0][0] = idx as u64;
                for k in 0..4u32 {
                    c.bus.write32(0x004d_3e98 + 4 * k, plat);
                }
                c.bus.write32(plat + 0x64, ((seed / 4) % 2) as u32);
                c.bus.write32(plat + 0x6c, DATA + 0x600);
                c.bus.write32(DATA + 0x604, (seed % 8) as u32);
            },
        );
    }

    #[test]
    fn song_instance_pick_random_loop_matches() {
        check_lohi(
            ee::s08::f_002b24c0__generated,
            song_instance_pick_random_loop,
            SEEDS_4,
            &|c, seed| {
                let _ = machine(c, seed, 0x2b24c0, 0);
                let self_at = DATA + 0x200;
                let song = DATA + 0x300;
                c.g[A0][0] = self_at as u64;
                c.bus.write32(self_at, song);
                c.bus.write32(
                    song + 0x40,
                    match seed % 3 {
                        0 => 0,
                        1 => 1,
                        _ => 1 + seed % 7,
                    },
                );
                c.bus.write32(
                    GAME_GP + 0x68c,
                    if seed % 4 == 0 { 0 } else { seed * 0x9e37_79b9 },
                );
            },
        );
    }

    #[test]
    fn song_instance_get_position_ms_matches() {
        check(
            ee::s08::f_002b2600__generated,
            song_instance_get_position_ms,
            SEEDS_4,
            &|c, seed| {
                let _ = machine(c, seed, 0x2b2600, 0);
                let self_at = DATA + 0x200;
                c.g[A0][0] = self_at as u64;
                c.bus.write32(self_at, (seed % 4) as u32);
                c.bus.write32(self_at + 0x14, (seed % 6) as u32);
            },
        );
    }

    #[test]
    fn music_player_play_preview_matches() {
        covered(
            ee::s08::f_002b4978__generated,
            music_player_play_preview,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2b_4978, 1);
                let p = DATA + 0x1000;
                let slot = (r.next_u32() % 4) as u32;
                let entry = DATA + 0x2000;
                c.bus.write32(p + 4 + 4 * slot, entry);
                c.bus
                    .write32(entry + 0xa0, if seed % 2 == 0 { u32::MAX } else { 1 });
                let vt = vtable(c, &mut r, &[0x18]);
                c.bus.write32(p + 0x5440, vt);
                c.g[A0][0] = sx(p);
                c.g[A1][0] = slot as u64;
            },
        );
    }

    #[test]
    fn path_plat_calc_track_mem_size_matches() {
        check(
            ee::s08::f_002ae478__generated,
            path_plat_calc_track_mem_size,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_e478, 0);
                c.g[A0][0] = (r.next_u32() % 0x1000) as u64;
                c.g[A1][0] = match seed % 4 {
                    0 => 0,
                    1 => u32::MAX as u64,
                    2 => 1,
                    _ => (r.next_u32() % 0x40) as u64,
                };
            },
        );
    }

    #[test]
    fn music_player_is_song_data_loaded_matches() {
        check(
            ee::s08::f_002b4aa0__generated,
            music_player_is_song_data_loaded,
            60,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x2b4a_a0, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(seed % 3);
                c.bus
                    .write32(this + 0x5438, if seed % 4 == 1 { 0 } else { DATA + 0x200 });
                c.bus.write32(DATA + 0x200, r.next_u32());
                c.bus.write32(DATA + 0x200 + 0x1c, seed as u32 % 3);
            },
        );
    }

    #[test]
    fn bank_monitor_stop_all_matches() {
        // 64 reset calls per seed: keep the seed count (and the run time) down.
        check(
            ee::s08::f_002ad3c0__generated,
            bank_monitor_stop_all,
            30,
            &|c, seed| {
                let _ = machine(c, seed, 0x2ad3_c0, 1);
                let this = DATA + 0x100;
                clear_2(c, this, 0x1000);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn path_plat_track_init_matches() {
        check(
            ee::s08::f_002ae138__generated,
            path_plat_track_init,
            SEEDS_5,
            &|c, seed| {
                let mut r = base(c, seed, 0x2ae1_38);
                let this = DATA + 0x100;
                for i in 0..0x20u32 {
                    c.bus.write32(this + 4 * i, r.next_u32());
                }
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn ini_parser_dtor_matches() {
        check(
            ee::s08::f_002a7890__generated,
            ini_parser_dtor,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2a78_90, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear_3(c, this, 0x20);
                let owned = if seed % 2 == 0 {
                    0
                } else {
                    heap_block(c, 0x20)
                };
                c.bus.write32(this + 4, owned);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 3 == 0) as u64;
            },
        );
    }

    #[test]
    fn bank_monitor_resume_all_matches() {
        check(
            ee::s08::f_002ad300__generated,
            bank_monitor_resume_all,
            SEEDS_6,
            &|c, seed| {
                let _ = common(c, seed, 0x2ad300, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                let inst = DATA + 0x1000;
                clear_4(c, this, 0x1000);
                clear_4(c, inst, 0x3000);
                c.bus.write32(this + 0x8ec, inst);
                c.g[A0][0] = sx(this);
                let _ = seed;
            },
        );
    }

    #[test]
    fn path_plat_get_track_volume_matches() {
        check(
            ee::s08::f_002af428__generated,
            path_plat_get_track_volume,
            SEEDS_6,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2af428, 0);
                let table = 0x004d_3e98u32;
                let obj = DATA + 0x2000;
                clear_4(c, obj, 0x100);
                let idx = seed % 5;
                let present = seed % 3 != 2;
                c.bus
                    .write32(table + 4 * idx, if present { obj } else { 0 });
                c.bus.write8(obj + 0x65, 0);
                c.bus.write32(obj + 0xc, r.next_u32());
                c.g[A0][0] = idx as u64;
            },
        );
    }

    #[test]
    fn bank_monitor_pause_all_matches() {
        check(
            ee::s08::f_002ad2a8__generated,
            bank_monitor_pause_all,
            20,
            &|c, seed| {
                let _ = common_2(c, seed, 0x2ad2a8, 1);
                let this = DATA + 0x100;
                let pool = DATA + 0x2000;
                clear_5(c, this, 0x300);
                clear_5(c, pool, 64 * 0xc0 / 4);
                c.bus.write32(this + 0x8ec, pool);
                c.bus.write32(pool + 0x0, seed % 3);
                c.bus.write32(pool + 0x54, (seed / 3) % 2);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn song_instance_pause_matches() {
        check(
            ee::s08::f_002b2018__generated,
            song_instance_pause,
            SEEDS_7,
            &|c, seed| {
                let _ = common_2(c, seed, 0x2b2018, 4);
                let this = DATA + 0x100;
                clear_5(c, this, 0x200);
                c.bus.write32(this + 0x408, DATA + 0x2000);
                c.g[A0][0] = sx(this);
                let _ = seed;
            },
        );
    }

    #[test]
    fn song_instance_resume_matches() {
        check(
            ee::s08::f_002b2070__generated,
            song_instance_resume,
            SEEDS_7,
            &|c, seed| {
                let _ = common_2(c, seed, 0x2b2070, 4);
                let this = DATA + 0x100;
                clear_5(c, this, 0x200);
                c.bus.write32(this + 0x408, DATA + 0x2000);
                c.g[A0][0] = sx(this);
                let _ = seed;
            },
        );
    }

    #[test]
    fn song_instance_post_event_matches() {
        check(
            ee::s08::f_002b20c8__generated,
            song_instance_post_event,
            SEEDS_7,
            &|c, seed| {
                let mut r = common_2(c, seed, 0x2b20c8, 1);
                let this = DATA + 0x100;
                clear_5(c, this, 0x200);
                c.bus.write32(this + 0x5c, (seed % 2) as u32);
                c.bus.write32(this + 0x2c, ((seed / 2) % 2) as u32);
                c.bus.write32(
                    this + 0x30,
                    if seed % 3 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32()
                    },
                );
                c.g[A0][0] = sx(this);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = ((seed / 4) % 2) as u64;
            },
        );
    }

    #[test]
    fn music_player_get_ms_to_next_beat_matches() {
        check(
            ee::s08::f_002b4920__generated,
            music_player_get_ms_to_next_beat,
            SEEDS_7,
            &|c, seed| {
                let _ = common_2(c, seed, 0x2b4920, 4);
                let this = DATA + 0x100;
                clear_5(c, this, 0x300);
                c.bus.write32(this + 0x418, (seed % 2) as u32);
                c.bus.write32(this + 0x408, DATA + 0x2000);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn speech_bank_dtor_matches() {
        check(
            ee::s08::f_002af900__generated,
            speech_bank_dtor,
            SEEDS_8,
            &|c, seed| {
                machine(c, seed, 0x002a_f900, 3);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 3) as u64;
                w(c, DATA, DATA + 0x600);
                w(c, DATA + 0x44, DATA + 0x800);
                w(c, DATA + 0x48, if seed % 2 == 0 { 0 } else { 1 });
            },
        );
    }

    #[test]
    fn path_plat_find_bank_slot_matches() {
        check(
            ee::s08::f_002ae230__generated,
            path_plat_find_bank_slot,
            100,
            &|c, seed| {
                machine(c, seed, 0x002a_e230, 0);
                belib(c);
                for i in 0..24u32 {
                    let base = 0x004d_3ef8 + 0x28 * i;
                    w(c, base, i);
                    let bank = if i < 20 { 0xffff_ffff } else { i };
                    w(c, base + 8, bank);
                }
                c.g[A0][0] = match seed % 5 {
                    0 => 0xffff_ffff_ffff_ffff,
                    1 => 21,
                    2 => 20 + (seed % 4) as u64,
                    3 => 0,
                    _ => 0xffff_ffff_ffff_fffe,
                };
                c.g[A1][0] = match seed % 3 {
                    0 => 0xffff_ffff_ffff_ffff,
                    1 => 0xffff_ffff_ffff_fffe,
                    _ => (seed % 25) as u64,
                };
            },
        );
    }

    #[test]
    fn song_instance_get_ms_to_next_beat_matches() {
        check(
            ee::s08::f_002b2718__generated,
            song_instance_get_ms_to_next_beat,
            SEEDS_8,
            &|c, seed| {
                machine(c, seed, 0x002b_2718, 3);
                belib(c);
                c.g[A0][0] = DATA as u64;
                w(c, DATA, DATA + 0x200);
                w(c, DATA + 0x200 + 0x90, fb(1.0 + (seed % 4) as f32));
                w(c, DATA + 0x200 + 0x4, (seed % 8) as u32);
            },
        );
    }

    #[test]
    fn music_player_set_event_state_matches() {
        check(
            ee::s08::f_002b3bc0__generated,
            music_player_set_event_state,
            12,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002b_3bc0, 1);
                be_library(c);
                c.g[A0][0] = DATA as u64;
                let v = (r.next_u32() & 7) as u64;
                let current = (seed as u32) & 7;
                c.g[A1][0] = if seed % 3 == 0 { current as u64 } else { v };
                c.bus
                    .write32(DATA + 0x418, if seed % 4 == 0 { 0 } else { 1 });
                c.bus.write32(DATA + 0x41c, current);
                c.bus.write32(DATA + 0x5440, DATA + 0x2000);
                c.bus.write16(DATA + 0x2000 + 0x18, 0);
                c.bus.write32(DATA + 0x2000 + 0x1c, FAKE);
            },
        );
    }

    #[test]
    fn bank_monitor_timer_disable_matches() {
        leaf(
            ssx3_ee::s08::f_002ad990__generated,
            bank_monitor_timer_disable,
            24,
        );
    }

    // --------------------------------------------------------- SPCH callback

    #[test]
    fn spch_notify_part_callback_matches() {
        fake_check(
            ssx3_ee::s14::f_003db040__generated,
            spch_notify_part_callback,
            46,
            &|c, seed, r| {
                c.g[SP][0] = (STACK - 0x100) as u64;
                c.g[A0][0] = (DATA + 0x200) as u64;
                c.g[A1][0] = DATA as u64;
                c.g[A2][0] = 0;
                c.g[A3][0] = (DATA + 0x2000) as u64;
                // A function pointer at the callback slot (0x44ff98).
                c.bus.write32(0x0044_ff98, 0x4000);
                // The instrument-class table (`.rodata 0x495ba0`).
                put_words(
                    c,
                    0x0049_5ba0,
                    &[
                        0x003d_b118,
                        0x003d_b1cc,
                        0x003d_b1cc,
                        0x003d_b118,
                        0x003d_b1cc,
                        0x003d_b1cc,
                    ],
                );
                // Bank descriptor at DATA.
                c.bus.write8(DATA + 0x6, 0);
                c.bus.write8(DATA + 0x7, 1);
                c.bus.write8(DATA + 0x2, 0);
                c.bus.write8(DATA + 0x4, if seed % 2 == 0 { 0 } else { 4 });
                c.bus.write8(DATA + 0x8, 0);
                c.bus.write16(DATA + 0xc, 0);
                c.bus.write8(DATA + 0xc, 0);
                c.bus.write8(DATA + 0xd, 0);
                c.bus.write8(DATA + 0xe, (r.next_u32() % 6) as u8);
                c.bus.write32(DATA + 0x2000, 0x1234);
            },
        );
    }

    #[test]
    fn ini_parser_at_section_end_matches() {
        check(
            ee::s08::f_002a7f90__generated,
            ini_parser_at_section_end,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x2a7f90, 0);
                c.g[A0][0] = OBJ_2 as u64;
                let (limit, start, end, ch) = match seed % 3 {
                    0 => (0u32, 0u32, 0u32, 0u8),
                    1 => (0x7fff_ffff, 0, OBJ_2 + 0x100, 0),
                    _ => (0x7fff_ffff, 0, OBJ_2 + 0x100, b'['),
                };
                w_2(c, OBJ_2, limit);
                w_2(c, OBJ_2 + 4, start);
                w_2(c, OBJ_2 + 8, end);
                c.bus.write8(OBJ_2 + 0x100, ch);
            },
        );
    }

    #[test]
    fn speech_instance_add_bank_matches() {
        check(
            ee::s08::f_002b0348__generated,
            speech_instance_add_bank,
            4,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x002b_0348, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (DATA + 0x4000) as u64;
                c.g[A2][0] = (r.next_u32() % 8) as u64;
                c.g[A3][0] = 0;
                c.bus.write8(DATA + 0x4000, b'q');
                c.bus.write8(DATA + 0x4001, 0);
                w(c, OBJ + 0xf0, seed % 3);
            },
        );
    }

    #[test]
    fn song_instance_fade_out_matches() {
        check(
            ee::s08::f_002b2418__generated,
            song_instance_fade_out,
            6,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x002b_2418, 3);
                c.g[A0][0] = OBJ as u64;
                c.f[12] = if seed % 2 == 0 {
                    0x3f80_0000
                } else {
                    r.edge_f32()
                };
            },
        );
    }

    #[test]
    fn bank_monitor_stop_by_owner_matches() {
        check(
            ee::s08::f_002ad4d8__generated,
            bank_monitor_stop_by_owner,
            12,
            &|c, seed| {
                machine(c, seed, 0x2a_d4d8, 2);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                let owner = 1 + (seed % 3);
                c.g[A1][0] = owner as u64;
                w(c, DATA + 0x2000 + 0x8ec, DATA + 0x3000);
                for i in 0..64u32 {
                    let inst = DATA + 0x3000 + 0xc0 * i;
                    let id = if i % 5 == 0 { owner } else { 1 + (i % 4) };
                    w(c, inst + 0x80, id);
                }
                c.pc = 0x002a_d4d8;
            },
        );
    }

    #[test]
    fn music_player_enable_song_matches() {
        check(
            ee::s08::f_002b3fe8__generated,
            music_player_enable_song,
            SEEDS_10,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2b3fe8, 0);
                be_library_2(c);
                let obj = DATA;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 40) as u64;
                w(c, obj + 0x3e8, seed % 70);
                w(c, obj + 0x3f8, r.next_u32());
                w(c, obj + 0x3fc, r.next_u32());
                for i in 0..5u32 {
                    w(
                        c,
                        obj + 0x420 + 4 * i,
                        if (seed + i) % 3 == 0 { 0 } else { 1 + i },
                    );
                }
                for i in 0..5u32 {
                    w(c, obj + 0x920 + 4 * i, r.next_u32() % 7);
                }
            },
        );
    }

    #[test]
    fn path_plat_remove_bank_matches() {
        check(
            ee::s08::f_002ae328__generated,
            path_plat_remove_bank,
            80,
            &|c, seed| {
                let _ = base_3(c, seed, 0x2ae3, 2);
                // 24 bank slots: +8 the platform id, +4 the SND bank handle.
                for i in 0..24u32 {
                    let e = 0x004d_3ef8 + 0x28 * i;
                    w(c, e + 8, i % 4);
                    w(
                        c,
                        e + 4,
                        if (seed / 4 + i) % 2 == 0 {
                            0
                        } else {
                            0xffff_ffff
                        },
                    );
                    w(c, e + 0x24, 0x1111_1111);
                }
                c.g[A0][0] = (seed % 4) as u64;
            },
        );
    }

    #[test]
    fn path_plat_get_status_matches() {
        check(
            ee::s08::f_002af2f0__generated,
            path_plat_get_status,
            50,
            &|c, seed| {
                let _ = base_3(c, seed, 0x2af2, 2);
                for i in 0..4u32 {
                    let e = 0x004d_3e98 + 4 * i;
                    if (seed / 4 + i) % 3 == 0 {
                        w(c, e, 0);
                    } else {
                        let bank = DATA + 0x400 + 0x80 * i;
                        w(c, e, bank);
                        wb(c, bank + 0x65, ((seed + i) % 3 == 0) as u8);
                        w(c, bank + 0x48, 0x10 + i);
                        w(c, bank + 0x4c, 0x20 + i);
                        w(c, bank + 0x5c, if seed % 2 == 0 { 0 } else { 0x30 + i });
                        w(c, bank + 0x60, if seed % 3 == 0 { 0x40 + i } else { 0 });
                    }
                }
                c.g[A0][0] = (seed % 4) as u64;
                c.g[A1][0] = sx(DATA + 0x200);
                c.g[A2][0] = sx(DATA + 0x300);
            },
        );
    }

    #[test]
    fn music_player_load_song_data_matches() {
        check(
            ee::s08::f_002b4a20__generated,
            music_player_load_song_data,
            50,
            &|c, seed| {
                let _ = base_3(c, seed, 0x2b4a, 3);
                let m = DATA;
                w(c, m + 0x5438, if seed % 3 == 0 { 0 } else { DATA + 0x800 });
                w(c, m + 0x5434, if seed % 4 == 0 { 0 } else { DATA + 0x900 });
                c.g[A0][0] = sx(m);
                c.g[A1][0] = sx(DATA + 0x200);
                // `a2 == 0`: the async loader. The EAFile path (a2 != 0) queues a
                // disc read and runs away on a test machine (0x3de768).
                c.g[A2][0] = 0;
            },
        );
    }

    #[test]
    fn speechsystem_shutdown_matches() {
        check(
            ee::s08::f_002b0da0__generated,
            speechsystem_shutdown,
            SEEDS_11,
            &|c, seed| {
                base_4(c, seed, 0xb0da0, 0);
                let s = DATA + 0x100;
                let insts = DATA + 0x800;
                c.g[A0][0] = sx(s);
                c.bus.write32(s + 4, insts);
                c.bus
                    .write32(insts, if seed % 2 == 0 { 0 } else { DATA + 0x900 });
                c.bus
                    .write32(insts + 4, if seed % 3 == 0 { 0 } else { DATA + 0x940 });
                c.bus
                    .write32(s + 8, if seed % 4 == 0 { 0 } else { DATA + 0xa00 });
            },
        );
    }

    // ------------------------------------------------------------------
    // Audio.

    #[test]
    fn path_plat_set_track_level_matches() {
        check(
            ssx3_ee::s08::f_002af580__generated,
            path_plat_set_track_level,
            SEEDS_12,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2af580, 4);
                be_library_3(c);
                world(c);
                const TABLE: u32 = 0x004d_3e98;
                let track = (r.next_u32() % 8) as u32;
                c.g[A0][0] = track as u64;
                c.g[A1][0] = (r.next_u32() % 0x100) as u64;
                if seed % 4 == 0 {
                    w(c, TABLE + 4 * track, 0);
                } else {
                    let snd = DATA + 0x300;
                    w(c, TABLE + 4 * track, snd);
                    w(c, snd, DATA + 0x400);
                    b(c, snd + 0x64, if seed % 2 == 0 { 0 } else { 1 });
                }
            },
        );
    }

    #[test]
    fn path_plat_add_bank_matches() {
        const TABLE: u32 = 0x004d_3ef8;
        check(
            ee::s08::f_002ae298__generated,
            path_plat_add_bank,
            120,
            &|c, seed| {
                let _ = base(c, seed, 0x2a_e298);
                globals(c);
                // Slot 0 is free (or holds bank 0), slot 1 free; the rest either
                // free (walk ends at the first free slot) or all taken.
                for i in 0..24u32 {
                    w(
                        c,
                        TABLE + 0x28 * i + 8,
                        if seed % 2 == 0 { 0 } else { 0x7fff_ffff },
                    );
                }
                if seed % 2 == 1 {
                    // Full table: the -13 path.
                    c.g[A0][0] = 5;
                    c.g[A2][0] = 7;
                } else {
                    c.g[A0][0] = 0;
                    c.g[A2][0] = 0;
                }
                c.g[A3][0] = sx(seed ^ 0x55);
            },
        );
    }

    #[test]
    fn path_plat_set_paused_matches() {
        check(
            ee::s08::f_002af7a0__generated,
            path_plat_set_paused,
            SEEDS_13,
            &alloc_base_setup(|c, seed, r| {
                let path = DATA + 0x800;
                let idx = seed % 4;
                w(c, 0x004d_3e98 + 4 * idx, path);
                wh(c, path + 0x3c, r.next_u32() as u16);
                w(c, path + 0x18, r.next_u32());
                wb(c, path + 0x64, (seed & 1) as u8);
                w(c, path + 0x6c, DATA + 0x900);
                w(c, DATA + 0x900 + 4, DATA + 0xa00);
                c.g[A0][0] = idx as u64;
                c.g[A1][0] = (seed & 2) as u64;
            }),
        );
    }

    #[test]
    fn music_player_load_loop_bank_matches() {
        check(
            ee::s08::f_002b43d8__generated,
            music_player_load_loop_bank,
            SEEDS_13,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                let song = DATA + 0x800;
                let bankman = DATA + 0x1200;
                w(c, this, bankman);
                w(c, bankman + 0xacc, DATA + 0x1300);
                if seed & 1 == 0 {
                    w(c, this + 0x408, 0);
                } else {
                    w(c, this + 0x408, song);
                    w(c, song, DATA + 0x1000);
                    w(c, DATA + 0x1000 + 0x40, if seed & 2 == 0 { 0 } else { 1 });
                    w(c, song + 0x34, DATA + 0x2000);
                    put_str(c, DATA + 0x2000, b"bank01");
                }
                c.g[S3][0] = 0;
                c.g[A0][0] = this as u64;
            }),
        );
    }

    /// A pool_3 block that `MEM_IsPoolPointer` accepts and `MEM_PoolFree` can
    /// take back.
    pub(crate) const POOL_BLOCK: u32 = DATA + 0x40_0000 + 0x40;

    #[test]
    fn song_instance_dtor_matches() {
        check(
            ssx3_ee::s08::f_002b1f78__generated,
            song_instance_dtor,
            30,
            &|c, seed| {
                let _r = base_6(c, seed, 0x2b1f78, 3);
                let this = DATA + 0x1000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed & 1) as u64;
                w_3(c, GAME_GP.wrapping_add(0x6fc), DATA + 0x2000);
                w_3(c, DATA + 0x2000, DATA + 0x2100);
                let obj = DATA + 0x3000;
                w_3(c, this, obj);
                w_3(c, obj + 0x40, (seed % 3) as u32);
                w_3(c, this + 0x34, 0);
                w_3(c, this + 0x24, if seed % 2 == 0 { 0 } else { POOL_BLOCK });
                w_3(c, this + 0x28, if seed % 5 == 0 { 0 } else { POOL_BLOCK });
            },
        );
    }

    // ------------------------------------------------------- speech instance

    #[test]
    fn spch_create_instance_matches() {
        check(
            ee::s08::f_002b0e60__generated,
            spch_create_instance,
            SEEDS_14,
            &|c, seed| {
                let _ = base_7(c, seed, 0x2b0e_60, 6);
                let obj = SCRATCH_2;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (obj + 0x1000) as u64;
                c.g[A3][0] = 2;
                c.g[T0][0] = 3;
                c.g[T1][0] = 4;
                c.g[T2][0] = 5;
                w(c, obj + 4, obj + 0x2000);
                w(c, obj + 0x1000 + 0x18, 0);
            },
        );
    }

    // -------------------------------------------------------- PATH platform

    #[test]
    fn path_plat_fade_volume_matches() {
        check_lohi(
            ee::s08::f_002af608__generated,
            path_plat_fade_volume,
            40,
            &|c, seed| {
                let mut r = base_8(c, seed, 0x2af6, 0);
                let idx = seed % 3;
                let obj = DATA + 0x1_000;
                w(c, 0x004d_3e98 + 4 * idx, obj);
                w(c, obj, DATA + 0x900);
                w(c, obj + 8, r.next_u32());
                w(c, obj + 0x6c, r.next_u32());
                wb(c, obj + 0x64, (seed % 3) as u8);
                c.g[A0][0] = idx as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = if seed % 4 == 0 { 1 } else { 0 };
            },
        );
    }

    #[test]
    fn music_player_stop_song_matches() {
        check(
            ee::s08::f_002b3ac0__generated,
            music_player_stop_song,
            SEEDS_15,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2b3a, 4);
                game_globals_2(c);
                let this = DATA;
                let song = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.bus.write32(this, DATA + 0x3000);
                c.bus.write32(DATA + 0x3000, DATA + 0x3400);
                c.bus.write32(DATA + 0x3400 + 0x1d8, 0);
                for (i, a) in [0x004a_3838u32, 0x004a_3840, 0x004a_3848]
                    .iter()
                    .enumerate()
                {
                    c.bus.write64(*a, 0x1000 * (i as u64 + 1));
                }
                c.bus.write32(this + 0x418, 0);
                if seed % 2 == 0 {
                    c.bus.write32(this + 0x408, 0);
                } else {
                    c.bus.write32(this + 0x408, song);
                    c.bus.write32(song + 0x34, seed % 5);
                }
            },
        );
    }

    #[test]
    fn bank_monitor_stop_by_callback_matches() {
        check_all(
            ee::s08::f_002ad410__generated,
            bank_monitor_stop_by_callback,
            SEEDS_15,
            &|c, seed| {
                let _ = machine(c, seed, 0x2ad4, 3);
                game_globals_2(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 8) as u64;
                c.bus.write32(this + 0x8ec, DATA + 0x2000);
                for i in 0..64u32 {
                    c.bus
                        .write16(DATA + 0x2000 + 0xc0 * i + 0xaa, (seed % 8) as u16);
                }
            },
        );
    }

    #[test]
    fn path_plat_remove_oldest_bank_matches() {
        check(
            ee::s08::f_002ae3a8__generated,
            path_plat_remove_oldest_bank,
            60,
            &|c, seed| {
                let _r = base_9(c, seed, 0x2ae3);
                c.g[A0][0] = DATA as u64;
                // No slot matches: the scan leaves the "none" sentinel.
                c.bus.write32(0x004d_3ef8 + 8, DATA + 0x1000);
            },
        );
    }

    #[test]
    fn csong_configuration_ctor_matches() {
        strict(
            ee::s08::f_002b1a98__generated,
            csong_configuration_ctor,
            40,
            &|c, seed| {
                let _r = base_9(c, seed, 0x2b1a);
                c.g[A0][0] = DATA as u64;
                for i in 0..8 {
                    c.bus.write8(
                        0x004a_35a8 + i,
                        (seed as u8).wrapping_mul(7).wrapping_add(i as u8),
                    );
                }
            },
        );
    }

    #[test]
    fn bank_monitor_stop_by_bank_matches() {
        diff(
            ee::s08::f_002ad650__generated,
            bank_monitor_stop_by_bank,
            30,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x2a_d650);
                let bank = (seed % 3) as u32;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = bank as u64;
                c.f[12] = if seed % 2 == 0 {
                    0.0f32.to_bits()
                } else {
                    uf(&mut r, 4.0)
                };
                w(c, DATA + 0x8ec, DATA + 0x9000);
                for i in 0..64u32 {
                    let inst = DATA + 0x9000 + i * 0xc0;
                    w(c, inst, if seed % 3 == 0 && i % 3 == 0 { 1 } else { 0 });
                    w(c, inst + 0xc, bank);
                    w(c, inst + 0x10, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn music_player_enable_song_by_name_matches() {
        strict_2(
            ssx3_ee::s08::f_002b3ee8__generated,
            music_player_enable_song_by_name,
            SEEDS,
            &|c, seed| {
                let mut r = base_10(c, seed, 0x3ee8);
                let player = DATA + 0x1000;
                let name = DATA + 0x3000;
                w(
                    c,
                    player + 0x3e8,
                    if seed % 5 == 0 { 64 } else { r.next_u32() % 60 },
                );
                w(c, player + 0x3e4, 1 + r.next_u32() % 2);
                w(c, player + 0x3f8, r.next_u32());
                w(c, player + 0x3fc, r.next_u32());
                for i in 0..4 {
                    w(c, player + 0x920 + 4 * i, r.next_u32() % 3);
                }
                c.bus.write8(name, if seed % 3 == 0 { b'A' } else { 0 });
                c.bus.write8(name + 1, 0);
                c.g[A0][0] = sx(player);
                c.g[A1][0] = sx(name);
            },
        );
    }
    use super::*;
    use super::*;
    use super::*;
    use super::*;

    // -- 0x002b0290 SpeechInstance_IsStreamReady -------------------------------

    /// `SpeechInstance_IsStreamReady` against the generated body, through the
    /// real stream array / stream buffer / SNDSTRM status calls (the SND lock
    /// hooks are fakes, some unwinding): an empty array slot (state 0), a
    /// stream in state 0, 1 or 2; in state 1 its request playing (status 2)
    /// or not, with 0..1000 ms left (around the 500 threshold).
    #[test]
    fn speech_instance_is_stream_ready_matches() {
        use super::speech_stream_ready::*;
        use crate::difftest::fakes::{covered, machine};
        use crate::difftest::{DATA, GAME_GP};
        const THIS: u32 = DATA + 0x100;
        const SPEECH: u32 = DATA + 0x200;
        const ARRAY: u32 = DATA + 0x300;
        const SLOTS: u32 = DATA + 0x400;
        const BUFFER: u32 = DATA + 0x500;
        const STREAM: u32 = DATA + 0x1000;
        const REQUEST: u32 = DATA + 0x1400;
        const SND_SLOT: u32 = DATA + 0x1800;
        /// SNDSTRM: the stream count (byte) and the stream table.
        const SNDSTRM_COUNT: u32 = 0x0051_0000 - 0x56d2;
        const SNDSTRM_STREAMS: u32 = 0x0051_0000 - 0x5f60;
        /// SNDSTRM: the per-slot table the buffered-bytes query reads.
        const SNDSTRM_SLOTS: u32 = 0x0051_57b8 + 0x304;
        covered(
            ee::s08::f_002b0290__generated,
            speech_instance_is_stream_ready,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002b_0290, 4);
                c.pc = 0x002b_0290;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                let slot = r.next_u32() % 4;
                c.g[A0][0] = sx(THIS);
                w(c, THIS + STREAM_SLOT, slot);
                w(c, GAME_GP + SPEECH_GP, SPEECH);
                w(c, SPEECH, ARRAY);
                w(c, ARRAY + 8, SLOTS);
                w(c, SLOTS + 4 * slot, if seed % 7 == 0 { 0 } else { BUFFER });
                // The buffer's SND stream handle 0.
                w(c, BUFFER + 8, 0);
                c.bus.write8(SNDSTRM_COUNT, 4);
                w(c, SNDSTRM_STREAMS, STREAM);
                // The stream: state, current request, rate (1000: the
                // remaining time is the request's length in ms), SND slot 0
                // with some bytes buffered.
                let state = [0, 1, 1, 1, 2][(r.next_u32() % 5) as usize];
                w(c, STREAM + 0x124, state);
                w(c, STREAM + 0x11c, REQUEST);
                c.bus.write16(STREAM + 0x14, 1000);
                w(c, STREAM + 8, 0);
                w(c, SNDSTRM_SLOTS, SND_SLOT);
                w(c, SND_SLOT + 0x48, 0x800 + r.next_u32() % 0x800);
                w(c, SND_SLOT + 0x4c, r.next_u32() % 0x800);
                // The request: id 0 (stream 0), playing or not, its length.
                w(c, REQUEST, 0);
                w(c, REQUEST + 0xc, 0);
                w(c, REQUEST + 0x10, 0);
                c.bus.write8(REQUEST + 0x24, (r.next_u32() % 3 != 0) as u8);
                w(c, REQUEST + 0x14, 0);
                w(c, REQUEST + 0x18, r.next_u32() % 1000);
                w(c, REQUEST + 0x1c, r.next_u32() % 1000);
            },
        );
    }

    // -- 0x002b2160 SongInstance_SetVolume -------------------------------------

    /// `SongInstance_SetVolume` against the generated body: volumes above
    /// 0xffff, negative, in range, and equal to the current one (nothing to
    /// do); the real `SongInstance_GetStreamHandle` finding the song's track
    /// (stream index valid or negative, stream handle valid or negative) or
    /// not; the real `SNDSTRM_SetLowpass` finds no stream. The SND lock
    /// hooks are fakes (some unwinding).
    #[test]
    fn song_instance_set_volume_matches() {
        use super::song_set_volume::*;
        use crate::difftest::fakes::{covered, machine};
        use crate::difftest::DATA;
        const THIS: u32 = DATA + 0x100;
        const TRACK: u32 = DATA + 0x200;
        const TRACK_DATA: u32 = DATA + 0x300;
        /// PATH: the track groups (`+0x30` flags, `+0x34` used, `+0x58`
        /// the tracks) and the per-stream track data pointers.
        const PATH_GROUPS: u32 = 0x0051_7610;
        const PATH_TRACK_DATA: u32 = 0x004d_3e98;
        covered(
            ee::s08::f_002b2160__generated,
            song_instance_set_volume,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002b_2160, 2);
                c.pc = 0x002b_2160;
                let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
                let current = r.next_u32() % 0x1_0000;
                w(c, THIS + VOLUME, current);
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = match seed % 6 {
                    0 => current as u64,
                    1 => 0x1_0000 + (r.next_u32() % 0x10_0000) as u64,
                    2 => (r.next_u32() | 0x8000_0000) as i32 as i64 as u64,
                    3 => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                    _ => (r.next_u32() % 0x1_0000) as u64,
                };
                // The song's track (key 0x11000001: group flags 0x1x000000, bit 0).
                if seed % 5 != 0 {
                    w(
                        c,
                        PATH_GROUPS + 0x30,
                        0x1100_0000 | (r.next_u32() & 0x00ff_ffff),
                    );
                    w(c, PATH_GROUPS + 0x34, 1);
                    w(c, PATH_GROUPS + 0x58, TRACK);
                }
                let index = if seed % 7 == 0 {
                    u32::MAX
                } else {
                    r.next_u32() % 4
                };
                w(c, TRACK + 0x54, index);
                for k in 0..4 {
                    w(c, PATH_TRACK_DATA + 4 * k, TRACK_DATA + 4 * k);
                    let handle = if r.next_u32() % 4 == 0 {
                        0xffff_fff8
                    } else {
                        r.next_u32() % 8
                    };
                    w(c, TRACK_DATA + 4 * k, handle);
                }
            },
        );
    }

    /// `PATH_PlatSetVolume`: path slot 0..23 of the table at 0x4d3e98 (the
    /// other slots empty) points at a record; the audio system (gp+0x410)
    /// is off on every sixth seed; the dirty flag is set on a third of the
    /// seeds (then the real `PATH_PlatRefreshStatus` runs its short path:
    /// stream handle negative); pending is -1, equal to or different from
    /// the current volume; the new volume is negative, equal to the
    /// current one or different; the current one is sometimes negative.
    /// `SND_Lock`/`SND_Unlock` run for real with their hook pointers
    /// (0x44c46c/0x44c470) at fakes; `SND_SetVolume` gets a negative channel
    /// handle and `SNDSTRM_GetStream` finds no stream (count 0), so both
    /// return early.
    #[test]
    fn path_plat_set_volume_matches() {
        use crate::difftest::GAME_GP;
        const PATH: u32 = DATA + 0x1000;
        const SOUND: u32 = DATA + 0x2000;
        const AUDIO: u32 = DATA + 0x10000;
        covered(
            ssx3_ee::s08::f_002af6c0__generated,
            path_plat_set_volume,
            300,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x2af6c0, 4);
                c.pc = 0x002a_f6c0;
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP + 0x410, AUDIO);
                c.bus.write32(
                    AUDIO + 0x6280,
                    if seed % 6 == 0 { 0 } else { r.next_u32() | 1 },
                );
                let slot = r.next_u32() % 24;
                for i in 0..24 {
                    c.bus
                        .write32(0x004d_3e98 + 4 * i, if i == slot { PATH } else { 0 });
                }
                c.g[A0][0] = slot as u64 | (r.next_u32() as u64) << 32;
                // SND_Lock/SND_Unlock hooks, stream count.
                c.bus.write32(0x0044_c46c, FAKE);
                c.bus.write32(0x0044_c470, FAKE + 0x10);
                c.bus.write8(0x0050_a92e, 0);
                let current = match seed % 5 {
                    0 => (r.next_u32() | 0x8000_0000) as i32,
                    _ => (r.next_u32() % 128) as i32,
                };
                let pending = match (seed / 5) % 3 {
                    0 => -1,
                    1 => current,
                    _ => current.wrapping_add(1 + (r.next_u32() % 50) as i32),
                };
                let volume = match (seed / 3) % 4 {
                    0 => -1 - (r.next_u32() % 100) as i32,
                    1 => current,
                    _ => (r.next_u32() % 128) as i32,
                };
                c.g[A1][0] = volume as i64 as u64;
                c.bus.write32(PATH, r.next_u32() | 0x8000_0000);
                c.bus.write32(PATH + 0x8, pending as u32);
                c.bus.write32(PATH + 0xc, current as u32);
                c.bus.write8(
                    PATH + 0x64,
                    (seed % 2) as u8 * (1 + r.next_u32() % 200) as u8,
                );
                c.bus.write8(PATH + 0x65, if seed % 3 == 0 { 1 } else { 0 });
                c.bus.write32(PATH + 0x6c, SOUND);
                c.bus.write32(SOUND + 4, r.next_u32() | 0x8000_0000);
            },
        );
    }

    /// `SpeechSystem_Update`: two instance slots, each empty or an instance
    /// (stream index 0..3, "started" flag 0..3) for the real
    /// `SpeechInstance_Update`: the speech listener (gp+0x6c4) has a stream
    /// array with no buffers (state 0: the instance is ready), its vtable
    /// slot 0x20 is a fake, `SPCH_ServiceChannel` is busy (gp+0x175c set)
    /// and `SPCH_DequeueChannel` finds an empty channel table (0x44fff8), so
    /// all return early; the real `SpeechSystem_TickEventContexts` counts
    /// down ten random contexts.
    #[test]
    fn speech_system_update_matches() {
        use crate::difftest::GAME_GP;
        const SYS: u32 = DATA;
        const SLOTS: u32 = DATA + 0x400;
        const INST: u32 = DATA + 0x1000;
        const LISTENER: u32 = DATA + 0x4000;
        const ARRAY: u32 = DATA + 0x4400;
        const BUFFERS: u32 = DATA + 0x4500;
        const LVT: u32 = DATA + 0x4600;
        covered(
            ssx3_ee::s08::f_002b0c78__generated,
            speech_system_update,
            300,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x2b0c78, 2);
                c.pc = 0x002b_0c78;
                c.g[A0][0] = sx(SYS);
                c.bus.write32(SYS + 4, SLOTS);
                for i in 0..2u32 {
                    let inst = INST + 0x1000 * i;
                    let present = (seed >> i) % 4 != 0;
                    c.bus.write32(SLOTS + 4 * i, if present { inst } else { 0 });
                    c.bus.write32(inst + 4, r.next_u32() % 4);
                    c.bus.write32(inst + 0x900, r.next_u32() % 4);
                    c.bus.write32(0x0044_fff8 + 4 * i, 0);
                }
                c.bus.write32(GAME_GP + 0x6c4, LISTENER);
                c.bus.write32(GAME_GP + 0x175c, 1);
                c.bus.write32(LISTENER, ARRAY);
                c.bus.write32(ARRAY + 8, BUFFERS);
                for k in 0..4 {
                    c.bus.write32(BUFFERS + 4 * k, 0);
                }
                c.bus.write32(LISTENER + 0x1bc, LVT);
                c.bus.write16(LVT + 0x20, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(LVT + 0x24, FAKE);
                for k in 0..10 {
                    let ctx = SYS + 0x18 + 0x20 * k;
                    c.bus.write32(ctx, r.next_u32() % 2);
                    c.bus.write32(ctx + 4, (r.next_u32() % 4).wrapping_sub(1));
                }
            },
        );
    }

    /// Four track slots with random loaded flags and class masks (often
    /// matching the id's nibbles), random track pointers, ids with and
    /// without low bits.
    #[test]
    fn path_find_track_matches() {
        use path::*;
        check(
            ssx3_ee::s14::f_003d5290__generated,
            path_find_track,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d5290, 0);
                let kind = (1 + r.next_u32() % 15) << 28;
                let group = (r.next_u32() % 16) << 24;
                let low = match seed % 4 {
                    0 => 0,
                    1 => 1 << (r.next_u32() % 24),
                    _ => r.next_u32() & 0xff_ffff,
                };
                let mut id = (kind | group | low) as u64;
                if seed % 5 == 0 {
                    id |= (r.next_u32() as u64) << 32;
                }
                c.g[A0][0] = id;
                for i in 0..SLOT_COUNT as u32 {
                    let slot = SLOTS + i * SLOT_SIZE;
                    c.bus.write32(slot + LOADED, (r.next_u32() % 3 != 0) as u32);
                    let mask = match r.next_u32() % 4 {
                        0 => r.next_u32(),
                        1 => kind,
                        _ => kind | group | (r.next_u32() & 0x00ff_ffff),
                    };
                    c.bus.write32(slot + MASK, mask);
                    for k in 0..24 {
                        c.bus.write32(slot + TRACKS + 4 * k, r.next_u32());
                    }
                }
                c.pc = 0x003d_5290;
            },
        );
    }

    /// A PATH state with `count` queues for PATH_ServiceQueues: heads that
    /// are pending (actions left, already run so no PATH_ExecuteAction),
    /// done, expired or forever, a zero head that ends the scan, slots past
    /// the count; the removal callback (0x44f428) a fake or none. Queue
    /// heads live outside the state's action pool, so PATH_RemoveEvent
    /// frees nothing.
    pub(crate) fn path_queues_fixture(c: &mut Cpu, r: &mut Rng, seed: u32) {
        let state = DATA + 0x1000;
        let now = 1000 + r.next_u32() % 1000;
        c.bus.write32(0x0045_0000 - PATH_STATE_OFF, state);
        c.bus.write32(0x0045_0000 - PATH_TIME_OFF, now);
        let callback = if seed % 2 == 0 {
            crate::difftest::fakes::FAKE
        } else {
            0
        };
        c.bus.write32(0x0045_0000 - 0xbd8, callback);
        let count = r.next_u32() % 17;
        c.bus.write32(state + PATH_QUEUE_COUNT, count);
        for i in 0..16u32 {
            let node = DATA + 0x10000 + i * 0x800;
            let head = if i >= count {
                r.next_u32() | 1 // cleared, never read
            } else if r.next_u32() % 12 == 0 {
                0
            } else {
                node
            };
            c.bus.write32(state + PATH_QUEUES + 4 * i, head);
            c.bus
                .write32(node + QUEUE_START, now.wrapping_sub(r.next_u32() % 300));
            let lifetime = if r.next_u32() % 2 == 0 {
                0
            } else {
                r.next_u32() % 400
            };
            c.bus.write32(node + QUEUE_LIFETIME, lifetime);
            c.bus.write32(node + 0x8, r.next_u32());
            let end = r.next_u32() % 5;
            let next = r.next_u32() % 5;
            let indices = (r.next_u32() & 0xf000_ffff) | end << 16 | next << 22;
            c.bus.write32(node + QUEUE_INDICES, indices);
            for k in 0..8u32 {
                let action = node + 0x10 + 16 * k;
                c.bus.write32(action + 4, 0);
                c.bus.write32(action + 8, r.next_u32() | 0x0400_0000);
            }
        }
    }

    crate::difftest::events::driven!(gen_path_service_queues, ee::s14::f_003d1e80__generated);

    crate::difftest::events::driven!(hand_path_service_queues, path_service_queues);

    /// PATH_ServiceQueues with the real PATH_ServiceQueue / PATH_RemoveEvent
    /// (fake callback, which may unwind), then with events delivered and
    /// unwinding at the loop heads (re-entered at 0x3d1f1c / 0x3d1ec0).
    #[test]
    fn path_service_queues_matches() {
        check(
            ee::s14::f_003d1e80__generated,
            path_service_queues,
            240,
            &|c, seed| {
                let mut r = crate::difftest::fakes::machine(c, seed, 0x3d1e80, 3);
                c.pc = SERVICE_QUEUES_ENTRY;
                path_queues_fixture(c, &mut r, seed);
            },
        );
        crate::difftest::events::check_events(
            gen_path_service_queues,
            hand_path_service_queues,
            120,
            &|c, seed| {
                let mut r = crate::difftest::fakes::machine(c, seed, 0x3d1e81, 0);
                path_queues_fixture(c, &mut r, seed / 3);
                let mode = crate::difftest::events::schedule(c, &mut r, seed, SERVICE_QUEUES_ENTRY);
                if mode != crate::difftest::events::Events::Off {
                    // Early enough for the short scans too.
                    c.next_event = c.cycles + 4 + (r.next_u32() % 40) as u64;
                }
                mode
            },
        );
    }
    use super::*;
    use super::*;
    use super::*;
    use super::*;

    // -- 0x003d54e8 PATH_UpdateTick42 ------------------------------------------

    /// `PATH_UpdateTick42` against the generated body, with the real
    /// `PATH_Update` (fixture as in `path_update_matches`): locked /
    /// uninitialised / busy early outs, and the full tick over four players
    /// that are inactive, active without a handle, or active with a handle.
    #[test]
    fn path_update_tick42_matches() {
        use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
        check(
            ee::s14::f_003d54e8__generated,
            path_update_tick42,
            120,
            &|c, seed| {
                let mut r = Rng(0x003d_54e8 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x003d_54e8;
                let gate = seed % 6;
                c.bus.write8(
                    0x0044_f42d,
                    (gate == 0) as u8 * (1 + r.next_u32() as u8 % 3),
                );
                c.bus.write8(0x0044_f42c, (gate != 1) as u8);
                c.bus
                    .write32(0x0044_f430, if gate == 2 { r.next_u32() | 1 } else { 0 });
                c.bus.write32(0x0045_0dcc, r.next_u32()); // timer count
                c.bus.write32(0x0045_0dc8, 1 + r.next_u32() % 0x1000); // timer period
                for i in 0..4u32 {
                    let p = 0x0051_7610 + 0x928 * i;
                    let kind = r.next_u32() % 3;
                    c.bus
                        .write32(p + 0x34, (kind != 0) as u32 * (1 + r.next_u32() % 5));
                    c.bus
                        .write32(p + 0x30, if kind == 2 { DATA + 0x100 * i } else { 0 });
                }
            },
        );
    }

    /// MusicPlayer_PathCmdA: inactive player, no song, or a song whose PATH
    /// command runs the real SongInstance_PathCmdA / PATH_SetTrackControlValue
    /// (PATH player not initialised: its early out).
    #[test]
    fn music_player_path_cmd_a_matches() {
        const SELF: u32 = DATA + 0x20_0000;
        check(
            ee::s08::f_002b3c28__generated,
            music_player_path_cmd_a,
            240,
            &|c, seed| {
                let mut r = Rng(0x002b_3c28 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x002b_3c28;
                c.g[A0][0] = sx(SELF);
                c.bus.write32(
                    SELF + MP_ACTIVE,
                    if seed % 3 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(
                    SELF + MP_SONG,
                    if seed % 3 == 1 { 0 } else { SELF + 0x1000 },
                );
                c.bus.write8(0x0044_f42c, 0); // PATH player not initialised
            },
        );
    }

    /// PATH_PlatGetQueuedEntryValue: empty and filled monitor slots, stale
    /// status (the real PATH_PlatRefreshStatus over 24 slots whose monitors
    /// have no stream; fake SND lock hooks), negative / out-of-range /
    /// in-range sub indices with and without a handle.
    #[test]
    fn path_plat_get_queued_entry_value_matches() {
        const BANKS: u32 = DATA + 0x20_0000;
        const SUBS: u32 = DATA + 0x20_1000;
        check(
            ee::s08::f_002aeaa8__generated,
            path_plat_get_queued_entry_value,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_eaa8, 0);
                c.pc = 0x002a_eaa8;
                c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
                c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
                for i in 0..4u32 {
                    let e = PQ_MON_TABLE + 4 * i;
                    if (seed / 4 + i) % 5 == 0 {
                        c.bus.write32(e, 0);
                        continue;
                    }
                    let bank = BANKS + 0x100 * i;
                    let subs = SUBS + 0x100 * i;
                    c.bus.write32(e, bank);
                    c.bus.write32(bank, u32::MAX); // no stream: refresh skips it
                    c.bus
                        .write8(bank + PQ_MON_STALE, ((seed + i) % 4 == 0) as u8);
                    c.bus
                        .write8(bank + PQ_MON_SUB_COUNT, (r.next_u32() % 6) as u8);
                    c.bus.write32(bank + PQ_MON_SUBS, subs);
                    c.bus.write32(bank + PQ_MON_VALUE, r.next_u32());
                    for j in 0..8u32 {
                        let h = if r.next_u32() % 3 == 0 {
                            0x8000_0000 | r.next_u32()
                        } else {
                            r.next_u32() >> 1
                        };
                        c.bus.write32(subs + 0xc * j + PQ_SUB_HANDLE, h);
                        c.bus.write32(subs + 0xc * j + PQ_SUB_VALUE, r.next_u32());
                    }
                }
                c.g[A0][0] = (seed % 4) as u64;
                c.g[A1][0] = match seed % 5 {
                    0 => sx(0x8000_0000 | r.next_u32()),
                    1 => sx(r.next_u32() >> 1),
                    _ => (r.next_u32() % 7) as u64,
                };
            },
        );
    }

    /// MusicPlayer_SetVolumePercent: inactive player, no song, or a song
    /// with and without a PATH track (+0x54) through the real
    /// SongInstance_SetVolumePercent / PATH call (PATH player not
    /// initialised: its early out); any percent byte (sign-extended).
    #[test]
    fn music_player_set_volume_percent_matches() {
        const SELF: u32 = DATA + 0x20_0000;
        const SONG: u32 = SELF + 0x1000;
        check(
            ee::s08::f_002b3d10__generated,
            music_player_set_volume_percent,
            240,
            &|c, seed| {
                let mut r = Rng(0x002b_3d10 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x002b_3d10;
                c.g[A0][0] = sx(SELF);
                c.bus.write32(
                    SELF + MP_ACTIVE,
                    if seed % 3 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus
                    .write32(SELF + MP_SONG, if seed % 3 == 1 { 0 } else { SONG });
                c.bus.write32(
                    SONG + 0x54,
                    if seed % 2 == 0 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write8(0x0044_f42c, 0); // PATH player not initialised
            },
        );
    }

    /// BankMonitor_AddInstance: a monitor with 64 instances, some free (the
    /// real FindFreeInstance/PollFree/Reset run), sound types 0..3 with
    /// filled per-type tables, a fade entry or not, a source or not (the
    /// owner's distance method is a fake); every fourth seed has no free
    /// instance and stops the sound (types 0..3, invalid handles take the
    /// SND library's short paths).
    #[test]
    fn bank_monitor_add_instance_matches() {
        const THIS: u32 = DATA + 0x2_0000;
        const TABLES: u32 = DATA + 0x1000;
        const SOURCES: u32 = DATA + 0x3000;
        const INSTANCES: u32 = DATA + 0x4000;
        const VTABLE: u32 = DATA + 0x8000;
        check(
            ee::s08::f_002acfe0__generated,
            bank_monitor_add_instance,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2acf_e0, 1);
                c.pc = 0x002a_cfe0;
                c.g[A0][0] = sx(THIS);
                let id = if seed % 5 == 0 {
                    u32::MAX
                } else {
                    r.next_u32()
                };
                c.g[A1][0] = sx(id);
                c.g[A2][0] = sx(r.next_u32());
                c.g[A3][0] = sx(seed % 4);
                let kind = r.next_u32() % 4;
                c.bus.write32(THIS + OC264_BM_KIND, kind);
                c.bus.write32(THIS + OC264_BM_ADD_COUNT, r.next_u32() % 3);
                c.bus.write32(THIS + OC264_BM_INSTANCES, INSTANCES);
                c.bus.write32(THIS + 0x8d8, VTABLE);
                c.bus.write16(VTABLE + 0x30, (r.next_u32() % 0x40) as u16);
                c.bus.write32(VTABLE + 0x34, FAKE);
                // Inline per-type source pointers (0 = no source) and params.
                for k in 0..4u32 {
                    let src = if r.next_u32() % 3 == 0 {
                        0
                    } else {
                        SOURCES + 0x10 * k
                    };
                    c.bus.write32(THIS + OC264_BM_SOURCES + 4 * k, src);
                }
                fill(c, &mut r, THIS + OC264_BM_PARAMS, 8);
                fill(c, &mut r, SOURCES, 0x40 / 4);
                // Per-type tables of 4 words each.
                let fields = [
                    OC264_BM_TABLE_FADE,
                    OC264_BM_TABLE_SLOT,
                    OC264_BM_TABLE_80,
                    OC264_BM_TABLE_84,
                    OC264_BM_TABLE_88,
                    OC264_BM_TABLE_8C,
                    OC264_BM_TABLE_94,
                    OC264_BM_TABLE_98,
                    OC264_BM_TABLE_9C,
                    OC264_BM_TABLE_A0,
                    OC264_BM_TABLE_A4,
                ];
                for (i, &f) in fields.iter().enumerate() {
                    let table = TABLES + 0x10 * i as u32;
                    c.bus.write32(THIS + f, table);
                    fill(c, &mut r, table, 4);
                }
                if seed % 2 == 0 {
                    c.bus.write32(TABLES + 4 * kind, 0); // no fade for this type
                }
                // Instances: free (type 0) or busy (type != 0, not done).
                let all_busy = seed % 4 == 0;
                for i in 0..64u32 {
                    let inst = INSTANCES + 0xc0 * i;
                    let busy = all_busy || r.next_u32() % 3 == 0;
                    c.bus
                        .write32(inst, if busy { 1 + r.next_u32() % 3 } else { 0 });
                    c.bus.write32(inst + 0x9c, 0);
                    c.bus.write32(inst + 0x54, r.next_u32() % 2);
                }
            },
        );
    }

    /// SongInstance_SetVolumePercent: playing or not; the real
    /// PATH_SetVolumePercent takes its short exits (not initialised, or a
    /// map selected); the volume a random 64-bit value (sign-extended byte).
    #[test]
    fn song_instance_set_volume_percent_matches() {
        const PATH_INITED: u32 = 0x0044_f42c;
        const PATH_SELECTED: u32 = 0x0044_f430;
        check(
            ee::s08::f_002b23e8__generated,
            song_instance_set_volume_percent,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x2b_23e8, 0);
                let song = DATA + 0x200;
                let playing = if seed % 3 == 0 { 0 } else { r.next_u32() | 1 };
                c.bus.write32(song + 0x54, playing);
                c.bus.write8(PATH_INITED, (seed % 2) as u8);
                c.bus.write32(PATH_SELECTED, r.next_u32() | 1);
                c.g[A0][0] = sx(song);
                c.g[A1][0] = r.next_u32() as u64 | (r.next_u32() as u64) << 32;
                c.pc = 0x002b_23e8;
            },
        );
    }
    use super::*;
    use super::*;

    // ----------------------------------------------- PATH_TrackGetQueuedCount

    /// `PATH_TrackGetQueuedCount`: 0..7 queued entries (count field among
    /// random flag bits) over the real `PATH_PlatGetQueuedEntryValue` with
    /// the same monitor fixture as its own test: empty and filled monitor
    /// slots (an empty one gives -8: skipped), stale status, sub handles
    /// with the sign bit (negative values skipped).
    #[test]
    fn path_track_get_queued_count_matches() {
        use crate::difftest::fakes::FAKE;
        use crate::game::sound::bankmonitor::{
            PQ_MON_STALE, PQ_MON_SUBS, PQ_MON_SUB_COUNT, PQ_MON_TABLE, PQ_MON_VALUE, PQ_SUB_HANDLE,
            PQ_SUB_VALUE,
        };
        const TRACK: u32 = DATA + 0x20_0000;
        const BANKS: u32 = DATA + 0x21_0000;
        const SUBS: u32 = DATA + 0x21_1000;
        check(
            ee::s14::f_003d4950__generated,
            path_track_get_queued_count,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_4950, 0);
                c.pc = 0x003d_4950;
                c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
                c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
                for i in 0..4u32 {
                    let e = PQ_MON_TABLE + 4 * i;
                    if (seed / 8 + i) % 5 == 0 {
                        c.bus.write32(e, 0);
                        continue;
                    }
                    let bank = BANKS + 0x100 * i;
                    let subs = SUBS + 0x100 * i;
                    c.bus.write32(e, bank);
                    c.bus.write32(bank, u32::MAX); // no stream: refresh skips it
                    c.bus
                        .write8(bank + PQ_MON_STALE, ((seed + i) % 4 == 0) as u8);
                    c.bus
                        .write8(bank + PQ_MON_SUB_COUNT, (r.next_u32() % 8) as u8);
                    c.bus.write32(bank + PQ_MON_SUBS, subs);
                    c.bus.write32(bank + PQ_MON_VALUE, r.next_u32());
                    for j in 0..8u32 {
                        let h = if r.next_u32() % 3 == 0 {
                            0x8000_0000 | r.next_u32()
                        } else {
                            r.next_u32() >> 1
                        };
                        c.bus.write32(subs + 0xc * j + PQ_SUB_HANDLE, h);
                        c.bus
                            .write32(subs + 0xc * j + PQ_SUB_VALUE, r.next_u32() >> 4);
                    }
                }
                let count = seed % 8;
                let flags = (r.next_u32() & !(0x1f << 14)) | count << 14;
                c.bus.write32(TRACK + PATH_TRACK_FLAGS, flags);
                c.bus.write32(TRACK + PATH_TRACK_SLOT, (seed / 2) % 4);
                c.g[A0][0] = sx(TRACK);
            },
        );
    }

    /// PATH_PlatGetTrackState: an empty slot (-8), a clean track, or a dirty
    /// one (the real PATH_PlatRefreshStatus runs first); fake hooks unwind on
    /// some seeds.
    #[test]
    fn path_plat_get_track_state_matches() {
        use crate::difftest::fakes::machine;
        check(
            ee::s08::f_002aebd0__generated,
            path_plat_get_track_state,
            120,
            &|c, seed| {
                let _ = machine(c, seed, 0x2a_ebd0, 2);
                c.pc = 0x002a_ebd0;
                for i in 0..4u32 {
                    let e = PATH_PLAT_TRACKS + 4 * i;
                    if (seed / 4 + i) % 3 == 0 {
                        c.bus.write32(e, 0);
                    } else {
                        let track = DATA + 0x400 + 0x80 * i;
                        c.bus.write32(e, track);
                        c.bus
                            .write8(track + PATH_PLAT_TRACK_DIRTY, ((seed + i) % 3 == 0) as u8);
                        c.bus.write32(track + 0x48, 0x10 + i);
                        c.bus.write32(track + 0x4c, 0x20 + i);
                        c.bus.write32(track + PATH_PLAT_TRACK_STATE, 0x50 + i);
                        c.bus
                            .write32(track + 0x5c, if seed % 2 == 0 { 0 } else { 0x30 + i });
                        c.bus
                            .write32(track + 0x60, if seed % 3 == 0 { 0x40 + i } else { 0 });
                    }
                }
                c.g[A0][0] = (seed % 4) as u64;
            },
        );
    }
    use super::*;
    use super::*;
    use super::*;

    /// `PATH_SelectBranch`: a network (at 0x44f420) whose node `a0` has
    /// 0..9 branches (sometimes 31: past the written ones, zeros) with small
    /// lane ranges, a tracker queued on that node or elsewhere, positive or
    /// non-positive node counts (lane consumed, tracker released at -1), and
    /// lanes inside or outside every range (nearest-branch search).
    #[test]
    fn path_select_branch_matches() {
        use path_f::*;
        const NET: u32 = DATA;
        const IDX: u32 = DATA + 0x800;
        const BASE: u32 = DATA + 0x1000;
        const TRACKER: u32 = DATA + 0x2000;
        check(
            ee::s14::f_003d37a0__generated,
            path_select_branch,
            300,
            &|c, seed| {
                let mut r = Rng(0x3d37_a0b1 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003d_37a0;
                c.bus.write32(NETWORK, NET);
                c.bus.write32(NET + NODE_BASE, BASE);
                c.bus.write32(NET + NODE_INDEX, IDX);
                for k in 0..32 {
                    c.bus.write32(NET + TRACKERS + 4 * k, TRACKER);
                }
                let node = r.next_u32() % 16;
                let word = 4 + r.next_u32() % 32;
                c.bus.write16(IDX + 2 * node, word as u16);
                let rec = BASE + 4 * word;
                let count = match seed % 4 {
                    0 => 0,
                    1 => 8 + r.next_u32() % 8, // negative count
                    _ => 1 + r.next_u32() % 7,
                };
                let slot = r.next_u32() % 32;
                c.bus
                    .write32(rec, count << 28 | slot << 16 | (r.next_u32() & 0xffff));
                let branches = match seed % 13 {
                    0 => 0,
                    1 => 31,
                    _ => 1 + r.next_u32() % 9,
                };
                c.bus
                    .write32(rec + NODE_WORD1, (r.next_u32() & !0x1f000) | branches << 12);
                for i in 0..9 {
                    let lo = (r.next_u32() % 21) as i32 - 10;
                    let hi = lo + (r.next_u32() % 6) as i32 - 1;
                    let e = rec + BRANCHES + 4 * i;
                    c.bus.write8(e, lo as u8);
                    c.bus.write8(e + 1, hi as u8);
                    c.bus.write16(e + 2, r.next_u32() as u16);
                }
                let on_node = seed % 3 != 0;
                c.bus.write16(
                    TRACKER + TRACKER_NODE,
                    if on_node {
                        node as u16
                    } else {
                        (node + 1) as u16
                    },
                );
                let lane_word = if seed % 5 == 0 {
                    r.next_u32() & !0xf // lane 0: released after the decrement
                } else {
                    r.next_u32()
                };
                c.bus.write32(TRACKER + TRACKER_LANE, lane_word);
                c.g[A0][0] = node as u64;
                c.g[A1][0] = if seed % 7 == 0 {
                    sx(r.next_u32())
                } else {
                    sx(((r.next_u32() % 25) as i32 - 12) as u32)
                };
            },
        );
    }

    /// `BankMonitor_FindFreeInstance`: 64 instances, each free (word 0
    /// is 0) or busy (word 0 set, nothing playing at +0x9c), asking for
    /// the 0th..5th free one (0 never matches: -1 after all 64), so the
    /// found and not-found exits occur. PollFree runs for real on its
    /// short paths.
    #[test]
    fn bank_monitor_find_free_instance_matches() {
        const THIS: u32 = DATA;
        const INSTANCES: u32 = DATA + 0x1000;
        check(
            ee::s08::f_002acf60__generated,
            bank_monitor_find_free_instance,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002a_cf60, 0);
                c.pc = 0x002a_cf60;
                c.bus.write32(THIS + BANK_MONITOR_INSTANCES, INSTANCES);
                let busy_odds = 1 + seed % 8;
                for i in 0..64 {
                    let inst = INSTANCES + i * BANK_MONITOR_INSTANCE_SIZE;
                    let busy = r.next_u32() % 8 < busy_odds;
                    c.bus.write32(inst, if busy { r.next_u32() | 1 } else { 0 });
                    c.bus.write32(inst + 0x9c, 0);
                }
                c.g[A0][0] = sx(THIS);
                c.g[A1][0] = sx(seed % 6);
            },
        );
    }

    /// `PATH_UpdateTick20` against the generated body, with the real
    /// `PATH_Update` (fixture as in `path_update_tick42_matches`): locked /
    /// uninitialised / busy early outs, and the full tick over four players
    /// that are inactive, active without a handle, or active with a handle.
    #[test]
    fn path_update_tick20_matches() {
        use crate::difftest::scramble;
        check(
            ee::s14::f_003d54c8__generated,
            path_update_tick20,
            120,
            &|c, seed| {
                let mut r = Rng(0x003d_54c8 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x003d_54c8;
                let gate = seed % 6;
                c.bus.write8(
                    0x0044_f42d,
                    (gate == 0) as u8 * (1 + r.next_u32() as u8 % 3),
                );
                c.bus.write8(0x0044_f42c, (gate != 1) as u8);
                c.bus
                    .write32(0x0044_f430, if gate == 2 { r.next_u32() | 1 } else { 0 });
                c.bus.write32(0x0045_0dcc, r.next_u32()); // timer count
                c.bus.write32(0x0045_0dc8, 1 + r.next_u32() % 0x1000); // timer period
                for i in 0..4u32 {
                    let p = 0x0051_7610 + 0x928 * i;
                    let kind = r.next_u32() % 3;
                    c.bus
                        .write32(p + 0x34, (kind != 0) as u32 * (1 + r.next_u32() % 5));
                    c.bus
                        .write32(p + 0x30, if kind == 2 { DATA + 0x100 * i } else { 0 });
                }
            },
        );
    }

    /// PATH_PlatQueue: in-memory slots with no bank slot, a negative bank and
    /// a played patch; streaming slots with a stop, the file-offset queue,
    /// the plain queue failing (no stream) and succeeding (a real
    /// SNDSTRM_QueueRequest -> StreamFile_QueueRange chain on a fake stream
    /// file), with 0..4 sub-streams; volume set or not.
    #[test]
    fn path_plat_queue_matches() {
        use crate::difftest::fakes::{covered, machine};
        use crate::difftest::DATA;
        const MON: u32 = DATA + 0x2000;
        const SUBS: u32 = DATA + 0x2400;
        const STREAM: u32 = DATA + 0x3000;
        const FILE: u32 = DATA + 0x3400;
        const NODE: u32 = DATA + 0x3600;
        const HEADER: u32 = DATA + 0x3800;
        const REQ: u32 = DATA + 0x4000;
        const NAME: u32 = DATA + 0x5000;
        /// SNDSTRM stream table and count (`0x510000 - 0x5f60`, `- 0x56d2`).
        const STRM_TABLE: u32 = 0x0050_a0a0;
        const STRM_COUNT: u32 = 0x0050_a92e;
        covered(
            ssx3_ee::s08::f_002ae7a8__generated,
            path_plat_queue,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ae7a8, 3);
                c.pc = 0x002a_e7a8;
                let track = r.next_u32() % 24;
                let case = seed % 8;
                c.bus.write32(PLAT_QUEUE_TRACKS + 4 * track, MON);
                c.g[A0][0] = track as u64;
                c.g[A1][0] = sx(r.next_u32());
                c.g[A2][0] = sx(NAME);
                c.bus.write32(NAME, 0x006b_7274); // "trk"
                let info = r.next_u32();
                c.g[A3][0] = sx(info);
                c.g[T0][0] = sx(r.next_u32()); // tag
                c.g[T1][0] = sx(if (seed / 8) % 2 == 0 {
                    r.next_u32() & 0x7f
                } else {
                    0x80 | r.next_u32()
                });
                c.g[T2][0] = sx(match (seed / 16) % 3 {
                    0 => (r.next_u32() % 4).wrapping_neg(),
                    _ => r.next_u32() % 0x1000,
                });
                c.g[T3][0] = (case == 3 || (case == 5 && seed % 3 == 0)) as u64;
                // Timer for PATH_PlatGetTimeMs.
                c.bus.write32(0x0045_0dc8, 1000);
                c.bus.write32(0x0045_0dcc, r.next_u32() % 100_000);
                // Bank slots: none owned by the track unless the case says so.
                for i in 0..24 {
                    c.bus
                        .write32(PLAT_QUEUE_BANK_SLOTS + 0x28 * i + 8, u32::MAX);
                }
                let count = r.next_u32() % 5;
                c.bus.write8(MON + 0x66, count as u8);
                c.bus.write32(MON + 0x6c, SUBS);
                for i in 0..16 {
                    c.bus.write32(SUBS + 4 * i, r.next_u32());
                }
                c.bus.write8(MON + 0x64, (case < 3) as u8);
                let handle = r.next_u32() % 4;
                c.bus.write32(MON, handle);
                match case {
                    0 => {}
                    1 | 2 => {
                        let k = r.next_u32() % 24;
                        let e = PLAT_QUEUE_BANK_SLOTS + 0x28 * k;
                        c.bus.write32(e + 8, track);
                        c.bus.write32(e, info >> 16);
                        let bank = if case == 1 {
                            0x8000_0000 | r.next_u32()
                        } else {
                            r.next_u32() % 4
                        };
                        c.bus.write32(e + 4, bank);
                    }
                    _ => {
                        let pos = info << 7;
                        let (base, limit) = match case {
                            4 => (1 | r.next_u32(), pos.wrapping_add(1).max(1)),
                            5 => (1 | r.next_u32(), pos.min(r.next_u32())),
                            _ => (0, 0),
                        };
                        c.bus.write32(MON + 0x28, base);
                        c.bus.write32(MON + 0x2c, limit);
                        if case >= 6 {
                            // A live stream whose queue request succeeds.
                            c.bus.write8(STRM_COUNT, 4);
                            c.bus.write32(STRM_TABLE + 4 * handle, STREAM);
                            c.bus.write32(STREAM, FILE);
                            c.bus.write32(STREAM + 0xc, 0);
                            c.bus.write32(STREAM + 0x128, NODE);
                            c.bus.write32(STREAM + 0x12c, NODE);
                            c.bus.write32(STREAM + 0x130, 1);
                            c.bus.write32(NODE, 0);
                            c.bus.write32(FILE, HEADER);
                            c.bus.write32(HEADER, 0x4d52_5453); // "STRM"
                            c.bus.write32(HEADER + 0x38, 1);
                            c.bus.write32(HEADER + 0x64, 0);
                            c.bus.write32(HEADER + 0x68, REQ);
                            c.bus.write32(REQ + 0xc, 0);
                        }
                    }
                }
            },
        );
    }
    use super::*;
    use super::*;

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
                        c.bus
                            .write8(at + i as u32, if i == 4 { b'0' + k as u8 } else { *b });
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
                w(
                    c,
                    VT + 0x1c,
                    if seed % 4 == 1 {
                        0x0014_06c0
                    } else {
                        FAKE + 0x20
                    },
                );
                w(c, SYSTEM + SPEECH_PLAY_PLAYER_VTABLE, VT);
                w(
                    c,
                    SYSTEM + 0x414,
                    if seed % 3 == 1 {
                        0x8000_0000 | r.next_u32()
                    } else {
                        r.next_u32() & 0xffff
                    },
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
                    b"bank_eng",
                    b"bank_fre",
                    b"bnk_ger",
                    b"x_ita",
                    b"bank",
                    b"s_spa",
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
                w(
                    c,
                    STREAM + 0x124,
                    if seed / 3 % 3 == 0 {
                        0
                    } else {
                        1 + r.next_u32() % 4
                    },
                );
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

    /// `BankMonitor_StopHandle`: the real `BankMonitor_FindInstance` over
    /// 64 instances (no match, a match among decoys with the wrong id), a
    /// zero (also -0.0) or non-zero fade: the real `FadeVolume` / `Reset` on
    /// instance states (4.., negative) that need no SND call.
    #[test]
    fn bank_monitor_stop_handle_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA};
        const MONITOR: u32 = DATA + 0x100;
        const INSTANCES: u32 = DATA + 0x1000;
        check(
            ee::s08::f_002ad5f0__generated,
            bank_monitor_stop_handle,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_d5f0, 0);
                c.pc = 0x002a_d5f0;
                c.bus.write32(MONITOR + 0x8ec, INSTANCES);
                for a in (INSTANCES..INSTANCES + 64 * 0xc0).step_by(4) {
                    c.bus.write32(a, 0);
                }
                let kind = if seed % 5 == 0 {
                    (1 + r.next_u32() % 8).wrapping_neg()
                } else {
                    4 + r.next_u32() % 6
                };
                let id = r.next_u32() % 50;
                // Decoys: same kind, other id.
                for _ in 0..3 {
                    let k = r.next_u32() % 64;
                    c.bus.write32(INSTANCES + 0xc0 * k, kind);
                    c.bus.write32(INSTANCES + 0xc0 * k + 4, id + 1);
                }
                if seed % 4 != 0 {
                    let k = r.next_u32() % 64;
                    c.bus.write32(INSTANCES + 0xc0 * k, kind);
                    c.bus.write32(INSTANCES + 0xc0 * k + 4, id);
                    c.bus.write32(INSTANCES + 0xc0 * k + 0x54, r.next_u32() % 2);
                }
                c.g[A0][0] = sx(MONITOR);
                c.g[A1][0] = sx(id);
                c.g[A2][0] = sx(kind);
                c.f[12] = match seed % 6 {
                    0 => 0,
                    1 => 0x8000_0000,
                    _ => r.f32(5.0),
                };
            },
        );
    }
}
