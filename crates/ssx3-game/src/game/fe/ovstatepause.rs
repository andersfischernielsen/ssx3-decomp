//! `src/fe/ovstatepause.cpp`: hand-written replacements (moved from the
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
use crate::call::Frame;
use crate::call::Callee;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use crate::call::VSlot;
use crate::call::sp;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::X;
use crate::call::Flow;
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::call::Unwound;
use crate::events::deliver_event;
pub(crate) use crate::game::dirtysock::tags2::p_2;
pub(crate) use crate::game::dirtysock::tags2::gp_5 as gp;
pub(crate) use crate::game::dirtysock::tags2::sw;
pub(crate) use crate::game::fe::festateoptions::gpa;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::sw_at;
pub(crate) use crate::game::main::gamemode::low;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::fe::festaterewards::at_4 as at;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw_2;
pub(crate) use crate::game::fe::ovstatemission2::ov_screen_set_two_visible;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_2;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw_3;
pub(crate) use crate::game::fe::feasyncfile::at as at_2;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_3;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_4;
pub(crate) use crate::game::fe::feasyncfile::sw_4 as sw_2;
pub(crate) use crate::game::fe::ovstatemission2::ov_screen_set_widget_visibility;
pub(crate) use crate::game::fe::feasyncfile::lw_7 as lw_5;
pub(crate) use crate::game::fe::festaterewards::gp_4;
pub(crate) use crate::game::fe::fepopup::at as at_3;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_6;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw_3;
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::ai::computer::sd;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x001f_aa78, ov_screen_be_options_on_widget_notify, "OVScreenBEOptions_OnWidgetNotify"),
    (0x001f_aaf0, ov_screen_be_options_v22, "OVScreenBEOptions_v22"),
    (0x001f_ab08, ov_screen_be_options_v21, "OVScreenBEOptions_v21"),
    (0x001f_af10, ovscreen_in_game_error_update, "OVScreenInGameError_Update"),
    (0x001f_aff8, ov_screen_in_game_error_show_cancel_game, "OVScreenInGameError_ShowCancelGame"),
    (0x001f_b180, ov_screen_in_game_error_update_server_timeout_text, "OVScreenInGameError_UpdateServerTimeoutText"),
    (0x001f_b218, ov_screen_in_game_error_update_opponent_disconnected_text, "OVScreenInGameError_UpdateOpponentDisconnectedText"),
    (0x001f_b378, ovscreeningameerror_show_continue_button, "OVScreenInGameError_ShowContinueButton"),
    (0x001f_b458, in_game_error_highlight_continue, "OVScreenInGameError_HighlightContinue"),
    (0x001f_b588, in_game_error_highlight_cancel, "OVScreenInGameError_HighlightCancel"),
    (0x001f_bcc8, ov_screen_race_pre_on_deactivate, "OVScreenRacePre_OnDeactivate"),
    (0x001f_bce0, ov_screen_race_pre_update, "OVScreenRacePre_Update"),
    (0x001f_d0d8, ov_screen_multi_freestyle_pre_on_activate, "OVScreenMultiFreestylePre_OnActivate"),
    (0x001f_d118, ov_multi_freestyle_pre_on_deactivate, "OVScreenMultiFreestylePre_OnDeactivate"),
    (0x001f_d150, ov_screen_multi_freestyle_pre_update, "OVScreenMultiFreestylePre_Update"),
    (0x001f_d268, ov_screen_rival_pre_init, "OVScreenRivalPre_Init"),
    (0x001f_db78, ov_screen_rival_pre_on_widget_notify, "OVScreenRivalPre_OnWidgetNotify"),
    (0x001f_dbf0, ov_screen_save_replay_ctor, "OVScreenSaveReplay_ctor"),
    (0x001f_df00, ov_screen_save_replay_v33, "OVScreenSaveReplay_v33"),
    (0x001f_e5b0, ov_screen_save_replay_v38, "OVScreenSaveReplay_v38"),
    (0x001f_e648, ov_screen_save_replay_v39, "OVScreenSaveReplay_v39"),
    (0x001f_e8f0, ov_screen_save_replay_on_child_event, "OVScreenSaveReplay_OnChildEvent"),
    (0x001f_e968, ov_screen_save_records_ctor, "OVScreenSaveRecords_ctor"),
    (0x001f_ec60, ov_screen_save_records_v33, "OVScreenSaveRecords_v33"),
    (0x001f_f278, ov_screen_save_records_v38, "OVScreenSaveRecords_v38"),
    (0x001f_f3b0, ov_screen_save_records_on_widget_action, "OVScreenSaveRecords_OnWidgetAction"),
    (0x001f_f6d0, ov_screen_reward_list_on_activate, "OVScreenRewardList_OnActivate"),
    (0x001f_f700, ov_screen_reward_list_on_widget_action, "OVScreenRewardList_OnWidgetAction"),
    (0x0023_d570, mc_manager_set_mode, "MCManager_SetMode"),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    use super::*;
    pub(crate) use crate::game::main::ssxappdtor::callee::NET_FE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::main::ssxappdtor::callee::SHARED_LOGIN_SET_ACCOUNT_NAME;
    pub(crate) use crate::game::main::ssxappdtor::callee::SHARED_LOGIN_SET_ACCOUNT_PASSWORD;
    pub(crate) use crate::game::main::ssxappdtor::callee::MESSAGE_CENTER_GET_INBOX;
    pub(crate) use crate::game::main::ssxappdtor::callee::UI_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::main::ssxappdtor::callee::AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::main::ssxappdtor::callee::AUDIO_PLAY_FE_SOUND;
    pub(crate) use crate::game::main::ssxappdtor::callee::AUDIO_ON_EVENT_SELECT_ENTER;
    pub(crate) use crate::game::main::ssxappdtor::callee::BUY_ATTRIB_COMMIT_PURCHASE;
    pub(crate) use crate::game::main::ssxappdtor::callee::FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::main::ssxappdtor::callee::CAREER_STATS_SELECT_PAGE;
    pub(crate) use crate::game::main::ssxappdtor::callee::REWARD_LIST_SHOW_PAGE;
    pub(crate) use crate::game::main::ssxappdtor::callee::EVENT_SELECT_SET_MODE;
    pub(crate) use crate::game::main::ssxappdtor::callee::EVENT_SELECT_SHUTDOWN;
    pub(crate) use crate::game::main::ssxappdtor::callee::OV_STATE_OVERLAY_SHUTDOWN;
    pub(crate) use crate::game::main::ssxappdtor::callee::UI_ENGINE_UPDATE;
    pub(crate) use crate::game::main::ssxappdtor::callee::APP_MODULE_UPDATE;
    pub(crate) use crate::game::main::ssxappdtor::callee::DELETE;
    pub(crate) use crate::game::be::beintmission::callee::FIND_MISSION;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_RESET_STATE;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_SET_ALGORITHM_IMMEDIATE;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_SET_ALGORITHM_2 as CHASE_CAM_SET_ALGORITHM;
    pub(crate) use crate::game::be::beintmission::callee::GET_RIDER_PREVIEW;
    pub(crate) use crate::game::be::beintmission::callee::RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::be::beintmission::callee::NET_FE_STATE_CTOR;
    pub(crate) use crate::game::be::beintmission::callee::LOBBY_MENU_REQUEST_JOIN_ROOM;
    pub(crate) use crate::game::be::beintmission::callee::POPUP_CONFIRM_CFG_RESET;
    pub(crate) use crate::game::be::beintmission::callee::REWARD_GALLERY_CHECK_REWARDS_READY;
    pub(crate) use crate::game::be::beintmission::callee::REWARD_GALLERY_POLL_SHAPE_LOAD;
    pub(crate) use crate::game::be::beintmission::callee::BUDDY_MAN_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::be::beintmission::callee::REPLAY_APPLY_CAMERA;
    pub(crate) use crate::game::be::beintmission::callee::RACE_PROGRESS_RESET_ALL;
    pub(crate) use crate::game::be::beintmission::callee::MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_STOP_MAIN_LISTS;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_STOP_AUX_LISTS;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_DEACTIVATE_BINDING;
    pub(crate) use crate::game::be::beintmission::callee::CROWD_AUDIO_STOP_ALL;
    pub(crate) use crate::game::be::beintmission::callee::BANK_MONITOR_STOP_ALL;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_LEVEL_TO_VOLUME;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::be::beintmission::callee::SND_BANK_SET_MEMORY;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_IS_GAME_MUSIC_ENABLED;
    pub(crate) use crate::game::be::beintmission::callee::MUSIC_PLAYER_STOP_SONG;
    pub(crate) use crate::game::be::beintmission::callee::UI_STATE_UPDATE;
    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::be::beintmission::callee::BE_NEW_PLAYER_GET_INBOX;
    pub(crate) use crate::game::be::beintmission::callee::WSCRIPT_MAN_EVENT_QUEUE_POP;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_START_PRERACE_MUSIC;
    pub(crate) use crate::game::be::beintmission::callee::SLOGIN_DELETE;
    pub(crate) use crate::game::be::beintmission::callee::CAMERA_TRIGGER_MAN_UPDATE_MASK1;
    pub(crate) use crate::game::be::beintmission::callee::EA_MEMSET;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::REWARDS_FILE_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOUNDED_CAMERA_ALGORITHM_BASE_CTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::NET_FE_STATE_CTOR_2 as NET_FE_STATE_CTOR_3;
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
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUILD_BUDDY_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SET_SELECTED_USER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GET_SELECTED_USER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SHOW_BUDDY_ACTION_POPUP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ON_CHILD_EVENT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GOTO_NEXT_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::HANDLE_NET_EVENT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ONLINE_CHARACTER_SELECT_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CHANNEL_GET_TRACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LIST_GET_ENTRY_TYPE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FIND_FREE_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LOAD_BANK_ASYNC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REQUEST_IMAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SEND_PACKET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BANK_MONITOR_RESET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_SAY_20B9;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::STOP_BOARD_LOOPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::START_RIDER_BOARD_LOOPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GET_RACE_MAN;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LEVEL_TO_VOLUME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::AUDIO_SYSTEM_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_END_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_SET_CONTROLLER_INDEX;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_UNLOAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_STATE_CTOR_2_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_CTOR_2 as UI_LIST_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_MENU_SET_SELECTION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_CARD_BASE_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_CARD_ON_CHILD_EVENT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_HAS_RENDER_SLOT;
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
    pub(crate) use crate::game::main::gamemode::callee::EA_MEM_FREE as EA_MEM_FREE_2;
    pub(crate) use crate::game::main::gamemode::callee::BUILTIN_DELETE as BUILTIN_DELETE_3;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::BUILTIN_DELETE_6 as BUILTIN_DELETE_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_FREE_4 as EA_MEM_FREE_3;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_PROFILE_5 as BE_GET_RIDER_PROFILE_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_LEVEL_TO_VOLUME_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_CHANNEL_VOLUME_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::MALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FREE2;
    pub(crate) use crate::game::fe::feasyncfile::callee::VEC3_MAX_COMPONENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_12 as NET_FE_STATE_CTOR_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_DTOR_4 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_3 as NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_SELECTED_USER;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_MANAGER_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST_2 as PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT_2 as PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_FREE_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_PARSE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_MONTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_DAY;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_YEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_GENDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CREATE_STEP2_SET_LEGAL;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARD_GALLERY_INIT_PAGES;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARD_GALLERY_REQUEST_SHAPE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OV_STATE_OVERLAY_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPTIONS_HUD_REFRESH_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_SET_FLAG_2 as MESSAGE_CENTER_SET_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_BIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_WRITE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_NEXT_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_PREPROC_EMPTY_STUB;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_CTOR_COPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_ALLOC_BUFFER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BXSTRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_LANGUAGE_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SET_EVENT_CONTEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRACK_TO_CATEGORY;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_SOMETHING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_NEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_MESSENGER_CLEAR_NAME_LIST;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_CTOR_2_4 as UI_STATE_CTOR_2_3;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_MENU_SET_SELECTION_3 as UI_MENU_SET_SELECTION_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED_4 as OPERATOR_NEW_NAMED_3;
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
    pub(crate) use crate::game::fe::festaterewards::callee::IS_DJ_SPEECH_ON_2;
    pub(crate) use crate::game::fe::festaterewards::callee::IS_ARCADE_SPEECH_ON;
    pub(crate) use crate::game::fe::festaterewards::callee::GET_LANGUAGE_MASK_2;
    pub(crate) use crate::game::fe::festaterewards::callee::SPEECH_STOP_INSTANCE;
    pub(crate) use crate::game::fe::festaterewards::callee::SPEECH_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_POP_FREE_NODE;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_PUSH_FREE_NODE;
    pub(crate) use crate::game::fe::festaterewards::callee::SCRIPT_START_SCRIPT;
    pub(crate) use crate::game::fe::festaterewards::callee::MOD_OBJ_RUN_SCRIPT_EVENT;
    pub(crate) use crate::game::fe::festaterewards::callee::REWARD_GALLERY_REGISTER_TEXTURES;
    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_SERVER_TIMEOUT;
    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_OPPONENT_DISCONNECTED;
    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_SHOW_CANCEL;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_LIBRARY_INSTANCE;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_LIBRARY_INTERFACE;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_REWARD_SET_BIT;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_REWARD_GET_FILE;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_ECON_SPEND_MONEY;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_ECON_GET_MONEY;
    pub(crate) use crate::game::fe::festaterewards::callee::TAG_FIELD_COPY_TAG;
    pub(crate) use crate::game::fe::festaterewards::callee::TAG_FIELD_SET_RAW;
    pub(crate) use crate::game::fe::festaterewards::callee::BRAGGING_RIGHTS_UPDATE_ROW_COLORS;
    pub(crate) use crate::game::fe::festaterewards::callee::BENEWPLAYER_IS_SHARING_CHAR_RECORD;
    pub(crate) use crate::game::fe::festaterewards::callee::BECHAR_GET_OVERALL_RATING;
    pub(crate) use crate::game::fe::festaterewards::callee::MESSAGE_IS_FLAG_SET;
    pub(crate) use crate::game::fe::festaterewards::callee::MESSAGE_SET_FLAG;
    pub(crate) use crate::game::fe::festaterewards::callee::MESSAGE_POST;
    pub(crate) use crate::game::fe::festaterewards::callee::TEXFLIP_SET_FRAME;
    pub(crate) use crate::game::fe::festaterewards::callee::BE_GET_RIDER_PROFILE_2 as BE_GET_RIDER_PROFILE_3;
    pub(crate) use crate::game::fe::festaterewards::callee::BENEWPLAYER_GET_SLOT;
    pub(crate) use crate::game::fe::festaterewards::callee::VFX_PLANTS_GET_SWAY_MATRIX;
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


    pub const MPEG2_SHOW_BITS: Callee = Callee {
        addr: 0x003b_1258,
        f: ee::s13::MPEG2_Show_Bits,
    };

    pub const MPEG2_FLUSH_BUFFER: Callee = Callee {
        addr: 0x003b_1290,
        f: ee::s13::MPEG2_Flush_Buffer,
    };

    pub const AI_DESTROY_MISSION_RIDERS: Callee = Callee {
        addr: 0x0012_af68,
        f: ee::s00::AI_DestroyMissionRiders,
    };

    pub const AI_CREATE_COMPUTER_RIDERS: Callee = Callee {
        addr: 0x0012_9e20,
        f: ee::s00::AI_CreateComputerRiders,
    };

    pub const ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY: Callee = Callee {
        addr: 0x0031_1ae8,
        f: ee::s10::AnimController_GetLayerAnimEntry,
    };

    pub const BE_NEW_PLAYER_INT_GET_SLOT: Callee = Callee {
        addr: 0x0014_7798,
        f: ee::s01::BENewPlayerInt_GetSlot,
    };

    pub const BE_REWARD_INT_COUNT_SET_BITS: Callee = Callee {
        addr: 0x0015_7390,
        f: ee::s01::BERewardInt_CountSetBits,
    };

    pub const CHASE_CAMERA_ALGORITHM_CTOR: Callee = Callee {
        addr: 0x0016_2318,
        f: ee::s01::ChaseCameraAlgorithm_ctor,
    };

    pub const BUILTIN_DELETE: Callee = Callee {
        addr: 0x0031_7e50,
        f: ee::s10::__builtin_delete,
    };

    pub const RIDER_METRIX_CLEAR_PEAK_A4: Callee = Callee {
        addr: 0x0011_76f8,
        f: ee::s00::RiderMetrix_ClearPeakA4,
    };

    pub const EA_MEM_SET: Callee = Callee {
        addr: 0x003e_6448,
        f: ee::s14::EA_MemSet,
    };

    pub const MC_MANAGER_GET: Callee = Callee {
        addr: 0x0022_7f80,
        f: ee::s05::MCManager_Get,
    };

    pub const MC_MANAGER_BEGIN_LOAD: Callee = Callee {
        addr: 0x0023_c8f0,
        f: ee::s06::MCManager_BeginLoad,
    };

    pub const UI_STATE_CTOR: Callee = Callee {
        addr: 0x0039_e2a0,
        f: ee::s13::UIState_ctor,
    };

    pub const FE_FORMAT_MONEY: Callee = Callee {
        addr: 0x0019_8af0,
        f: ee::s03::FE_FormatMoney,
    };

    pub const UI_TEXT_SET_STRING: Callee = Callee {
        addr: 0x003a_0c50,
        f: ee::s13::UIText_SetString,
    };

    pub const FE_MODULE_RIDER_PREVIEW_UNLOAD: Callee = Callee {
        addr: 0x0019_eba0,
        f: ee::s03::FEModule_RiderPreviewUnload,
    };

    pub const FE_MODULE_RIDER_PREVIEW_UPDATE: Callee = Callee {
        addr: 0x0019_f138,
        f: ee::s03::FEModule_RiderPreviewUpdate,
    };

    pub const CHECK_ACCOUNT_NAME: Callee = Callee {
        addr: 0x001a_c360,
        f: ee::s03::cFEUseExistingAccount_CheckAccountName,
    };

    pub const CHECK_PASSWORD: Callee = Callee {
        addr: 0x001a_c2b8,
        f: ee::s03::cFEUseExistingAccount_CheckPassword,
    };

    pub const NET_FE_STATE_ON_CHILD_EVENT: Callee = Callee {
        addr: 0x001a_97b8,
        f: ee::s03::NetFEState_OnChildEvent,
    };

    pub const NET_FE_STATE_CTOR_2: Callee = Callee {
        addr: 0x001a_8500,
        f: ee::s03::NetFEState_ctor,
    };

    pub const KEYBOARD_POPUP_IS_KEY_ENABLED: Callee = Callee {
        addr: 0x001c_e560,
        f: ee::s04::cKeyboardPopup_IsKeyEnabled,
    };

    pub const KEYBOARD_POPUP_MOVE_DOWN: Callee = Callee {
        addr: 0x001c_b9b0,
        f: ee::s04::cKeyboardPopup_MoveDown,
    };

    pub const EVENT_SELECT_POST_INPUT: Callee = Callee {
        addr: 0x0020_2770,
        f: ee::s05::EventSelect_PostInput,
    };

    pub const UI_STATE_POST_INPUT: Callee = Callee {
        addr: 0x0039_e508,
        f: ee::s13::UIState_PostInput,
    };

    pub const STR_HASH: Callee = Callee {
        addr: 0x0031_7670,
        f: ee::s10::StrHash,
    };

    pub const UI_TEXTURE_SET_FIND: Callee = Callee {
        addr: 0x0039_8380,
        f: ee::s13::UITextureSet_Find,
    };

    pub const VID_ENGINE_PLAYER_RENDER: Callee = Callee {
        addr: 0x0025_3890,
        f: ee::s06::VidEnginePlayer_Render,
    };

    pub const UI_STATE_RENDER: Callee = Callee {
        addr: 0x0039_e6b8,
        f: ee::s13::UIState_Render,
    };

    pub const SSX_AUDIO_SYSTEM_GET_INSTANCE: Callee = Callee {
        addr: 0x0028_b180,
        f: ee::s07::SSXAudioSystem_GetInstance,
    };

    pub const MUSIC_PLAYER_RESUME: Callee = Callee {
        addr: 0x002b_3a98,
        f: ee::s08::MusicPlayer_Resume,
    };

    pub const MOUNTAIN_ROOM_POLL_IMAGE_LOAD: Callee = Callee {
        addr: 0x001d_31c0,
        f: ee::s04::cFEStateMountainRoom_PollImageLoad,
    };

    pub const UI_STATE_UPDATE_2: Callee = Callee {
        addr: 0x0039_e510,
        f: ee::s13::UIState_Update,
    };

    pub const C_FE_STATE_ON_ACTIVATE: Callee = Callee {
        addr: 0x0018_6518,
        f: ee::s02::cFEState_OnActivate,
    };

    pub const C_OV_STATE_OVERLAY_UPDATE: Callee = Callee {
        addr: 0x0020_e900,
        f: ee::s05::cOVStateOverlay_Update,
    };

    pub const UI_STATE_ON_ACTIVATE_2: Callee = Callee {
        addr: 0x0039_e4c0,
        f: ee::s13::UIState_OnActivate,
    };

    pub const PLAY_FE_SOUND: Callee = Callee {
        addr: 0x0029_4f78,
        f: ee::s08::SSXAudioSystem_PlayFESound,
    };

    pub const MEM_CARD_STATE_BASE_CTOR_2: Callee = Callee {
        addr: 0x001d_5330,
        f: ee::s04::MemCardStateBase_ctor_2,
    };

    pub const UI_STATE_MGR_CLOSE_TOP_STATE_AND_POPUPS: Callee = Callee {
        addr: 0x0039_f718,
        f: ee::s13::UIStateMgr_CloseTopStateAndPopups,
    };

    pub const UI_STATE_MGR_CLOSE_TOP_STATES: Callee = Callee {
        addr: 0x0039_f190,
        f: ee::s13::UIStateMgr_CloseTopStates,
    };

    pub const OV_SCREEN_SESSION_DESTROY_INDICATORS: Callee = Callee {
        addr: 0x0020_a088,
        f: ee::s05::OVScreenSession_DestroyIndicators,
    };

    pub const C_OV_STATE_OVERLAY_SHUTDOWN: Callee = Callee {
        addr: 0x0020_a430,
        f: ee::s05::cOVStateOverlay_Shutdown,
    };

    pub const FE_ASYNC_MANAGER_CANCEL: Callee = Callee {
        addr: 0x001a_38c0,
        f: ee::s03::FEAsyncManager_Cancel,
    };

    pub const SECTION_MAN_REQUEST_ACTIVATE: Callee = Callee {
        addr: 0x0022_d478,
        f: ee::s05::SectionMan_RequestActivate,
    };

    pub const STRCPY: Callee = Callee {
        addr: 0x0041_66f4,
        f: ee::s15::strcpy,
    };

    pub const STRLEN: Callee = Callee {
        addr: 0x0041_6810,
        f: ee::s15::strlen,
    };
}

/// Addresses the generator emitted as `call_indirect` (outside the shard's
/// own direct-call set, so no readable alias is needed here).
pub(crate) mod addr {
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STR_HASH;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_ENGINE_CREATE_SCREEN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_SCREEN_START_THREAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::CLOSE_UNPINNED_POPUPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MEM_MAN_ALLOC;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCASECMP;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCMP;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRLEN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::SPRINTF;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MEMSET;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_BE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_INTERFACE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::CHASE_CAMERA_RESET_FOR_RIDER;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_POP_PAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_PUSH_PAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::FE_NET_SEND;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_DTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::WORLD_GET;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MC_STATUS_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::RIDER_PREVIEW;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::RIDER_PREVIEW_BOLTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_INTERFACE_9;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::HUD_SHOW_NUMBER_DELAYED;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::SPATIAL_INSERT;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::POSITION_MODIFIER_CTOR_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::WORLD_PAINTER_2D;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_OBJECT_BY_HASH;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BANK_MONITOR_ADD;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_ENGINE_TEARDOWN;
    pub(crate) use crate::game::fe::feasyncfile::addr::__BUILTIN_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::addr::EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::addr::C_BX_STRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::addr::__FIXUNSSFSI;
    pub(crate) use crate::game::fe::feasyncfile::addr::FONT_DTOR;
}

/// `GetHashValue32` (0x317670) and the UI object lookups, which the generator
/// emitted as cross-shard `call_indirect`s.
pub(crate) mod table_callee {
    pub(crate) use crate::game::fe::festaterewards::table_callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::fe::festaterewards::table_callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::fe::festaterewards::table_callee::STRLEN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::festaterewards::table_callee::STRCPY;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_OBJECT_PUSH;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_ADD_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_ENGINE_GET_INTERFACE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_MENU_SET_SELECTED_BY_INDEX;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_FIND;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_GET_STRING4;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BUDDY_PRODUCT_COMPARE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::SSX_AUDIO_GET_RACE_MAN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_REMOVE_SCRIPT_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_SET_SCRIPT_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_SCRIPT_CLEANUP;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_RICH_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::CODEC_DECODE_FRAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET_INTERFACE_FV;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_GET_BASE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BUY_POPUP_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_POSTERS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_TRADING_CARDS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_CONCEPT_ARTS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_VIDEOS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_TOYS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_CHEAT_CHARS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_OPEN_FINISH_SCREEN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::OPERATOR_DELETE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::AI_SET_AI_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_SET_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET_TEXT_FROM_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BENEW_PLAYER_INT_GET_CHAR_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_MODULE_SMALL_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_MODULE_BIG_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::APP_MAN_SET_NEXT_MODULE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UPLOAD_TEXTURE_PIXELS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::MUSIC_SLOT_RELEASE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_STATE_DTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BXSTRING_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BXSTRING_DTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::SECTION_TABLE_CTOR;
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod table {
    pub(crate) use crate::game::fe::feasyncfile::table::STR_HASH;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::table::BXSTRING_ASSIGN;
    pub(crate) use crate::game::fe::feasyncfile::table::BXSTRING_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_MENU_RESET_SCROLL;
    pub(crate) use crate::game::fe::feasyncfile::table::PS2_MEM_CARD_CREATE;
    pub(crate) use crate::game::fe::feasyncfile::table::PATH_STOP_TRACKS;
    pub(crate) use crate::game::fe::feasyncfile::table::EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::table::BUILTIN_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::table::VOIP_RPC_START;
    pub(crate) use crate::game::fe::feasyncfile::table::VOIP_RPC_SET_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::table::AUDIO_GET_RACE_MAN;
    pub(crate) use crate::game::fe::feasyncfile::table::ASYNC_FILE_GET_RESULT;
    pub(crate) use crate::game::fe::feasyncfile::table::SCRIPT_ENGINE_GET_RESULT_VTABLE;
    pub(crate) use crate::game::fe::feasyncfile::table::RIDER_MODEL_FIND;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_NEW_RACE_SET_GAME_MODE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_NEW_RACE_SET_EVENT_TYPE;
    pub(crate) use crate::game::fe::feasyncfile::table::REPLAY_END_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::table::REPLAY_BEGIN_PLAYBACK;
    pub(crate) use crate::game::fe::feasyncfile::table::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::table::OPERATOR_NEW_TAG;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_TEXT_SET_WIDE_STRING;
    pub(crate) use crate::game::fe::feasyncfile::table::SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::table::EAC_GET_UNPACKED_SIZE;
    pub(crate) use crate::game::fe::feasyncfile::table::EAC_UNPACK;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_SPHERE_TREE_TRIANGLE;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_HULL_TRIANGLE;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_HULL_TRIANGLE2;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_SPHERE_CALC;
    pub(crate) use crate::game::fe::feasyncfile::table::STR_CASECMP;
}


// ---------------------------------------------------------------------------
// Memory card manager.

/// `void MCManager_SetMode(MCManager *self, int mode)` (0x23d570): publish the
/// mode at +0xb8; when it is exactly 1, also store the mode at +0x108, 1.0f at
/// +0x110 and a global float (`gp-0x512c`) at +0x114.
pub fn mc_manager_set_mode(c: &mut Cpu) {
    c.cycles += 2;
    let this = Addr(abi::arg(c, 0));
    let mode = c.g[A1][0];
    this.off(0xb8).sw(c, mode);
    if mode != 0 {
        c.cycles += 3;
        c.g[V0][0] = 0x3c;
        if mode == 1 {
            c.cycles += 6;
            c.g[AT][0] = 0x3f80_0000;
            c.f[0] = c.g[AT][0] as u32; // mtc1 $at, $f0 (1.0f)
            c.f[1] = Addr((c.g[GP][0] as u32).wrapping_sub(0x512c)).read_u32(c);
            this.off(0x108).sw(c, c.g[V0][0]);
            this.off(0x110).write_fpu(c, FpuF32(c.f[0]));
            this.off(0x114).write_fpu(c, FpuF32(c.f[1]));
        }
    }
    c.cycles += 2;
    abi::ret(c);
}


/// `void OVScreenRewardList_OnActivate(self)` (0x1ff6d0): activate the base
/// screen state, then show the reward list's first page.
pub fn ov_screen_reward_list_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::UI_STATE_ON_ACTIVATE, 0x001ff6e4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        call_ptr(c, callee::REWARD_LIST_SHOW_PAGE, 0x001ff6f0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void OVScreenMultiFreestylePre_OnDeactivate(OVScreen* self)`
/// (0x1fd118): mark the freestyle screen inactive, then resume the
/// pre-race music.
pub fn ov_multi_freestyle_pre_on_deactivate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x204)).lw(c);
        let none = c.g[V1][0] == 0;
        frame.save_ra(c, 0);
        if !none {
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
            Addr((c.g[V1][0] as u32).wrapping_add(0x64)).sw(c, c.g[V0][0]);
        }
        c.cycles += 7;
        call_ptr(c, callee::AUDIO_GET_INSTANCE, 0x001f_d138)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, callee::AUDIO_START_PRERACE_MUSIC, 0x001f_d140)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `lw` at `base + off` (sign-extended), the generators' most common load.
#[inline(always)]
pub(crate) fn lw(c: &mut Cpu, base: u32, off: u32) -> u64 {
    Addr(base.wrapping_add(off)).lw(c)
}


/// `a + off`.
#[inline(always)]
pub(crate) fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}


// ---------------------------------------------------------------------------
// Overlay screens.

/// The shared tail of `OVScreenRacePre_Update` / `OVScreenMultiFreestylePre_Update`:
/// update the overlay base, then flag the loaded race "finished" once.
pub(crate) fn ov_screen_pre_update_common(c: &mut Cpu, ret: u32) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::C_OV_STATE_OVERLAY_UPDATE.call(c, ret)?;
        let g = crate::abi::r32(c, GP);
        let race = lw(c, g, 0xfffffdfc); // gp - 0x204
        c.g[A0][0] = race;
        let missing = race == 0;
        frame.restore_ra(c, 0);
        if !missing {
            c.cycles += 3;
            c.g[V0][0] = lw(c, race as u32, 0x64);
            let done = c.g[V0][0] != 0;
            c.g[V0][0] = u64::MAX;
            if !done {
                c.cycles += 4;
                c.g[V1][0] = 1;
                Addr(p(race as u32, 0x64)).sw(c, c.g[V0][0]);
                Addr(crate::abi::r32(c, GP).wrapping_sub(0x9f4)).sw(c, c.g[V1][0]);
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}


/// `void OVScreenRacePre_Update(Screen* s)` (0x1fbce0).
pub fn ov_screen_race_pre_update(c: &mut Cpu) {
    ov_screen_pre_update_common(c, 0x001f_bcf0)
}


/// `void OVScreenMultiFreestylePre_Update(Screen* s)` (0x1fd150).
pub fn ov_screen_multi_freestyle_pre_update(c: &mut Cpu) {
    ov_screen_pre_update_common(c, 0x001f_d160)
}


/// `void OVScreenMultiFreestylePre_OnActivate(Screen* s)` (0x1fd0d8):
/// activate the overlay, mark the loaded race with state 0x4b0, and play the
/// front-end "activate" sound (0xe).
pub fn ov_screen_multi_freestyle_pre_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::UI_STATE_ON_ACTIVATE_2.call(c, 0x001f_d0e8)?;
        let g = crate::abi::r32(c, GP);
        let race = lw(c, g, 0xfffffdfc); // gp - 0x204
        c.g[V1][0] = race;
        c.g[V0][0] = 0x4b0;
        if race != 0 {
            c.cycles += 1;
            Addr((race as u32).wrapping_add(0x64)).sw(c, c.g[V0][0]);
        }
        c.cycles += 8;
        callee::SSX_AUDIO_SYSTEM_GET_INSTANCE.call(c, 0x001f_d100)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xe;
        callee::PLAY_FE_SOUND.call(c, 0x001f_d10c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void* OVScreenSaveReplay_ctor(void* self, int a1, int a2)` (0x1fdbf0):
/// memory-card base state (variant 1), class vtable, `+0x22c` cleared.
pub fn ov_screen_save_replay_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = c.g[A2][0];
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 1;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MEM_CARD_STATE_BASE_CTOR_2.call(c, 0x001f_dc0c)?;
        let this = crate::abi::r32(c, S0);
        c.g[V1][0] = 0x47_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx(crate::abi::r32(c, V1).wrapping_add(0x2440));
        Addr(p(this, 0x22c)).sw(c, 0);
        Addr(p(this, 8)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void* OVScreenSaveRecords_ctor(void* self, int a1, int a2)` (0x1fe968):
/// the same with variant 2 and the records vtable.
pub fn ov_screen_save_records_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = c.g[A2][0];
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 2;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::MEM_CARD_STATE_BASE_CTOR_2.call(c, 0x001f_e984)?;
        let this = crate::abi::r32(c, S0);
        c.g[V1][0] = 0x47_0000;
        c.g[V0][0] = c.g[S0][0];
        c.g[V1][0] = sx(crate::abi::r32(c, V1).wrapping_add(0x22b8));
        Addr(p(this, 0x22c)).sw(c, 0);
        Addr(p(this, 8)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// The three "close the top state once" hooks: when `+0x1ac` is clear, close
/// the state at `*(self + 0x10) + 0x18`, then set `+0x1ac`.
pub(crate) fn close_top_state_once(c: &mut Cpu, ret: u32) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = crate::abi::r32(c, S0);
        c.g[V0][0] = lw(c, this, 0x1ac);
        let done = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if !done {
            c.cycles += 4;
            c.g[A0][0] = lw(c, this, 0x10);
            c.g[A0][0] = sx(p(crate::abi::r32(c, A0), 0x18));
            callee::UI_STATE_MGR_CLOSE_TOP_STATE_AND_POPUPS.call(c, ret)?;
            c.g[V0][0] = 1;
        }
        c.cycles += 5;
        frame.restore_ra(c, 0);
        Addr(p(crate::abi::r32(c, S0), 0x1ac)).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}


/// `void OVScreenSaveReplay_v33(Screen* s)` (0x1fdf00).
pub fn ov_screen_save_replay_v33(c: &mut Cpu) {
    close_top_state_once(c, 0x001f_df28)
}


/// `void OVScreenSaveRecords_v33(Screen* s)` (0x1fec60).
pub fn ov_screen_save_records_v33(c: &mut Cpu) {
    close_top_state_once(c, 0x001f_ec88)
}


/// `void OVScreenRewardList_OnWidgetAction(self, widget /*a1*/, action
/// /*a2*/)` (0x1ff700): when the "confirm" key is pressed, arm the reward
/// list and tell rider slot 0.
pub fn ov_screen_reward_list_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        let no_widget = c.g[A1][0] == 0;
        frame.save_ra(c, 0);
        if !no_widget {
            c.cycles += 3;
            c.g[V0][0] = 5;
            let not_confirm = c.g[A2][0] != c.g[V0][0];
            c.g[RA][0] = c.ld(p_2(sp(c), 0));
            if not_confirm {
                c.cycles += 2;
                frame.close(c);
                return Ok(());
            }
            c.cycles += 8;
            c.g[V0][0] = 1;
            sw(c, gp(c, -0x9f4), 0, c.g[V0][0]);
            call_ptr(c, callee::BE_LIBRARY_GET_INSTANCE, 0x001f_f724)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0xd;
            call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x001f_f730)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::BE_MESSAGE_FLAG_SET, 0x001f_f738)?;
        }
        // Both the no-widget path and the confirm path join here.
        c.cycles += 1;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}


/// `int OVScreenBEOptions_v22(void*, int, int id)` (0x1faaf0): id 4 (the
/// "back" widget of the back-end options overlay) is handled (0x100).
pub fn ov_screen_be_options_v22(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A2][0] ^= 0x4;
    c.g[V1][0] = 0x100;
    c.g[V0][0] = 0x101;
    if c.g[A2][0] == 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    abi::ret(c);
}


/// `int OVScreenBEOptions_v21(void*, int, int id)` (0x1fab08): 0x100 for the
/// "no id" case, as the other option screens.
pub fn ov_screen_be_options_v21(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = 0x100;
    c.g[V0][0] = 0x101;
    if c.g[A2][0] == 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    abi::ret(c);
}


/// `void OVScreenRacePre_OnDeactivate(void)` (0x1fbcc8): reset the pending
/// count of the overlay (`gp-0x204 -> +0x64`) to -1, when it exists.
pub fn ov_screen_race_pre_on_deactivate(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = Addr(gpa(c, -0x204)).lw(c);
    let absent = c.g[V1][0] == 0;
    c.g[V0][0] = u64::MAX;
    if absent {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 1;
    c.sw((c.g[V1][0] as u32).wrapping_add(0x64), c.g[V0][0]);
    c.cycles += 2;
    abi::ret(c);
}


/// `int OVScreenRivalPre_OnWidgetNotify(void*, int, int id)` (0x1fdb78):
/// id 6 handled (0x101), everything else 0.
pub fn ov_screen_rival_pre_on_widget_notify(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A2][0] ^= 0x6;
    c.g[V0][0] = 0x101;
    if c.g[A2][0] != 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}


/// `OVScreenSaveReplay_OnChildEvent` (0x1FE8F0): on the "save done" child event
/// of the replay save screen, stop the replay save and clear the pending flag;
/// everything else goes to the memory card state's child-event handler.
pub fn ov_screen_save_replay_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x16;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        let other_key = c.g[S1][0] != c.g[V0][0];
        c.g[S2][0] = c.g[A1][0];
        let mut tail = true;
        if !other_key {
            c.cycles += 4;
            c.g[V1][0] = lw_at(c, S0, 0x19c);
            c.g[V0][0] = 5;
            let not_five = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            if not_five {
                tail = false;
            } else {
                c.cycles += 5;
                c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
                c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x84)).lw(c);
                c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x28)).lw(c);
                callee::REPLAY_END_SAVE.call(c, 0x001f_e938)?;
                sw_at(c, S0, 0x1c8, 0);
            }
        }
        if tail {
            c.cycles += 1;
            c.g[A0][0] = c.g[S0][0];
        }
        c.cycles += 9;
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S1][0];
        callee::MEM_CARD_ON_CHILD_EVENT.call(c, 0x001f_e94c)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int OVScreenBEOptions_OnWidgetNotify(self, menu, key, index)` (0x1faa78):
/// the pause-menu widget notification filter — 0x101 for the keys it does not
/// handle, 0x100 for "start"/"options" and for the pause title.
pub fn ov_screen_be_options_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[SP][0] = sx(low(c, SP).wrapping_sub(0x20));
        c.g[A0][0] = c.g[A1][0];
        c.sq(16, sp_2(c).wrapping_add(0x10));
        c.g[A1][0] = c.g[A3][0];
        c.g[V0][0] = 0x6;
        let title = c.g[A2][0] == c.g[V0][0];
        c.sd(sp_2(c), c.g[RA][0]);
        if title {
            c.cycles += 8;
            c.g[A0][0] = sx(low(c, A0).wrapping_add(0x74));
            call_ptr(c, 0x0039_7870, 0x001f_aac4)?;
            c.g[A0][0] = 0x4a_0000;
            c.g[S0][0] = lw_r(c, V0, 0x38);
            c.g[A0][0] = sx(low(c, A0).wrapping_add(0x24e8));
            call_ptr(c, callee::STR_HASH_2, 0x001f_aad4)?;
            if c.g[S0][0] != c.g[V0][0] {
                c.g[V0][0] = 0x100;
            } else {
                c.cycles += 1;
                c.g[V0][0] = 0x101;
            }
        } else {
            c.cycles += 3;
            if c.g[A2][0] < 0x6 {
                c.g[V0][0] = 0x101;
            } else {
                c.cycles += 3;
                if !(c.g[A2][0] < 0xa) {
                    c.cycles += 1;
                    c.g[V0][0] = 0x101;
                } else if c.g[A2][0] < 0x8 {
                    c.cycles += 2;
                    c.g[V0][0] = 0x101;
                } else {
                    c.cycles += 2;
                    c.cycles += 2;
                    c.g[V0][0] = 0x100;
                }
            }
        }
        c.cycles += 4;
        c.lq(16, sp_2(c).wrapping_add(0x10));
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low(c, RA);
        c.g[SP][0] = sx(low(c, SP).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}


/// `void OVScreenSaveReplay_v38(self, on)` (0x1fe5b0).
pub fn ov_screen_save_replay_v38(c: &mut Cpu) {
    guest(c, |c| ov_screen_set_two_visible(c, 0x001f_e5b0, 0x188))
}


/// `void OVScreenSaveRecords_v38(self, on)` (0x1ff278).
pub fn ov_screen_save_records_v38(c: &mut Cpu) {
    guest(c, |c| ov_screen_set_two_visible(c, 0x001f_f278, 0x188))
}


/// `void OVScreenInGameError_UpdateServerTimeoutText(self)` (0x1fb180): update
/// the timeout text and its object's countdown value.
pub fn ov_screen_in_game_error_update_server_timeout_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_2(c, S1, 0x40);
        if c.g[V0][0] != 0 {
            // The text object (S0) is looked up by hash; the hash of the
            // unloaded string is 0 on the test machine.
            c.cycles += 9;
            c.g[A0][0] = 0x470000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4e8));
            call_ptr(c, table_callee::GET_HASH_VALUE32, 0x001f_b1ac)?;
            c.g[A0][0] = lw_2(c, S1, 0x40);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, table_callee::UI_SCREEN_FIND_OBJECT, 0x001f_b1b8)?;
            c.g[S0][0] = c.g[V0][0];
            if c.g[S0][0] == 0 {
                c.g[V0][0] = lw_2(c, S0, 0x8);
            } else {
                c.cycles += 3;
                c.g[V0][0] = lw_2(c, S1, 0xa0);
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = lw_2(c, S0, 0x8);
                } else {
                    c.cycles += 7;
                    c.g[A0][0] = 0x470000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4a0));
                    call_ptr(c, table_callee::GET_HASH_VALUE32, 0x001f_b1dc)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(
                        c,
                        table_callee::UI_TEXT_SET_UNICODE_STRING_BY_ID,
                        0x001f_b1e8,
                    )?;
                    c.g[V0][0] = lw_2(c, S0, 0x8);
                }
            }
            c.cycles += 6;
            c.g[A1][0] = lw_2(c, S1, 0xa0);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x48).lh(c);
            c.g[V1][0] = lw_2(c, V0, 0x4c);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, 0x001f_b200)?;
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `OVScreenBEOptions_Update` (0x1fa9e0): update the BE options overlay and,
/// when the pause overlay is active, toggle the option thread's value.
pub fn ov_screen_be_options_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[RA][0] = sx(0x001f_a9f4);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::OVERLAY_UPDATE, 0x001f_a9f4)?;
        c.g[V1][0] = Addr(gp_2(c, -0x204)).lw(c);
        let no_overlay = c.g[V1][0] == 0;
        if !no_overlay {
            c.cycles += 3;
            c.g[V0][0] = Addr(p(r(c, V1), 0x68)).lw(c);
            let zero68 = c.g[V0][0] == 0;
            c.g[A0][0] = 0;
            let active;
            if zero68 {
                c.cycles += 1;
                c.g[A0][0] = 1;
                active = true;
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(p(r(c, V1), 0x64)).lw(c);
                active = c.g[V0][0] != 0;
                if !active {
                    c.cycles += 1;
                    c.g[A0][0] = 1;
                }
            }
            let _ = active;
            c.cycles += 2;
            if c.g[A0][0] != 0 {
                c.cycles += 10;
                c.g[A0][0] = 0x0047_0000;
                c.g[RA][0] = sx(0x001f_aa30);
                c.g[A0][0] = sx(0x0047_0000u32.wrapping_sub(0x1fb0));
                call_ptr(c, callee::FE_STRING_HASH, 0x001f_aa30)?;
                c.g[A0][0] = Addr(p(r(c, S0), 0x40)).lw(c);
                c.g[RA][0] = sx(0x001f_aa3c);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, callee::UI_SCREEN_LOOKUP_VALUE, 0x001f_aa3c)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = 0xffff;
                let is_ffff = c.g[A1][0] == c.g[V0][0];
                c.g[A1][0] = c.g[A1][0] & 0xffff;
                if !is_ffff {
                    c.cycles += 3;
                    c.g[A0][0] = Addr(p(r(c, S0), 0x40)).lw(c);
                    c.g[RA][0] = sx(0x001f_aa58);
                    c.g[A2][0] = 1;
                    call_ptr(c, callee::UI_SCREEN_LAUNCH_THREAD, 0x001f_aa58)?;
                }
                c.cycles += 5;
                c.g[A0][0] = Addr(p(r(c, S0), 0x10)).lw(c);
                c.g[A1][0] = 1;
                c.g[RA][0] = sx(0x001f_aa68);
                c.g[A0][0] = sx(r(c, A0).wrapping_add(0x18));
                call_ptr(c, callee::UI_SCREEN_SET_OBJECT_VALUE, 0x001f_aa68)?;
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void OVScreenInGameError_UpdateOpponentDisconnectedText(Screen* self)`
/// (0x1fb218): find the disconnected-text object on the screen, set its text
/// id to the opponent-disconnected string and show the continue button.
pub fn ov_screen_in_game_error_update_opponent_disconnected_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_3(c, p(c.g[S1][0] as u32, 0x40));
        let no_screen = c.g[V0][0] == 0;
        c.g[A0][0] = 0x0047_0000;
        if !no_screen {
            c.cycles += 8;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4e8));
            call_ptr(c, 0x0031_7670, 0x001f_b240)?; // StrHash
            c.g[A0][0] = lw_3(c, p(c.g[S1][0] as u32, 0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001f_b24c)?; // UIScreen_FindObject
            c.g[S0][0] = c.g[V0][0];
            if c.g[S0][0] == 0 {
                c.g[V0][0] = lw_3(c, 8);
            } else {
                c.cycles += 3;
                c.g[V0][0] = lw_3(c, p(c.g[S1][0] as u32, 0xa0));
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = lw_3(c, p(c.g[S0][0] as u32, 8));
                } else {
                    c.cycles += 7;
                    c.g[A0][0] = 0x0047_0000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x478));
                    call_ptr(c, 0x0031_7670, 0x001f_b270)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003a_1000, 0x001f_b27c)?; // UIText_SetTextId
                    c.g[V0][0] = lw_3(c, p(c.g[S0][0] as u32, 8));
                }
            }
            c.cycles += 8;
            c.g[A1][0] = lw_3(c, p(c.g[S1][0] as u32, 0xa0));
            c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 0x48)).lh(c);
            c.g[V1][0] = lw_3(c, p(c.g[V0][0] as u32, 0x4c));
            c.g[V1][0] = sx(c.g[V1][0] as u32);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, 0x001f_b294)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 1;
            call_ptr(c, 0x001f_b378, 0x001f_b2a0)?; // ShowContinueButton
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}


/// `void OVScreenSaveReplay_Shutdown(self)` (0x1fde60): end the replay save if
/// active, pop the screen from the stack and begin playback.
pub fn ov_screen_save_replay_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        c.g[V0][0] = lw_4(c, S0, 0x1c8);
        let not_saving = c.g[V0][0] != c.g[V1][0];
        c.g[A1][0] = Addr(gp_3(c, -0x69c)).lw(c);
        if !not_saving {
            c.cycles += 6;
            c.g[V0][0] = Addr(gp_3(c, -0x848)).lw(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x84).lw(c);
            c.g[A0][0] = lw_4(c, V1, 0x28);
            call_ptr(c, table::REPLAY_END_SAVE, 0x001f_de90)?;
            sw_2(c, S0, 0x1c8, 0);
            c.g[A1][0] = Addr(gp_3(c, -0x69c)).lw(c);
        }
        c.cycles += 26;
        c.g[V0][0] = 0x0053_66e8;
        c.g[A2][0] = 0x0044_28f0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
        c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
        c.g[A0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        Addr(gp_3(c, -0x69c)).sw(c, c.g[A1][0]);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        Addr(gp_3(c, -0x6a0)).sw(c, c.g[V1][0]);
        call_ptr(c, 0x001d_58b8, 0x001f_ded4)?;
        c.g[V1][0] = Addr(gp_3(c, -0x848)).lw(c);
        c.g[A1][0] = 0;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x84).lw(c);
        c.g[A0][0] = lw_4(c, V0, 0x28);
        call_ptr(c, table::REPLAY_BEGIN_PLAYBACK, 0x001f_def0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


pub fn ov_screen_save_replay_v39(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        ov_screen_set_widget_visibility(c, c.g[A2][0])
    })
}


// ---------------------------------------------------------------------------
// FE overlay: rival pre-screen.

/// `void OVScreenRivalPre_Init(OVScreenRivalPre* s)` (0x1fd268): create the
/// overlay's two screens, play them, initialise the overlay base and start the
/// rival-entry speech.
pub fn ov_screen_rival_pre_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        let this = r(c, A0);
        c.g[S1][0] = sx(this);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = sx(0x0046_ff58);
        c.g[RA][0] = sx(0x001f_d28c);
        c.g[S0][0] = Addr(p(this, 0x10)).lw(c);
        call_ptr(c, callee::FE_STRING_HASH, 0x001f_d28c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[RA][0] = sx(0x001f_d2a0);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001f_d2a0)?;
        let none = c.g[V0][0] == 0;
        Addr(p(r(c, S1), 0x40)).sw(c, c.g[V0][0]);
        if !none {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x001f_d2b8);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001f_d2b8)?;
        }
        'second: {
            c.cycles += 11;
            c.g[A0][0] = sx(0x0047_0000);
            c.g[S0][0] = Addr(p(r(c, S1), 0x10)).lw(c);
            c.g[RA][0] = sx(0x001f_d2c8);
            c.g[A0][0] = sx(0x0046_e378);
            call_ptr(c, callee::FE_STRING_HASH, 0x001f_d2c8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x001f_d2dc);
            c.g[A3][0] = 0;
            call_ptr(c, 0x0039_7cc0, 0x001f_d2dc)?;
            let none = c.g[V0][0] == 0;
            c.g[A0][0] = c.g[V0][0];
            if none {
                break 'second;
            }
            c.cycles += 3;
            c.g[A1][0] = 0;
            c.g[RA][0] = sx(0x001f_d2f0);
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001f_d2f0)?;
        }
        c.cycles += 12;
        c.g[RA][0] = sx(0x001f_d2f8);
        c.g[A0][0] = c.g[S1][0];
        callee::OV_STATE_OVERLAY_INIT.call(c, 0x001f_d2f8)?;
        c.g[RA][0] = sx(0x001f_d300);
        call_ptr(c, 0x0028_b180, 0x001f_d300)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x001f_d30c);
        c.g[A1][0] = 0xe;
        call_ptr(c, 0x0029_4f78, 0x001f_d30c)?;
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// `OVScreenInGameError_OnWidgetAction` (0x001fb0c0).
//
// `int OVScreenInGameError::OnWidgetAction(self, UIWidget* w, int action)`:
// map the dialog buttons onto the highlight/select handlers.

pub fn ov_screen_in_game_error_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {

        'b_1fb16c: {
        'b_1fb168: {
        c.cycles += 8;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S1][0] = c.g[A1][0];
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S2][0] = c.g[A0][0];
        let t = c.g[S1][0] == 0u64;
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        if t {
        break 'b_1fb168;
        } else {
        'b_1fb104: {
        c.cycles += 3;
        c.g[V0][0] = 0x1;
        let t = c.g[A2][0] == c.g[V0][0];
        c.g[V0][0] = 0x5;
        if t {
        break 'b_1fb104;
        } else {
        c.cycles += 2;
        let t = c.g[A2][0] != c.g[V0][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        if t {
        break 'b_1fb16c;
        } else {
        c.cycles += 4;
        c.g[RA][0] = sx(0x001fb0fc);
        c.g[A2][0] = 0x5;
        call_ptr(c, 0x001fb2b8, 0x001fb0fc)?;
        let t = true;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        if t {
        break 'b_1fb16c;
        } else {
        break 'b_1fb104;
        }
        }
        }
        }
        'b_1fb13c: {
        c.cycles += 6;
        c.g[A0][0] = 0x470000;
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
        c.g[RA][0] = sx(0x001fb114);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x570));
        call_ptr(c, 0x00317670, 0x001fb114)?;
        let t = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = 0x470000;
        if t {
        break 'b_1fb13c;
        } else {
        c.cycles += 8;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x001fb128);
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x001fb588, 0x001fb128)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x001fb134);
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x001fb458, 0x001fb134)?;
        let t = true;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        if t {
        break 'b_1fb16c;
        } else {
        break 'b_1fb13c;
        }
        }
        }
        c.cycles += 5;
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
        c.g[RA][0] = sx(0x001fb148);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4b8));
        call_ptr(c, 0x00317670, 0x001fb148)?;
        let t = c.g[S0][0] != c.g[V0][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        if t {
        break 'b_1fb16c;
        } else {
        c.cycles += 6;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x001fb15c);
        c.g[A1][0] = 0x1;
        call_ptr(c, 0x001fb458, 0x001fb15c)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x001fb168);
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x001fb588, 0x001fb168)?;
        break 'b_1fb168;
        }
        }
        }
        c.cycles += 1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        break 'b_1fb16c;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t; return Ok(());

        Ok(())
    })
}


/// `OVScreenInGameError_ShowCancelGame(this, on)` (0x1faff8): show or hide
/// the "cancel game" prompt (and its second, menu-driven variant).
pub fn ov_screen_in_game_error_show_cancel_game(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let this = abi::arg(c, 0);
        let on = abi::arg(c, 1);
        let frame = Frame::open(c, 0x40);
        frame.save(c, 18, 0x10);
        c.g[S2][0] = abi::arg64(c, 0);
        frame.save(c, 17, 0x20);
        c.g[A0][0] = 0x0047_0000;
        c.g[S1][0] = abi::arg64(c, 1);
        frame.save(c, 16, 0x30);
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(0x0047_0000u32.wrapping_sub(0x570));
        callee::STR_HASH.call(c, 0x001f_b020)?;
        c.g[A0][0] = Addr(this).off(0x40).lw(c);
        c.g[A1][0] = c.g[V0][0];
        callee::UI_SCREEN_FIND_OBJECT.call(c, 0x001f_b02c)?;
        let obj = abi::r32(c, V0);
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[V0][0] = Addr(obj).off(8).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x48).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x4c).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001f_b048)?;

        c.g[V0][0] = Addr(obj).off(8).lw(c);
        c.g[A1][0] = c.g[S1][0] ^ 1;
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x40).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x44).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(obj.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001f_b060)?;

        let off = c.g[S1][0] == 0;
        c.g[A0][0] = 0x004a_0000;
        if !off {
            c.cycles += 15;
            c.g[A0][0] = sx(0x004a_0000u32.wrapping_add(0x2470));
            callee::STR_HASH.call(c, 0x001f_b070)?;
            c.g[A0][0] = Addr(this).off(0x40).lw(c);
            c.g[A1][0] = c.g[V0][0];
            callee::UI_SCREEN_FIND_OBJECT.call(c, 0x001f_b07c)?;
            let menu = abi::r32(c, V0);
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = 1;
            c.g[V0][0] = Addr(menu).off(8).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x38).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x3c).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx(menu.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001f_b098)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 1;
            callee::UI_MENU_SET_SELECTION_2.call(c, 0x001f_b0a4)?;
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


/// `void OVScreenInGameError_ShowContinueButton(self, int visible)`
/// (0x1fb378): find the error screen's buttons and show/hide the continue
/// button.
pub fn ovscreeningameerror_show_continue_button(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        c.g[A0][0] = 0x470000;
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0x4b8));
        call_ptr(c, 0x0031_7670, 0x001f_b3a0)?;
        c.g[A0][0] = Addr(p(r32(c, S2), 0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001f_b3ac)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 21;
            c.g[A0][0] = 0x470000;
            c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0x450));
            call_ptr(c, 0x0031_7670, 0x001f_b3c4)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x003a_1000, 0x001f_b3d0)?;
            c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
            c.g[A1][0] = c.g[S1][0] ^ 1;
            c.g[A0][0] = Addr(p(r32(c, V0), 0x40)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0x44)).lw(c);
            let f = r32(c, V1);
            c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
            call_ptr(c, f, 0x001f_b3e8)?;
            c.g[V0][0] = Addr(p(r32(c, S0), 8)).lw(c);
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = Addr(p(r32(c, V0), 0x48)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0x4c)).lw(c);
            let f = r32(c, V1);
            c.g[A0][0] = sx(p(r32(c, S0), r32(c, A0)));
            call_ptr(c, f, 0x001f_b400)?;
            c.g[V0][0] = 4;
            Addr(p(r32(c, S0), 0x18)).sw(c, c.g[V0][0]);
        }
        c.cycles += 8;
        c.g[A0][0] = 0x4a0000;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x2470));
        call_ptr(c, 0x0031_7670, 0x001f_b414)?;
        c.g[A0][0] = Addr(p(r32(c, S2), 0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_d860, 0x001f_b420)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0 {
            c.lq(S0, frame.at(c, 0x30));
        } else {
            c.cycles += 2;
            let hide = c.g[S1][0] == 0;
            c.g[A0][0] = c.g[S0][0];
            if !hide {
                c.cycles += 2;
                c.g[A1][0] = 0;
                call_ptr(c, 0x0039_b960, 0x001f_b43c)?;
            }
            c.cycles += 1;
            c.lq(S0, frame.at(c, 0x30));
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void OVScreenInGameError_Update(OVScreenInGameError* self)` (0x1faf10):
/// run the overlay's common update, then, while the online error is armed,
/// check the network timeout / disconnect / ping state and either show the
/// timeout or disconnect message, or hide the "ping" object; finally refresh
/// the cancel-game button and clear the armed flag.
pub fn ovscreen_in_game_error_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0020_e900, 0x001f_af24)?;

        c.g[V1][0] = Addr(gp_4(c, -0x204)).lw(c);
        if c.g[V1][0] == 0 {
            frame.restore(c, 16, 0x10);
            return game_error_tail(c, frame);
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(r(c, S0)).off(0x9c).lw(c);
        if c.g[V0][0] == 0 {
            frame.restore(c, 16, 0x10);
            return game_error_tail(c, frame);
        }

        // a0 = (p->+0x138 <= 0 && p->+0x13c == 0): the ping timed out.
        c.cycles += 3;
        c.g[V0][0] = Addr(r(c, V1)).off(0x138).lw(c);
        let positive = (c.g[V0][0] as i64) > 0;
        c.g[A0][0] = 0;
        if !positive {
            c.cycles += 2;
            c.g[V0][0] = Addr(r(c, V1)).off(0x13c).lw(c);
            c.g[A0][0] = (c.g[V0][0] < 1) as u64;
        }

        'outer: {
            'reply: {
                c.cycles += 2;
                if c.g[A0][0] == 0 {
                    c.g[V0][0] = Addr(r(c, V1)).off(0x13c).lw(c);
                    break 'reply;
                }
                c.cycles += 5;
                c.sw(Addr(r(c, V1)).off(0x9c).0, 0);
                c.g[A0][0] = c.g[S0][0];
                callee::GAME_ERROR_SERVER_TIMEOUT.call(c, 0x001f_af64)?;
                c.g[A0][0] = c.g[S0][0];
                break 'outer;
            }

            c.cycles += 2;
            if c.g[V0][0] == 0 {
                frame.restore(c, 16, 0x10);
                return game_error_tail(c, frame);
            }

            // a0 = (p->+0x80 != 0 && p->+0xa0 == 0 && p->+0x24 == 0).
            'received: {
                c.cycles += 3;
                c.g[V0][0] = Addr(r(c, V1)).off(0x80).lw(c);
                let none = c.g[V0][0] == 0;
                c.g[A0][0] = 0;
                if none {
                    break 'received;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(r(c, V1)).off(0xa0).lw(c);
                let busy = c.g[V0][0] != 0;
                if busy {
                    break 'received;
                }
                c.cycles += 2;
                c.g[V0][0] = Addr(r(c, V1)).off(0x24).lw(c);
                c.g[A0][0] = (c.g[V0][0] < 1) as u64;
            }

            'message: {
                c.cycles += 2;
                let not_received = c.g[A0][0] == 0;
                c.g[A0][0] = 0x470000;
                if not_received {
                    break 'message;
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                callee::GAME_ERROR_OPPONENT_DISCONNECTED.call(c, 0x001f_afa4)?;
                c.g[A0][0] = c.g[S0][0];
                break 'outer;
            }

            c.cycles += 12;
            c.g[A0][0] = sx(r(c, A0).wrapping_sub(0x4e8));
            call_ptr(c, 0x0031_7670, 0x001f_afb4)?;
            c.g[A0][0] = Addr(r(c, S0)).off(0x40).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001f_afc0)?;
            c.g[V1][0] = Addr(r(c, V0)).off(0x8).lw(c);
            c.g[A1][0] = 0;
            c.g[A0][0] = Addr(r(c, V1)).off(0x48).lh(c);
            c.g[A2][0] = Addr(r(c, V1)).off(0x4c).lw(c);
            let target = r(c, A2);
            c.g[A0][0] = sx(p(r(c, V0), r(c, A0)));
            call_ptr(c, target, 0x001f_afd8)?;
            c.g[A0][0] = c.g[S0][0];
        }

        c.cycles += 4;
        c.g[A1][0] = 1;
        callee::GAME_ERROR_SHOW_CANCEL.call(c, 0x001f_afe4)?;
        c.sw(Addr(r(c, S0)).off(0x9c).0, 0);
        frame.restore(c, 16, 0x10);
        game_error_tail(c, frame)
    })
}


/// The `ld ra; addiu sp; jr ra` tail.
pub(crate) fn game_error_tail(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}


/// The two in-game error highlight handlers (0x1fb458 / 0x1fb588): look up
/// the two popup buttons and the message node; with `on` set the highlighted
/// button gets four 1.0 floats, otherwise both buttons are cleared and the
/// node's setter is called.
pub(crate) fn in_game_error_highlight(c: &mut Cpu, names: [u32; 3], rets: [u32; 10]) {
    guest(c, |c| {
        c.cycles += 30;
        let f = Frame::open(c, 0x60);
        f.save(c, S0, 0x50);
        c.g[S0][0] = c.g[A0][0];
        f.save(c, S2, 0x30);
        c.g[A0][0] = 0x0047_0000;
        f.save(c, S1, 0x40);
        f.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        f.save_ra(c, 0x10);
        // Three name -> widget lookups on the screen.
        let hash_ret = [rets[0], rets[2], rets[4]];
        let find_ret = [rets[1], rets[3], rets[5]];
        for (i, name) in names.iter().enumerate() {
            c.g[RA][0] = sx(hash_ret[i]);
            c.g[A0][0] = sx(*name);
            call_ptr(c, 0x0031_7670, hash_ret[i])?;
            c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x40)).lw(c);
            c.g[RA][0] = sx(find_ret[i]);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, find_ret[i])?;
            match i {
                0 => c.g[S1][0] = c.g[V0][0],
                1 => c.g[S3][0] = c.g[V0][0],
                _ => c.g[S0][0] = c.g[V0][0],
            }
        }
        // The button: all four floats 1.0 when highlighted, only the first
        // (alpha) otherwise. The four words go through the stack.
        let on = c.g[S2][0] != 0;
        c.cycles += if on { 14 } else { 13 };
        c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(8)).lw(c);
        c.g[A1][0] = sx(f.at(c, 0));
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 0);
        c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x58)).lh(c);
        c.swc1(0, f.at(c, if on { 0xc } else { 0 }));
        if on {
            c.swc1(0, f.at(c, 0));
            c.swc1(0, f.at(c, 4));
            c.swc1(0, f.at(c, 8));
        } else {
            c.sw(f.at(c, 4), 0);
            c.sw(f.at(c, 8), 0);
            c.sw(f.at(c, 0xc), 0);
        }
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x5c)).lw(c);
        let func = c.g[V0][0] as u32;
        let ret = if on { rets[6] } else { rets[7] };
        c.g[RA][0] = sx(ret);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, func, ret)?;
        c.g[V0][0] = Addr((c.g[S3][0] as u32).wrapping_add(8)).lw(c);
        // Then the node's setter, and the node widget's own setter.
        c.cycles += 18;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
        let func = c.g[V1][0] as u32;
        c.g[RA][0] = sx(rets[8]);
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, func, rets[8])?;
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(8)).lw(c);
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
        let func = c.g[V1][0] as u32;
        c.g[RA][0] = sx(rets[9]);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, func, rets[9])?;
        f.restore(c, S0, 0x50);
        f.restore(c, S1, 0x40);
        f.restore(c, S2, 0x30);
        f.restore(c, S3, 0x20);
        f.restore_ra(c, 0x10);
        f.close(c);
        Ok(())
    })
}


/// `void OVScreenInGameError_HighlightContinue(OVScreen*, int on)` (0x1fb458).
pub fn in_game_error_highlight_continue(c: &mut Cpu) {
    in_game_error_highlight(
        c,
        [0x0046_fb48, 0x0046_fad8, 0x0046_fb00],
        [
            0x001f_b484, 0x001f_b490, 0x001f_b4a0, 0x001f_b4ac, 0x001f_b4bc, 0x001f_b4c8, 0x001f_b500,
            0x001f_b538, 0x001f_b550, 0x001f_b568,
        ],
    );
}


/// `void OVScreenInGameError_HighlightCancel(OVScreen*, int on)` (0x1fb588).
pub fn in_game_error_highlight_cancel(c: &mut Cpu) {
    in_game_error_highlight(
        c,
        [0x0046_fa90, 0x0046_fac8, 0x0046_fae8],
        [
            0x001f_b5b4, 0x001f_b5c0, 0x001f_b5d0, 0x001f_b5dc, 0x001f_b5ec, 0x001f_b5f8, 0x001f_b630,
            0x001f_b668, 0x001f_b680, 0x001f_b698,
        ],
    );
}


/// `void func_001FF3B0(void* self, void* msg, int type)` (0x1ff3b0): the save
/// records screen's widget action: type 5 stores the selected slot from the
/// message and starts the "save" movie sequence, type 6 cancels the movie
/// when the screen is in the right state, and type 7 does the same as 5 once
/// the memory card is ready, then starts the sequence.
pub fn ov_screen_save_records_on_widget_action(c: &mut Cpu) {
    let frame = Frame::open(c, 0x50);
    'b_1ff578: {
        'b_1ff574: {
            c.cycles += 9;
            frame.save(c, S0, 0x40);
            frame.save(c, S2, 0x20);
            c.g[S0][0] = c.g[A0][0];
            frame.save(c, S1, 0x30);
            c.g[S2][0] = c.g[A1][0];
            frame.save(c, S3, 0x10);
            let t = c.g[S2][0] == 0u64;
            sd(c, r32(c, SP), c.g[RA][0]);
            if t {
                break 'b_1ff574;
            } else {
                c.cycles += 3;
                c.g[V0][0] = lw_6(c, at_3(c, S0, 0x1a8));
                if c.g[V0][0] != 0u64 {
                    frame.restore(c, S0, 0x40);
                    break 'b_1ff578;
                } else {
                    'b_1ff4d4: {
                        'b_1ff46c: {
                            c.cycles += 3;
                            c.g[S1][0] = 0x6;
                            let t = c.g[A2][0] == c.g[S1][0];
                            c.g[V0][0] = (c.g[A2][0] < 0x7u64) as u64;
                            if t {
                                break 'b_1ff46c;
                            } else {
                                'b_1ff418: {
                                    'b_1ff404: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = 0x5;
                                        if t {
                                            break 'b_1ff404;
                                        } else {
                                            c.cycles += 2;
                                            if c.g[A2][0] == c.g[V0][0] {
                                                c.g[V0][0] = lw_6(c, at_3(c, S0, 0x1c0));
                                                break 'b_1ff418;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                frame.restore(c, S0, 0x40);
                                                if t {
                                                    break 'b_1ff578;
                                                } else {
                                                    break 'b_1ff404;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x7;
                                    if c.g[A2][0] == c.g[V0][0] {
                                        c.g[V0][0] = lw_6(c, at_3(c, S0, 0x1c0));
                                        break 'b_1ff4d4;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        frame.restore(c, S0, 0x40);
                                        if t {
                                            break 'b_1ff578;
                                        } else {
                                            break 'b_1ff418;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[V0][0] == c.g[S1][0];
                                c.g[S3][0] = 0x1;
                                if t {
                                    break 'b_1ff574;
                                } else {
                                    c.cycles += 7;
                                    c.g[RA][0] = sx(0x001ff428);
                                    c.g[A0][0] = lw_6(c, r32(c, GP).wrapping_sub(0x848));
                                    if callee::C00227f80.call(c, 0x001ff428).is_err() {
                                        return;
                                    }
                                    c.g[V1][0] = c.g[V0][0];
                                    c.g[S1][0] = lw_6(c, at_3(c, S0, 0x214));
                                    c.g[V0][0] = 0x3;
                                    if c.g[S1][0] != c.g[V0][0] {
                                        frame.restore(c, S0, 0x40);
                                        break 'b_1ff578;
                                    } else {
                                        c.cycles += 12;
                                        c.g[V0][0] = lw_6(c, at_3(c, S2, 0x18));
                                        c.g[A0][0] = c.g[V1][0];
                                        c.g[A1][0] = 0x2;
                                        c.g[A2][0] = 0xffffffffffffffff;
                                        sw_3(c, at_3(c, S0, 0x1c4), c.g[V0][0]);
                                        sw_3(c, at_3(c, V1, 0xf8), c.g[V0][0]);
                                        c.g[RA][0] = sx(0x001ff45c);
                                        sw_3(c, at_3(c, S0, 0x1a8), c.g[S3][0]);
                                        if call_ptr(c, 0x0023cae8, 0x001ff45c).is_err() {
                                            return;
                                        }
                                        sw_3(c, at_3(c, S0, 0x1c0), c.g[S1][0]);
                                        sw_3(c, at_3(c, S0, 0x1a8), c.g[S3][0]);
                                        let t = true;
                                        sw_3(c, at_3(c, S0, 0x1dc), c.g[S3][0]);
                                        if t {
                                            break 'b_1ff574;
                                        } else {
                                            break 'b_1ff46c;
                                        }
                                    }
                                }
                            }
                        }
                        c.cycles += 7;
                        c.g[RA][0] = sx(0x001ff474);
                        c.g[A0][0] = lw_6(c, r32(c, GP).wrapping_sub(0x848));
                        if callee::C00227f80.call(c, 0x001ff474).is_err() {
                            return;
                        }
                        c.g[A2][0] = c.g[V0][0];
                        c.g[A0][0] = 0x1;
                        c.g[V0][0] = lw_6(c, at_3(c, S0, 0x1a8));
                        if c.g[V0][0] == c.g[A0][0] {
                            frame.restore(c, S0, 0x40);
                            break 'b_1ff578;
                        } else {
                            c.cycles += 3;
                            c.g[V1][0] = lw_6(c, at_3(c, S0, 0x1c0));
                            if c.g[V1][0] == c.g[S1][0] {
                                frame.restore(c, S0, 0x40);
                                break 'b_1ff578;
                            } else {
                                'b_1ff4cc: {
                                    'b_1ff4b4: {
                                        'b_1ff4b0: {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] == 0u64;
                                            sw_3(c, at_3(c, S0, 0x1e4), c.g[A0][0]);
                                            if t {
                                                break 'b_1ff4b0;
                                            } else {
                                                c.cycles += 3;
                                                c.g[V0][0] = 0x2;
                                                if c.g[V1][0] == c.g[V0][0] {
                                                    c.g[V0][0] = lw_6(c, at_3(c, A2, 0x748));
                                                    break 'b_1ff4b4;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[V1][0] != c.g[A0][0];
                                                    c.g[V0][0] = 0x6;
                                                    if t {
                                                        break 'b_1ff4cc;
                                                    } else {
                                                        break 'b_1ff4b0;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[V0][0] = lw_6(c, at_3(c, A2, 0x748));
                                        break 'b_1ff4b4;
                                    }
                                    c.cycles += 6;
                                    c.g[A1][0] = 0u64;
                                    c.g[A0][0] = lh(c, at_3(c, V0, 0x8));
                                    c.g[V1][0] = lw_6(c, at_3(c, V0, 0xc));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001ff4c8);
                                    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                                    if call_ptr(c, t, 0x001ff4c8).is_err() {
                                        return;
                                    }
                                    c.g[V0][0] = 0x6;
                                    break 'b_1ff4cc;
                                }
                                c.cycles += 2;
                                let t = true;
                                sw_3(c, at_3(c, S0, 0x1c0), c.g[V0][0]);
                                if t {
                                    break 'b_1ff574;
                                } else {
                                    break 'b_1ff4d4;
                                }
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[V0][0] == c.g[S1][0];
                    c.g[S3][0] = 0x1;
                    if t {
                        break 'b_1ff574;
                    } else {
                        c.cycles += 12;
                        c.g[RA][0] = sx(0x001ff4e4);
                        c.g[A0][0] = lw_6(c, r32(c, GP).wrapping_sub(0x848));
                        if callee::C00227f80.call(c, 0x001ff4e4).is_err() {
                            return;
                        }
                        c.g[S1][0] = c.g[V0][0];
                        c.g[A2][0] = lw_6(c, at_3(c, S1, 0x434));
                        c.g[A1][0] = lw_6(c, at_3(c, S1, 0x428));
                        c.g[V1][0] = lw_6(c, r32(c, A2));
                        c.g[A0][0] = lh(c, at_3(c, V1, 0x1b0));
                        c.g[V0][0] = lw_6(c, at_3(c, V1, 0x1b4));
                        let t = c.g[V0][0] as u32;
                        c.g[RA][0] = sx(0x001ff504);
                        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                        if call_ptr(c, t, 0x001ff504).is_err() {
                            return;
                        }
                        if c.g[V0][0] == 0u64 {
                            frame.restore(c, S0, 0x40);
                            break 'b_1ff578;
                        } else {
                            c.cycles += 6;
                            c.g[V0][0] = lw_6(c, at_3(c, S2, 0x18));
                            sw_3(c, at_3(c, S0, 0x1c4), c.g[V0][0]);
                            sw_3(c, at_3(c, S1, 0xf8), c.g[V0][0]);
                            c.g[S2][0] = lw_6(c, at_3(c, S0, 0x1c0));
                            if c.g[S2][0] != c.g[S3][0] {
                                frame.restore(c, S0, 0x40);
                                break 'b_1ff578;
                            } else {
                                c.cycles += 9;
                                c.g[A2][0] = lw_6(c, at_3(c, S1, 0x434));
                                c.g[A1][0] = lw_6(c, at_3(c, S0, 0x1c4));
                                c.g[V1][0] = lw_6(c, r32(c, A2));
                                c.g[A0][0] = lh(c, at_3(c, V1, 0x218));
                                c.g[V0][0] = lw_6(c, at_3(c, V1, 0x21c));
                                let t = c.g[V0][0] as u32;
                                c.g[RA][0] = sx(0x001ff540);
                                c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
                                if call_ptr(c, t, 0x001ff540).is_err() {
                                    return;
                                }
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = 0x4;
                                if t {
                                    break 'b_1ff574;
                                } else {
                                    c.cycles += 11;
                                    sw_3(c, at_3(c, S0, 0x1a8), c.g[S2][0]);
                                    c.g[A0][0] = c.g[S1][0];
                                    sw_3(c, at_3(c, S0, 0x1c0), c.g[V0][0]);
                                    c.g[RA][0] = sx(0x001ff55c);
                                    c.g[A1][0] = 0u64;
                                    if call_ptr(c, 0x0023d570, 0x001ff55c).is_err() {
                                        return;
                                    }
                                    c.g[V0][0] = lw_6(c, at_3(c, S1, 0x748));
                                    c.g[A1][0] = 0x18;
                                    c.g[A0][0] = lh(c, at_3(c, V0, 0x8));
                                    c.g[V1][0] = lw_6(c, at_3(c, V0, 0xc));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001ff574);
                                    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                                    if call_ptr(c, t, 0x001ff574).is_err() {
                                        return;
                                    }
                                    break 'b_1ff574;
                                }
                            }
                        }
                    }
                }
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x40);
        break 'b_1ff578;
    }
    c.cycles += 6;
    frame.restore(c, S1, 0x30);
    frame.restore(c, S2, 0x20);
    frame.restore(c, S3, 0x10);
    c.g[RA][0] = ld(c, r32(c, SP));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_3(c, SP, 0x50));
    c.pc = t;
    return;
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_RESULTS;
    use ssx3_ee as ee;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fill;
    use crate::difftest::put_words;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::covered;
    use crate::difftest::check_lohi;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::SP;
    use crate::difftest::elf_image;
    use crate::difftest::SP as STACK;
    use crate::difftest::fakes;
    use crate::difftest::put_vec;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::seed_count;
    use crate::difftest::compare;
    use ssx3_ee_rt::funcs::GuestFn;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_2 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::clear;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_2 as be_library;
    pub(crate) use crate::game::fe::festatecharequipdetail::tests::world;
    pub(crate) use crate::game::dirtysock::tags2::tests::be_library_2;
    pub(crate) use crate::game::fe::festateoptions::tests::AUX;
    pub(crate) use crate::game::fe::festateoptions::tests::rng;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::put;
    pub(crate) use crate::game::fe::festateoptions::tests::setup;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::gpf;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w;
    pub(crate) use crate::game::dirtysock::tags2::tests::world_2;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_3;
    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_3;
    pub(crate) use crate::game::fe::festatebonusmaterial::tests::run;
    pub(crate) use crate::game::fe::festatebonusmaterial::tests::str_hash;
    pub(crate) use crate::game::fe::festaterewards::tests::w16;
    pub(crate) use crate::game::fe::festaterewards::tests::slot_2 as slot;
    pub(crate) use crate::game::fe::festaterewards::tests::globals_3;
    pub(crate) use crate::game::fe::festaterewards::tests::pool_4 as pool;
    pub(crate) use crate::game::fe::festaterewards::tests::heap_init_3 as heap_init;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4 as base;
    pub(crate) use crate::game::fe::ovstatemission2::tests::hash_object;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::BE_3 as BE;
    pub(crate) use crate::game::wscript::wscriptman::tests::WORLD_3 as WORLD_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::GAME;
    pub(crate) use crate::game::wscript::wscriptman::tests::globals_3 as globals_4;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_2;
    pub(crate) use crate::game::fe::ovstatemission::tests::str_hash as str_hash_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_9 as globals_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_7 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_8 as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_6 as W;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_11 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::eafile_pools;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_9 as run_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_11 as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_11 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_4 as alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_9 as BE_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_7 as WORLD_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::RMAN;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDERS_2 as RIDERS;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDER;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::fake_small;
    pub(crate) use crate::game::sound::soundsys::tests::BE as BE_4;
    pub(crate) use crate::game::sound::soundsys::tests::WORLD as WORLD_4;
    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear_3;
    pub(crate) use crate::game::sound::soundsys::tests::globals_7 as globals_6;
    pub(crate) use crate::game::sound::soundsys::tests::load_2 as load;
    pub(crate) use crate::game::sound::soundsys::tests::alloc_ready;
    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_5;
    pub(crate) use crate::game::fe::festaterewards::tests::BE_7 as BE_5;
    pub(crate) use crate::game::fe::festaterewards::tests::HOOKS;
    pub(crate) use crate::game::fe::festaterewards::tests::KEEP_F0;
    pub(crate) use crate::game::fe::festaterewards::tests::hook;
    pub(crate) use crate::game::fe::festaterewards::tests::strict;
    pub(crate) use crate::game::fe::festaterewards::tests::base_9 as base_6;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::LEAF;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::BE_DUMMY;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::WORLD_4 as WORLD_5;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base_14 as base_7;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::check3;
    pub(crate) use crate::game::fe::fepopup::tests::BEL;
    pub(crate) use crate::game::fe::fepopup::tests::WORLD_4 as WORLD_6;
    pub(crate) use crate::game::fe::fepopup::tests::AUX as AUX_2;
    pub(crate) use crate::game::fe::fepopup::tests::SYS;
    pub(crate) use crate::game::fe::fepopup::tests::RACE;
    pub(crate) use crate::game::fe::fepopup::tests::RMAN_2;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;
    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_8;


    #[test]
    fn mc_manager_set_mode_matches() {
        check(ssx3_ee::s06::f_0023d570, mc_manager_set_mode, 300, &|c, seed| {
            let mut r = Rng(0x23d5_7000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = match seed % 5 {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => ((r.next_u32() as u64) << 32) | (r.next_u32() as u64 | 2),
                _ => 0x1_0000_0000,
            };
            c.bus.write32(GAME_GP - 0x512c, r.edge_f32());
            c.bus.write32(DATA + 0x108, r.next_u32());
            c.bus.write32(DATA + 0x110, r.next_u32());
            c.bus.write32(DATA + 0x114, r.next_u32());
        });
    }


    #[test]
    fn ov_screen_reward_list_on_activate_matches() {
        check(ee::s05::f_001ff6d0__generated, ov_screen_reward_list_on_activate, 20, &|c, seed| {
            machine(c, seed, 0x1ff6d0, 6);
            globals(c);
            clear(c, DATA, 0x400);
            c.g[A0][0] = DATA as u64;
            let _ = seed;
        });
    }


    #[test]
    fn ov_multi_freestyle_pre_on_deactivate_matches() {
        check(ssx3_ee::s04::f_001fd118__generated, ov_multi_freestyle_pre_on_deactivate, SEEDS, &|c, seed| {
            let _r = machine(c, seed, 0xfd118, 1);
            be_library(c);
            world(c);
            c.bus.write32(GAME_GP + 0x410, DATA + 0x2000);
            c.bus.write32(
                GAME_GP.wrapping_sub(0x204),
                if seed % 2 == 0 { DATA + 0x2400 } else { 0 },
            );
            c.g[A0][0] = sx(DATA + 0x300);
        });
    }


    /// A non-null, zeroed stand-in for the BE-library singleton at
    /// `gp - 0x1f04` (its lazy constructor would otherwise load from disc).
    pub(crate) const BE_LIBRARY: u32 = DATA + 0x1_f000;

    /// A small zeroed world (`world -> +0x84 -> +0xc`, rider count `+0x7c`).
    pub(crate) const WORLD: u32 = DATA + 0x2_0000;


    pub(crate) fn globals_2(c: &mut Cpu) {
        c.bus.write32(GAME_GP - 0x1f04, BE_LIBRARY);
        c.bus.write32(GAME_GP - 0x848, WORLD);
        let game = WORLD + 0x1000;
        c.bus.write32(WORLD + 0x84, game);
        let race = WORLD + 0x2000;
        c.bus.write32(game + 0xc, race);
        c.bus.write32(race + 0x7c, 0);
    }


    // ------------------------------------------------------ overlay screens

    pub(crate) fn ov_pre_update_test(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 40, &|c, seed| {
            let _r = machine(c, seed, salt, 1);
            // Zeroed BE library: no disc load on first use.
            c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x6000);
            c.g[A0][0] = sx(DATA);
            match seed % 3 {
                0 => c.bus.write32(GAME_GP - 0x204, 0),
                1 => {
                    c.bus.write32(GAME_GP - 0x204, DATA + 0x3000);
                    c.bus.write32(DATA + 0x3064, 1);
                }
                _ => {
                    c.bus.write32(GAME_GP - 0x204, DATA + 0x3000);
                    c.bus.write32(DATA + 0x3064, 0);
                }
            }
        });
    }


    #[test]
    fn ov_screen_race_pre_update_matches() {
        ov_pre_update_test(ssx3_ee::s04::f_001fbce0__generated, ov_screen_race_pre_update, 0x28);
    }


    #[test]
    fn ov_screen_multi_freestyle_pre_update_matches() {
        ov_pre_update_test(
            ssx3_ee::s04::f_001fd150__generated,
            ov_screen_multi_freestyle_pre_update,
            0x29,
        );
    }


    #[test]
    fn ov_screen_multi_freestyle_pre_on_activate_matches() {
        check(
            ssx3_ee::s04::f_001fd0d8__generated,
            ov_screen_multi_freestyle_pre_on_activate,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x2a, 1);
                globals_2(c);
                c.g[A0][0] = sx(DATA);
                if seed % 2 == 0 {
                    c.bus.write32(GAME_GP - 0x204, 0);
                } else {
                    c.bus.write32(GAME_GP - 0x204, DATA + 0x3000);
                }
            },
        );
    }


    pub(crate) fn save_screen_ctor_test(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 30, &|c, seed| {
            let _r = machine(c, seed, salt, 1);
            // Zeroed BE library: no disc load on first use.
            c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x6000);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = sx(DATA + 0x3000);
            c.g[A2][0] = (seed % 5) as u64;
        });
    }


    #[test]
    fn ov_screen_save_replay_ctor_matches() {
        save_screen_ctor_test(ssx3_ee::s05::f_001fdbf0__generated, ov_screen_save_replay_ctor, 0x2b);
    }


    #[test]
    fn ov_screen_save_records_ctor_matches() {
        save_screen_ctor_test(ssx3_ee::s05::f_001fe968__generated, ov_screen_save_records_ctor, 0x2c);
    }


    pub(crate) fn close_top_state_once_test(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 40, &|c, seed| {
            let _r = machine(c, seed, salt, 1);
            c.g[A0][0] = sx(DATA);
            c.bus
                .write32(DATA + 0x1ac, if seed % 2 == 0 { 0 } else { 1 });
            c.bus.write32(DATA + 0x10, DATA + 0x3000);
        });
    }


    #[test]
    fn ov_screen_save_replay_v33_matches() {
        close_top_state_once_test(ssx3_ee::s05::f_001fdf00__generated, ov_screen_save_replay_v33, 0x2d);
    }


    #[test]
    fn ov_screen_save_records_v33_matches() {
        close_top_state_once_test(ssx3_ee::s05::f_001fec60__generated, ov_screen_save_records_v33, 0x2e);
    }


    #[test]
    fn ov_screen_reward_list_on_widget_action_matches() {
        check(
            ssx3_ee::s05::f_001ff700__generated,
            ov_screen_reward_list_on_widget_action,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x1ff700, 0);
                be_library_2(c);
                c.g[A0][0] = sx(DATA + 0x100);
                // No widget, or a widget with an action other than "confirm":
                // both return without touching the BE library.
                c.g[A1][0] = if seed % 2 == 0 { 0 } else { sx(DATA + 0x1400) };
                c.g[A2][0] = if seed % 4 == 1 { 5 } else { 0 };
            },
        );
    }


    #[test]
    fn ov_screen_be_options_v22_matches() {
        check(ee::s04::f_001faaf0__generated, ov_screen_be_options_v22, 200, &|c, seed| {
            setup(c, seed, 33);
            c.g[A2][0] = [4u64, 0, 9][(seed % 3) as usize];
        });
    }


    #[test]
    fn ov_screen_be_options_v21_matches() {
        check(ee::s04::f_001fab08__generated, ov_screen_be_options_v21, 200, &|c, seed| {
            setup(c, seed, 34);
            c.g[A2][0] = if seed % 2 == 0 { 0 } else { seed as u64 };
        });
    }


    #[test]
    fn ov_screen_race_pre_on_deactivate_matches() {
        check(ee::s04::f_001fbcc8__generated, ov_screen_race_pre_on_deactivate, 200, &|c, seed| {
            let mut r = setup(c, seed, 35);
            if seed % 3 == 0 {
                gpf(c, -0x204, 0);
            } else {
                gpf(c, -0x204, AUX);
                put(c, AUX + 0x64, r.next_u32());
            }
        });
    }


    #[test]
    fn ov_screen_rival_pre_on_widget_notify_matches() {
        check(ee::s05::f_001fdb78__generated, ov_screen_rival_pre_on_widget_notify, 200, &|c, seed| {
            setup(c, seed, 36);
            c.g[A2][0] = [6u64, 0, 9][(seed % 3) as usize];
        });
    }


    #[test]
    fn ov_screen_save_replay_on_child_event_matches() {
        check(
            ee::s05::f_001fe8f0__generated,
            ov_screen_save_replay_on_child_event,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x1f_e8f0, 3);
                world_2(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.g[A1][0] = (DATA + 0x2200) as u64;
                c.g[A2][0] = if seed % 2 == 0 { 0x16 } else { 0x11 };
                w(c, DATA + 0x2000 + 0x19c, if seed % 3 == 0 { 5 } else { 4 });
                c.pc = 0x001f_e8f0;
            },
        );
    }


    #[test]
    fn ov_screen_be_options_on_widget_notify_matches() {
        check(ee::s04::f_001faa78__generated, ov_screen_be_options_on_widget_notify, SEEDS_3, &|c, seed| {
            let mut r = machine(c, seed, 0x1faa78, 2);
            be_library_3(c);
            let hash = str_hash(c, 0x004a_24e8);
            let menu = DATA;
            c.g[A0][0] = r.next_u32() as u64;
            c.g[A1][0] = menu as u64;
            c.g[A2][0] = (seed % 12) as u64;
            c.g[A3][0] = (seed % 4) as u64;
            let s = DATA + 0x2000;
            let a = DATA + 0x2100;
            w(c, menu + 0x74 + 4, a);
            w(c, a + 4, s);
            w(c, a, 0);
            w(c, s + 4, s);
            w(c, s, 0);
            w(c, a + 0x38, if seed % 3 == 0 { hash } else { r.next_u32() });
        });
    }


        #[test]
    fn ov_screen_save_replay_v38_matches() {
        check(
            ee::s05::f_001fe5b0__generated,
            ov_screen_save_replay_v38,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x1fe5b0, 4);
                let obj = DATA + 0x1000;
                let screen = DATA + 0x2000;
                let node = DATA + 0x3000;
                let vt = DATA + 0x4_0000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(c, obj + 0x40, screen);
                hash_object(c, screen, if seed % 5 == 0 { 0 } else { node });
                w(c, node + 0x8, vt);
                slot(c, vt, 0x48, 6, FAKE);
                w(c, obj + 0x220, r.next_u32());
            },
        );
    }


    #[test]
    fn ov_screen_save_records_v38_matches() {
        check(
            ee::s05::f_001ff278__generated,
            ov_screen_save_records_v38,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x1ff278, 4);
                let obj = DATA + 0x1000;
                let screen = DATA + 0x2000;
                let node = DATA + 0x3000;
                let vt = DATA + 0x4_0000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(c, obj + 0x40, screen);
                hash_object(c, screen, if seed % 5 == 0 { 0 } else { node });
                w(c, node + 0x8, vt);
                slot(c, vt, 0x48, 6, FAKE);
                w(c, obj + 0x220, r.next_u32());
            },
        );
    }


    #[test]
    fn ov_screen_in_game_error_update_server_timeout_text_matches() {
        check(
            ee::s04::f_001fb180__generated,
            ov_screen_in_game_error_update_server_timeout_text,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x1fb180, 4);
                let obj = DATA + 0x1000;
                let screen = DATA + 0x2000;
                let node = DATA + 0x3000;
                let vt = DATA + 0x4_0000;
                c.g[A0][0] = obj as u64;
                w(c, obj + 0x40, if seed % 4 == 0 { 0 } else { screen });
                w(c, obj + 0xa0, (seed % 7) as u32);
                hash_object(c, screen, if seed % 5 == 0 { 0 } else { node });
                w(c, node + 0x8, vt);
                slot(c, vt, 0x48, 6, FAKE);
                let _ = r.next_u32();
            },
        );
    }


        #[test]
    fn ov_screen_in_game_error_update_opponent_disconnected_text_matches() {
        check(
            ssx3_ee::s04::f_001fb218__generated,
            ov_screen_in_game_error_update_opponent_disconnected_text,
            12,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x001f_b218);
                let self_obj = DATA + 0x100;
                let screen = DATA + 0x2000;
                let objects = DATA + 0x2100;
                let obj = DATA + 0x2200;
                let vtable = DATA + 0x3000;
                clear_2(c, self_obj, 0x100);
                clear_2(c, screen, 0x80);
                clear_2(c, objects, 0x20);
                clear_2(c, obj, 0x100);
                clear_2(c, vtable, 0x80);
                w(c, self_obj + 0xa0, r.next_u32());
                if seed % 4 != 0 {
                    w(c, self_obj + 0x40, screen);
                    w(c, screen + 0x3c, 1);
                    w(c, screen + 0x38, objects);
                    w(c, objects, obj);
                    let hv2 = str_hash_2(c, 0x0046_fb18) as u32;
                    w(c, obj + 0x38, hv2);
                    w(c, obj + 8, vtable);
                    c.bus.write16(vtable + 0x48, 0);
                    w(c, vtable + 0x4c, crate::difftest::fakes::FAKE);
                    w(c, crate::difftest::fakes::LOG_RESULTS, 1);
                }
                c.g[A0][0] = sx(self_obj);
            },
        );
    }


            #[test]
    fn ov_screen_save_replay_v39_matches() {
        check(ssx3_ee::s05::f_001fe648__generated, ov_screen_save_replay_v39, 20, &|c, seed| {
            let _r = base_3(c, seed, 0x1fe648, 2);
            let this = DATA + 0x1000;
            let screen = DATA + 0x2000;
            c.g[A0][0] = this as u64;
            c.g[A2][0] = (seed & 1) as u64;
            w_2(c, this + 0x40, screen);
            hash_object(c, screen, DATA + 0x3000);
            w_2(c, DATA + 0x3000 + 8, DATA + 0x3100);
            c.bus.write16(DATA + 0x3148, 0x48);
            w_2(c, DATA + 0x314c, 0);
        });
    }


    #[test]
    fn ov_screen_rival_pre_init_matches() {
        check(ee::s05::f_001fd268__generated, ov_screen_rival_pre_init, 40, &|c, seed| {
            alloc_base(c, seed, 0x1fd2, 5);
            c.g[A0][0] = DATA as u64;
            w(c, DATA + 0x10, DATA + 0x400);
            w(c, DATA + 0x400, DATA + 0x500);
        });
    }


    #[test]
    fn ov_screen_in_game_error_show_cancel_game_matches() {
        check(
            ee::s04::f_001faff8__generated,
            ov_screen_in_game_error_show_cancel_game,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1faf, 4);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 2) as u64;
                let screen = DATA + 0x2000;
                let obj = DATA + 0x2400;
                let menu = DATA + 0x2800;
                c.bus.write32(this + 0x40, screen);
                let vt = vtable(c, &mut r, &[0x40, 0x48]);
                c.bus.write32(screen + 0x40, obj);
                c.bus.write32(screen + 0x48, menu);
                c.bus.write32(obj + 8, vt);
                c.bus.write32(menu + 8, vt);
            },
        );
    }


    #[test]
    fn ovscreeningameerror_show_continue_button_matches() {
        check(
            ee::s04::f_001fb378__generated,
            ovscreeningameerror_show_continue_button,
            8,
            &|c, seed| {
                let _ = base_5(c, seed, 0x1fb378, 2);
                let this = DATA;
                clear_3(c, this, 0x400);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(c, this + 0x40, 0);
            },
        );
    }


    #[test]
    fn ovscreen_in_game_error_update_matches() {
        strict(
            ee::s04::f_001faf10__generated,
            ovscreen_in_game_error_update,
            120,
            &|c, seed| {
                let _ = base_6(c, seed, 0x1faf, 6);
                c.g[A0][0] = DATA as u64;
                let err = DATA + 0x300;
                w(
                    c,
                    GAME_GP.wrapping_sub(0x204),
                    if seed % 8 == 0 { 0 } else { err },
                );
                w(c, DATA + 0x9c, if seed % 7 == 0 { 0 } else { 1 });
                w(
                    c,
                    err + 0x138,
                    match seed % 3 {
                        0 => 0,
                        1 => 5,
                        _ => 0xffff_ffff,
                    },
                );
                w(c, err + 0x13c, if seed % 4 == 0 { 0 } else { 1 });
                w(c, err + 0x80, if seed % 5 == 0 { 0 } else { 1 });
                w(c, err + 0xa0, if seed % 6 == 0 { 0 } else { 1 });
                w(c, err + 0x24, if seed % 9 == 0 { 0 } else { 1 });
                // The screen and the "ping" object found by hash.
                w(c, DATA + 0x40, DATA + 0x400);
                let obj = DATA + 0x500;
                let vt = DATA + 0x1000;
                w(c, obj + 8, vt);
                w(c, vt + 0x48, 0);
                w(c, vt + 0x4c, FAKE);
            },
        );
    }


    /// A screen whose child list matches the three names the highlight
    /// handlers look up (the hashes are computed by the game's own hash
    /// function, so the lookups really find the widgets).
    pub(crate) fn highlight_screen(c: &mut Cpu, names: [u32; 3]) {
        let obj = DATA;
        let screen = DATA + 0x100;
        let count = DATA + 0x200;
        let list = DATA + 0x300;
        c.bus.write32(obj + 0x40, screen);
        c.bus.write32(screen + 0x38, count);
        c.bus.write32(count, 3);
        c.bus.write32(screen + 0x3c, list);
        for (i, name) in names.iter().enumerate() {
            c.g[A0][0] = sx(*name);
            ssx3_ee::s10::f_00317670(c);
            let hash = c.g[V0][0] as u32;
            let w = DATA + 0x1000 + 0x80 * i as u32;
            c.bus.write32(list + 4 * i as u32, w);
            c.bus.write32(w + 0x38, hash);
            c.bus.write32(w + 8, w + 0x40);
            for slot in [0x48u32, 0x58] {
                c.bus.write16(w + 0x40 + slot, 0);
                c.bus.write32(w + 0x40 + slot + 4, LEAF);
            }
            // The clear path also uses the third widget itself as a table.
            c.bus.write32(w + 0x4c, LEAF);
        }
    }


    #[test]
    fn in_game_error_highlight_continue_matches() {
        check3(ee::s04::f_001fb458__generated, in_game_error_highlight_continue, 40, &|c, seed| {
            let _ = base_7(c, seed, 0x1fb458);
            highlight_screen(c, [0x0046_fb48, 0x0046_fad8, 0x0046_fb00]);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 2) as u64;
        });
    }


    #[test]
    fn in_game_error_highlight_cancel_matches() {
        check3(ee::s04::f_001fb588__generated, in_game_error_highlight_cancel, 40, &|c, seed| {
            let _ = base_7(c, seed, 0x1fb588);
            highlight_screen(c, [0x0046_fa90, 0x0046_fac8, 0x0046_fae8]);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 2) as u64;
        });
    }


    pub(crate) const SAVE_REC: u32 = DATA + 0x1_0000;

    pub(crate) const SAVE_REC_MSG: u32 = DATA + 0x1_1000;

    pub(crate) const SAVE_MC: u32 = DATA + 0x1_2000;


    #[test]
    fn ov_screen_save_records_on_widget_action_matches() {
        check_all(
            ee::s05::f_001ff3b0__generated,
            ov_screen_save_records_on_widget_action,
            24,
            &|c, seed| {
                let mut r = base_8(c, seed, 0x1ff3_b0, 12);
                c.g[A0][0] = sx(SAVE_REC);
                c.g[A1][0] = sx(if seed % 5 == 0 { 0 } else { SAVE_REC_MSG });
                c.g[A2][0] = sx([5u32, 6, 7, 0, 4][seed as usize % 5]);
                c.bus.write32(SAVE_REC_MSG + 0x18, r.next_u32() % 4);
                c.bus.write32(SAVE_REC + 0x1a8, ((seed / 5) % 3) as u32);
                c.bus.write32(SAVE_REC + 0x1c0, ((seed / 7) % 8) as u32);
                c.bus.write32(SAVE_REC + 0x214, ((seed / 11) % 4) as u32);
                c.bus.write32(SAVE_REC + 0x1c4, r.next_u32() % 4);
                // The movie manager: the object at_3 +0x434 carries the vtable the
                // card-ready check goes through; +0x428 is its state.
                c.bus.write32(SAVE_MC + 0x428, ((seed / 3) % 3) as u32);
                c.bus.write32(SAVE_MC + 0x434, DATA + 0x1_3000);
                c.bus.write32(SAVE_MC + 0x748, 0);
                c.bus.write32(DATA + 0x1_3000 + 8, 0);
                let _ = &mut r;
            },
        );
    }
}
