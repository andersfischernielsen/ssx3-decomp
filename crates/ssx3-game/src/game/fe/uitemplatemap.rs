//! `src/fe/uitemplatemap.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::dirtysock::tags2::arg;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::debugfe::gp;
pub(crate) use crate::game::fe::debugfe::r32_2;
pub(crate) use crate::game::fe::feasyncfile::lw_8 as lw_2;
pub(crate) use crate::game::fe::festateload::event_select_notify;
pub(crate) use crate::game::fe::ovstatepause::lw;
pub(crate) use crate::game::main::ssxappdtor::p;
pub(crate) use crate::game::movie::movieplayer::at as at_2;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::forward;
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
    (
        0x0020_0ac0,
        event_select_on_activate,
        "EventSelect_OnActivate",
    ),
    (0x0020_0af0, event_select_shutdown, "EventSelect_Shutdown"),
    (
        0x0020_0d00,
        event_select_is_event_unlocked,
        "EventSelect_IsEventUnlocked",
    ),
    (
        0x0020_1868,
        event_select_hide_map_pic,
        "EventSelect_HideMapPic",
    ),
    (
        0x0020_2738,
        event_select_on_widget_notify,
        "EventSelect_OnWidgetNotify",
    ),
    (
        0x0020_4ff0,
        event_select_get_medal_text,
        "EventSelect_GetMedalText",
    ),
    (
        0x0020_83d8,
        event_select_set_highlight_color,
        "EventSelect_SetHighlightColor",
    ),
    (
        0x0020_8438,
        event_select_set_unlocked_color,
        "EventSelect_SetUnlockedColor",
    ),
    (0x0020_8498, ov_screen_map_init, "OVScreenMap_Init"),
    (
        0x0020_8558,
        ov_screen_map_on_activate,
        "OVScreenMap_OnActivate",
    ),
    (0x0020_8588, ov_screen_map_shutdown, "OVScreenMap_Shutdown"),
    (
        0x0020_8610,
        ov_screen_map_on_widget_notify,
        "OVScreenMap_OnWidgetNotify",
    ),
    (
        0x0020_8638,
        ov_screen_map_handle_input,
        "OVScreenMap_HandleInput",
    ),
    (0x0020_8678, ov_screen_map_update, "OVScreenMap_Update"),
];

// ---------------------------------------------------------------------------
// Callees (generated shard functions, reached through their override slots).

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
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_1;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::BEREWARD_INT_IS_PEAK_COMPLETE;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_SCORE_INT_UPDATE_BEST_STATS;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_DISCONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_IS_CONNECTED;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_RECONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_ALLOC_COPY;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_CTOR_COPY;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_DTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::CBE_GET_BE;
    pub(crate) use crate::game::movie::movieplayer::callee::CBE_GET_INTERFACE;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FE_STATE_CHAR_EQUIP_SET_BUY_ROW;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FE_STATE_CHAR_EQUIP_SET_EQUIP_ROW;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FLAG_MANAGER_GROUP_INIT_KEY;
    pub(crate) use crate::game::movie::movieplayer::callee::C_OV_STATE_OVERLAY_INIT;
    pub(crate) use crate::game::movie::movieplayer::callee::EVENT_SELECT_INIT;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_APPLY_BACKGROUND_SCALE;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_APPLY_CONFIG;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_RESET_LAYOUT;
    pub(crate) use crate::game::movie::movieplayer::callee::FILE_LOAD;
    pub(crate) use crate::game::movie::movieplayer::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES;
    pub(crate) use crate::game::movie::movieplayer::callee::FONT_PARSE_GLYPHS;
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_OVERLAY_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_STATE as GAME_MODULE_REQUEST_STATE_3;
    pub(crate) use crate::game::movie::movieplayer::callee::GET_HASH_VALUE_32;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN_DISCONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_CREATE_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_JOIN_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_END_OPERATION_4 as MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_ON_OPERATION_DONE;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_SET_MODE_2 as MC_MANAGER_SET_MODE;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_ALLOC_NAMED;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_FREE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_ITEM_ON_FOCUS;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_PAGE_ON_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_PAGE_SET_SELECTION;
    pub(crate) use crate::game::movie::movieplayer::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER;
    pub(crate) use crate::game::movie::movieplayer::callee::MULTI_SPLINE_MODIFIER_ADD_REF;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_SET_QUALITY_ICONS;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_CTOR_5 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_HANDLE_NET_EVENT;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::movie::movieplayer::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PDA_TEMPLATE_OPEN;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PROFILE_SAVE_PDA_INIT_SLOT_LABELS;
    pub(crate) use crate::game::movie::movieplayer::callee::PARENT_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_ON_SPLIT_MARKER;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_RESET_STATS;
    pub(crate) use crate::game::movie::movieplayer::callee::ROLLER_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_CMD_QUEUE_CLEAR as SCRIPT_CMD_QUEUE_CLEAR_2;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_CMD_QUEUE_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_FIND_SOUND_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS as SCRIPT_ENGINE_GET_PROCESS_3;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_RUN_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_UNLOAD_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::SPRINTF_2 as SPRINTF;
    pub(crate) use crate::game::movie::movieplayer::callee::STRCPY_2;
    pub(crate) use crate::game::movie::movieplayer::callee::STREAM_BUFFER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::TAG_FIELD_GET_STRING;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_LOAD_TEXTURES;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_STORE_WORD;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_CTOR_2 as UI_STATE_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_MGR_ADD_BACKGROUND_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESSES;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESS_COUNT;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_MAN_CREATE_PROCESS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::AI_CREATE_ACTOR_RIDERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::APP_VSYNC_CALLBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_CLEAR_DATA;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_REMOVE_ALL_TRIGGERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_STACK_CLEAR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_LOAD_STATE_SETUP_RESULT_WIDGET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_NET_ACCOUNT_MEM_SET_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_OPTIONS_CONTROLLER_SET_VIBRATION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CHASE_CAM_CONTROLLER_RESET_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::DYNAMIC_EMITTER_FREE_DATA_2 as DYNAMIC_EMITTER_FREE_DATA;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::EVENT_SELECT_ON_WIDGET_ACTION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_ALL_TABLES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_PREPROC_WRITE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_TEX_FLIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MOD_OBJ_CREATE_MODIFIER_BLOCK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MULTI_PARTICLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::NET_FE_STATE_DTOR_3 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_PLAYBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_RECORDING;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RCMP_DECODER_RELEASE_CODEC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_CACHE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_ADD_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_BONUS_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_SHOW_HUD_NUMBER_2 as RIDER_METRIX_SHOW_HUD_NUMBER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE_2 as SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_EXIT_SCRIPT_MODE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_PROCESS_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_TARGET_OBJECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_OBJ_TABLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_ON_SEQ_END;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_STOP_2 as SCRIPT_LIST_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_UNLOAD_MEMORY_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_DTOR;
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
}

/// Cross-shard calls the generator emitted as `call_indirect` (no alias).
pub(crate) mod table {
    pub(crate) use crate::game::movie::movieplayer::table::LOBBY_LIST_GET_NAME;
    pub(crate) use crate::game::movie::movieplayer::table::MC_CARD_DESTROY;
    pub(crate) use crate::game::movie::movieplayer::table::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::movie::movieplayer::table::SSX_AUDIO_SYSTEM_SET_GAME_STATE;
    pub(crate) use crate::game::movie::movieplayer::table::UI_LIST_SET_FIRST_FLAGS;
    pub(crate) use crate::game::movie::movieplayer::table::UI_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::table::UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::movie::movieplayer::table::VOIP_RPC_CALL;
}

// ---------------------------------------------------------------------------
// Menus / race progress / crowd.

/// `int EventSelect_OnWidgetNotify(EventSelect *this, int unused, int notify)`
/// (0x202738): maps widget notifications 6/9/8 to 0, anything else to 0x101.
pub fn event_select_on_widget_notify(c: &mut Cpu) {
    let notify = c.g[A2][0];
    c.cycles += 3;
    c.g[V0][0] = 6;
    let is_six = notify == c.g[V0][0];
    c.g[V0][0] = 9;
    if !is_six {
        c.cycles += 2;
        let is_nine = notify == c.g[V0][0];
        c.g[V0][0] = 8;
        if !is_nine {
            c.cycles += 2;
            if notify != c.g[V0][0] {
                c.cycles += 2;
                c.g[V0][0] = 0x101;
                abi::ret(c);
                return;
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `void OVScreenMap_HandleInput(this)` (0x00208638):
/// `EventSelect_HandleInput(this + 0x9c)`.
pub fn ov_screen_map_handle_input(c: &mut Cpu) {
    forward(
        c,
        7,
        0x10,
        |c| c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x9c)),
        callee::EVENT_SELECT_HANDLE_INPUT,
        0x0020_8648,
    )
}

/// `int OVScreenMap_OnWidgetNotify(self, a1, notify)` (0x208610): the same
/// notify hook on the screen map's event-select member at `self+0x9c`.
pub fn ov_screen_map_on_widget_notify(c: &mut Cpu) {
    event_select_notify(c, 0x9c, 0x0020_8620)
}

// ---------------------------------------------------------------------------
// Overlay screens.

/// `OVScreenMap_OnActivate(this)` 0x208558: activate the screen's widget list
/// and switch the event select into map mode.
pub fn ov_screen_map_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg64(c, 0);
        c.g[S0][0] = this;
        call_ptr(c, table::UI_STATE_ON_ACTIVATE, 0x0020_856c)?;
        c.g[A0][0] = sx((this as u32).wrapping_add(0x9c));
        callee::EVENT_SELECT_ON_ACTIVATE.call(c, 0x0020_8574)?;
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenMap_Update(this)` 0x208678: update the overlay's own state, then
/// the event-select widget embedded at +0x9c.
pub fn ov_screen_map_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg64(c, 0);
        c.g[S0][0] = this;
        callee::C_OV_STATE_OVERLAY_UPDATE.call(c, 0x0020_868c)?;
        c.g[A0][0] = sx((this as u32).wrapping_add(0x9c));
        callee::EVENT_SELECT_UPDATE.call(c, 0x0020_8694)?;
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void EventSelect_OnActivate(self)` (0x200ac0): put the map widget into
/// its current mode, then let the audio system react to entering event
/// select.
pub fn event_select_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.lw(abi::r32(c, GP).wrapping_sub(0xb54));
        frame.save_ra(c, 0);
        call_ptr(c, callee::EVENT_SELECT_SET_MODE, 0x0020_0ad4)?;
        call_ptr(c, callee::AUDIO_GET_INSTANCE, 0x0020_0adc)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, callee::AUDIO_ON_EVENT_SELECT_ENTER, 0x0020_0ae4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenMap_Shutdown(self)` (0x208588): shut the map's event-select
/// widget (`self+0x9c`) down, then the overlay state.
pub fn ov_screen_map_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(p(r32(c, S0), 0x9c));
        call_ptr(c, callee::EVENT_SELECT_SHUTDOWN, 0x0020_85a0)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::OV_STATE_OVERLAY_SHUTDOWN, 0x0020_85a8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void EventSelect_HideMapPic(EventSelect* s)` (0x201868): hide the map
/// picture (`+0x2dc = 0`), flag `+0x2f0`, and call the picture object's
/// "hide" slot (+0x48) when there is one.
pub fn event_select_hide_map_pic(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        let this = crate::abi::arg(c, 0);
        c.g[V0][0] = 1;
        frame.save_ra(c, 0);
        Addr(p(this, 0x2f0)).sw(c, c.g[V0][0]);
        c.g[A2][0] = lw(c, this, 0x2e4);
        let missing = c.g[A2][0] == 0;
        Addr(p(this, 0x2dc)).sw(c, 0);
        if !missing {
            c.cycles += 6;
            let obj = c.g[A2][0];
            let vt = lw(c, obj as u32, 8);
            c.g[V0][0] = vt;
            c.g[A1][0] = 0;
            let m = VSlot::read(c, vt, 0x48);
            c.g[A0][0] = m.delta;
            c.g[V1][0] = m.func;
            c.g[A0][0] = m.this(obj);
            crate::call::call_ptr(c, m.func as u32, 0x0020_189c)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenMap_OnWidgetAction(OVScreenMap* s, ...)` (0x2085b8): forward
/// to the event-select widget and, when it accepted, show the map legend.
pub fn ov_screen_map_on_widget_action(c: &mut Cpu) {
    c.cycles += 9;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9c));
    if callee::EVENT_SELECT_ON_WIDGET_ACTION
        .call(c, 0x0020_85d0)
        .is_err()
    {
        return;
    }
    c.g[V1][0] = 1;
    if c.g[V0][0] == c.g[V1][0] {
        c.cycles += 10;
        c.g[A0][0] = Addr(p_2(c, S0, 0x10)).lw(c);
        c.g[A1][0] = 1;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        if call_ptr(c, 0x0039_f190, 0x0020_85ec).is_err() {
            return;
        }
        c.g[V1][0] = Addr(p_2(c, S0, 0x1c)).lw(c);
        c.g[V0][0] = 0xffff_ffff_ffff_c0ff;
        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
        c.g[V1][0] = c.g[V1][0] | 0x780;
        c.sw(p_2(c, S0, 0x1c), c.g[V1][0]);
    }
    frame.restore(c, S0, 0x10);
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void EventSelect_Shutdown(EventSelect* self)` (0x200af0): close the
/// event-select screen and its PDA template.
pub fn event_select_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        let self_ = c.g[A0][0];
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save_ra(c, 0);
        c.g[A1][0] = Addr((self_ as u32).wrapping_add(0x2d8)).lw(c);
        c.g[A0][0] = Addr(r32_2(c, V0)).off(0x11c).lw(c);
        call_ptr(c, 0x001a_38c0, 0x0020_0b08)?;
        c.g[V1][0] = Addr(gp(c, -0xb5c)).lw(c);
        c.g[V0][0] = 0x1;
        let first = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x3;
        let mut close = first;
        if !first {
            c.cycles += 2;
            let second = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = 0x4;
            close = second;
            if !second {
                c.cycles += 2;
                close = c.g[V1][0] == c.g[V0][0];
            }
        }
        if close {
            c.cycles += 2;
            call_ptr(c, 0x0020_d190, 0x0020_0b30)?;
        }
        c.cycles += 8;
        call_ptr(c, 0x0028_b180, 0x0020_0b38)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0028_f678, 0x0020_0b44)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void EventSelect_SetHighlightColor(EventSelect* self, void* obj, void* c)`
/// (0x2083d8): tell the row object to use the highlighted colour.
pub fn event_select_set_highlight_color(c: &mut Cpu) {
    event_select_set_color(c, 0x74c8, 0x74d8, 0x0020_8408, 0x0020_842c)
}

/// `void EventSelect_SetUnlockedColor(EventSelect* self, void* obj, void* c)`
/// (0x208438): tell the row object to use the unlocked colour.
pub fn event_select_set_unlocked_color(c: &mut Cpu) {
    event_select_set_color(c, 0x74a8, 0x74b8, 0x0020_8468, 0x0020_848c)
}

/// The shared body of the two `EventSelect_Set*Color` functions: call the
/// row's virtual colour setter with the colour object (or with no argument
/// when it is null).
pub(crate) fn event_select_set_color(c: &mut Cpu, with: u32, without: u32, ret1: u32, ret2: u32) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let obj = c.g[A1][0];
        c.g[A3][0] = obj;
        let none = c.g[A3][0] == 0;
        frame.save_ra(c, 0);
        if !none {
            'set: {
                c.cycles += 2;
                let no_colour = c.g[A2][0] == 0;
                c.g[A1][0] = 0x4d_0000;
                if !no_colour {
                    c.cycles += 8;
                    c.g[V1][0] = Addr(at(c, A3, 0x8)).lw(c);
                    c.g[A1][0] = sx(0x004d_0000u32.wrapping_sub(with));
                    c.g[A0][0] = Addr(r32_2(c, V1)).off(0x58).lh(c);
                    c.g[V0][0] = Addr(r32_2(c, V1)).off(0x5c).lw(c);
                    let target = r32_2(c, V0);
                    c.g[A0][0] = abi::addu(c.g[A3][0], c.g[A0][0]);
                    call_ptr(c, target, ret1)?;
                    frame.restore_ra(c, 0);
                    c.cycles += 2;
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.pc = t;
                    return Ok(());
                }
            }
            c.cycles += 7;
            c.g[V1][0] = Addr(at(c, A3, 0x8)).lw(c);
            c.g[A1][0] = 0x4d_0000;
            c.g[A1][0] = sx(0x004d_0000u32.wrapping_sub(without));
            c.g[A0][0] = Addr(r32_2(c, V1)).off(0x58).lh(c);
            c.g[V0][0] = Addr(r32_2(c, V1)).off(0x5c).lw(c);
            let target = r32_2(c, V0);
            c.g[A0][0] = abi::addu(c.g[A3][0], c.g[A0][0]);
            call_ptr(c, target, ret2)?;
        }
        c.cycles += 1;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}

/// `void OVScreenMap_Init(self)` (0x00208498): create the map screen and
/// initialise the overlay's event select.
pub fn ov_screen_map_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = sx(0x4a0000);
        c.g[A0][0] = sx(0x4a0000u32.wrapping_add(0x26e8));
        c.g[S0][0] = Addr(at_2(c, S1, 0x10)).lw(c);
        callee::GET_HASH_VALUE_32.call(c, 0x0020_84bc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME.call(c, 0x0020_84d0)?;
        let absent = c.g[V0][0] == 0;
        Addr(at_2(c, S1, 0x40)).sw(c, c.g[V0][0]);
        if !absent {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            callee::UI_SCREEN_PLAY_FRAME.call(c, 0x0020_84e8)?;
        }
        c.cycles += 11;
        c.g[A0][0] = c.g[S1][0];
        callee::C_OV_STATE_OVERLAY_INIT.call(c, 0x0020_84f0)?;
        c.g[A1][0] = Addr(at_2(c, S1, 0x40)).lw(c);
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = sx(word(c, S1).wrapping_add(0x9c));
        callee::EVENT_SELECT_INIT.call(c, 0x0020_8500)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* EventSelect_GetMedalText(int peak)` (0x204ff0): the medal label for
/// a peak, hashed into the world's UI text table.
pub fn event_select_get_medal_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let is1 = c.g[A1][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        let mut world = 0u64;
        let mut text = 0u64;
        if is1 {
            c.cycles += 4;
            world = Addr(gp(c, -0x848)).lw(c);
            text = sx(0x0047_0e20);
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < 2) as u64;
            let lt2 = c.g[V0][0] != 0;
            world = Addr(gp(c, -0x848)).lw(c);
            if lt2 {
                c.cycles += 3;
                text = sx(0x0047_0e10);
            } else {
                c.cycles += 3;
                c.g[V0][0] = 2;
                let is2 = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = 3;
                if is2 {
                    c.cycles += 4;
                    world = Addr(gp(c, -0x848)).lw(c);
                    text = sx(0x0047_0e30);
                } else {
                    c.cycles += 2;
                    let is3 = c.g[A1][0] == c.g[V0][0];
                    world = Addr(gp(c, -0x848)).lw(c);
                    if is3 {
                        c.cycles += 2;
                        text = sx(0x0047_0e40);
                    } else {
                        c.cycles += 3;
                        text = sx(0x0047_0e10);
                    }
                }
            }
        }
        c.g[V0][0] = world;
        c.g[A0][0] = text;
        c.cycles += 14;
        c.g[S0][0] = Addr(p(r32(c, V0), 0x8c)).lw(c);
        c.g[S1][0] = Addr(p(r32(c, S0), 4)).lw(c);
        c.g[V0][0] = Addr(p(r32(c, S1), 0x20)).lh(c);
        c.g[S0][0] = sx(p(r32(c, S0), r32(c, V0)));
        call_ptr(c, 0x0031_7670, 0x00205070)?;
        c.g[V1][0] = Addr(p(r32(c, S1), 0x24)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        let get = r32(c, V1);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, get, 0x00205080)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// EventSelect_IsEventUnlocked (0x0000200d00).

/// `EventSelect_IsEventUnlocked`: report whether the event-select entry is unlocked.
pub fn event_select_is_event_unlocked(c: &mut Cpu) {
    guest(c, |c| {
        'b_200dd0: {
            'b_200dcc: {
                c.cycles += 16;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S0][0] = c.g[A1][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[S2][0] = c.g[A2][0];
                call_ptr(c, 0x0014dc80, 0x00200d20)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0014dd58, 0x00200d2c)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xb5c));
                c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[V0][0] = 0u64;
                if t {
                    break 'b_200dcc;
                } else {
                    'b_200d74: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x4;
                        let t = c.g[S0][0] != c.g[V0][0];
                        c.g[V0][0] = 0x5;
                        if t {
                            break 'b_200d74;
                        } else {
                            c.cycles += 10;
                            c.g[A0][0] = 0u64;
                            call_ptr(c, 0x0014a080, 0x00200d54)?;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A2][0] = c.g[V0][0];
                            c.g[A3][0] = c.g[S2][0];
                            c.g[A1][0] = 0u64;
                            c.g[T0][0] = 0x1;
                            call_ptr(c, 0x001464d0, 0x00200d6c)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_200dd0;
                            } else {
                                break 'b_200d74;
                            }
                        }
                    }
                    'b_200da4: {
                        c.cycles += 2;
                        let t = c.g[S0][0] != c.g[V0][0];
                        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x6));
                        if t {
                            break 'b_200da4;
                        } else {
                            c.cycles += 10;
                            c.g[A0][0] = 0u64;
                            call_ptr(c, 0x0014a080, 0x00200d84)?;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A2][0] = c.g[V0][0];
                            c.g[A3][0] = c.g[S2][0];
                            c.g[A1][0] = 0u64;
                            c.g[T0][0] = 0u64;
                            call_ptr(c, 0x001464d0, 0x00200d9c)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_200dd0;
                            } else {
                                break 'b_200da4;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V0][0] < 0x6u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0u64;
                    if t {
                        break 'b_200dcc;
                    } else {
                        c.cycles += 7;
                        c.g[A0][0] = 0u64;
                        call_ptr(c, 0x0014a080, 0x00200db8)?;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A2][0] = c.g[V0][0];
                        c.g[A3][0] = c.g[S0][0];
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x00146a70, 0x00200dcc)?;
                        break 'b_200dcc;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_200dd0;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::compare;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::seed_count;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL_RA;
    pub(crate) use crate::game::dirtysock::tags2::tests::LIST;
    pub(crate) use crate::game::fe::debugfe::tests::w;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::clear;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_10 as globals_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_12 as globals_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_12 as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_10 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_8 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::ui;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_3 as OBJ2_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_5 as WORLD;
    pub(crate) use crate::game::fe::fememcard::tests::vtable_at;
    pub(crate) use crate::game::fe::festateload::tests::notify_setup;
    pub(crate) use crate::game::fe::ovstatelodge::tests::slot;
    pub(crate) use crate::game::fe::ovstatelodge::tests::VT;
    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3 as base;
    pub(crate) use crate::game::movie::movieplayer::tests::globals;
    pub(crate) use crate::game::movie::movieplayer::tests::heap_init_2 as heap_init;
    pub(crate) use crate::game::movie::movieplayer::tests::pool_allocator;
    pub(crate) use crate::game::movie::movieplayer::tests::run_2 as run;
    pub(crate) use crate::game::movie::movieplayer::tests::w as w_2;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_2 as BE;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_LIBRARY;
    pub(crate) use crate::game::movie::movieplayer::tests::MEM_HEAP;
    pub(crate) use crate::game::movie::movieplayer::tests::W;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn event_select_on_widget_notify_matches() {
        check(
            ssx3_ee::s05::f_00202738,
            event_select_on_widget_notify,
            200,
            &|c, seed| {
                let mut r = Rng(0x0c10_0010 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A2][0] = match seed % 4 {
                    0 => 6,
                    1 => 9,
                    2 => 8,
                    _ => 0x100,
                };
            },
        );
    }

    #[test]
    fn ov_screen_map_handle_input_matches() {
        check(
            ee::s05::f_00208638__generated,
            ov_screen_map_handle_input,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x20_8638, 0);
                c.g[A0][0] = ptr(&mut r) as u64;
            },
        );
    }

    #[test]
    fn ov_screen_map_on_widget_notify_matches() {
        check(
            ssx3_ee::s05::f_00208610__generated,
            ov_screen_map_on_widget_notify,
            150,
            &|c, seed| notify_setup(c, seed, 0x18),
        );
    }

    #[test]
    fn ov_screen_map_on_activate_matches() {
        check(
            ssx3_ee::s05::f_00208558__generated,
            ov_screen_map_on_activate,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0xad, 3);
                globals(c);
                c.g[A0][0] = sx(DATA);
                // EventSelect mode 0 (the map tab).
                c.bus.write32(GAME_GP - 0xb54, 0);
            },
        );
    }

    #[test]
    fn ov_screen_map_update_matches() {
        check(
            ssx3_ee::s05::f_00208678__generated,
            ov_screen_map_update,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0xae, 3);
                globals(c);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(GAME_GP - 0xb54, 0);
            },
        );
    }

    #[test]
    fn event_select_on_activate_matches() {
        check(
            ee::s05::f_00200ac0__generated,
            event_select_on_activate,
            40,
            &|c, seed| {
                machine(c, seed, 0x200ac0, 4);
                globals_2(c);
                clear(c, DATA, 0x400);
                c.g[A0][0] = DATA as u64;
                // The map mode the widget is put into (`D_004A259C`).
                c.bus
                    .write32(GAME_GP.wrapping_sub(0xb54), (seed % 4) as u32);
            },
        );
    }

    #[test]
    fn ov_screen_map_shutdown_matches() {
        check(
            ee::s05::f_00208588__generated,
            ov_screen_map_shutdown,
            40,
            &|c, seed| {
                machine(c, seed, 0x208588, 6);
                globals_2(c);
                clear(c, DATA, 0x400);
                c.g[A0][0] = DATA as u64;
                let _ = seed;
            },
        );
    }

    #[test]
    fn event_select_hide_map_pic_matches() {
        check(
            ssx3_ee::s05::f_00201868__generated,
            event_select_hide_map_pic,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x30, 1);
                c.g[A0][0] = sx(DATA);
                if seed % 2 == 0 {
                    c.bus.write32(DATA + 0x2e4, 0);
                } else {
                    c.bus.write32(DATA + 0x2e4, DATA + 0x3000);
                    vtable_at(c, &mut r, DATA + 0x3000, 0x48);
                }
            },
        );
    }

    #[test]
    fn event_select_set_highlight_color_matches() {
        check(
            ssx3_ee::s05::f_002083d8__generated,
            event_select_set_highlight_color,
            80,
            &|c, seed| {
                fakes::machine(c, seed, 0x2083d8, 0);
                w(c, OBJ + 0x8, VT);
                slot(c, VT, 0x58, 0);
                c.g[A0][0] = OBJ2 as u64;
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { OBJ as u64 };
                c.g[A2][0] = if seed % 4 == 0 { 0 } else { OBJ3 as u64 };
            },
        );
    }

    #[test]
    fn event_select_set_unlocked_color_matches() {
        check(
            ssx3_ee::s05::f_00208438__generated,
            event_select_set_unlocked_color,
            80,
            &|c, seed| {
                fakes::machine(c, seed, 0x208438, 0);
                w(c, OBJ + 0x8, VT);
                slot(c, VT, 0x58, 0);
                c.g[A0][0] = OBJ2 as u64;
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { OBJ as u64 };
                c.g[A2][0] = if seed % 4 == 0 { 0 } else { OBJ3 as u64 };
            },
        );
    }

    #[test]
    fn event_select_shutdown_matches() {
        check(
            ssx3_ee::s05::f_00200af0__generated,
            event_select_shutdown,
            40,
            &|c, seed| {
                fakes::machine(c, seed, 0x200af0, 0);
                w(c, GAME_GP - 0x848, 0);
                w(
                    c,
                    GAME_GP - 0xb5c,
                    match seed % 4 {
                        0 => 1,
                        1 => 3,
                        2 => 4,
                        _ => seed,
                    },
                );
                w(c, OBJ + 0x2d8, OBJ2);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    // ---------------------------------------------------------------- overlay

    #[test]
    fn ov_screen_map_init_matches() {
        check(
            ee::s05::f_00208498__generated,
            ov_screen_map_init,
            8,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x2084_98, 4);
                let engine = DATA + 0x1000;
                w_2(c, DATA + 0x10, engine);
                w_2(c, engine, DATA + 0x1100);
                w_2(c, engine + 4, 0);
                w_2(c, DATA + 0x1100, 0);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn event_select_get_medal_text_matches() {
        check(
            ee::s05::f_00204ff0__generated,
            event_select_get_medal_text,
            10,
            &|c, seed| {
                let r = base_2(c, seed, 0x204ff0, 2);
                c.g[A1][0] = (seed % 6) as u64;
                w_3(c, WORLD + 0x8c, OBJ2_2);
                w_3(c, OBJ2_2 + 4, LIST);
                w_3(c, LIST + 0x20, 0);
                w_3(c, LIST + 0x24, 0);
                let _ = r;
            },
        );
    }

    #[test]
    fn event_select_is_event_unlocked_matches() {
        diff(
            ee::s05::f_00200d00__generated,
            event_select_is_event_unlocked,
            20,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x20_0d00);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = (seed % 5) as u64;
                w_4(
                    c,
                    GAME_GP.wrapping_sub(0xb5c),
                    if seed % 2 == 0 { 0 } else { DATA + 0x3000 },
                );
                w_4(c, DATA + 0x2000, r.next_u32());
                w_4(c, DATA + 0x3000, r.next_u32());
            },
        );
    }
}
