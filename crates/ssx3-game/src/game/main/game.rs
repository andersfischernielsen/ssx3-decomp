//! `src/main/game.cpp`: hand-written replacements (moved from the
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
use crate::events::deliver_event;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::computer::delete_if_flag;
pub(crate) use crate::game::ai::computer::DELETE;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_2;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::gpa;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lh_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::ofs;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::dirtysock::tags2::lw as lw_2;
pub(crate) use crate::game::fe::ovstatepause::lw;
pub(crate) use crate::game::movie::movieplayer::add32 as add32_2;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::render::particle::add32;
pub(crate) use crate::game::render::particle::at;
pub(crate) use crate::game::render::particle::gp;
pub(crate) use crate::game::render::particle::ldf;
pub(crate) use crate::game::render::particle::le;
pub(crate) use crate::game::render::particle::p;
pub(crate) use crate::game::render::particle::setf;
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
        0x0022_c410,
        game_module_draw_bezier_patch,
        "GameModule_DrawBezierPatch",
    ),
    (0x0022_cce8, section_man_ctor, "SectionMan_ctor"),
    (0x0022_cd18, section_man_dtor, "SectionMan_dtor"),
    (
        0x0022_d390,
        section_man_is_event_ready,
        "SectionMan_IsEventReady",
    ),
    (
        0x0022_d640,
        section_man_start_event_switch,
        "SectionMan_StartEventSwitch",
    ),
    (
        0x0022_de58,
        section_man_request_sky,
        "SectionMan_RequestSky",
    ),
    (
        0x0022_df50,
        section_man_begin_event_switch,
        "SectionMan_BeginEventSwitch",
    ),
    (
        0x0022_dff0,
        section_man_finish_event_switch,
        "SectionMan_FinishEventSwitch",
    ),
    (
        0x0022_e078,
        section_man_is_section_loaded,
        "SectionMan_IsSectionLoaded",
    ),
    (
        0x0022_e098,
        section_man_get_event_world_section,
        "SectionMan_GetEventWorldSection",
    ),
    (
        0x0022_e0c8,
        section_man_get_world_section_index,
        "SectionMan_GetWorldSectionIndex",
    ),
    (
        0x0022_e360,
        section_man_dequeue_section,
        "SectionMan_DequeueSection",
    ),
];

// ---------------------------------------------------------------------------
// World streamer.

/// Streamer view records: 0x50 bytes each at `wc + 0x250`.
pub(crate) mod view {

    pub const STRIDE: u32 = 0x50;

    pub const EYE: u32 = 0x250;

    /// View box (min, max).
    pub const BOX: u32 = 0x260;

    /// Child-plane points for the two children.
    pub const LEFT_POINT: u32 = 0x260;

    pub const RIGHT_POINT: u32 = 0x270;

    pub const HULL_RANGE: u32 = 0x280;

    pub const HULL_DIST: u32 = 0x284;

    pub const NEAREST: u32 = 0x288;
}

// ---------------------------------------------------------------------------
// Callees.

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
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::AVALANCHE_GET_OBJECT_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_TRACK_INFO;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_MISSION_SET_COLLECTED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_SET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_COUNT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUY_POPUP_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::CLEAR_NEW_REWARDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::DYNAMIC_EMITTER_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_MEM_ALLOC_NAMED_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_THREAD_YIELD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_TIMER_SERVICE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::FE_ASYNC_SET_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_MODULE_REQUEST_STATE_2 as GAME_MODULE_REQUEST_STATE_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_PROCESS_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GROW_LINES;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INIT_FROM_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INSTANCE_GET_NODE_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOAD_INDICATOR_RENDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_API_SEND;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_GET_ROOM_NAME;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_END_OPERATION;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_ON_OPERATION_DONE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MEM_REPORT_STATS_LOCKED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MSG_PRINTF;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::POPUP_CONFIRM_REBUILD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::QSORT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RESET_FOR_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_ADD_BOOST_2 as RIDER_ADD_BOOST;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_APPLY_SCRIPT_TRACK;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GATHER_CAM_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_RECORD_BEST_JUMP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_UPDATE_SCRIPT_SOUNDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SECTION_SELECT_PEAK_LIGHTING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SERIALIZE_READ_U32;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SET_ITEM_YEAR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SNOW_BOX_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_IS_TRACK_GROUP5;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_RESUME_SFX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STREAM_FILE_QUEUE_RANGE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STRLEN as STRLEN_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_HASH as STR_HASH_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_COPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_RAW;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TEX_MAN_INIT_TABLE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_CTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LISTBOX_SET_ITEM_TEXT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST_WRAPPER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_NODE_SENTINEL;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_FIND_BY_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_SCREEN_FIND_OBJECT as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_CTOR_2_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_MGR_PUSH_POPUP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_ASCII;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_EVENT_STAT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_STRING as UI_TEXT_SET_STRING_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_CPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::VRAM_HEAP_INIT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::WORLD_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::APP_SOMETHING;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_BAKE_GROUP;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_EXPORT_HISTORY_FILES;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_FREE_HISTORIES;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_LOAD_HISTORY_FILE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_INSTANCE_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BENEW_PLAYER_INT_IS_MESSAGE_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_INTERFACE_RIDER;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_MESSAGE_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_REWARD_INT_IS_UNLOCKED;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOUNDED_CAMERA_ALGORITHM_BASE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_DESTROY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_SET_PRESENCE_ADDR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE as BUILTIN_DELETE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE as BUILTIN_DELETE_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY_2 as CHAT_ROOM_UPDATE_PLAYER_QUALITY;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_BX_STRING_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_LUNO_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_GET_SAVE_NAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_LIST_DIR_ALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_READ_SAVE;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SETUP_MAN_SEND_QUIT;
    pub(crate) use crate::game::dirtysock::tags2::callee::DIRTY_SOCK_CREATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::DIRTY_SOCK_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEMSET;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE_2 as EA_MEM_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::EMPTY_CHILD_HANDLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::FEMODULE_SET_PREVIEW_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_MODULE_GET_PLAYER_PAD_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_CHANNEL_RELEASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_HEAP_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_UPDATE_CURSOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_API_ADD_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_LIST_GET_AT_3 as LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_LISTS;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_LOBBY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_PEER_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_REQUEST_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_REQUEST_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOC_STRING_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_END_OPERATION_2 as MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_MANAGER_REQUEST_CARD_INFO;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_CARD_ASCII_CHAR_TO_SJIS;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_CARD_SET_USE_SAVE_TITLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT0;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT1;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT2;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT3;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT4;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT5;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_REPORT_POOL_STAT6;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_ITEM_MEASURE_ROW;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_PAGE_SELECT_NEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_GET_INBOX;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_CTOR_3 as NET_FE_STATE_CTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_DTOR_2 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_ON_CHILD_EVENT_3 as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::ON_QUIT_POPUP_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::PATH_PLAT_REFRESH_STATUS;
    pub(crate) use crate::game::dirtysock::tags2::callee::RAND_NEXT_MAIN;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_ADD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_CACHE_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_END_PLAYBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_FREE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RECORD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RESTORE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SNAPSHOT_EMPTY_HOOK_B;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_STREAM_WRITE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REWARDS_FILE_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCE_MC_FORMAT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ANIM_BANK_MAN_FIND_BANK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_CANCEL_BANK_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_FIND_BANK_SLOT_BY_ID;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_IS_BANK_SLOT_LOADED;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_OBJ_TABLE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS_2 as SCRIPT_ENGINE_POLL_ASYNC_LOADS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_FIELD_B0;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_CURRENT_SKIPPABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MAN_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MAN_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_OBJ_CAM_SET_OWNER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_IS_EVENT_LOADED;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_PUSH_FREE_NODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SHARED_LOGIN_MAN_SHUTDOWN;
    pub(crate) use crate::game::dirtysock::tags2::callee::SLOGIN_MC_GET_PORT_SLOT_2 as SLOGIN_MC_GET_PORT_SLOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_FREE_MEMORY_2 as SND_BANK_FREE_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLINK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD_2 as SND_BANK_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SNOW_BOX_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPRINTF_2 as SPRINTF;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_APP_START_FE_MODULE;
    pub(crate) use crate::game::dirtysock::tags2::callee::STATS_SELECT_CATEGORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY as STRCPY_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::STREAM_BUFFER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::TROPHY_ROOM_RELEASE_IMAGES;
    pub(crate) use crate::game::dirtysock::tags2::callee::UIMENU_SET_SELECTION;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_LIST_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_3;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_ON_ACTIVATE as UI_STATE_ON_ACTIVATE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_TEXT_GET_FONT;
    pub(crate) use crate::game::dirtysock::tags2::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_CLOSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_MAN_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::WORLD_VIEW_SET;
    pub(crate) use crate::game::fe::ovstatepause::callee::AI_CREATE_COMPUTER_RIDERS;
    pub(crate) use crate::game::fe::ovstatepause::callee::AI_DESTROY_MISSION_RIDERS;
    pub(crate) use crate::game::fe::ovstatepause::callee::ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY;
    pub(crate) use crate::game::fe::ovstatepause::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::ovstatepause::callee::BE_REWARD_INT_COUNT_SET_BITS;
    pub(crate) use crate::game::fe::ovstatepause::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHECK_ACCOUNT_NAME;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHECK_PASSWORD;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_OV_STATE_OVERLAY_SHUTDOWN;
    pub(crate) use crate::game::fe::ovstatepause::callee::EA_MEM_SET;
    pub(crate) use crate::game::fe::ovstatepause::callee::EVENT_SELECT_POST_INPUT;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_ASYNC_MANAGER_CANCEL;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_FORMAT_MONEY;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_MODULE_RIDER_PREVIEW_UNLOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_MODULE_RIDER_PREVIEW_UPDATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::KEYBOARD_POPUP_IS_KEY_ENABLED;
    pub(crate) use crate::game::fe::ovstatepause::callee::KEYBOARD_POPUP_MOVE_DOWN;
    pub(crate) use crate::game::fe::ovstatepause::callee::MC_MANAGER_BEGIN_LOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::fe::ovstatepause::callee::MEM_CARD_STATE_BASE_CTOR_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::MOUNTAIN_ROOM_POLL_IMAGE_LOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::MPEG2_FLUSH_BUFFER;
    pub(crate) use crate::game::fe::ovstatepause::callee::MPEG2_SHOW_BITS;
    pub(crate) use crate::game::fe::ovstatepause::callee::MUSIC_PLAYER_RESUME;
    pub(crate) use crate::game::fe::ovstatepause::callee::NET_FE_STATE_CTOR_2 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::ovstatepause::callee::OV_SCREEN_SESSION_DESTROY_INDICATORS;
    pub(crate) use crate::game::fe::ovstatepause::callee::PLAY_FE_SOUND;
    pub(crate) use crate::game::fe::ovstatepause::callee::RIDER_METRIX_CLEAR_PEAK_A4;
    pub(crate) use crate::game::fe::ovstatepause::callee::SECTION_MAN_REQUEST_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::ovstatepause::callee::STRCPY;
    pub(crate) use crate::game::fe::ovstatepause::callee::STRLEN;
    pub(crate) use crate::game::fe::ovstatepause::callee::STR_HASH;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_CTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_MGR_CLOSE_TOP_STATE_AND_POPUPS;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_ON_ACTIVATE_2 as UI_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_POST_INPUT;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_RENDER;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_UPDATE_2 as UI_STATE_UPDATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXTURE_SET_FIND;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::fe::ovstatepause::callee::VID_ENGINE_PLAYER_RENDER;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_1;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::BEREWARD_INT_IS_PEAK_COMPLETE;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_SCORE_INT_UPDATE_BEST_STATS;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_DISCONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_IS_CONNECTED;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_RECONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::BUILTIN_DELETE_4;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_ALLOC_COPY;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_CTOR_COPY;
    pub(crate) use crate::game::movie::movieplayer::callee::BX_STRING_DTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::CBE_GET_BE;
    pub(crate) use crate::game::movie::movieplayer::callee::CBE_GET_INTERFACE;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FE_STATE_CHAR_EQUIP_SET_BUY_ROW;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FE_STATE_CHAR_EQUIP_SET_EQUIP_ROW;
    pub(crate) use crate::game::movie::movieplayer::callee::C_FLAG_MANAGER_GROUP_INIT_KEY;
    pub(crate) use crate::game::movie::movieplayer::callee::C_OV_STATE_OVERLAY_INIT;
    pub(crate) use crate::game::movie::movieplayer::callee::EVENT_SELECT_INIT;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_APPLY_BACKGROUND_SCALE;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_APPLY_CONFIG;
    pub(crate) use crate::game::movie::movieplayer::callee::FE_POPUP_CONFIRM_RESET_LAYOUT;
    pub(crate) use crate::game::movie::movieplayer::callee::FILE_LOAD;
    pub(crate) use crate::game::movie::movieplayer::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES;
    pub(crate) use crate::game::movie::movieplayer::callee::FONT_PARSE_GLYPHS;
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_OVERLAY_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_STATE as GAME_MODULE_REQUEST_STATE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::GET_HASH_VALUE_32;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MAN_DISCONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_CREATE_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::LOBBY_MENU_ON_JOIN_ROOM_RESULT;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_ON_OPERATION_DONE;
    pub(crate) use crate::game::movie::movieplayer::callee::MC_MANAGER_SET_MODE_2 as MC_MANAGER_SET_MODE;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_ALLOC_NAMED;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_FREE;
    pub(crate) use crate::game::movie::movieplayer::callee::MEM_FREE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_ITEM_ON_FOCUS;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_PAGE_ON_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::MENU_SYS_PAGE_SET_SELECTION;
    pub(crate) use crate::game::movie::movieplayer::callee::MOD_OBJ_SET_TRANSFORM_MODIFIER;
    pub(crate) use crate::game::movie::movieplayer::callee::MULTI_SPLINE_MODIFIER_ADD_REF;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_SET_QUALITY_ICONS;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_CTOR_5 as NET_FE_STATE_CTOR_3;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_HANDLE_NET_EVENT;
    pub(crate) use crate::game::movie::movieplayer::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::movie::movieplayer::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PDA_TEMPLATE_OPEN;
    pub(crate) use crate::game::movie::movieplayer::callee::OV_SCREEN_PROFILE_SAVE_PDA_INIT_SLOT_LABELS;
    pub(crate) use crate::game::movie::movieplayer::callee::PARENT_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_ON_SPLIT_MARKER;
    pub(crate) use crate::game::movie::movieplayer::callee::RIDER_METRIX_RESET_STATS;
    pub(crate) use crate::game::movie::movieplayer::callee::ROLLER_MODIFIER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_CMD_QUEUE_CLEAR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_CMD_QUEUE_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_FIND_SOUND_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_LAUNCH_PENDING_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_RUN_PROCESS;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_ENGINE_UNLOAD_BANK;
    pub(crate) use crate::game::movie::movieplayer::callee::SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::callee::SPRINTF_2;
    pub(crate) use crate::game::movie::movieplayer::callee::STRCPY_2 as STRCPY_3;
    pub(crate) use crate::game::movie::movieplayer::callee::STREAM_BUFFER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::TAG_FIELD_GET_STRING;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_LOAD_TEXTURES;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_STORE_WORD;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_CTOR_2 as UI_STATE_CTOR_4;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_MGR_ADD_BACKGROUND_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESSES;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESS_COUNT;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_MAN_CREATE_PROCESS;
}

/// `void SectionMan_StartEventSwitch(SectionMan *man, uint event)` (0x22d640):
/// if `event` is non-zero and no switch is already pending (`+0x1b4 == 0`),
/// record it at `+0x1b8` and set the pending word at `+0x1b4` to 1. `v0` is 1
/// on the non-zero path, untouched otherwise.
pub fn section_man_start_event_switch(c: &mut Cpu) {
    const PENDING: u32 = 0x1b4;
    const EVENT: u32 = 0x1b8;
    let man = Addr(abi::arg(c, 0));
    let event = c.g[A1][0];
    c.cycles += 2;
    if event != 0 {
        c.cycles += 3;
        let pending = man.off(PENDING).lw(c);
        c.g[V0][0] = 1;
        if pending == 0 {
            c.cycles += 2;
            man.off(EVENT).sw(c, event);
            man.off(PENDING).sw(c, c.g[V0][0]);
        }
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `bool SectionMan_IsSectionLoaded(int index)` (0x22e078): true when the
/// load-state word of section `index` (record base 0x442168 + index*16, field
/// `+8`) is 1 or 2.
pub fn section_man_is_section_loaded(c: &mut Cpu) {
    const SECTIONS: u32 = 0x44_2168;
    const STATE: u32 = 0x8;
    c.cycles += 8;
    let idx = abi::arg(c, 1);
    c.g[A1][0] = sx(idx << 4);
    let base = SECTIONS.wrapping_add(idx << 4);
    c.g[V1][0] = sx(base);
    let state = Addr(base).off(STATE).lw(c);
    c.g[V0][0] = (sx((state as u32).wrapping_sub(1)) < 2) as u64;
    abi::ret(c);
}

/// `int SectionMan_GetWorldSectionIndex(int index)` (0x22e0c8): the world
/// section word of record `index` (record base 0x442168 + index*16, `+4`).
pub fn section_man_get_world_section_index(c: &mut Cpu) {
    const SECTIONS: u32 = 0x44_2168;
    const WORLD: u32 = 0x4;
    c.cycles += 6;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 4);
    let base = SECTIONS.wrapping_add(c.g[A1][0] as u32);
    c.g[V1][0] = sx(base);
    c.g[V0][0] = Addr(base).off(WORLD).lw(c);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Bezier patches.

/// `GameModule_DrawBezierPatch(Patch* patch, View* view, void* arg /*a2*/)`
/// (0x22c410): tessellation level per patch edge from the squared
/// distances between the patch centre (+0x10) and the four corner points
/// held in the view object (+0x170..0x19c): `k * (d_a + d_b) + bias` for
/// the edges (0,1), (2,3), (0,2), (1,3), clamped to [min, max] (patch
/// +0x20..0x2c) and mapped through a byte table (gp-0x6e0); then the
/// renderer's virtual 0xa8 draws the patch with the four levels and two
/// halfwords of the view (+0x1a0/+0x1a2).
pub fn game_module_draw_bezier_patch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 75;
        let frame = Frame::open(c, 0x50);
        c.g[T4][0] = c.g[A1][0];
        frame.save_fpr(c, 23, 0x48);
        c.g[T5][0] = c.g[A0][0];
        frame.save_fpr(c, 20, 0x30);
        c.g[T0][0] = c.g[A2][0];
        frame.save_ra(c, 0x20);
        frame.save_fpr(c, 22, 0x40);
        frame.save_fpr(c, 21, 0x38);
        let patch = r32(c, T5);
        let view = r32(c, T4);
        let lod_min = ldf(c, 21, p(patch, 0x28));
        // The patch centre (+0x10) against the view's four points.
        let cx = ldf(c, 8, p(patch, 0x10));
        let cy = ldf(c, 2, p(patch, 0x14));
        setf(c, 23, lod_min);
        let ax = ldf(c, 0, p(view, 0x170));
        let ay = ldf(c, 1, p(view, 0x174));
        let dx = cx - ax;
        let cz = ldf(c, 4, p(patch, 0x18));
        let dy = cy - ay;
        let az = ldf(c, 0, p(view, 0x178));
        let bx = ldf(c, 1, p(view, 0x17c));
        let by = ldf(c, 3, p(view, 0x180));
        let dz = cz - az;
        let d0x = dx * dx;
        let bz = ldf(c, 0, p(view, 0x184));
        let d0y = dy * dy;
        let ex = ldf(c, 9, p(view, 0x188));
        let dx = cx - bx;
        let ey = ldf(c, 10, p(view, 0x18c));
        let dy = cy - by;
        let ez = ldf(c, 11, p(view, 0x190));
        let d0z = dz * dz;
        let gx = ldf(c, 14, p(view, 0x194));
        let dz = cz - bz;
        let gy = ldf(c, 12, p(view, 0x198));
        let d1y = dy * dy;
        let gz = ldf(c, 15, p(view, 0x19c));
        let d1x = dx * dx;
        let k = ldf(c, 6, p(patch, 0x20));
        let dx = cx - ex;
        let bias = ldf(c, 7, p(patch, 0x24));
        let dy = cy - ey;
        let lod_max = ldf(c, 22, p(patch, 0x2c));
        let d1z = dz * dz;
        setf(c, 13, d1z);
        let dz = cz - ez;
        let d2x = dx * dx;
        let d2y = dy * dy;
        setf(c, 9, d2y);
        let dx = cx - gx;
        setf(c, 19, dx);
        let dy = cy - gy;
        setf(c, 18, dy);
        let d1 = d1x + d1y;
        let d2z = dz * dz;
        setf(c, 8, d2z);
        let d0 = d0x + d0y;
        setf(c, 16, d0y);
        setf(c, 17, d0z);
        let d3y = dy * dy;
        let dz = cz - gz;
        setf(c, 20, dz);
        let d3x = dx * dx;
        let d0 = d0 + d0z;
        let d1 = d1 + d1z;
        let d2 = d2x + d2y;
        let d3 = d3x + d3y;
        let d3z = dz * dz;
        setf(c, 4, d3z);
        let s01 = d0 + d1;
        let d2 = d2 + d2z;
        let d3 = d3 + d3z;
        let s01 = k * s01;
        let s02 = d0 + d2;
        setf(c, 5, s02);
        let s13 = d1 + d3;
        let s23 = d2 + d3;
        let e01 = bias + s01;
        let s13 = k * s13;
        setf(c, 3, s13);
        let s23 = k * s23;
        let s02k = k * s02;
        setf(c, 6, s02k);
        let in01 = le(c, lod_min, e01);
        let e13 = bias + s13;
        let e23 = bias + s23;
        let e02 = bias + s02k;
        setf(c, 10, e01);
        setf(c, 1, e13);
        setf(c, 0, e23);
        setf(c, 7, e02);
        setf(c, 2, d3);
        // Clamp each edge level to [min, max].
        if in01 {
            c.cycles += 1;
            setf(c, 23, e01.min(lod_max));
        }
        c.cycles += 4;
        let in23 = le(c, lod_min, e23);
        c.mov_s(10, 23);
        setf(c, 6, lod_min);
        if in23 {
            c.cycles += 1;
            setf(c, 6, e23.min(lod_max));
        }
        c.cycles += 4;
        let in02 = le(c, lod_min, e02);
        c.mov_s(0, 6);
        setf(c, 6, lod_min);
        if in02 {
            c.cycles += 1;
            setf(c, 6, e02.min(lod_max));
        }
        c.cycles += 4;
        let in13 = le(c, lod_min, e13);
        c.mov_s(7, 6);
        setf(c, 2, lod_min);
        if in13 {
            c.cycles += 1;
            setf(c, 2, e13.min(lod_max));
        }
        // Levels through the byte table; call the renderer.
        c.cycles += 41;
        c.cvt_w_s(1, 0);
        c.mfc1(A1, 1);
        c.g[V1][0] = sx(gp(c, -0x6e0));
        c.cvt_w_s(1, 10);
        c.mfc1(A0, 1);
        c.g[T3][0] = c.lh(p(view, 0x1a0));
        c.cvt_w_s(0, 7);
        c.mfc1(A2, 0);
        c.g[T2][0] = c.lh(p(view, 0x1a2));
        c.cvt_w_s(0, 2);
        c.mfc1(V0, 0);
        let table = c.g[V1][0];
        c.g[A1][0] = add32(c.g[A1][0], table);
        c.g[A0][0] = add32(c.g[A0][0], table);
        let sp = sp(c);
        let v = c.g[T3][0];
        c.sw(p(sp, 0x10), v);
        c.g[A3][0] = c.lbu(r32(c, A0));
        c.g[A2][0] = add32(c.g[A2][0], table);
        c.g[V0][0] = add32(c.g[V0][0], table);
        c.g[A0][0] = c.lbu(r32(c, A1));
        c.g[V1][0] = c.lbu(r32(c, V0));
        c.g[A1][0] = c.g[T4][0];
        c.g[T1][0] = c.lbu(r32(c, A2));
        let v = c.g[A3][0];
        c.sb(sp, v);
        c.g[A2][0] = c.g[SP][0];
        let v = c.g[A0][0];
        c.sb(p(sp, 1), v);
        c.g[A3][0] = sx(p(sp, 0x10));
        let v = c.g[V1][0];
        c.sb(p(sp, 3), v);
        let v = c.g[T2][0];
        c.sw(p(sp, 0x14), v);
        let v = c.g[T1][0];
        c.sb(p(sp, 2), v);
        c.g[T1][0] = c.lw(patch);
        c.g[V0][0] = c.lw(at(c, T1, 4));
        let vt = r32(c, V0);
        c.g[A0][0] = c.lh(p(vt, 0xa8));
        c.g[V1][0] = c.lw(p(vt, 0xac));
        c.g[A0][0] = add32(c.g[T1][0], c.g[A0][0]);
        let target = r32(c, V1);
        call_ptr(c, target, 0x0022_c604)?;
        frame.restore_ra(c, 0x20);
        frame.restore_fpr(c, 23, 0x48);
        frame.restore_fpr(c, 22, 0x40);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void SectionMan_dtor(SectionMan* m, int flags)` (0x22cd18).
pub fn section_man_dtor(c: &mut Cpu) {
    guest(c, |c| delete_if_flag(c, 0x0022_cd30))
}

// ---------------------------------------------------------------------------
// Game and section managers.

/// `SectionMan_ctor(p, game)` 0x22cce8: remember the game object at +0x1b0 and
/// initialise the section tables; returns `p`.
pub fn section_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg64(c, 0);
        c.g[S0][0] = this;
        let game = c.g[A1][0];
        c.sw((this as u32).wrapping_add(0x1b0), game);
        callee::SECTION_MAN_INIT.call(c, 0x0022_cd00)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Section manager.

/// `int SectionMan_GetEventWorldSection(SectionMan* man, int event)`
/// (0x22e098): the world section id of an event, through the two global
/// tables at 0x442498 (event records, 0x28 bytes each: a section record
/// index at `+0x10`) and 0x442168 (section records, 16 bytes each, id at
/// `+4`). The `mult` the compiler left in clobbers `a1` with its own product
/// (`mult a1, v0` with `rd = a1`), so the table index is `event * 0x28`.
pub fn section_man_get_event_world_section(c: &mut Cpu) {
    c.cycles += 12;
    c.g[V0][0] = 0x28;
    c.g[V1][0] = 0x440000;
    c.mult(0, A1, A1, V0);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x2488));
    c.g[A0][0] = 0x440000;
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x2168));
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
    c.g[V0][0] = c.lw(p(r32(c, V1), 0x10));
    c.g[V0][0] = sx(r32(c, V0) << 4);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
    c.g[V0][0] = c.lw(p(r32(c, A0), 4));
    abi::ret(c);
}

/// `void SectionMan_RequestSky(SectionMan* m, int v)` (0x22de58): switch the
/// requested sky when it changes, activate the section and mark it
/// "requested".
pub fn section_man_request_sky(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = crate::abi::r32(c, S0);
        c.g[V0][0] = lw(c, this, 0x1ac);
        if c.g[V0][0] != c.g[V1][0] {
            c.cycles += 5;
            Addr(p(this, 0x1ac)).sw(c, c.g[V1][0]);
            callee::SECTION_MAN_REQUEST_ACTIVATE.call(c, 0x0022_de80)?;
            c.g[V0][0] = 1;
            Addr(p(crate::abi::r32(c, S0), 0x1a8)).sw(c, c.g[V0][0]);
        }
        c.cycles += 3;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Section manager, replay playback and the memory-card manager.

/// `bool SectionMan_IsEventReady(SectionMan* self)` (0x22d390): true when the
/// event is loaded and its ready flag (+0x1c0) is set.
pub fn section_man_is_event_ready(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = 0;
        callee::SECTION_MAN_IS_EVENT_LOADED.call(c, 0x0022_d3ac)?;
        let not_loaded = c.g[V0][0] == 0;
        c.g[V0][0] = c.g[S1][0];
        if !not_loaded {
            c.cycles += 3;
            c.g[V0][0] = c.lw(at(c, S0, 0x1c0));
            c.g[S1][0] = (0u64 < c.g[V0][0]) as u64;
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void SectionMan_DequeueSection(self, node)` (0x22e360): unlink a section
/// node from its list or record it as the new head, push it free and count it.
pub fn section_man_dequeue_section(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        c.g[A1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        if c.g[V1][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(r32(c, A1)).off(4).lw(c);
            Addr(p(r32(c, S0), 0x19c)).sw(c, c.g[V0][0]);
        } else {
            c.cycles += 2;
            c.g[V0][0] = Addr(r32(c, A1)).off(4).lw(c);
            Addr(p(r32(c, V1), 4)).sw(c, c.g[V0][0]);
        }
        c.cycles += 9;
        c.g[RA][0] = sx(0x0022_e398);
        c.g[A0][0] = c.g[S0][0];
        callee::SECTION_MAN_PUSH_FREE_NODE.call(c, 0x0022_e398)?;
        let this = r32(c, S0);
        c.g[V0][0] = Addr(this).off(0x1a0).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
        Addr(p(this, 0x1a0)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SectionMan_BeginEventSwitch(self)` (0x0022df50): request the section
/// state and overlay, then tell the interface to begin the switch.
pub fn section_man_begin_event_switch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A3][0] = 0x1;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = 0xb;
        c.g[A0][0] = Addr(at(c, S0, 0x1b0)).lw(c);
        callee::GAME_MODULE_REQUEST_STATE_2.call(c, 0x0022_df74)?;
        callee::CBE_GET_BE.call(c, 0x0022_df7c)?;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        callee::CBE_GET_INTERFACE.call(c, 0x0022_df88)?;
        c.g[A1][0] = Addr(at(c, S0, 0x1bc)).lw(c);
        c.g[V1][0] = sx(0x530000);
        c.g[A0][0] = 0x1;
        Addr(at(c, V1, 0x5c08)).sw(c, c.g[A1][0]);
        Addr(gp(c, -0x1f38)).sw(c, c.g[A0][0]);
        c.g[V1][0] = Addr(at(c, V0, 0xc)).lw(c);
        c.g[A0][0] = Addr(at(c, V1, 0x8)).lh(c);
        c.g[A1][0] = Addr(at(c, V1, 0xc)).lw(c);
        c.g[A0][0] = add32_2(word(c, V0), word(c, A0));
        call_ptr(c, word(c, A1), 0x0022_dfb0)?;
        c.g[A0][0] = Addr(at(c, S0, 0x1b0)).lw(c);
        c.g[A1][0] = 0xa;
        callee::GAME_MODULE_REQUEST_OVERLAY_STATE.call(c, 0x0022_dfbc)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void SectionMan_FinishEventSwitch(SectionMan* self)` (0x22dff0):
/// reselect peak lighting for the track, drop the pending event switch and
/// request the game state change.
pub fn section_man_finish_event_switch(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        open(c, 0x20);
        let sp0 = spv(c);
        c.sq(16, at_2(sp0, 0x10));
        c.sd(sp0, c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::BE_LIBRARY_GET_INSTANCE, 0x0022_e004)?;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x0022_e010)?;
        c.g[A0][0] = c.g[V0][0];
        callee::BE_GET_TRACK_INFO.call(c, 0x0022_e018)?;
        c.g[A1][0] = lw_at(c, c.g[V0][0], 0);
        let this = c.g[S0][0];
        c.g[A0][0] = c.g[S0][0];
        callee::SECTION_SELECT_PEAK_LIGHTING.call(c, 0x0022_e024)?;
        c.g[V0][0] = lw_at(c, this, 0x1cc);
        let none = c.g[V0][0] == 0;
        c.g[A1][0] = 3;
        if !none {
            c.cycles += 9;
            c.sw(at_2(this as u32, 0x1cc), 0);
            let g = gpa(c, -0x848);
            c.g[V0][0] = lw_at(c, g as u64, 0);
            c.g[V1][0] = lw_at(c, c.g[V0][0], 0x84);
            c.g[A2][0] = lw_at(c, c.g[V1][0], 0xc);
            c.g[V0][0] = lw_at(c, c.g[A2][0], 0xcc);
            c.g[A0][0] = lh_at(c, c.g[V0][0], 0x28);
            c.g[V1][0] = lw_at(c, c.g[V0][0], 0x2c);
            let func = c.g[V1][0] as u32;
            c.g[A0][0] = sx(at_2(c.g[A2][0] as u32, c.g[A0][0] as u32));
            call_ptr(c, func, 0x0022_e054)?;
        }
        c.cycles += 9;
        c.g[A0][0] = lw_at(c, this, 0x1b0);
        c.g[A1][0] = 0xa;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        call_ptr(c, callee::GAME_MODULE_REQUEST_STATE_3, 0x0022_e068)?;
        c.lq(16, at_2(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at_2(sp0, 0x20));
        c.pc = ra;
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::check_paths;
    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
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
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::RA as RET;
    use crate::difftest::SP as TEST_SP;
    pub(crate) use crate::game::bx::bxstring2::tests::be_library;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::world_2 as world;
    pub(crate) use crate::game::dirtysock::tags2::tests::be_library_2;
    pub(crate) use crate::game::dirtysock::tags2::tests::be_library_6 as be_library_3;
    pub(crate) use crate::game::dirtysock::tags2::tests::clear;
    pub(crate) use crate::game::dirtysock::tags2::tests::SEEDS_7 as SEEDS_2;
    pub(crate) use crate::game::fe::ovstatepause::tests::globals_2;
    pub(crate) use crate::game::fe::ovstatepause::tests::WORLD;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3 as base;
    pub(crate) use crate::game::movie::movieplayer::tests::globals;
    pub(crate) use crate::game::movie::movieplayer::tests::w;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_2 as BE;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_LIBRARY;
    pub(crate) use crate::game::movie::movieplayer::tests::W;
    pub(crate) use crate::game::render::particle::tests::f;
    pub(crate) use crate::game::render::particle::tests::gp_at;
    pub(crate) use crate::game::render::particle::tests::put_vec3;
    pub(crate) use crate::game::render::particle::tests::uf;
    pub(crate) use crate::game::render::particle::tests::SEEDS;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn section_man_start_event_switch_matches() {
        check_paths(
            ssx3_ee::s05::f_0022d640,
            section_man_start_event_switch,
            300,
            &[4, 7, 9],
            &|c, seed| {
                let mut r = Rng(0x0c11_0002 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let event = if seed % 3 == 0 {
                    0
                } else {
                    ((r.next_u32() as u64) << 32) | r.next_u32() as u64
                };
                c.g[A1][0] = event;
                c.bus
                    .write32(DATA + 0x1b4, if seed % 2 == 0 { 0 } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn section_man_is_section_loaded_matches() {
        check(
            ssx3_ee::s05::f_0022e078,
            section_man_is_section_loaded,
            200,
            &|c, seed| {
                let mut r = Rng(0x0c11_0003 ^ seed as u64);
                scramble(c, &mut r);
                let idx = (seed % 16) as u32;
                c.g[A1][0] = idx as u64;
                let base = 0x44_2168u32.wrapping_add(idx << 4);
                let state = match seed % 6 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => 0xffff_ffff,
                    _ => r.next_u32(),
                };
                c.bus.write32(base + 0x8, state);
            },
        );
    }

    #[test]
    fn section_man_get_world_section_index_matches() {
        check(
            ssx3_ee::s05::f_0022e0c8,
            section_man_get_world_section_index,
            100,
            &|c, seed| {
                let mut r = Rng(0x0c11_0004 ^ seed as u64);
                scramble(c, &mut r);
                let idx = (seed % 16) as u32;
                c.g[A1][0] = idx as u64;
                let base = 0x44_2168u32.wrapping_add(idx << 4);
                c.bus.write32(base + 0x4, r.next_u32());
            },
        );
    }

    #[test]
    fn game_module_draw_bezier_patch_matches() {
        check(
            ee::s05::f_0022c410__generated,
            game_module_draw_bezier_patch,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x22c4, 1);
                let (patch, view, obj, vt) = (DATA, DATA + 0x200, DATA + 0x400, DATA + 0x500);
                put_vec3(c, &mut r, patch + 0x10, -100.0, 100.0);
                for k in 0..12 {
                    c.bus
                        .write32(view + 0x170 + 4 * k, f(uf(&mut r, -100.0, 100.0)));
                }
                c.bus.write32(view + 0x1a0, r.next_u32());
                c.bus.write32(patch + 0x20, f(uf(&mut r, 0.0, 0.0005)));
                c.bus.write32(patch + 0x24, f(uf(&mut r, -2.0, 8.0)));
                let lo = uf(&mut r, 0.0, 2.0);
                c.bus.write32(patch + 0x28, f(lo));
                c.bus.write32(patch + 0x2c, f(lo + uf(&mut r, 0.0, 6.0)));
                fill(c, &mut r, gp_at(-0x6e0), 4);
                c.bus.write32(patch, obj);
                c.bus.write32(obj + 4, vt);
                c.bus.write16(vt + 0xa8, (r.next_u32() & 0x7c) as u16);
                c.bus.write32(vt + 0xac, FAKE);
                c.g[A0][0] = sx(patch);
                c.g[A1][0] = sx(view);
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn section_man_dtor_matches() {
        crate::difftest::check(
            ee::s05::f_0022cd18__generated,
            section_man_dtor,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x22cd_18, 2);
                be_library(c);
                if seed % 2 == 1 {
                    c.g[A0][0] = 0;
                    c.g[A1][0] = 1;
                } else {
                    c.g[A0][0] = sx(DATA + (r.next_u32() & 0xfff0));
                    c.g[A1][0] = sx(r.next_u32() & !1);
                }
            },
        );
    }

    #[test]
    fn section_man_ctor_matches() {
        check(
            ssx3_ee::s05::f_0022cce8__generated,
            section_man_ctor,
            20,
            &|c, seed| {
                let _r = machine(c, seed, 0xb1, 0);
                globals(c);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x3_0000 + (seed % 4) * 0x1000);
            },
        );
    }

    #[test]
    fn section_man_get_event_world_section_matches() {
        check(
            ee::s05::f_0022e098__generated,
            section_man_get_event_world_section,
            60,
            &|c, seed| {
                machine(c, seed, 0x22e098, 0);
                c.g[A0][0] = DATA as u64;
                // `event` is a byte offset into the event records; the function's
                // `mult` scales it by 0x28 to reach the record's section index.
                let event = (seed % 8) * 0x28;
                c.g[A1][0] = event as u64;
                let section = seed % 16;
                c.bus.write32(0x0044_2488 + 0x10 + event * 0x28, section);
                c.bus.write32(0x0044_216c + section * 16, 0x1000 + seed);
            },
        );
    }

    #[test]
    fn section_man_request_sky_matches() {
        check(
            ssx3_ee::s05::f_0022de58__generated,
            section_man_request_sky,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x32, 1);
                globals_2(c);
                c.g[A0][0] = sx(DATA);
                let v = (seed % 4) as u64;
                c.g[A1][0] = v;
                c.bus
                    .write32(DATA + 0x1ac, if seed % 3 == 0 { v as u32 } else { 0 });
            },
        );
    }

    #[test]
    fn section_man_is_event_ready_matches() {
        check(
            ssx3_ee::s05::f_0022d390__generated,
            section_man_is_event_ready,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x22d390, 0);
                be_library_2(c);
                let this_ = DATA + 0x100;
                c.bus
                    .write32(this_ + 0x1c0, if seed % 2 == 0 { 0 } else { 1 });
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = 0;
            },
        );
    }

    #[test]
    fn section_man_dequeue_section_matches() {
        check(
            ee::s05::f_0022e360__generated,
            section_man_dequeue_section,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x22e360, 0);
                let this = DATA + 0x100;
                clear(c, this, 0x200);
                let node = DATA + 0x2000;
                let next = DATA + 0x2100;
                clear(c, node, 0x40);
                clear(c, next, 0x40);
                c.bus.write32(next + 4, r.next_u32());
                c.bus.write32(this + 0x1a0, r.next_u32() % 100);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { sx(node) };
                c.g[A2][0] = sx(next);
                let _ = seed;
            },
        );
    }

    #[test]
    fn section_man_begin_event_switch_matches() {
        check(
            ee::s05::f_0022df50__generated,
            section_man_begin_event_switch,
            12,
            &|c, seed| {
                let mut r = base(c, seed, 0x22df_50, 4);
                let self_ = DATA;
                let module = DATA + 0x1000;
                w(c, self_ + 0x1b0, module);
                w(c, self_ + 0x1bc, r.next_u32());
                c.g[A0][0] = self_ as u64;
            },
        );
    }

    #[test]
    fn section_man_finish_event_switch_matches() {
        check(
            ee::s05::f_0022dff0__generated,
            section_man_finish_event_switch,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x22dff0, 4);
                be_library_3(c, &mut r);
                c.bus.write32(LOG_RESULTS, 0x11);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0x1cc, (seed % 2) as u32);
                c.bus.write32(DATA + 0x1b0, 0);
                // A minimal world -> race manager -> object with a `+0xcc` slot.
                let w = DATA + 0x1000;
                world(c, w);
                c.bus.write32(w + 0x84, DATA + 0x1100);
                c.bus.write32(DATA + 0x1100 + 0xc, DATA + 0x1200);
                c.bus.write32(DATA + 0x1200 + 0xcc, DATA + 0x1300);
                c.bus.write16(DATA + 0x1300 + 0x28, 0);
                c.bus.write32(DATA + 0x1300 + 0x2c, FAKE);
                // The track info's first word (a track id), and the lighting table
                // entry so the dispatcher takes a real arm.
                c.bus.write32(0x0043_d950, 0);
                c.bus.write32(0x0047_b4e0, 0x0022_e1ac);
            },
        );
    }
}
