//! `src/fe/festateoptions.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
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
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::Cpu;
use crate::call::VSlot;
use crate::abi::r32;
use crate::call::sp;
use crate::events::back_edge_unwinds;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use crate::call::Flow;
use crate::call;
pub(crate) use crate::game::fe::ovstatepause::p;
pub(crate) use crate::game::dirtysock::tags2::p_2;
pub(crate) use crate::game::dirtysock::tags2::gp_5 as gp;
pub(crate) use crate::game::dirtysock::tags2::sw;
pub(crate) use crate::game::dirtysock::tags2::leave;
pub(crate) use crate::game::dirtysock::tags2::const_reg;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw as sw_2;
pub(crate) use crate::game::fe::fepopup2::fe_state_screen_init;
pub(crate) use crate::game::fe::festateloadhint::p_3;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::scripter::ssxscriptengine::sw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::main::gamemode::low as low_2;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::main::gamemode::gpo;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::fe::feasyncfile::at;
pub(crate) use crate::game::fe::festaterewards::at_4 as at_2;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw_2;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw_3;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_3;
pub(crate) use crate::game::fe::feasyncfile::lh;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_2;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_3;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_4;
pub(crate) use crate::game::ai::rider::world;
use crate::call::returned;
use ssx3_ee_rt::funcs::call_indirect;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0018_8800, c_fe_state_options_init, "cFEStateOptions_Init"),
    (0x0018_8c58, cfe_state_options_handle_input, "cFEStateOptions_HandleInput"),
    (0x0018_8e58, c_fe_state_options_open_reset_popup, "cFEStateOptions_OpenResetPopup"),
    (0x0018_8ee8, c_fe_state_options_game_ctor, "cFEStateOptionsGame_ctor"),
    (0x0018_9958, cfe_state_options_game_on_widget_notify, "cFEStateOptionsGame_OnWidgetNotify"),
    (0x0018_9970, cfe_state_options_game_v21, "cFEStateOptionsGame_v21"),
    (0x0018_a258, c_fe_state_options_sound_ctor, "cFEStateOptionsSound_ctor"),
    (0x0018_a890, cfe_state_options_sound_on_widget_notify, "cFEStateOptionsSound_OnWidgetNotify"),
    (0x0018_a8c0, cfe_state_options_sound_v21, "cFEStateOptionsSound_v21"),
    (0x0018_c0e8, c_fe_state_options_hud_ctor, "cFEStateOptionsHUD_ctor"),
    (0x0018_c128, c_fe_state_options_hud_init, "cFEStateOptionsHUD_Init"),
    (0x0018_c1b8, fe_state_options_hud_on_child_event, "cFEStateOptionsHUD_OnChildEvent"),
    (0x0018_c270, cfe_state_options_hud_on_widget_notify, "cFEStateOptionsHUD_OnWidgetNotify"),
    (0x0018_c658, fe_state_options_controller_ctor, "cFEStateOptionsController_ctor"),
    (0x0018_c8b8, cfe_state_options_controller_shutdown, "cFEStateOptionsController_Shutdown"),
    (0x0018_cdb0, cfe_state_options_controller_v21, "cFEStateOptionsController_v21"),
    (0x0018_d058, cfe_state_options_controller_update, "cFEStateOptionsController_Update"),
    (0x0018_d240, c_fe_state_options_save_load_ctor, "cFEStateOptionsSaveLoad_ctor"),
    (0x0018_d280, c_fe_state_options_save_load_init, "cFEStateOptionsSaveLoad_Init"),
    (0x0018_d3c0, c_fe_state_options_save_load_on_render, "cFEStateOptionsSaveLoad_OnRender"),
    (0x0018_d438, cfe_state_options_save_load_on_widget_notify, "cFEStateOptionsSaveLoad_OnWidgetNotify"),
    (0x0018_d7f0, c_fe_state_options_device_select_ctor, "cFEStateOptionsDeviceSelect_ctor"),
    (0x0018_ddd0, cfe_state_options_device_select_on_widget_notify, "cFEStateOptionsDeviceSelect_OnWidgetNotify"),
    (0x0018_e178, c_fe_state_options_device_select_v37, "cFEStateOptionsDeviceSelect_v37"),
    (0x0018_e2c0, cfe_state_options_device_select_v38, "cFEStateOptionsDeviceSelect_v38"),
    (0x0018_ca10, fe_options_controller_on_widget_action, "cFEStateOptionsController_OnWidgetAction"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::camera::camera::callee::MEASURE_ROW;
    pub(crate) use crate::game::camera::camera::callee::MEMCARD_SET_STATE;
    pub(crate) use crate::game::camera::camera::callee::TRANSITION_MAN_START;
    pub(crate) use crate::game::camera::camera::callee::WSCRIPT_DELETE_ALL;
    pub(crate) use crate::game::camera::camera::callee::SPHERE_TREE_RESET_TRANSFORM;
    pub(crate) use crate::game::camera::camera::callee::CAM_CONTROLLER_RESET;
    pub(crate) use crate::game::camera::camera::callee::MANUAL_CAM_ALGO_BASE_CTOR;
    pub(crate) use crate::game::camera::camera::callee::CHASE_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::camera::camera::callee::CHASE_CAM_SET_ALGORITHM_2 as CHASE_CAM_SET_ALGORITHM;
    pub(crate) use crate::game::camera::camera::callee::CAMERA_TRIGGER_FREE_TRIGGERS;
    pub(crate) use crate::game::camera::camera::callee::CAMERA_TRIGGER_RESET_HEADER;
    pub(crate) use crate::game::camera::camera::callee::UI_TEXTURE_SET_FIND;
    pub(crate) use crate::game::camera::camera::callee::UI_STATE_CTOR;
    pub(crate) use crate::game::camera::camera::callee::BENEWPLAYER_GET_SLOT;
    pub(crate) use crate::game::camera::camera::callee::BE_LIBRARY_RESET_PROFILE;
    pub(crate) use crate::game::camera::camera::callee::BEBAGT_RESET_PROFILE;
    pub(crate) use crate::game::camera::camera::callee::BE_OPTION_GET_MAX_HANDICAP;
    pub(crate) use crate::game::camera::camera::callee::FE_REFRESH_PROFILE_LIST;
    pub(crate) use crate::game::camera::camera::callee::FE_REFRESH_SLOTS;
    pub(crate) use crate::game::fe::ovstatepause::callee::MPEG2_SHOW_BITS;
    pub(crate) use crate::game::fe::ovstatepause::callee::MPEG2_FLUSH_BUFFER;
    pub(crate) use crate::game::fe::ovstatepause::callee::AI_DESTROY_MISSION_RIDERS;
    pub(crate) use crate::game::fe::ovstatepause::callee::AI_CREATE_COMPUTER_RIDERS;
    pub(crate) use crate::game::fe::ovstatepause::callee::ANIM_CONTROLLER_GET_LAYER_ANIM_ENTRY;
    pub(crate) use crate::game::fe::ovstatepause::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::ovstatepause::callee::BE_REWARD_INT_COUNT_SET_BITS;
    pub(crate) use crate::game::fe::ovstatepause::callee::CHASE_CAMERA_ALGORITHM_CTOR as CHASE_CAMERA_ALGORITHM_CTOR_2;
    pub(crate) use crate::game::fe::ovstatepause::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::fe::ovstatepause::callee::RIDER_METRIX_CLEAR_PEAK_A4;
    pub(crate) use crate::game::fe::ovstatepause::callee::EA_MEM_SET;
    pub(crate) use crate::game::fe::ovstatepause::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::fe::ovstatepause::callee::MC_MANAGER_BEGIN_LOAD;
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_STATE_CTOR as UI_STATE_CTOR_2;
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
    pub(crate) use crate::game::fe::ovstatepause::callee::UI_TEXTURE_SET_FIND as UI_TEXTURE_SET_FIND_2;
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
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_CTOR_2 as UI_STATE_CTOR_3;
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
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::APP_VSYNC_CALLBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_PREPROC_WRITE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MOD_OBJ_CREATE_MODIFIER_BLOCK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_TEX_FLIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MODIFIER_BLOCK_FREE_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MULTI_PARTICLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::DYNAMIC_EMITTER_FREE_DATA_2 as DYNAMIC_EMITTER_FREE_DATA;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RCMP_DECODER_RELEASE_CODEC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_BONUS_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_ADD_BOOST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_SHOW_HUD_NUMBER_2 as RIDER_METRIX_SHOW_HUD_NUMBER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_METRIX_AWARD_POINTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::AI_CREATE_ACTOR_RIDERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CHASE_CAM_CONTROLLER_RESET_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ACTIVE_CAMERA_TRIGGER_LIST_CLEAR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_STACK_CLEAR_2 as CAMERA_TRIGGER_STACK_CLEAR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_REMOVE_ALL_TRIGGERS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_TRIGGER_MAN_CLEAR_DATA;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_OPTIONS_CONTROLLER_SET_VIBRATION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_ALL_TABLES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_STATE_NET_ACCOUNT_MEM_SET_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::NET_FE_STATE_DTOR_3 as NET_FE_STATE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CFE_LOAD_STATE_SETUP_RESULT_WIDGET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::EVENT_SELECT_ON_WIDGET_ACTION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_PLAYBACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PLAYBACK_MAN_STOP_RECORDING;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_CACHE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_STOP_2 as SCRIPT_LIST_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_EXIT_SCRIPT_MODE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_ON_SEQ_END;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_TARGET_OBJECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE_2 as SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_OBJ_TABLE_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_PROCESS_2 as SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_STOP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_UNLOAD_MEMORY_2 as SND_BANK_UNLOAD_MEMORY;
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
    pub(crate) use crate::game::main::gamemode::callee::SCRIPT_ENGINE_GET_SEQ_PROCESS_ID as SCRIPT_ENGINE_GET_SEQ_PROCESS_ID_2;
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
    pub(crate) use crate::game::main::gamemode::callee::MULTI_PARTICLE_DTOR as MULTI_PARTICLE_DTOR_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST as UI_LIST_FIRST_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_4 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_2 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_10 as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_2 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_HIDE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SHOW;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP_2 as NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_GET_CHARACTER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_FILL_PLAYER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_SET_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_PDA_TEMPLATE_CLOSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_MESSAGE_CENTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_COUNT_2 as BUDDY_MAN_GET_ROSTER_COUNT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_ENTRY_2 as BUDDY_MAN_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_DATA_GET_BANK_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_UNREGISTER_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ANIM_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_SOUND_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_INSTANCE_3 as SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_OBJ_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ON_ACTIVATED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_3 as SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::CUI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_3 as UI_LIST_NODE_DTOR_BODY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR_2 as UI_OBJECT_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_STREAMER_UPDATE_VIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_SET_IN_GAME_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SET_INPUT_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_SET_OWNER;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_3 as WORLD_OBJ_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_READ_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_FLAG_MANAGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_MANAGER_ADD_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_APPLY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_ADVANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_PROCESS_DTOR_2 as GAME_PROCESS_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_RESET_RACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODE_MAN_RESET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_REQUEST_STATE_5 as GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_START_NEXT_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_APP;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_INTERFACE_GET_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_BE_2 as BE_GET_BE;
    pub(crate) use crate::game::fe::feasyncfile::callee::VSPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::EDITOR_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_SELECT_ALGORITHM;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_FREE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_LOAD_BANK_FROM_MEMORY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_MUSIC_TIER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_TRACK_TYPE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS_2 as SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_KILL_ALL_PROCESSES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS_2 as SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE_2 as SCRIPT_ENGINE_RESET_PROCESS_TABLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COMPUTE_OFFSET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_AABB_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_PAD_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_NODE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME_2 as UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBE_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_2 as NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_ADD_CHILD;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_FROM_ASCII_2 as USTR_FROM_ASCII;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_SPRINTF;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_TO_ASCII as USTR_TO_ASCII_2;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_SET_WIDGET_ALIGN;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_SET_WRAPPED_TEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::RIDER_CAMERA_ALGORITHM_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::DEBUG_MENU_DISPATCH_ITEM_OP;
    pub(crate) use crate::game::bx::bxstring2::callee::WSCRIPT_MISSION_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::bx::bxstring2::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT_2 as WSCRIPT_PROCESS_LIST_PUSH_FRONT;
    pub(crate) use crate::game::bx::bxstring2::callee::MULTIPARTICLE_MAN_CLEAR;
    pub(crate) use crate::game::bx::bxstring2::callee::MULTIPARTICLE_CTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::MUSIC_PLAYER_SET_EVENT_STATE;
    pub(crate) use crate::game::bx::bxstring2::callee::MUSIC_PLAYER_GET_SONG_TAG;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_UPDATE as UI_STATE_UPDATE_2;
    pub(crate) use crate::game::bx::bxstring2::callee::OPTIONS_SET_VIBRATION;
    pub(crate) use crate::game::bx::bxstring2::callee::STR_HASH as STR_HASH_3;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_SCREEN_FIND_OBJECT as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_MENU_FIND_FIRST_SELECTABLE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_MENU_SET_SELECTION as UI_MENU_SET_SELECTION_2;
    pub(crate) use crate::game::bx::bxstring2::callee::STRLEN as STRLEN_3;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_DEQUEUE_SECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_POP_FREE_NODE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_ADD_ITEM_STR_ID;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_FIND_ITEM_BY_VALUE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_LISTBOX_SET_SELECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_FE_STATE_GET_POPUP as NET_FE_STATE_GET_POPUP_2;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_FE_STATE_SHOW_POPUP as NET_FE_STATE_SHOW_POPUP_2;
    pub(crate) use crate::game::bx::bxstring2::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::bx::bxstring2::callee::MESSAGE_CENTER_GET_INBOX;
    pub(crate) use crate::game::bx::bxstring2::callee::CBXSTRING_CTOR_COPY;
    pub(crate) use crate::game::bx::bxstring2::callee::CBXSTRING_DTOR;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::bx::bxstring2::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_GET_LANGUAGE_MASK;
    pub(crate) use crate::game::bx::bxstring2::callee::SSX_AUDIO_GET_SPEECH_CONTEXT_MASK;
    pub(crate) use crate::game::bx::bxstring2::callee::INI_PARSER_SKIP_WHITESPACE;
    pub(crate) use crate::game::bx::bxstring2::callee::INI_PARSER_EXPECT;
    pub(crate) use crate::game::bx::bxstring2::callee::INI_PARSER_GET_CHAR;
    pub(crate) use crate::game::bx::bxstring2::callee::CHUNK_RESOLVE_RELOCATE_DMA;
    pub(crate) use crate::game::bx::bxstring2::callee::FLAG_MANAGER_GROUP_BUILD_MESH;
    pub(crate) use crate::game::bx::bxstring2::callee::BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::bx::bxstring2::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_2;
    pub(crate) use crate::game::bx::bxstring2::callee::BE_GET_CHAR_PROFILE_EVENT_RECORD;
    pub(crate) use crate::game::bx::bxstring2::callee::OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::READ_PRIVATE_MESSAGE_SET_TEXT;
    pub(crate) use crate::game::bx::bxstring2::callee::NET_MANAGER;
    pub(crate) use crate::game::bx::bxstring2::callee::OV_SCREEN_PDA_TEMPLATE_CLOSE;
    pub(crate) use crate::game::bx::bxstring2::callee::PATH_DATA_READ_INT;
    pub(crate) use crate::game::bx::bxstring2::callee::PATH_DATA_READ_FLOAT_3 as PATH_DATA_READ_FLOAT;
    pub(crate) use crate::game::bx::bxstring2::callee::SUPERPIPE_COMPUTE_DESIRED_EYE;
    pub(crate) use crate::game::bx::bxstring2::callee::SUPERPIPE_COMPUTE_LOOK_AT;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAMERA_RESET_FOR_RIDER;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAMERA_INIT_FROM_RIDER;
    pub(crate) use crate::game::bx::bxstring2::callee::MEMSET as MEMSET_2;
    pub(crate) use crate::game::bx::bxstring2::callee::QSORT;
    pub(crate) use crate::game::bx::bxstring2::callee::CROWD_MAN3D_GROUP_BUILD_MATRICES;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAM_CONTROLLER_RESTORE_DEFAULT;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_LIST_START;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_SET_SKIP_DELAY;
    pub(crate) use crate::game::bx::bxstring2::callee::CUTSCENE_TRANSITION_MAN_FORCE_ON;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_SHUTDOWN as UI_STATE_SHUTDOWN_2;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_MGR_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::bx::bxstring2::callee::GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::bx::bxstring2::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_2;
    pub(crate) use crate::game::bx::bxstring2::callee::EA_MEM_FREE as EA_MEM_FREE_3;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_LEN;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_CPY;
    pub(crate) use crate::game::bx::bxstring2::callee::UITEXT_SET_WIDE_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::MALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FREE2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::VEC3_MAX_COMPONENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_12 as NET_FE_STATE_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_3 as NET_FE_STATE_INIT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_SELECTED_USER;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_MANAGER_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_SET_FLAG_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_BIND;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE_2 as AUDIO_GET_GAME_MODULE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_SOMETHING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_NEW;
    pub(crate) const CBE_GET_INTERFACE: Callee =
        Callee { addr: 0x0014_dd58, f: ee::s01::f_0014dd58 };

    pub(crate) const ALLOC: Callee = Callee { addr: 0x0031_7d70, f: ee::s10::f_00317d70 };

    pub(crate) const UI_POPUP_CTOR: Callee = Callee { addr: 0x0039_e318, f: ee::s13::f_0039e318 };

    pub(crate) const UI_SCREEN_ADD: Callee = Callee { addr: 0x0039_f290, f: ee::s13::f_0039f290 };

    pub(crate) const UI_SCREEN_ADD_OBJECT: Callee =
        Callee { addr: 0x0039_f400, f: ee::s13::f_0039f400 };

    pub(crate) const WIDGET_CHECKED: Callee = Callee { addr: 0x0039_a738, f: ee::s13::f_0039a738 };

    pub(crate) const VIBRATION_SET: Callee = Callee { addr: 0x0014_7528, f: ee::s01::f_00147528 };

    pub(crate) const VIBRATION_APPLY: Callee = Callee { addr: 0x0014_74e8, f: ee::s01::f_001474e8 };

    pub(crate) const SET_VIBRATION: Callee = Callee { addr: 0x0018_d108, f: ee::s02::f_0018d108 };

    pub(crate) const SHOW_CONFIG_TEXT: Callee =
        Callee { addr: 0x0018_cf50, f: ee::s02::f_0018cf50 };

    pub(crate) const CONFIG_SET: Callee = Callee { addr: 0x0014_7658, f: ee::s01::f_00147658 };
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
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


// ---------------------------------------------------------------------------
// cFEState constructors.

/// `cFEStateOptionsGame* ctor(this)` (0x188ee8): base `UIState_ctor`, then the
/// state vtable at `+0x8` (`D_0046C6F0`) and id 0x1c at `+0xc`.
pub fn c_fe_state_options_game_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        callee::UI_STATE_CTOR.call(c, 0x0018_8efc)?;
        c.g[V0][0] = 0x1c;
        c.g[V1][0] = 0x0047_0000;
        let id = c.g[V0][0];
        c.sw((this as u32).wrapping_add(0xc), id);
        c.g[V1][0] = sx(0x0046_c6f0);
        let vt = c.g[V1][0];
        c.sw((this as u32).wrapping_add(0x8), vt);
        c.g[V0][0] = this;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Front-end state constructors.

/// `void* cFEStateOptionsSound_ctor(void* self)` (0x18a258): UI state with
/// class id 0x1d, its vtable, and the global "sound changed" flag cleared.
pub fn c_fe_state_options_sound_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_STATE_CTOR_2.call(c, 0x0018_a26c)?;
        let this = crate::abi::r32(c, S0);
        c.g[V0][0] = 0x1d;
        c.g[V1][0] = 0x47_0000;
        Addr(p(this, 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = sx(crate::abi::r32(c, V1).wrapping_sub(0x39e0));
        Addr(p(this, 8)).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        Addr(crate::abi::r32(c, GP).wrapping_sub(0x1c1c)).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void* cFEStateOptionsHUD_ctor(void* self)` (0x18c0e8): class id 0x1e.
pub fn c_fe_state_options_hud_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_STATE_CTOR_2.call(c, 0x0018_c0fc)?;
        let this = crate::abi::r32(c, S0);
        c.g[V0][0] = 0x1e;
        c.g[V1][0] = 0x47_0000;
        Addr(p(this, 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = sx(crate::abi::r32(c, V1).wrapping_sub(0x3ab0));
        Addr(p(this, 8)).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        Addr(crate::abi::r32(c, GP).wrapping_sub(0x1c18)).sw(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void* cFEStateOptionsSaveLoad_ctor(void* self)` (0x18d240): class id
/// 0x20, `+0x48` (the "scanning" flag) cleared.
pub fn c_fe_state_options_save_load_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::UI_STATE_CTOR_2.call(c, 0x0018_d254)?;
        let this = crate::abi::r32(c, S0);
        c.g[V0][0] = 0x20;
        c.g[V1][0] = 0x47_0000;
        Addr(p(this, 0xc)).sw(c, c.g[V0][0]);
        c.g[V1][0] = sx(crate::abi::r32(c, V1).wrapping_sub(0x3c50));
        Addr(p(this, 8)).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(p(this, 0x48)).sb(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `cFEStateOptionsController* cFEStateOptionsController_ctor(self)`
/// (0x18c658): UI state constructor, then the options-controller vtable and
/// two cleared fields; clears the "options open" global.
pub fn fe_state_options_controller_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, callee::UI_STATE_CTOR_3, 0x0018_c66c)?;
        let this = r32(c, S0);
        c.g[V0][0] = 0x1f;
        const_reg(c, V1, 0x47_0000, -0x3b80);
        sw(c, this, 0x0c, c.g[V0][0]);
        sw(c, this, 0x08, c.g[V1][0]);
        c.g[V0][0] = c.g[S0][0];
        c.sb(p_2(this, 0x50), 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        sw(c, gp(c, -0x1c14), 0, 0);
        leave(c, 0x20);
        Ok(())
    });
}


/// `void cFEStateOptionsController_Shutdown(cFEStateOptionsController* s)`
/// (0x18c8b8): turn both vibration channels off and deactivate the state.
pub fn cfe_state_options_controller_shutdown(c: &mut Cpu) {
    c.cycles += 22;
    let frame = Frame::open(c, 0x30);
    c.g[A1][0] = 0;
    frame.save_fpr(c, 20, 0x20);
    frame.save(c, S0, 0x10);
    c.mtc1(0, 20);
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.mov_s(12, 20);
    c.mov_s(13, 20);
    if callee::CFE_STATE_OPTIONS_CONTROLLER_SET_VIBRATION
        .call(c, 0x0018_c8e0)
        .is_err()
    {
        return;
    }
    c.mov_s(12, 20);
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = 1;
    c.mov_s(13, 12);
    if callee::CFE_STATE_OPTIONS_CONTROLLER_SET_VIBRATION
        .call(c, 0x0018_c8f4)
        .is_err()
    {
        return;
    }
    c.g[A0][0] = c.g[S0][0];
    if call_ptr(c, 0x0039_e4a0, 0x0018_c8fc).is_err() {
        return;
    }
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 20, 0x20);
    frame.close(c);
}


// ---------------------------------------------------------------------------
// Small helpers.

/// A `gp`-relative global address.
#[inline(always)]
pub(crate) fn gpa(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}


// ---------------------------------------------------------------------------
// Front-end option screens (fe/feoptions.cpp).

/// `int cFEStateOptions_HandleInput(cFEStateOptions* self, FEUIInput* input)`
/// (0x188c58): latch the raw pad byte at `input+0x4` into the global input
/// word (`gp+0x394`) and return 0.
pub fn cfe_state_options_handle_input(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = Addr(r32(c, A1).wrapping_add(0x4)).lbu(c);
    c.g[V0][0] = 0;
    c.sw(gpa(c, 0x394), c.g[V1][0]);
    abi::ret(c);
}


/// `int cFEStateOptionsGame_OnWidgetNotify(void*, int, int id)`
/// (0x189958): the game-options page handles the "back" widget (9) with
/// 0x100 and everything else with 0x101.
pub fn cfe_state_options_game_on_widget_notify(c: &mut Cpu) {
    c.cycles += 5;
    c.g[A2][0] ^= 0x9;
    c.g[V1][0] = 0x100;
    c.g[V0][0] = 0x101;
    if c.g[A2][0] == 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    abi::ret(c);
}


/// `int cFEStateOptionsGame_v21(void*, int, int id)` (0x189970): 0x100 when
/// an id was passed, 0 otherwise.
pub fn cfe_state_options_game_v21(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x100;
    if c.g[A2][0] == 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}


/// `int cFEStateOptionsSound_OnWidgetNotify(void*, int, int id)`
/// (0x18a890): id 6 is ignored (0), id 9 is handled (0x100), anything else
/// is not for this page (0x101).
pub fn cfe_state_options_sound_on_widget_notify(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x6;
    let is6 = c.g[A2][0] == c.g[V0][0];
    c.g[V0][0] = 0x9;
    if is6 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    let not_back = c.g[A2][0] != c.g[V0][0];
    c.g[V0][0] = 0x100;
    if not_back {
        c.cycles += 2;
        c.g[V0][0] = 0x101;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    abi::ret(c);
}


/// `int cFEStateOptionsSound_v21(void*, int, int id)` (0x18a8c0): 0x100 when
/// an id was passed, 0 otherwise.
pub fn cfe_state_options_sound_v21(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x100;
    if c.g[A2][0] == 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}


/// `int cFEStateOptionsHUD_OnWidgetNotify(void*, int, int id)` (0x18c270):
/// as [`cfe_state_options_sound_on_widget_notify`] for the HUD page.
pub fn cfe_state_options_hud_on_widget_notify(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x6;
    let is6 = c.g[A2][0] == c.g[V0][0];
    c.g[V0][0] = 0x9;
    if is6 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    let not_back = c.g[A2][0] != c.g[V0][0];
    c.g[V0][0] = 0x100;
    if not_back {
        c.cycles += 2;
        c.g[V0][0] = 0x101;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    abi::ret(c);
}


/// `int cFEStateOptionsController_v21(void*, int, int id)` (0x18cdb0):
/// 0x100 when an id was passed, 0 otherwise.
pub fn cfe_state_options_controller_v21(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x100;
    if c.g[A2][0] == 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}


/// `int cFEStateOptionsSaveLoad_OnWidgetNotify(void*, int, int key)`
/// (0x18d438): keys 8 and 9 are handled (0x100), everything else ignored
/// (0x101); the two comparisons are done as unsigned `< 10` / `< 8`.
pub fn cfe_state_options_save_load_on_widget_notify(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = (c.g[A2][0] < 0xau64) as u64;
    let out_of_range = c.g[V1][0] == 0;
    c.g[V0][0] = 0x101;
    if out_of_range {
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 4;
    c.g[A0][0] = (c.g[A2][0] < 0x8u64) as u64;
    c.g[V1][0] = 0x101;
    c.g[V0][0] = 0x100;
    if c.g[A0][0] != 0 {
        c.g[V0][0] = c.g[V1][0];
    }
    c.cycles += 2;
    abi::ret(c);
}


/// `cFEStateOptionsSaveLoad_Init(cFEStateOptionsSaveLoad* self)` (0x18d280).
pub fn c_fe_state_options_save_load_init(c: &mut Cpu) {
    guest(c, |c| {
        fe_state_screen_init(c, 0x0045_e0c0, [0x0018_d2a4, 0x0018_d2b8, 0x0018_d2d0])
    })
}


/// `cFEStateOptionsHUD::Init(self)` (0x18c128): create the options HUD screen,
/// start it and refresh the selected row.
pub fn c_fe_state_options_hud_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x460000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fc8));
        c.g[S0][0] = Addr(p(c.g[S1][0] as u32, 0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x0018_c14c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x0018_c160)?;
        let missing = c.g[V0][0] == 0;
        Addr(p(c.g[S1][0] as u32, 0x40)).sw(c, c.g[V0][0]);
        if !missing {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x0018_c178)?;
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0018_c478, 0x0018_c180)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateOptions_Init(cFEStateOptions* s)` (0x188800): create the
/// options screen from the hash name and start its frame.
pub fn c_fe_state_options_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        sqsp(c, 17, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        sqsp(c, 16, 0x20);
        c.g[A0][0] = 0x0045_dd30;
        c.g[S0][0] = Addr(p_3(r32(c, S1), 0x10)).lw(c);
        call::call_ptr(c, 0x0031_7670, 0x0018_8824)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call::call_ptr(c, 0x0039_7cc0, 0x0018_8838)?;
        let no_screen = c.g[V0][0] == 0;
        Addr(p_3(r32(c, S1), 0x40)).sw(c, c.g[V0][0]);
        if !no_screen {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call::call_ptr(c, 0x0039_c870, 0x0018_8850)?;
        }
        c.cycles += 8;
        c.g[V0][0] = 1;
        Addr(p_3(r32(c, S1), 0x48)).sw(c, 0);
        Addr(p_3(r32(c, S1), 0x4c)).sw(c, c.g[V0][0]);
        lqsp(c, 16, 0x20);
        lqsp(c, 17, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    });
}


/// `cFEStateOptionsDeviceSelect_ctor` (0x18D7F0): construct the device-select
/// options state, remembering its two handlers and clearing the slot state.
pub fn c_fe_state_options_device_select_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[T0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A3][0];
        callee::MEM_CARD_BASE_CTOR.call(c, 0x0018_d814)?;
        c.g[V1][0] = 0x47_0000;
        c.g[V0][0] = 0x21;
        c.g[V1][0] = low(c, V1).wrapping_sub(0x3dd8);
        sw_at(c, S2, 0x230, c.g[S0][0]);
        sw_at(c, S2, 0x8, c.g[V1][0]);
        sw_at(c, S2, 0xc, c.g[V0][0]);
        sw_at(c, S2, 0x22c, c.g[S1][0]);
        sw_at(c, S2, 0x234, 0);
        let no_handler = c.g[S1][0] == 0;
        sw_at(c, S2, 0x238, 0);
        if no_handler {
            c.cycles += 3;
            c.g[A0][0] = Addr(gp(c, -0x848)).lw(c);
            call_ptr(c, addr::WORLD_GET, 0x0018_d844)?;
            sw_at(c, V0, 0x11c, 0);
        }
        c.cycles += 7;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateOptionsSaveLoad_OnRender(self)` (0x18d3c0): when the options
/// screen is live, reveal the two option widgets found by name hash.
pub fn c_fe_state_options_save_load_on_render(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[SP][0] = sx(low_2(c, SP).wrapping_sub(0x20));
        c.g[V0][0] = Addr(gpo(c, -0x1718)).lw(c);
        c.sq(16, sp_2(c).wrapping_add(0x10));
        c.sd(sp_2(c), c.g[RA][0]);
        let off = c.g[V0][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        if off {
            c.cycles += 1;
            c.lq(16, sp_2(c).wrapping_add(0x10));
        } else {
            c.cycles += 8;
            c.g[A0][0] = 0x46_0000;
            c.g[A0][0] = sx(low_2(c, A0).wrapping_sub(0x1f10));
            call_ptr(c, callee::STR_HASH_2, 0x0018_d3e4)?;
            c.g[A0][0] = lw_r(c, S0, 0x40);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, callee::C_UI_SCREEN_GET_OBJECT_BY_HASH_NAME, 0x0018_d3f0)?;
            if c.g[V0][0] != 0 {
                c.cycles += 3;
                c.g[A0][0] = c.g[V0][0];
                callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF.call(c, 0x0018_d400)?;
            }
            c.g[A0][0] = 0x46_0000;
            c.cycles += 7;
            c.g[A0][0] = sx(low_2(c, A0).wrapping_sub(0x1f00));
            call_ptr(c, callee::STR_HASH_2, 0x0018_d40c)?;
            c.g[A0][0] = lw_r(c, S0, 0x40);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, callee::C_UI_SCREEN_GET_OBJECT_BY_HASH_NAME, 0x0018_d418)?;
            let none = c.g[V0][0] == 0;
            c.lq(16, sp_2(c).wrapping_add(0x10));
            if none {
                c.cycles += 3;
                c.g[RA][0] = Addr(sp_2(c)).ld(c);
                let t = low_2(c, RA);
                c.g[SP][0] = sx(low_2(c, SP).wrapping_add(0x20));
                c.pc = t;
                return Ok(());
            }
            c.cycles += 2;
            c.g[A0][0] = c.g[V0][0];
            callee::UI_OBJECT_SHOW_CLAMP_ALPHA_HALF.call(c, 0x0018_d428)?;
            c.cycles += 1;
            c.lq(16, sp_2(c).wrapping_add(0x10));
        }
        c.cycles += 3;
        c.g[RA][0] = Addr(sp_2(c)).ld(c);
        let t = low_2(c, RA);
        c.g[SP][0] = sx(low_2(c, SP).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Script engine teardown.

/// `void cFEStateOptions_OpenResetPopup(self)` (0x188e58): allocate and build
/// the reset-options popup screen object, add it to the engine's screen list
/// and arm the global "reset" flags.
pub fn c_fe_state_options_open_reset_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x30);
        let self_ = abi::arg(c, 0);
        c.g[A1][0] = 0x0046_0000;
        frame.save(c, S1, 0x10);
        c.g[A3][0] = 0;
        c.g[S1][0] = sx(self_);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x2298));
        c.g[A0][0] = 0x4c;
        c.g[A2][0] = 0x100;
        call_ptr(c, callee::MEM_ALLOC, 0x0018_8e84)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.lw(at(c, S1, 0x10));
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        call_ptr(c, callee::UI_OBJECT_CTOR_2, 0x0018_8e98)?;
        c.g[A0][0] = c.lw(at(c, S1, 0x10));
        c.g[V0][0] = 0x0047_0000;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x4c30));
        c.g[A1][0] = c.g[S0][0];
        c.sw(at(c, S0, 8), c.g[V0][0]);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x18));
        c.sb(at(c, S0, 0x48), 0);
        call_ptr(c, callee::UI_OBJECT_ADD_CHILD, 0x0018_8eb8)?;
        c.g[V0][0] = 1;
        c.sw(gp(c, -0x1c18), 0);
        c.sw(gp(c, -0x1c10), c.g[V0][0]);
        c.sw(gp(c, -0x1c14), 0);
        c.sw(gp(c, -0x1c1c), 0);
        c.sw(gp(c, -0x1c20), 0);
        c.sw(at(c, S1, 0x4c), 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateOptionsDeviceSelect_v33(self)` (0x18e478): open the device
/// select popup once, with the mode selected from the four option flags.
pub fn cfe_state_options_device_select_v33(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw_2(c, S0, 0x1ac);
        let done = c.g[V0][0] != 0;
        c.g[V0][0] = 1;
        if !done {
            c.cycles += 21;
            c.g[V1][0] = lw_2(c, S0, 0x10);
            c.g[T2][0] = 2;
            c.g[A2][0] = lw_2(c, S0, 0x1bc);
            c.g[A1][0] = c.g[S0][0];
            c.g[T1][0] = Addr(c.g[V1][0] as u32).lw(c);
            c.g[V0][0] = lw_2(c, S0, 0x1e4);
            c.g[A2][0] = c.g[A2][0] ^ 1;
            c.g[T0][0] = lw_2(c, T1, 0x4);
            c.g[A2][0] = (c.g[A2][0] < 1) as u64;
            c.g[V0][0] = c.g[V0][0] ^ 1;
            c.g[V1][0] = lw_2(c, S0, 0x1b4);
            if c.g[V0][0] == 0 {
                c.g[A2][0] = 0;
            }
            c.g[A3][0] = lw_2(c, S0, 0x230);
            c.g[A0][0] = Addr(c.g[T0][0] as u32).off(0x28).lh(c);
            if c.g[V1][0] == 0 {
                c.g[A2][0] = 0;
            }
            c.g[V0][0] = lw_2(c, T0, 0x2c);
            if c.g[A3][0] != 0 {
                c.g[A2][0] = c.g[T2][0];
            }
            c.g[A0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V0][0] as u32, 0x0018_e4e0)?;
            let created = c.g[V0][0] == 0;
            c.g[A1][0] = c.g[V0][0];
            if !created {
                c.cycles += 3;
                c.g[A0][0] = lw_2(c, S0, 0x10);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                call_ptr(c, table_callee::UI_SCREEN_ADD_OBJECT, 0x0018_e4f4)?;
            }
        }
        c.cycles += 5;
        frame.restore_ra(c, 0);
        Addr(c.g[S0][0] as u32).off(0x1ac).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}


/// `int cFEStateOptions_OnWidgetNotify(cFEState*, void* widget, int event,
/// int index)` (0x188c68): map the widget events 6/7 onto the page's flag
/// word, or return 0x100/0x101 for the other ids.
pub fn c_fe_state_options_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A0][0];
        c.g[V0][0] = (c.g[S1][0] < 0xa) as u64;
        let big = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        let mut done = false;
        if !big {
            c.cycles += 3;
            c.g[V0][0] = (c.g[S1][0] < 8) as u64;
            let low = c.g[V0][0] != 0;
            c.g[V0][0] = 6;
            if !low {
                c.cycles += 2;
                c.g[V0][0] = 0x100;
                done = true;
            }
        } else {
            c.cycles += 1;
            c.g[V0][0] = 6;
        }
        if !done {
            c.cycles += 2;
            let not_six = c.g[S1][0] != c.g[V0][0];
            c.g[V0][0] = 7;
            let mut hit = false;
            if !not_six {
                c.cycles += 9;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x74));
                c.g[A1][0] = c.g[A3][0];
                call_ptr(c, 0x0039_7870, 0x0018_8cb8)?; // UIList_GetAt
                c.g[A0][0] = 0x004a_0000;
                c.g[S0][0] = lw_3(c, p(c.g[V0][0] as u32, 0x38));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1530));
                call_ptr(c, 0x0031_7670, 0x0018_8cc8)?; // StrHash
                if c.g[S0][0] == c.g[V0][0] {
                    c.g[A0][0] = lw_3(c, p(c.g[S2][0] as u32, 0x4c));
                    hit = true;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = 7;
                }
            }
            if hit {
                c.cycles += 3;
                c.g[V1][0] = 0x101;
                c.g[V0][0] = 1;
                if lw_3(c, c.g[A0][0] as u32) != 0 {
                    c.g[V0][0] = c.g[V1][0];
                }
            } else {
                c.cycles += 2;
                if c.g[S1][0] != c.g[V0][0] {
                    c.g[V0][0] = 0x101;
                } else {
                    c.cycles += 1;
                    c.g[A0][0] = lw_3(c, p(c.g[S2][0] as u32, 0x4c));
                    c.cycles += 3;
                    c.g[V1][0] = 0x101;
                    c.g[V0][0] = 1;
                    if lw_3(c, c.g[A0][0] as u32) != 0 {
                        c.g[V0][0] = c.g[V1][0];
                    }
                }
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


/// `void cFEStateOptionsDeviceSelect_v38(self, flag)` (0x18e2c0): if the
/// state's object exists, tell both device-select items the new selected
/// flag and store the inverted flag.
pub fn cfe_state_options_device_select_v38(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x40);
        c.cycles += 9;
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, at_3(c.g[S1][0], 0x22c));
        let t = c.g[V0][0] == 0;
        c.g[S2][0] = c.g[A1][0];
        if !t {
            c.cycles += 25;
            c.g[A0][0] = sx(0x004a_0000);
            c.g[S0][0] = c.g[S2][0] ^ 1;
            c.g[RA][0] = sx(0x0018_e2f4);
            c.g[A0][0] = sx(0x004a_1458);
            call_ptr(c, 0x0031_7670, 0x0018_e2f4)?;
            c.g[A0][0] = lw(c, at_3(c.g[S1][0], 0x40));
            c.g[RA][0] = sx(0x0018_e300);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0018_e300)?;
            c.g[V1][0] = lw(c, at_3(c.g[V0][0], 8));
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = lh(c, at_3(c.g[V1][0], 0x48));
            c.g[A2][0] = lw(c, at_3(c.g[V1][0], 0x4c));
            let target = c.g[A2][0] as u32;
            c.g[RA][0] = sx(0x0018_e318);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0018_e318)?;
            c.g[A0][0] = sx(0x004a_0000);
            c.g[RA][0] = sx(0x0018_e324);
            c.g[A0][0] = sx(0x004a_1460);
            call_ptr(c, 0x0031_7670, 0x0018_e324)?;
            c.g[A0][0] = lw(c, at_3(c.g[S1][0], 0x40));
            c.g[RA][0] = sx(0x0018_e330);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x0018_e330)?;
            c.g[V1][0] = lw(c, at_3(c.g[V0][0], 8));
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = lh(c, at_3(c.g[V1][0], 0x48));
            c.g[A2][0] = lw(c, at_3(c.g[V1][0], 0x4c));
            let target = c.g[A2][0] as u32;
            c.g[RA][0] = sx(0x0018_e348);
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0018_e348)?;
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S2][0] ^ 1;
        frame.restore(c, S0, 0x30);
        Addr(at_3(c.g[S1][0], 0x220)).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// `cFEStateOptionsController_Update` (0x18d058): update the screen, then tick
// down the two vibration timers; an expired timer clears its bit and tells
// the controller to stop vibrating that motor. The returned bits select the
// motors.
pub fn cfe_state_options_controller_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039e510, 0x0018_d06c)?;
        c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x50));
        c.g[V0][0] = c.g[V1][0] & 0x1;
        let no_first = c.g[V0][0] == 0;
        c.g[V0][0] = c.g[V1][0] & 0x2;
        if !no_first {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x48));
            let expired = (c.g[V0][0] as i64) <= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            if !expired {
                c.cycles += 2;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), c.g[V0][0]);
            } else {
                c.cycles += 8;
                c.mtc1(0, 12);
                c.g[V0][0] = c.g[V1][0] & 0xfe;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
                c.g[A0][0] = c.g[S0][0];
                c.sb((c.g[S0][0] as u32).wrapping_add(0x50), c.g[V0][0]);
                c.g[A1][0] = 0u64;
                c.mov_s(13, 12);
                callee::OPTIONS_SET_VIBRATION.call(c, 0x0018_d0b0)?;
            }
            c.cycles += 2;
            c.g[V1][0] = c.lbu((c.g[S0][0] as u32).wrapping_add(0x50));
            c.g[V0][0] = c.g[V1][0] & 0x2;
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.lq(16, frame.at(c, 0x10));
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4c));
            let expired = (c.g[V0][0] as i64) <= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
            if !expired {
                c.cycles += 2;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), c.g[V0][0]);
            } else {
                c.cycles += 8;
                c.mtc1(0, 12);
                c.g[V0][0] = c.g[V1][0] & 0xfd;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
                c.g[A0][0] = c.g[S0][0];
                c.sb((c.g[S0][0] as u32).wrapping_add(0x50), c.g[V0][0]);
                c.g[A1][0] = 0x1;
                c.mov_s(13, 12);
                callee::OPTIONS_SET_VIBRATION.call(c, 0x0018_d0f4)?;
            }
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x10));
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cFEStateOptionsHUD_OnChildEvent(cFEStateOptionsHUD* s, void* child,
/// int event)` (0x18c1b8): apply the option row's change when the HUD's child
/// screen reports the selection event.
pub fn fe_state_options_hud_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x50);
        'outer: {
            'done: {
                frame.save(c, S0, 0x40);
                frame.save(c, S1, 0x30);
                let event = r(c, A2);
                c.g[S0][0] = sx(event);
                frame.save(c, S2, 0x20);
                c.g[S1][0] = c.g[A1][0];
                let this = r(c, A0);
                frame.save(c, S3, 0x10);
                frame.save_ra(c, 0);
                c.g[S3][0] = sx(this);
                call_ptr(c, 0x0014_dc80, 0x0018_c1e0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0018_c1ec);
                c.g[A1][0] = 4;
                call_ptr(c, callee::BE_GET_INTERFACE, 0x0018_c1ec)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[V0][0] = 0x15;
                let open = c.g[S0][0] == c.g[V0][0];
                c.g[V0][0] = 0x16;
                if open {
                    break 'done;
                }
                c.cycles += 2;
                let not_close = c.g[S0][0] != c.g[V0][0];
                frame.restore(c, 16, 0x40);
                if not_close {
                    break 'outer;
                }
                c.cycles += 6;
                c.g[A0][0] = sx(0x0046_0000);
                c.g[S0][0] = Addr(p(r(c, S1), 0xc)).lw(c);
                c.g[RA][0] = sx(0x0018_c214);
                c.g[A0][0] = sx(0x0045_dd20);
                call_ptr(c, callee::FE_STRING_HASH, 0x0018_c214)?;
                let differs = c.g[S0][0] != c.g[V0][0];
                frame.restore(c, 16, 0x40);
                if differs {
                    break 'outer;
                }
                c.cycles += 8;
                c.g[V1][0] = Addr(p(r(c, S1), 8)).lw(c);
                c.g[A1][0] = 2;
                c.g[A0][0] = Addr(p(r(c, V1), 0xb8)).lh(c);
                c.g[V0][0] = Addr(p(r(c, V1), 0xbc)).lw(c);
                let target = r(c, V0);
                c.g[RA][0] = sx(0x0018_c234);
                c.g[A0][0] = sx(p(r(c, S1), r(c, A0)));
                call_ptr(c, target, 0x0018_c234)?;
                let empty = c.g[V0][0] == 0;
                frame.restore(c, 16, 0x40);
                if empty {
                    break 'outer;
                }
                c.cycles += 6;
                c.g[RA][0] = sx(0x0018_c244);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0014_f600, 0x0018_c244)?;
                c.g[RA][0] = sx(0x0018_c24c);
                c.g[A0][0] = c.g[S3][0];
                callee::OPTIONS_HUD_REFRESH_SELECTION.call(c, 0x0018_c24c)?;
                c.g[V0][0] = 1;
                Addr(gp_2(c, -0x1c18)).sw(c, c.g[V0][0]);
            }
            c.cycles += 1;
            frame.restore(c, 16, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int cFEStateOptionsDeviceSelect_OnWidgetNotify(self, void *widget,
/// int id)` (0x18ddd0): map a widget id to the menu's return code (0x100 for
/// "previous", 0x101 otherwise); for ids 6/8 it also notifies the selected
/// device entry.
pub fn cfe_state_options_device_select_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[A0][0] = sx(world(c));
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        call_ptr(c, 0x0022_7f80, 0x0018_ddf4)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V0][0] = 0x8;
        let is8 = c.g[S0][0] == c.g[V0][0];
        c.g[V0][0] = (c.g[S0][0] < 9) as u64;
        if is8 {
            c.cycles += 3;
            c.g[V0][0] = Addr(p_4(c.g[S2][0], 0x22c)).lw(c);
            c.g[A2][0] = Addr(p_4(c.g[S1][0], 0x434)).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 8;
                c.g[A1][0] = Addr(p_4(c.g[S1][0], 0x428)).lw(c);
                c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[A0][0] = Addr(p_4(c.g[V1][0], 0x1b0)).lh(c);
                c.g[V0][0] = Addr(p_4(c.g[V1][0], 0x1b4)).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0018_de54)?;
                let nonzero = c.g[V0][0] != 0;
                c.g[V0][0] = 0x101;
                if nonzero {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x30);
                    c.cycles += 5;
                    frame.restore(c, S1, 0x20);
                    frame.restore(c, S2, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                c.cycles += 1;
                c.g[A2][0] = Addr(p_4(c.g[S1][0], 0x434)).lw(c);
            } else {
                c.g[A2][0] = Addr(p_4(c.g[S1][0], 0x434)).lw(c);
            }
            // The shared device-entry notification (label b_18de60).
            c.cycles += 8;
            c.g[A1][0] = Addr(p_4(c.g[S1][0], 0x428)).lw(c);
            c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
            c.g[A0][0] = Addr(p_4(c.g[V1][0], 0x1b0)).lh(c);
            c.g[V0][0] = Addr(p_4(c.g[V1][0], 0x1b4)).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0018_de78)?;
            let nonzero = c.g[V0][0] != 0;
            c.g[V0][0] = 0x101;
            if nonzero {
                c.cycles += 1;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0x100;
            }
            frame.restore(c, S0, 0x30);
        } else if c.g[S0][0] >= 9 {
            c.cycles += 2;
            c.cycles += 3;
            if c.g[S0][0] == 9 {
                c.cycles += 2;
                c.g[V0][0] = 0x100;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0x101;
            }
            frame.restore(c, S0, 0x30);
        } else {
            c.cycles += 2;
            if c.g[S0][0] == 6 {
                c.cycles += 2;
                c.cycles += 1;
                c.g[A2][0] = Addr(p_4(c.g[S1][0], 0x434)).lw(c);
                c.cycles += 8;
                c.g[A1][0] = Addr(p_4(c.g[S1][0], 0x428)).lw(c);
                c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[A0][0] = Addr(p_4(c.g[V1][0], 0x1b0)).lh(c);
                c.g[V0][0] = Addr(p_4(c.g[V1][0], 0x1b4)).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0018_de78)?;
                let nonzero = c.g[V0][0] != 0;
                c.g[V0][0] = 0x101;
                if nonzero {
                    c.cycles += 1;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0x100;
                }
                frame.restore(c, S0, 0x30);
            } else {
                c.cycles += 2;
                c.cycles += 2;
                frame.restore(c, S0, 0x30);
            }
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Screen-object helpers.
//
// The front-end states look widgets up by hashed name and then call a method
// on the found object through its vtable (`lh vt+0x48` delta, `lw vt+0x4c`
// function).

/// `int StrHash(const char*)`.
pub(crate) const STR_HASH: u32 = 0x0031_7670;

/// `UIObject *UIScreen_FindObject(UIScreen*, int hash)`.
pub(crate) const UI_SCREEN_FIND_OBJECT: u32 = 0x0039_d860;


/// `StrHash(name)`, then `UIScreen_FindObject(*(this+0x40), hash)`; the found
/// object is left in `v0`.
pub(crate) fn screen_object(c: &mut Cpu, this: u32, name: u32, ret_hash: u32, ret_find: u32) -> Flow {
    c.g[A0][0] = name as u64;
    call_ptr(c, STR_HASH, ret_hash)?;
    c.g[A0][0] = Addr(this).off(0x40).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, UI_SCREEN_FIND_OBJECT, ret_find)
}


/// Call the method at vtable slot `+0x4c` of `*(obj+8)` with `a1 = arg`
/// (the function pointer is fetched into `a2`, as the original does).
pub(crate) fn call_object_a2(c: &mut Cpu, obj: u64, arg: u64, ret: u32) -> Flow {
    c.g[V1][0] = Addr(obj as u32).off(8).lw(c);
    c.g[A1][0] = arg;
    c.g[A0][0] = Addr(r32(c, V1)).off(0x48).lh(c);
    c.g[A2][0] = Addr(r32(c, V1)).off(0x4c).lw(c);
    let func = c.g[A2][0] as u32;
    c.g[A0][0] = sx((obj as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, func, ret)
}


// ---------------------------------------------------------------------------
// cFEStateOptionsDeviceSelect_v37 / cFEStateProfileLoad_v37.

/// The UIState's frame (0x40, `s0`/`s1`/`s2` at +0x30/+0x20/+0x10).
pub(crate) fn ui_state_frame(c: &mut Cpu) -> Frame {
    let frame = Frame::open(c, 0x40);
    frame.save(c, S0, 0x30);
    frame.save(c, S2, 0x10);
    frame.save(c, S1, 0x20);
    frame.save_ra(c, 0);
    frame
}


pub(crate) fn ui_state_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 6;
    frame.restore(c, S0, 0x30);
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}


/// `void cFEStateOptionsDeviceSelect_v37(FEState *s, int selected)` (0x18e178):
/// re-apply the state `selected` to the three device/controller widgets and
/// record it in `+0x234`.
pub fn c_fe_state_options_device_select_v37(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = ui_state_frame(c);
        c.g[S0][0] = abi::arg64(c, 0);
        c.g[S2][0] = abi::arg64(c, 1);
        c.g[V0][0] = Addr(r32(c, S0)).off(0x234).lw(c);
        let same = c.g[V0][0] == c.g[S2][0];
        c.g[S1][0] = c.g[S2][0] ^ 1;
        let this = r32(c, S0);
        let arg = c.g[S1][0];
        if !same {
            c.cycles += 15;
            screen_object(c, this, 0x004a_15e0, 0x0018_e1ac, 0x0018_e1b8)?;
            let obj = c.g[V0][0];
            call_object_a2(c, obj, arg, 0x0018_e1d0)?;

            c.g[V0][0] = Addr(this).off(0x22c).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 12;
                screen_object(c, this, 0x004a_15d8, 0x0018_e27c, 0x0018_e288)?;
                let obj = c.g[V0][0];
                call_object_a2(c, obj, arg, 0x0018_e2a0)?;
                Addr(this).off(0x234).sw(c, c.g[S2][0]);
            } else {
                c.cycles += 38;
                screen_object(c, this, 0x004a_15d0, 0x0018_e1e8, 0x0018_e1f4)?;
                let obj = c.g[V0][0];
                call_object_a2(c, obj, arg, 0x0018_e20c)?;
                screen_object(c, this, 0x004a_1458, 0x0018_e218, 0x0018_e224)?;
                let obj = c.g[V0][0];
                call_object_a2(c, obj, arg, 0x0018_e23c)?;
                screen_object(c, this, 0x004a_1460, 0x0018_e248, 0x0018_e254)?;
                let obj = c.g[V0][0];
                call_object_a2(c, obj, arg, 0x0018_e26c)?;
                Addr(this).off(0x234).sw(c, c.g[S2][0]);
            }
        }
        ui_state_epilogue(c, frame);
        Ok(())
    })
}




/// `jalr` through a vtable entry (the target is data).
#[inline]
pub(crate) fn call_vtable(c: &mut Cpu, target: u32, ret: u32) -> Flow {
    c.g[RA][0] = sx(ret);
    c.pc = target;
    call_indirect(c, target);
    returned(c, ret)
}



/// `void cFEStateOptionsController_OnWidgetAction(self, widget, event)`
/// (0x18ca10): the options screen's widget actions. Event 7 closes a popup,
/// event 6 opens the config popup, event 9 toggles vibration (ids
/// `D_004A1568`/`D_004A1570`, timers `+0x48`/`+0x4c`, flags `+0x50` bits 0/1)
/// and switches the config text (`D_004A15C8`/`D_004A1578`).
pub fn fe_options_controller_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        'b_18cccc: {
        'b_18ccc8: {
        'b_18ccc4: {
        c.cycles += 10;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[S0][0] = c.g[A2][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.g[A1][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[S2][0] = c.g[A0][0];
        let t = c.g[S1][0] == 0u64;
        c.sd((c.g[SP][0] as u32), c.g[RA][0]);
        if t {
        break 'b_18ccc4;
        } else {
        'b_18cb34: {
        'b_18cadc: {
        'b_18ca88: {
        c.cycles += 9;
        c.g[RA][0] = sx(0x0018ca40);
        call_vtable(c, 0x0014dc80, 0x0018ca40)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0018ca4c);
        c.g[A1][0] = 0x1;
        call_vtable(c, 0x0014dd58, 0x0018ca4c)?;
        c.g[S3][0] = c.g[V0][0];
        c.g[V0][0] = 0x7;
        let t = c.g[S0][0] == c.g[V0][0];
        c.g[V0][0] = (c.g[S0][0] < 0x8u64) as u64;
        if t {
        break 'b_18ca88;
        } else {
        'b_18ca74: {
        c.cycles += 2;
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0x6;
        if t {
        break 'b_18ca74;
        } else {
        c.cycles += 2;
        let t = c.g[S0][0] == c.g[V0][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        if t {
        break 'b_18cadc;
        } else {
        c.cycles += 2;
        let t = true;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        if t {
        break 'b_18cccc;
        } else {
        break 'b_18ca74;
        }
        }
        }
        }
        c.cycles += 3;
        c.g[V0][0] = 0x9;
        let t = c.g[S0][0] == c.g[V0][0];
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        if t {
        break 'b_18cb34;
        } else {
        c.cycles += 2;
        let t = true;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        if t {
        break 'b_18cccc;
        } else {
        break 'b_18ca88;
        }
        }
        }
        }
        c.cycles += 21;
        c.g[A1][0] = 0x460000;
        c.g[A0][0] = 0x4c;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x2298));
        c.g[A2][0] = 0x100;
        c.g[RA][0] = sx(0x0018caa0);
        c.g[A3][0] = 0u64;
        call_vtable(c, 0x00317d70, 0x0018caa0)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x0018cab4);
        c.g[A2][0] = c.g[S2][0];
        call_vtable(c, 0x0039e318, 0x0018cab4)?;
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = 0x470000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4c30));
        c.g[A1][0] = c.g[S0][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        c.g[RA][0] = sx(0x0018cad4);
        c.sb((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
        call_vtable(c, 0x0039f290, 0x0018cad4)?;
        let t = true;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        if t {
        break 'b_18ccc8;
        } else {
        break 'b_18cadc;
        }
        }
        c.cycles += 16;
        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x0018caf0);
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_vtable(c, t, 0x0018caf0)?;
        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        c.g[A3][0] = c.lw((c.g[V0][0] as u32));
        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x28));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x2c));
        let t = c.g[V0][0] as u32;
        c.g[RA][0] = sx(0x0018cb14);
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_vtable(c, t, 0x0018cb14)?;
        let t = c.g[V0][0] == 0u64;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        if t {
        break 'b_18ccc8;
        } else {
        c.cycles += 6;
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0018cb2c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        call_vtable(c, 0x0039f400, 0x0018cb2c)?;
        let t = true;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        if t {
        break 'b_18ccc8;
        } else {
        break 'b_18cb34;
        }
        }
        }
        'b_18ccc0: {
        'b_18ccbc: {
        'b_18cc34: {
        'b_18cc24: {
        'b_18cbb0: {
        c.cycles += 6;
        c.g[A0][0] = 0x4a0000;
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
        c.g[RA][0] = sx(0x0018cb44);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1568));
        call_vtable(c, 0x00317670, 0x0018cb44)?;
        let t = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = 0x4a0000;
        if t {
        break 'b_18cbb0;
        } else {
        'b_18cba4: {
        c.cycles += 11;
        c.g[RA][0] = sx(0x0018cb54);
        c.g[A0][0] = c.g[S1][0];
        call_vtable(c, 0x0039a738, 0x0018cb54)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 0u64;
        c.g[RA][0] = sx(0x0018cb64);
        c.g[A2][0] = (0u64 < c.g[V0][0]) as u64;
        call_vtable(c, 0x00147528, 0x0018cb64)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[RA][0] = sx(0x0018cb70);
        c.g[A1][0] = 0u64;
        call_vtable(c, 0x001474e8, 0x0018cb70)?;
        let t = c.g[V0][0] == 0u64;
        c.g[A0][0] = c.g[S2][0];
        if t {
        break 'b_18cba4;
        } else {
        c.cycles += 11;
        c.g[AT][0] = 0x41200000;
        c.mtc1(1, 12);
        c.g[A1][0] = 0u64;
        c.g[RA][0] = sx(0x0018cb8c);
        c.mov_s(13, 12);
        callee::SET_VIBRATION.call(c, 0x0018cb8c)?;
        c.g[V0][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x50));
        c.g[V1][0] = 0x3c;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x48), c.g[V1][0]);
        c.g[V0][0] = c.g[V0][0] | 0x1;
        let t = true;
        c.sb((c.g[S2][0] as u32).wrapping_add(0x50), c.g[V0][0]);
        if t {
        break 'b_18ccbc;
        } else {
        break 'b_18cba4;
        }
        }
        }
        c.cycles += 3;
        c.mtc1(0, 12);
        let t = true;
        c.g[A1][0] = 0u64;
        if t {
        break 'b_18cc24;
        } else {
        break 'b_18cbb0;
        }
        }
        }
        c.cycles += 5;
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
        c.g[RA][0] = sx(0x0018cbbc);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1570));
        call_vtable(c, 0x00317670, 0x0018cbbc)?;
        let t = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = 0x4a0000;
        if t {
        break 'b_18cc34;
        } else {
        'b_18cc1c: {
        c.cycles += 11;
        c.g[RA][0] = sx(0x0018cbcc);
        c.g[A0][0] = c.g[S1][0];
        call_vtable(c, 0x0039a738, 0x0018cbcc)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = 0x1;
        c.g[RA][0] = sx(0x0018cbdc);
        c.g[A2][0] = (0u64 < c.g[V0][0]) as u64;
        call_vtable(c, 0x00147528, 0x0018cbdc)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[RA][0] = sx(0x0018cbe8);
        c.g[A1][0] = 0x1;
        call_vtable(c, 0x001474e8, 0x0018cbe8)?;
        let t = c.g[V0][0] == 0u64;
        c.g[A0][0] = c.g[S2][0];
        if t {
        break 'b_18cc1c;
        } else {
        c.cycles += 11;
        c.g[AT][0] = 0x41200000;
        c.mtc1(1, 12);
        c.g[A1][0] = 0x1;
        c.g[RA][0] = sx(0x0018cc04);
        c.mov_s(13, 12);
        callee::SET_VIBRATION.call(c, 0x0018cc04)?;
        c.g[V0][0] = c.lbu((c.g[S2][0] as u32).wrapping_add(0x50));
        c.g[V1][0] = 0x3c;
        c.sw((c.g[S2][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
        c.g[V0][0] = c.g[V0][0] | 0x2;
        let t = true;
        c.sb((c.g[S2][0] as u32).wrapping_add(0x50), c.g[V0][0]);
        if t {
        break 'b_18ccbc;
        } else {
        break 'b_18cc1c;
        }
        }
        }
        c.cycles += 2;
        c.mtc1(0, 12);
        c.g[A1][0] = 0x1;
        break 'b_18cc24;
        }
        }
        c.cycles += 4;
        c.g[RA][0] = sx(0x0018cc2c);
        c.mov_s(13, 12);
        callee::SET_VIBRATION.call(c, 0x0018cc2c)?;
        let t = true;
        c.g[V0][0] = 0x1;
        if t {
        break 'b_18ccc0;
        } else {
        break 'b_18cc34;
        }
        }
        'b_18cc7c: {
        c.cycles += 5;
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
        c.g[RA][0] = sx(0x0018cc40);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x15c8));
        call_vtable(c, 0x00317670, 0x0018cc40)?;
        let t = c.g[S0][0] != c.g[V0][0];
        c.g[A0][0] = 0x4a0000;
        if t {
        break 'b_18cc7c;
        } else {
        c.cycles += 13;
        c.g[RA][0] = sx(0x0018cc50);
        c.g[A0][0] = c.g[S1][0];
        call_vtable(c, 0x0039a738, 0x0018cc50)?;
        c.g[A1][0] = 0u64;
        c.g[A0][0] = c.g[S3][0];
        c.g[RA][0] = sx(0x0018cc60);
        c.g[A2][0] = c.g[V0][0];
        call_vtable(c, 0x00147658, 0x0018cc60)?;
        c.g[RA][0] = sx(0x0018cc68);
        c.g[A0][0] = c.g[S1][0];
        call_vtable(c, 0x0039a738, 0x0018cc68)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x0018cc74);
        c.g[A1][0] = (c.g[V0][0] < 0x1u64) as u64;
        callee::SHOW_CONFIG_TEXT.call(c, 0x0018cc74)?;
        let t = true;
        c.g[V0][0] = 0x1;
        if t {
        break 'b_18ccc0;
        } else {
        break 'b_18cc7c;
        }
        }
        }
        c.cycles += 5;
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x38));
        c.g[RA][0] = sx(0x0018cc88);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1578));
        call_vtable(c, 0x00317670, 0x0018cc88)?;
        let t = c.g[S0][0] != c.g[V0][0];
        c.g[V0][0] = 0x1;
        if t {
        break 'b_18ccc0;
        } else {
        c.cycles += 11;
        c.g[RA][0] = sx(0x0018cc98);
        c.g[A0][0] = c.g[S1][0];
        call_vtable(c, 0x0039a738, 0x0018cc98)?;
        c.g[A1][0] = 0x1;
        c.g[A0][0] = c.g[S3][0];
        c.g[RA][0] = sx(0x0018cca8);
        c.g[A2][0] = c.g[V0][0];
        call_vtable(c, 0x00147658, 0x0018cca8)?;
        c.g[RA][0] = sx(0x0018ccb0);
        c.g[A0][0] = c.g[S1][0];
        call_vtable(c, 0x0039a738, 0x0018ccb0)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[RA][0] = sx(0x0018ccbc);
        c.g[A1][0] = (c.g[V0][0] < 0x1u64) as u64;
        callee::SHOW_CONFIG_TEXT.call(c, 0x0018ccbc)?;
        break 'b_18ccbc;
        }
        }
        c.cycles += 1;
        c.g[V0][0] = 0x1;
        break 'b_18ccc0;
        }
        c.cycles += 1;
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1c14), c.g[V0][0]);
        break 'b_18ccc4;
        }
        }
        c.cycles += 1;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        break 'b_18ccc8;
        }
        c.cycles += 1;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        break 'b_18cccc;
        }
        c.cycles += 5;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::check;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes::vtable;
    use crate::difftest::put_words;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fill;
    use crate::difftest::fakes::covered;
    use crate::difftest::check_lohi;
    use crate::difftest::scramble;
    use ssx3_ee as ee;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::fakes;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::SP;
    pub(crate) use crate::game::dirtysock::tags2::tests::be_library_2 as be_library;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY_PTR;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup as setup_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_2 as OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_2 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::w;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_5 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_2 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::ui_init_setup;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::ai::ai::tests::w16;
    pub(crate) use crate::game::camera::cameracontroller::tests::pool as pool_2;
    pub(crate) use crate::game::ai::ai::tests::slot;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::belib_2 as belib;
    pub(crate) use crate::game::dirtysock::tags2::tests::world_2 as world;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_3;
    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_2;
    pub(crate) use crate::game::fe::festatebonusmaterial::tests::run;
    pub(crate) use crate::game::fe::festatebonusmaterial::tests::str_hash;
    pub(crate) use crate::game::fe::feasyncfile::tests::gpat;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_8 as be_library_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_4 as pool_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_5 as world_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_6 as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::IFACE;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_4 as W;
    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH;
    pub(crate) use crate::game::fe::feasyncfile::tests::MEM_HEAP;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::wb;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_9 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_8 as run_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_10 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_10 as pool_allocator;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_2;
    pub(crate) use crate::game::bx::bxstring2::tests::W_2;
    pub(crate) use crate::game::bx::bxstring2::tests::RM;
    pub(crate) use crate::game::bx::bxstring2::tests::RIDERS;
    pub(crate) use crate::game::bx::bxstring2::tests::BE_2 as BE_3;
    pub(crate) use crate::game::bx::bxstring2::tests::DB;
    pub(crate) use crate::game::bx::bxstring2::tests::VTS;
    pub(crate) use crate::game::bx::bxstring2::tests::gp_at;
    pub(crate) use crate::game::bx::bxstring2::tests::base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_15 as SEEDS_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_8 as BE_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_6 as W_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_5;


    // -----------------------------------------------------------------------
    // cFEState constructors.

    pub(crate) fn fe_state_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        c.g[A0][0] = DATA as u64;
        for i in 0..0x120u32 {
            c.bus.write32(DATA + i * 4, r.next_u32());
        }
        // A zeroed class descriptor (the base ctor installs a real one): the
        // refresh helpers then read a null vtable slot and reach the fake.
        c.bus.write32(DATA + 0x8, 0);
        c.bus.write32(DATA + 0x214, match seed % 3 {
            0 => 3,
            1 => r.next_u32(),
            _ => 2,
        });
        c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x1000);
        c.bus.write32(DATA + 0x1000 + 0x84, DATA + 0x2000);
        c.bus.write32(DATA + 0x2000 + 0x84, DATA + 0x3000);
    }


    #[test]
    fn c_fe_state_options_game_ctor_matches() {
        check(ssx3_ee::s02::f_00188ee8__generated, c_fe_state_options_game_ctor, 24, &|c, seed| {
            fe_state_setup(c, seed, 0x188e_e8);
        });
    }


    // --------------------------------------------------------- state ctors

    #[test]
    fn c_fe_state_options_sound_ctor_matches() {
        check(
            ssx3_ee::s02::f_0018a258__generated,
            c_fe_state_options_sound_ctor,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x13, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x3000);
            },
        );
    }


    #[test]
    fn c_fe_state_options_hud_ctor_matches() {
        check(
            ssx3_ee::s02::f_0018c0e8__generated,
            c_fe_state_options_hud_ctor,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x14, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x3000);
            },
        );
    }


    #[test]
    fn c_fe_state_options_save_load_ctor_matches() {
        check(
            ssx3_ee::s02::f_0018d240__generated,
            c_fe_state_options_save_load_ctor,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x15, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x3000);
            },
        );
    }


    #[test]
    fn fe_state_options_controller_ctor_matches() {
        check(
            ssx3_ee::s02::f_0018c658__generated,
            fe_state_options_controller_ctor,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x18c658, 0);
                be_library(c);
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }


    #[test]
    fn cfe_state_options_controller_shutdown_matches() {
        check(
            ssx3_ee::s02::f_0018c8b8__generated,
            cfe_state_options_controller_shutdown,
            100,
            &|c, seed| {
                let _r = machine(c, seed, 0x18c8b8, 3);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.bus.write32(DATA + 0x400, seed % 4);
            },
        );
    }

    /// Scratch objects, tables and hit records.
    pub(crate) const AUX: u32 = DATA + 0x1000;


    pub(crate) fn rng(seed: u32, salt: u64) -> Rng {
        Rng(0x9e37_79b9_7f4a_7c15 ^ ((seed as u64) << 8) ^ salt)
    }


    /// Scrambled registers (so incidental leftovers are compared) and the
    /// game's `gp`.
    pub(crate) fn setup(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = rng(seed, salt);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        r
    }


    // ------------------------------------------------------------------
    // Front-end option screens.

    #[test]
    fn cfe_state_options_handle_input_matches() {
        check(ee::s02::f_00188c58__generated, cfe_state_options_handle_input, 200, &|c, seed| {
            let mut r = setup(c, seed, 23);
            c.g[A1][0] = AUX as u64;
            c.bus.write8(AUX + 4, r.next_u32() as u8);
        });
    }


    #[test]
    fn cfe_state_options_game_on_widget_notify_matches() {
        check(ee::s02::f_00189958__generated, cfe_state_options_game_on_widget_notify, 200, &|c, seed| {
            setup(c, seed, 24);
            c.g[A2][0] = [9u64, 0, 6, 1][(seed % 4) as usize];
        });
    }


    #[test]
    fn cfe_state_options_game_v21_matches() {
        check(ee::s02::f_00189970__generated, cfe_state_options_game_v21, 200, &|c, seed| {
            setup(c, seed, 25);
            c.g[A2][0] = if seed % 2 == 0 { 0 } else { seed as u64 };
        });
    }


    #[test]
    fn cfe_state_options_sound_on_widget_notify_matches() {
        check(ee::s02::f_0018a890__generated, cfe_state_options_sound_on_widget_notify, 200, &|c, seed| {
            setup(c, seed, 26);
            c.g[A2][0] = [6u64, 9, 0, 3][(seed % 4) as usize];
        });
    }


    #[test]
    fn cfe_state_options_sound_v21_matches() {
        check(ee::s02::f_0018a8c0__generated, cfe_state_options_sound_v21, 200, &|c, seed| {
            setup(c, seed, 27);
            c.g[A2][0] = if seed % 2 == 0 { 0 } else { seed as u64 };
        });
    }


    #[test]
    fn cfe_state_options_hud_on_widget_notify_matches() {
        check(ee::s02::f_0018c270__generated, cfe_state_options_hud_on_widget_notify, 200, &|c, seed| {
            setup(c, seed, 28);
            c.g[A2][0] = [6u64, 9, 0, 3][(seed % 4) as usize];
        });
    }


    #[test]
    fn cfe_state_options_controller_v21_matches() {
        check(ee::s02::f_0018cdb0__generated, cfe_state_options_controller_v21, 200, &|c, seed| {
            setup(c, seed, 29);
            c.g[A2][0] = if seed % 2 == 0 { 0 } else { seed as u64 };
        });
    }


    #[test]
    fn cfe_state_options_save_load_on_widget_notify_matches() {
        check(ee::s02::f_0018d438__generated, cfe_state_options_save_load_on_widget_notify, 200, &|c, seed| {
            setup(c, seed, 30);
            c.g[A2][0] = [0u64, 7, 8, 9, 10, 0xffff_ffff][(seed % 6) as usize];
        });
    }


    // ---------------------------------------------------------- FE state screens

    #[test]
    fn c_fe_state_options_save_load_init_matches() {
        check(
            ssx3_ee::s02::f_0018d280__generated,
            c_fe_state_options_save_load_init,
            40,
            &|c, seed| {
                let _r = setup_2(c, seed, 0x18d280);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x10, 0);
                c.bus.write32(DATA + 0x40, seed);
            },
        );
    }


    #[test]
    fn c_fe_state_options_hud_init_matches() {
        check(
            ee::s02::f_0018c128__generated,
            c_fe_state_options_hud_init,
            4,
            &|c, seed| {
                ui_init_setup(c, seed, 0x0018_c128);
            },
        );
    }


    #[test]
    fn c_fe_state_options_init_matches() {
        check(ssx3_ee::s02::f_00188800__generated, c_fe_state_options_init, SEEDS, &|c, seed| {
            let _ = machine(c, seed, 0x188800, 0);
            pool_2(c);
            let s = DATA + 0x1000;
            let engine = DATA + 0x2000;
            c.g[A0][0] = s as u64;
            w32(c, s + 0x10, engine);
            w32(c, engine + 4, 0);
            w32(c, engine + 0x38, 0);
            w32(c, engine + 0x3c, 0);
            let _ = seed;
        });
    }


    #[test]
    fn c_fe_state_options_device_select_ctor_matches() {
        check(
            ee::s02::f_0018d7f0__generated,
            c_fe_state_options_device_select_ctor,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x18_d7f0, 3);
                belib(c);
                world(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.g[A1][0] = (DATA + 0x2200) as u64;
                c.g[A2][0] = (seed % 5) as u64;
                c.g[A3][0] = if seed % 2 == 0 { 0 } else { (DATA + 0x2400) as u64 };
                c.g[T0][0] = (seed % 7) as u64;
                c.pc = 0x0018_d7f0;
            },
        );
    }


    #[test]
    fn c_fe_state_options_save_load_on_render_matches() {
        check(ee::s02::f_0018d3c0__generated, c_fe_state_options_save_load_on_render, SEEDS_3, &|c, seed| {
            let mut r = machine(c, seed, 0x18d3c0, 2);
            be_library_2(c);
            let h0 = str_hash(c, 0x0045_e0f0);
            let h1 = str_hash(c, 0x0045_e100);
            let obj = DATA;
            c.g[A0][0] = obj as u64;
            w(c, GAME_GP.wrapping_sub(0x1718), if seed % 2 == 0 { 0 } else { 1 });
            let screen = DATA + 0x1000;
            let arr = DATA + 0x1100;
            w(c, obj + 0x40, screen);
            let count = if seed % 3 == 0 { 0 } else { 2 };
            w(c, screen + 0x3c, count);
            w(c, screen + 0x38, arr);
            for i in 0..2u32 {
                let present = i < count;
                let p = DATA + 0x1200 + 0x80 * i;
                w(c, arr + 4 * i, if present { p } else { 0 });
                if present {
                    let hash = match (seed + i) % 3 {
                        0 => h0,
                        1 => h1,
                        _ => r.next_u32(),
                    };
                    w(c, p + 0x38, hash);
                    w(c, p + 8, 0);
                }
            }
        });
    }


    #[test]
    fn c_fe_state_options_open_reset_popup_matches() {
        check(
            ee::s02::f_00188e58__generated,
            c_fe_state_options_open_reset_popup,
            SEEDS,
            &|c, seed| {
                machine(c, seed, 0x188e, 0);
                env(c);
                let this = DATA + 0x400;
                w32(
                    c,
                    this + 0x10,
                    if seed % 2 == 0 { 0 } else { DATA + 0x1000 },
                );
                c.g[A0][0] = sx(this);
            },
        );
    }


        // ------------------------------------------------------- device select

    #[test]
    fn cfe_state_options_device_select_v38_matches() {
        check(
            ee::s02::f_0018e2c0__generated,
            cfe_state_options_device_select_v38,
            SEEDS_4,
            &|c, seed| {
                let _ = base_2(c, seed, 0x18e2_c0, 8);
                let obj = SCRATCH;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (seed % 2) as u64;
                w(c, obj + 0x22c, if seed % 3 == 0 { 0 } else { obj + 0x2000 });
                let screen = obj + 0x3000;
                w(c, obj + 0x40, screen);
                w(c, screen + 0x3c, 1);
                w(c, screen + 0x38, screen + 0x400);
                let entry = screen + 0x500;
                w(c, screen + 0x400, entry);
                let hash = run_2(c, 0x0031_7670, ee::s10::f_00317670, &[0x004a_1458u64]);
                w(c, entry + 0x38, hash as u32);
                w(c, entry + 8, screen + 0x600);
                slot_2(c, screen + 0x600, 0x48, 0);
                slot_2(c, screen + 0x600, 0x4c, 1);
            },
        );
    }


        #[test]
    fn cfe_state_options_controller_update_matches() {
        check(
            ee::s02::f_0018d058__generated,
            cfe_state_options_controller_update,
            80,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x18d0);
                c.g[A0][0] = sx(DATA);
                c.bus.write8(DATA + 0x50, [0, 1, 2, 3][(seed % 4) as usize]);
                c.bus
                    .write32(DATA + 0x48, if seed % 3 == 0 { 0 } else { 2 });
                c.bus
                    .write32(DATA + 0x4c, if seed % 5 == 0 { 0 } else { 1 });
                fill(c, &mut r, DATA + 0x10, 0x40);
            },
        );
    }


    #[test]
    fn fe_state_options_hud_on_child_event_matches() {
        check(
            ee::s02::f_0018c1b8__generated,
            fe_state_options_hud_on_child_event,
            SEEDS_5,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x18c1, 3);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                c.g[A2][0] = match seed % 3 {
                    0 => 0x15,
                    1 => 0x16,
                    _ => 0x17,
                };
                w(c, DATA + 0x800 + 0xc, r.next_u32());
                w(c, DATA + 0x800 + 8, DATA + 0x900);
                w(c, DATA + 0x900 + 0xb8, 0);
                let _ = r.next_u32();
            },
        );
    }


    #[test]
    fn cfe_state_options_device_select_on_widget_notify_matches() {
        check(
            ee::s02::f_0018ddd0__generated,
            cfe_state_options_device_select_on_widget_notify,
            80,
            &|c, seed| {
                let _r = base_5(c, seed, 0x18dd);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x1000) as u64;
                c.bus.write32(DATA + 0x1000, seed);
                c.g[A2][0] = [8u64, 9, 6, 0x101][(seed % 4) as usize];
                // A prepared manager so `MCManager_Get` does not construct one.
                c.bus.write32(gp_at(-0x47c), DATA + 0x2000);
            },
        );
    }


    pub(crate) const RET: u32 = 0x0012_3450;


    /// `int StrHash(const char *s)` run during setup (to build hash-keyed
    /// screens).
    pub(crate) fn str_hash_2(c: &mut Cpu, s: u32) -> u64 {
        c.g[A0][0] = sx(s);
        c.g[RA][0] = RET as u64;
        c.pc = 0x0031_7670;
        ee::s10::f_00317670(c);
        assert_eq!(c.pc, RET, "StrHash did not return");
        c.g[V0][0]
    }


    /// A one-object screen at `screen`: `UIScreen_FindObject` returns the
    /// object for `hash`, whose `+8` vtable sends slot `+0x4c` to the fake
    /// callee.
    pub(crate) fn screen_fixture(c: &mut Cpu, screen: u32, hash: u64) {
        let array = screen + 0x100;
        let obj = screen + 0x200;
        let vt = screen + 0x300;
        w(c, screen + 0x3c, 1);
        w(c, screen + 0x38, array);
        w(c, array, obj);
        w(c, obj + 0x38, hash as u32);
        w(c, obj + 8, vt);
        c.bus.write16(vt + 0x48, 0);
        w(c, vt + 0x4c, FAKE);
    }


    /// The shared fixture of the two `_v37` state switches: `this` at DATA,
    /// the `+0x234`/`+0x22c` fields and an optional widget screen.
    pub(crate) fn state_switch_setup(c: &mut Cpu, seed: u32, salt: u64, field: u32, calls: u32) {
        let _ = machine(c, seed, salt, calls);
        belib(c);
        let screen = DATA + 0x40_000;
        w(
            c,
            DATA + 0x234,
            if seed % 2 == 0 {
                seed % 2
            } else {
                (seed + 1) % 2
            },
        );
        w(c, DATA + field, if seed % 3 == 0 { 0 } else { 1 });
        let found = seed % 4 >= 2;
        if found {
            let hash = str_hash_2(c, 0x004a_15e0);
            screen_fixture(c, screen, hash);
            w(c, DATA + 0x40, screen);
        }
        // (Registers last: building the fixture calls guest code.)
        c.g[A0][0] = DATA as u64;
        c.g[A1][0] = (seed % 2) as u64;
    }


    #[test]
    fn c_fe_state_options_device_select_v37_matches() {
        check(
            ee::s02::f_0018e178__generated,
            c_fe_state_options_device_select_v37,
            40,
            &|c, seed| state_switch_setup(c, seed, 0x18_e178, 0x22c, 8),
        );
    }
    use super::*;


    /// `StrHash` of a `D_004A...` id string, by the generated hasher (the
    /// test machine holds the game's `.rodata`).
    pub(crate) fn hash_of(addr: u32) -> u32 {
        let mut c = crate::difftest::fresh();
        c.g[A0][0] = addr as u64;
        c.g[RA][0] = 0x0012_3450;
        c.g[GP][0] = crate::difftest::GAME_GP as u64;
        c.pc = 0x0031_7670;
        (ee::s10::f_00317670)(&mut c);
        c.g[V0][0] as u32
    }


    pub(crate) const ID_VIBRATION_0: u32 = 0x004a_1568;

    pub(crate) const ID_VIBRATION_1: u32 = 0x004a_1570;

    pub(crate) const ID_CONFIG_V0: u32 = 0x004a_15c8;

    pub(crate) const ID_CONFIG_V1: u32 = 0x004a_1578;


    pub(crate) fn fixture(c: &mut Cpu, seed: u32) {
        let _r = base_3(c, seed, 0x18ca10);
        c.pc = 0x0018_ca10;
        let self_ = DATA;
        let widget = DATA + 0x800;
        c.g[A0][0] = self_ as u64;
        c.g[A1][0] = if seed % 6 == 0 { 0 } else { widget as u64 };
        c.g[A2][0] = (seed % 12) as u64;
        c.bus.write32(self_ + 0x10, DATA + 0x1000);
        c.bus.write32(self_ + 0x48, if seed % 3 == 0 { 3 } else { 0 });
        c.bus.write32(self_ + 0x4c, if seed % 5 == 0 { 4 } else { 0 });
        c.bus.write8(self_ + 0x50, (seed % 4) as u8);
        let id = match seed % 4 {
            0 => ID_VIBRATION_0,
            1 => ID_VIBRATION_1,
            2 => ID_CONFIG_V0,
            _ => ID_CONFIG_V1,
        };
        let h = hash_of(id);
        c.bus
            .write32(widget + 0x38, if seed % 7 == 3 { 0 } else { h });
        c.bus.write32(widget + 0x18, 0);
    }


    #[test]
    fn on_widget_action_matches() {
        check(
            ee::s02::f_0018ca10__generated,
            fe_options_controller_on_widget_action,
            16,
            &|c, seed| {
                fixture(c, seed);
            },
        );
    }
}
