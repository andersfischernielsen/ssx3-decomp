//! `src/ui/uitext.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::call;
use crate::call::sp;
use crate::events::deliver_event;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2 as p;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p_2;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::main::gamemode::low;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::lh_r;
pub(crate) use crate::game::main::gamemode::sw_r;
pub(crate) use crate::game::main::gamestate::p_2 as p_3;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at;
pub(crate) use crate::game::main::gamestate::vslot;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at as lw_at_2;
pub(crate) use crate::game::scripter::ssxscriptengine::p as p_4;
pub(crate) use crate::game::fe::feasyncfile::p as p_5;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::feasyncfile::shl32;
pub(crate) use crate::game::wscript::wscriptman::p as p_6;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw;
pub(crate) use crate::game::wscript::wscriptman::sw;
pub(crate) use crate::game::fe::feasyncfile::p as p_7;
pub(crate) use crate::game::camera::script::scriptcontroller::vslot as vslot_2;
pub(crate) use crate::game::fe::feasyncfile::p as p_8;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw_2;
pub(crate) use crate::game::animation::rideranimbase::sw as sw_2;
pub(crate) use crate::game::camera::cameraalgorithms::lh;
pub(crate) use crate::game::util::menu::p as p_9;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003a_0b38, ui_text_free_strings, "UIText_FreeStrings"),
    (0x003a_0c00, ui_text_get_owned_string, "UIText_GetOwnedString"),
    (0x003a_0c28, ui_text_get_id_string, "UIText_GetIdString"),
    (0x003a_0c70, ui_text_set_ascii_string, "UIText_SetAsciiString"),
    (0x003a_0e90, ui_text_set_wide_string, "UIText_SetWideString"),
    (0x003a_1030, ui_text_get_text_size, "UIText_GetTextSize"),
    (0x003a_1148, c_ui_text_scroll_ctor, "cUITextScroll_ctor"),
    (0x003a_12d0, c_ui_text_scroll_get_font, "cUITextScroll_GetFont"),
    (0x003a_1310, c_ui_text_scroll_set_nav_buttons, "cUITextScroll_SetNavButtons"),
    (0x003a_13e0, cuit_ext_scroll_handle_input, "cUITextScroll_HandleInput"),
    (0x003a_18c0, ui_text_scroll_clear_lines, "cUITextScroll_ClearLines"),
    (0x003a_1958, c_ui_text_scroll_grow_lines, "cUITextScroll_GrowLines"),
    (0x003a_1bf8, c_ui_text_scroll_add_line, "cUITextScroll_AddLine"),
    (0x003a_1cf0, ui_text_scroll_replace_wide_string, "cUITextScroll_ReplaceWideString"),
    (0x003a_1f18, cui_text_scroll_set_text_id, "cUITextScroll_SetTextId"),
    (0x003a_2068, c_ui_text_scroll_get_line, "cUITextScroll_GetLine"),
    (0x003a_3280, ui_vector_ctor, "UIVector_ctor"),
    (0x003a_32f0, ui_vector_dtor, "UIVector_dtor"),
    (0x003a_3398, ui_vector_set_color, "UIVector_SetColor"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::be::beintmission::callee::WORLD_OBJ_RUN_KILL_SCRIPT;
    pub(crate) use crate::game::be::beintmission::callee::UITEXT_SET_WIDE_STRING_INTERNAL;
    pub(crate) use crate::game::be::beintmission::callee::MPEG2_ERROR;
    pub(crate) use crate::game::be::beintmission::callee::MISSION_COUNT;
    pub(crate) use crate::game::be::beintmission::callee::MISSION_NTH;
    pub(crate) use crate::game::be::beintmission::callee::REWARDS_FILE_LOAD;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_SET_ALGORITHM;
    pub(crate) use crate::game::be::beintmission::callee::CAMERA_TRIGGER_EMPTY_STUB;
    pub(crate) use crate::game::be::beintmission::callee::UBER_TRICK_REFRESH;
    pub(crate) use crate::game::be::beintmission::callee::MEMCARD_SCAN_STEP;
    pub(crate) use crate::game::be::beintmission::callee::NET_FE_STATE_ON_WIDGET_ACTION;
    pub(crate) use crate::game::be::beintmission::callee::VIEW_POPUP_SHOW_MESSAGE;
    pub(crate) use crate::game::be::beintmission::callee::BUY_ATTRIB_UPDATE_BARS;
    pub(crate) use crate::game::be::beintmission::callee::CAREER_STATS_SHOW_HIGHLIGHTS;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_NAMES;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_ICONS;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SCROLL_ARROWS;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_COUNT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_DISCONNECTED_POPUP_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_LOGIN_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_LOGIN_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_QUICK_PLAY_POPUP_ON_OPTIONS_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_STATS_REFRESH_RESULTS;
    pub(crate) use crate::game::fe::festaterewards2::callee::POPUP_CONFIRM_GET_TEXT_BOX_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::PREVIEW_REWARD_POLL_IMAGE_LOAD;
    pub(crate) use crate::game::fe::festaterewards2::callee::MOUNTAIN_ROOM_UPDATE_EVENT_ICONS;
    pub(crate) use crate::game::fe::festaterewards2::callee::TROPHY_ROOM_LOAD_TROPHY_TEXTURES;
    pub(crate) use crate::game::fe::festaterewards2::callee::SECTION_MAN_FIND_LOGICAL_SECTION;
    pub(crate) use crate::game::fe::festaterewards2::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::APP_MODULE_TRY_EXIT;
    pub(crate) use crate::game::fe::festaterewards2::callee::S_LOGIN_UPDATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::USB_KB_DEFAULT_CONNECT_CB;
    pub(crate) use crate::game::fe::festaterewards2::callee::WORLD_STATE_STREAM_RESET;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_STREAM_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_FIND_NEXT_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_FIND_PREV_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_SNAP_TO_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_APPLY_CAMERA;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_SET_STATE_2 as REPLAY_SET_STATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_LIST_ADD_MOVIE;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_LIST_POLL_LOADING;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_ENGINE_UNLOAD_WORLD_SCRIPTS;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_ENGINE_LIST_STOP;
    pub(crate) use crate::game::fe::festaterewards2::callee::CROWD_AUDIO_INSTANCE_STOP_SOUND;
    pub(crate) use crate::game::fe::festaterewards2::callee::SONG_INSTANCE_GET_DUCK_TO_LOOPS;
    pub(crate) use crate::game::fe::festaterewards2::callee::WORLD_TRIGGER_MANAGER_GET_TRIGGER_INFO;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_MAN_FREE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_GET_POOL_BLOCK_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::DL_HEAP_USABLE_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_REPORT_POOL_STATS;
    pub(crate) use crate::game::fe::festaterewards2::callee::PS2_PAD_CREATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::ANIM_OBJECT_V36;
    pub(crate) use crate::game::fe::festaterewards2::callee::DYNAMIC_EMITTER_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_STATE_UPDATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SCROLL_SET_WIDE_STRING;
    pub(crate) use crate::game::fe::festaterewards2::callee::TWSCRIPT_CACHE_RELOCATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::EA_MUTEX_LOCK;
    pub(crate) use crate::game::fe::festaterewards2::callee::EA_MUTEX_UNLOCK;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCE_USBKB_SET_ARRANGEMENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCE_USBKB_SET_READ_MODE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEMSET;
    pub(crate) use crate::game::fe::festaterewards2::callee::SPRINTF;
    pub(crate) use crate::game::fe::festaterewards::callee::SEQ_LISTENER_ON_START;
    pub(crate) use crate::game::fe::festaterewards::callee::SEQ_LISTENER_ON_STOP;
    pub(crate) use crate::game::fe::festaterewards::callee::GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_LIST_ON_SEQ_START;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_LIST_ON_SEQ_STOPPED;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_BINDING_CLEAR_LISTENER;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_ITEM_GET_INPUT_CODE;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_ITEM_IS_DISABLED;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_ITEM_HANDLE_INPUT;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_PAGE_VALIDATE_SELECTION;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_PUSH_PAGE;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_FORMAT_FLOAT;
    pub(crate) use crate::game::fe::festaterewards::callee::MENU_ITEM_DRAW_ROW;
    pub(crate) use crate::game::fe::festaterewards::callee::IN_SEQ_INSTANCE_SETUP;
    pub(crate) use crate::game::fe::festaterewards::callee::LIGHT_MAN_EMPTY_HOOK;
    pub(crate) use crate::game::fe::festaterewards::callee::UI_OBJECT_CTOR;
    pub(crate) use crate::game::fe::festaterewards::callee::UI_LIST_CTOR;
    pub(crate) use crate::game::fe::festaterewards::callee::CUI_TEXT_SCROLL_SET_NAV_BUTTONS;
    pub(crate) use crate::game::fe::festaterewards::callee::FE_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards::callee::REWARD_GALLERY_CTOR;
    pub(crate) use crate::game::fe::festaterewards::callee::AI_VISUAL_EFFECTS_MAN_CTOR;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_STATS;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_CLEAR_HUD_MESSAGES;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_TRICK;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_COMBO;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_BANK_PENDING_SCORE;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_GET_SYNC_CHECKSUM;
    pub(crate) use crate::game::main::gamemode::callee::CBE_GET_BE;
    pub(crate) use crate::game::main::gamemode::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::main::gamemode::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GET_MEDAL_REWARD_ID;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_IS_REWARD_GRANTED;
    pub(crate) use crate::game::main::gamemode::callee::BE_REWARD_INT_GRANT_REWARD;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_UNLOAD;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_PARSE;
    pub(crate) use crate::game::main::gamemode::callee::CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS_MASK2;
    pub(crate) use crate::game::main::gamemode::callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_UNLOAD_RIDER_PREVIEWS;
    pub(crate) use crate::game::main::gamemode::callee::FE_MODULE_RIDER_PREVIEW_DTOR;
    pub(crate) use crate::game::main::gamemode::callee::FE_ASYNC_MANAGER_SET_FILE_STATUS;
    pub(crate) use crate::game::main::gamemode::callee::REPLAY_FIND_SNAPSHOT;
    pub(crate) use crate::game::main::gamemode::callee::RUN_LIST_SEEK;
    pub(crate) use crate::game::main::gamemode::callee::SCRIPT_LIST_ON_SEQ_ADVANCE;
    pub(crate) use crate::game::main::gamemode::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::main::gamemode::callee::REPLAY_REQUEST_INSTANT_REPLAY;
    pub(crate) use crate::game::main::gamemode::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::main::gamemode::callee::CPS2_MEM_CARD_SET_STATE;
    pub(crate) use crate::game::main::gamemode::callee::MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::main::gamemode::callee::WSCRIPT_MAN_RESET_COLLECT_SET;
    pub(crate) use crate::game::main::gamemode::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::main::gamemode::callee::CBX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::main::gamemode::callee::EA_MEM_FREE;
    pub(crate) use crate::game::main::gamemode::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::main::gamemode::callee::MODIFIER_BLOCK_HAS_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::RAIL_MODIFIER_CTOR;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_SET_RAIL_MODIFIER;
    pub(crate) use crate::game::main::gamemode::callee::MULTI_PARTICLE_DTOR;
    pub(crate) use crate::game::main::gamemode::callee::MOD_OBJ_CTOR_2;
    pub(crate) use crate::game::main::gamemode::callee::FLEX_BRIDGE_ALLOC_SPRING_NODES;
    pub(crate) use crate::game::main::gamemode::callee::TEX_MAN_UNLOCK_VRAM;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_ACCEPT_VISITOR;
    pub(crate) use crate::game::main::gamemode::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::main::gamemode::callee::CUI_TEXT_SCROLL_SET_WIDE_STRING;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_RESET;
    pub(crate) use crate::game::main::gamemode::callee::CSTREAM_MAN_FREE_BUFFERS;
    pub(crate) use crate::game::main::gamemode::callee::STR_HASH;
    pub(crate) use crate::game::main::gamemode::callee::SPRINTF as SPRINTF_2;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET as MEMSET_2;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET_3E6448;
    pub(crate) use crate::game::main::gamemode::callee::MEMCPY;
    pub(crate) use crate::game::main::gamemode::callee::STRLEN;
    pub(crate) use crate::game::main::gamemode::callee::STRCPY;
    pub(crate) use crate::game::main::gamemode::callee::CVT_FLOAT_TO_INT;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::main::gamemode::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::main::gamemode::callee::FUNC_0028F140;
    pub(crate) use crate::game::main::gamemode::callee::C_MEM_MAN_ALLOC;
    pub(crate) use crate::game::main::gamemode::callee::AI_RAND;
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
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_STRING as UI_TEXT_SET_STRING_2;
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
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_MODULE_REQUEST_STATE_2;
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
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SET_LOCAL_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SEND_CHARACTER_MSG;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_CAN_SKIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_GET_PLAY_STATUS_2 as SCRIPT_LIST_GET_PLAY_STATUS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_4 as SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR_3 as SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_SCRIPT_SET_OWNER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PATH_PLAT_FIND_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MATH_EVAL_CUBIC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_CLOSE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_ADD_VERTEX;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_SCROLL_WINDOW;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_MASK_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_CURSOR_2 as KEYBOARD_POPUP_UPDATE_CURSOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_PACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_UNPACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_REWARD_INT_IS_PEAK_GOAL_COMPLETE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VEC3_NEG_DOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_3 as SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO_4 as SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_INSTANCE_GET_REMAINING_STEPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CONCAT_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ATTACHED_DYNAMIC_EMITTER_CREATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_MAN2D_CLEAR_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_STREAM_REWIND_READ;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_ALGORITHM_DEFAULT_FOV;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_CHARACTER as BE_GET_RIDER_CHARACTER_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_PROCESS_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_FLAG_MANAGER_GROUP_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_TABLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_ASCII_TO_WIDE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_TEXT_SET_WIDE_STRING_INTERNAL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BELIBRARY_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_MAN_FREE_ENTRY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_SCREEN_FIND_OBJECT_3 as UI_SCREEN_FIND_OBJECT_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_IS_LOADED_WITH_NAME as SND_BANK_IS_LOADED_WITH_NAME_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK as SND_LOAD_QUEUE_CANCEL_BANK_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_4 as SCRIPT_CMD_QUEUE_AT_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_6 as SPRINTF_3;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_GET_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE_CALLEE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_LOAD as SND_BANK_LOAD_2;
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

/// Cross-shard call targets (the generator emits `call_indirect` for them).
pub(crate) mod table {
    pub(crate) use crate::game::ai::computer::table::DELETE;
    pub(crate) use crate::game::ai::computer::table::EA_MEM_FREE;
    pub(crate) use crate::game::ai::computer::table::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::ai::computer::table::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::ai::computer::table::WAIT_SEMA;
    pub(crate) use crate::game::ai::computer::table::WAKEUP_THREAD;
    pub(crate) use crate::game::ai::computer::table::SLEEP_THREAD;
    pub(crate) use crate::game::ai::computer::table::STR_HASH;
    pub(crate) use crate::game::ai::computer::table::SSX_AUDIO_SYSTEM_GET_INSTANCE;
}


/// `void *cUITextScroll_GetFont(cUITextScroll *text)` (0x3a12d0): the font
/// record selected by bits 1..2 of the word at +0x74.
pub fn c_ui_text_scroll_get_font(c: &mut Cpu) {
    c.cycles += 5;
    let text = abi::arg(c, 0);
    c.g[V0][0] = Addr(text).off(0x5c).lw(c);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xd0).lw(c);
    c.g[A1][0] = Addr(c.g[V1][0] as u32).off(0x10).lw(c);
    if c.g[A1][0] != 0 {
        c.g[V1][0] = Addr(text).off(0x74).lw(c);
        c.cycles += 8;
        c.g[V0][0] = 0xc;
        c.g[A0][0] = Addr(c.g[A1][0] as u32).off(0x8).lw(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32) >> 1);
        c.g[V1][0] = c.g[V1][0] & 0x3;
        c.mult(0, 3, 3, 2);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0xc).lw(c);
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}


/// `void cUITextScroll_SetNavButtons(cUITextScroll *text, int enabled)`
/// (0x3a1310): store the four navigation-button indices at +0x88.
pub fn c_ui_text_scroll_set_nav_buttons(c: &mut Cpu) {
    c.cycles += 3;
    c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
    let enabled = c.g[A1][0] != 0;
    c.g[A2][0] = c.g[A0][0];
    let text = c.g[A2][0] as u32;
    if enabled {
        c.cycles += 9;
        c.g[V1][0] = 0x4;
        c.g[A0][0] = 0x5;
        c.g[A1][0] = 0x6;
        c.g[V0][0] = 0x7;
        Addr(text).off(0x8b).sb(c, c.g[V0][0]);
        Addr(text).off(0x88).sb(c, c.g[V1][0]);
        Addr(text).off(0x89).sb(c, c.g[A0][0]);
        Addr(text).off(0x8a).sb(c, c.g[A1][0]);
        abi::ret(c);
        return;
    }
    c.cycles += 8;
    c.g[V0][0] = 0x1;
    c.g[A0][0] = 0x2;
    c.g[V1][0] = 0x3;
    Addr(text).off(0x89).sb(c, c.g[V0][0]);
    Addr(text).off(0x8b).sb(c, c.g[V1][0]);
    Addr(text).off(0x8a).sb(c, c.g[A0][0]);
    Addr(text).off(0x88).sb(c, 0);
    abi::ret(c);
}


/// `void *cUITextScroll_GetLine(cUITextScroll *text, int index)`
/// (0x3a2068): the line pointer at `text + index*4`, or null when out of range.
pub fn c_ui_text_scroll_get_line(c: &mut Cpu) {
    c.cycles += 3;
    let text = abi::arg(c, 0);
    c.g[A2][0] = Addr(text).off(0xbc).lw(c);
    let lines_present = c.g[A2][0] != 0;
    if lines_present {
        c.cycles += 2;
        if (c.g[A1][0] as i64) >= 0 {
            c.cycles += 4;
            c.g[V0][0] = Addr(text).off(0xc8).lhu(c);
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let out_of_range = c.g[V0][0] == 0;
            c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
            if !out_of_range {
                c.cycles += 3;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A2][0] as u32));
                c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                abi::ret(c);
                return;
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// UIText_SetWideString / UIPairFlags_ctor / MPEG2_picture_temporal_scalable_extension.

/// `void UIText_SetWideString(UIText* t, char* str)` (0x3a0e90): clear the
/// cached byte length (`+0xb0`) and install a new wide string.
pub fn ui_text_set_wide_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(r32(c, V0).wrapping_add(0xb0)).sw(c, 0);
        callee::UITEXT_SET_WIDE_STRING_INTERNAL.call(c, 0x003a_0ea4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// UI text.

/// `UStr UIText_GetOwnedString(UIText *this)` (0x3a0c00): the owned string at
/// `+0xb4`, or 0 when bit 3 of the flags at `+0x74` is set.
pub fn ui_text_get_owned_string(c: &mut Cpu) {
    c.cycles += 5;
    let this = c.g[A0][0] as u32;
    let flags = Addr(this.wrapping_add(0x74)).lw(c);
    c.g[V0][0] = flags;
    c.g[V0][0] = sx(((flags as i32) >> 3) as u32);
    c.g[V0][0] &= 1;
    let owned = c.g[V0][0] != 0;
    c.g[V0][0] = 0;
    if owned {
        c.cycles += 2;
        abi::ret(c);
    } else {
        c.cycles += 2;
        c.g[V0][0] = Addr(this.wrapping_add(0xb4)).lw(c);
        abi::ret(c);
    }
}


/// `UStr UIText_GetIdString(UIText *this)` (0x3a0c28): the id string at
/// `+0xb4`, or 0 when bit 3 of the flags at `+0x74` is clear.
pub fn ui_text_get_id_string(c: &mut Cpu) {
    c.cycles += 5;
    let this = c.g[A0][0] as u32;
    let flags = Addr(this.wrapping_add(0x74)).lw(c);
    c.g[V0][0] = flags;
    c.g[V0][0] = sx(((flags as i32) >> 3) as u32);
    c.g[V0][0] &= 1;
    let owned = c.g[V0][0] == 0;
    c.g[V0][0] = 0;
    if owned {
        c.cycles += 2;
        abi::ret(c);
    } else {
        c.cycles += 2;
        c.g[V0][0] = Addr(this.wrapping_add(0xb4)).lw(c);
        abi::ret(c);
    }
}


/// `void cUITextScroll_ReplaceWideString(cUITextScroll* self, const
/// unsigned short* str)` (0x3A1CF0): drop the current lines and set the new
/// wide string.
pub fn ui_text_scroll_replace_wide_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        callee::UI_TEXT_SCROLL_CLEAR_LINES.call(c, 0x003a_1d0c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        callee::UI_TEXT_SCROLL_SET_WIDE_STRING.call(c, 0x003a_1d18)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void UIText_FreeStrings(UIText* t)` (0x3a0b38): clear the string count and
/// free the two string buffers at `+0xac` and `+0xb4`.
pub fn ui_text_free_strings(c: &mut Cpu) {
    c.cycles += 8;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    c.sw(p(c, S0, 0xa8), 0);
    c.g[A0][0] = Addr(p(c, S0, 0xac)).lw(c);
    if c.g[A0][0] != 0 {
        c.cycles += 4;
        if call_ptr(c, table::EA_MEM_FREE, 0x003a_0b60).is_err() {
            return;
        }
        c.sw(p(c, S0, 0xac), 0);
    }
    c.g[A0][0] = Addr(p(c, S0, 0xb4)).lw(c);
    c.cycles += 2;
    if c.g[A0][0] != 0 {
        c.cycles += 4;
        if call_ptr(c, table::EA_MEM_FREE, 0x003a_0b78).is_err() {
            return;
        }
        c.sw(p(c, S0, 0xb4), 0);
    }
    frame.restore(c, S0, 0x10);
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
}


/// `void UIVector_SetColor(UIVector* v, const Color* c)` (0x003a3398):
/// copy the colour to +0x1c and fill every quad of the array at +0x7c with
/// it, scaled to bytes (count = `+0x74 >> 17`).
pub fn ui_vector_set_color(c: &mut Cpu) {
    c.cycles += 21;
    let v = abi::arg(c, 0);
    let col = abi::arg(c, 1);
    c.g[T0][0] = sx(v);
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x10));
    // *c -> stack, and *c -> v+0x1c
    c.ldl(2, (col).wrapping_add(0x7));
    c.ldr(2, col);
    c.ldl(3, (col).wrapping_add(0xf));
    c.ldr(3, (col).wrapping_add(0x8));
    c.sdl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
    c.sdr(2, c.g[SP][0] as u32);
    c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
    c.sdr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
    c.ldl(9, (col).wrapping_add(0x7));
    c.ldr(9, col);
    c.ldl(2, (col).wrapping_add(0xf));
    c.ldr(2, (col).wrapping_add(0x8));
    c.sdl(9, (c.g[T0][0] as u32).wrapping_add(0x23));
    c.sdr(9, (c.g[T0][0] as u32).wrapping_add(0x1c));
    c.sdl(2, (c.g[T0][0] as u32).wrapping_add(0x2b));
    c.sdr(2, (c.g[T0][0] as u32).wrapping_add(0x24));
    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x7c));
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        abi::ret(c);
        return;
    }
    c.cycles += 34;
    c.g[AT][0] = 0x437f_0000; // 255.0f
    c.mtc1(1, 0);
    c.g[A3][0] = 0;
    c.lwc1(3, (col).wrapping_add(0x4));
    c.lwc1(1, col);
    c.lwc1(2, (col).wrapping_add(0x8));
    c.mul_s(3, 3, 0);
    c.lwc1(4, (col).wrapping_add(0xc));
    c.mul_s(1, 1, 0);
    c.mul_s(2, 2, 0);
    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x74));
    c.mul_s(4, 4, 0);
    c.cvt_w_s(0, 3);
    c.mfc1(3, 0);
    c.g[V0][0] = sx((c.g[V0][0] as u32) >> 17);
    c.cvt_w_s(0, 1);
    c.mfc1(6, 0);
    c.cvt_w_s(0, 2);
    c.mfc1(4, 0);
    c.cvt_w_s(0, 4);
    c.mfc1(5, 0);
    c.mtc1(6, 0);
    c.cvt_s_w(0, 0);
    c.mtc1(3, 1);
    c.cvt_s_w(1, 1);
    c.mtc1(4, 2);
    c.cvt_s_w(2, 2);
    c.mtc1(5, 3);
    c.cvt_s_w(3, 3);
    c.swc1(0, c.g[SP][0] as u32);
    c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x4));
    c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x8));
    c.swc1(3, (c.g[SP][0] as u32).wrapping_add(0xc));
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x7c));
    loop {
        c.cycles += 16;
        c.g[V1][0] = sx((c.g[A3][0] as u32) << 4);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x1));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
        c.ldr(2, c.g[SP][0] as u32);
        c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0xf));
        c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
        c.sdr(2, c.g[V1][0] as u32);
        c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
        c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x74));
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 17);
        c.g[V0][0] = (c.g[A3][0] < c.g[V0][0]) as u64;
        if c.g[V0][0] == 0 {
            break;
        }
        c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x7c));
        if event_due(c) {
            deliver_event_here(c, 0x003a_3478);
        }
    }
    c.cycles += 2;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    abi::ret(c);
}


/// `cUITextScroll* cUITextScroll_ctor(cUITextScroll* s, int a1, int a2)`
/// (0x3a1148).
pub fn c_ui_text_scroll_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x20);
        sqsp(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_OBJECT_CTOR.call(c, 0x003a_115c)?;
        c.g[V1][0] = Addr(p_2(r32(c, S0), 0x74)).lbu(c);
        c.g[V0][0] = 0x0049_4278;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8c));
        c.g[V1][0] = c.g[V1][0] & 0xc6;
        Addr(p_2(r32(c, S0), 8)).sw(c, c.g[V0][0]);
        Addr(p_2(r32(c, S0), 0x74)).sb(c, c.g[V1][0]);
        callee::UI_LIST_CTOR.call(c, 0x003a_117c)?;
        Addr(p_2(r32(c, S0), 0xb8)).sw(c, 0);
        c.g[A0][0] = c.g[S0][0];
        Addr(p_2(r32(c, S0), 0xbc)).sw(c, 0);
        c.g[A1][0] = 0;
        Addr(p_2(r32(c, S0), 0xc0)).sw(c, 0);
        Addr(p_2(r32(c, S0), 0xc4)).sh(c, 0);
        Addr(p_2(r32(c, S0), 0xc6)).sh(c, 0);
        Addr(p_2(r32(c, S0), 0xc8)).sh(c, 0);
        Addr(p_2(r32(c, S0), 0xcc)).sw(c, 0);
        callee::CUI_TEXT_SCROLL_SET_NAV_BUTTONS.call(c, 0x003a_11a4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        lqsp(c, 16, 0x10);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}


/// `UIVector* UIVector_ctor(UIVector* v, int a1, int a2)` (0x3a3280).
pub fn ui_vector_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x20);
        sqsp(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_OBJECT_CTOR.call(c, 0x003a_3294)?;
        c.g[V1][0] = Addr(p_2(r32(c, S0), 0x74)).lw(c);
        c.g[A1][0] = 0xffff_ffff_ffff_fffe;
        c.g[A2][0] = 0xffff_ffff_ffff_fffd;
        c.g[V0][0] = 0x1_0000;
        c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        c.g[V1][0] = c.g[V1][0] & c.g[A2][0];
        c.g[A0][0] = 0x0049_41b8;
        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
        Addr(p_2(r32(c, S0), 8)).sw(c, c.g[A0][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(p_2(r32(c, S0), 0x74)).sw(c, c.g[V1][0]);
        for off in [0x78u32, 0x7c, 0x80, 0x84, 0x88, 0x8c] {
            Addr(p_2(r32(c, S0), off)).sw(c, 0);
        }
        frame.restore_ra(c, 0);
        lqsp(c, 16, 0x10);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}


/// `void cUITextScroll_SetTextId(self, id)` (0x3a1f18): store the text id, look
/// the widget up through `+0x5c/+0xd0/+0x10` and, when present, ask it for its
/// wide string and set it, then re-wrap.
pub fn cui_text_scroll_set_text_id(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x20));
        c.sq(16, sp_2(c).wrapping_add(0x10));
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        sw_r(c, S0, 0xc0, c.g[A1][0]);
        c.g[V0][0] = lw_r(c, S0, 0x5c);
        c.g[V1][0] = lw_r(c, V0, 0xd0);
        c.g[A0][0] = lw_r(c, V1, 0x10);
        c.g[A2][0] = lw_r(c, A0, 0x10);
        if c.g[A2][0] == 0 {
            c.lq(16, sp_2(c).wrapping_add(0x10));
        } else {
            c.cycles += 7;
            c.g[V1][0] = lw_r(c, A2, 0x4);
            c.g[A0][0] = lh_r(c, V1, 0x20);
            c.g[V0][0] = lw_r(c, V1, 0x24);
            let target = low(c, V0);
            c.g[A0][0] = sx(low(c, A2).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x003a_1f58)?;
            let empty = c.g[V0][0] == 0;
            c.g[A1][0] = c.g[V0][0];
            if !empty {
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                callee::CUI_TEXT_SCROLL_SET_WIDE_STRING.call(c, 0x003a_1f68)?;
            }
            c.cycles += 7;
            c.g[V0][0] = lw_r(c, S0, 0x8);
            c.g[A1][0] = 0x1;
            c.g[A0][0] = lh_r(c, V0, 0x28);
            c.g[V1][0] = lw_r(c, V0, 0x2c);
            let target = low(c, V1);
            c.g[A0][0] = sx(low(c, S0).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x003a_1f80)?;
            c.lq(16, sp_2(c).wrapping_add(0x10));
        }
        c.cycles += 3;
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}


/// `void cUITextScroll_RefreshText(self)` (0x3a1360): re-resolve the text id
/// (`self+0xc0`) through the UI object chain and replace the wide string,
/// then relayout the widget.
pub fn cuitextscroll_refresh_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        let id = lw_at(c, p_3(self_, 0xc0));
        if id == 0 {
            frame.restore(c, 16, 0x10);
        } else {
            c.cycles += 6;
            c.g[V0][0] = lw_at(c, p_3(self_, 0x5c));
            c.g[V1][0] = lw_at(c, p_3(c.g[V0][0] as u32, 0xd0));
            c.g[A0][0] = lw_at(c, p_3(c.g[V1][0] as u32, 0x10));
            c.g[A2][0] = lw_at(c, p_3(c.g[A0][0] as u32, 0x10));
            if c.g[A2][0] == 0 {
                frame.restore(c, 16, 0x10);
            } else {
                c.cycles += 7;
                c.g[V1][0] = lw_at(c, p_3(c.g[A2][0] as u32, 4));
                let func = vslot(c, c.g[V1][0], 0x20, V0);
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, func, 0x003a_13a8)?;
                let text = c.g[V0][0];
                c.g[A1][0] = text;
                if text != 0 {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = text;
                    call_ptr(c, 0x003a_1cf0, 0x003a_13b8)?;
                    c.g[A1][0] = text;
                }
                c.cycles += 7;
                c.g[V0][0] = lw_at(c, p_3(r32(c, S0), 8));
                c.g[A1][0] = 1;
                let func = vslot(c, c.g[V0][0], 0x28, V1);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, func, 0x003a_13d0)?;
                frame.restore(c, 16, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cUITextScroll_AddLine(cUITextScroll* self, char16* line)`
/// (0x3a1bf8): grow the line array, duplicate the string and append it.
pub fn c_ui_text_scroll_add_line(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        open(c, 0x50);
        let sp0 = spv(c);
        c.sq(19, at(sp0, 0x10));
        c.sq(18, at(sp0, 0x20));
        c.g[S3][0] = c.g[A0][0];
        c.sq(16, at(sp0, 0x40));
        c.sq(17, at(sp0, 0x30));
        c.sd(sp0, c.g[RA][0]);
        c.g[S2][0] = c.g[A1][0];
        callee::GROW_LINES.call(c, 0x003a_1c1c)?;
        let this = c.g[S3][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[S0][0] = c.g[V0][0];
        call_ptr(c, callee::USTR_LEN, 0x003a_1c28)?;
        c.g[V0][0] = sx(at(c.g[V0][0] as u32, 1));
        c.g[A1][0] = 0x490000;
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        c.g[A0][0] = sx((c.g[V0][0] as u32) << 1);
        c.g[A1][0] = sx(at(c.g[A1][0] as u32, 0x40b8));
        call_ptr(c, callee::OPERATOR_NEW_NAMED_2, 0x003a_1c44)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::USTR_CPY, 0x003a_1c54)?;
        c.g[V0][0] = lw_at_2(c, this, 0xbc);
        c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
        c.lq(18, at(sp0, 0x20));
        c.g[S0][0] = sx(at(c.g[S0][0] as u32, c.g[V0][0] as u32));
        c.lq(19, at(sp0, 0x10));
        c.sw(c.g[S0][0] as u32, c.g[S1][0]);
        c.lq(16, at(sp0, 0x40));
        c.lq(17, at(sp0, 0x30));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at(sp0, 0x50));
        c.pc = ra;
        Ok(())
    })
}



// ---------------------------------------------------------------------------
// UI text.

/// `void cUIText_setAsciiString(cUIText* self, const char* str)` (0x3a0c70):
/// clear the text's animations, then convert up to 511 ASCII characters into
/// the stack buffer and set them.
pub fn ui_text_set_ascii_string(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x430);
        frame.save(c, S0, 0x420);
        frame.save(c, S1, 0x410);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x400);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = 1;
        c.g[V0][0] = Addr(p_4(r32(c, S0), 8)).lw(c);
        c.g[A0][0] = Addr(p_4(r32(c, V0), 0x28)).lh(c);
        c.g[V1][0] = Addr(p_4(r32(c, V0), 0x2c)).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx(p_4(r32(c, S0), r32(c, A0)));
        call_ptr(c, target, 0x003a_0ca0)?;
        c.g[V0][0] = Addr(p_4(r32(c, S0), 8)).lw(c);
        c.g[A1][0] = 0;
        c.g[A0][0] = Addr(p_4(r32(c, V0), 0xc0)).lh(c);
        c.g[V1][0] = Addr(p_4(r32(c, V0), 0xc4)).lw(c);
        let target = r32(c, V1);
        c.g[A0][0] = sx(p_4(r32(c, S0), r32(c, A0)));
        call_ptr(c, target, 0x003a_0cb8)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0041_6810, 0x003a_0cc0)?;
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        c.g[V0][0] = ((c.g[V0][0] as i64) < 512i64) as u64;
        if c.g[V0][0] != 0 {
            c.cycles += 7;
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = c.g[SP][0];
            callee::UI_ASCII_TO_WIDE.call(c, 0x003a_0cdc)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[SP][0];
            callee::UI_TEXT_SET_WIDE_STRING_INTERNAL.call(c, 0x003a_0ce8)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x420);
        frame.restore(c, S1, 0x410);
        frame.restore_ra(c, 0x400);
        frame.close(c);
        Ok(())
    })
}


/// `cUITextScroll_ClearLines` (0x3a18c0): free every stored line string,
/// null the line array and reset the counters.
pub fn ui_text_scroll_clear_lines(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(p_5(this, 0xbc)).lw(c);
        if c.g[V0][0] == 0 {
            Addr(p_5(this, 0xc8)).sh(c, 0);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(p_5(this, 0xc8)).lhu(c);
            let none = c.g[V0][0] == 0;
            c.g[S0][0] = 0;
            if !none {
                c.cycles += 1;
                c.g[V1][0] = Addr(p_5(this, 0xbc)).lw(c);
                loop {
                    c.cycles += 5;
                    c.g[V0][0] = shl32(c.g[S0][0], 2);
                    c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
                    c.g[A0][0] = Addr(r(c, V0)).lw(c);
                    let zero = c.g[A0][0] == 0;
                    c.g[V1][0] = sx(r(c, S0).wrapping_add(1));
                    if !zero {
                        c.cycles += 3;
                        c.g[RA][0] = sx(0x003a_190c);
                        callee::EA_MEM_FREE_2.call(c, 0x003a_190c)?;
                        c.g[V1][0] = sx(r(c, S0).wrapping_add(1));
                    }
                    c.cycles += 5;
                    c.g[V0][0] = Addr(p_5(this, 0xc8)).lhu(c);
                    c.g[S0][0] = c.g[V1][0] & 0xffff;
                    c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    c.g[V1][0] = Addr(p_5(this, 0xbc)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x003a_18f0);
                    }
                }
            }
            c.cycles += 3;
            c.g[A0][0] = Addr(p_5(this, 0xbc)).lw(c);
            if c.g[A0][0] != 0 {
                c.cycles += 3;
                c.g[RA][0] = sx(0x003a_1938);
                callee::EA_MEM_FREE_2.call(c, 0x003a_1938)?;
            }
            Addr(p_5(this, 0xbc)).sw(c, 0);
            c.cycles += 1;
            Addr(p_5(this, 0xc8)).sh(c, 0);
        }
        c.cycles += 6;
        Addr(p_5(this, 0xc6)).sh(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Text scroll.

/// `int cUITextScroll_GrowLines(cUITextScroll* self)` (0x3a1958): grow the
/// line-pointer array (`+0xbc`, count `+0xc8`) by one, keep the old entries,
/// and return the old count.
pub fn c_ui_text_scroll_grow_lines(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = 0x0049_0000;
        frame.save(c, S1, 0x30);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x40a8));
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        c.g[A2][0] = 0x100;
        frame.save_ra(c, 0);
        c.g[A3][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[S3][0] = Addr(p_6(c.g[S1][0] as u32, 0xc8)).lhu(c);
        c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_add(1));
        c.sh(p_6(c.g[S1][0] as u32, 0xc8), c.g[V0][0]);
        c.g[A0][0] = c.g[V0][0] & 0xffff;
        c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
        call_ptr(c, 0x0031_7e30, 0x003a_199c)?; // operator new
        c.g[A1][0] = lw(c, p_6(c.g[S1][0] as u32, 0xbc));
        let none = c.g[A1][0] == 0;
        c.g[S2][0] = c.g[V0][0];
        if !none {
            c.cycles += 8;
            c.g[S0][0] = sx((c.g[S3][0] as u32) << 2);
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x0041_605c, 0x003a_19b8)?; // memcpy
            c.g[A0][0] = lw(c, p_6(c.g[S1][0] as u32, 0xbc));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
            let gone = c.g[A0][0] == 0;
            sw(c, c.g[S0][0] as u32, 0);
            if !gone {
                c.cycles += 2;
                call_ptr(c, 0x0031_7e98, 0x003a_19d0)?; // EA_MemFree
            }
        }
        c.cycles += 9;
        sw(c, p_6(c.g[S1][0] as u32, 0xbc), c.g[S2][0]);
        c.g[V0][0] = c.g[S3][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void UIVector_dtor(self, flags)` (0x3a32f0): free the vector's five
/// buffers and destroy the UI object.
pub fn ui_vector_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = sx(0x0049_41b8);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p_7(r32(c, S0), 8)).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(p_7(r32(c, S0), 0x7c)).lw(c);
        let null = c.g[A0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !null {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e98, 0x003a3324)?;
        }
        c.cycles += 3;
        c.g[A0][0] = Addr(p_7(r32(c, S0), 0x80)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x003a3338)?;
            c.g[A0][0] = Addr(p_7(r32(c, S0), 0x84)).lw(c);
        } else {
            c.g[A0][0] = Addr(p_7(r32(c, S0), 0x84)).lw(c);
        }
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x003a334c)?;
            c.g[A0][0] = Addr(p_7(r32(c, S0), 0x88)).lw(c);
        } else {
            c.g[A0][0] = Addr(p_7(r32(c, S0), 0x88)).lw(c);
        }
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x003a3360)?;
            c.g[A0][0] = Addr(p_7(r32(c, S0), 0x8c)).lw(c);
        } else {
            c.g[A0][0] = Addr(p_7(r32(c, S0), 0x8c)).lw(c);
        }
        c.cycles += 2;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call_ptr(c, 0x0031_7e98, 0x003a3374)?;
            c.g[A0][0] = c.g[S0][0];
        } else {
            c.g[A0][0] = c.g[S0][0];
        }
        c.cycles += 7;
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0039_fc48, 0x003a3380)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// `UIText_GetTextSize` (0x003a1030).
//
// `void UIText::GetTextSize(self, V2* out)`: return the measured text size,
// from the layout object when it is available and from the font otherwise.

pub fn ui_text_get_text_size(c: &mut Cpu) {
    guest(c, |c| {

        'b_3a1078: {
        'b_3a1064: {
        c.cycles += 8;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S0][0] = c.g[A0][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xb4));
        let t = c.g[V0][0] == 0u64;
        c.g[S1][0] = c.g[A1][0];
        if t {
        break 'b_3a1064;
        } else {
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
        c.g[V0][0] = c.g[V0][0] & 0x1;
        let t = c.g[V0][0] != 0u64;

        if t {
        break 'b_3a1078;
        } else {
        break 'b_3a1064;
        }
        }
        }
        c.cycles += 5;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = vslot_2(c, c.g[V0][0], 0x80, V1);
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x003a1078);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x003a1078)?;
        break 'b_3a1078;
        }
        'b_3a10dc: {
        'b_3a10c0: {
        c.cycles += 5;
        c.g[RA][0] = sx(0x003a1080);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003a04f0, 0x003a1080)?;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xb4));
        if c.g[A1][0] == 0u64 {
        c.g[V0][0] = 0x500000;
        break 'b_3a10c0;
        } else {
        'b_3a10bc: {
        c.cycles += 2;
        let t = c.g[V0][0] == 0u64;
        c.g[A0][0] = c.g[V0][0];
        if t {
        break 'b_3a10bc;
        } else {
        c.cycles += 10;
        c.lwc1(13, (c.g[S0][0] as u32).wrapping_add(0x54));
        c.lwc1(12, (c.g[S0][0] as u32).wrapping_add(0x50));
        c.g[A2][0] = c.g[SP][0];
        c.g[RA][0] = sx(0x003a10a8);
        c.g[A3][0] = 0u64;
        call_ptr(c, 0x003921f0, 0x003a10a8)?;
        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0xc));
        c.swc1(1, (c.g[S1][0] as u32));
        let t = true;
        c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
        if t {
        break 'b_3a10dc;
        } else {
        break 'b_3a10bc;
        }
        }
        }
        c.cycles += 1;
        c.g[V0][0] = 0x500000;
        break 'b_3a10c0;
        }
        }
        c.cycles += 7;
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xf28));
        c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
        c.ldr(3, (c.g[A2][0] as u32));
        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
        c.sdl(3, (c.g[S1][0] as u32).wrapping_add(0x7));
        c.sdr(3, (c.g[S1][0] as u32));
        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
        break 'b_3a10dc;
        }
        c.cycles += 5;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t; return Ok(());

        Ok(())
    })
}


// ---------------------------------------------------------------------------
// 0x003a13e0 cUITextScroll_HandleInput
// ---------------------------------------------------------------------------

/// `int cUITextScroll_HandleInput(cUITextScroll* self, void* input)` (0x3a13e0).
///
/// Keyboard handling of a scrolling text box. The row count comes from the
/// box height divided by the scaled font height plus three; then, depending on
/// the "cursor mode" flag, either the cursor moves within the list (and the
/// visible top follows it) or the visible top scrolls and the cursor stays
/// put. Returns 1 when a key was consumed.
pub fn cuit_ext_scroll_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x20);
        frame.save(c, 18, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x003a_1400);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x003a_12d0, 0x003a_1400)?; // cUITextScroll_GetFont
        let self_ = c.g[S0][0] as u32;
        let input = c.g[S1][0] as u32;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
            c.cycles += 6;
            frame.restore(c, 16, 0x30);
            frame.restore(c, 17, 0x20);
            frame.restore(c, 18, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        let font = c.g[V0][0] as u32;

        'ret: {
            'ret0: {
                'ret1: {
                    'scroll: {
                        c.cycles += 18;
                        c.lwc1(2, p_9(font, 0x14));
                        c.cvt_s_w(2, 2);
                        c.lwc1(3, p_9(font, 0x34));
                        c.lwc1(0, p_9(self_, 0x54));
                        c.g[AT][0] = 0x4040_0000; // 3.0
                        c.mtc1(1, 4);
                        c.mul_s(2, 2, 3);
                        c.lwc1(1, p_9(self_, 0x64));
                        c.g[V1][0] = c.lw(p_9(self_, 0x74));
                        c.mul_s(0, 0, 2);
                        c.g[V1][0] = c.g[V1][0] & 1;
                        c.add_s(0, 0, 4);
                        c.div_s(1, 1, 0);
                        c.cvt_w_s(0, 1);
                        c.mfc1(2, 0);
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 24);
                        let cursor_mode = c.g[V1][0] == 0;
                        c.g[S2][0] = sx((((c.g[V0][0] as u32) as i32) >> 24) as u32);
                        let rows = c.g[S2][0] as i32;

                        if cursor_mode {
                            break 'scroll;
                        }
                        // --- cursor mode: up ---------------------------------
                        c.cycles += 8;
                        c.g[V1][0] = c.lw(p_9(input, 8));
                        c.g[A1][0] = c.lb(p_9(self_, 0x88));
                        c.g[A0][0] = c.lh(p_9(c.g[V1][0] as u32, 0x10));
                        c.g[V0][0] = c.lw(p_9(c.g[V1][0] as u32, 0x14));
                        let f = c.g[V0][0] as u32;
                        c.g[A0][0] = sx(input.wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, f, 0x003a_1468)?;
                        if c.g[V0][0] != 0 {
                            c.cycles += 3;
                            c.g[V0][0] = c.lhu(p_9(self_, 0xc4));
                            let empty = c.g[V0][0] == 0;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                            if empty {
                                break 'ret1;
                            }
                            c.cycles += 2;
                            c.sh(p_9(self_, 0xc4), c.g[V0][0]);
                            break 'ret1;
                        }
                        // --- cursor mode: down -------------------------------
                        c.g[V1][0] = c.lw(p_9(input, 8));
                        c.cycles += 7;
                        c.g[A1][0] = c.lb(p_9(self_, 0x89));
                        c.g[A0][0] = c.lh(p_9(c.g[V1][0] as u32, 0x10));
                        c.g[V0][0] = c.lw(p_9(c.g[V1][0] as u32, 0x14));
                        let f = c.g[V0][0] as u32;
                        c.g[A0][0] = sx(input.wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, f, 0x003a_1498)?;
                        if c.g[V0][0] != 0 {
                            c.cycles += 6;
                            c.g[V0][0] = c.lhu(p_9(self_, 0xc8));
                            c.g[V1][0] = c.lhu(p_9(self_, 0xc4));
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let last = c.g[V0][0] == 0;
                            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                            if last {
                                break 'ret1;
                            }
                            c.cycles += 2;
                            c.sh(p_9(self_, 0xc4), c.g[V0][0]);
                            break 'ret1;
                        }
                        // Neither key: keep the cursor visible.
                        c.g[V1][0] = c.lhu(p_9(self_, 0xc4));
                        c.cycles += 6;
                        c.g[V0][0] = c.lhu(p_9(self_, 0xc6));
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32) << 16);
                        c.g[V0][0] = sx((((c.g[V0][0] as u32) as i32) >> 16) as u32);
                        let above = (c.g[V0][0] as i64) >= 0;
                        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                        if !above {
                            c.cycles += 2;
                            c.sh(p_9(self_, 0xc6), c.g[V1][0]);
                            break 'ret0;
                        }
                        c.cycles += 2;
                        let visible = c.g[V0][0] != 0;
                        c.g[V0][0] = 0;
                        if visible {
                            break 'ret;
                        }
                        c.cycles += 4;
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[S2][0] as u32));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                        c.sh(p_9(self_, 0xc6), c.g[V0][0]);
                        let _ = rows;
                        break 'ret0;
                    }
                    // --- scroll mode: up ------------------------------------
                    c.cycles += 8;
                    c.g[V1][0] = c.lw(p_9(input, 8));
                    c.g[A1][0] = c.lb(p_9(self_, 0x88));
                    c.g[A0][0] = c.lh(p_9(c.g[V1][0] as u32, 0x10));
                    c.g[V0][0] = c.lw(p_9(c.g[V1][0] as u32, 0x14));
                    let f = c.g[V0][0] as u32;
                    c.g[A0][0] = sx(input.wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, f, 0x003a_1510)?;
                    if c.g[V0][0] != 0 {
                        c.cycles += 3;
                        c.g[V0][0] = c.lhu(p_9(self_, 0xc6));
                        let empty = c.g[V0][0] == 0;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                        if empty {
                            break 'ret1;
                        }
                        c.cycles += 2;
                        c.sh(p_9(self_, 0xc6), c.g[V0][0]);
                        break 'ret1;
                    }
                    // --- scroll mode: down ----------------------------------
                    c.g[V1][0] = c.lw(p_9(input, 8));
                    c.cycles += 7;
                    c.g[A1][0] = c.lb(p_9(self_, 0x89));
                    c.g[A0][0] = c.lh(p_9(c.g[V1][0] as u32, 0x10));
                    c.g[V0][0] = c.lw(p_9(c.g[V1][0] as u32, 0x14));
                    let f = c.g[V0][0] as u32;
                    c.g[A0][0] = sx(input.wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, f, 0x003a_1540)?;
                    let down = c.g[V0][0] == 0;
                    c.g[V0][0] = 0;
                    if down {
                        break 'ret;
                    }
                    c.cycles += 6;
                    c.g[V0][0] = c.lhu(p_9(self_, 0xc8));
                    c.g[V1][0] = c.lhu(p_9(self_, 0xc6));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S2][0] as u32));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let full = c.g[V0][0] == 0;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                    if full {
                        break 'ret1;
                    }
                    c.cycles += 1;
                    c.sh(p_9(self_, 0xc6), c.g[V0][0]);
                    break 'ret1;
                }
                // Return 1.
                c.cycles += 2;
                c.g[V0][0] = 1;
                break 'ret;
            }
            // Return 0.
            c.cycles += 1;
            c.g[V0][0] = 0;
            break 'ret;
        }

        c.cycles += 6;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
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
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fakes;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::put_vec;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fill;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::RA as RET;
    use ssx3_ee as ee;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use ssx3_ee_rt::funcs::GuestFn;
    use crate::difftest::elf_image;
    use crate::difftest::fakes::covered;
    pub(crate) use crate::game::bx::bxstring2::tests::be_library;
    pub(crate) use crate::game::bx::bxstring2::tests::SCRATCH;
    pub(crate) use crate::game::fe::festaterewards2::tests::w;
    pub(crate) use crate::game::fe::festaterewards2::tests::belib;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::MEM_HEAP;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::ARENA;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::ARENA_SIZE;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::run;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::heap_init;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::alloc;
    pub(crate) use crate::game::world::worldview::tests::SEEDS_5 as SEEDS;
    pub(crate) use crate::game::world::worldview::tests::word;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::be::beintplayer::tests::w8;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_3;
    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_2;
    pub(crate) use crate::game::main::gamemode::tests::slot;
    pub(crate) use crate::game::ui::uianimation::tests::pool_allocator;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::BE;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::WORLD;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_4 as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_3 as W;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_8 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::eafile_pools;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_7 as run_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_8 as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_9 as pool_allocator_2;
    pub(crate) use crate::game::input::inputmap::tests::alloc as alloc_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_3 as alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::BE_3;
    pub(crate) use crate::game::wscript::wscriptman::tests::WORLD_3 as WORLD_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::GAME;
    pub(crate) use crate::game::wscript::wscriptman::tests::globals_3 as globals_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_3;
    pub(crate) use crate::game::bx::bxstring::tests::heap;
    pub(crate) use crate::game::dirtysock::tags2::tests::pool_7 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_3 as OBJ;
    pub(crate) use crate::game::dirtysock::tags2::tests::DST;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_8 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::util::menu::tests::wf_2 as wf;


    /// Shared setup for the two font getters (`UIText` / `cUITextScroll`).
    pub(crate) fn get_font_setup(c: &mut Cpu, seed: u32, at_74_is_word: bool) {
        let mut r = Rng(0x3a04_f000 ^ seed as u64);
        scramble(c, &mut r);
        c.g[A0][0] = DATA as u64;
        let fontlist = DATA + 0x100;
        let fonts = DATA + 0x200;
        let base = DATA + 0x400;
        c.bus.write32(DATA + 0x5c, fontlist);
        c.bus.write32(fontlist + 0xd0, fonts);
        if seed % 3 == 0 {
            c.bus.write32(fonts + 0x10, 0);
        } else {
            c.bus.write32(fonts + 0x10, fonts);
            c.bus.write32(fonts + 0x8, base);
        }
        if at_74_is_word {
            c.bus.write32(DATA + 0x74, (r.next_u32() << 3) | (r.next_u32() & 0x7));
        } else {
            c.bus.write8(DATA + 0x74, (r.next_u32() & 0xff) as u8);
        }
        for k in 0..4 {
            c.bus.write32(base + k * 12 + 0xc, r.next_u32());
        }
    }


    #[test]
    fn c_ui_text_scroll_get_font_matches() {
        check_lohi(ssx3_ee::s13::f_003a12d0, c_ui_text_scroll_get_font, 200, &|c, seed| {
            get_font_setup(c, seed, true)
        });
    }


    #[test]
    fn c_ui_text_scroll_set_nav_buttons_matches() {
        check(ssx3_ee::s13::f_003a1310, c_ui_text_scroll_set_nav_buttons, 200, &|c, seed| {
            let mut r = Rng(0x3a13_1000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = ((r.next_u32() as u64) << 32) | if seed % 2 == 0 { 0 } else { (seed % 7) as u64 + 1 };
            for k in 0x88..=0x8b {
                c.bus.write8(DATA + k, r.next_u32() as u8);
            }
        });
    }


    #[test]
    fn c_ui_text_scroll_get_line_matches() {
        check(ssx3_ee::s13::f_003a2068, c_ui_text_scroll_get_line, 300, &|c, seed| {
            let mut r = Rng(0x3a20_6800 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            let lines = DATA + 0x400;
            c.g[A1][0] = match seed % 4 {
                0 => (r.next_u32() as u64) | 0x8000_0000_0000_0000, // negative
                1 => 0,
                2 => 20, // out of range below
                _ => (seed % 10) as u64,
            };
            if seed % 5 == 0 {
                c.bus.write32(DATA + 0xbc, 0);
            } else {
                c.bus.write32(DATA + 0xbc, lines);
                for k in 0..12 {
                    c.bus.write32(lines + k * 4, r.next_u32());
                }
            }
            c.bus.write16(DATA + 0xc8, 10);
        });
    }


    #[test]
    fn ui_text_set_wide_string_matches() {
        crate::difftest::check(ee::s13::f_003a0e90__generated, ui_text_set_wide_string, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x3a0e_90, 2);
            be_library(c);
            c.g[A0][0] = sx(DATA);
            // The wide string: a zeroed run so the length/copy loops stop at
            // the first unit.
            c.g[A1][0] = sx(DATA + 0x1000);
            for k in 0..8u32 {
                c.bus.write32(DATA + 0x1000 + 4 * k, 0);
            }
            c.bus.write32(DATA + 8, if seed % 3 == 0 { 0 } else { DATA + 0x200 });
        });
    }


    #[test]
    fn ui_text_get_owned_string_matches() {
        check(ee::s13::f_003a0c00__generated, ui_text_get_owned_string, 120, &|c, seed| {
            let mut r = Rng(0x3a0c_0000 ^ seed as u64);
            crate::difftest::scramble(c, &mut r);
            c.g[A0][0] = SCRATCH as u64;
            c.bus.write32(SCRATCH + 0x74, if seed % 2 == 0 { 0 } else { 1 << 3 });
            c.bus.write32(SCRATCH + 0xb4, r.next_u32());
        });
    }


    #[test]
    fn ui_text_get_id_string_matches() {
        check(ee::s13::f_003a0c28__generated, ui_text_get_id_string, 120, &|c, seed| {
            let mut r = Rng(0x3a0c_0028 ^ seed as u64);
            crate::difftest::scramble(c, &mut r);
            c.g[A0][0] = SCRATCH as u64;
            c.bus.write32(SCRATCH + 0x74, if seed % 2 == 0 { 0 } else { 1 << 3 });
            c.bus.write32(SCRATCH + 0xb4, r.next_u32());
        });
    }


    /// A 16-bit word of test data.
    pub(crate) fn h(c: &mut Cpu, a: u32, v: u16) {
        c.bus.write16(a, v);
    }


    #[test]
    fn ui_text_scroll_replace_wide_string_matches() {
        check(ssx3_ee::s13::f_003a1cf0__generated, ui_text_scroll_replace_wide_string, 60, &|c, seed| {
            machine(c, seed, 0x3a1c_f0, 2);
            belib(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x400) as u64;
            match seed % 3 {
                0 => {}
                1 => h(c, DATA + 0x400, 0x41),
                _ => {
                    h(c, DATA + 0x400, 0x41);
                    h(c, DATA + 0x402, 0x42);
                    h(c, DATA + 0x404, 0);
                }
            }
        });
    }


    #[test]
    fn ui_text_free_strings_matches() {
        check(
            ssx3_ee::s13::f_003a0b38__generated,
            ui_text_free_strings,
            150,
            &|c, seed| {
                let _r = machine(c, seed, 0xa0b38, 0);
                heap_init(c);
                let this = DATA;
                if seed % 2 == 0 {
                    let a = alloc(c, 0x20);
                    c.bus.write32(this + 0xac, a);
                }
                if (seed >> 1) % 2 == 0 {
                    let b = alloc(c, 0x20);
                    c.bus.write32(this + 0xb4, b);
                }
                c.g[A0][0] = this as u64;
            },
        );
    }


    #[test]
    fn ui_vector_set_color_matches() {
        check(ssx3_ee::s13::f_003a3398__generated, ui_vector_set_color, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3a3398, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x100);
            for k in 0..4 {
                word(c, DATA + 0x100 + 4 * k, r.edge_f32());
            }
            let count = match seed % 5 {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                _ => 1 + (r.next_u32() % 4),
            };
            word(c, DATA + 0x74, count << 17);
            word(c, DATA + 0x7c, if seed % 7 == 0 { 0 } else { DATA + 0x200 });
            for k in 0..16 {
                word(c, DATA + 0x200 + 4 * k, r.next_u32());
            }
            word(c, DATA + 0x1c, r.next_u32());
        });
    }


    #[test]
    fn ui_vector_set_color_delivers_events() {
        use crate::difftest::events::schedule;
        check(ssx3_ee::s13::f_003a3398__generated, ui_vector_set_color, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x3a3398, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x100);
            for k in 0..4 {
                word(c, DATA + 0x100 + 4 * k, r.edge_f32());
            }
            word(c, DATA + 0x74, 6 << 17);
            word(c, DATA + 0x7c, DATA + 0x200);
            schedule(c, &mut r, seed, 0x003a_3398);
        });
    }


    #[test]
    fn c_ui_text_scroll_ctor_matches() {
        check(ssx3_ee::s13::f_003a1148__generated, c_ui_text_scroll_ctor, SEEDS_2, &|c, seed| {
            let mut r = machine(c, seed, 0x3a1148, 0);
            let s = DATA + 0x1000;
            c.g[A0][0] = s as u64;
            w32(c, s + 0x14, r.next_u32());
            w8(c, s + 0x74, r.next_u32() as u8);
        });
    }


    #[test]
    fn ui_vector_ctor_matches() {
        check(ssx3_ee::s13::f_003a3280__generated, ui_vector_ctor, SEEDS_2, &|c, seed| {
            let mut r = machine(c, seed, 0x3a3280, 0);
            let v = DATA + 0x1000;
            c.g[A0][0] = v as u64;
            w32(c, v + 0x14, r.next_u32());
            w32(c, v + 0x74, r.next_u32());
        });
    }


        #[test]
    fn cui_text_scroll_set_text_id_matches() {
        check(ee::s13::f_003a1f18__generated, cui_text_scroll_set_text_id, SEEDS_3, &|c, seed| {
            let mut r = machine(c, seed, 0x3a1f18, 3);
            be_library_2(c);
            let obj = DATA;
            c.g[A0][0] = obj as u64;
            c.g[A1][0] = r.next_u32() as u64;
            w(c, obj + 0x5c, DATA + 0x1000);
            w(c, DATA + 0x1000 + 0xd0, DATA + 0x1100);
            w(c, DATA + 0x1100 + 0x10, DATA + 0x1200);
            let present = seed % 2 == 0;
            w(c, DATA + 0x1200 + 0x10, if present { DATA + 0x1300 } else { 0 });
            let vt = DATA + 0x3000;
            w(c, DATA + 0x1300 + 4, vt);
            slot(c, vt, 0x20, 0x30, FAKE);
            w(c, obj + 8, vt);
            slot(c, vt, 0x28, 0x40, FAKE + 0x40);
        });
    }


    #[test]
    fn c_ui_text_scroll_add_line_matches() {
        check(
            ee::s13::f_003a1bf8__generated,
            c_ui_text_scroll_add_line,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x3a1bf8, 3);
                pool_allocator(c);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x400);
                // A short UTF-16 line.
                let n = 1 + seed % 4;
                for i in 0..n {
                    c.bus.write16(DATA + 0x400 + 2 * i, b'a' as u16 + i as u16);
                }
                c.bus.write16(DATA + 0x400 + 2 * n, 0);
                // An empty line table: GrowLines allocates it.
                c.bus.write32(DATA + 0xbc, 0);
                c.bus.write16(DATA + 0xc8, 0);
            },
        );
    }


        #[test]
    fn ui_text_set_ascii_string_matches() {
        check(ee::s13::f_003a0c70__generated, ui_text_set_ascii_string, 60, &|c, seed| {
            let mut r = base(c, seed, 0x3a_0c70);
            globals(c);
            let vt = vtable(c, &mut r, &[0x28, 0xc0]);
            w(c, DATA + 8, vt);
            // A short ASCII string.
            let n = (seed % 20) as u32;
            for i in 0..n {
                c.bus.write8(DATA + 0x200 + i, b'a' + (i % 26) as u8);
            }
            c.bus.write8(DATA + 0x200 + n, 0);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x200);
        });
    }


    #[test]
    fn ui_text_scroll_clear_lines_matches() {
        check(
            ee::s13::f_003a18c0__generated,
            ui_text_scroll_clear_lines,
            SEEDS_4,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                let count = seed % 4;
                if seed & 1 == 0 {
                    w(c, this + 0xbc, 0);
                } else {
                    let array = alloc_2(c, 0x40);
                    w(c, this + 0xbc, array);
                    wh(c, this + 0xc8, count as u16);
                    for i in 0..count {
                        let entry = if seed & 2 == 0 { 0 } else { alloc_2(c, 0x40) };
                        w(c, array + 4 * i, entry);
                    }
                }
                c.g[A0][0] = this as u64;
            }),
        );
    }


    // ------------------------------------------------------------- text scroll

    #[test]
    fn c_ui_text_scroll_grow_lines_matches() {
        check(ssx3_ee::s13::f_003a1958__generated, c_ui_text_scroll_grow_lines, 12, &|c, seed| {
            let _ = base_3(c, seed, 0x003a_1958);
            heap(c);
            pool(c);
            let obj = DATA + 0x100;
            clear(c, obj, 0x100);
            let old = if seed % 3 == 0 { 0 } else { DATA + 0x40_0400 };
            w(c, obj + 0xbc, old);
            c.bus.write16(obj + 0xc8, (seed % 4) as u16);
            c.g[A0][0] = sx(obj);
        });
    }


    #[test]
    fn ui_vector_dtor_matches() {
        check(
            ee::s13::f_003a32f0__generated,
            ui_vector_dtor,
            10,
            &|c, seed| {
                let _ = base_4(c, seed, 0x3a32f0, 3);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(c, OBJ + 0x7c, if seed % 2 == 0 { 0 } else { DST });
                w(c, OBJ + 0x80, if seed % 3 == 0 { 0 } else { DST });
                w(c, OBJ + 0x84, 0);
                w(c, OBJ + 0x88, DST);
                w(c, OBJ + 0x8c, 0);
            },
        );
    }


    #[test]
    fn ui_text_get_text_size_matches() {
        strict(ee::s13::f_003a1030__generated, ui_text_get_text_size, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x3a10, 6);
            let this = DATA;
            c.bus.write32(this + 0xb4, if seed % 3 == 0 { 0 } else { 1 });
            c.bus.write32(this + 0x14, r.next_u32());
            c.bus.write32(this + 8, DATA + 0x1000);
            c.bus.write16(DATA + 0x1000 + 0x80, 0);
            c.bus.write32(DATA + 0x1000 + 0x84, FAKE);
            c.bus.write32(this + 0x50, r.edge_f32());
            c.bus.write32(this + 0x54, r.edge_f32());
            c.bus.write32(DATA + 0x2000 + 8, r.next_u32());
            c.bus.write32(DATA + 0x2000, r.next_u32());
            c.g[A0][0] = sx(this);
            c.g[A1][0] = sx(DATA + 0x200);
        });
    }


    // -- 0x003a13e0 cUITextScroll_HandleInput ---------------------------

    #[test]
    fn cuit_ext_scroll_handle_input_matches() {
        check(
            ssx3_ee::s13::f_003a13e0__generated,
            cuit_ext_scroll_handle_input,
            120,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x3a13e0, 3);
                let this = DATA + 0x1000;
                let input = DATA + 0x2000;
                let fman = DATA + 0x3000;
                let ftab = DATA + 0x3100;
                let fobj = DATA + 0x3200;
                let font = DATA + 0x3300;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(input);
                // font lookup
                w(c, this + 0x5c, fman);
                w(c, fman + 0xd0, ftab);
                w(c, ftab + 0x10, fobj);
                w(c, fobj + 8, font);
                w(c, this + 0x74, (seed % 2) as u32);
                w(c, font + 0xc, DATA + 0x3400);
                w(c, font + 0x18, DATA + 0x3400);
                // the box's own fields
                wf(c, this + 0x54, 2.0f32.to_bits());
                wf(c, this + 0x64, 100.0f32.to_bits());
                wf(c, font + 0x14, 16.0f32.to_bits());
                wf(c, font + 0x34, 1.0f32.to_bits());
                c.bus.write16(this + 0xc4, (seed % 7) as u16);
                c.bus.write16(this + 0xc6, (seed % 5) as u16);
                c.bus.write16(this + 0xc8, 9);
                c.bus.write8(this + 0x88, 1);
                c.bus.write8(this + 0x89, 2);
                // input object: vtable-ish words
                w(c, input + 8, DATA + 0x3500);
                w(c, DATA + 0x3500 + 0x10, 0x10);
                w(c, DATA + 0x3500 + 0x14, FAKE);
                w(c, DATA + 0x3500 + 0x20, 0);
                // one of the two key checks reports "pressed"
                w(c, LOG_RESULTS + 4 * ((seed % 2) as u32), 1);
                let _ = r;
            },
        );
    }
}
