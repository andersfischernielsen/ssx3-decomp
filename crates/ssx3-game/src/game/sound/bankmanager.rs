//! `src/sound/bankmanager.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::animation::rideranimbase::sw as sw_3;
pub(crate) use crate::game::be::beintplayer::lh;
pub(crate) use crate::game::be::bestate2::ea;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_2;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::gpa;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lh_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at as lw_at_2;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::ofs;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::dirtysock::tags2::arg;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::debugfe::gp;
pub(crate) use crate::game::fe::debugfe::r32_2;
pub(crate) use crate::game::fe::feasyncfile::at as at_3;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_4;
pub(crate) use crate::game::fe::feasyncfile::gp_16 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::lw as lw_2;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw_4;
pub(crate) use crate::game::fe::feasyncfile::lw_9 as lw_5;
pub(crate) use crate::game::fe::feasyncfile::p as p_3;
pub(crate) use crate::game::fe::feasyncfile::p as p_5;
pub(crate) use crate::game::fe::feasyncfile::p as p_6;
pub(crate) use crate::game::fe::feasyncfile::p as p_8;
pub(crate) use crate::game::fe::feasyncfile::p as p_9;
pub(crate) use crate::game::fe::feasyncfile::p as p_10;
pub(crate) use crate::game::fe::feasyncfile::p_7 as p_11;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::feasyncfile::sw_7 as sw_4;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::fe::ovtemplatepausemenu::p as p_2;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::p as p_4;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2 as p;
pub(crate) use crate::game::scripter::ssxscriptengine::sw_at;
pub(crate) use crate::game::world::wscriptcache::DELETE_2 as DELETE;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw_3;
pub(crate) use crate::game::wscript::wscriptman::p as p_7;
pub(crate) use crate::game::wscript::wscriptman::sw as sw_2;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0028_b588, snd_bank_set_memory, "SndBank_SetMemory"),
    (0x0028_b650, snd_bank_free_memory, "SndBank_FreeMemory"),
    (0x0028_b6c8, snd_bank_link, "SndBank_Link"),
    (0x0028_b730, snd_bank_unlink_2, "SndBank_Unlink"),
    (
        0x0028_b788,
        snd_bank_load_from_memory,
        "SndBank_LoadFromMemory",
    ),
    (0x0028_b7e0, snd_bank_unload_memory, "SndBank_UnloadMemory"),
    (
        0x0028_b830,
        snd_bank_is_loaded_with_name,
        "SndBank_IsLoadedWithName",
    ),
    (0x0028_b878, snd_bank_release, "SndBank_Release"),
    (
        0x0028_b930,
        snd_bank_man_init_load_request,
        "SndBankMan_InitLoadRequest",
    ),
    (
        0x0028_bce8,
        snd_bank_man_unload_bank,
        "SndBankMan_UnloadBank",
    ),
    (
        0x0028_bd10,
        snd_bank_man_load_bank_async,
        "SndBankMan_LoadBankAsync",
    ),
    (
        0x0028_bda8,
        snd_bank_man_set_bank_memory,
        "SndBankMan_SetBankMemory",
    ),
    (
        0x0028_bde0,
        snd_bank_man_free_bank_memory,
        "SndBankMan_FreeBankMemory",
    ),
    (
        0x0028_be90,
        snd_bank_man_unload_memory_bank,
        "SndBankMan_UnloadMemoryBank",
    ),
    (
        0x0028_beb8,
        snd_bank_man_release_bank,
        "SndBankMan_ReleaseBank",
    ),
    (
        0x0028_bee0,
        snd_bank_man_get_bank_buffer,
        "SndBankMan_GetBankBuffer",
    ),
    (
        0x0028_bf10,
        snd_bank_man_get_bank_buffer_size,
        "SndBankMan_GetBankBufferSize",
    ),
    (
        0x0028_bf38,
        snd_bank_man_init_bank_request,
        "SndBankMan_InitBankRequest",
    ),
    (
        0x0028_cd48,
        ssx_audio_system_play_peak_transition_music,
        "SSXAudioSystem_PlayPeakTransitionMusic",
    ),
    (
        0x0028_d5a0,
        ssx_audio_system_update_music_tier,
        "SSXAudioSystem_UpdateMusicTier",
    ),
    (
        0x0028_d740,
        ssx_audio_system_confirm_music_trigger,
        "SSXAudioSystem_ConfirmMusicTrigger",
    ),
    (
        0x0028_d960,
        ssx_audio_system_is_game_music_enabled,
        "SSXAudioSystem_IsGameMusicEnabled",
    ),
    (
        0x0028_def0,
        ssx_audio_system_set_music_state,
        "SSXAudioSystem_SetMusicState",
    ),
    (
        0x0028_e888,
        ssx_audio_system_end_script_music,
        "SSXAudioSystem_EndScriptMusic",
    ),
    (
        0x0028_ef90,
        ssx_audio_system_restart_peak_music,
        "SSXAudioSystem_RestartPeakMusic",
    ),
    (
        0x0028_f200,
        ssx_audio_system_on_replay_begin,
        "SSXAudioSystem_OnReplayBegin",
    ),
    (
        0x0028_f2c0,
        ssx_audio_system_on_replay_end,
        "SSXAudioSystem_OnReplayEnd",
    ),
    (
        0x0028_f328,
        ssx_audio_system_on_music_post_event,
        "SSXAudioSystem_OnMusicPostEvent",
    ),
    (
        0x0028_f520,
        audio_stop_fe_music,
        "SSXAudioSystem_StopFEMusic",
    ),
    (
        0x0028_f558,
        ssx_audio_system_set_selected_peak,
        "SSXAudioSystem_SetSelectedPeak",
    ),
    (
        0x0028_f678,
        ssx_audio_system_on_event_select_exit,
        "SSXAudioSystem_OnEventSelectExit",
    ),
    (
        0x0028_f730,
        ssx_audio_system_unload_loading_screen_bank,
        "SSXAudioSystem_UnloadLoadingScreenBank",
    ),
    (
        0x0028_fc58,
        ssx_audio_system_play_char_select_music,
        "SSXAudioSystem_PlayCharSelectMusic",
    ),
    (
        0x002a_3f90,
        ssx_audio_system_is_track_group5,
        "SSXAudioSystem_IsTrackGroup5",
    ),
    (
        0x002a_4030,
        ssx_audio_system_is_track_id_group4,
        "SSXAudioSystem_IsTrackIdGroup4",
    ),
    (
        0x002a_40e0,
        ssx_audio_system_is_track_id_group0,
        "SSXAudioSystem_IsTrackIdGroup0",
    ),
    (
        0x002a_4158,
        ssx_audio_system_is_track_id_group1,
        "SSXAudioSystem_IsTrackIdGroup1",
    ),
    (
        0x002a_41c8,
        ssx_audio_system_is_track_id_group2,
        "SSXAudioSystem_IsTrackIdGroup2",
    ),
    (
        0x002a_4238,
        ssx_audio_system_is_track_id_group3,
        "SSXAudioSystem_IsTrackIdGroup3",
    ),
    (
        0x002a_42c8,
        ssx_audio_system_is_race_type6_to11,
        "SSXAudioSystem_IsRaceType6to11",
    ),
    (
        0x002b_4070,
        music_player_set_playlist_mask,
        "MusicPlayer_SetPlaylistMask",
    ),
    (
        0x002b_4708,
        music_player_clear_loops_active,
        "MusicPlayer_ClearLoopsActive",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_IS_GAME_MUSIC_ENABLED;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_LEVEL_TO_VOLUME;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_START_PRERACE_MUSIC;
    pub(crate) use crate::game::be::beintmission::callee::BANK_MONITOR_STOP_ALL;
    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::be::beintmission::callee::BE_NEW_PLAYER_GET_INBOX;
    pub(crate) use crate::game::be::beintmission::callee::BUDDY_MAN_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::be::beintmission::callee::CAMERA_TRIGGER_MAN_UPDATE_MASK1;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_RESET_STATE;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_SET_ALGORITHM_2 as CHASE_CAM_SET_ALGORITHM;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_SET_ALGORITHM_IMMEDIATE;
    pub(crate) use crate::game::be::beintmission::callee::CROWD_AUDIO_STOP_ALL;
    pub(crate) use crate::game::be::beintmission::callee::EA_MEMSET;
    pub(crate) use crate::game::be::beintmission::callee::FIND_MISSION;
    pub(crate) use crate::game::be::beintmission::callee::GET_RIDER_PREVIEW;
    pub(crate) use crate::game::be::beintmission::callee::LOBBY_MENU_REQUEST_JOIN_ROOM;
    pub(crate) use crate::game::be::beintmission::callee::MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::be::beintmission::callee::MUSIC_PLAYER_STOP_SONG;
    pub(crate) use crate::game::be::beintmission::callee::NET_FE_STATE_CTOR;
    pub(crate) use crate::game::be::beintmission::callee::POPUP_CONFIRM_CFG_RESET;
    pub(crate) use crate::game::be::beintmission::callee::RACE_PROGRESS_RESET_ALL;
    pub(crate) use crate::game::be::beintmission::callee::REPLAY_APPLY_CAMERA;
    pub(crate) use crate::game::be::beintmission::callee::REWARD_GALLERY_CHECK_REWARDS_READY;
    pub(crate) use crate::game::be::beintmission::callee::REWARD_GALLERY_POLL_SHAPE_LOAD;
    pub(crate) use crate::game::be::beintmission::callee::RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_DEACTIVATE_BINDING;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_STOP_AUX_LISTS;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_STOP_MAIN_LISTS;
    pub(crate) use crate::game::be::beintmission::callee::SLOGIN_DELETE;
    pub(crate) use crate::game::be::beintmission::callee::SND_BANK_SET_MEMORY;
    pub(crate) use crate::game::be::beintmission::callee::UI_STATE_UPDATE;
    pub(crate) use crate::game::be::beintmission::callee::WSCRIPT_MAN_EVENT_QUEUE_POP;
    pub(crate) use crate::game::bx::bxstring2::callee::PATH_GET_LENGTH;
    pub(crate) use crate::game::bx::bxstring2::callee::PATH_HAS_EVENT_IN_RANGE;
    pub(crate) use crate::game::bx::bxstring2::callee::SND_BANK_MAN_INIT_LOAD_REQUEST;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::AVALANCHE_GET_OBJECT_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_TRACK_INFO;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_MISSION_SET_COLLECTED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_SET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_COUNT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUY_POPUP_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::CLEAR_NEW_REWARDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::DYNAMIC_EMITTER_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_THREAD_YIELD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_TIMER_SERVICE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::FE_ASYNC_SET_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_MODULE_REQUEST_STATE_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_PROCESS_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GROW_LINES;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INIT_FROM_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INSTANCE_GET_NODE_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOAD_INDICATOR_RENDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_API_SEND;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_GET_ROOM_NAME;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_END_OPERATION;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_ON_OPERATION_DONE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MEM_REPORT_STATS_LOCKED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MSG_PRINTF;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::POPUP_CONFIRM_REBUILD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::QSORT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RESET_FOR_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_ADD_BOOST_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_APPLY_SCRIPT_TRACK;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GATHER_CAM_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_RECORD_BEST_JUMP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_UPDATE_SCRIPT_SOUNDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SECTION_SELECT_PEAK_LIGHTING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SERIALIZE_READ_U32;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SET_ITEM_YEAR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SNOW_BOX_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_GET_INSTANCE as SSX_AUDIO_GET_INSTANCE_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_IS_TRACK_GROUP5;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_RESUME_SFX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STREAM_FILE_QUEUE_RANGE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STRLEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_HASH;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_COPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_RAW;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TEX_MAN_INIT_TABLE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_CTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LISTBOX_SET_ITEM_TEXT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST_WRAPPER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_NODE_SENTINEL;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_FIND_BY_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_CTOR_2_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_MGR_PUSH_POPUP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_ASCII;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_EVENT_STAT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_CPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::VRAM_HEAP_INIT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::WORLD_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_INSTANCE_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOLT_FILE_INDEX_MANAGER_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_COUNT_MESSAGES_FROM_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_FIND_ROSTER_ENTRY;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_IS_BLOCKED_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_IS_BUDDY_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_IS_ONLINE_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_IS_SAME_PRODUCT_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_IS_TEMP_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE_2 as BUILTIN_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAMERA_TRIGGER_STACK_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::CROWD_INSTANCE_PLAY_FALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::CROWD_INSTANCE_PLAY_TRICK;
    pub(crate) use crate::game::dirtysock::tags2::callee::CROWD_INSTANCE_STOP_ANTICIPATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::CSETUP_MAN_SEND_PACKET;
    pub(crate) use crate::game::dirtysock::tags2::callee::CVOIP_MAN_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::CW_SCRIPT_CACHE_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SETUP_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SHARED_LOGIN_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_VOIP_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::EATIMER_REMOVE_CALLBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::EVENT_SELECT_ON_WIDGET_NOTIFY;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_CAMERA_SET_CONTROLLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_STATE_GOTO_NEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_INSTANCE_START;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_CHANNEL_SET_LISTENER;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_INSTANCE_PAUSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_INSTANCE_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOAD_INDICATOR_CLEAR_BUSY;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_LIST_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_LIST_VISIBLE_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_FORWARD_EVENT_TO_UI;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_SEND_LOCK_CHAL;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MENU_ON_LOBBY_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LUNO_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_REQUEST_CARD_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_CLOSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_FLUSH;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_READ;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_WRITE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_ALLOC_NAMED_ALIGNED;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_GAME_UTIL_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_HIGHLIGHT_SLOT_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SAVE_EMIT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_CMD_QUEUE_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_CHOOSE_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_ADD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_PLAY_STATUS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_SKIP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_GET_WORLD_SECTION_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_FREE_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_GET_BUFFER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_GET_BUFFER_SIZE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_RELEASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_REMOVE_TICK_CALLBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_ARCADE_SAY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_EMPTY_CROWD_HANDLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_HANDLE_MUSIC_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_PLAY_CHARACTER_TUNE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_RIDER_SAY_20B9;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_SOUND_BASE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_RPC_SET_PARAMS;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_RPC_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::WSCRIPT_MAN_DELETE_ALL_PROCESSES;
    pub(crate) use crate::game::dirtysock::tags2::callee::WSCRIPT_MAN_SPAWN_MISSIONS;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_PUSH_2 as ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ANIM_CONTROLLER_IS_LAYER_ANIM_FINISHED;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_TRY_EXIT_2 as APP_MODULE_TRY_EXIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATOI;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATTACHED_EMITTER_FX_DESTROY_EMITTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_MUSIC_TIER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_TRACK_TYPE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_LEVEL_TO_VOLUME_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_CHANNEL_VOLUME_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_TRIGGER;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_APP;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_BE_2 as BE_GET_BE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_CHAR_DISPLAY_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_INTERFACE_GET_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INSTANCE_4 as BE_LIBRARY_GET_INSTANCE_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INTERFACE_4 as BE_LIBRARY_GET_INTERFACE_3;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_API_DESTROY;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_COUNT_2 as BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_ENTRY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_SET_OWNER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_SET_IN_GAME_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBE_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBXSTRING_ASSIGN;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFE_STATE_LOGIN_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFE_STATE_NET_CONFIG_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_FILL_PLAYER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_GET_CHARACTER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHAT_ROOM_SELECT_PLAYER_BY_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::CLEAR_MUSIC_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CUI_TEXT_SCROLL_CLEAR_LINES;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::C_TRANSITION_MAN_START;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_TRANSITION_MAN_START_CAPTURED;
    pub(crate) use crate::game::fe::feasyncfile::callee::DATA_INSTANCE_FREE_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::DEBUG_MENU_DISPATCH_ITEM_OP;
    pub(crate) use crate::game::fe::feasyncfile::callee::EAFILE_GET_PATH_SIZE_CHECKED;
    pub(crate) use crate::game::fe::feasyncfile::callee::EAFILE_LOAD_FILE_INTO;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_ALLOC_NAMED_5 as EA_MEM_ALLOC_NAMED_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::EDITOR_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ASYNC_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_FINISH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_START_PROFILE_SEARCH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_PLAYER_PAD_MASK_2 as FE_MODULE_GET_PLAYER_PAD_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_RELEASE_BOLT_FILES;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE as FE_MODULE_RIDER_PREVIEW_SET_VISIBLE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_UNLOAD_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_MANAGER_ADD_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_READ_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODE_MAN_RESET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_ADVANCE_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_QUIT_TO_LOAD_SCREEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_REQUEST_STATE_5 as GAME_MODULE_REQUEST_STATE_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_RESET_RACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_START_NEXT_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_PROCESS_DTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_RAND_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_IS_LOADING;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_FLAG_MANAGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::fe::feasyncfile::callee::GRAPHICS_MAN_FREE_ALL_TEXTURES;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_BINDING_EVAL_FLOAT;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_EVENT_TRACK_FIRE_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_EVENT_TRACK_NEXT_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_INSTANCE_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_CHANNEL_GET_LENGTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_SPEED;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_USER_DATA_2 as IN_SEQ_INSTANCE_SET_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_START_SEQUENCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_POPUP_FIND_ENABLED_IN_COLUMN_FROM_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_POPUP_IS_KEY_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_AGREEMENT_TERMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_INDICATOR_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_DESTROY_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_FLUSH_ROSTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_GET_RECORD;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_LIST_GET_AT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_CONNECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_HIDE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_RELEASE_SNAPSHOT_LISTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_DECLINE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SHOW;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_TABLE_FIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_CALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_PASS_THROUGH;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_AS_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR_2 as LUNO_VM_REGISTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_SELECT_ALGORITHM;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_FORMAT_SLOT_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_GET as MC_MANAGER_GET_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_GET_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MDF_FILE_CREATE_MODEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCPY_2 as MEMCPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC_NAMED_128;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_DRAW_ROW_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_MEASURE_ROW;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_SET_ACTIVE_2 as MENU_SYS_SET_ACTIVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_SET_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_EXTENSION_AND_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_GET_BITS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_MARKER_BIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_FADE_OUT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_PLAY_SONG;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_RESUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_STOP_SONG_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_GOTO_NEXT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_OPEN_KEYBOARD_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP as NET_FE_STATE_GET_POPUP_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_2 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_3 as NET_FE_STATE_GET_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GOTO_NEXT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_2 as NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_10 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_13 as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP_2 as NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_5 as NET_FE_STATE_SHOW_POPUP_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_GAME_UTIL_ADVERT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_GAME_UTIL_CREATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED_4 as OPERATOR_NEW_NAMED_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE_CALLEE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_MESSAGE_CENTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_PDA_TEMPLATE_CLOSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_DATA_FIND_MARKER;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_RESUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SET_RUNNING;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SUSPEND;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_SET_OPTION_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_PAD_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_COUNT_SAVE_SNAPSHOTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARDS_FILE_PARSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_ADD_WIPEOUT_RECOVERY_BOOST;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_CHECK_RESET_PATH;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_GET_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_GET_START_POSE_ANGLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_IS_LOADED;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_METRIX_ON_RIDER_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_METRIX_SHOW_HUD_MSG_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_PLACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SCRIPT_OBJ_MATCHES_RIDER_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SET_INPUT_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ANIM_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_4 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_DATA_GET_BANK_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_ADD_SCRIPT_OBJ;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_CHOOSE_SCRIPT as SCRIPT_ENGINE_CHOOSE_SCRIPT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_CLEAR_SKIP_DELAY_ON_EXIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_INSTANCE_3 as SCRIPT_ENGINE_GET_INSTANCE_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_3 as SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE_2 as SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_TRANSITION_TARGET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_KILL_ALL_PROCESSES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_POP_FREE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_PUSH_FREE_NODE_2 as SCRIPT_ENGINE_PUSH_FREE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS_2 as SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_BINDING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE_2 as SCRIPT_ENGINE_RESET_PROCESS_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_OBJ_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_START_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_TICK_PROCESS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_UNREGISTER_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_3 as SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_SET_PAUSED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_3 as SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_RIDER_GET_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_SOUND_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ADD_CAM;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ON_ACTIVATED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_SET_TRANSITION;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_IS_LOADED_WITH_NAME as SND_BANK_IS_LOADED_WITH_NAME_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_LOAD as SND_BANK_LOAD_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_LOAD_BANK_FROM_MEMORY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_UNLOAD_BANK_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_FREE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK as SND_LOAD_QUEUE_CANCEL_BANK_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK_2 as SND_LOAD_QUEUE_CANCEL_BANK_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_PICK_RANDOM_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPCH_MAKE_ID_2 as SPCH_MAKE_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_SYSTEM_IS_INSTANCE_READY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_AABB_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COMPUTE_OFFSET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_CONFIGURE_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_UPDATE_EMITTER_ENABLES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_5 as SPRINTF_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_6 as SPRINTF_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_CHARACTER_TO_SPEECH_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_FADE_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_3 as SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_4 as SSX_AUDIO_SYSTEM_GET_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_IS_SPEECH_VOLUME_ON;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_SET_FADE_GAIN;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_SET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCASECMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::STR_HASH as STR_HASH_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::STR_HASH_5 as STR_HASH_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SYNC_PROCESS_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TAG_FIELD_GET_BINARY;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_HIDE_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_TEXT;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_ADVANCE as UI_ANIM_ADVANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_APPLY as UI_ANIM_APPLY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME_2 as UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ENGINE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST as UI_LIST_FIRST_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_3 as UI_LIST_NODE_DTOR_BODY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_4 as UI_LIST_NODE_DTOR_BODY_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_MENU_SET_SELECTION_3 as UI_MENU_SET_SELECTION_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_ADD_CHILD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_LOAD as UI_OBJECT_LOAD_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_5 as UI_SCREEN_FIND_OBJECT_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_GET_OBJECT_BY_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LAUNCH_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LOOKUP_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_2 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_SET_OBJECT_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCROLL_SET_PARTS_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_CTOR_2_4 as UI_STATE_CTOR_2_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_ALL_POPUPS_2 as UI_STATE_MGR_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_TOP_STATES as UI_STATE_MGR_CLOSE_TOP_STATES_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_FIND_STATE_2 as UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_SWITCH_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXTURE_SET_FIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_GET_ID_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_GET_OWNED_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_SET_TEXT_ID_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_CPY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_LEN_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_VSPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_LEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_SPRINTF_ASCII_FMT;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_TO_ASCII;
    pub(crate) use crate::game::fe::feasyncfile::callee::VSPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_IS_HULL_FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_STREAMER_UPDATE_VIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_SOUND_GET_TRIGGER_IDS;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_CREATE_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_FINALIZE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_IS_FLAG4;
    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_NODE_DTOR;
    pub(crate) use crate::game::intersect::worldsphtree::callee::MEMCARD_SJIS_CHAR_TO_ASCII;
    pub(crate) use crate::game::intersect::worldsphtree::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::intersect::worldsphtree::callee::MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::intersect::worldsphtree::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER;
    pub(crate) use crate::game::intersect::worldsphtree::callee::POSITION_MODIFIER_CTOR;
    pub(crate) use crate::game::intersect::worldsphtree::callee::PS2_MEMCARD_SET_STATE;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_BINDING_HAS_PROCESS;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_GET_INSTANCE as SCRIPT_ENGINE_GET_INSTANCE_3;
    pub(crate) use crate::game::intersect::worldsphtree::callee::SCRIPT_ENGINE_GET_PROCESS as SCRIPT_ENGINE_GET_PROCESS_3;
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
    pub(crate) use crate::game::intersect::worldsphtree::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT;
    pub(crate) use crate::game::intersect::worldsphtree::callee::WSCRIPT_PROCESS_LIST_REMOVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR as ACTIVE_CAMERA_TRIGGER_LIST_CLEAR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::AI_CREATE_ACTOR_RIDERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::APP_VSYNC_CALLBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ATTACHED_DYNAMIC_EMITTER_CREATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::AUDIO_SYSTEM_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BANK_MONITOR_RESET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BELIBRARY_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_CHARACTER as BE_GET_RIDER_CHARACTER_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_REWARD_INT_IS_PEAK_GOAL_COMPLETE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUILD_BUDDY_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_ALGORITHM_DEFAULT_FOV;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_CLEAR_DATA;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_REMOVE_ALL_TRIGGERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_STACK_CLEAR_2 as CAMERA_TRIGGER_STACK_CLEAR_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CONCAT_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_LOAD_STATE_SETUP_RESULT_WIDGET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_NET_ACCOUNT_MEM_SET_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_OPTIONS_CONTROLLER_SET_VIBRATION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CHANNEL_GET_TRACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CHASE_CAM_CONTROLLER_RESET_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_MAN2D_CLEAR_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_FLAG_MANAGER_GROUP_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::DYNAMIC_EMITTER_FREE_DATA_2 as DYNAMIC_EMITTER_FREE_DATA;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::EVENT_SELECT_ON_WIDGET_ACTION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_ALL_TABLES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_TABLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FIND_FREE_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_PROCESS_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GET_RACE_MAN;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GET_SELECTED_USER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GOTO_NEXT_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::HANDLE_NET_EVENT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_PREPROC_WRITE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_PACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_UNPACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_INSTANCE_GET_REMAINING_STEPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_MASK_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_CURSOR_2 as KEYBOARD_POPUP_UPDATE_CURSOR;
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
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_TEX_FLIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MOD_OBJ_CREATE_MODIFIER_BLOCK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MULTI_PARTICLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::NET_FE_STATE_DTOR_3 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ONLINE_CHARACTER_SELECT_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ON_CHILD_EVENT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PATH_PLAT_FIND_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_PLAYBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_RECORDING;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RCMP_DECODER_RELEASE_CODEC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_CACHE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_END_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_SET_CONTROLLER_INDEX;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_STREAM_REWIND_READ;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REQUEST_IMAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_ADD_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_HAS_RENDER_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_BONUS_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_SHOW_HUD_NUMBER_2 as RIDER_METRIX_SHOW_HUD_NUMBER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_SAY_20B9;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_4 as SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO_4 as SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE_2 as SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_EXIT_SCRIPT_MODE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_PROCESS_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_TARGET_OBJECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_OBJ_TABLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_CAN_SKIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_GET_PLAY_STATUS_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_3 as SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_ON_SEQ_END;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_STOP_2 as SCRIPT_LIST_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR_3 as SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SEND_PACKET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SEND_CHARACTER_MSG;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SET_LOCAL_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SET_SELECTED_USER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SHOW_BUDDY_ACTION_POPUP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_UNLOAD_MEMORY_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::START_RIDER_BOARD_LOOPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::STOP_BOARD_LOOPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_SCRIPT_SET_OWNER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_ASCII_TO_WIDE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_CTOR_2 as UI_LIST_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_FIRST_5 as UI_LIST_FIRST_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_MENU_SET_SELECTION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_SCREEN_FIND_OBJECT_3 as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_STATE_CTOR_2_2 as UI_STATE_CTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_TEXT_SET_WIDE_STRING_INTERNAL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VEC3_NEG_DOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
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
    pub(crate) use crate::game::world::wscriptcache::callee::FONT_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::INPUT_PREPROC_WRITE_STRING;
    pub(crate) use crate::game::world::wscriptcache::callee::MDF_FILE_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::MPEG2_FLUSH_BUFFER_3 as MPEG2_FLUSH_BUFFER;
    pub(crate) use crate::game::world::wscriptcache::callee::MPEG2_GET_BITS1;
    pub(crate) use crate::game::world::wscriptcache::callee::PS2_SONY_CODEC_INTERNAL_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::RCMP_SYSTEM_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::RIDER_SET_MAIN_STATE;
    pub(crate) use crate::game::world::wscriptcache::callee::RIDER_SET_SUB_STATE;
    pub(crate) use crate::game::world::wscriptcache::callee::SCRIPT_ENGINE_GET_INSTANCE as SCRIPT_ENGINE_GET_INSTANCE_2;
    pub(crate) use crate::game::world::wscriptcache::callee::SCRIPT_SOUND_BANK_MAN_FIND_BANK;
    pub(crate) use crate::game::world::wscriptcache::callee::SECTION_ALLOCATOR_FREE_ALL;
    pub(crate) use crate::game::world::wscriptcache::callee::SND_BANK_MONITOR_MEM_SIZE;
    pub(crate) use crate::game::world::wscriptcache::callee::SPRINTF_2 as SPRINTF;
    pub(crate) use crate::game::world::wscriptcache::callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP5;
    pub(crate) use crate::game::world::wscriptcache::callee::SSX_AUDIO_SYSTEM_PAUSE;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_ANIM_ADVANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_ANIM_APPLY;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_LIST_COUNT;
    pub(crate) use crate::game::world::wscriptcache::callee::WSCRIPT_MAN_COLLECT_SET_ADD_ITEMS;
    pub(crate) use crate::game::world::wscriptcache::callee::WSCRIPT_MAN_COLLECT_SET_GET_COUNT;
}

/// Cross-shard direct calls the generator emitted as `call_indirect(c, addr)`.
pub(crate) mod table {
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

/// `void SndBank_Link(SndBank *self, SndBank *other)` (0x28b6c8): splice
/// `self` into the doubly-linked list right after `other`, then adopt
/// `other`'s data pointers.
pub fn snd_bank_link(c: &mut Cpu) {
    let self_ = Addr(abi::arg(c, 0));
    let other = Addr(abi::arg(c, 1));
    c.cycles += 3;
    let other_next = other.off(0x18).lw(c);
    if other_next == 0 {
        other.off(0x18).sw(c, c.g[A0][0]);
        c.cycles += 3;
        other.off(0x1c).sw(c, c.g[A0][0]);
        self_.off(0x18).sw(c, c.g[A1][0]);
        self_.off(0x1c).sw(c, c.g[A1][0]);
    } else {
        c.cycles += 7;
        self_.off(0x18).sw(c, c.g[A1][0]);
        let next = other.off(0x1c).lw(c);
        self_.off(0x1c).sw(c, next);
        other.off(0x1c).sw(c, c.g[A0][0]);
        let back = self_.off(0x1c).lw(c);
        Addr(back as u32).off(0x18).sw(c, c.g[A0][0]);
    }
    c.cycles += 3;
    let data = other.off(0x8).lw(c);
    c.g[V0][0] = data;
    if data != 0 {
        c.cycles += 7;
        self_.off(0x8).sw(c, data);
        let p10 = other.off(0x10).lw(c);
        self_.off(0x10).sw(c, p10);
        let p14 = other.off(0x14).lw(c);
        c.g[V1][0] = p14;
        self_.off(0x14).sw(c, p14);
        let p0c = other.off(0xc).lw(c);
        self_.off(0xc).sw(c, p0c);
        c.g[V0][0] = p0c;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void SndBank_Unlink(SndBank *self)` (0x28b730): remove `self` from the
/// doubly-linked list, keeping neighbours connected, and clear its links.
pub fn snd_bank_unlink(c: &mut Cpu) {
    let self_ = Addr(abi::arg(c, 0));
    c.cycles += 3;
    let prev = self_.off(0x18).lw(c);
    c.g[V1][0] = prev;
    c.g[V0][0] = u64::MAX;
    if prev != 0 {
        c.cycles += 6;
        let next = self_.off(0x1c).lw(c);
        c.g[A1][0] = next;
        self_.off(0xc).sw(c, u64::MAX);
        self_.off(0x10).sw(c, 0);
        self_.off(0x14).sw(c, 0);
        self_.off(0x8).sw(c, 0);
        if prev != next {
            c.cycles += 4;
            Addr(prev as u32).off(0x1c).sw(c, next);
            c.g[V1][0] = self_.off(0x1c).lw(c);
            c.g[V0][0] = self_.off(0x18).lw(c);
            Addr(c.g[V1][0] as u32).off(0x18).sw(c, c.g[V0][0]);
        } else {
            c.cycles += 4;
            Addr(prev as u32).off(0x18).sw(c, 0);
            c.g[V0][0] = self_.off(0x18).lw(c);
            Addr(c.g[V0][0] as u32).off(0x1c).sw(c, 0);
            c.g[V1][0] = prev;
        }
        c.cycles += 2;
        self_.off(0x18).sw(c, 0);
        self_.off(0x1c).sw(c, 0);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void SndBank_Unlink(SndBank *bank)` (0x28b730): unlink the bank from its
/// doubly-linked list and clear its link fields.
pub fn snd_bank_unlink_2(c: &mut Cpu) {
    let bank = Addr(abi::arg(c, 0));
    'outer: {
        c.cycles += 3;
        let next = bank.off(0x18).lw(c);
        c.g[V1][0] = next;
        c.g[V0][0] = u64::MAX;
        if next == 0 {
            break 'outer;
        }
        'prev_link: {
            'forward_link: {
                c.cycles += 6;
                c.g[A1][0] = bank.off(0x1c).lw(c);
                bank.off(0xc).sw(c, u64::MAX);
                bank.off(0x10).sw(c, 0);
                bank.off(0x14).sw(c, 0);
                bank.off(0x8).sw(c, 0);
                if next != c.g[A1][0] {
                    break 'forward_link;
                }
                c.cycles += 4;
                Addr(next as u32).off(0x18).sw(c, 0);
                c.g[V0][0] = bank.off(0x18).lw(c);
                Addr(c.g[V0][0] as u32).off(0x1c).sw(c, 0);
                break 'prev_link;
            }
            c.cycles += 4;
            Addr(next as u32).off(0x1c).sw(c, c.g[A1][0]);
            c.g[V1][0] = bank.off(0x1c).lw(c);
            c.g[V0][0] = bank.off(0x18).lw(c);
            Addr(c.g[V1][0] as u32).off(0x18).sw(c, c.g[V0][0]);
            break 'prev_link;
        }
        c.cycles += 2;
        bank.off(0x18).sw(c, 0);
        bank.off(0x1c).sw(c, 0);
        break 'outer;
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Music track-id group predicates. Each returns whether the id lies in one
// contiguous run, and leaves `v0` as the sign-extended `id - first` too.

/// `bool SSXAudioSystem_IsTrackIdGroup0(int self, int id)` (0x2a40e0): ids
/// 0..4.
pub fn ssx_audio_system_is_track_id_group0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = (c.g[A1][0] < 0x5) as u64;
    abi::ret(c);
}

/// `bool SSXAudioSystem_IsTrackIdGroup4(int self, int id)` (0x2a4030): ids
/// 0xe..0x10.
pub fn ssx_audio_system_is_track_id_group4(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = sx(abi::arg(c, 1).wrapping_sub(0xe));
    c.g[V0][0] = (c.g[V0][0] < 0x3) as u64;
    abi::ret(c);
}

/// `bool SSXAudioSystem_IsTrackIdGroup1(int self, int id)` (0x2a4158): ids
/// 5..7.
pub fn ssx_audio_system_is_track_id_group1(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = sx(abi::arg(c, 1).wrapping_sub(5));
    c.g[V0][0] = (c.g[V0][0] < 0x3) as u64;
    abi::ret(c);
}

/// `bool SSXAudioSystem_IsTrackIdGroup2(int self, int id)` (0x2a41c8): ids
/// 8..0xa.
pub fn ssx_audio_system_is_track_id_group2(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = sx(abi::arg(c, 1).wrapping_sub(8));
    c.g[V0][0] = (c.g[V0][0] < 0x3) as u64;
    abi::ret(c);
}

/// `bool SSXAudioSystem_IsTrackIdGroup3(int self, int id)` (0x2a4238): ids
/// 0xb..0xd.
pub fn ssx_audio_system_is_track_id_group3(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = sx(abi::arg(c, 1).wrapping_sub(0xb));
    c.g[V0][0] = (c.g[V0][0] < 0x3) as u64;
    abi::ret(c);
}

/// `bool SSXAudioSystem_SetMusicState(SSXAudioSystem* a, int state)`
/// (0x28def0): stores the new music state at +0x629c and returns whether it
/// differed from the old one.
pub fn ssx_audio_system_set_music_state(c: &mut Cpu) {
    c.cycles += 5;
    let selfp = abi::arg(c, 0);
    let state = c.g[A1][0];
    c.g[V0][0] = c.lw(selfp.wrapping_add(0x629c));
    c.sw(selfp.wrapping_add(0x629c), state);
    c.g[V0][0] ^= state;
    c.g[V0][0] = (0 < c.g[V0][0]) as u64;
    abi::ret(c);
}

/// `void MusicPlayer_ClearLoopsActive(MusicPlayer* p)` (0x2b4708): clears the
/// loops-active word at +0x4c of the current song, if there is one.
pub fn music_player_clear_loops_active(c: &mut Cpu) {
    c.cycles += 3;
    let selfp = abi::arg(c, 0);
    c.g[V0][0] = c.lw(selfp.wrapping_add(0x408));
    if c.g[V0][0] != 0 {
        let song = c.g[V0][0] as u32;
        c.sw(song.wrapping_add(0x4c), 0);
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Sound bank manager.

/// `SndBankMan` methods index a 0x60-byte `SndBank` array at `self+0xacc`.
pub(crate) fn snd_bank_call(c: &mut Cpu, target: Callee, ret: u32) -> Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = 0x60;
    frame.save_ra(c, 0);
    c.mult(0, A1, A1, V0);
    let base = Addr(arg(c, 0).wrapping_add(0xacc)).lw(c);
    c.g[A0][0] = sx((base as u32).wrapping_add(c.g[A1][0] as u32));
    target.call(c, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void SndBankMan_UnloadBank(self, int bank)` (0x28bce8).
pub fn snd_bank_man_unload_bank(c: &mut Cpu) {
    guest(c, |c| {
        snd_bank_call(c, callee::SND_BANK_UNLOAD, 0x0028_bd04)
    })
}

/// `void SndBankMan_FreeBankMemory(self, int bank)` (0x28bde0).
pub fn snd_bank_man_free_bank_memory(c: &mut Cpu) {
    guest(c, |c| {
        snd_bank_call(c, callee::SND_BANK_FREE_MEMORY, 0x0028_bdfc)
    })
}

/// `void SndBankMan_UnloadMemoryBank(self, int bank)` (0x28be90).
pub fn snd_bank_man_unload_memory_bank(c: &mut Cpu) {
    guest(c, |c| {
        snd_bank_call(c, callee::SND_BANK_UNLOAD_MEMORY, 0x0028_beac)
    })
}

/// `void SndBankMan_ReleaseBank(self, int bank)` (0x28beb8).
pub fn snd_bank_man_release_bank(c: &mut Cpu) {
    guest(c, |c| {
        snd_bank_call(c, callee::SND_BANK_RELEASE, 0x0028_bed4)
    })
}

/// `int SndBankMan_GetBankBufferSize(self, int bank)` (0x28bf10).
pub fn snd_bank_man_get_bank_buffer_size(c: &mut Cpu) {
    guest(c, |c| {
        snd_bank_call(c, callee::SND_BANK_GET_BUFFER_SIZE, 0x0028_bf2c)
    })
}

/// `bool SSXAudioSystem_IsGameMusicEnabled(SSXAudioSystem* self)`
/// (0x28d960): true when the game-music state at `+0x608c` is 0, 1 or 3.
pub fn ssx_audio_system_is_game_music_enabled(c: &mut Cpu) {
    c.cycles += 4;
    let state = Addr(arg(c, 0).wrapping_add(0x608c)).lw(c);
    c.g[V1][0] = state;
    let below_two = state < 2;
    c.g[V0][0] = below_two as u64;
    c.g[V0][0] = 3;
    if !below_two {
        c.cycles += 2;
        if state != 3 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `void *SndBankMan_GetBankBuffer(SndBankMan *self, int index)` (0x28bee0):
/// the memory buffer of bank `index` (each `SndBank` is 0x60 bytes in the
/// array at `self+0xacc`).
pub fn snd_bank_man_get_bank_buffer(c: &mut Cpu) {
    /// The array of `SndBank`s.
    const BANKS: u32 = 0xacc;
    /// Size of one `SndBank`.
    const BANK_SIZE: u32 = 0x60;

    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = BANK_SIZE as u64;
        frame.save_ra(c, 0);
        c.mult(0, V1, A1, V0);
        c.g[A1][0] = c.g[A2][0];
        c.g[A0][0] = Addr(c.g[A0][0] as u32).off(BANKS).lw(c);
        c.g[A2][0] = c.g[A3][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        callee::SND_BANK_GET_BUFFER.call(c, 0x0028_bf04)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SndBankMan_SetBankMemory(SndBankMan* self, int bank, void* mem,
/// uint32 flags, uint32 arg)` (0x28bda8): pass the per-bank memory block of
/// the `mem` pointer to `SndBank_SetMemory`.
pub fn snd_bank_man_set_bank_memory(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x60;
        frame.save_ra(c, 0);
        c.mult(0, V1, A1, V0);
        c.g[A1][0] = c.g[A2][0];
        c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xacc)).lw(c);
        c.g[A2][0] = c.g[A3][0];
        c.g[A3][0] = c.g[T0][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        callee::SND_BANK_SET_MEMORY.call(c, 0x0028_bdd0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SSXAudioSystem_StopFEMusic(SSXAudioSystem* self)` (0x28f520):
/// stop the front-end music player, if game music is enabled.
pub fn audio_stop_fe_music(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::AUDIO_IS_GAME_MUSIC_ENABLED.call(c, 0x0028_f534)?;
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 3;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x118));
            callee::MUSIC_PLAYER_STOP_SONG.call(c, 0x0028_f544)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Sound.

/// `void SSXAudioSystem_EndScriptMusic(SSXAudioSystem* self)` (0x28e888):
/// post the script-music stop event and clear the handle.
pub fn ssx_audio_system_end_script_music(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x6254).lw(c);
        let silent = c.g[V0][0] == 0;
        c.g[A1][0] = 0x13;
        if !silent {
            c.cycles += 3;
            c.g[A2][0] = 1;
            callee::SSX_AUDIO_HANDLE_MUSIC_EVENT.call(c, 0x0028_e8ac)?;
            let s0 = c.g[S0][0];
            Addr(s0 as u32).off(0x6254).sw(c, 0);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_UnloadLoadingScreenBank(SSXAudioSystem* self)`
/// (0x28f730): fade out the loading-screen loop and unload its bank.
pub fn ssx_audio_system_unload_loading_screen_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.mtc1(0, 12);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SSX_AUDIO_STOP_LOADING_SCREEN_LOOP.call(c, 0x0028_f748)?;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x118).lw(c);
        c.g[A1][0] = 0;
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        callee::SND_BANK_MAN_UNLOAD_BANK.call(c, 0x0028_f758)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool SSXAudioSystem_IsTrackGroup5(SSXAudioSystem* s)` (0x2a3f90): is the
/// current track's id in the group-5 range (0x11..0x15)? Reads the id through
/// the BE library's current phase interface.
pub fn ssx_audio_system_is_track_group5(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::BELIBRARY_GET_INSTANCE.call(c, 0x002a_3fa4)?;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        callee::BELIBRARY_GET_INTERFACE.call(c, 0x002a_3fb0)?;
        c.g[A0][0] = c.g[V0][0];
        callee::BE_NEW_RACE_INT_GET_TRACK_INFO.call(c, 0x002a_3fb8)?;
        c.g[A1][0] = c.lw(r32(c, V0));
        c.g[A0][0] = c.g[S0][0];
        callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP5.call(c, 0x002a_3fc4)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int SndBank_IsLoadedWithName(self, name)` (0x28b830): true when the bank
/// is in state 1 and its name (`self+0x20`) matches `name`.
pub fn snd_bank_is_loaded_with_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x10);
        c.g[V0][0] = 1;
        let self_ = c.g[V1][0] as u32;
        c.g[S0][0] = Addr(self_).lw(c);
        let not_loaded = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = c.g[A1][0];
        if not_loaded {
            c.cycles += 1;
            c.g[V0][0] = 0;
            c.cycles += 4;
            frame.restore(c, S0, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 5;
        c.g[A1][0] = sx(self_.wrapping_add(0x20));
        call_ptr(c, 0x0041_65a8, 0x0028_b858)?;
        if c.g[V0][0] != 0 {
            c.g[S0][0] = 0;
        }
        c.g[V0][0] = c.g[S0][0];
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared body of the three race-type range checks: is the current race
/// type (the byte at 0x535c12) inside `[lo, hi]`?
pub(crate) fn race_type_in_range(c: &mut Cpu, lo: i64, hi: i64, rets: [u32; 3]) -> Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    call_ptr(c, 0x0014_dc80, rets[0])?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x0014_dd58, rets[1])?;
    c.g[V1][0] = 0x530000;
    c.g[V1][0] = Addr(0x0053_5c12).lb(c);
    c.g[V0][0] = ((c.g[V1][0] as i64) < lo) as u64;
    let below = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    if !below {
        c.cycles += 3;
        c.g[V1][0] = ((c.g[V1][0] as i64) < hi) as u64;
        let inside = c.g[V1][0] != 0;
        c.g[V0][0] = 1;
        if !inside {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
    }
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `bool SSXAudioSystem_IsRaceType6to11(void)` (0x2a42c8).
pub fn ssx_audio_system_is_race_type6_to11(c: &mut Cpu) {
    guest(c, |c| {
        race_type_in_range(c, 6, 12, [0x002a_42d8, 0x002a_42e4, 0x002a_4308])
    })
}

/// `void SndBank_UnloadMemory(SndBank* self)` (0x28b7e0): detach the bank from
/// the sound system and clear the memory/state fields.
pub fn snd_bank_unload_memory(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = u64::MAX; // -1: no bank
        Addr(this).sw(c, 0);
        c.g[A0][0] = Addr(this).off(4).lw(c);
        if c.g[A0][0] != c.g[S1][0] {
            c.cycles += 4;
            call_ptr(c, table::SND_BANK_REMOVE, 0x0028_b810)?;
            Addr(this).off(4).sw(c, c.g[S1][0]);
            Addr(this).off(0x14).sw(c, 0);
        } else {
            Addr(this).off(0x14).sw(c, 0);
        }
        c.cycles += 6;
        Addr(this).off(0x10).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SndBank_LoadFromMemory(SndBank* b, u32 a1, u32 a2)` (0x28b788): drop
/// the bank's current memory, install the new pointer/length and rebuild the
/// tag table.
pub fn snd_bank_load_from_memory(c: &mut Cpu) {
    c.cycles += 22;
    let frame = Frame::open(c, 0x40);
    frame.save(c, S0, 0x30);
    c.g[S0][0] = c.g[A0][0];
    frame.save(c, S1, 0x20);
    frame.save(c, S2, 0x10);
    c.g[S1][0] = c.g[A2][0];
    frame.save_ra(c, 0);
    c.g[S2][0] = c.g[A1][0];
    if callee::SND_BANK_UNLOAD_MEMORY_2
        .call(c, 0x0028_b7ac)
        .is_err()
    {
        return;
    }
    c.sw(p(c, S0, 0x14), c.g[S1][0]);
    c.g[A1][0] = c.g[S2][0];
    c.sw(p(c, S0, 0x10), c.g[S2][0]);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
    if call_ptr(c, 0x003b_6098, 0x0028_b7c0).is_err() {
        return;
    }
    c.g[V1][0] = 1;
    frame.restore(c, S1, 0x20);
    c.sw(p(c, S0, 0), c.g[V1][0]);
    frame.restore(c, S0, 0x30);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Sound banks.

/// `void SndBankMan_InitBankRequest(SndBankMan* man, int, int, int, int,
/// int, int)` (0x28bf38): forward a bank-load request to
/// `SndBankMan_InitLoadRequest(man->banks + arg1*0x60, ...)` with the
/// arguments shifted up by one (the byte offset is built by the `mult`).
pub fn snd_bank_man_init_bank_request(c: &mut Cpu) {
    const RET: u32 = 0x0028_bf68;
    guest(c, |c| {
        c.cycles += 15;
        c.g[V0][0] = 0x60;
        let sp = r32(c, SP).wrapping_sub(0x10);
        c.g[SP][0] = sx(sp);
        c.mult(0, 3, 5, 2);
        Addr(sp).sd(c, c.g[RA][0]);
        c.g[A1][0] = c.g[A2][0];
        c.g[A2][0] = c.g[A3][0];
        c.g[A0][0] = Addr(r32(c, A0).wrapping_add(0xacc)).lw(c);
        c.g[A3][0] = c.g[T0][0];
        c.g[T0][0] = c.g[T1][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[T1][0] = c.g[T2][0];
        callee::SND_BANK_MAN_INIT_LOAD_REQUEST.call(c, RET)?;
        c.g[RA][0] = Addr(sp).ld(c);
        c.g[SP][0] = sx(sp.wrapping_add(0x10));
        abi::ret(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_SetSelectedPeak(SSXAudioSystem* self, int id)`
/// (0x28F558): store the peak-music id and restart that peak if the audio
/// manager is already playing it.
pub fn ssx_audio_system_set_selected_peak(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw(ea(c.g[S0][0], 0x6284), c.g[A1][0]);
        call_ptr(c, addr::CBE_GET_BE, 0x0028_f570)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, addr::CBE_GET_INTERFACE, 0x0028_f57c)?;
        if c.g[V0][0] != 0 {
            c.cycles += 6;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, addr::BE_INTERFACE_LOOKUP, 0x0028_f58c)?;
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            c.g[V0][0] = c.lw(ea(c.g[S0][0], 0x6284));
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                callee::SSX_AUDIO_SYSTEM_RESTART_PEAK_MUSIC.call(c, 0x0028_f5a4)?;
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SndBank_Release(SndBank* bank)` (0x28b878): unload the bank, mark it
/// released and unlink it from its parent when it was playing.
pub fn snd_bank_release(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let bank = c.g[A0][0];
        c.g[S0][0] = bank;
        call_ptr(c, 0x0028_b528, 0x0028_b88c)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0028_b2d0, 0x0028_b894)?;
        c.g[V0][0] = 0x2;
        c.g[V1][0] = Addr(at(c, S0, 0x8)).lw(c);
        let not_playing = c.g[V1][0] != c.g[V0][0];
        Addr(at(c, S0, 0x0)).sw(c, c.g[V0][0]);
        if !not_playing {
            c.cycles += 9;
            c.g[V1][0] = Addr(gp(c, 0x524)).lw(c);
            c.g[A1][0] = Addr(at(c, S0, 0xc)).lw(c);
            c.g[V0][0] = Addr(r32_2(c, V1)).off(0x1d8).lw(c);
            c.g[A2][0] = Addr(r32_2(c, V0)).lw(c);
            c.g[V1][0] = Addr(r32_2(c, A2)).lw(c);
            c.g[A0][0] = Addr(r32_2(c, V1)).off(0x30).lh(c);
            c.g[V0][0] = Addr(r32_2(c, V1)).off(0x34).lw(c);
            let target = r32_2(c, V0);
            c.g[A0][0] = abi::addu(c.g[A2][0], c.g[A0][0]);
            call_ptr(c, target, 0x0028_b8c8)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem_OnReplayEnd(SSXAudioSystem* self)` (0x28f2c0): stop the
/// music player backing the replay (`+0x6294`) or send the stop command
/// (`0x24`) through the bank manager vtable (`+0x5558`), then clear both
/// replay flags.
pub fn ssx_audio_system_on_replay_end(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        let base = this as u32;
        c.g[V0][0] = lw(c, p_2(base, 0x6294));
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = sx(p_2(base, 0x118));
            call_ptr(c, 0x002b_3ac0, 0x0028_f2e4)?;
            sw(c, p_2(base, 0x6298), 0);
            c.cycles += 5;
            sw(c, p_2(base, 0x6294), 0);
            frame.restore(c, S0, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.g[V0][0] = lw(c, p_2(base, 0x6298));
        c.cycles += 2;
        let idle = c.g[V0][0] == 0;
        c.g[A2][0] = sx(p_2(base, 0x118));
        if !idle {
            c.cycles += 6;
            c.g[V1][0] = lw(c, p_2(base, 0x5558));
            c.g[A1][0] = 0x24;
            c.g[A0][0] = lh(c, p_2(c.g[V1][0] as u32, 0x20));
            c.g[V0][0] = lw(c, p_2(c.g[V1][0] as u32, 0x24));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0028_f30c)?;
        }
        c.cycles += 1;
        sw(c, p_2(base, 0x6298), 0);
        c.cycles += 5;
        sw(c, p_2(base, 0x6294), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem::RestartPeakMusic(self)` (0x28ef90): reset the peak-music
/// state, pick a random song and arm its timer.
pub fn ssx_audio_system_restart_peak_music(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p_3(c.g[S0][0] as u32, 0x6258)).sw(c, 0);
        Addr(p_3(c.g[S0][0] as u32, 0x577c)).sw(c, 0);
        call_ptr(c, 0x0028_df08, 0x0028_efac)?;
        Addr(p_3(c.g[S0][0] as u32, 0x578c)).sw(c, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0028_d488, 0x0028_efb8)?;
        c.g[V1][0] = Addr(p_3(c.g[S0][0] as u32, 0x118)).lw(c);
        c.g[V0][0] = 0x4a0000;
        c.ldl(A2, 0x004a_364f);
        c.ldr(A2, 0x004a_3648);
        c.g[A1][0] = 0xa;
        c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.g[A3][0] = 0;
        c.g[T0][0] = 0x2;
        c.g[T1][0] = 0x1;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
        c.g[T2][0] = 0;
        c.g[T3][0] = 0;
        call_ptr(c, 0x002a_dca0, 0x0028_efec)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SndBank_FreeMemory` (0x28B650): unload a sound bank and release its stream
/// and memory.
pub fn snd_bank_free_memory(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::SND_BANK_UNLOAD.call(c, 0x0028_b664)?;
        c.g[V1][0] = lw_at(c, S0, 0x8);
        let free = c.g[V1][0] == 0;
        c.g[V0][0] = 2;
        if free {
            c.cycles += 4;
            frame.restore(c, S0, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 2;
        let streaming = c.g[V1][0] != c.g[V0][0];
        c.g[V1][0] = Addr(gp(c, 0x524)).lw(c);
        if !streaming {
            c.cycles += 10;
            c.g[A1][0] = lw_at(c, S0, 0xc);
            c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x1d8)).lw(c);
            c.g[A3][0] = Addr(r32(c, V0)).lw(c);
            c.g[V1][0] = Addr(r32(c, A3)).lw(c);
            c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x28)).lh(c);
            c.g[A2][0] = Addr(r32(c, V1).wrapping_add(0x2c)).lw(c);
            let stop = r32(c, A2);
            c.g[A0][0] = abi::addu(c.g[A3][0], c.g[A0][0]);
            call_ptr(c, stop, 0x0028_b698)?;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            sw_at(c, S0, 0xc, c.g[V0][0]);
        }
        c.cycles += 5;
        c.g[A0][0] = lw_at(c, S0, 0x10);
        call_ptr(c, addr::UI_ENGINE_TEARDOWN, 0x0028_b6a8)?;
        sw_at(c, S0, 0x8, 0);
        sw_at(c, S0, 0x10, 0);
        sw_at(c, S0, 0x14, 0);
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MusicPlayer_SetPlaylistMask(MusicPlayer* m, u32 mask)`
/// (0x002b4070): clear the playlist, then enable every song whose bit is set
/// in `mask`, finally poke the empty-playlist hook.
pub fn music_player_set_playlist_mask(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
        c.sq(S1, sp(c).wrapping_add(0x20));
        c.sq(S2, sp(c).wrapping_add(0x10));
        c.g[S1][0] = c.g[A0][0];
        c.sq(S0, sp(c).wrapping_add(0x30));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S2][0] = c.g[A1][0];
        call_ptr(c, 0x002b_3eb0, 0x002b_4090)?; // MusicPlayer_ClearPlaylist
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3ec));
        c.g[S0][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 1;
            c.g[V0][0] = c.g[S2][0] >> (c.g[S0][0] & 63);
            loop {
                c.cycles += 5;
                c.g[V0][0] = c.g[V0][0] & 1;
                c.g[V0][0] = c.g[V0][0] << 32;
                c.g[V0][0] = ((c.g[V0][0] as i64) >> 32) as u64;
                let zero = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S1][0];
                if !zero {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x002b_3fe8, 0x002b_40bc)?; // MusicPlayer_EnableSong
                }
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x3ec));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = c.g[S2][0] >> (c.g[S0][0] & 63);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002b_40a0);
                }
            }
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x002b_4060, 0x002b_40d8)?; // MusicPlayer_EmptyPlaylistHook
        c.lq(S0, sp(c).wrapping_add(0x30));
        c.lq(S1, sp(c).wrapping_add(0x20));
        c.lq(S2, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        abi::ret(c);
        Ok(())
    })
}

/// `void SndBankMan_InitLoadRequest(SndBankMan* self, u32 bank, u32 async,
/// void* cb)` (0x28b930): ask the bank's slot for the loading screen, mark
/// the request pending and start it.
pub fn snd_bank_man_init_load_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        open(c, 0x50);
        let sp0 = spv(c);
        let g = gpa(c, 0x524);
        c.g[V0][0] = lw_at_2(c, g as u64, 0);
        c.sq(16, at_2(sp0, 0x40));
        c.sq(17, at_2(sp0, 0x30));
        c.g[S0][0] = c.g[A1][0];
        c.sq(18, at_2(sp0, 0x20));
        c.g[S1][0] = c.g[A2][0];
        c.sq(19, at_2(sp0, 0x10));
        c.g[S2][0] = c.g[A0][0];
        c.sd(sp0, c.g[RA][0]);
        c.g[S3][0] = c.g[A3][0];
        c.g[V1][0] = lw_at_2(c, c.g[V0][0], 0x1d8);
        c.g[A1][0] = lw_at_2(c, c.g[V1][0], 0);
        c.g[V0][0] = lw_at_2(c, c.g[A1][0], 0);
        c.g[A0][0] = lh_at(c, c.g[V0][0], 0x38);
        c.g[V1][0] = lw_at_2(c, c.g[V0][0], 0x3c);
        let func = c.g[V1][0] as u32;
        c.g[A0][0] = sx(at_2(c.g[A1][0] as u32, c.g[A0][0] as u32));
        call_ptr(c, func, 0x0028_b978)?;
        let pending = c.g[S1][0] != 0;
        let this = c.g[S2][0];
        c.sw(at_2(this as u32, 4), c.g[S0][0]);
        if !pending {
            c.cycles += 3;
            c.g[V0][0] = 1;
            c.sw(this as u32, c.g[V0][0]);
        } else {
            c.cycles += 1;
            c.sw(this as u32, 0);
        }
        c.cycles += 10;
        c.g[A0][0] = sx(at_2(this as u32, 0x20));
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, 0x0041_66f4, 0x0028_b99c)?;
        c.lq(16, at_2(sp0, 0x40));
        c.lq(17, at_2(sp0, 0x30));
        c.lq(18, at_2(sp0, 0x20));
        c.lq(19, at_2(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at_2(sp0, 0x50));
        c.pc = ra;
        Ok(())
    })
}

/// `void SSXAudioSystem_OnEventSelectExit(SSXAudioSystem* self, int resume)`
/// (0x28f678): drop the event-select audio mode, stop the loading-screen
/// loop and (when asked) start the event music.
pub fn ssx_audio_system_on_event_select_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        open(c, 0x30);
        let sp0 = spv(c);
        c.sq(16, at_2(sp0, 0x20));
        c.sq(17, at_2(sp0, 0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp0, c.g[RA][0]);
        let this = c.g[S0][0];
        c.g[V0][0] = lw_at_2(c, this, 0x6088);
        let none = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !none {
            c.cycles += 3;
            call_ptr(c, callee::SSX_AUDIO_RESUME_SFX, 0x0028_f6a0)?;
            c.sw(at_2(this as u32, 0x6088), 0);
        }
        c.cycles += 3;
        c.g[V0][0] = lw_at_2(c, this, 0x62a8);
        if (c.g[V0][0] as i64) < 0 {
            c.lq(16, at_2(sp0, 0x20));
        } else {
            c.cycles += 6;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 12);
            c.g[A0][0] = c.g[S0][0];
            callee::STOP_LOADING_SCREEN_LOOP.call(c, 0x0028_f6c0)?;
            let skip = c.g[S1][0] == 0;
            if skip {
                c.lq(16, at_2(sp0, 0x20));
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, callee::SSX_AUDIO_IS_TRACK_GROUP5, 0x0028_f6d0)?;
                let not_group5 = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S0][0];
                if !not_group5 {
                    c.cycles += 5;
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 0;
                    c.g[A3][0] = u64::MAX;
                    c.g[T0][0] = 0;
                    callee::SSX_AUDIO_PLAY_MUSIC.call(c, 0x0028_f6ec)?;
                }
                c.cycles += 1;
                c.lq(16, at_2(sp0, 0x20));
            }
        }
        c.cycles += 4;
        c.lq(17, at_2(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at_2(sp0, 0x30));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Sound banks.

/// `void SndBankMan_LoadBank(SndBankMan* self, int id, void* a2, void* a3)`
/// (0x28bc58): if the bank is not loaded yet, cancel its pending load and
/// load it into the bank table entry.
pub fn snd_bank_man_load_bank(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x60;
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x50);
        c.mult(0, S3, S2, V0);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S4, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A3][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = Addr(p_4(r32(c, S0), 0xacc)).lw(c);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S3)));
        callee::SND_BANK_IS_LOADED_WITH_NAME.call(c, 0x0028_bc9c)?;
        if c.g[V0][0] == 0 {
            c.cycles += 9;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S2][0];
            callee::SND_LOAD_QUEUE_CANCEL_BANK.call(c, 0x0028_bcb0)?;
            c.g[A0][0] = Addr(p_4(r32(c, S0), 0xacc)).lw(c);
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S4][0];
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S3)));
            callee::SND_BANK_LOAD.call(c, 0x0028_bcc4)?;
        }
        c.cycles += 7;
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

/// `void SSXAudioSystem_UpdateMusicTier(self, int tier)` (0x28d5a0): when the
/// music system is not in state 0x17, play the track for the given tier if it
/// differs from the one already playing.
pub fn ssx_audio_system_update_music_tier(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        let self_ = abi::arg(c, 0);
        let tier = abi::arg64(c, 1);
        c.g[V0][0] = 0x17;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = sx(self_);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.lw(at_3(c, S0, 0x6284));
        let idle = c.g[V1][0] == c.g[V0][0];
        c.g[S1][0] = tier;
        if !idle {
            c.cycles += 10;
            c.g[A0][0] = sx(p_5(r32(c, S0), 0x118));
            call_ptr(c, callee::AUDIO_GET_MUSIC_TIER, 0x0028_d5d0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[S2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, callee::AUDIO_GET_TRACK_TYPE, 0x0028_d5e0)?;
            c.g[A3][0] = c.g[V0][0];
            c.g[V0][0] = 1;
            let one = c.g[A3][0] == c.g[V0][0];
            c.g[A0][0] = 0x65;
            if !one {
                c.cycles += 4;
                c.g[A0][0] = 0x67;
                c.g[V1][0] = 0x66;
                c.g[V0][0] = c.g[A3][0] ^ 2;
                if c.g[V0][0] == 0 {
                    c.g[A0][0] = c.g[V1][0];
                }
            }
            c.cycles += 2;
            let same = c.g[A0][0] == c.g[S2][0];
            c.g[A0][0] = c.g[S0][0];
            if !same {
                c.cycles += 4;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                c.g[T0][0] = 0;
                call_ptr(c, callee::AUDIO_PLAY_MUSIC, 0x0028_d618)?;
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

/// `SndBankMan_LoadBankAsync` (0x28bd10): if the bank is not loaded, cancel
/// any queued load of the same name and queue it again with the entry's id.
pub fn snd_bank_man_load_bank_async(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x60;
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x50);
        c.mult(0, 19, 17, 2);
        frame.save(c, S2, 0x30);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        frame.save(c, S4, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A3][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = Addr(p_6(this, 0xacc)).lw(c);
        c.g[RA][0] = sx(0x0028_bd54);
        c.g[A0][0] = sx(r(c, A0).wrapping_add(r(c, S3)));
        callee::SND_BANK_IS_LOADED_WITH_NAME_2.call(c, 0x0028_bd54)?;
        if c.g[V0][0] != 0 {
            frame.restore(c, S0, 0x50);
        } else {
            c.cycles += 12;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0028_bd68);
            c.g[A1][0] = c.g[S1][0];
            callee::SND_LOAD_QUEUE_CANCEL_BANK_2.call(c, 0x0028_bd68)?;
            c.g[V0][0] = Addr(p_6(r(c, S0), 0xacc)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[V0][0] = sx(r(c, S3).wrapping_add(r(c, V0)));
            c.g[T0][0] = c.g[S4][0];
            c.g[RA][0] = sx(0x0028_bd88);
            c.g[A3][0] = Addr(p_6(r(c, V0), 8)).lw(c);
            callee::SND_LOAD_QUEUE_PUSH.call(c, 0x0028_bd88)?;
            frame.restore(c, S0, 0x50);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem_ConfirmMusicTrigger` (0x28d740): confirm a music trigger
/// when the object's request id matches, clearing the trigger otherwise.
pub fn ssx_audio_system_confirm_music_trigger(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[V0][0] = Addr(p_6(this, 0x6240)).lw(c);
        let same = c.g[V0][0] == c.g[S2][0];
        let mut skip = false;
        if !same {
            c.cycles += 4;
            c.g[V1][0] = Addr(p_6(this, 0x62a0)).lw(c);
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            let none = c.g[V1][0] == c.g[V0][0];
            c.g[A0][0] = 0;
            if !none {
                c.cycles += 9;
                c.g[V1][0] = Addr(p_6(r(c, S1), 0x6c0)).lw(c);
                c.g[A1][0] = sx(r(c, S1).wrapping_add(0x6c0));
                c.g[A0][0] = Addr(p_6(r(c, V1), 0x38)).lh(c);
                c.g[V0][0] = Addr(p_6(r(c, V1), 0x3c)).lw(c);
                let target = r(c, V0);
                c.g[RA][0] = sx(0x0028_d790);
                c.g[A0][0] = sx(r(c, A1).wrapping_add(r(c, A0)));
                call_ptr(c, target, 0x0028_d790)?;
                c.g[V1][0] = Addr(p_6(r(c, S0), 0x62a0)).lw(c);
                c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
                c.g[A0][0] = (c.g[V0][0] < 1) as u64;
            }
            c.cycles += 2;
            if c.g[A0][0] == 0 {
                frame.restore(c, S0, 0x30);
                skip = true;
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(p_6(this, 0x623c)).lw(c);
                let different = c.g[V0][0] != c.g[S2][0];
                c.g[A0][0] = c.g[S0][0];
                if different {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x0028_d7c0);
                    c.g[A1][0] = c.g[S1][0];
                    callee::CLEAR_MUSIC_TRIGGER.call(c, 0x0028_d7c0)?;
                } else {
                    c.cycles += 2;
                    Addr(p_6(this, 0x6240)).sw(c, c.g[S2][0]);
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x30);
            }
        } else {
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
            skip = true;
        }
        let _ = skip;
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_OnMusicPostEvent(AudioSystem* self, int event)`
/// (0x28f328): post the event to the embedded music player, then, when the
/// current song tag came back as none, track the restart events (10, 11, 18)
/// in `+0x530` and clear it (stopping the song through the player's vtable
/// slot 0x18 for event 18).
pub fn ssx_audio_system_on_music_post_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x118));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x002b_3b88, 0x0028_f350)?; // MusicPlayer_PostEvent
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x002b_49e0, 0x0028_f358)?; // MusicPlayer_GetSongTag
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        let mut early = false;
        if c.g[V0][0] != c.g[V1][0] {
            frame.restore(c, S0, 0x30);
            early = true;
        } else {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xa));
            c.g[V0][0] = (c.g[V0][0] < 2) as u64;
            let restart = c.g[V0][0] != 0;
            c.g[V1][0] = lw_3(c, gp(c, 0x530));
            if !restart {
                c.cycles += 3;
                c.g[V0][0] = 0x12;
                let other = c.g[S0][0] != c.g[V0][0];
                frame.restore(c, S0, 0x30);
                if other {
                    early = true;
                } else {
                    c.cycles += 1;
                    c.g[V1][0] = lw_3(c, gp(c, 0x530));
                }
            }
        }
        if !early {
            c.cycles += 3;
            c.g[V0][0] = 1;
            if c.g[V1][0] != c.g[V0][0] {
                sw_2(c, p_7(c.g[S2][0] as u32, 0x530), 0);
            } else {
                c.cycles += 7;
                c.g[V0][0] = lw_3(c, p_7(c.g[S2][0] as u32, 0x5558));
                c.g[A1][0] = 8;
                c.g[A0][0] = Addr(p_7(c.g[V0][0] as u32, 0x18)).lh(c);
                c.g[V1][0] = lw_3(c, p_7(c.g[V0][0] as u32, 0x1c));
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, c.g[V1][0] as u32, 0x0028_f3a8)?;
                sw_2(c, p_7(c.g[S2][0] as u32, 0x530), 0);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SSXAudioSystem_PlayCharSelectMusic(self)` (0x28fc58): start the
/// character-select music on the sound bank's channel 0x12d and set its
/// volume from the current rider.
pub fn ssx_audio_system_play_char_select_music(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 41;
        let frame = Frame::open(c, 0x60);
        c.g[A1][0] = 1;
        frame.save(c, S1, 0x40);
        c.g[A2][0] = 0;
        frame.save(c, S2, 0x30);
        c.g[S1][0] = sx(0x004a_0000);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        c.g[S3][0] = sx(r32(c, S2).wrapping_add(0x118));
        frame.save(c, S4, 0x10);
        c.g[S1][0] = sx(0x004a_3598);
        frame.save_ra(c, 0);
        c.g[S4][0] = Addr(p_8(r32(c, S2), 0x5558)).lw(c);
        c.g[S0][0] = Addr(p_8(r32(c, S4), 0x10)).lh(c);
        c.g[S0][0] = sx(p_8(r32(c, S3), r32(c, S0)));
        call_ptr(c, 0x0028_7968, 0x0028fc9c)?;
        c.g[A3][0] = c.g[V0][0];
        c.g[T0][0] = 1;
        c.g[V0][0] = Addr(p_8(r32(c, S4), 0x14)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        let play = r32(c, V0);
        c.g[A2][0] = 0xc;
        call_ptr(c, play, 0x0028fcb8)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 0x12d;
        call_ptr(c, 0x002b_49c0, 0x0028fcc4)?;
        c.g[V1][0] = Addr(p_8(r32(c, S2), 0x5558)).lw(c);
        c.g[A1][0] = Addr(p_8(r32(c, S2), 0x6290)).lw(c);
        c.g[A0][0] = Addr(p_8(r32(c, V1), 0x20)).lh(c);
        c.g[V0][0] = Addr(p_8(r32(c, V1), 0x24)).lw(c);
        let set_vol = r32(c, V0);
        c.g[A0][0] = sx(p_8(r32(c, S3), r32(c, A0)));
        call_ptr(c, set_vol, 0x0028fcdc)?;
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

/// `void SSXAudioSystem_OnSongChanged(self)` (0x28f478): play the new song,
/// and if its gain is audible, push the song title to the active music state.
pub fn ssx_audio_system_on_song_changed(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        'outer: {
            'inner: {
                c.cycles += 15;
                frame.save(c, S0, 0x10);
                c.g[S0][0] = c.g[A0][0];
                frame.save_ra(c, 0);
                c.g[RA][0] = sx(0x0028_f490);
                c.g[A0][0] = sx(at_4(c.g[S0][0], 0x118));
                callee::MUSIC_PLAYER_PLAY_SONG.call(c, 0x0028_f490)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 1;
                c.g[RA][0] = sx(0x0028_f4a0);
                c.g[A2][0] = 0;
                callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN.call(c, 0x0028_f4a0)?;
                c.mtc1(0, 1);
                c.c_lt_s(1, 0);
                let t = !c.fcc();
                c.g[V0][0] = lw(c, gp_2(c, -0x848));
                if t {
                    break 'inner;
                }
                c.cycles += 3;
                c.g[V0][0] = lw(c, at_4(c.g[V0][0], 0x84));
                let t = c.g[V0][0] == 0;
                frame.restore(c, S0, 0x10);
                if t {
                    break 'outer;
                }
                c.cycles += 3;
                c.g[A0][0] = lw(c, at_4(c.g[V0][0], 0x48));
                let t = c.g[A0][0] == 0;
                c.g[S0][0] = sx(at_4(c.g[A0][0], 0x18));
                if t {
                    break 'inner;
                }
                c.cycles += 2;
                if c.g[S0][0] == 0 {
                    frame.restore(c, S0, 0x10);
                    break 'outer;
                }
                c.cycles += 9;
                c.g[A0][0] = sx(0x004a_3660);
                c.g[RA][0] = sx(0x0028_f4e0);
                call_ptr(c, 0x0031_7670, 0x0028_f4e0)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0028_f4ec);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_f9d8, 0x0028_f4ec)?;
                c.g[A3][0] = c.g[V0][0];
                let t = c.g[A3][0] == 0;
                c.g[A1][0] = 5;
                if !t {
                    c.cycles += 6;
                    c.g[V1][0] = lw(c, at_4(c.g[A3][0], 8));
                    c.g[A2][0] = 0;
                    c.g[A0][0] = lh(c, at_4(c.g[V1][0], 0xc0));
                    c.g[V0][0] = lw(c, at_4(c.g[V1][0], 0xc4));
                    let target = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0028_f510);
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0028_f510)?;
                }
                break 'inner;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
            break 'outer;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `SSXAudioSystem_OnMusicSetEventState` (0x28f3c8): set the bank monitor's
// event state and, when the song tag is no longer -1 and the code is one of
// 10/11/18, optionally issue the monitor's `v3(8)` call and clear `+0x530`.
pub fn ssx_audio_system_on_music_set_event_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x118));
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x002b3bc0, 0x0028_f3f4)?;
        c.g[S3][0] = c.g[V0][0];
        call_ptr(c, 0x002b49e0, 0x0028_f400)?;
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        let tagged = c.g[V0][0] != c.g[V1][0];
        c.g[V0][0] = c.g[S3][0];
        if tagged {
            c.g[V0][0] = c.g[S3][0];
        } else {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xa));
            c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
            let interesting = c.g[V0][0] != 0;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x530));
            let mut act = true;
            if !interesting {
                c.cycles += 3;
                c.g[V0][0] = 0x12;
                let other = c.g[S0][0] != c.g[V0][0];
                c.g[V0][0] = c.g[S3][0];
                if other {
                    act = false;
                } else {
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x530));
                }
            }
            if act {
                c.cycles += 3;
                c.g[V0][0] = 0x1;
                if c.g[V1][0] != c.g[V0][0] {
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x530), 0u64);
                } else {
                    c.cycles += 7;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x5558));
                    c.g[A1][0] = 0x8;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, target, 0x0028_f450)?;
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x530), 0u64);
                }
                c.cycles += 1;
                c.g[V0][0] = c.g[S3][0];
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

/// `void SSXAudioSystem_PlayPeakTransitionMusic(self)` (0x28cd48): start the
/// peak transition music on the music channel and set its volume.
pub fn ssx_audio_system_play_peak_transition_music(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 44;
        let frame = Frame::open(c, 0x60);
        c.g[A1][0] = 0x1;
        frame.save(c, S1, 0x40);
        c.g[A2][0] = 0;
        frame.save(c, S2, 0x30);
        c.g[S1][0] = 0x4a0000;
        frame.save(c, S3, 0x20);
        let this = c.g[A0][0];
        c.g[S2][0] = this;
        c.g[S3][0] = sx((this as u32).wrapping_add(0x118));
        frame.save(c, S0, 0x50);
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3628));
        frame.save(c, S4, 0x10);
        c.g[S4][0] = Addr(p_9(this as u32, 0x5558)).lw(c);
        c.g[S0][0] = Addr(p_9(c.g[S4][0] as u32, 0x10)).lh(c);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[S0][0] as u32)));
        call_ptr(c, 0x0028_7968, 0x0028_cd8c)?;
        c.g[A3][0] = c.g[V0][0];
        c.g[T0][0] = 0x1;
        c.g[V0][0] = Addr(p_9(c.g[S4][0] as u32, 0x14)).lw(c);
        c.g[A2][0] = 0xc;
        c.g[A0][0] = c.g[S0][0];
        let target = c.g[V0][0] as u32;
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, target, 0x0028_cda8)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 0x191;
        call_ptr(c, 0x002b_49c0, 0x0028_cdb4)?;
        c.g[V1][0] = Addr(p_9(c.g[S2][0] as u32, 0x5558)).lw(c);
        c.g[A1][0] = 0x1;
        c.g[A0][0] = Addr(p_9(c.g[V1][0] as u32, 0x20)).lh(c);
        c.g[V0][0] = Addr(p_9(c.g[V1][0] as u32, 0x24)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0028_cdcc)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 0x7f;
        call_ptr(c, 0x002b_3c28, 0x0028_cdd8)?;
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

// ---------------------------------------------------------------------------
// `SSXAudioSystem_OnReplayBegin` (0x0028f200).
//
// `void SSXAudioSystem::OnReplayBegin(self)`: start the replay music unless it
// is already playing.

pub fn ssx_audio_system_on_replay_begin(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        let this = c.g[A0][0] as u32;
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(this);
        let mon = this.wrapping_add(0x118);
        c.g[S1][0] = sx(mon);
        sw_3(c, this, 0x6294, 0);
        c.g[V0][0] = lw_4(c, this, 0x530);
        let t = c.g[V0][0] == 0;
        sw_3(c, this, 0x6298, 0);
        if !t {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x002b_49e0, 0x0028_f230)?;
            c.g[V1][0] = 0xc9;
            let not_playing = c.g[V0][0] != c.g[V1][0];
            c.g[V0][0] = 1;
            if not_playing {
                c.cycles += 1;
                sw_3(c, this, 0x6298, c.g[V0][0]);
                c.cycles += 5;
                frame.restore(c, 16, 0x20);
                frame.restore(c, 17, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
        c.cycles += 4;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0028_d960, 0x0028_f244)?;
        let t = c.g[V0][0] == 0;
        c.g[V0][0] = 1;
        if !t {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x002b_49e0, 0x0028_f254)?;
            c.g[V1][0] = 0xc9;
            if c.g[V0][0] != c.g[V1][0] {
                c.g[V0][0] = lw_4(c, this, 0x118);
            } else {
                c.cycles += 3;
                c.g[V0][0] = lw_4(c, this, 0x62ac);
                sw_3(c, this, 0x508, c.g[V0][0]);
                c.g[V0][0] = lw_4(c, this, 0x118);
            }
            c.cycles += 13;
            c.g[V1][0] = 0x4a0000;
            c.ldl(6, 0x4a364f);
            c.ldr(6, 0x4a3648);
            c.g[A1][0] = 0xa;
            c.g[A0][0] = lw_4(c, c.g[V0][0] as u32, 0);
            c.g[A3][0] = 0;
            c.g[T0][0] = 3;
            c.g[T1][0] = 1;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
            c.g[T2][0] = 0;
            c.g[T3][0] = 0;
            call_ptr(c, 0x002a_dca0, 0x0028_f29c)?;
            c.g[V0][0] = 1;
        }
        c.cycles += 2;
        sw_3(c, this, 0x6294, c.g[V0][0]);
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SndBank_SetMemory(this, size, extra, tag)` (0x28b588): store the flags,
/// try to allocate the `extra` region from the stream allocator and record
/// the sized allocation.
pub fn snd_bank_set_memory(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let this = abi::arg(c, 0);
        let size = abi::arg(c, 1);
        let extra = abi::arg(c, 2);
        let tag = abi::arg(c, 3);
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = abi::arg64(c, 2);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = abi::arg64(c, 1);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0);
        let nonzero = c.g[S1][0] != 0;
        c.g[S3][0] = abi::arg64(c, 3);

        if !nonzero {
            c.cycles += 2;
            if c.g[S0][0] == 0 {
                frame.restore(c, 16, 0x40);
                c.cycles += 6;
                frame.restore(c, 17, 0x30);
                frame.restore(c, 18, 0x20);
                frame.restore(c, 19, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }

        c.cycles += 3;
        c.g[V0][0] = 1;
        let no_extra = c.g[S0][0] == 0;
        Addr(this).off(8).sw(c, c.g[V0][0]);
        if !no_extra {
            c.cycles += 12;
            c.g[V1][0] = Addr(gp(c, 0x524)).lw(c);
            c.g[A1][0] = c.g[S0][0];
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1d8).lw(c);
            c.g[A2][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x20).lh(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x24).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0028_b5ec)?;
            c.g[A0][0] = c.g[V0][0];
            let failed = (c.g[A0][0] as i64) < 0;
            c.g[V0][0] = 2;
            if !failed {
                c.cycles += 3;
                Addr(this).off(0xc).sw(c, c.g[A0][0]);
                Addr(this).off(8).sw(c, c.g[V0][0]);
            } else {
                c.cycles += 4;
                c.g[V0][0] = sx(0xffff_fffd_u32);
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.g[V0][0] = c.g[A0][0] ^ c.g[V0][0];
                if c.g[V0][0] == 0 {
                    c.g[S1][0] = c.g[V1][0];
                }
            }
        }

        c.cycles += 8;
        Addr(this).off(0x14).sw(c, c.g[S1][0]);
        c.g[A0][0] = 0x0048_0000;
        c.g[A0][0] = sx(0x0048_0000u32.wrapping_add(0x2998));
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S3][0];
        callee::MEM_ALLOC_NAMED_128.call(c, 0x0028_b62c)?;
        Addr(this).off(0x10).sw(c, c.g[V0][0]);
        frame.restore(c, 16, 0x40);
        c.cycles += 6;
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem_SelectRandomSong(self)` (0x0028d488): pick a random song
/// index in `[0, tracks)`, store it and make the music player switch to it.
pub fn ssx_audio_system_select_random_song(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = 0x7fff;
        frame.save(c, S1, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[S2][0];
        let player = lw_5(c, this, 0x118);
        c.g[S1][0] = lw_5(c, this, 0x504);
        c.g[S3][0] = lw_5(c, this, 0x62a4);
        c.g[V1][0] = player;
        c.g[V0][0] = lw_5(c, player, 0);
        c.g[A0][0] = lw_5(c, c.g[V0][0], 0x1d8);
        call_ptr(c, 0x002a_df60, 0x0028_d4c0)?;
        c.g[V0][0] = c.g[V0][0] & 0x7fff;

        // Random fraction of the (scaled) track count.
        c.cycles += 14;
        c.mult(0, V0, V0, S1);
        c.g[V1][0] = lw_5(c, this, 0x118);
        c.divu(0, V0, S0);
        c.g[V0][0] = c.lo[0];
        sw_4(c, this, 0x508, c.g[V0][0]);
        c.g[V0][0] = lw_5(c, c.g[V1][0], 0);
        c.g[A0][0] = lw_5(c, c.g[V0][0], 0x1d8);
        call_ptr(c, 0x002a_df60, 0x0028_d4ec)?;
        c.g[V1][0] = 0xa;
        c.g[V0][0] = c.g[V0][0] & 0x7fff;
        c.mult(0, V0, V0, V1);
        c.g[A0][0] = 0x7fff;

        // Skip up to ten songs to find a playable one.
        c.cycles += 5;
        c.divu(0, V0, S0);
        c.g[V1][0] = c.lo[0];
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        let empty = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[V0][0];
        if !empty {
            c.cycles += 2;
            c.g[A0][0] = this;
            loop {
                c.cycles += 8;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
                Callee {
                    addr: 0x0028_d8a0,
                    f: ee::s07::SSXAudioSystem_GetTrackMusicGroup,
                }
                .call(c, 0x0028_d528)?;
                c.g[A2][0] = c.g[V0][0];
                c.g[A0][0] = sx((this as u32).wrapping_add(0x118));
                c.g[A1][0] = 1;
                call_ptr(c, 0x002b_42b8, 0x0028_d538)?;
                let more = c.g[S0][0] != 0;
                c.g[A0][0] = this;
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0028_d520);
                }
            }
        }

        // Restart the music player on the new selection.
        c.cycles += 3;
        c.g[V0][0] = lw_5(c, this, 0x508);
        let same = c.g[S3][0] == c.g[V0][0];
        c.g[S0][0] = sx((this as u32).wrapping_add(0x118));
        if !same {
            c.cycles += 1;
        } else {
            c.cycles += 9;
            c.g[A0][0] = this;
            Callee {
                addr: 0x0028_d8a0,
                f: ee::s07::SSXAudioSystem_GetTrackMusicGroup,
            }
            .call(c, 0x0028_d554)?;
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x002b_42b8, 0x0028_d564)?;
            c.g[V0][0] = lw_5(c, this, 0x508);
            if c.g[S3][0] == c.g[V0][0] {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                call_ptr(c, 0x002b_42b8, 0x0028_d580)?;
                c.cycles += 1;
            }
        }
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
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
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
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
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::RA as RET;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::put;
    pub(crate) use crate::game::ai::rider::tests::hh;
    pub(crate) use crate::game::ai::rider::tests::slot_2 as slot;
    pub(crate) use crate::game::be::bestate2::tests::belib;
    pub(crate) use crate::game::be::bestate2::tests::w;
    pub(crate) use crate::game::bx::bxstring2::tests::W;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::gpf;
    pub(crate) use crate::game::dirtysock::tags2::tests::be_library_6 as be_library_3;
    pub(crate) use crate::game::dirtysock::tags2::tests::put_str8;
    pub(crate) use crate::game::dirtysock::tags2::tests::rng_2;
    pub(crate) use crate::game::dirtysock::tags2::tests::zero;
    pub(crate) use crate::game::dirtysock::tags2::tests::SCRATCH;
    pub(crate) use crate::game::fe::debugfe::tests::w as w_2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_3 as alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_8 as be_library_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::eafile_pools;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::fe::feasyncfile::tests::fake_small;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_5 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::gpat;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_8 as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_2 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_3 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_4 as pool_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_9 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::put_str_3 as put_str;
    pub(crate) use crate::game::fe::feasyncfile::tests::put_str_4 as put_str_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_7 as run;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_4 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_8 as slot_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::w32;
    pub(crate) use crate::game::fe::feasyncfile::tests::wb;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_5 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_4 as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_9 as BE_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_3 as OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDER;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDERS_2 as RIDERS;
    pub(crate) use crate::game::fe::feasyncfile::tests::RMAN;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_2 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_7 as WORLD_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_3 as W_2;
    pub(crate) use crate::game::fe::festateoptions::tests::AUX;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY_PTR;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::gp_at;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::heap_setup;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_4;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS_3;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_4;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::globals_3 as globals_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::load as load_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::BE_3;
    pub(crate) use crate::game::wscript::wscriptman::tests::GAME;
    pub(crate) use crate::game::wscript::wscriptman::tests::WORLD_3 as WORLD_2;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn snd_bank_link_matches() {
        check(ssx3_ee::s07::f_0028b6c8, snd_bank_link, 300, &|c, seed| {
            let mut r = Rng(0x1111_000a ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            let other = DATA + 0x100;
            let prev = if seed % 2 == 0 { 0 } else { DATA + 0x200 };
            c.bus.write32(other + 0x18, prev);
            c.bus.write32(other + 0x1c, DATA + 0x300);
            c.bus
                .write32(other + 0x8, if seed % 3 == 0 { 0 } else { r.next_u32() });
            c.bus.write32(other + 0x10, r.next_u32());
            c.bus.write32(other + 0x14, r.next_u32());
            c.bus.write32(other + 0xc, r.next_u32());
            for i in 0..8 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
            c.bus.write32(DATA + 0x200 + 0x18, r.next_u32());
            c.bus.write32(DATA + 0x300 + 0x18, r.next_u32());
            c.bus.write32(DATA + 0x300 + 0x1c, r.next_u32());
        });
    }

    #[test]
    fn snd_bank_unlink_matches() {
        check(
            ssx3_ee::s07::f_0028b730,
            snd_bank_unlink,
            300,
            &|c, seed| {
                let mut r = Rng(0x1111_000b ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                // 0: no previous; 1: prev == next; 2: prev != next.
                let (prev, next, write_next) = match seed % 3 {
                    0 => (0, DATA + 0x200, false),
                    1 => (DATA + 0x100, DATA + 0x100, false),
                    _ => (DATA + 0x100, DATA + 0x200, true),
                };
                c.bus.write32(DATA + 0x18, prev);
                c.bus.write32(DATA + 0x1c, next);
                for off in [0x8u32, 0xc, 0x10, 0x14] {
                    c.bus.write32(DATA + off, r.next_u32());
                }
                if prev != 0 {
                    c.bus.write32(prev + 0x18, r.next_u32());
                    c.bus.write32(prev + 0x1c, r.next_u32());
                }
                if write_next {
                    c.bus.write32(DATA + 0x200 + 0x18, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn snd_bank_unlink_matches_2() {
        check(
            ssx3_ee::s07::f_0028b730,
            snd_bank_unlink_2,
            300,
            &|c, seed| {
                let mut r = Rng(0x28b7 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let (next, prev) = match seed % 3 {
                    0 => (0u32, r.next_u32()),
                    1 => {
                        let v = DATA + 0x400;
                        (v, v)
                    }
                    _ => (DATA + 0x400, DATA + 0x500),
                };
                c.bus.write32(DATA + 0x18, next);
                c.bus.write32(DATA + 0x1c, prev);
                c.bus.write32(DATA + 0x418, r.next_u32());
                c.bus.write32(DATA + 0x41c, r.next_u32());
                c.bus.write32(DATA + 0x518, r.next_u32());
            },
        );
    }

    pub(crate) const SEEDS: u32 = 120;

    /// A per-function RNG (the address keeps the functions' seeds apart).
    pub(crate) fn rng(tag: u32, seed: u32) -> Rng {
        Rng(0x9e37_79b9_7f4a_7c15 ^ ((tag as u64) << 20) ^ seed as u64)
    }

    /// A 64-bit value whose low half is small (indices, ids) and whose upper
    /// half is junk on odd seeds, so 64-bit compares are exercised.
    pub(crate) fn mixed(r: &mut Rng, seed: u32, low: u32) -> u64 {
        if seed % 2 == 0 {
            low as u64
        } else {
            ((r.next_u32() as u64) << 32) | low as u64
        }
    }

    /// An id inside or just outside `[base, base + len)`, with junk in the
    /// upper half on odd seeds.
    pub(crate) fn track_id(c: &mut Cpu, r: &mut Rng, seed: u32, base: u32, len: u32) {
        let id = match seed % 3 {
            0 => base.wrapping_add(seed % len),
            1 => base.wrapping_sub(1 + seed % 4),
            _ => r.next_u32() % 0x40,
        };
        c.g[A1][0] = mixed(r, seed, id);
    }

    #[test]
    fn ssx_audio_system_is_track_id_group0_matches() {
        check(
            ee::s08::f_002a40e0__generated,
            ssx_audio_system_is_track_id_group0,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x002a_40e0, seed);
                scramble(c, &mut r);
                track_id(c, &mut r, seed, 0, 5);
            },
        );
    }

    #[test]
    fn ssx_audio_system_is_track_id_group4_matches() {
        check(
            ee::s08::f_002a4030__generated,
            ssx_audio_system_is_track_id_group4,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x002a_4030, seed);
                scramble(c, &mut r);
                track_id(c, &mut r, seed, 0xe, 3);
            },
        );
    }

    #[test]
    fn ssx_audio_system_is_track_id_group1_matches() {
        check(
            ee::s08::f_002a4158__generated,
            ssx_audio_system_is_track_id_group1,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x002a_4158, seed);
                scramble(c, &mut r);
                track_id(c, &mut r, seed, 5, 3);
            },
        );
    }

    #[test]
    fn ssx_audio_system_is_track_id_group2_matches() {
        check(
            ee::s08::f_002a41c8__generated,
            ssx_audio_system_is_track_id_group2,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x002a_41c8, seed);
                scramble(c, &mut r);
                track_id(c, &mut r, seed, 8, 3);
            },
        );
    }

    #[test]
    fn ssx_audio_system_is_track_id_group3_matches() {
        check(
            ee::s08::f_002a4238__generated,
            ssx_audio_system_is_track_id_group3,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x002a_4238, seed);
                scramble(c, &mut r);
                track_id(c, &mut r, seed, 0xb, 3);
            },
        );
    }

    #[test]
    fn ssx_audio_system_set_music_state_matches() {
        check(
            ee::s07::f_0028def0__generated,
            ssx_audio_system_set_music_state,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0028_def0, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                let old = r.next_u32();
                c.bus.write32(DATA + 0x629c, old);
                c.g[A1][0] = match seed % 3 {
                    0 => old as u64,
                    _ => {
                        let v = r.next_u32();
                        mixed(&mut r, seed, v)
                    }
                };
            },
        );
    }

    #[test]
    fn music_player_clear_loops_active_matches() {
        check(
            ee::s08::f_002b4708__generated,
            music_player_clear_loops_active,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x002b_4708, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                if seed % 3 == 0 {
                    c.bus.write32(DATA + 0x408, 0);
                } else {
                    c.bus.write32(DATA + 0x408, DATA + 0x2000);
                    c.bus.write32(DATA + 0x2000 + 0x4c, r.next_u32());
                }
            },
        );
    }

    // ---------------------------------------------------------------- sound

    /// A sound bank manager with four zeroed 0x60-byte banks. The banks are
    /// arranged so `SndBank_Unload`/`_UnloadMemory`/`_Release` take their
    /// short paths and the frees see a null payload.
    pub(crate) fn snd_bank_man_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        let manager = DATA + 0x4000;
        let banks = DATA + 0x5000;
        c.bus.write32(manager + 0xacc, banks);
        for i in 0..4u32 {
            let bank = banks + i * 0x60;
            c.bus.write32(bank, (seed % 2) * 2);
            c.bus.write32(bank + 0x4, u32::MAX);
            c.bus.write32(bank + 0x8, (seed % 2) * 3);
            c.bus.write32(bank + 0x10, 0);
            c.bus.write32(bank + 0x14, r.next_u32());
            c.bus.write32(bank + 0x18, 0);
        }
        c.g[A0][0] = sx(manager);
        c.g[A1][0] = (seed % 4) as u64;
    }

    #[test]
    fn snd_bank_man_unload_bank_matches() {
        check_lohi(
            ssx3_ee::s07::f_0028bce8__generated,
            snd_bank_man_unload_bank,
            100,
            &|c, seed| snd_bank_man_setup(c, seed, 0x3b),
        );
    }

    #[test]
    fn snd_bank_man_free_bank_memory_matches() {
        check_lohi(
            ssx3_ee::s07::f_0028bde0__generated,
            snd_bank_man_free_bank_memory,
            100,
            &|c, seed| snd_bank_man_setup(c, seed, 0x3c),
        );
    }

    #[test]
    fn snd_bank_man_unload_memory_bank_matches() {
        check_lohi(
            ssx3_ee::s07::f_0028be90__generated,
            snd_bank_man_unload_memory_bank,
            100,
            &|c, seed| snd_bank_man_setup(c, seed, 0x3d),
        );
    }

    #[test]
    fn snd_bank_man_release_bank_matches() {
        check_lohi(
            ssx3_ee::s07::f_0028beb8__generated,
            snd_bank_man_release_bank,
            100,
            &|c, seed| snd_bank_man_setup(c, seed, 0x3e),
        );
    }

    #[test]
    fn snd_bank_man_get_bank_buffer_size_matches() {
        check_lohi(
            ssx3_ee::s07::f_0028bf10__generated,
            snd_bank_man_get_bank_buffer_size,
            100,
            &|c, seed| snd_bank_man_setup(c, seed, 0x3f),
        );
    }

    #[test]
    fn ssx_audio_system_is_game_music_enabled_matches() {
        check(
            ssx3_ee::s07::f_0028d960__generated,
            ssx_audio_system_is_game_music_enabled,
            300,
            &|c, seed| {
                let mut r = rng_2(seed, 0x40);
                scramble(c, &mut r);
                let state = match seed % 6 {
                    0 => 0,
                    1 => 1,
                    2 => 3,
                    3 => 2,
                    4 => r.next_u32(),
                    _ => 4,
                };
                c.bus.write32(DATA + 0x608c, state);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn snd_bank_man_get_bank_buffer_matches() {
        check(
            ee::s07::f_0028bee0__generated,
            snd_bank_man_get_bank_buffer,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28bee0, 3);
                let banks = SCRATCH + 0x1000;
                let index = seed % 4;
                let bank = banks + 0x60 * index;
                // A loaded bank (`+8` non-zero) just hands back its buffer at
                // `+0x10`; an empty one allocates through the sound heap, whose
                // arena must be set up first (see `MEM_AllocNamed128`'s test).
                let loaded = seed % 2 == 0;
                if !loaded {
                    heap_init(c);
                }
                c.g[A0][0] = SCRATCH as u64;
                c.g[A1][0] = index as u64;
                // `a2` becomes the allocation size and `a3` its flags (both are
                // forwarded to `MEM_AllocNamed128`), so keep the size modest.
                c.g[A2][0] = (0x40 + seed % 0x100) as u64;
                c.g[A3][0] = (r.next_u32() & 1) as u64;
                c.bus.write32(SCRATCH + 0xacc, banks);
                c.bus.write32(bank + 8, loaded as u32);
                c.bus.write32(bank + 0x10, r.next_u32());
            },
        );
    }

    /// Set up the sound heap the allocator inside `SndBank_GetBuffer` uses.
    pub(crate) fn heap_init(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        c.g[GP][0] = crate::difftest::GAME_GP as u64;
        load(
            c,
            0x0031_ff60,
            ssx3_ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
        );
    }

    /// Call a setup helper and check that it returned.
    pub(crate) fn load(c: &mut Cpu, addr: u32, f: fn(&mut Cpu), args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(
            c.pc,
            crate::difftest::RA,
            "setup call {addr:#x} did not return"
        );
        c.g[V0][0]
    }

    #[test]
    fn snd_bank_man_set_bank_memory_matches() {
        check(
            ssx3_ee::s07::f_0028bda8__generated,
            snd_bank_man_set_bank_memory,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28bda8, 0);
                let bank_man = DATA + 0x100;
                c.bus.write32(bank_man + 0xacc, DATA + 0x2000);
                fill(c, &mut r, DATA + 0x2000, 0x100);
                c.g[A0][0] = sx(bank_man);
                c.g[A1][0] = (r.next_u32() % 4) as u64;
                // Zero length and pointer: SndBank_SetMemory takes its early exit.
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                c.g[T0][0] = 0;
            },
        );
    }

    #[test]
    fn audio_stop_fe_music_matches() {
        check(
            ssx3_ee::s07::f_0028f520__generated,
            audio_stop_fe_music,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28f520, 1);
                let sys = DATA + 0x100;
                fill(c, &mut r, sys, 0x100);
                // +0x608c: 0/1/2 "music enabled", 3 also enabled, else disabled.
                c.bus
                    .write32(sys + 0x608c, [0u32, 2, 3, 4][seed as usize % 4]);
                // No current song instance (+0x118 + 0x408): the player only
                // touches its own fields.
                c.bus.write32(sys + 0x520, 0);
                c.bus.write32(sys + 0x118, 0);
                c.g[A0][0] = sx(sys);
            },
        );
    }

    // ---------------------------------------------------------------- sound

    #[test]
    fn ssx_audio_system_end_script_music_matches() {
        check(
            ee::s07::f_0028e888__generated,
            ssx_audio_system_end_script_music,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x28e8_88, 0);
                let self_ = DATA + 0x400;
                zero(c, self_, 0x6300);
                c.g[A0][0] = sx(self_);
                c.bus
                    .write32(self_ + 0x6254, if seed % 2 == 0 { 0 } else { 1 });
                // HandleMusicEvent exits early when +0x627c is clear.
                c.bus.write32(self_ + 0x627c, 0);
            },
        );
    }

    #[test]
    fn ssx_audio_system_unload_loading_screen_bank_matches() {
        check(
            ee::s07::f_0028f730__generated,
            ssx_audio_system_unload_loading_screen_bank,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x28f7_30, 0);
                let self_ = DATA + 0x400;
                zero(c, self_, 0x6300);
                c.g[A0][0] = sx(self_);
                // Negative handle: StopLoadingScreenLoop returns at once.
                c.bus.write32(self_ + 0x62a8, 0xffff_ffff);
                c.bus.write32(self_ + 0x118, DATA + 0x600);
                c.bus.write32(DATA + 0x600, DATA + 0x700);
                zero(c, DATA + 0x700, 0x100);
            },
        );
    }

    /// The BE library singleton, so `BELibrary_GetInstance` never builds it.
    pub(crate) fn be_library(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x3_0000);
    }

    /// [`be_library`], plus the current phase interface (`gp-0x1efc` selects
    /// one of the three) whose slot-2 method is the fake callee.
    pub(crate) fn be_library_interface(c: &mut Cpu, r: &mut crate::difftest::Rng) {
        be_library(c);
        let index = (r.next_u32() % 8) as u32;
        c.bus.write32(GAME_GP.wrapping_sub(0x1efc), index);
        let lib = ptr(r);
        c.bus.write32(0x0053_08a8 + 4 * index, lib);
        let vt = vtable(c, r, &[8]);
        c.bus.write32(lib + 4, vt);
    }

    #[test]
    fn ssx_audio_system_is_track_group5_matches() {
        covered(
            ee::s08::f_002a3f90__generated,
            ssx_audio_system_is_track_group5,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_3f90, 1);
                be_library_interface(c, &mut r);
                // `BENewRaceInt_GetTrackInfo` indexes the 0x64-byte station table
                // by the live track index.
                let index = (r.next_u32() % 4) as u32;
                c.bus.write32(0x0053_5c08, index);
                let id = match seed % 4 {
                    0 => 0x11,
                    1 => 0x15,
                    2 => 0x10,
                    _ => r.next_u32() % 0x20,
                };
                c.bus.write32(0x0043_d950 + 0x64 * index, id);
                c.g[A0][0] = sx(DATA + 0x1000);
            },
        );
    }

    #[test]
    fn snd_bank_is_loaded_with_name_matches() {
        check(
            ee::s07::f_0028b830__generated,
            snd_bank_is_loaded_with_name,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28b8_30, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x300);
                c.bus.write32(
                    this,
                    match seed % 4 {
                        0 => 0,
                        1 => 2,
                        _ => 1,
                    },
                );
                let name = if seed % 2 == 0 {
                    &b"bank_a\0"[..]
                } else {
                    &b"bank_b\0"[..]
                };
                put_str8(c, DATA + 0x300, name);
                let local = if seed % 4 == 1 {
                    &b"bank_a\0"[..]
                } else {
                    &b"other\0"[..]
                };
                put_str8(c, this + 0x20, local);
                c.bus.write32(this + 0x24, r.next_u32());
            },
        );
    }

    /// The BE library singleton: a zeroed dummy so `cBE_getBE` does not build
    /// the real one from disc.
    pub(crate) fn be_library_2(c: &mut Cpu) {
        c.bus.write32(gp_at(-0x1f04), DATA + 0x3f000);
    }

    #[test]
    fn ssx_audio_system_is_race_type6_to11_matches() {
        race_type_test(
            ee::s08::f_002a42c8__generated,
            ssx_audio_system_is_race_type6_to11,
            0x2a42_c8,
        );
    }

    /// The three race-type queries: the BE library dummy, an empty BE table
    /// (the interface call lands in the fake hook) and every race type.
    pub(crate) fn race_type_test(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, SEEDS_3, &move |c, seed| {
            let _ = machine(c, seed, salt, 0);
            be_library_2(c);
            clear(c, 0x0053_08a8, 0x20);
            c.bus.write8(0x0053_5c12, (seed % 20) as u8);
        });
    }

    #[test]
    fn snd_bank_unload_memory_matches() {
        check(
            ee::s07::f_0028b7e0__generated,
            snd_bank_unload_memory,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28b7_e0, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x20);
                c.g[A0][0] = sx(this);
                let bank = match seed % 3 {
                    0 => 0xffff_ffff,
                    1 => 0,
                    _ => seed % 4,
                };
                c.bus.write32(this + 4, bank);
                c.bus.write32(this + 0x10, r.next_u32());
                c.bus.write32(this + 0x14, r.next_u32());
            },
        );
    }

    #[test]
    fn snd_bank_load_from_memory_matches() {
        check(
            ssx3_ee::s07::f_0028b788__generated,
            snd_bank_load_from_memory,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x28b788, 2);
                let bank = DATA + 0x400;
                c.bus.write32(bank, seed);
                c.bus.write32(bank + 4, 0xffff_ffff);
                c.g[A0][0] = bank as u64;
                c.g[A1][0] = (DATA + 0x500) as u64;
                c.g[A2][0] = (seed * 0x10 + 4) as u64;
            },
        );
    }

    // ------------------------------------------------------------------
    // Sound banks.

    #[test]
    fn snd_bank_man_init_bank_request_matches() {
        covered(
            ee::s07::f_0028bf38__generated,
            snd_bank_man_init_bank_request,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 22, 3);
                c.g[A0][0] = W as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = if seed % 2 == 0 { 1 } else { 0 };
                c.g[A3][0] = (AUX + 0x400) as u64;
                c.g[T0][0] = (r.next_u32() % 4) as u64;
                c.g[T1][0] = (r.next_u32() % 4) as u64;
                c.g[T2][0] = (r.next_u32() % 4) as u64;
                put(c, W + 0xacc, AUX + 0x800);
                // `SndBankMan_InitLoadRequest` walks
                // gp+0x524 -> +0x1d8 -> [0] -> [0] and calls the function
                // pointer at +0x3c (an unregistered address, so the test
                // machine's fake records the call).
                gpf(c, 0x524, AUX + 0x900);
                put(c, AUX + 0x900 + 0x1d8, AUX + 0x920);
                put(c, AUX + 0x920, AUX + 0x940);
                put(c, AUX + 0x940, AUX + 0x960);
                c.bus.write16(AUX + 0x960 + 0x38, 0);
                put(c, AUX + 0x960 + 0x3c, FAKE);
                c.bus.write8(AUX + 0x400, 0); // the strcpy source is empty
            },
        );
    }

    #[test]
    fn ssx_audio_system_set_selected_peak_matches() {
        check(
            ee::s07::f_0028f558__generated,
            ssx_audio_system_set_selected_peak,
            64,
            &|c, seed| {
                machine(c, seed, 0x0028_f558, 3);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 8) as u64;
                w(c, DATA + 0x6284, (seed % 8) as u32);
                // `RestartPeakMusic` reaches the music player's round-robin slot
                // picker, which spins until the occupancy mask names a free slot:
                // give it a two-slot bank whose second slot is free.
                w(c, DATA + 0x4fc, 2); // slot count
                w(c, DATA + 0x508, 0); // current
                c.bus.write64(DATA + 0x510, 2); // slot mask (slot 1 free)
            },
        );
    }

    #[test]
    fn snd_bank_release_matches() {
        check(
            ssx3_ee::s07::f_0028b878__generated,
            snd_bank_release,
            40,
            &|c, seed| {
                fakes::machine(c, seed, 0x28b878, 0);
                w_2(c, OBJ + 0x8, (seed % 3) as u32);
                w_2(c, OBJ + 0xc, seed);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    // ------------------------------------------------------------- audio/triggers

    #[test]
    fn ssx_audio_system_on_replay_end_matches() {
        check(
            ssx3_ee::s07::f_0028f2c0__generated,
            ssx_audio_system_on_replay_end,
            90,
            &|c, seed| {
                let mut r = setup(c, seed, 0x28f2c0);
                c.g[A0][0] = DATA as u64;
                let mode = seed % 3;
                c.bus.write32(DATA + 0x6294, u32::from(mode == 1));
                c.bus.write32(DATA + 0x6298, u32::from(mode == 2));
                if mode == 1 {
                    // A song player with no song: the timer walks are empty.
                    c.bus.write32(DATA + 0x118, DATA + 0x500);
                    c.bus.write32(DATA + 0x500 + 0x408, 0);
                }
                if mode == 2 {
                    let vt = vtable(c, &mut r, &[0x20]);
                    c.bus.write32(DATA + 0x5558, vt);
                }
            },
        );
    }

    #[test]
    fn ssx_audio_system_restart_peak_music_matches() {
        check(
            ee::s07::f_0028ef90__generated,
            ssx_audio_system_restart_peak_music,
            4,
            &|c, seed| {
                let _ = base(c, seed, 0x0028_ef90, 4);
                c.g[A0][0] = OBJ_2 as u64;
                w(c, OBJ_2 + 0x118, DATA + 0x3000);
                w(c, DATA + 0x3000, DATA + 0x3200);
                w(c, DATA + 0x3200 + 0x1d8, 0);
                // `RestartPeakMusic` reaches the music player's round-robin
                // slot picker (`MusicPlayer_SelectNextSong`, 0x2b42b8), which
                // spins until the occupancy mask names a slot. The bank
                // manager is inline at `self + 0x118`: slot count at +0x3e4,
                // current slot at +0x3f0, 64-bit mask at +0x3f8. A two-slot
                // bank with slot 1 occupied and current 0 exits after one
                // step (same fixture as oc90's SetSelectedPeak test).
                w(c, OBJ_2 + 0x4fc, 2); // slot count
                w(c, OBJ_2 + 0x508, 0); // current
                c.bus.write64(OBJ_2 + 0x510, 2); // occupancy mask (slot 1)
            },
        );
    }

    #[test]
    fn snd_bank_free_memory_matches() {
        check(
            ee::s07::f_0028b650__generated,
            snd_bank_free_memory,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x28_b650, 2);
                heap_setup(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                w(c, DATA + 0x2000 + 0x8, [0u32, 1, 2][(seed % 3) as usize]);
                w(c, DATA + 0x2000 + 0xc, 0x11);
                w(c, DATA + 0x2000 + 0x10, DATA + 0x2600);
                w(c, DATA + 0x2000 + 0x14, 0x22);
                w(c, GAME_GP + 0x524, DATA + 0x3000);
                w(c, DATA + 0x3000 + 0x1d8, DATA + 0x3200);
                w(c, DATA + 0x3200, DATA + 0x3400);
                slot(c, DATA + 0x3400, 0x28, 0, FAKE);
                c.pc = 0x0028_b650;
            },
        );
    }

    #[test]
    fn music_player_set_playlist_mask_matches() {
        check(
            ee::s08::f_002b4070__generated,
            music_player_set_playlist_mask,
            80,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x2b40, 3);
                let m = DATA;
                fill(c, &mut r, m + 0x3e0, 3);
                fill(c, &mut r, m + 0x920, 8);
                w(c, m + 0x3ec, (seed % 5) as u32);
                c.g[A0][0] = sx(m);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn snd_bank_man_init_load_request_matches() {
        check(
            ee::s07::f_0028b930__generated,
            snd_bank_man_init_load_request,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x28b930, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (seed % 2) as u64;
                c.g[A3][0] = sx(DATA + 0x500);
                put_str(c, DATA + 0x500, b"bank");
                let m = DATA + 0x1000;
                c.bus.write32(GAME_GP + 0x524, m);
                c.bus.write32(m + 0x1d8, DATA + 0x1010);
                c.bus.write32(DATA + 0x1010, DATA + 0x1020);
                c.bus.write32(DATA + 0x1020, DATA + 0x1030);
                c.bus.write16(DATA + 0x1030 + 0x38, 0);
                c.bus.write32(DATA + 0x1030 + 0x3c, FAKE);
            },
        );
    }

    #[test]
    fn ssx_audio_system_on_event_select_exit_matches() {
        check(
            ee::s07::f_0028f678__generated,
            ssx_audio_system_on_event_select_exit,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28f678, 3);
                be_library_3(c, &mut r);
                c.bus.write32(LOG_RESULTS, DATA + 0x2100);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 2) as u64;
                c.bus.write32(DATA + 0x6088, (seed % 3 == 0) as u32);
                // `+0x62a8` < 0 takes the short path; otherwise the loading-screen
                // loop runs (and its own guard is the same field).
                c.bus
                    .write32(DATA + 0x62a8, if seed % 4 == 0 { 0xffff_ffff } else { 0 });
                c.bus.write32(DATA + 0x118, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000, 0);
            },
        );
    }

    #[test]
    fn ssx_audio_system_update_music_tier_matches() {
        check(
            ee::s07::f_0028d5a0__generated,
            ssx_audio_system_update_music_tier,
            SEEDS_5,
            &|c, seed| {
                machine(c, seed, 0x28d5, 0);
                env(c);
                let this = DATA + 0x400;
                w32(c, this + 0x6284, if seed % 4 == 0 { 0x17 } else { 0 });
                w32(c, gpat(0x410), 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 3) as u64;
            },
        );
    }

    #[test]
    fn snd_bank_man_load_bank_async_matches() {
        check(
            ee::s07::f_0028bd10__generated,
            snd_bank_man_load_bank_async,
            SEEDS_6,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                w(c, this + 0xacc, DATA + 0x1000);
                for i in 0..8 {
                    w(c, DATA + 0x1000 + 4 * i, 0);
                }
                c.g[S3][0] = 0;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = (DATA + 0x2100) as u64;
                c.g[A3][0] = seed as u64;
                put_str_2(c, DATA + 0x2000, b"music");
                put_str_2(c, DATA + 0x2100, b"bank");
            }),
        );
    }

    #[test]
    fn ssx_audio_system_confirm_music_trigger_matches() {
        check(
            ee::s07::f_0028d740__generated,
            ssx_audio_system_confirm_music_trigger,
            SEEDS_6,
            &alloc_base_setup(|c, seed, r| {
                let this = DATA;
                let obj = DATA + 0x800;
                let vt = vtable(c, r, &[0x38]);
                w(c, obj + 0x6c0, vt);
                match seed % 3 {
                    0 => {
                        w(c, this + 0x6240, 7);
                        w(c, this + 0x62a0, 7);
                    }
                    1 => {
                        w(c, this + 0x6240, 7);
                        w(c, this + 0x62a0, 0xffff_ffff);
                    }
                    _ => {
                        w(c, this + 0x6240, 7);
                        w(c, this + 0x62a0, 9);
                        w(c, this + 0x623c, (seed & 2) as u32);
                    }
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = 7;
                c.g[A2][0] = obj as u64;
            }),
        );
    }

    #[test]
    fn ssx_audio_system_on_music_post_event_matches() {
        check(
            ssx3_ee::s07::f_0028f328__generated,
            ssx_audio_system_on_music_post_event,
            16,
            &|c, seed| {
                let _ = base_4(c, seed, 0x0028_f328);
                let audio = DATA + 0x100;
                let player = audio + 0x118;
                let song = DATA + 0x3000;
                let queue = DATA + 0x4000;
                clear_2(c, audio, 0x6000);
                clear_2(c, song, 0x80);
                clear_2(c, queue, 0x40);
                // A live song object whose tag is "none" (-1).
                w(c, player + 0x418, 1);
                w(c, player + 0x408, song);
                w(c, song + 0x58, 0xffff_ffff);
                w(c, audio + 0x5558, queue);
                w(c, queue + 0x1c, 0);
                let event = match seed % 5 {
                    0 => 10,
                    1 => 11,
                    2 => 18,
                    3 => 9,
                    _ => (seed % 32) as u32,
                };
                w(c, GAME_GP.wrapping_add(0x530), (seed % 3) as u32);
                c.g[A0][0] = sx(audio);
                c.g[A1][0] = event as u64;
            },
        );
    }

    pub(crate) const SYS: u32 = DATA + 0x18000;

    #[test]
    fn ssx_audio_system_play_char_select_music_matches() {
        check(
            ee::s07::f_0028fc58__generated,
            ssx_audio_system_play_char_select_music,
            8,
            &|c, seed| {
                let mut r = base(c, seed, 0x28fc58, 3);
                c.g[A0][0] = SYS as u64;
                w(c, SYS + 0x118, 0x1000);
                w(c, SYS + 0x5558, OBJ2);
                w(c, SYS + 0x6290, (seed % 4) as u32);
                w(c, OBJ2 + 0x10, (seed % 5) as u32);
                let vt0 = vtable(c, &mut r, &[0x20, 0x24]);
                w(c, OBJ2 + 0x14, vt0);
                let vt1 = vtable(c, &mut r, &[0x20, 0x24]);
                w(c, OBJ2 + 0x24, vt1);
            },
        );
    }

    // ---------------------------------------------------------- audio and race

    #[test]
    fn ssx_audio_system_play_peak_transition_music_matches() {
        check(
            ee::s07::f_0028cd48__generated,
            ssx_audio_system_play_peak_transition_music,
            6,
            &|c, seed| {
                let mut r = base(c, seed, 0x0028_cd48, 3);
                c.g[A0][0] = OBJ_3 as u64;
                let vt = vtable(c, &mut r, &[0x10, 0x20]);
                w(c, OBJ_3 + 0x5558, vt);
                w(c, OBJ_3 + 0x118, OBJ2);
                w(c, 0x004a_3628, 0);
            },
        );
    }

    #[test]
    fn ssx_audio_system_on_replay_begin_matches() {
        check(
            ee::s07::f_0028f200__generated,
            ssx_audio_system_on_replay_begin,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28f2, 6);
                elf_image(c);
                let this = DATA;
                c.bus.write32(this + 0x530, [0u32, 1][(seed % 2) as usize]);
                c.bus.write32(this + 0x118, DATA + 0x2000);
                c.bus.write32(this + 0x62ac, r.next_u32());
                c.bus.write32(this + 0x508, r.next_u32());
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn snd_bank_set_memory_matches() {
        check(
            ee::s07::f_0028b588__generated,
            snd_bank_set_memory,
            SEEDS_7,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28b5, 3);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = ((seed % 3) as u64) * 0x100;
                c.g[A2][0] = if seed % 2 == 0 {
                    0
                } else {
                    ((seed % 4) as u64) * 0x40
                };
                c.g[A3][0] = (DATA + 0x2000) as u64;
                let obj = DATA + 0x2400;
                c.bus.write32(GAME_GP + 0x524, obj);
                let vt = vtable(c, &mut r, &[0x20]);
                c.bus.write32(obj + 0x1d8, DATA + 0x2800);
                c.bus.write32(DATA + 0x2800, vt);
            },
        );
    }
}
