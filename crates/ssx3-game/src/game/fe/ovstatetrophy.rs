//! `src/fe/ovstatetrophy.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::rider::world;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_3;
pub(crate) use crate::game::fe::feasyncfile::gp_3 as gp;
pub(crate) use crate::game::fe::feasyncfile::p as p_3;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_4;
pub(crate) use crate::game::fe::feasyncfile::sw_5 as sw_2;
pub(crate) use crate::game::fe::fepopup2::fe_state_screen_init;
pub(crate) use crate::game::fe::fepopup3::p as p_5;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::fereal::gp as gp_2;
pub(crate) use crate::game::fe::fereal::open;
pub(crate) use crate::game::fe::ovstaterewardslist::ov_screen_widget_action;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::fe::ovtemplatepausemenu::p as p_2;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;
pub(crate) use crate::game::main::gamestate::gw;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at;
pub(crate) use crate::game::world::worldview::p_2 as p;
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
        0x001c_b418,
        c_keyboard_popup_set_text,
        "cKeyboardPopup_SetText",
    ),
    (
        0x001c_cf00,
        c_keyboard_popup_set_static_text,
        "cKeyboardPopup_SetStaticText",
    ),
    (
        0x001c_d088,
        c_keyboard_popup_set_max_length,
        "cKeyboardPopup_SetMaxLength",
    ),
    (
        0x001c_e3c8,
        c_keyboard_popup_set_key_enabled,
        "cKeyboardPopup_SetKeyEnabled",
    ),
    (
        0x001c_e468,
        c_keyboard_popup_set_function_keys_enabled,
        "cKeyboardPopup_SetFunctionKeysEnabled",
    ),
    (0x0020_e938, ov_screen_trophy_init, "OVScreenTrophy_Init"),
    (0x0020_eaa0, hud_meter_ctor, "HUDMeter_ctor"),
    (0x0020_eb08, hud_meter_dtor, "HUDMeter_dtor"),
    (
        0x0020_eb50,
        cov_state_hud1p_load_meter_assets,
        "cOVStateHUD1P_LoadMeterAssets",
    ),
    (
        0x0021_05b0,
        race_progress_reset_rider,
        "RaceProgress_ResetRider",
    ),
    (
        0x0021_05d0,
        race_progress_set_path_data,
        "RaceProgress_SetPathData",
    ),
    (
        0x0021_0608,
        race_progress_clear_path_data,
        "RaceProgress_ClearPathData",
    ),
    (
        0x0021_0d20,
        ov_screen_read_private_message_update,
        "OVScreenReadPrivateMessage_Update",
    ),
    (
        0x0021_0dd0,
        ov_screen_read_private_message_on_child_event,
        "OVScreenReadPrivateMessage_OnChildEvent",
    ),
    (
        0x0020_eda0,
        c_ov_state_hud1p_draw_progress_meter,
        "cOVStateHUD1P_DrawProgressMeter",
    ),
];

// ---------------------------------------------------------------------------
// Callees (address + the generated shard function, which dispatches through
// its override slot). Entries for calls that the generator emitted as
// `call_indirect` are kept for reference next to the `call_ptr` sites.

#[allow(dead_code)]
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
    pub(crate) use crate::game::bx::bxstring2::callee::BE_GET_CHAR_PROFILE_EVENT_RECORD;
    pub(crate) use crate::game::bx::bxstring2::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::bx::bxstring2::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_2;
    pub(crate) use crate::game::bx::bxstring2::callee::CBXSTRING_CTOR_COPY;
    pub(crate) use crate::game::bx::bxstring2::callee::CBXSTRING_DTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAMERA_INIT_FROM_RIDER;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAMERA_RESET_FOR_RIDER;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAM_CONTROLLER_RESTORE_DEFAULT;
    pub(crate) use crate::game::bx::bxstring2::callee::CHUNK_RESOLVE_RELOCATE_DMA;
    pub(crate) use crate::game::bx::bxstring2::callee::CROWD_MAN3D_GROUP_BUILD_MATRICES;
    pub(crate) use crate::game::bx::bxstring2::callee::CUTSCENE_TRANSITION_MAN_FORCE_ON;
    pub(crate) use crate::game::bx::bxstring2::callee::DEBUG_MENU_DISPATCH_ITEM_OP;
    pub(crate) use crate::game::bx::bxstring2::callee::EA_MEM_ALLOC_NAMED_2;
    pub(crate) use crate::game::bx::bxstring2::callee::EA_MEM_FREE as EA_MEM_FREE_2;
    pub(crate) use crate::game::bx::bxstring2::callee::FLAG_MANAGER_GROUP_BUILD_MESH;
    pub(crate) use crate::game::bx::bxstring2::callee::GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::bx::bxstring2::callee::INI_PARSER_EXPECT;
    pub(crate) use crate::game::bx::bxstring2::callee::INI_PARSER_GET_CHAR;
    pub(crate) use crate::game::bx::bxstring2::callee::INI_PARSER_SKIP_WHITESPACE;
    pub(crate) use crate::game::bx::bxstring2::callee::MEMSET;
    pub(crate) use crate::game::bx::bxstring2::callee::MESSAGE_CENTER_GET_INBOX;
    pub(crate) use crate::game::bx::bxstring2::callee::MULTIPARTICLE_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::MULTIPARTICLE_MAN_CLEAR;
    pub(crate) use crate::game::bx::bxstring2::callee::MUSIC_PLAYER_GET_SONG_TAG;
    pub(crate) use crate::game::bx::bxstring2::callee::MUSIC_PLAYER_SET_EVENT_STATE;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_MANAGER;
    pub(crate) use crate::game::bx::bxstring2::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::bx::bxstring2::callee::OPTIONS_SET_VIBRATION;
    pub(crate) use crate::game::bx::bxstring2::callee::OV_SCREEN_PDA_TEMPLATE_CLOSE;
    pub(crate) use crate::game::bx::bxstring2::callee::OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::PATH_DATA_READ_FLOAT_3 as PATH_DATA_READ_FLOAT;
    pub(crate) use crate::game::bx::bxstring2::callee::PATH_DATA_READ_INT;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_SET_WIDGET_ALIGN;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_SET_WRAPPED_TEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::QSORT;
    pub(crate) use crate::game::bx::bxstring2::callee::READ_PRIVATE_MESSAGE_SET_TEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::RIDER_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_LIST_START as SCRIPT_ENGINE_LIST_START_2;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_SET_SKIP_DELAY;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_DEQUEUE_SECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_POP_FREE_NODE;
    pub(crate) use crate::game::bx::bxstring2::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT as SPEECH_SYSTEM_SET_EVENT_CONTEXT_2;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_GET_LANGUAGE_MASK;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_GET_SPEECH_CONTEXT_MASK;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::bx::bxstring2::callee::STRLEN;
    pub(crate) use crate::game::bx::bxstring2::callee::STR_HASH as STR_HASH_2;
    pub(crate) use crate::game::bx::bxstring2::callee::SUPERPIPE_COMPUTE_DESIRED_EYE;
    pub(crate) use crate::game::bx::bxstring2::callee::SUPERPIPE_COMPUTE_LOOK_AT;
    pub(crate) use crate::game::bx::bxstring2::callee::UITEXT_SET_WIDE_STRING;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_ADD_ITEM_STR_ID;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_FIND_ITEM_BY_VALUE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_SET_SELECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_MENU_FIND_FIRST_SELECTABLE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_MENU_SET_SELECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_MGR_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_MGR_FIND_STATE as UI_STATE_MGR_FIND_STATE_2;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_SHUTDOWN as UI_STATE_SHUTDOWN_2;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_UPDATE as UI_STATE_UPDATE_2;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_CPY;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_FROM_ASCII_2 as USTR_FROM_ASCII;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_LEN;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_SPRINTF;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::bx::bxstring2::callee::WSCRIPT_MISSION_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT_2 as WSCRIPT_PROCESS_LIST_PUSH_FRONT;
    pub(crate) use crate::game::main::gamestate::callee::ADF_DATA_SOURCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::ANIM_DATA_BASE_FREE_CHAN_CACHES;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::main::gamestate::callee::BE_ECON_INT_GET_TOTAL_EARNED;
    pub(crate) use crate::game::main::gamestate::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INSTANCE_3 as BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INTERFACE_3 as BE_LIBRARY_GET_INTERFACE_2;
    pub(crate) use crate::game::main::gamestate::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::main::gamestate::callee::BOLTS_FIND_BOLT_INDEX;
    pub(crate) use crate::game::main::gamestate::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_BRAGGING_RIGHTS_INIT_PLAYER_ROWS;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_CHAR_STATS;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::main::gamestate::callee::C_GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::main::gamestate::callee::C_KEYBOARD_POPUP_SET_KEY_ENABLED;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_ALLOC_NAMED_3 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_FREE_4 as EA_MEM_FREE;
    pub(crate) use crate::game::main::gamestate::callee::GAME_ANIM_MAP_UNREGISTER_BANK;
    pub(crate) use crate::game::main::gamestate::callee::GAME_STATE_LOAD_STATE;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_CLEAR_SLOTS;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_INSTALL_VIF_VU_HANDLERS;
    pub(crate) use crate::game::main::gamestate::callee::MEMCARD_STATE_BASE_INSTALL_CALLBACKS;
    pub(crate) use crate::game::main::gamestate::callee::MEM_FREE;
    pub(crate) use crate::game::main::gamestate::callee::MEM_MOVE;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_GET_INPUT_CODE;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_HANDLE_INPUT;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_IS_DISABLED;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_NEXT;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_PREV;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_VALIDATE_SELECTION;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_COUNT_GROUP_MESSAGES;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_GET_MESSAGE_COUNT;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_REMOVE_MESSAGE;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_ON_CHILD_EVENT_3 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_OPEN_KEYBOARD_TEXT_ID;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_REFRESH_LIST;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_COUNT_TEXT;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_SCROLL_ARROWS;
    pub(crate) use crate::game::main::gamestate::callee::RIDER_CLEAR_NEARBY_LIGHTS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_PAUSE_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_RESUME_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_CMD_QUEUE_AT_2 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_SCRIPT_OBJ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_IS_OBJ_BOUND_TO_PROCESS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_START;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_STOP_3 as SCRIPT_ENGINE_LIST_STOP;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_OBJ_CAM_HAS_BINDING;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_OBJ_CAM_IS_BOUND_TO_PROCESS;
    pub(crate) use crate::game::main::gamestate::callee::SECTION_ALLOCATOR_CTOR;
    pub(crate) use crate::game::main::gamestate::callee::SNDSTRM_CLOSE;
    pub(crate) use crate::game::main::gamestate::callee::SPCH_MAKE_ID;
    pub(crate) use crate::game::main::gamestate::callee::SPCH_SHUTDOWN;
    pub(crate) use crate::game::main::gamestate::callee::SPEECH_INSTANCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::main::gamestate::callee::SPHERE_TREE_COLLIDE_AABB;
    pub(crate) use crate::game::main::gamestate::callee::SPHERE_TREE_COLLIDE_SPHERE_TREE;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN_PTR;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_2 as SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_LANGUAGE_MASK;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_RESET_RACE_SOUNDS;
    pub(crate) use crate::game::main::gamestate::callee::STR_COPY;
    pub(crate) use crate::game::main::gamestate::callee::STR_HASH;
    pub(crate) use crate::game::main::gamestate::callee::STR_N_COPY;
    pub(crate) use crate::game::main::gamestate::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_FIRST_2 as UI_LIST_FIRST;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_CTOR_3 as UI_STATE_CTOR;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_SHUTDOWN_2 as UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::main::gamestate::callee::VD_TEXTURE_SINK_CTOR;
}

// ---------------------------------------------------------------------------
// Keyboard popup.

/// `void cKeyboardPopup_SetMaxLength(cKeyboardPopup *this, int max)` (0x1cd088):
/// stores `max` (if 0 <= max < 64), else 0x3f for a non-negative value, else 0.
pub fn c_keyboard_popup_set_max_length(c: &mut Cpu) {
    let this = Addr(abi::arg(c, 0));
    let max = c.g[A1][0];
    c.cycles += 2;
    let non_negative = (max as i64) >= 0;
    c.g[V0][0] = ((max as i64) < 64) as u64;
    if non_negative {
        c.cycles += 2;
        let in_range = c.g[V0][0] != 0;
        c.g[V0][0] = 0x3f;
        c.cycles += 2;
        if in_range {
            this.off(0x60).sw(c, max);
        } else {
            this.off(0x60).sw(c, c.g[V0][0]);
        }
    } else {
        c.cycles += 2;
        this.off(0x60).sw(c, 0);
    }
    abi::ret(c);
}

/// `void cKeyboardPopup_SetKeyEnabled(cKeyboardPopup *this, int index, int value, int key)`
/// (0x1ce3c8): stores `value` into the per-key table at +0x148 (key 0 or 2) or
/// +0x298 (key 1 or 2), leaving `index*4 + this` in v0.
pub fn c_keyboard_popup_set_key_enabled(c: &mut Cpu) {
    let this = c.g[A0][0];
    let index = c.g[A1][0];
    let value = c.g[A2][0];
    let key = c.g[A3][0];
    c.cycles += 2;
    let key_is_zero = key == 0;
    c.g[V0][0] = 2;
    let mut store_148 = false;
    if key_is_zero {
        store_148 = true;
    } else {
        c.cycles += 2;
        let key_not_two = key != c.g[V0][0];
        c.g[V0][0] = sx((key as u32).wrapping_sub(1));
        if !key_not_two {
            store_148 = true;
        }
    }
    if store_148 {
        c.cycles += 4;
        c.g[V0][0] = sx((index as u32) << 2);
        c.g[V0][0] = sx((this as u32).wrapping_add(c.g[V0][0] as u32));
        Addr(c.g[V0][0] as u32).off(0x148).sw(c, value);
        c.g[V0][0] = sx((key as u32).wrapping_sub(1));
    }
    c.cycles += 3;
    c.g[V0][0] = (c.g[V0][0] < 2) as u64;
    let skip = c.g[V0][0] == 0;
    c.g[V0][0] = sx((index as u32) << 2);
    if !skip {
        c.cycles += 2;
        c.g[V0][0] = sx((this as u32).wrapping_add(c.g[V0][0] as u32));
        Addr(c.g[V0][0] as u32).off(0x298).sw(c, value);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void RaceProgress_ResetRider(int rider)` (0x2105b0): stores 1 into the
/// per-rider record at `rider*0x18 + 0x4c8bc8`.
pub fn race_progress_reset_rider(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V1][0] = 0x18;
    c.g[V0][0] = 0x4d0000;
    let scaled = abi::mult(c, abi::r32(c, A0), 0x18);
    c.g[A0][0] = sx(scaled);
    c.g[V0][0] = sx(0x004d_0000u32.wrapping_sub(0x7438));
    c.g[V1][0] = 1;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    Addr(c.g[A0][0] as u32).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `void RaceProgress_ClearPathData(void)` (0x210608): clears the three
/// gp-relative path pointers.
pub fn race_progress_clear_path_data(c: &mut Cpu) {
    c.cycles += 4;
    Addr(abi::r32(c, GP).wrapping_sub(0x9a0)).sw(c, 0);
    Addr(abi::r32(c, GP).wrapping_sub(0x99c)).sw(c, 0);
    Addr(abi::r32(c, GP).wrapping_sub(0x998)).sw(c, 0);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Race progress and game state.

/// `void RaceProgress_SetPathData(RaceProgress* self, FileBlock* path)`
/// (0x2105d0): record the path block, its data and the course-name string
/// in the progress globals, then reset all progress.
pub fn race_progress_set_path_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(4)).lw(c);
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        Addr((c.g[GP][0] as u32).wrapping_sub(0x9a0)).sw(c, c.g[A0][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        Addr((c.g[GP][0] as u32).wrapping_sub(0x99c)).sw(c, c.g[V0][0]);
        Addr((c.g[GP][0] as u32).wrapping_sub(0x998)).sw(c, c.g[A0][0]);
        callee::RACE_PROGRESS_RESET_ALL.call(c, 0x0021_05f8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void HUDMeter_dtor(self, flags)` (0x20eb08): drop the HUD meter
/// (`+0x38` gets its base vtable), decrement the meter count (`gp-0x984`) —
/// clearing the "meters loaded" flag (`gp-0x988`) on the last one — and free
/// `self` when `flags & 1`.
pub fn hud_meter_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = Addr(gp(c, -0x984)).lw(c);
        c.g[V0][0] = 0x47_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(0x47_0000u32.wrapping_add(0x4230));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        let self_ = abi::r32(c, A0);
        Addr(self_.wrapping_add(0x38)).sw(c, c.g[V0][0]);
        let more = c.g[V1][0] != 0;
        Addr(gp(c, -0x984)).sw(c, c.g[V1][0]);
        if !more {
            c.cycles += 1;
            Addr(gp(c, -0x988)).sw(c, 0);
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[A1][0] & 1;
        let keep = c.g[V0][0] == 0;
        frame.restore_ra(c, 0);
        if !keep {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e50, 0x0020_eb44)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenReadPrivateMessage_OnChildEvent(OVScreen*, int id, int ev)`
/// (0x00210dd0): the event 0x16 clears the pending message id at +0xe0.
pub fn ov_screen_read_private_message_on_child_event(c: &mut Cpu) {
    c.cycles += 3;
    let self_ = abi::arg(c, 0);
    let id = abi::arg64(c, 1);
    c.g[V0][0] = 0x16;
    if abi::arg64(c, 2) == 0x16 {
        c.cycles += 3;
        c.g[V0][0] = Addr(p(self_, 0xe0)).lw(c);
        if id == c.g[V0][0] {
            c.sw(p(self_, 0xe0), 0);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void cKeyboardPopup_SetFunctionKeysEnabled(cKeyboardPopup* self, int on)`
/// (0x1ce468): enable/disable the fourteen function keys (indices 0..0xd).
pub fn c_keyboard_popup_set_function_keys_enabled(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A0][0];
        c.g[A1][0] = c.g[S0][0];
        loop {
            c.cycles += 8;
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[A3][0] = 2;
            call_ptr(c, 0x001c_e3c8, 0x001c_e4a0)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S0][0] as i64) < 14i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S0][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001c_e490);
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

/// `OVScreenTrophy_Init(OVScreenTrophy* self)` (0x20e938).
pub fn ov_screen_trophy_init(c: &mut Cpu) {
    guest(c, |c| {
        fe_state_screen_init(c, 0x0047_1cd8, [0x0020_e95c, 0x0020_e970, 0x0020_e988])
    })
}

/// `void HUDMeter_ctor(HUDMeter* self)` (0x20eaa0): install the vtable, set a
/// full-screen 640x480 rectangle and hand it to `HUDMeter_SetRect` as both
/// the source and the destination.
pub fn hud_meter_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        open(c, 0x30);
        c.g[AT][0] = 0x4420_0000;
        c.mtc1(1, 0);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = 0x470000;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[AT][0] = 0x43f0_0000;
        c.mtc1(1, 1);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4230));
        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x38), c.g[V0][0]);
        c.g[A1][0] = c.g[SP][0];
        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0xc));
        c.g[A2][0] = c.g[SP][0];
        c.sw((c.g[SP][0] as u32), 0u64);
        c.g[RA][0] = sx(0x0020_eae8);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0u64);
        call_ptr(c, 0x0020_ec18, 0x0020_eae8)?;
        c.g[V1][0] = c.lw(gp_2(c, -0x984));
        c.g[V0][0] = c.g[S0][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw(gp_2(c, -0x984), c.g[V1][0]);
        close(c, 0x30);
        Ok(())
    })
}

/// `cKeyboardPopup::SetStaticText(self, text)` (0x1ccf00): with no screen yet,
/// keep the string in the popup; otherwise set it on the found text object.
pub fn c_keyboard_popup_set_static_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p_3(c.g[S0][0] as u32, 0x40)).lw(c);
        let have_screen = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A1][0];
        if !have_screen {
            c.cycles += 4;
            c.g[A0][0] = sx(p_3(c.g[S0][0] as u32, 0x424));
            call_ptr(c, 0x0031_84e8, 0x001c_cf28)?;
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 8;
            c.g[A0][0] = 0x460000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6580));
            call_ptr(c, 0x0031_7670, 0x001c_cf3c)?;
            c.g[A0][0] = Addr(p_3(c.g[S0][0] as u32, 0x40)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001c_cf48)?;
            let missing = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[V0][0];
            if !missing {
                c.cycles += 2;
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x003a_0c50, 0x001c_cf58)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenReadPrivateMessage_TransitionIn(self, has_sender)`
/// (0x210b58): when a sender is given, copy its message into the screen;
/// otherwise open the on-screen keyboard for a new message.
pub fn ovscreenreadprivatemessage_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        let no_sender = c.g[A1][0] == 0;
        frame.save_ra(c, 0);
        if !no_sender {
            c.cycles += 7;
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9c));
            c.g[A0][0] = lw_at(c, gw(c, -0x204));
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = 0x41;
            call_ptr(c, callee::C_GAME_COMM_RECV_GAME_PACKET.addr, 0x0021_0b84)?;
            let empty = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !empty {
                c.cycles += 5;
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = 0;
                call_ptr(c, 0x0021_0bd8, 0x0021_0b98)?;
                c.cycles += 5;
                c.g[V0][0] = 1;
                frame.restore(c, 16, 0x20);
                frame.restore(c, 17, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
        c.cycles += 8;
        c.g[A2][0] = 0x4a2560;
        c.g[A1][0] = 0;
        c.g[T0][0] = 0;
        c.g[A3][0] = c.g[A2][0];
        c.g[T1][0] = 0;
        c.g[T2][0] = 0x3d;
        call_ptr(c, 0x0021_0f40, 0x0021_0bc0)?;
        c.cycles += 1;
        frame.restore(c, 16, 0x20);
        c.cycles += 5;
        c.g[V0][0] = 1;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `OVScreenReadPrivateMessage_Update` (0x210d20): run the overlay update; if
// the net object is idle, close its popups and set the message flag; when the
// screen has content, read the pending message into it.
pub fn ov_screen_read_private_message_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::OV_STATE_OVERLAY_UPDATE.call(c, 0x0021_0d38)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
        let no_net = c.g[V1][0] == 0;
        if no_net {
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x68));
            let idle = c.g[V0][0] == 0;
            c.g[A0][0] = 0u64;
            let mut closed = idle;
            if !idle {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x64));
                closed = c.g[V0][0] == 0;
            }
            if closed {
                c.cycles += 1;
                c.g[A0][0] = 0x1;
            }
            c.cycles += 2;
            if !closed {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            } else {
                c.cycles += 9;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                call_ptr(c, 0x0039f698, 0x0021_0d74)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                c.g[V0][0] = 0xffff_ffff_ffff_c0ff;
                c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
                c.g[V1][0] = c.g[V1][0] | 0x780;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
            }
        }
        c.cycles += 2;
        let empty = c.g[V0][0] == 0;
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9c));
        if !empty {
            c.cycles += 6;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = 0x41;
            call_ptr(c, 0x00258160, 0x0021_0da4)?;
            let no_msg = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !no_msg {
                c.cycles += 3;
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = 0u64;
                callee::READ_PRIVATE_MESSAGE_SET_TEXT.call(c, 0x0021_0db8)?;
            }
        }
        c.cycles += 5;
        c.lq(16, frame.at(c, 0x20));
        c.lq(17, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cOVStateHUD1P_LoadMeterAssets(self, a1, a2)` (0x20eb50): find the
/// meter textures/font and apply the rect.
pub fn cov_state_hud1p_load_meter_assets(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = Addr(gp_3(c, -0x988)).lw(c);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S5, 0x10);
        c.g[S4][0] = c.g[A1][0];
        frame.save(c, S0, 0x60);
        c.g[S5][0] = c.g[A2][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        let ready = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if !ready {
            c.cycles += 7;
            c.g[V0][0] = sx(world(c));
            c.g[V1][0] = 0x004d_0000;
            c.g[S0][0] = 0x004c_8c58;
            c.g[S2][0] = 0xa;
            c.g[A0][0] = Addr(p_4(c.g[V0][0], 0x84)).lw(c);
            c.g[S1][0] = Addr(p_4(c.g[A0][0], 0x48)).lw(c);
            loop {
                c.cycles += 7;
                c.g[A1][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x58));
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
                call_ptr(c, 0x0039_8380, 0x0020_ebb0)?;
                Addr(p_4(c.g[S0][0], 0xc)).sw(c, c.g[V0][0]);
                let more = (c.g[S2][0] as i64) >= 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0020_eba0);
                }
            }
            c.cycles += 10;
            c.g[A0][0] = 0x004a_21f0;
            c.g[S0][0] = Addr(p_4(c.g[S1][0], 0x8)).lw(c);
            call_ptr(c, 0x0031_7670, 0x0020_ebcc)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_83f0, 0x0020_ebd8)?;
            c.g[V1][0] = 1;
            Addr(gp_3(c, -0x98c)).sw(c, c.g[V0][0]);
            Addr(gp_3(c, -0x988)).sw(c, c.g[V1][0]);
        }
        c.cycles += 13;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = c.g[S4][0];
        c.g[A2][0] = c.g[S5][0];
        call_ptr(c, 0x0020_ec18, 0x0020_ebf4)?;
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
// cKeyboardPopup_SetText (0x00001cb418).

/// `cKeyboardPopup_SetText`: put the edited text into the keyboard popup and rebuild its entry list.
pub fn c_keyboard_popup_set_text(c: &mut Cpu) {
    guest(c, |c| {
        'b_1cb488: {
            c.cycles += 20;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S0][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S2][0] = 0x1;
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x74));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x418), c.g[S2][0]);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x004166f4, 0x001cb444)?;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x00416810, 0x001cb44c)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x138), 0u64);
            c.g[A0][0] = c.g[S1][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0x134), c.g[V0][0]);
            call_ptr(c, 0x00416810, 0x001cb45c)?;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x58));
            let t = c.g[V1][0] == 0u64;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x13c), c.g[V0][0]);
            if t {
                break 'b_1cb488;
            } else {
                c.cycles += 8;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x00416810, 0x001cb470)?;
                c.g[A2][0] = 0u64;
                c.g[A0][0] = c.g[S0][0];
                if c.g[V0][0] != 0 {
                    c.g[A2][0] = c.g[S2][0];
                }
                c.g[A1][0] = 0x4b;
                c.g[A3][0] = 0x2;
                call_ptr(c, 0x001ce3c8, 0x001cb488)?;
                break 'b_1cb488;
            }
        }
        'b_1cb4c8: {
            'b_1cb4b8: {
                c.cycles += 6;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x00416810, 0x001cb490)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x64));
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = 0x37;
                if t {
                    break 'b_1cb4b8;
                } else {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0x2;
                    call_ptr(c, 0x001ce3c8, 0x001cb4b0)?;
                    let t = true;

                    if t {
                        break 'b_1cb4c8;
                    } else {
                        break 'b_1cb4b8;
                    }
                }
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = 0x1;
            c.g[A3][0] = 0x2;
            call_ptr(c, 0x001ce3c8, 0x001cb4c8)?;
            break 'b_1cb4c8;
        }
        'b_1cb4e4: {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x001cd708, 0x001cb4d0)?;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x41c));
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.g[S0][0];
            if t {
                break 'b_1cb4e4;
            } else {
                c.cycles += 2;
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x001cc848, 0x001cb4e4)?;
                break 'b_1cb4e4;
            }
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
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

/// `void func_0020E9A0(OVScreen*, Widget* w, int action)` (0x20e9a0): the
/// trophy screen's widget-action handler.
pub fn ovscreen_trophy_on_widget_action(c: &mut Cpu) {
    guest(c, |c| ov_screen_widget_action(c, 0x0020_ea08))
}

/// `void cOVStateHUD1P::DrawProgressMeter(Rider* player)` (0x20eda0,
/// `src/fe/ovstatetrophy.cpp`, asm only in the reference): the race
/// progress meter of the 1P HUD.
///
/// Skipped when the HUD is hidden (`gp-0x990`), the meter is off
/// (`gp-0x994`), there is no player or no meter definition (`gp-0x9a0`).
/// Otherwise it asks the player for its racer slot (rider vtable +0x38),
/// slews each racer's displayed position (table 0x4c8bc8, six 0x18-byte
/// entries: +0x8 course distance, +0x10 shown position, +0x14 visible)
/// toward its distance (clamped to +-2000, stepped by the `.lit4` speeds
/// `gp-0x54ec`/`gp-0x54e8`), opens a sprite batch on the renderer
/// (`gp-0x854`, vtable at +0x10d8; GS packet pointer at +0xe84), draws the
/// percentage text (`sprintf("%d%%")` + `Font_DrawTextCentered` with the
/// HUD font at `gp-0x98c`), then the bar, every racer marker that falls in
/// the visible window (player distance -5000 .. +5000), the course markers
/// listed at `gp-0x998` (count at meter def +0x8), closes the batch and
/// remembers the player slot at `self+0x34`.
///
/// The body keeps the original's block structure (each block's cycle
/// charge, the scratch registers it leaves, the `.lit4` reads through
/// `gp`), with the calls through the call protocol and the loop back edges'
/// event checks through [`event_due`]/[`deliver_event_here`].
pub fn c_ov_state_hud1p_draw_progress_meter(c: &mut Cpu) {
    guest(c, |c| {
        'b_20faf8: {
            'b_20faf4: {
                c.cycles += 21;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x1d0));
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x990));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x170));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
                c.g[S1][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x160));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x150));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x140));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x130));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x100));
                c.sd((c.g[SP][0] as u32).wrapping_add(0xf0), c.g[RA][0]);
                c.swc1(26, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                c.swc1(25, (c.g[SP][0] as u32).wrapping_add(0x1b8));
                c.swc1(24, (c.g[SP][0] as u32).wrapping_add(0x1b0));
                c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x1a8));
                c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x198));
                let t = c.g[V0][0] != 0u64;
                c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x190));
                if t {
                    break 'b_20faf4;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x994));
                    let t = c.g[V0][0] == 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
                    if t {
                        break 'b_20faf8;
                    } else {
                        c.cycles += 2;
                        let t = c.g[A1][0] == 0u64;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x998));
                        if t {
                            break 'b_20faf8;
                        } else {
                            c.cycles += 3;
                            c.g[S7][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x9a0));
                            let t = c.g[S7][0] == 0u64;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[V0][0]);
                            if t {
                                break 'b_20faf8;
                            } else {
                                c.cycles += 33;
                                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x6c0));
                                c.g[S0][0] = 0x4c0000;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x6c0));
                                c.g[S5][0] = 0x4c0000;
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x38));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x3c));
                                let t = c.g[V0][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x0020ee34)?;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[S6][0] = c.g[V0][0];
                                c.g[V1][0] = 0x18;
                                c.g[V0][0] = 0x4d0000;
                                c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                c.mult(0, 3, 22, 3);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7438));
                                c.g[AT][0] = 0xffffffffc4fa0000;
                                c.mtc1(1, 7);
                                c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                                c.g[FP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                                c.g[AT][0] = 0x44fa0000;
                                c.mtc1(1, 6);
                                c.g[A3][0] = 0x1;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0xe4), c.g[A1][0]);
                                c.g[A0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                c.g[V1][0] = c.g[A0][0];
                                c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x54ec));
                                c.lwc1(20, (c.g[V1][0] as u32).wrapping_add(0x8));
                                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe4));
                                c.g[A1][0] = 0u64;
                                c.lwc1(4, (c.g[A0][0] as u32).wrapping_add(0xc));
                                c.g[S3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x78));
                                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x14));
                                'l_20ee98: loop {
                                    'b_20eea4: {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x34));
                                        if c.g[V0][0] != c.g[S6][0] {
                                            c.sw((c.g[V1][0] as u32), 0u64);
                                            break 'b_20eea4;
                                        } else {
                                            break 'b_20eea4;
                                        }
                                    }
                                    'b_20ef30: {
                                        c.cycles += 3;
                                        c.g[V0][0] =
                                            ((c.g[A1][0] as i64) < (c.g[S3][0] as i64)) as u64;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                            break 'b_20ef30;
                                        } else {
                                            'b_20eeec: {
                                                'b_20eed4: {
                                                    c.cycles += 5;
                                                    c.lwc1(
                                                        0,
                                                        (c.g[V1][0] as u32).wrapping_sub(0x8),
                                                    );
                                                    c.sub_s(0, 0, 4);
                                                    c.c_lt_s(0, 7);
                                                    let t = !c.fcc();

                                                    if t {
                                                        break 'b_20eed4;
                                                    } else {
                                                        c.cycles += 4;
                                                        c.g[AT][0] = 0xffffffffc4fa0000;
                                                        c.mtc1(1, 0);
                                                        let t = true;
                                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                                        if t {
                                                            break 'b_20eeec;
                                                        } else {
                                                            break 'b_20eed4;
                                                        }
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.c_lt_s(6, 0);
                                                let t = !c.fcc();
                                                c.g[V0][0] = c.lw((c.g[V1][0] as u32));
                                                if t {
                                                    break 'b_20eeec;
                                                } else {
                                                    c.cycles += 2;
                                                    c.g[AT][0] = 0x44fa0000;
                                                    c.mtc1(1, 0);
                                                    break 'b_20eeec;
                                                }
                                            }
                                            'b_20ef28: {
                                                c.cycles += 2;
                                                if c.g[V0][0] == 0u64 {
                                                    c.swc1(0, (c.g[A2][0] as u32));
                                                    break 'b_20ef28;
                                                } else {
                                                    'b_20ef24: {
                                                        'b_20ef14: {
                                                            c.cycles += 6;
                                                            c.lwc1(2, (c.g[A2][0] as u32));
                                                            c.lwc1(
                                                                3,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x54e8),
                                                            );
                                                            c.sub_s(1, 0, 2);
                                                            c.c_lt_s(3, 1);
                                                            let t = !c.fcc();

                                                            if t {
                                                                break 'b_20ef14;
                                                            } else {
                                                                c.cycles += 2;
                                                                let t = true;
                                                                c.add_s(0, 2, 3);
                                                                if t {
                                                                    break 'b_20ef24;
                                                                } else {
                                                                    break 'b_20ef14;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 4;
                                                        c.c_lt_s(1, 5);
                                                        if c.fcc() {
                                                            c.sub_s(0, 2, 3);
                                                            break 'b_20ef24;
                                                        } else {
                                                            break 'b_20ef24;
                                                        }
                                                    }
                                                    c.cycles += 1;
                                                    c.swc1(0, (c.g[A2][0] as u32));
                                                    break 'b_20ef28;
                                                }
                                            }
                                            c.cycles += 2;
                                            c.sw((c.g[V1][0] as u32), c.g[A3][0]);
                                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                                            break 'b_20ef30;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x18));
                                    c.g[V0][0] = ((c.g[A1][0] as i64) < 6i64) as u64;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x18));
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x0020ee98);
                                        }
                                        continue 'l_20ee98;
                                    } else {
                                        c.cycles += 2;
                                        c.g[V0][0] = 0x3;
                                        c.g[V1][0] = 0xffffffffffffffff;
                                        'l_20ef48: loop {
                                            c.cycles += 7;
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                            let t = c.g[V0][0] != c.g[V1][0];
                                            c.g[A0][0] = 0x80;
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x0020ef48);
                                                }
                                                continue 'l_20ef48;
                                            } else {
                                                c.cycles += 5;
                                                c.g[AT][0] = 0x3f800000;
                                                c.mtc1(1, 0);
                                                c.g[V1][0] = c.g[SP][0];
                                                c.g[A1][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0xc0));
                                                c.sw(
                                                    (c.g[V1][0] as u32).wrapping_add(0x10),
                                                    c.g[A0][0],
                                                );
                                                'l_20ef78: loop {
                                                    c.cycles += 8;
                                                    c.sw(
                                                        (c.g[V1][0] as u32).wrapping_add(0x14),
                                                        c.g[A0][0],
                                                    );
                                                    c.sw(
                                                        (c.g[V1][0] as u32).wrapping_add(0x18),
                                                        c.g[A0][0],
                                                    );
                                                    c.sw(
                                                        (c.g[V1][0] as u32).wrapping_add(0x1c),
                                                        c.g[A0][0],
                                                    );
                                                    c.swc1(
                                                        0,
                                                        (c.g[V1][0] as u32).wrapping_add(0x8),
                                                    );
                                                    c.g[V1][0] =
                                                        sx((c.g[V1][0] as u32).wrapping_add(0x30));
                                                    c.g[V0][0] = ((c.g[V1][0] as i64)
                                                        < (c.g[A1][0] as i64))
                                                        as u64;
                                                    if c.g[V0][0] != 0u64 {
                                                        c.sw(
                                                            (c.g[V1][0] as u32).wrapping_add(0x10),
                                                            c.g[A0][0],
                                                        );
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x0020ef78);
                                                        }
                                                        continue 'l_20ef78;
                                                    } else {
                                                        'b_20f114: {
                                                            c.cycles += 86;
                                                            c.g[A1][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                            c.g[V0][0] = c.lw((c.g[A1][0] as u32)
                                                                .wrapping_add(0x10d8));
                                                            c.g[A0][0] = c.lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0xf8));
                                                            c.g[V1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0xfc));
                                                            let t = c.g[V1][0] as u32;
                                                            c.g[A0][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add((c.g[A0][0] as u32)));
                                                            call_ptr(c, t, 0x0020efb0)?;
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                            c.g[T0][0] = 0xfffffffffe7f0000;
                                                            c.g[T0][0] = c.g[T0][0] | 0xffff;
                                                            c.g[T4][0] = 0x1000000;
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[T1][0] = 0xffffffffffcf0000;
                                                            c.g[T1][0] = c.g[T1][0] | 0xffff;
                                                            c.g[T3][0] = 0xfffffffffff00000;
                                                            c.ldl(
                                                                5,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(5, (c.g[V0][0] as u32));
                                                            c.ldl(
                                                                6,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                6,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[A3][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.sdl(
                                                                5,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x1b),
                                                            );
                                                            c.sdr(
                                                                5,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x14),
                                                            );
                                                            c.sdl(
                                                                6,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x23),
                                                            );
                                                            c.sdr(
                                                                6,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x1c),
                                                            );
                                                            c.sw(
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x24),
                                                                c.g[A3][0],
                                                            );
                                                            c.g[T3][0] = c.g[T3][0] | 0xfff;
                                                            c.g[T2][0] = 0x10000;
                                                            c.g[T5][0] = 0xffffffffffffff83;
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[T2][0] = c.g[T2][0] | 0x4000;
                                                            c.g[A2][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                            c.g[A1][0] = 0x500000;
                                                            c.g[V0][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x14));
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_sub(0xe60));
                                                            c.sw(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0xe84),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[V0][0] = 0x500000;
                                                            c.g[V1][0] = c.lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[T6][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0x1420));
                                                            c.ldl(
                                                                4,
                                                                (c.g[T6][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(4, (c.g[T6][0] as u32));
                                                            c.ldl(
                                                                6,
                                                                (c.g[T6][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                6,
                                                                (c.g[T6][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.g[A3][0] = c.lw((c.g[T6][0] as u32)
                                                                .wrapping_add(0x10));
                                                            c.sdl(
                                                                4,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.sdr(4, (c.g[V1][0] as u32));
                                                            c.sdl(
                                                                6,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.sdr(
                                                                6,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sw(
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x10),
                                                                c.g[A3][0],
                                                            );
                                                            c.g[A3][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                            c.g[A0][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.g[V0][0] = c.g[V0][0] & c.g[T0][0];
                                                            c.g[V0][0] = c.g[V0][0] | c.g[T4][0];
                                                            c.sw(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x4),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[V1][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
                                                            c.sw(
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x4),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[A0][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[V0][0] = c.lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.g[V0][0] = c.g[V0][0] & c.g[T3][0];
                                                            c.g[V0][0] = c.g[V0][0] | c.g[T2][0];
                                                            c.sw(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x4),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[A2][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[V0][0] = c.lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0x4));
                                                            c.g[V0][0] = c.g[V0][0] & c.g[T5][0];
                                                            c.g[V0][0] = c.g[V0][0] | 0x14;
                                                            c.sw(
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x4),
                                                                c.g[V0][0],
                                                            );
                                                            c.g[V1][0] = c.lw((c.g[A3][0] as u32)
                                                                .wrapping_add(0x10d8));
                                                            c.g[A0][0] = c.lh((c.g[V1][0] as u32)
                                                                .wrapping_add(0x110));
                                                            c.g[A2][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x114));
                                                            let t = c.g[A2][0] as u32;
                                                            c.g[A0][0] = sx((c.g[A3][0] as u32)
                                                                .wrapping_add((c.g[A0][0] as u32)));
                                                            call_ptr(c, t, 0x0020f0b8)?;
                                                            c.g[V0][0] = 0x18;
                                                            c.lwc1(
                                                                0,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x54e4),
                                                            );
                                                            c.mult(0, 2, 22, 2);
                                                            c.g[A0][0] = 0x4d0000;
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_sub(0x7438));
                                                            c.sub_s(25, 20, 0);
                                                            c.lwc1(
                                                                1,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x54e0),
                                                            );
                                                            c.lwc1(
                                                                2,
                                                                (c.g[S7][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            c.g[V0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add((c.g[V0][0] as u32)));
                                                            c.g[V1][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_add(0x1dc4));
                                                            c.lwc1(
                                                                0,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.div_s(26, 0, 2);
                                                            let t = c.g[V1][0] != 0u64;
                                                            c.add_s(24, 25, 1);
                                                            if t {
                                                                break 'b_20f114;
                                                            } else {
                                                                c.cycles += 9;
                                                                c.g[AT][0] = 0x3f800000;
                                                                c.mtc1(1, 0);
                                                                c.g[V0][0] = sx((c.g[S0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x6868));
                                                                c.g[V1][0] = 0x1;
                                                                c.swc1(
                                                                    0,
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0xc),
                                                                );
                                                                c.sw(
                                                                    (c.g[GP][0] as u32)
                                                                        .wrapping_add(0x1dc4),
                                                                    c.g[V1][0],
                                                                );
                                                                c.swc1(
                                                                    0,
                                                                    (c.g[S0][0] as u32)
                                                                        .wrapping_add(0x6868),
                                                                );
                                                                c.swc1(
                                                                    0,
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x4),
                                                                );
                                                                c.swc1(
                                                                    0,
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x8),
                                                                );
                                                                break 'b_20f114;
                                                            }
                                                        }
                                                        'b_20f148: {
                                                            c.cycles += 3;
                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_add(0x1dc8));
                                                            let t = c.g[V0][0] != 0u64;
                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_add(0x1dd8));
                                                            if t {
                                                                break 'b_20f148;
                                                            } else {
                                                                c.cycles += 10;
                                                                c.g[V0][0] = sx((c.g[S5][0]
                                                                    as u32)
                                                                    .wrapping_add(0x6878));
                                                                c.g[AT][0] = 0x3f800000;
                                                                c.mtc1(1, 0);
                                                                c.sw(
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0xc),
                                                                    0u64,
                                                                );
                                                                c.g[V1][0] = 0x1;
                                                                c.swc1(
                                                                    0,
                                                                    (c.g[S5][0] as u32)
                                                                        .wrapping_add(0x6878),
                                                                );
                                                                c.sw(
                                                                    (c.g[GP][0] as u32)
                                                                        .wrapping_add(0x1dc8),
                                                                    c.g[V1][0],
                                                                );
                                                                c.sw(
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x4),
                                                                    0u64,
                                                                );
                                                                c.sw(
                                                                    (c.g[V0][0] as u32)
                                                                        .wrapping_add(0x8),
                                                                    0u64,
                                                                );
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_add(0x1dd8));
                                                                break 'b_20f148;
                                                            }
                                                        }
                                                        'b_20f164: {
                                                            c.cycles += 2;
                                                            let t = c.g[V0][0] != 0u64;
                                                            c.g[V0][0] = 0x1;
                                                            if t {
                                                                break 'b_20f164;
                                                            } else {
                                                                c.cycles += 5;
                                                                c.g[AT][0] = 0x40000000;
                                                                c.mtc1(1, 0);
                                                                c.sw(
                                                                    (c.g[GP][0] as u32)
                                                                        .wrapping_add(0x1dd8),
                                                                    c.g[V0][0],
                                                                );
                                                                c.swc1(
                                                                    0,
                                                                    (c.g[GP][0] as u32)
                                                                        .wrapping_add(0x1dd4),
                                                                );
                                                                c.swc1(
                                                                    0,
                                                                    (c.g[GP][0] as u32)
                                                                        .wrapping_add(0x1dd0),
                                                                );
                                                                break 'b_20f164;
                                                            }
                                                        }
                                                        'b_20f3f0: {
                                                            c.cycles += 67;
                                                            c.g[AT][0] = 0x42c80000;
                                                            c.mtc1(1, 0);
                                                            c.g[V0][0] = 0x4d0000;
                                                            c.g[AT][0] = 0x3f000000;
                                                            c.mtc1(1, 1);
                                                            c.g[A1][0] = 0x4a0000;
                                                            c.mul_s(0, 26, 0);
                                                            c.g[A0][0] = c.g[FP][0];
                                                            c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                                .wrapping_add(0x2690));
                                                            c.g[S4][0] = sx((c.g[V0][0] as u32)
                                                                .wrapping_sub(0x7368));
                                                            c.add_s(0, 0, 1);
                                                            c.cvt_w_s(1, 0);
                                                            c.mfc1(6, 1);
                                                            c.g[S2][0] = 0u64;
                                                            call_ptr(c, 0x00417828, 0x0020f1a0)?;
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x98c));
                                                            c.g[A3][0] = 0xfffffffffffffc1f;
                                                            c.lwc1(
                                                                2,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_sub(0x54dc),
                                                            );
                                                            c.g[A1][0] = c.g[FP][0];
                                                            c.g[T0][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x6868));
                                                            c.ldl(
                                                                2,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(2, (c.g[T0][0] as u32));
                                                            c.ldl(
                                                                3,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                3,
                                                                (c.g[T0][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                2,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x47),
                                                            );
                                                            c.sdr(
                                                                2,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x40),
                                                            );
                                                            c.sdl(
                                                                3,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x4f),
                                                            );
                                                            c.sdr(
                                                                3,
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x48),
                                                            );
                                                            c.g[V1][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x98c));
                                                            c.lwc1(
                                                                0,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x30),
                                                            );
                                                            c.lwc1(
                                                                1,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x34),
                                                            );
                                                            c.mul_s(0, 0, 2);
                                                            c.mul_s(1, 1, 2);
                                                            c.swc1(
                                                                0,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x38),
                                                            );
                                                            c.swc1(
                                                                1,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x3c),
                                                            );
                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x98c));
                                                            c.g[A2][0] = sx((c.g[S5][0] as u32)
                                                                .wrapping_add(0x6878));
                                                            c.ldl(
                                                                14,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x7),
                                                            );
                                                            c.ldr(14, (c.g[A2][0] as u32));
                                                            c.ldl(
                                                                3,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0xf),
                                                            );
                                                            c.ldr(
                                                                3,
                                                                (c.g[A2][0] as u32)
                                                                    .wrapping_add(0x8),
                                                            );
                                                            c.sdl(
                                                                14,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x57),
                                                            );
                                                            c.sdr(
                                                                14,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x50),
                                                            );
                                                            c.sdl(
                                                                3,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x5f),
                                                            );
                                                            c.sdr(
                                                                3,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x58),
                                                            );
                                                            c.g[V1][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x98c));
                                                            c.ldl(
                                                                8,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_add(0x1dd7),
                                                            );
                                                            c.ldr(
                                                                8,
                                                                (c.g[GP][0] as u32)
                                                                    .wrapping_add(0x1dd0),
                                                            );
                                                            c.sdl(
                                                                8,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x2f),
                                                            );
                                                            c.sdr(
                                                                8,
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x28),
                                                            );
                                                            c.g[A2][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x98c));
                                                            c.g[V1][0] = c.lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0xe84));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[V0][0] = c.g[V0][0] & c.g[A3][0];
                                                            c.g[V0][0] = c.g[V0][0] | 0x140;
                                                            c.sw(
                                                                (c.g[V1][0] as u32)
                                                                    .wrapping_add(0x8),
                                                                c.g[V0][0],
                                                            );
                                                            c.lwc1(12, (c.g[S1][0] as u32));
                                                            c.lwc1(
                                                                13,
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x4),
                                                            );
                                                            call_ptr(c, 0x00392908, 0x0020f254)?;
                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x98c));
                                                            c.lwc1(
                                                                0,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x30),
                                                            );
                                                            c.lwc1(
                                                                1,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x34),
                                                            );
                                                            c.swc1(
                                                                0,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x38),
                                                            );
                                                            c.swc1(
                                                                1,
                                                                (c.g[V0][0] as u32)
                                                                    .wrapping_add(0x3c),
                                                            );
                                                            let t = (c.g[S3][0] as i64) <= 0;
                                                            c.g[S0][0] = c.lw((c.g[S4][0] as u32)
                                                                .wrapping_add(0xc));
                                                            if t {
                                                                break 'b_20f3f0;
                                                            } else {
                                                                c.cycles += 6;
                                                                c.g[AT][0] = 0x3f800000;
                                                                c.mtc1(1, 20);
                                                                c.g[S5][0] = 0x4d0000;
                                                                c.g[S5][0] = sx((c.g[S5][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x7438));
                                                                c.g[FP][0] = 0xfffffffffffffc1f;
                                                                c.g[V0][0] = 0x18;
                                                                'l_20f288: loop {
                                                                    'b_20f3e4: {
                                                                        'b_20f2b8: {
                                                                            c.cycles += 7;
                                                                            c.mult(0, 2, 18, 2);
                                                                            c.g[V1][0] =
                                                                                sx((c.g[S5][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        (c.g[V0][0]
                                                                                            as u32),
                                                                                    ));
                                                                            c.lwc1(
                                                                                1,
                                                                                (c.g[V1][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x8,
                                                                                    ),
                                                                            );
                                                                            c.c_le_s(25, 1);
                                                                            let t = !c.fcc();
                                                                            c.g[A1][0] = 0x4d0000;
                                                                            if t {
                                                                                break 'b_20f2b8;
                                                                            } else {
                                                                                c.cycles += 4;
                                                                                c.c_le_s(1, 24);
                                                                                if c.fcc() {
                                                                                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                                                                    break 'b_20f3e4;
                                                                                } else {
                                                                                    c.cycles += 1;
                                                                                    c.g[A1][0] =
                                                                                        0x4d0000;
                                                                                    break 'b_20f2b8;
                                                                                }
                                                                            }
                                                                        }
                                                                        'b_20f30c: {
                                                                            c.cycles += 18;
                                                                            c.lwc1(
                                                                                4,
                                                                                (c.g[GP][0] as u32)
                                                                                    .wrapping_sub(
                                                                                        0x54d8,
                                                                                    ),
                                                                            );
                                                                            c.g[A1][0] =
                                                                                sx((c.g[A1][0]
                                                                                    as u32)
                                                                                    .wrapping_sub(
                                                                                        0x7428,
                                                                                    ));
                                                                            c.c_lt_s(24, 1);
                                                                            c.g[V0][0] =
                                                                                sx((c.g[V0][0]
                                                                                    as u32)
                                                                                    .wrapping_add(
                                                                                        (c.g[A1][0]
                                                                                            as u32),
                                                                                    ));
                                                                            c.lwc1(
                                                                                5,
                                                                                (c.g[S0][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x18,
                                                                                    ),
                                                                            );
                                                                            c.lwc1(
                                                                                0,
                                                                                (c.g[V0][0] as u32),
                                                                            );
                                                                            c.lwc1(
                                                                                3,
                                                                                (c.g[S1][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x10,
                                                                                    ),
                                                                            );
                                                                            c.mov_s(8, 5);
                                                                            c.neg_s(0, 0);
                                                                            c.lwc1(
                                                                                2,
                                                                                (c.g[S4][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x4,
                                                                                    ),
                                                                            );
                                                                            c.g[AT][0] = 0x3f000000;
                                                                            c.mtc1(1, 1);
                                                                            c.lwc1(
                                                                                7,
                                                                                (c.g[S0][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xc,
                                                                                    ),
                                                                            );
                                                                            c.mul_s(0, 0, 4);
                                                                            c.lwc1(
                                                                                6,
                                                                                (c.g[S1][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0xc,
                                                                                    ),
                                                                            );
                                                                            c.mul_s(2, 2, 1);
                                                                            let t = !c.fcc();
                                                                            c.add_s(4, 3, 0);
                                                                            if t {
                                                                                break 'b_20f30c;
                                                                            } else {
                                                                                c.cycles += 3;
                                                                                c.mov_s(5, 7);
                                                                                c.lwc1(6, (c.g[S1][0] as u32).wrapping_add(0x8));
                                                                                c.mov_s(7, 8);
                                                                                break 'b_20f30c;
                                                                            }
                                                                        }
                                                                        c.cycles += 54;
                                                                        c.lwc1(
                                                                            0,
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(0x10),
                                                                        );
                                                                        c.sub_s(3, 4, 2);
                                                                        c.swc1(
                                                                            5,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x4),
                                                                        );
                                                                        c.add_s(4, 4, 2);
                                                                        c.swc1(
                                                                            0,
                                                                            (c.g[SP][0] as u32),
                                                                        );
                                                                        c.g[A1][0] = 0x4;
                                                                        c.g[A2][0] = c.g[SP][0];
                                                                        c.g[A3][0] = 0u64;
                                                                        c.lwc1(
                                                                            1,
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(0x14),
                                                                        );
                                                                        c.lwc1(
                                                                            2,
                                                                            (c.g[S4][0] as u32)
                                                                                .wrapping_add(0x8),
                                                                        );
                                                                        c.swc1(
                                                                            5,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x34),
                                                                        );
                                                                        c.swc1(
                                                                            1,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x30),
                                                                        );
                                                                        c.add_s(2, 6, 2);
                                                                        c.lwc1(
                                                                            0,
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(0x10),
                                                                        );
                                                                        c.swc1(
                                                                            7,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x64),
                                                                        );
                                                                        c.swc1(
                                                                            0,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x60),
                                                                        );
                                                                        c.lwc1(
                                                                            1,
                                                                            (c.g[S0][0] as u32)
                                                                                .wrapping_add(0x14),
                                                                        );
                                                                        c.swc1(
                                                                            3,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd0),
                                                                        );
                                                                        c.sw(
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd8),
                                                                            0u64,
                                                                        );
                                                                        c.swc1(
                                                                            6,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd4),
                                                                        );
                                                                        c.swc1(
                                                                            20,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xdc),
                                                                        );
                                                                        c.swc1(
                                                                            1,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x90),
                                                                        );
                                                                        c.lq(
                                                                            4,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd0),
                                                                        );
                                                                        c.swc1(
                                                                            4,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd0),
                                                                        );
                                                                        c.sw(
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd8),
                                                                            0u64,
                                                                        );
                                                                        c.swc1(
                                                                            7,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x94),
                                                                        );
                                                                        c.lq(
                                                                            3,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd0),
                                                                        );
                                                                        c.swc1(
                                                                            3,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd0),
                                                                        );
                                                                        c.swc1(
                                                                            2,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd4),
                                                                        );
                                                                        c.sw(
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd8),
                                                                            0u64,
                                                                        );
                                                                        c.sq(
                                                                            3,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x50),
                                                                        );
                                                                        c.lq(
                                                                            2,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd0),
                                                                        );
                                                                        c.swc1(
                                                                            4,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd0),
                                                                        );
                                                                        c.sq(
                                                                            2,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x80),
                                                                        );
                                                                        c.sq(
                                                                            4,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x20),
                                                                        );
                                                                        c.sw(
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd8),
                                                                            0u64,
                                                                        );
                                                                        c.lq(
                                                                            2,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xd0),
                                                                        );
                                                                        c.g[T0][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_sub(0x854));
                                                                        c.sq(
                                                                            2,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0xb0),
                                                                        );
                                                                        c.g[A0][0] = c.lw((c.g[T0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0xe84));
                                                                        c.g[V0][0] = c
                                                                            .lw((c.g[A0][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                        c.g[V0][0] =
                                                                            c.g[V0][0] & c.g[FP][0];
                                                                        c.g[V0][0] =
                                                                            c.g[V0][0] | 0x140;
                                                                        c.sw(
                                                                            (c.g[A0][0] as u32)
                                                                                .wrapping_add(0x8),
                                                                            c.g[V0][0],
                                                                        );
                                                                        c.g[V1][0] = c.lw((c.g[T0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0xe84));
                                                                        c.g[V0][0] =
                                                                            c.lw(
                                                                                (c.g[S0][0] as u32)
                                                                            );
                                                                        c.sh(
                                                                            (c.g[V1][0] as u32)
                                                                                .wrapping_add(0x10),
                                                                            c.g[V0][0],
                                                                        );
                                                                        c.g[T0][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_sub(0x854));
                                                                        c.g[V0][0] = c.lw((c.g[T0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10d8));
                                                                        c.g[A0][0] = c.lh((c.g[V0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x238));
                                                                        c.g[V1][0] = c.lw((c.g[V0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x23c));
                                                                        let t = c.g[V1][0] as u32;
                                                                        c.g[A0][0] =
                                                                            sx((c.g[T0][0] as u32)
                                                                                .wrapping_add(
                                                                                    (c.g[A0][0]
                                                                                        as u32),
                                                                                ));
                                                                        call_ptr(c, t, 0x0020f3e0)?;
                                                                        c.g[S2][0] =
                                                                            sx((c.g[S2][0] as u32)
                                                                                .wrapping_add(0x1));
                                                                        break 'b_20f3e4;
                                                                    }
                                                                    c.cycles += 3;
                                                                    c.g[V0][0] = ((c.g[S2][0]
                                                                        as i64)
                                                                        < (c.g[S3][0] as i64))
                                                                        as u64;
                                                                    let t = c.g[V0][0] != 0u64;
                                                                    c.g[V0][0] = 0x18;
                                                                    if t {
                                                                        if event_due(c) {
                                                                            deliver_event_here(
                                                                                c, 0x0020f288,
                                                                            );
                                                                        }
                                                                        continue 'l_20f288;
                                                                    } else {
                                                                        break 'b_20f3f0;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        'b_20f420: {
                                                            c.cycles += 8;
                                                            c.lwc1(
                                                                0,
                                                                (c.g[S7][0] as u32)
                                                                    .wrapping_add(0x10),
                                                            );
                                                            c.g[A2][0] = 0x4d0000;
                                                            c.g[V0][0] = sx((c.g[A2][0] as u32)
                                                                .wrapping_sub(0x73a8));
                                                            c.lwc1(
                                                                21,
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x18),
                                                            );
                                                            c.c_lt_s(0, 24);
                                                            c.g[T2][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0xc));
                                                            let t = !c.fcc();
                                                            c.lwc1(
                                                                22,
                                                                (c.g[S1][0] as u32)
                                                                    .wrapping_add(0x14),
                                                            );
                                                            if t {
                                                                break 'b_20f420;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.sub_s(0, 24, 0);
                                                                c.lwc1(
                                                                    1,
                                                                    (c.g[S1][0] as u32)
                                                                        .wrapping_add(0x1c),
                                                                );
                                                                c.mul_s(0, 0, 1);
                                                                c.add_s(22, 22, 0);
                                                                break 'b_20f420;
                                                            }
                                                        }
                                                        'b_20f444: {
                                                            c.cycles += 5;
                                                            c.mtc1(0, 23);
                                                            c.c_lt_s(25, 23);
                                                            if !c.fcc() {
                                                                c.lwc1(
                                                                    0,
                                                                    (c.g[T2][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                );
                                                                break 'b_20f444;
                                                            } else {
                                                                c.cycles += 4;
                                                                c.lwc1(
                                                                    0,
                                                                    (c.g[S1][0] as u32)
                                                                        .wrapping_add(0x1c),
                                                                );
                                                                c.mul_s(0, 25, 0);
                                                                c.add_s(21, 21, 0);
                                                                c.lwc1(
                                                                    0,
                                                                    (c.g[T2][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                );
                                                                break 'b_20f444;
                                                            }
                                                        }
                                                        c.cycles += 83;
                                                        c.g[T1][0] = 0xfffffffffffffc1f;
                                                        c.lwc1(
                                                            2,
                                                            (c.g[S1][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.g[A1][0] = 0x4;
                                                        c.swc1(0, (c.g[SP][0] as u32));
                                                        c.g[A2][0] = c.g[SP][0];
                                                        c.lwc1(
                                                            1,
                                                            (c.g[GP][0] as u32)
                                                                .wrapping_sub(0x54d4),
                                                        );
                                                        c.g[A3][0] = 0u64;
                                                        c.lwc1(
                                                            0,
                                                            (c.g[T2][0] as u32).wrapping_add(0xc),
                                                        );
                                                        c.sub_s(3, 2, 1);
                                                        c.g[AT][0] = 0x3f800000;
                                                        c.mtc1(1, 20);
                                                        c.swc1(
                                                            0,
                                                            (c.g[SP][0] as u32).wrapping_add(0x4),
                                                        );
                                                        c.add_s(2, 2, 1);
                                                        c.lwc1(
                                                            0,
                                                            (c.g[T2][0] as u32).wrapping_add(0x14),
                                                        );
                                                        c.swc1(
                                                            0,
                                                            (c.g[SP][0] as u32).wrapping_add(0x30),
                                                        );
                                                        c.lwc1(
                                                            1,
                                                            (c.g[T2][0] as u32).wrapping_add(0xc),
                                                        );
                                                        c.swc1(
                                                            1,
                                                            (c.g[SP][0] as u32).wrapping_add(0x34),
                                                        );
                                                        c.lwc1(
                                                            0,
                                                            (c.g[T2][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.swc1(
                                                            0,
                                                            (c.g[SP][0] as u32).wrapping_add(0x60),
                                                        );
                                                        c.lwc1(
                                                            1,
                                                            (c.g[T2][0] as u32).wrapping_add(0x18),
                                                        );
                                                        c.swc1(
                                                            1,
                                                            (c.g[SP][0] as u32).wrapping_add(0x64),
                                                        );
                                                        c.lwc1(
                                                            0,
                                                            (c.g[T2][0] as u32).wrapping_add(0x14),
                                                        );
                                                        c.swc1(
                                                            0,
                                                            (c.g[SP][0] as u32).wrapping_add(0x90),
                                                        );
                                                        c.lwc1(
                                                            1,
                                                            (c.g[T2][0] as u32).wrapping_add(0x18),
                                                        );
                                                        c.swc1(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.swc1(
                                                            21,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd4),
                                                        );
                                                        c.swc1(
                                                            23,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd8),
                                                        );
                                                        c.swc1(
                                                            20,
                                                            (c.g[SP][0] as u32).wrapping_add(0xdc),
                                                        );
                                                        c.swc1(
                                                            1,
                                                            (c.g[SP][0] as u32).wrapping_add(0x94),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.swc1(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.lq(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.swc1(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.swc1(
                                                            22,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd4),
                                                        );
                                                        c.sq(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x50),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x80),
                                                        );
                                                        c.swc1(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.g[T0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xb0),
                                                        );
                                                        c.g[A0][0] = c
                                                            .lw((c.g[T0][0] as u32)
                                                                .wrapping_add(0xe84));
                                                        c.g[V0][0] = c
                                                            .lw((c.g[A0][0] as u32)
                                                                .wrapping_add(0x8));
                                                        c.g[V0][0] = c.g[V0][0] & c.g[T1][0];
                                                        c.g[V0][0] = c.g[V0][0] | 0x120;
                                                        c.sw(
                                                            (c.g[A0][0] as u32).wrapping_add(0x8),
                                                            c.g[V0][0],
                                                        );
                                                        c.g[V0][0] = c
                                                            .lw((c.g[T0][0] as u32)
                                                                .wrapping_add(0xe84));
                                                        c.g[V1][0] = c.lw((c.g[T2][0] as u32));
                                                        c.sh(
                                                            (c.g[V0][0] as u32).wrapping_add(0x10),
                                                            c.g[V1][0],
                                                        );
                                                        c.g[T0][0] = c
                                                            .lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x854));
                                                        c.g[V0][0] = c.lw((c.g[T0][0] as u32)
                                                            .wrapping_add(0x10d8));
                                                        c.g[A0][0] = c
                                                            .lh((c.g[V0][0] as u32)
                                                                .wrapping_add(0x238));
                                                        c.g[V1][0] = c
                                                            .lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0x23c));
                                                        let t = c.g[V1][0] as u32;
                                                        c.g[A0][0] = sx((c.g[T0][0] as u32)
                                                            .wrapping_add((c.g[A0][0] as u32)));
                                                        call_ptr(c, t, 0x0020f524)?;
                                                        c.lwc1(
                                                            2,
                                                            (c.g[S1][0] as u32).wrapping_add(0x10),
                                                        );
                                                        c.sub_s(1, 22, 21);
                                                        c.lwc1(
                                                            0,
                                                            (c.g[GP][0] as u32)
                                                                .wrapping_sub(0x54d0),
                                                        );
                                                        c.g[A2][0] = 0x80;
                                                        c.swc1(
                                                            21,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd4),
                                                        );
                                                        c.g[A1][0] =
                                                            sx((c.g[SP][0] as u32)
                                                                .wrapping_add(0xc0));
                                                        c.sub_s(3, 2, 0);
                                                        c.swc1(
                                                            23,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd8),
                                                        );
                                                        c.swc1(
                                                            20,
                                                            (c.g[SP][0] as u32).wrapping_add(0xdc),
                                                        );
                                                        c.add_s(2, 2, 0);
                                                        c.mul_s(1, 1, 26);
                                                        c.g[A0][0] = c.g[SP][0];
                                                        c.swc1(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.lq(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.add_s(1, 21, 1);
                                                        c.swc1(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.sq(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x20),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.swc1(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.swc1(
                                                            1,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd4),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0x50),
                                                        );
                                                        c.lq(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.swc1(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.sq(
                                                            3,
                                                            (c.g[SP][0] as u32).wrapping_add(0x80),
                                                        );
                                                        c.lq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xd0),
                                                        );
                                                        c.sq(
                                                            2,
                                                            (c.g[SP][0] as u32).wrapping_add(0xb0),
                                                        );
                                                        c.sw(
                                                            (c.g[A0][0] as u32).wrapping_add(0x10),
                                                            c.g[A2][0],
                                                        );
                                                        'l_20f590: loop {
                                                            c.cycles += 7;
                                                            c.sw(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x14),
                                                                0u64,
                                                            );
                                                            c.sw(
                                                                (c.g[A0][0] as u32)
                                                                    .wrapping_add(0x18),
                                                                0u64,
                                                            );
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x30));
                                                            c.g[V0][0] = ((c.g[A0][0] as i64)
                                                                < (c.g[A1][0] as i64))
                                                                as u64;
                                                            if c.g[V0][0] != 0u64 {
                                                                c.sw(
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                    c.g[A2][0],
                                                                );
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x0020f590,
                                                                    );
                                                                }
                                                                continue 'l_20f590;
                                                            } else {
                                                                c.cycles += 23;
                                                                c.g[T0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x854));
                                                                c.g[V1][0] = 0xfffffffffffffc1f;
                                                                c.g[T1][0] = 0xffffffffffffffff;
                                                                c.g[A1][0] = 0x4;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[T0][0] as u32)
                                                                        .wrapping_add(0xe84));
                                                                c.g[A2][0] = c.g[SP][0];
                                                                c.g[A3][0] = 0u64;
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[A0][0] as u32)
                                                                        .wrapping_add(0x8));
                                                                c.g[V0][0] =
                                                                    c.g[V0][0] & c.g[V1][0];
                                                                c.g[V0][0] = c.g[V0][0] | 0x140;
                                                                c.sw(
                                                                    (c.g[A0][0] as u32)
                                                                        .wrapping_add(0x8),
                                                                    c.g[V0][0],
                                                                );
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[T0][0] as u32)
                                                                        .wrapping_add(0xe84));
                                                                c.sh(
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                    c.g[T1][0],
                                                                );
                                                                c.g[T0][0] = c
                                                                    .lw((c.g[GP][0] as u32)
                                                                        .wrapping_sub(0x854));
                                                                c.g[V0][0] = c
                                                                    .lw((c.g[T0][0] as u32)
                                                                        .wrapping_add(0x10d8));
                                                                c.g[A0][0] = c
                                                                    .lh((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x238));
                                                                c.g[V1][0] = c
                                                                    .lw((c.g[V0][0] as u32)
                                                                        .wrapping_add(0x23c));
                                                                let t = c.g[V1][0] as u32;
                                                                c.g[A0][0] = sx((c.g[T0][0]
                                                                    as u32)
                                                                    .wrapping_add(
                                                                        (c.g[A0][0] as u32),
                                                                    ));
                                                                call_ptr(c, t, 0x0020f5f8)?;
                                                                c.g[A0][0] = 0x80;
                                                                c.g[A1][0] = sx((c.g[SP][0]
                                                                    as u32)
                                                                    .wrapping_add(0xc0));
                                                                c.g[V1][0] = c.g[SP][0];
                                                                c.sw(
                                                                    (c.g[V1][0] as u32)
                                                                        .wrapping_add(0x10),
                                                                    c.g[A0][0],
                                                                );
                                                                'l_20f608: loop {
                                                                    c.cycles += 7;
                                                                    c.sw(
                                                                        (c.g[V1][0] as u32)
                                                                            .wrapping_add(0x14),
                                                                        c.g[A0][0],
                                                                    );
                                                                    c.sw(
                                                                        (c.g[V1][0] as u32)
                                                                            .wrapping_add(0x18),
                                                                        c.g[A0][0],
                                                                    );
                                                                    c.g[V1][0] = sx((c.g[V1][0]
                                                                        as u32)
                                                                        .wrapping_add(0x30));
                                                                    c.g[V0][0] = ((c.g[V1][0]
                                                                        as i64)
                                                                        < (c.g[A1][0] as i64))
                                                                        as u64;
                                                                    if c.g[V0][0] != 0u64 {
                                                                        c.sw(
                                                                            (c.g[V1][0] as u32)
                                                                                .wrapping_add(0x10),
                                                                            c.g[A0][0],
                                                                        );
                                                                        if event_due(c) {
                                                                            deliver_event_here(
                                                                                c, 0x0020f608,
                                                                            );
                                                                        }
                                                                        continue 'l_20f608;
                                                                    } else {
                                                                        'b_20f8a4: {
                                                                            c.cycles += 8;
                                                                            c.lwc1(
                                                                                1,
                                                                                (c.g[S1][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x1c,
                                                                                    ),
                                                                            );
                                                                            c.g[S0][0] = 0u64;
                                                                            c.g[AT][0] = 0x41a00000;
                                                                            c.mtc1(1, 0);
                                                                            c.div_s(0, 0, 1);
                                                                            c.add_s(24, 24, 0);
                                                                            let t = (c.g[S3][0]
                                                                                as i64)
                                                                                <= 0;
                                                                            c.sub_s(23, 25, 0);
                                                                            if t {
                                                                                break 'b_20f8a4;
                                                                            } else {
                                                                                c.cycles += 9;
                                                                                c.g[V0][0] =
                                                                                    0x4d0000;
                                                                                c.g[AT][0] =
                                                                                    0x3f000000;
                                                                                c.mtc1(1, 26);
                                                                                c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7438));
                                                                                c.g[AT][0] =
                                                                                    0x3f800000;
                                                                                c.mtc1(1, 22);
                                                                                c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x10));
                                                                                c.g[V0][0] = 0x18;
                                                                                'l_20f668: loop {
                                                                                    'b_20f898: {
                                                                                        c.cycles +=
                                                                                            7;
                                                                                        c.mult(
                                                                                            0, 4,
                                                                                            16, 2,
                                                                                        );
                                                                                        c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                                                                        c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                                                                                        c.c_lt_s(
                                                                                            0, 23,
                                                                                        );
                                                                                        if c.fcc() {
                                                                                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                                                                            break 'b_20f898;
                                                                                        } else {
                                                                                            c.cycles += 4;
                                                                                            c.c_lt_s(24, 0);
                                                                                            if c.fcc() {
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
break 'b_20f898;
} else {
'b_20f718: {
c.cycles += 10;
c.sub_s(0, 0, 25);
c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x1c));
c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x18));
c.g[T1][0] = 0xc;
c.lwc1(20, (c.g[S1][0] as u32).wrapping_add(0x10));
c.g[V0][0] = 0x1;
c.mul_s(0, 0, 2);
c.mov_s(4, 1);
let t = c.g[S6][0] == c.g[S0][0];
c.sub_s(21, 1, 0);
if t {
break 'b_20f718;
} else {
'b_20f70c: {
c.cycles += 17;
c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe4));
c.g[V0][0] = sx((c.g[S0][0] as u32) << 2);
c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[S4][0] as u32)));
c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x54cc));
c.g[V0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[V0][0] as u32)));
c.lwc1(0, (c.g[V1][0] as u32));
c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
c.mul_s(0, 0, 1);
c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x6c0));
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
c.g[V1][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x40));
c.sub_s(20, 20, 0);
c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x44));
let t = c.g[V0][0] as u32;
c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
call_ptr(c, t, 0x0020f6f8)?;
let t = c.g[V0][0] == 0u64;
c.g[T1][0] = 0xd;
if t {
break 'b_20f70c;
} else {
c.cycles += 3;
c.g[V0][0] = 0x3;
let t = true;
c.lwc1(4, (c.g[S1][0] as u32).wrapping_add(0x18));
if t {
break 'b_20f718;
} else {
break 'b_20f70c;
}
}
}
c.cycles += 3;
c.lwc1(4, (c.g[S1][0] as u32).wrapping_add(0x18));
c.g[T1][0] = 0xe;
c.g[V0][0] = 0x2;
break 'b_20f718;
}
}
'b_20f894: {
c.cycles += 12;
c.g[T0][0] = 0x4d0000;
c.g[V0][0] = sx((c.g[V0][0] as u32) << 4);
c.g[V1][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x73a8));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
c.g[T3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
c.mul_s(2, 0, 26);
c.sub_s(6, 21, 2);
c.c_le_s(4, 6);
let t = c.fcc();
c.add_s(7, 21, 2);
if t {
break 'b_20f894;
} else {
c.cycles += 5;
c.lwc1(1, (c.g[S1][0] as u32).wrapping_add(0x14));
c.c_le_s(7, 1);
if c.fcc() {
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
break 'b_20f898;
} else {
'b_20f7bc: {
'b_20f7b8: {
'b_20f78c: {
c.cycles += 4;
c.c_lt_s(6, 1);
c.lwc1(3, (c.g[T3][0] as u32).wrapping_add(0xc));
let t = !c.fcc();
c.lwc1(5, (c.g[T3][0] as u32).wrapping_add(0x18));
if t {
break 'b_20f78c;
} else {
c.cycles += 8;
c.mov_s(6, 1);
c.sub_s(0, 3, 5);
c.sub_s(1, 7, 1);
c.add_s(2, 2, 2);
c.mul_s(0, 0, 1);
c.div_s(0, 0, 2);
let t = true;
c.add_s(3, 5, 0);
if t {
break 'b_20f7b8;
} else {
break 'b_20f78c;
}
}
}
c.cycles += 4;
c.c_lt_s(4, 7);
if !c.fcc() {
c.lwc1(0, (c.g[T3][0] as u32).wrapping_add(0x10));
break 'b_20f7bc;
} else {
c.cycles += 7;
c.sub_s(0, 5, 3);
c.mov_s(7, 4);
c.sub_s(1, 4, 6);
c.add_s(2, 2, 2);
c.mul_s(0, 0, 1);
c.div_s(0, 0, 2);
c.add_s(5, 3, 0);
break 'b_20f7b8;
}
}
c.cycles += 1;
c.lwc1(0, (c.g[T3][0] as u32).wrapping_add(0x10));
break 'b_20f7bc;
}
c.cycles += 54;
c.g[T2][0] = 0xfffffffffffffc1f;
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x4));
c.g[T1][0] = sx((c.g[T1][0] as u32) << 5);
c.swc1(0, (c.g[SP][0] as u32));
c.g[A1][0] = 0x4;
c.g[A2][0] = c.g[SP][0];
c.g[A3][0] = 0u64;
c.lwc1(0, (c.g[T3][0] as u32).wrapping_add(0x14));
c.lwc1(2, (c.g[V0][0] as u32).wrapping_add(0x4));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x34));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
c.mul_s(2, 2, 26);
c.lwc1(1, (c.g[T3][0] as u32).wrapping_add(0x10));
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x64));
c.sub_s(3, 20, 2);
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x60));
c.add_s(2, 20, 2);
c.lwc1(0, (c.g[T3][0] as u32).wrapping_add(0x14));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.sw((c.g[SP][0] as u32).wrapping_add(0xd8), 0u64);
c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0xd4));
c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0xdc));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x90));
c.lq(4, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.sw((c.g[SP][0] as u32).wrapping_add(0xd8), 0u64);
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x94));
c.lq(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.swc1(7, (c.g[SP][0] as u32).wrapping_add(0xd4));
c.sw((c.g[SP][0] as u32).wrapping_add(0xd8), 0u64);
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(4, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sw((c.g[SP][0] as u32).wrapping_add(0xd8), 0u64);
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.g[V0][0] & c.g[T2][0];
c.g[V0][0] = c.g[V0][0] | c.g[T1][0];
c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
c.g[V1][0] = c.lw((c.g[T3][0] as u32));
c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x238));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x23c));
let t = c.g[V1][0] as u32;
c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0020f894)?;
break 'b_20f894;
}
}
}
c.cycles += 1;
c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
break 'b_20f898;
}
                                                                                        }
                                                                                    }
                                                                                    c.cycles += 3;
                                                                                    c.g[V0][0] = ((c
                                                                                        .g[S0][0]
                                                                                        as i64)
                                                                                        < (c.g[S3]
                                                                                            [0]
                                                                                            as i64))
                                                                                        as u64;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        != 0u64;
                                                                                    c.g[V0][0] =
                                                                                        0x18;
                                                                                    if t {
                                                                                        if event_due(
                                                                                            c,
                                                                                        ) {
                                                                                            deliver_event_here(c, 0x0020f668);
                                                                                        }
                                                                                        continue 'l_20f668;
                                                                                    } else {
                                                                                        break 'b_20f8a4;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        'b_20facc: {
                                                                            c.cycles += 3;
                                                                            c.g[A1][0] = c.lw((c.g
                                                                                [S7][0]
                                                                                as u32)
                                                                                .wrapping_add(0x8));
                                                                            let t =
                                                                                c.g[A1][0] == 0u64;
                                                                            c.g[V1][0] = 0x530000;
                                                                            if t {
                                                                                break 'b_20facc;
                                                                            } else {
                                                                                'b_20f8d4: {
                                                                                    c.cycles += 5;
                                                                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8d0));
                                                                                    c.g[V0][0] = sx(
                                                                                        ((c.g[V0][0]
                                                                                            as u32)
                                                                                            as i32
                                                                                            >> 9)
                                                                                            as u32,
                                                                                    );
                                                                                    c.g[V0][0] = c
                                                                                        .g[V0][0]
                                                                                        & 0x1;
                                                                                    let t = c.g[V0]
                                                                                        [0]
                                                                                        == 0u64;
                                                                                    c.g[S3][0] =
                                                                                        0x1;
                                                                                    if t {
                                                                                        break 'b_20f8d4;
                                                                                    } else {
                                                                                        c.cycles +=
                                                                                            4;
                                                                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                                                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
                                                                                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x88));
                                                                                        if c.g[A0]
                                                                                            [0]
                                                                                            != 0
                                                                                        {
                                                                                            c.g[S3][0] = 0u64;
                                                                                        }
                                                                                        break 'b_20f8d4;
                                                                                    }
                                                                                }
                                                                                c.cycles += 2;
                                                                                let t = (c.g[A1][0]
                                                                                    as i64)
                                                                                    <= 0;
                                                                                c.g[S2][0] = 0u64;
                                                                                if t {
                                                                                    break 'b_20facc;
                                                                                } else {
                                                                                    c.cycles += 9;
                                                                                    c.g[V0][0] =
                                                                                        0x4d0000;
                                                                                    c.g[AT][0] =
                                                                                        0x3f000000;
                                                                                    c.mtc1(1, 21);
                                                                                    c.g[AT][0] =
                                                                                        0x3f800000;
                                                                                    c.mtc1(1, 20);
                                                                                    c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x7358));
                                                                                    c.g[S0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0xe0));
                                                                                    c.g[S4][0] = 0xfffffffffffffc1f;
                                                                                    'l_20f900: loop {
                                                                                        'b_20fabc: {
                                                                                            c.cycles += 5;
                                                                                            c.lwc1(1, (c.g[S0][0] as u32).wrapping_add(0x4));
                                                                                            c.c_lt_s(1, 23);
                                                                                            if c.fcc() {
c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x8));
break 'b_20fabc;
} else {
c.cycles += 4;
c.c_lt_s(24, 1);
if c.fcc() {
c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x8));
break 'b_20fabc;
} else {
'b_20f940: {
c.cycles += 4;
c.g[V1][0] = c.lw((c.g[S0][0] as u32));
c.g[V0][0] = 0x1;
if c.g[V1][0] != c.g[V0][0] {
c.sub_s(1, 1, 25);
break 'b_20f940;
} else {
c.cycles += 2;
if c.g[S3][0] == 0u64 {
c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x8));
break 'b_20fabc;
} else {
c.cycles += 1;
c.sub_s(1, 1, 25);
break 'b_20f940;
}
}
}
'b_20fab8: {
c.cycles += 16;
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x1c));
c.g[V0][0] = sx((c.g[V1][0] as u32) << 4);
c.lwc1(4, (c.g[S1][0] as u32).wrapping_add(0x18));
c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[S5][0] as u32)));
c.mul_s(1, 1, 0);
c.lwc1(7, (c.g[V0][0] as u32).wrapping_add(0x8));
c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
c.mul_s(2, 7, 21);
c.g[T1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
c.sub_s(1, 4, 1);
c.mul_s(9, 0, 21);
c.sub_s(6, 1, 2);
c.c_le_s(4, 6);
let t = c.fcc();
c.add_s(8, 1, 2);
if t {
break 'b_20fab8;
} else {
c.cycles += 5;
c.lwc1(0, (c.g[S1][0] as u32).wrapping_add(0x14));
c.c_le_s(8, 0);
if c.fcc() {
c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x8));
break 'b_20fabc;
} else {
'b_20f9ec: {
'b_20f9e8: {
'b_20f9c0: {
c.cycles += 4;
c.c_lt_s(6, 0);
c.lwc1(3, (c.g[T1][0] as u32).wrapping_add(0xc));
let t = !c.fcc();
c.lwc1(5, (c.g[T1][0] as u32).wrapping_add(0x18));
if t {
break 'b_20f9c0;
} else {
c.cycles += 7;
c.mov_s(6, 0);
c.sub_s(1, 3, 5);
c.sub_s(0, 8, 0);
c.mul_s(1, 1, 0);
c.div_s(1, 1, 7);
let t = true;
c.add_s(3, 5, 1);
if t {
break 'b_20f9e8;
} else {
break 'b_20f9c0;
}
}
}
c.cycles += 4;
c.c_lt_s(4, 8);
if !c.fcc() {
c.lwc1(0, (c.g[T1][0] as u32).wrapping_add(0x10));
break 'b_20f9ec;
} else {
c.cycles += 6;
c.sub_s(1, 5, 3);
c.mov_s(8, 4);
c.sub_s(0, 4, 6);
c.mul_s(1, 1, 0);
c.div_s(1, 1, 7);
c.add_s(5, 3, 1);
break 'b_20f9e8;
}
}
c.cycles += 1;
c.lwc1(0, (c.g[T1][0] as u32).wrapping_add(0x10));
break 'b_20f9ec;
}
c.cycles += 51;
c.g[A1][0] = 0x4;
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x4));
c.g[A2][0] = c.g[SP][0];
c.swc1(0, (c.g[SP][0] as u32));
c.g[A3][0] = 0u64;
c.lwc1(0, (c.g[T1][0] as u32).wrapping_add(0x14));
c.lwc1(2, (c.g[S1][0] as u32).wrapping_add(0x10));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0x34));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x30));
c.sub_s(3, 2, 9);
c.add_s(2, 2, 9);
c.lwc1(0, (c.g[T1][0] as u32).wrapping_add(0x10));
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x64));
c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x60));
c.lwc1(1, (c.g[T1][0] as u32).wrapping_add(0x14));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.sw((c.g[SP][0] as u32).wrapping_add(0xd8), 0u64);
c.swc1(6, (c.g[SP][0] as u32).wrapping_add(0xd4));
c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xdc));
c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x90));
c.lq(4, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.sw((c.g[SP][0] as u32).wrapping_add(0xd8), 0u64);
c.swc1(5, (c.g[SP][0] as u32).wrapping_add(0x94));
c.lq(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.swc1(8, (c.g[SP][0] as u32).wrapping_add(0xd4));
c.sw((c.g[SP][0] as u32).wrapping_add(0xd8), 0u64);
c.sq(3, (c.g[SP][0] as u32).wrapping_add(0x50));
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0x80));
c.sq(4, (c.g[SP][0] as u32).wrapping_add(0x20));
c.sw((c.g[SP][0] as u32).wrapping_add(0xd8), 0u64);
c.lq(2, (c.g[SP][0] as u32).wrapping_add(0xd0));
c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.sq(2, (c.g[SP][0] as u32).wrapping_add(0xb0));
c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
c.g[V0][0] = c.g[V0][0] & c.g[S4][0];
c.g[V0][0] = c.g[V0][0] | 0x160;
c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xe84));
c.g[V1][0] = c.lw((c.g[T1][0] as u32));
c.sh((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
c.g[T0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x854));
c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10d8));
c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x238));
c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x23c));
let t = c.g[V1][0] as u32;
c.g[A0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
call_ptr(c, t, 0x0020fab8)?;
break 'b_20fab8;
}
}
}
c.cycles += 1;
c.g[V0][0] = c.lw((c.g[S7][0] as u32).wrapping_add(0x8));
break 'b_20fabc;
}
}
                                                                                        }
                                                                                        c.cycles +=
                                                                                            4;
                                                                                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                                                                        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                                                                                        let t = c.g
                                                                                            [V0][0]
                                                                                            != 0u64;
                                                                                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                                                                                        if t {
                                                                                            if event_due(c) { deliver_event_here(c, 0x0020f900); }
                                                                                            continue 'l_20f900;
                                                                                        } else {
                                                                                            break 'b_20facc;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        c.cycles += 10;
                                                                        c.g[A0][0] = c.lw((c.g[GP]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_sub(0x854));
                                                                        c.g[V0][0] = c.lw((c.g[A0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0xe84));
                                                                        c.g[A1][0] = c.lw((c.g[A0]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x10d8));
                                                                        c.g[V0][0] =
                                                                            sx((c.g[V0][0] as u32)
                                                                                .wrapping_sub(
                                                                                    0x14,
                                                                                ));
                                                                        c.sw(
                                                                            (c.g[A0][0] as u32)
                                                                                .wrapping_add(
                                                                                    0xe84,
                                                                                ),
                                                                            c.g[V0][0],
                                                                        );
                                                                        c.g[V1][0] = c.lh((c.g[A1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x100));
                                                                        c.g[V0][0] = c.lw((c.g[A1]
                                                                            [0]
                                                                            as u32)
                                                                            .wrapping_add(0x104));
                                                                        let t = c.g[V0][0] as u32;
                                                                        c.g[A0][0] =
                                                                            sx((c.g[A0][0] as u32)
                                                                                .wrapping_add(
                                                                                    (c.g[V1][0]
                                                                                        as u32),
                                                                                ));
                                                                        call_ptr(c, t, 0x0020faf0)?;
                                                                        c.sw(
                                                                            (c.g[S1][0] as u32)
                                                                                .wrapping_add(0x34),
                                                                            c.g[S6][0],
                                                                        );
                                                                        break 'b_20faf4;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x180));
            break 'b_20faf8;
        }
        c.cycles += 18;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x170));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x160));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x150));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x140));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x130));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xf0));
        c.lwc1(26, (c.g[SP][0] as u32).wrapping_add(0x1c0));
        c.lwc1(25, (c.g[SP][0] as u32).wrapping_add(0x1b8));
        c.lwc1(24, (c.g[SP][0] as u32).wrapping_add(0x1b0));
        c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x1a8));
        c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x1a0));
        c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x198));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x190));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1d0));
        c.pc = t;
        return Ok(());
    })
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
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::seed_count;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as RET;
    use crate::difftest::RA as SENTINEL;
    pub(crate) use crate::game::bx::bxstring2::tests::base_3 as base_4;
    pub(crate) use crate::game::bx::bxstring2::tests::BE_2;
    pub(crate) use crate::game::bx::bxstring2::tests::DB;
    pub(crate) use crate::game::bx::bxstring2::tests::RIDERS;
    pub(crate) use crate::game::bx::bxstring2::tests::RM;
    pub(crate) use crate::game::bx::bxstring2::tests::VTS;
    pub(crate) use crate::game::bx::bxstring2::tests::W_2 as W;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::clear_4 as clear;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_5 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::gp_at;
    pub(crate) use crate::game::fe::feasyncfile::tests::hash_object;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_12 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_10 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_2 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_5 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::ui;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2 as w;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ as OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_2 as OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_2 as WORLD;
    pub(crate) use crate::game::fe::fereal::tests::base as base_2;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY_PTR;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS;
    pub(crate) use crate::game::world::worldview::tests::word;
    pub(crate) use crate::game::world::worldview::tests::SEEDS_5 as SEEDS_2;
    use ssx3_ee as ee;

    #[test]
    fn c_keyboard_popup_set_max_length_matches() {
        check(
            ssx3_ee::s04::f_001cd088,
            c_keyboard_popup_set_max_length,
            300,
            &|c, seed| {
                let mut r = Rng(0x0c10_0005 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = match seed % 4 {
                    0 => (r.next_u32() % 64) as u64,
                    1 => 64 + (r.next_u32() % 64) as u64,
                    2 => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
                    _ => 0xffff_ffff_ffff_ffff,
                };
                c.bus.write32(DATA + 0x60, r.next_u32());
            },
        );
    }

    #[test]
    fn c_keyboard_popup_set_key_enabled_matches() {
        check(
            ssx3_ee::s04::f_001ce3c8,
            c_keyboard_popup_set_key_enabled,
            400,
            &|c, seed| {
                let mut r = Rng(0x0c10_0007 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (r.next_u32() % 4) as u64;
                c.g[A2][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
                c.g[A3][0] = match seed % 6 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => 0x1_0000_0000,
                    _ => 0xffff_ffff_ffff_ffff,
                };
            },
        );
    }

    #[test]
    fn race_progress_reset_rider_matches() {
        check_lohi(
            ssx3_ee::s05::f_002105b0,
            race_progress_reset_rider,
            100,
            &|c, seed| {
                let mut r = Rng(0x0c10_0012 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = (seed % 8) as u64;
            },
        );
    }

    #[test]
    fn race_progress_clear_path_data_matches() {
        check(
            ssx3_ee::s05::f_00210608,
            race_progress_clear_path_data,
            50,
            &|c, seed| {
                let mut r = Rng(0x0c10_0013 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP - 0x9a0, r.next_u32());
                c.bus.write32(GAME_GP - 0x99c, r.next_u32());
                c.bus.write32(GAME_GP - 0x998, r.next_u32());
            },
        );
    }

    // ------------------------------------------------------------------
    // Race progress / game state.

    #[test]
    fn race_progress_set_path_data_matches() {
        check(
            ssx3_ee::s05::f_002105d0__generated,
            race_progress_set_path_data,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x105d0, 0);
                let p = DATA + 0x100;
                c.bus.write32(p + 4, r.next_u32() % 0x800);
                c.bus.write32(p + 0xc, r.next_u32() % 0x800);
                c.g[A0][0] = sx(p);
            },
        );
    }

    #[test]
    fn hud_meter_dtor_matches() {
        check(
            ee::s05::f_0020eb08__generated,
            hud_meter_dtor,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x20eb_08, 1);
                let flags = seed % 4;
                // The free branch runs on a null object: freeing test data would
                // hand a bogus pointer to the heap.
                let this = if flags & 1 != 0 { 0 } else { DATA + 0x100 };
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(flags);
                c.bus.write32(GAME_GP - 0x984, 1 + r.next_u32() % 2);
                c.bus.write32(GAME_GP - 0x988, 0x1234_5678);
            },
        );
    }

    #[test]
    fn ov_screen_read_private_message_on_child_event_matches() {
        check(
            ssx3_ee::s05::f_00210dd0__generated,
            ov_screen_read_private_message_on_child_event,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x210dd0, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 2 == 0 {
                    0x77
                } else {
                    r.next_u32() as u64
                };
                c.g[A2][0] = if seed % 3 == 0 {
                    0x16
                } else {
                    (seed % 0x20) as u64
                };
                word(
                    c,
                    DATA + 0xe0,
                    if seed % 2 == 0 { 0x77 } else { r.next_u32() },
                );
            },
        );
    }

    #[test]
    fn keyboard_popup_set_function_keys_enabled_matches() {
        check(
            ee::s04::f_001ce468__generated,
            c_keyboard_popup_set_function_keys_enabled,
            48,
            &|c, seed| {
                let _ = base(c, seed, 0x1ce4_68);
                let s = DATA + 0x100;
                clear(c, s, 0x400);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn ov_screen_trophy_init_matches() {
        check(
            ssx3_ee::s05::f_0020e938__generated,
            ov_screen_trophy_init,
            40,
            &|c, seed| {
                let _r = setup(c, seed, 0x20e938);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(DATA + 0x40, seed);
            },
        );
    }

    #[test]
    fn hud_meter_ctor_matches() {
        check(
            ee::s05::f_0020eaa0__generated,
            hud_meter_ctor,
            40,
            &|c, seed| {
                base_2(c, seed, 0x20_eaa0);
                c.g[A0][0] = sx(DATA);
                if seed % 2 == 0 {
                    c.bus.write32(GAME_GP.wrapping_sub(0x988), 0);
                } else {
                    c.bus.write32(GAME_GP.wrapping_sub(0x988), 1);
                    c.bus.write32(GAME_GP.wrapping_sub(0x98c), DATA + 0x200);
                    c.bus.write32(DATA + 0x200 + 0x14, 320.0f32.to_bits());
                    c.bus.write32(DATA + 0x200 + 0x34, 240.0f32.to_bits());
                }
                c.bus
                    .write32(GAME_GP.wrapping_sub(0x54f4), 1.0f32.to_bits());
                c.bus
                    .write32(GAME_GP.wrapping_sub(0x54f0), 1.0f32.to_bits());
            },
        );
    }

    #[test]
    fn c_keyboard_popup_set_static_text_matches() {
        check(
            ee::s04::f_001ccf00__generated,
            c_keyboard_popup_set_static_text,
            8,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x001c_cf00, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (DATA + 0x4000) as u64;
                c.bus.write8(DATA + 0x4000, b'a');
                c.bus.write8(DATA + 0x4001, 0);
                let screen = DATA + 0x3800;
                match seed % 3 {
                    0 => w_2(c, OBJ + 0x40, 0),
                    1 => {
                        w_2(c, OBJ + 0x40, screen);
                        w_2(c, screen + 0x3c, 0);
                    }
                    _ => {
                        w_2(c, OBJ + 0x40, screen);
                        let node = DATA + 0x3c00;
                        hash_object(c, screen, node);
                        w_2(c, node + 0xb0, 0);
                        let _ = r.next_u32();
                    }
                }
            },
        );
    }

    #[test]
    fn ov_screen_read_private_message_update_matches() {
        check(
            ee::s05::f_00210d20__generated,
            ov_screen_read_private_message_update,
            60,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x210d);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x40, (seed % 2) as u32);
                c.bus
                    .write32(gp_at(-0x204), if seed % 3 == 0 { 0 } else { DATA + 0x1000 });
                c.bus.write32(DATA + 0x1000 + 0x68, (seed / 2 % 2) as u32);
                c.bus.write32(DATA + 0x1000 + 0x64, (seed / 4 % 2) as u32);
                c.bus.write32(DATA + 0x10, DATA + 0x2000);
                fill(c, &mut r, DATA + 0x9c, 0x50);
                fill(c, &mut r, DATA + 0x2000, 0x40);
            },
        );
    }

    #[test]
    fn cov_state_hud1p_load_meter_assets_matches() {
        check(
            ee::s05::f_0020eb50__generated,
            cov_state_hud1p_load_meter_assets,
            20,
            &|c, seed| {
                let _r = base_5(c, seed, 0x20eb);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (DATA + 0x3000) as u64;
                c.bus.write32(gp_at(-0x988), 0);
                c.bus.write32(gp_at(-0x98c), 0);
                c.bus.write32(DATA + 0x3000 + 8, seed as u32);
            },
        );
    }

    #[test]
    fn c_keyboard_popup_set_text_matches() {
        diff(
            ee::s04::f_001cb418__generated,
            c_keyboard_popup_set_text,
            20,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x1c_b418);
                let hb = DATA + 0x1000;
                w_3(c, DATA + 0x74, hb + 0xc);
                w_3(c, hb, 1);
                w_3(c, hb + 4, 2);
                w_3(c, hb + 8, 0x40);
                c.bus.write8(hb + 0xc, b'h');
                c.bus.write8(hb + 0xd, b'i');
                c.bus.write8(hb + 0xe, 0);
                w_3(c, DATA + 0x58, if seed % 2 == 0 { 0 } else { r.next_u32() });
                w_3(c, DATA + 0x64, r.next_u32());
                c.g[A0][0] = sx(DATA);
            },
        );
    }
    use super::*;
    use crate::difftest::fakes::ptr;
    use crate::difftest::{fresh, put_vec};
    pub(crate) use crate::game::ai::rider::tests::meter;

    pub(crate) fn draw_progress_meter_setup(c: &mut Cpu, seed: u32) {
        use meter::*;
        crate::difftest::elf_image(c);
        let mut r = machine(c, seed, 0x0020_eda0, 24);
        // Early outs: HUD hidden, meter off, no player, no meter definition.
        let early = seed % 10;
        c.bus.write32(gp_at(-0x990), (early == 0) as u32);
        c.bus.write32(gp_at(-0x994), (early != 1) as u32);
        c.bus.write32(gp_at(-0x998), MARKERS);
        c.bus
            .write32(gp_at(-0x9a0), if early == 2 { 0 } else { METER });
        c.bus.write32(gp_at(-0x98c), FONT);
        c.bus.write32(gp_at(-0x848), WORLD);
        c.bus.write32(gp_at(-0x854), RENDERER);
        for off in [0x1dc4, 0x1dc8, 0x1dd8] {
            c.bus.write32(gp_at(off), r.next_u32() & 1);
        }
        c.g[A0][0] = HUD as u64;
        c.g[A1][0] = if early == 3 { 0 } else { PLAYER as u64 };
        fill(c, &mut r, HUD, 0x10);
        for i in 0..8 {
            c.bus.write32(HUD + 4 * i, r.f32(400.0));
        }
        c.bus.write32(HUD + 0x34, r.next_u32() % 4);
        // Rider vtable: slot 0x38 (the player's racer slot, a fake).
        let vt = vtable(c, &mut r, &[0x38, 0x40]);
        c.bus.write32(PLAYER + 0x6c0, vt);
        // Slot 0x40 (asked of every other racer) is a real 0/1 function.
        c.bus
            .write32(vt + 0x44, if seed % 3 == 0 { RET1 } else { RET0 });
        let slot = r.next_u32() % 6;
        c.bus.write32(LOG_RESULTS, slot);
        // World -> race manager -> race; mode flag (+0x88).
        c.bus.write32(WORLD + 0x84, RACE_MAN);
        c.bus.write32(WORLD + 0xc0, MODE);
        c.bus.write32(MODE + 0x88, r.next_u32() & 1);
        c.bus.write32(RACE_MAN + 0xc, RACE);
        c.bus.write32(RACE + 0x78, r.next_u32() % 8);
        for i in 0..8 {
            let racer = RACERS + 0x800 * i;
            c.bus.write32(RACE + 0x28 + 4 * i, racer);
            c.bus.write32(racer + 0x6c0, vt);
        }
        // Racer progress: distances around the player's so some fall
        // inside the +-5000 window and some outside.
        for i in 0..6 {
            let e = PROGRESS + 0x18 * i;
            for w in 0..5 {
                c.bus.write32(e + 4 * w, r.f32(9000.0));
            }
            c.bus.write32(e + 0x14, r.next_u32() & 1);
        }
        // Style table entries {f32, f32, f32, ptr}; the pointed-to records
        // hold a colour word and a few floats.
        for i in 0..10 {
            let e = STYLE_TABLE + 0x10 * i;
            for w in 0..3 {
                c.bus.write32(e + 4 * w, r.f32(300.0));
            }
            let rec = STYLES + 0x40 * i;
            c.bus.write32(e + 0xc, rec);
            c.bus.write32(rec, r.next_u32());
            for w in 1..8 {
                c.bus.write32(rec + 4 * w, r.f32(300.0));
            }
        }
        for w in 0..8 {
            c.bus.write32(VECS + 4 * w, r.f32(2.0));
        }
        c.bus.write32(FLAGS, r.next_u32());
        // Meter definition: +0x8 marker count, +0x10 course length.
        fill(c, &mut r, METER, 8);
        c.bus.write32(METER + 0x8, r.next_u32() % 4);
        c.bus.write32(
            METER + 0x10,
            (5000.0 + (r.next_u32() % 20000) as f32).to_bits(),
        );
        for i in 0..4 {
            c.bus.write32(MARKERS + 8 * i, r.next_u32() % 3);
            c.bus.write32(MARKERS + 8 * i + 4, r.f32(9000.0));
        }
        // Renderer: vtable slots begin/state/draw/end, GS packet buffer.
        let rvt = vtable(c, &mut r, &[0xf8, 0x100, 0x110, 0x238]);
        c.bus.write32(RENDERER + 0x10d8, rvt);
        c.bus.write32(RENDERER + 0xe84, PACKET);
        fill(c, &mut r, PACKET, 0x10);
        c.pc = 0x0020_eda0;
    }

    /// Every early out, and the full meter with 0..7 racers, markers in and
    /// out of the window, and unwinds at the renderer calls.
    #[test]
    fn c_ov_state_hud1p_draw_progress_meter_matches() {
        covered(
            ee::s05::f_0020eda0__generated,
            c_ov_state_hud1p_draw_progress_meter,
            200,
            &draw_progress_meter_setup,
        );
    }

    // ---- cAngerFX_Render (0x2d5048) ----

    /// Guest functions that just return 0 / 1 (`GameProcess_v09`,
    /// `WorldObj_v16`), for vtable slots whose result picks a branch.
    pub(crate) const RET0: u32 = 0x0036_0758;

    pub(crate) const RET1: u32 = 0x0036_0910;

    /// Write vtable slot `off` = `{0, f}`.
    pub(crate) fn slot(c: &mut Cpu, vt: u32, off: u32, f: u32) {
        c.bus.write16(vt + off, 0);
        c.bus.write32(vt + off + 4, f);
    }
}
