//! Phase 2 wave module (docs/investigation/phase2-worker-guide.md): typed
//! replacements for cold-path front-end, audio, script and MPEG helpers, each
//! with a differential test against the generated body.
//!
//! Most of these are small state-machine hooks: they read a pointer graph
//! (`gp-0x848` world -> `+0x84` race manager, the `BELibrary` singleton at
//! `gp-0x1f04`, front-end state objects), make a handful of virtual calls and
//! store the results back. Every call follows the shape of the original:
//! the same registers, the same return address, the same unwind check.

use crate::abi::{self, r32};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::{Src, W, X, Y, Z};
use ssx3_ee_rt::Cpu;

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
        0x001ac9c0,
        cfe_use_existing_account_on_email_popup_event,
        "cFEUseExistingAccount_OnEmailPopupEvent",
    ),(0x00278ad0, script_engine_v06, "ScriptEngine_v06"),
    (
        0x0027cf88,
        script_engine_remove_script_obj,
        "ScriptEngine_RemoveScriptObj",
    ),
    (0x002b3ac0, music_player_stop_song, "MusicPlayer_StopSong"),
    (
        0x0037c570,
        ps2_graphics_man_set_brightness_rgb,
        "PS2GraphicsMan_SetBrightnessRGB",
    ),
    (
        0x001cbce8,
        c_keyboard_popup_move_down_from_bottom_row,
        "cKeyboardPopup_MoveDownFromBottomRow",
    ),(
        0x003b2230,
        mpeg2_picture_spatial_scalable_extension,
        "MPEG2_picture_spatial_scalable_extension",
    ),(
        0x00309e50,
        wscript_man_call_table_function,
        "WScriptMan_CallTableFunction",
    ),
    (
        0x0017fad0,
        debug_menu_module_shutdown,
        "DebugMenuModule_Shutdown",
    ),
    (
        0x001d1aa0,
        cfe_state_preview_reward_ctor,
        "cFEStatePreviewReward_ctor",
    ),(
        0x00199870,
        cfe_state_char_equip_on_child_event,
        "cFEStateCharEquip_OnChildEvent",
    ),
    (
        0x0019a308,
        cfe_state_char_equip_show_buy_mode,
        "cFEStateCharEquip_ShowBuyMode",
    ),
    (
        0x00237be8,
        input_sync_unpack_sample,
        "InputSync_UnpackSample",
    ),(0x0028b588, snd_bank_set_memory, "SndBank_SetMemory"),
    (0x003077f8, wscript_process_update, "WScriptProcess_Update"),
    (
        0x001faff8,
        ov_screen_in_game_error_show_cancel_game,
        "OVScreenInGameError_ShowCancelGame",
    ),(
        0x002e4b98,
        c_transition_man_script_activate,
        "cTransitionMan_ScriptActivate",
    ),(
        0x001d4698,
        cfe_state_trophy_room_on_widget_action,
        "cFEStateTrophyRoom_OnWidgetAction",
    ),
    (
        0x0019eba0,
        fe_module_rider_preview_unload,
        "FEModule_RiderPreviewUnload",
    ),
    (0x00398998, ui_icon_load, "UIIcon_Load"),(
        0x001d9e68,
        cfe_state_net_help_popup_update,
        "cFEStateNetHelpPopup_Update",
    ),
    (
        0x00186c98,
        cfe_state_mem_card_show_not_formatted_msg,
        "cFEStateMemCard_ShowNotFormattedMsg",
    ),
    (
        0x00351948,
        intersect_info_mul_matrix,
        "IntersectInfo_MulMatrix",
    ),(
        0x00283450,
        script_sound_bank_man_ctor,
        "ScriptSoundBankMan_ctor",
    ),
    (
        0x0020e818,
        ov_screen_replay_menu_update_progress_bar,
        "OVScreenReplayMenu_UpdateProgressBar",
    ),
    (
        0x001a37f8,
        fe_async_manager_request_session_texture,
        "FEAsyncManager_RequestSessionTexture",
    ),
    (
        0x001b47c0,
        cfe_state_net_config_show_checking_card_popup,
        "cFEStateNetConfig_ShowCheckingCardPopup",
    ),(
        0x001c2c50,
        cfe_state_net_quick_messages_on_save_popup_event,
        "cFEStateNetQuickMessages_OnSavePopupEvent",
    ),
    (
        0x00264b20,
        c_setup_man_flush_send_queue,
        "cSetupMan_FlushSendQueue",
    ),(
        0x00280458,
        rider_script_obj_accept_command,
        "RiderScriptObj_AcceptCommand",
    ),(
        0x001c8258,
        cfe_popup_confirm_get_widget_text,
        "cFEPopupConfirm_GetWidgetText",
    ),];

// ---------------------------------------------------------------------------
// Callees.

mod callee {
    use super::{ee, Callee};

    pub const C_FE_STATE_NET_MESSENGER_CLEAR_NAME_LIST: Callee = Callee {
        addr: 0x001a_5450,
        f: ee::s03::cFEStateNetMessenger_ClearNameList,
    };
    pub const NET_FE_STATE_DTOR: Callee = Callee {
        addr: 0x001a_85d0,
        f: ee::s03::NetFEState_dtor,
    };
    pub const NET_FE_STATE_ON_CHILD_EVENT: Callee = Callee {
        addr: 0x001a_97b8,
        f: ee::s03::NetFEState_OnChildEvent,
    };
    pub const NET_FE_STATE_SHOW_POPUP: Callee = Callee {
        addr: 0x001a_9bc0,
        f: ee::s03::NetFEState_ShowPopup,
    };
    pub const NET_FE_STATE_GET_POPUP: Callee = Callee {
        addr: 0x001a_9c50,
        f: ee::s03::NetFEState_GetPopup,
    };
    pub const C_FE_USE_EXISTING_ACCOUNT_CHECK_EMAIL: Callee = Callee {
        addr: 0x001a_c448,
        f: ee::s03::cFEUseExistingAccount_CheckEmail,
    };
    pub const LOBBY_MAN_CONNECT: Callee = Callee {
        addr: 0x0025_b6d8,
        f: ee::s06::LobbyMan_Connect,
    };
    pub const C_BX_STRING_ASSIGN_CSTR: Callee = Callee {
        addr: 0x0031_84e8,
        f: ee::s10::cBXString_assign_cstr,
    };
    pub const STR_HASH: Callee = Callee {
        addr: 0x0031_7670,
        f: ee::s10::StrHash,
    };

    pub const C_FE_STATE_NET_CONFIG_SET_LIST_VISIBLE: Callee = Callee {
        addr: 0x001b_5068,
        f: ee::s03::cFEStateNetConfig_SetListVisible,
    };
    pub const C_FE_STATE_NET_CONFIG_SHOW_CARD_ERROR_POPUP: Callee = Callee {
        addr: 0x001b_4888,
        f: ee::s03::cFEStateNetConfig_ShowCardErrorPopup,
    };
    pub const C_FE_STATE_NET_CONFIG_LOAD_CONFIG_LIST: Callee = Callee {
        addr: 0x001b_4cd8,
        f: ee::s03::cFEStateNetConfig_LoadConfigList,
    };

    pub const C_FE_STATE_NET_QUICK_PLAY_POPUP_SET_OPTIONS_HINT_VISIBLE: Callee = Callee {
        addr: 0x001c_13e0,
        f: ee::s03::cFEStateNetQuickPlayPopup_SetOptionsHintVisible,
    };
    pub const C_FE_STATE_NET_QUICK_PLAY_POPUP_SET_OPTIONS_ENABLED: Callee = Callee {
        addr: 0x001c_17f8,
        f: ee::s03::cFEStateNetQuickPlayPopup_SetOptionsEnabled,
    };
    pub const UI_TEXT_GET_OWNED_STRING: Callee = Callee {
        addr: 0x003a_0c00,
        f: ee::s13::UIText_GetOwnedString,
    };
    pub const UI_TEXT_GET_ID_STRING: Callee = Callee {
        addr: 0x003a_0c28,
        f: ee::s13::UIText_GetIdString,
    };
    pub const UI_TEXT_SET_TEXT_ID: Callee = Callee {
        addr: 0x003a_1000,
        f: ee::s13::UIText_SetTextId,
    };

    pub const BE_LIBRARY_GET_INSTANCE: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::BELibrary_GetInstance,
    };
    pub const BE_LIBRARY_GET_INTERFACE: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::BELibrary_GetInterface,
    };
    pub const BE_NEW_PLAYER_INT_GET_HUMAN_RIDER: Callee = Callee {
        addr: 0x0014_6e98,
        f: ee::s01::BENewPlayerInt_GetHumanRider,
    };
    pub const C_OV_STATE_OVERLAY_UPDATE: Callee = Callee {
        addr: 0x0020_e900,
        f: ee::s05::cOVStateOverlay_Update,
    };
    pub const UI_STATE_MGR_CLOSE_ALL: Callee = Callee {
        addr: 0x0039_f840,
        f: ee::s13::UIStateMgr_CloseAll,
    };
    pub const GAME_MODULE_ADVANCE_STATE: Callee = Callee {
        addr: 0x0023_1320,
        f: ee::s06::GameModule_AdvanceState,
    };

    pub const SCRIPT_ENGINE_TICK_PROCESS: Callee = Callee {
        addr: 0x0028_2450,
        f: ee::s07::ScriptEngine_TickProcess,
    };
    pub const SCRIPT_ENGINE_GET_PROCESS_IN_SEQ: Callee = Callee {
        addr: 0x0028_2bf0,
        f: ee::s07::ScriptEngine_GetProcessInSeq,
    };
    pub const SCRIPT_VIEW_OBJ_UPDATE: Callee = Callee {
        addr: 0x0028_1100,
        f: ee::s07::ScriptViewObj_Update,
    };
    pub const SCRIPT_ENGINE_REMOVE_BINDING: Callee = Callee {
        addr: 0x0027_d390,
        f: ee::s07::ScriptEngine_RemoveBinding,
    };

    pub const BANK_MONITOR_KILL_TIMERS_BY_CALLBACK: Callee = Callee {
        addr: 0x002a_dde0,
        f: ee::s08::BankMonitor_KillTimersByCallback,
    };
    pub const SND_LOAD_QUEUE_CANCEL_BANK: Callee = Callee {
        addr: 0x0028_a728,
        f: ee::s07::SndLoadQueue_CancelBank,
    };
    pub const SND_BANK_MAN_UNLOAD_BANK: Callee = Callee {
        addr: 0x0028_bce8,
        f: ee::s07::SndBankMan_UnloadBank,
    };
    pub const MUSIC_PLAYER_RESUME: Callee = Callee {
        addr: 0x002b_3a98,
        f: ee::s08::MusicPlayer_Resume,
    };
    pub const SONG_INSTANCE_DTOR: Callee = Callee {
        addr: 0x002b_1f78,
        f: ee::s08::SongInstance_dtor,
    };

    pub const KEYBOARD_POPUP_IS_KEY_ENABLED: Callee = Callee {
        addr: 0x001c_e560,
        f: ee::s04::cKeyboardPopup_IsKeyEnabled,
    };
    pub const KEYBOARD_POPUP_FIND_ENABLED_IN_COLUMN_FROM_TOP: Callee = Callee {
        addr: 0x001c_c090,
        f: ee::s04::cKeyboardPopup_FindEnabledInColumnFromTop,
    };

    pub const UI_STATE_CTOR_2: Callee = Callee {
        addr: 0x0039_e318,
        f: ee::s13::UIState_ctor_2,
    };
    pub const FE_MODULE_GET_PLAYER_PAD_MASK: Callee = Callee {
        addr: 0x001a_1cd0,
        f: ee::s03::FEModule_GetPlayerPadMask,
    };
    pub const BE_NEW_PLAYER_INT_GET_CHARACTER_BY_SLOT: Callee = Callee {
        addr: 0x0014_7488,
        f: ee::s01::BENewPlayerInt_GetCharacterBySlot,
    };
    pub const SSX_AUDIO_SYSTEM_GET_FADE_MASK: Callee = Callee {
        addr: 0x0028_4ba0,
        f: ee::s07::SSXAudioSystem_GetFadeMask,
    };
    pub const SSX_AUDIO_SYSTEM_SET_FADE_GAIN: Callee = Callee {
        addr: 0x0028_7a10,
        f: ee::s07::SSXAudioSystem_SetFadeGain,
    };
    pub const LUNO_VM_REGISTER_AS_ID: Callee = Callee {
        addr: 0x0022_6620,
        f: ee::s05::LunoVMRegister_AsId,
    };
    pub const LUNO_TABLE_FIND: Callee = Callee {
        addr: 0x0022_5248,
        f: ee::s05::cLunoTable_Find,
    };
    pub const LUNO_VM_PASS_THROUGH: Callee = Callee {
        addr: 0x0022_24a8,
        f: ee::s05::LunoVM_PassThrough,
    };
    pub const LUNO_VM_CALL: Callee = Callee {
        addr: 0x0022_24b8,
        f: ee::s05::LunoVM_Call,
    };
    pub const LUNO_VM_REGISTER_DTOR: Callee = Callee {
        addr: 0x0022_5b90,
        f: ee::s05::LunoVMRegister_dtor,
    };

    // MPEG-2 bit reader.
    pub const MPEG2_GET_BITS: Callee = Callee {
        addr: 0x003b_1300,
        f: ee::s13::MPEG2_Get_Bits,
    };
    pub const MPEG2_MARKER_BIT: Callee = Callee {
        addr: 0x003b_2360,
        f: ee::s13::MPEG2_marker_bit,
    };
    pub const MPEG2_EXTENSION_AND_USER_DATA: Callee = Callee {
        addr: 0x003b_1988,
        f: ee::s13::MPEG2_extension_and_user_data,
    };

    pub const EA_MEM_SET: Callee = Callee {
        addr: 0x003e_6448,
        f: ee::s14::EA_MemSet,
    };
    pub const RIDER_GET_START_POSE_ANGLE: Callee = Callee {
        addr: 0x0011_5b08,
        f: ee::s00::Rider_GetStartPoseAngle,
    };
    pub const RIDER_PLACE: Callee = Callee {
        addr: 0x0011_d660,
        f: ee::s00::Rider_Place,
    };
    pub const PATH_DATA_FIND_MARKER: Callee = Callee {
        addr: 0x0026_b5e0,
        f: ee::s07::PathData_FindMarker,
    };
    pub const MEM_ALLOC_NAMED_128: Callee = Callee {
        addr: 0x0025_2fa0,
        f: ee::s06::MEM_AllocNamed128,
    };

    pub const UI_STATE_MGR_CLOSE_TOP_STATES: Callee = Callee {
        addr: 0x0039_f190,
        f: ee::s13::UIStateMgr_CloseTopStates,
    };
    pub const UI_STATE_MGR_CLOSE_ALL_POPUPS: Callee = Callee {
        addr: 0x0039_f698,
        f: ee::s13::UIStateMgr_CloseAllPopups,
    };
    pub const UI_STATE_MGR_SWITCH_STATE: Callee = Callee {
        addr: 0x0039_f400,
        f: ee::s13::UIStateMgr_SwitchState,
    };
    pub const UI_SCREEN_FIND_OBJECT: Callee = Callee {
        addr: 0x0039_d860,
        f: ee::s13::UIScreen_FindObject,
    };
    pub const UI_MENU_SET_SELECTION: Callee = Callee {
        addr: 0x0039_b960,
        f: ee::s13::UIMenu_SetSelection,
    };
    pub const GAME_MODULE_QUIT_TO_LOAD_SCREEN: Callee = Callee {
        addr: 0x0023_0488,
        f: ee::s06::GameModule_QuitToLoadScreen,
    };

    pub const IN_SEQ_START_SEQUENCE: Callee = Callee {
        addr: 0x0027_2938,
        f: ee::s07::InSeq_StartSequence,
    };
    pub const IN_SEQ_GET_INSTANCE: Callee = Callee {
        addr: 0x0027_2cc0,
        f: ee::s07::InSeq_GetInstance,
    };
    pub const IN_SEQ_INSTANCE_SET_USER_DATA: Callee = Callee {
        addr: 0x0027_4c10,
        f: ee::s07::InSeq_InstanceSetUserData,
    };
    pub const IN_SEQ_INSTANCE_SET_SPEED: Callee = Callee {
        addr: 0x0027_4808,
        f: ee::s07::InSeq_InstanceSetSpeed,
    };
    pub const SCRIPT_ENGINE_GET_TRANSITION_TARGET: Callee = Callee {
        addr: 0x0027_a4a0,
        f: ee::s07::ScriptEngine_GetTransitionTarget,
    };
    pub const C_TRANSITION_MAN_START: Callee = Callee {
        addr: 0x002e_4370,
        f: ee::s09::cTransitionMan_Start,
    };
    pub const C_TRANSITION_MAN_START_CAPTURED: Callee = Callee {
        addr: 0x002e_4540,
        f: ee::s09::cTransitionMan_StartCaptured,
    };
    pub const C_CUT_SCENE_TRANSITION_MAN_SAVE_VIEWPORT: Callee = Callee {
        addr: 0x002e_aa28,
        f: ee::s09::cCutSceneTransitionMan_SaveViewport,
    };

    pub const BE_NEW_RACE_INT_GET_TRACK: Callee = Callee {
        addr: 0x0014_4be0,
        f: ee::s01::BENewRaceInt_GetTrack,
    };
    pub const BE_NEW_RACE_INT_GET_SPECIAL_EVENT_VALUE: Callee = Callee {
        addr: 0x0014_5668,
        f: ee::s01::BENewRaceInt_GetSpecialEventValue,
    };
    pub const RIDER_METRIX_SHOW_HUD_MSG_2: Callee = Callee {
        addr: 0x0011_95a8,
        f: ee::s00::RiderMetrix_ShowHUDMsg2A,
    };

    pub const FE_MODULE_RIDER_PREVIEW_SET_VISIBLE: Callee = Callee {
        addr: 0x0019_e538,
        f: ee::s03::FEModule_RiderPreviewSetVisible,
    };
    pub const FE_MODULE_GET_RIDER_PREVIEW: Callee = Callee {
        addr: 0x001a_0548,
        f: ee::s03::FEModule_GetRiderPreview,
    };
    pub const FE_MODULE_RIDER_PREVIEW_RELEASE_BOLT_FILES: Callee = Callee {
        addr: 0x0019_ec68,
        f: ee::s03::FEModule_RiderPreviewReleaseBoltFiles,
    };
    pub const FE_MODULE_RIDER_PREVIEW_UNLOAD_DTOR: Callee = Callee {
        addr: 0x0030_d540,
        f: ee::s10::RiderModel_dtor,
    };

    pub const UI_OBJECT_LOAD: Callee = Callee {
        addr: 0x0039_fe00,
        f: ee::s13::UIObject_Load,
    };
    pub const UI_TEXTURE_SET_FIND: Callee = Callee {
        addr: 0x0039_8380,
        f: ee::s13::UITextureSet_Find,
    };
    pub const INPUT_BINDING_EVAL_FLOAT: Callee = Callee {
        addr: 0x0032_0bf0,
        f: ee::s10::InputBinding_EvalFloat,
    };

    pub const MC_MANAGER_GET: Callee = Callee {
        addr: 0x0022_7f80,
        f: ee::s05::MCManager_Get,
    };
    pub const MC_MANAGER_FORMAT_SLOT_NAME: Callee = Callee {
        addr: 0x0024_1dc8,
        f: ee::s06::MCManager_FormatSlotName,
    };
    pub const U_STR_SPRINTF: Callee = Callee {
        addr: 0x002c_26d0,
        f: ee::s08::UStr_Sprintf,
    };
    pub const U_STR_SPRINTF_ASCII_FMT: Callee = Callee {
        addr: 0x002c_2718,
        f: ee::s08::UStr_SprintfAsciiFmt,
    };
    pub const U_STR_TO_ASCII: Callee = Callee {
        addr: 0x002c_2580,
        f: ee::s08::UStr_ToAscii,
    };
    pub const U_STR_LEN: Callee = Callee {
        addr: 0x003d_bca0,
        f: ee::s14::UStr_Len,
    };

    pub const FE_ASYNC_MANAGER_SET_STATE: Callee = Callee {
        addr: 0x001a_39d8,
        f: ee::s03::FEAsyncManager_SetState,
    };
    pub const BE_MISSION_INT_GET_SESSION_NAME: Callee = Callee {
        addr: 0x0015_4760,
        f: ee::s01::BEMissionInt_GetSessionName,
    };
    pub const SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_TOP: Callee = Callee {
        addr: 0x0028_2bf0,
        f: ee::s07::ScriptEngine_GetProcessInSeq,
    };

    pub const C_SETUP_MAN_ALLOC_PACKET: Callee = Callee {
        addr: 0x0026_4960,
        f: ee::s07::cSetupMan_AllocPacket,
    };
    pub const C_SETUP_MAN_FREE_PACKET: Callee = Callee {
        addr: 0x0026_4990,
        f: ee::s07::cSetupMan_FreePacket,
    };
    pub const C_COMM_SYSTEM_CHANNEL_RECV: Callee = Callee {
        addr: 0x0025_5dc0,
        f: ee::s06::cCommSystem_ChannelRecv,
    };
    pub const C_COMM_SYSTEM_CHANNEL_SEND: Callee = Callee {
        addr: 0x0025_5e00,
        f: ee::s06::cCommSystem_ChannelSend,
    };
    pub const C_BX_STRING_CTOR_CSTR: Callee = Callee {
        addr: 0x0031_82d8,
        f: ee::s10::cBXString_ctor_cstr,
    };
    pub const OPERATOR_NEW_NAMED: Callee = Callee {
        addr: 0x0031_7e30,
        f: ee::s10::operator_new_named,
    };

    pub const NET_FE_STATE_GOTO_NEXT_STATE: Callee = Callee {
        addr: 0x001a_8a40,
        f: ee::s03::NetFEState_GotoNextState,
    };
    pub const C_FE_STATE_NET_SSX_WORLD_CTOR: Callee = Callee {
        addr: 0x001c_1c50,
        f: ee::s03::cFEStateNetSSXWorld_ctor,
    };
    pub const C_FE_STATE_OPTIONS_DEVICE_SELECT_CTOR: Callee = Callee {
        addr: 0x0018_d7f0,
        f: ee::s02::cFEStateOptionsDeviceSelect_ctor,
    };

    pub const SSX_AUDIO_SYSTEM_IS_SPEECH_VOLUME_ON: Callee = Callee {
        addr: 0x0029_f0a0,
        f: ee::s08::SSXAudioSystem_IsSpeechVolumeOn,
    };
    pub const SPEECH_SYSTEM_IS_INSTANCE_READY: Callee = Callee {
        addr: 0x002b_0e28,
        f: ee::s08::SpeechSystem_IsInstanceReady,
    };
    pub const SPEECH_SYSTEM_SET_EVENT_CONTEXT: Callee = Callee {
        addr: 0x002b_1458,
        f: ee::s08::SpeechSystem_SetEventContext,
    };
    pub const SSX_AUDIO_SYSTEM_CHARACTER_TO_SPEECH_MASK: Callee = Callee {
        addr: 0x0029_f2b0,
        f: ee::s08::SSXAudioSystem_CharacterToSpeechMask,
    };
    pub const SPCH_MAKE_ID: Callee = Callee {
        addr: 0x003d_8008,
        f: ee::s14::SPCH_MakeId,
    };

    pub const RIDER_IS_LOADED: Callee = Callee {
        addr: 0x0011_d640,
        f: ee::s00::Rider_IsLoaded,
    };
    pub const RIDER_GET_CHARACTER: Callee = Callee {
        addr: 0x0012_3128,
        f: ee::s00::Rider_GetCharacter,
    };
    pub const SCRIPT_RIDER_GET_PARAMS: Callee = Callee {
        addr: 0x0027_c958,
        f: ee::s07::ScriptRider_GetParams,
    };
    pub const RIDER_SCRIPT_OBJ_MATCHES_RIDER_SLOT: Callee = Callee {
        addr: 0x0028_0560,
        f: ee::s07::RiderScriptObj_MatchesRiderSlot,
    };
    pub const BANK_MONITOR_INSTANCE_RESET: Callee = Callee {
        addr: 0x002a_bb38,
        f: ee::s08::BankMonitorInstance_Reset,
    };

    pub const C_FE_STATE_CHAR_EQUIP_BUY_SELECTED_ITEM: Callee = Callee {
        addr: 0x0019_be80,
        f: ee::s03::cFEStateCharEquip_BuySelectedItem,
    };
    pub const C_FE_STATE_CHAR_EQUIP_UPDATE_MONEY_TEXT: Callee = Callee {
        addr: 0x0019_bba0,
        f: ee::s03::cFEStateCharEquip_UpdateMoneyText,
    };
    pub const C_FE_STATE_CHAR_EQUIP_REMOVE_LIST_ITEM: Callee = Callee {
        addr: 0x0019_b458,
        f: ee::s03::cFEStateCharEquip_RemoveListItem,
    };
    pub const C_FE_STATE_CHAR_EQUIP_REFRESH_ITEM_LIST: Callee = Callee {
        addr: 0x0019_b618,
        f: ee::s03::cFEStateCharEquip_RefreshItemList,
    };
    pub const C_FE_STATE_TROPHY_ROOM_SHOW_TROPHY_IMAGE: Callee = Callee {
        addr: 0x001d_47a0,
        f: ee::s04::cFEStateTrophyRoom_ShowTrophyImage,
    };
    pub const C_FE_STATE_TROPHY_ROOM_SHOW_TROPHY_INFO: Callee = Callee {
        addr: 0x001d_4890,
        f: ee::s04::cFEStateTrophyRoom_ShowTrophyInfo,
    };
}

/// Addresses the generator emitted as `call_indirect` (outside the shard's
/// own direct-call set, so no readable alias is needed here).
mod addr {
    pub const __BUILTIN_DELETE: u32 = 0x0031_7e50;
    pub const EA_MEM_FREE: u32 = 0x0031_7e98;
    pub const C_BX_STRING_DTOR: u32 = 0x0031_8260;
    pub const __FIXUNSSFSI: u32 = 0x0041_39f8;
    pub const STRLEN: u32 = 0x0041_6810;
    pub const FONT_DTOR: u32 = 0x0039_16c0;
}

// ---------------------------------------------------------------------------
// Helpers.

/// A `gp`-relative global.
#[inline(always)]
fn gp(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `gpr = 0xNN0000 + lo` as the `lui`/`addiu` pair in the generated code does
/// it (through `v0`, as the original does).
#[inline(always)]
fn lui_add(c: &mut Cpu, hi: u32, lo: u32) -> u64 {
    c.g[V0][0] = hi as u64;
    c.g[V0][0] = sx(hi.wrapping_add(lo));
    c.g[V0][0]
}

// ---------------------------------------------------------------------------

/// `cFEStateNetMessenger::~cFEStateNetMessenger()` (0x1a3ea0): restore the
/// vtable, release the captured child object through its vtable slot 0x100,
/// free the name-list buffer (one element is deleted directly, more through
/// `EA_MemFree`), destroy the three strings and chain to the base dtor.
pub fn cfe_state_net_messenger_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let this = abi::arg(c, 0);
        let flags = abi::arg(c, 1);
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0);
        c.g[S1][0] = abi::arg64(c, 1);
        // `this->vt = &D_00467F28` (lui 0x46 / addiu 0x7f28).
        let vt = lui_add(c, 0x0046_0000, 0x7f28);
        Addr(this).off(8).sw(c, vt);

        // The captured child object: `vt[32].fn(child + vt[32].delta)`.
        let child = Addr(this).off(0x20).lw(c);
        c.g[A2][0] = child;
        let child_vt = Addr(child as u32).off(8).lw(c);
        c.g[V0][0] = child_vt;
        let delta = Addr(child_vt as u32).off(0x100).lh(c);
        c.g[A0][0] = delta;
        c.g[V1][0] = Addr(child_vt as u32).off(0x104).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx((child as u32).wrapping_add(delta as u32));
        call_ptr(c, target, 0x001a_3edc)?;

        c.g[A0][0] = c.g[S0][0];
        callee::C_FE_STATE_NET_MESSENGER_CLEAR_NAME_LIST.call(c, 0x001a_3ee4)?;

        // The name list: free `[begin, end)` — one element with `delete`, more
        // with `EA_MemFree`.
        c.g[A0][0] = Addr(this).off(0x6e8).lw(c);
        c.g[V0][0] = Addr(this).off(0x6f0).lw(c);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        let empty = c.g[A0][0] == 0;
        c.g[V1][0] = sx((((c.g[V0][0] as u32) as i32) >> 2) as u32);
        if empty {
            c.cycles += 1;
            c.g[A0][0] = sx(this.wrapping_add(0x6e0));
        } else {
            c.cycles += 3;
            c.g[V0][0] = 1;
            if c.g[V1][0] != c.g[V0][0] {
                c.cycles += 2;
                let begin = c.g[A0][0] as u32;
                call_ptr(c, addr::EA_MEM_FREE, 0x001a_3f20)?;
                c.cycles += 1;
                c.g[A0][0] = sx(this.wrapping_add(0x6e0));
            } else {
                c.cycles += 4;
                let begin = c.g[A0][0] as u32;
                call_ptr(c, addr::__BUILTIN_DELETE, 0x001a_3f10)?;
                c.g[A0][0] = sx(this.wrapping_add(0x6e0));
            }
        }

        // The three strings, then the base destructor.
        c.cycles += 16;
        c.g[A1][0] = 2;
        call_ptr(c, addr::C_BX_STRING_DTOR, 0x001a_3f2c)?;
        c.g[A0][0] = sx(this.wrapping_add(0x6d4));
        c.g[A1][0] = 2;
        call_ptr(c, addr::C_BX_STRING_DTOR, 0x001a_3f38)?;
        c.g[A0][0] = sx(this.wrapping_add(0x6d0));
        c.g[A1][0] = 2;
        call_ptr(c, addr::C_BX_STRING_DTOR, 0x001a_3f44)?;
        c.g[A0][0] = abi::arg64(c, 0);
        c.g[A1][0] = c.g[S1][0];
        callee::NET_FE_STATE_DTOR.call(c, 0x001a_3f50)?;

        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEUseExistingAccount::OnEmailPopupEvent(this, args, id)`
/// (0x1ac9c0): when the popup id is 0xf, fill the e-mail popup from the
/// logged-in account, show it, and connect the lobby if the address checks
/// out; otherwise just forward the event to the base state.
pub fn cfe_use_existing_account_on_email_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let this = abi::arg(c, 0);
        let args = abi::arg(c, 1);
        let id = abi::arg64(c, 2);
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = abi::arg64(c, 1);
        c.g[S1][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0);

        if id == 0xf {
            c.cycles += 25;
            c.g[A2][0] = 0xf;
            c.g[S2][0] = 3;
            callee::NET_FE_STATE_ON_CHILD_EVENT.call(c, 0x001a_c9f0)?;

            // The e-mail widget's text: copy the account's address into it.
            c.g[A1][0] = Addr(args).off(0x2b0).lw(c);
            c.g[A0][0] = sx(this.wrapping_add(0x6dc));
            callee::C_BX_STRING_ASSIGN_CSTR.call(c, 0x001a_c9fc)?;

            callee::NET_FE_STATE_GET_POPUP.call(c, 0x001a_ca04)?;
            let popup = abi::r32(c, V0);
            c.g[S0][0] = c.g[V0][0];
            let hash = lui_add(c, 0x0046_0000, 0x2d48);
            Addr(popup).off(0xbc).sw(c, 0);
            c.g[A0][0] = hash;
            callee::STR_HASH.call(c, 0x001a_ca18)?;
            c.g[V1][0] = 0x810;
            Addr(popup).off(0xc0).sw(c, c.g[S2][0]);
            Addr(popup).off(0x164).sw(c, 0);
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            Addr(popup).off(0xc4).sw(c, c.g[V0][0]);
            Addr(popup).off(0x150).sw(c, c.g[V1][0]);
            callee::NET_FE_STATE_SHOW_POPUP.call(c, 0x001a_ca38)?;

            c.g[A0][0] = c.g[S1][0];
            callee::C_FE_USE_EXISTING_ACCOUNT_CHECK_EMAIL.call(c, 0x001a_ca40)?;
            let checked = c.g[V0][0] == 0;
            c.lq(16, frame.at(c, 0x30));
            if !checked {
                c.cycles += 6;
                Addr(this).off(0x6d8).sw(c, c.g[S2][0]);
                c.g[A1][0] = 0;
                c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
                callee::LOBBY_MAN_CONNECT.call(c, 0x001a_ca58)?;
                c.lq(16, frame.at(c, 0x30));
            }
        } else {
            c.cycles += 4;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[S0][0];
            callee::NET_FE_STATE_ON_CHILD_EVENT.call(c, 0x001a_ca6c)?;
            c.lq(16, frame.at(c, 0x30));
        }

        c.cycles += 5;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateNetConfig::CheckCard(this)` (0x1b43a8): query the network
/// library's card state (v53/v55) and either load the config list or show
/// the card error popup (0x840/0x14 when there is no card, 0x840/2 when the
/// card is not readable).
pub fn cfe_state_net_config_check_card(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0);
        let v1 = Addr(gp(c, -0x1578)).lw(c);
        c.g[A1][0] = v1;
        c.g[V1][0] = v1;
        c.g[A0][0] = Addr(v1 as u32).off(0x1a8).lh(c);
        c.g[V0][0] = Addr(v1 as u32).off(0x1ac).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((v1 as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001b_43d4)?;
        let card = abi::r32(c, V0);

        let v1 = Addr(gp(c, -0x1578)).lw(c);
        c.g[A1][0] = v1;
        c.g[S1][0] = c.g[V0][0];
        c.g[V1][0] = v1;
        c.g[A0][0] = Addr(v1 as u32).off(0x1b8).lh(c);
        c.g[V0][0] = Addr(v1 as u32).off(0x1bc).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((v1 as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x001b_43f0)?;
        let readable = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[S0][0];

        if !readable {
            // No card: clear the slot and report 0x840 with code 0x14.
            c.cycles += 10;
            Addr(this).off(0x708).sw(c, 0);
            c.g[A1][0] = u64::MAX;
            callee::C_FE_STATE_NET_CONFIG_SET_LIST_VISIBLE.call(c, 0x001b_4404)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x840;
            c.g[A2][0] = 0x14;
            c.g[A3][0] = u64::MAX;
            callee::C_FE_STATE_NET_CONFIG_SHOW_CARD_ERROR_POPUP.call(c, 0x001b_4418)?;
            c.cycles += 4;
            frame.restore(c, 17, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        if card != 0 {
            c.cycles += 2;
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            callee::C_FE_STATE_NET_CONFIG_LOAD_CONFIG_LIST.call(c, 0x001b_4430)?;
            c.cycles += 4;
            frame.restore(c, 17, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        c.cycles += 2;
        Addr(this).off(0x708).sw(c, 0);
        c.cycles += 9;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = u64::MAX;
        callee::C_FE_STATE_NET_CONFIG_SET_LIST_VISIBLE.call(c, 0x001b_4444)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x840;
        c.g[A2][0] = 2;
        c.g[A3][0] = u64::MAX;
        callee::C_FE_STATE_NET_CONFIG_SHOW_CARD_ERROR_POPUP.call(c, 0x001b_4458)?;
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateNetQuickPlayPopup::OnOptionsPopupEvent(this, args, id)`
/// (0x1c1b88): for the confirm/cancel ids, close the options list, toggle the
/// options hint, re-label the hint text and forward the event; otherwise
/// forward it unchanged.
pub fn cfe_state_net_quick_play_popup_on_options_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let this = abi::arg(c, 0);
        let args = abi::arg(c, 1);
        let id = abi::arg64(c, 2);
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = abi::arg64(c, 0);
        c.g[S1][0] = id;
        frame.save_ra(c, 0);
        c.g[S2][0] = abi::arg64(c, 1);

        if id == 0xf || id == 0x14 {
            c.cycles += 3;
            c.cycles += 27;
            // Close the options list (`v09(0)` on the text object).
            let text = Addr(this).off(0x6e4).lw(c);
            c.g[A2][0] = text;
            c.g[A1][0] = 0;
            Addr(this).off(0x6ec).sw(c, 0);
            let vt = Addr(text as u32).off(8).lw(c);
            c.g[V0][0] = vt;
            c.g[A0][0] = Addr(vt as u32).off(0x48).lh(c);
            c.g[V1][0] = Addr(vt as u32).off(0x4c).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((text as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001c_1bdc)?;

            c.g[A1][0] = Addr(this).off(0x6ec).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] ^= 1;
            callee::C_FE_STATE_NET_QUICK_PLAY_POPUP_SET_OPTIONS_HINT_VISIBLE
                .call(c, 0x001c_1bec)?;

            let hash = lui_add(c, 0x0046_0000, 0x1618);
            c.g[A0][0] = hash;
            callee::STR_HASH.call(c, 0x001c_1bf8)?;
            c.g[A0][0] = Addr(this).off(0x6f4).lw(c);
            c.g[A1][0] = c.g[V0][0];
            callee::UI_TEXT_SET_TEXT_ID.call(c, 0x001c_1c04)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            callee::C_FE_STATE_NET_QUICK_PLAY_POPUP_SET_OPTIONS_ENABLED.call(c, 0x001c_1c10)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S1][0];
            callee::NET_FE_STATE_ON_CHILD_EVENT.call(c, 0x001c_1c20)?;
            c.lq(16, frame.at(c, 0x30));
        } else {
            c.cycles += 3;
            c.cycles += 4;
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S1][0];
            callee::NET_FE_STATE_ON_CHILD_EVENT.call(c, 0x001c_1c34)?;
            c.lq(16, frame.at(c, 0x30));
        }

        c.cycles += 5;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared body of the two finish-overlay updates (0x1e8098, 0x1e87b8):
/// pick the human rider's limit (1 s while a trick is running, else 3 s),
/// advance the overlay's timer and, once it is past the limit, close every
/// screen and advance the game module.
fn overlay_finish_update(c: &mut Cpu, one_second: i32) -> Flow {
    c.cycles += 23;
    let this = abi::arg(c, 0);
    let frame = Frame::open(c, 0x30);
    frame.save(c, 16, 0x10);
    frame.save_fpr(c, 20, 0x20);
    frame.save_ra(c, 0);
    c.g[S0][0] = abi::arg64(c, 0);

    callee::BE_LIBRARY_GET_INSTANCE.call(c, 0x001e_80b0)?;
    c.g[AT][0] = 0x4040_0000;
    c.mtc1(1, 20);
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 1;
    callee::BE_LIBRARY_GET_INTERFACE.call(c, 0x001e_80c4)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    callee::BE_NEW_PLAYER_INT_GET_HUMAN_RIDER.call(c, 0x001e_80d0)?;

    let world = Addr(gp(c, -0x848)).lw(c);
    c.g[A1][0] = world;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
    let manager = Addr(world as u32).off(0x84).lw(c);
    c.g[A0][0] = manager;
    let riders = Addr(manager as u32).off(0xc).lw(c);
    c.g[V1][0] = riders;
    c.g[V1][0] = sx((riders as u32).wrapping_add(c.g[V0][0] as u32));
    let rider = Addr(c.g[V1][0] as u32).off(0x28).lw(c);
    c.g[V0][0] = rider;
    let tricking = Addr(rider as u32).off(0x480).lw(c);
    c.g[A0][0] = tricking;
    if c.g[A0][0] != 0 {
        c.cycles += 2;
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 20);
    }

    // Advance the overlay, then the timer at +0x9c.
    c.cycles += 8;
    callee::C_OV_STATE_OVERLAY_UPDATE.call(c, 0x001e_8104)?;
    c.lwc1(0, this.wrapping_add(0x9c));
    c.lwc1(1, gp(c, one_second));
    c.g[V0][0] = Addr(this).off(0xa0).lw(c);
    c.add_s(0, 0, 1);
    let inactive = c.g[V0][0] == 0;
    c.swc1(0, this.wrapping_add(0x9c));
    if !inactive {
        c.cycles += 4;
        c.c_le_s(20, 0);
        let reached = c.fcc();
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        if reached {
            c.cycles += 7;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x84).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x48).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            callee::UI_STATE_MGR_CLOSE_ALL.call(c, 0x001e_813c)?;
            c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x84).lw(c);
            callee::GAME_MODULE_ADVANCE_STATE.call(c, 0x001e_8148)?;
        }
    }

    c.cycles += 5;
    frame.restore(c, 16, 0x10);
    frame.restore_ra(c, 0);
    frame.restore_fpr(c, 20, 0x20);
    frame.close(c);
    Ok(())
}

/// `OVScreenFinishOv::Update(this)` (0x1e8098).
pub fn ov_screen_finish_ov_update(c: &mut Cpu) {
    guest(c, |c| overlay_finish_update(c, -0x578c))
}

/// `OVScreenFinishOv2::Update(this)` (0x1e87b8) — same body, one second is
/// read from a different constant (4 bytes higher).
pub fn ov_screen_finish_ov2_update(c: &mut Cpu) {
    guest(c, |c| overlay_finish_update(c, -0x5788))
}

/// `int ScriptEngine_v06(this, key)` (0x278ad0): tick the process with `key`
/// and, if it returned non-zero, update the two views attached to the
/// engine's slot array while their predicate holds.
pub fn script_engine_v06(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let this = abi::arg(c, 0);
        let key = abi::arg(c, 1);
        let frame = Frame::open(c, 0x90);
        frame.save(c, 20, 0x40);
        frame.save(c, 22, 0x20);
        c.g[S4][0] = abi::arg64(c, 0);
        frame.save(c, 23, 0x10);
        frame.save(c, 16, 0x80);
        frame.save(c, 17, 0x70);
        frame.save(c, 18, 0x60);
        frame.save(c, 19, 0x50);
        frame.save(c, 21, 0x30);
        frame.save_ra(c, 0);
        c.g[S6][0] = abi::arg64(c, 1);

        callee::SCRIPT_ENGINE_TICK_PROCESS.call(c, 0x0027_8b04)?;
        let result = abi::r32(c, V0);
        c.g[S7][0] = c.g[V0][0];
        let started = c.g[S7][0] == 0;
        c.g[S3][0] = sx(this.wrapping_add(0x6d0));
        if !started {
            c.cycles += 2;
            c.g[S5][0] = 1;
            loop {
                c.cycles += 14;
                c.g[S1][0] = Addr(this).off(0x2a8).lw(c);
                c.g[A0][0] = c.g[S4][0];
                c.g[S2][0] = Addr(c.g[S3][0] as u32).off(4).lw(c);
                c.g[A1][0] = c.g[S6][0];
                c.g[S0][0] = Addr(c.g[S1][0] as u32).off(0x20).lh(c);
                c.g[S0][0] = sx(this.wrapping_add(c.g[S0][0] as u32));
                callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ.call(c, 0x0027_8b34)?;
                c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x24).lw(c);
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = Addr(abi::r32(c, V0)).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A1][0] = c.g[S2][0];
                call_ptr(c, target, 0x0027_8b48)?;
                if c.g[V0][0] == 0 {
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_sub(1));
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S3][0];
                    callee::SCRIPT_VIEW_OBJ_UPDATE.call(c, 0x0027_8b58)?;
                    c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_sub(1));
                }
                c.cycles += 2;
                let more = (c.g[S5][0] as i64) >= 0;
                c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x120));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0027_8b18);
                }
            }
        }

        c.cycles += 12;
        c.g[V0][0] = c.g[S7][0];
        frame.restore(c, 16, 0x80);
        frame.restore(c, 17, 0x70);
        frame.restore(c, 18, 0x60);
        frame.restore(c, 19, 0x50);
        frame.restore(c, 20, 0x40);
        frame.restore(c, 21, 0x30);
        frame.restore(c, 22, 0x20);
        frame.restore(c, 23, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int ScriptEngine_RemoveScriptObj(this, idx)` (0x27cf88): drop the object
/// in slot `idx`, unbinding each of its bindings, then clear the slot.
pub fn script_engine_remove_script_obj(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let this = abi::arg(c, 0);
        let idx = abi::arg(c, 1);
        let frame = Frame::open(c, 0x60);
        frame.save(c, 16, 0x50);
        frame.save(c, 19, 0x20);
        c.g[S0][0] = sx((idx as u32) << 2);
        frame.save(c, 18, 0x30);
        c.g[S3][0] = abi::arg64(c, 0);
        frame.save(c, 20, 0x10);
        c.g[V0][0] = sx(this.wrapping_add(c.g[S0][0] as u32));
        frame.save_ra(c, 0);
        frame.save(c, 17, 0x40);
        c.g[S1][0] = Addr(c.g[V0][0] as u32).lw(c);

        if c.g[S1][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.cycles += 8;
            frame.restore(c, 16, 0x50);
            frame.restore(c, 17, 0x40);
            frame.restore(c, 18, 0x30);
            frame.restore(c, 19, 0x20);
            frame.restore(c, 20, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0xc).lw(c);
        c.cycles += 9;
        c.g[S4][0] = c.g[S0][0];
        c.g[S0][0] = 0;
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x30).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x34).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0027_cfdc)?;
        c.g[S2][0] = c.g[V0][0];
        let none = (c.g[S2][0] as i64) <= 0;
        c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S4][0] as u32));
        if !none {
            c.cycles += 2;
            c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0xc).lw(c);
            loop {
                c.cycles += 12;
                c.g[A1][0] = c.g[S0][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x38).lh(c);
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x3c).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0027_d008)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S3][0];
                callee::SCRIPT_ENGINE_REMOVE_BINDING.call(c, 0x0027_d014)?;
                c.g[V1][0] = ((c.g[S0][0] as i64) < (c.g[S2][0] as i64)) as u64;
                if c.g[V1][0] == 0 {
                    c.cycles += 1;
                    c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S4][0] as u32));
                    break;
                }
                c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0xc).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0027_cff0);
                }
            }
        }

        c.cycles += 2;
        c.g[V0][0] = 1;
        Addr(c.g[V1][0] as u32).sw(c, 0);
        c.cycles += 8;
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `MusicPlayer_StopSong(this)` (0x2b3ac0): kill the three bank-monitor
/// timers, cancel and unload the current song's bank, resume the player,
/// release the song instance and clear the song pointer.
pub fn music_player_stop_song(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = abi::arg64(c, 0);

        // Three `ldl`/`ldr` pairs load the callbacks at 0x4a3838, 0x4a3840 and
        // 0x4a3848 into a1 (`gpr 5`).
        for base in [0x004a_3838u32, 0x004a_3840, 0x004a_3848] {
            c.g[V1][0] = Addr(this).lw(c);
            c.g[V0][0] = 0x004a_0000;
            c.ldl(5, base.wrapping_add(7));
            c.ldr(5, base);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1d8));
            let ret = match base {
                0x004a_3838 => 0x002b_3aec,
                0x004a_3840 => 0x002b_3b08,
                _ => 0x002b_3b24,
            };
            callee::BANK_MONITOR_KILL_TIMERS_BY_CALLBACK.call(c, ret)?;
        }

        c.g[V1][0] = Addr(this).off(0x408).lw(c);
        if c.g[V1][0] != 0 {
            c.cycles += 9;
            c.g[V0][0] = Addr(this).lw(c);
            c.g[A1][0] = Addr(c.g[V1][0] as u32).off(0x34).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
            callee::SND_LOAD_QUEUE_CANCEL_BANK.call(c, 0x002b_3b40)?;
            c.g[V0][0] = Addr(this).lw(c);
            c.g[V1][0] = Addr(this).off(0x408).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[A1][0] = Addr(c.g[V1][0] as u32).off(0x34).lw(c);
            callee::SND_BANK_MAN_UNLOAD_BANK.call(c, 0x002b_3b54)?;
        }

        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        callee::MUSIC_PLAYER_RESUME.call(c, 0x002b_3b5c)?;
        c.g[A0][0] = Addr(this).off(0x408).lw(c);
        let no_song = c.g[A0][0] == 0;
        Addr(this).off(0x418).sw(c, 0);
        if !no_song {
            c.cycles += 2;
            c.g[A1][0] = 3;
            callee::SONG_INSTANCE_DTOR.call(c, 0x002b_3b70)?;
        }

        c.cycles += 5;
        Addr(this).off(0x408).sw(c, 0);
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `PS2GraphicsMan_SetBrightnessRGB(this, rgb)` (0x37c570): store the three
/// channels, each clamped to [0, 1] and scaled to 0..128, into the manager's
/// brightness words.
pub fn ps2_graphics_man_set_brightness_rgb(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let this = abi::arg(c, 0);
        let rgb = abi::arg(c, 1);
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = abi::arg64(c, 1);
        frame.save_fpr(c, 21, 0x38);
        c.g[S1][0] = abi::arg64(c, 0);
        frame.save_fpr(c, 20, 0x30);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 22, 0x40);

        c.g[AT][0] = 0;
        c.mtc1(0, 21);
        c.lwc1(0, rgb);
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 20);
        c.c_le_s(21, 0);
        let negative = !c.fcc();
        c.mov_s(12, 21);
        if !negative {
            c.cycles += 1;
            c.min_s(12, 0, 20);
        }

        c.cycles += 10;
        c.g[AT][0] = 0x4300_0000;
        c.mtc1(1, 22);
        c.mul_s(12, 12, 22);
        call_ptr(c, addr::__FIXUNSSFSI, 0x0037_c5c8)?;
        Addr(this).off(0x59ec).sw(c, c.g[V0][0]);

        c.lwc1(0, rgb.wrapping_add(4));
        c.c_le_s(21, 0);
        let negative = !c.fcc();
        c.mov_s(12, 21);
        if !negative {
            c.cycles += 1;
            c.min_s(12, 0, 20);
        }

        c.cycles += 8;
        c.mul_s(12, 12, 22);
        call_ptr(c, addr::__FIXUNSSFSI, 0x0037_c5ec)?;
        Addr(this).off(0x59f0).sw(c, c.g[V0][0]);

        c.lwc1(0, rgb.wrapping_add(8));
        c.c_le_s(21, 0);
        let negative = !c.fcc();
        c.mov_s(12, 21);
        if !negative {
            c.cycles += 1;
            c.min_s(12, 0, 20);
        }

        c.cycles += 11;
        c.mul_s(12, 12, 22);
        call_ptr(c, addr::__FIXUNSSFSI, 0x0037_c610)?;
        Addr(this).off(0x59f4).sw(c, c.g[V0][0]);

        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 22, 0x40);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `cKeyboardPopup_MoveDownFromBottomRow(this, a1)` (0x1cbce8): from the
/// bottom row, move the cursor to the first enabled key of the next row
/// (wrapping from the last row back to the top), or find a key in the
/// current column.
pub fn c_keyboard_popup_move_down_from_bottom_row(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let this = abi::arg(c, 0);
        let col = abi::arg(c, 1);
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0x10);
        c.g[V1][0] = Addr(this).off(0x140).lw(c);
        let state = c.g[V1][0];
        c.g[S1][0] = abi::arg64(c, 1);

        // 0: fell through to the epilogue with the original's v0,
        // 1: the column search runs, 2: a new state was stored (return 1).
        let mut path = 0u32;
        if state == 0x4c {
            c.cycles += 5;
            c.g[A1][0] = 0;
            c.g[A2][0] = frame.at(c, 0) as u64;
            callee::KEYBOARD_POPUP_IS_KEY_ENABLED.call(c, 0x001c_bd18)?;
            let disabled = c.g[V0][0] == 0;
            c.g[V0][0] = 9;
            if !disabled {
                c.cycles += 2;
                Addr(this).off(0x140).sw(c, c.g[V0][0]);
                path = 2;
            } else {
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0x17;
                c.g[A2][0] = frame.at(c, 0) as u64;
                callee::KEYBOARD_POPUP_IS_KEY_ENABLED.call(c, 0x001c_bd38)?;
                let disabled = c.g[V0][0] == 0;
                c.g[V0][0] = 0x17;
                if !disabled {
                    c.cycles += 2;
                    Addr(this).off(0x140).sw(c, c.g[V0][0]);
                    path = 2;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = Addr(c.g[SP][0] as u32).lw(c);
                    path = 1;
                }
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x4b;
            let other_row = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = 0;
            if !other_row {
                c.cycles += 6;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0x13;
                c.g[A2][0] = sx(sp(c) | 4);
                callee::KEYBOARD_POPUP_IS_KEY_ENABLED.call(c, 0x001c_bd6c)?;
                let disabled = c.g[V0][0] == 0;
                c.g[V0][0] = 0x13;
                if !disabled {
                    c.cycles += 2;
                    Addr(this).off(0x140).sw(c, c.g[V0][0]);
                    path = 2;
                } else {
                    c.cycles += 1;
                    c.g[V0][0] = Addr(c.g[SP][0] as u32).off(4).lw(c);
                    path = 1;
                }
            }
        }

        if path == 1 {
            c.cycles += 2;
            let empty = c.g[V0][0] == 0;
            c.g[V0][0] = (c.g[S1][0] < 1) as u64;
            if !empty {
                c.cycles += 1;
                Addr(this).off(0x144).sw(c, c.g[V0][0]);
            }
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            callee::KEYBOARD_POPUP_FIND_ENABLED_IN_COLUMN_FROM_TOP.call(c, 0x001c_bd94)?;
        }
        if path != 0 {
            c.cycles += 1;
            c.g[V0][0] = 1;
        }

        c.cycles += 5;
        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cKeyboardPopup_InsertChar(this, ch)` (0x1cc620): insert `ch` at the
/// cursor in the typed buffer at +0x74, keeping the cursor inside the field
/// and re-terminating the string.
pub fn c_keyboard_popup_insert_char(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let this = abi::arg(c, 0);
        let ch = abi::arg(c, 1);
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = abi::arg64(c, 0);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = sx((ch as u32) << 24);
        c.g[S2][0] = sx(this.wrapping_add(0x74));
        c.g[S0][0] = sx(((c.g[S0][0] as u32) as i32 >> 24) as u32);
        frame.save_ra(c, 0);

        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, addr::STRLEN, 0x001c_c64c)?;
        c.g[A0][0] = Addr(this).off(0x134).lw(c);
        c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        let at_end = c.g[A0][0] == c.g[V0][0];
        Addr(c.g[V1][0] as u32).sb(c, c.g[S0][0]);

        if at_end {
            // The cursor is at the end: keep it inside the field and
            // re-terminate the buffer.
            c.cycles += 5;
            c.g[V0][0] = Addr(this).off(0x60).lw(c);
            c.g[A1][0] = Addr(this).off(0x134).lw(c);
            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
            if c.g[A1][0] != c.g[A2][0] {
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A2][0] as i64)) as u64;
                c.cycles += 2;
                let keep = c.g[V0][0] == 0;
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                if !keep {
                    c.cycles += 1;
                    Addr(this).off(0x134).sw(c, c.g[V0][0]);
                }
                c.cycles += 7;
                c.g[V0][0] = Addr(this).off(0x134).lw(c);
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                Addr(c.g[V0][0] as u32).sb(c, 0);
                c.g[V1][0] = Addr(this).off(0x13c).lw(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
                Addr(this).off(0x13c).sw(c, c.g[V1][0]);
            } else {
                // The field is full: terminate at max and bump the change
                // counter, then return without the shared tail.
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                Addr(c.g[V1][0] as u32).sb(c, 0);
                c.g[V0][0] = Addr(this).off(0x134).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                Addr(this).off(0x13c).sw(c, c.g[V0][0]);
                c.cycles += 6;
                frame.restore(c, 16, 0x30);
                frame.restore(c, 17, 0x20);
                frame.restore(c, 18, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }

        // Shared tail: advance the cursor when it is not at the field's end.
        c.cycles += 6;
        c.g[V0][0] = Addr(this).off(0x60).lw(c);
        c.g[V1][0] = Addr(this).off(0x134).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        let keep = c.g[V0][0] == 0;
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        if !keep {
            c.cycles += 1;
            Addr(this).off(0x134).sw(c, c.g[V0][0]);
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

/// `SpokeChaseCameraAlgorithm_GetFocusPoint(this, out)` (0x177ef8): the
/// focus point is 20% of the spoke's point plus 80% of the camera position,
/// both read from scaled quads through the object's vtable.
pub fn spoke_chase_camera_algorithm_get_focus_point(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 49;
        let this = abi::arg(c, 0);
        let out = abi::arg(c, 1);
        let frame = Frame::open(c, 0x70);
        frame.save(c, 16, 0x60);
        frame.save(c, 17, 0x50);
        c.g[S0][0] = abi::arg64(c, 1);
        frame.save_ra(c, 0x40);
        c.g[S1][0] = abi::arg64(c, 0);

        // e = &(*(o + 5))[5]; v = e->fn(o + delta) with o = *(self+0x30)+4+0x6c0.
        c.g[V0][0] = Addr(this).off(0x30).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(4).lw(c);
        c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0x6c0).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x6c0));
        c.g[A1][0] = Addr(c.g[V1][0] as u32).off(0x28).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x2c).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, target, 0x0017_7f30)?;

        // 0.2 * point
        c.lwc1(0, gp(c, -0x5e0c));
        c.g[A0][0] = sx(sp(c).wrapping_add(0x20));
        c.mfc1(3, 0);
        c.lqc2(4, abi::r32(c, V0));
        c.qmtc2(3, 3);
        c.vu0().mul(
            0b1111,
            5,
            4,
            ssx3_ee_rt::vu0::Src::Bc(3, ssx3_ee_rt::vu0::X),
        );
        c.sqc2(5, sp(c).wrapping_add(0x10));
        c.lq(2, sp(c).wrapping_add(0x10));
        c.g[A2][0] = Addr(this).off(0x30).lw(c);
        c.sq(2, sp(c));
        c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
        c.g[A1][0] = Addr(c.g[V1][0] as u32).off(8).lh(c);
        c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0xc).lw(c);
        let target = c.g[V0][0] as u32;
        c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A1][0] as u32));
        call_ptr(c, target, 0x0017_7f6c)?;

        // 0.8 * camera position
        c.lwc1(0, gp(c, -0x5e08));
        c.g[V0][0] = c.g[S1][0];
        c.mfc1(4, 0);
        c.lqc2(4, sp(c).wrapping_add(0x20));
        c.qmtc2(4, 3);
        c.vu0().mul(
            0b1111,
            5,
            4,
            ssx3_ee_rt::vu0::Src::Bc(3, ssx3_ee_rt::vu0::X),
        );
        c.sqc2(5, sp(c).wrapping_add(0x30));
        c.lq(16, sp(c).wrapping_add(0x60));
        c.lq(3, sp(c).wrapping_add(0x30));
        frame.restore_ra(c, 0x40);
        c.sq(3, sp(c).wrapping_add(0x10));
        c.lqc2(3, sp(c));
        c.lqc2(4, sp(c).wrapping_add(0x10));
        c.vu0().add(0b1111, 5, 3, ssx3_ee_rt::vu0::Src::Vec(4));
        c.sqc2(5, sp(c).wrapping_add(0x30));
        c.lq(3, sp(c).wrapping_add(0x30));
        c.sq(3, abi::r32(c, S1));
        c.lq(17, sp(c).wrapping_add(0x50));
        let ra = abi::r32(c, RA);
        c.g[SP][0] = sx(sp(c).wrapping_add(0x70));
        c.pc = ra;
        Ok(())
    })
}

/// `WScriptMan_CallTableFunction(this, idx, value)` (0x309e50): marshal the
/// value into a script argument, look the scripted function up in the table
/// at +0x3cc, call it and destroy the temporaries.
pub fn wscript_man_call_table_function(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 49;
        let this = abi::arg(c, 0);
        let idx = abi::arg(c, 1);
        let value = abi::arg(c, 2);
        let frame = Frame::open(c, 0x90);
        c.g[V0][0] = 3;
        frame.save(c, 17, 0x70);
        frame.save(c, 16, 0x80);
        c.g[S1][0] = abi::arg64(c, 0);
        frame.save(c, 18, 0x60);
        c.g[A0][0] = sp(c) as u64;
        frame.save(c, 19, 0x50);
        c.g[S2][0] = abi::arg64(c, 2);
        frame.save_ra(c, 0x40);
        c.g[S0][0] = abi::arg64(c, 1);
        c.g[A1][0] = 0;
        Addr(sp(c)).off(8).sw(c, c.g[V0][0]);

        // key = LunoVMRegister_AsId(&key); *key = value
        callee::LUNO_VM_REGISTER_AS_ID.call(c, 0x0030_9e84)?;
        Addr(abi::r32(c, V0)).sw(c, c.g[S2][0]);

        c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
        c.g[S1][0] = sx(this.wrapping_add(0x3cc));
        c.g[A1][0] = sp(c) as u64;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
        callee::LUNO_TABLE_FIND.call(c, 0x0030_9ea0)?;

        // Copy the found argument onto the stack (an unaligned doubleword).
        c.ldl(3, abi::r32(c, V0).wrapping_add(7));
        c.ldr(3, abi::r32(c, V0));
        c.sdl(3, sp(c).wrapping_add(0x17));
        c.sdr(3, sp(c).wrapping_add(0x10));
        c.g[S2][0] = sx(sp(c).wrapping_add(0x30));
        c.g[S3][0] = sx(sp(c).wrapping_add(0x10));
        Addr(sp(c)).off(0x28).sw(c, 0);
        Addr(sp(c)).off(0x20).sw(c, 0);

        c.g[A0][0] = c.g[S2][0];
        callee::LUNO_VM_PASS_THROUGH.call(c, 0x0030_9ec8)?;
        c.g[S0][0] = sx(sp(c).wrapping_add(0x20));
        c.g[A2][0] = Addr(c.g[S1][0] as u32).lw(c);
        c.g[A3][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S3][0];
        callee::LUNO_VM_CALL.call(c, 0x0030_9ee0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        callee::LUNO_VM_REGISTER_DTOR.call(c, 0x0030_9eec)?;
        c.g[A0][0] = sp(c) as u64;
        c.g[A1][0] = 2;
        callee::LUNO_VM_REGISTER_DTOR.call(c, 0x0030_9ef8)?;

        frame.restore(c, 16, 0x80);
        frame.restore(c, 17, 0x70);
        frame.restore(c, 18, 0x60);
        frame.restore(c, 19, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `DebugMenuModule_Shutdown(this)` (0x17fad0): release the two debug
/// objects, clear the world's two debug counters and the font handle, and
/// stop the two debug sounds.
pub fn debug_menu_module_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x40);
        frame.save(c, 18, 0x10);
        frame.save(c, 16, 0x30);
        c.g[S2][0] = abi::arg64(c, 0);
        frame.save(c, 17, 0x20);
        c.g[S0][0] = sx(this.wrapping_add(0xc));
        frame.save_ra(c, 0);
        c.g[S1][0] = 1;

        loop {
            c.cycles += 3;
            c.g[A2][0] = Addr(c.g[S0][0] as u32).lw(c);
            let absent = c.g[A2][0] == 0;
            c.g[A1][0] = 3;
            if !absent {
                c.cycles += 5;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
                c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0017_fb10)?;
            }
            c.cycles += 3;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0017_faf0);
            }
        }

        c.cycles += 6;
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        Addr(c.g[V0][0] as u32).off(0x80).sw(c, 0);
        Addr(c.g[V0][0] as u32).off(0x7c).sw(c, 0);

        c.g[A1][0] = Addr(this).off(0x20).lw(c);
        let no_sound20 = (c.g[A1][0] as i64) < 0;
        c.g[A2][0] = Addr(gp(c, -0x854)).lw(c);
        if !no_sound20 {
            c.cycles += 5;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x10d8).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x190).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x194).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0017_fb48)?;
        }

        c.cycles += 3;
        c.g[A1][0] = Addr(this).off(0x1c).lw(c);
        let no_sound1c = (c.g[A1][0] as i64) < 0;
        c.g[A2][0] = Addr(gp(c, -0x854)).lw(c);
        if !no_sound1c {
            c.cycles += 5;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x10d8).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x190).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x194).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0017_fb68)?;
        }

        c.cycles += 3;
        c.g[A0][0] = Addr(this).off(0x18).lw(c);
        let no_font = c.g[A0][0] == 0;
        c.lq(16, frame.at(c, 0x30));
        if !no_font {
            c.cycles += 3;
            c.g[A1][0] = 3;
            call_ptr(c, 0x0039_16c0, 0x0017_fb7c)?;
            c.lq(16, frame.at(c, 0x30));
        }

        c.cycles += 5;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStatePreviewReward::cFEStatePreviewReward(this, engine, owner, player)`
/// (0x1d1aa0): construct the base state, install the vtable, store the player
/// index, fetch the player's pad mask and zero the reward slots.
pub fn cfe_state_preview_reward_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let this = abi::arg(c, 0);
        let player = abi::arg64(c, 3);
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = sx((player as u32) << 24);
        c.g[S1][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0);

        c.g[S0][0] = sx((((c.g[S0][0] as u32) as i32) >> 24) as u32);
        callee::UI_STATE_CTOR_2.call(c, 0x001d_1ac0)?;
        c.g[V0][0] = 0x30;
        c.g[V1][0] = 0x0047_0000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x6608));
        Addr(this).off(0xc).sw(c, c.g[V0][0]);
        Addr(this).off(8).sw(c, c.g[V1][0]);
        c.g[A1][0] = c.g[S0][0];
        Addr(this).off(0x44).sb(c, c.g[S0][0]);
        c.g[V0][0] = Addr(this).off(0x10).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
        callee::FE_MODULE_GET_PLAYER_PAD_MASK.call(c, 0x001d_1ae8)?;
        Addr(this).off(0x15).sb(c, c.g[V0][0]);

        c.g[V1][0] = 2;
        c.g[V0][0] = sx(this.wrapping_add(0x54));
        loop {
            c.cycles += 7;
            Addr(c.g[V0][0] as u32).off(0xffff_fff4).sw(c, 0);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
            Addr(c.g[V0][0] as u32).sw(c, 0);
            let more = (c.g[V1][0] as i64) >= 0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001d_1af8);
            }
        }

        c.cycles += 20;
        Addr(this).off(0x60).sw(c, 0);
        callee::BE_LIBRARY_GET_INSTANCE.call(c, 0x001d_1b1c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        callee::BE_LIBRARY_GET_INTERFACE.call(c, 0x001d_1b28)?;
        c.g[A1][0] = Addr(this).off(0x44).lb(c);
        c.g[A0][0] = c.g[V0][0];
        callee::BE_NEW_PLAYER_INT_GET_CHARACTER_BY_SLOT.call(c, 0x001d_1b34)?;
        c.g[V1][0] = u64::MAX;
        Addr(this).off(0x64).sw(c, c.g[V0][0]);
        Addr(this).off(0x74).sw(c, c.g[V1][0]);
        c.g[V0][0] = c.g[S1][0];
        Addr(this).off(0x68).sw(c, c.g[V1][0]);
        Addr(this).off(0x6c).sw(c, c.g[V1][0]);
        Addr(this).off(0x70).sw(c, c.g[V1][0]);

        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem_SetFadeMode(this, mode, f0, f1)` (0x287f00): when the
/// requested mode differs and no fade is pending, look the mode's mask up and
/// start a fade; when it matches and a fade is pending, retarget it; otherwise
/// do nothing.
pub fn ssx_audio_system_set_fade_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let this = abi::arg(c, 0);
        let mode = abi::arg(c, 1);
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = abi::arg64(c, 0);
        frame.save_fpr(c, 21, 0x38);
        c.g[S1][0] = abi::arg64(c, 1);
        frame.save_fpr(c, 20, 0x30);
        c.mov_s(21, 12);
        frame.save_ra(c, 0);
        c.mov_s(20, 13);

        // `worked` marks the paths that reach 'b_287fa8 (one extra cycle
        // before the epilogue); the early exits skip it.
        let mut worked = false;
        if mode != 3 {
            c.cycles += 3;
            c.g[V0][0] = Addr(this).off(0x63dc).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 12;
                c.g[A0][0] = Addr(this).off(0x63d4).lw(c);
                callee::SSX_AUDIO_SYSTEM_GET_FADE_MASK.call(c, 0x0028_7f44)?;
                c.g[AT][0] = 0x3f80_0000;
                c.mtc1(1, 12);
                c.g[A1][0] = c.g[V0][0];
                c.mov_s(13, 21);
                c.g[A0][0] = c.g[S0][0];
                callee::SSX_AUDIO_SYSTEM_SET_FADE_GAIN.call(c, 0x0028_7f5c)?;
                c.swc1(20, this.wrapping_add(0x63d8));
                Addr(this).off(0x63d4).sw(c, c.g[S1][0]);
                Addr(this).off(0x63dc).sw(c, 0);
                worked = true;
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(this).off(0x63dc).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 12;
                c.g[A0][0] = c.g[S1][0];
                callee::SSX_AUDIO_SYSTEM_GET_FADE_MASK.call(c, 0x0028_7f80)?;
                c.g[A1][0] = c.g[V0][0];
                c.mov_s(13, 21);
                c.g[A0][0] = c.g[S0][0];
                c.mov_s(12, 20);
                callee::SSX_AUDIO_SYSTEM_SET_FADE_GAIN.call(c, 0x0028_7f94)?;
                c.g[V0][0] = 1;
                Addr(this).off(0x63d4).sw(c, c.g[S1][0]);
                Addr(this).off(0x63dc).sw(c, c.g[V0][0]);
                c.swc1(20, this.wrapping_add(0x63d8));
                Addr(this).off(0x63e0).sw(c, 0);
                worked = true;
            }
        }
        if worked {
            c.cycles += 1;
        }

        c.cycles += 6;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 21, 0x38);
        frame.restore_fpr(c, 20, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `IntersectInfo_MulMatrix(self, m)` (0x351948): `dst = m * dst` through the
/// VU0 matrix-multiply routine, then copy the result back.
pub fn intersect_info_mul_matrix(c: &mut Cpu) {
    c.cycles += 49;
    let m = abi::arg(c, 1);
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x80));
    let dst = abi::arg(c, 0).wrapping_add(0x90);
    c.g[A0][0] = sx(dst);
    c.g[V0][0] = sx(sp(c).wrapping_add(0x40));

    let tmp = sp(c).wrapping_add(0x40);
    c.lqc2(4, m);
    c.lqc2(5, m.wrapping_add(0x10));
    c.lqc2(6, m.wrapping_add(0x20));
    c.lqc2(7, m.wrapping_add(0x30));
    c.lqc2(8, dst);
    c.lqc2(9, dst.wrapping_add(0x10));
    c.lqc2(10, dst.wrapping_add(0x20));
    c.lqc2(11, dst.wrapping_add(0x30));
    c.vu0().mula(0b1111, 4, Src::Bc(8, X));
    c.vu0().madda(0b1111, 5, Src::Bc(8, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(8, Z));
    c.vu0().madd(0b1111, 12, 7, Src::Bc(8, W));
    c.vu0().mula(0b1111, 4, Src::Bc(9, X));
    c.vu0().madda(0b1111, 5, Src::Bc(9, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(9, Z));
    c.vu0().madd(0b1111, 13, 7, Src::Bc(9, W));
    c.vu0().mula(0b1111, 4, Src::Bc(10, X));
    c.vu0().madda(0b1111, 5, Src::Bc(10, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(10, Z));
    c.vu0().madd(0b1111, 14, 7, Src::Bc(10, W));
    c.vu0().mula(0b1111, 4, Src::Bc(11, X));
    c.vu0().madda(0b1111, 5, Src::Bc(11, Y));
    c.vu0().madda(0b1111, 6, Src::Bc(11, Z));
    c.vu0().madd(0b1111, 15, 7, Src::Bc(11, W));
    c.sqc2(12, tmp);
    c.sqc2(13, tmp.wrapping_add(0x10));
    c.sqc2(14, tmp.wrapping_add(0x20));
    c.sqc2(15, tmp.wrapping_add(0x30));
    // Copy the result through a second temporary and store it into `dst`.
    c.lqc2(1, tmp);
    c.lqc2(2, tmp.wrapping_add(0x10));
    c.lqc2(3, tmp.wrapping_add(0x20));
    c.lqc2(4, tmp.wrapping_add(0x30));
    c.sqc2(1, sp(c));
    c.sqc2(2, sp(c).wrapping_add(0x10));
    c.sqc2(3, sp(c).wrapping_add(0x20));
    c.sqc2(4, sp(c).wrapping_add(0x30));
    c.lqc2(1, sp(c));
    c.lqc2(2, sp(c).wrapping_add(0x10));
    c.lqc2(3, sp(c).wrapping_add(0x20));
    c.lqc2(4, sp(c).wrapping_add(0x30));
    c.sqc2(1, dst);
    c.sqc2(2, dst.wrapping_add(0x10));
    c.sqc2(3, dst.wrapping_add(0x20));
    c.sqc2(4, dst.wrapping_add(0x30));

    let ra = abi::r32(c, RA);
    c.g[SP][0] = sx(sp(c).wrapping_add(0x80));
    c.pc = ra;
}

/// `MPEG2_picture_spatial_scalable_extension()` (0x3b2230): read the spatial
/// scalability fields (lower/upper layer, priority, horizontal and vertical
/// sub-sampling) from the MPEG-2 bit reader.
pub fn mpeg2_picture_spatial_scalable_extension(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        c.g[V0][0] = 0x0051_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = Addr(0x0051_0000u32.wrapping_sub(0x5f78)).lw(c);
        c.g[A1][0] = 1;
        frame.save_ra(c, 0);
        c.g[A0][0] = 0xa;
        Addr(c.g[V1][0] as u32).off(0x430).sw(c, c.g[A1][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_2250)?;

        c.g[V1][0] = 0x0051_0000;
        c.g[A0][0] = 0x0049_0000;
        Addr(0x0051_0000u32.wrapping_sub(0x6a08)).sw(c, c.g[V0][0]);
        c.g[A0][0] = sx(0x0049_0000u32.wrapping_add(0x5598));
        callee::MPEG2_MARKER_BIT.call(c, 0x003b_2264)?;

        c.g[A0][0] = 0xf;
        callee::MPEG2_GET_BITS.call(c, 0x003b_226c)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[A0][0] = 0x0051_0000;
        c.g[V0][0] = ((c.g[V1][0] as i64) < 16384) as u64;
        let small = c.g[V0][0] != 0;
        Addr(0x0051_0000u32.wrapping_sub(0x6a04)).sw(c, c.g[V1][0]);
        if !small {
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x8000));
            Addr(0x0051_0000u32.wrapping_sub(0x6a04)).sw(c, c.g[V0][0]);
        }

        c.cycles += 10;
        c.g[A0][0] = 0x0049_0000;
        c.g[A0][0] = sx(0x0049_0000u32.wrapping_add(0x55d0));
        callee::MPEG2_MARKER_BIT.call(c, 0x003b_2294)?;
        c.g[A0][0] = 0xf;
        callee::MPEG2_GET_BITS.call(c, 0x003b_229c)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[A0][0] = 0x0051_0000;
        c.g[V0][0] = ((c.g[V1][0] as i64) < 16384) as u64;
        let small = c.g[V0][0] != 0;
        Addr(0x0051_0000u32.wrapping_sub(0x6a00)).sw(c, c.g[V1][0]);
        if !small {
            c.cycles += 2;
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x8000));
            Addr(0x0051_0000u32.wrapping_sub(0x6a00)).sw(c, c.g[V0][0]);
        }

        for (bits, off, ret) in [
            (2u32, 0x69fc, 0x003b_22c0u32),
            (1, 0x69f8, 0x003b_22d0),
            (1, 0x69f4, 0x003b_22e0),
        ] {
            c.cycles += if off == 0x69fc { 15 } else { 0 };
            c.g[A0][0] = bits as u64;
            callee::MPEG2_GET_BITS.call(c, ret)?;
            c.g[V1][0] = 0x0051_0000;
            c.g[A0][0] = 1;
            Addr(0x0051_0000u32.wrapping_sub(off)).sw(c, c.g[V0][0]);
            if off == 0x69f4 {
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
        frame.restore_ra(c, 0);
        frame.close(c);
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

/// `Rider_PlaceAtStart(this)` (0x11dd98): read the start marker's position
/// and direction, offset the position by the start pose angle and place the
/// rider with type 0x11f and the 3000-unit snap radius.
pub fn rider_place_at_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 49;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x60);
        c.g[A1][0] = 2;
        frame.save(c, S0, 0x50);
        frame.save_ra(c, 0x40);
        c.g[S0][0] = abi::arg64(c, 0);
        c.g[A0][0] = 0x004d_0000;
        c.g[A2][0] = Addr(this).off(0x86c).lw(c);
        c.g[A0][0] = sx(0x004d_0000u32.wrapping_add(0x33a0));
        callee::PATH_DATA_FIND_MARKER.call(c, 0x0011_ddbc)?;

        let p = abi::r32(c, V0);
        c.lwc1(1, p.wrapping_add(0x14));
        c.g[A0][0] = c.g[S0][0];
        c.g[AT][0] = 0x3f80_0000;
        c.mtc1(1, 2);
        c.swc1(1, sp(c));
        c.lwc1(0, p.wrapping_add(0x18));
        c.swc1(0, sp(c).wrapping_add(4));
        c.lwc1(1, p.wrapping_add(0x1c));
        Addr(sp(c)).off(0xc).sw(c, 0);
        c.swc1(1, sp(c).wrapping_add(8));
        c.lwc1(0, p.wrapping_add(8));
        c.swc1(0, sp(c).wrapping_add(0x10));
        c.lwc1(1, p.wrapping_add(0xc));
        c.swc1(1, sp(c).wrapping_add(0x14));
        c.lwc1(0, p.wrapping_add(0x10));
        c.swc1(2, sp(c).wrapping_add(0x1c));
        c.swc1(0, sp(c).wrapping_add(0x18));
        callee::RIDER_GET_START_POSE_ANGLE.call(c, 0x0011_de04)?;

        c.mfc1(2, 0);
        c.lqc2(4, sp(c));
        c.qmtc2(2, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sp(c).wrapping_add(0x30));
        c.lwc1(12, gp(c, -0x798c));
        c.lq(2, sp(c).wrapping_add(0x30));
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(sp(c).wrapping_add(0x10));
        c.g[A2][0] = sp(c) as u64;
        c.sq(2, sp(c).wrapping_add(0x20));
        c.g[A3][0] = 0x11f;
        c.lqc2(3, sp(c).wrapping_add(0x10));
        c.lqc2(4, sp(c).wrapping_add(0x20));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp(c).wrapping_add(0x10));
        callee::RIDER_PLACE.call(c, 0x0011_de4c)?;

        frame.restore(c, 16, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `SndBank_SetMemory(this, size, extra, tag)` (0x28b588): store the flags,
/// try to allocate the `extra` region from the stream allocator and record
/// the sized allocation.
pub fn snd_bank_set_memory(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let this = abi::arg(c, 0);
        let size = abi::arg(c, 1);
        let extra = abi::arg(c, 2);
        let tag = abi::arg(c, 3);
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = abi::arg64(c, 2);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = abi::arg64(c, 1);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0);
        let nonzero = c.g[S1][0] != 0;
        c.g[S3][0] = abi::arg64(c, 3);

        if !nonzero {
            c.cycles += 2;
            if c.g[S0][0] == 0 {
                frame.restore(c, 16, 0x40);
                c.cycles += 6;
                frame.restore(c, 17, 0x30);
                frame.restore(c, 18, 0x20);
                frame.restore(c, 19, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }

        c.cycles += 3;
        c.g[V0][0] = 1;
        let no_extra = c.g[S0][0] == 0;
        Addr(this).off(8).sw(c, c.g[V0][0]);
        if !no_extra {
            c.cycles += 12;
            c.g[V1][0] = Addr(gp(c, 0x524)).lw(c);
            c.g[A1][0] = c.g[S0][0];
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1d8).lw(c);
            c.g[A2][0] = Addr(c.g[V0][0] as u32).lw(c);
            c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x20).lh(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x24).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0028_b5ec)?;
            c.g[A0][0] = c.g[V0][0];
            let failed = (c.g[A0][0] as i64) < 0;
            c.g[V0][0] = 2;
            if !failed {
                c.cycles += 3;
                Addr(this).off(0xc).sw(c, c.g[A0][0]);
                Addr(this).off(8).sw(c, c.g[V0][0]);
            } else {
                c.cycles += 4;
                c.g[V0][0] = sx(0xffff_fffd_u32);
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.g[V0][0] = c.g[A0][0] ^ c.g[V0][0];
                if c.g[V0][0] == 0 {
                    c.g[S1][0] = c.g[V1][0];
                }
            }
        }

        c.cycles += 8;
        Addr(this).off(0x14).sw(c, c.g[S1][0]);
        c.g[A0][0] = 0x0048_0000;
        c.g[A0][0] = sx(0x0048_0000u32.wrapping_add(0x2998));
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S3][0];
        callee::MEM_ALLOC_NAMED_128.call(c, 0x0028_b62c)?;
        Addr(this).off(0x10).sw(c, c.g[V0][0]);
        frame.restore(c, 16, 0x40);
        c.cycles += 6;
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `WScriptProcess_Update(this)` (0x3077f8): the scripted process's update —
/// start it on the first tick, advance its current element and, while the
/// element did not change the state, run the process's own step hook.
pub fn wscript_process_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0);

        let state = Addr(this).lw(c);
        c.g[V1][0] = state;
        let is_one = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
        if !is_one {
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                frame.restore(c, 16, 0x20);
                c.cycles += 4;
                frame.restore(c, 17, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 3;
            c.g[V0][0] = 2;
            if c.g[V1][0] != c.g[V0][0] {
                c.cycles += 2;
                frame.restore(c, 16, 0x20);
                c.cycles += 4;
                frame.restore(c, 17, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        } else {
            // state == 1: move to state 2 and fall through.
            c.cycles += 4;
            c.g[V0][0] = 2;
            Addr(this).sw(c, c.g[V0][0]);
        }

        // state == 2
        c.g[S1][0] = Addr(this).lw(c);
        c.g[V0][0] = 2;
        c.cycles += 2;
        if c.g[S1][0] != c.g[V0][0] {
            frame.restore(c, 16, 0x20);
            c.cycles += 4;
            frame.restore(c, 17, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        let _ = state;
        c.cycles += 5;
        c.g[A1][0] = Addr(this).off(8).lw(c);
        c.g[V0][0] = Addr(this).off(4).lw(c);
        c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = Addr(this).lw(c);
        } else {
            c.cycles += 10;
            c.g[A0][0] = 0x44;
            c.g[V1][0] = Addr(this).off(0x14).lw(c);
            c.mult(0, 4, 5, 4);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x40).lw(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x10).lh(c);
            c.g[A1][0] = Addr(c.g[V0][0] as u32).off(0x14).lw(c);
            let target = c.g[A1][0] as u32;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            call_ptr(c, target, 0x0030_7888)?;
            c.g[V0][0] = Addr(this).lw(c);
        }

        c.cycles += 2;
        if c.g[V0][0] != c.g[S1][0] {
            frame.restore(c, 16, 0x20);
            c.cycles += 4;
            frame.restore(c, 17, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        c.cycles += 6;
        c.g[V0][0] = Addr(this).off(0x5c).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x28).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x2c).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, 0x0030_78a8)?;

        frame.restore(c, 16, 0x20);
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
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
            callee::UI_MENU_SET_SELECTION.call(c, 0x001f_b0a4)?;
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

/// `ScriptEngine_LaunchPendingProcess(this, script)` (0x282908): start the
/// script's pending sequence, bind the instance as the thread's owner and
/// bump the slot's use count.
pub fn script_engine_launch_pending_process(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let this = abi::arg(c, 0);
        let script = abi::arg(c, 1);
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = abi::arg64(c, 1);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0);
        frame.save_fpr(c, 20, 0x40);
        let idx = Addr(script).off(0x14).lw(c);
        c.g[A2][0] = idx;
        c.g[V0][0] = Addr(this).off(0x2b0).lw(c);
        c.g[V1][0] = sx((idx as u32) << 3);
        c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = Addr(c.g[S2][0] as u32).lw(c);
        let empty = c.g[A0][0] == 0;
        c.g[V1][0] = 2;
        if empty {
            c.cycles += 7;
            frame.restore(c, 16, 0x30);
            frame.restore(c, 17, 0x20);
            frame.restore(c, 18, 0x10);
            frame.restore_ra(c, 0);
            frame.restore_fpr(c, 20, 0x40);
            frame.close(c);
            return Ok(());
        }

        c.cycles += 21;
        c.g[V0][0] = sx(0xffff_ffff_u32);
        Addr(script).off(8).sw(c, c.g[A2][0]);
        Addr(script).off(4).sw(c, c.g[V1][0]);
        Addr(script).off(0x14).sw(c, c.g[V0][0]);
        Addr(script).off(0x12).sh(c, 0);
        c.mtc1(0, 20);
        c.g[A0][0] = Addr(this).off(0x2b8).lw(c);
        c.g[A1][0] = Addr(c.g[S2][0] as u32).lw(c);
        callee::IN_SEQ_START_SEQUENCE.call(c, 0x0028_2968)?;
        c.g[A0][0] = Addr(this).off(0x2b8).lw(c);
        c.g[A1][0] = c.g[V0][0];
        callee::IN_SEQ_GET_INSTANCE.call(c, 0x0028_2974)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        Addr(script).sw(c, c.g[V0][0]);
        callee::IN_SEQ_INSTANCE_SET_USER_DATA.call(c, 0x0028_2984)?;
        c.lwc1(12, script.wrapping_add(0x18));
        c.c_lt_s(20, 12);
        if c.fcc() {
            c.cycles += 4;
            c.g[A0][0] = Addr(script).lw(c);
            callee::IN_SEQ_INSTANCE_SET_SPEED.call(c, 0x0028_29a0)?;
            c.swc1(20, script.wrapping_add(0x18));
        }
        c.g[V0][0] = Addr(c.g[S2][0] as u32).off(4).lhu(c);
        c.cycles += 2;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        Addr(c.g[S2][0] as u32).off(4).sh(c, c.g[V0][0]);

        frame.restore(c, 16, 0x30);
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `cTransitionMan_ScriptActivate(this)` (0x2e4b98): activate the script's
/// transition — a captured start for type 7, a full start otherwise — and
/// record its level.
pub fn c_transition_man_script_activate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x40);
        frame.save(c, 17, 0x20);
        c.g[S1][0] = abi::arg64(c, 0);
        frame.save(c, 18, 0x10);
        frame.save(c, 16, 0x30);
        c.g[A0][0] = sx(this.wrapping_add(0x10));
        frame.save_ra(c, 0);
        c.g[S2][0] = sx(this.wrapping_add(0x28));
        call_ptr(c, 0x0028_3430, 0x002e_4bbc)?;

        c.g[A1][0] = Addr(this).off(0x28).lw(c);
        c.g[V0][0] = 7;
        let captured = c.g[A1][0] == c.g[V0][0];
        c.g[A0][0] = Addr(gp(c, -0x84c)).lw(c);
        if !captured {
            c.cycles += 19;
            c.g[A2][0] = Addr(this).off(0x38).lw(c);
            callee::SCRIPT_ENGINE_GET_TRANSITION_TARGET.call(c, 0x002e_4bd4)?;
            c.g[A0][0] = Addr(gp(c, -0x84c)).lw(c);
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = Addr(this).off(0x28).lw(c);
            c.g[A2][0] = Addr(this).off(0x38).lw(c);
            callee::SCRIPT_ENGINE_GET_TRANSITION_TARGET.call(c, 0x002e_4be8)?;
            c.lwc1(12, this.wrapping_add(0x34));
            c.g[A2][0] = c.g[S0][0];
            c.lwc1(13, this.wrapping_add(0x30));
            c.g[A3][0] = c.g[V0][0];
            c.lwc1(14, this.wrapping_add(0x2c));
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 2;
            c.g[T0][0] = 0;
            c.g[T1][0] = 0;
            callee::C_TRANSITION_MAN_START.call(c, 0x002e_4c10)?;
            c.lwc1(0, abi::r32(c, S2).wrapping_add(0x14));
        } else {
            c.cycles += 10;
            c.g[A1][0] = 7;
            c.g[A2][0] = Addr(this).off(0x38).lw(c);
            callee::SCRIPT_ENGINE_GET_TRANSITION_TARGET.call(c, 0x002e_4c24)?;
            c.lwc1(12, this.wrapping_add(0x34));
            c.g[A2][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 2;
            c.g[A3][0] = 0;
            callee::C_TRANSITION_MAN_START_CAPTURED.call(c, 0x002e_4c3c)?;
            c.lwc1(0, abi::r32(c, S2).wrapping_add(0x14));
        }

        c.cycles += 7;
        frame.restore(c, 16, 0x30);
        c.swc1(0, this.wrapping_add(0x48));
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `GMHandler4_v06(this)` (0x23b5f8): when the special event is in the
/// 17..21 range, show the HUD message and count the seconds.
pub fn gm_handler4_v06(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = abi::arg64(c, 0);
        callee::BE_LIBRARY_GET_INSTANCE.call(c, 0x0023_b610)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        callee::BE_LIBRARY_GET_INTERFACE.call(c, 0x0023_b61c)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        callee::BE_NEW_RACE_INT_GET_TRACK.call(c, 0x0023_b628)?;

        c.g[V1][0] = c.g[V0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 17) as u64;
        c.g[V0][0] = c.g[V0][0] ^ 1;
        let not_in_range = c.g[V0][0] == 0;
        c.g[A0][0] = 0;
        if !not_in_range {
            c.cycles += 1;
            c.g[A0][0] = ((c.g[V1][0] as i64) < 22) as u64;
        }

        c.cycles += 2;
        let skip = c.g[A0][0] == 0;
        c.g[V0][0] = 0x0053_0000;
        if !skip {
            c.cycles += 17;
            c.g[A2][0] = Addr(this).off(0xa).lh(c);
            c.g[A1][0] = Addr(0x0053_0000u32.wrapping_add(0x5c12)).lb(c);
            c.g[A0][0] = c.g[S1][0];
            c.g[A3][0] = Addr(this).off(8).lhu(c);
            callee::BE_NEW_RACE_INT_GET_SPECIAL_EVENT_VALUE.call(c, 0x0023_b65c)?;
            c.g[V1][0] = Addr(gp(c, -0x848)).lw(c);
            c.g[A0][0] = c.g[V0][0];
            c.lwc1(1, gp(c, -0x514c));
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x84).lw(c);
            c.g[V0][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
            c.lwc1(0, abi::r32(c, V0).wrapping_add(0xc));
            c.cvt_s_w(0, 0);
            c.mul_s(0, 0, 1);
            c.cvt_w_s(1, 0);
            c.mfc1(5, 1);
            let no_delta = c.g[A0][0] == 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
            if !no_delta {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[V0][0] as u32).off(0x28).lw(c);
                c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x790).lw(c);
                callee::RIDER_METRIX_SHOW_HUD_MSG_2.call(c, 0x0023_b698)?;
            }
        }

        c.cycles += 3;
        c.g[V0][0] = Addr(this).off(8).lhu(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        Addr(this).off(8).sh(c, c.g[V0][0]);

        c.cycles += 6;
        frame.restore(c, 16, 0x20);
        c.g[V0][0] = 1;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
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

/// `FEModule_RiderPreviewUnload(this)` (0x19eba0): release the preview's
/// model and its three sub-objects, clear the handles and reset the state.
pub fn fe_module_rider_preview_unload(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = abi::arg64(c, 0);
        c.g[A0][0] = Addr(this).off(8).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[A1][0] = 3;
            callee::FE_MODULE_RIDER_PREVIEW_UNLOAD_DTOR.call(c, 0x0019_ebc4)?;
        }
        c.g[A2][0] = Addr(this).off(0xc).lw(c);

        // The three sub-objects (offsets 0xc, 0xc74, 0xc70).
        c.cycles += 2;
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 3;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x58).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_ebe4)?;
        }

        c.cycles += 3;
        c.g[A2][0] = Addr(this).off(0xc74).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 3;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0xc4).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_ec04)?;
        }

        c.cycles += 3;
        c.g[A2][0] = Addr(this).off(0xc70).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 3;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0xa4).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(8).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_ec24)?;
        }

        c.cycles += 16;
        Addr(this).off(8).sw(c, 0);
        c.g[A0][0] = c.g[S0][0];
        Addr(this).off(0xc).sw(c, 0);
        Addr(this).off(0xc70).sw(c, 0);
        Addr(this).off(0xc74).sw(c, 0);
        callee::FE_MODULE_RIDER_PREVIEW_RELEASE_BOLT_FILES.call(c, 0x0019_ec3c)?;
        c.g[V0][0] = sx(0xffff_ffff_u32);
        Addr(this).off(0xcc8).sw(c, 0);
        Addr(this).off(0xcd4).sw(c, c.g[V0][0]);
        Addr(this).sw(c, c.g[V0][0]);
        Addr(this).off(4).sw(c, c.g[V0][0]);
        Addr(this).off(0xc78).sw(c, 0);

        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `UIIcon_Load(this, data)` (0x398998): load the icon's UI object, copy the
/// clip/align flags, find the texture in the scene's texture set and store
/// the scaled size.
pub fn ui_icon_load(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 49;
        let this = abi::arg(c, 0);
        let data = abi::arg(c, 1);
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0x10);
        c.g[S1][0] = abi::arg64(c, 1);
        callee::UI_OBJECT_LOAD.call(c, 0x0039_89b4)?;

        c.g[V0][0] = Addr(data).off(0x20).lw(c);
        c.g[A0][0] = sx(0xffff_fffd_u32);
        c.g[V1][0] = Addr(this).off(0x74).lw(c);
        c.g[A1][0] = sx(0xffff_fff3_u32);
        c.g[V0][0] = sx((c.g[V0][0] as u32) >> 1);
        c.g[A2][0] = Addr(this).off(0x5c).lw(c);
        c.g[V1][0] = c.g[V1][0] & c.g[A0][0];
        c.g[V0][0] = c.g[V0][0] & 2;
        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
        Addr(this).off(0x74).sw(c, c.g[V1][0]);
        c.g[V1][0] = c.g[V1][0] & c.g[A1][0];
        c.g[V0][0] = Addr(data).off(0x20).lbu(c);
        c.g[V0][0] = c.g[V0][0] & 3;
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V1][0] = c.g[V1][0] | c.g[V0][0];
        Addr(this).off(0x74).sw(c, c.g[V1][0]);

        c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0xd0).lw(c);
        c.g[A1][0] = Addr(data).off(0x24).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x10).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x58));
        callee::UI_TEXTURE_SET_FIND.call(c, 0x0039_8a08)?;
        Addr(this).off(0x7c).sw(c, c.g[V0][0]);

        c.lwc1(2, gp(c, -0x2548));
        c.g[V0][0] = Addr(data).off(0x2a).lhu(c);
        c.g[V1][0] = Addr(data).off(0x28).lhu(c);
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        frame.restore(c, 17, 0x20);
        c.mtc1(3, 1);
        c.cvt_s_w(1, 1);
        frame.restore_ra(c, 0x10);
        c.mul_s(0, 0, 2);
        c.mul_s(1, 1, 2);
        c.swc1(0, sp(c).wrapping_add(4));
        c.swc1(1, sp(c));
        c.ldl(2, sp(c).wrapping_add(7));
        c.ldr(2, sp(c));
        c.sdl(2, this.wrapping_add(0x87));
        c.sdr(2, this.wrapping_add(0x80));
        frame.restore(c, 16, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenEnterLodge_OnWidgetAction(this, widget, msg)` (0x1f72e0): the
/// "enter lodge" prompt — quit to the load screen for widget 0, mark the
/// screen done and close it for widget 1.
pub fn ov_screen_enter_lodge_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let this = abi::arg(c, 0);
        let widget = abi::arg(c, 1);
        let msg = abi::arg64(c, 2);
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = abi::arg64(c, 0);
        let no_widget = c.g[A1][0] == 0;
        frame.save_ra(c, 0);
        if no_widget {
            c.cycles += 1;
            c.cycles += 2;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        // The original's exits: case 6 goes through 'b_1f7398 (one extra
        // cycle), the others straight to the epilogue.
        let mut extra = false;
        c.cycles += 3;
        c.g[V0][0] = 5;
        let case5 = c.g[A2][0] == c.g[V0][0];
        c.g[V0][0] = 6;
        let mut case6 = false;
        if !case5 {
            c.cycles += 2;
            case6 = c.g[A2][0] == c.g[V0][0];
            frame.restore_ra(c, 0);
        }

        if case5 {
            // `game->flag->active = 1`, then the widget's own switch.
            c.cycles += 8;
            c.g[A0][0] = Addr(gp(c, -0x848)).lw(c);
            c.g[A2][0] = 1;
            c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x84).lw(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x200).lw(c);
            Addr(c.g[V1][0] as u32).off(8).sw(c, c.g[A2][0]);
            c.g[V0][0] = Addr(widget).off(0x18).lw(c);
            if c.g[V0][0] == 0 {
                c.g[A0][0] = Addr(c.g[A0][0] as u32).off(0x84).lw(c);
                // case 0: quit to the load screen.
                c.cycles += 6;
                c.g[V0][0] = 0x27;
                Addr(gp(c, -0x172c)).sw(c, c.g[V0][0]);
                c.g[A1][0] = 0;
                callee::GAME_MODULE_QUIT_TO_LOAD_SCREEN.call(c, 0x001f_734c)?;
            } else {
                c.cycles += 2;
                if c.g[V0][0] == c.g[A2][0] {
                    // case 1.
                    c.cycles += 7;
                    c.g[A0][0] = Addr(this).off(0x10).lw(c);
                    c.g[A1][0] = 1;
                    Addr(this).off(0x54).sw(c, c.g[V0][0]);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                    callee::UI_STATE_MGR_CLOSE_TOP_STATES.call(c, 0x001f_7368)?;
                } else {
                    c.cycles += 2;
                }
            }
        } else if case6 {
            c.cycles += 10;
            c.g[A0][0] = Addr(gp(c, -0x848)).lw(c);
            c.g[A2][0] = 1;
            Addr(this).off(0x54).sw(c, c.g[A2][0]);
            c.g[A1][0] = 1;
            c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0x84).lw(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x200).lw(c);
            Addr(c.g[V1][0] as u32).off(8).sw(c, c.g[A2][0]);
            c.g[A0][0] = Addr(this).off(0x10).lw(c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            callee::UI_STATE_MGR_CLOSE_TOP_STATES.call(c, 0x001f_7398)?;
            extra = true;
        } else {
            c.cycles += 2;
        }

        if extra {
            c.cycles += 1;
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateNetHelpPopup_Update(this)` (0x1d9e68): if neither help-popup slot
/// has its binding pressed, close the popup.
pub fn cfe_state_net_help_popup_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x60);
        frame.save_fpr(c, 20, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        c.g[S0][0] = 0;
        frame.save(c, 18, 0x20);
        c.g[S1][0] = 0;
        frame.save(c, 19, 0x10);
        c.g[S2][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0);
        c.g[S3][0] = 1;
        c.mtc1(0, 20);
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);

        loop {
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0xb0).lw(c);
            c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
            let absent = c.g[V0][0] == 0;
            c.g[A1][0] = 0;
            if !absent {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                c.g[A1][0] = c.g[S3][0];
                if c.g[V0][0] == 0 {
                    c.g[A1][0] = 0;
                }
            }

            c.cycles += 2;
            if c.g[A1][0] == 0 {
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 6;
                c.g[A1][0] = 0x99;
                callee::INPUT_BINDING_EVAL_FLOAT.call(c, 0x001d_9ed4)?;
                c.c_eq_s(0, 20);
                if !c.fcc() {
                    c.g[S1][0] = 1;
                }
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
            }

            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < 2) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001d_9ea0);
            }
        }

        c.cycles += 2;
        let found = c.g[S1][0] != 0;
        c.lq(16, frame.at(c, 0x40));
        if !found {
            c.cycles += 5;
            c.g[A0][0] = Addr(this).off(0x10).lw(c);
            c.g[A1][0] = 1;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
            callee::UI_STATE_MGR_CLOSE_TOP_STATES.call(c, 0x001d_9f0c)?;
            c.lq(16, frame.at(c, 0x40));
        }

        c.cycles += 7;
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x50);
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

/// `cCutSceneTransitionMan_StartIn(this, dur)` (0x2ea6b8): fade a cut-scene
/// transition in — save the viewport for the first stage and rescale the
/// remaining time by the new duration for the others.
pub fn c_cut_scene_transition_man_start_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x10);
        frame.save_fpr(c, 20, 0x20);
        c.g[S0][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(this).off(0x10).lw(c);
        let state = c.g[V1][0];
        let is_one = c.g[V1][0] == c.g[V0][0];
        c.mov_s(20, 12);

        if !is_one {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
            let at_least_two = c.g[V0][0] == 0;
            c.g[V0][0] = 2;
            if !at_least_two {
                c.cycles += 2;
                c.g[V0][0] = 3;
                if c.g[V1][0] == 0 {
                    // state 0: save the viewport and reset the timer.
                    c.cycles += 5;
                    callee::C_CUT_SCENE_TRANSITION_MAN_SAVE_VIEWPORT.call(c, 0x002e_a718)?;
                    c.swc1(20, this.wrapping_add(0x18));
                    Addr(this).off(0x14).sw(c, 0);
                } else {
                    c.cycles += 2;
                }
            } else {
                c.cycles += 2;
                let is_two = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = 3;
                if is_two {
                    // state 2: keep the elapsed fraction.
                    c.cycles += 10;
                    c.lwc1(0, this.wrapping_add(0x14));
                    c.lwc1(2, this.wrapping_add(0x18));
                    c.g[AT][0] = 0x3f80_0000;
                    c.mtc1(1, 1);
                    c.div_s(0, 0, 2);
                    c.swc1(20, this.wrapping_add(0x18));
                    c.sub_s(1, 1, 0);
                    c.mul_s(1, 1, 20);
                    c.swc1(1, this.wrapping_add(0x14));
                } else {
                    c.cycles += 2;
                    if c.g[V1][0] == c.g[V0][0] {
                        // state 3: rescale the elapsed time.
                        c.lwc1(0, this.wrapping_add(0x14));
                        c.cycles += 5;
                        c.lwc1(1, this.wrapping_add(0x18));
                        c.swc1(20, this.wrapping_add(0x18));
                        c.div_s(0, 0, 1);
                        c.mul_s(0, 0, 20);
                        c.swc1(0, this.wrapping_add(0x14));
                    } else {
                        c.cycles += 2;
                    }
                }
            }
            // The shared tail sets the state to 3.
            c.cycles += 2;
            c.g[V0][0] = 3;
            Addr(this).off(0x10).sw(c, c.g[V0][0]);
        }

        c.cycles += 5;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.restore_fpr(c, 20, 0x20);
        frame.close(c);
        let _ = state;
        Ok(())
    })
}

/// `ScriptSoundBankMan::ctor(this, flags, count, base, tag)` (0x283450):
/// install the vtable, allocate the bank array and initialise each entry.
pub fn script_sound_bank_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let this = abi::arg(c, 0);
        let flags = abi::arg(c, 1);
        let count = abi::arg(c, 2);
        let base = abi::arg(c, 3);
        let tag = abi::arg64(c, 4);
        let frame = Frame::open(c, 0x30);
        c.g[T0][0] = abi::arg64(c, 2);
        frame.save(c, 17, 0x10);
        c.g[V1][0] = abi::arg64(c, 1);
        c.g[S1][0] = abi::arg64(c, 0);
        frame.save(c, 16, 0x20);
        c.g[A0][0] = 0xc;
        c.g[V0][0] = 0x0048_0000;
        c.mult(0, 0, 8, 4);
        frame.save_ra(c, 0);
        c.g[V0][0] = sx(0x0048_0000u32.wrapping_add(0x2240));
        c.g[S0][0] = abi::arg64(c, 3);
        c.g[A1][0] = 0x0048_0000;
        c.g[A2][0] = c.g[V1][0];
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_add(0x1fb0));
        Addr(this).sw(c, c.g[V1][0]);
        c.g[A0][0] = c.lo[0];
        Addr(this).off(0xc).sw(c, c.g[V0][0]);
        c.g[A3][0] = 0;
        Addr(this).off(4).sw(c, c.g[T0][0]);
        callee::OPERATOR_NEW_NAMED.call(c, 0x0028_34a4)?;

        c.g[V1][0] = Addr(this).off(4).lw(c);
        c.g[A1][0] = 0;
        let none = (c.g[V1][0] as i64) <= 0;
        Addr(this).off(8).sw(c, c.g[V0][0]);
        if !none {
            c.cycles += 3;
            c.g[A2][0] = sx(0xffff_ffff_u32);
            c.g[A0][0] = 0;
            loop {
                c.cycles += 15;
                c.g[V1][0] = Addr(this).off(8).lw(c);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                Addr(c.g[V1][0] as u32).sw(c, c.g[A2][0]);
                c.g[V0][0] = Addr(this).off(8).lw(c);
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                Addr(c.g[V0][0] as u32).off(4).sw(c, c.g[S0][0]);
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[V0][0] = Addr(this).off(8).lw(c);
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                Addr(c.g[V0][0] as u32).off(8).sw(c, 0);
                c.g[V0][0] = Addr(this).off(4).lw(c);
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0028_34c0);
                }
            }
        }

        c.cycles += 6;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        let _ = (flags, count, base, tag);
        Ok(())
    })
}

/// `OVScreenReplayMenu_UpdateProgressBar(this)` (0x20e818): move the replay
/// progress handle to `170 + 315 * played/total` (0 when there is no replay).
pub fn ov_screen_replay_menu_update_progress_bar(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        c.g[S0][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0x10);
        c.g[A0][0] = sx(0x0047_0000u32.wrapping_add(0x1cc8));
        callee::STR_HASH.call(c, 0x0020_e834)?;
        c.g[A0][0] = Addr(this).off(0x40).lw(c);
        c.g[A1][0] = c.g[V0][0];
        callee::UI_SCREEN_FIND_OBJECT.call(c, 0x0020_e840)?;
        c.g[A0][0] = c.g[V0][0];
        let absent = c.g[A0][0] == 0;
        c.lq(16, frame.at(c, 0x20));
        if !absent {
            c.cycles += 12;
            c.ldl(2, abi::r32(c, A0).wrapping_add(0x4b));
            c.ldr(2, abi::r32(c, A0).wrapping_add(0x44));
            c.g[A1][0] = Addr(c.g[A0][0] as u32).off(0x4c).lw(c);
            c.sdl(2, sp(c).wrapping_add(7));
            c.sdr(2, sp(c));
            Addr(sp(c)).off(8).sw(c, c.g[A1][0]);
            c.g[V1][0] = Addr(gp(c, -0x848)).lw(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x84).lw(c);
            c.g[V0][0] = Addr(c.g[V0][0] as u32).off(0x28).lw(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x3cc).lw(c);
            let no_total = (c.g[V1][0] as i64) <= 0;
            if !no_total {
                c.cycles += 7;
                c.lwc1(1, abi::r32(c, V0).wrapping_add(0x484));
                c.cvt_s_w(1, 1);
                c.mtc1(3, 0);
                c.cvt_s_w(0, 0);
                c.div_s(2, 1, 0);
            } else {
                c.cycles += 1;
                c.mtc1(0, 2);
            }
            c.cycles += 13;
            c.lwc1(0, gp(c, -0x54f8));
            c.g[AT][0] = 0x432a_0000;
            c.mtc1(1, 1);
            c.mul_s(0, 2, 0);
            c.add_s(0, 0, 1);
            c.swc1(0, sp(c));
            c.ldl(2, sp(c).wrapping_add(7));
            c.ldr(2, sp(c));
            c.g[V1][0] = Addr(sp(c)).off(8).lw(c);
            c.sdl(2, abi::r32(c, A0).wrapping_add(0x4b));
            c.sdr(2, abi::r32(c, A0).wrapping_add(0x44));
            Addr(c.g[A0][0] as u32).off(0x4c).sw(c, c.g[V1][0]);
            c.lq(16, frame.at(c, 0x20));
        }

        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `FEAsyncManager_RequestSessionTexture(this, id, slot, param)` (0x1a37f8):
/// format the request file name and mark the slot as pending.
pub fn fe_async_manager_request_session_texture(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let this = abi::arg(c, 0);
        let id = abi::arg(c, 1);
        let slot = abi::arg(c, 2);
        let param = abi::arg(c, 3);
        let frame = Frame::open(c, 0x80);
        frame.save(c, 19, 0x40);
        frame.save(c, 20, 0x30);
        c.g[S3][0] = abi::arg64(c, 0);
        frame.save(c, 21, 0x20);
        c.g[S4][0] = abi::arg64(c, 1);
        frame.save(c, 22, 0x10);
        c.g[S5][0] = abi::arg64(c, 2);
        frame.save(c, 16, 0x70);
        c.g[S6][0] = abi::arg64(c, 3);
        frame.save(c, 17, 0x60);
        frame.save(c, 18, 0x50);
        let no_id = (c.g[S4][0] as i64) < 0;
        frame.save_ra(c, 0);
        if no_id {
            // fall through to the epilogue
        } else {
            c.cycles += 26;
            c.g[S0][0] = 0x11c;
            callee::BE_LIBRARY_GET_INSTANCE.call(c, 0x001a_3838)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0xa;
            callee::BE_LIBRARY_GET_INTERFACE.call(c, 0x001a_3844)?;
            c.g[S2][0] = 0x004a_0000;
            c.mult(0, 16, 21, 16);
            c.g[S1][0] = 0x004a_0000;
            c.g[S2][0] = sx(0x004a_0000u32.wrapping_add(0x1a08));
            c.g[S1][0] = sx(0x004a_0000u32.wrapping_add(0x1a10));
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S4][0];
            callee::BE_MISSION_INT_GET_SESSION_NAME.call(c, 0x001a_3864)?;
            c.g[T0][0] = 0x004a_0000;
            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[A1][0] = c.g[S2][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[A3][0] = c.g[V0][0];
            c.g[T0][0] = sx(0x004a_0000u32.wrapping_add(0x1a00));
            call_ptr(c, 0x0041_7828, 0x001a_3880)?; // sprintf
            c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[A0][0] = c.g[S3][0];
            Addr(c.g[S0][0] as u32).off(0x108).sw(c, c.g[S6][0]);
            c.g[A1][0] = c.g[S5][0];
            c.g[A2][0] = 1;
            callee::FE_ASYNC_MANAGER_SET_STATE.call(c, 0x001a_3898)?;
        }

        c.cycles += 10;
        frame.restore(c, 16, 0x70);
        frame.restore(c, 17, 0x60);
        frame.restore(c, 18, 0x50);
        frame.restore(c, 19, 0x40);
        frame.restore(c, 20, 0x30);
        frame.restore(c, 21, 0x20);
        frame.restore(c, 22, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        let _ = (this, slot, param);
        Ok(())
    })
}

/// `cFEStateNetConfig_ShowCheckingCardPopup(this, name)` (0x1b47c0):
/// format the "checking card" line and show the 0x83F popup.
pub fn cfe_state_net_config_show_checking_card_popup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 50;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x70);
        c.g[A2][0] = abi::arg64(c, 1);
        frame.save(c, 18, 0x40);
        c.g[V0][0] = 0x004a_0000;
        c.g[S2][0] = abi::arg64(c, 0);
        frame.save(c, 16, 0x60);
        c.g[A0][0] = sp(c) as u64;
        frame.save(c, 17, 0x50);
        frame.save(c, 19, 0x30);
        c.g[A1][0] = sx(0x004a_0000u32.wrapping_add(0x12c0));
        frame.save_ra(c, 0x20);
        c.g[S3][0] = sx(this.wrapping_add(0x71c));
        callee::U_STR_SPRINTF_ASCII_FMT.call(c, 0x001b_47f4)?;

        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x3f58));
        c.g[S0][0] = Addr(c.g[V0][0] as u32).off(0x8c).lw(c);
        c.g[S1][0] = Addr(c.g[S0][0] as u32).off(4).lw(c);
        c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x20).lh(c);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        callee::STR_HASH.call(c, 0x001b_4814)?;
        c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x24).lw(c);
        c.g[A0][0] = c.g[S0][0];
        let target = c.g[V1][0] as u32;
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, target, 0x001b_4824)?;

        c.g[A2][0] = sp(c) as u64;
        c.g[A1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S3][0];
        callee::U_STR_SPRINTF.call(c, 0x001b_4834)?;

        c.g[A0][0] = c.g[S2][0];
        callee::NET_FE_STATE_GET_POPUP.call(c, 0x001b_483c)?;
        c.g[A2][0] = 1;
        c.g[V1][0] = 0x83f;
        Addr(abi::r32(c, V0)).off(0xc4).sw(c, c.g[S3][0]);
        c.g[A0][0] = c.g[S2][0];
        Addr(abi::r32(c, V0)).off(0x154).sw(c, c.g[A2][0]);
        c.g[A1][0] = c.g[V0][0];
        Addr(abi::r32(c, V0)).off(0x150).sw(c, c.g[V1][0]);
        Addr(abi::r32(c, V0)).off(0xbc).sw(c, 0);
        Addr(abi::r32(c, V0)).off(0xc0).sw(c, c.g[A2][0]);
        Addr(abi::r32(c, V0)).off(0x164).sw(c, 0);
        Addr(abi::r32(c, V0)).off(0x168).sw(c, 0);
        callee::NET_FE_STATE_SHOW_POPUP.call(c, 0x001b_486c)?;

        frame.restore(c, 16, 0x60);
        frame.restore(c, 17, 0x50);
        frame.restore(c, 18, 0x40);
        frame.restore(c, 19, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateNetConfig_HandleLoadError(this, err)` (0x1b4fa0): map a network
/// load error to the config popup's message and hide the list.
pub fn cfe_state_net_config_handle_load_error(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = sx(0xffff_fff1_u32);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        let is_minus15 = c.g[A1][0] == c.g[V0][0];
        c.g[S0][0] = abi::arg64(c, 0);

        if is_minus15 {
            c.cycles += 7;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0x841;
            c.g[A2][0] = 6;
            c.g[A3][0] = sx(0xffff_ffff_u32);
            callee::C_FE_STATE_NET_CONFIG_SHOW_CARD_ERROR_POPUP.call(c, 0x001b_5030)?;
            c.g[A0][0] = c.g[S0][0];
        } else {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A1][0] as i64) < -14) as u64;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = sx(0xffff_ff38_u32);
                c.cycles += 2;
                let is_minus200 = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = ((c.g[A1][0] as i64) < -200) as u64;
                if is_minus200 {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x841;
                    c.g[A2][0] = 0x14;
                    c.g[A3][0] = 0;
                    callee::C_FE_STATE_NET_CONFIG_SHOW_CARD_ERROR_POPUP.call(c, 0x001b_504c)?;
                    c.g[A0][0] = c.g[S0][0];
                } else {
                    c.cycles += 2;
                    let below = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S0][0];
                    if below {
                        // nothing
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[A1][0] as i64) < -21) as u64;
                        let below21 = c.g[V0][0] != 0;
                        c.g[A3][0] = Addr(gp(c, -0x1570)).lw(c);
                        if below21 {
                            // nothing
                        } else {
                            c.cycles += 6;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = 0x842;
                            c.g[A2][0] = 0x18;
                            callee::C_FE_STATE_NET_CONFIG_SHOW_CARD_ERROR_POPUP
                                .call(c, 0x001b_5014)?;
                            c.g[A0][0] = c.g[S0][0];
                        }
                    }
                }
            } else {
                c.cycles += 2;
                let negative = (c.g[A1][0] as i64) < 0;
                c.g[A3][0] = Addr(gp(c, -0x1570)).lw(c);
                if negative {
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x842;
                    c.g[A2][0] = 0x18;
                    callee::C_FE_STATE_NET_CONFIG_SHOW_CARD_ERROR_POPUP.call(c, 0x001b_5014)?;
                    c.g[A0][0] = c.g[S0][0];
                } else {
                    c.cycles += 2;
                    if c.g[A1][0] != 0 {
                        c.g[A0][0] = c.g[S0][0];
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = Addr(this).off(0x10).lw(c);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                        callee::UI_STATE_MGR_CLOSE_ALL_POPUPS.call(c, 0x001b_4fe0)?;
                        c.g[A0][0] = c.g[S0][0];
                    }
                }
            }
        }

        c.cycles += 6;
        c.g[A1][0] = sx(0xffff_ffff_u32);
        callee::C_FE_STATE_NET_CONFIG_SET_LIST_VISIBLE.call(c, 0x001b_5058)?;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateNetQuickMessages_OnSavePopupEvent(this, args, id)` (0x1c2c50):
/// forward the save-popup event and open the matching next state.
pub fn cfe_state_net_quick_messages_on_save_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let this = abi::arg(c, 0);
        let args = abi::arg(c, 1);
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xf;
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        let is_f = c.g[A2][0] == c.g[V0][0];
        c.g[S0][0] = abi::arg64(c, 0);
        let mut next: u32 = 0;
        if !is_f {
            c.cycles += 3;
            c.g[V0][0] = 0x10;
            let is_10 = c.g[A2][0] == c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            if is_10 {
                next = 0x10;
            } else {
                c.cycles += 2;
                next = 0xdead;
            }
        }

        if next == 0 {
            // case 0xf
            c.cycles += 17;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = 0xf;
            callee::NET_FE_STATE_ON_CHILD_EVENT.call(c, 0x001c_2c88)?;
            c.g[A1][0] = 0x0046_0000;
            c.g[A0][0] = 0x23c;
            c.g[A1][0] = sx(0x0046_0000u32.wrapping_add(0xd90));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x001c_2ca0)?;
            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x10).lw(c);
            c.g[A0][0] = c.g[V0][0];
            c.g[T0][0] = 1;
            c.g[A2][0] = 2;
            c.g[A3][0] = 1;
            callee::C_FE_STATE_OPTIONS_DEVICE_SELECT_CTOR.call(c, 0x001c_2cb8)?;
        } else if next == 0x10 {
            c.cycles += 12;
            c.g[A2][0] = 0x10;
            callee::NET_FE_STATE_ON_CHILD_EVENT.call(c, 0x001c_2cc8)?;
            c.g[A1][0] = 0x0046_0000;
            c.g[A0][0] = 0x6d0;
            c.g[A1][0] = sx(0x0046_0000u32.wrapping_add(0x11a8));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, 0x001c_2ce0)?;
            c.g[A1][0] = Addr(c.g[S0][0] as u32).off(0x10).lw(c);
            c.g[A0][0] = c.g[V0][0];
            callee::C_FE_STATE_NET_SSX_WORLD_CTOR.call(c, 0x001c_2cec)?;
            c.g[A0][0] = c.g[S0][0];
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            callee::NET_FE_STATE_ON_CHILD_EVENT.call(c, 0x001c_2d08)?;
            c.cycles += 3;
            frame.restore(c, 16, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        c.cycles += 4;
        c.g[A1][0] = c.g[V0][0];
        callee::NET_FE_STATE_GOTO_NEXT_STATE.call(c, 0x001c_2cf8)?;
        c.lq(16, frame.at(c, 0x10));
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cSetupMan_FlushSendQueue(this)` (0x264b20): send the queued packets until
/// the channel refuses or the queue is empty.
pub fn c_setup_man_flush_send_queue(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x50);
        frame.save(c, 17, 0x30);
        frame.save(c, 16, 0x40);
        c.g[S1][0] = abi::arg64(c, 0);
        frame.save(c, 18, 0x20);
        frame.save(c, 19, 0x10);
        frame.save_ra(c, 0);

        c.g[V1][0] = Addr(this).off(0x94).lw(c);
        c.g[V0][0] = Addr(this).off(0x90).lw(c);
        let empty = c.g[V1][0] == c.g[V0][0];
        c.g[S3][0] = sx(0xffff_ffff_u32);
        if !empty {
            c.cycles += 3;
            c.g[S2][0] = sx(this.wrapping_add(0x98));
            c.g[A0][0] = Addr(this).off(0x94).lw(c);
            c.g[V0][0] = Addr(this).off(0x90).lw(c);
            loop {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let head_lt_tail = c.g[V0][0] == 0;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1f));
                if head_lt_tail {
                    c.cycles += 1;
                    c.g[S0][0] = 0;
                } else {
                    c.cycles += 9;
                    c.g[V1][0] = ((c.g[S3][0] as i64) < (c.g[A0][0] as i64)) as u64;
                    if c.g[V1][0] != 0 {
                        c.g[V0][0] = c.g[A0][0];
                    }
                    c.g[V0][0] = sx((((c.g[V0][0] as u32) as i32) >> 5) as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S0][0] = Addr(c.g[V0][0] as u32).lw(c);
                }

                c.cycles += 7;
                c.g[A0][0] = Addr(gp(c, -0x238)).lw(c);
                c.g[A2][0] = c.g[S0][0];
                c.g[A1][0] = Addr(this).off(0x120).lw(c);
                c.g[A3][0] = Addr(c.g[S0][0] as u32).off(2).lhu(c);
                callee::C_COMM_SYSTEM_CHANNEL_SEND.call(c, 0x0026_4ba0)?;
                let refused = c.g[V0][0] == 0;
                c.g[A1][0] = c.g[S0][0];
                if refused {
                    break;
                }
                c.cycles += 9;
                c.g[V0][0] = Addr(this).off(0x94).lw(c);
                c.g[A0][0] = c.g[S1][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                Addr(this).off(0x94).sw(c, c.g[V0][0]);
                callee::C_SETUP_MAN_FREE_PACKET.call(c, 0x0026_4bbc)?;
                c.g[V1][0] = Addr(this).off(0x94).lw(c);
                c.g[V0][0] = Addr(this).off(0x90).lw(c);
                if c.g[V1][0] == c.g[V0][0] {
                    break;
                }
                c.g[A0][0] = Addr(this).off(0x94).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0026_4b58);
                }
            }
        }

        c.cycles += 7;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore(c, 19, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `NetEventE3_Clone(src)` (0x2684c0): allocate and initialise a copy of the
/// network event.
pub fn net_event_e3_clone(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 50;
        let src = abi::arg(c, 0);
        let frame = Frame::open(c, 0x60);
        c.g[A1][0] = 0x0048_0000;
        frame.save(c, 18, 0x30);
        c.g[A2][0] = 0x2000_0000;
        c.g[S2][0] = abi::arg64(c, 0);
        frame.save(c, 16, 0x50);
        frame.save(c, 17, 0x40);
        c.g[A3][0] = 0;
        frame.save(c, 19, 0x20);
        c.g[A1][0] = sx(0x0048_0000u32.wrapping_add(0x488));
        frame.save(c, 20, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x2c;
        callee::OPERATOR_NEW_NAMED.call(c, 0x0026_84f8)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[V1][0] = 0x0048_0000;
        c.g[A1][0] = Addr(src).off(8).lw(c);
        c.g[V1][0] = sx(0x0048_0000u32.wrapping_add(0x1190));
        c.g[V0][0] = 0xe3;
        Addr(c.g[S0][0] as u32).off(4).sw(c, c.g[V1][0]);
        Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(8));
        c.g[S1][0] = Addr(src).off(0xc).lw(c);
        c.g[S3][0] = Addr(src).off(0x10).lw(c);
        c.g[S4][0] = Addr(src).off(0x14).lw(c);
        callee::C_BX_STRING_CTOR_CSTR.call(c, 0x0026_8528)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
        callee::C_BX_STRING_CTOR_CSTR.call(c, 0x0026_8534)?;
        Addr(c.g[S0][0] as u32).off(0x10).sw(c, c.g[S3][0]);
        c.g[V0][0] = c.g[S0][0];
        Addr(c.g[S0][0] as u32).off(0x14).sw(c, c.g[S4][0]);
        c.ldl(3, abi::r32(c, S2).wrapping_add(0x1f));
        c.ldr(3, abi::r32(c, S2).wrapping_add(0x18));
        c.ldl(4, abi::r32(c, S2).wrapping_add(0x27));
        c.ldr(4, abi::r32(c, S2).wrapping_add(0x20));
        c.g[A1][0] = Addr(c.g[S2][0] as u32).off(0x28).lw(c);
        c.sdl(3, abi::r32(c, S0).wrapping_add(0x1f));
        c.sdr(3, abi::r32(c, S0).wrapping_add(0x18));
        c.sdl(4, abi::r32(c, S0).wrapping_add(0x27));
        c.sdr(4, abi::r32(c, S0).wrapping_add(0x20));
        Addr(c.g[S0][0] as u32).off(0x28).sw(c, c.g[A1][0]);

        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore(c, 20, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `SSXAudioSystem_RiderSay20BC(this, id)` (0x2a16b0): speak the rider's line
/// if speech is on and the voice instance is ready.
pub fn ssx_audio_system_rider_say_20bc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0);
        callee::SSX_AUDIO_SYSTEM_IS_SPEECH_VOLUME_ON.call(c, 0x002a_16cc)?;
        let off = c.g[V0][0] == 0;
        c.g[S1][0] = abi::arg64(c, 1);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5560));

        // 0: epilogue straight, 1: exit through 'b_2a1764 (one extra cycle).
        let mut extra = false;
        if off {
            extra = true;
        } else {
            c.cycles += 5;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[S0][0];
            callee::SPEECH_SYSTEM_IS_INSTANCE_READY.call(c, 0x002a_16e0)?;
            if c.g[V0][0] == 0 {
                c.lq(16, frame.at(c, 0x20));
            } else {
                c.cycles += 9;
                callee::BE_LIBRARY_GET_INSTANCE.call(c, 0x002a_16f0)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                callee::BE_LIBRARY_GET_INTERFACE.call(c, 0x002a_16fc)?;
                c.g[V1][0] = 0x0053_0000;
                c.g[V0][0] = Addr(0x0053_0000u32.wrapping_add(0x5c11)).lb(c);
                let gate = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[S0][0];
                if gate {
                    extra = true;
                } else {
                    c.cycles += 10;
                    c.mtc1(0, 12);
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 0x20bc;
                    c.g[A3][0] = 0;
                    c.g[T0][0] = c.g[S1][0];
                    c.g[T1][0] = 0;
                    c.g[T2][0] = 1;
                    callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT.call(c, 0x002a_172c)?;
                    let failed = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[S1][0];
                    if failed {
                        extra = true;
                    } else {
                        c.cycles += 12;
                        c.g[A1][0] = 0;
                        callee::SSX_AUDIO_SYSTEM_CHARACTER_TO_SPEECH_MASK.call(c, 0x002a_173c)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A0][0] = 1;
                        c.g[A1][0] = 0;
                        c.g[A2][0] = 0x20bc;
                        callee::SPCH_MAKE_ID.call(c, 0x002a_1750)?;
                        c.g[V1][0] = Addr(gp(c, 0x173c)).lw(c);
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A2][0] = c.g[S0][0];
                        let target = c.g[V1][0] as u32;
                        c.g[A1][0] = 1;
                        call_ptr(c, target, 0x002a_1764)?;
                        extra = true;
                    }
                }
            }
        }

        if extra {
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x20));
        }
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        let _ = this;
        Ok(())
    })
}

/// `RiderScriptObj_AcceptCommand(this, msg, type)` (0x280458): accept a
/// script command for the rider when it is loaded and its team mask matches.
pub fn rider_script_obj_accept_command(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let this = abi::arg(c, 0);
        let msg = abi::arg(c, 1);
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 5;
        frame.save(c, 18, 0x20);
        frame.save(c, 19, 0x10);
        c.g[S2][0] = abi::arg64(c, 0);
        frame.save(c, 16, 0x40);
        c.g[S3][0] = abi::arg64(c, 1);
        frame.save(c, 17, 0x30);
        let not_five = c.g[A2][0] != c.g[V0][0];
        frame.save_ra(c, 0);

        if not_five {
            c.cycles += 1;
            c.g[V0][0] = 0;
            c.cycles += 1;
        } else {
            c.cycles += 10;
            c.g[V1][0] = Addr(this).lw(c);
            c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x30).lh(c);
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x34).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = sx(this.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0028_0494)?;
            c.g[S0][0] = c.g[V0][0];
            callee::RIDER_IS_LOADED.call(c, 0x0028_04a0)?;
            let not_loaded = c.g[V0][0] == 0;
            c.g[V0][0] = 0;
            if not_loaded {
                c.cycles += 1;
                c.lq(16, frame.at(c, 0x40));
            } else {
                c.cycles += 13;
                callee::RIDER_GET_CHARACTER.call(c, 0x0028_04b0)?;
                c.g[V1][0] = 0x0048_0000;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V1][0] = sx(0x0048_0000u32.wrapping_add(0x1bb0));
                c.g[A0][0] = c.g[S3][0];
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[S1][0] = Addr(c.g[V0][0] as u32).lw(c);
                callee::SCRIPT_RIDER_GET_PARAMS.call(c, 0x0028_04cc)?;
                c.g[V1][0] = c.g[V0][0];
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(2).lh(c);
                let no_mask = c.g[V0][0] == 0;
                c.g[V0][0] = c.g[V0][0] & c.g[S1][0];
                if no_mask {
                    // -> the MatchesRiderSlot tail
                } else {
                    c.cycles += 2;
                    let masked_off = c.g[V0][0] == 0;
                    c.g[V0][0] = 0;
                    if masked_off {
                        c.cycles += 1;
                        c.lq(16, frame.at(c, 0x40));
                        c.cycles += 6;
                        frame.restore(c, 17, 0x30);
                        frame.restore(c, 18, 0x20);
                        frame.restore(c, 19, 0x10);
                        frame.restore_ra(c, 0);
                        frame.close(c);
                        return Ok(());
                    }
                }
                c.cycles += 7;
                c.g[A2][0] = Addr(c.g[V1][0] as u32).lb(c);
                c.g[A0][0] = c.g[S2][0];
                c.g[A3][0] = Addr(c.g[S3][0] as u32).off(8).lw(c);
                c.g[A1][0] = c.g[S0][0];
                callee::RIDER_SCRIPT_OBJ_MATCHES_RIDER_SLOT.call(c, 0x0028_04f8)?;
                c.lq(16, frame.at(c, 0x40));
            }
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

/// `BankMonitor_StopByCallback(this, key)` (0x2ad410): reset every 0xc0-byte
/// bank-monitor instance whose key matches.
pub fn bank_monitor_stop_by_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x80);
        frame.save(c, 16, 0x70);
        frame.save(c, 17, 0x60);
        c.g[S0][0] = abi::arg64(c, 0);
        frame.save(c, 18, 0x50);
        c.g[S1][0] = 0x3f;
        frame.save(c, 19, 0x40);
        c.g[S2][0] = 0;
        frame.save(c, 20, 0x30);
        c.g[S3][0] = 0;
        c.g[S4][0] = sx(0xffff_ffff_u32);
        frame.save_ra(c, 0x20);
        c.sd(frame.at(c, 0), abi::arg64(c, 1));
        let _ = this;

        loop {
            c.cycles += 12;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x8ec).lw(c);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            c.ldl(3, frame.at(c, 7));
            c.ldr(3, frame.at(c, 0));
            c.sdl(3, frame.at(c, 0x17));
            c.sdr(3, frame.at(c, 0x10));
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A1][0] = 0;
            c.g[V1][0] = Addr(frame.at(c, 0x12)).lh(c);
            c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0xaa).lh(c);
            let differs = c.g[V1][0] != c.g[V0][0];
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc0));
            if !differs {
                c.cycles += 2;
                let is_minus1 = c.g[V1][0] == c.g[S4][0];
                c.g[V1][0] = Addr(frame.at(c, 0x14)).lh(c);
                if !is_minus1 {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(c.g[A0][0] as u32).off(0xac).lh(c);
                    if c.g[V1][0] == c.g[V0][0] {
                        c.g[A1][0] = 1;
                    }
                }
            }
            if !(c.g[A1][0] != 0) {
                c.cycles += 4;
                c.g[V1][0] = Addr(c.g[A0][0] as u32).off(0xac).lw(c);
                c.g[V0][0] = Addr(frame.at(c, 0x14)).lw(c);
                if c.g[V0][0] == c.g[V1][0] {
                    c.g[A1][0] = 1;
                }
            }

            c.cycles += 2;
            if c.g[A1][0] != 0 {
                c.cycles += 3;
                c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0x8ec).lw(c);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S3][0] as u32));
                callee::BANK_MONITOR_INSTANCE_RESET.call(c, 0x002a_d4b0)?;
            }

            c.cycles += 2;
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0xc0));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x002a_d448);
            }
        }

        c.cycles += 8;
        frame.restore(c, 16, 0x70);
        frame.restore(c, 17, 0x60);
        frame.restore(c, 18, 0x50);
        frame.restore(c, 19, 0x40);
        frame.restore(c, 20, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `cFEPopupConfirm_GetWidgetText(this, dst, text)` (0x1c8258): copy the
/// widget's text (ASCII or wide) into the string, or the default prompt.
pub fn cfe_popup_confirm_get_widget_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let this = abi::arg(c, 0);
        let dst = abi::arg(c, 1);
        let text = abi::arg(c, 2);
        let frame = Frame::open(c, 0x40);
        frame.save(c, 16, 0x30);
        c.g[S0][0] = abi::arg64(c, 2);
        frame.save(c, 18, 0x10);
        c.g[S2][0] = abi::arg64(c, 1);
        frame.save(c, 17, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        callee::UI_TEXT_GET_OWNED_STRING.call(c, 0x001c_827c)?;

        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[V0][0];
            callee::C_BX_STRING_ASSIGN_CSTR.call(c, 0x001c_828c)?;
            c.lq(16, frame.at(c, 0x30));
            c.cycles += 5;
            frame.restore(c, 17, 0x20);
            frame.restore(c, 18, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        // No ASCII text: try the wide string.
        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        callee::UI_TEXT_GET_ID_STRING.call(c, 0x001c_829c)?;
        c.g[S1][0] = c.g[V0][0];
        if c.g[S1][0] == 0 {
            c.g[A1][0] = 0x004a_0000;
            // -> the shared default-string tail
            c.cycles += 4;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = sx(0x004a_0000u32.wrapping_add(0x1408));
            callee::C_BX_STRING_ASSIGN_CSTR.call(c, 0x001c_8308)?;
            c.lq(16, frame.at(c, 0x30));
            c.cycles += 5;
            frame.restore(c, 17, 0x20);
            frame.restore(c, 18, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        c.cycles += 17;
        c.g[A0][0] = c.g[S1][0];
        callee::U_STR_LEN.call(c, 0x001c_82b0)?;
        c.g[A1][0] = 0x0046_0000;
        c.g[A2][0] = 0x100;
        c.g[A3][0] = 0;
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
        c.g[A1][0] = sx(0x0046_0000u32.wrapping_add(0x60b8));
        callee::OPERATOR_NEW_NAMED.call(c, 0x001c_82c8)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        callee::U_STR_TO_ASCII.call(c, 0x001c_82d8)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S0][0];
        callee::C_BX_STRING_ASSIGN_CSTR.call(c, 0x001c_82e4)?;
        if c.g[S0][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, addr::EA_MEM_FREE, 0x001c_82f4)?;
        }
        c.lq(16, frame.at(c, 0x30));
        c.cycles += 5;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        let _ = this;
        Ok(())
    })
}

/// `MPEG2_group_of_pictures_header()` (0x3b16e0): read the group-of-pictures
/// header fields from the bit reader.
pub fn mpeg2_group_of_pictures_header(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        c.g[V0][0] = 0x0051_0000;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 0x0051_0000;
        c.g[A0][0] = Addr(0x0051_0000u32.wrapping_sub(0x5f78)).lw(c);
        c.g[V1][0] = sx(0x0051_0000u32.wrapping_sub(0x69b8));
        let same = c.g[A0][0] == c.g[V1][0];
        frame.save_ra(c, 0);
        if same {
            c.cycles += 8;
            c.g[V1][0] = 0x0045_0000;
            c.g[A1][0] = 0x0045_0000;
            c.g[V0][0] = Addr(0x0045_0000u32.wrapping_sub(0x3c84)).lw(c);
            c.g[A0][0] = 0x0045_0000;
            c.g[V1][0] = 1;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
            Addr(0x0045_0000u32.wrapping_sub(0x3c80)).sw(c, c.g[V1][0]);
            Addr(0x0045_0000u32.wrapping_sub(0x3c88)).sw(c, c.g[V0][0]);
        }

        c.cycles += 35;
        c.g[A0][0] = 1;
        callee::MPEG2_GET_BITS.call(c, 0x003b_1724)?;
        c.g[V1][0] = 0x0051_0000;
        c.g[A0][0] = 5;
        Addr(0x0051_0000u32.wrapping_sub(0x69d8)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_1734)?;
        c.g[V1][0] = 0x0051_0000;
        c.g[A0][0] = 6;
        Addr(0x0051_0000u32.wrapping_sub(0x69d4)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_1744)?;
        c.g[V1][0] = 0x0051_0000;
        c.g[A0][0] = 0x0049_0000;
        Addr(0x0051_0000u32.wrapping_sub(0x69d0)).sw(c, c.g[V0][0]);
        c.g[A0][0] = sx(0x0049_0000u32.wrapping_add(0x5468));
        callee::MPEG2_MARKER_BIT.call(c, 0x003b_1758)?;
        c.g[A0][0] = 6;
        callee::MPEG2_GET_BITS.call(c, 0x003b_1760)?;
        c.g[V1][0] = 0x0051_0000;
        c.g[A0][0] = 6;
        Addr(0x0051_0000u32.wrapping_sub(0x69cc)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_1770)?;
        c.g[V1][0] = 0x0051_0000;
        c.g[A0][0] = 1;
        Addr(0x0051_0000u32.wrapping_sub(0x69c8)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_1780)?;
        c.g[V1][0] = 0x0051_0000;
        c.g[A0][0] = 1;
        Addr(0x0051_0000u32.wrapping_sub(0x69c4)).sw(c, c.g[V0][0]);
        callee::MPEG2_GET_BITS.call(c, 0x003b_1790)?;
        c.g[V1][0] = 0x0051_0000;
        Addr(0x0051_0000u32.wrapping_sub(0x69c0)).sw(c, c.g[V0][0]);
        callee::MPEG2_EXTENSION_AND_USER_DATA.call(c, 0x003b_179c)?;

        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateCharEquip_OnChildEvent(this, widget, msg)` (0x199870): keep the
/// equip screen's dirty flag in step with the selected item and buy it.
pub fn cfe_state_char_equip_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x15;
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        let is_15 = c.g[A2][0] == c.g[V0][0];
        c.g[S0][0] = abi::arg64(c, 0);

        if is_15 {
            c.cycles += 4;
            c.g[V1][0] = Addr(abi::r32(c, A1)).off(0xc).lw(c);
            c.g[V0][0] = 7;
            let selected = c.g[V1][0] == c.g[V0][0];
            c.g[V1][0] = sx(0xffff_fff7_u32);
            if !selected {
                c.cycles += 4;
                c.g[V0][0] = Addr(abi::r32(c, S0)).off(0x1c).lw(c);
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                Addr(abi::r32(c, S0)).off(0x1c).sw(c, c.g[V0][0]);
            }
            // 'b_199924 tail
            c.cycles += 1;
            c.lq(16, frame.at(c, 0x10));
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        c.cycles += 3;
        c.g[V0][0] = 0x16;
        if c.g[A2][0] != c.g[V0][0] {
            c.cycles += 2;
            c.lq(16, frame.at(c, 0x10));
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        // msg 0x16
        c.g[V1][0] = Addr(c.g[S0][0] as u32).off(0x1c).lw(c);
        c.cycles += 4;
        c.g[V0][0] = sx((((c.g[V1][0] as u32) as i32) >> 3) as u32);
        c.g[V0][0] = c.g[V0][0] & 1;
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = c.g[V1][0] | 8;
            Addr(c.g[S0][0] as u32).off(0x1c).sw(c, c.g[V0][0]);
        }
        c.g[V1][0] = Addr(abi::r32(c, A1)).off(0xc).lw(c);

        c.cycles += 3;
        c.g[V0][0] = 7;
        if c.g[V1][0] != c.g[V0][0] {
            c.lq(16, frame.at(c, 0x10));
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(abi::r32(c, A1)).off(0x6c).lw(c);
        if c.g[V0][0] == 0 {
            c.lq(16, frame.at(c, 0x10));
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }

        c.cycles += 10;
        c.g[A0][0] = c.g[S0][0];
        callee::C_FE_STATE_CHAR_EQUIP_BUY_SELECTED_ITEM.call(c, 0x0019_98f8)?;
        c.g[A0][0] = c.g[S0][0];
        callee::C_FE_STATE_CHAR_EQUIP_UPDATE_MONEY_TEXT.call(c, 0x0019_9900)?;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x5c).lw(c);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = Addr(c.g[V0][0] as u32).off(0x95).lbu(c);
        callee::C_FE_STATE_CHAR_EQUIP_REMOVE_LIST_ITEM.call(c, 0x0019_9910)?;
        let removed = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[S0][0];
        if !removed {
            c.cycles += 3;
            c.g[A1][0] = 0;
            c.g[A2][0] = 1;
            callee::C_FE_STATE_CHAR_EQUIP_REFRESH_ITEM_LIST.call(c, 0x0019_9924)?;
        }

        c.cycles += 1;
        c.lq(16, frame.at(c, 0x10));
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateCharEquip_ShowBuyMode(this)` (0x19a308): relabel the money text
/// and dismiss the three item panels before showing the rider preview.
pub fn cfe_state_char_equip_show_buy_mode(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = abi::arg64(c, 0);
        c.g[V0][0] = Addr(this).off(0x48).lw(c);
        let no_text = c.g[V0][0] == 0;
        c.g[A0][0] = 0x0046_0000;
        if !no_text {
            c.cycles += 5;
            c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(0x810));
            callee::STR_HASH.call(c, 0x0019_a32c)?;
            c.g[A0][0] = Addr(this).off(0x48).lw(c);
            c.g[A1][0] = c.g[V0][0];
            callee::UI_TEXT_SET_TEXT_ID.call(c, 0x0019_a338)?;
        }

        // The item panel at +0x80 (close with 0).
        c.cycles += 3;
        c.g[A2][0] = Addr(this).off(0x80).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 0;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(8).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x48).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x4c).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_a358)?;
        }

        // The item panel at +0x6c.
        c.cycles += 3;
        c.g[A2][0] = Addr(this).off(0x6c).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        if !none {
            c.cycles += 7;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(8).lw(c);
            c.g[A1][0] = 0;
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x48).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x4c).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_a37c)?;
            c.g[V0][0] = Addr(gp(c, -0x848)).lw(c);
        }

        // Hide the rider preview, then the panel at +0x60 (close with 1).
        c.cycles += 10;
        c.g[A1][0] = Addr(this).off(0x44).lb(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x7c).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xb0));
        callee::FE_MODULE_GET_RIDER_PREVIEW.call(c, 0x0019_a390)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE.call(c, 0x0019_a39c)?;
        c.g[A2][0] = Addr(this).off(0x60).lw(c);
        let none = c.g[A2][0] == 0;
        c.g[A1][0] = 1;
        if !none {
            c.cycles += 5;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(8).lw(c);
            c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x48).lh(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x4c).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x0019_a3bc)?;
        }

        c.cycles += 4;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ==== tests ====

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{fake_call, machine, vtable, FAKE, LOG_RESULTS};
    use crate::difftest::{check, Rng, DATA, GAME_GP};

    const SEEDS: u32 = 200;

    /// The BELibrary dummy and the world/race-manager/rider graph the
    /// front-end and overlay functions read.
    const BE: u32 = DATA + 0x0f_0000;
    const WORLD: u32 = DATA + 0x10_0000;
    const RMAN: u32 = DATA + 0x11_0000;
    const RIDERS: u32 = DATA + 0x12_0000;
    const RIDER: u32 = DATA + 0x13_0000;
    /// The BELibrary interface table (`.data` at 0x5308a8, indexed by
    /// `gp-0x1efc`).
    const IFACE_TABLE: u32 = 0x0053_08a8;

    fn game_globals(c: &mut Cpu) {
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(GAME_GP - 0x1f04, BE);
        c.bus.write32(GAME_GP - 0x848, WORLD);
        c.bus.write32(WORLD + 0x84, RMAN);
        c.bus.write32(RMAN + 0xc, RIDERS);
        c.bus.write32(RIDERS + 0x28, RIDER);
    }

    /// `BELibrary_GetInstance` returns the dummy; `GetInterface(dummy, 1)`
    /// finds an object whose slot 8 is a fake method.
    fn be_library(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(GAME_GP - 0x1efc, 0);
        let iface = DATA + 0x14_0000;
        c.bus.write32(IFACE_TABLE, iface);
        let vt = vtable(c, r, &[8]);
        c.bus.write32(iface + 4, vt);
    }

    /// A fake that returns a small value instead of a pointer, keyed by slot.
    fn fake_small(c: &mut Cpu, target: u32) {
        fake_call(c, target);
        let h = c.g[V1][0];
        c.g[V0][0] = match target.wrapping_sub(FAKE) {
            0 => ((h >> 3) & 1) as u64,
            0x10 => ((h >> 5) & 1) as u64,
            0x20 => match (h >> 9) & 3 {
                0 => (h >> 11) & 7,
                1 => sx(0xffff_fffd_u32),
                2 => sx(0xffff_fffe_u32),
                _ => sx(0xffff_fff0_u32),
            },
            0x30 => ((h >> 11) & 1) as u64,
            _ => 0,
        };
    }

        #[test]
    fn cfe_use_existing_account_on_email_popup_event_matches() {
        check(
            ee::s03::f_001ac9c0__generated,
            cfe_use_existing_account_on_email_popup_event,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1ac9, 4);
                game_globals(c);
                let this = DATA;
                let args = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = args as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    0xf
                } else {
                    1 + (seed % 14) as u64
                };
                let name = DATA + 0x3000;
                c.bus.write32(args + 0x2b0, name);
                c.bus.write8(name, 0);
                c.bus.write32(args + 0x18, 0);
            },
        );
    }

                    #[test]
    fn script_engine_v06_matches() {
        check(ee::s07::f_00278ad0__generated, script_engine_v06, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x278a, 4);
            c.hooks.miss = fake_small;
            game_globals(c);
            let this = DATA;
            let slot = DATA + 0x2000;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = 0x1000 + (seed % 4) as u64;
            c.bus.write32(this + 0x2a8, slot);
            c.bus.write32(this + 0x6d0, 1);
            c.bus.write32(this + 0x7f0, 2);
            let vt = vtable(c, &mut r, &[0x20]);
            c.bus.write32(slot + 0x24, vt.wrapping_add(4));
        });
    }

    #[test]
    fn script_engine_remove_script_obj_matches() {
        check(
            ee::s07::f_0027cf88__generated,
            script_engine_remove_script_obj,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x27cf, 4);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                let obj = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 4) as u64;
                if seed % 3 != 0 {
                    c.bus.write32(this + 4 * (seed % 4), obj);
                    let vt = vtable(c, &mut r, &[0x30, 0x38]);
                    c.bus.write32(obj + 0xc, vt);
                }
            },
        );
    }

    #[test]
    fn music_player_stop_song_matches() {
        check(
            ee::s08::f_002b3ac0__generated,
            music_player_stop_song,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2b3a, 4);
                game_globals(c);
                let this = DATA;
                let song = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.bus.write32(this, DATA + 0x3000);
                c.bus.write32(DATA + 0x3000, DATA + 0x3400);
                c.bus.write32(DATA + 0x3400 + 0x1d8, 0);
                for (i, a) in [0x004a_3838u32, 0x004a_3840, 0x004a_3848]
                    .iter()
                    .enumerate()
                {
                    c.bus.write64(*a, 0x1000 * (i as u64 + 1));
                }
                c.bus.write32(this + 0x418, 0);
                if seed % 2 == 0 {
                    c.bus.write32(this + 0x408, 0);
                } else {
                    c.bus.write32(this + 0x408, song);
                    c.bus.write32(song + 0x34, seed % 5);
                }
            },
        );
    }

    #[test]
    fn ps2_graphics_man_set_brightness_rgb_matches() {
        check(
            ee::s12::f_0037c570__generated,
            ps2_graphics_man_set_brightness_rgb,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x37c5, 0);
                game_globals(c);
                let this = DATA;
                let rgb = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = rgb as u64;
                match seed % 4 {
                    0 => {
                        c.bus.write32(rgb, 0x3f00_0000);
                        c.bus.write32(rgb + 4, 0x4000_0000);
                        c.bus.write32(rgb + 8, 0xbf80_0000);
                    }
                    1 => {
                        c.bus.write32(rgb, r.edge_f32());
                        c.bus.write32(rgb + 4, r.edge_f32());
                        c.bus.write32(rgb + 8, r.edge_f32());
                    }
                    _ => {
                        c.bus.write32(rgb, r.f32(1.5));
                        c.bus.write32(rgb + 4, r.f32(1.5));
                        c.bus.write32(rgb + 8, r.f32(1.5));
                    }
                }
            },
        );
    }

    #[test]
    fn c_keyboard_popup_move_down_from_bottom_row_matches() {
        check(
            ee::s04::f_001cbce8__generated,
            c_keyboard_popup_move_down_from_bottom_row,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1cbc, 4);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 2) as u64;
                c.bus.write32(
                    this + 0x140,
                    match seed % 3 {
                        0 => 0x4c,
                        1 => 0x4b,
                        _ => 0x38,
                    },
                );
                let _ = &mut r;
            },
        );
    }

            #[test]
    fn wscript_man_call_table_function_matches() {
        check(
            ee::s10::f_00309e50__generated,
            wscript_man_call_table_function,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x309e, 4);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = r.next_u32() as u64;
                let entry = DATA + 0x2000;
                for i in 0..4 {
                    c.bus.write32(this + 0x3cc + 4 * i, entry + 0x40 * i);
                }
                c.bus.write32(LOG_RESULTS, 0);
            },
        );
    }

    #[test]
    fn debug_menu_module_shutdown_matches() {
        check(
            ee::s02::f_0017fad0__generated,
            debug_menu_module_shutdown,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x17fa, 4);
                game_globals(c);
                c.bus.write32(GAME_GP - 0x854, DATA + 0x16_0000);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let o1 = DATA + 0x2000;
                let o2 = DATA + 0x2400;
                c.bus.write32(this + 0xc, o1);
                c.bus.write32(this + 0x10, o2);
                let vt1 = vtable(c, &mut r, &[8]);
                let vt2 = vtable(c, &mut r, &[8]);
                c.bus.write32(o1, vt1);
                c.bus.write32(o2, vt2);
                c.bus.write32(
                    this + 0x20,
                    if seed % 2 == 0 { 0xff_ffff } else { seed % 4 },
                );
                c.bus.write32(
                    this + 0x1c,
                    if seed % 3 == 0 {
                        0xff_ffff
                    } else {
                        1 + seed % 4
                    },
                );
                c.bus
                    .write32(this + 0x18, if seed % 4 == 0 { 0 } else { DATA + 0x2800 });
                let vt = vtable(c, &mut r, &[0x190]);
                let m = DATA + 0x16_0000;
                c.bus.write32(m + 0x10d8, vt);
            },
        );
    }

    #[test]
    fn cfe_state_preview_reward_ctor_matches() {
        check(
            ee::s04::f_001d1aa0__generated,
            cfe_state_preview_reward_ctor,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1d1a, 3);
                game_globals(c);
                be_library(c, &mut r);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = (DATA + 0x2400) as u64;
                c.g[A3][0] = (seed % 4) as u64;
                c.bus.write32(this + 0x10, DATA + 0x2800);
                c.bus.write32(DATA + 0x2800, DATA + 0x2c00);
            },
        );
    }

        #[test]
    fn intersect_info_mul_matrix_matches() {
        check(
            ee::s11::f_00351948__generated,
            intersect_info_mul_matrix,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3519, 0);
                game_globals(c);
                let this = DATA;
                let m = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = m as u64;
                for i in 0..32 {
                    c.bus.write32(m + 4 * i, r.edge_f32());
                    c.bus.write32(this + 0x90 + 4 * i, r.edge_f32());
                }
                let _ = seed;
            },
        );
    }

    #[test]
    fn mpeg2_picture_spatial_scalable_extension_matches() {
        check(
            ee::s13::f_003b2230__generated,
            mpeg2_picture_spatial_scalable_extension,
            SEEDS,
            &|c, seed| {
                let _ = machine(c, seed, 0x3b22, 0);
                game_globals(c);
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
    fn snd_bank_set_memory_matches() {
        check(
            ee::s07::f_0028b588__generated,
            snd_bank_set_memory,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x28b5, 3);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = ((seed % 3) as u64) * 0x100;
                c.g[A2][0] = if seed % 2 == 0 {
                    0
                } else {
                    ((seed % 4) as u64) * 0x40
                };
                c.g[A3][0] = (DATA + 0x2000) as u64;
                let obj = DATA + 0x2400;
                c.bus.write32(GAME_GP + 0x524, obj);
                let vt = vtable(c, &mut r, &[0x20]);
                c.bus.write32(obj + 0x1d8, DATA + 0x2800);
                c.bus.write32(DATA + 0x2800, vt);
            },
        );
    }

    #[test]
    fn wscript_process_update_matches() {
        check(
            ee::s10::f_003077f8__generated,
            wscript_process_update,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3077, 3);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.bus.write32(this, (seed % 4) as u32);
                c.bus.write32(this + 4, (seed % 3) as u32);
                c.bus.write32(this + 8, (seed % 4) as u32);
                c.bus.write32(this + 0x14, seed % 2);
                c.bus.write32(this + 0x40, DATA + 0x2000);
                let vt = vtable(c, &mut r, &[0x28]);
                c.bus.write32(this + 0x5c, vt);
            },
        );
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
    fn c_transition_man_script_activate_matches() {
        check(
            ee::s09::f_002e4b98__generated,
            c_transition_man_script_activate,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2e4b, 3);
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.bus.write32(
                    this + 0x28,
                    if seed % 2 == 0 { 7 } else { (seed % 8) as u32 },
                );
                c.bus.write32(this + 0x38, seed % 4);
                c.bus.write32(this + 0x34, r.f32(1.0));
                c.bus.write32(this + 0x30, r.f32(1.0));
                c.bus.write32(this + 0x2c, r.f32(1.0));
                c.bus.write32(GAME_GP - 0x84c, DATA + 0x2000);
            },
        );
    }

        #[test]
    fn cfe_state_trophy_room_on_widget_action_matches() {
        check(
            ee::s04::f_001d4698__generated,
            cfe_state_trophy_room_on_widget_action,
            SEEDS,
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
    fn fe_module_rider_preview_unload_matches() {
        check(
            ee::s03::f_0019eba0__generated,
            fe_module_rider_preview_unload,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x19eb, 4);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.bus
                    .write32(this + 8, if seed % 3 == 0 { 0 } else { DATA + 0x2000 });
                for (i, off) in [0xcu32, 0xc74, 0xc70].iter().enumerate() {
                    if seed % (i as u32 + 2) != 0 {
                        let o = DATA + 0x2400 + 0x100 * i as u32;
                        c.bus.write32(this + *off, o);
                        let slot = match *off {
                            0xc => 0x58,
                            0xc74 => 0xc4,
                            _ => 0xa4,
                        };
                        let vt = vtable(c, &mut r, &[slot]);
                        c.bus.write32(o + slot, vt);
                    }
                }
            },
        );
    }

    #[test]
    fn ui_icon_load_matches() {
        check(ee::s13::f_00398998__generated, ui_icon_load, SEEDS, &|c, seed| {
            let mut r = machine(c, seed, 0x3989, 3);
            c.hooks.miss = fake_small;
            game_globals(c);
            let this = DATA;
            let data = DATA + 0x2000;
            c.g[A0][0] = this as u64;
            c.g[A1][0] = data as u64;
            c.bus.write32(this + 0x74, r.next_u32());
            c.bus.write32(this + 0x5c, DATA + 0x2400);
            c.bus.write32(data + 0x20, r.next_u32());
            c.bus.write32(data + 0x24, seed % 4);
            c.bus.write16(data + 0x28, seed as u16);
            c.bus.write16(data + 0x2a, (seed * 3) as u16);
            c.bus.write32(GAME_GP - 0x2548, 0.01f32.to_bits());
            c.bus.write32(DATA + 0x2400 + 0xd0, DATA + 0x2800);
            c.bus.write32(DATA + 0x2800 + 0x10, DATA + 0x2c00);
        });
    }

        #[test]
    fn cfe_state_net_help_popup_update_matches() {
        check(
            ee::s04::f_001d9e68__generated,
            cfe_state_net_help_popup_update,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1d9e, 3);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                for i in 0..2u32 {
                    if seed % (i + 2) != 0 {
                        let bind = DATA + 0x2000 + 0x40 * i;
                        c.bus.write32(WORLD + 0xb0 + 4 * i, bind);
                        c.bus.write32(bind, DATA + 0x2400 + 0x40 * i);
                        c.bus.write32(DATA + 0x2400 + 0x40 * i, (seed % 2) as u32);
                    }
                }
                let _ = &mut r;
            },
        );
    }

    #[test]
    fn cfe_state_mem_card_show_not_formatted_msg_matches() {
        check(
            ee::s02::f_00186c98__generated,
            cfe_state_mem_card_show_not_formatted_msg,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x186c, 4);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.bus.write32(RMAN, 0);
                let mc = DATA + 0x2000;
                c.bus.write32(WORLD + 0x8c, mc);
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

        #[test]
    fn script_sound_bank_man_ctor_matches() {
        check(
            ee::s07::f_00283450__generated,
            script_sound_bank_man_ctor,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2834, 3);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (seed % 3) as u64;
                c.g[A3][0] = if seed % 2 == 0 { 0 } else { (seed % 5) as u64 };
                c.bus.write32(LOG_RESULTS, 0);
                let _ = &mut r;
            },
        );
    }

    #[test]
    fn ov_screen_replay_menu_update_progress_bar_matches() {
        check(
            ee::s05::f_0020e818__generated,
            ov_screen_replay_menu_update_progress_bar,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x20e8, 3);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let obj = DATA + 0x2000;
                c.bus.write32(this + 0x40, obj);
                for i in 0..3 {
                    c.bus.write32(obj + 0x44 + 4 * i, r.next_u32());
                }
                let replay = DATA + 0x2400;
                c.bus.write32(RMAN + 0x28, replay);
                c.bus.write32(
                    replay + 0x3cc,
                    if seed % 3 == 0 {
                        0
                    } else {
                        (seed % 100) as u32
                    },
                );
                c.bus.write32(replay + 0x484, seed % 100);
                c.bus.write32(GAME_GP - 0x54f8, 315.0f32.to_bits());
            },
        );
    }

    #[test]
    fn fe_async_manager_request_session_texture_matches() {
        check(
            ee::s03::f_001a37f8__generated,
            fe_async_manager_request_session_texture,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1a37, 3);
                game_globals(c);
                be_library(c, &mut r);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = if seed % 3 == 0 {
                    0xff_ffff
                } else {
                    (seed % 6) as u64
                };
                c.g[A2][0] = (seed % 8) as u64;
                c.g[A3][0] = (seed % 5) as u64;
            },
        );
    }

    #[test]
    fn cfe_state_net_config_show_checking_card_popup_matches() {
        check(
            ee::s03::f_001b47c0__generated,
            cfe_state_net_config_show_checking_card_popup,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1b47, 4);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                let mc = DATA + 0x2400;
                c.bus.write32(WORLD + 0x8c, mc);
                let obj = DATA + 0x2800;
                c.bus.write32(mc + 4, obj);
                let vt = vtable(c, &mut r, &[0x20]);
                c.bus.write32(obj, vt);
                c.bus.write32(obj + 0x20, 0);
            },
        );
    }

        #[test]
    fn cfe_state_net_quick_messages_on_save_popup_event_matches() {
        check(
            ee::s03::f_001c2c50__generated,
            cfe_state_net_quick_messages_on_save_popup_event,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1c2c, 4);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                c.g[A2][0] = match seed % 3 {
                    0 => 0xf,
                    1 => 0x10,
                    _ => 3,
                };
                c.bus.write32(this + 0x10, DATA + 0x2400);
                let _ = &mut r;
            },
        );
    }

    #[test]
    fn c_setup_man_flush_send_queue_matches() {
        check(
            ee::s07::f_00264b20__generated,
            c_setup_man_flush_send_queue,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x264b, 3);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                let tail = (seed % 3) as u32;
                c.bus.write32(this + 0x90, tail);
                c.bus.write32(this + 0x94, 0);
                c.bus.write32(this + 0x120, seed % 4);
                for i in 0..tail {
                    let pkt = DATA + 0x2000 + 0x40 * i;
                    c.bus.write32(this + 0x98 + 4 * i, pkt);
                    c.bus.write16(pkt + 2, (seed + i) as u16);
                }
                c.bus.write32(GAME_GP - 0x238, DATA + 0x2800);
            },
        );
    }

            #[test]
    fn rider_script_obj_accept_command_matches() {
        check(
            ee::s07::f_00280458__generated,
            rider_script_obj_accept_command,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2804, 4);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                let msg = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = msg as u64;
                c.g[A2][0] = if seed % 3 == 0 { 5 } else { (seed % 9) as u64 };
                let obj = DATA + 0x2400;
                c.bus.write32(this, obj);
                c.bus.write32(obj + 0x30, 0);
                c.bus.write32(obj + 0x34, FAKE);
                c.bus.write32(msg + 8, seed % 5);
                let _ = &mut r;
            },
        );
    }

        #[test]
    fn cfe_popup_confirm_get_widget_text_matches() {
        check(
            ee::s03::f_001c8258__generated,
            cfe_popup_confirm_get_widget_text,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1c82, 3);
                game_globals(c);
                let this = DATA;
                let dst = DATA + 0x2000;
                let text = DATA + 0x2400;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = dst as u64;
                c.g[A2][0] = text as u64;
                c.bus.write32(text + 0x0, seed % 4);
                c.bus.write32(text + 0x4, seed % 3);
                let _ = &mut r;
            },
        );
    }

    #[test]
    fn cfe_state_char_equip_on_child_event_matches() {
        check(
            ee::s03::f_00199870__generated,
            cfe_state_char_equip_on_child_event,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1998, 4);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                let widget = DATA + 0x2000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = widget as u64;
                c.g[A2][0] = match seed % 3 {
                    0 => 0x15,
                    1 => 0x16,
                    _ => 7,
                };
                c.bus.write32(widget + 0xc, (seed % 9) as u32);
                c.bus
                    .write32(widget + 0x6c, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(this + 0x1c, r.next_u32());
                c.bus.write32(this + 0x5c, DATA + 0x2400);
                c.bus.write32(DATA + 0x2400 + 0x95, seed % 4);
            },
        );
    }

    #[test]
    fn cfe_state_char_equip_show_buy_mode_matches() {
        check(
            ee::s03::f_0019a308__generated,
            cfe_state_char_equip_show_buy_mode,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x19a3, 4);
                c.hooks.miss = fake_small;
                game_globals(c);
                let this = DATA;
                c.g[A0][0] = this as u64;
                c.bus
                    .write32(this + 0x48, if seed % 3 == 0 { 0 } else { DATA + 0x2000 });
                for (i, off) in [0x80u32, 0x6c, 0x60].iter().enumerate() {
                    if seed % (i as u32 + 2) != 0 {
                        let o = DATA + 0x2400 + 0x100 * i as u32;
                        c.bus.write32(this + *off, o);
                        let vt = vtable(c, &mut r, &[0x48]);
                        c.bus.write32(o + 8, vt);
                    }
                }
                c.bus.write32(this + 0x44, seed % 4);
                c.bus.write32(WORLD + 0x7c, DATA + 0x2c00);
            },
        );
    }
}
