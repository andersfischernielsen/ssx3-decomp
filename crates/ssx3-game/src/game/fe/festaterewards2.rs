//! `src/fe/festaterewards2.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::camera::cameraalgorithms::lh as lh_2;
pub(crate) use crate::game::camera::script::scriptcontroller::vslot;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lh_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::dirtysock::tags2::lb_at;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_3;
pub(crate) use crate::game::fe::feasyncfile::gp_16 as gp;
pub(crate) use crate::game::fe::feasyncfile::lh;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw_3;
pub(crate) use crate::game::fe::feasyncfile::p as p_4;
pub(crate) use crate::game::fe::feasyncfile::p as p_5;
pub(crate) use crate::game::fe::feasyncfile::p as p_6;
pub(crate) use crate::game::fe::feasyncfile::sw_5 as sw_2;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::fe::fepopup2::fe_state_screen_init;
pub(crate) use crate::game::fe::festaterewards::at_4 as at_2;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw_2;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::fe::ovtemplatepausemenu::p as p_2;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::gamestate::p_2 as p_3;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2 as p;
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
        0x001d_4368,
        c_fe_state_trophy_room_init,
        "cFEStateTrophyRoom_Init",
    ),
    (
        0x001d_4660,
        cfe_state_trophy_room_on_widget_notify,
        "cFEStateTrophyRoom_OnWidgetNotify",
    ),
    (
        0x001d_4698,
        cfe_state_trophy_room_on_widget_action,
        "cFEStateTrophyRoom_OnWidgetAction",
    ),
    (
        0x001d_4760,
        fe_state_trophy_room_update,
        "cFEStateTrophyRoom_Update",
    ),
    (
        0x001d_4890,
        c_fe_state_trophy_room_show_trophy_info,
        "cFEStateTrophyRoom_ShowTrophyInfo",
    ),
    (
        0x001d_4b20,
        cfe_state_trophy_room_release_trophy_images,
        "cFEStateTrophyRoom_ReleaseTrophyImages",
    ),
    (
        0x001d_4bc8,
        cfe_state_rewards_room_ctor,
        "cFEStateRewardsRoom_ctor",
    ),
    (
        0x001d_4c60,
        c_fe_state_rewards_room_init,
        "cFEStateRewardsRoom_Init",
    ),
    (
        0x001d_4ea8,
        fe_state_rewards_room_on_widget_action,
        "cFEStateRewardsRoom_OnWidgetAction",
    ),
    (
        0x001d_4f68,
        cfe_state_rewards_room_on_widget_notify,
        "cFEStateRewardsRoom_OnWidgetNotify",
    ),
    (
        0x001d_4f90,
        cfe_rewards_room_set_count_text,
        "cFEStateRewardsRoom_SetCountText",
    ),
    (0x001d_5038, fe_state_relogin_ctor, "cFEStateRelogin_ctor"),
    (0x001d_5078, c_fe_state_relogin_init, "cFEStateRelogin_Init"),
    (
        0x001d_50e0,
        cfe_state_relogin_on_widget_created,
        "cFEStateRelogin_OnWidgetCreated",
    ),
    (
        0x001d_5240,
        fe_state_relogin_on_child_event,
        "cFEStateRelogin_OnChildEvent",
    ),
    (0x001d_5280, cfe_relogin_v33, "cFEStateRelogin_v33"),
    (
        0x001d_5330,
        memcardstatebase_ctor_2,
        "MemCardStateBase_ctor_2",
    ),
    (
        0x001d_53b0,
        mem_card_state_base_ctor,
        "MemCardStateBase_ctor",
    ),
    (
        0x001d_5428,
        mem_card_state_base_dtor,
        "MemCardStateBase_dtor",
    ),
    (
        0x001d_5460,
        mem_card_state_base_on_widget_notify,
        "MemCardStateBase_OnWidgetNotify",
    ),
    (
        0x001d_58b8,
        mem_card_state_base_shutdown,
        "MemCardStateBase_Shutdown",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::AVALANCHE_GET_OBJECT_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_TRACK_INFO;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_LIBRARY_GET_INSTANCE as BE_LIBRARY_GET_INSTANCE_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_LIBRARY_GET_INTERFACE as BE_LIBRARY_GET_INTERFACE_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_MISSION_SET_COLLECTED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_SET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_COUNT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUY_POPUP_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::CLEAR_NEW_REWARDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::DYNAMIC_EMITTER_RESET;
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
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_3;
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
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_GET_INSTANCE as SSX_AUDIO_GET_INSTANCE_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_IS_TRACK_GROUP5;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_RESUME_SFX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STREAM_FILE_QUEUE_RANGE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_COPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_RAW;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TEX_MAN_INIT_TABLE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_CTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LISTBOX_SET_ITEM_TEXT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST as UI_LIST_FIRST_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST_WRAPPER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_NODE_SENTINEL;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_FIND_BY_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_SCREEN_FIND_OBJECT;
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
    pub(crate) use crate::game::dirtysock::tags2::callee::LUNO_TABLE_DTOR;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_RENDER_FADE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_TRICK_METER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STOP_WIPEOUT_LOOP2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUTOSAVE_UPDATE_ICON;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_INSTANCE_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_KILL_TIMERS_BY_CALLBACK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_STOP_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INSTANCE_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INTERFACE_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_MISSION_INT_GET_SESSION_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_HUMAN_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_RACE_INT_GET_SPECIAL_EVENT_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_RACE_INT_GET_TRACK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SCORE_INT_GET_HIGH_SCORE_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_FILE_CACHE_SHUTDOWN;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_ALLOC_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_BX_STRING_ASSIGN_CSTR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_BX_STRING_CTOR_CSTR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_COMM_SYSTEM_CHANNEL_RECV;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_COMM_SYSTEM_CHANNEL_SEND;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_CUT_SCENE_TRANSITION_MAN_SAVE_VIEWPORT;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_BUY_SELECTED_ITEM;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_REFRESH_ITEM_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_REMOVE_LIST_ITEM;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_UPDATE_MONEY_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_LOAD_CONFIG_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_SET_LIST_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_SHOW_CARD_ERROR_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_MESSENGER_CLEAR_NAME_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_QUICK_PLAY_POPUP_SET_OPTIONS_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_QUICK_PLAY_POPUP_SET_OPTIONS_HINT_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_SSX_WORLD_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_OPTIONS_DEVICE_SELECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_TROPHY_ROOM_SHOW_TROPHY_IMAGE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_TROPHY_ROOM_SHOW_TROPHY_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_USE_EXISTING_ACCOUNT_CHECK_EMAIL;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_SETUP_MAN_ALLOC_PACKET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_SETUP_MAN_FREE_PACKET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_TRANSITION_MAN_START;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_TRANSITION_MAN_START_CAPTURED;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ASYNC_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_LOCALE_LOAD_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_PLAYER_PAD_MASK_2 as FE_MODULE_GET_PLAYER_PAD_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_RELEASE_BOLT_FILES;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_UNLOAD_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_ANIM_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_UNLOAD_RIDER_PREVIEWS;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_RAIL_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_ADVANCE_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_QUIT_TO_LOAD_SCREEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_EXPECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_GET_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_READ_QUOTED_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_BINDING_EVAL_FLOAT;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_SPEED;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_USER_DATA_2 as IN_SEQ_INSTANCE_SET_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_START_SEQUENCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_POPUP_FIND_ENABLED_IN_COLUMN_FROM_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_POPUP_IS_KEY_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_CONNECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MENU_HIDE_ROOM_TABS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_TABLE_FIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_CALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_PASS_THROUGH;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_AS_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR_2 as LUNO_VM_REGISTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_FORMAT_SLOT_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCARD_INSTALL_CALLBACKS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC_NAMED_128;
    pub(crate) use crate::game::fe::feasyncfile::callee::MOD_OBJ_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_EXTENSION_AND_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_GET_BITS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_MARKER_BIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_RESUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_9 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_DTOR_5 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GOTO_NEXT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_13 as NET_FE_STATE_ON_CHILD_EVENT_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_5 as NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_DATA_FIND_MARKER;
    pub(crate) use crate::game::fe::feasyncfile::callee::PLAYBACK_RESTORE_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_GET_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_GET_START_POSE_ANGLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_IS_LOADED;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_METRIX_SHOW_HUD_MSG_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_PLACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SCRIPT_OBJ_MATCHES_RIDER_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_TRANSITION_TARGET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_BINDING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_TICK_PROCESS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_RIDER_GET_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_UNLOAD_BANK_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK_2 as SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPCH_MAKE_ID_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_SYSTEM_IS_INSTANCE_READY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPLINE_MODIFIER_SET_SPEED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_CHARACTER_TO_SPEECH_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_FADE_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_IS_SPEECH_VOLUME_ON;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_SET_FADE_GAIN;
    pub(crate) use crate::game::fe::feasyncfile::callee::STR_HASH as STR_HASH_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_CTOR_2 as UI_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_MENU_SET_SELECTION_3 as UI_MENU_SET_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_LOAD as UI_OBJECT_LOAD_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_5 as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_CTOR_2_4 as UI_STATE_CTOR_2_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_ALL_POPUPS_2 as UI_STATE_MGR_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_TOP_STATES as UI_STATE_MGR_CLOSE_TOP_STATES_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_SWITCH_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXTURE_SET_FIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_GET_ID_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_GET_OWNED_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_SET_TEXT_ID_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_LEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_SPRINTF_ASCII_FMT;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_TO_ASCII;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CHUNK_RESOLVE_HOOK;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2 as WORLD_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_DTOR_2 as WORLD_OBJ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_RUN_KILL_SCRIPT_2 as WORLD_OBJ_RUN_KILL_SCRIPT;
    pub(crate) use crate::game::main::gamemode::callee::AI_RAND;
    pub(crate) use crate::game::main::gamemode::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::main::gamemode::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GET_MEDAL_REWARD_ID;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GRANT_REWARD;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_IS_REWARD_GRANTED;
    pub(crate) use crate::game::main::gamemode::callee::BUILTIN_DELETE as BUILTIN_DELETE_2;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_PARSE;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_UNLOAD;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS_MASK2;
    pub(crate) use crate::game::main::gamemode::callee::CBE_GET_BE;
    pub(crate) use crate::game::main::gamemode::callee::CBX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::main::gamemode::callee::CPS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_FREE_BUFFERS;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_RESET;
    pub(crate) use crate::game::main::gamemode::callee::CUI_TEXT_SCROLL_SET_WIDE_STRING;
    pub(crate) use crate::game::main::gamemode::callee::CVT_FLOAT_TO_INT;
    pub(crate) use crate::game::main::gamemode::callee::C_MEM_MAN_ALLOC;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::main::gamemode::callee::EA_MEM_FREE as EA_MEM_FREE_2;
    pub(crate) use crate::game::main::gamemode::callee::FE_ASYNC_MANAGER_SET_FILE_STATUS;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_RIDER_PREVIEW_DTOR;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_UNLOAD_RIDER_PREVIEWS;
    pub(crate) use crate::game::main::gamemode::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES;
    pub(crate) use crate::game::main::gamemode::callee::FUNC_0028F140;
    pub(crate) use crate::game::main::gamemode::callee::MEMCPY;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET as MEMSET_2;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET_3E6448;
    pub(crate) use crate::game::main::gamemode::callee::MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::main::gamemode::callee::MODIFIER_BLOCK_HAS_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_CTOR_2;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_SET_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::MULTI_PARTICLE_DTOR;
    pub(crate) use crate::game::main::gamemode::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_2;
    pub(crate) use crate::game::main::gamemode::callee::RAIL_MODIFIER_CTOR;
    pub(crate) use crate::game::main::gamemode::callee::REPLAY_FIND_SNAPSHOT;
    pub(crate) use crate::game::main::gamemode::callee::REPLAY_REQUEST_INSTANT_REPLAY;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_GET_SYNC_CHECKSUM;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_BANK_PENDING_SCORE;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_CLEAR_HUD_MESSAGES;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_COMBO;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_STATS;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_TRICK;
    pub(crate) use crate::game::main::gamemode::callee::RUN_LIST_SEEK;
    pub(crate) use crate::game::main::gamemode::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::main::gamemode::callee::SCRIPT_LIST_ON_SEQ_ADVANCE;
    pub(crate) use crate::game::main::gamemode::callee::SPRINTF as SPRINTF_2;
    pub(crate) use crate::game::main::gamemode::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::main::gamemode::callee::STRCPY;
    pub(crate) use crate::game::main::gamemode::callee::STRLEN;
    pub(crate) use crate::game::main::gamemode::callee::STR_HASH;
    pub(crate) use crate::game::main::gamemode::callee::TEX_MAN_UNLOCK_VRAM;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_ACCEPT_VISITOR;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::main::gamemode::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::main::gamemode::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::main::gamemode::callee::WSCRIPT_MAN_RESET_COLLECT_SET;
    pub(crate) use crate::game::main::gamestate::callee::ADF_DATA_SOURCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::ANIM_DATA_BASE_FREE_CHAN_CACHES;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_STOP_HANDLE as BANK_MONITOR_STOP_HANDLE_2;
    pub(crate) use crate::game::main::gamestate::callee::BE_ECON_INT_GET_TOTAL_EARNED;
    pub(crate) use crate::game::main::gamestate::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_2;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INSTANCE_3 as BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INTERFACE_3 as BE_LIBRARY_GET_INTERFACE_2;
    pub(crate) use crate::game::main::gamestate::callee::BE_NEW_PLAYER_INT_GET_SLOT as BE_NEW_PLAYER_INT_GET_SLOT_2;
    pub(crate) use crate::game::main::gamestate::callee::BOLTS_FIND_BOLT_INDEX;
    pub(crate) use crate::game::main::gamestate::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_3;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_BRAGGING_RIGHTS_INIT_PLAYER_ROWS;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_CHAR_STATS;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::main::gamestate::callee::C_GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::main::gamestate::callee::C_KEYBOARD_POPUP_SET_KEY_ENABLED;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_ALLOC_NAMED_3 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_FREE_4 as EA_MEM_FREE_3;
    pub(crate) use crate::game::main::gamestate::callee::GAME_ANIM_MAP_UNREGISTER_BANK;
    pub(crate) use crate::game::main::gamestate::callee::GAME_STATE_LOAD_STATE;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_CLEAR_SLOTS;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_INSTALL_VIF_VU_HANDLERS;
    pub(crate) use crate::game::main::gamestate::callee::MEMCARD_STATE_BASE_INSTALL_CALLBACKS;
    pub(crate) use crate::game::main::gamestate::callee::MEM_FREE;
    pub(crate) use crate::game::main::gamestate::callee::MEM_MOVE;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_GET_INPUT_CODE;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_HANDLE_INPUT;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_IS_DISABLED;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_NEXT;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_PREV;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_VALIDATE_SELECTION;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_COUNT_GROUP_MESSAGES;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_GET_MESSAGE_COUNT;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_REMOVE_MESSAGE;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_ON_CHILD_EVENT_3 as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_OPEN_KEYBOARD_TEXT_ID;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_REFRESH_LIST;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_COUNT_TEXT;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_SCROLL_ARROWS;
    pub(crate) use crate::game::main::gamestate::callee::RIDER_CLEAR_NEARBY_LIGHTS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_PAUSE_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_RESUME_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_CMD_QUEUE_AT_2;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_SCRIPT_OBJ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_IS_OBJ_BOUND_TO_PROCESS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_START;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_STOP_3 as SCRIPT_ENGINE_LIST_STOP_2;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_OBJ_CAM_HAS_BINDING;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_OBJ_CAM_IS_BOUND_TO_PROCESS;
    pub(crate) use crate::game::main::gamestate::callee::SECTION_ALLOCATOR_CTOR;
    pub(crate) use crate::game::main::gamestate::callee::SNDSTRM_CLOSE;
    pub(crate) use crate::game::main::gamestate::callee::SPCH_MAKE_ID;
    pub(crate) use crate::game::main::gamestate::callee::SPCH_SHUTDOWN;
    pub(crate) use crate::game::main::gamestate::callee::SPEECH_INSTANCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::main::gamestate::callee::SPHERE_TREE_COLLIDE_AABB;
    pub(crate) use crate::game::main::gamestate::callee::SPHERE_TREE_COLLIDE_SPHERE_TREE;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN_PTR;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_2;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_LANGUAGE_MASK;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_RESET_RACE_SOUNDS;
    pub(crate) use crate::game::main::gamestate::callee::STR_COPY;
    pub(crate) use crate::game::main::gamestate::callee::STR_N_COPY;
    pub(crate) use crate::game::main::gamestate::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_FIRST_2;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_NODE_DTOR_BODY as UI_LIST_NODE_DTOR_BODY_2;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_CTOR_3 as UI_STATE_CTOR_2;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_SHUTDOWN_2 as UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::main::gamestate::callee::VD_TEXTURE_SINK_CTOR;

    pub const FE_STATE_ON_ACTIVATE: Callee = Callee {
        addr: 0x0018_6518,
        f: ee::s02::cFEState_OnActivate,
    };

    pub const REQUEST_LINE_UPDATE_SONG_NAMES: Callee = Callee {
        addr: 0x0019_7bc8,
        f: ee::s02::cFEStateRequestLine_UpdateSongNames,
    };

    pub const REQUEST_LINE_UPDATE_SONG_ICONS: Callee = Callee {
        addr: 0x0019_7ca0,
        f: ee::s02::cFEStateRequestLine_UpdateSongIcons,
    };

    pub const REQUEST_LINE_UPDATE_SCROLL_ARROWS: Callee = Callee {
        addr: 0x0019_7db8,
        f: ee::s02::cFEStateRequestLine_UpdateScrollArrows,
    };

    pub const REQUEST_LINE_UPDATE_SONG_COUNT: Callee = Callee {
        addr: 0x0019_8118,
        f: ee::s02::cFEStateRequestLine_UpdateSongCount,
    };

    pub const NET_FE_STATE_ON_DISCONNECTED_POPUP_EVENT: Callee = Callee {
        addr: 0x001a_9668,
        f: ee::s03::NetFEState_OnDisconnectedPopupEvent,
    };

    pub const NET_FE_STATE_ON_CHILD_EVENT: Callee = Callee {
        addr: 0x001a_97b8,
        f: ee::s03::NetFEState_OnChildEvent,
    };

    pub const NET_FE_STATE_ON_ACTIVATE: Callee = Callee {
        addr: 0x001a_87d0,
        f: ee::s03::NetFEState_OnActivate,
    };

    pub const CHAT_ROOM_UPDATE_PLAYER_QUALITY: Callee = Callee {
        addr: 0x001b_3bf0,
        f: ee::s03::cFEStateChatRoom_UpdatePlayerQuality,
    };

    pub const FE_STATE_LOGIN_CTOR: Callee = Callee {
        addr: 0x001b_ee30,
        f: ee::s03::cFEStateLogin_ctor,
    };

    pub const FE_STATE_LOGIN_ON_CHILD_EVENT: Callee = Callee {
        addr: 0x001b_f6c0,
        f: ee::s03::cFEStateLogin_OnChildEvent,
    };

    pub const NET_QUICK_PLAY_POPUP_ON_OPTIONS_EVENT: Callee = Callee {
        addr: 0x001c_1b88,
        f: ee::s03::cFEStateNetQuickPlayPopup_OnOptionsPopupEvent,
    };

    pub const FE_STATE_STATS_REFRESH_RESULTS: Callee = Callee {
        addr: 0x001c_3310,
        f: ee::s03::cFEStateStats_RefreshResults,
    };

    pub const POPUP_CONFIRM_GET_TEXT_BOX_SIZE: Callee = Callee {
        addr: 0x001c_9210,
        f: ee::s03::cFEPopupConfirm_GetTextBoxSize,
    };

    pub const PREVIEW_REWARD_POLL_IMAGE_LOAD: Callee = Callee {
        addr: 0x001d_2198,
        f: ee::s04::cFEStatePreviewReward_PollImageLoad,
    };

    pub const MOUNTAIN_ROOM_UPDATE_EVENT_ICONS: Callee = Callee {
        addr: 0x001d_3340,
        f: ee::s04::cFEStateMountainRoom_UpdateEventIcons,
    };

    pub const TROPHY_ROOM_LOAD_TROPHY_TEXTURES: Callee = Callee {
        addr: 0x001d_4a20,
        f: ee::s04::cFEStateTrophyRoom_LoadTrophyTextures,
    };

    pub const SECTION_MAN_FIND_LOGICAL_SECTION: Callee = Callee {
        addr: 0x0022_e0e0,
        f: ee::s05::SectionMan_FindLogicalSection,
    };

    pub const GAME_MODULE_REQUEST_STATE: Callee = Callee {
        addr: 0x0023_1250,
        f: ee::s06::GameModule_RequestState,
    };

    pub const APP_MODULE_TRY_EXIT: Callee = Callee {
        addr: 0x0023_1cf0,
        f: ee::s06::AppModule_TryExit,
    };

    pub const S_LOGIN_UPDATE: Callee = Callee {
        addr: 0x003f_8048,
        f: ee::s14::SLogin_Update,
    };

    pub const USB_KB_DEFAULT_CONNECT_CB: Callee = Callee {
        addr: 0x003f_6358,
        f: ee::s14::UsbKb_DefaultConnectCb,
    };

    pub const WORLD_STATE_STREAM_RESET: Callee = Callee {
        addr: 0x0026_d628,
        f: ee::s07::WorldStateStream_Reset,
    };

    pub const REPLAY_STREAM_CTOR: Callee = Callee {
        addr: 0x0026_e8e0,
        f: ee::s07::ReplayStream_ctor,
    };

    pub const REPLAY_FIND_NEXT_SNAPSHOT: Callee = Callee {
        addr: 0x0026_e468,
        f: ee::s07::Replay_FindNextSnapshot,
    };

    pub const REPLAY_FIND_PREV_SNAPSHOT: Callee = Callee {
        addr: 0x0026_e490,
        f: ee::s07::Replay_FindPrevSnapshot,
    };

    pub const REPLAY_SNAP_TO_SNAPSHOT: Callee = Callee {
        addr: 0x0026_e4b8,
        f: ee::s07::Replay_SnapToSnapshot,
    };

    pub const REPLAY_APPLY_CAMERA: Callee = Callee {
        addr: 0x0026_edd8,
        f: ee::s07::Replay_ApplyCamera,
    };

    pub const REPLAY_SET_STATE_2: Callee = Callee {
        addr: 0x0026_f4a0,
        f: ee::s07::Replay_SetState,
    };

    pub const SCRIPT_LIST_ADD_MOVIE: Callee = Callee {
        addr: 0x0027_6048,
        f: ee::s07::ScriptList_AddMovie,
    };

    pub const SCRIPT_LIST_POLL_LOADING: Callee = Callee {
        addr: 0x0027_7778,
        f: ee::s07::ScriptList_PollLoading,
    };

    pub const SCRIPT_CMD_QUEUE_AT: Callee = Callee {
        addr: 0x0028_3d70,
        f: ee::s07::ScriptCmdQueue_At,
    };

    pub const SCRIPT_ENGINE_UNLOAD_WORLD_SCRIPTS: Callee = Callee {
        addr: 0x0027_8cd8,
        f: ee::s07::ScriptEngine_UnloadWorldScripts,
    };

    pub const SCRIPT_ENGINE_LIST_STOP: Callee = Callee {
        addr: 0x0027_90a0,
        f: ee::s07::ScriptEngine_ListStop,
    };

    pub const CROWD_AUDIO_INSTANCE_STOP_SOUND: Callee = Callee {
        addr: 0x002a_6848,
        f: ee::s08::CrowdAudio_InstanceStopSound,
    };

    pub const SONG_INSTANCE_GET_DUCK_TO_LOOPS: Callee = Callee {
        addr: 0x002b_4868,
        f: ee::s08::SongInstance_GetDuckToLoops,
    };

    pub const WORLD_TRIGGER_MANAGER_GET_TRIGGER_INFO: Callee = Callee {
        addr: 0x002b_63d0,
        f: ee::s08::WorldTriggerManager_GetTriggerInfo,
    };

    pub const MEM_MAN_FREE: Callee = Callee {
        addr: 0x0031_7e98,
        f: ee::s10::f_00317e98,
    };

    pub const MEM_GET_POOL_BLOCK_SIZE: Callee = Callee {
        addr: 0x0031_a130,
        f: ee::s10::MEM_GetPoolBlockSize,
    };

    pub const DL_HEAP_USABLE_SIZE: Callee = Callee {
        addr: 0x0031_fbb8,
        f: ee::s10::DLHeap_UsableSize,
    };

    pub const MEM_REPORT_POOL_STATS: Callee = Callee {
        addr: 0x0031_a200,
        f: ee::s10::MEM_ReportPoolStats,
    };

    pub const PS2_PAD_CREATE: Callee = Callee {
        addr: 0x0032_6d60,
        f: ee::s10::PS2Pad_Create,
    };

    pub const ANIM_OBJECT_V36: Callee = Callee {
        addr: 0x0034_1fe8,
        f: ee::s11::AnimObject_v36,
    };

    pub const DYNAMIC_EMITTER_CTOR: Callee = Callee {
        addr: 0x0037_0b60,
        f: ee::s12::DynamicEmitter_ctor,
    };

    pub const UI_STATE_CTOR: Callee = Callee {
        addr: 0x0039_e2a0,
        f: ee::s13::UIState_ctor,
    };

    pub const UI_STATE_UPDATE: Callee = Callee {
        addr: 0x0039_e510,
        f: ee::s13::UIState_Update,
    };

    pub const UI_TEXT_SET_STRING: Callee = Callee {
        addr: 0x003a_0c50,
        f: ee::s13::UIText_SetString,
    };

    pub const UI_TEXT_SCROLL_CLEAR_LINES: Callee = Callee {
        addr: 0x003a_18c0,
        f: ee::s13::cUITextScroll_ClearLines,
    };

    pub const UI_TEXT_SCROLL_SET_WIDE_STRING: Callee = Callee {
        addr: 0x003a_1d30,
        f: ee::s13::cUITextScroll_SetWideString,
    };

    pub const TWSCRIPT_CACHE_RELOCATE: Callee = Callee {
        addr: 0x003a_ccd8,
        f: ee::s13::tWScriptCache_Relocate,
    };

    pub const EA_MUTEX_LOCK: Callee = Callee {
        addr: 0x003e_5700,
        f: ee::s14::EAMutex_Lock,
    };

    pub const EA_MUTEX_UNLOCK: Callee = Callee {
        addr: 0x003e_5760,
        f: ee::s14::EAMutex_Unlock,
    };

    pub const SCE_USBKB_SET_ARRANGEMENT: Callee = Callee {
        addr: 0x0040_c7c0,
        f: ee::s15::sceUsbKbSetArrangement,
    };

    pub const SCE_USBKB_SET_READ_MODE: Callee = Callee {
        addr: 0x0040_d430,
        f: ee::s15::sceUsbKbSetReadMode,
    };

    pub const MEMSET: Callee = Callee {
        addr: 0x0041_6210,
        f: ee::s15::memset,
    };

    pub const SPRINTF: Callee = Callee {
        addr: 0x0041_7828,
        f: ee::s15::sprintf,
    };
}

/// Cross-shard call targets (the generator emits `call_indirect` for them).
pub(crate) mod table {
    pub(crate) use crate::game::ai::computer::table::DELETE;
    pub(crate) use crate::game::ai::computer::table::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::ai::computer::table::EA_MEM_FREE;
    pub(crate) use crate::game::ai::computer::table::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::ai::computer::table::SLEEP_THREAD;
    pub(crate) use crate::game::ai::computer::table::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::ai::computer::table::STR_HASH;
    pub(crate) use crate::game::ai::computer::table::WAIT_SEMA;
    pub(crate) use crate::game::ai::computer::table::WAKEUP_THREAD;
}

/// `GetHashValue32` (0x317670) and the UI object lookups, which the generator
/// emitted as cross-shard `call_indirect`s.
pub(crate) mod table_callee {
    pub(crate) use crate::game::fe::festaterewards::table_callee::AI_SET_AI_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::APP_MAN_SET_NEXT_MODULE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BENEW_PLAYER_INT_GET_CHAR_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET_INTERFACE_FV;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET_TEXT_FROM_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_SET_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BUDDY_PRODUCT_COMPARE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BUY_POPUP_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BXSTRING_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BXSTRING_DTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::CODEC_DECODE_FRAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_MODULE_BIG_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_MODULE_SMALL_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::fe::festaterewards::table_callee::MUSIC_SLOT_RELEASE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::OPERATOR_DELETE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_CHEAT_CHARS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_CONCEPT_ARTS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_GET_BASE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_POSTERS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_TOYS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_TRADING_CARDS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_VIDEOS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_REMOVE_SCRIPT_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_SCRIPT_CLEANUP;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_SET_SCRIPT_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::SECTION_TABLE_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::SSX_AUDIO_GET_RACE_MAN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::STRCPY;
    pub(crate) use crate::game::fe::festaterewards::table_callee::STRLEN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_FIND;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_GET_STRING4;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_ENGINE_GET_INTERFACE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_MENU_SET_SELECTED_BY_INDEX;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_OBJECT_PUSH;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_OPEN_FINISH_SCREEN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_ADD_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_STATE_DTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_RICH_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UPLOAD_TEXTURE_PIXELS;
}

/// Addresses the generator emitted as `call_indirect` (outside the shard's
/// own direct-call set, so no readable alias is needed here).
pub(crate) mod addr {
    pub(crate) use crate::game::fe::feasyncfile::addr::__BUILTIN_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::addr::__FIXUNSSFSI;
    pub(crate) use crate::game::fe::feasyncfile::addr::C_BX_STRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::addr::EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::addr::FONT_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::addr::STRLEN;
}

/// `void MemCardStateBase_dtor(MemCardStateBase* self, int flags)` (0x1d5428):
/// install the two intermediate vtables, clear the global and run the state
/// destructor.
pub fn mem_card_state_base_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x0047_0000;
        frame.save_ra(c, 0);
        c.g[A2][0] = c.g[A0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3680));
        c.g[V1][0] = 0x0047_0000;
        let a2 = c.g[A2][0];
        let v0 = c.g[V0][0];
        Addr(a2 as u32).off(8).sw(c, v0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4e08));
        Addr((c.g[GP][0] as u32).wrapping_sub(0x10c8)).sw(c, 0);
        let v1 = c.g[V1][0];
        Addr(a2 as u32).off(8).sw(c, v1);
        callee::UI_STATE_DTOR.call(c, 0x001d_5454)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Helpers.

/// The low word of a register.
#[inline(always)]
pub(crate) fn r(c: &Cpu, reg: usize) -> u32 {
    c.g[reg][0] as u32
}

/// `lw`/`lh`/`sw` into a named register, and `lwc1`/`swc1`. Methods on the CPU
/// so that the address may read the registers (two-phase borrows).
pub(crate) trait Ops {
    /// `lw reg, [a]`, leaving the sign-extended value in the register.
    fn lw_r_2(&mut self, reg: usize, a: u32) -> u64;
    /// `lh reg, [a]`.
    fn lh_r(&mut self, reg: usize, a: u32) -> u64;
    /// `sw reg, [a]`.
    fn sw_r_2(&mut self, a: u32, reg: usize);
    /// `reg += n` (`addiu`, sign-extended result).
    fn inc_r_2(&mut self, reg: usize, n: u32);
    /// `lwc1 f{n}, [a]`.
    fn ld_f(&mut self, n: usize, a: u32);
    /// `swc1 f{n}, [a]`.
    fn st_f(&mut self, n: usize, a: u32);
}

impl Ops for Cpu {
    #[inline(always)]
    fn lw_r_2(&mut self, reg: usize, a: u32) -> u64 {
        let v = self.lw(a);
        self.g[reg][0] = v;
        v
    }
    #[inline(always)]
    fn lh_r(&mut self, reg: usize, a: u32) -> u64 {
        let v = self.lh(a);
        self.g[reg][0] = v;
        v
    }
    #[inline(always)]
    fn sw_r_2(&mut self, a: u32, reg: usize) {
        let v = self.g[reg][0];
        self.sw(a, v);
    }
    #[inline(always)]
    fn inc_r_2(&mut self, reg: usize, n: u32) {
        let v = r(self, reg).wrapping_add(n);
        self.g[reg][0] = sx(v);
    }
    #[inline(always)]
    fn ld_f(&mut self, n: usize, a: u32) {
        self.lwc1(n, a);
    }
    #[inline(always)]
    fn st_f(&mut self, n: usize, a: u32) {
        self.swc1(n, a);
    }
}

/// The `+0x8`/`+0xC` pair a front-end state constructor writes: the class
/// vtable and the state id.
#[inline(always)]
pub(crate) fn init_state(c: &mut Cpu, this: u64, vtable: u32, id: u32) {
    c.g[V0][0] = id as u64;
    c.g[V1][0] = sx(vtable);
    c.sw_r_2((this as u32).wrapping_add(0xc), V0);
    c.sw_r_2((this as u32).wrapping_add(0x8), V1);
    c.g[V0][0] = this;
}

/// `void cFEStateTrophyRoom_Update(cFEStateTrophyRoom* self)` (0x1D4760):
/// load the trophy textures while any are pending (count > 0), then run the
/// UI state update.
pub fn fe_state_trophy_room_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let pending = c.lw_r_2(V0, r(c, S0).wrapping_add(0x5c));
        if (pending as i64) > 0 {
            c.cycles += 2;
            callee::TROPHY_ROOM_LOAD_TROPHY_TEXTURES.call(c, 0x001d_4784)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        callee::UI_STATE_UPDATE.call(c, 0x001d_478c)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateRelogin* cFEStateRelogin_ctor(cFEStateRelogin*)` (0x1D5038): the
/// login-state constructor, vtable `0x468050` and state id 0x34.
pub fn fe_state_relogin_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        callee::FE_STATE_LOGIN_CTOR.call(c, 0x001d_504c)?;
        init_state(c, this, 0x0046_8050, 0x34);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateRelogin_OnChildEvent(self, child, event)` (0x1D5240): the
/// disconnect popup (id 0x102) handles the event itself, every other child
/// goes to the login state.
pub fn fe_state_relogin_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x102;
        frame.save_ra(c, 0);
        let event = Addr(r(c, A1).wrapping_add(0x18)).lw(c);
        c.g[V1][0] = event;
        if event == c.g[V0][0] {
            c.cycles += 4;
            callee::NET_FE_STATE_ON_DISCONNECTED_POPUP_EVENT.call(c, 0x001d_5260)?;
        } else {
            c.cycles += 3;
            callee::FE_STATE_LOGIN_ON_CHILD_EVENT.call(c, 0x001d_5270)?;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateRelogin_OnWidgetCreated(cFEStateRelogin* s, UIWidget* w)`
/// (0x1d50e0): when the created widget is the retry button, give it the
/// "please wait" label.
pub fn cfe_state_relogin_on_widget_created(c: &mut Cpu) {
    c.cycles += 11;
    let frame = Frame::open(c, 0x30);
    c.g[A0][0] = sx(0x0046_7790);
    frame.save(c, S1, 0x10);
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A1][0];
    frame.save(c, S0, 0x20);
    c.g[S0][0] = Addr(p(c, S1, 0x38)).lw(c);
    if call_ptr(c, table::STR_HASH, 0x001d_5104).is_err() {
        return;
    }
    let same = c.g[S0][0] == c.g[V0][0];
    frame.restore(c, S0, 0x20);
    if same {
        c.cycles += 7;
        c.g[A0][0] = sx(0x0046_1cc0);
        if call_ptr(c, table::STR_HASH, 0x001d_5118).is_err() {
            return;
        }
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        if call_ptr(c, 0x003a_1000, 0x001d_5124).is_err() {
            return;
        }
        frame.restore(c, S0, 0x20);
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// The list-widget handlers that answer 0x100 (handled) for the scroll keys
/// 8 and 9 and 0x101 (ignored) otherwise, keeping the original's compare
/// chain (`key < 10`, then `key < 8`) and its cycle charges.
pub(crate) fn handle_list_key(c: &mut Cpu) {
    c.cycles += 3;
    let key = abi::arg64(c, 2);
    c.g[V1][0] = (key < 0xa) as u64;
    c.g[V0][0] = 0x101;
    if c.g[V1][0] != 0 {
        c.cycles += 4;
        c.g[A0][0] = (key < 0x8) as u64;
        c.g[V1][0] = 0x101;
        c.g[V0][0] = if key < 0x8 { 0x101 } else { 0x100 };
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `int cFEStateRewardsRoom_OnWidgetNotify(State*, void*, int key)`
/// (0x001d4f68).
pub fn cfe_state_rewards_room_on_widget_notify(c: &mut Cpu) {
    handle_list_key(c);
}

/// `int MemCardStateBase_OnWidgetNotify(State*, void*, int key)`
/// (0x001d5460).
pub fn mem_card_state_base_on_widget_notify(c: &mut Cpu) {
    handle_list_key(c);
}

/// The handlers that accept the keys 6, 8 and 9 (0x100) and ignore the rest
/// (0x101). The original tests 6, then `< 6`, `< 10`, `< 8`, so the cycle
/// charge depends on where the key leaves the chain.
pub(crate) fn handle_key_6_8_9(c: &mut Cpu) {
    let key = abi::arg64(c, 2);
    c.cycles += 3;
    c.g[V0][0] = 6;
    let is6 = key == 6;
    c.g[V0][0] = (key < 6) as u64;
    let accept = if is6 {
        true
    } else {
        c.cycles += 2;
        let below6 = c.g[V0][0] != 0;
        c.g[V0][0] = (key < 0xa) as u64;
        if below6 {
            false
        } else {
            c.cycles += 2;
            let above9 = c.g[V0][0] == 0;
            c.g[V0][0] = (key < 8) as u64;
            if above9 {
                false
            } else {
                c.cycles += 2;
                c.g[V0][0] == 0
            }
        }
    };
    c.cycles += 2;
    c.g[V0][0] = if accept { 0x100 } else { 0x101 };
    abi::ret(c);
}

/// `int cFEStateTrophyRoom_OnWidgetNotify(State*, void*, int key)`
/// (0x001d4660).
pub fn cfe_state_trophy_room_on_widget_notify(c: &mut Cpu) {
    handle_key_6_8_9(c);
}

/// `cFEStateTrophyRoom_Init(cFEStateTrophyRoom* self)` (0x1d4368).
pub fn c_fe_state_trophy_room_init(c: &mut Cpu) {
    guest(c, |c| {
        fe_state_screen_init(c, 0x0046_76c8, [0x001d_438c, 0x001d_43a0, 0x001d_43b8])
    })
}

/// `cFEStateRelogin_Init(cFEStateRelogin* self)` (0x1d5078).
pub fn c_fe_state_relogin_init(c: &mut Cpu) {
    guest(c, |c| {
        fe_state_screen_init(c, 0x0046_7778, [0x001d_509c, 0x001d_50b0, 0x001d_50c8])
    })
}

/// `lhu off(reg)`.
#[inline]
pub(crate) fn lhu_r(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    Addr(low(c, reg).wrapping_add(off)).lhu(c)
}

/// `sh off(reg)`.
#[inline]
pub(crate) fn sh_r(c: &mut Cpu, reg: usize, off: u32, v: u64) {
    let a = low(c, reg).wrapping_add(off);
    c.sh(a, v);
}

/// `func_001D53B0` (0x1d53b0): the memory-card state constructor. Runs the UI
/// state constructor (`0x39e2a0`), installs vtables `0x474e08`/`0x46c980`,
/// clears the callback/word fields at `+0x48`..`+0x98` and calls
/// `MemCardStateBase_InstallCallbacks`. Signature: `void ctor(this, a, b)`.
pub fn mem_card_state_base_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x001d_53cc);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0039_e2a0, 0x001d_53cc)?;
        c.g[V1][0] = 0x47_0000;
        sw_r(c, S0, 0x98, 0);
        c.g[V1][0] = sx(low(c, V1).wrapping_add(0x4e08));
        sw_r(c, S0, 0x48, 0);
        sw_r(c, S0, 0x8, c.g[V1][0]);
        c.g[A2][0] = 0x4a_0000;
        sw_r(c, S0, 0x4c, 0);
        c.g[V0][0] = 0x47_0000;
        sw_r(c, S0, 0x54, 0);
        c.g[V0][0] = sx(low(c, V0).wrapping_sub(0x3680));
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[V1][0] = lhu_r(c, A2, 0x1390);
        sw_r(c, S0, 0x8, c.g[V0][0]);
        c.g[RA][0] = sx(0x001d_540c);
        sh_r(c, S0, 0x58, c.g[V1][0]);
        callee::MEMCARD_INSTALL_CALLBACKS.call(c, 0x001d_540c)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateRewardsRoom_Init(self)` (0x1d4c60): create the rewards-room
/// screen by name hash, play its first frame and tell the audio system the FE
/// state is 6.
pub fn c_fe_state_rewards_room_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x30));
        c.sq(16, sp_2(c).wrapping_add(0x20));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp_2(c), c.g[RA][0]);
        c.sq(17, sp_2(c).wrapping_add(0x10));
        c.g[A0][0] = 0x46_0000;
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0x7738));
        c.g[S1][0] = lw_r(c, S0, 0x10);
        call_ptr(c, callee::STR_HASH, 0x001d_4c84)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S0][0];
        c.g[A3][0] = 0;
        call_ptr(c, callee::C_UI_ENGINE_ADD_SCREEN_BY_HASH_NAME, 0x001d_4c98)?;
        let failed = c.g[V0][0] == 0;
        sw_r(c, S0, 0x40, c.g[V0][0]);
        if !failed {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, callee::C_UI_SCREEN_PLAY_FRAME, 0x001d_4cb0)?;
        }
        c.cycles += 10;
        call_ptr(c, callee::SSX_AUDIO_SYSTEM_GET_INSTANCE, 0x001d_4cb8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x6;
        call_ptr(c, callee::FUNC_0028F140, 0x001d_4cc4)?;
        c.lq(16, sp_2(c).wrapping_add(0x20));
        c.lq(17, sp_2(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x30));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Front-end states and screens.

/// `MemCardStateBase_ctor_2(self, index, a2, a3)` (0x1d5330): the second
/// constructor entry: install the vtable, clear the state fields, copy the
/// memory-card slot count and install the callbacks.
pub fn memcardstatebase_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x20);
        frame.save(c, 18, 0x10);
        c.g[S1][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A2][0];
        callee::UI_STATE_CTOR_2.call(c, 0x001d_5354)?;
        c.g[V1][0] = 0x470000;
        c.sw(p_3(r32(c, S0), 0x98), c.g[S1][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3680));
        c.sw(p_3(r32(c, S0), 0x48), 0);
        c.sw(p_3(r32(c, S0), 8), c.g[V1][0]);
        c.g[A2][0] = 0x4a0000;
        c.sw(p_3(r32(c, S0), 0x4c), 0);
        c.g[V0][0] = 0x470000;
        c.sw(p_3(r32(c, S0), 0x54), 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x3680));
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[V1][0] = Addr(p_3(c.g[A2][0] as u32, 0x1390)).lhu(c);
        c.sw(p_3(r32(c, S0), 8), c.g[V0][0]);
        c.sh(p_3(r32(c, S0), 0x58), c.g[V1][0]);
        callee::MEMCARD_STATE_BASE_INSTALL_CALLBACKS.call(c, 0x001d_5394)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, 17, 0x20);
        frame.restore(c, 16, 0x30);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateTrophyRoom_ShowTrophyInfo(cFEStateTrophyRoom* self, int
/// which)` (0x1d4890): select the trophy list entry (`self+0x4c` -> list,
/// `+8` -> item) and refresh the event stat text.
pub fn c_fe_state_trophy_room_show_trophy_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        open(c, 0x30);
        let sp0 = spv(c);
        c.sq(16, at(sp0, 0x20));
        c.sq(17, at(sp0, 0x10));
        c.g[S0][0] = c.g[A0][0];
        c.sd(sp0, c.g[RA][0]);
        let this = c.g[S0][0];
        c.g[A2][0] = lw_at(c, this, 0x4c);
        let list = c.g[A2][0];
        let empty = c.g[A2][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !empty {
            c.cycles += 2;
            let which = c.g[S1][0] != 0;
            c.g[V0][0] = lw_at(c, list, 8);
            let item = c.g[V0][0];
            if which {
                c.cycles += 12;
                c.g[A1][0] = 1;
                c.g[A0][0] = lh_at(c, item, 0x48);
                c.g[V1][0] = lw_at(c, item, 0x4c);
                let func = c.g[V1][0] as u32;
                c.g[A0][0] = sx(at(list as u32, c.g[A0][0] as u32));
                call_ptr(c, func, 0x001d_48e8)?;
                c.g[T0][0] = lw_at(c, this, 0x58);
                c.g[T1][0] = sx(at(c.g[S1][0] as u32, 0xffff_ffff));
                c.g[A0][0] = lw_at(c, this, 0x4c);
                c.g[A1][0] = lb_at(c, this, 0x44);
                c.g[A2][0] = lw_at(c, this, 0x50);
                c.g[A3][0] = lw_at(c, this, 0x54);
                callee::UI_TEXT_SET_EVENT_STAT.call(c, 0x001d_4904)?;
                c.cycles += 1;
                c.lq(16, at(sp0, 0x20));
            } else {
                c.cycles += 7;
                c.g[A1][0] = 0;
                c.g[A0][0] = lh_at(c, item, 0x48);
                c.g[V1][0] = lw_at(c, item, 0x4c);
                let func = c.g[V1][0] as u32;
                c.g[A0][0] = sx(at(list as u32, c.g[A0][0] as u32));
                call_ptr(c, func, 0x001d_48cc)?;
                c.lq(16, at(sp0, 0x20));
            }
        } else {
            c.cycles += 1;
            c.lq(16, at(sp0, 0x20));
        }
        c.cycles += 4;
        c.lq(17, at(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x30));
        c.pc = ra;
        Ok(())
    })
}

/// `cFEStateRewardsRoom* cFEStateRewardsRoom_ctor(self, idx, char, a3)`
/// (0x1d4bc8): build the rewards-room state, remember the player's character
/// id and return the object.
pub fn cfe_state_rewards_room_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = sx((c.g[A2][0] as u32) << 24);
        c.g[S2][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = sx(((c.g[S1][0] as u32) as i32 >> 24) as u32);
        call_ptr(c, table_callee::FE_STATE_CTOR, 0x001d_4bf0)?;
        c.g[V0][0] = 0x2e;
        c.g[V1][0] = 0x470000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6a18));
        Addr(c.g[S0][0] as u32).off(0xc).sw(c, c.g[V0][0]);
        Addr(c.g[S0][0] as u32).off(0x8).sw(c, c.g[V1][0]);
        c.g[A1][0] = c.g[S1][0];
        Addr(c.g[S0][0] as u32).off(0x44).sb(c, c.g[S1][0]);
        c.g[V0][0] = lw_2(c, S0, 0x10);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        call_ptr(c, table_callee::BE_GET_TEXT_FROM_ID, 0x001d_4c18)?;
        Addr(c.g[S0][0] as u32).off(0x4c).sw(c, c.g[S2][0]);
        Addr(c.g[S0][0] as u32).off(0x15).sb(c, c.g[V0][0]);
        call_ptr(c, table_callee::BE_GET, 0x001d_4c24)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, table_callee::BE_GET_INTERFACE_FV, 0x001d_4c30)?;
        c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x44).lb(c);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, table_callee::BENEW_PLAYER_INT_GET_CHAR_ID, 0x001d_4c3c)?;
        Addr(c.g[S0][0] as u32).off(0x48).sw(c, c.g[V0][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x20);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateRewardsRoom_SetCountText(self, index, text)` (0x1d4f90):
/// format the collected/total count into a buffer and set it on the label.
pub fn cfe_rewards_room_set_count_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 41;
        let frame = Frame::open(c, 0x160);
        frame.save(c, S1, 0x140);
        frame.save(c, S3, 0x120);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S4, 0x110);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S0, 0x150);
        frame.save(c, S2, 0x130);
        frame.save_ra(c, 0x100);
        c.g[S4][0] = c.g[A2][0];
        call_ptr(c, 0x0014_dc80, 0x001d4fbc)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xd;
        call_ptr(c, 0x0014_dd58, 0x001d4fc8)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0015_7210, 0x001d4fd8)?;
        c.g[A1][0] = Addr(p_4(r32(c, S1), 0x44)).lb(c);
        c.g[S2][0] = c.g[V0][0];
        c.g[A2][0] = Addr(p_4(r32(c, S1), 0x48)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S3][0];
        call_ptr(c, 0x0015_7518, 0x001d4ff0)?;
        c.g[A1][0] = sx(0x004a_2020);
        c.g[A3][0] = c.g[S2][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = c.g[V0][0];
        call_ptr(c, 0x0041_7828, 0x001d5008)?;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = c.g[SP][0];
        call_ptr(c, 0x003a_0c50, 0x001d5014)?;
        frame.restore(c, S0, 0x150);
        frame.restore(c, S1, 0x140);
        frame.restore(c, S2, 0x130);
        frame.restore(c, S3, 0x120);
        frame.restore(c, S4, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateTrophyRoom_ReleaseTrophyImages(self)` (0x1d4b20): for the
/// five trophy images, unlink the image object and free it.
pub fn cfe_state_trophy_room_release_trophy_images(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x60);
        c.cycles += 14;
        c.g[V1][0] = lw(c, gp(c, -0x848));
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = 0;
        frame.save(c, S1, 0x40);
        c.g[S0][0] = sx(at_3(c.g[S2][0], 0x74));
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, at_3(c.g[V1][0], 0x7c));
        c.g[S4][0] = sx(at_3(c.g[V0][0], 0x1a70));
        loop {
            {
                c.cycles += 3;
                c.g[A1][0] = lw(c, at_3(c.g[S0][0], 0xffff_ffec));
                let t = (c.g[A1][0] as i64) < 0;
                c.g[A2][0] = lw(c, gp(c, -0x854));
                if !t {
                    c.cycles += 5;
                    c.g[V0][0] = lw(c, at_3(c.g[A2][0], 0x10d8));
                    c.g[A0][0] = lh(c, at_3(c.g[V0][0], 0x190));
                    c.g[V1][0] = lw(c, at_3(c.g[V0][0], 0x194));
                    let target = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001d_4b78);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x001d_4b78)?;
                }
            }
            {
                c.cycles += 4;
                c.g[S1][0] = sx(0xffff_ffff);
                c.g[A2][0] = lw(c, c.g[S0][0] as u32);
                let t = (c.g[A2][0] as i64) < 0;
                Addr(at_3(c.g[S0][0], 0xffff_ffec)).sw(c, c.g[S1][0]);
                if !t {
                    c.cycles += 3;
                    c.g[A1][0] = lw(c, at_3(c.g[S2][0], 0x50));
                    c.g[RA][0] = sx(0x001d_4b94);
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, 0x0019_dc20, 0x001d_4b94)?;
                }
            }
            c.cycles += 5;
            Addr(c.g[S0][0] as u32).sw(c, c.g[S1][0]);
            c.g[S3][0] = sx(at_3(c.g[S3][0], 1));
            c.g[V0][0] = ((c.g[S3][0] as i64) < 5i64) as u64;
            let t = c.g[V0][0] != 0;
            c.g[S0][0] = sx(at_3(c.g[S0][0], 4));
            if !t {
                c.cycles += 8;
                frame.restore(c, S0, 0x50);
                frame.restore(c, S1, 0x40);
                frame.restore(c, S2, 0x30);
                frame.restore(c, S3, 0x20);
                frame.restore(c, S4, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            if crate::events::event_due(c) {
                crate::events::deliver_event_here(c, 0x001d_4b58);
            }
        }
    })
}

/// `void cFEStateRelogin_v33(self, item)` (0x1d5280): widget event 0xf/0x14
/// tears down the relogin UI, marks the current UI state (0x80) and enters the
/// "relogin failed" state; every other event goes to the base handler.
pub fn cfe_relogin_v33(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        let msg = c.g[A2][0];
        let big = if msg == 0xf {
            true
        } else {
            c.cycles += 3;
            msg == 0x14
        };
        if !big {
            c.cycles += 3;
            c.g[A0][0] = this;
            call_ptr(c, 0x001a_97b8, 0x001d_5320)?;
        } else {
            c.cycles += 29;
            c.g[A0][0] = this;
            call_ptr(c, 0x001a_97b8, 0x001d_52ac)?;
            c.g[A0][0] = Addr(p_5(c.g[S0][0] as u32, 0x10)).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, 0x0039_f698, 0x001d_52b8)?;
            c.g[A0][0] = Addr(p_5(c.g[S0][0] as u32, 0x10)).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, 0x0039_fa88, 0x001d_52c4)?;
            c.g[V1][0] = Addr(p_5(c.g[V0][0] as u32, 0x1c)).lw(c);
            c.g[V1][0] = c.g[V1][0] | 0x80;
            c.sw(p_5(c.g[V0][0] as u32, 0x1c), c.g[V1][0]);
            call_ptr(c, 0x0025_91b8, 0x001d_52d4)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0026_36f0, 0x001d_52e0)?;
            c.g[A1][0] = 0x460000;
            c.g[A3][0] = 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xbf0));
            c.g[A0][0] = 0x91c;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x001d_52f8)?;
            c.g[A1][0] = Addr(p_5(c.g[S0][0] as u32, 0x10)).lw(c);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x001b_3f78, 0x001d_5304)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x001a_8a40, 0x001d_5310)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEStateRewardsRoom_OnWidgetAction` (0x001d4ea8).
//
// `int cFEStateRewardsRoom::OnWidgetAction(self, widget, action)`: route the
// rewards-room buttons.

pub fn fe_state_rewards_room_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        'b_1d4f5c: {
            'b_1d4f58: {
                c.cycles += 6;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
                c.g[V1][0] = c.g[A1][0];
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                let t = c.g[V1][0] == 0u64;
                c.g[S0][0] = c.g[A0][0];
                if t {
                    break 'b_1d4f58;
                } else {
                    'b_1d4f24: {
                        'b_1d4ee8: {
                            c.cycles += 3;
                            c.g[V0][0] = 0x5;
                            let t = c.g[A2][0] == c.g[V0][0];
                            c.g[V0][0] = (c.g[A2][0] < 0x6u64) as u64;
                            if t {
                                break 'b_1d4ee8;
                            } else {
                                c.cycles += 2;
                                if c.g[V0][0] != 0u64 {
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                                    break 'b_1d4f5c;
                                } else {
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x6;
                                    if c.g[A2][0] == c.g[V0][0] {
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                                        break 'b_1d4f24;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                                        if t {
                                            break 'b_1d4f5c;
                                        } else {
                                            break 'b_1d4ee8;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 15;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                        c.g[A3][0] = c.lw((c.g[V0][0] as u32));
                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = vslot(c, c.g[V1][0], 0x20, V0);
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001d4f0c);
                        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, t, 0x001d4f0c)?;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        c.g[A1][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001d4f1c);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                        call_ptr(c, 0x0039f400, 0x001d4f1c)?;
                        let t = true;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                        if t {
                            break 'b_1d4f5c;
                        } else {
                            break 'b_1d4f24;
                        }
                    }
                    c.cycles += 10;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.g[A3][0] = c.lw((c.g[V0][0] as u32));
                    c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = vslot(c, c.g[V1][0], 0x28, V0);
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x001d4f44);
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                    call_ptr(c, t, 0x001d4f44)?;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = c.g[V0][0];
                    if t {
                        break 'b_1d4f58;
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        c.g[RA][0] = sx(0x001d4f58);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                        call_ptr(c, 0x0039f400, 0x001d4f58)?;
                        break 'b_1d4f58;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_1d4f5c;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());

        Ok(())
    })
}

/// `cFEStateTrophyRoom_OnWidgetAction(this, item, msg)` (0x1d4698): show a
/// trophy's image and info, or switch the trophy room into the item's view.
pub fn cfe_state_trophy_room_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let this = abi::arg(c, 0);
        let item = abi::arg(c, 1);
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = abi::arg64(c, 1);
        frame.save_ra(c, 0);
        let no_item = c.g[S0][0] == 0;
        c.g[S1][0] = abi::arg64(c, 0);

        // The original's exits differ: 'b_1d4748 adds one cycle before the
        // epilogue, 'b_1d474c does not.
        let mut extra = true;
        if no_item {
            // straight to 'b_1d4748
        } else {
            c.cycles += 3;
            c.g[V0][0] = 1;
            let is_one = c.g[A2][0] == c.g[V0][0];
            c.g[V0][0] = 6;
            if is_one {
                // case 1: set the trophy and refresh the panel ('b_1d4748).
                c.cycles += 8;
                c.g[V0][0] = Addr(item).off(0x18).lw(c);
                c.g[A0][0] = c.g[S1][0];
                Addr(this).off(0x9c).sw(c, c.g[V0][0]);
                c.g[A1][0] = Addr(item).off(0x18).lw(c);
                callee::C_FE_STATE_TROPHY_ROOM_SHOW_TROPHY_IMAGE.call(c, 0x001d_473c)?;
                c.g[A1][0] = Addr(item).off(0x18).lw(c);
                c.g[A0][0] = c.g[S1][0];
                callee::C_FE_STATE_TROPHY_ROOM_SHOW_TROPHY_INFO.call(c, 0x001d_4748)?;
            } else {
                c.cycles += 2;
                if c.g[A2][0] != c.g[V0][0] {
                    extra = false;
                } else {
                    // case 6: ask the room's menu to show the item.
                    c.cycles += 3;
                    c.g[A2][0] = Addr(this).off(0x48).lw(c);
                    let none = c.g[A2][0] == 0;
                    c.g[A1][0] = 0;
                    if !none {
                        c.cycles += 5;
                        c.g[V0][0] = Addr(c.g[A2][0] as u32).off(8).lw(c);
                        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x48).lh(c);
                        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x4c).lw(c);
                        let target = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, target, 0x001d_46e8)?;
                    }
                    c.cycles += 11;
                    c.g[V0][0] = Addr(this).off(0x10).lw(c);
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = Addr(this).off(0x54).lw(c);
                    c.g[A3][0] = Addr(c.g[V0][0] as u32).lw(c);
                    c.g[V1][0] = Addr(c.g[A3][0] as u32).off(4).lw(c);
                    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x28).lh(c);
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x2c).lw(c);
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x001d_470c)?;
                    let no_target = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[V0][0];
                    if no_target {
                        // 'b_1d4748
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = Addr(this).off(0x10).lw(c);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                        callee::UI_STATE_MGR_SWITCH_STATE.call(c, 0x001d_4720)?;
                        extra = false;
                    }
                }
            }
        }

        if extra {
            c.cycles += 1;
        }
        frame.restore(c, 16, 0x20);
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// MemCardStateBase_Shutdown (0x00001d58b8).

/// `MemCardStateBase_Shutdown`: shut the memory-card state base down: close the dialogs and free the state.
pub fn mem_card_state_base_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        'b_1d591c: {
            c.cycles += 14;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[S1][0] = c.g[A0][0];
            call_ptr(c, 0x0014dc80, 0x001d58d0)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x5;
            call_ptr(c, 0x0014dd58, 0x001d58dc)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1bc));
            c.g[V0][0] = 0x1;
            let t = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = 0x2;
            if t {
                break 'b_1d591c;
            } else {
                c.cycles += 3;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x228));
                let t = c.g[V1][0] != c.g[V0][0];

                if t {
                    break 'b_1d591c;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1b4));
                    let t = c.g[V0][0] != 0u64;

                    if t {
                        break 'b_1d591c;
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x00152728, 0x001d5910)?;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x1d4), 0u64);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x1d0), 0u64);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x228), 0u64);
                        break 'b_1d591c;
                    }
                }
            }
        }
        'b_1d5958: {
            c.cycles += 8;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x00152700, 0x001d5924)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            call_ptr(c, 0x00227f80, 0x001d592c)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x434));
            if c.g[V0][0] == 0u64 {
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x228));
                break 'b_1d5958;
            } else {
                c.cycles += 7;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002410a0, 0x001d5944)?;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0023d5e8, 0x001d594c)?;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0023c860, 0x001d5954)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x228));
                break 'b_1d5958;
            }
        }
        'b_1d5980: {
            c.cycles += 3;
            c.g[V0][0] = 0x1;
            let t = c.g[V1][0] != c.g[V0][0];

            if t {
                break 'b_1d5980;
            } else {
                'b_1d5978: {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x1d4));
                    let t = c.g[A0][0] == 0u64;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x228), 0u64);
                    if t {
                        break 'b_1d5978;
                    } else {
                        c.cycles += 2;
                        call_ptr(c, 0x00317e98, 0x001d5978)?;
                        break 'b_1d5978;
                    }
                }
                c.cycles += 2;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x1d4), 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x1d0), 0u64);
                break 'b_1d5980;
            }
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0020a430, 0x001d5988)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::compare;
    use crate::difftest::elf_image;
    use crate::difftest::fakes;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fakes::*;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::scramble;
    use crate::difftest::seed_count;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::SP;
    pub(crate) use crate::game::bx::bxstring::tests::heap_init as heap_init_3;
    pub(crate) use crate::game::dirtysock::tags2::tests::alloc_machine;
    pub(crate) use crate::game::dirtysock::tags2::tests::heap_init_2 as heap_init;
    pub(crate) use crate::game::dirtysock::tags2::tests::pool_allocator;
    pub(crate) use crate::game::dirtysock::tags2::tests::run_2 as run;
    pub(crate) use crate::game::dirtysock::tags2::tests::zero;
    pub(crate) use crate::game::dirtysock::tags2::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::fake_small;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_10 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_11 as globals_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_12 as globals_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_10 as heap_init_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_12 as heap_init_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_10 as pool_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_8 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_9 as pool_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_10 as pool_allocator_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_8 as run_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_9 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::ui;
    pub(crate) use crate::game::fe::feasyncfile::tests::wb;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_6 as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_9 as BE_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::IFACE;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ as OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_3 as OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3_2 as OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ4_2 as OBJ4;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_3 as OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDER;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDERS_2 as RIDERS;
    pub(crate) use crate::game::fe::feasyncfile::tests::RMAN;
    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_5 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_7 as WORLD_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_4 as W;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4 as base_2;
    pub(crate) use crate::game::fe::festaterewards::tests::globals_3 as globals;
    pub(crate) use crate::game::fe::festaterewards::tests::heap_block;
    pub(crate) use crate::game::fe::festaterewards::tests::heap_init_3 as heap_init_4;
    pub(crate) use crate::game::fe::festaterewards::tests::pool_4 as pool;
    pub(crate) use crate::game::fe::festaterewards::tests::slot_2 as slot;
    pub(crate) use crate::game::fe::festaterewards::tests::w16;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY_PTR;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_3;
    pub(crate) use crate::game::main::gamestate::tests::base_3 as base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::run as run_2;
    pub(crate) use crate::game::world::worldview::tests::SEEDS_5 as SEEDS_2;
    use ssx3_ee as ee;

    #[test]
    fn mem_card_state_base_dtor_matches() {
        check(
            ee::s04::f_001d5428__generated,
            mem_card_state_base_dtor,
            60,
            &|c, seed| {
                let _r = alloc_machine(c, seed, 0x1d54_28);
                let obj = heap_block(c, 0x300);
                zero(c, obj, 0x300);
                c.g[A0][0] = sx(obj);
                c.g[A1][0] = sx((seed & 1) as u32);
            },
        );
    }

    pub(crate) const SEEDS: u32 = 100;

    /// A 32-bit word of test data.
    pub(crate) fn w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// The BELibrary singleton (`gp - 0x1f04`) and a zeroed dummy object it
    /// points at: without it a library lookup tries to load from disc forever.
    pub(crate) fn belib(c: &mut Cpu) {
        w(c, GAME_GP - 0x1f04, DATA + 0x8000);
    }

    #[test]
    fn fe_state_trophy_room_update_matches() {
        check(
            ssx3_ee::s04::f_001d4760__generated,
            fe_state_trophy_room_update,
            40,
            &|c, seed| {
                machine(c, seed, 0x1d47_60, 2);
                belib(c);
                c.g[A0][0] = DATA as u64;
                let pending = if seed % 2 == 0 { 0 } else { 2 };
                w(c, DATA + 0x5c, pending);
            },
        );
    }

    #[test]
    fn fe_state_relogin_ctor_matches() {
        check(
            ssx3_ee::s04::f_001d5038__generated,
            fe_state_relogin_ctor,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x1d50_38, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn fe_state_relogin_on_child_event_matches() {
        check(
            ssx3_ee::s04::f_001d5240__generated,
            fe_state_relogin_on_child_event,
            60,
            &|c, seed| {
                machine(c, seed, 0x1d52_40, 2);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
                let event = if seed % 2 == 0 { 0x102 } else { 0x100 };
                w(c, DATA + 0x300 + 0x18, event);
            },
        );
    }

    #[test]
    fn cfe_state_relogin_on_widget_created_matches() {
        check(
            ssx3_ee::s04::f_001d50e0__generated,
            cfe_state_relogin_on_widget_created,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1d50e0, 3);
                let this = DATA + 0x400;
                let widget = DATA + 0x500;
                let target = run_2(c, 0x0031_7670, ssx3_ee::s10::StrHash, &[0x0046_7790]);
                c.bus.write32(
                    widget + 0x38,
                    if seed % 2 == 0 {
                        target as u32
                    } else {
                        r.next_u32()
                    },
                );
                let vt = vtable(c, &mut r, &[0x28]);
                c.bus.write32(widget + 8, vt);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = widget as u64;
            },
        );
    }

    #[test]
    fn cfe_state_trophy_room_on_widget_notify_matches() {
        check(
            ssx3_ee::s04::f_001d4660__generated,
            cfe_state_trophy_room_on_widget_notify,
            SEEDS_2,
            &|c, seed| {
                let _ = machine(c, seed, 0x1d4660, 0);
                c.g[A2][0] = (seed % 13) as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_trophy_room_init_matches() {
        check(
            ssx3_ee::s04::f_001d4368__generated,
            c_fe_state_trophy_room_init,
            40,
            &|c, seed| {
                let _r = setup(c, seed, 0x1d4368);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(DATA + 0x40, seed);
            },
        );
    }

    #[test]
    fn c_fe_state_relogin_init_matches() {
        check(
            ssx3_ee::s04::f_001d5078__generated,
            c_fe_state_relogin_init,
            40,
            &|c, seed| {
                let _r = setup(c, seed, 0x1d5078);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(DATA + 0x40, seed);
            },
        );
    }

    #[test]
    fn mem_card_state_base_ctor_matches() {
        check(
            ee::s04::f_001d53b0__generated,
            mem_card_state_base_ctor,
            16,
            &|c, seed| {
                machine(c, seed, 0x1d53b0, 8);
                be_library(c);
                heap_init_2(c);
                world(c);
                c.g[A0][0] = (DATA + 0x2_0000) as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                c.g[A2][0] = (DATA + 0xc00) as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_rewards_room_init_matches() {
        check(
            ee::s04::f_001d4c60__generated,
            c_fe_state_rewards_room_init,
            SEEDS_3,
            &|c, seed| {
                let _r = machine(c, seed, 0x1d4c60, 4);
                be_library(c);
                heap_init_3(c);
                let obj = DATA;
                c.g[A0][0] = obj as u64;
                w(c, obj + 0x10, DATA + 0x1000);
                w(c, DATA + 0x1000, 0);
            },
        );
    }

    #[test]
    fn memcardstatebase_ctor_2_matches() {
        check(
            ee::s04::f_001d5330__generated,
            memcardstatebase_ctor_2,
            SEEDS,
            &|c, seed| {
                base(c, seed, 0x5330, 0);
                c.bus.write32(GAME_GP - 0x47c, DATA + 0x6000);
                let s = DATA + 0x100;
                c.g[A0][0] = sx(s);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = sx(DATA + 0x700);
                c.g[A3][0] = (seed / 4) as u64;
                c.bus.write32(s + 0x1bc, if seed % 3 == 0 { 2 } else { 0 });
                c.bus.write32(0x004a_1390, (seed * 3) as u32);
            },
        );
    }

    #[test]
    fn cfe_state_rewards_room_ctor_matches() {
        check(
            ee::s04::f_001d4bc8__generated,
            cfe_state_rewards_room_ctor,
            40,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x1d4bc8, 3);
                let obj = DATA + 0x1000;
                let engine = DATA + 0x2000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = (seed % 8) as u64;
                c.g[A3][0] = r.next_u32() as u64;
                w(c, obj + 0x10, engine);
                w(c, engine, DATA + 0x3000);
                w(
                    c,
                    0x0053_08a8,
                    if seed % 2 == 0 { 0 } else { DATA + 0x4000 },
                );
                w(c, GAME_GP.wrapping_sub(0x1efc), 0);
            },
        );
    }

    #[test]
    fn cfe_rewards_room_set_count_text_matches() {
        check(
            ee::s04::f_001d4f90__generated,
            cfe_rewards_room_set_count_text,
            6,
            &|c, seed| {
                let _ = base_3(c, seed, 0x1d4f90, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = OBJ2 as u64;
                w(c, OBJ + 0x44, seed % 10);
                w(c, OBJ + 0x48, seed % 5);
                w(c, 0x004a_2020, 0);
            },
        );
    }

    // ----------------------------------------------------- trophy room images

    #[test]
    fn cfe_state_trophy_room_release_trophy_images_matches() {
        check(
            ee::s04::f_001d4b20__generated,
            cfe_state_trophy_room_release_trophy_images,
            SEEDS_4,
            &|c, seed| {
                let _ = base_4(c, seed, 0x1d4b_20, 8);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                // The five image handles: -1 or a live image id.
                for i in 0..6u32 {
                    w(
                        c,
                        obj + 0x74 + 4 * i,
                        if (seed as u32 + i) % 3 == 0 {
                            0xffff_ffff
                        } else {
                            i + 1
                        },
                    );
                }
                w(c, obj + 0x50, obj + 0x2000);
                // The world's rider array (empty) and the interface vtable the
                // first call goes through.
                w(c, W, 0);
                w(c, IFACE + 0x10d8, obj + 0x2100);
                slot_2(c, obj + 0x2100, 0x190, 0);
            },
        );
    }

    #[test]
    fn cfe_relogin_v33_matches() {
        check(
            ee::s04::f_001d5280__generated,
            cfe_relogin_v33,
            8,
            &|c, seed| {
                let _ = base_5(c, seed, 0x001d_5280, 6);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                c.g[A2][0] = match seed % 4 {
                    0 => 0xf,
                    1 => 0x14,
                    _ => (seed % 32) as u64,
                };
                w(c, OBJ + 0x10, OBJ3);
                w(c, OBJ3 + 0x18, OBJ4);
                w(c, OBJ3 + 0x40, OBJ4);
            },
        );
    }

    #[test]
    fn fe_state_rewards_room_on_widget_action_matches() {
        strict(
            ee::s04::f_001d4ea8__generated,
            fe_state_rewards_room_on_widget_action,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1d4e, 6);
                let this = DATA;
                let widget = DATA + 0x1000;
                c.bus.write32(this + 0x10, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000, DATA + 0x3000);
                c.bus.write32(DATA + 0x3000 + 4, DATA + 0x4000);
                c.bus.write32(DATA + 0x4000 + 4, DATA + 0x5000);
                c.bus.write16(DATA + 0x5000 + 0x20, 0);
                c.bus.write32(DATA + 0x5000 + 0x24, FAKE);
                c.bus.write16(DATA + 0x5000 + 0x28, 0);
                c.bus.write32(DATA + 0x5000 + 0x2c, FAKE);
                c.bus.write32(widget + 0x18, r.next_u32());
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 5 == 0 { 0 } else { sx(widget) };
                c.g[A2][0] = [0u64, 1, 5, 6, 7][(seed % 5) as usize];
            },
        );
    }

    #[test]
    fn cfe_state_trophy_room_on_widget_action_matches() {
        check(
            ee::s04::f_001d4698__generated,
            cfe_state_trophy_room_on_widget_action,
            SEEDS_5,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1d46, 4);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                let item = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = if seed % 4 == 0 { 0 } else { item as u64 };
                c.g[A2][0] = match seed % 3 {
                    0 => 1,
                    1 => 6,
                    _ => 2,
                };
                c.bus.write32(item + 0x18, seed % 5);
                let panel = DATA + 0x2400;
                c.bus.write32(this + 0x48, panel);
                let vt = vtable(c, &mut r, &[0x48]);
                c.bus.write32(panel + 8, vt);
                c.bus.write32(this + 0x10, DATA + 0x2800);
                c.bus.write32(DATA + 0x2800, DATA + 0x2c00);
                c.bus.write32(DATA + 0x2c00 + 4, vt);
                c.bus.write32(this + 0x54, seed % 3);
            },
        );
    }

    #[test]
    fn mem_card_state_base_shutdown_matches() {
        diff(
            ee::s04::f_001d58b8__generated,
            mem_card_state_base_shutdown,
            20,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x1d_58b8);
                c.g[A0][0] = sx(DATA);
                w(c, DATA + 0x1bc, (seed % 4) as u32);
                w(c, DATA + 0x1c0, r.next_u32());
                w(c, DATA + 0x20, r.next_u32());
            },
        );
    }
}
