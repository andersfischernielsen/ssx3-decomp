//! `src/visualfx/avalanche.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::ps2f::FpuF32;
use crate::call::Flow;
use crate::abi::r32;
use ssx3_ee as ee;
use crate::call;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::X;
use crate::events::back_edge_unwinds;
use crate::call::sp;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
pub(crate) use crate::game::dirtysock::tags2::gp_9 as gp;
pub(crate) use crate::game::dirtysock::tags2::lw;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::gp as gp_2;
pub(crate) use crate::game::fe::feasyncfile::p;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw_2;
pub(crate) use crate::game::animation::rideranimbase::sw;
pub(crate) use crate::game::dirtysock::tags::lbu;
pub(crate) use crate::game::render::graphicsman::RegMem;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x002d_6378, vfx_avalanche_dtor, "VfxAvalanche_dtor"),
    (0x002d_6410, vfx_avalanche_init, "VfxAvalanche_Init"),
    (0x002d_64d8, avalanche_clamp_vector, "Avalanche_ClampVector"),
    (0x002d_7c00, avalanche_compute_instance_fade, "Avalanche_ComputeInstanceFade"),
    (0x002d_7dd8, avalanche_release_instance, "Avalanche_ReleaseInstance"),
    (0x002d_7e60, avalanche_build_active_node_array, "Avalanche_BuildActiveNodeArray"),
    (0x002d_9660, avalanche_bake_group, "Avalanche_BakeGroup"),
    (0x002d_9738, avalanche_add_node_event, "Avalanche_AddNodeEvent"),
    (0x002d_9a80, avalanche_free_histories, "Avalanche_FreeHistories"),
    (0x002d_9c00, avalanche_get_object_matrix, "Avalanche_GetObjectMatrix"),
    (0x002d_9fb8, avalanche_set_history_data, "Avalanche_SetHistoryData"),
    (0x002f_7e70, avalanche_destroy_all_thunk, "Avalanche_DestroyAllThunk"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::wscript::wscriptman::callee::SET_FADE_GAIN;
    pub(crate) use crate::game::wscript::wscriptman::callee::SND_BANK_UNLOAD;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_AUDIO_INSTANCE_STOP_SOUND;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_INSTANCE_POLL_FREE;
    pub(crate) use crate::game::wscript::wscriptman::callee::BANK_MONITOR_INSTANCE_SET_PITCH;
    pub(crate) use crate::game::wscript::wscriptman::callee::CPS2_MEMCARD_SET_STATE;
    pub(crate) use crate::game::wscript::wscriptman::callee::AVALANCHE_BAKE_NODE_HISTORY;
    pub(crate) use crate::game::wscript::wscriptman::callee::WSCRIPT_MAN_CALL_FUNC;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_MAN3D_GROUP_FREE_MESHES;
    pub(crate) use crate::game::wscript::wscriptman::callee::CROWD_MAN3D_GROUP_CREATE_MESHES_2 as CROWD_MAN3D_GROUP_CREATE_MESHES;
    pub(crate) use crate::game::wscript::wscriptman::callee::FLEX_RAIL_UPDATE_SPACE_NODE;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_FLAG_MANAGER_GROUP_LOAD;
    pub(crate) use crate::game::wscript::wscriptman::callee::FLOATING_V49;
    pub(crate) use crate::game::wscript::wscriptman::callee::MOD_OBJ_DTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::MODIFIER_LIST_UNLINK;
    pub(crate) use crate::game::wscript::wscriptman::callee::RAIL_MODIFIER_LIST_UNLINK;
    pub(crate) use crate::game::wscript::wscriptman::callee::DISABLE_INTC;
    pub(crate) use crate::game::wscript::wscriptman::callee::REMOVE_INTC_HANDLER;
    pub(crate) use crate::game::wscript::wscriptman::callee::TEX_MAN_LOAD_TEXTURE;
    pub(crate) use crate::game::wscript::wscriptman::callee::LIGHT_MAN_EMPTY_HOOK_A;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_GET_STATE_DATA;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::wscript::wscriptman::callee::CPS2_CHUNK_RESOLVE_CTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::AV_PLAYER_INIT;
    pub(crate) use crate::game::wscript::wscriptman::callee::FOCUS_MAN_ADD_FOCUS_BOX;
    pub(crate) use crate::game::wscript::wscriptman::callee::CAM_CONTROLLER_CLEAR_ALGORITHMS;
    pub(crate) use crate::game::wscript::wscriptman::callee::ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::wscript::wscriptman::callee::APP_MODULE_UPDATE_FADE;
    pub(crate) use crate::game::wscript::wscriptman::callee::DEBUG_MENU_MODULE_UPDATE_CURSOR_TRAIL;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_SETUP_MAN_SEND_QUIT;
    pub(crate) use crate::game::wscript::wscriptman::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_EMPTY_CHILD_HANDLER;
    pub(crate) use crate::game::wscript::wscriptman::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::wscript::wscriptman::callee::UI_OBJECT_SET_USER_DATA;
    pub(crate) use crate::game::wscript::wscriptman::callee::EA_FILE_MOUNT_BIG;
    pub(crate) use crate::game::wscript::wscriptman::callee::NET_FE_STATE_CTOR;
    pub(crate) use crate::game::wscript::wscriptman::callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::dirtysock::tags2::callee::ON_QUIT_POPUP_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::EMPTY_CHILD_HANDLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_ON_CHILD_EVENT_3 as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE_2 as NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_CTOR_3 as NET_FE_STATE_CTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_DTOR_2 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY_2 as CHAT_ROOM_UPDATE_PLAYER_QUALITY;
    pub(crate) use crate::game::dirtysock::tags2::callee::STATS_SELECT_CATEGORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_UPDATE_CURSOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_CARD_SET_USE_SAVE_TITLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::TROPHY_ROOM_RELEASE_IMAGES;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_GET_INBOX;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_PUSH_FREE_NODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_END_OPERATION_2 as MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_REQUEST_CARD_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_SET_PRESENCE_ADDR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_DESTROY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_OBJ_CAM_SET_OWNER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_OBJ_TABLE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD_2;
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
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_REWARD_INT_IS_UNLOCKED;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_3 as UI_STATE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEMSET;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_LIST_DIR_ALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_BX_STRING_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_LIST_GET_AT_3 as LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SLOGIN_MC_GET_PORT_SLOT_2 as SLOGIN_MC_GET_PORT_SLOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCE_MC_FORMAT;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPRINTF_2 as SPRINTF;
    pub(crate) use crate::game::dirtysock::tags2::callee::RAND_NEXT_MAIN;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SETUP_MAN_SEND_QUIT as C_SETUP_MAN_SEND_QUIT_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF as UI_OBJECT_SHOW_CLAMP_ALPHA_HALF_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_LUNO_TABLE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUILTIN_DELETE_6 as BUILTIN_DELETE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_FREE_4 as EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_3 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_2 as NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHAT_ROOM_SELECT_PLAYER_BY_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_AGREEMENT_TERMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_RELEASE_SNAPSHOT_LISTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_DECLINE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_PICK_RANDOM_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_COUNT_SAVE_SNAPSHOTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_CHANNEL_GET_LENGTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_PUSH_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_6 as UI_LIST_FIRST;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_3 as UI_SCREEN_PLAY_FRAME;
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


/// `void Avalanche_ComputeInstanceFade(void *self, float dt)` (0x2d7c00):
/// advance the 30Hz fade of the particle instance at `self+0x2e0`, writing the
/// fade ramp at `self+0xb0` and the fade fraction at `self+0xb4`.
pub fn avalanche_compute_instance_fade(c: &mut Cpu) {
    c.cycles += 9;
    c.g[AT][0] = 0x41f0_0000; // 30.0
    c.mtc1(AT, 1);
    let this = abi::arg(c, 0);
    let inst = Addr(Addr(this).off(0x2e0).lw(c) as u32);
    c.g[V0][0] = inst.0 as u64;
    c.mul_s(12, 12, 1);
    c.lwc1(0, inst.off(0x104).0);
    c.c_lt_s(12, 0);
    if !c.fcc() {
        // dt >= lifetime: either ramp down from the range, or snap to 1.
        c.lwc1(1, inst.off(0x100).0);
        c.cycles += 6;
        c.lwc1(2, inst.off(0x108).0);
        c.sub_s(0, 1, 2);
        c.c_lt_s(12, 0);
        if !c.fcc() {
            c.g[V1][0] = inst.off(0xf0).lhu(c);
            c.cycles += 3;
            c.g[V0][0] = 0x2;
            if c.g[V1][0] != c.g[V0][0] {
                c.cycles += 7;
                c.sub_s(0, 1, 12);
                c.mtc1(ZERO, 1);
                c.div_s(0, 0, 2);
                c.c_lt_s(1, 0);
                c.swc1(0, this.wrapping_add(0xb4));
                let nonpos = !c.fcc();
                c.swc1(0, this.wrapping_add(0xb0));
                if !nonpos {
                    c.cycles += 2;
                    c.sqrt_s(0, 0);
                    c.swc1(0, this.wrapping_add(0xb0));
                }
            }
            c.cycles += 2;
            abi::ret(c);
            return;
        } else {
            c.cycles += 5;
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(AT, 0);
            c.swc1(0, this.wrapping_add(0xb0));
            c.swc1(0, this.wrapping_add(0xb4));
            abi::ret(c);
            return;
        }
    } else {
        c.cycles += 7;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(AT, 1);
        c.swc1(1, this.wrapping_add(0xb0));
        c.lwc1(0, inst.off(0x104).0);
        c.div_s(0, 12, 0);
        c.swc1(0, this.wrapping_add(0xb4));
        abi::ret(c);
        return;
    }
}


// ---------------------------------------------------------------------------
// Avalanche node events.

/// `void Avalanche_AddNodeEvent(Avalanche *ava, int event, int node)`
/// (0x2d9738): if the node's event count at +0xe is below 0x40, look up the
/// node in the list at +4 (following +0xf4) and store `event` in the halfword
/// table at `ava + 0x110 + count*2` and the node's index in the table at
/// `ava + 0x112 + count*2`, then bump the count. Back-edge event check at
/// 0x2d9768.
pub fn avalanche_add_node_event(c: &mut Cpu) {
    const COUNT: u32 = 0xe;
    const LIST: u32 = 0x4;
    const LINK: u32 = 0xf4;
    const EVENTS: u32 = 0x110;
    const NODES: u32 = 0x112;
    const HEAD: u32 = 0x002d_9768;
    c.cycles += 4;
    let ava = abi::arg(c, 0);
    let event = abi::arg64(c, 1);
    let node = abi::arg64(c, 2);
    c.g[V0][0] = Addr(ava).off(COUNT).lhu(c);
    c.g[V0][0] = (c.g[V0][0] < 0x40) as u64;
    let full = c.g[V0][0] == 0;
    c.g[A3][0] = 0;
    if full {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    let mut immediate = false;
    {
        c.cycles += 5;
        let mut link = Addr(ava).off(LIST).lw(c);
        c.g[V1][0] = link;
        c.g[V0][0] = Addr(ava).off(COUNT).lhu(c);
        c.g[T0][0] = sx(ava.wrapping_add(EVENTS));
        let empty = link == 0;
        c.g[T1][0] = sx(ava.wrapping_add(NODES));
        if !empty {
            c.cycles += 2;
            if link == node {
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                immediate = true;
            } else {
                c.cycles += 1;
                link = Addr(link as u32).off(LINK).lw(c);
                c.g[V1][0] = link;
                loop {
                    c.cycles += 2;
                    let end = link == 0;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                    if end {
                        break;
                    }
                    c.cycles += 2;
                    if link != node {
                        link = Addr(link as u32).off(LINK).lw(c);
                        c.g[V1][0] = link;
                        if event_due(c) {
                            deliver_event_here(c, HEAD);
                        }
                        continue;
                    }
                    break;
                }
            }
        }
    }
    if !immediate {
        c.cycles += 1;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    }
    c.cycles += 9;
    c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.sh(c.g[V0][0] as u32, event);
    c.g[V1][0] = Addr(ava).off(COUNT).lhu(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
    c.g[V1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sh(c.g[V1][0] as u32, c.g[A3][0]);
    c.g[V0][0] = Addr(ava).off(COUNT).lhu(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
    c.sh(ava.wrapping_add(COUNT), c.g[V0][0]);
    c.cycles += 2;
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Avalanche active-node array.

/// `void Avalanche_BuildActiveNodeArray(Avalanche *ava)` (0x2d7e60): count
/// the nodes reachable through `ava->nodes(+4)` -> +0x2e4, allocate
/// `count*4` bytes (name 0x487548) and store the pointer at +8, then fill the
/// array with the node pointers. Back-edge event checks at 0x2d7e80 /
/// 0x2d7ec8.
pub fn avalanche_build_active_node_array(c: &mut Cpu) {
    guest(c, |c| {
        const NODES: u32 = 0x4;
        const ARRAY: u32 = 0x8;
        const LINK: u32 = 0x2e4;
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let ava = abi::arg(c, 0);
        c.g[S0][0] = ava as u64;
        let mut link = Addr(ava).off(NODES).lw(c);
        c.g[A1][0] = link;
        c.g[A0][0] = 0;
        if link != 0 {
            c.cycles += 1;
            let mut count = 0u32;
            loop {
                c.cycles += 7;
                link = Addr(link as u32).off(LINK).lw(c);
                c.g[A1][0] = link;
                let more = link != 0;
                count = count.wrapping_add(1);
                c.g[A0][0] = sx(count);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002d_7e80);
                }
            }
        }
        // operator_new_named(count*4, 0x487548, 0, 0)
        c.cycles += 10;
        c.g[A1][0] = 0x0048_0000;
        let size = (c.g[A0][0] as u32) << 2;
        c.g[A0][0] = sx(size);
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_add(0x7548));
        c.g[A2][0] = 0;
        c.g[RA][0] = sx(0x002d_7eb4);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7e30, 0x002d_7eb4)?;
        let array = c.g[V0][0];
        let mut link = Addr(ava).off(NODES).lw(c);
        c.g[A1][0] = link;
        c.g[A0][0] = 0;
        let empty = link == 0;
        Addr(ava).off(ARRAY).sw(c, array);
        if !empty {
            c.cycles += 1;
            let mut i = 0u32;
            loop {
                c.cycles += 7;
                let base = Addr(ava).off(ARRAY).lw(c);
                let p = (i << 2).wrapping_add(base as u32);
                c.g[V0][0] = sx(p);
                c.g[V1][0] = base;
                Addr(p).sw(c, link);
                link = Addr(link as u32).off(LINK).lw(c);
                c.g[A1][0] = link;
                let more = link != 0;
                i = i.wrapping_add(1);
                c.g[A0][0] = sx(i);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002d_7ec8);
                }
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void Avalanche_DestroyAllThunk(void)` (0x2f7e70): `Avalanche_DestroyAll()`.
pub fn avalanche_destroy_all_thunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        Callee { addr: 0x002d_9b40, f: ssx3_ee::s09::f_002d9b40 }.call(c, 0x002f_7e80)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void Avalanche_BakeGroup(AvalancheGroup* self)` (0x2d9660): bake the
/// history of every node linked from `+0x4` (through `+0xf4`).
pub fn avalanche_bake_group(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = Addr(this).off(4).lw(c);
        if c.g[S0][0] != 0 {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            loop {
                c.cycles += 5;
                c.g[A1][0] = c.g[S1][0];
                callee::AVALANCHE_BAKE_NODE_HISTORY.call(c, 0x002d_9690)?;
                c.g[S0][0] = Addr(r32(c, S0)).off(0xf4).lw(c);
                let more = c.g[S0][0] != 0;
                c.g[A0][0] = c.g[S0][0];
                if !more {
                    c.cycles += 1;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002d_9688);
                }
            }
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void Avalanche_BakeAll(void)` (0x2d96e0): free the histories, bake every
/// group, then export and reload the history files.
///
/// Not installed: the unconditional `Avalanche_ExportHistoryFiles` /
/// `Avalanche_LoadHistoryFile` enter the EAFile request queue, which cannot be
/// bounded in a test fixture (no request slots / no disc backing), so the body
/// has no differential test.
pub fn avalanche_bake_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        callee::AVALANCHE_FREE_HISTORIES.call(c, 0x002d_96f4)?;
        c.g[S0][0] = lw(c, gp(c, 0x9c0));
        if c.g[S0][0] != 0 {
            loop {
                c.cycles += 5;
                c.g[RA][0] = sx(0x002d_9708);
                c.g[A0][0] = c.g[S0][0];
                callee::AVALANCHE_BAKE_GROUP.call(c, 0x002d_9708)?;
                c.g[S0][0] = Addr(r32(c, S0)).off(8).lw(c);
                if c.g[S0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x002d_9700);
                }
            }
        }
        c.cycles += 8;
        c.g[RA][0] = sx(0x002d_971c);
        callee::AVALANCHE_EXPORT_HISTORY_FILES.call(c, 0x002d_971c)?;
        c.g[RA][0] = sx(0x002d_9724);
        callee::AVALANCHE_LOAD_HISTORY_FILE.call(c, 0x002d_9724)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void Avalanche_SetHistoryData(void* data)` (0x2d9fb8): replace the
/// avalanche history data, freeing the old block and re-resolving the new
/// data's pointers when it carries the magic tag.
pub fn avalanche_set_history_data(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        sqsp(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call::call_ptr(c, 0x002d_9b40, 0x002d_9fcc)?;
        c.g[A0][0] = Addr(gp_2(c, 0x9c4)).lw(c);
        let no_old = c.g[A0][0] == 0;
        c.g[V0][0] = Addr(gp_2(c, 0x9c8)).lw(c);
        let mut set = false;
        if no_old {
            set = true;
        } else {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                Addr(gp_2(c, 0x9c4)).sw(c, c.g[S0][0]);
            } else {
                c.cycles += 2;
                call::call_ptr(c, 0x0031_7e98, 0x002d_9fe8)?;
                set = true;
            }
        }
        if set {
            c.cycles += 1;
            Addr(gp_2(c, 0x9c4)).sw(c, c.g[S0][0]);
        }
        c.cycles += 6;
        c.g[V0][0] = 0x02be_ef00;
        Addr(gp_2(c, 0x9c8)).sw(c, 0);
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        let magic = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 1;
        if !magic {
            c.cycles += 3;
            Addr(gp_2(c, 0x9c4)).sw(c, 0);
            Addr(gp_2(c, 0x9d4)).sw(c, c.g[V0][0]);
        } else {
            c.cycles += 2;
            Addr(gp_2(c, 0x9d4)).sw(c, 0);
            call::call_ptr(c, 0x002d_a028, 0x002d_a018)?;
        }
        c.cycles += 4;
        lqsp(c, 16, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}


// ---------------------------------------------------------------------------
// Avalanche.

/// `void Avalanche_ReleaseInstance(Avalanche* self, int force)` (0x2d7dd8):
/// walk to the live emitter (or use the first one when forced), release its
/// sub-emitter and reset the avalanche's colours.
pub fn avalanche_release_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(p(r32(c, S0), 0x2e0)).lw(c);
        if c.g[A0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            // Pick the emitter: the first when forced, else the first live one.
            let mut release = true;
            c.cycles += 2;
            if c.g[A1][0] != 0 {
                c.g[A0][0] = Addr(p(r32(c, A0), 0xf8)).lw(c);
            } else {
                c.cycles += 4;
                c.g[V1][0] = Addr(p(r32(c, A0), 0xf0)).lhu(c);
                c.g[V0][0] = 0x2;
                if c.g[V1][0] == c.g[V0][0] {
                    release = false;
                } else {
                    c.cycles += 1;
                    c.g[A0][0] = Addr(p(r32(c, A0), 0xf8)).lw(c);
                }
            }
            if !release {
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 2;
                if c.g[A0][0] == 0 {
                    c.g[V0][0] = Addr(p(r32(c, S0), 0x2e0)).lw(c);
                } else {
                    c.cycles += 3;
                    c.g[A2][0] = Addr(p(r32(c, A0), 0xc)).lw(c);
                    let no_sub = c.g[A2][0] == 0;
                    c.g[A1][0] = 0x3;
                    if !no_sub {
                        c.cycles += 5;
                        c.g[V0][0] = Addr(p(r32(c, A2), 0xc)).lw(c);
                        c.g[A0][0] = Addr(p(r32(c, V0), 8)).lh(c);
                        c.g[V1][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
                        let target = r32(c, V1);
                        c.g[RA][0] = sx(0x002d_7e38);
                        c.g[A0][0] = sx(p(r32(c, A2), r32(c, A0)));
                        call_ptr(c, target, 0x002d_7e38)?;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = Addr(p(r32(c, S0), 0x2e0)).lw(c);
                }
                c.cycles += 5;
                c.g[A0][0] = sx(p(r32(c, S0), 0xd0));
                Addr(p(r32(c, V0), 0x114)).sw(c, 0);
                c.g[RA][0] = sx(0x002d_7e4c);
                Addr(p(r32(c, S0), 0x2e0)).sw(c, 0);
                call_ptr(c, 0x0037_1600, 0x002d_7e4c)?;
                frame.restore(c, S0, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `VfxAvalanche_dtor` (0x2d6378): destroy every avalanche effect in the
/// static bank, install the base vtable and optionally free the object.
pub fn vfx_avalanche_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 0x0049_0000;
        frame.save(c, S1, 0x30);
        c.g[V0][0] = sx(0x0049_0000u32.wrapping_sub(0x79b8));
        frame.save(c, S0, 0x40);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x002d_63a8);
        Addr(this).sw(c, c.g[V0][0]);
        callee::AVALANCHE_DESTROY_ALL.call(c, 0x002d_63a8)?;
        c.g[V0][0] = 0x004f_0000;
        c.g[V1][0] = 0xbc00;
        c.g[S0][0] = sx(0x004f_0000u32.wrapping_sub(0x17c0));
        c.g[S2][0] = sx(r(c, S0).wrapping_add(r(c, V1)));
        c.g[A0][0] = c.g[S0][0];
        loop {
            c.cycles += 6;
            c.g[RA][0] = sx(0x002d_63c8);
            c.g[A1][0] = 1;
            call_ptr(c, callee::AVALANCHE_RESET_ONE, 0x002d_63c8)?;
            c.g[S0][0] = sx(r(c, S0).wrapping_add(0x2f0));
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S0][0];
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002d_63c0);
            }
        }
        c.cycles += 5;
        c.g[V0][0] = 0x0049_0000;
        c.g[V1][0] = c.g[S3][0] & 1;
        c.g[V0][0] = sx(0x0049_0000u32.wrapping_sub(0x7980));
        let free_self = c.g[V1][0] == 0;
        Addr(this).sw(c, c.g[V0][0]);
        if !free_self {
            c.cycles += 2;
            c.g[RA][0] = sx(0x002d_63f4);
            c.g[A0][0] = c.g[S1][0];
            callee::BUILTIN_DELETE_2.call(c, 0x002d_63f4)?;
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


/// `void Avalanche_Stop(self)` (0x2d81b0): free the spawn buffer, release
/// every active instance and tell the weather manager the avalanche ended.
pub fn avalanche_stop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(p(r32(c, S1), 8)).lw(c);
        if c.g[A0][0] == 0 {
            c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        } else {
            c.cycles += 4;
            call_ptr(c, 0x0031_7e98, 0x002d81d8)?;
            Addr(p(r32(c, S1), 8)).sw(c, 0);
            c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        }
        c.cycles += 2;
        let done = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !done {
            c.cycles += 3;
            c.g[S0][0] = Addr(p(r32(c, S1), 4)).lw(c);
            let empty = c.g[S0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !empty {
                c.cycles += 1;
                loop {
                    c.cycles += 5;
                    c.g[A1][0] = 1;
                    call_ptr(c, 0x002d_7dd8, 0x002d8200)?;
                    c.g[S0][0] = Addr(p(r32(c, S0), 0x2e4)).lw(c);
                    let more = c.g[S0][0] != 0;
                    c.g[A0][0] = c.g[S0][0];
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x002d_81f8);
                    }
                }
            }
            c.cycles += 12;
            Addr(r32(c, S1)).sw(c, 0);
            call_ptr(c, 0x0028_b180, 0x002d8214)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0029_def0, 0x002d8220)?;
            c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0x84)).lw(c);
            c.g[A0][0] = Addr(p(r32(c, V1), 0xc)).lw(c);
            c.g[V1][0] = Addr(p(r32(c, A0), 0xa8)).lw(c);
            c.g[V0][0] = Addr(p(r32(c, V1), 0x708)).lw(c);
            let zero = c.g[V0][0] == 0;
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
            if !zero {
                c.cycles += 1;
                Addr(p(r32(c, V1), 0x708)).sw(c, c.g[V0][0]);
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// `Avalanche_GetObjectMatrix` (0x2d9c00): find the avalanche node whose object
// id matches, copy its rotation matrix, scale it and store its position in
// the output matrix.
pub fn avalanche_get_object_matrix(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x4f0000;
    c.g[V1][0] = 0xbc00;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1890));
    c.g[A2][0] = c.g[V0][0];
    c.g[A3][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x70));
    loop {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x2e0));
        let matched = if c.g[V0][0] == 0 {
            false
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xf8));
            c.g[V0][0] == c.g[A0][0]
        };
        if matched {
            c.cycles += 26;
            c.lqc2(1, c.g[V1][0] as u32);
            c.lqc2(2, (c.g[V1][0] as u32).wrapping_add(0x10));
            c.lqc2(3, (c.g[V1][0] as u32).wrapping_add(0x20));
            c.lqc2(4, (c.g[V1][0] as u32).wrapping_add(0x30));
            c.sqc2(1, c.g[A1][0] as u32);
            c.sqc2(2, (c.g[A1][0] as u32).wrapping_add(0x10));
            c.sqc2(3, (c.g[A1][0] as u32).wrapping_add(0x20));
            c.sqc2(4, (c.g[A1][0] as u32).wrapping_add(0x30));
            c.lwc1(0, (c.g[A2][0] as u32).wrapping_add(0xb0));
            c.mfc1(3, 0);
            c.lqc2(4, c.g[A1][0] as u32);
            c.qmtc2(3, 3);
            c.lqc2(5, (c.g[A1][0] as u32).wrapping_add(0x10));
            c.lqc2(6, (c.g[A1][0] as u32).wrapping_add(0x20));
            c.lqc2(7, (c.g[A1][0] as u32).wrapping_add(0x30));
            c.vu0().mul(0b1111, 8, 4, Src::Bc(3, X));
            c.vu0().mul(0b1111, 9, 5, Src::Bc(3, X));
            c.vu0().mul(0b1111, 10, 6, Src::Bc(3, X));
            c.vu0().mul(0b1111, 11, 7, Src::Bc(3, X));
            c.sqc2(8, c.g[A1][0] as u32);
            c.sqc2(9, (c.g[A1][0] as u32).wrapping_add(0x10));
            c.sqc2(10, (c.g[A1][0] as u32).wrapping_add(0x20));
            c.sqc2(11, (c.g[A1][0] as u32).wrapping_add(0x30));
            c.lq(2, (c.g[A2][0] as u32).wrapping_add(0x60));
            c.sq(2, (c.g[A1][0] as u32).wrapping_add(0x30));
            abi::ret(c);
            return;
        }
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2f0));
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[A3][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2f0));
        if !more {
            c.cycles += 2;
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x002d_9c18);
        }
    }
}


// ---------------------------------------------------------------------------
// `Avalanche_FreeHistories` (0x002d9a80).
//
// `void Avalanche::FreeHistories()`: stop every avalanche that has a history
// and release the history links.

pub fn avalanche_free_histories(c: &mut Cpu) {
    guest(c, |c| {

        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.g[V0][0] = 0x540000;
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x76c8));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = c.g[V0][0];
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c0));
        c.g[V0][0] = c.lw((c.g[S0][0] as u32));
        'l_2d9aa8: loop {
        'b_2d9abc: {
        c.cycles += 2;
        if c.g[V0][0] == 0u64 {
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
        break 'b_2d9abc;
        } else {
        c.cycles += 3;
        c.g[RA][0] = sx(0x002d9ab8);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x002d81b0, 0x002d9ab8)?;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1c));
        break 'b_2d9abc;
        }
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
        if c.g[V0][0] != 0u64 {
        c.g[V0][0] = c.lw((c.g[S0][0] as u32));
        if crate::events::event_due(c) { crate::events::deliver_event_here(c, 0x002d9aa8); }
        continue 'l_2d9aa8;
        } else {
        'b_2d9b30: {
        'b_2d9b2c: {
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x9c8));
        let t = c.g[V0][0] == 0u64;
        c.g[S1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x9c0));
        if t {
        break 'b_2d9b2c;
        } else {
        c.cycles += 2;
        let t = c.g[S1][0] == 0u64;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        if t {
        break 'b_2d9b30;
        } else {
        c.cycles += 1;
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        'l_2d9ae0: loop {
        'b_2d9b24: {
        c.cycles += 2;
        if c.g[S0][0] == 0u64 {
        c.g[S1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
        break 'b_2d9b24;
        } else {
        c.cycles += 2;
        c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xf3));
        'l_2d9af0: loop {
        'b_2d9b10: {
        c.cycles += 2;
        if c.g[V0][0] == 0u64 {
        c.sb((c.g[S0][0] as u32).wrapping_add(0xf3), 0u64);
        break 'b_2d9b10;
        } else {
        c.cycles += 3;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xfc));
        if c.g[A0][0] == 0u64 {
        c.sb((c.g[S0][0] as u32).wrapping_add(0xf3), 0u64);
        break 'b_2d9b10;
        } else {
        c.cycles += 3;
        c.g[RA][0] = sx(0x002d9b0c);
        call_ptr(c, 0x00317e98, 0x002d9b0c)?;
        c.sb((c.g[S0][0] as u32).wrapping_add(0xf3), 0u64);
        break 'b_2d9b10;
        }
        }
        }
        c.cycles += 4;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xfc), 0u64);
        c.g[S0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xf4));
        if c.g[S0][0] != 0u64 {
        c.g[V0][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0xf3));
        if crate::events::event_due(c) { crate::events::deliver_event_here(c, 0x002d9af0); }
        continue 'l_2d9af0;
        } else {
        c.cycles += 1;
        c.g[S1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
        break 'b_2d9b24;
        }
        }
        }
        }
        c.cycles += 2;
        if c.g[S1][0] != 0u64 {
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
        if crate::events::event_due(c) { crate::events::deliver_event_here(c, 0x002d9ae0); }
        continue 'l_2d9ae0;
        } else {
        break 'b_2d9b2c;
        }
        }
        }
        }
        }
        c.cycles += 1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        break 'b_2d9b30;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t; return Ok(());
        }
        }

        Ok(())
    })
}


/// `void VfxAvalanche_Init(void)` (0x2d6410): clear the 2048 avalanche
/// particles and the 32 trail entries.
pub fn vfx_avalanche_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
        c.g[V0][0] = 0x4d0000;
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5b70));
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 0);
        c.g[A1][0] = 0u64;
        c.g[A2][0] = 0xff;
        'l_2d6430: loop {
            c.cycles += 15;
            c.g[V0][0] = c.g[A1][0] & 0x1;
            c.g[V1][0] = c.g[A2][0];
            if c.g[V0][0] == 0 {
                c.g[V1][0] = 0u64;
            }
            c.sw((c.g[A0][0] as u32).wrapping_add(0x1c), c.g[A2][0]);
            c.sw((c.g[A0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
            c.sw((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
            c.g[V0][0] = ((c.g[A1][0] as i64) < 2048i64) as u64;
            c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
            c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
            c.sw((c.g[A0][0] as u32).wrapping_add(0xc), 0u64);
            c.sw((c.g[A0][0] as u32), 0u64);
            c.sw((c.g[A0][0] as u32).wrapping_add(0x4), 0u64);
            let t = c.g[V0][0] != 0u64;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x002d6430);
                }
                continue 'l_2d6430;
            } else {
                c.cycles += 7;
                c.g[V0][0] = 0x4f0000;
                c.g[AT][0] = 0x3f800000;
                c.mtc1(1, 0);
                c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2490));
                c.g[A0][0] = 0xff;
                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x600));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
                'l_2d6488: loop {
                    c.cycles += 11;
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x10), c.g[A0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x14), c.g[A0][0]);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x18), 0u64);
                    c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
                    c.sw((c.g[V1][0] as u32), 0u64);
                    c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x30));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[A0][0]);
                        if event_due(c) {
                            deliver_event_here(c, 0x002d6488);
                        }
                        continue 'l_2d6488;
                    } else {
                        c.cycles += 9;
                        c.g[A0][0] = 0x540000;
                        c.g[A1][0] = 0u64;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x7bb0));
                        c.g[A2][0] = 0x400;
                        c.g[RA][0] = sx(0x002d64cc);
                        c.sw((c.g[GP][0] as u32).wrapping_add(0x9d0), 0u64);
                        call_ptr(c, 0x00416210, 0x002d64cc)?;
                        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
                        let t = c.g[RA][0] as u32;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                        c.pc = t;
                        return Ok(());
                    }
                }
            }
        }
        Ok(())
    })
}


/// `void Avalanche_ClampVector(void* self, Vec4* v)` (0x2d64d8): clamp the
/// vector to the avalanche speed limit (`D_004A3A94`); when any component is
/// a PS2 inf/NaN, replace the vector with (10, 10, 10, 0).
pub fn avalanche_clamp_vector(c: &mut Cpu) {
    'b_2d6568: {
        c.cycles += 15;
        c.lwc1(0, c.pa(GP, 0x9a4));
        c.lqc2(3, c.pa(A1, 0));
        c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
        c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
        c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
        c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
        c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
        c.vu0().sqrt(4, 0);
        c.vu0().waitq();
        c.cfc2(2, 22);
        c.mtc1(2, 1);
        c.c_lt_s(0, 1);
        let t = !c.fcc();
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        if t {
            break 'b_2d6568;
        } else {
            c.cycles += 21;
            c.lqc2(3, c.pa(A1, 0));
            c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
            c.vu0().mul(0b1111, 4, 3, Src::Vec(3));
            c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
            c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
            c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
            c.vu0().rsqrt(0, 3, 4, 0);
            c.vu0().waitq();
            c.vu0().mul(0b1111, 5, 3, Src::Q);
            c.sqc2(5, c.pa(SP, 0x20));
            c.lq(2, c.pa(SP, 0x20));
            c.lwc1(0, c.pa(GP, 0x9a4));
            c.sq(2, c.pa(SP, 0x10));
            c.mfc1(3, 0);
            c.lqc2(4, c.pa(SP, 0x10));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, c.pa(SP, 0x20));
            c.lq(2, c.pa(SP, 0x20));
            c.sq(2, c.pa(A1, 0));
            c.sq(2, c.pa(SP, 0));
            break 'b_2d6568;
        }
    }
    'b_2d65a4: {
        'b_2d65a0: {
            c.cycles += 5;
            c.g[V1][0] = c.ldw_2(A1, 0);
            c.g[A2][0] = 0u64;
            c.g[V0][0] = 0x7f800000;
            let t = c.g[V1][0] == c.g[V0][0];
            c.g[A0][0] = 0u64;
            if t {
                break 'b_2d65a0;
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0x7fc00000;
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[A0][0] = 0x1;
                    break 'b_2d65a4;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0xffffffffff800000;
                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[A0][0] = 0x1;
                        break 'b_2d65a4;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0xffffffffffc00000;
                        let t = c.g[V1][0] != c.g[V0][0];

                        if t {
                            break 'b_2d65a4;
                        } else {
                            break 'b_2d65a0;
                        }
                    }
                }
            }
        }
        c.cycles += 1;
        c.g[A0][0] = 0x1;
        break 'b_2d65a4;
    }
    'b_2d666c: {
        c.cycles += 2;
        if c.g[A0][0] != 0u64 {
            c.g[A2][0] = 0x1;
            break 'b_2d666c;
        } else {
            'b_2d65e4: {
                'b_2d65e0: {
                    c.cycles += 4;
                    c.g[V1][0] = c.ldw_2(A1, 0x4);
                    c.g[V0][0] = 0x7f800000;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = 0u64;
                    if t {
                        break 'b_2d65e0;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0x7fc00000;
                        if c.g[V1][0] == c.g[V0][0] {
                            c.g[A0][0] = 0x1;
                            break 'b_2d65e4;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 0xffffffffff800000;
                            if c.g[V1][0] == c.g[V0][0] {
                                c.g[A0][0] = 0x1;
                                break 'b_2d65e4;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = 0xffffffffffc00000;
                                let t = c.g[V1][0] != c.g[V0][0];

                                if t {
                                    break 'b_2d65e4;
                                } else {
                                    break 'b_2d65e0;
                                }
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[A0][0] = 0x1;
                break 'b_2d65e4;
            }
            c.cycles += 2;
            if c.g[A0][0] != 0u64 {
                c.g[A2][0] = 0x1;
                break 'b_2d666c;
            } else {
                'b_2d6624: {
                    'b_2d6620: {
                        c.cycles += 4;
                        c.g[V1][0] = c.ldw_2(A1, 0x8);
                        c.g[V0][0] = 0x7f800000;
                        let t = c.g[V1][0] == c.g[V0][0];
                        c.g[A0][0] = 0u64;
                        if t {
                            break 'b_2d6620;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 0x7fc00000;
                            if c.g[V1][0] == c.g[V0][0] {
                                c.g[A0][0] = 0x1;
                                break 'b_2d6624;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = 0xffffffffff800000;
                                if c.g[V1][0] == c.g[V0][0] {
                                    c.g[A0][0] = 0x1;
                                    break 'b_2d6624;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0xffffffffffc00000;
                                    let t = c.g[V1][0] != c.g[V0][0];

                                    if t {
                                        break 'b_2d6624;
                                    } else {
                                        break 'b_2d6620;
                                    }
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[A0][0] = 0x1;
                    break 'b_2d6624;
                }
                c.cycles += 2;
                if c.g[A0][0] != 0u64 {
                    c.g[A2][0] = 0x1;
                    break 'b_2d666c;
                } else {
                    'b_2d6664: {
                        'b_2d6660: {
                            c.cycles += 4;
                            c.g[V1][0] = c.ldw_2(A1, 0xc);
                            c.g[V0][0] = 0x7f800000;
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.g[A0][0] = 0u64;
                            if t {
                                break 'b_2d6660;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = 0x7fc00000;
                                if c.g[V1][0] == c.g[V0][0] {
                                    c.g[A0][0] = 0x1;
                                    break 'b_2d6664;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0xffffffffff800000;
                                    if c.g[V1][0] == c.g[V0][0] {
                                        c.g[A0][0] = 0x1;
                                        break 'b_2d6664;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = 0xffffffffffc00000;
                                        let t = c.g[V1][0] != c.g[V0][0];

                                        if t {
                                            break 'b_2d6664;
                                        } else {
                                            break 'b_2d6660;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[A0][0] = 0x1;
                        break 'b_2d6664;
                    }
                    c.cycles += 2;
                    if c.g[A0][0] != 0u64 {
                        c.g[A2][0] = 0x1;
                        break 'b_2d666c;
                    } else {
                        break 'b_2d666c;
                    }
                }
            }
        }
    }
    'b_2d6694: {
        c.cycles += 2;
        let t = c.g[A2][0] == 0u64;

        if t {
            break 'b_2d6694;
        } else {
            c.cycles += 8;
            c.g[AT][0] = 0x41200000;
            c.mtc1(1, 0);
            c.stw_2(SP, 0xc, 0u64);
            c.swc1(0, c.pa(SP, 0x8));
            c.swc1(0, c.pa(SP, 0));
            c.swc1(0, c.pa(SP, 0x4));
            c.lq(2, c.pa(SP, 0));
            c.sq(2, c.pa(A1, 0));
            break 'b_2d6694;
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    c.pc = ra;
    return;
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
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::fill;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::put_words;
    use crate::difftest::RA as TEST_RA;
    use ssx3_ee_rt::funcs::GuestFn;
    use crate::difftest::RA as SENTINEL;
    use ssx3_ee as ee;
    use crate::difftest::elf_image;
    use crate::difftest::compare;
    use crate::difftest::put_vec;
    pub(crate) use crate::game::wscript::wscriptman::tests::SEEDS_6 as SEEDS;
    pub(crate) use crate::game::wscript::wscriptman::tests::gp_at;
    pub(crate) use crate::game::wscript::wscriptman::tests::load;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_3 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::alloc_ready;
    pub(crate) use crate::game::wscript::wscriptman::tests::dummy_world;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::h;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_5 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_4 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_4 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_3 as W;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_8 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::eafile_pools;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_7 as run;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_8 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_9 as pool_allocator;
    pub(crate) use crate::game::input::inputmap::tests::alloc;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_3 as alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;
    pub(crate) use crate::game::bx::bxstring2::tests::W_2;
    pub(crate) use crate::game::bx::bxstring2::tests::RM;
    pub(crate) use crate::game::bx::bxstring2::tests::RIDERS;
    pub(crate) use crate::game::bx::bxstring2::tests::BE_2;
    pub(crate) use crate::game::bx::bxstring2::tests::DB;
    pub(crate) use crate::game::bx::bxstring2::tests::VTS;
    pub(crate) use crate::game::bx::bxstring2::tests::base_3 as base_2;
    pub(crate) use crate::game::bx::bxstring2::tests::f;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE_3;
    pub(crate) use crate::game::be::bedbprofile::tests::strict as strict_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_3;
    pub(crate) use crate::game::render::graphicsman::tests::same;


    #[test]
    fn avalanche_compute_instance_fade_matches() {
        check(ssx3_ee::s09::f_002d7c00, avalanche_compute_instance_fade, 300, &|c, seed| {
            let mut r = Rng(0x1003 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let inst = DATA + 0x400;
            c.bus.write32(DATA + 0x2e0, inst);
            c.bus.write32(inst + 0x100, r.edge_f32());
            c.bus.write32(inst + 0x104, r.edge_f32());
            c.bus.write32(inst + 0x108, r.edge_f32());
            c.bus.write16(
                inst + 0xf0,
                if seed % 3 == 0 { 2 } else { (r.next_u32() & 0xffff) as u16 },
            );
            c.f[12] = match seed % 5 {
                0 => 0,
                1 => 0x8000_0000,
                2 => 0x7f7f_ffff,
                3 => r.edge_f32(),
                _ => r.f32(1e4),
            };
        });
    }


    /// A node list at DATA + 0x1000, linked at +0xf4, returning the first node
    /// (or 0 for an empty list) and the target node (`node`).
    pub(crate) fn node_list(c: &mut Cpu, r: &mut Rng, ava: u32, found: bool) -> u64 {
        let n = (r.next_u32() % 4) as u32;
        let mut node = DATA + 0x1000;
        c.bus.write32(ava + 4, if n == 0 { 0 } else { node });
        let mut target = DATA + 0x900;
        for i in 0..n {
            if found && i == n - 1 {
                target = node;
            }
            c.bus.write32(node + 0xf4, if i + 1 < n { node + 0x10 } else { 0 });
            node += 0x10;
        }
        target as u64
    }


    #[test]
    fn avalanche_add_node_event_matches() {
        check_events(
            ssx3_ee::s09::f_002d9738,
            avalanche_add_node_event,
            80,
            &|c, seed| {
                let mut r = Rng(0x2d97_3800 ^ seed as u64);
                scramble(c, &mut r);
                let ava = DATA;
                c.g[A0][0] = ava as u64;
                c.g[A1][0] = (r.next_u32() & 0xffff) as u64;
                let found = r.next_u32() % 2 == 0;
                let target = node_list(c, &mut r, ava, found);
                c.g[A2][0] = if seed % 7 == 0 { r.next_u32() as u64 } else { target };
                c.bus.write16(ava + 0xe, if seed % 5 == 0 { 0x40 } else { (r.next_u32() % 0x40) as u16 });
                for i in 0..0x40 {
                    c.bus.write16(ava + 0x110 + 2 * i, r.next_u32() as u16);
                }
                let mode = schedule(c, &mut r, seed, 0x002d_9738);
                if mode != Events::Off {
                    c.next_event = c.cycles;
                }
                mode
            },
        );
    }


    #[test]
    fn avalanche_build_active_node_array_matches() {
        check_events(
            ssx3_ee::s09::f_002d7e60,
            avalanche_build_active_node_array,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 24, 0);
                let ava = DATA;
                c.g[A0][0] = ava as u64;
                let n = r.next_u32() % 4;
                let mut node = DATA + 0x1000;
                c.bus.write32(ava + 0x4, if n == 0 { 0 } else { node });
                for i in 0..n {
                    c.bus.write32(node + 0x2e4, if i + 1 < n { node + 0x10 } else { 0 });
                    node += 0x10;
                }
                c.bus.write32(ava + 0x8, r.next_u32());
                let mode = schedule(c, &mut r, seed, 0x002d_7e60);
                if mode != Events::Off {
                    c.next_event = c.cycles;
                }
                mode
            },
        );
    }


    #[test]
    fn avalanche_destroy_all_thunk_matches() {
        check(ssx3_ee::s09::f_002f7e70, avalanche_destroy_all_thunk, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x7e70, 0);
            // Kill the singleton so the callee takes its short path.
            c.bus.write32(GAME_GP.wrapping_add(0x9c0), 0);
            let _ = &mut r;
        });
    }


    #[test]
    fn avalanche_bake_group_matches() {
        check(
            ee::s09::f_002d9660__generated,
            avalanche_bake_group,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2d96_60, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x40);
                dummy_world(c, DATA + 0x2000);
                let nodes = (seed % 3) as u32;
                let mut next = 0u32;
                for i in (0..nodes).rev() {
                    let node = DATA + 0x6000 + 0x200 * i;
                    clear(c, node, 0x80);
                    c.bus.write32(node + 0xf4, next);
                    next = node;
                }
                c.bus.write32(this + 4, next);
                c.g[A0][0] = sx(this);
            },
        );
    }


    #[test]
    fn avalanche_set_history_data_matches() {
        check(ssx3_ee::s09::f_002d9fb8__generated, avalanche_set_history_data, SEEDS_2, &|c, seed| {
            let _ = machine(c, seed, 0x2d9fb8, 0);
            let data = DATA + 0x1000;
            c.g[A0][0] = data as u64;
            w32(c, data, if seed % 4 == 0 { 0x02be_ef00 } else { 0x1234_5678 });
            w32(c, GAME_GP + 0x9c0, 0); // no live history blocks
            // Never the (freeing) "old data with buffers" combination.
            w32(c, GAME_GP + 0x9c4, if seed % 2 == 0 { 0 } else { DATA + 0x3000 });
            w32(c, GAME_GP + 0x9c8, 0);
        });
    }


    #[test]
    fn avalanche_release_instance_matches() {
        check(ssx3_ee::s09::f_002d7dd8__generated, avalanche_release_instance, SEEDS_3, &|c, seed| {
            let mut r = machine(c, seed, 0x2d7dd8, 4);
            be_library(c);
            world(c);
            let av = DATA + 0x100;
            c.g[A0][0] = av as u64;
            c.g[A1][0] = (seed % 2) as u64;
            if seed % 4 == 0 {
                w(c, av + 0x2e0, 0);
            } else {
                let first = DATA + 0x300;
                let live = DATA + 0x400;
                let vt = DATA + 0x500;
                w(c, av + 0x2e0, first);
                w(c, first + 0xf0, 2);
                w(c, first + 0xf8, live);
                w(c, live + 0xf0, 0);
                w(c, live + 0xf8, 0);
                if seed % 3 == 0 {
                    w(c, live + 0xc, 0);
                } else {
                    let sub = DATA + 0x600;
                    w(c, live + 0xc, sub);
                    w(c, sub + 0xc, vt);
                    slot(c, vt, 8, 0, FAKE);
                }
            }
        });
    }


    #[test]
    fn vfx_avalanche_dtor_matches() {
        check(
            ee::s09::f_002d6378__generated,
            vfx_avalanche_dtor,
            SEEDS_4,
            &alloc_base_setup(|c, seed, _r| {
                let free_self = seed & 1;
                let this = if free_self != 0 { alloc(c, 0x80) } else { DATA };
                c.g[A0][0] = this as u64;
                c.g[A1][0] = free_self as u64;
            }),
        );
    }


    #[test]
    fn avalanche_get_object_matrix_matches() {
        check(
            ee::s09::f_002d9c00__generated,
            avalanche_get_object_matrix,
            60,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x2d9c);
                let out = DATA + 0x400;
                let node0 = 0x004e_e770u32;
                let hit = (seed % 3 != 0) as u32;
                c.g[A0][0] = if hit == 1 { (seed % 64) as u64 } else { 0x1234 };
                c.g[A1][0] = sx(out);
                let node = node0 + hit * 0x2f0;
                c.bus.write32(node + 0x2e0, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0xf8, seed as u32 % 64);
                for k in 0..4 {
                    c.bus.write32(node + 0x70 + 4 * k, r.next_u32());
                }
                c.bus.write32(node + 0xb0, f(2.0));
                for k in 0..4 {
                    c.bus.write32(node + 0x60 + 4 * k, r.next_u32());
                }
                fill(c, &mut r, out, 0x40);
            },
        );
    }


    #[test]
    fn avalanche_free_histories_matches() {
        strict(ee::s09::f_002d9a80__generated, avalanche_free_histories, 30, &|c, seed| {
            let mut r = machine(c, seed, 0x2d9a, 8);
            let base = DATA + 0x1000;
            for k in 0..16 {
                c.bus.write32(base + 0x1c * k, (r.next_u32() % 2) as u32);
            }
            c.bus.write32(GAME_GP + 0x9c0, if seed % 3 == 0 { 0 } else { DATA + 0x2000 });
            c.bus.write32(GAME_GP + 0x9c8, if seed % 2 == 0 { 0 } else { 1 });
            c.bus.write32(DATA + 0x2000 + 4, DATA + 0x3000);
            c.bus.write32(DATA + 0x3000 + 0xf4, 0);
            c.bus.write32(DATA + 0x3000 + 0xfc, 0);
            c.bus.write8(DATA + 0x3000 + 0xf3, 1);
            c.g[A0][0] = sx(DATA);
        });
    }


    #[test]
    fn vfx_avalanche_init_matches() {
        strict_2(
            ee::s09::f_002d6410__generated,
            vfx_avalanche_init,
            4,
            &|c, seed| {
                let _r = base_3(c, seed, 0x2d64);
                for i in 0..0x20 {
                    c.bus.write32(0x004d_5b70 + 0x30 * i, seed + i as u32);
                    c.bus.write32(0x004e_db70 + 0x30 * i, seed + i as u32);
                }
            },
        );
    }


    #[test]
    fn avalanche_clamp_vector_matches() {
        same(
            ssx3_ee::s09::f_002d64d8__generated,
            avalanche_clamp_vector,
            200,
            &|c, seed| {
                let mut r = Rng(0x2d64d8 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let v = DATA + 0x100;
                c.g[A0][0] = (DATA + 0x200) as u64;
                c.g[A1][0] = v as u64;
                c.bus.write32(GAME_GP + 0x9a4, r.f32(100.0));
                let mut q = [r.edge_f32(), r.edge_f32(), r.edge_f32(), r.edge_f32()];
                if seed % 3 == 0 {
                    q[(seed as usize) % 4] = 0x7f80_0000;
                }
                put_vec(c, v, q);
            },
        );
    }
}
