//! `src/luno/lunovm.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
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
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::Cpu;
use crate::call::VSlot;
use crate::abi::r32;
use crate::call::sp;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
pub(crate) use crate::game::render::lightman::off;
pub(crate) use crate::game::render::lightman::Oc49Mem;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw;
pub(crate) use crate::game::fe::fepopup::at;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw;
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::be::beintnetwork::ldl;
pub(crate) use crate::game::be::beintnetwork::sdl;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0022_24a8, luno_vm_pass_through, "LunoVM_PassThrough"),
    (0x0022_4c78, luno_vm_register_read_ptr, "LunoVMRegister_ReadPtr"),
    (0x0022_4d00, luno_table_dtor, "LunoTable_dtor"),
    (0x0022_4da0, c_luno_table_ctor, "cLunoTable_ctor"),
    (0x0022_4f30, cluno_table_free_data, "cLunoTable_FreeData"),
    (0x0022_5b90, luno_vm_register_dtor, "LunoVMRegister_dtor"),
    (0x0022_5ca0, luno_vm_register_read, "LunoVMRegister_Read"),
    (0x0022_6600, luno_vm_register_set_raw, "LunoVMRegister_SetRaw"),
    (0x0022_6610, luno_vm_register_as_int, "LunoVMRegister_AsInt"),
    (0x0022_6618, luno_vm_register_as_float, "LunoVMRegister_AsFloat"),
    (0x0022_6768, c_luno_table_entry_dtor, "cLunoTableEntry_dtor"),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::render::lightman::callee::LUNO_TABLE_DTOR;
    pub(crate) use crate::game::render::lightman::callee::WORLD_GET_FLAG_MANAGER;
    pub(crate) use crate::game::render::lightman::callee::WORLD_GET_AI_FRAME_COUNT;
    pub(crate) use crate::game::render::lightman::callee::WORLD_GET_NASH;
    pub(crate) use crate::game::render::lightman::callee::LIGHT_CONTRIBUTION;
    pub(crate) use crate::game::render::lightman::callee::MATH_SIN_COS;
    pub(crate) use crate::game::render::lightman::callee::SPATIAL_HASH_UPDATE;
    pub(crate) use crate::game::render::lightman::callee::FLAG_MANAGER_GROUP_COMPUTE_VERTS;
    pub(crate) use crate::game::render::lightman::callee::INTERSECT_INFO_EVALUATE;
    pub(crate) use crate::game::render::lightman::callee::TYPED_PROCESS_SAVE;
    pub(crate) use crate::game::render::lightman::callee::SYNC_PROCESS_SAVE;
    pub(crate) use crate::game::render::lightman::callee::MULTI_SPLINE_MODIFIER_DEBUG_DRAW_PATH;
    pub(crate) use crate::game::render::lightman::callee::EA_MEMSET;
    pub(crate) use crate::game::render::lightman::callee::DMA_WAIT_FROM_SCRATCHPAD;
    pub(crate) use crate::game::render::lightman::callee::DMA_WAIT_TO_SCRATCHPAD;
    pub(crate) use crate::game::bx::bxstring2::callee::FE_MODULE_RIDER_PREVIEW_REQUEST_CHAR;
    pub(crate) use crate::game::bx::bxstring2::callee::C_FE_STATE_LOBBY_MENU_FILL_PLAYER_LIST;
    pub(crate) use crate::game::bx::bxstring2::callee::C_FE_STATE_LOBBY_MENU_UPDATE_PLAYER_SCROLL_ARROWS;
    pub(crate) use crate::game::bx::bxstring2::callee::EVENT_SELECT_ON_ACTIVATE;
    pub(crate) use crate::game::bx::bxstring2::callee::EVENT_SELECT_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::C_OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::C_LUNO_TABLE_ALLOC;
    pub(crate) use crate::game::bx::bxstring2::callee::CROWD_MAN2D_REMOVE_ALL;
    pub(crate) use crate::game::bx::bxstring2::callee::CROWD_MAN2D_RESET;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_INIT;
    pub(crate) use crate::game::bx::bxstring2::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::bx::bxstring2::callee::VID_ENGINE_PLAYER_START_PLAYBACK;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_LIST_PREPARE;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_LIST_SET_PAUSED_NESTED;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_LIST_GET_ENTRY_TYPE;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_STOP_MAIN_LISTS;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_STOP_AUX_LISTS;
    pub(crate) use crate::game::bx::bxstring2::callee::SNAPSHOT_LIST_REMOVE;
    pub(crate) use crate::game::bx::bxstring2::callee::PATH_DATA_PARSE;
    pub(crate) use crate::game::bx::bxstring2::callee::WORLD_STATE_RESTORE;
    pub(crate) use crate::game::bx::bxstring2::callee::EA_MEM_ALLOC_NAMED;
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
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE_2 as BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE_2 as BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_CAMERA_SET_CONTROLLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MENU_ON_LOBBY_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_FORWARD_EVENT_TO_UI;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::LUNO_TABLE_DTOR as LUNO_TABLE_DTOR_2;
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
    pub(crate) use crate::game::fe::fepopup::callee::C00113f88;
    pub(crate) use crate::game::fe::fepopup::callee::C00115ab0;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fe78;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fe98;
    pub(crate) use crate::game::fe::fepopup::callee::C0011fec8;
    pub(crate) use crate::game::fe::fepopup::callee::C00125108;
    pub(crate) use crate::game::fe::fepopup::callee::C0014a080;
    pub(crate) use crate::game::fe::fepopup::callee::C0014a0e0;
    pub(crate) use crate::game::fe::fepopup::callee::C0014bdb8;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d448;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d988;
    pub(crate) use crate::game::fe::fepopup::callee::C0014d998;
    pub(crate) use crate::game::fe::fepopup::callee::C0014dc80;
    pub(crate) use crate::game::fe::fepopup::callee::C0014dd58;
    pub(crate) use crate::game::fe::fepopup::callee::C00150928;
    pub(crate) use crate::game::fe::fepopup::callee::C00150b48;
    pub(crate) use crate::game::fe::fepopup::callee::C00151c48;
    pub(crate) use crate::game::fe::fepopup::callee::C001535c0;
    pub(crate) use crate::game::fe::fepopup::callee::C001543e0;
    pub(crate) use crate::game::fe::fepopup::callee::C00156988;
    pub(crate) use crate::game::fe::fepopup::callee::C00156a60;
    pub(crate) use crate::game::fe::fepopup::callee::C001577e0;
    pub(crate) use crate::game::fe::fepopup::callee::C00157bf0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158910;
    pub(crate) use crate::game::fe::fepopup::callee::C001589b0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158a50;
    pub(crate) use crate::game::fe::fepopup::callee::C00158af0;
    pub(crate) use crate::game::fe::fepopup::callee::C00158c80;
    pub(crate) use crate::game::fe::fepopup::callee::C00158d58;
    pub(crate) use crate::game::fe::fepopup::callee::C00159170;
    pub(crate) use crate::game::fe::fepopup::callee::C00159cd0;
    pub(crate) use crate::game::fe::fepopup::callee::C0015a2e0;
    pub(crate) use crate::game::fe::fepopup::callee::C0016c6b0;
    pub(crate) use crate::game::fe::fepopup::callee::C0016c718;
    pub(crate) use crate::game::fe::fepopup::callee::C00176890;
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
    pub(crate) use crate::game::fe::fepopup::callee::C00225068;
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
    pub(crate) use crate::game::fe::fepopup::callee::C00275498;
    pub(crate) use crate::game::fe::fepopup::callee::C002758c0;
    pub(crate) use crate::game::fe::fepopup::callee::C00275b98;
    pub(crate) use crate::game::fe::fepopup::callee::C00275cf8;
    pub(crate) use crate::game::fe::fepopup::callee::C00278308;
    pub(crate) use crate::game::fe::fepopup::callee::C00278e50;
    pub(crate) use crate::game::fe::fepopup::callee::C00278f38;
    pub(crate) use crate::game::fe::fepopup::callee::C00278f68;
    pub(crate) use crate::game::fe::fepopup::callee::C00281ed8;
    pub(crate) use crate::game::fe::fepopup::callee::C00283228;
    pub(crate) use crate::game::fe::fepopup::callee::C00283518;
    pub(crate) use crate::game::fe::fepopup::callee::C002837c8;
    pub(crate) use crate::game::fe::fepopup::callee::C0028b180;
    pub(crate) use crate::game::fe::fepopup::callee::C0028bb10;
    pub(crate) use crate::game::fe::fepopup::callee::C0028e8c0;
    pub(crate) use crate::game::fe::fepopup::callee::C002913d8;
    pub(crate) use crate::game::fe::fepopup::callee::C003457c8;
    pub(crate) use crate::game::fe::fepopup::callee::C00358020;
    pub(crate) use crate::game::fe::fepopup::callee::C00362478;
    pub(crate) use crate::game::fe::fepopup::callee::C003625c0;
    pub(crate) use crate::game::fe::fepopup::callee::C00362660;
    pub(crate) use crate::game::fe::fepopup::callee::C003626d8;
    pub(crate) use crate::game::fe::fepopup::callee::C003627a8;
    pub(crate) use crate::game::fe::fepopup::callee::C00364cd0;
    pub(crate) use crate::game::fe::fepopup::callee::C00366ce0;
    pub(crate) use crate::game::fe::fepopup::callee::C00368170;
    pub(crate) use crate::game::fe::fepopup::callee::C003684f0;
    pub(crate) use crate::game::fe::fepopup::callee::C00368970;
    pub(crate) use crate::game::fe::fepopup::callee::C0038f460;
    pub(crate) use crate::game::fe::fepopup::callee::C0038f668;
    pub(crate) use crate::game::fe::fepopup::callee::C0039c870;
    pub(crate) use crate::game::fe::fepopup::callee::C0039ff78;
    pub(crate) use crate::game::fe::fepopup::callee::C003b1300;
    pub(crate) use crate::game::fe::fepopup::callee::C003b2360;
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

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::put_words;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fill;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::elf_image;
    use crate::difftest::scramble;
    use crate::difftest::SP as STACK;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::compare;
    use ssx3_ee_rt::funcs::GuestFn;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_LIBRARY;
    pub(crate) use crate::game::bx::bxstring2::tests::MEM_MUTEX;
    pub(crate) use crate::game::movie::movieplayer::tests::globals;
    pub(crate) use crate::game::bx::bxstring2::tests::alloc_mutex;
    pub(crate) use crate::game::dirtysock::tags2::tests::pool_allocator;
    pub(crate) use crate::game::dirtysock::tags2::tests::MEM_HEAP;
    pub(crate) use crate::game::dirtysock::tags2::tests::run_2 as run;
    pub(crate) use crate::game::dirtysock::tags2::tests::heap_init_2 as heap_init;
    pub(crate) use crate::game::fe::festaterewards::tests::heap_block;
    pub(crate) use crate::game::dirtysock::tags2::tests::alloc_machine;
    pub(crate) use crate::game::dirtysock::tags2::tests::zero;
    pub(crate) use crate::game::lib::sif::tests::SEEDS;
    pub(crate) use crate::game::ai::ai::tests::rng;
    pub(crate) use crate::game::ai::ai::tests::leaf_setup;
    pub(crate) use crate::game::ai::ai::tests::leaf;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::h;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_5 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_4 as world;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::BE_3 as BE;
    pub(crate) use crate::game::wscript::wscriptman::tests::WORLD_3 as WORLD;
    pub(crate) use crate::game::wscript::wscriptman::tests::GAME;
    pub(crate) use crate::game::wscript::wscriptman::tests::globals_3 as globals_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base;
    pub(crate) use crate::game::bx::bxstring::tests::heap;
    pub(crate) use crate::game::bx::bxstring::tests::alloc;
    pub(crate) use crate::game::fe::fepopup::tests::BEL;
    pub(crate) use crate::game::fe::fepopup::tests::AUX;
    pub(crate) use crate::game::fe::fepopup::tests::SYS;
    pub(crate) use crate::game::fe::fepopup::tests::RACE;
    pub(crate) use crate::game::fe::fepopup::tests::RMAN_2 as RMAN;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_2;


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
        leaf(ssx3_ee::s05::f_00226600__generated, luno_vm_register_set_raw, 17);
    }


    #[test]
    fn luno_vm_pass_through_matches() {
        leaf(ssx3_ee::s05::f_002224a8__generated, luno_vm_pass_through, 18);
    }


    #[test]
    fn luno_vm_register_as_float_matches() {
        leaf(ssx3_ee::s05::f_00226618__generated, luno_vm_register_as_float, 19);
    }


    #[test]
    fn luno_vm_register_as_int_matches() {
        leaf(ssx3_ee::s05::f_00226610__generated, luno_vm_register_as_int, 20);
    }


        // ------------------------------------------------------------------
    // Luno VM.

    #[test]
    fn luno_vm_register_read_ptr_matches() {
        check(ssx3_ee::s05::f_00224c78__generated, luno_vm_register_read_ptr, SEEDS_2, &|c, seed| {
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
        });
    }


    #[test]
    fn c_luno_table_entry_dtor_matches() {
        check(ssx3_ee::s05::f_00226768__generated, c_luno_table_entry_dtor, SEEDS_2, &|c, seed| {
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
        });
    }


    // ------------------------------------------------------------------ luno

    #[test]
    fn luno_table_dtor_matches() {
        check(ssx3_ee::s05::f_00224d00__generated, luno_table_dtor, 8, &|c, seed| {
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
        });
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
}
