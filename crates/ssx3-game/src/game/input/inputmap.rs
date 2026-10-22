//! `src/input/inputmap.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::call::Flow;
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::vu0::Src;
use crate::abi::r32;
use crate::call::sp;
use crate::call::VSlot;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::events::deliver_event;
use ssx3_ee_rt::vu0::X;
pub(crate) use crate::game::render::lightman::off;
pub(crate) use crate::game::render::lightman::Oc49Mem;
pub(crate) use crate::game::render::lightman::addu;
pub(crate) use crate::game::sound::bankmonitor::p;
pub(crate) use crate::game::fe::fereal::open;
pub(crate) use crate::game::fe::fereal::close;
pub(crate) use crate::game::fe::feasyncfile::at;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::feasyncfile::shl32;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0032_0ca0, input_map_init, "InputMap_Init"),
    (0x0032_0f10, input_map_finalize_opcodes, "InputMap_FinalizeOpcodes"),
    (0x0032_0fa8, input_map_eval_float, "InputMap_EvalFloat"),
    (0x0032_1108, input_map_eval_bool, "InputMap_EvalBool"),
    (0x0032_1298, input_pad_update, "InputPad_Update"),
    (0x0032_1500, input_parser_error, "InputParser_Error"),
    (0x0032_15f8, input_parser_alloc_node, "InputParser_AllocNode"),
    (0x0032_16e8, input_parser_lookup_button, "InputParser_LookupButton"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::be::beintnetwork::callee::MEM_MAN_ALLOC;
    pub(crate) use crate::game::be::beintnetwork::callee::C_BOLTON_FX_REFRESH_EMITTERS;
    pub(crate) use crate::game::be::beintnetwork::callee::IRRADIANCE_REF_GET_ENTRY;
    pub(crate) use crate::game::be::beintnetwork::callee::WORLD_PAINTER_GET_IRRADIANCE_NAME0;
    pub(crate) use crate::game::be::beintnetwork::callee::WORLD_PAINTER_GET_IRRADIANCE_NAME1;
    pub(crate) use crate::game::be::beintnetwork::callee::PATH_QUERY_EVENTS;
    pub(crate) use crate::game::be::beintnetwork::callee::SSX_AUDIO_SYSTEM_GET_APP;
    pub(crate) use crate::game::be::beintnetwork::callee::SSX_AUDIO_SYSTEM_GET_VIEW_COUNT;
    pub(crate) use crate::game::be::beintnetwork::callee::INPUT_VM_INIT;
    pub(crate) use crate::game::be::beintnetwork::callee::INPUT_VM_EXECUTE;
    pub(crate) use crate::game::be::beintnetwork::callee::INPUT_MAP_EVAL_FLOAT;
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
    pub(crate) use crate::game::sound::bankmonitor::callee::GAME_STATE_GOTO_NEXT_2 as GAME_STATE_GOTO_NEXT;
    pub(crate) use crate::game::sound::bankmonitor::callee::PS2_MEMCARD_CREATE;
    pub(crate) use crate::game::sound::bankmonitor::callee::LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::sound::bankmonitor::callee::BUDDY_MAN_CONNECT;
    pub(crate) use crate::game::sound::bankmonitor::callee::STR_CASE_CMP;
    pub(crate) use crate::game::sound::bankmonitor::callee::SLOGIN_MC_GET_PORT_SLOT;
    pub(crate) use crate::game::sound::bankmonitor::callee::MEMCARD_REQUEST_GET_INFO_NOW;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_INT;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_VEC3;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_FLOAT_2 as PATH_DATA_READ_FLOAT;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_LIST_ADD_CHOICE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS;
    pub(crate) use crate::game::sound::bankmonitor::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SSX_AUDIO_SYSTEM_END_SCRIPT_MUSIC;
    pub(crate) use crate::game::sound::bankmonitor::callee::WORLD_PAINTER_SET_SCRIPT_SURFACE_MODE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SND_BANK_MAN_UNLOAD_MEMORY_BANK;
    pub(crate) use crate::game::sound::bankmonitor::callee::SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::sound::bankmonitor::callee::MUSIC_PLAYER_GET_SONG_TAG;
    pub(crate) use crate::game::sound::bankmonitor::callee::SPEECH_SYSTEM_STOP_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::BANK_MONITOR_TIMER_KILL;
    pub(crate) use crate::game::sound::bankmonitor::callee::SNDSTRM_SET_PREBUFFER;
    pub(crate) use crate::game::sound::bankmonitor::callee::AUDIO_RAND;
    pub(crate) use crate::game::sound::bankmonitor::callee::SONG_INSTANCE_GET_REQUEST_STATUS;
    pub(crate) use crate::game::sound::bankmonitor::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::sound::bankmonitor::callee::MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_MAN_CALL_TASK_FUNC;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_PROCESS_LIST_REMOVE;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_LIBRARY_GET_INSTANCE_3 as BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_MISSION_INT_GET_COLLECTED_COUNT;
    pub(crate) use crate::game::sound::bankmonitor::callee::ADF_DATA_SOURCE_SET_DATA;
    pub(crate) use crate::game::sound::bankmonitor::callee::EA_FILE_LOAD_DECOMPRESSED;
    pub(crate) use crate::game::sound::bankmonitor::callee::GAME_ANIM_MAP_FIND_HASH;
    pub(crate) use crate::game::sound::bankmonitor::callee::MEMSET_2 as MEMSET;
    pub(crate) use crate::game::sound::bankmonitor::callee::CROWD_MAN3D_GROUP_CREATE_MESHES;
    pub(crate) use crate::game::sound::bankmonitor::callee::TEX_MAN_FREE_TEXTURE;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_LIST_COUNT;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_MENU_SET_NAV_BUTTONS;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_OBJECT_LOAD;
    pub(crate) use crate::game::sound::bankmonitor::callee::SECTION_ALLOCATOR_DTOR;
    pub(crate) use crate::game::sound::bankmonitor::callee::STREAM_FILE_RELEASE_CHUNK;
    pub(crate) use crate::game::sound::bankmonitor::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE;
    pub(crate) use crate::game::sound::bankmonitor::callee::RIDER_METRIX_SHOW_HUD_NUMBER;
    pub(crate) use crate::game::sound::bankmonitor::callee::RIDER_METRIX_CLEAR_PEAK_A4;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_GET_EVENT_RECORD_INDEX;
    pub(crate) use crate::game::sound::bankmonitor::callee::BOLTS_FREE_DB;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL_2 as UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_4 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_2 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_3 as SSX_AUDIO_SYSTEM_GET_INSTANCE_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_INSTANCE_3 as SCRIPT_ENGINE_GET_INSTANCE_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_FREE_4 as EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_3 as NET_FE_STATE_GET_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHAT_ROOM_SELECT_PLAYER_BY_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_AGREEMENT_TERMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_RELEASE_SNAPSHOT_LISTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_DECLINE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_PICK_RANDOM_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_COUNT_SAVE_SNAPSHOTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_CHANNEL_GET_LENGTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_PUSH_2 as ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::CLEAR_MUSIC_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_CONFIGURE_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_UPDATE_EMITTER_ENABLES;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_PROFILE_5 as BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BENEW_PLAYER_INT_GET_SLOT_2 as BENEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_CHAR_PROFILE_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_RAND_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_IS_FLAG4;
    pub(crate) use crate::game::fe::feasyncfile::callee::DATA_INSTANCE_FREE_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_4 as SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_3 as SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE_2 as SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_SET_PAUSED;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_DESTROY_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATTACHED_EMITTER_FX_DESTROY_EMITTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_IS_HULL_FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SYNC_PROCESS_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MDF_FILE_CREATE_MODEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_START_PROFILE_SEARCH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_FINISH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_6 as UI_LIST_FIRST_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_DRAW_RECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_RESET_PROFILE_CHARACTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COPY_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_CREATE_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_LEVEL_TO_VOLUME_2 as AUDIO_LEVEL_TO_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_CHANNEL_VOLUME_2 as AUDIO_SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCPY_2 as MEMCPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCASECMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCROLL_SET_PARTS_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_GET_OBJECT_BY_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_DESTROY_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_FLUSH_ROSTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_API_DESTROY;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_HIDE_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_ITEM_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_BASE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_PHASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_SOUND_GET_TRIGGER_IDS;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_GET_2 as MC_MANAGER_GET;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE_CALLEE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_RESET_ONE;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_IS_LOADING;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_FINALIZE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LOOKUP_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LAUNCH_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_RESUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SUSPEND;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SET_RUNNING;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_SET_OBJECT_VALUE;
}

/// `sInputPad` button array starts at `+4`, stride 0x1c.
pub(crate) mod pad {

    pub const COUNT: u32 = 0x0;

    pub const BUTTONS: u32 = 0x4;

    pub const BUTTON_SIZE: u32 = 0x1c;
}


// ---------------------------------------------------------------------------
// Input VM.

/// `float InputMap_EvalFloat(InputMap *map, int value, int binding)`
/// (0x320fa8): run the binding's operand list through the input VM and return
/// its result.
pub fn input_map_eval_float(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x1d0);
        frame.save(c, S1, 0x1b0);
        frame.save(c, S0, 0x1c0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x1a0);
        c.g[S0][0] = c.g[A2][0];
        c.g[A0][0] = c.g[SP][0];
        frame.save_ra(c, 0x190);
        c.g[S2][0] = c.g[A1][0];
        callee::INPUT_VM_INIT.call(c, 0x0032_0fd0)?;
        // The operand list of binding `a2`: `ops + 4 * table[a2]`.
        c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x4).lw(c);
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x14).lw(c);
        c.g[A1][0] = c.g[S2][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        callee::INPUT_VM_EXECUTE.call(c, 0x0032_0ff8)?;
        frame.restore(c, S0, 0x1c0);
        frame.restore(c, S1, 0x1b0);
        frame.restore(c, S2, 0x1a0);
        frame.restore_ra(c, 0x190);
        frame.close(c);
        Ok(())
    })
}


/// `bool InputMap_EvalBool(InputMap *map, int value, int binding)`
/// (0x321108): [`input_map_eval_float`] with the result compared against 0.
pub fn input_map_eval_bool(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x1d0);
        frame.save(c, S1, 0x1b0);
        frame.save(c, S0, 0x1c0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x1a0);
        c.g[S0][0] = c.g[A2][0];
        c.g[A0][0] = c.g[SP][0];
        frame.save_ra(c, 0x190);
        c.g[S2][0] = c.g[A1][0];
        callee::INPUT_VM_INIT.call(c, 0x0032_1130)?;
        c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x4).lw(c);
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x14).lw(c);
        c.g[A1][0] = c.g[S2][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        callee::INPUT_VM_EXECUTE.call(c, 0x0032_1158)?;
        c.mtc1(ZERO, 1); // f1 = 0.0
        c.c_eq_s(0, 1);
        let mut result = 1u64;
        if c.fcc() {
            // The VM's value is exactly 0.
            c.cycles += 1;
            result = 0;
        }
        c.g[V0][0] = result;
        c.cycles += 6;
        frame.restore(c, S0, 0x1c0);
        frame.restore(c, S1, 0x1b0);
        frame.restore(c, S2, 0x1a0);
        frame.restore_ra(c, 0x190);
        frame.close(c);
        Ok(())
    })
}


/// `void InputPad_Update(sInputPad *self, int n, float *values)`
/// (0x00321298): resize the button array, then per button store the value,
/// run the debounce/settle logic and the auto-repeat timer.
pub fn input_pad_update(c: &mut Cpu) {
    use pad::*;
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.ldw(c.g[S0][0] as u32);
        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[S1][0] as i64)) as u64;
        let fits = c.g[V0][0] == 0;
        c.g[S2][0] = c.g[A2][0];
        if !fits {
            c.cycles += 9;
            c.g[A2][0] = BUTTON_SIZE as u64;
            c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
            c.mult(0, A0, A0, A2);
            c.g[A1][0] = 0;
            c.mult(0, 0, V0, A2);
            c.g[A0][0] = sx(off(c.g[A0][0] as u32, 4));
            c.g[A2][0] = c.lo[0];
            c.g[A0][0] = addu(c.g[S0][0], c.g[A0][0]);
            callee::EA_MEMSET.call(c, 0x0032_12e8)?;
        }
        c.cycles += 3;
        c.stw(c.g[S0][0] as u32, c.g[S1][0]);
        c.g[T4][0] = 0;
        let empty = (c.g[S1][0] as i64) <= 0;
        if !empty {
            c.cycles += 11;
            c.g[T5][0] = sx(off(c.g[S0][0] as u32, 0x18));
            c.mtc1(0, 1);
            c.g[A2][0] = c.g[S2][0];
            c.g[T3][0] = 1;
            c.g[T2][0] = sx(off(c.g[S0][0] as u32, 0x14));
            c.g[A3][0] = c.g[T5][0];
            c.g[T0][0] = 0;
            c.g[T1][0] = sx(off(c.g[S0][0] as u32, 0x10));
            c.g[A0][0] = sx(off(c.g[S0][0] as u32, 0x1c));
            c.g[T7][0] = 0x18;
            c.g[T6][0] = 0xc;
            loop {
                // value = *values; settle/timer below.
                c.cycles += 5;
                c.lwc1(0, c.g[A2][0] as u32);
                c.g[V1][0] = c.ldw(c.g[A0][0] as u32);
                c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                let settling = c.g[V0][0] != 0;
                c.swc1(0, (c.g[A0][0] as u32).wrapping_sub(0x18));
                if settling {
                    c.cycles += 6;
                    c.g[V0][0] = sx(off(c.g[V1][0] as u32, 1));
                    c.stw((c.g[A0][0] as u32).wrapping_sub(0x14), 0);
                    c.stw(c.g[A0][0] as u32, c.g[V0][0]);
                    c.g[A1][0] = sx(off(c.g[S0][0] as u32, 0x10));
                    c.stw((c.g[A0][0] as u32).wrapping_sub(0x10), 0);
                } else {
                    // on = 0 < value
                    c.cycles += 4;
                    c.c_lt_s(1, 0);
                    let on = c.fcc();
                    c.g[V1][0] = on as u64;
                    if !on {
                        c.cycles += 1;
                        c.g[V1][0] = 0;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.ldw((c.g[A0][0] as u32).wrapping_sub(0xc));
                    let changed = c.g[V1][0] != c.g[V0][0];
                    c.g[A1][0] = sx(off(c.g[S0][0] as u32, 0x10));
                    if !changed {
                        c.cycles += 3;
                        c.stw((c.g[A0][0] as u32).wrapping_sub(0x14), 0);
                        c.stw((c.g[A0][0] as u32).wrapping_sub(0x10), 0);
                    } else {
                        c.cycles += 2;
                        let released = c.g[V1][0] == 0;
                        c.stw(c.g[T1][0] as u32, c.g[V1][0]);
                        if released {
                            c.cycles += 2;
                            c.stw((c.g[A0][0] as u32).wrapping_sub(0x14), 0);
                            c.stw((c.g[A0][0] as u32).wrapping_sub(0x10), c.g[T3][0]);
                        } else {
                            c.cycles += 3;
                            c.stw((c.g[A0][0] as u32).wrapping_sub(0x14), c.g[T3][0]);
                            c.stw((c.g[A0][0] as u32).wrapping_sub(0x10), 0);
                        }
                        c.cycles += 1;
                        c.stw(c.g[A0][0] as u32, 0);
                    }
                }
                // Auto-repeat.
                c.cycles += 4;
                c.g[V0][0] = addu(c.g[A1][0], c.g[T0][0]);
                c.g[V1][0] = c.ldw(c.g[V0][0] as u32);
                let up = c.g[V1][0] == 0;
                c.g[V0][0] = sx(off(c.g[T2][0] as u32, c.g[T0][0] as u32));
                if up {
                    c.cycles += 3;
                    c.g[V1][0] = sx(off(c.g[T5][0] as u32, c.g[T0][0] as u32));
                    c.stw(c.g[V0][0] as u32, 0);
                    c.stw(c.g[V1][0] as u32, 0);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.ldw((c.g[A0][0] as u32).wrapping_sub(4));
                    let has_timer = c.g[V0][0] != 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                    if !has_timer {
                        c.cycles += 3;
                        c.stw((c.g[A0][0] as u32).wrapping_sub(8), c.g[T3][0]);
                        c.stw((c.g[A0][0] as u32).wrapping_sub(4), c.g[T7][0]);
                    } else {
                        c.cycles += 2;
                        let still = (c.g[V0][0] as i64) > 0;
                        c.stw(c.g[A3][0] as u32, c.g[V0][0]);
                        if still {
                            c.cycles += 3;
                            c.g[V0][0] = sx(off(c.g[T2][0] as u32, c.g[T0][0] as u32));
                            c.stw(c.g[V0][0] as u32, 0);
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = sx(off(c.g[T2][0] as u32, c.g[T0][0] as u32));
                            c.stw(c.g[V0][0] as u32, c.g[T3][0]);
                            c.stw(c.g[A3][0] as u32, c.g[T6][0]);
                        }
                    }
                }
                c.cycles += 8;
                c.g[T4][0] = sx(off(c.g[T4][0] as u32, 1));
                c.g[A3][0] = sx(off(c.g[A3][0] as u32, BUTTON_SIZE));
                c.g[T0][0] = sx(off(c.g[T0][0] as u32, BUTTON_SIZE));
                c.g[T1][0] = sx(off(c.g[T1][0] as u32, BUTTON_SIZE));
                c.g[A0][0] = sx(off(c.g[A0][0] as u32, BUTTON_SIZE));
                c.g[V0][0] = ((c.g[T4][0] as i64) < (c.g[S1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A2][0] = sx(off(c.g[A2][0] as u32, 4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0032_1320);
                }
            }
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


/// `void* InputParser_AllocNode(InputParser* self)` (0x3215f8): pop the free
/// list at `+0xe0`, clear the node (0x10 bytes) and return it.
pub fn input_parser_alloc_node(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0x10;
        c.g[S0][0] = Addr(r32(c, V0)).off(0xe0).lw(c);
        c.g[V1][0] = Addr(r32(c, S0)).off(4).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.sw(p(r32(c, V0), 0xe0), c.g[V1][0]);
        callee::MEMSET.call(c, 0x0032_1624)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}


/// `void InputMap_Init(InputMap* self, int count)` (0x320ca0): size the map
/// array, allocate `count * 4` bytes for it and fill the buffer with -1.
pub fn input_map_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        open(c, 0x20);
        c.g[V0][0] = c.g[A1][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = 0x490000;
        c.g[S0][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.g[A0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2300));
        c.g[RA][0] = sx(0x0032_0cd0);
        c.sw((c.g[S0][0] as u32), c.g[V0][0]);
        call_ptr(c, 0x0031_7e30, 0x0032_0cd0)?;
        c.g[A2][0] = c.lw((c.g[S0][0] as u32));
        c.g[A1][0] = 0xffff_ffff_ffff_0000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
        c.g[RA][0] = sx(0x0032_0cec);
        c.g[A1][0] = c.g[A1][0] | 0xffff;
        call_ptr(c, 0x003e_6448, 0x0032_0cec)?;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x20);
        Ok(())
    })
}


/// `void InputMap_CreateParser(InputMap* self, name, arg)` (0x320db0):
/// allocate a 0x109ec-byte parser from the game heap and construct it.
pub fn input_map_create_parser(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        open(c, 0x40);
        c.g[A3][0] = 0u64;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[A1][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S2][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[A1][0] = 0x490000;
        c.g[A0][0] = 0x10000;
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x22f0));
        c.g[A0][0] = c.g[A0][0] | 0x9ec;
        c.g[RA][0] = sx(0x0032_0dec);
        c.g[A2][0] = 0x100;
        call_ptr(c, 0x0031_7d70, 0x0032_0dec)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x0032_0dfc);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0032_1428, 0x0032_0dfc)?;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        close(c, 0x40);
        Ok(())
    })
}


/// `void InputParser_Error(InputParser* self, const char* fmt, ...)`
/// (0x321500): format the parser's name and the message into a local buffer
/// and flag the parser as failed.
pub fn input_parser_error(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x4b0);
        let self_ = abi::arg(c, 0);
        let fmt = abi::arg64(c, 1);
        let a2 = abi::arg64(c, 2);
        let a3 = abi::arg64(c, 3);
        let t0 = abi::arg64(c, 4);
        let t1 = abi::arg64(c, 5);
        let t2 = abi::arg64(c, 6);
        let t3 = abi::arg64(c, 7);
        frame.save(c, S0, 0x430);
        frame.save(c, S1, 0x420);
        c.g[S0][0] = sx(self_);
        frame.save(c, S2, 0x410);
        c.g[S1][0] = sx(frame.at(c, 0x480));
        frame.save_ra(c, 0x400);
        c.g[S2][0] = fmt;
        c.sd(frame.at(c, 0x488), a3);
        c.g[A1][0] = 0x0049_0000;
        c.g[A3][0] = c.lw(at(c, S0, 0x8c));
        c.g[A0][0] = sx(frame.at(c, 0));
        c.sd(frame.at(c, 0x480), a2);
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x22c0));
        c.sd(frame.at(c, 0x490), t0);
        c.g[A2][0] = sx(p(r32(c, S0), 0xc));
        c.sd(frame.at(c, 0x498), t1);
        c.sd(frame.at(c, 0x4a0), t2);
        c.sd(frame.at(c, 0x4a8), t3);
        frame.save_fpr(c, 12, 0x470);
        frame.save_fpr(c, 14, 0x474);
        frame.save_fpr(c, 16, 0x478);
        frame.save_fpr(c, 18, 0x47c);
        call_ptr(c, callee::SPRINTF, 0x0032_1560)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = sx(r32(c, SP).wrapping_add(r32(c, V0)));
        call_ptr(c, callee::VSPRINTF, 0x0032_1570)?;
        c.g[V1][0] = 1;
        frame.restore(c, S1, 0x420);
        c.sw(at(c, S0, 0x94), c.g[V1][0]);
        frame.restore(c, S0, 0x430);
        frame.restore(c, S2, 0x410);
        frame.restore_ra(c, 0x400);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Input map file

/// `InputMap_FinalizeOpcodes` (0x320f10): replace the map's binding array:
/// delete the old list, allocate a copy of the new one, copy it in and free
/// the temporary. `this+8` is the old array, `+0x10` the new array (word
/// count), `+0x14` the source list to copy from.
pub fn input_map_finalize_opcodes(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        frame.save_ra(c, 0);

        let old = Addr(p(this, 8)).lw(c);
        if old == 0 {
            c.g[A0][0] = Addr(p(this, 0x10)).lw(c);
        } else {
            c.cycles += 3;
            c.g[A0][0] = old;
            c.g[RA][0] = sx(0x0032_0f38);
            callee::BUILTIN_DELETE.call(c, 0x0032_0f38)?;
            c.g[A0][0] = Addr(p(this, 0x10)).lw(c);
        }

        // The new array: `count * 4` bytes from the named pool.
        c.cycles += 16;
        c.g[A1][0] = 0x0049_0000;
        Addr(p(this, 8)).sw(c, 0);
        c.g[A2][0] = 0;
        c.g[A1][0] = sx(0x0049_0000u32.wrapping_sub(0x22d0));
        c.g[A0][0] = shl32(c.g[A0][0], 2);
        c.g[A3][0] = 0;
        c.g[RA][0] = sx(0x0032_0f58);
        callee::OPERATOR_NEW_NAMED.call(c, 0x0032_0f58)?;
        c.g[A2][0] = Addr(p(this, 0x10)).lw(c);
        let array = c.g[V0][0];
        c.g[S1][0] = array;
        c.g[A1][0] = Addr(p(this, 0x14)).lw(c);
        c.g[A0][0] = array;
        c.g[RA][0] = sx(0x0032_0f70);
        c.g[A2][0] = shl32(c.g[A2][0], 2);
        call_ptr(c, callee::MEMCPY, 0x0032_0f70)?;
        c.g[A0][0] = Addr(p(this, 0x14)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[RA][0] = sx(0x0032_0f84);
            callee::EA_MEM_FREE.call(c, 0x0032_0f84)?;
        }
        c.g[V0][0] = Addr(p(this, 0x10)).lw(c);
        c.cycles += 7;
        Addr(p(this, 0x14)).sw(c, array);
        Addr(p(this, 0xc)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Input parser.

/// `int InputParser_LookupButton(const char* name)` (0x3216e8): parse
/// `"buttonNN[aA]?"` and return the button index, or -1 when the name does
/// not have the `button` prefix, has no digits, ends in a letter, or is `_`.
pub fn input_parser_lookup_button(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 6;
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x4a0000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(r(c, A1).wrapping_add(0x3ef0));
        call_ptr(c, 0x0041_ab78, 0x0032_170c)?;

        let mismatch = r(c, V0) != 0;
        c.g[V0][0] = u64::MAX;
        if mismatch {
            return input_parser_tail(c, frame);
        }
        c.cycles += 6;
        c.g[V1][0] = c.lb(Addr(r(c, S0)).off(0x6).0);
        c.g[V1][0] = sx(r(c, V1).wrapping_sub(0x30));
        c.g[V0][0] = c.g[V1][0] & 0xff;
        c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
        let no_digit = c.g[V0][0] == 0;
        c.g[V0][0] = u64::MAX;
        if no_digit {
            return input_parser_tail(c, frame);
        }

        'number: {
            c.cycles += 6;
            c.g[V0][0] = c.lb(Addr(r(c, S0)).off(0x7).0);
            c.g[A2][0] = c.g[V1][0];
            c.g[V0][0] = sx(r(c, V0).wrapping_sub(0x30));
            c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
            let no_more = c.g[V0][0] == 0;
            c.g[A1][0] = sx(p(r(c, S0), 0x7));
            if no_more {
                break 'number;
            }
            c.cycles += 1;
            c.g[A3][0] = 0xa;
            loop {
                c.cycles += 9;
                c.mult(0, V1, A2, A3);
                c.g[A0][0] = c.lb(r(c, A1));
                c.g[A1][0] = sx(p(r(c, A1), 0x1));
                c.g[V0][0] = c.lb(r(c, A1));
                c.g[V1][0] = sx(r(c, V1).wrapping_sub(0x30));
                c.g[V0][0] = sx(r(c, V0).wrapping_sub(0x30));
                c.g[V0][0] = (c.g[V0][0] < 0xa) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A2][0] = sx(p(r(c, V1), r(c, A0)));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0032_1748);
                }
            }
        }

        'alpha: {
            'inner: {
                c.cycles += 6;
                c.g[V1][0] = c.lb(r(c, A1));
                c.g[A3][0] = 0;
                c.g[V0][0] = sx(r(c, V1).wrapping_sub(0x61));
                c.g[V0][0] = (c.g[V0][0] < 0x1a) as u64;
                let lower = c.g[V0][0] != 0;
                c.g[A0][0] = c.lbu(r(c, A1));
                if lower {
                    break 'inner;
                }
                c.cycles += 4;
                c.g[V0][0] = sx(r(c, V1).wrapping_sub(0x41));
                c.g[V0][0] = (c.g[V0][0] < 0x1a) as u64;
                let not_upper = c.g[V0][0] == 0;
                if not_upper {
                    break 'alpha;
                }
            }
            c.cycles += 1;
            c.g[A3][0] = 1;
        }

        c.cycles += 2;
        let letter = c.g[A3][0] != 0;
        c.g[V0][0] = u64::MAX;
        if letter {
            return input_parser_tail(c, frame);
        }

        'end: {
            c.cycles += 5;
            c.g[V0][0] = sx(r(c, A0) << 24);
            c.g[V1][0] = 0x5f;
            c.g[V0][0] = sx((r(c, V0) as i32 >> 24) as u32);
            let underscore = c.g[V0][0] == c.g[V1][0];
            c.g[V1][0] = ((c.g[A2][0] as i64) < 924) as u64;
            if underscore {
                break 'end;
            }
            c.cycles += 2;
            let small = c.g[V1][0] != 0;
            c.g[V0][0] = c.g[A2][0];
            if small {
                return input_parser_tail(c, frame);
            }
        }
        c.cycles += 1;
        c.g[V0][0] = u64::MAX;
        input_parser_tail(c, frame)
    })
}


/// The `lq s0; ld ra; addiu sp; jr ra` tail.
pub(crate) fn input_parser_tail(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 4;
    frame.restore(c, 16, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::check;
    use crate::difftest::fill;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::SP;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::vtable;
    use crate::difftest::check_lohi;
    use crate::difftest::put_words;
    use crate::difftest::RA as RET;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;
    use crate::difftest::fakes;
    use crate::difftest::seed_count;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::SEEDS_2 as SEEDS;
    pub(crate) use crate::game::camera::camera::tests::INPUT_VM_TABLE;
    pub(crate) use crate::game::camera::camera::tests::INPUT_VM_STOP;
    pub(crate) use crate::game::camera::camera::tests::STOP_OPCODE;
    pub(crate) use crate::game::camera::camera::tests::input_map;
    pub(crate) use crate::game::fe::fereal::tests::base;
    pub(crate) use crate::game::dirtysock::tags2::tests::w8;
    pub(crate) use crate::game::fe::feasyncfile::tests::w32;
    pub(crate) use crate::game::fe::feasyncfile::tests::gpat;
    pub(crate) use crate::game::dirtysock::tags2::tests::cstr;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_8 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_4 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_5 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_4 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_3 as W;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_8 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::eafile_pools;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_7 as run;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_8 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_9 as pool_allocator_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_3 as alloc_base;
    pub(crate) use crate::game::fe::festaterewards::tests::HOOKS;
    pub(crate) use crate::game::fe::festaterewards::tests::KEEP_F0;
    pub(crate) use crate::game::fe::festaterewards::tests::hook;
    pub(crate) use crate::game::fe::festaterewards::tests::strict;
    pub(crate) use crate::game::fe::festaterewards::tests::base_9 as base_3;


    #[test]
    fn input_map_eval_float_matches() {
        check(ee::s10::f_00320fa8__generated, input_map_eval_float, SEEDS, &|c, seed| {
            let mut r = Rng(0x0032_0fa8 ^ seed as u64);
            scramble(c, &mut r);
            let map = DATA + 0x1000;
            c.g[A0][0] = map as u64;
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = (seed % 4) as u64; // binding
            input_map(c, map);
        });
    }


    #[test]
    fn input_map_eval_bool_matches() {
        check(ee::s10::f_00321108__generated, input_map_eval_bool, SEEDS, &|c, seed| {
            let mut r = Rng(0x0032_1108 ^ seed as u64);
            scramble(c, &mut r);
            let map = DATA + 0x1000;
            c.g[A0][0] = map as u64;
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = (seed % 4) as u64; // binding
            input_map(c, map);
            // The stopped VM leaves `*(parser + 8)` in f0 (parser = EvalBool's
            // frame): cover both compare outcomes.
            let value = if seed % 2 == 0 { 0.0f32 } else { 1.0f32 };
            c.bus.write32(SP - 0x1d0 + 8, value.to_bits());
        });
    }


    pub(crate) fn put_f32(c: &mut Cpu, a: u32, bits: u32) {
        c.bus.write32(a, bits);
    }


    #[test]
    fn input_pad_update_matches() {
        check_lohi(
            ssx3_ee::s10::f_00321298__generated,
            input_pad_update,
            300,
            &|c, seed| {
                let mut r = Rng(0x3212_98 ^ seed as u64);
                crate::difftest::scramble(c, &mut r);
                let selfp = DATA;
                let count = seed % 3;
                let n = (seed / 3) % 5;
                let vals = DATA + 0x400;
                c.g[A0][0] = selfp as u64;
                c.g[A1][0] = n as u64;
                c.g[A2][0] = vals as u64;
                c.bus.write32(selfp, count);
                for i in 0..4 {
                    let b = selfp + 4 + 0x1c * i;
                    put_f32(c, b, r.edge_f32());
                    c.bus.write32(b + 4, if i % 2 == 0 { 1 } else { 0 });
                    c.bus.write32(b + 8, 0);
                    c.bus.write32(b + 0xc, r.next_u32() % 2);
                    c.bus.write32(b + 0x10, 0);
                    c.bus.write32(b + 0x14, r.next_u32() % 0x20);
                    c.bus.write32(b + 0x18, r.next_u32() % 4);
                }
                for i in 0..4 {
                    put_f32(c, vals + 4 * i, r.edge_f32());
                }
            },
        );
    }


    #[test]
    fn input_parser_alloc_node_matches() {
        check(
            ee::s10::f_003215f8__generated,
            input_parser_alloc_node,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x3215f8, 0);
                let self_at = DATA + 0x200;
                let node = DATA + 0x400;
                c.g[A0][0] = self_at as u64;
                if seed % 3 == 0 {
                    c.bus.write32(self_at + 0xe0, 0);
                } else {
                    c.bus.write32(self_at + 0xe0, node);
                    c.bus
                        .write32(node + 4, if seed % 2 == 0 { 0 } else { DATA + 0x500 });
                }
            },
        );
    }


    /// The engine's size-class pool allocator (see `oc57`): short free lists
    /// behind `gp + 0x2a60..0x2a84`, enabled by the "pools ready" flag at
    /// `gp + 0xdd8`.
    pub(crate) fn pool_allocator(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x32000),
            (0x2a7c, 0x25800),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        let nodes = DATA + 0x28_000;
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base = nodes + 0x100 * i as u32;
            for n in 0..4u32 {
                let node = base + 0x40 * n;
                c.bus.write32(node, if n == 3 { 0 } else { node + 0x40 });
            }
            c.bus.write32(pool + head, base);
            c.bus.write32(GAME_GP + slot, pool);
        }
    }


    #[test]
    fn input_map_init_matches() {
        check(ee::s10::f_00320ca0__generated, input_map_init, 40, &|c, seed| {
            base(c, seed, 0x32_0ca0);
            pool_allocator(c);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (1 + seed % 8) as u64;
        });
    }


    #[test]
    fn input_parser_error_matches() {
        check(
            ee::s10::f_00321500__generated,
            input_parser_error,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3215, 0);
                env(c);
                let this = DATA + 0x400;
                let fmt = DATA + 0x800;
                cstr(c, fmt, "%d %d %d %d %d %d");
                w32(c, this + 0x8c, r.next_u32());
                w32(c, this + 0xc, 0);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(fmt);
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = r.next_u32() as u64;
                c.g[T1][0] = (seed % 7) as u64;
                c.g[T2][0] = r.next_u32() as u64;
                c.g[T3][0] = r.next_u32() as u64;
                for i in 0..7 {
                    c.f[12 + i] = r.edge_f32();
                }
            },
        );
    }


    /// A real heap block of `size` bytes.
    pub(crate) fn alloc(c: &mut Cpu, size: u32) -> u32 {
        run(
            c,
            0x0031_7d70,
            ee::s10::f_00317d70,
            &[size as u64, 0x0048_0000, 0, 0],
        ) as u32
    }


    // ------------------------------------------------------------------ input map

    #[test]
    fn input_map_finalize_opcodes_matches() {
        check(
            ee::s10::f_00320f10__generated,
            input_map_finalize_opcodes,
            SEEDS,
            &|c, seed| {
                let mut r = alloc_base(c, seed, 0x320f, 4);
                let this = DATA;
                let count = seed % 5;
                let src = alloc(c, 0x40);
                for i in 0..count {
                    w(c, src + 4 * i, r.next_u32());
                }
                w(c, this + 0x10, count);
                w(c, this + 0x14, src);
                let old = if seed & 1 == 0 { 0 } else { alloc(c, 0x40) };
                w(c, this + 8, old);
                c.g[A0][0] = this as u64;
            },
        );
    }


    // --------------------------------------------------------- input parser

    #[test]
    fn input_parser_lookup_button_matches() {
        strict(
            ee::s10::f_003216e8__generated,
            input_parser_lookup_button,
            60,
            &|c, seed| {
                let _ = base_3(c, seed, 0x3216, 6);
                // The "button" prefix the parser compares against.
                for (i, b) in b"button\0".iter().enumerate() {
                    c.bus.write8(0x004a_3ef0 + i as u32, *b);
                }
                let name: &[u8] = match seed % 8 {
                    0 => b"button0\0",
                    1 => b"button12\0",
                    2 => b"button12_\0",
                    3 => b"buttonA\0",
                    4 => b"button\0",
                    5 => b"buttons\0",
                    6 => b"button9999\0",
                    _ => b"butto\0",
                };
                for (i, b) in name.iter().enumerate() {
                    c.bus.write8(DATA + 0x900 + i as u32, *b);
                }
                c.g[A1][0] = (DATA + 0x900) as u64;
            },
        );
    }
}
