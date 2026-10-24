//! `src/fe/festaterewards3.cpp`: hand-written replacements (moved from the
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
pub(crate) use crate::game::animation::rideranimbase::sw;
pub(crate) use crate::game::dirtysock::tags2::lw as lw_2;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::feasyncfile::lh_r;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw_3;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::fepopup3::transition_in;
pub(crate) use crate::game::fe::festaterewards2::handle_key_6_8_9;
pub(crate) use crate::game::fe::festaterewards2::handle_list_key;
pub(crate) use crate::game::fe::ovstatepause::lw;
pub(crate) use crate::game::main::ssxappdtor::p;
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
        0x001f_4830,
        cfestatebuyattrib_init,
        "cFEStateBuyAttrib_Init",
    ),
    (
        0x001f_4910,
        fe_state_buy_attrib_on_widget_created,
        "cFEStateBuyAttrib_OnWidgetCreated",
    ),
    (
        0x001f_49d0,
        fe_state_buy_attrib_transition_in,
        "cFEStateBuyAttrib_TransitionIn",
    ),
    (
        0x001f_49f8,
        c_fe_state_buy_attrib_on_activate,
        "cFEStateBuyAttrib_OnActivate",
    ),
    (
        0x001f_4a38,
        cfe_state_buy_attrib_on_widget_notify,
        "cFEStateBuyAttrib_OnWidgetNotify",
    ),
    (
        0x001f_4a60,
        c_fe_state_buy_attrib_on_child_event,
        "cFEStateBuyAttrib_OnChildEvent",
    ),
    (
        0x001f_54b0,
        c_fe_state_career_stats_ctor,
        "cFEStateCareerStats_ctor",
    ),
    (
        0x001f_55c0,
        fe_state_career_stats_transition_in,
        "cFEStateCareerStats_TransitionIn",
    ),
    (
        0x001f_55e8,
        c_fe_state_career_stats_on_activate,
        "cFEStateCareerStats_OnActivate",
    ),
    (
        0x001f_5618,
        cfe_state_career_stats_on_widget_notify,
        "cFEStateCareerStats_OnWidgetNotify",
    ),
    (
        0x001f_5650,
        c_fe_state_career_stats_on_widget_action,
        "cFEStateCareerStats_OnWidgetAction",
    ),
    (
        0x001f_6840,
        cfe_state_career_stats_set_text,
        "cFEStateCareerStats_SetText",
    ),
];

// ---------------------------------------------------------------------------
// Callees (same-shard direct calls; cross-shard ones stay `call_ptr`s).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::be::beintmission::callee::BUY_ATTRIB_UPDATE_BARS;
    pub(crate) use crate::game::be::beintmission::callee::CAMERA_TRIGGER_EMPTY_STUB;
    pub(crate) use crate::game::be::beintmission::callee::CAREER_STATS_SHOW_HIGHLIGHTS;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_SET_ALGORITHM;
    pub(crate) use crate::game::be::beintmission::callee::MEMCARD_SCAN_STEP;
    pub(crate) use crate::game::be::beintmission::callee::MISSION_COUNT;
    pub(crate) use crate::game::be::beintmission::callee::MISSION_NTH;
    pub(crate) use crate::game::be::beintmission::callee::MPEG2_ERROR;
    pub(crate) use crate::game::be::beintmission::callee::NET_FE_STATE_ON_WIDGET_ACTION;
    pub(crate) use crate::game::be::beintmission::callee::REWARDS_FILE_LOAD;
    pub(crate) use crate::game::be::beintmission::callee::UBER_TRICK_REFRESH;
    pub(crate) use crate::game::be::beintmission::callee::UITEXT_SET_WIDE_STRING_INTERNAL;
    pub(crate) use crate::game::be::beintmission::callee::VIEW_POPUP_SHOW_MESSAGE;
    pub(crate) use crate::game::be::beintmission::callee::WORLD_OBJ_RUN_KILL_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_APPLY_OUTPUT_MODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_SET_OUTPUT;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_BAKE_GROUP;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_EXPORT_HISTORY_FILES;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_FREE_HISTORIES;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_LOAD_HISTORY_FILE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_INSTANCE_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_BE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_INTERFACE_CALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_REWARD_INT_IS_UNLOCKED;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOLT_FILE_INDEX_GET_NEXT_NODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_DESTROY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_SET_PRESENCE_ADDR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE as BUILTIN_DELETE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAM_CONTROLLER_SET_ALGORITHM;
    pub(crate) use crate::game::dirtysock::tags2::callee::CBXSTRING_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_REFRESH_BUY_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_REFRESH_EQUIP_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_UPDATE_HELP_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY_2 as CHAT_ROOM_UPDATE_PLAYER_QUALITY;
    pub(crate) use crate::game::dirtysock::tags2::callee::CROWD_MAN2D_INIT_SLOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_BX_STRING_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_LUNO_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_GET_SAVE_NAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_LIST_DIR_ALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_READ_SAVE;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SETUP_MAN_SEND_QUIT;
    pub(crate) use crate::game::dirtysock::tags2::callee::DYNAMIC_EMITTER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::DYNAMIC_EMITTER_FREE_DATA;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEMSET;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE_3 as EA_MEM_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::EMPTY_CHILD_HANDLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::FEMODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_MODULE_GET_PLAYER_PAD_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_ON_ACTIVATE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_STATS_REFRESH_RESULTS_2 as FE_STATE_STATS_REFRESH_RESULTS;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_UPDATE_BUTTONS_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::FLOAT_TO_UINT;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_COMM_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_STATE_PRE_RACE_MENU_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_EVENT_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_HEAP_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_RAMP_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::INSEQ_SPAN_TRACK_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_IS_KEY_ENABLED as KEYBOARD_POPUP_IS_KEY_ENABLED_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_SHOW;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_UPDATE_CURSOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_LIST_GET_AT_3 as LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DISCONNECT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_END_OPERATION_2 as MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_REQUEST_CARD_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMITTER_FREE_DATA;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMORY_FREE;
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
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_GET_INBOX as MESSAGE_CENTER_GET_INBOX_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_MESSAGE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_CTOR_3 as NET_FE_STATE_CTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_DTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_HANDLE_NET_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_ON_CHILD_EVENT_3 as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE_2 as NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_MESSENGER_SET_PLAYER_MENU_SELECTION;
    pub(crate) use crate::game::dirtysock::tags2::callee::ON_QUIT_POPUP_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::PATH_PLAT_REFRESH_STATUS;
    pub(crate) use crate::game::dirtysock::tags2::callee::RAND_NEXT_MAIN;
    pub(crate) use crate::game::dirtysock::tags2::callee::RCMP_DECODE_FRAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::RCMP_FRAME_NUMBER;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_ADD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_HIGHLIGHT_SLOT_CLEAR_2 as REPLAY_HIGHLIGHT_SLOT_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RECORD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_REFRESH_LIST;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_UPDATE_HELP_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_UPDATE_NOW_PLAYING;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCE_MC_FORMAT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_DATA_GET_INFO_2 as SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT_2 as SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_OBJ_TABLE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_2 as SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MOVIE_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_OBJ_CAM_SET_OWNER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_PUSH_FREE_NODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SETUP_MAN_SEND_CHARACTER_MSG;
    pub(crate) use crate::game::dirtysock::tags2::callee::SLOGIN_MC_GET_PORT_SLOT_2 as SLOGIN_MC_GET_PORT_SLOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_FREE_MEMORY_2 as SND_BANK_FREE_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLINK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD_2 as SND_BANK_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SNOW_BOX_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::SOUND_PLAYER_SET_BY_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::SOUND_PLAYER_START;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPRINTF_2 as SPRINTF;
    pub(crate) use crate::game::dirtysock::tags2::callee::STATS_SELECT_CATEGORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY as STRCPY_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY as STRCPY_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::STREAM_BUFFER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::TROPHY_ROOM_RELEASE_IMAGES;
    pub(crate) use crate::game::dirtysock::tags2::callee::UIMENU_SET_SELECTION;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_MENU_SET_SELECTED_BY_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SET_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_3 as UI_STATE_CTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_TEXT_GET_FONT;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_WIDGET_SLOT_0;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_WIDGET_SLOT_1;
    pub(crate) use crate::game::dirtysock::tags2::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::fe::ovstatepause::callee::AI_CREATE_COMPUTER_RIDERS;
    pub(crate) use crate::game::fe::ovstatepause::callee::AI_DESTROY_MISSION_RIDERS;
    pub(crate) use crate::game::fe::ovstatepause::callee::ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY;
    pub(crate) use crate::game::fe::ovstatepause::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::ovstatepause::callee::BE_REWARD_INT_COUNT_SET_BITS;
    pub(crate) use crate::game::fe::ovstatepause::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHECK_ACCOUNT_NAME;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHECK_PASSWORD;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_OV_STATE_OVERLAY_SHUTDOWN;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::EA_MEM_SET;
    pub(crate) use crate::game::fe::ovstatepause::callee::EVENT_SELECT_POST_INPUT;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_ASYNC_MANAGER_CANCEL;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_FORMAT_MONEY;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_MODULE_RIDER_PREVIEW_UNLOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_MODULE_RIDER_PREVIEW_UPDATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::KEYBOARD_POPUP_IS_KEY_ENABLED;
    pub(crate) use crate::game::fe::ovstatepause::callee::KEYBOARD_POPUP_MOVE_DOWN;
    pub(crate) use crate::game::fe::ovstatepause::callee::MC_MANAGER_BEGIN_LOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::fe::ovstatepause::callee::MEM_CARD_STATE_BASE_CTOR_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::MOUNTAIN_ROOM_POLL_IMAGE_LOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::MPEG2_FLUSH_BUFFER;
    pub(crate) use crate::game::fe::ovstatepause::callee::MPEG2_SHOW_BITS;
    pub(crate) use crate::game::fe::ovstatepause::callee::MUSIC_PLAYER_RESUME;
    pub(crate) use crate::game::fe::ovstatepause::callee::NET_FE_STATE_CTOR_2 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::ovstatepause::callee::OV_SCREEN_SESSION_DESTROY_INDICATORS;
    pub(crate) use crate::game::fe::ovstatepause::callee::PLAY_FE_SOUND;
    pub(crate) use crate::game::fe::ovstatepause::callee::RIDER_METRIX_CLEAR_PEAK_A4;
    pub(crate) use crate::game::fe::ovstatepause::callee::SECTION_MAN_REQUEST_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::ovstatepause::callee::STRCPY;
    pub(crate) use crate::game::fe::ovstatepause::callee::STRLEN;
    pub(crate) use crate::game::fe::ovstatepause::callee::STR_HASH;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_CTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_MGR_CLOSE_TOP_STATE_AND_POPUPS;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_ON_ACTIVATE_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_POST_INPUT;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_RENDER;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_UPDATE_2 as UI_STATE_UPDATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXTURE_SET_FIND;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::fe::ovstatepause::callee::VID_ENGINE_PLAYER_RENDER;
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
}

/// `bool cFEStateBuyAttrib_TransitionIn(cFEStateBuyAttrib* s, int on)`
/// (0x1f49d0).
pub fn fe_state_buy_attrib_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        transition_in(c, 0x001f_49e4, callee::BUY_ATTRIB_UPDATE_BARS)
    })
}

/// `bool cFEStateCareerStats_TransitionIn(cFEStateCareerStats* s, int on)`
/// (0x1f55c0).
pub fn fe_state_career_stats_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        transition_in(c, 0x001f_55d4, callee::CAREER_STATS_SHOW_HIGHLIGHTS)
    })
}

/// `void cFEStateBuyAttrib_OnChildEvent(self, sender, msg, arg)` (0x1f4a60):
/// on the "confirm purchase" message (0x16), commit the purchase if the
/// sender has one pending (`+0x6c`).
pub fn c_fe_state_buy_attrib_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x16;
        let msg = c.g[A2][0];
        frame.save_ra(c, 0);
        if msg == 0x16 {
            c.cycles += 3;
            c.g[V0][0] = c.lw(p(abi::arg(c, 1), 0x6c));
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                call_ptr(c, callee::BUY_ATTRIB_COMMIT_PURCHASE, 0x001f_4a84)?;
                c.cycles += 1;
            }
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCareerStats_OnActivate(self, arg)` (0x1f55e8): activate the
/// base front-end state, then select the page held in `self+0x48`.
pub fn c_fe_state_career_stats_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::FE_STATE_ON_ACTIVATE, 0x001f_55fc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.lw(p(r32(c, A0), 0x48));
        call_ptr(c, callee::CAREER_STATS_SELECT_PAGE, 0x001f_5608)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateBuyAttrib_OnActivate(State* s)` (0x1f49f8): activate the
/// state, then the selected attribute's "show" method (slot +0x38 of the
/// object at `+0xac`).
pub fn c_fe_state_buy_attrib_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::C_FE_STATE_ON_ACTIVATE.call(c, 0x001f_4a0c)?;
        let this = crate::abi::r32(c, S0);
        let obj = lw(c, this, 0xac);
        c.g[A2][0] = obj;
        c.g[A1][0] = 1;
        let vt = lw(c, obj as u32, 8);
        c.g[V0][0] = vt;
        let m = VSlot::read(c, vt, 0x38);
        c.g[A0][0] = m.delta;
        c.g[V1][0] = m.func;
        c.g[A0][0] = m.this(obj);
        crate::call::call_ptr(c, m.func as u32, 0x001f_4a28)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCareerStats_SetText(s, const char* name, const char* text)`
/// (0x1f6840): find the text object called `name` in the screen at +0x40 and
/// set it to `text`. 13 (+2) + 5 cycles plus the callees.
pub fn cfe_state_career_stats_set_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        c.g[A0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, callee::GET_HASH_VALUE32, 0x001f_6860)?;
        c.g[A0][0] = c.lw(p(self_, 0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, callee::UI_SCREEN_GET_OBJECT_BY_HASH_NAME, 0x001f_686c)?;
        let text = c.g[V0][0];
        c.g[A0][0] = text;
        if text != 0 {
            c.cycles += 2;
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, callee::UI_TEXT_SET_ASCII_STRING, 0x001f_687c)?;
        }
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cFEStateCareerStats_ctor(self)` (0x1f54b0): run the UI state ctor,
/// install the career stats class and copy the pad mask byte.
pub fn c_fe_state_career_stats_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[RA][0] = sx(0x001f_54c4);
        call_ptr(c, callee::UI_STATE_CTOR_2, 0x001f_54c4)?;
        c.g[V0][0] = 0x2a;
        c.g[V1][0] = 0x47_0000;
        Addr(p(r32(c, S0), 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3838));
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V1][0]);
        c.g[A1][0] = 0;
        Addr(p(r32(c, S0), 0x44)).sb(c, 0);
        c.g[V0][0] = Addr(r32(c, S0)).off(0x10).lw(c);
        c.g[RA][0] = sx(0x001f_54ec);
        c.g[A0][0] = Addr(r32(c, V0)).lw(c);
        call_ptr(c, callee::FE_MODULE_GET_PLAYER_PAD_MASK, 0x001f_54ec)?;
        Addr(p(r32(c, S0), 0x15)).sb(c, c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateBuyAttrib_OnWidgetNotify(State*, void*, int key)`
/// (0x001f4a38).
pub fn cfe_state_buy_attrib_on_widget_notify(c: &mut Cpu) {
    handle_list_key(c);
}

/// `int cFEStateCareerStats_OnWidgetNotify(State*, void*, int key)`
/// (0x001f5618).
pub fn cfe_state_career_stats_on_widget_notify(c: &mut Cpu) {
    handle_key_6_8_9(c);
}

// ---------------------------------------------------------------------------
// Front-end buy-attrib and event-select screens.

/// `void cFEStateBuyAttrib_UpdateTotalCost(cFEStateBuyAttrib* self)`
/// (0x1f5240): recompute the total cost label from the selected item.
pub fn c_fe_state_buy_attrib_update_total_cost(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        let self_ = c.g[A0][0];
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x46_f090;
        c.g[S1][0] = self_;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(0x0046_f090);
        call_ptr(c, 0x0031_7670, 0x001f_5260)?;
        c.g[A0][0] = Addr(at(c, S1, 0x48)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001f_526c)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 6;
            c.g[A0][0] = Addr(at(c, S1, 0xa0)).lw(c);
            call_ptr(c, 0x0019_8af0, 0x001f_5280)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_0c50, 0x001f_528c)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateBuyAttrib_UpdateMoney(cFEStateBuyAttrib* self)`
/// (0x1f52a0): refresh the money label from the player's balance.
pub fn c_fe_state_buy_attrib_update_money(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        let self_ = c.g[A0][0];
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x46_f0a0;
        c.g[S1][0] = self_;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(0x0046_f0a0);
        call_ptr(c, 0x0031_7670, 0x001f_52c0)?;
        c.g[A0][0] = Addr(at(c, S1, 0x48)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001f_52cc)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 6;
            c.g[A0][0] = Addr(at(c, S1, 0xa4)).lw(c);
            call_ptr(c, 0x0019_8af0, 0x001f_52e0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_0c50, 0x001f_52ec)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// UI widget event handlers.

/// `func_001F5650` (0x1f5650): `cFEStateCareerStats::OnWidgetAction`. For
/// widget action 6 only (action 5 and a null widget do nothing) it asks the
/// widget (`widget+0x18` through the owner's `vtable[0x28]`) whether it has a
/// selection; if it does, it forwards the event to `this->+0x10 + 0x18`
/// (`0x39f400`). Signature: `void on_widget_action(this, widget, action)`.
pub fn c_fe_state_career_stats_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let no_widget = c.g[V1][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        let mut tail_charge = true;
        if no_widget {
            // nothing to do
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x5;
            let five = c.g[A2][0] == c.g[V0][0];
            c.g[V0][0] = 0x6;
            if five {
                // nothing to do
            } else {
                c.cycles += 2;
                if c.g[A2][0] != c.g[V0][0] {
                    tail_charge = false;
                } else {
                    c.cycles += 11;
                    c.g[V0][0] = lw_r(c, S0, 0x10);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = lw_r(c, V1, 0x18);
                    c.g[A3][0] = Addr(low(c, V0)).lw(c);
                    c.g[V1][0] = lw_r(c, A3, 0x4);
                    c.g[A0][0] = lh_r(c, V1, 0x28);
                    c.g[V0][0] = lw_r(c, V1, 0x2c);
                    let target = low(c, V0);
                    c.g[RA][0] = sx(0x001f_56a0);
                    c.g[A0][0] = sx(low(c, A3).wrapping_add(low(c, A0)));
                    call_ptr(c, target, 0x001f_56a0)?;
                    let no_selection = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[V0][0];
                    if !no_selection {
                        c.cycles += 3;
                        c.g[A0][0] = lw_r(c, S0, 0x10);
                        c.g[RA][0] = sx(0x001f_56b4);
                        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x18));
                        call_ptr(c, 0x0039_f400, 0x001f_56b4)?;
                    }
                }
            }
        }
        if tail_charge {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEStateBuyAttrib_OnWidgetCreated` (0x001f4910).
//
// `int cFEStateBuyAttrib::OnWidgetCreated(self, UIWidget* w)`: install the
// created widget into the state's slots by its id.

pub fn fe_state_buy_attrib_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        'b_1f49bc: {
            'b_1f4964: {
                c.cycles += 13;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S2][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[S1][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[A0][0] = 0x470000;
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                c.g[RA][0] = sx(0x001f493c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xff8));
                call_ptr(c, 0x00317670, 0x001f493c)?;
                if c.g[S0][0] != c.g[V0][0] {
                    c.g[A0][0] = 0x470000;
                    break 'b_1f4964;
                } else {
                    c.cycles += 8;
                    c.g[A0][0] = 0x470000;
                    c.g[RA][0] = sx(0x001f4950);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xfe0));
                    call_ptr(c, 0x00317670, 0x001f4950)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001f495c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x001f495c)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        break 'b_1f49bc;
                    } else {
                        break 'b_1f4964;
                    }
                }
            }
            'b_1f49b8: {
                'b_1f4980: {
                    c.cycles += 5;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x001f4970);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xfc0));
                    call_ptr(c, 0x00317670, 0x001f4970)?;
                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x4a0000;
                        break 'b_1f4980;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.sw((c.g[S2][0] as u32).wrapping_add(0xac), c.g[S1][0]);
                        if t {
                            break 'b_1f49b8;
                        } else {
                            break 'b_1f4980;
                        }
                    }
                }
                'b_1f499c: {
                    c.cycles += 5;
                    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x001f498c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2320));
                    call_ptr(c, 0x00317670, 0x001f498c)?;
                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x4a0000;
                        break 'b_1f499c;
                    } else {
                        c.cycles += 2;
                        let t = true;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x18), 0u64);
                        if t {
                            break 'b_1f49b8;
                        } else {
                            break 'b_1f499c;
                        }
                    }
                }
                c.cycles += 5;
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
                c.g[RA][0] = sx(0x001f49a8);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2328));
                call_ptr(c, 0x00317670, 0x001f49a8)?;
                let t = c.g[S0][0] != c.g[V0][0];
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                if t {
                    break 'b_1f49bc;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0x1;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    break 'b_1f49b8;
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1f49bc;
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
    })
}

// ---------------------------------------------------------------------------
// Buy attributes screen.

/// `void cFEStateBuyAttrib_Init(self)` (0x1f4830): create the screen, fetch
/// the character's attributes and refresh every displayed value.
pub fn cfestatebuyattrib_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x470000;
        c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0x1008));
        c.g[S0][0] = Addr(p(r32(c, S1), 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x001f_4854)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001f_4868)?;
        let no_screen = c.g[V0][0] == 0;
        Addr(p(r32(c, S1), 0x48)).sw(c, c.g[V0][0]);
        if !no_screen {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001f_4880)?;
        }
        c.cycles += 35;
        call_ptr(c, 0x0014_dc80, 0x001f_4888)?;
        c.g[A1][0] = 0xb;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dd58, 0x001f_4894)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0;
        call_ptr(c, 0x0014_a080, 0x001f_48a0)?;
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0015_0928, 0x001f_48b0)?;
        Addr(p(r32(c, S1), 0xa8)).sw(c, 0);
        c.g[A0][0] = c.g[S1][0];
        Addr(p(r32(c, S1), 0xa0)).sw(c, 0);
        Addr(p(r32(c, S1), 0xa4)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x001f_5240, 0x001f_48c4)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x001f_4ea8, 0x001f_48cc)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x001f_4fc0, 0x001f_48d4)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x001f_50b0, 0x001f_48dc)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x001f_52a0, 0x001f_48e4)?;
        call_ptr(c, 0x0028_b180, 0x001f_48ec)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 3;
        call_ptr(c, 0x0028_f140, 0x001f_48f8)?;
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
    use crate::difftest::elf_image;
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
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::SP;
    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear_3;
    pub(crate) use crate::game::bx::bxstring2::tests::be_library;
    pub(crate) use crate::game::dirtysock::tags2::tests::alloc_ready;
    pub(crate) use crate::game::dirtysock::tags2::tests::base;
    pub(crate) use crate::game::dirtysock::tags2::tests::clear as clear_2;
    pub(crate) use crate::game::dirtysock::tags2::tests::common;
    pub(crate) use crate::game::dirtysock::tags2::tests::dummy_world;
    pub(crate) use crate::game::dirtysock::tags2::tests::gp_at;
    pub(crate) use crate::game::dirtysock::tags2::tests::load;
    pub(crate) use crate::game::dirtysock::tags2::tests::SEEDS_7 as SEEDS;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::clear;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_2 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::h;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_3 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_3;
    pub(crate) use crate::game::fe::fememcard::tests::vtable_at;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_2;
    pub(crate) use crate::game::sound::soundsys::tests::globals_7 as globals_2;
    pub(crate) use crate::game::sound::soundsys::tests::load_2;
    pub(crate) use crate::game::sound::soundsys::tests::pool_7 as pool;
    pub(crate) use crate::game::sound::soundsys::tests::BE;
    pub(crate) use crate::game::sound::soundsys::tests::WORLD;
    pub(crate) use crate::game::world::worldview::tests::SEEDS_5 as SEEDS_2;
    use ssx3_ee as ee;

    #[test]
    fn fe_state_buy_attrib_transition_in_matches() {
        crate::difftest::check(
            ee::s04::f_001f49d0__generated,
            fe_state_buy_attrib_transition_in,
            40,
            &|c, seed| {
                machine(c, seed, 0x1f49_d0, 2);
                be_library(c);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn fe_state_career_stats_transition_in_matches() {
        crate::difftest::check(
            ee::s04::f_001f55c0__generated,
            fe_state_career_stats_transition_in,
            40,
            &|c, seed| {
                machine(c, seed, 0x1f55_c0, 2);
                be_library(c);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_buy_attrib_on_child_event_matches() {
        check(
            ee::s04::f_001f4a60__generated,
            c_fe_state_buy_attrib_on_child_event,
            60,
            &|c, seed| {
                machine(c, seed, 0x1f4a60, 6);
                globals(c);
                clear(c, DATA, 0x800);
                c.g[A0][0] = DATA as u64;
                let sender = DATA + 0x400;
                c.g[A1][0] = sender as u64;
                // The purchase confirmation message (0x16) with and without a
                // pending purchase, and another message.
                c.g[A2][0] = match seed % 3 {
                    0 => 0x15,
                    1 => 0x16,
                    _ => 0x16,
                };
                c.bus
                    .write32(sender + 0x6c, if seed % 2 == 0 { 0 } else { 1 });
                c.g[A3][0] = 0;
            },
        );
    }

    #[test]
    fn c_fe_state_career_stats_on_activate_matches() {
        check(
            ee::s04::f_001f55e8__generated,
            c_fe_state_career_stats_on_activate,
            20,
            &|c, seed| {
                machine(c, seed, 0x1f55e8, 8);
                globals(c);
                clear(c, DATA, 0x400);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed & 1) as u64;
                // The page to select (`self+0x48`); page 0 is the default and
                // keeps `cFEStateCareerStats_SelectPage` on its first path.
                c.bus.write32(DATA + 0x48, 0);
            },
        );
    }

    #[test]
    fn c_fe_state_buy_attrib_on_activate_matches() {
        check(
            ssx3_ee::s04::f_001f49f8__generated,
            c_fe_state_buy_attrib_on_activate,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x27, 1);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0xac, DATA + 0x3000);
                vtable_at(c, &mut r, DATA + 0x3000, 0x38);
            },
        );
    }

    #[test]
    fn cfe_state_career_stats_set_text_matches() {
        check(
            ee::s04::f_001f6840__generated,
            cfe_state_career_stats_set_text,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x1f68);
                let s = DATA;
                let screen = DATA + 0x400;
                let name = DATA + 0x1000;
                let n = (r.next_u32() % 12) as usize;
                for k in 0..n {
                    c.bus
                        .write8(name + k as u32, b'A' + ((seed as usize + k) % 26) as u8);
                }
                c.bus.write8(name + n as u32, 0);
                fill(c, &mut Rng(0x1f68_0000 ^ seed as u64), screen, 4);
                c.bus.write32(s + 0x40, screen);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = sx(name);
                c.g[A2][0] = sx(DATA + 0x2000);
            },
        );
    }

    #[test]
    fn c_fe_state_career_stats_ctor_matches() {
        check(
            ee::s04::f_001f54b0__generated,
            c_fe_state_career_stats_ctor,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x1f54b0, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear_2(c, this, 0x400);
                let module = DATA + 0x2000;
                clear_2(c, module, 0x40);
                c.bus.write32(this + 0x10, module);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn cfe_state_career_stats_on_widget_notify_matches() {
        check(
            ssx3_ee::s04::f_001f5618__generated,
            cfe_state_career_stats_on_widget_notify,
            SEEDS_2,
            &|c, seed| {
                let _ = machine(c, seed, 0x1f5618, 0);
                c.g[A2][0] = (seed % 13) as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_career_stats_on_widget_action_matches() {
        check(
            ee::s04::f_001f5650__generated,
            c_fe_state_career_stats_on_widget_action,
            SEEDS_3,
            &|c, seed| {
                machine(c, seed, 0x1f5650, 4);
                be_library_2(c);
                world(c);
                heap_init(c);
                c.g[A0][0] = DATA as u64;
                let widget = DATA + 0x400;
                c.g[A1][0] = match seed % 3 {
                    0 => 0,
                    _ => widget as u64,
                };
                c.g[A2][0] = [5, 6, 7][(seed % 3) as usize] as u64;
                w(c, widget + 0x18, 0x10);
                let list = DATA + 0x600;
                w(c, DATA + 0x10, list);
                let owner = DATA + 0x800;
                w(c, list, owner);
                let vt = DATA + 0x2000;
                w(c, owner + 4, vt);
                slot(c, vt, 0x28, 0, FAKE);
            },
        );
    }

    #[test]
    fn fe_state_buy_attrib_on_widget_created_matches() {
        strict(
            ee::s04::f_001f4910__generated,
            fe_state_buy_attrib_on_widget_created,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1f49, 4);
                let this = DATA;
                let widget = DATA + 0x1000;
                c.bus
                    .write32(widget + 0x38, if seed % 4 == 0 { 0 } else { r.next_u32() });
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(widget);
            },
        );
    }

    #[test]
    fn cfestatebuyattrib_init_matches() {
        check(
            ee::s04::f_001f4830__generated,
            cfestatebuyattrib_init,
            6,
            &|c, seed| {
                let _ = base_2(c, seed, 0x1f4830, 3);
                let this = DATA;
                clear_3(c, this, 0x800);
                c.g[A0][0] = this as u64;
                w(c, this + 0x10, DATA + 0x400);
                w(c, this + 0x44, (seed % 3) as u32);
            },
        );
    }
}
