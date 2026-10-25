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
        0x0020_18a8,
        event_select_on_widget_action,
        "EventSelect_OnWidgetAction",
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

// ---------------------------------------------------------------------------
// EventSelect_OnWidgetAction (0x2018a8).

/// Callees of `EventSelect_OnWidgetAction`.
mod action {
    use super::ee;
    use super::Callee;
    pub(super) const BE_LIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;
    pub(super) const BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;
    pub(super) const BE_LIBRARY_SET_PHASE: u32 = 0x0014_de68;
    pub(super) const BE_NEW_RACE_INT_GET_TRACK_INFO: u32 = 0x0014_4bc0;
    pub(super) const BE_NEW_RACE_INT_SET_EVENT_TYPE: u32 = 0x0014_4df0;
    pub(super) const BE_NEW_RACE_INT_SET_EVENT: u32 = 0x0014_51e8;
    pub(super) const BE_NEW_PLAYER_INT_IS_CHAR_PROFILE_FLAG_CLEAR: u32 = 0x0014_5d38;
    pub(super) const GAME_MODULE_QUIT_TO_LOAD_SCREEN: u32 = 0x0023_0488;
    pub(super) const GAME_MODULE_REQUEST_STATE: u32 = 0x0023_1250;
    pub(super) const GAME_MODE_MAN_START_MODE: u32 = 0x0023_8160;
    pub(super) const AUDIO_GET_INSTANCE: u32 = 0x0028_b180;
    pub(super) const AUDIO_STOP_FE_MUSIC: u32 = 0x0028_f520;
    pub(super) const AUDIO_SET_SELECTED_PEAK: u32 = 0x0028_f558;
    pub(super) const AUDIO_ON_EVENT_SELECT_EXIT: u32 = 0x0028_f678;
    pub(super) const STR_HASH: u32 = 0x0031_7670;
    pub(super) const UI_SCREEN_FIND_OBJECT: u32 = 0x0039_d860;
    pub(super) const UI_STATE_HIDE_OBJECT: u32 = 0x0039_e8b8;
    pub(super) const UI_STATE_SHOW_OBJECT: u32 = 0x0039_e948;
    pub(super) const EA_MEM_SET: u32 = 0x003e_6448;
    pub(super) const SPRINTF: u32 = 0x0041_7828;
    pub(super) const GET_EVENT_RECORD: Callee = Callee {
        addr: 0x0020_0388,
        f: ee::s05::f_00200388,
    };
    pub(super) const HIDE_MAP_PIC: Callee = Callee {
        addr: 0x0020_1868,
        f: ee::s05::f_00201868,
    };
    pub(super) const UPDATE_MODE_PANELS: Callee = Callee {
        addr: 0x0020_2ad8,
        f: ee::s05::f_00202ad8,
    };
    pub(super) const SET_MODE: Callee = Callee {
        addr: 0x0020_30a0,
        f: ee::s05::f_002030a0,
    };
    pub(super) const LOAD_MAP_IMAGE: Callee = Callee {
        addr: 0x0020_3990,
        f: ee::s05::f_00203990,
    };
    pub(super) const UPDATE_CONFIRM_POPUP: Callee = Callee {
        addr: 0x0020_3a58,
        f: ee::s05::f_00203a58,
    };
    pub(super) const SET_MAP_TAB_MODE: Callee = Callee {
        addr: 0x0020_3cb8,
        f: ee::s05::f_00203cb8,
    };
    pub(super) const REFRESH_PEAK_LIST: Callee = Callee {
        addr: 0x0020_3d40,
        f: ee::s05::f_00203d40,
    };
    pub(super) const SET_HIGHLIGHT_COLOR: Callee = Callee {
        addr: 0x0020_83d8,
        f: ee::s05::f_002083d8,
    };
    pub(super) const SET_UNLOCKED_COLOR: Callee = Callee {
        addr: 0x0020_8438,
        f: ee::s05::f_00208438,
    };
}

/// `gp`-relative globals of the event-select screen.
mod es {
    /// The panel shown: 0 peaks, 1 a peak's tabs, 2 a tab's events.
    pub(super) const MODE: i32 = -0xb54;
    /// Selected tab (`gp-0xb50`) and peak (`gp-0xb4c`).
    pub(super) const TAB: i32 = -0xb50;
    pub(super) const PEAK: i32 = -0xb4c;
    /// Which flow event select runs in (0/1 front end, 2+ challenge
    /// variants, 3/4 from the in-game overlay).
    pub(super) const FLOW: i32 = -0xb5c;
    /// The world singleton.
    pub(super) const WORLD: i32 = -0x848;
    /// The selected peak handed to the race (`gp-0x1f34`).
    pub(super) const RACE_PEAK: i32 = -0x1f34;
    /// Set when an event is started from the event list (`gp-0x1f38`).
    pub(super) const EVENT_STARTED: i32 = -0x1f38;
    /// The game state to load next (`gp-0x9f4`).
    pub(super) const NEXT_STATE: i32 = -0x9f4;
    /// The loading-screen history: a depth (`gp-0x69c`) into
    /// `D_005366E8`, and the screen picked from `D_004428F0` (`gp-0x6a0`).
    pub(super) const LOAD_DEPTH: i32 = -0x69c;
    pub(super) const LOAD_SCREEN: i32 = -0x6a0;
}

/// The new-race settings block (`BENewRaceInt` state).
const NEW_RACE_INFO: u32 = 0x0053_5bc8;
/// `D_005366E8`, `D_004428F0`: the loading-screen stacks.
const LOAD_SCREEN_STACK: u32 = 0x0053_66e8;
const LOAD_SCREEN_TABLE: u32 = 0x0044_28f0;
/// `D_004C8B38`: the empty colour argument of a row's colour setter.
const NO_COLOUR: u32 = 0x004c_8b38;

/// Where `EventSelect_OnWidgetAction` leaves for its epilogue (the
/// original's three exit labels).
#[derive(Clone, Copy)]
enum Exit {
    /// `s0` already restored (0x202714): +8.
    Restored,
    /// Return v0 (0x202710): +1, restore s0, +8.
    Value,
    /// Return 0 (0x20270c): +1, v0 = 0, then as `Value`.
    Zero,
}

type Next = Result<Exit, crate::call::Unwound>;

/// `gp + off`.
#[inline(always)]
fn es_global(c: &Cpu, off: i32) -> Addr {
    Addr((c.g[GP][0] as u32).wrapping_add(off as u32))
}

/// `Addr(reg + off)`.
#[inline(always)]
fn reg_at(c: &Cpu, reg: usize, off: u32) -> Addr {
    Addr((c.g[reg][0] as u32).wrapping_add(off))
}

/// gcc's inline block move of `iters` 32-byte chunks from register `src` to
/// register `dst` (`ld`/`sd` when both are 8-aligned, else `ldl`/`ldr` and
/// `sdl`/`sdr`), the doublewords passing through `regs`, `cycles` per
/// iteration and the back-edge event check at `head`.
#[allow(clippy::too_many_arguments)]
fn es_block_move(
    c: &mut Cpu,
    src: usize,
    dst: usize,
    regs: [usize; 4],
    cycles: u64,
    head: u32,
    aligned: bool,
    iters: u32,
) {
    for i in 0..iters {
        c.cycles += cycles;
        let s = c.g[src][0] as u32;
        let d = c.g[dst][0] as u32;
        let mut v = [0u64; 4];
        for (k, w) in v.iter_mut().enumerate() {
            let a = Addr(s.wrapping_add(8 * k as u32));
            *w = if aligned {
                a.ld(c)
            } else {
                a.read_u64_unaligned(c)
            };
        }
        for (k, &w) in v.iter().enumerate() {
            let a = Addr(d.wrapping_add(8 * k as u32));
            if aligned {
                a.sd(c, w)
            } else {
                a.write_u64_unaligned(c, w)
            }
            c.g[regs[k]][0] = w;
        }
        c.g[src][0] = sx(s.wrapping_add(0x20));
        c.g[dst][0] = sx(d.wrapping_add(0x20));
        if i + 1 < iters && event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// The 12-byte tail of a 0x6c-byte record copy: `ldl/ldr` a doubleword
/// into `dw`, `lw` the last word into `word`, store both.
fn es_move_tail(c: &mut Cpu, src: usize, dst: usize, dw: usize, word: usize) {
    let s = c.g[src][0] as u32;
    let d = c.g[dst][0] as u32;
    c.g[dw][0] = Addr(s).read_u64_unaligned(c);
    c.g[word][0] = Addr(s.wrapping_add(8)).lw(c);
    Addr(d).write_u64_unaligned(c, c.g[dw][0]);
    Addr(d.wrapping_add(8)).sw(c, c.g[word][0]);
}

/// Play front-end sound `n` through the sound player in `s3`
/// (`player->vtbl[0x10](n)`; `swapped`: the original holds the vtable in v1
/// and the method in v0).
fn es_sound(c: &mut Cpu, n: u64, swapped: bool, ret: u32) -> Flow {
    let player = c.g[S3][0] as u32;
    let vt = Addr(player).lw(c);
    let m = VSlot::read(c, vt, 0x10);
    let (vt_reg, fn_reg) = if swapped { (V1, V0) } else { (V0, V1) };
    c.g[vt_reg][0] = vt;
    c.g[A1][0] = n;
    c.g[A0][0] = m.delta;
    c.g[fn_reg][0] = m.func;
    c.g[A0][0] = sx(player.wrapping_add(m.delta as u32));
    call_ptr(c, m.func as u32, ret)
}

/// Commit the new-race interface in `s0` (`iface->vtbl[+0xc][8]()`), the
/// method passing through `fn_reg`.
fn es_commit_race(c: &mut Cpu, fn_reg: usize, ret: u32) -> Flow {
    let iface = c.g[S0][0] as u32;
    c.g[V1][0] = Addr(iface.wrapping_add(0xc)).lw(c);
    let m = VSlot::read(c, c.g[V1][0], 8);
    c.g[A0][0] = m.delta;
    c.g[fn_reg][0] = m.func;
    c.g[A0][0] = sx(iface.wrapping_add(m.delta as u32));
    call_ptr(c, m.func as u32, ret)
}

/// `void EventSelect_OnWidgetAction(EventSelect* self, UIWidget* w, int action)`
/// (0x2018a8): the event-select screen's widget actions (`w->id` at +0x18 is
/// the row): 1/2 colour the row as highlighted/unlocked, 5 accepts (pick a
/// peak, a tab or an event, or confirm a challenge), 6 goes back, 7
/// toggles the map tab. Returns 1 when the screen is left, else 0.
pub fn event_select_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0xe0);
        frame.save(c, S0, 0xd0);
        frame.save(c, S1, 0xc0);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0xb0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0xa0);
        frame.save(c, S4, 0x90);
        c.g[S3][0] = sx(es_global(c, 0x2968).0);
        frame.save(c, S5, 0x80);
        frame.save_ra(c, 0x70);
        c.g[S2][0] = c.g[A0][0];
        let this = c.g[S2][0] as u32;
        let w = c.g[S1][0] as u32;
        // The new-race (0) and new-player (1) interfaces.
        call_ptr(c, action::BE_LIBRARY_GET_INSTANCE, 0x0020_18dc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, action::BE_LIBRARY_GET_INTERFACE, 0x0020_18e8)?;
        c.g[S4][0] = c.g[V0][0];
        call_ptr(c, action::BE_LIBRARY_GET_INSTANCE, 0x0020_18f0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, action::BE_LIBRARY_GET_INTERFACE, 0x0020_18fc)?;
        c.g[S5][0] = c.g[V0][0];
        // The screen state's sound player, else the default one.
        c.g[V0][0] = Addr(this.wrapping_add(0x2ec)).lw(c);
        c.g[A0][0] = reg_at(c, V0, 0x10).lw(c);
        c.g[V1][0] = reg_at(c, A0, 0x14).lw(c);
        if c.g[V1][0] != 0 {
            c.g[S3][0] = c.g[V1][0];
        }
        let exit = if c.g[S1][0] == 0 {
            Exit::Zero
        } else {
            es_dispatch(c, this, w)?
        };
        match exit {
            Exit::Zero => {
                c.cycles += 1;
                c.g[V0][0] = 0;
                c.cycles += 9;
            }
            Exit::Value => c.cycles += 9,
            Exit::Restored => c.cycles += 8,
        }
        frame.restore(c, S0, 0xd0);
        frame.restore(c, S1, 0xc0);
        frame.restore(c, S2, 0xb0);
        frame.restore(c, S3, 0xa0);
        frame.restore(c, S4, 0x90);
        frame.restore(c, S5, 0x80);
        frame.restore_ra(c, 0x70);
        frame.close(c);
        Ok(())
    })
}

/// The action switch (jump table `D_00470780`, actions 1..7).
fn es_dispatch(c: &mut Cpu, this: u32, w: u32) -> Next {
    c.cycles += 4;
    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(1));
    let in_range = c.g[V1][0] < 7;
    c.g[V0][0] = 0x0047_0000;
    if !in_range {
        return Ok(Exit::Zero);
    }
    c.cycles += 6;
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V0][0] = sx(0x0047_0780);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x0047_0780));
    c.g[A0][0] = reg_at(c, V1, 0).lw(c);
    match c.g[A0][0] as u32 {
        0x0020_23d4 => es_highlight(c, this, w),
        0x0020_261c => es_unlocked(c, this, w),
        0x0020_193c => es_accept(c, this, w),
        0x0020_21d8 => es_back(c, this),
        0x0020_22bc => es_toggle_tab(c, this),
        0x0020_270c => Ok(Exit::Zero),
        0x0020_2020 => {
            es_store_load_screen(c);
            es_start_special(c, this)
        }
        target => {
            c.jump_out(target, 0x0020_18a8, 0x0020_2734);
            Err(crate::call::Unwound)
        }
    }
}

// ----------------------------------------------------------- shared tails

/// 0x2021b8 / 0x201d60: "denied" sound (4), return 0.
fn es_deny(c: &mut Cpu, ret: u32) -> Next {
    c.cycles += 8;
    es_sound(c, 4, false, ret)?;
    c.g[V0][0] = 0;
    Ok(Exit::Value)
}

/// 0x2021e4: drop the confirm popup, then refresh the mode (a1 = mode).
fn es_clear_confirm(c: &mut Cpu, this: u32) -> Next {
    c.cycles += 2;
    c.g[A0][0] = c.g[S2][0];
    Addr(this.wrapping_add(0x2cc)).sw(c, 0);
    es_refresh_mode(c)
}

/// 0x2021ec: `SetMode(self, a1)`, refresh the panels and the confirm popup.
fn es_refresh_mode(c: &mut Cpu) -> Next {
    c.cycles += 8;
    action::SET_MODE.call(c, 0x0020_21f4)?;
    c.g[A0][0] = c.g[S2][0];
    action::UPDATE_MODE_PANELS.call(c, 0x0020_21fc)?;
    c.g[A0][0] = c.g[S2][0];
    action::UPDATE_CONFIRM_POPUP.call(c, 0x0020_2204)?;
    c.g[V0][0] = 0;
    Ok(Exit::Value)
}

/// 0x2022ac: hide the map picture and report the screen left (1).
fn es_leave(c: &mut Cpu) -> Next {
    c.cycles += 4;
    c.g[A0][0] = c.g[S2][0];
    action::HIDE_MAP_PIC.call(c, 0x0020_22b4)?;
    c.g[V0][0] = 1;
    Ok(Exit::Value)
}

/// 0x2023bc: refresh the panels and load the map image, return 0.
fn es_show_mode(c: &mut Cpu) -> Next {
    c.cycles += 6;
    c.g[A0][0] = c.g[S2][0];
    action::UPDATE_MODE_PANELS.call(c, 0x0020_23c4)?;
    c.g[A0][0] = c.g[S2][0];
    action::LOAD_MAP_IMAGE.call(c, 0x0020_23cc)?;
    c.g[V0][0] = 0;
    Ok(Exit::Value)
}

/// 0x202060: stop the front-end music and tell the audio system the peak
/// (`self+0x260`), return 0.
fn es_audio_peak(c: &mut Cpu, this: u32) -> Next {
    c.cycles += 11;
    call_ptr(c, action::AUDIO_GET_INSTANCE, 0x0020_2068)?;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, action::AUDIO_STOP_FE_MUSIC, 0x0020_2070)?;
    call_ptr(c, action::AUDIO_GET_INSTANCE, 0x0020_2078)?;
    c.g[A1][0] = Addr(this.wrapping_add(0x260)).lw(c);
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, action::AUDIO_SET_SELECTED_PEAK, 0x0020_2084)?;
    c.g[V0][0] = 0;
    Ok(Exit::Value)
}

/// 0x202020: pop the loading-screen history (registers from the block
/// before: a2 = new depth, v1 = stack entry * 4, a0 = table, a3 = state,
/// t1 = request, t0 = 1).
fn es_store_load_screen(c: &mut Cpu) {
    c.cycles += 6;
    es_global(c, es::NEXT_STATE).sw(c, c.g[A3][0]);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    es_global(c, es::LOAD_DEPTH).sw(c, c.g[A2][0]);
    c.g[V0][0] = reg_at(c, V1, 0).lw(c);
    es_global(c, es::LOAD_SCREEN).sw(c, c.g[V0][0]);
    reg_at(c, T1, 8).sw(c, c.g[T0][0]);
}

/// 0x202038: event types 5/6 start their game mode; hide the map picture
/// and hand the peak to the audio system.
fn es_start_special(c: &mut Cpu, this: u32) -> Next {
    c.cycles += 5;
    c.g[V0][0] = Addr(this.wrapping_add(0x264)).lw(c);
    let special = sx((c.g[V0][0] as u32).wrapping_sub(5)) < 2;
    c.g[V0][0] = es_global(c, es::WORLD).lw(c);
    if special {
        c.cycles += 3;
        c.g[A1][0] = Addr(this.wrapping_add(0x268)).lw(c);
        c.g[A0][0] = reg_at(c, V0, 0xc0).lw(c);
        call_ptr(c, action::GAME_MODE_MAN_START_MODE, 0x0020_2058)?;
    }
    c.cycles += 2;
    c.g[A0][0] = c.g[S2][0];
    action::HIDE_MAP_PIC.call(c, 0x0020_2060)?;
    es_audio_peak(c, this)
}

/// A row's colour setter with no colour (`w->vtbl(v1)[0x58]`, 0x2025fc /
/// 0x2026f4).
fn es_no_colour(c: &mut Cpu, w: u32, cycles: u64, ret: u32) -> Flow {
    c.cycles += cycles;
    c.g[A1][0] = sx(NO_COLOUR);
    let m = VSlot::read(c, c.g[V1][0], 0x58);
    c.g[A0][0] = m.delta;
    c.g[V0][0] = m.func;
    c.g[A0][0] = sx(w.wrapping_add(m.delta as u32));
    call_ptr(c, m.func as u32, ret)
}

/// `self + 4*w->id + off` (v0), loaded into a2: the row's flag.
fn es_row_flag(c: &mut Cpu, this: u32, w: u32, off: u32) {
    c.g[V0][0] = Addr(w.wrapping_add(0x18)).lw(c);
    es_row_flag_loaded(c, this, off);
}

/// As [`es_row_flag`] with `w->id` already in v0.
fn es_row_flag_loaded(c: &mut Cpu, this: u32, off: u32) {
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
    c.g[A2][0] = reg_at(c, V0, off).lw(c);
}

/// `self + peak*0x60 + tab*0x20 + 4*w->id` (v0) for the event rows: `scaled`
/// holds `gp-0xb4c` and becomes `peak*0x60` (multiplied by `by`, which
/// holds 0x60).
fn es_event_row(c: &mut Cpu, this: u32, w: u32, scaled: usize, by: usize) {
    c.g[V0][0] = Addr(w.wrapping_add(0x18)).lw(c);
    c.mult(0, scaled, scaled, by);
    c.g[V1][0] = es_global(c, es::TAB).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[scaled][0] as u32));
    c.g[V0][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
}

// ------------------------------------------------- 1, 2: row colours

/// Action 1: colour the row as highlighted (with the confirm popup up, its
/// yes/no buttons follow the row).
fn es_highlight(c: &mut Cpu, this: u32, w: u32) -> Next {
    c.cycles += 3;
    c.g[V0][0] = Addr(this.wrapping_add(0x2cc)).lw(c);
    c.g[A0][0] = c.g[S2][0];
    if c.g[V0][0] != 0 {
        c.cycles += 6;
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = 1;
        action::SET_HIGHLIGHT_COLOR.call(c, 0x0020_23ec)?;
        c.g[V0][0] = Addr(w.wrapping_add(0x18)).lw(c);
        c.g[A0][0] = Addr(this.wrapping_add(0x2ec)).lw(c);
        use self::action::{UI_STATE_HIDE_OBJECT as HIDE, UI_STATE_SHOW_OBJECT as SHOW};
        // ConfirmMenu_{Yes,No}_{x,a,na}: show "yes" on row 0, else "no".
        let steps: [(u32, u32, u32); 6] = if c.g[V0][0] == 0 {
            [
                (SHOW, 0x0047_06e8, 0x0020_2404),
                (HIDE, 0x0047_0700, 0x0020_2414),
                (SHOW, 0x0047_0718, 0x0020_2424),
                (HIDE, 0x0047_0730, 0x0020_2434),
                (SHOW, 0x0047_0748, 0x0020_2444),
                (HIDE, 0x0047_0760, 0x0020_2454),
            ]
        } else {
            [
                (SHOW, 0x0047_0700, 0x0020_2468),
                (HIDE, 0x0047_06e8, 0x0020_2478),
                (SHOW, 0x0047_0730, 0x0020_2488),
                (HIDE, 0x0047_0718, 0x0020_2498),
                (SHOW, 0x0047_0760, 0x0020_24a8),
                (HIDE, 0x0047_0748, 0x0020_24b8),
            ]
        };
        c.cycles += 25;
        for (i, (f, name, ret)) in steps.into_iter().enumerate() {
            if i > 0 {
                c.g[A0][0] = Addr(this.wrapping_add(0x2ec)).lw(c);
            }
            c.g[A1][0] = sx(name);
            call_ptr(c, f, ret)?;
        }
        c.g[V0][0] = 0;
        return Ok(Exit::Value);
    }
    c.cycles += 4;
    c.g[V1][0] = es_global(c, es::MODE).lw(c);
    let mode = c.g[V1][0];
    c.g[V0][0] = ((mode as i64) < 2) as u64;
    if mode == 1 {
        // A tab row: its flag at +0x10; the tab highlight ("pghl") grows.
        c.cycles += 10;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S2][0];
        es_row_flag(c, this, w, 0x10);
        action::SET_HIGHLIGHT_COLOR.call(c, 0x0020_2534)?;
        c.g[A0][0] = sx(0x004a_25f8);
        return es_grow_highlight(c, this, w);
    }
    c.cycles += 2;
    let below = c.g[V0][0] != 0;
    c.g[V0][0] = 2;
    if below {
        c.cycles += 2;
        if mode == 0 {
            // A peak row: its flag at +4.
            c.g[V0][0] = Addr(w.wrapping_add(0x18)).lw(c);
            c.cycles += 8;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S1][0];
            es_row_flag_loaded(c, this, 4);
            action::SET_HIGHLIGHT_COLOR.call(c, 0x0020_2510)?;
            c.g[V0][0] = 0;
            return Ok(Exit::Value);
        }
        c.cycles += 2;
        c.g[V1][0] = Addr(w.wrapping_add(8)).lw(c);
        es_no_colour(c, w, 8, 0x0020_2614)?;
        c.g[V0][0] = 0;
        return Ok(Exit::Value);
    }
    c.cycles += 2;
    c.g[A2][0] = es_global(c, es::PEAK).lw(c);
    if mode == 2 {
        // An event row; the event highlight ("ehl") grows.
        c.cycles += 15;
        c.g[A0][0] = 0x60;
        c.g[A1][0] = c.g[S1][0];
        es_event_row(c, this, w, A2, A0);
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = reg_at(c, V0, 0x20).lw(c);
        action::SET_HIGHLIGHT_COLOR.call(c, 0x0020_2574)?;
        c.g[A0][0] = sx(0x004a_2600);
        return es_grow_highlight(c, this, w);
    }
    c.cycles += 2;
    c.g[V1][0] = Addr(w.wrapping_add(8)).lw(c);
    es_no_colour(c, w, 8, 0x0020_2614)?;
    c.g[V0][0] = 0;
    Ok(Exit::Value)
}

/// 0x20257c: find the highlight object named by a0 in the screen and set
/// its y (+0x48) to the row's y + 100.
fn es_grow_highlight(c: &mut Cpu, this: u32, w: u32) -> Next {
    c.cycles += 7;
    call_ptr(c, action::STR_HASH, 0x0020_2584)?;
    c.g[A0][0] = Addr(this.wrapping_add(0x2e8)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, action::UI_SCREEN_FIND_OBJECT, 0x0020_2590)?;
    if c.g[V0][0] == 0 {
        c.g[V0][0] = 0;
        return Ok(Exit::Value);
    }
    c.cycles += 25;
    let obj = c.g[V0][0] as u32;
    let s = sp(c);
    // The row's and the highlight's position (x, y, z at +0x44) copied to
    // the stack, the highlight moved to the row's y + 100.
    c.g[A3][0] = Addr(w.wrapping_add(0x44)).read_u64_unaligned(c);
    c.g[T0][0] = Addr(w.wrapping_add(0x4c)).lw(c);
    Addr(s).write_u64_unaligned(c, c.g[A3][0]);
    Addr(s + 8).sw(c, c.g[T0][0]);
    c.g[A1][0] = Addr(obj.wrapping_add(0x44)).read_u64_unaligned(c);
    c.g[A2][0] = Addr(obj.wrapping_add(0x4c)).lw(c);
    Addr(s + 0x10).write_u64_unaligned(c, c.g[A1][0]);
    Addr(s + 0x18).sw(c, c.g[A2][0]);
    const HUNDRED: u32 = 0x42c8_0000;
    c.g[AT][0] = HUNDRED as u64;
    c.f[1] = HUNDRED;
    let y = Addr(s + 4).read_fpu(c) + FpuF32(HUNDRED);
    c.f[0] = y.0;
    Addr(s + 0x14).write_fpu(c, y);
    c.g[V1][0] = Addr(s + 0x10).read_u64_unaligned(c);
    c.g[A0][0] = Addr(s + 0x18).lw(c);
    Addr(obj.wrapping_add(0x44)).write_u64_unaligned(c, c.g[V1][0]);
    Addr(obj.wrapping_add(0x4c)).sw(c, c.g[A0][0]);
    c.g[V0][0] = 0;
    Ok(Exit::Value)
}

/// Action 2: colour the row as unlocked.
fn es_unlocked(c: &mut Cpu, this: u32, w: u32) -> Next {
    c.cycles += 3;
    c.g[V0][0] = Addr(this.wrapping_add(0x2cc)).lw(c);
    c.g[A0][0] = c.g[S2][0];
    if c.g[V0][0] != 0 {
        c.cycles += 5;
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = 1;
        action::SET_UNLOCKED_COLOR.call(c, 0x0020_2634)?;
        c.g[V0][0] = 0;
        return Ok(Exit::Value);
    }
    c.cycles += 4;
    c.g[V1][0] = es_global(c, es::MODE).lw(c);
    let mode = c.g[V1][0];
    c.g[V0][0] = ((mode as i64) < 2) as u64;
    if mode == 1 {
        c.cycles += 9;
        c.g[V0][0] = Addr(w.wrapping_add(0x18)).lw(c);
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S1][0];
        es_row_flag_loaded(c, this, 0x10);
        action::SET_UNLOCKED_COLOR.call(c, 0x0020_26b0)?;
        c.g[V0][0] = 0;
        return Ok(Exit::Value);
    }
    c.cycles += 2;
    let below = c.g[V0][0] != 0;
    c.g[V0][0] = 2;
    if below {
        c.cycles += 2;
        if mode == 0 {
            c.g[V0][0] = Addr(w.wrapping_add(0x18)).lw(c);
            c.cycles += 8;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S1][0];
            es_row_flag_loaded(c, this, 4);
            action::SET_UNLOCKED_COLOR.call(c, 0x0020_268c)?;
            c.g[V0][0] = 0;
            return Ok(Exit::Value);
        }
        c.cycles += 2;
        c.g[V1][0] = Addr(w.wrapping_add(8)).lw(c);
        es_no_colour(c, w, 6, 0x0020_270c)?;
        return Ok(Exit::Zero);
    }
    c.cycles += 2;
    c.g[A2][0] = es_global(c, es::PEAK).lw(c);
    if mode == 2 {
        c.cycles += 15;
        c.g[A1][0] = 0x60;
        c.g[A0][0] = c.g[S2][0];
        es_event_row(c, this, w, A2, A1);
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = reg_at(c, V0, 0x20).lw(c);
        action::SET_UNLOCKED_COLOR.call(c, 0x0020_26ec)?;
        c.g[V0][0] = 0;
        return Ok(Exit::Value);
    }
    c.cycles += 2;
    c.g[V1][0] = Addr(w.wrapping_add(8)).lw(c);
    es_no_colour(c, w, 6, 0x0020_270c)?;
    Ok(Exit::Zero)
}

// ------------------------------------------------------------ 5: accept

/// Action 5 (accept) on the current panel.
fn es_accept(c: &mut Cpu, this: u32, w: u32) -> Next {
    c.cycles += 4;
    c.g[V1][0] = es_global(c, es::MODE).lw(c);
    c.g[S4][0] = 1;
    let mode = c.g[V1][0];
    c.g[V0][0] = ((mode as i64) < 2) as u64;
    if mode == 1 {
        return es_accept_tab(c, this, w);
    }
    c.cycles += 2;
    let below = c.g[V0][0] != 0;
    c.g[V0][0] = 2;
    if below {
        c.cycles += 2;
        c.g[V0][0] = 0;
        if mode == 0 {
            return es_accept_peak(c, this, w);
        }
        c.cycles += 2;
        return Ok(Exit::Restored);
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    if mode == 2 {
        return es_accept_event(c, this, w);
    }
    c.cycles += 2;
    Ok(Exit::Restored)
}

/// Accept on the peak panel: with the confirm popup up, "yes" (row 0)
/// starts the peak's free ride; else pick the peak (rows 0..2, if
/// unlocked), or in the challenge flows (`gp-0xb5c` >= 2) take a
/// challenge record (peaks 1 and 2 need their character profile flag).
fn es_accept_peak(c: &mut Cpu, this: u32, w: u32) -> Next {
    c.cycles += 3;
    c.g[V0][0] = Addr(this.wrapping_add(0x2cc)).lw(c);
    if c.g[V0][0] != 0 {
        c.g[V0][0] = reg_at(c, S3, 0).lw(c);
        return es_accept_confirm(c, this, w);
    }
    c.cycles += 4;
    c.g[V1][0] = Addr(w.wrapping_add(0x18)).lw(c);
    let row = c.g[V1][0];
    let small = (row as i64) < 3;
    c.g[V0][0] = sx((row as u32) << 2);
    if !small {
        return es_deny(c, 0x0020_21d0);
    }
    c.cycles += 4;
    c.g[V0][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = reg_at(c, V0, 4).lw(c);
    c.g[A1][0] = 6;
    if c.g[V1][0] == 0 {
        return es_deny(c, 0x0020_21d0);
    }
    c.cycles += 9;
    es_sound(c, 6, false, 0x0020_19b4)?;
    c.g[V1][0] = Addr(w.wrapping_add(0x18)).lw(c);
    c.g[A0][0] = 0;
    c.g[A2][0] = 0xf;
    if c.g[V1][0] == c.g[S4][0] {
        c.cycles += 2;
        c.g[A0][0] = 1;
    } else {
        c.cycles += 3;
        c.g[V0][0] = 2;
        if c.g[V1][0] == 2 {
            c.cycles += 2;
            c.g[A0][0] = 1;
            c.g[A2][0] = 0x10;
        }
    }
    c.cycles += 2;
    let locked_peak = c.g[A0][0] != 0;
    c.g[A0][0] = c.g[S5][0];
    if locked_peak {
        // BENewPlayerInt_IsCharProfileFlagClear(iface1, 0, 0xf / 0x10).
        c.cycles += 4;
        c.g[A1][0] = 0;
        call_ptr(
            c,
            action::BE_NEW_PLAYER_INT_IS_CHAR_PROFILE_FLAG_CLEAR,
            0x0020_19f0,
        )?;
        let clear = c.g[V0][0] != 0;
        c.g[V0][0] = es_global(c, es::FLOW).lw(c);
        if clear {
            c.cycles += 3;
            c.g[V0][0] = (c.g[V0][0] < 2) as u64;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr(w.wrapping_add(0x18)).lw(c);
                return es_pick_peak(c);
            }
            return es_take_challenge(c, this, w);
        }
    }
    c.cycles += 1;
    c.g[V0][0] = Addr(w.wrapping_add(0x18)).lw(c);
    es_pick_peak(c)
}

/// 0x201bbc: the peak (v0) picked: show its tabs.
fn es_pick_peak(c: &mut Cpu) -> Next {
    c.cycles += 6;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = 1;
    es_global(c, es::PEAK).sw(c, c.g[V0][0]);
    action::SET_MODE.call(c, 0x0020_1bcc)?;
    es_show_mode(c)
}

/// The challenge flows: copy the challenge's 0x6c-byte event record (peak 1
/// from `D_00479248`, else `D_00478D38 + 0x360*peak`) to `self+0x260`, raise
/// the confirm popup and refresh the peak list.
fn es_take_challenge(c: &mut Cpu, this: u32, w: u32) -> Next {
    c.cycles += 4;
    c.g[A1][0] = Addr(w.wrapping_add(0x18)).lw(c);
    c.g[V0][0] = 1;
    c.g[A0][0] = 0x360;
    if c.g[A1][0] == 1 {
        c.cycles += 8;
        c.g[V1][0] = sx(this.wrapping_add(0x260));
        c.g[A1][0] = sx(0x0047_8ee8);
        c.g[A0][0] = sx(0x0047_9248);
        let aligned = (c.g[V1][0] | c.g[A0][0]) & 7 == 0;
        c.g[V0][0] = sx(0x0047_92a8);
        if aligned {
            es_block_move(c, A0, V1, [A1, A2, A3, T0], 11, 0x0020_1a88, true, 3);
        } else {
            es_block_move(c, A0, V1, [A1, A2, A3, T0], 19, 0x0020_1a34, false, 3);
            c.cycles += 2;
        }
        c.cycles += 7;
        es_move_tail(c, A0, V1, V0, A1);
    } else {
        c.cycles += 9;
        c.g[V1][0] = 0x0048_0000;
        c.mult(0, A0, A1, A0);
        c.g[V1][0] = sx(0x0047_8d38);
        c.g[A2][0] = sx(this.wrapping_add(0x260));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x0047_8d38));
        let aligned = (c.g[A0][0] | c.g[A2][0]) & 7 == 0;
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60));
        if aligned {
            es_block_move(c, A0, A2, [T0, V1, A1, A3], 11, 0x0020_1b4c, true, 3);
        } else {
            es_block_move(c, A0, A2, [T0, V1, A1, A3], 20, 0x0020_1af4, false, 3);
            c.cycles += 2;
        }
        c.cycles += 6;
        es_move_tail(c, A0, A2, T0, V0);
    }
    c.cycles += 10;
    c.g[V0][0] = 1;
    c.g[A1][0] = 0;
    Addr(this.wrapping_add(0x2cc)).sw(c, 1);
    c.g[A0][0] = c.g[S2][0];
    action::SET_MAP_TAB_MODE.call(c, 0x0020_1ba4)?;
    c.g[A0][0] = c.g[S2][0];
    action::REFRESH_PEAK_LIST.call(c, 0x0020_1bac)?;
    c.g[A1][0] = es_global(c, es::MODE).lw(c);
    c.g[A0][0] = c.g[S2][0];
    es_refresh_mode(c)
}

/// The confirm popup on the peak panel: "yes" (row 0) starts the peak's free
/// ride (event type 4, event 12) and leaves; "no" closes the popup.
fn es_accept_confirm(c: &mut Cpu, this: u32, w: u32) -> Next {
    c.cycles += 8;
    es_sound(c, 6, false, 0x0020_1be8)?;
    c.g[V0][0] = Addr(w.wrapping_add(0x18)).lw(c);
    if c.g[V0][0] != 0 {
        c.g[A1][0] = es_global(c, es::MODE).lw(c);
        c.cycles += 9;
        c.g[A0][0] = c.g[S2][0];
        Addr(this.wrapping_add(0x2cc)).sw(c, 0);
        action::SET_MODE.call(c, 0x0020_1d00)?;
        c.g[A0][0] = c.g[S2][0];
        action::UPDATE_MODE_PANELS.call(c, 0x0020_1d08)?;
        c.g[A0][0] = c.g[S2][0];
        action::UPDATE_CONFIRM_POPUP.call(c, 0x0020_1d10)?;
        c.g[V0][0] = 0;
        return Ok(Exit::Value);
    }
    c.cycles += 31;
    c.g[V0][0] = es_global(c, es::WORLD).lw(c);
    c.g[A1][0] = 0;
    c.g[A0][0] = reg_at(c, V0, 0x78).lw(c);
    call_ptr(c, action::BE_LIBRARY_GET_INTERFACE, 0x0020_1c04)?;
    c.g[V1][0] = es_global(c, es::WORLD).lw(c);
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = 0;
    c.g[S1][0] = reg_at(c, V1, 0xc0).lw(c);
    call_ptr(c, action::BE_LIBRARY_SET_PHASE, 0x0020_1c18)?;
    c.g[V0][0] = Addr(this.wrapping_add(0x260)).lw(c);
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = 4;
    es_global(c, es::RACE_PEAK).sw(c, c.g[V0][0]);
    call_ptr(c, action::BE_NEW_RACE_INT_SET_EVENT_TYPE, 0x0020_1c2c)?;
    c.g[A1][0] = 0xc;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, action::BE_NEW_RACE_INT_SET_EVENT, 0x0020_1c38)?;
    es_commit_race(c, V0, 0x0020_1c4c)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = 0xc;
    call_ptr(c, action::GAME_MODE_MAN_START_MODE, 0x0020_1c58)?;
    // The game module: into state 0xe (event select -> race), flagged.
    c.g[V1][0] = es_global(c, es::WORLD).lw(c);
    c.g[A0][0] = 0xe;
    c.g[V1][0] = reg_at(c, V1, 0x84).lw(c);
    c.g[V0][0] = reg_at(c, V1, 0x214).lw(c);
    let in_state = c.g[V0][0] == c.g[A0][0];
    c.g[A0][0] = c.g[V1][0];
    if in_state {
        c.cycles += 5;
        c.g[V1][0] = reg_at(c, V1, 0x200).lw(c);
        reg_at(c, V1, 8).sw(c, c.g[S4][0]);
        c.g[V0][0] = Addr(this.wrapping_add(0x260)).lw(c);
        reg_at(c, V1, 0x18).sw(c, c.g[V0][0]);
    } else {
        c.cycles += 8;
        c.g[A1][0] = 0xe;
        c.g[A2][0] = 1;
        c.g[A3][0] = 0;
        call_ptr(c, action::GAME_MODULE_REQUEST_STATE, 0x0020_1c94)?;
        c.g[V0][0] = es_global(c, es::WORLD).lw(c);
        c.g[V1][0] = reg_at(c, V0, 0x84).lw(c);
        c.g[A0][0] = reg_at(c, V1, 0x200).lw(c);
        reg_at(c, A0, 8).sw(c, c.g[S4][0]);
    }
    c.cycles += 5;
    c.g[V0][0] = 3;
    c.g[A0][0] = es_global(c, es::FLOW).lw(c);
    c.g[V1][0] = 2;
    es_global(c, es::NEXT_STATE).sw(c, c.g[V0][0]);
    if c.g[A0][0] == c.g[V1][0] {
        // Pop the loading-screen history.
        c.cycles += 15;
        c.g[V0][0] = es_global(c, es::LOAD_DEPTH).lw(c);
        c.g[A0][0] = sx(LOAD_SCREEN_STACK);
        c.g[A1][0] = sx(LOAD_SCREEN_TABLE);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
        es_global(c, es::LOAD_DEPTH).sw(c, c.g[V0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(LOAD_SCREEN_STACK));
        c.g[V0][0] = reg_at(c, V1, 0).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(LOAD_SCREEN_TABLE));
        c.g[V1][0] = reg_at(c, V0, 0).lw(c);
        es_global(c, es::LOAD_SCREEN).sw(c, c.g[V1][0]);
    }
    es_audio_peak(c, this)
}

/// Accept on the tab panel: an unlocked tab (`self[4 + id]`) opens its
/// event list, else the "denied" sound.
fn es_accept_tab(c: &mut Cpu, this: u32, w: u32) -> Next {
    c.cycles += 6;
    c.g[V0][0] = Addr(w.wrapping_add(0x18)).lw(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    c.g[V0][0] = sx(this.wrapping_add(c.g[V0][0] as u32));
    c.g[V1][0] = reg_at(c, V0, 0x10).lw(c);
    c.g[A1][0] = 6;
    if c.g[V1][0] == 0 {
        return es_deny(c, 0x0020_1d78);
    }
    c.cycles += 12;
    es_sound(c, 6, false, 0x0020_1d44)?;
    c.g[V0][0] = Addr(w.wrapping_add(0x18)).lw(c);
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = 2;
    es_global(c, es::TAB).sw(c, c.g[V0][0]);
    action::SET_MODE.call(c, 0x0020_1d58)?;
    es_show_mode(c)
}

/// Accept on the event panel.
fn es_accept_event(c: &mut Cpu, this: u32, w: u32) -> Next {
    c.cycles += 4;
    c.g[V0][0] = es_global(c, es::FLOW).lw(c);
    let front_end = c.g[V0][0] < 2;
    c.g[V0][0] = front_end as u64;
    c.g[A0][0] = es_global(c, es::PEAK).lw(c);
    if front_end {
        c.cycles += 12;
        c.g[A1][0] = 0x60;
        es_event_row(c, this, w, A0, A1);
        c.g[V1][0] = reg_at(c, V0, 0x20).lw(c);
        let unlocked = c.g[V1][0] != 0;
        c.g[V0][0] = es_global(c, es::WORLD).lw(c);
        if unlocked {
            return es_start_event(c, w);
        }
    }
    es_event_confirm(c, this, w)
}

/// Front end, an unlocked event: set up the race from the event record and
/// leave (flow 1 also quits to the load screen, clearing the rule flags when
/// the game mode changed).
fn es_start_event(c: &mut Cpu, w: u32) -> Next {
    c.cycles += 13;
    c.g[A1][0] = 0;
    c.g[A0][0] = reg_at(c, V0, 0x78).lw(c);
    call_ptr(c, action::BE_LIBRARY_GET_INTERFACE, 0x0020_1dcc)?;
    c.g[S0][0] = c.g[V0][0];
    es_sound(c, 6, false, 0x0020_1de8)?;
    c.g[V0][0] = es_global(c, es::FLOW).lw(c);
    if c.g[V0][0] == c.g[S4][0] {
        c.cycles += 3;
        c.g[A0][0] = 0;
        call_ptr(c, action::BE_LIBRARY_SET_PHASE, 0x0020_1dfc)?;
    }
    c.g[A2][0] = Addr(w.wrapping_add(0x18)).lw(c);
    c.cycles += 24;
    c.g[A0][0] = c.g[SP][0];
    c.g[A3][0] = es_global(c, es::TAB).lw(c);
    c.g[A1][0] = es_global(c, es::PEAK).lw(c);
    action::GET_EVENT_RECORD.call(c, 0x0020_1e10)?;
    let record = sp(c);
    c.g[V1][0] = Addr(record).lw(c);
    c.g[V0][0] = 0x0053_0000;
    c.g[A1][0] = Addr(record + 4).lw(c);
    c.g[S1][0] = sx(NEW_RACE_INFO);
    Addr(NEW_RACE_INFO + 0x40).sw(c, c.g[V1][0]);
    c.g[A0][0] = c.g[S0][0];
    c.g[S3][0] = Addr(NEW_RACE_INFO + 0x48).lb(c);
    es_global(c, es::EVENT_STARTED).sw(c, c.g[S4][0]);
    call_ptr(c, action::BE_NEW_RACE_INT_SET_EVENT_TYPE, 0x0020_1e34)?;
    c.g[A1][0] = Addr(record + 8).lw(c);
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, action::BE_NEW_RACE_INT_SET_EVENT, 0x0020_1e40)?;
    es_commit_race(c, A1, 0x0020_1e54)?;
    c.g[V0][0] = es_global(c, es::FLOW).lw(c);
    if c.g[V0][0] == c.g[S4][0] {
        c.cycles += 3;
        c.g[V0][0] = reg_at(c, S1, 0x48).lb(c);
        let same_mode = c.g[S3][0] == c.g[V0][0];
        c.g[A0][0] = 0x0053_0000;
        if !same_mode {
            c.cycles += 4;
            c.g[A1][0] = 0;
            c.g[A0][0] = sx(0x0053_08b8);
            c.g[A2][0] = 0x20;
            call_ptr(c, action::EA_MEM_SET, 0x0020_1e7c)?;
        }
        c.cycles += 6;
        c.g[V0][0] = es_global(c, es::WORLD).lw(c);
        c.g[A1][0] = 1;
        c.g[A0][0] = reg_at(c, V0, 0x84).lw(c);
        call_ptr(c, action::GAME_MODULE_QUIT_TO_LOAD_SCREEN, 0x0020_1e8c)?;
    }
    es_leave(c)
}

/// The event panel otherwise: with the confirm popup up, "go" (row 0)
/// starts the chosen event; else an unlocked event raises the popup.
fn es_event_confirm(c: &mut Cpu, this: u32, w: u32) -> Next {
    c.cycles += 3;
    c.g[V0][0] = Addr(this.wrapping_add(0x2cc)).lw(c);
    c.g[A1][0] = 6;
    if c.g[V0][0] == 0 {
        return es_choose_event(c, this, w);
    }
    c.cycles += 8;
    es_sound(c, 6, false, 0x0020_1eb4)?;
    c.g[V0][0] = Addr(w.wrapping_add(0x18)).lw(c);
    c.g[A1][0] = es_global(c, es::MODE).lw(c);
    if c.g[V0][0] != 0 {
        return es_clear_confirm(c, this);
    }
    // Event types 5/6 set race type and event from the record at +0x264;
    // otherwise a free ride (4, 12) unless the game mode is missions.
    c.cycles += 5;
    c.g[V0][0] = Addr(this.wrapping_add(0x264)).lw(c);
    let special = sx((c.g[V0][0] as u32).wrapping_sub(5)) < 2;
    c.g[V0][0] = es_global(c, es::WORLD).lw(c);
    if special {
        c.cycles += 19;
        c.g[A1][0] = 0;
        c.g[A0][0] = reg_at(c, V0, 0x78).lw(c);
        call_ptr(c, action::BE_LIBRARY_GET_INTERFACE, 0x0020_1ee0)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0;
        call_ptr(c, action::BE_LIBRARY_SET_PHASE, 0x0020_1eec)?;
        c.g[A1][0] = Addr(this.wrapping_add(0x264)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, action::BE_NEW_RACE_INT_SET_EVENT_TYPE, 0x0020_1ef8)?;
        c.g[A1][0] = Addr(this.wrapping_add(0x268)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, action::BE_NEW_RACE_INT_SET_EVENT, 0x0020_1f04)?;
        es_commit_race(c, V0, 0x0020_1f18)?;
        c.g[V0][0] = es_global(c, es::FLOW).lw(c);
    } else {
        c.cycles += 5;
        c.g[V1][0] = 0x0053_0000;
        let missions = Addr(NEW_RACE_INFO + 0x48).lb(c) ^ 4 == 0;
        c.g[V0][0] = es_global(c, es::WORLD).lw(c);
        if !missions {
            c.cycles += 17;
            c.g[A1][0] = 0;
            c.g[A0][0] = reg_at(c, V0, 0x78).lw(c);
            call_ptr(c, action::BE_LIBRARY_GET_INTERFACE, 0x0020_1f40)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = 0;
            call_ptr(c, action::BE_LIBRARY_SET_PHASE, 0x0020_1f4c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 4;
            call_ptr(c, action::BE_NEW_RACE_INT_SET_EVENT_TYPE, 0x0020_1f58)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0xc;
            call_ptr(c, action::BE_NEW_RACE_INT_SET_EVENT, 0x0020_1f64)?;
            es_commit_race(c, V0, 0x0020_1f78)?;
        }
        c.cycles += 1;
        c.g[V0][0] = es_global(c, es::FLOW).lw(c);
    }
    // The overlay flows (3, 4) hand the peak to a game module already in
    // state 0xe; the front end requests that state.
    c.cycles += 4;
    let overlay = sx((c.g[V0][0] as u32).wrapping_sub(3)) < 2;
    c.g[V0][0] = es_global(c, es::WORLD).lw(c);
    if overlay {
        c.cycles += 5;
        c.g[A0][0] = 0xe;
        c.g[V0][0] = reg_at(c, V0, 0x84).lw(c);
        c.g[V1][0] = reg_at(c, V0, 0x214).lw(c);
        if c.g[V1][0] == c.g[A0][0] {
            c.cycles += 8;
            c.g[A0][0] = reg_at(c, V0, 0x200).lw(c);
            c.g[V0][0] = 1;
            reg_at(c, A0, 8).sw(c, 1);
            c.g[V1][0] = Addr(this.wrapping_add(0x260)).lw(c);
            reg_at(c, A0, 0x18).sw(c, c.g[V1][0]);
            c.g[V0][0] = Addr(this.wrapping_add(0x260)).lw(c);
            es_global(c, es::RACE_PEAK).sw(c, c.g[V0][0]);
        }
        c.g[V0][0] = 3;
        c.cycles += 2;
        es_global(c, es::NEXT_STATE).sw(c, c.g[V0][0]);
    } else {
        c.cycles += 22;
        c.g[A1][0] = 0xe;
        c.g[V1][0] = Addr(this.wrapping_add(0x260)).lw(c);
        c.g[A2][0] = 1;
        c.g[A0][0] = reg_at(c, V0, 0x84).lw(c);
        c.g[A3][0] = 0;
        es_global(c, es::RACE_PEAK).sw(c, c.g[V1][0]);
        call_ptr(c, action::GAME_MODULE_REQUEST_STATE, 0x0020_1fe4)?;
        c.g[A2][0] = es_global(c, es::LOAD_DEPTH).lw(c);
        c.g[V1][0] = sx(LOAD_SCREEN_STACK);
        c.g[A3][0] = es_global(c, es::WORLD).lw(c);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
        c.g[V0][0] = sx((c.g[A2][0] as u32) << 2);
        c.g[A0][0] = sx(LOAD_SCREEN_TABLE);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(LOAD_SCREEN_STACK));
        c.g[A1][0] = reg_at(c, A3, 0x84).lw(c);
        c.g[V1][0] = reg_at(c, V0, 0).lw(c);
        c.g[A3][0] = 3;
        c.g[T1][0] = reg_at(c, A1, 0x200).lw(c);
        c.g[T0][0] = 1;
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        es_store_load_screen(c);
    }
    es_start_special(c, this)
}

/// No popup yet: an unlocked event row raises the confirm popup with the
/// event's record copied to `self+0x260`; a locked one is denied.
fn es_choose_event(c: &mut Cpu, this: u32, w: u32) -> Next {
    c.cycles += 13;
    c.g[A0][0] = es_global(c, es::PEAK).lw(c);
    c.g[A1][0] = 0x60;
    es_event_row(c, this, w, A0, A1);
    c.g[V1][0] = reg_at(c, V0, 0x20).lw(c);
    c.g[A1][0] = 6;
    if c.g[V1][0] == 0 {
        return es_deny(c, 0x0020_21d0);
    }
    c.cycles += 19;
    c.g[S0][0] = sx(this.wrapping_add(0x260));
    es_sound(c, 6, true, 0x0020_20d8)?;
    c.g[V1][0] = 1;
    c.g[A0][0] = c.g[S2][0];
    Addr(this.wrapping_add(0x2cc)).sw(c, 1);
    c.g[A1][0] = 0;
    action::SET_MAP_TAB_MODE.call(c, 0x0020_20ec)?;
    c.g[A2][0] = Addr(w.wrapping_add(0x18)).lw(c);
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] = es_global(c, es::PEAK).lw(c);
    c.g[A3][0] = es_global(c, es::TAB).lw(c);
    action::GET_EVENT_RECORD.call(c, 0x0020_2100)?;
    c.g[V0][0] = c.g[S0][0] & 7;
    let aligned = c.g[V0][0] == 0;
    c.g[V1][0] = c.g[SP][0];
    c.cycles += 1;
    c.g[V0][0] = sx(sp(c).wrapping_add(0x60));
    if aligned {
        es_block_move(c, V1, S0, [A1, A2, A3, T0], 11, 0x0020_2168, true, 3);
    } else {
        es_block_move(c, V1, S0, [A2, A3, T0, A0], 19, 0x0020_2110, false, 3);
        c.cycles += 2;
    }
    c.cycles += 9;
    es_move_tail(c, V1, S0, V0, A0);
    c.g[A1][0] = es_global(c, es::MODE).lw(c);
    c.g[A0][0] = c.g[S2][0];
    es_refresh_mode(c)
}

// -------------------------------------------------------------- 6: back

/// Action 6 (back): close the confirm popup, else step back a panel; from
/// the peak panel leave the screen (in the overlay flow 3 also handing the
/// race's track to the game module and telling the audio system).
fn es_back(c: &mut Cpu, this: u32) -> Next {
    c.cycles += 3;
    c.g[V0][0] = Addr(this.wrapping_add(0x2cc)).lw(c);
    c.g[A1][0] = es_global(c, es::MODE).lw(c);
    if c.g[V0][0] != 0 {
        return es_clear_confirm(c, this);
    }
    c.cycles += 4;
    c.g[A0][0] = es_global(c, es::FLOW).lw(c);
    c.g[V0][0] = 3;
    c.g[V1][0] = es_global(c, es::MODE).lw(c);
    let mode = c.g[V1][0];
    if c.g[A0][0] == c.g[V0][0] {
        c.cycles += 2;
        c.g[V0][0] = 1;
        if mode == 0 {
            c.cycles += 15;
            c.g[V1][0] = es_global(c, es::WORLD).lw(c);
            c.g[A1][0] = 1;
            c.g[A0][0] = c.g[S4][0];
            c.g[V0][0] = reg_at(c, V1, 0x84).lw(c);
            c.g[S0][0] = reg_at(c, V0, 0x200).lw(c);
            reg_at(c, S0, 8).sw(c, c.g[A1][0]);
            call_ptr(c, action::BE_NEW_RACE_INT_GET_TRACK_INFO, 0x0020_2240)?;
            c.g[V1][0] = reg_at(c, V0, 0).lw(c);
            reg_at(c, S0, 0x18).sw(c, c.g[V1][0]);
            call_ptr(c, action::AUDIO_GET_INSTANCE, 0x0020_224c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            call_ptr(c, action::AUDIO_ON_EVENT_SELECT_EXIT, 0x0020_2258)?;
            return es_leave(c);
        }
    } else {
        c.cycles += 1;
        c.g[V0][0] = 1;
    }
    c.cycles += 2;
    c.g[V0][0] = 2;
    if mode == 1 {
        c.cycles += 5;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 0;
        action::SET_MODE.call(c, 0x0020_2278)?;
        return es_show_mode(c);
    }
    c.cycles += 2;
    c.g[V0][0] = 4;
    if mode == 2 {
        c.cycles += 7;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 1;
        action::SET_MODE.call(c, 0x0020_2294)?;
        c.g[A0][0] = c.g[S2][0];
        action::UPDATE_MODE_PANELS.call(c, 0x0020_229c)?;
        c.g[V0][0] = 0;
        return Ok(Exit::Value);
    }
    c.cycles += 2;
    let stay = c.g[A0][0] == c.g[V0][0];
    c.g[V0][0] = 0;
    if stay {
        return Ok(Exit::Value);
    }
    es_leave(c)
}

// ---------------------------------------------------------- 7: map tab

/// Action 7 (challenge flows only, no popup): toggle the map tab, unless
/// the peak list (mode 0) or the peak's goal list (mode 1) is still
/// animating in (its object's state byte +0x95 is 3).
fn es_toggle_tab(c: &mut Cpu, this: u32) -> Next {
    c.cycles += 4;
    c.g[V0][0] = es_global(c, es::FLOW).lw(c);
    let front_end = c.g[V0][0] < 2;
    c.g[V0][0] = 0;
    if front_end {
        return Ok(Exit::Value);
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(this.wrapping_add(0x2cc)).lw(c);
    let confirm = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    if confirm {
        return Ok(Exit::Value);
    }
    c.cycles += 3;
    c.g[V1][0] = es_global(c, es::MODE).lw(c);
    c.g[V0][0] = 1;
    if c.g[V1][0] == 0 {
        // "SP_PeakList".
        c.cycles += 9;
        c.g[A0][0] = sx(0x0047_0600);
        c.g[S0][0] = 0;
        call_ptr(c, action::STR_HASH, 0x0020_22f4)?;
        c.g[A0][0] = Addr(this.wrapping_add(0x2e8)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, action::UI_SCREEN_FIND_OBJECT, 0x0020_2300)?;
        if c.g[V0][0] != 0 {
            c.g[S0][0] = reg_at(c, V0, 0x95).lbu(c);
        }
        c.cycles += 3;
        c.g[V0][0] = 3;
        if c.g[S0][0] == c.g[V0][0] {
            return es_show_mode(c);
        }
        c.cycles += 2;
        c.g[A1][0] = Addr(this).lw(c);
        return es_flip_tab(c, 11, 0x0020_2378, 0x0020_2390);
    }
    c.cycles += 2;
    if c.g[V1][0] != c.g[V0][0] {
        c.g[A1][0] = Addr(this).lw(c);
        return es_flip_tab(c, 9, 0x0020_23a4, 0x0020_23bc);
    }
    // "SPG_Peak%dGoals" for the selected peak.
    c.cycles += 13;
    c.g[A2][0] = es_global(c, es::PEAK).lw(c);
    c.g[A1][0] = sx(0x0047_06d8);
    c.g[A0][0] = c.g[SP][0];
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
    call_ptr(c, action::SPRINTF, 0x0020_233c)?;
    c.g[A0][0] = c.g[SP][0];
    call_ptr(c, action::STR_HASH, 0x0020_2344)?;
    c.g[A0][0] = Addr(this.wrapping_add(0x2e8)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, action::UI_SCREEN_FIND_OBJECT, 0x0020_2350)?;
    let found = c.g[V0][0] != 0;
    c.g[V1][0] = 0;
    if found {
        c.cycles += 1;
        c.g[V1][0] = reg_at(c, V0, 0x95).lbu(c);
    }
    c.cycles += 3;
    c.g[V0][0] = 3;
    if c.g[V1][0] == c.g[V0][0] {
        return es_show_mode(c);
    }
    c.cycles += 1;
    c.g[A1][0] = Addr(this).lw(c);
    es_flip_tab(c, 11, 0x0020_2378, 0x0020_2390)
}

/// `SetMapTabMode(self, self->tab ^ 1)` (a1 = `self->tab`), the "select"
/// sound, then refresh.
fn es_flip_tab(c: &mut Cpu, cycles: u64, ret1: u32, ret2: u32) -> Next {
    c.cycles += cycles;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] ^= 1;
    action::SET_MAP_TAB_MODE.call(c, ret1)?;
    es_sound(c, 6, false, ret2)?;
    es_show_mode(c)
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

    /// `EventSelect_OnWidgetAction` against the generated body. Twenty
    /// setups (`seed % 20`) steer it into every action and panel: row
    /// colours (with/without the confirm popup, every mode, the highlight
    /// object found or not), accept on each panel (deny, pick peak/tab,
    /// the challenge record copies aligned and not, the free ride, starting
    /// an event with and without the rule-flag reset, the confirm popup's
    /// special/missions/free-ride and overlay/front-end flows), back, the
    /// map-tab toggle, a null widget, out-of-range actions and an unknown
    /// jump-table target. The real callees take their short paths: the
    /// screen (`+0x2e8`) is null except where the highlight/toggle needs an
    /// object, the UI state has an empty list, and the BELibrary interfaces,
    /// sound player, widget and object vtables end in fakes.
    #[test]
    fn event_select_on_widget_action_matches() {
        use crate::difftest::fakes::FAKE;
        use crate::game::be::bedbreward::tests::alloc_ready;
        // Fixture objects (above the fake-pointer and heap ranges).
        const T_ES: u32 = 0x008a_0000;
        const T_WIDGET: u32 = 0x008a_1000;
        const T_STATE: u32 = 0x008a_2000;
        const T_SOUND: u32 = 0x008a_2100;
        const T_PLAYER: u32 = 0x008a_2200;
        const T_PVT: u32 = 0x008a_2300;
        const T_WVT: u32 = 0x008a_2400;
        const T_SCREEN: u32 = 0x008a_3000;
        const T_COUNT: u32 = 0x008a_3100;
        const T_LIST: u32 = 0x008a_3200;
        const T_OBJ: u32 = 0x008a_4000;
        const T_OVT: u32 = 0x008a_5000;
        const T_WORLD: u32 = 0x008a_6000;
        const T_GM: u32 = 0x008a_7000;
        const T_GM_STATE: u32 = 0x008a_7400;
        const T_REPLAY: u32 = 0x008a_7800;
        const T_MODE_MAN: u32 = 0x008a_8000;
        const T_ASYNC: u32 = 0x008a_9000;
        const T_AUDIO: u32 = 0x008b_0000;
        const T_BELIB: u32 = 0x008b_8000;
        const T_APP: u32 = 0x008b_9000;
        const NEG1: u32 = 0xffff_ffff;
        check(
            ssx3_ee::s05::f_002018a8__generated,
            event_select_on_widget_action,
            480,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0020_18a8, 6);
                alloc_ready(c);
                let pick = |r: &mut Rng, xs: &[u32]| xs[r.next_u32() as usize % xs.len()];
                let g = |off: i32| GAME_GP.wrapping_add(off as u32);

                // Globals: BELibrary dummy, world, audio system, default
                // sound player, the app (QuitToLoadScreen's next state).
                c.bus.write32(g(-0x1f04), T_BELIB);
                c.bus.write32(g(-0x848), T_WORLD);
                c.bus.write32(g(0x410), T_AUDIO);
                c.bus.write32(g(0x2968), T_PVT);
                c.bus.write32(0x004a_28a8, T_APP);
                c.bus.write32(T_WORLD + 0x78, T_BELIB);
                c.bus.write32(T_WORLD + 0x84, T_GM);
                c.bus.write32(T_WORLD + 0xc0, T_MODE_MAN);
                c.bus.write32(T_WORLD + 0x11c, T_ASYNC);
                c.bus.write32(T_GM + 0x28, T_REPLAY);
                c.bus.write32(T_GM + 0x200, T_GM_STATE);
                c.bus.write32(T_GM + 0x214, pick(&mut r, &[0xe, 0xe, 1]));
                // Music off (no StopSong), no loading-screen loop.
                c.bus.write32(T_AUDIO + 0x608c, 2);
                c.bus.write32(T_AUDIO + 0x62a8, NEG1);
                // Track ids that never equal the selected peak (no
                // RestartPeakMusic).
                for k in 0..4 {
                    c.bus.write32(0x0043_d950 + k * 100, 0x7fff_ffff);
                }
                // Loading-screen history.
                c.bus.write32(g(-0x69c), 1 + r.next_u32() % 3);
                for i in 0..4 {
                    c.bus.write32(0x0053_66e8 + 4 * i, i);
                    c.bus.write32(0x0044_28f0 + 4 * i, 0x100 + i);
                }
                // Character profile flags 15/16 (peaks 2 and 3).
                c.bus
                    .write32(0x004a_7774, pick(&mut r, &[0, 1 << 15, 1 << 16, 3 << 15]));
                // EventSelect_OnWidgetAction's jump table (`D_00470780`).
                let mut table = [
                    0x0020_23d4,
                    0x0020_261c,
                    0x0020_270c,
                    0x0020_270c,
                    0x0020_193c,
                    0x0020_21d8,
                    0x0020_22bc,
                ];

                // The case.
                let k = seed % 20;
                let mut action: u64 = 5;
                let mut mode: u32 = 0;
                let mut confirm = 0u32;
                let mut flow = 0u32;
                let mut row = r.next_u32() % 3;
                let mut screen = false;
                let mut widget = T_WIDGET;
                let mut event_flag = r.next_u32() % 2;
                let mut peak_flag = r.next_u32() % 4 != 0;
                let mut special = 0u32;
                let mut missions = false;
                let mut reset_rules = false;
                let modes = [0, 1, 2, NEG1, 3];
                match k {
                    0 => {
                        action = 1;
                        confirm = 1;
                        row = pick(&mut r, &[0, 0, 1, 2]);
                    }
                    1 => {
                        action = 1;
                        mode = pick(&mut r, &modes);
                        screen = r.next_u32() % 2 == 0;
                    }
                    2 => {
                        action = 1;
                        mode = pick(&mut r, &[1, 2]);
                        screen = true;
                    }
                    3 => {
                        action = 2;
                        confirm = (r.next_u32() % 3 == 0) as u32;
                        mode = pick(&mut r, &modes);
                    }
                    4 => {
                        action = 2;
                        mode = pick(&mut r, &[0, 1, 2, NEG1, 7]);
                    }
                    5 => {
                        mode = 1;
                        row = r.next_u32() % 4;
                    }
                    6 => {
                        confirm = 1;
                        row = pick(&mut r, &[0, 0, 1]);
                        flow = pick(&mut r, &[0, 1, 2, 2, 3]);
                    }
                    7 => row = r.next_u32() % 5,
                    8 | 9 => {
                        row = pick(&mut r, &[1, 2]);
                        peak_flag = true;
                        flow = pick(&mut r, &[0, 1, 2, 3, 4]);
                    }
                    10 => {
                        mode = 2;
                        flow = pick(&mut r, &[0, 1, 1]);
                        event_flag = 1;
                        reset_rules = r.next_u32() % 2 == 0;
                    }
                    11 => {
                        mode = 2;
                        flow = pick(&mut r, &[0, 1, 2, 3, 4]);
                    }
                    12 | 13 => {
                        mode = 2;
                        confirm = 1;
                        flow = pick(&mut r, &[0, 1, 2, 3, 4]);
                        if flow < 2 {
                            event_flag = 0;
                        }
                        row = pick(&mut r, &[0, 0, 1]);
                        special = pick(&mut r, &[5, 6, 0, 3]);
                        missions = r.next_u32() % 2 == 0;
                    }
                    14 => mode = pick(&mut r, &[NEG1, 3, 7]),
                    15 => {
                        action = 6;
                        confirm = (r.next_u32() % 3 == 0) as u32;
                        mode = pick(&mut r, &[0, 1, 2, NEG1]);
                        flow = pick(&mut r, &[0, 1, 3, 4]);
                    }
                    16 => {
                        action = 6;
                        flow = 3;
                    }
                    17 => {
                        action = 7;
                        flow = pick(&mut r, &[0, 1, 2, 3, 4]);
                        confirm = (r.next_u32() % 4 == 0) as u32;
                        mode = pick(&mut r, &[0, 1, 2, NEG1]);
                        screen = mode < 2 && r.next_u32() % 2 == 0;
                    }
                    18 => {
                        action = 7;
                        flow = pick(&mut r, &[2, 3, 4]);
                        mode = pick(&mut r, &[0, 1]);
                        screen = true;
                    }
                    _ => match seed / 20 % 5 {
                        0 => widget = 0,
                        1 => action = 0,
                        2 => action = 8,
                        3 => action = pick(&mut r, &[3, 4]) as u64,
                        _ => {
                            action = 4;
                            table[3] = FAKE + 0x100;
                        }
                    },
                }
                put_words(c, 0x0047_0780, &table);
                let peak = r.next_u32() % 3;
                let tab = if action == 5 {
                    r.next_u32() % 2
                } else {
                    r.next_u32() % 3
                };
                c.bus.write32(g(-0xb54), mode);
                c.bus.write32(g(-0xb5c), flow);
                c.bus.write32(g(-0xb4c), peak);
                c.bus.write32(g(-0xb50), tab);
                if missions {
                    c.bus.write8(0x0053_5c10, 4);
                }
                if reset_rules {
                    // SetEventType(3) is allowed: the game mode byte
                    // changes, so the rule flags are cleared.
                    c.bus.write8(0x0043_e700 + 3, 1);
                }
                // The event record GetEventRecord(peak, row, tab) copies.
                let rec = if tab == 0 {
                    0x0047_81d0 + peak * 0x1b0 + row * 0x6c
                } else {
                    0x0047_86e0 + peak * 0x21c + row * 0x6c
                };
                c.bus.write32(rec, r.next_u32() % 4);
                c.bus
                    .write32(rec + 4, if reset_rules { 3 } else { r.next_u32() % 3 });
                c.bus.write32(rec + 8, r.next_u32() % 16);

                // The event-select panel (4-aligned, so the record copies at
                // +0x260 take the unaligned loops too).
                let this = T_ES + 4 * (r.next_u32() % 2);
                c.bus.write32(this, r.next_u32() % 2);
                for i in 0..3 {
                    let f = if i == row {
                        peak_flag as u32
                    } else {
                        r.next_u32() % 2
                    };
                    c.bus.write32(this + 4 + 4 * i, f);
                    c.bus.write32(this + 0x10 + 4 * i, r.next_u32() % 2);
                }
                c.bus
                    .write32(this + 0x20 + peak * 0x60 + tab * 0x20 + 4 * row, event_flag);
                c.bus.write32(this + 0x260, r.next_u32() % 3);
                c.bus.write32(this + 0x264, special);
                c.bus.write32(this + 0x268, pick(&mut r, &[0, 1, 0xc, 3]));
                c.bus.write32(this + 0x2cc, confirm);
                c.bus.write32(this + 0x2d0, r.next_u32() % 2);
                c.bus
                    .write32(this + 0x2e8, if screen { T_SCREEN } else { 0 });
                c.bus.write32(this + 0x2ec, T_STATE);
                // Its UI state: the sound player (or the default one).
                c.bus.write32(T_STATE + 0x10, T_SOUND);
                c.bus.write32(
                    T_SOUND + 0x14,
                    if r.next_u32() % 4 == 0 { 0 } else { T_PLAYER },
                );
                c.bus.write32(T_PLAYER, T_PVT);
                c.bus.write16(T_PVT + 0x10, (r.next_u32() & 0x3c) as u16);
                c.bus.write32(T_PVT + 0x14, FAKE);
                // The screen: one object (every name hashes to 0 in test
                // RAM), its state byte, position and an all-fake vtable.
                c.bus.write32(T_SCREEN + 0x38, T_COUNT);
                c.bus.write32(T_SCREEN + 0x3c, T_LIST);
                c.bus.write32(T_COUNT, 1);
                c.bus.write32(T_LIST, T_OBJ);
                c.bus.write32(T_OBJ + 8, T_OVT);
                c.bus
                    .write8(T_OBJ + 0x95, pick(&mut r, &[3, 3, 0, 1]) as u8);
                for o in 0..3 {
                    c.bus.write32(T_OBJ + 0x44 + 4 * o, r.f32(500.0));
                    c.bus.write32(T_WIDGET + 0x44 + 4 * o, r.f32(500.0));
                }
                for slot in (0..0x100).step_by(8) {
                    c.bus.write32(T_OVT + slot + 4, FAKE + 0x20);
                }
                // The widget: its row and colour-setter vtable.
                c.bus.write32(T_WIDGET + 8, T_WVT);
                c.bus.write32(T_WIDGET + 0x18, row);
                c.bus.write16(T_WVT + 0x58, (r.next_u32() & 0x3c) as u16);
                c.bus.write32(T_WVT + 0x5c, FAKE + 0x10);

                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(widget);
                c.g[A2][0] = action;
                // `alloc_ready` leaves `pc` at the sentinel return address;
                // the generated body dispatches to its resume entries
                // unless entered at its start.
                c.pc = 0x0020_18a8;
            },
        );
    }
}
