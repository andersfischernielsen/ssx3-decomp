//! `src/main/gamestate.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Frame;
use ssx3_ee as ee;
use crate::abi::r32;
use crate::call::Callee;
use crate::call::Flow;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::call::VSlot;
use crate::call::sp;
use ssx3_ee_rt::funcs::call_indirect;
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::X;
use crate::call;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::call::Unwound;
use crate::ps2f::FpuF32;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::forward;
pub(crate) use crate::game::dirtysock::tags2::gp_2 as gp;
pub(crate) use crate::game::movie::movieplayer::lw_at;
pub(crate) use crate::game::fe::festaterewards2::r;
pub(crate) use crate::game::dirtysock::tags2::gp_3 as gp_2;
pub(crate) use crate::game::fe::festaterewards2::Ops;
pub(crate) use crate::game::sound::bankmonitor::leave;
pub(crate) use crate::game::dirtysock::tags2::at;
pub(crate) use crate::game::fe::festateoptions::gpa;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::gp as gp_3;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::fe::feasyncfile::gpa as gpa_2;
pub(crate) use crate::game::fe::feasyncfile::gpn;
pub(crate) use crate::game::fe::feasyncfile::lw;
pub(crate) use crate::game::scripter::ssxscriptengine::p as p_3;
pub(crate) use crate::game::be::be::gp as gp_4;
pub(crate) use crate::game::fe::festaterewards::at_4 as at_2;
pub(crate) use crate::game::fe::feasyncfile::gp_13 as gp_5;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::p as p_4;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_6;
pub(crate) use crate::game::fe::feasyncfile::shl32;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_3;
pub(crate) use crate::game::fe::feasyncfile::p as p_5;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw_4;
pub(crate) use crate::game::animation::rideranimbase::sw;
pub(crate) use crate::game::be::belibrary::lb;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0023_4be8, game_state_play_script_event, "GameState_PlayScriptEvent"),
    (0x0023_4c68, game_state_countdown_update, "GameStateCountdown_Update"),
    (0x0023_4e20, game_state_race_enter, "GameStateRace_Enter"),
    (0x0023_4ef8, game_state_reset_ctor, "GameStateReset_ctor"),
    (0x0023_5080, game_state_reset_overlay_update, "GameStateReset_OverlayUpdate"),
    (0x0023_5570, game_state_reset_overlay_exit, "GameStateReset_OverlayExit"),
    (0x0023_55b0, game_state_reset_v08, "GameStateReset_v08"),
    (0x0023_57f8, game_state_reset_update, "GameStateReset_Update"),
    (0x0023_5a18, gamestatereset_load_state, "GameStateReset_LoadState"),
    (0x0023_5cc8, game_state_restart_race_update, "GameStateRestartRace_Update"),
    (0x0023_5f20, game_state_restart_race_exit, "GameStateRestartRace_Exit"),
    (0x0023_5fd0, gamestaterestartrace_load_state, "GameStateRestartRace_LoadState"),
    (0x0023_6198, game_state_warp_to_event_update, "GameStateWarpToEvent_Update"),
    (0x0023_6208, game_state_event_select_ctor, "GameStateEventSelect_ctor"),
    (0x0023_67c8, game_state_event_select_load_state, "GameStateEventSelect_LoadState"),
    (0x0023_6868, game_state_event_switch_load_ctor, "GameStateEventSwitchLoad_ctor"),
    (0x0023_6b38, game_state_reset_ai_exit, "GameStateResetAI_Exit"),
    (0x0023_6bb0, game_state_prerace_menu_enter, "GameStatePreRaceMenu_Enter"),
    (0x0023_6c48, game_state_pre_race_menu_update, "GameStatePreRaceMenu_Update"),
    (0x0023_6c88, game_state_pre_race_menu_exit, "GameStatePreRaceMenu_Exit"),
    (0x0023_6cd8, gamestatepreracemenu_resume_race, "GameStatePreRaceMenu_ResumeRace"),
    (0x0023_6e60, game_state_results_update, "GameStateResults_Update"),
    (0x0023_6ea0, game_state_results_exit, "GameStateResults_Exit"),
    (0x0023_6f08, game_state_finish_cinematic_enter, "GameStateFinishCinematic_Enter"),
    (0x0023_6f40, game_state_finish_cinematic_update, "GameStateFinishCinematic_Update"),
    (0x0023_6f90, game_state_finish_cinematic_exit, "GameStateFinishCinematic_Exit"),
    (0x0023_6fe8, game_state_replay_playback_enter, "GameStateReplayPlayback_Enter"),
    (0x0023_7038, game_state_replay_playback_exit, "GameStateReplayPlayback_Exit"),
    (0x0023_7060, input_sync_ctor, "InputSync_ctor"),
    (0x0023_7950, input_sync_is_ready, "InputSync_IsReady"),
    (0x0023_79c8, inputsync_apply_input, "InputSync_ApplyInput"),
    (0x0023_7b28, input_sync_pack_sample, "InputSync_PackSample"),
    (0x0023_7be8, input_sync_unpack_sample, "InputSync_UnpackSample"),
    (0x0023_7cb0, game_rand_seed, "GameRand_Seed"),
    (0x0023_7cd8, game_rand_next, "GameRand_Next"),
    (0x0027_8f38, script_engine_list_prepare, "ScriptEngine_ListPrepare"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::EVENT_SELECT_HANDLE_INPUT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::MC_MANAGER_GET_INSTANCE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STATS_PROCESS_UPDATE_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STATS_PROCESS_RENDER_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_ENGINE_PUSH_FREE_NODE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_OBJ_DEACTIVATE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_DUMMY_OBJ_READ_FIELDS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::PATH_DATA_READ_FLOAT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::EA_MEM_COPY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RAND_NEXT_MAIN;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RAND_NEXT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::TEX_MAN_SET_ALLOC_FLAGS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::GRAPHICS_MAN_EMPTY_HOOK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_STATE_MGR_RENDER_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_PUSH_BACK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_NODE_UNLINK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_RESET_SECTIONS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STRLEN_AND_STRCPY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::CBXSTRING_ASSIGN_CSTR;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::LOBBY_MAN_REQUEST_STATS_PAGE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SSX_APP_BOOT_LOAD;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::USTR_FROM_ASCII;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::PATH_SET_TRACK_CONTROL_VALUE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_TRIGGER_GET_INFO;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_TRIGGER_GET_NAME;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STREAM_BUFFER_GET_STATUS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::MENU_SYS_GET_INPUT_CODE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::IRRADIANCE_REF_GET_ENTRY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::C_WORLD_SOUND_UNLINK_EMITTERS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RIDER_METRIX_BANK_COMBO;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAMERA_TRIGGER_STACK_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOLT_FILE_INDEX_MANAGER_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::EVENT_SELECT_ON_WIDGET_NOTIFY;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_STATE_GOTO_NEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_ALLOC_NAMED_ALIGNED;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_SEND_LOCK_CHAL;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SETUP_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SHARED_LOGIN_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_VOIP_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_PLAY_STATUS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_CHOOSE_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_CHANNEL_SET_LISTENER;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_INSTANCE_PAUSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_INSTANCE_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_CMD_QUEUE_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_FREE_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_RELEASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_GET_BUFFER_SIZE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_INSTANCE_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOAD_INDICATOR_CLEAR_BUSY;
    pub(crate) use crate::game::dirtysock::tags2::callee::WSCRIPT_MAN_DELETE_ALL_PROCESSES;
    pub(crate) use crate::game::dirtysock::tags2::callee::WSCRIPT_MAN_SPAWN_MISSIONS;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_CLOSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_READ;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_WRITE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_FLUSH;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_GET_WORLD_SECTION_INDEX;
    pub(crate) use crate::game::bx::bxstring2::callee::FE_MODULE_RIDER_PREVIEW_REQUEST_CHAR;
    pub(crate) use crate::game::bx::bxstring2::callee::C_FE_STATE_LOBBY_MENU_FILL_PLAYER_LIST;
    pub(crate) use crate::game::bx::bxstring2::callee::C_FE_STATE_LOBBY_MENU_UPDATE_PLAYER_SCROLL_ARROWS;
    pub(crate) use crate::game::bx::bxstring2::callee::EVENT_SELECT_ON_ACTIVATE;
    pub(crate) use crate::game::bx::bxstring2::callee::EVENT_SELECT_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::C_OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::C_LUNO_TABLE_ALLOC;
    pub(crate) use crate::game::bx::bxstring2::callee::CROWD_MAN2D_REMOVE_ALL;
    pub(crate) use crate::game::bx::bxstring2::callee::CROWD_MAN2D_RESET;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_INIT;
    pub(crate) use crate::game::bx::bxstring2::callee::GAME_MODULE_REQUEST_STATE as GAME_MODULE_REQUEST_STATE_2;
    pub(crate) use crate::game::bx::bxstring2::callee::VID_ENGINE_PLAYER_START_PLAYBACK;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_LIST_PREPARE;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_LIST_SET_PAUSED_NESTED;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_LIST_GET_ENTRY_TYPE;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_STOP_MAIN_LISTS;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_STOP_AUX_LISTS;
    pub(crate) use crate::game::bx::bxstring2::callee::SNAPSHOT_LIST_REMOVE;
    pub(crate) use crate::game::bx::bxstring2::callee::PATH_DATA_PARSE;
    pub(crate) use crate::game::bx::bxstring2::callee::WORLD_STATE_RESTORE;
    pub(crate) use crate::game::bx::bxstring2::callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_SOUND_BASE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_REMOVE_TICK_CALLBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::CW_SCRIPT_CACHE_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE_2 as BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE_2 as BE_LIBRARY_GET_INTERFACE;
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
    pub(crate) use crate::game::fe::festaterewards2::callee::APP_MODULE_TRY_EXIT;
    pub(crate) use crate::game::fe::festaterewards2::callee::S_LOGIN_UPDATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::USB_KB_DEFAULT_CONNECT_CB;
    pub(crate) use crate::game::fe::festaterewards2::callee::WORLD_STATE_STREAM_RESET;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_STREAM_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_FIND_NEXT_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_FIND_PREV_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_SNAP_TO_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_APPLY_CAMERA;
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
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_STATE_UPDATE;
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
    pub(crate) use crate::game::sound::bankmonitor::callee::GAME_STATE_GOTO_NEXT_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::PS2_MEMCARD_CREATE;
    pub(crate) use crate::game::sound::bankmonitor::callee::LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::sound::bankmonitor::callee::BUDDY_MAN_CONNECT;
    pub(crate) use crate::game::sound::bankmonitor::callee::STR_CASE_CMP;
    pub(crate) use crate::game::sound::bankmonitor::callee::SLOGIN_MC_GET_PORT_SLOT;
    pub(crate) use crate::game::sound::bankmonitor::callee::MEMCARD_REQUEST_GET_INFO_NOW;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_INT;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_VEC3;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_FLOAT_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_LIST_ADD_CHOICE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS;
    pub(crate) use crate::game::sound::bankmonitor::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SSX_AUDIO_SYSTEM_END_SCRIPT_MUSIC;
    pub(crate) use crate::game::sound::bankmonitor::callee::WORLD_PAINTER_SET_SCRIPT_SURFACE_MODE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_ENGINE_GET_INSTANCE;
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
    pub(crate) use crate::game::dirtysock::tags2::callee::REWARDS_FILE_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOUNDED_CAMERA_ALGORITHM_BASE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_CTOR_2 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_IS_EVENT_LOADED;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_CLOSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_LISTS;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_LOBBY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_PEER_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SHARED_LOGIN_MAN_SHUTDOWN;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_MAN_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_CACHE_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SNAPSHOT_EMPTY_HOOK_B;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RESTORE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_FREE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_END_PLAYBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_CHANNEL_RELEASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_HEAP_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ANIM_BANK_MAN_FIND_BANK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_IS_BANK_SLOT_LOADED;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_CURRENT_SKIPPABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_FIELD_B0;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_FIND_BANK_SLOT_BY_ID;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_CANCEL_BANK_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_LIST_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::FEMODULE_SET_PREVIEW_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_APP_START_FE_MODULE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOC_STRING_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE as BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE as BE_LIBRARY_GET_INTERFACE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::BENEW_PLAYER_INT_IS_MESSAGE_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_MESSAGE_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_INTERFACE_RIDER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MAN_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MAN_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::WORLD_VIEW_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_STREAM_WRITE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_REQUEST_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_REQUEST_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_API_ADD_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::DIRTY_SOCK_CREATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::DIRTY_SOCK_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::APP_SOMETHING;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::OV_SCREEN_PROFILE_SAVE_PDA_REFRESH_CARD_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::OV_SCREEN_PROFILE_SAVE_PDA_RESET_SLOT_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_STATE_GOTO_NEXT_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VID_ENGINE_INSTALL_MEM_HOOKS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VID_ENGINE_PLAYER_INIT_OPEN_PARAMS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_CLEAR_LOGIN;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_CLEAR_MESSAGES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_DISCONNECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_SAVE_EMIT as REPLAY_SAVE_EMIT_2;
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
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMITTER_FREE_DATA;
    pub(crate) use crate::game::dirtysock::tags2::callee::DYNAMIC_EMITTER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::DYNAMIC_EMITTER_FREE_DATA;
    pub(crate) use crate::game::dirtysock::tags2::callee::RCMP_DECODE_FRAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::RCMP_FRAME_NUMBER;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAM_CONTROLLER_SET_ALGORITHM;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_ON_ACTIVATE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_REFRESH_LIST;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_UPDATE_NOW_PLAYING;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_UPDATE_HELP_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::FEMODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_REFRESH_BUY_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_REFRESH_EQUIP_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_UPDATE_HELP_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOLT_FILE_INDEX_GET_NEXT_NODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_MESSENGER_SET_PLAYER_MENU_SELECTION;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_HANDLE_NET_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_STATS_REFRESH_RESULTS_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_IS_KEY_ENABLED;
    pub(crate) use crate::game::dirtysock::tags2::callee::CROWD_MAN2D_INIT_SLOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_STATE_PRE_RACE_MENU_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DISCONNECT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_COMM_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SETUP_MAN_SEND_CHARACTER_MSG;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_DATA_GET_INFO_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MOVIE_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_HIGHLIGHT_SLOT_CLEAR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_SPAN_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_EVENT_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_RAMP_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_HEAP_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::FLOAT_TO_UINT;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_MENU_SET_SELECTED_BY_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SET_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_WIDGET_SLOT_0;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_WIDGET_SLOT_1;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_UPDATE_BUTTONS_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_MESSAGE;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_APPLY_OUTPUT_MODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SOUND_PLAYER_SET_BY_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::SOUND_PLAYER_START;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_BE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_INTERFACE_CALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::CBXSTRING_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_SHOW;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMORY_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_ALLOC_NAMED_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_SET_OUTPUT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SET_LOCAL_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_CAN_SKIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_GET_PLAY_STATUS_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_4 as SCRIPT_BINDING_CTOR_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR_3 as SCRIPT_OBJ_CTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_SCRIPT_SET_OWNER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PATH_PLAT_FIND_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_DRAW_ROW_4 as MENU_SYS_ITEM_DRAW_ROW_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MATH_EVAL_CUBIC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_DTOR_BODY as UI_LIST_NODE_DTOR_BODY_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_CLOSE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_ADD_VERTEX;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_SCROLL_WINDOW;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_MASK_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_CURSOR_2 as KEYBOARD_POPUP_UPDATE_CURSOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_PACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_UNPACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_REWARD_INT_IS_PEAK_GOAL_COMPLETE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VEC3_NEG_DOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO_4 as SCRIPT_DATA_GET_INFO_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_INSTANCE_GET_REMAINING_STEPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CONCAT_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ATTACHED_DYNAMIC_EMITTER_CREATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_MAN2D_CLEAR_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_STREAM_REWIND_READ;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_ALGORITHM_DEFAULT_FOV;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_CHARACTER as BE_GET_RIDER_CHARACTER_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_PROCESS_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_FLAG_MANAGER_GROUP_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_TABLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_FIRST_5 as UI_LIST_FIRST_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_ASCII_TO_WIDE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_TEXT_SET_WIDE_STRING_INTERNAL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BELIBRARY_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_MAN_FREE_ENTRY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_SCREEN_FIND_OBJECT_3 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUILTIN_DELETE_6 as BUILTIN_DELETE_5;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_3 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_2 as NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHAT_ROOM_SELECT_PLAYER_BY_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_AGREEMENT_TERMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_RELEASE_SNAPSHOT_LISTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_DECLINE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_PICK_RANDOM_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_COUNT_SAVE_SNAPSHOTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_CHANNEL_GET_LENGTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_PUSH_2 as ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_IS_LOADED_WITH_NAME as SND_BANK_IS_LOADED_WITH_NAME_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK as SND_LOAD_QUEUE_CANCEL_BANK_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::CLEAR_MUSIC_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_CONFIGURE_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_UPDATE_EMITTER_ENABLES;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_PROFILE_5 as BE_GET_RIDER_PROFILE_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::BENEW_PLAYER_INT_GET_SLOT_2 as BENEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_CHAR_PROFILE_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_RAND_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_IS_FLAG4;
    pub(crate) use crate::game::fe::feasyncfile::callee::DATA_INSTANCE_FREE_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_4 as SCRIPT_CMD_QUEUE_AT_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_3 as SCRIPT_LIST_IS_MOVIE_ENTRY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE_2 as SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_SET_PAUSED;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_DESTROY_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATTACHED_EMITTER_FX_DESTROY_EMITTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_IS_HULL_FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SYNC_PROCESS_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MDF_FILE_CREATE_MODEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_START_PROFILE_SEARCH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_FINISH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_6 as UI_LIST_FIRST_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_DRAW_RECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_RESET_PROFILE_CHARACTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COPY_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_CREATE_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_LEVEL_TO_VOLUME_2 as AUDIO_LEVEL_TO_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_CHANNEL_VOLUME_2 as AUDIO_SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCPY_2 as MEMCPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCASECMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_6 as SPRINTF_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCROLL_SET_PARTS_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_3 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_GET_OBJECT_BY_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_DESTROY_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_FLUSH_ROSTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_API_DESTROY;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_HIDE_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_ITEM_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_BASE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_PHASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_SOUND_GET_TRIGGER_IDS;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_GET_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE_CALLEE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_LOAD as SND_BANK_LOAD_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_RESET_ONE;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_IS_LOADING;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_FINALIZE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LOOKUP_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LAUNCH_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_RESUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SUSPEND;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SET_RUNNING;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_SET_OBJECT_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_ABORT_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_EMAIL_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_LOGIN_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_12 as NET_FE_STATE_ON_CHILD_EVENT_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHALLENGE_POPUP_SET_SETTINGS_LOCKED;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_11 as NET_FE_STATE_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::VOIP_MAN_DEACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::VOIP_MAN_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_RESET_RACE_RESULT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_IS_TRACK_GROUP0_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STORE_RACE_RESULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_SKIP_WHITESPACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_GET_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_CHARACTER_2 as BE_GET_RIDER_CHARACTER_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COUNT_COLLECTED_ON_PEAK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COUNT_COMPLETED_ON_PEAK;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_GET_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::SETUP_BUDDY_ACTION_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_4 as NET_FE_STATE_GET_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_3 as NET_FE_STATE_SHOW_POPUP_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_GET_BITS_2 as MPEG2_GET_BITS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_MARKER_BIT_2 as MPEG2_MARKER_BIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_FLUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_MESSENGER_CLEAR_NAME_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_DTOR_5 as NET_FE_STATE_DTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_13 as NET_FE_STATE_ON_CHILD_EVENT_5;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_5 as NET_FE_STATE_SHOW_POPUP_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP as NET_FE_STATE_GET_POPUP_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_USE_EXISTING_ACCOUNT_CHECK_EMAIL;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_CONNECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_BX_STRING_ASSIGN_CSTR;
    pub(crate) use crate::game::fe::feasyncfile::callee::STR_HASH as STR_HASH_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_SET_LIST_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_SHOW_CARD_ERROR_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_LOAD_CONFIG_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_QUICK_PLAY_POPUP_SET_OPTIONS_HINT_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_QUICK_PLAY_POPUP_SET_OPTIONS_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_GET_OWNED_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_GET_ID_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_SET_TEXT_ID_2 as UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INSTANCE_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INTERFACE_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_HUMAN_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_ADVANCE_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_TICK_PROCESS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_BINDING;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_KILL_TIMERS_BY_CALLBACK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK_2 as SND_LOAD_QUEUE_CANCEL_BANK_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_UNLOAD_BANK_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_RESUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_POPUP_IS_KEY_ENABLED as KEYBOARD_POPUP_IS_KEY_ENABLED_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_POPUP_FIND_ENABLED_IN_COLUMN_FROM_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_CTOR_2_4 as UI_STATE_CTOR_2_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_PLAYER_PAD_MASK_2 as FE_MODULE_GET_PLAYER_PAD_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_FADE_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_SET_FADE_GAIN;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_AS_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_TABLE_FIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_PASS_THROUGH;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_CALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR_2 as LUNO_VM_REGISTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_GET_BITS as MPEG2_GET_BITS_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_MARKER_BIT as MPEG2_MARKER_BIT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_EXTENSION_AND_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_GET_START_POSE_ANGLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_PLACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_DATA_FIND_MARKER;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC_NAMED_128;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_TOP_STATES as UI_STATE_MGR_CLOSE_TOP_STATES_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_ALL_POPUPS_2 as UI_STATE_MGR_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_SWITCH_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_5 as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_MENU_SET_SELECTION_3 as UI_MENU_SET_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_QUIT_TO_LOAD_SCREEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_START_SEQUENCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_USER_DATA_2 as IN_SEQ_INSTANCE_SET_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_SPEED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_TRANSITION_TARGET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_TRANSITION_MAN_START_CAPTURED;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_CUT_SCENE_TRANSITION_MAN_SAVE_VIEWPORT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_RACE_INT_GET_TRACK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_RACE_INT_GET_SPECIAL_EVENT_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_METRIX_SHOW_HUD_MSG_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE as FE_MODULE_RIDER_PREVIEW_SET_VISIBLE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_RELEASE_BOLT_FILES;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_UNLOAD_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_LOAD as UI_OBJECT_LOAD_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXTURE_SET_FIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_BINDING_EVAL_FLOAT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_GET as MC_MANAGER_GET_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_FORMAT_SLOT_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_SPRINTF_ASCII_FMT;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_TO_ASCII;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_LEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ASYNC_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_MISSION_INT_GET_SESSION_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_SETUP_MAN_ALLOC_PACKET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_SETUP_MAN_FREE_PACKET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_COMM_SYSTEM_CHANNEL_RECV;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_COMM_SYSTEM_CHANNEL_SEND;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_BX_STRING_CTOR_CSTR;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED_4 as OPERATOR_NEW_NAMED_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GOTO_NEXT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_SSX_WORLD_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_OPTIONS_DEVICE_SELECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_IS_SPEECH_VOLUME_ON;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_SYSTEM_IS_INSTANCE_READY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_CHARACTER_TO_SPEECH_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPCH_MAKE_ID_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_IS_LOADED;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_GET_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_RIDER_GET_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SCRIPT_OBJ_MATCHES_RIDER_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_INSTANCE_RESET as BANK_MONITOR_INSTANCE_RESET_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_BUY_SELECTED_ITEM;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_UPDATE_MONEY_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_REMOVE_LIST_ITEM;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_REFRESH_ITEM_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_TROPHY_ROOM_SHOW_TROPHY_IMAGE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_TROPHY_ROOM_SHOW_TROPHY_INFO;


    pub const C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_OPPONENT_CHAR: Callee = Callee {
        addr: 0x001b_1760,
        f: ee::s03::cFEStateOnlineCharacterSelect_ShowOpponentChar,
    };

    pub const C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_CHAR_STATS: Callee = Callee {
        addr: 0x001b_1b98,
        f: ee::s03::cFEStateOnlineCharacterSelect_ShowCharStats,
    };

    pub const MESSAGE_CENTER_REMOVE_MESSAGE: Callee = Callee {
        addr: 0x001e_31b8,
        f: ee::s04::MessageCenter_RemoveMessage,
    };

    pub const MESSAGE_CENTER_COUNT_GROUP_MESSAGES: Callee = Callee {
        addr: 0x001e_2bb8,
        f: ee::s04::MessageCenter_CountGroupMessages,
    };

    pub const MESSAGE_CENTER_GET_MESSAGE_COUNT: Callee = Callee {
        addr: 0x001e_33e8,
        f: ee::s04::MessageCenter_GetMessageCount,
    };

    pub const OV_SCREEN_MESSAGE_CENTER_REFRESH_LIST: Callee = Callee {
        addr: 0x001e_48b0,
        f: ee::s04::OVScreenMessageCenter_RefreshList,
    };

    pub const OV_SCREEN_MESSAGE_CENTER_UPDATE_COUNT_TEXT: Callee = Callee {
        addr: 0x001e_4758,
        f: ee::s04::OVScreenMessageCenter_UpdateCountText,
    };

    pub const OV_SCREEN_MESSAGE_CENTER_UPDATE_SCROLL_ARROWS: Callee = Callee {
        addr: 0x001e_4658,
        f: ee::s04::OVScreenMessageCenter_UpdateScrollArrows,
    };

    pub const INTC_TABLE_CLEAR_SLOTS: Callee = Callee {
        addr: 0x0036_1f60,
        f: ee::s12::IntcTable_ClearSlots,
    };

    pub const INTC_TABLE_INSTALL_VIF_VU_HANDLERS: Callee = Callee {
        addr: 0x0036_1e30,
        f: ee::s12::IntcTable_InstallVifVuHandlers,
    };

    pub const GAME_STATE_LOAD_STATE: Callee = Callee {
        addr: 0x0023_3b28,
        f: ee::s06::GameState_LoadState,
    };

    pub const VD_TEXTURE_SINK_CTOR: Callee = Callee {
        addr: 0x0025_4410,
        f: ee::s06::VD_TextureSink_ctor,
    };

    pub const BE_GET_RIDER_PROFILE: Callee = Callee {
        addr: 0x0014_a0e0,
        f: ee::s01::BE_GetRiderProfile,
    };

    pub const BE_NEW_PLAYER_INT_GET_SLOT: Callee = Callee {
        addr: 0x0014_7798,
        f: ee::s01::BENewPlayerInt_GetSlot,
    };

    pub const BE_LIBRARY_GET_INSTANCE_3: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::BELibrary_GetInstance,
    };

    pub const BE_LIBRARY_GET_INTERFACE_3: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::BELibrary_GetInterface,
    };

    pub const BE_ECON_INT_GET_TOTAL_EARNED: Callee = Callee {
        addr: 0x0015_09c0,
        f: ee::s01::BEEconInt_GetTotalEarned,
    };

    pub const BOLTS_FIND_BOLT_INDEX: Callee = Callee {
        addr: 0x0014_d448,
        f: ee::s01::Bolts_FindBoltIndex,
    };

    pub const SCRIPT_CMD_QUEUE_AT_2: Callee = Callee {
        addr: 0x0028_3d70,
        f: ee::s07::ScriptCmdQueue_At,
    };

    pub const SCRIPT_LIST_IS_MOVIE_ENTRY: Callee = Callee {
        addr: 0x0027_7dd8,
        f: ee::s07::ScriptList_IsMovieEntry,
    };

    pub const SCRIPT_ENGINE_GET_PROCESS_IN_SEQ: Callee = Callee {
        addr: 0x0028_2bf0,
        f: ee::s07::ScriptEngine_GetProcessInSeq,
    };

    pub const SECTION_ALLOCATOR_CTOR: Callee = Callee {
        addr: 0x003a_77b0,
        f: ee::s13::SectionAllocator_ctor,
    };

    pub const UI_STATE_CTOR_3: Callee = Callee {
        addr: 0x0039_e2a0,
        f: ee::s13::UIState_ctor,
    };

    pub const MEMCARD_STATE_BASE_INSTALL_CALLBACKS: Callee = Callee {
        addr: 0x001d_5488,
        f: ee::s04::MemCardStateBase_InstallCallbacks,
    };

    pub const UI_STATE_MGR_FIND_STATE: Callee = Callee {
        addr: 0x0039_f9d8,
        f: ee::s13::UIStateMgr_FindState,
    };

    pub const UI_STATE_SHUTDOWN_2: Callee = Callee {
        addr: 0x0039_e4a0,
        f: ee::s13::UIState_Shutdown,
    };

    pub const SCRIPT_ENGINE_LIST_STOP_3: Callee = Callee {
        addr: 0x0027_90a0,
        f: ee::s07::ScriptEngine_ListStop,
    };

    pub const SCRIPT_ENGINE_LIST_START: Callee = Callee {
        addr: 0x0027_8f68,
        f: ee::s07::ScriptEngine_ListStart,
    };

    pub const SSX_AUDIO_SYSTEM_GET_INSTANCE_2: Callee = Callee {
        addr: 0x0028_b180,
        f: ee::s07::SSXAudioSystem_GetInstance,
    };

    pub const SSX_AUDIO_SYSTEM_RESET_RACE_SOUNDS: Callee = Callee {
        addr: 0x0028_71b0,
        f: ee::s07::SSXAudioSystem_ResetRaceSounds,
    };

    pub const SCRIPT_ENGINE_GET_SCRIPT_OBJ: Callee = Callee {
        addr: 0x0027_d320,
        f: ee::s07::ScriptEngine_GetScriptObj,
    };

    pub const SCRIPT_OBJ_CAM_IS_BOUND_TO_PROCESS: Callee = Callee {
        addr: 0x0027_d790,
        f: ee::s07::ScriptObjCam_IsBoundToProcess,
    };

    pub const SCRIPT_ENGINE_IS_OBJ_BOUND_TO_PROCESS: Callee = Callee {
        addr: 0x0027_d210,
        f: ee::s07::ScriptEngine_IsObjBoundToProcess,
    };

    pub const SCRIPT_OBJ_CAM_HAS_BINDING: Callee = Callee {
        addr: 0x0028_2ef0,
        f: ee::s07::ScriptObjCam_HasBinding,
    };

    pub const SPEECH_INSTANCE_DTOR: Callee = Callee {
        addr: 0x002a_fb38,
        f: ee::s08::SpeechInstance_dtor,
    };

    pub const SSX_AUDIO_SYSTEM_IS_DJ_SPEECH_ON: Callee = Callee {
        addr: 0x0029_f0d8,
        f: ee::s08::SSXAudioSystem_IsDJSpeechOn,
    };

    pub const SPEECH_SYSTEM_SET_EVENT_CONTEXT: Callee = Callee {
        addr: 0x002b_1458,
        f: ee::s08::SpeechSystem_SetEventContext,
    };

    pub const SSX_AUDIO_SYSTEM_GET_LANGUAGE_MASK: Callee = Callee {
        addr: 0x0029_f198,
        f: ee::s08::SSXAudioSystem_GetLanguageMask,
    };

    pub const MENU_SYS_PAGE_VALIDATE_SELECTION: Callee = Callee {
        addr: 0x002c_aab0,
        f: ee::s09::MenuSysPage_ValidateSelection,
    };

    pub const MENU_SYS_PAGE_SELECT_NEXT: Callee = Callee {
        addr: 0x002c_ab08,
        f: ee::s09::MenuSysPage_SelectNext,
    };

    pub const MENU_SYS_PAGE_SELECT_PREV: Callee = Callee {
        addr: 0x002c_abe0,
        f: ee::s09::MenuSysPage_SelectPrev,
    };

    pub const MENU_SYS_ITEM_HANDLE_INPUT: Callee = Callee {
        addr: 0x002c_a2b0,
        f: ee::s09::MenuSysItem_HandleInput,
    };

    pub const MENU_SYS_ITEM_IS_DISABLED: Callee = Callee {
        addr: 0x002c_a3b0,
        f: ee::s09::MenuSysItem_IsDisabled,
    };

    pub const MENU_SYS_ITEM_GET_INPUT_CODE: Callee = Callee {
        addr: 0x002c_a3e8,
        f: ee::s09::MenuSysItem_GetInputCode,
    };

    pub const SCRIPT_BINDING_PAUSE_SEQ: Callee = Callee {
        addr: 0x0028_3038,
        f: ee::s07::ScriptBinding_PauseSeq,
    };

    pub const SCRIPT_BINDING_RESUME_SEQ: Callee = Callee {
        addr: 0x0028_3060,
        f: ee::s07::ScriptBinding_ResumeSeq,
    };

    pub const GAME_ANIM_MAP_UNREGISTER_BANK: Callee = Callee {
        addr: 0x0031_4fe8,
        f: ee::s10::GameAnimMap_UnregisterBank,
    };

    pub const ANIM_DATA_BASE_FREE_CHAN_CACHES: Callee = Callee {
        addr: 0x0031_1110,
        f: ee::s10::AnimDataBase_FreeChanCaches,
    };

    pub const ADF_DATA_SOURCE_DTOR: Callee = Callee {
        addr: 0x0031_12c8,
        f: ee::s10::AdfDataSource_dtor,
    };

    pub const C_KEYBOARD_POPUP_SET_KEY_ENABLED: Callee = Callee {
        addr: 0x001c_e3c8,
        f: ee::s04::cKeyboardPopup_SetKeyEnabled,
    };

    pub const RIDER_CLEAR_NEARBY_LIGHTS: Callee = Callee {
        addr: 0x0012_2658,
        f: ee::s00::Rider_ClearNearbyLights,
    };

    pub const SNDSTRM_CLOSE: Callee = Callee {
        addr: 0x003b_7838,
        f: ee::s13::SNDSTRM_Close,
    };

    pub const EA_MEM_FREE_4: Callee = Callee {
        addr: 0x0031_7e98,
        f: ee::s10::EA_MemFree,
    };

    pub const BUILTIN_DELETE_4: Callee = Callee {
        addr: 0x0031_7e50,
        f: ee::s10::__builtin_delete,
    };

    pub const NET_FE_STATE_OPEN_KEYBOARD_TEXT_ID: Callee = Callee {
        addr: 0x001a_9150,
        f: ee::s03::NetFEState_OpenKeyboardTextId,
    };

    pub const NET_FE_STATE_ON_CHILD_EVENT_3: Callee = Callee {
        addr: 0x001a_97b8,
        f: ee::s03::NetFEState_OnChildEvent,
    };

    pub const SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN_PTR: Callee = Callee {
        addr: 0x0028_7968,
        f: ee::s07::SSXAudioSystem_GetChannelGainPtr,
    };

    pub const BANK_MONITOR_ADD_TIMER: Callee = Callee {
        addr: 0x002a_dca0,
        f: ee::s08::BankMonitor_AddTimer,
    };

    pub const BANK_MONITOR_STOP_HANDLE: Callee = Callee {
        addr: 0x002a_d5f0,
        f: ee::s08::BankMonitor_StopHandle,
    };

    pub const UI_LIST_FIRST_2: Callee = Callee {
        addr: 0x0039_7840,
        f: ee::s13::UIList_First,
    };

    pub const UI_LIST_NODE_DTOR_BODY: Callee = Callee {
        addr: 0x003a_4868,
        f: ee::s13::UIListNode_dtorBody,
    };

    pub const C_GAME_COMM_RECV_GAME_PACKET: Callee = Callee {
        addr: 0x0025_8160,
        f: ee::s06::cGameComm_RecvGamePacket,
    };

    pub const C_FE_STATE_BRAGGING_RIGHTS_INIT_PLAYER_ROWS: Callee = Callee {
        addr: 0x0019_26f0,
        f: ee::s02::cFEStateBraggingRights_InitPlayerRows,
    };


    // Cross-shard calls the generator emitted as `call_indirect` (direct
    // `jal`s whose target lives in another shard).
    pub const EA_MEM_ALLOC_NAMED_3: u32 = 0x0031_7d70;

    pub const MEM_MOVE: u32 = 0x0041_610c;

    pub const STR_COPY: u32 = 0x0041_66f4;

    pub const TAG_FIELD_SET_STRING: u32 = 0x003e_c180;

    pub const STR_N_COPY: u32 = 0x0041_6ce0;

    pub const SPHERE_TREE_COLLIDE_SPHERE_TREE: u32 = 0x0032_cb58;

    pub const SPHERE_TREE_COLLIDE_AABB: u32 = 0x0032_c928;

    pub const SPCH_SHUTDOWN: u32 = 0x003d_67f8;

    pub const SPCH_MAKE_ID: u32 = 0x003d_8008;

    pub const MEM_FREE: u32 = 0x0025_23a8;

    pub const STR_HASH: u32 = 0x0031_7670;
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

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod table {
    pub(crate) use crate::game::fe::feasyncfile::table::STR_HASH;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::table::BXSTRING_ASSIGN;
    pub(crate) use crate::game::fe::feasyncfile::table::BXSTRING_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_MENU_RESET_SCROLL;
    pub(crate) use crate::game::fe::feasyncfile::table::PS2_MEM_CARD_CREATE;
    pub(crate) use crate::game::fe::feasyncfile::table::PATH_STOP_TRACKS;
    pub(crate) use crate::game::fe::feasyncfile::table::EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::table::BUILTIN_DELETE;
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

/// Addresses the generator emitted as `call_indirect` (outside the shard's
/// own direct-call set, so no readable alias is needed here).
pub(crate) mod addr {
    pub(crate) use crate::game::fe::feasyncfile::addr::__BUILTIN_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::addr::EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::addr::C_BX_STRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::addr::__FIXUNSSFSI;
    pub(crate) use crate::game::fe::feasyncfile::addr::STRLEN;
    pub(crate) use crate::game::fe::feasyncfile::addr::FONT_DTOR;
}


/// `GameStateReset *GameStateReset_ctor(GameStateReset *self)` (0x234ef8):
/// state words `+0=4`, `+4=0`, `+0xc=0x47d428`, `+0x10=5`, `+0x14=0`,
/// `+0x18=0`, `+0x1c=0`.
pub fn game_state_reset_ctor(c: &mut Cpu) {
    const VTABLE: u32 = 0x48_0000 - 0x2bd8;
    let this = Addr(abi::arg(c, 0));
    c.cycles += 18;
    this.sw(c, 0);
    this.off(0xc).sw(c, sx(0x48_0000 - 0x2918));
    let vtable = sx(VTABLE);
    c.g[V1][0] = vtable;
    Addr((c.g[GP][0] as u32).wrapping_sub(0x488)).sw(c, 0);
    c.g[A1][0] = 4;
    c.g[A2][0] = 5;
    this.off(0xc).sw(c, vtable);
    c.g[V0][0] = c.g[A0][0];
    this.sw(c, 4);
    this.off(0x10).sw(c, 5);
    this.off(0x4).sw(c, 0);
    this.off(0x14).sw(c, 0);
    this.off(0x18).sw(c, 0);
    this.off(0x1c).sw(c, 0);
    abi::ret(c);
}


/// `GameStateEventSelect *GameStateEventSelect_ctor(GameStateEventSelect *self)`
/// (0x236208): state words `+0=4`, `+4=0`, `+8=0`, `+0xc=0x47d320`,
/// `+0x14=0`, `+0x18=0x17`.
pub fn game_state_event_select_ctor(c: &mut Cpu) {
    const VTABLE: u32 = 0x48_0000 - 0x2ce0;
    let this = Addr(abi::arg(c, 0));
    c.cycles += 17;
    this.sw(c, 0);
    this.off(0xc).sw(c, sx(0x48_0000 - 0x2918));
    let vtable = sx(VTABLE);
    c.g[V1][0] = vtable;
    Addr((c.g[GP][0] as u32).wrapping_sub(0x488)).sw(c, 0);
    c.g[A1][0] = 4;
    c.g[A2][0] = 0x17;
    this.off(0xc).sw(c, vtable);
    c.g[V0][0] = c.g[A0][0];
    this.sw(c, 4);
    this.off(0x18).sw(c, 0x17);
    this.off(0x4).sw(c, 0);
    this.off(0x8).sw(c, 0);
    this.off(0x14).sw(c, 0);
    abi::ret(c);
}


/// `GameStateEventSwitchLoad *GameStateEventSwitchLoad_ctor(
/// GameStateEventSwitchLoad *self)` (0x236868): state words `+0=0xa`, `+4=0`,
/// `+0xc=0x47d2c8`.
pub fn game_state_event_switch_load_ctor(c: &mut Cpu) {
    const VTABLE: u32 = 0x48_0000 - 0x2d38;
    let this = Addr(abi::arg(c, 0));
    c.cycles += 13;
    this.sw(c, 0);
    this.off(0xc).sw(c, sx(0x48_0000 - 0x2918));
    let vtable = sx(VTABLE);
    c.g[V1][0] = vtable;
    Addr((c.g[GP][0] as u32).wrapping_sub(0x488)).sw(c, 0);
    c.g[V0][0] = 0xa;
    this.sw(c, 0xa);
    this.off(0xc).sw(c, vtable);
    c.g[V0][0] = c.g[A0][0];
    this.off(0x4).sw(c, 0);
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Input sync.

/// `InputSync *InputSync_ctor(InputSync *self, uint a1, int divisor, uint a3)`
/// (0x237060): `self+0x10 = 0`, `self+0x8 = divisor`, `self+0 = a1*divisor`,
/// `self+4 = (60/divisor)*divisor` (both products signed 32-bit, low word);
/// the divide and multiplies also leave LO/HI set.
pub fn input_sync_ctor(c: &mut Cpu) {
    const B: u32 = 0x4;
    const C: u32 = 0x8;
    const D: u32 = 0xc;
    const E: u32 = 0x10;
    let this = Addr(abi::arg(c, 0));
    c.cycles += 13;
    // v1 is pre-loaded with the constant 60, the dividend of the `div`.
    c.g[V1][0] = 0x3c;
    this.off(B).sw(c, c.g[A1][0]); // a1 (overwritten at the end)
    let (q, _) = abi::div(c, 0x3c, c.g[A2][0] as u32); // div v1, a2
    this.off(C).sw(c, c.g[A2][0]);
    this.off(D).sw(c, 0);
    c.g[V0][0] = c.g[A0][0];
    this.off(E).sw(c, 0);
    c.g[V1][0] = sx(q);
    // Two scratch multiplies: `mult a1*a2` leaves the low word in a3, then
    // `mult v1*a2` leaves its low word back in v1 (and LO/HI).
    let a3 = abi::mult(c, c.g[A1][0] as u32, c.g[A2][0] as u32);
    c.g[A3][0] = sx(a3);
    let v = abi::mult(c, q, c.g[A2][0] as u32);
    c.g[V1][0] = sx(v);
    this.sw(c, c.g[A3][0]);
    c.cycles += 2;
    this.off(B).sw(c, c.g[V1][0]);
    abi::ret(c);
}


/// `unsigned int GameRand_Next(void)` (0x00237cd8): next value of the global
/// game RNG at `D_004C9548`.
pub fn game_rand_next(c: &mut Cpu) {
    forward(
        c,
        8,
        0x10,
        |c| c.g[A0][0] = sx(0x4d_0000u32.wrapping_sub(0x6ab8)),
        callee::RAND_NEXT,
        0x0023_7cec,
    )
}


/// `void GameStateResults_Exit(GameStateResults* s)` (0x236ea0): close every
/// UI state below the results state manager
/// (`UIStateMgr_CloseAll(D_004A2C68->+0x48 + 0x18)`).
pub fn game_state_results_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(0x488));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(0x48));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x18));
        call_ptr(c, 0x0039_f840, 0x0023_6eb8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void GameRand_Seed(unsigned seed)` (0x237cb0): seed the pseudo random
/// generator at `D_004C9548`.
pub fn game_rand_seed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[A1][0] = c.g[A0][0];
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x004d_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0x6ab8));
        call_ptr(c, 0x0031_7958, 0x0023_7cc8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void GameStateRestartRace_Exit(void)` (0x235f20): clear flag 3 of the
/// race manager's load indicator.
pub fn game_state_restart_race_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save_ra(c, 0);
        c.g[A1][0] = 3;
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x94)).lw(c);
        call_ptr(c, callee::LOAD_INDICATOR_CLEAR_BUSY, 0x0023_5f3c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void GameStateReplayPlayback_Exit(void)` (0x237038): tear down the
/// playback world script manager's processes and respawn its missions.
pub fn game_state_replay_playback_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(gp(c, 0xce8)).lw(c);
        frame.save_ra(c, 0);
        call_ptr(c, callee::WSCRIPT_MAN_DELETE_ALL_PROCESSES, 0x0023_704c)?;
        c.g[A0][0] = Addr(gp(c, 0xce8)).lw(c);
        call_ptr(c, callee::WSCRIPT_MAN_SPAWN_MISSIONS, 0x0023_7054)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `GameStateWarpToEvent_Update()` 0x236198: request the game module's race
/// state (`world->+0x84`, param 1).
pub fn game_state_warp_to_event_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        let world = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[V0][0] = world;
        frame.save_ra(c, 0);
        c.g[A1][0] = 1;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A0][0] = lw_at(c, world, 0x84);
        callee::GAME_MODULE_REQUEST_STATE_2.call(c, 0x0023_61b8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// The `ScriptEngine_List*` helpers: address list `i` of the engine's rider
/// script array (`*(engine + 0x54c)` + `i * 0xcc`).
pub(crate) fn script_list_call(c: &mut Cpu, ret: u32, second: Option<u64>, list: Callee) {
    guest(c, |c| {
        c.cycles += 11;
        let engine = abi::arg(c, 0);
        let index = abi::arg(c, 1);
        c.g[V0][0] = 0xcc;
        let frame = Frame::open(c, 0x10);
        let stride = abi::mult(c, index, 0xcc);
        c.g[V1][0] = sx(stride);
        frame.save_ra(c, 0);
        match second {
            Some(v) => c.g[A1][0] = v,
            None => c.g[A1][0] = 0,
        }
        let base = c.lw(engine.wrapping_add(0x54c));
        c.g[A0][0] = sx((base as u32).wrapping_add(stride));
        list.call(c, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `ScriptEngine_ListPrepare(engine, i)` 0x278f38: prepare list `i`.
pub fn script_engine_list_prepare(c: &mut Cpu) {
    script_list_call(c, 0x0027_8f58, None, callee::SCRIPT_LIST_PREPARE)
}


/// `void GameStateResetAI_Exit(GameState* self)` (0x236b38): tell the race
/// manager's AI controller (virtual slot +0x2c) to reset.
pub fn game_state_reset_ai_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x488)).lw(c);
        frame.save_ra(c, 0);
        c.g[A1][0] = 3;
        c.g[A2][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0xcc)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x28)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x2c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_6b60)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void GameStateFinishCinematic_Enter()` (0x236f08): hand the audio system to
/// the character-tune player and flag the race manager's cinematic end.
pub fn game_state_finish_cinematic_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::SSX_AUDIO_GET_INSTANCE.call(c, 0x0023_6f18)?;
        c.g[A0][0] = c.g[V0][0];
        callee::SSX_AUDIO_PLAY_CHARACTER_TUNE.call(c, 0x0023_6f20)?;
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        c.g[A1][0] = 1;
        frame.restore_ra(c, 0);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x84).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
        let v1 = c.g[V1][0];
        let a1 = c.g[A1][0];
        Addr(v1 as u32).off(0x8c).sw(c, a1);
        frame.close(c);
        Ok(())
    })
}


/// `void GameStateReset_OverlayExit(GameStateReset* self)` (0x235570): unload
/// the world scripts, stop the script lists, and move to state 5.
pub fn game_state_reset_overlay_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let world = c.lw_r_2(A0, gp_2(c, -0x84c));
        let _ = world;
        callee::SCRIPT_ENGINE_UNLOAD_WORLD_SCRIPTS.call(c, 0x0023_5588)?;
        let world = c.lw_r_2(A0, gp_2(c, -0x84c));
        let _ = world;
        c.g[A1][0] = 1;
        callee::SCRIPT_ENGINE_LIST_STOP.call(c, 0x0023_5594)?;
        c.g[V0][0] = 5;
        frame.restore_ra(c, 0);
        Addr(r(c, S0).wrapping_add(0x10)).sw(c, 5);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}


/// `void GameStateResults_Update(GameStateResults* self)` (0x236E60): once
/// the results screen reports itself ready (field +0x8 == 1), ask the game
/// module for state 0xE (leaving the results screen).
pub fn game_state_results_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 1;
        frame.save_ra(c, 0);
        let state = c.lw_r_2(V1, r(c, A0).wrapping_add(8));
        frame.restore_ra(c, 0);
        if state == c.g[V0][0] {
            c.cycles += 7;
            let world = c.lw_r_2(V0, gp_2(c, -0x848));
            c.g[A1][0] = 0xe;
            c.g[A2][0] = 2;
            c.g[A3][0] = 0;
            let module = c.lw_r_2(A0, (world as u32).wrapping_add(0x84));
            let _ = module;
            callee::GAME_MODULE_REQUEST_STATE.call(c, 0x0023_6e90)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Game state / mode dispatch.

/// `void GameStatePreRaceMenu_Update(GameState* self)` (0x236c48): leave the
/// pre-race menu when the lobby manager (gp-0x204) is gone or its `+4` is
/// clear, and when the state's `+8` is 1, advance to the next state.
pub fn game_state_pre_race_menu_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let gate = Addr(gp(c, -0x204)).lw(c);
        c.g[V0][0] = gate;
        let mut epilogue = false;
        if gate != 0 {
            c.cycles += 3;
            let next = Addr(gate as u32).off(4).lw(c);
            c.g[V0][0] = next;
            epilogue = next != 0;
            frame.restore_ra(c, 0);
        }
        if !epilogue {
            c.cycles += 4;
            c.g[V1][0] = Addr(abi::arg(c, 0)).off(8).lw(c);
            c.g[V0][0] = 1;
            let not_first = c.g[V1][0] != c.g[V0][0];
            frame.restore_ra(c, 0);
            if !not_first {
                c.cycles += 3;
                callee::GAME_STATE_GOTO_NEXT_2.call(c, 0x0023_6c7c)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        leave(c, 0x10);
        Ok(())
    })
}


/// `void GameStateReplayPlayback_Enter(void)` (0x236fe8): start a replay
/// playback: load the script, unload it, quiet the race manager's movie
/// player and step the replay manager.
pub fn game_state_replay_playback_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.lw(gp(c, 0xce8));
        frame.save_ra(c, 0);
        call_ptr(c, callee::SCRIPT_MAN_LOAD, 0x0023_6ffc)?;
        c.g[A0][0] = c.lw(gp(c, 0xce8));
        call_ptr(c, callee::SCRIPT_MAN_UNLOAD, 0x0023_7004)?;
        c.g[V0][0] = c.lw(gp(c, -0x848));
        c.g[A1][0] = 0;
        c.g[V1][0] = c.lw(at(c, V0, 0x84));
        c.g[A0][0] = c.lw(at(c, V1, 0x94));
        call_ptr(c, callee::WORLD_VIEW_SET, 0x0023_7018)?;
        c.g[A0][0] = 0x11;
        call_ptr(c, callee::MENU_SYS_EVENT, 0x0023_7020)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void GameStateFinishCinematic_Update(GameState* self)` (0x236f40): while
/// the script engine is idle (status 0 or 5), advance to the next state.
pub fn game_state_finish_cinematic_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(gp(c, -0x84c)).lw(c);
        call_ptr(c, 0x0027_9220, 0x0023_6f5c)?;
        let idle = c.g[V0][0] == 0;
        c.g[V1][0] = 0;
        if !idle {
            c.cycles += 2;
            c.g[V0][0] ^= 5;
            c.g[V1][0] = (c.g[V0][0] > 0) as u64;
        }
        c.cycles += 2;
        if c.g[V1][0] != 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            callee::GAME_STATE_GOTO_NEXT_3.call(c, 0x0023_6f7c)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `D_004A2A50` / `D_004A2A54`: the pre-race menu stack top and depth.
pub(crate) const PRE_RACE_STACK_TOP_GP: i32 = -0x6a0;

pub(crate) const PRE_RACE_STACK_DEPTH_GP: i32 = -0x69c;


// ---------------------------------------------------------------------------
// Game state.

/// `void GameStatePreRaceMenu_Exit(GameStatePreRaceMenu* s)` (0x236c88): pop
/// the pre-race menu stack (`D_004A2A54` deep, `D_005366E8` indexes into
/// `D_004428F0`) into the current-menu global and resume the race.
/// 20 cycles plus the callee.
pub fn game_state_pre_race_menu_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let depth = c.lw(gp(c, PRE_RACE_STACK_DEPTH_GP));
        c.g[V0][0] = depth;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx(0x0053_66e8);
        frame.save_ra(c, 0);
        let new_depth = sx(r32(c, V0).wrapping_sub(1));
        c.g[V0][0] = new_depth;
        c.g[V1][0] = sx(r32(c, V0) << 2);
        c.sw(gp(c, PRE_RACE_STACK_DEPTH_GP), c.g[V0][0]);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
        c.g[A2][0] = sx(0x0044_28f0);
        c.g[V0][0] = c.lw(r32(c, V1));
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A2)));
        c.g[V1][0] = c.lw(r32(c, V0));
        c.sw(gp(c, PRE_RACE_STACK_TOP_GP), c.g[V1][0]);
        callee::GAME_STATE_PRE_RACE_MENU_RESUME.call(c, 0x0023_6ccc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int GameStateReset_v08(GameState* state)` (0x2355b0): `state->+0x10 == 5`.
pub fn game_state_reset_v08(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x10)).lw(c);
    c.g[V0][0] ^= 0x5;
    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
    abi::ret(c);
}


/// `void GameStateFinishCinematic_Exit(void)` (0x236f90): clear the cinematic
/// state word (`race->+0x8c`) of the current race.
pub fn game_state_finish_cinematic_exit(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = Addr(gpa(c, -0x848)).lw(c);
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x8c), 0);
    abi::ret(c);
}


/// `lw rt, -off(gp)`.
fn gpw(c: &mut Cpu, off: u32) -> u64 {
    c.lw((c.g[GP][0] as u32).wrapping_sub(off))
}


/// `ScriptEngine_ListAddChoice(engine, 1, choice, 3, 0, -1, -1, -1)` with
/// `a0` already the script engine, returning to `ret`.
fn reset_overlay_add_choice(c: &mut Cpu, choice: u64, ret: u32) -> Flow {
    c.g[A1][0] = 0x1;
    c.g[A2][0] = choice;
    c.g[A3][0] = 0x3;
    c.g[T0][0] = 0;
    c.g[T1][0] = u64::MAX;
    c.g[T2][0] = u64::MAX;
    c.g[T3][0] = u64::MAX;
    call_ptr(c, 0x0027_8e50, ret)
}


/// `void GameStateReset_OverlayUpdate(GameStateReset* self)` (0x235080): the
/// reset overlay's load sequence. `+0x18` counts updates since the overlay
/// opened; `+0x10` is the step:
///
/// - 0: once the script engine has loaded, prepare list 0 and (for the peak
///   transition, `+0x14` set) queue the peak transition script and its two
///   choices (and the peak music) on list 1, else the two reset choices when
///   the rider's state is 14..16 and `+0x1c8` is set; then step 1.
/// - 1: wait until neither script list is in state 1; step 2.
/// - 2: once every computer and actor rider has a render slot, restart the
///   game mode (unless the world type is 6..11), hand the AI rider count and
///   characters to the BE interfaces, rebuild the computer riders (and the
///   actor riders when the world type and sub-type are 0); step 3.
/// - 3: load the riders one step at a time; once all are loaded, reset them
///   (world type 0) or reorder them (type 5, sub-type 0); step 4.
/// - 4: the peak transition holds the overlay for at least 180 frames (3 s)
///   since it opened; step 5.
///
/// Tick rate: `+0x18` goes up once per tick, so the 180-frame hold is
/// compared in ticks ([`crate::tick::ticks_from_frames60`]; 180 at 60 Hz).
pub fn game_state_reset_overlay_update(c: &mut Cpu) {
    /// How a step ends: store `v0` as the new step (0x23553c), leave the step
    /// (0x235540), or leave with `s0` already reloaded (0x235544).
    enum End {
        Store,
        Leave,
        Reloaded,
    }
    guest(c, |c| {
        c.cycles += 17;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S6][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        let this = c.g[S6][0] as u32;
        c.g[V0][0] = c.lw(this.wrapping_add(0x18));
        c.g[A0][0] = c.lw(this.wrapping_add(0x10));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.g[V1][0] = (c.g[A0][0] < 0x6u64) as u64;
        c.sw(this.wrapping_add(0x18), c.g[V0][0]);
        let step = c.g[A0][0];
        let end = if step >= 6 {
            End::Leave
        } else {
            // The jump table at 0x47c060.
            c.cycles += 7;
            c.g[V0][0] = sx(0x0047_c060);
            c.g[V1][0] = sx(((step as u32) << 2).wrapping_add(0x0047_c060));
            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
            match step {
                0 => 'step0: {
                    c.cycles += 4;
                    c.g[A0][0] = gpw(c, 0x84c);
                    call_ptr(c, 0x0027_8d58, 0x0023_50e8)?;
                    let loading = c.g[V0][0] == 0;
                    c.g[A0][0] = gpw(c, 0x84c);
                    if loading {
                        break 'step0 End::Leave;
                    }
                    c.cycles += 10;
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x0027_8f38, 0x0023_50f8)?;
                    c.g[V1][0] = gpw(c, 0x848);
                    c.g[A0][0] = c.lw(this.wrapping_add(0x14));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1bc));
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xe));
                    c.g[A1][0] = (c.g[A1][0] < 0x3u64) as u64;
                    if c.g[A0][0] == 0 {
                        // The plain reset: the two choices when the rider's
                        // state is 14..16 and `+0x1c8` is set.
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c8));
                        let off = c.g[V0][0] == 0;
                        c.g[V0][0] = 0x1;
                        if off {
                            break 'step0 End::Store;
                        }
                        c.cycles += 2;
                        let state_ok = c.g[A1][0] != 0;
                        c.g[A0][0] = gpw(c, 0x84c);
                        if !state_ok {
                            break 'step0 End::Store;
                        }
                        c.cycles += 19;
                        reset_overlay_add_choice(c, 0x10, 0x0023_5254)?;
                        c.g[A0][0] = gpw(c, 0x84c);
                        reset_overlay_add_choice(c, 0x11, 0x0023_5278)?;
                        c.g[V0][0] = 0x1;
                        break 'step0 End::Store;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0xe;
                    let state14 = c.g[V1][0] == 0xe;
                    c.g[V1][0] ^= 0xf;
                    if state14 {
                        // The peak transition script, its music, and the two
                        // peak choices.
                        c.cycles += 33;
                        c.g[A0][0] = gpw(c, 0x84c);
                        c.g[A1][0] = 0x1;
                        c.g[A2][0] = 0x1d;
                        c.g[A3][0] = 0x3;
                        call_ptr(c, 0x0027_8e20, 0x0023_5138)?;
                        call_ptr(c, 0x0028_b180, 0x0023_5140)?;
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x0028_cd48, 0x0023_5148)?;
                        call_ptr(c, 0x0028_b180, 0x0023_5150)?;
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x118));
                        call_ptr(c, 0x002b_3a70, 0x0023_5158)?;
                        c.g[A0][0] = gpw(c, 0x84c);
                        reset_overlay_add_choice(c, 0, 0x0023_517c)?;
                        c.g[A0][0] = gpw(c, 0x84c);
                        reset_overlay_add_choice(c, 1, 0x0023_51a0)?;
                    } else {
                        // Script 0x1e (state 15) or 0x1f, and the two reset
                        // choices.
                        c.cycles += 26;
                        c.g[V0][0] = 0x1f;
                        c.g[A0][0] = gpw(c, 0x84c);
                        c.g[A2][0] = if c.g[V1][0] != 0 { 0x1f } else { 0x1e };
                        c.g[A1][0] = 0x1;
                        c.g[A3][0] = 0x3;
                        call_ptr(c, 0x0027_8e20, 0x0023_51c4)?;
                        c.g[A0][0] = gpw(c, 0x84c);
                        reset_overlay_add_choice(c, 0x10, 0x0023_51e8)?;
                        c.g[A0][0] = gpw(c, 0x84c);
                        reset_overlay_add_choice(c, 0x11, 0x0023_520c)?;
                    }
                    c.g[A0][0] = gpw(c, 0x84c);
                    c.cycles += 4;
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0027_8f38, 0x0023_5218)?;
                    c.g[V0][0] = 0x1;
                    End::Store
                }
                1 => {
                    // Both script lists out of state 1.
                    c.cycles += 7;
                    c.g[A0][0] = gpw(c, 0x84c);
                    c.g[A1][0] = 0;
                    c.g[S0][0] = 0;
                    call_ptr(c, 0x0027_91d8, 0x0023_5290)?;
                    c.g[V0][0] ^= 0x1;
                    let busy = c.g[V0][0] == 0;
                    c.g[A0][0] = gpw(c, 0x84c);
                    if !busy {
                        c.cycles += 5;
                        c.g[A1][0] = 0x1;
                        call_ptr(c, 0x0027_91d8, 0x0023_52a4)?;
                        c.g[V0][0] ^= 0x1;
                        c.g[V1][0] = 0x1;
                        if c.g[V0][0] != 0 {
                            c.g[S0][0] = 1;
                        }
                    }
                    c.cycles += 2;
                    let done = c.g[S0][0] != 0;
                    c.g[V0][0] = 0x2;
                    if done {
                        c.cycles += 2;
                        c.sw(this.wrapping_add(0x10), c.g[V0][0]);
                    }
                    End::Leave
                }
                2 => 'step2: {
                    c.cycles += 5;
                    c.g[V0][0] = gpw(c, 0x488);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x0012_abd0, 0x0023_52cc)?;
                    let no_slots = c.g[V0][0] == 0;
                    c.g[V0][0] = gpw(c, 0x488);
                    if no_slots {
                        break 'step2 End::Leave;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x0012_acf8, 0x0023_52dc)?;
                    let no_slots = c.g[V0][0] == 0;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                    if no_slots {
                        break 'step2 End::Reloaded;
                    }
                    c.cycles += 19;
                    call_ptr(c, 0x0014_dc80, 0x0023_52ec)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x0014_dd58, 0x0023_52f8)?;
                    c.g[S1][0] = c.g[V0][0];
                    call_ptr(c, 0x0014_dc80, 0x0023_5300)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0014_dd58, 0x0023_530c)?;
                    c.g[V1][0] = 0x530000;
                    c.g[S4][0] = c.g[V0][0];
                    c.g[V0][0] = sx(0x0053_5bc8);
                    c.g[A0][0] = gpw(c, 0x848);
                    // The world type (0x535c12).
                    c.g[A1][0] = c.lb(0x0053_5c12);
                    c.g[S7][0] = c.g[V1][0];
                    c.g[V0][0] = ((c.g[A1][0] as i64) < 6) as u64;
                    let below = c.g[V0][0] != 0;
                    c.g[S0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc0));
                    let restart = below || {
                        c.cycles += 4;
                        c.g[V0][0] = (0xb < (c.g[A1][0] as i64)) as u64;
                        c.g[A2][0] = gpw(c, 0x488);
                        c.g[V0][0] != 0
                    };
                    if restart {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0023_8160, 0x0023_5348)?;
                        c.g[A2][0] = gpw(c, 0x488);
                    }
                    // The AI rider count and characters.
                    c.cycles += 16;
                    let mode = c.g[S0][0] as u32;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.lw(mode.wrapping_add(0x10));
                    c.g[S2][0] = 0;
                    c.g[V0][0] = c.lw(mode.wrapping_add(0x14));
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                    c.g[S5][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[S3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                    c.g[A1][0] = c.g[S5][0];
                    call_ptr(c, 0x0014_4d98, 0x0023_5370)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    let commit = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, commit, 0x0023_5384)?;
                    let none = (c.g[S5][0] as i64) <= 0;
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
                    if !none {
                        c.cycles += 3;
                        c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
                        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S3][0] as u32));
                        loop {
                            c.cycles += 13;
                            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x28));
                            c.g[A0][0] = c.g[S4][0];
                            c.g[A1][0] = c.g[S0][0];
                            call_ptr(c, 0x0014_7338, 0x0023_53a8)?;
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                            c.g[A2][0] = c.lw(c.g[S1][0] as u32);
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A0][0] = c.g[S4][0];
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                            call_ptr(c, 0x0014_73d0, 0x0023_53c0)?;
                            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S5][0] as i64)) as u64;
                            let more = c.g[V0][0] != 0;
                            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S3][0] as u32));
                            if !more {
                                break;
                            }
                            if crate::events::event_due(c) {
                                crate::events::deliver_event_here(c, 0x0023_5398);
                            }
                        }
                    }
                    // Rebuild the riders.
                    c.cycles += 16;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xc));
                    c.g[S0][0] = sx(0x0053_5bc8);
                    c.g[S1][0] = 0;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let commit = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, commit, 0x0023_53e8)?;
                    c.g[V0][0] = gpw(c, 0x488);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x0012_b030, 0x0023_53f4)?;
                    c.g[V0][0] = gpw(c, 0x488);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x0012_b000, 0x0023_5400)?;
                    c.g[V0][0] = c.lb(0x0053_5c10);
                    let typed = c.g[V0][0] != 0;
                    c.g[V0][0] = gpw(c, 0x488);
                    if !typed {
                        c.cycles += 3;
                        c.g[V0][0] = c.lb(0x0053_5c11);
                        c.g[S1][0] = (c.g[V0][0] < 0x1u64) as u64;
                        c.g[V0][0] = gpw(c, 0x488);
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x0012_8958, 0x0023_5420)?;
                    let some = c.g[S5][0] != 0;
                    c.g[V0][0] = 0x3;
                    if some {
                        break 'step2 End::Store;
                    }
                    c.cycles += 2;
                    if c.g[S1][0] == 0 {
                        c.sw(this.wrapping_add(0x10), c.g[V0][0]);
                        break 'step2 End::Leave;
                    }
                    // No AI riders, world type and sub-type 0: the actor
                    // riders (mode 0, or 2 for a typed world).
                    c.cycles += 3;
                    c.g[V0][0] = c.lb(0x0053_5c10);
                    let typed = c.g[V0][0] != 0;
                    c.g[V0][0] = gpw(c, 0x488);
                    c.cycles += 5;
                    c.g[A1][0] = if typed { 0x2 } else { 0 };
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x0012_8998, if typed { 0x0023_545c } else { 0x0023_5448 })?;
                    c.g[V0][0] = 0x3;
                    End::Store
                }
                3 => 'step3: {
                    c.cycles += 5;
                    c.g[V0][0] = gpw(c, 0x488);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x0012_a180, 0x0023_5470)?;
                    let loading = c.g[V0][0] == 0;
                    c.g[V0][0] = gpw(c, 0x488);
                    if loading {
                        c.cycles += 4;
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        call_ptr(c, 0x0012_96f8, 0x0023_5514)?;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                        break 'step3 End::Reloaded;
                    }
                    c.cycles += 10;
                    call_ptr(c, 0x0014_dc80, 0x0023_5480)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x0014_dd58, 0x0023_548c)?;
                    c.g[V1][0] = 0x530000;
                    c.g[V0][0] = sx(0x0053_5bc8);
                    c.g[S0][0] = c.lb(0x0053_5c10);
                    let typed = c.g[S0][0] != 0;
                    c.g[S7][0] = c.g[V1][0];
                    if !typed {
                        c.cycles += 8;
                        c.g[V0][0] = gpw(c, 0x488);
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        call_ptr(c, 0x0012_89f0, 0x0023_54ac)?;
                        c.g[V0][0] = gpw(c, 0x488);
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        let ai = c.g[A0][0] as u32;
                        c.g[V1][0] = c.lw(ai.wrapping_add(0x80));
                        let none = (c.g[V1][0] as i64) <= 0;
                        c.g[V0][0] = 0x1;
                        if !none {
                            c.cycles += 1;
                            c.sw(ai.wrapping_add(0x14), c.g[V0][0]);
                        }
                        c.cycles += 5;
                        c.g[V0][0] = gpw(c, 0x488);
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        let ai = c.g[A0][0] as u32;
                        c.g[V1][0] = c.lw(ai.wrapping_add(0x84));
                        let none = (c.g[V1][0] as i64) <= 0;
                        c.g[V0][0] = 0x1;
                        if !none {
                            c.cycles += 1;
                            c.sw(ai.wrapping_add(0x8c), c.g[V0][0]);
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0x5;
                    let type5 = c.g[S0][0] == 0x5;
                    c.g[V0][0] = 0x4;
                    if !type5 {
                        break 'step3 End::Store;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = sx(0x0053_5bc8);
                    c.g[V1][0] = c.lb(0x0053_5c11);
                    if c.g[V1][0] != 0 {
                        c.g[V0][0] = 0x4;
                        break 'step3 End::Store;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = gpw(c, 0x488);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x0012_8a10, 0x0023_5504)?;
                    c.g[V0][0] = 0x4;
                    End::Store
                }
                4 => 'step4: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(this.wrapping_add(0x14));
                    let plain = c.g[V0][0] == 0;
                    c.g[V0][0] = 0x5;
                    if plain {
                        break 'step4 End::Store;
                    }
                    // The peak transition's minimum hold: 180 frames since
                    // the overlay opened (`slti v0, v0, 180`).
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(this.wrapping_add(0x18));
                    let hold = crate::tick::ticks_from_frames60(c, 180) as i64;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < hold) as u64;
                    let wait = c.g[V0][0] != 0;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                    if wait {
                        break 'step4 End::Reloaded;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 0x5;
                    End::Store
                }
                _ => End::Leave,
            }
        };
        match end {
            End::Store => {
                c.cycles += 2;
                c.sw(this.wrapping_add(0x10), c.g[V0][0]);
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            }
            End::Leave => {
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            }
            End::Reloaded => {}
        }
        c.cycles += 10;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = ra;
        Ok(())
    });
}


/// `void GameStateRestartRace_Update(GameStateRestartRace* self)` (0x235cc8):
/// restart the race. `+0x14` counts updates; `+0x10` is the step:
///
/// - 0: once the computer riders have render slots, rebuild them (and drop
///   the actor riders with two or more AI riders); step 1.
/// - 1: load the riders one step at a time; once all are loaded, reset them,
///   arm the AI start, and put the streamer's view on the start marker;
///   step 2.
/// - 2: with `+0x18` set, wait until the streamer's view is clear, at least
///   120 frames (2 s) have passed, list 0 is not running, no list or
///   transition is active, then skip the finish script; step 3.
/// - 3: go to the next game state.
///
/// Tick rate: `+0x14` goes up once per tick, so the 120-frame minimum is
/// compared in ticks ([`crate::tick::ticks_from_frames60`]; 120 at 60 Hz).
pub fn game_state_restart_race_update(c: &mut Cpu) {
    /// Where a step leaves: reload s0 (0x235f04), s1 (0x235f08), or only the
    /// rest (0x235f0c).
    enum End {
        S0,
        S1,
        Rest,
    }
    guest(c, |c| {
        c.cycles += 24;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S1][0] = c.g[A0][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
        let this = c.g[S1][0] as u32;
        c.g[S3][0] = 0x1;
        c.g[V0][0] = c.lw(this.wrapping_add(0x14));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.sw(this.wrapping_add(0x14), c.g[V0][0]);
        call_ptr(c, 0x0014_dc80, 0x0023_5cf8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0023_5d04)?;
        c.g[V0][0] = gpw(c, 0x848);
        c.g[A2][0] = c.lw(this.wrapping_add(0x10));
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
        c.g[S2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xb0));
        // The streamer view index: 1 when `+0xb0` is clear.
        c.g[S0][0] = (c.g[A0][0] < 0x1u64) as u64;
        let step = c.g[A2][0];
        let end = 'step: {
            if step == 1 {
                // Load the riders; once loaded, reset them and set up the
                // start.
                c.cycles += 4;
                c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                call_ptr(c, 0x0012_a180, 0x0023_5db8)?;
                let loading = c.g[V0][0] == 0;
                c.g[V0][0] = gpw(c, 0x848);
                if loading {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x0012_96f8, 0x0023_5e38)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    break 'step End::S1;
                }
                c.cycles += 27;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                call_ptr(c, 0x0012_89f0, 0x0023_5dcc)?;
                c.g[V0][0] = gpw(c, 0x488);
                c.g[V1][0] = 0x50;
                c.g[A2][0] = gpw(c, 0x848);
                c.mult(0, V1, S0, V1);
                c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.g[SP][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[S3][0]);
                c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x250), c.g[S3][0]);
                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x18));
                call_ptr(c, 0x0012_2c28, 0x0023_5e08)?;
                c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                c.g[A1][0] = c.g[S0][0];
                c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x5170));
                c.g[A2][0] = c.g[SP][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                call_ptr(c, 0x003a_9658, 0x0023_5e20)?;
                c.g[V0][0] = 0x2;
                c.sw(this.wrapping_add(0x10), c.g[V0][0]);
                break 'step End::S0;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A2][0] as i64) < 2) as u64;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                if step != 0 {
                    c.cycles += 2;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    break 'step End::Rest;
                }
                // Rebuild the computer riders once they have render slots.
                c.cycles += 4;
                c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                call_ptr(c, 0x0012_abd0, 0x0023_5d64)?;
                let no_slots = c.g[V0][0] == 0;
                c.g[V0][0] = gpw(c, 0x848);
                if no_slots {
                    break 'step End::S0;
                }
                c.cycles += 8;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                call_ptr(c, 0x0012_b030, 0x0023_5d78)?;
                c.g[V1][0] = 0x530000;
                // The AI rider count (0x535c04).
                c.g[V0][0] = c.lw(0x0053_5c04);
                c.g[V0][0] = ((c.g[V0][0] as i64) < 2) as u64;
                let few = c.g[V0][0] != 0;
                c.g[V0][0] = gpw(c, 0x848);
                if !few {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x0012_b000, 0x0023_5d98)?;
                    c.g[V0][0] = gpw(c, 0x848);
                }
                c.cycles += 5;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                call_ptr(c, 0x0012_8958, 0x0023_5da8)?;
                c.sw(this.wrapping_add(0x10), c.g[S3][0]);
                break 'step End::S0;
            }
            c.g[V0][0] = 0x2;
            c.cycles += 2;
            let step2 = step == 2;
            c.g[V0][0] = 0x3;
            if !step2 {
                c.cycles += 2;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                if step == 3 {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x0023_3aa0, 0x0023_5f04)?;
                    break 'step End::S0;
                }
                c.cycles += 2;
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'step End::Rest;
            }
            // Wait for the restart to settle, then skip the finish script.
            c.cycles += 3;
            c.g[V0][0] = c.lw(this.wrapping_add(0x18));
            let skip = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S0][0];
            if skip {
                c.cycles += 11;
                c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                c.g[A2][0] = c.g[SP][0];
                c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                call_ptr(c, 0x003a_9770, 0x0023_5e60)?;
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x516c));
                c.c_lt_s(1, 0);
                let blocked = c.fcc();
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                if blocked {
                    break 'step End::S1;
                }
                // At least 120 frames (`slti v0, v0, 0x78`), in ticks.
                c.cycles += 4;
                c.g[V0][0] = c.lw(this.wrapping_add(0x14));
                let min = crate::tick::ticks_from_frames60(c, 120) as i64;
                c.g[V0][0] = ((c.g[V0][0] as i64) < min) as u64;
                if c.g[V0][0] != 0 {
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    break 'step End::Rest;
                }
                c.cycles += 5;
                c.g[A0][0] = gpw(c, 0x84c);
                c.g[A1][0] = 0;
                call_ptr(c, 0x0027_91d8, 0x0023_5e94)?;
                let running = c.g[V0][0] == c.g[S3][0];
                c.g[V0][0] = gpw(c, 0x848);
                if running {
                    break 'step End::S0;
                }
                c.cycles += 8;
                c.g[A1][0] = 0;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                call_ptr(c, 0x0023_0698, 0x0023_5ea8)?;
                c.g[A0][0] = gpw(c, 0x84c);
                c.g[S0][0] = c.g[V0][0];
                call_ptr(c, 0x0027_9298, 0x0023_5eb4)?;
                if c.g[V0][0] != 0 {
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    break 'step End::S1;
                }
                c.cycles += 8;
                let trans = c.g[S0][0] as u32;
                c.g[V1][0] = c.lw(trans.wrapping_add(0x1c));
                c.g[A1][0] = sx(trans.wrapping_add(0x10));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                let active = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, active, 0x0023_5ed4)?;
                if c.g[V0][0] != 0 {
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    break 'step End::S1;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw(trans.wrapping_add(0x44));
                let busy = c.g[V0][0] != 0;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                if busy {
                    break 'step End::S1;
                }
                c.cycles += 2;
                c.g[A0][0] = gpw(c, 0x84c);
                call_ptr(c, 0x0027_a9f0, 0x0023_5ef0)?;
            }
            c.cycles += 3;
            c.g[V0][0] = 0x3;
            c.sw(this.wrapping_add(0x10), c.g[V0][0]);
            End::S0
        };
        if let End::S0 = end {
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        }
        if !matches!(end, End::Rest) {
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        }
        c.cycles += 5;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = ra;
        Ok(())
    });
}


/// `void GameStateReset_Update(GameStateReset* s)` (0x2357f8): once the reset
/// flag is armed and the script engine is idle, flag the "reset" field on the
/// race manager and let the state machine advance.
pub fn game_state_reset_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        sqsp(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(r32(c, S0), 0x1c)).lw(c);
        let idle = c.g[V0][0] == 0;
        c.g[A0][0] = Addr(gp_3(c, -0x84c)).lw(c);
        if !idle {
            c.cycles += 4;
            c.g[A0][0] = Addr(gp_3(c, -0x84c)).lw(c);
            call::call_ptr(c, 0x0027_aa80, 0x0023_581c)?;
            let busy = c.g[V0][0] != 0;
            c.g[A0][0] = Addr(gp_3(c, -0x84c)).lw(c);
            if !busy {
                c.cycles += 7;
                c.g[A0][0] = Addr(gp_3(c, -0x848)).lw(c);
                c.g[A1][0] = 1;
                c.g[V0][0] = Addr(p(c.g[A0][0] as u32, 0x84)).lw(c);
                c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x78)).lw(c);
                Addr(p(c.g[V1][0] as u32, 0x1a4)).sw(c, c.g[A1][0]);
                Addr(p(r32(c, S0), 0x1c)).sw(c, 0);
                c.g[A0][0] = Addr(gp_3(c, -0x84c)).lw(c);
            }
        }
        c.cycles += 4;
        c.g[A1][0] = 1;
        call::call_ptr(c, 0x0027_9220, 0x0023_5848)?;
        if c.g[V0][0] != 0 {
            lqsp(c, 16, 0x10);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call::call_ptr(c, 0x0023_3aa0, 0x0023_5858)?;
            lqsp(c, 16, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}


/// `InputSync_IsReady` (0x237950): true when the input sync's frame counter is
/// past its startup delay, it is not waiting on a load, and every self slot's
/// cursors agree.
pub fn input_sync_is_ready(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = Addr(gp(c, -0x204)).lw(c);
    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x70)).lw(c);
    c.g[V0][0] = ((c.g[V0][0] as i64) < 14) as u64;
    let early = c.g[V0][0] != 0;
    if early {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x3f8)).lw(c);
    let loading = c.g[V0][0] != 0;
    c.g[A1][0] = 0;
    let mut busy = loading;
    if !loading {
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x3fc)).lw(c);
        let idle = (c.g[V0][0] as i64) <= 0;
        if !idle {
            busy = true;
        }
    }
    if busy {
        c.cycles += 1;
        c.g[A1][0] = 1;
    }
    c.cycles += 2;
    if c.g[A1][0] != 0 {
        // A busy/loading input sync is "not ready": the generated body breaks
        // out of the slot walk to the shared `return 0`.
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.g[V0][0] = lw_at_2(c, A0, 0x10);
    c.cycles += 2;
    let none = (c.g[V0][0] as i64) <= 0;
    c.g[A1][0] = 0;
    if !none {
        c.cycles += 2;
        c.g[A2][0] = c.g[V0][0];
        c.g[A0][0] = low(c, A0).wrapping_add(0x14);
        loop {
            c.cycles += 4;
            c.g[V1][0] = lw_at_2(c, A0, 0x4);
            c.g[V0][0] = lw_at_2(c, A0, 0x8);
            let full = c.g[V1][0] == c.g[V0][0];
            c.g[A1][0] = low(c, A1).wrapping_add(1);
            if full {
                c.cycles += 2;
                c.g[V0][0] = 0;
                abi::ret(c);
                return;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A2][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = low(c, A0).wrapping_add(0x18);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0023_79a0);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}


/// `void GameStateCountdown_Update(GameState* g)` (0x234c68): while the race
/// countdown (`world->race->+0x1c`, 60 Hz frames, `RaceStateCountdown_Update`)
/// runs, sound a start light at each multiple of 60; when it is out, open
/// the gate (script event, HUD, audio) and go to the next state.
///
/// Above 60 Hz the countdown holds each value for `rate / 60` ticks, so the
/// body runs only when the value changed (or is out): a light per value, not
/// per tick. With a positive countdown the body does nothing else (getters
/// and the light). Every call runs the original at 60.
pub fn game_state_countdown_update(c: &mut Cpu) {
    let world = c.bus.read32((c.g[GP][0] as u32).wrapping_sub(0x488));
    let race = if world == 0 { 0 } else { c.bus.read32(world.wrapping_add(0xc)) };
    if race != 0 {
        let timer = race.wrapping_add(0x1c);
        let value = c.bus.read32(timer);
        if !crate::tick::value_changed(c, timer, value) && (value as i32) > 0 {
            abi::ret(c);
            return;
        }
    }
    ee::s06::f_00234c68__generated(c);
}


/// `void GameState_PlayScriptEvent(GameState* g, UStr* name)`
/// (0x00234be8): resolve the current track's world section and call the
/// script function it names, when the table has it.
pub fn game_state_play_script_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x0014_dc80, 0x0023_4c00)?; // BELibrary_GetInstance
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dd58, 0x0023_4c0c)?; // BELibrary_GetInterface
        c.g[V1][0] = c.lw(gpn(c, 0x848));
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
        call_ptr(c, 0x0014_4bc0, 0x0023_4c20)?; // BENewRaceInt_GetTrackInfo
        c.g[A1][0] = c.lw(c.g[V0][0] as u32);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0022_e098, 0x0023_4c2c)?; // SectionMan_GetEventWorldSection
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.lw(gpa_2(c, 0xce8));
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        call_ptr(c, 0x0030_9dd0, 0x0023_4c40)?; // WScriptMan_TableHasFunction
        let missing = c.g[V0][0] == 0;
        c.g[A0][0] = c.lw(gpa_2(c, 0xce8));
        if !missing {
            c.cycles += 3;
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S1][0];
            call_ptr(c, 0x0030_9e50, 0x0023_4c54)?; // WScriptMan_CallTableFunction
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


// ---------------------------------------------------------------------------
// Register and memory helpers.

/// `a + off` (32-bit pointer arithmetic).
#[inline(always)]
pub(crate) fn p_2(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}


/// A `gp`-relative global.
#[inline(always)]
pub(crate) fn gw(c: &Cpu, off: i32) -> u32 {
    abi::r32(c, GP).wrapping_add(off as u32)
}


/// `lw` (sign-extended) from a 32-bit address.
#[inline(always)]
pub(crate) fn lw_at_3(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lw(c)
}


/// The two-instruction virtual-method read the originals use:
/// `a0 = lh(vt+off)`, `func = lw(vt+off+4)`. The original leaves `func` in
/// `vreg` (v0 or v1 depending on the site), which callers can see, so the
/// register is written here too. Returns `func` for the `call_ptr`.
#[inline(always)]
pub(crate) fn vslot(c: &mut Cpu, vt: u64, off: u32, vreg: usize) -> u32 {
    let base = (vt as u32).wrapping_add(off);
    c.g[A0][0] = Addr(base).lh(c);
    let func = Addr(base.wrapping_add(4)).lw(c);
    c.g[vreg][0] = func;
    func as u32
}


/// One of the three "copy my `+0x10/+0x14/+0x18` vector into the state" calls
/// of the game-state loaders: `vt[0x10](self + 0, self + dst, 4)` with the
/// vtable at `src->+0x10`.
pub(crate) fn load_state_vector(c: &mut Cpu, vt: u64, data: u32, dst: u32, ret: u32) -> Flow {
    let func = vslot(c, vt, 0x10, V0);
    c.g[A1][0] = sx(dst);
    c.g[A2][0] = 4;
    c.g[A0][0] = sx(data.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, func, ret)
}


/// `void GameStateReset_LoadState(self, data)` (0x235a18): load the state,
/// then copy the three transform vectors from the data object.
pub fn gamestatereset_load_state(c: &mut Cpu) {
    game_state_load_state(c, 0x0023_5a34, [0x0023_5a50, 0x0023_5a6c, 0x0023_5a88]);
}


/// `void GameStateRestartRace_LoadState(self, data)` (0x235fd0): the restart
/// twin of [`gamestatereset_load_state`].
pub fn gamestaterestartrace_load_state(c: &mut Cpu) {
    game_state_load_state(c, 0x0023_5fec, [0x0023_6008, 0x0023_6024, 0x0023_6040]);
}


pub(crate) fn game_state_load_state(c: &mut Cpu, load_ret: u32, vec_rets: [u32; 3]) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        callee::GAME_STATE_LOAD_STATE.call(c, load_ret)?;
        for (i, ret) in vec_rets.into_iter().enumerate() {
            c.g[V1][0] = lw_at_3(c, c.g[S0][0] as u32);
            load_state_vector(
                c,
                c.g[V1][0],
                r32(c, S0),
                p_2(r32(c, S1), 0x10 + 4 * i as u32),
                ret,
            )?;
        }
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void GameStatePreRaceMenu_ResumeRace()` (0x236cd8): stop the script
/// list, restart the race and reset the race sounds.
pub fn gamestatepreracemenu_resume_race(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = lw_at_3(c, gw(c, -0x84c));
        c.sd(p_2(sp(c), 0), c.g[RA][0]);
        c.g[A1][0] = 0;
        call_ptr(c, callee::SCRIPT_ENGINE_LIST_STOP_3.addr, 0x0023_6cec)?;
        callee::BE_LIBRARY_GET_INSTANCE_3.call(c, 0x0023_6cf4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        callee::BE_LIBRARY_GET_INTERFACE_3.call(c, 0x0023_6d00)?;
        c.g[V1][0] = 0x530000;
        c.g[V0][0] = Addr(p_2(c.g[V1][0] as u32, 0x5c10)).lb(c);
        c.g[V0][0] ^= 4;
        let no_restart = c.g[V0][0] == 0;
        c.g[V1][0] = lw_at_3(c, gw(c, -0x488));
        if !no_restart {
            c.cycles += 11;
            c.g[A1][0] = 3;
            c.g[A2][0] = lw_at_3(c, p_2(c.g[V1][0] as u32, 0xc));
            c.g[V0][0] = lw_at_3(c, p_2(c.g[A2][0] as u32, 0xcc));
            let func = vslot(c, c.g[V0][0], 0x28, V1);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, func, 0x0023_6d30)?;
            c.g[A0][0] = lw_at_3(c, gw(c, -0x488));
            c.g[V1][0] = 1;
            c.g[V0][0] = lw_at_3(c, p_2(c.g[A0][0] as u32, 0xc));
            c.sw(p_2(c.g[V0][0] as u32, 0x14), c.g[V1][0]);
        }
        c.cycles += 7;
        callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_2.call(c, 0x0023_6d48)?;
        c.g[A0][0] = c.g[V0][0];
        callee::SSX_AUDIO_SYSTEM_RESET_RACE_SOUNDS.call(c, 0x0023_6d50)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Input sync.

/// `void InputSync_SendSample(InputSync* self, void* src)` (0x237a98): pack the
/// sample into the frame buffer, unpack it back and, when the game comms
/// queue is open, send it.
pub fn input_sync_send_sample(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        c.g[A2][0] = c.g[SP][0];
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0x20);
        c.g[S1][0] = c.g[A1][0];
        callee::INPUT_SYNC_PACK_SAMPLE.call(c, 0x0023_7abc)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[SP][0];
        callee::INPUT_SYNC_UNPACK_SAMPLE.call(c, 0x0023_7ad0)?;
        c.g[V1][0] = Addr(gp_4(c, -0x204)).lw(c);
        c.g[V0][0] = Addr(p_3(r32(c, V1), 0x40)).lw(c);
        let idle = c.g[V0][0] == 0;
        c.g[A0][0] = 0;
        if !idle {
            c.cycles += 3;
            c.g[V0][0] = Addr(p_3(r32(c, V1), 4)).lw(c);
            if c.g[V0][0] != 0 {
                c.g[A0][0] = 0;
            } else {
                c.cycles += 2;
                c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                c.g[A0][0] = (0u64 < c.g[V0][0]) as u64;
            }
        }
        let send = !(c.g[A0][0] == 0);
        c.cycles += 2;
        c.g[A0][0] = Addr(gp_4(c, -0x204)).lw(c);
        if send {
            c.cycles += 4;
            c.g[A3][0] = c.g[S2][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = c.g[SP][0];
            call_ptr(c, 0x0025_7e98, 0x0023_7b0c)?;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}


/// `void GameStateRace_Enter(self)` (0x234e20): put the AI into state 5, reset
/// the race manager unless the game was already in mode 4, then open the
/// finish screen if the rider count looks wrong.
pub fn game_state_race_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(gp_5(c, -0x488)).lw(c);
        frame.save_ra(c, 0);
        c.g[A1][0] = 5;
        c.g[A0][0] = lw_2(c, V0, 0xc);
        call_ptr(c, table_callee::AI_SET_AI_STATE, 0x0023_4e38)?;
        call_ptr(c, table_callee::BE_GET, 0x0023_4e40)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, table_callee::BE_GET_INTERFACE_FV, 0x0023_4e4c)?;
        c.g[V1][0] = 0x530000;
        c.g[A0][0] = 4;
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x5c10)).lb(c);
        let mode_four = c.g[V0][0] == c.g[A0][0];
        c.g[A1][0] = Addr(gp_5(c, -0x488)).lw(c);
        if !mode_four {
            c.cycles += 5;
            c.g[A0][0] = 1;
            c.g[V0][0] = lw_2(c, A1, 0xc);
            Addr(c.g[V0][0] as u32).off(0x14).sw(c, c.g[A0][0]);
            c.g[V1][0] = lw_2(c, A1, 0xc);
            Addr(c.g[V1][0] as u32).off(0x8c).sw(c, 0);
        }
        c.cycles += 7;
        c.g[A0][0] = 1;
        call_ptr(c, table_callee::BE_SET_STATE, 0x0023_4e7c)?;
        c.g[V1][0] = Addr(gp_5(c, -0x488)).lw(c);
        c.g[V0][0] = lw_2(c, V1, 0x28);
        c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
        let empty = c.g[V0][0] == 0;
        c.g[A0][0] = 0;
        if !empty {
            c.cycles += 1;
            c.g[A0][0] = ((c.g[V0][0] as i64) < 10) as u64;
        }
        c.cycles += 2;
        let ok = c.g[A0][0] != 0;
        frame.restore_ra(c, 0);
        if !ok {
            c.cycles += 4;
            c.g[A0][0] = lw_2(c, V1, 0x48);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, table_callee::UI_OPEN_FINISH_SCREEN, 0x0023_4ea8)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}


/// `GameStatePreRaceMenu_Enter` (0x236bb0): leave the pause overlay, bump the
/// pre-race menu counter and enter the menu state (or the quick path when a
/// BE library / world is already active).
pub fn game_state_prerace_menu_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(gp_6(c, -0x204)).lw(c);
        frame.save_ra(c, 0);
        let be = c.g[V0][0] == 0;
        Addr(p_4(r(c, A0), 8)).sw(c, 0);
        if !be {
            c.cycles += 3;
            c.g[V0][0] = Addr(p_4(r(c, V0), 4)).lw(c);
            let no_lib = c.g[V0][0] == 0;
            c.g[V0][0] = Addr(gp_6(c, -0x848)).lw(c);
            if !no_lib {
                c.cycles += 7;
                c.g[V1][0] = Addr(p_4(r(c, V0), 0x84)).lw(c);
                c.g[RA][0] = sx(0x0023_6bdc);
                c.g[A0][0] = Addr(p_4(r(c, V1), 0x64)).lw(c);
                call_ptr(c, callee::OVERLAY_RESET, 0x0023_6bdc)?;
                c.g[RA][0] = sx(0x0023_6be4);
                c.g[A0][0] = 0;
                call_ptr(c, callee::BE_SET_PHASE, 0x0023_6be4)?;
                frame.restore_ra(c, 0);
                c.cycles += 2;
                let ra = r(c, RA);
                c.g[SP][0] = sx(r(c, SP).wrapping_add(0x10));
                c.pc = ra;
                return Ok(());
            }
        }
        c.cycles += 21;
        c.g[RA][0] = sx(0x0023_6bf4);
        c.g[A0][0] = 2;
        call_ptr(c, callee::GAME_MODULE_SET_STATE, 0x0023_6bf4)?;
        c.g[A2][0] = Addr(gp_6(c, -0x69c)).lw(c);
        c.g[V1][0] = 0x0053_0000;
        c.g[V1][0] = sx(0x0053_0000u32.wrapping_add(0x66e8));
        c.g[A0][0] = 0x0044_0000;
        c.g[A2][0] = sx(r(c, A2).wrapping_add(1));
        c.g[A1][0] = Addr(p_4(r(c, A0), 0x28f4)).lw(c);
        c.g[V0][0] = shl32(c.g[A2][0], 2);
        c.g[T0][0] = Addr(gp_6(c, -0x848)).lw(c);
        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
        c.g[A3][0] = 1;
        Addr(r(c, V0)).sw(c, c.g[A3][0]);
        Addr(gp_6(c, -0x6a0)).sw(c, c.g[A1][0]);
        c.g[V0][0] = Addr(p_4(r(c, T0), 0x84)).lw(c);
        Addr(gp_6(c, -0x69c)).sw(c, c.g[A2][0]);
        c.g[RA][0] = sx(0x0023_6c34);
        c.g[A0][0] = Addr(p_4(r(c, V0), 0x64)).lw(c);
        call_ptr(c, callee::OVERLAY_RESET, 0x0023_6c34)?;
        c.g[RA][0] = sx(0x0023_6c3c);
        c.g[A0][0] = 0;
        call_ptr(c, callee::BE_SET_PHASE, 0x0023_6c3c)?;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}


/// `void GameStateEventSelect_LoadState(self, stream)` (0x2367c8): base load
/// plus four 4-byte stream fields.
pub fn game_state_event_select_load_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S1][0] = sx(this);
        callee::GAME_STATE_LOAD_STATE_2.call(c, 0x0023_67e4)?;
        for (off, ret) in [
            (0x10u32, 0x0023_6800u32),
            (0x14, 0x0023_681c),
            (0x8, 0x0023_6838),
            (0x18, 0x0023_6854),
        ] {
            c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(off));
            c.g[A2][0] = 4;
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x10).lh(c);
            c.g[V0][0] = lw_3(c, V1, 0x14);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, ret)?;
        }
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void InputSync_dtor(this, flags)` (0x2370a0): free each input record's
/// buffer and two objects.
pub fn input_sync_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x10);
        let this = abi::arg(c, 0);
        c.g[S2][0] = sx(this);
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A1][0];
        let none = (lw_3(c, S2, 0x10) as i64) <= 0;
        if !none {
            c.cycles += 3;
            c.g[S0][0] = sx(this.wrapping_add(0x14));
            c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
            'records: loop {
                c.cycles += 2;
                if c.g[A0][0] == 0 {
                    c.g[A0][0] = lw_3(c, S0, 0x10);
                } else {
                    c.cycles += 3;
                    call_ptr(c, table::EA_MEM_FREE, 0x0023_70e8)?;
                    c.g[A0][0] = lw_3(c, S0, 0x10);
                }
                c.cycles += 9;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                call_ptr(c, table::BUILTIN_DELETE, 0x0023_70f4)?;
                c.g[A0][0] = lw_3(c, S0, 0x14);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                call_ptr(c, table::BUILTIN_DELETE, 0x0023_7100)?;
                c.g[V0][0] = lw_3(c, S2, 0x10);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0023_70d8);
                    }
                    continue 'records;
                }
                break 'records;
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S3][0] & 1;
        let no_delete = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x40);
        if !no_delete {
            c.cycles += 3;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x0023_7124)?;
            frame.restore(c, S0, 0x40);
        }
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
// Input.

/// `int InputSync_PackSample(SampleSrc* src, unsigned char* dst, u8* mask)`
/// (0x237b28): copy the first sample byte to `dst[0]`, zero `dst[1..4]`, then
/// pack the `src->count` floats at `src+4` as `(int)(v * 255)` — skipping
/// zeros — into `dst+4`, recording presence bits in the `dst+1` mask.
/// Returns the packed byte count + 4.
pub fn input_sync_pack_sample(c: &mut Cpu) {
    c.cycles += 6;
    let src = c.g[A1][0];
    c.g[T3][0] = src;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
    c.g[V0][0] = Addr(src as u32).lbu(c);
    c.g[A3][0] = 2;
    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(3));
    Addr(c.g[A2][0] as u32).sb(c, c.g[V0][0]);
    loop {
        c.cycles += 7;
        Addr(c.g[V1][0] as u32).sb(c, 0);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(1));
        let more = (c.g[A3][0] as i64) >= 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0023_7b40);
        }
    }
    c.cycles += 4;
    c.g[V0][0] = Addr(c.g[T3][0] as u32).lw(c);
    c.g[T2][0] = 0;
    let no_samples = (c.g[V0][0] as i64) <= 0;
    c.g[T0][0] = 0;
    if !no_samples {
        c.cycles += 7;
        c.g[AT][0] = 0x437f_0000;
        c.mtc1(1, 1);
        c.g[T4][0] = c.g[A0][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(4));
        c.g[T5][0] = 1;
        c.g[T1][0] = sx((c.g[T3][0] as u32).wrapping_add(4));
        loop {
            c.cycles += 7;
            c.lwc1(0, c.g[T1][0] as u32);
            c.mul_s(0, 0, 1);
            c.cvt_w_s(2, 0);
            c.mfc1(2, 2);
            c.g[A3][0] = c.g[V0][0] & 0xff;
            let zero = c.g[A3][0] == 0;
            c.g[A0][0] = sx(((c.g[T2][0] as u32) as i32 >> 3) as u32);
            if !zero {
                c.cycles += 9;
                c.g[V1][0] = c.g[T2][0] & 7;
                c.g[A0][0] = sx((c.g[T4][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = sx((c.g[T5][0] as u32) << ((c.g[V1][0] as u32) & 31));
                c.g[V0][0] = Addr(c.g[A0][0] as u32).lbu(c);
                c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                Addr(c.g[A0][0] as u32).sb(c, c.g[V0][0]);
                Addr(c.g[A1][0] as u32).sb(c, c.g[A3][0]);
            }
            c.cycles += 5;
            c.g[V0][0] = Addr(c.g[T3][0] as u32).lw(c);
            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[T2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0023_7b88);
            }
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(4));
    c.pc = ra;
}


// ---------------------------------------------------------------------------
// `GameStateEventSwitchLoad_Enter` (0x002368a0).
//
// `void GameStateEventSwitchLoad::Enter(self)`: load the game-state data and
// start playing its event tracks.

pub fn game_state_event_switch_load_enter(c: &mut Cpu) {
    guest(c, |c| {

        'b_236944: {
        c.cycles += 22;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S3][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[RA][0] = sx(0x002368c4);
        c.g[A0][0] = 0u64;
        call_ptr(c, 0x0014de68, 0x002368c4)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
        c.g[RA][0] = sx(0x002368d0);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
        call_ptr(c, 0x0026f228, 0x002368d0)?;
        c.g[RA][0] = sx(0x002368d8);
        call_ptr(c, 0x0014dc80, 0x002368d8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x002368e4);
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x0014dd58, 0x002368e4)?;
        c.g[V1][0] = 0x530000;
        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
        c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x49));
        if c.g[V0][0] != 0u64 {
        c.sw((c.g[S3][0] as u32).wrapping_add(0x10), 0u64);
        break 'b_236944;
        } else {
        c.cycles += 9;
        c.g[RA][0] = sx(0x00236900);
        c.g[S0][0] = 0u64;
        call_ptr(c, 0x0014dc80, 0x00236900)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0023690c);
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x0014dd58, 0x0023690c)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
        if (c.g[V0][0] as i64) <= 0 {
        c.sw((c.g[S3][0] as u32).wrapping_add(0x10), 0u64);
        break 'b_236944;
        } else {
        c.cycles += 1;
        c.g[A1][0] = c.g[S0][0];
        'l_236920: loop {
        c.cycles += 8;
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = 0u64;
        c.g[RA][0] = sx(0x00236930);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
        call_ptr(c, 0x00145cb0, 0x00236930)?;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let t = c.g[V0][0] != 0u64;
        c.g[A1][0] = c.g[S0][0];
        if t {
        if crate::events::event_due(c) { crate::events::deliver_event_here(c, 0x00236920); }
        continue 'l_236920;
        } else {
        c.cycles += 1;
        c.sw((c.g[S3][0] as u32).wrapping_add(0x10), 0u64);
        break 'b_236944;
        }
        }
        }
        }
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t; return Ok(());

        Ok(())
    })
}


/// `InputSync_UnpackSample(this, src, dst)` (0x237be8): copy the packed
/// sample's bit count and expand up to 24 channels of 8-bit values into
/// floats, zeroing the channels whose mask bit is clear.
pub fn input_sync_unpack_sample(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let this = abi::arg(c, 0);
        let src = abi::arg(c, 1);
        let dst = abi::arg(c, 2);
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = abi::arg64(c, 2);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = abi::arg64(c, 1);
        frame.save_ra(c, 0);
        c.g[S2][0] = sx(src.wrapping_add(4));
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S2][0];
        c.g[V0][0] = Addr(dst).lbu(c);
        c.g[A2][0] = 0x60;
        Addr(src).sw(c, c.g[V0][0]);
        callee::EA_MEM_SET.call(c, 0x0023_7c20)?;

        c.g[V0][0] = Addr(src).lw(c);
        c.g[T0][0] = 0;
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 4;
            c.lwc1(1, gp(c, -0x5160));
            c.g[T1][0] = sx(src.wrapping_add(4));
            c.g[A3][0] = c.g[S2][0];
            c.g[A2][0] = sx(src.wrapping_add(1));
            loop {
                c.cycles += 8;
                c.g[V0][0] = sx((((c.g[A1][0] as u32) as i32) >> 3) as u32);
                c.g[A0][0] = c.g[A1][0] & 0x7;
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lbu(c);
                let sh = (c.g[A0][0] as u32) & 31;
                c.g[V1][0] = sx((((c.g[V1][0] as u32) as i32) >> sh) as u32);
                c.g[V1][0] = c.g[V1][0] & 1;
                let clear = c.g[V1][0] == 0;
                c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[T0][0] as u32));
                if clear {
                    c.cycles += 1;
                    Addr(c.g[A3][0] as u32).sw(c, 0);
                } else {
                    c.cycles += 7;
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).lbu(c);
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
                    c.mtc1(3, 0);
                    c.cvt_s_w(0, 0);
                    c.mul_s(0, 0, 1);
                    c.swc1(0, abi::r32(c, A3));
                }
                c.cycles += 5;
                c.g[V0][0] = Addr(src).lw(c);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0023_7c40);
                }
            }
        }

        c.cycles += 6;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        let _ = this;
        Ok(())
    })
}


/// `int InputSync_ApplyInput(InputSync *self)` (0x2379c8): when the sync is
/// ready, feed every pending input slot to its handler.
pub fn inputsync_apply_input(c: &mut Cpu) {
    guest(c, |c| {
        'b_237a78: {
            'b_237a0c: {
                'b_237a04: {
                    c.cycles += 11;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x70));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 14i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S1][0] = c.g[A0][0];
                    if t {
                        break 'b_237a04;
                    } else {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x002379fc);
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x00237950, 0x002379fc)?;
                        if c.g[V0][0] != 0u64 {
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                            break 'b_237a0c;
                        } else {
                            break 'b_237a04;
                        }
                    }
                }
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0u64;
                if t {
                    break 'b_237a78;
                } else {
                    break 'b_237a0c;
                }
            }
            'b_237a74: {
                c.cycles += 2;
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[S2][0] = 0u64;
                if t {
                    break 'b_237a74;
                } else {
                    c.cycles += 3;
                    c.g[S3][0] = 0x64;
                    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x14));
                    'l_237a20: loop {
                        'b_237a38: {
                            c.cycles += 6;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                            c.div(0, 4, 2);
                            if c.g[V0][0] == 0u64 {
                                break 'b_237a38;
                            } else {
                                break 'b_237a38;
                            }
                        }
                        c.cycles += 15;
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32));
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        c.g[V1][0] = c.hi[0];
                        c.mult(0, 3, 3, 19);
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                        c.g[A1][0] = c.lw((c.g[A2][0] as u32));
                        c.g[RA][0] = sx(0x00237a58);
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
                        call_ptr(c, 0x00321298, 0x00237a58)?;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00237a20);
                            }
                            continue 'l_237a20;
                        } else {
                            break 'b_237a74;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0x1;
            break 'b_237a78;
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// Above 60 Hz the countdown's light check runs once per countdown value:
    /// a repeated positive value returns at once (nothing read past the
    /// counter, no cycles), a new one runs the body.
    #[test]
    fn game_state_countdown_update_once_per_value() {
        let mut c = crate::difftest::fresh();
        crate::tick::set_for_test(&mut c, 240);
        c.g[GP][0] = GAME_GP as u64;
        let world = DATA;
        let race = DATA + 0x100;
        c.bus.write32(GAME_GP - 0x488, world);
        c.bus.write32(world + 0xc, race);
        c.bus.write32(race + 0x1c, 120);
        // The first sight of a value runs the body; mark it as seen without
        // running it (this test's world is not a game).
        assert!(crate::tick::value_changed(&mut c, race + 0x1c, 120));
        let cycles = c.cycles;
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = 0x0023_4c68;
        game_state_countdown_update(&mut c);
        assert_eq!((c.pc, c.cycles), (crate::difftest::RA, cycles));
        assert!(crate::tick::value_changed(&mut c, race + 0x1c, 119));
        crate::tick::set_for_test(&mut c, 60);
        assert!(crate::tick::value_changed(&mut c, race + 0x1c, 119), "always at 60");
    }
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::check_paths;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fill;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::put_vec;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::put_words;
    use crate::difftest::RA as TEST_RA;
    use ssx3_ee as ee;
    use crate::difftest::fakes::*;
    use crate::difftest::SP;
    use ssx3_ee_rt::funcs::GuestFn;
    use crate::difftest::SP as STACK;
    use crate::difftest::fakes;
    use crate::difftest::RA as RET;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::RA as SENTINEL;
    pub(crate) use crate::game::bx::bxstring2::tests::be_library;
    pub(crate) use crate::game::dirtysock::tags2::tests::rng_2 as rng;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_LIBRARY;
    pub(crate) use crate::game::movie::movieplayer::tests::globals;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS;
    pub(crate) use crate::game::dirtysock::tags2::tests::zero;
    pub(crate) use crate::game::fe::festaterewards2::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::fe::festaterewards2::tests::w;
    pub(crate) use crate::game::fe::festaterewards2::tests::belib;
    pub(crate) use crate::game::dirtysock::tags2::tests::be_library_2;
    pub(crate) use crate::game::fe::festatestore::tests::world;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_2 as pool_allocator;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::gp_at;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::dirtysock::tags2::tests::base;
    pub(crate) use crate::game::fe::festateoptions::tests::AUX;
    pub(crate) use crate::game::fe::festateoptions::tests::rng as rng_2;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::put;
    pub(crate) use crate::game::fe::festateoptions::tests::setup;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::gpf;
    pub(crate) use crate::game::main::gamemode::tests::world as world_2;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_3 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_4 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_6 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_3 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_2;
    pub(crate) use crate::game::fe::festaterewards::tests::w16;
    pub(crate) use crate::game::fe::festaterewards::tests::slot_2 as slot_3;
    pub(crate) use crate::game::fe::festaterewards::tests::globals_3;
    pub(crate) use crate::game::fe::festaterewards::tests::pool_4 as pool_2;
    pub(crate) use crate::game::fe::festaterewards::tests::heap_init_3 as heap_init;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_4 as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_3 as W;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_8 as slot_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::eafile_pools;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_7 as run;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_8 as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_9 as pool_allocator_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_3 as alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_9 as globals_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_7 as pool_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_6;
    pub(crate) use crate::game::fe::festateloadscreen::tests::r32_seed;
    pub(crate) use crate::game::fe::ovstates::tests::base_3 as base_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM_3 as RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE_3;
    pub(crate) use crate::game::be::bedbprofile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_8;


    #[test]
    fn game_state_reset_ctor_matches() {
        check(ssx3_ee::s06::f_00234ef8, game_state_reset_ctor, 64, &|c, seed| {
            let mut r = Rng(0x0c11_000f ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.bus.write32(GAME_GP.wrapping_sub(0x488), r.next_u32());
            for i in 0..8 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }


    #[test]
    fn game_state_event_select_ctor_matches() {
        check(ssx3_ee::s06::f_00236208, game_state_event_select_ctor, 64, &|c, seed| {
            let mut r = Rng(0x0c11_0010 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.bus.write32(GAME_GP.wrapping_sub(0x488), r.next_u32());
            for i in 0..8 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }


    #[test]
    fn game_state_event_switch_load_ctor_matches() {
        check(ssx3_ee::s06::f_00236868, game_state_event_switch_load_ctor, 64, &|c, seed| {
            let mut r = Rng(0x0c11_0011 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.bus.write32(GAME_GP.wrapping_sub(0x488), r.next_u32());
            for i in 0..4 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }


    #[test]
    fn input_sync_ctor_matches() {
        check_lohi(ssx3_ee::s06::f_00237060, input_sync_ctor, 300, &|c, seed| {
            let mut r = Rng(0x0c11_0012 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.g[A2][0] = match seed % 5 {
                0 => 0,
                1 => 1,
                2 => (r.next_u32() & 0xffff) as u64,
                3 => 0xffff_ffff,
                _ => ((r.next_u32() as u64) << 32) | r.next_u32() as u64,
            };
            c.g[A3][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
        });
    }


    

    pub(crate) fn rand_setup(c: &mut Cpu, seed: u32, state: u32) {
        let mut r = machine(c, seed, 0x237c_d8, 0);
        for i in 0..8u32 {
            let v = if seed % 4 == 0 { 0 } else { r.next_u32() };
            c.bus.write32(state + 4 * i, v);
        }
    }


    #[test]
    fn game_rand_next_matches() {
        check(ee::s06::f_00237cd8__generated, game_rand_next, 200, &|c, seed| {
            rand_setup(c, seed, 0x004c_9548);
        });
    }


    #[test]
    fn game_state_results_exit_matches() {
        crate::difftest::check(ee::s06::f_00236ea0__generated, game_state_results_exit, 40, &|c, seed| {
            machine(c, seed, 0x236e_a0, 2);
            be_library(c);
            // `D_004A2C68` (the game state manager): a dummy whose `+0x48`
            // state list head is empty, so the close-all walk stops at once.
            c.bus.write32(GAME_GP.wrapping_sub(0x488), DATA + 0x2000);
            c.bus.write32(DATA + 0x2048, if seed % 2 == 0 { 0 } else { DATA + 0x2000 });
            c.g[A0][0] = sx(DATA);
        });
    }


    #[test]
    fn game_rand_seed_matches() {
        crate::difftest::check(ee::s06::f_00237cb0__generated, game_rand_seed, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x237c_b0, 1);
            c.g[A0][0] = if seed % 3 == 0 { 0 } else { r.next_u32() as u64 };
        });
    }


    #[test]
    fn game_state_restart_race_exit_matches() {
        check(ssx3_ee::s06::f_00235f20__generated, game_state_restart_race_exit, 200, &|c, seed| {
            let mut r = rng(seed, 0x1d);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let world = DATA + 0x8000;
            let race = DATA + 0x9000;
            let rider = DATA + 0xa000;
            c.bus.write32(GAME_GP - 0x848, world);
            c.bus.write32(world + 0x84, race);
            c.bus.write32(race + 0x94, rider);
            c.bus.write32(rider + 4, r.next_u32());
            c.bus.write32(rider + 8, r.next_u32());
        });
    }


    /// The replay-playback world: a world whose race manager owns an empty
    /// playback record and an empty world-script manager. The BE library
    /// singleton is a zeroed dummy and its interface slot points at a leaf
    /// function, so `WScriptMan_SpawnMissions` takes its early return.
    pub(crate) fn replay_exit_setup(c: &mut Cpu, seed: u32) {
        let mut r = rng(seed, 0x1e);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        let world = DATA + 0x8000;
        let race = DATA + 0x9000;
        let playback = DATA + 0xb000;
        let wscript = DATA + 0xa000;
        c.bus.write32(GAME_GP - 0x848, world);
        c.bus.write32(world + 0x84, race);
        c.bus.write32(race + 0x34, playback);
        // PlaybackMan_FreeData: a -1 block handle skips the release, and a
        // zeroed record takes the null-list path down to the null frees.
        c.bus.write32(playback + 0x78, u32::MAX);
        c.bus.write32(playback + 0x4c, 0);
        c.bus.write32(playback + 0x50, 0);
        c.bus.write32(playback + 0x58, 0);
        // WScriptMan_DeleteAllProcesses: empty process lists.
        c.bus.write32(GAME_GP + 0xce8, wscript);
        // BELibrary_GetInstance returns the existing singleton.
        c.bus.write32(GAME_GP - 0x1f04, DATA + 0xc000);
        // BELibrary_GetInterface: index 0 -> lib -> vtable slot 8 -> the
        // camera-trigger-stack clear leaf, which only writes its argument.
        const BE_LIBRARY: u32 = 0x0053_08a8;
        let lib = DATA + 0xd000;
        let vt = DATA + 0xe000;
        c.bus.write32(GAME_GP - 0x1efc, 0);
        c.bus.write32(BE_LIBRARY, lib);
        c.bus.write32(lib + 4, vt);
        c.bus.write16(vt + 8, 0);
        c.bus.write32(vt + 0xc, 0x0016_d200);
        // WScriptMan_SpawnMissions leaves early unless the world type is 4.
        c.bus.write8(0x0053_5c10, (seed % 3) as u8);
    }


    #[test]
    fn game_state_replay_playback_exit_matches() {
        check(ssx3_ee::s06::f_00237038__generated, game_state_replay_playback_exit, 60, &|c, seed| {
            replay_exit_setup(c, seed)
        });
    }


    #[test]
    fn game_state_warp_to_event_update_matches() {
        check(
            ssx3_ee::s06::f_00236198__generated,
            game_state_warp_to_event_update,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0xb3, 1);
                globals(c);
            },
        );
    }


    /// The three script-list helpers: engine + rider index (+ payload).
    pub(crate) fn script_list_test(generated: GuestFn, hand: GuestFn, salt: u64, payload: bool) {
        check(generated, hand, 40, &|c, seed| {
            let mut r = machine(c, seed, salt, 1);
            globals(c);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 3) as u64;
            if payload {
                c.g[A2][0] = (seed % 2) as u64;
            }
            // The rider script-list array.
            c.bus.write32(DATA + 0x54c, DATA + 0x4000);
            let _ = r.next_u32();
        });
    }


    #[test]
    fn script_engine_list_prepare_matches() {
        script_list_test(
            ssx3_ee::s07::f_00278f38__generated,
            script_engine_list_prepare,
            0xca,
            false,
        );
    }


    #[test]
    fn game_state_reset_ai_exit_matches() {
        check(ssx3_ee::s06::f_00236b38__generated, game_state_reset_ai_exit, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x36b38, 1);
            let state = DATA + 0x400;
            let ai = DATA + 0x500;
            let vt = DATA + 0x600;
            c.bus.write32(GAME_GP.wrapping_sub(0x488), state);
            c.bus.write32(state + 0xc, ai);
            c.bus.write32(ai + 0xcc, vt);
            c.bus.write16(vt + 0x28, ((r.next_u32() % 0x40) * 4) as u16);
            c.bus.write32(vt + 0x2c, FAKE);
            c.g[A0][0] = sx(DATA + 0x300);
        });
    }


    #[test]
    fn game_state_finish_cinematic_enter_matches() {
        check(
            ee::s06::f_00236f08__generated,
            game_state_finish_cinematic_enter,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x236f_08, 0);
                zero(c, DATA + 0x400, 0x40);
                c.bus.write32(
                    GAME_GP.wrapping_add(0x410),
                    if seed % 2 == 0 { 0 } else { DATA + 0x400 },
                );
                c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x1000);
                zero(c, DATA + 0x1000, 0x400);
                c.bus.write32(DATA + 0x1000 + 0x84, DATA + 0x1100);
                c.bus.write32(DATA + 0x1100 + 0xc, DATA + 0x1200);
            },
        );
    }


    #[test]
    fn game_state_reset_overlay_exit_matches() {
        check(ssx3_ee::s06::f_00235570__generated, game_state_reset_overlay_exit, 60, &|c, seed| {
            machine(c, seed, 0x2355_70, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
        });
    }


    #[test]
    fn game_state_results_update_matches() {
        check(ssx3_ee::s06::f_00236e60__generated, game_state_results_update, SEEDS_2, &|c, seed| {
            machine(c, seed, 0x236e_60, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            w(c, DATA + 8, if seed % 2 == 0 { 1 } else { 0 });
        });
    }


    // ------------------------------------------------------------------
    // Game state / mode dispatch.

    #[test]
    fn game_state_pre_race_menu_update_matches() {
        check(
            ee::s06::f_00236c48__generated,
            game_state_pre_race_menu_update,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x236c48, 0);
                let mode = seed % 4;
                let self_at = DATA + 0x200;
                c.g[A0][0] = self_at as u64;
                c.bus.write32(self_at, 0);
                c.bus
                    .write32(GAME_GP - 0x204, if mode < 2 { 0 } else { DATA + 0x300 });
                c.bus.write32(DATA + 0x304, (mode == 2) as u32);
                c.bus.write32(self_at + 8, (mode % 2 == 0) as u32);
            },
        );
    }


    #[test]
    fn game_state_replay_playback_enter_matches() {
        check(
            ssx3_ee::s06::f_00236fe8__generated,
            game_state_replay_playback_enter,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x236fe8, 0);
                be_library_2(c);
                pool_allocator(c);
                world(c);
                // No script manager installed: both script calls take their empty
                // path.
                c.bus.write32(GAME_GP.wrapping_add(0xce8), 0);
            },
        );
    }


    #[test]
    fn game_state_finish_cinematic_update_matches() {
        check(
            ee::s06::f_00236f40__generated,
            game_state_finish_cinematic_update,
            SEEDS_3,
            &|c, seed| {
                let _ = machine(c, seed, 0x236f_40, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x20);
                c.g[A0][0] = sx(this);
                clear(c, DATA + 0x400, 0x40);
                c.bus.write32(gp_at(-0x84c), if seed % 3 == 0 { 0 } else { DATA + 0x400 });
                c.bus.write32(LOG_RESULTS, 5);
            },
        );
    }


    // -------------------------------------------------------- game states

    #[test]
    fn game_state_pre_race_menu_exit_matches() {
        check(ee::s06::f_00236c88__generated, game_state_pre_race_menu_exit, 60, &|c, seed| {
            let mut r = base(c, seed, 0x236c);
            let depth = 1 + r.next_u32() % 4;
            c.bus.write32(gp_at(-0x69c), depth);
            for i in 0..4u32 {
                c.bus.write32(0x0053_66e8 + 4 * i, (i * 3 + seed) % 16);
            }
            for i in 0..16u32 {
                c.bus.write32(0x0044_28f0 + 4 * i, r.next_u32());
            }
            c.bus.write32(gp_at(-0x6a0), 0xdead);
            c.g[A0][0] = sx(DATA);
        });
    }


    #[test]
    fn game_state_reset_v08_matches() {
        check(ee::s06::f_002355b0__generated, game_state_reset_v08, 200, &|c, seed| {
            setup(c, seed, 19);
            c.g[A0][0] = AUX as u64;
            put(c, AUX + 0x10, [5, 4, 6][(seed % 3) as usize]);
        });
    }


    #[test]
    fn game_state_finish_cinematic_exit_matches() {
        check(ee::s06::f_00236f90__generated, game_state_finish_cinematic_exit, 200, &|c, seed| {
            let mut r = setup(c, seed, 20);
            world_2(c, 0);
            put(c, AUX + 0x300 + 0x8c, r.next_u32());
        });
    }


    #[test]
    fn game_state_reset_update_matches() {
        check(ssx3_ee::s06::f_002357f8__generated, game_state_reset_update, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x2357f8, 0);
            let s = DATA + 0x1000;
            let engine = DATA + 0x2000;
            let world = DATA + 0x3000;
            let race = DATA + 0x4000;
            let arr = DATA + 0x5000;
            let table = DATA + 0x6000;
            c.g[A0][0] = s as u64;
            w32(c, s + 0x1c, if seed % 2 == 0 { 0 } else { 1 });
            w32(c, s, 0); // GotoNext takes its early-out
            w32(c, GAME_GP.wrapping_sub(0x84c), engine);
            w32(c, GAME_GP.wrapping_sub(0x848), world);
            w32(c, engine + 0x550, 0);
            w32(c, engine + 0x54c, table);
            w32(c, table + 0xcc + 0xa4, 0); // GetPlayStatus short path
            w32(c, world + 0x84, race);
            w32(c, race + 0x78, arr);
            w32(c, arr + 0x1a4, 0);
        });
    }


    #[test]
    fn input_sync_is_ready_matches() {
        check(ee::s06::f_00237950__generated, input_sync_is_ready, SEEDS_4, &|c, seed| {
            machine(c, seed, 0x23_7950, 0);
            w(c, GAME_GP.wrapping_sub(0x204), DATA + 0x3000);
            w(c, DATA + 0x3000 + 0x70, (seed % 20) as u32);
            w(c, DATA + 0x3000 + 0x3f8, (seed % 3 == 0) as u32);
            w(c, DATA + 0x3000 + 0x3fc, if seed % 3 == 1 { 2 } else { 0 });
            c.g[A0][0] = (DATA + 0x2000) as u64;
            let n = ((seed / 3) % 4) as u32;
            w(c, DATA + 0x2000 + 0x10, n);
            for i in 0..4u32 {
                let s = DATA + 0x2000 + 0x14 + 0x18 * i;
                w(c, s + 4, 1 + seed % 3);
                let full = i + 1 == n && seed % 5 == 0;
                w(c, s + 8, if full { 1 + seed % 3 } else { 2 });
            }
            c.pc = 0x0023_7950;
        });
    }


    #[test]
    fn game_state_play_script_event_matches() {
        check(
            ee::s06::f_00234be8__generated,
            game_state_play_script_event,
            40,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x234b, 3);
                let m = DATA + 0x1000;
                let idx = (seed % 4) as u32;
                // The script table slot decides whether the event is called.
                w(
                    c,
                    m + 0x3cc + 4 * idx,
                    if seed % 2 == 0 { 0 } else { DATA + 0x2000 },
                );
                w(c, DATA + 0x2000 + 8, if seed % 3 == 0 { 5 } else { 0 });
                w(
                    c,
                    GAME_GP.wrapping_add(0xce8),
                    if seed % 2 == 0 { 0 } else { m },
                );
                fill(c, &mut r, DATA + 0x2000, 8);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x200);
            },
        );
    }


    pub(crate) const SEEDS_5: u32 = 100;


    /// Scrambled machine with the two singletons every game path expects: a
    /// non-null BELibrary (so it never builds itself) and a dummy world.
    pub(crate) fn base_3(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        c.bus.write32(GAME_GP - 0x1f04, DATA + 0xa000);
        c.bus.write32(GAME_GP - 0x848, DATA + 0x9000);
        c.bus.write32(DATA + 0x9000 + 0x84, DATA + 0x9400);
        r.next_u32();
        r
    }


    /// A `{ short delta; void* fn; }` vtable slot_2 with a chosen delta.
    pub(crate) fn slot_2(c: &mut Cpu, vt: u32, off: u32, delta: u16, func: u32) {
        c.bus.write16(vt + off, delta);
        c.bus.write32(vt + off + 4, func);
    }


    #[test]
    fn game_state_load_state_matches() {
        for (gen, hand) in [
            (
                ee::s06::f_00235a18__generated as fn(&mut Cpu),
                gamestatereset_load_state as fn(&mut Cpu),
            ),
            (ee::s06::f_00235fd0__generated, gamestaterestartrace_load_state),
        ] {
            covered(gen, hand, SEEDS_5, &|c, seed| {
                base_3(c, seed, 0x35a18, 4);
                let state = DATA + 0x100;
                let data = DATA + 0x200;
                c.g[A0][0] = sx(state);
                c.g[A1][0] = sx(data);
                let vtobj = DATA + 0x300;
                c.bus.write32(data, vtobj);
                slot_2(c, vtobj, 0x10, 0, FAKE);
            });
        }
    }


    #[test]
    fn gamestatepreracemenu_resume_race_matches() {
        covered(
            ee::s06::f_00236cd8__generated,
            gamestatepreracemenu_resume_race,
            SEEDS_5,
            &|c, seed| {
                base_3(c, seed, 0x36cd8, 3);
                let engine = DATA + 0x800;
                let obj = DATA + 0x300;
                let mgr = DATA + 0x500;
                let vtobj = DATA + 0x600;
                c.bus.write32(GAME_GP - 0x84c, engine);
                c.bus.write32(GAME_GP - 0x488, obj);
                c.bus.write32(obj + 0xc, mgr);
                c.bus.write32(mgr + 0xcc, vtobj);
                slot_2(c, vtobj, 0x28, 0, FAKE);
                c.bus.write8(0x0053_5c10, if seed % 2 == 0 { 4 } else { 0 });
            },
        );
    }

    // ------------------------------------------------------------------
    // Game state / world.

    #[test]
    fn game_state_race_enter_matches() {
        check(
            ee::s06::f_00234e20__generated,
            game_state_race_enter,
            40,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x234e20, 3);
                let gs = DATA + 0x1000;
                let ai = DATA + 0x2000;
                let riders = DATA + 0x4000;
                let scr = DATA + 0x5000;
                w(c, GAME_GP.wrapping_sub(0x488), gs);
                w(c, gs + 0xc, ai);
                w(c, gs + 0x28, riders);
                w(
                    c,
                    riders,
                    if seed % 3 == 0 { 0 } else { (seed % 12) as u32 },
                );
                w(c, gs + 0x48, scr);
                w(
                    c,
                    0x0053_5c10,
                    if seed % 2 == 0 { 4 } else { (seed % 8) as u32 },
                );
                let _ = r.next_u32();
            },
        );
    }


    #[test]
    fn game_state_prerace_menu_enter_matches() {
        check(
            ee::s06::f_00236bb0__generated,
            game_state_prerace_menu_enter,
            SEEDS,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                let be = DATA + 0x800;
                match seed % 3 {
                    0 => w(c, gp_6(c, -0x204), 0),
                    1 => {
                        w(c, gp_6(c, -0x204), be);
                        w(c, be + 4, 0);
                    }
                    _ => {
                        w(c, gp_6(c, -0x204), be);
                        w(c, be + 4, DATA + 0x900);
                    }
                }
                w(c, W + 0x1000 + 0x64, 0);
                w(c, DATA + 0x900 + 0x64, 0);
                c.g[A0][0] = this as u64;
            }),
        );
    }


    #[test]
    fn game_state_event_select_load_state_matches() {
        check(ssx3_ee::s06::f_002367c8__generated, game_state_event_select_load_state, 10, &|c, seed| {
            let _r = base_6(c, seed, 0x2367c8, 2);
            let this = DATA + 0x1000;
            let stream = DATA + 0x2000;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = stream as u64;
            w_2(c, stream, DATA + 0x3000);
            c.bus.write16(DATA + 0x3000 + 0x10, 0);
            w_2(c, DATA + 0x3000 + 0x14, 0);
            w_2(c, this + 0x10, r32_seed(seed));
        });
    }


            // ---------------------------------------------------------------- Input.

    #[test]
    fn input_sync_pack_sample_matches() {
        check(ee::s06::f_00237b28__generated, input_sync_pack_sample, 200, &|c, seed| {
            let mut r = base_7(c, seed, 0x23_7b28);
            let src = DATA + 0x1000;
            let dst = DATA + 0x2000;
            let n = (seed % 6) as u32;
            c.g[A1][0] = sx(src);
            c.g[A2][0] = sx(dst);
            c.g[A0][0] = sx(dst + 1);
            c.bus.write32(src, n);
            for i in 0..6u32 {
                let zero = seed % 3 == 0 && i < 2;
                let v = if zero { 0 } else { r.edge_f32() };
                c.bus.write32(src + 4 + i * 4, v);
            }
            for i in 0..8u32 {
                c.bus.write32(dst + i * 4, r.next_u32());
            }
        });
    }


    #[test]
    fn input_sync_unpack_sample_matches() {
        check(
            ee::s06::f_00237be8__generated,
            input_sync_unpack_sample,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x237b, 0);
                let src = DATA + 0x2000;
                let dst = DATA + 0x2400;
                c.g[A0][0] = 0;
                c.g[A1][0] = src as u64;
                c.g[A2][0] = dst as u64;
                let count = (seed % 5) as u32;
                c.bus.write8(src, count as u8);
                for i in 0..4 {
                    c.bus.write8(src + 1 + i, (seed >> i) as u8);
                    c.bus.write8(src + 4 + i, 0x10 + i as u8);
                }
            },
        );
    }


    #[test]
    fn inputsync_apply_input_matches() {
        strict(
            ee::s06::f_002379c8__generated,
            inputsync_apply_input,
            60,
            &|c, seed| {
                let _r = base_8(c, seed, 0x2379);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(gp_at(-0x204), DATA + 0x3000);
                c.bus
                    .write32(DATA + 0x3000 + 0x70, if seed % 4 == 0 { 0 } else { 14 });
                c.bus.write32(DATA + 0x3000 + 0x3f8, 0);
                c.bus.write32(DATA + 0x3000 + 0x3fc, 0);
                let count = (seed % 3) as u32;
                c.bus.write32(DATA + 0x10, count);
                c.bus.write32(DATA + 0x4, 1);
                for i in 0..3u32 {
                    let slot = DATA + 0x14 + 0x18 * i;
                    c.bus.write32(slot, DATA + 0x4000);
                    c.bus.write32(slot + 4, 0);
                    c.bus.write32(slot + 8, 1);
                    c.bus.write32(slot + 0x10, DATA + 0x5000);
                }
                c.bus.write32(DATA + 0x4000, 0x11);
            },
        );
    }


    // ------------------------------------------------------------------
    // GameStateReset_OverlayUpdate / GameStateRestartRace_Update.

    /// The reset/restart fixture's base (above the fakes' result pointers,
    /// `DATA..DATA + 0x4_0000`, and their log at `DATA + 0x10_0000`).
    const RS: u32 = DATA + 0x20_0000;
    const RS_ENGINE: u32 = RS + 0x1000;
    const RS_LISTS: u32 = RS + 0x2000;
    const RS_WORLD: u32 = RS + 0x3000;
    const RS_RACE: u32 = RS + 0x3100;
    const RS_RIDER: u32 = RS + 0x3200;
    const RS_GM: u32 = RS + 0x4000;
    const RS_AI: u32 = RS + 0x4100;
    const RS_OBJ: u32 = RS + 0x4400;
    const RS_RENDER: u32 = RS + 0x4500;
    const RS_LOAD: u32 = RS + 0x4600;
    const RS_MODE: u32 = RS + 0x5000;
    const RS_AUDIO: u32 = RS + 0x6000;
    const RS_AUDIO_VT: u32 = RS + 0xc000;
    const RS_PENDING: u32 = RS + 0xd000;

    /// The script engine (`gp-0x84c`): lists 0 and 1 not loading (state
    /// 2, so `ScriptList_Prepare` returns at once) and full (so the adds
    /// push nothing), no async load polled; the BE library dummy.
    fn rs_engine(c: &mut Cpu, pending: bool) {
        be_library(c);
        c.bus.write32(GAME_GP - 0x84c, RS_ENGINE);
        c.bus.write32(RS_ENGINE + 0x54c, RS_LISTS);
        c.bus.write32(RS_ENGINE + 0x550, 2);
        c.bus.write32(RS_ENGINE + 0x620, 0);
        c.bus.write32(RS_ENGINE + 0x538, if pending { RS_PENDING } else { 0 });
        c.bus.write32(RS_ENGINE + 0x630 + 16, 0);
        for i in 0..2 {
            let list = RS_LISTS + 0xcc * i;
            c.bus.write32(list + 0xa0, 1);
            c.bus.write32(list + 0xa4, 2);
            c.bus.write32(list + 0xbc, 0);
            c.bus.write32(list + 0xc0, 0);
        }
    }

    /// The AI (`gp-0x488 -> +0xc`, also `world -> +0x84 -> +0xc`): no
    /// riders, or one rider without a render slot (`computer`/`actor`), or
    /// one rider not loaded yet (`+0x880` = 8: `Rider_LoadStep` does nothing).
    fn rs_ai(c: &mut Cpu, computer_unslotted: bool, actor_unslotted: bool, unloaded: bool) {
        c.bus.write32(GAME_GP - 0x488, RS_GM);
        c.bus.write32(RS_GM + 0xc, RS_AI);
        c.bus.write32(RS_OBJ + 0x18, RS_RENDER);
        c.bus.write32(RS_RENDER + 0x888, u32::MAX);
        c.bus.write32(RS_LOAD + 0x880, 8);
        if computer_unslotted {
            c.bus.write32(RS_AI + 0x80, 1);
            c.bus.write32(RS_AI + 0x48, RS_OBJ);
        }
        if actor_unslotted {
            c.bus.write32(RS_AI + 0x84, 1);
            c.bus.write32(RS_AI + 0x5c, RS_OBJ);
        }
        if unloaded {
            c.bus.write32(RS_AI + 0x78, 1);
            c.bus.write32(RS_AI + 0x28, RS_LOAD);
        }
    }

    fn reset_overlay_setup(c: &mut Cpu, seed: u32) {
        // The step jump table (0x47c060) and the game's data.
        crate::difftest::elf_image(c);
        let mut r = machine(c, seed, 0x2350_80, 6);
        let step = seed % 7;
        let v = seed / 7;
        c.pc = 0x0023_5080;
        c.g[A0][0] = sx(RS);
        c.bus.write32(RS + 0x10, step);
        c.bus.write32(RS + 0x14, if v % 2 == 0 { 0 } else { r.next_u32() | 1 });
        let counter = match v % 5 {
            0 => 178,
            1 => 179,
            2 => 180,
            3 => r.next_u32() % 400,
            _ => r.next_u32(),
        };
        c.bus.write32(RS + 0x18, counter);
        rs_engine(c, step == 0 && v % 4 == 3);
        c.bus.write32(GAME_GP - 0x848, RS_WORLD);
        c.bus.write32(RS_WORLD + 0x84, RS_RACE);
        c.bus.write32(RS_RACE + 0x78, RS_RIDER);
        c.bus.write32(RS_RIDER + 0x1bc, 13 + r.next_u32() % 5);
        c.bus.write32(RS_RIDER + 0x1c8, r.next_u32() % 2);
        // The audio system: no song playing, its channel vtable empty (fakes).
        c.bus.write32(GAME_GP + 0x410, RS_AUDIO);
        c.bus.write32(RS_AUDIO + 0x5558, RS_AUDIO_VT);
        // World type, sub-type, mode (0x535c10..12).
        c.bus.write8(0x0053_5c10, [0, 0, 1, 5][(r.next_u32() % 4) as usize]);
        c.bus.write8(0x0053_5c11, [0, 0, 1][(r.next_u32() % 3) as usize]);
        c.bus.write8(0x0053_5c12, (r.next_u32() % 14) as u8);
        match step {
            1 => {
                for i in 0..2 {
                    c.bus.write32(RS_LISTS + 0xcc * i + 0xa4, r.next_u32() % 3);
                }
            }
            2 => {
                let k = v % 4;
                rs_ai(c, k == 0, k == 1, false);
                c.bus.write32(RS_AI + 0x7c, r.next_u32() % 3);
                c.bus.write32(GAME_GP - 0x848, RS_WORLD);
                c.bus.write32(RS_WORLD + 0xc0, RS_MODE);
                c.bus.write32(RS_MODE + 0x4, r.next_u32() % 13);
                let ai_riders = if k == 3 { 1 + r.next_u32() % 2 } else { 0 };
                let humans = r.next_u32() % 3;
                c.bus.write32(RS_MODE + 0x10, humans + ai_riders);
                c.bus.write32(RS_MODE + 0x14, humans);
                for i in 0..16 {
                    c.bus.write32(RS_MODE + 0x18 + 4 * i, r.next_u32() % 64);
                }
                if ai_riders > 0 {
                    // Creating AI riders is a whole subsystem: unwind at the
                    // second interface commit (after the character loop).
                    let c12 = c.bus.read8(0x0053_5c12);
                    let restart = !(6..=11).contains(&c12);
                    c.bus.write32(LOG_UNWIND_AT, if restart { 6 } else { 3 });
                }
            }
            3 => {
                rs_ai(c, false, false, v % 3 == 0);
                c.bus.write32(RS_AI + 0x80, r.next_u32() % 2);
                c.bus.write32(RS_AI + 0x84, r.next_u32() % 2);
                c.bus.write32(RS_AI + 0x88, 0);
            }
            _ => {}
        }
    }

    #[test]
    fn game_state_reset_overlay_update_matches() {
        check(
            ee::s06::f_00235080__generated,
            game_state_reset_overlay_update,
            7 * 40,
            &|c, seed| reset_overlay_setup(c, seed),
        );
    }

    /// Run `f` (entry `pc`) on the state at `RS` until its step reaches
    /// `want`: the number of updates it took.
    fn rs_ticks(c: &mut Cpu, f: GuestFn, pc: u32, want: u32) -> u32 {
        for n in 1..10_000 {
            c.g[29][0] = sx(SP);
            c.g[RA][0] = sx(TEST_RA);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = sx(RS);
            c.pc = pc;
            f(c);
            assert_eq!(c.pc, TEST_RA);
            if c.bus.read32(RS + 0x10) == want {
                return n;
            }
        }
        panic!("step {want} never reached");
    }

    /// The peak transition's 180-frame hold (step 4) lasts 3 s at any rate.
    #[test]
    fn game_state_reset_overlay_update_hold_follows_rate() {
        for (hz, ticks) in [(60, 180), (120, 360), (240, 720)] {
            let mut c = crate::difftest::fresh();
            crate::tick::set_for_test(&mut c, hz);
            c.bus.write32(RS + 0x10, 4);
            c.bus.write32(RS + 0x14, 1);
            c.bus.write32(RS + 0x18, 0);
            assert_eq!(rs_ticks(&mut c, game_state_reset_overlay_update, 0x0023_5080, 5), ticks, "{hz} Hz");
        }
    }

    fn restart_race_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x235c_c8, 6);
        let step = seed % 5;
        let v = seed / 5;
        c.g[A0][0] = sx(RS);
        c.bus.write32(RS + 0x10, step);
        let counter = match v % 5 {
            0 => 118,
            1 => 119,
            2 => 120,
            3 => r.next_u32() % 300,
            _ => r.next_u32(),
        };
        c.bus.write32(RS + 0x14, counter);
        c.bus.write32(RS + 0x18, if v % 3 == 0 { 0 } else { 1 });
        rs_engine(c, false);
        c.bus.write32(RS_LISTS + 0xa4, r.next_u32() % 3);
        if v % 2 == 1 {
            // An active list: its `+0xb0`.
            c.bus.write32(RS_ENGINE + 0x550, 0);
            c.bus.write32(RS_LISTS + 0xb0, r.next_u32() % 2);
        }
        // world -> +0x84 game: +0x84 -> +4 -> +0xb0 (the view index), +0x10
        // the streamer, +0xc the AI, +0x5c the transition manager.
        let game = RS + 0x7000;
        let x = RS + 0x7100;
        let y = RS + 0x7200;
        let streamer_ref = RS + 0x7300;
        let streamer = RS + 0x8000;
        let trans = RS + 0x7400;
        let trans_vt = RS + 0x7500;
        c.bus.write32(GAME_GP - 0x848, RS_WORLD);
        c.bus.write32(RS_WORLD + 0x84, game);
        c.bus.write32(game + 0x84, x);
        c.bus.write32(x + 4, y);
        c.bus.write32(y + 0xb0, r.next_u32() % 2);
        c.bus.write32(game + 0x10, streamer_ref);
        c.bus.write32(streamer_ref, streamer);
        c.bus.write32(game + 0xc, RS_AI);
        c.bus.write32(game + 0x5c, trans);
        c.bus.write32(trans + 0x1c, trans_vt);
        // The transition manager's "active" query: a leaf
        // (`GameStateReset_v08`: its argument's `+0x10 == 5`).
        let off = (r.next_u32() % 8) * 4;
        c.bus.write16(trans_vt + 0x40, off as u16);
        c.bus.write32(trans_vt + 0x44, 0x0023_55b0);
        c.bus.write32(trans + 0x10 + off + 0x10, if r.next_u32() % 3 == 0 { 5 } else { 0 });
        c.bus.write32(trans + 0x44, r.next_u32() % 2);
        let k = v % 4;
        rs_ai(c, k == 0, false, step == 1 && k == 1);
        // The start marker's rider (AI +0x40 -> +0x18).
        c.bus.write32(RS_AI + 0x40, RS_OBJ);
        // The AI rider count. Creating computer riders is a whole subsystem:
        // with riders to create, unwind at `AI_CreateComputerRiders`' first
        // interface call (fake 1).
        let riders = r.next_u32() % 4;
        c.bus.write32(0x0053_5c04, riders);
        if step == 0 && riders > 0 {
            c.bus.write32(LOG_UNWIND_AT, 1);
        }
        // The view clearance the streamer reports (its view is idle, so the
        // stack word stays as written): below or above the threshold 0.
        let clear: f32 = if r.next_u32() % 3 == 0 { -1.0 } else { 1.0 };
        c.bus.write32(SP - 0x70 + 0x10, clear.to_bits());
        c.bus.write32(GAME_GP - 0x516c, 0);
    }

    #[test]
    fn game_state_restart_race_update_matches() {
        check(
            ee::s06::f_00235cc8__generated,
            game_state_restart_race_update,
            5 * 48,
            &|c, seed| restart_race_setup(c, seed),
        );
    }

    /// The restart's 120-frame minimum (step 2) lasts 2 s at any rate.
    #[test]
    fn game_state_restart_race_update_minimum_follows_rate() {
        for (hz, ticks) in [(60, 120), (120, 240), (240, 480)] {
            let mut c = crate::difftest::fresh();
            c.hooks.miss = fake_call;
            c.bus.write32(LOG_UNWIND_AT, u32::MAX);
            crate::tick::set_for_test(&mut c, hz);
            restart_race_rate_fixture(&mut c);
            assert_eq!(rs_ticks(&mut c, game_state_restart_race_update, 0x0023_5cc8, 3), ticks, "{hz} Hz");
        }
    }

    /// Step 2 with the skip armed, a clear view, list 0 idle, no active
    /// list or transition: only the update count holds it.
    fn restart_race_rate_fixture(c: &mut Cpu) {
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(RS + 0x10, 2);
        c.bus.write32(RS + 0x14, 0);
        c.bus.write32(RS + 0x18, 1);
        rs_engine(c, false);
        c.bus.write32(RS_LISTS + 0xa4, 0);
        let game = RS + 0x7000;
        let x = RS + 0x7100;
        let streamer_ref = RS + 0x7300;
        let streamer = RS + 0x8000;
        let trans = RS + 0x7400;
        let trans_vt = RS + 0x7500;
        c.bus.write32(GAME_GP - 0x848, RS_WORLD);
        c.bus.write32(RS_WORLD + 0x84, game);
        c.bus.write32(game + 0x84, x);
        c.bus.write32(x + 4, RS + 0x7200);
        c.bus.write32(game + 0x10, streamer_ref);
        c.bus.write32(streamer_ref, streamer);
        c.bus.write32(game + 0xc, RS_AI);
        c.bus.write32(game + 0x5c, trans);
        c.bus.write32(trans + 0x1c, trans_vt);
        // The transition manager's "active" query: a leaf returning 0
        // (`GameStateReset_v08` on a state whose `+0x10` is not 5).
        c.bus.write32(trans_vt + 0x44, 0x0023_55b0);
        c.bus.write32(trans + 0x44, 0);
        // The streamer view is idle: the clearance word on the stack stays
        // as written (1.0 above the 0 threshold).
        c.bus.write32(SP - 0x70 + 0x10, 1.0f32.to_bits());
    }
}
