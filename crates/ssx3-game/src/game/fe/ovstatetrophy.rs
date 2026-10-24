//! `src/fe/ovstatetrophy.cpp`: hand-written replacements (moved from the
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
}
