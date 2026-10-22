//! `src/object/bucketman.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::float;
use ssx3_ee_rt::Cpu;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::call::Callee;
use crate::call::Flow;
use ssx3_ee as ee;
use ssx3_ee_rt::vu0::X;
pub(crate) use crate::game::fe::feasyncfile::at;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::fe::feasyncfile::r;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0035_4b38, process_list_shutdown, "ProcessList_Shutdown"),
    (0x0035_4c08, process_list_add, "ProcessList_Add"),
    (0x0035_4e48, process_list_remove, "ProcessList_Remove"),
    (0x0035_4ea8, process_list_first, "ProcessList_First"),
    (0x0035_4ed0, process_list_last, "ProcessList_Last"),
    (0x0035_4f20, process_list_prev, "ProcessList_Prev"),
    (0x0035_4f70, process_list_pending_next, "ProcessList_PendingNext"),
    (0x0035_5118, process_list_delete_priority, "ProcessList_DeletePriority"),
    (0x0035_51a8, process_list_delete_section_objects, "ProcessList_DeleteSectionObjects"),
    (0x0035_5260, process_list_set_sorted_insert, "ProcessList_SetSortedInsert"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL_2 as UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_4 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_2 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_3 as SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_10 as NET_FE_STATE_ON_CHILD_EVENT;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_COUNT_2 as BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_ENTRY_2 as BUDDY_MAN_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_DATA_GET_BANK_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_UNREGISTER_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ANIM_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_SOUND_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_INSTANCE_3 as SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_OBJ_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ON_ACTIVATED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_3 as SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CUI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_2 as UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_3 as UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR_2 as UI_OBJECT_DTOR;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_REQUEST_STATE_5 as GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_START_NEXT_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_APP;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_INTERFACE_GET_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_BE_2 as BE_GET_BE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_5 as SPRINTF;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS_2 as SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_KILL_ALL_PROCESSES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS_2 as SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE_2 as SCRIPT_ENGINE_RESET_PROCESS_TABLE;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::MALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FREE2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::VEC3_MAX_COMPONENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_12 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_DTOR_4 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_3 as NET_FE_STATE_INIT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_SELECTED_USER;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_MANAGER_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_FREE_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_PARSE_2 as CAMERA_TRIGGER_MAN_PARSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_MONTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_DAY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_YEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_GENDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_LEGAL;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARD_GALLERY_INIT_PAGES;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARD_GALLERY_REQUEST_SHAPE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OV_STATE_OVERLAY_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPTIONS_HUD_REFRESH_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_SET_FLAG_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_BIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_WRITE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_NEXT_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_EMPTY_STUB;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_CTOR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_ALLOC_BUFFER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_LANGUAGE_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SET_EVENT_CONTEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRACK_TO_CATEGORY;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE_2 as AUDIO_GET_GAME_MODULE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_SOMETHING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_NEW;
}


// ---------------------------------------------------------------------------
// Process list.

/// `void ProcessList_Add(ProcessList *list, void *entry, uint index)`
/// (0x354c08): insert `entry` into the list at `index`, on the used or free
/// side depending on the slot's flag.
pub fn process_list_add(c: &mut Cpu) {
    c.cycles += 8;
    let list = abi::arg(c, 0);
    let entry = c.g[A1][0];
    c.g[V0][0] = 0x44;
    c.g[V1][0] = Addr(list).off(4).lw(c);
    c.g[A2][0] = sx(abi::mult(c, c.g[A2][0] as u32, 0x44));
    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
    c.g[V0][0] &= 0x1;
    if c.g[V0][0] == 0 {
        c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
        c.cycles += 13;
        Addr(entry as u32).off(4).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(list).off(4).lw(c);
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
        Addr(c.g[V1][0] as u32).off(8).sw(c, entry);
        c.g[V0][0] = Addr(list).off(4).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4));
        Addr(entry as u32).off(8).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(list).off(4).lw(c);
        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
        Addr(c.g[V1][0] as u32).off(8).sw(c, entry);
    } else {
        c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
        c.cycles += 14;
        Addr(entry as u32).off(4).sw(c, c.g[V1][0]);
        c.g[V0][0] = Addr(list).off(4).lw(c);
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
        Addr(c.g[V1][0] as u32).off(8).sw(c, entry);
        c.g[V0][0] = Addr(list).off(4).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x24));
        Addr(entry as u32).off(8).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(list).off(4).lw(c);
        c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
        Addr(c.g[V1][0] as u32).off(0x28).sw(c, entry);
    }
    abi::ret(c);
}


/// `void ProcessList_Remove(void *entry)` (0x354e48): unlink an entry from its
/// doubly linked list.
pub fn process_list_remove(c: &mut Cpu) {
    c.cycles += 3;
    let entry = Addr(abi::arg(c, 1));
    c.g[V1][0] = entry.off(4).lw(c);
    if c.g[V1][0] != 0 {
        c.cycles += 3;
        c.g[V0][0] = entry.off(8).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            Addr(c.g[V0][0] as u32).off(4).sw(c, c.g[V1][0]);
            c.g[V1][0] = entry.off(8).lw(c);
            c.g[V0][0] = entry.off(4).lw(c);
            Addr(c.g[V0][0] as u32).off(8).sw(c, c.g[V1][0]);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}


/// `void* ProcessList_First(ProcessList *list, uint index)` (0x354ea8): the
/// head entry of slot `index`, or null.
pub fn process_list_first(c: &mut Cpu) {
    c.cycles += 9;
    c.g[V0][0] = 0x44;
    c.g[V1][0] = Addr(abi::arg(c, 0)).off(4).lw(c);
    c.g[A1][0] = sx(abi::mult(c, c.g[A1][0] as u32, 0x44));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x14));
    c.g[A1][0] = c.g[V0][0] ^ c.g[A1][0];
    if c.g[A1][0] == 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}


/// `void* ProcessList_Last(ProcessList *list, uint index)` (0x354ed0): the
/// tail entry of slot `index`, or null.
pub fn process_list_last(c: &mut Cpu) {
    c.cycles += 9;
    c.g[V0][0] = 0x44;
    c.g[V1][0] = Addr(abi::arg(c, 0)).off(4).lw(c);
    c.g[A1][0] = sx(abi::mult(c, c.g[A1][0] as u32, 0x44));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4));
    c.g[A1][0] = c.g[V0][0] ^ c.g[A1][0];
    if c.g[A1][0] == 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}


/// `void* ProcessList_Prev(ProcessList *list, void *entry, uint index)`
/// (0x354f20): the previous entry of `entry` in slot `index`, or null.
pub fn process_list_prev(c: &mut Cpu) {
    c.cycles += 9;
    c.g[V0][0] = 0x44;
    c.g[V1][0] = Addr(abi::arg(c, 0)).off(4).lw(c);
    c.g[A2][0] = sx(abi::mult(c, c.g[A2][0] as u32, 0x44));
    c.g[V0][0] = Addr(abi::arg(c, 1)).off(8).lw(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
    c.g[V1][0] = c.g[V0][0] ^ c.g[V1][0];
    if c.g[V1][0] == 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}


/// `void* ProcessList_PendingNext(ProcessList *list, void *entry, uint index)`
/// (0x354f70): the pending-next entry of `entry` in slot `index`, or null.
pub fn process_list_pending_next(c: &mut Cpu) {
    c.cycles += 9;
    c.g[V0][0] = 0x44;
    c.g[V1][0] = Addr(abi::arg(c, 0)).off(4).lw(c);
    c.g[A2][0] = sx(abi::mult(c, c.g[A2][0] as u32, 0x44));
    c.g[V0][0] = Addr(abi::arg(c, 1)).off(4).lw(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x34));
    c.g[V1][0] = c.g[V0][0] ^ c.g[V1][0];
    if c.g[V1][0] == 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}


/// `void ProcessList_SetSortedInsert(ProcessList *list, uint index)`
/// (0x355260): mark slot `index` as sorted-insert.
pub fn process_list_set_sorted_insert(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x44;
    c.g[V1][0] = Addr(abi::arg(c, 0)).off(4).lw(c);
    c.g[A1][0] = sx(abi::mult(c, c.g[A1][0] as u32, 0x44));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = c.lw(c.g[A1][0] as u32);
    c.g[V0][0] |= 0x1;
    c.sw(c.g[A1][0] as u32, c.g[V0][0]);
    abi::ret(c);
}


/// `void cBucketMan_purgeBucket(cBucketMan* self, int index)` (0x355118): for
/// every node the indexed process list yields, call the node's virtual
/// `purge(3)` (slot 1 of the vtable at `node + 0xc`).
pub fn process_list_delete_priority(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        let self_ = abi::arg(c, 0);
        let index = abi::arg(c, 1);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        c.g[S3][0] = sx(self_);
        c.g[S2][0] = sx(index);
        call_ptr(c, callee::PROCESS_LIST_FIRST, 0x0035_513c)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0 {
            frame.restore(c, S0, 0x40);
        } else {
            c.cycles += 2;
            c.g[A0][0] = c.g[S3][0];
            loop {
                c.cycles += 5;
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.g[S2][0];
                call_ptr(c, callee::PROCESS_LIST_NEXT, 0x0035_515c)?;
                let done = c.g[S0][0] == 0;
                c.g[S1][0] = c.g[V0][0];
                if !done {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw(at(c, S0, 0xc));
                    c.g[A1][0] = 3;
                    c.g[A0][0] = c.lh(at(c, V0, 8));
                    c.g[V1][0] = c.lw(at(c, V0, 0xc));
                    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                    let target = r32(c, V1);
                    call_ptr(c, target, 0x0035_517c)?;
                }
                c.cycles += 3;
                c.g[S0][0] = c.g[S1][0];
                if c.g[S0][0] == 0 {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x40);
                    break;
                }
                c.g[A0][0] = c.g[S3][0];
                if event_due(c) {
                    deliver_event_here(c, 0x0035_5150);
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Process list.

/// `void ProcessList_DeleteSectionObjects(ProcessList* l, int section, int id)`
/// (0x3551a8): walk the list and delete every section object whose key matches
/// `id`.
pub fn process_list_delete_section_objects(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        'outer: {
            frame.save(c, S2, 0x30);
            frame.save(c, S3, 0x20);
            c.g[S2][0] = c.g[A1][0];
            let this = r(c, A0);
            c.g[S3][0] = sx(this);
            frame.save(c, S4, 0x10);
            frame.save(c, S0, 0x50);
            frame.save(c, S1, 0x40);
            frame.save_ra(c, 0);
            c.g[S4][0] = c.g[A2][0];
            callee::PROCESS_LIST_FIRST_2.call(c, 0x0035_51d4)?;
            c.g[S0][0] = c.g[V0][0];
            if c.g[S0][0] != 0 {
                c.cycles += 2;
                c.g[A0][0] = c.g[S3][0];
                loop {
                    'entry: {
                        c.cycles += 11;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x0035_51f4);
                        c.g[A2][0] = c.g[S2][0];
                        callee::PROCESS_LIST_NEXT_2.call(c, 0x0035_51f4)?;
                        c.g[V1][0] = Addr(p(r(c, S0), 0xc)).lw(c);
                        c.g[S1][0] = c.g[V0][0];
                        c.g[A0][0] = Addr(p(r(c, V1), 0x58)).lh(c);
                        c.g[V0][0] = Addr(p(r(c, V1), 0x5c)).lw(c);
                        let target = r(c, V0);
                        c.g[RA][0] = sx(0x0035_520c);
                        c.g[A0][0] = sx(p(r(c, S0), r(c, A0)));
                        call_ptr(c, target, 0x0035_520c)?;
                        if c.g[V0][0] != c.g[S4][0] {
                            c.g[S0][0] = c.g[S1][0];
                            break 'entry;
                        }
                        'del: {
                            c.cycles += 2;
                            let gone = c.g[S0][0] == 0;
                            c.g[A1][0] = 3;
                            if gone {
                                break 'del;
                            }
                            c.cycles += 5;
                            c.g[V0][0] = Addr(p(r(c, S0), 0xc)).lw(c);
                            c.g[A0][0] = Addr(p(r(c, V0), 8)).lh(c);
                            c.g[V1][0] = Addr(p(r(c, V0), 0xc)).lw(c);
                            let target = r(c, V1);
                            c.g[RA][0] = sx(0x0035_5230);
                            c.g[A0][0] = sx(p(r(c, S0), r(c, A0)));
                            call_ptr(c, target, 0x0035_5230)?;
                            break 'del;
                        }
                        c.cycles += 1;
                        c.g[S0][0] = c.g[S1][0];
                        break 'entry;
                    }
                    c.cycles += 2;
                    if c.g[S0][0] == 0 {
                        c.cycles += 1;
                        frame.restore(c, 16, 0x50);
                        break 'outer;
                    }
                    c.g[A0][0] = c.g[S3][0];
                    if event_due(c) {
                        deliver_event_here(c, 0x0035_51e8);
                    }
                }
            }
            frame.restore(c, 16, 0x50);
        }
        c.cycles += 7;
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void ProcessList_Shutdown(ProcessList *self)` (0x354b38): delete every
/// priority bucket and free the pending game processes.
pub fn process_list_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        'b_354b7c: {
            c.cycles += 8;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[S1][0] = c.g[A0][0];
            c.g[V0][0] = c.lw((c.g[S1][0] as u32));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S0][0] = 0u64;
            if t {
                break 'b_354b7c;
            } else {
                c.cycles += 2;
                c.g[A1][0] = c.g[S0][0];
                'l_354b60: loop {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x00354b68);
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x00355118, 0x00354b68)?;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32));
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = c.g[S0][0];
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x00354b60);
                        }
                        continue 'l_354b60;
                    } else {
                        break 'b_354b7c;
                    }
                }
            }
        }
        'b_354be8: {
            c.cycles += 3;
            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
            let t = c.g[A1][0] == 0u64;
            c.g[V0][0] = 0x44;
            if t {
                break 'b_354be8;
            } else {
                'b_354be0: {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_sub(0x10));
                    c.mult(0, 4, 3, 2);
                    c.g[S0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                    if c.g[A1][0] == c.g[S0][0] {
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                        break 'b_354be0;
                    } else {
                        c.cycles += 1;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x44));
                        'l_354ba0: loop {
                            c.cycles += 15;
                            c.g[A1][0] = 0x2;
                            c.g[RA][0] = sx(0x00354bac);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x34));
                            call_ptr(c, 0x003546c8, 0x00354bac)?;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
                            c.g[RA][0] = sx(0x00354bb8);
                            c.g[A1][0] = 0x2;
                            call_ptr(c, 0x003546c8, 0x00354bb8)?;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
                            c.g[RA][0] = sx(0x00354bc4);
                            c.g[A1][0] = 0x2;
                            call_ptr(c, 0x003546c8, 0x00354bc4)?;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[RA][0] = sx(0x00354bd0);
                            c.g[A1][0] = 0x2;
                            call_ptr(c, 0x003546c8, 0x00354bd0)?;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                            let t = c.g[V0][0] != c.g[S0][0];
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x44));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x00354ba0);
                                }
                                continue 'l_354ba0;
                            } else {
                                c.cycles += 1;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                                break 'b_354be0;
                            }
                        }
                    }
                }
                c.cycles += 2;
                c.g[RA][0] = sx(0x00354be8);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
                call_ptr(c, 0x00317e98, 0x00354be8)?;
                break 'b_354be8;
            }
        }
        c.cycles += 7;
        c.sw((c.g[S1][0] as u32), 0u64);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x4), 0u64);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
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
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use ssx3_ee as ee;
    use crate::difftest::RA as SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS;
    pub(crate) use crate::game::fe::feasyncfile::tests::EMPTY;
    pub(crate) use crate::game::fe::feasyncfile::tests::w16;
    pub(crate) use crate::game::fe::feasyncfile::tests::w32;
    pub(crate) use crate::game::fe::feasyncfile::tests::gpat;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_6 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_8 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_4 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_5 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_15 as SEEDS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_8 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_6 as W;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM_3 as RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE_2;
    pub(crate) use crate::game::bx::bxstring2::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::gp_at;
    pub(crate) use crate::game::bx::bxstring2::tests::run;
    pub(crate) use crate::game::be::bedbprofile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_2;
    pub(crate) use crate::game::bx::bxstring2::tests::alloc_base;
    pub(crate) use crate::game::bx::bxstring2::tests::f;


    #[test]
    fn process_list_add_matches() {
        check_lohi(ssx3_ee::s11::f_00354c08, process_list_add, 200, &|c, seed| {
            let mut r = Rng(0x101f ^ seed as u64);
            scramble(c, &mut r);
            let index = (seed % 4) as u32;
            let base = DATA + 0x200;
            let slot = base + index * 0x44;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA as u64 + 0x100) | ((r.next_u32() as u64) << 32);
            c.g[A2][0] = index as u64;
            c.bus.write32(DATA + 4, base);
            let flag = if seed % 2 == 0 { 0 } else { 1 };
            c.bus.write32(slot, flag);
            c.bus.write32(slot + 8, DATA + 0x300);
            c.bus.write32(slot + 0x28, DATA + 0x400);
        });
    }


    #[test]
    fn process_list_remove_matches() {
        check(ssx3_ee::s11::f_00354e48, process_list_remove, 200, &|c, seed| {
            let mut r = Rng(0x1020 ^ seed as u64);
            scramble(c, &mut r);
            let entry = DATA;
            let prev = DATA + 0x100;
            let next = DATA + 0x200;
            c.g[A1][0] = entry as u64;
            c.bus.write32(entry + 4, if seed % 4 >= 2 { prev } else { 0 });
            c.bus.write32(entry + 8, if seed % 2 == 0 { 0 } else { next });
            c.bus.write32(prev + 4, r.next_u32());
            c.bus.write32(next + 8, r.next_u32());
        });
    }


    #[test]
    fn process_list_first_matches() {
        check_lohi(ssx3_ee::s11::f_00354ea8, process_list_first, 200, &|c, seed| {
            let mut r = Rng(0x1021 ^ seed as u64);
            scramble(c, &mut r);
            let index = (seed % 4) as u32;
            let base = DATA + 0x200;
            let slot = base + index * 0x44;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = index as u64;
            c.bus.write32(DATA + 4, base);
            let v = if seed % 3 == 0 { slot + 0x14 } else { r.next_u32() };
            c.bus.write32(slot + 8, v);
        });
    }


    #[test]
    fn process_list_last_matches() {
        check_lohi(ssx3_ee::s11::f_00354ed0, process_list_last, 200, &|c, seed| {
            let mut r = Rng(0x1022 ^ seed as u64);
            scramble(c, &mut r);
            let index = (seed % 4) as u32;
            let base = DATA + 0x200;
            let slot = base + index * 0x44;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = index as u64;
            c.bus.write32(DATA + 4, base);
            let v = if seed % 3 == 0 { slot + 4 } else { r.next_u32() };
            c.bus.write32(slot + 0x1c, v);
        });
    }


    #[test]
    fn process_list_prev_matches() {
        check_lohi(ssx3_ee::s11::f_00354f20, process_list_prev, 200, &|c, seed| {
            let mut r = Rng(0x1023 ^ seed as u64);
            scramble(c, &mut r);
            let index = (seed % 4) as u32;
            let base = DATA + 0x200;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.g[A2][0] = index as u64;
            c.bus.write32(DATA + 4, base);
            let target = base + index * 0x44 + 4;
            let v = if seed % 3 == 0 { target } else { r.next_u32() };
            c.bus.write32(DATA + 0x100 + 8, v);
        });
    }


    #[test]
    fn process_list_pending_next_matches() {
        check_lohi(ssx3_ee::s11::f_00354f70, process_list_pending_next, 200, &|c, seed| {
            let mut r = Rng(0x1024 ^ seed as u64);
            scramble(c, &mut r);
            let index = (seed % 4) as u32;
            let base = DATA + 0x200;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x100) as u64;
            c.g[A2][0] = index as u64;
            c.bus.write32(DATA + 4, base);
            let target = base + index * 0x44 + 0x34;
            let v = if seed % 3 == 0 { target } else { r.next_u32() };
            c.bus.write32(DATA + 0x100 + 4, v);
        });
    }


    #[test]
    fn process_list_set_sorted_insert_matches() {
        check_lohi(ssx3_ee::s11::f_00355260, process_list_set_sorted_insert, 200, &|c, seed| {
            let mut r = Rng(0x1025 ^ seed as u64);
            scramble(c, &mut r);
            let index = (seed % 4) as u32;
            let base = DATA + 0x200;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = index as u64;
            c.bus.write32(DATA + 4, base);
            c.bus.write32(base + index * 0x44, r.next_u32());
        });
    }


    #[test]
    fn process_list_delete_priority_matches() {
        check(
            ee::s11::f_00355118__generated,
            process_list_delete_priority,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x3551, 0);
                env(c);
                let this = DATA + 0x400;
                let index = (seed % 2) as u32;
                let base = DATA + 0x2000;
                let desc = base + 0x44 * index;
                let sentinel = desc + 0x14;
                w32(c, this + 4, base);
                let vt = DATA + 0x3200;
                slot(c, vt, 8, EMPTY);
                let n0 = DATA + 0x3000;
                let n1 = DATA + 0x3100;
                w32(c, n0 + 0xc, vt);
                w32(c, n1 + 0xc, vt);
                match seed % 3 {
                    0 => w32(c, desc + 8, sentinel),
                    1 => {
                        w32(c, desc + 8, n0);
                        w32(c, n0 + 4, n1);
                        w32(c, n1 + 4, sentinel);
                    }
                    _ => {
                        w32(c, desc + 8, n0);
                        w32(c, n0 + 4, sentinel);
                    }
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = index as u64;
            },
        );
    }


    // ------------------------------------------------------ world painter

        // --------------------------------------------------------- process list

    #[test]
    fn process_list_delete_section_objects_matches() {
        check(
            ee::s11::f_003551a8__generated,
            process_list_delete_section_objects,
            SEEDS_2,
            &|c, seed| {
                let mut r = base(c, seed, 0x3551, 2);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = (seed % 3) as u64;
                w(c, DATA, 0);
            },
        );
    }


    #[test]
    fn process_list_shutdown_matches() {
        strict(
            ee::s11::f_00354b38__generated,
            process_list_shutdown,
            20,
            &|c, seed| {
                let _r = alloc_base(c, seed, 0x354b);
                let block = run(
                    c,
                    0x0031_7d70,
                    ee::s10::f_00317d70,
                    &[0x80, 0x0048_0c30, 0x2000_0000, 0],
                ) as u32;
                c.g[A0][0] = (block.wrapping_add(0x10)) as u64;
                for i in 0..0x20 {
                    c.bus.write32(block.wrapping_add(i * 4), 0);
                }
            },
        );
    }
}
