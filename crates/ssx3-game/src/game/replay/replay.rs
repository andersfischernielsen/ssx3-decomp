//! `src/replay/replay.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
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
pub(crate) use crate::game::fe::feasyncfile::at_5 as at;
pub(crate) use crate::game::fe::feasyncfile::gpa;
pub(crate) use crate::game::fe::feasyncfile::lw;
pub(crate) use crate::game::fe::feasyncfile::lw_6 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::p as p_2;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festaterewards2::r;
pub(crate) use crate::game::fe::festaterewards2::Ops;
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
        0x0026_ddc0,
        world_state_read_objects,
        "WorldState_ReadObjects",
    ),
    (
        0x0026_de58,
        world_state_notify_objects_restored,
        "WorldState_NotifyObjectsRestored",
    ),
    (
        0x0026_e468,
        replay_find_next_snapshot,
        "Replay_FindNextSnapshot",
    ),
    (
        0x0026_e490,
        replay_find_prev_snapshot,
        "Replay_FindPrevSnapshot",
    ),
    (
        0x0026_e4b8,
        replay_snap_to_snapshot,
        "Replay_SnapToSnapshot",
    ),
    (
        0x0026_e528,
        replay_seek_next_snapshot,
        "Replay_SeekNextSnapshot",
    ),
    (
        0x0026_e568,
        replay_seek_prev_snapshot,
        "Replay_SeekPrevSnapshot",
    ),
    (0x0026_e5a8, snapshot_list_ctor, "SnapshotList_ctor"),
    (0x0026_e5c0, snapshot_list_clear, "SnapshotList_Clear"),
    (0x0026_e608, snapshot_list_remove, "SnapshotList_Remove"),
    (
        0x0026_e670,
        snapshot_list_pop_front,
        "SnapshotList_PopFront",
    ),
    (
        0x0026_e800,
        replay_highlight_slot_clear,
        "Replay_HighlightSlotClear",
    ),
    (
        0x0026_e838,
        replay_highlight_slot_score,
        "Replay_HighlightSlotScore",
    ),
    (0x0026_e8e0, replay_stream_ctor, "ReplayStream_ctor"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::be::beintoption::callee::BE_OPTION_INT_GET_MAX_HANDICAP;
    pub(crate) use crate::game::be::beintoption::callee::C_WORLD_SOUND_RELOCATE_EMITTERS;
    pub(crate) use crate::game::be::beintoption::callee::INSEQ_GET_INSTANCE;
    pub(crate) use crate::game::be::beintoption::callee::INSEQ_INSTANCE_STOP;
    pub(crate) use crate::game::be::beintoption::callee::IRRADIANCE_REF_GET_ENTRY;
    pub(crate) use crate::game::be::beintoption::callee::REPLAY_FIND_SNAPSHOT;
    pub(crate) use crate::game::be::beintoption::callee::SCRIPT_ENGINE_CLEAR_PROCESS;
    pub(crate) use crate::game::be::beintoption::callee::STREAM_ARRAY_STOP;
    pub(crate) use crate::game::be::beintoption::callee::USTR_FROM_ASCII_2;
    pub(crate) use crate::game::be::beintoption::callee::WORLD_PAINTER_GET_IRRADIANCE_NAME2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_PUSH_2 as ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATTACHED_EMITTER_FX_DESTROY_EMITTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_LEVEL_TO_VOLUME_2 as AUDIO_LEVEL_TO_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_CHANNEL_VOLUME_2 as AUDIO_SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_DESTROY_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_RESET_ONE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BENEW_PLAYER_INT_GET_SLOT_2 as BENEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COPY_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_PROFILE_5 as BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_RESET_PROFILE_CHARACTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_CHAR_PROFILE_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_PHASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_API_DESTROY;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUILTIN_DELETE_6 as BUILTIN_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHAT_ROOM_SELECT_PLAYER_BY_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::CLEAR_MUSIC_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::DATA_INSTANCE_FREE_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_FREE_4 as EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_FINISH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_START_PROFILE_SEARCH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_RAND_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_IS_LOADING;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_CHANNEL_GET_LENGTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_AGREEMENT_TERMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_DESTROY_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_FLUSH_ROSTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_RELEASE_SNAPSHOT_LISTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_DECLINE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_GET_2 as MC_MANAGER_GET;
    pub(crate) use crate::game::fe::feasyncfile::callee::MDF_FILE_CREATE_MODEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCPY_2 as MEMCPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_3 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_2 as NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE_CALLEE;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_RESUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SET_RUNNING;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SUSPEND;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_COUNT_SAVE_SNAPSHOTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_4 as SCRIPT_CMD_QUEUE_AT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE_2 as SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_3 as SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_SET_PAUSED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_PICK_RANDOM_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_CONFIGURE_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_UPDATE_EMITTER_ENABLES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_6 as SPRINTF_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCASECMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SYNC_PROCESS_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_HIDE_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_DRAW_RECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_BASE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_ITEM_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_6 as UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_GET_OBJECT_BY_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LAUNCH_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LOOKUP_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_3 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_SET_OBJECT_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCROLL_SET_PARTS_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_IS_HULL_FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_SOUND_GET_TRIGGER_IDS;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_CREATE_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_FINALIZE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_IS_FLAG4;
    pub(crate) use crate::game::fe::festaterewards2::callee::ANIM_OBJECT_V36;
    pub(crate) use crate::game::fe::festaterewards2::callee::APP_MODULE_TRY_EXIT;
    pub(crate) use crate::game::fe::festaterewards2::callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY;
    pub(crate) use crate::game::fe::festaterewards2::callee::CROWD_AUDIO_INSTANCE_STOP_SOUND;
    pub(crate) use crate::game::fe::festaterewards2::callee::DL_HEAP_USABLE_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::DYNAMIC_EMITTER_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::EA_MUTEX_LOCK;
    pub(crate) use crate::game::fe::festaterewards2::callee::EA_MUTEX_UNLOCK;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_LOGIN_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_LOGIN_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_STATS_REFRESH_RESULTS;
    pub(crate) use crate::game::fe::festaterewards2::callee::GAME_MODULE_REQUEST_STATE as GAME_MODULE_REQUEST_STATE_2;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEMSET;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_GET_POOL_BLOCK_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_MAN_FREE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_REPORT_POOL_STATS;
    pub(crate) use crate::game::fe::festaterewards2::callee::MOUNTAIN_ROOM_UPDATE_EVENT_ICONS;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_DISCONNECTED_POPUP_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_QUICK_PLAY_POPUP_ON_OPTIONS_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::POPUP_CONFIRM_GET_TEXT_BOX_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::PREVIEW_REWARD_POLL_IMAGE_LOAD;
    pub(crate) use crate::game::fe::festaterewards2::callee::PS2_PAD_CREATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_APPLY_CAMERA;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_FIND_NEXT_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_FIND_PREV_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_SET_STATE_2 as REPLAY_SET_STATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_SNAP_TO_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_STREAM_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SCROLL_ARROWS;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_COUNT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_ICONS;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_NAMES;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCE_USBKB_SET_ARRANGEMENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCE_USBKB_SET_READ_MODE;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_ENGINE_LIST_STOP;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_ENGINE_UNLOAD_WORLD_SCRIPTS;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_LIST_ADD_MOVIE;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_LIST_POLL_LOADING;
    pub(crate) use crate::game::fe::festaterewards2::callee::SECTION_MAN_FIND_LOGICAL_SECTION;
    pub(crate) use crate::game::fe::festaterewards2::callee::SONG_INSTANCE_GET_DUCK_TO_LOOPS;
    pub(crate) use crate::game::fe::festaterewards2::callee::SPRINTF;
    pub(crate) use crate::game::fe::festaterewards2::callee::S_LOGIN_UPDATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::TROPHY_ROOM_LOAD_TROPHY_TEXTURES;
    pub(crate) use crate::game::fe::festaterewards2::callee::TWSCRIPT_CACHE_RELOCATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_STATE_UPDATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SCROLL_SET_WIDE_STRING;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::fe::festaterewards2::callee::USB_KB_DEFAULT_CONNECT_CB;
    pub(crate) use crate::game::fe::festaterewards2::callee::WORLD_STATE_STREAM_RESET;
    pub(crate) use crate::game::fe::festaterewards2::callee::WORLD_TRIGGER_MANAGER_GET_TRIGGER_INFO;
}

/// `SnapshotList *SnapshotList_ctor(SnapshotList *list)` (0x26e5a8): zero the
/// head/tail/count words and return the list.
pub fn snapshot_list_ctor(c: &mut Cpu) {
    c.cycles += 5;
    let list = Addr(abi::arg(c, 0));
    c.g[V0][0] = c.g[A0][0];
    list.off(0x4).sw(c, 0);
    list.sw(c, 0);
    list.off(0x8).sw(c, 0);
    abi::ret(c);
}

/// `void SnapshotList_Clear(SnapshotList *list)` (0x26e5c0): zero the
/// head/tail/count words.
pub fn snapshot_list_clear(c: &mut Cpu) {
    c.cycles += 4;
    let list = Addr(abi::arg(c, 0));
    list.off(0x8).sw(c, 0);
    list.off(0x4).sw(c, 0);
    list.sw(c, 0);
    abi::ret(c);
}

/// `void SnapshotList_Remove(SnapshotList *list, Snapshot *node)` (0x26e608):
/// unlink `node` from the doubly linked list and decrement the count.
pub fn snapshot_list_remove(c: &mut Cpu) {
    const HEAD: u32 = 0x0;
    const TAIL: u32 = 0x4;
    const COUNT: u32 = 0x8;
    const NEXT: u32 = 0x14;
    const PREV: u32 = 0x18;

    let list = Addr(abi::arg(c, 0));
    let node = c.g[A1][0];
    let node_at = Addr(node as u32);

    c.cycles += 3;
    let head = list.off(HEAD).lw(c);
    c.g[V0][0] = head;
    let tail;
    if node != head {
        tail = list.off(TAIL).lw(c);
        c.g[V0][0] = tail;
    } else {
        c.cycles += 3;
        let next = node_at.off(NEXT).lw(c);
        c.g[V0][0] = next;
        list.off(HEAD).sw(c, next);
        tail = list.off(TAIL).lw(c);
        c.g[V0][0] = tail;
    }

    c.cycles += 2;
    let mut next;
    if node != tail {
        next = node_at.off(NEXT).lw(c);
        c.g[V1][0] = next;
    } else {
        c.cycles += 3;
        let prev = node_at.off(PREV).lw(c);
        c.g[V0][0] = prev;
        list.off(TAIL).sw(c, prev);
        next = node_at.off(NEXT).lw(c);
        c.g[V1][0] = next;
    }

    c.cycles += 2;
    let mut prev;
    if next == 0 {
        prev = node_at.off(PREV).lw(c);
        c.g[V1][0] = prev;
    } else {
        c.cycles += 3;
        prev = node_at.off(PREV).lw(c);
        c.g[V0][0] = prev;
        Addr(next as u32).off(PREV).sw(c, prev);
        prev = node_at.off(PREV).lw(c);
        c.g[V1][0] = prev;
    }

    c.cycles += 2;
    if prev == 0 {
        node_at.off(PREV).sw(c, 0);
    } else {
        c.cycles += 3;
        next = node_at.off(NEXT).lw(c);
        c.g[V0][0] = next;
        Addr(prev as u32).off(NEXT).sw(c, next);
        node_at.off(PREV).sw(c, 0);
    }

    c.cycles += 5;
    node_at.off(NEXT).sw(c, 0);
    let count = list.off(COUNT).lw(c);
    let result = sx((count as u32).wrapping_sub(1));
    c.g[V0][0] = result;
    list.off(COUNT).sw(c, result);
    abi::ret(c);
}

/// `void Replay_HighlightSlotClear(Replay *replay)` (0x26e800): clear the
/// highlight bit for the currently selected slot in its slot byte, then drop
/// the slot pointer.
pub fn replay_highlight_slot_clear(c: &mut Cpu) {
    const SLOT_INDEX: u32 = 0x0;
    const SLOT_PTR: u32 = 0xb0;
    const SLOT_FLAGS: u32 = 0x1e;

    c.cycles += 4;
    let replay = Addr(abi::arg(c, 0));
    c.g[A2][0] = c.g[A0][0];
    let slot = replay.off(SLOT_PTR).lw(c);
    c.g[A1][0] = slot;
    c.g[V1][0] = 1;
    if slot != 0 {
        c.cycles += 7;
        let index = replay.off(SLOT_INDEX).lw(c);
        c.g[V0][0] = index;
        let flags = Addr(slot as u32).off(SLOT_FLAGS).lbu(c);
        c.g[A0][0] = flags;
        c.g[V1][0] = sx(1u32 << (index as u32 & 31));
        c.g[V1][0] = !(0u64 | c.g[V1][0]);
        c.g[A0][0] = flags & c.g[V1][0];
        Addr(slot as u32).off(SLOT_FLAGS).sb(c, c.g[A0][0]);
        replay.off(SLOT_PTR).sw(c, 0);
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `ReplayStream *ReplayStream_ctor(ReplayStream *stream)` (0x26e8e0):
/// install the vtable and zero the buffer fields; returns the stream.
pub fn replay_stream_ctor(c: &mut Cpu) {
    const VTABLE: u32 = 0x48_0000 + 0x1898;

    c.cycles += 9;
    let stream = Addr(abi::arg(c, 0));
    let vtable = sx(VTABLE);
    c.g[V1][0] = vtable;
    c.g[V0][0] = c.g[A0][0];
    stream.off(0x4).sw(c, 0);
    stream.sw(c, vtable);
    stream.off(0x8).sw(c, 0);
    stream.off(0xc).sw(c, 0);
    stream.off(0x10).sw(c, 0);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Replay snapshot stepping.

/// `Snapshot *Replay_FindNextSnapshot(Replay *r, int frame)` (0x26e468): the
/// snapshot found for `frame`, or its `+0x14` successor when it has one.
pub fn replay_find_next_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::REPLAY_FIND_SNAPSHOT.call(c, 0x0026_e478)?;
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x14)).lw(c);
        frame.restore_ra(c, 0);
        if c.g[V1][0] != 0 {
            c.g[V0][0] = c.g[V1][0];
        }
        frame.close(c);
        Ok(())
    })
}

/// `Snapshot *Replay_FindPrevSnapshot(Replay *r, int frame)` (0x26e490): the
/// snapshot found for `frame`, or its `+0x18` predecessor when it has one.
pub fn replay_find_prev_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::REPLAY_FIND_SNAPSHOT.call(c, 0x0026_e4a0)?;
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x18)).lw(c);
        frame.restore_ra(c, 0);
        if c.g[V1][0] != 0 {
            c.g[V0][0] = c.g[V1][0];
        }
        frame.close(c);
        Ok(())
    })
}

/// `SnapshotList_PopFront(list)` 0x26e670: unlink and return the list's first
/// snapshot node.
pub fn snapshot_list_pop_front(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let list = abi::arg(c, 0);
        let node = c.lw(list);
        c.g[S0][0] = node;
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[S0][0];
        callee::SNAPSHOT_LIST_REMOVE.call(c, 0x0026_e688)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_SeekNextSnapshot(Replay* self)` (0x26E528): find the snapshot
/// after the current one (+0x484 index), keep it at +0x488, advance the index
/// to its successor and snap the replay there.
pub fn replay_seek_next_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let index = c.lw_r_2(A1, r(c, S0).wrapping_add(0x484));
        let _ = index;
        callee::REPLAY_FIND_NEXT_SNAPSHOT.call(c, 0x0026_e540)?;
        let snapshot = c.g[V0][0];
        Addr(r(c, S0).wrapping_add(0x488)).sw(c, snapshot);
        c.g[A0][0] = c.g[S0][0];
        let next = c.lw_r_2(V1, (snapshot as u32).wrapping_add(0x30));
        Addr(r(c, S0).wrapping_add(0x484)).sw(c, next);
        callee::REPLAY_SNAP_TO_SNAPSHOT.call(c, 0x0026_e554)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_SeekPrevSnapshot(Replay* self)` (0x26E568): the same one
/// snapshot backwards (index at +0x484, snapshot at +0x488).
pub fn replay_seek_prev_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let index = c.lw_r_2(A1, r(c, S0).wrapping_add(0x484));
        let _ = index;
        callee::REPLAY_FIND_PREV_SNAPSHOT.call(c, 0x0026_e580)?;
        let snapshot = c.g[V0][0];
        Addr(r(c, S0).wrapping_add(0x488)).sw(c, snapshot);
        c.g[A0][0] = c.g[S0][0];
        let next = c.lw_r_2(V1, (snapshot as u32).wrapping_add(0x30));
        Addr(r(c, S0).wrapping_add(0x484)).sw(c, next);
        callee::REPLAY_SNAP_TO_SNAPSHOT.call(c, 0x0026_e594)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Replay_SnapToSnapshot(Replay* r)` (0x26e4b8): seek both replay
/// streams to the snapshot nearest the current frame.
pub fn replay_snap_to_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x40);
        sqsp(c, 16, 0x30);
        sqsp(c, 17, 0x20);
        c.g[S0][0] = c.g[A0][0];
        sqsp(c, 18, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = 1;
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x48c));
        c.g[A1][0] = Addr(p(r32(c, S0), 0x484)).lw(c);
        call::call_ptr(c, 0x0026_e448, 0x0026_e4e0)?;
        Addr(p(r32(c, S0), 0x488)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x30)).lw(c);
        Addr(p(r32(c, S0), 0x484)).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
        loop {
            c.cycles += 4;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
            let empty = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[V0][0];
            if !empty {
                c.cycles += 2;
                c.g[A1][0] = Addr(p(r32(c, S0), 0x484)).lw(c);
                call::call_ptr(c, 0x0026_d4d8, 0x0026_e508)?;
            }
            c.cycles += 2;
            if (c.g[S2][0] as i64) >= 0 {
                c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0026_e4f0);
                }
                continue;
            }
            c.cycles += 6;
            lqsp(c, 16, 0x30);
            lqsp(c, 17, 0x20);
            lqsp(c, 18, 0x10);
            frame.restore_ra(c, 0);
            let ra = c.g[RA][0] as u32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
            c.pc = ra;
            return Ok(());
        }
    });
}

/// `void WorldState_NotifyObjectsRestored(WorldState*, void* arg)`
/// (0x0026de58): walk the process list at `gp+0x2898`, virtual-call each
/// process with `arg`, then hand the process back to the list iterator.
pub fn world_state_notify_objects_restored(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
        c.g[A0][0] = sx(gpa(c, 0x2898));
        c.sq(S1, sp(c).wrapping_add(0x20));
        c.sq(S2, sp(c).wrapping_add(0x10));
        c.g[S1][0] = c.g[A1][0];
        c.sq(S0, sp(c).wrapping_add(0x30));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S2][0] = c.g[A0][0];
        call_ptr(c, 0x0035_4ea8, 0x0026_de7c)?; // ProcessList_First
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
            loop {
                c.cycles += 11;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x18));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                let target = c.g[V1][0] as u32;
                call_ptr(c, target, 0x0026_dea0)?;
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S1][0];
                call_ptr(c, 0x0035_4ef8, 0x0026_deb0)?; // ProcessList_Next
                c.g[S0][0] = c.g[V0][0];
                if c.g[S0][0] == 0 {
                    c.cycles += 1;
                    c.lq(S0, sp(c).wrapping_add(0x30));
                    break;
                }
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                if event_due(c) {
                    deliver_event_here(c, 0x0026_de90);
                }
            }
        } else {
            c.lq(S0, sp(c).wrapping_add(0x30));
        }
        c.cycles += 5;
        c.lq(S1, sp(c).wrapping_add(0x20));
        c.lq(S2, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        abi::ret(c);
        Ok(())
    })
}

/// `WorldState_ReadObjects` (0x26ddc0): read the object count and create each
/// world object, then finalize the list.
pub fn world_state_read_objects(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        let this = r(c, A0);
        c.g[S3][0] = sx(this);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 8;
        c.g[V0][0] = Addr(r(c, S1)).lw(c);
        c.g[A0][0] = Addr(p_2(r(c, V0), 0x10)).lh(c);
        c.g[V1][0] = Addr(p_2(r(c, V0), 0x14)).lw(c);
        let target = r(c, V1);
        c.g[RA][0] = sx(0x0026_de04);
        c.g[A0][0] = sx(r(c, S1).wrapping_add(r(c, A0)));
        call_ptr(c, target, 0x0026_de04)?;
        c.g[V0][0] = Addr(r(c, SP)).lw(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[A0][0] = c.g[S3][0];
        if !empty {
            loop {
                c.cycles += 8;
                c.g[A1][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x0026_de20);
                c.g[S0][0] = sx(r(c, S0).wrapping_add(1));
                callee::WORLD_STATE_CREATE_OBJECT.call(c, 0x0026_de20)?;
                c.g[V0][0] = Addr(r(c, SP)).lw(c);
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[S3][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_de10);
                }
            }
        }
        c.cycles += 10;
        c.g[A0][0] = sx(r(c, GP).wrapping_add(0x2898u32));
        c.g[RA][0] = sx(0x0026_de3c);
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, callee::WORLD_STATE_FINALIZE, 0x0026_de3c)?;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Replay highlight.

/// `int Replay_HighlightSlotScore(self, replay, mode)` (0x26e838): compare a
/// replay slot's distance (or time) against the current highlight and set it
/// when the slot is better.
pub fn replay_highlight_slot_score(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x10);
        'end: {
            'mid: {
                'obj: {
                    'inner: {
                        'dist: {
                            'cmp: {
                                c.cycles += 3;
                                let t = c.g[A2][0] == 0;
                                frame.save_ra(c, 0);
                                if t {
                                    break 'cmp;
                                }
                                c.cycles += 5;
                                c.g[V1][0] = lw_2(c, at(c.g[A1][0], 0x9c));
                                c.g[V0][0] = lw_2(c, at(c.g[A0][0], 0xa0));
                                c.g[A2][0] =
                                    sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                                let t = c.g[A2][0] == 0;
                                c.g[V0][0] = 0;
                                if t {
                                    break 'end;
                                }
                                'gate: {
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[A2][0] as i64) < 1000i64) as u64;
                                    if c.g[V0][0] != 0 {
                                        c.lwc1(1, at(c.g[A1][0], 0x44));
                                        break 'gate;
                                    }
                                    c.cycles += 2;
                                    c.g[A1][0] = lw_2(c, c.g[A0][0] as u32);
                                    break 'inner;
                                }
                                c.cycles += 6;
                                c.g[AT][0] = 0x40a0_0000;
                                c.mtc1(1, 0);
                                c.c_le_s(0, 1);
                                if !c.fcc() {
                                    c.g[V1][0] = lw_2(c, at(c.g[A1][0], 0xa4));
                                    break 'dist;
                                }
                                c.cycles += 8;
                                c.g[AT][0] = 0x447a_0000;
                                c.mtc1(1, 0);
                                c.g[A1][0] = lw_2(c, c.g[A0][0] as u32);
                                c.mul_s(0, 1, 0);
                                c.cvt_w_s(1, 0);
                                c.mfc1(6, 1);
                                c.g[A0][0] = lw_2(c, at(c.g[A0][0], 0xb0));
                                break 'obj;
                            }
                            c.cycles += 1;
                            c.g[V1][0] = lw_2(c, at(c.g[A1][0], 0xa4));
                            break 'dist;
                        }
                        c.cycles += 4;
                        c.g[V0][0] = lw_2(c, at(c.g[A0][0], 0xa8));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                        let t = c.g[V1][0] == 0;
                        c.g[A2][0] = sx(0u32.wrapping_sub(c.g[V1][0] as u32));
                        if t {
                            break 'mid;
                        }
                        c.cycles += 1;
                        c.g[A1][0] = lw_2(c, c.g[A0][0] as u32);
                        break 'inner;
                    }
                    c.cycles += 1;
                    c.g[A0][0] = lw_2(c, at(c.g[A0][0], 0xb0));
                    break 'obj;
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x0026_e8c8);
                call_ptr(c, 0x0026_d730, 0x0026_e8c8)?;
                c.g[V0][0] = 1;
                break 'end;
            }
            c.cycles += 1;
            c.g[V0][0] = 0;
            break 'end;
        }
        c.cycles += 3;
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
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    pub(crate) use crate::game::ai::ai::tests::wf_2 as wf;
    pub(crate) use crate::game::bx::bxstring2::tests::game;
    pub(crate) use crate::game::bx::bxstring2::tests::game_globals;
    pub(crate) use crate::game::bx::bxstring2::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_3 as alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::eafile_pools;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_6 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_8 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_3 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_9 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_7 as run;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_8 as run_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_4 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_8 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_9 as slot_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::wb;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_4 as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_6 as BE_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::IFACE;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH as SCRATCH_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_3 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_3 as W;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_4 as W_2;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS;
    pub(crate) use crate::game::fe::festaterewards2::tests::belib;
    pub(crate) use crate::game::fe::festaterewards2::tests::w;
    pub(crate) use crate::game::movie::movieplayer::tests::globals;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_LIBRARY;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn snapshot_list_ctor_matches() {
        check(
            ssx3_ee::s07::f_0026e5a8,
            snapshot_list_ctor,
            50,
            &|c, seed| {
                let mut r = Rng(0x0c13_000d ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..3 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn snapshot_list_clear_matches() {
        check(
            ssx3_ee::s07::f_0026e5c0,
            snapshot_list_clear,
            50,
            &|c, seed| {
                let mut r = Rng(0x0c13_000e ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..3 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn snapshot_list_remove_matches() {
        check(
            ssx3_ee::s07::f_0026e608,
            snapshot_list_remove,
            300,
            &|c, seed| {
                let mut r = Rng(0x0c13_000f ^ seed as u64);
                scramble(c, &mut r);
                let list = DATA;
                let a = DATA + 0x100;
                let b = DATA + 0x120;
                let cc = DATA + 0x140;
                // a <-> b <-> cc
                c.bus.write32(list, a);
                c.bus.write32(list + 4, cc);
                c.bus.write32(list + 8, 3);
                c.bus.write32(a + 0x14, b);
                c.bus.write32(a + 0x18, 0);
                c.bus.write32(b + 0x14, cc);
                c.bus.write32(b + 0x18, a);
                c.bus.write32(cc + 0x14, 0);
                c.bus.write32(cc + 0x18, b);
                let node = match seed % 4 {
                    0 => a,            // head (prev == 0)
                    1 => cc,           // tail (next == 0)
                    2 => b,            // middle (both non-zero)
                    _ => DATA + 0x200, // not in the list
                };
                if seed % 4 == 3 {
                    c.bus.write32(DATA + 0x200 + 0x14, DATA + 0x300);
                    c.bus.write32(DATA + 0x200 + 0x18, DATA + 0x320);
                }
                for i in 0..4 {
                    c.bus.write32(DATA + 0x300 + i * 4, r.next_u32());
                    c.bus.write32(DATA + 0x320 + i * 4, r.next_u32());
                }
                c.g[A0][0] = list as u64;
                c.g[A1][0] = node as u64;
            },
        );
    }

    #[test]
    fn replay_highlight_slot_clear_matches() {
        check(
            ssx3_ee::s07::f_0026e800,
            replay_highlight_slot_clear,
            200,
            &|c, seed| {
                let mut r = Rng(0x0c13_0010 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let slot = DATA + 0x100;
                // Cover both the empty-slot and clear-bit paths.
                c.bus
                    .write32(DATA + 0xb0, if seed % 2 == 0 { 0 } else { slot });
                c.bus.write32(
                    DATA,
                    match seed % 3 {
                        0 => (seed % 31) as u32,
                        1 => 31,
                        _ => r.next_u32(),
                    },
                );
                c.bus.write8(slot + 0x1e, r.next_u32() as u8);
            },
        );
    }

    #[test]
    fn replay_stream_ctor_matches() {
        check(
            ssx3_ee::s07::f_0026e8e0,
            replay_stream_ctor,
            50,
            &|c, seed| {
                let mut r = Rng(0x0c13_0011 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for i in 0..5 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    // ------------------------------------------------------ replay snapshots

    /// A two-snapshot list at `+0x3b0` of the replay object; the frame searched
    /// for picks whether the found snapshot is the last one or not.
    pub(crate) fn snapshot_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = game(c, seed, salt, 1);
        let replay = SCRATCH;
        c.g[A0][0] = replay as u64;
        c.g[A1][0] = (if seed % 2 == 0 { 100u32 } else { 250u32 }) as u64;
        let a = SCRATCH + 0x200;
        let b = SCRATCH + 0x300;
        c.bus.write32(replay + 0x3b0, a);
        // a: frame 100, next/prev b; b: frame 200, end of the list.
        c.bus.write32(a + 0x14, b);
        c.bus.write32(a + 0x18, b);
        c.bus.write32(b + 0x14, 0);
        c.bus.write32(b + 0x18, 0);
        c.bus.write32(a + 0x30, 100);
        c.bus.write32(b + 0x30, 200u32 + r.next_u32() % 4);
        c.bus.write32(a + 0x34, r.next_u32() % 16);
    }

    #[test]
    fn replay_find_next_snapshot_matches() {
        check(
            ee::s07::f_0026e468__generated,
            replay_find_next_snapshot,
            100,
            &|c, seed| {
                snapshot_setup(c, seed, 0x26e468);
            },
        );
    }

    #[test]
    fn replay_find_prev_snapshot_matches() {
        check(
            ee::s07::f_0026e490__generated,
            replay_find_prev_snapshot,
            100,
            &|c, seed| {
                snapshot_setup(c, seed, 0x26e490);
            },
        );
    }

    #[test]
    fn snapshot_list_pop_front_matches() {
        check(
            ssx3_ee::s07::f_0026e670__generated,
            snapshot_list_pop_front,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0xc8, 0);
                globals(c);
                c.g[A0][0] = sx(DATA);
                // Empty list (null head) or one node with no neighbours.
                let head = if seed % 2 == 0 { 0 } else { DATA + 0x100 };
                c.bus.write32(DATA, head);
                c.bus.write32(DATA + 0x100 + 0x14, 0);
                c.bus.write32(DATA + 0x100 + 0x18, 0);
            },
        );
    }

    #[test]
    fn replay_seek_next_snapshot_matches() {
        check(
            ssx3_ee::s07::f_0026e528__generated,
            replay_seek_next_snapshot,
            60,
            &|c, seed| {
                machine(c, seed, 0x26e5_28, 2);
                belib(c);
                c.g[A0][0] = DATA as u64;
                w(c, DATA + 0x484, seed % 4);
            },
        );
    }

    #[test]
    fn replay_seek_prev_snapshot_matches() {
        check(
            ssx3_ee::s07::f_0026e568__generated,
            replay_seek_prev_snapshot,
            60,
            &|c, seed| {
                machine(c, seed, 0x26e5_68, 2);
                belib(c);
                c.g[A0][0] = DATA as u64;
                w(c, DATA + 0x484, seed % 4);
            },
        );
    }

    #[test]
    fn replay_snap_to_snapshot_matches() {
        check(
            ssx3_ee::s07::f_0026e4b8__generated,
            replay_snap_to_snapshot,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x26e4b8, 0);
                let replay = DATA + 0x1000;
                c.g[A0][0] = replay as u64;
                w32(c, replay + 0x484, r.next_u32());
                // Empty snapshot list and streams.
                w32(c, replay + 0x3b0, 0);
                w32(c, replay + 0x48c, 0);
                w32(c, replay + 0x490, 0);
            },
        );
    }

    #[test]
    fn world_state_notify_objects_restored_matches() {
        check(
            ee::s07::f_0026de58__generated,
            world_state_notify_objects_restored,
            80,
            &|c, seed| {
                let _ = base(c, seed, 0x26de, 1);
                let list = GAME_GP.wrapping_add(0x2898);
                let arg = DATA + 0x300;
                let proc = DATA + 0x400;
                let proc2 = DATA + 0x600;
                // `ProcessList_First(list, arg)`: `A1 = arg + lw(list+4)*0x44`,
                // `V0 = lw(A1+8)`; it is 0 when that equals `A1+0x14`.
                let n = (seed % 3) as u32; // 0: empty list, 1: one process
                w(c, list + 4, n);
                if n == 0 {
                    w(c, arg + 8, arg.wrapping_add(0x14));
                } else {
                    w(c, arg + 0x44 + 8, proc);
                    w(c, proc + 0xc, DATA + 0x500);
                    w(c, DATA + 0x500 + 0x18, 0);
                    w(c, DATA + 0x500 + 0x1c, FAKE);
                    // `ProcessList_Next` returns `lw(proc+4)` unless it equals
                    // `arg + n*0x44 + 0x14`; 0 ends the walk.
                    w(c, proc + 4, if seed % 2 == 0 { 0 } else { proc2 });
                    w(c, proc2 + 0xc, DATA + 0x520);
                    w(c, DATA + 0x520 + 0x18, 0);
                    w(c, DATA + 0x520 + 0x1c, FAKE);
                }
                w(c, arg, 1);
                c.g[A1][0] = sx(arg);
            },
        );
    }

    #[test]
    fn world_state_read_objects_matches() {
        check(
            ee::s07::f_0026ddc0__generated,
            world_state_read_objects,
            SEEDS,
            &alloc_base_setup(|c, seed, r| {
                let this = DATA;
                let obj = DATA + 0x800;
                let vt = DATA + 0x900;
                w(c, obj, vt);
                slot_2(c, vt, 0x10, FAKE);
                w(
                    c,
                    0x01f0_0000,
                    if seed & 1 == 0 { 0 } else { 1 + (seed % 2) },
                );
                w(c, this, 0);
                let game = DATA + 0xa00;
                w(c, this + 0x84, game);
                let _ = r;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0xb00) as u64;
                c.g[A2][0] = obj as u64;
            }),
        );
    }

    // ------------------------------------------------------- replay highlight

    #[test]
    fn replay_highlight_slot_score_matches() {
        check(
            ee::s07::f_0026e838__generated,
            replay_highlight_slot_score,
            SEEDS_2,
            &|c, seed| {
                let _ = base_3(c, seed, 0x26e8_38, 4);
                let obj = SCRATCH_2;
                let rep = obj + 0x1000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = rep as u64;
                c.g[A2][0] = (seed % 3) as u64;
                let d = (seed % 5) as i32 * 400;
                w(c, rep + 0x9c, (1000 + d) as u32);
                w(c, obj + 0xa0, 1000);
                wf(c, rep + 0x44, if seed % 2 == 0 { 4.0 } else { 6.0 });
                w(c, rep + 0xa4, seed.wrapping_mul(11));
                w(c, obj + 0xa8, (seed % 4) * 3);
                w(c, rep + 0xb0, rep + 0x200);
                w(c, obj, rep);
                w(c, obj + 0xb0, rep + 0x300);
            },
        );
    }
}
