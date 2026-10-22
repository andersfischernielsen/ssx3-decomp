//! `src/main/gamemode.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::abi;
use crate::abi::r32;
use crate::call::sp;
use crate::call::Flow;
use crate::call::VSlot;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::X;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
pub(crate) use crate::game::sound::bankmonitor::p;
pub(crate) use crate::game::sound::bankmonitor::add32;
pub(crate) use crate::game::sound::bankmonitor::leave;
pub(crate) use crate::game::fe::festateoptions::gpa;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::feasyncfile::shl32;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::gp;
pub(crate) use crate::game::fe::fepopup::at as at_2;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw;
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::fe::fepopup::lq;
pub(crate) use crate::game::fe::fepopup::sq;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0023_80e8, game_mode_man_destroy, "GameModeMan_Destroy"),
    (0x0023_8510, game_mode_man_handler_v06, "GameModeMan_HandlerV06"),
    (0x0023_8550, game_mode_man_handler_v07, "GameModeMan_HandlerV07"),
    (0x0023_8590, game_mode_man_handler_v09, "GameModeMan_HandlerV09"),
    (0x0023_8b70, game_mode_man_sort_by_score, "GameModeMan_SortByScore"),
    (0x0023_8bf8, game_mode_man_sort_by_time, "GameModeMan_SortByTime"),
    (0x0023_8c80, gm_handler_start, "GMHandler_Start"),
    (0x0023_8d30, gm_handler_clear_standings, "GMHandler_ClearStandings"),
    (0x0023_8da8, gm_handler_clear_standings2, "GMHandler_ClearStandings2"),
    (0x0023_9ce0, gm_handler0_v09, "GMHandler0_v09"),
    (0x0023_9d18, gm_handler0_save_state, "GMHandler0_SaveState"),
    (0x0023_9d50, gm_handler0_load_state, "GMHandler0_LoadState"),
    (0x0023_9d88, gm_handler9_reset, "GMHandler9_Reset"),
    (0x0023_a070, gm_handler9_v06, "GMHandler9_v06"),
    (0x0023_ac18, gm_handler1_save_state, "GMHandler1_SaveState"),
    (0x0023_ac50, gm_handler1_load_state, "GMHandler1_LoadState"),
    (0x0023_ae00, gm_handler8_on_rider_finished, "GMHandler8_OnRiderFinished"),
    (0x0023_b038, gm_handler8_save_state, "GMHandler8_SaveState"),
    (0x0023_b070, gm_handler8_load_state, "GMHandler8_LoadState"),
    (0x0023_b5f8, gm_handler4_v06, "GMHandler4_v06"),
    (0x0023_c560, gm_handler7_v06, "GMHandler7_v06"),
    (0x0023_c770, game_rand_shuffle, "GameRand_Shuffle"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RESET_FOR_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INIT_FROM_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::DYNAMIC_EMITTER_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_END_OPERATION;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_ON_OPERATION_DONE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUY_POPUP_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::CLEAR_NEW_REWARDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_RECORD_BEST_JUMP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_ADD_BOOST_2 as RIDER_ADD_BOOST;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_APPLY_SCRIPT_TRACK;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_UPDATE_SCRIPT_SOUNDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SNOW_BOX_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_CTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::VRAM_HEAP_INIT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TEX_MAN_INIT_TABLE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_COPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_RAW;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GROW_LINES;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_EVENT_STAT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_CTOR_2_2 as UI_STATE_CTOR_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOAD_INDICATOR_RENDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_FIND_BY_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST as UI_LIST_FIRST_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_NODE_SENTINEL;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SECTION_SELECT_PEAK_LIGHTING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INSTANCE_GET_NODE_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::AVALANCHE_GET_OBJECT_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_PROCESS_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::WORLD_GET_INSTANCE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_TRACK_INFO;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_SET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_GET_ROOM_NAME;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_COUNT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::FE_ASYNC_SET_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::POPUP_CONFIRM_REBUILD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STREAM_FILE_QUEUE_RANGE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_CPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_MGR_PUSH_POPUP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LISTBOX_SET_ITEM_TEXT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SET_ITEM_YEAR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_ASCII;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_MISSION_SET_COLLECTED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MSG_PRINTF;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::QSORT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_TIMER_SERVICE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_THREAD_YIELD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_MODULE_REQUEST_STATE_2 as GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_API_SEND;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_RESUME_SFX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_IS_TRACK_GROUP5;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GATHER_CAM_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MEM_REPORT_STATS_LOCKED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST_WRAPPER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SERIALIZE_READ_U32;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUILTIN_DELETE_6 as BUILTIN_DELETE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_FREE_4 as EA_MEM_FREE_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_PUSH_2 as ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::CLEAR_MUSIC_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_CONFIGURE_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_UPDATE_EMITTER_ENABLES;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_PROFILE_5 as BE_GET_RIDER_PROFILE_3;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_6 as UI_LIST_FIRST_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_DRAW_RECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_RESET_PROFILE_CHARACTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COPY_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_CREATE_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_LEVEL_TO_VOLUME_2 as AUDIO_LEVEL_TO_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_CHANNEL_VOLUME_2 as AUDIO_SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCPY_2;
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
    pub(crate) use crate::game::dirtysock::tags2::callee::MEMSET_3 as MEMSET_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::STR_HASH_4 as STR_HASH_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPRINTF_6 as SPRINTF_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::FORMAT_TAGGED;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_RESET_PROFILE_CHARACTERS_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_NEW_PLAYER_INT_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_CHAR_INT_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_BAGT_INT_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_MEM_MAN_ALLOC as C_MEM_MAN_ALLOC_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::TAG_FIELD_FIND;
    pub(crate) use crate::game::dirtysock::tags2::callee::TAG_FIELD_GET_NUMBER;
    pub(crate) use crate::game::dirtysock::tags2::callee::CBE_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_CHAR_ID;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_ALT_CHAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::GM_PICK_OPPONENTS;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_SET_MODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_GET_PRODUCT_CODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_GET_SAVE_TYPE_TAG;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_00152BA8;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_00152948;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_002C2540;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_002C26D0;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_IS_FLAG_SET_2 as MESSAGE_CENTER_IS_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_SET_FLAG_2 as MESSAGE_CENTER_SET_FLAG;
    pub(crate) use crate::game::dirtysock::tags2::callee::STR_INTERN;
    pub(crate) use crate::game::dirtysock::tags2::callee::F_0039E2A0;
    pub(crate) use crate::game::dirtysock::tags2::callee::SNPRINTF;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBYMAN_SEND_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_1;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_4;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FLUSH;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_5;
    pub(crate) use crate::game::dirtysock::tags2::callee::IS_PA_SPEECH_ON;
    pub(crate) use crate::game::dirtysock::tags2::callee::IS_REPLAY_PLAYING;
    pub(crate) use crate::game::dirtysock::tags2::callee::RIDER_123168;
    pub(crate) use crate::game::dirtysock::tags2::callee::IS_RACE_TYPE_4OR5;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_ADD_TIMER_2 as BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPEECH_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_LANGUAGE_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_RIDER_VOICE_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_FINISH_SPEECH_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPEECH_QUEUE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FIND;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_REMOVE;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_DEACTIVATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SETUPMAN_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::TEXT_OBJ;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEASURE_PLAIN;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEASURE_WRAP;
    pub(crate) use crate::game::dirtysock::tags2::callee::ANIM_GET_LAYER_SEQUENCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IS_LOCAL_PLAYER;
    pub(crate) use crate::game::dirtysock::tags2::callee::GET_TRANSITION_MAN;
    pub(crate) use crate::game::dirtysock::tags2::callee::TRANSITION_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::EFFECT_ALLOC;
    pub(crate) use crate::game::dirtysock::tags2::callee::EFFECT_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::TRANSITION_START;


    // s00
    pub const RIDER_METRIX_RESET_STATS: Callee = Callee { addr: 0x0011_75b8, f: ee::s00::f_001175b8 };

    pub const RIDER_METRIX_CLEAR_HUD_MESSAGES: Callee =
        Callee { addr: 0x0011_73b8, f: ee::s00::f_001173b8 };

    pub const RIDER_METRIX_RESET_TRICK: Callee =
        Callee { addr: 0x0011_7838, f: ee::s00::f_00117838 };

    pub const RIDER_METRIX_RESET_COMBO: Callee =
        Callee { addr: 0x0011_75f8, f: ee::s00::f_001175f8 };

    pub const RIDER_METRIX_BANK_PENDING_SCORE: Callee =
        Callee { addr: 0x0011_a7a8, f: ee::s00::f_0011a7a8 };

    pub const RIDER_GET_SYNC_CHECKSUM: Callee =
        Callee { addr: 0x0012_5ad8, f: ee::s00::f_00125ad8 };


    // s01
    pub const CBE_GET_BE: Callee = Callee { addr: 0x0014_dc80, f: ee::s01::f_0014dc80 };

    pub const BE_GET_RIDER_PROFILE: Callee = Callee { addr: 0x0014_a0e0, f: ee::s01::f_0014a0e0 };

    pub const BE_NEW_PLAYER_INT_GET_SLOT: Callee =
        Callee { addr: 0x0014_7798, f: ee::s01::f_00147798 };

    pub const BE_REWARD_INT_GET_MEDAL_REWARD_ID: Callee =
        Callee { addr: 0x0015_9080, f: ee::s01::f_00159080 };

    pub const BE_REWARD_INT_IS_REWARD_GRANTED: Callee =
        Callee { addr: 0x0015_a2e0, f: ee::s01::f_0015a2e0 };

    pub const BE_REWARD_INT_GRANT_REWARD: Callee =
        Callee { addr: 0x0015_9cd0, f: ee::s01::f_00159cd0 };


    // s02
    pub const CAMERA_TRIGGER_MAN_UNLOAD: Callee =
        Callee { addr: 0x0016_c9b0, f: ee::s02::f_0016c9b0 };

    pub const CAMERA_TRIGGER_MAN_PARSE: Callee =
        Callee { addr: 0x0016_c228, f: ee::s02::f_0016c228 };

    pub const CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS_MASK2: Callee =
        Callee { addr: 0x0016_d1b8, f: ee::s02::f_0016d1b8 };

    pub const UI_OBJECT_SHOW_CLAMP_ALPHA_HALF: Callee =
        Callee { addr: 0x0019_4498, f: ee::s02::f_00194498 };


    // s03
    pub const FE_MODULE_UNLOAD_RIDER_PREVIEWS: Callee =
        Callee { addr: 0x001a_04c8, f: ee::s03::f_001a04c8 };

    pub const FE_MODULE_RIDER_PREVIEW_DTOR: Callee =
        Callee { addr: 0x0019_e498, f: ee::s03::f_0019e498 };

    pub const FE_ASYNC_MANAGER_SET_FILE_STATUS: Callee =
        Callee { addr: 0x001a_39d8, f: ee::s03::f_001a39d8 };


    // s07
    pub const REPLAY_FIND_SNAPSHOT: Callee = Callee { addr: 0x0026_e448, f: ee::s07::f_0026e448 };

    pub const RUN_LIST_SEEK: Callee = Callee { addr: 0x0026_d4d8, f: ee::s07::f_0026d4d8 };

    pub const SCRIPT_LIST_ON_SEQ_ADVANCE: Callee =
        Callee { addr: 0x0027_7310, f: ee::s07::f_00277310 };

    pub const SCRIPT_ENGINE_GET_SEQ_PROCESS_ID: Callee =
        Callee { addr: 0x0028_2be0, f: ee::s07::f_00282be0 };

    pub const REPLAY_REQUEST_INSTANT_REPLAY: Callee =
        Callee { addr: 0x0027_0658, f: ee::s07::f_00270658 };


    // s08
    pub const USTR_TO_ASCII: Callee = Callee { addr: 0x002c_2580, f: ee::s08::f_002c2580 };

    pub const CPS2_MEM_CARD_SET_STATE: Callee =
        Callee { addr: 0x002c_48c0, f: ee::s08::f_002c48c0 };


    // s09
    pub const MENU_SYS_ITEM_DRAW_ROW: Callee =
        Callee { addr: 0x002c_a988, f: ee::s09::f_002ca988 };


    // s10
    pub const WSCRIPT_MAN_RESET_COLLECT_SET: Callee =
        Callee { addr: 0x0030_c390, f: ee::s10::f_0030c390 };

    pub const OPERATOR_NEW_NAMED: Callee = Callee { addr: 0x0031_7e30, f: ee::s10::f_00317e30 };

    pub const CBX_STRING_ALLOC_BUFFER: Callee =
        Callee { addr: 0x0031_8040, f: ee::s10::f_00318040 };

    pub const EA_MEM_FREE: Callee = Callee { addr: 0x0031_7e98, f: ee::s10::f_00317e98 };

    pub const BUILTIN_DELETE: Callee = Callee { addr: 0x0031_7e50, f: ee::s10::f_00317e50 };


    // s11
    pub const MODIFIER_BLOCK_HAS_RAIL_MODIFIER: Callee =
        Callee { addr: 0x0035_3418, f: ee::s11::f_00353418 };

    pub const RAIL_MODIFIER_CTOR: Callee = Callee { addr: 0x0035_b708, f: ee::s11::f_0035b708 };

    pub const MOD_OBJ_SET_RAIL_MODIFIER: Callee =
        Callee { addr: 0x0035_56a8, f: ee::s11::f_003556a8 };

    pub const MULTI_PARTICLE_DTOR: Callee = Callee { addr: 0x0035_7fa0, f: ee::s11::f_00357fa0 };

    pub const MOD_OBJ_CTOR_2: Callee = Callee { addr: 0x0035_52d8, f: ee::s11::f_003552d8 };

    pub const FLEX_BRIDGE_ALLOC_SPRING_NODES: Callee =
        Callee { addr: 0x0035_3f00, f: ee::s11::f_00353f00 };


    // s12
    pub const TEX_MAN_UNLOCK_VRAM: Callee = Callee { addr: 0x0036_7b60, f: ee::s12::f_00367b60 };


    // s13
    pub const UI_LIST_NODE_IS_SENTINEL: Callee =
        Callee { addr: 0x0039_7718, f: ee::s13::f_00397718 };

    pub const UI_LIST_FIRST: Callee = Callee { addr: 0x0039_7840, f: ee::s13::f_00397840 };

    pub const UI_LIST_DELETE_ALL: Callee = Callee { addr: 0x0039_7948, f: ee::s13::f_00397948 };

    pub const UI_LIST_ACCEPT_VISITOR: Callee =
        Callee { addr: 0x0039_79f8, f: ee::s13::f_003979f8 };

    pub const UI_LIST_NODE_DTOR_BODY: Callee =
        Callee { addr: 0x003a_4868, f: ee::s13::f_003a4868 };

    pub const CUI_TEXT_SCROLL_SET_WIDE_STRING: Callee =
        Callee { addr: 0x003a_1d30, f: ee::s13::f_003a1d30 };

    pub const CSTREAM_MAN_RESET: Callee = Callee { addr: 0x003a_6ed8, f: ee::s13::f_003a6ed8 };

    pub const CSTREAM_MAN_FREE_BUFFERS: Callee =
        Callee { addr: 0x003a_6f38, f: ee::s13::f_003a6f38 };


    // Cross-shard calls the generator wrote as `call_indirect`.
    pub const STR_HASH: u32 = 0x0031_7670;

    pub const SPRINTF: u32 = 0x0041_7828;

    pub const MEMSET: u32 = 0x0041_6210;

    /// The libc `memset` copy in the world/serialisation shard.
    pub const MEMSET_3E6448: u32 = 0x003e_6448;

    pub const MEMCPY: u32 = 0x0041_605c;

    pub const STRLEN: u32 = 0x0041_6810;

    pub const STRCPY: u32 = 0x0041_66f4;

    pub const CVT_FLOAT_TO_INT: u32 = 0x0041_3af8;

    pub const C_UI_SCREEN_GET_OBJECT_BY_HASH_NAME: u32 = 0x0039_d860;

    pub const C_UI_ENGINE_ADD_SCREEN_BY_HASH_NAME: u32 = 0x0039_7cc0;

    pub const C_UI_SCREEN_PLAY_FRAME: u32 = 0x0039_c870;

    pub const SSX_AUDIO_SYSTEM_GET_INSTANCE: u32 = 0x0028_b180;

    pub const FUNC_0028F140: u32 = 0x0028_f140;

    pub const C_MEM_MAN_ALLOC: u32 = 0x0031_7d70;

    pub const AI_RAND: u32 = 0x0031_7810;
}


// ---------------------------------------------------------------------------
// GM save/load handlers: `save`/`load` are virtual methods on the handler
// object at `this`; each handler forwards its own state block.

/// One GM handler forwarder: `fn(this, handler, size)` calls the handler's
/// virtual method at `slot` with `(handler + delta, this + data_off, size)`.
pub(crate) fn gm_handler_forward(c: &mut Cpu, this_off: u32, save_slot: u32, size: u64, ret: u32) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(this_off));
        c.g[A2][0] = size;
        c.g[V1][0] = Addr(c.g[A3][0] as u32).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(save_slot)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(save_slot + 4)).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void GMHandler0_SaveState(GMHandler0* self, Handler* h)` (0x239d18).
pub fn gm_handler0_save_state(c: &mut Cpu) {
    gm_handler_forward(c, 4, 0x08, 0xa0, 0x0023_9d40);
}


/// `void GMHandler0_LoadState(GMHandler0* self, Handler* h)` (0x239d50).
pub fn gm_handler0_load_state(c: &mut Cpu) {
    gm_handler_forward(c, 4, 0x10, 0xa0, 0x0023_9d78);
}


/// `void GMHandler1_SaveState(GMHandler1* self, Handler* h)` (0x23ac18).
pub fn gm_handler1_save_state(c: &mut Cpu) {
    gm_handler_forward(c, 0, 0x08, 0xd0, 0x0023_ac40);
}


/// `void GMHandler1_LoadState(GMHandler1* self, Handler* h)` (0x23ac50).
pub fn gm_handler1_load_state(c: &mut Cpu) {
    gm_handler_forward(c, 0, 0x10, 0xd0, 0x0023_ac78);
}


/// `void GMHandler8_SaveState(GMHandler8* self, Handler* h)` (0x23b038).
pub fn gm_handler8_save_state(c: &mut Cpu) {
    gm_handler_forward(c, 0, 0x08, 0x2c, 0x0023_b060);
}


/// `void GMHandler8_LoadState(GMHandler8* self, Handler* h)` (0x23b070).
pub fn gm_handler8_load_state(c: &mut Cpu) {
    gm_handler_forward(c, 0, 0x10, 0x2c, 0x0023_b098);
}


/// `lw` of a `gp`-relative global, as a register value.
#[inline(always)]
pub(crate) fn gp_lw(c: &mut Cpu, off: i32) -> u64 {
    Addr(gp(c, off)).lw(c)
}


/// The body of the `GameModeMan_HandlerV*` thunks: `obj =
/// D_00536668[*(int*)(self+4)]`, then `vt = *(void**)obj` and the vtable slot
/// at `slot` called with `a0 = obj + delta` and the other arguments as given.
pub(crate) fn game_mode_handler(c: &mut Cpu, slot: u32, ret: u32) -> Flow {
    c.cycles += 16;
    let frame = Frame::open(c, 0x10);
    c.g[V1][0] = 0x0053_6668;
    frame.save_ra(c, 0);
    c.g[V0][0] = Addr(p(abi::arg(c, 0), 4)).lw(c);
    c.g[V0][0] = sx((r32(c, V0) as u32) << 2);
    c.g[V0][0] = add32(c.g[V0][0], c.g[V1][0]);
    let index_at = r32(c, V0);
    c.g[A3][0] = Addr(index_at).lw(c);
    c.g[V1][0] = Addr(r32(c, A3)).lw(c);
    let vtable = c.g[V1][0];
    let entry = VSlot::read(c, vtable, slot);
    c.g[A0][0] = sx(entry.delta as u32);
    c.g[V0][0] = entry.func;
    c.g[RA][0] = sx(ret);
    c.g[A0][0] = abi::addu(c.g[A3][0], entry.delta);
    let func = entry.func as u32;
    c.pc = func;
    call_indirect(c, func);
    if c.pc != ret {
        return Err(crate::call::Unwound);
    }
    frame.restore_ra(c, 0);
    leave(c, 0x10);
    Ok(())
}


/// `void GameModeMan_HandlerV06(self, a, b)` (0x238510): vtable slot 6.
pub fn game_mode_man_handler_v06(c: &mut Cpu) {
    guest(c, |c| game_mode_handler(c, 0x30, 0x0023_8544))
}


/// `void GameModeMan_HandlerV07(self, a, b)` (0x238550): vtable slot 7.
pub fn game_mode_man_handler_v07(c: &mut Cpu) {
    guest(c, |c| game_mode_handler(c, 0x38, 0x0023_8584))
}


/// `void GameModeMan_HandlerV09(self, a, b)` (0x238590): vtable slot 9.
pub fn game_mode_man_handler_v09(c: &mut Cpu) {
    guest(c, |c| game_mode_handler(c, 0x48, 0x0023_85c4))
}


/// `int GMHandler0_v09(Handler* self, int col, int row)` (0x239ce0): a
/// 0x28-byte-row table lookup `self->rows[row][col]` (word at
/// `+row*0x28 + col*4 + 4`), 0 when `row` is past the count (`gp-0x480`).
pub fn gm_handler0_v09(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = Addr(gpa(c, -0x480)).lw(c);
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A2][0] as i64)) as u64;
    let out_of_range = c.g[V0][0] != 0;
    c.g[V0][0] = 0x28;
    if out_of_range {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 6;
    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
    c.mult(0, 2, 6, 2);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
    abi::ret(c);
}


/// `int GMHandler9_v06(Handler9* self, int player, int other)` (0x23a070):
/// bump `self->score[player]` and report whether it is now strictly ahead of
/// the other player; if so and the race is running, accumulate the frame
/// time into `manager->+0x78` (capped at the race's frame limit).
pub fn gm_handler9_v06(c: &mut Cpu) {
    const ONE: u64 = 0x1;
    c.cycles += 17;
    let self_ = r32(c, A0);
    let player = r32(c, A1);
    c.g[A0][0] = sx(self_.wrapping_add(0x4)); // score array
    c.g[A3][0] = sx(player << 1);
    c.g[A3][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A3][0] as u32)));
    c.g[T1][0] = ONE;
    c.g[V1][0] = c.lhu(c.g[A3][0] as u32);
    c.g[V0][0] = c.g[T1][0];
    if c.g[A1][0] != 0 {
        c.g[V0][0] = 0;
    }
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
    c.sh(c.g[A3][0] as u32, c.g[V1][0]);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
    c.g[V1][0] = sx((c.g[V1][0] as u32) << 16);
    c.g[V0][0] = c.lh(c.g[A0][0] as u32);
    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 16) as u32);
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    let ahead = c.g[V0][0] != 0;
    c.g[T0][0] = c.g[A2][0];
    if !ahead {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    c.g[A1][0] = Addr(gpa(c, -0x480)).lw(c);
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
    let not_running = c.g[V0][0] != c.g[T1][0];
    c.g[V0][0] = Addr(gpa(c, -0x848)).lw(c);
    if not_running {
        c.cycles += 2;
        c.g[V0][0] = 0x1;
        abi::ret(c);
        return;
    }
    c.cycles += 7;
    c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x78)); // accumulated
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84)); // race manager
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc)); // race
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8)); // frame limit
    c.g[V0][0] = (c.g[A2][0] < c.g[V0][0]) as u64;
    let limited = c.g[V0][0] != 0;
    c.g[V1][0] = 0x3c;
    if limited {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 5;
    c.g[V0][0] = 0x1;
    c.mult(0, 4, 8, 3);
    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[A2][0] as u32)));
    c.sw((c.g[A1][0] as u32).wrapping_add(0x78), c.g[V1][0]);
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Helpers.

/// The low 32 bits of a register (an object pointer or a 32-bit value).
#[inline]
pub(crate) fn low(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}


/// The current stack pointer.
#[inline]
pub(crate) fn sp_2(c: &Cpu) -> u32 {
    c.g[SP][0] as u32
}


/// `gp + off` for a `gp`-relative global (the game's `gp` is in `c.g[GP]`).
#[inline]
pub(crate) fn gpo(c: &Cpu, off: i32) -> u32 {
    if off >= 0 {
        low(c, GP).wrapping_add(off as u32)
    } else {
        low(c, GP).wrapping_sub((-off) as u32)
    }
}


/// `lw off(reg)`.
#[inline]
pub(crate) fn lw_r(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(low(c, reg).wrapping_add(off)).lw(c)
}


/// `lh off(reg)` (sign-extended).
#[inline]
pub(crate) fn lh_r(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(low(c, reg).wrapping_add(off)).lh(c)
}


/// `sw off(reg)`.
#[inline]
pub(crate) fn sw_r(c: &mut Cpu, reg: usize, off: u32, v: u64) {
    let a = low(c, reg).wrapping_add(off);
    c.sw(a, v);
}


/// `void GameModeMan_Destroy()` (0x2380e8): delete the ten mode singletons and
/// the optional extra object.
pub fn game_mode_man_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x30));
        c.g[V0][0] = 0x53_0000;
        c.sq(16, sp_2(c).wrapping_add(0x20));
        c.sq(17, sp_2(c).wrapping_add(0x10));
        c.g[S0][0] = sx(low(c, V0).wrapping_add(0x6668));
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[S1][0] = 0x9;
        loop {
            c.cycles += 3;
            c.g[A2][0] = Addr(low(c, S0)).lw(c);
            let none = c.g[A2][0] == 0;
            c.g[A1][0] = 0x3;
            if !none {
                c.cycles += 5;
                c.g[V0][0] = Addr(low(c, A2)).lw(c);
                c.g[A0][0] = lh_r(c, V0, 0x8);
                c.g[V1][0] = lw_r(c, V0, 0xc);
                let target = low(c, V1);
                c.g[A0][0] = sx(low(c, A2).wrapping_add(low(c, A0)));
                call_ptr(c, target, 0x0023_8128)?;
            }
            c.cycles += 3;
            c.g[S1][0] = sx(low(c, S1).wrapping_sub(1));
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx(low(c, S0).wrapping_add(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x0023_8108);
                }
                continue;
            }
            c.cycles += 3;
            c.g[A0][0] = Addr(gpo(c, -0x484)).lw(c);
            let gone = c.g[A0][0] == 0;
            c.lq(16, sp_2(c).wrapping_add(0x20));
            if !gone {
                c.cycles += 3;
                call_ptr(c, callee::BUILTIN_DELETE.addr, 0x0023_8148)?;
                c.lq(16, sp_2(c).wrapping_add(0x20));
            }
            c.cycles += 5;
            c.lq(17, sp_2(c).wrapping_add(0x10));
            c.g[RA][0] = Addr(sp_2(c)).ld(c);
            sw_r(c, GP, (-0x484i32) as u32, 0);
            let t = low(c, RA);
            c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
            c.pc = t;
            break;
        }
        Ok(())
    })
}


/// `void GMHandler_ClearStandings()` (0x238d30): zero the five standing tables.
pub fn gm_handler_clear_standings(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x10));
        c.g[A0][0] = 0x53_0000;
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x6730));
        c.g[A1][0] = 0x9;
        c.g[A2][0] = 0x28;
        call_ptr(c, callee::MEMSET, 0x0023_8d4c)?;
        c.g[A0][0] = 0x53_0000;
        c.g[A1][0] = 0;
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x6708));
        c.g[A2][0] = 0x28;
        call_ptr(c, callee::MEMSET, 0x0023_8d60)?;
        c.g[A0][0] = 0x53_0000;
        c.g[A1][0] = 0;
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x6640));
        c.g[A2][0] = 0x28;
        call_ptr(c, callee::MEMSET, 0x0023_8d74)?;
        c.g[A0][0] = 0x53_0000;
        c.g[A1][0] = 0;
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x66a8));
        c.g[A2][0] = 0x28;
        call_ptr(c, callee::MEMSET, 0x0023_8d88)?;
        c.g[A0][0] = 0x53_0000;
        c.g[A1][0] = 0;
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x66d0));
        c.g[A2][0] = 0x18;
        call_ptr(c, callee::MEMSET, 0x0023_8d9c)?;
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}


/// `void GMHandler_ClearStandings2()` (0x238da8): same, but the third table is
/// filled with `0x09`.
pub fn gm_handler_clear_standings2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x10));
        c.g[A0][0] = 0x53_0000;
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x6730));
        c.g[A1][0] = 0x9;
        c.g[A2][0] = 0x28;
        call_ptr(c, callee::MEMSET, 0x0023_8dc4)?;
        c.g[A0][0] = 0x53_0000;
        c.g[A1][0] = 0;
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x6708));
        c.g[A2][0] = 0x28;
        call_ptr(c, callee::MEMSET, 0x0023_8dd8)?;
        c.g[A0][0] = 0x53_0000;
        c.g[A1][0] = 0x9;
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x6640));
        c.g[A2][0] = 0x28;
        call_ptr(c, callee::MEMSET, 0x0023_8dec)?;
        c.g[A0][0] = 0x53_0000;
        c.g[A1][0] = 0;
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x66a8));
        c.g[A2][0] = 0x28;
        call_ptr(c, callee::MEMSET, 0x0023_8e00)?;
        c.g[A0][0] = 0x53_0000;
        c.g[A1][0] = 0;
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x66d0));
        c.g[A2][0] = 0x18;
        call_ptr(c, callee::MEMSET, 0x0023_8e14)?;
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x10));
        c.pc = t;
        Ok(())
    })
}


/// Zero ten score/time table entries at `0x536708` and `0x536730`, then sort
/// both with `qsort` (element size 4, comparator `cmp`).
pub(crate) fn game_mode_man_sort(c: &mut Cpu, head: u32, ret_first: u32, ret_second: u32, cmp: u32) -> Flow {
    c.cycles += 10;
    open(c, 0x30);
    let sp0 = spv(c);
    c.g[A0][0] = 0x530000;
    c.sq(16, at(sp0, 0x20));
    c.g[V1][0] = 0;
    c.sq(17, at(sp0, 0x10));
    c.g[S0][0] = c.g[A1][0];
    c.g[S1][0] = 0x530000;
    c.sd(sp0, c.g[RA][0]);
    c.g[A2][0] = sx(at(c.g[A0][0] as u32, 0x6708));
    c.g[A1][0] = sx(at(c.g[S1][0] as u32, 0x6730));
    loop {
        c.cycles += 7;
        let a1 = c.g[A1][0] as u32;
        let a2 = c.g[A2][0] as u32;
        c.sw(a1, c.g[V1][0]);
        c.sw(a2, c.g[V1][0]);
        c.g[A1][0] = sx(at(a1, 4));
        c.g[V1][0] = sx(at(c.g[V1][0] as u32, 1));
        c.g[V0][0] = ((c.g[V1][0] as i64) < 10i64) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A2][0] = sx(at(a2, 4));
        if !more {
            c.cycles += 17;
            c.g[A3][0] = sx(cmp);
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = sx(at(c.g[A0][0] as u32, 0x6708));
            c.g[A2][0] = 4;
            call_ptr(c, callee::QSORT, ret_first)?;
            c.g[A3][0] = 0x245ec0;
            c.g[A0][0] = sx(at(c.g[S1][0] as u32, 0x6730));
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = 4;
            call_ptr(c, callee::QSORT, ret_second)?;
            c.lq(16, at(sp0, 0x20));
            c.lq(17, at(sp0, 0x10));
            c.g[RA][0] = c.ld(sp0);
            let ra = c.g[RA][0] as u32;
            c.g[SP][0] = sx(at(sp0, 0x30));
            c.pc = ra;
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}


/// `void GameModeMan_SortByScore(GameModeMan* self, void* cmp)` (0x238b70).
pub fn game_mode_man_sort_by_score(c: &mut Cpu) {
    guest(c, |c| {
        game_mode_man_sort(c, 0x0023_8b98, 0x0023_8bcc, 0x0023_8be4, 0x0024_5e30)
    })
}


/// `void GameModeMan_SortByTime(GameModeMan* self, void* cmp)` (0x238bf8).
pub fn game_mode_man_sort_by_time(c: &mut Cpu) {
    guest(c, |c| {
        game_mode_man_sort(c, 0x0023_8c20, 0x0023_8c54, 0x0023_8c6c, 0x0024_5e78)
    })
}


/// `GameRand_Shuffle` (0x23c770): Fisher–Yates shuffle of 25 words using the
/// game RNG (two draws per swap).
pub fn game_rand_shuffle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        let this = r(c, A0);
        c.g[S2][0] = sx(this);
        frame.save(c, S0, 0x40);
        c.g[S3][0] = 0x18;
        frame.save_ra(c, 0);
        loop {
            c.cycles += 5;
            c.g[RA][0] = sx(0x0023_c7a0);
            c.g[S3][0] = sx(r(c, S3).wrapping_sub(1));
            callee::GAME_RAND_NEXT.call(c, 0x0023_c7a0)?;
            c.divu(0, 2, 17);
            c.cycles += 6;
            c.g[S0][0] = c.hi[0];
            c.g[RA][0] = sx(0x0023_c7b8);
            callee::GAME_RAND_NEXT.call(c, 0x0023_c7b8)?;
            c.divu(0, 2, 17);
            c.cycles += 10;
            c.g[S0][0] = shl32(c.g[S0][0], 2);
            c.g[S0][0] = sx(r(c, S0).wrapping_add(r(c, S2)));
            c.g[A0][0] = Addr(r(c, S0)).lw(c);
            c.g[V1][0] = c.hi[0];
            c.g[V1][0] = shl32(c.g[V1][0], 2);
            c.g[V1][0] = sx(r(c, V1).wrapping_add(r(c, S2)));
            c.g[V0][0] = Addr(r(c, V1)).lw(c);
            Addr(r(c, S0)).sw(c, c.g[V0][0]);
            let more = (c.g[S3][0] as i64) >= 0;
            Addr(r(c, V1)).sw(c, c.g[A0][0]);
            if !more {
                c.cycles += 7;
                frame.restore(c, S0, 0x40);
                frame.restore(c, S1, 0x30);
                frame.restore(c, S2, 0x20);
                frame.restore(c, S3, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            if event_due(c) {
                deliver_event_here(c, 0x0023_c798);
            }
        }
    })
}


// ---------------------------------------------------------------------------
// `GMHandler_Start` (0x238c80): reset the game-mode singleton's fields, clear
// its two 0x28-byte arrays and dispatch its entry vtable function.
pub fn gm_handler_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 43;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 0x1;
        frame.save_ra(c, 0);
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        c.g[S0][0] = c.g[A0][0];
        let gm = c.g[V0][0] as u32;
        c.sw(gm.wrapping_add(0x84), c.g[A1][0]);
        c.g[A0][0] = sx(gm.wrapping_add(0x18));
        c.sw(gm.wrapping_add(0x8), c.g[V1][0]);
        c.sw(gm.wrapping_add(0x4), c.g[V1][0]);
        c.g[A1][0] = 0u64;
        c.sw(gm.wrapping_add(0x68), c.g[V1][0]);
        c.g[A2][0] = 0x28;
        c.sw(gm.wrapping_add(0x6c), c.g[V1][0]);
        c.sw(gm.wrapping_add(0x98), 0u64);
        c.sw(gm, 0u64);
        c.sw(gm.wrapping_add(0x70), 0u64);
        c.sw(gm.wrapping_add(0x74), 0u64);
        c.sw(gm.wrapping_add(0x10), 0u64);
        c.sw(gm.wrapping_add(0x14), 0u64);
        c.sw(gm.wrapping_add(0x78), 0u64);
        c.sw(gm.wrapping_add(0x7c), 0u64);
        c.sw(gm.wrapping_add(0x80), 0u64);
        c.sw(gm.wrapping_add(0x88), 0u64);
        c.sw(gm.wrapping_add(0x8c), 0u64);
        c.sw(gm.wrapping_add(0x90), 0u64);
        c.sw(gm.wrapping_add(0x94), 0u64);
        call_ptr(c, 0x00416210, 0x0023_8cf4)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x480));
        c.g[A1][0] = 0u64;
        c.g[A2][0] = 0x28;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
        call_ptr(c, 0x00416210, 0x0023_8d08)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, target, 0x0023_8d1c)?;
        c.lq(16, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int GMHandler7_v06(self)` (0x23c560): the "7" GM handler: dispatch an
/// input event to the active game (or its HUD) for the 17..21 range.
pub fn gm_handler7_v06(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        call_ptr(c, 0x0014_dc80, 0x0023_c578)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0023_c584)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0014_4be0, 0x0023_c590)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 17) as u64;
        c.g[V0][0] = c.g[V0][0] ^ 0x1;
        c.g[A0][0] = 0;
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            c.g[A0][0] = ((c.g[V1][0] as i64) < 22) as u64;
        }
        c.cycles += 2;
        c.g[V0][0] = 0x530000;
        if c.g[A0][0] != 0 {
            c.cycles += 14;
            c.g[A2][0] = Addr(p(c.g[S0][0] as u32, 0xa)).lh(c);
            c.g[A1][0] = Addr(p(c.g[V0][0] as u32, 0x5c12)).lb(c);
            c.g[A0][0] = c.g[S1][0];
            c.g[A3][0] = Addr(p(c.g[S0][0] as u32, 8)).lhu(c);
            call_ptr(c, 0x0014_5668, 0x0023_c5c4)?;
            c.g[A0][0] = Addr(p(r32(c, GP), 0xffff_f7b8)).lw(c);
            c.g[V1][0] = 0x64;
            c.mult(0, 6, 2, 3);
            c.g[V0][0] = Addr(p(c.g[A0][0] as u32, 0x84)).lw(c);
            c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0xc)).lw(c);
            c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x28)).lw(c);
            c.g[A0][0] = Addr(p(c.g[A0][0] as u32, 0x790)).lw(c);
            let none = c.g[A2][0] == 0;
            c.g[A1][0] = Addr(p(c.g[A0][0] as u32, 0x198)).lw(c);
            if !none {
                c.cycles += 2;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub((c.g[A2][0] as u32)));
                call_ptr(c, 0x0011_95d8, 0x0023_c5f0)?;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(p(c.g[S0][0] as u32, 8)).lhu(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.sh(p(c.g[S0][0] as u32, 8), c.g[V0][0]);
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x20);
        c.g[V0][0] = 0x1;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `GMHandler4_v06(this)` (0x23b5f8): when the special event is in the
/// 17..21 range, show the HUD message and count the seconds.
pub fn gm_handler4_v06(c: &mut Cpu) {
    'b_23b640: {
        c.cycles += 17;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        c.g[RA][0] = sx(0x0023b610);
        c.g[S0][0] = c.g[A0][0];
        if call_ptr(c, 0x0014dc80, 0x0023b610).is_err() {
            return;
        }
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0023b61c);
        c.g[A1][0] = 0u64;
        if call_ptr(c, 0x0014dd58, 0x0023b61c).is_err() {
            return;
        }
        c.g[S1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0023b628);
        c.g[A0][0] = c.g[S1][0];
        if call_ptr(c, 0x00144be0, 0x0023b628).is_err() {
            return;
        }
        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 17i64) as u64;
        c.g[V0][0] = c.g[V0][0] ^ 0x1;
        let t = c.g[V0][0] == 0u64;
        c.g[A0][0] = 0u64;
        if t {
            break 'b_23b640;
        } else {
            c.cycles += 1;
            c.g[A0][0] = ((c.g[V1][0] as i64) < 22i64) as u64;
            break 'b_23b640;
        }
    }
    'b_23b6a4: {
        c.cycles += 2;
        let t = c.g[A0][0] == 0u64;
        c.g[V0][0] = 0x530000;
        if t {
            break 'b_23b6a4;
        } else {
            'b_23b698: {
                c.cycles += 17;
                c.g[A2][0] = c.lh((c.g[S0][0] as u32).wrapping_add(0xa));
                c.g[A1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c12));
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x0023b65c);
                c.g[A3][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x8));
                if call_ptr(c, 0x00145668, 0x0023b65c).is_err() {
                    return;
                }
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A0][0] = c.g[V0][0];
                c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x514c));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
                c.cvt_s_w(0, 0);
                c.mul_s(0, 0, 1);
                c.cvt_w_s(1, 0);
                c.mfc1(5, 1);
                let t = c.g[A0][0] == 0u64;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub((c.g[A0][0] as u32)));
                if t {
                    break 'b_23b698;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                    c.g[RA][0] = sx(0x0023b698);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x790));
                    if call_ptr(c, 0x001195a8, 0x0023b698).is_err() {
                        return;
                    }
                    break 'b_23b698;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = c.lhu((c.g[S0][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.sh((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
            break 'b_23b6a4;
        }
    }
    c.cycles += 6;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[V0][0] = 0x1;
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    c.pc = t;
    return;
}


/// `void GMHandler9_Reset(GMHandler9* self)` (0x239d88): clear the race
/// manager's player/handler counters and re-seat the two player interfaces.
pub fn gm_handler9_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 80;
        let f = Frame::open(c, 0x40);
        f.save(c, S2, 0x10);
        f.save(c, S0, 0x30);
        f.save(c, S1, 0x20);
        f.save_ra(c, 0);
        c.g[RA][0] = sx(0x0023_9da4);
        c.g[S2][0] = c.g[A0][0];
        call_ptr(c, 0x0014_dc80, 0x0023_9da4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0023_9db0);
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0023_9db0)?;
        c.g[RA][0] = sx(0x0023_9db8);
        c.g[S1][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x0023_9db8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0023_9dc4);
        c.g[A1][0] = 1;
        call_ptr(c, 0x0014_dd58, 0x0023_9dc4)?;
        // The handler's state block.
        c.g[V1][0] = Addr(gp(c, -0x480)).lw(c);
        c.g[T0][0] = u64::MAX;
        c.g[A2][0] = 3;
        c.g[A3][0] = 1;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x6c), c.g[T0][0]);
        c.g[S0][0] = c.g[V0][0];
        c.sw((c.g[V1][0] as u32).wrapping_add(0x68), c.g[T0][0]);
        c.g[A0][0] = c.g[S1][0];
        c.sw((c.g[V1][0] as u32).wrapping_add(0x88), c.g[A3][0]);
        c.g[A1][0] = 3;
        c.sw((c.g[V1][0] as u32).wrapping_add(0x74), c.g[A2][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x98), c.g[A3][0]);
        c.sw(c.g[V1][0] as u32, c.g[A2][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x70), c.g[A2][0]);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x80), 0);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x84), 0);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x94), 0);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x90), 0);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x9c), 0);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x10), 0);
        c.g[RA][0] = sx(0x0023_9e1c);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x14), 0);
        call_ptr(c, 0x0014_54f8, 0x0023_9e1c)?;
        c.g[V1][0] = 0x3c;
        c.g[A3][0] = Addr(gp(c, -0x480)).lw(c);
        c.mult(0, V0, V0, V1);
        c.g[A2][0] = 0x28;
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x18));
        c.sw((c.g[A3][0] as u32).wrapping_add(0x7c), 0);
        c.sw((c.g[A3][0] as u32).wrapping_add(0x8c), 0);
        c.g[A1][0] = u64::MAX;
        c.g[RA][0] = sx(0x0023_9e44);
        c.sw((c.g[A3][0] as u32).wrapping_add(0x78), c.g[V0][0]);
        call_ptr(c, 0x0041_6210, 0x0023_9e44)?;
        c.g[V1][0] = Addr(c.g[S2][0] as u32).lw(c);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x20)).lh(c);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x24)).lw(c);
        let func = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0023_9e58);
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, func, 0x0023_9e58)?;
        // The two player-ID slots (a1 = 0, then 1) and their names.
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0023_9e64);
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_7488, 0x0023_9e64)?;
        c.g[V1][0] = Addr(gp(c, -0x480)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        c.g[RA][0] = sx(0x0023_9e78);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x18), c.g[V0][0]);
        call_ptr(c, 0x0014_7488, 0x0023_9e78)?;
        c.g[V1][0] = Addr(gp(c, -0x480)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.g[RA][0] = sx(0x0023_9e8c);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
        call_ptr(c, 0x0014_74c8, 0x0023_9e8c)?;
        c.g[V1][0] = Addr(gp(c, -0x480)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 1;
        c.g[RA][0] = sx(0x0023_9ea0);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
        call_ptr(c, 0x0014_74c8, 0x0023_9ea0)?;
        c.g[V1][0] = Addr(gp(c, -0x480)).lw(c);
        f.restore(c, S0, 0x30);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x44), c.g[V0][0]);
        c.sh((c.g[S2][0] as u32).wrapping_add(6), 0);
        c.sh((c.g[S2][0] as u32).wrapping_add(4), 0);
        f.restore(c, S1, 0x20);
        f.restore(c, S2, 0x10);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}


/// `void func_0023AE00(sSelf* self, int idx)` (0x23ae00): rider `idx` finished
/// the race: store the finish time (or 360000 for a DNF), then when every
/// player is finished also fill in the estimate for the others, sort the
/// results by time and mark the handler done.
pub fn gm_handler8_on_rider_finished(c: &mut Cpu) {
    let frame = Frame::open(c, 0xb0);
    'b_23af0c: {
        'b_23ae80: {
            c.cycles += 21;
            c.g[V1][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
            frame.save(c, S7, 0x20);
            sq(c, 30, at_2(c, SP, 0x10));
            c.g[S7][0] = c.g[A0][0];
            frame.save(c, S0, 0x90);
            c.g[FP][0] = c.g[A1][0];
            frame.save(c, S1, 0x80);
            frame.save(c, S2, 0x70);
            frame.save(c, S3, 0x60);
            frame.save(c, S4, 0x50);
            frame.save(c, S5, 0x40);
            frame.save(c, S6, 0x30);
            sd(c, r32(c, SP), c.g[RA][0]);
            c.swc1(20, at_2(c, SP, 0xa0));
            c.g[V0][0] = lw(c, at_2(c, V1, 0x84));
            c.g[S0][0] = lw(c, at_2(c, V0, 0xc));
            c.g[RA][0] = sx(0x0023ae4c);
            c.g[A0][0] = c.g[S0][0];
            if call_ptr(c, 0x0012a250, 0x0023ae4c).is_err() {
                return;
            }
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = sx(r32(c, FP) << 2);
            if t {
                break 'b_23ae80;
            } else {
                c.cycles += 11;
                c.g[V1][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
                c.g[S4][0] = c.g[V0][0];
                c.g[A0][0] = lw(c, at_2(c, V1, 0x28));
                c.g[A1][0] = lw(c, at_2(c, A0, 0x6c0));
                c.g[A0][0] = sx(at_2(c, A0, 0x6c0));
                c.g[V1][0] = lh(c, at_2(c, A1, 0x48));
                c.g[V0][0] = lw(c, at_2(c, A1, 0x4c));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x0023ae78);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
                if call_ptr(c, t, 0x0023ae78).is_err() {
                    return;
                }
                let t = c.g[V0][0] != 0u64;

                if t {
                    break 'b_23af0c;
                } else {
                    break 'b_23ae80;
                }
            }
        }
        'b_23aedc: {
            c.cycles += 10;
            c.g[A1][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
            c.g[A0][0] = sx(r32(c, FP) << 2);
            c.g[S4][0] = c.g[A0][0];
            c.g[V1][0] = lw(c, at_2(c, A1, 0x84));
            c.g[V0][0] = lw(c, at_2(c, V1, 0xc));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
            c.g[V1][0] = lw(c, at_2(c, V0, 0x28));
            c.g[A0][0] = lw(c, at_2(c, V1, 0x480));
            let t = c.g[A0][0] == 0u64;
            c.g[V1][0] = 0x530000;
            if t {
                break 'b_23aedc;
            } else {
                c.cycles += 13;
                c.g[A0][0] = 0x50000;
                c.g[A0][0] = c.g[A0][0] | 0x7e40;
                c.g[A1][0] = sx(r32(c, S7).wrapping_add(r32(c, S4)));
                c.g[V0][0] = 0x530000;
                c.g[V1][0] = sx(at_2(c, V1, 0x6640));
                sw(c, at_2(c, A1, 0x4), c.g[A0][0]);
                c.g[V0][0] = sx(at_2(c, V0, 0x66a8));
                c.g[V1][0] = sx(r32(c, S4).wrapping_add(r32(c, V1)));
                c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
                c.g[A1][0] = 0x1;
                sw(c, r32(c, V1), c.g[A0][0]);
                let t = true;
                sw(c, r32(c, V0), c.g[A1][0]);
                if t {
                    break 'b_23af0c;
                } else {
                    break 'b_23aedc;
                }
            }
        }
        c.cycles += 12;
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, S4)));
        c.g[A2][0] = lw(c, at_2(c, A0, 0x28));
        c.g[V0][0] = 0x530000;
        c.g[V1][0] = sx(at_2(c, V1, 0x6640));
        c.g[A0][0] = sx(r32(c, S7).wrapping_add(r32(c, S4)));
        c.g[A1][0] = lw(c, at_2(c, A2, 0x478));
        c.g[V0][0] = sx(at_2(c, V0, 0x66a8));
        c.g[V1][0] = sx(r32(c, S4).wrapping_add(r32(c, V1)));
        c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
        sw(c, at_2(c, A0, 0x4), c.g[A1][0]);
        sw(c, r32(c, V1), c.g[A1][0]);
        sw(c, r32(c, V0), 0u64);
        break 'b_23af0c;
    }
    'b_23b000: {
        c.cycles += 4;
        c.g[RA][0] = sx(0x0023af14);
        c.g[A0][0] = c.g[S0][0];
        if call_ptr(c, 0x0012a250, 0x0023af14).is_err() {
            return;
        }
        let t = c.g[V0][0] == 0u64;
        c.g[V1][0] = lw(c, r32(c, GP).wrapping_sub(0x480));
        if t {
            break 'b_23b000;
        } else {
            'b_23afb4: {
                c.cycles += 4;
                c.g[V0][0] = lw(c, at_2(c, V1, 0x10));
                c.g[V0][0] = sx(at_2(c, V0, 0x2));
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[S1][0] = 0u64;
                if t {
                    break 'b_23afb4;
                } else {
                    c.cycles += 7;
                    c.g[V0][0] = 0x530000;
                    c.mtc1(0, 20);
                    c.g[S6][0] = sx(at_2(c, V0, 0x6640));
                    c.g[S3][0] = sx(at_2(c, S0, 0x28));
                    c.g[S5][0] = 0u64;
                    c.g[S2][0] = sx(at_2(c, S7, 0x4));
                    'l_23af48: loop {
                        'b_23af64: {
                            c.cycles += 6;
                            c.g[A0][0] = lw(c, r32(c, S3));
                            c.lwc1(0, at_2(c, A0, 0x470));
                            c.c_le_s(20, 0);
                            let t = c.fcc();
                            c.g[V0][0] = 0x1;
                            if t {
                                break 'b_23af64;
                            } else {
                                c.cycles += 1;
                                c.g[V0][0] = 0u64;
                                break 'b_23af64;
                            }
                        }
                        'b_23af84: {
                            'b_23af80: {
                                c.cycles += 2;
                                let t = c.g[V0][0] != 0u64;
                                c.g[S0][0] = sx(at_2(c, S7, 0x4));
                                if t {
                                    break 'b_23af80;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[S1][0] == c.g[FP][0];
                                    c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, S5)));
                                    if t {
                                        break 'b_23af84;
                                    } else {
                                        c.cycles += 3;
                                        c.g[RA][0] = sx(0x0023af7c);
                                        if call_ptr(c, 0x00122d78, 0x0023af7c).is_err() {
                                            return;
                                        }
                                        sw(c, r32(c, S2), c.g[V0][0]);
                                        break 'b_23af80;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = sx(r32(c, S0).wrapping_add(r32(c, S5)));
                            break 'b_23af84;
                        }
                        c.cycles += 12;
                        c.g[A0][0] = lw(c, r32(c, GP).wrapping_sub(0x480));
                        c.g[V1][0] = lw(c, r32(c, V0));
                        c.g[S1][0] = sx(at_2(c, S1, 0x1));
                        c.g[S5][0] = sx(at_2(c, S5, 0x4));
                        c.g[S2][0] = sx(at_2(c, S2, 0x4));
                        sw(c, r32(c, S6), c.g[V1][0]);
                        c.g[S3][0] = sx(at_2(c, S3, 0x4));
                        c.g[V0][0] = lw(c, at_2(c, A0, 0x10));
                        c.g[V0][0] = sx(at_2(c, V0, 0x2));
                        c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[S6][0] = sx(at_2(c, S6, 0x4));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x0023af48);
                            }
                            continue 'l_23af48;
                        } else {
                            break 'b_23afb4;
                        }
                    }
                }
            }
            'b_23aff8: {
                c.cycles += 10;
                c.g[A0][0] = lw(c, r32(c, GP).wrapping_sub(0x480));
                c.g[RA][0] = sx(0x0023afc0);
                c.g[A1][0] = 0xa;
                if callee::C00238bf8.call(c, 0x0023afc0).is_err() {
                    return;
                }
                c.g[V0][0] = 0x530000;
                c.g[V0][0] = sx(at_2(c, V0, 0x6730));
                c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
                c.g[V1][0] = lw(c, r32(c, V0));
                c.g[V1][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                let t = c.g[V1][0] != 0u64;
                c.g[V1][0] = lw(c, r32(c, GP).wrapping_sub(0x480));
                if t {
                    break 'b_23aff8;
                } else {
                    c.cycles += 7;
                    c.g[V1][0] = lw(c, r32(c, GP).wrapping_sub(0x848));
                    c.g[A0][0] = lw(c, at_2(c, V1, 0x84));
                    c.g[V0][0] = lw(c, at_2(c, A0, 0xc));
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S4)));
                    c.g[V1][0] = lw(c, at_2(c, V0, 0x28));
                    sw(c, at_2(c, V1, 0x100), 0u64);
                    c.g[V1][0] = lw(c, r32(c, GP).wrapping_sub(0x480));
                    break 'b_23aff8;
                }
            }
            c.cycles += 2;
            c.g[V0][0] = 0x1;
            sw(c, at_2(c, V1, 0x9c), c.g[V0][0]);
            break 'b_23b000;
        }
    }
    c.cycles += 13;
    frame.restore(c, S0, 0x90);
    frame.restore(c, S1, 0x80);
    frame.restore(c, S2, 0x70);
    frame.restore(c, S3, 0x60);
    frame.restore(c, S4, 0x50);
    frame.restore(c, S5, 0x40);
    frame.restore(c, S6, 0x30);
    frame.restore(c, S7, 0x20);
    lq(c, 30, at_2(c, SP, 0x10));
    c.g[RA][0] = ld(c, r32(c, SP));
    c.lwc1(20, at_2(c, SP, 0xa0));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_2(c, SP, 0xb0));
    c.pc = t;
    return;
}


/// `D_004A2C70`: the current GM handler object (`lw(gp - 0x480)`).
pub(crate) const GM_HANDLER: u32 = 0x480;

pub(crate) const D_00535C04: u32 = 0x0053_5c04;


/// `lw(gp - off)`.
#[inline(always)]
pub(crate) fn gpl(c: &mut Cpu, off: u32) -> u64 {
    c.lw((c.g[GP][0] as u32).wrapping_sub(off))
}


/// `void GMHandler8_Reset(void *self)` (0x23ac88): reset the GM handler's
/// screen fields, clear the 5*8-byte table at `self+4`, run its vtable's
/// slot 5 and copy the player's character ids and alt-character ids into the
/// handler; pick opponents when the mode word was 1.
pub fn gmhandler8_reset(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 17, 0x20);
        frame.save(c, 16, 0x30);
        c.g[S1][0] = abi::arg64(c, 0);
        frame.save(c, 18, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = 1;

        // player = cBE_getInterface(cBE_getBE(), 1)
        callee::CBE_GET_BE.call(c, 0x0023_aca8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        callee::CBE_GET_INTERFACE.call(c, 0x0023_acb4)?;
        c.g[S0][0] = c.g[V0][0];

        let gm = gpl(c, GM_HANDLER);
        let this = c.g[S1][0] as u32;
        c.g[V1][0] = 4;
        c.g[V0][0] = gm;
        c.g[A0][0] = 3;
        c.g[A2][0] = 2;
        c.g[A3][0] = u64::MAX;
        c.sw((c.g[V0][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        c.g[A1][0] = 7;
        c.g[V1][0] = gpl(c, 0x848);
        c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[A2][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x74), c.g[A0][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x6c), c.g[A3][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x94), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x90), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x9c), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x98), c.g[S2][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x88), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0);
        c.sw(c.g[V0][0] as u32, c.g[A0][0]);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x70), c.g[A0][0]);
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x78));
        callee::CBE_GET_INTERFACE.call(c, 0x0023_ad0c)?;

        c.g[V1][0] = 0x0053_0000;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4b30));
        let profile_missing = c.g[V0][0] == 0;
        c.g[V0][0] = gpl(c, GM_HANDLER);
        if !profile_missing {
            c.cycles += 10;
            callee::CBE_GET_BE.call(c, 0x0023_ad24)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            callee::CBE_GET_INTERFACE.call(c, 0x0023_ad30)?;
            c.g[V1][0] = 0x0053_0000;
            c.g[A0][0] = gpl(c, GM_HANDLER);
            c.g[V0][0] = c.lw(D_00535C04);
            c.sw((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
            c.g[V0][0] = gpl(c, GM_HANDLER);
        }

        c.cycles += 36;
        c.g[A2][0] = 0x28;
        c.g[A1][0] = 9;
        c.g[A0][0] = sx(this.wrapping_add(4));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x8c), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x14), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x80), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x78), 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x7c), 0);
        callee::MEMSET_2.call(c, 0x0023_ad68)?;

        // self's vtable slot 5: {int16 delta; fn}(self + delta)
        c.g[V1][0] = c.lw(this);
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
        let f = c.g[V0][0] as u32;
        c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
        crate::call::call_ptr(c, f, 0x0023_ad7c)?;

        let player = c.g[S0][0];
        for (alt, ret, slot) in [(0u64, 0x0023_ad88u32, 0x18u32), (1, 0x0023_ad9c, 0x1c)] {
            if alt == 0 {
                c.g[A0][0] = player;
                c.g[A1][0] = 0;
                callee::GET_CHAR_ID.call(c, ret)?;
            } else {
                c.g[A0][0] = player;
                c.g[A1][0] = 1;
                callee::GET_CHAR_ID.call(c, ret)?;
            }
            c.g[V1][0] = gpl(c, GM_HANDLER);
            c.sw((c.g[V1][0] as u32).wrapping_add(slot), c.g[V0][0]);
        }
        for (slot, ret) in [(0x40u32, 0x0023_adb0u32), (0x44, 0x0023_adc4)] {
            let alt = if slot == 0x40 { 0 } else { 1 };
            c.g[A0][0] = player;
            c.g[A1][0] = alt;
            callee::GET_ALT_CHAR.call(c, ret)?;
            c.g[V1][0] = gpl(c, GM_HANDLER);
            c.sw((c.g[V1][0] as u32).wrapping_add(slot), c.g[V0][0]);
        }
        c.g[V1][0] = gpl(c, GM_HANDLER);
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
        let pick = c.g[A0][0] != c.g[S2][0];
        c.sw((c.g[V1][0] as u32).wrapping_add(0x44), c.g[V0][0]);
        if !pick {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x0023_addc);
            c.pc = 0x0023_b0a8;
            ee::s06::GMHandler8_PickOpponents(c);
            if c.pc != 0x0023_addc {
                return Err(crate::call::Unwound);
            }
        }

        c.cycles += 8;
        c.g[V0][0] = gpl(c, GM_HANDLER);
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        c.sw((c.g[V0][0] as u32).wrapping_add(0x84), 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::check;
    use crate::difftest::fill;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::check_lohi;
    use crate::difftest::fakes::covered;
    use crate::difftest::scramble;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::RA as RET;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;
    use crate::difftest::put_words;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::elf_image;
    use crate::difftest::compare;
    pub(crate) use crate::game::bx::bxstring2::tests::W;
    pub(crate) use crate::game::fe::festateoptions::tests::AUX;
    pub(crate) use crate::game::fe::festateoptions::tests::rng;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::put;
    pub(crate) use crate::game::fe::festateoptions::tests::setup;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::gpf;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_4 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_3 as W_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base;
    pub(crate) use crate::game::bx::bxstring2::tests::W_2 as W_3;
    pub(crate) use crate::game::bx::bxstring2::tests::RM;
    pub(crate) use crate::game::bx::bxstring2::tests::RIDERS;
    pub(crate) use crate::game::bx::bxstring2::tests::BE_2;
    pub(crate) use crate::game::bx::bxstring2::tests::DB;
    pub(crate) use crate::game::bx::bxstring2::tests::VTS;
    pub(crate) use crate::game::bx::bxstring2::tests::gp_at;
    pub(crate) use crate::game::bx::bxstring2::tests::base_3 as base_2;
    pub(crate) use crate::game::bx::bxstring2::tests::slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_4 as OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3_2 as OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_6 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_11 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_9 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_9 as BE_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_7 as WORLD_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::RMAN;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDERS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDER;
    pub(crate) use crate::game::fe::feasyncfile::tests::IFACE_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_9 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::check_all;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::LEAF;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::BE_DUMMY;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::WORLD_4 as WORLD_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base_14 as base_4;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::check3;
    pub(crate) use crate::game::fe::fepopup::tests::BEL;
    pub(crate) use crate::game::fe::fepopup::tests::WORLD_4;
    pub(crate) use crate::game::fe::fepopup::tests::AUX as AUX_2;
    pub(crate) use crate::game::fe::fepopup::tests::SYS;
    pub(crate) use crate::game::fe::fepopup::tests::RACE;
    pub(crate) use crate::game::fe::fepopup::tests::RMAN_2;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all_2;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_5;


    pub(crate) const SEEDS: u32 = 200;


    // ------------------------------------------------------------------
    // GM save/load handlers.

    pub(crate) fn gm_setup(c: &mut Cpu, seed: u32, salt: u64, slot: u32) {
        let mut r = machine(c, seed, salt, 1);
        let h = DATA + 0x500;
        let vt = DATA + 0x600;
        c.bus.write32(h, vt);
        c.bus.write16(vt + slot, ((r.next_u32() % 0x40) * 4) as u16);
        c.bus.write32(vt + slot + 4, FAKE);
        c.g[A0][0] = sx(DATA + 0x300);
        c.g[A1][0] = sx(h);
    }


    #[test]
    fn gm_handler0_save_state_matches() {
        check(ssx3_ee::s06::f_00239d18__generated, gm_handler0_save_state, SEEDS, &|c, seed| {
            gm_setup(c, seed, 0x39d18, 0x08);
        });
    }


    #[test]
    fn gm_handler0_load_state_matches() {
        check(ssx3_ee::s06::f_00239d50__generated, gm_handler0_load_state, SEEDS, &|c, seed| {
            gm_setup(c, seed, 0x39d50, 0x10);
        });
    }


    #[test]
    fn gm_handler1_save_state_matches() {
        check(ssx3_ee::s06::f_0023ac18__generated, gm_handler1_save_state, SEEDS, &|c, seed| {
            gm_setup(c, seed, 0x3ac18, 0x08);
        });
    }


    #[test]
    fn gm_handler1_load_state_matches() {
        check(ssx3_ee::s06::f_0023ac50__generated, gm_handler1_load_state, SEEDS, &|c, seed| {
            gm_setup(c, seed, 0x3ac50, 0x10);
        });
    }


    #[test]
    fn gm_handler8_save_state_matches() {
        check(ssx3_ee::s06::f_0023b038__generated, gm_handler8_save_state, SEEDS, &|c, seed| {
            gm_setup(c, seed, 0x3b038, 0x08);
        });
    }


    #[test]
    fn gm_handler8_load_state_matches() {
        check(ssx3_ee::s06::f_0023b070__generated, gm_handler8_load_state, SEEDS, &|c, seed| {
            gm_setup(c, seed, 0x3b070, 0x10);
        });
    }


    pub(crate) fn handler_test(generated: GuestFn, hand: GuestFn, salt: u64, slot: u32) {
        check(generated, hand, SEEDS, &move |c, seed| {
            let mut r = machine(c, seed, salt, 0);
            let obj = DATA + 0x100;
            let vt = vtable(c, &mut r, &[slot]);
            let index = seed % 4;
            c.bus.write32(0x0053_6668 + 4 * index, obj);
            c.bus.write32(obj, vt);
            c.bus.write32(obj + 4, r.next_u32());
            c.g[A0][0] = (DATA + 0x200) as u64;
            c.bus.write32(DATA + 0x204, index);
        });
    }


    #[test]
    fn game_mode_man_handler_v06_matches() {
        handler_test(
            ee::s06::f_00238510__generated,
            game_mode_man_handler_v06,
            0x238510,
            0x30,
        );
    }


    #[test]
    fn game_mode_man_handler_v07_matches() {
        handler_test(
            ee::s06::f_00238550__generated,
            game_mode_man_handler_v07,
            0x238550,
            0x38,
        );
    }


    #[test]
    fn game_mode_man_handler_v09_matches() {
        handler_test(
            ee::s06::f_00238590__generated,
            game_mode_man_handler_v09,
            0x238590,
            0x48,
        );
    }


    /// The world fixture: `gp-0x848 -> world -> +0x84 -> manager -> +0xc ->
    /// race`, `race+8` the race's frame limit.
    pub(crate) fn world(c: &mut Cpu, frame_limit: u32) {
        put(c, AUX + 0x100 + 0x84, AUX + 0x200);
        put(c, AUX + 0x200 + 0xc, AUX + 0x300);
        put(c, AUX + 0x300 + 8, frame_limit);
        gpf(c, -0x848, AUX + 0x100);
    }


    #[test]
    fn gm_handler0_v09_matches() {
        check_lohi(ee::s06::f_00239ce0__generated, gm_handler0_v09, 200, &|c, seed| {
            let mut r = setup(c, seed, 45);
            c.g[A0][0] = AUX as u64;
            c.g[A1][0] = (seed % 4) as u64;
            c.g[A2][0] = (seed % 5) as u64;
            put(c, AUX + 0x600, 3); // the row count
            gpf(c, -0x480, AUX + 0x600);
            for w in 0..(3u32 * 0x28 / 4) {
                put(c, AUX + 4 * w, r.next_u32());
            }
        });
    }


    #[test]
    fn gm_handler9_v06_matches() {
        check_lohi(ee::s06::f_0023a070__generated, gm_handler9_v06, 200, &|c, seed| {
            let mut r = setup(c, seed, 46);
            world(c, r.next_u32() % 400);
            c.g[A0][0] = W as u64;
            c.g[A1][0] = (seed % 2) as u64;
            c.g[A2][0] = ((seed % 5) * 3) as u64;
            c.bus.write16(W + 4, if seed % 3 == 0 { 5 } else { (r.next_u32() % 20) as u16 });
            c.bus.write16(W + 6, (r.next_u32() % 20) as u16);
            gpf(c, -0x480, AUX + 0x600);
            put(c, AUX + 0x600 + 8, if seed % 2 == 0 { 1 } else { 0 });
            put(c, AUX + 0x600 + 0x78, r.next_u32() % 400);
        });
    }


    pub(crate) const SEEDS_2: u32 = 24;


    pub(crate) fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    pub(crate) fn h(c: &mut Cpu, a: u32, v: u16) {
        c.bus.write16(a, v);
    }


    /// The zeroed BE-library singleton at `gp-0x1f04` (else the FE helpers walk
    /// into the lazy disc load).
    pub(crate) fn be_library(c: &mut Cpu) {
        w(c, GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
    }


    /// `{delta, fn}` at `vt + off`.
    pub(crate) fn slot(c: &mut Cpu, vt: u32, off: u32, delta: i16, f: u32) {
        h(c, vt + off, delta as u16);
        w(c, vt + off + 4, f);
    }


    #[test]
    fn game_mode_man_destroy_matches() {
        check(ee::s06::f_002380e8__generated, game_mode_man_destroy, SEEDS_2, &|c, seed| {
            machine(c, seed, 0x2380e8, 10);
            be_library(c);
            for i in 0..10u32 {
                let p = DATA + 0x1000 + 0x100 * i;
                let present = (seed >> i) & 1 != 0;
                w(c, 0x0053_6668 + 4 * i, if present { p } else { 0 });
                if present {
                    let vt = DATA + 0x3000;
                    w(c, p, vt);
                    slot(c, vt, 8, 0x20, FAKE);
                }
            }
            w(c, GAME_GP.wrapping_sub(0x484), if seed % 2 == 0 { 0 } else { DATA + 0x4000 });
        });
    }


    #[test]
    fn gm_handler_clear_standings_matches() {
        check(ee::s06::f_00238d30__generated, gm_handler_clear_standings, SEEDS_2, &|c, seed| {
            machine(c, seed, 0x238d30, 5);
            be_library(c);
            for i in 0..0x40u32 {
                w(c, 0x0053_6640 + 4 * i, seed ^ i);
            }
        });
    }


    #[test]
    fn gm_handler_clear_standings2_matches() {
        check(ee::s06::f_00238da8__generated, gm_handler_clear_standings2, SEEDS_2, &|c, seed| {
            machine(c, seed, 0x238da8, 5);
            be_library(c);
            for i in 0..0x40u32 {
                w(c, 0x0053_6640 + 4 * i, seed ^ i);
            }
        });
    }


    #[test]
    fn game_mode_man_sort_by_score_matches() {
        check(
            ee::s06::f_00238b70__generated,
            game_mode_man_sort_by_score,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x238b70, 2);
                c.g[A1][0] = (1 + seed % 5) as u64; // the entry count handed to qsort
                                                    // The score table lives in RAM at fixed addresses; give it values.
                for i in 0..10u32 {
                    c.bus.write32(0x0053_6708 + 4 * i, (seed * 7 + i) % 13);
                    c.bus.write32(0x0053_6730 + 4 * i, (seed * 5 + i) % 11);
                }
            },
        );
    }


    #[test]
    fn game_mode_man_sort_by_time_matches() {
        check(
            ee::s06::f_00238bf8__generated,
            game_mode_man_sort_by_time,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x238bf8, 2);
                c.g[A1][0] = (1 + seed % 5) as u64; // the entry count handed to qsort
                for i in 0..10u32 {
                    c.bus.write32(0x0053_6708 + 4 * i, (seed * 3 + i) % 17);
                    c.bus.write32(0x0053_6730 + 4 * i, (seed * 11 + i) % 19);
                }
            },
        );
    }


    #[test]
    fn game_rand_shuffle_matches() {
        check_lohi(ee::s06::f_0023c770__generated, game_rand_shuffle, SEEDS, &|c, seed| {
            let _ = base(c, seed, 0x3c770, 2);
            let array = DATA;
            for i in 0..25 {
                w(c, array + 4 * i, (seed + i) as u32);
            }
            c.g[A0][0] = array as u64;
            c.g[A1][0] = 25;
        });
    }


    #[test]
    fn gm_handler_start_matches() {
        check(ee::s06::f_00238c80__generated, gm_handler_start, 40, &|c, seed| {
            let mut r = base_2(c, seed, 0x238c);
            let gm = DATA + 0x2000;
            c.g[A0][0] = sx(DATA);
            c.bus.write32(gp_at(-0x480), gm);
            c.bus.write32(gm, DATA + 0x3000);
            slot_2(c, DATA + 0x3000, 0x28, 1);
            fill(c, &mut r, gm, 0x100);
        });
    }


    #[test]
    fn gm_handler7_v06_matches() {
        check(ee::s06::f_0023c560__generated, gm_handler7_v06, 6, &|c, seed| {
            let _ = base_3(c, seed, 0x0023_c560, 2);
            c.g[A0][0] = OBJ as u64;
            c.bus.write16(OBJ + 8, (seed as u16) & 0x1f);
            c.bus.write16(OBJ + 0xa, (seed as u16) & 7);
            let game = WORLD + 0x1000;
            let race = WORLD + 0x2000;
            w(c, race + 0x28, OBJ2);
            w(c, OBJ2 + 0x790, OBJ3);
            w(c, OBJ3 + 0x198, (seed % 1000) as u32);
            w(c, game + 0x100, 0);
        });
    }


    #[test]
    fn gm_handler4_v06_matches() {
        check_all(ee::s06::f_0023b5f8__generated, gm_handler4_v06, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x23b5, 3);
            game_globals(c);
            be_library_2(c, &mut r);
            c.bus.write32(GAME_GP - 0x514c, (1.0f32 / 60.0).to_bits());
            c.bus.write32(0x0053_0000 + 0x5c12, (seed % 3) as u32);
            let this = DATA;
            c.g[A0][0] = this as u64;
            c.bus.write16(this + 0xa, seed as u16);
            c.bus.write16(this + 8, seed as u16);
            c.bus.write32(RMAN + 0x28, DATA + 0x150000);
            c.bus.write32(DATA + 0x150000 + 0x790, DATA + 0x160000);
        });
    }


    #[test]
    fn gm_handler9_reset_matches() {
        check3(ee::s06::f_00239d88__generated, gm_handler9_reset, 40, &|c, seed| {
            let mut r = base_4(c, seed, 0x239d88);
            // The handler object with a vtable, and the global state block.
            let h = DATA;
            let vt = DATA + 0x100;
            c.bus.write32(h, vt);
            c.bus.write32(h + 4, r.next_u32());
            c.bus.write16(vt + 0x20, 0);
            c.bus.write32(vt + 0x24, LEAF);
            c.bus.write32(GAME_GP - 0x480, DATA + 0x800);
            c.g[A0][0] = sx(h);
        });
    }


    pub(crate) const HANDLER: u32 = DATA + 0x1_0000;

    pub(crate) const RIDERS_3: u32 = DATA + 0x1_1000;


    #[test]
    fn gm_handler8_on_rider_finished_matches() {
        check_all_2(
            ee::s06::f_0023ae00__generated,
            gm_handler8_on_rider_finished,
            20,
            &|c, seed| {
                let _ = base_5(c, seed, 0x23ae_00, 14);
                c.g[A0][0] = sx(HANDLER);
                c.g[A1][0] = sx(seed % 2);
                // The race and its riders come from the world singleton chain:
                // keep the rider list short by leaving the count at_2 1.
                c.bus.write32(RACE + 0xc, RMAN_2);
                c.bus.write32(RMAN_2 + 0x7c, 1);
                for i in 0..2u32 {
                    c.bus.write32(RIDERS_3 + 4 * i, DATA + 0x1_2000 + 0x1000 * i);
                    c.bus.write32(DATA + 0x1_3000 + 4 * i, 0);
                }
            },
        );
    }
}
