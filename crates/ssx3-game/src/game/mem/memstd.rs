//! `src/mem/memstd.cpp`: hand-written replacements (moved from the
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
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw as lw_3;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::p_8 as p_5;
pub(crate) use crate::game::dirtysock::tags2::arg;
pub(crate) use crate::game::dirtysock::tags2::deleting_dtor;
pub(crate) use crate::game::dirtysock::tags2::vslot;
pub(crate) use crate::game::dirtysock::tags2::OPERATOR_DELETE;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_2;
pub(crate) use crate::game::fe::feasyncfile::gpa;
pub(crate) use crate::game::fe::feasyncfile::gpn;
pub(crate) use crate::game::fe::feasyncfile::lh_3 as lh_2;
pub(crate) use crate::game::fe::feasyncfile::lw as lw_2;
pub(crate) use crate::game::fe::feasyncfile::lw_9 as lw_4;
pub(crate) use crate::game::fe::feasyncfile::p as p_4;
pub(crate) use crate::game::fe::feasyncfile::p_7 as p_6;
pub(crate) use crate::game::fe::feasyncfile::sw_7 as sw_2;
pub(crate) use crate::game::fe::fepopup3::lh;
pub(crate) use crate::game::fe::fepopup3::lw_2 as lw;
pub(crate) use crate::game::fe::fepopup3::p_2;
pub(crate) use crate::game::fe::fepopup3::sw;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::fereal::gp;
pub(crate) use crate::game::fe::fereal::open;
pub(crate) use crate::game::intersect::riderspheretree::p_2 as p;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at_2;
pub(crate) use crate::game::main::gamestate::p_2 as p_3;
pub(crate) use crate::game::movie::movieplayer::lw_at;
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
    (0x0025_1f68, mem_debug_printf, "MEM_DebugPrintf"),
    (0x0025_2248, mem_is_valid_block_link, "MEM_IsValidBlockLink"),
    (0x0025_2660, mem_get_block_tag, "MEM_GetBlockTag"),
    (
        0x0025_26b8,
        mem_resize_block_locked,
        "MEM_ResizeBlockLocked",
    ),
    (0x0025_2fa0, mem_alloc_named128, "MEM_AllocNamed128"),
    (
        0x0025_3390,
        vid_engine_player_init_open_params,
        "VidEnginePlayer_InitOpenParams",
    ),
    (0x0025_33c8, vid_engine_player_ctor, "VidEnginePlayer_ctor"),
    (0x0025_3418, vid_engine_player_dtor, "VidEnginePlayer_dtor"),
    (
        0x0025_35f8,
        vid_engine_player_close,
        "VidEnginePlayer_Close",
    ),
    (
        0x0025_36c8,
        vid_engine_player_start_playback,
        "VidEnginePlayer_StartPlayback",
    ),
    (0x0025_3860, vid_engine_player_play, "VidEnginePlayer_Play"),
    (
        0x0025_3938,
        vid_engine_player_update,
        "VidEnginePlayer_Update",
    ),
    (
        0x0025_39e0,
        vid_engine_player_pause,
        "VidEnginePlayer_Pause",
    ),
    (
        0x0025_3a40,
        vid_engine_player_resume,
        "VidEnginePlayer_Resume",
    ),
    (
        0x0025_3aa0,
        vid_engine_install_mem_hooks,
        "VidEngine_InstallMemHooks",
    ),
    (0x0025_4350, vd_floor_log2, "VD_FloorLog2"),
    (0x0025_4368, vd_create_texture_sink, "VD_CreateTextureSink"),
    (
        0x0025_43f0,
        vd_texture_sink_set_frame_size,
        "VD_TextureSink_SetFrameSize",
    ),
    (0x0025_47b8, vd_texture_sink_dtor, "VD_TextureSink_dtor"),
    (0x0025_4dc0, vd_texture_sink_flush, "VD_TextureSink_Flush"),
    (0x0025_5638, vd_sink_base_dtor, "VD_SinkBase_dtor"),
    (
        0x0025_57e0,
        c_dsock_channel_base_ctor,
        "cDSockChannel_BaseCtor",
    ),
    (
        0x0025_5800,
        c_d_sock_channel_base_dtor,
        "cDSockChannel_BaseDtor",
    ),
    (
        0x0025_5830,
        c_dsock_channel_set_open,
        "cDSockChannel_SetOpen",
    ),
    (
        0x0025_5840,
        c_dsock_channel_reset_state,
        "cDSockChannel_ResetState",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

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
    pub(crate) use crate::game::dirtysock::tags2::callee::APP_SOMETHING;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_INSTANCE_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BENEW_PLAYER_INT_IS_MESSAGE_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_INTERFACE_RIDER;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_MESSAGE_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOLT_FILE_INDEX_MANAGER_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOUNDED_CAMERA_ALGORITHM_BASE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAMERA_TRIGGER_STACK_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SETUP_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SHARED_LOGIN_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_VOIP_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::DIRTY_SOCK_CREATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::DIRTY_SOCK_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE_2 as EA_MEM_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::EVENT_SELECT_ON_WIDGET_NOTIFY;
    pub(crate) use crate::game::dirtysock::tags2::callee::FEMODULE_SET_PREVIEW_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_STATE_GOTO_NEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_CHANNEL_RELEASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_CHANNEL_SET_LISTENER;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_HEAP_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_INSTANCE_PAUSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_INSTANCE_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOAD_INDICATOR_CLEAR_BUSY;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_API_ADD_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_LISTS;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_LOBBY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_PEER_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_REQUEST_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_REQUEST_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_SEND_LOCK_CHAL;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOC_STRING_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_CLOSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_FLUSH;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_READ;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_WRITE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_ALLOC_NAMED_ALIGNED;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_CTOR_2 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_ON_CHILD_EVENT_2 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_CACHE_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_END_PLAYBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_FREE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RESTORE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SNAPSHOT_EMPTY_HOOK_B;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_STREAM_WRITE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REWARDS_FILE_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ANIM_BANK_MAN_FIND_BANK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_CMD_QUEUE_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_CANCEL_BANK_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_CHOOSE_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_FIND_BANK_SLOT_BY_ID;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_IS_BANK_SLOT_LOADED;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS_2 as SCRIPT_ENGINE_POLL_ASYNC_LOADS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_FIELD_B0;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_PLAY_STATUS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_CURRENT_SKIPPABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MAN_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MAN_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_GET_WORLD_SECTION_INDEX;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_IS_EVENT_LOADED;
    pub(crate) use crate::game::dirtysock::tags2::callee::SHARED_LOGIN_MAN_SHUTDOWN;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_FREE_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_GET_BUFFER_SIZE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_RELEASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_APP_START_FE_MODULE;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_LIST_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_2 as UI_STATE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_CLOSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_MAN_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::WORLD_VIEW_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::WSCRIPT_MAN_DELETE_ALL_PROCESSES;
    pub(crate) use crate::game::dirtysock::tags2::callee::WSCRIPT_MAN_SPAWN_MISSIONS;
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
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_ALLOC_NAMED_3 as EA_MEM_ALLOC_NAMED_2;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_FREE_4 as EA_MEM_FREE_2;
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
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_SHUTDOWN_2;
    pub(crate) use crate::game::main::gamestate::callee::VD_TEXTURE_SINK_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ANIM_OBJECT_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BANK_MONITOR_RESET_INSTANCE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_CLEAR_LOGIN;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_CLEAR_MESSAGES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_DISCONNECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_BX_STRING_COPY_BEFORE_WRITE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_TRANSITION_MAN_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_STATE_GOTO_NEXT_3 as GAME_STATE_GOTO_NEXT_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INTC_TABLE_ADD_HANDLER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MOD_OBJ_CREATE_MODIFIER_BLOCK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MUSIC_PLAYER_GET_SELECTED_SONG_SED_VALUE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::OV_SCREEN_PROFILE_SAVE_PDA_REFRESH_CARD_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::OV_SCREEN_PROFILE_SAVE_PDA_RESET_SLOT_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RAIL_MODIFIER_LIST_PUSH_FRONT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_SAVE_EMIT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_2 as SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_LIST_STOP_2 as SCRIPT_ENGINE_LIST_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SPEECH_SYSTEM_CANCEL_PENDING_INSTANCE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SSX_AUDIO_SYSTEM_DJ_SAY_20E5;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SYNC_PROCESS_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VID_ENGINE_INSTALL_MEM_HOOKS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VID_ENGINE_PLAYER_INIT_OPEN_PARAMS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WSCRIPT_MAN_DEACTIVATE_PROCESS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WSCRIPT_MAN_UNLOAD_SECTION;
}

// ---------------------------------------------------------------------------
// Video engine.

/// `void VidEnginePlayer_InitOpenParams(VidEnginePlayer *self)`
/// (0x253390): reset the player's open parameters.
pub fn vid_engine_player_init_open_params(c: &mut Cpu) {
    c.cycles += 13;
    let this = Addr(abi::arg(c, 0));
    c.g[AT][0] = 0x3f80_0000;
    c.f[0] = c.g[AT][0] as u32; // mtc1 $at, $f0 (1.0f)
    c.g[V0][0] = 0x1f;
    this.off(0x18).sw(c, 0);
    this.off(0x20).write_fpu(c, FpuF32(c.f[0]));
    this.off(0x1c).sw(c, c.g[V0][0]);
    this.sw(c, 0);
    this.off(0x4).sw(c, 0);
    this.off(0xc).sw(c, 0);
    this.off(0x8).sw(c, 0);
    this.off(0x10).sw(c, 0);
    this.off(0x14).sw(c, 0);
    abi::ret(c);
}

/// `void VidEngine_InstallMemHooks(void)` (0x253aa0): write the two heap
/// hooks into the global hook record at 0x509430.
pub fn vid_engine_install_mem_hooks(c: &mut Cpu) {
    c.cycles += 11;
    c.g[V0][0] = 0x25_0000;
    c.g[A2][0] = 0x51_0000;
    c.g[V1][0] = 0x25_0000;
    c.g[V0][0] = sx(0x25_0000u32.wrapping_add(0x3ad0));
    c.g[A1][0] = sx(0x51_0000u32.wrapping_sub(0x6bd0));
    Addr(c.g[A1][0] as u32).sw(c, c.g[V0][0]);
    c.g[V1][0] = sx(0x25_0000u32.wrapping_add(0x3af8));
    c.g[A0][0] = 0x400;
    Addr(c.g[A1][0] as u32).off(0x8).sw(c, c.g[A0][0]);
    Addr(c.g[A1][0] as u32).off(0x4).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `int VD_FloorLog2(int x)` (0x254350): the classic float exponent trick —
/// bit-reinterpret `x`, convert it, extract the exponent and unbias it.
pub fn vd_floor_log2(c: &mut Cpu) {
    c.cycles += 6;
    // mtc1 $a0, $f0 then cvt.s.w $f0, $f0 (raw bits reinterpreted as an int).
    let conv = FpuF32::from_i32(c.g[A0][0] as u32 as i32);
    c.f[0] = conv.0;
    let exp = (conv.0 as i32) >> 23;
    c.g[V0][0] = sx((exp as u32).wrapping_sub(0x7f));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Networking (d-elf comm system).

/// `cDSockChannel *cDSockChannel_BaseCtor(cDSockChannel *self)` (0x2557e0):
/// clear the vtable-derived fields and install the base vtable at +0x8.
pub fn c_dsock_channel_base_ctor(c: &mut Cpu) {
    c.cycles += 7;
    let this = Addr(abi::arg(c, 0));
    c.g[V1][0] = 0x48_0000;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = sx(0x48_0000u32.wrapping_add(0x1320));
    this.sw(c, 0);
    this.off(0x8).sw(c, c.g[V1][0]);
    this.off(0x4).sw(c, 0);
    abi::ret(c);
}

/// `void cDSockChannel_SetOpen(cDSockChannel *self, int open)` (0x255830):
/// store `open` (only 1 is ever passed) at the state field and clear the
/// error field.
pub fn c_dsock_channel_set_open(c: &mut Cpu) {
    c.cycles += 4;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = 0x1;
    this.off(0x4).sw(c, 0);
    this.sw(c, c.g[V0][0]);
    abi::ret(c);
}

/// `void cDSockChannel_ResetState(cDSockChannel *self)` (0x255840): clear the
/// connection fields when the channel is open.
pub fn c_dsock_channel_reset_state(c: &mut Cpu) {
    c.cycles += 3;
    let this = Addr(abi::arg(c, 0));
    let state = this.lw(c);
    c.g[V0][0] = state;
    if state != 0 {
        c.cycles += 2;
        this.off(0x4).sw(c, 0);
        this.sw(c, 0);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void VD_TextureSink_SetFrameSize(VD_TextureSink* s, float a, float b,
/// float c)` (0x2543f0): the three frame dimensions at +0x40..0x48.
pub fn vd_texture_sink_set_frame_size(c: &mut Cpu) {
    c.cycles += 4;
    let selfp = abi::arg(c, 0);
    c.swc1(14, selfp.wrapping_add(0x48));
    c.swc1(12, selfp.wrapping_add(0x40));
    c.swc1(13, selfp.wrapping_add(0x44));
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Memory manager.

/// The tag of an allocated block whose size word has bit 0x800 set, at
/// `+4` of the tag record the block's `-0xc` word points at.
pub(crate) const BLOCK_HEADER_SIZE: u32 = 0xe;

pub(crate) const BLOCK_TAG_POINTER: u32 = 0xc;

pub(crate) const BLOCK_TAGGED: u32 = 0x800;

/// `int MEM_GetBlockTag(void* block)` (0x252660): the block's tag string, or
/// 0 when the block is not tagged.
pub fn mem_get_block_tag(c: &mut Cpu) {
    let p = arg(c, 0);
    c.cycles += 4;
    c.g[V0][0] = Addr(p.wrapping_sub(BLOCK_HEADER_SIZE)).lhu(c);
    c.g[V0][0] &= BLOCK_TAGGED as u64;
    let tagged = c.g[V0][0] != 0;
    c.g[V1][0] = 0;
    if tagged {
        c.cycles += 4;
        c.g[V0][0] = Addr(p.wrapping_sub(BLOCK_TAG_POINTER)).lw(c);
        c.g[V0][0] = sx(p.wrapping_add(c.g[V0][0] as u32));
        let rec = c.g[V0][0] as u32;
        c.lwl(V1, rec.wrapping_add(7));
        c.lwr(V1, rec.wrapping_add(4));
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[V1][0];
    abi::ret(c);
}

/// `void* MEM_AllocNamed128(const char* name, size_t size, uint32_t flags)`
/// (0x252fa0): named allocation with 128-byte alignment
/// (`MEM_AllocNamedAligned(name, size, 0x80, 0, flags, 1)`).
pub fn mem_alloc_named128(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[T0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = 0x80;
        c.g[A3][0] = 0;
        c.g[T1][0] = 1;
        callee::MEM_ALLOC_NAMED_ALIGNED.call(c, 0x0025_2fbc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Video, lobby and setup managers.

/// `VidEnginePlayer_Play(this)` 0x253860: start playback when a video file is
/// queued (+0x38); reports whether it started.
pub fn vid_engine_player_play(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        let file = lw_at(c, this as u64, 0x38);
        c.g[V0][0] = file;
        let none = file == 0;
        c.g[V0][0] = 0;
        if !none {
            c.cycles += 3;
            callee::VID_ENGINE_PLAYER_START_PLAYBACK.call(c, 0x0025_387c)?;
            c.g[V0][0] = 1;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void VD_SinkBase_dtor(VD_SinkBase *self, int flags)` (0x255638):
/// republishes `D_004802B0` at `self+4`, deletes on bit 0.
pub fn vd_sink_base_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_02b0, 4, 0x0025_565c))
}

/// `void cDSockChannel_BaseDtor(cDSockChannel *self, int flags)` (0x255800):
/// republishes `D_00481320` at `self+8`, deletes on bit 0.
pub fn c_d_sock_channel_base_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_1320, 8, 0x0025_5824))
}

/// `void VidEnginePlayer_dtor(VidEnginePlayer* self, int flags)` (0x253418):
/// close the player, then `operator delete` it when `flags & 1`.
pub fn vid_engine_player_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        callee::VID_ENGINE_PLAYER_CLOSE.call(c, 0x0025_3434)?;
        c.g[S0][0] = c.g[S0][0] & 1;
        let keep = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, callee::BUILTIN_DELETE, 0x0025_3448)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Video / buddies / login.

/// `VidEnginePlayer* VidEnginePlayer_ctor(VidEnginePlayer* self)`
/// (0x2533c8): install the memory hooks, init the open parameters and clear
/// the playback fields.
pub fn vid_engine_player_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        callee::VID_ENGINE_INSTALL_MEM_HOOKS.call(c, 0x0025_33dc)?;
        c.g[A0][0] = sx(this.wrapping_add(0x14));
        callee::VID_ENGINE_PLAYER_INIT_OPEN_PARAMS.call(c, 0x0025_33e4)?;
        c.lwc1(0, p(this, 0x34));
        c.g[V0][0] = c.g[S0][0];
        Addr(this).off(0x38).sw(c, 0);
        c.swc1(0, p(this, 0x10));
        Addr(this).off(0x3c).sw(c, 0);
        Addr(this).off(0xc).sw(c, 0);
        Addr(this).off(0x40).sw(c, 0);
        Addr(this).off(8).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `sd reg, off(sp)`: a register the original spills through the frame.
#[inline(always)]
pub(crate) fn save_reg(c: &mut Cpu, frame: Frame, reg: usize, off: u32) {
    let a = frame.at(c, off);
    let v = c.g[reg][0];
    c.sd(a, v);
}

// ---------------------------------------------------------------------------
// Debug printf and movie/audio helpers.

/// `void MEM_DebugPrintf(const char* fmt, ...)` (0x251f68): format the
/// varargs into a 0x200-byte stack buffer and hand it to the debug output.
pub fn mem_debug_printf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x290);
        save_reg(c, frame, A1, 0x258);
        save_reg(c, frame, A2, 0x260);
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0x200);
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x258));
        save_reg(c, frame, A3, 0x268);
        save_reg(c, frame, T0, 0x270);
        save_reg(c, frame, T1, 0x278);
        save_reg(c, frame, T2, 0x280);
        save_reg(c, frame, T3, 0x288);
        c.swc1(12, p_2(c.g[SP][0], 0x248));
        c.swc1(14, p_2(c.g[SP][0], 0x24c));
        c.swc1(16, p_2(c.g[SP][0], 0x250));
        c.swc1(18, p_2(c.g[SP][0], 0x254));
        call_ptr(c, 0x0041_86c8, 0x0025_1fac)?;
        c.g[A0][0] = 2;
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x003e_5d30, 0x0025_1fb8)?;
        frame.restore_ra(c, 0x200);
        frame.close(c);
        Ok(())
    })
}

/// `void VidEnginePlayer_Resume(VidEnginePlayer* self)` (0x253a40): resume
/// playback if the player has data, is enabled, is not paused and is stopped.
pub fn vid_engine_player_resume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = lw(c, p_2(c.g[S0][0], 0x38));
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = lw(c, p_2(c.g[S0][0], 0x3c));
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[V0][0] = lw(c, p_2(c.g[S0][0], 0));
                if c.g[V0][0] == 0 {
                    c.cycles += 3;
                    c.g[V0][0] = lw(c, p_2(c.g[S0][0], 8));
                    if c.g[V0][0] != 0 {
                        c.cycles += 4;
                        call_ptr(c, 0x003a_e958, 0x0025_3a88)?;
                        sw(c, p_2(c.g[S0][0], 8), 0);
                    }
                }
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void VD_TextureSink_Flush(VD_TextureSink* self)` (0x254dc0): spin until
/// the sink's device reports completion, then release the pending buffer.
pub fn vd_texture_sink_flush(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = lw(c, p_2(c.g[S0][0], 8));
        loop {
            c.cycles += 7;
            c.g[V1][0] = lw(c, p_2(c.g[A1][0], 0x10d8));
            c.g[A0][0] = lh(c, p_2(c.g[V1][0], 0x90));
            c.g[V0][0] = lw(c, p_2(c.g[V1][0], 0x94));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0025_4dec)?;
            if c.g[V0][0] != 0 {
                break;
            }
            c.g[A1][0] = lw(c, p_2(c.g[S0][0], 8));
            if event_due(c) {
                deliver_event_here(c, 0x0025_4dd8);
            }
        }
        c.cycles += 3;
        c.g[A1][0] = lw(c, p_2(c.g[S0][0], 0xc));
        if c.g[A1][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = lw(c, p_2(c.g[S0][0], 0));
            call_ptr(c, 0x003b_0680, 0x0025_4e08)?;
            sw(c, p_2(c.g[S0][0], 0xc), 0);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* MEM_ResizeBlockLocked(void* block, size_t size)` (0x2526b8):
/// resize a block under the allocator lock.
pub fn mem_resize_block_locked(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = sx(0x0053_8b00);
        frame.save(c, S1, 0x20);
        let block = c.g[A0][0];
        let size = c.g[A1][0];
        c.g[S2][0] = block;
        frame.save_ra(c, 0);
        c.g[S1][0] = size;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_5700, 0x0025_26e4)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0031_9bd8, 0x0025_26f0)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_5760, 0x0025_26fc)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void VidEnginePlayer_Pause(VidEnginePlayer* player)` (0x2539e0): pause
/// the player once its stream is ready and idle.
pub fn vid_engine_player_pause(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let player = c.g[A0][0];
        c.g[S0][0] = player;
        c.g[A0][0] = Addr(at(c, S0, 0x38)).lw(c);
        let mut done = c.g[A0][0] == 0;
        if !done {
            c.cycles += 3;
            c.g[V0][0] = Addr(at(c, S0, 0x3c)).lw(c);
            done = c.g[V0][0] == 0;
            if !done {
                c.cycles += 3;
                c.g[V0][0] = Addr(at(c, S0, 0x0)).lw(c);
                done = c.g[V0][0] != 0;
                if !done {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(at(c, S0, 0x8)).lw(c);
                    done = c.g[V0][0] != 0;
                    if !done {
                        c.cycles += 5;
                        call_ptr(c, 0x003a_e938, 0x0025_3a28)?;
                        c.g[V1][0] = 0x1;
                        Addr(at(c, S0, 0x8)).sw(c, c.g[V1][0]);
                    }
                }
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Small list / object helpers.

/// `void MEM_IsValidBlockLink(void* p)` (0x252248): true if `p` is a valid
/// allocator pointer whose block magic is `"BF"` or `"BS"`; otherwise record
/// the failure in `D_004A2E74` and return false.
pub fn mem_is_valid_block_link(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        open(c, 0x30);
        c.g[A1][0] = 0x18;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[A2][0] = 0u64;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[RA][0] = sx(0x0025_226c);
        c.g[S1][0] = 0u64;
        call_ptr(c, 0x003e_6240, 0x0025_226c)?;
        let mut ok = 0u64;
        if c.g[V0][0] == 0 {
            // Not a pointer EA considers valid: flag the error.
            c.cycles += 1;
            c.cycles += 1;
            c.sw(gp(c, -0x27c), 1);
        } else {
            c.cycles += 3;
            let magic = Addr(c.g[S0][0] as u32).lhu(c);
            c.g[A0][0] = magic;
            c.g[V0][0] = 0x4253;
            if magic == 0x4246 {
                c.cycles += 2;
                ok = 1;
            } else {
                c.cycles += 2;
                if magic == 0x4253 {
                    c.cycles += 2;
                    ok = 1;
                } else {
                    c.cycles += 1;
                    c.sw(gp(c, -0x27c), 1);
                }
            }
        }
        c.cycles += 6;
        c.g[V0][0] = ok;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x30);
        Ok(())
    })
}

/// `void VidEnginePlayer_StartPlayback(VidEnginePlayer* p)` (0x002536c8):
/// restart the AV player on the player's current frame, pause it, create the
/// texture sink for the decoded frames and resume.
pub fn vid_engine_player_start_playback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x20));
        c.g[A1][0] = 1;
        c.sq(S0, sp(c).wrapping_add(0x10));
        c.g[A2][0] = 0x21;
        c.sd(sp(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.sw(c.g[S0][0] as u32, 0);
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
        c.sw((c.g[S0][0] as u32).wrapping_add(4), 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0);
        call_ptr(c, 0x003a_e450, 0x0025_36f4)?; // AV_Player_Start
        c.g[V1][0] = 1;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x40), c.g[V1][0]);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        call_ptr(c, 0x003a_e938, 0x0025_3708)?; // AV_Player_Pause
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
        c.g[A3][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x30));
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
        c.g[A2][0] = c.lw(gpa(c, 0x2a90));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x64));
        call_ptr(c, 0x0025_4368, 0x0025_3720)?; // VD_CreateTextureSink
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x3c), c.g[V0][0]);
        call_ptr(c, 0x003a_e958, 0x0025_372c)?; // AV_Player_Resume
        c.g[V1][0] = c.lw(gpn(c, 0x254));
        c.lq(S0, sp(c).wrapping_add(0x10));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.g[RA][0] = c.ld(sp(c));
        c.sw(gpn(c, 0x254), c.g[V1][0]);
        c.g[SP][0] = sx(sp(c).wrapping_add(0x20));
        abi::ret(c);
        Ok(())
    })
}

/// `vd_CreateTextureSink(sink, w, h, fmt)` (0x254368): allocate a
/// `VD_TextureSink` through the render-device hook and construct it.
pub fn vd_create_texture_sink(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = 0x510000;
        frame.save(c, 16, 0x40);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6bd0));
        frame.save(c, 17, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, 18, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, 19, 0x10);
        c.g[S2][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A0][0];
        c.g[A0][0] = 0x4a0000;
        c.g[A1][0] = 0x54;
        c.g[T0][0] = lw_at_2(c, p_3(c.g[V0][0] as u32, 8));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2ea0));
        c.g[V0][0] = lw_at_2(c, (c.g[V1][0] as u32).wrapping_sub(0x6bd0));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, c.g[V0][0] as u32, 0x0025_43b8)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[T0][0] = c.g[S2][0];
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, callee::VD_TEXTURE_SINK_CTOR.addr, 0x0025_43d0)?;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void VidEnginePlayer_Render(self, int force)` (0x253890): drive the
/// player object's present or its frame-advance method.
pub fn vid_engine_player_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p_4(r32(c, S0), 0x38)).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A2][0] = Addr(p_4(r32(c, S0), 0x3c)).lw(c);
            let no_obj = c.g[A2][0] == 0;
            c.g[A3][0] = c.g[A2][0];
            if !no_obj {
                // 1: the object's vtable slot (a3 + delta); 2: the player's
                // own word field; 3: the object's other slot.
                let mut via = 0u32;
                c.cycles += 2;
                if c.g[A1][0] != 0 {
                    c.g[V0][0] = Addr(p_4(r32(c, A3), 4)).lw(c);
                    via = 1;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p_4(r32(c, S0), 8)).lw(c);
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = Addr(p_4(r32(c, A3), 4)).lw(c);
                        via = 1;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = Addr(p_4(r32(c, S0), 0x40)).lw(c);
                        if c.g[V0][0] != 0 {
                            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                            via = 2;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = Addr(p_4(r32(c, S0), 4)).lw(c);
                            if c.g[V0][0] != 0 {
                                c.g[V0][0] = Addr(p_4(r32(c, A3), 4)).lw(c);
                                via = 1;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                                via = 2;
                            }
                        }
                        if via == 2 {
                            c.cycles += 2;
                            if c.g[V0][0] == 0 {
                                c.g[V0][0] = Addr(p_4(r32(c, A2), 4)).lw(c);
                                via = 3;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = Addr(p_4(r32(c, A3), 4)).lw(c);
                                via = 1;
                            }
                        }
                    }
                }
                if via == 1 {
                    c.cycles += 6;
                    c.g[A0][0] = Addr(p_4(r32(c, V0), 0x28)).lh(c);
                    c.g[V1][0] = Addr(p_4(r32(c, V0), 0x2c)).lw(c);
                    let present = r32(c, V1);
                    c.g[A0][0] = sx(p_4(r32(c, A3), r32(c, A0)));
                    call_ptr(c, present, 0x00253904)?;
                    frame.restore(c, S0, 0x10);
                } else {
                    if via != 0 {
                        c.cycles += 6;
                        c.g[A1][0] = Addr(p_4(r32(c, S0), 0xc)).lw(c);
                        c.g[A0][0] = Addr(p_4(r32(c, V0), 0x20)).lh(c);
                        c.g[V1][0] = Addr(p_4(r32(c, V0), 0x24)).lw(c);
                        let advance = r32(c, V1);
                        c.g[A0][0] = sx(p_4(r32(c, A2), r32(c, A0)));
                        call_ptr(c, advance, 0x00253920)?;
                        Addr(p_4(r32(c, S0), 0x40)).sw(c, 0);
                    }
                    c.cycles += 1;
                    frame.restore(c, S0, 0x10);
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void VidEnginePlayer_Update(player)` (0x253938): wait for the current
/// frame, decode it, and when both video and audio are done restart.
pub fn vid_engine_player_update(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        {
            'first: {
                'due: {
                    c.cycles += 7;
                    frame.save(c, S0, 0x10);
                    frame.save_ra(c, 0);
                    c.g[S0][0] = c.g[A0][0];
                    c.g[V0][0] = lw(c, at_2(c.g[S0][0], 0x40));
                    let t = c.g[V0][0] != 0;
                    c.g[V0][0] = 1;
                    if t {
                        break 'due;
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0025_395c);
                    c.g[A0][0] = lw(c, at_2(c.g[S0][0], 0x38));
                    call_ptr(c, 0x003a_e780, 0x0025_395c)?;
                    let t = c.g[V0][0] == 0;
                    c.g[V0][0] = 1;
                    if t {
                        break 'due;
                    }
                    c.cycles += 14;
                    c.g[A1][0] = lw(c, at_2(c.g[S0][0], 0x3c));
                    c.g[V0][0] = lw(c, at_2(c.g[A1][0], 4));
                    c.g[A0][0] = lh(c, at_2(c.g[V0][0], 0x30));
                    c.g[V1][0] = lw(c, at_2(c.g[V0][0], 0x34));
                    let target = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0025_397c);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x0025_397c)?;
                    c.g[A0][0] = lw(c, at_2(c.g[S0][0], 0x38));
                    c.g[RA][0] = sx(0x0025_3988);
                    c.lwc1(12, at_2(c.g[A0][0], 0x48));
                    call_ptr(c, 0x003a_e6c8, 0x0025_3988)?;
                    c.g[V1][0] = 1;
                    Addr(at_2(c.g[S0][0], 0xc)).sw(c, c.g[V0][0]);
                    Addr(at_2(c.g[S0][0], 0x40)).sw(c, c.g[V1][0]);
                    Addr(at_2(c.g[S0][0], 4)).sw(c, 0);
                    break 'first;
                }
                c.cycles += 1;
                Addr(at_2(c.g[S0][0], 4)).sw(c, c.g[V0][0]);
                break 'first;
            }
            'sec: {
                c.cycles += 3;
                c.g[V0][0] = lw(c, at_2(c.g[S0][0], 0xc));
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x10);
                    break 'sec;
                }
                c.cycles += 3;
                c.g[V0][0] = lw(c, c.g[S0][0] as u32);
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x10);
                    break 'sec;
                }
                c.cycles += 7;
                c.g[V0][0] = 1;
                Addr(at_2(c.g[S0][0], 0x40)).sw(c, 0);
                c.g[A0][0] = lw(c, at_2(c.g[S0][0], 0x38));
                c.g[RA][0] = sx(0x0025_39cc);
                Addr(at_2(c.g[S0][0], 4)).sw(c, c.g[V0][0]);
                call_ptr(c, 0x003a_e860, 0x0025_39cc)?;
                Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
                frame.restore(c, S0, 0x10);
                break 'sec;
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void VidEnginePlayer_Close(VidEnginePlayer *self)` (0x2535f8): release the
/// player's texture/handle objects and clear its fields.
pub fn vid_engine_player_close(c: &mut Cpu) {
    guest(c, |c| {
        'b_253638: {
            c.cycles += 7;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
            if c.g[V0][0] == 0u64 {
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                break 'b_253638;
            } else {
                'b_253634: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3c));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a90));
                    if t {
                        break 'b_253634;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10d8));
                        vslot(c, c.g[V0][0], 0x98, V1);
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x00253634);
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x00253634)?;
                        break 'b_253634;
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                break 'b_253638;
            }
        }
        'b_25365c: {
            c.cycles += 2;
            if c.g[V0][0] == 0u64 {
                c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3c));
                break 'b_25365c;
            } else {
                c.cycles += 3;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                if c.g[A1][0] == 0u64 {
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3c));
                    break 'b_25365c;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x00253658);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x64));
                    call_ptr(c, 0x003b0680, 0x00253658)?;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x3c));
                    break 'b_25365c;
                }
            }
        }
        'b_253688: {
            c.cycles += 2;
            let t = c.g[A2][0] == 0u64;
            c.g[A1][0] = 0x3;
            if t {
                break 'b_253688;
            } else {
                c.cycles += 9;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x4));
                vslot(c, c.g[V0][0], 0x8, V1);
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00253678);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                call_ptr(c, t, 0x00253678)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x254));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x3c), 0u64);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                c.sw((c.g[GP][0] as u32).wrapping_sub(0x254), c.g[V0][0]);
                break 'b_253688;
            }
        }
        'b_2536a4: {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x38));
            if c.g[A0][0] == 0u64 {
                c.sw((c.g[S0][0] as u32).wrapping_add(0x40), 0u64);
                break 'b_2536a4;
            } else {
                c.cycles += 4;
                c.g[RA][0] = sx(0x0025369c);
                c.g[A1][0] = 0x3;
                call_ptr(c, 0x003ae300, 0x0025369c)?;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x38), 0u64);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x40), 0u64);
                break 'b_2536a4;
            }
        }
        c.cycles += 8;
        c.sw((c.g[S0][0] as u32), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

/// `MEM_ValidateBlock(block)` (0x2522b0): validate an allocated block's
/// magic (`BS`, `BF`, `BM`), its flag word and (for `BM`) the end marker,
/// recording the error code in `gp - 0x27c` and returning 1 when valid.
pub fn mem_validate_block(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        let block = abi::arg(c, 0);
        frame.save(c, 16, 0x10);
        c.g[V0][0] = 0x4253;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V1][0] = c.lhu(c.g[S0][0] as u32);
        let t = c.g[V1][0] != c.g[V0][0];
        c.g[T0][0] = 0;
        // 0: done without the error store, 1: error code in V0, 2: error
        // code written to gp - 0x27c
        let mut state = 0u32;
        if !t {
            c.cycles += 4;
            c.g[V0][0] = c.lhu(p_5(c.g[S0][0] as u32, 2));
            c.g[V0][0] &= 0x8000;
            if c.g[V0][0] != 0 {
                c.g[T0][0] = 1;
                state = 1;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 2;
                state = 2;
            }
        }
        if state == 0 {
            // not `BS`: `BF` (free) and `BM` (mixed) blocks
            c.cycles += 3;
            c.g[V0][0] = 0x4246;
            let t = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = 0x424d;
            let mut direct = false;
            if !t {
                c.cycles += 4;
                c.g[V0][0] = c.lhu(p_5(c.g[S0][0] as u32, 2));
                c.g[V0][0] &= 0x4000;
                if c.g[V0][0] == 0 {
                    direct = true;
                } else {
                    c.cycles += 5;
                    c.g[A0][0] = lw_3(c, p_5(c.g[S0][0] as u32, 0x10));
                    call_ptr(c, 0x0025_2248, 0x0025_230c)?;
                    c.g[T0][0] = c.g[V0][0];
                    if c.g[T0][0] == 0 {
                        direct = true;
                    } else {
                        c.cycles += 4;
                        c.g[A0][0] = lw_3(c, p_5(c.g[S0][0] as u32, 0x14));
                        call_ptr(c, 0x0025_2248, 0x0025_2320)?;
                        c.g[T0][0] = c.g[V0][0];
                        state = 1;
                    }
                }
            } else {
                c.cycles += 2;
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 4;
                if t {
                    state = 2;
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = c.lhu(p_5(c.g[S0][0] as u32, 2));
                    c.g[V0][0] &= 0x2000;
                    let t = c.g[V0][0] == 0;
                    c.g[A2][0] = 0x4245_0000;
                    if t {
                        c.cycles += 2;
                        c.g[T0][0] = 1;
                        state = 1;
                    } else {
                        c.cycles += 16;
                        c.g[V1][0] = lw_3(c, p_5(c.g[S0][0] as u32, 4));
                        c.g[A2][0] |= 0x4e44;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[S0][0] as u32)));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                        c.g[V0][0] = c.lbu(c.g[V1][0] as u32);
                        c.g[A0][0] = c.lbu(p_5(c.g[V1][0] as u32, 1));
                        c.g[A1][0] = c.lbu(p_5(c.g[V1][0] as u32, 2));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                        c.g[A0][0] = sx((c.g[A0][0] as u32) << 16);
                        c.g[A3][0] = c.lbu(p_5(c.g[V1][0] as u32, 3));
                        c.g[V0][0] |= c.g[A0][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32) << 8);
                        c.g[V0][0] |= c.g[A1][0];
                        c.g[V0][0] |= c.g[A3][0];
                        let t = c.g[V0][0] != c.g[A2][0];
                        c.g[V0][0] = 3;
                        if t {
                            state = 2;
                        } else {
                            c.cycles += 2;
                            c.g[T0][0] = 1;
                            state = 1;
                        }
                    }
                }
            }
            if direct {
                state = 3;
            }
        }
        if state == 2 {
            c.cycles += 1;
            c.sw(gp(c, -0x27c), c.g[V0][0]);
        }
        if state != 3 {
            c.cycles += 1;
        }
        c.cycles += 4;
        c.g[V0][0] = c.g[T0][0];
        frame.restore_ra(c, 0);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}

/// `VD_TextureSink_dtor(self, flags)` (0x002547b8): release every texture of
/// the sink, then free the table, the name and the decoder.
pub fn vd_texture_sink_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = 0;
        frame.save(c, S0, 0x60);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[S2][0];
        c.g[S5][0] = c.g[A1][0];
        c.g[V0][0] = sx(0x0048_0270);
        sw_2(c, this, 4, c.g[V0][0]);

        'scan: {
            c.g[V1][0] = lw_4(c, this, 0x18);
            if (c.g[V1][0] as i64) <= 0 {
                break 'scan;
            }
            c.cycles += 4;
            c.g[S4][0] = 0xffff_ffff_ffff_ffff;
            c.g[S3][0] = sx(0x0050_9430);
            c.g[V1][0] = lw_4(c, this, 0x14);
            loop {
                // Release texture `i`.
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[S1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[S0][0] = lw_4(c, c.g[V0][0], 0);
                let empty = c.g[S0][0] == 0;
                c.g[A2][0] = Addr(gp(c, 0x2a90)).lw(c);
                if !empty {
                    c.cycles += 10;
                    c.g[A1][0] = lw_4(c, c.g[S0][0], 0);
                    c.g[V0][0] = lw_4(c, c.g[A2][0], 0x10d8);
                    c.g[A0][0] = lh_2(c, c.g[V0][0], 0x190);
                    c.g[V1][0] = lw_4(c, c.g[V0][0], 0x194);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, c.g[V1][0] as u32, 0x0025_4834)?;
                    sw_2(c, c.g[S0][0], 0, c.g[S4][0]);
                    c.g[V0][0] = lw_4(c, c.g[S3][0], 4);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, c.g[V0][0] as u32, 0x0025_4844)?;
                }
                c.cycles += 5;
                c.g[V0][0] = lw_4(c, this, 0x18);
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break 'scan;
                }
                c.g[V1][0] = lw_4(c, this, 0x14);
                if event_due(c) {
                    deliver_event_here(c, 0x0025_4808);
                }
            }
        }

        // Free the texture table.
        c.cycles += 3;
        c.g[A0][0] = lw_4(c, this, 0x14);
        let no_table = c.g[A0][0] == 0;
        let mut name;
        if no_table {
            name = lw_4(c, this, 0xc);
        } else {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x0025_486c)?;
            name = lw_4(c, this, 0xc);
        }
        c.g[A1][0] = name;
        // Free the name.
        c.cycles += 2;
        let no_name = c.g[A1][0] == 0;
        sw_2(c, this, 0x14, 0);
        if !no_name {
            c.cycles += 3;
            c.g[A0][0] = lw_4(c, this, 0);
            call_ptr(c, 0x003b_0680, 0x0025_4880)?;
            sw_2(c, this, 0xc, 0);
        }
        // Virtual destructor tail: restore the base-object vtable and, for a
        // deleting destructor, call the operator delete.
        c.cycles += 5;
        c.g[V0][0] = sx(0x0048_02b0);
        c.g[V1][0] = c.g[S5][0] & 1;
        sw_2(c, this, 4, c.g[V0][0]);
        if c.g[V1][0] != 0 {
            c.cycles += 4;
            c.g[V1][0] = sx(0x0051_0000);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_sub(0x6bcc)).lw(c);
            c.g[A0][0] = this;
            call_ptr(c, c.g[V0][0] as u32, 0x0025_48a8)?;
        }
        c.cycles += 9;
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fakes::*;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::RA as RET;
    use crate::difftest::RA as SENTINEL;
    pub(crate) use crate::game::be::bedbprofile::tests::strict;
    pub(crate) use crate::game::bx::bxstring::tests::alloc_ready;
    pub(crate) use crate::game::bx::bxstring::tests::step;
    pub(crate) use crate::game::camera::camera::tests::alloc_base;
    pub(crate) use crate::game::camera::camera::tests::heap_init as heap_init_3;
    pub(crate) use crate::game::camera::camera::tests::pool_allocator_3 as pool_allocator_2;
    pub(crate) use crate::game::camera::camera::tests::run as run_4;
    pub(crate) use crate::game::camera::camera::tests::BE_5 as BE_4;
    pub(crate) use crate::game::camera::camera::tests::RM as RM_2;
    pub(crate) use crate::game::camera::camera::tests::W_2 as W_3;
    pub(crate) use crate::game::dirtysock::tags2::tests::be_library_2 as be_library;
    pub(crate) use crate::game::dirtysock::tags2::tests::dtor_setup;
    pub(crate) use crate::game::dirtysock::tags2::tests::heap_init;
    pub(crate) use crate::game::dirtysock::tags2::tests::rng_2;
    pub(crate) use crate::game::dirtysock::tags2::tests::run;
    pub(crate) use crate::game::dirtysock::tags2::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::debugfe::tests::w;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::check_all_2 as check_all;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_6 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::gp_at;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_10 as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_3 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_10 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::put_str_2 as put_str;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_8 as run_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_4 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_9 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::wb;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_6 as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::IFACE;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM_3 as RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_3 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_4 as W;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W_2;
    pub(crate) use crate::game::fe::fepopup3::tests::setup;
    pub(crate) use crate::game::fe::fereal::tests::base;
    pub(crate) use crate::game::main::gamestate::tests::base_3;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_3;
    pub(crate) use crate::game::movie::movieplayer::tests::globals;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_LIBRARY;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS_2;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS;
    use ssx3_ee as ee;

    #[test]
    fn vid_engine_player_init_open_params_matches() {
        check(
            ssx3_ee::s06::f_00253390,
            vid_engine_player_init_open_params,
            100,
            &|c, seed| {
                let mut r = Rng(0x2533_9000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                for off in [0u32, 4, 8, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20] {
                    c.bus.write32(DATA + off, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn vid_engine_install_mem_hooks_matches() {
        check(
            ssx3_ee::s06::f_00253aa0,
            vid_engine_install_mem_hooks,
            50,
            &|c, seed| {
                let mut r = Rng(0x253a_a000 ^ seed as u64);
                scramble(c, &mut r);
                for addr in [0x509430u32, 0x509434, 0x509438] {
                    c.bus.write32(addr, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn vd_floor_log2_matches() {
        check(ssx3_ee::s06::f_00254350, vd_floor_log2, 300, &|c, seed| {
            let mut r = Rng(0x2543_5000 ^ seed as u64);
            scramble(c, &mut r);
            let bits = match seed % 3 {
                0 => r.edge_f32(),
                1 => r.next_u32(),
                _ => (seed as u32).wrapping_mul(0x9e37_79b9),
            };
            c.g[A0][0] = ((r.next_u32() as u64) << 32) | bits as u64;
        });
    }

    #[test]
    fn c_dsock_channel_base_ctor_matches() {
        check(
            ssx3_ee::s06::f_002557e0,
            c_dsock_channel_base_ctor,
            100,
            &|c, seed| {
                let mut r = Rng(0x2557_e000 ^ seed as u64);
                scramble(c, &mut r);
                let base = DATA + (seed % 4) * 4;
                c.g[A0][0] = base as u64;
                for off in [0u32, 4, 8] {
                    c.bus.write32(base + off, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn c_dsock_channel_set_open_matches() {
        check(
            ssx3_ee::s06::f_00255830,
            c_dsock_channel_set_open,
            50,
            &|c, seed| {
                let mut r = Rng(0x2558_3000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA, r.next_u32());
                c.bus.write32(DATA + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn c_dsock_channel_reset_state_matches() {
        check(
            ssx3_ee::s06::f_00255840,
            c_dsock_channel_reset_state,
            200,
            &|c, seed| {
                let mut r = Rng(0x2558_4000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.bus
                    .write32(DATA, if seed % 2 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(DATA + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn vd_texture_sink_set_frame_size_matches() {
        check(
            ee::s06::f_002543f0__generated,
            vd_texture_sink_set_frame_size,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0025_43f0, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.f[12] = r.next_u32();
                c.f[13] = r.edge_f32();
                c.f[14] = r.next_u32();
            },
        );
    }

    // -------------------------------------------------------------- memory

    #[test]
    fn mem_get_block_tag_matches() {
        check(
            ssx3_ee::s06::f_00252660__generated,
            mem_get_block_tag,
            250,
            &|c, seed| {
                let mut r = rng_2(seed, 0x25);
                scramble(c, &mut r);
                let block = DATA + 0x8000;
                let tagged = seed % 2 == 0;
                let size_word = if tagged {
                    0x800u16 | (r.next_u32() as u16 & 0x7fff)
                } else {
                    r.next_u32() as u16 & 0x7ff
                };
                c.bus.write16(block - 0xe, size_word);
                // The word at -0xc is the signed offset from the block to the tag
                // record; the value is the unaligned word at record + 4.
                let tag = DATA + 0x9000;
                let extra = match seed % 4 {
                    0 => 0u32,
                    1 => 0x10,
                    2 => 0x1000,
                    _ => (-0x20i32) as u32,
                };
                c.bus.write32(
                    block - 0xc,
                    (tag as u32).wrapping_sub(block as u32).wrapping_add(extra),
                );
                for i in 0..8u32 {
                    c.bus.write32(tag + 4 * i, r.next_u32());
                }
                c.g[A0][0] = sx(block);
            },
        );
    }

    #[test]
    fn mem_alloc_named128_matches() {
        check(
            ssx3_ee::s06::f_00252fa0__generated,
            mem_alloc_named128,
            60,
            &|c, seed| {
                let mut r = rng_2(seed, 0x26);
                scramble(c, &mut r);
                heap_init(c);
                let name = DATA + 0x9000;
                let len = 1 + (seed % 8) as u32;
                for i in 0..len {
                    c.bus.write8(name + i, b'A' + (r.next_u32() % 26) as u8);
                }
                c.bus.write8(name + len, 0);
                c.g[A0][0] = sx(name);
                c.g[A1][0] = (8 + (seed as u64 % 0x400)) as u64;
                c.g[A2][0] = (r.next_u32() & 0x0f00_0000) as u64;
            },
        );
    }

    #[test]
    fn vid_engine_player_play_matches() {
        check(
            ssx3_ee::s06::f_00253860__generated,
            vid_engine_player_play,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0xbf, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
                c.bus
                    .write32(DATA + 0x38, if seed % 2 == 0 { 0 } else { DATA + 0x3000 });
            },
        );
    }

    #[test]
    fn vd_sink_base_dtor_matches() {
        check(
            ee::s06::f_00255638__generated,
            vd_sink_base_dtor,
            60,
            &|c, seed| dtor_setup(c, seed, 0x255638),
        );
    }

    #[test]
    fn c_d_sock_channel_base_dtor_matches() {
        check(
            ee::s06::f_00255800__generated,
            c_d_sock_channel_base_dtor,
            60,
            &|c, seed| dtor_setup(c, seed, 0x255800),
        );
    }

    #[test]
    fn vid_engine_player_dtor_matches() {
        check(
            ssx3_ee::s06::f_00253418__generated,
            vid_engine_player_dtor,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x253418, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = (seed & 1) as u64;
            },
        );
    }

    // ------------------------------------------------------------------
    // Video / buddies / login.

    #[test]
    fn vid_engine_player_ctor_matches() {
        check(
            ee::s06::f_002533c8__generated,
            vid_engine_player_ctor,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2533_c8, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x20);
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x34, r.edge_f32());
                c.bus.write32(GAME_GP + 0x2898, 0);
            },
        );
    }

    // -------------------------------------------------- debug and movie

    #[test]
    fn mem_debug_printf_matches() {
        check(
            ssx3_ee::s06::f_00251f68__generated,
            mem_debug_printf,
            100,
            &|c, seed| {
                let mut r = setup(c, seed, 0x251f68);
                put_str(
                    c,
                    DATA + 0x100,
                    if seed % 3 == 0 { "hello" } else { "hi %d" },
                );
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = r.next_u32() as u64;
                c.g[T1][0] = r.next_u32() as u64;
                c.g[T2][0] = r.next_u32() as u64;
                c.g[T3][0] = r.next_u32() as u64;
                for i in 0..4 {
                    c.f[12 + i] = r.edge_f32();
                }
            },
        );
    }

    #[test]
    fn vid_engine_player_resume_matches() {
        check(
            ssx3_ee::s06::f_00253a40__generated,
            vid_engine_player_resume,
            150,
            &|c, seed| {
                let _r = setup(c, seed, 0x253a40);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let stage = seed % 5;
                c.bus.write32(this + 0x38, if stage == 0 { 0 } else { 1 });
                c.bus.write32(this + 0x3c, if stage <= 1 { 0 } else { 1 });
                c.bus.write32(this + 0x00, if stage <= 2 { 0 } else { 1 });
                c.bus.write32(this + 0x08, if stage <= 3 { 0 } else { 1 });
            },
        );
    }

    #[test]
    fn vd_texture_sink_flush_matches() {
        check(
            ssx3_ee::s06::f_00254dc0__generated,
            vd_texture_sink_flush,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x254dc0);
                let this = DATA;
                c.g[A0][0] = this as u64;
                // A device whose completion method is a fake (non-zero result, so
                // the spin loop is left at once).
                c.bus.write32(this + 8, DATA + 0x900);
                let vt = vtable(c, &mut r, &[0x90]);
                c.bus.write32(DATA + 0x900 + 0x10d8, vt);
                c.bus.write32(this + 0xc, if seed % 3 == 0 { 0 } else { 1 });
                c.bus.write32(this + 0, DATA + 0xa00);
                let vt = vtable(c, &mut r, &[0x30]);
                c.bus.write32(DATA + 0xa00 + 0x14, vt);
            },
        );
    }

    /// Run one generated function as a setup step, with `args` in a0..t1.
    pub(crate) fn run_2(c: &mut Cpu, addr: u32, f: fn(&mut Cpu), args: &[u64]) -> u64 {
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

    pub(crate) fn heap_block(c: &mut Cpu, size: u32) -> u32 {
        heap_init(c);
        run_2(
            c,
            0x0032_0058,
            ssx3_ee::s10::MEM_HeapMemalign,
            &[MEM_HEAP as u64, 16, size as u64],
        ) as u32
    }

    #[test]
    fn mem_resize_block_locked_matches() {
        check(
            ssx3_ee::s06::f_002526b8__generated,
            mem_resize_block_locked,
            10,
            &|c, seed| {
                fakes::machine(c, seed, 0x2526b8, 0);
                let block = heap_block(c, 0x40);
                c.g[A0][0] = block as u64;
                c.g[A1][0] = 0x20 + (seed as u64 % 0x40);
            },
        );
    }

    #[test]
    fn vid_engine_player_pause_matches() {
        check(
            ssx3_ee::s06::f_002539e0__generated,
            vid_engine_player_pause,
            80,
            &|c, seed| {
                fakes::machine(c, seed, 0x2539e0, 0);
                w(c, OBJ + 0x38, if seed % 2 == 0 { 0 } else { seed });
                w(c, OBJ + 0x3c, if seed % 3 == 0 { 0 } else { seed });
                w(c, OBJ, (seed / 2) % 2);
                w(c, OBJ + 0x8, (seed / 4) % 2);
                c.g[A0][0] = OBJ as u64;
            },
        );
    }

    #[test]
    fn mem_is_valid_block_link_matches() {
        check(
            ee::s06::f_00252248__generated,
            mem_is_valid_block_link,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x25_2248);
                // `EA_IsValidPointer` accepts the low EE window (< 0x100000) and
                // the high kernel windows; main RAM (0x800000) is rejected.
                let valid_ptr = seed % 4 != 0;
                c.g[A0][0] = if valid_ptr { 0x1000 } else { sx(DATA + 0x400) };
                let magic = match seed % 3 {
                    0 => 0x4246u32,
                    1 => 0x4253,
                    _ => r.next_u32() & 0xffff,
                };
                c.bus.write16(0x1000, magic as u16);
                c.bus.write32(GAME_GP.wrapping_sub(0x27c), 0xbeef);
            },
        );
    }

    #[test]
    fn vid_engine_player_start_playback_matches() {
        check(
            ee::s06::f_002536c8__generated,
            vid_engine_player_start_playback,
            40,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x2536, 3);
                let p = DATA;
                w_2(c, p + 0x38, DATA + 0x400);
                w_2(c, p + 0x30, seed);
                fill(c, &mut r, DATA + 0x400, 0x40);
                c.g[A0][0] = sx(p);
            },
        );
    }

    #[test]
    fn vd_create_texture_sink_matches() {
        covered(
            ee::s06::f_00254368__generated,
            vd_create_texture_sink,
            SEEDS_3,
            &|c, seed| {
                base_3(c, seed, 0x54368, 1);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = (seed % 0x40) as u64;
                c.g[A2][0] = ((seed / 0x40) % 0x40) as u64;
                c.g[A3][0] = (seed % 4) as u64;
                c.bus.write32(0x0050_9430, FAKE);
                c.bus.write32(0x0050_9438, 0x1234);
            },
        );
    }

    // ----------------------------------------------------------- video player

    #[test]
    fn vid_engine_player_update_matches() {
        check(
            ee::s06::f_00253938__generated,
            vid_engine_player_update,
            SEEDS_4,
            &|c, seed| {
                let _ = base_4(c, seed, 0x2539_38, 6);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                let player = obj + 0x2000;
                w_3(c, obj + 0x38, player);
                w_3(c, obj + 0x3c, obj + 0x2100);
                w_3(c, obj + 0x40, (seed % 2) as u32);
                w_3(c, obj + 0xc, (seed % 3 == 0) as u32);
                w_3(c, obj, (seed % 4 == 0) as u32);
                // The player's audio track object and its vtables.
                w_3(c, obj + 0x2100 + 4, obj + 0x2200);
                slot_2(c, obj + 0x2200, 0x30, 0);
                w_3(c, player + 0x48, 0);
                w_3(c, player + 0x470, 0);
            },
        );
    }

    #[test]
    fn vid_engine_player_close_matches() {
        strict(
            ee::s06::f_002535f8__generated,
            vid_engine_player_close,
            60,
            &|c, seed| {
                let _r = base_5(c, seed, 0x2535);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(gp_at(0x2a90), DATA + 0x2000);
                c.bus.write32(DATA + 0x38, 0);
                c.bus
                    .write32(DATA + 0x3c, if seed % 3 == 0 { 0 } else { DATA + 0x3000 });
                c.bus
                    .write32(DATA + 0x40, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(DATA + 0xc, (seed % 2) as u32);
                c.bus.write32(DATA + 0x3004, DATA + 0x3100);
                c.bus.write16(DATA + 0x3100 + 8, 0);
                c.bus.write32(DATA + 0x3100 + 0xc, FAKE);
            },
        );
    }

    /// A real heap block of `size` bytes.
    pub(crate) fn alloc(c: &mut Cpu, size: u32) -> u32 {
        run_4(
            c,
            0x0031_7d70,
            ee::s10::f_00317d70,
            &[size as u64, 0x0048_0000, 0, 0],
        ) as u32
    }

    #[test]
    fn vd_texture_sink_dtor_matches() {
        check_all(
            ee::s06::f_002547b8__generated,
            vd_texture_sink_dtor,
            60,
            &|c, seed| {
                let _r = alloc_base(c, seed, 0x2547);
                let this_ = DATA + 0x1000;
                let n = seed % 4;
                let table = alloc(c, 4 * (n + 1));
                for i in 0..n {
                    if seed % 2 == 0 || i == 0 {
                        let tex = alloc(c, 0x10);
                        c.bus.write32(table + 4 * i, tex);
                    } else {
                        c.bus.write32(table + 4 * i, 0);
                    }
                }
                c.bus.write32(this_ + 0x18, n);
                c.bus.write32(this_ + 0x14, table);
                c.bus.write32(this_ + 0xc, 0);
                c.bus.write32(this_, 0);
                c.g[A0][0] = this_ as u64;
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }
}
