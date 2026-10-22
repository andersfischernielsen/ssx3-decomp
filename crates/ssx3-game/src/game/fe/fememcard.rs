//! `src/fe/fememcard.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::call::VSlot;
use crate::call::sp;
use crate::events::back_edge_unwinds;
use callee::*;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
pub(crate) use crate::game::fe::ovstatepause::lw;
pub(crate) use crate::game::fe::ovstatepause::p;
pub(crate) use crate::game::dirtysock::tags2::at;
pub(crate) use crate::game::dirtysock::tags2::p_2;
pub(crate) use crate::game::dirtysock::tags2::gp_5 as gp;
pub(crate) use crate::game::dirtysock::tags2::sw;
pub(crate) use crate::game::dirtysock::tags2::leave;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_2;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw_2;
pub(crate) use crate::game::wscript::wscriptman::sw as sw_2;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_3;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw_3;
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::ai::computer::sd;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0018_6b50, c_fe_state_mem_card_init, "cFEStateMemCard_Init"),
    (0x0018_6bf0, cfe_mem_card_update, "cFEStateMemCard_Update"),
    (0x0018_6c98, cfe_state_mem_card_show_not_formatted_msg, "cFEStateMemCard_ShowNotFormattedMsg"),
    (0x0018_6f40, c_fe_state_mem_card_is_scan_done_with_card, "cFEStateMemCard_IsScanDoneWithCard"),
    (0x0018_7148, cfe_state_mem_card_close_and_set_state6, "cFEStateMemCard_CloseAndSetState6"),
    (0x0018_7180, fe_state_memcard_update_card_scan, "cFEStateMemCard_UpdateCardScan"),
    (0x0018_71a8, c_fe_state_mem_card_restart_scan, "cFEStateMemCard_RestartScan"),
    (0x0018_7230, c_fe_state_mem_card_reset_scan, "cFEStateMemCard_ResetScan"),
    (0x0018_7270, cfe_mem_card_on_scan_done, "cFEStateMemCard_OnScanDone"),
    (0x0018_7318, fe_state_mem_card_finish, "cFEStateMemCard_Finish"),
    (0x0018_74e8, fe_state_mem_card_v33, "cFEStateMemCard_v33"),
    (0x0018_7a38, c_fe_state_mem_card_scan_step, "cFEStateMemCard_ScanStep"),
];

// ---------------------------------------------------------------------------
// Callees.

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
    pub(crate) use crate::game::dirtysock::tags2::callee::REWARDS_FILE_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOUNDED_CAMERA_ALGORITHM_BASE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_IS_EVENT_LOADED;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_CLOSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_LISTS;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_LOBBY_API;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_DESTROY_PEER_TABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SHARED_LOGIN_MAN_SHUTDOWN;
    pub(crate) use crate::game::dirtysock::tags2::callee::VOIP_MAN_STOP;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_CACHE_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_SNAPSHOT_EMPTY_HOOK_B;
    pub(crate) use crate::game::dirtysock::tags2::callee::WORLD_STATE_RESTORE;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_RESTORE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_FREE_SNAPSHOT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_END_PLAYBACK;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_CHANNEL_RELEASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_HEAP_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ANIM_BANK_MAN_FIND_BANK;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS_2 as SCRIPT_ENGINE_POLL_ASYNC_LOADS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_IS_BANK_SLOT_LOADED;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_CURRENT_SKIPPABLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_FIELD_B0;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_FIND_BANK_SLOT_BY_ID;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_CANCEL_BANK_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_LIST_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::dirtysock::tags2::callee::BUILTIN_DELETE as BUILTIN_DELETE_2;
    pub(crate) use crate::game::dirtysock::tags2::callee::EA_MEM_FREE_2 as EA_MEM_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::FEMODULE_SET_PREVIEW_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SSX_APP_START_FE_MODULE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOC_STRING_TABLE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BENEW_PLAYER_INT_IS_MESSAGE_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_MESSAGE_FLAG_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::BE_GET_INTERFACE_RIDER;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MAN_LOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_MAN_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::WORLD_VIEW_SET;
    pub(crate) use crate::game::dirtysock::tags2::callee::MENU_SYS_EVENT;
    pub(crate) use crate::game::dirtysock::tags2::callee::REPLAY_STREAM_WRITE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_REQUEST_UPDATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_REQUEST_FREE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_API_ADD_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::DIRTY_SOCK_CREATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::DIRTY_SOCK_DESTROY;
    pub(crate) use crate::game::dirtysock::tags2::callee::VID_ENGINE_PLAYER_REQUEST;
    pub(crate) use crate::game::dirtysock::tags2::callee::APP_SOMETHING;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUILTIN_DELETE_6 as BUILTIN_DELETE_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_6 as SPRINTF;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_MESSENGER_CLEAR_NAME_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_DTOR_5 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_5 as NET_FE_STATE_SHOW_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP as NET_FE_STATE_GET_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_USE_EXISTING_ACCOUNT_CHECK_EMAIL;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_CONNECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_BX_STRING_ASSIGN_CSTR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_SET_LIST_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_SHOW_CARD_ERROR_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_LOAD_CONFIG_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_QUICK_PLAY_POPUP_SET_OPTIONS_HINT_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_QUICK_PLAY_POPUP_SET_OPTIONS_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_GET_OWNED_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_GET_ID_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_SET_TEXT_ID_2 as UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INSTANCE_4 as BE_LIBRARY_GET_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INTERFACE_4 as BE_LIBRARY_GET_INTERFACE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_HUMAN_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_ADVANCE_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_TICK_PROCESS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_3 as SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_BINDING;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_KILL_TIMERS_BY_CALLBACK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_UNLOAD_BANK_2 as SND_BANK_MAN_UNLOAD_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_POPUP_FIND_ENABLED_IN_COLUMN_FROM_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_CTOR_2_4 as UI_STATE_CTOR_2_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_PLAYER_PAD_MASK_2 as FE_MODULE_GET_PLAYER_PAD_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_FADE_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_SET_FADE_GAIN;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_AS_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_TABLE_FIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_PASS_THROUGH;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_CALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR_2 as LUNO_VM_REGISTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_GET_BITS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_MARKER_BIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_EXTENSION_AND_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_GET_START_POSE_ANGLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_PLACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_DATA_FIND_MARKER;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC_NAMED_128;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_ALL_POPUPS_2 as UI_STATE_MGR_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_SWITCH_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_5 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_MENU_SET_SELECTION_3 as UI_MENU_SET_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_QUIT_TO_LOAD_SCREEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_START_SEQUENCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_USER_DATA_2 as IN_SEQ_INSTANCE_SET_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_SPEED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_TRANSITION_TARGET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_TRANSITION_MAN_START;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_TRANSITION_MAN_START_CAPTURED;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_CUT_SCENE_TRANSITION_MAN_SAVE_VIEWPORT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_RACE_INT_GET_TRACK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_RACE_INT_GET_SPECIAL_EVENT_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_METRIX_SHOW_HUD_MSG_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_RELEASE_BOLT_FILES;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_UNLOAD_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_BINDING_EVAL_FLOAT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_FORMAT_SLOT_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_SPRINTF_ASCII_FMT;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_TO_ASCII;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_LEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ASYNC_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_MISSION_INT_GET_SESSION_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_SETUP_MAN_ALLOC_PACKET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_SETUP_MAN_FREE_PACKET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_COMM_SYSTEM_CHANNEL_RECV;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_COMM_SYSTEM_CHANNEL_SEND;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_BX_STRING_CTOR_CSTR;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED_4 as OPERATOR_NEW_NAMED_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GOTO_NEXT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_SSX_WORLD_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_OPTIONS_DEVICE_SELECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_IS_SPEECH_VOLUME_ON;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_SYSTEM_IS_INSTANCE_READY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_CHARACTER_TO_SPEECH_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPCH_MAKE_ID_2 as SPCH_MAKE_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_IS_LOADED;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_GET_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_RIDER_GET_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SCRIPT_OBJ_MATCHES_RIDER_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_INSTANCE_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_BUY_SELECTED_ITEM;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_UPDATE_MONEY_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_REMOVE_LIST_ITEM;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_REFRESH_ITEM_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_TROPHY_ROOM_SHOW_TROPHY_IMAGE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_TROPHY_ROOM_SHOW_TROPHY_INFO;
}

/// Addresses the generator emitted as `call_indirect` (outside the shard's
/// own direct-call set, so no readable alias is needed here).
pub(crate) mod addr {
    pub(crate) use crate::game::fe::feasyncfile::addr::__BUILTIN_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::addr::EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::addr::C_BX_STRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::addr::__FIXUNSSFSI;
    pub(crate) use crate::game::fe::feasyncfile::addr::STRLEN;
    pub(crate) use crate::game::fe::feasyncfile::addr::FONT_DTOR;
}


// ---------------------------------------------------------------------------
// cFEStateMemCard_UpdateCardScan.

/// `void cFEStateMemCard_UpdateCardScan(cFEStateMemCard* s)` (0x187180): poll
/// the memory cards, then advance the scan at `D_004A14B8`.
pub fn fe_state_memcard_update_card_scan(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x001d_83a8, 0x0018_7190)?;
        c.g[A0][0] = c.lw(r32(c, GP).wrapping_sub(0x1c38));
        callee::MEMCARD_SCAN_STEP.call(c, 0x0018_7198)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateMemCard_CloseAndSetState6(void)` (0x187148): call vtable slot
/// 0x148 of the current front-end state, then move it to state 6.
pub fn cfe_state_mem_card_close_and_set_state6(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1c38)).lw(c);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(c.g[A1][0] as u32).off(8).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x148).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x14c).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0018_7168)?;
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1c38)).lw(c);
        c.g[V0][0] = 6;
        frame.restore_ra(c, 0);
        let a0 = c.g[A0][0];
        let v0 = c.g[V0][0];
        Addr(a0 as u32).off(0x1c0).sw(c, v0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateMemCard_ResetScan(void)` (0x187230): restart the memory
/// card manager's scan (mode 2) with no slot selected.
pub fn c_fe_state_mem_card_reset_scan(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x10);
        let g = crate::abi::r32(c, GP);
        c.g[A0][0] = lw(c, g, 0xfffff7b8); // gp - 0x848
        frame.save_ra(c, 0);
        callee::MC_MANAGER_GET.call(c, 0x0018_7244)?;
        let g = crate::abi::r32(c, GP);
        let mgr = lw(c, g, 0xffffe3c8); // gp - 0x1c38
        c.g[A2][0] = mgr;
        c.g[V1][0] = u64::MAX;
        Addr(crate::abi::r32(c, GP).wrapping_sub(0x1c34)).sw(c, 0);
        c.g[A0][0] = c.g[V0][0];
        Addr(p(mgr as u32, 0x22c)).sw(c, c.g[V1][0]);
        c.g[A1][0] = 2;
        Addr(p(mgr as u32, 0x234)).sw(c, 0);
        callee::MC_MANAGER_BEGIN_LOAD.call(c, 0x0018_7264)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateMemCard_Finish(cFEStateMemCard* self)` (0x187318): ask the
/// front end to start its module once (when the "started" flag is clear),
/// then mark the state started.
pub fn fe_state_mem_card_finish(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 6;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        sw(c, r32(c, S0), 0x1c0, c.g[V0][0]);
        c.g[V1][0] = c.lw(at(c, S0, 0x1ac));
        let already = c.g[V1][0] != 0;
        c.g[V0][0] = 1;
        if !already {
            c.cycles += 3;
            c.g[A0][0] = c.lw(gp(c, -0x848));
            call_ptr(c, callee::SSX_APP_START_FE_MODULE, 0x0018_7344)?;
            c.g[V0][0] = 1;
        }
        c.cycles += 5;
        frame.restore_ra(c, 0);
        sw(c, r32(c, S0), 0x1ac, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        leave(c, 0x20);
        Ok(())
    });
}


/// `bool cFEStateMemCard_IsScanDoneWithCard(self)` (0x186f40): false until the
/// memory-card manager is in a scan state (>= 4) and reports the card ready.
pub fn c_fe_state_mem_card_is_scan_done_with_card(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0022_7f80, 0x0018_6f54)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x1c34)).lw(c);
        c.g[V0][0] = ((c.g[V0][0] as i64) < 4) as u64;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 5;
            call_ptr(c, 0x0023_c8d0, 0x0018_6f70)?;
            c.g[V1][0] = c.g[V0][0];
            let ready = c.g[V1][0] != 0;
            c.g[V0][0] = 1;
            if !ready {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateMemCard_RestartScan(cFEStateMemCard* self)` (0x1871a8):
/// rewind the memory-card manager, close its slots, start the card state's
/// own scan and switch the UI state.
pub fn c_fe_state_mem_card_restart_scan(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x0018_71c0);
        call_ptr(c, 0x0022_7f80, 0x0018_71c0)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        c.g[RA][0] = sx(0x0018_71d0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0023_fb18, 0x0018_71d0)?;
        c.g[RA][0] = sx(0x0018_71d8);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0023_fae0, 0x0018_71d8)?;
        c.g[V0][0] = Addr(p(r32(c, S0), 0x748)).lw(c);
        c.g[A1][0] = 0x36;
        c.g[A0][0] = Addr(p(r32(c, V0), 8)).lh(c);
        c.g[V1][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
        let target = r32(c, V1);
        c.g[RA][0] = sx(0x0018_71f0);
        c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
        call_ptr(c, target, 0x0018_71f0)?;
        c.g[A2][0] = Addr(gp(c, -0x1c38)).lw(c);
        c.g[V0][0] = u64::MAX;
        Addr(gp(c, -0x1c34)).sw(c, 0);
        c.g[A0][0] = c.g[S0][0];
        Addr(p(r32(c, A2), 0x22c)).sw(c, c.g[V0][0]);
        c.g[A1][0] = 0x2;
        c.g[RA][0] = sx(0x0018_7210);
        Addr(p(r32(c, A2), 0x234)).sw(c, 0);
        call_ptr(c, 0x0023_c8f0, 0x0018_7210)?;
        c.g[V1][0] = Addr(gp(c, -0x1c38)).lw(c);
        c.g[V0][0] = 0x2;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        Addr(p(r32(c, V1), 0x1bc)).sw(c, c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateMemCard_v33` (0x1874e8): memory-card state helper: once the card
/// worker is busy, finish; otherwise start the profile search unless a save
/// is in progress.
pub fn fe_state_mem_card_v33(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        c.g[V0][0] = Addr(p(this, 0x1ac)).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 6;
            c.g[RA][0] = sx(0x0018_750c);
            c.g[A0][0] = Addr(gp_2(c, -0x848)).lw(c);
            call_ptr(c, callee::MC_GET_WORLD, 0x0018_750c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[V0][0] = Addr(p(r(c, A0), 0x440)).lw(c);
            let busy = c.g[V0][0] != 0;
            c.g[V1][0] = 0;
            if !busy {
                c.cycles += 2;
                c.g[V0][0] = Addr(p(r(c, A0), 0x43c)).lw(c);
                c.g[V1][0] = (0u64 < c.g[V0][0]) as u64;
            }
            c.cycles += 2;
            let mut early = false;
            if c.g[V1][0] != 0 {
                frame.restore(c, S0, 0x10);
                early = true;
            } else {
                c.cycles += 4;
                c.g[V1][0] = Addr(gp_2(c, -0x1c34)).lw(c);
                c.g[V0][0] = 4;
                let mut finish = true;
                if c.g[V1][0] == c.g[V0][0] {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(p(this, 0x238)).lw(c);
                    c.g[V0][0] = c.g[V0][0] ^ 1;
                    c.g[V0][0] = c.g[V0][0] & 1;
                    if c.g[V0][0] != 0 {
                        c.cycles += 6;
                        c.g[RA][0] = sx(0x0018_7558);
                        call_ptr(c, callee::MC_MANAGER_GET_2, 0x0018_7558)?;
                        c.g[RA][0] = sx(0x0018_7560);
                        callee::FE_MEMCARD_START_PROFILE_SEARCH.call(c, 0x0018_7560)?;
                        frame.restore(c, S0, 0x10);
                        finish = false;
                    }
                }
                if finish {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x0018_7570);
                    c.g[A0][0] = c.g[S0][0];
                    callee::FE_MEMCARD_FINISH.call(c, 0x0018_7570)?;
                    frame.restore(c, S0, 0x10);
                }
                let _ = early;
            }
        } else {
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Memory card front end.

/// `void cFEStateMemCard_Init(cFEStateMemCard* self)` (0x186b50): create the
/// screen named by the state's engine string, start it, then find the "text"
/// object and call its setup method.
pub fn c_fe_state_mem_card_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2390));
        c.g[S0][0] = lw_2(c, p(c.g[S1][0] as u32, 0x10));
        call_ptr(c, 0x0031_7670, 0x0018_6b74)?; // StrHash
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0018_6b88)?; // UIEngine_CreateScreen
        let failed = c.g[V0][0] == 0;
        sw_2(c, p(c.g[S1][0] as u32, 0x40), c.g[V0][0]);
        if !failed {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0018_6ba0)?; // UIScreen_StartThread
        }
        c.cycles += 19;
        c.g[A0][0] = lw_2(c, gp(c, -0x848));
        call_ptr(c, 0x0022_7f80, 0x0018_6ba8)?; // MCManager_Get
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2380));
        call_ptr(c, 0x0031_7670, 0x0018_6bb4)?;
        c.g[A0][0] = lw_2(c, p(c.g[S1][0] as u32, 0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x0018_6bc0)?; // UIScreen_FindObject
        c.g[V1][0] = lw_2(c, p(c.g[V0][0] as u32, 8));
        c.g[A1][0] = 0;
        c.g[A0][0] = Addr(p(c.g[V1][0] as u32, 0x48)).lh(c);
        c.g[A2][0] = lw_2(c, p(c.g[V1][0] as u32, 0x4c));
        c.g[A2][0] = sx(c.g[A2][0] as u32);
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, c.g[A2][0] as u32, 0x0018_6bd8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void cFEStateMemCard_Update(self)` (0x186bf0): run the memcard state,
/// then refresh its status label when the scan finished or the state is 6.
pub fn cfe_mem_card_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x001d_8de0, 0x00186c04)?;
        c.g[A0][0] = Addr(gp(c, -0x848)).lw(c);
        call_ptr(c, 0x0022_7f80, 0x00186c0c)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0023_c898, 0x00186c14)?;
        if c.g[V0][0] == 0 {
            c.cycles += 4;
            c.g[V1][0] = Addr(p(r32(c, S0), 0x1c0)).lw(c);
            c.g[V0][0] = 6;
            if c.g[V1][0] != c.g[V0][0] {
                frame.restore(c, S0, 0x10);
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 1;
        }
        c.g[A0][0] = sx(0x004a_0000);
        c.cycles += 7;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x14c0));
        call_ptr(c, 0x0031_7670, 0x00186c38)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x00186c44)?;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0039_fd38, 0x00186c54)?;
        }
        c.g[A0][0] = sx(0x0046_0000);
        c.cycles += 12;
        c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0x2380));
        call_ptr(c, 0x0031_7670, 0x00186c60)?;
        c.g[A0][0] = Addr(p(r32(c, S0), 0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x00186c6c)?;
        c.g[V1][0] = Addr(p(r32(c, V0), 8)).lw(c);
        c.g[A1][0] = 0;
        c.g[A0][0] = Addr(p(r32(c, V1), 0x48)).lh(c);
        c.g[A2][0] = Addr(p(r32(c, V1), 0x4c)).lw(c);
        let set = r32(c, A2);
        c.g[A0][0] = sx(p(r32(c, V0), r32(c, A0)));
        call_ptr(c, set, 0x00186c84)?;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateMemCard_OnScanDone(self)` (0x187270): start the system load
/// or the profile search according to the pending flags, else finish.
pub fn cfe_mem_card_on_scan_done(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 1;
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(gp(c, -0x848)).lw(c);
        Addr(p(r32(c, S1), 0x234)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0022_7f80, 0x00187294)?;
        c.g[V1][0] = Addr(p(r32(c, S1), 0x238)).lw(c);
        c.g[S0][0] = c.g[V0][0];
        c.g[V0][0] = c.g[V1][0] & 2;
        let flagged = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[V1][0] ^ 1;
        if flagged {
            c.cycles += 3;
            c.g[V0][0] = c.g[V0][0] & 1;
            let finish = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if finish {
                c.cycles += 3;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x0018_7318, 0x00187300)?;
            } else {
                c.cycles += 8;
                c.g[A1][0] = 0;
                call_ptr(c, 0x0023_fb18, 0x001872e0)?;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0023_fae0, 0x001872e8)?;
                call_ptr(c, 0x0018_77b0, 0x001872f0)?;
            }
        } else {
            c.cycles += 9;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0023_fb18, 0x001872b4)?;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0023_fae0, 0x001872bc)?;
            call_ptr(c, 0x0018_7920, 0x001872c4)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateMemCard_ShowNotFormattedMsg(this)` (0x186c98): format "slot N"
/// for the current slot and open the memcard state's not-formatted dialog.
pub fn cfe_state_mem_card_show_not_formatted_msg(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 49;
        let frame = Frame::open(c, 0x6a0);
        c.g[A0][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save(c, 16, 0x690);
        frame.save(c, 17, 0x680);
        frame.save(c, 18, 0x670);
        frame.save_ra(c, 0x660);
        callee::MC_MANAGER_GET.call(c, 0x0018_6cb8)?;
        let mp = abi::r32(c, V0);
        c.g[A0][0] = mp as u64;
        c.g[S2][0] = sx(sp(c).wrapping_add(0x640));
        c.g[A2][0] = Addr(mp).off(0x428).lw(c);
        c.g[A1][0] = c.g[S2][0];
        callee::MC_MANAGER_FORMAT_SLOT_NAME.call(c, 0x0018_6ccc)?;

        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_sub(0x2370));
        c.g[S0][0] = Addr(c.g[V0][0] as u32).off(0x8c).lw(c);
        c.g[S1][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x20).lh(c);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        callee::STR_HASH.call(c, 0x0018_6cec)?;
        c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x24).lw(c);
        c.g[A0][0] = c.g[S0][0];
        let target = c.g[V1][0] as u32;
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, target, 0x0018_6cfc)?;

        c.g[A0][0] = sp(c) as u64;
        c.g[A1][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S2][0];
        callee::U_STR_SPRINTF.call(c, 0x0018_6d0c)?;

        c.g[V1][0] = Addr(gp(c, -0x1c38)).lw(c);
        c.g[V0][0] = 4;
        c.g[A1][0] = sp(c) as u64;
        c.g[A2][0] = 1;
        Addr(c.g[V1][0] as u32).off(0x19c).sw(c, c.g[V0][0]);
        c.g[A3][0] = 1;
        c.g[T3][0] = Addr(c.g[V1][0] as u32).off(8).lw(c);
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        c.g[T2][0] = 0;
        c.g[A0][0] = Addr(c.g[T3][0] as u32).off(0x120).lh(c);
        c.g[V0][0] = Addr(c.g[T3][0] as u32).off(0x124).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0018_6d44)?;

        frame.restore(c, 16, 0x690);
        frame.restore(c, 17, 0x680);
        frame.restore(c, 18, 0x670);
        frame.restore_ra(c, 0x660);
        frame.close(c);
        Ok(())
    })
}


/// `void func_00187A38(void* self)` (0x187a38): one step of the memory-card
/// scan state machine: drive the card manager's scan (state in `D_004A14BC`),
/// turn the screen's flags off when the card call reports busy, and save or
/// restore the screen's saved flags.
pub fn c_fe_state_mem_card_scan_step(c: &mut Cpu) {
    let frame = Frame::open(c, 0x40);
    'b_187a78: {
        c.cycles += 15;
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[A0][0] = lw_3(c, r32(c, GP).wrapping_sub(0x848));
        frame.save(c, S1, 0x20);
        sd(c, r32(c, SP), c.g[RA][0]);
        c.g[RA][0] = sx(0x00187a5c);
        if call_ptr(c, 0x00227f80, 0x00187a5c).is_err() {
            return;
        }
        c.g[S1][0] = c.g[V0][0];
        c.g[S0][0] = lw_3(c, at(c, S1, 0x434));
        c.g[RA][0] = sx(0x00187a6c);
        c.g[A0][0] = c.g[S1][0];
        if call_ptr(c, 0x0023c8d0, 0x00187a6c).is_err() {
            return;
        }
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0x3;
        if t {
            break 'b_187a78;
        } else {
            c.cycles += 1;
            sw_3(c, r32(c, GP).wrapping_sub(0x1c34), c.g[V0][0]);
            break 'b_187a78;
        }
    }
    'b_187c00: {
        'b_187bfc: {
            'b_187bf8: {
                'b_187be0: {
                    'b_187bac: {
                        'b_187b14: {
                            c.cycles += 4;
                            c.g[V1][0] = lw_3(c, r32(c, GP).wrapping_sub(0x1c34));
                            c.g[V0][0] = 0x1;
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                            if t {
                                break 'b_187b14;
                            } else {
                                'b_187ab8: {
                                    'b_187aa0: {
                                        c.cycles += 2;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[V0][0] = 0x2;
                                            break 'b_187aa0;
                                        } else {
                                            c.cycles += 2;
                                            if c.g[V1][0] == 0u64 {
                                                c.g[V1][0] = lw_3(c, r32(c, S0));
                                                break 'b_187ab8;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                frame.restore(c, S0, 0x30);
                                                if t {
                                                    break 'b_187bfc;
                                                } else {
                                                    break 'b_187aa0;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = c.g[V1][0] == c.g[V0][0];
                                    c.g[V0][0] = 0x3;
                                    if t {
                                        break 'b_187bac;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] == c.g[V0][0];
                                        frame.restore(c, S0, 0x30);
                                        if t {
                                            break 'b_187be0;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            frame.restore(c, S1, 0x20);
                                            if t {
                                                break 'b_187c00;
                                            } else {
                                                break 'b_187ab8;
                                            }
                                        }
                                    }
                                }
                                'b_187adc: {
                                    c.cycles += 6;
                                    c.g[A0][0] = lh(c, at(c, V1, 0x68));
                                    c.g[V0][0] = lw_3(c, at(c, V1, 0x6c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x00187ac8);
                                    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                                    if call_ptr(c, t, 0x00187ac8).is_err() {
                                        return;
                                    }
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V1][0] = 0xfffffffffffffffd;
                                    if t {
                                        break 'b_187adc;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = lw_3(c, at(c, S2, 0x22c));
                                        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                                        sw_3(c, at(c, S2, 0x22c), c.g[V0][0]);
                                        break 'b_187adc;
                                    }
                                }
                                c.cycles += 14;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[RA][0] = sx(0x00187ae8);
                                c.g[A1][0] = 0x1;
                                if call_ptr(c, 0x0023c8f0, 0x00187ae8).is_err() {
                                    return;
                                }
                                c.g[V0][0] = lw_3(c, at(c, S1, 0x748));
                                c.g[A1][0] = 0x33;
                                c.g[A0][0] = lh(c, at(c, V0, 0x8));
                                c.g[V1][0] = lw_3(c, at(c, V0, 0xc));
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x00187b00);
                                c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                                if call_ptr(c, t, 0x00187b00).is_err() {
                                    return;
                                }
                                c.g[V0][0] = lw_3(c, r32(c, GP).wrapping_sub(0x1c34));
                                c.g[V0][0] = sx(at(c, V0, 0x1));
                                sw_3(c, r32(c, GP).wrapping_sub(0x1c34), c.g[V0][0]);
                                let t = true;
                                sw_3(c, at(c, S2, 0x1bc), 0u64);
                                if t {
                                    break 'b_187bf8;
                                } else {
                                    break 'b_187b14;
                                }
                            }
                        }
                        'b_187b3c: {
                            c.cycles += 7;
                            c.g[V1][0] = lw_3(c, r32(c, S0));
                            c.g[A0][0] = lh(c, at(c, V1, 0x68));
                            c.g[V0][0] = lw_3(c, at(c, V1, 0x6c));
                            let t = c.g[V0][0] as u32;
                            c.g[RA][0] = sx(0x00187b28);
                            c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                            if call_ptr(c, t, 0x00187b28).is_err() {
                                return;
                            }
                            let t = c.g[V0][0] == 0u64;
                            c.g[V1][0] = 0xfffffffffffffffe;
                            if t {
                                break 'b_187b3c;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = lw_3(c, at(c, S2, 0x22c));
                                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                                sw_3(c, at(c, S2, 0x22c), c.g[V0][0]);
                                break 'b_187b3c;
                            }
                        }
                        'b_187b74: {
                            c.cycles += 3;
                            c.g[V0][0] = lw_3(c, at(c, S1, 0x428));
                            if c.g[V0][0] != 0u64 {
                                c.g[A0][0] = c.g[S1][0];
                                break 'b_187b74;
                            } else {
                                'b_187b70: {
                                    c.cycles += 9;
                                    c.g[A1][0] = lw_3(c, at(c, S1, 0x434));
                                    c.g[V1][0] = lw_3(c, r32(c, A1));
                                    c.g[A0][0] = lh(c, at(c, V1, 0x88));
                                    c.g[V0][0] = lw_3(c, at(c, V1, 0x8c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x00187b60);
                                    c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
                                    if call_ptr(c, t, 0x00187b60).is_err() {
                                        return;
                                    }
                                    c.g[V1][0] = 0xffffffffffffffff;
                                    let t = c.g[V0][0] != c.g[V1][0];
                                    sw_3(c, r32(c, GP).wrapping_sub(0x1c3c), c.g[V0][0]);
                                    if t {
                                        break 'b_187b70;
                                    } else {
                                        c.cycles += 1;
                                        sw_3(c, r32(c, GP).wrapping_sub(0x1c3c), 0u64);
                                        break 'b_187b70;
                                    }
                                }
                                c.cycles += 1;
                                c.g[A0][0] = c.g[S1][0];
                                break 'b_187b74;
                            }
                        }
                        c.cycles += 14;
                        c.g[RA][0] = sx(0x00187b7c);
                        c.g[A1][0] = 0u64;
                        if call_ptr(c, 0x0023c8f0, 0x00187b7c).is_err() {
                            return;
                        }
                        c.g[V0][0] = lw_3(c, at(c, S1, 0x748));
                        c.g[A1][0] = 0x33;
                        c.g[A0][0] = lh(c, at(c, V0, 0x8));
                        c.g[V1][0] = lw_3(c, at(c, V0, 0xc));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x00187b94);
                        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                        if call_ptr(c, t, 0x00187b94).is_err() {
                            return;
                        }
                        c.g[V0][0] = lw_3(c, r32(c, GP).wrapping_sub(0x1c34));
                        c.g[V1][0] = 0x1;
                        c.g[V0][0] = sx(at(c, V0, 0x1));
                        sw_3(c, r32(c, GP).wrapping_sub(0x1c34), c.g[V0][0]);
                        let t = true;
                        sw_3(c, at(c, S2, 0x1bc), c.g[V1][0]);
                        if t {
                            break 'b_187bf8;
                        } else {
                            break 'b_187bac;
                        }
                    }
                    'b_187bd4: {
                        c.cycles += 7;
                        c.g[V1][0] = lw_3(c, r32(c, S0));
                        c.g[A0][0] = lh(c, at(c, V1, 0x68));
                        c.g[V0][0] = lw_3(c, at(c, V1, 0x6c));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x00187bc0);
                        c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                        if call_ptr(c, t, 0x00187bc0).is_err() {
                            return;
                        }
                        let t = c.g[V0][0] == 0u64;
                        c.g[V1][0] = 0xfffffffffffffffb;
                        if t {
                            break 'b_187bd4;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = lw_3(c, at(c, S2, 0x22c));
                            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                            sw_3(c, at(c, S2, 0x22c), c.g[V0][0]);
                            break 'b_187bd4;
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = lw_3(c, r32(c, GP).wrapping_sub(0x1c34));
                    c.g[V0][0] = sx(at(c, V0, 0x1));
                    sw_3(c, r32(c, GP).wrapping_sub(0x1c34), c.g[V0][0]);
                    break 'b_187be0;
                }
                c.cycles += 3;
                c.g[V0][0] = lw_3(c, at(c, S1, 0x428));
                if c.g[V0][0] != 0u64 {
                    sw_3(c, at(c, S2, 0x234), 0u64);
                    break 'b_187bf8;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = lw_3(c, at(c, S2, 0x22c));
                    sw_3(c, at(c, S2, 0x238), c.g[V0][0]);
                    sw_3(c, at(c, S2, 0x234), 0u64);
                    break 'b_187bf8;
                }
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
            break 'b_187bfc;
        }
        c.cycles += 1;
        frame.restore(c, S1, 0x20);
        break 'b_187c00;
    }
    c.cycles += 4;
    frame.restore(c, S2, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x40));
    c.pc = t;
    return;
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::check;
    use crate::difftest::fill;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::fakes::vtable;
    use crate::difftest::put_words;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::ptr;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::check_lohi;
    use ssx3_ee_rt::funcs::GuestFn;
    use crate::difftest::elf_image;
    use ssx3_ee as ee;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::compare;
    pub(crate) use crate::game::bx::bxstring2::tests::be_library;
    pub(crate) use crate::game::fe::ovstatepause::tests::BE_LIBRARY;
    pub(crate) use crate::game::fe::ovstatepause::tests::WORLD;
    pub(crate) use crate::game::fe::ovstatepause::tests::globals_2 as globals;
    pub(crate) use crate::game::dirtysock::tags2::tests::be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_2 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::h;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_5 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_4 as world;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_4 as BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_3 as W;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_8 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::eafile_pools;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_7 as run;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_8 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_9 as pool_allocator_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_3 as alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear;
    pub(crate) use crate::game::wscript::wscriptman::tests::BE_3 as BE_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::WORLD_3 as WORLD_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::GAME;
    pub(crate) use crate::game::wscript::wscriptman::tests::globals_3 as globals_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_2;
    pub(crate) use crate::game::bx::bxstring::tests::heap;
    pub(crate) use crate::game::dirtysock::tags2::tests::pool_7 as pool;
    pub(crate) use crate::game::bx::bxstring::tests::put_str_2 as put_str;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_3 as OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_3 as OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SCR;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE as BE_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_5 as WORLD_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_10 as globals_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_8 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16 as SEEDS_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_9 as BE_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_7 as WORLD_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::RMAN;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDERS_2 as RIDERS;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDER;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::fake_small;
    pub(crate) use crate::game::fe::fepopup::tests::BEL;
    pub(crate) use crate::game::fe::fepopup::tests::WORLD_4 as WORLD_5;
    pub(crate) use crate::game::fe::fepopup::tests::AUX;
    pub(crate) use crate::game::fe::fepopup::tests::SYS;
    pub(crate) use crate::game::fe::fepopup::tests::RACE;
    pub(crate) use crate::game::fe::fepopup::tests::RMAN_2;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_4;


    #[test]
    fn fe_state_memcard_update_card_scan_matches() {
        crate::difftest::check(ee::s02::f_00187180__generated, fe_state_memcard_update_card_scan, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x1871_80, 2);
            be_library(c);
            // `D_004A14B8`, the card scan state the second call advances.
            c.bus.write32(GAME_GP.wrapping_sub(0x1c38), if seed % 2 == 0 { 0 } else { DATA + 0x800 });
            c.g[A0][0] = sx(DATA);
            c.bus.write32(DATA + 0x800, r.next_u32() % 4);
        });
    }


    /// Write `{delta, fake}` at `at + off` (a gcc vtable slot).
    pub(crate) fn put_slot(c: &mut Cpu, r: &mut Rng, at: u32, off: u32) {
        let delta = (r.next_u32() as i16 as i32 & !3) as u32;
        c.bus.write16(at + off, delta as u16);
        c.bus.write32(at + off + 4, FAKE);
    }


    #[test]
    fn cfe_state_mem_card_close_and_set_state6_matches() {
        check(
            ee::s02::f_00187148__generated,
            cfe_state_mem_card_close_and_set_state6,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1871_48, 0);
                let state = DATA + 0x400;
                fill(c, &mut r, state, 0x40);
                c.bus.write32(GAME_GP.wrapping_sub(0x1c38), state);
                c.bus.write32(state + 8, DATA + 0x700);
                put_slot(c, &mut r, DATA + 0x700, 0x148);
            },
        );
    }


    /// A vtable whose one slot `off` is the fake, installed at `*(obj + 8)`.
    pub(crate) fn vtable_at(c: &mut Cpu, r: &mut Rng, obj: u32, off: u32) -> u32 {
        let vt = vtable(c, r, &[off]);
        c.bus.write32(obj + 8, vt);
        vt
    }


    #[test]
    fn c_fe_state_mem_card_reset_scan_matches() {
        check(
            ssx3_ee::s02::f_00187230__generated,
            c_fe_state_mem_card_reset_scan,
            30,
            &|c, seed| {
                let mut r = machine(c, seed, 0x12, 1);
                globals(c);
                let mgr = DATA + 0x4000;
                // The manager singleton and its scan object, both zeroed
                // dummies (the lazy constructor would call the allocator).
                c.bus.write32(GAME_GP - 0x47c, mgr);
                c.bus.write32(GAME_GP - 0x1c38, mgr);
                c.bus.write32(GAME_GP - 0x1c34, r.next_u32());
                // The card object the manager scans (`+0x434`) and its vtable.
                c.bus.write32(mgr + 0x434, DATA + 0x5000);
                vtable_at(c, &mut r, DATA + 0x5000, 0x98);
            },
        );
    }


    #[test]
    fn fe_state_mem_card_finish_matches() {
        check(
            ssx3_ee::s02::f_00187318__generated,
            fe_state_mem_card_finish,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x187318, 0);
                be_library_2(c);
                pool_allocator(c);
                let this_ = DATA + 0x100;
                c.bus
                    .write32(this_ + 0x1ac, if seed % 2 == 0 { 0 } else { 1 });
                c.g[A0][0] = sx(this_);
            },
        );
    }


    // -----------------------------------------------------------------------
    // 0x186f40 cFEStateMemCard_IsScanDoneWithCard.

    #[test]
    fn c_fe_state_mem_card_is_scan_done_with_card_matches() {
        check(ee::s02::f_00186f40__generated, c_fe_state_mem_card_is_scan_done_with_card, 16, &|c, seed| {
            let _r = machine(c, seed, 0x186f_40, 0);
            let world = DATA + 0x1000;
            let mc = DATA + 0x2000;
            c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
            c.bus.write32(GAME_GP.wrapping_sub(0x47c), mc);
            c.bus.write32(mc + 0x43c, if seed % 2 == 0 { 0 } else { 1 });
            c.bus.write32(GAME_GP.wrapping_sub(0x1c34), (seed % 5) as u32);
        });
    }


    #[test]
    fn c_fe_state_mem_card_restart_scan_matches() {
        check(
            ssx3_ee::s02::f_001871a8__generated,
            c_fe_state_mem_card_restart_scan,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1871a8, 6);
                be_library_3(c);
                world(c);
                let mgr = DATA + 0x200;
                let vt = DATA + 0x400;
                w(c, GAME_GP.wrapping_sub(0x1c38), mgr);
                w(c, GAME_GP.wrapping_sub(0x1c34), 0);
                w(c, mgr + 0x748, vt);
                // The slot itself is a vtable: delta at +8, function at +0xc.
                slot(c, vt, 8, 0, FAKE);
                slot(c, vt, 0xc, 0, FAKE + 0x10);
                c.g[A0][0] = DATA as u64;
                let _ = r.next_u32();
            },
        );
    }


        #[test]
    fn fe_state_mem_card_v33_matches() {
        check(
            ee::s02::f_001874e8__generated,
            fe_state_mem_card_v33,
            SEEDS_2,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                w(c, this + 0x1ac, (seed & 1) as u32);
                w(c, gp_2(c, -0x1c34), (seed % 6) as u32);
                w(c, this + 0x238, (seed & 2) as u32);
                c.g[A0][0] = this as u64;
            }),
        );
    }


    // ----------------------------------------------------------- mem card

    #[test]
    fn c_fe_state_mem_card_init_matches() {
        check(ssx3_ee::s02::f_00186b50__generated, c_fe_state_mem_card_init, 8, &|c, seed| {
            let _ = base_2(c, seed, 0x0018_6b50);
            elf_image(c);
            heap(c);
            pool(c);
            let state = DATA + 0x100;
            let engine = DATA + 0x2000;
            clear(c, state, 0x100);
            clear(c, engine, 0x400);
            put_str(c, DATA + 0x3000, "memcard");
            w(c, state + 0x10, engine);
            c.g[A0][0] = sx(state);
        });
    }


    #[test]
    fn cfe_mem_card_update_matches() {
        check(
            ee::s02::f_00186bf0__generated,
            cfe_mem_card_update,
            4,
            &|c, seed| {
                let _ = base_3(c, seed, 0x186bf0, 4);
                c.g[A0][0] = OBJ as u64;
                // A manager that is idle (IsIdle of a zeroed manager) and a
                // base_3 update that returns immediately.
                w(c, GAME_GP.wrapping_sub(0x47c), OBJ2);
                w(c, GAME_GP.wrapping_sub(0x848), WORLD_3);
                w(c, OBJ + 0x218, 1);
                w(c, OBJ + 0x1c0, if seed % 3 == 0 { 6 } else { (seed % 8) as u32 });
                w(c, OBJ + 0x40, SCR);
                w(c, 0x004a_14c0, 0);
                w(c, 0x0045_dc80, 0);
            },
        );
    }


    #[test]
    fn cfe_mem_card_on_scan_done_matches() {
        check(
            ee::s02::f_00187270__generated,
            cfe_mem_card_on_scan_done,
            8,
            &|c, seed| {
                let _ = base_3(c, seed, 0x187270, 4);
                c.g[A0][0] = OBJ as u64;
                w(c, GAME_GP.wrapping_sub(0x47c), OBJ2);
                w(c, OBJ + 0x234, 0);
                w(c, OBJ + 0x238, (seed % 4) as u32);
            },
        );
    }


    #[test]
    fn cfe_state_mem_card_show_not_formatted_msg_matches() {
        check(
            ee::s02::f_00186c98__generated,
            cfe_state_mem_card_show_not_formatted_msg,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x186c, 4);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.bus.write32(RMAN, 0);
                let mc = DATA + 0x2000;
                c.bus.write32(WORLD_4 + 0x8c, mc);
                let obj = DATA + 0x2400;
                c.bus.write32(mc + 4, obj);
                c.bus.write32(obj + 0x20, 0);
                let vt = vtable(c, &mut r, &[0x20]);
                c.bus.write32(obj, vt);
                c.bus.write32(GAME_GP - 0x1c38, DATA + 0x2800);
                c.bus.write32(DATA + 0x2800 + 8, DATA + 0x2c00);
                c.bus.write32(DATA + 0x2c00, vt.wrapping_add(0x20));
            },
        );
    }


    pub(crate) const MEMCARD: u32 = DATA + 0x1_0000;

    pub(crate) const MEMCARD_MP: u32 = DATA + 0x1_1000;

    pub(crate) const MEMCARD_CARD: u32 = DATA + 0x1_2000;


    #[test]
    fn c_fe_state_mem_card_scan_step_matches() {
        check_all(
            ee::s02::f_00187a38__generated,
            c_fe_state_mem_card_scan_step,
            24,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x187a_38, 10);
                c.g[A0][0] = sx(MEMCARD);
                c.bus.write32(0x004a_28a8, WORLD_5);
                c.bus.write32(WORLD_5 + 0xa4, MEMCARD_MP);
                c.bus.write32(MEMCARD_MP + 0x434, MEMCARD_CARD);
                c.bus.write32(MEMCARD_MP + 0x428, (seed % 2) as u32);
                c.bus.write32(0x004a_14bc, seed % 4); // scan step
                c.bus.write32(MEMCARD + 0x1bc, r.next_u32());
                c.bus.write32(MEMCARD + 0x234, r.next_u32() % 4);
                let _ = &mut r;
            },
        );
    }
}
