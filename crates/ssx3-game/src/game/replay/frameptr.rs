//! `src/replay/frameptr.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::vu0::W;
use crate::events::back_edge_unwinds;
use crate::call::VSlot;
use crate::call::Unwound;
pub(crate) use crate::game::render::particle::at;
pub(crate) use crate::game::render::lightman::off;
pub(crate) use crate::game::render::lightman::Oc49Mem;
pub(crate) use crate::game::render::lightman::addu;
pub(crate) use crate::game::fe::festaterewards2::r;
pub(crate) use crate::game::fe::festaterewards2::Ops;
pub(crate) use crate::game::dirtysock::tags2::p_2 as p;
pub(crate) use crate::game::dirtysock::tags2::sw;
pub(crate) use crate::game::dirtysock::tags2::leave;
pub(crate) use crate::game::dirtysock::tags2::p as p_2;
pub(crate) use crate::game::dirtysock::tags2::p as p_3;
pub(crate) use crate::game::dirtysock::tags2::lw;
pub(crate) use crate::game::fe::feasyncfile::at_2;
pub(crate) use crate::game::scripter::ssxscriptengine::p as p_4;
pub(crate) use crate::game::be::be::gp;
pub(crate) use crate::game::fe::feasyncfile::p as p_5;
pub(crate) use crate::game::fe::feasyncfile::gp_12 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::p as p_6;
pub(crate) use crate::game::fe::feasyncfile::p as p_7;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw_2;
pub(crate) use crate::game::animation::rideranimbase::sw as sw_2;
pub(crate) use crate::game::camera::cameraalgorithms::lh;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::p_8;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0026_d5e8, world_state_stream_ctor, "WorldStateStream_ctor"),
    (0x0026_e908, replay_stream_alloc_buffer, "ReplayStream_AllocBuffer"),
    (0x0026_e950, replay_stream_set_buffer, "ReplayStream_SetBuffer"),
    (0x0026_e968, replay_stream_free_buffer, "ReplayStream_FreeBuffer"),
    (0x0026_e9b0, replay_stream_rewind_read, "ReplayStream_RewindRead"),
    (0x0026_ea20, replay_stream_read, "ReplayStream_Read"),
    (0x0026_ea68, replay_stream_peek_int, "ReplayStream_PeekInt"),
    (0x0026_eb48, replay_stream_read_object_ref, "ReplayStream_ReadObjectRef"),
    (0x0026_ecd8, replay_stream_write_object_ref, "ReplayStream_WriteObjectRef"),
    (0x0026_ed28, replay_stream_write_node_ref, "ReplayStream_WriteNodeRef"),
    (0x0026_ed88, replay_snapshot_empty_hook_a, "Replay_SnapshotEmptyHookA"),
    (0x0026_ed98, replay_next_camera, "Replay_NextCamera"),
    (0x0026_edd8, replay_apply_camera, "Replay_ApplyCamera"),
    (0x0026_ee68, replay_enable_replay_cameras, "Replay_EnableReplayCameras"),
    (0x0026_eea0, replay_restore_game_cameras, "Replay_RestoreGameCameras"),
    (0x0026_ef30, replay_enable_instant_replay_cameras, "Replay_EnableInstantReplayCameras"),
    (0x0026_ef80, replay_restore_instant_replay_cameras, "Replay_RestoreInstantReplayCameras"),
    (0x0026_f180, replay_init, "Replay_Init"),
    (0x0026_f428, replay_free_buffers, "Replay_FreeBuffers"),
    (0x0026_f7b8, replay_advance_from_state10, "Replay_AdvanceFromState10"),
    (0x0026_f7f8, replay_save_resume_snapshot, "Replay_SaveResumeSnapshot"),
];

// ---------------------------------------------------------------------------
// Callees (addresses the generated body reaches through the function table).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::be::beintmission::callee::FIND_MISSION;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_RESET_STATE;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_SET_ALGORITHM_IMMEDIATE;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_SET_ALGORITHM_2 as CHASE_CAM_SET_ALGORITHM;
    pub(crate) use crate::game::be::beintmission::callee::GET_RIDER_PREVIEW;
    pub(crate) use crate::game::be::beintmission::callee::RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::be::beintmission::callee::NET_FE_STATE_CTOR;
    pub(crate) use crate::game::be::beintmission::callee::LOBBY_MENU_REQUEST_JOIN_ROOM;
    pub(crate) use crate::game::be::beintmission::callee::POPUP_CONFIRM_CFG_RESET;
    pub(crate) use crate::game::be::beintmission::callee::REWARD_GALLERY_CHECK_REWARDS_READY;
    pub(crate) use crate::game::be::beintmission::callee::REWARD_GALLERY_POLL_SHAPE_LOAD;
    pub(crate) use crate::game::be::beintmission::callee::BUDDY_MAN_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::be::beintmission::callee::REPLAY_APPLY_CAMERA;
    pub(crate) use crate::game::be::beintmission::callee::RACE_PROGRESS_RESET_ALL;
    pub(crate) use crate::game::be::beintmission::callee::MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_STOP_MAIN_LISTS;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_STOP_AUX_LISTS;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_DEACTIVATE_BINDING;
    pub(crate) use crate::game::be::beintmission::callee::CROWD_AUDIO_STOP_ALL;
    pub(crate) use crate::game::be::beintmission::callee::BANK_MONITOR_STOP_ALL;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_LEVEL_TO_VOLUME;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::be::beintmission::callee::SND_BANK_SET_MEMORY;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_IS_GAME_MUSIC_ENABLED;
    pub(crate) use crate::game::be::beintmission::callee::MUSIC_PLAYER_STOP_SONG;
    pub(crate) use crate::game::be::beintmission::callee::UI_STATE_UPDATE;
    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::be::beintmission::callee::BE_NEW_PLAYER_GET_INBOX;
    pub(crate) use crate::game::be::beintmission::callee::WSCRIPT_MAN_EVENT_QUEUE_POP;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_START_PRERACE_MUSIC;
    pub(crate) use crate::game::be::beintmission::callee::SLOGIN_DELETE;
    pub(crate) use crate::game::be::beintmission::callee::CAMERA_TRIGGER_MAN_UPDATE_MASK1;
    pub(crate) use crate::game::be::beintmission::callee::EA_MEMSET;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE_2 as BUILTIN_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_SOUND_BASE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR_2 as UI_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_REMOVE_TICK_CALLBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::CW_SCRIPT_CACHE_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_CAMERA_SET_CONTROLLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MENU_ON_LOBBY_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_FORWARD_EVENT_TO_UI;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::LUNO_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_PLAY_CHARACTER_TUNE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_GAME_UTIL_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::CSETUP_MAN_SEND_PACKET;
    pub(crate) use crate::game::dirtysock::tags2::callee::CVOIP_MAN_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_RPC_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_RPC_SET_PARAMS;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_HIGHLIGHT_SLOT_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SAVE_EMIT;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_REQUEST_CARD_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_HANDLE_MUSIC_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_ARCADE_SAY;
    pub(crate) use crate::game::dirtysock::tags2::callee::EATIMER_REMOVE_CALLBACK;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_NAMES;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_ICONS;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SCROLL_ARROWS;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_COUNT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_DISCONNECTED_POPUP_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_LOGIN_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_LOGIN_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_QUICK_PLAY_POPUP_ON_OPTIONS_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_STATS_REFRESH_RESULTS;
    pub(crate) use crate::game::fe::festaterewards2::callee::POPUP_CONFIRM_GET_TEXT_BOX_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::PREVIEW_REWARD_POLL_IMAGE_LOAD;
    pub(crate) use crate::game::fe::festaterewards2::callee::MOUNTAIN_ROOM_UPDATE_EVENT_ICONS;
    pub(crate) use crate::game::fe::festaterewards2::callee::TROPHY_ROOM_LOAD_TROPHY_TEXTURES;
    pub(crate) use crate::game::fe::festaterewards2::callee::SECTION_MAN_FIND_LOGICAL_SECTION;
    pub(crate) use crate::game::fe::festaterewards2::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::APP_MODULE_TRY_EXIT;
    pub(crate) use crate::game::fe::festaterewards2::callee::S_LOGIN_UPDATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::USB_KB_DEFAULT_CONNECT_CB;
    pub(crate) use crate::game::fe::festaterewards2::callee::WORLD_STATE_STREAM_RESET;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_STREAM_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_FIND_NEXT_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_FIND_PREV_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_SNAP_TO_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_SET_STATE_2;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_LIST_ADD_MOVIE;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_LIST_POLL_LOADING;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_ENGINE_UNLOAD_WORLD_SCRIPTS;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_ENGINE_LIST_STOP;
    pub(crate) use crate::game::fe::festaterewards2::callee::CROWD_AUDIO_INSTANCE_STOP_SOUND;
    pub(crate) use crate::game::fe::festaterewards2::callee::SONG_INSTANCE_GET_DUCK_TO_LOOPS;
    pub(crate) use crate::game::fe::festaterewards2::callee::WORLD_TRIGGER_MANAGER_GET_TRIGGER_INFO;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_MAN_FREE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_GET_POOL_BLOCK_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::DL_HEAP_USABLE_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_REPORT_POOL_STATS;
    pub(crate) use crate::game::fe::festaterewards2::callee::PS2_PAD_CREATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::ANIM_OBJECT_V36;
    pub(crate) use crate::game::fe::festaterewards2::callee::DYNAMIC_EMITTER_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_STATE_UPDATE as UI_STATE_UPDATE_2;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SCROLL_SET_WIDE_STRING;
    pub(crate) use crate::game::fe::festaterewards2::callee::TWSCRIPT_CACHE_RELOCATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::EA_MUTEX_LOCK;
    pub(crate) use crate::game::fe::festaterewards2::callee::EA_MUTEX_UNLOCK;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCE_USBKB_SET_ARRANGEMENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCE_USBKB_SET_READ_MODE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEMSET;
    pub(crate) use crate::game::fe::festaterewards2::callee::SPRINTF;
    pub(crate) use crate::game::dirtysock::tags2::callee::REWARDS_FILE_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOUNDED_CAMERA_ALGORITHM_BASE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_CTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_IS_EVENT_LOADED;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_CLOSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_LISTS;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_LOBBY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_PEER_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SHARED_LOGIN_MAN_SHUTDOWN;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_MAN_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_CACHE_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SNAPSHOT_EMPTY_HOOK_B;
    pub(crate) use crate::game::dirtysock::tags2::callee::WORLD_STATE_RESTORE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RESTORE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_FREE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_END_PLAYBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_CHANNEL_RELEASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_HEAP_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ANIM_BANK_MAN_FIND_BANK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS_2 as SCRIPT_ENGINE_POLL_ASYNC_LOADS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_IS_BANK_SLOT_LOADED;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_CURRENT_SKIPPABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_FIELD_B0;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_FIND_BANK_SLOT_BY_ID;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_CANCEL_BANK_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_LIST_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE as BUILTIN_DELETE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::FEMODULE_SET_PREVIEW_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_APP_START_FE_MODULE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOC_STRING_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BENEW_PLAYER_INT_IS_MESSAGE_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_MESSAGE_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_INTERFACE_RIDER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MAN_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MAN_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::WORLD_VIEW_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_STREAM_WRITE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_REQUEST_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_REQUEST_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_API_ADD_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::DIRTY_SOCK_CREATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::DIRTY_SOCK_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::APP_SOMETHING;
    pub(crate) use crate::game::dirtysock::tags2::callee::ON_QUIT_POPUP_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::EMPTY_CHILD_HANDLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_ON_CHILD_EVENT_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_CTOR_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_DTOR_2 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::STATS_SELECT_CATEGORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_UPDATE_CURSOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_CARD_SET_USE_SAVE_TITLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::TROPHY_ROOM_RELEASE_IMAGES;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_GET_INBOX;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_PUSH_FREE_NODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_END_OPERATION_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_SET_PRESENCE_ADDR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_DESTROY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_OBJ_CAM_SET_OWNER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_OBJ_TABLE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD_2 as SND_BANK_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLINK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_FREE_MEMORY_2 as SND_BANK_FREE_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RECORD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_ADD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::STREAM_BUFFER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_INSTANCE_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::PATH_PLAT_REFRESH_STATUS;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_CARD_ASCII_CHAR_TO_SJIS;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_GET_SAVE_NAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_READ_SAVE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_PAGE_SELECT_NEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_ITEM_MEASURE_ROW;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_FREE_HISTORIES;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_BAKE_GROUP;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_EXPORT_HISTORY_FILES;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_LOAD_HISTORY_FILE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SNOW_BOX_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT0;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT1;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT2;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT3;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT4;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT5;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT6;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_MODULE_GET_PLAYER_PAD_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::UIMENU_SET_SELECTION;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_TEXT_GET_FONT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_REWARD_INT_IS_UNLOCKED;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR as UI_STATE_DTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_LIST_DIR_ALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_BX_STRING_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE as BUILTIN_DELETE_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_LIST_GET_AT_3 as LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SLOGIN_MC_GET_PORT_SLOT_2 as SLOGIN_MC_GET_PORT_SLOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCE_MC_FORMAT;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPRINTF_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::RAND_NEXT_MAIN;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SETUP_MAN_SEND_QUIT;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_LUNO_TABLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SET_LOCAL_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SEND_CHARACTER_MSG;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_CAN_SKIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_GET_PLAY_STATUS_2 as SCRIPT_LIST_GET_PLAY_STATUS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_4 as SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR_3 as SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_SCRIPT_SET_OWNER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PATH_PLAT_FIND_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_DRAW_ROW_4 as MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MATH_EVAL_CUBIC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_CLOSE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_ADD_VERTEX;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_SCROLL_WINDOW;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_MASK_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_CURSOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_PACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_UNPACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_REWARD_INT_IS_PEAK_GOAL_COMPLETE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VEC3_NEG_DOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_3 as SCRIPT_LIST_IS_PLAYING_SCRIPT_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO_4 as SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_INSTANCE_GET_REMAINING_STEPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CONCAT_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ATTACHED_DYNAMIC_EMITTER_CREATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_MAN2D_CLEAR_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_STREAM_REWIND_READ;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_ALGORITHM_DEFAULT_FOV;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_CHARACTER as BE_GET_RIDER_CHARACTER_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_PROCESS_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_FLAG_MANAGER_GROUP_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_TABLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_FIRST_5 as UI_LIST_FIRST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_ASCII_TO_WIDE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_TEXT_SET_WIDE_STRING_INTERNAL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BELIBRARY_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_MAN_FREE_ENTRY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_SCREEN_FIND_OBJECT_3 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST as UI_LIST_FIRST_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_4 as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_2 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_3 as SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_10 as NET_FE_STATE_ON_CHILD_EVENT_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_2 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_HIDE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SHOW;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP_2 as NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_GET_CHARACTER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_FILL_PLAYER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_SET_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_PDA_TEMPLATE_CLOSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_MESSAGE_CENTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_COUNT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_ENTRY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_DATA_GET_BANK_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_UNREGISTER_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ANIM_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_SOUND_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_INSTANCE_3 as SCRIPT_ENGINE_GET_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_OBJ_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ON_ACTIVATED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_3 as SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CUI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_3 as UI_LIST_NODE_DTOR_BODY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_STREAMER_UPDATE_VIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_SET_IN_GAME_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SET_INPUT_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_SET_OWNER;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_3 as WORLD_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_READ_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_FLAG_MANAGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_MANAGER_ADD_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_APPLY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_ADVANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_PROCESS_DTOR_2 as GAME_PROCESS_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_RESET_RACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODE_MAN_RESET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_REQUEST_STATE_5 as GAME_MODULE_REQUEST_STATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_START_NEXT_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_APP;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_INTERFACE_GET_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_BE_2 as BE_GET_BE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_5 as SPRINTF_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::VSPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::EDITOR_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_SELECT_ALGORITHM;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_FREE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_LOAD_BANK_FROM_MEMORY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_MUSIC_TIER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_TRACK_TYPE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_KILL_ALL_PROCESSES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS_2 as SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COMPUTE_OFFSET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_AABB_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_PAD_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_NODE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME_2 as UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBE_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_2 as NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_ADD_CHILD;
}


/// `void ReplayStream_SetBuffer(ReplayStream *stream, void *buf, uint len)`
/// (0x26e950): set begin/end/cursor and the length.
pub fn replay_stream_set_buffer(c: &mut Cpu) {
    c.cycles += 6;
    let stream = Addr(abi::arg(c, 0));
    let buf = abi::arg(c, 1);
    let len = abi::arg(c, 2);
    c.g[V0][0] = sx(buf.wrapping_add(len));
    stream.off(0x10).sw(c, c.g[A2][0]);
    stream.off(0x8).sw(c, c.g[V0][0]);
    stream.off(0xc).sw(c, c.g[A1][0]);
    stream.off(0x4).sw(c, c.g[A1][0]);
    abi::ret(c);
}


/// `void ReplayStream_RewindRead(ReplayStream *stream)` (0x26e9b0): reset the
/// read cursor to the buffer begin.
pub fn replay_stream_rewind_read(c: &mut Cpu) {
    c.cycles += 3;
    let stream = Addr(abi::arg(c, 0));
    let begin = stream.off(0x4).lw(c);
    c.g[V0][0] = begin;
    stream.off(0xc).sw(c, begin);
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Replay stream.

/// `void ReplayStream_WriteObjectRef(Stream* out, void* obj)` (0x26ecd8):
/// write the object's id at +0x78 (or -1) as four bytes through the stream's
/// virtual slot 1.
pub fn replay_stream_write_object_ref(c: &mut Cpu) {
    guest(c, |c| -> Flow {
        c.cycles += 7;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] |= 0xffff;
        frame.save_ra(c, 0x10);
        c.g[A3][0] = c.g[A0][0];
        let none = c.g[A1][0] == 0;
        let sp = sp(c);
        c.sw(sp, c.g[V0][0]);
        if !none {
            c.cycles += 2;
            c.g[V0][0] = c.lw(at(c, A1, 0x78));
            c.sw(sp, c.g[V0][0]);
        }
        c.cycles += 10;
        c.g[V0][0] = c.lw(r32(c, A3));
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0x4;
        c.g[A0][0] = c.lh(at(c, V0, 0x8));
        c.g[V1][0] = c.lw(at(c, V0, 0xc));
        let target = r32(c, V1);
        c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0026_ed18)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------

/// `void ReplayStream_WriteNodeRef(ReplayStream *out, NodeRef *node)`
/// (0x0026ed28): append `{ u32 id; u16 type; }` (id = `node->+0x18->+0x78`,
/// type = `+0x18->+8`; `-1`/0 when the node has no reference) through
/// `out->vt[1](out + delta, &rec, 8)`.
pub fn replay_stream_write_node_ref(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.g[V0][0] |= 0xffff;
        frame.save_ra(c, 0x10);
        let rec = frame.at(c, 0);
        c.stw(rec, c.g[V0][0]);
        let out = abi::arg(c, 0);
        let node = abi::arg(c, 1);
        c.g[A3][0] = sx(out);
        c.g[V1][0] = c.ldw(off(node, 0x18));
        if c.g[V1][0] == 0 {
            c.cycles += 1;
            c.sth(off(rec, 4), 0);
        } else {
            c.cycles += 5;
            let p = c.g[V1][0] as u32;
            c.g[V0][0] = c.ldw(off(p, 0x78));
            c.stw(rec, c.g[V0][0]);
            c.g[V1][0] = c.ldhu(off(p, 8));
            c.sth(off(rec, 4), c.g[V1][0]);
        }
        c.cycles += 10;
        c.g[V0][0] = c.ldw(out);
        c.g[A1][0] = sx(frame.at(c, 0));
        c.g[A2][0] = 8;
        c.g[A0][0] = c.ldh(off(c.g[V0][0] as u32, 8));
        c.g[V1][0] = c.ldw(off(c.g[V0][0] as u32, 0xc));
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = addu(c.g[A3][0], c.g[A0][0]);
        call_ptr(c, target, 0x0026_ed7c)?;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Replay.

/// `void Replay_EnableReplayCameras(Replay* self)` (0x26ee68): refresh the
/// camera triggers, then apply the replay camera.
pub fn replay_enable_replay_cameras(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x004c_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5830));
        call_ptr(c, callee::CAMERA_TRIGGER_MAN_UPDATE_MASK1, 0x0026_ee84)?;
        c.g[A0][0] = c.g[S0][0];
        callee::REPLAY_APPLY_CAMERA.call(c, 0x0026_ee8c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void Replay_RestoreInstantReplayCameras(void)` (0x26ef80): re-arm the
/// in-game camera triggers and restore the replay camera controller.
pub fn replay_restore_instant_replay_cameras(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x004c_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5830));
        callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS.call(c, 0x0026_ef94)?;
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        c.g[A1][0] = 2;
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x84).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x84).lw(c);
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(4).lw(c);
        callee::GAME_CAMERA_SET_CONTROLLER.call(c, 0x0026_efac)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// World / replay streams.

/// `WorldStateStream* WorldStateStream_ctor(WorldStateStream*)` (0x26D5E8): the
/// replay-stream constructor, vtable `0x481850`, and an empty stream.
pub fn world_state_stream_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        callee::REPLAY_STREAM_CTOR.call(c, 0x0026_d5fc)?;
        c.g[V1][0] = sx(0x0048_1850);
        c.g[A0][0] = c.g[S0][0];
        Addr(this as u32).sw(c, 0x0048_1850);
        callee::WORLD_STATE_STREAM_RESET.call(c, 0x0026_d610)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}


/// `void Replay_NextCamera(Replay* self)` (0x26ED98): step the camera index at
/// +0x630 through the nine replay cameras (`(n + 1) % 9`) and apply it.
pub fn replay_next_camera(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 9;
        c.cycles += 9;
        let index = c.lw_r_2(V0, r(c, A2).wrapping_add(0x630));
        let bumped = sx((index as u32).wrapping_add(1));
        c.g[V0][0] = bumped;
        let divisor = c.g[A1][0] as u32;
        let (_, rem) = abi::div(c, bumped as u32, divisor);
        c.g[V1][0] = sx(rem);
        Addr(r(c, A2).wrapping_add(0x630)).sw(c, sx(rem));
        callee::REPLAY_APPLY_CAMERA.call(c, 0x0026_edc8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void Replay_AdvanceFromState10(Replay* self)` (0x26F7B8): a replay in state
/// 10 moves on to state 11; states 14 and 15 are ignored.
pub fn replay_advance_from_state10(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let state = c.lw_r_2(V1, r(c, A0));
        let frame = Frame::open(c, 0x10);
        let d = sx((state as u32).wrapping_sub(0xe));
        c.g[V0][0] = d;
        let ignored = d < 2;
        c.g[V0][0] = (ignored as u64) ^ 1;
        frame.save_ra(c, 0);
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0xa;
            let not_ten = c.g[V1][0] != c.g[V0][0];
            frame.restore_ra(c, 0);
            if !not_ten {
                c.cycles += 2;
                c.g[A1][0] = 0xb;
                // The call returns to 0x26F7E8, which is the loop head: control
                // falls through the shared tail block (one more cycle, `ra`
                // reloaded from the frame) instead of returning straight away.
                callee::REPLAY_SET_STATE_2.call(c, 0x0026_f7e8)?;
                c.cycles += 1;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}


/// `void ReplayStream_FreeBuffer(ReplayStream* self)` (0x26e968): free the
/// stream's frame buffer and clear its block fields.
pub fn replay_stream_free_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.lw(at(c, S0, 0x04));
        if c.g[A0][0] == 0 {
            sw(c, r32(c, S0), 0x10, 0);
        } else {
            c.cycles += 3;
            call_ptr(c, callee::EA_MEM_FREE_2, 0x0026_e98c)?;
            sw(c, r32(c, S0), 0x10, 0);
        }
        c.cycles += 7;
        sw(c, r32(c, S0), 0x04, 0);
        sw(c, r32(c, S0), 0x08, 0);
        sw(c, r32(c, S0), 0x0c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void ReplayStream_Read(ReplayStream* self, void* dst /*a1*/, int n
/// /*a2*/)` (0x26ea20): copy `n` bytes to `dst` and advance the stream's
/// write cursor.
pub fn replay_stream_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[A1][0] = c.lw(at(c, S0, 0x0c));
        call_ptr(c, callee::REPLAY_STREAM_WRITE, 0x0026_ea44)?;
        c.g[V0][0] = c.lw(at(c, S0, 0x0c));
        c.g[RA][0] = c.ld(p(sp(c), 0));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
        sw(c, r32(c, S0), 0x0c, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        leave(c, 0x30);
        Ok(())
    });
}


/// `void Replay_RestoreResumeSnapshot(Replay* self)` (0x26f850): restore the
/// resume snapshot, then free it.
pub fn replay_restore_resume_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = c.lw(at(c, S0, 0x3d0));
        if c.g[A1][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 7;
            callee::REPLAY_RESTORE_SNAPSHOT.call(c, 0x0026_f874)?;
            c.g[A1][0] = c.lw(at(c, S0, 0x3d0));
            c.g[A0][0] = c.g[S0][0];
            callee::REPLAY_FREE_SNAPSHOT.call(c, 0x0026_f880)?;
            sw(c, r32(c, S0), 0x3d0, 0);
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void ReplayStream_AllocBuffer(self, size)` (0x26e908): allocate the
/// replay stream's buffer and point `+0xc`, `+0x4` and `+0x8` at it, with the
/// write cursor (`+0x10`) cleared.
pub fn replay_stream_alloc_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x48_0000;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        c.g[A3][0] = 0;
        c.g[A1][0] = sx(0x48_0000u32.wrapping_add(0x1730));
        call_ptr(c, 0x0031_7e30, 0x0026_e930)?;
        let self_ = abi::r32(c, S0);
        Addr(self_.wrapping_add(0x10)).sw(c, 0);
        Addr(self_.wrapping_add(0xc)).sw(c, c.g[V0][0]);
        Addr(self_.wrapping_add(4)).sw(c, c.g[V0][0]);
        Addr(self_.wrapping_add(8)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Replay.

/// `void Replay_EnableInstantReplayCameras(ReplayCache* self)` (0x26ef30):
/// refresh the camera triggers and step the instant-replay camera index
/// (`(n + 1) % 9`).
pub fn replay_enable_instant_replay_cameras(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x4c0000;
        c.g[A0][0] = sx(0x004c_5830);
        call_ptr(c, 0x0016_d1d8, 0x0026_ef4c)?;
        c.g[V1][0] = Addr(this).off(0x634).lw(c);
        c.g[V0][0] = 9;
        c.cycles += 8;
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
        frame.restore_ra(c, 0);
        c.div(0, V1, V0);
        c.g[A0][0] = c.hi[0];
        Addr(this).off(0x634).sw(c, c.g[A0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Replay.

/// `int ReplayStream_PeekInt(ReplayStream* s)` (0x26ea68): save the read
/// cursor (+0xc), read the next 4 bytes through the stream's vtable slot 0x10
/// into a stack header, restore the cursor and return the 16-bit id.
/// 20 cycles plus the call.
pub fn replay_stream_peek_int(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = 4;
        frame.save(c, 16, 0x30);
        c.g[A1][0] = sx(frame.at(c, 0));
        c.sd(p_2(frame.at(c, 0), 0x10), c.g[RA][0]);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x20);
        let pos = c.lw(p_2(self_, 0xc));
        c.g[S1][0] = pos;
        let vt = c.lw(self_);
        c.g[V1][0] = vt;
        c.g[V0][0] = c.lw(p_2(vt as u32, 0x14));
        let m = VSlot::read(c, vt, 0x10);
        c.g[A0][0] = m.this(sx(self_));
        call_ptr(c, m.func as u32, 0x0026_ea9c)?;
        c.sw(p_2(self_, 0xc), c.g[S1][0]);
        frame.restore(c, 16, 0x30);
        c.g[V0][0] = Addr(frame.at(c, 0)).lh(c);
        frame.restore(c, 17, 0x20);
        c.g[RA][0] = c.ld(p_2(frame.at(c, 0), 0x10));
        frame.close(c);
        Ok(())
    })
}


/// `void Replay_SaveResumeSnapshot(self)` (0x26f7f8): if the resume block has
/// not been saved yet, record and add an empty frame snapshot.
pub fn replay_save_resume_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(r32(c, S0)).off(0x3d0).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 11;
            c.g[V0][0] = 1;
            c.g[A1][0] = sx(c.g[SP][0] as u32);
            c.sh(c.g[SP][0] as u32, c.g[V0][0]);
            c.sb(p_3(c.g[SP][0] as u32, 2), 0);
            c.g[RA][0] = sx(0x0026_f82c);
            c.sb(p_3(c.g[SP][0] as u32, 3), 0);
            callee::REPLAY_RECORD_SNAPSHOT.call(c, 0x0026_f82c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x0026_f83c);
            Addr(p_3(r32(c, S0), 0x3d0)).sw(c, c.g[V0][0]);
            callee::REPLAY_ADD_SNAPSHOT.call(c, 0x0026_f83c)?;
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}


/// `void Replay_SnapshotEmptyHookA(void)` (0x26ed88): replay snapshot hook, not
/// installed.
pub fn replay_snapshot_empty_hook_a(c: &mut Cpu) {
    c.cycles += 2;
    abi::ret(c);
}


/// `void Replay_FreeBuffers(Replay* self)` (0x26f428): free every replay
/// stream buffer and clear both snapshot lists.
pub fn replay_free_buffers(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(at_2(c, S2, 8)).lw(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0;
        if !empty {
            c.cycles += 1;
            c.g[S0][0] = sx(at_2(c, S2, 0x2c));
            loop {
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                call_ptr(c, 0x0026_e968, 0x0026_f45c)?;
                c.g[V0][0] = Addr(at_2(c, S2, 8)).lw(c);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x38));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_f450);
                }
            }
        }
        c.cycles += 10;
        c.g[A0][0] = sx(at_2(c, S2, 0x3b0));
        call_ptr(c, 0x0026_e5c0, 0x0026_f474)?;
        c.g[A0][0] = sx(at_2(c, S2, 0x3bc));
        call_ptr(c, 0x0026_e5c0, 0x0026_f47c)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int ReplayStream_Decompress(ReplayStream* self, u8** pp, u8* src)`
/// (0x26eab8): decode one compressed frame: header, body, then rewind the
/// read pointer.
pub fn replay_stream_decompress(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = 4;
        call_ptr(c, 0x003e_6574, 0x0026_eae0)?;
        c.g[A1][0] = Addr(r32(c, S0)).lw(c);
        c.g[A0][0] = c.g[SP][0] | 4;
        c.g[A2][0] = 4;
        call_ptr(c, 0x003e_6574, 0x0026_eaf0)?;
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        c.g[A1][0] = 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(4));
        Addr(r32(c, S0)).sw(c, c.g[A0][0]);
        c.g[A2][0] = Addr(p_4(r32(c, S1), 4)).lw(c);
        call_ptr(c, 0x002c_85a0, 0x0026_eb08)?;
        c.g[V1][0] = Addr(p_4(r32(c, SP), 4)).lw(c);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = Addr(r32(c, S0)).lw(c);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V1)));
        Addr(r32(c, S0)).sw(c, c.g[A1][0]);
        c.g[V1][0] = Addr(p_4(r32(c, S1), 4)).lw(c);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
        Addr(p_4(r32(c, S1), 8)).sw(c, c.g[V1][0]);
        callee::REPLAY_STREAM_REWIND_READ.call(c, 0x0026_eb2c)?;
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = 1;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}


/// `void Replay_ApplyCamera(void* self)` (0x26edd8): apply the recorded
/// camera index to every rider in the world's rider list.
pub fn replay_apply_camera(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p_4(r32(c, V1), 0x84)).lw(c);
        c.g[V1][0] = Addr(p_4(r32(c, V0), 0x84)).lw(c);
        c.g[A0][0] = Addr(r32(c, V1)).lw(c);
        let empty = (c.g[A0][0] as i64) <= 0;
        c.g[V0][0] = 0x0044_0000;
        if !empty {
            c.cycles += 5;
            c.g[S1][0] = sx(p_4(r32(c, V1), 4));
            c.g[S3][0] = sx(r32(c, V0).wrapping_add(0x5438));
            c.g[S0][0] = c.g[A0][0];
            c.g[V0][0] = Addr(p_4(r32(c, S2), 0x630)).lw(c);
            loop {
                c.cycles += 10;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
                c.g[V1][0] = Addr(r32(c, S1)).lw(c);
                c.g[V0][0] = sx(r32(c, V0) << 2);
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(4));
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S3)));
                c.g[A0][0] = Addr(p_4(r32(c, V1), 0xa8)).lw(c);
                c.g[A1][0] = Addr(r32(c, V0)).lw(c);
                call_ptr(c, 0x0016_1fa0, 0x0026_ee40)?;
                if c.g[S0][0] == 0 {
                    break;
                }
                c.g[V0][0] = Addr(p_4(r32(c, S2), 0x630)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0026_ee20);
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void Replay_RestoreGameCameras(Replay* self)` (0x26eea0): after restoring
/// the in-game camera triggers, hand every rider's camera back to it and
/// clear the replay's camera-lock flag.
pub fn replay_restore_game_cameras(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        let self_ = abi::arg(c, 0);
        frame.save(c, S3, 0x10);
        c.g[S3][0] = sx(self_);
        frame.save(c, S0, 0x40);
        c.g[A0][0] = 0x004c_0000;
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(p_5(r32(c, A0), 0x5830));
        call_ptr(
            c,
            callee::CAMERA_TRIGGER_MAN_SET_IN_GAME_TRIGGERS,
            0x0026_eec8,
        )?;
        c.g[V0][0] = c.lw(gp_2(c, -0x848));
        c.g[V1][0] = c.lw(at(c, V0, 0x84));
        c.g[V0][0] = c.lw(at(c, V1, 0x84));
        c.g[V1][0] = c.lw(r32(c, V0));
        let none = (c.g[V1][0] as i64) <= 0;
        c.g[S2][0] = sx(p_5(r32(c, V0), 4));
        if !none {
            c.cycles += 2;
            c.g[S1][0] = c.g[V1][0];
            c.g[V0][0] = c.lw(r32(c, S2));
            loop {
                c.cycles += 10;
                c.g[A1][0] = 2;
                c.g[S2][0] = sx(r32(c, S2).wrapping_add(4));
                c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
                c.g[A0][0] = c.g[V0][0];
                c.g[S0][0] = c.lw(at(c, V0, 0xa8));
                call_ptr(c, callee::RIDER_SET_INPUT_ENABLED, 0x0026_ef00)?;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, callee::CAMERA_SET_OWNER, 0x0026_ef08)?;
                if c.g[S1][0] == 0 {
                    break;
                }
                c.g[V0][0] = c.lw(r32(c, S2));
                if event_due(c) {
                    deliver_event_here(c, 0x0026_eee8);
                }
            }
        }
        c.cycles += 8;
        c.sw(at(c, S3, 0x630), 0);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void Replay_Init(self)` (0x26f180): size the replay buffer, allocate its
/// blocks, then reset the replay stream.
pub fn replay_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        c.g[V1][0] = 0x1_0000;
        c.g[A1][0] = 0xc_0000;
        c.g[V1][0] = c.g[V1][0] | 0x4000;
        c.g[A1][0] = c.g[A1][0] | 0x9800;
        c.div(0, 5, 3);
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[V0][0] = 0x1_0000;
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[V0][0] = c.g[V0][0] | 0x4000;
        frame.save_ra(c, 0);
        c.g[S2][0] = 0;
        Addr(p_6(r32(c, S0), 0x14)).sw(c, c.g[V1][0]);
        c.g[S1][0] = sx(p_6(r32(c, S0), 0x2c));
        c.cycles += 4;
        Addr(p_6(r32(c, S0), 0x18)).sw(c, 0);
        c.g[A1][0] = c.lo[0];
        Addr(p_6(r32(c, S0), 8)).sw(c, c.g[A1][0]);
        c.g[A1][0] = Addr(p_6(r32(c, S0), 0x14)).lw(c);
        loop {
            c.cycles += 8;
            c.g[A0][0] = c.g[S1][0];
            c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
            c.g[S1][0] = sx(p_6(r32(c, S1), 0x38));
            call_ptr(c, 0x0026_e908, 0x0026f1e8)?;
            c.g[V0][0] = Addr(p_6(r32(c, S0), 8)).lw(c);
            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[A1][0] = Addr(p_6(r32(c, S0), 0x14)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0026_f1d8);
                }
                continue;
            }
            c.cycles += 11;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0xa;
            call_ptr(c, 0x0026_f4a0, 0x0026f204)?;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0026_f228, 0x0026f20c)?;
            break;
        }
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// `ReplayStream_ReadObjectRef` (0x0026eb48).
//
// `void* ReplayStream::ReadObjectRef(self)`: read an object reference from
// the stream and resolve it against the current layer's object table.

pub fn replay_stream_read_object_ref(c: &mut Cpu) {
    guest(c, |c| {

        'b_26ebf8: {
        'b_26ebf4: {
        c.cycles += 18;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.g[V0][0] = 0xffffffffffff0000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0x4;
        c.g[S0][0] = 0u64;
        c.g[A3][0] = c.lw((c.g[A0][0] as u32));
        c.sw((c.g[SP][0] as u32), c.g[V0][0]);
        c.g[V0][0] = c.lh((c.g[A3][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x14));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0026eb80);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
        call_ptr(c, t, 0x0026eb80)?;
        c.g[A1][0] = c.lw((c.g[SP][0] as u32));
        c.g[V0][0] = !(0u64 | c.g[A1][0]);
        let t = c.g[V0][0] == 0u64;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        if t {
        break 'b_26ebf4;
        } else {
        c.cycles += 6;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
        c.g[RA][0] = sx(0x0026eb9c);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x88));
        call_ptr(c, 0x003512c0, 0x0026eb9c)?;
        c.g[S0][0] = c.g[V0][0];
        let t = c.g[S0][0] != 0u64;
        c.g[V0][0] = c.g[S0][0];
        if t {
        break 'b_26ebf8;
        } else {
        'b_26ebf0: {
        'b_26ebec: {
        c.cycles += 10;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x16c8));
        c.g[A1][0] = c.lw((c.g[SP][0] as u32));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32));
        c.g[V0][0] = c.g[A1][0] & 0xff;
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
        let t = c.g[V1][0] == 0u64;
        c.g[V0][0] = sx((c.g[A1][0] as u32) >> 8);
        if t {
        break 'b_26ebec;
        } else {
        c.cycles += 7;
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32));
        c.g[V0][0] = sx((c.g[V1][0] as u32) >> 8);
        let t = c.g[V0][0] != 0u64;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        if t {
        break 'b_26ebf0;
        } else {
        break 'b_26ebec;
        }
        }
        }
        c.cycles += 1;
        c.g[V0][0] = 0u64;
        break 'b_26ebf0;
        }
        c.cycles += 1;
        c.g[S0][0] = c.g[V0][0];
        break 'b_26ebf4;
        }
        }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[S0][0];
        break 'b_26ebf8;
        }
        c.cycles += 4;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t; return Ok(());

        Ok(())
    })
}


/// `Replay_dtor(self, flags)` (0x26f088): tear a replay object down: end
/// playback, drop the reset hook, free the buffers, run the destructors of
/// the embedded objects, and free the object itself when `flags & 1`.
pub fn replay_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        let this = abi::arg(c, 0);
        let flags = abi::arg(c, 1);
        c.g[V1][0] = 0;
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        frame.save(c, 18, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, 19, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, c.g[S1][0] as u32);
        let t = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[A1][0];
        if !t {
            c.cycles += 1;
            c.g[V1][0] = ((c.g[V0][0] as i64) < 10) as u64;
        }
        c.cycles += 2;
        if c.g[V1][0] != 0 {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0026_f0c8);
            c.g[A0][0] = c.g[S1][0];
            c.pc = 0x0026_f980;
            ee::s07::Replay_EndPlayback(c);
            if c.pc != 0x0026_f0c8 {
                return Err(Unwound);
            }
        }
        c.g[A0][0] = c.g[S1][0];
        c.cycles += 8;
        c.g[RA][0] = sx(0x0026_f0d4);
        c.g[S3][0] = c.g[S0][0] & 1;
        c.pc = 0x0026_f498;
        ee::s07::Replay_EmptyResetHook(c);
        if c.pc != 0x0026_f0d4 {
            return Err(Unwound);
        }
        c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2c));
        c.g[RA][0] = sx(0x0026_f0e0);
        c.g[A0][0] = c.g[S1][0];
        c.pc = 0x0026_f428;
        ee::s07::Replay_FreeBuffers(c);
        if c.pc != 0x0026_f0e0 {
            return Err(Unwound);
        }
        c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x494));
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x5fc));
        if c.g[V1][0] != c.g[V0][0] {
            c.cycles += 2;
            loop {
                c.cycles += 7;
                if c.g[V1][0] == c.g[V0][0] {
                    break;
                }
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xb4));
                if event_due(c) {
                    deliver_event_here(c, 0x0026_f100);
                }
            }
        }
        c.cycles += 2;
        let t = c.g[S2][0] == 0;
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x3ac));
        if !t {
            c.cycles += 2;
            let t = c.g[S2][0] == c.g[S0][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x38));
            if !t {
                c.cycles += 1;
                loop {
                    c.cycles += 8;
                    c.g[A1][0] = 0;
                    c.g[V0][0] = lw(c, c.g[S0][0] as u32);
                    c.g[A0][0] = c.lh(p_8(c.g[V0][0] as u32, 0x38));
                    c.g[V1][0] = lw(c, p_8(c.g[V0][0] as u32, 0x3c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x0026_f148);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.pc = t;
                    ssx3_ee_rt::funcs::call_indirect(c, t);
                    if c.pc != 0x0026_f148 {
                        return Err(Unwound);
                    }
                    let t = c.g[S2][0] != c.g[S0][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x38));
                    if !t {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0026_f130);
                    }
                }
            }
        }
        c.cycles += 2;
        let t = c.g[S3][0] == 0;
        frame.restore(c, 16, 0x40);
        if !t {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e50, 0x0026_f160)?; // operator delete(this)
            frame.restore(c, 16, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
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
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::put_words;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::put_vec;
    use crate::difftest::fakes;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::elf_image;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::SP as STACK;
    use crate::difftest::SP as STACK_TOP;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;
    use crate::difftest::fakes::ptr;
    pub(crate) use crate::game::input::inputmap::tests::put_f32;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS;
    pub(crate) use crate::game::fe::festatecharequipdetail::tests::world;
    pub(crate) use crate::game::dirtysock::tags2::tests::zero;
    pub(crate) use crate::game::fe::festaterewards2::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::fe::festaterewards2::tests::w;
    pub(crate) use crate::game::fe::festaterewards2::tests::belib;
    pub(crate) use crate::game::dirtysock::tags2::tests::be_library_2 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::run;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_3 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::call_machine;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::dirtysock::tags2::tests::gp_at;
    pub(crate) use crate::game::dirtysock::tags2::tests::base;
    pub(crate) use crate::game::dirtysock::tags2::tests::put_vtable;
    pub(crate) use crate::game::dirtysock::tags2::tests::SEEDS_7 as SEEDS_4;
    pub(crate) use crate::game::dirtysock::tags2::tests::load;
    pub(crate) use crate::game::dirtysock::tags2::tests::alloc_ready;
    pub(crate) use crate::game::dirtysock::tags2::tests::dummy_world;
    pub(crate) use crate::game::dirtysock::tags2::tests::common;
    pub(crate) use crate::game::ai::ai::tests::rng;
    pub(crate) use crate::game::ai::ai::tests::leaf_setup;
    pub(crate) use crate::game::ai::ai::tests::leaf;
    pub(crate) use crate::game::fe::feasyncfile::tests::W;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_2 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::BE as BE_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::WORLD;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base as base_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::w32;
    pub(crate) use crate::game::fe::feasyncfile::tests::gpat;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_8 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_4 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_5 as world_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_3 as OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_8 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict;


    #[test]
    fn replay_stream_set_buffer_matches() {
        check(ssx3_ee::s07::f_0026e950, replay_stream_set_buffer, 100, &|c, seed| {
            let mut r = Rng(0x0c13_0012 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            for i in 0..5 {
                c.bus.write32(DATA + i * 4, r.next_u32());
            }
        });
    }


    #[test]
    fn replay_stream_rewind_read_matches() {
        check(ssx3_ee::s07::f_0026e9b0, replay_stream_rewind_read, 50, &|c, seed| {
            let mut r = Rng(0x0c13_0013 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA + 4, r.next_u32());
            c.bus.write32(DATA + 0xc, r.next_u32());
        });
    }


    #[test]
    fn replay_stream_write_object_ref_matches() {
        covered(ssx3_ee::s07::f_0026ecd8__generated, replay_stream_write_object_ref, 150, &|c, seed| {
            let mut r = machine(c, seed, 41, 1);
            let out = DATA + 0x100;
            let vt = vtable(c, &mut r, &[0x8]);
            c.bus.write32(out, vt);
            if seed % 2 == 0 {
                let obj = DATA + 0x400;
                c.bus.write32(obj + 0x78, r.next_u32());
                c.g[A1][0] = sx(obj);
            } else {
                c.g[A1][0] = 0;
            }
            c.g[A0][0] = sx(out);
        });
    }


    #[test]
    fn replay_stream_write_node_ref_matches() {
        covered(
            ssx3_ee::s07::f_0026ed28__generated,
            replay_stream_write_node_ref,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x26ed28, 1);
                let out = DATA;
                let node = DATA + 0x400;
                c.g[A0][0] = out as u64;
                c.g[A1][0] = node as u64;
                let vt = vtable(c, &mut r, &[8]);
                c.bus.write32(out, vt);
                let target = if seed % 3 == 0 { 0 } else { DATA + 0x800 };
                c.bus.write32(node + 0x18, target);
                put_f32(c, target + 0x78, r.next_u32());
                c.bus.write16(target + 8, r.next_u32() as u16);
            },
        );
    }


    // ------------------------------------------------------------------
    // Replay.

    #[test]
    fn replay_enable_replay_cameras_matches() {
        check(ssx3_ee::s07::f_0026ee68__generated, replay_enable_replay_cameras, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x26ee68, 0);
            world(c);
            // No active triggers (`+4 <= 0`) and no riders (`*riders <= 0`).
            c.bus.write32(0x004c_5834, 0);
            c.bus.write32(DATA + 0x7000, 0);
            fill(c, &mut r, DATA + 0x300, 0x20);
            c.g[A0][0] = sx(DATA + 0x300);
        });
    }


    #[test]
    fn replay_restore_instant_replay_cameras_matches() {
        check(
            ee::s07::f_0026ef80__generated,
            replay_restore_instant_replay_cameras,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x26ef_80, 0);
                // No in-game camera triggers: CameraTriggerMan_UpdateTriggers exits.
                c.bus.write32(0x004c_5830 + 4, 0);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x1000);
                zero(c, DATA + 0x1000, 0x400);
                c.bus.write32(DATA + 0x1000 + 0x84, DATA + 0x1100);
                c.bus.write32(DATA + 0x1100 + 0x84, DATA + 0x1200);
                c.bus.write32(DATA + 0x1200 + 4, DATA + 0x1300);
                zero(c, DATA + 0x1300, 0x100);
            },
        );
    }


    // ------------------------------------------------------------- replay

    #[test]
    fn world_state_stream_ctor_matches() {
        check(ssx3_ee::s07::f_0026d5e8__generated, world_state_stream_ctor, 60, &|c, seed| {
            machine(c, seed, 0x26d5_e8, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
        });
    }


    #[test]
    fn replay_next_camera_matches() {
        check_lohi(ssx3_ee::s07::f_0026ed98__generated, replay_next_camera, SEEDS_2, &|c, seed| {
            machine(c, seed, 0x26ed_98, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            w(c, DATA + 0x630, seed % 9);
        });
    }


    #[test]
    fn replay_advance_from_state10_matches() {
        check(ssx3_ee::s07::f_0026f7b8__generated, replay_advance_from_state10, SEEDS_2, &|c, seed| {
            machine(c, seed, 0x26f7_b8, 1);
            belib(c);
            c.g[A0][0] = DATA as u64;
            let state = match seed % 3 {
                0 => 0x0a,
                1 => 0x0e,
                _ => 3,
            };
            w(c, DATA, state);
        });
    }


    

    #[test]
    fn replay_stream_free_buffer_matches() {
        check(
            ssx3_ee::s07::f_0026e968__generated,
            replay_stream_free_buffer,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x26e968, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                c.bus
                    .write32(this_ + 4, if seed % 2 == 0 { 0 } else { DATA + 0x2000 });
                c.g[A0][0] = sx(this_);
            },
        );
    }


    #[test]
    fn replay_stream_read_matches() {
        check(
            ssx3_ee::s07::f_0026ea20__generated,
            replay_stream_read,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x26ea20, 0);
                be_library(c);
                let this_ = DATA + 0x100;
                c.bus.write32(this_ + 0x0c, 0x100);
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = sx(DATA + 0x2000);
                c.g[A2][0] = (seed % 0x40) as u64;
            },
        );
    }


    #[test]
    fn replay_stream_alloc_buffer_matches() {
        check(
            ee::s07::f_0026e908__generated,
            replay_stream_alloc_buffer,
            60,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x26e9_08, 1);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(0x40 + (seed as u32 % 16) * 4);
                let _ = r.next_u32();
            },
        );
    }


    // ------------------------------------------------------------------
    // Replay.

    #[test]
    fn replay_enable_instant_replay_cameras_matches() {
        check(
            ee::s07::f_0026ef30__generated,
            replay_enable_instant_replay_cameras,
            SEEDS_3,
            &|c, seed| {
                let _ = machine(c, seed, 0x26ef_30, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x200);
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x634, [0, 8, 9, 0xffff_ffff][(seed % 4) as usize]);
                // The camera trigger manager reads this global.
                clear(c, 0x004c_5830, 0x40);
            },
        );
    }


    // --------------------------------------------------------------- replay

    #[test]
    fn replay_stream_peek_int_matches() {
        check(ee::s07::f_0026ea68__generated, replay_stream_peek_int, 60, &|c, seed| {
            let mut r = base(c, seed, 0x26ea);
            let s = DATA;
            let vt = put_vtable(c, &mut r, DATA + 0x2000, &[0x10]);
            c.bus.write32(s, vt);
            c.bus.write32(s + 0xc, r.next_u32());
            fill(c, &mut r, DATA + 0x40, 16);
            c.g[A0][0] = sx(s);
            c.g[A2][0] = 4;
        });
    }


    #[test]
    fn replay_save_resume_snapshot_matches() {
        check(
            ee::s07::f_0026f7f8__generated,
            replay_save_resume_snapshot,
            SEEDS_4,
            &|c, seed| {
                let _ = common(c, seed, 0x26f7f8, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x800);
                c.bus
                    .write32(this + 0x3d0, if seed % 3 == 0 { 0 } else { DATA + 0x3000 });
                c.g[A0][0] = sx(this);
            },
        );
    }


    #[test]
    fn replay_snapshot_empty_hook_a_matches() {
        leaf(ssx3_ee::s07::f_0026ed88__generated, replay_snapshot_empty_hook_a, 10);
    }


    #[test]
    fn replay_free_buffers_matches() {
        check(ee::s07::f_0026f428__generated, replay_free_buffers, 60, &|c, seed| {
            let mut r = base_2(c, seed, 0x26f4_28);
            let count = seed % 4;
            c.g[A0][0] = sx(DATA);
            c.bus.write32(DATA + 8, count);
            for i in 0..count {
                let e = DATA + 0x2c + i * 0x38;
                c.bus.write32(e + 4, 0);
                fill(c, &mut r, e, 0xe);
            }
            fill(c, &mut r, DATA + 0x3b0, 3);
            fill(c, &mut r, DATA + 0x3bc, 3);
        });
    }


        #[test]
    fn replay_apply_camera_matches() {
        check(ee::s07::f_0026edd8__generated, replay_apply_camera, 100, &|c, seed| {
            let _ = base_3(c, seed, 0x26_edd8);
            globals(c);
            const LIST: u32 = DATA + 0x4000;
            let n = (seed % 3) as u32;
            // world -> +0x84 (game) -> +0x84 (rider list)
            let game = WORLD + 0x1000;
            w(c, WORLD + 0x84, game);
            w(c, game + 0x84, LIST);
            w(c, LIST, n);
            for i in 0..3u32 {
                let rider = DATA + 0x5000 + 0x100 * i;
                w(c, LIST + 4 + 4 * i, rider);
                w(c, rider + 0xa8, DATA + 0x6000 + 0x100 * i);
            }
            w(c, 0x0044_5438, 0);
            w(c, DATA + 0x630, [0, 1, 2][seed as usize % 3]);
            c.g[A0][0] = sx(DATA);
        });
    }


    #[test]
    fn replay_restore_game_cameras_matches() {
        check(
            ee::s07::f_0026eea0__generated,
            replay_restore_game_cameras,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x26ee, 0);
                env(c);
                let this = DATA + 0x400;
                let list = DATA + 0x2_4000;
                let n = (seed % 3) as u32;
                w32(c, list, n);
                for i in 0..n {
                    let rider = DATA + 0x3000 + 0x100 * i;
                    w32(c, DATA + 0x2_4000 + 4 + 4 * i, rider);
                    w32(c, rider + 0xa8, DATA + 0x2800 + 0x100 * i);
                }
                c.g[A0][0] = sx(this);
            },
        );
    }


    #[test]
    fn replay_init_matches() {
        check(
            ee::s07::f_0026f180__generated,
            replay_init,
            4,
            &|c, seed| {
                let _ = base_4(c, seed, 0x26f180, 4);
                c.g[A0][0] = OBJ as u64;
                fill(c, &mut Rng(seed as u64 ^ 0x26f1), OBJ, 0x120 / 4);
            },
        );
    }


    #[test]
    fn replay_stream_read_object_ref_matches() {
        strict(ee::s07::f_0026eb48__generated, replay_stream_read_object_ref, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x26eb, 8);
            let this = DATA;
            let obj = DATA + 0x1000;
            c.bus.write32(this, obj);
            c.bus.write16(obj + 0x10, 0);
            c.bus.write32(obj + 0x14, FAKE);
            c.bus.write32(GAME_GP - 0x848, DATA + 0x2000);
            c.bus.write32(DATA + 0x2000 + 0x84, DATA + 0x3000);
            c.bus.write32(DATA + 0x3000 + 0x88, FAKE);
            c.bus.write32(GAME_GP + 0x16c8, DATA + 0x4000);
            c.bus.write32(DATA + 0x4000, DATA + 0x5000);
            c.bus.write32(DATA + 0x5000 + 8, DATA + 0x6000);
            c.g[A0][0] = sx(this);
            c.g[A2][0] = sx(r.next_u32() & 0xff);
        });
    }
}
