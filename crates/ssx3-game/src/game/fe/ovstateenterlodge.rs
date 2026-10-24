//! `src/fe/ovstateenterlodge.cpp`: hand-written replacements (moved from the
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
pub(crate) use crate::game::dirtysock::tags2::gp_8 as gp_2;
pub(crate) use crate::game::dirtysock::tags2::p;
pub(crate) use crate::game::fe::feasyncfile::at as at_3;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_3;
pub(crate) use crate::game::fe::feasyncfile::gp_3 as gp;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::sw_4 as sw_2;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::fe::festateloadhint::accept_key_6;
pub(crate) use crate::game::fe::festaterewards2::handle_list_key;
pub(crate) use crate::game::fe::festaterewards2::r;
pub(crate) use crate::game::fe::festaterewards2::Ops;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at;
pub(crate) use crate::game::movie::movieplayer::at as at_2;
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
        0x001f_7198,
        ov_screen_enter_lodge_init,
        "OVScreenEnterLodge_Init",
    ),
    (
        0x001f_7210,
        ov_screen_enter_lodge_on_activate,
        "OVScreenEnterLodge_OnActivate",
    ),
    (
        0x001f_72b8,
        ov_screen_enter_lodge_on_widget_notify,
        "OVScreenEnterLodge_OnWidgetNotify",
    ),
    (0x001f_73a8, ov_screen_bc_start_init, "OVScreenBCStart_Init"),
    (
        0x001f_7418,
        ov_screen_bcstart_transition_in,
        "OVScreenBCStart_TransitionIn",
    ),
    (
        0x001f_74e0,
        ov_screen_bc_start_on_widget_created,
        "OVScreenBCStart_OnWidgetCreated",
    ),
    (
        0x001f_7548,
        ov_screen_bc_start_pop_event_arg,
        "OVScreenBCStart_PopEventArg",
    ),
    (
        0x001f_7590,
        ov_screen_bc_start_on_widget_notify,
        "OVScreenBCStart_OnWidgetNotify",
    ),
    (0x001f_76b8, ov_screen_bc_fail_init, "OVScreenBCFail_Init"),
    (
        0x001f_7738,
        ov_bcfail_pop_event_arg,
        "OVScreenBCFail_PopEventArg",
    ),
    (
        0x001f_7770,
        ovscreenbcfail_on_widget_created,
        "OVScreenBCFail_OnWidgetCreated",
    ),
    (
        0x001f_77f0,
        ov_screen_bc_fail_on_widget_notify,
        "OVScreenBCFail_OnWidgetNotify",
    ),
    (
        0x001f_7e68,
        ov_screen_top_times_on_widget_created,
        "OVScreenTopTimes_OnWidgetCreated",
    ),
    (
        0x001f_7f08,
        ov_screen_top_times_on_activate,
        "OVScreenTopTimes_OnActivate",
    ),
    (
        0x001f_80c8,
        ov_screen_top_times_on_widget_notify,
        "OVScreenTopTimes_OnWidgetNotify",
    ),
    (
        0x001f_80d8,
        ov_screen_top_times_on_widget_action,
        "OVScreenTopTimes_OnWidgetAction",
    ),
    (0x001f_8118, ov_screen_top_times_v26, "OVScreenTopTimes_v26"),
    (
        0x0020_a430,
        cov_state_overlay_shutdown,
        "cOVStateOverlay_Shutdown",
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
    pub(crate) use crate::game::bx::bxstring2::callee::EA_MEM_FREE as EA_MEM_FREE_3;
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
    pub(crate) use crate::game::dirtysock::tags2::callee::AUDIO_APPLY_OUTPUT_MODE;
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
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_IS_KEY_ENABLED;
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
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_MENU_SET_SELECTED_BY_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SET_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_WIDGET_SLOT_0;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_WIDGET_SLOT_1;
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
    pub(crate) use crate::game::main::gamestate::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_2;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_BRAGGING_RIGHTS_INIT_PLAYER_ROWS;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_CHAR_STATS;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::main::gamestate::callee::C_GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::main::gamestate::callee::C_KEYBOARD_POPUP_SET_KEY_ENABLED;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_FREE_4 as EA_MEM_FREE_2;
    pub(crate) use crate::game::main::gamestate::callee::GAME_ANIM_MAP_UNREGISTER_BANK;
    pub(crate) use crate::game::main::gamestate::callee::GAME_STATE_LOAD_STATE;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_CLEAR_SLOTS;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_INSTALL_VIF_VU_HANDLERS;
    pub(crate) use crate::game::main::gamestate::callee::MEMCARD_STATE_BASE_INSTALL_CALLBACKS;
    pub(crate) use crate::game::main::gamestate::callee::MEM_FREE as MEM_FREE_3;
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
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ as SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_2;
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
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_CTOR_3 as UI_STATE_CTOR_2;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_SHUTDOWN_2 as UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::main::gamestate::callee::VD_TEXTURE_SINK_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_1;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::BEREWARD_INT_IS_PEAK_COMPLETE;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_SCORE_INT_UPDATE_BEST_STATS;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_DISCONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_IS_CONNECTED;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_RECONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE;
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
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_END_OPERATION_4 as MC_MANAGER_END_OPERATION_2;
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
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod table {
    pub(crate) use crate::game::fe::feasyncfile::table::ASYNC_FILE_GET_RESULT;
    pub(crate) use crate::game::fe::feasyncfile::table::AUDIO_GET_RACE_MAN;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_NEW_RACE_SET_EVENT_TYPE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_NEW_RACE_SET_GAME_MODE;
    pub(crate) use crate::game::fe::feasyncfile::table::BUILTIN_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::table::BXSTRING_ASSIGN;
    pub(crate) use crate::game::fe::feasyncfile::table::BXSTRING_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_HULL_TRIANGLE;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_HULL_TRIANGLE2;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_SPHERE_CALC;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_SPHERE_TREE_TRIANGLE;
    pub(crate) use crate::game::fe::feasyncfile::table::EAC_GET_UNPACKED_SIZE;
    pub(crate) use crate::game::fe::feasyncfile::table::EAC_UNPACK;
    pub(crate) use crate::game::fe::feasyncfile::table::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::table::EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::table::OPERATOR_NEW_TAG;
    pub(crate) use crate::game::fe::feasyncfile::table::PATH_STOP_TRACKS;
    pub(crate) use crate::game::fe::feasyncfile::table::PS2_MEM_CARD_CREATE;
    pub(crate) use crate::game::fe::feasyncfile::table::REPLAY_BEGIN_PLAYBACK;
    pub(crate) use crate::game::fe::feasyncfile::table::REPLAY_END_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::table::RIDER_MODEL_FIND;
    pub(crate) use crate::game::fe::feasyncfile::table::SCRIPT_ENGINE_GET_RESULT_VTABLE;
    pub(crate) use crate::game::fe::feasyncfile::table::SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::table::STR_CASECMP;
    pub(crate) use crate::game::fe::feasyncfile::table::STR_HASH;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_MENU_RESET_SCROLL;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_TEXT_SET_WIDE_STRING;
    pub(crate) use crate::game::fe::feasyncfile::table::VOIP_RPC_SET_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::table::VOIP_RPC_START;
}

/// `void OVScreenBCFail_PopEventArg(OVScreenBCFail* self)` (0x1f7738):
/// pop one pending script event and stash its argument at +0x9c.
pub fn ov_bcfail_pop_event_arg(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0xce8)).lw(c);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1c4));
        call_ptr(c, callee::WSCRIPT_MAN_EVENT_QUEUE_POP, 0x001f_7754)?;
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        frame.restore_ra(c, 0);
        Addr((c.g[S0][0] as u32).wrapping_add(0x9c)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `void OVScreenTopTimes_OnWidgetAction(self, widget, action)` (0x1F80D8):
/// action 5 on a widget forwards to the screen's virtual handler at vtable
/// slot `+0xD0`, with `this` adjusted by the slot's short delta.
pub fn ov_screen_top_times_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let this = c.g[A0][0];
        c.g[A3][0] = this;
        let widget = c.g[A1][0];
        frame.save_ra(c, 0);
        if widget == 0 {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 5;
            let action = c.g[A2][0];
            frame.restore_ra(c, 0);
            if action == c.g[V0][0] {
                c.cycles += 5;
                let vt = c.lw_r_2(V0, r(c, A3).wrapping_add(8));
                let delta = c.lh_r(A0, (vt as u32).wrapping_add(0xd0));
                let target = c.lw_r_2(V1, (vt as u32).wrapping_add(0xd4));
                let t = target as u32;
                let a0 = sx(r(c, A3).wrapping_add(delta as u32));
                c.g[A0][0] = a0;
                call_ptr(c, t, 0x001f_8108)?;
                c.cycles += 1;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenBCStart_PopEventArg(self)` (0x1f7548): pop the pending
/// broadcast-start event argument (the global `gp+0xce8` holds it; `+0x1c4`
/// is the queue) into `+0x9c`, falling back to the global's `+0x2a0` when the
/// queue is empty.
pub fn ov_screen_bc_start_pop_event_arg(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = Addr(gp(c, 0xce8)).lw(c);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let queue = (c.g[V0][0] as u32).wrapping_add(0x1c4);
        c.g[A0][0] = sx(queue);
        call_ptr(c, 0x0030_c820, 0x001f_7564)?;
        let self_ = abi::r32(c, S0);
        if c.g[V0][0] == 0 {
            c.g[V0][0] = Addr(gp(c, 0xce8)).lw(c);
            c.cycles += 2;
            let fallback = c.g[V0][0] as u32;
            c.g[V1][0] = Addr(fallback.wrapping_add(0x2a0)).lw(c);
            Addr(self_.wrapping_add(0x9c)).sw(c, c.g[V1][0]);
        } else {
            c.cycles += 3;
            let value = c.g[V0][0] as u32;
            c.g[V0][0] = Addr(value).lw(c);
            Addr(self_.wrapping_add(0x9c)).sw(c, c.g[V0][0]);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `D_004A26FC`: the "top times visible" flag.
pub(crate) const TOP_TIMES_VISIBLE_GP: i32 = -0x9f4;

/// `void OVScreenTopTimes_v26(s, void* widget)` (0x1f8118): once the screen's
/// +0xa0 word is clear, interpret the widget's `+0x18` selection: 0 shows the
/// top times (setting the flag at +0xa0 and `D_004A26FC`), 1 plays the
/// confirmation sound. 5 (+3, +2, +3) + 2 (+2) + 1 + 2 cycles.
pub fn ov_screen_top_times_v26(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        let busy = c.lw(p(self_, 0xa0));
        c.g[V0][0] = busy;
        frame.restore_ra(c, 0);
        if busy == 0 {
            c.cycles += 3;
            let sel = c.lw(p(r32(c, A1), 0x18));
            c.g[A1][0] = sel;
            let none = sel == 0;
            c.g[V0][0] = 1;
            if none {
                c.cycles += 3;
                c.sw(p(self_, 0xa0), c.g[V0][0]);
                c.sw(gp_2(c, TOP_TIMES_VISIBLE_GP), c.g[V0][0]);
                c.cycles += 1;
                frame.restore_ra(c, 0);
            } else {
                c.cycles += 2;
                if sel == c.g[V0][0] {
                    c.cycles += 2;
                    c.sw(p(self_, 0xa0), c.g[A1][0]);
                    c.cycles += 2;
                    c.g[A0][0] = 0x13;
                    call_ptr(c, callee::KEYBOARD_POPUP_SHOW, 0x001f_815c)?;
                    c.cycles += 1;
                    frame.restore_ra(c, 0);
                } else {
                    c.cycles += 2;
                }
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `int OVScreenBCStart_OnWidgetNotify(OVScreen*, void*, int key)`
/// (0x001f7590).
pub fn ov_screen_bc_start_on_widget_notify(c: &mut Cpu) {
    accept_key_6(c);
}

/// `int OVScreenBCFail_OnWidgetNotify(OVScreen*, void*, int key)`
/// (0x001f77f0).
pub fn ov_screen_bc_fail_on_widget_notify(c: &mut Cpu) {
    accept_key_6(c);
}

/// `int OVScreenTopTimes_OnWidgetNotify(OVScreen*, void*, int key)`
/// (0x001f80c8).
pub fn ov_screen_top_times_on_widget_notify(c: &mut Cpu) {
    accept_key_6(c);
}

/// `int OVScreenEnterLodge_OnWidgetNotify(OVScreen*, void*, int key)`
/// (0x001f72b8).
pub fn ov_screen_enter_lodge_on_widget_notify(c: &mut Cpu) {
    handle_list_key(c);
}

/// `OVScreenBCStart_OnWidgetCreated(OVScreenBCStart* self, Widget* item)`
/// (0x1f74e0): compare the widget's id with the two screen-name hashes and set
/// the widget's `+0x18` to 0 or 1 when one matches.
pub fn ov_screen_bc_start_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0x004a_0000;
        frame.save(c, S1, 0x10);
        c.g[A0][0] = 0x004a_23e0;
        frame.save_ra(c, 0);
        let item = c.g[A1][0];
        c.g[S1][0] = item;
        frame.save(c, S0, 0x20);
        c.g[S0][0] = lw(c, p(item as u32, 0x38));
        call_ptr(c, 0x0031_7670, 0x001f_7504)?;
        let first = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = 0x004a_0000;
        if !first {
            c.cycles += 2;
            sw(c, p(item as u32, 0x18), 0);
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 5;
            c.g[S0][0] = lw(c, p(item as u32, 0x38));
            c.g[A0][0] = 0x004a_23e8;
            call_ptr(c, 0x0031_7670, 0x001f_7520)?;
            let second = c.g[S0][0] != c.g[V0][0];
            frame.restore(c, S0, 0x20);
            if !second {
                c.cycles += 2;
                c.g[V0][0] = 1;
                sw(c, p(item as u32, 0x18), c.g[V0][0]);
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenBCStart_Init(OVScreenBCStart* self)` (0x1f73a8): create the
/// screen by name, start its thread and initialise the overlay state.
pub fn ov_screen_bc_start_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x46f4_e0;
        c.g[S0][0] = Addr(at(c, S1, 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x001f_73cc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001f_73e0)?;
        let t = c.g[V0][0] == 0;
        Addr(at(c, S1, 0x40)).sw(c, c.g[V0][0]);
        if !t {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001f_73f8)?;
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0020_a380, 0x001f_7400)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end screens.

/// `func_001F7198` (0x1f7198): `OVScreenEnterLodge::Init`. Creates the screen
/// widget named `0x46f4d0` from `this->+0x10` (`0x397cc0`), stores it at
/// `+0x40`, and if it exists shows it (`0x39c870`); finally runs the two shared
/// finish calls (`0x20d110`, `0x20a380`). Signature: `void init(this)`.
pub fn ov_screen_enter_lodge_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x47_0000;
        c.g[A0][0] = sx(low(c, A0).wrapping_sub(0xb30));
        c.g[RA][0] = sx(0x001f_71bc);
        c.g[S0][0] = lw_r(c, S1, 0x10);
        call_ptr(c, 0x0031_7670, 0x001f_71bc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x001f_71d0);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001f_71d0)?;
        let missing = c.g[V0][0] == 0;
        sw_r(c, S1, 0x40, c.g[V0][0]);
        if !missing {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x001f_71e8);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001f_71e8)?;
        }
        c.cycles += 9;
        c.g[RA][0] = sx(0x001f_71f0);
        call_ptr(c, 0x0020_d110, 0x001f_71f0)?;
        c.g[RA][0] = sx(0x001f_71f8);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0020_a380, 0x001f_71f8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenBCFail_Init(self)` (0x001f76b8): create the BC-fail screen
/// and initialise the overlay.
pub fn ov_screen_bc_fail_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(0x470000);
        c.g[S1][0] = c.g[A0][0];
        c.g[S0][0] = sx(0x470000u32.wrapping_sub(0xb00));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        callee::GET_HASH_VALUE_32.call(c, 0x001f_76dc)?;
        c.g[A0][0] = c.g[S0][0];
        Addr(at_2(c, S1, 0xc)).sw(c, c.g[V0][0]);
        c.g[S0][0] = Addr(at_2(c, S1, 0x10)).lw(c);
        callee::GET_HASH_VALUE_32.call(c, 0x001f_76ec)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME.call(c, 0x001f_7700)?;
        let absent = c.g[V0][0] == 0;
        Addr(at_2(c, S1, 0x40)).sw(c, c.g[V0][0]);
        if !absent {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            callee::UI_SCREEN_PLAY_FRAME.call(c, 0x001f_7718)?;
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        callee::C_OV_STATE_OVERLAY_INIT.call(c, 0x001f_7720)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenBCFail_OnWidgetCreated(self, widget)` (0x1f7770): pick the
/// failure screen's mode (0..2) from the created widget's name hash.
pub fn ovscreenbcfail_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0x4a0000;
        frame.save(c, 17, 0x10);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x23e0));
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, 16, 0x20);
        c.g[S0][0] = lw_at(c, p(r32(c, S1), 0x38));
        call_ptr(c, callee::STR_HASH, 0x001f_7794)?;
        let differs = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = 0x4a0000;
        if !differs {
            c.cycles += 2;
            c.sw(p(r32(c, S1), 0x18), 0);
            c.cycles += 1;
            frame.restore(c, 16, 0x20);
        } else {
            c.cycles += 5;
            c.g[S0][0] = lw_at(c, p(r32(c, S1), 0x38));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x23e8));
            call_ptr(c, callee::STR_HASH, 0x001f_77b0)?;
            if c.g[S0][0] != c.g[V0][0] {
                c.g[A0][0] = 0x4a0000;
                c.cycles += 5;
                c.g[S0][0] = lw_at(c, p(r32(c, S1), 0x38));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x23f8));
                call_ptr(c, callee::STR_HASH, 0x001f_77cc)?;
                let differs = c.g[S0][0] != c.g[V0][0];
                frame.restore(c, 16, 0x20);
                if differs {
                    c.cycles += 4;
                    frame.restore(c, 17, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                c.cycles += 1;
                c.g[V0][0] = 2;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 1;
            }
            c.cycles += 1;
            c.sw(p(r32(c, S1), 0x18), c.g[V0][0]);
            c.cycles += 1;
            frame.restore(c, 16, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenEnterLodge_OnActivate(OVScreenEnterLodge* self, int)` (0x1f7210):
/// clear the "disabled" flag of the lodge button, set the "enabled" flag of
/// the leave button and activate the state.
pub fn ov_screen_enter_lodge_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        c.g[A0][0] = 0x4a_0000;
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x001f_7234);
        c.g[A0][0] = sx(0x004a_23e0);
        call_ptr(c, 0x0031_7670, 0x001f_7234)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x40)).lw(c);
        c.g[RA][0] = sx(0x001f_7240);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001f_7240)?;
        c.g[V1][0] = c.g[V0][0];
        if c.g[V1][0] != 0 {
            Addr(p(r32(c, V1), 0x18)).sw(c, 0);
        }
        c.cycles += 9;
        c.g[A0][0] = 0x4a_0000;
        c.g[RA][0] = sx(0x001f_7258);
        c.g[A0][0] = sx(0x004a_23e8);
        call_ptr(c, 0x0031_7670, 0x001f_7258)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x40)).lw(c);
        c.g[RA][0] = sx(0x001f_7264);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001f_7264)?;
        c.g[V1][0] = c.g[V0][0];
        let null = c.g[V1][0] == 0;
        c.g[V0][0] = 0x1;
        if !null {
            c.cycles += 1;
            Addr(p(r32(c, V1), 0x18)).sw(c, c.g[V0][0]);
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x001f_7280);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0039_e4c0, 0x001f_7280)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenTopTimes_OnWidgetCreated(self, widget)` (0x1f7e68): pick the
/// top-times title string and store the "online" flag.
pub fn ov_screen_top_times_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[A0][0] = 0x0046_f578;
        frame.save(c, S0, 0x20);
        let widget = abi::arg(c, 1);
        c.g[S0][0] = sx(widget);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = lw_2(c, S0, 0x38);
        call_ptr(c, table::STR_HASH, 0x001f_7e8c)?;
        c.g[A0][0] = 0x0047_0000;
        let other = c.g[S1][0] != c.g[V0][0];
        if other {
            c.cycles += 4;
            c.g[A0][0] = 0x0046_f5a8;
            call_ptr(c, table::STR_HASH, 0x001f_7ee4)?;
            if c.g[S1][0] != c.g[V0][0] {
                frame.restore(c, S0, 0x20);
            } else {
                c.cycles += 2;
                c.g[V0][0] = 1;
                sw_2(c, S0, 0x18, c.g[V0][0]);
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(gp_3(c, -0x9f0)).lw(c);
            if c.g[V0][0] != 0 {
                c.g[A0][0] = 0x0046_f598;
                c.cycles += 7;
                call_ptr(c, table::STR_HASH, 0x001f_7ec8)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, table::UI_TEXT_SET_TEXT_ID, 0x001f_7ed4)?;
                sw_2(c, S0, 0x18, 0);
            } else {
                c.cycles += 8;
                c.g[A0][0] = 0x0046_f588;
                call_ptr(c, table::STR_HASH, 0x001f_7eac)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, table::UI_TEXT_SET_TEXT_ID, 0x001f_7eb8)?;
                sw_2(c, S0, 0x18, 0);
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

// ---------------------------------------------------------------------------
// `cOVStateOverlay_Shutdown` (0x20a430): stop the overlay's cut scene, close
// its PDA template, find and shut down its UI state, then run the base
// shutdown.
pub fn cov_state_overlay_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x50));
        let no_cutscene = c.g[V0][0] == 0;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        if no_cutscene {
            c.cycles += 1;
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            if c.g[V0][0] == 0 {
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            } else {
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x64));
                call_ptr(c, 0x002ea860, 0x0020_a464)?;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x50), 0u64);
                c.cycles += 1;
                c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            }
        }
        c.cycles += 10;
        c.g[A0][0] = 0x4a0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20d0));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x00317670, 0x0020_a47c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039f9d8, 0x0020_a488)?;
        c.g[A3][0] = c.g[V0][0];
        let no_state = c.g[A3][0] == 0;
        c.g[A1][0] = 0x4;
        if !no_state {
            c.cycles += 6;
            c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
            c.g[A2][0] = 0u64;
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, target, 0x0020_a4ac)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x54));
        let no_pda = c.g[V0][0] == 0;
        if !no_pda {
            c.cycles += 2;
            callee::OV_SCREEN_PDA_TEMPLATE_CLOSE.call(c, 0x0020_a4c0)?;
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0039e4a0, 0x0020_a4c8)?;
        c.lq(16, frame.at(c, 0x20));
        c.lq(17, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenEnterLodge_OnWidgetAction(this, widget, msg)` (0x1f72e0): the
/// "enter lodge" prompt — quit to the load screen for widget 0, mark the
/// screen done and close it for widget 1.
pub fn ov_screen_enter_lodge_on_widget_action(c: &mut Cpu) {
    'b_1f739c: {
        'b_1f7398: {
            c.cycles += 4;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
            c.g[A3][0] = c.g[A0][0];
            let t = c.g[A1][0] == 0u64;
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            if t {
                break 'b_1f7398;
            } else {
                'b_1f7370: {
                    'b_1f730c: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x5;
                        let t = c.g[A2][0] == c.g[V0][0];
                        c.g[V0][0] = 0x6;
                        if t {
                            break 'b_1f730c;
                        } else {
                            c.cycles += 2;
                            let t = c.g[A2][0] == c.g[V0][0];
                            c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                            if t {
                                break 'b_1f7370;
                            } else {
                                c.cycles += 2;
                                let t = true;

                                if t {
                                    break 'b_1f739c;
                                } else {
                                    break 'b_1f730c;
                                }
                            }
                        }
                    }
                    'b_1f7354: {
                        'b_1f733c: {
                            c.cycles += 8;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[A2][0] = 0x1;
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x200));
                            c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                            c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
                            if c.g[V0][0] == 0u64 {
                                c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                break 'b_1f733c;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V0][0] == c.g[A2][0];
                                c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                                if t {
                                    break 'b_1f7354;
                                } else {
                                    c.cycles += 2;
                                    let t = true;

                                    if t {
                                        break 'b_1f739c;
                                    } else {
                                        break 'b_1f733c;
                                    }
                                }
                            }
                        }
                        c.cycles += 6;
                        c.g[V0][0] = 0x27;
                        c.sw((c.g[GP][0] as u32).wrapping_sub(0x172c), c.g[V0][0]);
                        c.g[RA][0] = sx(0x001f734c);
                        c.g[A1][0] = 0u64;
                        if call_ptr(c, 0x00230488, 0x001f734c).is_err() {
                            return;
                        }
                        let t = true;
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                        if t {
                            break 'b_1f739c;
                        } else {
                            break 'b_1f7354;
                        }
                    }
                    c.cycles += 7;
                    c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = 0x1;
                    c.sw((c.g[A3][0] as u32).wrapping_add(0x54), c.g[V0][0]);
                    c.g[RA][0] = sx(0x001f7368);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    if call_ptr(c, 0x0039f190, 0x001f7368).is_err() {
                        return;
                    }
                    let t = true;
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                    if t {
                        break 'b_1f739c;
                    } else {
                        break 'b_1f7370;
                    }
                }
                c.cycles += 10;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A2][0] = 0x1;
                c.sw((c.g[A3][0] as u32).wrapping_add(0x54), c.g[A2][0]);
                c.g[A1][0] = 0x1;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x200));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10));
                c.g[RA][0] = sx(0x001f7398);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                if call_ptr(c, 0x0039f190, 0x001f7398).is_err() {
                    return;
                }
                break 'b_1f7398;
            }
        }
        c.cycles += 1;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        break 'b_1f739c;
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.pc = t;
    return;
}

/// `void OVScreenBCStart_TransitionIn(OVScreenBCStart *self, int a1)`
/// (0x1f7418): fade/slide the BC-start widgets in.
pub fn ov_screen_bcstart_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        'b_1f74c8: {
            'b_1f74c4: {
                c.cycles += 7;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S1][0] = c.g[A0][0];
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[A1][0] == 0u64;
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                if t {
                    break 'b_1f74c4;
                } else {
                    'b_1f748c: {
                        c.cycles += 15;
                        c.g[RA][0] = sx(0x001f743c);
                        call_ptr(c, 0x0014dc80, 0x001f743c)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001f7448);
                        c.g[A1][0] = 0xa;
                        call_ptr(c, 0x0014dd58, 0x001f7448)?;
                        c.g[A0][0] = 0x470000;
                        c.g[S2][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001f7458);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xb10));
                        call_ptr(c, 0x00317670, 0x001f7458)?;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x001f7464);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001f7464)?;
                        c.g[S0][0] = c.g[V0][0];
                        if c.g[S0][0] == 0u64 {
                            c.g[A0][0] = 0x4a0000;
                            break 'b_1f748c;
                        } else {
                            c.cycles += 7;
                            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x9c));
                            c.g[RA][0] = sx(0x001f747c);
                            c.g[A0][0] = c.g[S2][0];
                            call_ptr(c, 0x00153c88, 0x001f747c)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x001f7488);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a0e90, 0x001f7488)?;
                            c.g[A0][0] = 0x4a0000;
                            break 'b_1f748c;
                        }
                    }
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x001f7494);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x23f0));
                    call_ptr(c, 0x00317670, 0x001f7494)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001f74a0);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001f74a0)?;
                    c.g[S0][0] = c.g[V0][0];
                    if c.g[S0][0] == 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                        break 'b_1f74c8;
                    } else {
                        c.cycles += 6;
                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x9c));
                        c.g[RA][0] = sx(0x001f74b8);
                        c.g[A0][0] = c.g[S2][0];
                        call_ptr(c, 0x00153d28, 0x001f74b8)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x001f74c4);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a0d00, 0x001f74c4)?;
                        break 'b_1f74c4;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_1f74c8;
        }
        c.cycles += 6;
        c.g[V0][0] = 0u64;
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
// OVScreenTopTimes_OnActivate.

pub(crate) const UI_STATE_ON_ACTIVATE: u32 = 0x0039_e4c0;

pub(crate) const BE_LIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;

pub(crate) const BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;

pub(crate) const SPRINTF: u32 = 0x0041_7828;

pub(crate) const PLAY_OBJECT_ANIM: u32 = 0x0020_a6f0;

/// Format strings in `.rodata` and the global byte that selects the second
/// round of labels.
pub(crate) const LABEL_FORMATS: [u32; 4] = [0x004a_2440, 0x004a_2428, 0x004a_2430, 0x004a_2438];

pub(crate) const LABEL_MODE: u32 = 0x0053_5c11;

/// Four `sprintf(buf, fmt[i], count)` + `PlayObjectAnim(this, buf, anim)`
/// pairs, with the original's return addresses.
pub(crate) fn play_object_labels(
    c: &mut Cpu,
    this: u64,
    count: u64,
    rets: &[(u32, u32); 4],
) -> Flow {
    for i in 0..4 {
        c.g[A1][0] = LABEL_FORMATS[i] as u64;
        c.g[A0][0] = sp(c) as u64;
        c.g[A2][0] = count;
        call_ptr(c, SPRINTF, rets[i].0)?;
        c.g[S0][0] = 0x0047_0000;
        c.g[A1][0] = sp(c) as u64;
        c.g[S0][0] = sx(0x0047_0000u32.wrapping_sub(0x1b18));
        c.g[A0][0] = this;
        c.g[A2][0] = c.g[S0][0];
        call_ptr(c, PLAY_OBJECT_ANIM, rets[i].1)?;
    }
    Ok(())
}

/// `void OVScreenTopTimes_OnActivate(OVScreenTopTimes *s)` (0x1f7f08): format
/// and play the screen's label animations for the current mode.
pub fn ov_screen_top_times_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S2, 0x40);
        frame.save(c, S1, 0x50);
        frame.save(c, S3, 0x30);
        frame.save(c, S0, 0x60);
        frame.save_ra(c, 0x20);
        c.g[S2][0] = abi::arg64(c, 0);
        let this = c.g[S2][0];

        call_ptr(c, UI_STATE_ON_ACTIVATE, 0x001f_7f28)?;
        call_ptr(c, BE_LIBRARY_GET_INSTANCE, 0x001f_7f30)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 8;
        call_ptr(c, BE_LIBRARY_GET_INTERFACE, 0x001f_7f3c)?;
        c.g[S3][0] = c.g[V0][0];
        call_ptr(c, BE_LIBRARY_GET_INSTANCE, 0x001f_7f44)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, BE_LIBRARY_GET_INTERFACE, 0x001f_7f50)?;

        c.g[S1][0] = Addr(r32(c, S3)).off(0x18).lw(c);
        let negative = (c.g[S1][0] as i64) < 0;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
        if !negative {
            c.cycles += 38;
            play_object_labels(
                c,
                this,
                c.g[S1][0],
                &[
                    (0x001f_7f70, 0x001f_7f88),
                    (0x001f_7f9c, 0x001f_7fac),
                    (0x001f_7fc0, 0x001f_7fd0),
                    (0x001f_7fe4, 0x001f_7ff4),
                ],
            )?;
        }

        // The second block only runs for label mode 2.
        c.cycles += 5;
        c.g[V0][0] = 0x0053_0000;
        c.g[A0][0] = 2;
        c.g[V1][0] = Addr(LABEL_MODE).lb(c);
        if c.g[V1][0] == c.g[A0][0] {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, S3)).off(0x1c).lw(c);
            let negative = (c.g[V0][0] as i64) < 0;
            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            if !negative {
                c.cycles += 39;
                play_object_labels(
                    c,
                    this,
                    c.g[S1][0],
                    &[
                        (0x001f_8028, 0x001f_8040),
                        (0x001f_8054, 0x001f_8064),
                        (0x001f_8078, 0x001f_8088),
                        (0x001f_809c, 0x001f_80ac),
                    ],
                )?;
            }
        }

        c.cycles += 6;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fakes::*;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::SP as STACK_TOP;
    use crate::difftest::SP as STACK;
    use crate::difftest::SP;
    pub(crate) use crate::game::be::bedbprofile::tests::strict;
    pub(crate) use crate::game::bx::bxstring2::tests::base_3 as base_6;
    pub(crate) use crate::game::bx::bxstring2::tests::f;
    pub(crate) use crate::game::bx::bxstring2::tests::run as run_4;
    pub(crate) use crate::game::bx::bxstring2::tests::slot_2 as slot;
    pub(crate) use crate::game::bx::bxstring2::tests::BE_2 as BE_3;
    pub(crate) use crate::game::bx::bxstring2::tests::DB;
    pub(crate) use crate::game::bx::bxstring2::tests::RIDERS;
    pub(crate) use crate::game::bx::bxstring2::tests::RM as RM_2;
    pub(crate) use crate::game::bx::bxstring2::tests::VTS;
    pub(crate) use crate::game::bx::bxstring2::tests::W_2 as W_3;
    pub(crate) use crate::game::dirtysock::tags2::tests::base;
    pub(crate) use crate::game::dirtysock::tags2::tests::gp_at;
    pub(crate) use crate::game::dirtysock::tags2::tests::put_text;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_9 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_4 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_9 as heap_init_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_7 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_7 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_4 as run_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_2 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::W;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W_4;
    pub(crate) use crate::game::fe::fememcard2::tests::hash_screen;
    pub(crate) use crate::game::fe::fememcard2::tests::str_hash;
    pub(crate) use crate::game::fe::festateloadhint::cfe_state_progressive_verify_ps2_on_widget_notify;
    pub(crate) use crate::game::fe::festateoptions::tests::belib as belib_2;
    pub(crate) use crate::game::fe::festaterewards2::tests::belib;
    pub(crate) use crate::game::fe::festaterewards2::tests::w;
    pub(crate) use crate::game::fe::festaterewards2::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::fe::ovtemplatedialog::ov_screen_overlay_menu_on_widget_notify;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY_PTR;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS;
    pub(crate) use crate::game::main::gamestate::tests::base_3 as base_4;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_4;
    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base as alloc_base_2;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3;
    pub(crate) use crate::game::movie::movieplayer::tests::run_2 as run_3;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_2;
    pub(crate) use crate::game::movie::movieplayer::tests::W as W_2;
    pub(crate) use crate::game::world::worldview::tests::SEEDS_5 as SEEDS_3;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn ov_bcfail_pop_event_arg_matches() {
        check(
            ssx3_ee::s04::f_001f7738__generated,
            ov_bcfail_pop_event_arg,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0xf7738, 0);
                let q = DATA + 0x200;
                c.bus.write32(GAME_GP + 0xce8, q);
                let head = r.next_u32();
                c.bus.write32(q + 0x1c4, head);
                c.bus
                    .write32(q + 0x1c8, if seed % 3 == 0 { head } else { r.next_u32() });
                c.g[A0][0] = sx(DATA + 0x300);
            },
        );
    }

    #[test]
    fn ov_screen_top_times_on_widget_action_matches() {
        check(
            ssx3_ee::s04::f_001f80d8__generated,
            ov_screen_top_times_on_widget_action,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1f80_d8, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
                match seed % 3 {
                    0 => {
                        c.g[A1][0] = 0;
                        c.g[A2][0] = 5;
                    }
                    1 => {
                        c.g[A1][0] = (DATA + 0x300) as u64;
                        c.g[A2][0] = 4;
                    }
                    _ => {
                        c.g[A1][0] = (DATA + 0x300) as u64;
                        c.g[A2][0] = 5;
                        let vt = vtable(c, &mut r, &[0xd0]);
                        w(c, DATA + 8, vt);
                    }
                }
            },
        );
    }

    #[test]
    fn ov_screen_bc_start_pop_event_arg_matches() {
        check(
            ee::s04::f_001f7548__generated,
            ov_screen_bc_start_pop_event_arg,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1f75_48, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.bus.write32(GAME_GP + 0xce8, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x2a0, r.next_u32());
                let head = r.next_u32();
                c.bus.write32(DATA + 0x11c4, head);
                c.bus.write32(
                    DATA + 0x11c8,
                    if seed % 3 == 0 { head } else { r.next_u32() },
                );
            },
        );
    }

    #[test]
    fn ov_screen_top_times_v26_matches() {
        check(
            ee::s04::f_001f8118__generated,
            ov_screen_top_times_v26,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x1f81);
                let s = DATA;
                let widget = DATA + 0x400;
                c.bus.write32(s + 0xa0, if seed % 3 == 0 { 0 } else { 1 });
                c.bus.write32(
                    widget + 0x18,
                    match seed % 4 {
                        0 => 0,
                        _ => 2 + r.next_u32() % 6,
                    },
                );
                c.bus.write32(gp_at(-0x9f4), 0);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = sx(widget);
            },
        );
    }

    #[test]
    fn accept_key_6_family_matches() {
        check(
            ssx3_ee::s04::f_001f7590__generated,
            ov_screen_bc_start_on_widget_notify,
            SEEDS_3,
            &|c, seed| {
                let _ = machine(c, seed, 0x1f7590, 0);
                c.g[A2][0] = (seed % 13) as u64;
            },
        );
        check(
            ssx3_ee::s04::f_001f77f0__generated,
            ov_screen_bc_fail_on_widget_notify,
            SEEDS_3,
            &|c, seed| {
                let _ = machine(c, seed, 0x1f77f0, 0);
                c.g[A2][0] = (seed % 13) as u64;
            },
        );
        check(
            ssx3_ee::s04::f_001f80c8__generated,
            ov_screen_top_times_on_widget_notify,
            SEEDS_3,
            &|c, seed| {
                let _ = machine(c, seed, 0x1f80c8, 0);
                c.g[A2][0] = (seed % 13) as u64;
            },
        );
        check(
            ssx3_ee::s05::f_0020d308__generated,
            ov_screen_overlay_menu_on_widget_notify,
            SEEDS_3,
            &|c, seed| {
                let _ = machine(c, seed, 0x20d308, 0);
                c.g[A2][0] = (seed % 13) as u64;
            },
        );
        check(
            ssx3_ee::s06::f_00246268__generated,
            cfe_state_progressive_verify_ps2_on_widget_notify,
            SEEDS_3,
            &|c, seed| {
                let _ = machine(c, seed, 0x246268, 0);
                c.g[A2][0] = (seed % 13) as u64;
            },
        );
    }

    /// Runs a generated function to completion as a setup step.
    pub(crate) fn run(c: &mut Cpu, addr: u32, f: fn(&mut Cpu), args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(
            c.pc,
            crate::difftest::RA,
            "setup call {addr:#x} did not return"
        );
        c.g[V0][0]
    }

    // ---------------------------------------------------------- misc FE widgets

    #[test]
    fn ov_screen_bc_start_on_widget_created_matches() {
        check(
            ssx3_ee::s04::f_001f74e0__generated,
            ov_screen_bc_start_on_widget_created,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1f74e0);
                let h0 = run(c, 0x0031_7670, ssx3_ee::s10::StrHash, &[0x004a_23e0]);
                let h1 = run(c, 0x0031_7670, ssx3_ee::s10::StrHash, &[0x004a_23e8]);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                let id = match seed % 3 {
                    0 => h0,
                    1 => h1,
                    _ => r.next_u32() as u64,
                };
                c.bus.write32(DATA + 0x138, id as u32);
                c.bus.write32(DATA + 0x118, r.next_u32());
            },
        );
    }

    #[test]
    fn ov_screen_bc_start_init_matches() {
        check(
            ee::s04::f_001f73a8__generated,
            ov_screen_bc_start_init,
            30,
            &|c, seed| {
                alloc_base(c, seed, 0x1f73_a8);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(W + 0x84, 0);
            },
        );
    }

    #[test]
    fn ov_screen_enter_lodge_init_matches() {
        check(
            ee::s04::f_001f7198__generated,
            ov_screen_enter_lodge_init,
            16,
            &|c, seed| {
                machine(c, seed, 0x1f7198, 8);
                be_library(c);
                world(c);
                heap_init_2(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = this as u64;
                w(c, this + 0x10, DATA + 0x4000);
            },
        );
    }

    #[test]
    fn ov_screen_bc_fail_init_matches() {
        check(
            ee::s04::f_001f76b8__generated,
            ov_screen_bc_fail_init,
            8,
            &|c, seed| {
                let _ = alloc_base_2(c, seed, 0x1f76_b8, 4);
                let engine = DATA + 0x1000;
                w(c, DATA + 0x10, engine);
                w(c, engine, DATA + 0x1100);
                w(c, engine + 4, 0);
                w(c, DATA + 0x1100, 0);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    /// `str_hash(text)` computed on a scratch copy (the hasher overwrites the
    /// string it hashes with the result).
    pub(crate) fn hash_text(c: &mut Cpu, bytes: &[u8]) -> u32 {
        let tmp = DATA + 0x3f00;
        for (i, b) in bytes.iter().enumerate() {
            c.bus.write8(tmp + i as u32, *b);
        }
        c.bus.write8(tmp + bytes.len() as u32, 0);
        c.g[A0][0] = sx(tmp);
        c.g[RA][0] = sx(0x0012_3450);
        c.pc = 0x0031_7670;
        ee::s10::f_00317670(c);
        c.g[V0][0] as u32
    }

    #[test]
    fn ovscreenbcfail_on_widget_created_matches() {
        check(
            ee::s04::f_001f7770__generated,
            ovscreenbcfail_on_widget_created,
            SEEDS_4,
            &|c, seed| {
                base_4(c, seed, 0x7770, 0);
                // Short strings at the three name addresses the original hashes.
                for (addr, text) in [
                    (0x004a_23e0u32, b'A'),
                    (0x004a_23e8, b'B'),
                    (0x004a_23f8, b'C'),
                ] {
                    c.bus.write8(addr, text);
                    c.bus.write8(addr + 1, 0);
                }
                let h0 = hash_text(c, b"A");
                let h1 = hash_text(c, b"B");
                let h2 = hash_text(c, b"C");
                let widget = DATA + 0x100;
                c.g[A1][0] = sx(widget);
                let h = match seed % 4 {
                    0 => h0,
                    1 => h1,
                    2 => h2,
                    _ => 0xdead_beef,
                };
                c.bus.write32(widget + 0x38, h);
            },
        );
    }

    #[test]
    fn ov_screen_enter_lodge_on_activate_matches() {
        check(
            ssx3_ee::s04::f_001f7210__generated,
            ov_screen_enter_lodge_on_activate,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1f7210, 6);
                be_library(c);
                world(c);
                let st = DATA + 0x100;
                c.g[A0][0] = st as u64;
                c.g[A1][0] = r.next_u32() as u64;
                w(c, st + 0x40, DATA + 0x300);
                if seed % 3 != 0 {
                    let objs = DATA + 0x400;
                    let obj = DATA + 0x500;
                    w(c, DATA + 0x300 + 0x38, objs);
                    w(c, DATA + 0x300 + 0x3c, 1);
                    w(c, objs, obj);
                    w(c, obj + 0x38, 0);
                }
            },
        );
    }

    /// `GetHashValue32` over the bytes at_3 `addr` (the same algorithm as
    /// `StrHashElf`, for fixtures whose strings the test writes itself).
    pub(crate) fn hash(c: &mut Cpu, addr: u32) -> u32 {
        let mut h: u32 = 0;
        let mut at_3 = addr;
        loop {
            let b = c.bus.read8(at_3);
            if b == 0 {
                return h;
            }
            h = h.wrapping_shl(4).wrapping_add(b as i32 as u32);
            let tmp = h & 0xf000_0000;
            if tmp != 0 {
                h ^= tmp >> 23;
                h ^= tmp;
            }
            at_3 += 1;
        }
    }

    #[test]
    fn ov_screen_top_times_on_widget_created_matches() {
        check(
            ssx3_ee::s04::f_001f7e68__generated,
            ov_screen_top_times_on_widget_created,
            20,
            &|c, seed| {
                let _r = base_5(c, seed, 0x1f7e68, 2);
                let widget = DATA + 0x1000;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = widget as u64;
                put_text(c, 0x0046_f578, b"aaa");
                put_text(c, 0x0046_f588, b"bbb");
                put_text(c, 0x0046_f598, b"ccc");
                put_text(c, 0x0046_f5a8, b"ddd");
                let h1 = hash(c, 0x0046_f578);
                let h4 = hash(c, 0x0046_f5a8);
                let h = match seed % 4 {
                    0 | 1 => h1,
                    2 => h4,
                    _ => 0x1111_2222,
                };
                w_2(c, widget + 0x38, h);
                w_2(c, GAME_GP.wrapping_sub(0x9f0), (seed % 3 == 0) as u32);
            },
        );
    }

    #[test]
    fn cov_state_overlay_shutdown_matches() {
        check(
            ee::s05::f_0020a430__generated,
            cov_state_overlay_shutdown,
            40,
            &|c, seed| {
                let mut r = base_6(c, seed, 0x20a4);
                let screen = DATA + 0x1000;
                let node = DATA + 0x2000;
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x50, (seed % 2) as u32);
                c.bus.write32(DATA + 0x54, (seed / 2 % 2) as u32);
                c.bus.write32(DATA + 0x10, screen - 0x18);
                hash_screen(c, screen, node, 0x004a_20d0);
                c.bus.write32(node + 8, DATA + 0x3000);
                slot(c, DATA + 0x3000, 0xc0, 1);
                fill(c, &mut r, DATA + 0x100, 0x40);
            },
        );
    }

    #[test]
    fn ov_screen_bcstart_transition_in_matches() {
        strict(
            ee::s04::f_001f7418__generated,
            ov_screen_bcstart_transition_in,
            40,
            &|c, seed| {
                let _r = base_7(c, seed, 0x1f74);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.bus.write32(DATA + 0x40, 0);
                c.bus.write32(DATA + 0x9c, seed as u32);
            },
        );
    }

    #[test]
    fn ov_screen_top_times_on_activate_matches() {
        check(
            ee::s04::f_001f7f08__generated,
            ov_screen_top_times_on_activate,
            30,
            &|c, seed| {
                let _ = machine(c, seed, 0x1f_7f08, 12);
                belib_2(c);
                c.bus.write8(LABEL_MODE, if seed % 2 == 0 { 0 } else { 2 });
                // No screen: PlayObjectAnim takes its not-found path.
                w(c, DATA + 0x40, 0);
                c.g[A0][0] = DATA as u64;
            },
        );
    }
}
