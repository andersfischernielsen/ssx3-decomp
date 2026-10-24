//! `src/sound/bankmonitor.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
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
}
