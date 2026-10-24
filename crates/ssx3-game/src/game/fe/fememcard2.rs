//! `src/fe/fememcard2.cpp`: hand-written replacements (moved from the
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
use crate::events::deliver_event;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_3;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at as lw_at_2;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::dirtysock::tags2::vslot;
pub(crate) use crate::game::fe::feasyncfile::at_2 as at;
pub(crate) use crate::game::fe::feasyncfile::gpr;
pub(crate) use crate::game::fe::feasyncfile::lh_3 as lh_2;
pub(crate) use crate::game::fe::feasyncfile::lw_8 as lw_3;
pub(crate) use crate::game::fe::feasyncfile::lw_9 as lw_4;
pub(crate) use crate::game::fe::feasyncfile::p_7 as p_4;
pub(crate) use crate::game::fe::feasyncfile::sw_5 as sw_2;
pub(crate) use crate::game::fe::feasyncfile::sw_7 as sw_3;
pub(crate) use crate::game::fe::feasyncfile::STR_HASH;
pub(crate) use crate::game::fe::fepopup3::gp_2 as gp;
pub(crate) use crate::game::fe::fepopup3::lh;
pub(crate) use crate::game::fe::fepopup3::lw_2;
pub(crate) use crate::game::fe::fepopup3::p_2;
pub(crate) use crate::game::fe::fepopup3::sw;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::p_3;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festateoptions::gpa;
pub(crate) use crate::game::fe::ovstatepause::lw;
pub(crate) use crate::game::fe::ovstatepause::p;
pub(crate) use crate::game::movie::movieplayer::at as at_2;
pub(crate) use crate::game::scripter::ssxscriptengine::gp as gp_2;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::sw_at;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use callee::*;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001d_72a8,
        memcard_ui_on_load_aborted,
        "MemCardUI_OnLoadAborted",
    ),
    (
        0x001d_8160,
        mem_card_ui_try_next_slot,
        "MemCardUI_TryNextSlot",
    ),
    (
        0x001d_8358,
        mem_card_ui_show_file_display,
        "MemCardUI_ShowFileDisplay",
    ),
    (0x001d_8670, mem_card_state_base_v46, "MemCardStateBase_v46"),
    (0x001d_86b0, mem_card_state_base_v45, "MemCardStateBase_v45"),
    (
        0x001d_8be0,
        mem_card_state_base_reset_state,
        "MemCardStateBase_ResetState",
    ),
    (
        0x001d_8c50,
        memcard_state_base_handle_input,
        "MemCardStateBase_HandleInput",
    ),
    (
        0x001d_8d68,
        mem_card_state_base_show_status,
        "MemCardStateBase_ShowStatus",
    ),
    (
        0x001d_9258,
        memcard_state_base_sync_menu_selection,
        "MemCardStateBase_SyncMenuSelection",
    ),
    (
        0x001d_9308,
        mem_card_state_base_set_use_save_title,
        "MemCardStateBase_SetUseSaveTitle",
    ),
    (
        0x001d_9368,
        mem_card_state_base_set_mode_menu,
        "MemCardStateBase_SetModeMenu",
    ),
    (
        0x001d_93d8,
        mem_card_state_base_transition_out,
        "MemCardStateBase_TransitionOut",
    ),
    (
        0x001d_93e8,
        mem_card_state_base_play_widget_anim,
        "MemCardStateBase_PlayWidgetAnim",
    ),
    (
        0x001d_97e0,
        mem_card_state_base_step_slot_dir,
        "MemCardStateBase_StepSlotDir",
    ),
    (
        0x001d_9818,
        mem_card_state_base_on_activate,
        "MemCardStateBase_OnActivate",
    ),
    (
        0x001d_9880,
        memcard_state_base_is_card_ready,
        "MemCardStateBase_IsCardReady",
    ),
    (
        0x001d_9948,
        mem_card_state_base_alloc_read_buffer,
        "MemCardStateBase_AllocReadBuffer",
    ),
    (0x001d_99c8, cfe_popup_confirm_ctor, "cFEPopupConfirm_ctor"),
    (0x001d_9ca8, c_fe_popup_confirm_v27, "cFEPopupConfirm_v27"),
    (0x001d_9d18, c_fe_popup_confirm_v25, "cFEPopupConfirm_v25"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
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
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_THREAD_YIELD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_TIMER_SERVICE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::FE_ASYNC_SET_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_MODULE_REQUEST_STATE_2;
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
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STRLEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_HASH;
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
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_CPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::VRAM_HEAP_INIT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::WORLD_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_BAKE_GROUP;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_EXPORT_HISTORY_FILES;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_FREE_HISTORIES;
    pub(crate) use crate::game::dirtysock::tags2::callee::AVALANCHE_LOAD_HISTORY_FILE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_INSTANCE_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_REWARD_INT_IS_UNLOCKED;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_DESTROY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUDDY_MAN_SET_PRESENCE_ADDR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY_2 as CHAT_ROOM_UPDATE_PLAYER_QUALITY;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_BX_STRING_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_LUNO_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_GET_SAVE_NAME;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_LIST_DIR_ALL;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_REQUEST_READ_SAVE;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_PS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SETUP_MAN_SEND_QUIT;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEMSET;
    pub(crate) use crate::game::dirtysock::tags2::callee::EMPTY_CHILD_HANDLER;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_MODULE_GET_PLAYER_PAD_MASK;
    pub(crate) use crate::game::dirtysock::tags2::callee::KEYBOARD_POPUP_UPDATE_CURSOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_LIST_GET_AT_3 as LOBBY_LIST_GET_AT;
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
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_ITEM_MEASURE_ROW;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_PAGE_SELECT_NEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::MESSAGE_CENTER_GET_INBOX;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_CTOR_3 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_DTOR_2 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_ON_CHILD_EVENT_3 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE_2 as NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::ON_QUIT_POPUP_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::PATH_PLAT_REFRESH_STATUS;
    pub(crate) use crate::game::dirtysock::tags2::callee::RAND_NEXT_MAIN;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_ADD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RECORD_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCE_MC_FORMAT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_OBJ_TABLE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_OBJ_CAM_SET_OWNER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_PUSH_FREE_NODE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SLOGIN_MC_GET_PORT_SLOT_2 as SLOGIN_MC_GET_PORT_SLOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_FREE_MEMORY_2 as SND_BANK_FREE_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLINK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD_2 as SND_BANK_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SNOW_BOX_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::SPRINTF_2 as SPRINTF;
    pub(crate) use crate::game::dirtysock::tags2::callee::STATS_SELECT_CATEGORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY;
    pub(crate) use crate::game::dirtysock::tags2::callee::STREAM_BUFFER_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::TROPHY_ROOM_RELEASE_IMAGES;
    pub(crate) use crate::game::dirtysock::tags2::callee::UIMENU_SET_SELECTION;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_3 as UI_STATE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_TEXT_GET_FONT;
    pub(crate) use crate::game::dirtysock::tags2::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_1;
    pub(crate) use crate::game::movie::movieplayer::callee::ASYNC_FILE_RELEASE_2;
    pub(crate) use crate::game::movie::movieplayer::callee::BEREWARD_INT_IS_PEAK_COMPLETE;
    pub(crate) use crate::game::movie::movieplayer::callee::BE_SCORE_INT_UPDATE_BEST_STATS;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_DISCONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_IS_CONNECTED;
    pub(crate) use crate::game::movie::movieplayer::callee::BUDDY_MAN_RECONNECT;
    pub(crate) use crate::game::movie::movieplayer::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_2;
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
    pub(crate) use crate::game::movie::movieplayer::callee::GAME_MODULE_REQUEST_STATE;
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
    pub(crate) use crate::game::movie::movieplayer::callee::STRCPY_2;
    pub(crate) use crate::game::movie::movieplayer::callee::STREAM_BUFFER_CTOR;
    pub(crate) use crate::game::movie::movieplayer::callee::TAG_FIELD_GET_STRING;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_LOAD_TEXTURES;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_ENGINE_STORE_WORD;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_CTOR_2;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_STATE_MGR_ADD_BACKGROUND_STATE;
    pub(crate) use crate::game::movie::movieplayer::callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESSES;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_CACHE_GET_PROCESS_COUNT;
    pub(crate) use crate::game::movie::movieplayer::callee::WSCRIPT_MAN_CREATE_PROCESS;

    pub const MEM_CARD_STATE_BASE_SET_USE_SAVE_TITLE: Callee = Callee {
        addr: 0x001d_9308,
        f: ee::s04::f_001d9308,
    };
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod addr {
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BANK_MONITOR_ADD;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_BE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_INTERFACE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_INTERFACE_9;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::CHASE_CAMERA_RESET_FOR_RIDER;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::CLOSE_UNPINNED_POPUPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::FE_NET_SEND;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::HUD_SHOW_NUMBER_DELAYED;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MC_STATUS_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MEMSET;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MEM_MAN_ALLOC;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_DTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_POP_PAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_PUSH_PAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::POSITION_MODIFIER_CTOR_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::RIDER_PREVIEW;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::RIDER_PREVIEW_BOLTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::SPATIAL_INSERT;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::SPRINTF;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCASECMP;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCMP;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRLEN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STR_HASH;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_ENGINE_CREATE_SCREEN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_ENGINE_TEARDOWN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_OBJECT_BY_HASH;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_SCREEN_START_THREAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::WORLD_GET;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::WORLD_PAINTER_2D;
}

/// `cFEPopupConfirm_...` (v24) 0x1d9d18: store `a1` at +0x168 and, when the
/// popup has a live object at +0x40, apply its layout (`cFEStateNetHelpPopup`'s
/// sibling hook).
pub fn c_fe_popup_confirm_v25(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        let value = c.g[A1][0];
        c.sw(this.wrapping_add(0x168), value);
        let obj = c.lw(this.wrapping_add(0x40));
        c.g[V0][0] = obj;
        let idle = obj == 0;
        frame.restore_ra(c, 0);
        if !idle {
            c.cycles += 3;
            call_ptr(c, 0x001c_5dd8, 0x001d_9d38)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `int MemCardStateBase_StepSlotDir(MemCardStateBase* self, void* list, int idx,
/// int back)` (0x1d97e0): step `idx` by ±1 and report whether it is still in
/// the list (1) or out of range (-1).
pub fn mem_card_state_base_step_slot_dir(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 1;
    c.g[V1][0] = u64::MAX;
    if c.g[A3][0] != 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    let negative = (c.g[A2][0] as i64) < 0;
    c.g[V0][0] = u64::MAX;
    if negative {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 5;
    c.g[V1][0] = Addr(r32(c, A1).wrapping_add(0x424)).lw(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
    let too_big = c.g[V1][0] != 0;
    c.g[V0][0] = 1;
    if too_big {
        c.cycles += 1;
        c.g[V0][0] = u64::MAX;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void MemCardStateBase_SetModeMenu(State* s)` (0x1d9368): enter mode 1
/// once (`+0x214`), calling the state's own `v` slot (+0x118) with 1.
pub fn mem_card_state_base_set_mode_menu(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        let this = crate::abi::r32(c, A0);
        c.g[A2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = 1;
        c.g[V0][0] = lw(c, this, 0x214);
        let already = c.g[V0][0] == c.g[V1][0];
        c.g[A1][0] = 1;
        if !already {
            c.cycles += 6;
            Addr(p(this, 0x214)).sw(c, c.g[V1][0]);
            c.g[V0][0] = lw(c, this, 8);
            let vt = c.g[V0][0];
            let m = VSlot::read(c, vt, 0x118);
            c.g[A0][0] = m.delta;
            c.g[V1][0] = m.func;
            c.g[A0][0] = m.this(c.g[A2][0]);
            crate::call::call_ptr(c, m.func as u32, 0x001d_939c)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MemCardUI_ShowFileDisplay(self)` (0x1d8358): move the front end into
/// the file-display state (mode 2) unless it is already there or a save is in
/// progress, and tell its vtable slot 0x118/0x11c.
pub fn mem_card_ui_show_file_display(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x10c8)).lw(c);
        frame.save_ra(c, 0);
        c.g[V1][0] = 0x2;
        let fe = c.g[A2][0] as u32;
        c.g[V0][0] = Addr(fe).off(0x214).lw(c);
        let in_mode = c.g[V0][0] == c.g[V1][0];
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        if !in_mode {
            c.cycles += 3;
            c.g[V0][0] = Addr(fe).off(0x1c0).lw(c);
            let busy = c.g[V0][0] != 0;
            c.g[A1][0] = 0x2;
            if !busy {
                c.cycles += 7;
                Addr(fe).off(0x214).sw(c, c.g[V1][0]);
                c.g[V0][0] = Addr(fe).off(8).lw(c);
                let vt = c.g[V0][0] as u32;
                c.g[A0][0] = Addr(vt).off(0x118).lh(c);
                c.g[V1][0] = Addr(vt).off(0x11c).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x001d_8398);
                c.g[A0][0] = sx(fe.wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x001d_8398)?;
                c.g[RA][0] = c.ld(c.g[SP][0] as u32);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void MemCardStateBase_v45(self)` (0x1d86b0): enable the save title, set
/// the "check cards" mode (5) and call the state's vtable slot 0x178/0x17c.
pub fn mem_card_state_base_v45(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        MEM_CARD_STATE_BASE_SET_USE_SAVE_TITLE.call(c, 0x001d_86c8)?;
        c.g[V0][0] = 0x1;
        c.g[V1][0] = 0x5;
        let this = c.g[S0][0] as u32;
        Addr(this).off(0x1a8).sw(c, c.g[V0][0]);
        c.g[V0][0] = Addr(this).off(8).lw(c);
        Addr(this).off(0x1c0).sw(c, c.g[V1][0]);
        let vt = c.g[V0][0] as u32;
        c.g[A0][0] = Addr(vt).off(0x178).lh(c);
        c.g[V1][0] = Addr(vt).off(0x17c).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x001d_86ec);
        c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001d_86ec)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MemCardStateBase_ResetState(self)` (0x1d8be0): clear the save-title
/// and dialog fields and set the default use-save-title mode.
pub fn mem_card_state_base_reset_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 0;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = r32(c, S0);
        Addr(p(this, 0x1a8)).sw(c, 0);
        Addr(p(this, 0x1f0)).sw(c, 0);
        Addr(p(this, 0x1ec)).sw(c, 0);
        Addr(p(this, 0x1b0)).sw(c, 0);
        c.g[RA][0] = sx(0x001d_8c10);
        Addr(p(this, 0x1e4)).sw(c, c.g[V0][0]);
        callee::MEM_CARD_SET_USE_SAVE_TITLE.call(c, 0x001d_8c10)?;
        c.g[V0][0] = 2;
        Addr(p(this, 0x1dc)).sw(c, 0);
        Addr(p(this, 0x1c0)).sw(c, c.g[V0][0]);
        Addr(p(this, 0x1e0)).sw(c, 0);
        Addr(p(this, 0x1e8)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[inline(always)]
pub(crate) fn sh(c: &mut Cpu, a: u32, v: u64) {
    Addr(a).sh(c, v);
}

/// `void MemCardStateBase_SetUseSaveTitle(MemCardStateBase* self, int on)`
/// (0x1d9308): store the flag and, when on, let the overlay manager's current
/// movie player know that the state now owns the title area.
pub fn mem_card_state_base_set_use_save_title(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let off = c.g[A1][0] == 0;
        sw(c, p_2(c.g[S0][0], 0x1f4), c.g[A1][0]);
        if !off {
            c.cycles += 12;
            c.g[A0][0] = lw_2(c, gp(c, -0x848));
            call_ptr(c, 0x0022_7f80, 0x001d_9328)?;
            c.g[A3][0] = lw_2(c, p_2(c.g[V0][0], 0x434));
            c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x9c));
            c.g[A1][0] = lw_2(c, p_2(c.g[V0][0], 0xf8));
            c.g[V0][0] = lw_2(c, p_2(c.g[A3][0], 0));
            c.g[A0][0] = lh(c, p_2(c.g[V0][0], 0x70));
            c.g[V1][0] = lw_2(c, p_2(c.g[V0][0], 0x74));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001d_9348)?;
        } else {
            c.cycles += 2;
            sh(c, p_2(c.g[S0][0], 0x9c), 0);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MemCardStateBase_OnActivate(MemCardStateBase* self)` (0x1d9818):
/// activate the UI screen, then show the "busy" widget found by its name hash.
pub fn mem_card_state_base_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e4c0, 0x001d_982c)?;
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x78b0));
        call_ptr(c, 0x0031_7670, 0x001d_9838)?;
        c.g[A0][0] = lw_2(c, p_2(c.g[S0][0], 0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001d_9844)?;
        c.g[A2][0] = c.g[V0][0];
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = lw_2(c, p_2(c.g[A2][0], 8));
            c.g[A0][0] = lh(c, p_2(c.g[V0][0], 0x48));
            c.g[V1][0] = lw_2(c, p_2(c.g[V0][0], 0x4c));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001d_9864)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Memory-card and front-end overlay states.

/// `void MemCardStateBase_v46(void*)` (0x1d8670): restart the front-end's
/// state machine if it is in state 4, then clear the two progress words of
/// the memory-card screen (the FE singleton at `gp-0x10c8`).
pub fn mem_card_state_base_v46(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A0][0] = Addr(gpa(c, -0x10c8)).lw(c);
    c.g[V1][0] = 0x4;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1c0));
    let not4 = c.g[V0][0] != c.g[V1][0];
    c.g[V0][0] = Addr(gpa(c, -0x10c8)).lw(c);
    if !not4 {
        c.cycles += 3;
        c.g[V0][0] = 0x1;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x1c0), c.g[V0][0]);
        c.g[V0][0] = Addr(gpa(c, -0x10c8)).lw(c);
    }
    c.cycles += 3;
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1e0), 0);
    c.sw((c.g[V0][0] as u32).wrapping_add(0x1a8), 0);
    abi::ret(c);
}

/// `int MemCardStateBase_TransitionOut(MemCardStateBase* state)` (0x1d93d8):
/// `state->+0x1c0 == 6`.
pub fn mem_card_state_base_transition_out(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x1c0)).lw(c);
    c.g[V0][0] ^= 0x6;
    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// The ports.

/// `void MemCardUI_OnLoadAborted(MemCardUI* self)` (0x1d72a8): abort the
/// front-end's memory-card load: notify the FE v-table entry 41, reset the
/// card manager mode and tear down its object's v-table entry 1.
pub fn memcard_ui_on_load_aborted(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = Addr(gpr(c, -0x10c8)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(at(c, A1, 8)).lw(c);
        c.g[A0][0] = Addr(at(c, V0, 0x148)).lh(c);
        c.g[V1][0] = Addr(at(c, V0, 0x14c)).lw(c);
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx(at(c, A1, 0).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x001d_72cc)?;

        c.g[V0][0] = Addr(gpr(c, -0x10c8)).lw(c);
        c.g[A0][0] = Addr(gpr(c, -0x848)).lw(c);
        Addr(at(c, V0, 0x1e4)).sw(c, 0);
        call_ptr(c, 0x0022_7f80, 0x001d_72dc)?;
        c.g[S0][0] = c.g[V0][0];

        c.g[A1][0] = 0x24;
        c.g[V0][0] = Addr(at(c, S0, 0x748)).lw(c);
        c.g[A0][0] = Addr(at(c, V0, 8)).lh(c);
        c.g[V1][0] = Addr(at(c, V0, 0xc)).lw(c);
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx(at(c, S0, 0).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x001d_72f8)?;

        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0023_d570, 0x001d_7304)?;

        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEPopupConfirm_v27(cFEPopupConfirm* self, const u16* src)`
/// (0x1d9ca8): set the popup's text from a source UTF-16 string.
pub fn c_fe_popup_confirm_v27(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        sqsp(c, 17, 0x20);
        sqsp(c, 18, 0x10);
        c.g[S1][0] = c.g[A1][0];
        sqsp(c, 16, 0x30);
        c.g[S2][0] = c.g[A0][0];
        let no_src = c.g[S1][0] == 0;
        frame.save_ra(c, 0);
        if !no_src {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            call::call_ptr(c, 0x002c_24d0, 0x001d_9cd0)?;
            let empty = c.g[V0][0] == 0;
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x360));
            if !empty {
                c.cycles += 8;
                c.g[A1][0] = c.g[S1][0];
                c.g[A0][0] = c.g[S0][0];
                call::call_ptr(c, 0x002c_2508, 0x001d_9ce4)?;
                c.g[V1][0] = 1;
                c.g[V0][0] = Addr(p_3(r32(c, S2), 0x40)).lw(c);
                Addr(p_3(r32(c, S2), 0xc4)).sw(c, c.g[S0][0]);
                let no_refresh = c.g[V0][0] == 0;
                Addr(p_3(r32(c, S2), 0xc0)).sw(c, c.g[V1][0]);
                if !no_refresh {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S2][0];
                    call::call_ptr(c, 0x001c_5dd8, 0x001d_9d00)?;
                }
            }
        }
        c.cycles += 6;
        lqsp(c, 16, 0x30);
        lqsp(c, 17, 0x20);
        lqsp(c, 18, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = ra;
        Ok(())
    });
}

/// `MemCardStateBase_ShowStatus` (0x1D8D68): show the memory card status text and
/// hand it to the state's UI object.
pub fn mem_card_state_base_show_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x660);
        frame.save(c, S0, 0x650);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x640);
        c.g[A0][0] = Addr(gp_2(c, -0x848)).lw(c);
        call_ptr(c, addr::WORLD_GET, 0x001d_8d80)?;
        c.g[V1][0] = 0x16;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[SP][0];
        sw_at(c, S0, 0x19c, c.g[V1][0]);
        c.g[A2][0] = 0x320;
        call_ptr(c, addr::MC_STATUS_TEXT, 0x001d_8d98)?;
        c.g[V0][0] = lw_at(c, S0, 0x8);
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 1;
        c.g[A3][0] = 1;
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x120)).lh(c);
        c.g[T0][0] = 0;
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x124)).lw(c);
        c.g[T1][0] = 0;
        c.g[A0][0] = abi::addu(c.g[S0][0], c.g[A0][0]);
        let show = r32(c, V1);
        c.g[T2][0] = 1;
        call_ptr(c, show, 0x001d_8dc4)?;
        c.g[V0][0] = 1;
        c.g[RA][0] = Addr(frame.at(c, 0x640)).ld(c);
        sw_at(c, S0, 0x210, c.g[V0][0]);
        frame.restore(c, S0, 0x650);
        let ret = c.g[RA][0] as u32;
        c.g[SP][0] = sx(r32(c, SP).wrapping_add(0x660));
        c.pc = ret;
        Ok(())
    })
}

/// `void MemCardStateBase_AllocReadBuffer(self, size)` (0x001d9948):
/// (re)allocate the memcard state's read buffer.
pub fn mem_card_state_base_alloc_read_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(at_2(c, S0, 0x228)).lw(c);
        let fresh = c.g[V1][0] != c.g[V0][0];
        c.g[S1][0] = c.g[A1][0];
        if fresh {
            c.cycles += 15;
        } else {
            c.cycles += 3;
            c.g[A0][0] = Addr(at_2(c, S0, 0x1d4)).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                callee::MEM_FREE_2.call(c, 0x001d_9980)?;
            }
            Addr(at_2(c, S0, 0x1d4)).sw(c, 0);
            c.cycles += 1;
            Addr(at_2(c, S0, 0x228)).sw(c, 0);
            c.cycles += 15;
        }
        c.g[A1][0] = sx(0x460000);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = sx(0x460000u32.wrapping_add(0x7990));
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        callee::OPERATOR_NEW_NAMED.call(c, 0x001d_99a0)?;
        c.g[V1][0] = 1;
        Addr(at_2(c, S0, 0x1d4)).sw(c, c.g[V0][0]);
        Addr(at_2(c, S0, 0x228)).sw(c, c.g[V1][0]);
        Addr(at_2(c, S0, 0x1d0)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void MemCardStateBase_PlayWidgetAnim(MemCardStateBase* self, char* name,
/// char* anim)` (0x1d93e8): find the widget named `name` and play the named
/// animation on it (animations come from the anim set at_3
/// `self->screen->something + 0x50`).
pub fn mem_card_state_base_play_widget_anim(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        open(c, 0x40);
        let sp0 = spv(c);
        c.sq(16, at_3(sp0, 0x30));
        c.sq(18, at_3(sp0, 0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sq(17, at_3(sp0, 0x20));
        c.g[A0][0] = c.g[A1][0];
        c.sd(sp0, c.g[RA][0]);
        c.g[S2][0] = c.g[A2][0];
        call_ptr(c, callee::STR_HASH, 0x001d_940c)?;
        let this = c.g[S0][0];
        c.g[A0][0] = lw_at_2(c, this, 0x40);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, callee::UI_SCREEN_FIND_OBJECT_2, 0x001d_9418)?;
        c.g[S1][0] = c.g[V0][0];
        let found = c.g[S1][0] != 0;
        c.g[A0][0] = c.g[S2][0];
        if found {
            c.cycles += 8;
            c.g[S0][0] = lw_at_2(c, this, 0x10);
            c.g[S0][0] = sx(at_3(c.g[S0][0] as u32, 0x50));
            call_ptr(c, callee::STR_HASH, 0x001d_9430)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, callee::UI_ANIM_SET_CREATE_ANIM, 0x001d_943c)?;
            let anim = c.g[V0][0];
            let none = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S1][0];
            if !none {
                c.cycles += 5;
                c.g[A1][0] = anim;
                c.g[A2][0] = 3;
                c.g[A3][0] = 0;
                c.g[T0][0] = 1;
                call_ptr(c, callee::UI_OBJECT_SET_ANIM, 0x001d_9458)?;
            }
        }
        c.cycles += 6;
        c.lq(16, at_3(sp0, 0x30));
        c.lq(17, at_3(sp0, 0x20));
        c.lq(18, at_3(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at_3(sp0, 0x40));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `MemCardStateBase_SyncMenuSelection` (0x1d9258): look up the memory-card
// menu by name hash, choose the selected item (first selectable when there is
// no current item or it is disabled), apply it, and remember its index.
pub fn memcard_state_base_sync_menu_selection(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[A0][0] = 0x4a0000;
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1600));
        call_ptr(c, 0x00317670, 0x001d_927c)?;
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039d860, 0x001d_9288)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xa0));
            let no_item = c.g[V0][0] == 0;
            c.g[S1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x95));
            let mut find_first = no_item;
            if !no_item {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x14));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
                c.g[V0][0] = c.g[V0][0] & 0x1;
                find_first = c.g[V0][0] == 0;
                c.g[V0][0] = 0xff;
            }
            if find_first {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0039b7b0, 0x001d_92c0)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[V0][0] = 0xff;
            }
            c.cycles += 2;
            let current = c.g[S1][0] == c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            if !current {
                c.cycles += 4;
                c.g[A1][0] = c.g[S1][0] & 0xff;
                call_ptr(c, 0x0039b960, 0x001d_92d8)?;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x1c4), c.g[S1][0]);
            } else {
                c.cycles += 3;
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0039b960, 0x001d_92e8)?;
                c.sw((c.g[S2][0] as u32).wrapping_add(0x1c4), 0u64);
            }
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x30));
        } else {
            c.lq(16, frame.at(c, 0x30));
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cFEPopupConfirm_ctor(self, engine, owner, text, a4..a8)` (0x1d99c8):
/// construct a confirmation popup, choosing the button count from the global
/// popup mode.
pub fn cfe_popup_confirm_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        c.g[S1][0] = c.g[A3][0];
        c.g[S2][0] = c.g[T0][0];
        c.g[S3][0] = c.g[T1][0];
        c.g[S4][0] = c.g[T2][0];
        c.g[S5][0] = c.g[T3][0];
        call_ptr(c, 0x001c_58e8, 0x001d_9a04)?;
        c.g[V1][0] = 0x470000;
        c.g[V0][0] = Addr(p(r32(c, GP), 0xffff_ef4c)).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3770));
        let already = c.g[V0][0] != 0;
        c.sw(p(c.g[S0][0] as u32, 8), c.g[V1][0]);
        if already {
            c.cycles += 4;
            c.g[V0][0] = 0x2;
            c.g[V1][0] = 0x4;
            c.sw(p(c.g[S0][0] as u32, 0x17c), c.g[V0][0]);
            c.sw(p(c.g[S0][0] as u32, 0x14c), c.g[V1][0]);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x1;
            c.sw(p(c.g[S0][0] as u32, 0x17c), c.g[V0][0]);
        }
        c.cycles += 18;
        c.g[T2][0] = Addr(p(c.g[SP][0] as u32, 0x70)).lw(c);
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A3][0] = c.g[S3][0];
        c.g[T0][0] = c.g[S4][0];
        c.g[T1][0] = c.g[S5][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001d_9a80, 0x001d_9a54)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x50);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Memory card / load screens.

/// `bool MemCardStateBase_IsCardReady(MemCardStateBase *self)` (0x1d9880):
/// the card is ready when the state is neither "none" (0) nor checking (6),
/// the menu entry is set, the dialog is idle and the port check passes.
pub fn memcard_state_base_is_card_ready(c: &mut Cpu) {
    guest(c, |c| {
        'b_1d9930: {
            'b_1d992c: {
                c.cycles += 12;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S0][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                c.g[RA][0] = sx(0x001d98a4);
                c.g[S2][0] = 0u64;
                call_ptr(c, 0x00227f80, 0x001d98a4)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
                let t = c.g[V1][0] == 0u64;
                c.g[S1][0] = c.g[V0][0];
                if t {
                    break 'b_1d992c;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0x6;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = c.g[S2][0];
                    if t {
                        break 'b_1d9930;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x340));
                        let t = c.g[V0][0] == 0u64;
                        c.g[V0][0] = c.g[S2][0];
                        if t {
                            break 'b_1d9930;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe0));
                            let t = c.g[V0][0] != 0u64;
                            c.g[V0][0] = c.g[S2][0];
                            if t {
                                break 'b_1d9930;
                            } else {
                                c.cycles += 8;
                                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
                                c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                                vslot(c, c.g[V1][0], 0x58, V0);
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x001d98ec);
                                c.g[A0][0] =
                                    sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                call_ptr(c, t, 0x001d98ec)?;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = c.g[S2][0];
                                if t {
                                    break 'b_1d9930;
                                } else {
                                    c.cycles += 4;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x130));
                                    c.g[V0][0] = c.g[V1][0] ^ 0x6;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.g[V1][0] ^ 0x1;
                                    if t {
                                        break 'b_1d992c;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = c.g[S2][0];
                                        if t {
                                            break 'b_1d9930;
                                        } else {
                                            c.cycles += 8;
                                            c.g[A1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
                                            c.g[V1][0] = c.lw((c.g[A1][0] as u32));
                                            vslot(c, c.g[V1][0], 0x258, V0);
                                            let t = c.g[V0][0] as u32;
                                            c.g[RA][0] = sx(0x001d9924);
                                            c.g[A0][0] = sx((c.g[A1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                            call_ptr(c, t, 0x001d9924)?;
                                            c.g[V1][0] = 0x1;
                                            if c.g[V0][0] != 0 {
                                                c.g[S2][0] = c.g[V1][0];
                                            }
                                            break 'b_1d992c;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S2][0];
            break 'b_1d9930;
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// MemCardUI_TryNextSlot (0x00001d8160).

/// `MemCardUI_TryNextSlot`: advance the memory-card screen to the next usable slot.
pub fn mem_card_ui_try_next_slot(c: &mut Cpu) {
    guest(c, |c| {
        'b_1d8234: {
            'b_1d8210: {
                'b_1d8208: {
                    'b_1d81f0: {
                        'b_1d81a8: {
                            c.cycles += 13;
                            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                            call_ptr(c, 0x00227f80, 0x001d8178)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x428));
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x424));
                            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            if c.g[V0][0] == 0u64 {
                                c.g[A0][0] = c.g[S0][0];
                                break 'b_1d81a8;
                            } else {
                                c.cycles += 5;
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x0023fb18, 0x001d819c)?;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
                                let t = true;
                                c.g[A1][0] = 0x31;
                                if t {
                                    break 'b_1d81f0;
                                } else {
                                    break 'b_1d81a8;
                                }
                            }
                        }
                        c.cycles += 11;
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x0023fb18, 0x001d81b0)?;
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
                        c.g[A1][0] = 0u64;
                        c.g[V1][0] = c.lw((c.g[A2][0] as u32));
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1a0));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1a4));
                        let t = c.g[V0][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        c.g[RA][0] = sx(0x001d81cc);
                        c.pc = t;
                        ssx3_ee_rt::funcs::call_indirect(c, t);
                        if c.pc != 0x001d81cc {
                            return Err(crate::call::Unwound);
                        }
                        let t = c.g[V0][0] != 0u64;

                        if t {
                            break 'b_1d8210;
                        } else {
                            c.cycles += 4;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x10c8));
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x208));
                            let t = c.g[V0][0] == 0u64;
                            c.g[V1][0] = 0x6;
                            if t {
                                break 'b_1d8208;
                            } else {
                                c.cycles += 3;
                                c.g[A1][0] = 0x2;
                                c.sw((c.g[A0][0] as u32).wrapping_add(0x1c0), c.g[V1][0]);
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
                                break 'b_1d81f0;
                            }
                        }
                    }
                    c.cycles += 6;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let t = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    c.g[RA][0] = sx(0x001d8200);
                    c.pc = t;
                    ssx3_ee_rt::funcs::call_indirect(c, t);
                    if c.pc != 0x001d8200 {
                        return Err(crate::call::Unwound);
                    }
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        break 'b_1d8234;
                    } else {
                        break 'b_1d8208;
                    }
                }
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00241400, 0x001d8210)?;
                break 'b_1d8210;
            }
            c.cycles += 9;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0023fae0, 0x001d8218)?;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x748));
            c.g[A1][0] = 0x36;
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            c.g[RA][0] = sx(0x001d8230);
            c.pc = t;
            ssx3_ee_rt::funcs::call_indirect(c, t);
            if c.pc != 0x001d8230 {
                return Err(crate::call::Unwound);
            }
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_1d8234;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
        Ok(())
    });
}

/// `MemCardUI_OnNoCards(self)` (0x001d8240): the "no memory cards" reply
/// arrived; mark the screen as busy, ask the front end to show the "no cards"
/// panel and, when the object exists, relabel it with the localized text.
pub fn memcard_ui_on_no_cards(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save_ra(c, 0);
        let fe = Addr(gp(c, -0x10c8)).lw(c);
        c.g[A2][0] = fe;
        c.g[V1][0] = 1;
        sw_3(c, fe, 0x1c0, 1);
        c.g[V0][0] = lw_4(c, fe, 0x214);
        let already = c.g[V0][0] == c.g[V1][0];
        c.g[A1][0] = 1;
        if !already {
            c.cycles += 6;
            sw_3(c, fe, 0x214, c.g[V1][0]);
            c.g[V0][0] = lw_4(c, fe, 8);
            let delta = lh_2(c, c.g[V0][0], 0x118);
            let func = lw_4(c, c.g[V0][0], 0x11c);
            c.g[V1][0] = func;
            c.g[A0][0] = sx((fe as u32).wrapping_add(delta as u32));
            call_ptr(c, func as u32, 0x001d_8288)?;
        }

        // The "no cards" object (found by name hash).
        c.cycles += 10;
        c.g[A0][0] = sx(0x0046_78b0);
        c.g[S0][0] = Addr(gp(c, -0x10c8)).lw(c);
        call_ptr(c, STR_HASH, 0x001d_8298)?;
        c.g[A0][0] = lw_4(c, c.g[S0][0], 0x40);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001d_82a4)?;
        c.g[S3][0] = c.g[V0][0];
        let none = c.g[S3][0] == 0;
        c.g[A1][0] = 0x0046_0000;
        let mut tail = true;
        if !none {
            c.cycles += 32;
            c.g[A2][0] = 0x100;
            c.g[A3][0] = 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1d60));
            c.g[A0][0] = 0x7d0;
            call_ptr(c, 0x0031_7e30, 0x001d_82c4)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[A0][0] = sx(0x0046_78c8);
            c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
            c.g[S0][0] = lw_4(c, c.g[V0][0], 0x8c);
            c.g[S1][0] = lw_4(c, c.g[S0][0], 4);
            c.g[V0][0] = lh_2(c, c.g[S1][0], 0x20);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            call_ptr(c, STR_HASH, 0x001d_82e8)?;
            c.g[V1][0] = lw_4(c, c.g[S1][0], 0x24);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, c.g[V1][0] as u32, 0x001d_82f8)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x002c_2508, 0x001d_8304)?;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[S2][0];
            call_ptr(c, 0x003a_0d00, 0x001d_8310)?;
            c.g[V0][0] = lw_4(c, c.g[S3][0], 8);
            c.g[A1][0] = 1;
            c.g[A0][0] = lh_2(c, c.g[V0][0], 0x48);
            c.g[V1][0] = lw_4(c, c.g[V0][0], 0x4c);
            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, 0x001d_8328)?;
            let no_buf = c.g[S2][0] == 0;
            tail = !no_buf;
            if !no_buf {
                c.cycles += 2;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0031_7e98, 0x001d_8338)?;
            }
        }
        if tail {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x40);
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MemCardStateBase_HandleInput(self, input)` (0x001d8c50): let the input
/// object's "confirm" and "cancel" slots drive the two save/load actions and
/// play the matching screen frame. Returns whether input was consumed.
pub fn memcard_state_base_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = 1;
        c.g[S0][0] = c.g[A1][0]; // input
        c.g[S1][0] = c.g[A0][0]; // self
        let input = c.g[S0][0];
        let this = c.g[S1][0];
        c.g[S2][0] = 0;
        c.g[V1][0] = lw_4(c, this, 0x1a8);
        let busy = c.g[V1][0] != 1;
        let mut tail = false;
        if busy {
            // Confirm slot: `input->vt[19]()`.
            c.cycles += 7;
            let vt = lw_4(c, input, 8);
            c.g[V1][0] = vt;
            let delta = lh_2(c, vt, 0x98);
            let func = lw_4(c, vt, 0x9c);
            c.g[V0][0] = func;
            c.g[A0][0] = sx((input as u32).wrapping_add(delta as u32));
            call_ptr(c, func as u32, 0x001d_8c98)?;
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[S2][0] = 1;
                c.g[V0][0] = lw_4(c, this, 0x40);
                let no_screen = c.g[V0][0] == 0;
                sw_3(c, this, 0x1fc, c.g[S2][0]);
                if no_screen {
                    tail = true;
                } else {
                    c.cycles += 10;
                    c.g[A0][0] = sx(0x0046_7940);
                    call_ptr(c, STR_HASH, 0x001d_8cbc)?;
                    c.g[A0][0] = lw_4(c, this, 0x40);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_c778, 0x001d_8cc8)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = 0xffff;
                    let no_frame = c.g[A1][0] == c.g[V0][0];
                    c.g[V0][0] = c.g[S2][0];
                    if !no_frame {
                        c.cycles += 5;
                        c.g[A0][0] = lw_4(c, this, 0x40);
                        c.g[A2][0] = 1;
                        call_ptr(c, 0x0039_c870, 0x001d_8ce4)?;
                        c.g[V0][0] = c.g[S2][0];
                    }
                }
            } else {
                // Cancel slot: `input->vt[20]()`.
                c.g[V1][0] = lw_4(c, input, 8);
                c.cycles += 6;
                c.g[A0][0] = lh_2(c, c.g[V1][0], 0xa0);
                c.g[V0][0] = lw_4(c, c.g[V1][0], 0xa4);
                c.g[A0][0] = sx((input as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, c.g[V0][0] as u32, 0x001d_8cfc)?;
                let none = c.g[V0][0] == 0;
                c.g[V0][0] = c.g[S2][0];
                if !none {
                    c.cycles += 4;
                    c.g[S2][0] = 1;
                    c.g[V0][0] = lw_4(c, this, 0x40);
                    let no_screen = c.g[V0][0] == 0;
                    sw_3(c, this, 0x1f8, c.g[S2][0]);
                    if no_screen {
                        tail = true;
                    } else {
                        c.cycles += 10;
                        c.g[A0][0] = sx(0x0046_7950);
                        call_ptr(c, STR_HASH, 0x001d_8d20)?;
                        c.g[A0][0] = lw_4(c, this, 0x40);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039_c778, 0x001d_8d2c)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[V0][0] = 0xffff;
                        let no_frame = c.g[A1][0] == c.g[V0][0];
                        c.g[V0][0] = c.g[S2][0];
                        if no_frame {
                            tail = true;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = lw_4(c, this, 0x40);
                            c.g[A2][0] = 1;
                            call_ptr(c, 0x0039_c870, 0x001d_8d48)?;
                            tail = true;
                        }
                    }
                }
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        if tail {
            c.cycles += 1;
        }
        c.g[V0][0] = c.g[S2][0];
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
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
    use crate::difftest::compare;
    use crate::difftest::elf_image;
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
    use crate::difftest::seed_count;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::RA as RET;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::SP as STACK_TOP;
    pub(crate) use crate::game::ai::ai::tests::w16;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::put;
    pub(crate) use crate::game::ai::rider::tests::hh;
    pub(crate) use crate::game::ai::rider::tests::slot_2 as slot;
    pub(crate) use crate::game::be::bedbprofile::tests::strict;
    pub(crate) use crate::game::bx::bxstring2::tests::base_3;
    pub(crate) use crate::game::bx::bxstring2::tests::f;
    pub(crate) use crate::game::bx::bxstring2::tests::run as run_3;
    pub(crate) use crate::game::bx::bxstring2::tests::slot_2;
    pub(crate) use crate::game::bx::bxstring2::tests::BE_2 as BE_3;
    pub(crate) use crate::game::bx::bxstring2::tests::DB;
    pub(crate) use crate::game::bx::bxstring2::tests::RIDERS;
    pub(crate) use crate::game::bx::bxstring2::tests::RM as RM_2;
    pub(crate) use crate::game::bx::bxstring2::tests::VTS;
    pub(crate) use crate::game::bx::bxstring2::tests::W_2 as W_3;
    pub(crate) use crate::game::camera::cameracontroller::tests::put_vtable;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::gpf;
    pub(crate) use crate::game::dirtysock::tags2::tests::clear;
    pub(crate) use crate::game::dirtysock::tags2::tests::common;
    pub(crate) use crate::game::dirtysock::tags2::tests::dummy_world;
    pub(crate) use crate::game::dirtysock::tags2::tests::gp_at;
    pub(crate) use crate::game::dirtysock::tags2::tests::world_2 as world;
    pub(crate) use crate::game::dirtysock::tags2::tests::zero;
    pub(crate) use crate::game::dirtysock::tags2::tests::SEEDS_7 as SEEDS;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_4 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::check_all_2 as check_all;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_11 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_12 as globals_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_12 as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_10 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_9 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::put_str_3 as put_str;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_6 as run_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::screen_with;
    pub(crate) use crate::game::fe::feasyncfile::tests::str_hash;
    pub(crate) use crate::game::fe::feasyncfile::tests::ui;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_2 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ as OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_4 as OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3_2 as OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ4_2 as OBJ4;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4 as OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM;
    pub(crate) use crate::game::fe::feasyncfile::tests::RM_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::W;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_6 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W_4;
    pub(crate) use crate::game::fe::fememcard::tests::vtable_at;
    pub(crate) use crate::game::fe::fepopup3::tests::setup;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2;
    pub(crate) use crate::game::fe::festateoptions::tests::rng;
    pub(crate) use crate::game::fe::festateoptions::tests::setup as setup_2;
    pub(crate) use crate::game::fe::festateoptions::tests::AUX;
    pub(crate) use crate::game::movie::movieplayer::tests::alloc_base;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3 as base_2;
    pub(crate) use crate::game::movie::movieplayer::tests::globals;
    pub(crate) use crate::game::movie::movieplayer::tests::heap_init_2 as heap_init;
    pub(crate) use crate::game::movie::movieplayer::tests::pool_allocator;
    pub(crate) use crate::game::movie::movieplayer::tests::run_2 as run;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_2;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_LIBRARY;
    pub(crate) use crate::game::movie::movieplayer::tests::MEM_HEAP;
    pub(crate) use crate::game::movie::movieplayer::tests::W as W_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::belib_2 as belib;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::heap_setup;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn c_fe_popup_confirm_v25_matches() {
        check(
            ssx3_ee::s04::f_001d9d18__generated,
            c_fe_popup_confirm_v25,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0xab, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed as u64) << 4 | 3;
                let live = (seed % 2) as u32;
                c.bus.write32(DATA + 0x40, live);
                // `cFEPopupConfirm_ResetLayout` (+0x2bc) is not up: quick return.
                c.bus.write32(DATA + 0x2bc, 0);
            },
        );
    }

    #[test]
    fn mem_card_state_base_step_slot_dir_matches() {
        check(
            ee::s04::f_001d97e0__generated,
            mem_card_state_base_step_slot_dir,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x1d97_e0, 0);
                let list = DATA + 0x400;
                zero(c, list, 0x40);
                c.g[A1][0] = sx(list);
                c.g[A2][0] = sx((seed % 6) as u32);
                c.g[A3][0] = sx((seed & 1) as u32);
                c.bus.write32(list + 0x424, (seed % 6) + 1);
            },
        );
    }

    #[test]
    fn mem_card_state_base_set_mode_menu_matches() {
        check(
            ssx3_ee::s04::f_001d9368__generated,
            mem_card_state_base_set_mode_menu,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x26, 1);
                c.g[A0][0] = sx(DATA);
                c.bus
                    .write32(DATA + 0x214, if seed % 2 == 0 { 0 } else { 1 });
                vtable_at(c, &mut r, DATA, 0x118);
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x1d23e0 cFEStateVideoPlayer_Init.

    // -----------------------------------------------------------------------
    // 0x1d8358 MemCardUI_ShowFileDisplay.

    #[test]
    fn mem_card_ui_show_file_display_matches() {
        check(
            ee::s04::f_001d8358__generated,
            mem_card_ui_show_file_display,
            16,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1d83_58, 1);
                let fe = DATA + 0x100;
                c.bus.write32(GAME_GP.wrapping_sub(0x10c8), fe);
                match seed % 3 {
                    0 => c.bus.write32(fe + 0x214, 2), // already in mode 2
                    1 => c.bus.write32(fe + 0x1c0, 1), // a save is in progress
                    _ => {
                        c.bus.write32(fe + 0x214, 0);
                        c.bus.write32(fe + 0x1c0, 0);
                        let vt = vtable(c, &mut r, &[0x118]);
                        c.bus.write32(fe + 8, vt);
                    }
                }
            },
        );
    }

    // -----------------------------------------------------------------------
    // 0x1d86b0 MemCardStateBase_v45.

    #[test]
    fn mem_card_state_base_v45_matches() {
        check(
            ee::s04::f_001d86b0__generated,
            mem_card_state_base_v45,
            16,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1d86_b0, 2);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                let vt = vtable(c, &mut r, &[0x178]);
                c.bus.write32(this + 8, vt);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), 0);
                c.bus.write32(GAME_GP.wrapping_sub(0x47c), DATA + 0x2000);
            },
        );
    }

    #[test]
    fn mem_card_state_base_reset_state_matches() {
        check(
            ee::s04::f_001d8be0__generated,
            mem_card_state_base_reset_state,
            SEEDS,
            &|c, seed| {
                let _ = common(c, seed, 0x1d8be0, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x400);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn mem_card_state_base_set_use_save_title_matches() {
        check(
            ssx3_ee::s04::f_001d9308__generated,
            mem_card_state_base_set_use_save_title,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x1d9308);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = if seed % 3 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
                // The overlay manager and its movie player.
                c.bus.write32(GAME_GP.wrapping_sub(0x47c), DATA + 0x700);
                c.bus.write32(DATA + 0x700 + 0x434, DATA + 0x900);
                let vt = vtable(c, &mut r, &[0x70]);
                c.bus.write32(DATA + 0x900, vt);
                c.bus.write32(DATA + 0x700 + 0xf8, r.next_u32());
            },
        );
    }

    #[test]
    fn mem_card_state_base_on_activate_matches() {
        check(
            ssx3_ee::s04::f_001d9818__generated,
            mem_card_state_base_on_activate,
            120,
            &|c, seed| {
                let _r = setup(c, seed, 0x1d9818);
                c.g[A0][0] = DATA as u64;
                // No UI engine: the by-hash lookup returns null and takes the
                // leave-the-widget-alone path.
                c.bus.write32(DATA + 0x40, 0);
            },
        );
    }

    // ------------------------------------------------------------------
    // Memory-card and overlay states.

    #[test]
    fn mem_card_state_base_v46_matches() {
        check(
            ee::s04::f_001d8670__generated,
            mem_card_state_base_v46,
            200,
            &|c, seed| {
                let mut r = setup_2(c, seed, 31);
                gpf(c, -0x10c8, AUX);
                put(
                    c,
                    AUX + 0x1c0,
                    if seed % 2 == 0 { 4 } else { r.next_u32() % 8 },
                );
                put(c, AUX + 0x1a8, r.next_u32());
                put(c, AUX + 0x1e0, r.next_u32());
            },
        );
    }

    #[test]
    fn mem_card_state_base_transition_out_matches() {
        check(
            ee::s04::f_001d93d8__generated,
            mem_card_state_base_transition_out,
            200,
            &|c, seed| {
                setup_2(c, seed, 32);
                c.g[A0][0] = AUX as u64;
                put(c, AUX + 0x1c0, [6u32, 5, 7, 0x100][(seed % 4) as usize]);
            },
        );
    }

    // -----------------------------------------------------------------------

    #[test]
    fn memcard_ui_on_load_aborted_matches() {
        check(
            ee::s04::f_001d72a8__generated,
            memcard_ui_on_load_aborted,
            60,
            &|c, seed| {
                base(c, seed, 0x1d72_a8);
                let fe = DATA + 0x1000;
                let vt = DATA + 0x1200;
                c.bus.write32(GAME_GP - 0x10c8, fe);
                c.bus.write32(fe + 8, vt);
                put_vtable(c, vt, &[0x148]);
                let mc = DATA + 0x1400;
                let vt2 = DATA + 0x1600;
                c.bus.write32(GAME_GP - 0x47c, mc);
                c.bus.write32(mc + 0x748, vt2);
                put_vtable(c, vt2, &[8]);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn c_fe_popup_confirm_v27_matches() {
        check(
            ssx3_ee::s04::f_001d9ca8__generated,
            c_fe_popup_confirm_v27,
            SEEDS_2,
            &|c, seed| {
                let _ = machine(c, seed, 0x1d9ca8, 0);
                let s = DATA + 0x1000;
                let src = DATA + 0x2000;
                c.g[A0][0] = s as u64;
                c.g[A1][0] = if seed % 4 == 0 { 0 } else { src as u64 };
                // A short UTF-16 string.
                w16(c, src, if seed % 3 == 0 { 0 } else { 0x41 });
                w16(c, src + 2, 0x42);
                w16(c, src + 4, 0);
                w32(c, s + 0x40, if seed % 2 == 0 { 0 } else { 1 });
                w32(c, s + 0x2bc, 0); // ResetLayout path avoided
            },
        );
    }

    #[test]
    fn mem_card_state_base_show_status_matches() {
        check(
            ee::s04::f_001d8d68__generated,
            mem_card_state_base_show_status,
            12,
            &|c, seed| {
                machine(c, seed, 0x1d_8d68, 3);
                belib(c);
                world(c);
                heap_setup(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                w(c, DATA + 0x2000 + 0x8, DATA + 0x2400);
                slot(c, DATA + 0x2400, 0x120, 0, FAKE);
                c.pc = 0x001d_8d68;
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

    #[test]
    fn mem_card_state_base_alloc_read_buffer_matches() {
        check(
            ee::s04::f_001d9948__generated,
            mem_card_state_base_alloc_read_buffer,
            12,
            &|c, seed| {
                let _ = alloc_base(c, seed, 0x1d99_48, 4);
                let self_ = DATA;
                w(c, self_ + 0x228, (seed % 2) as u32);
                let old = if seed % 3 == 0 { 0 } else { alloc(c, 0x100) };
                w(c, self_ + 0x1d4, old);
                c.g[A0][0] = self_ as u64;
                c.g[A1][0] = 0x100;
            },
        );
    }

    #[test]
    fn mem_card_state_base_play_widget_anim_matches() {
        check(
            ee::s04::f_001d93e8__generated,
            mem_card_state_base_play_widget_anim,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1d93e8, 2);
                put_str(c, DATA + 0x400, b"widget");
                let h = str_hash(c, DATA + 0x400);
                let screen = DATA + 0x1000;
                screen_with(c, &mut r, screen, h);
                // The animation set at_3 `*(self+0x10)+0x50` has no entries, so
                // CreateAnim answers 0.
                c.bus.write32(DATA + 0x10, DATA + 0x2000);
                c.bus.write32(DATA + 0x2050, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x400);
                c.g[A2][0] = sx(DATA + 0x500);
                c.bus
                    .write32(DATA + 0x40, if seed % 4 == 0 { 0 } else { screen });
            },
        );
    }

    /// A screen at `screen` whose object list holds `node` for the string at
    /// `name` (the zeroed game string). `+0x38` points at the object count and
    /// `+0x3c` at the object array, as `cUIScreen_getObjectByHashName` reads
    /// them; `node+0x38` is that string's hash.
    pub(crate) fn hash_screen(c: &mut Cpu, screen: u32, node: u32, name: u32) {
        let h = str_hash(c, name);
        let list = DATA + 0x5_6000;
        let count = DATA + 0x5_6010;
        c.bus.write32(screen + 0x38, count);
        c.bus.write32(screen + 0x3c, list);
        c.bus.write32(count, 1);
        c.bus.write32(list, node);
        c.bus.write32(node + 0x38, h);
    }

    #[test]
    fn memcard_state_base_sync_menu_selection_matches() {
        check(
            ee::s04::f_001d9258__generated,
            memcard_state_base_sync_menu_selection,
            80,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x1d92);
                let screen = DATA + 0x1000;
                let node = DATA + 0x2000;
                c.g[A0][0] = sx(DATA);
                c.bus
                    .write32(DATA + 0x40, if seed % 6 == 0 { 0 } else { screen });
                hash_screen(c, screen, node, 0x004a_1600);
                let item = DATA + 0x3000;
                let variant = seed % 4;
                c.bus
                    .write32(node + 0xa0, if variant == 0 { 0 } else { item });
                c.bus.write8(node + 0x95, (seed % 5) as u8);
                c.bus.write32(item + 0x14, variant << 5);
                c.bus.write32(node + 8, DATA + 0x4000);
                slot_2(c, DATA + 0x4000, 0x48, 1);
                fill(c, &mut r, DATA + 0x1c4, 0x10);
            },
        );
    }

    // ------------------------------------------------------------ more screens

    #[test]
    fn cfe_popup_confirm_ctor_matches() {
        check(
            ee::s04::f_001d99c8__generated,
            cfe_popup_confirm_ctor,
            6,
            &|c, seed| {
                let _ = base_4(c, seed, 0x001d_99c8, 2);
                w(c, GAME_GP.wrapping_sub(0x10b4), (seed % 2) as u32);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                c.g[A2][0] = OBJ3 as u64;
                c.g[A3][0] = OBJ4 as u64;
                c.g[T0][0] = (seed % 5) as u64;
                c.g[T1][0] = (seed % 7) as u64;
                c.g[T2][0] = (seed % 9) as u64;
                c.g[T3][0] = (seed % 11) as u64;
                w(c, OBJ + 0x17c, 0);
                w(c, OBJ + 0x14c, 0);
            },
        );
    }

    #[test]
    fn memcard_state_base_is_card_ready_matches() {
        strict(
            ee::s04::f_001d9880__generated,
            memcard_state_base_is_card_ready,
            80,
            &|c, seed| {
                let _r = base_5(c, seed, 0x1d98);
                c.g[A0][0] = DATA as u64;
                let m = DATA + 0x3000;
                c.bus.write32(gp_at(-0x47c), m);
                c.bus
                    .write32(DATA + 0x1c0, [0u32, 6, 2][(seed % 3) as usize]);
                c.bus.write32(m + 0x340, (seed % 2) as u32);
                c.bus.write32(m + 0xe0, ((seed / 2) % 2) as u32);
                c.bus.write32(m + 0x130, (seed % 4) as u32);
                c.bus.write32(m + 0x434, m + 0x1000);
                c.bus.write32(m + 0x1000, m + 0x1100);
                c.bus.write16(m + 0x1100 + 0x58, 0);
                c.bus.write32(m + 0x1100 + 0x5c, FAKE);
                c.bus.write16(m + 0x1100 + 0x258, 0);
                c.bus.write32(m + 0x1100 + 0x25c, FAKE + 0x10);
            },
        );
    }

    #[test]
    fn mem_card_ui_try_next_slot_matches() {
        diff(
            ee::s04::f_001d8160__generated,
            mem_card_ui_try_next_slot,
            30,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x1d_8160);
                c.g[A0][0] = sx(DATA);
                w(c, DATA + 0x424, (seed % 4) as u32);
                w(c, DATA + 0x428, (seed % 3) as u32);
                w(c, DATA + 0x434, r.next_u32());
                w(c, DATA + 0x1a0, r.next_u32());
                w(c, DATA + 0x1a4, r.next_u32());
                w(c, DATA + 0x1c0, r.next_u32());
            },
        );
    }

    #[test]
    fn memcard_state_base_handle_input_matches() {
        check_all(
            ee::s04::f_001d8c50__generated,
            memcard_state_base_handle_input,
            60,
            &|c, seed| {
                machine(c, seed, 0x1d8c, 0);
                let this_ = DATA + 0x1000;
                let input = DATA + 0x2000;
                c.g[A0][0] = this_ as u64;
                c.g[A1][0] = input as u64;
                c.bus.write32(this_ + 0x1a8, 0);
                c.bus.write32(this_ + 0x40, 0);
                // `MenuSys_IsActive` stands in for the input slots: it returns
                // whether the object's first word is non-zero and +0x54 is too.
                let vt = DATA + 0x3000;
                c.bus.write16(vt + 0x98, 0);
                c.bus.write32(vt + 0x9c, 0x002c_c098);
                c.bus.write16(vt + 0xa0, 0x100);
                c.bus.write32(vt + 0xa4, 0x002c_c098);
                c.bus.write32(input + 8, vt);
                let first = if seed % 3 == 0 { 1 } else { 0 };
                c.bus.write32(input, first);
                c.bus.write32(input + 0x54, 1);
                let second = if seed % 3 == 1 { 1 } else { 0 };
                c.bus.write32(input + 0x100, second);
                c.bus.write32(input + 0x154, 1);
            },
        );
    }
}
