//! `src/fe/festateloadscreen.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
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
pub(crate) use crate::game::be::beintplayer::lh;
pub(crate) use crate::game::be::beintscore::gp as gp_2;
pub(crate) use crate::game::dirtysock::tags2::gp_2 as gp;
pub(crate) use crate::game::fe::feasyncfile::at;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_3;
pub(crate) use crate::game::fe::feasyncfile::lh_r;
pub(crate) use crate::game::fe::feasyncfile::low_2;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_2;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::fereal::open;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;
pub(crate) use crate::game::main::ssxappdtor::p;
pub(crate) use crate::game::movie::movieplayer::lw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2;
pub(crate) use crate::game::scripter::ssxscriptengine::sw_at;
pub(crate) use crate::game::world::wscriptcache::DELETE_2 as DELETE;
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
    (0x0012_8a48, ai_set_rider_order_mode, "AI_SetRiderOrderMode"),
    (0x0023_38c8, c_fe_load_screen_enter, "cFELoadScreen_Enter"),
    (0x0023_3930, cfe_load_screen_unload, "cFELoadScreen_Unload"),
    (0x0023_39f8, c_fe_load_screen_render, "cFELoadScreen_Render"),
    (0x0023_3a70, c_fe_load_screen_update, "cFELoadScreen_Update"),
    (0x0023_3aa0, game_state_goto_next, "GameState_GotoNext"),
    (0x0023_3b60, game_state_quit_enter, "GameStateQuit_Enter"),
    (0x0023_3c10, game_state_finish_ctor, "GameStateFinish_ctor"),
    (
        0x0023_3c50,
        game_state_finish_enter,
        "GameStateFinish_Enter",
    ),
    (0x0023_4008, game_state_finish_exit, "GameStateFinish_Exit"),
    (
        0x0023_4120,
        game_state_finish_load_state,
        "GameStateFinish_LoadState",
    ),
    (
        0x0023_4188,
        game_state_pre_race_load_ctor,
        "GameStatePreRaceLoad_ctor",
    ),
    (
        0x0023_4a30,
        game_state_pre_race_load_load_state,
        "GameStatePreRaceLoad_LoadState",
    ),
    (
        0x0028_fa98,
        ssx_audio_system_stop_loading_screen_loop,
        "SSXAudioSystem_StopLoadingScreenLoop",
    ),
];

// ---------------------------------------------------------------------------
// Callees the generator emitted as direct calls (the readable alias, which
// dispatches through the override slot) and as cross-shard `call_indirect`s.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::bx::bxstring2::callee::CROWD_MAN2D_REMOVE_ALL;
    pub(crate) use crate::game::bx::bxstring2::callee::CROWD_MAN2D_RESET;
    pub(crate) use crate::game::bx::bxstring2::callee::C_FE_STATE_LOBBY_MENU_FILL_PLAYER_LIST;
    pub(crate) use crate::game::bx::bxstring2::callee::C_FE_STATE_LOBBY_MENU_UPDATE_PLAYER_SCROLL_ARROWS;
    pub(crate) use crate::game::bx::bxstring2::callee::C_LUNO_TABLE_ALLOC;
    pub(crate) use crate::game::bx::bxstring2::callee::C_OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::bx::bxstring2::callee::EVENT_SELECT_ON_ACTIVATE;
    pub(crate) use crate::game::bx::bxstring2::callee::EVENT_SELECT_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::FE_MODULE_RIDER_PREVIEW_REQUEST_CHAR;
    pub(crate) use crate::game::bx::bxstring2::callee::GAME_MODULE_REQUEST_STATE as GAME_MODULE_REQUEST_STATE_2;
    pub(crate) use crate::game::bx::bxstring2::callee::PATH_DATA_PARSE;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_STOP_AUX_LISTS;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_STOP_MAIN_LISTS;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_LIST_GET_ENTRY_TYPE;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_LIST_PREPARE;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_LIST_SET_PAUSED_NESTED;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_INIT;
    pub(crate) use crate::game::bx::bxstring2::callee::SNAPSHOT_LIST_REMOVE;
    pub(crate) use crate::game::bx::bxstring2::callee::VID_ENGINE_PLAYER_START_PLAYBACK;
    pub(crate) use crate::game::bx::bxstring2::callee::WORLD_STATE_RESTORE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_INSTANCE_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOLT_FILE_INDEX_MANAGER_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAMERA_TRIGGER_STACK_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SETUP_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SHARED_LOGIN_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_VOIP_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::EVENT_SELECT_ON_WIDGET_NOTIFY;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_STATE_GOTO_NEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_CHANNEL_SET_LISTENER;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_INSTANCE_PAUSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_INSTANCE_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOAD_INDICATOR_CLEAR_BUSY;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_SEND_LOCK_CHAL;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_CLOSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_FLUSH;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_READ;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_WRITE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_ALLOC_NAMED_ALIGNED;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_CMD_QUEUE_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_CHOOSE_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_PLAY_STATUS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_GET_WORLD_SECTION_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_FREE_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_GET_BUFFER_SIZE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_RELEASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::WSCRIPT_MAN_DELETE_ALL_PROCESSES;
    pub(crate) use crate::game::dirtysock::tags2::callee::WSCRIPT_MAN_SPAWN_MISSIONS;
    pub(crate) use crate::game::fe::feasyncfile::callee::ACTIVE_CAMERA_TRIGGER_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_RENDER_FADE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_IS_TRACK_GROUP0_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_RESET_RACE_RESULT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_TRICK_METER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STORE_RACE_RESULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUTOSAVE_UPDATE_ICON;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COUNT_COLLECTED_ON_PEAK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COUNT_COMPLETED_ON_PEAK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_CHAR_PROFILE_EVENT_RECORD;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_CHARACTER_2 as BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INSTANCE_2 as BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INTERFACE_2 as BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_SLOT_3 as BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_CLEAR_EMITTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_INIT_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_STACK_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_ALLOC_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHALLENGE_POPUP_SET_SETTINGS_LOCKED;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_INIT_FROM_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_RESET_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CROWD_MAN3D_GROUP_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::CROWD_MAN3D_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::DEAD_NODE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FEMODULE_RIDER_PREVIEW_UNLOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_LOCALE_LOAD_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_ABORT_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_EMAIL_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_LOGIN_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_CREATED_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_ERROR_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_SHOW_ROOM_TAB;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_UPDATE_PLAYERS_IN_ROOM;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_ANIM_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_RIDER_PREVIEWS;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_BUILD_MESH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_SPRINGS_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_UPDATE_VERTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_RAIL_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_ANIM_MAP_SET_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_LOAD_UPDATE_PLAYER_RANKING;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_FINISH_CLEAR_FLAG40;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_LOAD_STATE_2 as GAME_STATE_LOAD_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_GET_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_EXPECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_GET_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_NEXT_SECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_READ_QUOTED_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_SKIP_WHITESPACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSTANCE_GET_NODE_MATRIX;
    pub(crate) use crate::game::fe::feasyncfile::callee::LIGHT_MAN_EMPTY_HOOK_D;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_REQUEST;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MENU_HIDE_ROOM_TABS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_ON_CARD_SCANNED;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCARD_INSTALL_CALLBACKS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_CARD_STATE_BASE_SHUTDOWN;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_SET_ACTIVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_EMPTY_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_IS_FLAG_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::MOD_OBJ_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_GET_BITS_2 as MPEG2_GET_BITS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_MARKER_BIT_2 as MPEG2_MARKER_BIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_11 as NET_FE_STATE_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_9 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_4 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_9 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_3 as NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_UPDATE_3 as NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::callee::PLAYBACK_RESTORE_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID_2 as POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_GET_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_MEM_CARD_CLEAR_DIR_CACHE;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_SET_STATE_2 as REPLAY_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_BUILD_WORLD_MATRIX;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_UPDATE_BOARD_LEAN;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_3 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_2 as SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_2 as SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CAM_BUILD_MATRIX_FROM_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SECTION_TABLE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SETUP_BUDDY_ACTION_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_FLUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_TREE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPLINE_MODIFIER_SET_SPEED;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_CTOR_2 as UI_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_4 as UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_2 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::VOIP_MAN_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::VOIP_MAN_DEACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_BLOCK_POOL_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CHUNK_RESOLVE_HOOK;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_MEMMAN_CREATE_SECTION_ALLOCATOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2 as WORLD_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2_2 as WORLD_OBJ_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_DTOR_2 as WORLD_OBJ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_RUN_KILL_SCRIPT_2 as WORLD_OBJ_RUN_KILL_SCRIPT;
    pub(crate) use crate::game::main::ssxappdtor::callee::APP_MODULE_UPDATE;
    pub(crate) use crate::game::main::ssxappdtor::callee::AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::main::ssxappdtor::callee::AUDIO_ON_EVENT_SELECT_ENTER;
    pub(crate) use crate::game::main::ssxappdtor::callee::AUDIO_PLAY_FE_SOUND;
    pub(crate) use crate::game::main::ssxappdtor::callee::BUY_ATTRIB_COMMIT_PURCHASE;
    pub(crate) use crate::game::main::ssxappdtor::callee::CAREER_STATS_SELECT_PAGE;
    pub(crate) use crate::game::main::ssxappdtor::callee::DELETE;
    pub(crate) use crate::game::main::ssxappdtor::callee::EVENT_SELECT_SET_MODE;
    pub(crate) use crate::game::main::ssxappdtor::callee::EVENT_SELECT_SHUTDOWN;
    pub(crate) use crate::game::main::ssxappdtor::callee::FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::main::ssxappdtor::callee::MESSAGE_CENTER_GET_INBOX;
    pub(crate) use crate::game::main::ssxappdtor::callee::NET_FE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::main::ssxappdtor::callee::OV_STATE_OVERLAY_SHUTDOWN;
    pub(crate) use crate::game::main::ssxappdtor::callee::REWARD_LIST_SHOW_PAGE;
    pub(crate) use crate::game::main::ssxappdtor::callee::SHARED_LOGIN_SET_ACCOUNT_NAME;
    pub(crate) use crate::game::main::ssxappdtor::callee::SHARED_LOGIN_SET_ACCOUNT_PASSWORD;
    pub(crate) use crate::game::main::ssxappdtor::callee::UI_ENGINE_UPDATE;
    pub(crate) use crate::game::main::ssxappdtor::callee::UI_STATE_ON_ACTIVATE;
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
    pub(crate) use crate::game::world::wscriptcache::callee::DELETE as DELETE_2;
    pub(crate) use crate::game::world::wscriptcache::callee::EA_MEM_FREE_2 as EA_MEM_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::FONT_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::INPUT_PREPROC_WRITE_STRING;
    pub(crate) use crate::game::world::wscriptcache::callee::MDF_FILE_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::MPEG2_FLUSH_BUFFER_3 as MPEG2_FLUSH_BUFFER;
    pub(crate) use crate::game::world::wscriptcache::callee::MPEG2_GET_BITS1;
    pub(crate) use crate::game::world::wscriptcache::callee::PS2_SONY_CODEC_INTERNAL_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::RCMP_SYSTEM_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::RIDER_SET_MAIN_STATE;
    pub(crate) use crate::game::world::wscriptcache::callee::RIDER_SET_SUB_STATE;
    pub(crate) use crate::game::world::wscriptcache::callee::SCRIPT_ENGINE_GET_INSTANCE;
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

// ---------------------------------------------------------------------------
// Game-state constructors. Each installs two vtables back to back (only the
// second survives at the same slot), clears the global game-state pointer at
// `gp-0x488`, and fills a small set of state words.

/// `GameStateFinish *GameStateFinish_ctor(GameStateFinish *self)` (0x233c10):
/// state words `+0=7`, `+4=0`, `+0xc=0x47d5e0`, `+0x10=2`, `+0x14=0`.
pub fn game_state_finish_ctor(c: &mut Cpu) {
    const VTABLE: u32 = 0x48_0000 - 0x2a20;
    let this = Addr(abi::arg(c, 0));
    c.cycles += 16;
    this.sw(c, 0);
    this.off(0xc).sw(c, sx(0x48_0000 - 0x2918));
    let vtable = sx(VTABLE);
    c.g[V1][0] = vtable;
    Addr((c.g[GP][0] as u32).wrapping_sub(0x488)).sw(c, 0);
    c.g[A1][0] = 7;
    c.g[A2][0] = 2;
    this.off(0xc).sw(c, vtable);
    c.g[V0][0] = c.g[A0][0];
    this.sw(c, 7);
    this.off(0x10).sw(c, 2);
    this.off(0x4).sw(c, 0);
    this.off(0x14).sw(c, 0);
    abi::ret(c);
}

/// `GameStatePreRaceLoad *GameStatePreRaceLoad_ctor(GameStatePreRaceLoad *self)`
/// (0x234188): state words `+0=2`, `+4=0`, `+8=0`, `+0xc=0x47d530`,
/// `+0x10=0`, `+0x14=0`, `+0x1c=4`.
pub fn game_state_pre_race_load_ctor(c: &mut Cpu) {
    const VTABLE: u32 = 0x48_0000 - 0x2ad0;
    let this = Addr(abi::arg(c, 0));
    c.cycles += 18;
    this.sw(c, 0);
    this.off(0xc).sw(c, sx(0x48_0000 - 0x2918));
    let vtable = sx(VTABLE);
    c.g[V1][0] = vtable;
    Addr((c.g[GP][0] as u32).wrapping_sub(0x488)).sw(c, 0);
    c.g[A1][0] = 2;
    c.g[A2][0] = 4;
    this.off(0xc).sw(c, vtable);
    c.g[V0][0] = c.g[A0][0];
    this.sw(c, 2);
    this.off(0x1c).sw(c, 4);
    this.off(0x4).sw(c, 0);
    this.off(0x10).sw(c, 0);
    this.off(0x14).sw(c, 0);
    this.off(0x8).sw(c, 0);
    abi::ret(c);
}

/// `void GameStateQuit_Enter(void)` (0x233b60): request game state 10 (quit).
pub fn game_state_quit_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[A0][0] = Addr(gp(c, -0x488)).lw(c);
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0xa;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        callee::GAME_MODULE_REQUEST_STATE.call(c, 0x0023_3b7c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `GameState_GotoNext(state)` 0x233aa0: if the state names a target state
/// (`*(state) != 0`), ask the game module for it with the delay at +4.
pub fn game_state_goto_next(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        let target = c.lw(this);
        c.g[A1][0] = target;
        let none = target == 0;
        c.g[A3][0] = 0;
        if !none {
            c.cycles += 3;
            c.g[A2][0] = lw_at(c, this as u64, 4);
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            callee::GAME_MODULE_REQUEST_STATE_2.call(c, 0x0023_3ac0)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFELoadScreen_Update(self)` (0x233a70): the front-end load screen's
/// update (same shape as the game one).
pub fn c_fe_load_screen_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.lw(p(r32(c, S0), 0xc));
        call_ptr(c, callee::UI_ENGINE_UPDATE, 0x0023_3a88)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::APP_MODULE_UPDATE, 0x0023_3a90)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SSXAudioSystem_StopLoadingScreenLoop(SSXAudioSystem* s)` (0x28fa98):
/// when a loading-screen loop is playing (`s+0x62a8` >= 0), fade it out and
/// clear the index.
pub fn ssx_audio_system_stop_loading_screen_loop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(0x62a8));
        let idle = (c.g[A1][0] as i64) < 0;
        c.g[A2][0] = 1;
        if !idle {
            c.cycles += 6;
            c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x118));
            c.g[A0][0] = c.lw(r32(c, V0));
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x1d8));
            callee::BANK_MONITOR_DELETE_INSTANCE.call(c, 0x0028_fac4)?;
            c.g[V0][0] = u64::MAX;
            c.sw(r32(c, S0).wrapping_add(0x62a8), c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFELoadScreen_Enter()` (0x2338c8): the front-end twin of
/// [`c_game_load_screen_enter`].
pub fn c_fe_load_screen_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = lw(c, gp_2(c, -0x854));
        frame.save_ra(c, 0x10);
        call_ptr(c, 0x0036_9ff8, 0x0023_38dc)?;
        c.g[A0][0] = lw(c, gp_2(c, -0x848));
        call_ptr(c, 0x0022_8238, 0x0023_38e4)?;
        sw(c, gp_2(c, 0x1234), 0);
        let g = lw(c, gp_2(c, -0x854));
        c.g[V0][0] = g;
        c.g[V1][0] = lw(c, p(g as u32, 0x10d8));
        c.g[A0][0] = lh(c, p(c.g[V1][0] as u32, 0x78));
        sw(c, sp(c), 0);
        sw(c, p(sp(c), 4), 0);
        c.g[A0][0] = sx((g as u32).wrapping_add(c.g[A0][0] as u32));
        sw(c, p(sp(c), 8), 0);
        c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, 0x7c));
        let target = c.g[V0][0] as u32;
        c.g[A1][0] = sp(c) as u64;
        call_ptr(c, target, 0x0023_3910)?;
        call_ptr(c, 0x0028_b180, 0x0023_3918)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0028_f768, 0x0023_3920)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void GameStateFinish_LoadState(self, stream)` (0x234120): run the base
/// load, then read the two 4-byte fields through stream slot 2.
pub fn game_state_finish_load_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        open(c, 0x30);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A1][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[RA][0] = sx(0x0023_413c);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, 0x0023_3b28, 0x0023_413c)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32));
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
        c.g[A2][0] = 0x4;
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let target = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0023_4158);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, target, 0x0023_4158)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32));
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x14));
        c.g[A2][0] = 0x4;
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let target = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0023_4174);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, target, 0x0023_4174)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x30);
        Ok(())
    })
}

/// `AI_SetRiderOrderMode` (0x128A48): store the ordering mode and number every
/// rider (or clear all their order fields when the mode is 0).
pub fn ai_set_rider_order_mode(c: &mut Cpu) {
    c.cycles += 2;
    let enabled = c.g[A1][0] != 0;
    sw_at(c, A0, 0x74, c.g[A1][0]);
    if enabled {
        c.cycles += 3;
        c.g[V0][0] = lw_at_2(c, A0, 0x78);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[V1][0] = 0;
        if empty {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        c.g[A1][0] = low(c, A0).wrapping_add(0x28);
        loop {
            c.cycles += 7;
            c.g[V0][0] = Addr(r32(c, A1)).lw(c);
            sw_at(c, V0, 0xec, c.g[V1][0]);
            c.g[V1][0] = low(c, V1).wrapping_add(1);
            c.g[V0][0] = lw_at_2(c, A0, 0x78);
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = low(c, A1).wrapping_add(4);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0012_8a98);
                }
                continue;
            }
            c.cycles += 2;
            abi::ret(c);
            return;
        }
    }
    c.cycles += 3;
    c.g[V0][0] = lw_at_2(c, A0, 0x78);
    let empty = (c.g[V0][0] as i64) <= 0;
    c.g[A2][0] = 0;
    if empty {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    c.g[A1][0] = low(c, A0).wrapping_add(0x28);
    loop {
        c.cycles += 7;
        c.g[V1][0] = Addr(r32(c, A1)).lw(c);
        c.g[A2][0] = low(c, A2).wrapping_add(1);
        sw_at(c, V1, 0xec, 0);
        c.g[V0][0] = lw_at_2(c, A0, 0x78);
        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A1][0] = low(c, A1).wrapping_add(4);
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0012_8a60);
            }
            continue;
        }
        break;
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Screen rendering.

/// The shared body of the three `Render` methods (0x232da8, 0x233390,
/// 0x2339f8): look up the app module (`gp-0x854`), call its `canRender` slot
/// (`vtable[0x88]/[0x8c]`); unless it says no, draw the fade
/// (`AppModule_RenderFade`), optionally refresh the widget (`0x397df8`), then
/// call the module's `render` slot (`vtable[0xa0]/[0xa4]`) and return 1.
pub(crate) fn render_screen(c: &mut Cpu, rets: [u32; 4]) -> Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x20);
    c.g[A1][0] = lw_r(c, GP, 0u32.wrapping_sub(0x854));
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    c.g[V1][0] = lw_r(c, A1, 0x10d8);
    c.g[A0][0] = lh_r(c, V1, 0x88);
    c.g[V0][0] = lw_r(c, V1, 0x8c);
    let target = low_2(c, V0);
    c.g[RA][0] = sx(rets[0]);
    c.g[A0][0] = sx(low_2(c, A1).wrapping_add(low_2(c, A0)));
    call_ptr(c, target, rets[0])?;
    let hidden = c.g[V0][0] == 0;
    c.g[V0][0] = 0;
    if hidden {
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        return Ok(());
    }
    c.cycles += 4;
    c.g[RA][0] = sx(rets[1]);
    c.g[A0][0] = c.g[S0][0];
    callee::APP_MODULE_RENDER_FADE.call(c, rets[1])?;
    let faded = c.g[V0][0] != 0;
    c.g[A1][0] = lw_r(c, GP, 0u32.wrapping_sub(0x854));
    if !faded {
        c.cycles += 3;
        c.g[RA][0] = sx(rets[2]);
        c.g[A0][0] = lw_r(c, S0, 0xc);
        call_ptr(c, 0x0039_7df8, rets[2])?;
        c.g[A1][0] = lw_r(c, GP, 0u32.wrapping_sub(0x854));
    }
    c.cycles += 6;
    c.g[V0][0] = lw_r(c, A1, 0x10d8);
    c.g[A0][0] = lh_r(c, V0, 0xa0);
    c.g[V1][0] = lw_r(c, V0, 0xa4);
    let target = low_2(c, V1);
    c.g[RA][0] = sx(rets[3]);
    c.g[A0][0] = sx(low_2(c, A1).wrapping_add(low_2(c, A0)));
    call_ptr(c, target, rets[3])?;
    c.g[V0][0] = 0x1;
    c.cycles += 4;
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `cFELoadScreen_Render` (0x2339f8).
pub fn c_fe_load_screen_render(c: &mut Cpu) {
    guest(c, |c| {
        render_screen(c, [0x0023_3a20, 0x0023_3a30, 0x0023_3a40, 0x0023_3a58])
    })
}

// ---------------------------------------------------------------------------
// Game state.

/// `void GameStateFinish_Enter(GameStateFinish* self)` (0x233c50): stop the
/// replay recorder, close the UI states, stop the script list, mark the
/// loading indicator busy and flag the finished race.
///
/// Tick rate: the finish screen's delay `+0x14` (180 frames at 60 Hz) is
/// taken down by 1 per tick, so it is stored as
/// [`crate::tick::ticks_from_frames60`]`(180)`.
pub fn game_state_finish_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = Addr(gp(c, -0x488)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[RA][0] = sx(0x0023_3c6c);
        c.g[A0][0] = Addr(p(r32(c, V0), 0x28)).lw(c);
        call_ptr(c, 0x0026_fa50, 0x0023_3c6c)?;
        c.g[V0][0] = Addr(gp(c, -0x488)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lw(c);
        c.g[RA][0] = sx(0x0023_3c7c);
        c.g[A0][0] = sx(p(r32(c, A0), 0x18));
        call_ptr(c, 0x0039_f840, 0x0023_3c7c)?;
        c.g[A0][0] = Addr(gp(c, -0x84c)).lw(c);
        c.g[RA][0] = sx(0x0023_3c88);
        c.g[A1][0] = 0;
        call_ptr(c, 0x0027_90a0, 0x0023_3c88)?;
        // The delay before the finish screen: 180 frames (3 s), counted down
        // once per tick by GameStateFinish_Update, so stored in ticks.
        c.g[V0][0] = crate::tick::ticks_from_frames60(c, 180) as u64;
        Addr(p(r32(c, S0), 0x10)).sw(c, 0);
        Addr(p(r32(c, S0), 0x14)).sw(c, c.g[V0][0]);
        c.g[A1][0] = 0x4;
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        c.g[V1][0] = Addr(p(r32(c, V0), 0x84)).lw(c);
        c.g[RA][0] = sx(0x0023_3ca8);
        c.g[A0][0] = Addr(p(r32(c, V1), 0x94)).lw(c);
        call_ptr(c, 0x001f_36c0, 0x0023_3ca8)?;
        c.g[S0][0] = Addr(gp(c, -0x204)).lw(c);
        if c.g[S0][0] != 0 {
            c.cycles += 5;
            c.g[RA][0] = sx(0x0023_3cbc);
            c.g[A0][0] = c.g[S0][0];
            callee::GAME_STATE_FINISH_CLEAR_FLAG40.call(c, 0x0023_3cbc)?;
            c.g[V0][0] = 0x1;
            Addr(p(r32(c, S0), 0x90)).sw(c, c.g[V0][0]);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GameStatePreRaceLoad_LoadState(self, stream)` (0x234a30): base load
/// plus four 4-byte stream fields (0x10, 0x14, 0x1c, 0x8).
pub fn game_state_pre_race_load_load_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S1][0] = sx(this);
        callee::GAME_STATE_LOAD_STATE.call(c, 0x0023_4a4c)?;
        for (off, ret) in [
            (0x10u32, 0x0023_4a68u32),
            (0x14, 0x0023_4a84),
            (0x1c, 0x0023_4aa0),
            (0x8, 0x0023_4abc),
        ] {
            c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(off));
            c.g[A2][0] = 4;
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x10).lh(c);
            c.g[V0][0] = lw_2(c, V1, 0x14);
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

// ---------------------------------------------------------------------------
// `GameStateFinish_Exit` (0x234008): restore the default camera on every
// rider, then arm the script engine's skip delay/list and set the finish
// state.
pub fn game_state_finish_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
        c.g[S1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
        let none = (c.g[S1][0] as i64) <= 0;
        c.g[S0][0] = 0u64;
        if !none {
            c.cycles += 2;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            loop {
                c.cycles += 11;
                c.g[A1][0] = sx((c.g[S0][0] as u32) << 2);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa8));
                call_ptr(c, 0x00162290, 0x0023_4060)?;
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0023_4040);
                }
            }
        }
        c.cycles += 18;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
        c.g[V0][0] = 0x2;
        c.g[A2][0] = 0x1;
        c.g[A3][0] = 0u64;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x00278f68, 0x0023_4088)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x00278de8, 0x0023_4094)?;
        c.g[V0][0] = 0x7;
        c.lq(16, frame.at(c, 0x30));
        c.sw((c.g[S2][0] as u32), c.g[V0][0]);
        c.lq(17, frame.at(c, 0x20));
        c.lq(18, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Load screen, avalanche init, menu cursor, transition and standings helpers.

/// `void cFELoadScreen_Unload(cFELoadScreen *self)` (0x233930): destroy the
/// screen's UI objects, release the load-screen graphics and the meter.
pub fn cfe_load_screen_unload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = Addr(gp_3(c, -0x854)).lw(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p_2(c.g[A1][0], 0x10d8)).lw(c);
        c.g[A0][0] = Addr(p_2(c.g[V0][0], 0x98)).lh(c);
        c.g[V1][0] = Addr(p_2(c.g[V0][0], 0x9c)).lw(c);
        let first = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, first, 0x0023_395c)?;
        c.g[A1][0] = Addr(p_2(c.g[S0][0], 0x18)).lw(c);
        let none = (c.g[A1][0] as i64) < 0;
        c.g[A2][0] = Addr(gp_3(c, -0x854)).lw(c);
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(p_2(c.g[A2][0], 0x10d8)).lw(c);
            c.g[A0][0] = Addr(p_2(c.g[V0][0], 0x190)).lh(c);
            c.g[V1][0] = Addr(p_2(c.g[V0][0], 0x194)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_397c)?;
        }
        c.cycles += 3;
        c.g[A0][0] = Addr(p_2(c.g[S0][0], 0xc)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 0x3;
            call_ptr(c, 0x0039_7b70, 0x0023_3990)?;
        }
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
        c.cycles += 1;
        c.g[S1][0] = 1;
        loop {
            c.cycles += 3;
            c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
            let none = c.g[A1][0] == 0;
            c.g[A2][0] = Addr(gp_3(c, -0x854)).lw(c);
            if !none {
                c.cycles += 5;
                c.g[V0][0] = Addr(p_2(c.g[A2][0], 0x10d8)).lw(c);
                c.g[A0][0] = Addr(p_2(c.g[V0][0], 0x190)).lh(c);
                c.g[V1][0] = Addr(p_2(c.g[V0][0], 0x194)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0023_39b8)?;
            }
            c.cycles += 3;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0023_3998);
            }
        }
        c.cycles += 13;
        c.g[A0][0] = Addr(gp_3(c, -0x854)).lw(c);
        call_ptr(c, 0x0036_a020, 0x0023_39cc)?;
        call_ptr(c, 0x0028_b180, 0x0023_39d4)?;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 12);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0028_fa98, 0x0023_39e4)?;
        frame.restore(c, S0, 0x20);
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
    use crate::difftest::check_paths;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as RET;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::SP as STACK;
    pub(crate) use crate::game::bx::bxstring2::tests::base_3 as base_2;
    pub(crate) use crate::game::bx::bxstring2::tests::gp_at;
    pub(crate) use crate::game::bx::bxstring2::tests::BE_2 as BE;
    pub(crate) use crate::game::bx::bxstring2::tests::DB;
    pub(crate) use crate::game::bx::bxstring2::tests::RIDERS;
    pub(crate) use crate::game::bx::bxstring2::tests::RM;
    pub(crate) use crate::game::bx::bxstring2::tests::VTS;
    pub(crate) use crate::game::bx::bxstring2::tests::W_2 as W;
    pub(crate) use crate::game::dirtysock::tags2::tests::rng_2 as rng;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::clear;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_9 as globals_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::h;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_9 as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_7 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_3 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_2;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY_PTR;
    pub(crate) use crate::game::movie::movieplayer::tests::globals;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_LIBRARY;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS;
    use ssx3_ee as ee;

    #[test]
    fn game_state_finish_ctor_matches() {
        check(
            ssx3_ee::s06::f_00233c10,
            game_state_finish_ctor,
            64,
            &|c, seed| {
                let mut r = Rng(0x0c11_000d ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.bus.write32(GAME_GP.wrapping_sub(0x488), r.next_u32());
                for i in 0..8 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn game_state_pre_race_load_ctor_matches() {
        check(
            ssx3_ee::s06::f_00234188,
            game_state_pre_race_load_ctor,
            64,
            &|c, seed| {
                let mut r = Rng(0x0c11_000e ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.bus.write32(GAME_GP.wrapping_sub(0x488), r.next_u32());
                for i in 0..8 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn game_state_quit_enter_matches() {
        check(
            ssx3_ee::s06::f_00233b60__generated,
            game_state_quit_enter,
            100,
            &|c, seed| {
                let mut r = rng(seed, 0x1c);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP - 0x488, DATA + 0x3000);
            },
        );
    }

    #[test]
    fn game_state_goto_next_matches() {
        check(
            ssx3_ee::s06::f_00233aa0__generated,
            game_state_goto_next,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0xb2, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
                let target = if seed % 2 == 0 {
                    0
                } else {
                    r.next_u32() % 0x40
                };
                c.bus.write32(DATA, target);
                c.bus.write32(DATA + 4, r.next_u32() % 0x100);
            },
        );
    }

    #[test]
    fn c_fe_load_screen_update_matches() {
        check(
            ee::s06::f_00233a70__generated,
            c_fe_load_screen_update,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x233a70, 4);
                globals_2(c);
                clear(c, DATA, 0x1000);
                c.g[A0][0] = DATA as u64;
                // UIEngine -> (+0xc) view -> (+8) vtable -> +0x170 slot.
                let engine = DATA + 0x800;
                let view = DATA + 0x900;
                c.bus.write32(DATA + 0xc, engine);
                c.bus.write32(engine + 0xc, view);
                let vt = vtable(c, &mut r, &[0x170]);
                c.bus.write32(view + 8, vt);
                let _ = seed;
            },
        );
    }

    #[test]
    fn ssx_audio_system_stop_loading_screen_loop_matches() {
        check(
            ee::s07::f_0028fa98__generated,
            ssx_audio_system_stop_loading_screen_loop,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28_fa98, 0);
                let obj = DATA + 0x1000;
                c.bus.write32(
                    obj + 0x62a8,
                    match seed % 3 {
                        0 => u32::MAX,
                        1 => 3,
                        _ => 0,
                    },
                );
                c.bus.write32(obj + 0x118, DATA + 0x2000);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn c_fe_load_screen_enter_matches() {
        check(
            ssx3_ee::s06::f_002338c8__generated,
            c_fe_load_screen_enter,
            30,
            &|c, seed| {
                let mut r = setup(c, seed, 0x2338c8);
                c.bus.write32(GAME_GP.wrapping_sub(0x854), DATA + 0x200);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x300);
                let rm = DATA + 0x400;
                c.bus.write32(DATA + 0x200 + 0x10d8, rm);
                c.bus.write32(rm + 0x78, 0);
                c.bus.write32(rm + 0x7c, 0);
                c.bus.write32(DATA + 0x300 + 0x7c, 0);
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn game_state_finish_load_state_matches() {
        check(
            ee::s06::f_00234120__generated,
            game_state_finish_load_state,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x23_4120, 3);
                // The stream's vtable slot 2 (`+0x10`) is the fake field reader.
                let stream = vtable(c, &mut r, &[0x10]);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(stream);
            },
        );
    }

    // ------------------------------------------------------------- AI / riders

    #[test]
    fn ai_set_rider_order_mode_matches() {
        check(
            ee::s00::f_00128a48__generated,
            ai_set_rider_order_mode,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x12_8a48, 0);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.g[A1][0] = (seed % 2) as u64;
                let n = (seed % 4) as u32;
                w(c, DATA + 0x2000 + 0x78, n);
                for i in 0..4u32 {
                    let rider = DATA + 0x2400 + 0x40 * i;
                    w(c, DATA + 0x2000 + 0x28 + 4 * i, rider);
                    w(c, rider + 0xec, 0xffff_ffff);
                }
                c.pc = 0x0012_8a48;
            },
        );
    }

    #[test]
    fn c_fe_load_screen_render_matches() {
        check(
            ee::s06::f_002339f8__generated,
            c_fe_load_screen_render,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x2339f8, 4);
                be_library(c);
                world(c);
                heap_init(c);
                let app = DATA + 0x2000;
                w(c, GAME_GP.wrapping_sub(0x854), app);
                c.g[A0][0] = DATA as u64;
                let vt = DATA + 0x3000;
                w(c, app + 0x10d8, vt);
                slot(c, vt, 0x88, 0, FAKE);
                slot(c, vt, 0xa0, 0, FAKE + 0x10);
                w(c, DATA + 0xc, DATA + 0x400);
            },
        );
    }

    // ------------------------------------------------------------------
    // Game state.

    fn game_state_finish_enter_fixture(c: &mut Cpu, seed: u32, calls: u32) {
        let mut r = machine(c, seed, 0x233c50, calls);
        be_library(c);
        world(c);
        c.g[A0][0] = (DATA + 0x100) as u64;
        let replay = DATA + 0x200;
        w(c, GAME_GP.wrapping_sub(0x488), replay);
        w(c, replay + 0x28, 0);
        w(c, replay + 0x48, DATA + 0x280);
        w(c, GAME_GP.wrapping_sub(0x84c), 0);
        w(
            c,
            GAME_GP.wrapping_sub(0x204),
            if seed % 3 == 0 { 0 } else { DATA + 0x300 },
        );
        let _ = r.next_u32();
    }

    /// The finish screen's 180-frame delay is stored in ticks (3 s at any
    /// rate).
    #[test]
    fn game_state_finish_enter_delay_follows_rate() {
        for (hz, ticks) in [(60, 180), (120, 360), (240, 720)] {
            let mut c = crate::difftest::fresh();
            game_state_finish_enter_fixture(&mut c, 1, 0);
            crate::tick::set_for_test(&mut c, hz);
            game_state_finish_enter(&mut c);
            assert_eq!(c.pc, TEST_RA);
            assert_eq!(c.bus.read32(DATA + 0x100 + 0x14), ticks, "{hz} Hz");
        }
    }

    #[test]
    fn game_state_finish_enter_matches() {
        check(
            ssx3_ee::s06::f_00233c50__generated,
            game_state_finish_enter,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x233c50, 6);
                be_library(c);
                world(c);
                let st = DATA + 0x100;
                c.g[A0][0] = st as u64;
                // gp-0x488: the replay manager; +0x28 and +0x48 its objects.
                let replay = DATA + 0x200;
                w(c, GAME_GP.wrapping_sub(0x488), replay);
                w(c, replay + 0x28, 0);
                w(c, replay + 0x48, DATA + 0x280);
                // gp-0x84c: the script engine for ScriptEngine_ListStop.
                w(c, GAME_GP.wrapping_sub(0x84c), 0);
                // gp-0x204: the GameStateFinish instance (or none).
                if seed % 3 == 0 {
                    w(c, GAME_GP.wrapping_sub(0x204), 0);
                } else {
                    w(c, GAME_GP.wrapping_sub(0x204), DATA + 0x300);
                }
                let _ = r.next_u32();
            },
        );
    }

    pub(crate) fn r32_seed(seed: u32) -> u32 {
        seed.wrapping_mul(0x9e37_79b9) ^ 0x1234
    }

    #[test]
    fn game_state_pre_race_load_load_state_matches() {
        check(
            ssx3_ee::s06::f_00234a30__generated,
            game_state_pre_race_load_load_state,
            10,
            &|c, seed| {
                let _r = base(c, seed, 0x234a30, 2);
                let this = DATA + 0x1000;
                let stream = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = stream as u64;
                w_2(c, stream, DATA + 0x3000);
                c.bus.write16(DATA + 0x3000 + 0x10, 0);
                w_2(c, DATA + 0x3000 + 0x14, 0);
                w_2(c, this + 0x10, r32_seed(seed));
            },
        );
    }

    #[test]
    fn game_state_finish_exit_matches() {
        check(
            ee::s06::f_00234008__generated,
            game_state_finish_exit,
            40,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x2340);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(RIDERS + 0x10, (seed % 2) as u32);
                c.bus.write32(RIDERS + 4, DATA + 0x600);
                c.bus.write32(DATA + 0x600 + 0xa8, DATA + 0x700);
                c.bus.write32(gp_at(-0x84c), DATA + 0x800);
                fill(c, &mut r, DATA + 0x700, 0x40);
                fill(c, &mut r, DATA + 0x800, 0x40);
                c.bus.write32(DATA + 0x10, 0);
            },
        );
    }

    #[test]
    fn cfe_load_screen_unload_matches() {
        check(
            ee::s06::f_00233930__generated,
            cfe_load_screen_unload,
            40,
            &|c, seed| {
                let _r = base_3(c, seed, 0x2339);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x18, (seed % 4) as u32);
                c.bus
                    .write32(DATA + 0xc, if seed % 3 == 0 { 0 } else { DATA + 0x1000 });
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(DATA + 0x14, seed as u32);
            },
        );
    }
}
