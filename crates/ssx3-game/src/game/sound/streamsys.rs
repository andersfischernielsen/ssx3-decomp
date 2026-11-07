//! `src/sound/streamsys.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::tail_wrapper;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Target;
use crate::call::Unwound;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::game::animation::animsequencer::ASL_PUSH_NEW;
use crate::game::animation::animsequencer::ASL_SET_TRACK_ANIM;
use crate::game::animation::animsequencer::ASL_SET_TRACK_LOOP;
use crate::game::animation::animsequencer::ASL_SET_TRACK_TIME;
pub(crate) use crate::game::dirtysock::tags2::lw;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::feasyncfile::gp_8 as gp;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::p as p_2;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at;
pub(crate) use crate::game::main::gamestate::p_2 as p_3;
pub(crate) use crate::game::movie::movieplayer::at as at_2;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::replay::replay2::destroy_if_flag;
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
    (0x002a_9df0, streambuffer_dtor, "StreamBuffer_dtor"),
    (0x002a_9e78, stream_buffer_set_file, "StreamBuffer_SetFile"),
    (
        0x002a_9f30,
        stream_buffer_set_base_volume,
        "StreamBuffer_SetBaseVolume",
    ),
    (
        0x002a_a428,
        stream_buffer_get_state,
        "StreamBuffer_GetState",
    ),
    (
        0x002a_a7f0,
        stream_array_create_stream,
        "StreamArray_CreateStream",
    ),
    (
        0x002a_ab98,
        stream_array_set_stream_file,
        "StreamArray_SetStreamFile",
    ),
    (
        0x002a_abd0,
        stream_array_destroy_stream,
        "StreamArray_DestroyStream",
    ),
    (0x002a_ae40, stream_array_stop, "StreamArray_Stop"),
    (
        0x002a_b0d0,
        stream_array_set_volume_scale_ptr,
        "StreamArray_SetVolumeScalePtr",
    ),
    (
        0x002a_b128,
        stream_array_get_stream_handle,
        "StreamArray_GetStreamHandle",
    ),
    (
        0x002a_b188,
        stream_array_get_stream,
        "StreamArray_GetStream",
    ),
    (
        0x002a_b9e0,
        bank_monitor_instance_ctor,
        "BankMonitorInstance_ctor",
    ),
    (
        0x002a_ba18,
        bank_monitor_instance_dtor,
        "BankMonitorInstance_dtor",
    ),
    (
        0x002a_bca0,
        bank_monitor_calc_doppler_pitch,
        "BankMonitor_CalcDopplerPitch",
    ),
    (
        0x002a_bce8,
        sound_request_set_bank_sample,
        "SoundRequest_SetBankSample",
    ),
    (
        0x002a_cb30,
        bank_monitor_instance_resume,
        "BankMonitorInstance_Resume",
    ),
    (0x002a_de88, sound_manager_ref_ctor, "SoundManagerRef_ctor"),
    (0x002a_dee8, sound_manager_ref_dtor, "SoundManagerRef_dtor"),
    (
        0x002a_c7f0,
        bank_monitor_instance_update_range,
        "BankMonitorInstance_UpdateRange",
    ),
    (
        0x002a_9f80,
        stream_buffer_update_volume,
        "StreamBuffer_UpdateVolume",
    ),
    (
        0x002a_a020,
        stream_buffer_get_status,
        "StreamBuffer_GetStatus",
    ),
    (
        0x002a_ac28,
        stream_array_update_all,
        "StreamArray_UpdateAll",
    ),
    (
        0x002a_a068,
        stream_buffer_get_request_status,
        "StreamBuffer_GetRequestStatus",
    ),
    (0x002a_ae08, stream_array_get_state, "StreamArray_GetState"),
    (
        0x002a_a108,
        stream_buffer_get_time_remaining,
        "StreamBuffer_GetTimeRemaining",
    ),
    (
        0x002a_a1b8,
        stream_buffer_is_playing,
        "StreamBuffer_IsPlaying",
    ),
    (
        0x002a_ae78,
        stream_array_get_time_remaining,
        "StreamArray_GetTimeRemaining",
    ),
    (
        0x002a_b028,
        stream_array_is_playing,
        "StreamArray_IsPlaying",
    ),
    (
        0x002a_bb38,
        bank_monitor_instance_reset,
        "BankMonitorInstance_Reset",
    ),
    (
        0x002a_c180,
        bank_monitor_instance_fade_out,
        "BankMonitorInstance_FadeOut",
    ),
    (
        0x002a_bd48,
        bank_monitor_instance_update_distance,
        "BankMonitorInstance_UpdateDistance",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_BAKE_GROUP;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_EXPORT_HISTORY_FILES;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_FREE_HISTORIES;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_LOAD_HISTORY_FILE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_INSTANCE_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_REWARD_INT_IS_UNLOCKED;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_DESTROY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_SET_PRESENCE_ADDR;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY_2 as CHAT_ROOM_UPDATE_PLAYER_QUALITY;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_BX_STRING_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_LUNO_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_GET_SAVE_NAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_LIST_DIR_ALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_READ_SAVE;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SETUP_MAN_SEND_QUIT as C_SETUP_MAN_SEND_QUIT_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEMSET;
    pub(crate) use crate::game::dirtysock::tags2::callee::EMPTY_CHILD_HANDLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_MODULE_GET_PLAYER_PAD_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_UPDATE_CURSOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_LIST_GET_AT_3 as LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_END_OPERATION_2 as MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_REQUEST_CARD_INFO;
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
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_GET_INBOX;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_CTOR_3 as NET_FE_STATE_CTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_DTOR_2 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_ON_CHILD_EVENT_3 as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE_2 as NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::ON_QUIT_POPUP_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::PATH_PLAT_REFRESH_STATUS;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_ADD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RECORD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCE_MC_FORMAT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_OBJ_TABLE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_OBJ_CAM_SET_OWNER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_PUSH_FREE_NODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SLOGIN_MC_GET_PORT_SLOT_2 as SLOGIN_MC_GET_PORT_SLOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_FREE_MEMORY_2 as SND_BANK_FREE_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLINK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SNOW_BOX_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPRINTF_2 as SPRINTF;
    pub(crate) use crate::game::dirtysock::tags2::callee::STATS_SELECT_CATEGORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY as STRCPY_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::STREAM_BUFFER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::TROPHY_ROOM_RELEASE_IMAGES;
    pub(crate) use crate::game::dirtysock::tags2::callee::UIMENU_SET_SELECTION;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF as UI_OBJECT_SHOW_CLAMP_ALPHA_HALF_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_3 as UI_STATE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_TEXT_GET_FONT;
    pub(crate) use crate::game::dirtysock::tags2::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::main::gamestate::callee::ADF_DATA_SOURCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::ANIM_DATA_BASE_FREE_CHAN_CACHES;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_STOP_HANDLE as BANK_MONITOR_STOP_HANDLE_2;
    pub(crate) use crate::game::main::gamestate::callee::BE_ECON_INT_GET_TOTAL_EARNED;
    pub(crate) use crate::game::main::gamestate::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INSTANCE_3 as BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INTERFACE_3 as BE_LIBRARY_GET_INTERFACE_2;
    pub(crate) use crate::game::main::gamestate::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::main::gamestate::callee::BOLTS_FIND_BOLT_INDEX;
    pub(crate) use crate::game::main::gamestate::callee::BUILTIN_DELETE_4;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_BRAGGING_RIGHTS_INIT_PLAYER_ROWS;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_CHAR_STATS;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::main::gamestate::callee::C_GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::main::gamestate::callee::C_KEYBOARD_POPUP_SET_KEY_ENABLED;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_ALLOC_NAMED_3 as EA_MEM_ALLOC_NAMED_2;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_FREE_4 as EA_MEM_FREE;
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
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_NEXT as MENU_SYS_PAGE_SELECT_NEXT_2;
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
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_FIRST_2;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_CTOR_3;
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
    pub(crate) use crate::game::movie::movieplayer::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_3;
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
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN_DISCONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_CREATE_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_JOIN_ROOM_RESULT;
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
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_CTOR_5 as NET_FE_STATE_CTOR_3;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_HANDLE_NET_EVENT;
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
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_FIND_SOUND_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_RUN_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_UNLOAD_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::SPRINTF_2;
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
    pub(crate) use crate::game::worldpainter::quadtree::callee::AI_VFX_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::BUILTIN_DELETE_2;
    pub(crate) use crate::game::worldpainter::quadtree::callee::CONSOLE_CONFIG_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EA_MEM_ALLOC;
    pub(crate) use crate::game::worldpainter::quadtree::callee::EMITTER_RENDER;
    pub(crate) use crate::game::worldpainter::quadtree::callee::GET_TRANSFORM_BOUNDS;
    pub(crate) use crate::game::worldpainter::quadtree::callee::GRAPHICS_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::GRAPHICS_ON_VIF1_DMA_END;
    pub(crate) use crate::game::worldpainter::quadtree::callee::INTC_REMOVE_HANDLER;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MEM_CARD_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MEM_CARD_INIT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MEM_CARD_SET_STATE;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MENU_ITEM_MEASURE_ROW;
    pub(crate) use crate::game::worldpainter::quadtree::callee::MOD_OBJ_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::PS2_SOUND_MANAGER_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SNDSTRM_GET_REQUEST_STATUS;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SONG_FADE_OUT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SONG_GET_STREAM_STATUS;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SONG_POST_EVENT;
    pub(crate) use crate::game::worldpainter::quadtree::callee::SPEECH_INSTANCE_STOP_STREAM;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STATS_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STRCPY;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STREAM_BUFFER_SET_FILE;
    pub(crate) use crate::game::worldpainter::quadtree::callee::STREAM_BUFFER_STOP;
    pub(crate) use crate::game::worldpainter::quadtree::callee::VFX_MAN_CTOR;
    pub(crate) use crate::game::worldpainter::quadtree::callee::WORLD_GET_SECTION_SCRIPT_FUNC;
    pub(crate) use crate::game::worldpainter::quadtree::callee::WORLD_OBJ_CTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::wscript::wscriptman::callee::APP_MODULE_UPDATE_FADE;
    pub(crate) use crate::game::wscript::wscriptman::callee::AVALANCHE_BAKE_NODE_HISTORY;
    pub(crate) use crate::game::wscript::wscriptman::callee::AV_PLAYER_INIT;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_INSTANCE_POLL_FREE;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_INSTANCE_SET_PITCH;
    pub(crate) use crate::game::wscript::wscriptman::callee::CAM_CONTROLLER_CLEAR_ALGORITHMS;
    pub(crate) use crate::game::wscript::wscriptman::callee::CPS2_CHUNK_RESOLVE_CTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::CPS2_MEMCARD_SET_STATE;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_AUDIO_INSTANCE_STOP_SOUND;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_MAN3D_GROUP_CREATE_MESHES_2 as CROWD_MAN3D_GROUP_CREATE_MESHES;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_MAN3D_GROUP_FREE_MESHES;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_EMPTY_CHILD_HANDLER;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_FLAG_MANAGER_GROUP_LOAD;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_SETUP_MAN_SEND_QUIT;
    pub(crate) use crate::game::wscript::wscriptman::callee::DEBUG_MENU_MODULE_UPDATE_CURSOR_TRAIL;
    pub(crate) use crate::game::wscript::wscriptman::callee::DISABLE_INTC;
    pub(crate) use crate::game::wscript::wscriptman::callee::EA_FILE_MOUNT_BIG;
    pub(crate) use crate::game::wscript::wscriptman::callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::wscript::wscriptman::callee::FLEX_RAIL_UPDATE_SPACE_NODE;
    pub(crate) use crate::game::wscript::wscriptman::callee::FLOATING_V49;
    pub(crate) use crate::game::wscript::wscriptman::callee::FOCUS_MAN_ADD_FOCUS_BOX;
    pub(crate) use crate::game::wscript::wscriptman::callee::LIGHT_MAN_EMPTY_HOOK_A;
    pub(crate) use crate::game::wscript::wscriptman::callee::MODIFIER_LIST_UNLINK;
    pub(crate) use crate::game::wscript::wscriptman::callee::MOD_OBJ_DTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::NET_FE_STATE_CTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::wscript::wscriptman::callee::RAIL_MODIFIER_LIST_UNLINK;
    pub(crate) use crate::game::wscript::wscriptman::callee::REMOVE_INTC_HANDLER;
    pub(crate) use crate::game::wscript::wscriptman::callee::SET_FADE_GAIN;
    pub(crate) use crate::game::wscript::wscriptman::callee::SND_BANK_UNLOAD;
    pub(crate) use crate::game::wscript::wscriptman::callee::TEX_MAN_LOAD_TEXTURE;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_GET_STATE_DATA;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_SET_USER_DATA;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::wscript::wscriptman::callee::WSCRIPT_MAN_CALL_FUNC;
}

/// Cross-shard calls the generator emitted as `call_indirect(c, addr)`: they
/// resolve through the function table like the original's `jal`.
pub(crate) mod table_callee {
    pub(crate) use crate::game::be::bedbreward::table_callee::EA_MEMSET;
    pub(crate) use crate::game::be::bedbreward::table_callee::OPERATOR_DELETE;
    pub(crate) use crate::game::be::bedbreward::table_callee::WORLD_CREATE_DEAD_NODE;
}

/// Cross-shard direct calls the generator emitted as `call_indirect(c, addr)`.
pub(crate) mod table {
    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_DELETE;
    pub(crate) use crate::game::wscript::wscriptman::table::BUILTIN_VEC_DELETE;
    pub(crate) use crate::game::wscript::wscriptman::table::BXFILE_EXISTS;
    pub(crate) use crate::game::wscript::wscriptman::table::GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::wscript::wscriptman::table::IS_REPLAY_PLAYING;
    pub(crate) use crate::game::wscript::wscriptman::table::MEM_FREE;
    pub(crate) use crate::game::wscript::wscriptman::table::SCE_MC_FLUSH;
    pub(crate) use crate::game::wscript::wscriptman::table::SERIALIZE_READ_U32;
    pub(crate) use crate::game::wscript::wscriptman::table::SERIALIZE_READ_U32_2;
    pub(crate) use crate::game::wscript::wscriptman::table::SND_BANK_REMOVE;
    pub(crate) use crate::game::wscript::wscriptman::table::SND_MIDI_RESUME;
    pub(crate) use crate::game::wscript::wscriptman::table::STREAM_FILE_DESTROY;
    pub(crate) use crate::game::wscript::wscriptman::table::STR_HASH;
    pub(crate) use crate::game::wscript::wscriptman::table::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::wscript::wscriptman::table::WORLD_GET_OBJECT_SCRIPT_FUNC;
    pub(crate) use crate::game::wscript::wscriptman::table::WSCRIPT_PROCESS_LIST_POP_FRONT;
}

/// Offset of the stream-array element table pointer.
pub(crate) const STREAM_ARRAY_TABLE: u32 = 0x8;

/// Offset of a stream's volume-scale pointer.
pub(crate) const STREAM_VOLUME_SCALE_PTR: u32 = 0x10;

/// `void StreamArray_SetVolumeScalePtr(StreamArray* a, int i, float* p)`
/// (0x002ab0d0): `streams[i]->volumeScalePtr = p` (no-op if slot `i` is null).
/// `a1` is left holding the stream pointer (or 0).
pub fn stream_array_set_volume_scale_ptr(c: &mut Cpu) {
    c.cycles += 6;
    let array = Addr(abi::arg(c, 0));
    c.g[V0][0] = array.off(STREAM_ARRAY_TABLE).lw(c);
    c.g[A1][0] = sx(abi::arg(c, 1) << 2);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A1][0] = Addr(c.g[A1][0] as u32).lw(c);
    if c.g[A1][0] != 0 {
        Addr(c.g[A1][0] as u32)
            .off(STREAM_VOLUME_SCALE_PTR)
            .sw(c, c.g[A2][0]);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void* StreamArray_GetStream(StreamArray* a, int i)` (0x002ab188): returns
/// `streams[i]`. `v0` first holds the table pointer, then the element; `a1` is
/// left holding the element address.
pub fn stream_array_get_stream(c: &mut Cpu) {
    c.cycles += 5;
    let array = Addr(abi::arg(c, 0));
    c.g[V0][0] = array.off(STREAM_ARRAY_TABLE).lw(c);
    c.g[A1][0] = sx(abi::arg(c, 1) << 2);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Stream array.

/// `int StreamArray_GetStreamHandle(StreamArray *self, int index)`
/// (0x2ab128): the +0x8 field of the stream at `self->streams[index]`, or -1
/// when the slot is empty.
pub fn stream_array_get_stream_handle(c: &mut Cpu) {
    const STREAMS: u32 = 0x8;
    c.cycles += 6;
    let arr = Addr(abi::arg(c, 0));
    let base = arr.off(STREAMS).lw(c);
    c.g[V0][0] = base;
    let slot = sx((abi::arg(c, 1) << 2).wrapping_add(base as u32));
    let entry = Addr(slot as u32).lw(c);
    c.g[A1][0] = entry;
    c.g[V0][0] = u64::MAX;
    if entry != 0 {
        c.cycles += 2;
        c.g[V0][0] = Addr(entry as u32).off(STREAMS).lw(c);
    } else {
        c.cycles += 2;
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Bank monitor.

/// `BankMonitorInstance_ctor(BankMonitorInstance *self)` (0x2ab9e0): set the
/// default distance/gain sentinels and store `-1.0` at +0x90.
pub fn bank_monitor_instance_ctor(c: &mut Cpu) {
    c.cycles += 14;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = 0x8000;
    c.g[AT][0] = 0xffff_ffff_bf80_0000; // mtc1 f0 = -1.0
    c.f[0] = c.g[AT][0] as u32;
    c.g[V1][0] = u64::MAX;
    c.g[A1][0] = 0xffff_ffff_ffff_8ad0;
    this.off(0xb0).sw(c, 0x8000);
    this.off(0x90).write_fpu(c, FpuF32(c.f[0]));
    c.g[V0][0] = c.g[A0][0];
    this.off(0xb4).sw(c, u64::MAX);
    this.off(0xb8).sw(c, 0xffff_ffff_ffff_8ad0);
    this.sw(c, 0);
    this.off(0x54).sw(c, 0);
    this.off(0x58).sw(c, 0);
    abi::ret(c);
}

/// `float BankMonitor_CalcDopplerPitch(float ratio, float unused)` (0x2abca0):
/// clamp `f12 / f13` to `[gp-0x4560, gp-0x455c]`, then return
/// `(clamped + 8000.0) * gp-0x4558`.
pub fn bank_monitor_calc_doppler_pitch(c: &mut Cpu) {
    c.cycles += 6;
    let ratio = abi::farg(c, 0).div(abi::farg(c, 1));
    let lower = Addr((c.g[GP][0] as u32).wrapping_sub(0x4560)).read_fpu(c);
    let clamped;
    if abi::c_lt_s(c, ratio, lower) {
        clamped = lower;
    } else {
        c.cycles += 5;
        let upper = Addr((c.g[GP][0] as u32).wrapping_sub(0x455c)).read_fpu(c);
        if abi::c_lt_s(c, upper, ratio) {
            clamped = upper;
        } else {
            clamped = ratio;
        }
    }
    c.cycles += 6;
    c.g[AT][0] = 0x45fa_0000;
    let addend = FpuF32(c.g[AT][0] as u32); // mtc1 f0 = 8000.0
    let scale = Addr((c.g[GP][0] as u32).wrapping_sub(0x4558)).read_fpu(c);
    c.f[12] = clamped.0;
    c.f[1] = scale.0;
    c.f[0] = ((clamped + addend) * scale).0;
    abi::ret(c);
}

/// `int StreamBuffer_GetState(StreamBuffer *b)` (0x002aa428): fill a stack
/// status block and return its first word.
pub fn stream_buffer_get_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A1][0] = sx(frame.at(c, 0));
        call_ptr(c, callee::STREAM_BUFFER_GET_STATUS, 0x002a_a438)?;
        let block = frame.at(c, 0);
        c.g[V0][0] = Addr(block).lw(c);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void BankMonitorInstance_dtor(BankMonitorInstance *this, int flags)`
/// (0x2aba18).
pub fn bank_monitor_instance_dtor(c: &mut Cpu) {
    guest(c, |c| destroy_if_flag(c, 0x002a_ba30))
}

// ---------------------------------------------------------------------------
// Stream array / speech / song / music player.

/// `void StreamArray_SetStreamFile(StreamArray* a, int i, const char* file)`
/// (0x2aab98): call the setter of stream slot `i` if it exists.
///
/// Leaves the slot list base in `v0`.
pub fn stream_array_set_stream_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        let i = r32(c, A1) << 2;
        frame.save_ra(c, 0);
        let base = Addr(abi::arg(c, 0)).off(8).lw(c);
        c.g[V0][0] = base;
        c.g[A1][0] = sx(i.wrapping_add(base as u32));
        c.g[A0][0] = Addr(i.wrapping_add(base as u32)).lw(c);
        let empty = c.g[A0][0] == 0;
        frame.restore_ra(c, 0);
        if !empty {
            c.cycles += 3;
            c.g[A1][0] = c.g[A2][0];
            callee::STREAM_BUFFER_SET_FILE.call(c, 0x002a_abc0)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void StreamArray_Stop(StreamArray* a, int i)` (0x2aae40): stop stream slot
/// `i` if it exists. Leaves the slot list base in `v0`.
pub fn stream_array_stop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        let i = r32(c, A1) << 2;
        frame.save_ra(c, 0);
        let base = Addr(abi::arg(c, 0)).off(8).lw(c);
        c.g[V0][0] = base;
        c.g[A1][0] = sx(i.wrapping_add(base as u32));
        c.g[A0][0] = Addr(i.wrapping_add(base as u32)).lw(c);
        let empty = c.g[A0][0] == 0;
        frame.restore_ra(c, 0);
        if !empty {
            c.cycles += 3;
            callee::STREAM_BUFFER_STOP.call(c, 0x002a_ae68)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void BankMonitorInstance_Resume(void* self)` (0x2acb30): resume a paused
/// bank monitor instance and restart its MIDI slot.
pub fn bank_monitor_instance_resume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        callee::BANK_MONITOR_INSTANCE_POLL_FREE.call(c, 0x002a_cb44)?;
        if c.g[V0][0] == 0 {
            c.cycles += 7;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x1000;
            callee::BANK_MONITOR_INSTANCE_SET_PITCH.call(c, 0x002a_cb58)?;
            c.g[V1][0] = Addr(this).lw(c);
            c.g[V0][0] = 0x2;
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 3;
                c.g[A0][0] = Addr(this).off(4).lw(c);
                call_ptr(c, table::SND_MIDI_RESUME, 0x002a_cb70)?;
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void StreamArray_DestroyStream(self, i)` (0x2aabd0): destroy and clear
/// entry `i` of the stream array.
pub fn stream_array_destroy_stream(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = sx((c.g[A1][0] as u32) << 2);
        c.g[V0][0] = Addr(r32(c, S0)).off(8).lw(c);
        c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
        c.g[A0][0] = c.lw(r32(c, V0));
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[RA][0] = sx(0x002a_ac04);
            c.g[A1][0] = 3;
            callee::STREAM_BUFFER_DTOR.call(c, 0x002a_ac04)?;
            c.g[V0][0] = Addr(r32(c, S0)).off(8).lw(c);
        } else {
            c.g[V0][0] = Addr(r32(c, S0)).off(8).lw(c);
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = sx(r32(c, S1).wrapping_add(r32(c, V0)));
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        Addr(r32(c, V0)).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SoundManagerRef* SoundManagerRef_ctor(SoundManagerRef* self, void* a1)`
/// (0x2ade88): install the reference vtable, create the sound manager and
/// pass the argument on.
pub fn sound_manager_ref_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0048_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3c00));
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        Addr(p(c.g[S0][0], 4)).sw(c, c.g[V0][0]);
        Addr(p(c.g[GP][0], 0x68c)).sw(c, c.g[S0][0]);
        call_ptr(c, 0x002a_8fa0, 0x002a_deb4)?;
        Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
        c.g[A1][0] = c.g[S1][0];
        c.g[V1][0] = lw_2(c, c.g[V0][0] as u32);
        c.g[A0][0] = Addr(p(c.g[V1][0], 8)).lh(c);
        c.g[A2][0] = lw_2(c, p(c.g[V1][0], 0xc));
        let target = c.g[A2][0] as u32;
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002a_ded0)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Audio / streaming.

/// `void StreamBuffer_SetFile(StreamBuffer* buf, const char* name)`
/// (0x2a9e78): look the name up and register it on the buffer.
pub fn stream_buffer_set_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save(c, S0, 0x30);
        let buf = c.g[A0][0];
        let name = c.g[A1][0];
        c.g[S1][0] = buf;
        c.g[S2][0] = name;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0031_7f38, 0x002a_9e9c)?;
        let missing = c.g[V0][0] == 0;
        c.g[S0][0] = sx(at(c, S1, 0x14));
        if !missing {
            c.cycles += 7;
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0041_66f4, 0x002a_9eb0)?;
            c.g[A1][0] = Addr(at(c, S1, 0x7c)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x003e_1ec8, 0x002a_9ebc)?;
            Addr(at(c, S1, 0x74)).sw(c, c.g[V0][0]);
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void StreamBuffer_SetBaseVolume(StreamBuffer* b, u8 volume)` (0x2a9f30):
/// store the volume byte at +0xc.
pub fn stream_buffer_set_base_volume(c: &mut Cpu) {
    c.cycles += 2;
    let b = abi::arg(c, 0);
    let v = c.g[A1][0];
    Addr(b.wrapping_add(0xc)).sb(c, v);
    abi::ret(c);
}

/// `void SoundRequest_SetBankSample(SoundRequest* r, int bank, int sample)`
/// (0x2abce8): store the bank at +0xc and the sample at +0x10.
pub fn sound_request_set_bank_sample(c: &mut Cpu) {
    c.cycles += 3;
    let r = abi::arg(c, 0);
    let sample = c.g[A2][0];
    Addr(r.wrapping_add(0x10)).sw(c, sample);
    let bank = c.g[A1][0];
    Addr(r.wrapping_add(0xc)).sw(c, bank);
    abi::ret(c);
}

/// `SoundManagerRef_dtor(self, flags)` (0x2adee8): destroy the referenced
/// sound manager and free the reference.
pub fn sound_manager_ref_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x480000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3c00));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        Addr(p_2(c.g[S1][0] as u32, 4)).sw(c, c.g[V0][0]);
        c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
        c.g[A0][0] = Addr(p_2(c.g[V0][0] as u32, 0x18)).lh(c);
        c.g[V1][0] = Addr(p_2(c.g[V0][0] as u32, 0x1c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x002a_df24)?;
        c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
        call_ptr(c, 0x0031_7e50, 0x002a_df2c)?;
        c.g[S0][0] = c.g[S0][0] & 1;
        let no_delete = c.g[S0][0] == 0;
        Addr(gp(c, 0x68c)).sw(c, 0);
        if !no_delete {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x002a_df40)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* StreamArray_CreateStream(self, idx, heap, extra)` (0x002aa7f0):
/// allocate a stream buffer and store it in slot `idx`.
pub fn stream_array_create_stream(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = sx(word(c, A1) << 2);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A3][0];
        c.g[A1][0] = sx(0x480000);
        c.g[V0][0] = Addr(at_2(c, S1, 0x8)).lw(c);
        c.g[A1][0] = sx(0x480000u32.wrapping_add(0x3228));
        c.g[A0][0] = 0x84;
        c.g[A3][0] = 0;
        c.g[S0][0] = sx(word(c, S0).wrapping_add(word(c, V0)));
        callee::MEM_ALLOC_NAMED.call(c, 0x002a_a834)?;
        c.g[A1][0] = Addr(at_2(c, S1, 0)).lw(c);
        c.g[A2][0] = c.g[S2][0];
        c.g[A3][0] = c.g[S3][0];
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = sx(word(c, A1).wrapping_add(0x1d8));
        callee::STREAM_BUFFER_CTOR.call(c, 0x002a_a84c)?;
        Addr(at_2(c, S0, 0)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void StreamBuffer_dtor(self, flags)` (0x2a9df0): close the stream, free
/// its buffer and the buffer object, then the object itself when
/// `flags & 1`.
pub fn streambuffer_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_at(c, c.g[S0][0] as u32);
        let no_stream = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_stream {
            c.cycles += 5;
            c.g[A0][0] = lw_at(c, p_3(r32(c, S0), 8));
            call_ptr(c, callee::SNDSTRM_CLOSE.addr, 0x002a_9e18)?;
            c.g[V1][0] = lw_at(c, p_3(r32(c, S0), 4));
            if c.g[V1][0] != 0 {
                c.g[A0][0] = lw_at(c, p_3(r32(c, S0), 0x74));
            } else {
                c.cycles += 3;
                c.g[A0][0] = lw_at(c, c.g[S0][0] as u32);
                if c.g[A0][0] != 0 {
                    c.cycles += 2;
                    callee::EA_MEM_FREE.call(c, 0x002a_9e38)?;
                    c.cycles += 1;
                }
                c.g[A0][0] = lw_at(c, p_3(r32(c, S0), 0x74));
            }
        } else {
            c.cycles += 1;
            c.g[A0][0] = lw_at(c, p_3(r32(c, S0), 0x74));
        }
        c.cycles += 2;
        let none = c.g[A0][0] == 0;
        c.g[V0][0] = c.g[S1][0] & 1;
        if !none {
            c.cycles += 3;
            call_ptr(c, callee::MEM_FREE_3, 0x002a_9e4c)?;
            c.g[V0][0] = c.g[S1][0] & 1;
        }
        c.cycles += 2;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::BUILTIN_DELETE_4.addr, 0x002a_9e60)?;
        }
        frame.restore(c, 16, 0x20);
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod bank_range {
    use super::*;

    /// `BankMonitorInstance`: playing flag, audible range (0: always on).
    pub const PLAYING: u32 = 0x8;

    pub const RANGE: u32 = 0x8c;

    pub const BANK_MONITOR_INSTANCE_FADE_OUT: Callee = Callee {
        addr: 0x002a_c180,
        f: ee::s08::f_002ac180,
    };

    pub const BANK_MONITOR_INSTANCE_PLAY: Callee = Callee {
        addr: 0x002a_c220,
        f: ee::s08::f_002ac220,
    };
}

/// `bool BankMonitorInstance_UpdateRange(BankMonitorInstance* this, float
/// dist)` (0x2ac7f0): starts or fades a positional sound by distance.
/// Without a range: 1. Playing: beyond the range it fades out (0), else 1.
/// Stopped: inside the range it starts playing (1), else 0.
pub fn bank_monitor_instance_update_range(c: &mut Cpu) {
    use bank_range::*;
    guest(c, |c| {
        let frame = Frame::open(c, 0x10);
        c.cycles += 8;
        frame.save_ra(c, 0);
        let this = Addr(abi::arg(c, 0));
        let dist = abi::farg(c, 0);
        let range = this.off(RANGE).read_fpu(c);
        abi::set_fpr(c, 0, FpuF32::ZERO);
        abi::set_fpr(c, 1, range);
        let no_range = range.eq_s(FpuF32::ZERO);
        abi::set_fcc(c, no_range);
        c.g[V0][0] = 0x1;
        if !no_range {
            c.cycles += 3;
            c.g[V0][0] = this.off(PLAYING).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                let beyond = abi::c_lt_s(c, range, dist);
                c.g[V0][0] = 0x1;
                if beyond {
                    c.cycles += 4;
                    BANK_MONITOR_INSTANCE_FADE_OUT.call(c, 0x002a_c834)?;
                    c.g[V0][0] = 0;
                }
            } else {
                c.cycles += 4;
                let inside = abi::c_lt_s(c, dist, range);
                c.g[V0][0] = 0;
                if inside {
                    c.cycles += 3;
                    BANK_MONITOR_INSTANCE_PLAY.call(c, 0x002a_c854)?;
                    c.g[V0][0] = 0x1;
                }
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamBuffer_GetStatus (0x2aa020, streamsys.cpp)

pub(crate) const SND_LOCK: u32 = 0x003b_58a0;

pub(crate) const SND_UNLOCK: u32 = 0x003b_58d8;

pub(crate) const SNDSTRM_GET_STATUS: u32 = 0x003b_7b48;

/// StreamBuffer field: the SND stream id.
pub(crate) const SB_STREAM: u32 = 0x8;

/// `int StreamBuffer_GetStatus(StreamBuffer* self, SndStreamStatus* out)`
/// (0x2aa020): `SNDSTRM_GetStatus(self->stream, out)` under the sound
/// library lock; returns its result.
pub fn stream_buffer_get_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        let this = r32(c, S0);
        call_ptr(c, SND_LOCK, 0x002a_a03c)?;
        c.g[A0][0] = Addr(this.wrapping_add(SB_STREAM)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, SNDSTRM_GET_STATUS, 0x002a_a048)?;
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, SND_UNLOCK, 0x002a_a050)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamArray_UpdateAll (0x2aac28, streamsys.cpp)

/// StreamArray fields: buffer count, buffer pointer array.
pub(crate) const SA_COUNT: u32 = 0x4;

pub(crate) const SA_BUFFERS: u32 = 0x8;

pub(crate) const STREAM_BUFFER_UPDATE_VOLUME: Callee = Callee {
    addr: 0x002a_9f80,
    f: ee::s08::StreamBuffer_UpdateVolume,
};

/// `void StreamArray_UpdateAll(StreamArray* self)` (0x2aac28): updates the
/// volume of every non-null stream buffer (`StreamBuffer_UpdateVolume`).
pub fn stream_array_update_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(SA_COUNT)).lw(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S0][0] = 0;
        if !empty {
            c.cycles += 2;
            c.g[V1][0] = Addr(r32(c, S1).wrapping_add(SA_BUFFERS)).lw(c);
            loop {
                c.cycles += 5;
                c.g[V0][0] = sx(r32(c, S0) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                if c.g[A0][0] != 0 {
                    c.cycles += 3;
                    STREAM_BUFFER_UPDATE_VOLUME.call(c, 0x002a_ac6c)?;
                }
                c.g[V0][0] = Addr(r32(c, S1).wrapping_add(SA_COUNT)).lw(c);
                c.cycles += 4;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.g[V1][0] = Addr(r32(c, S1).wrapping_add(SA_BUFFERS)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x002a_ac50);
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamBuffer_UpdateVolume (0x2a9f80)

/// StreamBuffer fields: the SNDSTRM stream handle, the volume byte, the
/// pointer to the volume scale (a float) and the starvation counter.
pub(crate) const SB_HANDLE: u32 = 0x8;

pub(crate) const SB_VOLUME: u32 = 0xc;

pub(crate) const SB_SCALE_PTR: u32 = 0x10;

pub(crate) const SB_STARVED: u32 = 0x80;

/// `StreamBuffer_Stop` (0x2aa408).
pub(crate) const STREAM_BUFFER_STOP: Callee = Callee {
    addr: 0x002a_a408,
    f: ee::s08::StreamBuffer_Stop,
};

/// `void StreamBuffer_UpdateVolume(StreamBuffer *sb)` (0x2a9f80): stops the
/// stream when its starvation counter (+0x80) has passed two seconds of
/// audio frames (and clears it); otherwise, with a volume scale (+0x10),
/// sets the stream's volume to `volume byte * scale` clamped to 0..127,
/// under the sound lock.
pub fn stream_buffer_update_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        call_ptr(c, 0x0028_b240, 0x002a_9f98)?; // Audio_GetFrameRate

        let sb = Addr(r32(c, S1));
        let starved = sb.off(SB_STARVED).lw(c);
        c.g[V1][0] = starved;
        let limit = sx(r32(c, V0) << 1);
        c.g[V0][0] = ((limit as i64) < (starved as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = sx(sb.0);
            STREAM_BUFFER_STOP.call(c, 0x002a_9fb4)?;
            Addr(r32(c, S1)).off(SB_STARVED).sw(c, 0);
        } else {
            let scale_ptr = sb.off(SB_SCALE_PTR).lw(c);
            c.g[V1][0] = scale_ptr;
            c.cycles += 2;
            c.g[A0][0] = 0x7f;
            if scale_ptr != 0 {
                c.cycles += 18;
                let volume = sb.off(SB_VOLUME).lb(c);
                c.g[V0][0] = volume;
                let scale = Addr(scale_ptr as u32).read_fpu(c);
                c.f[1] = scale.0;
                let vol_f = FpuF32::from_i32(volume as i32);
                let prod = vol_f * scale;
                c.f[0] = prod.0;
                let mut v = prod.to_i32();
                c.f[1] = v as u32;
                let below = v < 128;
                c.g[V0][0] = below as u64;
                if !below {
                    v = 0x7f;
                }
                let nonneg = -1 < v;
                c.g[V1][0] = nonneg as u64;
                if !nonneg {
                    v = 0;
                }
                c.g[S0][0] = sx(v as u32);
                call_ptr(c, 0x003b_58a0, 0x002a_9ff8)?; // SND_Lock
                c.g[A0][0] = Addr(r32(c, S1)).off(SB_HANDLE).lw(c);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003b_7e70, 0x002a_a004)?; // SNDSTRM_SetVolume
                call_ptr(c, 0x003b_58d8, 0x002a_a00c)?; // SND_Unlock
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamBuffer_GetRequestStatus (0x2aa068, streamsys.cpp)

/// `SND_Lock` / `SND_Unlock` (sound library lock, through the table).
pub(crate) const SBRS_SND_LOCK: u32 = 0x003b_58a0;

pub(crate) const SBRS_SND_UNLOCK: u32 = 0x003b_58d8;

/// `SNDSTRM_GetRequestStatus(int request, SndRequestStatus* out)` (another shard).
pub(crate) const SBRS_SNDSTRM_GET_REQUEST_STATUS: u32 = 0x003b_7c40;

/// `StreamBuffer_GetStatus(self, SndStreamStatus* out)`.
pub(crate) const SBRS_STREAM_BUFFER_GET_STATUS: Callee = Callee {
    addr: 0x002a_a020,
    f: ee::s08::StreamBuffer_GetStatus,
};

/// SndStreamStatus field: the id of the stream's current request.
pub(crate) const SBRS_STATUS_REQUEST: u32 = 0x4;

/// `int StreamBuffer_GetRequestStatus(StreamBuffer* self, SndRequestStatus*
/// out, SndStreamStatus* scratch)` (0x2aa068): reads the stream status into
/// `scratch` (or a frame-local buffer when null), then, under the sound
/// library lock, `SNDSTRM_GetRequestStatus(status.request, out)`; returns its
/// result.
pub fn stream_buffer_get_request_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        // movn: the caller's status buffer, else the frame's own.
        c.g[S0][0] = if c.g[A2][0] != 0 {
            c.g[A2][0]
        } else {
            c.g[SP][0]
        };
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[S0][0];
        SBRS_STREAM_BUFFER_GET_STATUS.call(c, 0x002a_a08c)?;
        call_ptr(c, SBRS_SND_LOCK, 0x002a_a094)?;
        let status = r32(c, S0);
        c.g[A0][0] = Addr(status.wrapping_add(SBRS_STATUS_REQUEST)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, SBRS_SNDSTRM_GET_REQUEST_STATUS, 0x002a_a0a0)?;
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, SBRS_SND_UNLOCK, 0x002a_a0a8)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x20);
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamArray_GetState (0x2aae08, streamsys.cpp)

/// StreamArray: pointer to its array of StreamBuffer pointers.
pub(crate) const OC264_SA_BUFFERS: u32 = 0x8;

pub(crate) const OC264_STREAM_BUFFER_GET_STATE: Callee = Callee {
    addr: 0x002a_a428,
    f: ee::s08::StreamBuffer_GetState,
};

/// `int StreamArray_GetState(StreamArray* self, int i)` (0x2aae08): the
/// play state of stream buffer `i` (`StreamBuffer_GetState`), or 0 when
/// that slot is empty.
pub fn stream_array_get_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        let slot_off = r32(c, A1) << 2;
        c.g[A1][0] = sx(slot_off);
        frame.save_ra(c, 0);
        let buffers = Addr(r32(c, A0).wrapping_add(OC264_SA_BUFFERS)).lw(c);
        c.g[V0][0] = buffers;
        let slot = slot_off.wrapping_add(buffers as u32);
        c.g[A1][0] = sx(slot);
        c.g[A0][0] = Addr(slot).lw(c);
        let empty = c.g[A0][0] == 0;
        c.g[V0][0] = 0;
        if !empty {
            c.cycles += 2;
            OC264_STREAM_BUFFER_GET_STATE.call(c, 0x002a_ae30)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamBuffer_IsPlaying (0x2aa1b8)
// ---------------------------------------------------------------------------

/// SndRequestStatus: the request's state (2 = playing).
pub(crate) const SBIP_REQUEST_STATE_PLAYING: u64 = 2;

/// `bool StreamBuffer_IsPlaying(StreamBuffer* self)` (0x2aa1b8): whether the
/// stream's current request is playing (`StreamBuffer_GetRequestStatus` into
/// a frame-local status, state == 2).
pub fn stream_buffer_is_playing(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        Callee {
            addr: 0x002a_a068,
            f: ee::s08::f_002aa068,
        }
        .call(c, 0x002a_a1cc)?;
        let state = Addr(sp(c)).lw(c);
        frame.restore_ra(c, 0x10);
        c.g[V0][0] = (state == SBIP_REQUEST_STATE_PLAYING) as u64;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BankMonitorInstance_Reset

/// `BankMonitorInstance` fields cleared by `Reset`.
pub(crate) const BMI_RESET_KIND: u32 = 0x0;

pub(crate) const BMI_RESET_FADING: u32 = 0x54;

pub(crate) const BMI_RESET_TIMER: u32 = 0x58;

pub(crate) const BMI_RESET_PRIORITY: u32 = 0xb0;

pub(crate) const BMI_RESET_KEY: u32 = 0xb4;

pub(crate) const BMI_RESET_ID: u32 = 0xb8;

/// `void BankMonitorInstance_Reset(BankMonitorInstance* self)` (0x2abb38):
/// fade the instance out, then free it: kind 0, not fading, no timer,
/// priority 0x8000, key -1, id 0xffff.
pub fn bank_monitor_instance_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Callee {
            addr: 0x002a_c180,
            f: ee::s08::BankMonitorInstance_FadeOut,
        }
        .call(c, 0x002a_bb4c)?;
        let inst = Addr(c.g[S0][0] as u32);
        c.g[V1][0] = 0x8000;
        c.g[A0][0] = u64::MAX;
        c.g[V0][0] = 0xffff;
        inst.off(BMI_RESET_PRIORITY).sw(c, c.g[V1][0]);
        inst.off(BMI_RESET_ID).sw(c, c.g[V0][0]);
        inst.off(BMI_RESET_KEY).sw(c, c.g[A0][0]);
        inst.off(BMI_RESET_KIND).sw(c, 0);
        inst.off(BMI_RESET_FADING).sw(c, 0);
        inst.off(BMI_RESET_TIMER).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BankMonitorInstance_FadeOut (0x2ac180, streamsys.cpp)

/// Bank monitor instance fields: kind (+0; 1 = patch voice, 2 = stream), the
/// sound handle (+4), the playing flag (+8) and the fading flag (+0x54).
pub(crate) const BMI_KIND: u32 = 0x0;

pub(crate) const BMI_HANDLE: u32 = 0x4;

pub(crate) const BMI_PLAYING: u32 = 0x8;

pub(crate) const BMI_FADING: u32 = 0x54;

/// Fade length and target volume passed to the fade calls.
pub(crate) const BMI_FADE_TICKS: u64 = 0x19;

/// `SND_Lock` / `SND_Unlock` and the two fade functions (cross-shard
/// direct calls, emitted as table calls).
pub(crate) const BMI_SND_LOCK: u32 = 0x003b_58a0;

pub(crate) const BMI_SND_UNLOCK: u32 = 0x003b_58d8;

pub(crate) const BMI_FADE_VOICE: u32 = 0x003b_8530;

pub(crate) const BMI_FADE_STREAM: u32 = 0x003b_9f38;

/// `void BankMonitorInstance_FadeOut(BankMonitorInstance* self)`
/// (0x2ac180): once only (the fading flag), clears the playing flag and
/// fades the instance's voice (kind 1, `SND_FadeVolume`) or stream (kind 2)
/// to silence over 25 ticks, under the sound lock. Frame 0x20 (ra at 0, s0
/// at 0x10).
pub fn bank_monitor_instance_fade_out(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let inst = r32(c, S0);
        c.g[V0][0] = c.lw(inst.wrapping_add(BMI_FADING));
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            c.g[V0][0] = 1;
            c.g[V1][0] = c.lw(inst.wrapping_add(BMI_KIND));
            c.sw(inst.wrapping_add(BMI_PLAYING), 0);
            c.sw(inst.wrapping_add(BMI_FADING), 1);
            // (fade function, its three return addresses)
            let fade = if c.g[V1][0] == 1 {
                c.cycles += 10;
                Some((BMI_FADE_VOICE, [0x002a_c1cc, 0x002a_c1dc, 0x002a_c1e4]))
            } else {
                c.cycles += 3;
                c.g[V0][0] = 2;
                if c.g[V1][0] == 2 {
                    c.cycles += 9;
                    Some((BMI_FADE_STREAM, [0x002a_c1f4, 0x002a_c204, 0x002a_c20c]))
                } else {
                    c.cycles += 2;
                    None
                }
            };
            if let Some((target, rets)) = fade {
                call_ptr(c, BMI_SND_LOCK, rets[0])?;
                c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(BMI_HANDLE));
                c.g[A1][0] = BMI_FADE_TICKS;
                c.g[A2][0] = u64::MAX;
                call_ptr(c, target, rets[1])?;
                call_ptr(c, BMI_SND_UNLOCK, rets[2])?;
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
// StreamArray_GetTimeRemaining (0x2aae78, sound/streamsys.cpp)

/// StreamArray: pointer to its array of StreamBuffer pointers.
pub(crate) const SAT_BUFFERS: u32 = 0x8;

/// `StreamBuffer_GetTimeRemaining` (0x2aa108).
pub(crate) const SAT_STREAM_BUFFER_GET_TIME_REMAINING: Callee = Callee {
    addr: 0x002a_a108,
    f: ee::s08::StreamBuffer_GetTimeRemaining,
};

/// `float StreamArray_GetTimeRemaining(StreamArray* self, int i)` (0x2aae78):
/// the time left in stream buffer `i` (`StreamBuffer_GetTimeRemaining`), or
/// 0.0 when `i` is negative or that slot is empty.
pub fn stream_array_get_time_remaining(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let mut present = false;
        if (c.g[A1][0] as i64) >= 0 {
            c.cycles += 6;
            let buffers = Addr(r32(c, A0).wrapping_add(SAT_BUFFERS)).lw(c);
            c.g[V1][0] = buffers;
            let slot = (r32(c, A1) << 2).wrapping_add(buffers as u32);
            c.g[V0][0] = sx(slot);
            c.g[A0][0] = Addr(slot).lw(c);
            present = c.g[A0][0] != 0;
        }
        if present {
            c.cycles += 3;
            SAT_STREAM_BUFFER_GET_TIME_REMAINING.call(c, 0x002a_aeb0)?;
        } else {
            c.cycles += 3;
            c.f[0] = 0;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamBuffer_GetTimeRemaining (0x2aa108)
// ---------------------------------------------------------------------------

/// `StreamBuffer_GetRequestStatus(self, status, scratch)`.
pub(crate) const STREAM_BUFFER_GET_REQUEST_STATUS: Callee = Callee {
    addr: 0x002a_a068,
    f: ee::s08::StreamBuffer_GetRequestStatus,
};

/// Request status record (on the stack): state (2 = playing) and the
/// remaining time at +8.
pub(crate) const SB_REQ_STATE: u32 = 0x0;

pub(crate) const SB_REQ_REMAINING: u32 = 0x8;

pub(crate) const SB_REQ_PLAYING: u64 = 2;

/// `float StreamBuffer_GetTimeRemaining(StreamBuffer* self)` (0x2aa108):
/// asks for the current request's status (into a stack record); while it
/// is playing (state 2) returns its remaining time as a float, else 0.
pub fn stream_buffer_get_time_remaining(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        STREAM_BUFFER_GET_REQUEST_STATUS.call(c, 0x002a_a11c)?;
        let state = Addr(frame.at(c, SB_REQ_STATE)).lw(c);
        c.g[V1][0] = state;
        c.g[V0][0] = SB_REQ_PLAYING;
        frame.restore_ra(c, 0x10);
        if state == SB_REQ_PLAYING {
            c.cycles += 3;
            let remaining = Addr(frame.at(c, SB_REQ_REMAINING)).lw(c);
            c.g[V0][0] = remaining;
            abi::set_fpr(c, 0, FpuF32::from_i32(remaining as i32));
        } else {
            c.cycles += 3;
            abi::set_fpr(c, 0, FpuF32::ZERO);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// StreamArray_IsPlaying.

/// `StreamArray::buffers`: the array of `StreamBuffer*` slots.
pub(crate) const STREAM_ARRAY_BUFFERS: u32 = 0x8;

/// `bool StreamArray_IsPlaying(StreamArray* self, int i)` (0x2ab028): whether
/// stream slot `i` holds a buffer that is playing (`StreamBuffer_IsPlaying`);
/// false for an empty slot.
pub fn stream_array_is_playing(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 2);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(STREAM_ARRAY_BUFFERS)).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = Addr(c.g[A1][0] as u32).lw(c);
        let empty = c.g[A0][0] == 0;
        c.g[V0][0] = 0;
        if !empty {
            c.cycles += 2;
            Callee {
                addr: 0x002a_a1b8,
                f: ee::s08::f_002aa1b8,
            }
            .call(c, 0x002a_b050)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BankMonitorInstance_UpdateDistance (0x2abd48)

/// Listener object -> its position-source vtable (slot 0x30/0x34: distance²).
pub(crate) const LISTENER_VTABLE: u32 = 0x8d8;

/// BankMonitorInstance -> distance to the listener.
pub(crate) const BANK_MONITOR_DISTANCE: u32 = 0x5c;

/// `void BankMonitorInstance::UpdateDistance(Listener* l)` (0x2abd48): asks
/// the listener's virtual (vtable slot 0x30/0x34, `this` adjusted by the
/// slot's delta) for the squared distance to this instance (with a 0x20-byte
/// scratch buffer on the stack) and stores its square root at `self+0x5c`.
pub fn bank_monitor_instance_update_distance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x40);
        c.g[A3][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        c.g[A2][0] = c.g[SP][0];
        frame.save_ra(c, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = c.g[S0][0];
        let listener = c.g[A3][0] as u32;
        c.g[V1][0] = Addr(listener.wrapping_add(LISTENER_VTABLE)).lw(c);
        let vt = Addr(c.g[V1][0] as u32);
        c.g[A0][0] = vt.off(0x30).lh(c);
        c.g[V0][0] = vt.off(0x34).lw(c);
        let method = c.g[V0][0] as u32;
        c.g[A0][0] = sx(listener.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, method, 0x002a_bd78)?;
        let dist = crate::ps2f::FpuF32(c.f[0]).sqrt();
        c.f[0] = dist.0;
        frame.restore_ra(c, 0x20);
        let this = c.g[S0][0] as u32;
        Addr(this.wrapping_add(BANK_MONITOR_DISTANCE)).write_fpu(c, dist);
        frame.restore(c, S0, 0x30);
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
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
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
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::SP as STACK_TOP;
    use crate::difftest::SP as STACK;
    pub(crate) use crate::game::ai::ai::tests::leaf;
    pub(crate) use crate::game::ai::ai::tests::leaf_setup;
    pub(crate) use crate::game::ai::ai::tests::rng;
    pub(crate) use crate::game::bx::bxstring2::tests::game;
    pub(crate) use crate::game::bx::bxstring2::tests::game_globals;
    pub(crate) use crate::game::bx::bxstring2::tests::SCRATCH;
    pub(crate) use crate::game::dirtysock::tags2::tests::common;
    pub(crate) use crate::game::dirtysock::tags2::tests::dummy_world;
    pub(crate) use crate::game::dirtysock::tags2::tests::gp_at;
    pub(crate) use crate::game::dirtysock::tags2::tests::heap_block_2 as heap_block;
    pub(crate) use crate::game::dirtysock::tags2::tests::heap_init_6 as heap_init;
    pub(crate) use crate::game::dirtysock::tags2::tests::run;
    pub(crate) use crate::game::dirtysock::tags2::tests::ARENA;
    pub(crate) use crate::game::dirtysock::tags2::tests::ARENA_SIZE;
    pub(crate) use crate::game::dirtysock::tags2::tests::MEM_HEAP;
    pub(crate) use crate::game::dirtysock::tags2::tests::SEEDS_7 as SEEDS_2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_5 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_2 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_5 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2 as w;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_2 as WORLD;
    pub(crate) use crate::game::lib::sif::tests::SEEDS as SEEDS_3;
    pub(crate) use crate::game::main::gamestate::tests::base_3 as base_4;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_4;
    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3;
    pub(crate) use crate::game::movie::movieplayer::tests::heap_init_2;
    pub(crate) use crate::game::movie::movieplayer::tests::pool_allocator as pool_allocator_2;
    pub(crate) use crate::game::movie::movieplayer::tests::run_2;
    pub(crate) use crate::game::movie::movieplayer::tests::w as w_3;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_2;
    pub(crate) use crate::game::movie::movieplayer::tests::W;
    pub(crate) use crate::game::replay::replay2::tests::dtor_setup;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::load;
    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn stream_array_set_volume_scale_ptr_matches() {
        check(
            ssx3_ee::s08::f_002ab0d0__generated,
            stream_array_set_volume_scale_ptr,
            300,
            &|c, seed| {
                let mut r = Rng(0x9abc_def0 ^ seed as u64);
                scramble(c, &mut r);
                let table = DATA + 0x100;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (r.next_u32() % 4) as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.bus.write32(DATA + STREAM_ARRAY_TABLE, table);
                fill(c, &mut r, table, 8);
                if seed % 2 == 0 {
                    for i in 0..4 {
                        c.bus.write32(table + i * 4, 0);
                    }
                } else {
                    for i in 0..4 {
                        c.bus.write32(table + i * 4, DATA + 0x300 + i * 0x20);
                    }
                }
                fill(c, &mut r, DATA + 0x300, 0x100);
            },
        );
    }

    #[test]
    fn stream_array_get_stream_matches() {
        check(
            ssx3_ee::s08::f_002ab188__generated,
            stream_array_get_stream,
            300,
            &|c, seed| {
                let mut r = Rng(0xabcd_ef01 ^ seed as u64);
                scramble(c, &mut r);
                let table = DATA + 0x100;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (r.next_u32() % 4) as u64;
                c.bus.write32(DATA + STREAM_ARRAY_TABLE, table);
                for i in 0..4 {
                    c.bus
                        .write32(table + i * 4, if seed % 3 == 0 { 0 } else { r.next_u32() });
                }
            },
        );
    }

    #[test]
    fn stream_array_get_stream_handle_matches() {
        check(
            ssx3_ee::s08::f_002ab128,
            stream_array_get_stream_handle,
            300,
            &|c, seed| {
                let mut r = Rng(0x1111_0010 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let idx = (seed % 8) as u64;
                c.g[A1][0] = idx;
                let base = DATA + 0x100;
                c.bus.write32(DATA + 8, base);
                let entry = if seed % 3 == 0 { 0 } else { DATA + 0x200 };
                c.bus.write32(base + (idx as u32) * 4, entry);
                c.bus.write32(DATA + 0x200 + 8, r.next_u32());
            },
        );
    }

    #[test]
    fn bank_monitor_instance_ctor_matches() {
        check(
            ssx3_ee::s08::f_002ab9e0,
            bank_monitor_instance_ctor,
            50,
            &|c, seed| {
                let mut r = Rng(0x1111_0011 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..48 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn bank_monitor_calc_doppler_pitch_matches() {
        check(
            ssx3_ee::s08::f_002abca0,
            bank_monitor_calc_doppler_pitch,
            300,
            &|c, seed| {
                let mut r = Rng(0x1111_0012 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(GAME_GP - 0x4560, (-1.0f32).to_bits()); // lower clamp
                c.bus.write32(GAME_GP - 0x455c, (1.0f32).to_bits()); // upper clamp
                c.bus.write32(GAME_GP - 0x4558, r.edge_f32()); // scale
                match seed % 4 {
                    0 => {
                        c.f[12] = (-2.0f32).to_bits(); // ratio < lower
                        c.f[13] = 1.0f32.to_bits();
                    }
                    1 => {
                        c.f[12] = 2.0f32.to_bits(); // ratio > upper
                        c.f[13] = 1.0f32.to_bits();
                    }
                    2 => {
                        c.f[12] = 0.5f32.to_bits(); // inside
                        c.f[13] = 1.0f32.to_bits();
                    }
                    _ => {
                        c.f[12] = r.edge_f32();
                        c.f[13] = r.edge_f32();
                    }
                }
            },
        );
    }

    #[test]
    fn stream_buffer_get_state_matches() {
        check(
            ee::s08::f_002aa428__generated,
            stream_buffer_get_state,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_a428, 0);
                c.g[A0][0] = (DATA + 0x1000) as u64;
                for i in 0..8u32 {
                    c.bus.write32(DATA + 0x1000 + 4 * i, r.next_u32() % 4);
                }
            },
        );
    }

    #[test]
    fn bank_monitor_instance_dtor_matches() {
        check(
            ee::s08::f_002aba18__generated,
            bank_monitor_instance_dtor,
            60,
            &|c, seed| dtor_setup(c, seed, 0x2aba18),
        );
    }

    #[test]
    fn stream_array_set_stream_file_matches() {
        check(
            ee::s08::f_002aab98__generated,
            stream_array_set_stream_file,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2aab_98, 0);
                let arr = DATA + 0x100;
                c.g[A0][0] = sx(arr);
                c.g[A1][0] = sx(seed % 16);
                c.g[A2][0] = sx(DATA + 0x300);
                // The slot list lives at arr + 8; the slots are arr + 0x200 + i*4.
                c.bus.write32(arr + 8, DATA + 0x200);
                let slot = DATA + 0x200 + 4 * (seed % 16);
                let entry = if seed % 3 == 0 { 0 } else { DATA + 0x320 };
                c.bus.write32(slot, entry);
                fill(c, &mut r, DATA + 0x300, 8);
            },
        );
    }

    #[test]
    fn stream_array_stop_matches() {
        check(
            ee::s08::f_002aae40__generated,
            stream_array_stop,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2aae_40, 0);
                let arr = DATA + 0x100;
                c.g[A0][0] = sx(arr);
                c.g[A1][0] = sx(seed % 16);
                c.bus.write32(arr + 8, DATA + 0x200);
                let slot = DATA + 0x200 + 4 * (seed % 16);
                let entry = if seed % 3 == 0 { 0 } else { DATA + 0x340 };
                c.bus.write32(slot, entry);
                fill(c, &mut r, DATA + 0x340, 8);
            },
        );
    }

    #[test]
    fn bank_monitor_instance_resume_matches() {
        check(
            ee::s08::f_002acb30__generated,
            bank_monitor_instance_resume,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2acb_30, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x120);
                c.g[A0][0] = sx(this);
                // `*(self)` non-zero takes the "resume" path in PollFree.
                c.bus.write32(this, (seed % 3) as u32);
                c.bus.write32(this + 0x9c, 0);
                c.bus.write32(
                    this + 4,
                    match seed % 4 {
                        0 => u32::MAX,
                        n => n,
                    },
                );
            },
        );
    }

    #[test]
    fn stream_array_destroy_stream_matches() {
        check(
            ee::s08::f_002aabd0__generated,
            stream_array_destroy_stream,
            SEEDS_2,
            &|c, seed| {
                let _ = common(c, seed, 0x2aabd0, 1);
                alloc_ready(c);
                let this = DATA + 0x100;
                let array = DATA + 0x2000;
                clear(c, this, 0x40);
                clear(c, array, 0x100);
                c.bus.write32(this + 8, array);
                let i = seed % 4;
                let buf = if seed % 3 == 0 {
                    0
                } else {
                    heap_block(c, 0x100)
                };
                if buf != 0 {
                    clear(c, buf, 0x40);
                }
                c.bus.write32(array + 4 * i, buf);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = i as u64;
            },
        );
    }

    #[test]
    fn sound_manager_ref_ctor_matches() {
        check(
            ee::s08::f_002ade88__generated,
            sound_manager_ref_ctor,
            32,
            &|c, seed| {
                let mut r = base(c, seed, 0x2ade_88);
                // `PS2SoundManager_Create` allocates 0x320 bytes (heap path).
                heap_init(c);
                let s = DATA + 0x100;
                clear(c, s, 0x40);
                w(c, gp_at(0x68c), r.next_u32());
                c.g[A0][0] = sx(s);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    // ------------------------------------------------------------- streaming

    #[test]
    fn stream_buffer_set_file_matches() {
        check(
            ssx3_ee::s08::f_002a9e78__generated,
            stream_buffer_set_file,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x2a9e78, 0);
                for i in 0..8u32 {
                    c.bus
                        .write8(OBJ2 + i, if i < 4 { b"data"[i as usize] } else { 0 });
                }
                w(c, OBJ + 0x7c, seed);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
            },
        );
    }

    #[test]
    fn stream_buffer_set_base_volume_matches() {
        leaf(
            ssx3_ee::s08::f_002a9f30__generated,
            stream_buffer_set_base_volume,
            23,
        );
    }

    #[test]
    fn sound_request_set_bank_sample_matches() {
        leaf(
            ssx3_ee::s08::f_002abce8__generated,
            sound_request_set_bank_sample,
            29,
        );
    }

    #[test]
    fn sound_manager_ref_dtor_matches() {
        check(
            ee::s08::f_002adee8__generated,
            sound_manager_ref_dtor,
            6,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x002a_dee8, 2);
                c.g[A0][0] = OBJ_2 as u64;
                // Keep the reference's manager null and do not free the object
                // (a non-pool pointer would walk the test heap).
                c.g[A1][0] = 0;
                w_2(c, OBJ_2, 0);
                let vt = vtable(c, &mut r, &[0x18]);
                w_2(c, OBJ2_2, vt);
                let _ = seed;
            },
        );
    }

    // ----------------------------------------------------------------- streams

    #[test]
    fn stream_array_create_stream_matches() {
        check(
            ee::s08::f_002aa7f0__generated,
            stream_array_create_stream,
            12,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x2aa7_f0, 4);
                let self_ = DATA;
                w_3(c, self_ + 0x8, DATA + 0x2000);
                w_3(c, self_, DATA + 0x2100);
                c.g[A0][0] = self_ as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
            },
        );
    }

    #[test]
    fn streambuffer_dtor_matches() {
        check(
            ee::s08::f_002a9df0__generated,
            streambuffer_dtor,
            SEEDS_4,
            &|c, seed| {
                base_4(c, seed, 0xa9df0, 0);
                let s = DATA + 0x100;
                c.g[A0][0] = sx(s);
                c.g[A1][0] = (seed & 1) as u64;
                c.bus.write32(s, if seed % 5 == 0 { 0 } else { 1 });
                c.bus.write32(s + 4, 1);
                c.bus.write32(s + 8, 0);
                c.bus.write32(s + 0x74, 0);
            },
        );
    }
    use super::*;

    // -- 0x002ac7f0 BankMonitorInstance_UpdateRange --------------------------

    /// `BankMonitorInstance_UpdateRange`: no range, playing inside and
    /// beyond, stopped inside and beyond (the fade-out and play callees on
    /// their short paths: already fading, or a state without a voice).
    #[test]
    fn bank_monitor_instance_update_range_matches() {
        use crate::difftest::{check, scramble, Rng, DATA};
        const THIS: u32 = DATA + 0x9_0000;
        check(
            ssx3_ee::s08::f_002ac7f0__generated,
            bank_monitor_instance_update_range,
            300,
            &|c, seed| {
                let mut r = Rng(0x002a_c7f0 ^ (seed as u64) << 20);
                scramble(c, &mut r);
                let range = if seed % 5 == 0 {
                    0
                } else {
                    (10.0 + (r.next_u32() % 100) as f32).to_bits()
                };
                c.bus.write32(THIS + 0x8c, range);
                c.bus
                    .write32(THIS + 0x8, (seed / 5) % 2 * (1 + r.next_u32() % 4));
                c.bus.write32(THIS, [0, 5][(r.next_u32() % 2) as usize]);
                c.bus.write32(THIS + 0x54, r.next_u32() % 2);
                c.g[A0][0] = sx(THIS);
                c.f[12] = match r.next_u32() % 4 {
                    0 => r.edge_f32(),
                    1 => range,
                    _ => ((r.next_u32() % 200) as f32).to_bits(),
                };
                c.pc = 0x002a_c7f0;
            },
        );
    }
    use super::*;
    use super::*;

    /// StreamBuffer_GetStatus: an unknown stream (-8), a stream with no
    /// request (0x124 = 0) and one with a request but no rate (+0x14 = 0),
    /// the lock/unlock hooks fakes.
    #[test]
    fn stream_buffer_get_status_matches() {
        use crate::difftest::fakes::FAKE;
        const SELF: u32 = DATA + 0x20_0000;
        const OUT: u32 = SELF + 0x100;
        const STREAM: u32 = SELF + 0x1000;
        const REQ: u32 = SELF + 0x2000;
        check(
            ee::s08::f_002aa020__generated,
            stream_buffer_get_status,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_a020, 2);
                c.pc = 0x002a_a020;
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(OUT);
                // SND lock / unlock hooks.
                c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
                c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
                // One stream slot.
                c.bus.write8(0x0051_0000 - 0x56d2, 1);
                c.bus.write32(0x0051_0000 - 0x5f60, STREAM);
                let id = if seed % 3 == 0 {
                    1 + r.next_u32() % 4
                } else {
                    0
                };
                c.bus.write32(SELF + SB_STREAM, id);
                c.bus.write32(
                    STREAM + 0x124,
                    if seed % 3 == 1 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(STREAM + 0x11c, REQ);
                c.bus.write32(REQ + 0xc, r.next_u32());
                c.bus.write16(STREAM + 0x14, 0);
                c.bus
                    .write8(0x0051_0000 - 0x5718 + 0x176, r.next_u32() as u8);
            },
        );
    }

    /// StreamArray_UpdateAll: counts -1..4, null and real buffers; each
    /// buffer runs the real StreamBuffer_UpdateVolume on its short paths
    /// (no volume source, or a volume set on an unknown SND stream; the
    /// lock hooks are fakes, the age stays below the stop limit).
    #[test]
    fn stream_array_update_all_matches() {
        use crate::difftest::fakes::FAKE;
        const SELF: u32 = DATA + 0x20_0000;
        const ARRAY: u32 = SELF + 0x100;
        const BUFFERS: u32 = SELF + 0x1000;
        const VOLUME: u32 = SELF + 0x3000;
        check(
            ee::s08::f_002aac28__generated,
            stream_array_update_all,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_ac28, 6);
                c.pc = 0x002a_ac28;
                c.g[A0][0] = sx(SELF);
                c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
                c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
                c.bus.write8(0x0051_0000 - 0x56d2, 0); // no SND streams
                let n = (seed % 6) as i32 - 1;
                c.bus.write32(SELF + SA_COUNT, n as u32);
                c.bus.write32(SELF + SA_BUFFERS, ARRAY);
                c.bus.write32(VOLUME, r.f32(1.0));
                for i in 0..5u32 {
                    let b = BUFFERS + 0x100 * i;
                    c.bus
                        .write32(ARRAY + 4 * i, if r.next_u32() % 4 == 0 { 0 } else { b });
                    c.bus.write32(b + 0x80, r.next_u32() % 121);
                    c.bus
                        .write32(b + 0x10, if r.next_u32() % 2 == 0 { 0 } else { VOLUME });
                    c.bus.write8(b + 0xc, r.next_u32() as u8);
                    c.bus.write32(b + 0x8, r.next_u32() % 4);
                }
            },
        );
    }

    /// StreamBuffer_UpdateVolume: starved (stop and clear), no scale
    /// pointer, or a volume set through the real SND lock / SNDSTRM path
    /// (fake lock hooks) with products below 0, inside and above 0..127.
    #[test]
    fn stream_buffer_update_volume_matches() {
        check(
            ee::s08::f_002a9f80__generated,
            stream_buffer_update_volume,
            240,
            &|c, seed| {
                let mut r = crate::difftest::fakes::machine(c, seed, 0x2a9f80, 0);
                c.pc = 0x002a_9f80;
                let sb = DATA + 0x100;
                let scale = DATA + 0x400;
                c.g[A0][0] = sx(sb);
                c.bus.write32(sb + SB_HANDLE, r.next_u32() % 8);
                c.bus.write8(sb + SB_VOLUME, r.next_u32() as u8);
                // Audio_GetFrameRate is 60: the limit is 120.
                let starved = match seed % 4 {
                    0 => 121 + r.next_u32() % 100,
                    1 => 120,
                    _ => r.next_u32() % 120,
                };
                c.bus.write32(sb + SB_STARVED, starved);
                c.bus
                    .write32(sb + SB_SCALE_PTR, if seed % 5 == 1 { 0 } else { scale });
                let f = match seed % 3 {
                    0 => r.f32(4.0),
                    1 => r.edge_f32(),
                    _ => r.f32(1.0),
                };
                c.bus.write32(scale, f);
            },
        );
    }
    use super::*;
    use super::*;

    /// StreamBuffer_GetRequestStatus: caller's status buffer or the frame's,
    /// an unknown SND stream (status zeroed: request 0) or a known one with
    /// a request id that is negative (-8 early out) or unknown; fake lock
    /// hooks, real StreamBuffer_GetStatus / SNDSTRM_GetRequestStatus.
    #[test]
    fn stream_buffer_get_request_status_matches() {
        const SELF: u32 = DATA + 0x20_0000;
        const OUT: u32 = SELF + 0x100;
        const SCRATCH: u32 = SELF + 0x200;
        const STREAM: u32 = SELF + 0x1000;
        const REQ: u32 = SELF + 0x2000;
        check(
            ee::s08::f_002aa068__generated,
            stream_buffer_get_request_status,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_a068, 4);
                c.pc = 0x002a_a068;
                c.g[A0][0] = sx(SELF);
                c.g[A1][0] = sx(OUT);
                c.g[A2][0] = if seed % 2 == 0 { 0 } else { sx(SCRATCH) };
                // SND lock / unlock hooks.
                c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
                c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
                // One stream slot.
                c.bus.write8(0x0051_0000 - 0x56d2, 1);
                c.bus.write32(0x0051_0000 - 0x5f60, STREAM);
                let id = if seed % 3 == 0 {
                    1 + r.next_u32() % 4
                } else {
                    0
                };
                c.bus.write32(SELF + 0x8, id);
                c.bus.write32(
                    STREAM + 0x124,
                    if seed % 5 == 1 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(STREAM + 0x11c, REQ);
                c.bus.write32(
                    REQ + 0xc,
                    if seed % 4 < 2 {
                        0x8000_0000 | r.next_u32()
                    } else {
                        r.next_u32() & 0x7fff_ffff
                    },
                );
                c.bus.write16(STREAM + 0x14, 0);
                c.bus
                    .write8(0x0051_0000 - 0x5718 + 0x176, r.next_u32() as u8);
            },
        );
    }

    /// StreamArray_GetState: empty and non-empty slots; the real
    /// StreamBuffer_GetState runs on a buffer whose stream ids are small.
    #[test]
    fn stream_array_get_state_matches() {
        check(
            ee::s08::f_002aae08__generated,
            stream_array_get_state,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2aae_08, 0);
                c.pc = 0x002a_ae08;
                let arr = DATA + 0x100;
                let list = DATA + 0x200;
                let buffer = DATA + 0x1000;
                c.g[A0][0] = sx(arr);
                c.g[A1][0] = sx(seed % 16);
                c.bus.write32(arr + OC264_SA_BUFFERS, list);
                let entry = if seed % 3 == 0 { 0 } else { buffer };
                c.bus.write32(list + 4 * (seed % 16), entry);
                for i in 0..8u32 {
                    c.bus.write32(buffer + 4 * i, r.next_u32() % 4);
                }
            },
        );
    }
    use super::*;
    use super::*;
    use super::*;
    use super::*;
    use super::*;
    use super::*;
    use crate::difftest::RA as RET_ADDR;

    /// `StreamBuffer_IsPlaying`: the fixture of
    /// `stream_buffer_get_request_status_matches` (fake lock hooks, one SND
    /// stream slot, known/unknown streams and requests), with the status
    /// word in the frame preset to 2 (playing) or junk where the real
    /// callees leave it alone.
    #[test]
    fn stream_buffer_is_playing_matches() {
        use crate::difftest::{check, SP};
        const SELF: u32 = DATA + 0x20_0000;
        const STREAM: u32 = SELF + 0x1000;
        const REQ: u32 = SELF + 0x2000;
        check(
            ee::s08::f_002aa1b8__generated,
            stream_buffer_is_playing,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_a1b8, 4);
                c.pc = 0x002a_a1b8;
                c.g[A0][0] = sx(SELF);
                c.bus
                    .write32(SP - 0x20, if seed % 2 == 0 { 2 } else { r.next_u32() % 4 });
                c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
                c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
                c.bus.write8(0x0051_0000 - 0x56d2, 1);
                c.bus.write32(0x0051_0000 - 0x5f60, STREAM);
                let id = if seed % 3 == 0 {
                    1 + r.next_u32() % 4
                } else {
                    0
                };
                c.bus.write32(SELF + 0x8, id);
                c.bus.write32(
                    STREAM + 0x124,
                    if seed % 5 == 1 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(STREAM + 0x11c, REQ);
                c.bus.write32(
                    REQ + 0xc,
                    if seed % 4 < 2 {
                        0x8000_0000 | r.next_u32()
                    } else {
                        r.next_u32() & 0x7fff_ffff
                    },
                );
                c.bus.write16(STREAM + 0x14, 0);
                c.bus
                    .write8(0x0051_0000 - 0x5718 + 0x176, r.next_u32() as u8);
            },
        );
    }

    /// `BankMonitorInstance_Reset`: the `FadeOut` fixture (already fading,
    /// or kind 1 / 2 / other; null SND lock callbacks are fakes, some
    /// unwind; a handle that finds no channel), with the real `FadeOut`.
    #[test]
    fn bank_monitor_instance_reset_matches() {
        covered(
            ee::s08::f_002abb38__generated,
            bank_monitor_instance_reset,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_bb38, 4);
                c.pc = 0x002a_bb38;
                let inst = DATA;
                c.g[A0][0] = inst as u64;
                c.bus.write32(inst + 0x8, r.next_u32()); // playing
                c.bus.write32(
                    inst + 0x54,
                    if seed % 5 == 0 {
                        1 + r.next_u32() % 3
                    } else {
                        0
                    },
                );
                let kind = match seed % 4 {
                    0 | 1 => 1,
                    2 => 2,
                    _ => r.next_u32() % 6,
                };
                c.bus.write32(inst, kind);
                let handle = if r.next_u32() % 2 == 0 {
                    0xffff_ffff
                } else {
                    r.next_u32() & 0x7fff_ffff
                };
                c.bus.write32(inst + 0x4, handle);
            },
        );
    }

    /// BankMonitorInstance_FadeOut: already fading, or kind 1 / 2 / other.
    /// SND_Lock/SND_Unlock's lock callbacks are null (fakes, some unwind);
    /// the handle is negative or out of range, so the fade calls find no
    /// channel and return at once.
    #[test]
    fn bank_monitor_instance_fade_out_matches() {
        use crate::difftest::fakes::{covered, machine};
        covered(
            ee::s08::f_002ac180__generated,
            bank_monitor_instance_fade_out,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2ac180, 4);
                c.pc = 0x002a_c180;
                let inst = DATA;
                c.g[A0][0] = inst as u64;
                c.bus.write32(
                    inst + BMI_FADING,
                    if seed % 5 == 0 {
                        1 + r.next_u32() % 3
                    } else {
                        0
                    },
                );
                let kind = match seed % 4 {
                    0 | 1 => 1,
                    2 => 2,
                    _ => r.next_u32() % 6,
                };
                c.bus.write32(inst + BMI_KIND, kind);
                c.bus.write32(inst + BMI_PLAYING, r.next_u32());
                let handle = if r.next_u32() % 2 == 0 {
                    0xffff_ffff
                } else {
                    r.next_u32() & 0x7fff_ffff
                };
                c.bus.write32(inst + BMI_HANDLE, handle);
            },
        );
    }

    /// StreamArray_GetTimeRemaining: negative index, empty slot, and a
    /// present buffer on which the real StreamBuffer_GetTimeRemaining runs
    /// (fixture as in `stream_buffer_get_request_status_matches`: one stream
    /// slot, SND lock/unlock and the request lookup on fakes).
    #[test]
    fn stream_array_get_time_remaining_matches() {
        const ARR: u32 = DATA + 0x20_0000;
        const LIST: u32 = ARR + 0x100;
        const BUF: u32 = ARR + 0x400;
        const STREAM: u32 = ARR + 0x1000;
        const REQ: u32 = ARR + 0x2000;
        check(
            ee::s08::f_002aae78__generated,
            stream_array_get_time_remaining,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_ae78, 4);
                c.pc = 0x002a_ae78;
                let i = seed % 8;
                c.g[A0][0] = sx(ARR);
                c.g[A1][0] = if seed % 5 == 0 {
                    sx(0x8000_0000 | r.next_u32())
                } else {
                    i as u64
                };
                c.bus.write32(ARR + SAT_BUFFERS, LIST);
                c.bus
                    .write32(LIST + 4 * i, if seed % 5 == 1 { 0 } else { BUF });
                // SND lock / unlock hooks.
                c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
                c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
                // One stream slot.
                c.bus.write8(0x0051_0000 - 0x56d2, 1);
                c.bus.write32(0x0051_0000 - 0x5f60, STREAM);
                let id = if seed % 3 == 0 {
                    1 + r.next_u32() % 4
                } else {
                    0
                };
                c.bus.write32(BUF + 0x8, id);
                c.bus.write32(
                    STREAM + 0x124,
                    if seed % 7 == 1 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(STREAM + 0x11c, REQ);
                c.bus.write32(REQ + 0xc, r.next_u32());
                c.bus.write16(STREAM + 0x14, 0);
                c.bus
                    .write8(0x0051_0000 - 0x5718 + 0x176, r.next_u32() as u8);
            },
        );
    }

    /// StreamBuffer_GetTimeRemaining: the real StreamBuffer_GetRequestStatus
    /// chain (fake SND lock/unlock hooks, some unwinding) on one SND stream
    /// whose idle status hands request id 0 on: the request playing (head
    /// of the list: state 2, remaining time from its byte counters and the
    /// sample rate), queued behind another (state 1), inactive (state 0) or
    /// not found (state 3).
    #[test]
    fn stream_buffer_get_time_remaining_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        use crate::difftest::{check, DATA};
        const SELF: u32 = DATA + 0x20_0000;
        const STREAM: u32 = SELF + 0x1000;
        const HEAD: u32 = SELF + 0x2000;
        const REQ: u32 = SELF + 0x2100;
        check(
            ssx3_ee::s08::f_002aa108__generated,
            stream_buffer_get_time_remaining,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_a108, 4);
                c.pc = 0x002a_a108;
                c.g[A0][0] = sx(SELF);
                // SND lock / unlock hooks.
                c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
                c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
                // One stream slot (SND stream 0), idle: status 0, request id 0.
                c.bus.write8(0x0051_0000 - 0x56d2, 1);
                c.bus.write32(0x0051_0000 - 0x5f60, STREAM);
                c.bus.write32(SELF + 0x8, 0);
                c.bus.write32(STREAM + 0x124, 0);
                c.bus
                    .write16(STREAM + 0x14, 1 + (r.next_u32() % 48000) as u16);
                c.bus
                    .write16(STREAM + 0x18, 1 + (r.next_u32() % 48000) as u16);
                c.bus
                    .write8(0x0051_0000 - 0x5718 + 0x176, r.next_u32() as u8);
                // Request list: REQ (id 0) first, behind HEAD (id 1), or absent.
                let case = seed % 4;
                let first = if case == 1 { HEAD } else { REQ };
                c.bus.write32(STREAM + 0x11c, first);
                c.bus.write32(HEAD, REQ);
                c.bus.write32(HEAD + 0xc, 1);
                c.bus.write32(REQ, 0);
                c.bus.write32(REQ + 0xc, if case == 3 { 7 } else { 0 });
                c.bus.write8(
                    REQ + 0x24,
                    if case == 2 {
                        0
                    } else {
                        1 + r.next_u32() as u8 % 4
                    },
                );
                let done = r.next_u32() & 0xf_ffff;
                c.bus.write32(REQ + 0x14, done);
                c.bus.write32(REQ + 0x18, done + (r.next_u32() & 0xf_ffff));
                c.bus.write32(REQ + 0x1c, r.next_u32() & 0xffff);
            },
        );
    }

    /// StreamArray_IsPlaying: an empty slot (false) or a buffer whose real
    /// StreamBuffer_IsPlaying -> GetRequestStatus chain runs on one SND
    /// stream slot (unknown or known stream, request state random), with
    /// fake lock hooks that unwind on some seeds.
    #[test]
    fn stream_array_is_playing_matches() {
        use crate::difftest::fakes::{machine, FAKE};
        const ARRAY: u32 = DATA + 0x20_0000;
        const LIST: u32 = ARRAY + 0x40;
        const BUFFER: u32 = ARRAY + 0x400;
        const STREAM: u32 = ARRAY + 0x1000;
        const REQ: u32 = ARRAY + 0x2000;
        check(
            ee::s08::f_002ab028__generated,
            stream_array_is_playing,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2a_b028, 4);
                c.pc = 0x002a_b028;
                let i = seed % 8;
                c.g[A0][0] = sx(ARRAY);
                c.g[A1][0] = i as u64;
                c.bus.write32(ARRAY + STREAM_ARRAY_BUFFERS, LIST);
                c.bus
                    .write32(LIST + 4 * i, if seed % 4 == 0 { 0 } else { BUFFER });
                // SND lock / unlock hooks.
                c.bus.write32(0x0045_0000 - 0x3b94, FAKE);
                c.bus.write32(0x0045_0000 - 0x3b90, FAKE + 0x10);
                // One stream slot.
                c.bus.write8(0x0051_0000 - 0x56d2, 1);
                c.bus.write32(0x0051_0000 - 0x5f60, STREAM);
                let id = if seed % 3 == 0 {
                    1 + r.next_u32() % 4
                } else {
                    0
                };
                c.bus.write32(BUFFER + 0x8, id);
                c.bus.write32(
                    STREAM + 0x124,
                    if seed % 5 == 1 { 0 } else { r.next_u32() | 1 },
                );
                c.bus.write32(STREAM + 0x11c, REQ);
                c.bus.write32(
                    REQ + 0xc,
                    if seed % 6 < 3 {
                        0x8000_0000 | r.next_u32()
                    } else {
                        r.next_u32() % 4
                    },
                );
                c.bus.write16(STREAM + 0x14, 0);
                c.bus
                    .write8(0x0051_0000 - 0x5718 + 0x176, r.next_u32() as u8);
            },
        );
    }
    use super::*;

    /// BankMonitorInstance_UpdateDistance: the virtual is a fake that
    /// returns a random f0 (sqrt of its bits) and sometimes unwinds.
    #[test]
    fn bank_monitor_instance_update_distance_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        covered(
            ee::s08::f_002abd48__generated,
            bank_monitor_instance_update_distance,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2abd48, 1);
                let this = DATA;
                let listener = DATA + 0x1000;
                let vt = DATA + 0x3000;
                fill(c, &mut r, this, 0x20);
                c.bus.write32(listener + LISTENER_VTABLE, vt);
                c.bus
                    .write16(vt + 0x30, (r.next_u32() as i16 as i32 & !3) as u16);
                c.bus.write32(vt + 0x34, FAKE);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(listener);
                c.pc = 0x002a_bd48;
            },
        );
    }
}
