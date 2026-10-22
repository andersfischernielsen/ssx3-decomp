//! `src/fe/ovstatemap.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::call::guest;
use crate::call::Callee;
use crate::call::Frame;
use crate::call::VSlot;
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
use crate::call::call_ptr;
use crate::call::sp;
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::call::Flow;
use crate::ps2f::FpuF32;
use crate::events::deliver_event;
pub(crate) use crate::game::fe::ovstatepause::lw;
pub(crate) use crate::game::dirtysock::tags2::at;
pub(crate) use crate::game::dirtysock::tags2::p_2 as p;
pub(crate) use crate::game::dirtysock::tags2::gp_5 as gp;
pub(crate) use crate::game::fe::fepopup3::p_2;
pub(crate) use crate::game::fe::fepopup3::lw_2;
pub(crate) use crate::game::fe::fepopup3::lh;
pub(crate) use crate::game::fe::feasyncfile::at_2;
pub(crate) use crate::game::fe::feasyncfile::low_2 as low;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::lh_r;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::gamemode::gpo;
pub(crate) use crate::game::main::gamemode::sw_r;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_3;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_3;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw;
pub(crate) use crate::game::ai::computer::lh_2;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::be::beintnetwork::ldl;
pub(crate) use crate::game::be::beintnetwork::sdl;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0020_8b00, ov_screen_session_on_activate, "OVScreenSession_OnActivate"),
    (0x0020_8b78, ov_screen_session_shutdown, "OVScreenSession_Shutdown"),
    (0x0020_8bb8, ov_screen_session_hide_map_pic, "OVScreenSession_HideMapPic"),
    (0x0020_8ef8, ov_screen_session_on_widget_notify, "OVScreenSession_OnWidgetNotify"),
    (0x0020_8f10, ov_screen_session_load_map_image, "OVScreenSession_LoadMapImage"),
    (0x0020_9300, ov_screen_session_post_input, "OVScreenSession_PostInput"),
    (0x0020_9370, ov_screen_session_update, "OVScreenSession_Update"),
    (0x0020_95e8, ov_screen_session_set_highlight_color, "OVScreenSession_SetHighlightColor"),
    (0x0020_9648, ov_screen_session_set_unlocked_color, "OVScreenSession_SetUnlockedColor"),
    (0x0020_9e78, ov_screen_session_update_indicators, "OVScreenSession_UpdateIndicators"),
    (0x0020_a6a8, ov_state_overlay_on_widget_action, "cOVStateOverlay_OnWidgetAction"),
    (0x0020_a6f0, c_ov_state_overlay_play_object_anim, "cOVStateOverlay_PlayObjectAnim"),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::fe::ovstatepause::callee::MPEG2_SHOW_BITS;
    pub(crate) use crate::game::fe::ovstatepause::callee::MPEG2_FLUSH_BUFFER;
    pub(crate) use crate::game::fe::ovstatepause::callee::AI_DESTROY_MISSION_RIDERS;
    pub(crate) use crate::game::fe::ovstatepause::callee::AI_CREATE_COMPUTER_RIDERS;
    pub(crate) use crate::game::fe::ovstatepause::callee::ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY;
    pub(crate) use crate::game::fe::ovstatepause::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::ovstatepause::callee::BE_REWARD_INT_COUNT_SET_BITS;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::fe::ovstatepause::callee::RIDER_METRIX_CLEAR_PEAK_A4;
    pub(crate) use crate::game::fe::ovstatepause::callee::EA_MEM_SET;
    pub(crate) use crate::game::fe::ovstatepause::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::fe::ovstatepause::callee::MC_MANAGER_BEGIN_LOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_CTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_FORMAT_MONEY;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_MODULE_RIDER_PREVIEW_UNLOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_MODULE_RIDER_PREVIEW_UPDATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHECK_ACCOUNT_NAME;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHECK_PASSWORD;
    pub(crate) use crate::game::fe::ovstatepause::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::ovstatepause::callee::NET_FE_STATE_CTOR_2 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::ovstatepause::callee::KEYBOARD_POPUP_IS_KEY_ENABLED;
    pub(crate) use crate::game::fe::ovstatepause::callee::KEYBOARD_POPUP_MOVE_DOWN;
    pub(crate) use crate::game::fe::ovstatepause::callee::EVENT_SELECT_POST_INPUT;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_POST_INPUT;
    pub(crate) use crate::game::fe::ovstatepause::callee::STR_HASH;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXTURE_SET_FIND;
    pub(crate) use crate::game::fe::ovstatepause::callee::VID_ENGINE_PLAYER_RENDER;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_RENDER;
    pub(crate) use crate::game::fe::ovstatepause::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::ovstatepause::callee::MUSIC_PLAYER_RESUME;
    pub(crate) use crate::game::fe::ovstatepause::callee::MOUNTAIN_ROOM_POLL_IMAGE_LOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_UPDATE_2 as UI_STATE_UPDATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_ON_ACTIVATE_2 as UI_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::PLAY_FE_SOUND;
    pub(crate) use crate::game::fe::ovstatepause::callee::MEM_CARD_STATE_BASE_CTOR_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_MGR_CLOSE_TOP_STATE_AND_POPUPS;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_MGR_CLOSE_TOP_STATES;
    pub(crate) use crate::game::fe::ovstatepause::callee::OV_SCREEN_SESSION_DESTROY_INDICATORS;
    pub(crate) use crate::game::fe::ovstatepause::callee::C_OV_STATE_OVERLAY_SHUTDOWN;
    pub(crate) use crate::game::fe::ovstatepause::callee::FE_ASYNC_MANAGER_CANCEL;
    pub(crate) use crate::game::fe::ovstatepause::callee::SECTION_MAN_REQUEST_ACTIVATE;
    pub(crate) use crate::game::fe::ovstatepause::callee::STRCPY;
    pub(crate) use crate::game::fe::ovstatepause::callee::STRLEN;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_STATS;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_CLEAR_HUD_MESSAGES;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_TRICK;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_RESET_COMBO;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_METRIX_BANK_PENDING_SCORE;
    pub(crate) use crate::game::main::gamemode::callee::RIDER_GET_SYNC_CHECKSUM;
    pub(crate) use crate::game::main::gamemode::callee::CBE_GET_BE;
    pub(crate) use crate::game::main::gamemode::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::main::gamemode::callee::BE_NEW_PLAYER_INT_GET_SLOT as BE_NEW_PLAYER_INT_GET_SLOT_2;
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
    pub(crate) use crate::game::main::gamemode::callee::BUILTIN_DELETE as BUILTIN_DELETE_2;
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
    pub(crate) use crate::game::main::gamemode::callee::STR_HASH as STR_HASH_2;
    pub(crate) use crate::game::main::gamemode::callee::SPRINTF;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET;
    pub(crate) use crate::game::main::gamemode::callee::MEMSET_3E6448;
    pub(crate) use crate::game::main::gamemode::callee::MEMCPY;
    pub(crate) use crate::game::main::gamemode::callee::STRLEN as STRLEN_2;
    pub(crate) use crate::game::main::gamemode::callee::STRCPY as STRCPY_2;
    pub(crate) use crate::game::main::gamemode::callee::CVT_FLOAT_TO_INT;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_GET_OBJECT_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::main::gamemode::callee::C_UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::main::gamemode::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE as SSX_AUDIO_SYSTEM_GET_INSTANCE_2;
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
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_HASH as STR_HASH_3;
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
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STRLEN as STRLEN_3;
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


/// `void OVScreenSession_Shutdown(Session* s)` (0x208b78): cancel the
/// session's async file, destroy the indicators and shut the overlay down.
pub fn ov_screen_session_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        let g = crate::abi::r32(c, GP);
        let world = lw(c, g, 0xfffff7b8); // gp - 0x848
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = crate::abi::r32(c, S0);
        c.g[A1][0] = lw(c, this, 0xc4);
        c.g[A0][0] = lw(c, world as u32, 0x11c);
        callee::FE_ASYNC_MANAGER_CANCEL.call(c, 0x0020_8b98)?;
        c.g[A0][0] = c.g[S0][0];
        callee::OV_SCREEN_SESSION_DESTROY_INDICATORS.call(c, 0x0020_8ba0)?;
        c.g[A0][0] = c.g[S0][0];
        callee::C_OV_STATE_OVERLAY_SHUTDOWN.call(c, 0x0020_8ba8)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cOVStateOverlay_OnWidgetAction(self, widget, action)` (0x20a6a8):
/// pass the action to the overlay screen's widget handler, when one exists.
pub fn ov_state_overlay_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw(gp(c, -0x848));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(at(c, V0, 0x84));
        let no_race = c.g[A0][0] == 0;
        c.g[RA][0] = c.ld(p(sp(c), 0));
        if no_race {
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[A3][0] = c.lw(at(c, A0, 0x200));
        let no_screen = c.g[A3][0] == 0;
        if no_screen {
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 6;
        c.g[V0][0] = c.lw(at(c, A3, 0x0c));
        c.g[A0][0] = c.lh(p(r32(c, V0), 0x48));
        c.g[V1][0] = c.lw(p(r32(c, V0), 0x4c));
        let target = r32(c, V1);
        c.g[RA][0] = sx(0x0020_a6e0);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0020_a6e0)?;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}


/// `void OVScreenSession_SetHighlightColor(Session* s, void* obj, int on)`
/// (0x2095e8): forward the highlight colour (`D_004C8B38`) or the default
/// colour (`D_004C8B28`) to the object's vtable slot 0x58.
pub fn ov_screen_session_set_highlight_color(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0];
        let color = c.g[A2][0];
        frame.save_ra(c, 0);
        if color != 0 {
            c.cycles += 9;
            c.g[V1][0] = lw_2(c, p_2(c.g[A3][0], 8));
            c.g[A1][0] = 0x004d_0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x74c8));
            c.g[A0][0] = lh(c, p_2(c.g[V1][0], 0x58));
            c.g[V0][0] = lw_2(c, p_2(c.g[V1][0], 0x5c));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0020_9614)?;
            c.g[RA][0] = Addr(p_2(c.g[SP][0], 0)).ld(c);
        } else {
            c.cycles += 8;
            c.g[V1][0] = lw_2(c, p_2(c.g[A3][0], 8));
            c.g[A1][0] = 0x004d_0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x74d8));
            c.g[A0][0] = lh(c, p_2(c.g[V1][0], 0x58));
            c.g[V0][0] = lw_2(c, p_2(c.g[V1][0], 0x5c));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0020_9638)?;
            c.g[RA][0] = Addr(p_2(c.g[SP][0], 0)).ld(c);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}


/// `void OVScreenSession_SetUnlockedColor(Session* s, void* obj, int on)`
/// (0x209648): forward the unlocked colour (`D_004C8B58`) or the locked
/// colour (`D_004C8B48`) to the object's vtable slot 0x58.
pub fn ov_screen_session_set_unlocked_color(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A1][0];
        let color = c.g[A2][0];
        frame.save_ra(c, 0);
        if color != 0 {
            c.cycles += 9;
            c.g[V1][0] = lw_2(c, p_2(c.g[A3][0], 8));
            c.g[A1][0] = 0x004d_0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x74a8));
            c.g[A0][0] = lh(c, p_2(c.g[V1][0], 0x58));
            c.g[V0][0] = lw_2(c, p_2(c.g[V1][0], 0x5c));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0020_9674)?;
            c.g[RA][0] = Addr(p_2(c.g[SP][0], 0)).ld(c);
        } else {
            c.cycles += 8;
            c.g[V1][0] = lw_2(c, p_2(c.g[A3][0], 8));
            c.g[A1][0] = 0x004d_0000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x74b8));
            c.g[A0][0] = lh(c, p_2(c.g[V1][0], 0x58));
            c.g[V0][0] = lw_2(c, p_2(c.g[V1][0], 0x5c));
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0020_9698)?;
            c.g[RA][0] = Addr(p_2(c.g[SP][0], 0)).ld(c);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}


/// `int OVScreenSession_OnWidgetNotify(OVScreen*, void*, int key)`
/// (0x00208ef8): 0 for the keys 8 and 9, 0x101 for the rest.
pub fn ov_screen_session_on_widget_notify(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x8));
    c.g[V0][0] = 0x101;
    c.g[A2][0] = (c.g[A2][0] < 0x2) as u64;
    if c.g[A2][0] != 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}


/// `void OVScreenSession_HideMapPic(OVScreenSession* self)` (0x208bb8):
/// drop both map pictures and hide the map.
pub fn ov_screen_session_hide_map_pic(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        Addr(at_2(c, S0, 0xdc)).sw(c, c.g[V0][0]);
        c.g[A2][0] = Addr(at_2(c, S0, 0xd0)).lw(c);
        let none = c.g[A2][0] == 0;
        Addr(at_2(c, S0, 0xc8)).sw(c, 0);
        if !none {
            c.cycles += 6;
            c.g[V0][0] = Addr(at_2(c, A2, 8)).lw(c);
            c.g[A1][0] = 0;
            c.g[A0][0] = Addr(at_2(c, V0, 0x48)).lh(c);
            c.g[V1][0] = Addr(at_2(c, V0, 0x4c)).lw(c);
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx(at_2(c, A2, 0).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0020_8bf4)?;
        }
        c.cycles += 3;
        c.g[A2][0] = Addr(at_2(c, S0, 0xe0)).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(at_2(c, A2, 8)).lw(c);
            c.g[A0][0] = Addr(at_2(c, V0, 0x48)).lh(c);
            c.g[V1][0] = Addr(at_2(c, V0, 0x4c)).lw(c);
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx(at_2(c, A2, 0).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0020_8c14)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void OVScreenSession_PostInput(OVScreenSession* self, void* widget)`
/// (0x209300): when either of the widget's two status methods reports a
/// change, refresh the session's indicators.
pub fn ov_screen_session_post_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        c.g[V1][0] = Addr(at_2(c, S0, 8)).lw(c);
        c.g[A0][0] = Addr(at_2(c, V1, 0x88)).lh(c);
        c.g[V0][0] = Addr(at_2(c, V1, 0x8c)).lw(c);
        let t = c.g[V0][0] as u32;
        c.g[A0][0] = sx(at_2(c, S0, 0).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x0020_932c)?;
        let first = c.g[V0][0] != 0;
        let mut update = first;
        if !first {
            c.cycles += 7;
            c.g[V1][0] = Addr(at_2(c, S0, 8)).lw(c);
            c.g[A0][0] = Addr(at_2(c, V1, 0x90)).lh(c);
            c.g[V0][0] = Addr(at_2(c, V1, 0x94)).lw(c);
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx(at_2(c, S0, 0).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x0020_9348)?;
            update = c.g[V0][0] != 0;
            frame.restore(c, S0, 0x20);
        }
        if update {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0020_9e78, 0x0020_9358)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `func_00208B00` (0x208b00): `OVScreenSession::OnActivate`. Activates the
/// screen (`0x39e4c0`), looks up the widget named `0x470a20`
/// (`0x39d860(this->+0xd4, hash)`) and, if found, calls its `vtable[0x38]`
/// (with 0) and `vtable[0x48]` (with 0). Signature: `void on_activate(this)`.
pub fn ov_screen_session_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0020_8b14);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e4c0, 0x0020_8b14)?;
        c.g[A0][0] = 0x47_0000;
        c.g[RA][0] = sx(0x0020_8b20);
        c.g[A0][0] = sx(low(c, A0).wrapping_add(0xa20));
        call_ptr(c, 0x0031_7670, 0x0020_8b20)?;
        c.g[A0][0] = lw_r(c, S0, 0xd4);
        c.g[RA][0] = sx(0x0020_8b2c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0020_8b2c)?;
        c.g[S0][0] = c.g[V0][0];
        let missing = c.g[S0][0] == 0;
        c.g[A1][0] = 0;
        if !missing {
            c.cycles += 11;
            c.g[V0][0] = lw_r(c, S0, 0x8);
            c.g[A0][0] = lh_r(c, V0, 0x38);
            c.g[V1][0] = lw_r(c, V0, 0x3c);
            let target = low(c, V1);
            c.g[RA][0] = sx(0x0020_8b4c);
            c.g[A0][0] = sx(low(c, S0).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x0020_8b4c)?;
            c.g[V0][0] = lw_r(c, S0, 0x8);
            c.g[A1][0] = 0;
            c.g[A0][0] = lh_r(c, V0, 0x48);
            c.g[V1][0] = lw_r(c, V0, 0x4c);
            let target = low(c, V1);
            c.g[RA][0] = sx(0x0020_8b64);
            c.g[A0][0] = sx(low(c, S0).wrapping_add(low(c, A0)));
            call_ptr(c, target, 0x0020_8b64)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void OVScreenSession_LoadMapImage(self)` (0x208f10): unload the previous
/// map image through the async manager, then start loading this screen's.
pub fn ov_screen_session_load_map_image(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x40));
        c.sq(17, sp_2(c).wrapping_add(0x20));
        c.sq(16, sp_2(c).wrapping_add(0x30));
        c.sq(18, sp_2(c).wrapping_add(0x10));
        c.sd(sp_2(c), c.g[RA][0]);
        c.g[S1][0] = c.g[A0][0];
        callee::CBE_GET_BE.call(c, 0x0020_8f2c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0020_8f38)?;
        c.g[V1][0] = Addr(gpo(c, -0x848)).lw(c);
        c.g[S2][0] = c.g[V0][0];
        c.g[A1][0] = lw_r(c, S1, 0xc4);
        c.g[S0][0] = lw_r(c, V1, 0x11c);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001a_38c0, 0x0020_8f50)?;
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0014_4bc0, 0x0020_8f58)?;
        c.g[A2][0] = lw_r(c, S1, 0xc4);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = Addr(low(c, V0)).lw(c);
        c.g[A3][0] = 0x1;
        call_ptr(c, 0x001a_37f8, 0x0020_8f6c)?;
        sw_r(c, S1, 0xc8, 0);
        c.lq(16, sp_2(c).wrapping_add(0x30));
        c.lq(17, sp_2(c).wrapping_add(0x20));
        c.lq(18, sp_2(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}


/// `void cOVStateOverlay_PlayObjectAnim(cOVStateOverlay* self, char* widget,
/// char* anim)` (0x20a6f0): find the widget and play the named animation on
/// it (the overlay counterpart of `MemCardStateBase_PlayWidgetAnim`).
pub fn c_ov_state_overlay_play_object_anim(c: &mut Cpu) {
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
        call_ptr(c, callee::STR_HASH_3, 0x0020_a714)?;
        let this = c.g[S0][0];
        c.g[A0][0] = lw_at(c, this, 0x40);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, callee::UI_SCREEN_FIND_OBJECT, 0x0020_a720)?;
        c.g[S1][0] = c.g[V0][0];
        let found = c.g[S1][0] != 0;
        c.g[A0][0] = c.g[S2][0];
        if found {
            c.cycles += 8;
            c.g[S0][0] = lw_at(c, this, 0x10);
            c.g[S0][0] = sx(at_3(c.g[S0][0] as u32, 0x50));
            call_ptr(c, callee::STR_HASH_3, 0x0020_a738)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, callee::UI_ANIM_SET_CREATE_ANIM, 0x0020_a744)?;
            let anim = c.g[V0][0];
            let none = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S1][0];
            if !none {
                c.cycles += 5;
                c.g[A1][0] = anim;
                c.g[A2][0] = 3;
                c.g[A3][0] = 0;
                c.g[T0][0] = 0;
                call_ptr(c, callee::UI_OBJECT_SET_ANIM, 0x0020_a760)?;
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

/// The virtual method call the indicator destructor makes: `obj`'s vtable at
/// `+8`, slot `{delta at +delta_off, fn at +fn_off}`, called with
/// `this = obj + delta` and the caller's `a1` untouched.  Reproduces the
/// generated body's register dance (the target stays in `v1`, the loaded
/// delta in `a0`) and returns the unwind flow.
pub(crate) fn call_indicator_method(c: &mut Cpu, obj: u64, delta_off: u32, fn_off: u32, ret: u32) -> Flow {
    c.g[V0][0] = c.lw((obj as u32).wrapping_add(0x8));
    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(delta_off));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(fn_off));
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((obj as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}


/// `OVScreenSession_DestroyIndicators` (0x0020a088): destroy an overlay
/// screen session's indicator objects.  The object at `self+0xe0` gets its
/// `+0x4c` method (arg 0) and then its `+0xc` method (arg 3), and the eight
/// `self+0xe4..`/`self+0x104..` slots are cleared the same way, each pair
/// through the slot object's vtable at `+8`.
pub fn ov_screen_session_destroy_indicators(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe0));
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 0u64;
        if !none {
            c.cycles += 8;
            call_indicator_method(c, c.g[A2][0], 0x48, 0x4c, 0x0020_a0c0)?;
            c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xe0));
            let none = c.g[A2][0] == 0;
            c.g[A1][0] = 0x3;
            if !none {
                c.cycles += 5;
                call_indicator_method(c, c.g[A2][0], 0x8, 0xc, 0x0020_a0e0)?;
            }
        }
        c.cycles += 1;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xe0), 0u64);
        c.cycles += 3;
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xe4));
        c.g[S2][0] = 0u64;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x104));
        loop {
            c.cycles += 3;
            c.g[A2][0] = c.lw(c.g[S1][0] as u32);
            let none = c.g[A2][0] == 0;
            c.g[A1][0] = 0u64;
            if !none {
                c.cycles += 8;
                call_indicator_method(c, c.g[A2][0], 0x48, 0x4c, 0x0020_a110)?;
                c.g[A2][0] = c.lw(c.g[S1][0] as u32);
                let none = c.g[A2][0] == 0;
                c.g[A1][0] = 0x3;
                if !none {
                    c.cycles += 5;
                    call_indicator_method(c, c.g[A2][0], 0x8, 0xc, 0x0020_a130)?;
                }
            }
            c.cycles += 1;
            c.sw(c.g[S1][0] as u32, 0u64);
            c.cycles += 3;
            c.g[A2][0] = c.lw(c.g[S0][0] as u32);
            let none = c.g[A2][0] == 0;
            c.g[A1][0] = 0u64;
            if !none {
                c.cycles += 8;
                call_indicator_method(c, c.g[A2][0], 0x48, 0x4c, 0x0020_a154)?;
                c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                let none = c.g[A2][0] == 0;
                c.g[A1][0] = 0x3;
                if !none {
                    c.cycles += 5;
                    call_indicator_method(c, c.g[A2][0], 0x8, 0xc, 0x0020_a174)?;
                }
            }
            c.cycles += 1;
            c.sw(c.g[S0][0] as u32, 0u64);
            c.cycles += 5;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = ((c.g[S2][0] as i64) < 8i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0020_a0f0);
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void func_00209370(sMap_9370* self)` (0x209370): per-frame update of the
/// session/loading overlay: run the overlay's base update, then either fade
/// the image in (`fade += 0.1` up to 1) and push it to the colour setter, or
/// clear it; poll the async file status and, once the file is ready, hand the
/// loaded data to the image; then bob the eight map icons (sinusoidal scale
/// and offset) around the icon positions.
pub fn ov_screen_session_update(c: &mut Cpu) {
    let frame = Frame::open(c, 0x80);
    'b_2095c8: {
        c.cycles += 13;
        frame.save(c, S0, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save_ra(c, 0x20);
        c.swc1(21, at(c, SP, 0x78));
        c.g[RA][0] = sx(0x00209398);
        c.swc1(20, at(c, SP, 0x70));
        if callee::C0020e900.call(c, 0x00209398).is_err() {
            return;
        }
        c.g[V0][0] = lw_3(c, at(c, S0, 0xdc));
        if c.g[V0][0] != 0u64 {
            frame.restore(c, S0, 0x60);
            break 'b_2095c8;
        } else {
            'b_209460: {
                'b_209440: {
                    c.cycles += 3;
                    c.g[V0][0] = lw_3(c, at(c, S0, 0xc8));
                    if c.g[V0][0] == 0u64 {
                        sw(c, at(c, S0, 0xcc), 0u64);
                        break 'b_209440;
                    } else {
                        'b_2093dc: {
                            c.cycles += 7;
                            c.lwc1(1, at(c, S0, 0xcc));
                            c.g[AT][0] = 0x3f800000;
                            c.mtc1(1, 0);
                            c.c_lt_s(1, 0);
                            if !c.fcc() {
                                c.g[A0][0] = lw_3(c, at(c, S0, 0xd0));
                                break 'b_2093dc;
                            } else {
                                c.cycles += 4;
                                c.lwc1(0, r32(c, GP).wrapping_sub(0x5504));
                                c.add_s(0, 1, 0);
                                c.swc1(0, at(c, S0, 0xcc));
                                c.g[A0][0] = lw_3(c, at(c, S0, 0xd0));
                                break 'b_2093dc;
                            }
                        }
                        c.cycles += 25;
                        c.g[A1][0] = c.g[SP][0];
                        c.lwc1(0, at(c, S0, 0xcc));
                        ldl(c, 2, at(c, A0, 0x1c));
                        ldl(c, 3, at(c, A0, 0x24));
                        sdl(c, 2, r32(c, SP));
                        sdl(c, 3, at(c, SP, 0x8));
                        c.swc1(0, r32(c, SP));
                        c.g[V0][0] = lw_3(c, at(c, A0, 0x8));
                        c.g[V1][0] = lh_2(c, at(c, V0, 0x58));
                        c.g[A2][0] = lw_3(c, at(c, V0, 0x5c));
                        let t = c.g[A2][0] as u32;
                        c.g[RA][0] = sx(0x0020941c);
                        c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
                        if call_ptr(c, t, 0x0020941c).is_err() {
                            return;
                        }
                        c.g[A3][0] = lw_3(c, at(c, S0, 0xd0));
                        c.g[A1][0] = 0x1;
                        c.g[V0][0] = lw_3(c, at(c, A3, 0x8));
                        c.g[A0][0] = lh_2(c, at(c, V0, 0x48));
                        c.g[V1][0] = lw_3(c, at(c, V0, 0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x00209438);
                        c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
                        if call_ptr(c, t, 0x00209438).is_err() {
                            return;
                        }
                        let t = true;
                        c.g[V0][0] = lw_3(c, r32(c, GP).wrapping_sub(0x848));
                        if t {
                            break 'b_209460;
                        } else {
                            break 'b_209440;
                        }
                    }
                }
                c.cycles += 8;
                c.g[A1][0] = 0u64;
                c.g[A2][0] = lw_3(c, at(c, S0, 0xd0));
                c.g[V0][0] = lw_3(c, at(c, A2, 0x8));
                c.g[A0][0] = lh_2(c, at(c, V0, 0x48));
                c.g[V1][0] = lw_3(c, at(c, V0, 0x4c));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x0020945c);
                c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                if call_ptr(c, t, 0x0020945c).is_err() {
                    return;
                }
                c.g[V0][0] = lw_3(c, r32(c, GP).wrapping_sub(0x848));
                break 'b_209460;
            }
            'b_2094bc: {
                c.cycles += 7;
                c.g[A1][0] = lw_3(c, at(c, S0, 0xc4));
                c.g[S1][0] = lw_3(c, at(c, V0, 0x11c));
                c.g[RA][0] = sx(0x00209470);
                c.g[A0][0] = c.g[S1][0];
                if call_ptr(c, 0x001a39c0, 0x00209470).is_err() {
                    return;
                }
                c.g[V1][0] = 0x3;
                if c.g[V0][0] != c.g[V1][0] {
                    c.lwc1(1, at(c, S0, 0x1d0));
                    break 'b_2094bc;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = lw_3(c, at(c, S0, 0xc8));
                    if c.g[V0][0] != 0u64 {
                        c.lwc1(1, at(c, S0, 0x1d0));
                        break 'b_2094bc;
                    } else {
                        'b_2094b8: {
                            c.cycles += 3;
                            c.g[A2][0] = lw_3(c, at(c, S0, 0xd0));
                            let t = c.g[A2][0] == 0u64;
                            c.g[V1][0] = 0x11c;
                            if t {
                                break 'b_2094b8;
                            } else {
                                c.cycles += 9;
                                c.g[V0][0] = lw_3(c, at(c, S0, 0xc4));
                                c.g[A1][0] = 0x1;
                                c.mult(0, A0, V0, V1);
                                c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
                                c.g[A0][0] = lw_3(c, at(c, V0, 0x110));
                                sw(c, at(c, A2, 0x7c), 0u64);
                                sw(c, at(c, A2, 0x78), c.g[A0][0]);
                                sw(c, at(c, S0, 0xc8), c.g[A1][0]);
                                sw(c, at(c, S0, 0xcc), 0u64);
                                break 'b_2094b8;
                            }
                        }
                        c.cycles += 1;
                        c.lwc1(1, at(c, S0, 0x1d0));
                        break 'b_2094bc;
                    }
                }
            }
            'b_209518: {
                'b_209514: {
                    'b_2094f4: {
                        c.cycles += 5;
                        c.mtc1(0, 0);
                        c.c_lt_s(0, 1);
                        let t = !c.fcc();
                        c.lwc1(1, at(c, S0, 0x1cc));
                        if t {
                            break 'b_2094f4;
                        } else {
                            c.cycles += 6;
                            c.g[AT][0] = 0x41ac0000;
                            c.mtc1(1, 0);
                            c.c_lt_s(0, 1);
                            if !c.fcc() {
                                c.lwc1(0, at(c, S0, 0x1cc));
                                break 'b_209518;
                            } else {
                                c.cycles += 3;
                                c.lwc1(0, r32(c, GP).wrapping_sub(0x5500));
                                let t = true;
                                c.swc1(0, at(c, S0, 0x1d0));
                                if t {
                                    break 'b_209514;
                                } else {
                                    break 'b_2094f4;
                                }
                            }
                        }
                    }
                    c.cycles += 6;
                    c.g[AT][0] = 0x41900000;
                    c.mtc1(1, 0);
                    c.c_lt_s(1, 0);
                    if !c.fcc() {
                        c.lwc1(0, at(c, S0, 0x1cc));
                        break 'b_209518;
                    } else {
                        c.cycles += 2;
                        c.lwc1(0, r32(c, GP).wrapping_sub(0x54fc));
                        c.swc1(0, at(c, S0, 0x1d0));
                        break 'b_209514;
                    }
                }
                c.cycles += 1;
                c.lwc1(0, at(c, S0, 0x1cc));
                break 'b_209518;
            }
            c.cycles += 10;
            c.g[S2][0] = sx(at(c, S0, 0x124));
            c.lwc1(1, at(c, S0, 0x1d0));
            c.g[S1][0] = sx(at(c, S0, 0x104));
            c.g[AT][0] = 0x41900000;
            c.mtc1(1, 21);
            c.g[S3][0] = 0x7;
            c.add_s(0, 0, 1);
            c.g[AT][0] = 0x3f000000;
            c.mtc1(1, 20);
            c.swc1(0, at(c, S0, 0x1cc));
            'l_209540: loop {
                'b_2095b8: {
                    c.cycles += 3;
                    c.g[V0][0] = lw_3(c, r32(c, S1));
                    if c.g[V0][0] == 0u64 {
                        c.g[S2][0] = sx(at(c, S2, 0xc));
                        break 'b_2095b8;
                    } else {
                        c.cycles += 27;
                        c.lwc1(0, at(c, S0, 0x1cc));
                        c.lwc1(2, r32(c, S2));
                        c.sub_s(0, 0, 21);
                        c.lwc1(1, at(c, S2, 0x4));
                        c.mul_s(0, 0, 20);
                        c.sub_s(1, 1, 0);
                        c.sub_s(2, 2, 0);
                        c.swc1(1, at(c, SP, 0x4));
                        c.swc1(2, r32(c, SP));
                        ldl(c, 6, r32(c, SP));
                        c.g[A3][0] = lw_3(c, at(c, SP, 0x8));
                        sdl(c, 6, at(c, V0, 0x44));
                        sw(c, at(c, V0, 0x4c), c.g[A3][0]);
                        c.g[V0][0] = lw_3(c, r32(c, S1));
                        c.lwc1(0, at(c, S0, 0x1cc));
                        c.g[V1][0] = lw_3(c, at(c, V0, 0x8));
                        c.g[A0][0] = lh_2(c, at(c, V1, 0xa8));
                        c.swc1(0, at(c, SP, 0x14));
                        c.swc1(0, at(c, SP, 0x10));
                        c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                        sw(c, at(c, SP, 0x18), 0u64);
                        c.g[V0][0] = lw_3(c, at(c, V1, 0xac));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x002095b4);
                        c.g[A1][0] = sx(at(c, SP, 0x10));
                        if call_ptr(c, t, 0x002095b4).is_err() {
                            return;
                        }
                        c.g[S2][0] = sx(at(c, S2, 0xc));
                        break 'b_2095b8;
                    }
                }
                c.cycles += 3;
                c.g[S3][0] = sx(r32(c, S3).wrapping_sub(0x1));
                let t = (c.g[S3][0] as i64) >= 0;
                c.g[S1][0] = sx(at(c, S1, 0x4));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00209540);
                    }
                    continue 'l_209540;
                } else {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x60);
                    break 'b_2095c8;
                }
            }
        }
    }
    c.cycles += 8;
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    c.g[RA][0] = ld(c, at(c, SP, 0x20));
    c.lwc1(21, at(c, SP, 0x78));
    c.lwc1(20, at(c, SP, 0x70));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x80));
    c.pc = t;
    return;
}


/// `void func_00209E78(void* self)` (0x209e78): update the session map's
/// indicator icons: show the cursor, read the list box's selection out of the
/// screen's object table and show the "selected" icon (and hide the other)
/// for the selected entry, the opposite for the rest.
pub fn ov_screen_session_update_indicators(c: &mut Cpu) {
    let frame = Frame::open(c, 0x70);
    'b_209ebc: {
        c.cycles += 12;
        frame.save(c, S2, 0x40);
        frame.save(c, S0, 0x60);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        sd(c, r32(c, SP), c.g[RA][0]);
        c.g[A2][0] = lw_3(c, at(c, S2, 0xe0));
        let t = c.g[A2][0] == 0u64;
        c.g[A1][0] = 0x1;
        if t {
            break 'b_209ebc;
        } else {
            c.cycles += 5;
            c.g[V0][0] = lw_3(c, at(c, A2, 0x8));
            c.g[A0][0] = lh_2(c, at(c, V0, 0x48));
            c.g[V1][0] = lw_3(c, at(c, V0, 0x4c));
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x00209ebc);
            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
            if call_ptr(c, t, 0x00209ebc).is_err() {
                return;
            }
            break 'b_209ebc;
        }
    }
    'b_209ee4: {
        c.cycles += 9;
        c.g[A0][0] = 0x470000;
        c.g[RA][0] = sx(0x00209ec8);
        c.g[A0][0] = sx(at(c, A0, 0x1850));
        if call_ptr(c, 0x00317670, 0x00209ec8).is_err() {
            return;
        }
        c.g[A0][0] = lw_3(c, at(c, S2, 0xd4));
        c.g[RA][0] = sx(0x00209ed4);
        c.g[A1][0] = c.g[V0][0];
        if call_ptr(c, 0x0039d860, 0x00209ed4).is_err() {
            return;
        }
        c.g[V0][0] = lw_3(c, at(c, V0, 0xa0));
        let t = c.g[V0][0] == 0u64;
        c.g[V1][0] = 0x1;
        if t {
            break 'b_209ee4;
        } else {
            c.cycles += 1;
            c.g[V1][0] = lw_3(c, at(c, V0, 0x18));
            break 'b_209ee4;
        }
    }
    'b_209fa8: {
        c.cycles += 3;
        c.g[V0][0] = lw_3(c, at(c, S2, 0xc0));
        let t = (c.g[V0][0] as i64) <= 0;
        c.g[S1][0] = 0u64;
        if t {
            break 'b_209fa8;
        } else {
            c.cycles += 4;
            c.g[S5][0] = sx(r32(c, V1).wrapping_sub(0x1));
            c.g[S4][0] = sx(at(c, S2, 0x104));
            c.g[S0][0] = sx(at(c, S2, 0xe4));
            c.g[S3][0] = 0u64;
            'l_209f00: loop {
                'b_209f94: {
                    'b_209f90: {
                        'b_209f50: {
                            c.cycles += 2;
                            let t = c.g[S1][0] != c.g[S5][0];
                            c.g[A2][0] = lw_3(c, r32(c, S0));
                            if t {
                                break 'b_209f50;
                            } else {
                                'b_209f24: {
                                    c.cycles += 2;
                                    let t = c.g[A2][0] == 0u64;
                                    c.g[A1][0] = 0u64;
                                    if t {
                                        break 'b_209f24;
                                    } else {
                                        c.cycles += 5;
                                        c.g[V0][0] = lw_3(c, at(c, A2, 0x8));
                                        c.g[A0][0] = lh_2(c, at(c, V0, 0x48));
                                        c.g[V1][0] = lw_3(c, at(c, V0, 0x4c));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x00209f24);
                                        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                                        if call_ptr(c, t, 0x00209f24).is_err() {
                                            return;
                                        }
                                        break 'b_209f24;
                                    }
                                }
                                c.cycles += 4;
                                c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, S3)));
                                c.g[A2][0] = lw_3(c, r32(c, V0));
                                let t = c.g[A2][0] == 0u64;
                                c.g[A1][0] = 0x1;
                                if t {
                                    break 'b_209f90;
                                } else {
                                    c.cycles += 7;
                                    c.g[V0][0] = lw_3(c, at(c, A2, 0x8));
                                    c.g[A0][0] = lh_2(c, at(c, V0, 0x48));
                                    c.g[V1][0] = lw_3(c, at(c, V0, 0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x00209f48);
                                    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                                    if call_ptr(c, t, 0x00209f48).is_err() {
                                        return;
                                    }
                                    let t = true;
                                    c.g[V0][0] = lw_3(c, at(c, S2, 0xc0));
                                    if t {
                                        break 'b_209f94;
                                    } else {
                                        break 'b_209f50;
                                    }
                                }
                            }
                        }
                        'b_209f6c: {
                            c.cycles += 2;
                            let t = c.g[A2][0] == 0u64;
                            c.g[A1][0] = 0x1;
                            if t {
                                break 'b_209f6c;
                            } else {
                                c.cycles += 5;
                                c.g[V0][0] = lw_3(c, at(c, A2, 0x8));
                                c.g[A0][0] = lh_2(c, at(c, V0, 0x48));
                                c.g[V1][0] = lw_3(c, at(c, V0, 0x4c));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x00209f6c);
                                c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                                if call_ptr(c, t, 0x00209f6c).is_err() {
                                    return;
                                }
                                break 'b_209f6c;
                            }
                        }
                        c.cycles += 4;
                        c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, S3)));
                        c.g[A2][0] = lw_3(c, r32(c, V0));
                        let t = c.g[A2][0] == 0u64;
                        c.g[A1][0] = 0u64;
                        if t {
                            break 'b_209f90;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = lw_3(c, at(c, A2, 0x8));
                            c.g[A0][0] = lh_2(c, at(c, V0, 0x48));
                            c.g[V1][0] = lw_3(c, at(c, V0, 0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x00209f90);
                            c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                            if call_ptr(c, t, 0x00209f90).is_err() {
                                return;
                            }
                            break 'b_209f90;
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = lw_3(c, at(c, S2, 0xc0));
                    break 'b_209f94;
                }
                c.cycles += 5;
                c.g[S1][0] = sx(at(c, S1, 0x1));
                c.g[S0][0] = sx(at(c, S0, 0x4));
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let t = c.g[V0][0] != 0u64;
                c.g[S3][0] = sx(at(c, S3, 0x4));
                if t {
                    if event_due(c) {
                        deliver_event_here(c, 0x00209f00);
                    }
                    continue 'l_209f00;
                } else {
                    break 'b_209fa8;
                }
            }
        }
    }
    c.cycles += 9;
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    frame.restore(c, S4, 0x20);
    frame.restore(c, S5, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x70));
    c.pc = t;
    return;
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::check;
    use crate::difftest::put_words;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::check_lohi;
    use crate::difftest::fill;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::SP as STACK_TOP;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::RA as RET;
    use crate::difftest::compare;
    pub(crate) use crate::game::fe::ovstatepause::tests::BE_LIBRARY;
    pub(crate) use crate::game::fe::ovstatepause::tests::WORLD;
    pub(crate) use crate::game::fe::ovstatepause::tests::globals_2 as globals;
    pub(crate) use crate::game::dirtysock::tags2::tests::be_library_2 as be_library;
    pub(crate) use crate::game::fe::festatestore::tests::world;
    pub(crate) use crate::game::fe::feasyncfile::tests::fake_method;
    pub(crate) use crate::game::fe::fepopup3::tests::setup;
    pub(crate) use crate::game::world::worldview::tests::SEEDS_5 as SEEDS;
    pub(crate) use crate::game::camera::cameracontroller::tests::put_vtable;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::h;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3 as world_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_3 as slot;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_3;
    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_6 as run_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::put_str_3 as put_str;
    pub(crate) use crate::game::fe::feasyncfile::tests::screen_with;
    pub(crate) use crate::game::fe::fepopup::tests::BEL;
    pub(crate) use crate::game::fe::fepopup::tests::WORLD_4 as WORLD_2;
    pub(crate) use crate::game::fe::fepopup::tests::AUX;
    pub(crate) use crate::game::fe::fepopup::tests::SYS;
    pub(crate) use crate::game::fe::fepopup::tests::RACE;
    pub(crate) use crate::game::fe::fepopup::tests::RMAN_2 as RMAN;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base;


    #[test]
    fn ov_screen_session_shutdown_matches() {
        check(
            ssx3_ee::s05::f_00208b78__generated,
            ov_screen_session_shutdown,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x31, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 0xc4, (seed % 8) as u32);
                c.bus.write32(WORLD + 0x11c, DATA + 0x3000);
            },
        );
    }


    #[test]
    fn ov_state_overlay_on_widget_action_matches() {
        check(
            ssx3_ee::s05::f_0020a6a8__generated,
            ov_state_overlay_on_widget_action,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x20a6a8, 0);
                be_library(c);
                let screen = DATA + 0x8000;
                let vt = DATA + 0x9000;
                match seed % 3 {
                    0 => {}
                    1 => {
                        world(c);
                    }
                    _ => {
                        world(c);
                        c.bus.write32(DATA + 0x6000 + 0x200, screen);
                        c.bus.write32(screen + 0x0c, vt);
                        fake_method(c, vt, 0x48);
                    }
                }
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = sx(DATA + 0x1400);
                c.g[A2][0] = 3;
            },
        );
    }


    /// The session colour setters: an object whose vtable slot 0x58 takes the
    /// colour, and both the on (custom colour) and off (default colour) paths.
    pub(crate) fn color_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = setup(c, seed, salt);
        let obj = DATA + 0x900;
        c.g[A0][0] = DATA as u64;
        c.g[A1][0] = obj as u64;
        c.g[A2][0] = if seed % 2 == 0 {
            0
        } else {
            r.next_u32() as u64
        };
        let vt = vtable(c, &mut r, &[0x58]);
        c.bus.write32(obj + 8, vt);
    }


    #[test]
    fn ov_screen_session_set_highlight_color_matches() {
        check(
            ssx3_ee::s05::f_002095e8__generated,
            ov_screen_session_set_highlight_color,
            120,
            &|c, seed| color_setup(c, seed, 0x2095e8),
        );
    }


    #[test]
    fn ov_screen_session_set_unlocked_color_matches() {
        check(
            ssx3_ee::s05::f_00209648__generated,
            ov_screen_session_set_unlocked_color,
            120,
            &|c, seed| color_setup(c, seed, 0x209648),
        );
    }


    #[test]
    fn ov_screen_session_on_widget_notify_matches() {
        check(ssx3_ee::s05::f_00208ef8__generated, ov_screen_session_on_widget_notify, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x208ef8, 0);
            c.g[A2][0] = (seed % 13) as u64;
        });
    }


    #[test]
    fn ov_screen_session_hide_map_pic_matches() {
        check(
            ee::s05::f_00208bb8__generated,
            ov_screen_session_hide_map_pic,
            60,
            &|c, seed| {
                machine(c, seed, 0x208b_b8, 0);
                c.g[A0][0] = sx(DATA);
                let a = if seed % 3 == 0 { 0 } else { DATA + 0x1000 };
                let b = if seed % 2 == 0 { 0 } else { DATA + 0x2000 };
                c.bus.write32(DATA + 0xd0, a);
                c.bus.write32(DATA + 0xe0, b);
                if a != 0 {
                    put_vtable(c, a, &[0x48]);
                }
                if b != 0 {
                    put_vtable(c, b, &[0x48]);
                }
            },
        );
    }


    #[test]
    fn ov_screen_session_post_input_matches() {
        check(
            ee::s05::f_00209300__generated,
            ov_screen_session_post_input,
            60,
            &|c, seed| {
                machine(c, seed, 0x2093_00, 0);
                let obj = DATA + 0x300;
                let vt = DATA + 0x400;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(obj);
                c.bus.write32(obj + 8, vt);
                put_vtable(c, vt, &[0x88, 0x90]);
                // The two status methods are answered by
                // UIListNode_IsSentinel (a real function returning 0/1), so
                // the zero-result branches are reachable.
                c.bus.write32(vt + 0x88, 0);
                c.bus.write16(vt + 0x88, 0);
                c.bus.write32(vt + 0x8c, 0x0039_7718);
                c.bus.write32(vt + 0x94, 0x0039_7718);
                if seed % 3 == 0 {
                    c.bus.write32(obj + 4, obj);
                } else {
                    c.bus.write32(obj, 0);
                    c.bus.write32(obj + 4, 0);
                }
                if seed % 3 == 2 {
                    c.bus.write16(vt + 0x90, 0x40);
                    let n = obj + 0x40;
                    c.bus.write32(n, n);
                    c.bus.write32(n + 4, n);
                }
                c.bus.write32(DATA + 0xe0, 0);
                c.bus.write32(DATA + 0xd4, 0);
                c.bus.write32(DATA + 0xc0, 0);
            },
        );
    }


    /// Run a guest helper during setup; returns its `v0`.
    pub(crate) fn run(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = TEST_RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, TEST_RA as u32, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }


    /// `StrHash` (0x317670).
    pub(crate) fn str_hash(c: &mut Cpu, s: u32) -> u32 {
        run(c, 0x0031_7670, ee::s10::f_00317670, &[s as u64]) as u32
    }


    #[test]
    fn ov_screen_session_on_activate_matches() {
        check(
            ee::s05::f_00208b00__generated,
            ov_screen_session_on_activate,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x208b00, 5);
                be_library_2(c);
                world_2(c);
                heap_init(c);
                let this = DATA;
                let screen = DATA + 0x400;
                let arr = DATA + 0x500;
                let child = DATA + 0x600;
                // `StrHash` clobbers the argument registers: hash first.
                let hash = str_hash(c, 0x0047_0a20);
                c.g[A0][0] = this as u64;
                if seed % 2 == 0 {
                    w(c, this + 0xd4, 0);
                } else {
                    w(c, this + 0xd4, screen);
                    w(c, screen + 0x38, arr);
                    w(c, screen + 0x3c, arr + 8);
                    w(c, arr, 1);
                    w(c, arr + 8, child);
                    w(c, child + 0x38, hash);
                    w(c, child + 0x8, DATA + 0x2000);
                    slot(c, DATA + 0x2000, 0x38, 0, FAKE);
                    slot(c, DATA + 0x2000, 0x48, 0, FAKE + 0x10);
                }
            },
        );
    }


    #[test]
    fn ov_screen_session_load_map_image_matches() {
        check(ee::s05::f_00208f10__generated, ov_screen_session_load_map_image, SEEDS_3, &|c, seed| {
            let mut r = machine(c, seed, 0x208f10, 4);
            be_library_3(c);
            let obj = DATA;
            c.g[A0][0] = obj as u64;
            w(c, obj + 0xc4, r.next_u32() % 4);
            let wld = DATA + 0x1000;
            w(c, GAME_GP.wrapping_sub(0x848), wld);
            w(c, wld + 0x11c, DATA + 0x2000);
            w(c, DATA + 0x2000, 0);
            w(c, DATA + 0x30_0000, 0);
        });
    }


        #[test]
    fn c_ov_state_overlay_play_object_anim_matches() {
        check(
            ee::s05::f_0020a6f0__generated,
            c_ov_state_overlay_play_object_anim,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x20a6f0, 2);
                put_str(c, DATA + 0x500, b"overlay");
                let h = str_hash(c, DATA + 0x500);
                let screen = DATA + 0x1000;
                screen_with(c, &mut r, screen, h);
                c.bus.write32(DATA + 0x10, DATA + 0x2000);
                c.bus.write32(DATA + 0x2050, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x500);
                c.g[A2][0] = sx(DATA + 0x600);
                c.bus
                    .write32(DATA + 0x40, if seed % 4 == 0 { 0 } else { screen });
            },
        );
    }


    pub(crate) const SESSION: u32 = DATA + 0x1_0000;

    pub(crate) const SESSION_IMG: u32 = DATA + 0x1_1000;


    #[test]
    fn ov_screen_session_update_matches() {
        check_all(
            ee::s05::f_00209370__generated,
            ov_screen_session_update,
            30,
            &|c, seed| {
                let mut r = base(c, seed, 0x2093_70, 6);
                c.g[A0][0] = sx(SESSION);
                c.bus.write32(SESSION + 0xdc, (seed % 2) as u32); // busy
                c.bus.write32(SESSION + 0xc8, ((seed / 2) % 2) as u32); // loaded
                c.bus.write32(SESSION + 0xcc, r.edge_f32()); // fade
                c.bus
                    .write32(SESSION + 0xd0, if seed % 3 == 0 { 0 } else { SESSION_IMG });
                c.bus.write32(SESSION + 0xc4, (seed % 4) as u32); // async file
                c.bus.write32(SESSION + 0x1cc, r.edge_f32()); // bob
                c.bus.write32(SESSION + 0x1d0, r.f32(1.0)); // bob velocity
                                                            // The eight icons at +0x104 stay null: the loop only advances
                                                            // the icon positions.
            },
        );
    }


    pub(crate) const MAP_IND: u32 = DATA + 0x1_0000;

    pub(crate) const MAP_SCREEN: u32 = DATA + 0x1_1000;

    pub(crate) const MAP_LIST: u32 = DATA + 0x1_2000;

    pub(crate) const MAP_ICONS: u32 = DATA + 0x1_3000;


    #[test]
    fn ov_screen_session_update_indicators_matches() {
        check_all(
            ee::s05::f_00209e78__generated,
            ov_screen_session_update_indicators,
            24,
            &|c, seed| {
                let _ = base(c, seed, 0x209e_78, 12);
                c.g[A0][0] = sx(MAP_IND);
                c.bus
                    .write32(MAP_IND + 0x0, if seed % 2 == 0 { 0 } else { MAP_ICONS });
                c.bus.write32(MAP_IND + 0x4, MAP_SCREEN);
                c.bus.write32(MAP_IND + 0x8, (seed % 4) as u32); // count
                c.bus.write32(MAP_IND + 0xc, MAP_LIST); // list object
                c.bus.write32(MAP_SCREEN + 0xa0, MAP_LIST);
                c.bus
                    .write32(MAP_LIST + 0x18, ((seed % 4) as u32).wrapping_add(1));
                for i in 0..4u32 {
                    c.bus.write32(MAP_IND + 0x10 + 4 * i, MAP_ICONS + 0x100 * i);
                    c.bus
                        .write32(MAP_IND + 0x20 + 4 * i, MAP_ICONS + 0x800 + 0x100 * i);
                }
            },
        );
    }
}
