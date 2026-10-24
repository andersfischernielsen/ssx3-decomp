//! `src/fe/festatecharequip2.cpp`: hand-written replacements (moved from the
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
pub(crate) use crate::game::ai::rider::world;
pub(crate) use crate::game::animation::rideranimbase::sw;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw as lw_4;
pub(crate) use crate::game::dirtysock::tags2::p;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_2;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw_3;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_2;
pub(crate) use crate::game::fe::festaterewards::at_4 as at_2;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw_2;
pub(crate) use crate::game::fe::ovstatepause::lw;
pub(crate) use crate::game::movie::movieplayer::at;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::scripter::ssxscriptengine::gp;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use callee::*;
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
        0x0019_a238,
        cfe_state_char_equip_show_equip_mode,
        "cFEStateCharEquip_ShowEquipMode",
    ),
    (
        0x0019_a308,
        cfe_state_char_equip_show_buy_mode,
        "cFEStateCharEquip_ShowBuyMode",
    ),
    (
        0x0019_a3d0,
        cfe_state_char_equip_toggle_buy_equip,
        "cFEStateCharEquip_ToggleBuyEquip",
    ),
    (
        0x0019_a498,
        c_fe_state_char_equip_release_preview,
        "cFEStateCharEquip_ReleasePreview",
    ),
    (
        0x0019_a9b8,
        cfe_state_char_equip_refresh_item_info,
        "cFEStateCharEquip_RefreshItemInfo",
    ),
    (
        0x0019_aca0,
        c_fe_state_char_equip_refresh_rider_bolts,
        "cFEStateCharEquip_RefreshRiderBolts",
    ),
    (
        0x0019_b458,
        cfe_state_char_equip_remove_list_item,
        "cFEStateCharEquip_RemoveListItem",
    ),
    (
        0x0019_b518,
        c_fe_state_char_equip_fill_equip_rows,
        "cFEStateCharEquip_FillEquipRows",
    ),
    (
        0x0019_b598,
        c_fe_state_char_equip_fill_buy_rows,
        "cFEStateCharEquip_FillBuyRows",
    ),
    (
        0x0019_bba0,
        c_fe_state_char_equip_update_money_text,
        "cFEStateCharEquip_UpdateMoneyText",
    ),
    (
        0x0019_bbe0,
        cfe_state_char_equip_set_category_title,
        "cFEStateCharEquip_SetCategoryTitle",
    ),
    (
        0x0019_bc90,
        cfe_state_char_equip_update_stat_bar,
        "cFEStateCharEquip_UpdateStatBar",
    ),
    (
        0x0019_c7e8,
        cfe_state_char_equip_open_buy_popup,
        "cFEStateCharEquip_OpenBuyPopup",
    ),
    (
        0x0019_c8f8,
        bolt_file_index_manager_get_next_node,
        "BoltFileIndexManager_GetNextNode",
    ),
    (
        0x0019_cb60,
        bolt_file_index_manager_reset,
        "BoltFileIndexManager_Reset",
    ),
    (0x0019_ce68, boltfile_cache_init, "BoltFile_CacheInit"),
    (
        0x0019_cf40,
        bolt_file_cache_shutdown,
        "BoltFile_CacheShutdown",
    ),
    (0x0019_e238, bolt_file_is_loaded, "BoltFile_IsLoaded"),
    (
        0x0019_e538,
        femodule_rider_preview_set_visible,
        "FEModule_RiderPreviewSetVisible",
    ),
];

/// Same-shard direct callees used above.
pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::ai::rider::callee::BENEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::ai::rider::callee::BE_GET_RIDER_PROFILE_2 as BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::ai::rider::callee::BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::ai::rider::callee::BUDDY_MAN_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::ai::rider::callee::BUY_POPUP_INIT_BUY_BOLT;
    pub(crate) use crate::game::ai::rider::callee::CODEC_POP_FREE_FRAME;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_ALL_FIELDS_FILLED;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_BACKUP_ACCOUNT;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_ACCOUNT_NAME;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_CONFIRM_PASSWORD;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_EMAIL;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_INFORM;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_PASSWORD;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_SHARE_INFO;
    pub(crate) use crate::game::ai::rider::callee::CROWD_MAN_GROUP_FREE;
    pub(crate) use crate::game::ai::rider::callee::GAME_UI_OPEN_SCREEN;
    pub(crate) use crate::game::ai::rider::callee::IN_SEQ_PLAY;
    pub(crate) use crate::game::ai::rider::callee::LOBBY_MAN_GET_LANGUAGE;
    pub(crate) use crate::game::ai::rider::callee::LOBBY_MAN_SEND_REQUEST;
    pub(crate) use crate::game::ai::rider::callee::LOGIN_ON_ACCOUNT_MEM_POPUP;
    pub(crate) use crate::game::ai::rider::callee::LOGIN_ON_DISCONNECT_POPUP;
    pub(crate) use crate::game::ai::rider::callee::MESSAGE_CENTER_GET_LIST_INDEX;
    pub(crate) use crate::game::ai::rider::callee::MESSAGE_CENTER_OPEN_VIEW_POPUP;
    pub(crate) use crate::game::ai::rider::callee::MPEG2_GET_BITS;
    pub(crate) use crate::game::ai::rider::callee::MPEG2_MARKER_BIT;
    pub(crate) use crate::game::ai::rider::callee::NET_ACCOUNT_MEM_CTOR;
    pub(crate) use crate::game::ai::rider::callee::RAIL_MAN_ENABLE_RAIL;
    pub(crate) use crate::game::ai::rider::callee::REPLAY_FREE_SNAPSHOT;
    pub(crate) use crate::game::ai::rider::callee::RIDER_GET_START_DIST_TO_FINISH;
    pub(crate) use crate::game::ai::rider::callee::SCRIPT_ENGINE_GET_PROCESS as SCRIPT_ENGINE_GET_PROCESS_2;
    pub(crate) use crate::game::ai::rider::callee::SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::ai::rider::callee::SCRIPT_OBJ_DEACTIVATE;
    pub(crate) use crate::game::ai::rider::callee::SHOW_CHAR_STATS;
    pub(crate) use crate::game::ai::rider::callee::SHOW_CHECKING_CARD_POPUP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_GET_SURFACE_SOUND_TYPE;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_PLAY_FE_SOUND;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_BASE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_EDGE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_SURFACE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_TRICK_METER_SOUND;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_WIPEOUT_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_WIPEOUT_LOOP2;
    pub(crate) use crate::game::ai::rider::callee::START_CARD_SCAN;
    pub(crate) use crate::game::ai::rider::callee::STREAMER_GET_SECTION_COUNT;
    pub(crate) use crate::game::ai::rider::callee::SYNC_PROCESS_DTOR;
    pub(crate) use crate::game::ai::rider::callee::TEX_MAN_EVICT_TEXTURE;
    pub(crate) use crate::game::ai::rider::callee::VRAM_HEAP_FREE_TEXTURE_BLOCKS;
    pub(crate) use crate::game::ai::rider::callee::WORLD_CACHE_LOAD;
    pub(crate) use crate::game::ai::rider::callee::WORLD_CACHE_RESET;
    pub(crate) use crate::game::ai::rider::callee::WSCRIPT_CACHE_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_APPLY_OUTPUT_MODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_SET_OUTPUT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_BE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_INTERFACE_CALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOLT_FILE_INDEX_GET_NEXT_NODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAM_CONTROLLER_SET_ALGORITHM;
    pub(crate) use crate::game::dirtysock::tags2::callee::CBXSTRING_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_REFRESH_BUY_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_REFRESH_EQUIP_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAR_EQUIP_UPDATE_HELP_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::CROWD_MAN2D_INIT_SLOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::DYNAMIC_EMITTER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::DYNAMIC_EMITTER_FREE_DATA;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE_3 as EA_MEM_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::FEMODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_STATE_ON_ACTIVATE_2 as FE_STATE_ON_ACTIVATE;
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
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DISCONNECT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMITTER_FREE_DATA;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMORY_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_MESSAGE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_HANDLE_NET_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_MESSENGER_SET_PLAYER_MENU_SELECTION;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::RCMP_DECODE_FRAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::RCMP_FRAME_NUMBER;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_HIGHLIGHT_SLOT_CLEAR_2 as REPLAY_HIGHLIGHT_SLOT_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_REFRESH_LIST;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_UPDATE_HELP_TEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REQUEST_LINE_UPDATE_NOW_PLAYING;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_DATA_GET_INFO_2 as SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT_2 as SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_2 as SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MOVIE_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SETUP_MAN_SEND_CHARACTER_MSG;
    pub(crate) use crate::game::dirtysock::tags2::callee::SOUND_PLAYER_SET_BY_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::SOUND_PLAYER_START;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY as STRCPY_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_MENU_SET_SELECTED_BY_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SET_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_WIDGET_SLOT_0;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_WIDGET_SLOT_1;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_INSTANCE_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_KILL_TIMERS_BY_CALLBACK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INSTANCE_4 as BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INTERFACE_4 as BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_MISSION_INT_GET_SESSION_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_HUMAN_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_RACE_INT_GET_SPECIAL_EVENT_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_RACE_INT_GET_TRACK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_GET_NEXT_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_GET_PREV_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_NODE_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUILD_ITEM_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_BX_STRING_ASSIGN_CSTR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_BX_STRING_CTOR_CSTR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_COMM_SYSTEM_CHANNEL_RECV;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_COMM_SYSTEM_CHANNEL_SEND;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_CUT_SCENE_TRANSITION_MAN_SAVE_VIEWPORT;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::C_SETUP_MAN_ALLOC_PACKET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_SETUP_MAN_FREE_PACKET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_TRANSITION_MAN_START;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_TRANSITION_MAN_START_CAPTURED;
    pub(crate) use crate::game::fe::feasyncfile::callee::FEMODULE_GET_RIDER_PREVIEW_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FEMODULE_RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ASYNC_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_PLAYER_PAD_MASK_2 as FE_MODULE_GET_PLAYER_PAD_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_RELEASE_BOLT_FILES;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_UNLOAD_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_ADVANCE_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_QUIT_TO_LOAD_SCREEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUTSYNC_IS_READY;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_BINDING_EVAL_FLOAT;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_SPEED;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_USER_DATA_2 as IN_SEQ_INSTANCE_SET_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_START_SEQUENCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::ISEQ_HEAP_ALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::ISEQ_SPAN_TRACK_ACTIVATE_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::ISEQ_SPAN_TRACK_NEXT_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_FIND_ENABLED_FROM_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_IS_KEY_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_POPUP_FIND_ENABLED_IN_COLUMN_FROM_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_INDICATOR_ADVANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_CONNECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_TABLE_FIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_CALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_PASS_THROUGH;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_AS_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR_2 as LUNO_VM_REGISTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_FORMAT_SLOT_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC_NAMED_128;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_GET_INPUT_CODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_HANDLE_INPUT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_IS_DISABLED_3 as MENU_SYS_ITEM_IS_DISABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_EXTENSION_AND_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_ACCOUNT_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_DTOR_5 as NET_FE_STATE_DTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GOTO_NEXT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_5 as NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED_4 as OPERATOR_NEW_NAMED_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_DATA_FIND_MARKER;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_FIND_BANK_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_GRID_MESH_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::REFRESH_ITEM_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_BASE_RESET_FX;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_GET_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_GET_START_POSE_ANGLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_IS_LOADED;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_METRIX_SHOW_HUD_MSG_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_PLACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SCRIPT_OBJ_MATCHES_RIDER_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_3 as SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_TRANSITION_TARGET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_BINDING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_TICK_PROCESS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_RIDER_GET_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SET_CATEGORY_TITLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SHOW_BUY_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SHOW_EQUIP_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_UNLOAD_BANK_2 as SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK_2 as SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPCH_MAKE_ID_2 as SPCH_MAKE_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_SYSTEM_IS_INSTANCE_READY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPLINE_GET_PATH_LENGTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_CHARACTER_TO_SPEECH_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_FADE_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_IS_SPEECH_VOLUME_ON;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_SET_FADE_GAIN;
    pub(crate) use crate::game::fe::feasyncfile::callee::START_SCREEN_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_MENU_SET_SELECTION_3 as UI_MENU_SET_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_5 as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_CTOR_2_4 as UI_STATE_CTOR_2_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_ALL_POPUPS_2 as UI_STATE_MGR_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_SWITCH_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_GET_ID_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_GET_OWNED_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_SET_TEXT_ID_2 as UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_LEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_SPRINTF_ASCII_FMT;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_TO_ASCII;
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
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_ON_ACTIVATE_2 as UI_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_POST_INPUT;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_RENDER;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_UPDATE_2 as UI_STATE_UPDATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXTURE_SET_FIND;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::fe::ovstatepause::callee::VID_ENGINE_PLAYER_RENDER;
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
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::GET_HASH_VALUE_32;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN_DISCONNECT as LOBBY_MAN_DISCONNECT_2;
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
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_CTOR_5 as NET_FE_STATE_CTOR_2;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_HANDLE_NET_EVENT as NET_FE_STATE_HANDLE_NET_EVENT_2;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::movie::movieplayer::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PDA_TEMPLATE_OPEN;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PROFILE_SAVE_PDA_INIT_SLOT_LABELS;
    pub(crate) use crate::game::movie::movieplayer::callee::PARENT_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_ON_SPLIT_MARKER;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_RESET_STATS;
    pub(crate) use crate::game::movie::movieplayer::callee::ROLLER_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_CMD_QUEUE_CLEAR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_CMD_QUEUE_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_DATA_GET_INFO as SCRIPT_DATA_GET_INFO_2;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_FIND_SOUND_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT as SCRIPT_ENGINE_GET_INSTANCE_SCRIPT_2;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_RUN_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_UNLOAD_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::SPRINTF_2 as SPRINTF;
    pub(crate) use crate::game::movie::movieplayer::callee::STRCPY_2 as STRCPY_3;
    pub(crate) use crate::game::movie::movieplayer::callee::STREAM_BUFFER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::TAG_FIELD_GET_STRING;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_LOAD_TEXTURES;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_STORE_WORD;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_CTOR_2;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_MGR_ADD_BACKGROUND_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESSES;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESS_COUNT;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_MAN_CREATE_PROCESS;

    pub const BOLT_FILE_RELEASE: Callee = Callee {
        addr: 0x0019_dc20,
        f: ee::s03::f_0019dc20,
    };
}

/// Addresses the generator emitted as `call_indirect` (outside the shard's
/// own direct-call set, so no readable alias is needed here).
pub(crate) mod addr {
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
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MEM_MAN_ALLOC;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_DTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_POP_PAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_PUSH_PAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::POSITION_MODIFIER_CTOR_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::RIDER_PREVIEW;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::RIDER_PREVIEW_BOLTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::SPATIAL_INSERT;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::SPRINTF;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCASECMP;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCMP;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRLEN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STR_HASH;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_ENGINE_CREATE_SCREEN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_ENGINE_TEARDOWN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_OBJECT_BY_HASH;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_SCREEN_START_THREAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::WORLD_GET;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::WORLD_PAINTER_2D;
}

/// `GetHashValue32` (0x317670) and the UI object lookups, which the generator
/// emitted as cross-shard `call_indirect`s.
pub(crate) mod table_callee {
    pub(crate) use crate::game::fe::festaterewards::table_callee::AI_SET_AI_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::APP_MAN_SET_NEXT_MODULE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BENEW_PLAYER_INT_GET_CHAR_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET_INTERFACE_FV;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET_TEXT_FROM_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_SET_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BUDDY_PRODUCT_COMPARE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BUY_POPUP_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BXSTRING_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BXSTRING_DTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::CODEC_DECODE_FRAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_MODULE_BIG_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_MODULE_SMALL_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::fe::festaterewards::table_callee::MUSIC_SLOT_RELEASE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::OPERATOR_DELETE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_CHEAT_CHARS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_CONCEPT_ARTS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_GET_BASE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_POSTERS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_TOYS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_TRADING_CARDS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_VIDEOS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_REMOVE_SCRIPT_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_SCRIPT_CLEANUP;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_SET_SCRIPT_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::SECTION_TABLE_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::SSX_AUDIO_GET_RACE_MAN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::STRCPY;
    pub(crate) use crate::game::fe::festaterewards::table_callee::STRLEN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_FIND;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_GET_STRING4;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_ENGINE_GET_INTERFACE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_MENU_SET_SELECTED_BY_INDEX;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_OBJECT_PUSH;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_OPEN_FINISH_SCREEN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_ADD_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_STATE_DTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_RICH_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UPLOAD_TEXTURE_PIXELS;
}

// ---------------------------------------------------------------------------
// Bolt file stream (front-end file manager).

/// `bool BoltFile_IsLoaded(BoltFile *file, int outer, int inner)` (0x19e238):
/// reads the word at `file + outer*0x12000 + inner*0x120` and returns whether
/// it equals 1.
pub fn bolt_file_is_loaded(c: &mut Cpu) {
    c.cycles += 11;
    c.g[V0][0] = 0x10000;
    c.g[V1][0] = 0x120;
    c.g[V0][0] = c.g[V0][0] | 0x2000; // 0x12000: stride of the outer table
    c.mult(0, A2, A2, V1); // inner *= 0x120
    c.mult(0, A1, A1, V0); // outer *= 0x12000
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
    c.g[V0][0] = c.g[V0][0] ^ 0x1;
    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Front-end module: rider previews.

/// `void FEModule_RiderPreviewSetVisible(FEModule *self, uint visible)`
/// (0x19e538): when `visible` is zero clears the +0xcc4/+0xcc8 flags; otherwise
/// sets +0xcc4 to 1 if any of the +0xcc0/+0xcb8/+0xcb4 words is zero or +0xcd4
/// is non-negative, else sets +0xcc8 to 1.
pub fn femodule_rider_preview_set_visible(c: &mut Cpu) {
    c.cycles += 2;
    let this = Addr(abi::arg(c, 0));
    if abi::arg64(c, 1) == 0 {
        this.off(0xcc4).sw(c, 0);
        c.cycles += 2;
        this.off(0xcc8).sw(c, 0);
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    let mut early;
    c.g[V0][0] = this.off(0xcc0).lw(c);
    early = c.g[V0][0] == 0;
    c.g[V0][0] = 0x1;
    if !early {
        c.cycles += 3;
        c.g[V0][0] = this.off(0xcb8).lw(c);
        early = c.g[V0][0] == 0;
        c.g[V0][0] = 0x1;
        if !early {
            c.cycles += 3;
            c.g[V0][0] = this.off(0xcb4).lw(c);
            early = c.g[V0][0] == 0;
            c.g[V0][0] = 0x1;
            if !early {
                c.cycles += 3;
                c.g[V0][0] = this.off(0xcd4).lw(c);
                early = (c.g[V0][0] as i64) >= 0;
                c.g[V0][0] = 0x1;
                if !early {
                    c.cycles += 2;
                    this.off(0xcc8).sw(c, c.g[V0][0]);
                    abi::ret(c);
                    return;
                }
            }
        }
    }
    c.cycles += 2;
    this.off(0xcc4).sw(c, c.g[V0][0]);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Bolt file index manager.

/// Shared body of `BoltFileIndexManager_GetPrev/NextNode` (0x19c8d0/0x19c8f8):
/// follow the 16-bit node link at `node + link` and return `base + index*16`.
pub(crate) fn bolt_file_index_manager_get_node(c: &mut Cpu, link: u32) {
    c.cycles += 2;
    let node = abi::arg64(c, 1);
    if node != 0 {
        c.cycles += 3;
        let index = Addr(node as u32).off(link).lh(c);
        c.g[V0][0] = index;
        let negative = (index as i64) < 0;
        let scaled = sx((index as u32) << 4);
        c.g[V0][0] = scaled;
        if !negative {
            c.cycles += 2;
            c.g[V0][0] = sx(abi::arg(c, 0).wrapping_add(scaled as u32));
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `Node* BoltFileIndexManager_GetNextNode(Node *base, Node *node)` (0x19c8f8).
pub fn bolt_file_index_manager_get_next_node(c: &mut Cpu) {
    bolt_file_index_manager_get_node(c, 0xe);
}

/// `void cFEStateCharEquip_UpdateMoneyText(State* s)` (0x19bba0): the same
/// with the text at `+0x64` and the value at `+0xc0`.
pub fn c_fe_state_char_equip_update_money_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = crate::abi::r32(c, S0);
        c.g[V0][0] = lw(c, this, 0x64);
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 6;
            c.g[A0][0] = lw(c, this, 0xc0);
            callee::FE_FORMAT_MONEY.call(c, 0x0019_bbc4)?;
            let this = crate::abi::r32(c, S0);
            c.g[A0][0] = lw(c, this, 0x64);
            c.g[A1][0] = c.g[V0][0];
            callee::UI_TEXT_SET_STRING.call(c, 0x0019_bbd0)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCharEquip_ReleasePreview(self)` (0x19a498): release the
/// previewed equipment's bolt file (if a slot is held) and clear the preview
/// state.
pub fn c_fe_state_char_equip_release_preview(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = c.g[S0][0] as u32;
        c.g[A2][0] = Addr(this).off(0xa6c).lw(c);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        if (c.g[A2][0] as i64) >= 0 {
            c.cycles += 4;
            c.g[A1][0] = Addr(this).off(0xbc).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x7c).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1a70));
            BOLT_FILE_RELEASE.call(c, 0x0019_a4c4)?;
        }
        c.cycles += 8;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        Addr(this).off(0x960).sb(c, 0);
        Addr(this).off(0x95c).sw(c, c.g[V0][0]);
        Addr(this).off(0xa6c).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCharEquip_RefreshItemInfo(cFEStateCharEquipDetail* s)`
/// (0x19a9b8): refresh the buy panel when the detail flags word at +0xc8 is
/// 1, else the equip panel, then the help text. 8 (+4, +2) + 6 cycles plus
/// the callees.
pub fn cfe_state_char_equip_refresh_item_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw(p(self_, 0xc8));
        c.g[V0][0] ^= 1;
        if c.g[V0][0] == 0 {
            c.cycles += 4;
            callee::CHAR_EQUIP_REFRESH_BUY_INFO.call(c, 0x0019_a9e0)?;
        } else {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            callee::CHAR_EQUIP_REFRESH_EQUIP_INFO.call(c, 0x0019_a9f0)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        callee::CHAR_EQUIP_UPDATE_HELP_TEXT.call(c, 0x0019_a9f8)?;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Riders, cameras, memory cards.

/// `cFEStateCharEquip_RefreshRiderBolts` (0x19ACA0): ask the char-equip state's
/// rider preview for the rider's currently equipped bolts.
pub fn c_fe_state_char_equip_refresh_rider_bolts(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x7c)).lw(c);
        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(0x44)).lb(c);
        c.g[A0][0] = low(c, A0).wrapping_add(0xb0);
        call_ptr(c, addr::RIDER_PREVIEW, 0x0019_acc8)?;
        c.g[S1][0] = c.g[V0][0];
        call_ptr(c, addr::BE_GET_BE, 0x0019_acd0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 9;
        call_ptr(c, addr::BE_GET_INTERFACE, 0x0019_acdc)?;
        c.g[A2][0] = lw_at(c, S0, 0xbc);
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(0x44)).lb(c);
        call_ptr(c, addr::BE_GET_INTERFACE_9, 0x0019_acec)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = 0;
        c.g[A3][0] = 1;
        call_ptr(c, addr::RIDER_PREVIEW_BOLTS, 0x0019_ad00)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCharEquip_FillEquipRows(self)` (0x0019b518): fill up to six
/// equip rows for the selected profile.
pub fn c_fe_state_char_equip_fill_equip_rows(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(at(c, S2, 0x5c)).lw(c);
        c.g[V0][0] = Addr(at(c, S2, 0x958)).lw(c);
        c.g[S0][0] = Addr(at(c, V1, 0x98)).lbu(c);
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let none = c.g[V0][0] == 0;
        c.g[S1][0] = 0;
        if !none {
            c.cycles += 2;
            c.g[A1][0] = c.g[S1][0];
            loop {
                c.cycles += 6;
                c.g[A0][0] = c.g[S2][0];
                callee::C_FE_STATE_CHAR_EQUIP_SET_EQUIP_ROW.call(c, 0x0019_b558)?;
                c.g[S1][0] = sx(word(c, S1).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < 6i64) as u64;
                let six = c.g[V0][0] == 0;
                c.g[S0][0] = sx(word(c, S0).wrapping_add(1));
                if six {
                    break;
                }
                c.cycles += 4;
                c.g[V0][0] = Addr(at(c, S2, 0x958)).lw(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = c.g[S1][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0019_b550);
                }
            }
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

/// `void cFEStateCharEquip_FillBuyRows(self)` (0x0019b598): fill up to six
/// buy rows for the selected profile.
pub fn c_fe_state_char_equip_fill_buy_rows(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(at(c, S2, 0x5c)).lw(c);
        c.g[V0][0] = Addr(at(c, S2, 0x958)).lw(c);
        c.g[S0][0] = Addr(at(c, V1, 0x98)).lbu(c);
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let none = c.g[V0][0] == 0;
        c.g[S1][0] = 0;
        if !none {
            c.cycles += 2;
            c.g[A1][0] = c.g[S1][0];
            loop {
                c.cycles += 6;
                c.g[A0][0] = c.g[S2][0];
                callee::C_FE_STATE_CHAR_EQUIP_SET_BUY_ROW.call(c, 0x0019_b5d8)?;
                c.g[S1][0] = sx(word(c, S1).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < 6i64) as u64;
                let six = c.g[V0][0] == 0;
                c.g[S0][0] = sx(word(c, S0).wrapping_add(1));
                if six {
                    break;
                }
                c.cycles += 4;
                c.g[V0][0] = Addr(at(c, S2, 0x958)).lw(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = c.g[S1][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0019_b5d0);
                }
            }
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

/// `void BoltFileIndexManager_Reset(BoltFileIndexManager* m)` (0x0019cb60):
/// clear the index and the tail table, point every one of the 400 entries at
/// the list head and mark the table's end with -1.
pub fn bolt_file_index_manager_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        c.g[A1][0] = sx(0xffff_ffff);
        c.sq(S0, sp(c).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp(c), c.g[RA][0]);
        c.g[A2][0] = 0x1900;
        call_ptr(c, 0x003e_6448, 0x0019_cb80)?; // EA_MemSet
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1900));
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x640;
        call_ptr(c, 0x003e_6448, 0x0019_cb90)?; // EA_MemSet
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1f40), c.g[S0][0]);
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S0][0];
        loop {
            c.cycles += 10;
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            c.sh((c.g[A0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
            c.g[A1][0] = c.g[V1][0];
            c.sh((c.g[A0][0] as u32).wrapping_add(0xe), c.g[V1][0]);
            c.g[V0][0] = ((c.g[A1][0] as i64) < 400) as u64;
            c.sh((c.g[A0][0] as u32).wrapping_add(4), 0);
            c.sh((c.g[A0][0] as u32).wrapping_add(6), 0);
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
            if !more {
                c.cycles += 6;
                c.g[V0][0] = sx(0xffff_ffff);
                c.g[RA][0] = c.ld(sp(c));
                c.sh((c.g[S0][0] as u32).wrapping_add(0x18fe), c.g[V0][0]);
                c.lq(S0, sp(c).wrapping_add(0x10));
                c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
                abi::ret(c);
                return Ok(());
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_cba0);
            }
        }
    })
}

/// `void cFEStateCharEquip_OpenBuyPopup(self, item)` (0x19c7e8): allocate a buy
/// popup, fill it from the item's price/id and post it to the screen.
pub fn cfe_state_char_equip_open_buy_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x30);
        c.g[A3][0] = 0;
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = 0x460000;
        frame.save(c, S2, 0x10);
        c.g[A0][0] = 0x70;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x790));
        call_ptr(c, table_callee::EA_MEM_ALLOC_NAMED, 0x0019_c81c)?;
        c.g[A1][0] = lw_2(c, S1, 0x10);
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = Addr(c.g[S1][0] as u32).off(0x44).lb(c);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, table_callee::BUY_POPUP_CTOR, 0x0019_c830)?;
        c.g[A2][0] = Addr(c.g[S0][0] as u32).off(0xe).lh(c);
        c.g[S2][0] = c.g[V0][0];
        let no_price = (c.g[A2][0] as i64) <= 0;
        c.g[A1][0] = lw_2(c, S0, 0x14);
        if !no_price {
            c.cycles += 2;
            c.g[V0][0] = 0xa;
            c.mult(0, 6, 6, 2);
        }
        c.cycles += 13;
        c.g[A3][0] = lw_2(c, S1, 0xc0);
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x48));
        callee::BUY_POPUP_INIT_BUY_BOLT.call(c, 0x0019_c854)?;
        c.g[A0][0] = lw_2(c, S1, 0x10);
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, table_callee::UI_OBJECT_PUSH, 0x0019_c864)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCharEquip_SetCategoryTitle(self, int id)` (0x19bbe0): write
/// the category's localised title into the screen's text widget; `id` picks a
/// table entry by hash, with a special case for the "default" category id.
pub fn cfe_state_char_equip_set_category_title(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[V0][0] = Addr(p(this as u32, 0x58)).lw(c);
        let no_text = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if no_text {
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r32(c, GP), 0xffff_e0e0)).lw(c);
            let default_id = c.g[S1][0] != c.g[V0][0];
            c.g[A0][0] = 0x460000;
            if !default_id {
                // the "default" category's own name
                c.cycles += 7;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x9b0));
                call_ptr(c, 0x0031_7670, 0x0019_bc18)?;
                c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x58)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_1000, 0x0019_bc24)?;
                frame.restore(c, S0, 0x30);
            } else {
                // look `id` up in the BE entry table and hash its name
                c.cycles += 10;
                call_ptr(c, 0x0014_bdb8, 0x0019_bc34)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0xbc)).lw(c);
                c.g[A0][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S1][0];
                call_ptr(c, 0x0014_d448, 0x0019_bc48)?;
                c.g[A0][0] = c.g[V0][0];
                let found = (c.g[A0][0] as i64) >= 0;
                c.g[V0][0] = 0;
                if found {
                    c.cycles += 4;
                    c.g[V0][0] = 0x38;
                    c.g[V1][0] = Addr(p(c.g[S2][0] as u32, 4)).lw(c);
                    c.mult(0, 2, 4, 2);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                }
                c.cycles += 5;
                c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x14)).lw(c);
                call_ptr(c, 0x0031_7670, 0x0019_bc6c)?;
                c.g[A0][0] = Addr(p(c.g[S0][0] as u32, 0x58)).lw(c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_1000, 0x0019_bc78)?;
                c.cycles += 1;
                frame.restore(c, S0, 0x30);
            }
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateCharEquip_UpdateStatBar(self)` (0x19bc90): animate the
/// stat-bar fill (0..1 normalised by the global max) and hand the value to
/// the bar widget.
pub fn cfe_state_char_equip_update_stat_bar(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.swc1(20, p(c.g[SP][0] as u32, 0x30));
        c.g[A2][0] = Addr(p(this as u32, 0x70)).lw(c);
        let no_bar = c.g[A2][0] == 0;
        c.g[V1][0] = Addr(p(r32(c, GP), 0xffff_e7e8)).lw(c);
        if !no_bar {
            c.cycles += 13;
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0xc4)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5dc));
            c.mtc1(0, 2);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5dc));
            c.mtc1(2, 1);
            c.cvt_s_w(1, 1);
            c.mtc1(3, 0);
            c.cvt_s_w(0, 0);
            c.div_s(20, 1, 0);
            c.c_lt_s(20, 2);
            if c.fcc() {
                c.mov_s(20, 2);
            }
            c.cycles += 19;
            c.g[V0][0] = Addr(p(c.g[A2][0] as u32, 8)).lw(c);
            c.g[A1][0] = 0x1;
            c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x48)).lh(c);
            c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x4c)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_bcfc)?;
            c.neg_s(1, 20);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 0x70)).lw(c);
            c.swc1(0, p(c.g[SP][0] as u32, 8));
            c.swc1(1, p(c.g[SP][0] as u32, 4));
            c.swc1(0, c.g[SP][0] as u32);
            c.ldl(3, p(c.g[SP][0] as u32, 7));
            c.ldr(3, c.g[SP][0] as u32);
            c.g[A0][0] = Addr(p(c.g[SP][0] as u32, 8)).lw(c);
            c.sdl(3, p(c.g[V0][0] as u32, 0x57));
            c.sdr(3, p(c.g[V0][0] as u32, 0x50));
            c.sw(p(c.g[V0][0] as u32, 0x58), c.g[A0][0]);
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 5;
        frame.restore_ra(c, 0x10);
        c.lwc1(20, p(c.g[SP][0] as u32, 0x30));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// FE lists.

/// `int cFEStateCharEquip_RemoveListItem(cFEStateCharEquip* self, int idx)`
/// (0x19b458): drop entry `idx` from the array at +0x124 (`count` at +0x958),
/// zero the vacated tail slot and clamp the list view (`top` at list+0x98,
/// `sel` at list+0x95). Returns 1, or 0 when `idx` is out of range.
pub fn cfe_state_char_equip_remove_list_item(c: &mut Cpu) {
    c.cycles += 5;
    let this = c.g[A0][0];
    c.g[T3][0] = this;
    let count = Addr(this as u32).off(0x958).lw(c);
    c.g[V1][0] = count;
    let in_range = (c.g[A1][0] as i64) < (count as i64);
    c.g[V0][0] = in_range as u64;
    c.g[V0][0] = sx((count as u32).wrapping_sub(1));
    if !in_range {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    let shift = (c.g[A1][0] as i64) < (c.g[V0][0] as i64);
    c.g[V0][0] = shift as u64;
    let items = sx((this as u32).wrapping_add(0x124));
    c.g[A2][0] = items;
    if shift {
        c.cycles += 2;
        c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(items as u32));
        loop {
            c.cycles += 8;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(4).lw(c);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
            let n = Addr(this as u32).off(0x958).lw(c);
            c.g[V0][0] = n;
            c.g[V0][0] = sx((n as u32).wrapping_sub(1));
            let more = (c.g[A1][0] as i64) < (c.g[V0][0] as i64);
            c.g[V0][0] = more as u64;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_b488);
            }
        }
    }
    c.cycles += 27;
    c.g[A0][0] = Addr(this as u32).off(0x958).lw(c);
    c.g[T1][0] = u64::MAX;
    c.g[V0][0] = 1;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
    c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
    Addr(this as u32).off(0x958).sw(c, c.g[A0][0]);
    c.g[V1][0] = sx((items as u32).wrapping_add(c.g[V1][0] as u32));
    Addr(c.g[V1][0] as u32).sw(c, 0);
    c.g[A3][0] = Addr(this as u32).off(0x5c).lw(c);
    c.g[A2][0] = Addr(this as u32).off(0x958).lw(c);
    c.g[A0][0] = Addr(c.g[A3][0] as u32).off(0x98).lbu(c);
    c.g[T2][0] = sx((c.g[A2][0] as u32).wrapping_sub(6));
    c.g[A1][0] = Addr(c.g[A3][0] as u32).off(0x95).lbu(c);
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(6));
    c.g[T0][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
    c.g[V1][0] = ((c.g[A2][0] as i64) < (c.g[V1][0] as i64)) as u64;
    if c.g[V1][0] != 0 {
        c.g[A0][0] = c.g[T2][0];
    }
    c.g[A2][0] = ((c.g[A1][0] as i64) < (c.g[A2][0] as i64)) as u64;
    c.g[V1][0] = ((c.g[T1][0] as i64) < (c.g[A0][0] as i64)) as u64;
    if c.g[A2][0] == 0 {
        c.g[A1][0] = c.g[T0][0];
    }
    if c.g[V1][0] == 0 {
        c.g[A0][0] = 0;
    }
    c.g[T1][0] = ((c.g[T1][0] as i64) < (c.g[A1][0] as i64)) as u64;
    Addr(c.g[A3][0] as u32).off(0x98).sb(c, c.g[A0][0]);
    if c.g[T1][0] == 0 {
        c.g[A1][0] = 0;
    }
    c.g[V1][0] = Addr(this as u32).off(0x5c).lw(c);
    let ra = c.g[RA][0] as u32;
    Addr(c.g[V1][0] as u32).off(0x95).sb(c, c.g[A1][0]);
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// `BoltFile_CacheShutdown` (0x0019cf40).
//
// `void BoltFile_CacheShutdown(BoltFile* self)`: free every cached slot of
// every bolt file and drop the cache.

pub fn bolt_file_cache_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
        c.g[V0][0] = 0xb0000;
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[V0][0] = c.g[V0][0] | 0x4008;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.g[S4][0] = c.g[A0][0];
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = 0u64;
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S5][0] = sx((c.g[S4][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.g[S6][0] = 0x1;
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        'l_19cf80: loop {
            c.cycles += 4;
            c.g[S0][0] = 0u64;
            c.g[S2][0] = sx((c.g[S1][0] as u32) << 2);
            c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
            c.g[A2][0] = c.g[S0][0];
            'l_19cf90: loop {
                c.cycles += 7;
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x0019cfa0);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                call_ptr(c, 0x0019d4b8, 0x0019cfa0)?;
                c.g[V0][0] = ((c.g[S0][0] as i64) < 256i64) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[A2][0] = c.g[S0][0];
                if t {
                    if crate::events::event_due(c) {
                        crate::events::deliver_event_here(c, 0x0019cf90);
                    }
                    continue 'l_19cf90;
                } else {
                    c.cycles += 5;
                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add((c.g[S2][0] as u32)));
                    c.g[S1][0] = c.g[S3][0];
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 10i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.sw((c.g[V1][0] as u32), c.g[S6][0]);
                    if t {
                        if crate::events::event_due(c) {
                            crate::events::deliver_event_here(c, 0x0019cf80);
                        }
                        continue 'l_19cf80;
                    } else {
                        'b_19cfd8: {
                            c.cycles += 3;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1788));
                            let t = c.g[A0][0] == 0u64;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                            if t {
                                break 'b_19cfd8;
                            } else {
                                c.cycles += 3;
                                c.g[RA][0] = sx(0x0019cfd4);
                                call_ptr(c, 0x00317e50, 0x0019cfd4)?;
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                                break 'b_19cfd8;
                            }
                        }
                        c.cycles += 10;
                        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1788), 0u64);
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }

        Ok(())
    })
}

/// `cFEStateCharEquip_ShowBuyMode(this)` (0x19a308): relabel the money text
/// and dismiss the three item panels before showing the rider preview.
pub fn cfe_state_char_equip_show_buy_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = abi::arg64(c, 0);
        c.g[V0][0] = Addr(this).off(0x48).lw(c);
        let no_text = c.g[V0][0] == 0;
        c.g[A0][0] = 0x0046_0000;
        if !no_text {
            c.cycles += 5;
            c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x810));
            callee::STR_HASH.call(c, 0x0019_a32c)?;
            c.g[A0][0] = Addr(this).off(0x48).lw(c);
            c.g[A1][0] = c.g[V0][0];
            callee::UI_TEXT_SET_TEXT_ID.call(c, 0x0019_a338)?;
        }

        // The item panel at +0x80 (close with 0).
        c.cycles += 3;
        c.g[A2][0] = Addr(this).off(0x80).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(8).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x48).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x4c).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_a358)?;
        }

        // The item panel at +0x6c.
        c.cycles += 3;
        c.g[A2][0] = Addr(this).off(0x6c).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        if !none {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(8).lw(c);
            c.g[A1][0] = 0;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x48).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x4c).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_a37c)?;
            c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        }

        // Hide the rider preview, then the panel at +0x60 (close with 1).
        c.cycles += 10;
        c.g[A1][0] = Addr(this).off(0x44).lb(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x7c).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
        callee::FE_MODULE_GET_RIDER_PREVIEW.call(c, 0x0019_a390)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE.call(c, 0x0019_a39c)?;
        c.g[A2][0] = Addr(this).off(0x60).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 1;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(8).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x48).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x4c).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_a3bc)?;
        }

        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Character-equip screen.

/// `void cFEStateCharEquip_ToggleBuyEquip(cFEStateCharEquip *self)` (0x19a3d0):
/// flip the `is-buy` flag, switch the detail panel's mode, retitle the
/// category, rebuild and refresh the item list.
pub fn cfe_state_char_equip_toggle_buy_equip(c: &mut Cpu) {
    guest(c, |c| {
        'b_19a450: {
            'b_19a44c: {
                'b_19a420: {
                    c.cycles += 8;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    c.g[S0][0] = c.g[A0][0];
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc8));
                    c.g[V0][0] = c.g[V0][0] ^ 0x1;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x1;
                    if t {
                        break 'b_19a420;
                    } else {
                        c.cycles += 5;
                        c.g[RA][0] = sx(0x0019a3f8);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc8), 0u64);
                        call_ptr(c, 0x0019a238, 0x0019a3f8)?;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = 0x460000;
                        if t {
                            break 'b_19a44c;
                        } else {
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x0019a40c);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x810));
                            call_ptr(c, 0x00317670, 0x0019a40c)?;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                            c.g[RA][0] = sx(0x0019a418);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x0019a418)?;
                            let t = true;
                            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f20));
                            if t {
                                break 'b_19a450;
                            } else {
                                break 'b_19a420;
                            }
                        }
                    }
                }
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                c.g[RA][0] = sx(0x0019a42c);
                c.sw((c.g[S0][0] as u32).wrapping_add(0xc8), c.g[V0][0]);
                call_ptr(c, 0x0019a308, 0x0019a42c)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                let t = c.g[V0][0] == 0u64;
                c.g[A0][0] = 0x460000;
                if t {
                    break 'b_19a44c;
                } else {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x0019a440);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x820));
                    call_ptr(c, 0x00317670, 0x0019a440)?;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x50));
                    c.g[RA][0] = sx(0x0019a44c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x0019a44c)?;
                    break 'b_19a44c;
                }
            }
            c.cycles += 1;
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f20));
            break 'b_19a450;
        }
        c.cycles += 18;
        c.g[A2][0] = 0x50;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x120), 0u64);
        c.g[RA][0] = sx(0x0019a460);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xd0));
        call_ptr(c, 0x003e6448, 0x0019a460)?;
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f20));
        c.g[RA][0] = sx(0x0019a46c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0019bbe0, 0x0019a46c)?;
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f20));
        c.g[RA][0] = sx(0x0019a478);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0019b180, 0x0019a478)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x1;
        c.g[RA][0] = sx(0x0019a488);
        c.g[A2][0] = 0x1;
        call_ptr(c, 0x0019b618, 0x0019a488)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

/// `void cFEStateCharEquip_ShowEquipMode(cFEStateCharEquip *self)` (0x19a238):
/// refresh the equip-mode labels, the preview panel, and the rider preview.
pub fn cfe_state_char_equip_show_equip_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p_2(c.g[S0][0], 0x48)).lw(c);
        let no_label = c.g[V0][0] == 0;
        c.g[A0][0] = 0x0046_0000;
        if !no_label {
            c.cycles += 5;
            c.g[A0][0] = 0x0046_0820;
            call_ptr(c, 0x0031_7670, 0x0019_a25c)?;
            c.g[A0][0] = Addr(p_2(c.g[S0][0], 0x48)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_1000, 0x0019_a268)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(p_2(c.g[S0][0], 0x60)).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(p_2(c.g[A2][0], 0x8)).lw(c);
            c.g[A0][0] = Addr(p_2(c.g[V0][0], 0x48)).lh(c);
            c.g[V1][0] = Addr(p_2(c.g[V0][0], 0x4c)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_a288)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(p_2(c.g[S0][0], 0x88)).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(p_2(c.g[A2][0], 0x8)).lw(c);
            c.g[A0][0] = Addr(p_2(c.g[V0][0], 0x48)).lh(c);
            c.g[V1][0] = Addr(p_2(c.g[V0][0], 0x4c)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_a2a8)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(p_2(c.g[S0][0], 0x80)).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[V0][0] = sx(world(c));
        if !none {
            c.cycles += 7;
            c.g[V0][0] = Addr(p_2(c.g[A2][0], 0x8)).lw(c);
            c.g[A1][0] = 0x1;
            c.g[A0][0] = Addr(p_2(c.g[V0][0], 0x48)).lh(c);
            c.g[V1][0] = Addr(p_2(c.g[V0][0], 0x4c)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_a2cc)?;
            c.g[V0][0] = sx(world(c));
        }
        c.cycles += 13;
        c.g[A1][0] = Addr(p_2(c.g[S0][0], 0x44)).lb(c);
        c.g[A0][0] = Addr(p_2(c.g[V0][0], 0x7c)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
        callee::FEMODULE_GET_RIDER_PREVIEW_2.call(c, 0x0019_a2e0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x1;
        callee::FEMODULE_RIDER_PREVIEW_SET_VISIBLE.call(c, 0x0019_a2ec)?;
        c.g[V0][0] = 0x1;
        frame.restore_ra(c, 0);
        Addr(p_2(c.g[S0][0], 0xae0)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        c.g[RA][0] = c.g[RA][0];
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Bolt cache.

/// `void BoltFile_CacheInit(void* table)` (0x19ce68): clear the ten 256-entry
/// bolt rows and their counts, then create (or reset) the global index
/// manager.
pub fn boltfile_cache_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx(r32(c, SP).wrapping_sub(0x10));
        c.g[V0][0] = 0xb0000;
        c.g[T2][0] = c.g[A0][0];
        c.g[V0][0] = c.g[V0][0] | 0x4008;
        c.g[A2][0] = 0x10000;
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[T0][0] = sx(p(r32(c, T2), r32(c, V0)));
        c.g[A0][0] = 0;
        c.g[A3][0] = 0xffff_ffff_ffff_ffff;
        c.g[T1][0] = 1;
        c.g[A2][0] = c.g[A2][0] | 0x2000;
        loop {
            c.cycles += 6;
            c.mult(0, 2, 4, 6);
            c.g[A1][0] = sx(r32(c, A0) << 2);
            c.g[V1][0] = sx(p(r32(c, V0), r32(c, T2)));
            c.g[V0][0] = sx(r32(c, A0).wrapping_add(1));
            c.g[A0][0] = 0xff;
            loop {
                c.cycles += 10;
                c.sw(r32(c, V1), 0);
                c.g[A0][0] = sx(r32(c, A0).wrapping_sub(1));
                Addr(p(r32(c, V1), 4)).sb(c, 0);
                Addr(p(r32(c, V1), 0x104)).sw(c, 0);
                Addr(p(r32(c, V1), 0x118)).sw(c, 0);
                Addr(p(r32(c, V1), 0x108)).sw(c, 0);
                Addr(p(r32(c, V1), 0x114)).sw(c, 0);
                Addr(p(r32(c, V1), 0x110)).sw(c, c.g[A3][0]);
                let more = (c.g[A0][0] as i64) >= 0;
                c.g[V1][0] = sx(p(r32(c, V1), 0x120));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0019_ceb0);
                }
            }
            c.cycles += 5;
            c.g[V1][0] = sx(p(r32(c, T0), r32(c, A1)));
            c.g[A0][0] = c.g[V0][0];
            c.g[V0][0] = ((c.g[A0][0] as i64) < 10i64) as u64;
            let more = c.g[V0][0] != 0;
            Addr(r32(c, V1)).sw(c, c.g[T1][0]);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0019_ce98);
            }
        }
        c.cycles += 6;
        c.g[V0][0] = 0xb0000;
        c.g[A0][0] = Addr(gp(c, -0x1788)).lw(c);
        c.g[V0][0] = sx(p(r32(c, T2), r32(c, V0)));
        Addr(p(r32(c, V0), 0x4000)).sw(c, 0);
        let have = c.g[A0][0] != 0;
        Addr(p(r32(c, V0), 0x4004)).sw(c, 0);
        if have {
            c.cycles += 2;
            call_ptr(c, 0x0019_cb60, 0x0019_cf34)?;
        } else {
            c.cycles += 10;
            c.g[A1][0] = 0x460000;
            c.g[A0][0] = 0x1f44;
            c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x9c0));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x0019_cf1c)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0019_cb38, 0x0019_cf24)?;
            Addr(gp(c, -0x1788)).sw(c, c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(r32(c, SP));
        let ra = r32(c, RA);
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    })
}

/// `BoltFile_Release(self, slot, layer)` (0x19dc20): drop one reference on a
/// bolt-file slot's entry; at zero either clear the slot (when it is not in
/// the loading state) or unpack it and free the slot.
pub fn boltfile_release(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x60);
        let this = abi::arg(c, 0);
        let slot = abi::arg(c, 1);
        let layer = abi::arg(c, 2);
        c.g[A3][0] = 0x1_0000;
        frame.save(c, 18, 0x30);
        c.g[A3][0] |= 0x2000;
        frame.save(c, 19, 0x20);
        c.g[S2][0] = c.g[A1][0];
        c.mult(0, V0, S2, A3); // v0 = slot * 0x12000
        c.g[S3][0] = c.g[A2][0];
        c.g[V1][0] = 0x120;
        frame.save(c, 17, 0x40);
        c.mult(0, V1, S3, V1); // v1 = layer * 0x120
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 16, 0x50);
        c.sd(p(sp(c), 0x10), c.g[RA][0]);
        c.g[A3][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S1][0] as u32)));
        c.g[S0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        let entry = c.g[S0][0] as u32;
        c.g[V0][0] = lw_4(c, p(entry, 0x114));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        let t = (c.g[V0][0] as i64) > 0;
        c.sw(p(entry, 0x114), c.g[V0][0]);
        let mut skip_one = false;
        if !t {
            c.cycles += 4;
            c.g[V1][0] = lw_4(c, entry);
            c.g[V0][0] = 2;
            let mut go = true;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[V0][0] = lw_4(c, p(entry, 0x104));
            } else {
                c.cycles += 5;
                call_ptr(c, 0x003d_f980, 0x0019_dc88)?;
                c.g[V1][0] = 1;
                if c.g[V0][0] != c.g[V1][0] {
                    skip_one = true;
                    go = false;
                } else {
                    c.cycles += 16;
                    c.g[A0][0] = lw_4(c, p(entry, 0x108));
                    c.g[A2][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x0019_dca4);
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x003d_f9d8, 0x0019_dca4)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x0019_dcb4);
                    c.g[A2][0] = c.g[S0][0];
                    c.pc = 0x0019_d000;
                    ee::s03::BoltFile_UnpackSlot(c);
                    if c.pc != 0x0019_dcb4 {
                        return Err(Unwound);
                    }
                    c.sw(p(c.g[S0][0] as u32, 0x10c), 0);
                    c.g[V1][0] = 0xb_0000;
                    c.sw(c.g[S0][0] as u32, 0);
                    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    c.g[V0][0] = lw_4(c, p(c.g[V1][0] as u32, 0x4004));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    c.sw(p(c.g[V1][0] as u32, 0x4004), c.g[V0][0]);
                    c.g[V0][0] = lw_4(c, p(c.g[S0][0] as u32, 0x104));
                }
            }
            if go {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.sw(c.g[S0][0] as u32, 0);
                } else {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x0019_dcec);
                    c.g[A2][0] = c.g[S3][0];
                    c.pc = 0x0019_d4b8;
                    ee::s03::BoltFile_FreeSlot(c);
                    if c.pc != 0x0019_dcec {
                        return Err(Unwound);
                    }
                }
            }
        }
        if !skip_one {
            c.cycles += 1;
        }
        frame.restore(c, 16, 0x50);
        c.cycles += 6;
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore_ra(c, 0x10);
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
    use crate::difftest::fakes::fake_call;
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
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::RA as RET;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::SP;
    pub(crate) use crate::game::ai::computer::tests::wb;
    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear;
    pub(crate) use crate::game::be::bedbprofile::tests::strict as strict_2;
    pub(crate) use crate::game::be::beintplayer::tests::w8;
    pub(crate) use crate::game::dirtysock::tags2::tests::base;
    pub(crate) use crate::game::dirtysock::tags2::tests::bb;
    pub(crate) use crate::game::dirtysock::tags2::tests::gp_at;
    pub(crate) use crate::game::dirtysock::tags2::tests::world_2 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::fake_small;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_6 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_3 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_9 as pool_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_4 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_9 as BE_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_4 as OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDER;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDERS_2 as RIDERS;
    pub(crate) use crate::game::fe::feasyncfile::tests::RMAN;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM_3 as RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_3 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_7 as WORLD_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W_2;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4;
    pub(crate) use crate::game::fe::festaterewards::tests::globals_3 as globals_2;
    pub(crate) use crate::game::fe::festaterewards::tests::heap_init_3 as heap_init_2;
    pub(crate) use crate::game::fe::festaterewards::tests::pool_4 as pool_2;
    pub(crate) use crate::game::fe::festaterewards::tests::slot_2;
    pub(crate) use crate::game::fe::festaterewards::tests::w16;
    pub(crate) use crate::game::fe::ovstates::tests::base_3 as base_6;
    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3 as base_2;
    pub(crate) use crate::game::movie::movieplayer::tests::heap_init_2 as heap_init;
    pub(crate) use crate::game::movie::movieplayer::tests::pool_allocator;
    pub(crate) use crate::game::movie::movieplayer::tests::run_2 as run;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_2 as BE;
    pub(crate) use crate::game::movie::movieplayer::tests::MEM_HEAP;
    pub(crate) use crate::game::movie::movieplayer::tests::W;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::belib_2 as belib;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::heap_setup;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w;
    pub(crate) use crate::game::sound::soundsys::tests::alloc_ready;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_8;
    pub(crate) use crate::game::sound::soundsys::tests::load_2 as load;
    pub(crate) use crate::game::sound::soundsys::tests::pool_7 as pool_4;
    pub(crate) use crate::game::sound::soundsys::tests::BE as BE_6;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn bolt_file_is_loaded_matches() {
        check_lohi(
            ssx3_ee::s03::f_0019e238,
            bolt_file_is_loaded,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0001 ^ seed as u64);
                scramble(c, &mut r);
                let outer = (seed % 2) as u32;
                let inner = ((seed / 2) % 4) as u32;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = outer as u64;
                c.g[A2][0] = inner as u64;
                let base = DATA
                    .wrapping_add(outer.wrapping_mul(0x12000))
                    .wrapping_add(inner.wrapping_mul(0x120));
                c.bus
                    .write32(base, if seed % 3 == 0 { 1 } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn femodule_rider_preview_set_visible_matches() {
        check(
            ssx3_ee::s03::f_0019e538,
            femodule_rider_preview_set_visible,
            300,
            &|c, seed| {
                let mut r = Rng(0x1111_0004 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = if seed % 5 == 0 {
                    0
                } else {
                    (r.next_u32() as u64) | 1
                };
                for (k, off) in [0xcc0u32, 0xcb8, 0xcb4, 0xcd4].iter().enumerate() {
                    let v = if (seed >> k) & 1 == 0 {
                        0
                    } else if k == 3 && (seed >> 6) & 1 == 0 {
                        0x8000_0000
                    } else {
                        r.next_u32() | 1
                    };
                    c.bus.write32(DATA + off, v);
                }
                c.bus.write32(DATA + 0xcc4, r.next_u32());
                c.bus.write32(DATA + 0xcc8, r.next_u32());
            },
        );
    }

    #[test]
    fn bolt_file_index_manager_get_next_node_matches() {
        check(
            ssx3_ee::s03::f_0019c8f8,
            bolt_file_index_manager_get_next_node,
            200,
            &|c, seed| {
                let mut r = Rng(0x19c9 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let node = if seed % 3 == 0 { 0 } else { DATA + 0x400 };
                c.g[A1][0] = node as u64;
                if node != 0 {
                    let val = if seed % 2 == 0 {
                        r.next_u32() & 0x7fff
                    } else {
                        0x8000 | (r.next_u32() & 0x7fff)
                    };
                    c.bus.write16(node + 0xe, val as u16);
                }
            },
        );
    }

    // ---------------------------------------------------------- money text

    pub(crate) fn money_text_test(
        generated: GuestFn,
        hand: GuestFn,
        text_off: u32,
        value_off: u32,
        salt: u64,
    ) {
        check(generated, hand, 40, &|c, seed| {
            let mut r = machine(c, seed, salt, 1);
            // The money formatter asks the BE library; a zeroed singleton
            // keeps it from lazily loading the rewards file from disc.
            c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x6000);
            c.g[A0][0] = sx(DATA);
            if seed % 3 == 0 {
                c.bus.write32(DATA + text_off, 0);
            } else {
                c.bus.write32(DATA + text_off, DATA + 0x3000);
            }
            c.bus.write32(
                DATA + value_off,
                if seed % 4 == 0 {
                    0
                } else {
                    r.next_u32() % 100_000
                },
            );
            // The formatted string is a static buffer in the game image; the
            // text object at DATA + 0x3000 is only a target of the setter.
            c.bus.write32(DATA + 0x3000, 0);
        });
    }

    #[test]
    fn c_fe_state_char_equip_update_money_text_matches() {
        money_text_test(
            ssx3_ee::s03::f_0019bba0__generated,
            c_fe_state_char_equip_update_money_text,
            0x64,
            0xc0,
            0x1b,
        );
    }

    // -----------------------------------------------------------------------
    // 0x19a498 cFEStateCharEquip_ReleasePreview.

    #[test]
    fn c_fe_state_char_equip_release_preview_matches() {
        check(
            ee::s03::f_0019a498__generated,
            c_fe_state_char_equip_release_preview,
            16,
            &|c, seed| {
                let _r = machine(c, seed, 0x19a4_98, 1);
                let this = DATA + 0x100;
                let world = DATA + 0x1000;
                c.g[A0][0] = sx(this);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x7c, DATA + 0x2000);
                let idx = if seed % 2 == 0 { 0xffff_ffffu32 } else { 2 };
                c.bus.write32(this + 0xa6c, idx);
                c.bus.write32(this + 0xbc, 0); // slot 0: offsets collapse to +0x120
                                               // A live refcount so BoltFile_Release only decrements it.
                let bolt = DATA + 0x2000 + 0x1a70;
                let node = bolt + 2 * 0x120;
                c.bus.write32(node + 0x114, 5);
                c.bus.write32(node, 0);
            },
        );
    }

    #[test]
    fn cfe_state_char_equip_refresh_item_info_matches() {
        check(
            ee::s03::f_0019a9b8__generated,
            cfe_state_char_equip_refresh_item_info,
            24,
            &|c, seed| {
                let _r = base(c, seed, 0x19a9);
                c.bus.write32(DATA + 0xc8, (seed % 3) as u32);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    // ------------------------------------------------ riders, cards, misc

    #[test]
    fn c_fe_state_char_equip_refresh_rider_bolts_matches() {
        check(
            ee::s03::f_0019aca0__generated,
            c_fe_state_char_equip_refresh_rider_bolts,
            12,
            &|c, seed| {
                machine(c, seed, 0x19_aca0, 5);
                belib(c);
                world(c);
                heap_setup(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                bb(c, DATA + 0x2000 + 0x44, (seed % 3) as u8);
                w(c, DATA + 0x2000 + 0xbc, 7 + seed % 3);
                c.pc = 0x0019_aca0;
            },
        );
    }

    pub(crate) fn fill_rows_test(gen: GuestFn, hand: GuestFn, salt: u64, row_target: u32) {
        check(gen, hand, 8, &|c, seed| {
            let _ = alloc_base(c, seed, salt, 4);
            let self_ = DATA;
            let profile = DATA + 0x1000;
            w(c, self_ + 0x5c, profile);
            wb(c, profile + 0x98, (seed % 4) as u8);
            w(c, self_ + 0x958, (seed % 10) as u32);
            let _ = row_target;
            c.g[A0][0] = self_ as u64;
        });
    }

    #[test]
    fn c_fe_state_char_equip_fill_equip_rows_matches() {
        fill_rows_test(
            ee::s03::f_0019b518__generated,
            c_fe_state_char_equip_fill_equip_rows,
            0x19b5_18,
            0x0019_b7e0,
        );
    }

    #[test]
    fn c_fe_state_char_equip_fill_buy_rows_matches() {
        fill_rows_test(
            ee::s03::f_0019b598__generated,
            c_fe_state_char_equip_fill_buy_rows,
            0x19b5_98,
            0x0019_ba60,
        );
    }

    #[test]
    fn bolt_file_index_manager_reset_matches() {
        check(
            ee::s03::f_0019cb60__generated,
            bolt_file_index_manager_reset,
            60,
            &|c, seed| {
                let _ = base_3(c, seed, 0x19cb, 1);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn cfe_state_char_equip_open_buy_popup_matches() {
        check_lohi(
            ee::s03::f_0019c7e8__generated,
            cfe_state_char_equip_open_buy_popup,
            30,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x19c7e8, 4);
                let obj = DATA + 0x1000;
                let item = DATA + 0x2000;
                let screen = DATA + 0x3000;
                let list = DATA + 0x4000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = item as u64;
                w(c, obj + 0x10, screen);
                w(c, obj + 0xc0, r.next_u32());
                w8(c, obj + 0x44, seed as u8);
                w(c, screen + 0x18, list);
                w16(
                    c,
                    item + 0xe,
                    if seed % 3 == 0 {
                        0
                    } else {
                        (seed % 0x100) as u16
                    },
                );
                w(c, item + 0x14, r.next_u32());
            },
        );
    }

    // ------------------------------------------------ more front end / scripts

    #[test]
    fn cfe_state_char_equip_set_category_title_matches() {
        check(
            ee::s03::f_0019bbe0__generated,
            cfe_state_char_equip_set_category_title,
            9,
            &|c, seed| {
                let _ = base_5(c, seed, 0x0019_bbe0, 3);
                w(c, GAME_GP.wrapping_sub(0x1f20), 0x1234);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = if seed % 3 == 0 {
                    0x1234
                } else {
                    (0x1000 + seed * 7) as u64
                };
                w(c, OBJ + 0x58, if seed % 4 == 0 { 0 } else { OBJ2 });
                w(c, OBJ + 0xbc, (seed % 5) as u32);
                w(c, OBJ2 + 0x18, 0);
            },
        );
    }

    #[test]
    fn cfe_state_char_equip_update_stat_bar_matches() {
        check(
            ee::s03::f_0019bc90__generated,
            cfe_state_char_equip_update_stat_bar,
            8,
            &|c, seed| {
                let mut r = base_5(c, seed, 0x0019_bc90, 2);
                c.g[A0][0] = OBJ as u64;
                c.g[GP][0] = GAME_GP as u64;
                w(c, GAME_GP.wrapping_sub(0x1818), 0x5dc + (seed % 200));
                w(c, OBJ + 0xc4, 0x5dc + (seed % 300));
                w(c, OBJ + 0x70, if seed % 4 == 0 { 0 } else { OBJ2 });
                let vt = vtable(c, &mut r, &[0x48]);
                w(c, OBJ2 + 8, vt);
                w(c, OBJ2 + 0x50, 0);
                w(c, OBJ2 + 0x54, 0);
                w(c, OBJ2 + 0x58, 0);
                c.f[20] = r.edge_f32();
            },
        );
    }

    // -------------------------------------------------------------- FE lists.

    #[test]
    fn cfe_state_char_equip_remove_list_item_matches() {
        check(
            ee::s03::f_0019b458__generated,
            cfe_state_char_equip_remove_list_item,
            200,
            &|c, seed| {
                let mut r = base_6(c, seed, 0x19_b458);
                c.g[A0][0] = sx(DATA);
                let count = (seed % 6) as u32;
                let idx = r.next_u32() % (count + 2);
                c.g[A1][0] = sx(idx);
                c.bus.write32(DATA + 0x958, count);
                for i in 0..8 {
                    c.bus.write32(DATA + 0x124 + i * 4, r.next_u32());
                }
                let list = DATA + 0x1000;
                c.bus.write32(DATA + 0x5c, list);
                c.bus.write8(list + 0x95, (r.next_u32() % 9) as u8);
                c.bus.write8(list + 0x98, (r.next_u32() % 9) as u8);
            },
        );
    }

    #[test]
    fn bolt_file_cache_shutdown_matches() {
        strict(
            ee::s03::f_0019cf40__generated,
            bolt_file_cache_shutdown,
            6,
            &|c, _seed| {
                let _r = machine(c, 0, 0x19cf, 8);
                c.bus.write32(GAME_GP - 0x1788, 0);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn cfe_state_char_equip_show_buy_mode_matches() {
        check(
            ee::s03::f_0019a308__generated,
            cfe_state_char_equip_show_buy_mode,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x19a3, 4);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.bus
                    .write32(this + 0x48, if seed % 3 == 0 { 0 } else { DATA + 0x2000 });
                for (i, off) in [0x80u32, 0x6c, 0x60].iter().enumerate() {
                    if seed % (i as u32 + 2) != 0 {
                        let o = DATA + 0x2400 + 0x100 * i as u32;
                        c.bus.write32(this + *off, o);
                        let vt = vtable(c, &mut r, &[0x48]);
                        c.bus.write32(o + 8, vt);
                    }
                }
                c.bus.write32(this + 0x44, seed % 4);
                c.bus.write32(WORLD_2 + 0x7c, DATA + 0x2c00);
            },
        );
    }

    #[test]
    fn cfe_state_char_equip_show_equip_mode_matches() {
        check(
            ee::s03::f_0019a238__generated,
            cfe_state_char_equip_show_equip_mode,
            60,
            &|c, seed| {
                let _r = base_7(c, seed, 0x19a2);
                c.g[A0][0] = DATA as u64;
                c.bus
                    .write32(DATA + 0x48, if seed % 2 == 0 { 0 } else { DATA + 0x2000 });
                c.bus.write8(DATA + 0x44, seed as u8);
                for (field, ret) in [(0x60u32, 0x19a288u32), (0x88, 0x19a2a8), (0x80, 0x19a2cc)] {
                    let obj = DATA + field;
                    c.bus
                        .write32(DATA + field, if seed % 3 == 0 { 0 } else { obj });
                    c.bus.write32(obj + 8, obj + 0x100);
                    c.bus.write16(obj + 0x100 + 0x48, 0);
                    c.bus.write32(obj + 0x100 + 0x4c, FAKE + (ret & 0xf));
                }
                c.bus.write32(DATA + 0x2008, DATA + 0x2100);
                c.bus.write16(DATA + 0x2100 + 0x48, 0);
                c.bus.write32(DATA + 0x2100 + 0x4c, FAKE);
            },
        );
    }

    #[test]
    fn cfe_state_char_equip_toggle_buy_equip_matches() {
        strict_2(
            ee::s03::f_0019a3d0__generated,
            cfe_state_char_equip_toggle_buy_equip,
            120,
            &|c, seed| {
                let _r = base_7(c, seed, 0x19a3);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0xc8, (seed & 1) as u32);
                if seed % 4 == 0 {
                    c.bus.write32(DATA + 0x50, 0);
                } else {
                    c.bus.write32(DATA + 0x50, DATA + 0x2000);
                    c.bus.write32(DATA + 0x2008, DATA + 0x2100);
                    c.bus.write16(DATA + 0x2100 + 0x48, 0);
                    c.bus.write32(DATA + 0x2100 + 0x4c, FAKE);
                }
                for off in [0x48u32, 0x60, 0x80, 0x88, 0x6c] {
                    c.bus.write32(DATA + off, 0);
                }
            },
        );
    }

    #[test]
    fn boltfile_cache_init_matches() {
        check_lohi(
            ee::s03::f_0019ce68__generated,
            boltfile_cache_init,
            8,
            &|c, seed| {
                let _ = base_8(c, seed, 0x19ce68, 2);
                // A large private region for the 10x256 entry table.
                let base_at = 0x0100_0000u32;
                c.g[A0][0] = base_at as u64;
                w(
                    c,
                    GAME_GP.wrapping_sub(0x1788),
                    if seed % 2 == 0 { DATA + 0x2_0000 } else { 0 },
                );
                clear(c, DATA + 0x2_0000, 0x3000);
            },
        );
    }
}
