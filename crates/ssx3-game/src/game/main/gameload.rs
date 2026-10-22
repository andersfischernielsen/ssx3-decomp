//! `src/main/gameload.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::abi::r32;
use ssx3_ee as ee;
use crate::call;
use crate::events::back_edge_unwinds;
use crate::ps2f::FpuF32;
pub(crate) use crate::game::render::lightman::gp_2 as gp;
pub(crate) use crate::game::render::lightman::at;
pub(crate) use crate::game::dirtysock::tags2::arg;
pub(crate) use crate::game::fe::festaterewards2::r;
pub(crate) use crate::game::dirtysock::tags2::gp_3 as gp_2;
pub(crate) use crate::game::fe::festaterewards2::Ops;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::scripter::ssxscriptengine::p as p_2;
pub(crate) use crate::game::fe::feasyncfile::at as at_2;
pub(crate) use crate::game::fe::feasyncfile::gp_12 as gp_3;
pub(crate) use crate::game::fe::feasyncfile::at as at_3;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw;
pub(crate) use crate::game::fe::feasyncfile::sw_4 as sw;
pub(crate) use crate::game::fe::feasyncfile::lw_6 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::lh;
pub(crate) use crate::game::fe::feasyncfile::gp_16 as gp_4;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_3;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0023_00f0, game_module_reset_scripts_and_world, "GameModule_ResetScriptsAndWorld"),
    (0x0023_02a8, game_module_quick_restart, "GameModule_QuickRestart"),
    (0x0023_0338, game_module_invalidate_focus, "GameModule_InvalidateFocus"),
    (0x0023_0360, game_module_on_section_unload, "GameModule_OnSectionUnload"),
    (0x0023_05c8, game_module_end_replay_playback, "GameModule_EndReplayPlayback"),
    (0x0023_0698, game_module_get_transition_man, "GameModule_GetTransitionMan"),
    (0x0023_06a8, game_module_get_splash_manager, "GameModule_GetSplashManager"),
    (0x0023_0e98, game_module_try_exit, "GameModule_TryExit"),
    (0x0023_1320, game_module_advance_state, "GameModule_AdvanceState"),
    (0x0023_1840, game_module_check_pause_request, "GameModule_CheckPauseRequest"),
    (0x0023_1c70, app_module_try_exit_when_loaded, "AppModule_TryExitWhenLoaded"),
    (0x0023_1cd0, app_module_ctor, "AppModule_ctor"),
    (0x0023_2328, pre_start_screen_ctor, "PreStartScreen_ctor"),
    (0x0023_82d8, game_mode_man_reset_mode, "GameModeMan_ResetMode"),
];

// ---------------------------------------------------------------------------
// GameModule_CheckPauseRequest (0x231840).

/// `gp` globals of the pause check.
pub(crate) mod pause {

    /// `D_004A2EEC`: the active "loading" screen (its `+8` freezes input).
    pub const LOADING: i32 = -0x204;

    /// `D_004A2E9C`: disables the select-confirm input evaluation.
    pub const NO_CONFIRM: i32 = -0x254;

    /// `D_00534B30`: a pending restart flag.
    pub const RESTART: u32 = 0x0053_4b30;
}

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
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
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_SET_STATE_2 as REPLAY_SET_STATE;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL_2 as UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_4 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_2 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_3 as SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_10 as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_2 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_HIDE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SHOW;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP_2 as NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_GET_CHARACTER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_FILL_PLAYER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_SET_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_PDA_TEMPLATE_CLOSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_MESSAGE_CENTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_COUNT_2 as BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_ENTRY_2 as BUDDY_MAN_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_DATA_GET_BANK_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_UNREGISTER_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ANIM_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_SOUND_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_INSTANCE_3 as SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_OBJ_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ON_ACTIVATED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_3 as SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CUI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_2 as UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_3 as UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR_2 as UI_OBJECT_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_STREAMER_UPDATE_VIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_SET_IN_GAME_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SET_INPUT_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_SET_OWNER;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_3 as WORLD_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_READ_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_FLAG_MANAGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_MANAGER_ADD_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_APPLY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_ADVANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_PROCESS_DTOR_2 as GAME_PROCESS_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_RESET_RACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODE_MAN_RESET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_REQUEST_STATE_5 as GAME_MODULE_REQUEST_STATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_START_NEXT_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_APP;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_INTERFACE_GET_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_BE_2 as BE_GET_BE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_5 as SPRINTF_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::VSPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::EDITOR_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_SELECT_ALGORITHM;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_FREE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_LOAD_BANK_FROM_MEMORY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_MUSIC_TIER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_TRACK_TYPE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS_2 as SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_KILL_ALL_PROCESSES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS_2 as SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE_2 as SCRIPT_ENGINE_RESET_PROCESS_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COMPUTE_OFFSET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_AABB_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_PAD_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_NODE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME_2 as UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBE_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_2 as NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_ADD_CHILD;
    pub(crate) use crate::game::fe::feasyncfile::callee::STR_HASH_5 as STR_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_CREATE_SCREEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::UISCREEN_START_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UISCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_CHAR_DISPLAY_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_7 as SPRINTF_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UITEXTSCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::feasyncfile::callee::UITEXTSCROLL_SET_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_GET_ACTIVE_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_SET_OPTION_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_OPEN_KEYBOARD_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_GOTO_NEXT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFE_STATE_LOGIN_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFE_STATE_NET_CONFIG_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBXSTRING_ASSIGN;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_SET_ACTIVE_2 as MENU_SYS_SET_ACTIVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_DRAW_ROW_2 as MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_MEASURE_ROW;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_CHOOSE_SCRIPT as SCRIPT_ENGINE_CHOOSE_SCRIPT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_POP_FREE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_START_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_PUSH_FREE_NODE_2 as SCRIPT_ENGINE_PUSH_FREE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_ADD_SCRIPT_OBJ;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ADD_CAM;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_SET_TRANSITION;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_INSTANCE_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_EVENT_TRACK_FIRE_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_EVENT_TRACK_NEXT_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_VSPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_LEN_2 as USTR_LEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_CPY_2 as USTR_CPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_ALLOC_NAMED_5 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_FREE_5 as EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::EAFILE_GET_PATH_SIZE_CHECKED;
    pub(crate) use crate::game::fe::feasyncfile::callee::EAFILE_LOAD_FILE_INTO;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_FILE_ACQUIRE;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARDS_FILE_PARSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_BACKUP_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_METRIX_ON_RIDER_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_ADD_WIPEOUT_RECOVERY_BOOST;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_CHECK_RESET_PATH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ANIM_CONTROLLER_IS_LAYER_ANIM_FINISHED;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_IS_FRAME_DUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_DECODE_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_IS_AUDIO_DONE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_3 as UI_LIST_DELETE_ALL_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_4 as UI_LIST_NODE_DTOR_BODY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ENGINE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_FIND_STATE_2 as UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_SET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_KILL_ALL_TIMERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_STOP_SONG_2 as MUSIC_PLAYER_STOP_SONG;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_FADE_OUT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_PLAY_SONG;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_TRY_EXIT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_CLEAR_SKIP_DELAY_ON_EXIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_INDICATOR_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GRAPHICS_MAN_FREE_ALL_TEXTURES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_4 as SSX_AUDIO_SYSTEM_GET_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_GET_RECORD;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_GAME_UTIL_CREATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_GAME_UTIL_ADVERT;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_POPUP_CONFIRM_EMPTY_INIT_HOOK;
    pub(crate) use crate::game::fe::feasyncfile::callee::DEBUG_MENU_DISPATCH_ITEM_OP;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_LIST_GET_AT_2 as LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATOI;
    pub(crate) use crate::game::fe::feasyncfile::callee::TAG_FIELD_GET_BINARY;
}


// ---------------------------------------------------------------------------
// Game module.

/// `void *GameModule_GetTransitionMan(GameModule *self, int index)` (0x230698):
/// `self->transitions[index]` (word at `self + index*4 + 0x5c`).
pub fn game_module_get_transition_man(c: &mut Cpu) {
    const TRANSITIONS: u32 = 0x5c;
    c.cycles += 4;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    let slot = (c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32);
    c.g[A0][0] = sx(slot);
    c.g[V0][0] = Addr(slot).off(TRANSITIONS).lw(c);
    abi::ret(c);
}


/// `void *GameModule_GetSplashManager(GameModule *self, int index)` (0x2306a8):
/// `self->splash[index]` (word at `self + index*4 + 0x68`).
pub fn game_module_get_splash_manager(c: &mut Cpu) {
    const SPLASH: u32 = 0x68;
    c.cycles += 4;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
    let slot = (c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32);
    c.g[A0][0] = sx(slot);
    c.g[V0][0] = Addr(slot).off(SPLASH).lw(c);
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// App module.

/// `AppModule *AppModule_ctor(AppModule *self)` (0x231cd0): install vtable
/// 0x47d980, zero the flag at `+8`, copy the float at `gp-0x51b8` into `+4`,
/// and return `self`.
pub fn app_module_ctor(c: &mut Cpu) {
    const VTABLE: u32 = 0x48_0000 - 0x2680;
    const FADE: u32 = 0x4;
    const FLAG: u32 = 0x8;
    let this = Addr(abi::arg(c, 0));
    c.cycles += 8;
    let fade = Addr((c.g[GP][0] as u32).wrapping_sub(0x51b8)).read_fpu(c);
    c.f[0] = fade.0;
    let vtable = sx(VTABLE);
    c.g[V0][0] = vtable;
    this.off(FLAG).sw(c, 0);
    this.sw(c, vtable);
    c.g[V0][0] = c.g[A0][0];
    this.off(FADE).write_fpu(c, fade);
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Simple screen/state constructors: install a vtable and zero some fields.

/// `PreStartScreen *PreStartScreen_ctor(PreStartScreen *self)` (0x232328):
/// vtable 0x47d938, zero `+4` and `+8`, return `self`.
pub fn pre_start_screen_ctor(c: &mut Cpu) {
    const VTABLE: u32 = 0x48_0000 - 0x26c8;
    let this = Addr(abi::arg(c, 0));
    c.cycles += 7;
    let vtable = sx(VTABLE);
    c.g[V1][0] = vtable;
    c.g[V0][0] = c.g[A0][0];
    this.off(0x4).sw(c, 0);
    this.sw(c, vtable);
    this.off(0x8).sw(c, 0);
    abi::ret(c);
}


/// The body after the early out: pick the player the pause menu should
/// select and store the ready mask (+0x224) and selection (+0x228).
pub(crate) fn pause_request_body(c: &mut Cpu) -> Result<(), Unwound> {
    // `cBE_getBE()` / `cBE_getInterface(be, 1)`.
    c.cycles += 10;
    c.g[S4][0] = 0;
    call_ptr(c, 0x0014_dc80, 0x0023_1884)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 1;
    call_ptr(c, 0x0014_dd58, 0x0023_1890)?;
    c.g[V1][0] = c.lw(at(c.g[S2][0], 0x28));
    c.g[S6][0] = c.g[V0][0];
    c.g[V0][0] = c.lw(at(c.g[V1][0], 0x608));
    let single = c.g[V0][0] != 0;
    c.g[S3][0] = 0xffff_ffff_ffff_ffff;
    if single {
        // One active player (`game+0x28`): mask him if not ready, else let
        // the confirm input select him.
        c.cycles += 8;
        c.g[S0][0] = c.lw(at(c.g[V1][0], 0x28));
        c.g[V0][0] = c.lw(gp(c, -0x848));
        c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[A0][0] = c.lw(at(c.g[V0][0], 0xb0));
        c.g[V0][0] = c.lw(at(c.g[A0][0], 0));
        c.g[A1][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = c.lw(at(c.g[V0][0], 0));
            c.g[A1][0] = (0 < c.g[V0][0]) as u64;
        }
        c.cycles += 2;
        c.g[V0][0] = c.lw(gp(c, pause::NO_CONFIRM));
        if c.g[A1][0] != 0 {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.cycles += 4;
                c.g[A1][0] = 0x3c;
                call_ptr(c, 0x0032_0c48, 0x0023_18f0)?;
                if c.g[V0][0] != 0 {
                    c.g[S3][0] = c.g[S0][0];
                }
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = 1;
            c.g[S4][0] = sx((c.g[V0][0] as u32) << ((c.g[S0][0] as u32) & 31));
        }
    } else {
        // Several riders: mask every active one that is not ready.
        c.cycles += 4;
        c.g[V1][0] = c.lw(at(c.g[S2][0], 0xc));
        c.g[V0][0] = c.lw(at(c.g[V1][0], 0x7c));
        let no_riders = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !no_riders {
            c.cycles += 2;
            c.g[S5][0] = 1;
            c.g[V0][0] = c.lw(at(c.g[S2][0], 0xc));
            loop {
                c.cycles += 7;
                c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                c.g[A0][0] = c.lw(at(c.g[V0][0], 0x40));
                c.g[V1][0] = c.lw(at(c.g[A0][0], 0x18));
                c.g[V0][0] = c.lw(at(c.g[V1][0], 0x87c));
                let inactive = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S6][0];
                if !inactive {
                    c.cycles += 10;
                    c.g[A1][0] = c.g[S1][0];
                    call_ptr(c, 0x0014_7290, 0x0023_1934)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V0][0] = c.lw(gp(c, -0x848));
                    c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[A0][0] = c.lw(at(c.g[V0][0], 0xb0));
                    c.g[V0][0] = c.lw(at(c.g[A0][0], 0));
                    let not_ready = c.g[V0][0] == 0;
                    c.g[A1][0] = 0;
                    if !not_ready {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw(at(c.g[V0][0], 0));
                        c.g[A1][0] = c.g[S5][0];
                        if c.g[V0][0] == 0 {
                            c.g[A1][0] = 0;
                        }
                    }
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[S5][0] as u32) << ((c.g[S0][0] as u32) & 31));
                    if c.g[A1][0] != 0 {
                        // Ready: select it unless one is selected already.
                        c.cycles += 2;
                        if (c.g[S3][0] as i64) >= 0 {
                            c.g[V1][0] = c.lw(at(c.g[S2][0], 0xc));
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(gp(c, pause::NO_CONFIRM));
                            if c.g[V0][0] != 0 {
                                c.g[V1][0] = c.lw(at(c.g[S2][0], 0xc));
                            } else {
                                c.cycles += 3;
                                c.g[A1][0] = 0x3c;
                                call_ptr(c, 0x0032_0c48, 0x0023_198c)?;
                                if c.g[V0][0] != 0 {
                                    c.g[S3][0] = c.g[S0][0];
                                }
                                c.cycles += 1;
                                c.g[V1][0] = c.lw(at(c.g[S2][0], 0xc));
                            }
                        }
                    } else {
                        c.cycles += 2;
                        let bit = c.g[V0][0];
                        c.g[S4][0] |= bit;
                        c.cycles += 1;
                        c.g[V1][0] = c.lw(at(c.g[S2][0], 0xc));
                    }
                } else {
                    c.cycles += 1;
                    c.g[V1][0] = c.lw(at(c.g[S2][0], 0xc));
                }
                c.cycles += 5;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V0][0] = c.lw(at(c.g[V1][0], 0x7c));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = c.lw(at(c.g[S2][0], 0xc));
                    if event_due(c) {
                        deliver_event_here(c, 0x0023_1910);
                    }
                    continue;
                }
                break;
            }
        }
    }

    // The ready mask and the selection.
    c.cycles += 2;
    let no_mask = c.g[S4][0] == 0;
    let s2 = c.g[S2][0];
    let mask = c.g[S4][0];
    c.sw(at(s2, 0x224), mask);
    if !no_mask {
        c.cycles += 2;
        if (c.g[S3][0] as i64) >= 0 {
            c.sw(at(s2, 0x228), c.g[S3][0]);
        } else {
            c.cycles += 5;
            c.g[V1][0] = c.lw(at(s2, 0xc));
            c.g[S0][0] = 0;
            c.g[V0][0] = c.lw(at(c.g[V1][0], 0x7c));
            let no_riders = (c.g[V0][0] as i64) <= 0;
            c.g[S3][0] = 0;
            if no_riders {
                c.cycles += 1;
                c.sw(at(s2, 0x228), c.g[S3][0]);
            } else {
                // Search for the first ready rider to select.
                c.cycles += 3;
                c.g[S1][0] = 1;
                c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                loop {
                    c.cycles += 6;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                    c.g[V1][0] = c.lw(at(c.g[V0][0], 0x40));
                    c.g[A0][0] = c.lw(at(c.g[V1][0], 0x18));
                    c.g[V0][0] = c.lw(at(c.g[A0][0], 0x87c));
                    let inactive = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[S6][0];
                    if !inactive {
                        c.cycles += 10;
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, 0x0014_7290, 0x0023_19f8)?;
                        c.g[A2][0] = c.g[V0][0];
                        c.g[V0][0] = c.lw(gp(c, -0x848));
                        c.g[A0][0] = sx((c.g[A2][0] as u32) << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        c.g[V1][0] = c.lw(at(c.g[V0][0], 0xb0));
                        c.g[V0][0] = c.lw(at(c.g[V1][0], 0));
                        let not_ready = c.g[V0][0] == 0;
                        c.g[A1][0] = 0;
                        if !not_ready {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw(at(c.g[V0][0], 0));
                            c.g[A1][0] = c.g[S1][0];
                            if c.g[V0][0] == 0 {
                                c.g[A1][0] = 0;
                            }
                        }
                        c.cycles += 2;
                        if c.g[A1][0] != 0 {
                            c.g[S3][0] = c.g[A2][0];
                            c.cycles += 1;
                            c.sw(at(s2, 0x228), c.g[S3][0]);
                            break;
                        }
                    }
                    c.cycles += 6;
                    c.g[V1][0] = c.lw(at(s2, 0xc));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    c.g[V0][0] = c.lw(at(c.g[V1][0], 0x7c));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0023_19d8);
                        }
                        continue;
                    }
                    c.cycles += 2;
                    c.sw(at(s2, 0x228), c.g[S3][0]);
                    break;
                }
            }
        }
        c.cycles += 1;
        c.g[V0][0] = 1;
        return Ok(());
    }

    // No one is ready: only a pending restart lets the pause be left.
    c.cycles += 2;
    if (c.g[S3][0] as i64) < 0 {
        c.g[V0][0] = 0;
        return Ok(());
    }
    c.cycles += 9;
    call_ptr(c, 0x0014_dc80, 0x0023_1a5c)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 7;
    call_ptr(c, 0x0014_dd58, 0x0023_1a68)?;
    c.g[V1][0] = 0x0053_0000;
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4b30));
    if c.g[V0][0] != 0 {
        c.sw(at(s2, 0x228), c.g[S3][0]);
        c.cycles += 1;
        c.g[V0][0] = 1;
        return Ok(());
    }
    c.cycles += 4;
    c.g[A0][0] = c.g[S2][0];
    Callee { addr: 0x0023_1ab8, f: ssx3_ee::s06::GameModule_CanRestartFromPause }
        .call(c, 0x0023_1a80)?;
    let restart = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    if !restart {
        return Ok(());
    }
    c.sw(at(s2, 0x228), c.g[S3][0]);
    c.cycles += 1;
    c.g[V0][0] = 1;
    Ok(())
}


/// `int GameModule_CheckPauseRequest(GameModule *g)` (0x231840): decides
/// which player the pause menu selects. Returns 1 when the pause can be
/// confirmed (a ready player is selected, or a restart is permitted), 0
/// otherwise; the ready mask goes to `g+0x224` and the selection to `g+0x228`.
///
/// Frame 0x80: s2 at +0x50, s0 at +0x70, s1 at +0x60, s3 at +0x40, s4 at
/// +0x30, s5 at +0x20, s6 at +0x10, ra at 0.
pub fn game_module_check_pause_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = c.lw(gp(c, pause::LOADING));
        frame.save(c, S2, 0x50);
        frame.save(c, S0, 0x70);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x60);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save(c, S5, 0x20);
        frame.save(c, S6, 0x10);
        let loading = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        let mut stopped = false;
        if loading {
            c.cycles += 3;
            c.g[V0][0] = c.lw(at(c.g[V0][0], 8));
            stopped = (c.g[V0][0] as i64) > 0;
            c.g[V0][0] = 0;
        }
        if !stopped {
            pause_request_body(c)?;
        }
        c.cycles += 10;
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
// GameModule_InvalidateFocus / GameStateResults_Exit / GameRand_Seed.

/// `void GameModule_InvalidateFocus(GameModule* m)` (0x230338): drop the
/// focused widget by setting its focus timer (`+0xd0`) to -1.
pub fn game_module_invalidate_focus(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0xc));
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0xa4));
        let none = c.g[V1][0] == 0;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        if !none {
            c.cycles += 1;
            Addr(r32(c, V1).wrapping_add(0xd0)).sw(c, c.g[V0][0]);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Game states.

/// `void GameModule_AdvanceState(GameModule* self)` (0x231320): if the module
/// has a pending state at `+0x200`, ask it for the next state.
pub fn game_module_advance_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(arg(c, 0).wrapping_add(0x200)).lw(c);
        let none = c.g[A0][0] == 0;
        frame.restore_ra(c, 0);
        if !none {
            c.cycles += 3;
            callee::GAME_STATE_GOTO_NEXT.call(c, 0x0023_133c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Game loading and results.

/// `int AppModule_TryExitWhenLoaded(AppModule* self)` (0x231C70): once the
/// loading progress global reaches 100%, ask the module to exit.
pub fn app_module_try_exit_when_loaded(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        let loader = c.lw_r_2(V0, gp_2(c, 0x2a74));
        frame.save_ra(c, 0);
        c.g[AT][0] = 0x42c8_0000;
        c.mtc1(1, 1);
        c.ld_f(0, (loader as u32).wrapping_add(0x38));
        c.c_eq_s(0, 1);
        let loaded = c.fcc();
        c.g[V0][0] = 0;
        if loaded {
            c.cycles += 2;
            callee::APP_MODULE_TRY_EXIT.call(c, 0x0023_1ca0)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cGameModeMan_restartHeat(GameModeMan* gm)` (0x2382d8): restart the
/// current mode object (virtual slot 2) and clear the six mode-history words.
pub fn game_mode_man_reset_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x0053_6668;
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(p(r32(c, A0), 4)).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[V1][0] = abi::addu(c.g[V1][0], c.g[V0][0]);
        c.g[A1][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x10)).lh(c);
        c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x14)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = abi::addu(c.g[A1][0], c.g[A0][0]);
        call::call_ptr(c, target, 0x0023_830c)?;
        c.g[V0][0] = 0x0053_66d0;
        c.g[V1][0] = 5;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
        loop {
            c.cycles += 7;
            Addr(c.g[V0][0] as u32).sw(c, 0);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0023_8320);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    });
}


/// `func_002305C8` (0x2305c8): `GameModule_EndReplayPlayback`. Stops the
/// replay (`0x26f980`), hides the replay UI (`0x2cc0b8(ui, 1)`), closes it
/// (`0x2cc460(ui)`), then pops one entry off the two replay stacks at
/// `D_005367`/`D_005368` (a counter at `gp-0x69c` and the saved slot at
/// `gp-0x6a0`). Signature: `void end_replay(GameModule* this)`.
pub fn game_module_end_replay_playback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[RA][0] = sx(0x0023_05e0);
        c.g[A0][0] = lw_r(c, S0, 0x28);
        call_ptr(c, 0x0026_f980, 0x0023_05e0)?;
        c.g[A0][0] = lw_r(c, S0, 0xa4);
        c.g[RA][0] = sx(0x0023_05ec);
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x002c_c0b8, 0x0023_05ec)?;
        c.g[RA][0] = sx(0x0023_05f4);
        c.g[A0][0] = lw_r(c, S0, 0xa4);
        call_ptr(c, 0x002c_c460, 0x0023_05f4)?;
        c.g[V0][0] = lw_r(c, GP, 0u32.wrapping_sub(0x69c));
        c.g[A0][0] = 0x53_0000;
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x66e8));
        c.g[A1][0] = 0x44_0000;
        c.g[V0][0] = sx(low(c, V0).wrapping_sub(0x1));
        c.g[A1][0] = sx(low(c, A1).wrapping_add(0x28f0));
        c.g[V1][0] = sx(low(c, V0) << 2);
        sw_r(c, GP, 0u32.wrapping_sub(0x69c), c.g[V0][0]);
        c.g[V1][0] = sx(low(c, V1).wrapping_add(low(c, A0)));
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = Addr(low(c, V1)).lw(c);
        c.g[RA][0] = c.ld(low(c, SP));
        c.g[V0][0] = sx(low(c, V0) << 2);
        c.g[V0][0] = sx(low(c, V0).wrapping_add(low(c, A1)));
        c.g[V1][0] = Addr(low(c, V0)).lw(c);
        sw_r(c, GP, 0u32.wrapping_sub(0x6a0), c.g[V1][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Game module reset.

/// `void GameModule_ResetScriptsAndWorld(GameModule* self)` (0x2300f0): tear
/// down the script/animation banks and reset the world map.
pub fn game_module_reset_scripts_and_world(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(gp(c, 0x2898));
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[A0][0] = Addr(p_2(r32(c, S1), 0x34)).lw(c);
        call_ptr(c, 0x0026_ca90, 0x0023_0110)?;
        c.g[A0][0] = Addr(p_2(r32(c, S1), 0x30)).lw(c);
        call_ptr(c, 0x0030_9030, 0x0023_0118)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0035_4c98, 0x0023_0124)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0035_5118, 0x0023_0130)?;
        c.g[A1][0] = 8;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0035_5118, 0x0023_013c)?;
        c.g[V0][0] = Addr(p_2(r32(c, S1), 0xc)).lw(c);
        c.g[A0][0] = Addr(p_2(r32(c, V0), 0xa4)).lw(c);
        call_ptr(c, 0x0010_3358, 0x0023_0148)?;
        call_ptr(c, 0x002d_9b40, 0x0023_0150)?;
        c.g[A0][0] = Addr(p_2(r32(c, S1), 0x10)).lw(c);
        call_ptr(c, 0x003a_6800, 0x0023_0158)?;
        c.g[A0][0] = Addr(gp(c, 0xce8)).lw(c);
        call_ptr(c, 0x0030_9f18, 0x0023_0160)?;
        c.g[A0][0] = Addr(p_2(r32(c, S1), 0x30)).lw(c);
        call_ptr(c, 0x0030_8c60, 0x0023_0168)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Game module and FE state entry points.

/// `void cGame_restart(GameModule* self)` (0x2302a8): reset the race and the
/// game mode, clear every rider's state word, then request the restart state
/// and re-enter the menu flow.
pub fn game_module_quick_restart(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        let self_ = abi::arg(c, 0);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(self_);
        call_ptr(c, callee::GAME_MODULE_RESET_RACE, 0x0023_02bc)?;
        c.g[V0][0] = c.lw(gp_3(c, -0x848));
        c.g[A0][0] = c.lw(at_2(c, V0, 0xc0));
        call_ptr(c, callee::GAME_MODE_MAN_RESET_MODE, 0x0023_02c8)?;
        c.g[V1][0] = c.lw(at_2(c, S0, 0xc));
        c.g[V0][0] = c.lw(at_2(c, V1, 0x78));
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 2;
            c.g[V1][0] = c.lw(at_2(c, S0, 0xc));
            c.g[A0][0] = sx(r32(c, A1) << 2);
            loop {
                c.cycles += 9;
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(1));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
                c.g[V0][0] = c.lw(at_2(c, V1, 0x28));
                c.sw(at_2(c, V0, 0x2f8), 0);
                c.g[V1][0] = c.lw(at_2(c, S0, 0xc));
                c.g[V0][0] = c.lw(at_2(c, V1, 0x78));
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[A0][0] = sx(r32(c, A1) << 2);
                if event_due(c) {
                    deliver_event_here(c, 0x0023_02e0);
                }
            }
        }
        c.cycles += 13;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        c.g[A2][0] = 2;
        c.g[A3][0] = 1;
        call_ptr(c, callee::GAME_MODULE_REQUEST_STATE_2, 0x0023_0318)?;
        call_ptr(c, callee::SSX_AUDIO_SYSTEM_GET_INSTANCE, 0x0023_0320)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, callee::GAME_MODULE_START_NEXT_MODE, 0x0023_0328)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void GameModule_Enter(self)` (0x230050): tear down the front-end, reset
/// the managers and set the game-entered flags.
pub fn game_module_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        c.g[A0][0] = lw(c, V0, 0x8c);
        call_ptr(c, 0x0019_5d50, 0x0023_006c)?;
        c.g[A0][0] = lw(c, S0, 0x94);
        Addr(gp(c, -0x9e4)).sw(c, 0);
        call_ptr(c, 0x001f_30d8, 0x0023_0078)?;
        c.g[A0][0] = Addr(gp(c, -0x84c)).lw(c);
        call_ptr(c, 0x0027_8710, 0x0023_0080)?;
        c.g[A1][0] = Addr(gp(c, 0x1158)).lw(c);
        c.g[V0][0] = Addr(c.g[A1][0] as u32).off(4).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x88).lh(c);
        c.g[V1][0] = lw(c, V0, 0x8c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_0098)?;
        c.g[A1][0] = Addr(gp(c, 0x1158)).lw(c);
        c.g[V0][0] = Addr(c.g[A1][0] as u32).off(4).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x20).lh(c);
        c.g[V1][0] = lw(c, V0, 0x24);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_00b0)?;
        c.g[A0][0] = Addr(gp(c, -0x854)).lw(c);
        call_ptr(c, 0x0036_9ff8, 0x0023_00b8)?;
        c.g[A0][0] = lw(c, S0, 0x10);
        call_ptr(c, 0x003a_67e0, 0x0023_00c0)?;
        c.g[A0][0] = Addr(gp(c, 0x21e4)).lw(c);
        call_ptr(c, 0x002b_68d0, 0x0023_00c8)?;
        c.g[A0][0] = Addr(gp(c, 0x410)).lw(c);
        call_ptr(c, 0x0028_67e8, 0x0023_00d0)?;
        c.g[V0][0] = 1;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        Addr(gp(c, 0x12d8)).sw(c, c.g[V0][0]);
        Addr(gp(c, 0x1234)).sw(c, c.g[V0][0]);
        Addr(gp(c, 0x14e8)).sw(c, c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}


/// `int GameModule_TryExit(self)` (0x230e98): if the module's virtual "can
/// exit" lets it, shut the app module down and report success.
pub fn game_module_try_exit(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x30);
        'end: {
            c.cycles += 12;
            frame.save(c, S0, 0x20);
            frame.save_ra(c, 0x10);
            c.g[S0][0] = c.g[A0][0];
            c.g[A1][0] = lw_2(c, at(c.g[S0][0], 0x98));
            c.g[V1][0] = lw_2(c, c.g[A1][0] as u32);
            c.g[A0][0] = lh(c, at(c.g[V1][0], 0x18));
            c.g[V0][0] = lw_2(c, at(c.g[V1][0], 0x1c));
            let target = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x0023_0ec0);
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_0ec0)?;
            let t = c.g[V0][0] == 0;
            c.g[V0][0] = 0;
            if !t {
                'attempt: {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0023_0ed0);
                    c.g[A0][0] = c.g[S0][0];
                    callee::APP_MODULE_TRY_EXIT_2.call(c, 0x0023_0ed0)?;
                    let t = c.g[V0][0] == 0;
                    c.g[V0][0] = lw_2(c, gp_4(c, -0x854));
                    if t {
                        break 'attempt;
                    }
                    c.cycles += 21;
                    c.g[V1][0] = lw_2(c, at(c.g[V0][0], 0x10d8));
                    c.g[A0][0] = lh(c, at(c.g[V1][0], 0x78));
                    Addr(frame.at(c, 0)).sw(c, 0);
                    Addr(frame.at(c, 4)).sw(c, 0);
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    Addr(frame.at(c, 8)).sw(c, 0);
                    c.g[V0][0] = lw_2(c, at(c.g[V1][0], 0x7c));
                    let target = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x0023_0efc);
                    c.g[A1][0] = sx(frame.at(c, 0));
                    call_ptr(c, target, 0x0023_0efc)?;
                    c.g[V1][0] = lw_2(c, gp_4(c, -0x854));
                    c.g[V0][0] = 1;
                    c.g[A0][0] = lw_2(c, gp_4(c, -0x84c));
                    Addr(at(c.g[V1][0], 0x270)).sw(c, c.g[V0][0]);
                    Addr(gp_4(c, 0x1234)).sw(c, 0);
                    Addr(gp_4(c, 0x14e8)).sw(c, 0);
                    c.g[RA][0] = sx(0x0023_0f1c);
                    Addr(gp_4(c, 0x12d8)).sw(c, 0);
                    call_ptr(c, 0x0027_8718, 0x0023_0f1c)?;
                    c.g[RA][0] = sx(0x0023_0f24);
                    c.g[A0][0] = lw_2(c, at(c.g[S0][0], 0x94));
                    call_ptr(c, 0x001f_3170, 0x0023_0f24)?;
                    c.g[V0][0] = 1;
                    break 'end;
                }
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            break 'end;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}


/// `void GameModule_OnSectionUnload(GameModule *self, int section)`
/// (0x230360): notify every section manager and release the module's crowd
/// and rail resources.
pub fn game_module_on_section_unload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0035_8700, 0x0023_0384)?;
        c.g[S0][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2898));
        c.g[A0][0] = Addr(gp(c, -0x6f0)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0022_9408, 0x0023_0394)?;
        c.g[A0][0] = Addr(gp(c, 0xf00)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0034_3c08, 0x0023_03a0)?;
        c.g[A0][0] = Addr(gp(c, 0xf38)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0035_7b90, 0x0023_03ac)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        call_ptr(c, 0x0035_4c98, 0x0023_03b8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        c.g[A2][0] = c.g[S1][0];
        call_ptr(c, 0x0035_51a8, 0x0023_03c8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 8;
        c.g[A2][0] = c.g[S1][0];
        call_ptr(c, 0x0035_51a8, 0x0023_03d8)?;
        c.g[A0][0] = Addr(p_3(c.g[S2][0], 0xc)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = Addr(p_3(c.g[A0][0], 0xa4)).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 7;
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x0010_3308, 0x0023_03f8)?;
                c.g[V0][0] = Addr(p_3(c.g[S2][0], 0xc)).lw(c);
                c.g[A1][0] = 0x80;
                c.g[A0][0] = Addr(p_3(c.g[V0][0], 0xa4)).lw(c);
                call_ptr(c, 0x0010_3308, 0x0023_0408)?;
            }
        }
        c.g[A0][0] = Addr(p_3(c.g[S2][0], 0x30)).lw(c);
        c.cycles += 8;
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0030_8fe0, 0x0023_0414)?;
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
    use crate::difftest::check_paths;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::put_vec;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use ssx3_ee as ee;
    use crate::difftest::fakes;
    use crate::difftest::RA as SENTINEL;
    pub(crate) use crate::game::dirtysock::tags2::tests::rng_2 as rng;
    pub(crate) use crate::game::fe::festaterewards2::tests::SEEDS;
    pub(crate) use crate::game::fe::festaterewards2::tests::w;
    pub(crate) use crate::game::fe::festaterewards2::tests::belib;
    pub(crate) use crate::game::input::inputparse::tests::EMPTY;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::ai::ai::tests::w16;
    pub(crate) use crate::game::ai::ai::tests::slot;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::BE;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::WORLD;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals;
    pub(crate) use crate::game::bx::bxstring::tests::heap_init_4 as heap_init_2;
    pub(crate) use crate::game::bx::bxstring::tests::pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::gpat;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_8 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_4 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_5 as world_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_6 as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::IFACE;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_4 as W;
    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::wb;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_9 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_8 as run;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_10 as heap_init_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_10 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM_3 as RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::gp_at;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_3;


    #[test]
    fn game_module_get_transition_man_matches() {
        check(ssx3_ee::s06::f_00230698, game_module_get_transition_man, 100, &|c, seed| {
            let mut r = Rng(0x0c11_0006 ^ seed as u64);
            scramble(c, &mut r);
            let idx = (seed % 16) as u32;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = idx as u64;
            c.bus.write32(DATA + idx * 4 + 0x5c, r.next_u32());
        });
    }


    #[test]
    fn game_module_get_splash_manager_matches() {
        check(ssx3_ee::s06::f_002306a8, game_module_get_splash_manager, 100, &|c, seed| {
            let mut r = Rng(0x0c11_0007 ^ seed as u64);
            scramble(c, &mut r);
            let idx = (seed % 16) as u32;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = idx as u64;
            c.bus.write32(DATA + idx * 4 + 0x68, r.next_u32());
        });
    }


    #[test]
    fn app_module_ctor_matches() {
        check(ssx3_ee::s06::f_00231cd0, app_module_ctor, 64, &|c, seed| {
            let mut r = Rng(0x0c11_0008 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.bus.write32(GAME_GP.wrapping_sub(0x51b8), r.next_u32());
            for i in 0..3 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }


    #[test]
    fn pre_start_screen_ctor_matches() {
        check(ssx3_ee::s06::f_00232328, pre_start_screen_ctor, 64, &|c, seed| {
            let mut r = Rng(0x0c11_000a ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            for i in 0..3 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }


    #[test]
    fn check_pause_request_matches() {
        check(
            ssx3_ee::s06::f_00231840__generated,
            game_module_check_pause_request,
            300,
            &|c, seed| {
                let mut r = Rng(0x2318_4067 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let g = GAME_GP;
                // A loading screen, sometimes already frozen.
                let loading = DATA + 0x500;
                c.bus.write32(g - 0x204, match seed % 4 {
                    0 | 1 => 0,
                    _ => loading,
                });
                c.bus.write32(loading + 8, match seed % 3 {
                    0 => 0,
                    1 => 1,
                    _ => 0xffff_ffff,
                });
                // The confirm input evaluation runs the input VM; most seeds
                // skip it, some reach it with a stream that stops at once.
                c.bus.write32(g - 0x254, if seed % 5 == 0 { 0 } else { 1 });
                c.bus.write32(g - 0x84c, 0x002c_c098);
                let map = DATA + 0x6300;
                c.bus.write32(map + 4, DATA + 0x6400);
                c.bus.write32(map + 0x14, DATA + 0x6500);
                c.bus.write32(DATA + 0x6400 + 0xf0, 0);
                c.bus.write32(DATA + 0x6500, 0x20); // the VM's stop opcode
                // The cBE singleton and the interface slot it indexes.
                c.bus.write32(g - 0x1f04, 0x0080_9000);
                c.bus.write32(g - 0x1efc, 0);
                c.bus.write32(0x0053_08a8, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000, 0);
                c.bus.write32(DATA + 0x1004, DATA + 0x1200);
                c.bus.write16(DATA + 0x1208, 0);
                c.bus.write32(DATA + 0x120c, 0x002c_c098);
                // The game module and its player/rider managers.
                let game = DATA + 0x200;
                c.g[A0][0] = game as u64;
                c.bus.write32(game + 0x28, DATA + 0x3000);
                c.bus.write32(game + 0xc, DATA + 0x4000);
                c.bus.write32(game + 0x210, 0); // CanRestartFromPause: 0
                c.bus.write32(game + 0x224, r.next_u32());
                c.bus.write32(game + 0x228, r.next_u32());
                // One active player, or a rider list.
                let one = seed % 3 != 0;
                c.bus.write32(DATA + 0x3000 + 0x608, one as u32);
                c.bus.write32(DATA + 0x3000 + 0x28, (seed % 4) as u32);
                c.bus.write32(DATA + 0x4000 + 0x7c, (seed % 4) as u32);
                for i in 0..4u32 {
                    let rider = DATA + 0x4100 + i * 0x20;
                    c.bus.write32(DATA + 0x4000 + 0x40 + i * 4, rider);
                    c.bus.write32(rider + 0x18, DATA + 0x4200 + i * 0x20);
                    c.bus.write32(DATA + 0x4200 + i * 0x20 + 0x87c, ((seed + i) % 2) as u32);
                    // BENewPlayerInt slot table: slot i, player i.
                    c.bus.write32(0x0053_05b0 + i * 4, i);
                    c.bus.write32(0x0053_4fe0 + i * 0x1c + 0xc, i);
                }
                c.bus.write32(g - 0x848, DATA + 0x6000);
                for i in 0..4u32 {
                    c.bus.write32(DATA + 0x6000 + i * 4 + 0xb0, DATA + 0x6100 + i * 0x20);
                    let ready = (seed + i * 7) % 2 == 0;
                    c.bus.write32(DATA + 0x6100 + i * 0x20, if ready { DATA + 0x6200 } else { 0 });
                    c.bus.write32(DATA + 0x6100 + i * 0x20 + 4, map);
                    c.bus.write32(DATA + 0x6200, 1);
                }
                c.bus.write32(pause::RESTART, match seed % 3 {
                    0 => 0,
                    1 => 1,
                    _ => r.next_u32(),
                });
            },
        );
    }


    #[test]
    fn game_module_invalidate_focus_matches() {
        crate::difftest::check(ee::s06::f_00230338__generated, game_module_invalidate_focus, 200, &|c, seed| {
            let mut r = Rng(0x2303_3801 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = sx(DATA);
            match seed % 3 {
                0 => c.bus.write32(DATA + 0xc, 0),
                1 => {
                    c.bus.write32(DATA + 0xc, DATA + 0x100);
                    c.bus.write32(DATA + 0x100 + 0xa4, 0);
                }
                _ => {
                    c.bus.write32(DATA + 0xc, DATA + 0x100);
                    c.bus.write32(DATA + 0x100 + 0xa4, DATA + 0x200);
                    c.bus.write32(DATA + 0x200 + 0xd0, r.next_u32());
                }
            }
        });
    }


    // ------------------------------------------------------------ game state

    #[test]
    fn game_module_advance_state_matches() {
        check(ssx3_ee::s06::f_00231320__generated, game_module_advance_state, 150, &|c, seed| {
            let mut r = rng(seed, 0x1b);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.bus.write32(GAME_GP - 0x488, DATA + 0x3000);
            c.g[A0][0] = sx(DATA + 0x4000);
            let pending = DATA + 0x5000;
            c.bus.write32(DATA + 0x4000 + 0x200, if seed % 3 == 0 { 0 } else { pending });
            c.bus.write32(pending, if seed % 6 == 0 { 0 } else { r.next_u32() });
            c.bus.write32(pending + 4, r.next_u32());
        });
    }


    pub(crate) fn fb(x: f32) -> u32 {
        x.to_bits()
    }


    // ------------------------------------------------------- game and results

    #[test]
    fn app_module_try_exit_when_loaded_matches() {
        check(ssx3_ee::s06::f_00231c70__generated, app_module_try_exit_when_loaded, SEEDS, &|c, seed| {
            machine(c, seed, 0x231c_70, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            w(c, GAME_GP + 0x2a74, DATA + 0x100);
            let progress = if seed % 2 == 0 { 100.0 } else { 1.0 };
            w(c, DATA + 0x100 + 0x38, fb(progress));
        });
    }


    #[test]
    fn game_mode_man_reset_mode_matches() {
        check(ssx3_ee::s06::f_002382d8__generated, game_mode_man_reset_mode, SEEDS_2, &|c, seed| {
            let mut r = machine(c, seed, 0x2382d8, 0);
            let gm = DATA + 0x1000;
            c.g[A0][0] = gm as u64;
            let mode = (seed % 4) as u32;
            w32(c, gm + 4, mode);
            let obj = DATA + 0x3000 + mode * 0x100;
            let vt = DATA + 0x5000;
            w32(c, 0x0053_6668 + mode * 4, obj);
            w32(c, obj, vt);
            slot(c, vt, 0x10, 0, EMPTY);
            for i in 0..6u32 {
                w32(c, 0x0053_66d0 + 4 * i, r.next_u32());
            }
        });
    }


    #[test]
    fn game_module_end_replay_playback_matches() {
        check(
            ee::s06::f_002305c8__generated,
            game_module_end_replay_playback,
            SEEDS_3,
            &|c, seed| {
                machine(c, seed, 0x2305c8, 4);
                be_library(c);
                world(c);
                heap_init(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                w(c, this + 0x28, DATA + 0x400);
                w(c, this + 0xa4, DATA + 0x500);
                w(c, GAME_GP.wrapping_sub(0x69c), seed % 4);
            },
        );
    }


        #[test]
    fn game_module_reset_scripts_and_world_matches() {
        check(ee::s06::f_002300f0__generated, game_module_reset_scripts_and_world, 40, &|c, seed| {
            let _ = base(c, seed, 0x23_00f0);
            globals(c);
            pool(c);
            heap_init_2(c);
            w(c, DATA + 0x30, DATA + 0x1000);
            w(c, DATA + 0x34, DATA + 0x2000);
            w(c, DATA + 0xc, DATA + 0x3000);
            w(c, DATA + 0x10, DATA + 0x4000);
            w(c, GAME_GP.wrapping_add(0xce8), 0);
            let _ = seed;
            c.g[A0][0] = sx(DATA);
        });
    }


    #[test]
    fn game_module_quick_restart_matches() {
        check(
            ee::s06::f_002302a8__generated,
            game_module_quick_restart,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x2302, 0);
                env(c);
                let this = DATA + 0x400;
                let race = DATA + 0x1000;
                let n = (seed % 4) as u32;
                w32(c, this + 0xc, race);
                w32(c, race + 0x78, n);
                for i in 0..n {
                    w32(c, race + 0x28 + 4 * i, DATA + 0x1800 + 0x40 * i);
                }
                w32(c, gpat(0x410), 0);
                c.g[A0][0] = sx(this);
            },
        );
    }


    /// A vtable slot_2 pointing at a fixed translated function.
    pub(crate) fn slot_to(c: &mut Cpu, vt: u32, off: u32, target: u32) {
        wh(c, vt + off, 0);
        w(c, vt + off + 4, target);
    }


    pub(crate) const RET_ZERO: u32 = 0x001a_8828;


    // ------------------------------------------------------ reward database

        // --------------------------------------------------------- game module

    #[test]
    fn game_module_try_exit_matches() {
        check(
            ee::s06::f_00230e98__generated,
            game_module_try_exit,
            SEEDS_5,
            &|c, seed| {
                let _ = base_2(c, seed, 0x230e_98, 8);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                let module = obj + 0x1000;
                w(c, obj + 0x98, module);
                w(c, module, obj + 0x1100);
                if seed % 2 == 0 {
                    slot_to(c, obj + 0x1100, 0x1c, RET_ZERO);
                } else {
                    slot_2(c, obj + 0x1100, 0x1c, 0);
                }
                slot_to(c, obj + 0x1100, 0x18, RET_ZERO);
                // The world/race manager and the callback object.
                let game = W + 0x1000;
                w(c, game + 0x270, 0);
                w(c, IFACE + 0x10d8, obj + 0x2200);
                slot_2(c, obj + 0x2200, 0x78, 1);
                w(c, obj + 0x2200 + 0x7c, fakes::FAKE + 0x30);
                w(c, GAME_GP.wrapping_sub(0x84c), obj + 0x3000);
                w(c, obj + 0x94, obj + 0x3100);
            },
        );
    }


    #[test]
    fn game_module_on_section_unload_matches() {
        check(
            ee::s06::f_00230360__generated,
            game_module_on_section_unload,
            30,
            &|c, seed| {
                let _r = base_3(c, seed, 0x2303);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 16) as u64;
                c.bus.write32(gp_at(-0x6f0), DATA + 0x1000);
                c.bus.write32(gp_at(0xf00), DATA + 0x1100);
                c.bus.write32(gp_at(0xf38), DATA + 0x1200);
                c.bus.write32(DATA + 0xc, 0);
                c.bus.write32(DATA + 0x30, 0);
            },
        );
    }
}
