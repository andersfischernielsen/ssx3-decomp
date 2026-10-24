//! `src/fe/ovstatelodge.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::be::belibrary::lb;
pub(crate) use crate::game::fe::debugfe::at;
pub(crate) use crate::game::fe::debugfe::r32_2;
pub(crate) use crate::game::fe::feasyncfile::at as at_3;
pub(crate) use crate::game::fe::feasyncfile::gp_13 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::lw_5;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw_6;
pub(crate) use crate::game::fe::feasyncfile::p as p_3;
pub(crate) use crate::game::fe::fepopup::at as at_4;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_7;
pub(crate) use crate::game::fe::fepopup2::fe_state_screen_init;
pub(crate) use crate::game::fe::fepopup3::gp_2 as gp;
pub(crate) use crate::game::fe::fepopup3::lw_2;
pub(crate) use crate::game::fe::fepopup3::p_2 as p;
pub(crate) use crate::game::fe::fepopup3::sw;
pub(crate) use crate::game::fe::festaterewards::at_4 as at_2;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw_4;
pub(crate) use crate::game::fe::festaterewards2::handle_list_key;
pub(crate) use crate::game::fe::ovstatepause::lw;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw as lw_3;
pub(crate) use crate::game::fe::ovtemplatepausemenu::p as p_2;
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
        0x001d_28f8,
        cfe_state_mountain_room_dtor,
        "cFEStateMountainRoom_dtor",
    ),
    (
        0x001d_2a90,
        fe_state_mountain_room_on_activate,
        "cFEStateMountainRoom_OnActivate",
    ),
    (
        0x001d_2ea0,
        cfe_state_mountain_room_handle_input,
        "cFEStateMountainRoom_HandleInput",
    ),
    (
        0x001d_2f40,
        cfe_state_mountain_room_on_widget_notify,
        "cFEStateMountainRoom_OnWidgetNotify",
    ),
    (
        0x001d_2f68,
        c_fe_state_mountain_room_on_widget_action,
        "cFEStateMountainRoom_OnWidgetAction",
    ),
    (
        0x001d_3120,
        c_fe_state_mountain_room_update,
        "cFEStateMountainRoom_Update",
    ),
    (
        0x001d_3160,
        cfe_state_mountain_room_request_image,
        "cFEStateMountainRoom_RequestImage",
    ),
    (
        0x001d_32e0,
        c_fe_state_mountain_room_show_image,
        "cFEStateMountainRoom_ShowImage",
    ),
    (
        0x001d_3890,
        c_fe_state_peak_room_init,
        "cFEStatePeakRoom_Init",
    ),
    (
        0x001d_3e08,
        fe_state_peak_room_on_widget_notify,
        "cFEStatePeakRoom_OnWidgetNotify",
    ),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
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
    pub(crate) use crate::game::fe::festaterewards2::callee::ANIM_OBJECT_V36;
    pub(crate) use crate::game::fe::festaterewards2::callee::APP_MODULE_TRY_EXIT;
    pub(crate) use crate::game::fe::festaterewards2::callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY;
    pub(crate) use crate::game::fe::festaterewards2::callee::CROWD_AUDIO_INSTANCE_STOP_SOUND;
    pub(crate) use crate::game::fe::festaterewards2::callee::DL_HEAP_USABLE_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::DYNAMIC_EMITTER_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::EA_MUTEX_LOCK;
    pub(crate) use crate::game::fe::festaterewards2::callee::EA_MUTEX_UNLOCK;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_LOGIN_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_LOGIN_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_STATS_REFRESH_RESULTS;
    pub(crate) use crate::game::fe::festaterewards2::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEMSET;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_GET_POOL_BLOCK_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_MAN_FREE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_REPORT_POOL_STATS;
    pub(crate) use crate::game::fe::festaterewards2::callee::MOUNTAIN_ROOM_UPDATE_EVENT_ICONS;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_DISCONNECTED_POPUP_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_QUICK_PLAY_POPUP_ON_OPTIONS_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::POPUP_CONFIRM_GET_TEXT_BOX_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::PREVIEW_REWARD_POLL_IMAGE_LOAD;
    pub(crate) use crate::game::fe::festaterewards2::callee::PS2_PAD_CREATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_APPLY_CAMERA;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_FIND_NEXT_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_FIND_PREV_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_SET_STATE_2 as REPLAY_SET_STATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_SNAP_TO_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_STREAM_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SCROLL_ARROWS;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_COUNT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_ICONS;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_NAMES;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCE_USBKB_SET_ARRANGEMENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCE_USBKB_SET_READ_MODE;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_ENGINE_LIST_STOP;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_ENGINE_UNLOAD_WORLD_SCRIPTS;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_LIST_ADD_MOVIE;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_LIST_POLL_LOADING;
    pub(crate) use crate::game::fe::festaterewards2::callee::SECTION_MAN_FIND_LOGICAL_SECTION;
    pub(crate) use crate::game::fe::festaterewards2::callee::SONG_INSTANCE_GET_DUCK_TO_LOOPS;
    pub(crate) use crate::game::fe::festaterewards2::callee::SPRINTF;
    pub(crate) use crate::game::fe::festaterewards2::callee::S_LOGIN_UPDATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::TROPHY_ROOM_LOAD_TROPHY_TEXTURES;
    pub(crate) use crate::game::fe::festaterewards2::callee::TWSCRIPT_CACHE_RELOCATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_STATE_UPDATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SCROLL_SET_WIDE_STRING;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::fe::festaterewards2::callee::USB_KB_DEFAULT_CONNECT_CB;
    pub(crate) use crate::game::fe::festaterewards2::callee::WORLD_STATE_STREAM_RESET;
    pub(crate) use crate::game::fe::festaterewards2::callee::WORLD_TRIGGER_MANAGER_GET_TRIGGER_INFO;
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
    pub(crate) use crate::game::fe::ovstatepause::callee::C_OV_STATE_OVERLAY_UPDATE;
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
    pub(crate) use crate::game::fe::ovstatepause::callee::NET_FE_STATE_ON_CHILD_EVENT as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::OV_SCREEN_SESSION_DESTROY_INDICATORS;
    pub(crate) use crate::game::fe::ovstatepause::callee::PLAY_FE_SOUND;
    pub(crate) use crate::game::fe::ovstatepause::callee::RIDER_METRIX_CLEAR_PEAK_A4;
    pub(crate) use crate::game::fe::ovstatepause::callee::SECTION_MAN_REQUEST_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::ovstatepause::callee::STRCPY;
    pub(crate) use crate::game::fe::ovstatepause::callee::STRLEN;
    pub(crate) use crate::game::fe::ovstatepause::callee::STR_HASH;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_CTOR as UI_STATE_CTOR_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_MGR_CLOSE_TOP_STATE_AND_POPUPS;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_ON_ACTIVATE_2 as UI_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_POST_INPUT;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_RENDER;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_UPDATE_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXTURE_SET_FIND;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXT_SET_STRING as UI_TEXT_SET_STRING_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::VID_ENGINE_PLAYER_RENDER;
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

/// `void cFEStateMountainRoom_OnActivate(self, arg)` (0x1D2A90): rebuild the
/// event icons, then run the shared state activation.
pub fn fe_state_mountain_room_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        callee::MOUNTAIN_ROOM_UPDATE_EVENT_ICONS.call(c, 0x001d_2aac)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        callee::FE_STATE_ON_ACTIVATE.call(c, 0x001d_2ab8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateMountainRoom_Update(State* s)` (0x1d3120): poll the room
/// image load unless `+0xbc` is all ones, then update the UI.
pub fn c_fe_state_mountain_room_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = crate::abi::r32(c, S0);
        c.g[V0][0] = lw(c, this, 0xbc);
        c.g[V0][0] = !(0u64 | c.g[V0][0]);
        if c.g[V0][0] != 0 {
            c.cycles += 2;
            callee::MOUNTAIN_ROOM_POLL_IMAGE_LOAD.call(c, 0x001d_3148)?;
        }
        c.cycles += 6;
        c.g[A0][0] = c.g[S0][0];
        callee::UI_STATE_UPDATE_2.call(c, 0x001d_3150)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateMountainRoom_RequestImage(cFEStateMountainRoom* self)`
/// (0x1d3160): if the image request has not started (`+0xbc < 0`) and an item
/// is queued (`+0xc0`), start loading item `+0xb0` through the bolt file of
/// the world's rider-0 manager.
pub fn cfe_state_mountain_room_request_image(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw_2(c, p(c.g[S0][0], 0xbc));
        if (c.g[V0][0] as i64) >= 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 3;
            c.g[V1][0] = lw_2(c, p(c.g[S0][0], 0xc0));
            let no_item = c.g[V1][0] == 0;
            c.g[V0][0] = lw_2(c, gp(c, -0x848));
            if !no_item {
                c.cycles += 9;
                c.g[A3][0] = 0xa;
                c.g[A2][0] = lw_2(c, p(c.g[V1][0], 4));
                c.g[T0][0] = 1;
                c.g[A0][0] = lw_2(c, p(c.g[V0][0], 0x7c));
                c.g[T1][0] = 0;
                c.g[A1][0] = lw_2(c, p(c.g[S0][0], 0xb0));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1a70));
                call_ptr(c, 0x0019_da10, 0x001d_31a8)?;
                sw(c, p(c.g[S0][0], 0xbc), c.g[V0][0]);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cFEStateMountainRoom_OnWidgetNotify(State*, void*, int key)`
/// (0x001d2f40).
pub fn cfe_state_mountain_room_on_widget_notify(c: &mut Cpu) {
    handle_list_key(c);
}

/// `void cFEStateMountainRoom_ShowImage(cFEStateMountainRoom* self, int a1)`
/// (0x1d32e0): show the room's image, and the second one when it has been
/// unlocked.
pub fn c_fe_state_mountain_room_show_image(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let self_ = c.g[A0][0];
        c.g[S0][0] = self_;
        c.g[A2][0] = Addr(at(c, S0, 0x48)).lw(c);
        c.g[V0][0] = Addr(r32_2(c, A2)).off(0x8).lw(c);
        c.g[A0][0] = Addr(r32_2(c, V0)).off(0x48).lh(c);
        c.g[V1][0] = Addr(r32_2(c, V0)).off(0x4c).lw(c);
        let target = r32_2(c, V1);
        c.g[A0][0] = abi::addu(c.g[A2][0], c.g[A0][0]);
        call_ptr(c, target, 0x001d_330c)?;
        c.g[V0][0] = Addr(at(c, S0, 0xc4)).lw(c);
        let locked = (c.g[V0][0] as i64) < 0;
        c.g[A1][0] = 0x1;
        if !locked {
            c.cycles += 6;
            c.g[A2][0] = Addr(at(c, S0, 0x4c)).lw(c);
            c.g[V0][0] = Addr(r32_2(c, A2)).off(0x8).lw(c);
            c.g[A0][0] = Addr(r32_2(c, V0)).off(0x48).lh(c);
            c.g[V1][0] = Addr(r32_2(c, V0)).off(0x4c).lw(c);
            let target = r32_2(c, V1);
            c.g[A0][0] = abi::addu(c.g[A2][0], c.g[A0][0]);
            call_ptr(c, target, 0x001d_3330)?;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 4;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStatePeakRoom_Init(cFEStatePeakRoom* self)` (0x1d3890).
pub fn c_fe_state_peak_room_init(c: &mut Cpu) {
    guest(c, |c| {
        fe_state_screen_init(c, 0x0046_7648, [0x001d_38b4, 0x001d_38c8, 0x001d_38e0])
    })
}

/// `void cFEStateMountainRoom_dtor(self, flags)` (0x1d28f8): release the room's
/// music slot and rider, repoint the vtable and run the base destructor.
pub fn cfe_state_mountain_room_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x470000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x67a8));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(c.g[S0][0] as u32).off(0x8).sw(c, c.g[V0][0]);
        c.g[A2][0] = lw_4(c, S0, 0xbc);
        let no_slot = (c.g[A2][0] as i64) < 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_slot {
            c.cycles += 7;
            c.g[V0][0] = Addr(gp_2(c, -0x848)).lw(c);
            c.g[A1][0] = lw_4(c, S0, 0xb0);
            c.g[A0][0] = lw_4(c, V0, 0x7c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1a70));
            call_ptr(c, table_callee::MUSIC_SLOT_RELEASE, 0x001d_2938)?;
            c.g[V0][0] = u64::MAX;
            Addr(c.g[S0][0] as u32).off(0xbc).sw(c, c.g[V0][0]);
        }
        c.cycles += 3;
        c.g[A1][0] = lw_4(c, S0, 0xc4);
        let no_rider = (c.g[A1][0] as i64) < 0;
        c.g[A2][0] = Addr(gp_2(c, -0x854)).lw(c);
        if !no_rider {
            c.cycles += 5;
            c.g[V0][0] = lw_4(c, A2, 0x10d8);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x190).lh(c);
            c.g[V1][0] = lw_4(c, V0, 0x194);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, 0x001d_2960)?;
        }
        c.cycles += 11;
        c.g[V0][0] = 0x470000;
        c.g[A0][0] = c.g[S0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2f30));
        c.g[A1][0] = c.g[S1][0];
        Addr(c.g[S0][0] as u32).off(0x8).sw(c, c.g[V0][0]);
        call_ptr(c, table_callee::UI_STATE_DTOR, 0x001d_2978)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool cFEStateMountainRoom_HandleInput(self, input)` (0x1d2ea0): when the
/// widget is shown and its sound event fires, play the activate sound and
/// hide the widget.
pub fn cfe_state_mountain_room_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        c.g[V1][0] = lw_5(c, S0, 0x48);
        let no_widget = c.g[V1][0] == 0;
        c.g[V0][0] = 0;
        if !no_widget {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x14).lw(c);
            c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
            c.g[V0][0] &= 1;
        }
        c.cycles += 2;
        let hidden = c.g[V0][0] == 0;
        c.g[V0][0] = 0;
        if !hidden {
            c.cycles += 7;
            c.g[V1][0] = Addr(abi::arg(c, 1)).off(8).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x30).lh(c);
            c.g[V0][0] = lw_5(c, V1, 0x34);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001d_2ee4)?;
            let click = c.g[V0][0] == 0;
            c.g[A1][0] = 3;
            if !click {
                c.cycles += 16;
                c.g[V0][0] = lw_5(c, S0, 0x10);
                c.g[V1][0] = sx(gp(c, 0x2968));
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x14).lw(c);
                if c.g[A0][0] == 0 {
                    c.g[A0][0] = c.g[V1][0];
                }
                c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x10).lh(c);
                c.g[A2][0] = lw_5(c, V0, 0x14);
                let t = c.g[A2][0] as u32;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                call_ptr(c, t, 0x001d_2f10)?;
                c.g[A3][0] = lw_5(c, S0, 0x48);
                c.g[A1][0] = 0;
                c.g[V0][0] = Addr(c.g[A3][0] as u32).off(8).lw(c);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x48).lh(c);
                c.g[V1][0] = lw_5(c, V0, 0x4c);
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001d_2f2c)?;
            }
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cFEStatePeakRoom_OnWidgetNotify` (0x001d3e08).
//
// `int cFEStatePeakRoom::OnWidgetNotify(cFEState* self, UIWidget* item,
//  int key, int id)`: keys 8 and 9 report "handled with 0x100"; key 6 routes
// the selection into the lodge screen and reports 0x10/0x101 by whether the
// comparison succeeded; everything else reports 0x101.

pub fn fe_state_peak_room_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        'b_1d3ea8: {
            'b_1d3ea4: {
                'b_1d3e5c: {
                    c.cycles += 11;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                    c.g[V0][0] = 0x6;
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S2][0] = c.g[A3][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.g[S3][0] = c.g[A1][0];
                    let t = c.g[A2][0] == c.g[V0][0];
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    if t {
                        break 'b_1d3e5c;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = (c.g[A2][0] < 0x6u64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = 0x101;
                        if t {
                            break 'b_1d3ea8;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = (c.g[A2][0] < 0xau64) as u64;
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = (c.g[A2][0] < 0x8u64) as u64;
                            if t {
                                break 'b_1d3ea4;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V0][0] != 0u64;
                                c.g[V0][0] = 0x101;
                                if t {
                                    break 'b_1d3ea8;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[V0][0] = 0x100;
                                    if t {
                                        break 'b_1d3ea8;
                                    } else {
                                        break 'b_1d3e5c;
                                    }
                                }
                            }
                        }
                    }
                }
                c.cycles += 18;
                c.g[RA][0] = sx(0x001d3e64);
                call_ptr(c, 0x0014dc80, 0x001d3e64)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001d3e70);
                c.g[A1][0] = 0xd;
                call_ptr(c, 0x0014dd58, 0x001d3e70)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x74));
                c.g[RA][0] = sx(0x001d3e80);
                c.g[A1][0] = c.g[S2][0];
                call_ptr(c, 0x00397870, 0x001d3e80)?;
                c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xe0));
                c.g[A0][0] = c.g[S0][0];
                c.g[T0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
                c.g[A1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x44));
                c.g[RA][0] = sx(0x001d3e98);
                c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xdc));
                call_ptr(c, 0x00157bf0, 0x001d3e98)?;
                c.g[V1][0] = c.g[V0][0];
                let t = c.g[V1][0] == 0u64;
                c.g[V0][0] = 0x10;
                if t {
                    break 'b_1d3ea8;
                } else {
                    break 'b_1d3ea4;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0x101;
            break 'b_1d3ea8;
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());

        Ok(())
    })
}

/// `void func_001D2F68(void* self, void* widget, int msg)` (0x1d2f68): the
/// mountain room screen's widget action handler: message 5 replaces a room
/// image object with the next one from the factory, 6 does the same for a
/// specific index, and 1 sets the caption text of `self+0x54` from the
/// widget's index.
pub fn c_fe_state_mountain_room_on_widget_action(c: &mut Cpu) {
    let frame = Frame::open(c, 0x30);
    'b_1d3110: {
        'b_1d310c: {
            c.cycles += 7;
            c.g[V1][0] = c.g[A1][0];
            frame.save(c, S0, 0x20);
            frame.save(c, S1, 0x10);
            c.g[S0][0] = c.g[A0][0];
            let t = c.g[V1][0] == 0u64;
            sd(c, r32(c, SP), c.g[RA][0]);
            if t {
                break 'b_1d310c;
            } else {
                'b_1d3040: {
                    'b_1d302c: {
                        'b_1d3000: {
                            'b_1d2fbc: {
                                c.cycles += 3;
                                c.g[V0][0] = 0x5;
                                let t = c.g[A2][0] == c.g[V0][0];
                                c.g[V0][0] = (c.g[A2][0] < 0x6u64) as u64;
                                if t {
                                    break 'b_1d2fbc;
                                } else {
                                    'b_1d2fa8: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = 0x1;
                                        if t {
                                            break 'b_1d2fa8;
                                        } else {
                                            c.cycles += 2;
                                            if c.g[A2][0] == c.g[V0][0] {
                                                c.g[V0][0] = lw_7(c, at_4(c, S0, 0x54));
                                                break 'b_1d3040;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                frame.restore(c, S0, 0x20);
                                                if t {
                                                    break 'b_1d3110;
                                                } else {
                                                    break 'b_1d2fa8;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x6;
                                    if c.g[A2][0] == c.g[V0][0] {
                                        c.g[V0][0] = lw_7(c, at_4(c, S0, 0x10));
                                        break 'b_1d3000;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        frame.restore(c, S0, 0x20);
                                        if t {
                                            break 'b_1d3110;
                                        } else {
                                            break 'b_1d2fbc;
                                        }
                                    }
                                }
                            }
                            'b_1d2fd8: {
                                c.cycles += 3;
                                c.g[A1][0] = lw_7(c, at_4(c, V1, 0x18));
                                if c.g[A1][0] != 0u64 {
                                    c.g[V0][0] = lw_7(c, at_4(c, S0, 0x10));
                                    break 'b_1d2fd8;
                                } else {
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x001d2fd0);
                                    c.g[A0][0] = c.g[S0][0];
                                    if callee::C001d32e0.call(c, 0x001d2fd0).is_err() {
                                        return;
                                    }
                                    let t = true;
                                    frame.restore(c, S0, 0x20);
                                    if t {
                                        break 'b_1d3110;
                                    } else {
                                        break 'b_1d2fd8;
                                    }
                                }
                            }
                            c.cycles += 10;
                            c.g[A2][0] = sx(r32(c, A1).wrapping_sub(0x1));
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A3][0] = lw_7(c, r32(c, V0));
                            c.g[V1][0] = lw_7(c, at_4(c, A3, 0x4));
                            c.g[A0][0] = lh(c, at_4(c, V1, 0x20));
                            c.g[V0][0] = lw_7(c, at_4(c, V1, 0x24));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x001d2ff8);
                            c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
                            if call_ptr(c, t, 0x001d2ff8).is_err() {
                                return;
                            }
                            let t = true;
                            c.g[A0][0] = lw_7(c, at_4(c, S0, 0x10));
                            if t {
                                break 'b_1d302c;
                            } else {
                                break 'b_1d3000;
                            }
                        }
                        c.cycles += 10;
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A2][0] = lw_7(c, at_4(c, V1, 0x18));
                        c.g[A3][0] = lw_7(c, r32(c, V0));
                        c.g[V1][0] = lw_7(c, at_4(c, A3, 0x4));
                        c.g[A0][0] = lh(c, at_4(c, V1, 0x28));
                        c.g[V0][0] = lw_7(c, at_4(c, V1, 0x2c));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001d3020);
                        c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
                        if call_ptr(c, t, 0x001d3020).is_err() {
                            return;
                        }
                        if c.g[V0][0] == 0u64 {
                            frame.restore(c, S0, 0x20);
                            break 'b_1d3110;
                        } else {
                            c.cycles += 1;
                            c.g[A0][0] = lw_7(c, at_4(c, S0, 0x10));
                            break 'b_1d302c;
                        }
                    }
                    c.cycles += 5;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001d3038);
                    c.g[A0][0] = sx(at_4(c, A0, 0x18));
                    if call_ptr(c, 0x0039f400, 0x001d3038).is_err() {
                        return;
                    }
                    let t = true;
                    frame.restore(c, S0, 0x20);
                    if t {
                        break 'b_1d3110;
                    } else {
                        break 'b_1d3040;
                    }
                }
                c.cycles += 2;
                if c.g[V0][0] == 0u64 {
                    frame.restore(c, S0, 0x20);
                    break 'b_1d3110;
                } else {
                    'b_1d30e8: {
                        'b_1d30d8: {
                            'b_1d30bc: {
                                'b_1d309c: {
                                    'b_1d3090: {
                                        c.cycles += 3;
                                        c.g[A1][0] = lw_7(c, at_4(c, V1, 0x18));
                                        let t = c.g[A1][0] == c.g[A2][0];
                                        c.g[S1][0] = 0u64;
                                        if t {
                                            break 'b_1d3090;
                                        } else {
                                            'b_1d3088: {
                                                'b_1d3070: {
                                                    c.cycles += 3;
                                                    c.g[V0][0] =
                                                        ((c.g[A1][0] as i64) < 2i64) as u64;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0x2;
                                                    if t {
                                                        break 'b_1d3070;
                                                    } else {
                                                        c.cycles += 2;
                                                        if c.g[A1][0] == 0u64 {
                                                            c.g[A0][0] = 0x460000;
                                                            break 'b_1d3088;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[A2][0] = lw_7(c, at_4(c, S0, 0x54));
                                                            if t {
                                                                break 'b_1d30e8;
                                                            } else {
                                                                break 'b_1d3070;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = c.g[A1][0] == c.g[V0][0];
                                                c.g[V0][0] = 0x3;
                                                if t {
                                                    break 'b_1d309c;
                                                } else {
                                                    c.cycles += 2;
                                                    if c.g[A1][0] == c.g[V0][0] {
                                                        c.g[V0][0] = lw_7(c, at_4(c, S0, 0xb4));
                                                        break 'b_1d30bc;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.g[A2][0] = lw_7(c, at_4(c, S0, 0x54));
                                                        if t {
                                                            break 'b_1d30e8;
                                                        } else {
                                                            break 'b_1d3088;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[A0][0] = sx(at_4(c, A0, 0x7450));
                                            if t {
                                                break 'b_1d30d8;
                                            } else {
                                                break 'b_1d3090;
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[A0][0] = 0x460000;
                                    let t = true;
                                    c.g[A0][0] = sx(at_4(c, A0, 0x7460));
                                    if t {
                                        break 'b_1d30d8;
                                    } else {
                                        break 'b_1d309c;
                                    }
                                }
                                'b_1d30b4: {
                                    c.cycles += 3;
                                    c.g[V0][0] = lw_7(c, at_4(c, S0, 0xb4));
                                    if (c.g[V0][0] as i64) <= 0 {
                                        c.g[A0][0] = 0x460000;
                                        break 'b_1d30b4;
                                    } else {
                                        c.cycles += 3;
                                        c.g[A0][0] = 0x460000;
                                        let t = true;
                                        c.g[A0][0] = sx(at_4(c, A0, 0x7460));
                                        if t {
                                            break 'b_1d30d8;
                                        } else {
                                            break 'b_1d30b4;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = true;
                                c.g[A0][0] = sx(at_4(c, A0, 0x7478));
                                if t {
                                    break 'b_1d30d8;
                                } else {
                                    break 'b_1d30bc;
                                }
                            }
                            'b_1d30d4: {
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[V0][0] as i64) < 2i64) as u64;
                                if c.g[V0][0] != 0u64 {
                                    c.g[A0][0] = 0x460000;
                                    break 'b_1d30d4;
                                } else {
                                    c.cycles += 3;
                                    c.g[A0][0] = 0x460000;
                                    let t = true;
                                    c.g[A0][0] = sx(at_4(c, A0, 0x7460));
                                    if t {
                                        break 'b_1d30d8;
                                    } else {
                                        break 'b_1d30d4;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[A0][0] = sx(at_4(c, A0, 0x7490));
                            break 'b_1d30d8;
                        }
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001d30e0);
                        if call_ptr(c, 0x00317670, 0x001d30e0).is_err() {
                            return;
                        }
                        c.g[S1][0] = c.g[V0][0];
                        c.g[A2][0] = lw_7(c, at_4(c, S0, 0x54));
                        break 'b_1d30e8;
                    }
                    c.cycles += 9;
                    c.g[A1][0] = 0x1;
                    c.g[V0][0] = lw_7(c, at_4(c, A2, 0x8));
                    c.g[A0][0] = lh(c, at_4(c, V0, 0x48));
                    c.g[V1][0] = lw_7(c, at_4(c, V0, 0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001d3100);
                    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                    if call_ptr(c, t, 0x001d3100).is_err() {
                        return;
                    }
                    c.g[A0][0] = lw_7(c, at_4(c, S0, 0x54));
                    c.g[RA][0] = sx(0x001d310c);
                    c.g[A1][0] = c.g[S1][0];
                    if call_ptr(c, 0x003a1000, 0x001d310c).is_err() {
                        return;
                    }
                    break 'b_1d310c;
                }
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x20);
        break 'b_1d3110;
    }
    c.cycles += 4;
    frame.restore(c, S1, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_4(c, SP, 0x30));
    c.pc = t;
    return;
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::compare;
    use crate::difftest::elf_image;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::SP;
    use crate::difftest::SP as STACK;
    pub(crate) use crate::game::fe::debugfe::tests::w as w_2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ2;
    pub(crate) use crate::game::fe::debugfe::tests::OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_9 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_7 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::put_str_2 as put_str;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_3;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_3;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;
    pub(crate) use crate::game::fe::fepopup::tests::AUX;
    pub(crate) use crate::game::fe::fepopup::tests::BEL;
    pub(crate) use crate::game::fe::fepopup::tests::RACE;
    pub(crate) use crate::game::fe::fepopup::tests::RMAN_2 as RMAN;
    pub(crate) use crate::game::fe::fepopup::tests::SYS;
    pub(crate) use crate::game::fe::fepopup::tests::WORLD_4 as WORLD;
    pub(crate) use crate::game::fe::fepopup3::tests::setup;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4 as base;
    pub(crate) use crate::game::fe::festaterewards::tests::globals_3 as globals;
    pub(crate) use crate::game::fe::festaterewards::tests::heap_init_3 as heap_init;
    pub(crate) use crate::game::fe::festaterewards::tests::pool_4 as pool;
    pub(crate) use crate::game::fe::festaterewards::tests::slot_2;
    pub(crate) use crate::game::fe::festaterewards::tests::w16;
    pub(crate) use crate::game::fe::festaterewards2::cfe_state_rewards_room_on_widget_notify;
    pub(crate) use crate::game::fe::festaterewards2::mem_card_state_base_on_widget_notify;
    pub(crate) use crate::game::fe::festaterewards2::tests::belib;
    pub(crate) use crate::game::fe::festaterewards2::tests::w;
    pub(crate) use crate::game::fe::festaterewards3::cfe_state_buy_attrib_on_widget_notify;
    pub(crate) use crate::game::fe::ovstateenterlodge::ov_screen_enter_lodge_on_widget_notify;
    pub(crate) use crate::game::fe::ovtemplatedialog::ov_screen_yn_dialog_on_widget_notify;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup as setup_2;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY_PTR;
    pub(crate) use crate::game::world::worldview::tests::SEEDS_5 as SEEDS;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn fe_state_mountain_room_on_activate_matches() {
        check(
            ssx3_ee::s04::f_001d2a90__generated,
            fe_state_mountain_room_on_activate,
            40,
            &|c, seed| {
                machine(c, seed, 0x1d2a_90, 2);
                belib(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x200) as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_mountain_room_update_matches() {
        check(
            ssx3_ee::s04::f_001d3120__generated,
            c_fe_state_mountain_room_update,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x25, 1);
                c.g[A0][0] = sx(DATA);
                c.bus
                    .write32(DATA + 0xbc, if seed % 2 == 0 { 0 } else { 0xffff_ffff });
            },
        );
    }

    #[test]
    fn cfe_state_mountain_room_request_image_matches() {
        check(
            ssx3_ee::s04::f_001d3160__generated,
            cfe_state_mountain_room_request_image,
            120,
            &|c, seed| {
                let _r = setup(c, seed, 0x1d3160);
                let this = DATA;
                c.g[A0][0] = this as u64;
                // Both cheap exits on some seeds, the request path on the rest.
                c.bus
                    .write32(this + 0xbc, if seed % 4 == 0 { 0 } else { 0xffff_ffff });
                c.bus
                    .write32(this + 0xc0, if seed % 3 == 0 { 0 } else { DATA + 0x300 });
                c.bus.write32(this + 0xb0, 0);
                // The world, its rider-0 manager, and slot 0 of the bolt-file
                // table already holding the requested item: `FindSlotByName`
                // matches (strcmp of the two equal names) and returns slot 0,
                // so the callee takes the refcount path instead of the
                // free-slot/evict/load walk.
                c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x800);
                c.bus.write32(DATA + 0x87c, DATA + 0x1000);
                c.bus.write32(DATA + 0x304, DATA + 0x100);
                put_str(c, DATA + 0x100, "bolt");
                put_str(c, DATA + 0x2a74, "bolt");
            },
        );
    }

    #[test]
    fn handle_list_key_family_matches() {
        check(
            ssx3_ee::s04::f_001d2f40__generated,
            cfe_state_mountain_room_on_widget_notify,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x1d2f40, 0);
                c.g[A2][0] = (seed % 13) as u64;
            },
        );
        check(
            ssx3_ee::s04::f_001d4f68__generated,
            cfe_state_rewards_room_on_widget_notify,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x1d4f68, 0);
                c.g[A2][0] = (seed % 13) as u64;
            },
        );
        check(
            ssx3_ee::s04::f_001d5460__generated,
            mem_card_state_base_on_widget_notify,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x1d5460, 0);
                c.g[A2][0] = (seed % 13) as u64;
            },
        );
        check(
            ssx3_ee::s04::f_001f4a38__generated,
            cfe_state_buy_attrib_on_widget_notify,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x1f4a38, 0);
                c.g[A2][0] = (seed % 13) as u64;
            },
        );
        check(
            ssx3_ee::s04::f_001f72b8__generated,
            ov_screen_enter_lodge_on_widget_notify,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x1f72b8, 0);
                c.g[A2][0] = (seed % 13) as u64;
            },
        );
        check(
            ssx3_ee::s05::f_0020d568__generated,
            ov_screen_yn_dialog_on_widget_notify,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x20d568, 0);
                c.g[A2][0] = (seed % 13) as u64;
            },
        );
    }

    /// A fake vtable page.
    pub(crate) const VT: u32 = DATA + 0x8000;

    /// A vtable slot `{ short delta; short; fn }` at `vt + off`.
    pub(crate) fn slot(c: &mut Cpu, vt: u32, off: u32, delta: i16) {
        c.bus.write16(vt + off, delta as u16);
        c.bus.write32(vt + off + 4, fakes::FAKE);
    }

    #[test]
    fn c_fe_state_mountain_room_show_image_matches() {
        check(
            ssx3_ee::s04::f_001d32e0__generated,
            c_fe_state_mountain_room_show_image,
            80,
            &|c, seed| {
                fakes::machine(c, seed, 0x1d32e0, 0);
                w_2(c, OBJ + 0x48, OBJ2);
                w_2(c, OBJ + 0x4c, OBJ3);
                w_2(c, OBJ2 + 0x8, VT);
                w_2(c, OBJ3 + 0x8, VT);
                slot(c, VT, 0x48, 0);
                w_2(
                    c,
                    OBJ + 0xc4,
                    if seed % 2 == 0 { 0xffff_ffff } else { seed },
                );
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = seed as u64;
            },
        );
    }

    #[test]
    fn c_fe_state_peak_room_init_matches() {
        check(
            ssx3_ee::s04::f_001d3890__generated,
            c_fe_state_peak_room_init,
            40,
            &|c, seed| {
                let _r = setup_2(c, seed, 0x1d3890);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(DATA + 0x40, seed);
            },
        );
    }

    #[test]
    fn cfe_state_mountain_room_dtor_matches() {
        check(
            ee::s04::f_001d28f8__generated,
            cfe_state_mountain_room_dtor,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x1d28f8, 3);
                let obj = DATA + 0x1000;
                let world = DATA + 0x8_0000;
                let game = world + 0x1000;
                let reg = DATA + 0x2_0000;
                let vt = DATA + 0x3_0000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(
                    c,
                    obj + 0xbc,
                    if seed % 2 == 0 {
                        0xffff_ffff
                    } else {
                        (seed % 8) as u32
                    },
                );
                w(c, obj + 0xb0, r.next_u32());
                w(
                    c,
                    obj + 0xc4,
                    if seed % 3 == 0 {
                        0xffff_ffff
                    } else {
                        (seed % 8) as u32
                    },
                );
                w(c, world + 0x7c, game);
                w(c, GAME_GP.wrapping_sub(0x854), reg);
                w(c, reg + 0x10d8, vt);
                slot_2(c, vt, 0x190, 0, FAKE);
            },
        );
    }

    #[test]
    fn cfe_state_mountain_room_handle_input_matches() {
        check(
            ssx3_ee::s04::f_001d2ea0__generated,
            cfe_state_mountain_room_handle_input,
            20,
            &|c, seed| {
                let _r = base_2(c, seed, 0x1d2ea0, 2);
                let this = DATA + 0x1000;
                let input = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = input as u64;
                let widget = DATA + 0x3000;
                w_3(c, this + 0x48, if seed % 3 == 0 { 0 } else { widget });
                w_3(c, widget + 0x14, if seed % 2 == 0 { 0 } else { 1 << 6 });
                w_3(c, input + 8, DATA + 0x4000);
                let zero_fn = ssx3_ee::s09::f_002cc098 as fn(&mut Cpu) as usize as u32;
                c.bus.write16(DATA + 0x4000 + 0x30, 0x2000);
                w_3(
                    c,
                    DATA + 0x4000 + 0x34,
                    if seed % 2 == 0 { zero_fn } else { 1 },
                );
                w_3(c, this + 0x10, DATA + 0x5000);
                w_3(c, DATA + 0x5000 + 0x14, 0);
                w_3(c, GAME_GP.wrapping_add(0x2968), DATA + 0x6000);
                w_3(c, DATA + 0x6000, DATA + 0x6100);
                c.bus.write16(DATA + 0x6100 + 0x10, 0);
                w_3(c, DATA + 0x6100 + 0x14, 0);
                w_3(c, widget + 8, DATA + 0x7000);
                c.bus.write16(DATA + 0x7000 + 0x48, 0);
                w_3(c, DATA + 0x7000 + 0x4c, 0);
            },
        );
    }

    #[test]
    fn fe_state_peak_room_on_widget_notify_matches() {
        strict(
            ee::s04::f_001d3e08__generated,
            fe_state_peak_room_on_widget_notify,
            120,
            &|c, seed| {
                let mut r = Rng(0x001d_3e08 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                c.g[A2][0] = [0u64, 5, 7, 8, 9, 10, 12][(seed % 7) as usize];
                c.g[A3][0] = sx(DATA + 0x200);
            },
        );
    }

    pub(crate) const ROOM: u32 = DATA + 0x1_0000;

    pub(crate) const ROOM_WIDGET: u32 = DATA + 0x1_1000;

    pub(crate) const ROOM_FACTORY: u32 = DATA + 0x1_2000;

    #[test]
    fn c_fe_state_mountain_room_on_widget_action_matches() {
        check_all(
            ee::s04::f_001d2f68__generated,
            c_fe_state_mountain_room_on_widget_action,
            24,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x1d2f_68, 8);
                c.g[A0][0] = sx(ROOM);
                c.g[A1][0] = sx(if seed % 5 == 0 { 0 } else { ROOM_WIDGET });
                c.g[A2][0] = sx([5u32, 6, 1, 0, 2][seed as usize % 5]);
                c.bus.write32(ROOM_WIDGET + 0x18, r.next_u32() % 4);
                c.bus.write32(ROOM + 0x10, ROOM_FACTORY);
                let vt = vtable(c, &mut r, &[0x20, 0x24, 0x28, 0x2c]);
                c.bus.write32(ROOM_FACTORY, vt);
                c.bus
                    .write32(ROOM + 0x54, if seed % 3 == 0 { 0 } else { DATA + 0x1_3000 });
                c.bus.write32(ROOM + 0xb4, (seed % 3) as u32);
                let vt = vtable(c, &mut r, &[0x48, 0x4c]);
                c.bus.write32(DATA + 0x1_3000 + 8, vt);
            },
        );
    }
}
