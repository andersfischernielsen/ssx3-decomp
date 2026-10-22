//! `src/main/ssxappdtor.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::call;
use crate::call::call_ptr;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use ssx3_ee as ee;
use crate::call::guest;
use crate::abi::r32;
use crate::call::VSlot;
use crate::call::Unwound;
use crate::call::sp;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::forward;
pub(crate) use crate::game::fe::festateloadhint::ui_state_dtor_with_vtable;
pub(crate) use crate::game::movie::movieplayer::lw_at;
pub(crate) use crate::game::dirtysock::tags2::OPERATOR_DELETE;
pub(crate) use crate::game::dirtysock::tags2::deleting_dtor;
pub(crate) use crate::game::ui::uivector::r;
pub(crate) use crate::game::fe::feasyncfile::ui_init_screen;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p_2;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::sdsp;
pub(crate) use crate::game::fe::festateloadhint::gp;
pub(crate) use crate::game::fe::festateloadhint::load_state_transition_in;
pub(crate) use crate::game::fe::feasyncfile::at;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw;
pub(crate) use crate::game::fe::feasyncfile::lw_8 as lw_2;
pub(crate) use crate::game::fe::feasyncfile::sw_5 as sw;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0024_38f0, ssx_app_dtor, "SSXApp_dtor"),
    (0x0024_3a40, app_module_dtor, "AppModule_dtor"),
    (0x0024_3a80, pre_start_screen_dtor, "PreStartScreen_dtor"),
    (0x0024_3ae8, app_module_vt47d818_dtor, "AppModuleVt47d818_dtor"),
    (0x0024_3b50, start_screen_dtor, "StartScreen_dtor"),
    (0x0024_3b80, c_game_load_screen_dtor, "cGameLoadScreen_dtor"),
    (0x0024_3c00, c_fe_load_screen_dtor, "cFELoadScreen_dtor"),
    (0x0024_3c80, game_state_dtor, "GameState_dtor"),
    (0x0024_3cf0, game_state_quit_v01, "GameStateQuit_v01"),
    (0x0024_3d58, game_state_idle_v01, "GameStateIdle_v01"),
    (0x0024_3d88, game_state_finish_v01, "GameStateFinish_v01"),
    (0x0024_3db8, game_state_reset_ai_v01, "GameStateResetAI_v01"),
    (0x0024_3e20, game_state_pre_race_load_v01, "GameStatePreRaceLoad_v01"),
    (0x0024_3e50, game_state_countdown_v01, "GameStateCountdown_v01"),
    (0x0024_3eb8, game_state_race_v01, "GameStateRace_v01"),
    (0x0024_3f20, game_state_reset_v01, "GameStateReset_v01"),
    (0x0024_3f50, game_state_restart_race_v01, "GameStateRestartRace_v01"),
    (0x0024_3fc0, game_state_warp_to_event_v01, "GameStateWarpToEvent_v01"),
    (0x0024_4028, game_state_event_select_v01, "GameStateEventSelect_v01"),
    (0x0024_4060, game_state_event_switch_load_v01, "GameStateEventSwitchLoad_v01"),
    (0x0024_4090, game_state_pre_race_menu_v01, "GameStatePreRaceMenu_v01"),
    (0x0024_40f8, game_state_results_v01, "GameStateResults_v01"),
    (0x0024_4160, game_state_finish_cinematic_v01, "GameStateFinishCinematic_v01"),
    (0x0024_41c8, game_state_replay_playback_v01, "GameStateReplayPlayback_v01"),
    (0x0024_4240, pause_stack_set_base, "PauseStack_SetBase"),
    (0x0024_43c0, generic_stream_pause_v01, "GenericStreamPause_v01"),
    (0x0024_43f0, generic_stream_pause_v02, "GenericStreamPause_v02"),
    (0x0024_4410, vfx_process_dtor, "VfxProcess_dtor"),
    (0x0024_4478, vfx_process_update, "VfxProcess_Update"),
    (0x0024_44a8, vfx_process_render, "VfxProcess_Render"),
    (0x0024_44f0, gm_handler_dtor, "GMHandler_dtor"),
    (0x0024_4560, gm_handler0_v01, "GMHandler0_v01"),
    (0x0024_4590, gm_handler1_v01, "GMHandler1_v01"),
    (0x0024_45c0, gm_handler8_v01, "GMHandler8_v01"),
    (0x0024_45f0, gm_handler9_v01, "GMHandler9_v01"),
    (0x0024_4630, gm_handler2_v01, "GMHandler2_v01"),
    (0x0024_4660, gm_handler4_v01, "GMHandler4_v01"),
    (0x0024_4690, gm_handler5_v01, "GMHandler5_v01"),
    (0x0024_46c0, gm_handler6_v01, "GMHandler6_v01"),
    (0x0024_46f0, gm_handler7_v01, "GMHandler7_v01"),
    (0x0024_4720, gm_handler_unused_v01, "GMHandlerUnused_v01"),
    (0x0024_48e0, aivfx_process_dtor, "AIVfxProcess_dtor"),
    (0x0024_4948, aivfx_process_update, "AIVfxProcess_Update"),
    (0x0024_4978, aivfx_process_render, "AIVfxProcess_Render"),
    (0x0024_49a8, stats_process_dtor, "StatsProcess_dtor"),
    (0x0024_49f8, stats_process_update, "StatsProcess_Update"),
    (0x0024_4a18, stats_process_render, "StatsProcess_Render"),
    (0x0024_5248, c_fe_state_progressive_init, "cFEStateProgressive_Init"),
    (0x0024_53e0, c_fe_state_progressive_on_widget_notify, "cFEStateProgressive_OnWidgetNotify"),
    (0x0024_53f0, c_game_load_state_dtor, "cGameLoadState_dtor"),
    (0x0024_5418, c_game_load_state_init, "cGameLoadState_Init"),
    (0x0024_54f8, c_game_load_state_transition_in, "cGameLoadState_TransitionIn"),
    (0x0024_5598, c_game_load_state_on_widget_created, "cGameLoadState_OnWidgetCreated"),
    (0x0024_5600, c_game_load_state_out_lodge_dtor, "cGameLoadStateOutLodge_dtor"),
    (0x0024_5628, c_game_load_state_out_lodge_init, "cGameLoadStateOutLodge_Init"),
    (0x0024_5698, c_game_load_state_out_lodge_transition_in, "cGameLoadStateOutLodge_TransitionIn"),
    (0x0024_5708, c_game_load_state_conquer_dtor, "cGameLoadStateConquer_dtor"),
    (0x0024_58b8, c_game_load_state_conquer_transition_in, "cGameLoadStateConquer_TransitionIn"),
    (0x0024_5928, c_fe_load_hint_state_dtor, "cFELoadHintState_dtor"),
];

// ---------------------------------------------------------------------------
// Callees the generator emitted as direct calls (the readable alias, which
// dispatches through the override slot) and as cross-shard `call_indirect`s.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::EVENT_SELECT_HANDLE_INPUT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::MC_MANAGER_GET_INSTANCE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STATS_PROCESS_UPDATE_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STATS_PROCESS_RENDER_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_ENGINE_PUSH_FREE_NODE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_OBJ_DEACTIVATE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_DUMMY_OBJ_READ_FIELDS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::PATH_DATA_READ_FLOAT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::EA_MEM_COPY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RAND_NEXT_MAIN;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RAND_NEXT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::TEX_MAN_SET_ALLOC_FLAGS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::GRAPHICS_MAN_EMPTY_HOOK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_STATE_MGR_RENDER_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_PUSH_BACK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_NODE_UNLINK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_RESET_SECTIONS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STRLEN_AND_STRCPY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::CBXSTRING_ASSIGN_CSTR;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::LOBBY_MAN_REQUEST_STATS_PAGE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SSX_APP_BOOT_LOAD;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::USTR_FROM_ASCII;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::PATH_SET_TRACK_CONTROL_VALUE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_TRIGGER_GET_INFO;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_TRIGGER_GET_NAME;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STREAM_BUFFER_GET_STATUS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::MENU_SYS_GET_INPUT_CODE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::IRRADIANCE_REF_GET_ENTRY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::C_WORLD_SOUND_UNLINK_EMITTERS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RIDER_METRIX_BANK_COMBO;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::CAMERA_TRIGGER_STACK_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::BOLT_FILE_INDEX_MANAGER_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::dirtysock::tags2::callee::EVENT_SELECT_ON_WIDGET_NOTIFY;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_STATE_GOTO_NEXT;
    pub(crate) use crate::game::dirtysock::tags2::callee::GAME_MODULE_REQUEST_STATE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MEM_ALLOC_NAMED_ALIGNED;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOBBY_MAN_SEND_LOCK_CHAL;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SETUP_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_SHARED_LOGIN_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::C_VOIP_MAN_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_GET_PLAY_STATUS;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_LIST_COUNT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_CHOOSE_SCRIPT;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_ENGINE_GET_PROCESS;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_CHANNEL_SET_LISTENER;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_INSTANCE_PAUSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::IN_SEQ_INSTANCE_RESUME;
    pub(crate) use crate::game::dirtysock::tags2::callee::SCRIPT_CMD_QUEUE_CLEAR;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_FREE_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_UNLOAD_MEMORY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_RELEASE;
    pub(crate) use crate::game::dirtysock::tags2::callee::SND_BANK_GET_BUFFER_SIZE;
    pub(crate) use crate::game::dirtysock::tags2::callee::BANK_MONITOR_INSTANCE_RESET;
    pub(crate) use crate::game::dirtysock::tags2::callee::OPERATOR_DELETE;
    pub(crate) use crate::game::dirtysock::tags2::callee::LOAD_INDICATOR_CLEAR_BUSY;
    pub(crate) use crate::game::dirtysock::tags2::callee::WSCRIPT_MAN_DELETE_ALL_PROCESSES;
    pub(crate) use crate::game::dirtysock::tags2::callee::WSCRIPT_MAN_SPAWN_MISSIONS;
    pub(crate) use crate::game::dirtysock::tags2::callee::UI_STATE_DTOR;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_CLOSE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_READ;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_WRITE;
    pub(crate) use crate::game::dirtysock::tags2::callee::MC_REQUEST_FLUSH;
    pub(crate) use crate::game::dirtysock::tags2::callee::STRCPY;
    pub(crate) use crate::game::dirtysock::tags2::callee::SECTION_MAN_GET_WORLD_SECTION_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL_2 as UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_4 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_2 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_3 as SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_10 as NET_FE_STATE_ON_CHILD_EVENT;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_2 as UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_3 as UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR_2 as UI_OBJECT_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_STREAMER_UPDATE_VIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_SET_IN_GAME_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SET_INPUT_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_SET_OWNER;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_3 as WORLD_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_READ_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_FLAG_MANAGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_MANAGER_ADD_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_APPLY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_ADVANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_PROCESS_DTOR_2 as GAME_PROCESS_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_RESET_RACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODE_MAN_RESET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_REQUEST_STATE_5 as GAME_MODULE_REQUEST_STATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_START_NEXT_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_APP;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_INTERFACE_GET_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_BE_2 as BE_GET_BE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_5 as SPRINTF;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_ABORT_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_EMAIL_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_LOGIN_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_12 as NET_FE_STATE_ON_CHILD_EVENT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHALLENGE_POPUP_SET_SETTINGS_LOCKED;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_11 as NET_FE_STATE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::VOIP_MAN_DEACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::VOIP_MAN_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_RESET_RACE_RESULT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_IS_TRACK_GROUP0_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STORE_RACE_RESULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_SKIP_WHITESPACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_GET_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_CHARACTER_2 as BE_GET_RIDER_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COUNT_COLLECTED_ON_PEAK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COUNT_COMPLETED_ON_PEAK;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_GET_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::SETUP_BUDDY_ACTION_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_4 as NET_FE_STATE_GET_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_3 as NET_FE_STATE_SHOW_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_CARD_STATE_BASE_SHUTDOWN;
    pub(crate) use crate::game::fe::feasyncfile::callee::DEAD_NODE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CROWD_MAN3D_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::CROWD_MAN3D_GROUP_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_TREE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FEMODULE_RIDER_PREVIEW_UNLOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_UPDATE_BOARD_LEAN;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_BUILD_WORLD_MATRIX;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_LOAD_STATE_2 as GAME_STATE_LOAD_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_ON_CARD_SCANNED;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_LOAD_UPDATE_PLAYER_RANKING;
    pub(crate) use crate::game::fe::feasyncfile::callee::LIGHT_MAN_EMPTY_HOOK_D;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_CHAR_PROFILE_EVENT_RECORD;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_SLOT_3 as BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_SET_ACTIVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_GET_BITS_2 as MPEG2_GET_BITS;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_MARKER_BIT_2 as MPEG2_MARKER_BIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_FLUSH;

    /// `NetFEState_OnChildEventDefault` (same shard as its caller).
    pub const NET_FE_ON_CHILD_EVENT_DEFAULT: u32 = 0x001a_93d0;

    /// `cSharedLoginMan_SetAccountName` / `..._Password`.
    pub const SHARED_LOGIN_SET_ACCOUNT_NAME: u32 = 0x0026_5f18;

    pub const SHARED_LOGIN_SET_ACCOUNT_PASSWORD: u32 = 0x0026_5f68;

    /// `MessageCenter_GetInbox`.
    pub const MESSAGE_CENTER_GET_INBOX: u32 = 0x001e_39f8;

    /// `UIState_OnActivate`.
    pub const UI_STATE_ON_ACTIVATE: u32 = 0x0039_e4c0;

    /// `SSXAudioSystem_GetInstance` / `_PlayFESound` / `_OnEventSelectEnter`.
    pub const AUDIO_GET_INSTANCE: u32 = 0x0028_b180;

    pub const AUDIO_PLAY_FE_SOUND: u32 = 0x0029_4f78;

    pub const AUDIO_ON_EVENT_SELECT_ENTER: u32 = 0x0028_f5b8;

    /// `cFEStateBuyAttrib_CommitPurchase`.
    pub const BUY_ATTRIB_COMMIT_PURCHASE: u32 = 0x001f_5300;

    /// `cFEState_OnActivate` / `cFEStateCareerStats_SelectPage`.
    pub const FE_STATE_ON_ACTIVATE: u32 = 0x0018_6518;

    pub const CAREER_STATS_SELECT_PAGE: u32 = 0x001f_5ba8;

    /// `OVScreenRewardList_ShowPage`.
    pub const REWARD_LIST_SHOW_PAGE: u32 = 0x001f_fd08;

    /// `EventSelect_SetMode` / `EventSelect_Shutdown`.
    pub const EVENT_SELECT_SET_MODE: u32 = 0x0020_30a0;

    pub const EVENT_SELECT_SHUTDOWN: u32 = 0x0020_0af0;

    /// `cOVStateOverlay_Shutdown`.
    pub const OV_STATE_OVERLAY_SHUTDOWN: u32 = 0x0020_a430;

    /// `UIEngine_Update` / `AppModule_Update`.
    pub const UI_ENGINE_UPDATE: u32 = 0x0039_8038;

    pub const APP_MODULE_UPDATE: u32 = 0x0023_1cb0;

    /// `operator delete`.
    pub const DELETE: u32 = 0x0031_7e50;
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
// Pause stack.

/// `void PauseStack_SetBase(int index)` (0x244240): look up a pointer in the
/// table at 0x4428f0 indexed by `index`, cache it at `gp-0x6a0` and store
/// `index` in the record reached through the table at `(gp-0x69c)`.
pub fn pause_stack_set_base(c: &mut Cpu) {
    c.cycles += 13;
    let index = abi::arg(c, 0);
    c.g[V1][0] = sx(index << 2);
    c.g[V0][0] = sx(0x44_0000u32.wrapping_add(0x28f0));
    c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x69c)).lw(c);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A1][0] = 0x53_0000;
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
    c.g[A1][0] = sx(0x53_0000u32.wrapping_add(0x66e8));
    c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
    Addr((c.g[GP][0] as u32).wrapping_sub(0x6a0)).sw(c, c.g[V0][0]);
    Addr(c.g[A2][0] as u32).sw(c, c.g[A0][0]);
    abi::ret(c);
}

/// Cross-shard direct call emitted as `call_indirect`.
pub(crate) const GAME_PROCESS_DTOR: u32 = 0x0035_46c8;


// ---------------------------------------------------------------------------
// AI visual-effects processes and menus.

/// `void AIVfxProcess_dtor(AIVfxProcess *this, uint flags)` (0x2448e0): set the
/// vtable at +0xc, call the sub-effect's destructor (`this+0x10 -> +4` slot
/// +8), then `GameProcess_dtor(this, flags)`.
pub fn aivfx_process_dtor(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x48_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x48_0000u32.wrapping_sub(0x34a0));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[V0][0]);
        c.g[A2][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x10)).lw(c);
        let has_effect = c.g[A2][0] != 0;
        c.g[S1][0] = c.g[A1][0];
        if has_effect {
            c.cycles += 6;
            c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(4)).lw(c);
            c.g[A1][0] = 0x3;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(8)).lh(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, c.g[V1][0] as u32, 0x0024_4924)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, GAME_PROCESS_DTOR, 0x0024_4930)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void AIVfxProcess_Update(AIVfxProcess *this)` (0x244948): call the
/// sub-effect's update (`this+0x10 -> +4` vtable slot +0x10).
pub fn aivfx_process_update(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let inner = Addr(c.g[A0][0] as u32).off(0x10).lw(c);
        c.g[A1][0] = inner;
        let eff = Addr(inner as u32).off(4).lw(c);
        c.g[V0][0] = eff;
        c.g[A0][0] = Addr(eff as u32).off(0x10).lh(c);
        let func = Addr(eff as u32).off(0x14).lw(c);
        c.g[V1][0] = func;
        c.g[A0][0] = abi::addu(inner, c.g[A0][0]);
        call_ptr(c, func as u32, 0x0024_4968)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void AIVfxProcess_Render(AIVfxProcess *this)` (0x244978): call the
/// sub-effect's render (`this+0x10 -> +4` vtable slot +0x18).
pub fn aivfx_process_render(c: &mut Cpu) {
    call::guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let inner = Addr(c.g[A0][0] as u32).off(0x10).lw(c);
        c.g[A1][0] = inner;
        let eff = Addr(inner as u32).off(4).lw(c);
        c.g[V0][0] = eff;
        c.g[A0][0] = Addr(eff as u32).off(0x18).lh(c);
        let func = Addr(eff as u32).off(0x1c).lw(c);
        c.g[V1][0] = func;
        c.g[A0][0] = abi::addu(inner, c.g[A0][0]);
        call_ptr(c, func as u32, 0x0024_4998)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void StatsProcess_Update(this)` (0x002449f8): forward to the stub on the
/// owned processor at `+0x10`.
pub fn stats_process_update(c: &mut Cpu) {
    forward(
        c,
        7,
        0x10,
        |c| c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x10)).lw(c),
        callee::STATS_PROCESS_UPDATE_STUB,
        0x0024_4a08,
    )
}


/// `void StatsProcess_Render(this)` (0x00244a18): forward to the stub on the
/// owned processor at `+0x10`.
pub fn stats_process_render(c: &mut Cpu) {
    forward(
        c,
        7,
        0x10,
        |c| c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x10)).lw(c),
        callee::STATS_PROCESS_RENDER_STUB,
        0x0024_4a28,
    )
}


/// `void cGameLoadState_dtor(self)` (0x2453f0).
pub fn c_game_load_state_dtor(c: &mut Cpu) {
    guest(c, |c| ui_state_dtor_with_vtable(c, 0x36b8, 0x0024_540c))
}


/// `void cGameLoadStateOutLodge_dtor(self)` (0x245600).
pub fn c_game_load_state_out_lodge_dtor(c: &mut Cpu) {
    guest(c, |c| ui_state_dtor_with_vtable(c, 0x3788, 0x0024_561c))
}


/// `void cGameLoadStateConquer_dtor(self)` (0x245708).
pub fn c_game_load_state_conquer_dtor(c: &mut Cpu) {
    guest(c, |c| ui_state_dtor_with_vtable(c, 0x3858, 0x0024_5724))
}


/// `void cFELoadHintState_dtor(self)` (0x245928).
pub fn c_fe_load_hint_state_dtor(c: &mut Cpu) {
    guest(c, |c| ui_state_dtor_with_vtable(c, 0x3928, 0x0024_5944))
}


// ---------------------------------------------------------------------------
// Vfx process hooks.

/// The `VfxProcess` hooks take the process object at `*(this + 0x10)`, whose
/// vtable lives at `+0x44`.
pub(crate) fn vfx_process_vcall(c: &mut Cpu, slot: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        let obj = lw_at(c, this as u64, 0x10);
        c.g[A1][0] = obj;
        let vt = lw_at(c, obj, 0x44);
        c.g[V0][0] = vt;
        let m = VSlot::read(c, vt, slot);
        c.g[A0][0] = m.delta;
        c.g[V1][0] = m.func;
        c.g[A0][0] = sx((obj as u32).wrapping_add(m.delta as u32));
        call_ptr(c, m.func as u32, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `VfxProcess_Update(this)` 0x244478: slot +0x10 of the process vtable.
pub fn vfx_process_update(c: &mut Cpu) {
    vfx_process_vcall(c, 0x10, 0x0024_4498)
}


/// `VfxProcess_Render(this)` 0x2444a8: slot +0x18 of the process vtable.
pub fn vfx_process_render(c: &mut Cpu) {
    vfx_process_vcall(c, 0x18, 0x0024_44c8)
}


/// `a + off` (32-bit address arithmetic).
#[inline(always)]
pub(crate) fn p(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}


// ---------------------------------------------------------------------------
// Deleting destructors.

/// The shared body of the tiny deleting destructors: republish the class's
/// vtable pointer at `off` in `this`, then `operator delete(this)` when bit 0
/// of the destructor flags (`a1`) is set. `ret` is the address the delete
/// returns to (the instruction after its call).
pub(crate) fn vtable_dtor(c: &mut Cpu, vtable: u32, off: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[V0][0] = vtable as u64;
        let frame = Frame::open(c, 0x10);
        let this = r32(c, A0);
        frame.save_ra(c, 0);
        c.g[A1][0] &= 1;
        let deleting = c.g[A1][0] != 0;
        let v = c.g[V0][0];
        c.sw(p(this, off), v);
        if deleting {
            c.cycles += 2;
            call_ptr(c, callee::DELETE, ret)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `AppModule` destructor (vtable at `+0`).
pub fn app_module_dtor(c: &mut Cpu) {
    vtable_dtor(c, 0x0046_d970, 0, 0x0024_3a64);
}


/// `PreStartScreen` destructor (vtable at `+0`).
pub fn pre_start_screen_dtor(c: &mut Cpu) {
    vtable_dtor(c, 0x0046_d970, 0, 0x0024_3aa4);
}


/// `AppModule`'s secondary vtable (0x47d818) destructor (vtable at `+0`).
pub fn app_module_vt47d818_dtor(c: &mut Cpu) {
    vtable_dtor(c, 0x0046_d970, 0, 0x0024_3b0c);
}


/// `StartScreen` destructor (vtable at `+0`).
pub fn start_screen_dtor(c: &mut Cpu) {
    vtable_dtor(c, 0x0046_d970, 0, 0x0024_3b74);
}


/// `cGameLoadScreen` destructor (vtable at `+0`).
pub fn c_game_load_screen_dtor(c: &mut Cpu) {
    vtable_dtor(c, 0x0046_d970, 0, 0x0024_3ba4);
}


/// `cFELoadScreen` destructor (vtable at `+0`).
pub fn c_fe_load_screen_dtor(c: &mut Cpu) {
    vtable_dtor(c, 0x0046_d970, 0, 0x0024_3c24);
}


/// `GameState` destructor (vtable at `+0xc`).
pub fn game_state_dtor(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_3ca4);
}


/// `GameStateQuit` destructor (vtable at `+0xc`).
pub fn game_state_quit_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_3d14);
}


/// `GameStateIdle` destructor (vtable at `+0xc`).
pub fn game_state_idle_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_3d7c);
}


/// `GameStateFinish` destructor (vtable at `+0xc`).
pub fn game_state_finish_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_3dac);
}


/// `GameStateResetAI` destructor (vtable at `+0xc`).
pub fn game_state_reset_ai_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_3ddc);
}


/// `GameStatePreRaceLoad` destructor (vtable at `+0xc`).
pub fn game_state_pre_race_load_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_3e44);
}


/// `GameStateCountdown` destructor (vtable at `+0xc`).
pub fn game_state_countdown_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_3e74);
}


/// `GameStateRace` destructor (vtable at `+0xc`).
pub fn game_state_race_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_3edc);
}


/// `GameStateReset` destructor (vtable at `+0xc`).
pub fn game_state_reset_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_3f44);
}


/// `GameStateRestartRace` destructor (vtable at `+0xc`).
pub fn game_state_restart_race_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_3f74);
}


/// `GameStateWarpToEvent` destructor (vtable at `+0xc`).
pub fn game_state_warp_to_event_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_3fe4);
}


/// `GameStateEventSelect` destructor (vtable at `+0xc`).
pub fn game_state_event_select_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_404c);
}


/// `GameStateEventSwitchLoad` destructor (vtable at `+0xc`).
pub fn game_state_event_switch_load_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_4084);
}


/// `GameStatePreRaceMenu` destructor (vtable at `+0xc`).
pub fn game_state_pre_race_menu_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_40b4);
}


/// `GameStateResults` destructor (vtable at `+0xc`).
pub fn game_state_results_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_411c);
}


/// `GameStateFinishCinematic` destructor (vtable at `+0xc`).
pub fn game_state_finish_cinematic_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_4184);
}


/// `GameStateReplayPlayback` destructor (vtable at `+0xc`).
pub fn game_state_replay_playback_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d6e8, 0xc, 0x0024_41ec);
}


/// `GenericStreamPause` destructor (vtable at `+0`).
pub fn generic_stream_pause_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_db28, 0, 0x0024_43e4);
}


/// `GMHandler` destructor (vtable at `+0`).
pub fn gm_handler_dtor(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d008, 0, 0x0024_4514);
}


/// `GMHandler0` destructor (vtable at `+0`).
pub fn gm_handler0_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d008, 0, 0x0024_4584);
}


/// `GMHandler1` destructor (vtable at `+0`).
pub fn gm_handler1_v01(c: &mut Cpu) {
    vtable_dtor(c, 0x0047_d008, 0, 0x0024_45b4);
}


/// `void GMHandler8_v01(void *self, int flags)` (0x2445c0): deleting
/// destructor that republishes `D_0047D008` at `self+0`.
pub fn gm_handler8_v01(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0047_d008, 0, 0x0024_45e4))
}


/// `void GMHandler9_v01(void *self, int flags)` (0x2445f0): republishes
/// `D_0047D008` at `self+0` and deletes when asked.
pub fn gm_handler9_v01(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0047_d008, 0, 0x0024_4614))
}


/// `void GMHandler2_v01(void *self, int flags)` (0x244630): republishes
/// `D_0047D008` at `self+0` and deletes when asked.
pub fn gm_handler2_v01(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0047_d008, 0, 0x0024_4654))
}


/// `void GMHandler4_v01(void *self, int flags)` (0x244660): republishes
/// `D_0047D008` at `self+0` and deletes when asked.
pub fn gm_handler4_v01(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0047_d008, 0, 0x0024_4684))
}


/// `void GMHandler5_v01(void *self, int flags)` (0x244690): republishes
/// `D_0047D008` at `self+0` and deletes when asked.
pub fn gm_handler5_v01(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0047_d008, 0, 0x0024_46b4))
}


/// `void GMHandler6_v01(void *self, int flags)` (0x2446c0): republishes
/// `D_0047D008` at `self+0` and deletes when asked.
pub fn gm_handler6_v01(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0047_d008, 0, 0x0024_46e4))
}


/// `void GMHandler7_v01(void *self, int flags)` (0x2446f0): republishes
/// `D_0047D008` at `self+0` and deletes when asked.
pub fn gm_handler7_v01(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0047_d008, 0, 0x0024_4714))
}


/// `void GMHandlerUnused_v01(void *self, int flags)` (0x244720): republishes
/// `D_0047D008` at `self+0` and deletes when asked.
pub fn gm_handler_unused_v01(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0047_d008, 0, 0x0024_4744))
}


/// `void StatsProcess_dtor(StatsProcess* self, int flags)` (0x2449a8):
/// install the destructor vtable, free the owned buffer and run the base
/// destructor.
pub fn stats_process_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x480000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx(0x0047_cae8);
        frame.save(c, S1, 0x10);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        Addr(this).off(0xc).sw(c, c.g[V0][0]);
        c.g[A0][0] = Addr(this).off(0x10).lw(c);
        call_ptr(c, 0x0031_7e50, 0x0024_49d4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0035_46c8, 0x0024_49e0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `SSXApp_dtor` (0x2438f0): destroy the three embedded strings, then the app
/// base.
pub fn ssx_app_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xcc));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0024_3914)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc8));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0024_3920)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc4));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0024_392c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0031_6d28, 0x0024_3938)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `GenericStreamPause_v02` (0x002443f0): returns its second argument.
pub fn generic_stream_pause_v02(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.g[A1][0];
    abi::ret(c);
}


/// `cFEStateProgressive_OnWidgetNotify` (0x002453e0): 0x101 when the widget
/// id is 6, else 0.
pub fn c_fe_state_progressive_on_widget_notify(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A2][0] ^= 0x6;
    c.g[V0][0] = 0x101;
    if c.g[A2][0] != 0 {
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}


/// `void VfxProcess_dtor(VfxProcess* self, int flags)` (0x244410).
pub fn vfx_process_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0x0048_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x2f90));
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p(c.g[S0][0] as u32, 0xc)).sw(c, c.g[V0][0]);
        c.g[A2][0] = Addr(p(c.g[S0][0] as u32, 0x10)).lw(c);
        let no_obj = c.g[A2][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_obj {
            c.cycles += 6;
            c.g[V0][0] = Addr(p(c.g[A2][0] as u32, 0x44)).lw(c);
            c.g[A1][0] = 0x3;
            c.g[A0][0] = Addr(p(c.g[V0][0] as u32, 8)).lh(c);
            c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0xc)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0024_4454)?;
        }
        c.cycles += 8;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0035_46c8, 0x0024_4460)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cGameLoadStateOutLodge_Init(void* self)` (0x245628).
pub fn c_game_load_state_out_lodge_init(c: &mut Cpu) {
    guest(c, |c| ui_init_screen(c, 0x0024_5628, 0x0047_bcc8))
}


/// `cGameLoadState_OnWidgetCreated(self, widget)` 0x245598: tell the BE
/// interface about the widget, then run the QP or MP setup hook depending on
/// the game-mode byte at 0x535c11.
pub fn c_game_load_state_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        let this = abi::arg64(c, 0);
        c.g[S0][0] = this;
        frame.save_ra(c, 0);
        let widget = abi::arg64(c, 1);
        c.g[S1][0] = widget;
        call_ptr(c, 0x0014_dc80, 0x0024_55b4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0024_55c0)?;
        c.g[V1][0] = 0x530000;
        c.g[V0][0] = Addr(0x0053_5c11).lb(c);
        c.g[V0][0] ^= 2;
        c.g[A0][0] = c.g[S0][0];
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0024_6278, 0x0024_55ec)?;
        } else {
            c.cycles += 4;
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0024_64e8, 0x0024_55dc)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int cGameLoadStateOutLodge_TransitionIn(LoadState* s)` (0x245698).
pub fn c_game_load_state_out_lodge_transition_in(c: &mut Cpu) {
    guest(c, |c| load_state_transition_in(c, 0x0024_5698))
}


/// `int cGameLoadStateConquer_TransitionIn(LoadState* s)` (0x2458b8).
pub fn c_game_load_state_conquer_transition_in(c: &mut Cpu) {
    guest(c, |c| load_state_transition_in(c, 0x0024_58b8))
}


/// `void cFEStateProgressive_Init(self)` (0x245248): add the state's screen to
/// the UI engine by hash name, play it, then set the instruction text.
pub fn c_fe_state_progressive_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        let self_ = abi::arg(c, 0);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = sx(self_);
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x004a_0000;
        c.g[A0][0] = sx(p(r32(c, A0), 0x2ad8));
        c.g[S0][0] = c.lw(at(c, S1, 0x10));
        call_ptr(c, callee::GET_HASH_VALUE32, 0x0024_526c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME, 0x0024_5280)?;
        let no_screen = c.g[V0][0] == 0;
        c.sw(at(c, S1, 0x40), c.g[V0][0]);
        if !no_screen {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, callee::UI_SCREEN_PLAY_FRAME, 0x0024_5298)?;
        }
        c.cycles += 8;
        c.g[A0][0] = 0x004a_0000;
        c.g[A0][0] = sx(p(r32(c, A0), 0x2ac0));
        call_ptr(c, callee::GET_HASH_VALUE32, 0x0024_52a4)?;
        c.g[A0][0] = c.lw(at(c, S1, 0x40));
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, callee::UI_SCREEN_FIND_OBJECT, 0x0024_52b0)?;
        let no_text = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[V0][0];
        if !no_text {
            c.cycles += 3;
            c.g[A1][0] = 0x0048_0000;
            c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0x4678));
            call_ptr(c, callee::UI_TEXT_SET_ASCII_STRING, 0x0024_52c4)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void cGameLoadState_TransitionIn(self, refresh)` (0x2454f8): refresh the
/// loading screen's progress text and, when requested, both player rankings.
pub fn c_game_load_state_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x110);
        frame.save(c, S0, 0x100);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        frame.save(c, S2, 0xe0);
        frame.save(c, S1, 0xf0);
        frame.save_ra(c, 0xd0);
        c.g[S2][0] = c.g[A1][0];
        c.g[A0][0] = 0x004a_2bc8;
        call_ptr(c, table::STR_HASH, 0x0024_5520)?;
        c.g[A0][0] = lw(c, S0, 0x40);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, table::UI_SCREEN_FIND_OBJECT, 0x0024_552c)?;
        c.g[S1][0] = c.g[V0][0];
        let none = c.g[S1][0] == 0;
        c.g[V0][0] = Addr(gp(c, 0x2a74)).lw(c);
        if !none {
            c.cycles += 10;
            c.g[A1][0] = 0x004a_2bd0;
            c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x38));
            c.cvt_w_s(1, 0);
            c.mfc1(6, 1);
            c.g[A0][0] = c.g[SP][0];
            call_ptr(c, table::SPRINTF, 0x0024_5554)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, table::UI_TEXT_SET_WIDE_STRING, 0x0024_5560)?;
        }
        c.cycles += 2;
        let no_refresh = c.g[S2][0] == 0;
        c.g[A0][0] = c.g[S0][0];
        if !no_refresh {
            c.cycles += 5;
            c.g[A1][0] = 0;
            callee::GAME_LOAD_UPDATE_PLAYER_RANKING.call(c, 0x0024_5570)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 1;
            callee::GAME_LOAD_UPDATE_PLAYER_RANKING.call(c, 0x0024_557c)?;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x100);
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0xf0);
        frame.restore(c, S2, 0xe0);
        frame.restore_ra(c, 0xd0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// cGameLoadState_Init (0x0000245418).

/// `cGameLoadState_Init`: initialise the game loader state: load the level file and set up its script and world.
pub fn c_game_load_state_init(c: &mut Cpu) {
    guest(c, |c| {
    'b_2454d0: {
    'b_2454cc: {
    'b_245494: {
    c.cycles += 14;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[S1][0] = c.g[A0][0];
    call_ptr(c, 0x0014dc80, 0x00245430)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0u64;
    call_ptr(c, 0x0014dd58, 0x0024543c)?;
    c.g[V1][0] = 0x530000;
    c.g[A0][0] = 0x2;
    c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c11));
    if c.g[V0][0] != c.g[A0][0] {
    c.g[A0][0] = 0x480000;
    break 'b_245494;
    } else {
    c.cycles += 11;
    c.g[A0][0] = 0x480000;
    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x45d8));
    call_ptr(c, 0x00317670, 0x00245460)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S1][0];
    c.g[A3][0] = 0u64;
    call_ptr(c, 0x00397cc0, 0x00245474)?;
    let t = c.g[V0][0] == 0u64;
    c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
    if t {
    break 'b_2454cc;
    } else {
    c.cycles += 6;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0u64;
    c.g[A2][0] = 0u64;
    call_ptr(c, 0x0039c870, 0x0024548c)?;
    let t = true;
    c.g[A0][0] = c.g[S1][0];
    if t {
    break 'b_2454d0;
    } else {
    break 'b_245494;
    }
    }
    }
    }
    c.cycles += 10;
    c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x45c8));
    call_ptr(c, 0x00317670, 0x002454a0)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S1][0];
    c.g[A3][0] = 0u64;
    call_ptr(c, 0x00397cc0, 0x002454b4)?;
    let t = c.g[V0][0] == 0u64;
    c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
    if t {
    break 'b_2454cc;
    } else {
    c.cycles += 4;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0u64;
    c.g[A2][0] = 0u64;
    call_ptr(c, 0x0039c870, 0x002454cc)?;
    break 'b_2454cc;
    }
    }
    c.cycles += 1;
    c.g[A0][0] = c.g[S1][0];
    break 'b_2454d0;
    }
    c.cycles += 10;
    c.g[A1][0] = 0u64;
    call_ptr(c, 0x00246cf8, 0x002454d8)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = 0x1;
    call_ptr(c, 0x00246cf8, 0x002454e4)?;
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
    c.pc = t; return Ok(());
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::check_lohi;
    use ssx3_ee as ee;
    use crate::difftest::fakes;
    use crate::difftest::put_words;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::put_vec;
    use crate::difftest::SP as STACK;
    use crate::difftest::compare;
    use crate::difftest::fill;
    use crate::difftest::seed_count;
    pub(crate) use crate::game::world::wscriptcache::tests::SEEDS;
    pub(crate) use crate::game::dirtysock::tags2::tests::rng_2 as rng;
    pub(crate) use crate::game::fe::festateloadhint::tests::load_state_dtor_setup;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_LIBRARY;
    pub(crate) use crate::game::movie::movieplayer::tests::globals;
    pub(crate) use crate::game::dirtysock::tags2::tests::dtor_setup;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::bx::bxstring::tests::step;
    pub(crate) use crate::game::bx::bxstring::tests::alloc_ready;
    pub(crate) use crate::game::fe::fepopup3::tests::EMPTY_STRING;
    pub(crate) use crate::game::fe::fepopup3::tests::setup;
    pub(crate) use crate::game::ui::uivector::tests::start;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_4 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::world;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::init_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ4;
    pub(crate) use crate::game::fe::feasyncfile::tests::VT;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_3 as w;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_2 as slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_4 as globals_2;
    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS_3;
    pub(crate) use crate::game::fe::festateloadhint::tests::load_state_fixture;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::gpat;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_8 as be_library_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_4 as pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_5 as world_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::dirtysock::tags2::tests::put_text;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_9 as globals_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_7 as pool_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_9 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base;
    pub(crate) use crate::game::fe::feasyncfile::tests::hash_object_2 as hash_object;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_8 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::w as w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_12 as globals_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::ui;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_12 as heap_init_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_10 as pool_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;


    #[test]
    fn pause_stack_set_base_matches() {
        check(ssx3_ee::s06::f_00244240, pause_stack_set_base, 200, &|c, seed| {
            let mut r = Rng(0x2442_4000 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = GAME_GP as u64;
            let index = seed % 8;
            // Exercise the 32-bit `<< 2` with garbage in the upper half.
            c.g[A0][0] = if seed % 2 == 0 {
                index as u64
            } else {
                0xffff_ffff_0000_0000u64 | index as u64
            };
            let entry = (index << 2).wrapping_add(0x4428f0);
            c.bus.write32(entry, r.next_u32());
            let ptr = 0x100u32;
            c.bus.write32(GAME_GP - 0x69c, ptr);
            c.bus.write32((ptr << 2).wrapping_add(0x5366e8), r.next_u32());
        });
    }


    // ------------------------------------ AI visual-effect processes / menus

    /// A `this` whose sub-effect pointer (`+0x10`) is a fake vtable entry.
    pub(crate) fn fx_process(c: &mut Cpu, r: &mut Rng, sub: bool, slot: u32) -> u32 {
        let obj = ptr(r);
        c.g[A0][0] = obj as u64;
        if sub {
            let inner = ptr(r);
            c.bus.write32(obj + 0x10, inner);
            let vt = vtable(c, r, &[slot]);
            c.bus.write32(inner + 4, vt);
        } else {
            c.bus.write32(obj + 0x10, 0);
        }
        obj
    }


    #[test]
    fn aivfx_process_dtor_matches() {
        covered(ee::s06::f_002448e0, aivfx_process_dtor, 150, &|c, seed| {
            let mut r = machine(c, seed, 0x48e0, 1);
            c.g[A1][0] = (seed % 2) as u64;
            fx_process(c, &mut r, seed % 3 != 0, 8);
        });
    }


    #[test]
    fn aivfx_process_update_render_match() {
        for (gen, hand, slot) in [
            (ee::s06::f_00244948 as fn(&mut Cpu), aivfx_process_update as fn(&mut Cpu), 0x10u32),
            (ee::s06::f_00244978, aivfx_process_render, 0x18),
        ] {
            covered(gen, hand, SEEDS, &|c, seed| {
                let mut r = machine(c, seed, 0x4948, 1);
                fx_process(c, &mut r, true, slot);
            });
        }
    }


    /// The stats stubs ignore their argument, so only the load matters.
    pub(crate) fn stats_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x2449_f8, 0);
        c.g[A0][0] = DATA as u64;
        c.bus.write32(DATA + 0x10, ptr(&mut r));
    }


    #[test]
    fn stats_process_update_matches() {
        check(ee::s06::f_002449f8__generated, stats_process_update, 200, &stats_setup);
    }


    #[test]
    fn stats_process_render_matches() {
        check(ee::s06::f_00244a18__generated, stats_process_render, 200, &stats_setup);
    }


    #[test]
    fn c_game_load_state_dtor_matches() {
        check(ssx3_ee::s06::f_002453f0__generated, c_game_load_state_dtor, 80, &|c, seed| {
            load_state_dtor_setup(c, seed, 0x1f)
        });
    }


    #[test]
    fn c_game_load_state_out_lodge_dtor_matches() {
        check(ssx3_ee::s06::f_00245600__generated, c_game_load_state_out_lodge_dtor, 80, &|c, seed| {
            load_state_dtor_setup(c, seed, 0x20)
        });
    }


    #[test]
    fn c_game_load_state_conquer_dtor_matches() {
        check(ssx3_ee::s06::f_00245708__generated, c_game_load_state_conquer_dtor, 80, &|c, seed| {
            load_state_dtor_setup(c, seed, 0x21)
        });
    }


    #[test]
    fn c_fe_load_hint_state_dtor_matches() {
        check(ssx3_ee::s06::f_00245928__generated, c_fe_load_hint_state_dtor, 80, &|c, seed| {
            load_state_dtor_setup(c, seed, 0x22)
        });
    }


    /// `VfxProcess_Update`/`Render`: process object at +0x10.
    pub(crate) fn vfx_test(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 40, &|c, seed| {
            let _r = machine(c, seed, salt, 1);
            globals(c);
            c.g[A0][0] = sx(DATA);
            c.bus
                .write32(DATA + 0x10, if seed % 2 == 0 { DATA + 0x3000 } else { 0 });
        });
    }


    #[test]
    fn vfx_process_update_matches() {
        vfx_test(ssx3_ee::s06::f_00244478__generated, vfx_process_update, 0xbd);
    }


    #[test]
    fn vfx_process_render_matches() {
        vfx_test(ssx3_ee::s06::f_002444a8__generated, vfx_process_render, 0xbe);
    }


    /// The allocator mutex (`EA_Mutex` at 0x538b00); "held by this thread"
    /// (id 0x42, which the first fake syscall also returns) so the real
    /// `__builtin_delete` takes its fast path.
    pub(crate) const MEM_MUTEX: u32 = 0x0053_8b00;


    pub(crate) fn alloc_setup(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(MEM_MUTEX + 4, 0x42);
        c.bus.write32(LOG_RESULTS, 0x42);
        c.bus.write32(MEM_MUTEX + 8, r.next_u32() % 4);
    }


    // ------------------------------------------------------------ destructors

    /// A deleting-destructor test: `this` is a scratch object, `a1` picks the
    /// deleting / non-deleting branch, and the real `operator delete` runs
    /// (fast path of the allocator mutex).
    pub(crate) fn dtor_case(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 40, &|c, seed| {
            let mut r = machine(c, seed, salt, 1);
            alloc_setup(c, &mut r);
            c.g[A0][0] = ptr(&mut r) as u64;
            c.g[A1][0] = (seed % 2) as u64;
        });
    }


    #[test]
    fn app_module_dtor_matches() {
        dtor_case(ee::s06::f_00243a40__generated, app_module_dtor, 0x243a40);
    }


    #[test]
    fn pre_start_screen_dtor_matches() {
        dtor_case(ee::s06::f_00243a80__generated, pre_start_screen_dtor, 0x243a80);
    }


    #[test]
    fn app_module_vt47d818_dtor_matches() {
        dtor_case(ee::s06::f_00243ae8__generated, app_module_vt47d818_dtor, 0x243ae8);
    }


    #[test]
    fn start_screen_dtor_matches() {
        dtor_case(ee::s06::f_00243b50__generated, start_screen_dtor, 0x243b50);
    }


    #[test]
    fn c_game_load_screen_dtor_matches() {
        dtor_case(ee::s06::f_00243b80__generated, c_game_load_screen_dtor, 0x243b80);
    }


    #[test]
    fn c_fe_load_screen_dtor_matches() {
        dtor_case(ee::s06::f_00243c00__generated, c_fe_load_screen_dtor, 0x243c00);
    }


    #[test]
    fn game_state_dtor_matches() {
        dtor_case(ee::s06::f_00243c80__generated, game_state_dtor, 0x243c80);
    }


    #[test]
    fn game_state_quit_v01_matches() {
        dtor_case(ee::s06::f_00243cf0__generated, game_state_quit_v01, 0x243cf0);
    }


    #[test]
    fn game_state_idle_v01_matches() {
        dtor_case(ee::s06::f_00243d58__generated, game_state_idle_v01, 0x243d58);
    }


    #[test]
    fn game_state_finish_v01_matches() {
        dtor_case(ee::s06::f_00243d88__generated, game_state_finish_v01, 0x243d88);
    }


    #[test]
    fn game_state_reset_ai_v01_matches() {
        dtor_case(ee::s06::f_00243db8__generated, game_state_reset_ai_v01, 0x243db8);
    }


    #[test]
    fn game_state_pre_race_load_v01_matches() {
        dtor_case(ee::s06::f_00243e20__generated, game_state_pre_race_load_v01, 0x243e20);
    }


    #[test]
    fn game_state_countdown_v01_matches() {
        dtor_case(ee::s06::f_00243e50__generated, game_state_countdown_v01, 0x243e50);
    }


    #[test]
    fn game_state_race_v01_matches() {
        dtor_case(ee::s06::f_00243eb8__generated, game_state_race_v01, 0x243eb8);
    }


    #[test]
    fn game_state_reset_v01_matches() {
        dtor_case(ee::s06::f_00243f20__generated, game_state_reset_v01, 0x243f20);
    }


    #[test]
    fn game_state_restart_race_v01_matches() {
        dtor_case(ee::s06::f_00243f50__generated, game_state_restart_race_v01, 0x243f50);
    }


    #[test]
    fn game_state_warp_to_event_v01_matches() {
        dtor_case(ee::s06::f_00243fc0__generated, game_state_warp_to_event_v01, 0x243fc0);
    }


    #[test]
    fn game_state_event_select_v01_matches() {
        dtor_case(ee::s06::f_00244028__generated, game_state_event_select_v01, 0x244028);
    }


    #[test]
    fn game_state_event_switch_load_v01_matches() {
        dtor_case(ee::s06::f_00244060__generated, game_state_event_switch_load_v01, 0x244060);
    }


    #[test]
    fn game_state_pre_race_menu_v01_matches() {
        dtor_case(ee::s06::f_00244090__generated, game_state_pre_race_menu_v01, 0x244090);
    }


    #[test]
    fn game_state_results_v01_matches() {
        dtor_case(ee::s06::f_002440f8__generated, game_state_results_v01, 0x2440f8);
    }


    #[test]
    fn game_state_finish_cinematic_v01_matches() {
        dtor_case(ee::s06::f_00244160__generated, game_state_finish_cinematic_v01, 0x244160);
    }


    #[test]
    fn game_state_replay_playback_v01_matches() {
        dtor_case(ee::s06::f_002441c8__generated, game_state_replay_playback_v01, 0x2441c8);
    }


    #[test]
    fn generic_stream_pause_v01_matches() {
        dtor_case(ee::s06::f_002443c0__generated, generic_stream_pause_v01, 0x2443c0);
    }


    #[test]
    fn gm_handler_dtor_matches() {
        dtor_case(ee::s06::f_002444f0__generated, gm_handler_dtor, 0x2444f0);
    }


    #[test]
    fn gm_handler0_v01_matches() {
        dtor_case(ee::s06::f_00244560__generated, gm_handler0_v01, 0x244560);
    }


    #[test]
    fn gm_handler1_v01_matches() {
        dtor_case(ee::s06::f_00244590__generated, gm_handler1_v01, 0x244590);
    }


    #[test]
    fn gm_handler8_v01_matches() {
        check(ee::s06::f_002445c0__generated, gm_handler8_v01, 60, &|c, seed| {
            dtor_setup(c, seed, 0x2445c0)
        });
    }


    #[test]
    fn gm_handler9_v01_matches() {
        check(ee::s06::f_002445f0__generated, gm_handler9_v01, 60, &|c, seed| {
            dtor_setup(c, seed, 0x2445f0)
        });
    }


    #[test]
    fn gm_handler2_v01_matches() {
        check(ee::s06::f_00244630__generated, gm_handler2_v01, 60, &|c, seed| {
            dtor_setup(c, seed, 0x244630)
        });
    }


    #[test]
    fn gm_handler4_v01_matches() {
        check(ee::s06::f_00244660__generated, gm_handler4_v01, 60, &|c, seed| {
            dtor_setup(c, seed, 0x244660)
        });
    }


    #[test]
    fn gm_handler5_v01_matches() {
        check(ee::s06::f_00244690__generated, gm_handler5_v01, 60, &|c, seed| {
            dtor_setup(c, seed, 0x244690)
        });
    }


    #[test]
    fn gm_handler6_v01_matches() {
        check(ee::s06::f_002446c0__generated, gm_handler6_v01, 60, &|c, seed| {
            dtor_setup(c, seed, 0x2446c0)
        });
    }


    #[test]
    fn gm_handler7_v01_matches() {
        check(ee::s06::f_002446f0__generated, gm_handler7_v01, 60, &|c, seed| {
            dtor_setup(c, seed, 0x2446f0)
        });
    }


    #[test]
    fn gm_handler_unused_v01_matches() {
        check(ee::s06::f_00244720__generated, gm_handler_unused_v01, 60, &|c, seed| {
            dtor_setup(c, seed, 0x244720)
        });
    }


    #[test]
    fn stats_process_dtor_matches() {
        check(
            ee::s06::f_002449a8__generated,
            stats_process_dtor,
            SEEDS_2,
            &|c, seed| {
                let _ = machine(c, seed, 0x2449_a8, 2);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x40);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
                c.bus.write32(this + 0x10, if seed % 3 == 0 { 0 } else { DATA + 0x1_2000 });
            },
        );
    }


    #[test]
    fn ssx_app_dtor_matches() {
        check(ssx3_ee::s06::f_002438f0__generated, ssx_app_dtor, 120, &|c, seed| {
            let mut r = setup(c, seed, 0x2438f0);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = 0;
            for off in [0xcc, 0xc8, 0xc4] {
                c.bus.write32(DATA + off, EMPTY_STRING);
            }
            c.bus.write32(GAME_GP.wrapping_add(0x2a74), 0);
            let _ = &mut r;
        });
    }


    #[test]
    fn generic_stream_pause_v02_matches() {
        check(ssx3_ee::s06::f_002443f0__generated, generic_stream_pause_v02, 200, &|c, seed| {
            let _r = start(c, seed, 0x443f0);
            c.g[A1][0] = (seed as u64) << 32 | 0x1234_5678;
        });
    }


    #[test]
    fn c_fe_state_progressive_on_widget_notify_matches() {
        check(ssx3_ee::s06::f_002453e0__generated, c_fe_state_progressive_on_widget_notify, 200, &|c, seed| {
            let _r = start(c, seed, 0x453e0);
            c.g[A2][0] = match seed % 4 {
                0 => 6,
                1 => 0,
                2 => 0x1_0000_0006,
                _ => 7,
            };
        });
    }


    #[test]
    fn vfx_process_dtor_matches() {
        check(ee::s06::f_00244410__generated, vfx_process_dtor, 12, &|c, seed| {
            machine(c, seed, 0x0024_4410, 2);
            be_library(c);
            world(c);
            pool(c);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = 0;
            c.bus
                .write32(DATA + 0x10, if seed % 2 == 0 { 0 } else { DATA + 0x1000 });
            c.bus.write32(DATA + 0x1000 + 0x44, DATA + 0x2000);
            c.bus.write16(DATA + 0x2000 + 8, 0);
            c.bus.write32(DATA + 0x2000 + 0xc, FAKE);
        });
    }


    #[test]
    fn c_game_load_state_out_lodge_init_matches() {
        check(
            ee::s06::f_00245628__generated,
            c_game_load_state_out_lodge_init,
            6,
            &|c, seed| {
                init_setup(c, seed, 0x0024_5628);
            },
        );
    }


    #[test]
    fn c_game_load_state_on_widget_created_matches() {
        check(
            ee::s06::f_00245598__generated,
            c_game_load_state_on_widget_created,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x245598, 0);
                globals_2(c);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                w(c, OBJ + 8, VT);
                slot(c, VT, 0x48, 0, fakes::FAKE);
                w(c, 0x0053_5c11, if seed % 2 == 0 { 2 } else { 0 });
                w(c, OBJ + 0x38, 0x1234);
                w(c, OBJ2 + 0x38, 0x5678);
            },
        );
    }


    #[test]
    fn c_game_load_state_out_lodge_transition_in_matches() {
        check(
            ssx3_ee::s06::f_00245698__generated,
            c_game_load_state_out_lodge_transition_in,
            SEEDS_3,
            &|c, seed| load_state_fixture(c, seed, 0x245698),
        );
    }


    #[test]
    fn c_game_load_state_conquer_transition_in_matches() {
        check(
            ssx3_ee::s06::f_002458b8__generated,
            c_game_load_state_conquer_transition_in,
            SEEDS_3,
            &|c, seed| load_state_fixture(c, seed, 0x2458b8),
        );
    }


        #[test]
    fn c_fe_state_progressive_init_matches() {
        check(
            ee::s06::f_00245248__generated,
            c_fe_state_progressive_init,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x2452, 0);
                env(c);
                let this = DATA + 0x400;
                w32(
                    c,
                    this + 0x10,
                    if seed % 2 == 0 { 0 } else { DATA + 0x1000 },
                );
                w32(c, this + 0x40, 0);
                c.g[A0][0] = sx(this);
            },
        );
    }


        #[test]
    fn c_game_load_state_transition_in_matches() {
        check(ssx3_ee::s06::f_002454f8__generated, c_game_load_state_transition_in, 20, &|c, seed| {
            let _r = base(c, seed, 0x2454f8, 2);
            let this = DATA + 0x1000;
            let screen = DATA + 0x2000;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = (seed & 1) as u64;
            w(c, this + 0x40, screen);
            hash_object(c, screen, DATA + 0x3000);
            w(c, DATA + 0x3000 + 8, DATA + 0x3100);
            c.bus.write16(DATA + 0x3148, 0x48);
            w(c, DATA + 0x314c, 0);
            w(c, GAME_GP.wrapping_add(0x2a74), DATA + 0x4000);
            w(c, DATA + 0x4000 + 0x38, 3.5f32.to_bits());
            put_text(c, 0x004a_2bd0, b"");
        });
    }


    #[test]
    fn c_game_load_state_init_matches() {
        diff(ee::s06::f_00245418__generated, c_game_load_state_init, 20, &|c, seed| {
            let mut r = fe_base(c, seed, 0x24_5418);
            let _ = r.next_u32();
            c.g[A0][0] = sx(DATA);
            w_2(c, DATA + 0x10, r.next_u32());
        });
    }
}
