//! `src/fe/feridermanager.cpp`: hand-written replacements (moved from the
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
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::dirtysock::tags2::arg;
pub(crate) use crate::game::fe::feasyncfile::at_2;
pub(crate) use crate::game::fe::feasyncfile::gpr;
pub(crate) use crate::game::fe::fepopup::at as at_3;
pub(crate) use crate::game::fe::fepopup::lb;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lq;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw;
pub(crate) use crate::game::fe::fepopup::sb;
pub(crate) use crate::game::fe::fepopup::sq;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw;
pub(crate) use crate::game::fe::festateload::at;
pub(crate) use crate::game::fe::ovstatepause::p;
pub(crate) use crate::game::main::gamemode::low;
pub(crate) use crate::game::main::gamemode::sp_2;
use crate::guest::Addr;
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
        0x0019_e680,
        fe_module_rider_preview_find_bones,
        "FEModule_RiderPreviewFindBones",
    ),
    (
        0x0019_eba0,
        fe_module_rider_preview_unload,
        "FEModule_RiderPreviewUnload",
    ),
    (
        0x0019_f3e8,
        femodule_rider_preview_set_transform,
        "FEModule_RiderPreviewSetTransform",
    ),
    (
        0x0019_f548,
        femodule_rider_preview_set_transform2,
        "FEModule_RiderPreviewSetTransform2",
    ),
    (
        0x0019_f908,
        fe_module_rider_preview_load_shape_textures,
        "FEModule_RiderPreviewLoadShapeTextures",
    ),
    (
        0x001a_03c0,
        fe_module_init_rider_previews,
        "FEModule_InitRiderPreviews",
    ),
    (
        0x001a_0498,
        femodule_set_rider_preview_indices,
        "FEModule_SetRiderPreviewIndices",
    ),
    (
        0x001a_04c8,
        fe_module_unload_rider_previews,
        "FEModule_UnloadRiderPreviews",
    ),
    (
        0x001a_0508,
        fe_module_request_preview_char,
        "FEModule_RequestPreviewChar",
    ),
    (
        0x001a_0538,
        femodule_get_rider_preview_2,
        "FEModule_GetRiderPreview_2",
    ),
    (
        0x001a_0548,
        femodule_get_rider_preview,
        "FEModule_GetRiderPreview",
    ),
    (
        0x001a_0570,
        fe_module_set_preview_visible,
        "FEModule_SetPreviewVisible",
    ),
    (
        0x001a_0598,
        fe_module_render_rider_previews,
        "FEModule_RenderRiderPreviews",
    ),
    (
        0x001a_0608,
        fe_module_update_rider_previews,
        "FEModule_UpdateRiderPreviews",
    ),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

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
    pub(crate) use crate::game::bx::bxstring2::callee::EVENT_SELECT_ON_ACTIVATE as EVENT_SELECT_ON_ACTIVATE_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_INSTANCE_RESET as BANK_MONITOR_INSTANCE_RESET_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_KILL_TIMERS_BY_CALLBACK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INSTANCE_4 as BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INTERFACE_4 as BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_MISSION_INT_GET_SESSION_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_HUMAN_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_RACE_INT_GET_SPECIAL_EVENT_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_RACE_INT_GET_TRACK;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ASYNC_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_PLAYER_PAD_MASK_2 as FE_MODULE_GET_PLAYER_PAD_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_RELEASE_BOLT_FILES;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE as FE_MODULE_RIDER_PREVIEW_SET_VISIBLE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_UNLOAD_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_ADVANCE_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_QUIT_TO_LOAD_SCREEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_BINDING_EVAL_FLOAT;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_SPEED;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_USER_DATA_2 as IN_SEQ_INSTANCE_SET_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_START_SEQUENCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_POPUP_FIND_ENABLED_IN_COLUMN_FROM_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_CONNECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_TABLE_FIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_CALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_PASS_THROUGH;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_AS_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR_2 as LUNO_VM_REGISTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_FORMAT_SLOT_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC_NAMED_128;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_EXTENSION_AND_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_GET_BITS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_MARKER_BIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_DTOR_5 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GOTO_NEXT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_5 as NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED_4 as OPERATOR_NEW_NAMED_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_DATA_FIND_MARKER;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_GET_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_GET_START_POSE_ANGLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_IS_LOADED;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_METRIX_SHOW_HUD_MSG_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_PLACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SCRIPT_OBJ_MATCHES_RIDER_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_3 as SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_TRANSITION_TARGET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_BINDING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_TICK_PROCESS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_RIDER_GET_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_UNLOAD_BANK_2 as SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK_2 as SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPCH_MAKE_ID_2 as SPCH_MAKE_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_SYSTEM_IS_INSTANCE_READY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_CHARACTER_TO_SPEECH_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_FADE_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_IS_SPEECH_VOLUME_ON;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_SET_FADE_GAIN;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_MENU_SET_SELECTION_3 as UI_MENU_SET_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_5 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_CTOR_2_4 as UI_STATE_CTOR_2;
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
    pub(crate) use crate::game::fe::fepopup::callee::C00113f88;
    pub(crate) use crate::game::fe::fepopup::callee::C00115ab0;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fe78;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fe98;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fec8;
    pub(crate) use crate::game::fe::fepopup::callee::C0014a080;
    pub(crate) use crate::game::fe::fepopup::callee::C0014a0e0;
    pub(crate) use crate::game::fe::fepopup::callee::C0014bdb8;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d448;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d988;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d998;
    pub(crate) use crate::game::fe::fepopup::callee::C0014dc80;
    pub(crate) use crate::game::fe::fepopup::callee::C0014dd58;
    pub(crate) use crate::game::fe::fepopup::callee::C00150b48;
    pub(crate) use crate::game::fe::fepopup::callee::C00151c48;
    pub(crate) use crate::game::fe::fepopup::callee::C001535c0;
    pub(crate) use crate::game::fe::fepopup::callee::C001543e0;
    pub(crate) use crate::game::fe::fepopup::callee::C00156a60;
    pub(crate) use crate::game::fe::fepopup::callee::C001577e0;
    pub(crate) use crate::game::fe::fepopup::callee::C00157bf0;
    pub(crate) use crate::game::fe::fepopup::callee::C001589b0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158a50;
    pub(crate) use crate::game::fe::fepopup::callee::C00158af0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158c80;
    pub(crate) use crate::game::fe::fepopup::callee::C00158d58;
    pub(crate) use crate::game::fe::fepopup::callee::C00159cd0;
    pub(crate) use crate::game::fe::fepopup::callee::C0015a2e0;
    pub(crate) use crate::game::fe::fepopup::callee::C0016c6b0;
    pub(crate) use crate::game::fe::fepopup::callee::C0016c718;
    pub(crate) use crate::game::fe::fepopup::callee::C0017f500;
    pub(crate) use crate::game::fe::fepopup::callee::C0019e2b0;
    pub(crate) use crate::game::fe::fepopup::callee::C0019f878;
    pub(crate) use crate::game::fe::fepopup::callee::C001bb9b0;
    pub(crate) use crate::game::fe::fepopup::callee::C001c84a8;
    pub(crate) use crate::game::fe::fepopup::callee::C001c8568;
    pub(crate) use crate::game::fe::fepopup::callee::C001c9b68;
    pub(crate) use crate::game::fe::fepopup::callee::C001cb9b0;
    pub(crate) use crate::game::fe::fepopup::callee::C001cbc48;
    pub(crate) use crate::game::fe::fepopup::callee::C001cbce8;
    pub(crate) use crate::game::fe::fepopup::callee::C001cbfc0;
    pub(crate) use crate::game::fe::fepopup::callee::C001ce560;
    pub(crate) use crate::game::fe::fepopup::callee::C001d32e0;
    pub(crate) use crate::game::fe::fepopup::callee::C001e1a30;
    pub(crate) use crate::game::fe::fepopup::callee::C001e3388;
    pub(crate) use crate::game::fe::fepopup::callee::C001e91a8;
    pub(crate) use crate::game::fe::fepopup::callee::C001e91f8;
    pub(crate) use crate::game::fe::fepopup::callee::C001e9290;
    pub(crate) use crate::game::fe::fepopup::callee::C001f10f8;
    pub(crate) use crate::game::fe::fepopup::callee::C0020e900;
    pub(crate) use crate::game::fe::fepopup::callee::C00224c78;
    pub(crate) use crate::game::fe::fepopup::callee::C00224da0;
    pub(crate) use crate::game::fe::fepopup::callee::C00224df0;
    pub(crate) use crate::game::fe::fepopup::callee::C002259a8;
    pub(crate) use crate::game::fe::fepopup::callee::C00227f80;
    pub(crate) use crate::game::fe::fepopup::callee::C00231f80;
    pub(crate) use crate::game::fe::fepopup::callee::C00238bf8;
    pub(crate) use crate::game::fe::fepopup::callee::C0026d4d8;
    pub(crate) use crate::game::fe::fepopup::callee::C0026db88;
    pub(crate) use crate::game::fe::fepopup::callee::C0026e448;
    pub(crate) use crate::game::fe::fepopup::callee::C0026f4a0;
    pub(crate) use crate::game::fe::fepopup::callee::C002702f8;
    pub(crate) use crate::game::fe::fepopup::callee::C002703f0;
    pub(crate) use crate::game::fe::fepopup::callee::C002758c0;
    pub(crate) use crate::game::fe::fepopup::callee::C00275b98;
    pub(crate) use crate::game::fe::fepopup::callee::C00275cf8;
    pub(crate) use crate::game::fe::fepopup::callee::C00278e50;
    pub(crate) use crate::game::fe::fepopup::callee::C00278f38;
    pub(crate) use crate::game::fe::fepopup::callee::C00278f68;
    pub(crate) use crate::game::fe::fepopup::callee::C00281ed8;
    pub(crate) use crate::game::fe::fepopup::callee::C002837c8;
    pub(crate) use crate::game::fe::fepopup::callee::C0028b180;
    pub(crate) use crate::game::fe::fepopup::callee::C0028bb10;
    pub(crate) use crate::game::fe::fepopup::callee::C0028e8c0;
    pub(crate) use crate::game::fe::fepopup::callee::C002913d8;
    pub(crate) use crate::game::fe::fepopup::callee::C003457c8;
    pub(crate) use crate::game::fe::fepopup::callee::C003625c0;
    pub(crate) use crate::game::fe::fepopup::callee::C003626d8;
    pub(crate) use crate::game::fe::fepopup::callee::C003627a8;
    pub(crate) use crate::game::fe::fepopup::callee::C00364cd0;
    pub(crate) use crate::game::fe::fepopup::callee::C00366ce0;
    pub(crate) use crate::game::fe::fepopup::callee::C003684f0;
    pub(crate) use crate::game::fe::fepopup::callee::C0038f460;
    pub(crate) use crate::game::fe::fepopup::callee::C0038f668;
    pub(crate) use crate::game::fe::fepopup::callee::C0039c870;
    pub(crate) use crate::game::fe::fepopup::callee::C0039ff78;
    pub(crate) use crate::game::fe::fepopup::callee::C003b1300;
    pub(crate) use crate::game::fe::fepopup::callee::C003b2360;
    pub(crate) use crate::game::fe::fepopup::callee::C00125108;
    pub(crate) use crate::game::fe::fepopup::callee::C00150928;
    pub(crate) use crate::game::fe::fepopup::callee::C00156988;
    pub(crate) use crate::game::fe::fepopup::callee::C00158910;
    pub(crate) use crate::game::fe::fepopup::callee::C00159170;
    pub(crate) use crate::game::fe::fepopup::callee::C00176890;
    pub(crate) use crate::game::fe::fepopup::callee::C00225068;
    pub(crate) use crate::game::fe::fepopup::callee::C00275498;
    pub(crate) use crate::game::fe::fepopup::callee::C00278308;
    pub(crate) use crate::game::fe::fepopup::callee::C00283228;
    pub(crate) use crate::game::fe::fepopup::callee::C00283518;
    pub(crate) use crate::game::fe::fepopup::callee::C00358020;
    pub(crate) use crate::game::fe::fepopup::callee::C00362478;
    pub(crate) use crate::game::fe::fepopup::callee::C00362660;
    pub(crate) use crate::game::fe::fepopup::callee::C00368170;
    pub(crate) use crate::game::fe::fepopup::callee::C00368970;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_CREATE_STEP1_ON_EDIT_DONE_POPUP_EVENT;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_CREATE_STEP1_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_LOBBY_MENU_ON_CREATE_ROOM_POPUP_EVENT;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_LOBBY_MENU_PROMPT_ROOM_PASSWORD;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_NET_QUICK_MESSAGES_ON_RESET_POPUP_EVENT;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_NET_QUICK_MESSAGES_ON_SAVE_POPUP_EVENT;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_ONLINE_MAIN_MENU_ON_DISCONNECT_POPUP_EVENT;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_ONLINE_MAIN_MENU_ON_LEAVE_POPUP_EVENT;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_REWARD_GALLERY_RELEASE_TEXTURES;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_REWARD_GALLERY_V29;
    pub(crate) use crate::game::fe::fepopup3::callee::FE_ASYNC_MANAGER_GET_STATE;
    pub(crate) use crate::game::fe::fepopup3::callee::FE_LOCALE_UNLOAD_TABLE;
    pub(crate) use crate::game::fe::fepopup3::callee::FE_MODULE_RIDER_PREVIEW_CTOR;
    pub(crate) use crate::game::fe::fepopup3::callee::GAME_MODULE_CTOR;
    pub(crate) use crate::game::fe::fepopup3::callee::HUD_LAYOUT_ICON_TEXT;
    pub(crate) use crate::game::fe::fepopup3::callee::LOBBY_MAN_GET_ROOM_NAME;
    pub(crate) use crate::game::fe::fepopup3::callee::LOBBY_MAN_LOOKUP_PEER_ADDR;
    pub(crate) use crate::game::fe::fepopup3::callee::LOBBY_MAN_MOVE_TO_ROOM;
    pub(crate) use crate::game::fe::fepopup3::callee::MC_MANAGER_END_OPERATION_2 as MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::fe::fepopup3::callee::MC_MANAGER_END_PROGRESS;
    pub(crate) use crate::game::fe::fepopup3::callee::MC_MANAGER_REQUEST_CARD_INFO_2 as MC_MANAGER_REQUEST_CARD_INFO;
    pub(crate) use crate::game::fe::fepopup3::callee::MC_MANAGER_SET_MODE;
    pub(crate) use crate::game::fe::fepopup3::callee::MESSAGE_CENTER_GET_INBOX_3 as MESSAGE_CENTER_GET_INBOX;
    pub(crate) use crate::game::fe::fepopup3::callee::NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::fepopup3::callee::NET_FE_STATE_SETUP_BUDDY_ACTION_POPUP;
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
    pub(crate) use crate::game::fe::ovstatepause::callee::STRCPY as STRCPY_2;
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
    pub(crate) use crate::game::main::gamemode::callee::AI_RAND;
    pub(crate) use crate::game::main::gamemode::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::main::gamemode::callee::BE_NEW_PLAYER_INT_GET_SLOT as BE_NEW_PLAYER_INT_GET_SLOT_2;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GET_MEDAL_REWARD_ID;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GRANT_REWARD;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_IS_REWARD_GRANTED;
    pub(crate) use crate::game::main::gamemode::callee::BUILTIN_DELETE as BUILTIN_DELETE_2;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_PARSE;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_UNLOAD;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS_MASK2;
    pub(crate) use crate::game::main::gamemode::callee::CBE_GET_BE;
    pub(crate) use crate::game::main::gamemode::callee::CBX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::main::gamemode::callee::CPS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_FREE_BUFFERS;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_RESET;
    pub(crate) use crate::game::main::gamemode::callee::CUI_TEXT_SCROLL_SET_WIDE_STRING;
    pub(crate) use crate::game::main::gamemode::callee::CVT_FLOAT_TO_INT;
    pub(crate) use crate::game::main::gamemode::callee::C_MEM_MAN_ALLOC;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::main::gamemode::callee::EA_MEM_FREE as EA_MEM_FREE_2;
    pub(crate) use crate::game::main::gamemode::callee::FE_ASYNC_MANAGER_SET_FILE_STATUS;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_RIDER_PREVIEW_DTOR;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_UNLOAD_RIDER_PREVIEWS;
    pub(crate) use crate::game::main::gamemode::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES;
    pub(crate) use crate::game::main::gamemode::callee::FUNC_0028F140;
    pub(crate) use crate::game::main::gamemode::callee::MEMCPY;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET_3E6448;
    pub(crate) use crate::game::main::gamemode::callee::MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::main::gamemode::callee::MODIFIER_BLOCK_HAS_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_CTOR_2;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_SET_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::MULTI_PARTICLE_DTOR;
    pub(crate) use crate::game::main::gamemode::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::main::gamemode::callee::RAIL_MODIFIER_CTOR;
    pub(crate) use crate::game::main::gamemode::callee::REPLAY_FIND_SNAPSHOT;
    pub(crate) use crate::game::main::gamemode::callee::REPLAY_REQUEST_INSTANT_REPLAY;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_GET_SYNC_CHECKSUM;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_BANK_PENDING_SCORE;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_CLEAR_HUD_MESSAGES;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_COMBO;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_STATS;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_TRICK;
    pub(crate) use crate::game::main::gamemode::callee::RUN_LIST_SEEK;
    pub(crate) use crate::game::main::gamemode::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::main::gamemode::callee::SCRIPT_LIST_ON_SEQ_ADVANCE;
    pub(crate) use crate::game::main::gamemode::callee::SPRINTF;
    pub(crate) use crate::game::main::gamemode::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE as SSX_AUDIO_SYSTEM_GET_INSTANCE_2;
    pub(crate) use crate::game::main::gamemode::callee::STRLEN as STRLEN_2;
    pub(crate) use crate::game::main::gamemode::callee::STR_HASH as STR_HASH_2;
    pub(crate) use crate::game::main::gamemode::callee::TEX_MAN_UNLOCK_VRAM;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_ACCEPT_VISITOR;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::main::gamemode::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::main::gamemode::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::main::gamemode::callee::WSCRIPT_MAN_RESET_COLLECT_SET;
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

/// Addresses the generator emitted as `call_indirect` (outside the shard's
/// own direct-call set, so no readable alias is needed here).
pub(crate) mod addr {
    pub(crate) use crate::game::fe::feasyncfile::addr::__BUILTIN_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::addr::__FIXUNSSFSI;
    pub(crate) use crate::game::fe::feasyncfile::addr::C_BX_STRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::addr::EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::addr::FONT_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::addr::STRLEN;
}

/// `RiderPreview* FEModule_GetRiderPreview_2(FEModule *self, int index)`
/// (0x1a0538): `self + index*0xce0`, the preview record array.
pub fn femodule_get_rider_preview_2(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = 0xce0;
    c.mult(0, A2, A1, V1);
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    abi::ret(c);
}

/// `RiderPreview* FEModule_GetRiderPreview(FEModule *self, int index)`
/// (0x1a0548): identical to [`femodule_get_rider_preview_2`].
pub fn femodule_get_rider_preview(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = 0xce0;
    c.mult(0, A2, A1, V1);
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Front-end module: preview visibility.

/// `void FEModule_SetPreviewVisible(self, int rider, int visible)`
/// (0x1a0570): forward to the rider preview object at `self + rider*0xce0`.
pub fn fe_module_set_preview_visible(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[V0][0] = 0xce0;
        let frame = Frame::open(c, 0x10);
        c.mult(0, V1, A1, V0);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A2][0];
        c.g[A0][0] = sx(arg(c, 0).wrapping_add(c.g[V1][0] as u32));
        callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE.call(c, 0x001a_058c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void FEModule_RiderPreviewSetTransform(FEModule* self, sQuadRM* src)`
/// (0x19f3e8): copy the two quads into `+0xc30`/`+0xc40` and apply them.
pub fn femodule_rider_preview_set_transform(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.lq(3, abi::arg(c, 1));
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A0][0];
        let dst = c.g[A2][0] as u32;
        c.sq(3, at(dst, 0xc30));
        c.lq(2, at(abi::arg(c, 1), 0x10));
        c.sq(2, at(dst, 0xc40));
        callee::RIDER_PREVIEW_APPLY_TRANSFORM.call(c, 0x0019_f408)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void FEModule_RiderPreviewSetTransform2(FEModule* self, sQuadRM* src)`
/// (0x19f548): the second pair of quads (`+0xc50`/`+0xc60`).
pub fn femodule_rider_preview_set_transform2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.lq(3, abi::arg(c, 1));
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A0][0];
        let dst = c.g[A2][0] as u32;
        c.sq(3, at(dst, 0xc50));
        c.lq(2, at(abi::arg(c, 1), 0x10));
        c.sq(2, at(dst, 0xc60));
        callee::RIDER_PREVIEW_APPLY_TRANSFORM.call(c, 0x0019_f568)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void FEModule_SetRiderPreviewIndices(FEModule* self)` (0x1a0498): number
/// the two rider preview slots (`FEModule` stride 0xce0, index at `+0x4`).
pub fn femodule_set_rider_preview_indices(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V1][0] = 0;
    loop {
        c.cycles += 7;
        let a = c.g[A0][0] as u32;
        let v = c.g[V1][0];
        Addr(at(a, 4)).sw(c, v);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A0][0] = sx(a.wrapping_add(0xce0));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x001a_04a0);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Front end: preview characters.

/// `FEModule_RequestPreviewChar(module, rider, a2, a3)` 0x1a0508: forward to
/// rider `idx`'s preview object (`module + idx * 0xce0`), passing `a2`/`a3` as
/// the request's character and flag.
pub fn fe_module_request_preview_char(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let module = abi::arg(c, 0);
        let idx = abi::arg(c, 1);
        // The arguments are moved, not truncated: `a1 = a2; a2 = a3`.
        let a2 = abi::arg64(c, 2);
        let a3 = abi::arg64(c, 3);
        c.g[V0][0] = 0xce0;
        let frame = Frame::open(c, 0x10);
        // `mult v1, a1, v0`: the byte stride to the rider's preview object.
        let stride = abi::mult(c, idx, 0xce0);
        c.g[V1][0] = sx(stride);
        frame.save_ra(c, 0);
        c.g[A1][0] = a2;
        c.g[A2][0] = a3;
        c.g[A0][0] = sx(module.wrapping_add(stride));
        callee::FE_MODULE_RIDER_PREVIEW_REQUEST_CHAR.call(c, 0x001a_0528)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rider previews: two slots 0xce0 bytes apart.

/// `void FEModule_UnloadRiderPreviews(FEModule* m)` (0x1a04c8): unload the
/// two rider preview slots at `+0x19c0` and `+0x26a0` (the loop the compiler
/// left, with an event check at the back edge).
pub fn fe_module_unload_rider_previews(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(p(crate::abi::r32(c, S0), 0x19c0));
        loop {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            callee::FE_MODULE_RIDER_PREVIEW_UNLOAD.call(c, 0x001a_04e8)?;
            c.g[S0][0] = sx(p(crate::abi::r32(c, S0), 0xce0));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            frame.restore_ra(c, 0);
            if !more {
                c.cycles += 4;
                frame.restore(c, S0, 0x20);
                frame.restore(c, S1, 0x10);
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001a_04e0);
            }
        }
        frame.close(c);
        Ok(())
    })
}

/// `void FEModule_UpdateRiderPreviews(FEModule* m)` (0x1a0608): the same
/// walk with `FEModule_RiderPreviewUpdate`.
pub fn fe_module_update_rider_previews(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(p(crate::abi::r32(c, S0), 0x19c0));
        loop {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            callee::FE_MODULE_RIDER_PREVIEW_UPDATE.call(c, 0x001a_0628)?;
            c.g[S0][0] = sx(p(crate::abi::r32(c, S0), 0xce0));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            frame.restore_ra(c, 0);
            if !more {
                c.cycles += 4;
                frame.restore(c, S0, 0x20);
                frame.restore(c, S1, 0x10);
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001a_0620);
            }
        }
        frame.close(c);
        Ok(())
    })
}

/// `void* FEModule_InitRiderPreviews(FEModule* self)` (0x1a03c0): construct
/// the two rider-preview objects 0xce0 bytes apart; returns `self`.
pub fn fe_module_init_rider_previews(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[S2][0];
        frame.save(c, S3, 0x10);
        c.g[S1][0] = 1;
        frame.save_ra(c, 0);
        c.g[S3][0] = 0xffff_ffff_ffff_ffff;
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            callee::FE_MODULE_RIDER_PREVIEW_CTOR.call(c, 0x001a_03f4)?;
            let more = c.g[S1][0] != c.g[S3][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xce0));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001a_03e8);
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void FEModule_RenderRiderPreviews(FEModule* self)` (0x1a0598): point the
/// render targets at_2 the FE canvas and render both rider previews.
pub fn fe_module_render_rider_previews(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = Addr(gpr(c, -0x854)).lw(c);
        frame.save(c, S0, 0x20);
        c.g[V1][0] = 0x2e;
        frame.save(c, S1, 0x10);
        c.g[A2][0] = 0x2f;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Addr(at_2(c, V0, 0x24)).sw(c, c.g[V1][0]);
        c.g[S1][0] = sx(at_2(c, S0, 0x19c0));
        c.g[A1][0] = Addr(at_2(c, V0, 0x1008)).lw(c);
        Addr(at_2(c, V0, 0x84)).sw(c, c.g[A2][0]);
        Addr(at_2(c, V0, 0x10a8)).sw(c, c.g[A1][0]);
        c.g[V0][0] = Addr(at_2(c, S0, 0xcc8)).lw(c);
        loop {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.g[S0][0] = sx(at_2(c, S0, 0xce0));
            } else {
                c.cycles += 3;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0019_ee88, 0x001a_05e0)?;
                c.g[S0][0] = sx(at_2(c, S0, 0xce0));
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 5;
                frame.restore(c, S0, 0x20);
                frame.restore(c, S1, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.g[V0][0] = Addr(at_2(c, S0, 0xcc8)).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x001a_05d0);
            }
        }
    })
}

// ---------------------------------------------------------------------------
// FE module teardown, text scroll and group focus.

/// `void FEModule_DestroyRiderPreviews(self, flags)` (0x1a0420): unload the
/// previews, destroy the two rider-preview objects at `+0xce0`/`+0x19c0` and
/// free the module when `flags & 1`.
pub fn fe_module_destroy_rider_previews(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x40));
        c.sq(17, sp_2(c).wrapping_add(0x20));
        c.g[S1][0] = c.g[A0][0];
        c.sq(18, sp_2(c).wrapping_add(0x10));
        c.sq(16, sp_2(c).wrapping_add(0x30));
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[S2][0] = c.g[A1][0];
        callee::FE_MODULE_UNLOAD_RIDER_PREVIEWS.call(c, 0x001a_0440)?;
        let null = c.g[S1][0] == 0;
        c.g[S0][0] = sx(low(c, S1).wrapping_add(0x19c0));
        let mut extra = true;
        let mut result = low(c, S2) & 1;
        if !null {
            c.cycles += 2;
            let last = c.g[S1][0] == c.g[S0][0];
            result = low(c, S2) & 1;
            if !last {
                c.cycles += 2;
                c.g[S0][0] = sx(low(c, S0).wrapping_sub(0xce0));
                loop {
                    c.cycles += 5;
                    c.g[A1][0] = 0;
                    c.g[A0][0] = c.g[S0][0];
                    callee::FE_MODULE_RIDER_PREVIEW_DTOR.call(c, 0x001a_0464)?;
                    let more = c.g[S1][0] != c.g[S0][0];
                    c.g[S0][0] = sx(low(c, S0).wrapping_sub(0xce0));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x001a_0458);
                    }
                }
            } else {
                extra = false;
            }
        }
        if extra {
            c.cycles += 1;
        }
        c.g[V0][0] = result as u64;
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.lq(16, sp_2(c).wrapping_add(0x30));
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, callee::BUILTIN_DELETE_2.addr, 0x001a_0480)?;
            c.lq(16, sp_2(c).wrapping_add(0x30));
        }
        c.cycles += 5;
        c.lq(17, sp_2(c).wrapping_add(0x20));
        c.lq(18, sp_2(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end rider preview.

/// `int FEModule_RiderPreviewFindTexture(RiderPreview* self, char* name)`
/// (0x19f878): the id of the first preview entry whose name matches, else -1.
pub fn fe_module_rider_preview_find_texture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = 0;
        frame.save(c, S0, 0x50);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A1][0];
        c.g[V0][0] = Addr(p(r32(c, S3), 0x18)).lw(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        let mut found = false;
        if !empty {
            c.cycles += 2;
            c.g[S1][0] = sx(p(r32(c, S3), 0x1c));
            c.g[S0][0] = sx(p(r32(c, S3), 0x20));
            loop {
                c.cycles += 5;
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0041_65a8, 0x0019_f8bc)?;
                if c.g[V0][0] == 0 {
                    c.cycles += 2;
                    c.g[V0][0] = Addr(r32(c, S1)).lw(c);
                    found = true;
                    break;
                }
                c.g[V0][0] = Addr(p(r32(c, S3), 0x18)).lw(c);
                c.cycles += 5;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(0xc));
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(0xc));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0019_f8b0);
                }
            }
        }
        if !found {
            c.cycles += 1;
            c.g[V0][0] = u64::MAX;
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x10);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x40);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `FEModule_RiderPreviewUnload(this)` (0x19eba0): release the preview's
/// model and its three sub-objects, clear the handles and reset the state.
pub fn fe_module_rider_preview_unload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = abi::arg64(c, 0);
        c.g[A0][0] = Addr(this).off(8).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            callee::FE_MODULE_RIDER_PREVIEW_UNLOAD_DTOR.call(c, 0x0019_ebc4)?;
        }
        c.g[A2][0] = Addr(this).off(0xc).lw(c);

        // The three sub-objects (offsets 0xc, 0xc74, 0xc70).
        c.cycles += 2;
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 3;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x58).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_ebe4)?;
        }

        c.cycles += 3;
        c.g[A2][0] = Addr(this).off(0xc74).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 3;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0xc4).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_ec04)?;
        }

        c.cycles += 3;
        c.g[A2][0] = Addr(this).off(0xc70).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 3;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0xa4).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_ec24)?;
        }

        c.cycles += 16;
        Addr(this).off(8).sw(c, 0);
        c.g[A0][0] = c.g[S0][0];
        Addr(this).off(0xc).sw(c, 0);
        Addr(this).off(0xc70).sw(c, 0);
        Addr(this).off(0xc74).sw(c, 0);
        callee::FE_MODULE_RIDER_PREVIEW_RELEASE_BOLT_FILES.call(c, 0x0019_ec3c)?;
        c.g[V0][0] = sx(0xffff_ffff_u32);
        Addr(this).off(0xcc8).sw(c, 0);
        Addr(this).off(0xcd4).sw(c, c.g[V0][0]);
        Addr(this).sw(c, c.g[V0][0]);
        Addr(this).off(4).sw(c, c.g[V0][0]);
        Addr(this).off(0xc78).sw(c, 0);

        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int RiderModel_FindBone(RiderModel*, int bucket, const char *name)`.
pub(crate) const RIDER_MODEL_FIND_BONE: u32 = 0x0031_0c48;

// ---------------------------------------------------------------------------
// Rider preview bones.

/// The bone-name addresses `FEModule_RiderPreviewFindBones` looks up and the
/// second argument each is found with (`.rodata` strings).
pub(crate) const PREVIEW_BONE_NAMES: [(u32, u64); 14] = [
    (0x004a_1970, 0),
    (0x004a_1978, 0),
    (0x0046_0a00, 0),
    (0x0046_0a10, 0),
    (0x0046_0a20, 0),
    (0x0046_0a30, 0),
    (0x0046_0a40, 0),
    (0x0046_0a50, 0),
    (0x0046_0a60, 0),
    (0x0046_0a70, 0),
    (0x0046_0a80, 0),
    (0x0046_0a90, 0),
    (0x0046_0aa0, 1),
    (0x0046_0ab0, 1),
];

/// `void FEModule_RiderPreviewFindBones(FEModule *m)` (0x19e680): resolve the
/// preview rider's bones by name and cache the indices at `+0xc7c..+0xcb0`.
pub fn fe_module_rider_preview_find_bones(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 92;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;

        for (i, &(name, second)) in PREVIEW_BONE_NAMES.iter().enumerate() {
            c.g[A2][0] = name as u64;
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(8)).lw(c);
            c.g[A1][0] = second;
            call_ptr(c, RIDER_MODEL_FIND_BONE, 0x0019_e6a4 + 0x18 * i as u32)?;
            Addr(r32(c, S0).wrapping_add(0xc7c + 4 * i as u32)).sw(c, c.g[V0][0]);
        }

        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void func_0019F908(void* self, sPkg* pkg, int replace)` (0x19f908): load a
/// rider preview's shape textures: for every entry of the package, bind the
/// data to the already-known model id (replace) or append the entry's name
/// and a fresh id to `self`'s table.
pub fn fe_module_rider_preview_load_shape_textures(c: &mut Cpu) {
    let frame = Frame::open(c, 0xb0);
    'b_19fa48: {
        c.cycles += 17;
        frame.save(c, S0, 0xa0);
        frame.save(c, S2, 0x80);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S4, 0x60);
        c.g[S2][0] = 0u64;
        frame.save(c, S7, 0x30);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S1, 0x90);
        frame.save(c, S3, 0x70);
        frame.save(c, S6, 0x40);
        sq(c, 30, at_3(c, SP, 0x20));
        frame.save_ra(c, 0x10);
        frame.save(c, S5, 0x50);
        c.g[S5][0] = lw(c, at_3(c, S4, 0x8));
        let t = (c.g[S5][0] as i64) <= 0;
        c.g[S7][0] = c.g[A2][0];
        if t {
            break 'b_19fa48;
        } else {
            c.cycles += 3;
            c.g[S6][0] = 0xc;
            c.g[S3][0] = sx(at_3(c, S4, 0x14));
            c.g[FP][0] = 0x4a0000;
            'l_19f958: loop {
                'b_19fa3c: {
                    'b_19fa38: {
                        'b_19f9dc: {
                            c.cycles += 21;
                            c.g[V0][0] = lw(c, r32(c, S3));
                            c.g[V1][0] = 0x4a0000;
                            c.g[A0][0] = sx(at_3(c, SP, 0x2));
                            c.g[A1][0] = 0u64;
                            c.g[S1][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
                            c.g[T1][0] = sx(at_3(c, V1, 0x19a8));
                            c.g[V0][0] = lb(c, r32(c, T1));
                            c.g[A3][0] = lb(c, at_3(c, T1, 0x1));
                            sb(c, r32(c, SP), c.g[V0][0]);
                            sb(c, at_3(c, SP, 0x1), c.g[A3][0]);
                            c.g[RA][0] = sx(0x0019f988);
                            c.g[A2][0] = 0x6;
                            if call_ptr(c, 0x00416210, 0x0019f988).is_err() {
                                return;
                            }
                            c.g[A0][0] = c.g[S4][0];
                            c.g[A1][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x0019f998);
                            c.g[A2][0] = c.g[SP][0];
                            if call_ptr(c, 0x003b3d00, 0x0019f998).is_err() {
                                return;
                            }
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x0019f9a4);
                            c.g[A1][0] = c.g[SP][0];
                            if callee::C0019f878.call(c, 0x0019f9a4).is_err() {
                                return;
                            }
                            if (c.g[V0][0] as i64) < 0 {
                                c.g[A0][0] = lw(c, at_3(c, S0, 0x18));
                                break 'b_19f9dc;
                            } else {
                                c.cycles += 2;
                                let t = c.g[S7][0] == 0u64;
                                c.g[T0][0] = lw(c, r32(c, GP).wrapping_sub(0x854));
                                if t {
                                    break 'b_19fa38;
                                } else {
                                    c.cycles += 10;
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[A2][0] = c.g[S1][0];
                                    c.g[A3][0] = 0x1;
                                    c.g[V0][0] = lw(c, at_3(c, T0, 0x10d8));
                                    c.g[A0][0] = lh(c, at_3(c, V0, 0x180));
                                    c.g[V1][0] = lw(c, at_3(c, V0, 0x184));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x0019f9d4);
                                    c.g[A0][0] = sx(r32(c, T0).wrapping_add(r32(c, A0)));
                                    if call_ptr(c, t, 0x0019f9d4).is_err() {
                                        return;
                                    }
                                    let t = true;
                                    c.g[S2][0] = sx(at_3(c, S2, 0x1));
                                    if t {
                                        break 'b_19fa3c;
                                    } else {
                                        break 'b_19f9dc;
                                    }
                                }
                            }
                        }
                        c.cycles += 23;
                        c.g[A1][0] = c.g[SP][0];
                        c.mult(0, V0, A0, S6);
                        c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, S0)));
                        c.g[RA][0] = sx(0x0019f9f0);
                        c.g[A0][0] = sx(at_3(c, A0, 0x20));
                        if call_ptr(c, 0x004166f4, 0x0019f9f0).is_err() {
                            return;
                        }
                        c.g[T2][0] = lw(c, r32(c, GP).wrapping_sub(0x854));
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A2][0] = sx(at_3(c, FP, 0x19b0));
                        c.g[A3][0] = 0u64;
                        c.g[V1][0] = lw(c, at_3(c, T2, 0x10d8));
                        c.g[T0][0] = 0x1;
                        c.g[T1][0] = 0xffffffffffffffff;
                        c.g[A0][0] = lh(c, at_3(c, V1, 0x170));
                        c.g[V0][0] = lw(c, at_3(c, V1, 0x174));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x0019fa1c);
                        c.g[A0][0] = sx(r32(c, T2).wrapping_add(r32(c, A0)));
                        if call_ptr(c, t, 0x0019fa1c).is_err() {
                            return;
                        }
                        c.g[V1][0] = lw(c, at_3(c, S0, 0x18));
                        c.mult(0, A0, V1, S6);
                        c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, S0)));
                        sw(c, at_3(c, V1, 0x1c), c.g[V0][0]);
                        c.g[V0][0] = lw(c, at_3(c, S0, 0x18));
                        c.g[V0][0] = sx(at_3(c, V0, 0x1));
                        sw(c, at_3(c, S0, 0x18), c.g[V0][0]);
                        break 'b_19fa38;
                    }
                    c.cycles += 1;
                    c.g[S2][0] = sx(at_3(c, S2, 0x1));
                    break 'b_19fa3c;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S5][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[S3][0] = sx(at_3(c, S3, 0x8));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x0019f958);
                    }
                    continue 'l_19f958;
                } else {
                    break 'b_19fa48;
                }
            }
        }
    }
    c.cycles += 12;
    frame.restore(c, S0, 0xa0);
    frame.restore(c, S1, 0x90);
    frame.restore(c, S2, 0x80);
    frame.restore(c, S3, 0x70);
    frame.restore(c, S4, 0x60);
    frame.restore(c, S5, 0x50);
    frame.restore(c, S6, 0x40);
    frame.restore(c, S7, 0x30);
    lq(c, 30, at_3(c, SP, 0x20));
    c.g[RA][0] = ld(c, at_3(c, SP, 0x10));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_3(c, SP, 0xb0));
    c.pc = t;
    return;
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::compare;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::SP as STACK_TOP;
    use crate::difftest::SP;
    pub(crate) use crate::game::ai::ai::tests::CHILD;
    pub(crate) use crate::game::dirtysock::tags2::tests::rng_2 as rng;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::fake_small;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_2 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_9 as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDER;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDERS_2 as RIDERS;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::RMAN;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS;
    pub(crate) use crate::game::fe::feasyncfile::tests::W;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_7 as WORLD_2;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_2;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;
    pub(crate) use crate::game::fe::fepopup::tests::AUX;
    pub(crate) use crate::game::fe::fepopup::tests::BEL;
    pub(crate) use crate::game::fe::fepopup::tests::RACE;
    pub(crate) use crate::game::fe::fepopup::tests::RMAN_2;
    pub(crate) use crate::game::fe::fepopup::tests::SYS;
    pub(crate) use crate::game::fe::fepopup::tests::WORLD_4 as WORLD_3;
    pub(crate) use crate::game::fe::fepopup3::tests::setup;
    pub(crate) use crate::game::fe::festateload::tests::clear;
    pub(crate) use crate::game::fe::festateload::tests::OBJ;
    pub(crate) use crate::game::fe::ovstatepause::tests::globals_2;
    pub(crate) use crate::game::fe::ovstatepause::tests::WORLD;
    pub(crate) use crate::game::main::debugmenu::tests::w;
    pub(crate) use crate::game::movie::movieplayer::tests::globals;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_LIBRARY;
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
    fn femodule_get_rider_preview_2_matches() {
        check_lohi(
            ssx3_ee::s03::f_001a0538,
            femodule_get_rider_preview_2,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0005 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 8) as u64;
            },
        );
    }

    #[test]
    fn femodule_get_rider_preview_matches() {
        check_lohi(
            ssx3_ee::s03::f_001a0548,
            femodule_get_rider_preview,
            200,
            &|c, seed| {
                let mut r = Rng(0x1111_0006 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 8) as u64;
            },
        );
    }

    #[test]
    fn fe_module_set_preview_visible_matches() {
        check_lohi(
            ssx3_ee::s03::f_001a0570__generated,
            fe_module_set_preview_visible,
            200,
            &|c, seed| {
                let mut r = rng(seed, 0x1a);
                scramble(c, &mut r);
                let index = seed % 4;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = index as u64;
                c.g[A2][0] = (seed % 3 != 0) as u64;
                let base = DATA + index * 0xce0;
                // Cover the early-out branches of the preview update.
                for off in [0xcb4u32, 0xcb8, 0xcc0, 0xcc4, 0xcc8] {
                    c.bus.write32(
                        base + off,
                        if r.next_u32() % 2 == 0 {
                            0
                        } else {
                            r.next_u32()
                        },
                    );
                }
                c.bus.write32(base + 0xcd4, r.next_u32() | 0x8000_0000);
            },
        );
    }

    #[test]
    fn femodule_rider_preview_set_transform_matches() {
        check(
            s03::f_0019f3e8__generated,
            femodule_rider_preview_set_transform,
            120,
            &|c, seed| {
                machine(c, seed, 0x19f3e8, 1);
                clear(c, OBJ, 0x100);
                clear(c, CHILD, 0x40);
                c.g[A0][0] = sx(OBJ);
                c.g[A1][0] = sx(CHILD);
                for i in 0..16u32 {
                    c.bus
                        .write32(CHILD + 4 * i, seed.wrapping_mul(0x0101_0101) ^ i);
                }
            },
        );
    }

    #[test]
    fn femodule_rider_preview_set_transform2_matches() {
        check(
            s03::f_0019f548__generated,
            femodule_rider_preview_set_transform2,
            120,
            &|c, seed| {
                machine(c, seed, 0x19f548, 1);
                clear(c, OBJ, 0x100);
                clear(c, CHILD, 0x40);
                c.g[A0][0] = sx(OBJ);
                c.g[A1][0] = sx(CHILD);
                for i in 0..16u32 {
                    c.bus
                        .write32(CHILD + 4 * i, seed.wrapping_mul(0x0101_0101) ^ i);
                }
            },
        );
    }

    #[test]
    fn femodule_set_rider_preview_indices_matches() {
        check(
            s03::f_001a0498__generated,
            femodule_set_rider_preview_indices,
            200,
            &|c, seed| {
                let mut r = Rng(0x1a04_98aa ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = sx(OBJ);
                clear(c, OBJ, 0x1c00);
                c.bus.write32(OBJ + 4, r.next_u32());
                c.bus.write32(OBJ + 0xce4, r.next_u32());
            },
        );
    }

    #[test]
    fn fe_module_request_preview_char_matches() {
        check(
            ssx3_ee::s03::f_001a0508__generated,
            fe_module_request_preview_char,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0xa1, 1);
                globals(c);
                let idx = (seed % 3) as u32;
                // The callee compares the character it is asked for with
                // `*(preview)`; `*(preview + 0x10)` and `*(+0xcd4)` stay zero, so
                // both variants take one of the two call-free paths.
                let want = (seed % 2) as u32;
                let stored = if seed % 4 < 2 { want } else { want ^ 1 };
                c.bus.write32(DATA + idx * 0xce0, stored);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = idx as u64;
                c.g[A2][0] = want as u64;
                c.g[A3][0] = 0;
            },
        );
    }

    #[test]
    fn fe_module_unload_rider_previews_matches() {
        check(
            ssx3_ee::s03::f_001a04c8__generated,
            fe_module_unload_rider_previews,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x1d, 1);
                globals_2(c);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn fe_module_update_rider_previews_matches() {
        check(
            ssx3_ee::s03::f_001a0608__generated,
            fe_module_update_rider_previews,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e, 1);
                globals_2(c);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn fe_module_init_rider_previews_matches() {
        check(
            ssx3_ee::s03::f_001a03c0__generated,
            fe_module_init_rider_previews,
            100,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1a03c0);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn fe_module_render_rider_previews_matches() {
        check(
            ee::s03::f_001a0598__generated,
            fe_module_render_rider_previews,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x1a05_98);
                let canvas = DATA + 0x1000;
                c.bus.write32(GAME_GP - 0x854, canvas);
                fill(c, &mut r, canvas, 0x400);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0xcc8, 0);
                c.bus.write32(DATA + 0xce0 + 0xcc8, 0);
            },
        );
    }

    #[test]
    fn fe_module_rider_preview_unload_matches() {
        check(
            ee::s03::f_0019eba0__generated,
            fe_module_rider_preview_unload,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x19eb, 4);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.bus
                    .write32(this + 8, if seed % 3 == 0 { 0 } else { DATA + 0x2000 });
                for (i, off) in [0xcu32, 0xc74, 0xc70].iter().enumerate() {
                    if seed % (i as u32 + 2) != 0 {
                        let o = DATA + 0x2400 + 0x100 * i as u32;
                        c.bus.write32(this + *off, o);
                        let slot = match *off {
                            0xc => 0x58,
                            0xc74 => 0xc4,
                            _ => 0xa4,
                        };
                        let vt = vtable(c, &mut r, &[slot]);
                        c.bus.write32(o + slot, vt);
                    }
                }
            },
        );
    }

    #[test]
    fn fe_module_rider_preview_find_bones_matches() {
        check(
            ee::s03::f_0019e680__generated,
            fe_module_rider_preview_find_bones,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x19_e680, 14);
                c.g[A0][0] = DATA as u64;
                // A model with no bones: every lookup takes the not-found path.
                w(c, DATA + 8, DATA + 0x2000);
            },
        );
    }

    pub(crate) const RIDER_TABLE: u32 = DATA + 0x1_0000;

    pub(crate) const TEX_PKG: u32 = DATA + 0x1_1000;

    #[test]
    fn fe_module_rider_preview_load_shape_textures_matches() {
        check_all(
            ee::s03::f_0019f908__generated,
            fe_module_rider_preview_load_shape_textures,
            24,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x19f9_08, 10);
                c.g[A0][0] = sx(RIDER_TABLE);
                c.g[A1][0] = sx(TEX_PKG);
                c.g[A2][0] = sx(seed % 2);
                c.bus.write32(TEX_PKG + 0x8, (seed % 3) as u32); // entry count
                c.bus.write32(RIDER_TABLE + 0x4, seed % 3); // stored entry count
                if seed % 2 == 0 {
                    c.bus.write32(TEX_PKG + 0xc, 0x10); // first entry offset
                    c.bus.write32(TEX_PKG + 0x10, 0x20);
                }
                let _ = &mut r;
            },
        );
    }
}
