//! `src/luno/lunovm.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
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
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::be::beintnetwork::ldl;
pub(crate) use crate::game::be::beintnetwork::sdl;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::fe::fepopup::at;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw;
pub(crate) use crate::game::render::lightman::off;
pub(crate) use crate::game::render::lightman::Oc49Mem;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw;
use crate::guest::Addr;
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
use crate::abi::ret;
use crate::ps2f::FpuF32;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0022_24a8, luno_vm_pass_through, "LunoVM_PassThrough"),
    (
        0x0022_4c78,
        luno_vm_register_read_ptr,
        "LunoVMRegister_ReadPtr",
    ),
    (0x0022_4d00, luno_table_dtor, "LunoTable_dtor"),
    (0x0022_4da0, c_luno_table_ctor, "cLunoTable_ctor"),
    (0x0022_4f30, cluno_table_free_data, "cLunoTable_FreeData"),
    (0x0022_5b90, luno_vm_register_dtor, "LunoVMRegister_dtor"),
    (0x0022_5ca0, luno_vm_register_read, "LunoVMRegister_Read"),
    (
        0x0022_6600,
        luno_vm_register_set_raw,
        "LunoVMRegister_SetRaw",
    ),
    (0x0022_6610, luno_vm_register_as_int, "LunoVMRegister_AsInt"),
    (
        0x0022_6618,
        luno_vm_register_as_float,
        "LunoVMRegister_AsFloat",
    ),
    (0x0022_6768, c_luno_table_entry_dtor, "cLunoTableEntry_dtor"),
    (0x0022_24b8, luno_vm_call, "LunoVM_Call"),
    (0x0022_5068, cluno_table_assign, "cLunoTable_Assign"),
    (0x0022_57e0, c_luno_table_write, "cLunoTable_Write"),
    (0x0022_5be8, luno_vm_register_write, "LunoVMRegister_Write"),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

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
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE_2 as BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE_2 as BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE_2 as BUILTIN_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS;
    pub(crate) use crate::game::dirtysock::tags2::callee::CSETUP_MAN_SEND_PACKET;
    pub(crate) use crate::game::dirtysock::tags2::callee::CVOIP_MAN_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::CW_SCRIPT_CACHE_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::EATIMER_REMOVE_CALLBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_CAMERA_SET_CONTROLLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_FORWARD_EVENT_TO_UI;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MENU_ON_LOBBY_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LUNO_TABLE_DTOR as LUNO_TABLE_DTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_REQUEST_CARD_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_GAME_UTIL_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_HIGHLIGHT_SLOT_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SAVE_EMIT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_REMOVE_TICK_CALLBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_ARCADE_SAY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_HANDLE_MUSIC_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_PLAY_CHARACTER_TUNE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_AUDIO_STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_SOUND_BASE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR_2 as UI_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_RPC_SET_PARAMS;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_RPC_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::ACTIVE_CAMERA_TRIGGER_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_CLEAR_EMITTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_INIT_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_STACK_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_INIT_FROM_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_RESET_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_CREATED_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_ERROR_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_SHOW_ROOM_TAB;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_UPDATE_PLAYERS_IN_ROOM;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_BUILD_MESH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_SPRINGS_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_UPDATE_VERTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_ANIM_MAP_SET_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_FINISH_CLEAR_FLAG40;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_NEXT_SECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSTANCE_GET_NODE_MATRIX;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_REQUEST;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_EMPTY_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_IS_FLAG_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_9 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_UPDATE_3 as NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_MEM_CARD_CLEAR_DIR_CACHE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_3 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_2 as SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_2 as SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CAM_BUILD_MATRIX_FROM_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SECTION_TABLE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_4 as UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM_2 as UI_OBJECT_UPDATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_2 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_BLOCK_POOL_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_MEMMAN_CREATE_SECTION_ALLOCATOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2_2 as WORLD_OBJ_CTOR_2;
    pub(crate) use crate::game::fe::fepopup::callee::C00113f88;
    pub(crate) use crate::game::fe::fepopup::callee::C00115ab0;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fe78;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fe98;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fec8;
    pub(crate) use crate::game::fe::fepopup::callee::C0014a080;
    pub(crate) use crate::game::fe::fepopup::callee::C0014a0e0;
    pub(crate) use crate::game::fe::fepopup::callee::C0014bdb8;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d448;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d988;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d998;
    pub(crate) use crate::game::fe::fepopup::callee::C0014dc80;
    pub(crate) use crate::game::fe::fepopup::callee::C0014dd58;
    pub(crate) use crate::game::fe::fepopup::callee::C00150b48;
    pub(crate) use crate::game::fe::fepopup::callee::C00151c48;
    pub(crate) use crate::game::fe::fepopup::callee::C001535c0;
    pub(crate) use crate::game::fe::fepopup::callee::C001543e0;
    pub(crate) use crate::game::fe::fepopup::callee::C00156a60;
    pub(crate) use crate::game::fe::fepopup::callee::C001577e0;
    pub(crate) use crate::game::fe::fepopup::callee::C00157bf0;
    pub(crate) use crate::game::fe::fepopup::callee::C001589b0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158a50;
    pub(crate) use crate::game::fe::fepopup::callee::C00158af0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158c80;
    pub(crate) use crate::game::fe::fepopup::callee::C00158d58;
    pub(crate) use crate::game::fe::fepopup::callee::C00159cd0;
    pub(crate) use crate::game::fe::fepopup::callee::C0015a2e0;
    pub(crate) use crate::game::fe::fepopup::callee::C0016c6b0;
    pub(crate) use crate::game::fe::fepopup::callee::C0016c718;
    pub(crate) use crate::game::fe::fepopup::callee::C0017f500;
    pub(crate) use crate::game::fe::fepopup::callee::C0019e2b0;
    pub(crate) use crate::game::fe::fepopup::callee::C0019f878;
    pub(crate) use crate::game::fe::fepopup::callee::C001bb9b0;
    pub(crate) use crate::game::fe::fepopup::callee::C001c84a8;
    pub(crate) use crate::game::fe::fepopup::callee::C001c8568;
    pub(crate) use crate::game::fe::fepopup::callee::C001c9b68;
    pub(crate) use crate::game::fe::fepopup::callee::C001cb9b0;
    pub(crate) use crate::game::fe::fepopup::callee::C001cbc48;
    pub(crate) use crate::game::fe::fepopup::callee::C001cbce8;
    pub(crate) use crate::game::fe::fepopup::callee::C001cbfc0;
    pub(crate) use crate::game::fe::fepopup::callee::C001ce560;
    pub(crate) use crate::game::fe::fepopup::callee::C001d32e0;
    pub(crate) use crate::game::fe::fepopup::callee::C001e1a30;
    pub(crate) use crate::game::fe::fepopup::callee::C001e3388;
    pub(crate) use crate::game::fe::fepopup::callee::C001e91a8;
    pub(crate) use crate::game::fe::fepopup::callee::C001e91f8;
    pub(crate) use crate::game::fe::fepopup::callee::C001e9290;
    pub(crate) use crate::game::fe::fepopup::callee::C001f10f8;
    pub(crate) use crate::game::fe::fepopup::callee::C0020e900;
    pub(crate) use crate::game::fe::fepopup::callee::C00224c78;
    pub(crate) use crate::game::fe::fepopup::callee::C00224da0;
    pub(crate) use crate::game::fe::fepopup::callee::C00224df0;
    pub(crate) use crate::game::fe::fepopup::callee::C002259a8;
    pub(crate) use crate::game::fe::fepopup::callee::C00227f80;
    pub(crate) use crate::game::fe::fepopup::callee::C00231f80;
    pub(crate) use crate::game::fe::fepopup::callee::C00238bf8;
    pub(crate) use crate::game::fe::fepopup::callee::C0026d4d8;
    pub(crate) use crate::game::fe::fepopup::callee::C0026db88;
    pub(crate) use crate::game::fe::fepopup::callee::C0026e448;
    pub(crate) use crate::game::fe::fepopup::callee::C0026f4a0;
    pub(crate) use crate::game::fe::fepopup::callee::C002702f8;
    pub(crate) use crate::game::fe::fepopup::callee::C002703f0;
    pub(crate) use crate::game::fe::fepopup::callee::C002758c0;
    pub(crate) use crate::game::fe::fepopup::callee::C00275b98;
    pub(crate) use crate::game::fe::fepopup::callee::C00275cf8;
    pub(crate) use crate::game::fe::fepopup::callee::C00278e50;
    pub(crate) use crate::game::fe::fepopup::callee::C00278f38;
    pub(crate) use crate::game::fe::fepopup::callee::C00278f68;
    pub(crate) use crate::game::fe::fepopup::callee::C00281ed8;
    pub(crate) use crate::game::fe::fepopup::callee::C002837c8;
    pub(crate) use crate::game::fe::fepopup::callee::C0028b180;
    pub(crate) use crate::game::fe::fepopup::callee::C0028bb10;
    pub(crate) use crate::game::fe::fepopup::callee::C0028e8c0;
    pub(crate) use crate::game::fe::fepopup::callee::C002913d8;
    pub(crate) use crate::game::fe::fepopup::callee::C003457c8;
    pub(crate) use crate::game::fe::fepopup::callee::C003625c0;
    pub(crate) use crate::game::fe::fepopup::callee::C003626d8;
    pub(crate) use crate::game::fe::fepopup::callee::C003627a8;
    pub(crate) use crate::game::fe::fepopup::callee::C00364cd0;
    pub(crate) use crate::game::fe::fepopup::callee::C00366ce0;
    pub(crate) use crate::game::fe::fepopup::callee::C003684f0;
    pub(crate) use crate::game::fe::fepopup::callee::C0038f460;
    pub(crate) use crate::game::fe::fepopup::callee::C0038f668;
    pub(crate) use crate::game::fe::fepopup::callee::C0039c870;
    pub(crate) use crate::game::fe::fepopup::callee::C0039ff78;
    pub(crate) use crate::game::fe::fepopup::callee::C003b1300;
    pub(crate) use crate::game::fe::fepopup::callee::C003b2360;
    pub(crate) use crate::game::fe::fepopup::callee::C00125108;
    pub(crate) use crate::game::fe::fepopup::callee::C00150928;
    pub(crate) use crate::game::fe::fepopup::callee::C00156988;
    pub(crate) use crate::game::fe::fepopup::callee::C00158910;
    pub(crate) use crate::game::fe::fepopup::callee::C00159170;
    pub(crate) use crate::game::fe::fepopup::callee::C00176890;
    pub(crate) use crate::game::fe::fepopup::callee::C00225068;
    pub(crate) use crate::game::fe::fepopup::callee::C00275498;
    pub(crate) use crate::game::fe::fepopup::callee::C00278308;
    pub(crate) use crate::game::fe::fepopup::callee::C00283228;
    pub(crate) use crate::game::fe::fepopup::callee::C00283518;
    pub(crate) use crate::game::fe::fepopup::callee::C00358020;
    pub(crate) use crate::game::fe::fepopup::callee::C00362478;
    pub(crate) use crate::game::fe::fepopup::callee::C00362660;
    pub(crate) use crate::game::fe::fepopup::callee::C00368170;
    pub(crate) use crate::game::fe::fepopup::callee::C00368970;
    pub(crate) use crate::game::render::lightman::callee::DMA_WAIT_FROM_SCRATCHPAD;
    pub(crate) use crate::game::render::lightman::callee::DMA_WAIT_TO_SCRATCHPAD;
    pub(crate) use crate::game::render::lightman::callee::EA_MEMSET;
    pub(crate) use crate::game::render::lightman::callee::FLAG_MANAGER_GROUP_COMPUTE_VERTS;
    pub(crate) use crate::game::render::lightman::callee::INTERSECT_INFO_EVALUATE;
    pub(crate) use crate::game::render::lightman::callee::LIGHT_CONTRIBUTION;
    pub(crate) use crate::game::render::lightman::callee::LUNO_TABLE_DTOR;
    pub(crate) use crate::game::render::lightman::callee::MATH_SIN_COS;
    pub(crate) use crate::game::render::lightman::callee::MULTI_SPLINE_MODIFIER_DEBUG_DRAW_PATH;
    pub(crate) use crate::game::render::lightman::callee::SPATIAL_HASH_UPDATE;
    pub(crate) use crate::game::render::lightman::callee::SYNC_PROCESS_SAVE;
    pub(crate) use crate::game::render::lightman::callee::TYPED_PROCESS_SAVE;
    pub(crate) use crate::game::render::lightman::callee::WORLD_GET_AI_FRAME_COUNT;
    pub(crate) use crate::game::render::lightman::callee::WORLD_GET_FLAG_MANAGER;
    pub(crate) use crate::game::render::lightman::callee::WORLD_GET_NASH;
}

/// `void LunoVMRegister_dtor(cLunoValue *self, int flags)` (0x00225b90):
/// release a table reference when the value's type is 4, clear the type, and
/// free the value when `flags & 1`.
pub fn luno_vm_register_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 4;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = c.ldw(off(c.g[S0][0] as u32, 8));
        let is_table = c.g[V1][0] == c.g[V0][0];
        c.g[S1][0] = c.g[A1][0];
        if is_table {
            c.cycles += 2;
            c.g[A1][0] = 2;
            callee::LUNO_TABLE_DTOR.call(c, 0x0022_5bbc)?;
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 1;
        let no_free = c.g[V0][0] == 0;
        c.stw(off(c.g[S0][0] as u32, 8), 0);
        if !no_free {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x0022_5bd0)?;
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Luno table and crowd manager.

/// `cLunoTable_ctor(mem, a1)` 0x224da0: clear the table pointer, build the
/// table, return the object.
pub fn c_luno_table_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg64(c, 0);
        c.g[S0][0] = this;
        c.sw(this as u32, 0);
        callee::C_LUNO_TABLE_ALLOC.call(c, 0x0022_4db8)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Luno / movie / memory card managers.

/// `void cLunoTable_FreeData(cLunoTableRef* ref)` (0x224f30): destroy and clear
/// the table the reference holds.
pub fn cluno_table_free_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
        if c.g[A0][0] == 0 {
            let s0 = c.g[S0][0];
            Addr(s0 as u32).sw(c, 0);
        } else {
            c.cycles += 3;
            c.g[A1][0] = 3;
            callee::LUNO_TABLE_DTOR_2.call(c, 0x0022_4f54)?;
            let s0 = c.g[S0][0];
            Addr(s0 as u32).sw(c, 0);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* LunoVMRegister_SetRaw(Register* r, void* value, void* tag)`
/// (0x226600): store the value and its tag in the register object.
pub fn luno_vm_register_set_raw(c: &mut Cpu) {
    c.cycles += 3;
    let r = abi::arg(c, 0);
    Addr(r.wrapping_add(0x4)).sw(c, c.g[A2][0]);
    let v = c.g[A1][0];
    Addr(r).sw(c, v);
    abi::ret(c);
}

/// `void* LunoVM_PassThrough(void* value)` (0x2224a8): the identity converter.
pub fn luno_vm_pass_through(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    abi::ret(c);
}

/// `void* LunoVMRegister_AsFloat(Register* r)` (0x226618): the float view of
/// a register (the same word).
pub fn luno_vm_register_as_float(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    abi::ret(c);
}

/// `void* LunoVMRegister_AsInt(Register* r)` (0x226610): the integer view of
/// a register (the same word).
pub fn luno_vm_register_as_int(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Luno VM.

/// `void LunoVMRegister_ReadPtr(LunoVMRegister* self, void* stream, int* out)`
/// (0x224c78): read a 32-bit base and a 32-bit offset from the stream and
/// publish `base + offset` at `out[0]` and the offset at `out[1]`.
pub fn luno_vm_register_read_ptr(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[S2][0] = c.g[A0][0];
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0x4;
        c.g[V1][0] = Addr(r32(c, S1)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V1), 0x10)).lh(c);
        c.g[V0][0] = Addr(p(r32(c, V1), 0x14)).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(0x0022_4cb4);
        c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
        call_ptr(c, target, 0x0022_4cb4)?;
        c.g[V1][0] = Addr(r32(c, SP)).lw(c);
        c.g[A1][0] = c.g[SP][0] | 0x4;
        c.g[A2][0] = 0x4;
        c.g[S0][0] = sx(p(r32(c, S0), r32(c, V1)));
        Addr(r32(c, S2)).sw(c, c.g[S0][0]);
        c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        c.g[A0][0] = Addr(p(r32(c, V0), 0x10)).lh(c);
        c.g[V1][0] = Addr(p(r32(c, V0), 0x14)).lw(c);
        let target = r32(c, V1);
        c.g[RA][0] = sx(0x0022_4cdc);
        c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
        call_ptr(c, target, 0x0022_4cdc)?;
        c.g[V0][0] = Addr(p(r32(c, SP), 4)).lw(c);
        frame.restore(c, S0, 0x40);
        Addr(p(r32(c, S2), 4)).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void cLunoTableEntry_dtor(cLunoTableEntry* self, int flags)` (0x226768):
/// release the owned register value, destroy the two embedded registers and
/// free the entry when `flags & 1`.
pub fn c_luno_table_entry_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x48_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0047_9890);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p(r32(c, S0), 0x1c)).sw(c, c.g[V0][0]);
        c.g[A2][0] = Addr(p(r32(c, S0), 0x18)).lw(c);
        let null = c.g[A2][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !null {
            c.cycles += 6;
            c.g[V0][0] = Addr(p(r32(c, A2), 0x1c)).lw(c);
            c.g[A1][0] = 0x3;
            c.g[A0][0] = Addr(p(r32(c, V0), 8)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
            let target = r32(c, V1);
            c.g[RA][0] = sx(0x0022_67ac);
            c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
            call_ptr(c, target, 0x0022_67ac)?;
        }
        c.cycles += 9;
        c.g[A0][0] = sx(p(r32(c, S0), 0xc));
        c.g[RA][0] = sx(0x0022_67b8);
        c.g[A1][0] = 0x2;
        callee::LUNO_VM_REGISTER_DTOR.call(c, 0x0022_67b8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0022_67c4);
        c.g[A1][0] = 0x2;
        callee::LUNO_VM_REGISTER_DTOR.call(c, 0x0022_67c4)?;
        c.g[V0][0] = c.g[S1][0] & 0x1;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0022_67d8);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0031_7e50, 0x0022_67d8)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Luno table.

/// `void LunoTable_dtor(LunoTable* self, int flags)` (0x224d00): call the
/// destructor of every entry in the block at `+0x10` (0x20 bytes each, count at
/// block-0x10), free it, and delete the table when `flags & 1`.
pub fn luno_table_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[V1][0] = lw(c, p(c.g[S1][0] as u32, 0x10));
        let empty = c.g[V1][0] == 0;
        c.g[S2][0] = c.g[A1][0];
        if !empty {
            c.cycles += 5;
            c.g[V0][0] = lw(c, p(c.g[V1][0] as u32, 0xffff_fff0));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            if c.g[V1][0] == c.g[S0][0] {
                c.g[A0][0] = lw(c, p(c.g[S1][0] as u32, 0x10));
            } else {
                c.cycles += 2;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x20));
                loop {
                    c.cycles += 9;
                    c.g[A1][0] = 0;
                    c.g[V0][0] = lw(c, p(c.g[S0][0] as u32, 0x1c));
                    c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 8)).lh(c);
                    c.g[V1][0] = lw(c, p(c.g[V0][0] as u32, 0xc));
                    c.g[V1][0] = sx(c.g[V1][0] as u32);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, c.g[V1][0] as u32, 0x0022_4d58)?;
                    c.g[V0][0] = lw(c, p(c.g[S1][0] as u32, 0x10));
                    let more = c.g[V0][0] != c.g[S0][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x20));
                    if !more {
                        c.cycles += 1;
                        c.g[A0][0] = lw(c, p(c.g[S1][0] as u32, 0x10));
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0022_4d40);
                    }
                }
            }
            c.cycles += 2;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x10));
            call_ptr(c, 0x0031_7e98, 0x0022_4d70)?; // EA_MemFree
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S2][0] & 1;
        let no_delete = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x30);
        if !no_delete {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0022_4d84)?; // __builtin_delete
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void func_00225CA0(cLunoValue* self, sVEntry00224C00** stream, int base)`
/// (0x225ca0): read one Luno value from `stream` into `self`. A stream type of
/// 5 reads a pair, 4 a table, anything else is stored as the value type
/// after calling the stream's "read" slot; a previous table value at `self`
/// is destroyed first.
pub fn luno_vm_register_read(c: &mut Cpu) {
    let frame = Frame::open(c, 0x70);
    'b_225dcc: {
        'b_225dc8: {
            'b_225d44: {
                c.cycles += 20;
                frame.save(c, S0, 0x60);
                frame.save(c, S1, 0x50);
                c.g[S0][0] = c.g[A0][0];
                frame.save(c, S2, 0x40);
                c.g[S1][0] = c.g[A1][0];
                frame.save(c, S3, 0x30);
                c.g[A1][0] = sx(at(c, SP, 0x10));
                frame.save_ra(c, 0x20);
                c.g[S3][0] = c.g[A2][0];
                c.g[A2][0] = 0x4;
                c.g[V1][0] = lw(c, r32(c, S1));
                c.g[A0][0] = lh(c, at(c, V1, 0x10));
                c.g[V0][0] = lw(c, at(c, V1, 0x14));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00225ce0);
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x00225ce0).is_err() {
                    return;
                }
                c.g[S2][0] = lw(c, at(c, SP, 0x10));
                c.g[V0][0] = 0x5;
                let t = c.g[S2][0] != c.g[V0][0];
                c.g[V0][0] = 0x4;
                if t {
                    break 'b_225d44;
                } else {
                    'b_225d28: {
                        c.cycles += 10;
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A2][0] = c.g[S3][0];
                        sw(c, r32(c, SP), 0u64);
                        c.g[A0][0] = c.g[SP][0];
                        c.g[RA][0] = sx(0x00225d08);
                        sw(c, at(c, SP, 0x4), 0u64);
                        if callee::C00224c78.call(c, 0x00225d08).is_err() {
                            return;
                        }
                        c.g[V1][0] = lw(c, at(c, S0, 0x8));
                        c.g[V0][0] = 0x4;
                        if c.g[V1][0] != c.g[V0][0] {
                            sw(c, at(c, S0, 0x8), 0u64);
                            break 'b_225d28;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x00225d24);
                            c.g[A1][0] = 0x2;
                            if callee::C00224df0.call(c, 0x00225d24).is_err() {
                                return;
                            }
                            sw(c, at(c, S0, 0x8), 0u64);
                            break 'b_225d28;
                        }
                    }
                    c.cycles += 7;
                    sw(c, r32(c, S0), 0u64);
                    ldl(c, 2, r32(c, SP));
                    sdl(c, 2, r32(c, S0));
                    let t = true;
                    sw(c, at(c, S0, 0x8), c.g[S2][0]);
                    if t {
                        break 'b_225dc8;
                    } else {
                        break 'b_225d44;
                    }
                }
            }
            'b_225dac: {
                c.cycles += 2;
                if c.g[S2][0] != c.g[V0][0] {
                    sw(c, at(c, S0, 0x8), c.g[S2][0]);
                    break 'b_225dac;
                } else {
                    'b_225d84: {
                        c.cycles += 10;
                        c.g[A0][0] = c.g[SP][0];
                        c.g[RA][0] = sx(0x00225d58);
                        c.g[A1][0] = 0x20;
                        if callee::C00224da0.call(c, 0x00225d58).is_err() {
                            return;
                        }
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A2][0] = c.g[S3][0];
                        c.g[RA][0] = sx(0x00225d68);
                        c.g[A0][0] = c.g[SP][0];
                        if callee::C002259a8.call(c, 0x00225d68).is_err() {
                            return;
                        }
                        c.g[V0][0] = lw(c, at(c, S0, 0x8));
                        if c.g[V0][0] != c.g[S2][0] {
                            sw(c, at(c, S0, 0x8), 0u64);
                            break 'b_225d84;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x00225d80);
                            c.g[A1][0] = 0x2;
                            if callee::C00224df0.call(c, 0x00225d80).is_err() {
                                return;
                            }
                            sw(c, at(c, S0, 0x8), 0u64);
                            break 'b_225d84;
                        }
                    }
                    c.cycles += 10;
                    c.g[A0][0] = c.g[S0][0];
                    sw(c, r32(c, S0), 0u64);
                    c.g[RA][0] = sx(0x00225d94);
                    c.g[A1][0] = c.g[SP][0];
                    if callee::C00225068.call(c, 0x00225d94).is_err() {
                        return;
                    }
                    sw(c, at(c, S0, 0x8), c.g[S2][0]);
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x00225da4);
                    c.g[A1][0] = 0x2;
                    if callee::C00224df0.call(c, 0x00225da4).is_err() {
                        return;
                    }
                    let t = true;
                    frame.restore(c, S0, 0x60);
                    if t {
                        break 'b_225dcc;
                    } else {
                        break 'b_225dac;
                    }
                }
            }
            c.cycles += 7;
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = 0x4;
            c.g[V0][0] = lw(c, r32(c, S1));
            c.g[A0][0] = lh(c, at(c, V0, 0x10));
            c.g[V1][0] = lw(c, at(c, V0, 0x14));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x00225dc8);
            c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
            if call_ptr(c, t, 0x00225dc8).is_err() {
                return;
            }
            break 'b_225dc8;
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x60);
        break 'b_225dcc;
    }
    c.cycles += 6;
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    c.g[RA][0] = ld(c, at(c, SP, 0x20));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x70));
    c.pc = t;
    return;
}




// ---------------------------------------------------------------------------
// LunoVM_Call (0x2224b8)

/// `operator new(size, name, flags, align)` (through the function table).
pub(crate) const LUNO_CALL_NEW_NAMED: u32 = 0x0031_7e30;


/// The allocation name (`D_00479780`).
pub(crate) const LUNO_CALL_NEW_NAME: u32 = 0x0047_9780;


/// The allocation flags.
pub(crate) const LUNO_CALL_NEW_FLAGS: u64 = 0x2000_0000;


/// The VM's default second register (`D_004C9098`, a `cLunoValue`).
pub(crate) const LUNO_CALL_DEFAULT_VALUE: u32 = 0x004c_9098;


/// `cLunoValue`: 12 bytes, `{value (8 bytes, a table pointer for type 4), type}`.
pub(crate) const LUNO_VALUE_SIZE: u64 = 0xc;


pub(crate) const LUNO_VALUE_TYPE: u32 = 0x8;


/// Value types: a table reference (counted) and an 8-byte pair.
pub(crate) const LUNO_TYPE_TABLE: u64 = 4;


pub(crate) const LUNO_TYPE_PAIR: u64 = 5;



pub(crate) const LUNO_CALL_TABLE_DTOR: Callee = Callee {
    addr: 0x0022_4df0,
    f: ee::s05::cLunoTable_dtor,
};


pub(crate) const LUNO_CALL_TABLE_ASSIGN: Callee = Callee {
    addr: 0x0022_5068,
    f: ee::s05::cLunoTable_Assign,
};


pub(crate) const LUNO_CALL_EXECUTE: Callee = Callee {
    addr: 0x0022_27d0,
    f: ee::s05::LunoVM_Execute,
};



/// `void LunoVM_Call(cLunoValue** out, LunoState* state, cLunoValue* table,
/// void* ctx)` (0x2224b8): run a Luno script function. Allocates the
/// register file (`n` values, `n` read from the script's function record at
/// `*state + *(*state + 4) + 8`, a 16-byte count header in front, every
/// value cleared), stores it in `*out`, sets register 0 to a reference to
/// `table` and register 1 to a copy of the VM default value (`D_004C9098`:
/// a table reference is counted, a type-5 pair copied as 8 bytes, anything
/// else as one word), each first releasing an old table reference, then
/// runs `LunoVM_Execute(out, state, ctx)`.
pub fn luno_vm_call(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x90);
        c.g[V1][0] = LUNO_VALUE_SIZE;
        for (reg, off) in [
            (S1, 0x70),
            (S3, 0x50),
            (S4, 0x40),
            (S5, 0x30),
            (S6, 0x20),
            (S7, 0x10),
            (S0, 0x80),
            (S2, 0x60),
        ] {
            frame.save(c, reg, off);
        }
        c.g[S3][0] = c.g[A2][0]; // table
        c.g[S4][0] = c.g[A0][0]; // out
        c.g[S6][0] = c.g[A1][0]; // state
        c.g[S7][0] = c.g[A3][0]; // ctx
        c.g[A2][0] = LUNO_CALL_NEW_FLAGS;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(LUNO_CALL_NEW_NAME);
        c.g[A3][0] = 0;
        c.g[S5][0] = 0x4d_0000;
        // n = the register count of the function record.
        let script = Addr(r32(c, S6)).lw(c);
        let code_off = Addr(script as u32).off(4).lw(c);
        c.g[A0][0] = code_off;
        let record = (script as u32).wrapping_add(code_off as u32);
        c.g[V0][0] = sx(record);
        c.g[S0][0] = Addr(record).off(8).lw(c);
        c.mult(0, V1, S0, V1); // v1 = n * 12
        c.g[A0][0] = sx(r32(c, V1).wrapping_add(0x10));
        call_ptr(c, LUNO_CALL_NEW_NAMED, 0x0022_2524)?;

        // The count header, then clear every value.
        let block = r32(c, V0);
        c.g[S1][0] = sx(block.wrapping_add(0x10));
        let w_s0 = c.g[S0][0];
        Addr(block).sw(c, w_s0);
        c.g[V1][0] = sx(r32(c, S0).wrapping_sub(1));
        c.g[V0][0] = c.g[S1][0];
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[A0][0] = u64::MAX;
            loop {
                c.cycles += 7;
                let value = r32(c, V0);
                Addr(value).off(LUNO_VALUE_TYPE).sw(c, 0);
                c.g[V1][0] = sx(r32(c, V1).wrapping_sub(1));
                Addr(value).sw(c, 0);
                let more = c.g[V1][0] != c.g[A0][0];
                c.g[V0][0] = sx(value.wrapping_add(LUNO_VALUE_SIZE as u32));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0022_2540);
                }
            }
        }

        // Register 0: release an old table reference, then reference `table`.
        c.cycles += 4;
        let regs = r32(c, S1);
        c.g[V0][0] = Addr(regs).off(LUNO_VALUE_TYPE).lw(c);
        c.g[S2][0] = LUNO_TYPE_TABLE;
        let w_s1 = c.g[S1][0];
        Addr(r32(c, S4)).sw(c, w_s1);
        if c.g[V0][0] == LUNO_TYPE_TABLE {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 2;
            LUNO_CALL_TABLE_DTOR.call(c, 0x0022_2578)?;
        }
        c.cycles += 11;
        c.g[A1][0] = c.g[S3][0];
        let regs = r32(c, S1);
        Addr(regs).off(LUNO_VALUE_TYPE).sw(c, 0);
        Addr(regs).sw(c, 0);
        c.g[A0][0] = c.g[S1][0];
        LUNO_CALL_TABLE_ASSIGN.call(c, 0x0022_258c)?;
        c.g[S3][0] = sx(LUNO_CALL_DEFAULT_VALUE);
        c.g[S0][0] = Addr(r32(c, S4)).lw(c);
        let w_s2 = c.g[S2][0];
        Addr(r32(c, S1)).off(LUNO_VALUE_TYPE).sw(c, w_s2);

        // Register 1: release an old table reference.
        let regs = r32(c, S0);
        c.g[V0][0] = Addr(regs).off(LUNO_VALUE_SIZE as u32 + LUNO_VALUE_TYPE).lw(c);
        c.g[S1][0] = sx(regs.wrapping_add(LUNO_VALUE_SIZE as u32));
        if c.g[V0][0] == c.g[S2][0] {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 2;
            LUNO_CALL_TABLE_DTOR.call(c, 0x0022_25b0)?;
        }

        // Register 1 = a copy of the default value.
        c.cycles += 5;
        let reg1 = r32(c, S1);
        Addr(reg1).off(LUNO_VALUE_TYPE).sw(c, 0);
        let regs = r32(c, S0);
        Addr(regs).off(0xc).sw(c, 0);
        let default = Addr(LUNO_CALL_DEFAULT_VALUE);
        c.g[V1][0] = default.off(LUNO_VALUE_TYPE).lw(c);
        c.g[V0][0] = LUNO_TYPE_PAIR;
        if c.g[V1][0] == c.g[S2][0] {
            c.cycles += 5;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S3][0];
            LUNO_CALL_TABLE_ASSIGN.call(c, 0x0022_25d0)?;
        } else {
            c.cycles += 2;
            let pair = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = default.lw(c);
            let regs = r32(c, S0);
            if pair {
                // `ldl/ldr` + `sdl/sdr`: an unaligned 8-byte copy.
                c.cycles += 7;
                c.g[A1][0] = sx(LUNO_CALL_DEFAULT_VALUE);
                c.g[V0][0] = default.ld(c);
                let lo = default.lw(c);
                let hi = default.off(4).lw(c);
                Addr(regs).off(0xc).sw(c, lo);
                Addr(regs).off(0x10).sw(c, hi);
            } else {
                c.cycles += 2;
                let w_v0 = c.g[V0][0];
                Addr(regs).off(0xc).sw(c, w_v0);
            }
        }
        c.g[V0][0] = default.off(LUNO_VALUE_TYPE).lw(c);

        c.cycles += 16;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = c.g[S6][0];
        c.g[A2][0] = c.g[S7][0];
        let w_v0 = c.g[V0][0];
        Addr(r32(c, S1)).off(LUNO_VALUE_TYPE).sw(c, w_v0);
        LUNO_CALL_EXECUTE.call(c, 0x0022_2618)?;
        for (reg, off) in [
            (S0, 0x80),
            (S1, 0x70),
            (S2, 0x60),
            (S3, 0x50),
            (S4, 0x40),
            (S5, 0x30),
            (S6, 0x20),
            (S7, 0x10),
        ] {
            frame.restore(c, reg, off);
        }
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}



pub(crate) const LUNO_TYPE_PTR: u64 = 5;


pub(crate) const LUNO_STREAM_WRITE: u32 = 0x8;



/// `void LunoVMRegister_Write(cLunoValue* self, Stream* stream, int base)`
/// (0x225be8): save a Luno register: write its type tag (4 bytes), then a
/// pointer relative to `base` (`LunoVMRegister_WritePtr`), a table
/// (`cLunoTable_Write`) or the 4-byte payload. Frame 0x40 (s0-s2).
pub fn luno_vm_register_write(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        // stream->Write(&self->type, 4)
        c.g[A1][0] = sx(r32(c, S0).wrapping_add(LUNO_VALUE_TYPE));
        c.g[A2][0] = 4;
        let vt = Addr(r32(c, S1)).lw(c);
        c.g[V1][0] = vt;
        let delta = Addr((vt as u32).wrapping_add(LUNO_STREAM_WRITE)).lh(c);
        let write = Addr((vt as u32).wrapping_add(LUNO_STREAM_WRITE + 4)).lw(c);
        c.g[V0][0] = write;
        c.g[A0][0] = abi::addu(c.g[S1][0], delta);
        call_ptr(c, write as u32, 0x0022_5c24)?;
        let ty = Addr(r32(c, S0).wrapping_add(LUNO_VALUE_TYPE)).lw(c);
        c.g[V1][0] = ty;
        c.g[V0][0] = LUNO_TYPE_TABLE;
        if ty == LUNO_TYPE_PTR {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S2][0];
            Callee { addr: 0x0022_4c00, f: ee::s05::f_00224c00 }.call(c, 0x0022_5c44)?;
        } else {
            c.cycles += 2;
            if ty == LUNO_TYPE_TABLE {
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.g[S2][0];
                Callee { addr: 0x0022_57e0, f: ee::s05::f_002257e0 }.call(c, 0x0022_5c64)?;
            } else {
                // stream->Write(self, 4)
                let vt = Addr(r32(c, S1)).lw(c);
                c.g[V0][0] = vt;
                c.cycles += 7;
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = 4;
                let delta = Addr((vt as u32).wrapping_add(LUNO_STREAM_WRITE)).lh(c);
                let write = Addr((vt as u32).wrapping_add(LUNO_STREAM_WRITE + 4)).lw(c);
                c.g[V1][0] = write;
                c.g[A0][0] = abi::addu(c.g[S1][0], delta);
                call_ptr(c, write as u32, 0x0022_5c84)?;
            }
        }
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}



// ---------------------------------------------------------------------------
// cLunoTable_Assign (0x225068, lunovm.cpp)

/// cLunoTable field: the reference count.
pub(crate) const LTA_REFCOUNT: u32 = 0x8;


/// `cLunoTable_FreeData(cLunoTableRef*)`: destroys and frees the table.
pub(crate) const LTA_FREE_DATA: Callee = Callee {
    addr: 0x0022_4f30,
    f: ee::s05::cLunoTable_FreeData,
};



/// `cLunoTableRef* cLunoTable_Assign(cLunoTableRef* self, cLunoTableRef*
/// other)` (0x225068): reference assignment. When the tables differ, drops
/// the old reference (freeing the table at refcount 0), takes `other`'s
/// table and increments its refcount; returns `self`.
pub fn cluno_table_assign(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[V1][0] = Addr(r32(c, S0)).lw(c);
        c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        let same = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = c.g[S0][0];
        if !same {
            c.cycles += 2;
            if c.g[V1][0] != 0 {
                // Drop the old reference.
                c.cycles += 4;
                let old = c.g[V1][0] as u32;
                c.g[V0][0] = sx((Addr(old.wrapping_add(LTA_REFCOUNT)).lw(c) as u32).wrapping_sub(1));
                let live = c.g[V0][0] != 0;
                Addr(old.wrapping_add(LTA_REFCOUNT)).sw(c, c.g[V0][0]);
                if !live {
                    c.cycles += 2;
                    LTA_FREE_DATA.call(c, 0x0022_50b0)?;
                }
                c.cycles += 1;
            }
            c.g[V1][0] = Addr(r32(c, S1)).lw(c);
            // Take the new one.
            c.cycles += 5;
            let new = c.g[V1][0] as u32;
            Addr(r32(c, S0)).sw(c, c.g[V1][0]);
            c.g[V0][0] = sx((Addr(new.wrapping_add(LTA_REFCOUNT)).lw(c) as u32).wrapping_add(1));
            Addr(new.wrapping_add(LTA_REFCOUNT)).sw(c, c.g[V0][0]);
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}



// ---------------------------------------------------------------------------
// cLunoTable_Write (0x2257e0)

pub(crate) const LUNO_REGISTER_WRITE: Callee = Callee {
    addr: 0x0022_5be8,
    f: ee::s05::LunoVMRegister_Write,
};


/// Table: highest bucket index (-1 without a hash part) and the node array.
pub(crate) const LUNO_TABLE_MODULUS: u32 = 0xc;


pub(crate) const LUNO_TABLE_NODES: u32 = 0x10;


/// Hash node (0x20 bytes): key register (+0), value register (+0xc, its
/// type tag at +0x14) and the chain link (+0x18).
pub(crate) const LUNO_NODE_VALUE: u32 = 0xc;


pub(crate) const LUNO_NODE_VALUE_TYPE: u32 = 0x14;


pub(crate) const LUNO_NODE_NEXT: u32 = 0x18;


pub(crate) const LUNO_NODE_SIZE: u32 = 0x20;



/// `r += 1` on a 32-bit counter register (`addiu r, r, 1`).
pub(crate) fn luno_table_inc(c: &mut Cpu, r: usize) {
    c.g[r][0] = sx((c.g[r][0] as u32).wrapping_add(1));
}



/// `stream->Write(data, n)` (vtable slot 8) returning to `ret`; `vt`/`m`
/// are the registers the original loads the vtable and method into.
pub(crate) fn luno_table_stream_write(c: &mut Cpu, vt: usize, m: usize, ret: u32) -> crate::call::Flow {
    let stream = r32(c, S2);
    c.g[vt][0] = Addr(stream).lw(c);
    let table = r32(c, vt);
    c.g[A0][0] = Addr(table).off(8).lh(c);
    c.g[m][0] = Addr(table).off(0xc).lw(c);
    let method = r32(c, m);
    c.g[A0][0] = sx(stream.wrapping_add(r32(c, A0)));
    call_ptr(c, method, ret)
}



/// `void cLunoTable_Write(cLunoTableRef *self, Stream *stream, int base)`
/// (0x2257e0): save a Luno table: the table pointer (8 bytes), the number
/// of live entries (non-nil values over every bucket and its chain), then
/// each live entry's key and value register (`LunoVMRegister_Write`).
pub fn c_luno_table_write(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S2, 0x60);
        frame.save(c, S3, 0x50);
        c.g[S2][0] = c.g[A1][0]; // stream
        frame.save(c, S6, 0x20);
        c.g[S3][0] = c.g[A2][0]; // base
        frame.save(c, S0, 0x80);
        c.g[S6][0] = c.g[A0][0]; // self
        frame.save(c, S1, 0x70);
        c.g[A2][0] = 8;
        frame.save(c, S4, 0x40);
        frame.save(c, S5, 0x30);
        frame.save_ra(c, 0x10);
        // stream->Write(self->table, 8) (the a1 load sits before the method's).
        let stream = r32(c, S2);
        c.g[V1][0] = Addr(stream).lw(c);
        c.g[A1][0] = Addr(r32(c, S6)).lw(c);
        let vt = r32(c, V1);
        c.g[A0][0] = Addr(vt).off(8).lh(c);
        c.g[V0][0] = Addr(vt).off(0xc).lw(c);
        let method = r32(c, V0);
        c.g[A0][0] = sx(stream.wrapping_add(r32(c, A0)));
        call_ptr(c, method, 0x0022_582c)?;

        // Count the live entries into the local at sp+0 (a1; a3 = bucket,
        // t0 = modulus, a0 = node, a2 = next bucket).
        c.g[V1][0] = Addr(r32(c, S6)).lw(c);
        c.g[A3][0] = 0;
        let count_at = Addr(sp(c));
        count_at.sw(c, 0);
        let table = r32(c, V1);
        c.g[V0][0] = Addr(table).off(LUNO_TABLE_MODULUS).lw(c);
        c.g[A0][0] = Addr(table).off(LUNO_TABLE_NODES).lw(c);
        if (c.g[V0][0] as i64) >= 0 {
            c.cycles += 3;
            c.g[T0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            loop {
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, A0)).off(LUNO_NODE_VALUE_TYPE).lw(c);
                if c.g[V0][0] != 0 {
                    luno_table_inc(c, A1);
                }
                c.cycles += 4;
                let node = r32(c, A0);
                c.g[V1][0] = Addr(node).off(LUNO_NODE_NEXT).lw(c);
                luno_table_inc(c, A3);
                c.g[A2][0] = sx(node.wrapping_add(LUNO_NODE_SIZE));
                if c.g[V1][0] != 0 {
                    // The chain (a0 counts, v1 walks).
                    c.cycles += 3;
                    c.g[A0][0] = c.g[A1][0];
                    c.g[V0][0] = Addr(r32(c, V1)).off(LUNO_NODE_VALUE_TYPE).lw(c);
                    loop {
                        c.cycles += 2;
                        if c.g[V0][0] != 0 {
                            luno_table_inc(c, A0);
                        }
                        c.cycles += 3;
                        c.g[V1][0] = Addr(r32(c, V1)).off(LUNO_NODE_NEXT).lw(c);
                        if c.g[V1][0] != 0 {
                            c.g[V0][0] = Addr(r32(c, V1)).off(LUNO_NODE_VALUE_TYPE).lw(c);
                            if event_due(c) {
                                deliver_event_here(c, 0x0022_5878);
                            }
                        } else {
                            c.cycles += 1;
                            c.g[A1][0] = c.g[A0][0];
                            break;
                        }
                    }
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[T0][0] as i64) < (c.g[A3][0] as i64)) as u64;
                c.g[A0][0] = c.g[A2][0];
                if c.g[V0][0] == 0 {
                    if event_due(c) {
                        deliver_event_here(c, 0x0022_5850);
                    }
                } else {
                    c.cycles += 1;
                    let count = c.g[A1][0];
                    Addr(sp(c)).sw(c, count);
                    break;
                }
            }
        }

        // stream->Write(&count, 4)
        c.cycles += 10;
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 4;
        luno_table_stream_write(c, V0, V1, 0x0022_58bc)?;
        c.g[V0][0] = Addr(sp(c)).lw(c);
        c.g[S5][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, S6)).lw(c);
            let table = r32(c, V0);
            c.g[V1][0] = Addr(table).off(LUNO_TABLE_MODULUS).lw(c);
            c.g[S1][0] = Addr(table).off(LUNO_TABLE_NODES).lw(c);
            if (c.g[V1][0] as i64) >= 0 {
                // s1 = bucket node, s0 = chain node / value, s5 = bucket
                // index, s4 = next bucket.
                loop {
                    c.cycles += 3;
                    let node = r32(c, S1);
                    c.g[V0][0] = Addr(node).off(LUNO_NODE_VALUE_TYPE).lw(c);
                    c.g[S0][0] = sx(node.wrapping_add(LUNO_NODE_VALUE));
                    if c.g[V0][0] != 0 {
                        c.cycles += 11;
                        luno_table_write_entry(c, S1, S0, 0x0022_5900, 0x0022_5910)?;
                    }
                    c.cycles += 4;
                    let node = r32(c, S1);
                    c.g[S0][0] = Addr(node).off(LUNO_NODE_NEXT).lw(c);
                    luno_table_inc(c, S5);
                    c.g[S4][0] = sx(node.wrapping_add(LUNO_NODE_SIZE));
                    if c.g[S0][0] != 0 {
                        c.cycles += 2;
                        c.g[V0][0] = Addr(r32(c, S0)).off(LUNO_NODE_VALUE_TYPE).lw(c);
                        loop {
                            c.cycles += 2;
                            let link = r32(c, S0);
                            c.g[S1][0] = sx(link.wrapping_add(LUNO_NODE_VALUE));
                            if c.g[V0][0] != 0 {
                                c.cycles += 11;
                                luno_table_write_entry(c, S0, S1, 0x0022_594c, 0x0022_595c)?;
                            }
                            c.cycles += 3;
                            c.g[S0][0] = Addr(r32(c, S0)).off(LUNO_NODE_NEXT).lw(c);
                            if c.g[S0][0] != 0 {
                                c.g[V0][0] = Addr(r32(c, S0)).off(LUNO_NODE_VALUE_TYPE).lw(c);
                                if event_due(c) {
                                    deliver_event_here(c, 0x0022_5928);
                                }
                            } else {
                                break;
                            }
                        }
                    }
                    c.cycles += 5;
                    c.g[V1][0] = Addr(r32(c, S6)).lw(c);
                    c.g[V0][0] = Addr(r32(c, V1)).off(LUNO_TABLE_MODULUS).lw(c);
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S5][0] as i64)) as u64;
                    c.g[S1][0] = c.g[S4][0];
                    if c.g[V0][0] == 0 {
                        if event_due(c) {
                            deliver_event_here(c, 0x0022_58d8);
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        c.cycles += 10;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}



/// One live entry: `--count`, then `LunoVMRegister_Write(key)` and
/// `LunoVMRegister_Write(value)` (registers `key`/`value` hold them; `ret_*`
/// are the two return addresses).
pub(crate) fn luno_table_write_entry(
    c: &mut Cpu,
    key: usize,
    value: usize,
    ret_key: u32,
    ret_value: u32,
) -> crate::call::Flow {
    let count_at = Addr(sp(c));
    c.g[V0][0] = count_at.lw(c);
    c.g[A0][0] = c.g[key][0];
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = c.g[S3][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
    let count = c.g[V0][0];
    count_at.sw(c, count);
    LUNO_REGISTER_WRITE.call(c, ret_key)?;
    c.g[A0][0] = c.g[value][0];
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = c.g[S3][0];
    LUNO_REGISTER_WRITE.call(c, ret_value)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::compare;
    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fill;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::SP as STACK;
    pub(crate) use crate::game::ai::ai::tests::leaf;
    pub(crate) use crate::game::ai::ai::tests::leaf_setup;
    pub(crate) use crate::game::ai::ai::tests::rng;
    pub(crate) use crate::game::bx::bxstring::tests::alloc;
    pub(crate) use crate::game::bx::bxstring::tests::heap;
    pub(crate) use crate::game::bx::bxstring2::tests::alloc_mutex;
    pub(crate) use crate::game::bx::bxstring2::tests::MEM_MUTEX;
    pub(crate) use crate::game::dirtysock::tags2::tests::alloc_machine;
    pub(crate) use crate::game::dirtysock::tags2::tests::heap_init_2 as heap_init;
    pub(crate) use crate::game::dirtysock::tags2::tests::pool_allocator;
    pub(crate) use crate::game::dirtysock::tags2::tests::run_2 as run;
    pub(crate) use crate::game::dirtysock::tags2::tests::zero;
    pub(crate) use crate::game::dirtysock::tags2::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::h;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_5 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_4 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_2;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_2;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;
    pub(crate) use crate::game::fe::fepopup::tests::AUX;
    pub(crate) use crate::game::fe::fepopup::tests::BEL;
    pub(crate) use crate::game::fe::fepopup::tests::RACE;
    pub(crate) use crate::game::fe::fepopup::tests::RMAN_2 as RMAN;
    pub(crate) use crate::game::fe::fepopup::tests::SYS;
    pub(crate) use crate::game::fe::festaterewards::tests::heap_block;
    pub(crate) use crate::game::lib::sif::tests::SEEDS;
    pub(crate) use crate::game::movie::movieplayer::tests::globals;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_LIBRARY;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::globals_3 as globals_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::BE_3 as BE;
    pub(crate) use crate::game::wscript::wscriptman::tests::GAME;
    pub(crate) use crate::game::wscript::wscriptman::tests::WORLD_3 as WORLD;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn luno_vm_register_dtor_matches() {
        check(
            ssx3_ee::s05::f_00225b90__generated,
            luno_vm_register_dtor,
            200,
            &|c, seed| {
                let mut r = Rng(0x225b_90 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                let selfp = DATA + 0x100;
                c.g[A0][0] = selfp as u64;
                c.g[A1][0] = ((seed % 2) * 2) as u64;
                c.bus
                    .write32(selfp + 8, if seed % 3 == 0 { 4 } else { r.next_u32() % 4 });
                let vt = DATA + 0x800;
                c.bus.write32(selfp, vt);
                c.bus.write32(vt + 8, 2 + r.next_u32() % 4);
            },
        );
    }

    #[test]
    fn c_luno_table_ctor_matches() {
        check(
            ssx3_ee::s05::f_00224da0__generated,
            c_luno_table_ctor,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0xaf, 6);
                globals(c);
                alloc_mutex(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 8) as u64;
            },
        );
    }

    // ------------------------------------------------ luno / movie / cards

    #[test]
    fn cluno_table_free_data_matches() {
        check(
            ee::s05::f_00224f30__generated,
            cluno_table_free_data,
            60,
            &|c, seed| {
                let _r = alloc_machine(c, seed, 0x224f_30);
                let r = DATA + 0x400;
                zero(c, r, 0x40);
                c.g[A0][0] = sx(r);
                if seed % 2 != 0 {
                    let table = heap_block(c, 0x300);
                    zero(c, table, 0x300);
                    c.bus.write32(r, table);
                } else {
                    c.bus.write32(r, 0);
                }
            },
        );
    }

    #[test]
    fn luno_vm_register_set_raw_matches() {
        leaf(
            ssx3_ee::s05::f_00226600__generated,
            luno_vm_register_set_raw,
            17,
        );
    }

    #[test]
    fn luno_vm_pass_through_matches() {
        leaf(
            ssx3_ee::s05::f_002224a8__generated,
            luno_vm_pass_through,
            18,
        );
    }

    #[test]
    fn luno_vm_register_as_float_matches() {
        leaf(
            ssx3_ee::s05::f_00226618__generated,
            luno_vm_register_as_float,
            19,
        );
    }

    #[test]
    fn luno_vm_register_as_int_matches() {
        leaf(
            ssx3_ee::s05::f_00226610__generated,
            luno_vm_register_as_int,
            20,
        );
    }

    // ------------------------------------------------------------------
    // Luno VM.

    #[test]
    fn luno_vm_register_read_ptr_matches() {
        check(
            ssx3_ee::s05::f_00224c78__generated,
            luno_vm_register_read_ptr,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x224c78, 4);
                be_library(c);
                world(c);
                let reg = DATA + 0x100;
                let stream = DATA + 0x200;
                let out = DATA + 0x300;
                c.g[A0][0] = reg as u64;
                c.g[A1][0] = stream as u64;
                c.g[A2][0] = out as u64;
                let vt = DATA + 0x400;
                w(c, stream, vt);
                slot(c, vt, 0x10, 0, FAKE);
                slot(c, vt, 0x14, 0, FAKE + 0x10);
                w(c, out, r.next_u32());
                w(c, out + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn c_luno_table_entry_dtor_matches() {
        check(
            ssx3_ee::s05::f_00226768__generated,
            c_luno_table_entry_dtor,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x226768, 4);
                be_library(c);
                world(c);
                let entry = DATA + 0x100;
                c.g[A0][0] = entry as u64;
                c.g[A1][0] = ((seed % 2) * 2) as u64;
                if seed % 3 == 0 {
                    w(c, entry + 0x18, 0);
                } else {
                    let child = DATA + 0x400;
                    let vt = DATA + 0x800;
                    w(c, entry + 0x18, child);
                    w(c, child + 0x1c, vt);
                    slot(c, vt, 8, 0, FAKE);
                }
                let _ = r.next_u32();
            },
        );
    }

    // ------------------------------------------------------------------ luno

    #[test]
    fn luno_table_dtor_matches() {
        check(
            ssx3_ee::s05::f_00224d00__generated,
            luno_table_dtor,
            8,
            &|c, seed| {
                let _ = base(c, seed, 0x0022_4d00);
                heap(c);
                let table = DATA + 0x100;
                clear(c, table, 0x100);
                let n = seed % 3;
                let block = alloc(c, 0x20 * (n + 1) + 0x10);
                let block = if block == 0 { DATA + 0x2000 } else { block };
                clear(c, block, 0x20 * (n + 1) + 0x10);
                w(c, block, n);
                let vtable = DATA + 0x3000;
                clear(c, vtable, 0x40);
                c.bus.write16(vtable + 8, 0);
                w(c, vtable + 0xc, crate::difftest::fakes::FAKE);
                for i in 0..n {
                    let e = block + 0x10 + 0x20 * (n - 1 - i);
                    w(c, e + 0x1c, vtable);
                }
                w(c, table + 0x10, block + 0x10);
                c.g[A0][0] = sx(table);
                c.g[A1][0] = (seed % 4) as u64;
            },
        );
    }

    pub(crate) const LUNO_VALUE: u32 = DATA + 0x1_0000;

    pub(crate) const LUNO_STREAM: u32 = DATA + 0x1_1000;

    #[test]
    fn luno_vm_register_read_matches() {
        check_all(
            ee::s05::f_00225ca0__generated,
            luno_vm_register_read,
            30,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x225c_a0, 8);
                c.g[A0][0] = sx(LUNO_VALUE);
                c.g[A1][0] = sx(LUNO_STREAM);
                c.g[A2][0] = sx(seed % 4);
                c.bus.write32(LUNO_VALUE, r.next_u32());
                c.bus.write32(LUNO_VALUE + 0x8, (seed % 6) as u32); // old type
                if seed % 2 == 0 {
                    let vt0 = vtable(c, &mut r, &[0x10, 0x14]);
                    c.bus.write32(LUNO_STREAM, vt0);
                }
            },
        );
    }
    use super::*;
    use crate::difftest::fakes;
    use crate::game::visualfx::renderstateman::tests::table_getter_setup;


    /// The 16-pool-node classes of `pool_allocator` (block >= 16) filled so
    /// a value read past the register file looks like a counted table
    /// reference (type 4 at +4/+8, a table pointer at +0/+0xc of each 16
    /// bytes), which makes the "release the old reference" branches reachable.
    pub(crate) fn luno_call_fill_pools(c: &mut Cpu, table: u32) {
        let mut nodes = DATA + 0x24_000;
        for block in [8u32, 0x10, 0x20, 0x40, 0x80, 0x100, 0x200] {
            if block >= 0x10 {
                for n in 0..16 {
                    let node = nodes + block * n;
                    for off in (4..block).step_by(4) {
                        let v = match off % 16 {
                            4 | 8 => 4,
                            _ => table,
                        };
                        c.bus.write32(node + off, v);
                    }
                }
            }
            nodes += block * 16;
        }
    }


    /// A script whose function record has `n` registers and no code (the
    /// interpreter returns at once and frees the register file), the
    /// allocator fixtures, and the default value of each type.
    #[test]
    fn luno_vm_call_matches() {
        use crate::difftest::check;
        use crate::game::worldpainter::quadtree::tests::alloc_machine;
        check(ee::s05::f_002224b8__generated, luno_vm_call, 120, &|c, seed| {
            let mut r = alloc_machine(c, seed, 0x2224_b8);
            let state = DATA + 0x1000;
            let script = DATA + 0x1100;
            let out = DATA + 0x1200;
            let table = DATA + 0x1800; // refcount at +8
            let table2 = DATA + 0x1900;
            let gc_table = DATA + 0x1a00; // what the pool fill points at
            c.bus.write32(table + 8, 100);
            c.bus.write32(table2 + 8, 100);
            c.bus.write32(gc_table + 8, 100);
            if seed % 2 == 0 {
                luno_call_fill_pools(c, gc_table);
            }
            // state = {script, function 0}; script+4 = code offset 0x10, the
            // record at script+0x10: {_, start pc 0, n registers, 0 locals}.
            c.bus.write32(state, script);
            c.bus.write32(state + 4, 0);
            c.bus.write32(script + 4, 0x10);
            c.bus.write32(script + 0x14, 0);
            c.bus.write32(script + 0x18, seed % 4); // n = 0..3
            c.bus.write32(script + 0x1c, 0);
            // the table argument: a cLunoValue whose word 0 is the table.
            let arg = DATA + 0x1300;
            c.bus.write32(arg, table);
            // the default value: a table reference, a pair, or a plain word.
            let d = LUNO_CALL_DEFAULT_VALUE;
            let (word0, ty) = match seed / 4 % 3 {
                0 => (table2, 4),
                1 => (r.next_u32(), 5),
                _ => (r.next_u32(), 2),
            };
            c.bus.write32(d, word0);
            c.bus.write32(d + 4, r.next_u32());
            c.bus.write32(d + 8, ty);
            c.g[A0][0] = sx(out);
            c.g[A1][0] = sx(state);
            c.g[A2][0] = sx(arg);
            c.g[A3][0] = sx(DATA + 0x1400);
            c.pc = 0x0022_24b8;
        });
    }
    use super::*;
    use super::*;
    use crate::difftest::{put_vec};
    use super::*;
    // (fakes imported above)


    // -- 0x00225be8 LunoVMRegister_Write ---------------------------------------

    /// `LunoVMRegister_Write` against the generated body: type tags 5
    /// (the real `LunoVMRegister_WritePtr`), 4 (the real `cLunoTable_Write`
    /// over a table with no hash part and no entries) and others (payload
    /// written directly). The stream's Write virtual is a fake (some calls
    /// unwind).
    #[test]
    fn luno_vm_register_write_matches() {
        use crate::difftest::fakes::{covered, machine, vtable};
        use crate::difftest::DATA;
        const VALUE: u32 = DATA + 0x20_0000;
        const STREAM: u32 = DATA + 0x20_0100;
        const TABLE: u32 = DATA + 0x20_0200;
        covered(ee::s05::f_00225be8__generated, luno_vm_register_write, 300, &|c, seed| {
            let mut r = machine(c, seed, 0x0022_5be8, 3);
            c.pc = 0x0022_5be8;
            let vt = vtable(c, &mut r, &[LUNO_STREAM_WRITE]);
            c.bus.write32(STREAM, vt);
            let ty = [4, 5, 4, 5, 0, 1, 2, 3, 6][(r.next_u32() % 9) as usize];
            c.bus.write32(VALUE + LUNO_VALUE_TYPE, ty);
            c.bus.write32(VALUE, if ty == 4 { TABLE } else { r.next_u32() });
            c.bus.write32(VALUE + 4, r.next_u32());
            // Table: no hash part (+0xc = -1), so nothing to count or write.
            c.bus.write32(TABLE + 0xc, u32::MAX);
            c.bus.write32(TABLE + 0x10, TABLE + 0x40);
            c.g[A0][0] = sx(VALUE);
            c.g[A1][0] = sx(STREAM);
            c.g[A2][0] = r.next_u32() as u64;
        });
    }


    /// cLunoTable_Assign: the same table, a null old reference, an old
    /// reference that stays alive (refcount > 1) and one whose last
    /// reference is dropped (a real zeroed heap table freed by the real
    /// cLunoTable_FreeData on the pool allocator / dlmalloc arena).
    #[test]
    fn cluno_table_assign_matches() {
        use crate::game::dirtysock::tags2::tests::zero;
        use crate::game::fe::festaterewards::tests::heap_block;
        use crate::game::worldpainter::quadtree::tests::alloc_machine;
        const SELF: u32 = DATA + 0x400;
        const OTHER: u32 = DATA + 0x440;
        const OLD: u32 = DATA + 0x800;
        const NEW: u32 = DATA + 0x900;
        check(ee::s05::f_00225068__generated, cluno_table_assign, 120, &|c, seed| {
            let mut r = alloc_machine(c, seed, 0x22_5068);
            c.pc = 0x0022_5068;
            zero(c, SELF, 0x80);
            c.g[A0][0] = sx(SELF);
            c.g[A1][0] = sx(OTHER);
            c.bus.write32(NEW + LTA_REFCOUNT, r.next_u32() % 50);
            c.bus.write32(OTHER, NEW);
            match seed % 4 {
                0 => c.bus.write32(SELF, NEW),
                1 => c.bus.write32(SELF, 0),
                2 => {
                    c.bus.write32(SELF, OLD);
                    c.bus.write32(OLD + LTA_REFCOUNT, 2 + r.next_u32() % 50);
                }
                _ => {
                    let table = heap_block(c, 0x300);
                    zero(c, table, 0x300);
                    c.bus.write32(table + LTA_REFCOUNT, 1);
                    c.bus.write32(SELF, table);
                }
            }
        });
    }


    // ------------------------------------------------------------ cLunoTable_Write

    /// Tables without a hash part, or with 1..4 buckets whose nodes and
    /// chains (0..3 links) hold nil and non-nil values; plain register
    /// types only (the real `LunoVMRegister_Write` writes them directly).
    /// The stream's Write is a fake (some calls unwind).
    #[test]
    fn c_luno_table_write_matches() {
        covered(
            ee::s05::f_002257e0__generated,
            c_luno_table_write,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0022_57e0, 8);
                c.pc = 0x0022_57e0;
                let this = DATA + 0x5_0000;
                let stream = DATA + 0x5_0100;
                let table = DATA + 0x5_0200;
                let nodes = DATA + 0x5_1000;
                let links = DATA + 0x5_2000;
                let vt = fakes::vtable(c, &mut r, &[8]);
                c.bus.write32(stream, vt);
                c.bus.write32(this, table);
                let modulus = [-1i32, 0, 1, 3][(seed % 4) as usize];
                c.bus.write32(table + LUNO_TABLE_MODULUS, modulus as u32);
                c.bus.write32(table + LUNO_TABLE_NODES, nodes);
                let types = [0u32, 0, 1, 2, 3, 6];
                let node = |c: &mut Cpu, r: &mut Rng, at: u32, next: u32| {
                    c.bus.write32(at, r.next_u32());
                    c.bus.write32(at + 8, types[r.next_u32() as usize % types.len()]);
                    c.bus.write32(at + LUNO_NODE_VALUE, r.next_u32());
                    c.bus
                        .write32(at + LUNO_NODE_VALUE_TYPE, types[r.next_u32() as usize % types.len()]);
                    c.bus.write32(at + LUNO_NODE_NEXT, next);
                };
                let mut link = links;
                for b in 0..4u32 {
                    let mut next = 0;
                    for _ in 0..r.next_u32() % 4 {
                        node(c, &mut r, link, next);
                        next = link;
                        link += LUNO_NODE_SIZE;
                    }
                    node(c, &mut r, nodes + LUNO_NODE_SIZE * b, next);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(stream);
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }
}
