//! `src/replay/playbackman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::addu;
use crate::abi::r32;
use crate::abi::reg;
use crate::abi::ret;
use crate::abi::set_fcc;
use crate::abi::set_fpr;
use crate::abi::set_reg;
use crate::call;
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
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_3 as lw;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::p as p_4;
pub(crate) use crate::game::fe::feasyncfile::p_5 as p;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::fereal::open;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p_2;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::misc::add32;
pub(crate) use crate::game::movie::movieplayer::at;
pub(crate) use crate::game::movie::movieplayer::lw_at;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::scripter::ssxscriptengine::p as p_3;
pub(crate) use crate::game::sound::soundsys::p as p_5;
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
    (0x0026_ca90, playback_man_reset, "PlaybackMan_Reset"),
    (
        0x0026_cbb8,
        playback_man_save_world_state,
        "PlaybackMan_SaveWorldState",
    ),
    (
        0x0026_cc18,
        playback_man_restore_world_state,
        "PlaybackMan_RestoreWorldState",
    ),
    (
        0x0026_cc48,
        playbackman_start_playback,
        "PlaybackMan_StartPlayback",
    ),
    (
        0x0026_cd20,
        playback_man_stop_playback,
        "PlaybackMan_StopPlayback",
    ),
    (0x0026_d0a0, replay_cache_ctor, "ReplayCache_ctor"),
    (0x0026_d0e8, replay_cache_dtor, "ReplayCache_dtor"),
    (0x0026_d130, replay_cache_free, "ReplayCache_Free"),
    (0x0026_d168, replay_cache_reset, "ReplayCache_Reset"),
    (
        0x0026_d228,
        replay_cache_push_frame,
        "ReplayCache_PushFrame",
    ),
    (
        0x0026_d420,
        replay_cache_read_frame,
        "ReplayCache_ReadFrame",
    ),
    (0x0026_d4d8, run_list_seek, "RunList_Seek"),
    (
        0x0026_d730,
        replay_snapshot_set_highlight_value,
        "Replay_SnapshotSetHighlightValue",
    ),
    (0x0026_d7d0, world_state_save, "WorldState_Save"),
    (
        0x0026_ce50,
        playback_man_read_input_frame,
        "PlaybackMan_ReadInputFrame",
    ),
    (
        0x0026_c5f8,
        playback_man_poll_file_load,
        "PlaybackMan_PollFileLoad",
    ),
    (
        0x0026_d2b0,
        replay_cache_record_frame,
        "ReplayCache_RecordFrame",
    ),
    (0x0026_d178, replay_cache_update, "ReplayCache_Update"),
    (
        0x0026_da88,
        world_state_write_dead_objects,
        "WorldState_WriteDeadObjects",
    ),
];

// ---------------------------------------------------------------------------
// Intra-shard callees the generator called directly (its `jal` aliases).

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
    pub(crate) use crate::game::bx::bxstring2::callee::GAME_MODULE_REQUEST_STATE;
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
    pub(crate) use crate::game::dirtysock::tags2::callee::BENEW_PLAYER_INT_IS_MESSAGE_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_INTERFACE_RIDER;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE as BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE_2 as BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE as BE_LIBRARY_GET_INTERFACE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE_2 as BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_MESSAGE_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOUNDED_CAMERA_ALGORITHM_BASE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE as BUILTIN_DELETE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE_2 as BUILTIN_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS;
    pub(crate) use crate::game::dirtysock::tags2::callee::CSETUP_MAN_SEND_PACKET;
    pub(crate) use crate::game::dirtysock::tags2::callee::CVOIP_MAN_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::CW_SCRIPT_CACHE_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::DIRTY_SOCK_CREATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::DIRTY_SOCK_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::EATIMER_REMOVE_CALLBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::FEMODULE_SET_PREVIEW_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_CAMERA_SET_CONTROLLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_CHANNEL_RELEASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_HEAP_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_API_ADD_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_LISTS;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_LOBBY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_PEER_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_FORWARD_EVENT_TO_UI;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_REQUEST_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_REQUEST_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MENU_ON_LOBBY_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOC_STRING_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::LUNO_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_REQUEST_CARD_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_CTOR_2 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_ON_CHILD_EVENT_2 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_GAME_UTIL_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_CACHE_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_END_PLAYBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_FREE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_HIGHLIGHT_SLOT_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RESTORE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SAVE_EMIT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SNAPSHOT_EMPTY_HOOK_B;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_STREAM_WRITE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REWARDS_FILE_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ANIM_BANK_MAN_FIND_BANK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_CANCEL_BANK_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_FIND_BANK_SLOT_BY_ID;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_IS_BANK_SLOT_LOADED;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS_2 as SCRIPT_ENGINE_POLL_ASYNC_LOADS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_FIELD_B0;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_CURRENT_SKIPPABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MAN_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MAN_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_IS_EVENT_LOADED;
    pub(crate) use crate::game::dirtysock::tags2::callee::SHARED_LOGIN_MAN_SHUTDOWN;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_REMOVE_TICK_CALLBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_APP_START_FE_MODULE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_ARCADE_SAY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_HANDLE_MUSIC_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_PLAY_CHARACTER_TUNE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_LIST_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_SOUND_BASE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_2 as UI_STATE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR_2 as UI_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_CLOSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_MAN_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_RPC_SET_PARAMS;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_RPC_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::WORLD_VIEW_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_RENDER_FADE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_TRICK_METER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUTOSAVE_UPDATE_ICON;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_PAUSE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SCORE_INT_GET_HIGH_SCORE_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_FILE_CACHE_SHUTDOWN;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_ALLOC_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHAT_ROOM_REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_LOCALE_LOAD_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_ANIM_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_RIDER_PREVIEWS;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_RAIL_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_EXPECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_GET_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_READ_QUOTED_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_PAUSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MENU_HIDE_ROOM_TABS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_UPDATE_MODE1;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_UPDATE_STATES;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCARD_INSTALL_CALLBACKS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_IS_DISABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_ITEM_IS_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MOD_OBJ_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_ACCOUNT_MEM_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_DISCONNECT_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_4 as NET_FE_STATE_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_9 as NET_FE_STATE_CTOR_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_4 as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::OV_MESSAGE_CENTER_REFRESH_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::OV_MESSAGE_CENTER_UPDATE_COUNT_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::PLAYBACK_RESTORE_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_CACHE_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SAY_20D3;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SAY_20D4;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_IS_FULL;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_ACTIVATE_BINDING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_ALLOC_PROCESS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_INSTANCE_2 as SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPLINE_MODIFIER_SET_SPEED;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_CTOR_2 as UI_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CHUNK_RESOLVE_HOOK;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2 as WORLD_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_DTOR_2 as WORLD_OBJ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_RUN_KILL_SCRIPT_2 as WORLD_OBJ_RUN_KILL_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_WRITE;
}

/// `void ReplayCache_Reset(ReplayCache *cache)` (0x26d168): zero the count and
/// the two list pointers.
pub fn replay_cache_reset(c: &mut Cpu) {
    c.cycles += 4;
    let cache = Addr(abi::arg(c, 0));
    cache.off(0x8).sw(c, 0);
    cache.sw(c, 0);
    cache.off(0x4).sw(c, 0);
    abi::ret(c);
}

/// `void ReplayCache_PushFrame(ReplayCache *cache, const void *frame)`
/// (0x26d228): append an 8-byte frame to the ring buffer (base at +0xc, count
/// at +0), tagging the header's low 12 bits, or flag the world when full.
pub fn replay_cache_push_frame(c: &mut Cpu) {
    const FRAME_BUFFER: u32 = 0xc;
    const FRAME_SIZE: u32 = 8;
    const HEADER_MASK: u64 = 0xffff_ffff_ffff_f000;
    const MAX_FRAMES: i64 = 4096;
    const WORLD_VIEWS: u32 = 0x84;
    const VIEW_FLAG: u32 = 0x610;
    const VIEW_FLAG_VALUE: u32 = 0xb4;

    c.cycles += 5;
    let cache = Addr(abi::arg(c, 0));
    let frame = abi::arg(c, 1);
    c.g[A2][0] = c.g[A0][0];
    let count = cache.lw(c);
    c.g[V0][0] = ((count as i64) < MAX_FRAMES) as u64;
    let room = c.g[V0][0] != 0;
    c.g[A3][0] = c.g[A1][0];
    if room {
        c.cycles += 20;
        let base = cache.off(FRAME_BUFFER).lw(c);
        c.g[V1][0] = base;
        let slot = sx(((count as u32) << 3).wrapping_add(base as u32));
        c.g[V0][0] = slot;
        c.g[A1][0] = HEADER_MASK;
        let data = Addr(frame).read_u64_unaligned(c);
        Addr(slot as u32).write_u64_unaligned(c, data);
        let count2 = cache.lw(c);
        let base2 = cache.off(FRAME_BUFFER).lw(c);
        let slot2 = sx(((count2 as u32) << 3).wrapping_add(base2 as u32));
        c.g[A0][0] = slot2;
        let header = Addr(slot2 as u32).lw(c);
        let tagged = (header & c.g[A1][0]) | 1;
        c.g[V0][0] = tagged;
        Addr(slot2 as u32).sw(c, tagged);
        c.g[V1][0] = sx((cache.lw(c) as u32).wrapping_add(1));
        cache.sw(c, c.g[V1][0]);
        abi::ret(c);
        return;
    }
    c.cycles += 8;
    let world = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
    c.g[A0][0] = world;
    c.g[A2][0] = 1;
    c.g[A1][0] = VIEW_FLAG_VALUE as u64;
    let views = Addr(world as u32).off(WORLD_VIEWS).lw(c);
    c.g[V0][0] = views;
    let view = Addr(views as u32).off(0x28).lw(c);
    c.g[V1][0] = view;
    Addr(view as u32).off(VIEW_FLAG).off(0x4).sw(c, c.g[A1][0]);
    Addr(view as u32).off(VIEW_FLAG).sw(c, c.g[A2][0]);
    abi::ret(c);
}

/// `void Replay_SnapshotSetHighlightValue(Replay *replay, int slot, uint value)`
/// (0x26d730): `replay->snapshots[slot].highlight = value`.
pub fn replay_snapshot_set_highlight_value(c: &mut Cpu) {
    const HIGHLIGHT: u32 = 0x20;

    c.cycles += 4;
    c.g[A1][0] = sx(abi::arg(c, 1) << 2);
    c.g[A0][0] = sx(abi::arg(c, 0).wrapping_add(c.g[A1][0] as u32));
    Addr(c.g[A0][0] as u32).off(HIGHLIGHT).sw(c, c.g[A2][0]);
    abi::ret(c);
}

/// `PlaybackMan_RestoreWorldState(this)` 0x26cc18: restore the saved world
/// state when a snapshot is pending at +0x10.
pub fn playback_man_restore_world_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        let pending = lw_at(c, this as u64, 0x10);
        c.g[V0][0] = pending;
        c.g[A1][0] = sx(this.wrapping_add(0x14));
        if pending != 0 {
            c.cycles += 3;
            let stream = lw_at(c, this as u64, 0x5c);
            c.g[V0][0] = stream;
            c.g[A0][0] = lw_at(c, stream, 0x28);
            callee::WORLD_STATE_RESTORE.call(c, 0x0026_cc38)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Replay.

/// `void ReplayCache_Free(ReplayCache* self)` (0x26d130): release the buffer at
/// `+0xc` and clear it.
pub fn replay_cache_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0xc).lw(c);
        if c.g[A0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 4;
            callee::EA_MEM_FREE.call(c, 0x0026_d154)?;
            let s0 = c.g[S0][0];
            Addr(s0 as u32).off(0xc).sw(c, 0);
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Replay cache and stream.

/// `void ReplayCache_dtor(ReplayCache* self, int flags)` (0x26d0e8): free the
/// cache, then `operator delete` it when `flags & 1`.
pub fn replay_cache_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        callee::REPLAY_CACHE_FREE.call(c, 0x0026_d104)?;
        c.g[S0][0] = c.g[S0][0] & 1;
        let keep = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, callee::BUILTIN_DELETE_2, 0x0026_d118)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void Replay_RestoreSnapshot(Replay* self, ReplayFrame* frame)`
/// (0x26db88): mark the frame used, then restore the world state from it.
pub fn replay_restore_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        callee::REPLAY_SNAPSHOT_EMPTY_HOOK_B.call(c, 0x0026_dbac)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        callee::WORLD_STATE_RESTORE.call(c, 0x0026_dbb8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `ReplayCache* ReplayCache_ctor(self)` (0x26d0a0): allocate the 32 KiB
/// replay buffer, store it at `+0xc` and reset the cache.
pub fn replay_cache_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x48_0000;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = sx(0x48_0000u32.wrapping_add(0x15a0));
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A0][0] = 0x8000;
        call_ptr(c, 0x0031_7e30, 0x0026_d0c8)?;
        let self_ = abi::r32(c, S0);
        Addr(self_.wrapping_add(0xc)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S0][0];
        callee::REPLAY_CACHE_RESET.call(c, 0x0026_d0d4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WorldState_Save(self, out, src)` (0x26d7d0): save the world state
/// into `out` (an unaligned 4-byte copy of `*src` at `out+0x1c`, the world
/// state proper, then the replay frame counter from `self+0x3c8` at
/// `out+0x30`).
pub fn world_state_save(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let out = abi::r32(c, S0);
        let src = abi::r32(c, A2);
        c.lwl(2, src.wrapping_add(3));
        c.lwr(2, src);
        c.swl(2, out.wrapping_add(0x1f));
        c.swr(2, out.wrapping_add(0x1c));
        c.g[S1][0] = c.g[A0][0];
        callee::WORLD_STATE_WRITE.call(c, 0x0026_d7fc)?;
        let self_ = abi::r32(c, S1);
        c.g[V0][0] = Addr(self_.wrapping_add(0x3c8)).lw(c);
        frame.restore(c, S1, 0x10);
        Addr(out.wrapping_add(0x30)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* PlaybackMan_SaveWorldState(PlaybackMan* self)` (0x26cbb8): save one
/// world-state snapshot once (the `+0x10` latch).
pub fn playback_man_save_world_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[V0][0] = lw(c, p(c.g[S1][0], 0x10));
        let saved = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x30);
        if !saved {
            c.cycles += 11;
            c.g[S0][0] = 1;
            Addr(p(c.g[SP][0], 2)).sb(c, 0);
            Addr(p(c.g[SP][0], 0)).sh(c, c.g[S0][0]);
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x14));
            Addr(p(c.g[SP][0], 3)).sb(c, 0);
            c.g[A2][0] = c.g[SP][0];
            c.g[V0][0] = lw(c, p(c.g[S1][0], 0x5c));
            c.g[A0][0] = lw(c, p(c.g[V0][0], 0x28));
            call_ptr(c, 0x0026_d7d0, 0x0026_cbfc)?;
            Addr(p(c.g[S1][0], 0x10)).sw(c, c.g[S0][0]);
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `void PlaybackMan_Reset(PlaybackMan* self)` (0x26ca90): stop playback,
/// clear the state words and re-arm the replay buffer header.
pub fn playback_man_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        open(c, 0x30);
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[RA][0] = sx(0x0026_caa8);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0026_cdf8, 0x0026_caa8)?;
        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x70), 0u64);
        let no_buf = c.g[S1][0] == 0;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
        if !no_buf {
            c.cycles += 11;
            c.g[A2][0] = 0x10000;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0u64;
            c.g[RA][0] = sx(0x0026_cacc);
            c.g[A2][0] = c.g[A2][0] | 0xc22c;
            call_ptr(c, 0x003e_6448, 0x0026_cacc)?;
            c.g[V1][0] = 0xffff_ffff_dbac_0000;
            c.g[V0][0] = 0x10000;
            c.g[V1][0] = c.g[V1][0] | 0x101;
            c.g[V0][0] = c.g[V0][0] | 0xc200;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            c.sw((c.g[S1][0] as u32), c.g[V1][0]);
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x30);
        Ok(())
    })
}

/// `PlaybackMan* PlaybackMan_dtor(PlaybackMan* p_2, int flags)` (0x26c588):
/// destroy the owned big file and data, free the two buffers, and optionally
/// free the manager.
pub fn playback_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        sqsp(c, 16, 0x20);
        sqsp(c, 17, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        let no_file = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_file {
            c.cycles += 2;
            c.g[A1][0] = 3;
            call::call_ptr(c, 0x0031_68e8, 0x0026_c5b0)?;
        }
        c.cycles += 11;
        c.g[A0][0] = Addr(p_2(r32(c, S0), 0x6c)).lw(c);
        call::call_ptr(c, 0x0031_7e50, 0x0026_c5b8)?;
        c.g[A0][0] = Addr(p_2(r32(c, S0), 0x68)).lw(c);
        call::call_ptr(c, 0x0031_7e50, 0x0026_c5c0)?;
        c.g[A0][0] = c.g[S0][0];
        call::call_ptr(c, 0x0026_c898, 0x0026_c5c8)?;
        c.g[V0][0] = 0x0048_1898;
        c.g[V1][0] = c.g[S1][0] & 1;
        let no_free = c.g[V1][0] == 0;
        Addr(p_2(r32(c, S0), 0x14)).sw(c, c.g[V0][0]);
        if !no_free {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call::call_ptr(c, 0x0031_7e50, 0x0026_c5e4)?;
        }
        c.cycles += 5;
        lqsp(c, 16, 0x20);
        lqsp(c, 17, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    });
}

/// `func_0026CD20` (0x26cd20): stop playback. When `this+0x70` is 1: stop the
/// two streams at `+0x60`, restore the recorded object states
/// (`Replay_SetState` / `PlaybackMan_RestoreWorldState`), clear `+0x70` and
/// notify the scene at `*(*(this+0x5c)+0x84)`. Signature: `void stop(this)`.
pub fn playback_man_stop_playback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 0x1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw_r(c, S0, 0x70);
        if c.g[V0][0] != c.g[V1][0] {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 18;
            c.g[RA][0] = sx(0x0026_cd48);
            c.g[A0][0] = lw_r(c, S0, 0x60);
            call_ptr(c, 0x0012_af68, 0x0026_cd48)?;
            c.g[RA][0] = sx(0x0026_cd50);
            c.g[A0][0] = lw_r(c, S0, 0x60);
            call_ptr(c, 0x0012_8ac0, 0x0026_cd50)?;
            c.g[V0][0] = lw_r(c, S0, 0x5c);
            c.g[A0][0] = lw_r(c, V0, 0x28);
            c.g[RA][0] = sx(0x0026_cd60);
            c.g[A1][0] = lw_r(c, A0, 0x4);
            callee::REPLAY_SET_STATE.call(c, 0x0026_cd60)?;
            c.g[RA][0] = sx(0x0026_cd68);
            c.g[A0][0] = c.g[S0][0];
            callee::PLAYBACK_RESTORE_WORLD.call(c, 0x0026_cd68)?;
            sw_r(c, S0, 0x70, 0);
            c.g[A0][0] = lw_r(c, S0, 0x5c);
            c.g[V0][0] = lw_r(c, S0, 0x60);
            c.g[V1][0] = lw_r(c, A0, 0x84);
            c.g[A1][0] = lw_r(c, V0, 0x28);
            c.g[RA][0] = sx(0x0026_cd84);
            c.g[A0][0] = lw_r(c, V1, 0x4);
            call_ptr(c, 0x0015_edc8, 0x0026_cd84)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int RunList_Seek(list, pos)` (0x0026d4d8): find the run containing byte
/// `pos`, returning its index and the offset inside it.
pub fn run_list_seek(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A2][0] = Addr(at(c, A0, 0)).lw(c);
    c.g[T0][0] = 0;
    let empty = (c.g[A2][0] as i64) <= 0;
    c.g[A3][0] = Addr(at(c, A0, 0xc)).lw(c);
    if !empty {
        c.cycles += 6;
        c.g[V1][0] = Addr(at(c, A3, 0)).lw(c);
        c.g[T2][0] = c.g[A2][0];
        c.g[A2][0] = sx(word(c, V1) & 0xfff);
        c.g[V0][0] = (c.g[A1][0] < c.g[A2][0]) as u64;
        c.g[A3][0] = sx(word(c, A3).wrapping_add(8));
        // The loop's "ran out of entries" exit breaks all the way out to the
        // `v0 = 0` tail (skipping the +3 block); finding the run falls through.
        let mut exhausted = false;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[T1][0] = c.g[T2][0];
            loop {
                c.cycles += 4;
                c.g[T0][0] = sx(word(c, T0).wrapping_add(1));
                c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[T1][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    exhausted = true;
                    break;
                }
                c.cycles += 6;
                c.g[V0][0] = Addr(at(c, A3, 0)).lw(c);
                c.g[V0][0] = sx(word(c, V0) & 0xfff);
                c.g[A2][0] = sx(word(c, A2).wrapping_add(word(c, V0)));
                c.g[V1][0] = (c.g[A1][0] < c.g[A2][0]) as u64;
                let not_yet = c.g[V1][0] == 0;
                c.g[A3][0] = sx(word(c, A3).wrapping_add(8));
                if not_yet {
                    if event_due(c) {
                        deliver_event_here(c, 0x0026_d508);
                    }
                    continue;
                }
                break;
            }
        }
        if exhausted {
            c.cycles += 2;
            let ra = word(c, RA);
            c.g[V0][0] = 0;
            c.pc = ra;
            return;
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[T2][0] as i64)) as u64;
        let found = c.g[V0][0] != 0;
        c.g[V0][0] = sx(word(c, A2).wrapping_sub(word(c, A1)));
        if found {
            c.cycles += 4;
            Addr(at(c, A0, 0x4)).sw(c, c.g[T0][0]);
            Addr(at(c, A0, 0x8)).sw(c, c.g[V0][0]);
            let ra = word(c, RA);
            c.g[V0][0] = 0x1;
            c.pc = ra;
            return;
        }
    }
    c.cycles += 2;
    let ra = word(c, RA);
    c.g[V0][0] = 0;
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// Replay headers.

/// `int ReplayCache_Deserialize(ReplayCache* self, u8** pp)` (0x26d558): read
/// the entry header, copy the body through the replay codec, advance the
/// stream and return 1.
pub fn replay_cache_deserialize(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x50);
        c.g[A2][0] = 8;
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x20);
        c.g[S1][0] = c.g[A0][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = Addr(r32(c, S0)).lw(c);
        call_ptr(c, 0x003e_6574, 0x0026_d580)?;
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        c.g[A0][0] = sx(p_3(r32(c, SP), 0x10));
        c.g[V1][0] = Addr(p_3(r32(c, SP), 4)).lw(c);
        c.g[A2][0] = 4;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(8));
        Addr(r32(c, S0)).sw(c, c.g[V0][0]);
        Addr(r32(c, S1)).sw(c, c.g[V1][0]);
        c.g[A1][0] = Addr(r32(c, S0)).lw(c);
        call_ptr(c, 0x003e_6574, 0x0026_d5a4)?;
        c.g[A0][0] = Addr(r32(c, S0)).lw(c);
        c.g[A1][0] = 0;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(4));
        Addr(r32(c, S0)).sw(c, c.g[A0][0]);
        c.g[A2][0] = Addr(p_3(r32(c, S1), 0xc)).lw(c);
        call_ptr(c, 0x002c_85a0, 0x0026_d5bc)?;
        c.g[V1][0] = Addr(r32(c, S0)).lw(c);
        c.g[V0][0] = 1;
        c.g[A0][0] = Addr(p_3(r32(c, SP), 0x10)).lw(c);
        frame.restore(c, S1, 0x30);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
        frame.restore_ra(c, 0x20);
        Addr(r32(c, S0)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Replay cache.

/// `int ReplayCache_ReadFrame(ReplayCache* c, void* out)` (0x26d420): pop one
/// recorded frame — a 4-byte word and a 16-byte state block — from the ring at
/// `+0xc`; returns 1 on success and 0 when the cache is empty.
pub fn replay_cache_read_frame(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        'outer: {
            'count: {
                c.g[A2][0] = 8;
                frame.save(c, S0, 0x20);
                frame.save(c, S1, 0x10);
                let cache = r(c, A0);
                c.g[S0][0] = sx(cache);
                c.g[S1][0] = c.g[A1][0];
                frame.save_ra(c, 0);
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x0026_d448);
                c.g[A1][0] = 0;
                call_ptr(c, 0x003e_6448, 0x0026_d448)?;
                c.g[V1][0] = Addr(p_4(r(c, S0), 4)).lw(c);
                c.g[V0][0] = Addr(r(c, S0)).lw(c);
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = Addr(p_4(r(c, S0), 8)).lw(c);
                    break 'count;
                }
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'outer;
            }
            'head: {
                c.cycles += 2;
                if (c.g[V0][0] as i64) > 0 {
                    c.g[V0][0] = Addr(p_4(r(c, S0), 4)).lw(c);
                    break 'head;
                }
                c.cycles += 7;
                c.g[V0][0] = Addr(p_4(r(c, S0), 0xc)).lw(c);
                c.g[V1][0] = sx((r(c, V1) as u32) << 3);
                c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, V0)));
                c.g[A0][0] = Addr(r(c, V1)).lw(c);
                c.g[A0][0] = c.g[A0][0] & 0xfff;
                Addr(p_4(r(c, S0), 8)).sw(c, c.g[A0][0]);
                c.g[V0][0] = Addr(p_4(r(c, S0), 4)).lw(c);
            }
            'copy: {
                c.cycles += 11;
                c.g[V1][0] = Addr(p_4(r(c, S0), 0xc)).lw(c);
                c.g[V0][0] = sx((r(c, V0) as u32) << 3);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                c.ldl(3, p_4(r(c, V0), 7));
                c.ldr(3, r(c, V0));
                c.sdl(3, p_4(r(c, S1), 7));
                c.sdr(3, r(c, S1));
                c.g[V1][0] = Addr(p_4(r(c, S0), 8)).lw(c);
                c.g[V1][0] = sx(r(c, V1).wrapping_sub(1));
                let more = (c.g[V1][0] as i64) > 0;
                Addr(p_4(r(c, S0), 8)).sw(c, c.g[V1][0]);
                if more {
                    break 'copy;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(p_4(r(c, S0), 4)).lw(c);
                c.g[V0][0] = sx(r(c, V0).wrapping_add(1));
                Addr(p_4(r(c, S0), 4)).sw(c, c.g[V0][0]);
            }
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
        c.cycles += 5;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Playback manager.

/// `void PlaybackMan_StartPlayback(PlaybackMan* self, int frame, int a2)`
/// (0x26cc48): if not already playing, purge the replay riders, reset the
/// replay, save the world state and start frame stepping.
pub fn playbackman_start_playback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p_5(r32(c, S0), 0x70)).lw(c);
        let mut done = false;
        if c.g[V0][0] == 0 {
            c.cycles += 4;
            c.g[V0][0] = Addr(p_5(r32(c, S0), 4)).lw(c);
            c.g[V1][0] = Addr(p_5(r32(c, V0), 0xc)).lw(c);
            if (c.g[V1][0] as i64) > 0 {
                c.cycles += 33;
                c.g[A0][0] = Addr(p_5(r32(c, S0), 0x60)).lw(c);
                call_ptr(c, 0x0012_af68, 0x0026_cc7c)?;
                c.g[A0][0] = Addr(p_5(r32(c, S0), 0x60)).lw(c);
                call_ptr(c, 0x0012_8ac0, 0x0026_cc84)?;
                c.g[V1][0] = Addr(p_5(r32(c, S0), 0x5c)).lw(c);
                c.g[A1][0] = 0xe;
                c.g[V0][0] = Addr(p_5(r32(c, V1), 0x28)).lw(c);
                c.g[A2][0] = Addr(r32(c, V0)).lw(c);
                c.g[A0][0] = c.g[V0][0];
                Addr(p_5(r32(c, V0), 4)).sw(c, c.g[A2][0]);
                call_ptr(c, 0x0026_f4a0, 0x0026_cca0)?;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0026_cbb8, 0x0026_cca8)?;
                c.g[V1][0] = Addr(p_5(r32(c, S0), 4)).lw(c);
                c.g[A0][0] = 0x10000;
                c.g[A0][0] = c.g[A0][0] | 0xc214;
                Addr(p_5(r32(c, V1), 8)).sw(c, 0);
                c.g[V0][0] = Addr(p_5(r32(c, S0), 4)).lw(c);
                c.g[A0][0] = sx(p_5(r32(c, V0), r32(c, A0)));
                call_ptr(c, 0x0031_7908, 0x0026_ccc4)?;
                c.g[V1][0] = Addr(p_5(r32(c, S0), 4)).lw(c);
                c.g[V0][0] = 0x10000;
                c.g[V0][0] = c.g[V0][0] | 0x8000;
                c.g[A1][0] = Addr(p_5(r32(c, S0), 0x60)).lw(c);
                c.g[V1][0] = sx(p_5(r32(c, V1), r32(c, V0)));
                c.g[A0][0] = 1;
                c.g[V0][0] = Addr(p_5(r32(c, V1), 0x4210)).lw(c);
                Addr(p_5(r32(c, A1), 8)).sw(c, c.g[V0][0]);
                Addr(p_5(r32(c, S0), 0x70)).sw(c, c.g[A0][0]);
                c.g[V0][0] = Addr(p_5(r32(c, S0), 0x60)).lw(c);
                c.g[V0][0] = Addr(p_5(r32(c, V0), 0x70)).lw(c);
                let no_cur = c.g[V0][0] == 0;
                c.g[A2][0] = 0;
                if !no_cur {
                    c.cycles += 1;
                    c.g[A2][0] = Addr(p_5(r32(c, V0), 0x18)).lw(c);
                }
                c.cycles += 6;
                c.g[V1][0] = Addr(p_5(r32(c, S0), 0x5c)).lw(c);
                c.g[A1][0] = c.g[A2][0];
                c.g[V0][0] = Addr(p_5(r32(c, V1), 0x84)).lw(c);
                c.g[A0][0] = Addr(p_5(r32(c, V0), 4)).lw(c);
                call_ptr(c, 0x0015_edc8, 0x0026_cd10)?;
                done = true;
            }
        }
        if !done {
            // both early exits did `lq s0` and jumped straight to the tail
        }
        c.lq(S0, frame.at(c, 0x10));
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod playback {

    pub const STREAM: u32 = 0x4;

    pub const LIVE_PAD: u32 = 0x64;

    pub const OUT_PAD: u32 = 0x68;

    pub const MODE: u32 = 0x70;

    pub const MODE_PLAY: u64 = 1;

    pub const MODE_RECORD: u64 = 3;

    pub const STREAM_CAP: u32 = 0x4;

    pub const STREAM_POS: u32 = 0x8;

    pub const STREAM_LEN: u32 = 0xc;

    pub const STREAM_BYTES: u64 = 0x10;

    /// `1/255` (byte to button value, `gp-0x4920` and `gp-0x491c`).
    pub const PLAY_SCALE: u32 = 0x4920;

    pub const RECORD_SCALE: u32 = 0x491c;

    pub const BUTTON_SIZE: u64 = 0x1c;

    /// Resume entries (the original's loop heads that can unwind).
    pub const ENTRY: u32 = 0x0026_ce50;

    pub const PLAY_LOOP: u32 = 0x0026_cea0;

    pub const RECORD_LOOP: u32 = 0x0026_cf48;

    pub const FEED: u32 = 0x0026_cee0;
}

pub(crate) const INPUT_PAD_UPDATE: u32 = 0x0032_1298;

/// Stages of [`playback_man_read_input_frame`] (the original's blocks).
#[derive(Clone, Copy)]
pub(crate) enum ReadFrame {
    PlayLoop,
    PlayDone,
    Feed,
    RecordLoop,
    RecordDone,
    Failed,
    Exit,
}

/// `bool PlaybackMan_ReadInputFrame(PlaybackMan* self)` (0x26ce50): one
/// replay frame of input. Playing back: read `n` and `n` button bytes from
/// the stream, scale them to floats and feed them to the pad. Recording:
/// write the live pad's `n` button values as bytes (`value * 255`), feed
/// the quantised values back the same way and grow the stream. Returns 1
/// when the stream is exhausted/full, else 0 (any other mode: 0).
///
/// The three loop back edges can unwind; the function is re-entered at
/// the loop head (or the feed call) with its state in the registers.
pub fn playback_man_read_input_frame(c: &mut Cpu) {
    use crate::events::back_edge_unwinds;
    use crate::ps2f::FpuF32;
    use playback::*;
    guest(c, |c| {
        let mut stage = match c.pc {
            ENTRY => {
                c.cycles += 7;
                let frame = Frame::open(c, 0x70);
                c.g[T1][0] = c.g[A0][0];
                frame.save_ra(c, 0x60);
                let this = r32(c, T1);
                c.g[V0][0] = 1;
                c.g[V1][0] = Addr(this + MODE).lw(c);
                let mode = c.g[V1][0];
                c.g[V0][0] = MODE_RECORD;
                if mode == MODE_PLAY {
                    c.cycles += 6;
                    c.g[V1][0] = Addr(this + STREAM).lw(c);
                    let stream = r32(c, V1);
                    c.g[A0][0] = Addr(stream + STREAM_POS).lw(c);
                    c.g[V0][0] = Addr(stream + STREAM_LEN).lw(c);
                    let more = (c.g[A0][0] as i64) < (c.g[V0][0] as i64);
                    c.g[V0][0] = add32(c.g[A0][0], STREAM_BYTES);
                    if !more {
                        ReadFrame::Failed
                    } else {
                        c.cycles += 4;
                        c.g[V1][0] = add32(c.g[V1][0], c.g[V0][0]);
                        c.g[A3][0] = Addr(r32(c, V1)).lbu(c);
                        c.g[V1][0] = add32(c.g[V1][0], 1);
                        if c.g[A3][0] == 0 {
                            ReadFrame::PlayDone
                        } else {
                            c.cycles += 3;
                            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(PLAY_SCALE));
                            c.g[A1][0] = c.g[SP][0];
                            c.g[A0][0] = c.g[A3][0];
                            ReadFrame::PlayLoop
                        }
                    }
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    if mode != MODE_RECORD {
                        ReadFrame::Exit
                    } else {
                        c.cycles += 7;
                        c.g[A0][0] = Addr(this + STREAM).lw(c);
                        let stream = r32(c, A0);
                        c.g[V0][0] = Addr(stream + STREAM_CAP).lw(c);
                        c.g[A2][0] = Addr(stream + STREAM_POS).lw(c);
                        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x18));
                        let room = (c.g[A2][0] as i64) < (c.g[V0][0] as i64);
                        c.g[V0][0] = add32(c.g[A2][0], STREAM_BYTES);
                        if !room {
                            ReadFrame::Failed
                        } else {
                            c.cycles += 7;
                            c.g[V1][0] = Addr(this + LIVE_PAD).lw(c);
                            c.g[A2][0] = add32(c.g[A0][0], c.g[V0][0]);
                            c.g[A3][0] = 0;
                            c.g[T0][0] = Addr(r32(c, V1)).lw(c);
                            Addr(r32(c, A2)).sb(c, c.g[T0][0]);
                            c.g[A2][0] = add32(c.g[A2][0], 1);
                            if (c.g[T0][0] as i64) <= 0 {
                                ReadFrame::RecordDone
                            } else {
                                c.cycles += 6;
                                c.g[AT][0] = 0x437f_0000;
                                c.f[2] = 0x437f_0000; // 255.0
                                c.g[A1][0] = c.g[SP][0];
                                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(RECORD_SCALE));
                                c.g[T2][0] = 0;
                                ReadFrame::RecordLoop
                            }
                        }
                    }
                }
            }
            PLAY_LOOP => ReadFrame::PlayLoop,
            RECORD_LOOP => ReadFrame::RecordLoop,
            FEED => ReadFrame::Feed,
            pc => {
                c.jump_out(pc, ENTRY, 0x0026_cfe8);
                return Ok(());
            }
        };
        loop {
            stage = match stage {
                ReadFrame::PlayLoop => loop {
                    // Byte -> float: `value = byte * scale`.
                    c.cycles += 9;
                    c.g[V0][0] = Addr(r32(c, V1)).lbu(c);
                    c.g[A0][0] = sx(r32(c, A0).wrapping_sub(1));
                    c.g[V1][0] = add32(c.g[V1][0], 1);
                    let v = FpuF32::from_i32(c.g[V0][0] as i32) * FpuF32(c.f[1]);
                    c.f[0] = v.0;
                    c.swc1(0, r32(c, A1));
                    let more = c.g[A0][0] != 0;
                    c.g[A1][0] = add32(c.g[A1][0], 4);
                    if !more {
                        break ReadFrame::PlayDone;
                    }
                    if back_edge_unwinds(c, PLAY_LOOP) {
                        return Ok(());
                    }
                },
                ReadFrame::PlayDone => {
                    c.cycles += 7;
                    c.g[V1][0] = Addr(r32(c, T1) + STREAM).lw(c);
                    c.g[A1][0] = c.g[A3][0];
                    c.g[A2][0] = c.g[SP][0];
                    let pos = Addr(r32(c, V1) + STREAM_POS);
                    c.g[V0][0] = pos.lw(c);
                    c.g[V0][0] = add32(add32(c.g[V0][0], 1), c.g[A3][0]);
                    pos.sw(c, c.g[V0][0]);
                    ReadFrame::Feed
                }
                ReadFrame::Feed => {
                    // InputPad_Update(out_pad, n, values)
                    c.cycles += 4;
                    c.g[A0][0] = Addr(r32(c, T1) + OUT_PAD).lw(c);
                    call_ptr(c, INPUT_PAD_UPDATE, 0x0026_cee8)?;
                    c.g[V0][0] = 0;
                    ReadFrame::Exit
                }
                ReadFrame::RecordLoop => loop {
                    // Button value -> byte (`value * 255`, truncated) and back.
                    c.cycles += 5;
                    c.g[V1][0] = Addr(r32(c, T1) + LIVE_PAD).lw(c);
                    c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                    let present = (c.g[A3][0] as i64) < (c.g[V0][0] as i64);
                    c.g[V0][0] = add32(c.g[V1][0], c.g[T2][0]);
                    if present {
                        c.cycles += 2;
                        c.lwc1(0, r32(c, V0) + 4);
                    } else {
                        c.cycles += 1;
                        c.f[0] = 0;
                    }
                    c.cycles += 15;
                    let scaled = FpuF32(c.f[0]) * FpuF32(c.f[2]);
                    c.f[0] = scaled.0;
                    c.g[A3][0] = add32(c.g[A3][0], 1);
                    c.g[T2][0] = add32(c.g[T2][0], BUTTON_SIZE);
                    c.g[A0][0] = ((c.g[A3][0] as i64) < (c.g[T0][0] as i64)) as u64;
                    c.f[3] = scaled.to_i32() as u32;
                    c.g[V0][0] = sx(c.f[3]);
                    Addr(r32(c, A2)).sb(c, c.g[V0][0]);
                    c.g[V1][0] = c.g[V0][0] & 0xff;
                    c.g[A2][0] = add32(c.g[A2][0], 1);
                    let v = FpuF32::from_i32(c.g[V1][0] as i32) * FpuF32(c.f[1]);
                    c.f[0] = v.0;
                    c.swc1(0, r32(c, A1));
                    let more = c.g[A0][0] != 0;
                    c.g[A1][0] = add32(c.g[A1][0], 4);
                    if !more {
                        break ReadFrame::RecordDone;
                    }
                    if back_edge_unwinds(c, RECORD_LOOP) {
                        return Ok(());
                    }
                },
                ReadFrame::RecordDone => {
                    // pos += n + 1; len += n + 1.
                    c.cycles += 13;
                    c.g[A0][0] = Addr(r32(c, T1) + STREAM).lw(c);
                    c.g[A1][0] = c.g[T0][0];
                    c.g[A2][0] = c.g[SP][0];
                    let pos = Addr(r32(c, A0) + STREAM_POS);
                    c.g[V0][0] = add32(add32(pos.lw(c), 1), c.g[T0][0]);
                    pos.sw(c, c.g[V0][0]);
                    c.g[V1][0] = Addr(r32(c, T1) + STREAM).lw(c);
                    let len = Addr(r32(c, V1) + STREAM_LEN);
                    c.g[V0][0] = add32(add32(len.lw(c), 1), c.g[T0][0]);
                    len.sw(c, c.g[V0][0]);
                    if back_edge_unwinds(c, FEED) {
                        return Ok(());
                    }
                    ReadFrame::Feed
                }
                ReadFrame::Failed => {
                    c.cycles += 1;
                    c.g[V0][0] = 1;
                    ReadFrame::Exit
                }
                ReadFrame::Exit => {
                    c.cycles += 3;
                    let frame = Frame { size: 0x70 };
                    frame.restore_ra(c, 0x60);
                    frame.close(c);
                    return Ok(());
                }
            };
        }
    })
}

// ---------------------------------------------------------------------------
// PlaybackMan_PollFileLoad (0x26c5f8)

/// Cross-shard calls through the function table.
pub(crate) const ASYNC_FILE_IS_DONE: u32 = 0x003d_f980;

pub(crate) const ASYNC_FILE_GET_RESULT: u32 = 0x003d_f9d8;

pub(crate) const OPERATOR_NEW_NAMED: u32 = 0x0031_7e30;

pub(crate) const EA_MEM_COPY: u32 = 0x003e_6574;

pub(crate) const MEM_FREE: u32 = 0x0025_23a8;

/// The allocation name passed to `operator_new_named` and its flags.
pub(crate) const PLAYBACK_ALLOC_NAME: u64 = 0x0048_1560;

pub(crate) const PLAYBACK_ALLOC_FLAGS: u64 = 0x2000_0000;

/// PlaybackMan: entry array (+0x58, 12 bytes each: +4 buffer, +8 size),
/// current entry (+0x74) and the pending async file handle (+0x78, -1:
/// none).
pub(crate) const PBM_ENTRIES: u32 = 0x58;

pub(crate) const PBM_CURRENT: u32 = 0x74;

pub(crate) const PBM_HANDLE: u32 = 0x78;

pub(crate) const PBE_SIZE: u64 = 0xc;

pub(crate) const PBE_BUFFER: u32 = 0x4;

pub(crate) const PBE_BYTES: u32 = 0x8;

/// `void PlaybackMan_PollFileLoad(PlaybackMan* self)` (0x26c5f8): when the
/// pending async load (`+0x78`) has finished, takes its result
/// (`AsyncFile_GetResult(handle, &data, &size)`); if it holds data, copies
/// it into a new buffer (`operator_new_named(size, ..)`) of the current
/// entry (`entries[cur]`: buffer, size) and frees the loaded block. The
/// handle is then cleared (-1).
///
/// s2 = self, s0 = the entry, s1 = 12 (the entry size; LO/HI from the
/// index products), `data`/`size` on the stack at 0/4.
pub fn playback_man_poll_file_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S0, 0x40);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0x10);
        let this = Addr(r32(c, S2));
        c.g[A0][0] = this.off(PBM_HANDLE).lw(c);
        'done: {
            if (c.g[A0][0] as i64) < 0 {
                break 'done;
            }
            c.cycles += 5;
            call_ptr(c, ASYNC_FILE_IS_DONE, 0x0026_c624)?;
            c.g[V1][0] = 1;
            if c.g[V0][0] != 1 {
                break 'done;
            }
            c.cycles += 9;
            c.g[A0][0] = this.off(PBM_HANDLE).lw(c);
            let data_at = Addr(sp(c));
            let size_at = data_at.off(4);
            c.g[A1][0] = c.g[SP][0];
            data_at.sw(c, 0);
            c.g[A2][0] = c.g[SP][0] | 4;
            size_at.sw(c, 0);
            call_ptr(c, ASYNC_FILE_GET_RESULT, 0x0026_c648)?;
            'clear: {
                c.g[V1][0] = data_at.lw(c);
                c.g[V0][0] = u64::MAX;
                if c.g[V1][0] == 0 {
                    break 'clear;
                }
                c.cycles += 3;
                c.g[A0][0] = size_at.lw(c);
                c.g[S1][0] = PBE_SIZE;
                if (c.g[A0][0] as i64) <= 0 {
                    break 'clear;
                }
                // entries[cur].buffer = operator_new_named(size, ..)
                c.cycles += 27;
                c.g[S0][0] = this.off(PBM_CURRENT).lw(c);
                c.g[V0][0] = this.off(PBM_ENTRIES).lw(c);
                c.mult(0, S0, S0, S1);
                c.g[A1][0] = PLAYBACK_ALLOC_NAME;
                c.g[A2][0] = PLAYBACK_ALLOC_FLAGS;
                c.g[A3][0] = 0;
                c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
                call_ptr(c, OPERATOR_NEW_NAMED, 0x0026_c684)?;
                let entry = Addr(r32(c, S0));
                entry.off(PBE_BUFFER).sw(c, c.g[V0][0]);
                // entries[cur].size = size
                c.g[V0][0] = this.off(PBM_CURRENT).lw(c);
                c.g[V1][0] = this.off(PBM_ENTRIES).lw(c);
                c.mult(0, V0, V0, S1);
                c.g[A0][0] = size_at.lw(c);
                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                Addr(r32(c, V0)).off(PBE_BYTES).sw(c, c.g[A0][0]);
                // EA_MemCopy(entries[cur].buffer, data, size)
                c.g[V1][0] = this.off(PBM_CURRENT).lw(c);
                c.g[V0][0] = this.off(PBM_ENTRIES).lw(c);
                c.mult(0, V1, V1, S1);
                c.g[A2][0] = size_at.lw(c);
                c.g[A1][0] = data_at.lw(c);
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                c.g[A0][0] = Addr(r32(c, V1)).off(PBE_BUFFER).lw(c);
                call_ptr(c, EA_MEM_COPY, 0x0026_c6c0)?;
                c.g[A0][0] = data_at.lw(c);
                call_ptr(c, MEM_FREE, 0x0026_c6c8)?;
                c.g[V0][0] = u64::MAX;
            }
            c.cycles += 2;
            this.off(PBM_HANDLE).sw(c, c.g[V0][0]);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ReplayCache_RecordFrame (0x26d2b0)

/// ReplayCache: number of chunks.
pub(crate) const REPLAY_COUNT: u32 = 0x0;

/// The chunk array: 8 bytes each, the low 12 bits of the first word a
/// repeat count.
pub(crate) const REPLAY_CHUNKS: u32 = 0xc;

pub(crate) const REPLAY_REPEAT_MASK: u64 = 0xfff;

/// `memcmp` (cross-shard, through the table).
pub(crate) const MEMCMP: u32 = 0x0041_5fc8;

pub(crate) const REPLAY_CACHE_PUSH_FRAME: Callee = Callee {
    addr: 0x0026_d228,
    f: ee::s07::ReplayCache_PushFrame,
};

/// `int ReplayCache::RecordFrame(ReplayCache* cache, const u64* frame)`
/// (0x26d2b0): run-length records one 8-byte frame. If the last chunk
/// (repeat count masked out) equals the frame and its count is below 0xfff,
/// the count goes up; otherwise the frame is pushed as a new chunk
/// (`ReplayCache_PushFrame`). Returns 1.
pub fn replay_cache_record_frame(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        set_reg(c, S0, reg(c, A0));
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0x10);
        let cache = r32(c, S0);
        let count = Addr(cache.wrapping_add(REPLAY_COUNT)).lw(c);
        set_reg(c, V1, count);
        set_reg(c, S1, reg(c, A1));
        let mut extended = false;
        if count as i64 <= 0 {
            c.cycles += 3;
            set_reg(c, A0, reg(c, S0));
            set_reg(c, A1, reg(c, S1));
            REPLAY_CACHE_PUSH_FRAME.call(c, 0x0026_d370)?;
            extended = true;
        } else {
            // Compare the last chunk, repeat count cleared, with the frame.
            c.cycles += 18;
            let chunks = Addr(cache.wrapping_add(REPLAY_CHUNKS)).lw(c);
            set_reg(c, V0, chunks);
            let last = (count as u32).wrapping_sub(1);
            set_reg(c, V1, sx(last));
            let off = last << 3;
            set_reg(c, S2, sx(off));
            set_reg(c, A3, !REPLAY_REPEAT_MASK);
            let entry = off.wrapping_add(chunks as u32);
            set_reg(c, V0, sx(entry));
            let s = sp(c);
            set_reg(c, A0, reg(c, SP));
            c.ldl(V1, entry.wrapping_add(7));
            c.ldr(V1, entry);
            c.sdl(V1, s.wrapping_add(7));
            c.sdr(V1, s);
            let key = c.ld(s) & !REPLAY_REPEAT_MASK;
            set_reg(c, A1, reg(c, S1));
            set_reg(c, A2, 8);
            set_reg(c, V0, key);
            c.sd(s, key);
            call_ptr(c, MEMCMP, 0x0026_d314)?;
            let differs = reg(c, V0) != 0;
            set_reg(c, A0, reg(c, S0));
            if !differs {
                c.cycles += 7;
                let chunks = Addr(cache.wrapping_add(REPLAY_CHUNKS)).lw(c);
                set_reg(c, V0, chunks);
                let at = sx(r32(c, S2).wrapping_add(chunks as u32));
                set_reg(c, A1, at);
                let word = Addr(at as u32).lw(c);
                set_reg(c, A0, word);
                let repeat = word & REPLAY_REPEAT_MASK;
                let room = (repeat as i64) < 0xfff;
                set_reg(c, V0, room as u64);
                set_reg(c, V1, addu(repeat, 1));
                if room {
                    c.cycles += 6;
                    let v = (word & !REPLAY_REPEAT_MASK) | (reg(c, V1) & REPLAY_REPEAT_MASK);
                    set_reg(c, V0, v);
                    set_reg(c, V1, reg(c, V1) & REPLAY_REPEAT_MASK);
                    Addr(at as u32).sw(c, v);
                    extended = true;
                } else {
                    c.cycles += 1;
                    set_reg(c, A0, reg(c, S0));
                }
            }
            if !extended {
                c.cycles += 4;
                set_reg(c, A1, reg(c, S1));
                REPLAY_CACHE_PUSH_FRAME.call(c, 0x0026_d35c)?;
                frame.restore(c, S0, 0x40);
            }
        }
        if extended {
            c.cycles += 1;
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 6;
        set_reg(c, V0, 1);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ReplayCache_Update (0x26d178)

/// World pointer (`gp - 0x848`).
pub(crate) const GP_WORLD: u32 = 0x848;

/// World: the race manager (`+0x84`); race manager: the game state (`+0x28`).
pub(crate) const WORLD_RACE: u32 = 0x84;

pub(crate) const RACE_STATE: u32 = 0x28;

/// Game state: three "paused/busy" flags that stop recording.
pub(crate) const STATE_BUSY_A: u32 = 0x60c;

pub(crate) const STATE_BUSY_B: u32 = 0x608;

pub(crate) const STATE_BUSY_C: u32 = 0x610;

/// Replay cache: the chunk buffer (null: no replay).
pub(crate) const CACHE_CHUNKS: u32 = 0xc;

/// The switch's jump table on the game mode (15 entries).
pub(crate) const UPDATE_JUMP_TABLE: u32 = 0x0048_15b0;

pub(crate) const UPDATE_RECORD: u32 = 0x0026_d1d0;

pub(crate) const UPDATE_IDLE: u32 = 0x0026_d204;

pub(crate) const UPDATE_READ: u32 = 0x0026_d20c;

pub(crate) const UPDATE_DONE: u32 = 0x0026_d218;

pub(crate) const REPLAY_CACHE_RECORD_FRAME: Callee = Callee {
    addr: 0x0026_d2b0,
    f: ee::s07::ReplayCache_RecordFrame,
};

pub(crate) const REPLAY_CACHE_READ_FRAME: Callee = Callee {
    addr: 0x0026_d420,
    f: ee::s07::ReplayCache_ReadFrame,
};

/// `int ReplayCache_Update(ReplayCache* cache, void* frame)` (0x26d178):
/// once per tick, by game mode (the state word at world->race->state):
/// mode 0 records `frame` unless one of the state's busy flags is set
/// (then returns 1), modes 10-12/14 return 0, modes 1/2/4 read the next
/// recorded frame into `frame` (its result), the rest return 1. A cache
/// without a buffer returns 0.
pub fn replay_cache_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let cache = r32(c, A2);
        c.g[V0][0] = Addr(cache.wrapping_add(CACHE_CHUNKS)).lw(c);
        let gp = r32(c, GP);
        c.g[A0][0] = Addr(gp.wrapping_sub(GP_WORLD)).lw(c);
        'out: {
            if c.g[V0][0] == 0 {
                c.cycles += 2;
                c.g[V0][0] = 0;
                break 'out;
            }
            c.cycles += 7;
            c.g[A3][0] = 1;
            let world = r32(c, A0);
            c.g[V0][0] = Addr(world.wrapping_add(WORLD_RACE)).lw(c);
            let race = r32(c, V0);
            c.g[V1][0] = Addr(race.wrapping_add(RACE_STATE)).lw(c);
            let state = r32(c, V1);
            c.g[V1][0] = Addr(state).lw(c);
            c.g[V0][0] = (c.g[V1][0] < 0xf) as u64;
            c.g[T0][0] = c.g[A0][0];
            'done: {
                if c.g[V0][0] == 0 {
                    break 'done;
                }
                c.cycles += 7;
                c.g[V0][0] = sx(UPDATE_JUMP_TABLE);
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(UPDATE_JUMP_TABLE));
                let slot = r32(c, V1);
                c.g[A0][0] = Addr(slot).lw(c);
                let target = r32(c, A0);
                let mut case = target;
                if case == UPDATE_RECORD {
                    c.cycles += 5;
                    let world = r32(c, T0);
                    c.g[V0][0] = Addr(world.wrapping_add(WORLD_RACE)).lw(c);
                    let race = r32(c, V0);
                    c.g[A0][0] = Addr(race.wrapping_add(RACE_STATE)).lw(c);
                    let state = r32(c, A0);
                    c.g[V1][0] = Addr(state.wrapping_add(STATE_BUSY_A)).lw(c);
                    c.g[V0][0] = c.g[A3][0];
                    if c.g[V1][0] != 0 {
                        break 'out;
                    }
                    for busy in [STATE_BUSY_B, STATE_BUSY_C] {
                        c.cycles += 3;
                        let flag = Addr(state.wrapping_add(busy)).lw(c);
                        c.g[V0][0] = c.g[A3][0];
                        if flag != 0 {
                            break 'out;
                        }
                    }
                    c.cycles += 2;
                    c.g[A0][0] = c.g[A2][0];
                    REPLAY_CACHE_RECORD_FRAME.call(c, UPDATE_IDLE)?;
                    case = UPDATE_IDLE;
                }
                match case {
                    UPDATE_IDLE => {
                        c.cycles += 2;
                        c.g[A3][0] = 0;
                    }
                    UPDATE_READ => {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[A2][0];
                        REPLAY_CACHE_READ_FRAME.call(c, 0x0026_d214)?;
                        c.g[A3][0] = c.g[V0][0];
                    }
                    UPDATE_DONE => {}
                    _ => {
                        c.jump_out(target, 0x0026_d178, 0x0026_d228);
                        return Ok(());
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[A3][0];
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void WorldState::WriteDeadObjects(cPbStream* s)` (0x26da88): counts the
/// processes in list 8 of the process manager (gp+0x2898), writes the count
/// as an 8-byte record (`s->Write(&rec, 8)`, vtable +8), then for each of
/// those processes the word at +0x78 of its object (node +0x18).
pub fn world_state_write_dead_objects(c: &mut Cpu) {
    const PROCESS_LIST_FIRST: u32 = 0x0035_4ea8;
    const PROCESS_LIST_NEXT: u32 = 0x0035_4ef8;
    const DEAD_LIST: u64 = 0x8;
    const OBJECT: u32 = 0x18;
    const SAVED_WORD: u32 = 0x78;
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x60);
        c.g[A0][0] = sx((c.g[GP][0] as u32).wrapping_add(0x2898));
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x50);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[A1][0] = DEAD_LIST;
        // rec = { count, 0 } on the stack
        c.sw(frame.at(c, 0), 0);
        c.sw(frame.at(c, 4), 0);
        call_ptr(c, PROCESS_LIST_FIRST, 0x0026_dab8)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 1;
            c.g[V0][0] = c.lw(frame.at(c, 0));
            loop {
                c.cycles += 9;
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S2][0];
                c.g[A2][0] = DEAD_LIST;
                let count = sx((c.g[V0][0] as u32).wrapping_add(1));
                c.g[V0][0] = count;
                c.sw(frame.at(c, 0), count);
                call_ptr(c, PROCESS_LIST_NEXT, 0x0026_dae0)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[V0][0] = c.lw(frame.at(c, 0));
                if c.g[S0][0] == 0 {
                    c.cycles += 1;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_dac8);
                }
            }
        }
        // s->Write(&rec, 8)
        let vt = c.lw(c.g[S1][0] as u32);
        c.g[V0][0] = vt;
        c.cycles += 9;
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0x8;
        let delta = c.lh((vt as u32).wrapping_add(0x8));
        let write = c.lw((vt as u32).wrapping_add(0xc));
        c.g[V1][0] = write;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(delta as u32));
        call_ptr(c, write as u32, 0x0026_db08)?;
        let count = c.lw(frame.at(c, 0));
        c.g[V0][0] = count;
        c.g[A0][0] = c.g[S2][0];
        let mut tail = true;
        if count as i64 > 0 {
            c.cycles += 5;
            c.g[A1][0] = DEAD_LIST;
            call_ptr(c, PROCESS_LIST_FIRST, 0x0026_db1c)?;
            c.g[S0][0] = c.g[V0][0];
            if c.g[S0][0] == 0 {
                frame.restore(c, S0, 0x50);
                tail = false;
            } else {
                c.cycles += 2;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(OBJECT));
                loop {
                    // s->Write(&object->word78 (copied to the stack), 4)
                    c.cycles += 16;
                    c.g[A1][0] = sx(frame.at(c, 0x10));
                    let vt = c.lw(c.g[S1][0] as u32);
                    c.g[V1][0] = vt;
                    c.g[A2][0] = 0x4;
                    let word = c.lw((c.g[A0][0] as u32).wrapping_add(SAVED_WORD));
                    c.sw(frame.at(c, 0x10), word);
                    let delta = c.lh((vt as u32).wrapping_add(0x8));
                    let write = c.lw((vt as u32).wrapping_add(0xc));
                    c.g[V0][0] = write;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(delta as u32));
                    call_ptr(c, write as u32, 0x0026_db54)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A2][0] = DEAD_LIST;
                    call_ptr(c, PROCESS_LIST_NEXT, 0x0026_db64)?;
                    c.g[S0][0] = c.g[V0][0];
                    if c.g[S0][0] == 0 {
                        break;
                    }
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(OBJECT));
                    if event_due(c) {
                        deliver_event_here(c, 0x0026_db30);
                    }
                }
            }
        }
        if tail {
            c.cycles += 1;
            frame.restore(c, S0, 0x50);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
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
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::RA as RET;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::SP;
    pub(crate) use crate::game::ai::rider::tests::clear_2;
    pub(crate) use crate::game::dirtysock::tags2::tests::alloc_machine;
    pub(crate) use crate::game::dirtysock::tags2::tests::be_library_2 as be_library;
    pub(crate) use crate::game::dirtysock::tags2::tests::heap_init_2 as heap_init;
    pub(crate) use crate::game::dirtysock::tags2::tests::pool_allocator;
    pub(crate) use crate::game::dirtysock::tags2::tests::run_2 as run;
    pub(crate) use crate::game::dirtysock::tags2::tests::zero;
    pub(crate) use crate::game::dirtysock::tags2::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::call_machine;
    pub(crate) use crate::game::fe::feasyncfile::tests::clear_4 as clear;
    pub(crate) use crate::game::fe::feasyncfile::tests::gp_at;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_5 as pool_allocator_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::run as run_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2 as w;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_15 as SEEDS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS;
    pub(crate) use crate::game::fe::fepopup3::tests::dummy_world;
    pub(crate) use crate::game::fe::fereal::tests::base as base_2;
    pub(crate) use crate::game::fe::festaterewards::tests::heap_block;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3;
    pub(crate) use crate::game::movie::movieplayer::tests::globals;
    pub(crate) use crate::game::movie::movieplayer::tests::w as w_3;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_2 as BE;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_LIBRARY;
    pub(crate) use crate::game::movie::movieplayer::tests::W;
    pub(crate) use crate::game::sound::soundsys::tests::alloc_ready;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_5;
    pub(crate) use crate::game::sound::soundsys::tests::globals_7 as globals_2;
    pub(crate) use crate::game::sound::soundsys::tests::load_2 as load;
    pub(crate) use crate::game::sound::soundsys::tests::pool_7 as pool;
    pub(crate) use crate::game::sound::soundsys::tests::w as w_5;
    pub(crate) use crate::game::sound::soundsys::tests::BE as BE_2;
    pub(crate) use crate::game::sound::soundsys::tests::WORLD;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn replay_cache_reset_matches() {
        check(
            ssx3_ee::s07::f_0026d168,
            replay_cache_reset,
            50,
            &|c, seed| {
                let mut r = Rng(0x0c13_0009 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..3 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn replay_cache_push_frame_matches() {
        check(
            ssx3_ee::s07::f_0026d228,
            replay_cache_push_frame,
            300,
            &|c, seed| {
                let mut r = Rng(0x0c13_000a ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
                if seed % 2 == 0 {
                    // Room left in the ring buffer.
                    let count = r.next_u32() % 8;
                    let base = DATA + 0x200 + (seed % 3);
                    c.bus.write32(DATA, count);
                    c.bus.write32(DATA + 0xc, base);
                    for i in 0..2 {
                        c.bus.write32(DATA + 0x300 + i * 4, r.next_u32());
                    }
                    for i in 0..2 {
                        c.bus.write32(base + count * 8 + i * 4, r.next_u32());
                    }
                } else {
                    // Full: falls back to flagging the world view.
                    c.bus.write32(DATA, 4096 + (r.next_u32() % 8));
                    let world = DATA + 0x400;
                    let views = DATA + 0x500;
                    let view = DATA + 0x600;
                    c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                    c.bus.write32(world + 0x84, views);
                    c.bus.write32(views + 0x28, view);
                    c.bus.write32(view + 0x610, r.next_u32());
                    c.bus.write32(view + 0x614, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn replay_snapshot_set_highlight_value_matches() {
        check(
            ssx3_ee::s07::f_0026d730,
            replay_snapshot_set_highlight_value,
            200,
            &|c, seed| {
                let mut r = Rng(0x0c13_000c ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 16) as u64;
                c.g[A2][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                for i in 0..0x30 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn playback_man_restore_world_state_matches() {
        check(
            ssx3_ee::s07::f_0026cc18__generated,
            playback_man_restore_world_state,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0xc7, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
                // No snapshot pending: the world-state reader is skipped.
                c.bus.write32(DATA + 0x10, 0);
            },
        );
    }

    // --------------------------------------------------------------- replay

    #[test]
    fn replay_cache_free_matches() {
        check(
            ee::s07::f_0026d130__generated,
            replay_cache_free,
            60,
            &|c, seed| {
                let _r = alloc_machine(c, seed, 0x26d1_30);
                let self_ = DATA + 0x400;
                zero(c, self_, 0x100);
                c.g[A0][0] = sx(self_);
                if seed % 2 != 0 {
                    let buf = heap_block(c, 0x400);
                    zero(c, buf, 0x400);
                    c.bus.write32(self_ + 0xc, buf);
                } else {
                    c.bus.write32(self_ + 0xc, 0);
                }
            },
        );
    }

    #[test]
    fn replay_cache_dtor_matches() {
        check(
            ssx3_ee::s07::f_0026d0e8__generated,
            replay_cache_dtor,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x26d0e8, 0);
                be_library(c);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = (seed & 1) as u64;
            },
        );
    }

    #[test]
    fn replay_cache_ctor_matches() {
        check(
            ee::s07::f_0026d0a0__generated,
            replay_cache_ctor,
            60,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x26d0_a0, 1);
                c.g[A0][0] = sx(DATA + 0x100);
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn world_state_save_matches() {
        check(
            ee::s07::f_0026d7d0__generated,
            world_state_save,
            40,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x26d7_d0, 2);
                dummy_world(c);
                let this = DATA + 0x100;
                let out = DATA + 0x200;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(out);
                c.g[A2][0] = sx(DATA + 0x300 + (seed as u32 & 3));
                c.bus.write32(DATA + 0x300, r.next_u32());
                c.bus.write32(DATA + 0x304, r.next_u32());
                c.bus.write32(this + 0x3c8, r.next_u32());
                c.bus.write32(out + 0x1c, 0xdead_beef);
            },
        );
    }

    #[test]
    fn playback_man_save_world_state_matches() {
        check(
            ee::s07::f_0026cbb8__generated,
            playback_man_save_world_state,
            48,
            &|c, seed| {
                base(c, seed, 0x26cb_b8);
                let s = DATA + 0x100;
                clear(c, s, 0x100);
                w(c, s + 0x10, if seed % 3 == 0 { 0 } else { 1 });
                let holder = DATA + 0x300;
                clear(c, holder, 0x40);
                w(c, s + 0x5c, holder as u32);
                w(
                    c,
                    holder + 0x28,
                    if seed % 4 == 0 { 0 } else { DATA + 0x400 },
                );
                clear(c, DATA + 0x400, 0x40);
                c.g[A0][0] = sx(s);
            },
        );
    }

    #[test]
    fn playback_man_reset_matches() {
        check(
            ee::s07::f_0026ca90__generated,
            playback_man_reset,
            40,
            &|c, seed| {
                base_2(c, seed, 0x26_ca90);
                c.g[A0][0] = sx(DATA);
                c.bus
                    .write32(DATA + 0x4, if seed % 3 == 0 { 0 } else { DATA + 0x1000 });
                // 2 = recording (the cheap stop path); 3 = already idle.
                c.bus
                    .write32(DATA + 0x70, if seed % 2 == 0 { 2 } else { 3 });
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(DATA + 0x1000, 0xdead);
                c.bus.write32(DATA + 0x1004, 0xdead);
            },
        );
    }

    #[test]
    fn playback_man_stop_playback_matches() {
        check(
            ee::s07::f_0026cd20__generated,
            playback_man_stop_playback,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x26cd20, 6);
                be_library_2(c);
                world(c);
                heap_init_3(c);
                c.g[A0][0] = DATA as u64;
                if seed % 2 == 0 {
                    w_2(c, DATA + 0x70, 0);
                } else {
                    w_2(c, DATA + 0x70, 1);
                    w_2(c, DATA + 0x60, DATA + 0x400);
                    w_2(c, DATA + 0x5c, DATA + 0x600);
                    w_2(c, DATA + 0x600 + 0x28, DATA + 0x700);
                    w_2(c, DATA + 0x600 + 0x84, DATA + 0x900);
                }
            },
        );
    }

    #[test]
    fn run_list_seek_matches() {
        check(
            ee::s07::f_0026d4d8__generated,
            run_list_seek,
            16,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x26d4_d8, 0);
                let list = DATA;
                let entries = DATA + 0x1000;
                let n = (seed % 5) as u32;
                w_3(c, list, n);
                w_3(c, list + 0xc, entries);
                let mut pos = 0u32;
                for i in 0..n {
                    let size = 1 + r.next_u32() % 0x100;
                    w_3(c, entries + 8 * i, size);
                    w_3(c, entries + 8 * i + 4, pos);
                    pos = pos.wrapping_add(size);
                }
                c.g[A0][0] = list as u64;
                c.g[A1][0] = match seed % 4 {
                    0 => 0,
                    1 => pos as u64,
                    2 => pos as u64 + 1,
                    _ => (r.next_u32() % (pos + 1).max(1)) as u64,
                };
            },
        );
    }

    // ------------------------------------------------------- replay cache

    #[test]
    fn replay_cache_read_frame_matches() {
        check(
            ee::s07::f_0026d420__generated,
            replay_cache_read_frame,
            SEEDS_2,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x26d4, 0);
                let ring = DATA + 0x1_000;
                for i in 0..0x20u32 {
                    w_4(c, ring + 4 * i, r.next_u32());
                }
                w_4(c, DATA, r.next_u32() % 4);
                w_4(c, DATA + 4, r.next_u32() % 4);
                w_4(c, DATA + 8, r.next_u32() % 4);
                w_4(c, DATA + 0xc, ring);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
            },
        );
    }

    #[test]
    fn playbackman_start_playback_matches() {
        check(
            ee::s07::f_0026cc48__generated,
            playbackman_start_playback,
            12,
            &|c, seed| {
                let _ = base_5(c, seed, 0x26cc48, 2);
                let this = DATA;
                clear_2(c, this, 0x400);
                c.g[A0][0] = this as u64;
                match seed % 3 {
                    0 => w_5(c, this + 0x70, 1),
                    1 => {
                        w_5(c, this + 4, DATA + 0x200);
                        w_5(c, DATA + 0x200 + 0xc, 0);
                    }
                    _ => {
                        w_5(c, this + 4, DATA + 0x200);
                        w_5(c, DATA + 0x200 + 0xc, 0);
                    }
                }
            },
        );
    }
    use super::*;
    use crate::difftest::events::driven;
    use crate::difftest::put_words;

    /// A playback manager in play, record or another mode (`sel % 6`), its
    /// stream (exhausted/full or not, 0..12 buttons) and an output pad.
    pub(crate) fn read_input_frame_setup(c: &mut Cpu, r: &mut Rng, sel: u32) {
        use playback::*;
        let (this, stream, live, out) = (DATA, DATA + 0x100, DATA + 0x800, DATA + 0x1000);
        let mode = [MODE_PLAY, MODE_RECORD, MODE_PLAY, MODE_RECORD, 0, 2][(sel % 6) as usize];
        c.bus.write32(this + MODE, mode as u32);
        c.bus.write32(this + STREAM, stream);
        c.bus.write32(this + LIVE_PAD, live);
        c.bus.write32(this + OUT_PAD, out);
        c.bus.write32(GAME_GP - PLAY_SCALE, 0x3b80_8081); // 1/255
        c.bus.write32(GAME_GP - RECORD_SCALE, 0x3b80_8081);
        let n = r.next_u32() % 13;
        let pos = r.next_u32() % 0x40;
        c.bus.write32(stream + STREAM_POS, pos);
        let full = r.next_u32() % 6 == 0;
        c.bus.write32(
            stream + STREAM_LEN,
            if full {
                pos
            } else {
                pos + 1 + n + r.next_u32() % 8
            },
        );
        c.bus.write32(
            stream + STREAM_CAP,
            if full { pos + 0x18 } else { pos + 0x40 },
        );
        let bytes = stream + 0x10 + pos;
        c.bus.write8(bytes, n as u8);
        for k in 0..n {
            c.bus.write8(bytes + 1 + k, r.next_u32() as u8);
        }
        c.bus.write32(live, n);
        for k in 0..n {
            let v = match r.next_u32() % 4 {
                0 => 0,
                1 => 0x3f80_0000,
                _ => r.f32(1.0) & 0x7fff_ffff,
            };
            c.bus.write32(live + 4 + 0x1c * k, v);
        }
        c.bus.write32(out, r.next_u32() % (n + 1));
        c.g[A0][0] = sx(this);
        c.pc = ENTRY;
    }

    #[test]
    fn playback_man_read_input_frame_matches() {
        check(
            ee::s07::f_0026ce50__generated,
            playback_man_read_input_frame,
            150,
            &|c, seed| {
                let mut r: Rng = machine(c, seed, 0x26_ce50, 0);
                read_input_frame_setup(c, &mut r, seed);
            },
        );
    }
    use super::*;
    use super::*;

    /// PlaybackMan_PollFileLoad: the engine allocator and fake syscalls
    /// (`feasyncfile::tests::base_2`), four async file slots (0x450c34,
    /// count 0x519bf0) and a PlaybackMan whose handle is negative, unknown,
    /// a slot still loading, or a finished slot whose result has no data,
    /// a zero/negative size, or a real allocated block (copied into a new
    /// buffer of the current entry, then freed).
    #[test]
    fn playback_man_poll_file_load_matches() {
        use crate::game::fe::feasyncfile::tests::base_2;
        const RET: u64 = 0x0012_3450;
        check(
            ee::s07::f_0026c5f8__generated,
            playback_man_poll_file_load,
            300,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x0026_c5f8);
                let slots = DATA + 0x10000;
                let this = DATA + 0x11000;
                let entries = DATA + 0x11100;
                c.bus.write32(0x0045_0c34, slots);
                c.bus.write32(0x0051_9bf0, 4);
                fill(c, &mut r, slots, 4 * 12);
                for i in 0..4u32 {
                    // Not done, no error, by default.
                    let file = slots + 0x30 * i;
                    c.bus.write32(file, 0x100 | i);
                    c.bus.write32(file + 0x1c, 1);
                    c.bus.write32(file + 0x10, 0);
                }
                fill(c, &mut r, this, 0x20);
                fill(c, &mut r, entries, 0x30);
                c.bus.write32(this + PBM_ENTRIES, entries);
                c.bus.write32(this + PBM_CURRENT, r.next_u32() % 4);
                let slot = r.next_u32() % 4;
                let file = slots + 0x30 * slot;
                let size = 1 + r.next_u32() % 0x180;
                let handle = match seed % 6 {
                    0 => 0x8000_0000 | r.next_u32(),
                    1 => 0x200 | slot,
                    _ => 0x100 | slot,
                };
                if seed % 6 != 2 {
                    c.bus.write32(file + 0x1c, 0);
                }
                match seed % 6 {
                    3 => {
                        // Finished without data.
                        c.bus.write32(file + 0x8, size);
                        c.bus.write32(file + 0x2c, size);
                    }
                    4 => {
                        // Finished with an empty or negative size.
                        let n = if r.next_u32() % 2 == 0 {
                            0
                        } else {
                            0x8000_0000 | r.next_u32()
                        };
                        c.bus.write32(file + 0x8, n);
                        c.bus.write32(file + 0x2c, (DATA + 0x12000).wrapping_add(n));
                    }
                    _ => {
                        // A loaded block from the engine allocator.
                        c.g[A0][0] = size as u64;
                        c.g[A1][0] = PLAYBACK_ALLOC_NAME;
                        c.g[A2][0] = PLAYBACK_ALLOC_FLAGS;
                        c.g[A3][0] = 0;
                        c.g[RA][0] = RET;
                        c.pc = OPERATOR_NEW_NAMED;
                        ee::s10::f_00317e30(c);
                        assert_eq!(c.pc, RET as u32, "operator_new_named did not return");
                        let data = c.g[V0][0] as u32;
                        assert!(data != 0, "no block");
                        fill(c, &mut r, data, size / 4);
                        c.bus.write32(file + 0x8, size);
                        c.bus.write32(file + 0x2c, data + size);
                    }
                }
                c.bus.write32(this + PBM_HANDLE, handle);
                c.g[A0][0] = sx(this);
                c.g[ssx3_ee_rt::cpu::reg::SP][0] = crate::difftest::SP as u64;
                c.pc = 0x0026_c5f8;
            },
        );
    }

    /// `ReplayCache_RecordFrame`: chunk counts -1..12 (zero or less: pushed
    /// at once), the last chunk's repeat count anywhere up to 0xfff, the
    /// frame equal to the last chunk (repeat bits ignored) or differing in
    /// one byte; the real `memcmp` and `ReplayCache_PushFrame` run.
    #[test]
    fn replay_cache_record_frame_matches() {
        const CACHE: u32 = DATA;
        const CHUNKS: u32 = DATA + 0x1000;
        const NEW: u32 = DATA + 0x2000;
        crate::difftest::check(
            ee::s07::f_0026d2b0__generated,
            replay_cache_record_frame,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x26d2b0, 0);
                c.pc = 0x0026_d2b0;
                let count = match seed % 4 {
                    0 => (r.next_u32() % 2) as i32 - 1,
                    _ => 1 + (r.next_u32() % 12) as i32,
                };
                c.bus.write32(CACHE + REPLAY_COUNT, count as u32);
                c.bus.write32(CACHE + REPLAY_CHUNKS, CHUNKS);
                fill(c, &mut r, CHUNKS, 32);
                let last = CHUNKS + 8 * (count.max(1) as u32 - 1);
                let repeat = match (seed / 4) % 4 {
                    0 => 0xfff,
                    1 => 0xffe,
                    _ => r.next_u32() & 0xfff,
                };
                let w0 = c.bus.read32(last) & !0xfff | repeat;
                c.bus.write32(last, w0);
                let mut frame = c.bus.read64(last) & !0xfff;
                frame |= (r.next_u32() & 0xfff) as u64;
                frame &= !0xfffu64;
                if seed % 3 == 0 {
                    frame ^= 1u64 << (12 + r.next_u32() % 52);
                }
                c.bus.write64(NEW, frame);
                c.g[A0][0] = sx(CACHE);
                c.g[A1][0] = sx(NEW);
            },
        );
    }
    use super::*;

    /// `ReplayCache_Update`: every game mode 0..14 and some out of range,
    /// the busy flags (none or one of three) in record mode, a cache with or
    /// without a buffer; the real `ReplayCache_RecordFrame` (with `memcmp`
    /// and `ReplayCache_PushFrame`) and `ReplayCache_ReadFrame` run.
    #[test]
    fn replay_cache_update_matches() {
        const CACHE: u32 = DATA;
        const WORLD: u32 = DATA + 0x100;
        const RACE: u32 = DATA + 0x200;
        const STATE: u32 = DATA + 0x400;
        const CHUNKS: u32 = DATA + 0x1000;
        const FRAME: u32 = DATA + 0x2000;
        check(
            ee::s07::f_0026d178__generated,
            replay_cache_update,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x26d178, 0);
                c.pc = 0x0026_d178;
                put_words(
                    c,
                    UPDATE_JUMP_TABLE,
                    &[
                        0x26d1d0, 0x26d20c, 0x26d20c, 0x26d218, 0x26d20c, 0x26d218, 0x26d218,
                        0x26d218, 0x26d218, 0x26d218, 0x26d204, 0x26d204, 0x26d204, 0x26d218,
                        0x26d204,
                    ],
                );
                c.bus.write32(GAME_GP - GP_WORLD, WORLD);
                c.bus.write32(WORLD + WORLD_RACE, RACE);
                c.bus.write32(RACE + RACE_STATE, STATE);
                let mode = match seed % 20 {
                    15 => 15 + r.next_u32() % 100,
                    16 => 0x8000_0000 | r.next_u32(),
                    17..=19 => 0,
                    m => m,
                };
                c.bus.write32(STATE, mode);
                for (i, off) in [STATE_BUSY_A, STATE_BUSY_B, STATE_BUSY_C]
                    .into_iter()
                    .enumerate()
                {
                    let set = (seed / 20) % 4 == i as u32 + 1;
                    c.bus
                        .write32(STATE + off, if set { r.next_u32() | 1 } else { 0 });
                }
                // The cache: count, read position, repeat left, chunks.
                fill(c, &mut r, CHUNKS, 32);
                let count = 1 + r.next_u32() % 12;
                c.bus.write32(CACHE, count);
                c.bus.write32(CACHE + 4, r.next_u32() % (count + 1));
                c.bus.write32(CACHE + 8, r.next_u32() % 4);
                let chunks = if seed % 23 == 0 { 0 } else { CHUNKS };
                c.bus.write32(CACHE + CACHE_CHUNKS, chunks);
                let last = CHUNKS + 8 * (count - 1);
                let mut frame = c.bus.read64(last) & !0xfff;
                if seed % 3 == 0 {
                    frame ^= 1u64 << (12 + r.next_u32() % 52);
                }
                c.bus.write64(FRAME, frame);
                c.g[A0][0] = sx(CACHE);
                c.g[A1][0] = sx(FRAME);
            },
        );
    }
    use super::*;

    /// `WorldState_WriteDeadObjects`: the process manager at gp+0x2898
    /// points at list headers (0x44 each); list 8 holds 0..4 nodes (empty:
    /// first == the sentinel at header+0x14), each with an object (+0x18)
    /// whose word +0x78 is random. ProcessList_First/Next run for real; the
    /// stream's `Write` (vtable +8) is a fake (logs the registers, sometimes
    /// unwinds).
    #[test]
    fn world_state_write_dead_objects_matches() {
        const STREAM: u32 = DATA + 0x100;
        const VT: u32 = DATA + 0x200;
        const PROCS: u32 = DATA + 0x1000;
        const NODES: u32 = DATA + 0x2000;
        const OBJECTS: u32 = DATA + 0x3000;
        covered(
            ee::s07::f_0026da88__generated,
            world_state_write_dead_objects,
            240,
            &|c, seed| {
                let nodes = seed % 5;
                let mut r = machine(c, seed, 0x26da88, 1 + nodes);
                c.pc = 0x0026_da88;
                c.bus.write32(GAME_GP + 0x2898 + 4, PROCS);
                let head = PROCS + 8 * 0x44;
                let sentinel = head + 0x14;
                let node = |i: u32| {
                    if i < nodes {
                        NODES + 0x40 * i
                    } else {
                        sentinel
                    }
                };
                c.bus.write32(head + 8, node(0));
                for i in 0..nodes {
                    c.bus.write32(node(i) + 4, node(i + 1));
                    c.bus.write32(node(i) + 0x18, OBJECTS + 0x100 * i);
                    c.bus.write32(OBJECTS + 0x100 * i + 0x78, r.next_u32());
                }
                c.bus.write32(STREAM, VT);
                c.bus.write16(VT + 8, (r.next_u32() % 0x20) as u16 & !3);
                c.bus.write32(VT + 0xc, FAKE);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(STREAM);
            },
        );
    }
}
