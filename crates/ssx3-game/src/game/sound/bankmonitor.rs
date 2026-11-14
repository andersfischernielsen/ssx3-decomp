//! `src/sound/bankmonitor.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::Unwound;
use crate::events;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::beprepoststate::ea;
pub(crate) use crate::game::bx::bxstringctor::frame_less_return;
pub(crate) use crate::game::dirtysock::tagsunk::arg;
pub(crate) use crate::game::dirtysock::tagsunk::lw;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_3;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_2;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_3;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::lw_6 as lw_3;
pub(crate) use crate::game::fe::feasyncfile::lw_at_2 as lw_at;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_3;
pub(crate) use crate::game::fe::feasyncfile::shl32;
pub(crate) use crate::game::fe::feasyncfile::sw_6 as sw_2;
pub(crate) use crate::game::fe::festateload::at;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p_2;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festatetrophyroom::r;
pub(crate) use crate::game::fe::festatetrophyroom::Ops;
pub(crate) use crate::game::main::gamemode::low as low_2;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::sw_r;
pub(crate) use crate::game::main::gamestate::lw_at_3;
pub(crate) use crate::game::replay::replaycache::destroy_if_flag;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::forward;
pub(crate) use crate::game::visualfx::worldlightman::deleting_dtor;
pub(crate) use crate::game::wscript::wscriptman::sw;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
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
    (
        0x003d_0d70,
        path_set_track_control_value,
        "PATH_SetTrackControlValue",
    ),
    (
        0x003d_4b00,
        path_select_master_track,
        "PATH_SelectMasterTrack",
    ),
    (
        0x003d_5508,
        path_set_volume_percent,
        "PATH_SetVolumePercent",
    ),
    (0x002a_d888, bankmonitor_timer_set, "BankMonitor_TimerSet"),
    (
        0x002a_dc30,
        bank_monitor_find_free_timer,
        "BankMonitor_FindFreeTimer",
    ),
    (
        0x002b_03b8,
        speech_get_language_from_suffix,
        "Speech_GetLanguageFromSuffix",
    ),
    (
        0x002b_1348,
        speech_system_stream_wait_callback,
        "SpeechSystem_StreamWaitCallback",
    ),
    (
        0x002b_1520,
        speech_system_apply_event_context,
        "SpeechSystem_ApplyEventContext",
    ),
    (
        0x002a_d940,
        bank_monitor_timer_kill,
        "BankMonitor_TimerKill",
    ),
    (
        0x002b_0e28,
        speech_system_is_instance_ready,
        "SpeechSystem_IsInstanceReady",
    ),
    (
        0x002b_0f48,
        speech_system_on_spch_event,
        "SpeechSystem_OnSpchEvent",
    ),
    (
        0x002a_f8a8,
        speech_system_spch_event_callback,
        "SpeechSystem_SpchEventCallback",
    ),
    (
        0x002b_1248,
        speech_system_clear_prebuffer,
        "SpeechSystem_ClearPrebuffer",
    ),
    (
        0x002b_1268,
        speech_system_stream_start_callback,
        "SpeechSystem_StreamStartCallback",
    ),
    (0x003d_3900, path_track_queue_node, "PATH_TrackQueueNode"),
    (0x003d_3d20, path_track_advance, "PATH_TrackAdvance"),
    (
        0x003d_41a8,
        path_track_submit_queue,
        "PATH_TrackSubmitQueue",
    ),
];

/// Same-shard direct callees used above.
pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::ai::ai::callee::SPCH_FIND_BANK_TYPE;

    pub(crate) use crate::game::be::beintoption::callee::STREAM_ARRAY_STOP;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::BANK_MONITOR_INSTANCE_RESET;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::BANK_MONITOR_INSTANCE_RESUME;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::BE_LIBRARY_GET_INTERFACE_2 as BE_LIBRARY_GET_INTERFACE;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::EATIMER_REMOVE_CALLBACK;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::PATH_PLAT_REFRESH_STATUS;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::SND_REMOVE_TICK_CALLBACK;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::UI_OBJECT_LOAD;

    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_KILL_TIMERS_BY_CALLBACK;

    pub(crate) use crate::game::fe::feasyncfile::callee::MALLOC;

    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_RESUME;

    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_FIND_BANK_SLOT as PATH_PLAT_FIND_BANK_SLOT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_RESUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SET_RUNNING;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SUSPEND;

    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_LOAD as SND_BANK_LOAD_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_UNLOAD_BANK_2;

    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK_2 as SND_LOAD_QUEUE_CANCEL_BANK_3;

    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_PICK_RANDOM_LOOP;

    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_INSTANCE_CTOR;

    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_6 as SPRINTF_4;

    pub(crate) use crate::game::fe::festateload::callee::SONG_INSTANCE_PAUSE;
    pub(crate) use crate::game::fe::festateload::callee::SONG_INSTANCE_RESUME;

    pub(crate) use crate::game::fe::festatetrophyroom::callee::MEMSET;

    pub(crate) use crate::game::fe::festatetrophyroom::callee::SONG_INSTANCE_GET_DUCK_TO_LOOPS;

    pub(crate) use crate::game::intersect::worldsphtree::callee::SONG_INSTANCE_GET_BEAT_MS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SONG_INSTANCE_GET_POSITION_MS;

    pub(crate) use crate::game::main::gamestate::callee::MEM_FREE;

    pub(crate) use crate::game::main::gamestate::callee::SPCH_SHUTDOWN;
    pub(crate) use crate::game::main::gamestate::callee::SPEECH_INSTANCE_DTOR;

    pub(crate) use crate::game::render::bezierman::callee::SND_LOCK;
    pub(crate) use crate::game::render::bezierman::callee::SND_STATUS;
    pub(crate) use crate::game::render::bezierman::callee::SND_STREAM_INFO;
    pub(crate) use crate::game::render::bezierman::callee::SND_STREAM_POSITION;
    pub(crate) use crate::game::render::bezierman::callee::SND_SUB_STATE;
    pub(crate) use crate::game::render::bezierman::callee::SND_SUB_STATUS;
    pub(crate) use crate::game::render::bezierman::callee::SND_SUB_TIME;
    pub(crate) use crate::game::render::bezierman::callee::SND_UNLOCK;

    pub(crate) use crate::game::render::particle::callee::BANK_MONITOR_INSTANCE_PAUSE;

    pub(crate) use crate::game::render::particle::callee::PATH_POST_EVENT;
    pub(crate) use crate::game::render::particle::callee::PATH_SET_PAUSED;

    pub(crate) use crate::game::render::particle::callee::SNDSTRM_SET_PITCH;

    pub(crate) use crate::game::render::particle::callee::SONG_INSTANCE_GET_MS_TO_NEXT_BEAT;
    pub(crate) use crate::game::render::particle::callee::SONG_INSTANCE_GET_STREAM_HANDLE;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BANK_MONITOR_RESET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BANK_MONITOR_RESET_INSTANCE;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PATH_PLAT_FIND_BANK_SLOT;

    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::PATH_SET_TRACK_CONTROL_VALUE;

    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RAND_NEXT_MAIN;

    pub(crate) use crate::game::world::wscriptcache::callee::SND_BANK_MONITOR_MEM_SIZE;

    pub(crate) use crate::game::worldpainter::quadtree::callee::SNDSTRM_GET_REQUEST_STATUS;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SONG_FADE_OUT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SONG_GET_STREAM_STATUS;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SONG_POST_EVENT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SPEECH_INSTANCE_STOP_STREAM;

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
    pub(crate) use crate::game::lib::snd::callee::BANK_MONITOR_TIMER_ENABLE;
    pub(crate) use crate::game::lib::snd::callee::BANK_MONITOR_TIMER_KILL as BANK_MONITOR_TIMER_KILL_2;
    pub(crate) use crate::game::lib::snd::callee::BANK_MONITOR_TIMER_SET_PERIOD;
}

/// The monitor slots `PATH_PlatRefreshStatus` walks (`D_004D3E98`, 24
/// pointers) and the fields of one slot (`sMonSlot`) and its sub-streams.
pub(crate) mod monitor {

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

    /// `sMonSub`: size (+0x0), id, pos.
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
pub(crate) mod table_callee {}

/// `gp`-relative globals used here.
pub(crate) mod g {}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod table {

    pub(crate) use crate::game::fe::feasyncfile::table::EA_MEM_FREE;

    pub(crate) use crate::game::fe::feasyncfile::table::PATH_STOP_TRACKS;

    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_DELETE;
    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_VEC_DELETE;
}

/// Addresses the generator emitted as `call_indirect` (outside the shard's
/// own direct-call set, so no readable alias is needed here).
pub(crate) mod addr {

    pub(crate) use crate::game::bx::bxstringctor::addr::FMODF;

    pub(crate) use crate::game::bx::bxstringctor::addr::MEM_MAN_FREE;

    pub(crate) use crate::game::bx::bxstringctor::addr::SPEECH_BANK_FREE_CALLBACK;

    pub(crate) use crate::game::bx::bxstringctor::addr::UI_ENGINE_TEARDOWN;
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
    /// Slot fields: in use (+0x0), ticks left, the two keys, then the context
    /// (two words, a float, a word).
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
                    {
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
    c.sd(c.g[SP][0] as u32, c.g[A1][0]);
    'l_2ad448: loop {
        'b_2ad49c: {
            c.cycles += 12;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8ec));
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x1));
            c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
            c.ldr(3, c.g[SP][0] as u32);
            c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x17));
            c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
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
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S3][0] as u32));
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
        c.sb(c.g[T6][0] as u32, c.g[V0][0]);
        c.g[V1][0] = c.lbu((c.g[A0][0] as u32).wrapping_add(0x35a8));
        c.sb((c.g[T6][0] as u32).wrapping_add(0x20), c.g[V1][0]);
        'l_2b1b40: loop {
            c.cycles += 7;
            c.g[V0][0] = c.lbu((c.g[T7][0] as u32).wrapping_add(0x35a8));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x1));
            c.sb(c.g[A1][0] as u32, c.g[V0][0]);
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
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.mov_s(20, 12);
        c.mtc1(0, 21);
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8ec));
        'l_2ad698: loop {
            'b_2ad700: {
                c.cycles += 5;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                call_ptr(c, 0x002abb80, 0x002ad6a4)?;
                if c.g[V0][0] != 0u64 {
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc0));
                    break 'b_2ad700;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8ec));
                    c.g[A3][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[A3][0] as u32);
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
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
                c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x68));
                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x60));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                c.pc = t;
                return Ok(());
            }
        }
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

pub(crate) mod path_music {

    /// Initialised flag (byte), the master volume (byte), a map selection
    /// in progress (word: the selected map's flags), the selected map.
    pub const INITED: u32 = 0x0044_f42c;

    pub const MASTER_VOLUME: u32 = 0x0044_f42e;

    pub const SELECTED: u32 = 0x0044_f430;

    pub const CURRENT_MAP: u32 = 0x0044_f420;

    /// The four maps (0x928 bytes each): flags (`+0x30`, high nibbles
    /// matched against the mask), loaded (`+0x34`); a map's 24 tracks at
    /// `+0x58`.
    pub const MAPS: u32 = 0x0051_7610;

    pub const MAP_SIZE: u64 = 0x928;

    pub const MAP_FLAGS: u32 = 0x30;

    pub const MAP_LOADED: u32 = 0x34;

    pub const MAP_TRACKS: u32 = 0x58;

    pub const TRACKS: u32 = 24;
}

pub(crate) mod path_callee {
    use super::ee;
    use super::Callee;

    pub const SELECT_MAP: Callee = Callee {
        addr: 0x003d_5450,
        f: ee::s14::PATH_SelectMap,
    };

    pub const TRACK_SET_VOLUME: Callee = Callee {
        addr: 0x003d_62c8,
        f: ee::s14::PATH_TrackSetVolume,
    };
}

pub(crate) mod path_ctl {

    /// `u8`: the PATH player is initialised.
    pub const ENABLED: u32 = 0x0044_f42c;

    /// `PathMap*` of the selected map (its 24 track slots at +0x58).
    pub const CURRENT_MAP: u32 = 0x0044_f420;

    /// `int` id of the selected map (0 = none selected).
    pub const CURRENT_ID: u32 = 0x0044_f430;

    /// `&maps[0].loaded` (+0x34) and `&maps[0].id` (+0x30): the loop walks
    /// both with a stride of one map.
    pub const MAP0_LOADED: u32 = 0x0051_7644;

    pub const MAP0_ID: u32 = 0x0051_7640;

    pub const MAP_SIZE: u64 = 0x928;

    pub const SLOT0: u32 = 0x58;

    /// Track bytes: the control value and its previous value.
    pub const VALUE: u32 = 0x1;

    pub const PREV_VALUE: u32 = 0x2;
}

/// `void PATH_SetVolumePercent(u32 mask, s8 percent)` (0x3d5508): clamp
/// the volume to 0..100 (mask -1 also sets the master volume) and give it
/// to every track whose bit is set in `mask`, of every loaded map whose
/// flag nibbles match the mask. Does nothing before init or while a map is
/// selected.
pub fn path_set_volume_percent(c: &mut Cpu) {
    use path_callee::*;
    use path_music::*;
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = 0x0045_0000;
        frame.save(c, S1, 0x80);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
        frame.save(c, S2, 0x70);
        c.g[S1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
        c.g[V1][0] = Addr(INITED).lbu(c);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x90);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);
        let mask = c.g[S2][0];
        // Whether the exit passes the block at 0x3d5668 (+1 cycle).
        let mut tail = true;
        'body: {
            if c.g[V1][0] == 0 {
                break 'body;
            }
            c.cycles += 4;
            c.g[V1][0] = 0x0045_0000;
            c.g[V0][0] = Addr(SELECTED).lw(c);
            if c.g[V0][0] != 0 {
                tail = false;
                break 'body;
            }
            c.cycles += 4;
            c.g[A0][0] = u64::MAX;
            SELECT_MAP.call(c, 0x003d_5564)?;
            c.g[V1][0] = 100;
            if c.g[V0][0] == 0 {
                break 'body;
            }
            // Clamp to 0..100.
            c.cycles += 8;
            c.g[A0][0] = ((c.g[S1][0] as i64) < 101) as u64;
            if c.g[A0][0] == 0 {
                c.g[S1][0] = 100;
            }
            c.g[V0][0] = (-1 < c.g[S1][0] as i64) as u64;
            c.g[V1][0] = u64::MAX;
            if c.g[V0][0] == 0 {
                c.g[S1][0] = 0;
            }
            if mask == u64::MAX {
                c.cycles += 2;
                c.g[V0][0] = 0x0045_0000;
                let vol = c.g[S1][0];
                Addr(MASTER_VOLUME).sb(c, vol);
            }
            c.cycles += 9;
            c.g[V0][0] = 0x0051_0000;
            c.g[A0][0] = 0;
            c.g[S5][0] = sx(MAPS + MAP_LOADED);
            c.g[S4][0] = 0x0045_0000;
            c.g[FP][0] = MAP_SIZE;
            c.g[S7][0] = 0xffff_ffff_f000_0000;
            c.g[S6][0] = 0x0f00_0000;
            c.mult(0, A1, A0, FP);
            loop {
                // Map i = a0 (a1 = i * 0x928).
                let map = MAPS.wrapping_add(c.g[A1][0] as u32);
                c.cycles += 4;
                c.g[V0][0] = sx(map.wrapping_add(MAP_LOADED));
                c.g[V1][0] = Addr(map.wrapping_add(MAP_LOADED)).lw(c);
                c.g[S3][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                let mut skip_tail = true;
                'map: {
                    if c.g[V1][0] == 0 {
                        break 'map;
                    }
                    c.cycles += 8;
                    c.g[V0][0] = sx(MAPS + MAP_FLAGS);
                    c.g[V1][0] = sx(map.wrapping_add(MAP_FLAGS));
                    c.g[V1][0] = Addr(map.wrapping_add(MAP_FLAGS)).lw(c);
                    let flags = c.g[V1][0];
                    let high = flags & c.g[S7][0] & mask;
                    c.g[V0][0] = flags & c.g[S6][0];
                    if high == 0 {
                        break 'map;
                    }
                    c.cycles += 3;
                    c.g[V0][0] &= mask;
                    if c.g[V0][0] == 0 {
                        c.g[A0][0] = c.g[S3][0];
                        skip_tail = false;
                        break 'map;
                    }
                    c.cycles += 4;
                    SELECT_MAP.call(c, 0x003d_55fc)?;
                    let selected = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S3][0];
                    if !selected {
                        skip_tail = false;
                        break 'map;
                    }
                    // The selected map's tracks in the mask.
                    c.cycles += 3;
                    c.g[S0][0] = 0;
                    c.g[V0][0] = Addr(CURRENT_MAP).lw(c);
                    loop {
                        let j = c.g[S0][0] as u32;
                        c.cycles += 5;
                        c.g[V1][0] = sx(j << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(j << 2));
                        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(MAP_TRACKS)).lw(c);
                        let track = c.g[A0][0];
                        c.g[V0][0] = sx((mask as u32) >> (j & 31));
                        let mut step_tail = true;
                        if track != 0 {
                            c.cycles += 4;
                            c.g[V0][0] = (c.g[V0][0] ^ 1) & 1;
                            if c.g[V0][0] != 0 {
                                step_tail = false;
                            } else {
                                c.cycles += 2;
                                c.g[A1][0] = c.g[S1][0];
                                TRACK_SET_VOLUME.call(c, 0x003d_563c)?;
                            }
                        }
                        if step_tail {
                            c.cycles += 1;
                        }
                        c.g[S0][0] = sx(j.wrapping_add(1));
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S0][0] as i64) < TRACKS as i64) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[V0][0] = Addr(CURRENT_MAP).lw(c);
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003d_5610);
                        }
                    }
                }
                if skip_tail {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S3][0];
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A0][0] as i64) < 4) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.mult(0, A1, A0, FP);
                if event_due(c) {
                    deliver_event_here(c, 0x003d_55b8);
                }
            }
            // Done: forget the selection.
            c.cycles += 3;
            c.g[V0][0] = 0x0045_0000;
            Addr(CURRENT_MAP).sw(c, 0);
            Addr(SELECTED).sw(c, 0);
        }
        if tail {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x90);
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
// PATH_SelectMasterTrack (0x3d4b00)

/// The PATH module's state pointer (`0x44f420`): the master track selection
/// (+0 index, +4 length, +8 kind) and 24 track pointers from +0x58.
pub(crate) const PATH_STATE: u32 = 0x0044_f420;

pub(crate) const PATH_MASTER_INDEX: u32 = 0x0;

pub(crate) const PATH_MASTER_LENGTH: u32 = 0x4;

pub(crate) const PATH_MASTER_KIND: u32 = 0x8;

pub(crate) const PATH_TRACKS: u32 = 0x58;

pub(crate) const PATH_TRACK_COUNT: i64 = 24;

/// Track: disabled byte (+3), length (+0x24), header pointer (+0x30, its
/// first word holds the kind in bits 21-26), signed halfword (+0x36,
/// negative: not a candidate).
pub(crate) const TRACK_DISABLED: u32 = 0x3;

pub(crate) const TRACK_LENGTH: u32 = 0x24;

pub(crate) const TRACK_HEADER: u32 = 0x30;

pub(crate) const TRACK_FLAGS: u32 = 0x36;

/// `void PATH_SelectMasterTrack(void)` (0x3d4b00): picks the longest of the
/// 24 PATH tracks that is present, enabled (`+3 == 0`), has a header and a
/// non-negative `+0x36`: index, length (unsigned compare, strictly longer
/// wins) and kind (`header[0] >> 21 & 0x3f`) into the state's master
/// selection (reset to -1/0/0 first).
///
/// Registers as the original: a1 = state, a2 = track slot cursor, a3 =
/// index, t0 = 1, a0/v0/v1 scratch.
pub fn path_select_master_track(c: &mut Cpu) {
    c.cycles += 12;
    c.g[V0][0] = 0x0045_0000;
    c.g[A0][0] = u64::MAX;
    c.g[V1][0] = Addr(PATH_STATE).lw(c);
    c.g[A3][0] = 0;
    c.g[T0][0] = 1;
    c.g[A1][0] = c.g[V1][0];
    let state = Addr(r32(c, A1));
    state.off(PATH_MASTER_INDEX).sw(c, u64::MAX);
    state.off(PATH_MASTER_LENGTH).sw(c, 0);
    c.g[A2][0] = sx(state.0.wrapping_add(PATH_TRACKS));
    state.off(PATH_MASTER_KIND).sw(c, 0);
    c.g[A0][0] = Addr(r32(c, A2)).lw(c);
    loop {
        c.cycles += 2;
        let track = r32(c, A0);
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(4));
        // Through the shared `a3++` with its extra cycle, or straight to the
        // loop test (the original's two increments).
        let mut tail_cycle = true;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(track).off(TRACK_DISABLED).lbu(c);
            c.g[V1][0] = 0;
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[V0][0] = Addr(track).off(TRACK_FLAGS).lh(c);
                if (c.g[V0][0] as i64) >= 0 {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(track).off(TRACK_HEADER).lw(c);
                    c.g[V1][0] = if c.g[V0][0] == 0 { 0 } else { c.g[T0][0] };
                }
                c.cycles += 2;
                if c.g[V1][0] == 0 {
                    tail_cycle = false;
                } else {
                    c.cycles += 5;
                    c.g[V1][0] = Addr(track).off(TRACK_LENGTH).lw(c);
                    c.g[V0][0] = state.off(PATH_MASTER_LENGTH).lw(c);
                    c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                    if c.g[V0][0] == 0 {
                        tail_cycle = false;
                    } else {
                        // The longest so far.
                        c.cycles += 7;
                        state.off(PATH_MASTER_LENGTH).sw(c, c.g[V1][0]);
                        state.off(PATH_MASTER_INDEX).sw(c, c.g[A3][0]);
                        c.g[V1][0] = Addr(track).off(TRACK_HEADER).lw(c);
                        c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                        c.g[V0][0] = sx(r32(c, V0) >> 21) & 0x3f;
                        state.off(PATH_MASTER_KIND).sw(c, c.g[V0][0]);
                    }
                }
            }
        }
        if tail_cycle {
            c.cycles += 1;
        }
        c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A3][0] as i64) < PATH_TRACK_COUNT) as u64;
        if c.g[V0][0] == 0 {
            break;
        }
        c.g[A0][0] = Addr(r32(c, A2)).lw(c);
        if event_due(c) {
            deliver_event_here(c, 0x003d_4b30);
        }
    }
    c.cycles += 2;
    ret(c);
}

pub(crate) const PATH_SELECT_MAP: Callee = Callee {
    addr: 0x003d_5450,
    f: ee::s14::PATH_SelectMap,
};

/// `int PATH_SetTrackControlValue(uint mask, uint value)` (0x3d0d70): set
/// the control byte of every track of every loaded path map whose id
/// matches `mask` in both its top and second nibbles (the old value moves
/// to the previous-value byte). Fails with -18 unless the player is on, no
/// map is selected and the `PATH_SelectMap(-1)` query succeeds; values of
/// 0x80 and above change nothing. Clears the selection and returns 0.
pub fn path_set_track_control_value(c: &mut Cpu) {
    use path_ctl::*;
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = 0x45_0000;
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        c.g[S1][0] = c.g[A1][0];
        c.g[V1][0] = Addr(ENABLED).lbu(c);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x90);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);

        let mut ready = false;
        let mut busy = false;
        if c.g[V1][0] != 0 {
            c.cycles += 4;
            c.g[V1][0] = 0x45_0000;
            busy = Addr(CURRENT_ID).lw(c) != 0;
            c.g[V0][0] = sx(-18i32 as u32);
            if !busy {
                c.cycles += 4;
                c.g[A0][0] = u64::MAX;
                PATH_SELECT_MAP.call(c, 0x003d_0dc8)?;
                ready = c.g[V0][0] != 0;
                c.g[V0][0] = (c.g[S1][0] < 0x80) as u64;
            }
        }
        if !busy && !ready {
            c.cycles += 2;
            c.g[V0][0] = sx(-18i32 as u32);
        } else if ready {
            c.cycles += 2;
            c.g[S5][0] = 0x45_0000;
            if c.g[V0][0] != 0 {
                set_values(c)?;
            }
            c.cycles += 4;
            c.g[V0][0] = 0x45_0000;
            Addr(CURRENT_MAP).sw(c, 0);
            Addr(CURRENT_ID).sw(c, 0);
            c.g[V0][0] = 0;
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

/// The map loop of `PATH_SetTrackControlValue`: a0 = map index, a1 = its
/// byte offset (`mult`), s1 = value, s2 = mask.
pub(crate) fn set_values(c: &mut Cpu) -> crate::call::Flow {
    use path_ctl::*;
    c.cycles += 8;
    c.g[V0][0] = 0x51_0000;
    c.g[A0][0] = 0;
    c.g[S3][0] = sx(MAP0_LOADED);
    c.g[S7][0] = MAP_SIZE;
    c.g[FP][0] = sx(MAP0_ID);
    c.g[S6][0] = 0xffff_ffff_f000_0000;
    c.g[S4][0] = 0x0f00_0000;
    c.mult(0, A1, A0, S7);
    loop {
        c.cycles += 4;
        c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, S3)));
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        c.g[S0][0] = sx(r32(c, A0).wrapping_add(1));
        // `direct`: the second-nibble test failed; skip the 1-cycle tail.
        let mut direct = false;
        if c.g[V1][0] != 0 {
            c.cycles += 6;
            c.g[V1][0] = Addr(r32(c, A1).wrapping_add(r32(c, FP))).lw(c);
            let top = c.g[V1][0] & c.g[S6][0] & c.g[S2][0];
            c.g[V0][0] = c.g[V1][0] & c.g[S4][0];
            if top != 0 {
                c.cycles += 3;
                c.g[V0][0] &= c.g[S2][0];
                if c.g[V0][0] == 0 {
                    c.g[A0][0] = c.g[S0][0];
                    direct = true;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] &= 0xff;
                    PATH_SELECT_MAP.call(c, 0x003d_0e3c)?;
                    let selected = c.g[V0][0] != 0;
                    c.g[A0][0] = 0;
                    if selected {
                        set_map_tracks(c);
                    }
                }
            }
        }
        if !direct {
            c.cycles += 1;
            c.g[A0][0] = c.g[S0][0];
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A0][0] as i64) < 4) as u64;
        if c.g[V0][0] == 0 {
            return Ok(());
        }
        c.mult(0, A1, A0, S7);
        if event_due(c) {
            deliver_event_here(c, 0x003d_0e00);
        }
    }
}

/// Write the value (s1) into the 24 track slots of the selected map.
pub(crate) fn set_map_tracks(c: &mut Cpu) {
    use path_ctl::*;
    c.cycles += 3;
    c.g[A1][0] = 0x45_0000;
    c.g[V0][0] = Addr(CURRENT_MAP).lw(c);
    loop {
        c.cycles += 5;
        c.g[V1][0] = sx(r32(c, A0) << 2);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(SLOT0)).lw(c);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
        if c.g[V1][0] != 0 {
            c.cycles += 3;
            let track = Addr(r32(c, V1));
            let (value, prev) = (c.g[S1][0], track.off(VALUE).lbu(c));
            c.g[V0][0] = prev;
            track.off(VALUE).sb(c, value);
            track.off(PREV_VALUE).sb(c, prev);
        }
        c.cycles += 3;
        let more = (c.g[A0][0] as i64) < 24;
        c.g[V0][0] = Addr(CURRENT_MAP).lw(c);
        if !more {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003d_0e50);
        }
    }
}

// ---------------------------------------------------------------------------
// SpeechSystem_ApplyEventContext (0x2b1520)

/// `sBMon.ents` entry `j`, as the body addresses it: `self + j*0x20`, with the
/// fields at these absolute offsets (`ents[0]` is `self + 0x18`, i.e. the
/// `used` word of entry 0).
#[cfg(test)]
pub(crate) const SPEECH_EVENT_ENT_SIZE: u32 = 0x20;

/// `sBEnt_1520` fields, relative to `self + j*0x20`.
pub(crate) const SPEECH_EVENT_ENT_USED: u32 = 0x18;

pub(crate) const SPEECH_EVENT_ENT_ID: u32 = 0x20;

pub(crate) const SPEECH_EVENT_ENT_KEY: u32 = 0x24;

pub(crate) const SPEECH_EVENT_ENT_OBJ: u32 = 0x28;

pub(crate) const SPEECH_EVENT_ENT_W: u32 = 0x2c;

pub(crate) const SPEECH_EVENT_ENT_T: u32 = 0x30;

pub(crate) const SPEECH_EVENT_ENT_K: u32 = 0x34;

/// `sBMon` fields.
pub(crate) const SPEECH_EVENT_SLOTS: u32 = 0x04;

pub(crate) const SPEECH_EVENT_CUR_K: u32 = 0x0c;

pub(crate) const SPEECH_EVENT_CUR_ID: u32 = 0x10;

/// `sBMObj_1520`: the voice (a `cBMVoice_1520`) and the stored `z`.
pub(crate) const SPEECH_EVENT_OBJ_VOICE: u32 = 0x6c0;

pub(crate) const SPEECH_EVENT_OBJ_Z: u32 = 0x870;

/// Virtual slots of the voice: `v05()` at +0x28 and `v08()` at +0x40.
pub(crate) const SPEECH_EVENT_V05: u32 = 0x28;

pub(crate) const SPEECH_EVENT_V08: u32 = 0x40;

/// Where the found entry writes the voice's state (`slots[id]` fields).
pub(crate) const SPEECH_EVENT_SLOT_V05: u32 = 0x8cc;

pub(crate) const SPEECH_EVENT_SLOT_V08: u32 = 0x8f8;

pub(crate) const SPEECH_EVENT_SLOT_Z: u32 = 0x8fc;

pub(crate) const SPEECH_EVENT_SLOT_T: u32 = 0x8f4;

pub(crate) const SPEECH_EVENT_SLOT_W: u32 = 0x8d0;

/// `int SpeechSystem_ApplyEventContext(sBMon* self, int id, int key)`
/// (0x2b1520): find the used entry of the ten whose `(id, key)` matches; copy
/// its voice's `v05`/`v08`/`z`/`t`/`w` into `self->slots[id]`, remember its
/// `k` (or clear the current key when `k` is 0) and return 1. No match: 0.
/// Frame 0x70.
pub fn speech_system_apply_event_context(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S5, 0x10);
        c.g[S0][0] = c.g[A0][0]; // self
        frame.save(c, S3, 0x30);
        c.g[S5][0] = c.g[A1][0]; // id
        frame.save(c, S1, 0x50);
        c.g[S3][0] = sx((c.g[S5][0] as u32) << 2); // id * 4 (slots index)
        frame.save(c, S2, 0x40);
        c.g[A0][0] = 0; // j
        frame.save(c, S4, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = sx((c.g[A0][0] as u32) << 5); // j * 0x20
        'search: loop {
            'b_2b167c: {
                'b_2b166c: {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(SPEECH_EVENT_ENT_USED));
                    if c.g[V1][0] == 0 {
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                        break 'b_2b166c;
                    }
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(SPEECH_EVENT_ENT_ID));
                    if c.g[V1][0] != c.g[S5][0] {
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                        break 'b_2b166c;
                    }
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(SPEECH_EVENT_ENT_KEY));
                    if c.g[V1][0] != c.g[A2][0] {
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                        break 'b_2b166c;
                    }
                    // The entry matches: read the voice's v05/v08 and the z.
                    'b_2b15e0: {
                        c.cycles += 7;
                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(SPEECH_EVENT_ENT_OBJ));
                        c.g[S4][0] = 0;
                        c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                        c.g[A3][0] = 0;
                        c.g[A1][0] = c.lw(c.g[S2][0] as u32); // the entry's object
                        let no_object = c.g[A1][0] == 0;
                        c.g[T0][0] = sx(0xffff_ffff);
                        if no_object {
                            break 'b_2b15e0;
                        }
                        c.cycles += 17;
                        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(SPEECH_EVENT_OBJ_VOICE));
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(SPEECH_EVENT_OBJ_VOICE));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(SPEECH_EVENT_V05));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(SPEECH_EVENT_V05 + 4));
                        let v05 = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, v05, 0x002b_15b4)?;
                        c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                        c.g[S4][0] = c.g[V0][0]; // v = voice->v05()
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(SPEECH_EVENT_OBJ_VOICE));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(SPEECH_EVENT_OBJ_VOICE));
                        c.g[A1][0] = c.lh((c.g[V1][0] as u32).wrapping_add(SPEECH_EVENT_V08));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(SPEECH_EVENT_V08 + 4));
                        let v08 = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                        call_ptr(c, v08, 0x002b_15d4)?;
                        c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                        c.g[A3][0] = c.g[V0][0]; // u = voice->v08()
                        c.g[T0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(SPEECH_EVENT_OBJ_Z));
                        break 'b_2b15e0;
                    }
                    'b_2b1664: {
                        'b_2b165c: {
                            c.cycles += 28;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SPEECH_EVENT_SLOTS));
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                            c.g[A2][0] = c.g[A0][0];
                            c.g[A1][0] = c.g[A0][0];
                            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                            c.sw(
                                (c.g[V1][0] as u32).wrapping_add(SPEECH_EVENT_SLOT_V05),
                                c.g[S4][0],
                            );
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SPEECH_EVENT_SLOTS));
                            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                            c.sw(
                                (c.g[V1][0] as u32).wrapping_add(SPEECH_EVENT_SLOT_V08),
                                c.g[A3][0],
                            );
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SPEECH_EVENT_SLOTS));
                            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                            c.sw(
                                (c.g[V1][0] as u32).wrapping_add(SPEECH_EVENT_SLOT_Z),
                                c.g[T0][0],
                            );
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SPEECH_EVENT_SLOTS));
                            c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(SPEECH_EVENT_ENT_T));
                            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(SPEECH_EVENT_SLOT_T));
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(SPEECH_EVENT_SLOTS));
                            c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(SPEECH_EVENT_ENT_W));
                            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                            c.sw(
                                (c.g[V1][0] as u32).wrapping_add(SPEECH_EVENT_SLOT_W),
                                c.g[A0][0],
                            );
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(SPEECH_EVENT_ENT_K));
                            if c.g[V0][0] == 0 {
                                c.g[V0][0] = sx(0xffff_ffff);
                                break 'b_2b165c;
                            }
                            c.cycles += 3;
                            c.sw(
                                (c.g[S0][0] as u32).wrapping_add(SPEECH_EVENT_CUR_ID),
                                c.g[S5][0],
                            );
                            c.sw(
                                (c.g[S0][0] as u32).wrapping_add(SPEECH_EVENT_CUR_K),
                                c.g[V0][0],
                            );
                            break 'b_2b1664;
                        }
                        c.cycles += 2;
                        c.sw((c.g[S0][0] as u32).wrapping_add(SPEECH_EVENT_CUR_K), 0);
                        c.sw(
                            (c.g[S0][0] as u32).wrapping_add(SPEECH_EVENT_CUR_ID),
                            c.g[V0][0],
                        );
                    }
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    break 'b_2b167c;
                }
                // Next entry, or out of entries.
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A0][0] as i64) < 10) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S1][0] = sx((c.g[A0][0] as u32) << 5);
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x002b_1558);
                    }
                    continue 'search;
                }
                c.cycles += 1;
                c.g[V0][0] = 0;
                break 'b_2b167c;
            }
            c.cycles += 9;
            break 'search;
        }
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
// Speech_GetLanguageFromSuffix (0x2b03b8)
// ---------------------------------------------------------------------------

/// `int Speech_GetLanguageFromSuffix(const char* name)` (0x2b03b8): a speech
/// bank name ends in `_eng`, `_frn`, `_grm` or `_spn`; return 1..4 for those
/// languages and 0 otherwise. The check is on the last four characters, so a
/// name shorter than four still reads whatever precedes it.
pub fn speech_get_language_from_suffix(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0041_6810, 0x002b_03d0)?; // strlen
        c.g[A1][0] = c.g[V0][0]; // len
        c.g[V1][0] = 0x5f; // '_'
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, S0))); // name + len
        c.g[V0][0] = Addr(r32(c, A0).wrapping_sub(4)).lb(c);
        let is_underscore = c.g[V0][0] == c.g[V1][0];
        c.g[S1][0] = 0; // lang
        let mut plus_one = true;
        'suffix: {
            if !is_underscore {
                break 'suffix;
            }
            // "_eng"
            c.cycles += 4;
            c.g[V1][0] = Addr(r32(c, A0).wrapping_sub(3)).lb(c);
            c.g[V0][0] = 0x65;
            let not_e = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, S0)));
            if !not_e {
                c.cycles += 4;
                c.g[V1][0] = Addr(r32(c, A0).wrapping_sub(2)).lb(c);
                c.g[V0][0] = 0x6e;
                let not_n = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, S0)));
                if !not_n {
                    c.cycles += 4;
                    c.g[V1][0] = Addr(r32(c, A0).wrapping_sub(1)).lb(c);
                    c.g[V0][0] = 0x67;
                    let not_g = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, S0)));
                    if !not_g {
                        c.cycles += 2;
                        c.g[S1][0] = 1;
                        break 'suffix;
                    }
                }
            }
            // "_frn"
            c.cycles += 4;
            c.g[A0][0] = 0x66;
            c.g[V1][0] = Addr(r32(c, V0).wrapping_sub(3)).lb(c);
            let not_f = c.g[V1][0] != c.g[A0][0];
            c.g[A1][0] = c.g[V0][0];
            if not_f {
                c.cycles += 1;
                c.g[V1][0] = Addr(r32(c, A1).wrapping_sub(3)).lb(c);
            } else {
                c.cycles += 4;
                c.g[V1][0] = Addr(r32(c, A1).wrapping_sub(2)).lb(c);
                c.g[V0][0] = 0x72;
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[V1][0] = Addr(r32(c, A1).wrapping_sub(3)).lb(c);
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = Addr(r32(c, A1).wrapping_sub(1)).lb(c);
                    c.g[V0][0] = 0x6e;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V1][0] = Addr(r32(c, A1).wrapping_sub(3)).lb(c);
                    } else {
                        c.cycles += 2;
                        c.g[S1][0] = 2;
                        break 'suffix;
                    }
                }
            }
            // "_grm"
            c.cycles += 3;
            c.g[V0][0] = 0x67;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[V1][0] = Addr(r32(c, A1).wrapping_sub(3)).lb(c);
            } else {
                c.cycles += 4;
                c.g[V1][0] = Addr(r32(c, A1).wrapping_sub(2)).lb(c);
                c.g[V0][0] = 0x72;
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[V1][0] = Addr(r32(c, A1).wrapping_sub(3)).lb(c);
                } else {
                    c.cycles += 4;
                    c.g[V1][0] = Addr(r32(c, A1).wrapping_sub(1)).lb(c);
                    c.g[V0][0] = 0x6d;
                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[V1][0] = Addr(r32(c, A1).wrapping_sub(3)).lb(c);
                    } else {
                        c.cycles += 2;
                        c.g[S1][0] = 3;
                        break 'suffix;
                    }
                }
            }
            // "_spn"
            c.cycles += 3;
            c.g[V0][0] = 0x73;
            let not_s = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = c.g[S1][0];
            if not_s {
                plus_one = false;
                break 'suffix;
            }
            c.cycles += 4;
            c.g[V1][0] = Addr(r32(c, A1).wrapping_sub(2)).lb(c);
            c.g[V0][0] = 0x70;
            let not_p = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = c.g[S1][0];
            if not_p {
                plus_one = false;
                break 'suffix;
            }
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, A1).wrapping_sub(1)).lb(c);
            c.g[S1][0] = 4;
            c.g[V0][0] = c.g[V0][0] ^ 0x6e;
            if c.g[V0][0] != 0 {
                c.g[S1][0] = 0;
            }
        }
        if plus_one {
            c.cycles += 1;
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        c.g[RA][0] = c.ld(frame.at(c, 0));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BankMonitor_TimerSet (0x2ad888)
// ---------------------------------------------------------------------------

/// `void BankMonitor_TimerSet(BankMonitorTimer* t, int period, sBmKey key,
/// int b, int c, int d, int e, int f)` (0x2ad888): re-arm a bank-monitor
/// timer — kill it, set its period, store the 8-byte key (an unaligned
/// copy through the stack) and five payload words, then enable it.
pub fn bankmonitor_timer_set(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 42;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        c.g[S0][0] = c.g[A0][0]; // the timer
        frame.save(c, S1, 0x70);
        frame.save(c, S2, 0x60);
        c.g[S1][0] = c.g[A1][0]; // period
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A3][0]; // b
        frame.save(c, S4, 0x40);
        c.g[S3][0] = c.g[T0][0]; // c
        frame.save(c, S5, 0x30);
        c.g[S4][0] = c.g[T1][0]; // d
        frame.save(c, S6, 0x20);
        c.g[S5][0] = c.g[T2][0]; // e
        c.g[S6][0] = c.g[T3][0]; // f
        c.sd(frame.at(c, 0x10), c.g[RA][0]);
        let key = c.g[A2][0];
        c.sd(frame.at(c, 0), key);
        callee::BANK_MONITOR_TIMER_KILL_2.call(c, 0x002a_d8d0)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        callee::BANK_MONITOR_TIMER_SET_PERIOD.call(c, 0x002a_d8dc)?;
        // The key is copied 8 bytes at a time, unaligned.
        c.ldl(2, frame.at(c, 7));
        c.ldr(2, frame.at(c, 0));
        let key_src = Addr(r32(c, S0));
        c.sdl(2, key_src.off(0x13).0);
        c.sdr(2, key_src.off(0xc).0);
        let timer = Addr(r32(c, S0));
        let b = c.g[S2][0];
        timer.off(0x18).sw(c, b);
        c.g[A0][0] = c.g[S0][0];
        let cw = c.g[S3][0];
        timer.off(0x1c).sw(c, cw);
        let dw = c.g[S4][0];
        timer.off(0x20).sw(c, dw);
        let ew = c.g[S5][0];
        timer.off(0x24).sw(c, ew);
        let fw = c.g[S6][0];
        timer.off(0x28).sw(c, fw);
        callee::BANK_MONITOR_TIMER_ENABLE.call(c, 0x002a_d908)?;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        c.g[RA][0] = c.ld(frame.at(c, 0x10));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(sp(c).wrapping_add(0x90));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SpeechSystem_StreamWaitCallback (0x2b1348)

/// `SND` request message: the entry pointer at +0x18, the request id at
/// +0x1c and the timer id at +0x20.
pub(crate) const STREAM_MSG_ENTRY: u32 = 0x18;

pub(crate) const STREAM_MSG_REQUEST: u32 = 0x1c;

pub(crate) const STREAM_MSG_TIMER: u32 = 0x20;

/// The request entry: the stream slot at +0, the stream index at +4.
#[cfg(test)]
pub(crate) const STREAM_ENTRY_SLOT: u32 = 0;

pub(crate) const STREAM_ENTRY_INDEX: u32 = 0x4;

/// `D_004A37B4` (the SND manager's `**`) at `gp + 0x6c4`.
pub(crate) const STREAM_SND_GP: u32 = 0x6c4;

/// The bank monitor's timer object at +0x1d8 of the manager.
pub(crate) const STREAM_MONITOR: u32 = 0x1d8;

/// The timer descriptor `D_004A37D8` (8 bytes, loaded unaligned).
pub(crate) const STREAM_TIMER_DESC: u32 = 0x004a_37d8;

/// Callees.
pub(crate) const STREAM_ARRAY_GET_STATE: u32 = 0x002a_ae08;

pub(crate) const STREAM_ARRAY_IS_PLAYING: u32 = 0x002a_b028;

pub(crate) const STREAM_ARRAY_TIME_LEFT: u32 = 0x002a_ae78;

pub(crate) const STREAM_ARRAY_CURRENT: u32 = 0x002a_b150;

pub(crate) const STREAM_MONITOR_ADD_TIMER: u32 = 0x002a_dca0;

pub(crate) const STREAM_INSTANCE_SLOT: u32 = 0x002b_1220;

pub(crate) const STREAM_START_CALLBACK: u32 = 0x002b_1268;

/// `int SpeechSystem_StreamWaitCallback(SpeechSystem* self, SndMsg* msg)`
/// (0x2b1348): when the stream of the message's entry is beyond its first
/// state: if it is playing, add a bank-monitor timer for its remaining time
/// and return 1; otherwise, when the current request is the message's, run
/// the start callback. In every other case run the start callback (the first
/// state) or return 0. Frame 0x40 (s0 = self, s1 = entry, s2 = msg).
pub fn speech_system_stream_wait_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        c.g[S1][0] = Addr(r32(c, S2).wrapping_add(STREAM_MSG_ENTRY)).lw(c);
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        c.g[A1][0] = Addr(r32(c, S1).wrapping_add(STREAM_ENTRY_INDEX)).lw(c);
        call_ptr(c, STREAM_ARRAY_GET_STATE, 0x002b_1374)?;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        let first_state = (c.g[V0][0] as i64) <= 0;
        c.g[A0][0] = c.g[S0][0];
        if !first_state {
            // Past the first state: playing (timer) or pending (compare)?
            c.cycles += 5;
            c.g[A0][0] = Addr(r32(c, S0)).lw(c);
            c.g[A1][0] = Addr(r32(c, S1).wrapping_add(STREAM_ENTRY_INDEX)).lw(c);
            call_ptr(c, STREAM_ARRAY_IS_PLAYING, 0x002b_138c)?;
            if c.g[V0][0] == 0 {
                c.g[A1][0] = Addr(r32(c, S1)).lw(c);
            } else {
                c.cycles += 20;
                c.g[A0][0] = Addr(r32(c, S0)).lw(c);
                c.g[A1][0] = Addr(r32(c, S1).wrapping_add(STREAM_ENTRY_INDEX)).lw(c);
                call_ptr(c, STREAM_ARRAY_TIME_LEFT, 0x002b_13a0)?;
                // `BankMonitor_AddTimer(**(gp+0x6c4) + 0x1d8, (int)time,
                // D_004A37D8[0].v, entry, msg->request, msg->timer, 0, 0)`.
                c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(STREAM_SND_GP)).lw(c);
                c.cvt_w_s(1, 0);
                c.mfc1(5, 1);
                c.g[V0][0] = 0x004a_0000;
                c.g[T1][0] = Addr(r32(c, S2).wrapping_add(STREAM_MSG_TIMER)).lw(c);
                c.g[T0][0] = Addr(r32(c, V1)).lw(c);
                c.g[A3][0] = c.g[S1][0];
                c.ldl(6, STREAM_TIMER_DESC + 7);
                c.ldr(6, STREAM_TIMER_DESC);
                c.g[T2][0] = 0;
                c.g[A0][0] = Addr(r32(c, T0)).lw(c);
                c.g[T3][0] = 0;
                c.g[T0][0] = Addr(r32(c, S2).wrapping_add(STREAM_MSG_REQUEST)).lw(c);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(STREAM_MONITOR));
                call_ptr(c, STREAM_MONITOR_ADD_TIMER, 0x002b_13dc)?;
                c.g[V0][0] = 1;
                frame.restore(c, S0, 0x30);
                frame.restore(c, S1, 0x20);
                frame.restore(c, S2, 0x10);
                frame.restore_ra(c, 0);
                c.cycles += 6;
                frame.close(c);
                return Ok(());
            }
            // The request is still pending: is it the message's?
            c.cycles += 8;
            c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_add(STREAM_SND_GP)).lw(c);
            call_ptr(c, STREAM_INSTANCE_SLOT, 0x002b_13ec)?;
            c.g[A0][0] = Addr(r32(c, S0)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, STREAM_ARRAY_CURRENT, 0x002b_13f8)?;
            c.g[V1][0] = Addr(r32(c, S2).wrapping_add(STREAM_MSG_REQUEST)).lw(c);
            let not_ours = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = 0;
            if !not_ours {
                c.cycles += 1;
                c.g[A0][0] = c.g[S0][0];
                speech_stream_start(c, frame)?;
                return Ok(());
            }
        } else {
            speech_stream_start(c, frame)?;
            return Ok(());
        }
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        c.cycles += 6;
        frame.close(c);
        Ok(())
    })
}

/// The shared tail of `SpeechSystem_StreamWaitCallback`: run the start
/// callback and return its result.
pub(crate) fn speech_stream_start(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 2;
    c.g[A1][0] = c.g[S2][0];
    call_ptr(c, STREAM_START_CALLBACK, 0x002b_1410)?;
    c.cycles += 6;
    frame.restore(c, S0, 0x30);
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `BankMonitor_TimerIsFree` (0x2ad930).
pub(crate) const OC327_BANK_TIMER_IS_FREE: Callee = Callee {
    addr: 0x002a_d930,
    f: ee::s08::BankMonitor_TimerIsFree,
};

// ---------------------------------------------------------------------------
// Sound.

/// `s32 BankMonitor_FindFreeTimer(BankMonitor* self, s32 n)` (0x2adc30): the
/// index of the `n`-th free timer among the 48 at `self+0x98` (0x2c bytes
/// apart), or -1.
pub fn bank_monitor_find_free_timer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x98));
        frame.save(c, S2, 0x10);
        c.g[S1][0] = 0;
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        let mut done = false;
        loop {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            OC327_BANK_TIMER_IS_FREE.call(c, 0x002a_dc58)?;
            if c.g[V0][0] == 0 {
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 3;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                if c.g[S2][0] != 0 {
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S1][0];
                    done = true;
                }
            }
            if done {
                break;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 48) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2c));
            if !more {
                c.cycles += 1;
                c.g[V0][0] = u64::MAX;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002a_dc50);
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// BankMonitor_TimerKill (0x2ad940)

pub(crate) const BANKKILL_TIMER_DISABLE: Callee = Callee {
    addr: 0x002a_d990,
    f: ee::s08::f_002ad990,
};

pub(crate) const BANKKILL_TIMER_SET_PERIOD: Callee = Callee {
    addr: 0x002a_d970,
    f: ee::s08::f_002ad970,
};

/// `void BankMonitor_TimerKill(BankMonitor* m)` (0x2ad940): disarm the timer
/// (`m+0` = 0) and set its period to -1 (`m+4` and `m+8`).
pub fn bank_monitor_timer_kill(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        BANKKILL_TIMER_DISABLE.call(c, 0x002a_d954)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = u64::MAX;
        BANKKILL_TIMER_SET_PERIOD.call(c, 0x002a_d960)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SpeechSystem_OnSpchEvent (0x2b0f48)

pub(crate) const SPCH_SPEECH_PLAY_EVENT: Callee = Callee {
    addr: 0x002b_04d8,
    f: ee::s08::f_002b04d8,
};

/// `int SpeechSystem_OnSpchEvent(SpeechSystem* self, SpeechEvent* ev)`
/// (0x2b0f48): hand the event to the first of the two speech instances
/// (`self+4`, instance pointers) that plays it (a non-negative result),
/// calling the instance's callback (`self+0xc`) with the result when the
/// instance index `self+0x10` matches. -1 when neither plays it.
pub fn speech_system_on_spch_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(r32(c, S2)).off(0x4).lw(c);
        'l_2b0f70: loop {
            'b_2b0fb8: {
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                if c.g[A0][0] == 0 {
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    break 'b_2b0fb8;
                }
                c.cycles += 5;
                c.g[A1][0] = c.g[S3][0];
                SPCH_SPEECH_PLAY_EVENT.call(c, 0x002b_0f8c)?;
                c.g[S0][0] = c.g[V0][0];
                if (c.g[S0][0] as i64) < 0 {
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    break 'b_2b0fb8;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, S2)).off(0x10).lw(c);
                if c.g[V0][0] != c.g[S1][0] {
                    c.g[V0][0] = c.g[S0][0];
                    break 'l_2b0f70;
                }
                // The active instance's callback.
                c.cycles += 5;
                c.g[V0][0] = Addr(r32(c, S2)).off(0xc).lw(c);
                let callback = c.g[V0][0] as u32;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, callback, 0x002b_0fb0)?;
                c.g[V0][0] = c.g[S0][0];
                break 'l_2b0f70;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 2i64) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 1;
                c.g[V0][0] = u64::MAX;
                break 'l_2b0f70;
            }
            c.g[V1][0] = Addr(r32(c, S2)).off(0x4).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x002b_0f70);
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
// SpeechSystem_IsInstanceReady (0x2b0e28)

/// The speech system's instance array (at `speech + 4`).
pub(crate) const SIR_INSTANCES: u32 = 0x4;

/// `bool SpeechSystem_IsInstanceReady(SpeechSystem* self, int i)`
/// (0x2b0e28): whether instance `i` of the speech system's array exists and
/// its stream is ready (`SpeechInstance_IsStreamReady`), false when the slot
/// is empty.
pub fn speech_system_is_instance_ready(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, A0).wrapping_add(SIR_INSTANCES)).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = Addr(c.g[A1][0] as u32).lw(c);
        let empty = c.g[A0][0] == 0;
        c.g[V0][0] = 0;
        if !empty {
            c.cycles += 2;
            Callee {
                addr: 0x002b_0290,
                f: ee::s08::SpeechInstance_IsStreamReady,
            }
            .call(c, 0x002b_0e50)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SpeechSystem_SpchEventCallback (0x2af8a8).

/// `int SpeechSystem_OnSpchEvent(SpeechSystem*, SpchEvent*)` (0x2b0f48).
pub(crate) const SPCH_EVENT_CB_ON_SPCH_EVENT: Callee = Callee {
    addr: 0x002b_0f48,
    f: ee::s08::SpeechSystem_OnSpchEvent,
};

/// gp-relative: the flag that makes the callback refuse events.
pub(crate) const SPCH_EVENT_CB_BLOCKED: u32 = 0x6cc;

/// gp-relative: the speech system the events go to.
pub(crate) const SPCH_EVENT_CB_SYSTEM: u32 = 0x6c4;

/// `int SpeechSystem_SpchEventCallback(SpchEvent* ev)` (0x2af8a8): the
/// speech library's event callback. Refused (-1) while the flag at
/// `gp+0x6cc` is set; otherwise the event goes to
/// `SpeechSystem_OnSpchEvent(system at gp+0x6c4, ev)` and its result is
/// returned.
pub fn speech_system_spch_event_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        let gp = r32(c, GP);
        c.g[V0][0] = Addr(gp.wrapping_add(SPCH_EVENT_CB_BLOCKED)).lw(c);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A0][0];
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 4;
            c.g[A0][0] = Addr(gp.wrapping_add(SPCH_EVENT_CB_SYSTEM)).lw(c);
            SPCH_EVENT_CB_ON_SPCH_EVENT.call(c, 0x002a_f8c4)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// SpeechSystem_ClearPrebufferCallback (0x2ab1a0) / SpeechSystem_ClearPrebuffer (0x2b1248)

/// The speech stream object's stream handle.
pub(crate) const SPEECH_STREAM_HANDLE: u32 = 0x1c;

pub(crate) const SPEECH_CLEAR_PREBUFFER_CALLBACK: Callee = Callee {
    addr: 0x002a_b1a0,
    f: ee::s08::SpeechSystem_ClearPrebufferCallback,
};

/// `void SpeechSystem_ClearPrebuffer(SpeechSystem* self)` (0x2b1248): run the
/// clear-prebuffer callback with `a0 = *self` (`a1` passes through).
pub fn speech_system_clear_prebuffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(r32(c, A0));
        SPEECH_CLEAR_PREBUFFER_CALLBACK.call(c, 0x002b_1258)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PATH_TrackSubmitQueue (0x3d41a8)
// ---------------------------------------------------------------------------

/// `PathSystem* g_pathSystem` (`*0x44f420`): `+0` the index of the master
/// (beat) track or -1, `+0x58` the track table, `+0x34` the node array,
/// `+0x38` the node index table (`short`s), `+0x50` the sample table
/// (8-byte entries: `{ s5 value, length }`).
pub(crate) const PTS_PATH_SYS: u32 = 0x0044_f420;

/// `int g_pathTime` (`0x44f438`), the music clock.
pub(crate) const PTS_TIME: u32 = 0x0044_f438;

/// `int g_pathLastSyncQueue` (`0x44f79c`): when a synced track last queued.
#[cfg(test)]
pub(crate) const PTS_LAST_SYNC: u32 = 0x0044_f79c;

/// `double` volume scale at `0x495b98`.
pub(crate) const PTS_VOLUME_SCALE: u32 = 0x0049_5b98;

/// Track fields.
pub(crate) const PTS_VOLUME: u32 = 0x04;

pub(crate) const PTS_FLAGS: u32 = 0x20;

pub(crate) const PTS_LOOPING: u32 = 0x2f;

pub(crate) const PTS_NEXT_NODE: u32 = 0x30;

pub(crate) const PTS_GAIN: u32 = 0x34;

pub(crate) const PTS_NODE: u32 = 0x36;

pub(crate) const PTS_LENGTH: u32 = 0x40;

pub(crate) const PTS_BEAT_LENGTH: u32 = 0x44;

pub(crate) const PTS_START: u32 = 0x48;

pub(crate) const PTS_IDLE_SINCE: u32 = 0x4c;

pub(crate) const PTS_PLAT_TRACK: u32 = 0x54;

pub(crate) const PTS_HANDLE: u32 = 0x58;

pub(crate) const PTS_QUEUE_STATE: u32 = 0x5c;

pub(crate) const PTS_HISTORY: u32 = 0xfc;

pub(crate) const PTS_RESYNC: u32 = 0x10c;

/// `PATH_PlatGetQueuedEntryValue(track, i)` (through the table).
pub(crate) const PTS_GET_QUEUED: u32 = 0x002a_eaa8;

/// `PATH_PlatGetTrackVolume(track)`.
pub(crate) const PTS_GET_VOLUME: u32 = 0x002a_f428;

/// `PATH_PlatQueue(track, ...)`.
pub(crate) const PTS_PLAT_QUEUE: u32 = 0x002a_e7a8;

pub(crate) const PTS_FLOATSIDF: u32 = 0x0041_2550;

pub(crate) const PTS_MULDF3: u32 = 0x0041_1fd8;

pub(crate) const PTS_FIXDFSI: u32 = 0x0041_2608;

pub(crate) const PTS_CALC_REPEAT: Callee = Callee {
    addr: 0x003d_3658,
    f: ee::s14::PATH_CalcNodeRepeatCount,
};

pub(crate) const PTS_GET_BEAT_POS: Callee = Callee {
    addr: 0x003d_32b8,
    f: ee::s14::PATH_TrackGetBeatPos,
};

#[inline(always)]
pub(crate) fn pts_srl(v: u64, n: u32) -> u64 {
    sx((v as u32) >> n)
}

#[inline(always)]
pub(crate) fn pts_sra(v: u64, n: u32) -> u64 {
    sx(((v as u32) as i32 >> n) as u32)
}

#[inline(always)]
pub(crate) fn pts_sll(v: u64, n: u32) -> u64 {
    sx((v as u32) << n)
}

#[inline(always)]
pub(crate) fn pts_addu(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}

#[inline(always)]
pub(crate) fn pts_at(c: &Cpu, r: usize, off: u32) -> u32 {
    r32(c, r).wrapping_add(off)
}

/// The argument setup shared by the three `PATH_PlatQueue` calls (`t2`
/// already holds the track flags): `(plat track, s7 mode, &queue state,
/// s5 sample value, length, (s8)volume, repeat count, 0)`.
pub(crate) fn pts_queue_args(c: &mut Cpu, load_length: bool) {
    c.g[T1][0] = pts_sll(c.g[S2][0], 24);
    c.g[A1][0] = c.g[S7][0];
    c.g[T2][0] = pts_sll(c.g[T2][0], 23);
    c.g[A0][0] = c.lw(pts_at(c, S1, PTS_PLAT_TRACK));
    if load_length {
        c.g[T0][0] = c.lw(pts_at(c, S1, PTS_LENGTH));
    }
    c.g[A3][0] = c.g[S5][0];
    c.g[T1][0] = pts_sra(c.g[T1][0], 24);
    c.g[T2][0] = pts_sra(c.g[T2][0], 27);
    c.g[A2][0] = pts_addu(c.g[S1][0], PTS_QUEUE_STATE as u64);
    c.g[T3][0] = 0;
}

/// How the synced part of [`path_track_submit_queue`] ended.
pub(crate) enum PtsSynced {
    /// The next node does not sync: queue at once (`t2` = flags).
    Unsynced,
    /// Return `v0` (-9999: wait) now.
    Return,
    /// Too early to queue (idle time cleared).
    Wait,
    /// Queued on the voice.
    Queued,
}

/// `PATH_TrackGetBeatPos` failed: no beat (`pos.beat = 0`, `pos.next = -1`).
pub(crate) fn pts_no_beat(c: &mut Cpu) {
    c.g[V0][0] = 0xffff_ffff_ffff_0000;
    c.sw(sp(c).wrapping_add(0x24), 0);
    c.g[V0][0] |= 0xffff;
    c.sw(pts_at(c, S0, 0x10), c.g[V0][0]);
}

/// A looping track that follows the master track `s4` (`a0` = 0x100): if
/// the next node syncs (mode 0x100/0x200/0x300 in its `+8`), split the
/// lengths by the node's beats and bars, line the start (`+0x48`) up with
/// the master's beat position, and queue once half a length has passed
/// since the last synced queue.
pub(crate) fn pts_synced(c: &mut Cpu) -> Result<PtsSynced, Unwound> {
    c.cycles += 5;
    c.g[V0][0] = c.lw(pts_at(c, S1, PTS_NEXT_NODE));
    c.g[V1][0] = c.lw(pts_at(c, V0, 0x8));
    c.g[V1][0] &= 0x300;
    let mut synced = c.g[V1][0] == c.g[A0][0];
    c.g[V0][0] = 0x200;
    if !synced {
        c.cycles += 2;
        synced = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x300;
        if !synced {
            c.cycles += 2;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[T2][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
                return Ok(PtsSynced::Unsynced);
            }
        }
    }

    // Beat length /= beats (node +4 bits 20..23); length /= beats * bars.
    c.cycles += 5;
    c.g[V1][0] = c.lw(pts_at(c, S3, 0x4));
    c.g[V0][0] = 0xf00000;
    c.g[V0][0] &= c.g[V1][0];
    let no_beats = c.g[V0][0] == 0;
    c.g[V0][0] = pts_srl(c.g[V1][0], 20);
    if !no_beats {
        c.cycles += 4;
        c.g[V1][0] = c.lw(pts_at(c, S1, PTS_BEAT_LENGTH));
        c.g[V0][0] &= 0xf;
        c.cycles += 4;
        c.divu(0, V1, V0);
        c.g[V1][0] = c.lo[0];
        c.sw(pts_at(c, S1, PTS_BEAT_LENGTH), c.g[V1][0]);
        c.g[V1][0] = c.lw(pts_at(c, S3, 0x4));
    }
    c.cycles += 4;
    c.g[V0][0] = 0xf000000;
    c.g[V0][0] &= c.g[V1][0];
    let no_bars = c.g[V0][0] == 0;
    c.g[A0][0] = pts_srl(c.g[V1][0], 20);
    if !no_bars {
        c.cycles += 8;
        c.g[V0][0] = pts_srl(c.g[V1][0], 24) & 0xf;
        c.g[A0][0] &= 0xf;
        c.mult(0, V0, V0, A0);
        c.g[V1][0] = c.lw(pts_at(c, S1, PTS_LENGTH));
        c.divu(0, V1, V0);
        c.cycles += 2;
        c.g[V1][0] = c.lo[0];
        c.sw(pts_at(c, S1, PTS_LENGTH), c.g[V1][0]);
    }

    'queue_check: {
        'set_start: {
            'repeated: {
                c.cycles += 2;
                if c.g[FP][0] != 0 {
                    c.g[V0][0] = c.lw(pts_at(c, S1, PTS_NEXT_NODE));
                    break 'repeated;
                }
                'fresh: {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw(pts_at(c, S1, PTS_RESYNC));
                    c.g[V1][0] = 0x450000;
                    if c.g[A0][0] == 0 {
                        break 'fresh;
                    }
                    // A pending resync: wait until it is due.
                    c.cycles += 5;
                    c.g[V0][0] = c.lw(PTS_TIME);
                    c.g[V0][0] = pts_addu(c.g[V0][0], 5);
                    c.g[V0][0] = (c.g[A0][0] < c.g[V0][0]) as u64;
                    c.g[FP][0] = c.g[V1][0];
                    if c.g[V0][0] == 0 {
                        c.cycles += 2;
                        c.g[V0][0] = 0xffff_ffff_ffff_d8f1;
                        return Ok(PtsSynced::Return);
                    }
                    c.cycles += 7;
                    c.g[S0][0] = pts_addu(c.g[SP][0], 0x20);
                    c.sw(pts_at(c, S1, PTS_RESYNC), 0);
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A1][0] = c.g[S0][0];
                    PTS_GET_BEAT_POS.call(c, 0x003d_4510)?;
                    let found = (c.g[V0][0] as i64) >= 0;
                    c.g[V0][0] = c.lw(sp(c).wrapping_add(0x24));
                    if !found {
                        c.cycles += 5;
                        pts_no_beat(c);
                        c.g[V0][0] = c.lw(sp(c).wrapping_add(0x24));
                    }
                    // Next beat, or the one after if past half a beat.
                    c.cycles += 11;
                    c.g[A1][0] = c.lw(sp(c).wrapping_add(0x30));
                    c.g[V1][0] = pts_srl(c.g[V0][0], 31);
                    c.g[A0][0] = c.lw(r32(c, FP).wrapping_sub(0xbc8));
                    c.g[V1][0] = pts_addu(c.g[V0][0], c.g[V1][0]);
                    c.g[V1][0] = pts_sra(c.g[V1][0], 1);
                    c.g[V0][0] = pts_addu(c.g[A1][0], c.g[V0][0]);
                    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    if c.g[V1][0] == 0 {
                        c.g[A1][0] = c.g[V0][0];
                    }
                    c.g[A0][0] = pts_addu(c.g[A1][0], c.g[A0][0]);
                    c.sw(pts_at(c, S1, PTS_START), c.g[A0][0]);
                    break 'queue_check;
                }
                c.cycles += 6;
                c.g[S0][0] = pts_addu(c.g[SP][0], 0x20);
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[S0][0];
                PTS_GET_BEAT_POS.call(c, 0x003d_4568)?;
                if (c.g[V0][0] as i64) < 0 {
                    c.cycles += 5;
                    pts_no_beat(c);
                }
                c.g[V0][0] = c.lw(pts_at(c, S1, PTS_LENGTH));
                c.cycles += 7;
                c.g[A1][0] = c.lw(sp(c).wrapping_add(0x30));
                c.g[V1][0] = pts_srl(c.g[V0][0], 31);
                c.g[V0][0] = pts_addu(c.g[V0][0], c.g[V1][0]);
                c.g[V0][0] = pts_sra(c.g[V0][0], 1);
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                let late = c.g[V0][0] != 0;
                c.g[A0][0] = c.lw(sp(c).wrapping_add(0x24));
                if !late {
                    c.cycles += 2;
                    c.g[A1][0] = pts_addu(c.g[A1][0], c.g[A0][0]);
                }
                break 'set_start;
            }
            // Repeated node (v0 = next node): by its sync mode.
            'mode_200: {
                'free: {
                    c.cycles += 5;
                    c.g[A0][0] = 0x100;
                    c.g[V1][0] = c.lw(pts_at(c, V0, 0x8));
                    c.g[V1][0] &= 0x300;
                    let m100 = c.g[V1][0] == c.g[A0][0];
                    c.g[V0][0] = 0x300;
                    if m100 {
                        break 'free;
                    }
                    c.cycles += 2;
                    let not300 = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x200;
                    if not300 {
                        break 'mode_200;
                    }
                }
                // Resync later: at the repeat's end, rounded down to 10.
                c.cycles += 11;
                c.g[V0][0] = 0x450000;
                c.g[A1][0] = c.lw(sp(c).wrapping_add(0x10));
                c.g[V1][0] = c.lw(PTS_TIME);
                c.g[A0][0] = 0xa;
                c.sw(pts_at(c, S1, PTS_START), 0);
                c.g[V0][0] = 0xffff_ffff_ffff_d8f1;
                c.g[V1][0] = pts_addu(c.g[A1][0], c.g[V1][0]);
                c.divu(0, V1, A0);
                c.sw(pts_at(c, S1, PTS_RESYNC), c.g[V1][0]);
                c.cycles += 4;
                c.g[A1][0] = c.hi[0];
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[A1][0] as u32));
                c.sw(pts_at(c, S1, PTS_RESYNC), c.g[V1][0]);
                return Ok(PtsSynced::Return);
            }
            c.cycles += 2;
            let not200 = c.g[V1][0] != c.g[V0][0];
            c.g[FP][0] = 0x450000;
            if not200 {
                break 'queue_check;
            }
            c.cycles += 2;
            let ahead = (c.g[S6][0] as i64) >= 0;
            c.g[A1][0] = c.lw(sp(c).wrapping_add(0x10));
            if !ahead {
                c.cycles += 5;
                c.g[V0][0] = c.lw(sp(c).wrapping_add(0x4));
                c.g[S6][0] = sx(0u32.wrapping_sub(c.g[S6][0] as u32));
                c.g[V1][0] = c.lw(sp(c).wrapping_add(0x10));
                c.mult(0, V0, S6, V0);
                c.g[A1][0] = pts_addu(c.g[V1][0], c.g[V0][0]);
            }
        }
        // start = now + a1
        c.cycles += 5;
        c.g[V1][0] = 0x450000;
        c.g[V0][0] = c.lw(PTS_TIME);
        c.g[FP][0] = c.g[V1][0];
        c.g[V0][0] = pts_addu(c.g[V0][0], c.g[A1][0]);
        c.sw(pts_at(c, S1, PTS_START), c.g[V0][0]);
    }

    // Queue once half a length has passed since the last synced queue.
    c.cycles += 9;
    c.g[S0][0] = 0x450000;
    c.g[V0][0] = c.lw(r32(c, FP).wrapping_sub(0xbc8));
    c.g[A0][0] = c.lw(r32(c, S0).wrapping_sub(0x864));
    c.g[T0][0] = c.lw(pts_at(c, S1, PTS_LENGTH));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
    c.g[V1][0] = pts_srl(c.g[T0][0], 1);
    c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
    if c.g[V0][0] != 0 {
        c.sw(pts_at(c, S1, PTS_IDLE_SINCE), 0);
        return Ok(PtsSynced::Wait);
    }
    c.cycles += 17;
    c.g[T2][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
    c.g[V0][0] = 1;
    pts_queue_args(c, false);
    c.sw(sp(c).wrapping_add(0x44), c.g[V0][0]);
    call_ptr(c, PTS_PLAT_QUEUE, 0x003d_4690)?;
    c.g[V1][0] = c.lw(r32(c, FP).wrapping_sub(0xbc8));
    c.sw(pts_at(c, S1, PTS_HANDLE), c.g[V0][0]);
    c.sw(r32(c, S0).wrapping_sub(0x864), c.g[V1][0]);
    Ok(PtsSynced::Queued)
}

/// `int PATH_TrackSubmitQueue(PathTrack* track)` (0x3d41a8): queues the
/// track's current node on its platform voice. A track with no node is
/// reset (`-1` handle, `0` returned). Otherwise it sums the voice's queued
/// entries, works out the repeat count of the node (looping tracks), the
/// volume (`track->volume * scale * track->gain`, or the voice's own when
/// negative), the node's length and sample, and either queues at once or,
/// for a looping track synced to the master track, lines the start up with
/// the master's beat (returning -9999 when it must wait). A queued entry is
/// recorded in the track's 16-entry history ring. Returns the voice handle.
pub fn path_track_submit_queue(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0xf0);
        c.g[V0][0] = 0x450000;
        frame.save(c, S1, 0xd0);
        frame.save(c, S4, 0xa0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S6, 0x80);
        c.g[S4][0] = 0;
        frame.save(c, S7, 0x70);
        c.g[S6][0] = 0;
        frame.save(c, FP, 0x60);
        c.g[S7][0] = 0;
        frame.save(c, S0, 0xe0);
        c.g[FP][0] = 0;
        frame.save(c, S2, 0xc0);
        frame.save(c, S3, 0xb0);
        frame.save(c, S5, 0x90);
        frame.save_ra(c, 0x50);
        // The master track (if any).
        c.g[V1][0] = c.lw(PTS_PATH_SYS);
        c.g[V0][0] = c.lw(r32(c, V1));
        c.sw(sp(c).wrapping_add(0x44), 0);
        if (c.g[V0][0] as i64) >= 0 {
            c.cycles += 3;
            c.g[V0][0] = pts_sll(c.g[V0][0], 2);
            c.g[V0][0] = pts_addu(c.g[V1][0], c.g[V0][0]);
            c.g[S4][0] = c.lw(pts_at(c, V0, 0x58));
        }

        'ret: {
            'tail: {
                c.cycles += 3;
                c.g[V0][0] = c.lh(pts_at(c, S1, PTS_NODE));
                c.g[S2][0] = 0;
                if (c.g[V0][0] as i64) < 0 {
                    // No node: reset the track.
                    c.cycles += 10;
                    c.g[V0][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
                    c.g[V1][0] = 0xffff_ffff_ffff_fe0f;
                    c.g[A0][0] = u64::MAX;
                    c.sw(pts_at(c, S1, PTS_NEXT_NODE), 0);
                    c.g[V0][0] &= c.g[V1][0];
                    c.sh(pts_at(c, S1, PTS_NODE), c.g[A0][0]);
                    c.sw(pts_at(c, S1, PTS_FLAGS), c.g[V0][0]);
                    c.g[V0][0] = 0;
                    c.sw(pts_at(c, S1, PTS_HANDLE), c.g[A0][0]);
                    break 'ret;
                }

                // Sum the voice's queued entries.
                c.cycles += 5;
                c.g[V0][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
                c.g[V0][0] = pts_srl(c.g[V0][0], 14) & 0x1f;
                c.g[S0][0] = 0;
                if c.g[V0][0] != 0 {
                    loop {
                        c.cycles += 12;
                        c.g[A0][0] = c.lw(pts_at(c, S1, PTS_PLAT_TRACK));
                        c.g[A1][0] = c.g[S0][0];
                        c.g[S0][0] = pts_addu(c.g[S0][0], 1);
                        call_ptr(c, PTS_GET_QUEUED, 0x003d_4238)?;
                        c.g[V1][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
                        c.g[A0][0] = ((c.g[V0][0] as i64) < 0) as u64;
                        c.g[V0][0] = pts_addu(c.g[S2][0], c.g[V0][0]);
                        c.g[V1][0] = pts_srl(c.g[V1][0], 14) & 0x1f;
                        c.g[V1][0] = (c.g[S0][0] < c.g[V1][0]) as u64;
                        if c.g[A0][0] == 0 {
                            c.g[S2][0] = c.g[V0][0];
                        }
                        if c.g[V1][0] == 0 {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003d_4228);
                        }
                    }
                }

                // The current node (s3) and its repeat count.
                'reload_done: {
                    'reload: {
                        'store: {
                            c.cycles += 12;
                            c.g[V1][0] = 0x450000;
                            c.g[A1][0] = c.lbu(pts_at(c, S1, PTS_LOOPING));
                            c.g[V0][0] = c.lw(PTS_PATH_SYS);
                            c.g[V1][0] = c.lh(pts_at(c, S1, PTS_NODE));
                            c.g[A0][0] = c.lw(pts_at(c, V0, 0x38));
                            c.g[V1][0] = pts_sll(c.g[V1][0], 1);
                            c.g[A2][0] = c.lw(pts_at(c, V0, 0x34));
                            c.g[V1][0] = pts_addu(c.g[V1][0], c.g[A0][0]);
                            c.g[V0][0] = c.lh(r32(c, V1));
                            c.g[V0][0] = pts_sll(c.g[V0][0], 2);
                            c.g[S3][0] = pts_addu(c.g[A2][0], c.g[V0][0]);
                            if c.g[A1][0] == 0 {
                                // One-shot: nothing queued means idle since now.
                                c.cycles += 2;
                                if (c.g[S2][0] as i64) > 0 {
                                    c.g[V0][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw(pts_at(c, S1, PTS_IDLE_SINCE));
                                    c.g[S7][0] = u64::MAX;
                                    if c.g[V0][0] == 0 {
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x450000;
                                        c.g[V1][0] = c.lw(PTS_TIME);
                                        c.sw(pts_at(c, S1, PTS_IDLE_SINCE), c.g[V1][0]);
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
                                }
                                c.cycles += 2;
                                c.g[V0][0] |= 0x1f0;
                                break 'store;
                            }
                            c.cycles += 5;
                            c.g[V0][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
                            c.g[V0][0] = pts_sra(pts_sll(c.g[V0][0], 23), 27);
                            if (c.g[V0][0] as i64) > 0 {
                                c.g[V0][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
                                break 'reload_done;
                            }
                            c.cycles += 17;
                            c.g[A0][0] = c.g[S3][0];
                            c.g[A1][0] = c.g[S4][0];
                            c.g[A2][0] = c.g[SP][0];
                            c.g[FP][0] = 1;
                            PTS_CALC_REPEAT.call(c, 0x003d_42dc)?;
                            c.g[A0][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
                            c.g[V1][0] = c.g[V0][0] & 0x1f;
                            c.g[A1][0] = 0xffff_ffff_ffff_fe0f;
                            c.g[V0][0] = pts_sll(c.g[V0][0], 27);
                            c.g[V1][0] = pts_sll(c.g[V1][0], 4);
                            c.g[A0][0] &= c.g[A1][0];
                            c.g[A0][0] |= c.g[V1][0];
                            c.g[S6][0] = pts_sra(c.g[V0][0], 27);
                            c.g[V0][0] = pts_sra(pts_sll(c.g[A0][0], 23), 27);
                            c.sw(pts_at(c, S1, PTS_FLAGS), c.g[A0][0]);
                            if (c.g[V0][0] as i64) > 0 {
                                break 'reload;
                            }
                            c.cycles += 2;
                            c.g[V0][0] = (c.g[A0][0] & c.g[A1][0]) | 0x10;
                        }
                        c.cycles += 1;
                        c.sw(pts_at(c, S1, PTS_FLAGS), c.g[V0][0]);
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
                }

                // Volume: (int)(volume * scale * gain), else the voice's.
                c.cycles += 20;
                c.g[A0][0] = c.lb(pts_at(c, S1, PTS_VOLUME));
                c.g[V0][0] = pts_sra(pts_sll(c.g[V0][0], 23), 27);
                c.sw(sp(c).wrapping_add(0x40), c.g[V0][0]);
                call_ptr(c, PTS_FLOATSIDF, 0x003d_4330)?;
                c.g[AT][0] = 0x490000;
                c.g[A1][0] = c.ld(PTS_VOLUME_SCALE);
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, PTS_MULDF3, 0x003d_4340)?;
                c.g[A0][0] = c.lb(pts_at(c, S1, PTS_GAIN));
                c.g[S0][0] = c.g[V0][0];
                call_ptr(c, PTS_FLOATSIDF, 0x003d_434c)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, PTS_MULDF3, 0x003d_4358)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, PTS_FIXDFSI, 0x003d_4360)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[V0][0] = 0x450000;
                if (c.g[S2][0] as i64) < 0 {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw(pts_at(c, S1, PTS_PLAT_TRACK));
                    call_ptr(c, PTS_GET_VOLUME, 0x003d_4374)?;
                    c.g[S2][0] = c.g[V0][0];
                    c.g[V0][0] = 0x450000;
                }

                // Next node, length and the sample value (s5).
                c.cycles += 21;
                c.g[V1][0] = c.lw(pts_at(c, S3, 0x8));
                c.g[A2][0] = c.lw(PTS_PATH_SYS);
                c.g[A3][0] = 0;
                c.g[V1][0] = pts_srl(c.g[V1][0], 20);
                c.g[T0][0] = c.lh(pts_at(c, S1, PTS_NODE));
                c.g[A1][0] = c.lw(pts_at(c, A2, 0x38));
                c.g[V1][0] = pts_sll(c.g[V1][0], 1);
                c.g[A0][0] = c.lw(pts_at(c, A2, 0x34));
                c.g[V1][0] = pts_addu(c.g[V1][0], c.g[A1][0]);
                c.g[V0][0] = c.lh(r32(c, V1));
                c.g[V0][0] = pts_sll(c.g[V0][0], 2);
                c.g[A0][0] = pts_addu(c.g[A0][0], c.g[V0][0]);
                c.sw(pts_at(c, S1, PTS_NEXT_NODE), c.g[A0][0]);
                c.g[V0][0] = c.lh(r32(c, S3));
                c.g[A0][0] = c.lw(pts_at(c, A2, 0x50));
                c.g[V0][0] = pts_sll(c.g[V0][0], 3);
                c.g[V0][0] = pts_addu(c.g[V0][0], c.g[A0][0]);
                c.g[V1][0] = c.lw(r32(c, V0).wrapping_sub(4));
                c.sw(pts_at(c, S1, PTS_LENGTH), c.g[V1][0]);
                c.sw(pts_at(c, S1, PTS_BEAT_LENGTH), c.g[V1][0]);
                let mut in_range = false;
                if (c.g[T0][0] as i64) >= 0 {
                    c.cycles += 5;
                    c.g[A1][0] = c.lw(pts_at(c, A2, 0x34));
                    c.g[V0][0] = c.lhu(pts_at(c, A1, 0x12));
                    c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = c.lw(pts_at(c, A2, 0x38));
                        in_range = true;
                    }
                }
                if in_range {
                    c.cycles += 8;
                    c.g[V1][0] = pts_sll(c.g[T0][0], 1);
                    c.g[V1][0] = pts_addu(c.g[V1][0], c.g[V0][0]);
                    c.g[A0][0] = c.lh(r32(c, V1));
                    c.g[A0][0] = pts_sll(c.g[A0][0], 2);
                    c.g[A0][0] = pts_addu(c.g[A1][0], c.g[A0][0]);
                    c.g[V0][0] = c.lh(r32(c, A0));
                    let sample = c.g[V0][0] as i64;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    if sample > 0 {
                        c.cycles += 4;
                        c.g[V1][0] = c.lw(pts_at(c, A2, 0x50));
                        c.g[V0][0] = pts_sll(c.g[V0][0], 3);
                        c.g[V0][0] = pts_addu(c.g[V0][0], c.g[V1][0]);
                        c.g[A3][0] = c.lw(r32(c, V0));
                    }
                    c.cycles += 1;
                    c.g[S5][0] = c.g[A3][0];
                } else {
                    c.cycles += 2;
                    c.g[S5][0] = 0;
                }

                // Queue (t0: the handle to record, or < 0).
                'have_handle: {
                    'reload_handle: {
                        'plain: {
                            c.cycles += 3;
                            c.g[V0][0] = c.lbu(pts_at(c, S1, PTS_LOOPING));
                            if c.g[V0][0] == 0 {
                                c.g[T2][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
                                break 'plain;
                            }
                            'counted: {
                                'clear_idle: {
                                    'unsynced: {
                                        c.cycles += 2;
                                        if c.g[S4][0] == 0 {
                                            c.g[T2][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
                                            break 'unsynced;
                                        }
                                        'is_master: {
                                            c.cycles += 2;
                                            let master = c.g[S4][0] == c.g[S1][0];
                                            c.g[A0][0] = 0x100;
                                            if master {
                                                break 'is_master;
                                            }
                                            match pts_synced(c)? {
                                                PtsSynced::Unsynced => break 'unsynced,
                                                PtsSynced::Return => break 'ret,
                                                PtsSynced::Wait => break 'counted,
                                                PtsSynced::Queued => break 'clear_idle,
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[T2][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
                                    }
                                    // Queue now, start time 0.
                                    c.cycles += 13;
                                    c.sw(pts_at(c, S1, PTS_START), 0);
                                    pts_queue_args(c, true);
                                    call_ptr(c, PTS_PLAT_QUEUE, 0x003d_46d4)?;
                                    c.sw(pts_at(c, S1, PTS_HANDLE), c.g[V0][0]);
                                }
                                c.cycles += 1;
                                c.sw(pts_at(c, S1, PTS_IDLE_SINCE), 0);
                            }
                            // Advance the repeat counter if this queued.
                            c.cycles += 3;
                            c.g[V1][0] = c.lw(sp(c).wrapping_add(0x44));
                            c.g[A2][0] = 0xffff_ffff_ffff_fe0f;
                            if c.g[V1][0] == 0 {
                                break 'reload_handle;
                            }
                            c.cycles += 18;
                            c.g[V0][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
                            c.g[A0][0] = pts_sll(c.g[V0][0], 23);
                            c.g[A1][0] = c.g[V0][0] & c.g[A2][0];
                            c.g[A0][0] = pts_sra(c.g[A0][0], 27);
                            c.g[V0][0] = pts_addu(c.g[A0][0], 1) & 0x1f;
                            c.g[V0][0] = pts_sll(c.g[V0][0], 4);
                            c.g[A1][0] |= c.g[V0][0];
                            c.sw(pts_at(c, S1, PTS_FLAGS), c.g[A1][0]);
                            c.g[V0][0] = c.lw(pts_at(c, S3, 0x4));
                            c.g[V1][0] = pts_srl(c.g[V0][0], 20);
                            c.g[V0][0] = pts_srl(c.g[V0][0], 24);
                            c.g[V1][0] &= 0xf;
                            c.g[V0][0] &= 0xf;
                            c.mult(0, V0, V0, V1);
                            c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let done = c.g[A0][0] == 0;
                            c.g[V0][0] = c.g[A1][0] & c.g[A2][0];
                            if !done {
                                c.cycles += 4;
                                c.g[V0][0] = c.lw(pts_at(c, S3, 0xc));
                                c.g[V0][0] &= 2;
                                if c.g[V0][0] != 0 {
                                    c.g[T0][0] = c.lw(pts_at(c, S1, PTS_HANDLE));
                                    break 'have_handle;
                                }
                                c.cycles += 1;
                                c.g[V0][0] = c.g[A1][0] & c.g[A2][0];
                            }
                            c.cycles += 2;
                            c.sw(pts_at(c, S1, PTS_FLAGS), c.g[V0][0]);
                            break 'reload_handle;
                        }
                        // One-shot: queue now.
                        c.cycles += 12;
                        pts_queue_args(c, true);
                        call_ptr(c, PTS_PLAT_QUEUE, 0x003d_4778)?;
                        c.sw(pts_at(c, S1, PTS_HANDLE), c.g[V0][0]);
                    }
                    c.cycles += 1;
                    c.g[T0][0] = c.lw(pts_at(c, S1, PTS_HANDLE));
                }

                // Record the queued node in the history ring.
                c.cycles += 2;
                c.g[V0][0] = 0x450000;
                if (c.g[T0][0] as i64) < 0 {
                    break 'tail;
                }
                c.cycles += 42;
                c.g[A0][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
                c.g[A2][0] = c.lw(PTS_PATH_SYS);
                c.g[A3][0] = 0xffff_ffff_ff07_0000;
                c.g[V1][0] = c.lh(pts_at(c, S1, PTS_NODE));
                c.g[A0][0] = pts_srl(c.g[A0][0], 15);
                c.g[V0][0] = c.lw(pts_at(c, A2, 0x38));
                c.g[A0][0] &= 0x1f0;
                c.g[A1][0] = c.lw(pts_at(c, S1, PTS_HISTORY));
                c.g[V1][0] = pts_sll(c.g[V1][0], 1);
                c.g[V1][0] = pts_addu(c.g[V1][0], c.g[V0][0]);
                c.g[A3][0] |= 0xffff;
                c.g[A1][0] = pts_addu(c.g[A1][0], c.g[A0][0]);
                c.g[V0][0] = c.lh(r32(c, V1));
                c.g[A0][0] = c.lw(pts_at(c, A2, 0x34));
                c.sw(pts_at(c, A1, 0xc), c.g[T0][0]);
                c.g[V0][0] = pts_sll(c.g[V0][0], 2);
                c.g[S3][0] = pts_addu(c.g[A0][0], c.g[V0][0]);
                c.g[V1][0] = c.lh(pts_at(c, S1, PTS_NODE));
                c.sw(r32(c, A1), c.g[V1][0]);
                c.g[V0][0] = c.lh(r32(c, S3));
                c.g[V1][0] = c.lw(pts_at(c, A2, 0x50));
                c.g[V0][0] = pts_sll(c.g[V0][0], 3);
                c.g[V0][0] = pts_addu(c.g[V0][0], c.g[V1][0]);
                c.g[A0][0] = c.lw(r32(c, V0).wrapping_sub(4));
                c.g[V1][0] = c.lw(sp(c).wrapping_add(0x40));
                c.sw(pts_at(c, A1, 0x4), c.g[A0][0]);
                c.sw(pts_at(c, A1, 0x8), c.g[V1][0]);
                c.g[V1][0] = c.lw(pts_at(c, S1, PTS_FLAGS));
                c.g[V0][0] = pts_srl(c.g[V1][0], 19);
                c.g[A0][0] = c.g[V1][0] & c.g[A3][0];
                c.g[V0][0] &= 0x1f;
                c.g[V0][0] = pts_addu(c.g[V0][0], 1) & 0x1f;
                c.g[V0][0] = pts_sll(c.g[V0][0], 19);
                c.g[A0][0] |= c.g[V0][0];
                c.g[V1][0] = pts_srl(c.g[A0][0], 19) & 0x1f;
                c.g[V0][0] = pts_srl(c.g[A0][0], 14) & 0x1f;
                c.g[V1][0] = (c.g[V1][0] < c.g[V0][0]) as u64;
                c.sw(pts_at(c, S1, PTS_FLAGS), c.g[A0][0]);
                if c.g[V1][0] == 0 {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[A0][0] & c.g[A3][0];
                    c.sw(pts_at(c, S1, PTS_FLAGS), c.g[V0][0]);
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw(pts_at(c, S1, PTS_HANDLE));
        }

        c.cycles += 12;
        frame.restore(c, S0, 0xe0);
        frame.restore(c, S1, 0xd0);
        frame.restore(c, S2, 0xc0);
        frame.restore(c, S3, 0xb0);
        frame.restore(c, S4, 0xa0);
        frame.restore(c, S5, 0x90);
        frame.restore(c, S6, 0x80);
        frame.restore(c, S7, 0x70);
        frame.restore(c, FP, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// PATH_TrackQueueNode (0x3d3900, EA path/music-track library)

/// The path library's state pointer (`0x450000 - 0xbe0`).
pub(crate) const PATH_STATE_PTR: u32 = 0x0044_f420;

/// Per-node callback `void (*)(arg, int node)` and its first argument.
pub(crate) const PATH_NODE_CALLBACK: u32 = 0x0044_f424;

pub(crate) const PATH_NODE_CALLBACK_ARG: u32 = 0x0044_f430;

/// The current track time (divided by 23 into a 7-bit "velocity").
pub(crate) const PATH_TRACK_TIME: u32 = 0x0044_f438;

/// Path state fields: node pool base, node index table (shorts, in words
/// from the pool), branch-target index table, event table, voice pointers.
pub(crate) const PATH_NODES: u32 = 0x34;

pub(crate) const PATH_NODE_INDEX: u32 = 0x38;

pub(crate) const PATH_BRANCH_INDEX: u32 = 0x40;

pub(crate) const PATH_EVENT_TABLE: u32 = 0x54;

pub(crate) const PATH_VOICES: u32 = 0x58;

/// Voice fields: flags word, current node index (short), current node.
pub(crate) const PATH_VOICE_FLAGS: u32 = 0x20;

pub(crate) const PATH_VOICE_NODE: u32 = 0x2c;

pub(crate) const PATH_VOICE_CUR: u32 = 0x30;

pub(crate) const PTQN_REMAP_NODE: Callee = Callee {
    addr: 0x003d_3c90,
    f: ee::s14::PATH_RemapNode,
};

pub(crate) const PTQN_SELECT_BRANCH: Callee = Callee {
    addr: 0x003d_37a0,
    f: ee::s14::PATH_SelectBranch,
};

pub(crate) const PTQN_QUEUE_APPEND: Callee = Callee {
    addr: 0x003d_0b18,
    f: ee::s14::PATH_QueueAppend,
};

pub(crate) const PTQN_COPY_EVENT: Callee = Callee {
    addr: 0x003d_2068,
    f: ee::s14::PATHI_copyevent,
};

pub(crate) const PTQN_ADD_EVENT_TO_MAPS: Callee = Callee {
    addr: 0x003d_1f80,
    f: ee::s14::PATH_AddEventToMaps,
};

/// `memset` (through the function table).
pub(crate) const PTQN_MEMSET: u32 = 0x0041_6210;

/// `PATH_EventMaps` base (0x517610); its +0x30 is the event map handle.
pub(crate) const PTQN_EVENT_MAPS: u32 = 0x0051_7610;

/// `k = (short)RemapNode(...)` (v0 on return), then the node `e` of index `k`
/// into s0, the raw first halfword (its type) into a1 and the loaded type in
/// v1 / v0 as the loop-end reload does.
pub(crate) fn ptqn_reload_node(c: &mut Cpu) {
    c.g[A1][0] = c.lw(PATH_STATE_PTR);
    c.g[V0][0] = sx(r32(c, V0) << 16);
    c.g[S1][0] = sx((r32(c, V0) as i32 >> 16) as u32);
    c.g[A0][0] = c.lw(r32(c, A1).wrapping_add(PATH_NODE_INDEX));
    c.g[V1][0] = sx(r32(c, S1) << 1);
    c.g[A2][0] = c.lw(r32(c, A1).wrapping_add(PATH_NODES));
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
    c.g[V0][0] = c.lh(r32(c, V1));
    c.g[V0][0] = sx(r32(c, V0) << 2);
    c.g[S0][0] = sx(r32(c, A2).wrapping_add(r32(c, V0)));
    c.g[V1][0] = c.lh(r32(c, S0));
}

/// `vel = (track time / 23) & 0x7f` into s3 (with the trap-check cycles).
pub(crate) fn ptqn_vel_from_time(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.lw(PATH_TRACK_TIME);
    c.cycles += 4;
    c.divu(0, V0, S4);
    c.g[V0][0] = c.lo[0];
    c.g[S3][0] = c.g[V0][0] & 0x7f;
}

/// Type-0 node with an event (`w8 & 0x7c00`): builds a 16-byte event and a
/// queue on the stack, appends, copies it and adds it to the event maps.
pub(crate) fn ptqn_queue_node_event(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 17;
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x10;
    call_ptr(c, PTQN_MEMSET, 0x003d_39c8)?; // memset(&q, 0, 16)
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x10;
    call_ptr(c, PTQN_MEMSET, 0x003d_39d8)?; // memset(&ev, 0, 16)
    let stk = sp(c);
    c.g[V1][0] = c.lw(PATH_STATE_PTR);
    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(8));
    c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(PATH_EVENT_TABLE));
    c.g[V0][0] = sx(r32(c, V0) >> 8);
    c.g[V0][0] &= 0x7c;
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
    // t = event table[((w8 >> 10) & 0x1f) - 1]
    c.g[A0][0] = c.lw(r32(c, V0).wrapping_sub(4));
    c.g[V1][0] = c.g[A0][0] & 0x80;
    c.g[S5][0] = sx(stk.wrapping_add(0x10));
    if c.g[V1][0] != 0 {
        c.cycles += 3;
        c.g[V0][0] = 0x64;
        c.sb(stk.wrapping_add(0xc), 0x64); // ev.bC = 100
    } else {
        c.cycles += 1;
        c.sb(stk.wrapping_add(0xc), 0);
    }
    c.cycles += 62;
    c.g[V1][0] = sx(r32(c, A0) >> 7);
    c.g[V0][0] = 0xffff_7fff_ffff_ffff; // clears ev.f1 (bit 47)
    c.g[A0][0] = c.ld(stk.wrapping_add(8));
    c.g[V1][0] &= 1;
    c.g[V1][0] <<= 47;
    c.g[A2][0] = 6;
    c.g[A0][0] &= c.g[V0][0];
    c.g[T0][0] = 0xffff_80ff_ffff_ffff; // clears ev.f7 (bits 40..46)
    c.g[A0][0] |= c.g[V1][0];
    c.g[V0][0] = 0x25a0;
    c.sd(stk.wrapping_add(8), c.g[A0][0]);
    c.g[A1][0] = 0x1f00_0001;
    c.sh(stk.wrapping_add(0xe), c.g[V0][0]); // ev.hE = 0x25a0
    c.sb(stk.wrapping_add(9), c.g[A2][0]); // ev.b9 = 6
    c.g[A0][0] = 0xffff_0fff_ffff_ffff; // clears q.b (bits 44..47)
    c.g[A2][0] = c.ld(stk.wrapping_add(0x18));
    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(8));
    c.g[V1][0] = c.ld(stk.wrapping_add(8));
    c.g[A2][0] &= c.g[A0][0];
    c.g[V0][0] = sx(r32(c, V0) >> 10);
    c.g[A3][0] = c.lw(PATH_TRACK_TIME);
    c.g[V0][0] &= 0x1f;
    c.g[V1][0] &= c.g[T0][0];
    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
    c.g[T0][0] = u64::MAX;
    c.g[V0][0] &= 0x7f;
    c.sw(stk, c.g[A1][0]); // ev.w0 = 0x1f000001
    c.g[V0][0] <<= 40;
    c.g[A1][0] = c.g[SP][0];
    c.g[V1][0] |= c.g[V0][0];
    c.sd(stk.wrapping_add(0x18), c.g[A2][0]);
    c.sd(stk.wrapping_add(8), c.g[V1][0]);
    c.g[A0][0] = c.g[S5][0];
    c.sb(stk.wrapping_add(8), c.g[T0][0]); // ev.b8 = -1
    c.g[V0][0] = c.lhu(stk.wrapping_add(0xe));
    c.g[V1][0] = c.g[V0][0];
    c.g[A3][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
    c.sw(stk.wrapping_add(0x14), c.g[V1][0]); // q.p1 = ev.hE
    c.sw(stk.wrapping_add(4), c.g[A3][0]); // ev.w4 = time + ev.hE
    PTQN_QUEUE_APPEND.call(c, 0x003d_3aec)?;
    c.g[A0][0] = c.g[S5][0];
    PTQN_COPY_EVENT.call(c, 0x003d_3af4)?;
    c.g[V1][0] = sx(PTQN_EVENT_MAPS);
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.lw(PTQN_EVENT_MAPS.wrapping_add(0x30));
    PTQN_ADD_EVENT_TO_MAPS.call(c, 0x003d_3b08)?;
    Ok(())
}

/// `short PATH_TrackQueueNode(int track, int node, int vel)` (0x3d3900):
/// walks the track's node graph from `node` (remapped through the track),
/// handling the control nodes (type <= 0) on the way: type 0 makes the
/// node current on its voice, queues its event (if any) and calls the node
/// callback; -1 sets the voice's repeat count once per node; -2 and type-0
/// nodes with the time flag take the velocity from the track time; -3 adds
/// a branch event to the event maps. Each step picks the next node with
/// `PATH_SelectBranch`. Returns the first playable node (type > 0),
/// remapped.
pub fn path_track_queue_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0xd0);
        frame.save(c, S3, 0x90);
        frame.save(c, S6, 0x60);
        c.g[S3][0] = c.g[A2][0]; // vel
        c.g[S6][0] = c.g[A0][0]; // track
        frame.save(c, S0, 0xc0);
        frame.save(c, S1, 0xb0);
        frame.save(c, S2, 0xa0);
        frame.save(c, FP, 0x40);
        frame.save(c, S4, 0x80);
        frame.save(c, S5, 0x70);
        frame.save(c, S7, 0x50);
        frame.save_ra(c, 0x30);
        let saved_vel = frame.at(c, 0x20);
        c.sw(saved_vel, c.g[S3][0]);
        PTQN_REMAP_NODE.call(c, 0x003d_393c)?;

        // k = (short)remap; e = node k; v = voices[e->h2 & 0x1f]
        c.g[V1][0] = 0x450000;
        c.g[V0][0] = sx(r32(c, V0) << 16);
        c.g[A0][0] = c.lw(PATH_STATE_PTR);
        c.g[S1][0] = sx((r32(c, V0) as i32 >> 16) as u32);
        c.g[FP][0] = c.g[V1][0];
        c.g[A1][0] = c.lw(r32(c, A0).wrapping_add(PATH_NODE_INDEX));
        c.g[V1][0] = sx(r32(c, S1) << 1);
        c.g[A2][0] = c.lw(r32(c, A0).wrapping_add(PATH_NODES));
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
        c.g[V0][0] = c.lh(r32(c, V1));
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[S0][0] = sx(r32(c, A2).wrapping_add(r32(c, V0)));
        c.g[V1][0] = c.lhu(r32(c, S0).wrapping_add(2));
        c.g[V0][0] = c.lh(r32(c, S0));
        c.g[V1][0] &= 0x1f;
        c.g[A1][0] = c.lhu(r32(c, S0));
        c.g[V1][0] = sx(r32(c, V1) << 2);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
        let playable = (c.g[V0][0] as i64) > 0;
        c.g[S2][0] = c.lw(r32(c, A0).wrapping_add(PATH_VOICES));

        if !playable {
            c.cycles += 3;
            c.g[S7][0] = 0x450000;
            c.g[S4][0] = 23;
            loop {
                c.cycles += 4;
                c.g[V0][0] = sx(r32(c, A1) << 16);
                c.g[V1][0] = sx((r32(c, V0) as i32 >> 16) as u32);
                let ty = c.g[V1][0];
                c.g[V0][0] = u64::MAX;
                // true: the paths through 0x3d3bfc (one more cycle).
                let via_bfc;
                if ty == 0 {
                    c.cycles += 5;
                    let cur = r32(c, S2).wrapping_add(PATH_VOICE_CUR);
                    c.sw(cur, c.g[S0][0]); // v->cur = e
                    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(8)) & 0x7c00;
                    c.g[A0][0] = sx(sp(c).wrapping_add(0x10));
                    if c.g[V0][0] != 0 {
                        ptqn_queue_node_event(c)?;
                    }
                    c.cycles += 6;
                    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(4));
                    c.g[V1][0] = 0xe0000;
                    c.g[A0][0] = 0x20000;
                    c.g[V0][0] &= c.g[V1][0];
                    let timed = c.g[V0][0] == c.g[A0][0];
                    c.g[V0][0] = 0x450000;
                    if timed {
                        ptqn_vel_from_time(c);
                        c.g[V0][0] = 0x450000;
                    }
                    c.cycles += 3;
                    c.g[V1][0] = c.lw(PATH_NODE_CALLBACK);
                    let callback = c.g[V1][0];
                    c.g[V0][0] = 0x450000;
                    if callback == 0 {
                        via_bfc = true;
                    } else {
                        c.cycles += 5;
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A0][0] = c.lw(PATH_NODE_CALLBACK_ARG);
                        call_ptr(c, callback as u32, 0x003d_3b54)?;
                        c.g[A0][0] = c.g[S1][0];
                        via_bfc = false;
                    }
                } else {
                    c.cycles += 2;
                    let is_repeat = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = (-2i64) as u64;
                    if is_repeat {
                        // -1: repeat count in the top nibble of the node word.
                        c.cycles += 5;
                        c.g[A0][0] = c.lw(r32(c, S0));
                        c.g[V0][0] = 0xffff_ffff_f000_0000;
                        c.g[V0][0] &= c.g[A0][0];
                        if c.g[V0][0] == 0 {
                            c.g[A0][0] = c.g[S1][0];
                            via_bfc = false;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lh(r32(c, S2).wrapping_add(PATH_VOICE_NODE));
                            let same = c.g[S1][0] == c.g[V0][0];
                            c.g[V1][0] = 0xffff_ffff_ffff_fff0;
                            if !same {
                                c.cycles += 7;
                                let flags = r32(c, S2).wrapping_add(PATH_VOICE_FLAGS);
                                c.g[V0][0] = c.lw(flags);
                                c.g[A0][0] = sx(r32(c, A0) >> 28);
                                let node = r32(c, S2).wrapping_add(PATH_VOICE_NODE);
                                c.sh(node, c.g[S1][0]);
                                c.g[V0][0] &= c.g[V1][0];
                                c.g[V0][0] |= c.g[A0][0];
                                c.sw(flags, c.g[V0][0]);
                            }
                            via_bfc = true;
                        }
                    } else {
                        c.cycles += 2;
                        let is_timed = c.g[V1][0] == c.g[V0][0];
                        c.g[V0][0] = (-3i64) as u64;
                        if is_timed {
                            ptqn_vel_from_time(c);
                            via_bfc = true;
                        } else {
                            c.cycles += 2;
                            let is_branch_event = c.g[V1][0] == c.g[V0][0];
                            c.g[A0][0] = c.g[S1][0];
                            if !is_branch_event {
                                via_bfc = false;
                            } else {
                                // -3: AddEventToMaps(state->x30, node b[(wC >> 2) & 0xfff])
                                c.cycles += 12;
                                c.g[V1][0] = c.lw(PATH_STATE_PTR);
                                c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0xc));
                                c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(PATH_BRANCH_INDEX));
                                c.g[V0][0] = sx(r32(c, V0) >> 1);
                                c.g[A2][0] = c.lw(r32(c, V1).wrapping_add(PATH_NODES));
                                c.g[V0][0] &= 0x1ffe;
                                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                                c.g[A1][0] = c.lh(r32(c, V0));
                                c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x30));
                                c.g[A1][0] = sx(r32(c, A1) << 2);
                                c.g[A1][0] = sx(r32(c, A2).wrapping_add(r32(c, A1)));
                                PTQN_ADD_EVENT_TO_MAPS.call(c, 0x003d_3bfc)?;
                                via_bfc = true;
                            }
                        }
                    }
                }
                if via_bfc {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S1][0];
                }
                // next = RemapNode(track, SelectBranch(k, vel)); vel = saved
                c.cycles += 19;
                c.g[A1][0] = c.g[S3][0];
                PTQN_SELECT_BRANCH.call(c, 0x003d_3c08)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S6][0];
                let saved_vel = sp(c).wrapping_add(0x20);
                c.g[S3][0] = c.lw(saved_vel);
                PTQN_REMAP_NODE.call(c, 0x003d_3c18)?;
                ptqn_reload_node(c);
                let more = (c.g[V1][0] as i64) <= 0;
                c.g[A1][0] = c.lhu(r32(c, S0));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003d_3998);
                }
            }
        }

        c.cycles += 17;
        c.g[A0][0] = c.g[S6][0];
        c.g[A1][0] = c.g[S1][0];
        PTQN_REMAP_NODE.call(c, 0x003d_3c58)?;
        c.g[V0][0] = sx(r32(c, V0) << 16);
        frame.restore(c, S0, 0xc0);
        frame.restore(c, S1, 0xb0);
        c.g[V0][0] = sx((r32(c, V0) as i32 >> 16) as u32);
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

// ---------------------------------------------------------------------------
// Shared: gcc's inline 32-byte block copy

/// gcc's inline struct copy loop: 32-byte blocks from register `src` into
/// register `dst` until `src` reaches the value in register `end`, through
/// the temporaries `tmp` (`ldl/ldr` + `sdl/sdr`, 19 cycles a block, when
/// `unaligned`; `ld`/`sd`, 11 cycles, through `tmp_aligned` otherwise).
/// Back edges check events at `head` / `head_aligned`.
pub(crate) fn oc374_block_copy(
    c: &mut Cpu,
    src: usize,
    dst: usize,
    end: usize,
    unaligned: bool,
    tmp: [usize; 4],
    head: u32,
    tmp_aligned: [usize; 4],
    head_aligned: u32,
) {
    loop {
        let s = r32(c, src);
        let d = r32(c, dst);
        if unaligned {
            c.cycles += 19;
            for (i, &t) in tmp.iter().enumerate() {
                let o = 8 * i as u32;
                c.ldl(t, s.wrapping_add(o + 7));
                c.ldr(t, s.wrapping_add(o));
            }
            for (i, &t) in tmp.iter().enumerate() {
                let o = 8 * i as u32;
                c.sdl(t, d.wrapping_add(o + 7));
                c.sdr(t, d.wrapping_add(o));
            }
        } else {
            c.cycles += 11;
            for (i, &t) in tmp_aligned.iter().enumerate() {
                c.g[t][0] = c.ld(s.wrapping_add(8 * i as u32));
            }
            for (i, &t) in tmp_aligned.iter().enumerate() {
                c.sd(d.wrapping_add(8 * i as u32), c.g[t][0]);
            }
        }
        c.g[src][0] = sx(s.wrapping_add(0x20));
        let more = c.g[src][0] != c.g[end][0];
        c.g[dst][0] = sx(d.wrapping_add(0x20));
        if !more {
            if unaligned {
                c.cycles += 2;
            }
            return;
        }
        if event_due(c) {
            deliver_event_here(c, if unaligned { head } else { head_aligned });
        }
    }
}

// ---------------------------------------------------------------------------
// PATH_TrackAdvance (0x3d3d20)

/// Path state: the queued-event write pointer (restored on a failed submit).
pub(crate) const PATH_EVENT_WRITE: u32 = 0x91c;

/// Voice fields: velocity byte, "chord" flag byte, current node (short);
/// the voice struct is 0x110 bytes.
pub(crate) const PATH_VOICE_VEL: u32 = 0x1;

pub(crate) const PATH_VOICE_CHORD: u32 = 0x2f;

pub(crate) const PATH_VOICE_CUR_NODE: u32 = 0x36;

pub(crate) const PTA_SELECT_BRANCH: Callee = Callee {
    addr: 0x003d_37a0,
    f: ee::s14::PATH_SelectBranch,
};

pub(crate) const PTA_TRACK_QUEUE_NODE: Callee = Callee {
    addr: 0x003d_3900,
    f: ee::s14::PATH_TrackQueueNode,
};

pub(crate) const PTA_POST_NODE_EVENTS: Callee = Callee {
    addr: 0x003d_4050,
    f: ee::s14::PATH_PostNodeEvents,
};

pub(crate) const PTA_TRACK_SUBMIT_QUEUE: Callee = Callee {
    addr: 0x003d_41a8,
    f: ee::s14::PATH_TrackSubmitQueue,
};

/// `void PATH_TrackAdvance(int voice)` (0x3d3d20): advances a voice to its
/// next node. Saves the voice (0x110 bytes on the stack) and the event
/// write pointer; repeats the current node while its repeat count allows
/// (`wC & 2`, count <= rows * columns of w4), else selects the branch for
/// the voice's velocity and queues the path to the next playable node. A
/// node owned by another voice only posts its events (the voice stops:
/// node -1); otherwise the voice moves there and submits its queue, and a
/// failed submit (negative, not -9999) restores the saved voice and
/// pointer.
pub fn path_track_advance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x160);
        c.g[V1][0] = 0x450000;
        frame.save(c, S2, 0x130);
        c.g[A0][0] = sx(r32(c, A0) << 2);
        c.g[V0][0] = c.lw(PATH_STATE_PTR);
        c.g[S2][0] = c.g[V1][0];
        frame.save(c, S0, 0x150);
        c.g[T0][0] = u64::MAX; // sel = -1
        frame.save(c, S1, 0x140);
        c.g[A1][0] = c.g[SP][0];
        frame.save_ra(c, 0x110);
        frame.save(c, S3, 0x120);
        c.g[S3][0] = c.lw(r32(c, V0).wrapping_add(PATH_EVENT_WRITE)); // old
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
        c.g[S0][0] = c.lw(r32(c, V0).wrapping_add(PATH_VOICES)); // v
        c.g[V0][0] = c.g[S0][0] & 7;
        let unaligned = c.g[V0][0] != 0;
        c.g[V1][0] = c.g[S0][0];
        // save = *v
        c.cycles += 1;
        c.g[V0][0] = sx(r32(c, S0).wrapping_add(0x100));
        oc374_block_copy(
            c,
            V1,
            A1,
            V0,
            unaligned,
            [A0, A2, A3, T1],
            0x003d_3d6c,
            [A0, A2, A3, T1],
            0x003d_3dc4,
        );
        c.cycles += 25;
        let (src, dst) = (r32(c, V1), r32(c, A1));
        c.ldl(V0, src.wrapping_add(7));
        c.ldr(V0, src);
        c.ldl(A0, src.wrapping_add(0xf));
        c.ldr(A0, src.wrapping_add(8));
        c.sdl(V0, dst.wrapping_add(7));
        c.sdr(V0, dst);
        c.sdl(A0, dst.wrapping_add(0xf));
        c.g[S1][0] = u64::MAX; // ch = -1
        c.sdr(A0, dst.wrapping_add(8));

        // k = v->h36; e = node k; n = v->ch; ch = v->b2F ? 1 : -1
        let voice = r32(c, S0);
        c.g[A1][0] = c.lw(PATH_STATE_PTR);
        c.g[A3][0] = 1;
        c.g[T1][0] = c.lh(voice.wrapping_add(PATH_VOICE_CUR_NODE));
        c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(PATH_NODE_INDEX));
        c.g[V0][0] = sx(r32(c, T1) << 1);
        c.g[A0][0] = c.lw(voice.wrapping_add(PATH_VOICE_FLAGS));
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        c.g[A2][0] = c.lw(r32(c, A1).wrapping_add(PATH_NODES));
        c.g[V1][0] = c.lh(r32(c, V0));
        c.g[A0][0] = sx(r32(c, A0) << 23);
        c.g[V0][0] = c.lbu(voice.wrapping_add(PATH_VOICE_CHORD));
        c.g[A0][0] = sx((r32(c, A0) as i32 >> 27) as u32); // n
        c.g[V1][0] = sx(r32(c, V1) << 2);
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(r32(c, V1)));
        let no_count = (c.g[A0][0] as i64) <= 0;
        if c.g[V0][0] != 0 {
            c.g[S1][0] = c.g[A3][0];
        }
        // The current node repeats (sel = k, ch = n).
        let mut repeat = false;
        if no_count {
            c.cycles += 1;
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(0xc)) & 2;
            if c.g[V0][0] != 0 {
                c.cycles += 9;
                c.g[V0][0] = c.lw(r32(c, A2).wrapping_add(4));
                c.g[V1][0] = sx(r32(c, V0) >> 20);
                c.g[V0][0] = sx(r32(c, V0) >> 24);
                c.g[V1][0] &= 0xf;
                c.g[V0][0] &= 0xf;
                c.mult(0, V0, V0, V1);
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.cycles += 3;
                    c.g[T0][0] = c.g[T1][0];
                    c.g[S1][0] = c.g[A0][0];
                    repeat = true;
                }
            }
        }
        if !repeat {
            c.g[A0][0] = c.lh(r32(c, S0).wrapping_add(PATH_VOICE_CUR_NODE));
            c.cycles += 2;
            if (c.g[A0][0] as i64) >= 0 {
                c.cycles += 3;
                c.g[A1][0] = c.lbu(r32(c, S0).wrapping_add(PATH_VOICE_VEL));
                PTA_SELECT_BRANCH.call(c, 0x003d_3ea8)?;
                c.g[T0][0] = c.g[V0][0];
            }
        }
        c.cycles += 2;
        c.g[A1][0] = c.g[T0][0];
        if (c.g[T0][0] as i64) >= 0 {
            c.cycles += 4;
            c.g[A0][0] = c.lh(r32(c, S0).wrapping_add(PATH_VOICE_CUR_NODE));
            c.g[A2][0] = c.lbu(r32(c, S0).wrapping_add(PATH_VOICE_VEL));
            PTA_TRACK_QUEUE_NODE.call(c, 0x003d_3ec0)?;
            c.g[T0][0] = c.g[V0][0];
        }

        // w = voices[node sel ->h2 & 0x1f]
        c.cycles += 15;
        c.g[A0][0] = c.lw(PATH_STATE_PTR);
        c.g[V1][0] = sx(r32(c, T0) << 1);
        c.g[A1][0] = c.lw(r32(c, A0).wrapping_add(PATH_NODE_INDEX));
        c.g[A2][0] = c.lw(r32(c, A0).wrapping_add(PATH_NODES));
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
        c.g[V0][0] = c.lh(r32(c, V1));
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(r32(c, V0)));
        c.g[V1][0] = c.lhu(r32(c, A2).wrapping_add(2)) & 0x1f;
        c.g[V1][0] = sx(r32(c, V1) << 2);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
        c.g[A2][0] = c.lw(r32(c, A0).wrapping_add(PATH_VOICES));
        let voice = r32(c, S0);
        if c.g[A2][0] != 0 {
            c.cycles += 2;
            let own = c.g[A2][0] == c.g[S0][0];
            c.g[A0][0] = 0xffff_ffff_ffff_fe0f;
            if !own {
                // Another voice's node: post its events, stop this voice.
                c.cycles += 12;
                c.g[V1][0] = c.lw(voice.wrapping_add(PATH_VOICE_FLAGS));
                c.g[V0][0] = c.g[S1][0] & 0x1f;
                c.g[A1][0] = u64::MAX;
                c.g[V1][0] &= c.g[A0][0];
                c.g[V0][0] = sx(r32(c, V0) << 4);
                c.g[V1][0] |= c.g[V0][0];
                c.sh(voice.wrapping_add(PATH_VOICE_CUR_NODE), c.g[A1][0]);
                c.sw(voice.wrapping_add(PATH_VOICE_FLAGS), c.g[V1][0]);
                c.g[A0][0] = c.g[T0][0];
                PTA_POST_NODE_EVENTS.call(c, 0x003d_3f30)?;
                frame.restore(c, S0, 0x150);
                return pta_epilogue(c, frame);
            }
            c.cycles += 1;
        }
        c.g[V0][0] = c.lw(voice.wrapping_add(PATH_VOICE_FLAGS));
        // v->h36 = sel; v->ch = ch; r = SubmitQueue(v)
        c.cycles += 12;
        c.g[A0][0] = 0xffff_ffff_ffff_fe0f;
        c.g[V1][0] = c.g[S1][0] & 0x1f;
        c.sh(voice.wrapping_add(PATH_VOICE_CUR_NODE), c.g[T0][0]);
        c.g[V1][0] = sx(r32(c, V1) << 4);
        c.g[V0][0] &= c.g[A0][0];
        c.g[V0][0] |= c.g[V1][0];
        c.g[A0][0] = c.g[S0][0];
        c.sw(voice.wrapping_add(PATH_VOICE_FLAGS), c.g[V0][0]);
        PTA_TRACK_SUBMIT_QUEUE.call(c, 0x003d_3f60)?;
        c.g[V1][0] = c.g[V0][0];
        if (c.g[V1][0] as i64) >= 0 {
            frame.restore(c, S0, 0x150);
            return pta_epilogue(c, frame);
        }
        c.cycles += 3;
        c.g[V0][0] = (-9999i64) as u64;
        let busy = c.g[V1][0] == c.g[V0][0];
        c.g[V1][0] = c.lw(PATH_STATE_PTR);
        if !busy {
            // Failed: restore the event write pointer and the voice.
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[SP][0];
            c.g[V0][0] = c.g[A0][0] & 7;
            let unaligned = c.g[V0][0] != 0;
            c.sw(r32(c, V1).wrapping_add(PATH_EVENT_WRITE), c.g[S3][0]);
            c.cycles += 1;
            c.g[V0][0] = sx(sp(c).wrapping_add(0x100));
            oc374_block_copy(
                c,
                A1,
                A0,
                V0,
                unaligned,
                [T0, T1, V1, A2],
                0x003d_3f90,
                [A3, T0, T1, V1],
                0x003d_3fe8,
            );
            c.cycles += 8;
            let (src, dst) = (r32(c, A1), r32(c, A0));
            c.ldl(A2, src.wrapping_add(7));
            c.ldr(A2, src);
            c.ldl(A3, src.wrapping_add(0xf));
            c.ldr(A3, src.wrapping_add(8));
            c.sdl(A2, dst.wrapping_add(7));
            c.sdr(A2, dst);
            c.sdl(A3, dst.wrapping_add(0xf));
            c.sdr(A3, dst.wrapping_add(8));
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x150);
        pta_epilogue(c, frame)
    })
}

/// `PATH_TrackAdvance`'s epilogue (s0 already reloaded).
pub(crate) fn pta_epilogue(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 6;
    frame.restore(c, S1, 0x140);
    frame.restore(c, S2, 0x130);
    frame.restore(c, S3, 0x120);
    frame.restore_ra(c, 0x110);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// SpeechSystem_StreamStartCallback
// ---------------------------------------------------------------------------

/// Stream message fields: `+0x18` the speech entry (`+0` id, `+4` volume
/// slot, `+0x8d0` arg), `+0x1c` the request it was started for, `+0x20`
/// the stream slot (-1: none).
pub(crate) const STREAM_START_ENTRY: u32 = 0x18;

pub(crate) const STREAM_START_REQUEST: u32 = 0x1c;

pub(crate) const STREAM_START_SLOT: u32 = 0x20;

pub(crate) const STREAM_START_INSTANCE_SLOT: Callee = Callee {
    addr: 0x002b_1220,
    f: ee::s08::SpeechSystem_GetInstanceStreamSlot,
};

pub(crate) const STREAM_START_CURRENT_REQUEST: Callee = Callee {
    addr: 0x002a_b150,
    f: ee::s08::StreamArray_GetCurrentRequest,
};

pub(crate) const STREAM_START_ADD_TIMER: Callee = Callee {
    addr: 0x002a_dca0,
    f: ee::s08::BankMonitor_AddTimer,
};

pub(crate) const STREAM_START_VOLUME_SCALE: Callee = Callee {
    addr: 0x002a_b0d0,
    f: ee::s08::StreamArray_SetVolumeScalePtr,
};

pub(crate) const STREAM_START_CLEAR_PREBUFFER: Callee = Callee {
    addr: 0x002b_1248,
    f: ee::s08::SpeechSystem_ClearPrebuffer,
};

/// `int SpeechSystem_StreamStartCallback(SpeechSystem* s, StreamMsg* m)`
/// (0x2b1268): a stream started. If its instance's current request is no
/// longer the message's, retries in 200 ticks (BankMonitor_AddTimer with
/// the message) and returns 1. Otherwise, with a stream slot, asks the
/// system's volume method (vtable `+0x18`) for a scale pointer and sets it
/// on the entry's volume slot; then clears the prebuffer and returns its
/// result.
pub fn speech_system_stream_start_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        c.g[S0][0] = c.lw(r32(c, S2).wrapping_add(STREAM_START_ENTRY));
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x6c4));
        c.g[A1][0] = c.lw(r32(c, S0));
        STREAM_START_INSTANCE_SLOT.call(c, 0x002b_1294)?;
        c.g[A0][0] = c.lw(r32(c, S1));
        c.g[A1][0] = c.g[V0][0];
        STREAM_START_CURRENT_REQUEST.call(c, 0x002b_12a0)?;
        c.g[T0][0] = c.lw(r32(c, S2).wrapping_add(STREAM_START_REQUEST));
        let current = c.g[V0][0] == c.g[T0][0];
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x6c4));
        if !current {
            // BankMonitor_AddTimer(params, 200, timer cb, entry, request,
            // slot, 0, 0).
            c.cycles += 14;
            c.ldl(A2, 0x004a_37e7);
            c.ldr(A2, 0x004a_37e0);
            c.g[A1][0] = 0xc8;
            c.g[V0][0] = c.lw(r32(c, V1));
            c.g[T2][0] = 0;
            c.g[T1][0] = c.lw(r32(c, S2).wrapping_add(STREAM_START_SLOT));
            c.g[T3][0] = 0;
            c.g[A0][0] = c.lw(r32(c, V0));
            c.g[A3][0] = c.lw(r32(c, S2).wrapping_add(STREAM_START_ENTRY));
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x1d8));
            STREAM_START_ADD_TIMER.call(c, 0x002b_12dc)?;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 4;
            c.g[A2][0] = c.lw(r32(c, S2).wrapping_add(STREAM_START_SLOT));
            c.g[V0][0] = u64::MAX;
            if c.g[A2][0] != c.g[V0][0] {
                // scale = s->vt[3](s, entry->id, slot, entry->arg).
                c.cycles += 9;
                c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(0x1bc));
                c.g[A1][0] = c.lw(r32(c, S0));
                c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x18));
                c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x1c));
                c.g[A3][0] = c.lw(r32(c, S0).wrapping_add(0x8d0));
                let method = r32(c, V0);
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                call_ptr(c, method, 0x002b_1310)?;
                c.g[A2][0] = c.g[V0][0];
                if c.g[V0][0] != 0 {
                    c.cycles += 3;
                    c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(4));
                    c.g[A0][0] = c.lw(r32(c, S1));
                    STREAM_START_VOLUME_SCALE.call(c, 0x002b_1324)?;
                }
                c.cycles += 1;
            }
            c.g[A0][0] = c.g[S1][0];
            c.cycles += 2;
            c.g[A1][0] = c.g[S2][0];
            STREAM_START_CLEAR_PREBUFFER.call(c, 0x002b_1330)?;
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
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;

    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::fill;

    use crate::difftest::put_words;
    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    use crate::difftest::RA as TEST_RA;

    use crate::difftest::SP as STACK_TOP;
    use crate::difftest::SP as STACK;
    pub(crate) use crate::game::ai::ai::tests::fake_check;
    pub(crate) use crate::game::ai::ai::tests::leaf;

    pub(crate) use crate::game::ai::ai::tests::uf;
    pub(crate) use crate::game::ai::ai::tests::CHILD;
    pub(crate) use crate::game::be::bedbprofile::tests::strict;
    pub(crate) use crate::game::be::beprepoststate::tests::SEEDS as SEEDS_8;
    pub(crate) use crate::game::bx::bxstring::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::bx::bxstringctor::tests::game;

    pub(crate) use crate::game::bx::bxstringctor::tests::SCRATCH;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::common;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear as clear_4;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::SEEDS_7 as SEEDS_6;

    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::b;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_9;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_18 as base_10;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_3;

    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_4 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::call_machine;
    pub(crate) use crate::game::fe::feasyncfile::tests::check_all;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;

    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals as game_globals_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::globals_5 as globals;

    pub(crate) use crate::game::fe::feasyncfile::tests::put_str_4 as put_str;

    pub(crate) use crate::game::fe::feasyncfile::tests::strict_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::w_3 as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_4 as world;

    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ as OBJ_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH as SCRATCH_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_12;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS_15;

    pub(crate) use crate::game::fe::fepopupmisc::tests::wb;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::belib;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::w;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::SEEDS as SEEDS_3;

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
    pub(crate) use crate::game::replay::replaycache::tests::dtor_setup;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear as clear_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS_5;
    pub(crate) use crate::game::sound::bankmanager::tests::mixed;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS;

    pub(crate) use crate::game::visualfx::worldlightman::tests::dtor_case;

    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3;

    use ssx3_ee as ee;

    use ssx3_ee::s08;

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
                let _r = machine(c, seed, 0x2b3a, 4);
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

    /// PATH_SetVolumePercent: not initialised, a selection in progress, or
    /// the four maps (loaded or not, flag nibbles matching the mask or not)
    /// with 24 tracks (present or not); masks -1 or random, volumes in and
    /// out of 0..100.
    #[test]
    fn path_set_volume_percent_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA};
        let w_m2 = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
        check(
            ee::s14::f_003d5508__generated,
            path_set_volume_percent,
            300,
            &|c, seed| {
                let mut r_m2 = machine(c, seed, 0x3d_5508, 0);
                c.pc = 0x003d_5508;
                c.bus.write8(0x0044_f42c, (seed % 11 != 0) as u8);
                c.bus.write8(0x0044_f42e, r_m2.next_u32() as u8);
                w_m2(
                    c,
                    0x0044_f430,
                    if seed % 13 == 1 {
                        r_m2.next_u32() | 1
                    } else {
                        0
                    },
                );
                w_m2(c, 0x0044_f420, r_m2.next_u32());
                let mask = if seed % 4 == 0 {
                    u32::MAX
                } else {
                    r_m2.next_u32()
                };
                for i in 0..4u32 {
                    let map = 0x0051_7610 + 0x928 * i;
                    w_m2(c, map + 0x34, (r_m2.next_u32() % 4 != 0) as u32);
                    let flags = match r_m2.next_u32() % 3 {
                        0 => r_m2.next_u32(),
                        1 => mask & 0xff00_0000 | 1,
                        _ => mask & 0xf000_0000,
                    };
                    w_m2(c, map + 0x30, flags);
                    for j in 0..24u32 {
                        let t = if r_m2.next_u32() % 3 == 0 {
                            0
                        } else {
                            DATA + 0x40 * (j + 24 * i)
                        };
                        w_m2(c, map + 0x58 + 4 * j, t);
                    }
                }
                c.g[A0][0] = sx(mask);
                c.g[A1][0] = match seed % 5 {
                    0 => 200,
                    1 => sx(0xffff_ff80),
                    _ => r_m2.next_u32() as u64 | (r_m2.next_u32() as u64) << 32,
                };
            },
        );
    }

    /// PATH_SelectMasterTrack: the state at DATA (pointer at 0x44f420) with
    /// 24 track slots, each empty, disabled, with a negative `+0x36`, no
    /// header, or a candidate with a random length (sometimes equal to an
    /// earlier one, sometimes with the top bit set) and header word.
    #[test]
    fn path_select_master_track_matches() {
        check(
            ee::s14::f_003d4b00__generated,
            path_select_master_track,
            300,
            &|c, seed| {
                let mut r_m2 = Rng(0x003d_4b00 ^ (seed as u64) << 9);
                scramble(c, &mut r_m2);
                let state = DATA;
                c.bus.write32(PATH_STATE, state);
                fill(c, &mut r_m2, state, 4);
                let lengths = [r_m2.next_u32() % 100, r_m2.next_u32()];
                for i in 0..24u32 {
                    let track = DATA + 0x1000 + 0x100 * i;
                    let header = DATA + 0x4000 + 0x10 * i;
                    let empty = r_m2.next_u32() % 6 == 0;
                    c.bus
                        .write32(state + PATH_TRACKS + 4 * i, if empty { 0 } else { track });
                    fill(c, &mut r_m2, track, 0x10);
                    c.bus
                        .write8(track + TRACK_DISABLED, (r_m2.next_u32() % 5 == 0) as u8);
                    let flags = if r_m2.next_u32() % 5 == 0 {
                        0x8000 | r_m2.next_u32()
                    } else {
                        r_m2.next_u32() & 0x7fff
                    };
                    c.bus.write16(track + TRACK_FLAGS, flags as u16);
                    c.bus.write32(
                        track + TRACK_HEADER,
                        if r_m2.next_u32() % 5 == 0 { 0 } else { header },
                    );
                    c.bus.write32(header, r_m2.next_u32());
                    let len = match r_m2.next_u32() % 4 {
                        0 => lengths[0],
                        1 => lengths[1],
                        _ => r_m2.next_u32() % (100 + 10 * seed),
                    };
                    c.bus.write32(track + TRACK_LENGTH, len);
                }
                c.pc = 0x003d_4b00;
            },
        );
    }

    /// PATH_SetTrackControlValue: player on/off, a map already selected or
    /// not, values below and above 0x80, random masks (sometimes all ones)
    /// against four maps with random ids and loaded flags, each with 24
    /// slots that are empty or point at a random track; the real
    /// `PATH_SelectMap`.
    #[test]
    fn path_set_track_control_value_matches() {
        use super::path_ctl::*;
        check_lohi(
            ee::s14::f_003d0d70__generated,
            path_set_track_control_value,
            300,
            &|c, seed| {
                let mut r_m2 = Rng(0x003d_0d70 ^ (seed as u64) << 8);
                scramble(c, &mut r_m2);
                c.pc = 0x003d_0d70;
                let enabled = if seed % 8 == 0 {
                    0
                } else {
                    1 + r_m2.next_u32() % 255
                };
                c.bus.write8(ENABLED, enabled as u8);
                let id = if seed % 8 == 1 {
                    r_m2.next_u32() | 1
                } else {
                    0
                };
                c.bus.write32(CURRENT_ID, id);
                c.bus.write32(CURRENT_MAP, r_m2.next_u32());
                let mask = if seed % 3 == 0 {
                    u64::MAX
                } else {
                    (r_m2.next_u32() as u64) << 32 | r_m2.next_u32() as u64
                };
                c.g[A0][0] = mask;
                c.g[A1][0] = if seed % 5 == 0 {
                    (r_m2.next_u32() as u64) << 32 | r_m2.next_u32() as u64
                } else {
                    (r_m2.next_u32() % 0x80) as u64
                };
                for m in 0..4u32 {
                    let map = MAP0_ID - 0x30 + m * MAP_SIZE as u32;
                    c.bus.write32(map + 0x30, r_m2.next_u32());
                    c.bus
                        .write32(map + 0x34, if r_m2.next_u32() % 3 == 0 { 0 } else { 1 });
                    for j in 0..24u32 {
                        let track = DATA + (m * 24 + j) * 0x10;
                        fill(c, &mut r_m2, track, 4);
                        let p_m2 = if r_m2.next_u32() % 4 == 0 { 0 } else { track };
                        c.bus.write32(map + SLOT0 + 4 * j, p_m2);
                    }
                }
            },
        );
    }

    pub(crate) use crate::game::lib::snd::tests::STREAM_FAKE_CONTROL;
    pub(crate) use crate::game::lib::snd::tests::{StreamFakeSlots, FAKE_CTRL, FAKE_MAGIC};

    // --------------------------------- SpeechSystem_ApplyEventContext

    /// The ten-entry table: entry 0 is another id, entry 1 another key,
    /// entry 2 unused, entry 3 the match (with or without an object).
    pub(crate) fn speech_event_setup(c: &mut Cpu, seed: u32) {
        let mut r = Rng(0x002b_1520 ^ (seed as u64) << 8);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        c.hooks.miss = crate::difftest::fakes::fake_call;
        c.hooks.jump_out = crate::difftest::fakes::fake_jump_out;
        c.hooks.syscall = crate::difftest::fakes::fake_syscall;
        // Unwind at the first voice call for one seed in four (those carry an
        // object on the matching entry, see below), so `covered` sees a
        // requested unwind inside the matched path.
        let unwind = if seed % 4 == 1 { 0 } else { u32::MAX };
        c.bus.write32(crate::difftest::fakes::LOG_UNWIND_AT, unwind);

        let this = DATA + 0x100;
        let id = (seed % 3) as u32;
        let key = ((seed / 3) % 2) as u32;
        let no_match = seed % 5 == 4;
        c.g[A0][0] = this as u64;
        c.g[A1][0] = id as u64;
        c.g[A2][0] = key as u64;
        c.pc = 0x002b_1520;

        // self->slots: an array of four slot records.
        c.bus.write32(this + SPEECH_EVENT_SLOTS, DATA + 0x40);
        for i in 0..4u32 {
            c.bus
                .write32(DATA + 0x40 + 4 * i, DATA + 0x2000 + 0x1000 * i);
        }
        // The ten entries: the body indexes entry `j` as `self + (j << 5)` and
        // then uses the absolute offsets above, so write each entry's field
        // block there (`ents[0]` is `self + 0x18`, the `used` word of j = 0).
        for j in 0..10u32 {
            let e = this + j * SPEECH_EVENT_ENT_SIZE;
            fill(c, &mut r, e + SPEECH_EVENT_ENT_USED, 8);
            let (used, e_id, e_key) = match j {
                0 => (1u32, id ^ 1, key),
                1 => (1, id, key ^ 1),
                2 => (0, id, key),
                3 => (1, id, if no_match { key ^ 1 } else { key }),
                _ => (0, r.next_u32() % 3, r.next_u32() % 2),
            };
            c.bus.write32(e + SPEECH_EVENT_ENT_USED, used);
            c.bus.write32(e + SPEECH_EVENT_ENT_ID, e_id);
            c.bus.write32(e + SPEECH_EVENT_ENT_KEY, e_key);
            c.bus.write32(e + SPEECH_EVENT_ENT_W, r.next_u32());
            c.bus.write32(e + SPEECH_EVENT_ENT_T, r.edge_f32());
            c.bus.write32(
                e + SPEECH_EVENT_ENT_K,
                if seed % 3 == 0 { 0 } else { 1 + seed % 7 },
            );
            c.bus.write32(e + SPEECH_EVENT_ENT_OBJ, 0);
        }
        // The matching entry's object, with a fake voice vtable.
        if !no_match && seed % 2 == 1 {
            let e = this + 3 * SPEECH_EVENT_ENT_SIZE;
            let obj = DATA + 0x4000;
            let vt = vtable(c, &mut r, &[SPEECH_EVENT_V05, SPEECH_EVENT_V08]);
            c.bus.write32(obj + SPEECH_EVENT_OBJ_VOICE, vt);
            c.bus.write32(obj + SPEECH_EVENT_OBJ_Z, r.next_u32());
            c.bus.write32(e + SPEECH_EVENT_ENT_OBJ, obj);
        }
    }

    /// `SpeechSystem_ApplyEventContext` over a ten-entry table: used/id/key
    /// mismatches, the end of the table, and a match whose object is absent
    /// or has a voice whose `v05`/`v08` are logging fakes.
    #[test]
    fn speech_system_apply_event_context_matches() {
        covered(
            ee::s08::f_002b1520__generated,
            speech_system_apply_event_context,
            200,
            &speech_event_setup,
        );
    }

    /// Speech_GetLanguageFromSuffix: names ending in `_eng`, `_frn`, `_grm`,
    /// `_spn`, an unknown suffix, and no `_` suffix at all (with and without
    /// a name prefix).
    #[test]
    fn speech_get_language_from_suffix_matches() {
        check(
            ee::s08::f_002b03b8__generated,
            speech_get_language_from_suffix,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2b_03b8, 0);
                let name = DATA + 0x100;
                c.g[A0][0] = sx(name);
                let suffix: &[u8] = match seed % 6 {
                    0 => &b"eng"[..],
                    1 => &b"frn"[..],
                    2 => &b"grm"[..],
                    3 => &b"spn"[..],
                    4 => &b"xyz"[..],
                    _ => &b""[..],
                };
                let mut p = name;
                for _ in 0..(seed / 6) % 5 {
                    c.bus.write8(p, b'a' + (r.next_u32() % 26) as u8);
                    p += 1;
                }
                if seed % 2 == 0 {
                    c.bus.write8(p, b'_');
                    p += 1;
                }
                for (i, b) in suffix.iter().enumerate() {
                    c.bus.write8(p + i as u32, *b);
                }
                c.bus.write8(p + suffix.len() as u32, 0);
            },
        );
    }

    /// BankMonitor_TimerSet: a timer record with random fields; the real
    /// kill/set-period/enable leaves store the period, the 8-byte key and
    /// the five payload words.
    #[test]
    fn bankmonitor_timer_set_matches() {
        check(
            ee::s08::f_002ad888__generated,
            bankmonitor_timer_set,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_d888, 0);
                let timer = DATA + 0x100;
                for i in 0..0x30u32 {
                    c.bus.write32(timer + 4 * i, r.next_u32());
                }
                c.g[A0][0] = sx(timer);
                c.g[A1][0] = sx(r.next_u32() % 5000);
                c.g[A2][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
                c.g[A3][0] = sx(r.next_u32());
                c.g[T0][0] = sx(r.next_u32());
                c.g[T1][0] = sx(r.next_u32());
                c.g[T2][0] = sx(r.next_u32());
                c.g[T3][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn speech_system_stream_wait_callback_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA, GAME_GP};
        let _slots = StreamFakeSlots::install();
        check(
            ssx3_ee::s08::f_002b1348__generated,
            speech_system_stream_wait_callback,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x002b_1348, 12);
                // This machine's call to the faked callees (see snd.rs).
                c.bus.write32(FAKE_CTRL, FAKE_MAGIC);
                let (this, array, table) = (DATA + 0x1000, DATA + 0x1100, DATA + 0x1200);
                let (msg, ent) = (DATA + 0x2000, DATA + 0x2100);
                // Control values for the faked callees: the stream state, the
                // playing flag, the remaining time and the instance slot.
                c.bus.write32(
                    STREAM_FAKE_CONTROL,
                    match seed % 3 {
                        0 => 0,
                        1 => 1,
                        _ => 2 + r.next_u32() % 4,
                    },
                );
                c.bus.write32(STREAM_FAKE_CONTROL + 4, (seed % 2) as u32);
                c.bus.write32(STREAM_FAKE_CONTROL + 8, r.f32(120.0));
                c.bus.write32(STREAM_FAKE_CONTROL + 12, 0);
                // self -> array; the array's buffer table is empty, so the
                // current-request lookup returns -1 at once.
                c.bus.write32(this, array);
                c.bus.write32(array + 8, table);
                for i in 0..8u32 {
                    c.bus.write32(table + 4 * i, 0);
                }
                // The message and its request entry.
                c.bus.write32(msg + STREAM_MSG_ENTRY, ent);
                c.bus.write32(
                    msg + STREAM_MSG_REQUEST,
                    if seed % 4 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() & 0x7fff
                    },
                );
                c.bus.write32(msg + STREAM_MSG_TIMER, u32::MAX);
                c.bus.write32(ent + STREAM_ENTRY_SLOT, r.next_u32() % 4);
                c.bus.write32(ent + STREAM_ENTRY_INDEX, r.next_u32() % 8);
                // The SND manager chain `**(gp+0x6c4) + 0x1d8`.
                let (p1, p2) = (DATA + 0x3000, DATA + 0x3100);
                c.bus.write32(GAME_GP + STREAM_SND_GP, p1);
                c.bus.write32(p1, p2);
                c.bus.write32(p2, DATA + 0x3200);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = msg as u64;
            },
        );
    }

    /// `BankMonitor_FindFreeTimer`: timers free on a stride, `n` taking 1..3
    /// (found), 0 and 48, and 200 (the whole table, -1).
    #[test]
    fn bank_monitor_find_free_timer_matches() {
        check(
            ee::s08::f_002adc30__generated,
            bank_monitor_find_free_timer,
            200,
            &|c, seed| {
                let mut r = Rng(0x002a_dc30 ^ seed as u64);
                scramble(c, &mut r);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                let n = match seed % 4 {
                    0 => 0,
                    1 => 1 + (seed % 3),
                    2 => 200,
                    _ => 48,
                };
                c.g[A1][0] = sx(n);
                for i in 0..48u32 {
                    let t = this + 0x98 + i * 0x2c;
                    let free = (i + seed) % 5 == 0;
                    c.bus
                        .write32(t + 4, if free { 0xffff_ffff } else { (seed ^ i) | 1 });
                    c.bus.write32(t, r.next_u32());
                }
            },
        );
    }

    use crate::difftest::fakes::fake;

    use crate::game::sound::soundsys::tests::rs207d_world;

    pub(crate) use crate::game::sound::soundsys::tests::OC330_SYS;

    /// Gated slot replacements (the suite runs tests in parallel): a fake
    /// runs only while this magic is in the machine's RAM, else the generated
    /// body (the slot's default: the tests do not install the ports).
    pub(crate) const OC329_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC329_MAGIC: u32 = 0x0c32_9f01;

    // -- BankMonitor_TimerKill -------------------------------------------

    #[test]
    fn bank_monitor_timer_kill_matches() {
        check(
            ee::s08::f_002ad940__generated,
            bank_monitor_timer_kill,
            20,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_d940, 0);
                c.pc = 0x002a_d940;
                let m = DATA + 0x300;
                for i in 0..4u32 {
                    c.bus.write32(m + 4 * i, r.next_u32());
                }
                c.g[A0][0] = sx(m);
            },
        );
    }

    // -- SpeechSystem_OnSpchEvent ----------------------------------------

    static OC329_PREV_SPEECH_PLAY_EVENT: std::sync::atomic::AtomicUsize =
        std::sync::atomic::AtomicUsize::new(0);

    pub(crate) fn oc329_fake_speech_play_event(c: &mut Cpu) {
        if c.bus.read32(OC329_CTRL) != OC329_MAGIC || c.bus.read32(OC329_CTRL + 4) == 0 {
            return crate::difftest::fakes::fall_back(
                c,
                &OC329_PREV_SPEECH_PLAY_EVENT,
                ee::s08::f_002b04d8__generated,
            );
        }
        fake(c, 0x002b_04d8, c.g[RA][0] as u32);
    }

    #[test]
    fn speech_system_on_spch_event_matches() {
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s08::OVERRIDE_002B04D8),
            oc329_fake_speech_play_event,
            &OC329_PREV_SPEECH_PLAY_EVENT,
        );
        check(
            ee::s08::f_002b0f48__generated,
            speech_system_on_spch_event,
            160,
            &|c, seed| {
                let _r = machine(c, seed, 0x2b_0f48, 0);
                c.pc = 0x002b_0f48;
                c.bus.write32(OC329_CTRL, OC329_MAGIC);
                let sys = DATA + 0x200;
                let insts = DATA + 0x400;
                let ev = DATA + 0x800;
                // The real callee finds no bank (id 0x1234 in an empty bank
                // table) and answers -1.
                let use_fake = (seed % 3 != 0) as u32;
                c.bus.write32(OC329_CTRL + 4, use_fake);
                c.bus.write32(
                    crate::difftest::fakes::LOG_RESULTS,
                    if use_fake == 1 { 5 } else { 0 },
                );
                c.bus.write32(crate::difftest::fakes::LOG_RESULTS + 4, 7);
                // Instance 0 and 1: present or absent.
                for i in 0..2u32 {
                    let inst = insts + 0x40 * i;
                    let present = (seed >> i) & 1 != 0;
                    c.bus
                        .write32(sys + 4 + 4 * i, if present { inst } else { 0 });
                    c.bus.write32(inst + 0xf0, 0); // no banks
                    c.bus.write32(inst + 0xf4, DATA + 0x1000);
                }
                // The active instance index and the callback (a leaf that
                // returns 0).
                c.bus.write32(sys + 0xc, 0x0014_06c0);
                c.bus.write32(sys + 0x10, (seed % 3) as u32);
                c.bus.write32(ev, 0x1234);
                c.g[A0][0] = sx(sys);
                c.g[A1][0] = sx(ev);
            },
        );
    }

    // -- 0x2b0e28 SpeechSystem_IsInstanceReady ------------------------------

    /// The speech system of the 2077 world (its stream array), slot 0 with a
    /// stream (ready or not) and the empty slots 1..3.
    #[test]
    fn speech_system_is_instance_ready_matches() {
        check(
            ee::s08::f_002b0e28__generated,
            speech_system_is_instance_ready,
            60,
            &|c, seed| {
                rs207d_world(c, seed);
                c.pc = 0x002b_0e28;
                c.g[A0][0] = (OC330_SYS + 0x5560) as u64;
                c.g[A1][0] = (seed % 4) as u64;
            },
        );
    }

    use std::sync::atomic::AtomicUsize;

    pub(crate) use crate::game::lib::snd::tests::oc337_arm;
    pub(crate) use crate::game::lib::snd::tests::OC337_CTRL;

    pub(crate) const OC337_MAGIC_SPEECH: u32 = 0x0c33_7a01;

    // -- SpeechSystem_SpchEventCallback -------------------------------------

    pub(crate) static OC337_PREV_ON_SPCH_EVENT: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn oc337_fake_on_spch_event(c: &mut Cpu) {
        if c.bus.read32(OC337_CTRL) != OC337_MAGIC_SPEECH {
            return crate::difftest::fakes::fall_back(
                c,
                &OC337_PREV_ON_SPCH_EVENT,
                ee::s08::f_002b0f48__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fake(c, 0x002b_0f48, ra);
    }

    /// Blocked (-1) or passed to a fake `SpeechSystem_OnSpchEvent`
    /// (sometimes unwinding).
    #[test]
    fn speech_system_spch_event_callback_matches() {
        crate::difftest::fakes::hook(
            std::ptr::addr_of_mut!(ee::s08::OVERRIDE_002B0F48),
            oc337_fake_on_spch_event,
            &OC337_PREV_ON_SPCH_EVENT,
        );
        covered(
            ee::s08::f_002af8a8__generated,
            speech_system_spch_event_callback,
            90,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_f8a8, 1);
                c.pc = 0x002a_f8a8;
                oc337_arm(c, OC337_MAGIC_SPEECH);
                let blocked = if seed % 3 == 0 { r.next_u32() | 1 } else { 0 };
                c.bus.write32(GAME_GP + SPCH_EVENT_CB_BLOCKED, blocked);
                c.bus.write32(GAME_GP + SPCH_EVENT_CB_SYSTEM, DATA + 0x200);
                c.g[A0][0] = sx(DATA + 0x400 + 0x10 * (seed % 16));
            },
        );
    }

    /// A speech stream at `DATA + 0x100` whose handle (0 or -1) has a
    /// request or not in `snd_world` (fake lock/unlock hooks).
    pub(crate) fn oc339_speech_stream(c: &mut Cpu, r: &mut Rng, seed: u32) -> u32 {
        use crate::game::lib::snd::tests::snd_world;
        let found = seed % 2 == 0;
        snd_world(c, r, found);
        let stream = DATA + 0x100;
        c.bus.write32(
            stream + SPEECH_STREAM_HANDLE,
            if found { 0 } else { 0xffff_ffff },
        );
        stream
    }

    /// `SpeechSystem_ClearPrebuffer`: `*self` is random, `a1` the stream the
    /// callback reads.
    #[test]
    fn speech_system_clear_prebuffer_matches() {
        covered(
            ee::s08::f_002b1248__generated,
            speech_system_clear_prebuffer,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2b_1248, 2);
                c.pc = 0x002b_1248;
                let stream = oc339_speech_stream(c, &mut r, seed);
                c.bus.write32(DATA + 0x200, r.next_u32());
                c.g[A0][0] = (DATA + 0x200) as u64;
                c.g[A1][0] = stream as u64;
            },
        );
    }

    use crate::difftest::elf_image;

    pub(crate) use crate::game::lib::snd::tests::oc375_machine;
    pub(crate) use crate::game::lib::snd::tests::oc375_script;

    /// Gate of PATH_TrackSubmitQueue's fakes (off: the slot's previous body).
    pub(crate) const PTS_CTRL: u32 = DATA + 0x7_0000;

    pub(crate) const PTS_MAGIC: u32 = 0x3d41_a8c5;

    pub(crate) static PTS_PREV_QUEUED: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static PTS_PREV_VOLUME: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static PTS_PREV_QUEUE: AtomicUsize = AtomicUsize::new(0);

    /// A logged fake returning the sign-extended word at `result`.
    pub(crate) fn pts_fake_with(c: &mut Cpu, target: u32, result: u32) {
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, target);
        if c.pc == ra {
            c.g[V0][0] = sx(c.bus.read32(result));
        }
    }

    /// `PATH_PlatGetQueuedEntryValue(track, i)`: the value at `PTS_CTRL +
    /// 0x10 + 4 * (i & 3)`.
    pub(crate) fn pts_fake_queued(c: &mut Cpu) {
        if c.bus.read32(PTS_CTRL) != PTS_MAGIC {
            return fakes::fall_back(c, &PTS_PREV_QUEUED, ee::s08::f_002aeaa8__generated);
        }
        let i = c.g[A1][0] as u32 & 3;
        pts_fake_with(c, PTS_GET_QUEUED, PTS_CTRL + 0x10 + 4 * i);
    }

    /// `PATH_PlatGetTrackVolume(track)`: the word at `PTS_CTRL + 4`.
    pub(crate) fn pts_fake_volume(c: &mut Cpu) {
        if c.bus.read32(PTS_CTRL) != PTS_MAGIC {
            return fakes::fall_back(c, &PTS_PREV_VOLUME, ee::s08::f_002af428__generated);
        }
        pts_fake_with(c, PTS_GET_VOLUME, PTS_CTRL + 4);
    }

    /// `PATH_PlatQueue(...)`: the handle at `PTS_CTRL + 8`.
    pub(crate) fn pts_fake_queue(c: &mut Cpu) {
        if c.bus.read32(PTS_CTRL) != PTS_MAGIC {
            return fakes::fall_back(c, &PTS_PREV_QUEUE, ee::s08::f_002ae7a8__generated);
        }
        pts_fake_with(c, PTS_PLAT_QUEUE, PTS_CTRL + 8);
    }

    #[allow(unused_unsafe)]
    pub(crate) fn pts_install_fakes() {
        use std::ptr::addr_of_mut;
        unsafe {
            fakes::hook(
                addr_of_mut!(ee::s08::OVERRIDE_002AEAA8),
                pts_fake_queued,
                &PTS_PREV_QUEUED,
            );
            fakes::hook(
                addr_of_mut!(ee::s08::OVERRIDE_002AF428),
                pts_fake_volume,
                &PTS_PREV_VOLUME,
            );
            fakes::hook(
                addr_of_mut!(ee::s08::OVERRIDE_002AE7A8),
                pts_fake_queue,
                &PTS_PREV_QUEUE,
            );
        }
    }

    /// PATH_TrackSubmitQueue: no node (reset); 0-3 queued entries with
    /// negative and positive values; one-shot (idle time set or kept) and
    /// looping tracks (repeat count positive, or recomputed by the real
    /// `PATH_CalcNodeRepeatCount`); volume positive or negative (voice
    /// volume); the node index in and out of range and samples 0/-1/n; no
    /// master, the track itself as master, or another master with every
    /// sync mode, beat/bar nibbles zero or not, a pending resync early or
    /// due, the real `PATH_TrackGetBeatPos` (its voice idle), the last
    /// synced queue recent or old; queue handles negative or not and the
    /// repeat/history counters wrapping. The voice calls are gated logged
    /// fakes; one may unwind.
    #[test]
    fn path_track_submit_queue_matches() {
        const TRACK: u32 = DATA + 0x1000;
        const MASTER: u32 = DATA + 0x2000;
        const SYS: u32 = DATA + 0x3000;
        const NODES: u32 = DATA + 0x4000;
        const HIST: u32 = DATA + 0x5000;
        const IDX: u32 = DATA + 0x8000;
        const SAMPLES: u32 = DATA + 0xc000;
        /// `PATH_Plat` voice table (`0x4d3e98`): empty, so the beat
        /// position's status query returns at once.
        const VOICES: u32 = 0x004d_3e98;
        pts_install_fakes();
        covered(
            ee::s14::f_003d41a8__generated,
            path_track_submit_queue,
            300,
            &|c, seed| {
                elf_image(c);
                let mut r = machine(c, seed, 0x003d_41a8, 6);
                c.pc = 0x003d_41a8;
                c.bus.write32(PTS_CTRL, PTS_MAGIC);
                for i in 0..4 {
                    let v = match r.next_u32() % 3 {
                        0 => (-8i32) as u32,
                        _ => r.next_u32() % 100,
                    };
                    c.bus.write32(PTS_CTRL + 0x10 + 4 * i, v);
                }
                c.bus.write32(PTS_CTRL + 4, r.next_u32() % 128);
                let handle = if r.next_u32() % 4 == 0 {
                    u32::MAX
                } else {
                    r.next_u32() % 64
                };
                c.bus.write32(PTS_CTRL + 8, handle);

                let now = 100 + r.next_u32() % 1000;
                c.bus.write32(PTS_TIME, now);
                c.bus
                    .write32(PTS_LAST_SYNC, now.wrapping_sub(r.next_u32() % 700));
                c.bus.write32(PTS_PATH_SYS, SYS);
                for i in 0..4 {
                    c.bus.write32(VOICES + 4 * i, 0);
                }

                // The path system: master index -1..3 over {0, track, master, master}.
                let master = (r.next_u32() % 5) as i32 - 1;
                c.bus.write32(SYS, master as u32);
                c.bus.write32(SYS + 0x58, 0);
                c.bus.write32(SYS + 0x5c, TRACK);
                c.bus.write32(SYS + 0x60, MASTER);
                c.bus.write32(SYS + 0x64, MASTER);
                c.bus.write32(SYS + 0x34, NODES);
                c.bus.write32(SYS + 0x38, IDX);
                c.bus.write32(SYS + 0x50, SAMPLES);
                // Node indices: eight 16-byte nodes.
                for i in 0..0x1000u32 {
                    c.bus.write16(IDX + 2 * i, ((r.next_u32() % 8) * 4) as u16);
                }
                for k in 0..8u32 {
                    let n = NODES + 16 * k;
                    let sample = match r.next_u32() % 6 {
                        0 => 0,
                        1 => 0xffff,
                        _ => 1 + r.next_u32() % 8,
                    };
                    c.bus.write32(n, (r.next_u32() % 10) << 16 | sample);
                    let mut beats = r.next_u32();
                    if r.next_u32() % 3 == 0 {
                        beats &= 0xff0f_ffff;
                    }
                    if r.next_u32() % 3 == 0 {
                        beats &= 0xf0ff_ffff;
                    }
                    c.bus.write32(n + 4, beats);
                    c.bus.write32(n + 8, r.next_u32());
                    c.bus.write32(n + 0xc, r.next_u32());
                }
                for i in 0..0x18u32 {
                    let v = if r.next_u32() % 5 == 0 {
                        r.next_u32()
                    } else {
                        r.next_u32() % 2000
                    };
                    c.bus.write32(SAMPLES - 0x10 + 4 * i, v);
                }

                // The track.
                for i in 0..0x44u32 {
                    c.bus.write32(TRACK + 4 * i, r.next_u32());
                }
                let node = if seed % 8 == 0 {
                    0xffff
                } else {
                    r.next_u32() % 8
                };
                c.bus.write16(TRACK + PTS_NODE, node as u16);
                let flags = r.next_u32() & !(0x1f << 14) | (r.next_u32() % 4) << 14;
                c.bus.write32(TRACK + PTS_FLAGS, flags);
                let looping = if r.next_u32() % 3 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 255
                };
                c.bus.write8(TRACK + PTS_LOOPING, looping as u8);
                if r.next_u32() % 2 == 0 {
                    c.bus.write32(TRACK + PTS_IDLE_SINCE, 0);
                }
                c.bus.write32(TRACK + PTS_PLAT_TRACK, r.next_u32() % 4);
                c.bus.write32(TRACK + PTS_HISTORY, HIST);
                let resync = match r.next_u32() % 3 {
                    0 => 0,
                    1 => now + 5 + r.next_u32() % 100,
                    _ => r.next_u32() % (now + 5),
                };
                c.bus.write32(TRACK + PTS_RESYNC, resync);

                // The master track.
                for i in 0..0x44u32 {
                    c.bus.write32(MASTER + 4 * i, r.next_u32());
                }
                c.bus.write16(MASTER + PTS_NODE, (r.next_u32() % 8) as u16);
                c.bus.write32(MASTER + PTS_PLAT_TRACK, r.next_u32() % 4);
                if r.next_u32() % 4 == 0 {
                    c.bus.write32(MASTER + PTS_NEXT_NODE, 0);
                }

                c.g[A0][0] = sx(TRACK);
            },
        );
    }

    /// PATH_TrackQueueNode: a chain of 1-6 nodes (types 0, -1, -2, -3, -4
    /// ending in a playable one), each with one catch-all branch to the
    /// next; type-0 nodes with and without an event (flag bit 7 set or
    /// not) and the time flag, a node callback (fake) or none, repeat
    /// nodes on the voice's current node or another; real RemapNode
    /// (negative track, or a node with no remap table), SelectBranch,
    /// QueueAppend, copyevent, memset and AddEventToMaps (no maps).
    #[test]
    fn path_track_queue_node_matches() {
        const STATE: u32 = DATA + 0x2000;
        const EVENTS: u32 = DATA + 0x4000;
        const NODES: u32 = DATA + 0x5000;
        const INDEX: u32 = DATA + 0x6000;
        const BRANCHES: u32 = DATA + 0x6800;
        const VOICES: u32 = DATA + 0x7000;
        check(
            ee::s14::f_003d3900__generated,
            path_track_queue_node,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_3900, 3);
                c.pc = 0x003d_3900;
                c.bus.write32(PATH_STATE_PTR, STATE);
                for off in (0..0xa00).step_by(4) {
                    c.bus.write32(STATE + off, 0);
                }
                c.bus.write32(STATE + PATH_NODES, NODES);
                c.bus.write32(STATE + PATH_NODE_INDEX, INDEX);
                c.bus.write32(STATE + PATH_BRANCH_INDEX, BRANCHES);
                c.bus.write32(STATE + PATH_EVENT_TABLE, EVENTS);
                for v in 0..4 {
                    let voice = VOICES + 0x40 * v;
                    c.bus.write32(STATE + PATH_VOICES + 4 * v, voice);
                    c.bus.write32(voice + PATH_VOICE_FLAGS, r.next_u32());
                    c.bus
                        .write16(voice + PATH_VOICE_NODE, (r.next_u32() % 4) as u16);
                }
                for i in 0..32 {
                    c.bus.write32(EVENTS + 4 * i, r.next_u32());
                }
                for i in 0..8 {
                    c.bus.write16(BRANCHES + 2 * i, (r.next_u32() % 8) as u16);
                }
                c.bus.write32(PATH_TRACK_TIME, r.next_u32() % 100_000);
                let callback = if seed % 3 == 0 { 0 } else { FAKE };
                c.bus.write32(PATH_NODE_CALLBACK, callback);
                c.bus.write32(PATH_NODE_CALLBACK_ARG, r.next_u32());
                let last = seed % 6;
                for k in 0..8u32 {
                    c.bus.write16(INDEX + 2 * k, (k * 8) as u16);
                    let node = NODES + 0x20 * k;
                    let ty: u16 = if k >= last {
                        1 + (r.next_u32() % 5) as u16
                    } else {
                        match r.next_u32() % 5 {
                            0 | 1 => 0,
                            2 => 0xffff,
                            3 => 0xfffe,
                            _ => {
                                if r.next_u32() % 2 == 0 {
                                    0xfffd
                                } else {
                                    0xfffc
                                }
                            }
                        }
                    };
                    c.bus.write16(node, ty);
                    let h2 = (r.next_u32() as u16 & 0xffe0) | (r.next_u32() % 4) as u16;
                    c.bus.write16(node + 2, h2);
                    let timed = if r.next_u32() % 2 == 0 {
                        0x20000
                    } else {
                        (r.next_u32() % 8) << 17
                    };
                    // One branch (count in bits 12..16); remap byte 0.
                    c.bus
                        .write32(node + 4, timed | 1 << 12 | (r.next_u32() & 0xfff0_0f00));
                    let event = if r.next_u32() % 2 == 0 {
                        0
                    } else {
                        r.next_u32() % 32
                    };
                    c.bus
                        .write32(node + 8, (r.next_u32() & !0x7c00) | event << 10);
                    c.bus.write32(node + 0xc, (r.next_u32() % 8) << 2);
                    // Catch-all branch to the next node.
                    c.bus.write8(node + 0x10, 0x80);
                    c.bus.write8(node + 0x11, 0x7f);
                    c.bus.write16(node + 0x12, (k + 1) as u16);
                }
                c.g[A0][0] = if seed % 2 == 0 {
                    u64::MAX
                } else {
                    (r.next_u32() % 4) as u64
                };
                c.g[A1][0] = 0;
                c.g[A2][0] = (r.next_u32() % 128) as u64;
            },
        );
    }

    /// PATH_TrackAdvance: an aligned or unaligned voice whose current node
    /// is none (-1) or 0-3; repeat counts off, below or within the node's
    /// rows * columns; SelectBranch to a node or to none (no branches);
    /// the target owned by another voice (PostNodeEvents) or, for "none",
    /// by the voice itself / nobody / another voice; real callees (the
    /// queue-node walk over playable nodes, SubmitQueue on its short path).
    /// Not reached: a failed submit (needs SubmitQueue's long path).
    #[test]
    fn path_track_advance_matches() {
        const STATE: u32 = DATA + 0x2000;
        const NODES: u32 = DATA + 0x5000;
        const INDEX: u32 = DATA + 0x6000;
        const VOICES: u32 = DATA + 0x7000;
        check(
            ee::s14::f_003d3d20__generated,
            path_track_advance,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_3d20, 0);
                c.pc = 0x003d_3d20;
                c.bus.write32(PATH_STATE_PTR, STATE);
                for off in (0..0xa00).step_by(4) {
                    c.bus.write32(STATE + off, 0);
                }
                c.bus.write32(STATE, u32::MAX); // no current voice
                c.bus.write32(STATE + PATH_NODES, NODES);
                c.bus.write32(STATE + PATH_NODE_INDEX, INDEX);
                c.bus.write32(STATE + PATH_EVENT_WRITE, r.next_u32());
                let voice = VOICES + 4 * (seed / 2 % 2);
                let voices = [voice, VOICES + 0x200, VOICES + 0x400, 0];
                for (i, &v) in voices.iter().enumerate() {
                    c.bus.write32(STATE + PATH_VOICES + 4 * i as u32, v);
                }
                for off in (0..0x600).step_by(4) {
                    c.bus.write32(VOICES + off, r.next_u32());
                }
                let cur: u16 = if seed % 5 == 0 {
                    0xffff
                } else {
                    (r.next_u32() % 4) as u16
                };
                c.bus.write16(voice + PATH_VOICE_CUR_NODE, cur);
                let n = [0u32, 0x1f, 1, 2, 4, 9][(seed / 3 % 6) as usize];
                let flags = c.bus.read32(voice + PATH_VOICE_FLAGS) & !0x1f0 | n << 4;
                c.bus.write32(voice + PATH_VOICE_FLAGS, flags);
                let chord = if r.next_u32() % 2 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 255
                };
                c.bus.write8(voice + PATH_VOICE_CHORD, chord as u8);
                c.bus
                    .write8(voice + PATH_VOICE_VEL, (r.next_u32() % 128) as u8);
                for k in 0..9u32 {
                    c.bus.write16(INDEX + 2 * k, (k * 8) as u16);
                    let node = NODES + 0x20 * k;
                    c.bus.write16(node, 1 + (r.next_u32() % 4) as u16);
                    let slot = if k < 8 {
                        1 + r.next_u32() % 2
                    } else {
                        [0, 1, 3][(seed % 3) as usize]
                    };
                    c.bus
                        .write16(node + 2, (r.next_u32() as u16 & 0xffe0) | slot as u16);
                    let branches = if r.next_u32() % 4 == 0 { 0 } else { 1 };
                    c.bus.write32(
                        node + 4,
                        (r.next_u32() & 0x0ff0_0000)
                            | branches << 12
                            | (r.next_u32() & 0xf000_0f00),
                    );
                    c.bus.write32(node + 8, r.next_u32());
                    c.bus.write32(node + 0xc, r.next_u32());
                    c.bus.write8(node + 0x10, 0x80);
                    c.bus.write8(node + 0x11, 0x7f);
                    c.bus.write16(node + 0x12, (r.next_u32() % 8) as u16);
                }
                // node -1 is node 8
                c.bus.write16(INDEX - 2, 8 * 8);
                c.g[A0][0] = 0;
            },
        );
    }

    /// SpeechSystem_StreamStartCallback: the instance's current request
    /// is another one (retry timer) or this one; with no stream slot or a
    /// slot whose volume method returns a scale (a fake) or 0 (a real leaf
    /// returning 0).
    #[test]
    fn speech_system_stream_start_callback_matches() {
        covered(
            ee::s08::f_002b1268__generated,
            speech_system_stream_start_callback,
            200,
            &|c, seed| {
                let mut r = oc375_machine(c, seed, 0x002b_1268, 5);
                c.pc = 0x002b_1268;
                let sys = DATA + 0x100;
                let msg = DATA + 0x200;
                let entry = DATA + 0x400;
                let vt = DATA + 0x600;
                let p = DATA + 0x700;
                let q = DATA + 0x740;
                fill(c, &mut r, sys, 4);
                fill(c, &mut r, entry, 4);
                c.bus.write32(entry + 0x8d0, r.next_u32());
                c.bus.write32(sys + 0x1bc, vt);
                c.bus.write16(vt + 0x18, (r.next_u32() % 0x40) as u16 & !3);
                c.bus.write32(
                    vt + 0x1c,
                    if seed % 3 == 0 {
                        0x0014_06c0
                    } else {
                        fakes::FAKE
                    },
                );
                let request = r.next_u32();
                c.bus.write32(msg + STREAM_START_ENTRY, entry);
                c.bus.write32(msg + STREAM_START_REQUEST, request);
                c.bus.write32(
                    msg + STREAM_START_SLOT,
                    if seed % 4 == 0 {
                        u32::MAX
                    } else {
                        r.next_u32() % 4
                    },
                );
                c.bus.write32(GAME_GP + 0x6c4, p);
                c.bus.write32(p, q);
                c.bus.write32(q, DATA + 0x800);
                c.bus.write32(0x004a_37e0, r.next_u32());
                c.bus.write32(0x004a_37e4, r.next_u32());
                oc375_script(
                    c,
                    0x002a_b150,
                    if seed % 5 == 0 { request ^ 1 } else { request },
                );
                c.g[A0][0] = sx(sys);
                c.g[A1][0] = sx(msg);
            },
        );
    }
}
