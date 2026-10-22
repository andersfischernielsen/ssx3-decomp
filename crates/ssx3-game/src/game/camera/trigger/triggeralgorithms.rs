//! `src/camera/trigger/triggeralgorithms.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call;
use crate::call::call_ptr;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::call::guest;
use crate::abi::r32;
use crate::call::sp;
use crate::ps2f::FpuF32;
use callee::*;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
pub(crate) use crate::game::ai::aianimeventmap::EA_MEM_ALLOC_NAMED;
pub(crate) use crate::game::be::bedbreward::BUILTIN_DELETE_ADDR;
pub(crate) use crate::game::world::wscriptcache::DELETE_2 as DELETE;
pub(crate) use crate::game::fe::feasyncfile::p;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0016_c628, active_camera_trigger_list_clear, "ActiveCameraTriggerList_Clear"),
    (0x0016_c678, active_camera_trigger_list_clear_flags, "ActiveCameraTriggerList_ClearFlags"),
    (0x0016_c6b0, active_camera_trigger_list_find, "ActiveCameraTriggerList_Find"),
    (0x0016_c6e0, active_camera_trigger_list_find_by_id, "ActiveCameraTriggerList_FindById"),
    (0x0016_c718, active_camera_trigger_list_add, "ActiveCameraTriggerList_Add"),
    (0x0016_c778, active_camera_trigger_list_remove, "ActiveCameraTriggerList_Remove"),
    (0x0016_c800, camera_trigger_man_ctor, "CameraTriggerMan_ctor"),
    (0x0016_c888, camera_trigger_man_dtor, "CameraTriggerMan_dtor"),
    (0x0016_c918, camera_trigger_man_reset, "CameraTriggerMan_Reset"),
    (0x0016_c968, camera_trigger_man_reset_active, "CameraTriggerMan_ResetActive"),
];

// ---------------------------------------------------------------------------
// Callees (addresses the generated body reaches through the function table).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::world::wscriptcache::callee::SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::SCRIPT_SOUND_BANK_MAN_FIND_BANK;
    pub(crate) use crate::game::world::wscriptcache::callee::SSX_AUDIO_SYSTEM_PAUSE;
    pub(crate) use crate::game::world::wscriptcache::callee::BANK_MONITOR_DELETE_INSTANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::BELIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::BELIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::world::wscriptcache::callee::BE_NEW_RACE_INT_GET_TRACK_INFO;
    pub(crate) use crate::game::world::wscriptcache::callee::SSX_AUDIO_SYSTEM_IS_TRACK_ID_GROUP5;
    pub(crate) use crate::game::world::wscriptcache::callee::CROWD_AUDIO_FREE_SAMPLE_LIST;
    pub(crate) use crate::game::world::wscriptcache::callee::CUT_SCENE_TRANSITION_MAN_BASE_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::CUT_SCENE_TRANSITION_MAN_INIT;
    pub(crate) use crate::game::world::wscriptcache::callee::WSCRIPT_MAN_COLLECT_SET_GET_COUNT;
    pub(crate) use crate::game::world::wscriptcache::callee::WSCRIPT_MAN_COLLECT_SET_ADD_ITEMS;
    pub(crate) use crate::game::world::wscriptcache::callee::EA_MEM_FREE_2 as EA_MEM_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::MDF_FILE_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::SPRINTF_2 as SPRINTF;
    pub(crate) use crate::game::world::wscriptcache::callee::INPUT_PREPROC_WRITE_STRING;
    pub(crate) use crate::game::world::wscriptcache::callee::CINSTANCE_NODE_CTOR_V;
    pub(crate) use crate::game::world::wscriptcache::callee::FONT_FREE;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_LIST_COUNT;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_ANIM_APPLY;
    pub(crate) use crate::game::world::wscriptcache::callee::UI_ANIM_ADVANCE;
    pub(crate) use crate::game::world::wscriptcache::callee::SECTION_ALLOCATOR_FREE_ALL;
    pub(crate) use crate::game::world::wscriptcache::callee::C_WORLD_CACHE_STREAMER_DTOR_STUB;
    pub(crate) use crate::game::world::wscriptcache::callee::RCMP_SYSTEM_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::PS2_SONY_CODEC_INTERNAL_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::MPEG2_GET_BITS1;
    pub(crate) use crate::game::world::wscriptcache::callee::MPEG2_FLUSH_BUFFER_3 as MPEG2_FLUSH_BUFFER;
    pub(crate) use crate::game::world::wscriptcache::callee::RIDER_SET_SUB_STATE;
    pub(crate) use crate::game::world::wscriptcache::callee::RIDER_SET_MAIN_STATE;
    pub(crate) use crate::game::world::wscriptcache::callee::BE_SCORE_INT_GET_HIGH_SCORE_TABLE;
    pub(crate) use crate::game::world::wscriptcache::callee::CHASE_CAMERA_ALGORITHM_CTOR_2 as CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::world::wscriptcache::callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR;
    pub(crate) use crate::game::world::wscriptcache::callee::CAMERA_TRIGGER_STACK_CLEAR;
    pub(crate) use crate::game::world::wscriptcache::callee::CAMERA_RESET_CHASE_CONTROLLER_SWITCHES;
    pub(crate) use crate::game::world::wscriptcache::callee::BOLT_FILE_CACHE_SHUTDOWN;
    pub(crate) use crate::game::world::wscriptcache::callee::DELETE;
    pub(crate) use crate::game::world::wscriptcache::callee::SND_BANK_MONITOR_MEM_SIZE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_3 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_2 as SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_2 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::BX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_INIT_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_CLEAR_EMITTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_BLOCK_POOL_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_MEMMAN_CREATE_SECTION_ALLOCATOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SECTION_TABLE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_4 as UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM_2 as UI_OBJECT_UPDATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2_2 as WORLD_OBJ_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_SPRINGS_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_BUILD_MESH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_UPDATE_VERTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSTANCE_GET_NODE_MATRIX;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_FINISH_CLEAR_FLAG40;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_ANIM_MAP_SET_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::ACTIVE_CAMERA_TRIGGER_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_STACK_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_ERROR_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_CREATED_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_9 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_SHOW_ROOM_TAB;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_UPDATE_PLAYERS_IN_ROOM;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_UPDATE_3 as NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_IS_FLAG_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_EMPTY_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_REQUEST;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_NEXT_SECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_2 as SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CAM_BUILD_MATRIX_FROM_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_MEM_CARD_CLEAR_DIR_CACHE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_RESET_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_INIT_FROM_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST as UI_LIST_FIRST_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_4 as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_2 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_3 as SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_10 as NET_FE_STATE_ON_CHILD_EVENT_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_INSTANCE_3 as SCRIPT_ENGINE_GET_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_OBJ_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ON_ACTIVATED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_3 as SCRIPT_OBJ_ACTIVATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::CUI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_2 as UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_3 as UI_LIST_NODE_DTOR_BODY;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_APPLY as UI_ANIM_APPLY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_ADVANCE as UI_ANIM_ADVANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_PROCESS_DTOR_2 as GAME_PROCESS_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_RESET_RACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODE_MAN_RESET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_REQUEST_STATE_5 as GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_START_NEXT_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_APP;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_INTERFACE_GET_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_BE_2 as BE_GET_BE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_5 as SPRINTF_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_ADD_CHILD;

    pub const CAMERA_TRIGGER_MAN_CLEAR_DATA: Callee = Callee {
        addr: 0x0016_bf20,
        f: ee::s02::f_0016bf20,
    };
}


// ---------------------------------------------------------------------------
// Active camera trigger list (a linked list of 0x10-byte nodes: id at +0,
// next at +4, flags at +8/+0xc).

/// The first word of a list holds the head node.
pub(crate) const LIST_HEAD: u32 = 0x0;

pub(crate) const NODE_NEXT: u32 = 0x4;

pub(crate) const NODE_ID: u32 = 0x0;

pub(crate) const NODE_FLAG_A: u32 = 0x8;

pub(crate) const NODE_FLAG_B: u32 = 0xc;


/// `void ActiveCameraTriggerList_Clear(ActiveCameraTriggerList *list)`
/// (0x16c628): delete every node and set the head to null. 8 cycles + per
/// node.
pub fn active_camera_trigger_list_clear(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = Addr(c.g[S1][0] as u32).off(LIST_HEAD).lw(c);
        if c.g[S0][0] == 0 {
            c.sw(c.g[S1][0] as u32, 0);
        } else {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            loop {
                c.cycles += 4;
                c.g[S0][0] = Addr((c.g[S0][0] as u32).wrapping_add(NODE_NEXT)).lw(c);
                call_ptr(c, BUILTIN_DELETE_ADDR, 0x0016_c658)?;
                let more = c.g[S0][0] != 0;
                c.g[A0][0] = c.g[S0][0];
                if !more {
                    c.cycles += 1;
                    c.sw(c.g[S1][0] as u32, 0);
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0016_c650);
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


/// `void ActiveCameraTriggerList_ClearFlags(ActiveCameraTriggerList *list)`
/// (0x16c678): zero the flag words of every node.
pub fn active_camera_trigger_list_clear_flags(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A0][0] = Addr(c.g[A0][0] as u32).off(LIST_HEAD).lw(c);
    if c.g[A0][0] == 0 {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    c.sw((c.g[A0][0] as u32).wrapping_add(NODE_FLAG_B), 0);
    loop {
        c.cycles += 7;
        c.sw((c.g[A0][0] as u32).wrapping_add(NODE_FLAG_A), 0);
        c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(NODE_NEXT)).lw(c);
        if c.g[A0][0] == 0 {
            break;
        }
        c.sw((c.g[A0][0] as u32).wrapping_add(NODE_FLAG_B), 0);
        if event_due(c) {
            deliver_event_here(c, 0x0016_c688);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}


/// `Node *ActiveCameraTriggerList_Find(ActiveCameraTriggerList *list, int id)`
/// (0x16c6b0): the first node whose id equals `id`, or null.
pub fn active_camera_trigger_list_find(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A0][0] = Addr(c.g[A0][0] as u32).off(LIST_HEAD).lw(c);
    if c.g[A0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(NODE_ID).lw(c);
    loop {
        c.cycles += 2;
        if c.g[V0][0] == c.g[A1][0] {
            c.cycles += 2;
            c.g[V0][0] = c.g[A0][0];
            abi::ret(c);
            return;
        }
        c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(NODE_NEXT)).lw(c);
        c.cycles += 2;
        if c.g[A0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(NODE_ID).lw(c);
        if event_due(c) {
            deliver_event_here(c, 0x0016_c6c0);
        }
    }
}


/// `Node *ActiveCameraTriggerList_Add(ActiveCameraTriggerList *list, int id)`
/// (0x16c718): allocate a 0x10-byte node, push it at the head, and return it.
pub fn active_camera_trigger_list_add(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x30);
        c.g[A2][0] = 0x2000_0000;
        frame.save(c, S1, 0x10);
        c.g[A3][0] = 0;
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = 0x46_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x10;
        c.g[A1][0] = sx(0x46_0000u32.wrapping_sub(0x4098));
        EA_MEM_ALLOC_NAMED.call(c, 0x0016_c748)?;
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(LIST_HEAD).lw(c);
        c.g[V1][0] = 1;
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        c.sw(c.g[V0][0] as u32, c.g[S1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(NODE_FLAG_A), c.g[V1][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(NODE_NEXT), c.g[A0][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(NODE_FLAG_B), c.g[V1][0]);
        frame.close(c);
        Ok(())
    })
}


/// `Node *ActiveCameraTriggerList_Remove(ActiveCameraTriggerList *list, Node *node)`
/// (0x16c778): unlink the node with pointer identity `node` and delete it;
/// returns the new head. A `match pc` dispatcher re-enterable at its loop
/// heads after an unwind (0x16c7b0, 0x16c7d0).
pub fn active_camera_trigger_list_remove(c: &mut Cpu) {
    const ENTRY: u32 = 0x0016_c778;
    const END: u32 = 0x0016_c800;
    const AT_LINK: u32 = 0x0016_c7b0;
    const AT_CMP: u32 = 0x0016_c7d0;
    call::guest(c, |c| {
        // Stage: 0 = entry, 1 = remove head, 2 = relink+delete, 3 = compare,
        // 4 = advance, 5..7 = transient, 8 = epilogue.
        let mut stage = match c.pc {
            ENTRY => 0u8,
            AT_LINK => 2,
            AT_CMP => 3,
            pc => {
                c.jump_out(pc, ENTRY, END);
                return Ok(());
            }
        };
        const FRAME_SIZE: u32 = 0x30;
        loop {
            match stage {
                0 => {
                    c.cycles += 8;
                    let frame = Frame::open(c, FRAME_SIZE);
                    frame.save(c, S1, 0x10);
                    frame.save(c, S0, 0x20);
                    c.g[S1][0] = c.g[A0][0];
                    frame.save_ra(c, 0);
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(LIST_HEAD).lw(c);
                    let head_match = c.g[A1][0] == c.g[V0][0];
                    c.g[S0][0] = c.g[V0][0];
                    stage = if head_match { 1 } else { 4 };
                }
                1 => {
                    c.cycles += 6;
                    c.g[S0][0] = Addr((c.g[A1][0] as u32).wrapping_add(NODE_NEXT)).lw(c);
                    c.g[A0][0] = c.g[A1][0];
                    call_ptr(c, BUILTIN_DELETE_ADDR, 0x0016_c7a4)?;
                    c.sw(c.g[S1][0] as u32, c.g[S0][0]);
                    c.g[V0][0] = c.g[S0][0];
                    stage = 8;
                }
                4 => {
                    c.cycles += 3;
                    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(NODE_NEXT)).lw(c);
                    if c.g[A0][0] == 0 {
                        c.g[V0][0] = 0;
                        stage = 8;
                    } else {
                        c.cycles += 1;
                        stage = 3;
                    }
                }
                3 => {
                    c.cycles += 2;
                    if c.g[A0][0] == c.g[A1][0] {
                        c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(NODE_NEXT)).lw(c);
                        if back_edge_unwinds(c, AT_LINK) {
                            return Ok(());
                        }
                        stage = 2;
                    } else {
                        c.cycles += 4;
                        c.g[S0][0] = c.g[A0][0];
                        c.g[A0][0] =
                            Addr((c.g[A0][0] as u32).wrapping_add(NODE_NEXT)).lw(c);
                        if c.g[A0][0] != 0 {
                            if back_edge_unwinds(c, AT_CMP) {
                                return Ok(());
                            }
                            stage = 3;
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = 0;
                            stage = 8;
                        }
                    }
                }
                2 => {
                    c.cycles += 4;
                    c.sw((c.g[S0][0] as u32).wrapping_add(NODE_NEXT), c.g[V0][0]);
                    call_ptr(c, BUILTIN_DELETE_ADDR, 0x0016_c7b8)?;
                    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(NODE_NEXT)).lw(c);
                    stage = 8;
                }
                _ => {
                    c.cycles += 5;
                    let frame = Frame { size: FRAME_SIZE };
                    frame.restore(c, S0, 0x20);
                    frame.restore(c, S1, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
            }
        }
    })
}


/// `ActiveCameraTrigger* ActiveCameraTriggerList_FindById(list, int id)`
/// (0x16c6e0): walk the array of trigger pointers, return the node whose
/// first word is `id`.
pub fn active_camera_trigger_list_find_by_id(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A0][0] = Addr(c.g[A0][0] as u32).lw(c);
    if c.g[A0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        crate::abi::ret(c);
        return;
    }
    c.cycles += 1;
    c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
    loop {
        c.cycles += 3;
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        if c.g[V0][0] != c.g[A1][0] {
            c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(4)).lw(c);
            c.cycles += 2;
            if c.g[A0][0] == 0 {
                c.cycles += 2;
                c.g[V0][0] = 0;
                crate::abi::ret(c);
                return;
            }
            c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0016_c6f0);
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = c.g[V1][0];
            crate::abi::ret(c);
            return;
        }
    }
}


/// `void CameraTriggerMan_ResetActive(CameraTriggerMan* m)` (0x16c968): purge
/// both active-trigger lists, re-init both trigger stacks and reset the chase
/// controller switches.
pub fn camera_trigger_man_reset_active(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x14));
        callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR.call(c, 0x0016_c980)?;
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x18));
        callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR.call(c, 0x0016_c988)?;
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x1c));
        callee::CAMERA_TRIGGER_STACK_CLEAR.call(c, 0x0016_c990)?;
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x3c));
        callee::CAMERA_TRIGGER_STACK_CLEAR.call(c, 0x0016_c998)?;
        callee::CAMERA_RESET_CHASE_CONTROLLER_SWITCHES.call(c, 0x0016_c9a0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void CameraTriggerMan_Reset(man)` (0x16c918): clear the camera trigger
/// manager's header, its two active lists, its two trigger stacks and reset
/// the chase controller switches.
pub fn camera_trigger_man_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        CAMERA_TRIGGER_MAN_CLEAR_DATA.call(c, 0x0016_c92c)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
        ACTIVE_CAMERA_TRIGGER_LIST_CLEAR.call(c, 0x0016_c934)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
        ACTIVE_CAMERA_TRIGGER_LIST_CLEAR.call(c, 0x0016_c93c)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
        CAMERA_TRIGGER_STACK_CLEAR.call(c, 0x0016_c944)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x3c));
        CAMERA_TRIGGER_STACK_CLEAR.call(c, 0x0016_c94c)?;
        call_ptr(c, 0x0015_dc18, 0x0016_c954)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Camera triggers.

/// `CameraTriggerMan* CameraTriggerMan_ctor(CameraTriggerMan* self)`
/// (0x16c800): construct the trigger list, the two active-trigger lists and
/// the two trigger stacks.
pub fn camera_trigger_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = 0x1;
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0016_c828);
        c.g[S0][0] = sx(p(r32(c, S2), 0x14));
        callee::CAMERA_TRIGGER_LIST_CTOR.call(c, 0x0016_c828)?;
        c.g[S3][0] = u64::MAX;
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0016_c83c);
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            callee::ACTIVE_CAMERA_TRIGGER_LIST_CTOR.call(c, 0x0016_c83c)?;
            let more = c.g[S1][0] != c.g[S3][0];
            c.g[S0][0] = sx(p(r32(c, S0), 4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0016_c830);
            }
        }
        c.cycles += 3;
        c.g[S0][0] = sx(p(r32(c, S2), 0x1c));
        c.g[S1][0] = 0x1;
        c.g[S3][0] = u64::MAX;
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[RA][0] = sx(0x0016_c85c);
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            callee::CAMERA_TRIGGER_STACK_CTOR.call(c, 0x0016_c85c)?;
            let more = c.g[S1][0] != c.g[S3][0];
            c.g[S0][0] = sx(p(r32(c, S0), 0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0016_c850);
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


/// `void CameraTriggerMan_dtor(self, int flags)` (0x16c888): purge the active
/// trigger list and the four trigger slots, then the manager itself.
pub fn camera_trigger_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        let self_ = abi::arg(c, 0);
        let flags = abi::arg64(c, 1);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = sx(self_);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[S3][0] = flags;
        call_ptr(c, 0x0016_cbe0, 0x0016_c8ac)?;
        c.g[V0][0] = sx(p(r32(c, S1), 0x14));
        let none = c.g[V0][0] == 0;
        c.g[S0][0] = sx(p(r32(c, S1), 0x1c));
        if !none {
            c.cycles += 2;
            let empty = c.g[V0][0] == c.g[S0][0];
            c.g[S2][0] = c.g[V0][0];
            if !empty {
                c.cycles += 2;
                c.g[S0][0] = sx(r32(c, S0).wrapping_sub(4));
                loop {
                    c.cycles += 5;
                    c.g[A1][0] = 0;
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0016_c5e0, 0x0016_c8d4)?;
                    let more = c.g[S2][0] != c.g[S0][0];
                    c.g[S0][0] = sx(r32(c, S0).wrapping_sub(4));
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0016_c8c8);
                    }
                }
            }
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0016_bef8, 0x0016_c8e8)?;
        c.g[V0][0] = c.g[S3][0] & 1;
        let keep = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x40);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, callee::OPERATOR_DELETE, 0x0016_c8fc)?;
            frame.restore(c, S0, 0x40);
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fill;
    use crate::difftest::RA as TEST_RA;
    use ssx3_ee as ee;
    pub(crate) use crate::game::world::wscriptcache::tests::SEEDS;
    pub(crate) use crate::game::ai::aianimeventmap::tests::MEM_MUTEX;
    pub(crate) use crate::game::ai::aianimeventmap::tests::alloc_setup;
    pub(crate) use crate::game::camera::trigger::cameratriggervolumes::tests::build_list;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_4 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::w32;
    pub(crate) use crate::game::fe::feasyncfile::tests::gpat;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_8 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_4 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_5 as world_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;


    #[test]
    fn active_camera_trigger_list_clear_matches() {
        check(ee::s02::f_0016c628, active_camera_trigger_list_clear, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xc628, 0);
            alloc_setup(c, &mut r);
            c.g[A0][0] = DATA as u64;
            build_list(c, &mut r, DATA, seed % 2);
        });
    }


    #[test]
    fn active_camera_trigger_list_clear_flags_matches() {
        check(ee::s02::f_0016c678, active_camera_trigger_list_clear_flags, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xc678, 0);
            c.g[A0][0] = DATA as u64;
            build_list(c, &mut r, DATA, seed % 5);
        });
    }


    #[test]
    fn active_camera_trigger_list_find_matches() {
        check(ee::s02::f_0016c6b0, active_camera_trigger_list_find, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xc6b0, 0);
            let nodes = seed % 4;
            let base = build_list(c, &mut r, DATA, nodes);
            c.g[A0][0] = DATA as u64;
            let id = if nodes > 0 && seed % 3 != 0 {
                c.bus.read32(base + (seed % nodes) * 0x10)
            } else {
                r.next_u32()
            };
            c.g[A1][0] = sx(id);
        });
    }


    #[test]
    fn active_camera_trigger_list_add_matches() {
        check(ee::s02::f_0016c718, active_camera_trigger_list_add, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xc718, 0);
            alloc_setup(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = sx(r.next_u32());
            c.bus.write32(DATA, if seed % 2 == 0 { 0 } else { DATA + 0x1000 });
        });
    }


    #[test]
    fn active_camera_trigger_list_remove_matches() {
        check(ee::s02::f_0016c778, active_camera_trigger_list_remove, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0xc778, 0);
            alloc_setup(c, &mut r);
            let nodes = (seed % 4) + 1;
            let base = build_list(c, &mut r, DATA, nodes);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = sx(match seed % 4 {
                0 => base,
                1 => base + ((seed as u32 % nodes) * 0x10),
                2 => base + (nodes - 1) * 0x10,
                _ => base + 0x100,
            });
        });
    }


    #[test]
    fn active_camera_trigger_list_find_by_id_matches() {
        check(ssx3_ee::s02::f_0016c6e0__generated, active_camera_trigger_list_find_by_id, SEEDS_2, &|c, seed| {
            let _r = machine(c, seed, 0x6c6e0, 0);
            let head = DATA + 0x1000;
            let ids = [11u32, 22, 33];
            for k in 0..3u32 {
                let node = DATA + 0x3000 + 0x10 * k;
                c.bus.write32(head + 4 * k, DATA + 0x2000 + 0x10 * k);
                c.bus.write32(DATA + 0x2000 + 0x10 * k, node);
                c.bus.write32(node, ids[k as usize]);
            }
            c.bus.write32(head + 12, 0);
            if seed % 5 == 0 {
                c.bus.write32(head, 0); // empty list
            }
            let pick = [11u64, 33, 99, 22][(seed as usize / 5) % 4];
            c.g[A0][0] = sx(head);
            c.g[A1][0] = pick;
        });
    }


    #[test]
    fn camera_trigger_man_reset_active_matches() {
        check(
            ee::s02::f_0016c968__generated,
            camera_trigger_man_reset_active,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x16_c968, 0);
                c.g[A0][0] = sx(DATA + 0x1000);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }


    // -----------------------------------------------------------------------
    // 0x16c918 CameraTriggerMan_Reset.

    #[test]
    fn camera_trigger_man_reset_matches() {
        check(ee::s02::f_0016c918__generated, camera_trigger_man_reset, 8, &|c, seed| {
            let _r = machine(c, seed, 0x16c9_18, 0);
            let this = DATA + 0x100;
            c.g[A0][0] = sx(this);
            for off in [0x14u32, 0x18, 0x1c, 0x3c] {
                c.bus.write32(this + off, 0); // empty lists/stacks
            }
        });
    }


    #[test]
    fn camera_trigger_man_ctor_matches() {
        check(ssx3_ee::s02::f_0016c800__generated, camera_trigger_man_ctor, SEEDS_3, &|c, seed| {
            let mut r = machine(c, seed, 0x16c800, 0);
            be_library(c);
            world(c);
            let man = DATA + 0x100;
            c.g[A0][0] = man as u64;
            let _ = r.next_u32();
        });
    }


    #[test]
    fn camera_trigger_man_dtor_matches() {
        check(
            ee::s02::f_0016c888__generated,
            camera_trigger_man_dtor,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x16c8, 0);
                env(c);
                let this = DATA + 0x400;
                w32(c, this + 4, (seed % 2) as u32);
                w32(c, this + 0x14, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }
}
