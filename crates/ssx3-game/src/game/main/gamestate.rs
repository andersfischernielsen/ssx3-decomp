//! `src/main/gamestate.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::abi::set_word;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::tail_wrapper;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Target;
use crate::call::Unwound;
use crate::events::deliver_event;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::be::be::gp as gp_4;
pub(crate) use crate::game::dirtysock::tagsunk::at_3 as at;
pub(crate) use crate::game::dirtysock::tagsunk::gp_2 as gp;
pub(crate) use crate::game::dirtysock::tagsunk::gp_3 as gp_2;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_6;
pub(crate) use crate::game::fe::feasyncfile::gp_13 as gp_5;
pub(crate) use crate::game::fe::feasyncfile::gpa as gpa_2;
pub(crate) use crate::game::fe::feasyncfile::gpn;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_3;
pub(crate) use crate::game::fe::feasyncfile::p as p_4;
pub(crate) use crate::game::fe::feasyncfile::shl32;
pub(crate) use crate::game::fe::festateloadhint::gp as gp_3;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::festateoptions::gpa;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw_2;
pub(crate) use crate::game::fe::festatetrophyroom::r;
pub(crate) use crate::game::fe::festatetrophyroom::Ops;
pub(crate) use crate::game::movie::movieplayer::lw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2;
pub(crate) use crate::game::scripter::ssxscriptengine::p as p_3;
pub(crate) use crate::game::sound::bankmonitor::leave;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::forward;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0023_4be8,
        game_state_play_script_event,
        "GameState_PlayScriptEvent",
    ),
    (
        0x0023_4c68,
        game_state_countdown_update,
        "GameStateCountdown_Update",
    ),
    (0x0023_4e20, game_state_race_enter, "GameStateRace_Enter"),
    (0x0023_4ef8, game_state_reset_ctor, "GameStateReset_ctor"),
    (
        0x0023_5080,
        game_state_reset_overlay_update,
        "GameStateReset_OverlayUpdate",
    ),
    (
        0x0023_5570,
        game_state_reset_overlay_exit,
        "GameStateReset_OverlayExit",
    ),
    (0x0023_55b0, game_state_reset_v08, "GameStateReset_v08"),
    (
        0x0023_57f8,
        game_state_reset_update,
        "GameStateReset_Update",
    ),
    (
        0x0023_5a18,
        gamestatereset_load_state,
        "GameStateReset_LoadState",
    ),
    (
        0x0023_5cc8,
        game_state_restart_race_update,
        "GameStateRestartRace_Update",
    ),
    (
        0x0023_5f20,
        game_state_restart_race_exit,
        "GameStateRestartRace_Exit",
    ),
    (
        0x0023_5fd0,
        gamestaterestartrace_load_state,
        "GameStateRestartRace_LoadState",
    ),
    (
        0x0023_6198,
        game_state_warp_to_event_update,
        "GameStateWarpToEvent_Update",
    ),
    (
        0x0023_6208,
        game_state_event_select_ctor,
        "GameStateEventSelect_ctor",
    ),
    (
        0x0023_67c8,
        game_state_event_select_load_state,
        "GameStateEventSelect_LoadState",
    ),
    (
        0x0023_6868,
        game_state_event_switch_load_ctor,
        "GameStateEventSwitchLoad_ctor",
    ),
    (
        0x0023_6b38,
        game_state_reset_ai_exit,
        "GameStateResetAI_Exit",
    ),
    (
        0x0023_6bb0,
        game_state_prerace_menu_enter,
        "GameStatePreRaceMenu_Enter",
    ),
    (
        0x0023_6c48,
        game_state_pre_race_menu_update,
        "GameStatePreRaceMenu_Update",
    ),
    (
        0x0023_6c88,
        game_state_pre_race_menu_exit,
        "GameStatePreRaceMenu_Exit",
    ),
    (
        0x0023_6cd8,
        gamestatepreracemenu_resume_race,
        "GameStatePreRaceMenu_ResumeRace",
    ),
    (
        0x0023_6e60,
        game_state_results_update,
        "GameStateResults_Update",
    ),
    (
        0x0023_6ea0,
        game_state_results_exit,
        "GameStateResults_Exit",
    ),
    (
        0x0023_6f08,
        game_state_finish_cinematic_enter,
        "GameStateFinishCinematic_Enter",
    ),
    (
        0x0023_6f40,
        game_state_finish_cinematic_update,
        "GameStateFinishCinematic_Update",
    ),
    (
        0x0023_6f90,
        game_state_finish_cinematic_exit,
        "GameStateFinishCinematic_Exit",
    ),
    (
        0x0023_6fe8,
        game_state_replay_playback_enter,
        "GameStateReplayPlayback_Enter",
    ),
    (
        0x0023_7038,
        game_state_replay_playback_exit,
        "GameStateReplayPlayback_Exit",
    ),
    (0x0023_7060, input_sync_ctor, "InputSync_ctor"),
    (0x0023_7950, input_sync_is_ready, "InputSync_IsReady"),
    (0x0023_79c8, inputsync_apply_input, "InputSync_ApplyInput"),
    (0x0023_7b28, input_sync_pack_sample, "InputSync_PackSample"),
    (
        0x0023_7be8,
        input_sync_unpack_sample,
        "InputSync_UnpackSample",
    ),
    (0x0023_7cb0, game_rand_seed, "GameRand_Seed"),
    (0x0023_7cd8, game_rand_next, "GameRand_Next"),
    (
        0x0027_8f38,
        script_engine_list_prepare,
        "ScriptEngine_ListPrepare",
    ),
    (
        0x0023_5990,
        game_state_reset_save_state,
        "GameStateReset_SaveState",
    ),
    (
        0x0023_5f48,
        game_state_restart_race_save_state,
        "GameStateRestartRace_SaveState",
    ),
    (
        0x0023_6728,
        game_state_event_select_save_state,
        "GameStateEventSelect_SaveState",
    ),
    (
        0x0023_4de0,
        game_state_countdown_save_state,
        "GameStateCountdown_SaveState",
    ),
    (
        0x0023_4eb8,
        game_state_race_save_state,
        "GameStateRace_SaveState",
    ),
    (
        0x0023_61c8,
        game_state_warp_to_event_save_state,
        "GameStateWarpToEvent_SaveState",
    ),
    (
        0x0023_6af8,
        game_state_event_switch_load_save_state,
        "GameStateEventSwitchLoad_SaveState",
    ),
    (
        0x0023_6b70,
        game_state_reset_ai_save_state,
        "GameStateResetAI_SaveState",
    ),
    (
        0x0023_6d60,
        game_state_pre_race_menu_save_state,
        "GameStatePreRaceMenu_SaveState",
    ),
    (
        0x0023_6ec8,
        game_state_results_save_state,
        "GameStateResults_SaveState",
    ),
    (
        0x0023_6fa8,
        game_state_finish_cinematic_save_state,
        "GameStateFinishCinematic_SaveState",
    ),
    (
        0x0023_4ad0,
        game_state_countdown_enter,
        "GameStateCountdown_Enter",
    ),
    (
        0x0023_4f40,
        game_state_reset_overlay_enter,
        "GameStateReset_OverlayEnter",
    ),
    (0x0023_5868, game_state_reset_exit, "GameStateReset_Exit"),
    (
        0x0023_6da0,
        game_state_results_enter,
        "GameStateResults_Enter",
    ),
    (
        0x0023_4e00,
        game_state_countdown_load_state,
        "GameStateCountdown_LoadState",
    ),
    (
        0x0023_4ed8,
        game_state_race_load_state,
        "GameStateRace_LoadState",
    ),
    (
        0x0023_61e8,
        game_state_warp_to_event_load_state,
        "GameStateWarpToEvent_LoadState",
    ),
    (
        0x0023_6b18,
        game_state_event_switch_load_load_state,
        "GameStateEventSwitchLoad_LoadState",
    ),
    (
        0x0023_6b90,
        game_state_reset_ai_load_state,
        "GameStateResetAI_LoadState",
    ),
    (
        0x0023_6d80,
        game_state_pre_race_menu_load_state,
        "GameStatePreRaceMenu_LoadState",
    ),
    (
        0x0023_6ee8,
        game_state_results_load_state,
        "GameStateResults_LoadState",
    ),
    (
        0x0023_6fc8,
        game_state_finish_cinematic_load_state,
        "GameStateFinishCinematic_LoadState",
    ),
    (
        0x0023_7280,
        input_sync_add_remote_pad,
        "InputSync_AddRemotePad",
    ),
    (0x0023_73b8, input_sync_update, "InputSync_Update"),
    (0x0023_7948, input_sync_empty_hook, "InputSync_EmptyHook"),
    (
        0x0023_7140,
        input_sync_add_local_pad,
        "InputSync_AddLocalPad",
    ),
    (
        0x0023_55c0,
        game_state_reset_enter_3,
        "GameStateReset_Enter",
    ),
    (
        0x0023_5aa0,
        oc681_game_state_restart_race_enter,
        "GameStateRestartRace_Enter",
    ),
    (
        0x0023_6058,
        oc681_game_state_warp_to_event_enter,
        "GameStateWarpToEvent_Enter",
    ),
    (
        0x0023_6250,
        game_state_event_select_enter,
        "GameStateEventSelect_Enter",
    ),
    (
        0x0023_6418,
        game_state_event_select_update,
        "GameStateEventSelect_Update",
    ),
    (0x0023_70a0, input_sync_dtor_2, "InputSync_dtor"),
    (
        0x0023_7a98,
        input_sync_send_sample_3,
        "InputSync_SendSample",
    ),
    (
        0x0023_6960,
        game_state_event_switch_load_update,
        "GameStateEventSwitchLoad_Update",
    ),
    (
        0x0023_68a0,
        game_state_event_switch_load_enter_2,
        "GameStateEventSwitchLoad_Enter",
    ),
    (
        0x0023_6af0,
        game_state_event_switch_load_exit_00236af0,
        "GameStateEventSwitchLoad_Exit",
    ),
    (
        0x0023_7030,
        game_state_replay_playback_update_00237030,
        "GameStateReplayPlayback_Update",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::bx::bxstringctor::callee::GAME_MODULE_REQUEST_STATE as GAME_MODULE_REQUEST_STATE_2;

    pub(crate) use crate::game::bx::bxstringctor::callee::SCRIPT_LIST_PREPARE;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::GAME_MODULE_REQUEST_STATE;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::GAME_STATE_PRE_RACE_MENU_RESUME;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::LOAD_INDICATOR_CLEAR_BUSY;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::MENU_SYS_EVENT;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::SCRIPT_MAN_LOAD;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::SCRIPT_MAN_UNLOAD;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::SSX_AUDIO_GET_INSTANCE;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::SSX_AUDIO_PLAY_CHARACTER_TUNE;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::WORLD_VIEW_SET;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::WSCRIPT_MAN_DELETE_ALL_PROCESSES;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::WSCRIPT_MAN_SPAWN_MISSIONS;

    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_PHASE;

    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_SET;

    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_SET_STATE;

    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_LOAD_STATE_2;

    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_RESET;

    pub(crate) use crate::game::fe::festatetrophyroom::callee::SCRIPT_ENGINE_LIST_STOP;
    pub(crate) use crate::game::fe::festatetrophyroom::callee::SCRIPT_ENGINE_UNLOAD_WORLD_SCRIPTS;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_STATE_GOTO_NEXT_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_PACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_UNPACK_SAMPLE;

    pub(crate) use crate::game::sound::bankmonitor::callee::GAME_STATE_GOTO_NEXT_2;

    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RAND_NEXT;

    pub const C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_OPPONENT_CHAR: Callee = Callee {
        addr: 0x001b_1760,
        f: ee::s03::cFEStateOnlineCharacterSelect_ShowOpponentChar,
    };

    pub const C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_CHAR_STATS: Callee = Callee {
        addr: 0x001b_1b98,
        f: ee::s03::cFEStateOnlineCharacterSelect_ShowCharStats,
    };

    pub const MESSAGE_CENTER_REMOVE_MESSAGE: Callee = Callee {
        addr: 0x001e_31b8,
        f: ee::s04::MessageCenter_RemoveMessage,
    };

    pub const MESSAGE_CENTER_COUNT_GROUP_MESSAGES: Callee = Callee {
        addr: 0x001e_2bb8,
        f: ee::s04::MessageCenter_CountGroupMessages,
    };

    pub const MESSAGE_CENTER_GET_MESSAGE_COUNT: Callee = Callee {
        addr: 0x001e_33e8,
        f: ee::s04::MessageCenter_GetMessageCount,
    };

    pub const OV_SCREEN_MESSAGE_CENTER_REFRESH_LIST: Callee = Callee {
        addr: 0x001e_48b0,
        f: ee::s04::OVScreenMessageCenter_RefreshList,
    };

    pub const OV_SCREEN_MESSAGE_CENTER_UPDATE_COUNT_TEXT: Callee = Callee {
        addr: 0x001e_4758,
        f: ee::s04::OVScreenMessageCenter_UpdateCountText,
    };

    pub const OV_SCREEN_MESSAGE_CENTER_UPDATE_SCROLL_ARROWS: Callee = Callee {
        addr: 0x001e_4658,
        f: ee::s04::OVScreenMessageCenter_UpdateScrollArrows,
    };

    pub const INTC_TABLE_CLEAR_SLOTS: Callee = Callee {
        addr: 0x0036_1f60,
        f: ee::s12::IntcTable_ClearSlots,
    };

    pub const INTC_TABLE_INSTALL_VIF_VU_HANDLERS: Callee = Callee {
        addr: 0x0036_1e30,
        f: ee::s12::IntcTable_InstallVifVuHandlers,
    };

    pub const GAME_STATE_LOAD_STATE: Callee = Callee {
        addr: 0x0023_3b28,
        f: ee::s06::GameState_LoadState,
    };

    pub const VD_TEXTURE_SINK_CTOR: Callee = Callee {
        addr: 0x0025_4410,
        f: ee::s06::VD_TextureSink_ctor,
    };

    pub const BE_GET_RIDER_PROFILE: Callee = Callee {
        addr: 0x0014_a0e0,
        f: ee::s01::BE_GetRiderProfile,
    };

    pub const BE_LIBRARY_GET_INSTANCE_3: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::BELibrary_GetInstance,
    };

    pub const BE_LIBRARY_GET_INTERFACE_3: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::BELibrary_GetInterface,
    };

    pub const BE_ECON_INT_GET_TOTAL_EARNED: Callee = Callee {
        addr: 0x0015_09c0,
        f: ee::s01::BEEconInt_GetTotalEarned,
    };

    pub const BOLTS_FIND_BOLT_INDEX: Callee = Callee {
        addr: 0x0014_d448,
        f: ee::s01::Bolts_FindBoltIndex,
    };

    pub const SCRIPT_CMD_QUEUE_AT_2: Callee = Callee {
        addr: 0x0028_3d70,
        f: ee::s07::ScriptCmdQueue_At,
    };

    pub const SCRIPT_LIST_IS_MOVIE_ENTRY: Callee = Callee {
        addr: 0x0027_7dd8,
        f: ee::s07::ScriptList_IsMovieEntry,
    };

    pub const SCRIPT_ENGINE_GET_PROCESS_IN_SEQ: Callee = Callee {
        addr: 0x0028_2bf0,
        f: ee::s07::ScriptEngine_GetProcessInSeq,
    };

    pub const SECTION_ALLOCATOR_CTOR: Callee = Callee {
        addr: 0x003a_77b0,
        f: ee::s13::SectionAllocator_ctor,
    };

    pub const UI_STATE_CTOR_3: Callee = Callee {
        addr: 0x0039_e2a0,
        f: ee::s13::UIState_ctor,
    };

    pub const MEMCARD_STATE_BASE_INSTALL_CALLBACKS: Callee = Callee {
        addr: 0x001d_5488,
        f: ee::s04::MemCardStateBase_InstallCallbacks,
    };

    pub const UI_STATE_MGR_FIND_STATE: Callee = Callee {
        addr: 0x0039_f9d8,
        f: ee::s13::UIStateMgr_FindState,
    };

    pub const UI_STATE_SHUTDOWN_2: Callee = Callee {
        addr: 0x0039_e4a0,
        f: ee::s13::UIState_Shutdown,
    };

    pub const SCRIPT_ENGINE_LIST_STOP_3: Callee = Callee {
        addr: 0x0027_90a0,
        f: ee::s07::ScriptEngine_ListStop,
    };

    pub const SCRIPT_ENGINE_LIST_START: Callee = Callee {
        addr: 0x0027_8f68,
        f: ee::s07::ScriptEngine_ListStart,
    };

    pub const SSX_AUDIO_SYSTEM_GET_INSTANCE_2: Callee = Callee {
        addr: 0x0028_b180,
        f: ee::s07::SSXAudioSystem_GetInstance,
    };

    pub const SSX_AUDIO_SYSTEM_RESET_RACE_SOUNDS: Callee = Callee {
        addr: 0x0028_71b0,
        f: ee::s07::SSXAudioSystem_ResetRaceSounds,
    };

    pub const SCRIPT_ENGINE_GET_SCRIPT_OBJ: Callee = Callee {
        addr: 0x0027_d320,
        f: ee::s07::ScriptEngine_GetScriptObj,
    };

    pub const SCRIPT_OBJ_CAM_IS_BOUND_TO_PROCESS: Callee = Callee {
        addr: 0x0027_d790,
        f: ee::s07::ScriptObjCam_IsBoundToProcess,
    };

    pub const SCRIPT_ENGINE_IS_OBJ_BOUND_TO_PROCESS: Callee = Callee {
        addr: 0x0027_d210,
        f: ee::s07::ScriptEngine_IsObjBoundToProcess,
    };

    pub const SCRIPT_OBJ_CAM_HAS_BINDING: Callee = Callee {
        addr: 0x0028_2ef0,
        f: ee::s07::ScriptObjCam_HasBinding,
    };

    pub const SPEECH_INSTANCE_DTOR: Callee = Callee {
        addr: 0x002a_fb38,
        f: ee::s08::SpeechInstance_dtor,
    };

    pub const SSX_AUDIO_SYSTEM_IS_DJ_SPEECH_ON: Callee = Callee {
        addr: 0x0029_f0d8,
        f: ee::s08::SSXAudioSystem_IsDJSpeechOn,
    };

    pub const SPEECH_SYSTEM_SET_EVENT_CONTEXT: Callee = Callee {
        addr: 0x002b_1458,
        f: ee::s08::SpeechSystem_SetEventContext,
    };

    pub const SSX_AUDIO_SYSTEM_GET_LANGUAGE_MASK: Callee = Callee {
        addr: 0x0029_f198,
        f: ee::s08::SSXAudioSystem_GetLanguageMask,
    };

    pub const MENU_SYS_PAGE_VALIDATE_SELECTION: Callee = Callee {
        addr: 0x002c_aab0,
        f: ee::s09::MenuSysPage_ValidateSelection,
    };

    pub const MENU_SYS_PAGE_SELECT_NEXT: Callee = Callee {
        addr: 0x002c_ab08,
        f: ee::s09::MenuSysPage_SelectNext,
    };

    pub const MENU_SYS_PAGE_SELECT_PREV: Callee = Callee {
        addr: 0x002c_abe0,
        f: ee::s09::MenuSysPage_SelectPrev,
    };

    pub const MENU_SYS_ITEM_HANDLE_INPUT: Callee = Callee {
        addr: 0x002c_a2b0,
        f: ee::s09::MenuSysItem_HandleInput,
    };

    pub const MENU_SYS_ITEM_IS_DISABLED: Callee = Callee {
        addr: 0x002c_a3b0,
        f: ee::s09::MenuSysItem_IsDisabled,
    };

    pub const MENU_SYS_ITEM_GET_INPUT_CODE: Callee = Callee {
        addr: 0x002c_a3e8,
        f: ee::s09::MenuSysItem_GetInputCode,
    };

    pub const SCRIPT_BINDING_PAUSE_SEQ: Callee = Callee {
        addr: 0x0028_3038,
        f: ee::s07::ScriptBinding_PauseSeq,
    };

    pub const SCRIPT_BINDING_RESUME_SEQ: Callee = Callee {
        addr: 0x0028_3060,
        f: ee::s07::ScriptBinding_ResumeSeq,
    };

    pub const GAME_ANIM_MAP_UNREGISTER_BANK: Callee = Callee {
        addr: 0x0031_4fe8,
        f: ee::s10::GameAnimMap_UnregisterBank,
    };

    pub const ANIM_DATA_BASE_FREE_CHAN_CACHES: Callee = Callee {
        addr: 0x0031_1110,
        f: ee::s10::AnimDataBase_FreeChanCaches,
    };

    pub const ADF_DATA_SOURCE_DTOR: Callee = Callee {
        addr: 0x0031_12c8,
        f: ee::s10::AdfDataSource_dtor,
    };

    pub const C_KEYBOARD_POPUP_SET_KEY_ENABLED: Callee = Callee {
        addr: 0x001c_e3c8,
        f: ee::s04::cKeyboardPopup_SetKeyEnabled,
    };

    pub const RIDER_CLEAR_NEARBY_LIGHTS: Callee = Callee {
        addr: 0x0012_2658,
        f: ee::s00::Rider_ClearNearbyLights,
    };

    pub const SNDSTRM_CLOSE: Callee = Callee {
        addr: 0x003b_7838,
        f: ee::s13::SNDSTRM_Close,
    };

    pub const EA_MEM_FREE_4: Callee = Callee {
        addr: 0x0031_7e98,
        f: ee::s10::EA_MemFree,
    };

    pub const BUILTIN_DELETE_4: Callee = Callee {
        addr: 0x0031_7e50,
        f: ee::s10::__builtin_delete,
    };

    pub const NET_FE_STATE_OPEN_KEYBOARD_TEXT_ID: Callee = Callee {
        addr: 0x001a_9150,
        f: ee::s03::NetFEState_OpenKeyboardTextId,
    };

    pub const NET_FE_STATE_ON_CHILD_EVENT_3: Callee = Callee {
        addr: 0x001a_97b8,
        f: ee::s03::NetFEState_OnChildEvent,
    };

    pub const SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN_PTR: Callee = Callee {
        addr: 0x0028_7968,
        f: ee::s07::SSXAudioSystem_GetChannelGainPtr,
    };

    pub const BANK_MONITOR_STOP_HANDLE: Callee = Callee {
        addr: 0x002a_d5f0,
        f: ee::s08::BankMonitor_StopHandle,
    };

    pub const UI_LIST_FIRST_2: Callee = Callee {
        addr: 0x0039_7840,
        f: ee::s13::UIList_First,
    };

    pub const UI_LIST_NODE_DTOR_BODY: Callee = Callee {
        addr: 0x003a_4868,
        f: ee::s13::UIListNode_dtorBody,
    };

    pub const C_GAME_COMM_RECV_GAME_PACKET: Callee = Callee {
        addr: 0x0025_8160,
        f: ee::s06::cGameComm_RecvGamePacket,
    };

    pub const C_FE_STATE_BRAGGING_RIGHTS_INIT_PLAYER_ROWS: Callee = Callee {
        addr: 0x0019_26f0,
        f: ee::s02::cFEStateBraggingRights_InitPlayerRows,
    };

    // Cross-shard calls the generator emitted as `call_indirect` (direct
    // `jal`s whose target lives in another shard).
    pub const EA_MEM_ALLOC_NAMED_3: u32 = 0x0031_7d70;

    pub const MEM_MOVE: u32 = 0x0041_610c;

    pub const STR_COPY: u32 = 0x0041_66f4;

    pub const TAG_FIELD_SET_STRING: u32 = 0x003e_c180;

    pub const STR_N_COPY: u32 = 0x0041_6ce0;

    pub const SPHERE_TREE_COLLIDE_SPHERE_TREE: u32 = 0x0032_cb58;

    pub const SPHERE_TREE_COLLIDE_AABB: u32 = 0x0032_c928;

    pub const SPCH_SHUTDOWN: u32 = 0x003d_67f8;

    pub const SPCH_MAKE_ID: u32 = 0x003d_8008;

    pub const MEM_FREE: u32 = 0x0025_23a8;

    pub const STR_HASH: u32 = 0x0031_7670;
}

/// `GetHashValue32` (0x317670) and the UI object lookups, which the generator
/// emitted as cross-shard `call_indirect`s.
pub(crate) mod table_callee {
    pub(crate) use crate::game::fe::festaterewards::table_callee::AI_SET_AI_STATE;

    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET_INTERFACE_FV;

    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_SET_STATE;

    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_OPEN_FINISH_SCREEN;
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod table {

    pub(crate) use crate::game::fe::feasyncfile::table::BUILTIN_DELETE;

    pub(crate) use crate::game::fe::feasyncfile::table::EA_MEM_FREE;
}

/// Addresses the generator emitted as `call_indirect` (outside the shard's
/// own direct-call set, so no readable alias is needed here).
pub(crate) mod addr {}

/// `GameStateReset *GameStateReset_ctor(GameStateReset *self)` (0x234ef8):
/// state words `+0=4`, `+4=0`, `+0xc=0x47d428`, `+0x10=5`, `+0x14=0`,
/// `+0x18=0`, `+0x1c=0`.
pub fn game_state_reset_ctor(c: &mut Cpu) {
    const VTABLE: u32 = 0x48_0000 - 0x2bd8;
    let this = Addr(abi::arg(c, 0));
    c.cycles += 18;
    this.sw(c, 0);
    this.off(0xc).sw(c, sx(0x48_0000 - 0x2918));
    let vtable = sx(VTABLE);
    c.g[V1][0] = vtable;
    Addr((c.g[GP][0] as u32).wrapping_sub(0x488)).sw(c, 0);
    c.g[A1][0] = 4;
    c.g[A2][0] = 5;
    this.off(0xc).sw(c, vtable);
    c.g[V0][0] = c.g[A0][0];
    this.sw(c, 4);
    this.off(0x10).sw(c, 5);
    this.off(0x4).sw(c, 0);
    this.off(0x14).sw(c, 0);
    this.off(0x18).sw(c, 0);
    this.off(0x1c).sw(c, 0);
    abi::ret(c);
}

/// `GameStateEventSelect *GameStateEventSelect_ctor(GameStateEventSelect *self)`
/// (0x236208): state words `+0=4`, `+4=0`, `+8=0`, `+0xc=0x47d320`,
/// `+0x14=0`, `+0x18=0x17`.
pub fn game_state_event_select_ctor(c: &mut Cpu) {
    const VTABLE: u32 = 0x48_0000 - 0x2ce0;
    let this = Addr(abi::arg(c, 0));
    c.cycles += 17;
    this.sw(c, 0);
    this.off(0xc).sw(c, sx(0x48_0000 - 0x2918));
    let vtable = sx(VTABLE);
    c.g[V1][0] = vtable;
    Addr((c.g[GP][0] as u32).wrapping_sub(0x488)).sw(c, 0);
    c.g[A1][0] = 4;
    c.g[A2][0] = 0x17;
    this.off(0xc).sw(c, vtable);
    c.g[V0][0] = c.g[A0][0];
    this.sw(c, 4);
    this.off(0x18).sw(c, 0x17);
    this.off(0x4).sw(c, 0);
    this.off(0x8).sw(c, 0);
    this.off(0x14).sw(c, 0);
    abi::ret(c);
}

/// `GameStateEventSwitchLoad *GameStateEventSwitchLoad_ctor(
/// GameStateEventSwitchLoad *self)` (0x236868): state words `+0=0xa`, `+4=0`,
/// `+0xc=0x47d2c8`.
pub fn game_state_event_switch_load_ctor(c: &mut Cpu) {
    const VTABLE: u32 = 0x48_0000 - 0x2d38;
    let this = Addr(abi::arg(c, 0));
    c.cycles += 13;
    this.sw(c, 0);
    this.off(0xc).sw(c, sx(0x48_0000 - 0x2918));
    let vtable = sx(VTABLE);
    c.g[V1][0] = vtable;
    Addr((c.g[GP][0] as u32).wrapping_sub(0x488)).sw(c, 0);
    c.g[V0][0] = 0xa;
    this.sw(c, 0xa);
    this.off(0xc).sw(c, vtable);
    c.g[V0][0] = c.g[A0][0];
    this.off(0x4).sw(c, 0);
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Input sync.

/// `InputSync *InputSync_ctor(InputSync *self, uint a1, int divisor, uint a3)`
/// (0x237060): `self+0x10 = 0`, `self+0x8 = divisor`, `self+0 = a1*divisor`,
/// `self+4 = (60/divisor)*divisor` (both products signed 32-bit, low word);
/// the divide and multiplies also leave LO/HI set.
pub fn input_sync_ctor(c: &mut Cpu) {
    const B: u32 = 0x4;
    const C: u32 = 0x8;
    const D: u32 = 0xc;
    const E: u32 = 0x10;
    let this = Addr(abi::arg(c, 0));
    c.cycles += 13;
    // v1 is pre-loaded with the constant 60, the dividend of the `div`.
    c.g[V1][0] = 0x3c;
    this.off(B).sw(c, c.g[A1][0]); // a1 (overwritten at the end)
    let (q, _) = abi::div(c, 0x3c, c.g[A2][0] as u32); // div v1, a2
    this.off(C).sw(c, c.g[A2][0]);
    this.off(D).sw(c, 0);
    c.g[V0][0] = c.g[A0][0];
    this.off(E).sw(c, 0);
    c.g[V1][0] = sx(q);
    // Two scratch multiplies: `mult a1*a2` leaves the low word in a3, then
    // `mult v1*a2` leaves its low word back in v1 (and LO/HI).
    let a3 = abi::mult(c, c.g[A1][0] as u32, c.g[A2][0] as u32);
    c.g[A3][0] = sx(a3);
    let v = abi::mult(c, q, c.g[A2][0] as u32);
    c.g[V1][0] = sx(v);
    this.sw(c, c.g[A3][0]);
    c.cycles += 2;
    this.off(B).sw(c, c.g[V1][0]);
    abi::ret(c);
}

/// `unsigned int GameRand_Next(void)` (0x00237cd8): next value of the global
/// game RNG at `D_004C9548`.
pub fn game_rand_next(c: &mut Cpu) {
    forward(
        c,
        8,
        0x10,
        |c| c.g[A0][0] = sx(0x4d_0000u32.wrapping_sub(0x6ab8)),
        callee::RAND_NEXT,
        0x0023_7cec,
    )
}

/// `void GameStateResults_Exit(GameStateResults* s)` (0x236ea0): close every
/// UI state below the results state manager
/// (`UIStateMgr_CloseAll(D_004A2C68->+0x48 + 0x18)`).
pub fn game_state_results_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = c.lw(r32(c, GP).wrapping_sub(0x488));
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(0x48));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x18));
        call_ptr(c, 0x0039_f840, 0x0023_6eb8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GameRand_Seed(unsigned seed)` (0x237cb0): seed the pseudo random
/// generator at `D_004C9548`.
pub fn game_rand_seed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[A1][0] = c.g[A0][0];
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = 0x004d_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(r32(c, A0).wrapping_sub(0x6ab8));
        call_ptr(c, 0x0031_7958, 0x0023_7cc8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GameStateRestartRace_Exit(void)` (0x235f20): clear flag 3 of the
/// race manager's load indicator.
pub fn game_state_restart_race_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = Addr(gp(c, -0x848)).lw(c);
        frame.save_ra(c, 0);
        c.g[A1][0] = 3;
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x84)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x94)).lw(c);
        call_ptr(c, callee::LOAD_INDICATOR_CLEAR_BUSY, 0x0023_5f3c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GameStateReplayPlayback_Exit(void)` (0x237038): tear down the
/// playback world script manager's processes and respawn its missions.
pub fn game_state_replay_playback_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(gp(c, 0xce8)).lw(c);
        frame.save_ra(c, 0);
        call_ptr(c, callee::WSCRIPT_MAN_DELETE_ALL_PROCESSES, 0x0023_704c)?;
        c.g[A0][0] = Addr(gp(c, 0xce8)).lw(c);
        call_ptr(c, callee::WSCRIPT_MAN_SPAWN_MISSIONS, 0x0023_7054)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `GameStateWarpToEvent_Update()` 0x236198: request the game module's race
/// state (`world->+0x84`, param 1).
pub fn game_state_warp_to_event_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        let world = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[V0][0] = world;
        frame.save_ra(c, 0);
        c.g[A1][0] = 1;
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A0][0] = lw_at(c, world, 0x84);
        callee::GAME_MODULE_REQUEST_STATE_2.call(c, 0x0023_61b8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The `ScriptEngine_List*` helpers: address list `i` of the engine's rider
/// script array (`*(engine + 0x54c)` + `i * 0xcc`).
pub(crate) fn script_list_call(c: &mut Cpu, ret: u32, second: Option<u64>, list: Callee) {
    guest(c, |c| {
        c.cycles += 11;
        let engine = abi::arg(c, 0);
        let index = abi::arg(c, 1);
        c.g[V0][0] = 0xcc;
        let frame = Frame::open(c, 0x10);
        let stride = abi::mult(c, index, 0xcc);
        c.g[V1][0] = sx(stride);
        frame.save_ra(c, 0);
        match second {
            Some(v) => c.g[A1][0] = v,
            None => c.g[A1][0] = 0,
        }
        let base = c.lw(engine.wrapping_add(0x54c));
        c.g[A0][0] = sx((base as u32).wrapping_add(stride));
        list.call(c, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `ScriptEngine_ListPrepare(engine, i)` 0x278f38: prepare list `i`.
pub fn script_engine_list_prepare(c: &mut Cpu) {
    script_list_call(c, 0x0027_8f58, None, callee::SCRIPT_LIST_PREPARE)
}

/// `void GameStateResetAI_Exit(GameState* self)` (0x236b38): tell the race
/// manager's AI controller (virtual slot +0x2c) to reset.
pub fn game_state_reset_ai_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x488)).lw(c);
        frame.save_ra(c, 0);
        c.g[A1][0] = 3;
        c.g[A2][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(0xcc)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x28)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x2c)).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0023_6b60)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void GameStateFinishCinematic_Enter()` (0x236f08): hand the audio system to
/// the character-tune player and flag the race manager's cinematic end.
pub fn game_state_finish_cinematic_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::SSX_AUDIO_GET_INSTANCE.call(c, 0x0023_6f18)?;
        c.g[A0][0] = c.g[V0][0];
        callee::SSX_AUDIO_PLAY_CHARACTER_TUNE.call(c, 0x0023_6f20)?;
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        c.g[A1][0] = 1;
        frame.restore_ra(c, 0);
        c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x84).lw(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
        let v1 = c.g[V1][0];
        let a1 = c.g[A1][0];
        Addr(v1 as u32).off(0x8c).sw(c, a1);
        frame.close(c);
        Ok(())
    })
}

/// `void GameStateReset_OverlayExit(GameStateReset* self)` (0x235570): unload
/// the world scripts, stop the script lists, and move to state 5.
pub fn game_state_reset_overlay_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let world = c.lw_r_2(A0, gp_2(c, -0x84c));
        let _ = world;
        callee::SCRIPT_ENGINE_UNLOAD_WORLD_SCRIPTS.call(c, 0x0023_5588)?;
        let world = c.lw_r_2(A0, gp_2(c, -0x84c));
        let _ = world;
        c.g[A1][0] = 1;
        callee::SCRIPT_ENGINE_LIST_STOP.call(c, 0x0023_5594)?;
        c.g[V0][0] = 5;
        frame.restore_ra(c, 0);
        Addr(r(c, S0).wrapping_add(0x10)).sw(c, 5);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void GameStateResults_Update(GameStateResults* self)` (0x236E60): once
/// the results screen reports itself ready (field +0x8 == 1), ask the game
/// module for state 0xE (leaving the results screen).
pub fn game_state_results_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 1;
        frame.save_ra(c, 0);
        let state = c.lw_r_2(V1, r(c, A0).wrapping_add(8));
        frame.restore_ra(c, 0);
        if state == c.g[V0][0] {
            c.cycles += 7;
            let world = c.lw_r_2(V0, gp_2(c, -0x848));
            c.g[A1][0] = 0xe;
            c.g[A2][0] = 2;
            c.g[A3][0] = 0;
            let module = c.lw_r_2(A0, (world as u32).wrapping_add(0x84));
            let _ = module;
            callee::GAME_MODULE_REQUEST_STATE.call(c, 0x0023_6e90)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Game state / mode dispatch.

/// `void GameStatePreRaceMenu_Update(GameState* self)` (0x236c48): leave the
/// pre-race menu when the lobby manager (gp-0x204) is gone or its `+4` is
/// clear, and when the state's `+8` is 1, advance to the next state.
pub fn game_state_pre_race_menu_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let gate = Addr(gp(c, -0x204)).lw(c);
        c.g[V0][0] = gate;
        let mut epilogue = false;
        if gate != 0 {
            c.cycles += 3;
            let next = Addr(gate as u32).off(4).lw(c);
            c.g[V0][0] = next;
            epilogue = next != 0;
            frame.restore_ra(c, 0);
        }
        if !epilogue {
            c.cycles += 4;
            c.g[V1][0] = Addr(abi::arg(c, 0)).off(8).lw(c);
            c.g[V0][0] = 1;
            let not_first = c.g[V1][0] != c.g[V0][0];
            frame.restore_ra(c, 0);
            if !not_first {
                c.cycles += 3;
                callee::GAME_STATE_GOTO_NEXT_2.call(c, 0x0023_6c7c)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        leave(c, 0x10);
        Ok(())
    })
}

/// `void GameStateReplayPlayback_Enter(void)` (0x236fe8): start a replay
/// playback: load the script, unload it, quiet the race manager's movie
/// player and step the replay manager.
pub fn game_state_replay_playback_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.lw(gp(c, 0xce8));
        frame.save_ra(c, 0);
        call_ptr(c, callee::SCRIPT_MAN_LOAD, 0x0023_6ffc)?;
        c.g[A0][0] = c.lw(gp(c, 0xce8));
        call_ptr(c, callee::SCRIPT_MAN_UNLOAD, 0x0023_7004)?;
        c.g[V0][0] = c.lw(gp(c, -0x848));
        c.g[A1][0] = 0;
        c.g[V1][0] = c.lw(at(c, V0, 0x84));
        c.g[A0][0] = c.lw(at(c, V1, 0x94));
        call_ptr(c, callee::WORLD_VIEW_SET, 0x0023_7018)?;
        c.g[A0][0] = 0x11;
        call_ptr(c, callee::MENU_SYS_EVENT, 0x0023_7020)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void GameStateFinishCinematic_Update(GameState* self)` (0x236f40): while
/// the script engine is idle (status 0 or 5), advance to the next state.
pub fn game_state_finish_cinematic_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(gp(c, -0x84c)).lw(c);
        call_ptr(c, 0x0027_9220, 0x0023_6f5c)?;
        let idle = c.g[V0][0] == 0;
        c.g[V1][0] = 0;
        if !idle {
            c.cycles += 2;
            c.g[V0][0] ^= 5;
            c.g[V1][0] = (c.g[V0][0] > 0) as u64;
        }
        c.cycles += 2;
        if c.g[V1][0] != 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            callee::GAME_STATE_GOTO_NEXT_3.call(c, 0x0023_6f7c)?;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `D_004A2A50` / `D_004A2A54`: the pre-race menu stack top and depth.
pub(crate) const PRE_RACE_STACK_TOP_GP: i32 = -0x6a0;

pub(crate) const PRE_RACE_STACK_DEPTH_GP: i32 = -0x69c;

// ---------------------------------------------------------------------------
// Game state.

/// `void GameStatePreRaceMenu_Exit(GameStatePreRaceMenu* s)` (0x236c88): pop
/// the pre-race menu stack (`D_004A2A54` deep, `D_005366E8` indexes into
/// `D_004428F0`) into the current-menu global and resume the race.
/// 20 cycles plus the callee.
pub fn game_state_pre_race_menu_exit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let depth = c.lw(gp(c, PRE_RACE_STACK_DEPTH_GP));
        c.g[V0][0] = depth;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx(0x0053_66e8);
        frame.save_ra(c, 0);
        let new_depth = sx(r32(c, V0).wrapping_sub(1));
        c.g[V0][0] = new_depth;
        c.g[V1][0] = sx(r32(c, V0) << 2);
        c.sw(gp(c, PRE_RACE_STACK_DEPTH_GP), c.g[V0][0]);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
        c.g[A2][0] = sx(0x0044_28f0);
        c.g[V0][0] = c.lw(r32(c, V1));
        c.g[V0][0] = sx(r32(c, V0) << 2);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, A2)));
        c.g[V1][0] = c.lw(r32(c, V0));
        c.sw(gp(c, PRE_RACE_STACK_TOP_GP), c.g[V1][0]);
        callee::GAME_STATE_PRE_RACE_MENU_RESUME.call(c, 0x0023_6ccc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int GameStateReset_v08(GameState* state)` (0x2355b0): `state->+0x10 == 5`.
pub fn game_state_reset_v08(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = Addr(r32(c, A0).wrapping_add(0x10)).lw(c);
    c.g[V0][0] ^= 0x5;
    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
    abi::ret(c);
}

/// `void GameStateFinishCinematic_Exit(void)` (0x236f90): clear the cinematic
/// state word (`race->+0x8c`) of the current race.
pub fn game_state_finish_cinematic_exit(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = Addr(gpa(c, -0x848)).lw(c);
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x8c), 0);
    abi::ret(c);
}

/// `lw rt, -off(gp)`.
fn gpw(c: &mut Cpu, off: u32) -> u64 {
    c.lw((c.g[GP][0] as u32).wrapping_sub(off))
}

/// `ScriptEngine_ListAddChoice(engine, 1, choice, 3, 0, -1, -1, -1)` with
/// `a0` already the script engine, returning to `ret`.
fn reset_overlay_add_choice(c: &mut Cpu, choice: u64, ret: u32) -> Flow {
    c.g[A1][0] = 0x1;
    c.g[A2][0] = choice;
    c.g[A3][0] = 0x3;
    c.g[T0][0] = 0;
    c.g[T1][0] = u64::MAX;
    c.g[T2][0] = u64::MAX;
    c.g[T3][0] = u64::MAX;
    call_ptr(c, 0x0027_8e50, ret)
}

/// `void GameStateReset_OverlayUpdate(GameStateReset* self)` (0x235080): the
/// reset overlay's load sequence. `+0x18` counts updates since the overlay
/// opened; `+0x10` is the step:
///
/// - 0: once the script engine has loaded, prepare list 0 and (for the peak
///   transition, `+0x14` set) queue the peak transition script and its two
///   choices (and the peak music) on list 1, else the two reset choices when
///   the rider's state is 14..16 and `+0x1c8` is set; then step 1.
/// - 1: wait until neither script list is in state 1; step 2.
/// - 2: once every computer and actor rider has a render slot, restart the
///   game mode (unless the world type is 6..11), hand the AI rider count and
///   characters to the BE interfaces, rebuild the computer riders (and the
///   actor riders when the world type and sub-type are 0); step 3.
/// - 3: load the riders one step at a time; once all are loaded, reset them
///   (world type 0) or reorder them (type 5, sub-type 0); step 4.
/// - 4: the peak transition holds the overlay for at least 180 frames (3 s)
///   since it opened; step 5.
///
/// Tick rate: `+0x18` goes up once per tick, so the 180-frame hold is
/// compared in ticks ([`crate::tick::ticks_from_frames60`]; 180 at 60 Hz).
pub fn game_state_reset_overlay_update(c: &mut Cpu) {
    /// How a step ends: store `v0` as the new step (0x23553c), leave the step
    /// (0x235540), or leave with `s0` already reloaded (0x235544).
    enum End {
        Store,
        Leave,
        Reloaded,
    }
    guest(c, |c| {
        c.cycles += 17;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x90));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.g[S6][0] = c.g[A0][0];
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        let this = c.g[S6][0] as u32;
        c.g[V0][0] = c.lw(this.wrapping_add(0x18));
        c.g[A0][0] = c.lw(this.wrapping_add(0x10));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.g[V1][0] = (c.g[A0][0] < 0x6u64) as u64;
        c.sw(this.wrapping_add(0x18), c.g[V0][0]);
        let step = c.g[A0][0];
        let end = if step >= 6 {
            End::Leave
        } else {
            // The jump table at 0x47c060.
            c.cycles += 7;
            c.g[V0][0] = sx(0x0047_c060);
            c.g[V1][0] = sx(((step as u32) << 2).wrapping_add(0x0047_c060));
            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
            match step {
                0 => 'step0: {
                    c.cycles += 4;
                    c.g[A0][0] = gpw(c, 0x84c);
                    call_ptr(c, 0x0027_8d58, 0x0023_50e8)?;
                    let loading = c.g[V0][0] == 0;
                    c.g[A0][0] = gpw(c, 0x84c);
                    if loading {
                        break 'step0 End::Leave;
                    }
                    c.cycles += 10;
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x0027_8f38, 0x0023_50f8)?;
                    c.g[V1][0] = gpw(c, 0x848);
                    c.g[A0][0] = c.lw(this.wrapping_add(0x14));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1bc));
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xe));
                    c.g[A1][0] = (c.g[A1][0] < 0x3u64) as u64;
                    if c.g[A0][0] == 0 {
                        // The plain reset: the two choices when the rider's
                        // state is 14..16 and `+0x1c8` is set.
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x1c8));
                        let off = c.g[V0][0] == 0;
                        c.g[V0][0] = 0x1;
                        if off {
                            break 'step0 End::Store;
                        }
                        c.cycles += 2;
                        let state_ok = c.g[A1][0] != 0;
                        c.g[A0][0] = gpw(c, 0x84c);
                        if !state_ok {
                            break 'step0 End::Store;
                        }
                        c.cycles += 19;
                        reset_overlay_add_choice(c, 0x10, 0x0023_5254)?;
                        c.g[A0][0] = gpw(c, 0x84c);
                        reset_overlay_add_choice(c, 0x11, 0x0023_5278)?;
                        c.g[V0][0] = 0x1;
                        break 'step0 End::Store;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0xe;
                    let state14 = c.g[V1][0] == 0xe;
                    c.g[V1][0] ^= 0xf;
                    if state14 {
                        // The peak transition script, its music, and the two
                        // peak choices.
                        c.cycles += 33;
                        c.g[A0][0] = gpw(c, 0x84c);
                        c.g[A1][0] = 0x1;
                        c.g[A2][0] = 0x1d;
                        c.g[A3][0] = 0x3;
                        call_ptr(c, 0x0027_8e20, 0x0023_5138)?;
                        call_ptr(c, 0x0028_b180, 0x0023_5140)?;
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x0028_cd48, 0x0023_5148)?;
                        call_ptr(c, 0x0028_b180, 0x0023_5150)?;
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x118));
                        call_ptr(c, 0x002b_3a70, 0x0023_5158)?;
                        c.g[A0][0] = gpw(c, 0x84c);
                        reset_overlay_add_choice(c, 0, 0x0023_517c)?;
                        c.g[A0][0] = gpw(c, 0x84c);
                        reset_overlay_add_choice(c, 1, 0x0023_51a0)?;
                    } else {
                        // Script 0x1e (state 15) or 0x1f, and the two reset
                        // choices.
                        c.cycles += 26;
                        c.g[V0][0] = 0x1f;
                        c.g[A0][0] = gpw(c, 0x84c);
                        c.g[A2][0] = if c.g[V1][0] != 0 { 0x1f } else { 0x1e };
                        c.g[A1][0] = 0x1;
                        c.g[A3][0] = 0x3;
                        call_ptr(c, 0x0027_8e20, 0x0023_51c4)?;
                        c.g[A0][0] = gpw(c, 0x84c);
                        reset_overlay_add_choice(c, 0x10, 0x0023_51e8)?;
                        c.g[A0][0] = gpw(c, 0x84c);
                        reset_overlay_add_choice(c, 0x11, 0x0023_520c)?;
                    }
                    c.g[A0][0] = gpw(c, 0x84c);
                    c.cycles += 4;
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0027_8f38, 0x0023_5218)?;
                    c.g[V0][0] = 0x1;
                    End::Store
                }
                1 => {
                    // Both script lists out of state 1.
                    c.cycles += 7;
                    c.g[A0][0] = gpw(c, 0x84c);
                    c.g[A1][0] = 0;
                    c.g[S0][0] = 0;
                    call_ptr(c, 0x0027_91d8, 0x0023_5290)?;
                    c.g[V0][0] ^= 0x1;
                    let busy = c.g[V0][0] == 0;
                    c.g[A0][0] = gpw(c, 0x84c);
                    if !busy {
                        c.cycles += 5;
                        c.g[A1][0] = 0x1;
                        call_ptr(c, 0x0027_91d8, 0x0023_52a4)?;
                        c.g[V0][0] ^= 0x1;
                        c.g[V1][0] = 0x1;
                        if c.g[V0][0] != 0 {
                            c.g[S0][0] = 1;
                        }
                    }
                    c.cycles += 2;
                    let done = c.g[S0][0] != 0;
                    c.g[V0][0] = 0x2;
                    if done {
                        c.cycles += 2;
                        c.sw(this.wrapping_add(0x10), c.g[V0][0]);
                    }
                    End::Leave
                }
                2 => 'step2: {
                    c.cycles += 5;
                    c.g[V0][0] = gpw(c, 0x488);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x0012_abd0, 0x0023_52cc)?;
                    let no_slots = c.g[V0][0] == 0;
                    c.g[V0][0] = gpw(c, 0x488);
                    if no_slots {
                        break 'step2 End::Leave;
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x0012_acf8, 0x0023_52dc)?;
                    let no_slots = c.g[V0][0] == 0;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                    if no_slots {
                        break 'step2 End::Reloaded;
                    }
                    c.cycles += 19;
                    call_ptr(c, 0x0014_dc80, 0x0023_52ec)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x0014_dd58, 0x0023_52f8)?;
                    c.g[S1][0] = c.g[V0][0];
                    call_ptr(c, 0x0014_dc80, 0x0023_5300)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0014_dd58, 0x0023_530c)?;
                    c.g[V1][0] = 0x530000;
                    c.g[S4][0] = c.g[V0][0];
                    c.g[V0][0] = sx(0x0053_5bc8);
                    c.g[A0][0] = gpw(c, 0x848);
                    // The world type (0x535c12).
                    c.g[A1][0] = c.lb(0x0053_5c12);
                    c.g[S7][0] = c.g[V1][0];
                    c.g[V0][0] = ((c.g[A1][0] as i64) < 6) as u64;
                    let below = c.g[V0][0] != 0;
                    c.g[S0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc0));
                    let restart = below || {
                        c.cycles += 4;
                        c.g[V0][0] = (0xb < (c.g[A1][0] as i64)) as u64;
                        c.g[A2][0] = gpw(c, 0x488);
                        c.g[V0][0] != 0
                    };
                    if restart {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x0023_8160, 0x0023_5348)?;
                        c.g[A2][0] = gpw(c, 0x488);
                    }
                    // The AI rider count and characters.
                    c.cycles += 16;
                    let mode = c.g[S0][0] as u32;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.lw(mode.wrapping_add(0x10));
                    c.g[S2][0] = 0;
                    c.g[V0][0] = c.lw(mode.wrapping_add(0x14));
                    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                    c.g[S5][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[S3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                    c.g[A1][0] = c.g[S5][0];
                    call_ptr(c, 0x0014_4d98, 0x0023_5370)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    let commit = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, commit, 0x0023_5384)?;
                    let none = (c.g[S5][0] as i64) <= 0;
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x40));
                    if !none {
                        c.cycles += 3;
                        c.g[V0][0] = sx((c.g[S3][0] as u32) << 2);
                        c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S3][0] as u32));
                        loop {
                            c.cycles += 13;
                            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x28));
                            c.g[A0][0] = c.g[S4][0];
                            c.g[A1][0] = c.g[S0][0];
                            call_ptr(c, 0x0014_7338, 0x0023_53a8)?;
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                            c.g[A2][0] = c.lw(c.g[S1][0] as u32);
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A0][0] = c.g[S4][0];
                            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                            call_ptr(c, 0x0014_73d0, 0x0023_53c0)?;
                            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S5][0] as i64)) as u64;
                            let more = c.g[V0][0] != 0;
                            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S3][0] as u32));
                            if !more {
                                break;
                            }
                            if crate::events::event_due(c) {
                                crate::events::deliver_event_here(c, 0x0023_5398);
                            }
                        }
                    }
                    // Rebuild the riders.
                    c.cycles += 16;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xc));
                    c.g[S0][0] = sx(0x0053_5bc8);
                    c.g[S1][0] = 0;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let commit = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, commit, 0x0023_53e8)?;
                    c.g[V0][0] = gpw(c, 0x488);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x0012_b030, 0x0023_53f4)?;
                    c.g[V0][0] = gpw(c, 0x488);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x0012_b000, 0x0023_5400)?;
                    c.g[V0][0] = c.lb(0x0053_5c10);
                    let typed = c.g[V0][0] != 0;
                    c.g[V0][0] = gpw(c, 0x488);
                    if !typed {
                        c.cycles += 3;
                        c.g[V0][0] = c.lb(0x0053_5c11);
                        c.g[S1][0] = (c.g[V0][0] < 0x1u64) as u64;
                        c.g[V0][0] = gpw(c, 0x488);
                    }
                    c.cycles += 4;
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x0012_8958, 0x0023_5420)?;
                    let some = c.g[S5][0] != 0;
                    c.g[V0][0] = 0x3;
                    if some {
                        break 'step2 End::Store;
                    }
                    c.cycles += 2;
                    if c.g[S1][0] == 0 {
                        c.sw(this.wrapping_add(0x10), c.g[V0][0]);
                        break 'step2 End::Leave;
                    }
                    // No AI riders, world type and sub-type 0: the actor
                    // riders (mode 0, or 2 for a typed world).
                    c.cycles += 3;
                    c.g[V0][0] = c.lb(0x0053_5c10);
                    let typed = c.g[V0][0] != 0;
                    c.g[V0][0] = gpw(c, 0x488);
                    c.cycles += 5;
                    c.g[A1][0] = if typed { 0x2 } else { 0 };
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(
                        c,
                        0x0012_8998,
                        if typed { 0x0023_545c } else { 0x0023_5448 },
                    )?;
                    c.g[V0][0] = 0x3;
                    End::Store
                }
                3 => 'step3: {
                    c.cycles += 5;
                    c.g[V0][0] = gpw(c, 0x488);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x0012_a180, 0x0023_5470)?;
                    let loading = c.g[V0][0] == 0;
                    c.g[V0][0] = gpw(c, 0x488);
                    if loading {
                        c.cycles += 4;
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        call_ptr(c, 0x0012_96f8, 0x0023_5514)?;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                        break 'step3 End::Reloaded;
                    }
                    c.cycles += 10;
                    call_ptr(c, 0x0014_dc80, 0x0023_5480)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, 0x0014_dd58, 0x0023_548c)?;
                    c.g[V1][0] = 0x530000;
                    c.g[V0][0] = sx(0x0053_5bc8);
                    c.g[S0][0] = c.lb(0x0053_5c10);
                    let typed = c.g[S0][0] != 0;
                    c.g[S7][0] = c.g[V1][0];
                    if !typed {
                        c.cycles += 8;
                        c.g[V0][0] = gpw(c, 0x488);
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        call_ptr(c, 0x0012_89f0, 0x0023_54ac)?;
                        c.g[V0][0] = gpw(c, 0x488);
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        let ai = c.g[A0][0] as u32;
                        c.g[V1][0] = c.lw(ai.wrapping_add(0x80));
                        let none = (c.g[V1][0] as i64) <= 0;
                        c.g[V0][0] = 0x1;
                        if !none {
                            c.cycles += 1;
                            c.sw(ai.wrapping_add(0x14), c.g[V0][0]);
                        }
                        c.cycles += 5;
                        c.g[V0][0] = gpw(c, 0x488);
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        let ai = c.g[A0][0] as u32;
                        c.g[V1][0] = c.lw(ai.wrapping_add(0x84));
                        let none = (c.g[V1][0] as i64) <= 0;
                        c.g[V0][0] = 0x1;
                        if !none {
                            c.cycles += 1;
                            c.sw(ai.wrapping_add(0x8c), c.g[V0][0]);
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0x5;
                    let type5 = c.g[S0][0] == 0x5;
                    c.g[V0][0] = 0x4;
                    if !type5 {
                        break 'step3 End::Store;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = sx(0x0053_5bc8);
                    c.g[V1][0] = c.lb(0x0053_5c11);
                    if c.g[V1][0] != 0 {
                        c.g[V0][0] = 0x4;
                        break 'step3 End::Store;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = gpw(c, 0x488);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x0012_8a10, 0x0023_5504)?;
                    c.g[V0][0] = 0x4;
                    End::Store
                }
                4 => 'step4: {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw(this.wrapping_add(0x14));
                    let plain = c.g[V0][0] == 0;
                    c.g[V0][0] = 0x5;
                    if plain {
                        break 'step4 End::Store;
                    }
                    // The peak transition's minimum hold: 180 frames since
                    // the overlay opened (`slti v0, v0, 180`).
                    c.cycles += 4;
                    c.g[V0][0] = c.lw(this.wrapping_add(0x18));
                    let hold = crate::tick::ticks_from_frames60(c, 180) as i64;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < hold) as u64;
                    let wait = c.g[V0][0] != 0;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
                    if wait {
                        break 'step4 End::Reloaded;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 0x5;
                    End::Store
                }
                _ => End::Leave,
            }
        };
        match end {
            End::Store => {
                c.cycles += 2;
                c.sw(this.wrapping_add(0x10), c.g[V0][0]);
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            }
            End::Leave => {
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x80));
            }
            End::Reloaded => {}
        }
        c.cycles += 10;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
        c.pc = ra;
        Ok(())
    });
}

/// `void GameStateRestartRace_Update(GameStateRestartRace* self)` (0x235cc8):
/// restart the race. `+0x14` counts updates; `+0x10` is the step:
///
/// - 0: once the computer riders have render slots, rebuild them (and drop
///   the actor riders with two or more AI riders); step 1.
/// - 1: load the riders one step at a time; once all are loaded, reset them,
///   arm the AI start, and put the streamer's view on the start marker;
///   step 2.
/// - 2: with `+0x18` set, wait until the streamer's view is clear, at least
///   120 frames (2 s) have passed, list 0 is not running, no list or
///   transition is active, then skip the finish script; step 3.
/// - 3: go to the next game state.
///
/// Tick rate: `+0x14` goes up once per tick, so the 120-frame minimum is
/// compared in ticks ([`crate::tick::ticks_from_frames60`]; 120 at 60 Hz).
pub fn game_state_restart_race_update(c: &mut Cpu) {
    /// Where a step leaves: reload s0 (0x235f04), s1 (0x235f08), or only the
    /// rest (0x235f0c).
    enum End {
        S0,
        S1,
        Rest,
    }
    guest(c, |c| {
        c.cycles += 24;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[S1][0] = c.g[A0][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
        let this = c.g[S1][0] as u32;
        c.g[S3][0] = 0x1;
        c.g[V0][0] = c.lw(this.wrapping_add(0x14));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.sw(this.wrapping_add(0x14), c.g[V0][0]);
        call_ptr(c, 0x0014_dc80, 0x0023_5cf8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0023_5d04)?;
        c.g[V0][0] = gpw(c, 0x848);
        c.g[A2][0] = c.lw(this.wrapping_add(0x10));
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
        c.g[S2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xb0));
        // The streamer view index: 1 when `+0xb0` is clear.
        c.g[S0][0] = (c.g[A0][0] < 0x1u64) as u64;
        let step = c.g[A2][0];
        let end = 'step: {
            if step == 1 {
                // Load the riders; once loaded, reset them and set up the
                // start.
                c.cycles += 4;
                c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                call_ptr(c, 0x0012_a180, 0x0023_5db8)?;
                let loading = c.g[V0][0] == 0;
                c.g[V0][0] = gpw(c, 0x848);
                if loading {
                    c.cycles += 5;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x0012_96f8, 0x0023_5e38)?;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    break 'step End::S1;
                }
                c.cycles += 27;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                call_ptr(c, 0x0012_89f0, 0x0023_5dcc)?;
                c.g[V0][0] = gpw(c, 0x488);
                c.g[V1][0] = 0x50;
                c.g[A2][0] = gpw(c, 0x848);
                c.mult(0, V1, S0, V1);
                c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.g[SP][0];
                c.sw((c.g[A1][0] as u32).wrapping_add(0x14), c.g[S3][0]);
                c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x250), c.g[S3][0]);
                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x84));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x18));
                call_ptr(c, 0x0012_2c28, 0x0023_5e08)?;
                c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                c.g[A1][0] = c.g[S0][0];
                c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x5170));
                c.g[A2][0] = c.g[SP][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                call_ptr(c, 0x003a_9658, 0x0023_5e20)?;
                c.g[V0][0] = 0x2;
                c.sw(this.wrapping_add(0x10), c.g[V0][0]);
                break 'step End::S0;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A2][0] as i64) < 2) as u64;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                if step != 0 {
                    c.cycles += 2;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    break 'step End::Rest;
                }
                // Rebuild the computer riders once they have render slots.
                c.cycles += 4;
                c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                call_ptr(c, 0x0012_abd0, 0x0023_5d64)?;
                let no_slots = c.g[V0][0] == 0;
                c.g[V0][0] = gpw(c, 0x848);
                if no_slots {
                    break 'step End::S0;
                }
                c.cycles += 8;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                call_ptr(c, 0x0012_b030, 0x0023_5d78)?;
                c.g[V1][0] = 0x530000;
                // The AI rider count (0x535c04).
                c.g[V0][0] = c.lw(0x0053_5c04);
                c.g[V0][0] = ((c.g[V0][0] as i64) < 2) as u64;
                let few = c.g[V0][0] != 0;
                c.g[V0][0] = gpw(c, 0x848);
                if !few {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x0012_b000, 0x0023_5d98)?;
                    c.g[V0][0] = gpw(c, 0x848);
                }
                c.cycles += 5;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                call_ptr(c, 0x0012_8958, 0x0023_5da8)?;
                c.sw(this.wrapping_add(0x10), c.g[S3][0]);
                break 'step End::S0;
            }
            c.g[V0][0] = 0x2;
            c.cycles += 2;
            let step2 = step == 2;
            c.g[V0][0] = 0x3;
            if !step2 {
                c.cycles += 2;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                if step == 3 {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x0023_3aa0, 0x0023_5f04)?;
                    break 'step End::S0;
                }
                c.cycles += 2;
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                break 'step End::Rest;
            }
            // Wait for the restart to settle, then skip the finish script.
            c.cycles += 3;
            c.g[V0][0] = c.lw(this.wrapping_add(0x18));
            let skip = c.g[V0][0] != 0;
            c.g[A1][0] = c.g[S0][0];
            if skip {
                c.cycles += 11;
                c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                c.g[A2][0] = c.g[SP][0];
                c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                call_ptr(c, 0x003a_9770, 0x0023_5e60)?;
                c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x516c));
                c.c_lt_s(1, 0);
                let blocked = c.fcc();
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                if blocked {
                    break 'step End::S1;
                }
                // At least 120 frames (`slti v0, v0, 0x78`), in ticks.
                c.cycles += 4;
                c.g[V0][0] = c.lw(this.wrapping_add(0x14));
                let min = crate::tick::ticks_from_frames60(c, 120) as i64;
                c.g[V0][0] = ((c.g[V0][0] as i64) < min) as u64;
                if c.g[V0][0] != 0 {
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
                    break 'step End::Rest;
                }
                c.cycles += 5;
                c.g[A0][0] = gpw(c, 0x84c);
                c.g[A1][0] = 0;
                call_ptr(c, 0x0027_91d8, 0x0023_5e94)?;
                let running = c.g[V0][0] == c.g[S3][0];
                c.g[V0][0] = gpw(c, 0x848);
                if running {
                    break 'step End::S0;
                }
                c.cycles += 8;
                c.g[A1][0] = 0;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                call_ptr(c, 0x0023_0698, 0x0023_5ea8)?;
                c.g[A0][0] = gpw(c, 0x84c);
                c.g[S0][0] = c.g[V0][0];
                call_ptr(c, 0x0027_9298, 0x0023_5eb4)?;
                if c.g[V0][0] != 0 {
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    break 'step End::S1;
                }
                c.cycles += 8;
                let trans = c.g[S0][0] as u32;
                c.g[V1][0] = c.lw(trans.wrapping_add(0x1c));
                c.g[A1][0] = sx(trans.wrapping_add(0x10));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x40));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x44));
                let active = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, active, 0x0023_5ed4)?;
                if c.g[V0][0] != 0 {
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                    break 'step End::S1;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw(trans.wrapping_add(0x44));
                let busy = c.g[V0][0] != 0;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
                if busy {
                    break 'step End::S1;
                }
                c.cycles += 2;
                c.g[A0][0] = gpw(c, 0x84c);
                call_ptr(c, 0x0027_a9f0, 0x0023_5ef0)?;
            }
            c.cycles += 3;
            c.g[V0][0] = 0x3;
            c.sw(this.wrapping_add(0x10), c.g[V0][0]);
            End::S0
        };
        if let End::S0 = end {
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        }
        if !matches!(end, End::Rest) {
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        }
        c.cycles += 5;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = ra;
        Ok(())
    });
}

/// `void GameStateReset_Update(GameStateReset* s)` (0x2357f8): once the reset
/// flag is armed and the script engine is idle, flag the "reset" field on the
/// race manager and let the state machine advance.
pub fn game_state_reset_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        sqsp(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(p(r32(c, S0), 0x1c)).lw(c);
        let idle = c.g[V0][0] == 0;
        c.g[A0][0] = Addr(gp_3(c, -0x84c)).lw(c);
        if !idle {
            c.cycles += 4;
            c.g[A0][0] = Addr(gp_3(c, -0x84c)).lw(c);
            call::call_ptr(c, 0x0027_aa80, 0x0023_581c)?;
            let busy = c.g[V0][0] != 0;
            c.g[A0][0] = Addr(gp_3(c, -0x84c)).lw(c);
            if !busy {
                c.cycles += 7;
                c.g[A0][0] = Addr(gp_3(c, -0x848)).lw(c);
                c.g[A1][0] = 1;
                c.g[V0][0] = Addr(p(c.g[A0][0] as u32, 0x84)).lw(c);
                c.g[V1][0] = Addr(p(c.g[V0][0] as u32, 0x78)).lw(c);
                Addr(p(c.g[V1][0] as u32, 0x1a4)).sw(c, c.g[A1][0]);
                Addr(p(r32(c, S0), 0x1c)).sw(c, 0);
                c.g[A0][0] = Addr(gp_3(c, -0x84c)).lw(c);
            }
        }
        c.cycles += 4;
        c.g[A1][0] = 1;
        call::call_ptr(c, 0x0027_9220, 0x0023_5848)?;
        if c.g[V0][0] != 0 {
            lqsp(c, 16, 0x10);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call::call_ptr(c, 0x0023_3aa0, 0x0023_5858)?;
            lqsp(c, 16, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}

/// `InputSync_IsReady` (0x237950): true when the input sync's frame counter is
/// past its startup delay, it is not waiting on a load, and every self slot's
/// cursors agree.
pub fn input_sync_is_ready(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V1][0] = Addr(gp(c, -0x204)).lw(c);
    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x70)).lw(c);
    c.g[V0][0] = ((c.g[V0][0] as i64) < 14) as u64;
    let early = c.g[V0][0] != 0;
    if early {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x3f8)).lw(c);
    let loading = c.g[V0][0] != 0;
    c.g[A1][0] = 0;
    let mut busy = loading;
    if !loading {
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x3fc)).lw(c);
        let idle = (c.g[V0][0] as i64) <= 0;
        if !idle {
            busy = true;
        }
    }
    if busy {
        c.cycles += 1;
        c.g[A1][0] = 1;
    }
    c.cycles += 2;
    if c.g[A1][0] != 0 {
        // A busy/loading input sync is "not ready": the generated body breaks
        // out of the slot walk to the shared `return 0`.
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.g[V0][0] = lw_at_2(c, A0, 0x10);
    c.cycles += 2;
    let none = (c.g[V0][0] as i64) <= 0;
    c.g[A1][0] = 0;
    if !none {
        c.cycles += 2;
        c.g[A2][0] = c.g[V0][0];
        c.g[A0][0] = low(c, A0).wrapping_add(0x14);
        loop {
            c.cycles += 4;
            c.g[V1][0] = lw_at_2(c, A0, 0x4);
            c.g[V0][0] = lw_at_2(c, A0, 0x8);
            let full = c.g[V1][0] == c.g[V0][0];
            c.g[A1][0] = low(c, A1).wrapping_add(1);
            if full {
                c.cycles += 2;
                c.g[V0][0] = 0;
                abi::ret(c);
                return;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A2][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[A0][0] = low(c, A0).wrapping_add(0x18);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0023_79a0);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 1;
    abi::ret(c);
}

/// `void GameStateCountdown_Update(GameState* g)` (0x234c68): while the race
/// countdown (`world->race->+0x1c`, 60 Hz frames, `RaceStateCountdown_Update`)
/// runs, sound a start light at each multiple of 60; when it is out, open
/// the gate (script event, HUD, audio) and go to the next state.
///
/// Above 60 Hz the countdown holds each value for `rate / 60` ticks, so the
/// body runs only when the value changed (or is out): a light per value, not
/// per tick. With a positive countdown the body does nothing else (getters
/// and the light). Every call runs the original at 60.
pub fn game_state_countdown_update(c: &mut Cpu) {
    let world = c.bus.read32((c.g[GP][0] as u32).wrapping_sub(0x488));
    let race = if world == 0 {
        0
    } else {
        c.bus.read32(world.wrapping_add(0xc))
    };
    if race != 0 {
        let timer = race.wrapping_add(0x1c);
        let value = c.bus.read32(timer);
        if !crate::tick::value_changed(c, timer, value) && (value as i32) > 0 {
            abi::ret(c);
            return;
        }
    }
    game_state_countdown_update_orig(c);
}

/// Native body of `GameStateCountdown_Update` (0x234c68) as the original runs it, converted from the generated code (registers,
/// cycles and calls as the original).
#[allow(
    unreachable_code,
    unused_assignments,
    unused_labels,
    unused_mut,
    unused_variables
)]
fn game_state_countdown_update_orig(c: &mut Cpu) {
    guest(c, |c| {
        'b_234dcc: {
            c.cycles += 11;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S1][0] = c.g[A0][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[RA][0] = sx(0x00234c8c);
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
            call_ptr(c, 0x00270280, 0x00234c8c)?;
            let t = c.g[V0][0] != 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            if t {
                break 'b_234dcc;
            } else {
                'b_234dc8: {
                    c.cycles += 10;
                    c.g[RA][0] = sx(0x00234c9c);
                    call_ptr(c, 0x0014dc80, 0x00234c9c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00234ca8);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x00234ca8)?;
                    c.g[V1][0] = 0x530000;
                    c.g[A0][0] = 0x4;
                    c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c10));
                    let t = c.g[V0][0] == c.g[A0][0];
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                    if t {
                        break 'b_234dc8;
                    } else {
                        'b_234d60: {
                            c.cycles += 4;
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                            let t = (c.g[A0][0] as i64) > 0;
                            c.g[V1][0] = 0x3c;
                            if t {
                                break 'b_234d60;
                            } else {
                                'b_234d2c: {
                                    c.cycles += 18;
                                    c.g[A0][0] = 0x480000;
                                    c.g[RA][0] = sx(0x00234cd8);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3fb8));
                                    call_ptr(c, 0x00317670, 0x00234cd8)?;
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x00234ce4);
                                    c.g[A0][0] = c.g[S1][0];
                                    Callee {
                                        addr: 0x00234be8,
                                        f: ee::s06::GameState_PlayScriptEvent,
                                    }
                                    .call(c, 0x00234ce4)?;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                                    c.g[A0][0] = 0x4a0000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2ab0));
                                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[RA][0] = sx(0x00234cfc);
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                                    call_ptr(c, 0x00317670, 0x00234cfc)?;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[RA][0] = sx(0x00234d08);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039f9d8, 0x00234d08)?;
                                    c.g[A3][0] = c.g[V0][0];
                                    let t = c.g[A3][0] == 0u64;
                                    c.g[A1][0] = 0x6;
                                    if t {
                                        break 'b_234d2c;
                                    } else {
                                        c.cycles += 6;
                                        c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x8));
                                        c.g[A2][0] = 0u64;
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0xc0));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc4));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x00234d2c);
                                        c.g[A0][0] =
                                            sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x00234d2c)?;
                                        break 'b_234d2c;
                                    }
                                }
                                c.cycles += 13;
                                c.g[RA][0] = sx(0x00234d34);
                                call_ptr(c, 0x0028b180, 0x00234d34)?;
                                c.g[RA][0] = sx(0x00234d3c);
                                c.g[A0][0] = c.g[V0][0];
                                call_ptr(c, 0x0029c7b0, 0x00234d3c)?;
                                c.g[RA][0] = sx(0x00234d44);
                                call_ptr(c, 0x0028b180, 0x00234d44)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x00234d50);
                                c.g[A1][0] = 0x78;
                                call_ptr(c, 0x0028c8c0, 0x00234d50)?;
                                c.g[RA][0] = sx(0x00234d58);
                                c.g[A0][0] = c.g[S1][0];
                                Callee {
                                    addr: 0x00233aa0,
                                    f: ee::s06::GameState_GotoNext,
                                }
                                .call(c, 0x00234d58)?;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                                c.g[V1][0] = 0x3c;
                                break 'b_234d60;
                            }
                        }
                        'b_234d68: {
                            c.cycles += 2;
                            if c.g[V1][0] == 0u64 {
                                break 'b_234d68;
                            } else {
                                break 'b_234d68;
                            }
                        }
                        c.cycles += 10;
                        c.g[A0][0] = 0x3c;
                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x1c));
                        c.div(0, 2, 3);
                        c.g[V0][0] = c.lo[0];
                        c.g[A0][0] = c.hi[0];
                        c.mtc1(2, 20);
                        c.cvt_s_w(20, 20);
                        let t = c.g[A0][0] != 0u64;
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                        if t {
                            break 'b_234dcc;
                        } else {
                            'b_234db8: {
                                c.cycles += 7;
                                c.g[RA][0] = sx(0x00234d98);
                                call_ptr(c, 0x0028b180, 0x00234d98)?;
                                c.cvt_w_s(1, 20);
                                c.cvt_s_w(1, 1);
                                c.c_lt_s(20, 1);
                                let t = !c.fcc();

                                if t {
                                    break 'b_234db8;
                                } else {
                                    c.cycles += 3;
                                    c.g[AT][0] = 0x3f800000;
                                    c.mtc1(1, 0);
                                    c.sub_s(1, 1, 0);
                                    break 'b_234db8;
                                }
                            }
                            c.cycles += 4;
                            c.cvt_w_s(0, 1);
                            c.mfc1(5, 0);
                            c.g[RA][0] = sx(0x00234dc8);
                            c.g[A0][0] = c.g[V0][0];
                            call_ptr(c, 0x0029c420, 0x00234dc8)?;
                            break 'b_234dc8;
                        }
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                break 'b_234dcc;
            }
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
        Ok(())
    });
}

/// `void GameState_PlayScriptEvent(GameState* g, UStr* name)`
/// (0x00234be8): resolve the current track's world section and call the
/// script function it names, when the table has it.
pub fn game_state_play_script_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x30));
        c.sq(S1, sp(c).wrapping_add(0x10));
        c.sq(S0, sp(c).wrapping_add(0x20));
        c.sd(sp(c), c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, 0x0014_dc80, 0x0023_4c00)?; // BELibrary_GetInstance
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dd58, 0x0023_4c0c)?; // BELibrary_GetInterface
        c.g[V1][0] = c.lw(gpn(c, 0x848));
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
        call_ptr(c, 0x0014_4bc0, 0x0023_4c20)?; // BENewRaceInt_GetTrackInfo
        c.g[A1][0] = c.lw(c.g[V0][0] as u32);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0022_e098, 0x0023_4c2c)?; // SectionMan_GetEventWorldSection
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.lw(gpa_2(c, 0xce8));
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        call_ptr(c, 0x0030_9dd0, 0x0023_4c40)?; // WScriptMan_TableHasFunction
        let missing = c.g[V0][0] == 0;
        c.g[A0][0] = c.lw(gpa_2(c, 0xce8));
        if !missing {
            c.cycles += 3;
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S1][0];
            call_ptr(c, 0x0030_9e50, 0x0023_4c54)?; // WScriptMan_CallTableFunction
        }
        c.cycles += 5;
        c.lq(S0, sp(c).wrapping_add(0x20));
        c.lq(S1, sp(c).wrapping_add(0x10));
        c.g[RA][0] = c.ld(sp(c));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Register and memory helpers.

/// `a + off` (32-bit pointer arithmetic).
#[inline(always)]
pub(crate) fn p_2(a: u32, off: u32) -> u32 {
    a.wrapping_add(off)
}

/// A `gp`-relative global.
#[inline(always)]
pub(crate) fn gw(c: &Cpu, off: i32) -> u32 {
    abi::r32(c, GP).wrapping_add(off as u32)
}

/// `lw` (sign-extended) from a 32-bit address.
#[inline(always)]
pub(crate) fn lw_at_3(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lw(c)
}

/// The two-instruction virtual-method read the originals use:
/// `a0 = lh(vt+off)`, `func = lw(vt+off+4)`. The original leaves `func` in
/// `vreg` (v0 or v1 depending on the site), which callers can see, so the
/// register is written here too. Returns `func` for the `call_ptr`.
#[inline(always)]
pub(crate) fn vslot(c: &mut Cpu, vt: u64, off: u32, vreg: usize) -> u32 {
    let base = (vt as u32).wrapping_add(off);
    c.g[A0][0] = Addr(base).lh(c);
    let func = Addr(base.wrapping_add(4)).lw(c);
    c.g[vreg][0] = func;
    func as u32
}

/// One of the three "copy my `+0x10/+0x14/+0x18` vector into the state" calls
/// of the game-state loaders: `vt[0x10](self + 0, self + dst, 4)` with the
/// vtable at `src->+0x10`.
pub(crate) fn load_state_vector(c: &mut Cpu, vt: u64, data: u32, dst: u32, ret: u32) -> Flow {
    let func = vslot(c, vt, 0x10, V0);
    c.g[A1][0] = sx(dst);
    c.g[A2][0] = 4;
    c.g[A0][0] = sx(data.wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, func, ret)
}

/// `void GameStateReset_LoadState(self, data)` (0x235a18): load the state,
/// then copy the three transform vectors from the data object.
pub fn gamestatereset_load_state(c: &mut Cpu) {
    game_state_load_state(c, 0x0023_5a34, [0x0023_5a50, 0x0023_5a6c, 0x0023_5a88]);
}

/// `void GameStateRestartRace_LoadState(self, data)` (0x235fd0): the restart
/// twin of [`gamestatereset_load_state`].
pub fn gamestaterestartrace_load_state(c: &mut Cpu) {
    game_state_load_state(c, 0x0023_5fec, [0x0023_6008, 0x0023_6024, 0x0023_6040]);
}

pub(crate) fn game_state_load_state(c: &mut Cpu, load_ret: u32, vec_rets: [u32; 3]) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        callee::GAME_STATE_LOAD_STATE.call(c, load_ret)?;
        for (i, ret) in vec_rets.into_iter().enumerate() {
            c.g[V1][0] = lw_at_3(c, c.g[S0][0] as u32);
            load_state_vector(
                c,
                c.g[V1][0],
                r32(c, S0),
                p_2(r32(c, S1), 0x10 + 4 * i as u32),
                ret,
            )?;
        }
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GameStatePreRaceMenu_ResumeRace()` (0x236cd8): stop the script
/// list, restart the race and reset the race sounds.
pub fn gamestatepreracemenu_resume_race(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = lw_at_3(c, gw(c, -0x84c));
        c.sd(p_2(sp(c), 0), c.g[RA][0]);
        c.g[A1][0] = 0;
        call_ptr(c, callee::SCRIPT_ENGINE_LIST_STOP_3.addr, 0x0023_6cec)?;
        callee::BE_LIBRARY_GET_INSTANCE_3.call(c, 0x0023_6cf4)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        callee::BE_LIBRARY_GET_INTERFACE_3.call(c, 0x0023_6d00)?;
        c.g[V1][0] = 0x530000;
        c.g[V0][0] = Addr(p_2(c.g[V1][0] as u32, 0x5c10)).lb(c);
        c.g[V0][0] ^= 4;
        let no_restart = c.g[V0][0] == 0;
        c.g[V1][0] = lw_at_3(c, gw(c, -0x488));
        if !no_restart {
            c.cycles += 11;
            c.g[A1][0] = 3;
            c.g[A2][0] = lw_at_3(c, p_2(c.g[V1][0] as u32, 0xc));
            c.g[V0][0] = lw_at_3(c, p_2(c.g[A2][0] as u32, 0xcc));
            let func = vslot(c, c.g[V0][0], 0x28, V1);
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, func, 0x0023_6d30)?;
            c.g[A0][0] = lw_at_3(c, gw(c, -0x488));
            c.g[V1][0] = 1;
            c.g[V0][0] = lw_at_3(c, p_2(c.g[A0][0] as u32, 0xc));
            c.sw(p_2(c.g[V0][0] as u32, 0x14), c.g[V1][0]);
        }
        c.cycles += 7;
        callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_2.call(c, 0x0023_6d48)?;
        c.g[A0][0] = c.g[V0][0];
        callee::SSX_AUDIO_SYSTEM_RESET_RACE_SOUNDS.call(c, 0x0023_6d50)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Input sync.

/// `void InputSync_SendSample(InputSync* self, void* src)` (0x237a98): pack the
/// sample into the frame buffer, unpack it back and, when the game comms
/// queue is open, send it.
pub fn input_sync_send_sample(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        c.g[A2][0] = c.g[SP][0];
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0x20);
        c.g[S1][0] = c.g[A1][0];
        callee::INPUT_SYNC_PACK_SAMPLE.call(c, 0x0023_7abc)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[SP][0];
        callee::INPUT_SYNC_UNPACK_SAMPLE.call(c, 0x0023_7ad0)?;
        c.g[V1][0] = Addr(gp_4(c, -0x204)).lw(c);
        c.g[V0][0] = Addr(p_3(r32(c, V1), 0x40)).lw(c);
        let idle = c.g[V0][0] == 0;
        c.g[A0][0] = 0;
        if !idle {
            c.cycles += 3;
            c.g[V0][0] = Addr(p_3(r32(c, V1), 4)).lw(c);
            if c.g[V0][0] != 0 {
                c.g[A0][0] = 0;
            } else {
                c.cycles += 2;
                c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                c.g[A0][0] = (0u64 < c.g[V0][0]) as u64;
            }
        }
        let send = !(c.g[A0][0] == 0);
        c.cycles += 2;
        c.g[A0][0] = Addr(gp_4(c, -0x204)).lw(c);
        if send {
            c.cycles += 4;
            c.g[A3][0] = c.g[S2][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = c.g[SP][0];
            call_ptr(c, 0x0025_7e98, 0x0023_7b0c)?;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void GameStateRace_Enter(self)` (0x234e20): put the AI into state 5, reset
/// the race manager unless the game was already in mode 4, then open the
/// finish screen if the rider count looks wrong.
pub fn game_state_race_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(gp_5(c, -0x488)).lw(c);
        frame.save_ra(c, 0);
        c.g[A1][0] = 5;
        c.g[A0][0] = lw_2(c, V0, 0xc);
        call_ptr(c, table_callee::AI_SET_AI_STATE, 0x0023_4e38)?;
        call_ptr(c, table_callee::BE_GET, 0x0023_4e40)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, table_callee::BE_GET_INTERFACE_FV, 0x0023_4e4c)?;
        c.g[V1][0] = 0x530000;
        c.g[A0][0] = 4;
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x5c10)).lb(c);
        let mode_four = c.g[V0][0] == c.g[A0][0];
        c.g[A1][0] = Addr(gp_5(c, -0x488)).lw(c);
        if !mode_four {
            c.cycles += 5;
            c.g[A0][0] = 1;
            c.g[V0][0] = lw_2(c, A1, 0xc);
            Addr(c.g[V0][0] as u32).off(0x14).sw(c, c.g[A0][0]);
            c.g[V1][0] = lw_2(c, A1, 0xc);
            Addr(c.g[V1][0] as u32).off(0x8c).sw(c, 0);
        }
        c.cycles += 7;
        c.g[A0][0] = 1;
        call_ptr(c, table_callee::BE_SET_STATE, 0x0023_4e7c)?;
        c.g[V1][0] = Addr(gp_5(c, -0x488)).lw(c);
        c.g[V0][0] = lw_2(c, V1, 0x28);
        c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
        let empty = c.g[V0][0] == 0;
        c.g[A0][0] = 0;
        if !empty {
            c.cycles += 1;
            c.g[A0][0] = ((c.g[V0][0] as i64) < 10) as u64;
        }
        c.cycles += 2;
        let ok = c.g[A0][0] != 0;
        frame.restore_ra(c, 0);
        if !ok {
            c.cycles += 4;
            c.g[A0][0] = lw_2(c, V1, 0x48);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, table_callee::UI_OPEN_FINISH_SCREEN, 0x0023_4ea8)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `GameStatePreRaceMenu_Enter` (0x236bb0): leave the pause overlay, bump the
/// pre-race menu counter and enter the menu state (or the quick path when a
/// BE library / world is already active).
pub fn game_state_prerace_menu_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(gp_6(c, -0x204)).lw(c);
        frame.save_ra(c, 0);
        let be = c.g[V0][0] == 0;
        Addr(p_4(r(c, A0), 8)).sw(c, 0);
        if !be {
            c.cycles += 3;
            c.g[V0][0] = Addr(p_4(r(c, V0), 4)).lw(c);
            let no_lib = c.g[V0][0] == 0;
            c.g[V0][0] = Addr(gp_6(c, -0x848)).lw(c);
            if !no_lib {
                c.cycles += 7;
                c.g[V1][0] = Addr(p_4(r(c, V0), 0x84)).lw(c);
                c.g[RA][0] = sx(0x0023_6bdc);
                c.g[A0][0] = Addr(p_4(r(c, V1), 0x64)).lw(c);
                call_ptr(c, callee::OVERLAY_RESET, 0x0023_6bdc)?;
                c.g[RA][0] = sx(0x0023_6be4);
                c.g[A0][0] = 0;
                call_ptr(c, callee::BE_SET_PHASE, 0x0023_6be4)?;
                frame.restore_ra(c, 0);
                c.cycles += 2;
                let ra = r(c, RA);
                c.g[SP][0] = sx(r(c, SP).wrapping_add(0x10));
                c.pc = ra;
                return Ok(());
            }
        }
        c.cycles += 21;
        c.g[RA][0] = sx(0x0023_6bf4);
        c.g[A0][0] = 2;
        call_ptr(c, callee::GAME_MODULE_SET_STATE, 0x0023_6bf4)?;
        c.g[A2][0] = Addr(gp_6(c, -0x69c)).lw(c);
        c.g[V1][0] = 0x0053_0000;
        c.g[V1][0] = sx(0x0053_0000u32.wrapping_add(0x66e8));
        c.g[A0][0] = 0x0044_0000;
        c.g[A2][0] = sx(r(c, A2).wrapping_add(1));
        c.g[A1][0] = Addr(p_4(r(c, A0), 0x28f4)).lw(c);
        c.g[V0][0] = shl32(c.g[A2][0], 2);
        c.g[T0][0] = Addr(gp_6(c, -0x848)).lw(c);
        c.g[V0][0] = sx(r(c, V0).wrapping_add(r(c, V1)));
        c.g[A3][0] = 1;
        Addr(r(c, V0)).sw(c, c.g[A3][0]);
        Addr(gp_6(c, -0x6a0)).sw(c, c.g[A1][0]);
        c.g[V0][0] = Addr(p_4(r(c, T0), 0x84)).lw(c);
        Addr(gp_6(c, -0x69c)).sw(c, c.g[A2][0]);
        c.g[RA][0] = sx(0x0023_6c34);
        c.g[A0][0] = Addr(p_4(r(c, V0), 0x64)).lw(c);
        call_ptr(c, callee::OVERLAY_RESET, 0x0023_6c34)?;
        c.g[RA][0] = sx(0x0023_6c3c);
        c.g[A0][0] = 0;
        call_ptr(c, callee::BE_SET_PHASE, 0x0023_6c3c)?;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void GameStateEventSelect_LoadState(self, stream)` (0x2367c8): base load
/// plus four 4-byte stream fields.
pub fn game_state_event_select_load_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S1][0] = sx(this);
        callee::GAME_STATE_LOAD_STATE_2.call(c, 0x0023_67e4)?;
        for (off, ret) in [
            (0x10u32, 0x0023_6800u32),
            (0x14, 0x0023_681c),
            (0x8, 0x0023_6838),
            (0x18, 0x0023_6854),
        ] {
            c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
            c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(off));
            c.g[A2][0] = 4;
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x10).lh(c);
            c.g[V0][0] = lw_3(c, V1, 0x14);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, ret)?;
        }
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void InputSync_dtor(this, flags)` (0x2370a0): free each input record's
/// buffer and two objects.
pub fn input_sync_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x10);
        let this = abi::arg(c, 0);
        c.g[S2][0] = sx(this);
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A1][0];
        let none = (lw_3(c, S2, 0x10) as i64) <= 0;
        if !none {
            c.cycles += 3;
            c.g[S0][0] = sx(this.wrapping_add(0x14));
            c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
            'records: loop {
                c.cycles += 2;
                if c.g[A0][0] == 0 {
                    c.g[A0][0] = lw_3(c, S0, 0x10);
                } else {
                    c.cycles += 3;
                    call_ptr(c, table::EA_MEM_FREE, 0x0023_70e8)?;
                    c.g[A0][0] = lw_3(c, S0, 0x10);
                }
                c.cycles += 9;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                call_ptr(c, table::BUILTIN_DELETE, 0x0023_70f4)?;
                c.g[A0][0] = lw_3(c, S0, 0x14);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                call_ptr(c, table::BUILTIN_DELETE, 0x0023_7100)?;
                c.g[V0][0] = lw_3(c, S2, 0x10);
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x0023_70d8);
                    }
                    continue 'records;
                }
                break 'records;
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S3][0] & 1;
        let no_delete = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x40);
        if !no_delete {
            c.cycles += 3;
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, table::BUILTIN_DELETE, 0x0023_7124)?;
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Input.

/// `int InputSync_PackSample(SampleSrc* src, unsigned char* dst, u8* mask)`
/// (0x237b28): copy the first sample byte to `dst[0]`, zero `dst[1..4]`, then
/// pack the `src->count` floats at `src+4` as `(int)(v * 255)` — skipping
/// zeros — into `dst+4`, recording presence bits in the `dst+1` mask.
/// Returns the packed byte count + 4.
pub fn input_sync_pack_sample(c: &mut Cpu) {
    c.cycles += 6;
    let src = c.g[A1][0];
    c.g[T3][0] = src;
    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
    c.g[V0][0] = Addr(src as u32).lbu(c);
    c.g[A3][0] = 2;
    c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(3));
    Addr(c.g[A2][0] as u32).sb(c, c.g[V0][0]);
    loop {
        c.cycles += 7;
        Addr(c.g[V1][0] as u32).sb(c, 0);
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(1));
        let more = (c.g[A3][0] as i64) >= 0;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0023_7b40);
        }
    }
    c.cycles += 4;
    c.g[V0][0] = Addr(c.g[T3][0] as u32).lw(c);
    c.g[T2][0] = 0;
    let no_samples = (c.g[V0][0] as i64) <= 0;
    c.g[T0][0] = 0;
    if !no_samples {
        c.cycles += 7;
        c.g[AT][0] = 0x437f_0000;
        c.mtc1(1, 1);
        c.g[T4][0] = c.g[A0][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(4));
        c.g[T5][0] = 1;
        c.g[T1][0] = sx((c.g[T3][0] as u32).wrapping_add(4));
        loop {
            c.cycles += 7;
            c.lwc1(0, c.g[T1][0] as u32);
            c.mul_s(0, 0, 1);
            c.cvt_w_s(2, 0);
            c.mfc1(2, 2);
            c.g[A3][0] = c.g[V0][0] & 0xff;
            let zero = c.g[A3][0] == 0;
            c.g[A0][0] = sx(((c.g[T2][0] as u32) as i32 >> 3) as u32);
            if !zero {
                c.cycles += 9;
                c.g[V1][0] = c.g[T2][0] & 7;
                c.g[A0][0] = sx((c.g[T4][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V1][0] = sx((c.g[T5][0] as u32) << ((c.g[V1][0] as u32) & 31));
                c.g[V0][0] = Addr(c.g[A0][0] as u32).lbu(c);
                c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[T0][0] as u32));
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
                c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                Addr(c.g[A0][0] as u32).sb(c, c.g[V0][0]);
                Addr(c.g[A1][0] as u32).sb(c, c.g[A3][0]);
            }
            c.cycles += 5;
            c.g[V0][0] = Addr(c.g[T3][0] as u32).lw(c);
            c.g[T2][0] = sx((c.g[T2][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[T2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0023_7b88);
            }
        }
    }
    c.cycles += 2;
    let ra = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(4));
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// `GameStateEventSwitchLoad_Enter` (0x002368a0).
//
// `void GameStateEventSwitchLoad::Enter(self)`: load the game-state data and
// start playing its event tracks.

pub fn game_state_event_switch_load_enter(c: &mut Cpu) {
    guest(c, |c| {
        'b_236944: {
            c.cycles += 22;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S3][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[RA][0] = sx(0x002368c4);
            c.g[A0][0] = 0u64;
            call_ptr(c, 0x0014de68, 0x002368c4)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            c.g[RA][0] = sx(0x002368d0);
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
            call_ptr(c, 0x0026f228, 0x002368d0)?;
            c.g[RA][0] = sx(0x002368d8);
            call_ptr(c, 0x0014dc80, 0x002368d8)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x002368e4);
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014dd58, 0x002368e4)?;
            c.g[V1][0] = 0x530000;
            c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
            c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x49));
            if c.g[V0][0] != 0u64 {
                c.sw((c.g[S3][0] as u32).wrapping_add(0x10), 0u64);
                break 'b_236944;
            } else {
                c.cycles += 9;
                c.g[RA][0] = sx(0x00236900);
                c.g[S0][0] = 0u64;
                call_ptr(c, 0x0014dc80, 0x00236900)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x0023690c);
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0014dd58, 0x0023690c)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
                if (c.g[V0][0] as i64) <= 0 {
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x10), 0u64);
                    break 'b_236944;
                } else {
                    c.cycles += 1;
                    c.g[A1][0] = c.g[S0][0];
                    'l_236920: loop {
                        c.cycles += 8;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A2][0] = 0u64;
                        c.g[RA][0] = sx(0x00236930);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        call_ptr(c, 0x00145cb0, 0x00236930)?;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
                        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A1][0] = c.g[S0][0];
                        if t {
                            if crate::events::event_due(c) {
                                crate::events::deliver_event_here(c, 0x00236920);
                            }
                            continue 'l_236920;
                        } else {
                            c.cycles += 1;
                            c.sw((c.g[S3][0] as u32).wrapping_add(0x10), 0u64);
                            break 'b_236944;
                        }
                    }
                }
            }
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `InputSync_UnpackSample(this, src, dst)` (0x237be8): copy the packed
/// sample's bit count and expand up to 24 channels of 8-bit values into
/// floats, zeroing the channels whose mask bit is clear.
pub fn input_sync_unpack_sample(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let this = abi::arg(c, 0);
        let src = abi::arg(c, 1);
        let dst = abi::arg(c, 2);
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = abi::arg64(c, 2);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = abi::arg64(c, 1);
        frame.save_ra(c, 0);
        c.g[S2][0] = sx(src.wrapping_add(4));
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S2][0];
        c.g[V0][0] = Addr(dst).lbu(c);
        c.g[A2][0] = 0x60;
        Addr(src).sw(c, c.g[V0][0]);
        callee::EA_MEM_SET.call(c, 0x0023_7c20)?;

        c.g[V0][0] = Addr(src).lw(c);
        c.g[T0][0] = 0;
        let none = (c.g[V0][0] as i64) <= 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 4;
            c.lwc1(1, gp(c, -0x5160));
            c.g[T1][0] = sx(src.wrapping_add(4));
            c.g[A3][0] = c.g[S2][0];
            c.g[A2][0] = sx(src.wrapping_add(1));
            loop {
                c.cycles += 8;
                c.g[V0][0] = sx((((c.g[A1][0] as u32) as i32) >> 3) as u32);
                c.g[A0][0] = c.g[A1][0] & 0x7;
                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lbu(c);
                let sh = (c.g[A0][0] as u32) & 31;
                c.g[V1][0] = sx((((c.g[V1][0] as u32) as i32) >> sh) as u32);
                c.g[V1][0] = c.g[V1][0] & 1;
                let clear = c.g[V1][0] == 0;
                c.g[V0][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[T0][0] as u32));
                if clear {
                    c.cycles += 1;
                    Addr(c.g[A3][0] as u32).sw(c, 0);
                } else {
                    c.cycles += 7;
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).lbu(c);
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(1));
                    c.mtc1(3, 0);
                    c.cvt_s_w(0, 0);
                    c.mul_s(0, 0, 1);
                    c.swc1(0, abi::r32(c, A3));
                }
                c.cycles += 5;
                c.g[V0][0] = Addr(src).lw(c);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(4));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0023_7c40);
                }
            }
        }

        c.cycles += 6;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        let _ = this;
        Ok(())
    })
}

/// `int InputSync_ApplyInput(InputSync *self)` (0x2379c8): when the sync is
/// ready, feed every pending input slot to its handler.
pub fn inputsync_apply_input(c: &mut Cpu) {
    guest(c, |c| {
        'b_237a78: {
            'b_237a0c: {
                'b_237a04: {
                    c.cycles += 11;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x70));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 14i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S1][0] = c.g[A0][0];
                    if t {
                        break 'b_237a04;
                    } else {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x002379fc);
                        c.g[A0][0] = c.g[S1][0];
                        call_ptr(c, 0x00237950, 0x002379fc)?;
                        if c.g[V0][0] != 0u64 {
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                            break 'b_237a0c;
                        } else {
                            break 'b_237a04;
                        }
                    }
                }
                c.cycles += 2;
                let t = true;
                c.g[V0][0] = 0u64;
                if t {
                    break 'b_237a78;
                } else {
                    break 'b_237a0c;
                }
            }
            'b_237a74: {
                c.cycles += 2;
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[S2][0] = 0u64;
                if t {
                    break 'b_237a74;
                } else {
                    c.cycles += 3;
                    c.g[S3][0] = 0x64;
                    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x14));
                    'l_237a20: loop {
                        'b_237a38: {
                            c.cycles += 6;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                            c.div(0, 4, 2);
                            if c.g[V0][0] == 0u64 {
                                break 'b_237a38;
                            } else {
                                break 'b_237a38;
                            }
                        }
                        c.cycles += 15;
                        c.g[A2][0] = c.lw(c.g[S0][0] as u32);
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                        c.g[V1][0] = c.hi[0];
                        c.mult(0, 3, 3, 19);
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V1][0] as u32));
                        c.g[A1][0] = c.lw(c.g[A2][0] as u32);
                        c.g[RA][0] = sx(0x00237a58);
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4));
                        call_ptr(c, 0x00321298, 0x00237a58)?;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
                        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00237a20);
                            }
                            continue 'l_237a20;
                        } else {
                            break 'b_237a74;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0x1;
            break 'b_237a78;
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `GameState_SaveState` (same shard; through the override slot).
pub(crate) const EVENT_SELECT_BASE_SAVE: Callee = Callee {
    addr: 0x0023_3af0,
    f: ee::s06::GameState_SaveState,
};

/// `void GameStateEventSelect_SaveState(GameStateEventSelect* self,
/// Stream* stream)` (0x236728): save the base game state, then the four
/// selection words at `+0x10`, `+0x14`, `+0x8`, `+0x18` (4 bytes each,
/// through the stream's write method, vtable slot 1).
pub fn game_state_event_select_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0]; // stream
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0]; // self
        EVENT_SELECT_BASE_SAVE.call(c, 0x0023_6744)?;
        for (field, ret_addr) in [
            (0x10u32, 0x0023_6760u32),
            (0x14, 0x0023_677c),
            (0x8, 0x0023_6798),
            (0x18, 0x0023_67b4),
        ] {
            let stream = r32(c, S0);
            c.g[V1][0] = Addr(stream).lw(c);
            let vt = r32(c, V1);
            c.g[A1][0] = sx(r32(c, S1).wrapping_add(field));
            c.g[A2][0] = 4;
            c.g[A0][0] = Addr(vt + 0x8).lh(c);
            c.g[V0][0] = Addr(vt + 0xc).lw(c);
            let write = r32(c, V0);
            c.g[A0][0] = sx(stream.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, write, ret_addr)?;
        }
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// GameStateRestartRace_SaveState (0x235f48)

/// `GameState_SaveState` (same shard; through the override slot).
pub(crate) const RESTART_RACE_BASE_SAVE: Callee = Callee {
    addr: 0x0023_3af0,
    f: ee::s06::GameState_SaveState,
};

/// `void GameStateRestartRace_SaveState(GameStateRestartRace* self,
/// Stream* stream)` (0x235f48): save the base game state, then the three
/// restart words at `+0x10`, `+0x14`, `+0x18` (4 bytes each, through the
/// stream's write method, vtable slot 1). Frame 0x30 (s0 = stream,
/// s1 = self); one 33-cycle block.
pub fn game_state_restart_race_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0]; // stream
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0]; // self
        RESTART_RACE_BASE_SAVE.call(c, 0x0023_5f64)?;
        for (field, ret_addr) in [
            (0x10u32, 0x0023_5f80u32),
            (0x14, 0x0023_5f9c),
            (0x18, 0x0023_5fb8),
        ] {
            let stream = r32(c, S0);
            c.g[V1][0] = Addr(stream).lw(c);
            let vt = r32(c, V1);
            c.g[A1][0] = sx(r32(c, S1).wrapping_add(field));
            c.g[A2][0] = 4;
            c.g[A0][0] = Addr(vt.wrapping_add(0x8)).lh(c);
            c.g[V0][0] = Addr(vt.wrapping_add(0xc)).lw(c);
            let write = r32(c, V0);
            c.g[A0][0] = sx(stream.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, write, ret_addr)?;
        }
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// GameStateReset_SaveState (0x235990)

/// `GameState_SaveState` (0x233af0).
pub(crate) const OC300_GAME_STATE_SAVE_STATE: Callee = Callee {
    addr: 0x0023_3af0,
    f: ee::s06::GameState_SaveState,
};

/// The stream's `Write(data, size)` vtable slot.
pub(crate) const OC300_STREAM_WRITE: u32 = 0x8;

/// `GameStateReset`: the three state words written after the base state.
pub(crate) const OC300_RESET_SAVED_WORDS: u32 = 0x10;

/// `void GameStateReset_SaveState(GameStateReset* self, cAsStream* s)`
/// (0x235990): saves the base game state (`GameState_SaveState`), then
/// writes the three words at `self+0x10/+0x14/+0x18` (`s->Write(p, 4)`,
/// vtable slot `+0x8`).
pub fn game_state_reset_save_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        OC300_GAME_STATE_SAVE_STATE.call(c, 0x0023_59ac)?;
        for (i, ret) in [0x0023_59c8u32, 0x0023_59e4, 0x0023_5a00]
            .into_iter()
            .enumerate()
        {
            let stream = r32(c, S0);
            let vt = Addr(stream).lw(c);
            c.g[V1][0] = vt;
            c.g[A1][0] = sx(r32(c, S1).wrapping_add(OC300_RESET_SAVED_WORDS + 4 * i as u32));
            c.g[A2][0] = 0x4;
            let slot = crate::call::VSlot::read(c, vt, OC300_STREAM_WRITE);
            c.g[A0][0] = slot.delta;
            c.g[V0][0] = slot.func;
            c.g[A0][0] = sx(stream.wrapping_add(slot.delta as u32));
            call_ptr(c, slot.func as u32, ret)?;
        }
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// GameState_*_SaveState wrappers (0x236fa8, 0x236ec8, 0x236d60, 0x236b70)

/// `GameState_SaveState` (0x233af0, through its override slot).
pub(crate) const OC325_GAME_STATE_SAVE_STATE: Callee = Callee {
    addr: 0x0023_3af0,
    f: ee::s06::GameState_SaveState,
};

/// A per-state `SaveState` wrapper: save the base game state, then return.
pub(crate) fn oc325_gamestate_save_wrapper(c: &mut Cpu, ret_addr: u32) {
    tail_wrapper(
        c,
        None,
        Target::Direct(OC325_GAME_STATE_SAVE_STATE),
        ret_addr,
    );
}

/// `void GameStateFinishCinematic_SaveState(GameState* self,
/// Serialiser* s)` (0x236fa8): the base game state's save.
pub fn game_state_finish_cinematic_save_state(c: &mut Cpu) {
    oc325_gamestate_save_wrapper(c, 0x0023_6fb8);
}

/// `void GameStateResults_SaveState(GameState* self, Serialiser* s)`
/// (0x236ec8): the base game state's save.
pub fn game_state_results_save_state(c: &mut Cpu) {
    oc325_gamestate_save_wrapper(c, 0x0023_6ed8);
}

/// `void GameStatePreRaceMenu_SaveState(GameState* self, Serialiser* s)`
/// (0x236d60): the base game state's save.
pub fn game_state_pre_race_menu_save_state(c: &mut Cpu) {
    oc325_gamestate_save_wrapper(c, 0x0023_6d70);
}

/// `void GameStateResetAI_SaveState(GameState* self, Serialiser* s)`
/// (0x236b70): the base game state's save.
pub fn game_state_reset_ai_save_state(c: &mut Cpu) {
    oc325_gamestate_save_wrapper(c, 0x0023_6b80);
}

// ---------------------------------------------------------------------------
// GameState*_SaveState (0x236af8, 0x2361c8, 0x234eb8, 0x234de0, 0x233bd0,
// 0x233b90)
//
// Six identical gcc forwarding wrappers: a 0x10 frame holding `ra`, one call
// to the base `GameState_SaveState(self, serialiser)` and a plain return.

/// `GameState_SaveState` (0x233af0), reached through its override slot.
pub(crate) const OC326_GAME_STATE_SAVE_STATE: Callee = Callee {
    addr: 0x0023_3af0,
    f: ee::s06::GameState_SaveState,
};

/// `void GameStateEventSwitchLoad_SaveState(GameState* self, Serialiser* s)`
/// (0x236af8): save the base game state, forwarded unchanged.
pub fn game_state_event_switch_load_save_state(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(OC326_GAME_STATE_SAVE_STATE),
        0x0023_6b08,
    );
}

/// `void GameStateWarpToEvent_SaveState(GameState* self, Serialiser* s)`
/// (0x2361c8): save the base game state, forwarded unchanged.
pub fn game_state_warp_to_event_save_state(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(OC326_GAME_STATE_SAVE_STATE),
        0x0023_61d8,
    );
}

/// `void GameStateRace_SaveState(GameState* self, Serialiser* s)` (0x234eb8):
/// save the base game state, forwarded unchanged.
pub fn game_state_race_save_state(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(OC326_GAME_STATE_SAVE_STATE),
        0x0023_4ec8,
    );
}

/// `void GameStateCountdown_SaveState(GameState* self, Serialiser* s)`
/// (0x234de0): save the base game state, forwarded unchanged.
pub fn game_state_countdown_save_state(c: &mut Cpu) {
    tail_wrapper(
        c,
        None,
        Target::Direct(OC326_GAME_STATE_SAVE_STATE),
        0x0023_4df0,
    );
}

/// `void GameStateReset_Enter(GameStateReset* self)` (0x2355c0): enters the
/// "reset" game state. Records whether the script engine is in a transport
/// sequence (`+0x1c`), clears the user awards, reads the race setup
/// (`0x535bc8`: byte `+0x49` game path, byte `+0x48` game mode), picks the
/// state (`+0x0`, `+0x4`: path set: 1/0; no path and the section id (`+0x1bc`
/// of the section manager) in 0xe..0x10: 1/1; otherwise 4/0), switches the AI
/// to state 3 in mode 4, resets the BE state, re-initialises the missions and
/// runs the global scripts, enters the hub when the location type is 1, hides
/// the load indicator and starts the NIS lists (path set) or the idle list
/// (`+0x14`), then re-enables the audio engine.
///
/// NOT registered: `0x158e30` (reward bookkeeping) and `0x309f18` (global
/// script reset) are real, unslotted callees with their own loops, so no bounded
/// differential fixture exists.
pub fn game_state_reset_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), 0);
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
        call_ptr(c, 0x0027_aa80, 0x0023_55f0)?; // IsInTransportSequence
        let not_transport = c.g[V0][0] == 0;
        c.g[V0][0] = 1;
        if !not_transport {
            c.cycles += 1;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
        }
        c.cycles += 17;
        call_ptr(c, 0x0014_dc80, 0x0023_5604)?; // cBE_getBE
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0023_5610)?; // cBE_getInterface(0)
        c.g[S2][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dc80, 0x0023_5618)?; // cBE_getBE
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0xd;
        call_ptr(c, 0x0014_dd58, 0x0023_5624)?; // cBE_getInterface(13)
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0015_8e30, 0x0023_562c)?; // clear user awards
        c.g[V0][0] = 0x53_0000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
        c.g[S3][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
        let has_path = c.g[S3][0] != 0;
        c.g[S4][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
        let mut in_range = false;
        if !has_path {
            c.cycles += 8;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x78));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1bc));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe));
            c.g[V0][0] = (c.g[V0][0] < 3) as u64;
            in_range = c.g[V0][0] != 0;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 1;
            c.g[V0][0] = 1;
        }
        if has_path || in_range {
            c.cycles += 4;
            c.g[V1][0] = (c.g[S3][0] < 1) as u64;
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            c.sw((c.g[S1][0] as u32).wrapping_add(4), c.g[V1][0]);
            c.g[V0][0] = 4;
        } else {
            c.cycles += 4;
            c.g[V0][0] = 4;
            c.sw((c.g[S1][0] as u32).wrapping_add(4), 0);
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
        }
        c.cycles += 2;
        if c.g[S4][0] != c.g[V0][0] {
            c.g[A0][0] = 0;
        } else {
            c.cycles += 5;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            c.g[A1][0] = 3;
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            call_ptr(c, 0x0011_3b10, 0x0023_569c)?; // cAI_setAIState
            c.g[A0][0] = 0;
        }
        c.cycles += 13;
        c.g[S5][0] = 1;
        call_ptr(c, 0x0014_de68, 0x0023_56a8)?; // cBE_setState(0)
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
        call_ptr(c, 0x0030_9030, 0x0023_56b0)?; // WScriptMan_initMissions
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
        call_ptr(c, 0x0030_9f18, 0x0023_56b8)?; // WScriptMan_runGlobalScripts
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0014_4be0, 0x0023_56c0)?; // getLocationID
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0014_4cc0, 0x0023_56cc)?; // getLocationType
        let not_hub = c.g[V0][0] != c.g[S5][0];
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
        if !not_hub {
            c.cycles += 18;
            call_ptr(c, 0x0014_dc80, 0x0023_56dc)?; // cBE_getBE
            c.g[A1][0] = 1;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014_dd58, 0x0023_56e8)?; // cBE_getInterface(1)
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x0014_4be0, 0x0023_56f4)?; // getLocationID
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, 0x0014_6e10, 0x0023_5704)?; // setLastHub
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(8));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0023_5718)?; // player->onHub()
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
        }
        c.cycles += 11;
        c.g[V1][0] = c.g[S4][0] ^ 4;
        c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[V1][0] = (c.g[V1][0] != 0) as u64;
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = 0;
        c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
        c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x84));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x94));
        call_ptr(c, 0x001f_36d8, 0x0023_5740)?; // cBXLoadIndicator_Hide
        if !has_path {
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
            c.cycles += 2;
            let idle = c.g[V0][0] == 0;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
            if !idle {
                c.cycles += 4;
                c.g[A1][0] = 1;
                c.g[A2][0] = 1;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0027_8f68, 0x0023_57b0)?; // PlayNISList(1, 1, 0)
            }
        } else {
            c.cycles += 7;
            c.sw(c.g[S1][0] as u32, c.g[S5][0]);
            c.sw((c.g[S1][0] as u32).wrapping_add(4), 0);
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x550));
            c.g[V0][0] ^= 2;
            let not_mode2 = c.g[V0][0] != 0;
            c.g[A1][0] = 1;
            if !not_mode2 {
                c.cycles += 8;
                c.g[A2][0] = 0;
                c.g[A3][0] = 1;
                call_ptr(c, 0x0027_8f68, 0x0023_5770)?; // PlayNISList(1, 0, 1)
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x550));
                c.g[V0][0] ^= 2;
                let not_mode2 = c.g[V0][0] != 0;
                c.g[A1][0] = 0;
                if !not_mode2 {
                    c.cycles += 5;
                    c.g[A2][0] = 0;
                    c.g[A3][0] = 1;
                    call_ptr(c, 0x0027_8f68, 0x0023_5790)?; // PlayNISList(0, 0, 1)
                }
            }
        }
        c.cycles += 17;
        call_ptr(c, 0x0028_b180, 0x0023_57b8)?; // cAudioInterface_bxAudio
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x118));
        call_ptr(c, 0x002b_3a98, 0x0023_57c0)?; // audio func
        call_ptr(c, 0x0028_b180, 0x0023_57c8)?; // cAudioInterface_bxAudio
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x002a_4b68, 0x0023_57d0)?; // audio func
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `gp - off` as an address (the globals the game addresses off `gp`).
pub(crate) fn oc432_gp_minus(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(off)
}

/// `void GameStateReset_OverlayEnter(self)` (0x234f40): enters the reset
/// overlay. Clears `self + 0x18`, resets the replay, the script engine's NIS
/// list (`ScriptEngine_ResetNISList(engine, 0)`), unloads and reloads the
/// world scripts, resets the AI render slots, clears `self + 0x14`. On the
/// network path (byte 0x535c11 != 0) queues the intro scripts; otherwise, for
/// locations 14..=16 (`SectionMan + 0x1bc`), asks `IsCharProfileFlagClear`
/// whether the rider's first time on the track and, if so, sets the audio
/// flags (+0x5790 = 0, +0x578c = 1, +0x6254 = 1) and `self + 0x14 = 1`. Ends
/// with `self + 0x10 = 0`. Frame 0x40 (s0-s2).
pub fn game_state_reset_overlay_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = c.lw(oc432_gp_minus(c, 0x488));
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, S1);
        c.sw(this.wrapping_add(0x18), 0);
        c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(0x28));
        call_ptr(c, 0x0026_f228, 0x0023_4f68)?;
        c.g[A0][0] = c.lw(oc432_gp_minus(c, 0x84c));
        c.g[A1][0] = 0;
        call_ptr(c, 0x0027_90a0, 0x0023_4f74)?;
        c.g[A0][0] = c.lw(oc432_gp_minus(c, 0x84c));
        call_ptr(c, 0x0027_8cd8, 0x0023_4f7c)?;
        c.g[A0][0] = c.lw(oc432_gp_minus(c, 0x84c));
        call_ptr(c, 0x0027_8b98, 0x0023_4f84)?;
        call_ptr(c, 0x0014_dc80, 0x0023_4f8c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0023_4f98)?;
        c.sw(this.wrapping_add(0x14), 0);
        c.g[V0][0] = 0x53_0000;
        c.g[V1][0] = c.lw(oc432_gp_minus(c, 0x488));
        c.g[S0][0] = c.lb(r32(c, V0).wrapping_add(0x5c11));
        c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0xc));
        call_ptr(c, 0x0012_ab20, 0x0023_4fb0)?;
        c.g[V0][0] = c.lw(oc432_gp_minus(c, 0x488));
        c.g[A0][0] = c.lw(r32(c, V0).wrapping_add(0xc));
        call_ptr(c, 0x0012_ac48, 0x0023_4fbc)?;
        let network = c.g[S0][0] != 0;
        c.g[A0][0] = c.lw(oc432_gp_minus(c, 0x84c));
        if network {
            // Network path: queue the intro scripts.
            c.cycles += 2;
            c.g[A1][0] = 0;
            call_ptr(c, 0x0027_aaf8, 0x0023_5064)?;
            c.cycles += 1;
            c.sw(this.wrapping_add(0x10), 0);
        } else {
            'out: {
                c.cycles += 8;
                c.g[V0][0] = c.lw(oc432_gp_minus(c, 0x848));
                c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x84));
                c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x78));
                c.g[S2][0] = c.lw(r32(c, A0).wrapping_add(0x1bc));
                c.g[V0][0] = sx(r32(c, S2).wrapping_sub(0xe));
                c.g[V0][0] = (c.g[V0][0] < 3) as u64;
                if c.g[V0][0] == 0 {
                    c.sw(this.wrapping_add(0x10), 0);
                    break 'out;
                }
                c.cycles += 22;
                call_ptr(c, 0x0014_dc80, 0x0023_4fec)?;
                c.g[A1][0] = 1;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014_dd58, 0x0023_4ff8)?;
                c.g[A0][0] = c.lw(oc432_gp_minus(c, 0x488));
                c.g[S0][0] = c.g[V0][0];
                c.g[V0][0] = c.lw(r32(c, A0).wrapping_add(0xc));
                c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x40));
                c.g[A0][0] = c.lw(r32(c, V1).wrapping_add(0x18));
                c.g[A1][0] = c.lw(r32(c, A0).wrapping_add(0x6c0));
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x6c0));
                c.g[V1][0] = c.lh(r32(c, A1).wrapping_add(0x38));
                c.g[V0][0] = c.lw(r32(c, A1).wrapping_add(0x3c));
                let method = r32(c, V0);
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
                call_ptr(c, method, 0x0023_5024)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                c.g[A2][0] = c.g[S2][0];
                call_ptr(c, 0x0014_5d38, 0x0023_5034)?;
                if c.g[V0][0] == 0 {
                    c.sw(this.wrapping_add(0x10), 0);
                    break 'out;
                }
                c.cycles += 8;
                call_ptr(c, 0x0028_b180, 0x0023_5044)?;
                c.g[V1][0] = 1;
                let audio = r32(c, V0);
                c.sw(audio.wrapping_add(0x5790), 0);
                c.sw(audio.wrapping_add(0x6254), c.g[V1][0]);
                c.sw(audio.wrapping_add(0x578c), c.g[V1][0]);
                c.sw(this.wrapping_add(0x14), c.g[V1][0]);
                c.cycles += 1;
                c.sw(this.wrapping_add(0x10), 0);
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GameStateReset_Exit(GameState* self)` (0x235868): leave the reset state:
/// stop the script manager, run the rider/camera resets, clear the state's
/// `+0x14`, set up the mode handler (HandlerV06 for modes 5 and 6) and clear a
/// word of the first rider when the world is in single-player mode.
#[allow(unreachable_code)]
pub fn game_state_reset_exit(c: &mut Cpu) {
    guest(c, |c| {
        'b_235914: {
            c.cycles += 29;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S0][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            call_ptr(c, 0x00308f38, 0x0023588c)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
            call_ptr(c, 0x00308c60, 0x00235894)?;
            call_ptr(c, 0x0014dc80, 0x0023589c)?;
            c.g[A1][0] = 0u64;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0014dd58, 0x002358a8)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
            call_ptr(c, 0x0026f228, 0x002358b8)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
            call_ptr(c, 0x0026f7b8, 0x002358c4)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), 0u64);
            c.g[V0][0] = 0x530000;
            c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
            c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x49));
            if c.g[V1][0] != 0u64 {
                c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x48));
                break 'b_235914;
            } else {
                c.cycles += 14;
                call_ptr(c, 0x0014dc80, 0x002358e4)?;
                c.g[A1][0] = 0x1;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014dd58, 0x002358f0)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x00144bc0, 0x002358fc)?;
                c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x00145dd0, 0x00235910)?;
                c.g[V1][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x48));
                break 'b_235914;
            }
        }
        'b_235940: {
            'b_235928: {
                c.cycles += 3;
                c.g[V0][0] = 0x5;
                let t = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 0x6;
                if t {
                    break 'b_235928;
                } else {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        break 'b_235940;
                    } else {
                        break 'b_235928;
                    }
                }
            }
            c.cycles += 6;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A1][0] = 0u64;
            c.g[A2][0] = 0u64;
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
            Callee {
                addr: 0x00238510,
                f: ee::s06::GameModeMan_HandlerV06,
            }
            .call(c, 0x0023593c)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            break 'b_235940;
        }
        'b_23597c: {
            c.cycles += 6;
            c.g[V1][0] = 0x1;
            c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x84));
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x10));
            let t = c.g[V0][0] != c.g[V1][0];
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            if t {
                break 'b_23597c;
            } else {
                c.cycles += 9;
                c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                c.g[A2][0] = 0x50;
                c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x10));
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xb0));
                c.g[V1][0] = c.lw(c.g[A0][0] as u32);
                c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
                c.mult(0, 2, 2, 6);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x250), 0u64);
                break 'b_23597c;
            }
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

/// `void GameStateCountdown_Enter(GameState* self)` (0x234ad0): enter the race
/// countdown: reset the world pointers, announce the start (`cGameComm_OnCountdownStart`),
/// optionally reset the scripts and the world, and for modes 4..6 go to the next
/// state, otherwise start the countdown audio and play the countdown script event.
#[allow(unreachable_code)]
pub fn game_state_countdown_enter(c: &mut Cpu) {
    guest(c, |c| {
        'b_234afc: {
            c.cycles += 9;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
            let t = c.g[A0][0] == 0u64;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x8c), 0u64);
            if t {
                break 'b_234afc;
            } else {
                c.cycles += 2;
                Callee {
                    addr: 0x00258968,
                    f: ee::s06::cGameComm_OnCountdownStart,
                }
                .call(c, 0x00234afc)?;
                break 'b_234afc;
            }
        }
        'b_234b30: {
            c.cycles += 10;
            c.g[A0][0] = 0u64;
            call_ptr(c, 0x0014de68, 0x00234b04)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A1][0] = 0x7;
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
            call_ptr(c, 0x0014dd58, 0x00234b14)?;
            c.g[V1][0] = 0x530000;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4b30));
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            if t {
                break 'b_234b30;
            } else {
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                Callee {
                    addr: 0x002300f0,
                    f: ee::s06::GameModule_ResetScriptsAndWorld,
                }
                .call(c, 0x00234b2c)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                break 'b_234b30;
            }
        }
        'b_234bdc: {
            'b_234b80: {
                c.cycles += 16;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                call_ptr(c, 0x0026f228, 0x00234b38)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
                call_ptr(c, 0x0026f7b8, 0x00234b44)?;
                call_ptr(c, 0x0014dc80, 0x00234b4c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x00234b58)?;
                c.g[V1][0] = 0x530000;
                c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c10));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x4));
                c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                if t {
                    break 'b_234b80;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00233aa0,
                        f: ee::s06::GameState_GotoNext,
                    }
                    .call(c, 0x00234b78)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
                    if t {
                        break 'b_234bdc;
                    } else {
                        break 'b_234b80;
                    }
                }
            }
            c.cycles += 23;
            c.g[A1][0] = 0x4;
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            call_ptr(c, 0x00113b10, 0x00234b8c)?;
            call_ptr(c, 0x0028b180, 0x00234b94)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0029c418, 0x00234b9c)?;
            call_ptr(c, 0x0028b180, 0x00234ba4)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x3;
            call_ptr(c, 0x0029c420, 0x00234bb0)?;
            c.g[A0][0] = 0x480000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3fc8));
            call_ptr(c, 0x00317670, 0x00234bbc)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            Callee {
                addr: 0x00234be8,
                f: ee::s06::GameState_PlayScriptEvent,
            }
            .call(c, 0x00234bc8)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x48));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            call_ptr(c, 0x0039f840, 0x00234bd8)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x10));
            break 'b_234bdc;
        }
        c.cycles += 3;
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

/// `GameStateReset_Enter` (0x002355c0): Enter handler of the Reset state: clears the user awards, picks the next state from the course and race mode, restarts the mission scripts and the audio, and plays the transport sequence lists. `(self)`.
/// NOT INSTALLED (no OVERRIDES entry, no test): a real unported callee frees fixture memory (DLHeap_Consolidate runaway); needs a faithful object graph.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn game_state_reset_enter_2(c: &mut Cpu) {
    guest(c, |c| {
        'b_2355fc: {
            c.cycles += 14;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S1][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), 0u64);
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
            call_ptr(c, 0x0027aa80, 0x002355f0)?; // call_indirect
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = 0x1;
            if t {
                break 'b_2355fc;
            } else {
                c.cycles += 1;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                break 'b_2355fc;
            }
        }
        'b_235684: {
            'b_235674: {
                'b_235670: {
                    c.cycles += 17;
                    call_ptr(c, 0x0014dc80, 0x00235604)?; // call_indirect
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x00235610)?; // call_indirect
                    c.g[S2][0] = c.g[V0][0];
                    call_ptr(c, 0x0014dc80, 0x00235618)?; // call_indirect
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0xd;
                    call_ptr(c, 0x0014dd58, 0x00235624)?; // call_indirect
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x00158e30, 0x0023562c)?; // call_indirect
                    c.g[V0][0] = 0x530000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                    c.g[S3][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
                    let t = c.g[S3][0] != 0u64;
                    c.g[S4][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
                    if t {
                        break 'b_235670;
                    } else {
                        c.cycles += 8;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x78));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1bc));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe));
                        c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = 0x1;
                        if t {
                            break 'b_235674;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = 0x4;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x4), 0u64);
                            let t = true;
                            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                            if t {
                                break 'b_235684;
                            } else {
                                break 'b_235670;
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0x1;
                break 'b_235674;
            }
            c.cycles += 4;
            c.g[V1][0] = (c.g[S3][0] < 0x1u64) as u64;
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
            c.g[V0][0] = 0x4;
            break 'b_235684;
        }
        'b_2356a0: {
            c.cycles += 2;
            if c.g[S4][0] != c.g[V0][0] {
                c.g[A0][0] = 0u64;
                break 'b_2356a0;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                c.g[A1][0] = 0x3;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                call_ptr(c, 0x00113b10, 0x0023569c)?; // call_indirect
                c.g[A0][0] = 0u64;
                break 'b_2356a0;
            }
        }
        'b_23571c: {
            c.cycles += 13;
            c.g[S5][0] = 0x1;
            call_ptr(c, 0x0014de68, 0x002356a8)?; // call_indirect
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
            call_ptr(c, 0x00309030, 0x002356b0)?; // call_indirect
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
            call_ptr(c, 0x00309f18, 0x002356b8)?; // call_indirect
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x00144be0, 0x002356c0)?; // call_indirect
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x00144cc0, 0x002356cc)?; // call_indirect
            let t = c.g[V0][0] != c.g[S5][0];
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            if t {
                break 'b_23571c;
            } else {
                c.cycles += 18;
                call_ptr(c, 0x0014dc80, 0x002356dc)?; // call_indirect
                c.g[A1][0] = 0x1;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014dd58, 0x002356e8)?; // call_indirect
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x00144be0, 0x002356f4)?; // call_indirect
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x00146e10, 0x00235704)?; // call_indirect
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00235718)?; // call_indirect
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                break 'b_23571c;
            }
        }
        'b_2357b0: {
            'b_235798: {
                c.cycles += 11;
                c.g[V1][0] = c.g[S4][0] ^ 0x4;
                c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[V1][0] = (0u64 < c.g[V1][0]) as u64;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[A1][0] = 0u64;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x84));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x94));
                call_ptr(c, 0x001f36d8, 0x00235740)?; // call_indirect
                if c.g[S3][0] == 0u64 {
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                    break 'b_235798;
                } else {
                    c.cycles += 7;
                    c.sw(c.g[S1][0] as u32, c.g[S5][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x4), 0u64);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x550));
                    c.g[V0][0] = c.g[V0][0] ^ 0x2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = 0x1;
                    if t {
                        break 'b_2357b0;
                    } else {
                        c.cycles += 8;
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = 0x1;
                        call_ptr(c, 0x00278f68, 0x00235770)?; // call_indirect
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x550));
                        c.g[V0][0] = c.g[V0][0] ^ 0x2;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A1][0] = 0u64;
                        if t {
                            break 'b_2357b0;
                        } else {
                            c.cycles += 5;
                            c.g[A2][0] = 0u64;
                            c.g[A3][0] = 0x1;
                            call_ptr(c, 0x00278f68, 0x00235790)?; // call_indirect
                            let t = true;

                            if t {
                                break 'b_2357b0;
                            } else {
                                break 'b_235798;
                            }
                        }
                    }
                }
            }
            c.cycles += 2;
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
            if t {
                break 'b_2357b0;
            } else {
                c.cycles += 4;
                c.g[A1][0] = 0x1;
                c.g[A2][0] = 0x1;
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00278f68, 0x002357b0)?; // call_indirect
                break 'b_2357b0;
            }
        }
        c.cycles += 17;
        call_ptr(c, 0x0028b180, 0x002357b8)?; // call_indirect
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x118));
        call_ptr(c, 0x002b3a98, 0x002357c0)?; // call_indirect
        call_ptr(c, 0x0028b180, 0x002357c8)?; // call_indirect
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x002a4b68, 0x002357d0)?; // call_indirect
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    })
}

/// `GameStateRestartRace_Enter` (0x00235aa0): Enter handler of the NextHeat (restart race) state: resets the BE phase, fails the active local mission, restarts the heat, copies the AI characters into the player interface, queues the start sequence and shows the load indicator. `(self)`.
/// NOT INSTALLED (no OVERRIDES entry, no test): a real unported callee runs memcpy (0x4160e8) with a length from fixture data and runs away.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn game_state_restart_race_enter(c: &mut Cpu) {
    guest(c, |c| {
        'b_235c98: {
            'b_235b64: {
                'b_235b5c: {
                    'b_235b58: {
                        c.cycles += 40;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[S7][0] = c.g[A0][0];
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                        c.g[A0][0] = 0u64;
                        call_ptr(c, 0x0014de68, 0x00235ad8)?; // call_indirect
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                        call_ptr(c, 0x0030b7f8, 0x00235ae0)?; // call_indirect
                        call_ptr(c, 0x0014dc80, 0x00235ae8)?; // call_indirect
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x0014dd58, 0x00235af4)?; // call_indirect
                        c.g[S0][0] = c.g[V0][0];
                        call_ptr(c, 0x0014dc80, 0x00235afc)?; // call_indirect
                        c.g[A1][0] = 0x1;
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x0014dd58, 0x00235b08)?; // call_indirect
                        c.g[S5][0] = c.g[V0][0];
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[S6][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
                        c.g[A0][0] = c.g[S6][0];
                        call_ptr(c, 0x002382d8, 0x00235b1c)?; // GameModeMan_ResetMode
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                        call_ptr(c, 0x00230180, 0x00235b28)?; // GameModule_ResetRace
                        c.g[V0][0] = 0x530000;
                        c.g[V1][0] = 0x5;
                        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                        c.g[A0][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0x48));
                        let t = c.g[A0][0] == c.g[V1][0];
                        c.g[FP][0] = c.g[V0][0];
                        if t {
                            break 'b_235b58;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 0x6;
                            let t = c.g[A0][0] == c.g[V0][0];
                            c.g[V0][0] = 0x3;
                            if t {
                                break 'b_235b5c;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0x49));
                                let t = c.g[V0][0] == 0u64;
                                c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                                if t {
                                    break 'b_235b64;
                                } else {
                                    break 'b_235b58;
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 0x3;
                    break 'b_235b5c;
                }
                c.cycles += 2;
                let t = true;
                c.sw((c.g[S7][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                if t {
                    break 'b_235c98;
                } else {
                    break 'b_235b64;
                }
            }
            'b_235be4: {
                c.cycles += 16;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10));
                c.g[S2][0] = 0u64;
                c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x14));
                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                c.g[S3][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[S4][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, 0x00144d98, 0x00235b88)?; // call_indirect
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00235b9c)?; // call_indirect
                let t = (c.g[S3][0] as i64) <= 0;
                c.g[V1][0] = sx((c.g[S4][0] as u32) << 2);
                if t {
                    break 'b_235be4;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x40));
                    c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S4][0] as u32));
                    'l_235bb0: loop {
                        c.cycles += 13;
                        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x28));
                        c.g[A0][0] = c.g[S5][0];
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, 0x00147338, 0x00235bc0)?; // call_indirect
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        c.g[A2][0] = c.lw(c.g[S1][0] as u32);
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = c.g[S5][0];
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                        call_ptr(c, 0x001473d0, 0x00235bd8)?; // call_indirect
                        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S3][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S4][0] as u32));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00235bb0);
                            }
                            continue 'l_235bb0;
                        } else {
                            break 'b_235be4;
                        }
                    }
                }
            }
            'b_235c28: {
                c.cycles += 12;
                c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00235bf8)?; // call_indirect
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                call_ptr(c, 0x0012ab20, 0x00235c08)?; // call_indirect
                c.g[V0][0] = ((c.g[S3][0] as i64) < 2i64) as u64;
                if c.g[V0][0] != 0u64 {
                    c.sw((c.g[S7][0] as u32).wrapping_add(0x18), 0u64);
                    break 'b_235c28;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x0012ac48, 0x00235c24)?; // call_indirect
                    c.sw((c.g[S7][0] as u32).wrapping_add(0x18), 0u64);
                    break 'b_235c28;
                }
            }
            'b_235c50: {
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x5bc8));
                c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
                let t = c.g[V1][0] != 0u64;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                if t {
                    break 'b_235c50;
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = 0x1;
                    c.sw((c.g[S7][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    c.g[A1][0] = 0x1;
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x0027a860, 0x00235c4c)?; // call_indirect
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    break 'b_235c50;
                }
            }
            c.cycles += 18;
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x002790a0, 0x00235c58)?; // call_indirect
            c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x98));
            c.g[V0][0] = 0x2;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
            c.g[A1][0] = 0x3;
            if c.g[V1][0] == 0 {
                c.g[A1][0] = c.g[V0][0];
            }
            call_ptr(c, 0x0027aaf8, 0x00235c70)?; // call_indirect
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x00278f38, 0x00235c7c)?; // call_indirect
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A1][0] = 0x3;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x94));
            call_ptr(c, 0x001f36c0, 0x00235c90)?; // call_indirect
            c.sw((c.g[S7][0] as u32).wrapping_add(0x10), 0u64);
            c.sw((c.g[S7][0] as u32).wrapping_add(0x14), 0u64);
            break 'b_235c98;
        }
        c.cycles += 12;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        Ok(())
    })
}

/// `GameStateResults_Enter` (0x00236da0): Enter handler of the PostEvent (results) state: outside a freeroam path, clears the first-time flag of every rider, stops the script list, clears the timer and opens the results overlay (mode 7). `(self)`.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn game_state_results_enter(c: &mut Cpu) {
    guest(c, |c| {
        'b_236e2c: {
            c.cycles += 16;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[S3][0] = c.g[A0][0];
            call_ptr(c, 0x0014dc80, 0x00236dc0)?; // call_indirect
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014dd58, 0x00236dcc)?; // call_indirect
            c.g[V1][0] = 0x530000;
            c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
            c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x49));
            let t = c.g[V0][0] != 0u64;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
            if t {
                break 'b_236e2c;
            } else {
                c.cycles += 9;
                c.g[S0][0] = 0u64;
                call_ptr(c, 0x0014dc80, 0x00236de8)?; // call_indirect
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0014dd58, 0x00236df4)?; // call_indirect
                c.g[S2][0] = c.g[V0][0];
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                if t {
                    break 'b_236e2c;
                } else {
                    c.cycles += 1;
                    c.g[A1][0] = c.g[S0][0];
                    'l_236e08: loop {
                        c.cycles += 8;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A2][0] = 0u64;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        call_ptr(c, 0x00145cb0, 0x00236e18)?; // call_indirect
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x30));
                        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A1][0] = c.g[S0][0];
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00236e08);
                            }
                            continue 'l_236e08;
                        } else {
                            c.cycles += 1;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                            break 'b_236e2c;
                        }
                    }
                }
            }
        }
        c.cycles += 12;
        c.g[A1][0] = 0u64;
        call_ptr(c, 0x002790a0, 0x00236e34)?; // call_indirect
        c.sw((c.g[S3][0] as u32).wrapping_add(0x8), 0u64);
        c.g[A0][0] = 0x7;
        call_ptr(c, 0x0020a8f8, 0x00236e40)?; // call_indirect
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        Ok(())
    })
}

/// `GameState_LoadState` (0x233b28), the shared body of the `*_LoadState`
/// wrappers below. It loads `a1`'s vtable and calls its slot 0x14 with
/// `a0 = a1 + (lh slot 0x10)`; the wrappers only forward `a0`/`a1`.
pub(crate) const GAME_STATE_LOAD_STATE: Callee = Callee {
    addr: 0x0023_3b28,
    f: ee::s06::f_00233b28,
};

/// `GameStateCountdown_LoadState(state)` (0x234e00): `GameState_LoadState`
/// on the same arguments; returns to 0x234e10.
pub fn game_state_countdown_load_state(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(GAME_STATE_LOAD_STATE), 0x0023_4e10)
}

/// `GameStateRace_LoadState(state)` (0x234ed8): `GameState_LoadState`; returns
/// to 0x234ee8.
pub fn game_state_race_load_state(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(GAME_STATE_LOAD_STATE), 0x0023_4ee8)
}

/// `GameStateWarpToEvent_LoadState(state)` (0x2361e8): `GameState_LoadState`;
/// returns to 0x2361f8.
pub fn game_state_warp_to_event_load_state(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(GAME_STATE_LOAD_STATE), 0x0023_61f8)
}

/// `GameStateEventSwitchLoad_LoadState(state)` (0x236b18): `GameState_LoadState`;
/// returns to 0x236b28.
pub fn game_state_event_switch_load_load_state(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(GAME_STATE_LOAD_STATE), 0x0023_6b28)
}

/// `GameStateResetAI_LoadState(state)` (0x236b90): `GameState_LoadState`;
/// returns to 0x236ba0.
pub fn game_state_reset_ai_load_state(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(GAME_STATE_LOAD_STATE), 0x0023_6ba0)
}

/// `GameStatePreRaceMenu_LoadState(state)` (0x236d80): `GameState_LoadState`;
/// returns to 0x236d90.
pub fn game_state_pre_race_menu_load_state(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(GAME_STATE_LOAD_STATE), 0x0023_6d90)
}

/// `GameStateResults_LoadState(state)` (0x236ee8): `GameState_LoadState`;
/// returns to 0x236ef8.
pub fn game_state_results_load_state(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(GAME_STATE_LOAD_STATE), 0x0023_6ef8)
}

/// `GameStateFinishCinematic_LoadState(state)` (0x236fc8): `GameState_LoadState`;
/// returns to 0x236fd8.
pub fn game_state_finish_cinematic_load_state(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(GAME_STATE_LOAD_STATE), 0x0023_6fd8)
}

/// Entry and end of `InputSync_Update` (0x2373b8..0x237944).
pub(crate) const UPDATE_ENTRY: u32 = 0x0023_73b8;

pub(crate) const UPDATE_END: u32 = 0x0023_7944;

/// Its stack frame (`addiu sp, sp, -0x160`).
pub(crate) const UPDATE_FRAME: u32 = 0x160;

/// The shared epilogue block (restores s0..s4 and `ra`, returns).
pub(crate) const UPDATE_EPILOGUE: u32 = 0x0023_7924;

/// `InputSync_UnpackSample(this, src, dst)` (0x237be8).
pub(crate) const UPDATE_UNPACK_SAMPLE: Callee = Callee {
    addr: 0x0023_7be8,
    f: ee::s06::f_00237be8,
};

/// `InputSync_SendSample(this, sample)` (0x237a98).
pub(crate) const UPDATE_SEND_SAMPLE: Callee = Callee {
    addr: 0x0023_7a98,
    f: ee::s06::f_00237a98,
};

/// `cGameComm_PopPacket(gc, channel, dst, size)` (0x257bd8).
pub(crate) const UPDATE_POP_PACKET: Callee = Callee {
    addr: 0x0025_7bd8,
    f: ee::s06::f_00257bd8,
};

/// `cGameComm_GetInputDelay(gc)` (0x258360).
pub(crate) const UPDATE_INPUT_DELAY: Callee = Callee {
    addr: 0x0025_8360,
    f: ee::s06::f_00258360,
};

/// `gp - off`: the address of a gp-relative global.
#[inline(always)]
pub(crate) fn update_gp_slot(c: &Cpu, off: u32) -> u32 {
    r32(c, GP).wrapping_sub(off)
}

/// `off(reg)`: an address off a register.
#[inline(always)]
pub(crate) fn update_reg_off(c: &Cpu, reg: usize, off: u32) -> u32 {
    r32(c, reg).wrapping_add(off)
}

/// Signed `a < b` as the compiled `slt` leaves it (0 or 1, 64-bit).
#[inline(always)]
pub(crate) fn update_slt(a: u64, b: u64) -> u64 {
    ((a as i64) < (b as i64)) as u64
}

/// Event check at a loop back edge with the loop head `head`: when an event
/// is due, deliver it with `pc = head`; an unwind request returns from the
/// body (the trampoline re-enters at `head`).
#[inline(always)]
pub(crate) fn update_back_edge(c: &mut Cpu, head: u32) -> Flow {
    if event_due(c) && deliver_event(c, head) {
        return Err(Unwound);
    }
    Ok(())
}

/// One 0x20-byte step of the sample copy with unaligned accesses (`ldl`/`ldr`
/// pairs, the generated loop at 0x2374ac), from `src` to `dst`.
#[inline(always)]
pub(crate) fn update_copy_unaligned(c: &mut Cpu, src: u32, dst: u32) {
    c.ldl(A1, src.wrapping_add(0x7));
    c.ldr(A1, src);
    c.ldl(A3, src.wrapping_add(0xf));
    c.ldr(A3, src.wrapping_add(0x8));
    c.ldl(T2, src.wrapping_add(0x17));
    c.ldr(T2, src.wrapping_add(0x10));
    c.ldl(T3, src.wrapping_add(0x1f));
    c.ldr(T3, src.wrapping_add(0x18));
    c.sdl(A1, dst.wrapping_add(0x7));
    c.sdr(A1, dst);
    c.sdl(A3, dst.wrapping_add(0xf));
    c.sdr(A3, dst.wrapping_add(0x8));
    c.sdl(T2, dst.wrapping_add(0x17));
    c.sdr(T2, dst.wrapping_add(0x10));
    c.sdl(T3, dst.wrapping_add(0x1f));
    c.sdr(T3, dst.wrapping_add(0x18));
}

/// One 0x20-byte step of the sample copy with aligned doublewords (the
/// generated loop at 0x237504), from `src` to `dst`.
#[inline(always)]
pub(crate) fn update_copy_aligned(c: &mut Cpu, src: u32, dst: u32) {
    c.g[A1][0] = c.ld(src);
    c.g[A3][0] = c.ld(src.wrapping_add(0x8));
    c.g[T2][0] = c.ld(src.wrapping_add(0x10));
    c.g[T3][0] = c.ld(src.wrapping_add(0x18));
    let (a1, a3, t2, t3) = (c.g[A1][0], c.g[A3][0], c.g[T2][0], c.g[T3][0]);
    c.sd(dst, a1);
    c.sd(dst.wrapping_add(0x8), a3);
    c.sd(dst.wrapping_add(0x10), t2);
    c.sd(dst.wrapping_add(0x18), t3);
}

/// `void InputSync_Update(InputSync *self)` (0x2373b8): for each synced pad,
/// fill its sample ring. A remote pad (no source pad) takes the network
/// packets while the ring is short (or the game comms are idle, a neutral
/// sample otherwise). A local pad (a source pad) samples its buttons every
/// `numsamples` ticks, sends them, and refreshes the send limit from the
/// comms' input delay. Returns early while the comms are below state 14.
///
/// The body is the generated block structure, one arm per block, so a call
/// that unwinds or an event that asks for a thread switch resumes at the
/// same block (`c.pc`), as the compiled code does. The struct fields:
/// self `+0` sendlimit, `+4` latency, `+8` numsamples, `+0xc` tick, `+0x10`
/// numsyncedpads, `+0x14` syncedpads (stride 0x18: `+0` samples, `+4`
/// readpos, `+8` writepos, `+0xc` sourcepad).
pub fn input_sync_update(c: &mut Cpu) {
    guest(c, input_sync_update_body);
}

pub(crate) fn input_sync_update_body(c: &mut Cpu) -> Flow {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            // Prologue: `if (GameComm()->mState < 14) return;`.
            0x0023_73b8 => {
                c.cycles += 12;
                let frame = Frame::open(c, UPDATE_FRAME);
                let gc_slot = update_gp_slot(c, 0x204);
                c.g[V1][0] = c.lw(gc_slot);
                frame.save(c, S2, 0x130);
                frame.save(c, S0, 0x150);
                frame.save(c, S1, 0x140);
                frame.save(c, S3, 0x120);
                frame.save(c, S4, 0x110);
                frame.save_ra(c, 0x100);
                let state_slot = update_reg_off(c, V1, 0x70);
                c.g[V0][0] = c.lw(state_slot);
                c.g[V0][0] = update_slt(c.g[V0][0], 14);
                let t = c.g[V0][0] != 0;
                c.g[S2][0] = c.g[A0][0];
                pc = if t { UPDATE_EPILOGUE } else { 0x0023_73e8 };
                continue;
            }
            // `for (i = 0; i < self->numsyncedpads; i++)`: the test before the loop.
            0x0023_73e8 => {
                c.cycles += 3;
                let count_slot = update_reg_off(c, S2, 0x10);
                c.g[V0][0] = c.lw(count_slot);
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[S0][0] = 0;
                pc = if t { UPDATE_EPILOGUE } else { 0x0023_73f4 };
                continue;
            }
            0x0023_73f4 => {
                c.cycles += 3;
                set_word(c, S4, r32(c, SP).wrapping_add(0x70));
                c.g[V0][0] = 0x18;
                pc = 0x0023_7400;
                continue;
            }
            // Loop head: `psp = &syncedpads[i]`; a local pad (source pad set) goes to 0x237720.
            0x0023_7400 => {
                c.cycles += 6;
                c.mult(0, V0, S0, V0);
                set_word(c, V0, r32(c, V0).wrapping_add(0x14));
                set_word(c, S1, r32(c, S2).wrapping_add(r32(c, V0)));
                let pad_source_slot = update_reg_off(c, S1, 0xc);
                c.g[V1][0] = c.lw(pad_source_slot);
                if c.g[V1][0] != 0 {
                    let sendlimit_slot = update_reg_off(c, S2, 0);
                    c.g[V0][0] = c.lw(sendlimit_slot);
                    pc = 0x0023_7720;
                    continue;
                }
                pc = 0x0023_7418;
                continue;
            }
            // Remote pad: `if (sp.sourcepad == 0)`, with `net` from the comms state.
            0x0023_7418 => {
                c.cycles += 4;
                c.g[V1][0] = c.lw(update_gp_slot(c, 0x204));
                let sync_slot = update_reg_off(c, V1, 0x40);
                c.g[V0][0] = c.lw(sync_slot);
                let t = c.g[V0][0] == 0;
                c.g[A0][0] = 0;
                pc = if t { 0x0023_743c } else { 0x0023_7428 };
                continue;
            }
            0x0023_7428 => {
                c.cycles += 3;
                let error_slot = update_reg_off(c, V1, 0x4);
                c.g[V0][0] = c.lw(error_slot);
                let t = c.g[V0][0] != 0;
                pc = if t { 0x0023_743c } else { 0x0023_7434 };
                continue;
            }
            0x0023_7434 => {
                c.cycles += 2;
                let connected_slot = update_reg_off(c, V1, 0);
                c.g[V0][0] = c.lw(connected_slot);
                c.g[A0][0] = (0u64 < c.g[V0][0]) as u64;
                pc = 0x0023_743c;
                continue;
            }
            // `net` is `a0`; s3 = i + 1 for the loop tail.
            0x0023_743c => {
                c.cycles += 2;
                let t = c.g[A0][0] == 0;
                set_word(c, S3, r32(c, S0).wrapping_add(0x1));
                pc = if t { 0x0023_75a4 } else { 0x0023_7444 };
                continue;
            }
            // Network `while` loop: its condition sits at 0x23755c.
            0x0023_7444 => {
                c.cycles += 2;
                let latency_slot = update_reg_off(c, S2, 0x4);
                c.g[V0][0] = c.lw(latency_slot);
                pc = 0x0023_755c;
                continue;
            }
            0x0023_744c => {
                c.cycles += 1;
                pc = 0x0023_7450;
                continue;
            }
            // Network body: decode the packet (a2 holds the packet index s0 here).
            0x0023_7450 => {
                c.cycles += 6;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[SP][0];
                UPDATE_UNPACK_SAMPLE.call(c, 0x0023_745c)?;
                pc = 0x0023_745c;
                continue;
            }
            // Resume after the unpack.
            0x0023_745c => {
                let numsamples_slot = update_reg_off(c, S2, 0x8);
                c.g[V0][0] = c.lw(numsamples_slot);
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[A3][0] = 0;
                pc = if t { 0x0023_7558 } else { 0x0023_7468 };
                continue;
            }
            // Network copy loop over numsamples: write the sample into the ring.
            0x0023_7468 => {
                c.cycles += 4;
                c.g[T1][0] = 0;
                c.g[T0][0] = 0x64;
                let latency_slot = update_reg_off(c, S2, 0x4);
                c.g[A0][0] = c.lw(latency_slot);
                pc = 0x0023_7478;
                continue;
            }
            0x0023_7478 => {
                c.cycles += 4;
                c.g[A2][0] = c.g[SP][0];
                let writepos_slot = update_reg_off(c, S1, 0x8);
                c.g[V0][0] = c.lw(writepos_slot);
                pc = 0x0023_7488;
                continue;
            }
            0x0023_7488 => {
                c.cycles += 8;
                c.div(0, V0, A0);
                let samples_slot = update_reg_off(c, S1, 0);
                c.g[A1][0] = c.lw(samples_slot);
                c.g[V1][0] = c.hi[0];
                c.mult(0, V1, V1, T0);
                set_word(c, V1, r32(c, V1).wrapping_add(r32(c, A1)));
                c.g[V0][0] = c.g[V1][0] & 0x7;
                let t = c.g[V0][0] == 0;
                set_word(c, A0, r32(c, A3).wrapping_add(0x1));
                pc = if t { 0x0023_7500 } else { 0x0023_74a8 };
                continue;
            }
            0x0023_74a8 => {
                c.cycles += 1;
                set_word(c, V0, r32(c, SP).wrapping_add(0x60));
                pc = 0x0023_74ac;
                continue;
            }
            // Unaligned 0x20-byte copy steps until a2 reaches the sample end.
            0x0023_74ac => {
                c.cycles += 19;
                let src = r32(c, A2);
                let dst = r32(c, V1);
                update_copy_unaligned(c, src, dst);
                set_word(c, A2, src.wrapping_add(0x20));
                let t = c.g[A2][0] != c.g[V0][0];
                set_word(c, V1, dst.wrapping_add(0x20));
                if t {
                    update_back_edge(c, 0x0023_74ac)?;
                    pc = 0x0023_74ac;
                    continue;
                }
                pc = 0x0023_74f8;
                continue;
            }
            0x0023_74f8 => {
                c.cycles += 2;
                pc = 0x0023_7530;
                continue;
            }
            0x0023_7500 => {
                c.cycles += 1;
                set_word(c, V0, r32(c, SP).wrapping_add(0x60));
                pc = 0x0023_7504;
                continue;
            }
            // Aligned doubleword copy steps.
            0x0023_7504 => {
                c.cycles += 11;
                let src = r32(c, A2);
                let dst = r32(c, V1);
                update_copy_aligned(c, src, dst);
                set_word(c, A2, src.wrapping_add(0x20));
                let t = c.g[A2][0] != c.g[V0][0];
                set_word(c, V1, dst.wrapping_add(0x20));
                if t {
                    update_back_edge(c, 0x0023_7504)?;
                    pc = 0x0023_7504;
                    continue;
                }
                pc = 0x0023_7530;
                continue;
            }
            // The last word, then `s1->writepos++` and the inner `for` test.
            0x0023_7530 => {
                c.cycles += 10;
                let src = r32(c, A2);
                c.g[V0][0] = c.lw(src);
                let dst = r32(c, V1);
                let word = c.g[V0][0];
                c.sw(dst, word);
                let writepos_slot = update_reg_off(c, S1, 0x8);
                c.g[V0][0] = c.lw(writepos_slot);
                c.g[A3][0] = c.g[A0][0];
                set_word(c, V0, r32(c, V0).wrapping_add(0x1));
                let writepos_value = c.g[V0][0];
                c.sw(writepos_slot, writepos_value);
                let numsamples_slot = update_reg_off(c, S2, 0x8);
                c.g[V1][0] = c.lw(numsamples_slot);
                c.g[V1][0] = update_slt(c.g[A3][0], c.g[V1][0]);
                if c.g[V1][0] != 0 {
                    let latency_slot = update_reg_off(c, S2, 0x4);
                    c.g[A0][0] = c.lw(latency_slot);
                    update_back_edge(c, 0x0023_7478)?;
                    pc = 0x0023_7478;
                    continue;
                }
                pc = 0x0023_7558;
                continue;
            }
            0x0023_7558 => {
                c.cycles += 1;
                let latency_slot = update_reg_off(c, S2, 0x4);
                c.g[V0][0] = c.lw(latency_slot);
                pc = 0x0023_755c;
                continue;
            }
            // While condition: `writepos - readpos < latency + 1 - numsamples`.
            // s0 becomes the compressed-sample buffer (sp + 0x70).
            0x0023_755c => {
                c.cycles += 9;
                let writepos_slot = update_reg_off(c, S1, 0x8);
                c.g[V1][0] = c.lw(writepos_slot);
                let readpos_slot = update_reg_off(c, S1, 0x4);
                c.g[A1][0] = c.lw(readpos_slot);
                set_word(c, V0, r32(c, V0).wrapping_add(0x1));
                let numsamples_slot = update_reg_off(c, S2, 0x8);
                c.g[A0][0] = c.lw(numsamples_slot);
                set_word(c, V1, r32(c, V1).wrapping_sub(r32(c, A1)));
                set_word(c, V0, r32(c, V0).wrapping_sub(r32(c, A0)));
                c.g[V1][0] = update_slt(c.g[V1][0], c.g[V0][0]);
                let t = c.g[V1][0] == 0;
                set_word(c, S0, r32(c, SP).wrapping_add(0x70));
                if t {
                    pc = 0x0023_7910;
                    continue;
                }
                pc = 0x0023_7580;
                continue;
            }
            // Network: `cGameComm_PopPacket(GameComm(), 0, compressed, 0x1c)`.
            0x0023_7580 => {
                c.cycles += 7;
                c.g[A0][0] = c.lw(update_gp_slot(c, 0x204));
                c.g[A1][0] = 0;
                c.g[A2][0] = c.g[S0][0];
                c.g[A3][0] = 0x1c;
                UPDATE_POP_PACKET.call(c, 0x0023_7594)?;
                pc = 0x0023_7594;
                continue;
            }
            // Resume after `PopPacket`: a packet came in, so unpack it.
            0x0023_7594 => {
                let t = c.g[V0][0] != 0;
                c.g[A2][0] = c.g[S0][0];
                if t {
                    update_back_edge(c, 0x0023_7450)?;
                    pc = 0x0023_7450;
                    continue;
                }
                pc = 0x0023_759c;
                continue;
            }
            0x0023_759c => {
                c.cycles += 2;
                let t = true;
                let numsamples_slot = update_reg_off(c, S2, 0x10);
                c.g[V0][0] = c.lw(numsamples_slot);
                pc = if t { 0x0023_7914 } else { 0x0023_75a4 };
                continue;
            }
            // Remote pad with the comms idle: a neutral sample, unpacked,
            // then the ring fill.
            0x0023_75a4 => {
                c.cycles += 20;
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x1c;
                call_ptr(c, 0x003e_6448, 0x0023_75b4)?;
                pc = 0x0023_75b4;
                continue;
            }
            // Resume after `EA_MemSet`: flag the neutral sample as 24 buttons, unpack it.
            0x0023_75b4 => {
                c.g[V0][0] = 0x18;
                c.g[A0][0] = c.g[S2][0];
                let compressed_buttons = r32(c, SP).wrapping_add(0x70);
                let value = c.g[V0][0];
                c.sb(compressed_buttons, value);
                c.g[A1][0] = c.g[SP][0];
                c.g[A2][0] = c.g[S4][0];
                UPDATE_UNPACK_SAMPLE.call(c, 0x0023_75cc)?;
                pc = 0x0023_75cc;
                continue;
            }
            // Resume after the second unpack: fill the ring while it is short.
            0x0023_75cc => {
                let latency_slot = update_reg_off(c, S2, 0x4);
                c.g[V0][0] = c.lw(latency_slot);
                let writepos_slot = update_reg_off(c, S1, 0x8);
                c.g[V1][0] = c.lw(writepos_slot);
                let readpos_slot = update_reg_off(c, S1, 0x4);
                c.g[A0][0] = c.lw(readpos_slot);
                set_word(c, V0, r32(c, V0).wrapping_add(0x1));
                let numsamples_slot = update_reg_off(c, S2, 0x8);
                c.g[A1][0] = c.lw(numsamples_slot);
                set_word(c, V1, r32(c, V1).wrapping_sub(r32(c, A0)));
                set_word(c, V0, r32(c, V0).wrapping_sub(r32(c, A1)));
                c.g[V1][0] = update_slt(c.g[V1][0], c.g[V0][0]);
                if c.g[V1][0] == 0 {
                    let count_slot = update_reg_off(c, S2, 0x10);
                    c.g[V0][0] = c.lw(count_slot);
                    pc = 0x0023_7914;
                    continue;
                }
                pc = 0x0023_75f4;
                continue;
            }
            0x0023_75f4 => {
                c.cycles += 1;
                pc = 0x0023_75f8;
                continue;
            }
            // Ring fill loop head (`numsamples <= 0` skips the body).
            0x0023_75f8 => {
                c.cycles += 2;
                let t = (c.g[A1][0] as i64) <= 0;
                c.g[A3][0] = 0;
                pc = if t { 0x0023_76f0 } else { 0x0023_7600 };
                continue;
            }
            0x0023_7600 => {
                c.cycles += 4;
                c.g[T1][0] = 0;
                c.g[T0][0] = 0x64;
                let latency_slot = update_reg_off(c, S2, 0x4);
                c.g[A0][0] = c.lw(latency_slot);
                pc = 0x0023_7610;
                continue;
            }
            0x0023_7610 => {
                c.cycles += 4;
                c.g[A2][0] = c.g[SP][0];
                let writepos_slot = update_reg_off(c, S1, 0x8);
                c.g[V0][0] = c.lw(writepos_slot);
                pc = 0x0023_7620;
                continue;
            }
            0x0023_7620 => {
                c.cycles += 8;
                c.div(0, V0, A0);
                let samples_slot = update_reg_off(c, S1, 0);
                c.g[A1][0] = c.lw(samples_slot);
                c.g[V1][0] = c.hi[0];
                c.mult(0, V1, V1, T0);
                set_word(c, V1, r32(c, V1).wrapping_add(r32(c, A1)));
                c.g[V0][0] = c.g[V1][0] & 0x7;
                let t = c.g[V0][0] == 0;
                set_word(c, A0, r32(c, A3).wrapping_add(0x1));
                pc = if t { 0x0023_7698 } else { 0x0023_7640 };
                continue;
            }
            0x0023_7640 => {
                c.cycles += 1;
                set_word(c, V0, r32(c, SP).wrapping_add(0x60));
                pc = 0x0023_7644;
                continue;
            }
            0x0023_7644 => {
                c.cycles += 19;
                let src = r32(c, A2);
                let dst = r32(c, V1);
                update_copy_unaligned(c, src, dst);
                set_word(c, A2, src.wrapping_add(0x20));
                let t = c.g[A2][0] != c.g[V0][0];
                set_word(c, V1, dst.wrapping_add(0x20));
                if t {
                    update_back_edge(c, 0x0023_7644)?;
                    pc = 0x0023_7644;
                    continue;
                }
                pc = 0x0023_7690;
                continue;
            }
            0x0023_7690 => {
                c.cycles += 2;
                pc = 0x0023_76c8;
                continue;
            }
            0x0023_7698 => {
                c.cycles += 1;
                set_word(c, V0, r32(c, SP).wrapping_add(0x60));
                pc = 0x0023_769c;
                continue;
            }
            0x0023_769c => {
                c.cycles += 11;
                let src = r32(c, A2);
                let dst = r32(c, V1);
                update_copy_aligned(c, src, dst);
                set_word(c, A2, src.wrapping_add(0x20));
                let t = c.g[A2][0] != c.g[V0][0];
                set_word(c, V1, dst.wrapping_add(0x20));
                if t {
                    update_back_edge(c, 0x0023_769c)?;
                    pc = 0x0023_769c;
                    continue;
                }
                pc = 0x0023_76c8;
                continue;
            }
            0x0023_76c8 => {
                c.cycles += 10;
                let src = r32(c, A2);
                c.g[V0][0] = c.lw(src);
                let dst = r32(c, V1);
                let word = c.g[V0][0];
                c.sw(dst, word);
                let writepos_slot = update_reg_off(c, S1, 0x8);
                c.g[V0][0] = c.lw(writepos_slot);
                c.g[A3][0] = c.g[A0][0];
                set_word(c, V0, r32(c, V0).wrapping_add(0x1));
                let writepos_value = c.g[V0][0];
                c.sw(writepos_slot, writepos_value);
                let numsamples_slot = update_reg_off(c, S2, 0x8);
                c.g[V1][0] = c.lw(numsamples_slot);
                c.g[V1][0] = update_slt(c.g[A3][0], c.g[V1][0]);
                if c.g[V1][0] != 0 {
                    let latency_slot = update_reg_off(c, S2, 0x4);
                    c.g[A0][0] = c.lw(latency_slot);
                    update_back_edge(c, 0x0023_7610)?;
                    pc = 0x0023_7610;
                    continue;
                }
                pc = 0x0023_76f0;
                continue;
            }
            // Ring fill done: the while condition again (writepos - readpos < ...).
            0x0023_76f0 => {
                c.cycles += 10;
                let latency_slot = update_reg_off(c, S2, 0x4);
                c.g[V0][0] = c.lw(latency_slot);
                let writepos_slot = update_reg_off(c, S1, 0x8);
                c.g[V1][0] = c.lw(writepos_slot);
                let readpos_slot = update_reg_off(c, S1, 0x4);
                c.g[A0][0] = c.lw(readpos_slot);
                set_word(c, V0, r32(c, V0).wrapping_add(0x1));
                let numsamples_slot = update_reg_off(c, S2, 0x8);
                c.g[A1][0] = c.lw(numsamples_slot);
                set_word(c, V1, r32(c, V1).wrapping_sub(r32(c, A0)));
                set_word(c, V0, r32(c, V0).wrapping_sub(r32(c, A1)));
                c.g[V1][0] = update_slt(c.g[V1][0], c.g[V0][0]);
                let t = c.g[V1][0] != 0;
                if t {
                    update_back_edge(c, 0x0023_75f8)?;
                    pc = 0x0023_75f8;
                    continue;
                }
                pc = 0x0023_7718;
                continue;
            }
            0x0023_7718 => {
                c.cycles += 2;
                let t = true;
                let count_slot = update_reg_off(c, S2, 0x10);
                c.g[V0][0] = c.lw(count_slot);
                pc = if t { 0x0023_7914 } else { 0x0023_7720 };
                continue;
            }
            // Local pad (source pad set): the ring fill is due while the ring is short.
            0x0023_7720 => {
                c.cycles += 9;
                let writepos_slot = update_reg_off(c, S1, 0x8);
                c.g[V1][0] = c.lw(writepos_slot);
                let readpos_slot = update_reg_off(c, S1, 0x4);
                c.g[A0][0] = c.lw(readpos_slot);
                set_word(c, V0, r32(c, V0).wrapping_add(0x1));
                let numsamples_slot = update_reg_off(c, S2, 0x8);
                c.g[A1][0] = c.lw(numsamples_slot);
                set_word(c, V1, r32(c, V1).wrapping_sub(r32(c, A0)));
                set_word(c, V0, r32(c, V0).wrapping_sub(r32(c, A1)));
                c.g[V1][0] = update_slt(c.g[V1][0], c.g[V0][0]);
                let t = c.g[V1][0] == 0;
                set_word(c, S3, r32(c, S0).wrapping_add(0x1));
                if t {
                    pc = 0x0023_7910;
                    continue;
                }
                pc = 0x0023_7744;
                continue;
            }
            // `if (++self->tick % self->numsamples == 0)`.
            0x0023_7744 => {
                c.cycles += 6;
                let tick_slot = update_reg_off(c, S2, 0xc);
                c.g[V0][0] = c.lw(tick_slot);
                set_word(c, V0, r32(c, V0).wrapping_add(0x1));
                c.div(0, V0, A1);
                let tick_value = c.g[V0][0];
                c.sw(tick_slot, tick_value);
                pc = 0x0023_775c;
                continue;
            }
            0x0023_775c => {
                c.cycles += 3;
                c.g[V1][0] = c.hi[0];
                if c.g[V1][0] != 0 {
                    let count_slot = update_reg_off(c, S2, 0x10);
                    c.g[V0][0] = c.lw(count_slot);
                    pc = 0x0023_7914;
                    continue;
                }
                pc = 0x0023_7768;
                continue;
            }
            // Local sample: the pad's buttons go into the sample at sp + 0x90.
            0x0023_7768 => {
                c.cycles += 6;
                let pad_slot = update_reg_off(c, S1, 0xc);
                c.g[V1][0] = c.lw(pad_slot);
                c.g[A1][0] = 0;
                set_word(c, S0, r32(c, SP).wrapping_add(0x90));
                let buttons_slot = r32(c, V1);
                c.g[V0][0] = c.lw(buttons_slot);
                let t = (c.g[V0][0] as i64) <= 0;
                let sample_count_slot = r32(c, SP).wrapping_add(0x90);
                let value = c.g[V0][0];
                c.sw(sample_count_slot, value);
                pc = if t { 0x0023_77c0 } else { 0x0023_7780 };
                continue;
            }
            0x0023_7780 => {
                c.cycles += 4;
                c.g[A2][0] = c.g[V1][0];
                set_word(c, A0, r32(c, SP).wrapping_add(0x94));
                set_word(c, V1, r32(c, A2).wrapping_add(0x4));
                pc = 0x0023_7790;
                continue;
            }
            0x0023_7790 => {
                c.cycles += 5;
                let count_slot = r32(c, A2);
                c.g[V0][0] = c.lw(count_slot);
                c.mtc1(0, 0);
                c.g[V0][0] = update_slt(c.g[A1][0], c.g[V0][0]);
                if c.g[V0][0] != 0 {
                    let float_slot = r32(c, V1);
                    c.lwc1(0, float_slot);
                    pc = 0x0023_77a4;
                    continue;
                }
                pc = 0x0023_77a4;
                continue;
            }
            0x0023_77a4 => {
                c.cycles += 7;
                let out_slot = r32(c, A0);
                c.swc1(0, out_slot);
                set_word(c, A1, r32(c, A1).wrapping_add(0x1));
                set_word(c, A0, r32(c, A0).wrapping_add(0x4));
                let count_slot = r32(c, SP).wrapping_add(0x90);
                c.g[V0][0] = c.lw(count_slot);
                c.g[V0][0] = update_slt(c.g[A1][0], c.g[V0][0]);
                let t = c.g[V0][0] != 0;
                set_word(c, V1, r32(c, V1).wrapping_add(0x1c));
                if t {
                    update_back_edge(c, 0x0023_7790)?;
                    pc = 0x0023_7790;
                    continue;
                }
                pc = 0x0023_77c0;
                continue;
            }
            // `InputSync_SendSample(self, localsample)`.
            0x0023_77c0 => {
                c.cycles += 6;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S0][0];
                UPDATE_SEND_SAMPLE.call(c, 0x0023_77cc)?;
                pc = 0x0023_77cc;
                continue;
            }
            // Resume after `SendSample`: copy the sample into the ring.
            0x0023_77cc => {
                let numsamples_slot = update_reg_off(c, S2, 0x8);
                c.g[V0][0] = c.lw(numsamples_slot);
                let t = (c.g[V0][0] as i64) <= 0;
                c.g[A3][0] = 0;
                pc = if t { 0x0023_78c8 } else { 0x0023_77d8 };
                continue;
            }
            0x0023_77d8 => {
                c.cycles += 4;
                c.g[T1][0] = 0;
                c.g[T0][0] = 0x64;
                let latency_slot = update_reg_off(c, S2, 0x4);
                c.g[A0][0] = c.lw(latency_slot);
                pc = 0x0023_77e8;
                continue;
            }
            0x0023_77e8 => {
                c.cycles += 4;
                c.g[A2][0] = c.g[S0][0];
                let writepos_slot = update_reg_off(c, S1, 0x8);
                c.g[V0][0] = c.lw(writepos_slot);
                pc = 0x0023_77f8;
                continue;
            }
            0x0023_77f8 => {
                c.cycles += 8;
                c.div(0, V0, A0);
                let samples_slot = update_reg_off(c, S1, 0);
                c.g[A1][0] = c.lw(samples_slot);
                c.g[V1][0] = c.hi[0];
                c.mult(0, V1, V1, T0);
                set_word(c, V1, r32(c, V1).wrapping_add(r32(c, A1)));
                c.g[V0][0] = c.g[V1][0] & 0x7;
                let t = c.g[V0][0] == 0;
                set_word(c, A0, r32(c, A3).wrapping_add(0x1));
                pc = if t { 0x0023_7870 } else { 0x0023_7818 };
                continue;
            }
            0x0023_7818 => {
                c.cycles += 1;
                set_word(c, V0, r32(c, SP).wrapping_add(0xf0));
                pc = 0x0023_781c;
                continue;
            }
            0x0023_781c => {
                c.cycles += 19;
                let src = r32(c, A2);
                let dst = r32(c, V1);
                update_copy_unaligned(c, src, dst);
                set_word(c, A2, src.wrapping_add(0x20));
                let t = c.g[A2][0] != c.g[V0][0];
                set_word(c, V1, dst.wrapping_add(0x20));
                if t {
                    update_back_edge(c, 0x0023_781c)?;
                    pc = 0x0023_781c;
                    continue;
                }
                pc = 0x0023_7868;
                continue;
            }
            0x0023_7868 => {
                c.cycles += 2;
                pc = 0x0023_78a0;
                continue;
            }
            0x0023_7870 => {
                c.cycles += 1;
                set_word(c, V0, r32(c, SP).wrapping_add(0xf0));
                pc = 0x0023_7874;
                continue;
            }
            0x0023_7874 => {
                c.cycles += 11;
                let src = r32(c, A2);
                let dst = r32(c, V1);
                update_copy_aligned(c, src, dst);
                set_word(c, A2, src.wrapping_add(0x20));
                let t = c.g[A2][0] != c.g[V0][0];
                set_word(c, V1, dst.wrapping_add(0x20));
                if t {
                    update_back_edge(c, 0x0023_7874)?;
                    pc = 0x0023_7874;
                    continue;
                }
                pc = 0x0023_78a0;
                continue;
            }
            0x0023_78a0 => {
                c.cycles += 10;
                let src = r32(c, A2);
                c.g[V0][0] = c.lw(src);
                let dst = r32(c, V1);
                let word = c.g[V0][0];
                c.sw(dst, word);
                let writepos_slot = update_reg_off(c, S1, 0x8);
                c.g[V0][0] = c.lw(writepos_slot);
                c.g[A3][0] = c.g[A0][0];
                set_word(c, V0, r32(c, V0).wrapping_add(0x1));
                let writepos_value = c.g[V0][0];
                c.sw(writepos_slot, writepos_value);
                let numsamples_slot = update_reg_off(c, S2, 0x8);
                c.g[V1][0] = c.lw(numsamples_slot);
                c.g[V1][0] = update_slt(c.g[A3][0], c.g[V1][0]);
                if c.g[V1][0] != 0 {
                    let latency_slot = update_reg_off(c, S2, 0x4);
                    c.g[A0][0] = c.lw(latency_slot);
                    update_back_edge(c, 0x0023_77e8)?;
                    pc = 0x0023_77e8;
                    continue;
                }
                pc = 0x0023_78c8;
                continue;
            }
            // `n = (int)(cGameComm_getLatency(GameComm()) / SSXApp->TimerPeriod)`,
            // `if (n <= 1) n = 2; sendlimit = n * numsamples`, capped at latency.
            0x0023_78c8 => {
                c.cycles += 17;
                c.g[A0][0] = c.lw(update_gp_slot(c, 0x204));
                UPDATE_INPUT_DELAY.call(c, 0x0023_78d0)?;
                pc = 0x0023_78d0;
                continue;
            }
            // Resume after `cGameComm_GetInputDelay`: the send limit.
            0x0023_78d0 => {
                c.g[V0][0] = c.lw(update_gp_slot(c, 0x848));
                c.g[V1][0] = 0x1;
                c.g[A0][0] = 0x2;
                let numsamples_slot = update_reg_off(c, S2, 0x8);
                c.g[A1][0] = c.lw(numsamples_slot);
                let period_slot = update_reg_off(c, V0, 0x14);
                c.lwc1(1, period_slot);
                let latency_slot = update_reg_off(c, S2, 0x4);
                c.g[A2][0] = c.lw(latency_slot);
                c.div_s(0, 0, 1);
                c.cvt_w_s(1, 0);
                c.mfc1(2, 1);
                c.g[V1][0] = update_slt(c.g[V1][0], c.g[V0][0]);
                if c.g[V1][0] == 0 {
                    c.g[V0][0] = c.g[A0][0];
                }
                c.mult(0, V0, V0, A1);
                c.g[V1][0] = update_slt(c.g[A2][0], c.g[V0][0]);
                let t = c.g[V1][0] == 0;
                let sendlimit_slot = r32(c, S2);
                let sendlimit = c.g[V0][0];
                c.sw(sendlimit_slot, sendlimit);
                pc = if t { 0x0023_7910 } else { 0x0023_790c };
                continue;
            }
            0x0023_790c => {
                c.cycles += 1;
                let sendlimit_slot = r32(c, S2);
                let latency = c.g[A2][0];
                c.sw(sendlimit_slot, latency);
                pc = 0x0023_7910;
                continue;
            }
            0x0023_7910 => {
                c.cycles += 1;
                let count_slot = update_reg_off(c, S2, 0x10);
                c.g[V0][0] = c.lw(count_slot);
                pc = 0x0023_7914;
                continue;
            }
            // Loop tail: `i = i + 1`, while `i < numsyncedpads` (back edge with event check).
            0x0023_7914 => {
                c.cycles += 4;
                c.g[S0][0] = c.g[S3][0];
                c.g[V0][0] = update_slt(c.g[S0][0], c.g[V0][0]);
                let t = c.g[V0][0] != 0;
                c.g[V0][0] = 0x18;
                if t {
                    update_back_edge(c, 0x0023_7400)?;
                    pc = 0x0023_7400;
                    continue;
                }
                pc = UPDATE_EPILOGUE;
                continue;
            }
            // Epilogue: restore s0..s4 and `ra`, pop the frame, return.
            0x0023_7924 => {
                c.cycles += 8;
                let frame = Frame { size: UPDATE_FRAME };
                frame.restore(c, S0, 0x150);
                frame.restore(c, S1, 0x140);
                frame.restore(c, S2, 0x130);
                frame.restore(c, S3, 0x120);
                frame.restore(c, S4, 0x110);
                frame.restore_ra(c, 0x100);
                frame.close(c);
                return Ok(());
            }
            _ => {
                c.jump_out(pc, UPDATE_ENTRY, UPDATE_END);
                return Ok(());
            }
        }
    }
}

/// `cInputSync_SyncedPad* InputSync_AddRemotePad(InputSync* self (a0), void*
/// realinputmap (a1))` (0x237280; reference `cInputSync_createremoteinput`).
///
/// Appends a pad record to `self->syncedpads` (the `numsyncedpads`-th, 0x18
/// bytes each, from +0x14). Its sample buffer is `latency * 100` bytes with
/// every sample's `NumButtons` (+0 of each 0x64-byte sample) set to 0x18;
/// the record gets `readpos = 0`, `writepos = sendlimit`, `sourcepad = 0`,
/// a 676-byte destination pad (its first word zeroed) and an 8-byte input
/// object, linked to the pad with `ValuePair_SetFirst/Second`. Returns the
/// input object.
pub fn input_sync_add_remote_pad(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = 0x18;
        frame.save(c, S0, 0x40);
        c.g[A2][0] = 0;
        frame.save(c, S1, 0x30);
        c.g[S0][0] = 0x64;
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[A3][0] = 0;
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A1][0];
        c.g[A1][0] = 0x47_c078; // InputSamples tag
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x10).lw(c); // numsyncedpads
        c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0x4).lw(c); // latency
        c.mult(0, V1, V0, V1); // v1 = numsyncedpads * 0x18
        c.mult(0, 0, A0, S0); // lo = latency * 0x64
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        let v0 = c.g[V0][0];
        Addr(c.g[S1][0] as u32).off(0x10).sw(c, v0); // numsyncedpads++
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x14));
        c.g[A0][0] = c.lo[0]; // latency * 100
        c.g[RA][0] = sx(0x002372e0);
        c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
        call_ptr(c, 0x00317e30, 0x002372e0)?; // operator_new_named
        let v0 = c.g[V0][0];
        Addr(c.g[S2][0] as u32).sw(c, v0); // syncedPad->samples
        c.g[A0][0] = c.g[V0][0];
        c.g[A2][0] = Addr(c.g[S1][0] as u32).off(0x4).lw(c);
        c.mult(0, 0, A2, S0); // lo = latency * 0x64
        c.g[A2][0] = c.lo[0];
        c.g[RA][0] = sx(0x002372fc);
        c.g[A1][0] = 0;
        call_ptr(c, 0x003e6448, 0x002372fc)?; // EA_MemSet (sample buffer)
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x4).lw(c);
        let skip = (c.g[V0][0] as i64) <= 0;
        c.g[V1][0] = 0;
        if !skip {
            c.cycles += 2;
            c.g[A1][0] = 0x18;
            c.g[A0][0] = 0;
            loop {
                c.cycles += 8;
                c.g[V0][0] = Addr(c.g[S2][0] as u32).lw(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                let sample = c.g[V0][0];
                let buttons = c.g[A1][0];
                Addr(sample as u32).sw(c, buttons); // NumButtons = 0x18
                c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x4).lw(c);
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x64));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x00237310);
                }
            }
        }
        c.cycles += 34;
        Addr(c.g[S2][0] as u32).off(0x4).sw(c, 0); // readpos
        c.g[A1][0] = 0x47_c088; // SyncedPad tag
        c.g[A0][0] = 0x2a4; // sizeof(SyncedPad) = 676
        c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c); // sendlimit
        c.g[A2][0] = 0;
        Addr(c.g[S2][0] as u32).off(0xc).sw(c, 0); // sourcepad
        c.g[A3][0] = 0;
        c.g[RA][0] = sx(0x00237358);
        let v0 = c.g[V0][0];
        Addr(c.g[S2][0] as u32).off(0x8).sw(c, v0); // writepos = sendlimit
        call_ptr(c, 0x00317d70, 0x00237358)?; // EA_MemAllocNamed
        let v0 = c.g[V0][0];
        Addr(c.g[S2][0] as u32).off(0x10).sw(c, v0); // destinationpad
        c.g[A1][0] = 0x47_c098; // SyncedInput tag
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A0][0] = 0x8;
        c.g[RA][0] = sx(0x00237378);
        Addr(c.g[V0][0] as u32).sw(c, 0); // *destinationpad = 0
        call_ptr(c, 0x00317d70, 0x00237378)?; // EA_MemAllocNamed
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = Addr(c.g[S2][0] as u32).off(0x10).lw(c);
        c.g[RA][0] = sx(0x00237388);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00244400, 0x00237388)?; // ValuePair_SetFirst
        c.g[A1][0] = c.g[S3][0];
        c.g[RA][0] = sx(0x00237394);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x00244408, 0x00237394)?; // ValuePair_SetSecond
        let s0 = c.g[S0][0];
        Addr(c.g[S2][0] as u32).off(0x14).sw(c, s0); // input
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void InputSync_EmptyHook(void)` (0x237948): a hook that does nothing.
pub fn input_sync_empty_hook(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void InputSync_AddLocalPad(InputSync* self, const void* src)` (0x237140):
/// appends a local pad record to the table at `+0x14` (0x18-byte records, the
/// count at `+0x10`). The record holds a zeroed buffer of `count * 0x64`
/// bytes (first word of each 0x64 block set to 0x18), the sync's word 0 (`+8`),
/// `*src` (`+0xc`), a 0x2a4-byte block (`+0x10`, its first word cleared) and
/// an 8-byte `ValuePair` (`+0x14`) made from `+0x10` and `src[1]`. Frame 0x50.
pub fn input_sync_add_local_pad(c: &mut Cpu) {
    const ALLOC_BUFFER: u32 = 0x0031_7e30;
    const ALLOC_BLOCK: u32 = 0x0031_7d70;
    const MEM_FILL: u32 = 0x003e_6448;
    const VALUE_PAIR_SET_FIRST: u32 = 0x0024_4400;
    const VALUE_PAIR_SET_SECOND: u32 = 0x0024_4408;
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = 0x18;
        frame.save(c, S0, 0x40);
        c.g[A2][0] = 0;
        frame.save(c, S1, 0x30);
        c.g[S0][0] = 0x64;
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[A3][0] = 0;
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A1][0];
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x3f88));
        c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(0x10));
        c.g[A0][0] = c.lw(r32(c, S1).wrapping_add(4));
        c.mult(0, 3, 2, 3);
        c.mult(0, 0, 4, 16);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
        c.sw(r32(c, S1).wrapping_add(0x10), c.g[V0][0]);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x14));
        c.g[A0][0] = c.lo[0];
        c.g[S2][0] = sx(r32(c, S1).wrapping_add(r32(c, V1)));
        call_ptr(c, ALLOC_BUFFER, 0x0023_71a0)?;
        c.sw(r32(c, S2), c.g[V0][0]);
        c.g[A0][0] = c.g[V0][0];
        let count = c.lw(r32(c, S1).wrapping_add(4));
        c.g[A2][0] = count;
        c.mult(0, 0, 6, 16);
        c.g[A2][0] = c.lo[0];
        c.g[A1][0] = 0;
        call_ptr(c, MEM_FILL, 0x0023_71bc)?;
        c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(4));
        let no_blocks = (c.g[V0][0] as i64) <= 0;
        c.g[V1][0] = 0;
        if !no_blocks {
            c.cycles += 2;
            c.g[A1][0] = 0x18;
            c.g[A0][0] = 0;
            loop {
                c.cycles += 8;
                c.g[V0][0] = c.lw(r32(c, S2));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
                c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
                c.sw(r32(c, V0), c.g[A1][0]);
                c.g[V0][0] = c.lw(r32(c, S1).wrapping_add(4));
                c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let again = c.g[V0][0] != 0;
                c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x64));
                if !again {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0023_71d0);
                }
            }
        }
        c.cycles += 36;
        c.sw(r32(c, S2).wrapping_add(4), 0);
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x3f78));
        c.g[A0][0] = 0x2a4;
        c.g[V1][0] = c.lw(r32(c, S1));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.sw(r32(c, S2).wrapping_add(8), c.g[V1][0]);
        c.g[V0][0] = c.lw(r32(c, S3));
        c.sw(r32(c, S2).wrapping_add(0xc), c.g[V0][0]);
        call_ptr(c, ALLOC_BLOCK, 0x0023_721c)?;
        c.sw(r32(c, S2).wrapping_add(0x10), c.g[V0][0]);
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_sub(0x3f68));
        c.g[A2][0] = 0;
        c.g[A3][0] = 0;
        c.g[A0][0] = 8;
        c.sw(r32(c, V0), 0);
        call_ptr(c, ALLOC_BLOCK, 0x0023_723c)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.lw(r32(c, S2).wrapping_add(0x10));
        c.g[S1][0] = c.lw(r32(c, S3).wrapping_add(4));
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, VALUE_PAIR_SET_FIRST, 0x0023_7250)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, VALUE_PAIR_SET_SECOND, 0x0023_725c)?;
        c.sw(r32(c, S2).wrapping_add(0x14), c.g[S0][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void GameStateReset_Enter(GameStateReset* self)` (0x2355c0): enters the
/// "reset" game state: records whether the script engine is in a transport
/// sequence (`+0x1c`), clears the user awards, reads the race setup
/// (`0x535bc8`: byte `+0x49` game path, byte `+0x48` game mode), picks the
/// next state (`+0x0`, `+0x4`) from the course and race mode, switches the AI
/// state in mode 4, resets the BE state, restarts the missions and the global
/// scripts, enters the hub when the location type is 1, starts the NIS lists
/// (path set) or the idle list (`+0x14`), and re-enables the audio engine.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut,
    clippy::all
)]
pub fn game_state_reset_enter_3(c: &mut Cpu) {
    guest(c, |c| {
        'b_2355fc: {
            c.cycles += 14;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S1][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), 0u64);
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
            call_ptr(c, 0x0027aa80, 0x002355f0)?; // call_indirect
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = 0x1;
            if t {
                break 'b_2355fc;
            } else {
                c.cycles += 1;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
                break 'b_2355fc;
            }
        }
        'b_235684: {
            'b_235674: {
                'b_235670: {
                    c.cycles += 17;
                    call_ptr(c, 0x0014dc80, 0x00235604)?; // call_indirect
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x00235610)?; // call_indirect
                    c.g[S2][0] = c.g[V0][0];
                    call_ptr(c, 0x0014dc80, 0x00235618)?; // call_indirect
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0xd;
                    call_ptr(c, 0x0014dd58, 0x00235624)?; // call_indirect
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x00158e30, 0x0023562c)?; // call_indirect
                    c.g[V0][0] = 0x530000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                    c.g[S3][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
                    let t = c.g[S3][0] != 0u64;
                    c.g[S4][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
                    if t {
                        break 'b_235670;
                    } else {
                        c.cycles += 8;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x78));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1bc));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe));
                        c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[V0][0] = 0x1;
                        if t {
                            break 'b_235674;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = 0x4;
                            c.sw((c.g[S1][0] as u32).wrapping_add(0x4), 0u64);
                            let t = true;
                            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                            if t {
                                break 'b_235684;
                            } else {
                                break 'b_235670;
                            }
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = 0x1;
                break 'b_235674;
            }
            c.cycles += 4;
            c.g[V1][0] = (c.g[S3][0] < 0x1u64) as u64;
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
            c.g[V0][0] = 0x4;
            break 'b_235684;
        }
        'b_2356a0: {
            c.cycles += 2;
            if c.g[S4][0] != c.g[V0][0] {
                c.g[A0][0] = 0u64;
                break 'b_2356a0;
            } else {
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                c.g[A1][0] = 0x3;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                call_ptr(c, 0x00113b10, 0x0023569c)?; // call_indirect
                c.g[A0][0] = 0u64;
                break 'b_2356a0;
            }
        }
        'b_23571c: {
            c.cycles += 13;
            c.g[S5][0] = 0x1;
            call_ptr(c, 0x0014de68, 0x002356a8)?; // call_indirect
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
            call_ptr(c, 0x00309030, 0x002356b0)?; // call_indirect
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
            call_ptr(c, 0x00309f18, 0x002356b8)?; // call_indirect
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x00144be0, 0x002356c0)?; // call_indirect
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x00144cc0, 0x002356cc)?; // call_indirect
            let t = c.g[V0][0] != c.g[S5][0];
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            if t {
                break 'b_23571c;
            } else {
                c.cycles += 18;
                call_ptr(c, 0x0014dc80, 0x002356dc)?; // call_indirect
                c.g[A1][0] = 0x1;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0014dd58, 0x002356e8)?; // call_indirect
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x00144be0, 0x002356f4)?; // call_indirect
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x00146e10, 0x00235704)?; // call_indirect
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00235718)?; // call_indirect
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                break 'b_23571c;
            }
        }
        'b_2357b0: {
            'b_235798: {
                c.cycles += 11;
                c.g[V1][0] = c.g[S4][0] ^ 0x4;
                c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[V1][0] = (0u64 < c.g[V1][0]) as u64;
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                c.g[A1][0] = 0u64;
                c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x84));
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x94));
                call_ptr(c, 0x001f36d8, 0x00235740)?; // call_indirect
                if c.g[S3][0] == 0u64 {
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x14));
                    break 'b_235798;
                } else {
                    c.cycles += 7;
                    c.sw(c.g[S1][0] as u32, c.g[S5][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x4), 0u64);
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x550));
                    c.g[V0][0] = c.g[V0][0] ^ 0x2;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A1][0] = 0x1;
                    if t {
                        break 'b_2357b0;
                    } else {
                        c.cycles += 8;
                        c.g[A2][0] = 0u64;
                        c.g[A3][0] = 0x1;
                        call_ptr(c, 0x00278f68, 0x00235770)?; // call_indirect
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x550));
                        c.g[V0][0] = c.g[V0][0] ^ 0x2;
                        let t = c.g[V0][0] != 0u64;
                        c.g[A1][0] = 0u64;
                        if t {
                            break 'b_2357b0;
                        } else {
                            c.cycles += 5;
                            c.g[A2][0] = 0u64;
                            c.g[A3][0] = 0x1;
                            call_ptr(c, 0x00278f68, 0x00235790)?; // call_indirect
                            let t = true;

                            if t {
                                break 'b_2357b0;
                            } else {
                                break 'b_235798;
                            }
                        }
                    }
                }
            }
            c.cycles += 2;
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
            if t {
                break 'b_2357b0;
            } else {
                c.cycles += 4;
                c.g[A1][0] = 0x1;
                c.g[A2][0] = 0x1;
                c.g[A3][0] = 0u64;
                call_ptr(c, 0x00278f68, 0x002357b0)?; // call_indirect
                break 'b_2357b0;
            }
        }
        c.cycles += 17;
        call_ptr(c, 0x0028b180, 0x002357b8)?; // call_indirect
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x118));
        call_ptr(c, 0x002b3a98, 0x002357c0)?; // call_indirect
        call_ptr(c, 0x0028b180, 0x002357c8)?; // call_indirect
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x002a4b68, 0x002357d0)?; // call_indirect
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        Ok(())
    })
}

/// `void GameStateRestartRace_Enter(GameState* self)` (0x235aa0): resets the game
/// mode and the race, then either (mode 5/6 or a flagged race) sets state 3, or
/// re-adds the computer riders and restarts and sets state 0.
pub fn oc681_game_state_restart_race_enter(c: &mut Cpu) {
    guest(c, |c| {
        'b_235c98: {
            'b_235b64: {
                'b_235b5c: {
                    'b_235b58: {
                        c.cycles += 40;
                        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
                        c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[S7][0] = c.g[A0][0];
                        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
                        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
                        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
                        c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
                        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
                        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
                        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
                        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
                        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                        c.g[RA][0] = sx(0x00235ad8);
                        c.g[A0][0] = 0u64;
                        call_ptr(c, 0x0014de68, 0x00235ad8)?;
                        c.g[RA][0] = sx(0x00235ae0);
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
                        call_ptr(c, 0x0030b7f8, 0x00235ae0)?;
                        c.g[RA][0] = sx(0x00235ae8);
                        call_ptr(c, 0x0014dc80, 0x00235ae8)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x00235af4);
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x0014dd58, 0x00235af4)?;
                        c.g[RA][0] = sx(0x00235afc);
                        c.g[S0][0] = c.g[V0][0];
                        call_ptr(c, 0x0014dc80, 0x00235afc)?;
                        c.g[A1][0] = 0x1;
                        c.g[RA][0] = sx(0x00235b08);
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x0014dd58, 0x00235b08)?;
                        c.g[S5][0] = c.g[V0][0];
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[S6][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
                        c.g[RA][0] = sx(0x00235b1c);
                        c.g[A0][0] = c.g[S6][0];
                        Callee {
                            addr: 0x002382d8,
                            f: ee::s06::GameModeMan_ResetMode,
                        }
                        .call(c, 0x00235b1c)?;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[RA][0] = sx(0x00235b28);
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                        Callee {
                            addr: 0x00230180,
                            f: ee::s06::GameModule_ResetRace,
                        }
                        .call(c, 0x00235b28)?;
                        c.g[V0][0] = 0x530000;
                        c.g[V1][0] = 0x5;
                        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                        c.g[A0][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0x48));
                        let t = c.g[A0][0] == c.g[V1][0];
                        c.g[FP][0] = c.g[V0][0];
                        if t {
                            break 'b_235b58;
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 0x6;
                            let t = c.g[A0][0] == c.g[V0][0];
                            c.g[V0][0] = 0x3;
                            if t {
                                break 'b_235b5c;
                            } else {
                                c.cycles += 3;
                                c.g[V0][0] = c.lb((c.g[A1][0] as u32).wrapping_add(0x49));
                                let t = c.g[V0][0] == 0u64;
                                c.g[A2][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
                                if t {
                                    break 'b_235b64;
                                } else {
                                    break 'b_235b58;
                                }
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 0x3;
                    break 'b_235b5c;
                }
                c.cycles += 2;
                let t = true;
                c.sw((c.g[S7][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                if t {
                    break 'b_235c98;
                } else {
                    break 'b_235b64;
                }
            }
            'b_235be4: {
                c.cycles += 16;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x10));
                c.g[S2][0] = 0u64;
                c.g[V0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x14));
                c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0xc));
                c.g[S3][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[S4][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                c.g[RA][0] = sx(0x00235b88);
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, 0x00144d98, 0x00235b88)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x00235b9c);
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00235b9c)?;
                let t = (c.g[S3][0] as i64) <= 0;
                c.g[V1][0] = sx((c.g[S4][0] as u32) << 2);
                if t {
                    break 'b_235be4;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x40));
                    c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S4][0] as u32));
                    'l_235bb0: loop {
                        c.cycles += 13;
                        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_sub(0x28));
                        c.g[A0][0] = c.g[S5][0];
                        c.g[RA][0] = sx(0x00235bc0);
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, 0x00147338, 0x00235bc0)?;
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        c.g[A2][0] = c.lw(c.g[S1][0] as u32);
                        c.g[A1][0] = c.g[S0][0];
                        c.g[A0][0] = c.g[S5][0];
                        c.g[RA][0] = sx(0x00235bd8);
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
                        call_ptr(c, 0x001473d0, 0x00235bd8)?;
                        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[S3][0] as i64)) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S4][0] as u32));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x00235bb0);
                            }
                            continue 'l_235bb0;
                        } else {
                            break 'b_235be4;
                        }
                    }
                }
            }
            'b_235c28: {
                c.cycles += 12;
                c.g[V0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0xc));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                let t = c.g[V1][0] as u32;
                c.g[RA][0] = sx(0x00235bf8);
                c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x00235bf8)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[RA][0] = sx(0x00235c08);
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                call_ptr(c, 0x0012ab20, 0x00235c08)?;
                c.g[V0][0] = ((c.g[S3][0] as i64) < 2i64) as u64;
                if c.g[V0][0] != 0u64 {
                    c.sw((c.g[S7][0] as u32).wrapping_add(0x18), 0u64);
                    break 'b_235c28;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                    c.g[RA][0] = sx(0x00235c24);
                    c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                    call_ptr(c, 0x0012ac48, 0x00235c24)?;
                    c.sw((c.g[S7][0] as u32).wrapping_add(0x18), 0u64);
                    break 'b_235c28;
                }
            }
            'b_235c50: {
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x5bc8));
                c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
                let t = c.g[V1][0] != 0u64;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                if t {
                    break 'b_235c50;
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = 0x1;
                    c.sw((c.g[S7][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    c.g[A1][0] = 0x1;
                    c.g[RA][0] = sx(0x00235c4c);
                    c.g[A2][0] = 0u64;
                    call_ptr(c, 0x0027a860, 0x00235c4c)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    break 'b_235c50;
                }
            }
            c.cycles += 18;
            c.g[RA][0] = sx(0x00235c58);
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x002790a0, 0x00235c58)?;
            c.g[V1][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x98));
            c.g[V0][0] = 0x2;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
            c.g[A1][0] = 0x3;
            c.g[RA][0] = sx(0x00235c70);
            if c.g[V1][0] == 0 {
                c.g[A1][0] = c.g[V0][0];
            }
            call_ptr(c, 0x0027aaf8, 0x00235c70)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
            c.g[RA][0] = sx(0x00235c7c);
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x00278f38, 0x00235c7c)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A1][0] = 0x3;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
            c.g[RA][0] = sx(0x00235c90);
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x94));
            call_ptr(c, 0x001f36c0, 0x00235c90)?;
            c.sw((c.g[S7][0] as u32).wrapping_add(0x10), 0u64);
            c.sw((c.g[S7][0] as u32).wrapping_add(0x14), 0u64);
            break 'b_235c98;
        }
        c.cycles += 12;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        return Ok(());
    });
}

/// `void GameStateWarpToEvent_Enter(GameState* self)` (0x236058): fails the active
/// mission, starts game mode 0xc, resets the race, puts the rider at the start
/// point and starts a white fade transition.
pub fn oc681_game_state_warp_to_event_enter(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 79;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x10), c.g[RA][0]);
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
        c.g[RA][0] = sx(0x0023607c);
        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
        call_ptr(c, 0x0030b7f8, 0x0023607c)?;
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 20);
        c.g[A0][0] = c.g[S0][0];
        c.g[RA][0] = sx(0x00236090);
        c.g[A1][0] = 0xc;
        Callee {
            addr: 0x00238160,
            f: ee::s06::GameModeMan_StartMode,
        }
        .call(c, 0x00236090)?;
        c.g[RA][0] = sx(0x00236098);
        call_ptr(c, 0x0014dc80, 0x00236098)?;
        c.g[A1][0] = 0xa;
        c.g[RA][0] = sx(0x002360a4);
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014dd58, 0x002360a4)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[S0][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x002360b4);
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
        Callee {
            addr: 0x00230180,
            f: ee::s06::GameModule_ResetRace,
        }
        .call(c, 0x002360b4)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A2][0] = 0x2;
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = sx(0x002360d4);
        c.g[A0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0011de60, 0x002360d4)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A1][0] = 0x1;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x40));
        c.g[RA][0] = sx(0x002360f0);
        c.g[A0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0011df18, 0x002360f0)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A1][0] = 0u64;
        c.g[RA][0] = sx(0x00236100);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
        Callee {
            addr: 0x00230698,
            f: ee::s06::GameModule_GetTransitionMan,
        }
        .call(c, 0x00236100)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[RA][0] = sx(0x0023610c);
        c.g[A0][0] = 0x14;
        call_ptr(c, 0x002e4ce8, 0x0023610c)?;
        c.g[S0][0] = c.g[V0][0];
        c.swc1(20, c.g[SP][0] as u32);
        c.g[A0][0] = c.g[S0][0];
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x4));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.g[RA][0] = sx(0x00236128);
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0xc));
        call_ptr(c, 0x002e4d70, 0x00236128)?;
        c.g[V1][0] = 0x490000;
        c.mtc1(0, 12);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x7f40));
        c.g[A0][0] = c.g[S1][0];
        c.sw(c.g[S0][0] as u32, c.g[V1][0]);
        c.g[A3][0] = c.g[S0][0];
        c.mov_s(14, 20);
        c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x7));
        c.ldr(2, c.g[SP][0] as u32);
        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0xf));
        c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x8));
        c.sdl(2, (c.g[S0][0] as u32).wrapping_add(0xb));
        c.sdr(2, (c.g[S0][0] as u32).wrapping_add(0x4));
        c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x13));
        c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = 0x1;
        c.g[A2][0] = 0u64;
        c.mov_s(13, 12);
        c.g[T0][0] = 0u64;
        c.g[RA][0] = sx(0x0023617c);
        c.g[T1][0] = 0u64;
        call_ptr(c, 0x002e4370, 0x0023617c)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x10));
        c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x40));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `void GameStateEventSelect_Enter(GameStateEventSelect *self, int mode)` (0x236250, `cGFUserSelectState::gainFocus`): enter the event-select state. Fails the active local mission, sets the AI state to 3 and clears the first rider's boost bar, resets the state fields (`+0x18` next location = 0x17, `+0x14` mode, `+0x8`/`+0x10`/`+0x1c` flags; mode 1 marks the overlay done and takes the next location from the new-race interface), restarts the race when the next location equals the current one, then queues the NIS list for the mode (0: two categories 22/23; 1..3: category 11 unless mode 1, ready, `cRider_initAlways`, play).
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn game_state_event_select_enter(c: &mut Cpu) {
    guest(c, |c| {
        'b_2362ec: {
            c.cycles += 32;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S0][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xce8));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[RA][0] = sx(0x00236274);
            c.g[S1][0] = c.g[A1][0];
            call_ptr(c, 0x0030b7f8, 0x00236274)?;
            c.g[RA][0] = sx(0x0023627c);
            call_ptr(c, 0x0014dc80, 0x0023627c)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00236288);
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014dd58, 0x00236288)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[A1][0] = 0x3;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            c.g[RA][0] = sx(0x0023629c);
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
            call_ptr(c, 0x00113b10, 0x0023629c)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x488));
            c.g[A1][0] = 0x17;
            c.g[A2][0] = 0x1;
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x40));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
            c.sw((c.g[A0][0] as u32).wrapping_add(0x2f8), 0u64);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[A1][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x10), 0u64);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S1][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
            let t = c.g[S1][0] != c.g[A2][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), 0u64);
            if t {
                break 'b_2362ec;
            } else {
                c.cycles += 7;
                c.g[RA][0] = sx(0x002362d8);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[S1][0]);
                call_ptr(c, 0x0014dc80, 0x002362d8)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x002362e4);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x002362e4)?;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x1f34));
                c.sw((c.g[S0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                break 'b_2362ec;
            }
        }
        'b_236314: {
            c.cycles += 6;
            c.g[RA][0] = sx(0x002362f4);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x00144bc0, 0x002362f4)?;
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
            if c.g[V0][0] != c.g[V1][0] {
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                break 'b_236314;
            } else {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[RA][0] = sx(0x00236310);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                Callee {
                    addr: 0x00230180,
                    f: ee::s06::GameModule_ResetRace,
                }
                .call(c, 0x00236310)?;
                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                break 'b_236314;
            }
        }
        'b_236404: {
            'b_236388: {
                c.cycles += 2;
                let t = c.g[V1][0] != 0u64;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                if t {
                    break 'b_236388;
                } else {
                    c.cycles += 27;
                    c.g[A1][0] = 0x1;
                    c.g[A2][0] = 0x16;
                    c.g[A3][0] = 0x3;
                    c.g[T0][0] = 0u64;
                    c.g[T1][0] = 0xffffffffffffffff;
                    c.g[T2][0] = 0xffffffffffffffff;
                    c.g[RA][0] = sx(0x0023633c);
                    c.g[T3][0] = 0xffffffffffffffff;
                    call_ptr(c, 0x00278e50, 0x0023633c)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    c.g[A2][0] = 0x17;
                    c.g[A3][0] = 0x1;
                    c.g[A1][0] = 0x1;
                    c.g[T0][0] = 0u64;
                    c.g[T1][0] = 0xffffffffffffffff;
                    c.g[T2][0] = 0xffffffffffffffff;
                    c.g[RA][0] = sx(0x00236360);
                    c.g[T3][0] = 0xffffffffffffffff;
                    call_ptr(c, 0x00278e50, 0x00236360)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    c.g[RA][0] = sx(0x0023636c);
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x00278f38, 0x0023636c)?;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                    c.g[A1][0] = 0x1;
                    c.g[A2][0] = 0x1;
                    c.g[RA][0] = sx(0x00236380);
                    c.g[A3][0] = 0u64;
                    call_ptr(c, 0x00278f68, 0x00236380)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                    if t {
                        break 'b_236404;
                    } else {
                        break 'b_236388;
                    }
                }
            }
            'b_236400: {
                c.cycles += 2;
                let t = (c.g[V1][0] as i64) < 0;
                c.g[V0][0] = ((c.g[V1][0] as i64) < 4i64) as u64;
                if t {
                    break 'b_236400;
                } else {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = c.g[V1][0] ^ 0x1;
                    if t {
                        break 'b_236400;
                    } else {
                        'b_2363c0: {
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0u64;
                            c.g[A1][0] = 0x1;
                            if t {
                                break 'b_2363c0;
                            } else {
                                c.cycles += 8;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                                c.g[A2][0] = 0xb;
                                c.g[A3][0] = 0x3;
                                c.g[T0][0] = 0u64;
                                c.g[T1][0] = 0xffffffffffffffff;
                                c.g[T2][0] = 0xffffffffffffffff;
                                c.g[RA][0] = sx(0x002363c0);
                                c.g[T3][0] = 0xffffffffffffffff;
                                call_ptr(c, 0x00278e50, 0x002363c0)?;
                                break 'b_2363c0;
                            }
                        }
                        c.cycles += 16;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                        c.g[RA][0] = sx(0x002363cc);
                        c.g[A1][0] = 0x1;
                        call_ptr(c, 0x00278f38, 0x002363cc)?;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x002363e4);
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                        call_ptr(c, 0x0011d390, 0x002363e4)?;
                        c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                        c.g[A1][0] = 0x1;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                        c.g[A3][0] = 0u64;
                        c.g[A2][0] = c.g[A2][0] ^ 0x2;
                        c.g[RA][0] = sx(0x00236400);
                        c.g[A2][0] = (c.g[A2][0] < 0x1u64) as u64;
                        call_ptr(c, 0x00278f68, 0x00236400)?;
                        break 'b_236400;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_236404;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `void GameStateEventSelect_Update(GameStateEventSelect *self)` (0x236418, `cGFUserSelectState::update`): the event-select state machine over `self->+0x10`. State 0 waits for the NIS list, triggers the overlay (0x1f, or 33 with the map style) and pushes the pause state; state 1 waits for the overlay, then moves on to state 2 (mode 0) or 3 (location change / request the module state); state 2 handles the NIS transitions and the location change; state 3 goes to the next game state.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn game_state_event_select_update(c: &mut Cpu) {
    guest(c, |c| {
        'b_236714: {
            'b_236710: {
                'b_23670c: {
                    'b_236658: {
                        'b_236540: {
                            c.cycles += 10;
                            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
                            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
                            c.g[S0][0] = c.g[A0][0];
                            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
                            c.g[S2][0] = 0x1;
                            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                            let t = c.g[V1][0] == c.g[S2][0];
                            c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
                            if t {
                                break 'b_236540;
                            } else {
                                'b_236468: {
                                    'b_236458: {
                                        c.cycles += 2;
                                        if c.g[V0][0] == 0u64 {
                                            c.g[V0][0] = 0x2;
                                            break 'b_236458;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] == 0u64;
                                            c.g[A0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                                            if t {
                                                break 'b_236468;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                                if t {
                                                    break 'b_236714;
                                                } else {
                                                    break 'b_236458;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    if c.g[V1][0] == c.g[V0][0] {
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                        break 'b_236658;
                                    } else {
                                        c.cycles += 2;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                        if t {
                                            break 'b_236714;
                                        } else {
                                            break 'b_236468;
                                        }
                                    }
                                }
                                'b_236490: {
                                    c.cycles += 7;
                                    c.g[RA][0] = sx(0x00236470);
                                    c.g[A1][0] = 0x1;
                                    call_ptr(c, 0x00279220, 0x00236470)?;
                                    c.g[V1][0] = c.g[V0][0];
                                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x4));
                                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                                    if c.g[V0][0] != 0u64 {
                                        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                        break 'b_236490;
                                    } else {
                                        c.cycles += 2;
                                        if c.g[V1][0] != 0u64 {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                            break 'b_236714;
                                        } else {
                                            c.cycles += 1;
                                            c.g[V1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                            break 'b_236490;
                                        }
                                    }
                                }
                                'b_2364fc: {
                                    'b_2364a0: {
                                        c.cycles += 2;
                                        if c.g[V1][0] != 0u64 {
                                            c.g[V0][0] = 0x2;
                                            break 'b_2364a0;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[A0][0] = 0x1f;
                                            if t {
                                                break 'b_2364fc;
                                            } else {
                                                break 'b_2364a0;
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = c.g[V1][0] != c.g[V0][0];
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_23670c;
                                    } else {
                                        'b_2364e8: {
                                            c.cycles += 12;
                                            c.g[RA][0] = sx(0x002364b0);
                                            c.g[S1][0] = 0u64;
                                            call_ptr(c, 0x0014dc80, 0x002364b0)?;
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[RA][0] = sx(0x002364bc);
                                            c.g[A1][0] = 0u64;
                                            call_ptr(c, 0x0014dd58, 0x002364bc)?;
                                            c.g[RA][0] = sx(0x002364c4);
                                            c.g[A0][0] = c.g[V0][0];
                                            call_ptr(c, 0x00144bc0, 0x002364c4)?;
                                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                            c.g[V1][0] = ((c.g[V1][0] as i64) < 17i64) as u64;
                                            c.g[V1][0] = c.g[V1][0] ^ 0x1;
                                            let t = c.g[V1][0] == 0u64;
                                            c.g[V1][0] = 0x530000;
                                            if t {
                                                break 'b_2364e8;
                                            } else {
                                                c.cycles += 4;
                                                c.g[S1][0] = c.g[S2][0];
                                                c.g[V0][0] =
                                                    c.lb((c.g[V1][0] as u32).wrapping_add(0x5c10));
                                                c.g[V0][0] = c.g[V0][0] ^ 0x4;
                                                if c.g[V0][0] != 0 {
                                                    c.g[S1][0] = 0u64;
                                                }
                                                break 'b_2364e8;
                                            }
                                        }
                                        'b_2364f4: {
                                            c.cycles += 2;
                                            let t = c.g[S1][0] == 0u64;
                                            c.g[V0][0] = 0x4;
                                            if t {
                                                break 'b_2364f4;
                                            } else {
                                                c.cycles += 1;
                                                c.g[V0][0] = 0x3;
                                                break 'b_2364f4;
                                            }
                                        }
                                        c.cycles += 2;
                                        c.sw((c.g[GP][0] as u32).wrapping_sub(0xb5c), c.g[V0][0]);
                                        c.g[A0][0] = 0x21;
                                        break 'b_2364fc;
                                    }
                                }
                                c.cycles += 17;
                                c.g[RA][0] = sx(0x00236504);
                                call_ptr(c, 0x0020ab50, 0x00236504)?;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
                                c.g[A1][0] = 0x440000;
                                c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x28f8));
                                c.g[V1][0] = 0x530000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1));
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x66e8));
                                c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
                                c.sw((c.g[GP][0] as u32).wrapping_sub(0x6a0), c.g[A2][0]);
                                c.g[A1][0] = 0x2;
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                                c.sw(c.g[V0][0] as u32, c.g[A1][0]);
                                c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), c.g[A0][0]);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
                                let t = true;
                                c.g[V0][0] = 0x1;
                                if t {
                                    break 'b_23670c;
                                } else {
                                    break 'b_236540;
                                }
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        if c.g[V0][0] == 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            break 'b_236714;
                        } else {
                            'b_23659c: {
                                c.cycles += 7;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                                c.g[RA][0] = sx(0x00236558);
                                c.g[A1][0] = 0x1e;
                                call_ptr(c, 0x00278de8, 0x00236558)?;
                                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                c.g[V0][0] = c.g[V0][0] ^ 0x1;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
                                if t {
                                    break 'b_23659c;
                                } else {
                                    c.cycles += 13;
                                    c.g[A0][0] = 0x530000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x66e8));
                                    c.g[A1][0] = 0x440000;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x28f0));
                                    c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), c.g[V0][0]);
                                    c.g[V1][0] =
                                        sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                                    c.g[V0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                    c.sw((c.g[GP][0] as u32).wrapping_sub(0x6a0), c.g[V1][0]);
                                    break 'b_23659c;
                                }
                            }
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                            let t = c.g[V0][0] == 0u64;
                            c.g[V0][0] = 0x2;
                            if t {
                                break 'b_23670c;
                            } else {
                                'b_236638: {
                                    'b_23662c: {
                                        'b_236624: {
                                            c.cycles += 14;
                                            c.g[RA][0] = sx(0x002365b0);
                                            call_ptr(c, 0x0014dc80, 0x002365b0)?;
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[RA][0] = sx(0x002365bc);
                                            c.g[A1][0] = 0u64;
                                            call_ptr(c, 0x0014dd58, 0x002365bc)?;
                                            c.g[RA][0] = sx(0x002365c4);
                                            c.g[A0][0] = c.g[V0][0];
                                            call_ptr(c, 0x00144bc0, 0x002365c4)?;
                                            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                            c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0xe));
                                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe));
                                            c.g[A2][0] = (c.g[A2][0] < 0x3u64) as u64;
                                            let t = c.g[V0][0] != c.g[V1][0];
                                            c.g[A0][0] = (c.g[A0][0] < 0x3u64) as u64;
                                            if t {
                                                break 'b_236624;
                                            } else {
                                                c.cycles += 2;
                                                let t = c.g[A0][0] != 0u64;
                                                c.g[V0][0] = 0x530000;
                                                if t {
                                                    break 'b_23662c;
                                                } else {
                                                    c.cycles += 15;
                                                    c.g[V0][0] = 0x3;
                                                    c.g[RA][0] = sx(0x002365f4);
                                                    c.sw(
                                                        (c.g[S0][0] as u32).wrapping_add(0x10),
                                                        c.g[V0][0],
                                                    );
                                                    call_ptr(c, 0x0014dc80, 0x002365f4)?;
                                                    c.g[A0][0] = c.g[V0][0];
                                                    c.g[RA][0] = sx(0x00236600);
                                                    c.g[A1][0] = 0xa;
                                                    call_ptr(c, 0x0014dd58, 0x00236600)?;
                                                    c.sw(
                                                        (c.g[V0][0] as u32).wrapping_add(0x10),
                                                        c.g[S2][0],
                                                    );
                                                    c.g[A1][0] = 0xf;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x848)
                                                        );
                                                    c.g[A2][0] = 0u64;
                                                    c.g[A3][0] = 0u64;
                                                    c.g[RA][0] = sx(0x0023661c);
                                                    c.g[A0][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                                    Callee {
                                                        addr: 0x00231250,
                                                        f: ee::s06::GameModule_RequestState,
                                                    }
                                                    .call(c, 0x0023661c)?;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x30),
                                                    );
                                                    if t {
                                                        break 'b_236714;
                                                    } else {
                                                        break 'b_236624;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = c.g[A0][0] == 0u64;
                                        c.g[V0][0] = 0x530000;
                                        if t {
                                            break 'b_236638;
                                        } else {
                                            break 'b_23662c;
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c11));
                                    if c.g[V1][0] == 0u64 {
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), 0u64);
                                        break 'b_236638;
                                    } else {
                                        break 'b_236638;
                                    }
                                }
                                'b_236644: {
                                    c.cycles += 2;
                                    let t = c.g[A0][0] != 0u64;
                                    c.g[A1][0] = 0u64;
                                    if t {
                                        break 'b_236644;
                                    } else {
                                        c.cycles += 1;
                                        c.g[A1][0] = (c.g[A2][0] < 0x1u64) as u64;
                                        break 'b_236644;
                                    }
                                }
                                c.cycles += 5;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                                c.g[RA][0] = sx(0x00236650);
                                c.g[A2][0] = 0x1;
                                call_ptr(c, 0x0027a860, 0x00236650)?;
                                let t = true;
                                c.g[V0][0] = 0x2;
                                if t {
                                    break 'b_23670c;
                                } else {
                                    break 'b_236658;
                                }
                            }
                        }
                    }
                    'b_236700: {
                        c.cycles += 2;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x84c));
                        if t {
                            break 'b_236700;
                        } else {
                            'b_2366d4: {
                                'b_236684: {
                                    c.cycles += 7;
                                    c.g[A1][0] = 0x1;
                                    c.g[RA][0] = sx(0x0023666c);
                                    c.g[A2][0] = 0u64;
                                    call_ptr(c, 0x00279148, 0x0023666c)?;
                                    c.g[V1][0] = c.g[V0][0];
                                    c.g[V0][0] = 0xe;
                                    let t = c.g[V1][0] == c.g[V0][0];
                                    c.g[V0][0] = 0x14;
                                    if t {
                                        break 'b_236684;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] != c.g[V0][0];
                                        c.g[V0][0] = 0xd;
                                        if t {
                                            break 'b_2366d4;
                                        } else {
                                            break 'b_236684;
                                        }
                                    }
                                }
                                'b_2366a8: {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    if c.g[V0][0] != 0u64 {
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), 0u64);
                                        break 'b_2366a8;
                                    } else {
                                        c.cycles += 3;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
                                        if c.g[V0][0] != 0u64 {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                            break 'b_236714;
                                        } else {
                                            c.cycles += 3;
                                            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[S2][0]);
                                            let t = true;
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0x1c),
                                                c.g[S2][0],
                                            );
                                            if t {
                                                break 'b_236710;
                                            } else {
                                                break 'b_2366a8;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 11;
                                c.g[A2][0] = 0x7;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                                c.sw((c.g[V1][0] as u32).wrapping_add(0x1c8), c.g[S2][0]);
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x84));
                                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x18));
                                c.g[RA][0] = sx(0x002366cc);
                                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                                call_ptr(c, 0x0022cea8, 0x002366cc)?;
                                let t = true;
                                c.g[V0][0] = 0x3;
                                if t {
                                    break 'b_23670c;
                                } else {
                                    break 'b_2366d4;
                                }
                            }
                            'b_2366e4: {
                                c.cycles += 2;
                                let t = c.g[V1][0] == c.g[V0][0];
                                c.g[V0][0] = 0x13;
                                if t {
                                    break 'b_2366e4;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V1][0] != c.g[V0][0];
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                                    if t {
                                        break 'b_236714;
                                    } else {
                                        break 'b_2366e4;
                                    }
                                }
                            }
                            c.cycles += 7;
                            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[A1][0] = 0u64;
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
                            c.g[RA][0] = sx(0x002366f8);
                            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x94));
                            call_ptr(c, 0x001f36c0, 0x002366f8)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
                            if t {
                                break 'b_236714;
                            } else {
                                break 'b_236700;
                            }
                        }
                    }
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x00236708);
                    c.g[A0][0] = c.g[S0][0];
                    Callee {
                        addr: 0x00233aa0,
                        f: ee::s06::GameState_GotoNext,
                    }
                    .call(c, 0x00236708)?;
                    c.g[V0][0] = 0x3;
                    break 'b_23670c;
                }
                c.cycles += 1;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                break 'b_236710;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            break 'b_236714;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
    });
}

/// `void GameStateEventSwitchLoad_Update(self)` (0x236960, `cGFSectionLoadState::update`): while the section manager waits for the transport, state 0 sets the stream focus at the NIS end position once the section is loaded, state 1 waits for the focus, the transition manager and the NIS transition, ends the transport sequence and activates the section.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]

/// `void InputSync_dtor(InputSync *self, int flags)` (0x2370a0, `cInputSync::~cInputSync`): for every synced pad free its samples (if any) and delete its destination pad and input; delete `self` when `flags & 1`.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn input_sync_dtor_2(c: &mut Cpu) {
    guest(c, |c| {
        'b_237110: {
            c.cycles += 11;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[S1][0] = 0u64;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S2][0] = c.g[A0][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
            let t = (c.g[V0][0] as i64) <= 0;
            c.g[S3][0] = c.g[A1][0];
            if t {
                break 'b_237110;
            } else {
                c.cycles += 3;
                c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x14));
                c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                'l_2370d8: loop {
                    'b_2370ec: {
                        c.cycles += 2;
                        if c.g[A0][0] == 0u64 {
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                            break 'b_2370ec;
                        } else {
                            c.cycles += 3;
                            c.g[RA][0] = sx(0x002370e8);
                            call_ptr(c, 0x00317e98, 0x002370e8)?;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                            break 'b_2370ec;
                        }
                    }
                    c.cycles += 9;
                    c.g[RA][0] = sx(0x002370f4);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    call_ptr(c, 0x00317e50, 0x002370f4)?;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                    c.g[RA][0] = sx(0x00237100);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x00317e50, 0x00237100)?;
                    c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                        if event_due(c) {
                            deliver_event_here(c, 0x002370d8);
                        }
                        continue 'l_2370d8;
                    } else {
                        break 'b_237110;
                    }
                }
            }
        }
        'b_237128: {
            c.cycles += 3;
            c.g[V0][0] = c.g[S3][0] & 0x1;
            let t = c.g[V0][0] == 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            if t {
                break 'b_237128;
            } else {
                c.cycles += 3;
                c.g[RA][0] = sx(0x00237124);
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x00317e50, 0x00237124)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                break 'b_237128;
            }
        }
        c.cycles += 6;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `void InputSync_SendSample(InputSync *self, Sample *src)` (0x237a98, `cInputSync::sendClientInput`): pack the sample into a 0x20-byte stack buffer, unpack it back into `self` (the local copy), and send the packet through `cGameComm` when it is synced, error-free and connected.
#[allow(
    unused_labels,
    unused_assignments,
    unused_variables,
    unreachable_code,
    unused_mut
)]
pub fn input_sync_send_sample_3(c: &mut Cpu) {
    guest(c, |c| {
        'b_237af4: {
            c.cycles += 18;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[A2][0] = c.g[SP][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S0][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.g[RA][0] = sx(0x00237abc);
            c.g[S1][0] = c.g[A1][0];
            Callee {
                addr: 0x00237b28,
                f: ee::s06::InputSync_PackSample,
            }
            .call(c, 0x00237abc)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x00237ad0);
            c.g[A2][0] = c.g[SP][0];
            Callee {
                addr: 0x00237be8,
                f: ee::s06::InputSync_UnpackSample,
            }
            .call(c, 0x00237ad0)?;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x40));
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = 0u64;
            if t {
                break 'b_237af4;
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                let t = c.g[V0][0] != 0u64;

                if t {
                    break 'b_237af4;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    c.g[A0][0] = (0u64 < c.g[V0][0]) as u64;
                    break 'b_237af4;
                }
            }
        }
        'b_237b0c: {
            c.cycles += 2;
            let t = c.g[A0][0] == 0u64;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
            if t {
                break 'b_237b0c;
            } else {
                c.cycles += 4;
                c.g[A3][0] = c.g[S2][0];
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x00237b0c);
                c.g[A2][0] = c.g[SP][0];
                Callee {
                    addr: 0x00257e98,
                    f: ee::s06::cGameComm_SendPacket,
                }
                .call(c, 0x00237b0c)?;
                break 'b_237b0c;
            }
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    });
}

/// `void GameStateEventSwitchLoad_Update(GameState* self)` (0x236960), the C++
/// `cGFSectionLoadState_update`. While the section manager waits for the
/// transport (`mWaitTransport`, +0x1c8) it steps `self->state` (+0x10):
/// state 0 waits for the section to be loaded, then puts the world-view
/// stream focus (slot `idx`, the one the player camera does not use) at the
/// NIS end position (radius 45000) and moves to state 1; state 1 waits until
/// the focus is closer than 20000 and the transition manager, the NIS
/// transition and the script engine are idle, then ends the transport
/// sequence, activates the section (7) and moves to state 2.
pub fn game_state_event_switch_load_update(c: &mut Cpu) {
    guest(c, |c| {
        let gp = c.g[GP][0] as u32;
        c.cycles += 14;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = Addr(gp.wrapping_sub(0x848)).lw(c);
        frame.save(c, S4, 0x30);
        frame.save(c, S0, 0x70);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        frame.save_ra(c, 0x20);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
        c.g[S2][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x78)).lw(c);
        c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(0x1c8)).lw(c);
        let not_waiting = c.g[V0][0] == 0;
        c.g[S3][0] = 0x1;
        'exit: {
            'done: {
                if not_waiting {
                    break 'done;
                }
                'state1: {
                    c.cycles += 7;
                    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x84)).lw(c);
                    c.g[A1][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x10)).lw(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4)).lw(c);
                    c.g[S1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x10)).lw(c);
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xb0)).lw(c);
                    let is_state1 = c.g[A1][0] == c.g[S3][0];
                    c.g[S0][0] = (c.g[V0][0] < 0x1u64) as u64;
                    if is_state1 {
                        break 'state1;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A1][0] as i64) < 2i64) as u64;
                    if c.g[V0][0] == 0 {
                        frame.restore(c, S0, 0x70);
                        break 'exit;
                    }
                    c.cycles += 2;
                    if c.g[A1][0] != 0 {
                        frame.restore(c, S0, 0x70);
                        break 'exit;
                    }

                    // State 0: wait for the section to be loaded.
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x1bc)).lw(c);
                    call_ptr(c, 0x0022_d278, 0x0023_69d4)?; // cSectionMan_isLoaded
                    let not_loaded = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[SP][0];
                    if not_loaded {
                        break 'done;
                    }
                    c.cycles += 19;
                    c.g[A1][0] = Addr(gp.wrapping_sub(0x848)).lw(c);
                    c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x84)).lw(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xc)).lw(c);
                    c.g[A2][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x40)).lw(c);
                    c.g[A1][0] = Addr((c.g[A2][0] as u32).wrapping_add(0x18)).lw(c);
                    call_ptr(c, 0x0012_3f38, 0x0023_69f4)?; // cRider_GetNISEndPosition
                    c.g[V0][0] = 0x50;
                    c.g[V1][0] = Addr(c.g[S1][0] as u32).lw(c);
                    c.mult(0, 2, 16, 2);
                    c.lwc1(12, gp.wrapping_sub(0x5168));
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[SP][0];
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    let slot = c.g[V1][0] as u32;
                    c.sw(slot.wrapping_add(0x250), c.g[S3][0]);
                    c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                    call_ptr(c, 0x003a_9658, 0x0023_6a20)?; // cWorldView_setFocus
                    let state = c.g[S4][0] as u32;
                    c.sw(state.wrapping_add(0x10), c.g[S3][0]);
                    break 'done;
                }

                // State 1: wait for the focus to arrive and everything to idle.
                c.cycles += 12;
                c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.g[SP][0];
                c.g[A3][0] = sx(sp(c).wrapping_add(0x10));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
                call_ptr(c, 0x003a_9770, 0x0023_6a40)?; // cWorldView_getTestFocus
                c.lwc1(1, sp(c).wrapping_add(0x10));
                c.lwc1(0, gp.wrapping_sub(0x5164));
                c.c_lt_s(1, 0);
                let close = c.fcc();
                frame.restore(c, S0, 0x70);
                if close {
                    break 'exit;
                }
                c.cycles += 13;
                c.g[V0][0] = Addr(gp.wrapping_sub(0x848)).lw(c);
                c.g[A1][0] = 0;
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x84)).lw(c);
                call_ptr(c, 0x0023_0698, 0x0023_6a68)?; // GameModule_GetTransitionMan
                c.g[S0][0] = c.g[V0][0];
                c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x1c)).lw(c);
                c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x10));
                c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x40)).lh(c);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x44)).lw(c);
                let is_busy = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, is_busy, 0x0023_6a84)?; // isBusy()
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x70);
                    break 'exit;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x44)).lw(c);
                let playing = c.g[V0][0] != 0;
                frame.restore(c, S0, 0x70);
                if playing {
                    break 'exit;
                }
                c.cycles += 4;
                c.g[A0][0] = Addr(gp.wrapping_sub(0x84c)).lw(c);
                call_ptr(c, 0x0027_9298, 0x0023_6aa0)?; // IsActiveNISInTransition
                let nis = c.g[V0][0] != 0;
                frame.restore(c, S0, 0x70);
                if nis {
                    break 'exit;
                }
                c.cycles += 9;
                c.g[A0][0] = Addr(gp.wrapping_sub(0x84c)).lw(c);
                call_ptr(c, 0x0027_a9f0, 0x0023_6ab0)?; // EndTransportSequence
                let man = c.g[S2][0] as u32;
                c.sw(man.wrapping_add(0x1c8), 0);
                c.g[A0][0] = c.g[S2][0];
                c.g[A2][0] = 0x7;
                c.g[A1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x1bc)).lw(c);
                call_ptr(c, 0x0022_d088, 0x0023_6ac4)?; // cSectionMan_activate
                c.g[V0][0] = 0x2;
                let state = c.g[S4][0] as u32;
                c.sw(state.wrapping_add(0x10), c.g[V0][0]);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x70);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    });
}

/// `GameStateEventSwitchLoad_Exit` (0x0023_6af0): straight-line leaf. Mechanical port of the generated body.
pub fn game_state_event_switch_load_exit_00236af0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GameStateReplayPlayback_Update` (0x0023_7030): straight-line leaf. Mechanical port of the generated body.
pub fn game_state_replay_playback_update_00237030(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GameStateEventSwitchLoad_Enter(self)` (0x2368a0): `BELibrary_SetPhase(0)`,
/// `Replay_Reset` of the replay at `gp-0x488`'s `+0x28`, then the BE interface
/// work: unless the flag byte at `0x535bc8 + 0x49` is set, one
/// `BENewPlayerInt_SetFirstTimeWCFlag(iface, i, 0)` for each of the `0x535bc8 +
/// 0x30` entries. `self + 0x10` is cleared on every exit that does not unwind.
pub fn game_state_event_switch_load_enter_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S3, 0x10);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0;
        call_ptr(c, 0x0014_de68, 0x0023_68c4)?;
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x488)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x28)).lw(c);
        call_ptr(c, 0x0026_f228, 0x0023_68d0)?;
        call_ptr(c, 0x0014_dc80, 0x0023_68d8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x0023_68e4)?;
        c.g[V1][0] = 0x0053_0000;
        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x49)).lb(c);
        if c.g[V0][0] != 0 {
            c.sw((c.g[S3][0] as u32).wrapping_add(0x10), 0u64);
        } else {
            c.cycles += 9;
            c.g[S0][0] = 0;
            call_ptr(c, 0x0014_dc80, 0x0023_6900)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0014_dd58, 0x0023_690c)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x30)).lw(c);
            if (c.g[V0][0] as i64) <= 0 {
                c.sw((c.g[S3][0] as u32).wrapping_add(0x10), 0u64);
            } else {
                c.cycles += 1;
                c.g[A1][0] = c.g[S0][0];
                loop {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A2][0] = 0;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    call_ptr(c, 0x0014_5cb0, 0x0023_6930)?;
                    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x30)).lw(c);
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A1][0] = c.g[S0][0];
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0023_6920);
                        }
                        continue;
                    }
                    c.cycles += 1;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x10), 0u64);
                    break;
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::game::main::game::tests::fake_get_be;
    use crate::game::main::game::tests::PREV_GET_BE;
    use crate::game::main::game::tests::{fake_be_get_interface, PREV_BE_GET_INTERFACE};

    /// Above 60 Hz the countdown's light check runs once per countdown value:
    /// a repeated positive value returns at once (nothing read past the
    /// counter, no cycles), a new one runs the body.
    #[test]
    fn game_state_countdown_update_once_per_value() {
        let mut c = crate::difftest::fresh();
        crate::tick::set_for_test(&mut c, 240);
        c.g[GP][0] = GAME_GP as u64;
        let world = DATA;
        let race = DATA + 0x100;
        c.bus.write32(GAME_GP - 0x488, world);
        c.bus.write32(world + 0xc, race);
        c.bus.write32(race + 0x1c, 120);
        // The first sight of a value runs the body; mark it as seen without
        // running it (this test's world is not a game).
        assert!(crate::tick::value_changed(&mut c, race + 0x1c, 120));
        let cycles = c.cycles;
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = 0x0023_4c68;
        game_state_countdown_update(&mut c);
        assert_eq!((c.pc, c.cycles), (crate::difftest::RA, cycles));
        assert!(crate::tick::value_changed(&mut c, race + 0x1c, 119));
        crate::tick::set_for_test(&mut c, 60);
        assert!(
            crate::tick::value_changed(&mut c, race + 0x1c, 119),
            "always at 60"
        );
    }
    use crate::difftest::check;
    use crate::difftest::check_lohi;

    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fakes::LOG_UNWIND_AT;

    use crate::difftest::fill;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    use crate::difftest::RA as TEST_RA;

    use crate::difftest::SP;

    pub(crate) use crate::game::ai::control::handplantcontrol::tests::put;
    pub(crate) use crate::game::be::bedbprofile::tests::strict;
    pub(crate) use crate::game::bx::bxstringctor::tests::be_library;

    pub(crate) use crate::game::dirtysock::tagsunk::tests::base_8 as base;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::be_library_2;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::rng_2 as rng;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::zero;

    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_2 as pool_allocator;

    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::W_3 as W;

    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadscreen::tests::r32_seed;

    pub(crate) use crate::game::fe::festateoptions::tests::setup;
    pub(crate) use crate::game::fe::festateoptions::tests::AUX;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4;

    pub(crate) use crate::game::fe::festatestore::tests::world;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::belib;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::w;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::fe::ovstates::tests::base_3 as base_7;
    pub(crate) use crate::game::main::gamemode::tests::world as world_2;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS;
    pub(crate) use crate::game::movie::movieplayer::tests::globals;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::gp_at;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_4;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS_3;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn game_state_reset_ctor_matches() {
        check(
            ssx3_ee::s06::f_00234ef8,
            game_state_reset_ctor,
            64,
            &|c, seed| {
                let mut r = Rng(0x0c11_000f ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.bus.write32(GAME_GP.wrapping_sub(0x488), r.next_u32());
                for i in 0..8 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn game_state_event_select_ctor_matches() {
        check(
            ssx3_ee::s06::f_00236208,
            game_state_event_select_ctor,
            64,
            &|c, seed| {
                let mut r = Rng(0x0c11_0010 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.bus.write32(GAME_GP.wrapping_sub(0x488), r.next_u32());
                for i in 0..8 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn game_state_event_switch_load_ctor_matches() {
        check(
            ssx3_ee::s06::f_00236868,
            game_state_event_switch_load_ctor,
            64,
            &|c, seed| {
                let mut r = Rng(0x0c11_0011 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.g[A0][0] = DATA as u64;
                c.bus.write32(GAME_GP.wrapping_sub(0x488), r.next_u32());
                for i in 0..4 {
                    c.bus.write32(DATA + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn input_sync_ctor_matches() {
        check_lohi(
            ssx3_ee::s06::f_00237060,
            input_sync_ctor,
            300,
            &|c, seed| {
                let mut r = Rng(0x0c11_0012 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.g[A2][0] = match seed % 5 {
                    0 => 0,
                    1 => 1,
                    2 => (r.next_u32() & 0xffff) as u64,
                    3 => 0xffff_ffff,
                    _ => ((r.next_u32() as u64) << 32) | r.next_u32() as u64,
                };
                c.g[A3][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
            },
        );
    }

    pub(crate) fn rand_setup(c: &mut Cpu, seed: u32, state: u32) {
        let mut r = machine(c, seed, 0x237c_d8, 0);
        for i in 0..8u32 {
            let v = if seed % 4 == 0 { 0 } else { r.next_u32() };
            c.bus.write32(state + 4 * i, v);
        }
    }

    #[test]
    fn game_rand_next_matches() {
        check(
            ee::s06::f_00237cd8__generated,
            game_rand_next,
            200,
            &|c, seed| {
                rand_setup(c, seed, 0x004c_9548);
            },
        );
    }

    #[test]
    fn game_state_results_exit_matches() {
        crate::difftest::check(
            ee::s06::f_00236ea0__generated,
            game_state_results_exit,
            40,
            &|c, seed| {
                machine(c, seed, 0x236e_a0, 2);
                be_library(c);
                // `D_004A2C68` (the game state manager): a dummy whose `+0x48`
                // state list head is empty, so the close-all walk stops at once.
                c.bus.write32(GAME_GP.wrapping_sub(0x488), DATA + 0x2000);
                c.bus
                    .write32(DATA + 0x2048, if seed % 2 == 0 { 0 } else { DATA + 0x2000 });
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn game_rand_seed_matches() {
        crate::difftest::check(
            ee::s06::f_00237cb0__generated,
            game_rand_seed,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x237c_b0, 1);
                c.g[A0][0] = if seed % 3 == 0 {
                    0
                } else {
                    r.next_u32() as u64
                };
            },
        );
    }

    #[test]
    fn game_state_restart_race_exit_matches() {
        check(
            ssx3_ee::s06::f_00235f20__generated,
            game_state_restart_race_exit,
            200,
            &|c, seed| {
                let mut r = rng(seed, 0x1d);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let world = DATA + 0x8000;
                let race = DATA + 0x9000;
                let rider = DATA + 0xa000;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x84, race);
                c.bus.write32(race + 0x94, rider);
                c.bus.write32(rider + 4, r.next_u32());
                c.bus.write32(rider + 8, r.next_u32());
            },
        );
    }

    /// The replay-playback world: a world whose race manager owns an empty
    /// playback record and an empty world-script manager. The BE library
    /// singleton is a zeroed dummy and its interface slot points at a leaf
    /// function, so `WScriptMan_SpawnMissions` takes its early return.
    pub(crate) fn replay_exit_setup(c: &mut Cpu, seed: u32) {
        let mut r = rng(seed, 0x1e);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        let world = DATA + 0x8000;
        let race = DATA + 0x9000;
        let playback = DATA + 0xb000;
        let wscript = DATA + 0xa000;
        c.bus.write32(GAME_GP - 0x848, world);
        c.bus.write32(world + 0x84, race);
        c.bus.write32(race + 0x34, playback);
        // PlaybackMan_FreeData: a -1 block handle skips the release, and a
        // zeroed record takes the null-list path down to the null frees.
        c.bus.write32(playback + 0x78, u32::MAX);
        c.bus.write32(playback + 0x4c, 0);
        c.bus.write32(playback + 0x50, 0);
        c.bus.write32(playback + 0x58, 0);
        // WScriptMan_DeleteAllProcesses: empty process lists.
        c.bus.write32(GAME_GP + 0xce8, wscript);
        // BELibrary_GetInstance returns the existing singleton.
        c.bus.write32(GAME_GP - 0x1f04, DATA + 0xc000);
        // BELibrary_GetInterface: index 0 -> lib -> vtable slot 8 -> the
        // camera-trigger-stack clear leaf, which only writes its argument.
        const BE_LIBRARY: u32 = 0x0053_08a8;
        let lib = DATA + 0xd000;
        let vt = DATA + 0xe000;
        c.bus.write32(GAME_GP - 0x1efc, 0);
        c.bus.write32(BE_LIBRARY, lib);
        c.bus.write32(lib + 4, vt);
        c.bus.write16(vt + 8, 0);
        c.bus.write32(vt + 0xc, 0x0016_d200);
        // WScriptMan_SpawnMissions leaves early unless the world type is 4.
        c.bus.write8(0x0053_5c10, (seed % 3) as u8);
    }

    #[test]
    fn game_state_replay_playback_exit_matches() {
        check(
            ssx3_ee::s06::f_00237038__generated,
            game_state_replay_playback_exit,
            60,
            &|c, seed| replay_exit_setup(c, seed),
        );
    }

    #[test]
    fn game_state_warp_to_event_update_matches() {
        check(
            ssx3_ee::s06::f_00236198__generated,
            game_state_warp_to_event_update,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0xb3, 1);
                globals(c);
            },
        );
    }

    /// The three script-list helpers: engine + rider index (+ payload).
    pub(crate) fn script_list_test(generated: GuestFn, hand: GuestFn, salt: u64, payload: bool) {
        check(generated, hand, 40, &|c, seed| {
            let mut r = machine(c, seed, salt, 1);
            globals(c);
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed % 3) as u64;
            if payload {
                c.g[A2][0] = (seed % 2) as u64;
            }
            // The rider script-list array.
            c.bus.write32(DATA + 0x54c, DATA + 0x4000);
            let _ = r.next_u32();
        });
    }

    #[test]
    fn script_engine_list_prepare_matches() {
        script_list_test(
            ssx3_ee::s07::f_00278f38__generated,
            script_engine_list_prepare,
            0xca,
            false,
        );
    }

    #[test]
    fn game_state_reset_ai_exit_matches() {
        check(
            ssx3_ee::s06::f_00236b38__generated,
            game_state_reset_ai_exit,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x36b38, 1);
                let state = DATA + 0x400;
                let ai = DATA + 0x500;
                let vt = DATA + 0x600;
                c.bus.write32(GAME_GP.wrapping_sub(0x488), state);
                c.bus.write32(state + 0xc, ai);
                c.bus.write32(ai + 0xcc, vt);
                c.bus.write16(vt + 0x28, ((r.next_u32() % 0x40) * 4) as u16);
                c.bus.write32(vt + 0x2c, FAKE);
                c.g[A0][0] = sx(DATA + 0x300);
            },
        );
    }

    #[test]
    fn game_state_finish_cinematic_enter_matches() {
        check(
            ee::s06::f_00236f08__generated,
            game_state_finish_cinematic_enter,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x236f_08, 0);
                zero(c, DATA + 0x400, 0x40);
                c.bus.write32(
                    GAME_GP.wrapping_add(0x410),
                    if seed % 2 == 0 { 0 } else { DATA + 0x400 },
                );
                c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x1000);
                zero(c, DATA + 0x1000, 0x400);
                c.bus.write32(DATA + 0x1000 + 0x84, DATA + 0x1100);
                c.bus.write32(DATA + 0x1100 + 0xc, DATA + 0x1200);
            },
        );
    }

    #[test]
    fn game_state_reset_overlay_exit_matches() {
        check(
            ssx3_ee::s06::f_00235570__generated,
            game_state_reset_overlay_exit,
            60,
            &|c, seed| {
                machine(c, seed, 0x2355_70, 2);
                belib(c);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn game_state_results_update_matches() {
        check(
            ssx3_ee::s06::f_00236e60__generated,
            game_state_results_update,
            SEEDS_2,
            &|c, seed| {
                machine(c, seed, 0x236e_60, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
                w(c, DATA + 8, if seed % 2 == 0 { 1 } else { 0 });
            },
        );
    }

    // ------------------------------------------------------------------
    // Game state / mode dispatch.

    #[test]
    fn game_state_pre_race_menu_update_matches() {
        check(
            ee::s06::f_00236c48__generated,
            game_state_pre_race_menu_update,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x236c48, 0);
                let mode = seed % 4;
                let self_at = DATA + 0x200;
                c.g[A0][0] = self_at as u64;
                c.bus.write32(self_at, 0);
                c.bus
                    .write32(GAME_GP - 0x204, if mode < 2 { 0 } else { DATA + 0x300 });
                c.bus.write32(DATA + 0x304, (mode == 2) as u32);
                c.bus.write32(self_at + 8, (mode % 2 == 0) as u32);
            },
        );
    }

    #[test]
    fn game_state_replay_playback_enter_matches() {
        check(
            ssx3_ee::s06::f_00236fe8__generated,
            game_state_replay_playback_enter,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x236fe8, 0);
                be_library_2(c);
                pool_allocator(c);
                world(c);
                // No script manager installed: both script calls take their empty
                // path.
                c.bus.write32(GAME_GP.wrapping_add(0xce8), 0);
            },
        );
    }

    #[test]
    fn game_state_finish_cinematic_update_matches() {
        check(
            ee::s06::f_00236f40__generated,
            game_state_finish_cinematic_update,
            SEEDS_3,
            &|c, seed| {
                let _ = machine(c, seed, 0x236f_40, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x20);
                c.g[A0][0] = sx(this);
                clear(c, DATA + 0x400, 0x40);
                c.bus
                    .write32(gp_at(-0x84c), if seed % 3 == 0 { 0 } else { DATA + 0x400 });
                c.bus.write32(LOG_RESULTS, 5);
            },
        );
    }

    // -------------------------------------------------------- game states

    #[test]
    fn game_state_pre_race_menu_exit_matches() {
        check(
            ee::s06::f_00236c88__generated,
            game_state_pre_race_menu_exit,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x236c);
                let depth = 1 + r.next_u32() % 4;
                c.bus.write32(gp_at(-0x69c), depth);
                for i in 0..4u32 {
                    c.bus.write32(0x0053_66e8 + 4 * i, (i * 3 + seed) % 16);
                }
                for i in 0..16u32 {
                    c.bus.write32(0x0044_28f0 + 4 * i, r.next_u32());
                }
                c.bus.write32(gp_at(-0x6a0), 0xdead);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn game_state_reset_v08_matches() {
        check(
            ee::s06::f_002355b0__generated,
            game_state_reset_v08,
            200,
            &|c, seed| {
                setup(c, seed, 19);
                c.g[A0][0] = AUX as u64;
                put(c, AUX + 0x10, [5, 4, 6][(seed % 3) as usize]);
            },
        );
    }

    #[test]
    fn game_state_finish_cinematic_exit_matches() {
        check(
            ee::s06::f_00236f90__generated,
            game_state_finish_cinematic_exit,
            200,
            &|c, seed| {
                let mut r = setup(c, seed, 20);
                world_2(c, 0);
                put(c, AUX + 0x300 + 0x8c, r.next_u32());
            },
        );
    }

    #[test]
    fn game_state_reset_update_matches() {
        check(
            ssx3_ee::s06::f_002357f8__generated,
            game_state_reset_update,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x2357f8, 0);
                let s = DATA + 0x1000;
                let engine = DATA + 0x2000;
                let world = DATA + 0x3000;
                let race = DATA + 0x4000;
                let arr = DATA + 0x5000;
                let table = DATA + 0x6000;
                c.g[A0][0] = s as u64;
                w32(c, s + 0x1c, if seed % 2 == 0 { 0 } else { 1 });
                w32(c, s, 0); // GotoNext takes its early-out
                w32(c, GAME_GP.wrapping_sub(0x84c), engine);
                w32(c, GAME_GP.wrapping_sub(0x848), world);
                w32(c, engine + 0x550, 0);
                w32(c, engine + 0x54c, table);
                w32(c, table + 0xcc + 0xa4, 0); // GetPlayStatus short path
                w32(c, world + 0x84, race);
                w32(c, race + 0x78, arr);
                w32(c, arr + 0x1a4, 0);
            },
        );
    }

    #[test]
    fn input_sync_is_ready_matches() {
        check(
            ee::s06::f_00237950__generated,
            input_sync_is_ready,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x23_7950, 0);
                w(c, GAME_GP.wrapping_sub(0x204), DATA + 0x3000);
                w(c, DATA + 0x3000 + 0x70, (seed % 20) as u32);
                w(c, DATA + 0x3000 + 0x3f8, (seed % 3 == 0) as u32);
                w(c, DATA + 0x3000 + 0x3fc, if seed % 3 == 1 { 2 } else { 0 });
                c.g[A0][0] = (DATA + 0x2000) as u64;
                let n = ((seed / 3) % 4) as u32;
                w(c, DATA + 0x2000 + 0x10, n);
                for i in 0..4u32 {
                    let s = DATA + 0x2000 + 0x14 + 0x18 * i;
                    w(c, s + 4, 1 + seed % 3);
                    let full = i + 1 == n && seed % 5 == 0;
                    w(c, s + 8, if full { 1 + seed % 3 } else { 2 });
                }
                c.pc = 0x0023_7950;
            },
        );
    }

    #[test]
    fn game_state_play_script_event_matches() {
        check(
            ee::s06::f_00234be8__generated,
            game_state_play_script_event,
            40,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x234b, 3);
                let m = DATA + 0x1000;
                let idx = (seed % 4) as u32;
                // The script table slot decides whether the event is called.
                w(
                    c,
                    m + 0x3cc + 4 * idx,
                    if seed % 2 == 0 { 0 } else { DATA + 0x2000 },
                );
                w(c, DATA + 0x2000 + 8, if seed % 3 == 0 { 5 } else { 0 });
                w(
                    c,
                    GAME_GP.wrapping_add(0xce8),
                    if seed % 2 == 0 { 0 } else { m },
                );
                fill(c, &mut r, DATA + 0x2000, 8);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x200);
            },
        );
    }

    pub(crate) const SEEDS_5: u32 = 100;

    /// Scrambled machine with the two singletons every game path expects: a
    /// non-null BELibrary (so it never builds itself) and a dummy world.
    pub(crate) fn base_3(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        c.bus.write32(GAME_GP - 0x1f04, DATA + 0xa000);
        c.bus.write32(GAME_GP - 0x848, DATA + 0x9000);
        c.bus.write32(DATA + 0x9000 + 0x84, DATA + 0x9400);
        r.next_u32();
        r
    }

    /// A `{ short delta; void* fn; }` vtable slot_2 with a chosen delta.
    pub(crate) fn slot_2(c: &mut Cpu, vt: u32, off: u32, delta: u16, func: u32) {
        c.bus.write16(vt + off, delta);
        c.bus.write32(vt + off + 4, func);
    }

    #[test]
    fn game_state_load_state_matches() {
        for (gen, hand) in [
            (
                ee::s06::f_00235a18__generated as fn(&mut Cpu),
                gamestatereset_load_state as fn(&mut Cpu),
            ),
            (
                ee::s06::f_00235fd0__generated,
                gamestaterestartrace_load_state,
            ),
        ] {
            covered(gen, hand, SEEDS_5, &|c, seed| {
                base_3(c, seed, 0x35a18, 4);
                let state = DATA + 0x100;
                let data = DATA + 0x200;
                c.g[A0][0] = sx(state);
                c.g[A1][0] = sx(data);
                let vtobj = DATA + 0x300;
                c.bus.write32(data, vtobj);
                slot_2(c, vtobj, 0x10, 0, FAKE);
            });
        }
    }

    #[test]
    fn gamestatepreracemenu_resume_race_matches() {
        covered(
            ee::s06::f_00236cd8__generated,
            gamestatepreracemenu_resume_race,
            SEEDS_5,
            &|c, seed| {
                base_3(c, seed, 0x36cd8, 3);
                let engine = DATA + 0x800;
                let obj = DATA + 0x300;
                let mgr = DATA + 0x500;
                let vtobj = DATA + 0x600;
                c.bus.write32(GAME_GP - 0x84c, engine);
                c.bus.write32(GAME_GP - 0x488, obj);
                c.bus.write32(obj + 0xc, mgr);
                c.bus.write32(mgr + 0xcc, vtobj);
                slot_2(c, vtobj, 0x28, 0, FAKE);
                c.bus.write8(0x0053_5c10, if seed % 2 == 0 { 4 } else { 0 });
            },
        );
    }

    // ------------------------------------------------------------------
    // Game state / world.

    #[test]
    fn game_state_race_enter_matches() {
        check(
            ee::s06::f_00234e20__generated,
            game_state_race_enter,
            40,
            &|c, seed| {
                let mut r = base_4(c, seed, 0x234e20, 3);
                let gs = DATA + 0x1000;
                let ai = DATA + 0x2000;
                let riders = DATA + 0x4000;
                let scr = DATA + 0x5000;
                w(c, GAME_GP.wrapping_sub(0x488), gs);
                w(c, gs + 0xc, ai);
                w(c, gs + 0x28, riders);
                w(
                    c,
                    riders,
                    if seed % 3 == 0 { 0 } else { (seed % 12) as u32 },
                );
                w(c, gs + 0x48, scr);
                w(
                    c,
                    0x0053_5c10,
                    if seed % 2 == 0 { 4 } else { (seed % 8) as u32 },
                );
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn game_state_prerace_menu_enter_matches() {
        check(
            ee::s06::f_00236bb0__generated,
            game_state_prerace_menu_enter,
            SEEDS,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                let be = DATA + 0x800;
                match seed % 3 {
                    0 => w(c, gp_6(c, -0x204), 0),
                    1 => {
                        w(c, gp_6(c, -0x204), be);
                        w(c, be + 4, 0);
                    }
                    _ => {
                        w(c, gp_6(c, -0x204), be);
                        w(c, be + 4, DATA + 0x900);
                    }
                }
                w(c, W + 0x1000 + 0x64, 0);
                w(c, DATA + 0x900 + 0x64, 0);
                c.g[A0][0] = this as u64;
            }),
        );
    }

    #[test]
    fn game_state_event_select_load_state_matches() {
        check(
            ssx3_ee::s06::f_002367c8__generated,
            game_state_event_select_load_state,
            10,
            &|c, seed| {
                let _r = base_6(c, seed, 0x2367c8, 2);
                let this = DATA + 0x1000;
                let stream = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = stream as u64;
                w_2(c, stream, DATA + 0x3000);
                c.bus.write16(DATA + 0x3000 + 0x10, 0);
                w_2(c, DATA + 0x3000 + 0x14, 0);
                w_2(c, this + 0x10, r32_seed(seed));
            },
        );
    }

    // ---------------------------------------------------------------- Input.

    #[test]
    fn input_sync_pack_sample_matches() {
        check(
            ee::s06::f_00237b28__generated,
            input_sync_pack_sample,
            200,
            &|c, seed| {
                let mut r = base_7(c, seed, 0x23_7b28);
                let src = DATA + 0x1000;
                let dst = DATA + 0x2000;
                let n = (seed % 6) as u32;
                c.g[A1][0] = sx(src);
                c.g[A2][0] = sx(dst);
                c.g[A0][0] = sx(dst + 1);
                c.bus.write32(src, n);
                for i in 0..6u32 {
                    let zero = seed % 3 == 0 && i < 2;
                    let v = if zero { 0 } else { r.edge_f32() };
                    c.bus.write32(src + 4 + i * 4, v);
                }
                for i in 0..8u32 {
                    c.bus.write32(dst + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn input_sync_unpack_sample_matches() {
        check(
            ee::s06::f_00237be8__generated,
            input_sync_unpack_sample,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x237b, 0);
                let src = DATA + 0x2000;
                let dst = DATA + 0x2400;
                c.g[A0][0] = 0;
                c.g[A1][0] = src as u64;
                c.g[A2][0] = dst as u64;
                let count = (seed % 5) as u32;
                c.bus.write8(src, count as u8);
                for i in 0..4 {
                    c.bus.write8(src + 1 + i, (seed >> i) as u8);
                    c.bus.write8(src + 4 + i, 0x10 + i as u8);
                }
            },
        );
    }

    #[test]
    fn inputsync_apply_input_matches() {
        strict(
            ee::s06::f_002379c8__generated,
            inputsync_apply_input,
            60,
            &|c, seed| {
                let _r = base_8(c, seed, 0x2379);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(gp_at(-0x204), DATA + 0x3000);
                c.bus
                    .write32(DATA + 0x3000 + 0x70, if seed % 4 == 0 { 0 } else { 14 });
                c.bus.write32(DATA + 0x3000 + 0x3f8, 0);
                c.bus.write32(DATA + 0x3000 + 0x3fc, 0);
                let count = (seed % 3) as u32;
                c.bus.write32(DATA + 0x10, count);
                c.bus.write32(DATA + 0x4, 1);
                for i in 0..3u32 {
                    let slot = DATA + 0x14 + 0x18 * i;
                    c.bus.write32(slot, DATA + 0x4000);
                    c.bus.write32(slot + 4, 0);
                    c.bus.write32(slot + 8, 1);
                    c.bus.write32(slot + 0x10, DATA + 0x5000);
                }
                c.bus.write32(DATA + 0x4000, 0x11);
            },
        );
    }

    // ------------------------------------------------------------------
    // GameStateReset_OverlayUpdate / GameStateRestartRace_Update.

    /// The reset/restart fixture's base (above the fakes' result pointers,
    /// `DATA..DATA + 0x4_0000`, and their log at `DATA + 0x10_0000`).
    const RS: u32 = DATA + 0x20_0000;
    const RS_ENGINE: u32 = RS + 0x1000;
    const RS_LISTS: u32 = RS + 0x2000;
    const RS_WORLD: u32 = RS + 0x3000;
    const RS_RACE: u32 = RS + 0x3100;
    const RS_RIDER: u32 = RS + 0x3200;
    const RS_GM: u32 = RS + 0x4000;
    const RS_AI: u32 = RS + 0x4100;
    const RS_OBJ: u32 = RS + 0x4400;
    const RS_RENDER: u32 = RS + 0x4500;
    const RS_LOAD: u32 = RS + 0x4600;
    const RS_MODE: u32 = RS + 0x5000;
    const RS_AUDIO: u32 = RS + 0x6000;
    const RS_AUDIO_VT: u32 = RS + 0xc000;
    const RS_PENDING: u32 = RS + 0xd000;

    /// The script engine (`gp-0x84c`): lists 0 and 1 not loading (state
    /// 2, so `ScriptList_Prepare` returns at once) and full (so the adds
    /// push nothing), no async load polled; the BE library dummy.
    fn rs_engine(c: &mut Cpu, pending: bool) {
        be_library(c);
        c.bus.write32(GAME_GP - 0x84c, RS_ENGINE);
        c.bus.write32(RS_ENGINE + 0x54c, RS_LISTS);
        c.bus.write32(RS_ENGINE + 0x550, 2);
        c.bus.write32(RS_ENGINE + 0x620, 0);
        c.bus
            .write32(RS_ENGINE + 0x538, if pending { RS_PENDING } else { 0 });
        c.bus.write32(RS_ENGINE + 0x630 + 16, 0);
        for i in 0..2 {
            let list = RS_LISTS + 0xcc * i;
            c.bus.write32(list + 0xa0, 1);
            c.bus.write32(list + 0xa4, 2);
            c.bus.write32(list + 0xbc, 0);
            c.bus.write32(list + 0xc0, 0);
        }
    }

    /// The AI (`gp-0x488 -> +0xc`, also `world -> +0x84 -> +0xc`): no
    /// riders, or one rider without a render slot (`computer`/`actor`), or
    /// one rider not loaded yet (`+0x880` = 8: `Rider_LoadStep` does nothing).
    fn rs_ai(c: &mut Cpu, computer_unslotted: bool, actor_unslotted: bool, unloaded: bool) {
        c.bus.write32(GAME_GP - 0x488, RS_GM);
        c.bus.write32(RS_GM + 0xc, RS_AI);
        c.bus.write32(RS_OBJ + 0x18, RS_RENDER);
        c.bus.write32(RS_RENDER + 0x888, u32::MAX);
        c.bus.write32(RS_LOAD + 0x880, 8);
        if computer_unslotted {
            c.bus.write32(RS_AI + 0x80, 1);
            c.bus.write32(RS_AI + 0x48, RS_OBJ);
        }
        if actor_unslotted {
            c.bus.write32(RS_AI + 0x84, 1);
            c.bus.write32(RS_AI + 0x5c, RS_OBJ);
        }
        if unloaded {
            c.bus.write32(RS_AI + 0x78, 1);
            c.bus.write32(RS_AI + 0x28, RS_LOAD);
        }
    }

    fn reset_overlay_setup(c: &mut Cpu, seed: u32) {
        // The step jump table (0x47c060) and the game's data.
        crate::difftest::elf_image(c);
        let mut r = machine(c, seed, 0x2350_80, 6);
        let step = seed % 7;
        let v = seed / 7;
        c.pc = 0x0023_5080;
        c.g[A0][0] = sx(RS);
        c.bus.write32(RS + 0x10, step);
        c.bus
            .write32(RS + 0x14, if v % 2 == 0 { 0 } else { r.next_u32() | 1 });
        let counter = match v % 5 {
            0 => 178,
            1 => 179,
            2 => 180,
            3 => r.next_u32() % 400,
            _ => r.next_u32(),
        };
        c.bus.write32(RS + 0x18, counter);
        rs_engine(c, step == 0 && v % 4 == 3);
        c.bus.write32(GAME_GP - 0x848, RS_WORLD);
        c.bus.write32(RS_WORLD + 0x84, RS_RACE);
        c.bus.write32(RS_RACE + 0x78, RS_RIDER);
        c.bus.write32(RS_RIDER + 0x1bc, 13 + r.next_u32() % 5);
        c.bus.write32(RS_RIDER + 0x1c8, r.next_u32() % 2);
        // The audio system: no song playing, its channel vtable empty (fakes).
        c.bus.write32(GAME_GP + 0x410, RS_AUDIO);
        c.bus.write32(RS_AUDIO + 0x5558, RS_AUDIO_VT);
        // World type, sub-type, mode (0x535c10..12).
        c.bus
            .write8(0x0053_5c10, [0, 0, 1, 5][(r.next_u32() % 4) as usize]);
        c.bus
            .write8(0x0053_5c11, [0, 0, 1][(r.next_u32() % 3) as usize]);
        c.bus.write8(0x0053_5c12, (r.next_u32() % 14) as u8);
        match step {
            1 => {
                for i in 0..2 {
                    c.bus.write32(RS_LISTS + 0xcc * i + 0xa4, r.next_u32() % 3);
                }
            }
            2 => {
                let k = v % 4;
                rs_ai(c, k == 0, k == 1, false);
                c.bus.write32(RS_AI + 0x7c, r.next_u32() % 3);
                c.bus.write32(GAME_GP - 0x848, RS_WORLD);
                c.bus.write32(RS_WORLD + 0xc0, RS_MODE);
                c.bus.write32(RS_MODE + 0x4, r.next_u32() % 13);
                let ai_riders = if k == 3 { 1 + r.next_u32() % 2 } else { 0 };
                let humans = r.next_u32() % 3;
                c.bus.write32(RS_MODE + 0x10, humans + ai_riders);
                c.bus.write32(RS_MODE + 0x14, humans);
                for i in 0..16 {
                    c.bus.write32(RS_MODE + 0x18 + 4 * i, r.next_u32() % 64);
                }
                if ai_riders > 0 {
                    // Creating AI riders is a whole subsystem: unwind at the
                    // second interface commit (after the character loop).
                    let c12 = c.bus.read8(0x0053_5c12);
                    let restart = !(6..=11).contains(&c12);
                    c.bus.write32(LOG_UNWIND_AT, if restart { 6 } else { 3 });
                }
            }
            3 => {
                rs_ai(c, false, false, v % 3 == 0);
                c.bus.write32(RS_AI + 0x80, r.next_u32() % 2);
                c.bus.write32(RS_AI + 0x84, r.next_u32() % 2);
                c.bus.write32(RS_AI + 0x88, 0);
            }
            _ => {}
        }
    }

    #[test]
    fn game_state_reset_overlay_update_matches() {
        check(
            ee::s06::f_00235080__generated,
            game_state_reset_overlay_update,
            7 * 40,
            &|c, seed| reset_overlay_setup(c, seed),
        );
    }

    /// Run `f` (entry `pc`) on the state at `RS` until its step reaches
    /// `want`: the number of updates it took.
    fn rs_ticks(c: &mut Cpu, f: GuestFn, pc: u32, want: u32) -> u32 {
        for n in 1..10_000 {
            c.g[29][0] = sx(SP);
            c.g[RA][0] = sx(TEST_RA);
            c.g[GP][0] = GAME_GP as u64;
            c.g[A0][0] = sx(RS);
            c.pc = pc;
            f(c);
            assert_eq!(c.pc, TEST_RA);
            if c.bus.read32(RS + 0x10) == want {
                return n;
            }
        }
        panic!("step {want} never reached");
    }

    /// The peak transition's 180-frame hold (step 4) lasts 3 s at any rate.
    #[test]
    fn game_state_reset_overlay_update_hold_follows_rate() {
        for (hz, ticks) in [(60, 180), (120, 360), (240, 720)] {
            let mut c = crate::difftest::fresh();
            crate::tick::set_for_test(&mut c, hz);
            c.bus.write32(RS + 0x10, 4);
            c.bus.write32(RS + 0x14, 1);
            c.bus.write32(RS + 0x18, 0);
            assert_eq!(
                rs_ticks(&mut c, game_state_reset_overlay_update, 0x0023_5080, 5),
                ticks,
                "{hz} Hz"
            );
        }
    }

    fn restart_race_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x235c_c8, 6);
        let step = seed % 5;
        let v = seed / 5;
        c.g[A0][0] = sx(RS);
        c.bus.write32(RS + 0x10, step);
        let counter = match v % 5 {
            0 => 118,
            1 => 119,
            2 => 120,
            3 => r.next_u32() % 300,
            _ => r.next_u32(),
        };
        c.bus.write32(RS + 0x14, counter);
        c.bus.write32(RS + 0x18, if v % 3 == 0 { 0 } else { 1 });
        rs_engine(c, false);
        c.bus.write32(RS_LISTS + 0xa4, r.next_u32() % 3);
        if v % 2 == 1 {
            // An active list: its `+0xb0`.
            c.bus.write32(RS_ENGINE + 0x550, 0);
            c.bus.write32(RS_LISTS + 0xb0, r.next_u32() % 2);
        }
        // world -> +0x84 game: +0x84 -> +4 -> +0xb0 (the view index), +0x10
        // the streamer, +0xc the AI, +0x5c the transition manager.
        let game = RS + 0x7000;
        let x = RS + 0x7100;
        let y = RS + 0x7200;
        let streamer_ref = RS + 0x7300;
        let streamer = RS + 0x8000;
        let trans = RS + 0x7400;
        let trans_vt = RS + 0x7500;
        c.bus.write32(GAME_GP - 0x848, RS_WORLD);
        c.bus.write32(RS_WORLD + 0x84, game);
        c.bus.write32(game + 0x84, x);
        c.bus.write32(x + 4, y);
        c.bus.write32(y + 0xb0, r.next_u32() % 2);
        c.bus.write32(game + 0x10, streamer_ref);
        c.bus.write32(streamer_ref, streamer);
        c.bus.write32(game + 0xc, RS_AI);
        c.bus.write32(game + 0x5c, trans);
        c.bus.write32(trans + 0x1c, trans_vt);
        // The transition manager's "active" query: a leaf
        // (`GameStateReset_v08`: its argument's `+0x10 == 5`).
        let off = (r.next_u32() % 8) * 4;
        c.bus.write16(trans_vt + 0x40, off as u16);
        c.bus.write32(trans_vt + 0x44, 0x0023_55b0);
        c.bus.write32(
            trans + 0x10 + off + 0x10,
            if r.next_u32() % 3 == 0 { 5 } else { 0 },
        );
        c.bus.write32(trans + 0x44, r.next_u32() % 2);
        let k = v % 4;
        rs_ai(c, k == 0, false, step == 1 && k == 1);
        // The start marker's rider (AI +0x40 -> +0x18).
        c.bus.write32(RS_AI + 0x40, RS_OBJ);
        // The AI rider count. Creating computer riders is a whole subsystem:
        // with riders to create, unwind at `AI_CreateComputerRiders`' first
        // interface call (fake 1).
        let riders = r.next_u32() % 4;
        c.bus.write32(0x0053_5c04, riders);
        if step == 0 && riders > 0 {
            c.bus.write32(LOG_UNWIND_AT, 1);
        }
        // The view clearance the streamer reports (its view is idle, so the
        // stack word stays as written): below or above the threshold 0.
        let clear: f32 = if r.next_u32() % 3 == 0 { -1.0 } else { 1.0 };
        c.bus.write32(SP - 0x70 + 0x10, clear.to_bits());
        c.bus.write32(GAME_GP - 0x516c, 0);
    }

    #[test]
    fn game_state_restart_race_update_matches() {
        check(
            ee::s06::f_00235cc8__generated,
            game_state_restart_race_update,
            5 * 48,
            &|c, seed| restart_race_setup(c, seed),
        );
    }

    /// The restart's 120-frame minimum (step 2) lasts 2 s at any rate.
    #[test]
    fn game_state_restart_race_update_minimum_follows_rate() {
        for (hz, ticks) in [(60, 120), (120, 240), (240, 480)] {
            let mut c = crate::difftest::fresh();
            c.hooks.miss = fake_call;
            c.bus.write32(LOG_UNWIND_AT, u32::MAX);
            crate::tick::set_for_test(&mut c, hz);
            restart_race_rate_fixture(&mut c);
            assert_eq!(
                rs_ticks(&mut c, game_state_restart_race_update, 0x0023_5cc8, 3),
                ticks,
                "{hz} Hz"
            );
        }
    }

    /// Step 2 with the skip armed, a clear view, list 0 idle, no active
    /// list or transition: only the update count holds it.
    fn restart_race_rate_fixture(c: &mut Cpu) {
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(RS + 0x10, 2);
        c.bus.write32(RS + 0x14, 0);
        c.bus.write32(RS + 0x18, 1);
        rs_engine(c, false);
        c.bus.write32(RS_LISTS + 0xa4, 0);
        let game = RS + 0x7000;
        let x = RS + 0x7100;
        let streamer_ref = RS + 0x7300;
        let streamer = RS + 0x8000;
        let trans = RS + 0x7400;
        let trans_vt = RS + 0x7500;
        c.bus.write32(GAME_GP - 0x848, RS_WORLD);
        c.bus.write32(RS_WORLD + 0x84, game);
        c.bus.write32(game + 0x84, x);
        c.bus.write32(x + 4, RS + 0x7200);
        c.bus.write32(game + 0x10, streamer_ref);
        c.bus.write32(streamer_ref, streamer);
        c.bus.write32(game + 0xc, RS_AI);
        c.bus.write32(game + 0x5c, trans);
        c.bus.write32(trans + 0x1c, trans_vt);
        // The transition manager's "active" query: a leaf returning 0
        // (`GameStateReset_v08` on a state whose `+0x10` is not 5).
        c.bus.write32(trans_vt + 0x44, 0x0023_55b0);
        c.bus.write32(trans + 0x44, 0);
        // The streamer view is idle: the clearance word on the stack stays
        // as written (1.0 above the 0 threshold).
        c.bus.write32(SP - 0x70 + 0x10, 1.0f32.to_bits());
    }

    /// `GameStateEventSelect_SaveState`: the base save (real
    /// `GameState_SaveState`) and the four field writes all go to a fake
    /// stream write method (logged, unwinding at a random call).
    #[test]
    fn game_state_event_select_save_state_matches() {
        use crate::difftest::fakes::{covered, FAKE};
        const ME: u32 = DATA;
        const STREAM: u32 = DATA + 0x100;
        const VT: u32 = DATA + 0x200;
        covered(
            ee::s06::f_00236728__generated,
            game_state_event_select_save_state,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x23_6728, 5);
                c.pc = 0x0023_6728;
                c.bus.write32(STREAM, VT);
                c.bus.write16(VT + 8, (r.next_u32() % 0x20) as u16 & !3);
                c.bus.write32(VT + 0xc, FAKE);
                c.g[A0][0] = if seed % 4 == 0 {
                    (r.next_u32() as u64) << 32 | ME as u64
                } else {
                    sx(ME)
                };
                c.g[A1][0] = sx(STREAM);
            },
        );
    }

    /// GameStateRestartRace_SaveState: the stream's write method (and the
    /// base save's) is a fake that may unwind at any of the four calls;
    /// `self` sometimes carries upper-half garbage.
    #[test]
    fn game_state_restart_race_save_state_matches() {
        use crate::difftest::fakes::{covered, machine, FAKE};
        const ME: u32 = DATA;
        const STREAM: u32 = DATA + 0x100;
        const VT: u32 = DATA + 0x200;
        covered(
            ee::s06::f_00235f48__generated,
            game_state_restart_race_save_state,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x23_5f48, 4);
                c.pc = 0x0023_5f48;
                c.bus.write32(STREAM, VT);
                c.bus.write16(VT + 8, (r.next_u32() % 0x20) as u16 & !3);
                c.bus.write32(VT + 0xc, FAKE);
                c.g[A0][0] = if seed % 4 == 0 {
                    (r.next_u32() as u64) << 32 | ME as u64
                } else {
                    sx(ME)
                };
                c.g[A1][0] = sx(STREAM);
            },
        );
    }

    /// The base `GameState_SaveState` (real) and the three word writes go to
    /// a fake `Write` method (random `this` delta); some seeds unwind at any
    /// of the four calls.
    #[test]
    fn game_state_reset_save_state_matches() {
        use crate::difftest::fakes::{covered, FAKE};
        covered(
            ee::s06::f_00235990__generated,
            game_state_reset_save_state,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x23_5990, 4);
                let state = DATA + 0x100;
                let stream = DATA + 0x200;
                let vt = DATA + 0x300;
                crate::difftest::fill(c, &mut r, state, 8);
                c.bus.write32(stream, vt);
                let delta = (r.next_u32() as i16 as i32 & !3) as u32;
                c.bus.write16(vt + OC300_STREAM_WRITE, delta as u16);
                c.bus.write32(vt + OC300_STREAM_WRITE + 4, FAKE);
                c.g[A0][0] = sx(state);
                c.g[A1][0] = sx(stream);
                c.pc = 0x0023_5990;
            },
        );
    }

    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;

    use crate::difftest::RA as SENTINEL;

    // -- GameState_*_SaveState wrappers --------------------------------------

    /// A serialiser whose `Save(size)` slot (vtable +8) is a fake; the
    /// wrapper's call to `GameState_SaveState` passes it through.
    pub(crate) fn oc325_save_state_setup(c: &mut Cpu, seed: u32, entry: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 1);
        c.pc = entry;
        let serialiser = DATA + 0x20_0000 + (r.next_u32() & 0xfff0);
        let vt = vtable(c, &mut r, &[0x8]);
        c.bus.write32(serialiser, vt);
        c.g[A0][0] = sx(ptr(&mut r));
        c.g[A1][0] = sx(serialiser);
        c.g[RA][0] = SENTINEL as u64;
    }

    #[test]
    fn game_state_finish_cinematic_save_state_matches() {
        covered(
            ee::s06::f_00236fa8__generated,
            game_state_finish_cinematic_save_state,
            100,
            &|c, seed| oc325_save_state_setup(c, seed, 0x0023_6fa8, 0x236f_a8),
        );
    }

    #[test]
    fn game_state_results_save_state_matches() {
        covered(
            ee::s06::f_00236ec8__generated,
            game_state_results_save_state,
            100,
            &|c, seed| oc325_save_state_setup(c, seed, 0x0023_6ec8, 0x236e_c8),
        );
    }

    #[test]
    fn game_state_pre_race_menu_save_state_matches() {
        covered(
            ee::s06::f_00236d60__generated,
            game_state_pre_race_menu_save_state,
            100,
            &|c, seed| oc325_save_state_setup(c, seed, 0x0023_6d60, 0x236d_60),
        );
    }

    #[test]
    fn game_state_reset_ai_save_state_matches() {
        covered(
            ee::s06::f_00236b70__generated,
            game_state_reset_ai_save_state,
            100,
            &|c, seed| oc325_save_state_setup(c, seed, 0x0023_6b70, 0x236b_70),
        );
    }

    // ------------------------------------------------- game state wrappers

    /// A game state at `DATA` and a serialiser at `DATA + 0x100` whose write
    /// method (vtable slot `+0x8`/`+0xc`) is a fake that may unwind; `a0` is
    /// the state (sometimes with upper-half garbage), `a1` the serialiser.
    pub(crate) fn oc326_state_fixture(c: &mut Cpu, seed: u32, salt: u64, entry: u32) -> Rng {
        let mut r = machine(c, seed, salt, 1);
        c.pc = entry;
        const THIS: u32 = DATA;
        const STREAM: u32 = DATA + 0x100;
        let vt = vtable(c, &mut r, &[0x8]);
        c.bus.write32(STREAM, vt);
        c.bus.write32(THIS, r.next_u32());
        c.g[A0][0] = if seed % 4 == 0 {
            (r.next_u32() as u64) << 32 | THIS as u64
        } else {
            sx(THIS)
        };
        c.g[A1][0] = sx(STREAM);
        r
    }

    /// GameStateEventSwitchLoad_SaveState: the base state's save, through a
    /// serialiser whose write method is a fake (some of them unwind).
    #[test]
    fn game_state_event_switch_load_save_state_matches() {
        covered(
            ee::s06::f_00236af8__generated,
            game_state_event_switch_load_save_state,
            200,
            &|c, seed| {
                oc326_state_fixture(c, seed, 0x23_6af8, 0x0023_6af8);
            },
        );
    }

    /// GameStateWarpToEvent_SaveState: as above.
    #[test]
    fn game_state_warp_to_event_save_state_matches() {
        covered(
            ee::s06::f_002361c8__generated,
            game_state_warp_to_event_save_state,
            200,
            &|c, seed| {
                oc326_state_fixture(c, seed, 0x23_61c8, 0x0023_61c8);
            },
        );
    }

    /// GameStateRace_SaveState: as above.
    #[test]
    fn game_state_race_save_state_matches() {
        covered(
            ee::s06::f_00234eb8__generated,
            game_state_race_save_state,
            200,
            &|c, seed| {
                oc326_state_fixture(c, seed, 0x23_4eb8, 0x0023_4eb8);
            },
        );
    }

    /// GameStateCountdown_SaveState: as above.
    #[test]
    fn game_state_countdown_save_state_matches() {
        covered(
            ee::s06::f_00234de0__generated,
            game_state_countdown_save_state,
            200,
            &|c, seed| {
                oc326_state_fixture(c, seed, 0x23_4de0, 0x0023_4de0);
            },
        );
    }

    use crate::difftest::fakes;
    pub(crate) use crate::game::ai::ai::tests::oc432_fake;
    pub(crate) use crate::game::ai::ai::tests::OC432_CTRL;
    pub(crate) use crate::game::ai::ai::tests::OC432_MAGIC;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc433_force;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc433_gp;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc433_run;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::oc433_w;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::OC433_FIX;

    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    oc432_fake!(
        fake_script_list_stop,
        PREV_SCRIPT_LIST_STOP,
        ee::s07::f_00276868__generated,
        0x0027_6868
    );

    oc432_fake!(
        fake_get_track_info,
        PREV_GET_TRACK_INFO,
        ee::s01::f_00144bc0__generated,
        0x0014_4bc0,
        |c: &mut Cpu| {
            c.g[V0][0] = (DATA + 0x5_9000) as u64;
        }
    );

    oc432_fake!(
        fake_bxstring_ctor_cstr,
        PREV_BXSTRING_CTOR_CSTR,
        ee::s10::f_003182d8__generated,
        0x0031_82d8
    );

    oc432_fake!(
        fake_bxstring_plus_cstr_str,
        PREV_BXSTRING_PLUS_CSTR_STR,
        ee::s10::f_003186d0__generated,
        0x0031_86d0
    );

    oc432_fake!(
        fake_bxstring_plus_str_cstr,
        PREV_BXSTRING_PLUS_STR_CSTR,
        ee::s10::f_00318630__generated,
        0x0031_8630
    );

    oc432_fake!(
        fake_bxstring_dtor,
        PREV_BXSTRING_DTOR,
        ee::s10::f_00318260__generated,
        0x0031_8260
    );

    oc432_fake!(
        fake_bxstring_assign,
        PREV_BXSTRING_ASSIGN,
        ee::s10::f_00318448__generated,
        0x0031_8448
    );

    oc432_fake!(
        fake_script_load_bank_async,
        PREV_SCRIPT_LOAD_BANK_ASYNC,
        ee::s07::f_0027b370__generated,
        0x0027_b370
    );

    oc432_fake!(
        fake_eamutex_lock,
        PREV_EAMUTEX_LOCK,
        ee::s14::f_003e5700__generated,
        0x003e_5700
    );

    oc432_fake!(
        fake_eamutex_unlock,
        PREV_EAMUTEX_UNLOCK,
        ee::s14::f_003e5760__generated,
        0x003e_5760
    );

    oc432_fake!(
        fake_async_file_assign_serial,
        PREV_ASYNC_FILE_ASSIGN_SERIAL,
        ee::s14::f_003def40__generated,
        0x003d_ef40
    );

    oc432_fake!(
        fake_eafile_queue_open,
        PREV_EAFILE_QUEUE_OPEN,
        ee::s14::f_003dd4e8__generated,
        0x003d_d4e8,
        |c: &mut Cpu| {
            if c.bus.read32(OC432_CTRL + 0x24) != 0 {
                c.g[V0][0] = 0;
            }
        }
    );

    oc432_fake!(
        fake_eafile_set_request_callback,
        PREV_EAFILE_SET_REQUEST_CALLBACK,
        ee::s14::f_003dcf70__generated,
        0x003d_cf70
    );

    oc432_fake!(
        fake_replay_reset,
        PREV_REPLAY_RESET,
        ee::s07::f_0026f228__generated,
        0x0026_f228
    );

    oc432_fake!(
        fake_script_unload_world,
        PREV_SCRIPT_UNLOAD_WORLD,
        ee::s07::f_00278cd8__generated,
        0x0027_8cd8
    );

    oc432_fake!(
        fake_ai_reset_computer_slots,
        PREV_AI_RESET_COMPUTER_SLOTS,
        ee::s00::f_0012ab20__generated,
        0x0012_ab20
    );

    oc432_fake!(
        fake_ai_reset_actor_slots,
        PREV_AI_RESET_ACTOR_SLOTS,
        ee::s00::f_0012ac48__generated,
        0x0012_ac48
    );

    oc432_fake!(
        fake_script_queue_intro,
        PREV_SCRIPT_QUEUE_INTRO,
        ee::s07::f_0027aaf8__generated,
        0x0027_aaf8
    );

    oc432_fake!(
        fake_char_profile_flag_clear,
        PREV_CHAR_PROFILE_FLAG_CLEAR,
        ee::s01::f_00145d38__generated,
        0x0014_5d38,
        |c: &mut Cpu| {
            c.g[V0][0] = c.bus.read32(OC432_CTRL + 0x30) as u64;
        }
    );

    oc432_fake!(
        fake_audio_instance,
        PREV_AUDIO_INSTANCE,
        ee::s07::f_0028b180__generated,
        0x0028_b180,
        |c: &mut Cpu| {
            c.g[V0][0] = (DATA + 0x5_c000) as u64;
        }
    );

    /// `GameStateReset_OverlayEnter`: the world-script reload runs real on
    /// the `LoadWorldScripts` fixture (its callees are fakes), the NIS list
    /// reset runs real with `ScriptList_Stop` faked; network path on/off,
    /// location 10..17, first-time-on-track flag on/off.
    #[test]
    fn game_state_reset_overlay_enter_matches() {
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026F228),
            fake_replay_reset,
            &PREV_REPLAY_RESET,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00278CD8),
            fake_script_unload_world,
            &PREV_SCRIPT_UNLOAD_WORLD,
        );
        fakes::hook(
            addr_of_mut!(ee::s00::OVERRIDE_0012AB20),
            fake_ai_reset_computer_slots,
            &PREV_AI_RESET_COMPUTER_SLOTS,
        );
        fakes::hook(
            addr_of_mut!(ee::s00::OVERRIDE_0012AC48),
            fake_ai_reset_actor_slots,
            &PREV_AI_RESET_ACTOR_SLOTS,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0027AAF8),
            fake_script_queue_intro,
            &PREV_SCRIPT_QUEUE_INTRO,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00145D38),
            fake_char_profile_flag_clear,
            &PREV_CHAR_PROFILE_FLAG_CLEAR,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0028B180),
            fake_audio_instance,
            &PREV_AUDIO_INSTANCE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00276868),
            fake_script_list_stop,
            &PREV_SCRIPT_LIST_STOP,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            fake_get_be,
            &PREV_GET_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            fake_be_get_interface,
            &PREV_BE_GET_INTERFACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00144BC0),
            fake_get_track_info,
            &PREV_GET_TRACK_INFO,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003182D8),
            fake_bxstring_ctor_cstr,
            &PREV_BXSTRING_CTOR_CSTR,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_003186D0),
            fake_bxstring_plus_cstr_str,
            &PREV_BXSTRING_PLUS_CSTR_STR,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318630),
            fake_bxstring_plus_str_cstr,
            &PREV_BXSTRING_PLUS_STR_CSTR,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318260),
            fake_bxstring_dtor,
            &PREV_BXSTRING_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00318448),
            fake_bxstring_assign,
            &PREV_BXSTRING_ASSIGN,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0027B370),
            fake_script_load_bank_async,
            &PREV_SCRIPT_LOAD_BANK_ASYNC,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E5700),
            fake_eamutex_lock,
            &PREV_EAMUTEX_LOCK,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E5760),
            fake_eamutex_unlock,
            &PREV_EAMUTEX_UNLOCK,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DEF40),
            fake_async_file_assign_serial,
            &PREV_ASYNC_FILE_ASSIGN_SERIAL,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DD4E8),
            fake_eafile_queue_open,
            &PREV_EAFILE_QUEUE_OPEN,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003DCF70),
            fake_eafile_set_request_callback,
            &PREV_EAFILE_SET_REQUEST_CALLBACK,
        );
        fakes::covered(
            ee::s06::f_00234f40__generated,
            game_state_reset_overlay_enter,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x234f40, 40);
                c.bus.write32(OC432_CTRL, OC432_MAGIC);
                c.bus
                    .write32(OC432_CTRL + 0x24, (r.next_u32() % 3 == 0) as u32);
                c.bus.write32(OC432_CTRL + 0x30, (r.next_u32() % 2) as u32);
                let gp = crate::difftest::GAME_GP;
                let this = DATA + 0x5_0000;
                let engine = DATA + 0x5_1000;
                let game = DATA + 0x5_2000;
                let world = DATA + 0x5_3000;
                let x = DATA + 0x5_4000;
                let section_man = DATA + 0x5_5000;
                let ai = DATA + 0x5_6000;
                let b = DATA + 0x5_7000;
                let cc = DATA + 0x5_7100;
                let d = DATA + 0x5_7200;
                let vt = DATA + 0x5_7300;
                c.g[A0][0] = this as u64;
                c.bus.write32(gp - 0x488, game);
                c.bus.write32(gp - 0x84c, engine);
                c.bus.write32(gp - 0x848, world);
                c.bus.write32(game + 0x28, DATA + 0x5_7400);
                c.bus.write32(game + 0xc, ai);
                c.bus.write32(world + 0x84, x);
                c.bus.write32(x + 0x78, section_man);
                c.bus.write32(section_man + 0x1bc, 10 + r.next_u32() % 8);
                c.bus.write32(ai + 0x40, b);
                c.bus.write32(b + 0x18, cc);
                c.bus.write32(cc + 0x6c0, d);
                c.bus.write16(d + 0x38, (r.next_u32() % 32) as u16);
                c.bus.write32(d + 0x3c, fakes::FAKE);
                let _ = vt;
                c.bus.write8(0x0053_5c11, (r.next_u32() % 3 == 0) as u8);
                // NIS list reset: a non-zero +0x550 skips ExitScriptMode.
                c.bus.write32(engine + 0x550, 1);
                c.bus.write32(engine + 0x620, r.next_u32() % 4);
                let node = DATA + 0x5_a000;
                c.bus
                    .write32(0x0051_9bf8, if r.next_u32() % 3 == 0 { 0 } else { node });
                c.bus.write32(node + 4, 0);
            },
        );
    }

    /// `GameStateReset_Exit` against the generated body: mode byte 0, 5, 6, 7,
    /// the "already reset" byte set or not, and the single-player check (the
    /// first rider's word cleared only when the world's slot holds 1).
    #[test]
    fn game_state_reset_exit_matches() {
        oc433_run(
            ee::s06::f_00235868__generated,
            game_state_reset_exit,
            0x0023_5868,
            12,
            120,
            &|c, r, seed| {
                let f = OC433_FIX;
                let (app, x, a3, a0, a1, a0b, v) = (
                    f + 0x1000,
                    f + 0x1100,
                    f + 0x1300,
                    f + 0x1400,
                    f + 0x1500,
                    f + 0x1600,
                    f + 0x1700,
                );
                oc433_gp(c, -0x848, app);
                oc433_gp(c, -0x488, x);
                oc433_gp(c, 0xce8, f + 0x1800);
                fill(c, r, x + 0x28, 1);
                oc433_w(c, app + 0xc0, f + 0x1900);
                oc433_w(c, app + 0x84, a3);
                oc433_w(c, a3 + 0x84, a0);
                oc433_w(c, a3 + 0x10, a0b);
                oc433_w(c, a0 + 0x10, [1u32, 1, 0][(seed / 4 % 3) as usize]);
                oc433_w(c, a0 + 4, a1);
                oc433_w(c, a1 + 0xb0, (seed / 12) % 3);
                oc433_w(c, a0b, v);
                c.sb(0x0053_5bc8 + 0x49, (seed % 2) as u64);
                c.sb(0x0053_5bc8 + 0x48, [0u64, 5, 6, 7][(seed / 2 % 4) as usize]);
                c.g[A0][0] = sx(f + 0x2000);
            },
        );
    }

    /// `GameStateCountdown_Enter` against the generated body: the race-start
    /// notification object set or not, the scripts-reset flag, and the mode
    /// byte (4..6: go to the next state; others: the countdown audio and
    /// script event, with the replay check forced to "replaying").
    #[test]
    fn game_state_countdown_enter_matches() {
        oc433_run(
            ee::s06::f_00234ad0__generated,
            game_state_countdown_enter,
            0x0023_4ad0,
            17,
            120,
            &|c, r, seed| {
                let f = OC433_FIX;
                let (x, y, z, comm, app) =
                    (f + 0x1000, f + 0x1100, f + 0x1200, f + 0x1300, f + 0x1400);
                oc433_gp(c, -0x488, x);
                oc433_w(c, x + 0xc, y);
                fill(c, r, x + 0x28, 1);
                oc433_w(c, x + 0x48, z);
                oc433_gp(c, -0x204, if seed % 2 == 0 { comm } else { 0 });
                oc433_w(c, comm + 0x70, r.next_u32() % 30);
                oc433_gp(c, -0x848, app);
                fill(c, r, app + 0x78, 1);
                oc433_w(c, 0x0053_4b30, (seed / 2) % 2);
                c.sb(0x0053_5c10, [0u64, 4, 5, 6, 7][(seed / 4 % 5) as usize]);
                oc433_force(c, 0x0028_8ae0, &[1]);
                c.g[A0][0] = sx(f + 0x2000);
            },
        );
    }

    pub(crate) use crate::game::fe::festateloadscreen::tests::oc444_run;

    pub(crate) use crate::game::fe::festateloadscreen::tests::OC444_NEWRACE;
    pub(crate) use crate::game::fe::festateloadscreen::tests::OC444_OBJ;

    /// `GameStateResults_Enter(self)`.
    #[test]
    fn game_state_results_enter_matches() {
        oc444_run(
            ee::s06::f_00236da0__generated,
            game_state_results_enter,
            0x0023_6da0,
            0x23_6da0,
            120,
            &|c, r, _seed| {
                c.g[A0][0] = OC444_OBJ as u64;
                let n = r.next_u32() % 4;
                c.bus.write32(OC444_NEWRACE + 0x30, n);
            },
        );
    }

    use ssx3_ee_rt::Cpu;

    pub(crate) use crate::game::main::gamemode::tests::oc520_w;

    pub(crate) const SEEDS_6: u32 = 300;

    /// The `*_LoadState` fixture: `a1` is the state, whose vtable's slot
    /// (`lh` at +0x10, function at +0x14) is a fake. `c.pc` is the entry.
    pub(crate) fn load_state_setup(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        let mut r = machine(c, seed, salt, 1);
        let state = ptr(&mut r);
        c.g[A1][0] = state as u64;
        let vt = vtable(c, &mut r, &[0x10]);
        c.bus.write32(state, vt);
        c.pc = entry;
    }

    #[test]
    fn game_state_load_state_wrappers_match() {
        let cases: [(fn(&mut Cpu), fn(&mut Cpu), u32); 8] = [
            (
                ssx3_ee::s06::f_00234e00__generated,
                super::game_state_countdown_load_state,
                0x0023_4e00,
            ),
            (
                ssx3_ee::s06::f_00234ed8__generated,
                super::game_state_race_load_state,
                0x0023_4ed8,
            ),
            (
                ssx3_ee::s06::f_002361e8__generated,
                super::game_state_warp_to_event_load_state,
                0x0023_61e8,
            ),
            (
                ssx3_ee::s06::f_00236b18__generated,
                super::game_state_event_switch_load_load_state,
                0x0023_6b18,
            ),
            (
                ssx3_ee::s06::f_00236b90__generated,
                super::game_state_reset_ai_load_state,
                0x0023_6b90,
            ),
            (
                ssx3_ee::s06::f_00236d80__generated,
                super::game_state_pre_race_menu_load_state,
                0x0023_6d80,
            ),
            (
                ssx3_ee::s06::f_00236ee8__generated,
                super::game_state_results_load_state,
                0x0023_6ee8,
            ),
            (
                ssx3_ee::s06::f_00236fc8__generated,
                super::game_state_finish_cinematic_load_state,
                0x0023_6fc8,
            ),
        ];
        for (salt, (gen, hand, entry)) in cases.into_iter().enumerate() {
            covered(gen, hand, SEEDS_6, &|c, seed| {
                load_state_setup(c, seed, 0x516 + salt as u64, entry)
            });
        }
    }

    /// The object `InputSync` (self), the GameComm, the SSXApp world and the
    /// comm-system input-delay table, and the sample buffers.
    pub(crate) const SELF: u32 = DATA + 0x2000;

    pub(crate) const GC: u32 = DATA + 0x3000;

    pub(crate) const SAMPLES: u32 = DATA + 0x4000;

    pub(crate) const WORLD: u32 = DATA + 0x5000;

    pub(crate) const DELAY_TABLE: u32 = DATA + 0x6000;

    pub(crate) const DELAY_OBJ: u32 = DATA + 0x6100;

    pub(crate) const DELAY_VT: u32 = DATA + 0x6200;

    pub(crate) const LOCAL_PADS: u32 = DATA + 0x7000;

    /// Both sides from the same state. Remote seeds keep `writepos` far enough
    /// ahead that the network `while` loop never runs (its body calls
    /// `cGameComm_PopPacket`, which is not faked here); the local seeds
    /// keep `mSyncInput` at 0 so `SendSample` does not send.
    #[test]
    fn input_sync_update_matches() {
        check(
            ee::s06::f_002373b8__generated,
            input_sync_update,
            200,
            &|c, seed| {
                let _r = machine(c, seed, 0x2373_b8, 0);
                c.pc = UPDATE_ENTRY;
                c.g[A0][0] = SELF as u64;
                let local = seed % 3 == 0;
                // The network branch: mSyncInput, no error, connected (see below).
                let net = !local && seed % 4 != 1 && seed % 4 != 2 && seed % 5 != 3;
                c.bus.write32(GAME_GP.wrapping_sub(0x204), GC);
                c.bus
                    .write32(GC + 0x70, if seed % 7 == 0 { 10 } else { 14 + seed % 2 });
                c.bus
                    .write32(GC + 0x40, if local || seed % 4 == 1 { 0 } else { 1 });
                c.bus.write32(GC + 4, if seed % 4 == 2 { 1 } else { 0 });
                c.bus.write32(GC, if seed % 5 == 3 { 0 } else { 1 });
                c.bus.write32(GC + 0x6c, 0);
                c.bus
                    .write32(GC + 0x120, ((seed % 9) as f32 + 0.5).to_bits());
                c.bus.write32(GAME_GP.wrapping_sub(0x238), DELAY_TABLE);
                c.bus.write32(DELAY_TABLE, DELAY_OBJ);
                c.bus.write32(DELAY_OBJ + 8, DELAY_VT);
                c.bus.write32(DELAY_VT + 0x2c, FAKE);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), WORLD);
                c.bus.write32(WORLD + 0x14, (1.0f32 / 60.0).to_bits());

                let count = (seed / 3 % 4) as u32;
                let latency = 4 + seed % 5;
                let numsamples = 1 + seed % 3;
                c.bus.write32(SELF, seed % 9);
                c.bus.write32(SELF + 4, latency);
                c.bus.write32(SELF + 8, numsamples);
                c.bus.write32(SELF + 0xc, seed.wrapping_mul(7));
                c.bus.write32(SELF + 0x10, count);
                for i in 0..count {
                    let pad = SELF + 0x14 + 0x18 * i;
                    let buffer = SAMPLES + 0x200 * i;
                    c.bus.write32(pad, buffer);
                    if local {
                        let read = (seed + i) % 3;
                        c.bus.write32(pad + 4, read);
                        c.bus.write32(pad + 8, read + (seed >> i) % 7);
                        c.bus.write32(pad + 0xc, LOCAL_PADS + 0x100 * i);
                        let source = LOCAL_PADS + 0x100 * i;
                        c.bus.write32(source, (seed + i) % 4);
                        for k in 0..4 {
                            c.bus
                                .write32(source + 4 + 4 * k, ((seed + k) as f32 * 0.25).to_bits());
                        }
                    } else {
                        c.bus.write32(pad + 4, 0);
                        c.bus
                            .write32(pad + 8, if net { 16 } else { (seed >> i) % 7 });
                        c.bus.write32(pad + 0xc, 0);
                    }
                }
            },
        );
    }

    #[test]
    fn input_sync_add_remote_pad_matches() {
        check(
            ssx3_ee::s06::f_00237280__generated,
            input_sync_add_remote_pad,
            120,
            &|c, seed| {
                let _ = machine(c, seed, 0x237280, 0);
                crate::game::be::beintstat::tests::allocator(c);
                c.pc = 0x0023_7280;
                let obj = DATA + 0x100;
                // latency -1..=3 (0 and below skip the sample loop), 0..=3 pads.
                oc520_w(c, obj, seed * 7 + 3);
                oc520_w(c, obj + 0x4, ((seed % 5) as i32 - 1) as u32);
                oc520_w(c, obj + 0x10, seed % 4);
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
            },
        );
    }

    #[test]
    fn input_sync_empty_hook_matches() {
        check(
            ssx3_ee::s06::f_00237948__generated,
            input_sync_empty_hook,
            8,
            &|c, seed| {
                let _ = machine(c, seed, 0x237948, 0);
                c.pc = 0x0023_7948;
            },
        );
    }

    use crate::game::ai::computer::tests::rbc_test_allocator;

    /// The record table of `input_sync_add_local_pad` against the generated
    /// body: the count 0..3 (the zero count skips the fill loop), a record
    /// index 0..2, the allocator pools and the real `ValuePair` setters.
    #[test]
    fn input_sync_add_local_pad_matches() {
        check(
            ee::s06::f_00237140__generated,
            input_sync_add_local_pad,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0023_7140, 0);
                rbc_test_allocator(c);
                let this = DATA + 0x1000;
                let src = DATA + 0x3000;
                let count = [0u32, 1, 2, 3][seed as usize % 4];
                c.bus.write32(this, r.next_u32());
                c.bus.write32(this + 4, count);
                c.bus.write32(this + 0x10, (seed / 4) % 3);
                c.bus.write32(src, r.next_u32());
                c.bus.write32(src + 4, r.next_u32());
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(src);
            },
        );
    }

    use crate::difftest::fakes::fake;
    use crate::difftest::RA as DT_RA;

    use ssx3_ee_rt::funcs::register_functions;

    pub(crate) use crate::game::be::beintstat::tests::oc673_machine;
    pub(crate) use crate::game::be::beintstat::tests::oc673_result_at;

    /// `GameStateReset_Enter` against the generated body: the transport
    /// sequence flag, both race-setup branches (game path set or not, mode 4,
    /// 5, 6), the section id in 0xe..0x10, the NIS list branches, the audio
    /// re-enable, and unwinds in each call. Callees are fakes; the race
    /// interface object's vtable slot is a fake method.
    #[test]
    fn game_state_reset_enter_matches() {
        fakes::covered(
            ee::s06::f_002355c0__generated,
            game_state_reset_enter_3,
            300,
            &|c, seed| {
                let mut r = oc673_machine(c, seed, 0x2355c0, 26, 0x0023_55c0);
                let this = DATA + 0x8_1000;
                let app = DATA + 0x8_0000;
                let world = DATA + 0x8_2000;
                let x = DATA + 0x8_3000;
                let y = DATA + 0x8_4000;
                let m = DATA + 0x8_5000;
                let n = DATA + 0x8_5800;
                let iface = DATA + 0x8_6000;
                let obj = DATA + 0x8_6800;
                c.g[A0][0] = sx(this);
                c.bus.write32(this + 0x14, r.next_u32() % 3);
                c.bus.write32(GAME_GP - 0x84c, app);
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(GAME_GP - 0x488, m);
                c.bus.write32(GAME_GP + 0xce8, r.next_u32());
                c.bus
                    .write32(app + 0x550, [2, 2, 3][(r.next_u32() % 3) as usize]);
                c.bus.write32(world + 0x84, x);
                c.bus.write32(x + 0x78, y);
                c.bus.write32(x + 0x94, r.next_u32());
                c.bus.write32(
                    y + 0x1bc,
                    [0xd, 0xe, 0xf, 0x10, 0x20][(r.next_u32() % 5) as usize],
                );
                c.bus.write32(m + 0xc, n);
                c.bus.write32(m + 0x28, r.next_u32());
                c.bus.write8(0x0053_5bc8 + 0x49, (r.next_u32() % 2) as u8);
                c.bus.write8(0x0053_5bc8 + 0x48, (r.next_u32() % 7) as u8);
                c.bus.write32(iface + 0xc, obj);
                c.bus.write16(obj + 8, ((r.next_u32() % 64) * 4) as u16);
                c.bus.write32(obj + 0xc, FAKE);
                oc673_result_at(c, 0x0014_dd58, iface);
                oc673_result_at(c, 0x0027_aa80, r.next_u32() % 2);
                oc673_result_at(c, 0x0014_4cc0, [1, 1, 0, 2][(r.next_u32() % 4) as usize]);
            },
        );
    }

    /// Environment switch of the child process (see [`oc682_child`]).
    pub(crate) const OC682_CHILD_ENV: &str = "SSX3_OC682_CHILD";

    /// Seeds per test.
    pub(crate) const OC682_SEEDS: u32 = 160;

    /// Controls the fakes read: the "unknown indirect call returns 0" flag
    /// (+4) and the forced-result table (see [`oc682_force`]).
    pub(crate) const OC682_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC682_FORCE: u32 = OC682_CTRL + 0x100;

    pub(crate) const OC682_FORCE_SLOTS: u32 = 12;

    /// Objects of the fixtures (outside the pointer soup, filled like it).
    pub(crate) const OC682_OBJ: u32 = DATA + 0x5_0000;

    /// The pointer soup: 32-bit words that are pointers into the soup itself,
    /// zero, or small values the state machines compare against.
    pub(crate) const OC682_SOUP_WORDS: u32 = 0x1_0000;

    pub(crate) const OC682_SMALL: [u32; 16] = [
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        8,
        9,
        10,
        12,
        13,
        14,
        17,
        0x16,
        0xffff_ffff,
    ];

    /// A direct callee (a slot of an installed function) replaced by a logging
    /// fake with the forced-result table (child process only).
    macro_rules! oc682_slot_fake {
        ($fake:ident, $addr:expr) => {
            fn $fake(c: &mut Cpu) {
                let ra = c.g[RA][0] as u32;
                oc682_fake_at(c, $addr, ra)
            }
        };
    }

    oc682_slot_fake!(oc682_fake_process_state_change, 0x0023_0f40);

    oc682_slot_fake!(oc682_fake_goto_next, 0x0023_3aa0);

    oc682_slot_fake!(oc682_fake_request_state, 0x0023_1250);

    oc682_slot_fake!(oc682_fake_get_transition_man, 0x0023_0698);

    oc682_slot_fake!(oc682_fake_rand_shuffle, 0x0023_c770);

    oc682_slot_fake!(oc682_fake_sort_by_time, 0x0023_8bf8);

    oc682_slot_fake!(oc682_fake_apply_input, 0x0023_79c8);

    oc682_slot_fake!(oc682_fake_is_ready, 0x0023_7950);

    oc682_slot_fake!(oc682_fake_check_pause_request, 0x0023_1840);

    oc682_slot_fake!(oc682_fake_unpack_sample, 0x0023_7be8);

    oc682_slot_fake!(oc682_fake_pack_sample, 0x0023_7b28);

    /// Child process only: an empty function table (every `call_indirect`
    /// reaches the fakes) and fakes in the slots of the installed functions
    /// the ports call directly. Ungated: the child runs this one test.
    pub(crate) fn oc682_install() {
        if std::env::var_os(OC682_CHILD_ENV).is_none() {
            return;
        }
        register_functions(&[]);
        // SAFETY: the child process runs this one test, single-threaded.
        unsafe {
            ee::s06::OVERRIDE_00230F40 = oc682_fake_process_state_change;
            ee::s06::OVERRIDE_00233AA0 = oc682_fake_goto_next;
            ee::s06::OVERRIDE_00231250 = oc682_fake_request_state;
            ee::s06::OVERRIDE_00230698 = oc682_fake_get_transition_man;
            ee::s06::OVERRIDE_0023C770 = oc682_fake_rand_shuffle;
            ee::s06::OVERRIDE_00238BF8 = oc682_fake_sort_by_time;
            ee::s06::OVERRIDE_002379C8 = oc682_fake_apply_input;
            ee::s06::OVERRIDE_00237950 = oc682_fake_is_ready;
            ee::s06::OVERRIDE_00231840 = oc682_fake_check_pause_request;
            ee::s06::OVERRIDE_00237BE8 = oc682_fake_unpack_sample;
            ee::s06::OVERRIDE_00237B28 = oc682_fake_pack_sample;
        }
    }

    /// Force the result of calls to `target`: call k returns `v1` if bit k of
    /// `mask` is set, else `v0`.
    pub(crate) fn oc682_force(c: &mut Cpu, slot: u32, target: u32, v0: u32, v1: u32, mask: u32) {
        let e = OC682_FORCE + slot * 20;
        c.bus.write32(e, target);
        c.bus.write32(e + 4, v0);
        c.bus.write32(e + 8, v1);
        c.bus.write32(e + 12, mask);
        c.bus.write32(e + 16, 0);
    }

    /// A logging fake call of `target` returning to `ra`, then the forced
    /// result (or, for unknown indirect targets, 0 half the time when the
    /// fixture asks for it).
    pub(crate) fn oc682_fake_at(c: &mut Cpu, target: u32, ra: u32) {
        fake(c, target, ra);
        if c.pc != ra {
            return;
        }
        for slot in 0..OC682_FORCE_SLOTS {
            let e = OC682_FORCE + slot * 20;
            if c.bus.read32(e) == target {
                let k = c.bus.read32(e + 16);
                c.bus.write32(e + 16, k + 1);
                let mask = c.bus.read32(e + 12);
                let v = if (mask >> (k & 31)) & 1 != 0 {
                    c.bus.read32(e + 8)
                } else {
                    c.bus.read32(e + 4)
                };
                c.g[V0][0] = sx(v);
                return;
            }
        }
        let indirect = target < 0x0010_0000 || target >= 0x0080_0000;
        if indirect && c.bus.read32(OC682_CTRL + 4) != 0 && target & 4 != 0 {
            c.g[V0][0] = 0;
        }
    }

    /// The `miss` hook: calls to addresses outside the (empty) table.
    pub(crate) fn oc682_miss(c: &mut Cpu, target: u32) {
        let ra = c.g[RA][0] as u32;
        oc682_fake_at(c, target, ra)
    }

    /// Run `test` in a child process (a private function table) unless this
    /// is that child. Returns true in the child.
    pub(crate) fn oc682_child(module: &str, test: &str) -> bool {
        if std::env::var_os(OC682_CHILD_ENV).is_some() {
            return true;
        }
        let name = format!("{}::{test}", module.split_once("::").map_or("", |p| p.1));
        let out = std::process::Command::new(std::env::current_exe().unwrap())
            .args([name.as_str(), "--exact", "--test-threads=1", "--nocapture"])
            .env(OC682_CHILD_ENV, "1")
            .output()
            .unwrap();
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(
            out.status.success() && text.contains("1 passed"),
            "child run of {name}:\n{text}"
        );
        false
    }

    /// Scrambled registers, the fakes, the pointer soup at `DATA`, the entry
    /// `pc` and the return address.
    pub(crate) fn oc682_base(c: &mut Cpu, seed: u32, salt: u64, calls: u32, entry: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        c.hooks.miss = oc682_miss;
        oc682_install();
        c.pc = entry;
        c.g[RA][0] = DT_RA as u64;
        // The soup, and the fixture objects (also soup until a test sets a field).
        for i in 0..OC682_SOUP_WORDS + 0x1400 {
            let x = r.next_u32();
            let v = match x % 6 {
                0 | 1 => DATA + ((x >> 8) % OC682_SOUP_WORDS) * 4,
                2 => 0,
                _ => OC682_SMALL[((x >> 8) % 16) as usize],
            };
            let at = if i < OC682_SOUP_WORDS {
                DATA + i * 4
            } else {
                OC682_OBJ + (i - OC682_SOUP_WORDS) * 4
            };
            c.bus.write32(at, v);
        }
        r
    }

    /// One of `xs`.
    pub(crate) fn oc682_pick(r: &mut Rng, xs: &[u32]) -> u32 {
        xs[(r.next_u32() as usize) % xs.len()]
    }

    /// A word at `gp + off`.
    pub(crate) fn oc682_gp(c: &mut Cpu, off: i32, v: u32) {
        c.bus.write32(GAME_GP.wrapping_add(off as u32), v);
    }

    /// `GameStateEventSelect_Enter` against the generated body: every mode
    /// (0..3, out of range), the next location equal to the current one (the
    /// race restart) or not, and unwinds at random calls. `GameModule_ResetRace`
    /// is the real generated body (its calls are fakes).
    #[test]
    fn game_state_event_select_enter_matches() {
        if !oc682_child(module_path!(), "game_state_event_select_enter_matches") {
            return;
        }
        covered(
            ee::s06::f_00236250__generated,
            game_state_event_select_enter,
            OC682_SEEDS,
            &|c, seed| {
                let mut r = oc682_base(c, seed, 0x23_6250, 14, 0x0023_6250);
                let mode = [0u32, 1, 2, 3, 4, 0xffff_ffff][(seed % 6) as usize];
                let location = oc682_pick(&mut r, &OC682_SMALL);
                oc682_gp(c, -0x1f34, location);
                // The next location is 0x17, or (mode 1) the new-race one.
                let expected = if mode == 1 { location } else { 0x17 };
                let at = OC682_OBJ + 0x1000;
                let equal = (seed / 6) % 2 == 0;
                c.bus
                    .write32(at, if equal { expected } else { expected ^ 4 });
                oc682_force(c, 0, 0x0014_4bc0, at, at, 0);
                c.g[A0][0] = OC682_OBJ as u64;
                c.g[A1][0] = sx(mode);
            },
        );
    }

    /// `GameStateEventSelect_Update` against the generated body: the states
    /// 0..2 and out-of-range ones, every mode, the NIS play state, overlay
    /// done flags, locations in and out of the back country range.
    #[test]
    fn game_state_event_select_update_matches() {
        if !oc682_child(module_path!(), "game_state_event_select_update_matches") {
            return;
        }
        covered(
            ee::s06::f_00236418__generated,
            game_state_event_select_update,
            OC682_SEEDS,
            &|c, seed| {
                let mut r = oc682_base(c, seed, 0x23_6418, 14, 0x0023_6418);
                let this = OC682_OBJ;
                let bits = r.next_u32();
                let state = [0u32, 1, 2, 3, 0xffff_ffff, 1, 0, 2][(seed % 8) as usize];
                c.bus.write32(this + 0x10, state);
                c.bus
                    .write32(this + 0x14, [0u32, 1, 2, 3][((seed / 8) % 4) as usize]);
                c.bus.write32(this + 0x8, bits & 1);
                c.bus.write32(this + 0x1c, (bits >> 1) & 1);
                c.bus.write32(
                    this + 0x18,
                    [0xdu32, 0xe, 0xf, 0x10, 0x11, 0x17][((bits >> 2) % 6) as usize],
                );
                oc682_gp(c, -0x69c, (bits >> 8) % 6);
                oc682_gp(c, -0x6a0, r.next_u32());
                oc682_gp(c, -0xb5c, r.next_u32());
                c.bus
                    .write8(0x0053_5c10, [0u8, 4, 2][((bits >> 12) % 3) as usize]);
                c.bus.write8(0x0053_5c11, ((bits >> 14) & 1) as u8);
                // The NIS list play state, the location and the NIS state.
                let play = oc682_pick(&mut r, &[4, 5, 2, 9, 0, 1]);
                oc682_force(c, 0, 0x0027_9220, play, 5, 0);
                let at = OC682_OBJ + 0x1000;
                let location = oc682_pick(&mut r, &[0xd, 0xe, 0xf, 0x10, 0x11, 0x17, 5]);
                c.bus.write32(at, location);
                oc682_force(c, 1, 0x0014_4bc0, at, at, 0);
                let nis = oc682_pick(&mut r, &[0xe, 0x14, 0xd, 0x13, 7]);
                oc682_force(c, 2, 0x0027_9148, nis, nis, 0);
                c.g[A0][0] = this as u64;
            },
        );
    }

    /// `GameStateEventSwitchLoad_Update` against the generated body: the
    /// states 0..2, a missing transport wait value, the stream focus distance
    /// on both sides of the limit, a busy or idle transition manager.

    /// `InputSync_dtor` against the generated body: 0..3 synced pads (and a
    /// negative count), with and without samples, `flags` bit 0 set or not.
    #[test]
    fn input_sync_dtor_matches() {
        if !oc682_child(module_path!(), "input_sync_dtor_matches") {
            return;
        }
        covered(
            ee::s06::f_002370a0__generated,
            input_sync_dtor_2,
            OC682_SEEDS,
            &|c, seed| {
                let mut r = oc682_base(c, seed, 0x23_70a0, 10, 0x0023_70a0);
                let this = OC682_OBJ;
                let n = [0u32, 1, 2, 3, 0xffff_ffff, 2][(seed % 6) as usize];
                c.bus.write32(this + 0x10, n);
                for i in 0..3 {
                    let entry = this + 0x14 + i * 0x18;
                    let samples = if r.next_u32() & 1 != 0 {
                        OC682_OBJ + 0x800
                    } else {
                        0
                    };
                    c.bus.write32(entry, samples);
                    c.bus.write32(entry + 0x10, OC682_OBJ + 0x900);
                    c.bus.write32(entry + 0x14, OC682_OBJ + 0xa00);
                }
                c.g[A0][0] = this as u64;
                c.g[A1][0] = ((seed / 6) % 4) as u64;
            },
        );
    }

    /// `InputSync_SendSample` against the generated body: the game comm
    /// synced/error/connected flags (the packet is sent only when all agree;
    /// the send itself runs the real `cGameComm_SendPacket` with an empty
    /// send queue), pack/unpack are fakes.
    #[test]
    fn input_sync_send_sample_matches() {
        if !oc682_child(module_path!(), "input_sync_send_sample_matches") {
            return;
        }
        covered(
            ee::s06::f_00237a98__generated,
            input_sync_send_sample_3,
            OC682_SEEDS,
            &|c, seed| {
                let mut r = oc682_base(c, seed, 0x23_7a98, 6, 0x0023_7a98);
                let comm = OC682_OBJ + 0x2000;
                oc682_gp(c, -0x204, comm);
                c.bus
                    .write32(comm + 0x40, [1u32, 0, 1, 1][(seed % 4) as usize]);
                c.bus
                    .write32(comm + 4, [0u32, 0, 1, 0][((seed / 4) % 4) as usize]);
                c.bus
                    .write32(comm, [5u32, 1, 0, 0xffff_ffff][((seed / 16) % 4) as usize]);
                c.bus.write32(comm + 0x364, 0);
                c.bus.write32(comm + 0x368, 0);
                c.g[A0][0] = (OC682_OBJ + 0x100) as u64;
                c.g[A1][0] = (OC682_OBJ + 0x200) as u64;
                let _ = r.next_u32();
            },
        );
    }

    pub(crate) use crate::game::sound::asyncsys::tests::oc738_install_fakes;
    pub(crate) use crate::game::sound::asyncsys::tests::oc738_machine;
    pub(crate) use crate::game::sound::asyncsys::tests::OC738_CTRL;

    pub(crate) const OC738_NOT_LOADED: u32 = OC738_CTRL + 8;

    pub(crate) const OC738_NIS_IDLE: u32 = OC738_CTRL + 0xc;

    /// `sp` inside the frame of the 0x80-byte functions called with the
    /// machine's stack.
    pub(crate) const SP_AFTER_FRAME: u32 = crate::difftest::SP - 0x80;

    /// `GameStateEventSwitchLoad_Update`: not waiting; state 0 (section not
    /// loaded / loaded); state 1 (focus too far; transition manager busy, a
    /// transition playing, NIS active, or all idle); other states.
    #[test]
    fn game_state_event_switch_load_update_matches() {
        oc738_install_fakes();
        fakes::covered(
            ee::s06::f_00236960__generated,
            game_state_event_switch_load_update,
            600,
            &|c, seed| {
                let mut r = oc738_machine(c, seed, 0x23_6960, 6);
                let gp = GAME_GP;
                let (holder, game, sm, vm, vm2) = (
                    DATA + 0x1000,
                    DATA + 0x1100,
                    DATA + 0x1200,
                    DATA + 0x1300,
                    DATA + 0x1400,
                );
                let (focus, views, this, tm, tm_vt) = (
                    DATA + 0x1500,
                    DATA + 0x1600,
                    DATA + 0x1700,
                    DATA + 0x1800,
                    DATA + 0x1900,
                );
                c.bus.write32(gp.wrapping_sub(0x848), holder);
                c.bus.write32(holder + 0x84, game);
                c.bus.write32(game + 0x78, sm);
                c.bus
                    .write32(sm + 0x1c8, [0u32, 1, 1, 1][(seed % 4) as usize]);
                c.bus.write32(game + 0x84, vm);
                c.bus.write32(vm + 4, vm2);
                c.bus
                    .write32(vm2 + 0xb0, [0u32, 1, 2][((seed / 4) % 3) as usize]);
                c.bus.write32(game + 0x10, focus);
                c.bus.write32(focus, views);
                c.bus.write32(game + 0xc, DATA + 0x1a00);
                c.bus.write32(DATA + 0x1a00 + 0x40, DATA + 0x1b00);
                // The player camera slot read for the NIS end position.
                c.bus.write32(DATA + 0x1b00 + 0x18, DATA + 0x1c00);
                c.bus.write32(sm + 0x1bc, r.next_u32());
                let state = [0u32, 1, 1, 1, 2, 3, u32::MAX][((seed / 12) % 7) as usize];
                c.bus.write32(this + 0x10, state);
                // 45000.0 / 20000.0 float constants.
                c.bus.write32(gp.wrapping_sub(0x5168), 0x472f_c800);
                c.bus.write32(gp.wrapping_sub(0x5164), 0x469c_4000);
                // The focus distance getTestFocus leaves at sp+0x10.
                let dist = if (seed / 3) % 2 == 0 {
                    0x461c_4000u32
                } else {
                    0x46ea_6000
                };
                c.bus.write32(SP_AFTER_FRAME + 0x10, dist);
                c.bus.write32(OC738_NOT_LOADED, (seed / 7) % 2);
                c.bus.write32(OC738_NIS_IDLE, (seed / 5) % 2);
                // Fake call 1 of state 1 is GetTransitionMan: a real object.
                c.bus.write32(fakes::LOG_RESULTS + 4, tm);
                c.bus.write32(tm + 0x1c, tm_vt);
                c.bus
                    .write16(tm_vt + 0x40, (r.next_u32() as i16 as i32 & !3) as u16);
                // isBusy: the zero leaf (idle) or a fake (busy).
                let busy = if (seed / 2) % 2 == 0 {
                    0x0014_06c0
                } else {
                    fakes::FAKE
                };
                c.bus.write32(tm_vt + 0x44, busy);
                c.bus.write32(tm + 0x44, (seed / 11) % 2);
                c.g[A0][0] = sx(this);
                c.pc = 0x0023_6960;
            },
        );
    }

    use crate::difftest::fakes::machine_rt;

    /// `GameStateEventSwitchLoad_Exit` (0x0023_6af0) against the generated body.
    #[test]
    fn game_state_event_switch_load_exit_00236af0_matches() {
        check(
            ee::s06::f_00236af0__generated,
            game_state_event_switch_load_exit_00236af0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x236af0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GameStateReplayPlayback_Update` (0x0023_7030) against the generated body.
    #[test]
    fn game_state_replay_playback_update_00237030_matches() {
        check(
            ee::s06::f_00237030__generated,
            game_state_replay_playback_update_00237030,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x237030, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GameStateEventSwitchLoad`'s globals (`D_00535BC8`).
    pub(crate) const OC1543_SWITCH_GLOBALS: u32 = 0x0053_5bc8;

    /// The generated loop runs `seed % 4` times (0 skips it), and the
    /// `lb` flag at `+0x49` takes the early exit on every third seed.
    #[test]
    fn game_state_event_switch_load_enter_matches() {
        check(
            ee::s06::f_002368a0__generated,
            game_state_event_switch_load_enter_2,
            200,
            &|c, seed| {
                machine_rt(c, seed, 0x2368, 0);
                c.bus.write32(GAME_GP.wrapping_sub(0x488), DATA + 0x3000);
                c.bus.write32(DATA + 0x3028, DATA + 0x3100);
                c.bus
                    .write8(OC1543_SWITCH_GLOBALS + 0x49, (seed % 3 == 0) as u8);
                c.bus.write32(OC1543_SWITCH_GLOBALS + 0x30, seed % 4);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.pc = 0x0023_68a0;
            },
        );
    }
}
